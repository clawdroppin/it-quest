@@lesson ua1-motion-paths Motion Along Paths: Arcs, Constant Speed and Orienting to the Path
@@lang python
@@text
Natural motion rarely moves in straight lines. A thrown object follows an arc, and a card flying into a basket curves. Synfig, Glaxnimate, Penpot prototypes and CSS (`offset-path`) all let you move an object along a curve, and they all hit the same subtle problem: **a Bézier's parameter t isn't distance**. Equal steps in t give unequal steps along the curve, so the object speeds up and slows down on its own. This lesson shows the problem and the standard fix.

## Moving along a curve

- A cubic Bézier: B(t) = (1−t)³P0 + 3(1−t)²t P1 + 3(1−t)t² P2 + t³ P3, for t from 0 to 1.
- Where the control points are bunched together, points for equal t steps bunch together too, so the object **slows down** there. Spread out, it **speeds up**.
- **Arc-length parameterisation**: sample the curve into many small segments, accumulate their lengths into a table, and for a wanted distance s find the t at which the cumulative length reaches s (with a binary search and linear interpolation). Then **easing applies to distance**, not to t, so you control speed precisely.
- CSS `offset-path: path("M...")` with `offset-distance: 0% → 100%` already does this: offset-distance is a true length.

## Orienting to the path

- **Tangent**: B'(t) = 3(1−t)²(P1−P0) + 6(1−t)t(P2−P1) + 3t²(P3−P2). The rotation angle is atan2(dy, dx).
- CSS: `offset-rotate: auto`. Synfig: link the angle to the path tangent. Glaxnimate and After Effects: "auto-orient".
- Keep rotations subtle for UI. A paper plane may turn fully, but a card should only tilt a few degrees in the direction of travel.

## Arcs in UI

- Material motion and others use **arc motion** for elements that move diagonally: they curve rather than travel in a straight diagonal line, which feels more physical.
- Throw and drop interactions follow a **parabola**: x moves linearly, y follows gravity (y = v·t + ½g·t²).

@@quiz
Why does an object moved with equal steps in t not move at a constant speed?
* t isn't proportional to distance along the curve
- Browsers drop frames
- Béziers are always straight
- Easing is applied twice
? Use arc-length parameterisation.

@@quiz
What does arc-length parameterisation let you do?
* Map a wanted distance along the curve to the right t, so easing controls real speed
- Remove control points
- Convert a curve to a straight line
- Make SVGs smaller
? Build a table of cumulative lengths.

@@quiz
Which CSS property rotates an element to follow its offset-path?
- transform-origin
* offset-rotate: auto
- rotate: path
- will-change
? offset-distance moves it along the path.

@@quiz
Where do the points for equal t steps bunch together on a Bézier?
* Near control points that are close together, where the object moves slowly
- Always in the middle
- Never
- At the ends only
? Closer points = slower motion.

@@quiz
How is the rotation angle at a point on the path computed?
* atan2 of the tangent vector (the derivative)
- From the curve's length
- From the second control point only
- From the easing curve
? B'(t) gives the direction of travel.

@@flashcards
cubic Bézier :: B(t) from P0, P1, P2, P3
t vs distance :: not proportional
arc length table :: cumulative lengths of small segments
distance → t :: binary search the table, then interpolate
easing on distance :: controls real speed
tangent :: B'(t), the direction of travel
angle :: atan2(dy, dx)
offset-path :: CSS motion path
offset-distance :: true length along the path
offset-rotate: auto :: face the direction of travel
arc motion :: curve instead of a straight diagonal
parabola :: x linear, y with gravity

@@drill
What is B(0) for a cubic Bézier?
= P0
? The curve starts at P0 and ends at P3.

@@drill
Which CSS property sets how far along the offset-path an element is?
= offset-distance
? From 0% to 100%.

@@exercise Constant speed along a Bézier
Sample a cubic Bézier into 1,000 segments to build a cumulative arc-length table, then compare positions at equal t steps with positions at equal distance steps. Print the length of each step for both, showing that the t-based steps vary a lot while the distance-based ones stay nearly constant (straight-line chords are slightly shorter than the arc where the curve bends).
@@concepts numbers, lists, functions
@@starter
import math
from bisect import bisect_left
P = [(0, 0), (20, 160), (40, 160), (300, 0)]

def bezier(t):
    raise NotImplementedError

def t_at_distance(s, table, ts):
    raise NotImplementedError
