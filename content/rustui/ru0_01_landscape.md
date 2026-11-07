@@lesson ru0-landscape Rust GUIs: Toolkits, Architecture and Widget State
@@concepts objects, conditions, functions
@@lang rust
@@text
Rust can build desktop apps that start instantly, use little memory and look exactly how you want. This course is about making them **beautiful and pleasant to use**: themes, typography, icons and interactive, animated buttons. The Rust track's guided builds (Focus Timer, Vault, Organizer, Chat) teach how to structure an app; this course teaches how to make one *feel* great.

The exercises use only Rust's standard library, so they run inside IT Quest. They implement the logic every UI toolkit runs underneath (state machines, colours, easing, layout), which is exactly the code you write when you build custom widgets. The toolkit snippets are for your own projects (`cargo new`, add the crate, paste).

## The toolkits

| Toolkit | Model | Strengths | Good for |
|---|---|---|---|
| **egui / eframe** | immediate mode | tiny API, very fast to build with, easy custom painting, web (WASM) too | tools, dashboards, editors, game UIs, this app's Rust track |
| **iced** | retained, Elm architecture (state → view → message → update) | clean architecture, good theming, animations, used by COSMIC desktop | polished apps with clear state |
| **Slint** | declarative `.slint` markup + Rust logic | designer-friendly language, live preview, embedded devices, small runtime | product UIs, kiosks, embedded screens |
| **Tauri** | web front end (HTML/CSS/JS or Leptos/Dioxus) + Rust back end | any web design skill applies, tiny installers (this app!) | rich-looking apps, teams that know CSS |
| **Dioxus** | React-like components in Rust (`rsx!`) | one codebase for desktop, web and mobile | React developers |
| **gpui** | GPU-first, from the Zed editor | extreme performance | editors, heavy apps |
| **Bevy UI** | ECS game engine UI | games | in-game menus |

**Immediate mode** (egui): every frame (up to 60+ times a second when something changes) your code describes the whole UI again, and the call that draws a button also tells you whether it was clicked:

```rust,norun
// Cargo.toml: eframe = "0.37"
use eframe::egui;

fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([420.0, 260.0]).with_title("Hello"),
        ..Default::default()
    };
    eframe::run_native("hello", options, Box::new(|_cc| Ok(Box::new(App::default()))))
}

#[derive(Default)]
struct App {
    count: u32,
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Hello, egui");
            if ui.button("Click me").clicked() {
                self.count += 1;
            }
            ui.label(format!("Clicked {} times", self.count));
        });
    }
}
```

**Retained / Elm architecture** (iced): state is a struct, the view is a pure function of the state, and user actions become **messages** handled in one `update` function:

```rust,norun
// Cargo.toml: iced = "0.13"
use iced::widget::{button, column, text};

#[derive(Default)]
struct Counter { count: u32 }

#[derive(Debug, Clone)]
enum Message { Increment }

impl Counter {
    fn update(&mut self, message: Message) {
        match message {
            Message::Increment => self.count += 1,
        }
    }
    fn view(&self) -> iced::Element<'_, Message> {
        column![button("Click me").on_press(Message::Increment), text(self.count)].spacing(12).into()
    }
}

fn main() -> iced::Result {
    iced::run("Counter", Counter::update, Counter::view)
}
```

**Declarative markup** (Slint): the look lives in a `.slint` file a designer can edit, with properties, states and animations built in:

```text
export component AppWindow inherits Window {
    in-out property <int> count: 0;
    VerticalLayout {
        padding: 16px; spacing: 12px;
        Text { text: "Clicked \{count} times"; }
        Button { text: "Click me"; clicked => { count += 1; } }
    }
}
```

**Which one?** egui for speed of building and custom drawing; iced when the app's state is complex and you like strict structure; Slint when look-and-feel and designers matter; Tauri when you want the full power of CSS (and everything from the GUI Design course applies directly).

## Widgets are state machines

Every interactive control moves between **states**, and good design gives each one a distinct look:

| State | When | Typical look |
|---|---|---|
| **Idle** | nothing happening | base colour |
| **Hovered** | pointer over it | slightly lighter, cursor becomes a hand |
| **Pressed** | button held down | darker, scaled to ~97%, shadow shrinks |
| **Focused** | selected with the keyboard (Tab) | a clear focus ring, never removed |
| **Disabled** | not available | faded, no hover effect, not clickable |
| **Loading** | working after a click | spinner, not clickable again |

A **click** is not "the mouse went down": it's *pressed inside, then released inside*. Drag out and release, and nothing happens: that's how users cancel a click. Toolkits also need **hit testing**: is the pointer inside this widget's rectangle (or rounded shape)? The two exercises build both pieces.

## Project setup tips

- `cargo new my_app`, `cargo add eframe` (or iced, slint), `cargo run --release` for smooth animation (debug builds are much slower).
- Hide the console window on Windows: `#![windows_subsystem = "windows"]` at the top of `main.rs`.
- Persist window size and settings: eframe's `persistence` feature and `cc.storage`.
- Ship it: `cargo build --release`, an icon in the executable (lesson 3), and an installer with `cargo-packager`, `cargo-wix`, or Tauri's bundler.

