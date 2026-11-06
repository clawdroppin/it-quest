@@lesson ru0-a11y-keyboard Accessible Rust UIs: AccessKit, Roles, Names and Keyboard Navigation
@@concepts objects, lists, conditions
@@lang rust
@@text
A UI that only works with a mouse and only makes sense to sighted users shuts people out. It also fails automated testing, since UI test tools drive apps through the same accessibility tree as screen readers. In Rust, **AccessKit** is the shared accessibility layer: egui, iced (in progress), Slint and Xilem build an AccessKit tree, and AccessKit adapts it to Windows UI Automation, macOS NSAccessibility and Linux AT-SPI.

## The accessibility tree

- Every meaningful widget becomes a **node** with a **role** (Button, CheckBox, TextInput, Slider, List, Dialog...), a **name** (what a screen reader says: the label), a **value** or state (checked, expanded, 40%), and **actions** (Click, Focus, Increment).
- Icon-only buttons need an explicit name. In egui, a button's text is its name, so an icon-only button needs `.on_hover_text()` plus a label via `Response::widget_info`. In Slint, set `accessible-label`.
- Decorative images and layout containers shouldn't add noise to the tree.
- Changes that appear without focus moving (toasts, validation errors) should be **announced** (a live region).

## Keyboard navigation

- **Tab / Shift+Tab** move focus between interactive widgets in a logical order (usually reading order). egui handles this, and custom widgets must request focus with `Sense::focusable_noninteractive()` or `ui.memory_mut(|m| m.request_focus(id))`.
- **Arrow keys** move **within** a composite widget (a list, a grid, tabs, a radio group): **roving focus**, where only one item of the group is a Tab stop.
- **Enter/Space** activate, **Escape** closes dialogs and menus, and **Home/End** jump to the first and last item.
- Focus must be **visible** (a ring that contrasts with the background) and must never be lost (closing a dialog returns focus to the button that opened it).

## Testing it

- Windows: Accessibility Insights, Narrator. macOS: VoiceOver (Cmd+F5). Linux: Orca, Accerciser to inspect the tree.
- Unplug the mouse and try every task with the keyboard alone.

@@quiz
What does AccessKit do?
* Provides one accessibility tree that is adapted to each OS's accessibility API
- Draws widgets with the GPU
- Translates the UI
- Adds keyboard shortcuts automatically to every app
? Windows UIA, macOS NSAccessibility, Linux AT-SPI.

@@quiz
An icon-only "trash" button is announced as "button". What's missing?
* An accessible name, such as "Delete"
- A role
- A tab index
- A tooltip colour
? Names are what screen readers say.

@@quiz
In a list of 50 items, how should keyboard users move between items?
* Arrow keys within the list, with only one item in the Tab order (roving focus)
- Tab through all 50 items
- Only with the mouse
- Page Up and Page Down only
? Tab moves between widgets, arrows within them.

@@quiz
A dialog closes. Where should keyboard focus go?
* Back to the control that opened it
- To the top of the window
- Nowhere
- To the first button on the page
? Never lose focus.

@@quiz
Which node property tells a screen reader that a checkbox is ticked?
- Name
- Role
* Value or state (checked)
- Bounds
? Role says what it is, state says how it is.

@@quiz
What should a toast message that appears without moving focus be?
* Announced through a live region
- Hidden from screen readers
- Given keyboard focus every time
- Ignored
? Announce, don't steal focus.

@@flashcards
AccessKit :: cross-platform Rust accessibility tree
node :: role, name, value, actions
role :: what the widget is: Button, CheckBox...
name :: what the screen reader says
value / state :: checked, expanded, 40%
actions :: Click, Focus, Increment
icon-only button :: needs an explicit name
live region :: announce changes without moving focus
Tab / Shift+Tab :: between widgets
arrow keys :: within a composite widget
roving focus :: one Tab stop per group
Escape :: close dialogs and menus
focus return :: back to the opener
Accerciser :: inspect the AT-SPI tree on Linux

@@drill
Which key usually closes a dialog or menu?
= Escape
= esc
= Esc
? Then return focus to the opener.

@@drill
What is the technique called where only one item of a group is in the Tab order and arrows move within it?
= roving focus
= roving tabindex
? Used for lists, grids, tabs and toolbars.

