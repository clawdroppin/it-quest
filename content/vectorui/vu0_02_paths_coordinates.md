@@lesson vu0-paths-coords SVG Geometry: Path Commands, Béziers, viewBox and Transforms
@@concepts numbers, strings
@@lang python
@@text
Design tools hide the maths, but when an icon is off by half a pixel, a shape gets clipped, or an exported SVG scales strangely, you need to understand what's underneath: path commands, Bézier curves, the `viewBox` coordinate system and transforms. This lesson explains each, and the exercises compute a curve's true bounding box and map viewBox coordinates to screen pixels.

## Path commands

| Command | Meaning | Parameters |
|---|---|---|
| `M` / `m` | move to | x y |
| `L` / `l`, `H`, `V` | line to, horizontal, vertical | x y / x / y |
| `C` / `c` | cubic Bézier | x1 y1 x2 y2 x y |
| `S` / `s` | smooth cubic (mirrors the previous control point) | x2 y2 x y |
| `Q` / `q`, `T` | quadratic Bézier, smooth quadratic | x1 y1 x y / x y |
| `A` / `a` | elliptical arc | rx ry rotation large-arc sweep x y |
| `Z` | close path | none |

Upper case = absolute coordinates, lower case = relative to the current point. Relative commands make shorter files and easier-to-move shapes.

## Bézier curves

A cubic Bézier from P0 to P3 with control points P1 and P2:
`B(t) = (1−t)³P0 + 3(1−t)²t P1 + 3(1−t)t² P2 + t³P3`, for t from 0 to 1.
- The curve starts heading towards P1 and arrives from the direction of P2; the **handles** you drag in Penpot or Inkscape are those control points.
- The curve stays inside the convex hull of its four points, but its **bounding box** can be smaller than the control points' box: the true extremes are at t where the derivative is zero (solve a quadratic per axis).
- Smooth nodes keep the handles on both sides collinear; **cusp** nodes break them.
- Circles are approximated with four cubics using the handle length 0.5523 × radius.

## viewBox and preserveAspectRatio

`viewBox="minX minY width height"` defines the user coordinate system; `width`/`height` set the size on the page. The browser scales the viewBox to fit:
- `preserveAspectRatio="xMidYMid meet"` (the default): uniform scale = **min**(sx, sy), centred; the whole drawing is visible (letterboxed).
- `slice`: uniform scale = **max**(sx, sy); fills the area, cropping the overflow.
- `none`: stretch each axis independently.
Icons drawn on a 24 × 24 viewBox scale cleanly to any size; keep important lines on whole or half pixels at the sizes you ship (a 2 px stroke centred on an integer coordinate looks crisp).

## Transforms

`transform="translate(x y) rotate(a cx cy) scale(s)"` applies **right to left** to coordinates: the last listed transform is applied first. A transform is a 2D affine matrix `matrix(a b c d e f)` meaning x' = a·x + c·y + e, y' = b·x + d·y + f. Inkscape's "apply transforms" (or SVGO's convertTransform) bakes them into coordinates for cleaner output.

@@quiz
What's the difference between `L 10 10` and `l 10 10`?
* L is absolute; l is relative to the current point
- l draws a thinner line
- L closes the path
- There is no difference
? Lower case is relative.

@@quiz
A viewBox of 0 0 100 50 is drawn in a 300 × 300 area with xMidYMid meet. What's the scale?
- 6
* 3
- 1
- 300
? min(300/100, 300/50) = 3.

@@quiz
With `slice` instead of `meet`, what happens?
- The drawing is stretched
* The area is filled with a uniform scale and overflow is cropped
- Nothing is drawn
- The viewBox is ignored
? max of the two scales.

@@quiz
Can a cubic Bézier's bounding box be smaller than the box around its control points?
* Yes; the curve rarely reaches its control points
- No, they're always equal
- Only for straight lines
- Only in Inkscape
? Find extremes where the derivative is zero.

@@quiz
In `transform="translate(10 0) scale(2)"`, which is applied to the coordinates first?
- translate
* scale
- They're applied together
- Neither; only the last counts
? Right to left.

