@@lesson bl1-modeling Modeling: Edit Mode, Topology and Modifiers
@@concepts numbers, lists, functions
@@lang python
@@text
Most 3D work starts with **polygon modeling**: meshes made of **vertices**, **edges** and **faces**. Blender's tools make it fast, and **modifiers** add non-destructive steps (bevels, mirrors, arrays, subdivision) that you can tweak at any time.

## Edit Mode

Tab toggles **Object Mode** ↔ **Edit Mode**. In Edit Mode, 1 / 2 / 3 switch to vertex / edge / face selection.

| Tool | Key | Use |
|---|---|---|
| **Extrude** | E | pull new geometry out of faces or edges |
| **Inset** | I | a smaller face inside a face (panels, windows) |
| **Bevel** | Ctrl+B (vertices: Ctrl+Shift+B) | round edges; scroll for segments |
| **Loop cut** | Ctrl+R | add an edge loop around a mesh |
| **Knife** | K | cut new edges by hand |
| **Merge** | M | merge vertices (at centre, by distance...) |
| **Fill / Grid fill** | F / Face → Grid Fill | make faces between edges |
| **Bridge edge loops** | Edge menu | connect two loops with faces |
| **Spin / Screw** | toolbar | lathe a profile around an axis |
| **Select loop / ring** | Alt+click / Ctrl+Alt+click | select edge loops and rings |
| **Select linked** | L / Ctrl+L | select a connected piece |
| **Proportional editing** | O | soft falloff around the selection |
| **Separate / Join** | P / Ctrl+J | split into objects / combine objects |

## Good topology

