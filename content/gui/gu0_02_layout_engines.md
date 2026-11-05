@@lesson gu0-layout-engines How Layout Engines Work: Box Model, Flex and Grid
@@lang python
@@text
Every GUI toolkit answers the same question each frame: given a window size and a tree of widgets, where does each one go and how big is it? CSS, WPF, Qt, Flutter, SwiftUI and the Rust toolkits use remarkably similar answers: a **box model**, **stack/flex** layouts that share space along an axis, and **grid** layouts with tracks. Knowing the algorithms explains the surprising cases: why a long word overflows a flex item, why `1fr` columns aren't always equal, why margins sometimes collapse.

## Two passes: measure and arrange

Most toolkits lay out in two passes:
1. **Measure** (bottom-up): each widget reports the size it would like given the space offered (*constraints*: min and max width and height).
2. **Arrange** (top-down): each parent decides its children's final positions and sizes.
Flutter phrases it as "constraints go down, sizes go up, parent sets position". WPF literally calls them `MeasureOverride` and `ArrangeOverride`.

## The box model

Content + **padding** + **border** + **margin**. With CSS `box-sizing: border-box` (what you almost always want), `width` includes padding and border. Margins between stacked blocks can **collapse** in CSS normal flow; flex and grid items' margins don't.

## Flex (stack) layout

Along the **main axis**:
1. Each item starts at its **basis** (`flex-basis`, or its content size).
2. **Free space** = container size − sum of bases − gaps.
3. If positive, share it in proportion to `flex-grow`. If negative, shrink items in proportion to `flex-shrink × basis` (big items shrink more).
4. Clamp to each item's min and max size; if clamping happens, redistribute the remainder among the others and repeat.
5. `justify-content` places leftover space; `align-items` handles the cross axis.

