@@lesson gu2-responsive Responsive Interfaces: Fluid Type, Breakpoints and Container Queries
@@text
A desktop app window gets resized; a web app runs on a phone and a 4K monitor; a Tauri app may be docked in a narrow side panel. **Responsive** design means the layout adapts to the space it has, without separate versions. This lesson covers the modern toolbox: intrinsic layouts that need no breakpoints, fluid sizes with `clamp()`, media queries for big structural changes, and container queries for components that adapt to their own slot.

## Start intrinsic

Many layouts need no breakpoints at all:
- `display: grid; grid-template-columns: repeat(auto-fit, minmax(16rem, 1fr));` fits as many cards per row as there's room for.
- `display: flex; flex-wrap: wrap;` with sensible `flex-basis` values wraps toolbars and tag lists.
- `max-width: 65ch` keeps text readable on wide screens; `margin-inline: auto` centres it.
- Images: `max-width: 100%; height: auto;` and `aspect-ratio` for placeholders that don't shift the layout.

## Fluid sizes

`clamp(min, preferred, max)` scales a value with the viewport but within limits: `font-size: clamp(1rem, 0.9rem + 0.5vw, 1.25rem);`. Use it for headings, spacing and container padding. Prefer `rem` for type so the user's font-size setting still works (an accessibility requirement: WCAG 1.4.4 resize text to 200%).

## Media queries

For structural changes (a sidebar becoming a bottom bar), use **min-width** queries in a mobile-first style:

```css,norun
.layout { display: grid; gap: 1rem; }
@media (min-width: 48rem) { .layout { grid-template-columns: 16rem 1fr; } }
```

Write breakpoints in `rem`/`em` so they follow the zoom level. Other useful media features: `prefers-color-scheme`, `prefers-reduced-motion`, `prefers-contrast`, `hover: hover` and `pointer: coarse` (touch: make targets bigger), `orientation`.

## Container queries

A card in a narrow sidebar and the same card in a wide main column should look different, but the viewport is the same. **Container queries** respond to the component's container:

```css,norun
.card-slot { container-type: inline-size; container-name: card; }
@container card (min-width: 30rem) { .card { grid-template-columns: 8rem 1fr; } }
```

Container query units (`cqi`, `cqw`) size things relative to the container. Supported in all modern browsers (and WebView2/WebKitGTK used by Tauri).

## Desktop toolkits

WPF uses star sizing and `VisualStateManager` with adaptive triggers; WinUI has `AdaptiveTrigger` (MinWindowWidth); Qt uses layouts with size policies; egui can branch on `ui.available_width()`. The principle is the same: measure the space, choose a layout.

@@quiz
Which CSS function keeps a font size fluid but within limits?
- calc()
* clamp()
- min()
- var()
? clamp(min, preferred, max).

@@quiz
What does a container query respond to?
- The viewport width
* The size of the component's container
- The device pixel ratio
- The user's language
? Components adapt to their slot.

@@quiz
Why write breakpoints in rem or em?
* They follow the user's zoom and font size
- They're faster
- Pixels are invalid in media queries
- They work only on phones
? Accessible breakpoints.

@@quiz
Which media feature detects a touch screen as the main pointer?
- hover: hover
* pointer: coarse
- orientation: portrait
- prefers-contrast: more
? Make targets bigger.

@@quiz
`repeat(auto-fit, minmax(16rem, 1fr))` does what?
- Always makes three columns
* Fits as many columns of at least 16rem as there's room for
- Hides overflowing items
- Requires media queries
? Intrinsic responsive grid.

@@flashcards
intrinsic layout :: adapts without breakpoints
auto-fit + minmax :: responsive grid
flex-wrap :: wrap items to new lines
max-width: 65ch :: readable line length
aspect-ratio :: reserve space, avoid shifts
clamp() :: fluid value with limits
rem for type :: respects user font size
mobile-first :: min-width queries
pointer: coarse :: touch input
hover: hover :: device can hover
container-type: inline-size :: make a query container
@container :: style by container size
cqi :: container inline-size unit
AdaptiveTrigger :: WinUI responsive states

@@drill
Which CSS property turns an element into a size container for container queries?
= container-type
? Usually container-type: inline-size.

@@drill
Which media feature value detects that the device can hover?
= hover: hover
= hover:hover
= (hover: hover)
? Combine with pointer: fine for mice.

