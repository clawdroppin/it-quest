@@lesson h2-layout Layout with Flexbox and Grid
@@text
## Flexbox: one direction

`display: flex` lines up the children in a **row** (or a column), and controls their spacing and alignment. Perfect for navigation bars, toolbars, and anything in a line.

```html
<style>
  body { font-family: system-ui, sans-serif; margin: 0; }
  .nav {
    display: flex;
    align-items: center;              /* vertical alignment in a row */
    gap: 1rem;                        /* space between the items */
    padding: 0.75rem 1rem;
    background: #1e293b;
  }
  .nav a { color: #e2e8f0; text-decoration: none; }
  .nav .logo { font-weight: bold; color: #38bdf8; }
  .nav .spacer { flex: 1; }           /* takes all the free space */
</style>
<nav class="nav">
  <a class="logo" href="#">IT Portal</a>
  <a href="#">Tickets</a>
  <a href="#">Wiki</a>
  <span class="spacer"></span>
  <a href="#">Log out</a>
</nav>
```

The main flex properties:

| On the container | Does |
|---|---|
| `flex-direction: row / column` | the main direction |
| `justify-content: flex-start / center / space-between / flex-end` | spacing along the main direction |
| `align-items: stretch / center / flex-start` | alignment across it |
| `gap: 1rem` | space between the items |
| `flex-wrap: wrap` | move items to a new line when they don't fit |

| On an item | Does |
|---|---|
| `flex: 1` | grow to fill the free space (`flex: 2` gets twice as much) |
| `flex: 0 0 200px` | exactly 200px, never grow or shrink |

Centring anything, the old CSS nightmare, is now:

```html
<style>
  .center { display: flex; justify-content: center; align-items: center; height: 120px; background: #f1f5f9; }
</style>
<div class="center"><strong>Perfectly centred</strong></div>
```

## Grid: two directions

`display: grid` places items in **rows and columns**: page layouts, dashboards, card galleries.

```html
<style>
  .dashboard {
    display: grid;
    grid-template-columns: repeat(3, 1fr);   /* three equal columns */
    gap: 12px;
    font-family: system-ui, sans-serif;
  }
  .tile { background: #e0f2fe; padding: 1rem; border-radius: 8px; }
  .wide { grid-column: span 2; }             /* this tile takes two columns */
</style>
<div class="dashboard">
  <div class="tile wide">Uptime: 99.98%</div>
  <div class="tile">Tickets: 12</div>
  <div class="tile">CPU: 41%</div>
  <div class="tile">Disk: 73%</div>
  <div class="tile">Backups: OK</div>
</div>
```

`fr` means "a fraction of the free space". `grid-template-columns: 200px 1fr` gives a fixed sidebar and a flexible main area.

The famous responsive gallery, with no media queries at all:

```html
<style>
  .cards { display: grid; grid-template-columns: repeat(auto-fill, minmax(160px, 1fr)); gap: 10px; }
  .cards div { background: #fef3c7; padding: 1rem; border-radius: 6px; font-family: sans-serif; }
</style>
<div class="cards"><div>web01</div><div>web02</div><div>db01</div><div>mail01</div><div>vpn01</div></div>
```

As many columns of at least 160px as fit; resize the window and they rearrange.

## Named areas for a page layout

```html
<style>
  .page {
    display: grid;
    grid-template-columns: 180px 1fr;
    grid-template-rows: auto 1fr auto;
    grid-template-areas:
      "header header"
      "side   main"
      "footer footer";
    min-height: 260px;
    gap: 8px;
    font-family: system-ui, sans-serif;
  }
  header { grid-area: header; background: #c7d2fe; }
  aside  { grid-area: side;   background: #e0e7ff; }
  main   { grid-area: main;   background: #eef2ff; }
  footer { grid-area: footer; background: #c7d2fe; }
  .page > * { padding: 0.5rem; }
</style>
<div class="page">
  <header>Header</header>
  <aside>Menu</aside>
  <main>Content</main>
  <footer>Footer</footer>
</div>
```

Rule of thumb: **grid for the page and 2-D layouts, flexbox for the things inside**.

## position

- `position: relative`: the element stays in the flow; it becomes the reference for absolutely positioned children.
- `position: absolute; top: 0; right: 0`: taken out of the flow, placed relative to the nearest positioned ancestor (badges, close buttons).
- `position: sticky; top: 0`: scrolls normally, then sticks (table headers, nav bars).
- `position: fixed`: stays put while the page scrolls (chat buttons, cookie bars).

@@quiz
Which property puts space between flex items without margins?
* `gap`
- `padding`
- `space`
- `justify-items`
? `gap` works for both flexbox and grid.

@@quiz
`grid-template-columns: 1fr 2fr` makes...
* Two columns, the second twice as wide as the first
- Two rows
- Two columns of 1px and 2px
- One column
? `fr` shares out the free space by ratio.

