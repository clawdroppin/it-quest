@@lesson ua0-motion Motion Design for Interfaces: Principles, Keyframes and Formats
@@concepts numbers, lists, functions
@@lang python
@@text
A heart that bursts into confetti when you like something, a toggle that squishes as it slides, a checkmark that draws itself after saving: these **micro-interactions** are tiny vector animations, and they're a big part of why some apps feel delightful. This course teaches you to make them with two free tools: **Synfig Studio** (a full 2D vector animation suite) and **Glaxnimate** (a lightweight editor for Lottie UI animations), and to put the results into real apps.

The exercises are in Python and build the maths those tools use: keyframes, interpolation, easing curves, and the Lottie JSON format itself.

## Principles of animation, applied to UI

Disney's animators wrote twelve principles; these matter most for interfaces:

| Principle | In UI |
|---|---|
| **Timing** | 100–300 ms for most UI; small things faster, big things slower |
| **Easing (slow in and out)** | nothing real starts or stops instantly; ease-out for entering, ease-in for leaving |
| **Anticipation** | a tiny wind-up before a big move (a button dips before launching a menu) |
| **Squash and stretch** | a bouncing dot squashes on impact: playful, shows weight |
| **Follow-through and overlap** | parts settle at different times (a card arrives, its contents follow 40 ms later) |
| **Arcs** | natural motion curves; a FAB that becomes a sheet moves along an arc |
| **Staging** | one thing moves at a time, so attention goes where it should |
| **Exaggeration** | a little overshoot (back-out easing) makes success feel celebratory |
| **Secondary action** | sparkles around a heart, a shadow that shrinks as an icon lifts |

## Keyframes and tweening