@@flashcards
M / L / Z :: move, line, close
H / V :: horizontal / vertical line
C :: cubic Bézier, two control points
S :: smooth cubic, mirrored handle
Q :: quadratic Bézier
A :: elliptical arc
upper vs lower case :: absolute vs relative
Bézier handles :: control points
cusp node :: handles not collinear
circle handle factor :: 0.5523 × r
viewBox :: user coordinate system
meet :: fit inside, min scale
slice :: fill and crop, max scale
preserveAspectRatio none :: stretch
transform order :: right to left
matrix(a b c d e f) :: affine transform

@@drill
Which path command draws a cubic Bézier curve with absolute coordinates?
= C
? c is the relative form.

@@drill
What is the default preserveAspectRatio value?
= xMidYMid meet
? Centred and fully visible.

@@exercise The true bounding box of a curve
For each cubic Bézier, compute the naive bounding box (all four points) and the true bounding box: evaluate the curve at t = 0, t = 1 and every root in (0, 1) of the derivative for each axis. The derivative of one axis is the quadratic `a·t² + b·t + c` with `a = 3(−p0 + 3p1 − 3p2 + p3)`, `b = 6(p0 − 2p1 + p2)`, `c = 3(p1 − p0)`.
@@starter
import math

curves = {
    "arch": [(0, 0), (10, 40), (90, 40), (100, 0)],
    "s-curve": [(0, 50), (100, -20), (0, 120), (100, 50)],
    "gentle": [(0, 0), (30, 10), (70, 10), (100, 0)],
}

def point(curve, t):
    raise NotImplementedError
@@expected
arch: naive (0, 0, 100, 40) true (0, 0, 100, 30)
s-curve: naive (0, -20, 100, 120) true (0, 29.79, 100, 70.21)
gentle: naive (0, 0, 100, 10) true (0, 0, 100, 7.50)
@@requires
def point
def roots
math.sqrt(
@@hint
`roots(a, b, c)`: if `abs(a) < 1e-12` it's linear (`-c / b` if b isn't 0); otherwise use the discriminant `b*b - 4*a*c` and keep roots strictly between 0 and 1. Collect candidate t values for both axes, evaluate the curve at each and take the min/max of x and y.
@@solution
import math

curves = {
    "arch": [(0, 0), (10, 40), (90, 40), (100, 0)],
    "s-curve": [(0, 50), (100, -20), (0, 120), (100, 50)],
    "gentle": [(0, 0), (30, 10), (70, 10), (100, 0)],
}

def point(curve, t):
    (x0, y0), (x1, y1), (x2, y2), (x3, y3) = curve
    u = 1 - t
    w = (u ** 3, 3 * u * u * t, 3 * u * t * t, t ** 3)
    return (sum(wi * x for wi, x in zip(w, (x0, x1, x2, x3))),
            sum(wi * y for wi, y in zip(w, (y0, y1, y2, y3))))

def roots(a, b, c):
    if abs(a) < 1e-12:
        return [-c / b] if abs(b) > 1e-12 else []
    disc = b * b - 4 * a * c
    if disc < 0:
        return []
    r = math.sqrt(disc)
    return [(-b + r) / (2 * a), (-b - r) / (2 * a)]

def bbox(points):
    xs, ys = [p[0] for p in points], [p[1] for p in points]
    return min(xs), min(ys), max(xs), max(ys)

for name, curve in curves.items():
    ts = [0.0, 1.0]
    for axis in (0, 1):
        p0, p1, p2, p3 = (pt[axis] for pt in curve)
        a = 3 * (-p0 + 3 * p1 - 3 * p2 + p3)
        b = 6 * (p0 - 2 * p1 + p2)
        c = 3 * (p1 - p0)
        ts += [t for t in roots(a, b, c) if 0 < t < 1]
    true = bbox([point(curve, t) for t in ts])
    naive = bbox(curve)
    fmt = lambda box: "(" + ", ".join(f"{v:g}" if v == int(v) else f"{v:.2f}" for v in box) + ")"
    print(f"{name}: naive {fmt(naive)} true {fmt(true)}")

