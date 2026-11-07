@@lesson bl3-animation Animation, the Graph Editor and Rigging
@@concepts numbers, lists, functions
@@lang python
@@text
Anything in Blender can be animated: transforms, material colours, modifier values, light strength, even a node's input. Hover over a property and press **I** to insert a keyframe.

## Keyframes and the timeline

- **I** in the viewport inserts keyframes for the active **keying set** (Location, Rotation, Scale or all three); **Auto Keying** (the record button) keys every change.
- **Timeline**: frame range, current frame, play (Space), jump to start/end (Shift+Left/Right), next keyframe (Up/Down).
- **Dope Sheet**: every keyframe of every object, for retiming. G moves keys, S scales timing around the current frame, Shift+D duplicates, **Action Editor** mode shows reusable **actions**.
- **Graph Editor**: the **F-curves**. Each animated property is a curve of value over time. Select keys and press **T** for interpolation (Constant, Linear, Bézier, and easing presets: Sine, Quad, Cubic, Back, Bounce, Elastic) and **V** for handle types (Auto Clamped, Vector, Aligned, Free).
- **Modifiers on F-curves**: **Cycles** (loop the animation, with offset to keep walking forward), **Noise** (camera shake, flicker), **Stepped** (a choppy stop-motion look), Generator, Envelope.
- **NLA Editor** (non-linear animation): stack and blend actions like clips, e.g. a walk cycle repeated plus a wave on top.

The F-curve between two Bézier keyframes is a **cubic Bézier** in (frame, value) space, with each key's handles as the middle control points. The first exercise evaluates one.

## Animation principles in the Graph Editor

- **Ease in/out**: flat handles near keys.
- **Overshoot**: a key beyond the final value, then settle (or the Back/Elastic easing).
- **Spacing**: where keys fall in time determines speed; bunched keys are slow, spread keys fast.
- **Arcs**: check motion paths (Object → Motion Paths) for natural curves.
- **Offset**: shift secondary parts a few frames later for follow-through.
- **Breakdowns**: the in-between key that defines how a move happens (Shift+E on keys to set the key type: keyframe, breakdown, extreme, jitter).

## Drivers and constraints

- **Drivers** (right-click a property → Add Driver): a property computed from others with a Python expression. For example, a wheel's rotation `= distance / radius` from the car's X location, so wheels never slide.
- **Constraints**: Copy Location/Rotation/Scale, **Track To** / **Damped Track** (aim at a target), **Child Of** (switchable parenting: a hand picks up a cup), **Follow Path** (animate along a curve), **Limit** constraints, **IK** (below), **Floor**, **Clamp To**.

## Rigging with armatures

An **armature** is a skeleton of **bones** that deforms a mesh.

1. Shift+A → Armature. In Edit Mode, E extrudes new bones from a bone's tip; name them (`upper_arm.L`) so mirroring works (Symmetrize).
2. Parent the mesh to the armature: select mesh, then armature, Ctrl+P → **With Automatic Weights** (Blender computes vertex groups).
3. Fix deformation in **Weight Paint** mode: each vertex has a weight 0–1 per bone.
4. Animate in **Pose Mode** (Ctrl+Tab): rotate bones and key them.

**Forward kinematics (FK)**: you rotate each bone down the chain (shoulder, elbow, wrist): great for arcs and overlapping action. **Inverse kinematics (IK)**: you move the hand and an IK constraint solves the chain; set the chain length and a **pole target** that controls the elbow direction. The second exercise computes FK.

**Rigify** (bundled add-on) generates a full production rig (FK/IK switching, face rig, controls) from a meta-rig: add Armature → Human (Meta-Rig), fit it to the character, Generate Rig.

**Shape keys** (Object Data → Shape Keys) store alternative vertex positions for facial expressions and corrective shapes; their values are animatable and can be driven by bones.

## Motion graphics animation

For motion graphics you rarely rig characters: you animate transforms with strong easing, stagger copies (with drivers, the NLA or Geometry Nodes), and loop seamlessly. Tips:

- Make loops by matching the first and last keyframes and using Cycles modifiers.
- Use **empties** as controllers: parent many objects to an empty and animate it.
- **Follow Path** text and logos along curves; animate **text** body properties (offset, extrude, bevel).
- Bake animation (Object → Animation → Bake Action) before exporting to game engines.
- Import **Mixamo** animations or motion-capture BVH files for characters.

@@quiz
Which key inserts a keyframe on a hovered property?
* I
- K
- Alt+I removes it
- F12
? Alt+I deletes the keyframe.

@@quiz
Which F-curve modifier makes an animation loop forever?
- Noise
* Cycles
- Stepped
- Envelope
? "Repeat with offset" keeps a walk moving forward each cycle.

@@quiz
What's the difference between FK and IK?
* FK rotates each bone down the chain; IK places the end (hand, foot) and solves the bone rotations
- FK is for faces, IK for bodies
- IK only works in EEVEE
- They are the same
? Feet planted on the ground are the classic IK use.

@@quiz
How do you make wheels rotate correctly as a car moves, without keyframing them?
- Shape keys
* A driver computing rotation from the car's location divided by the wheel radius
- An Array modifier
- Weight painting
? angle (radians) = distance / radius.

@@quiz
What does Ctrl+P → With Automatic Weights do?
- Applies all modifiers
* Parents a mesh to an armature and creates vertex groups weighted to the nearest bones
- Creates an IK chain
- Bakes physics
? Fix problem areas afterwards in Weight Paint mode.

@@drill
Which key inserts a keyframe on the hovered property or selected object?
= I
? The timeline shows keyframes as diamonds.

@@flashcards
I / Alt+I :: insert / delete keyframe
Auto Keying :: record every change as keyframes
Dope Sheet :: keyframe timing overview
Graph Editor :: F-curves of value over time
T / V (graph editor) :: interpolation / handle type
Cycles modifier :: loop an F-curve
Noise modifier :: shake and flicker
NLA Editor :: layer and blend actions like clips
driver :: property computed from other properties by an expression
Track To / Damped Track :: aim constraints
Child Of :: switchable parent constraint
Follow Path :: animate along a curve
armature / bone / Pose Mode :: skeleton, joint, animating it
weight painting :: per-vertex bone influence
FK / IK :: rotate down the chain / solve from the end
pole target :: controls the elbow or knee direction
Rigify :: auto-generated production rig
shape keys :: stored vertex positions for expressions

@@exercise Evaluate an F-curve
Between two keyframes `k0` and `k1`, a Bézier F-curve uses four control points: `P0 = (f0, v0)`, `P1 = k0's right handle`, `P2 = k1's left handle`, `P3 = (f1, v1)`. Write `bezier_point(p0, p1, p2, p3, s)` returning the 2D point at curve parameter `s`. Then `fcurve_value(p0, p1, p2, p3, frame)`: find `s` whose x (frame) equals `frame` by bisection (60 steps), return the y value rounded to 3. Finally `auto_handles(f0, v0, f1, v1)` returns flat "ease" handles one third of the way along: `((f0 + d, v0), (f1 - d, v1))` with `d = (f1 - f0) / 3`.
@@starter
def bezier_point(p0, p1, p2, p3, s):
    pass


def fcurve_value(p0, p1, p2, p3, frame):
    pass


def auto_handles(f0, v0, f1, v1):
    pass


h1, h2 = auto_handles(1, 0, 25, 10)
print([fcurve_value((1, 0), h1, h2, (25, 10), f) for f in (1, 7, 13, 19, 25)])
@@tests
check(bezier_point((0, 0), (0, 0), (1, 1), (1, 1), 0.5), (0.5, 0.5), "symmetrical midpoint")
check(auto_handles(1, 0, 25, 10), ((9.0, 0), (17.0, 10)), "flat handles")
h1, h2 = auto_handles(1, 0, 25, 10)
check(fcurve_value((1, 0), h1, h2, (25, 10), 1), 0.0, "first key")
check(fcurve_value((1, 0), h1, h2, (25, 10), 13), 5.0, "halfway")
check(fcurve_value((1, 0), h1, h2, (25, 10), 25), 10.0, "last key")
check(fcurve_value((1, 0), h1, h2, (25, 10), 4), 0.43, "eased start: slow")
check(fcurve_value((0, 0), (10 / 3, 0), (20 / 3, 10), (10, 10), 3) < 3, True, "slower than linear early on")
linear = fcurve_value((0, 0), (10 / 3, 10 / 3), (20 / 3, 20 / 3), (10, 10), 4)
check(linear, 4.0, "handles on the line give linear motion")
@@hint
Each coordinate uses `(1−s)³·a + 3(1−s)²·s·b + 3(1−s)·s²·c + s³·d`. Bisect on the x coordinate: x increases with s when the handles don't cross.
@@solution
def bezier_point(p0, p1, p2, p3, s):
    u = 1 - s
    return tuple(u ** 3 * a + 3 * u * u * s * b + 3 * u * s * s * c + s ** 3 * d
                 for a, b, c, d in zip(p0, p1, p2, p3))


