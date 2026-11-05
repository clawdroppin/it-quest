@@lesson vu3-icon-geometry Crisp Icons: Keylines, Pixel Snapping and Scaling Icon Sets
@@concepts numbers, lists, functions
@@lang python
@@text
An icon set looks professional when every icon feels the same size and weight, and when lines are **crisp** rather than blurry at their intended size. Both depend on simple geometry: keyline shapes, stroke alignment and pixel snapping. This lesson works through them with numbers, so you can audit icons from Inkscape or Penpot and scale a set correctly.

## Keylines and optical size

- A 24 × 24 icon usually has a **2 px padding** (live area 20 × 20) and **keyline shapes**: a circle of about 20 px diameter, a square of about 18 px, and portrait and landscape rectangles. A circle drawn as large as a square **looks smaller**, so round shapes get slightly bigger (**optical correction**).
- Keep one **stroke width** across the set (e.g. 2 px at 24 px), the same corner radius, and the same cap and join style.

## Pixel snapping

- On a 1x screen, a **1 px** (or any odd-width) stroke centred on a whole-number coordinate covers two half pixels, so it renders as a 2 px grey blur. Put odd-width strokes on **half-pixel** coordinates (x = 4.5), and even-width strokes (2 px) on **whole** coordinates.
- Filled shapes should have edges on **whole** pixels.
- On 2x and 3x screens this matters less, but 1x displays and small sizes (16 px) still show the blur.
- In Inkscape: enable snapping to the pixel grid (Document Properties → Grid, 1 px spacing), and check the X/Y/W/H fields in the toolbar.

## Scaling an icon set

- Scaling a 24 px icon to 16 px scales the stroke too: 2 px becomes 1.33 px, which is blurry and too thin. Proper sets are **redrawn or adjusted per size**: keep the stroke at a whole pixel value (1.5 or 2 at 16 px), simplify details, and re-snap.
- In SVG code, `vector-effect="non-scaling-stroke"` keeps the stroke width constant while the shape scales. It's useful for UI icons that are resized by CSS.

@@quiz
A 1 px vertical line drawn at x = 4 on a 1x screen looks blurry. Why?
* It's centred on a pixel boundary, so it covers half of two pixels
- 1 px lines can't be drawn
- SVG antialiasing is broken
- The colour is wrong
? Move it to x = 4.5.

@@quiz
Where should a 2 px stroke be centred to render crisply?
* On whole-pixel coordinates
- On half-pixel coordinates
- Anywhere
- On quarter pixels
? Even widths go on whole pixels; odd widths on halves.

@@quiz
Why is a keyline circle larger than the keyline square?
* Round shapes look smaller than squares of the same width, so they are optically corrected
- To save space
- Circles are harder to draw
- It's a Penpot limitation
? Optical size beats mathematical size.

@@quiz
What happens to a 2 px stroke when a 24 px icon is scaled to 16 px?
- It stays 2 px
* It becomes about 1.33 px, blurry and thin
- It becomes 3 px
- It disappears
? Adjust stroke widths per size, or use non-scaling-stroke.

@@quiz
What does vector-effect="non-scaling-stroke" do?
* Keeps the stroke width constant while the shape is scaled
- Removes the stroke
- Converts strokes to fills
- Snaps to pixels
? Handy for icons resized with CSS.

@@flashcards
live area :: icon area inside the padding, e.g. 20 × 20 in 24
keyline shapes :: guide circle, square and rectangles
optical correction :: round shapes drawn slightly bigger
consistent stroke :: same width, caps, joins across the set
odd stroke width :: centre on half pixels
even stroke width :: centre on whole pixels
fills :: edges on whole pixels
1x screens :: where blur shows most
per-size icons :: adjust or redraw for 16, 20, 24, 32
non-scaling-stroke :: stroke width ignores the transform

@@drill
A 1 px horizontal line should sit at y = 12 or y = 12.5 for crisp rendering?
= 12.5
? Odd widths go on half pixels.

@@drill
What does a 2 px stroke become when an icon is scaled from 24 px to 12 px?
= 1
= 1px
? 2 × 12 ÷ 24.

@@exercise Snap icon strokes for crisp rendering
For each line in an icon (endpoints and stroke width), check whether it renders crisply on a 1x screen: odd widths need half-pixel coordinates across the line, even widths need whole ones (only the coordinate across the line matters, i.e. x for vertical lines and y for horizontal ones). Suggest the nearest crisp coordinate for each blurry line.
@@starter
import math
lines = [
    ("left edge", (4, 3), (4, 21), 1),
    ("top bar", (4.5, 6.5), (19.5, 6.5), 1),
    ("divider", (3, 12.5), (21, 12.5), 2),
    ("handle", (12, 2), (12, 7), 2),
    ("right edge", (19.25, 3), (19.25, 21), 1),
    ("diagonal", (5, 5), (19, 19), 2),
]

