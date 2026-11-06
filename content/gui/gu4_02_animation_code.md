@@lesson gu4-animation-code Animating in Code: Easing Functions, Tweens and Desktop Animation
@@concepts functions, numbers, loops
@@lang python
@@text
CSS hides the maths, but desktop toolkits like tkinter, WinForms and games don't have `transition`. There you animate **by hand**: change a value a little on every frame. Once you understand how that works, you understand every animation system, including CSS, WPF storyboards and game engines.

## An animation is a function of time

Every animation answers one question: *at time t, what is the value?*

1. **Progress**: `p = elapsed / duration`, clamped to 0..1.
2. **Easing**: `e = ease(p)` bends the progress (still 0 at the start and 1 at the end, but not a straight line).
3. **Interpolation** ("lerp"): `value = start + (end - start) * e`.

```python
def lerp(a, b, t):
    return a + (b - a) * t

def ease_out_quad(p):
    return 1 - (1 - p) ** 2

# A box sliding from x=0 to x=200 over 10 frames
for frame in range(11):
    p = frame / 10
    print(f"frame {frame:2}: linear {lerp(0, 200, p):6.1f}   ease-out {lerp(0, 200, ease_out_quad(p)):6.1f}")
```

See how ease-out covers most of the distance early and creeps into place at the end: fast start, gentle stop.

## The classic easing functions

```python
import math

def linear(p):          return p
def ease_in_cubic(p):   return p ** 3
def ease_out_cubic(p):  return 1 - (1 - p) ** 3
def ease_in_out_cubic(p):
    return 4 * p ** 3 if p < 0.5 else 1 - (-2 * p + 2) ** 3 / 2

def ease_out_back(p, s=1.70158):            # overshoots, then settles: bouncy!
    return 1 + (s + 1) * (p - 1) ** 3 + s * (p - 1) ** 2

def ease_out_elastic(p):                     # springs back and forth
    if p in (0, 1):
        return p
    return 2 ** (-10 * p) * math.sin((p * 10 - 0.75) * (2 * math.pi / 3)) + 1

for p in (0, 0.25, 0.5, 0.75, 1):
    print(f"p={p:<4}  cubic-out {ease_out_cubic(p):.2f}  back-out {ease_out_back(p):.2f}  elastic {ease_out_elastic(p):.2f}")
```

`ease_out_back` goes past 1.0 (to about 1.1) before settling: that's the CSS `cubic-bezier(.34, 1.56, .64, 1)` feeling. Robert Penner wrote the classic set of these functions in 2001, and every animation library still uses them; easings.net shows them all with graphs.

## Animating a tkinter window

tkinter's `after(ms, fn)` schedules a function on the event loop. Call it again from inside the function and you have an **animation loop**, without ever blocking the window:

```python,norun
import time, tkinter as tk

root = tk.Tk()
canvas = tk.Canvas(root, width=320, height=120, bg="#fdf4ff", highlightthickness=0)
canvas.pack()
ball = canvas.create_oval(10, 40, 50, 80, fill="#ec4899", outline="")

def animate(item, x0, x1, duration=0.6, ease=ease_out_back):
    start = time.perf_counter()
    def frame():
        p = min((time.perf_counter() - start) / duration, 1.0)   # use the clock, not frame counts
        x = lerp(x0, x1, ease(p))
        canvas.coords(item, x, 40, x + 40, 80)
        if p < 1:
            root.after(16, frame)                                # about 60 frames per second
    frame()

canvas.bind("<Button-1>", lambda e: animate(ball, 10, e.x - 20))
root.mainloop()
```

Two important habits:

- **Use the real clock** (`time.perf_counter()`), not "move 5 pixels per frame". Frames arrive late when the computer is busy; time-based animation still finishes on time, frame-based animation slows down.
- **Cancel the old animation** when a new one starts on the same thing (keep the id from `after` and call `after_cancel`), or two loops fight over the value.

## The same idea elsewhere

```csharp,norun
// WPF: a Storyboard-free animation in code
var grow = new DoubleAnimation(1.0, 1.15, TimeSpan.FromMilliseconds(180))
{
    AutoReverse = true,
    EasingFunction = new BackEase { EasingMode = EasingMode.EaseOut, Amplitude = 0.5 },
};
likeButton.RenderTransform = new ScaleTransform();
likeButton.RenderTransformOrigin = new Point(0.5, 0.5);
likeButton.RenderTransform.BeginAnimation(ScaleTransform.ScaleXProperty, grow);
likeButton.RenderTransform.BeginAnimation(ScaleTransform.ScaleYProperty, grow);
```

```xml
<!-- WPF XAML: a storyboard that fades a panel in when it loads -->
<Border x:Name="Panel" Opacity="0">
  <Border.Triggers>
    <EventTrigger RoutedEvent="Loaded">
      <BeginStoryboard>
        <Storyboard>
          <DoubleAnimation Storyboard.TargetProperty="Opacity" To="1" Duration="0:0:0.3">
            <DoubleAnimation.EasingFunction><CubicEase EasingMode="EaseOut" /></DoubleAnimation.EasingFunction>
          </DoubleAnimation>
        </Storyboard>
      </BeginStoryboard>
    </EventTrigger>
  </Border.Triggers>
</Border>
```

