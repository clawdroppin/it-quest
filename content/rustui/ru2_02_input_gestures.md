@@lesson ru2-input Input and Gestures: Clicks, Drags, Keyboard Shortcuts and Momentum
@@concepts conditions, numbers, objects
@@lang rust
@@text
A UI feels "native" when its input handling matches what people's hands expect: a click that wiggles a few pixels is still a click, a double-click has a time window, drags start only after a threshold, shortcuts follow the platform, and flung lists coast to a stop. Toolkits give you raw events (pointer down/move/up, keys, wheel); this lesson shows how to turn them into gestures.

## Pointer events into gestures

- **Click**: press and release on the same widget, with movement below a **slop** threshold (about 4–8 logical px; larger for touch). egui's `Response::clicked()` does this for you.
- **Double-click**: a second click within about **500 ms** (the OS setting) and a few pixels of the first.
- **Drag**: begins when movement exceeds the slop while the button is held; from then on it's no longer a click. Capture the pointer so the drag continues outside the widget.
- **Long press** (touch): held about 500 ms without moving beyond the slop.
- Work in **logical pixels** (points), not physical pixels: multiply by the scale factor only when drawing.

## Keyboard shortcuts

- Use the platform's primary modifier: **Ctrl** on Windows/Linux, **Cmd** on macOS (egui's `Modifiers::command` handles both).
- Match shortcuts on the **logical key** (what the key produces) for letters, and on the physical key for layout-independent game-style controls.
- Don't steal common text-editing shortcuts inside text fields; let focused widgets handle keys first.
- Show shortcuts in menus and tooltips; respect IME composition (Japanese input) by not treating keys as shortcuts while composing.

## Scrolling and momentum

- Mouse wheels send discrete line steps; touchpads send smooth pixel deltas with their own momentum on macOS. Normalise both.
- **Kinetic scrolling**: on release, keep the current velocity and decay it each frame (`v *= friction^dt`) until it falls below a threshold. Clamp at the content edges, or overscroll with a spring back ("rubber banding").
- Estimate release velocity from the last ~50–100 ms of movement samples, not the last single event (noisy).

## Hover and focus

Hover effects need a pointer; on touch devices there's no hover, so never hide essential actions behind it. Keyboard focus must be visible and follow a logical order (see the layout lesson).

@@quiz
A user presses, moves 3 px, and releases. With a 6 px slop, what is it?
* A click
- A drag
- A long press
- Nothing
? Movement stayed under the threshold.

@@quiz
Which modifier should "Save" use on macOS?
- Ctrl
* Cmd
- Alt
- Shift
? egui's Modifiers::command maps to the platform key.

@@quiz
How should release velocity for kinetic scrolling be estimated?
- From the very last event only
* From movement samples over the last ~50–100 ms
- From the total drag distance
- It should always be zero
? Single events are noisy.

@@quiz
Why avoid hiding essential actions behind hover?
- Hover is slow
* Touch devices have no hover
- Rust can't detect hover
- It breaks dark mode
? Make actions reachable without a mouse.

@@quiz
While a Japanese IME is composing text, what should happen to key shortcuts?
- They should fire normally
* They should be ignored so the IME gets the keys
- They should be doubled
- They should open a menu
? Composition events take priority.

@@flashcards
slop :: movement allowed in a click
double-click window :: about 500 ms
drag start :: movement beyond slop
pointer capture :: keep the drag outside widget
long press :: about 500 ms held still
logical pixels :: points, scale-independent
command modifier :: Ctrl or Cmd
logical vs physical key :: produced vs position
IME composition :: don't treat keys as shortcuts
kinetic scroll :: decay velocity each frame
friction^dt :: frame-rate independent decay
rubber banding :: spring back at the edges
release velocity :: average of recent samples

@@drill
What is the movement threshold that separates a click from a drag usually called?
= slop
= touch slop
= drag threshold
= click slop
? Typically a few logical pixels.

@@drill
Which egui modifier field means Ctrl on Windows and Cmd on macOS?
= command
= Modifiers::command
= modifiers.command
? There's also mac_cmd for macOS only.

