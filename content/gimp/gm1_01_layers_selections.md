@@lesson gm1-layers Layers, Selections and Masks
@@concepts numbers, lists, functions
@@lang python
@@text
Three ideas make GIMP powerful, and they're the same three in every serious image editor: **layers** stack images on top of each other, **selections** limit where an edit happens, and **masks** hide parts of a layer without deleting anything. Learn these and the rest is just tools.

## Layers

The **Layers** dialog (Ctrl+L) shows the stack: the top layer covers the ones below. Each layer has:

- an **eye** (visibility) and a **chain** (link layers to move them together); GIMP 3 lets you select several layers with Shift/Ctrl+click and move, group or delete them together;
- **Opacity** (0–100%) and a **Mode** (blend mode, below);
- an **alpha channel** (transparency). Layers without one show their name in bold; add one with **Layer → Transparency → Add Alpha Channel**, otherwise erasing paints the background colour instead of transparency.

Useful commands:

| Command | Does |
|---|---|
| New Layer (Shift+Ctrl+N) | an empty layer: choose fill (transparent!) |
| Duplicate Layer (Shift+Ctrl+D) | copy, the safest way to try something |
| Anchor (Ctrl+H) / To New Layer | finish a **floating selection** (what a paste creates) |
| Merge Down / Flatten Image | combine layers (flatten removes all transparency) |
| Layer Groups | folders of layers, with their own opacity and mode |
| Layer → Layer Attributes | name, offsets, colour tag |
| Lock pixels / position / alpha | the lock icons, so you don't paint on the wrong layer |

**Work non-destructively**: duplicate before big changes, keep the original photo as the bottom layer, use masks instead of the eraser, and use GIMP 3's **non-destructive filters** (the filter stays editable in the layer's fx list).

## How layers combine

Every pixel has red, green, blue and alpha. "Normal" mode puts the top layer **over** the one below, weighted by its alpha:

```text
out_alpha = top_alpha + bottom_alpha × (1 − top_alpha)
out_color = (top_color × top_alpha + bottom_color × bottom_alpha × (1 − top_alpha)) / out_alpha
```

(with values from 0 to 1). The first exercise implements it.

**Blend modes** use other formulas, applied channel by channel:

| Mode | Formula (0–1 values, a = top, b = bottom) | Effect |
|---|---|---|
| **Multiply** | a × b | always darker; white disappears: line art, shadows |
| **Screen** | 1 − (1 − a)(1 − b) | always lighter; black disappears: glows, light leaks |
| **Overlay** | b < 0.5: 2ab, else 1 − 2(1 − a)(1 − b) | more contrast; 50% grey disappears |
| Soft light | a gentler overlay | subtle contrast |
| Addition / Subtract | a + b / b − a | light effects |
| Difference | abs(a − b) | comparing two versions: identical pixels go black |
| Hue / Saturation / Color / Value (LCh) | take one property from the top | recolouring |

The second exercise implements Multiply, Screen and Overlay.

## Selections

A selection is the area where tools and filters work; outside it nothing changes. The "marching ants" show its edge.

| Tool | Best for |
|---|---|
| Rectangle / Ellipse (R/E) | regular shapes; hold Shift for squares and circles after starting the drag |
| Free Select (F) | polygons (click) and freehand (drag); press Enter to close |
| Fuzzy Select (U) | areas of similar colour touching the click; the **Threshold** option sets how similar |
| Select by Color (Shift+O) | similar colours anywhere in the layer |
| Scissors (I) | follows edges between clicks |
| **Foreground Select** | cuts out hair and fuzzy subjects: rough outline, paint over the subject, preview, Enter |
| Paths (B) | precise curves; **Selection from Path** (Shift+V) |

**Modes** combine selections. Hold a key **before** you start dragging:

| Key | Mode |
|---|---|
| (none) | replace |
| **Shift** | add (union) |
| **Ctrl** | subtract |
| **Shift+Ctrl** | intersect |

