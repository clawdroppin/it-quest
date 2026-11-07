@@lesson ru1-themes Pretty Themes: Colour, Tokens, Type and Dark Mode in Rust
@@concepts objects, numbers, strings
@@lang rust
@@text
A beautiful app is mostly **consistency**: a small palette used the same way everywhere, one spacing scale, two or three text sizes, and rounded corners that match. Professional designers capture these decisions as **design tokens**, and a Rust app should too: one `Theme` struct the whole UI reads from, with a light and a dark version.

## A palette that works

Start with **one brand colour** and build everything else from neutrals:

| Token | Use | Light | Dark |
|---|---|---|---|
| `bg` | window background | `#f8fafc` | `#0f172a` |
| `surface` | cards, panels | `#ffffff` | `#1e293b` |
| `surface_alt` | hovered rows, inputs | `#f1f5f9` | `#273449` |
| `border` | separators | `#e2e8f0` | `#334155` |
| `text` | body text | `#0f172a` | `#e2e8f0` |
| `text_muted` | secondary text | `#64748b` | `#94a3b8` |
| `accent` | primary buttons, links, focus | `#6366f1` | `#818cf8` |
| `accent_text` | text on the accent | `#ffffff` | `#0f172a` |
| `success` / `warning` / `danger` | states | `#16a34a` / `#d97706` / `#dc2626` | lighter versions |

Rules of thumb:

- **60-30-10**: ~60% background neutrals, ~30% surfaces and text, ~10% accent. The accent marks what's clickable or important, so don't spend it on decoration.
- Dark mode isn't inverted light mode: use dark **blue-grey** backgrounds (not pure black), lighter and slightly desaturated accents, and less contrast for large surfaces.
- Derive hover and pressed colours from the base (mix with white or black by 8–15%) instead of picking random new colours. The exercises do this.
- **Contrast**: body text needs a WCAG contrast ratio of at least **4.5:1** against its background (3:1 for large text and UI shapes). The second exercise computes it, so your theme can test itself.

## Spacing, shape and type

- A **spacing scale**: 4, 8, 12, 16, 24, 32, 48. Never 13 or 17: the eye notices.
- **Corner radius**: one family, e.g. 6 for inputs, 10 for buttons, 16 for cards; nested shapes use a smaller radius than their container.
- **Type scale**: body 14–15 px, small 12, headings 18 / 24 / 32. One or two font families. egui and iced can load your own fonts (Inter, Noto Sans JP for Japanese, JetBrains Mono for code).
- **Elevation**: soft shadows (large blur, low opacity) separate layers in light mode; in dark mode use lighter surfaces instead.

## Themes in egui

egui's look is a `Style` containing `Visuals`. Change it once at startup (and again when the user switches theme):

```rust,norun
use eframe::egui::{self, Color32, CornerRadius, FontFamily, FontId, Stroke, TextStyle};

fn apply_theme(ctx: &egui::Context, dark: bool) {
    let mut visuals = if dark { egui::Visuals::dark() } else { egui::Visuals::light() };
    let accent = if dark { Color32::from_rgb(0x81, 0x8c, 0xf8) } else { Color32::from_rgb(0x63, 0x66, 0xf1) };
    visuals.panel_fill = if dark { Color32::from_rgb(0x0f, 0x17, 0x2a) } else { Color32::from_rgb(0xf8, 0xfa, 0xfc) };
    visuals.window_fill = if dark { Color32::from_rgb(0x1e, 0x29, 0x3b) } else { Color32::WHITE };
    visuals.selection.bg_fill = accent;
    visuals.hyperlink_color = accent;
    visuals.window_corner_radius = CornerRadius::same(16);
    for w in [&mut visuals.widgets.inactive, &mut visuals.widgets.hovered, &mut visuals.widgets.active] {
        w.corner_radius = CornerRadius::same(10);
    }
    visuals.widgets.hovered.bg_stroke = Stroke::new(1.0, accent);
    ctx.set_visuals(visuals);

    ctx.style_mut(|style| {
        style.spacing.item_spacing = egui::vec2(8.0, 8.0);
        style.spacing.button_padding = egui::vec2(14.0, 8.0);
        style.text_styles.insert(TextStyle::Heading, FontId::new(24.0, FontFamily::Proportional));
        style.text_styles.insert(TextStyle::Body, FontId::new(15.0, FontFamily::Proportional));
    });
}
```