@@exercise Audit an accessibility tree
Walk a widget tree and report problems: interactive nodes (buttons, checkboxes, text inputs, sliders) without a name, checkboxes without a checked state, images that aren't marked decorative but have no name, and the count of focusable nodes in order. Return the problems as strings.
@@starter
#[derive(Debug, Clone, Copy, PartialEq)]
enum Role { Window, Group, Button, CheckBox, TextInput, Slider, Image, Label }

struct Node { role: Role, name: Option<String>, checked: Option<bool>, decorative: bool, children: Vec<Node> }

fn node(role: Role, name: Option<&str>) -> Node {
    Node { role, name: name.map(String::from), checked: None, decorative: false, children: vec![] }
}

fn audit(root: &Node) -> (Vec<String>, usize) {
    todo!()
}

fn main() {
    let mut toolbar = node(Role::Group, None);
    toolbar.children = vec![node(Role::Button, Some("Save")), node(Role::Button, None), node(Role::Image, None)];
    let mut root = node(Role::Window, Some("Settings"));
    let mut dark = node(Role::CheckBox, Some("Dark mode"));
    dark.checked = Some(true);
    root.children = vec![toolbar, dark, node(Role::CheckBox, Some("Sync")), node(Role::Slider, None)];
    let (problems, focusable) = audit(&root);
    println!("{focusable} focusable");
    for p in problems { println!("- {p}"); }
}
@@tests
#[test]
fn finds_problems() {
    let mut toolbar = node(Role::Group, None);
    let mut logo = node(Role::Image, None);
    logo.decorative = true;
    toolbar.children = vec![node(Role::Button, Some("Save")), node(Role::Button, None), node(Role::Image, None), logo];
    let mut root = node(Role::Window, Some("Settings"));
    let mut dark = node(Role::CheckBox, Some("Dark mode"));
    dark.checked = Some(false);
    root.children = vec![toolbar, dark, node(Role::CheckBox, Some("Sync")), node(Role::Slider, None), node(Role::TextInput, Some("  "))];
    let (problems, focusable) = audit(&root);
    assert_eq!(focusable, 6, "2 buttons, 2 checkboxes, 1 slider, 1 text input");
    assert_eq!(problems.len(), 5, "{problems:?}");
    assert!(problems.iter().any(|p| p.contains("Button") && p.contains("name")));
    assert!(problems.iter().any(|p| p.contains("Sync") && p.contains("checked")));
    assert!(problems.iter().any(|p| p.contains("Image")));
    assert!(problems.iter().any(|p| p.contains("TextInput")), "blank names count as missing");
}

#[test]
fn clean_tree() {
    let mut root = node(Role::Window, Some("App"));
    root.children = vec![node(Role::Button, Some("OK")), node(Role::Label, None)];
    assert_eq!(audit(&root), (vec![], 1));
}
@@hint
Write a recursive helper `fn walk(n: &Node, problems: &mut Vec<String>, focusable: &mut usize)`. A name is missing if it's `None` or only whitespace (`trim().is_empty()`). Interactive roles are Button, CheckBox, TextInput and Slider. Use `format!("{:?} has no name", n.role)` style messages, and mention the checkbox's name when it has no checked state.
@@solution
#[derive(Debug, Clone, Copy, PartialEq)]
enum Role { Window, Group, Button, CheckBox, TextInput, Slider, Image, Label }

struct Node { role: Role, name: Option<String>, checked: Option<bool>, decorative: bool, children: Vec<Node> }

fn node(role: Role, name: Option<&str>) -> Node {
    Node { role, name: name.map(String::from), checked: None, decorative: false, children: vec![] }
}

fn walk(n: &Node, problems: &mut Vec<String>, focusable: &mut usize) {
    let named = n.name.as_deref().map_or(false, |s| !s.trim().is_empty());
    let interactive = matches!(n.role, Role::Button | Role::CheckBox | Role::TextInput | Role::Slider);
    if interactive {
        *focusable += 1;
        if !named {
            problems.push(format!("{:?} has no name", n.role));
        }
    }
    if n.role == Role::CheckBox && n.checked.is_none() {
        problems.push(format!("CheckBox '{}' has no checked state", n.name.as_deref().unwrap_or("?")));
    }
    if n.role == Role::Image && !n.decorative && !named {
        problems.push("Image has no name and isn't marked decorative".to_string());
    }
    for c in &n.children {
        walk(c, problems, focusable);
    }
}

