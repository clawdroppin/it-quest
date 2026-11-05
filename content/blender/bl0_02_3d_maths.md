@@lesson bl0-maths 3D Maths for Blender: Vectors, Matrices, Parenting and Rotations
@@lang python
@@text
You can use Blender for years without writing a matrix, but the moment you script with `bpy`, debug a rig that flips, or wonder why rotating 90° on X then Y differs from Y then X, the maths shows through. This lesson covers the essentials: vectors, transformation matrices and parenting, Euler angles and gimbal lock, and quaternions. Blender's Python module `mathutils` implements all of them.

## Vectors

- A vector `(x, y, z)` is a position or a direction. Blender is **Z-up**, right-handed; 1 unit = 1 metre by default.
- **Length** `√(x² + y² + z²)`; **normalise** to length 1 for directions.
- **Dot product** `a·b = |a||b|cos θ`: 1 for same direction, 0 for perpendicular, −1 opposite. Used for facing ratios, lighting (`N·L`) and backface tests.
- **Cross product** `a × b`: perpendicular to both; its length is the parallelogram area. Face normals come from the cross product of two edges.

## Matrices and parenting

- An object's transform is a 4 × 4 **matrix** combining location, rotation and scale: `matrix_world` in bpy.
- Applying it: `world_point = matrix_world @ local_point` (`@` is matrix multiplication in Python).
- **Parenting**: `child.matrix_world = parent.matrix_world @ child.matrix_parent_inverse @ child.matrix_basis`. The parent inverse is what keeps the child from jumping when you parent with Ctrl+P.
- Order matters: matrix multiplication isn't commutative. Blender applies scale, then rotation, then translation (T · R · S).
- *Apply → All Transforms* (Ctrl+A) bakes the matrix into the mesh so the object's scale is 1; do this before modifiers like Bevel and Solidify or physics that depend on scale.

## Rotations

- **Euler angles** (X, Y, Z in a chosen order, default XYZ): intuitive, but they suffer **gimbal lock**: when the middle axis rotates 90°, the first and last axes line up and a degree of freedom disappears. They also interpolate oddly for large rotations.
- **Quaternions** `(w, x, y, z)`: four numbers representing an axis and angle (`w = cos(θ/2)`, `xyz = axis · sin(θ/2)`). No gimbal lock; **slerp** (spherical linear interpolation) gives smooth, shortest-path rotation between orientations. Blender bones default to quaternions.
- **Axis-angle**: an axis and an angle; easy to read, useful for single-axis spins.
- Change the rotation mode per object or bone in the N panel; the F-curves change too.

@@quiz
What does a dot product of 0 between two unit vectors mean?
- They point the same way
* They're perpendicular
- They're opposite
- One is zero length
? cos 90° = 0.

@@quiz
How do you compute a face normal from two edges?
- Dot product
* Cross product (then normalise)
- Add them
- Matrix inverse
? It's perpendicular to both edges.

@@quiz
What is gimbal lock?
* Two Euler rotation axes line up, losing one degree of freedom
- A locked transform channel
- A quaternion error
- A modifier setting
? Happens when the middle axis reaches 90°.

@@quiz
Why does a child not jump when parented with Ctrl+P?
- Blender copies the parent's location into it
* The parent inverse matrix compensates for the parent's transform at parenting time
- Parenting only affects rotation
- Children are always in world space
? matrix_parent_inverse.

@@quiz
Which rotation representation interpolates smoothly along the shortest path?
- XYZ Euler
* Quaternion with slerp
- Scale
- Delta transforms
? No gimbal lock either.

@@flashcards
Blender up axis :: Z
dot product :: |a||b|cos θ
cross product :: perpendicular vector
normalise :: length 1
matrix_world :: object to world transform
@ :: matrix multiplication in Python
TRS order :: translate · rotate · scale
parent inverse :: stops the jump on parenting
Ctrl+A :: apply transforms
Euler :: three angles in an order
gimbal lock :: aligned axes
quaternion :: w = cos(θ/2), xyz = axis · sin(θ/2)
slerp :: spherical interpolation
mathutils :: Blender's vector and matrix module

