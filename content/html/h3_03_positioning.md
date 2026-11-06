@@lesson h3-position Positioning, Stacking, Overflow and Display
@@text
Flexbox and grid handle most layouts. For everything that **sits on top of** other things (sticky headers, badges, tooltips, overlays) you need positioning, and with it, an understanding of how browsers decide what's in front.

## The position property

| Value | Behaviour |
|---|---|
| `static` | the default: normal flow; `top`/`left` are ignored |
| `relative` | normal flow, but can be nudged with `top`/`left`; **becomes the reference box** for absolute children |
| `absolute` | taken out of the flow; placed relative to the nearest **positioned** ancestor (any position except static) |
| `fixed` | relative to the viewport; stays put when scrolling |
| `sticky` | normal flow until it reaches its `top` offset while scrolling, then sticks within its parent |

The classic pattern: parent `position: relative`, child `position: absolute` with offsets.

```html
<style>
  .avatar { position: relative; display: inline-block; width: 48px; height: 48px; border-radius: 50%; background: #cbd5e1; }
  .badge { position: absolute; top: -4px; right: -4px; min-width: 1.25rem; padding: 0 0.3rem; border-radius: 999px;
           background: #e11d48; color: white; font: bold 0.75rem/1.25rem system-ui; text-align: center; }
</style>
<span class="avatar"><span class="badge">3</span></span>
```

`inset: 0` is shorthand for `top: 0; right: 0; bottom: 0; left: 0`: an absolute child that covers its parent completely (overlays, clickable cards).

## Sticky headers

```css
thead th { position: sticky; top: 0; background: white; }
```

Sticky needs a `top` (or bottom) value, and it only sticks **inside its parent**: if the parent is short, it scrolls away with it. An ancestor with `overflow: hidden` breaks sticky, a common "why doesn't sticky work" cause.

## Stacking and z-index

Positioned elements can overlap. `z-index` (on positioned or flex/grid items) decides the order: higher is in front. But z-index values are only compared **within the same stacking context**. A new stacking context is created by, among others: a positioned element with a z-index, `opacity` below 1, `transform`, `filter`, `isolation: isolate`, and `position: fixed/sticky`.

That's the "z-index: 9999 doesn't work" bug: the element is inside a parent whose stacking context sits below the other element; no z-index inside can escape it. Fixes: move the element in the DOM (modals as direct children of `body`), or use the `<dialog>` element / popover API, which render in the browser's **top layer** above everything.

Keep z-index values in a small, named scale (`--z-dropdown: 10; --z-header: 20; --z-modal: 100;`) instead of random 9999s.

## Overflow

When content is bigger than its box:

- `overflow: visible` (default): it spills out.
- `hidden`: clipped. (Careful: also clips focus rings and breaks sticky.)
- `auto`: scrollbars only when needed. Use this for scrollable panels and wide tables: wrap the table in `<div style="overflow-x: auto">` so the page itself never scrolls sideways.
- `clip`: like hidden, but doesn't create a scroll container.

`overscroll-behavior: contain` stops a scrolled panel from scrolling the page behind it.

## Display types you'll meet

| Value | Does |
|---|---|
| `block` | full width, starts on a new line (div, p, h1) |
| `inline` | flows in text; width/height and vertical margins ignored (span, a) |
| `inline-block` | flows in text but accepts width/height (badges, buttons) |
| `flex` / `inline-flex` | flex container |
| `grid` / `inline-grid` | grid container |
| `none` | removed from layout and from the accessibility tree |
| `contents` | the element's box disappears; its children act as children of its parent |

To hide something **visually** but keep it for screen readers, use a `.visually-hidden` class (position absolute, 1px size, clip) instead of `display: none`.

## Centring, all the ways

```css
.parent { display: grid; place-items: center; }                 /* both axes */
.box { margin-inline: auto; width: fit-content; }               /* horizontally, in flow */
.overlay-child { position: absolute; inset: 0; margin: auto; width: 200px; height: 100px; }
```

## Aspect ratio

