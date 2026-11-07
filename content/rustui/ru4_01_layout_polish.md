@@lesson ru4-polish Layout, Typography and Polish: a Nice Rust UI End to End
@@concepts lists, numbers, strings
@@lang rust
@@text
A good-looking app comes from many small decisions applied consistently. This last lesson covers layout (how space gets shared), text that fits, keyboard and screen-reader access, and a checklist that turns a working Rust app into a polished one.

## Layout

All toolkits share the same ideas as CSS flexbox:

- **Rows and columns** with a **gap** between children and **padding** around them.
- Children have a **base size** and may **grow** to share leftover space (by weight) or **shrink** when there isn't enough.
- **Alignment** on both axes: start, center, end, stretch.

| Toolkit | Row / column | Grow / align |
|---|---|---|
| egui | `ui.horizontal`, `ui.vertical`, `egui::Grid`, `StripBuilder` / `egui_extras::TableBuilder`, panels (`SidePanel`, `TopBottomPanel`, `CentralPanel`) | `ui.add_sized`, `Layout::right_to_left`, `ui.with_layout(Layout::centered_and_justified(...))` |
| iced | `row![]`, `column![]`, `container` | `Length::Fill`, `Length::FillPortion(2)`, `.align_x(Center)`, `.spacing(8).padding(16)` |
| Slint | `HorizontalLayout`, `VerticalLayout`, `GridLayout` | `horizontal-stretch: 2`, `alignment: center`, `spacing`, `padding` |

The first exercise implements the core algorithm (base sizes, gaps, grow weights) that all of these run.

**App structure that looks professional**: a sidebar (200–260 px) for navigation, a top bar with the page title and actions, content in cards with 16–24 px padding, and a max content width (~720–960 px) so lines don't get too long on big screens. Leave empty space; crowding looks cheap.

## Text that fits

- Line length 50–80 characters for reading; 1.4–1.6 line height for paragraphs.
- Long labels in narrow places: **truncate with an ellipsis** (`…`) and show the full text in a tooltip (`response.on_hover_text(full)` in egui). The second exercise truncates correctly by characters, not bytes, so Japanese and emoji don't break.
- Numbers in tables: right-aligned, tabular (monospace) digits.
- Use real typography: `…` not `...`, `×` not `x`, `–` for ranges.

## Keyboard and accessibility

- Every action reachable with the keyboard: **Tab** order follows the visual order, **Enter/Space** activates, **Esc** closes dialogs, and common shortcuts (Ctrl+S, Ctrl+F) where they make sense. The third exercise computes focus order.
- A **visible focus ring** on every focusable control (lesson 2).
- **Screen readers**: egui, iced and Slint support **AccessKit**, which exposes your widgets to Narrator, NVDA, Orca and VoiceOver. Give icon-only buttons a label (`egui::Button::new(icon).on_hover_text("Delete")`, accessible names in Slint).
- Don't rely on colour alone: pair red with an icon and words ("⚠ Failed").
- Text size setting or zoom (`ctx.set_zoom_factor` in egui), and the reduced-motion setting from lesson 2.

## Feedback and states

- **Empty states**: a friendly illustration or icon, one sentence, and the main action button ("No notes yet. ➕ New note").
- **Loading**: a spinner for short waits, a progress bar with text for long ones; never freeze the window (run work on a thread, `ctx.request_repaint()` when done).
- **Toasts** for completed actions (the `egui-notify` crate, or your own animated panel), with **Undo** where possible instead of "Are you sure?" dialogs.
- **Errors** in plain words, next to the thing that failed, with what to do next.

## The polish checklist

1. One `Theme`: palette, spacing scale, radius, type scale (lesson 1). No hard-coded colours in widgets.
2. Light and dark themes, following the system by default.
3. Custom font loaded; consistent icon set with `currentColor` (lesson 3).
4. Every interactive element has hover, pressed, focus and disabled states, animated 80–150 ms with ease-out (lessons 0 and 2).
5. Layout aligned to the spacing grid, generous padding, max content width.
6. Text fits: truncation with tooltips, correct ellipsis.
7. Keyboard navigation and AccessKit labels.
8. Empty, loading, error and success states designed, not left blank.
9. Window: sensible minimum size, remembered size and position, app icon in the title bar and the executable.
10. Release build checked for smooth 60 fps animation.

