@@lesson ua3-perf-a11y Smooth and Respectful Motion: Frame Budgets, Compositing and Reduced Motion
@@concepts numbers, lists, maps, conditions
@@lang python
@@text
An animation that stutters looks worse than no animation, and one that ignores a user's motion settings can make them feel ill. This lesson covers the frame budget, which properties are cheap to animate, how to find jank in frame timings, and how to design for `prefers-reduced-motion` and other accessibility needs.

## The frame budget

At 60 Hz each frame has **16.7 ms**; at 120 Hz only **8.3 ms**, and the browser or toolkit needs part of that. When work takes longer, frames are **dropped** and motion stutters (**jank**). Long tasks on the main thread (JavaScript, layout, image decoding) are the usual cause.

## The rendering pipeline

**Style → Layout → Paint → Composite.** Changing a property re-runs its stage and everything after it:

| Changing | Triggers | Examples |
|---|---|---|
| geometry | layout + paint + composite | `width`, `height`, `top`, `left`, `margin`, `padding`, `font-size` |
| appearance | paint + composite | `color`, `background-color`, `box-shadow`, `border-radius` |
| compositor-only | composite only | **`transform`**, **`opacity`**, (often) `filter` |

So animate `transform: translate/scale` instead of `left`/`width`, and fade a pre-rendered shadow layer's opacity instead of animating `box-shadow`. `will-change: transform` promotes an element to its own layer ahead of time; use it sparingly (each layer costs memory). Native toolkits follow the same idea: egui repaints everything cheaply, but expensive layout or text shaping in a frame still drops frames.

## Measuring

- Browser DevTools Performance panel: frames, long tasks, layout shifts; the Rendering tab can highlight paint flashing and layer borders.
- `requestAnimationFrame` timestamps show frame intervals; anything well over the refresh interval is a dropped frame.
- Report percentiles (p95 frame time), not just the average FPS.

## Reduced motion and accessibility

- `@media (prefers-reduced-motion: reduce)` reflects an OS setting (Windows "Animation effects" off, macOS/iOS "Reduce motion", GNOME "Reduce animation"). Respect it: replace large movement, parallax, zooms and spinning with **fades or instant changes**. Don't remove all feedback; a short opacity change is fine.
- WCAG 2.2: **2.3.1** no more than **three flashes per second**; **2.2.2 Pause, Stop, Hide** for anything that moves, blinks or scrolls automatically for more than **5 seconds**; **2.3.3** (AAA) motion triggered by interaction can be disabled.
- Large-area motion, parallax and spinning cause vestibular discomfort. Keep looping decorative animations short or stoppable.
- Never rely on animation alone to convey information; pair it with text or state changes that screen readers announce.

```css,norun
@media (prefers-reduced-motion: reduce) {
  *, *::before, *::after { animation-duration: 0.01ms !important; animation-iteration-count: 1 !important; transition-duration: 0.01ms !important; scroll-behavior: auto !important; }
}
```

This blanket rule is a safety net; better still, design a calmer alternative for each animation.

@@quiz
What's the frame budget at 120 Hz?
- 16.7 ms
* 8.3 ms
- 33 ms
- 4 ms
? 1000 / 120.

@@quiz
Which property is cheapest to animate?
- width
- top
* transform
- box-shadow
? Composite only.

@@quiz
Animating `background-color` triggers which stages?
- Layout, paint and composite
* Paint and composite
- Composite only
- None
? It doesn't change geometry.

@@quiz
A user has "reduce motion" on. What should a page-transition slide do?
- Play normally
- Play twice as fast
* Become a simple fade or instant change
- Show a warning
? Keep feedback, drop the movement.

@@quiz
According to WCAG 2.3.1, how many flashes per second are too many?
- More than one
* More than three
- More than ten
- Any flash
? The three flashes threshold.

@@flashcards
60 Hz budget :: 16.7 ms
120 Hz budget :: 8.3 ms
jank :: dropped frames
pipeline :: style, layout, paint, composite
layout triggers :: width, height, top, margin
paint triggers :: color, background, box-shadow
compositor-only :: transform, opacity
will-change :: promote to a layer
p95 frame time :: better than average FPS
prefers-reduced-motion :: OS motion setting
reduced-motion alternative :: fade or instant
WCAG 2.3.1 :: max three flashes per second
WCAG 2.2.2 :: pause anything over 5 s
vestibular :: parallax and zoom discomfort

