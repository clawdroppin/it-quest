@@lesson ru2-buttons Interactive Animated Buttons in Rust
@@concepts numbers, functions, objects
@@lang rust
@@text
Animation is what makes an interface feel alive: a button that eases to a lighter colour on hover, sinks slightly when pressed, and springs back when released feels *physical*, and users trust it more. In Rust you have full control: you can draw any shape and animate any value. This lesson covers the maths of motion, then builds a custom animated button in egui, and shows how iced and Slint do the same.

## Rules for UI motion

- **Fast**: hover 100–150 ms, press 60–100 ms, panels and dialogs 200–300 ms. Slower feels sluggish.
- **Eased**, never linear: things *start quickly and settle gently* (ease-out) when they react to the user.
- **Interruptible**: if the pointer leaves mid-animation, reverse smoothly from where it is, don't jump.
- **Purposeful**: motion shows cause and effect (this button reacted, this panel came from there). No decoration that slows people down.
- **Respect reduced motion**: offer a setting (and follow the OS where the toolkit exposes it) to turn animations off.

## Easing functions

An easing function maps time progress `t` (0 to 1) to animation progress:

| Name | Formula | Feel |
|---|---|---|
| linear | `t` | mechanical |
| ease-in (quad) | `t²` | slow start: things *leaving* |
| ease-out (cubic) | `1 − (1 − t)³` | fast start, gentle stop: **most UI reactions** |
| ease-in-out (cubic) | `t < 0.5 ? 4t³ : 1 − (−2t + 2)³ / 2` | moving between two places |
| back-out | overshoots slightly, then settles | playful, bouncy buttons |
| spring | physics: stiffness, damping | natural, interruptible |

The value at any moment is `start + (end − start) × ease(t)`: **linear interpolation** (lerp) of the eased progress. The first exercise implements these.

## An animated value

Every animation needs a value that **moves towards a target** over time and can change direction at any moment. Each frame you advance it by the elapsed time `dt`:

```text
progress += dt / duration   (towards 1 if the target is "on", towards 0 if "off")
shown = ease(progress)
```

Because `progress` is stored, reversing halfway simply starts counting down from where it is. This is exactly how egui's `ctx.animate_bool_with_time(id, on, seconds)` works, and the second exercise builds it.

## Springs

A **spring** gives motion that feels physical and handles interruptions perfectly: the value accelerates towards the target, overshoots a little (if under-damped), and settles.

```text
force    = stiffness × (target − position) − damping × velocity
velocity += force × dt
position += velocity × dt
```

Run this every frame with a small `dt` (or several fixed sub-steps). Stiffness ~170–300 and damping ~20–26 make snappy UI springs. The third exercise steps one.

## A custom animated button in egui

egui lets you allocate space, read the interaction, and paint anything:

```rust,norun
use eframe::egui::{self, Color32, CornerRadius, Sense, Stroke, StrokeKind, Vec2};

/// A pill button that lifts on hover and sinks when pressed.
fn fancy_button(ui: &mut egui::Ui, label: &str, accent: Color32) -> egui::Response {
    let size = Vec2::new(160.0, 44.0);
    let (rect, response) = ui.allocate_exact_size(size, Sense::click());
    let ctx = ui.ctx();
    // 0..1 values that animate towards the current state.
    let hover = ctx.animate_bool_with_time(response.id.with("hover"), response.hovered(), 0.12);
    let press = ctx.animate_bool_with_time(response.id.with("press"), response.is_pointer_button_down_on(), 0.06);

    if ui.is_rect_visible(rect) {
        let painter = ui.painter();
        let lift = 2.0 * hover - 2.0 * press;                  // px upwards
        let scale = 1.0 + 0.03 * hover - 0.05 * press;
        let r = egui::Rect::from_center_size(rect.center() - Vec2::new(0.0, lift), rect.size() * scale);

        // Soft shadow that grows with the lift.
        let shadow = egui::epaint::Shadow { offset: [0, (2.0 + 4.0 * hover) as i8], blur: (6.0 + 10.0 * hover) as u8, spread: 0, color: Color32::from_black_alpha(60) };
        painter.add(shadow.as_shape(r, CornerRadius::same(22)));

        // Colour: lighter on hover, darker when pressed.
        let fill = lerp_color(lerp_color(accent, Color32::WHITE, 0.12 * hover), Color32::BLACK, 0.18 * press);
        painter.rect(r, CornerRadius::same(22), fill, Stroke::NONE, StrokeKind::Inside);
        painter.text(r.center(), egui::Align2::CENTER_CENTER, label, egui::FontId::proportional(16.0), Color32::WHITE);

        if response.has_focus() {
            painter.rect_stroke(r.expand(3.0), CornerRadius::same(25), Stroke::new(2.0, accent), StrokeKind::Outside);
        }
    }
    response.on_hover_cursor(egui::CursorIcon::PointingHand)
}

fn lerp_color(a: Color32, b: Color32, t: f32) -> Color32 {
    let l = |x: u8, y: u8| (x as f32 + (y as f32 - x as f32) * t).round() as u8;
    Color32::from_rgb(l(a.r(), b.r()), l(a.g(), b.g()), l(a.b(), b.b()))
}
```