```js,norun
// The browser's animation loop: requestAnimationFrame runs once per screen refresh
function animate(el, from, to, duration, ease) {
  const start = performance.now();
  requestAnimationFrame(function frame(now) {
    const p = Math.min((now - start) / duration, 1);
    el.style.transform = `translateX(${from + (to - from) * ease(p)}px)`;
    if (p < 1) requestAnimationFrame(frame);
  });
}
```

Qt has `QPropertyAnimation` with `QEasingCurve.OutBack`; WinForms has no animation system, so you use a `Timer` and the maths above.

## Springs: physics instead of curves

Modern UI libraries (Framer Motion, SwiftUI, React Spring) often use **springs**: instead of a fixed duration, the value is pulled towards its target by a spring and slowed by friction. Each frame:

```python
def spring_step(x, v, target, stiffness=170, damping=26, dt=1 / 60):
    force = -stiffness * (x - target) - damping * v
    v += force * dt
    x += v * dt
    return x, v

x, v = 0.0, 0.0
for frame in range(0, 40, 5):
    for _ in range(5):
        x, v = spring_step(x, v, 100)
    print(f"after {frame + 5:2} frames: x = {x:6.1f}")
```

Springs feel natural, and they handle interruptions gracefully: if the target changes mid-animation, the current velocity carries over smoothly.

## Animating colours

Colours animate by interpolating each channel: red, green and blue separately (or better, in a perceptual space like OKLCH, which CSS does for you with `color-mix(in oklch, ...)`).

@@quiz
Which three steps turn time into an animated value?
- Draw, wait, erase
* Progress (elapsed / duration), easing (bend it), interpolation (start + (end - start) × eased)
- Start, stop, reverse
- Frame count × speed
? Every animation system, from CSS to game engines, works this way.

@@quiz
Why base an animation on the clock instead of moving a fixed number of pixels per frame?
- Clocks are more accurate than pixels
* Frames can arrive late; time-based animation still finishes on time
- Pixels can't be fractions
- `after()` requires it
? Frame-based animation slows down on a busy computer.

@@quiz
What makes `ease_out_back` feel bouncy?
- It's slower
* It goes past the target (above 1.0) and then settles back
- It repeats
- It uses random numbers
? Overshoot is the key ingredient of playful motion.

@@quiz
In tkinter, how do you run an animation without freezing the window?
- A `while` loop with `time.sleep(0.016)`
* A function that updates one frame and reschedules itself with `root.after(16, frame)`
- A second `mainloop()`
- `canvas.animate()`
? A sleep loop blocks the event loop; `after` lets it keep handling events between frames.

@@quiz
A spring animation has high stiffness and low damping. How does it move?
* Fast, and it overshoots and oscillates before settling
- Slowly, with no overshoot
- Linearly, at constant speed
- It never reaches the target
? Damping removes energy; with little of it the value bounces around the target.

@@flashcards
progress :: `p = elapsed / duration`, clamped to 0..1
lerp :: `a + (b - a) * t`: the value between a and b
easing function :: bends progress: still 0→1, but not a straight line
`ease_out_cubic` :: `1 - (1 - p) ** 3`: fast start, gentle stop
`ease_out_back` :: overshoots past 1 and settles: bouncy
time-based animation :: use the clock so it finishes on time even when frames drop
`root.after(16, frame)` :: a tkinter animation loop at about 60 fps
`after_cancel(id)` :: stop an old animation before starting a new one
`requestAnimationFrame` :: the browser's once-per-refresh callback
WPF `DoubleAnimation` + `BackEase` :: animate a property in code with easing
Storyboard :: WPF's container for timed animations in XAML
`QPropertyAnimation` :: Qt's property animation with `QEasingCurve`
spring animation :: pulled to the target by stiffness, slowed by damping
Robert Penner's easing equations :: the classic set used by almost every library

@@exercise Easing functions
Write three easing functions. Each takes progress `p` from 0 to 1 and returns the eased value, which is 0 at `p = 0` and 1 at `p = 1`.

- `ease_in_quad(p)`: `p` squared (slow start)
- `ease_out_quad(p)`: `1 - (1 - p)` squared (slow end)
- `ease_in_out_quad(p)`: below `0.5` it's `2 * p * p`; from `0.5` on it's `1 - (-2 * p + 2) ** 2 / 2`

Also write `clamp01(x)`, which limits any number to the range 0..1, because real timers overshoot.
@@starter
def clamp01(x):
    pass


def ease_in_quad(p):
    pass


def ease_out_quad(p):
    pass


def ease_in_out_quad(p):
    pass


for p in (0, 0.25, 0.5, 0.75, 1):
    print(p, ease_in_quad(p), ease_out_quad(p), ease_in_out_quad(p))
