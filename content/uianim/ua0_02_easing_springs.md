@@lesson ua0-easing-springs Easing Curves, Springs and Interpolating Colour
@@concepts numbers, functions, lists
@@lang python
@@text
Timing makes motion look intentional or cheap. The same 200 px move feels sluggish, snappy or bouncy depending on its **easing**. This lesson goes deeper into easing families, physical **springs** (what iOS, Android Compose, Framer Motion and egui-style animations use), and a frequent bug: interpolating colours in the wrong space.

## Easing families

| Family | Use |
|---|---|
| **linear** | progress bars, spinners, things not moving through space |
| **ease-out** (decelerate) | elements **entering**: they arrive fast and settle; the default for most UI |
| **ease-in** (accelerate) | elements **leaving** the screen |
| **ease-in-out** | moving from one place on screen to another |
| **back / overshoot** | playful emphasis; use sparingly |
| **steps(n)** | sprite sheets, typewriter effects, ticking clocks |

Polynomial easings: `easeOutQuad(t) = 1 − (1 − t)²`, cubic `1 − (1 − t)³`, and so on: higher powers feel snappier. CSS `cubic-bezier(x1, y1, x2, y2)` describes the same shapes with two control points. Material uses `cubic-bezier(0.2, 0, 0, 1)` ("emphasized") for a lot of motion.

## Durations

Small changes (a checkbox, a hover) take 100–150 ms; medium (a card expanding, a dialog) 200–300 ms; large or full-screen 300–500 ms. Exits can be shorter than entrances. On large screens, distance-based durations look more natural than fixed ones.

## Springs

A spring has no fixed duration: it's defined by **stiffness** k, **damping** c and **mass** m, and you simulate it: `a = (−k·(x − target) − c·v) / m`, then integrate velocity and position each frame.
- **Damping ratio** ζ = c / (2√(k·m)). ζ < 1 **underdamped** (overshoots and wobbles), ζ = 1 **critically damped** (fastest without overshoot), ζ > 1 **overdamped** (slow, no overshoot).
- Springs are **interruptible**: a new target keeps the current velocity, so a cancelled gesture turns smoothly. That's why gesture-driven UIs use them.
- The animation ends when both distance and velocity are below a small threshold ("rest").
- SwiftUI and Framer Motion also offer a duration + bounce form that converts to stiffness and damping.

Integrate with **semi-implicit Euler** (update velocity first, then position with the new velocity) at a fixed timestep: it's stable for UI springs. Plain explicit Euler can gain energy and explode at large timesteps.

## Interpolating colour

Blending sRGB hex values channel by channel passes through muddy greys (red → green goes through a dull brown). Better spaces:
1. **Linear RGB**: undo the sRGB gamma (`((c + 0.055) / 1.055)^2.4`), interpolate, re-encode. Physically correct light mixing.
2. **OKLab / OKLCH**: perceptually uniform; CSS supports `color-mix(in oklch, red, green)` and gradients `in oklch`. Interpolating hue in OKLCH keeps colours vivid.

@@quiz
Which easing suits an element entering the screen?
- ease-in
* ease-out
- linear
- steps(4)
? It arrives quickly and settles.

@@quiz
A spring has damping ratio 0.4. How does it behave?
* It overshoots the target and oscillates before settling
- It settles without overshoot as fast as possible
- It moves slowly without overshoot
- It never moves
? Underdamped.

@@quiz
Why are springs popular for gesture-driven animations?
- They're shorter than any easing
* They keep velocity when the target changes, so interruptions are smooth
- They use less CPU
- CSS can't do easing
? Interruptible and physical.

@@quiz
Why does red-to-green look muddy when interpolated in sRGB?
- sRGB lacks green
* sRGB values are gamma-encoded and not perceptually uniform
- Hex values can't be interpolated
- The browser rounds colours
? Use linear RGB or OKLCH.

@@quiz
About how long should a hover or checkbox animation take?
* 100–150 ms
- 500 ms
- 1 s
- 2 s
? Small changes need short motion.

