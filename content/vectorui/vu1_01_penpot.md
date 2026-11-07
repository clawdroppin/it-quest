@@lesson vu1-penpot Penpot: Boards, Layout, Components and Prototypes
@@concepts hello, files
@@lang html
@@text
Penpot is where app screens get designed before anyone writes code, and because it speaks SVG and CSS, the design translates directly into a real interface. This lesson goes through Penpot the way you'll use it: set up a file, design a screen with flex layout, turn repeated pieces into components, connect screens into a prototype, and hand it off.

## The workspace

| Area | What it holds |
|---|---|
| **Left sidebar** | Layers (the tree of everything), Assets (components, colours, typographies of this file and shared libraries), Pages |
| **Canvas** | your boards (artboards); zoom with Ctrl+scroll, pan with Space+drag |
| **Right sidebar** | Design (size, layout, fill, stroke, shadow, blur, text), **Prototype** (interactions), **Inspect** (code) |
| **Top bar** | file menu, history, sharing, view mode (presentation) |

Essential shortcuts: **B** board, **R** rectangle, **E** ellipse, **T** text, **P** path, **K** curve (pencil), **Ctrl+G** group, **Ctrl+K** make component, **Shift+A** add flex layout, **Ctrl+Shift+A** grid layout, **Ctrl+D** duplicate, **Alt+drag** duplicate while moving, **Shift+2** zoom to selection, **Alt** (hold) to see distances. Press **?** for the full list.

## Boards