@@quiz
In egui (immediate mode), when is a button drawn?
* Every frame your `update` function runs; the same call that draws it returns whether it was clicked
- Once, when the window opens
- Only when the user hovers it
- When you call `button.render()` manually
? egui repaints when something changes (input, animation), so it stays cheap.

@@quiz
In iced's Elm architecture, how does a click change the state?
- The button changes the state directly
* The button produces a Message; `update` receives it and changes the state; `view` then shows the new state
- Through a global variable
- By redrawing the window manually
? State → view → message → update, in one direction.

@@quiz
What counts as a click?
- The mouse button going down
* Pressing inside the widget and releasing inside it
- Releasing anywhere in the window
- Hovering for one second
? Dragging out before releasing cancels: users rely on it.

@@quiz
Which toolkit lets a designer edit the look in a separate markup file with live preview?
- egui
* Slint
- Bevy UI
- std::io
? `.slint` files describe components, properties, states and animations.

@@quiz
Why run animated UIs with `cargo run --release`?
- Debug builds can't open windows
* Debug builds are unoptimized and much slower, so animations can stutter
- Release builds have nicer colours
- Animations are disabled in debug builds
? Profile and judge smoothness in release mode.

@@drill
Which Rust GUI library is immediate mode?
= egui
? iced and Slint are retained mode.

@@flashcards
egui / eframe :: immediate-mode GUI / its app framework
immediate mode :: describe the whole UI every frame; calls return interactions
retained mode :: a tree of widget objects that persists between frames
iced :: Elm architecture: state, view, message, update
Slint :: declarative .slint markup with Rust logic
Tauri :: web front end with a Rust back end
Dioxus :: React-like components in Rust
widget states :: idle, hovered, pressed, focused, disabled, loading
click :: press inside, release inside
hit testing :: is the pointer inside this shape?
#![windows_subsystem = "windows"] :: no console window on Windows
cargo run --release :: optimized build for smooth animation

@@exercise A button's state machine
Implement `next(state, event, inside)` for a button. States: `Idle`, `Hovered`, `Pressed`, `Disabled`. Events: `Move` (pointer moved; `inside` says where it is now), `Down`, `Up`, `Enable`, `Disable`. Rules: `Disable` always goes to `Disabled`; a disabled button only reacts to `Enable` (→ `Hovered` if inside, else `Idle`). `Move` gives `Hovered`/`Idle` (but a `Pressed` button stays `Pressed` while the mouse is held). `Down` inside → `Pressed`. `Up` → `Hovered` if inside else `Idle`. Return the new state and whether a **click** happened (only on `Up` from `Pressed` while inside).
@@starter
#[derive(Debug, Clone, Copy, PartialEq)]
enum State { Idle, Hovered, Pressed, Disabled }

#[derive(Debug, Clone, Copy, PartialEq)]
enum Event { Move, Down, Up, Enable, Disable }

fn next(state: State, event: Event, inside: bool) -> (State, bool) {
    todo!()
}

fn main() {
    let mut s = State::Idle;
    for (e, inside) in [(Event::Move, true), (Event::Down, true), (Event::Up, true)] {
        let (n, clicked) = next(s, e, inside);
        println!("{s:?} --{e:?}--> {n:?}{}", if clicked { "  CLICK!" } else { "" });
        s = n;
    }
}
@@tests
#[test]
fn checks() {
    use Event::*;
    use State::*;
    assert_eq!(next(Idle, Move, true), (Hovered, false), "hover");
    assert_eq!(next(Hovered, Move, false), (Idle, false), "leave");
    assert_eq!(next(Hovered, Down, true), (Pressed, false), "press");
    assert_eq!(next(Pressed, Up, true), (Hovered, true), "click");
    assert_eq!(next(Pressed, Move, false), (Pressed, false), "stays pressed while dragging out");
    assert_eq!(next(Pressed, Up, false), (Idle, false), "released outside: no click");
    assert_eq!(next(Hovered, Up, true), (Hovered, false), "up without a press is not a click");
    assert_eq!(next(Hovered, Disable, true), (Disabled, false), "disable");
    assert_eq!(next(Disabled, Down, true), (Disabled, false), "disabled ignores presses");
    assert_eq!(next(Disabled, Move, true), (Disabled, false), "disabled ignores hover");
    assert_eq!(next(Disabled, Enable, true), (Hovered, false), "enable under the pointer");
    assert_eq!(next(Disabled, Enable, false), (Idle, false), "enable elsewhere");
}
@@hint
Handle `Disable` first, then `if state == Disabled` (only `Enable` matters). Then `match event`. A helper `let rest = if inside { Hovered } else { Idle };` saves typing. The click is `state == Pressed && inside` for `Up`.
@@solution
#[derive(Debug, Clone, Copy, PartialEq)]
enum State { Idle, Hovered, Pressed, Disabled }

#[derive(Debug, Clone, Copy, PartialEq)]
enum Event { Move, Down, Up, Enable, Disable }