The **Select** menu: All (Ctrl+A), None (Shift+Ctrl+A), **Invert** (Ctrl+I), **Feather** (soft edges), Grow / Shrink, Border, Rounded Rectangle, **Save to Channel**, and **Quick Mask** (Shift+Q): paint the selection with a brush in red overlay, then toggle back. The third exercise models the selection modes as sets of pixels.

## Layer masks

A **layer mask** is a greyscale image attached to a layer: **white shows** the layer, **black hides** it, grey is partly transparent. Unlike the eraser, nothing is lost: paint white to bring it back.

1. **Layer → Mask → Add Layer Mask**: start from White (full opacity), Black, the **Selection** (great after cutting something out), or a greyscale copy.
2. Click the mask thumbnail (it gets a white border) and paint with black/white, or a **gradient** for a smooth fade between two photos.
3. **Ctrl+click** the mask to disable it temporarily, **Alt+click** to view it alone.
4. **Apply Layer Mask** when finished (or keep it).

The resulting transparency is the layer's alpha multiplied by the mask (`alpha × mask / 255`), which is part of the third exercise.

Classic projects that use all three ideas: **replace a sky** (select the sky with Fuzzy Select, mask the new sky layer with that selection, feather the edge), **cut out a subject** (Foreground Select → mask → new background), **blend two photos** (a black-to-white gradient on a mask).

@@quiz
Erasing on the bottom layer paints white instead of making it transparent. Why?
- The eraser is in Multiply mode
* The layer has no alpha channel; add one with Layer → Transparency → Add Alpha Channel
- The image is in Indexed mode
- The layer is locked
? Layers without an alpha channel show their names in bold.

@@quiz
Which blend mode makes white disappear and keeps dark lines, as used for line art over colours?
* Multiply
- Screen
- Addition
- Difference
? a × 1 = a: white leaves the bottom unchanged.

@@quiz
You have a rectangle selection and want to add an ellipse to it. What do you hold?
- Ctrl
* Shift, before starting the ellipse drag
- Alt
- Shift+Ctrl
? Ctrl subtracts, Shift+Ctrl intersects.

@@quiz
Why use a layer mask instead of the eraser?
- Masks are faster to paint
* A mask hides pixels without deleting them: painting white brings them back at any time
- The eraser doesn't work on photos
- Masks reduce the file size
? Non-destructive editing keeps every option open.

@@quiz
You pasted an image and the Layers dialog shows a "Floating Selection". What do you do?
- Flatten the image
* Anchor it to the current layer (Ctrl+H) or make it a new layer (Layer → To New Layer)
- Save as JPEG
- Invert the selection
? Most operations are blocked until the floating selection is placed.

@@flashcards
layer stack :: upper layers cover lower ones
alpha channel :: per-pixel transparency of a layer
floating selection :: a pasted area waiting to be anchored or made a new layer
Duplicate Layer :: Shift+Ctrl+D, the safe way to experiment
Flatten Image :: merge everything and remove transparency
non-destructive filters :: GIMP 3 filters that stay editable
Normal / over :: top weighted by its alpha over the bottom
Multiply :: a × b, darker, white vanishes
Screen :: 1 − (1 − a)(1 − b), lighter, black vanishes
Overlay :: contrast, 50% grey vanishes
Difference :: compare versions; identical = black
Fuzzy Select :: similar touching colours, Threshold option
Foreground Select :: cut out hair and fuzzy edges
Shift / Ctrl / Shift+Ctrl :: add / subtract / intersect selections
Ctrl+I :: invert the selection
Feather :: soft selection edges
Quick Mask (Shift+Q) :: paint a selection
layer mask :: white shows, black hides, grey partly
Alt+click mask :: view the mask alone