A **board** is a screen, a frame or a container. Create one with **B** and pick a preset size from the right sidebar (iPhone, Android, Desktop 1440 × 1024, or your app's window size). Boards can be nested: a card is a board inside the screen board, with its own fill, radius, shadow and **clip content**.

**Constraints** (left, right, center, scale...) tell a child how to behave when its board is resized: a footer stuck to the bottom, a sidebar that keeps its width.

## Flex layout and grid layout

Add **flex layout** to a board (Shift+A) and its children arrange themselves in a row or column, exactly like CSS flexbox:

| Penpot setting | CSS |
|---|---|
| Direction: row / column | `flex-direction` |
| Gap (row and column) | `gap` |
| Padding | `padding` |
| Align items / justify content | `align-items`, `justify-content` |
| Wrap | `flex-wrap` |
| Child sizing: Fix / **Fill** / **Fit content** | fixed `width`, `flex: 1`, `width: fit-content` |
| Child "absolute position" | `position: absolute` |

**Grid layout** (Ctrl+Shift+A) defines rows and columns (`fr`, `px`, `%`, auto) and areas, like CSS grid: perfect for dashboards and galleries.

Design with layouts from the start: buttons that grow with their label (Fit content, padding 10/20), cards that stack in a column with a 16 px gap, a toolbar row with `space-between`. Then resizing a screen or changing a label never breaks the design, and Inspect gives the developer correct CSS. The first exercise rebuilds a Penpot layout in CSS.

## Colours, typographies and tokens

- **Colours** and **typographies** saved in Assets are reused everywhere; change the asset and every use updates.
- **Design tokens** (the Tokens panel) are named values (`color.accent`, `spacing.md`, `radius.button`, `font.size.body`) that can reference each other (`color.button.bg = {color.accent}`). Group them into **sets** (base, light, dark, brand) and **themes** that switch sets on and off, so the same design shows in light or dark with one click.
- Tokens follow the **W3C Design Tokens** JSON format, so they can be exported and turned into CSS variables, Rust constants or app themes (lesson 4).

## Components and variants

Anything used more than once (buttons, inputs, icons, list items, nav bars) should be a **component**:

1. Select the design → **Ctrl+K**. That's the **main component**.
2. Drag **copies** from Assets. Change the main and every copy updates; copies can **override** text, fills, icons and visibility without losing the link.
3. **Variants**: put several versions of a component together with properties, e.g. `Type = Primary | Secondary | Ghost`, `State = Default | Hover | Pressed | Disabled`, `Size = S | M | L`. A copy can switch variant from the right sidebar.
4. **Shared libraries**: publish a file's assets as a library and connect it to other files and team members; updates arrive as notifications to accept.

Nested components (an icon component inside a button component) keep things consistent: change the icon set once, every button follows.

## Prototyping

In the **Prototype** tab, select an element and add an **interaction**:

- trigger: *on click*, *mouse enter/leave*, *after delay*;
- action: *navigate to* a board, *open/close overlay* (menus, dialogs, toasts), *previous screen*, *open URL*;
- animation: *dissolve*, *slide*, *push*, with duration and easing.

Set a **flow** start, press **play** (presentation mode) and click through the app as if it were real. Share a view-only link to test with users before writing code: the cheapest way to find out a design doesn't work.

## Inspect and handoff

The **Inspect** tab shows any element's measurements, colours, typography, and **code**: CSS (including flex/grid), SVG, and HTML for the board. Developers can also download the assets with export settings (SVG, PNG at 1×/2×/3×, JPEG, PDF) you define per element in the Design tab. Use **comments** (C) to discuss details on the canvas.

## Plugins and automation

Penpot has a plugin system (Plugins manager, Ctrl+Alt+P) with plugins for icons (Lucide, Tabler, Material), lorem ipsum, contrast checks, table generation and more, written in JavaScript/TypeScript against the Penpot plugin API. Self-hosted instances also have an access-token API for exporting files from scripts.

@@quiz
What is the Penpot equivalent of `display: flex; flex-direction: column; gap: 16px; padding: 24px`?
* A board with flex layout, direction column, gap 16 and padding 24
- A group with a shadow
- A grid layout with 16 columns
- A component with 24 variants
? Penpot's layouts map one to one to CSS.

@@quiz
A button should get wider when its label gets longer. Which child sizing does the button use?
- Fix
* Fit content (with padding)
- Fill
- Scale constraint
? Fill would stretch it to the parent's width instead.

@@quiz
You change the corner radius of a main component. What happens to its copies?
* They all update, while keeping their own overrides (like their text)
- Nothing; copies are independent
- They're deleted
- Only copies on the same page update
? That's the point of components.

@@quiz
How do you show a design in light and dark versions with tokens?
- Duplicate every screen
* Put colour tokens in light and dark sets and switch between themes that activate one or the other
- Use a dark board background only
- Export twice with different PNG settings
? Tokens that reference other tokens make this cheap.

@@quiz
Where does a developer get CSS for an element?
- The Layers panel
* The Inspect tab, which shows measurements, colours, typography and code
- The Prototype tab
- Only from exported PNGs
? Exports for assets are set in the Design tab.

@@drill
What's the name of Penpot's container that acts like a screen or artboard?
= board
= a board
? Boards can use flex or grid layout.

@@flashcards
board :: a screen or container (B)
flex layout :: Shift+A, rows or columns like CSS flexbox
grid layout :: Ctrl+Shift+A, rows and columns like CSS grid
Fix / Fill / Fit content :: fixed size / take remaining space / hug the content
constraints :: how a child behaves when its board resizes
main component :: Ctrl+K, the source of truth
copy :: linked instance with overrides
variants :: versions of a component by properties (Type, State, Size)
shared library :: a file's assets used by other files
design tokens :: named values that can reference each other
token sets and themes :: switch light, dark and brand values
prototype interaction :: trigger, action, animation
overlay :: menu, dialog or toast shown over a screen
Inspect :: measurements and CSS/SVG/HTML code
export settings :: SVG, PNG 1×/2×/3×, PDF per element

@@exercise From Penpot layout to CSS
A Penpot settings card is designed like this: the board `.card` has flex layout, **column**, gap 16, padding 24, radius 16, white fill, a shadow, width 360. Inside it, `.row` has flex layout **row**, align items **center**, justify **space-between**, gap 12. The `.title` text is Fill (takes the remaining space). The `.toggle` is Fix 44 × 24 with radius 999. Write the CSS for these four classes.
@@starter
<!doctype html>
<html lang="en">
<head>
  <meta charset="utf-8">
  <title>Settings card</title>
  <style>
    body { font-family: system-ui, sans-serif; background: #f1f5f9; padding: 40px; }
    /* Your CSS here */
  </style>
</head>
<body>
  <div class="card">
    <div class="row"><span class="title">Dark mode</span><span class="toggle"></span></div>
    <div class="row"><span class="title">Notifications</span><span class="toggle"></span></div>
  </div>
</body>
</html>
@@requires
.card
display: flex
flex-direction: column
gap: 16px
padding: 24px
border-radius: 16px
box-shadow:
.row
align-items: center
justify-content: space-between
.title
flex: 1
.toggle
width: 44px
height: 24px
border-radius: 999px
@@hint
`.card { display: flex; flex-direction: column; gap: 16px; padding: 24px; border-radius: 16px; background: #fff; box-shadow: 0 8px 24px rgb(15 23 42 / .08); width: 360px; }` and `.row { display: flex; align-items: center; justify-content: space-between; gap: 12px; }`.
@@solution
<!doctype html>
<html lang="en">
<head>
  <meta charset="utf-8">
  <title>Settings card</title>
  <style>
    body { font-family: system-ui, sans-serif; background: #f1f5f9; padding: 40px; }
    .card { display: flex; flex-direction: column; gap: 16px; padding: 24px; border-radius: 16px; background: #fff;
            box-shadow: 0 8px 24px rgb(15 23 42 / .08); width: 360px; }
    .row { display: flex; align-items: center; justify-content: space-between; gap: 12px; }
    .title { flex: 1; color: #0f172a; }
    .toggle { width: 44px; height: 24px; border-radius: 999px; background: #6366f1; flex-shrink: 0; }
  </style>
</head>
<body>
  <div class="card">
    <div class="row"><span class="title">Dark mode</span><span class="toggle"></span></div>
    <div class="row"><span class="title">Notifications</span><span class="toggle"></span></div>
  </div>
</body>
</html>

@@exercise A grid layout dashboard
Penpot's grid layout maps to CSS grid. Build `.dashboard` with `display: grid`, columns `240px 1fr 1fr`, rows `64px 1fr`, gap 16, and named areas: the top row is `"side top top"`, the second `"side main stats"` (use `grid-template-areas`). Place `.side`, `.top`, `.main` and `.stats` with `grid-area`.
@@starter
<!doctype html>
<html lang="en">
<head>
  <meta charset="utf-8">
  <title>Dashboard</title>
  <style>
    body { margin: 0; font-family: system-ui, sans-serif; background: #0f172a; color: #e2e8f0; }
    .dashboard > div { background: #1e293b; border-radius: 12px; padding: 16px; }
    /* Your CSS here */
  </style>
</head>
<body>
  <div class="dashboard">
    <div class="side">Navigation</div>
    <div class="top">Search and profile</div>
    <div class="main">Chart</div>
    <div class="stats">Numbers</div>
  </div>
</body>
</html>
@@requires
display: grid
grid-template-columns: 240px 1fr 1fr
grid-template-rows: 64px 1fr
grid-template-areas:
"side top top"
"side main stats"
grid-area: side
grid-area: top
grid-area: main
grid-area: stats
@@hint
`.dashboard { display: grid; grid-template-columns: 240px 1fr 1fr; grid-template-rows: 64px 1fr; grid-template-areas: "side top top" "side main stats"; gap: 16px; padding: 16px; height: 100vh; box-sizing: border-box; }` then `.side { grid-area: side; }` and so on.
@@solution
<!doctype html>
<html lang="en">
<head>
  <meta charset="utf-8">
  <title>Dashboard</title>
  <style>
    body { margin: 0; font-family: system-ui, sans-serif; background: #0f172a; color: #e2e8f0; }
    .dashboard > div { background: #1e293b; border-radius: 12px; padding: 16px; }
    .dashboard { display: grid; grid-template-columns: 240px 1fr 1fr; grid-template-rows: 64px 1fr;
                 grid-template-areas: "side top top" "side main stats";
                 gap: 16px; padding: 16px; height: 100vh; box-sizing: border-box; }
    .side { grid-area: side; }
    .top { grid-area: top; }
    .main { grid-area: main; }
    .stats { grid-area: stats; }
  </style>
</head>
<body>
  <div class="dashboard">
    <div class="side">Navigation</div>
    <div class="top">Search and profile</div>
    <div class="main">Chart</div>
    <div class="stats">Numbers</div>
  </div>
</body>
</html>