@@drill
What is the dot product of (1, 0, 0) and (0, 1, 0)?
= 0
? Perpendicular.

@@drill
Which Python operator multiplies matrices in Blender's mathutils?
= @
? * multiplies element-wise.

@@exercise Parent and child transforms
Without libraries, build 4 × 4 matrices for translation, rotation about Z and uniform scale, compose them as T · R · S, and compute world positions for a small hierarchy (a turntable with an arm with a lamp), including the lamp's world position when the turntable rotates. Then show that rotate-then-translate differs from translate-then-rotate.
@@starter
import math

def translate(x, y, z):
    return [[1, 0, 0, x], [0, 1, 0, y], [0, 0, 1, z], [0, 0, 0, 1]]

def matmul(a, b):
    raise NotImplementedError
@@expected
turntable   0°: arm base (2.0, 0.0, 1.0), lamp (3.5, 0.0, 1.5), lamp tip (4.0, 0.0, 1.5)
turntable  90°: arm base (2.0, 0.0, 1.0), lamp (2.0, 1.5, 1.5), lamp tip (2.0, 2.0, 1.5)
turntable 135°: arm base (2.0, 0.0, 1.0), lamp (0.939, 1.061, 1.5), lamp tip (0.586, 1.414, 1.5)
T @ R moves (1,0,0) to (3.0, 1.0, 0.0) but R @ T moves it to (0.0, 4.0, 0.0)
@@requires
def matmul
def rotate_z
math.cos(
@@hint
`matmul`: `[[sum(a[i][k] * b[k][j] for k in range(4)) for j in range(4)] for i in range(4)]`. `rotate_z(deg)` has cos/−sin in the first row and sin/cos in the second. Apply a matrix to a point by appending 1 and multiplying. A child's world matrix is `parent_world @ child_local`.
@@solution
import math

def translate(x, y, z):
    return [[1, 0, 0, x], [0, 1, 0, y], [0, 0, 1, z], [0, 0, 0, 1]]

def rotate_z(deg):
    c, s = math.cos(math.radians(deg)), math.sin(math.radians(deg))
    return [[c, -s, 0, 0], [s, c, 0, 0], [0, 0, 1, 0], [0, 0, 0, 1]]

def scale(k):
    return [[k, 0, 0, 0], [0, k, 0, 0], [0, 0, k, 0], [0, 0, 0, 1]]

def matmul(a, b):
    return [[sum(a[i][k] * b[k][j] for k in range(4)) for j in range(4)] for i in range(4)]

def apply(m, p):
    v = list(p) + [1]
    return tuple(round(sum(m[i][k] * v[k] for k in range(4)), 3) + 0.0 for i in range(3))

def trs(t, deg, k):
    return matmul(matmul(translate(*t), rotate_z(deg)), scale(k))

for spin in (0, 90, 135):
    table = trs((2, 0, 0), spin, 1)
    arm = matmul(table, trs((0, 0, 1), 0, 1))
    lamp = matmul(arm, trs((1.5, 0, 0.5), 0, 0.5))
    print(f"turntable {spin:>3}°: arm base {apply(arm, (0, 0, 0))}, lamp {apply(lamp, (0, 0, 0))}, lamp tip {apply(lamp, (1, 0, 0))}")

a = matmul(translate(3, 0, 0), rotate_z(90))
b = matmul(rotate_z(90), translate(3, 0, 0))
print("T @ R moves (1,0,0) to", apply(a, (1, 0, 0)), "but R @ T moves it to", apply(b, (1, 0, 0)))

@@exercise Euler, gimbal lock and slerp
Convert axis-angle rotations to quaternions, multiply quaternions to combine rotations, rotate a vector with a quaternion, and compare linear interpolation of Euler angles with slerp between two orientations. Then show gimbal lock: with Y at 90°, changing X and changing Z by the same amount produce the same orientation.
@@starter
import math

def quat(axis, deg):
    raise NotImplementedError
@@expected
90° about Z: (0.707, 0.0, 0.0, 0.707) turns (1,0,0) into (0.0, 1.0, 0.0)
X 90° then Y 90° sends (0,1,0) to (1.0, 0.0, 0.0)
Y 90° then X 90° sends (0,1,0) to (0.0, 0.0, 1.0)
t=0: euler lerp (1.0, 0.0, 0.0)  slerp (1.0, 0.0, 0.0)
t=0.5: euler lerp (-0.087, 0.996, 0.0)  slerp (0.015, -0.172, 0.985)
t=1: euler lerp (-0.985, -0.174, 0.0)  slerp (-0.985, -0.174, 0.0)
gimbal lock: X+30 with Y=90 -> (0.683, 0.183, 0.683, -0.183) ; Z-30 with Y=90 -> (0.683, 0.183, 0.683, -0.183) ; same orientation: True
@@requires
def quat
def slerp
math.acos(
@@hint
Normalise the axis; `w = cos(θ/2)`, `(x, y, z) = axis · sin(θ/2)`. Hamilton product for multiplication. Rotate v with `q · (0, v) · q*`. Slerp: `θ = acos(dot)` (flip one quaternion if the dot is negative), weights `sin((1−t)θ)/sin θ` and `sin(tθ)/sin θ`. Euler XYZ to quaternion = `qz · qy · qx`.
@@solution
import math

def quat(axis, deg):
    n = math.sqrt(sum(a * a for a in axis))
    h = math.radians(deg) / 2
    return (math.cos(h),) + tuple(a / n * math.sin(h) for a in axis)

def mul(a, b):
    w1, x1, y1, z1 = a
    w2, x2, y2, z2 = b
    return (w1 * w2 - x1 * x2 - y1 * y2 - z1 * z2,
            w1 * x2 + x1 * w2 + y1 * z2 - z1 * y2,
            w1 * y2 - x1 * z2 + y1 * w2 + z1 * x2,
            w1 * z2 + x1 * y2 - y1 * x2 + z1 * w2)

def rotate(q, v):
    conj = (q[0], -q[1], -q[2], -q[3])
    return tuple(round(c, 3) + 0.0 for c in mul(mul(q, (0,) + tuple(v)), conj)[1:])

def euler_xyz(x, y, z):
    return mul(mul(quat((0, 0, 1), z), quat((0, 1, 0), y)), quat((1, 0, 0), x))

def slerp(a, b, t):
    dot = sum(p * q for p, q in zip(a, b))
    if dot < 0:
        b, dot = tuple(-q for q in b), -dot
    theta = math.acos(min(1.0, dot))
    if theta < 1e-6:
        return a
    wa, wb = math.sin((1 - t) * theta) / math.sin(theta), math.sin(t * theta) / math.sin(theta)
    return tuple(wa * p + wb * q for p, q in zip(a, b))

q = quat((0, 0, 1), 90)
print("90° about Z:", tuple(round(c, 3) for c in q), "turns (1,0,0) into", rotate(q, (1, 0, 0)))
both = mul(quat((0, 1, 0), 90), quat((1, 0, 0), 90))
print("X 90° then Y 90° sends (0,1,0) to", rotate(both, (0, 1, 0)))
other = mul(quat((1, 0, 0), 90), quat((0, 1, 0), 90))
print("Y 90° then X 90° sends (0,1,0) to", rotate(other, (0, 1, 0)))

start, end = (0, 0, 0), (170, 0, 190)
for t in (0, 0.5, 1):
    e = tuple(s + (f - s) * t for s, f in zip(start, end))
    by_euler = rotate(euler_xyz(*e), (1, 0, 0))
    by_slerp = rotate(slerp(euler_xyz(*start), euler_xyz(*end), t), (1, 0, 0))
    print(f"t={t}: euler lerp {by_euler}  slerp {by_slerp}")

a = tuple(round(c, 3) for c in euler_xyz(30, 90, 0))
b = tuple(round(c, 3) for c in euler_xyz(0, 90, -30))
print("gimbal lock: X+30 with Y=90 ->", a, "; Z-30 with Y=90 ->", b, "; same orientation:", a == b)
