@@lesson gm0-precision Precision Work: Guides, Grids, Measuring, Alignment and Paths
@@concepts numbers, lists, functions
@@lang python
@@text
Freehand editing is fine for photos, but layouts, thumbnails, banners and UI mock-ups need precision: things lined up, evenly spaced, a horizon that's actually level, a curve that's smooth. GIMP has a full set of precision tools that many users never discover. This lesson covers them and the simple geometry behind them.

## Rulers, guides and grids

- **Rulers** (View → Show Rulers, Ctrl+Shift+R) show coordinates in pixels or other units. The pointer position is shown in the status bar.
- **Guides**: drag from a ruler to create one, or use Image → Guides → New Guide (by Percent) for exact positions. Image → Guides → New Guides from Selection puts guides around a selection.
- **Snap to guides** (View → Snap to Guides) makes selections, moves and transforms stick to them. Snap to canvas edges and to the grid exist as well.
- A **grid** (Image → Configure Grid, View → Show Grid) suits pixel art and icon work, e.g. a 1 px grid at high zoom, or an 8 px grid for UI layouts.
- **Safe areas**: social platforms crop thumbnails and banners differently on mobile and desktop. Put guides at the safe area and keep text inside it.

## Measuring and straightening

- The **Measure tool** (Shift+M) shows the distance and angle between two points.
- To level a tilted horizon, draw a measure line along the horizon and press **Straighten** in the tool options: GIMP rotates the layer by the measured angle. Then crop away the transparent corners (or use Clipping: Crop with aspect).
- The angle is atan2(dy, dx). A horizon 2° off level is clearly visible, and even 0.5° can bother careful viewers on wide shots.
- For vertical lines (buildings), use the **Perspective** or **Unified Transform** tool, or the **Straighten** option on a vertical measure line, which GIMP rotates to vertical.

## Aligning and distributing

- The **Alignment tool** (Q) aligns layers relative to the first selected item, the selection, the active layer or the image: left, centre, right, top, middle, bottom.
- **Distribute** spreads layers so their left edges, centres or the **gaps** between them are equal. Equal gaps look right when items have different widths, and equal centres don't.
- Layer → **Layer to Image Size** and Image → **Fit Canvas to Layers** tidy up after moving things around.
- Keep elements on whole pixels: a 1 px line at x = 10.5 is drawn as two half-transparent pixels and looks blurry.

## Paths (vectors)

- The **Paths tool** (B) draws Bézier paths: click for corner points, drag for smooth curves with handles. Ctrl-click to edit, Shift for a new component.
- A path can be:
  - **stroked** (Edit → Stroke Path) with a line or a paint tool, which suits arrows, borders and signatures.
  - **turned into a selection** (Select → From Path, Shift+V), which suits precise cutouts of products.
  - used for **text along a path** (Text → Text along Path).
  - **exported as SVG** (in the Paths dialog), so you can reuse the shape in Inkscape or a web page.
- **Selection to path** works the other way: a rough selection becomes an editable outline.
- Paths are resolution independent: scale the image and the path stays smooth until you stroke it.

## A layout workflow

1. Set the canvas to the target size (e.g. 1280 × 720 for a video thumbnail).
2. Add guides for margins (5% from each edge) and the safe area.
3. Place elements, snap them to guides, then align and distribute.
4. Check alignment at 100% zoom and at the small size it will be viewed at.

@@quiz
How do you make a level horizon from a tilted photo in GIMP?
* Draw a Measure tool line along the horizon and press Straighten
- Use Image → Flatten
- Use the Smudge tool
- Change the canvas size
? Then crop the transparent corners.

@@quiz
Three buttons have different widths. Which distribution makes the spacing look even?
* Equal gaps between them
- Equal left edges
- Equal centres
- Random
? Gaps are what the eye compares.

@@quiz
What does Select → From Path do?
* Turns a closed path into a precise selection
- Exports the path as SVG
- Strokes the path with a brush
- Deletes the path
? Useful for product cutouts.