@@exercise A gesture recogniser
Turn raw pointer events into gestures. A press followed by a release within `SLOP` pixels is a `Click`; a second click within `DOUBLE_MS` and `SLOP` of the previous click becomes a `DoubleClick` (and the next click starts fresh); moving beyond `SLOP` while pressed starts a drag, reported once as `DragStart` and then as `DragMove(dx, dy)` relative to the last position, and `DragEnd` on release (no click).
@@starter
const SLOP: f32 = 6.0;
const DOUBLE_MS: u64 = 500;

#[derive(Debug, Clone, Copy)]
enum Event { Down { x: f32, y: f32, t: u64 }, Move { x: f32, y: f32 }, Up { x: f32, y: f32, t: u64 } }

#[derive(Debug, PartialEq)]
enum Gesture { Click, DoubleClick, DragStart, DragMove(f32, f32), DragEnd }

#[derive(Default)]
struct Recogniser { pressed: Option<(f32, f32)>, last: Option<(f32, f32)>, dragging: bool, last_click: Option<(f32, f32, u64)> }

impl Recogniser {
    fn handle(&mut self, e: Event) -> Vec<Gesture> {
        todo!()
    }
}

fn main() {
    let mut r = Recogniser::default();
    for e in [Event::Down { x: 10.0, y: 10.0, t: 0 }, Event::Up { x: 11.0, y: 10.0, t: 90 }] {
        println!("{:?}", r.handle(e));
    }
}
@@tests
fn run(events: &[Event]) -> Vec<Gesture> {
    let mut r = Recogniser::default();
    events.iter().flat_map(|e| r.handle(*e)).collect()
}

#[test]
fn click_with_wiggle() {
    let g = run(&[Event::Down { x: 10.0, y: 10.0, t: 0 }, Event::Move { x: 13.0, y: 12.0 }, Event::Up { x: 13.0, y: 12.0, t: 100 }]);
    assert_eq!(g, vec![Gesture::Click]);
}

#[test]
fn double_click_and_reset() {
    let g = run(&[
        Event::Down { x: 10.0, y: 10.0, t: 0 }, Event::Up { x: 10.0, y: 10.0, t: 80 },
        Event::Down { x: 12.0, y: 11.0, t: 300 }, Event::Up { x: 12.0, y: 11.0, t: 380 },
        Event::Down { x: 12.0, y: 11.0, t: 500 }, Event::Up { x: 12.0, y: 11.0, t: 560 },
    ]);
    assert_eq!(g, vec![Gesture::Click, Gesture::DoubleClick, Gesture::Click]);
}

#[test]
fn slow_second_click_is_single() {
    let g = run(&[
        Event::Down { x: 10.0, y: 10.0, t: 0 }, Event::Up { x: 10.0, y: 10.0, t: 80 },
        Event::Down { x: 10.0, y: 10.0, t: 900 }, Event::Up { x: 10.0, y: 10.0, t: 950 },
    ]);
    assert_eq!(g, vec![Gesture::Click, Gesture::Click]);
}

#[test]
fn drag() {
    let g = run(&[
        Event::Down { x: 0.0, y: 0.0, t: 0 }, Event::Move { x: 3.0, y: 0.0 }, Event::Move { x: 10.0, y: 0.0 },
        Event::Move { x: 15.0, y: 5.0 }, Event::Up { x: 15.0, y: 5.0, t: 200 },
    ]);
    assert_eq!(g, vec![Gesture::DragStart, Gesture::DragMove(10.0, 0.0), Gesture::DragMove(5.0, 5.0), Gesture::DragEnd]);
}

#[test]
fn moves_without_press_do_nothing() {
    assert!(run(&[Event::Move { x: 50.0, y: 50.0 }]).is_empty());
}
@@hint
Store the press position and the last position. On `Move` while pressed: if not dragging and the distance from the press position exceeds `SLOP`, set `dragging` and emit `DragStart` followed by `DragMove` from the press position; if already dragging emit `DragMove` from `last`. On `Up`: if dragging emit `DragEnd`; otherwise compare with `last_click` (time and distance) to choose `DoubleClick` (then clear `last_click`) or `Click` (then store it).
@@solution
const SLOP: f32 = 6.0;
const DOUBLE_MS: u64 = 500;