The classic trap: flex items have `min-width: auto` (their content's minimum), so a long URL refuses to shrink. Set `min-width: 0` on the item to let it shrink and truncate.

## Grid layout

Define **tracks**: fixed (`200px`), content-based (`auto`, `min-content`, `max-content`), `minmax(a, b)` and **fractions** (`1fr`). Fixed and content tracks are sized first; the remaining space is split between `fr` tracks proportionally. But an `fr` track never goes below its content's minimum (`1fr` is really `minmax(auto, 1fr)`), so one long item can make columns unequal. `minmax(0, 1fr)` forces truly equal columns. `repeat(auto-fit, minmax(240px, 1fr))` gives responsive card grids without media queries.

## In other toolkits

WPF `Grid` has `*` (star) sizing, which is `fr`; `StackPanel` is a non-flexing stack; `DockPanel` docks to edges. Qt layouts use stretch factors and size policies. egui lays out immediately in one pass (with `ui.horizontal`, `ui.columns`, `egui_extras::StripBuilder`). Slint and iced use flex-like rows and columns.

@@quiz
In a flex container 600 px wide, two items have basis 200 px and flex-grow 1 and 2. How wide is the second item?
- 200 px
- 300 px
* 333 px
- 400 px
? Free 200 px split 1:2, so 200 + 133.

@@quiz
Why might a flex item containing a long URL refuse to shrink?
- flex-shrink defaults to 0
* Its min-width is auto (its content's minimum width)
- URLs are images
- Flex doesn't support text
? Set min-width: 0.

@@quiz
What does `minmax(0, 1fr)` fix?
* Columns that become unequal because an fr track won't go below its content's minimum
- Rows that are too tall
- Gaps that collapse
- Items that overflow vertically
? 1fr means minmax(auto, 1fr).

@@quiz
What's the order of the two layout passes?
- Arrange, then measure
* Measure (sizes up), then arrange (positions down)
- Only arrange
- Paint, then measure
? Constraints go down, sizes go up.

@@quiz
With `box-sizing: border-box`, what does `width: 200px` include?
- Content only
* Content, padding and border
- Content, padding, border and margin
- Margin only
? Margin is always outside.

@@flashcards
measure :: child reports desired size
arrange :: parent sets final position and size
constraints :: min and max offered
border-box :: width includes padding and border
margin collapse :: normal flow only
flex-basis :: starting size
flex-grow :: share of positive free space
flex-shrink :: shrink weight × basis
min-width: auto :: content minimum on flex items
min-width: 0 :: let a flex item truncate
fr :: fraction of leftover grid space
1fr :: minmax(auto, 1fr)
minmax(0, 1fr) :: truly equal columns
auto-fit + minmax :: responsive grid
WPF star sizing :: like fr

@@drill
Which CSS value lets a flex item shrink below its content's minimum width? (property: value)
= min-width: 0
= min-width:0
= min-width: 0px
? The default is auto.

@@drill
What grid track definition gives truly equal columns even with long content?
= minmax(0, 1fr)
= minmax(0,1fr)
? 1fr alone respects the content minimum.

@@exercise Resolve a flex row
Implement the flex algorithm for one line: start from each item's basis, distribute positive free space by `grow` or negative free space by `shrink × basis`, clamp to `min`/`max`, and repeat the distribution among unfrozen items until nothing violates a constraint. Print the final sizes for several container widths.
@@starter
items = [
    {"name": "sidebar", "basis": 240, "grow": 0, "shrink": 1, "min": 180, "max": 320},
    {"name": "content", "basis": 400, "grow": 1, "shrink": 1, "min": 200, "max": 10_000},
    {"name": "inspector", "basis": 200, "grow": 1, "shrink": 2, "min": 0, "max": 300},
]
GAP = 16

def flex(items, width):
    raise NotImplementedError
@@expected
1400 px: sidebar 240, content 828, inspector 300
900 px: sidebar 240, content 414, inspector 214
700 px: sidebar 200.3, content 333.8, inspector 133.8
500 px: sidebar 180, content 244, inspector 44
@@requires
def flex
GAP
frozen
@@hint
Keep `sizes` and a `frozen` set. Loop: free = width − gaps − sum(sizes); among unfrozen items compute each one's share (by grow if free > 0, else by shrink × basis); tentatively apply; clamp; if any item was clamped, freeze those items with their clamped size, reset the others to their basis, and loop again; otherwise stop. Items with weight 0 are frozen at their basis.
@@solution
items = [
    {"name": "sidebar", "basis": 240, "grow": 0, "shrink": 1, "min": 180, "max": 320},
    {"name": "content", "basis": 400, "grow": 1, "shrink": 1, "min": 200, "max": 10_000},
    {"name": "inspector", "basis": 200, "grow": 1, "shrink": 2, "min": 0, "max": 300},
]
GAP = 16

def flex(items, width):
    available = width - GAP * (len(items) - 1)
    sizes = {it["name"]: it["basis"] for it in items}
    frozen = set()
    while True:
        free = available - sum(sizes.values())
        growing = free > 0
        weights = {it["name"]: (it["grow"] if growing else it["shrink"] * it["basis"])
                   for it in items if it["name"] not in frozen}
        total = sum(weights.values())
        if total == 0 or abs(free) < 1e-9:
            break
        tentative = dict(sizes)
        for name, w in weights.items():
            tentative[name] = sizes[name] + free * w / total
        clamped = []
        for it in items:
            name = it["name"]
            if name in weights:
                bounded = min(max(tentative[name], it["min"]), it["max"])
                if bounded != tentative[name]:
                    clamped.append((name, bounded))
        if not clamped:
            sizes = tentative
            break
        for name, bounded in clamped:
            sizes[name] = bounded
            frozen.add(name)
    return {k: round(v, 1) for k, v in sizes.items()}

for width in (1400, 900, 700, 500):
    sizes = flex(items, width)
    used = sum(sizes.values()) + GAP * (len(items) - 1)
    note = f", overflows by {used - width:.0f} px" if used > width + 0.5 else ""
    print(f"{width} px: " + ", ".join(f"{k} {v:g}" for k, v in sizes.items()) + note)

@@exercise Size grid tracks
Given a container width, a gap and track definitions (`"200px"`, `"auto"` with a content width, or `"Nfr"` with a content minimum), size the tracks: fixed and auto first, then split the rest between fr tracks in proportion, but never below a track's content minimum (when that happens, fix the track at its minimum and re-split among the others). Compare `1fr` with `minmax(0, 1fr)` behaviour.
@@starter
GAP = 12

def size_tracks(width, tracks):
    raise NotImplementedError
@@expected
sidebar 240px | 1fr | auto(80) in 900px: 240, 556, 80
1fr 2fr 1fr in 900px: 219, 438, 219
1fr x3, one with a 420px table in 900px: 228, 420, 228
minmax(0,1fr) x3, same table in 900px: 292, 292, 292
@@requires
def size_tracks
fr
GAP
@@hint
Each track is a tuple: `("px", 200)`, `("auto", content)` or `("fr", n, content_min)`. Leftover = width − gaps − fixed − auto. Loop: per-fr share = leftover / Σn over unfixed fr tracks; any fr track with `n × share < content_min` gets fixed at content_min and removed from the leftover; repeat. For the `minmax(0, 1fr)` case pass content minimum 0.
@@solution
GAP = 12

def size_tracks(width, tracks):
    sizes = [None] * len(tracks)
    leftover = width - GAP * (len(tracks) - 1)
    for i, t in enumerate(tracks):
        if t[0] in ("px", "auto"):
            sizes[i] = t[1]
            leftover -= t[1]
    flexible = [i for i, t in enumerate(tracks) if t[0] == "fr"]
    while flexible:
        share = max(leftover, 0) / sum(tracks[i][1] for i in flexible)
        too_small = [i for i in flexible if tracks[i][1] * share < tracks[i][2]]
        if not too_small:
            for i in flexible:
                sizes[i] = tracks[i][1] * share
            break
        for i in too_small:
            sizes[i] = tracks[i][2]
            leftover -= tracks[i][2]
            flexible.remove(i)
    return [round(s, 1) for s in sizes]

layouts = {
    "sidebar 240px | 1fr | auto(80)": [("px", 240), ("fr", 1, 0), ("auto", 80)],
    "1fr 2fr 1fr": [("fr", 1, 0), ("fr", 2, 0), ("fr", 1, 0)],
    "1fr x3, one with a 420px table": [("fr", 1, 50), ("fr", 1, 420), ("fr", 1, 50)],
    "minmax(0,1fr) x3, same table": [("fr", 1, 0), ("fr", 1, 0), ("fr", 1, 0)],
}
for name, tracks in layouts.items():
    print(f"{name} in 900px: " + ", ".join(f"{s:g}" for s in size_tracks(900, tracks)))