def crisp(coord, width):
    raise NotImplementedError
@@expected
left edge   1px at x=4: blurry -> move to x=4.5
top bar     1px at y=6.5: crisp
divider     2px at y=12.5: blurry -> move to y=12
handle      2px at x=12: crisp
right edge  1px at x=19.25: blurry -> move to x=19.5
diagonal    diagonal: always antialiased, fine
@@requires
def crisp
width % 2
math.floor
@@hint
Vertical lines have equal x values, and horizontal lines have equal y values. Diagonals can't be pixel-snapped, so report them as antialiased (that's fine). The crisp target is `math.floor(c) + 0.5` for odd widths and `round(c)` for even widths. A coordinate is crisp if it already equals its target.
@@solution
import math
lines = [
    ("left edge", (4, 3), (4, 21), 1),
    ("top bar", (4.5, 6.5), (19.5, 6.5), 1),
    ("divider", (3, 12.5), (21, 12.5), 2),
    ("handle", (12, 2), (12, 7), 2),
    ("right edge", (19.25, 3), (19.25, 21), 1),
    ("diagonal", (5, 5), (19, 19), 2),
]

def crisp(coord, width):
    return math.floor(coord) + 0.5 if width % 2 else round(coord)

for name, (x1, y1), (x2, y2), width in lines:
    if x1 == x2:
        axis, coord = "x", x1
    elif y1 == y2:
        axis, coord = "y", y1
    else:
        print(f"{name:<11} diagonal: always antialiased, fine")
        continue
    target = crisp(coord, width)
    if target == coord:
        print(f"{name:<11} {width}px at {axis}={coord:g}: crisp")
    else:
        print(f"{name:<11} {width}px at {axis}={coord:g}: blurry -> move to {axis}={target:g}")

@@exercise Scale an icon set to new sizes
Scale icon geometry from 24 px to 16, 20 and 32 px. For each size, compare the naive scaled stroke width with a hand-tuned stroke (rounded to the nearest 0.5 px, at least 1 px), and report whether the scaled keyline circle still has whole or half-pixel edges, flagging sizes where the icon should be redrawn rather than scaled.
@@starter
BASE = 24
icon = {"stroke": 2.0, "padding": 2.0, "circle_d": 20.0, "corner_r": 2.0}
SIZES = [12, 16, 20, 32, 48]

def scale(size):
    raise NotImplementedError
@@expected
12px: stroke 1.00 -> tune to 1, padding 1.00, circle 10.00; scales cleanly
16px: stroke 1.33 -> tune to 1.5, padding 1.33, circle 13.33; redraw: off the half-pixel grid (stroke, padding, circle_d, corner_r)
20px: stroke 1.67 -> tune to 1.5, padding 1.67, circle 16.67; redraw: off the half-pixel grid (stroke, padding, circle_d, corner_r)
32px: stroke 2.67 -> tune to 2.5, padding 2.67, circle 26.67; redraw: off the half-pixel grid (stroke, padding, circle_d, corner_r)
48px: stroke 4.00 -> tune to 4, padding 4.00, circle 40.00; scales cleanly
@@requires
def scale
BASE
round(
@@hint
The factor is `size / BASE`. Multiply every value. Tuned stroke = `max(1, round(stroke * 2) / 2)`. A value is "on the half-pixel grid" when `value * 2` is a whole number. If the stroke or padding is off the grid, recommend redrawing at that size.
@@solution
BASE = 24
icon = {"stroke": 2.0, "padding": 2.0, "circle_d": 20.0, "corner_r": 2.0}
SIZES = [12, 16, 20, 32, 48]

def scale(size):
    f = size / BASE
    return {k: v * f for k, v in icon.items()}

def on_half_grid(v):
    return abs(v * 2 - round(v * 2)) < 1e-9

for size in SIZES:
    s = scale(size)
    tuned = max(1, round(s["stroke"] * 2) / 2)
    off = [k for k, v in s.items() if not on_half_grid(v)]
    verdict = f"redraw: off the half-pixel grid ({', '.join(off)})" if off else "scales cleanly"
    print(f"{size}px: stroke {s['stroke']:.2f} -> tune to {tuned:g}, padding {s['padding']:.2f}, circle {s['circle_d']:.2f}; {verdict}")
