@@lesson ru4-dpi-responsive DPI Scaling and Responsive Layouts: Logical Pixels, Snapping and Breakpoints
@@concepts numbers, conditions, functions
@@lang rust
@@text
The same app runs on a 1080p laptop at 125% scaling, a 4K monitor at 200%, and a phone-sized window. To look sharp and usable everywhere, a Rust UI needs two things: correct handling of **scale factors** (logical vs physical pixels), and **responsive layout** that changes structure at sensible widths.

## Logical and physical pixels

- **Physical pixels**: the screen's real pixels. **Logical pixels** (points): what your layout code uses. physical = logical × **scale factor**.
- winit reports `scale_factor()` (1.0, 1.25, 1.5, 2.0...) and sends `ScaleFactorChanged` when a window moves to another monitor. egui calls it `pixels_per_point`. iced and Slint handle it for you, but your custom drawing must use it.
- **Fractional scaling** (1.25, 1.5) makes 1-logical-pixel lines land between physical pixels and blur. Snap borders and thin lines to **physical** pixels: round(x × scale) / scale.
- Raster assets: load images at the physical size (`@2x` versions), or render SVG icons at the physical size.
- Text: font sizes are logical, and the rasteriser renders them at the physical size, so text is naturally sharp.

## Responsive layout

- Choose layout by the **window's logical width**, not the screen size: compact (< 600), medium (600–1024), expanded (≥ 1024) are common breakpoints (Material uses these window size classes).
  - Compact: one column, navigation in a bottom bar or a hamburger menu.
  - Medium: a navigation rail (icons only) and one or two panes.
  - Expanded: a full sidebar plus list and detail panes side by side.
- **Hysteresis**: when a user drags the window edge near a breakpoint, the layout can flicker between modes. Switch up at, say, 1024 but back down only below 1000.
- Minimum window size: set it (winit `with_min_inner_size`) so the compact layout always fits.

@@quiz
A window on a 150% display has a 400-logical-pixel-wide panel. How many physical pixels wide is it?
- 400
* 600
- 267
- 800
? physical = logical × scale.

@@quiz
Why do 1 px borders look blurry at 125% scaling?
* 1.25 physical pixels don't line up with the pixel grid; snap to whole physical pixels
- Borders aren't supported
- The GPU is too slow
- Scaling is always 2x
? round(x × scale) / scale.

@@quiz
What should layout breakpoints be based on?
* The window's logical width
- The monitor's physical resolution
- The operating system
- The DPI setting alone
? Users resize windows.

@@quiz
What does hysteresis prevent at a breakpoint?
* Flickering between layouts when the width hovers near the threshold
- Text blurring
- Memory leaks
- Scale factor changes
? Different thresholds for switching up and down.

@@quiz
Which winit event tells you a window moved to a monitor with a different scale?
* ScaleFactorChanged
- Resized only
- Focused
- RedrawRequested
? Re-layout and reload raster assets.

@@quiz
At 200% scale, what size should a 24-logical-pixel icon be rasterised at?
- 24 px
* 48 px
- 12 px
- 96 px
? Render SVGs at the physical size.

@@flashcards
logical pixel :: unit your layout code uses
physical pixel :: real screen pixel
scale factor :: physical ÷ logical
pixels_per_point :: egui's name for the scale factor
ScaleFactorChanged :: winit event when moving monitors
fractional scaling :: 1.25, 1.5: lines can blur
snap :: round(x × scale) / scale
@2x assets :: raster images for 200%
compact :: < 600 logical px wide
medium :: 600–1024
expanded :: ≥ 1024
navigation rail :: icon-only side navigation
hysteresis :: different up and down thresholds
min inner size :: keep the compact layout usable

@@drill
At scale 1.5, what is round(10.3 × 1.5) / 1.5? Give one decimal place.
= 10.0
= 10
? round(15.45) = 15, then 15 / 1.5 = 10.

@@drill
Which window size class is a 700-logical-pixel window?
= medium
? Between 600 and 1024.

@@exercise Snap geometry to physical pixels
Implement `snap(v, scale)` (round a logical coordinate to the nearest physical pixel and convert back), `snap_rect` that snaps both edges (so widths stay consistent with neighbours), and `line_offset` that returns where to centre a line of a given physical width so it's crisp (half-pixel offset for odd widths).
@@starter
fn snap(v: f32, scale: f32) -> f32 {
    todo!()
}

/// (x, y, w, h) in logical pixels -> snapped (x, y, w, h)
fn snap_rect(r: (f32, f32, f32, f32), scale: f32) -> (f32, f32, f32, f32) {
    todo!()
}

/// Logical coordinate at which to centre a line `width_px` physical pixels wide, near `v`.
fn line_offset(v: f32, width_px: u32, scale: f32) -> f32 {
    todo!()
}