fn next(state: State, event: Event, inside: bool) -> (State, bool) {
    let rest = if inside { State::Hovered } else { State::Idle };
    if event == Event::Disable {
        return (State::Disabled, false);
    }
    if state == State::Disabled {
        return if event == Event::Enable { (rest, false) } else { (State::Disabled, false) };
    }
    match event {
        Event::Move if state == State::Pressed => (State::Pressed, false),
        Event::Move => (rest, false),
        Event::Down if inside => (State::Pressed, false),
        Event::Down => (state, false),
        Event::Up => (rest, state == State::Pressed && inside),
        Event::Enable | Event::Disable => (state, false),
    }
}

fn main() {
    let mut s = State::Idle;
    for (e, inside) in [(Event::Move, true), (Event::Down, true), (Event::Up, true)] {
        let (n, clicked) = next(s, e, inside);
        println!("{s:?} --{e:?}--> {n:?}{}", if clicked { "  CLICK!" } else { "" });
        s = n;
    }
}

@@exercise Hit testing rounded buttons
A `Rect` has `x`, `y` (top-left), `w`, `h`. Write `contains(&self, px, py)` (edges count as inside), and `contains_rounded(&self, px, py, radius)` for a rectangle with rounded corners: a point in a corner square is inside only if it's within `radius` of that corner's circle centre. Then `top_hit(buttons, px, py)`: buttons are drawn in order, so the **last** one containing the point is on top; return its index, or `None`.
@@starter
#[derive(Debug, Clone, Copy)]
struct Rect { x: f32, y: f32, w: f32, h: f32 }

impl Rect {
    fn contains(&self, px: f32, py: f32) -> bool {
        todo!()
    }

    fn contains_rounded(&self, px: f32, py: f32, radius: f32) -> bool {
        todo!()
    }
}

fn top_hit(buttons: &[Rect], px: f32, py: f32) -> Option<usize> {
    todo!()
}

fn main() {
    let b = Rect { x: 10.0, y: 10.0, w: 100.0, h: 40.0 };
    println!("{} {}", b.contains(11.0, 11.0), b.contains_rounded(11.0, 11.0, 12.0));
}
@@tests
#[test]
fn checks() {
    let b = Rect { x: 10.0, y: 10.0, w: 100.0, h: 40.0 };
    assert!(b.contains(10.0, 10.0) && b.contains(110.0, 50.0), "edges are inside");
    assert!(!b.contains(9.9, 30.0) && !b.contains(50.0, 50.1), "outside");
    assert!(b.contains_rounded(60.0, 30.0, 12.0), "the middle");
    assert!(b.contains_rounded(60.0, 10.0, 12.0), "top edge between the corners");
    assert!(!b.contains_rounded(11.0, 11.0, 12.0), "the cut-off corner");
    assert!(b.contains_rounded(16.0, 16.0, 12.0), "inside the corner arc");
    assert!(!b.contains_rounded(109.0, 49.0, 12.0), "bottom-right corner");
    assert!(!b.contains_rounded(200.0, 30.0, 12.0), "far outside");
    let stack = [Rect { x: 0.0, y: 0.0, w: 50.0, h: 50.0 }, Rect { x: 25.0, y: 25.0, w: 50.0, h: 50.0 }];
    assert_eq!(top_hit(&stack, 30.0, 30.0), Some(1), "the later one is on top");
    assert_eq!(top_hit(&stack, 5.0, 5.0), Some(0), "only the first");
    assert_eq!(top_hit(&stack, 90.0, 90.0), None, "nothing");
}
@@hint
For the rounded test, first require `contains`. Then clamp the point into the "inner" rectangle shrunk by the radius: `cx = px.clamp(x + r, x + w - r)`, `cy = py.clamp(y + r, y + h - r)`. The point is inside when its distance to `(cx, cy)` is at most `r`. For `top_hit`, iterate with `.iter().enumerate().rev()`.
@@solution
#[derive(Debug, Clone, Copy)]
struct Rect { x: f32, y: f32, w: f32, h: f32 }

impl Rect {
    fn contains(&self, px: f32, py: f32) -> bool {
        px >= self.x && px <= self.x + self.w && py >= self.y && py <= self.y + self.h
    }

    fn contains_rounded(&self, px: f32, py: f32, radius: f32) -> bool {
        if !self.contains(px, py) {
            return false;
        }
        let cx = px.clamp(self.x + radius, self.x + self.w - radius);
        let cy = py.clamp(self.y + radius, self.y + self.h - radius);
        let (dx, dy) = (px - cx, py - cy);
        dx * dx + dy * dy <= radius * radius
    }
}

fn top_hit(buttons: &[Rect], px: f32, py: f32) -> Option<usize> {
    buttons.iter().enumerate().rev().find(|(_, b)| b.contains(px, py)).map(|(i, _)| i)
}

fn main() {
    let b = Rect { x: 10.0, y: 10.0, w: 100.0, h: 40.0 };
    println!("{} {}", b.contains(11.0, 11.0), b.contains_rounded(11.0, 11.0, 12.0));
}
