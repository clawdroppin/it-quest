@@lesson ua2-css-svg CSS and SVG Animation: Keyframes, Transitions and Path Morphing
@@concepts strings, lists, numbers
@@lang python
@@text
Before reaching for Lottie, check whether the browser can do it natively. CSS transitions and keyframes cover most micro-interactions with zero dependencies, the Web Animations API gives JavaScript control, and SVG can animate strokes and morph shapes. This lesson covers all of them and the rules that make path morphing work.

## CSS transitions

```css,norun
.button { transition: transform 150ms cubic-bezier(0.2, 0, 0, 1), background-color 150ms ease-out; }
.button:hover { transform: translateY(-2px); }
.button:active { transform: scale(0.97); }
```

A transition animates between two states when a property changes. List specific properties; `transition: all` animates things you didn't intend (and layout properties).

## CSS keyframes

```css,norun
@keyframes pop {
  0%   { transform: scale(0.6); opacity: 0; }
  60%  { transform: scale(1.08); opacity: 1; }
  100% { transform: scale(1); }
}
.badge { animation: pop 320ms ease-out both; }
```

`animation` shorthand: name, duration, timing function, delay, iteration count (`infinite`), direction (`alternate`), fill mode (`both` keeps the first/last frame), play state. Per-keyframe easing is allowed (`animation-timing-function` inside a keyframe applies to the segment that starts there).

## Web Animations API

`el.animate([{ transform: "scale(1)" }, { transform: "scale(1.1)" }], { duration: 200, easing: "ease-out", fill: "forwards" })` returns an `Animation` with `play()`, `pause()`, `reverse()`, `finished` (a promise) and `currentTime`. Good for state-driven buttons without a library.

## SVG animation

- **Stroke drawing**: set `stroke-dasharray` and `stroke-dashoffset` to the path length (`path.getTotalLength()`), then animate the offset to 0: the line draws itself. `pathLength="1"` on the element normalises the length to 1.
- **Transforms** on groups (`transform-box: fill-box; transform-origin: center` so they rotate around their own centre).
- **Path morphing**: interpolating the `d` attribute (CSS `d: path(...)` in Chromium and Firefox, or JS/SMIL). It works only when both paths have the **same commands in the same order** with the same number of points; otherwise tools like flubber resample the paths first. Convert both to absolute cubic Béziers (`C`) for the most robust result.
- **SMIL** (`<animate>` inside SVG) still works in browsers and in some icon formats, but CSS/JS are usually easier to control.

## View transitions

`document.startViewTransition(() => updateDom())` cross-fades or morphs between DOM states; `view-transition-name` pairs elements across states. Same-document transitions are widely supported; cross-document ones are arriving.

@@quiz
Why avoid `transition: all`?
- It's invalid CSS
* It animates unintended properties, including expensive layout ones
- It only works in Chrome
- It disables easing
? Name the properties you mean.

@@quiz
What does `animation-fill-mode: both` do?
* Applies the first keyframe before the start and keeps the last keyframe after the end
- Plays the animation twice
- Plays forwards then backwards
- Fills the SVG shape
? Avoids a jump at the start and end.

@@quiz
How do you make an SVG line appear to draw itself?
- Animate its width
* Animate stroke-dashoffset from the path length to 0
- Animate opacity per point
- Use a GIF
? With stroke-dasharray equal to the length.

@@quiz
Two paths have different command sequences. What's needed to morph between them?
- Nothing; browsers handle it
* Normalise or resample them to the same commands and point count
- Convert them to PNG
- Use steps() easing
? Interpolation works point by point.

@@quiz
Which API animates an element from JavaScript and returns a controllable object?
- requestAnimationFrame only
* The Web Animations API (`element.animate`)
- setInterval
- CSS @property
? It has play, pause, reverse and finished.

@@flashcards
transition :: animate between two states
@keyframes :: multi-step animation
fill-mode both :: hold first and last frames
iteration-count infinite :: loop forever
direction alternate :: back and forth
element.animate() :: Web Animations API
animation.finished :: promise when done
stroke-dasharray :: dash pattern
stroke-dashoffset :: shift the dashes
pathLength="1" :: normalised length
transform-box: fill-box :: transform around own box
path morph rule :: same commands, same points
SMIL :: <animate> inside SVG
view transitions :: animate DOM state changes

@@drill
Which CSS property shifts the start of a dash pattern to make a line draw itself?
= stroke-dashoffset
? Paired with stroke-dasharray.

@@drill
Which JavaScript method starts a Web Animations API animation on an element?
= animate
= element.animate
= el.animate
= .animate()
? It returns an Animation object.

@@exercise Generate CSS keyframes
Write `keyframes(name, frames)` that turns a list of `(time_ms, props, easing)` tuples into a `@keyframes` block with percentages (rounded to 2 decimals, trailing zeros removed) and per-segment `animation-timing-function`, plus the `animation` shorthand line for a selector. Reject frames that aren't in time order.
@@starter
pop = [
    (0, {"transform": "scale(0.6)", "opacity": "0"}, "cubic-bezier(0.2, 0, 0, 1)"),
    (190, {"transform": "scale(1.08)", "opacity": "1"}, "ease-in-out"),
    (320, {"transform": "scale(1)"}, None),
]
shake = [
    (0, {"transform": "translateX(0)"}, None),
    (100, {"transform": "translateX(-6px)"}, None),
    (50, {"transform": "translateX(6px)"}, None),
]

def keyframes(name, frames):
    raise NotImplementedError