`aspect-ratio: 16 / 9` keeps a box's shape as its width changes: video embeds, cards, thumbnails. With `object-fit: cover`, images fill such boxes without distortion.

@@quiz
An absolutely positioned element is placed relative to what?
- Always the page
- Its previous sibling
* The nearest ancestor whose position isn't static (or the initial containing block)
- The viewport
? That's why parents get `position: relative`.

@@quiz
Your modal has `z-index: 9999` but appears behind the header. What's the likely cause?
- 9999 is too small
* The modal is inside a stacking context (e.g. a parent with transform or opacity) that sits below the header
- z-index only works on flex items
- Fixed elements ignore z-index
? Move it out of that context, or use `<dialog>` / popover (top layer).

@@quiz
Why doesn't `position: sticky` stick?
- It requires JavaScript
* Often a missing `top` value, or an ancestor with `overflow: hidden`
- Sticky only works in tables
- Sticky is deprecated
? Sticky also only works within its parent.

@@quiz
What's the right way to stop a wide table from making the whole page scroll sideways?
- `overflow: hidden` on body
* Wrap it in a container with `overflow-x: auto`
- Set `width: 100vw`
- Use smaller fonts
? The table scrolls on its own; the page stays put.

@@drill
Which `position` value keeps a header in view after scrolling past it?
= sticky
? Give it top: 0.

@@flashcards
`position: relative` :: nudge + reference box for absolute children
`position: absolute` :: out of flow, relative to positioned ancestor
`position: fixed` :: relative to the viewport
`position: sticky; top: 0` :: sticks while scrolling within its parent
`inset: 0` :: top/right/bottom/left 0
stacking context :: z-index is compared only within it
top layer :: `<dialog>` and popovers, above everything
`isolation: isolate` :: create a stacking context on purpose
`overflow-x: auto` :: scroll only this box
`display: contents` :: remove the box, keep the children
`.visually-hidden` :: hidden visually, kept for screen readers
`place-items: center` :: centre in a grid
`aspect-ratio: 16 / 9` :: keep a box's shape

