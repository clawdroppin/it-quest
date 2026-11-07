@@lesson ru3-icons Icons for Rust Apps: SVG, Icon Fonts and App Icons
@@concepts strings, loops, numbers
@@lang rust
@@text
Icons make an interface faster to read, as long as they're **consistent**: one style, one stroke width, one grid. This lesson shows the three ways to get icons into a Rust GUI, how to draw your own straight from code, and how to give your executable a proper app icon.

## Three ways to show icons

| Approach | How | Pros / cons |
|---|---|---|
| **Icon font** | a font whose glyphs are icons: `egui-phosphor`, `egui_material_icons`, Lucide/Phosphor fonts in iced and Slint | one line per icon, scales and recolours like text; only one colour |
| **SVG files** | render with `resvg` (egui: `egui_extras` with the `svg` feature, `egui::Image::new(egui::include_image!("icons/save.svg"))`; iced: `iced::widget::svg`; Slint: `@image-url("save.svg")`) | any design from Inkscape or Penpot; multicolour |
| **Painted in code** | draw circles, lines and paths with the painter (egui `Painter`, iced `Canvas`, Slint `Path`) | animatable, no files at all; needs maths |

The icon font is the quickest start:

```rust,norun
// Cargo.toml: egui-phosphor = "*"   (match it to your egui version)
let mut fonts = egui::FontDefinitions::default();
egui_phosphor::add_to_fonts(&mut fonts, egui_phosphor::Variant::Regular);
ctx.set_fonts(fonts);
// later, inside a UI:
if ui.button(format!("{} Save", egui_phosphor::regular::FLOPPY_DISK)).clicked() { /* ... */ }
```

Good free icon sets with matching fonts and SVGs: **Lucide**, **Phosphor**, **Tabler**, **Material Symbols**, **Remix Icon**. Check each licence (most are MIT, ISC or Apache).

## Designing a consistent icon set

- A **24 × 24 grid** with 2 px padding (live area 20 × 20); draw at 24 and let it scale.
- One **stroke width** (1.5 or 2 px), round caps and joins, one corner radius.
- Similar **visual weight**: a circle looks smaller than a square of the same width, so make it slightly bigger ("optical sizing").
- **Pixel-snap** horizontal and vertical lines (on whole or half pixels) so they stay crisp at 16 and 24 px.
- Use `stroke="currentColor"` in SVGs, so the icon takes the text colour of the theme (light and dark for free).

The Vector UI Design course in Extras teaches drawing them in Inkscape and Penpot; here we generate them from Rust.

## SVG in two minutes

```text
<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none"
     stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
  <circle cx="12" cy="12" r="9"/>
  <path d="M8 12l3 3 5-6"/>
</svg>
```

A path's `d` attribute is a list of commands: `M x y` move to, `L x y` line to (lower-case `l` is relative), `H`/`V` horizontal/vertical lines, `A rx ry rotation large-arc sweep x y` arcs, `C` cubic Béziers, `Z` close. Generating these strings from Rust lets you build icons from parameters (a progress ring, a battery level, a signal-strength icon), and the exercises do exactly that.

## Painting icons in egui

```rust,norun
fn check_icon(painter: &egui::Painter, center: egui::Pos2, size: f32, color: egui::Color32, t: f32) {
    let s = size / 24.0;                       // scale from the 24-unit grid
    let stroke = egui::Stroke::new(2.0 * s, color);
    painter.circle_stroke(center, 9.0 * s, stroke);
    // The tick draws itself as t goes 0 -> 1: an animated "done" icon.
    let p = |x: f32, y: f32| center + egui::vec2((x - 12.0) * s, (y - 12.0) * s);
    let (a, b, c) = (p(8.0, 12.0), p(11.0, 15.0), p(16.0, 9.0));
    let first = (t * 2.0).min(1.0);
    painter.line_segment([a, a + (b - a) * first], stroke);
    if t > 0.5 {
        painter.line_segment([b, b + (c - b) * ((t - 0.5) * 2.0)], stroke);
    }
}
```

## The app icon

- **Windows**: the `.exe` icon is a resource. Add `winresource` as a build dependency and a `build.rs`:

```rust,norun
// build.rs
fn main() {
    if std::env::var("CARGO_CFG_TARGET_OS").unwrap() == "windows" {
        winresource::WindowsResource::new().set_icon("assets/app.ico").compile().unwrap();
    }
}
```

