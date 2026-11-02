@@lesson ua1-synfig Synfig Studio: Vector Tweening, Bones and Linked Parameters
@@concepts numbers, functions, maps
@@lang python
@@text
**Synfig Studio** (synfig.org, free for Windows, macOS and Linux) is an industrial-strength 2D vector animation suite, an open-source alternative to Adobe Animate and Spine. You draw with vector shapes and let Synfig **tween** everything: positions, colours, and even the shapes themselves (path morphing). Skeleton **bones** deform artwork, and **linked parameters** let one value drive many. It's ideal for animated icons, mascots, button state transitions, onboarding scenes and explainer animations.

## The interface

| Panel | Use |
|---|---|
| **Canvas** | drawing and animating; the red border means **Animation mode** is on |
| **Toolbox** | Transform (A), Smooth Move, Scale, Rotate, Mirror, Circle (E), Rectangle (R), Star, Polygon, Gradient, **Spline** (B), Draw (pencil), Cutout, Width, Fill, Eyedropper, Text, **Skeleton** |
| **Params** | every parameter of the selected layer: origin, colour, amount (opacity), blend method, radius, vertices... |
| **Layers** | the stack; groups (formerly "Paste Canvas") contain layers and have their own transformation |
| **Timetrack** | waypoints per parameter, aligned with the Params panel |
| **Navigator, Library, Keyframes, Graphs** | overview, exported values, document keyframes, curve editor |

Set the canvas in **Canvas → Properties**: image size (e.g. 512 × 512 for an icon), and in the **Time** tab the frame rate (24, 30 or 60 fps) and the end time.

## Layers

Synfig builds artwork from **layers**, not objects:

- **Geometry**: Region (filled spline), Outline (stroked spline), Advanced Outline (variable width, dashes, smooth caps), Circle, Rectangle, Polygon, Star, Checkerboard.
- **Gradients**: linear, radial, conical, spiral, noise.
- **Transform**: Translate, Rotate, Scale, Stretch, **Skeleton Deformation**, Warp, Twirl, Noise Distort.
- **Filters**: Blur, Motion Blur, Shade (drop shadow), Bevel, Luma Key, Colour Correct.
- **Group** layers organise everything and have a transformation (offset, angle, skew, scale) you can animate as a unit, plus a **time offset** and **speed**, so one animated group can be reused at different moments.

The Spline tool (B) creates a Region and an Outline that share the same vertices: move a vertex and both follow.

## Animating

1. Turn on **Animation mode** (the button at the bottom right of the canvas, or the red border).
2. Move the time cursor, change something: Synfig creates a **waypoint** for that parameter.
3. Right-click a waypoint to set its **interpolation** for the in and out sides: **Clamped** (smooth without overshoot, the default), **TCB** (smooth through many points, can overshoot), **Constant** (hold), **Ease In/Out**, **Linear**.
4. **Keyframes** (Keyframes panel) are document-wide markers: changing timing between keyframes shifts all waypoints together, and keyframe **lock** keeps poses from drifting when you edit others.
5. The **Graphs** panel shows the curves of the selected parameters: smooth bumps mean smooth motion.

**Path morphing**: animate a spline's vertices from one shape to another (a play icon morphing into a pause icon, a hamburger into an X). Keep the same number of vertices in both shapes and they morph cleanly.

## Bones and skeleton deformation

