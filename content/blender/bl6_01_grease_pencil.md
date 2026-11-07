@@lesson bl6-greasepencil Grease Pencil: 2D Animation in a 3D World
@@concepts lists, numbers, functions, loops
@@lang python
@@text
**Grease Pencil** turns Blender into a 2D animation studio: you draw strokes with a tablet or mouse, but the strokes live in 3D space, so a camera can move around hand-drawn art, 2D characters can walk through 3D sets, and drawings can be lit and composited. Blender 4.3 rewrote it as **Grease Pencil v3**, faster and built on the same geometry system as Geometry Nodes. Productions use it for animated series, storyboards, animatics and stylised motion graphics.

## Getting started

- **File → New → 2D Animation** opens a workspace with a Grease Pencil object, a camera looking straight at it, and the Draw mode active.
- Or Shift+A → Grease Pencil → Blank / Stroke / Monkey in any scene.
- Modes: **Draw**, **Edit** (move points), **Sculpt** (push, smooth, thickness, twist strokes), **Weight Paint** (for armatures), **Vertex Paint** (colour strokes).

## Strokes, layers and materials

- A **stroke** is a list of points, each with a position, **pressure** (radius) and **strength** (opacity). Tablet pressure shapes the line.
- **Layers** (like Krita/Photoshop): line art, colour, shadows, background; each with opacity, blend mode, onion skin toggle, lock, and **masks** (a layer limited to another layer's shapes).
- **Materials** define **stroke** (line) and **fill** colours, textures, gradients and holdout (cut through other strokes). Keep a palette of materials: "Line", "Skin", "Skin Shadow", "Hair"...
- **Drawing planes**: draw on the view, on the front (XZ) plane, on the 3D cursor's plane, or **on surfaces** (paint on a 3D mesh).
- **Guides** (rulers, circles, radial) and **stabilizer** (Shift while drawing) give clean lines.
- Brushes: Pencil, Ink Pen, Marker, Airbrush, Fill (bucket), Erase, Tint. Fill needs closed gaps: use **gap closure** in the fill tool settings.

## Animating

- **Frame-by-frame**: each Grease Pencil keyframe is a whole drawing. Draw on frame 1, move to frame 3, draw the next drawing ("on twos" at 24 fps = 12 drawings a second).
- **Onion skinning** shows previous and next drawings in colour so you can draw in-betweens.
- **Dope Sheet → Grease Pencil mode** retimes drawings; **Interpolate** (Ctrl+E in Draw mode) generates in-between drawings automatically from two keyframes with matching strokes; **Interpolate Sequence** fills a range with easing.
- **Transforms and rigs**: animate the object or layers like any 3D object; parent layers to bones of an armature for cut-out style animation (move a 2D arm without redrawing).
- **Time Offset** modifier: holds, loops or reverses drawings (blinking, looping fire).

## Modifiers and effects

| Modifier | Use |
|---|---|
| **Smooth / Simplify / Subdivide** | clean or adjust stroke points |
| **Thickness / Opacity / Tint** | vary line weight and colour over time |
| **Noise** | boiling, hand-drawn wobble |
| **Build** | strokes draw themselves on (handwriting reveals) |
| **Array / Mirror / Lattice / Offset / Envelope** | repeat and deform strokes |
| **Outline** | outline around strokes |
| **Line Art** | generates strokes from 3D geometry: an anime/comic look from 3D scenes |
| **Time Offset** | timing tricks per layer |

**Shader effects** (in older versions) and now the compositor add blur, glow, rim light, shadow, pixelate and wave distortion.

## Line Art: 3D to 2D

Shift+A → Grease Pencil → **Scene Line Art** (or Collection / Object Line Art) traces contours, creases, material borders and intersections from 3D meshes into Grease Pencil strokes. Combined with flat-shaded EEVEE materials and toon shading (Shader to RGB → Color Ramp in EEVEE), this produces the anime/cel-shaded look used in many motion graphics.

## A storyboard and animatic workflow

1. One scene per shot; a Grease Pencil object for sketches; markers in the timeline per shot (M), with cameras bound to markers (Ctrl+B in the timeline).
2. Rough drawings on key poses; play back with sound (add audio in the VSE or the Sequencer track).
3. Refine timing in the Dope Sheet before cleaning lines.
4. Export with Ctrl+F12 to video, or as SVG/PDF frames for print boards (File → Export).

## Strokes as data

A stroke is an ordered list of points. The exercises work on that representation: smoothing points (what the Smooth modifier and stabilizer do), simplifying strokes by removing near-collinear points, a "Build" modifier that reveals a stroke over time by length, and onion skin colours.

@@quiz
What makes Grease Pencil different from a normal 2D drawing app?
* Strokes live in 3D space, so cameras, lights and 3D objects interact with hand-drawn art
- It can only draw straight lines
- It's a separate application
- It renders only in Cycles
? Draw on planes or surfaces, then move the camera around your drawings.

@@quiz
What does onion skinning show?
- The render passes
* Faded previous and next drawings, to help draw in-betweens
- The modifier stack
- UV seams
? Usually tinted (e.g. red before, green after).

@@quiz
Which modifier makes strokes appear to draw themselves on?
- Noise
* Build
- Mirror
- Thickness
? Build reveals strokes over time; great for handwriting effects.

@@quiz
What does Line Art do?
- Draws a grid
* Generates Grease Pencil strokes from 3D geometry's outlines, creases and intersections
- Converts strokes to meshes
- Smooths every stroke
? It's the base of many anime-style 3D renders.

@@quiz
At 24 fps, what does animating "on twos" mean?
- 2 drawings per second
* A new drawing every 2 frames: 12 drawings per second
- 48 drawings per second
- Two layers per drawing
? Common in hand-drawn animation; fast actions go on ones.

@@drill
What's the name of Blender's tool for drawing 2D strokes in 3D space?
= Grease Pencil
? Strokes live on layers like a 2D animation app.

@@flashcards
Grease Pencil :: 2D drawing and animation inside 3D
Grease Pencil v3 :: rewrite in Blender 4.3 on the geometry system
stroke :: list of points with position, pressure, strength
Draw / Edit / Sculpt / Weight Paint / Vertex Paint :: Grease Pencil modes
layer masks :: limit a layer to another layer's shapes
stroke vs fill material :: line colour vs enclosed area
drawing plane :: view, front, cursor, or on surfaces
onion skinning :: show previous/next drawings
on ones / on twos :: new drawing every frame / every second frame
Interpolate (Ctrl+E) :: generate in-between drawings
Build modifier :: strokes draw themselves on
Noise modifier :: hand-drawn wobble ("boil")
Line Art :: strokes traced from 3D geometry
Time Offset modifier :: hold, loop, reverse drawings
camera markers (Ctrl+B) :: switch cameras per shot in the timeline

@@exercise Smooth and simplify a stroke
Points are `(x, y)` tuples. Write `smooth(points, factor, passes=1)`: each pass moves every point except the first and last toward the midpoint of its neighbours: `p + factor * (mid - p)`, using the previous pass's values; rounded to 3 at the end. Write `point_line_distance(p, a, b)`: the distance from point p to the line through a and b (or to a if a == b). Then `simplify(points, tolerance)`: the **Ramer–Douglas–Peucker** algorithm: keep the first and last point; find the point farthest from the line between them; if its distance > tolerance, keep it and simplify each half recursively; otherwise drop all the points in between.
@@starter
import math


def smooth(points, factor, passes=1):
    pass


def point_line_distance(p, a, b):
    pass


def simplify(points, tolerance):
    pass


wobbly = [(0, 0), (1, 0.1), (2, -0.1), (3, 0.05), (4, 2), (5, 4.1), (6, 6)]
print(smooth(wobbly, 0.5))
print(simplify(wobbly, 0.2))
@@tests
check(smooth([(0, 0), (1, 1), (2, 0)], 1.0), [(0, 0), (1.0, 0.0), (2, 0)], "full smoothing flattens a spike")
check(smooth([(0, 0), (1, 1), (2, 0)], 0.5), [(0, 0), (1.0, 0.5), (2, 0)], "half")
check(smooth([(0, 0), (5, 5)], 1.0), [(0, 0), (5, 5)], "endpoints never move")
check(smooth([(0, 0), (1, 1), (2, 0), (3, 1), (4, 0)], 0.5, passes=2)[1], (1.0, 0.375), "two passes")
check(round(point_line_distance((1, 1), (0, 0), (2, 0)), 3), 1.0, "distance to a line")
check(round(point_line_distance((3, 4), (0, 0), (0, 0)), 3), 5.0, "degenerate line")
wobbly = [(0, 0), (1, 0.1), (2, -0.1), (3, 0.05), (4, 2), (5, 4.1), (6, 6)]
check(simplify(wobbly, 0.2), [(0, 0), (3, 0.05), (6, 6)], "an L-ish stroke keeps its corner")
check(simplify([(0, 0), (1, 1), (2, 2), (3, 3)], 0.01), [(0, 0), (3, 3)], "a straight line needs two points")
check(simplify([(0, 0), (1, 5), (2, 0)], 10), [(0, 0), (2, 0)], "a big tolerance removes everything")
@@hint
Distance to a line: `|cross(b - a, p - a)| / |b - a|` with the 2D cross product `dx1 * dy2 - dy1 * dx2`. For RDP, return `left[:-1] + right` so the split point isn't duplicated.
@@solution
import math


def smooth(points, factor, passes=1):
    pts = [tuple(p) for p in points]
    for _ in range(passes):
        new = [pts[0]]
        for prev, p, nxt in zip(pts, pts[1:], pts[2:]):
            mid = ((prev[0] + nxt[0]) / 2, (prev[1] + nxt[1]) / 2)
            new.append((p[0] + factor * (mid[0] - p[0]), p[1] + factor * (mid[1] - p[1])))
        if len(pts) > 1:
            new.append(pts[-1])
        pts = new
    first, last = points[0], points[-1]
    middle = [(round(x, 3) + 0.0, round(y, 3) + 0.0) for x, y in pts[1:-1]]
    return [first] + middle + ([last] if len(points) > 1 else [])


def point_line_distance(p, a, b):
    dx, dy = b[0] - a[0], b[1] - a[1]
    if dx == 0 and dy == 0:
        return math.hypot(p[0] - a[0], p[1] - a[1])
    return abs(dx * (p[1] - a[1]) - dy * (p[0] - a[0])) / math.hypot(dx, dy)


def simplify(points, tolerance):
    if len(points) < 3:
        return list(points)
    a, b = points[0], points[-1]
    index, far = 0, -1.0
    for i in range(1, len(points) - 1):
        d = point_line_distance(points[i], a, b)
        if d > far:
            index, far = i, d
    if far <= tolerance:
        return [a, b]
    left = simplify(points[:index + 1], tolerance)
    right = simplify(points[index:], tolerance)
    return left[:-1] + right


wobbly = [(0, 0), (1, 0.1), (2, -0.1), (3, 0.05), (4, 2), (5, 4.1), (6, 6)]
print(smooth(wobbly, 0.5))
print(simplify(wobbly, 0.2))

@@exercise The Build modifier and onion skins
Write `stroke_length(points)`: the total length of a stroke, rounded to 3. Write `build(points, progress)`: the part of the stroke drawn when `progress` (0–1) of its length has been revealed: the full points up to that distance, plus an interpolated end point (rounded to 3) if the reveal stops inside a segment. `progress <= 0` gives just the first point. Then `onion_opacity(offset, count, falloff)`: for a drawing `offset` frames before or after the current one (offset ≠ 0), return its onion-skin opacity `falloff ** (|offset| - 1)` rounded to 2 when `|offset| <= count`, otherwise 0.
@@starter
import math


def stroke_length(points):
    pass


def build(points, progress):
    pass


def onion_opacity(offset, count, falloff):
    pass


check_mark = [(0, 1), (1, 0), (3, 2)]
print(stroke_length(check_mark), build(check_mark, 0.5), [onion_opacity(o, 3, 0.6) for o in (-4, -3, -2, -1, 1, 2)])
@@tests
check(stroke_length([(0, 0), (3, 4), (3, 10)]), 11.0, "5 + 6")
s = [(0, 0), (3, 4), (3, 10)]
check(build(s, 1.0), [(0, 0), (3, 4), (3, 10)], "fully drawn")
check(build(s, 0.0), [(0, 0)], "not started")
check(build(s, 0.5), [(0, 0), (3, 4), (3.0, 4.5)], "half of 11 = 5.5 along")
check(build(s, 0.2), [(0, 0), (1.32, 1.76)], "inside the first segment")
check(build(s, 5.5 / 11), build(s, 0.5), "same distance")
check(onion_opacity(1, 3, 0.6), 1.0, "nearest drawing")
check(onion_opacity(-3, 3, 0.6), 0.36, "third drawing back")
check(onion_opacity(4, 3, 0.6), 0, "outside the count")
@@hint
Walk the segments with a running total. When `total + seg >= target`, interpolate with `t = (target - total) / seg` and stop. If the reveal lands exactly on a point, don't add a duplicate.
@@solution
import math


def stroke_length(points):
    return round(sum(math.hypot(b[0] - a[0], b[1] - a[1]) for a, b in zip(points, points[1:])), 3)


def build(points, progress):
    if progress <= 0:
        return [points[0]]
    if progress >= 1:
        return list(points)
    target = progress * sum(math.hypot(b[0] - a[0], b[1] - a[1]) for a, b in zip(points, points[1:]))
    out = [points[0]]
    total = 0.0
    for a, b in zip(points, points[1:]):
        seg = math.hypot(b[0] - a[0], b[1] - a[1])
        if total + seg >= target:
            t = (target - total) / seg if seg else 0
            if t >= 1:
                out.append(b)
            elif t > 0:
                out.append((round(a[0] + (b[0] - a[0]) * t, 3), round(a[1] + (b[1] - a[1]) * t, 3)))
            return out
        out.append(b)
        total += seg
    return out


def onion_opacity(offset, count, falloff):
    if abs(offset) > count:
        return 0
    return round(falloff ** (abs(offset) - 1), 2)


check_mark = [(0, 1), (1, 0), (3, 2)]
print(stroke_length(check_mark), build(check_mark, 0.5), [onion_opacity(o, 3, 0.6) for o in (-4, -3, -2, -1, 1, 2)])