- The `.ico` should contain **16, 24, 32, 48, 64 and 256 px** versions (simplify the small ones by hand: fewer details, thicker lines). ImageMagick: `magick app-256.png -define icon:auto-resize=256,64,48,32,24,16 app.ico`.
- **Window icon** (title bar and taskbar while running): eframe `ViewportBuilder::with_icon(...)` from PNG bytes; iced `window::Settings { icon: Some(window::icon::from_file_data(...)) }`.
- **Linux**: a 256 or 512 px PNG (or SVG) plus a `.desktop` file; **macOS**: an `.icns` with sizes up to 1024.
- **Tauri**: `cargo tauri icon app-1024.png` generates every size and format at once (this app does exactly that).

A good app icon reads at 16 px: one bold shape, two or three colours, no text.

@@quiz
Why use `stroke="currentColor"` in icon SVGs?
* The icon takes the current text colour, so it follows light and dark themes automatically
- It makes the file smaller
- It's required by resvg
- It animates the stroke
? Set the colour once on the parent, like text.

@@quiz
Which sizes should a Windows .ico contain?
- Only 256 px
* 16, 24, 32, 48, 64 and 256 px, with the small ones simplified
- 1024 px only
- Any single size; Windows scales it perfectly
? Small sizes need hand-tuning to stay readable.

@@quiz
What does `M8 12l3 3 5-6` draw in a 24-unit icon?
- A circle
* A check mark: move to (8,12), a relative line down-right, then up-right
- A square
- Nothing, it's invalid
? Lower-case l is a relative line.

@@quiz
What's the main limitation of icon fonts?
- They don't scale
* Each icon is a single colour, like text
- They can't be used in egui
- They're always huge files
? Use SVG for multicolour icons.

@@drill
Which crate renders SVG files to pixels in Rust?
= resvg
? Pair it with tiny-skia.

@@flashcards
icon font :: icons as glyphs: one colour, scales like text
egui-phosphor :: Phosphor icon font for egui
resvg :: Rust SVG renderer used by egui_extras, iced, Slint
24 × 24 grid :: common icon canvas, 2 px padding
consistent stroke :: one width, round caps and joins
optical sizing :: circles drawn slightly bigger than squares
currentColor :: SVG colour that follows the text colour
M L H V A C Z :: SVG path commands
winresource + build.rs :: embed an .ico into a Windows executable
.ico sizes :: 16, 24, 32, 48, 64, 256
with_icon :: eframe window icon
cargo tauri icon :: generate all app icon sizes

@@exercise Generate an icon SVG
Write `svg(size, body)` that wraps elements in the standard 24 × 24 icon root shown in the lesson, with `width` and `height` set to `size`: `<svg xmlns="http://www.w3.org/2000/svg" width="SIZE" height="SIZE" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">BODY</svg>`. Then `circle(cx, cy, r)` → `<circle cx=".." cy=".." r=".."/>` and `polyline(points)` → `<path d="M8 12L11 15L16 9"/>` (absolute `M` then `L` for each next point). Numbers print without trailing `.0` (use `{}` on `f32`: Rust prints `12` for `12.0`).
@@starter
fn svg(size: u32, body: &str) -> String { todo!() }
fn circle(cx: f32, cy: f32, r: f32) -> String { todo!() }
fn polyline(points: &[(f32, f32)]) -> String { todo!() }