@@drill
Name one of the two properties that only trigger compositing.
= transform
= opacity
? Both skip layout and paint.

@@drill
Which CSS media feature reflects the user's reduce-motion setting?
= prefers-reduced-motion
? Value "reduce".

@@exercise Find jank in frame timings
Given `requestAnimationFrame` timestamps (ms) from three recordings at different refresh rates, compute the frame intervals, count dropped frames (each interval counts as `round(interval / budget) - 1` dropped frames), the effective FPS, the p95 frame time, and the longest stall. Rate each recording "smooth" if fewer than 2% of frames were dropped and p95 is under 1.5 × budget.
@@starter
recordings = {
    "menu open @60Hz": (60, [0, 16.7, 33.4, 50.1, 66.8, 83.5, 100.2, 116.9, 133.6, 150.3, 167.0, 183.7, 200.4]),
    "list scroll @60Hz": (60, [0, 16.7, 33.4, 83.5, 100.2, 116.9, 167.0, 183.7, 200.4, 250.5, 267.2]),
    "card flip @120Hz": (120, [0, 8.3, 16.7, 25.0, 33.3, 41.7, 58.3, 66.7, 75.0, 83.3, 91.7, 100.0]),
}

def analyse(hz, stamps):
    raise NotImplementedError
@@expected
menu open @60Hz: budget 16.7 ms, 60 fps, dropped 0 (0%), p95 16.7 ms, worst 16.7 ms -> smooth
list scroll @60Hz: budget 16.7 ms, 37 fps, dropped 6 (38%), p95 50.1 ms, worst 50.1 ms -> janky
card flip @120Hz: budget 8.3 ms, 110 fps, dropped 1 (8%), p95 16.6 ms, worst 16.6 ms -> janky
@@requires
def analyse
1000 /
round(
@@hint
`budget = 1000 / hz`; intervals = differences of consecutive stamps. Effective FPS = number of intervals / total time × 1000. p95: sort the intervals and take the value at index `ceil(0.95 * n) - 1`. Dropped share = dropped / (frames shown + dropped).
@@solution
import math

recordings = {
    "menu open @60Hz": (60, [0, 16.7, 33.4, 50.1, 66.8, 83.5, 100.2, 116.9, 133.6, 150.3, 167.0, 183.7, 200.4]),
    "list scroll @60Hz": (60, [0, 16.7, 33.4, 83.5, 100.2, 116.9, 167.0, 183.7, 200.4, 250.5, 267.2]),
    "card flip @120Hz": (120, [0, 8.3, 16.7, 25.0, 33.3, 41.7, 58.3, 66.7, 75.0, 83.3, 91.7, 100.0]),
}

def analyse(hz, stamps):
    budget = 1000 / hz
    intervals = [b - a for a, b in zip(stamps, stamps[1:])]
    dropped = sum(max(0, round(i / budget) - 1) for i in intervals)
    fps = len(intervals) / (stamps[-1] - stamps[0]) * 1000
    ordered = sorted(intervals)
    p95 = ordered[math.ceil(0.95 * len(ordered)) - 1]
    share = dropped / (len(intervals) + dropped)
    smooth = share < 0.02 and p95 < 1.5 * budget
    return budget, dropped, fps, p95, max(intervals), share, smooth

for name, (hz, stamps) in recordings.items():
    budget, dropped, fps, p95, worst, share, smooth = analyse(hz, stamps)
    print(f"{name}: budget {budget:.1f} ms, {fps:.0f} fps, dropped {dropped} ({share:.0%}), p95 {p95:.1f} ms, worst {worst:.1f} ms -> {'smooth' if smooth else 'janky'}")

@@exercise Audit an animation spec
Each animation in `spec` lists the properties it animates, its duration, whether it loops, flashes per second and how large the moving area is. Flag properties that trigger layout or paint (suggest a compositor-only replacement), loops longer than 5 s without a pause control, flashing above 3 per second, and large movement without a reduced-motion alternative. Print an OK line for clean animations.
@@starter
spec = [
    {"name": "toast-in", "props": ["transform", "opacity"], "ms": 250, "loop_s": 0, "flashes": 0, "large": False, "reduced": "fade"},
    {"name": "sidebar", "props": ["width", "left"], "ms": 300, "loop_s": 0, "flashes": 0, "large": True, "reduced": None},
    {"name": "card-hover", "props": ["box-shadow", "transform"], "ms": 150, "loop_s": 0, "flashes": 0, "large": False, "reduced": None},
    {"name": "hero-bg", "props": ["transform"], "ms": 8000, "loop_s": 60, "flashes": 0, "large": True, "reduced": "static", "pausable": False},
    {"name": "alert-blink", "props": ["opacity"], "ms": 200, "loop_s": 10, "flashes": 5, "large": False, "reduced": None, "pausable": True},
]
LAYOUT = {"width", "height", "top", "left", "right", "bottom", "margin", "padding", "font-size"}
PAINT = {"color", "background-color", "box-shadow", "border-radius", "outline"}

def audit(anim):
    raise NotImplementedError
@@expected
toast-in: OK
sidebar: width triggers layout; use transform: scale
sidebar: left triggers layout; use transform: translate
sidebar: large movement needs a prefers-reduced-motion alternative
card-hover: box-shadow triggers paint; use opacity of a shadow layer
hero-bg: loops for 60 s without a pause control (WCAG 2.2.2)
alert-blink: 5 flashes per second exceeds 3 (WCAG 2.3.1)
@@requires
def audit
LAYOUT
prefers-reduced-motion
@@hint
Return a list of messages. Replacements: width/height → `transform: scale`, top/left/right/bottom/margin → `transform: translate`, box-shadow → opacity of a pre-rendered shadow layer, other paint properties → accept or cross-fade. Use `anim.get("pausable", False)`.
@@solution
spec = [
    {"name": "toast-in", "props": ["transform", "opacity"], "ms": 250, "loop_s": 0, "flashes": 0, "large": False, "reduced": "fade"},
    {"name": "sidebar", "props": ["width", "left"], "ms": 300, "loop_s": 0, "flashes": 0, "large": True, "reduced": None},
    {"name": "card-hover", "props": ["box-shadow", "transform"], "ms": 150, "loop_s": 0, "flashes": 0, "large": False, "reduced": None},
    {"name": "hero-bg", "props": ["transform"], "ms": 8000, "loop_s": 60, "flashes": 0, "large": True, "reduced": "static", "pausable": False},
    {"name": "alert-blink", "props": ["opacity"], "ms": 200, "loop_s": 10, "flashes": 5, "large": False, "reduced": None, "pausable": True},
]
LAYOUT = {"width", "height", "top", "left", "right", "bottom", "margin", "padding", "font-size"}
PAINT = {"color", "background-color", "box-shadow", "border-radius", "outline"}
REPLACE = {"width": "transform: scale", "height": "transform: scale", "box-shadow": "opacity of a shadow layer"}

def audit(anim):
    issues = []
    for prop in anim["props"]:
        if prop in LAYOUT:
            issues.append(f"{prop} triggers layout; use {REPLACE.get(prop, 'transform: translate')}")
        elif prop in PAINT:
            issues.append(f"{prop} triggers paint; use {REPLACE.get(prop, 'a cross-fade')}")
    if anim["loop_s"] > 5 and not anim.get("pausable", False):
        issues.append(f"loops for {anim['loop_s']} s without a pause control (WCAG 2.2.2)")
    if anim["flashes"] > 3:
        issues.append(f"{anim['flashes']} flashes per second exceeds 3 (WCAG 2.3.1)")
    if anim["large"] and not anim["reduced"]:
        issues.append("large movement needs a prefers-reduced-motion alternative")
    return issues

for anim in spec:
    issues = audit(anim)
    if not issues:
        print(f"{anim['name']}: OK")
    for issue in issues:
        print(f"{anim['name']}: {issue}")