- **Skeleton layer** + Skeleton tool: draw a chain of bones (a mascot's arm, a wiggling antenna on an icon).
- **Skeleton Deformation layer**: bends the artwork in its group smoothly as bones rotate.
- Or link spline vertices directly to bones (select vertices + a bone → right-click → Link to Bone).

Bones give organic motion with few waypoints: rotate two bones and a whole tail swishes.

## Linking and exporting values

Synfig's superpower for UI work:

- **Link**: select several parameters (with Ctrl in the Params panel or several vertices) → right-click → **Link**. They become one value: move one, all move.
- **Convert** a parameter to a different type of value node: **Linear** (a value that changes at a constant rate forever: spinners!), **Scale**, **Add**, **Subtract**, **Composite** (x and y separately), **Reference**, **Switch**, **Random**, **Sine**, **Exponential**, **Time Loop**, **Bone Link**...
- **Export Value**: name a value and reuse it from the Library panel; change it once and everything linked updates.

Example: link a button icon's **scale** to its shadow's **blur** and **opacity** through Scale/Add converts, then animate only the scale: the shadow follows automatically. The second exercise computes such linked values.

## Rendering and export

**File → Render** (or the render button):

- **PNG** image sequence or **PNG spritesheet**, **GIF**, **AVI/MP4/WebM** via FFmpeg, **SVG** (a single frame).
- **Lottie**: File → Export → **Lottie** (a plugin bundled with Synfig) writes a `.json` + HTML preview, for use in apps (Lesson 3). Not every Synfig feature exists in Lottie (filters, some converts): keep UI animations to shapes, groups, transforms, colours and opacity for best results.
- From the command line: `synfig project.sifz -t png -o frame.png` renders frames in scripts.

## A workflow for an animated icon

1. Import or draw the icon on a 24-unit grid scaled up (e.g. 480 × 480 = 20×).
2. Group each moving part; set each group's **origin** where it should rotate or scale (the hinge of a bell, the base of a heart).
3. Block the key poses on keyframes (idle, anticipation, action, overshoot, settle).
4. Set interpolation: ease-out into big poses, Clamped elsewhere; add a small overshoot.
5. Add secondary motion (sparkles, a shadow) last, linked to the main parameters.
6. Preview at 100% and at the real size; keep it under ~600 ms for UI.
7. Export to Lottie (or GIF/PNG for documentation).

## Easing curves as numbers

The custom Bézier easing used by Synfig's graph editor, Glaxnimate, Lottie and CSS's `cubic-bezier(x1, y1, x2, y2)` are the same idea: a curve from (0, 0) to (1, 1) with two control points. For a given time `t`, you find the curve parameter `s` whose x equals `t`, then the eased value is the curve's y at `s`. The first exercise implements it; it's exactly what browsers and Lottie players run.

@@quiz
What does a waypoint record in Synfig?
* The value of one parameter at one moment in time
- A keyboard shortcut
- A layer's name
- The render settings
? Keyframes are document-wide markers; waypoints belong to parameters.

@@quiz
How do you morph a play icon into a pause icon cleanly?
- Fade one out and the other in
* Animate the vertices of the same spline(s), keeping the same number of vertices in both shapes
- Use a Blur layer
- Change the canvas size
? Equal vertex counts let Synfig interpolate each vertex.

@@quiz
What is "Link" in Synfig?
- A web hyperlink in the animation
* Making several parameters share one value, so changing one changes all
- Connecting two Synfig files
- Exporting to Lottie
? Converts and exported values build on the same idea.

@@quiz
Which interpolation is smooth but never overshoots between waypoints?
- TCB
* Clamped
- Constant
- Random
? TCB can overshoot when values change direction.

@@quiz
A loading spinner should rotate forever at a constant speed. Which Synfig convert type helps?
- Constant
* Linear (a rate of change per second)
- Switch
- Bone Link
? Set the rotation's rate, e.g. 360° per second.

@@flashcards
Synfig Studio :: open-source 2D vector animation suite
Animation mode :: changes create waypoints (red border)
waypoint :: a parameter's value at a time
keyframe (Synfig) :: document-wide time marker that keeps poses
Clamped / TCB / Constant / Ease / Linear :: Synfig interpolations
Region / Outline / Advanced Outline :: filled / stroked / variable-width splines
Group layer :: container with its own transform, time offset and speed
path morphing :: animating spline vertices between shapes
Skeleton + Skeleton Deformation :: bones that bend artwork
Link :: parameters share one value
Convert :: turn a parameter into Linear, Scale, Add, Sine, Bone Link...
Export Value :: named reusable value in the Library
Lottie export :: Synfig plugin writing .json for apps
cubic-bezier(x1, y1, x2, y2) :: custom easing curve from (0,0) to (1,1)

@@exercise Solve a cubic-bezier easing
A cubic Bézier easing has control points P1 = (x1, y1) and P2 = (x2, y2), with P0 = (0, 0) and P3 = (1, 1). For curve parameter `s`, `bez(s, a, b) = 3(1−s)²·s·a + 3(1−s)·s²·b + s³` gives x (with x1, x2) or y (with y1, y2). Write `cubic_bezier(x1, y1, x2, y2, t)`: find `s` with `x(s) = t` by **bisection** (60 halvings between 0 and 1 is plenty), then return `y(s)` rounded to 4 decimals. Clamp `t` to 0–1. Then `css_ease(t)` = `cubic_bezier(0.25, 0.1, 0.25, 1, t)` (CSS `ease`).
@@starter
def bez(s, a, b):
    return 3 * (1 - s) ** 2 * s * a + 3 * (1 - s) * s ** 2 * b + s ** 3


def cubic_bezier(x1, y1, x2, y2, t):
    pass


def css_ease(t):
    pass


for t in (0, 0.25, 0.5, 0.75, 1):
    print(t, css_ease(t), cubic_bezier(0.34, 1.56, 0.64, 1, t))
@@tests
check(cubic_bezier(0, 0, 1, 1, 0.3), 0.3, "control points on the diagonal: linear")
check(css_ease(0), 0.0, "start")
check(css_ease(1), 1.0, "end")
check(css_ease(0.5), 0.8024, "CSS ease at half time")
check(cubic_bezier(0.42, 0, 1, 1, 0.5), 0.3154, "ease-in")
check(cubic_bezier(0, 0, 0.58, 1, 0.5), 0.6846, "ease-out")
check(cubic_bezier(0.34, 1.56, 0.64, 1, 0.5) > 1, True, "back-out overshoots above 1")
check(css_ease(2), 1.0, "clamped")
@@hint
`lo, hi = 0.0, 1.0`; repeat 60 times: `mid = (lo + hi) / 2`; if `bez(mid, x1, x2) < t` then `lo = mid` else `hi = mid`. Then `round(bez((lo + hi) / 2, y1, y2), 4)`. x(s) always increases when x1 and x2 are within 0–1, which is why bisection works.
@@solution
def bez(s, a, b):
    return 3 * (1 - s) ** 2 * s * a + 3 * (1 - s) * s ** 2 * b + s ** 3


def cubic_bezier(x1, y1, x2, y2, t):
    t = max(0.0, min(1.0, t))
    lo, hi = 0.0, 1.0
    for _ in range(60):
        mid = (lo + hi) / 2
        if bez(mid, x1, x2) < t:
            lo = mid
        else:
            hi = mid
    return round(bez((lo + hi) / 2, y1, y2), 4)


def css_ease(t):
    return cubic_bezier(0.25, 0.1, 0.25, 1, t)


for t in (0, 0.25, 0.5, 0.75, 1):
    print(t, css_ease(t), cubic_bezier(0.34, 1.56, 0.64, 1, t))

@@exercise Linked parameters
Model Synfig-style linked values. A `values` dict holds base parameters, and `links` maps a parameter name to a rule: `("link", other)` (same value), `("scale", other, factor)`, `("add", other, amount)`, `("linear", rate)` (a value growing by `rate` per second from 0, using `time`). Write `resolve(name, values, links, time)` that returns a parameter's value, following rules recursively (a rule can point at another linked parameter); unlinked names come from `values`. Then `frame_values(names, values, links, fps, frame)` returns a dict of every name's value at that frame, rounded to 3 decimals.
@@starter
def resolve(name, values, links, time):
    pass


def frame_values(names, values, links, fps, frame):
    pass


values = {"icon_scale": 1.2}
links = {
    "shadow_blur": ("scale", "icon_scale", 10),        # the shadow blurs more as the icon grows
    "shadow_opacity": ("add", "shadow_blur", -11.5),   # and darkens with the blur
    "badge_scale": ("link", "icon_scale"),
    "spinner_angle": ("linear", 360),                  # degrees per second
}
print(frame_values(["icon_scale", "shadow_blur", "shadow_opacity", "badge_scale", "spinner_angle"], values, links, 60, 15))
@@tests
values = {"icon_scale": 1.2}
links = {
    "shadow_blur": ("scale", "icon_scale", 10),
    "shadow_opacity": ("add", "shadow_blur", -11.5),
    "badge_scale": ("link", "icon_scale"),
    "spinner_angle": ("linear", 360),
}
check(resolve("icon_scale", values, links, 0), 1.2, "a base value")
check(resolve("badge_scale", values, links, 0), 1.2, "linked")
check(resolve("shadow_blur", values, links, 0), 12.0, "scaled")
check(round(resolve("shadow_opacity", values, links, 0), 3), 0.5, "a chain of rules")
check(resolve("spinner_angle", values, links, 0.5), 180.0, "linear over time")
r = frame_values(["shadow_blur", "spinner_angle"], values, links, 60, 15)
check(r, {"shadow_blur": 12.0, "spinner_angle": 90.0}, "frame 15 at 60 fps")
values["icon_scale"] = 1.0
check(resolve("shadow_blur", values, links, 0), 10.0, "change the base: everything follows")
@@hint
Look up `rule = links.get(name)`; if `None`, return `values[name]`. Otherwise branch on `rule[0]` and call `resolve` again for the referenced parameter. For frames, `time = frame / fps`.
@@solution
def resolve(name, values, links, time):
    rule = links.get(name)
    if rule is None:
        return values[name]
    kind = rule[0]
    if kind == "link":
        return resolve(rule[1], values, links, time)
    if kind == "scale":
        return resolve(rule[1], values, links, time) * rule[2]
    if kind == "add":
        return resolve(rule[1], values, links, time) + rule[2]
    if kind == "linear":
        return rule[1] * time
    raise ValueError(kind)


def frame_values(names, values, links, fps, frame):
    time = frame / fps
    return {n: round(resolve(n, values, links, time), 3) for n in names}


values = {"icon_scale": 1.2}
links = {
    "shadow_blur": ("scale", "icon_scale", 10),        # the shadow blurs more as the icon grows
    "shadow_opacity": ("add", "shadow_blur", -11.5),   # and darkens with the blur
    "badge_scale": ("link", "icon_scale"),
    "spinner_angle": ("linear", 360),                  # degrees per second
}
print(frame_values(["icon_scale", "shadow_blur", "shadow_opacity", "badge_scale", "spinner_angle"], values, links, 60, 15))
