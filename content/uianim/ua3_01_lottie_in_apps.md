@@lesson ua3-lottie-apps Animations in Real Apps: Web, Tauri and Rust
@@concepts maps, conditions, functions, numbers
@@lang python
@@text
You've animated a like button in Glaxnimate. Now make it respond to the user: play the hover segment on hover, the burst on click, reverse when un-liked, and respect people who prefer less motion.

## Web and Tauri: lottie-web and dotlottie-web

Tauri apps (the Rust UI course) render HTML, so web players work directly:

```text
npm install @lottiefiles/dotlottie-web

<canvas id="like" width="96" height="96"></canvas>

import { DotLottie } from "@lottiefiles/dotlottie-web";

const like = new DotLottie({
  canvas: document.getElementById("like"),
  src: "/anim/like.lottie",    // or a .json Lottie
  autoplay: false,
  loop: false,
});

button.addEventListener("mouseenter", () => like.setSegment(0, 20) || like.play());
button.addEventListener("click", () => {
  liked = !liked;
  like.setMode(liked ? "forward" : "reverse");
  like.setSegment(20, 60);
  like.play();
});
```

The older **lottie-web** (`lottie.loadAnimation({container, renderer: "svg", path, autoplay: false})`) plays JSON with `playSegments([a, b], true)`, `goToAndStop(frame, true)`, `setDirection(-1)` and `setSpeed(1.5)`. Its SVG renderer is crisp and lets CSS style layers by their names (`#heart path`), which makes theming possible. Use the **light** build (`lottie_light.min.js`) when you don't need expressions.

Markers from the previous lesson can be played by name: `like.setMarker("hover")` (dotLottie) or `anim.goToAndPlay("hover", true)` (lottie-web).

## Theming animations

- Ship the animation in neutral colours and recolour at runtime: with lottie-web's SVG renderer, CSS on layer classes (`.lottie-accent path { fill: var(--accent) }`) using `cl` (class) set on layers in the editor.
- dotLottie **themes** swap colours declared in the file (light/dark variants).
- Or generate variants with a script that rewrites the `"c"` colour arrays: the first exercise of the previous lesson showed the format.

## Native Rust

| Library | What it is |
|---|---|
| **dotlottie-rs** | LottieFiles' Rust player (ThorVG engine) used by their mobile SDKs; renders frames to a pixel buffer |
| **velato** (Linebender) | Lottie renderer on top of **vello** (GPU 2D); great with Xilem or wgpu apps |
| **rlottie** bindings | Samsung's C++ renderer, via the `rlottie` crate |
| **Rive** (`rive-rs`) | Rive files with interactive state machines, also on vello |

In **egui** the simple route is: render the current frame to an RGBA buffer (dotlottie-rs or rlottie), upload it as a texture with `ctx.load_texture`, and draw it with `ui.image`; advance the frame using `ui.input(|i| i.stable_dt)` and call `ctx.request_repaint()` while playing. **Slint** and **iced** can do the same with their image types. For simple icons, the egui animation helpers from the Rust UI course are often enough without Lottie at all.

## A micro-interaction catalogue

| Interaction | Motion | Timing |
|---|---|---|
| Like / favourite | scale dip 0.9 → burst 1.2 with particles → settle 1.0 | 400–600 ms |
| Toggle | knob slides with slight squash, track colour crossfades | 150–250 ms |
| Checkbox | box fills, check draws with trim path | 200–300 ms |
| Submit button | text → spinner → check (success) or shake (error) | spinner loops; success 400 ms |
| Pull to refresh | arrow rotates with pull distance, then spinner | linked to the gesture |
| Copy to clipboard | icon morphs to a check, back after 1.5 s | 200 ms each way |
| Notification bell | swing ±15° with decay | 600 ms |
| Empty state | gentle looping illustration | 2–4 s loop |
| Skeleton loading | shimmer gradient sliding across | 1.2–1.5 s loop |
| Tab switch | indicator slides; icons swap fill | 200 ms |

## Interaction states as a state machine

A good animated button is a small **state machine**: each state maps to a segment of the animation, and events (hover, press, success...) choose the next state and the segment to play. Defining that table first keeps the animation and the code in sync, which is exactly what Rive's state machines and dotLottie's newer state machine spec formalise. The first exercise builds one.

## Sprite sheets