@@quiz
Why does a 1 px line at x = 10.5 look blurry?
* It falls across two pixels, each half covered
- GIMP can't draw at x = 10.5
- Lines are always blurry
- The grid is too large
? Keep thin elements on whole pixels.

@@quiz
Which feature keeps text inside the area a platform won't crop?
* Guides placed at the safe area, with snapping on
- The Smudge tool
- Indexed mode
- Layer groups
? Check both mobile and desktop crops.

@@quiz
What is an advantage of paths over painted strokes?
* They are resolution independent and stay editable
- They are always filled
- They use less colour
- They only work in Indexed mode
? Stroke them at the end.

@@flashcards
rulers :: Ctrl+Shift+R
guide :: drag from a ruler, or Image → Guides
snap to guides :: View → Snap to Guides
grid :: Image → Configure Grid
Measure tool :: Shift+M, distance and angle
Straighten :: rotate by the measured angle
angle :: atan2(dy, dx)
Alignment tool :: Q
distribute gaps :: equal spacing for different widths
Paths tool :: B, Bézier curves
Select → From Path :: Shift+V
Stroke Path :: paint along a path
Text along Path :: text follows a curve
export path :: SVG for Inkscape or the web
safe area :: region that won't be cropped

@@drill
A measure line goes from (0, 0) to (100, 2). What is the angle in degrees, to one decimal place?
= 1.1
? atan2(2, 100) ≈ 1.15°.

@@drill
Which tool has the keyboard shortcut Q?
= Alignment
= Alignment tool
= Align
? It also distributes.

@@exercise Measure and straighten
Implement the Measure tool's calculations: distance and angle between two points, and the rotation needed to make the line horizontal (or vertical, if the line is closer to vertical). Also compute the crop: after rotating a W × H image by θ, the largest centred rectangle with the same aspect ratio that has no transparent corners.
@@starter
import math

def measure(p1, p2):
    raise NotImplementedError

def straighten(p1, p2):
    raise NotImplementedError

def crop_size(w, h, degrees):
    raise NotImplementedError

for p1, p2 in [((100, 400), (1900, 460)), ((50, 300), (1000, 280)), ((800, 100), (830, 1000))]:
    dist, ang = measure(p1, p2)
    rot = straighten(p1, p2)
    print(f"{p1} -> {p2}: {dist:.1f} px at {ang:.2f} deg, rotate {rot:+.2f} deg, crop to {crop_size(2000, 1200, rot)}")