@@exercise Normal mode: one pixel over another
Pixels are `(r, g, b, a)` tuples of integers 0–255. Write `over(top, bottom)` implementing the formula from the lesson: convert to 0–1, compute, convert back with `round(x * 255)`. If the resulting alpha is 0, return `(0, 0, 0, 0)`. Then `flatten(layers)`: combine a list of pixels from bottom to top (the first item is the bottom layer).
@@starter
def over(top, bottom):
    pass


def flatten(layers):
    pass


print(over((255, 0, 0, 128), (0, 0, 255, 255)))
print(flatten([(255, 255, 255, 255), (0, 0, 0, 64), (255, 200, 0, 0)]))
@@tests
check(over((255, 0, 0, 128), (0, 0, 255, 255)), (128, 0, 127, 255), "half red over blue")
check(over((10, 20, 30, 255), (200, 200, 200, 255)), (10, 20, 30, 255), "opaque top wins")
check(over((10, 20, 30, 0), (200, 100, 50, 255)), (200, 100, 50, 255), "transparent top")
check(over((0, 0, 0, 0), (0, 0, 0, 0)), (0, 0, 0, 0), "nothing at all")
check(over((255, 255, 255, 128), (0, 0, 0, 128)), (170, 170, 170, 192), "two half-transparent layers")
check(flatten([(255, 255, 255, 255), (0, 0, 0, 64), (255, 200, 0, 0)]), (191, 191, 191, 255), "a stack")
@@hint
`ta = top[3] / 255`, `ba = bottom[3] / 255`, `oa = ta + ba * (1 - ta)`. For each colour channel `i` in 0..2: `(top[i] / 255 * ta + bottom[i] / 255 * ba * (1 - ta)) / oa`. For `flatten`, start with `layers[0]` and put each next layer `over` the result.
@@solution
def over(top, bottom):
    ta = top[3] / 255
    ba = bottom[3] / 255
    oa = ta + ba * (1 - ta)
    if oa == 0:
        return (0, 0, 0, 0)
    color = [round((top[i] / 255 * ta + bottom[i] / 255 * ba * (1 - ta)) / oa * 255) for i in range(3)]
    return (color[0], color[1], color[2], round(oa * 255))


def flatten(layers):
    result = layers[0]
    for layer in layers[1:]:
        result = over(layer, result)
    return result


print(over((255, 0, 0, 128), (0, 0, 255, 255)))
print(flatten([(255, 255, 255, 255), (0, 0, 0, 64), (255, 200, 0, 0)]))

@@exercise Blend modes
Write `multiply(a, b)`, `screen(a, b)` and `overlay(a, b)` for single channel values 0–255 (`a` is the top layer, `b` the bottom), using the formulas from the lesson on 0–1 values and returning `round(x * 255)`. Overlay chooses its formula by the **bottom** value. Then `blend(mode, top_rgb, bottom_rgb)` applies one of them (by name) to each channel of two RGB tuples.
@@starter
def multiply(a, b):
    pass


def screen(a, b):
    pass


def overlay(a, b):
    pass


def blend(mode, top_rgb, bottom_rgb):
    pass


print(blend("multiply", (200, 255, 0), (100, 100, 100)))
print(blend("screen", (200, 0, 255), (100, 100, 100)))
print(blend("overlay", (200, 200, 128), (100, 200, 128)))
@@tests
check(multiply(200, 100), 78, "multiply darkens")
check(multiply(255, 123), 123, "white disappears in multiply")
check(screen(200, 100), 222, "screen lightens")
check(screen(0, 123), 123, "black disappears in screen")
check(overlay(200, 100), 157, "overlay on a dark base")
check(overlay(200, 200), 231, "overlay on a light base")
check(overlay(128, 60), 60, "about 50% grey disappears in overlay")
check(blend("multiply", (200, 255, 0), (100, 100, 100)), (78, 100, 0), "per channel")
check(blend("screen", (200, 0, 255), (100, 100, 100)), (222, 100, 255), "screen per channel")
@@hint
Divide by 255 first: `a, b = a / 255, b / 255`. Overlay: `2 * a * b if b < 0.5 else 1 - 2 * (1 - a) * (1 - b)`. In `blend`, look the function up in a dict: `{"multiply": multiply, ...}[mode]`, then use a `tuple(...)` of `f(t, b)` for each pair from `zip`.
@@solution
def multiply(a, b):
    return round(a / 255 * b / 255 * 255)