@@tests
check(clamp01(-0.2), 0, "clamps below 0")
check(clamp01(1.7), 1, "clamps above 1")
check(clamp01(0.4), 0.4, "leaves 0..1 alone")
check([ease_in_quad(p) for p in (0, 0.5, 1)], [0, 0.25, 1], "ease in")
check([ease_out_quad(p) for p in (0, 0.5, 1)], [0, 0.75, 1], "ease out")
check([ease_in_out_quad(p) for p in (0, 0.25, 0.5, 0.75, 1)], [0, 0.125, 0.5, 0.875, 1], "ease in-out")
@@hint
`clamp01` is `max(0, min(1, x))`. `ease_out_quad` is `1 - (1 - p) ** 2`.
@@solution
def clamp01(x):
    return max(0, min(1, x))


def ease_in_quad(p):
    return p * p


def ease_out_quad(p):
    return 1 - (1 - p) ** 2


def ease_in_out_quad(p):
    if p < 0.5:
        return 2 * p * p
    return 1 - (-2 * p + 2) ** 2 / 2


for p in (0, 0.25, 0.5, 0.75, 1):
    print(p, ease_in_quad(p), ease_out_quad(p), ease_in_out_quad(p))

@@exercise Tween frames
Write `tween(start, end, duration_ms, fps, ease)` that returns the list of values to draw, one per frame, **rounded to 1 decimal place**. There are `round(duration_ms / 1000 * fps)` frames after the start, so the list has that many plus one values: the first is exactly `start` and the last exactly `end`. Frame `i` has progress `i / frames`.
@@starter
def ease_out_quad(p):
    return 1 - (1 - p) ** 2


def tween(start, end, duration_ms, fps, ease):
    pass


print(tween(0, 100, 100, 40, ease_out_quad))
@@tests
lin = lambda p: p
check(tween(0, 100, 100, 40, lin), [0, 25.0, 50.0, 75.0, 100.0], "linear, 4 frames")
check(tween(0, 100, 100, 40, ease_out_quad), [0, 43.8, 75.0, 93.8, 100.0], "ease out")
check(tween(200, 0, 50, 60, lin), [200, 133.3, 66.7, 0.0], "moving backwards, 3 frames")
check(len(tween(0, 1, 1000, 60, lin)), 61, "one second at 60 fps")
@@hint
`frames = round(duration_ms / 1000 * fps)`, then `[round(start + (end - start) * ease(i / frames), 1) for i in range(frames + 1)]`.
@@solution
def ease_out_quad(p):
    return 1 - (1 - p) ** 2


def tween(start, end, duration_ms, fps, ease):
    frames = round(duration_ms / 1000 * fps)
    return [round(start + (end - start) * ease(i / frames), 1) for i in range(frames + 1)]


print(tween(0, 100, 100, 40, ease_out_quad))

@@exercise Fade between two colours
Write `mix(c1, c2, t)` that blends two hex colours like `"#ff0000"` and `"#0000ff"`: at `t = 0` it's `c1`, at `t = 1` it's `c2`, and in between each of red, green and blue is interpolated separately and **rounded** to a whole number. Return lower-case hex `"#rrggbb"`. Then `gradient(c1, c2, steps)` returns `steps` colours from `c1` to `c2` inclusive (for a fade animation, or a gradient progress bar).
@@starter
def mix(c1, c2, t):
    pass


def gradient(c1, c2, steps):
    pass


print(mix("#000000", "#ffffff", 0.5))
print(gradient("#ec4899", "#8b5cf6", 3))
@@tests
check(mix("#000000", "#ffffff", 0), "#000000", "t = 0 is the first colour")
check(mix("#000000", "#ffffff", 1), "#ffffff", "t = 1 is the second colour")
check(mix("#000000", "#ffffff", 0.5), "#808080", "halfway is grey (127.5 rounds to 128)")
check(mix("#FF0000", "#0000FF", 0.25), "#bf0040", "upper-case input, quarter way")
check(gradient("#ec4899", "#8b5cf6", 3), ["#ec4899", "#bc52c8", "#8b5cf6"], "a three-step gradient")
check(len(gradient("#000000", "#ffffff", 5)), 5, "five steps")
@@hint
Parse with `int(c[1:3], 16)`, `int(c[3:5], 16)`, `int(c[5:7], 16)`. Blend each with `round(a + (b - a) * t)`. Format with `f"#{r:02x}{g:02x}{b:02x}"`. In `gradient`, use `t = i / (steps - 1)`.
@@solution
def _rgb(c):
    return [int(c[i:i + 2], 16) for i in (1, 3, 5)]


def mix(c1, c2, t):
    r, g, b = (round(a + (b - a) * t) for a, b in zip(_rgb(c1), _rgb(c2)))
    return f"#{r:02x}{g:02x}{b:02x}"


def gradient(c1, c2, steps):
    return [mix(c1, c2, i / (steps - 1)) for i in range(steps)]


print(mix("#000000", "#ffffff", 0.5))
print(gradient("#ec4899", "#8b5cf6", 3))