You set **keyframes**: the value of a property (position, scale, rotation, opacity, colour, a path's shape) at chosen moments. The software **tweens** (interpolates) every frame in between. How it interpolates is the **interpolation** or **easing** of the keyframe:

| Interpolation | Behaviour |
|---|---|
| **Constant / hold** | jumps at the next keyframe (blinking, frame-by-frame) |
| **Linear** | constant speed |
| **Ease in / out** | slows near the keyframe |
| **Bézier (custom curve)** | any speed profile, edited with handles: the "graph editor" |
| **TCB** (Synfig) | Tension, Continuity, Bias: smooth curves through many keyframes |

Time is measured in **frames**: at 60 fps, frame 30 is 0.5 s. UI animations are usually made at **60 fps** (or 30 to keep files small) and played at the screen's rate. The first exercise converts between frames and time and finds the value between keyframes.

## Formats for animated UI

| Format | Good for | Notes |
|---|---|---|
| **CSS transitions/keyframes** | hover, press, simple loops | no tools needed; the GUI Design course |
| **Animated SVG** (SMIL or CSS inside SVG) | icons that animate themselves | works in browsers, limited elsewhere |
| **Lottie** (`.json`) / **dotLottie** (`.lottie`, zipped) | rich vector micro-interactions | tiny, scalable, controllable from code; players for web, Android, iOS, Flutter, React Native, Rust |
| **Rive** (`.riv`) | interactive state machines | proprietary editor (free tier), very capable |
| **GIF / animated WebP / APNG** | previews, docs, chat | raster, bigger files, no interactivity |
| **Video** (MP4, WebM) | onboarding, marketing | not for small UI elements |
| **Sprite sheet** (PNG grid) | games, very simple players | raster frames in one image |

Lottie is the sweet spot for app micro-interactions: designers animate vectors, developers play them with a few lines of code, and the file stays a few kilobytes.

## Planning a micro-interaction

1. **Trigger**: what starts it (click, success, hover, data loaded)?
2. **Rules**: what happens, and what states exist (off → on → off, idle → loading → success → idle)?
3. **Feedback**: what the user sees and how long it takes.
4. **Loops and modes**: does it repeat (a loader) or play once (a like)? Does it reverse?

Sketch the states, block out the timing (a bar per element on a timeline), then animate the main motion first and add secondary details last.

@@quiz
Which easing suits an element entering the screen?
* Ease-out: it arrives quickly and settles gently
- Ease-in: it starts slowly
- Linear
- Constant (hold)
? Leaving elements use ease-in: they accelerate away.

@@quiz
At 60 fps, which frame is 0.75 seconds?
- Frame 60
* Frame 45
- Frame 75
- Frame 30
? frame = seconds × fps.

@@quiz
What is the best format for a small, scalable "like" animation controlled from app code?
- A GIF
- An MP4 video
* Lottie (JSON) or dotLottie
- A PNG sprite sheet
? Lottie stays vector and tiny, and players can play segments on demand.

@@quiz
What does "constant" (hold) interpolation do?
- Moves at a constant speed
* Keeps the value until the next keyframe, then jumps
- Eases in and out
- Loops forever
? Linear is the constant-speed one.

@@flashcards
micro-interaction :: small animation in response to a user action
keyframe :: a property's value at a chosen time
tweening :: generating the in-between frames
interpolation :: how values move between keyframes
constant / linear / ease / Bézier / TCB :: interpolation types
frame = seconds × fps :: time to frames
anticipation :: small wind-up before the main move
squash and stretch :: shows weight and playfulness
follow-through :: parts settle at different times
staging :: one focus at a time
Lottie :: JSON vector animation format with players everywhere
dotLottie :: zipped Lottie with images and multiple animations
Rive :: interactive vector animation with state machines
trigger, rules, feedback, loops :: anatomy of a micro-interaction

@@exercise Frames, time and keyframes
Write `to_frame(seconds, fps)` (rounded to the nearest frame), `to_time(frame, fps)` (seconds, rounded to 3 decimals), and `value_at(keyframes, frame)`: keyframes are a list of `(frame, value, interp)` sorted by frame, where `interp` is `"linear"` or `"hold"` and describes how the value moves **from that keyframe to the next**. Before the first keyframe return its value; after the last, the last value.
@@starter
def to_frame(seconds, fps):
    pass


def to_time(frame, fps):
    pass


def value_at(keyframes, frame):
    pass


scale = [(0, 1.0, "linear"), (6, 0.9, "linear"), (12, 1.1, "linear"), (18, 1.0, "hold")]
print([round(value_at(scale, f), 3) for f in range(0, 20, 3)])
print(to_frame(0.25, 60), to_time(45, 60))
@@tests
check(to_frame(0.25, 60), 15, "a quarter second at 60 fps")
check(to_frame(1 / 3, 24), 8, "rounded")
check(to_time(45, 60), 0.75, "frame 45")
check(to_time(1, 30), 0.033, "one frame at 30 fps")
scale = [(0, 1.0, "linear"), (6, 0.9, "linear"), (12, 1.1, "linear"), (18, 1.0, "hold")]
check(round(value_at(scale, 3), 3), 0.95, "halfway down")
check(round(value_at(scale, 9), 3), 1.0, "halfway up")
check(value_at(scale, 12), 1.1, "on a keyframe")
check(value_at(scale, 25), 1.0, "after the last")
check(value_at(scale, -5), 1.0, "before the first")
blink = [(0, 1, "hold"), (10, 0, "hold"), (20, 1, "hold")]
check([value_at(blink, f) for f in (0, 9, 10, 19, 20)], [1, 1, 0, 0, 1], "hold jumps")
@@hint
Loop over pairs `for (f0, v0, how), (f1, v1, _) in zip(keyframes, keyframes[1:])` and find the pair with `f0 <= frame < f1`. For linear, `t = (frame - f0) / (f1 - f0)` and `v0 + (v1 - v0) * t`.
@@solution
def to_frame(seconds, fps):
    return round(seconds * fps)


def to_time(frame, fps):
    return round(frame / fps, 3)


def value_at(keyframes, frame):
    if frame <= keyframes[0][0]:
        return keyframes[0][1]
    for (f0, v0, how), (f1, v1, _) in zip(keyframes, keyframes[1:]):
        if f0 <= frame < f1:
            if how == "hold":
                return v0
            t = (frame - f0) / (f1 - f0)
            return v0 + (v1 - v0) * t
    return keyframes[-1][1]


scale = [(0, 1.0, "linear"), (6, 0.9, "linear"), (12, 1.1, "linear"), (18, 1.0, "hold")]
print([round(value_at(scale, f), 3) for f in range(0, 20, 3)])
print(to_frame(0.25, 60), to_time(45, 60))

@@exercise Stagger a list
Follow-through in UI often means **staggering**: items of a list animate one after another. Write `stagger(n, start, delay, duration, fps)` returning, for each of `n` items, `(start_frame, end_frame)`: item `i` starts at `start + i * delay` seconds and lasts `duration` seconds, converted to frames with rounding. Then `total_frames(n, start, delay, duration, fps)`: the last item's end frame. Then `progress(n, start, delay, duration, fps, frame)`: a list with each item's progress 0–1 at that frame (clamped), rounded to 2 decimals.
@@starter
def stagger(n, start, delay, duration, fps):
    pass


def total_frames(n, start, delay, duration, fps):
    pass


def progress(n, start, delay, duration, fps, frame):
    pass


print(stagger(4, 0.1, 0.05, 0.3, 60))
print(progress(4, 0.1, 0.05, 0.3, 60, 18))
@@tests
check(stagger(4, 0.1, 0.05, 0.3, 60), [(6, 24), (9, 27), (12, 30), (15, 33)], "four cards")
check(total_frames(4, 0.1, 0.05, 0.3, 60), 33, "the whole animation")
check(total_frames(0, 0, 0.05, 0.3, 60), 0, "no items")
check(progress(4, 0.1, 0.05, 0.3, 60, 18), [0.67, 0.5, 0.33, 0.17], "mid-way")
check(progress(2, 0, 0.1, 0.2, 30, 0), [0.0, 0.0], "start")
check(progress(2, 0, 0.1, 0.2, 30, 99), [1.0, 1.0], "finished")
@@hint
`s = round((start + i * delay) * fps)` and `e = s + round(duration * fps)`. Progress is `(frame - s) / (e - s)` clamped with `max(0, min(1, ...))`.
@@solution
def stagger(n, start, delay, duration, fps):
    out = []
    for i in range(n):
        s = round((start + i * delay) * fps)
        out.append((s, s + round(duration * fps)))
    return out


def total_frames(n, start, delay, duration, fps):
    spans = stagger(n, start, delay, duration, fps)
    return spans[-1][1] if spans else 0


def progress(n, start, delay, duration, fps, frame):
    out = []
    for s, e in stagger(n, start, delay, duration, fps):
        p = max(0, min(1, (frame - s) / (e - s)))
        out.append(round(p, 2))
    return out


print(stagger(4, 0.1, 0.05, 0.3, 60))
print(progress(4, 0.1, 0.05, 0.3, 60, 18))