def screen(a, b):
    a, b = a / 255, b / 255
    return round((1 - (1 - a) * (1 - b)) * 255)


def overlay(a, b):
    a, b = a / 255, b / 255
    x = 2 * a * b if b < 0.5 else 1 - 2 * (1 - a) * (1 - b)
    return round(x * 255)


def blend(mode, top_rgb, bottom_rgb):
    f = {"multiply": multiply, "screen": screen, "overlay": overlay}[mode]
    return tuple(f(t, b) for t, b in zip(top_rgb, bottom_rgb))


print(blend("multiply", (200, 255, 0), (100, 100, 100)))
print(blend("screen", (200, 0, 255), (100, 100, 100)))
print(blend("overlay", (200, 200, 128), (100, 200, 128)))

@@exercise Selection modes and masks
Model a selection as a `set` of `(x, y)` pixels. Write `rect(x, y, w, h)` (all pixels in the rectangle), `combine(current, new, mode)` with modes `"replace"`, `"add"`, `"subtract"`, `"intersect"`, and `invert(selection, width, height)`. Then `apply_mask(alpha, mask)` for one pixel: the visible alpha is `round(alpha * mask / 255)`.
@@starter
def rect(x, y, w, h):
    pass


def combine(current, new, mode):
    pass


def invert(selection, width, height):
    pass


def apply_mask(alpha, mask):
    pass


sel = rect(0, 0, 4, 4)
sel = combine(sel, rect(2, 2, 4, 4), "add")         # Shift + drag
sel = combine(sel, rect(0, 0, 2, 2), "subtract")    # Ctrl + drag
print(len(sel), "pixels selected")
print(apply_mask(255, 128), apply_mask(200, 0))
@@tests
check(len(rect(0, 0, 4, 3)), 12, "4 x 3")
check((3, 2) in rect(0, 0, 4, 3) and (4, 0) not in rect(0, 0, 4, 3), True, "edges")
a, b = rect(0, 0, 4, 4), rect(2, 2, 4, 4)
check(len(combine(a, b, "replace")), 16, "replace")
check(len(combine(a, b, "add")), 28, "add: 16 + 16 - 4 overlap")
check(len(combine(a, b, "subtract")), 12, "subtract")
check(combine(a, b, "intersect"), rect(2, 2, 2, 2), "intersect")
check(len(invert(rect(0, 0, 2, 2), 4, 4)), 12, "invert")
check([apply_mask(255, m) for m in (255, 128, 0)], [255, 128, 0], "white, grey, black mask")
check(apply_mask(200, 64), 50, "partly transparent layer and mask")
@@hint
`{(i, j) for i in range(x, x + w) for j in range(y, y + h)}`. Sets have `|` (union), `-` (difference) and `&` (intersection). Invert is `rect(0, 0, width, height) - selection`.
@@solution
def rect(x, y, w, h):
    return {(i, j) for i in range(x, x + w) for j in range(y, y + h)}


def combine(current, new, mode):
    if mode == "replace":
        return set(new)
    if mode == "add":
        return current | new
    if mode == "subtract":
        return current - new
    if mode == "intersect":
        return current & new
    raise ValueError(mode)


def invert(selection, width, height):
    return rect(0, 0, width, height) - selection


def apply_mask(alpha, mask):
    return round(alpha * mask / 255)


sel = rect(0, 0, 4, 4)
sel = combine(sel, rect(2, 2, 4, 4), "add")         # Shift + drag
sel = combine(sel, rect(0, 0, 2, 2), "subtract")    # Ctrl + drag
print(len(sel), "pixels selected")
print(apply_mask(255, 128), apply_mask(200, 0))
