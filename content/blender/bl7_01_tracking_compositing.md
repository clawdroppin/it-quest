@@lesson bl7-compositing Motion Tracking and the Node Compositor
@@concepts numbers, lists, functions
@@lang python
@@text
VFX means putting things that weren't there into real footage, convincingly. Blender does the whole chain: **track** the camera from video, **solve** its 3D motion, render CG with a matching virtual camera, and **composite** CG and footage together in the node **Compositor**.

## Motion tracking (the Movie Clip Editor)

Use the **Motion Tracking** workspace (File → New → VFX), then open your footage in the **Movie Clip Editor**.

1. **Set Scene Frames** to match the clip; **Prefetch** loads frames into memory. Use high-quality footage: sharp, fast shutter, lots of parallax (camera moving, not just rotating), visible detail.
2. **Markers**: Ctrl+click on high-contrast features (corners, spots) spread across the frame and at different depths. Pattern size and search size control what is matched and where it's searched.
3. **Track** forwards (Ctrl+T) and backwards (Shift+Ctrl+T). Motion models: Loc, LocRot, Affine, **Perspective** (for planar surfaces). Watch for drifting markers and delete or disable bad sections.
4. Aim for at least **8 good tracks** across every frame range used by the solve.
5. **Camera data**: focal length and sensor width (from the camera's specs); **lens distortion** coefficients if you can, or solve for them (refine focal length / K1, K2).
6. **Solve Camera Motion** with two keyframes A and B with enough parallax. The **solve error** (average reprojection error in pixels) should be **below 0.3–0.5 px** for good results; above 1 px something is wrong.
7. **Orientation**: select a track on the floor → Set Origin; three tracks on the floor → Set Floor; a track along an axis → Set X Axis; two tracks at a known distance → Set Scale.
8. **Setup Tracking Scene**: creates a camera with a **Camera Solver** constraint, a background with the footage, a shadow catcher floor, and compositor nodes for foreground/background layers.

**Object tracking** solves a moving object (a face, a box) separately. **Plane tracking** tracks four corners of a flat surface (a phone screen, a billboard) for screen replacements: the **Plane Track Deform** node in the compositor warps an image onto it.

**Reprojection error** is the distance in pixels between where a tracked feature was seen and where the solved 3D point projects back into the camera: the first exercise computes it.

## The Compositor

The **Compositing** workspace: enable **Use Nodes**. A **Render Layers** node outputs the render and its passes; a **Composite** node is the final image; a **Viewer** node shows any node's result as the backdrop. Since Blender 4.2 the **realtime compositor** also previews in the 3D viewport.

| Category | Nodes |
|---|---|
| Input | Render Layers, Image, Movie Clip, Mask, RGB, Value, Time Curve, Bokeh Image |
| Colour | **Color Balance** (lift/gamma/gain or offset/power/slope), Color Correction, RGB Curves, Hue/Saturation/Value, Exposure, Gamma, **Alpha Over**, Mix (blend modes), Z Combine, Tonemap |
| Filter | **Glare** (bloom, streaks, fog glow, ghosts), Blur, Bilateral Blur, Defocus (using Z), Vector Blur (motion blur from the vector pass), Denoise, Filter (sharpen, soften), Kuwahara (painterly), Pixelate |
| Matte | **Keying** (green screen), Chroma Key, Color Key, Difference Key, Luminance Key, **Cryptomatte**, ID Mask, Box/Ellipse Mask, Channel Key, Distance Key |
| Distort | Lens Distortion, **Movie Distortion** (apply/remove tracked lens distortion), Plane Track Deform, Corner Pin, Transform, Translate, Scale, Rotate, Crop, Displace, Stabilize 2D |
| Converter | Separate/Combine Color, Math, Map Range, Set Alpha, ID Mask |

## Common compositing recipes

- **CG over a plate**: Movie Clip (background) → Alpha Over ← Render Layers image (foreground with transparency: Film → Transparent). Add the shadow catcher layer multiplied over the plate.
- **Green screen**: Movie Clip → **Keying** node (pick the screen colour, adjust Clip Black/White, Despill) → Alpha Over a background. Use a garbage **Mask** to remove rigs and edges outside the subject.
- **Glow**: Glare (Fog Glow or Bloom) on emissive passes; Mix → Add.
- **Depth of field in post**: Defocus with the Z pass.
- **Colour matching**: Color Balance to match CG blacks, whites and tint to the plate; add **film grain** (noise texture image Mixed in Overlay) and a slight lens distortion so CG looks shot by the same camera.
- **Light wrap**: a blurred background bleeding onto the foreground edges for integration.
- **Stabilisation**: track two markers, Stabilize 2D node.

## Premultiplied alpha

Renders are **premultiplied**: colour values are already multiplied by alpha. The **over** operation is `result = fg + bg × (1 − fg_alpha)`. If you composite a **straight** (unpremultiplied) image as if it were premultiplied, edges look too bright or show halos, which is why Alpha Over has a "Convert Premultiplied" option. The second exercise implements Alpha Over and a simple green screen key.

## Masks and rotoscoping

The **Mask** editor (Movie Clip Editor → Mask mode, or Image Editor) draws animated spline shapes with feathered edges. Parent mask points to tracks so they follow the footage: **rotoscoping** to isolate a person or remove an object.

## When to use DaVinci Resolve's Fusion instead

Blender's compositor is excellent for compositing its own renders and tracked shots. For longer edits, heavy keying, and grading across many shots, many artists export EXR sequences and finish in **DaVinci Resolve** (Fusion and Color pages): the next course.

@@quiz
What does a camera solve error of 2.5 pixels suggest?
- An excellent solve
* Bad tracks, wrong camera data or lens distortion: the solve isn't reliable
- The footage is too short
- The render is too bright
? Aim for under about 0.3–0.5 px.

@@quiz
Which footage is best for camera tracking?
* Sharp, detailed footage with the camera moving through space (parallax)
- A tripod pan across a blank wall
- Heavily motion-blurred handheld footage at night
- Footage with no visible features
? Parallax lets the solver recover depth.

@@quiz
Which node puts a rendered CG element with transparency over a background plate?
- Z Combine
* Alpha Over
- Glare
- Vector Blur
? Enable Film → Transparent in render settings.

@@quiz
What does a plane track let you do?
- Fly a plane
* Replace a flat surface such as a phone screen or a sign, warping an image onto its four tracked corners
- Solve a full 3D camera
- Remove lens distortion
? The Plane Track Deform node does the warping.

@@quiz
In the "over" operation with premultiplied alpha, what is the result?
- fg × bg
* fg + bg × (1 − fg alpha)
- (fg + bg) / 2
- bg × fg alpha
? Premultiplied colour already includes alpha.

@@flashcards
Movie Clip Editor :: footage, markers, tracking and solving
marker / pattern / search area :: feature tracked / patch matched / where it's searched
Ctrl+T :: track markers forwards
solve error :: average reprojection error in pixels (aim < 0.5)
parallax :: depth cues from camera movement
Set Origin / Floor / X Axis / Scale :: orient the solved scene
Setup Tracking Scene :: camera, background, shadow catcher, nodes
plane track :: four-corner track for screen replacements
Render Layers / Composite / Viewer :: input / final output / preview
Alpha Over :: layer foreground over background
Keying node :: green/blue screen extraction with despill
Glare :: bloom, streaks, fog glow
Defocus :: depth of field using the Z pass
Movie Distortion :: apply or remove tracked lens distortion
premultiplied alpha :: colour already multiplied by alpha
rotoscoping :: animated masks following footage
light wrap :: background light bleeding onto foreground edges

@@exercise Project a point and measure the solve
A simple pinhole camera sits at the origin looking down −Z with focal length `f` in millimetres and sensor width `sensor` mm, rendering `width × height` pixels. Write `project(point, f, sensor, width, height)`: for a 3D point `(x, y, z)` with z < 0, image-plane coordinates are `u = f * x / -z`, `v = f * y / -z` (in mm); convert to pixels with `px = width / 2 + u * width / sensor` and `py = height / 2 - v * width / sensor` (square pixels, y down), rounded to 2. Return `None` for points behind the camera (z ≥ 0). Then `reprojection_error(points, observed, f, sensor, width, height)`: the average pixel distance between each projected 3D point and its observed 2D track position, rounded to 3.
@@starter
import math


def project(point, f, sensor, width, height):
    pass


def reprojection_error(points, observed, f, sensor, width, height):
    pass


pts = [(0, 0, -5), (1, 0.5, -5), (-2, 1, -10)]
print([project(p, 35, 36, 1920, 1080) for p in pts])
@@tests
check(project((0, 0, -5), 35, 36, 1920, 1080), (960.0, 540.0), "centre of the image")
check(project((1, 0.5, -5), 35, 36, 1920, 1080), (1333.33, 353.33), "right and up")
check(project((0, 0, 5), 35, 36, 1920, 1080), None, "behind the camera")
check(project((1, 0, -10), 35, 36, 1920, 1080), (1146.67, 540.0), "further away appears closer to the centre")
pts = [(0, 0, -5), (1, 0.5, -5)]
check(reprojection_error(pts, [(960, 540), (1333.33, 353.33)], 35, 36, 1920, 1080), 0.0, "perfect tracks")
check(reprojection_error(pts, [(963, 544), (1333.33, 353.33)], 35, 36, 1920, 1080), 2.5, "one track 5 px off")
@@hint
`scale = width / sensor` converts millimetres on the sensor to pixels. The error averages `math.hypot(px - ox, py - oy)` over all pairs.
@@solution
import math


def project(point, f, sensor, width, height):
    x, y, z = point
    if z >= 0:
        return None
    u = f * x / -z
    v = f * y / -z
    scale = width / sensor
    return round(width / 2 + u * scale, 2), round(height / 2 - v * scale, 2)


def reprojection_error(points, observed, f, sensor, width, height):
    total = 0.0
    for p, (ox, oy) in zip(points, observed):
        px, py = project(p, f, sensor, width, height)
        total += math.hypot(px - ox, py - oy)
    return round(total / len(points), 3)


pts = [(0, 0, -5), (1, 0.5, -5), (-2, 1, -10)]
print([project(p, 35, 36, 1920, 1080) for p in pts])

@@exercise Alpha Over and a green screen key
Pixels are `(r, g, b, a)` floats 0–1. Write `premultiply(px)` (multiply r, g, b by a). Write `alpha_over(fg, bg)` for premultiplied pixels: each channel (including alpha) is `fg + bg * (1 - fg_a)`, rounded to 3. Write `green_key(px, threshold, softness)` returning the pixel (straight colour) with a new alpha: `spill = g - max(r, b)`; alpha = 1 when spill ≤ threshold, 0 when spill ≥ threshold + softness, linear in between; then **despill**: if g > max(r, b), set g to max(r, b). Round to 3.
@@starter
def premultiply(px):
    pass


def alpha_over(fg, bg):
    pass


def green_key(px, threshold, softness):
    pass


plate = (0.2, 0.3, 0.6, 1.0)
for px in [(0.1, 0.9, 0.1, 1), (0.8, 0.6, 0.5, 1), (0.4, 0.6, 0.3, 1)]:
    keyed = green_key(px, 0.1, 0.3)
    print(keyed, alpha_over(premultiply(keyed), plate))
@@tests
check(premultiply((1, 0.5, 0, 0.5)), (0.5, 0.25, 0.0, 0.5), "premultiply")
check(alpha_over((1, 0, 0, 1), (0, 0, 1, 1)), (1.0, 0.0, 0.0, 1.0), "opaque foreground wins")
check(alpha_over((0, 0, 0, 0), (0, 0, 1, 1)), (0.0, 0.0, 1.0, 1.0), "transparent shows the background")
check(alpha_over((0.5, 0, 0, 0.5), (0, 0, 1, 1)), (0.5, 0.0, 0.5, 1.0), "half red over blue")
check(green_key((0.1, 0.9, 0.1, 1), 0.1, 0.3), (0.1, 0.1, 0.1, 0.0), "pure green screen is removed")
check(green_key((0.8, 0.6, 0.5, 1), 0.1, 0.3), (0.8, 0.6, 0.5, 1.0), "skin tones stay")
check(green_key((0.4, 0.6, 0.3, 1), 0.1, 0.3), (0.4, 0.4, 0.3, 0.667), "edge: partly transparent, despilled")
@@hint
`spill = g - max(r, b)`; alpha is `1 - (spill - threshold) / softness` clamped to 0–1. Despill changes only g.
@@solution
def premultiply(px):
    r, g, b, a = px
    return (r * a, g * a, b * a, a)


def alpha_over(fg, bg):
    k = 1 - fg[3]
    return tuple(round(f + b * k, 3) for f, b in zip(fg, bg))


def green_key(px, threshold, softness):
    r, g, b, _ = px
    m = max(r, b)
    spill = g - m
    alpha = 1 - (spill - threshold) / softness
    alpha = max(0.0, min(1.0, alpha))
    if g > m:
        g = m
    return (round(r, 3), round(g, 3), round(b, 3), round(alpha, 3))


plate = (0.2, 0.3, 0.6, 1.0)
for px in [(0.1, 0.9, 0.1, 1), (0.8, 0.6, 0.5, 1), (0.4, 0.6, 0.3, 1)]:
    keyed = green_key(px, 0.1, 0.3)
    print(keyed, alpha_over(premultiply(keyed), plate))
