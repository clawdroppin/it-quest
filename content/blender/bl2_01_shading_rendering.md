@@lesson bl2-shading Materials, Shader Nodes, Lighting and Rendering
@@concepts numbers, lists, functions
@@lang python
@@text
A model becomes an image through **materials** (how surfaces react to light), **lighting**, a **camera** and a **render engine**. Blender's node-based shading is the same system used for VFX and product shots.

## Render engines

| Engine | How it works | Use |
|---|---|---|
| **EEVEE** (EEVEE Next since 4.2) | real-time rasteriser with ray-traced reflections, shadows, volumetrics | motion graphics, stylised work, previews, fast turnarounds |
| **Cycles** | path tracer: simulates light bounces physically | photoreal product shots, VFX, architecture |
| **Workbench** | viewport shading | modeling, clay renders |

Switch in Properties → Render. **Viewport shading** modes (Z pie): Wireframe, Solid, **Material Preview** (EEVEE with an HDRI), **Rendered**.

## Principled BSDF

The **Principled BSDF** node is one shader for almost every material (it follows the industry PBR standard, OpenPBR-like):

| Input | Meaning |
|---|---|
| **Base Color** | the surface colour (albedo) |
| **Metallic** | 0 dielectric (plastic, wood, skin), 1 metal: use 0 or 1, rarely in between |
| **Roughness** | 0 mirror-sharp reflections, 1 matte |
| **IOR** | index of refraction (1.45 plastic, 1.5 glass, 1.33 water) |
| **Alpha** | transparency for cut-outs (leaves) |
| **Normal** | bumps from a Normal Map or Bump node |
| **Subsurface** | light scattering under the surface (skin, wax, marble) |
| **Specular** | reflection strength and tint for dielectrics |
| **Transmission** | glass and liquids (with roughness for frosted glass) |
| **Coat** | a clear coat layer (car paint, varnish) |
| **Sheen** | soft fabric highlights (velvet) |
| **Emission** | glows; strength > 0 lights the scene in Cycles and with bloom in compositing |

## The Shader Editor

Shading workspace. **Nodes** connect from left (inputs) to right (Material Output). Shift+A adds nodes; Ctrl+T (with the Node Wrangler add-on) adds texture coordinates and mapping for an image texture.

- **Textures**: Image Texture (set colour maps to sRGB and data maps such as roughness and normal to **Non-Color**), Noise, Voronoi, Wave, Musgrave/Noise variants, Gradient, Brick, Checker.
- **Converters**: **Color Ramp** (map 0–1 to colours), Map Range, Math, Vector Math, Separate/Combine XYZ, Mix (colour blending modes: multiply, screen, overlay...).
- **Input**: Texture Coordinate (Generated, UV, Object), Geometry, Fresnel, Layer Weight, Object Info (random per object), Attribute.
- **Shaders**: Principled BSDF, Emission, Glass, Translucent, Transparent, Mix Shader, Add Shader, **Principled Volume** (smoke, fog).
- Group nodes (Ctrl+G) make reusable materials with exposed parameters.

**Procedural materials** use no images: a noise texture → color ramp → base colour + roughness gives infinite-resolution marble, rust or stylised gradients. The first exercise implements the Color Ramp and Mix nodes.

**PBR texture sets** (from Poly Haven or ambientCG, free) have Base Color, Roughness, Metallic, Normal, Displacement maps; connect each to the matching input.

## Lighting

| Light | Behaviour |
|---|---|
| **Point** | from a point in all directions; falls off with distance squared |
| **Sun** | parallel rays, strength in W/m² (irradiance); position irrelevant, rotation matters |
| **Spot** | cone with blend |
| **Area** | soft light from a rectangle or disk: the studio softbox |
| **World / HDRI** | an environment image lights everything (Poly Haven HDRIs) |
| **Emissive meshes** | neon signs, screens |

**Three-point lighting**: key (main, 45° to the side and above), fill (softer, opposite side, about a half to a quarter as strong), rim/back (separates the subject from the background). For product shots, big area lights and a curved backdrop (a "cyc"); for drama, one hard key and darkness.

Light falloff follows the **inverse square law**: double the distance, a quarter of the light. The second exercise uses it to place lights.

## Camera

Focal length (24 mm wide, 50 mm natural, 85–135 mm portraits and products), sensor size, **depth of field** (focus object + f-stop: lower f-stop, blurrier background), **clipping**. Numpad 0 for camera view; Ctrl+Alt+Numpad 0 snaps the camera to the current view; lock the camera to view (N panel → View) to fly it. Use **Track To** constraints to aim at a target.

## Rendering with Cycles

