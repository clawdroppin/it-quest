@@lesson vu1-components-layout Components in Depth: Variant Matrices, Hug and Fill, and Flex Sizing
@@concepts lists, loops, functions
@@lang python
@@text
A design system lives or dies by its components. Two skills separate a tidy Penpot library from a messy one: planning **variants** as a matrix (so nothing is missing and nothing is duplicated), and understanding how **flex layout** sizes things, because Penpot's "hug", "fill" and "fixed" options are CSS flexbox underneath.

## Variants as a matrix

- A component like a button has **properties**: `variant` (primary, secondary, ghost, danger), `size` (sm, md, lg), `state` (default, hover, pressed, disabled, focus) and `icon` (none, leading, trailing).
- The full matrix is the product of the options: 4 × 3 × 5 × 3 = **180** combinations. Most libraries **don't draw them all**. States like hover and focus are often handled in code, or shown once as documentation.
- Rules of thumb:
  - Make something a property when it changes the **structure or tokens**. Make it an **override** when it's just text or an icon swap.
  - Name layers the same across variants, so overrides survive swapping.
  - Check for **gaps** (a combination that should exist but doesn't) and **orphans** (variants with misspelled property values).

## Hug, fill, fixed: flexbox underneath

| Penpot | CSS (child of a flex container) |
|---|---|
| Fixed width | `flex: none; width: 120px` |
| Hug contents | `flex: none; width: auto` (sized by content) |
| Fill container | `flex: 1 1 0` (shares the free space) |
| Min / max width | `min-width` / `max-width` |

How the row is sized:
1. Take the container width minus padding and gaps.
2. Subtract the fixed and hug widths.
3. Share the rest among the fill children in proportion to their `flex-grow`, but clamp each to its min and max widths. If one is clamped, re-share what's left among the others.

This is why a "fill" search box shrinks when you add a button beside it, and why a min-width keeps it usable.

@@quiz
A button has 4 variants, 3 sizes and 2 icon options. How many combinations?
- 9
* 24
- 12
- 48
? 4 × 3 × 2.

@@quiz
Which Penpot sizing option corresponds to `flex: 1 1 0`?
- Fixed
- Hug contents
* Fill container
- Absolute
? Fill children share the free space.

@@quiz
When should something be a variant property rather than an override?
* When it changes the structure or tokens, like size or colour role
- When it's just the label text
- Never; use separate components
- Only for icons
? Overrides are for content swaps.

@@quiz
A fill child has a max-width and hits it. What happens to the remaining space?
- It disappears
* It is shared among the other fill children
- It goes to the hug children
- The layout breaks
? Flexbox re-distributes after clamping.

@@quiz
Why keep layer names identical across variants?
* So overrides like text and icons survive when you swap variants
- Penpot requires unique names
- To make files smaller
- For export to PNG
? Swapping matches layers by name.

@@flashcards
variant property :: axis like size, state, variant
variant matrix :: product of all property options
override :: content change on an instance, like text
gap :: a missing combination
orphan :: a variant with an unexpected property value
fixed :: flex none, explicit width
hug :: sized by its content
fill :: flex 1 1 0, shares free space
min / max width :: clamp a fill child
free space :: container − padding − gaps − fixed/hug widths
flex-grow ratio :: how fill children share space

@@drill
A row is 600 px wide with 16 px padding on each side, two 8 px gaps, one fixed 100 px button and two fill children. How wide is each fill child?
= 226
= 226px
? 600 − 32 − 16 − 100 = 452, ÷ 2.

@@drill
Which CSS property limits how small a fill child can shrink?
= min-width
? Penpot shows it as Min W.

@@exercise Find gaps and orphans in a variant set
Given a component's property options and the variants that were actually drawn (as `prop=value` strings like Penpot names them), list the variants that are expected but missing (only for the states the library draws), and the drawn variants that use unknown property values.
@@starter
from itertools import product
PROPS = {"variant": ["primary", "secondary", "danger"], "size": ["sm", "md", "lg"], "state": ["default", "disabled"]}
drawn = [
    "variant=primary, size=sm, state=default", "variant=primary, size=md, state=default", "variant=primary, size=lg, state=default",
    "variant=primary, size=md, state=disabled", "variant=primary, size=sm, state=disabled", "variant=primary, size=lg, state=disabled",
    "variant=secondary, size=sm, state=default", "variant=secondary, size=md, state=default", "variant=secondary, size=lg, state=default",
    "variant=secondary, size=md, state=disabled",
    "variant=danger, size=md, state=default", "variant=danger, size=medium, state=default", "variant=Danger, size=lg, state=default",
]

def parse(name):
    raise NotImplementedError
@@expected
11 of 18 combinations drawn
missing: variant=danger, size=lg, state=default
missing: variant=danger, size=lg, state=disabled
missing: variant=danger, size=md, state=disabled
missing: variant=danger, size=sm, state=default
missing: variant=danger, size=sm, state=disabled
missing: variant=secondary, size=lg, state=disabled
missing: variant=secondary, size=sm, state=disabled
orphan:  variant=danger, size=medium, state=default  (unknown: size=medium)
orphan:  variant=Danger, size=lg, state=default  (unknown: variant=Danger)
@@requires
def parse
product
PROPS
product(
@@hint
`parse` splits on ", " and then on "=" into a dict. The expected set is every combination from `product(*PROPS.values())`. A drawn variant is an orphan if any value isn't in its property's options. The gaps are the expected combinations not drawn.
@@solution
from itertools import product
PROPS = {"variant": ["primary", "secondary", "danger"], "size": ["sm", "md", "lg"], "state": ["default", "disabled"]}
drawn = [
    "variant=primary, size=sm, state=default", "variant=primary, size=md, state=default", "variant=primary, size=lg, state=default",
    "variant=primary, size=md, state=disabled", "variant=primary, size=sm, state=disabled", "variant=primary, size=lg, state=disabled",
    "variant=secondary, size=sm, state=default", "variant=secondary, size=md, state=default", "variant=secondary, size=lg, state=default",
    "variant=secondary, size=md, state=disabled",
    "variant=danger, size=md, state=default", "variant=danger, size=medium, state=default", "variant=Danger, size=lg, state=default",
]

def parse(name):
    return dict(part.split("=") for part in name.split(", "))

keys = list(PROPS)
have, orphans = set(), []
for name in drawn:
    props = parse(name)
    bad = [f"{k}={v}" for k, v in props.items() if v not in PROPS.get(k, [])]
    if bad:
        orphans.append(f"{name}  (unknown: {', '.join(bad)})")
    else:
        have.add(tuple(props[k] for k in keys))
expected = set(product(*PROPS.values()))
print(f"{len(have)} of {len(expected)} combinations drawn")
for combo in sorted(expected - have):
    print("missing:", ", ".join(f"{k}={v}" for k, v in zip(keys, combo)))
for o in orphans:
    print("orphan: ", o)

@@exercise Compute a flex row like Penpot does
Lay out a row of children (fixed, hug or fill, with optional min and max widths) inside a container with padding and gaps: subtract the fixed and hug widths, share the remaining space among fill children by grow factor, clamp to min and max, and re-share until nothing changes. Print each child's width and x position for two container widths.
@@starter
PADDING, GAP = 16, 8
children = [
    {"name": "logo", "mode": "fixed", "width": 40},
    {"name": "search", "mode": "fill", "grow": 2, "min": 160, "max": 480},
    {"name": "tabs", "mode": "fill", "grow": 1, "min": 120},
    {"name": "avatar button", "mode": "hug", "content": 36},
]

def layout(container):
    raise NotImplementedError
@@expected
container 1200px (content needs 1200px)
  logo           x=  16.0 width=  40.0
  search         x=  64.0 width= 480.0
  tabs           x= 552.0 width= 588.0
  avatar button  x=1148.0 width=  36.0
container 420px (content needs 420px)
  logo           x=  16.0 width=  40.0
  search         x=  64.0 width= 168.0
  tabs           x= 240.0 width= 120.0
  avatar button  x= 368.0 width=  36.0
@@requires
def layout
grow
min
max
sum(
@@hint
Free space = container − 2 × PADDING − GAP × (n − 1) − the fixed and hug widths. Loop: share the free space among the unfrozen fill children by grow, and if any share is outside its min or max, freeze that child at the limit, subtract it, and repeat. Then walk left to right adding widths and gaps for the x positions.
@@solution
PADDING, GAP = 16, 8
children = [
    {"name": "logo", "mode": "fixed", "width": 40},
    {"name": "search", "mode": "fill", "grow": 2, "min": 160, "max": 480},
    {"name": "tabs", "mode": "fill", "grow": 1, "min": 120},
    {"name": "avatar button", "mode": "hug", "content": 36},
]

def layout(container):
    widths = {}
    free = container - 2 * PADDING - GAP * (len(children) - 1)
    for c in children:
        if c["mode"] != "fill":
            widths[c["name"]] = c["width"] if c["mode"] == "fixed" else c["content"]
            free -= widths[c["name"]]
    fills = [c for c in children if c["mode"] == "fill"]
    while fills:
        total_grow = sum(c["grow"] for c in fills)
        shares = {c["name"]: free * c["grow"] / total_grow for c in fills}
        clamped = [c for c in fills if shares[c["name"]] < c.get("min", 0) or shares[c["name"]] > c.get("max", float("inf"))]
        if not clamped:
            widths.update(shares)
            break
        for c in clamped:
            widths[c["name"]] = max(c.get("min", 0), min(c.get("max", float("inf")), shares[c["name"]]))
            free -= widths[c["name"]]
        fills = [c for c in fills if c not in clamped]
    x, out = PADDING, []
    for c in children:
        out.append((c["name"], x, widths[c["name"]]))
        x += widths[c["name"]] + GAP
    return out, x - GAP + PADDING

for container in (1200, 420):
    rows, used = layout(container)
    print(f"container {container}px (content needs {used:g}px{', overflows!' if used > container else ''})")
    for name, x, w in rows:
        print(f"  {name:<14} x={x:>6.1f} width={w:>6.1f}")