fn main() {
    let body = circle(12.0, 12.0, 9.0) + &polyline(&[(8.0, 12.0), (11.0, 15.0), (16.0, 9.0)]);
    println!("{}", svg(24, &body));
}
@@tests
#[test]
fn checks() {
    assert_eq!(circle(12.0, 12.0, 9.0), r#"<circle cx="12" cy="12" r="9"/>"#, "circle");
    assert_eq!(circle(4.5, 7.0, 1.5), r#"<circle cx="4.5" cy="7" r="1.5"/>"#, "decimals");
    assert_eq!(polyline(&[(8.0, 12.0), (11.0, 15.0), (16.0, 9.0)]), r#"<path d="M8 12L11 15L16 9"/>"#, "check mark");
    assert_eq!(polyline(&[]), r#"<path d=""/>"#, "no points");
    let s = svg(32, "<x/>");
    assert!(s.starts_with(r#"<svg xmlns="http://www.w3.org/2000/svg" width="32" height="32" viewBox="0 0 24 24""#), "root attributes");
    assert!(s.contains(r#"stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">"#), "stroke attributes");
    assert!(s.ends_with("<x/></svg>"), "body inside");
}
@@hint
Use raw strings `r#"..."#` or escape quotes with `\"`. For the path: iterate with `.enumerate()` and push `M` for index 0, `L` otherwise, then `format!("{x} {y}")`.
@@solution
fn svg(size: u32, body: &str) -> String {
    format!(
        r#"<svg xmlns="http://www.w3.org/2000/svg" width="{size}" height="{size}" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">{body}</svg>"#
    )
}

fn circle(cx: f32, cy: f32, r: f32) -> String {
    format!(r#"<circle cx="{cx}" cy="{cy}" r="{r}"/>"#)
}

fn polyline(points: &[(f32, f32)]) -> String {
    let mut d = String::new();
    for (i, (x, y)) in points.iter().enumerate() {
        d.push_str(if i == 0 { "M" } else { "L" });
        d.push_str(&format!("{x} {y}"));
    }
    format!(r#"<path d="{d}"/>"#)
}

fn main() {
    let body = circle(12.0, 12.0, 9.0) + &polyline(&[(8.0, 12.0), (11.0, 15.0), (16.0, 9.0)]);
    println!("{}", svg(24, &body));
}

@@exercise A progress ring and a battery icon
Icons generated from data. `ring_point(cx, cy, r, fraction)`: the point on a circle at `fraction` of a full turn **clockwise from 12 o'clock** (angle = fraction × 2π; x = cx + r·sin, y = cy − r·cos), each coordinate rounded to 2 decimals. `progress_arc(fraction)`: the SVG path for a ring of radius 9 around (12, 12) from the top: `M12 3A9 9 0 F 1 X Y` where F (large-arc flag) is 1 if fraction > 0.5, else 0, and X Y is `ring_point` of the fraction. `battery_bars(level)`: how many of 4 bars to fill for a level 0–100 (`ceil(level / 25)`, but 0 only at exactly 0).
@@starter
fn ring_point(cx: f64, cy: f64, r: f64, fraction: f64) -> (f64, f64) { todo!() }
fn progress_arc(fraction: f64) -> String { todo!() }
fn battery_bars(level: u32) -> u32 { todo!() }

fn main() {
    for f in [0.25, 0.5, 0.75] {
        println!("{f}: {}", progress_arc(f));
    }
    println!("{:?}", [0, 1, 25, 26, 99, 100].map(battery_bars));
}
@@tests
#[test]
fn checks() {
    assert_eq!(ring_point(12.0, 12.0, 9.0, 0.0), (12.0, 3.0), "top");
    assert_eq!(ring_point(12.0, 12.0, 9.0, 0.25), (21.0, 12.0), "a quarter: right");
    assert_eq!(ring_point(12.0, 12.0, 9.0, 0.5), (12.0, 21.0), "half: bottom");
    assert_eq!(ring_point(12.0, 12.0, 9.0, 0.125), (18.36, 5.64), "an eighth");
    assert_eq!(progress_arc(0.25), "M12 3A9 9 0 0 1 21 12", "small arc");
    assert_eq!(progress_arc(0.75), "M12 3A9 9 0 1 1 3 12", "large arc");
    assert_eq!([0, 1, 25, 26, 50, 99, 100].map(battery_bars), [0, 1, 1, 2, 2, 4, 4], "bars");
}
@@hint
`let a = fraction * std::f64::consts::TAU;` and round with `(v * 100.0).round() / 100.0`. Watch out for `-0`: adding `0.0` to a rounded value turns `-0.0` into `0.0`. Format the arc with `format!("M12 3A9 9 0 {flag} 1 {x} {y}")`. Bars: `(level + 24) / 25` with integer division.
@@solution
fn ring_point(cx: f64, cy: f64, r: f64, fraction: f64) -> (f64, f64) {
    let a = fraction * std::f64::consts::TAU;
    let round = |v: f64| (v * 100.0).round() / 100.0 + 0.0;
    (round(cx + r * a.sin()), round(cy - r * a.cos()))
}

fn progress_arc(fraction: f64) -> String {
    let (x, y) = ring_point(12.0, 12.0, 9.0, fraction);
    let flag = if fraction > 0.5 { 1 } else { 0 };
    format!("M12 3A9 9 0 {flag} 1 {x} {y}")
}

fn battery_bars(level: u32) -> u32 {
    (level.min(100) + 24) / 25
}

fn main() {
    for f in [0.25, 0.5, 0.75] {
        println!("{f}: {}", progress_arc(f));
    }
    println!("{:?}", [0, 1, 25, 26, 99, 100].map(battery_bars));
}