- **Quads** (four-sided faces) subdivide and deform cleanly; avoid long thin triangles and **n-gons** on curved or deforming surfaces (they're fine on flat hard surfaces).
- **Edge loops** should follow the form: around eyes and mouths, along a limb's joints.
- **Poles** (vertices where 3 or 5+ edges meet) are necessary but should sit where they don't pinch.
- Keep the poly count appropriate: a hero prop can have hundreds of thousands of polygons with subdivision; a game asset might need under 10,000 triangles.
- **Normals** must point outwards (Overlays → Face Orientation: blue is outside, red is inside; Shift+N recalculates).
- Use **Merge by Distance** to remove duplicate vertices after imports.

## Modifiers

Properties → **Modifiers** (wrench) stack non-destructive operations applied top to bottom:

| Modifier | What it does |
|---|---|
| **Mirror** | model half; the other half mirrors (enable Clipping) |
| **Subdivision Surface** | smooths by splitting each face (Ctrl+1/2/3 sets levels); support edges control sharpness |
| **Bevel** | rounded edges everywhere, limit by angle or weight; great for hard surface realism |
| **Solidify** | gives thickness to a surface |
| **Array** | repeats with relative/constant offset or an object offset (spirals!) |
| **Boolean** | union, difference, intersect with another object |
| **Weighted Normal** | fixes shading on bevelled hard surfaces |
| **Curve / Lattice / Simple Deform / Shrinkwrap** | bend along curves, deform with cages, twist/taper, project onto surfaces |
| **Displace** | moves vertices by a texture (terrain) |
| **Decimate / Remesh** | reduce polygons / rebuild uniform topology |
| **Screw / Skin / Wireframe** | lathe, tubes from edges, wireframe geometry |
| **Geometry Nodes** | anything at all (Lesson 4) |

Order matters: Mirror → Bevel → Subdivision gives a different result from Subdivision → Mirror. **Apply** (Ctrl+A over the modifier) when you need to edit the result.

## Hard surface and sculpting

- **Hard surface** (machines, props, UI-like objects): boolean cutters (the free Bool Tool add-on), bevel modifier with angle limit, weighted normals, **shade auto smooth** (Object → Shade Auto Smooth).
- **Sculpt Mode**: digital clay. Brushes: Draw, Clay Strips, Grab, Smooth (Shift), Crease, Inflate, Snake Hook. **Dyntopo** adds detail where you sculpt; **Voxel Remesh** (Ctrl+R in sculpt mode) rebuilds even topology. Multires modifier holds detail levels; then **retopologise** to a clean low-poly mesh and bake normal maps.

## UV unwrapping

Textures need **UV coordinates**: the mesh flattened onto a 2D square.

1. Mark **seams** (Edge → Mark Seam) where you'd cut a paper model.
2. Select all → U → **Unwrap** (or Smart UV Project for quick hard surface work).
3. Check stretching with a checker texture; pack islands (UV → Pack Islands).

## Curves and text

**Curves** (Bézier and NURBS) make cables, logos and paths; give them depth (Geometry → Bevel) or sweep a profile. **Text objects** become 3D logos with extrude and bevel; Object → Convert → Mesh when you need to edit them. Importing SVG (from the Vector UI course!) turns logos and icons into curves you can extrude: a classic motion graphics start.

## Mesh counts

Subdivision multiplies faces: each quad becomes 4 per level, so level 2 has 16× the faces. Knowing that keeps scenes fast; the first exercise computes it, the second builds an array modifier's offsets.

@@quiz
Which key inserts an edge loop?
- E
* Ctrl+R
- K
- Ctrl+B
? E extrudes, K is the knife, Ctrl+B bevels.

@@quiz
Why are quads preferred for subdivided and animated meshes?
* They subdivide into even grids and deform predictably
- They render faster than triangles in every engine
- Blender can't display triangles
- They use less memory than any other face
? Game engines triangulate anyway, but modeling in quads keeps control.

@@quiz
In Face Orientation overlay, what does red mean?
- A selected face
* The face's normal points inward (flip it)
- A UV seam
- A face with too many vertices
? Shift+N recalculates normals outside.

@@quiz
A cube with a Subdivision Surface modifier at level 2: how many faces?
- 12
- 24
* 96
- 48
? 6 faces × 4² = 96.

@@quiz
Why does modifier order matter?
- It doesn't
* Each modifier works on the result of the one above, so Mirror → Bevel → Subdivision differs from other orders
- Blender sorts them automatically
- Only the last modifier is applied
? The stack runs top to bottom.

@@drill
Which key switches between Object Mode and Edit Mode?
= Tab
? Edit Mode edits vertices, edges and faces.

@@flashcards
Tab :: toggle Object and Edit Mode
1 / 2 / 3 (edit mode) :: vertex / edge / face select
E / I / Ctrl+B / Ctrl+R / K :: extrude, inset, bevel, loop cut, knife
Alt+click :: select an edge loop
quad topology :: four-sided faces for clean subdivision and deformation
n-gon :: face with more than four sides
pole :: vertex with 3 or 5+ edges
Shift+N :: recalculate normals
modifier stack :: non-destructive operations, top to bottom
Subdivision Surface :: smooths; each level ×4 faces
Mirror (with clipping) :: symmetrical modeling
Array :: repeated copies with offsets
Boolean :: union, difference, intersect
Weighted Normal :: clean shading on bevelled hard surfaces
Voxel Remesh / Dyntopo :: sculpting topology tools
UV unwrap / seams :: flatten a mesh for textures

@@exercise Mesh statistics
A mesh is a list of vertices and a list of faces (each a tuple of vertex indices). Write `stats(faces)` returning `{"faces": n, "tris": t, "quads": q, "ngons": g, "triangles": total}` where `triangles` is the count after triangulation (a face with k vertices makes k − 2 triangles). Write `subdivided_faces(faces, levels)`: Catmull-Clark turns a face with k vertices into k quads on the first level, and every later level multiplies by 4. Write `edges(faces)` returning the number of unique edges (an edge is an unordered pair of vertex indices).
@@starter
def stats(faces):
    pass


def subdivided_faces(faces, levels):
    pass


def edges(faces):
    pass


cube = [(0, 1, 2, 3), (4, 5, 6, 7), (0, 1, 5, 4), (1, 2, 6, 5), (2, 3, 7, 6), (3, 0, 4, 7)]
print(stats(cube), subdivided_faces(cube, 2), edges(cube))
@@tests
cube = [(0, 1, 2, 3), (4, 5, 6, 7), (0, 1, 5, 4), (1, 2, 6, 5), (2, 3, 7, 6), (3, 0, 4, 7)]
check(stats(cube), {"faces": 6, "tris": 0, "quads": 6, "ngons": 0, "triangles": 12}, "a cube")
check(edges(cube), 12, "a cube has 12 edges")
check(subdivided_faces(cube, 0), 6, "level 0")
check(subdivided_faces(cube, 2), 96, "level 2")
mixed = [(0, 1, 2), (0, 2, 3, 4, 5)]
check(stats(mixed), {"faces": 2, "tris": 1, "quads": 0, "ngons": 1, "triangles": 4}, "a triangle and a pentagon")
check(subdivided_faces(mixed, 1), 8, "3 + 5 quads after one level")
check(subdivided_faces(mixed, 3), 128, "then ×4 per level")
check(edges(mixed), 7, "shared edge counted once")
@@hint
For edges, loop over each face's consecutive pairs including last→first: `zip(f, f[1:] + f[:1])`, and add `tuple(sorted(pair))` to a set.
@@solution
def stats(faces):
    tris = sum(1 for f in faces if len(f) == 3)
    quads = sum(1 for f in faces if len(f) == 4)
    ngons = sum(1 for f in faces if len(f) > 4)
    return {"faces": len(faces), "tris": tris, "quads": quads, "ngons": ngons,
            "triangles": sum(len(f) - 2 for f in faces)}


def subdivided_faces(faces, levels):
    if levels == 0:
        return len(faces)
    return sum(len(f) for f in faces) * 4 ** (levels - 1)


def edges(faces):
    seen = set()
    for f in faces:
        f = list(f)
        for a, b in zip(f, f[1:] + f[:1]):
            seen.add(tuple(sorted((a, b))))
    return len(seen)


cube = [(0, 1, 2, 3), (4, 5, 6, 7), (0, 1, 5, 4), (1, 2, 6, 5), (2, 3, 7, 6), (3, 0, 4, 7)]
print(stats(cube), subdivided_faces(cube, 2), edges(cube))

@@exercise An Array modifier
Recreate the Array modifier. `array_offsets(count, relative, size, constant)` returns the location of each copy along one axis: copy `i` sits at `i * (relative * size + constant)` where `size` is the object's dimension (relative offset is a fraction of the size, constant offset is in metres), rounded to 3 decimals. `fit_count(length, relative, size, constant)` is "Fit Length": the largest count whose last copy's far edge (`offset + size`) stays within `length` (at least 1). Then `spiral(count, radius, step_deg, rise)` mimics an array with an **object offset** that rotates and lifts each copy: copy `i` is at angle `i * step_deg`, position `(radius cos, radius sin, i * rise)`, rounded to 3.
@@starter
import math


def array_offsets(count, relative, size, constant):
    pass


def fit_count(length, relative, size, constant):
    pass


def spiral(count, radius, step_deg, rise):
    pass


print(array_offsets(4, 1.0, 2.0, 0.5), fit_count(10, 1.0, 2.0, 0.5), spiral(3, 2, 90, 0.25))
@@tests
check(array_offsets(4, 1.0, 2.0, 0.0), [0.0, 2.0, 4.0, 6.0], "touching copies")
check(array_offsets(4, 1.0, 2.0, 0.5), [0.0, 2.5, 5.0, 7.5], "with a gap")
check(array_offsets(3, 0.5, 1.0, 0.0), [0.0, 0.5, 1.0], "overlapping")
check(fit_count(10, 1.0, 2.0, 0.5), 4, "four planks fit in 10 m")
check(fit_count(10, 1.0, 2.0, 0.0), 5, "exactly five")
check(fit_count(1, 1.0, 2.0, 0.0), 1, "at least one")
check(spiral(3, 2, 90, 0.25), [(2.0, 0.0, 0.0), (0.0, 2.0, 0.25), (-2.0, 0.0, 0.5)], "a spiral staircase")
@@hint
For fit_count, increase the count while `(count) * step + size <= length` where `step = relative * size + constant`. Add `0.0` to rounded values to avoid `-0.0`.
@@solution
import math


def array_offsets(count, relative, size, constant):
    step = relative * size + constant
    return [round(i * step, 3) for i in range(count)]


def fit_count(length, relative, size, constant):
    step = relative * size + constant
    count = 1
    while count * step + size <= length + 1e-9:
        count += 1
    return count


def spiral(count, radius, step_deg, rise):
    out = []
    for i in range(count):
        a = math.radians(i * step_deg)
        out.append((round(radius * math.cos(a), 3) + 0.0, round(radius * math.sin(a), 3) + 0.0, round(i * rise, 3)))
    return out


print(array_offsets(4, 1.0, 2.0, 0.5), fit_count(10, 1.0, 2.0, 0.5), spiral(3, 2, 90, 0.25))