- **Samples**: more samples, less noise, longer renders. Use **adaptive sampling** (noise threshold 0.01–0.1) and the **denoiser** (OpenImageDenoise or OptiX) so 128–512 samples often look clean.
- **Light paths**: max bounces (diffuse, glossy, transmission, volume); caustics off for speed.
- **Colour management**: **AgX** view transform (default since 4.0, Filmic before it) handles bright lights gracefully; Look sets contrast. Use **Standard** only for flat graphics where colours must match exactly.
- **Output**: resolution and %, frame rate, file format. For animation, render **PNG/EXR image sequences** (a crash doesn't lose the whole video), then assemble in the VSE or DaVinci Resolve. **OpenEXR multilayer** keeps render passes for compositing.
- **Render passes** (View Layer properties): Diffuse, Glossy, Emission, AO, Mist, Z (depth), Normal, Cryptomatte (masks per object/material): the raw material for compositing.
- F12 renders a frame, Ctrl+F12 the animation. **Render region** (Ctrl+B in camera view) renders just part of the frame while tweaking.

## Render time budget

Render time ≈ frames × time per frame. 10 seconds at 24 fps is 240 frames; at 2 minutes each that's 8 hours. Plan with test renders at 50% resolution and fewer samples, and render overnight or on a render farm (SheepIt is a free community farm).

@@quiz
Which engine path-traces light physically for photoreal results?
- Workbench
- EEVEE
* Cycles
- Grease Pencil
? EEVEE is a real-time rasteriser (with some ray tracing in EEVEE Next).

@@quiz
What colour space should a roughness or normal map image use?
- sRGB
* Non-Color
- AgX
- Filmic Log
? Data maps hold numbers, not colours; sRGB would distort them.

@@quiz
A point light 2 m from an object is moved to 4 m. How much light reaches the object?
- Half
* A quarter
- The same
- Double
? Inverse square: (2/4)² = 1/4.

@@quiz
Why render an animation as an image sequence instead of a video file?
* A crash loses only the current frame, frames can be re-rendered individually, and quality stays lossless for compositing
- Video files can't be made by Blender
- Image sequences are always smaller
- It's required by Cycles
? Assemble the sequence in the VSE or Resolve afterwards.

@@quiz
What does Cryptomatte provide?
- A faster denoiser
* Automatic per-object or per-material masks for compositing
- An encrypted .blend file
- A camera lens model
? Pick objects in the compositor to isolate them.

@@flashcards
EEVEE / Cycles / Workbench :: real-time, path traced, viewport engines
Principled BSDF :: all-purpose PBR shader
Metallic :: 0 for dielectrics, 1 for metals
Roughness :: 0 sharp reflections, 1 matte
IOR :: index of refraction (glass 1.5, water 1.33)
Non-Color :: colour space for data maps
Color Ramp :: maps 0–1 values to colours
Mix (Color) :: blend modes: multiply, screen, overlay...
procedural texture :: generated by nodes, no image
HDRI :: environment image lighting the scene
inverse square law :: light falls off with distance squared
three-point lighting :: key, fill, rim
AgX :: Blender's default view transform since 4.0
adaptive sampling + denoiser :: clean renders with fewer samples
render passes :: separated layers for compositing
OpenEXR multilayer :: format keeping all passes
F12 / Ctrl+F12 :: render image / animation

@@exercise Color Ramp and Mix nodes
Colours are `(r, g, b)` floats 0–1. Write `lerp_color(a, b, t)` (rounded to 3). Write `color_ramp(stops, fac)`: `stops` is a list of `(position, color)` sorted by position; return the interpolated colour at `fac` (linear interpolation between the surrounding stops, clamped to the first and last colours), rounded to 3. Then `mix(mode, a, b, fac)` like the Mix Color node: result = lerp between `a` and the blended colour by `fac`, where the blend is `"mix"` (b), `"multiply"` (a × b), `"screen"` (1 − (1 − a)(1 − b)), `"add"` (a + b, clamped to 1), per channel, rounded to 3.
@@starter
def lerp_color(a, b, t):
    pass


def color_ramp(stops, fac):
    pass


def mix(mode, a, b, fac):
    pass


fire = [(0.0, (0, 0, 0)), (0.4, (0.8, 0.1, 0)), (0.8, (1, 0.7, 0.1)), (1.0, (1, 1, 0.9))]
print([color_ramp(fire, f) for f in (0, 0.2, 0.6, 0.9, 1.2)])
print(mix("multiply", (1, 0.5, 0.2), (0.5, 0.5, 1), 1.0))
@@tests
check(lerp_color((0, 0, 0), (1, 0.5, 0), 0.5), (0.5, 0.25, 0.0), "halfway")
fire = [(0.0, (0, 0, 0)), (0.4, (0.8, 0.1, 0)), (0.8, (1, 0.7, 0.1)), (1.0, (1, 1, 0.9))]
check(color_ramp(fire, 0.2), (0.4, 0.05, 0.0), "between the first stops")
check(color_ramp(fire, 0.6), (0.9, 0.4, 0.05), "between the middle stops")
check(color_ramp(fire, 0.4), (0.8, 0.1, 0.0), "on a stop")
check(color_ramp(fire, -1), (0, 0, 0), "clamped low")
check(color_ramp(fire, 1.2), (1, 1, 0.9), "clamped high")
check(mix("multiply", (1, 0.5, 0.2), (0.5, 0.5, 1), 1.0), (0.5, 0.25, 0.2), "multiply")
check(mix("screen", (0.5, 0.5, 0.5), (0.5, 0.5, 0.5), 1.0), (0.75, 0.75, 0.75), "screen brightens")
check(mix("add", (0.7, 0.2, 0), (0.6, 0.2, 0), 1.0), (1.0, 0.4, 0.0), "add clamps")
check(mix("mix", (1, 0, 0), (0, 0, 1), 0.25), (0.75, 0.0, 0.25), "a quarter blend")
check(mix("multiply", (1, 1, 1), (0, 0, 0), 0.5), (0.5, 0.5, 0.5), "fac 0.5")
@@hint
For the ramp, return the first colour if `fac <= stops[0][0]` and the last if `fac >= stops[-1][0]`; otherwise find the pair with `p0 <= fac <= p1` and use `t = (fac - p0) / (p1 - p0)`. For mix, compute the blend per channel, then `lerp_color(a, blended, fac)`.
@@solution
def lerp_color(a, b, t):
    return tuple(round(x + (y - x) * t, 3) for x, y in zip(a, b))


def color_ramp(stops, fac):
    if fac <= stops[0][0]:
        return stops[0][1]
    if fac >= stops[-1][0]:
        return stops[-1][1]
    for (p0, c0), (p1, c1) in zip(stops, stops[1:]):
        if p0 <= fac <= p1:
            return lerp_color(c0, c1, (fac - p0) / (p1 - p0))


def mix(mode, a, b, fac):
    if mode == "multiply":
        blend = [x * y for x, y in zip(a, b)]
    elif mode == "screen":
        blend = [1 - (1 - x) * (1 - y) for x, y in zip(a, b)]
    elif mode == "add":
        blend = [min(1.0, x + y) for x, y in zip(a, b)]
    else:
        blend = list(b)
    return lerp_color(a, blend, fac)


fire = [(0.0, (0, 0, 0)), (0.4, (0.8, 0.1, 0)), (0.8, (1, 0.7, 0.1)), (1.0, (1, 1, 0.9))]
print([color_ramp(fire, f) for f in (0, 0.2, 0.6, 0.9, 1.2)])
print(mix("multiply", (1, 0.5, 0.2), (0.5, 0.5, 1), 1.0))

@@exercise Lights and render budgets
Write `irradiance(power, distance)`: a point light's intensity at a distance, `power / (4π d²)`, rounded to 3. Write `fill_power(key_power, key_dist, fill_dist, ratio)`: the power a fill light needs at `fill_dist` so the subject receives `ratio` (e.g. 0.5) of the key's light, rounded to 1. Then `render_time(seconds, fps, sec_per_frame, machines=1)` returns `(frames, hours)` with hours rounded to 2, frames split evenly across machines (round the per-machine frame count up).
@@starter
import math


def irradiance(power, distance):
    pass


def fill_power(key_power, key_dist, fill_dist, ratio):
    pass


def render_time(seconds, fps, sec_per_frame, machines=1):
    pass


print(irradiance(1000, 2), fill_power(1000, 2, 3, 0.5), render_time(10, 24, 120))
@@tests
check(irradiance(1000, 2), 19.894, "1000 W at 2 m")
check(irradiance(1000, 4), 4.974, "a quarter at twice the distance")
check(fill_power(1000, 2, 3, 0.5), 1125.0, "a fill further away needs more power")
check(fill_power(1000, 2, 2, 0.25), 250.0, "same distance, a quarter")
check(render_time(10, 24, 120), (240, 8.0), "10 s at 2 min per frame")
check(render_time(10, 24, 120, machines=3), (240, 2.67), "three machines")
check(render_time(1, 25, 30, machines=2), (25, 0.11), "13 frames on the busiest machine")
@@hint
The fill needs `ratio × key irradiance` at its distance: `fill = ratio × key_power × (fill_dist / key_dist)²`. For machines, `per = -(-frames // machines)` and hours = `per × sec_per_frame / 3600`.
@@solution
import math


def irradiance(power, distance):
    return round(power / (4 * math.pi * distance ** 2), 3)


def fill_power(key_power, key_dist, fill_dist, ratio):
    return round(ratio * key_power * (fill_dist / key_dist) ** 2, 1)


def render_time(seconds, fps, sec_per_frame, machines=1):
    frames = round(seconds * fps)
    per = -(-frames // machines)
    return frames, round(per * sec_per_frame / 3600, 2)


print(irradiance(1000, 2), fill_power(1000, 2, 3, 0.5), render_time(10, 24, 120))