Use it like any widget: `if fancy_button(ui, "Save", accent).clicked() { ... }`. egui only repaints while something is animating, so idle apps use no CPU.

More effects worth building: a **ripple** (a circle that expands from the click point and fades: store the click position and time, draw while `age < 0.4 s`), a **loading state** (swap the label for a spinner with `ui.add(egui::Spinner::new())` or a rotating arc), a **toggle switch** (animate the knob's x with `animate_bool`), and **success feedback** (briefly morph the button green with a check mark).

## The same in iced and Slint

- **iced**: style closures receive the button `Status` (Active, Hovered, Pressed, Disabled); for smooth transitions keep an animated value in your state and drive it from a frame subscription (`iced::window::frames()`), or use iced's `Animation` helpers in newer releases.
- **Slint** has animation built into the language:

```text
Rectangle {
    background: touch.pressed ? #4f46e5 : touch.has-hover ? #818cf8 : #6366f1;
    animate background { duration: 150ms; easing: ease-out; }
    y: touch.pressed ? 2px : 0px;
    animate y { duration: 80ms; easing: ease-out; }
    touch := TouchArea { clicked => { root.clicked(); } }
}
```

- **Tauri**: CSS transitions and keyframes, exactly as in the GUI Design course.

@@quiz
Which easing suits a button reacting to hover?
- linear
- ease-in (slow start)
* ease-out: fast start, gentle stop
- a 2-second ease-in-out
? Reactions should start immediately and settle softly.

@@quiz
The pointer leaves a button halfway through its hover animation. What should happen?
* The animation reverses smoothly from its current value
- It finishes the hover animation first, then reverses
- It jumps straight back to idle
- Nothing; hover animations can't be interrupted
? Storing progress (or using a spring) makes reversal natural.

@@quiz
How long should a hover transition take?
- 1–2 seconds
* About 100–150 ms
- 1 ms
- Exactly one frame
? Panels and dialogs take a bit longer, 200–300 ms.

@@quiz
What does `ctx.animate_bool_with_time(id, on, 0.12)` return in egui?
- true or false
* A number from 0 to 1 that moves towards 1 when `on` is true (or 0 when false) over 0.12 s
- The number of frames left
- A new Response
? Use it to blend colours, sizes and positions.

@@drill
Which physical model gives motion a natural overshoot and settle?
= spring
= springs
= a spring
? Stiffness and damping control the feel.

@@flashcards
ease-out :: 1 − (1 − t)³, fast start, gentle stop
ease-in :: slow start, for things leaving
ease-in-out :: moving between two places
lerp :: a + (b − a) × t
animated value :: progress stored and advanced by dt towards a target
spring :: stiffness pulls to the target, damping slows it
hover / press timings :: about 120 ms / 80 ms
reduced motion :: setting that turns animations off
allocate_exact_size + Sense::click :: custom egui widget with interaction
ui.painter() :: draw shapes, text and shadows
animate_bool_with_time :: egui animated 0..1 value
Slint animate :: animation of a property built into the language
ripple :: expanding, fading circle from the click point

@@exercise Easing functions
Write `lerp(a, b, t)`, `ease_out_cubic(t)`, `ease_in_quad(t)`, `ease_in_out_cubic(t)` (formulas in the lesson) and `ease_out_back(t)`: `1 + c3 (t − 1)³ + c1 (t − 1)²` with `c1 = 1.70158` and `c3 = c1 + 1`. All take `t` clamped to 0–1 first.
@@starter
fn lerp(a: f64, b: f64, t: f64) -> f64 { todo!() }
fn ease_in_quad(t: f64) -> f64 { todo!() }
fn ease_out_cubic(t: f64) -> f64 { todo!() }
fn ease_in_out_cubic(t: f64) -> f64 { todo!() }
fn ease_out_back(t: f64) -> f64 { todo!() }

fn main() {
    for i in 0..=10 {
        let t = i as f64 / 10.0;
        let bar = "#".repeat((ease_out_cubic(t) * 30.0).round() as usize);
        println!("{t:.1} {bar}");
    }
}
@@tests
#[test]
fn checks() {
    let close = |a: f64, b: f64| (a - b).abs() < 1e-3;
    assert!(close(lerp(10.0, 20.0, 0.25), 12.5), "lerp");
    assert!(close(ease_in_quad(0.5), 0.25), "ease in");
    assert!(close(ease_out_cubic(0.5), 0.875), "ease out at half time is already 87.5%");
    assert!(close(ease_in_out_cubic(0.25), 0.0625), "in-out first half");
    assert!(close(ease_in_out_cubic(0.75), 0.9375), "in-out second half");
    assert!(close(ease_in_out_cubic(0.5), 0.5), "in-out middle");
    for f in [ease_in_quad, ease_out_cubic, ease_in_out_cubic, ease_out_back] {
        assert!(close(f(0.0), 0.0) && close(f(1.0), 1.0), "every easing starts at 0 and ends at 1");
        assert!(close(f(-1.0), 0.0) && close(f(2.0), 1.0), "t is clamped");
    }
    assert!(ease_out_back(0.7) > 1.0, "back overshoots");
}
@@hint
Start each easing with `let t = t.clamp(0.0, 1.0);`. `(1.0 - t).powi(3)`; for in-out, `if t < 0.5 { 4.0 * t * t * t } else { 1.0 - (-2.0 * t + 2.0).powi(3) / 2.0 }`.
@@solution
fn lerp(a: f64, b: f64, t: f64) -> f64 { a + (b - a) * t }
fn ease_in_quad(t: f64) -> f64 { let t = t.clamp(0.0, 1.0); t * t }
fn ease_out_cubic(t: f64) -> f64 { let t = t.clamp(0.0, 1.0); 1.0 - (1.0 - t).powi(3) }
fn ease_in_out_cubic(t: f64) -> f64 {
    let t = t.clamp(0.0, 1.0);
    if t < 0.5 { 4.0 * t * t * t } else { 1.0 - (-2.0 * t + 2.0).powi(3) / 2.0 }
}
fn ease_out_back(t: f64) -> f64 {
    let t = t.clamp(0.0, 1.0);
    let c1 = 1.70158;
    let c3 = c1 + 1.0;
    1.0 + c3 * (t - 1.0).powi(3) + c1 * (t - 1.0).powi(2)
}

fn main() {
    for i in 0..=10 {
        let t = i as f64 / 10.0;
        let bar = "#".repeat((ease_out_cubic(t) * 30.0).round() as usize);
        println!("{t:.1} {bar}");
    }
}

@@exercise An interruptible animated value
Build `Anim { progress, duration, on }` (like egui's `animate_bool`). `new(duration)` starts off at progress 0. `set(on)` changes the target. `update(dt)` moves `progress` towards 1 (if on) or 0 (if off) by `dt / duration`, clamped, and returns the eased value `ease_out_cubic(progress)` (given). `is_animating()` is true while progress hasn't reached its target.
@@starter
fn ease_out_cubic(t: f64) -> f64 { let t = t.clamp(0.0, 1.0); 1.0 - (1.0 - t).powi(3) }

struct Anim { progress: f64, duration: f64, on: bool }

impl Anim {
    fn new(duration: f64) -> Self { todo!() }
    fn set(&mut self, on: bool) { todo!() }
    fn update(&mut self, dt: f64) -> f64 { todo!() }
    fn is_animating(&self) -> bool { todo!() }
}

fn main() {
    let mut hover = Anim::new(0.12);
    hover.set(true);
    for frame in 0..10 {
        if frame == 5 { hover.set(false); println!("-- pointer left --"); }
        println!("frame {frame}: {:.3}", hover.update(1.0 / 60.0));
    }
}
@@tests
#[test]
fn checks() {
    let close = |a: f64, b: f64| (a - b).abs() < 1e-9;
    let mut a = Anim::new(0.1);
    assert!(!a.is_animating(), "starts at rest");
    a.set(true);
    assert!(a.is_animating(), "animating after set");
    assert!(close(a.update(0.05), 0.875), "half the time: eased 0.875");
    assert!(close(a.progress, 0.5), "progress is linear");
    a.set(false);
    assert!(close(a.update(0.025), ease_out_cubic(0.25)), "reverses from where it was");
    a.set(true);
    a.update(1.0);
    assert!(close(a.progress, 1.0) && !a.is_animating(), "clamped at 1 and finished");
    a.set(false);
    a.update(5.0);
    assert!(close(a.progress, 0.0), "clamped at 0");
    let mut z = Anim::new(0.0);
    z.set(true);
    assert!(close(z.update(0.0), 1.0), "zero duration jumps");
}
@@hint
In `update`: `let step = if self.duration <= 0.0 { 1.0 } else { dt / self.duration };` then add or subtract it depending on `self.on` and `.clamp(0.0, 1.0)`. The target is `if self.on { 1.0 } else { 0.0 }`.
@@solution
fn ease_out_cubic(t: f64) -> f64 { let t = t.clamp(0.0, 1.0); 1.0 - (1.0 - t).powi(3) }

struct Anim { progress: f64, duration: f64, on: bool }

impl Anim {
    fn new(duration: f64) -> Self { Anim { progress: 0.0, duration, on: false } }
    fn set(&mut self, on: bool) { self.on = on; }
    fn update(&mut self, dt: f64) -> f64 {
        let step = if self.duration <= 0.0 { 1.0 } else { dt / self.duration };
        self.progress = if self.on { self.progress + step } else { self.progress - step }.clamp(0.0, 1.0);
        ease_out_cubic(self.progress)
    }
    fn is_animating(&self) -> bool {
        self.progress != if self.on { 1.0 } else { 0.0 }
    }
}

fn main() {
    let mut hover = Anim::new(0.12);
    hover.set(true);
    for frame in 0..10 {
        if frame == 5 { hover.set(false); println!("-- pointer left --"); }
        println!("frame {frame}: {:.3}", hover.update(1.0 / 60.0));
    }
}

@@exercise A spring for a bouncy press
Implement `Spring { pos, vel, target, stiffness, damping }` with `step(dt)` using the formulas in the lesson (update velocity first, then position, "semi-implicit Euler"), and `settled()`: true when both `|target − pos|` and `|vel|` are below 0.001. Then `simulate(spring, seconds, dt) -> (frames, max_pos)`: step until settled (or `seconds` run out), returning how many steps it took and the highest position reached.
@@starter
struct Spring { pos: f64, vel: f64, target: f64, stiffness: f64, damping: f64 }

impl Spring {
    fn step(&mut self, dt: f64) { todo!() }
    fn settled(&self) -> bool { todo!() }
}

fn simulate(mut s: Spring, seconds: f64, dt: f64) -> (usize, f64) {
    todo!()
}

fn main() {
    let bouncy = Spring { pos: 0.0, vel: 0.0, target: 1.0, stiffness: 300.0, damping: 12.0 };
    let (frames, max) = simulate(bouncy, 3.0, 1.0 / 120.0);
    println!("settled after {frames} steps, overshot to {max:.3}");
}
@@tests
#[test]
fn checks() {
    let mut s = Spring { pos: 0.0, vel: 0.0, target: 1.0, stiffness: 100.0, damping: 10.0 };
    s.step(0.01);
    assert!((s.vel - 1.0).abs() < 1e-9, "velocity after one step: force 100 * 0.01");
    assert!((s.pos - 0.01).abs() < 1e-9, "position uses the new velocity");
    let bouncy = Spring { pos: 0.0, vel: 0.0, target: 1.0, stiffness: 300.0, damping: 12.0 };
    let (frames, max) = simulate(bouncy, 3.0, 1.0 / 120.0);
    assert!(max > 1.1, "a low-damping spring overshoots");
    assert!(frames > 10 && frames < 360, "and settles within 3 seconds");
    let stiff = Spring { pos: 0.0, vel: 0.0, target: 1.0, stiffness: 300.0, damping: 40.0 };
    let (_, max2) = simulate(stiff, 3.0, 1.0 / 120.0);
    assert!(max2 < 1.01, "high damping barely overshoots");
    let at_rest = Spring { pos: 5.0, vel: 0.0, target: 5.0, stiffness: 300.0, damping: 20.0 };
    assert_eq!(simulate(at_rest, 1.0, 0.01), (0, 5.0), "already settled");
}
@@hint
`let force = self.stiffness * (self.target - self.pos) - self.damping * self.vel; self.vel += force * dt; self.pos += self.vel * dt;`. In `simulate`, start `max = s.pos`, loop `while !s.settled() && steps < (seconds / dt) as usize`.
@@solution
struct Spring { pos: f64, vel: f64, target: f64, stiffness: f64, damping: f64 }

impl Spring {
    fn step(&mut self, dt: f64) {
        let force = self.stiffness * (self.target - self.pos) - self.damping * self.vel;
        self.vel += force * dt;
        self.pos += self.vel * dt;
    }
    fn settled(&self) -> bool {
        (self.target - self.pos).abs() < 0.001 && self.vel.abs() < 0.001
    }
}

fn simulate(mut s: Spring, seconds: f64, dt: f64) -> (usize, f64) {
    let limit = (seconds / dt) as usize;
    let mut steps = 0;
    let mut max = s.pos;
    while !s.settled() && steps < limit {
        s.step(dt);
        max = max.max(s.pos);
        steps += 1;
    }
    (steps, max)
}

fn main() {
    let bouncy = Spring { pos: 0.0, vel: 0.0, target: 1.0, stiffness: 300.0, damping: 12.0 };
    let (frames, max) = simulate(bouncy, 3.0, 1.0 / 120.0);
    println!("settled after {frames} steps, overshot to {max:.3}");
}
