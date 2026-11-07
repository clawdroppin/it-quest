@@lesson bl4-geonodes Geometry Nodes and Procedural Motion Graphics
@@concepts lists, functions, numbers, loops
@@lang python
@@text
**Geometry Nodes** is Blender's procedural system: a node graph that creates and modifies geometry, scatters instances, and animates anything over time. It's how motion designers build abstract loops, data-driven visuals and endlessly variable scenes, much like Houdini or Cinema 4D's MoGraph.

## Getting started

Geometry Nodes workspace → select an object → **New** (adds a Geometry Nodes modifier). The graph runs from **Group Input** (the object's original geometry) to **Group Output**. Shift+A adds nodes; Ctrl+Shift+click (Node Wrangler) previews a node in the viewport, and the **Viewer** node with the **Spreadsheet** shows every attribute value.

## Core ideas

- **Geometry** types: Mesh, Curve, Point Cloud, Instances, Volume. Nodes convert between them (Mesh to Points, Curve to Mesh, Points to Volume, Realize Instances).
- **Attributes**: data stored per **domain** (point, edge, face, face corner, spline, instance): position, normal, index, ID, custom ones like `scale` or `color`.
- **Fields**: the diamond-shaped sockets. A field is a function evaluated for every element: `Position`, `Index`, `Normal`, `Random Value`, a Noise Texture... The node that uses it decides where it's evaluated. Example: Set Position with Offset = Normal × Noise moves each vertex along its normal by noise.
- **Instances**: thousands of lightweight copies of one object (trees, cubes, particles) that render fast. **Realize Instances** turns them into real geometry when needed.

## Essential nodes

| Category | Nodes |
|---|---|
| Mesh primitives | Grid, Cube, UV Sphere, Ico Sphere, Cylinder, Cone, Mesh Line, Mesh Circle |
| Points and instances | **Distribute Points on Faces** (random or Poisson disk), **Instance on Points**, Rotate/Scale/Translate Instances |
| Geometry operations | Set Position, Extrude Mesh, Subdivide, Join Geometry, Transform Geometry, Merge by Distance, Delete Geometry, Separate Geometry, Mesh Boolean, Dual Mesh, Triangulate |
| Curves | Curve Line, Curve Circle, Spiral, Resample Curve, Curve to Mesh (sweeps a profile), Trim Curve, Fillet Curve, Set Curve Radius |
| Utilities | Math, Vector Math, Map Range, Float Curve, Compare, Switch, Random Value, Align Euler to Vector / Align Rotation to Vector, Rotate Vector |
| Input | Position, Index, Normal, ID, Object Info, Collection Info, Scene Time, Named Attribute |
| Textures | Noise, Voronoi, Wave, Gradient, Image |
| Simulation | **Simulation Zone** (state carried frame to frame), **Repeat Zone** (loops), **For Each Element Zone** |
| Materials | Set Material, Store Named Attribute (pass colours to shaders) |

## Classic motion graphics setups

1. **Grid of cubes animated by a wave**: Grid → Mesh to Points → Instance on Points (a Cube) → Scale Instances by `sin(distance from centre − time)`. Use **Scene Time** for animation; it's the C4D "Effector" look.
2. **Scatter**: Distribute Points on Faces of a ground plane (density from a texture or a painted vertex group), Instance on Points with a Collection Info of rocks/trees, Random Value for rotation and scale. Pick instances randomly with "Pick Instance".
3. **Growing curves**: Curve to Mesh a circle profile along a spiral, Trim Curve end animated 0 → 1: vines, cables, logo reveals.
4. **Text effects**: String to Curves with your text, Fill Curve, Extrude Mesh; animate per-letter offsets using Index.
5. **Audio-reactive**: bake sound to F-curves (Graph Editor → Channel → Bake Sound) and drive node values.
6. **Simulation Zone**: particles that move by their velocity each frame, trails, growth, flocking. The Simulation Zone replaced many old particle-system tricks.

## Node groups and assets

Select nodes → Ctrl+G makes a **node group** with its own inputs (exposed on the modifier panel as sliders). Mark it as an **asset** (right-click → Mark as Asset) to drag it from the Asset Browser into any file. Blender ships **Essentials** assets such as hair node groups. Expose parameters like "Count", "Seed", "Wave Speed" so the setup becomes a reusable tool.

## Performance

- Keep instances as instances until you need real geometry.
- Use **Poisson disk** distribution for natural spacing, with a sensible density.
- Bake heavy simulations and node trees (Bake button on the modifier or in the Physics tab) so playback is real-time.
- The **Timings** overlay in the node editor shows which node is slow.

## Fields in Python terms

A field is "a function of the element": `position.z = noise(position) * 0.2` evaluated for every vertex. The exercises do exactly that with lists: a grid of points, a radial wave field, and a seeded scatter with random rotation and scale.

@@quiz
What is a field in Geometry Nodes?
- A large flat plane
* A function evaluated per element (point, face, instance), shown with diamond sockets
- A render pass
- A texture image
? The node consuming the field decides the domain it's evaluated on.

@@quiz
Why use Instance on Points instead of real copies?
* Instances are lightweight references to one object, so thousands render and play back fast
- Instances can't be rendered
- Real copies are the only way to randomise rotation
- Instances are required for UV mapping
? Realize Instances only when you must edit the result as one mesh.

@@quiz
Which node gives you animation over time in a node tree?
- Index
* Scene Time
- Normal
- Join Geometry
? Use its seconds or frame output in math nodes.

@@quiz
Which zone carries state from one frame to the next (e.g. particles moving by their velocity)?
- Repeat Zone
* Simulation Zone
- For Each Element Zone
- Frame node
? Repeat zones loop within a single frame.

@@quiz
How do you make a node setup reusable with sliders on the modifier?
- Copy and paste the nodes
* Group the nodes (Ctrl+G), expose inputs on the group, and mark it as an asset
- Apply the modifier
- Bake it
? Exposed group inputs show up on the modifier panel.

@@drill
Which node places instances on the points of a geometry?
= Instance on Points
? Combine with Distribute Points on Faces to scatter objects.

@@flashcards
Geometry Nodes :: Blender's procedural node system for geometry
Group Input / Output :: original geometry in, result out
field :: per-element function (diamond socket)
attribute :: per-element data on a domain
domain :: point, edge, face, face corner, spline, instance
instances :: lightweight copies; Realize Instances makes them real
Distribute Points on Faces :: scatter points (random or Poisson disk)
Instance on Points :: put an object on every point
Set Position :: move elements by a field offset
Scene Time :: seconds and frame for animation
Curve to Mesh :: sweep a profile along a curve
Simulation Zone :: state carried across frames
Repeat Zone :: loop inside one evaluation
Viewer node + Spreadsheet :: inspect attribute values
node group + asset :: reusable procedural tool

@@exercise A wave field over a grid
Write `grid(nx, ny, spacing)` returning points `(x, y, 0)` centred on the origin (x goes from `-(nx-1)*spacing/2` upwards), ordered row by row (y outer loop, x inner), rounded to 3. Write `wave(points, time, speed, wavelength, amplitude)`: a radial wave field, setting each point's z to `amplitude * sin(2π (d / wavelength − time * speed))` where `d` is the distance from the origin in XY, rounded to 3. Then `scales(points, base, amount)`: instance scales `base + z * amount` for each point, rounded to 3 (the cube-grid effector look).
@@starter
import math


def grid(nx, ny, spacing):
    pass


def wave(points, time, speed, wavelength, amplitude):
    pass


def scales(points, base, amount):
    pass


g = grid(3, 3, 1.0)
w = wave(g, 0.25, 1.0, 4.0, 0.5)
print(g)
print(w)
print(scales(w, 1.0, 0.4))
@@tests
check(grid(2, 1, 2.0), [(-1.0, 0.0, 0), (1.0, 0.0, 0)], "two points")
g = grid(3, 3, 1.0)
check(len(g), 9, "nine points")
check(g[0], (-1.0, -1.0, 0), "first corner")
check(g[4], (0.0, 0.0, 0), "centre")
check(g[5], (1.0, 0.0, 0), "row by row")
w = wave(g, 0.25, 1.0, 4.0, 0.5)
check(w[4], (0.0, 0.0, -0.5), "centre at time 0.25: trough")
check(w[5], (1.0, 0.0, 0.0), "distance 1 = a quarter wavelength later")
check(wave([(0, 0, 0)], 0, 1, 4, 0.5), [(0, 0, 0.0)], "time 0 at centre")
check(scales([(0, 0, -0.5), (1, 0, 0.5)], 1.0, 0.4), [0.8, 1.2], "scales")
@@hint
Start x at `-(nx - 1) * spacing / 2` and add `i * spacing`. Keep x and y unchanged in `wave` and replace z. Add `0.0` after rounding to avoid `-0.0`.
@@solution
import math


def grid(nx, ny, spacing):
    x0 = -(nx - 1) * spacing / 2
    y0 = -(ny - 1) * spacing / 2
    return [(round(x0 + i * spacing, 3) + 0.0, round(y0 + j * spacing, 3) + 0.0, 0)
            for j in range(ny) for i in range(nx)]


def wave(points, time, speed, wavelength, amplitude):
    out = []
    for x, y, _ in points:
        d = math.hypot(x, y)
        z = amplitude * math.sin(2 * math.pi * (d / wavelength - time * speed))
        out.append((x, y, round(z, 3) + 0.0))
    return out


def scales(points, base, amount):
    return [round(base + p[2] * amount, 3) for p in points]


g = grid(3, 3, 1.0)
w = wave(g, 0.25, 1.0, 4.0, 0.5)
print(g)
print(w)
print(scales(w, 1.0, 0.4))

@@exercise A seeded scatter
Geometry Nodes' Random Value is deterministic: the same seed and ID give the same number. Write `rand(seed, i)` returning a float in [0, 1) from a simple hash: `h = (seed * 73856093 ^ i * 19349663) % 1000003`, then `h / 1000003`. Write `scatter(count, size, seed, min_dist)`: generate candidate points `(x, y)` with `x = rand(seed, 2k) * size`, `y = rand(seed, 2k + 1) * size` for k = 0, 1, 2, ... up to `count * 30` candidates, keeping a candidate only if it's at least `min_dist` from every kept point (a simple Poisson disk), until `count` points are kept. Round coordinates to 3. Then `instance_params(points, seed)` returns for each point `(rotation_deg, scale)`: rotation `rand(seed + 1, i) * 360` rounded to 1, scale `0.8 + rand(seed + 2, i) * 0.4` rounded to 3.
@@starter
import math


def rand(seed, i):
    pass


def scatter(count, size, seed, min_dist):
    pass


def instance_params(points, seed):
    pass


pts = scatter(5, 10, 42, 2.0)
print(pts)
print(instance_params(pts, 42))
@@tests
check(rand(1, 0), 73856093 % 1000003 / 1000003, "the hash")
check(rand(7, 3) == rand(7, 3), True, "deterministic")
check(0 <= rand(99, 12345) < 1, True, "in range")
pts = scatter(5, 10, 42, 2.0)
check(len(pts), 5, "five points")
check(all(0 <= x <= 10 and 0 <= y <= 10 for x, y in pts), True, "inside the area")
ok = all(math.hypot(a[0] - b[0], a[1] - b[1]) >= 2.0 - 0.002 for i, a in enumerate(pts) for b in pts[i + 1:])
check(ok, True, "every pair at least min_dist apart")
check(scatter(5, 10, 42, 2.0) == pts, True, "same seed, same scatter")
check(scatter(3, 10, 7, 0) == scatter(3, 10, 8, 0), False, "another seed, another scatter")
check(len(scatter(50, 1, 1, 5)), 1, "gives up when nothing more fits")
params = instance_params(pts, 42)
check(len(params), 5, "one per point")
check(all(0 <= r < 360 and 0.8 <= s <= 1.2 for r, s in params), True, "ranges")
@@hint
`^` is XOR in Python and binds looser than `*`, so write it as `(seed * 73856093) ^ (i * 19349663)`. Loop `for k in range(count * 30)`, stop when `len(kept) == count`.
@@solution
import math


def rand(seed, i):
    h = ((seed * 73856093) ^ (i * 19349663)) % 1000003
    return h / 1000003


def scatter(count, size, seed, min_dist):
    kept = []
    for k in range(count * 30):
        if len(kept) == count:
            break
        p = (round(rand(seed, 2 * k) * size, 3), round(rand(seed, 2 * k + 1) * size, 3))
        if all(math.hypot(p[0] - q[0], p[1] - q[1]) >= min_dist for q in kept):
            kept.append(p)
    return kept


def instance_params(points, seed):
    return [(round(rand(seed + 1, i) * 360, 1), round(0.8 + rand(seed + 2, i) * 0.4, 3))
            for i in range(len(points))]


pts = scatter(5, 10, 42, 2.0)
print(pts)
print(instance_params(pts, 42))