@@expected
curve length 417.0px, ideal step 52.1px
equal t steps:       53.1  39.0  27.8  26.0  36.9  55.0  76.8 101.4
equal distance:      52.1  51.8  48.9  52.0  52.1  52.1  52.1  52.1
@@requires
def bezier
bisect_left
def t_at_distance
@@hint
`bezier(t)` uses the Bernstein weights `(1-t)**3, 3*(1-t)**2*t, 3*(1-t)*t**2, t**3`. Build `ts = [i / N for i in range(N + 1)]` and the cumulative `table` of `math.dist` between consecutive points. `t_at_distance` uses `bisect_left(table, s)` and interpolates between the two neighbouring t values.
@@solution
import math
from bisect import bisect_left
P = [(0, 0), (20, 160), (40, 160), (300, 0)]

def bezier(t):
    w = ((1 - t) ** 3, 3 * (1 - t) ** 2 * t, 3 * (1 - t) * t ** 2, t ** 3)
    return (sum(wi * p[0] for wi, p in zip(w, P)), sum(wi * p[1] for wi, p in zip(w, P)))

N = 1000
ts = [i / N for i in range(N + 1)]
table = [0.0]
for a, b in zip(ts, ts[1:]):
    table.append(table[-1] + math.dist(bezier(a), bezier(b)))
total = table[-1]

def t_at_distance(s, table, ts):
    i = bisect_left(table, s)
    if i == 0:
        return 0.0
    f = (s - table[i - 1]) / (table[i] - table[i - 1])
    return ts[i - 1] + f * (ts[i] - ts[i - 1])

STEPS = 8
by_t = [bezier(i / STEPS) for i in range(STEPS + 1)]
by_s = [bezier(t_at_distance(total * i / STEPS, table, ts)) for i in range(STEPS + 1)]
print(f"curve length {total:.1f}px, ideal step {total / STEPS:.1f}px")
print("equal t steps:     ", " ".join(f"{math.dist(a, b):5.1f}" for a, b in zip(by_t, by_t[1:])))
print("equal distance:    ", " ".join(f"{math.dist(a, b):5.1f}" for a, b in zip(by_s, by_s[1:])))

@@exercise Orient a paper plane along its path
Compute the tangent of the curve at several points and the rotation angle in degrees, then make a "subtle" variant for UI cards that keeps only 15% of the turn, clamped to ±8 degrees. Also compute a throw parabola's positions frame by frame until it lands.
@@concepts numbers, functions, loops
@@starter
import math
P = [(0, 0), (80, -120), (220, -120), (300, 0)]

def tangent(t):
    raise NotImplementedError
@@expected
t=0    tangent (  240.0,  -360.0) plane  -56.3 deg, card tilt  -8.0 deg
t=0.25 tangent (  307.5,  -180.0) plane  -30.3 deg, card tilt  -4.6 deg
t=0.5  tangent (  330.0,     0.0) plane    0.0 deg, card tilt   0.0 deg
t=0.75 tangent (  307.5,   180.0) plane   30.3 deg, card tilt   4.6 deg
t=1    tangent (  240.0,   360.0) plane   56.3 deg, card tilt   8.0 deg
throw lands after 23 frames at x=276: (12,-16) (60,-68) (108,-94) (156,-98) (204,-76) (252,-32)
@@requires
def tangent
math.atan2
math.degrees
@@hint
B'(t) = 3(1−t)²(P1−P0) + 6(1−t)t(P2−P1) + 3t²(P3−P2), separately for x and y. Angle = `math.degrees(math.atan2(dy, dx))`. Subtle = `max(-8, min(8, angle * 0.15))`. Parabola: x += vx each frame, vy += g, y += vy, until y ≥ 0 again.
@@solution
import math
P = [(0, 0), (80, -120), (220, -120), (300, 0)]

def tangent(t):
    d = [(P[i + 1][0] - P[i][0], P[i + 1][1] - P[i][1]) for i in range(3)]
    w = (3 * (1 - t) ** 2, 6 * (1 - t) * t, 3 * t ** 2)
    return sum(wi * di[0] for wi, di in zip(w, d)), sum(wi * di[1] for wi, di in zip(w, d))

for t in (0, 0.25, 0.5, 0.75, 1):
    dx, dy = tangent(t)
    angle = math.degrees(math.atan2(dy, dx))
    subtle = max(-8, min(8, angle * 0.15))
    print(f"t={t:<4} tangent ({dx:7.1f}, {dy:7.1f}) plane {angle:6.1f} deg, card tilt {subtle:5.1f} deg")

x, y, vx, vy, g, frame = 0.0, 0.0, 12.0, -18.0, 1.5, 0
points = []
while True:
    x += vx
    vy += g
    y += vy
    frame += 1
    points.append(f"({x:.0f},{y:.0f})")
    if y >= 0:
        break
print(f"throw lands after {frame} frames at x={x:.0f}: " + " ".join(points[::4]))