@@expected
@keyframes pop {
  0% { transform: scale(0.6); opacity: 0; animation-timing-function: cubic-bezier(0.2, 0, 0, 1); }
  59.38% { transform: scale(1.08); opacity: 1; animation-timing-function: ease-in-out; }
  100% { transform: scale(1); }
}
.pop { animation: pop 320ms both; }
error: shake: frames out of order [0, 100, 50]
@@requires
def keyframes
@keyframes
animation-timing-function
@@hint
Total = the last frame's time. Percentage `f"{t / total * 100:.2f}".rstrip("0").rstrip(".")`. Raise `ValueError` if any time is smaller than the previous. Return the CSS text; the animation line is `animation: name {total}ms both;`.
@@solution
pop = [
    (0, {"transform": "scale(0.6)", "opacity": "0"}, "cubic-bezier(0.2, 0, 0, 1)"),
    (190, {"transform": "scale(1.08)", "opacity": "1"}, "ease-in-out"),
    (320, {"transform": "scale(1)"}, None),
]
shake = [
    (0, {"transform": "translateX(0)"}, None),
    (100, {"transform": "translateX(-6px)"}, None),
    (50, {"transform": "translateX(6px)"}, None),
]

def pct(t, total):
    return f"{t / total * 100:.2f}".rstrip("0").rstrip(".") + "%"

def keyframes(name, frames):
    times = [t for t, _, _ in frames]
    if any(b < a for a, b in zip(times, times[1:])):
        raise ValueError(f"{name}: frames out of order {times}")
    total = times[-1]
    lines = [f"@keyframes {name} {{"]
    for t, props, easing in frames:
        body = " ".join(f"{k}: {v};" for k, v in props.items())
        if easing:
            body += f" animation-timing-function: {easing};"
        lines.append(f"  {pct(t, total)} {{ {body} }}")
    lines.append("}")
    lines.append(f".{name} {{ animation: {name} {total}ms both; }}")
    return "\n".join(lines)

for name, frames in (("pop", pop), ("shake", shake)):
    try:
        print(keyframes(name, frames))
    except ValueError as e:
        print("error:", e)

@@exercise Morph between two SVG paths
Parse absolute SVG path data made of `M`, `L`, `C` and `Z` commands into (command, points) pairs. Check whether two paths are morph-compatible (same command sequence), and if so print the interpolated `d` at t = 0, 0.5 and 1. If not, say where they differ. Numbers in the output use at most 2 decimals.
@@starter
import re

shapes = {
    "play": "M 8 5 L 19 12 L 8 19 L 8 5 Z",
    "pause": "M 6 5 L 10 5 L 10 19 L 6 19 Z",
    "square": "M 6 6 L 18 6 L 18 18 L 6 18 Z",
    "blob": "M 12 4 C 18 4 20 10 20 12 C 20 18 14 20 12 20 Z",
}

def parse(d):
    raise NotImplementedError
@@expected
play -> pause:
  t=0: M 8 5 L 19 12 L 8 19 L 8 5 Z
  t=0.5: M 7 5 L 14.5 8.5 L 9 19 L 7 12 Z
  t=1: M 6 5 L 10 5 L 10 19 L 6 19 Z
pause -> square:
  t=0: M 6 5 L 10 5 L 10 19 L 6 19 Z
  t=0.5: M 6 5.5 L 14 5.5 L 14 18.5 L 6 18.5 Z
  t=1: M 6 6 L 18 6 L 18 18 L 6 18 Z
square -> blob: not compatible (command 1: L vs C); resample first
@@requires
def parse
re.findall(
def compatible
@@hint
Tokenise with `re.findall(r"[MLCZ]|-?\d+(?:\.\d+)?", d)`. Walk the tokens: a letter starts a new command; numbers after it go into its list. Group numbers into (x, y) pairs. Format numbers with `f"{v:.2f}".rstrip("0").rstrip(".")`.
@@solution
import re

shapes = {
    "play": "M 8 5 L 19 12 L 8 19 L 8 5 Z",
    "pause": "M 6 5 L 10 5 L 10 19 L 6 19 Z",
    "square": "M 6 6 L 18 6 L 18 18 L 6 18 Z",
    "blob": "M 12 4 C 18 4 20 10 20 12 C 20 18 14 20 12 20 Z",
}

def parse(d):
    commands = []
    for token in re.findall(r"[MLCZ]|-?\d+(?:\.\d+)?", d):
        if token.isalpha():
            commands.append((token, []))
        else:
            commands[-1][1].append(float(token))
    return [(cmd, list(zip(nums[0::2], nums[1::2]))) for cmd, nums in commands]

def compatible(a, b):
    for i, ((ca, pa), (cb, pb)) in enumerate(zip(a, b)):
        if ca != cb or len(pa) != len(pb):
            return False, f"command {i}: {ca} vs {cb}"
    if len(a) != len(b):
        return False, f"{len(a)} vs {len(b)} commands"
    return True, ""

def num(v):
    return f"{v:.2f}".rstrip("0").rstrip(".")

def interpolate(a, b, t):
    parts = []
    for (cmd, pa), (_, pb) in zip(a, b):
        coords = " ".join(f"{num(x1 + (x2 - x1) * t)} {num(y1 + (y2 - y1) * t)}" for (x1, y1), (x2, y2) in zip(pa, pb))
        parts.append(f"{cmd} {coords}".strip())
    return " ".join(parts)

for a, b in (("play", "pause"), ("pause", "square"), ("square", "blob")):
    pa, pb = parse(shapes[a]), parse(shapes[b])
    ok, why = compatible(pa, pb)
    if not ok:
        print(f"{a} -> {b}: not compatible ({why}); resample first")
        continue
    print(f"{a} -> {b}:")
    for t in (0, 0.5, 1):
        print(f"  t={t}: {interpolate(pa, pb, t)}")