fn audit(root: &Node) -> (Vec<String>, usize) {
    let (mut problems, mut focusable) = (Vec::new(), 0);
    walk(root, &mut problems, &mut focusable);
    (problems, focusable)
}

fn main() {
    let mut toolbar = node(Role::Group, None);
    toolbar.children = vec![node(Role::Button, Some("Save")), node(Role::Button, None), node(Role::Image, None)];
    let mut root = node(Role::Window, Some("Settings"));
    let mut dark = node(Role::CheckBox, Some("Dark mode"));
    dark.checked = Some(true);
    root.children = vec![toolbar, dark, node(Role::CheckBox, Some("Sync")), node(Role::Slider, None)];
    let (problems, focusable) = audit(&root);
    println!("{focusable} focusable");
    for p in problems { println!("- {p}"); }
}

@@exercise Roving focus in a grid
Implement keyboard navigation for a grid of items (like an icon picker) with `cols` columns and `len` items: arrows move by one cell or one row and stop at the edges (no wrapping), Home and End go to the first and last item in the current row, Ctrl+Home and Ctrl+End to the first and last item overall. The last row may be partial: moving down into a missing cell goes to the last item.
@@starter
#[derive(Clone, Copy)]
enum Key { Left, Right, Up, Down, Home, End, CtrlHome, CtrlEnd }

fn next_focus(current: usize, key: Key, cols: usize, len: usize) -> usize {
    todo!()
}

fn main() {
    let (cols, len) = (4, 10);
    let mut f = 0;
    for key in [Key::Right, Key::Down, Key::Down, Key::Down, Key::End, Key::CtrlHome] {
        f = next_focus(f, key, cols, len);
        print!("{f} ");
    }
    println!();
}
@@tests
#[test]
fn moves_within_bounds() {
    let (c, n) = (4, 10);
    assert_eq!(next_focus(0, Key::Left, c, n), 0, "stays at the left edge");
    assert_eq!(next_focus(3, Key::Right, c, n), 3, "stays at the right edge of the row");
    assert_eq!(next_focus(1, Key::Down, c, n), 5);
    assert_eq!(next_focus(5, Key::Down, c, n), 9, "row 3 has only items 8 and 9");
    assert_eq!(next_focus(7, Key::Down, c, n), 9, "missing cell goes to the last item");
    assert_eq!(next_focus(9, Key::Down, c, n), 9);
    assert_eq!(next_focus(2, Key::Up, c, n), 2);
    assert_eq!(next_focus(9, Key::Right, c, n), 9, "end of the list");
}

#[test]
fn home_end() {
    let (c, n) = (4, 10);
    assert_eq!(next_focus(6, Key::Home, c, n), 4);
    assert_eq!(next_focus(5, Key::End, c, n), 7);
    assert_eq!(next_focus(8, Key::End, c, n), 9, "partial last row");
    assert_eq!(next_focus(6, Key::CtrlHome, c, n), 0);
    assert_eq!(next_focus(2, Key::CtrlEnd, c, n), 9);
}
@@hint
Compute `row = current / cols` and `col = current % cols`. Left: `if col > 0 { current - 1 }`. Right: only if `col + 1 < cols && current + 1 < len`. Down: `if current + cols < len { current + cols } else if row < (len - 1) / cols { len - 1 } else { current }`. End: `min(row * cols + cols - 1, len - 1)`.
@@solution
#[derive(Clone, Copy)]
enum Key { Left, Right, Up, Down, Home, End, CtrlHome, CtrlEnd }

fn next_focus(current: usize, key: Key, cols: usize, len: usize) -> usize {
    let (row, col) = (current / cols, current % cols);
    let last_row = (len - 1) / cols;
    match key {
        Key::Left => if col > 0 { current - 1 } else { current },
        Key::Right => if col + 1 < cols && current + 1 < len { current + 1 } else { current },
        Key::Up => if row > 0 { current - cols } else { current },
        Key::Down => {
            if current + cols < len { current + cols } else if row < last_row { len - 1 } else { current }
        }
        Key::Home => row * cols,
        Key::End => (row * cols + cols - 1).min(len - 1),
        Key::CtrlHome => 0,
        Key::CtrlEnd => len - 1,
    }
}

fn main() {
    let (cols, len) = (4, 10);
    let mut f = 0;
    for key in [Key::Right, Key::Down, Key::Down, Key::Down, Key::End, Key::CtrlHome] {
        f = next_focus(f, key, cols, len);
        print!("{f} ");
    }
    println!();
}
