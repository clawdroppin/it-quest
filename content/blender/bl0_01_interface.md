@@lesson bl0-interface Blender Basics: Interface, Navigation and 3D Space
@@concepts numbers, lists, functions
@@lang python
@@text
**Blender** (blender.org, free and open source for Windows, macOS and Linux) is a full 3D production suite. One program covers modeling, sculpting, materials, rigging, animation, simulations (smoke, fire, fluids, cloth, rigid bodies), 2D animation with Grease Pencil, motion tracking, node compositing and video editing. Studios use it for films, ads, games and motion graphics, and it replaces tools that cost thousands: Maya, Cinema 4D, After Effects for compositing, and more.

This course goes from the interface to VFX shots and motion graphics. The exercises are in Python, Blender's scripting language, and build the maths Blender runs underneath: vectors, transforms, curves, noise, physics and compositing.

## Installing

- Download from blender.org (or Steam, the Microsoft Store, `winget install BlenderFoundation.Blender`, Flatpak/Snap on Linux). The **LTS** releases get two years of fixes; pick one for long projects.
- A GPU helps a lot: **Cycles** renders on NVIDIA (CUDA/OptiX), AMD (HIP), Intel (oneAPI) and Apple (Metal). Set it in **Edit → Preferences → System**.
- Useful preferences: **Interface → Resolution Scale** for high-DPI screens; **Input → Emulate 3 Button Mouse** for laptops (Alt + left drag orbits); **Save & Load → Auto Save**.

## The interface

Blender's window is split into **areas**, each showing an **editor**: 3D Viewport, Outliner, Properties, Timeline, Shader Editor, Graph Editor, Compositor, Video Sequencer, Image Editor, Text Editor... Any area can become any editor via the editor-type menu at its top left. Drag area corners to split or join them.

**Workspaces** (tabs at the top) are saved layouts for a task: Layout, Modeling, Sculpting, UV Editing, Texture Paint, Shading, Animation, Rendering, Compositing, Geometry Nodes, Scripting. Add more with the + tab (2D Animation, VFX → Motion Tracking, Video Editing → Video Editing).

| Editor | Purpose |
|---|---|
| **3D Viewport** | see and edit the scene; toolbar on the left (T), sidebar on the right (N) |
| **Outliner** | the scene tree: collections, objects, their data; visibility toggles |
| **Properties** | tabs for Render, Output, View Layer, Scene, World, Object, Modifiers, Particles, Physics, Constraints, Data, Material |
| **Timeline** | play, frame range, keyframes of selected objects |

## Navigating the viewport