Build a small app (a habit tracker, a pomodoro timer, a notes app) and go through the list; then compare it with apps you admire and steal their best ideas.

@@quiz
In iced, how do you make one column take twice as much leftover width as another?
- `.width(200)`
* `Length::FillPortion(2)` for the wider one and `Length::FillPortion(1)` for the other
- Put it in a `container` twice
- It's not possible
? Grow weights share the leftover space proportionally.

@@quiz
Why truncate strings by characters instead of bytes in Rust?
* Slicing a UTF-8 string in the middle of a multi-byte character (Japanese, emoji) panics or breaks the text
- Bytes are slower
- Fonts are measured in characters
- Rust strings have no bytes
? `s.chars().take(n)` is safe; `&s[..n]` may not be.

@@quiz
What does AccessKit give a Rust GUI?
- Faster rendering
* Screen reader and accessibility API support (Narrator, NVDA, Orca, VoiceOver)
- Automatic dark mode
- Icon fonts
? Icon-only buttons still need text labels for it.

@@quiz
A delete action happens instantly. What's the friendliest way to protect against mistakes?
- A modal "Are you absolutely sure?" every time
* Do it, then show a toast with Undo for a few seconds
- Disable deleting
- Ask for a password
? Undo is faster for the common case and still safe.

@@drill
Which key should move focus to the next control?
= Tab
? Shift+Tab moves back.

@@flashcards
gap / padding :: space between children / around them
grow weight :: share of leftover space (FillPortion, stretch)
egui panels :: SidePanel, TopBottomPanel, CentralPanel
max content width :: about 720–960 px keeps lines readable
ellipsis truncation :: shorten with … and show the rest in a tooltip
chars() vs bytes :: safe vs unsafe slicing for non-ASCII text
focus order :: Tab follows the visual order
AccessKit :: accessibility tree for screen readers
empty state :: icon, one sentence, the main action
toast with Undo :: friendly alternative to confirm dialogs
request_repaint :: wake egui after background work

@@exercise Distribute space in a row
Write `layout_row(available, gap, items)` where each item is `(base, grow)`. Leftover space = `available − sum(bases) − gap × (n − 1)`. If it's positive, give each item `leftover × grow / total_grow` extra (if total grow is 0, nobody grows). If it's negative, every item shrinks **proportionally to its base** (but never below 0). Return the item widths, each rounded to 1 decimal.
@@starter
fn layout_row(available: f64, gap: f64, items: &[(f64, f64)]) -> Vec<f64> {
    todo!()
}

fn main() {
    // A sidebar that stays 200 wide, a content area that grows 3x, an inspector that grows 1x.
    println!("{:?}", layout_row(1000.0, 16.0, &[(200.0, 0.0), (300.0, 3.0), (150.0, 1.0)]));
}
@@tests
#[test]
fn checks() {
    assert_eq!(layout_row(1000.0, 16.0, &[(200.0, 0.0), (300.0, 3.0), (150.0, 1.0)]), vec![200.0, 538.5, 229.5], "grow 3:1");
    assert_eq!(layout_row(300.0, 0.0, &[(100.0, 1.0), (100.0, 1.0)]), vec![150.0, 150.0], "equal grow");
    assert_eq!(layout_row(500.0, 10.0, &[(100.0, 0.0), (100.0, 0.0)]), vec![100.0, 100.0], "nobody grows");
    assert_eq!(layout_row(200.0, 0.0, &[(100.0, 1.0), (300.0, 1.0)]), vec![50.0, 150.0], "shrink by base");
    assert_eq!(layout_row(100.0, 10.0, &[(100.0, 1.0), (100.0, 1.0), (100.0, 1.0)]), vec![26.7, 26.7, 26.7], "shrink with gaps");
    assert_eq!(layout_row(10.0, 50.0, &[(10.0, 0.0), (10.0, 0.0)]), vec![0.0, 0.0], "never negative");
    assert_eq!(layout_row(100.0, 8.0, &[]), Vec::<f64>::new(), "no items");
}
@@hint
`let n = items.len();` (return early when 0). `let bases: f64 = items.iter().map(|i| i.0).sum();`. For shrinking: `width = base + leftover * base / bases`, then `.max(0.0)`. Round with `(w * 10.0).round() / 10.0`.
@@solution
fn layout_row(available: f64, gap: f64, items: &[(f64, f64)]) -> Vec<f64> {
    let n = items.len();
    if n == 0 {
        return vec![];
    }
    let bases: f64 = items.iter().map(|i| i.0).sum();
    let grows: f64 = items.iter().map(|i| i.1).sum();
    let leftover = available - bases - gap * (n as f64 - 1.0);
    let round = |w: f64| (w * 10.0).round() / 10.0;
    items
        .iter()
        .map(|&(base, grow)| {
            let w = if leftover >= 0.0 {
                if grows > 0.0 { base + leftover * grow / grows } else { base }
            } else if bases > 0.0 {
                base + leftover * base / bases
            } else {
                0.0
            };
            round(w.max(0.0))
        })
        .collect()
}