@@flashcards
ease-out :: entering, decelerates
ease-in :: leaving, accelerates
ease-in-out :: moving across the screen
linear :: progress and spinners
steps(n) :: discrete frames
easeOutCubic :: 1 − (1 − t)³
spring parameters :: stiffness, damping, mass
damping ratio :: c / (2√(k·m))
underdamped :: overshoots
critically damped :: fastest, no overshoot
overdamped :: slow, no overshoot
rest threshold :: stop when slow and close
semi-implicit Euler :: velocity first, then position
linear RGB :: gamma removed
OKLCH :: perceptual colour space
color-mix(in oklch) :: CSS colour blending

@@drill
What is the damping ratio of a spring with stiffness 100, damping 20 and mass 1?
= 1
= 1.0
? 20 / (2·√100) = 1: critically damped.

@@drill
Which easing family is best for elements leaving the screen?
= ease-in
? They accelerate away.

@@exercise Simulate springs
Simulate a spring from 0 to 1 with semi-implicit Euler at 120 steps per second for several stiffness/damping settings. For each, print the damping ratio and its type, the overshoot (maximum position beyond 1, as a %), and the settle time: when the position stays within 0.001 of the target and the speed is below 0.01.
@@starter
import math

DT = 1 / 120
springs = {
    "bouncy": (300, 10, 1),
    "snappy": (400, 30, 1),
    "critical": (100, 20, 1),
    "heavy": (100, 40, 1),
}

def simulate(k, c, m, steps=600):
    raise NotImplementedError
