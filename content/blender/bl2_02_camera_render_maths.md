@@lesson bl2-camera-render Cameras, Light and Render Budgets
@@lang python
@@text
Good renders come from the same decisions a photographer makes (focal length, aperture, light placement) plus the budgeting of a render engine: samples, noise and time. Blender's camera and lights use real-world units on purpose, so photography knowledge carries over directly. This lesson connects them and the exercises compute fields of view, depth of field and render budgets.

## The camera

- **Focal length** and **sensor width** set the **field of view**: `FOV = 2·atan(sensor / (2·focal))`. Blender's default 50 mm on a 36 mm sensor gives about 39.6° horizontally. Short lenses (14–24 mm) exaggerate depth and distort near objects; long lenses (85–200 mm) compress depth and flatter faces.
- **Depth of field** (Camera → Depth of Field): set a **focus object** or distance and an **F-stop**. Lower f-numbers mean shallower focus. The circle of confusion and hyperfocal distance work as in photography.
- Use **Camera → View** (Numpad 0) and *Lock Camera to View* to frame; add composition guides (thirds, golden ratio) in the camera's Viewport Display.
- **Orthographic** cameras remove perspective: isometric and technical renders, UI icons.

## Light

- **Point** and **Spot** lights obey the **inverse-square law**: intensity falls with 1/d². Doubling the distance gives a quarter of the light. Their power is in watts (not equal to real bulbs' electrical watts).
- **Sun** light has constant strength (irradiance in W/m²) regardless of distance.
- **Area** lights give soft shadows: shadow softness depends on the light's size relative to its distance.
- The **three-point lighting** setup: key (main, about 45°), fill (softer, opposite side, lower), rim/back (separates the subject from the background). HDRI environments give realistic ambient light and reflections.
- **Exposure** (Render → Color Management) works in stops; **AgX** (default since 4.0) handles bright highlights more naturally than Standard.

## Render budgets

- **Cycles** is a path tracer: noise falls with **1/√samples**. Halving the noise needs 4× the samples (and roughly 4× the time). **Adaptive sampling** (noise threshold) stops early on clean pixels; **denoisers** (OpenImageDenoise, OptiX) clean the rest.
- **EEVEE** is a rasteriser: fast, ideal for motion graphics and previews; its newer Next version has better global illumination and shadows.
- Animation cost = frames × time per frame. 10 s at 24 fps is 240 frames; at 2 minutes a frame that's 8 hours. Render to an **image sequence** (EXR or PNG), not a video file, so a crash doesn't lose everything; encode in the VSE or Resolve afterwards.
- Light paths (bounces), resolution percentage, simplify settings and persistent data are the main levers.

@@quiz
What field of view does a longer focal length give?
* Narrower
- Wider
- The same
- It depends on the F-stop
? FOV = 2·atan(sensor / 2f).

@@quiz
A point light moves from 2 m to 4 m away. How bright is the subject?
- Half as bright
* A quarter as bright
- The same
- Twice as bright
? Inverse-square law.

@@quiz
To halve Cycles noise, roughly how many more samples are needed?
- 2×
* 4×
- 8×
- 1.4×
? Noise ∝ 1/√samples.

@@quiz
Why render animations to an image sequence?
- It's the only option
* A crash or change only costs the affected frames
- Videos can't have colour management
- Image files are always smaller
? Encode afterwards.

@@quiz
Which light's strength doesn't depend on distance?
- Point
- Spot
- Area
* Sun
? It models a very distant source.

@@flashcards
FOV :: 2·atan(sensor / 2·focal)
50 mm on 36 mm sensor :: about 39.6°
wide lens :: exaggerated depth
long lens :: compressed depth
lower F-stop :: shallower focus
orthographic :: no perspective
inverse-square law :: light ∝ 1/d²
sun strength :: constant irradiance
area light size :: softer shadows
three-point lighting :: key, fill, rim
AgX :: default view transform
Cycles noise :: ∝ 1/√samples
adaptive sampling :: stop clean pixels early
image sequence :: safe animation output
EEVEE :: real-time rasteriser

@@drill
How many frames is a 10-second animation at 24 fps?
= 240
? 10 × 24.

@@drill
By what factor does light intensity drop when the distance triples? (as 1/n)
= 1/9
= 9
? 1 / 3².

@@exercise Lens and depth of field
For several focal lengths on a full-frame (36 × 24 mm) sensor, compute the horizontal and vertical field of view and how wide an area (in metres) is visible at 3 m. Then compute the depth of field for a subject at 3 m at f/1.8 and f/8 using the circle of confusion 0.03 mm: hyperfocal `H = f² / (N·c) + f`, near `= s(H − f) / (H + s − 2f)`, far `= s(H − f) / (H − s)` (infinity when s ≥ H).
@@starter
import math

SENSOR_W, SENSOR_H = 36, 24
COC = 0.03
lenses = [18, 35, 50, 85, 135]

def fov(sensor, focal):
    raise NotImplementedError
@@expected
 18 mm: FOV  90.0° x  67.4°, at 3 m shows 6.00 m wide
 35 mm: FOV  54.4° x  37.8°, at 3 m shows 3.09 m wide
 50 mm: FOV  39.6° x  27.0°, at 3 m shows 2.16 m wide
 85 mm: FOV  23.9° x  16.1°, at 3 m shows 1.27 m wide
135 mm: FOV  15.2° x  10.2°, at 3 m shows 0.80 m wide
35 mm f/1.8: sharp from 2.65 m to 3.45 m (depth 0.80 m, hyperfocal 22.7 m)
35 mm f/8: sharp from 1.90 m to 7.16 m (depth 5.26 m, hyperfocal 5.1 m)
85 mm f/1.8: sharp from 2.94 m to 3.07 m (depth 0.13 m, hyperfocal 133.9 m)
85 mm f/8: sharp from 2.74 m to 3.32 m (depth 0.59 m, hyperfocal 30.2 m)
@@requires
def fov
math.atan(
def depth_of_field
@@hint
`fov = math.degrees(2 * math.atan(sensor / (2 * focal)))`. Visible width at distance d = `2 * d * tan(fov / 2)` = `d * sensor / focal`. Do the DoF maths in millimetres (s = 3000) and convert back to metres.
@@solution
import math

SENSOR_W, SENSOR_H = 36, 24
COC = 0.03
lenses = [18, 35, 50, 85, 135]

def fov(sensor, focal):
    return math.degrees(2 * math.atan(sensor / (2 * focal)))

def depth_of_field(focal, n, s_m):
    s = s_m * 1000
    h = focal ** 2 / (n * COC) + focal
    near = s * (h - focal) / (h + s - 2 * focal)
    far = math.inf if s >= h else s * (h - focal) / (h - s)
    return near / 1000, far / 1000, h / 1000

for f in lenses:
    print(f"{f:>3} mm: FOV {fov(SENSOR_W, f):5.1f}° x {fov(SENSOR_H, f):5.1f}°, at 3 m shows {3 * SENSOR_W / f:.2f} m wide")
for f in (35, 85):
    for n in (1.8, 8):
        near, far, h = depth_of_field(f, n, 3)
        far_text = "infinity" if far == math.inf else f"{far:.2f} m"
        depth = "infinite" if far == math.inf else f"{far - near:.2f} m"
        print(f"{f} mm f/{n}: sharp from {near:.2f} m to {far_text} (depth {depth}, hyperfocal {h:.1f} m)")

@@exercise Plan a render
Estimate render settings for a shot: samples needed to reach a target noise level (noise ∝ 1/√samples, measured at 128 samples), the time per frame (proportional to samples, adjusted by resolution percentage), and the total time for the animation on one or several machines. Recommend a change when it won't finish before the deadline.
@@starter
import math

shots = [
    {"name": "product spin", "frames": 240, "noise_at_128": 0.08, "target_noise": 0.02, "sec_at_128": 40, "res_pct": 100},
    {"name": "logo sting", "frames": 120, "noise_at_128": 0.03, "target_noise": 0.03, "sec_at_128": 12, "res_pct": 100},
    {"name": "interior flythrough", "frames": 720, "noise_at_128": 0.12, "target_noise": 0.03, "sec_at_128": 90, "res_pct": 50},
]
DEADLINE_HOURS = 12
MACHINES = 2

def samples_for(noise_now, target, base=128):
    raise NotImplementedError
@@expected
product spin: 2048 samples, 640 s/frame, 21.3 h on 2 machines -> misses 12 h; denoise at 512 samples: 5.3 h
logo sting: 128 samples, 12 s/frame, 0.2 h on 2 machines
interior flythrough: 2048 samples, 360 s/frame, 36.0 h on 2 machines -> misses 12 h; denoise at 512 samples: 9.0 h
@@requires
def samples_for
math.ceil(
DEADLINE_HOURS
@@hint
`samples = base × (noise_now / target)²`, rounded up to a power of two if you like (here: `2 ** math.ceil(math.log2(...))`). Time per frame = `sec_at_128 × samples / 128 × (res_pct / 100)²`. Total hours = frames × seconds / 3600 / MACHINES. If over the deadline, suggest the denoiser with a quarter of the samples.
@@solution
import math

shots = [
    {"name": "product spin", "frames": 240, "noise_at_128": 0.08, "target_noise": 0.02, "sec_at_128": 40, "res_pct": 100},
    {"name": "logo sting", "frames": 120, "noise_at_128": 0.03, "target_noise": 0.03, "sec_at_128": 12, "res_pct": 100},
    {"name": "interior flythrough", "frames": 720, "noise_at_128": 0.12, "target_noise": 0.03, "sec_at_128": 90, "res_pct": 50},
]
DEADLINE_HOURS = 12
MACHINES = 2

def samples_for(noise_now, target, base=128):
    return 2 ** math.ceil(math.log2(base * (noise_now / target) ** 2))

def hours(shot, samples):
    per_frame = shot["sec_at_128"] * samples / 128 * (shot["res_pct"] / 100) ** 2
    return per_frame, shot["frames"] * per_frame / 3600 / MACHINES

for shot in shots:
    samples = samples_for(shot["noise_at_128"], shot["target_noise"])
    per_frame, total = hours(shot, samples)
    line = f"{shot['name']}: {samples} samples, {per_frame:.0f} s/frame, {total:.1f} h on {MACHINES} machines"
    if total > DEADLINE_HOURS:
        reduced = samples // 4
        _, faster = hours(shot, reduced)
        line += f" -> misses {DEADLINE_HOURS} h; denoise at {reduced} samples: {faster:.1f} h"
    print(line)