fn main() {
    // A sidebar that stays 200 wide, a content area that grows 3x, an inspector that grows 1x.
    println!("{:?}", layout_row(1000.0, 16.0, &[(200.0, 0.0), (300.0, 3.0), (150.0, 1.0)]));
}

@@exercise Truncate with an ellipsis
Write `truncate(s, max)` that returns `s` unchanged if it has at most `max` **characters**, otherwise its first `max − 1` characters (with trailing spaces removed) followed by `…`. `max` of 0 gives an empty string and 1 gives just `…`. Then `middle(s, max)` for file paths: keep the start and the end, `…` in the middle; the start gets the extra character when `max − 1` is odd.
@@starter
fn truncate(s: &str, max: usize) -> String {
    todo!()
}

fn middle(s: &str, max: usize) -> String {
    todo!()
}

fn main() {
    println!("{}", truncate("Quarterly infrastructure review", 16));
    println!("{}", truncate("東京都の天気予報です", 6));
    println!("{}", middle("C:/Users/aiko/Projects/rust-ui/src/main.rs", 20));
}
@@tests
#[test]
fn checks() {
    assert_eq!(truncate("Save", 10), "Save", "fits");
    assert_eq!(truncate("Quarterly infrastructure review", 16), "Quarterly infra…", "cut");
    assert_eq!(truncate("Hello world again", 7), "Hello…", "no space before the ellipsis");
    assert_eq!(truncate("東京都の天気予報です", 6), "東京都の天…", "characters, not bytes");
    assert_eq!(truncate("🎨🖌️✨🌈", 3), "🎨🖌…", "emoji are safe");
    assert_eq!(truncate("abc", 0), "", "zero");
    assert_eq!(truncate("abc", 1), "…", "one");
    assert_eq!(middle("C:/Users/aiko/Projects/rust-ui/src/main.rs", 20), "C:/Users/a…c/main.rs", "middle");
    assert_eq!(middle("short.txt", 20), "short.txt", "fits");
    assert_eq!(middle("abcdefgh", 5), "ab…gh", "even split");
    assert_eq!(middle("abcdefgh", 6), "abc…gh", "start gets the extra one");
}
@@hint
`let chars: Vec<char> = s.chars().collect();`. For `truncate`, `chars[..max - 1].iter().collect::<String>().trim_end().to_string() + "…"`. For `middle`, `let keep = max - 1; let head = (keep + 1) / 2; let tail = keep / 2;` and join `chars[..head]`, `…`, `chars[len - tail..]`.
@@solution
fn truncate(s: &str, max: usize) -> String {
    let chars: Vec<char> = s.chars().collect();
    if chars.len() <= max {
        return s.to_string();
    }
    if max == 0 {
        return String::new();
    }
    let head: String = chars[..max - 1].iter().collect();
    format!("{}…", head.trim_end())
}

fn middle(s: &str, max: usize) -> String {
    let chars: Vec<char> = s.chars().collect();
    if chars.len() <= max {
        return s.to_string();
    }
    if max == 0 {
        return String::new();
    }
    let keep = max - 1;
    let head = (keep + 1) / 2;
    let tail = keep / 2;
    let start: String = chars[..head].iter().collect();
    let end: String = chars[chars.len() - tail..].iter().collect();
    format!("{start}…{end}")
}