Custom fonts: `egui::FontDefinitions::default()`, insert your `.ttf` bytes with `include_bytes!`, put its name first in the `Proportional` family, then `ctx.set_fonts(fonts)`. (Field names shift a little between egui versions; your editor's autocomplete shows the current ones.) The popular **catppuccin-egui** crate ships four ready-made pastel themes to study.

## Themes in iced and Slint

- **iced**: `iced::Theme` has built-in themes (`Theme::Dark`, `Theme::CatppuccinMocha`, `Theme::TokyoNight`...) and `Theme::custom(name, Palette { background, text, primary, success, danger })`; widgets take a `.style(|theme, status| ...)` closure where `status` is `Active`, `Hovered`, `Pressed` or `Disabled`: exactly the state machine from lesson 0.
- **Slint**: a `global Theme { out property <color> accent: dark ? #818cf8 : #6366f1; ... }` read by every component, and `Palette.color-scheme` for the system dark mode.
- **Follow the system**: eframe exposes the OS theme (`ctx.system_theme()` / `ThemePreference::System`), iced has `iced::system::theme()`; offer System / Light / Dark in settings and remember the choice.

## Your own Theme struct

Whatever the toolkit, keep tokens in one place:

```rust,norun
pub struct Theme {
    pub bg: Rgb, pub surface: Rgb, pub text: Rgb, pub muted: Rgb,
    pub accent: Rgb, pub accent_text: Rgb,
    pub radius: f32, pub space: [f32; 7],
}
impl Theme {
    pub fn light() -> Self { /* ... */ }
    pub fn dark() -> Self { /* ... */ }
}
```

The exercises build the colour maths such a struct needs: parsing hex, mixing colours for hover states, and checking contrast.

@@quiz
A theme's text is #94a3b8 on #ffffff, a contrast ratio of about 2.6:1. Is that OK for body text?
- Yes, any visible difference is fine
* No: body text needs at least 4.5:1; use a darker grey
- Only in dark mode
- Only for headings
? 3:1 is the minimum for large text and UI shapes.

@@quiz
How should hover and pressed colours be chosen?
- Random new colours for each state
* Derived from the base colour, for example mixed 8–15% with white or black
- Always pure white and pure black
- The same as the base
? Derived colours keep the palette consistent.

@@quiz
What is the 60-30-10 rule?
* About 60% neutral background, 30% surfaces and text, 10% accent colour
- 60 px padding, 30 px gaps, 10 px borders
- Three fonts in three sizes
- 60 fps, 30 fps, 10 fps
? The accent stays meaningful because it's rare.

@@quiz
In iced, how does a button look different when hovered?
- It can't; iced has no hover styles
* Its `.style(|theme, status| ...)` closure gets a status (Active, Hovered, Pressed, Disabled) and returns a style for it
- By setting a global CSS file
- With a separate HoverButton widget
? Styles are functions of theme and status.

@@drill
Which egui type holds the colours of a theme?
= Visuals
= egui::Visuals
? Visuals::dark() and Visuals::light().

@@flashcards
design tokens :: named design decisions: colours, spacing, radius, type sizes
bg / surface / text / muted / accent :: core colour tokens
60-30-10 :: neutrals, surfaces and text, accent
WCAG contrast :: 4.5:1 body text, 3:1 large text and UI shapes
spacing scale :: 4, 8, 12, 16, 24, 32, 48
nested radius :: inner shapes use a smaller radius than the container
egui Visuals :: colours and shapes of every widget state
ctx.set_visuals / ctx.style_mut :: apply a look / change spacing and text styles
FontDefinitions :: load custom fonts into egui
iced Theme::custom + Palette :: your own iced theme
Slint global Theme :: tokens shared by all components
system theme :: follow the OS light or dark preference

@@exercise Parse and format hex colours
Write `parse_hex(s) -> Option<(u8, u8, u8)>` accepting `"#6366f1"`, `"6366f1"` and the short form `"#63f"` (each digit doubled: `#63f` = `#6633ff`), any case; return `None` for anything else. Then `to_hex((r, g, b))` returning lowercase `"#rrggbb"`.
@@starter
fn parse_hex(s: &str) -> Option<(u8, u8, u8)> {
    todo!()
}

fn to_hex(c: (u8, u8, u8)) -> String {
    todo!()
}

fn main() {
    println!("{:?} {:?}", parse_hex("#6366F1"), parse_hex("#63f"));
    println!("{}", to_hex((99, 102, 241)));
}
@@tests
#[test]
fn checks() {
    assert_eq!(parse_hex("#6366f1"), Some((99, 102, 241)), "six digits");
    assert_eq!(parse_hex("6366F1"), Some((99, 102, 241)), "no hash, upper case");
    assert_eq!(parse_hex("#63f"), Some((0x66, 0x33, 0xff)), "short form");
    assert_eq!(parse_hex("#fff"), Some((255, 255, 255)), "white");
    assert_eq!(parse_hex("#12345"), None, "wrong length");
    assert_eq!(parse_hex("#gg0000"), None, "not hex");
    assert_eq!(to_hex((99, 102, 241)), "#6366f1", "format");
    assert_eq!(to_hex((0, 10, 255)), "#000aff", "leading zeros");
}
@@hint
`let h = s.strip_prefix('#').unwrap_or(s);`. For 3 digits, build a 6-digit string by repeating each char. Then `u8::from_str_radix(&h[0..2], 16).ok()?` for each pair (check `h.is_ascii()` first). Format with `format!("#{r:02x}{g:02x}{b:02x}")`.
@@solution
fn parse_hex(s: &str) -> Option<(u8, u8, u8)> {
    let h = s.strip_prefix('#').unwrap_or(s);
    if !h.is_ascii() {
        return None;
    }
    let full: String = match h.len() {
        3 => h.chars().flat_map(|c| [c, c]).collect(),
        6 => h.to_string(),
        _ => return None,
    };
    let part = |i: usize| u8::from_str_radix(&full[i..i + 2], 16).ok();
    Some((part(0)?, part(2)?, part(4)?))
}

fn to_hex(c: (u8, u8, u8)) -> String {
    format!("#{:02x}{:02x}{:02x}", c.0, c.1, c.2)
}

fn main() {
    println!("{:?} {:?}", parse_hex("#6366F1"), parse_hex("#63f"));
    println!("{}", to_hex((99, 102, 241)));
}

@@exercise Contrast ratio
Implement the WCAG formulas. `luminance(c)`: convert each channel to 0–1 (`v / 255`), linearise it (`v / 12.92` if `v <= 0.04045`, else `((v + 0.055) / 1.055)^2.4`), then `0.2126 R + 0.7152 G + 0.0722 B`. `contrast(a, b)`: `(L_lighter + 0.05) / (L_darker + 0.05)`. `passes(fg, bg, large)`: at least 4.5, or 3.0 for large text.
@@starter
type Rgb = (u8, u8, u8);

fn luminance(c: Rgb) -> f64 {
    todo!()
}

fn contrast(a: Rgb, b: Rgb) -> f64 {
    todo!()
}

fn passes(fg: Rgb, bg: Rgb, large: bool) -> bool {
    todo!()
}

fn main() {
    let white = (255, 255, 255);
    for (name, c) in [("slate-400", (0x94, 0xa3, 0xb8)), ("slate-500", (0x64, 0x74, 0x8b)), ("indigo-500", (0x63, 0x66, 0xf1))] {
        println!("{name} on white: {:.2}:1 body ok: {}", contrast(c, white), passes(c, white, false));
    }
}
@@tests
#[test]
fn checks() {
    let close = |a: f64, b: f64| (a - b).abs() < 0.01;
    assert!(close(luminance((255, 255, 255)), 1.0), "white is 1");
    assert!(close(luminance((0, 0, 0)), 0.0), "black is 0");
    assert!(close(contrast((0, 0, 0), (255, 255, 255)), 21.0), "black on white is 21:1");
    assert!(close(contrast((255, 255, 255), (0, 0, 0)), 21.0), "order doesn't matter");
    assert!(close(contrast((0x63, 0x66, 0xf1), (255, 255, 255)), 4.47), "indigo-500 on white");
    assert!(close(contrast((0x64, 0x74, 0x8b), (255, 255, 255)), 4.76), "slate-500 on white");
    assert!(!passes((0x94, 0xa3, 0xb8), (255, 255, 255), false), "slate-400 fails for body text");
    assert!(passes((0x64, 0x74, 0x8b), (255, 255, 255), false), "slate-500 passes");
    assert!(passes((0x63, 0x66, 0xf1), (255, 255, 255), true), "indigo passes for large text");
    assert!(!passes((0x63, 0x66, 0xf1), (255, 255, 255), false), "but not for body text");
}
@@hint
Write a closure `lin = |v: u8| { let v = v as f64 / 255.0; if v <= 0.04045 { v / 12.92 } else { ((v + 0.055) / 1.055).powf(2.4) } }`. For contrast, compute both luminances and use `max` and `min`.
@@solution
type Rgb = (u8, u8, u8);

fn luminance(c: Rgb) -> f64 {
    let lin = |v: u8| {
        let v = v as f64 / 255.0;
        if v <= 0.04045 { v / 12.92 } else { ((v + 0.055) / 1.055).powf(2.4) }
    };
    0.2126 * lin(c.0) + 0.7152 * lin(c.1) + 0.0722 * lin(c.2)
}

fn contrast(a: Rgb, b: Rgb) -> f64 {
    let (la, lb) = (luminance(a), luminance(b));
    (la.max(lb) + 0.05) / (la.min(lb) + 0.05)
}

fn passes(fg: Rgb, bg: Rgb, large: bool) -> bool {
    contrast(fg, bg) >= if large { 3.0 } else { 4.5 }
}

fn main() {
    let white = (255, 255, 255);
    for (name, c) in [("slate-400", (0x94, 0xa3, 0xb8)), ("slate-500", (0x64, 0x74, 0x8b)), ("indigo-500", (0x63, 0x66, 0xf1))] {
        println!("{name} on white: {:.2}:1 body ok: {}", contrast(c, white), passes(c, white, false));
    }
}

@@exercise A theme with derived state colours
Write `mix(a, b, t)`: each channel `a + (b - a) * t`, rounded (t from 0 to 1). Then a `Theme` with `accent`, `bg` and `dark: bool`, and methods `hover()` (mix the accent 12% towards white in dark themes, towards black in light themes), `pressed()` (24% the same way), `disabled()` (mix the accent 55% towards `bg`), and `text_on_accent()`: white or near-black `(15, 23, 42)`, whichever has the higher contrast against the accent (use the WCAG functions from the previous exercise, given here).
@@starter
type Rgb = (u8, u8, u8);

fn luminance(c: Rgb) -> f64 {
    let lin = |v: u8| {
        let v = v as f64 / 255.0;
        if v <= 0.04045 { v / 12.92 } else { ((v + 0.055) / 1.055).powf(2.4) }
    };
    0.2126 * lin(c.0) + 0.7152 * lin(c.1) + 0.0722 * lin(c.2)
}

fn contrast(a: Rgb, b: Rgb) -> f64 {
    let (la, lb) = (luminance(a), luminance(b));
    (la.max(lb) + 0.05) / (la.min(lb) + 0.05)
}

fn mix(a: Rgb, b: Rgb, t: f64) -> Rgb {
    todo!()
}

struct Theme { accent: Rgb, bg: Rgb, dark: bool }

impl Theme {
    fn hover(&self) -> Rgb { todo!() }
    fn pressed(&self) -> Rgb { todo!() }
    fn disabled(&self) -> Rgb { todo!() }
    fn text_on_accent(&self) -> Rgb { todo!() }
}

fn main() {
    let light = Theme { accent: (99, 102, 241), bg: (248, 250, 252), dark: false };
    println!("hover {:?} pressed {:?} disabled {:?} text {:?}", light.hover(), light.pressed(), light.disabled(), light.text_on_accent());
}
@@tests
#[test]
fn checks() {
    assert_eq!(mix((0, 0, 0), (255, 255, 255), 0.5), (128, 128, 128), "halfway");
    assert_eq!(mix((10, 20, 30), (10, 20, 30), 0.3), (10, 20, 30), "same colour");
    assert_eq!(mix((200, 100, 0), (0, 0, 0), 0.0), (200, 100, 0), "t = 0");
    let light = Theme { accent: (99, 102, 241), bg: (248, 250, 252), dark: false };
    assert_eq!(light.hover(), (87, 90, 212), "light hover darkens");
    assert_eq!(light.pressed(), (75, 78, 183), "light pressed darkens more");
    assert_eq!(light.disabled(), (181, 183, 247), "disabled fades into the background");
    assert_eq!(light.text_on_accent(), (255, 255, 255), "white on indigo");
    let dark = Theme { accent: (129, 140, 248), bg: (15, 23, 42), dark: true };
    assert_eq!(dark.hover(), (144, 154, 249), "dark hover lightens");
    assert_eq!(dark.text_on_accent(), (15, 23, 42), "dark text on a light accent");
}
@@hint
`mix`: a closure `ch = |x: u8, y: u8| (x as f64 + (y as f64 - x as f64) * t).round() as u8`. For hover/pressed, `let target = if self.dark { (255, 255, 255) } else { (0, 0, 0) };`. For the text, compare `contrast(white, accent)` with `contrast(ink, accent)`.
@@solution
type Rgb = (u8, u8, u8);

fn luminance(c: Rgb) -> f64 {
    let lin = |v: u8| {
        let v = v as f64 / 255.0;
        if v <= 0.04045 { v / 12.92 } else { ((v + 0.055) / 1.055).powf(2.4) }
    };
    0.2126 * lin(c.0) + 0.7152 * lin(c.1) + 0.0722 * lin(c.2)
}

fn contrast(a: Rgb, b: Rgb) -> f64 {
    let (la, lb) = (luminance(a), luminance(b));
    (la.max(lb) + 0.05) / (la.min(lb) + 0.05)
}

fn mix(a: Rgb, b: Rgb, t: f64) -> Rgb {
    let ch = |x: u8, y: u8| (x as f64 + (y as f64 - x as f64) * t).round() as u8;
    (ch(a.0, b.0), ch(a.1, b.1), ch(a.2, b.2))
}

struct Theme { accent: Rgb, bg: Rgb, dark: bool }

impl Theme {
    fn target(&self) -> Rgb {
        if self.dark { (255, 255, 255) } else { (0, 0, 0) }
    }
    fn hover(&self) -> Rgb { mix(self.accent, self.target(), 0.12) }
    fn pressed(&self) -> Rgb { mix(self.accent, self.target(), 0.24) }
    fn disabled(&self) -> Rgb { mix(self.accent, self.bg, 0.55) }
    fn text_on_accent(&self) -> Rgb {
        let (white, ink) = ((255, 255, 255), (15, 23, 42));
        if contrast(white, self.accent) >= contrast(ink, self.accent) { white } else { ink }
    }
}

fn main() {
    let light = Theme { accent: (99, 102, 241), bg: (248, 250, 252), dark: false };
    println!("hover {:?} pressed {:?} disabled {:?} text {:?}", light.hover(), light.pressed(), light.disabled(), light.text_on_accent());
}