#[derive(Debug, Clone, Copy)]
enum Event { Down { x: f32, y: f32, t: u64 }, Move { x: f32, y: f32 }, Up { x: f32, y: f32, t: u64 } }

#[derive(Debug, PartialEq)]
enum Gesture { Click, DoubleClick, DragStart, DragMove(f32, f32), DragEnd }

#[derive(Default)]
struct Recogniser { pressed: Option<(f32, f32)>, last: Option<(f32, f32)>, dragging: bool, last_click: Option<(f32, f32, u64)> }

fn dist(a: (f32, f32), b: (f32, f32)) -> f32 {
    ((a.0 - b.0).powi(2) + (a.1 - b.1).powi(2)).sqrt()
}

impl Recogniser {
    fn handle(&mut self, e: Event) -> Vec<Gesture> {
        let mut out = vec![];
        match e {
            Event::Down { x, y, .. } => {
                self.pressed = Some((x, y));
                self.last = Some((x, y));
                self.dragging = false;
            }
            Event::Move { x, y } => {
                if let (Some(start), Some(last)) = (self.pressed, self.last) {
                    if !self.dragging && dist(start, (x, y)) > SLOP {
                        self.dragging = true;
                        out.push(Gesture::DragStart);
                        out.push(Gesture::DragMove(x - start.0, y - start.1));
                    } else if self.dragging {
                        out.push(Gesture::DragMove(x - last.0, y - last.1));
                    }
                    self.last = Some((x, y));
                }
            }
            Event::Up { x, y, t } => {
                if self.pressed.take().is_none() {
                    return out;
                }
                if self.dragging {
                    self.dragging = false;
                    out.push(Gesture::DragEnd);
                } else {
                    match self.last_click {
                        Some((px, py, pt)) if t - pt <= DOUBLE_MS && dist((px, py), (x, y)) <= SLOP => {
                            out.push(Gesture::DoubleClick);
                            self.last_click = None;
                        }
                        _ => {
                            out.push(Gesture::Click);
                            self.last_click = Some((x, y, t));
                        }
                    }
                }
                self.last = None;
            }
        }
        out
    }
}

fn main() {
    let mut r = Recogniser::default();
    for e in [Event::Down { x: 10.0, y: 10.0, t: 0 }, Event::Up { x: 11.0, y: 10.0, t: 90 }] {
        println!("{:?}", r.handle(e));
    }
}

@@exercise Kinetic scrolling
Implement a scroll view with momentum. While dragging, `drag(dy, t_ms)` moves the content directly and records samples; `release(t_ms)` sets the velocity (px per second) from samples in the last `VELOCITY_WINDOW_MS`; `step(dt)` coasts with `v *= FRICTION.powf(dt)` and stops below `MIN_SPEED`, clamping the offset to `0..=max` and stopping at an edge.
@@starter
const FRICTION: f32 = 0.05; // fraction of velocity left after one second
const MIN_SPEED: f32 = 5.0;
const VELOCITY_WINDOW_MS: u64 = 100;

struct Scroll { offset: f32, max: f32, velocity: f32, samples: Vec<(u64, f32)> }

impl Scroll {
    fn new(content: f32, viewport: f32) -> Self {
        todo!()
    }
    fn drag(&mut self, dy: f32, t_ms: u64) {
        todo!()
    }
    fn release(&mut self, t_ms: u64) {
        todo!()
    }
    fn step(&mut self, dt: f32) {
        todo!()
    }
}

fn main() {
    let mut s = Scroll::new(5000.0, 800.0);
    for i in 1..=10 { s.drag(30.0, i * 16); }
    s.release(160);
    let mut frames = 0;
    while s.velocity != 0.0 { s.step(1.0 / 60.0); frames += 1; }
    println!("coasted {frames} frames to offset {:.0}", s.offset);
}
@@tests
#[test]
fn max_offset() {
    let s = Scroll::new(5000.0, 800.0);
    assert_eq!(s.max, 4200.0);
    assert_eq!(Scroll::new(300.0, 800.0).max, 0.0, "short content can't scroll");
}