@@exercise A notification badge
Put a red count badge on the top-right corner of the bell button: make the button the reference box (`position: relative`) and the badge `position: absolute` with `top: -6px; right: -6px`. The badge is a pill (`border-radius: 999px`). Also give the button a visually hidden text "Notifications" for screen readers, using a `.visually-hidden` class (position absolute, width and height 1px, overflow hidden, `clip: rect(0 0 0 0)`, `white-space: nowrap`).
@@starter
<!doctype html>
<html lang="en">
<head><meta charset="utf-8"><title>Badge</title>
<style>
  .bell { font-size: 1.5rem; padding: 0.5rem; border: 1px solid #cbd5e1; border-radius: 8px; background: white; }
  .badge { background: #e11d48; color: white; font: bold 0.75rem system-ui; padding: 0.1rem 0.4rem; }
</style>
</head>
<body>
  <button class="bell">🔔 <span class="badge">4</span></button>
</body>
</html>
@@requires
position: relative
position: absolute
top: -6px
right: -6px
border-radius: 999px
.visually-hidden
clip: rect(0 0 0 0)
@@hint
`.bell { position: relative; } .badge { position: absolute; top: -6px; right: -6px; border-radius: 999px; }`, and `<span class="visually-hidden">Notifications</span>` inside the button. Hide the emoji itself from screen readers with `aria-hidden="true"` on a span around it.
@@solution
<!doctype html>
<html lang="en">
<head><meta charset="utf-8"><title>Badge</title>
<style>
  .bell { position: relative; font-size: 1.5rem; padding: 0.5rem; border: 1px solid #cbd5e1; border-radius: 8px; background: white; }
  .badge { position: absolute; top: -6px; right: -6px; border-radius: 999px;
           background: #e11d48; color: white; font: bold 0.75rem system-ui; padding: 0.1rem 0.4rem; }
  .visually-hidden { position: absolute; width: 1px; height: 1px; overflow: hidden; clip: rect(0 0 0 0); white-space: nowrap; }
</style>
</head>
<body>
  <button class="bell"><span aria-hidden="true">🔔</span><span class="visually-hidden">Notifications</span> <span class="badge">4</span></button>
</body>
</html>

@@exercise A sticky header and a scrollable table
Make the page header stick to the top while scrolling (`position: sticky; top: 0` with a background and `z-index: 10`), and make the wide inventory table scroll **inside** its own wrapper (`overflow-x: auto`) with sticky column headers inside the wrapper (`thead th { position: sticky; top: 0; }`) and a fixed height of `12rem` (`overflow-y: auto` too).
@@starter
<!doctype html>
<html lang="en">
<head><meta charset="utf-8"><title>Inventory</title>
<style>
  header { padding: 1rem; background: #1e293b; color: white; }
  table { border-collapse: collapse; min-width: 900px; }
  td, th { padding: 0.5rem 1rem; border-bottom: 1px solid #e2e8f0; text-align: left; background: white; }
</style>
</head>
<body>
  <header>IT Inventory</header>
  <table>
    <thead><tr><th>Tag</th><th>Type</th><th>Model</th><th>OS</th><th>Owner</th><th>Purchased</th><th>Warranty</th></tr></thead>
    <tbody>
      <tr><td>LT-0042</td><td>laptop</td><td>ThinkPad T14</td><td>Windows 11</td><td>Ana Silva</td><td>2023-04-01</td><td>2026-03-31</td></tr>
      <tr><td>LT-0043</td><td>laptop</td><td>ThinkPad T14</td><td>Windows 11</td><td>Mia Lee</td><td>2023-04-01</td><td>2026-03-31</td></tr>
      <tr><td>LT-0050</td><td>laptop</td><td>MacBook Air</td><td>macOS 15</td><td>Ben Jones</td><td>2024-05-20</td><td>2027-05-19</td></tr>
      <tr><td>LT-0051</td><td>laptop</td><td>ThinkPad X1</td><td>Windows 11</td><td>Kenji Tanaka</td><td>2025-01-15</td><td>2028-01-14</td></tr>
      <tr><td>DT-0007</td><td>desktop</td><td>OptiPlex 7010</td><td>Windows 11</td><td>Leo Martin</td><td>2022-10-01</td><td>2025-09-30</td></tr>
      <tr><td>DT-0008</td><td>desktop</td><td>OptiPlex 7010</td><td>Windows 11</td><td>Emi Suzuki</td><td>2022-10-01</td><td>2025-09-30</td></tr>
      <tr><td>SV-0001</td><td>server</td><td>PowerEdge R650</td><td>Ubuntu 24.04</td><td></td><td>2023-06-01</td><td>2028-05-31</td></tr>
    </tbody>
  </table>
</body>
</html>
@@requires
position: sticky
top: 0
z-index: 10
overflow-x: auto
overflow-y: auto
thead th
@@hint
`header { position: sticky; top: 0; z-index: 10; }`, wrap the table in `<div class="table-wrap">` with `.table-wrap { overflow-x: auto; overflow-y: auto; height: 12rem; }` and `thead th { position: sticky; top: 0; }`.
@@solution
<!doctype html>
<html lang="en">
<head><meta charset="utf-8"><title>Inventory</title>
<style>
  header { position: sticky; top: 0; z-index: 10; padding: 1rem; background: #1e293b; color: white; }
  .table-wrap { overflow-x: auto; overflow-y: auto; height: 12rem; border: 1px solid #e2e8f0; }
  table { border-collapse: collapse; min-width: 900px; }
  td, th { padding: 0.5rem 1rem; border-bottom: 1px solid #e2e8f0; text-align: left; background: white; }
  thead th { position: sticky; top: 0; background: #f1f5f9; }
</style>
</head>
<body>
  <header>IT Inventory</header>
  <div class="table-wrap">
  <table>
    <thead><tr><th>Tag</th><th>Type</th><th>Model</th><th>OS</th><th>Owner</th><th>Purchased</th><th>Warranty</th></tr></thead>
    <tbody>
      <tr><td>LT-0042</td><td>laptop</td><td>ThinkPad T14</td><td>Windows 11</td><td>Ana Silva</td><td>2023-04-01</td><td>2026-03-31</td></tr>
      <tr><td>LT-0043</td><td>laptop</td><td>ThinkPad T14</td><td>Windows 11</td><td>Mia Lee</td><td>2023-04-01</td><td>2026-03-31</td></tr>
      <tr><td>LT-0050</td><td>laptop</td><td>MacBook Air</td><td>macOS 15</td><td>Ben Jones</td><td>2024-05-20</td><td>2027-05-19</td></tr>
      <tr><td>LT-0051</td><td>laptop</td><td>ThinkPad X1</td><td>Windows 11</td><td>Kenji Tanaka</td><td>2025-01-15</td><td>2028-01-14</td></tr>
      <tr><td>DT-0007</td><td>desktop</td><td>OptiPlex 7010</td><td>Windows 11</td><td>Leo Martin</td><td>2022-10-01</td><td>2025-09-30</td></tr>
      <tr><td>DT-0008</td><td>desktop</td><td>OptiPlex 7010</td><td>Windows 11</td><td>Emi Suzuki</td><td>2022-10-01</td><td>2025-09-30</td></tr>
      <tr><td>SV-0001</td><td>server</td><td>PowerEdge R650</td><td>Ubuntu 24.04</td><td></td><td>2023-06-01</td><td>2028-05-31</td></tr>
    </tbody>
  </table>
  </div>
</body>
</html>

@@exercise A clickable card with a cover image
Make the whole card clickable while keeping a real link: the card is `position: relative`, the link's `::after` covers it (`position: absolute; inset: 0; content: "";`). The image keeps a `16 / 9` shape with `aspect-ratio` and fills it with `object-fit: cover`. Show a focus ring on the card when the link is keyboard-focused (`.card:has(a:focus-visible)`).
@@starter
<!doctype html>
<html lang="en">
<head><meta charset="utf-8"><title>Cards</title>
<style>
  .card { width: 18rem; border: 1px solid #e2e8f0; border-radius: 12px; overflow: hidden; font-family: system-ui; }
  .card img { width: 100%; display: block; }
  .card h2 { font-size: 1.1rem; margin: 0.75rem; }
</style>
</head>
<body>
  <article class="card">
    <img src="https://picsum.photos/seed/vpn/600/600" alt="">
    <h2><a href="/kb/vpn">Set up the VPN on a new laptop</a></h2>
  </article>
</body>
</html>
@@requires
position: relative
inset: 0
aspect-ratio: 16 / 9
object-fit: cover
.card:has(a:focus-visible)
@@hint
`.card { position: relative; } .card a::after { content: ""; position: absolute; inset: 0; } .card img { aspect-ratio: 16 / 9; object-fit: cover; } .card:has(a:focus-visible) { outline: 3px solid #2563eb; outline-offset: 2px; } .card a:focus-visible { outline: none; }`
@@solution
<!doctype html>
<html lang="en">
<head><meta charset="utf-8"><title>Cards</title>
<style>
  .card { position: relative; width: 18rem; border: 1px solid #e2e8f0; border-radius: 12px; overflow: hidden; font-family: system-ui; }
  .card img { width: 100%; display: block; aspect-ratio: 16 / 9; object-fit: cover; }
  .card h2 { font-size: 1.1rem; margin: 0.75rem; }
  .card a { color: inherit; text-decoration: none; }
  .card a::after { content: ""; position: absolute; inset: 0; }
  .card:hover { box-shadow: 0 4px 12px rgb(0 0 0 / 0.15); }
  .card:has(a:focus-visible) { outline: 3px solid #2563eb; outline-offset: 2px; }
  .card a:focus-visible { outline: none; }
</style>
</head>
<body>
  <article class="card">
    <img src="https://picsum.photos/seed/vpn/600/600" alt="">
    <h2><a href="/kb/vpn">Set up the VPN on a new laptop</a></h2>
  </article>
</body>
</html>
