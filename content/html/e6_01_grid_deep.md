@@lesson e6-grid Grid in Depth: Areas, auto-fit, minmax and Subgrid
@@text
Module 2 used grid for simple columns. Grid can do far more: name regions of a page, build card galleries that fit any width without a single media query, and align nested content across cards.

## Tracks: fr, minmax and repeat

```css
grid-template-columns: 200px 1fr 2fr;              /* fixed, then shares of the rest */
grid-template-columns: repeat(4, 1fr);             /* four equal columns */
grid-template-columns: minmax(12rem, 1fr) 3fr;     /* a sidebar at least 12rem wide */
grid-template-rows: auto 1fr auto;                 /* header, stretchy main, footer */
```

`1fr` is really `minmax(auto, 1fr)`: a column won't shrink below its content's minimum width. Long words or wide images can therefore blow up a "1fr" column. `minmax(0, 1fr)` lets it shrink.

## The auto-fit gallery

The most useful line of CSS grid:

```html
<style>
  .gallery { display: grid; gap: 1rem; grid-template-columns: repeat(auto-fit, minmax(min(12rem, 100%), 1fr)); }
  .gallery div { background: #e0f2fe; padding: 1rem; border-radius: 8px; }
</style>
<div class="gallery"><div>VPN</div><div>Email</div><div>Printers</div><div>Wi-Fi</div><div>Accounts</div></div>
```

"As many columns as fit, each at least 12rem, sharing the leftover space". It goes from 1 column on a phone to 5 on a wide monitor with no media queries. (`min(12rem, 100%)` prevents overflow on screens narrower than 12rem.)

`auto-fill` vs `auto-fit`: both create as many tracks as fit; with few items, `auto-fill` keeps the empty tracks (items stay narrow), `auto-fit` collapses them (items stretch to fill the row).

## Named areas

Draw the layout as ASCII art:

```html
<style>
  .page { display: grid; gap: 0.5rem; min-height: 16rem;
          grid-template-columns: 10rem 1fr;
          grid-template-rows: auto 1fr auto;
          grid-template-areas: "head head" "nav main" "foot foot"; }
  .page > * { background: #f1f5f9; padding: 0.5rem; }
  header { grid-area: head; } nav { grid-area: nav; } main { grid-area: main; } footer { grid-area: foot; }
  @media (max-width: 40rem) {
    .page { grid-template-columns: 1fr; grid-template-areas: "head" "nav" "main" "foot"; }
  }
</style>
<div class="page"><header>Header</header><nav>Nav</nav><main>Main</main><footer>Footer</footer></div>
```

Rearranging for mobile is just a new picture. A `.` in the template leaves a cell empty.

## Placing items by line

Grid lines are numbered from 1 (and from -1 at the end):

```css
.wide  { grid-column: 1 / -1; }      /* span the whole row */
.tall  { grid-row: span 2; }         /* two rows high */
.hero  { grid-column: 2 / span 3; }
```

Items can overlap when placed in the same cells; `z-index` decides which is on top. That's a clean way to put text over an image without absolute positioning.

## Implicit grid and dense packing

Items that don't fit the explicit tracks create **implicit** rows sized by `grid-auto-rows` (e.g. `grid-auto-rows: 8rem`). `grid-auto-flow: dense` backfills gaps left by large items, which is handy for dashboards with tiles of different sizes (but changes visual order versus DOM order, which can confuse keyboard users).

## Alignment