| Action | Mouse / key |
|---|---|
| Orbit | middle mouse drag (or the gizmo at top right) |
| Pan | Shift + middle mouse |
| Zoom | scroll wheel or Ctrl + middle mouse |
| Frame selected | numpad . (View → Frame Selected) |
| Front / right / top | numpad 1 / 3 / 7 (Ctrl for the opposite side) |
| Camera view | numpad 0 |
| Toggle perspective/orthographic | numpad 5 |
| Pie menu for views | ` (backtick / tilde) |
| Local view (isolate) | numpad / |
| Search any command | F3 |

## Objects and transforms

- **Add** objects with Shift+A: meshes, curves, surfaces, metaballs, text, Grease Pencil, armatures, lights, cameras, empties, force fields.
- Select with left click, Shift+click to add, A to select all, Alt+A to deselect, B for box select. The last selected is the **active** object (lighter outline).
- **Transform**: **G** grab/move, **R** rotate, **S** scale. Then type an axis (X, Y, Z; press twice for local axes; Shift+Z for "not Z") and a number: `G Z 2 Enter` moves 2 m up. Right-click or Esc cancels.
- Ctrl+A **applies** transforms (bakes scale into the mesh: do this before modifiers and physics, since many expect scale 1).
- Alt+G / Alt+R / Alt+S clear location / rotation / scale.
- **Pivot point** (the "." pie): median, 3D cursor, individual origins, active element. **Snapping** (Shift+Tab) to grid, vertices, faces...
- **3D cursor**: Shift+right-click places it; new objects appear there. Shift+S is the snap pie.
- **Origin**: every object's pivot; Object → Set Origin.
- **Parenting**: Ctrl+P makes the active object the parent; children follow its transform.
- Duplicate with Shift+D (a copy) or **Alt+D** (a **linked duplicate** sharing the same mesh data).

Blender is **Z-up**, right-handed, in metres. Rotations are **Euler XYZ** by default (also quaternion and axis-angle for rigs).

## Collections, scenes and files

- **Collections** group objects in the Outliner (like folders); they can be hidden, excluded from a view layer, or instanced elsewhere.
- A `.blend` file can hold several **scenes**; **File → Link/Append** pulls objects or materials from other files (linking keeps them in sync with the source).
- **File → External Data → Pack Resources** embeds textures so a file travels safely.
- Save often (Ctrl+S); Blender keeps `.blend1` backups and auto-saves to the temp folder.

## Coordinates in Python

Scripting (Scripting workspace) uses the `bpy` module and `mathutils.Vector`, `Matrix`, `Euler` and `Quaternion`:

```text
import bpy
from mathutils import Vector
cube = bpy.data.objects["Cube"]
cube.location = Vector((0, 0, 2))
cube.rotation_euler.z = 0.785   # radians
cube.scale *= 2
```

The exercises use plain tuples so they run anywhere, but the maths is identical.

@@quiz
Which key moves a selected object, and how do you move it exactly 2 metres up?
- M, then type 2
* G, then Z, 2, Enter
- Ctrl+M
- Shift+D
? G grabs, an axis key constrains, a number sets the distance.

@@quiz
Which way is up in Blender?
- Y
* Z
- X
- −Y
? Blender is Z-up; many game engines are Y-up, so exporters convert.

@@quiz
What does Alt+D create?
- A deleted copy
* A linked duplicate that shares the original's mesh data
- A new scene
- An armature
? Edit one, and all linked duplicates change.

@@quiz
Why apply scale (Ctrl+A → Scale) before adding bevels or physics?
- It makes the file smaller
* Modifiers and simulations work in object space and expect a scale of 1, so a non-uniform scale distorts them
- It's required to render
- It deletes the object's history
? A bevel on a cube scaled (1, 1, 5) is stretched unless scale is applied.

@@quiz
How do you find any command whose shortcut you've forgotten?
- Ctrl+F
* F3 (search)
- Tab
- Numpad 0
? Menu search also shows the shortcut.

@@flashcards
editor / area / workspace :: one tool / a region of the window / a saved layout
Outliner :: scene tree of collections and objects
Properties editor :: tabs for render, world, object, modifiers, physics, material...
G / R / S :: grab, rotate, scale
X, Y, Z (twice for local) :: constrain a transform to an axis
Ctrl+A :: apply transforms
Shift+A :: add menu
Shift+D / Alt+D :: duplicate / linked duplicate
numpad 0 / 1 / 3 / 7 / 5 :: camera, front, right, top, persp/ortho
F3 :: search for any command
3D cursor :: where new objects appear; a pivot option
origin :: an object's own pivot point
Ctrl+P :: parent to the active object
collection :: folder of objects in the Outliner
Z-up, metres :: Blender's coordinate convention

@@exercise Vector maths for 3D
Represent vectors as tuples `(x, y, z)`. Write `add(a, b)`, `sub(a, b)`, `scale(v, s)`, `dot(a, b)`, `cross(a, b)`, `length(v)` and `normalize(v)` (each component rounded to 4 decimals; a zero vector stays zero). Then `distance(a, b)` and `angle_deg(a, b)`: the angle between two vectors in degrees, rounded to 2 decimals.
@@starter
import math


def add(a, b):
    pass


def sub(a, b):
    pass


def scale(v, s):
    pass


def dot(a, b):
    pass


def cross(a, b):
    pass


def length(v):
    pass


def normalize(v):
    pass


def distance(a, b):
    pass


def angle_deg(a, b):
    pass


print(cross((1, 0, 0), (0, 1, 0)), normalize((3, 0, 4)), angle_deg((1, 0, 0), (1, 1, 0)))
@@tests
check(add((1, 2, 3), (4, 5, 6)), (5, 7, 9), "add")
check(sub((4, 5, 6), (1, 2, 3)), (3, 3, 3), "subtract")
check(scale((1, -2, 3), 2), (2, -4, 6), "scale")
check(dot((1, 2, 3), (4, -5, 6)), 12, "dot product")
check(cross((1, 0, 0), (0, 1, 0)), (0, 0, 1), "X cross Y is Z (right-handed)")
check(length((3, 0, 4)), 5.0, "length")
check(normalize((3, 0, 4)), (0.6, 0.0, 0.8), "normalize")
check(normalize((0, 0, 0)), (0, 0, 0), "zero vector")
check(distance((1, 1, 1), (4, 5, 1)), 5.0, "distance")
check(angle_deg((1, 0, 0), (1, 1, 0)), 45.0, "45 degrees")
check(angle_deg((0, 0, 1), (0, 0, -1)), 180.0, "opposite")
@@hint
`cross(a, b) = (a1*b2 - a2*b1, a2*b0 - a0*b2, a0*b1 - a1*b0)`. The angle is `acos(dot / (|a| |b|))`; clamp the ratio to −1..1 to avoid rounding errors.
@@solution
import math


def add(a, b):
    return tuple(x + y for x, y in zip(a, b))


def sub(a, b):
    return tuple(x - y for x, y in zip(a, b))


def scale(v, s):
    return tuple(x * s for x in v)


def dot(a, b):
    return sum(x * y for x, y in zip(a, b))


def cross(a, b):
    return (a[1] * b[2] - a[2] * b[1], a[2] * b[0] - a[0] * b[2], a[0] * b[1] - a[1] * b[0])


def length(v):
    return math.sqrt(dot(v, v))


def normalize(v):
    n = length(v)
    if n == 0:
        return v
    return tuple(round(x / n, 4) for x in v)


def distance(a, b):
    return length(sub(a, b))


def angle_deg(a, b):
    c = dot(a, b) / (length(a) * length(b))
    return round(math.degrees(math.acos(max(-1.0, min(1.0, c)))), 2)


print(cross((1, 0, 0), (0, 1, 0)), normalize((3, 0, 4)), angle_deg((1, 0, 0), (1, 1, 0)))

@@exercise Transforms and parenting
An object's transform is location, rotation around Z (degrees) and uniform scale. Write `rotate_z(p, deg)` rotating a point around the Z axis (rounded to 4 decimals). Write `to_world(p, loc, rot_z, s)` that takes a point in the object's local space and returns it in world space: scale, then rotate, then translate (Blender's order), rounded to 4 decimals. Then `chain(p, transforms)` applies a list of parent transforms from the **innermost child outwards** (each item `(loc, rot_z, s)`), the way parenting works.
@@starter
import math


def rotate_z(p, deg):
    pass


def to_world(p, loc, rot_z, s):
    pass


def chain(p, transforms):
    pass


# A wheel's bolt, on a wheel parented to a car
print(chain((0.3, 0, 0), [((1, 0, 0.4), 90, 1), ((10, 5, 0), 180, 2)]))
@@tests
check(rotate_z((1, 0, 0), 90), (0.0, 1.0, 0), "90 degrees")
check(rotate_z((1, 2, 3), 180), (-1.0, -2.0, 3), "Z is unchanged")
check(to_world((1, 0, 0), (0, 0, 0), 0, 3), (3.0, 0.0, 0), "scale")
check(to_world((1, 0, 0), (5, 0, 1), 90, 2), (5.0, 2.0, 1), "scale, rotate, translate")
check(chain((1, 0, 0), []), (1, 0, 0), "no parents")
check(chain((0.3, 0, 0), [((1, 0, 0.4), 90, 1), ((10, 5, 0), 180, 2)]), (8.0, 4.4, 0.8), "a bolt on a wheel on a car")
@@hint
Rotation: `x' = x cos θ − y sin θ`, `y' = x sin θ + y cos θ`, z unchanged; `θ = math.radians(deg)`. Round with `round(v, 4) + 0.0` to turn −0.0 into 0.0 if you like. `chain` is a loop calling `to_world` with each transform in turn.
@@solution
import math


def rotate_z(p, deg):
    t = math.radians(deg)
    x, y, z = p
    return (round(x * math.cos(t) - y * math.sin(t), 4) + 0.0,
            round(x * math.sin(t) + y * math.cos(t), 4) + 0.0, z)


def to_world(p, loc, rot_z, s):
    scaled = (p[0] * s, p[1] * s, p[2] * s)
    r = rotate_z(scaled, rot_z)
    return (round(r[0] + loc[0], 4), round(r[1] + loc[1], 4), round(r[2] + loc[2], 4))


def chain(p, transforms):
    for loc, rot, s in transforms:
        p = to_world(p, loc, rot, s)
    return p


# A wheel's bolt, on a wheel parented to a car
print(chain((0.3, 0, 0), [((1, 0, 0.4), 90, 1), ((10, 5, 0), 180, 2)]))