@@quiz
In a flex row, which property centres the items **horizontally**?
* `justify-content: center`
- `align-items: center`
- `text-align: center`
- `margin: center`
? `justify-content` works along the main direction (the row); `align-items` across it.

@@quiz
Grid or flexbox for a navigation bar of links in one line?
* Flexbox: it's a one-direction layout
- Grid, always
- Neither: use a table
- position: absolute
? Use grid when you need rows *and* columns.

@@flashcards
`display: flex` :: lay out children in a row (or column)
`justify-content` :: spacing along the main direction
`align-items` :: alignment across the main direction
`flex: 1` :: grow to fill free space
`display: grid` :: rows and columns
`repeat(3, 1fr)` :: three equal columns
`minmax(160px, 1fr)` :: at least 160px, otherwise share the space
`position: sticky` :: scroll, then stick at an edge

@@exercise Header bar
Style the `<header>` as a flex row: the logo on the left, the links in the middle area, and the user name pushed to the far right (use `margin-left: auto` or a spacer with `flex: 1`). Items should be vertically centred, with a `gap`.
@@starter
<!doctype html>
<html lang="en">
<head>
  <meta charset="utf-8">
  <title>Header</title>
  <style>
    body { margin: 0; font-family: system-ui, sans-serif; }
    header { background: #0f172a; color: white; padding: 0.75rem 1rem; }
    header a { color: #cbd5e1; }
  </style>
</head>
<body>
  <header>
    <strong class="logo">HelpDesk</strong>
    <a href="#">Open</a>
    <a href="#">Mine</a>
    <a href="#">Reports</a>
    <span class="user">aiko.tanaka</span>
  </header>
</body>
</html>
@@requires
display: flex
align-items: center
gap:
.user
@@hint
`header { display: flex; align-items: center; gap: 1rem; ... }` and `.user { margin-left: auto; }`: an auto margin swallows all the free space.
@@solution
<!doctype html>
<html lang="en">
<head>
  <meta charset="utf-8">
  <title>Header</title>
  <style>
    body { margin: 0; font-family: system-ui, sans-serif; }
    header { display: flex; align-items: center; gap: 1rem; background: #0f172a; color: white; padding: 0.75rem 1rem; }
    header a { color: #cbd5e1; }
    .user { margin-left: auto; }
  </style>
</head>
<body>
  <header>
    <strong class="logo">HelpDesk</strong>
    <a href="#">Open</a>
    <a href="#">Mine</a>
    <a href="#">Reports</a>
    <span class="user">aiko.tanaka</span>
  </header>
</body>
</html>

@@exercise Monitoring dashboard
Lay out the tiles as a grid with 4 equal columns and a `gap`. The `alerts` tile must span **2 columns** and **2 rows**.
@@starter
<!doctype html>
<html lang="en">
<head>
  <meta charset="utf-8">
  <title>Dashboard</title>
  <style>
    body { font-family: system-ui, sans-serif; margin: 1rem; }
    .tile { background: #ecfeff; border: 1px solid #a5f3fc; border-radius: 8px; padding: 1rem; }
    .alerts { background: #fff1f2; border-color: #fecdd3; }
  </style>
</head>
<body>
  <div class="grid">
    <div class="tile alerts">3 active alerts</div>
    <div class="tile">CPU 41%</div>
    <div class="tile">RAM 63%</div>
    <div class="tile">Disk 73%</div>
    <div class="tile">Net 120 Mbps</div>
    <div class="tile">Users 214</div>
    <div class="tile">Tickets 12</div>
  </div>
</body>
</html>
@@requires
display: grid
grid-template-columns: repeat(4, 1fr)
gap:
grid-column: span 2
grid-row: span 2
@@hint
`.grid { display: grid; grid-template-columns: repeat(4, 1fr); gap: 10px; }` and `.alerts { grid-column: span 2; grid-row: span 2; }`.
@@solution
<!doctype html>
<html lang="en">
<head>
  <meta charset="utf-8">
  <title>Dashboard</title>
  <style>
    body { font-family: system-ui, sans-serif; margin: 1rem; }
    .grid { display: grid; grid-template-columns: repeat(4, 1fr); gap: 10px; }
    .tile { background: #ecfeff; border: 1px solid #a5f3fc; border-radius: 8px; padding: 1rem; }
    .alerts { background: #fff1f2; border-color: #fecdd3; grid-column: span 2; grid-row: span 2; }
  </style>
</head>
<body>
  <div class="grid">
    <div class="tile alerts">3 active alerts</div>
    <div class="tile">CPU 41%</div>
    <div class="tile">RAM 63%</div>
    <div class="tile">Disk 73%</div>
    <div class="tile">Net 120 Mbps</div>
    <div class="tile">Users 214</div>
    <div class="tile">Tickets 12</div>
  </div>
</body>
</html>