| Property | Aligns |
|---|---|
| `justify-items` / `align-items` | items inside their cells (default `stretch`) |
| `justify-content` / `align-content` | the whole grid inside the container (when tracks don't fill it) |
| `justify-self` / `align-self` | one item |
| `place-items: center` | both axes at once |

## Subgrid

Cards in a row often have titles of different lengths, so their buttons don't line up. With **subgrid**, a card's rows join the parent grid's rows, so the same parts of every card align:

```css
.cards { display: grid; grid-template-columns: repeat(3, 1fr); grid-template-rows: auto; gap: 1rem; }
.card  { display: grid; grid-row: span 3; grid-template-rows: subgrid; }   /* title, text, button */
```

Supported in all current browsers.

## Grid or flexbox?

- **Flexbox**: one dimension, content decides the sizes (toolbars, tags, button groups, a row of items that wrap).
- **Grid**: two dimensions, or when the **layout** decides the sizes (page layouts, galleries, forms with aligned labels, dashboards).

They nest freely: a grid page whose header is a flex row.

@@quiz
What does `repeat(auto-fit, minmax(12rem, 1fr))` produce?
* As many columns as fit at 12rem minimum, stretched to fill the row
- Exactly 12 columns
- One column of 12rem
- Columns of exactly 12rem
? The gallery adapts to any width without media queries.

@@quiz
A long URL makes a `1fr` column wider than the others. Why?
- fr units are broken
* `1fr` means `minmax(auto, 1fr)`, so it won't shrink below its content's minimum width
- URLs are always wide
- Grid ignores fr for text
? Use `minmax(0, 1fr)` or `overflow-wrap: anywhere`.

@@quiz
What does `grid-column: 1 / -1` do?
- Hides the item
* Spans from the first to the last grid line: the full row
- Places it in column -1
- Reverses the order
? Negative numbers count from the end.

@@quiz
What problem does subgrid solve?
- Grids inside tables
* Aligning the inner parts of sibling cards to shared rows of the parent grid
- Older browser support
- Nested flexbox
? Titles, texts and buttons line up across cards.

@@flashcards
`fr` :: a share of the free space
`minmax(a, b)` :: track between a and b
`repeat(auto-fit, minmax(12rem, 1fr))` :: responsive gallery
auto-fill vs auto-fit :: keep empty tracks vs collapse them
`grid-template-areas` :: name regions as ASCII art
`grid-area: main` :: place in a named area
`grid-column: 1 / -1` :: full width
`span 2` :: two tracks
`grid-auto-rows` :: size of implicit rows
`grid-auto-flow: dense` :: backfill gaps
`subgrid` :: inherit the parent's tracks
`minmax(0, 1fr)` :: a column that can shrink

@@exercise A responsive dashboard with areas
Lay out the dashboard with named areas: on wide screens `"head head head" "nav kpis kpis" "nav chart alerts"` with columns `12rem 1fr 18rem`; under `50rem` everything is one column in the order head, kpis, alerts, chart, nav. The KPI tiles inside `.kpis` use the auto-fit pattern with a `9rem` minimum.
@@starter
<!doctype html>
<html lang="en">
<head><meta charset="utf-8"><title>Dashboard</title>
<style>
  body { margin: 0; font-family: system-ui; }
  .dash > * { background: #f1f5f9; padding: 1rem; border-radius: 8px; }
  .kpi { background: white; padding: 0.75rem; border-radius: 6px; }
</style>
</head>
<body>
  <div class="dash">
    <header>IT Operations</header>
    <nav>Menu</nav>
    <section class="kpis">
      <div class="kpi">Open tickets: 5</div><div class="kpi">SLA met: 62%</div><div class="kpi">Servers up: 11/12</div><div class="kpi">Patches due: 3</div>
    </section>
    <section class="chart">Chart</section>
    <aside class="alerts">Alerts</aside>
  </div>
</body>
</html>
@@requires
grid-template-areas: "head head head" "nav kpis kpis" "nav chart alerts"
grid-template-columns: 12rem 1fr 18rem
@media (max-width: 50rem)
"head" "kpis" "alerts" "chart" "nav"
repeat(auto-fit, minmax(9rem, 1fr))
grid-area: kpis
@@hint
`.dash { display: grid; gap: 1rem; padding: 1rem; grid-template-columns: 12rem 1fr 18rem; grid-template-areas: "head head head" "nav kpis kpis" "nav chart alerts"; }`, then `header { grid-area: head; }` etc. In the media query: `grid-template-columns: 1fr; grid-template-areas: "head" "kpis" "alerts" "chart" "nav";`.
@@solution
<!doctype html>
<html lang="en">
<head><meta charset="utf-8"><title>Dashboard</title>
<style>
  body { margin: 0; font-family: system-ui; }
  .dash { display: grid; gap: 1rem; padding: 1rem;
          grid-template-columns: 12rem 1fr 18rem;
          grid-template-areas: "head head head" "nav kpis kpis" "nav chart alerts"; }
  .dash > * { background: #f1f5f9; padding: 1rem; border-radius: 8px; }
  header { grid-area: head; }
  nav { grid-area: nav; }
  .kpis { grid-area: kpis; display: grid; gap: 0.75rem; grid-template-columns: repeat(auto-fit, minmax(9rem, 1fr)); }
  .chart { grid-area: chart; min-height: 12rem; }
  .alerts { grid-area: alerts; }
  .kpi { background: white; padding: 0.75rem; border-radius: 6px; }
  @media (max-width: 50rem) {
    .dash { grid-template-columns: 1fr; grid-template-areas: "head" "kpis" "alerts" "chart" "nav"; }
  }
</style>
</head>
<body>
  <div class="dash">
    <header>IT Operations</header>
    <nav>Menu</nav>
    <section class="kpis">
      <div class="kpi">Open tickets: 5</div><div class="kpi">SLA met: 62%</div><div class="kpi">Servers up: 11/12</div><div class="kpi">Patches due: 3</div>
    </section>
    <section class="chart">Chart</section>
    <aside class="alerts">Alerts</aside>
  </div>
</body>
</html>

@@exercise Cards that line up with subgrid
The three service cards have titles and texts of different lengths, so the "Open" buttons don't line up. Make `.cards` a 3-column grid, and each `.card` span 3 rows with `grid-template-rows: subgrid`, so titles, texts and buttons share rows across all cards. Push each button to the bottom of its row with `align-self: end`.
@@starter
<!doctype html>
<html lang="en">
<head><meta charset="utf-8"><title>Services</title>
<style>
  body { font-family: system-ui; }
  .card { border: 1px solid #cbd5e1; border-radius: 10px; padding: 1rem; }
  .card h2 { margin: 0; font-size: 1.1rem; }
</style>
</head>
<body>
  <div class="cards">
    <article class="card"><h2>Email</h2><p>Exchange Online for everyone.</p><a href="#">Open</a></article>
    <article class="card"><h2>Virtual Private Network for remote and travelling staff</h2><p>Connect securely from home.</p><a href="#">Open</a></article>
    <article class="card"><h2>Printing</h2><p>Follow-me printing on every floor, with badge release, colour quotas per department and scan-to-email.</p><a href="#">Open</a></article>
  </div>
</body>
</html>
@@requires
grid-template-columns: repeat(3, 1fr)
grid-row: span 3
grid-template-rows: subgrid
align-self: end
@@hint
`.cards { display: grid; grid-template-columns: repeat(3, 1fr); gap: 1rem; } .card { display: grid; grid-row: span 3; grid-template-rows: subgrid; gap: 0.5rem; } .card a { align-self: end; }`
@@solution
<!doctype html>
<html lang="en">
<head><meta charset="utf-8"><title>Services</title>
<style>
  body { font-family: system-ui; }
  .cards { display: grid; grid-template-columns: repeat(3, 1fr); gap: 1rem; }
  .card { display: grid; grid-row: span 3; grid-template-rows: subgrid; gap: 0.5rem;
          border: 1px solid #cbd5e1; border-radius: 10px; padding: 1rem; }
  .card h2 { margin: 0; font-size: 1.1rem; }
  .card p { margin: 0; }
  .card a { align-self: end; }
</style>
</head>
<body>
  <div class="cards">
    <article class="card"><h2>Email</h2><p>Exchange Online for everyone.</p><a href="#">Open</a></article>
    <article class="card"><h2>Virtual Private Network for remote and travelling staff</h2><p>Connect securely from home.</p><a href="#">Open</a></article>
    <article class="card"><h2>Printing</h2><p>Follow-me printing on every floor, with badge release, colour quotas per department and scan-to-email.</p><a href="#">Open</a></article>
  </div>
</body>
</html>

@@exercise A photo mosaic with spans
Build a mosaic: a grid with `repeat(auto-fill, minmax(8rem, 1fr))` columns, `grid-auto-rows: 8rem` and `grid-auto-flow: dense`. Tiles with class `wide` span 2 columns, `tall` span 2 rows, and `big` span both. Make the caption overlap its tile at the bottom by placing the image and caption in the **same** grid cell inside each tile (`grid-area: 1 / 1`), with the caption `align-self: end`.
@@starter
<!doctype html>
<html lang="en">
<head><meta charset="utf-8"><title>Mosaic</title>
<style>
  .tile { background: #94a3b8; border-radius: 6px; overflow: hidden; }
  .tile img { width: 100%; height: 100%; object-fit: cover; }
  .tile span { background: rgb(0 0 0 / 0.6); color: white; padding: 0.25rem 0.5rem; }
</style>
</head>
<body>
  <div class="mosaic">
    <figure class="tile big"><img src="https://picsum.photos/seed/a/400" alt=""><span>Tokyo office</span></figure>
    <figure class="tile"><img src="https://picsum.photos/seed/b/200" alt=""><span>Server room</span></figure>
    <figure class="tile tall"><img src="https://picsum.photos/seed/c/200/400" alt=""><span>Osaka</span></figure>
    <figure class="tile wide"><img src="https://picsum.photos/seed/d/400/200" alt=""><span>Team day</span></figure>
    <figure class="tile"><img src="https://picsum.photos/seed/e/200" alt=""><span>Help desk</span></figure>
    <figure class="tile"><img src="https://picsum.photos/seed/f/200" alt=""><span>Fukuoka</span></figure>
  </div>
</body>
</html>
@@requires
repeat(auto-fill, minmax(8rem, 1fr))
grid-auto-rows: 8rem
grid-auto-flow: dense
grid-column: span 2
grid-row: span 2
grid-area: 1 / 1
@@hint
`.mosaic { display: grid; gap: 0.5rem; grid-template-columns: repeat(auto-fill, minmax(8rem, 1fr)); grid-auto-rows: 8rem; grid-auto-flow: dense; } .wide, .big { grid-column: span 2; } .tall, .big { grid-row: span 2; } .tile { display: grid; margin: 0; } .tile img, .tile span { grid-area: 1 / 1; } .tile span { align-self: end; }`
@@solution
<!doctype html>
<html lang="en">
<head><meta charset="utf-8"><title>Mosaic</title>
<style>
  .mosaic { display: grid; gap: 0.5rem; grid-template-columns: repeat(auto-fill, minmax(8rem, 1fr));
            grid-auto-rows: 8rem; grid-auto-flow: dense; }
  .wide, .big { grid-column: span 2; }
  .tall, .big { grid-row: span 2; }
  .tile { display: grid; margin: 0; background: #94a3b8; border-radius: 6px; overflow: hidden; }
  .tile img, .tile span { grid-area: 1 / 1; }
  .tile img { width: 100%; height: 100%; object-fit: cover; }
  .tile span { align-self: end; background: rgb(0 0 0 / 0.6); color: white; padding: 0.25rem 0.5rem; }
</style>
</head>
<body>
  <div class="mosaic">
    <figure class="tile big"><img src="https://picsum.photos/seed/a/400" alt=""><span>Tokyo office</span></figure>
    <figure class="tile"><img src="https://picsum.photos/seed/b/200" alt=""><span>Server room</span></figure>
    <figure class="tile tall"><img src="https://picsum.photos/seed/c/200/400" alt=""><span>Osaka</span></figure>
    <figure class="tile wide"><img src="https://picsum.photos/seed/d/400/200" alt=""><span>Team day</span></figure>
    <figure class="tile"><img src="https://picsum.photos/seed/e/200" alt=""><span>Help desk</span></figure>
    <figure class="tile"><img src="https://picsum.photos/seed/f/200" alt=""><span>Fukuoka</span></figure>
  </div>
</body>
</html>