#[test]
fn drag_moves_directly_and_clamps() {
    let mut s = Scroll::new(1000.0, 800.0);
    s.drag(150.0, 16);
    assert_eq!(s.offset, 150.0);
    s.drag(150.0, 32);
    assert_eq!(s.offset, 200.0);
    s.drag(-500.0, 48);
    assert_eq!(s.offset, 0.0);
}

#[test]
fn release_velocity_uses_recent_samples() {
    let mut s = Scroll::new(5000.0, 800.0);
    s.drag(500.0, 10);
    for i in 1..=5 { s.drag(20.0, 500 + i * 20); }
    s.release(600);
    assert!((s.velocity - 1000.0).abs() < 1.0, "100 px in the last 100 ms is 1000 px/s, got {}", s.velocity);
}

#[test]
fn coasting_decays_and_stops() {
    let mut s = Scroll::new(100000.0, 800.0);
    for i in 1..=5 { s.drag(20.0, i * 20); }
    s.release(100);
    let start = s.offset;
    s.step(0.5);
    assert!(s.offset > start);
    let mut frames = 0;
    while s.velocity != 0.0 && frames < 10000 { s.step(1.0 / 60.0); frames += 1; }
    assert!(frames < 10000, "must stop");
    assert!(s.velocity == 0.0);
}

#[test]
fn stops_at_edge() {
    let mut s = Scroll::new(1200.0, 800.0);
    for i in 1..=5 { s.drag(60.0, i * 20); }
    s.release(100);
    for _ in 0..600 { s.step(1.0 / 60.0); }
    assert_eq!(s.offset, 400.0);
    assert_eq!(s.velocity, 0.0);
}
@@hint
`max = (content - viewport).max(0.0)`. In `drag`, clamp with `.clamp(0.0, self.max)` and push `(t_ms, dy)`. In `release`, keep samples with `t >= t_ms - VELOCITY_WINDOW_MS` (use `saturating_sub`), sum their dy and divide by the window in seconds. In `step`, move by `velocity * dt`, decay, and set velocity to 0 when it's below `MIN_SPEED` or the offset hit 0 or max.
@@solution
const FRICTION: f32 = 0.05; // fraction of velocity left after one second
const MIN_SPEED: f32 = 5.0;
const VELOCITY_WINDOW_MS: u64 = 100;

struct Scroll { offset: f32, max: f32, velocity: f32, samples: Vec<(u64, f32)> }

impl Scroll {
    fn new(content: f32, viewport: f32) -> Self {
        Scroll { offset: 0.0, max: (content - viewport).max(0.0), velocity: 0.0, samples: vec![] }
    }
    fn drag(&mut self, dy: f32, t_ms: u64) {
        self.velocity = 0.0;
        self.offset = (self.offset + dy).clamp(0.0, self.max);
        self.samples.push((t_ms, dy));
    }
    fn release(&mut self, t_ms: u64) {
        let from = t_ms.saturating_sub(VELOCITY_WINDOW_MS);
        let moved: f32 = self.samples.iter().filter(|(t, _)| *t >= from).map(|(_, d)| d).sum();
        self.velocity = moved / (VELOCITY_WINDOW_MS as f32 / 1000.0);
        self.samples.clear();
    }
    fn step(&mut self, dt: f32) {
        if self.velocity == 0.0 {
            return;
        }
        self.offset = (self.offset + self.velocity * dt).clamp(0.0, self.max);
        self.velocity *= FRICTION.powf(dt);
        if self.velocity.abs() < MIN_SPEED || self.offset == 0.0 || self.offset == self.max {
            self.velocity = 0.0;
        }
    }
}

fn main() {
    let mut s = Scroll::new(5000.0, 800.0);
    for i in 1..=10 { s.drag(30.0, i * 16); }
    s.release(160);
    let mut frames = 0;
    while s.velocity != 0.0 { s.step(1.0 / 60.0); frames += 1; }
    println!("coasted {frames} frames to offset {:.0}", s.offset);
}