def fcurve_value(p0, p1, p2, p3, frame):
    lo, hi = 0.0, 1.0
    for _ in range(60):
        mid = (lo + hi) / 2
        if bezier_point(p0, p1, p2, p3, mid)[0] < frame:
            lo = mid
        else:
            hi = mid
    return round(bezier_point(p0, p1, p2, p3, (lo + hi) / 2)[1], 3) + 0.0


def auto_handles(f0, v0, f1, v1):
    d = (f1 - f0) / 3
    return (f0 + d, v0), (f1 - d, v1)


h1, h2 = auto_handles(1, 0, 25, 10)
print([fcurve_value((1, 0), h1, h2, (25, 10), f) for f in (1, 7, 13, 19, 25)])

@@exercise Forward kinematics for an arm
A 2D bone chain starts at a root point. Each bone has a length and a rotation **relative to its parent** (degrees). Write `fk(root, bones)` returning the list of joint positions: the root, then the tip of each bone, rounded to 3. The first bone's angle is measured from the +X axis; each child adds its angle to the accumulated angle. Then `reach(bones)` returns the distance from the root to the last tip rounded to 3, and `can_reach(bones, target, root)`: True if the target is within the total length of the chain (what an IK solver checks first).
@@starter
import math


def fk(root, bones):
    pass


def reach(bones):
    pass


def can_reach(bones, target, root):
    pass


arm = [(0.3, 90), (0.25, -90), (0.1, 0)]   # upper arm up, forearm bent forward, hand straight
print(fk((0, 1.4), arm), reach(arm))
@@tests
check(fk((0, 0), [(1, 0)]), [(0, 0), (1.0, 0.0)], "one bone along X")
check(fk((0, 0), [(1, 90), (1, -90)]), [(0, 0), (0.0, 1.0), (1.0, 1.0)], "an L shape")
arm = [(0.3, 90), (0.25, -90), (0.1, 0)]
check(fk((0, 1.4), arm), [(0, 1.4), (0.0, 1.7), (0.25, 1.7), (0.35, 1.7)], "an arm")
check(reach([(1, 0), (1, 0)]), 2.0, "straight chain")
check(reach([(1, 0), (1, 90)]), 1.414, "bent 90 degrees")
check(can_reach([(1, 0), (1, 0)], (1.5, 1), (0, 0)), True, "within reach")
check(can_reach([(1, 0), (1, 0)], (2, 1), (0, 0)), False, "too far")
@@hint
Keep `angle` and `(x, y)`; for each bone `angle += rel`, then `x += length * cos(radians(angle))`, `y += length * sin(...)`. Add `0.0` after rounding to avoid `-0.0`.
@@solution
import math


def fk(root, bones):
    x, y = root
    angle = 0
    joints = [root]
    for length, rel in bones:
        angle += rel
        x += length * math.cos(math.radians(angle))
        y += length * math.sin(math.radians(angle))
        joints.append((round(x, 3) + 0.0, round(y, 3) + 0.0))
    return joints


def reach(bones):
    tip = fk((0, 0), bones)[-1]
    return round(math.hypot(tip[0], tip[1]), 3)


def can_reach(bones, target, root):
    total = sum(length for length, _ in bones)
    return math.hypot(target[0] - root[0], target[1] - root[1]) <= total


arm = [(0.3, 90), (0.25, -90), (0.1, 0)]   # upper arm up, forearm bent forward, hand straight
print(fk((0, 1.4), arm), reach(arm))
