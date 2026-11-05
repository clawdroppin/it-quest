@@lesson ua1-bones-kinematics Bones in Depth: Forward and Inverse Kinematics, Skeleton Deformation and Rendering Settings
@@lang python
@@text
Synfig's **Skeleton layer** and **Skeleton Deformation layer** let you animate a character's arm or a UI mascot's waving hand by rotating bones instead of moving every vertex. Under the hood, this is the same maths used by every 2D and 3D rig. Knowing it explains why bones behave the way they do and lets you script rigs yourself.

## Bones and forward kinematics (FK)

- A bone has an **origin**, a **length** and an **angle** relative to its **parent**. Rotating a parent rotates all its children with it.
- **Forward kinematics**: given each bone's local angle, compute where the joints end up. Walk down the chain, adding the angles: the world angle of bone *i* is the sum of the local angles from the root to *i*, and each joint is the previous joint plus `length × (cos θ, sin θ)`.
- In Synfig: create a Skeleton layer, add bones with the Bone tool, set the parent in the Params panel, and animate the **angle** parameter in Animation mode. Waypoints on angles interpolate smoothly (TCB, ease, linear).

## Inverse kinematics (IK)

- **IK**: given a target for the end of the chain (the hand), find the angles. With two bones (upper arm *a*, forearm *b*) and a target at distance *d*, the law of cosines gives the elbow angle:
  - cos(elbow interior) = (a² + b² − d²) ÷ (2ab)
  - shoulder angle = atan2(target) − angle between the upper arm and the target line, where cos(that angle) = (a² + d² − b²) ÷ (2ad)
- **Two solutions** (elbow up or elbow down). Rigs choose one with a **bend direction** or **pole target**.
- If *d* > *a* + *b*, the target is unreachable. Clamp *d*, so the arm points straight at it.
- Synfig has no built-in IK solver, so animators pose with FK. Tools like Blender, Spine and Rive add IK constraints, and the maths here is what they compute.

## Skeleton deformation

- **Skeleton Deformation layer**: a mesh of points is bound to bones, and each point follows its bones with **weights** (0–1). A point weighted 0.5/0.5 between the upper arm and the forearm bends smoothly at the elbow.
- Weight painting that's too hard (0 or 1 only) causes sharp creases. Too soft, and the limb looks like rubber.

## Rendering settings that matter for UI motion

- **Image size and resolution**: render at the display size × device pixel ratio (2x for HiDPI).
- **Frame rate**: 24 fps for illustrative motion, 30 or 60 fps for UI feedback.
- **Formats**: PNG sequence (lossless, with alpha) → sprite sheet, APNG or video. Export to **Lottie** with the `lottie` exporter plug-in for web and apps (check feature support: not every Synfig layer maps to Lottie).
- **Quality** (1 = best, 10 = draft) and **anti-aliasing** settings: preview at draft, render final at quality 3 or better.

@@quiz
A parent bone rotates by 30°. What happens to its child bone?
* It rotates with the parent, keeping its local angle
- Nothing
- It rotates −30° to compensate
- It detaches
? World angle = sum of local angles.

@@quiz
A two-bone arm has bones of length 3 and 2. The target is 6 units away. What should a solver do?
- Bend the elbow fully
* Treat it as unreachable and point the straightened arm at the target
- Stretch the bones
- Give an error
? The maximum reach is a + b = 5.

@@quiz
Why does two-bone IK have two solutions?
* The elbow can bend either way (up or down) and still reach the target
- Because of floating-point error
- Because bones have two ends
- There is only one
? Choose one with a pole target or bend direction.

@@quiz
A point is weighted 1.0 to the forearm and 0 to the upper arm, right at the elbow. What happens when the elbow bends?
- It bends smoothly
* A sharp crease appears at the joint
- The point disappears
- Nothing moves
? Blend the weights near joints.

@@quiz
At what size should you render a 100 × 100 px icon animation for 2x displays?
- 100 × 100
* 200 × 200
- 50 × 50
- 400 × 400
? Display size × device pixel ratio.

@@flashcards
bone :: origin, length, angle relative to the parent
forward kinematics :: angles → joint positions
world angle :: sum of local angles from the root
inverse kinematics :: target → angles
law of cosines :: c² = a² + b² − 2ab cos C
unreachable target :: d > a + b, so straighten the arm
pole target :: chooses the bend direction
Skeleton Deformation :: points follow bones by weight
weights :: 0–1 influence per bone
hard weights :: creases at joints
render size :: display size × DPR
PNG sequence :: lossless with alpha

@@drill
A two-bone arm with lengths 4 and 3 is fully stretched. How far does it reach?
= 7
? a + b.

@@drill
Bone 1 has a local angle of 30° and bone 2 a local angle of 45°. What is bone 2's world angle in degrees?
= 75
? Add the angles down the chain.

@@exercise Forward kinematics for a bone chain
Compute the joint positions of a bone chain from local angles and lengths, for several keyframes of a waving-hand animation, and interpolate the angles between keyframes with ease-in-out so the hand moves smoothly.
@@concepts numbers, loops, functions
@@starter
import math
bones = [("upper arm", 40), ("forearm", 30), ("hand", 12)]
keys = {0: [90, 0, 0], 12: [60, 45, 20], 24: [60, 80, -20]}   # frame: local angles in degrees

def fk(angles, root=(0.0, 0.0)):
    raise NotImplementedError