fn main() {
    println!("{}", truncate("Quarterly infrastructure review", 16));
    println!("{}", truncate("東京都の天気予報です", 6));
    println!("{}", middle("C:/Users/aiko/Projects/rust-ui/src/main.rs", 20));
}

@@exercise Keyboard focus order
Focusable widgets are `(name, x, y, enabled)`. Write `focus_order(widgets)`: the names of **enabled** widgets in reading order: sorted by row (`y`, treating values within 10 px as the same row) then by `x`. Then `next_focus(order, current, backwards)`: the next name after `current` (Tab) or before it (Shift+Tab), wrapping around; with no current focus, Tab goes to the first and Shift+Tab to the last. Return `None` if the order is empty.
@@starter
fn focus_order(widgets: &[(&str, f32, f32, bool)]) -> Vec<String> {
    todo!()
}

fn next_focus(order: &[String], current: Option<&str>, backwards: bool) -> Option<String> {
    todo!()
}

fn main() {
    let form = [("save", 300.0, 200.0, true), ("name", 20.0, 40.0, true), ("email", 220.0, 44.0, true), ("cancel", 200.0, 202.0, true), ("admin", 20.0, 120.0, false)];
    let order = focus_order(&form);
    println!("{order:?}");
    println!("{:?}", next_focus(&order, Some("save"), false));
}
@@tests
#[test]
fn checks() {
    let form = [("save", 300.0, 200.0, true), ("name", 20.0, 40.0, true), ("email", 220.0, 44.0, true), ("cancel", 200.0, 202.0, true), ("admin", 20.0, 120.0, false)];
    let order = focus_order(&form);
    assert_eq!(order, vec!["name", "email", "cancel", "save"], "rows, then left to right, skipping disabled");
    assert_eq!(next_focus(&order, Some("name"), false).as_deref(), Some("email"), "tab");
    assert_eq!(next_focus(&order, Some("save"), false).as_deref(), Some("name"), "wraps forward");
    assert_eq!(next_focus(&order, Some("name"), true).as_deref(), Some("save"), "shift+tab wraps back");
    assert_eq!(next_focus(&order, None, false).as_deref(), Some("name"), "first tab");
    assert_eq!(next_focus(&order, None, true).as_deref(), Some("save"), "first shift+tab");
    assert_eq!(next_focus(&[], None, false), None, "nothing focusable");
}
@@hint
Group rows by rounding: `let row = (y / 10.0).round()` isn't quite "within 10 px"; a simpler approach is to sort by `y`, then walk the list starting a new row whenever `y` is more than 10 below the current row's first `y`, assigning a row number; then sort by `(row, x)`. Use `partial_cmp` for floats.
@@solution
fn focus_order(widgets: &[(&str, f32, f32, bool)]) -> Vec<String> {
    let mut items: Vec<(&str, f32, f32)> = widgets.iter().filter(|w| w.3).map(|w| (w.0, w.1, w.2)).collect();
    items.sort_by(|a, b| a.2.partial_cmp(&b.2).unwrap());
    let mut rows: Vec<(usize, f32, &str)> = Vec::new();
    let (mut row, mut row_y) = (0usize, f32::NEG_INFINITY);
    for (name, x, y) in items {
        if y - row_y > 10.0 {
            row += 1;
            row_y = y;
        }
        rows.push((row, x, name));
    }
    rows.sort_by(|a, b| a.0.cmp(&b.0).then(a.1.partial_cmp(&b.1).unwrap()));
    rows.into_iter().map(|(_, _, n)| n.to_string()).collect()
}

fn next_focus(order: &[String], current: Option<&str>, backwards: bool) -> Option<String> {
    if order.is_empty() {
        return None;
    }
    let n = order.len();
    let i = match current.and_then(|c| order.iter().position(|o| o == c)) {
        Some(i) => if backwards { (i + n - 1) % n } else { (i + 1) % n },
        None => if backwards { n - 1 } else { 0 },
    };
    Some(order[i].clone())
}

fn main() {
    let form = [("save", 300.0, 200.0, true), ("name", 20.0, 40.0, true), ("email", 220.0, 44.0, true), ("cancel", 200.0, 202.0, true), ("admin", 20.0, 120.0, false)];
    let order = focus_order(&form);
    println!("{order:?}");
    println!("{:?}", next_focus(&order, Some("save"), false));
}