@@expected
bouncy    zeta=0.29 underdamped       overshoot 38.0%  settles in 1417 ms
snappy    zeta=0.75 underdamped       overshoot  1.4%  settles in 442 ms
critical  zeta=1.00 critically damped overshoot  0.0%  settles in 983 ms
heavy     zeta=2.00 overdamped        overshoot  0.0%  settles in 2625 ms
@@requires
def simulate
math.sqrt(
DT
@@hint
`a = (-k * (x - 1) - c * v) / m; v += a * DT; x += v * DT`. Record positions. The settle time is the first step after which every later step is within the rest thresholds: track the last step that was outside them.
@@solution
import math

DT = 1 / 120
springs = {
    "bouncy": (300, 10, 1),
    "snappy": (400, 30, 1),
    "critical": (100, 20, 1),
    "heavy": (100, 40, 1),
}

def simulate(k, c, m, steps=600):
    x, v, peak, last_moving = 0.0, 0.0, 0.0, 0
    for i in range(1, steps + 1):
        a = (-k * (x - 1) - c * v) / m
        v += a * DT
        x += v * DT
        peak = max(peak, x)
        if abs(x - 1) > 0.001 or abs(v) > 0.01:
            last_moving = i
    return peak, last_moving * DT

for name, (k, c, m) in springs.items():
    zeta = c / (2 * math.sqrt(k * m))
    kind = "underdamped" if zeta < 1 else "critically damped" if zeta == 1 else "overdamped"
    peak, settle = simulate(k, c, m)
    print(f"{name:<9} zeta={zeta:.2f} {kind:<17} overshoot {max(0, peak - 1) * 100:4.1f}%  settles in {settle * 1000:.0f} ms")

@@exercise Interpolate colours properly
Interpolate between two colours in five steps in three ways: naive sRGB, linear RGB (undo and redo the sRGB transfer function), and OKLab (convert linear RGB → OKLab with the matrices given, interpolate, convert back). Print the hex values and each colour's relative luminance to show the naive midpoint is darker.
@@starter
def hex_to_rgb(h):
    return [int(h[i:i + 2], 16) / 255 for i in (1, 3, 5)]

def to_linear(c):
    raise NotImplementedError

pairs = [("#ff0000", "#00ff00"), ("#0000ff", "#ffff00")]
@@expected
#ff0000 -> #00ff00
  t=0    srgb #ff0000 (Y 0.21)  linear #ff0000 (Y 0.21)  oklab #ff0000 (Y 0.21)
  t=0.25 srgb #bf4000 (Y 0.15)  linear #e18900 (Y 0.34)  oklab #ed7300 (Y 0.30)
  t=0.5  srgb #808000 (Y 0.20)  linear #bcbc00 (Y 0.46)  oklab #d0a800 (Y 0.42)
  t=0.75 srgb #40bf00 (Y 0.38)  linear #89e100 (Y 0.59)  oklab #a0d500 (Y 0.55)
  t=1    srgb #00ff00 (Y 0.72)  linear #00ff00 (Y 0.72)  oklab #00ff00 (Y 0.72)
#0000ff -> #ffff00
  t=0    srgb #0000ff (Y 0.07)  linear #0000ff (Y 0.07)  oklab #0000ff (Y 0.07)
  t=0.25 srgb #4040bf (Y 0.08)  linear #8989e1 (Y 0.29)  oklab #1d77e7 (Y 0.19)
  t=0.5  srgb #808080 (Y 0.21)  linear #bcbcbc (Y 0.50)  oklab #6cabc7 (Y 0.37)
  t=0.75 srgb #bfbf40 (Y 0.49)  linear #e1e189 (Y 0.71)  oklab #b5d797 (Y 0.61)
  t=1    srgb #ffff00 (Y 0.93)  linear #ffff00 (Y 0.93)  oklab #ffff00 (Y 0.93)
@@requires
def to_linear
def to_oklab
2.4
@@hint
sRGB → linear: `c / 12.92` if `c <= 0.04045` else `((c + 0.055) / 1.055) ** 2.4`; reverse with `12.92 * c` or `1.055 * c ** (1 / 2.4) - 0.055`. OKLab: `lms = M1 · rgb`, take cube roots, then `M2 ·`. The inverse cubes after the inverse M2. Luminance = 0.2126 R + 0.7152 G + 0.0722 B in linear RGB.
@@solution
def hex_to_rgb(h):
    return [int(h[i:i + 2], 16) / 255 for i in (1, 3, 5)]

def to_hex(rgb):
    return "#" + "".join(f"{round(max(0, min(1, c)) * 255):02x}" for c in rgb)

def to_linear(c):
    return c / 12.92 if c <= 0.04045 else ((c + 0.055) / 1.055) ** 2.4

def to_srgb(c):
    c = max(0.0, min(1.0, c))
    return 12.92 * c if c <= 0.0031308 else 1.055 * c ** (1 / 2.4) - 0.055

def mat(m, v):
    return [sum(a * b for a, b in zip(row, v)) for row in m]

M1 = [[0.4122214708, 0.5363325363, 0.0514459929], [0.2119034982, 0.6806995451, 0.1073969566], [0.0883024619, 0.2817188376, 0.6299787005]]
M2 = [[0.2104542553, 0.7936177850, -0.0040720468], [1.9779984951, -2.4285922050, 0.4505937099], [0.0259040371, 0.7827717662, -0.8086757660]]
M2_INV = [[1, 0.3963377774, 0.2158037573], [1, -0.1055613458, -0.0638541728], [1, -0.0894841775, -1.2914855480]]
M1_INV = [[4.0767416621, -3.3077115913, 0.2309699292], [-1.2684380046, 2.6097574011, -0.3413193965], [-0.0041960863, -0.7034186147, 1.7076147010]]

def to_oklab(lin):
    return mat(M2, [v ** (1 / 3) for v in mat(M1, lin)])

def from_oklab(lab):
    return mat(M1_INV, [v ** 3 for v in mat(M2_INV, lab)])

def lerp(a, b, t):
    return [x + (y - x) * t for x, y in zip(a, b)]

def luminance(rgb):
    r, g, b = (to_linear(c) for c in rgb)
    return 0.2126 * r + 0.7152 * g + 0.0722 * b

pairs = [("#ff0000", "#00ff00"), ("#0000ff", "#ffff00")]
for a, b in pairs:
    print(f"{a} -> {b}")
    ra, rb = hex_to_rgb(a), hex_to_rgb(b)
    la, lb = [to_linear(c) for c in ra], [to_linear(c) for c in rb]
    for t in (0, 0.25, 0.5, 0.75, 1):
        naive = lerp(ra, rb, t)
        linear = [to_srgb(c) for c in lerp(la, lb, t)]
        ok = [to_srgb(c) for c in from_oklab(lerp(to_oklab(la), to_oklab(lb), t))]
        cells = "  ".join(f"{name} {to_hex(c)} (Y {luminance(c):.2f})" for name, c in (("srgb", naive), ("linear", linear), ("oklab", ok)))
        print(f"  t={t:<4} {cells}")