@@expected
frame  0: angles [90.0, 0.0, 0.0], elbow (0.0, 40.0), fingertip (0.0, 82.0)
frame  6: angles [75.0, 22.5, 10.0], elbow (10.4, 38.6), fingertip (2.8, 79.8)
frame 12: angles [60.0, 45.0, 20.0], elbow (20.0, 34.6), fingertip (5.4, 73.4)
frame 18: angles [60.0, 62.5, 0.0], elbow (20.0, 34.6), fingertip (-2.6, 70.1)
frame 24: angles [60.0, 80.0, -20.0], elbow (20.0, 34.6), fingertip (-9.0, 64.3)
@@requires
def fk
math.cos
math.radians
@@hint
Keep a running world angle: add each bone's local angle, then step `x += length * math.cos(math.radians(world))` and the same with sin for y. Return the list of joint positions, root included. To interpolate, find the surrounding keyframes, compute t, ease it with `t * t * (3 - 2 * t)`, and blend each angle.
@@solution
import math
bones = [("upper arm", 40), ("forearm", 30), ("hand", 12)]
keys = {0: [90, 0, 0], 12: [60, 45, 20], 24: [60, 80, -20]}   # frame: local angles in degrees

def fk(angles, root=(0.0, 0.0)):
    x, y = root
    world, joints = 0.0, [(x, y)]
    for (_, length), local in zip(bones, angles):
        world += local
        x += length * math.cos(math.radians(world))
        y += length * math.sin(math.radians(world))
        joints.append((x, y))
    return joints

def angles_at(frame):
    frames = sorted(keys)
    for f0, f1 in zip(frames, frames[1:]):
        if f0 <= frame <= f1:
            t = (frame - f0) / (f1 - f0)
            t = t * t * (3 - 2 * t)
            return [a + (b - a) * t for a, b in zip(keys[f0], keys[f1])]
    return keys[frames[-1]]

for frame in (0, 6, 12, 18, 24):
    angles = angles_at(frame)
    tip = fk(angles)[-1]
    elbow = fk(angles)[1]
    print(f"frame {frame:>2}: angles {[round(a, 1) for a in angles]}, elbow ({elbow[0]:.1f}, {elbow[1]:.1f}), fingertip ({tip[0]:.1f}, {tip[1]:.1f})")

@@exercise Two-bone inverse kinematics
Solve two-bone IK with the law of cosines: given the bone lengths and a target, find the shoulder and elbow angles for both bend directions, clamp unreachable targets, and check each solution by running it back through forward kinematics.
@@concepts numbers, functions, conditions
@@starter
import math
A, B = 40.0, 30.0
targets = [(50, 20), (0, 70), (30, -30), (90, 0), (5, 5)]

def solve(tx, ty, bend=1):
    raise NotImplementedError
@@expected
target ( 50,  20) bend + shoulder   -11.5 deg, elbow    80.4 deg, tip error  0.00
target ( 50,  20) bend - shoulder    55.1 deg, elbow   -80.4 deg, tip error  0.00
target (  0,  70) bend + shoulder    90.0 deg, elbow     0.0 deg, tip error  0.00
target (  0,  70) bend - shoulder    90.0 deg, elbow     0.0 deg, tip error  0.00
target ( 30, -30) bend + shoulder   -87.6 deg, elbow   107.0 deg, tip error  0.00
target ( 30, -30) bend - shoulder    -2.4 deg, elbow  -107.0 deg, tip error  0.00
target ( 90,   0) bend + shoulder     0.0 deg, elbow     0.0 deg, tip error 20.00 (clamped: out of reach)
target ( 90,   0) bend - shoulder     0.0 deg, elbow     0.0 deg, tip error 20.00 (clamped: out of reach)
target (  5,   5) bend + shoulder    45.0 deg, elbow   180.0 deg, tip error  2.93 (clamped: too close)
target (  5,   5) bend - shoulder    45.0 deg, elbow  -180.0 deg, tip error  2.93 (clamped: too close)
@@requires
def solve
math.acos
math.atan2
@@hint
d = the distance to the target, clamped to [|A − B| + ε, A + B]. Elbow interior angle: `acos((A² + B² − d²) / (2AB))`, and the local elbow angle is `bend × (π − interior)`. Shoulder: `atan2(ty, tx) − bend × acos((A² + d² − B²) / (2Ad))`. Check by computing the FK tip and measuring the error against the target (non-zero only when the target was clamped).
@@solution
import math
A, B = 40.0, 30.0
targets = [(50, 20), (0, 70), (30, -30), (90, 0), (5, 5)]

def solve(tx, ty, bend=1):
    d = math.hypot(tx, ty)
    clamped = min(max(d, abs(A - B) + 1e-9), A + B)
    interior = math.acos((A * A + B * B - clamped * clamped) / (2 * A * B))
    elbow = bend * (math.pi - interior)
    shoulder = math.atan2(ty, tx) - bend * math.acos((A * A + clamped * clamped - B * B) / (2 * A * clamped))
    return shoulder, elbow, clamped != d

def tip(shoulder, elbow):
    ex, ey = A * math.cos(shoulder), A * math.sin(shoulder)
    return ex + B * math.cos(shoulder + elbow), ey + B * math.sin(shoulder + elbow)

for tx, ty in targets:
    for bend, name in ((1, "bend +"), (-1, "bend -")):
        s, e, clamped = solve(tx, ty, bend)
        x, y = tip(s, e)
        error = math.hypot(x - tx, y - ty)
        note = " (clamped: out of reach)" if clamped and math.hypot(tx, ty) > A + B else " (clamped: too close)" if clamped else ""
        print(f"target ({tx:>3}, {ty:>3}) {name:<6} shoulder {math.degrees(s):7.1f} deg, elbow {math.degrees(e) + 0.0:7.1f} deg, tip error {error:5.2f}{note}")