@@exercise Map viewBox coordinates to pixels
Write `fit(viewbox, width, height, mode)` returning the scale(s) and offset that map viewBox coordinates to the viewport for `meet`, `slice` and `none` (with xMidYMid alignment), and `to_screen` for a point. Print where an icon's centre and corner land in several viewports, and how much is cropped with `slice`.
@@starter
icon_box = (0, 0, 24, 24)
banner_box = (0, 0, 200, 100)

def fit(viewbox, width, height, mode):
    raise NotImplementedError
@@expected
viewBox 24x24 in 48x48 meet : scale 2x2, (12, 12)->(24.0, 24.0), (24, 24)->(48.0, 48.0)
viewBox 24x24 in 48x48 slice: scale 2x2, (12, 12)->(24.0, 24.0), (24, 24)->(48.0, 48.0)
viewBox 24x24 in 48x48 none : scale 2x2, (12, 12)->(24.0, 24.0), (24, 24)->(48.0, 48.0)
viewBox 24x24 in 64x32 meet : scale 1.33333x1.33333, (12, 12)->(32.0, 16.0), (24, 24)->(48.0, 32.0)
viewBox 24x24 in 64x32 slice: scale 2.66667x2.66667, (12, 12)->(32.0, 16.0), (24, 24)->(64.0, 48.0), cropped 0x32px
viewBox 24x24 in 64x32 none : scale 2.66667x1.33333, (12, 12)->(32.0, 16.0), (24, 24)->(64.0, 32.0)
viewBox 200x100 in 300x300 meet : scale 1.5x1.5, (100, 50)->(150.0, 150.0), (0, 0)->(0.0, 75.0)
viewBox 200x100 in 300x300 slice: scale 3x3, (100, 50)->(150.0, 150.0), (0, 0)->(-150.0, 0.0), cropped 300x0px
viewBox 200x100 in 300x300 none : scale 1.5x3, (100, 50)->(150.0, 150.0), (0, 0)->(0.0, 0.0)
viewBox 200x100 in 400x100 meet : scale 1x1, (100, 50)->(200.0, 50.0), (0, 0)->(100.0, 0.0)
viewBox 200x100 in 400x100 slice: scale 2x2, (100, 50)->(200.0, 50.0), (0, 0)->(0.0, -50.0), cropped 0x100px
viewBox 200x100 in 400x100 none : scale 2x1, (100, 50)->(200.0, 50.0), (0, 0)->(0.0, 0.0)
@@requires
def fit
def to_screen
min(
max(
@@hint
`sx = width / vw`, `sy = height / vh`. For meet use `s = min(sx, sy)`, slice `s = max(sx, sy)`, both axes the same; none keeps sx and sy. Offsets centre the content: `tx = (width - vw * sx) / 2 - minx * sx` (likewise for y). Cropped width = `max(0, vw * sx - width)`.
@@solution
icon_box = (0, 0, 24, 24)
banner_box = (0, 0, 200, 100)

def fit(viewbox, width, height, mode):
    minx, miny, vw, vh = viewbox
    sx, sy = width / vw, height / vh
    if mode == "meet":
        sx = sy = min(sx, sy)
    elif mode == "slice":
        sx = sy = max(sx, sy)
    tx = (width - vw * sx) / 2 - minx * sx
    ty = (height - vh * sy) / 2 - miny * sy
    return sx, sy, tx, ty

def to_screen(p, transform):
    sx, sy, tx, ty = transform
    return (round(p[0] * sx + tx, 2), round(p[1] * sy + ty, 2))

for box, points, sizes in ((icon_box, [(12, 12), (24, 24)], [(48, 48), (64, 32)]),
                           (banner_box, [(100, 50), (0, 0)], [(300, 300), (400, 100)])):
    for width, height in sizes:
        for mode in ("meet", "slice", "none"):
            t = fit(box, width, height, mode)
            crop_w = max(0, box[2] * t[0] - width)
            crop_h = max(0, box[3] * t[1] - height)
            spots = ", ".join(f"{p}->{to_screen(p, t)}" for p in points)
            crop = f", cropped {crop_w:g}x{crop_h:g}px" if crop_w or crop_h else ""
            print(f"viewBox {box[2]}x{box[3]} in {width}x{height} {mode:<5}: scale {t[0]:g}x{t[1]:g}, {spots}{crop}")

@@exercise Shorten path data
Write `minify(d, precision)` that rounds every number to `precision` decimals, removes trailing zeros and leading `0` before the decimal point, and joins tokens with the fewest spaces (no space before a minus sign or after a command letter). Report the size saving for each path and check the result parses back to the same rounded numbers.
@@starter
import re

paths = {
    "check": "M 4.000000 12.500000 L 9.250001 17.749999 L 20.000000 6.000000",
    "arrow": "M 12.000 4.000 L 12.000 20.000 M 5.500 13.000 L 12.000 19.500 L 18.500 13.000",
    "wave": "M 0.5 12.0 C 3.333333 8.0, 6.666667 8.0, 9.5 12.0 C 12.333333 16.0, 15.666667 16.0, 18.5 12.0",
}

def minify(d, precision=2):
    raise NotImplementedError
@@expected
check: 62 -> 23 chars (63% smaller), same numbers: True
  M4 12.5L9.25 17.75L20 6
arrow: 77 -> 34 chars (56% smaller), same numbers: True
  M12 4L12 20M5.5 13L12 19.5L18.5 13
wave: 93 -> 53 chars (43% smaller), same numbers: True
  M.5 12C3.33 8 6.67 8 9.5 12C12.33 16 15.67 16 18.5 12
@@requires
def minify
re.findall(
round(
@@hint
Tokenise with `re.findall(r"[A-Za-z]|-?\d*\.?\d+", d)`. Format a number with `f"{round(v, precision):.{precision}f}".rstrip("0").rstrip(".")`, turn `-0` into `0`, and strip a leading `0` from `0.5` / `-0.5`. While joining, add a space only between two numbers, and skip it when the next number starts with `-`, or starts with `.` while the previous number has no `.` of its own (otherwise `1.5` followed by `.5` would read as `1.5.5`, which parsers split correctly, but it's clearer to keep the space).
@@solution
import re

paths = {
    "check": "M 4.000000 12.500000 L 9.250001 17.749999 L 20.000000 6.000000",
    "arrow": "M 12.000 4.000 L 12.000 20.000 M 5.500 13.000 L 12.000 19.500 L 18.500 13.000",
    "wave": "M 0.5 12.0 C 3.333333 8.0, 6.666667 8.0, 9.5 12.0 C 12.333333 16.0, 15.666667 16.0, 18.5 12.0",
}
TOKEN = r"[A-Za-z]|-?\d*\.?\d+"

def fmt(v, precision):
    s = f"{round(v, precision):.{precision}f}".rstrip("0").rstrip(".")
    if s in ("-0", ""):
        s = "0"
    if s.startswith("0."):
        s = s[1:]
    elif s.startswith("-0."):
        s = "-" + s[2:]
    return s

def minify(d, precision=2):
    out = ""
    prev = None
    for token in re.findall(TOKEN, d):
        if token.isalpha():
            out += token
            prev = token
            continue
        num = fmt(float(token), precision)
        needs_space = prev is not None and not prev.isalpha() and not num.startswith("-") and not (num.startswith(".") and "." not in prev)
        out += (" " if needs_space else "") + num
        prev = num
    return out

for name, d in paths.items():
    small = minify(d)
    before = [round(float(t), 2) for t in re.findall(TOKEN, d) if not t.isalpha()]
    after = [float(t) for t in re.findall(TOKEN, small) if not t.isalpha()]
    print(f"{name}: {len(d)} -> {len(small)} chars ({1 - len(small) / len(d):.0%} smaller), same numbers: {before == after}")
    print(f"  {small}")