@@expected
(100, 400) -> (1900, 460): 1801.0 px at 1.91 deg, rotate -1.91 deg, crop to (1895, 1137)
(50, 300) -> (1000, 280): 950.2 px at -1.21 deg, rotate +1.21 deg, crop to (1932, 1159)
(800, 100) -> (830, 1000): 900.5 px at 88.09 deg, rotate +1.91 deg, crop to (1895, 1137)
@@requires
def measure
def straighten
math.atan2(
@@hint
`measure`: `dist = math.hypot(dx, dy)` and `ang = math.degrees(math.atan2(dy, dx))`. `straighten`: if `abs(ang) <= 45` rotate by `-ang`; otherwise the line is nearly vertical, so rotate by `90 - ang` (or `-90 - ang` for negative angles), whichever is smaller in size. `crop_size`: with `t = radians(abs(deg))`, the scale is `min(w / (w*cos t + h*sin t), h / (w*sin t + h*cos t))`, and the crop is `(floor(w*s), floor(h*s))`.
@@solution
import math

def measure(p1, p2):
    dx, dy = p2[0] - p1[0], p2[1] - p1[1]
    return math.hypot(dx, dy), math.degrees(math.atan2(dy, dx))

def straighten(p1, p2):
    _, ang = measure(p1, p2)
    if abs(ang) <= 45:
        return -ang
    return (90 - ang) if ang > 0 else (-90 - ang)

def crop_size(w, h, degrees):
    t = math.radians(abs(degrees))
    s = min(w / (w * math.cos(t) + h * math.sin(t)), h / (w * math.sin(t) + h * math.cos(t)))
    return (math.floor(w * s), math.floor(h * s))

for p1, p2 in [((100, 400), (1900, 460)), ((50, 300), (1000, 280)), ((800, 100), (830, 1000))]:
    dist, ang = measure(p1, p2)
    rot = straighten(p1, p2)
    print(f"{p1} -> {p2}: {dist:.1f} px at {ang:.2f} deg, rotate {rot:+.2f} deg, crop to {crop_size(2000, 1200, rot)}")

@@exercise Align and distribute layers
Layers are `(name, x, y, w, h)`. Implement `align(layers, edge)` for "left", "top" and "middle" (vertical centres) relative to the first layer, and `distribute_gaps(layers)` that keeps the leftmost and rightmost layers in place and moves the others so the horizontal gaps are equal (whole pixels, rounding each position). Print the results.
@@starter
layers = [("logo", 40, 30, 120, 60), ("title", 210, 52, 400, 40), ("button", 700, 41, 160, 48), ("badge", 960, 35, 60, 60)]

def align(layers, edge):
    raise NotImplementedError

def distribute_gaps(layers):
    raise NotImplementedError

def show(title, ls):
    print(title)
    for name, x, y, w, h in ls:
        print(f"  {name:<7} x {x:>4} y {y:>3}  ({w}x{h})")

show("align top", align(layers, "top"))
show("align middle", align(layers, "middle"))
show("distribute gaps", distribute_gaps(layers))
@@expected
align top
  logo    x   40 y  30  (120x60)
  title   x  210 y  30  (400x40)
  button  x  700 y  30  (160x48)
  badge   x  960 y  30  (60x60)
align middle
  logo    x   40 y  30  (120x60)
  title   x  210 y  40  (400x40)
  button  x  700 y  36  (160x48)
  badge   x  960 y  30  (60x60)
distribute gaps
  logo    x   40 y  30  (120x60)
  title   x  240 y  52  (400x40)
  button  x  720 y  41  (160x48)
  badge   x  960 y  35  (60x60)
@@requires
def align
def distribute_gaps
sorted(
@@hint
For `align`, take the first layer as the reference: "left" sets each x to its x; "top" sets each y to its y; "middle" sets `y = ref_y + ref_h / 2 - h / 2` (rounded). For `distribute_gaps`, sort by x, compute `span = right edge of last - left of first`, `gap = (span - sum of widths) / (n - 1)`, then place each layer at the running position, rounding.
@@solution
layers = [("logo", 40, 30, 120, 60), ("title", 210, 52, 400, 40), ("button", 700, 41, 160, 48), ("badge", 960, 35, 60, 60)]

def align(layers, edge):
    _, rx, ry, rw, rh = layers[0]
    out = []
    for name, x, y, w, h in layers:
        if edge == "left":
            x = rx
        elif edge == "middle":
            y = round(ry + rh / 2 - h / 2)
        elif edge == "top":
            y = ry
        out.append((name, x, y, w, h))
    return out

def distribute_gaps(layers):
    ls = sorted(layers, key=lambda l: l[1])
    left = ls[0][1]
    right = ls[-1][1] + ls[-1][3]
    gap = (right - left - sum(l[3] for l in ls)) / (len(ls) - 1)
    out, pos = [], left
    for name, _, y, w, h in ls:
        out.append((name, round(pos), y, w, h))
        pos += w + gap
    return out

def show(title, ls):
    print(title)
    for name, x, y, w, h in ls:
        print(f"  {name:<7} x {x:>4} y {y:>3}  ({w}x{h})")

show("align top", align(layers, "top"))
show("align middle", align(layers, "middle"))
show("distribute gaps", distribute_gaps(layers))