For tiny raster players (games, embedded screens) or GIF-free previews, Synfig and Glaxnimate can render **sprite sheets**: every frame in a grid in one PNG. Playing it means showing the cell of the current frame: column `frame % columns`, row `frame // columns`. In CSS this is `background-position` with `steps()` timing. The second exercise does the maths.

## Performance and accessibility

- Don't autoplay looping animations everywhere: pause offscreen ones (`IntersectionObserver`), and don't run ten players at once on a list.
- Prefer the canvas/WASM players (dotlottie-web) for many or complex animations; SVG renderers are crisp but heavier with lots of nodes.
- Respect **prefers-reduced-motion**: jump to the final frame (`goToAndStop(last)`) instead of playing.
- Never convey information only through animation: the "liked" state also changes `aria-pressed` and the count.
- Keep files small: simplify paths, avoid big embedded images, round numbers (LottieFiles and Glaxnimate minify).

@@quiz
How do you play just frames 20–60 of an animation with dotlottie-web?
- `like.play(20, 60)`
* `like.setSegment(20, 60)` then `like.play()`
- Change `op` in the file
- Use two separate files
? lottie-web's equivalent is `playSegments([20, 60], true)`.

@@quiz
What should happen to a decorative animation when the user prefers reduced motion?
- Play it faster
* Skip to its final frame (or don't play it) instead of animating
- Play it twice
- Hide the whole button
? Keep the result, drop the motion.

@@quiz
In a sprite sheet with 8 columns, which row and column hold frame 19?
- Row 3, column 2
* Row 2, column 3 (0-based)
- Row 1, column 8
- Row 19, column 0
? 19 // 8 = 2 and 19 % 8 = 3.

@@quiz
Which Rust library renders Lottie on the GPU with vello?
- serde_json
* velato
- clap
- tokio
? dotlottie-rs (ThorVG) and rlottie are the CPU-renderer options.

@@quiz
How can a Lottie animation follow the app's dark theme?
* Recolour at runtime: CSS on named layers (SVG renderer), dotLottie themes, or rewriting the colour arrays
- It can't; export two GIFs
- Add a Blur layer
- Change the frame rate
? Name your layers in the editor so code can target them.

@@drill
Which JavaScript library plays Lottie animations in a web page?
= lottie-web
= dotlottie-web
? dotlottie-web plays the smaller .lottie format.

@@flashcards
dotlottie-web :: LottieFiles' canvas/WASM web player
lottie-web :: Airbnb's original player (svg, canvas, html renderers)
setSegment / playSegments :: play a frame range
setMode("reverse") / setDirection(-1) :: play backwards
goToAndStop(frame, true) :: show one frame
markers :: named segments played by name
dotlottie-rs :: Rust Lottie player on ThorVG
velato :: Lottie renderer on vello (GPU)
Rive :: interactive animation with state machines (rive-rs)
egui + Lottie :: render frames to a texture, request_repaint while playing
sprite sheet :: animation frames in a grid image
prefers-reduced-motion :: jump to the end state instead of animating
IntersectionObserver :: pause offscreen animations
aria-pressed :: the toggle state for assistive tech

@@exercise An animated button controller
Write a controller for a Lottie like button. `SEGMENTS` maps a state name to its `(start, end)` frames. `TRANSITIONS` maps `(state, event)` to the next state. Write `step(state, event)` returning `(next_state, segment)` where `segment` is the frames to play: the next state's segment, or `None` when the event doesn't apply (state unchanged). Then `run(events, reduced_motion=False)` starting from `"idle"` returns the list of segments played (skipping `None`); with reduced motion, each segment becomes `(end, end)` (jump to its last frame).
@@starter
SEGMENTS = {
    "idle": (0, 0),
    "hover": (0, 20),
    "liked": (20, 60),
    "unliked": (60, 80),
}
TRANSITIONS = {
    ("idle", "enter"): "hover",
    ("hover", "leave"): "idle",
    ("hover", "click"): "liked",
    ("idle", "click"): "liked",
    ("liked", "click"): "unliked",
    ("unliked", "click"): "liked",
    ("unliked", "leave"): "idle",
}


def step(state, event):
    pass


def run(events, reduced_motion=False):
    pass


print(run(["enter", "click", "click", "leave"]))
print(run(["enter", "click"], reduced_motion=True))
@@tests
check(step("idle", "enter"), ("hover", (0, 20)), "hover")
check(step("hover", "click"), ("liked", (20, 60)), "like")
check(step("liked", "leave"), ("liked", None), "leaving keeps liked")
check(step("liked", "click"), ("unliked", (60, 80)), "unlike")
check(run(["enter", "click", "click", "leave"]), [(0, 20), (20, 60), (60, 80), (0, 0)], "a session")
check(run(["leave", "click"]), [(20, 60)], "ignored events play nothing")
check(run(["enter", "click"], reduced_motion=True), [(20, 20), (60, 60)], "reduced motion jumps to the end")
check(run([]), [], "nothing happens")
@@hint
`nxt = TRANSITIONS.get((state, event))`; if `None` return `(state, None)`. In `run`, keep a `state` variable and append each segment that isn't `None`.
@@solution
SEGMENTS = {
    "idle": (0, 0),
    "hover": (0, 20),
    "liked": (20, 60),
    "unliked": (60, 80),
}
TRANSITIONS = {
    ("idle", "enter"): "hover",
    ("hover", "leave"): "idle",
    ("hover", "click"): "liked",
    ("idle", "click"): "liked",
    ("liked", "click"): "unliked",
    ("unliked", "click"): "liked",
    ("unliked", "leave"): "idle",
}


def step(state, event):
    nxt = TRANSITIONS.get((state, event))
    if nxt is None:
        return state, None
    return nxt, SEGMENTS[nxt]


def run(events, reduced_motion=False):
    state = "idle"
    played = []
    for e in events:
        state, seg = step(state, e)
        if seg is None:
            continue
        if reduced_motion:
            seg = (seg[1], seg[1])
        played.append(seg)
    return played


print(run(["enter", "click", "click", "leave"]))
print(run(["enter", "click"], reduced_motion=True))

@@exercise Sprite sheet maths
Write `sheet_size(frames, columns, cell_w, cell_h)` returning the PNG's `(width, height)` (rows = frames divided by columns, rounded up). Write `cell(frame, columns, cell_w, cell_h)` returning the `(x, y)` pixel offset of a frame's top-left corner. Write `frame_at(ms, fps, frames, loop)`: the frame showing at `ms` milliseconds (`int(ms * fps / 1000)`); when looping wrap around with `%`, otherwise stop on the last frame. Finally `css_position(frame, columns, cell_w, cell_h)` returns the CSS `background-position` string, e.g. `"-192px -64px"` (offsets are negative; zero is `"0px"`).
@@starter
def sheet_size(frames, columns, cell_w, cell_h):
    pass


def cell(frame, columns, cell_w, cell_h):
    pass


def frame_at(ms, fps, frames, loop):
    pass


def css_position(frame, columns, cell_w, cell_h):
    pass


print(sheet_size(30, 8, 64, 64), cell(19, 8, 64, 64), frame_at(750, 30, 30, False), css_position(11, 8, 64, 64))
@@tests
check(sheet_size(30, 8, 64, 64), (512, 256), "30 frames in 8 columns: 4 rows")
check(sheet_size(16, 4, 32, 48), (128, 192), "exact fit")
check(cell(0, 8, 64, 64), (0, 0), "first frame")
check(cell(19, 8, 64, 64), (192, 128), "row 2, column 3")
check(frame_at(750, 30, 30, False), 22, "0.75 s at 30 fps")
check(frame_at(1500, 30, 30, False), 29, "stops on the last frame")
check(frame_at(1500, 30, 30, True), 15, "loops around")
check(css_position(11, 8, 64, 64), "-192px -64px", "CSS offset")
check(css_position(0, 8, 64, 64), "0px 0px", "zero")
@@hint
Rows: `-(-frames // columns)` or `math.ceil`. Cell: `(frame % columns * cell_w, frame // columns * cell_h)`. For CSS, format each offset as `"0px"` when zero, else `f"-{v}px"`.
@@solution
def sheet_size(frames, columns, cell_w, cell_h):
    rows = -(-frames // columns)
    return columns * cell_w, rows * cell_h


def cell(frame, columns, cell_w, cell_h):
    return frame % columns * cell_w, frame // columns * cell_h


def frame_at(ms, fps, frames, loop):
    f = int(ms * fps / 1000)
    return f % frames if loop else min(f, frames - 1)


def css_position(frame, columns, cell_w, cell_h):
    x, y = cell(frame, columns, cell_w, cell_h)
    fmt = lambda v: "0px" if v == 0 else f"-{v}px"
    return fmt(x) + " " + fmt(y)


print(sheet_size(30, 8, 64, 64), cell(19, 8, 64, 64), frame_at(750, 30, 30, False), css_position(11, 8, 64, 64))