fn main() {
    for scale in [1.0, 1.25, 1.5, 2.0] {
        println!("scale {scale}: rect {:?}, 1px line at {}", snap_rect((10.3, 4.6, 99.7, 20.2), scale), line_offset(10.3, 1, scale));
    }
}
@@tests
#[test]
fn snapping() {
    let close = |a: f32, b: f32| (a - b).abs() < 1e-4;
    assert!(close(snap(10.3, 1.0), 10.0));
    assert!(close(snap(10.3, 2.0), 10.5));
    assert!(close(snap(10.3, 1.25), 10.4), "12.875 physical rounds to 13, 13 / 1.25 = 10.4");
    let (x, y, w, h) = snap_rect((10.3, 4.6, 99.7, 20.2), 1.5);
    assert!(close(x * 1.5, (x * 1.5).round()) && close((x + w) * 1.5, ((x + w) * 1.5).round()), "both edges on physical pixels");
    assert!(close(y, snap(4.6, 1.5)) && close(y + h, snap(24.8, 1.5)));
    assert!(close(line_offset(10.0, 1, 1.0), 10.5), "odd width: half pixel");
    assert!(close(line_offset(10.0, 2, 1.0), 10.0), "even width: whole pixel");
    assert!(close(line_offset(10.0, 1, 2.0) * 2.0, 20.5));
}
@@hint
`snap`: `(v * scale).round() / scale`. `snap_rect`: snap x and x + w separately, then w = right − left (the same for y). `line_offset`: in physical pixels, take `(v * scale).floor()`, add 0.5 if the width is odd, and divide by the scale.
@@solution
fn snap(v: f32, scale: f32) -> f32 {
    (v * scale).round() / scale
}

/// (x, y, w, h) in logical pixels -> snapped (x, y, w, h)
fn snap_rect(r: (f32, f32, f32, f32), scale: f32) -> (f32, f32, f32, f32) {
    let (x0, y0) = (snap(r.0, scale), snap(r.1, scale));
    let (x1, y1) = (snap(r.0 + r.2, scale), snap(r.1 + r.3, scale));
    (x0, y0, x1 - x0, y1 - y0)
}

/// Logical coordinate at which to centre a line `width_px` physical pixels wide, near `v`.
fn line_offset(v: f32, width_px: u32, scale: f32) -> f32 {
    let base = (v * scale).floor();
    let physical = if width_px % 2 == 1 { base + 0.5 } else { base };
    physical / scale
}

fn main() {
    for scale in [1.0, 1.25, 1.5, 2.0] {
        println!("scale {scale}: rect {:?}, 1px line at {}", snap_rect((10.3, 4.6, 99.7, 20.2), scale), line_offset(10.3, 1, scale));
    }
}

@@exercise Window size classes with hysteresis
Implement a `Layout` state machine: given the current class and a new logical width, return the new class. Thresholds are 600 and 1024 going up, but the class only goes back down below 580 and 1000 respectively. Also implement `panes(class)` returning how many panes and which navigation to show.
@@starter
#[derive(Debug, Clone, Copy, PartialEq)]
enum Class { Compact, Medium, Expanded }

fn next_class(current: Class, width: f32) -> Class {
    todo!()
}

fn panes(class: Class) -> (u8, &'static str) {
    todo!()
}

fn main() {
    let mut c = Class::Compact;
    for w in [500.0, 610.0, 590.0, 570.0, 1030.0, 1010.0, 990.0, 400.0] {
        c = next_class(c, w);
        let (n, nav) = panes(c);
        println!("{w:>6}: {c:?} ({n} panes, {nav})");
    }
}
@@tests
#[test]
fn going_up() {
    assert_eq!(next_class(Class::Compact, 599.0), Class::Compact);
    assert_eq!(next_class(Class::Compact, 600.0), Class::Medium);
    assert_eq!(next_class(Class::Compact, 1100.0), Class::Expanded, "can jump two classes");
    assert_eq!(next_class(Class::Medium, 1024.0), Class::Expanded);
}

#[test]
fn hysteresis_going_down() {
    assert_eq!(next_class(Class::Medium, 590.0), Class::Medium, "stays until below 580");
    assert_eq!(next_class(Class::Medium, 579.0), Class::Compact);
    assert_eq!(next_class(Class::Expanded, 1010.0), Class::Expanded);
    assert_eq!(next_class(Class::Expanded, 999.0), Class::Medium);
    assert_eq!(next_class(Class::Expanded, 300.0), Class::Compact);
}

#[test]
fn pane_counts() {
    assert_eq!(panes(Class::Compact).0, 1);
    assert_eq!(panes(Class::Medium).0, 2);
    assert_eq!(panes(Class::Expanded), (3, "sidebar"));
}
@@hint
Compute the class you'd pick going up (`width >= 1024` → Expanded, `>= 600` → Medium). If it's higher than the current class, take it. Otherwise apply the lower thresholds: from Expanded, drop to Medium below 1000 (and to Compact below 580). From Medium, drop to Compact below 580. Deriving `PartialOrd` on the enum lets you compare classes.
@@solution
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
enum Class { Compact, Medium, Expanded }

fn next_class(current: Class, width: f32) -> Class {
    let up = if width >= 1024.0 { Class::Expanded } else if width >= 600.0 { Class::Medium } else { Class::Compact };
    if up >= current {
        return up;
    }
    let down = if width < 580.0 { Class::Compact } else if width < 1000.0 { Class::Medium } else { Class::Expanded };
    if down < current { down } else { current }
}

fn panes(class: Class) -> (u8, &'static str) {
    match class {
        Class::Compact => (1, "bottom bar"),
        Class::Medium => (2, "navigation rail"),
        Class::Expanded => (3, "sidebar"),
    }
}

fn main() {
    let mut c = Class::Compact;
    for w in [500.0, 610.0, 590.0, 570.0, 1030.0, 1010.0, 990.0, 400.0] {
        c = next_class(c, w);
        let (n, nav) = panes(c);
        println!("{w:>6}: {c:?} ({n} panes, {nav})");
    }
}