@@exercise A responsive dashboard
Make the dashboard adapt: mobile-first, the sidebar stacks above the content; from `48rem` it becomes a two-column grid (`16rem 1fr`). The stat cards use `repeat(auto-fit, minmax(12rem, 1fr))`. Headings use `clamp()`, the page text is limited to `65ch`, and on touch devices (`pointer: coarse`) buttons are at least 44 px tall.
@@starter
<!doctype html>
<html lang="en">
<head>
  <meta charset="utf-8">
  <meta name="viewport" content="width=device-width, initial-scale=1">
  <title>Dashboard</title>
  <style>
    body { margin: 0; font-family: system-ui, sans-serif; background: #f1f5f9; color: #0f172a; }
    .layout { display: grid; gap: 1rem; padding: 1rem; }
    nav, main { background: #fff; border-radius: 12px; padding: 1rem; }
    .stat { background: #eef2ff; border-radius: 10px; padding: 1rem; }
  </style>
</head>
<body>
  <div class="layout">
    <nav><button>Overview</button> <button>Reports</button> <button>Settings</button></nav>
    <main>
      <h1>Overview</h1>
      <p>Your projects at a glance. Resize the window to see the layout adapt.</p>
      <div class="stats">
        <div class="stat"><strong>12</strong> open tasks</div>
        <div class="stat"><strong>3</strong> reviews</div>
        <div class="stat"><strong>98%</strong> uptime</div>
        <div class="stat"><strong>4</strong> deploys today</div>
      </div>
    </main>
  </div>
</body>
</html>
@@requires
@media (min-width: 48rem)
16rem 1fr
repeat(auto-fit, minmax(12rem, 1fr))
clamp(
65ch
@media (pointer: coarse)
min-height: 44px
@@hint
`.stats { display: grid; gap: 0.75rem; grid-template-columns: repeat(auto-fit, minmax(12rem, 1fr)); }`, `h1 { font-size: clamp(1.5rem, 1.2rem + 1.5vw, 2.5rem); }`, `@media (min-width: 48rem) { .layout { grid-template-columns: 16rem 1fr; } nav { display: flex; flex-direction: column; } }`.
@@solution
<!doctype html>
<html lang="en">
<head>
  <meta charset="utf-8">
  <meta name="viewport" content="width=device-width, initial-scale=1">
  <title>Dashboard</title>
  <style>
    body { margin: 0; font-family: system-ui, sans-serif; background: #f1f5f9; color: #0f172a; }
    .layout { display: grid; gap: 1rem; padding: 1rem; }
    nav, main { background: #fff; border-radius: 12px; padding: 1rem; }
    nav { display: flex; flex-wrap: wrap; gap: 0.5rem; }
    button { font: inherit; padding: 0.4rem 0.9rem; border-radius: 8px; border: 1px solid #cbd5e1; background: #fff; }
    h1 { font-size: clamp(1.5rem, 1.2rem + 1.5vw, 2.5rem); margin-top: 0; }
    p { max-width: 65ch; line-height: 1.6; }
    .stats { display: grid; gap: 0.75rem; grid-template-columns: repeat(auto-fit, minmax(12rem, 1fr)); }
    .stat { background: #eef2ff; border-radius: 10px; padding: 1rem; }
    .stat strong { display: block; font-size: clamp(1.25rem, 1rem + 1vw, 2rem); }
    @media (min-width: 48rem) {
      .layout { grid-template-columns: 16rem 1fr; min-height: 100vh; box-sizing: border-box; }
      nav { flex-direction: column; align-items: stretch; }
    }
    @media (pointer: coarse) {
      button { min-height: 44px; }
    }
  </style>
</head>
<body>
  <div class="layout">
    <nav><button>Overview</button> <button>Reports</button> <button>Settings</button></nav>
    <main>
      <h1>Overview</h1>
      <p>Your projects at a glance. Resize the window to see the layout adapt.</p>
      <div class="stats">
        <div class="stat"><strong>12</strong> open tasks</div>
        <div class="stat"><strong>3</strong> reviews</div>
        <div class="stat"><strong>98%</strong> uptime</div>
        <div class="stat"><strong>4</strong> deploys today</div>
      </div>
    </main>
  </div>
</body>
</html>

@@exercise A card that adapts to its slot
Show the same profile card in a narrow sidebar and a wide main area. Make each slot a container (`container-type: inline-size`), stack the card's avatar above its text by default, and switch to a side-by-side grid with an `@container (min-width: 28rem)` query. Size the avatar with container units (`cqi`) clamped between 48 and 96 px.
@@starter
<!doctype html>
<html lang="en">
<head>
  <meta charset="utf-8">
  <title>Container queries</title>
  <style>
    body { font-family: system-ui, sans-serif; margin: 0; padding: 1rem; display: grid; grid-template-columns: 14rem 1fr; gap: 1rem; background: #f8fafc; }
    .slot { background: #fff; border-radius: 12px; padding: 1rem; }
    .card { display: grid; gap: 0.75rem; }
    .avatar { border-radius: 50%; background: linear-gradient(135deg, #818cf8, #ec4899); aspect-ratio: 1; }
  </style>
</head>
<body>
  <aside class="slot">
    <div class="card"><div class="avatar"></div><div><h2>Aiko Sato</h2><p>Frontend engineer, Osaka</p></div></div>
  </aside>
  <main class="slot">
    <div class="card"><div class="avatar"></div><div><h2>Aiko Sato</h2><p>Frontend engineer, Osaka</p></div></div>
  </main>
</body>
</html>
@@requires
container-type: inline-size
@container (min-width: 28rem)
cqi
clamp(48px
@@hint
`.slot { container-type: inline-size; }`, `.avatar { width: clamp(48px, 20cqi, 96px); }`, and `@container (min-width: 28rem) { .card { grid-template-columns: auto 1fr; align-items: center; } }`.
@@solution
<!doctype html>
<html lang="en">
<head>
  <meta charset="utf-8">
  <title>Container queries</title>
  <style>
    body { font-family: system-ui, sans-serif; margin: 0; padding: 1rem; display: grid; grid-template-columns: 14rem 1fr; gap: 1rem; background: #f8fafc; }
    .slot { background: #fff; border-radius: 12px; padding: 1rem; container-type: inline-size; }
    .card { display: grid; gap: 0.75rem; }
    .card h2 { margin: 0; font-size: 1.1rem; }
    .card p { margin: 0.25rem 0 0; color: #475569; }
    .avatar { border-radius: 50%; background: linear-gradient(135deg, #818cf8, #ec4899); aspect-ratio: 1; width: clamp(48px, 20cqi, 96px); }
    @container (min-width: 28rem) {
      .card { grid-template-columns: auto 1fr; align-items: center; }
      .card h2 { font-size: 1.4rem; }
    }
  </style>
</head>
<body>
  <aside class="slot">
    <div class="card"><div class="avatar"></div><div><h2>Aiko Sato</h2><p>Frontend engineer, Osaka</p></div></div>
  </aside>
  <main class="slot">
    <div class="card"><div class="avatar"></div><div><h2>Aiko Sato</h2><p>Frontend engineer, Osaka</p></div></div>
  </main>
</body>
</html>
