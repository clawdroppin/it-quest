@@lesson bl1-mesh-data Inside a Mesh: Vertices, Faces, Normals and Clean Topology
@@lang python
@@text
A mesh is just lists: vertex positions, edges between vertex indexes, and faces as loops of vertex indexes. Understanding that data explains most modelling problems: shading artefacts from flipped normals, holes that break boolean and 3D printing, subdivision that pinches around triangles, and polygon counts that explode. This lesson looks at mesh data the way `bpy` and the Mesh Analysis tools see it.

## Mesh data

- `mesh.vertices[i].co` positions; `mesh.edges` pairs of indexes; `mesh.polygons[j].vertices` the loop of indexes for each face.
- Faces are **tris**, **quads** or **n-gons** (5+ sides). Quads subdivide and deform best; triangles are fine on flat, rigid areas and for game engines (everything becomes triangles at render time); n-gons are fine on flat surfaces that won't deform or subdivide.
- **Winding order**: vertices listed counter-clockwise when seen from the front give a normal pointing at you (right-hand rule).

## Normals

- A **face normal** is the normalised cross product of two edges (for n-gons, Newell's method sums over all edges).
- **Flipped normals** (some faces facing inwards) cause dark patches, wrong backface culling and broken booleans. Show them with *Overlays → Face Orientation* (blue = outside, red = inside); fix with *Mesh → Normals → Recalculate Outside* (Shift+N).
- **Smooth shading** interpolates vertex normals across faces; **Auto Smooth** / *Shade Auto Smooth* keeps sharp edges above an angle. Custom normals (Weighted Normal modifier) help hard-surface models.

## Manifold meshes

A mesh is **manifold** (watertight) when every edge is shared by exactly **two** faces and the faces are consistently oriented. Problems:
- **Boundary edges** (1 face): holes, open borders.
- **Non-manifold edges** (3+ faces): internal faces, fins.
- **Loose** vertices or edges, **duplicate** vertices (*Merge by Distance*), zero-area faces.
Select them with *Select → Select All by Trait → Non Manifold*. 3D printing, booleans and many simulations need manifold meshes. The **3D-Print Toolbox** add-on checks all of this.

For a closed manifold mesh with no holes through it, **Euler's formula** holds: V − E + F = 2 (for each hole like a torus, subtract 2).

## Polygon budgets

Subdivision Surface multiplies faces: each level turns a quad into 4 quads, and an n-sided face into n quads at the first level. Three levels on a 10,000-quad model = 640,000 quads. Use the viewport level low and the render level higher, or Adaptive Subdivision in Cycles.

@@quiz
What is a non-manifold edge?
* An edge shared by three or more faces
- An edge shared by exactly two faces
- A sharp edge
- A seam
? Boundary edges (one face) are also problems.

@@quiz
In Face Orientation overlay, what does red mean?
- Selected faces
* You're seeing the back (inside) of the face
- N-gons
- UV seams
? Recalculate Outside to fix.

@@quiz
A closed cube has 8 vertices and 6 faces. How many edges?
- 8
* 12
- 14
- 24
? V − E + F = 2.

@@quiz
One level of subdivision turns a pentagon into how many quads?
- 4
* 5
- 10
- 1
? An n-gon becomes n quads.

@@quiz
Why prefer quads in areas that deform?
- They render faster
* They subdivide cleanly and bend predictably with edge loops
- Triangles can't be UV-unwrapped
- n-gons are illegal
? Edge loops follow the form.

@@flashcards
tri / quad / n-gon :: 3 / 4 / 5+ sides
winding order :: counter-clockwise = front
face normal :: normalised cross product
Face Orientation :: blue outside, red inside
Shift+N :: recalculate normals
smooth shading :: interpolated vertex normals
manifold :: every edge has two faces
boundary edge :: one face
non-manifold edge :: three or more faces
Merge by Distance :: remove duplicate vertices
Euler's formula :: V − E + F = 2
subdivision level :: quads × 4
n-gon subdivided :: n quads
3D-Print Toolbox :: mesh checks

@@drill
For a closed manifold mesh without holes, what is V − E + F?
= 2
? Euler characteristic.

@@drill
Which shortcut recalculates normals in Edit Mode?
= Shift+N
= shift n
= shift+n
? Ctrl+Shift+N recalculates inside.

@@exercise Normals and flipped faces
Compute each face's normal with Newell's method (works for tris, quads and n-gons), find faces whose normals point towards the mesh centre (flipped), and print counts of tris, quads and n-gons. The mesh is a cube with one face listed in the wrong winding order and one side split into two triangles.
@@starter
import math

verts = [(-1, -1, -1), (1, -1, -1), (1, 1, -1), (-1, 1, -1), (-1, -1, 1), (1, -1, 1), (1, 1, 1), (-1, 1, 1)]
faces = {
    "bottom": [0, 3, 2, 1],
    "top": [4, 5, 6, 7],
    "front": [0, 1, 5, 4],
    "back": [3, 7, 6, 2],
    "left": [0, 3, 7, 4],
    "right-a": [1, 2, 6],
    "right-b": [1, 6, 5],
}

def newell(face):
    raise NotImplementedError
@@expected
bottom   normal (0.0, 0.0, -1.0)  ok
top      normal (0.0, 0.0, 1.0)  ok
front    normal (0.0, -1.0, 0.0)  ok
back     normal (0.0, 1.0, 0.0)  ok
left     normal (1.0, 0.0, 0.0)  FLIPPED: recalculate outside
right-a  normal (1.0, 0.0, 0.0)  ok
right-b  normal (1.0, 0.0, 0.0)  ok
{'tris': 2, 'quads': 5, 'n-gons': 0}
@@requires
def newell
math.sqrt(
centre
@@hint
Newell: for each edge (current, next) add `(y1 − y2)(z1 + z2)` to nx, `(z1 − z2)(x1 + x2)` to ny and `(x1 − x2)(y1 + y2)` to nz; then normalise. A face is flipped when the dot product of its normal with (face centre − mesh centre) is negative.
@@solution
import math

verts = [(-1, -1, -1), (1, -1, -1), (1, 1, -1), (-1, 1, -1), (-1, -1, 1), (1, -1, 1), (1, 1, 1), (-1, 1, 1)]
faces = {
    "bottom": [0, 3, 2, 1],
    "top": [4, 5, 6, 7],
    "front": [0, 1, 5, 4],
    "back": [3, 7, 6, 2],
    "left": [0, 3, 7, 4],
    "right-a": [1, 2, 6],
    "right-b": [1, 6, 5],
}

def newell(face):
    n = [0.0, 0.0, 0.0]
    for i, a in enumerate(face):
        x1, y1, z1 = verts[a]
        x2, y2, z2 = verts[face[(i + 1) % len(face)]]
        n[0] += (y1 - y2) * (z1 + z2)
        n[1] += (z1 - z2) * (x1 + x2)
        n[2] += (x1 - x2) * (y1 + y2)
    length = math.sqrt(sum(c * c for c in n))
    return tuple(round(c / length, 3) + 0.0 for c in n)

centre = tuple(sum(v[i] for v in verts) / len(verts) for i in range(3))
kinds = {"tris": 0, "quads": 0, "n-gons": 0}
for name, face in faces.items():
    kinds["tris" if len(face) == 3 else "quads" if len(face) == 4 else "n-gons"] += 1
    normal = newell(face)
    fc = [sum(verts[i][k] for i in face) / len(face) for k in range(3)]
    outward = sum(n * (c - m) for n, c, m in zip(normal, fc, centre)) > 0
    print(f"{name:<8} normal {normal}  {'ok' if outward else 'FLIPPED: recalculate outside'}")
print(kinds)

@@exercise Check a mesh for printing
Given vertices and faces, count edges by how many faces use them, report boundary and non-manifold edges, loose vertices and duplicate vertices (closer than 0.0001), check Euler's formula, and estimate face counts after 1–3 subdivision levels.
@@starter
from collections import Counter
from itertools import combinations

meshes = {
    "closed cube": ([(0,0,0),(1,0,0),(1,1,0),(0,1,0),(0,0,1),(1,0,1),(1,1,1),(0,1,1)],
                    [[0,3,2,1],[4,5,6,7],[0,1,5,4],[3,7,6,2],[0,4,7,3],[1,2,6,5]]),
    "open box": ([(0,0,0),(1,0,0),(1,1,0),(0,1,0),(0,0,1),(1,0,1),(1,1,1),(0,1,1),(5,5,5)],
                 [[0,3,2,1],[0,1,5,4],[3,7,6,2],[0,4,7,3],[1,2,6,5]]),
    "fin and duplicate": ([(0,0,0),(1,0,0),(1,1,0),(0,1,0),(0.5,0.5,1),(1,0,0.00001)],
                          [[0,1,2,3],[0,1,4],[1,0,4,2]]),
}

def edge_use(faces):
    raise NotImplementedError
@@expected
closed cube: V=8 E=12 F=6, V - E + F = 2
  boundary none; non-manifold none; loose none; duplicates none
  printable: True; subdivided faces: [24, 96, 384]
open box: V=8 E=12 F=5, V - E + F = 1
  boundary [(4, 5), (4, 7), (5, 6), (6, 7)]; non-manifold none; loose [8]; duplicates none
  printable: False; subdivided faces: [20, 80, 320]
fin and duplicate: V=5 E=7 F=3, V - E + F = 1
  boundary [(0, 3), (1, 4), (2, 3), (2, 4)]; non-manifold [(0, 1)]; loose [5]; duplicates [(1, 5)]
  printable: False; subdivided faces: [11, 44, 176]
@@requires
def edge_use
Counter(
V - E + F
@@hint
Edges are sorted vertex-index pairs of consecutive face vertices (`tuple(sorted((f[i], f[(i + 1) % len(f)])))`); count how many faces use each. Loose vertices are indexes in no face. Duplicates: `combinations(range(len(verts)), 2)` with distance below the tolerance. After one subdivision level a face with n sides becomes n quads; each further level multiplies by 4.
@@solution
from collections import Counter
from itertools import combinations

meshes = {
    "closed cube": ([(0,0,0),(1,0,0),(1,1,0),(0,1,0),(0,0,1),(1,0,1),(1,1,1),(0,1,1)],
                    [[0,3,2,1],[4,5,6,7],[0,1,5,4],[3,7,6,2],[0,4,7,3],[1,2,6,5]]),
    "open box": ([(0,0,0),(1,0,0),(1,1,0),(0,1,0),(0,0,1),(1,0,1),(1,1,1),(0,1,1),(5,5,5)],
                 [[0,3,2,1],[0,1,5,4],[3,7,6,2],[0,4,7,3],[1,2,6,5]]),
    "fin and duplicate": ([(0,0,0),(1,0,0),(1,1,0),(0,1,0),(0.5,0.5,1),(1,0,0.00001)],
                          [[0,1,2,3],[0,1,4],[1,0,4,2]]),
}

def edge_use(faces):
    return Counter(tuple(sorted((f[i], f[(i + 1) % len(f)]))) for f in faces for i in range(len(f)))

for name, (verts, faces) in meshes.items():
    use = edge_use(faces)
    boundary = sorted(e for e, n in use.items() if n == 1)
    non_manifold = sorted(e for e, n in use.items() if n > 2)
    used = {i for f in faces for i in f}
    loose = [i for i in range(len(verts)) if i not in used]
    dupes = [(a, b) for a, b in combinations(range(len(verts)), 2)
             if sum((p - q) ** 2 for p, q in zip(verts[a], verts[b])) ** 0.5 < 0.0001]
    V, E, F = len(used), len(use), len(faces)
    level1 = sum(len(f) for f in faces)
    print(f"{name}: V={V} E={E} F={F}, V - E + F = {V - E + F}")
    print(f"  boundary {boundary or 'none'}; non-manifold {non_manifold or 'none'}; loose {loose or 'none'}; duplicates {dupes or 'none'}")
    print(f"  printable: {not (boundary or non_manifold or loose or dupes)}; subdivided faces: {[level1 * 4 ** k for k in range(3)]}")
