@@lesson e6-project Project: An Operations Dashboard Layout
@@text
Time to combine the module: tokens and layers for structure, grid areas for the page, auto-fit tiles, container queries for components that live in different slots, SVG icons and charts, and motion that respects user preferences. The result is the static shell of an IT operations dashboard, the kind of page a help desk keeps open all day.

## The plan

```text
┌──────────────────────── header (brand · search · user) ────────────────────────┐
│ nav  │  KPI tiles (auto-fit)                                                    │
│      ├──────────────────────────────────────────────┬──────────────────────────┤
│      │  chart (SVG)                                 │  alerts (container query)│
│      ├──────────────────────────────────────────────┴──────────────────────────┤
│      │  ticket table                                                            │
└──────┴──────────────────────────────────────────────────────────────────────────┘
```

Under 60rem the nav becomes a horizontal strip at the top and everything stacks.

## Step by step

1. **Tokens and layers.** Declare `@layer reset, base, layout, components, utilities;` and put colours, spacing and radius tokens on `:root`, with a dark variant.
2. **Page grid.** Named areas on `.app`, a media query for the narrow layout.
3. **KPI tiles.** `repeat(auto-fit, minmax(10rem, 1fr))`, each tile with a label, a big number and an SVG trend icon in `currentColor`.
4. **Alert list.** Each alert is a component that is compact in a narrow container and shows its full detail when its container is wide (`@container`).
5. **Chart.** A small SVG bar chart with `role="img"`, a title and a description.
6. **Table.** A real `<table>` with a caption, a sticky header and zebra rows; wrapped so it scrolls horizontally on small screens rather than breaking the layout.
7. **Motion.** A subtle hover lift on tiles and a pulsing "live" dot, both disabled under `prefers-reduced-motion`.

## Things to check when you're done

- Resize from 320px to 1600px wide: no horizontal page scroll, nothing overlaps.
- Tab through the page: every interactive element has a visible focus style, in a logical order.
- Toggle dark mode in your OS (or dev tools: Rendering → Emulate prefers-color-scheme).
- Turn on reduced motion (dev tools: Rendering → prefers-reduced-motion).
- Zoom to 200%: still usable.

@@quiz
Why use a container query for the alert component instead of a media query?
* The same component appears in slots of different widths, so it should respond to its own space
- Container queries are faster
- Media queries don't work in dashboards
- Alerts need JavaScript otherwise
? Components respond to their container, the page to the viewport.

@@quiz
A wide table on a phone pushes the whole page sideways. Best fix?
- Hide the table
* Wrap it in an element with `overflow-x: auto` so only the table scrolls
- Set the font size to 6px
- Use `white-space: nowrap` on the body
? Contain the overflow instead of letting it break the page.

@@flashcards
page layout :: grid areas + media query
KPI tiles :: auto-fit minmax
component layout :: container query
table on mobile :: wrapper with `overflow-x: auto`
sticky header :: `position: sticky; top: 0`
check list :: resize, keyboard, dark, reduced motion, 200% zoom

@@exercise Build the dashboard shell
Build the dashboard described in the lesson. It must use: `@layer` with at least reset, layout and components; tokens on `:root` including a dark-mode override; named grid areas with a narrow-screen media query; auto-fit KPI tiles; an alerts section that is a size container with an `@container` rule; an SVG chart with `role="img"`; a table with `<caption>` inside an `overflow-x: auto` wrapper with a sticky `th`; and a `prefers-reduced-motion` rule.
@@starter
<!doctype html>
<html lang="en">
<head>
<meta charset="utf-8">
<meta name="viewport" content="width=device-width, initial-scale=1">
<title>IT Operations</title>
<style>
  /* TODO: layers, tokens, layout, components */
</style>
</head>
<body>
  <div class="app">
    <header class="top">IT Operations</header>
    <nav class="side">Overview · Tickets · Servers · Reports</nav>
    <section class="kpis"></section>
    <section class="chart"></section>
    <section class="alerts"></section>
    <section class="tickets"></section>
  </div>
</body>
</html>
@@requires
@layer reset, base, layout, components, utilities;
prefers-color-scheme: dark
grid-template-areas:
repeat(auto-fit, minmax(10rem, 1fr))
container-type: inline-size
@container
role="img"
<caption>
overflow-x: auto
position: sticky
prefers-reduced-motion: reduce
@@hint
Build it in the order of the lesson's steps and check the page after each one. For the app grid: `grid-template-columns: 12rem 1fr 20rem; grid-template-areas: "top top top" "side kpis kpis" "side chart alerts" "side tickets tickets";` and in `@media (max-width: 60rem)` a single column with `"top" "side" "kpis" "alerts" "chart" "tickets"`.
@@solution
<!doctype html>
<html lang="en">
<head>
<meta charset="utf-8">
<meta name="viewport" content="width=device-width, initial-scale=1">
<title>IT Operations</title>
<style>
  @layer reset, base, layout, components, utilities;

  @layer reset {
    *, *::before, *::after { box-sizing: border-box; }
    body, h1, h2, p, ul, figure { margin: 0; }
    ul { padding: 0; list-style: none; }
  }

  @layer base {
    :root {
      --bg: #f1f5f9; --surface: #ffffff; --text: #0f172a; --muted: #64748b;
      --accent: #2563eb; --ok: #15803d; --warn: #b45309; --bad: #b91c1c;
      --space-2: 0.5rem; --space-3: 1rem; --radius: 10px;
      color-scheme: light dark;
    }
    @media (prefers-color-scheme: dark) {
      :root { --bg: #0f172a; --surface: #1e293b; --text: #f1f5f9; --muted: #94a3b8; --accent: #60a5fa; }
    }
    body { font-family: system-ui, sans-serif; background: var(--bg); color: var(--text); line-height: 1.5; }
    h2 { font-size: 1rem; margin-block-end: var(--space-2); }
    a { color: var(--accent); }
    :focus-visible { outline: 3px solid var(--accent); outline-offset: 2px; }
  }

  @layer layout {
    .app { display: grid; gap: var(--space-3); padding: var(--space-3); min-height: 100vh;
           grid-template-columns: 12rem minmax(0, 1fr) 20rem;
           grid-template-areas: "top top top" "side kpis kpis" "side chart alerts" "side tickets tickets"; }
    .top { grid-area: top; } .side { grid-area: side; } .kpis { grid-area: kpis; }
    .chart { grid-area: chart; } .alerts { grid-area: alerts; } .tickets { grid-area: tickets; }
    @media (max-width: 60rem) {
      .app { grid-template-columns: minmax(0, 1fr);
             grid-template-areas: "top" "side" "kpis" "alerts" "chart" "tickets"; }
      .side ul { display: flex; flex-wrap: wrap; gap: var(--space-2); }
    }
  }

  @layer components {
    .panel { background: var(--surface); border-radius: var(--radius); padding: var(--space-3); }
    .top { display: flex; justify-content: space-between; align-items: center; }
    .side a { display: block; padding: 0.3rem 0.5rem; border-radius: 6px; text-decoration: none; }

    .kpis { display: grid; gap: var(--space-3); grid-template-columns: repeat(auto-fit, minmax(10rem, 1fr)); }
    .kpi { background: var(--surface); border-radius: var(--radius); padding: var(--space-3);
           transition: transform 150ms ease-out; }
    .kpi:hover { transform: translateY(-2px); }
    .kpi__value { font-size: 1.8rem; font-weight: 700; display: flex; align-items: center; gap: 0.4rem; }
    .kpi__label { color: var(--muted); font-size: 0.85rem; }
    .icon { width: 1em; height: 1em; }
    .up { color: var(--bad); } .down { color: var(--ok); }

    .alerts { container-type: inline-size; }
    .alert { display: grid; gap: 0.1rem; padding-block: var(--space-2); border-block-end: 1px solid color-mix(in srgb, var(--muted) 30%, transparent); }
    .alert__detail { display: none; color: var(--muted); font-size: 0.85rem; }
    @container (min-width: 24rem) {
      .alert { grid-template-columns: 5rem 1fr; }
      .alert__detail { display: block; grid-column: 2; }
    }
    .live { color: var(--ok); animation: pulse 2s ease-in-out infinite; }
    @keyframes pulse { 50% { opacity: 0.35; } }

    .chart svg { inline-size: 100%; block-size: auto; }
    .bar { fill: var(--accent); }
    .chart text { font: 8px system-ui; fill: var(--muted); }

    .table-wrap { overflow-x: auto; max-block-size: 18rem; }
    table { border-collapse: collapse; inline-size: 100%; min-inline-size: 36rem; }
    caption { text-align: start; font-weight: 600; padding-block-end: var(--space-2); }
    th, td { text-align: start; padding: 0.4rem 0.6rem; }
    th { position: sticky; top: 0; background: var(--surface); border-block-end: 2px solid var(--muted); }
    tbody tr:nth-child(even) { background: color-mix(in srgb, var(--muted) 10%, transparent); }
  }

  @layer utilities {
    .visually-hidden { position: absolute; width: 1px; height: 1px; overflow: hidden; clip: rect(0 0 0 0); white-space: nowrap; }
    @media (prefers-reduced-motion: reduce) {
      .kpi, .live { transition: none; animation: none; }
      .kpi:hover { transform: none; }
    }
  }
</style>
</head>
<body>
  <svg style="display:none">
    <symbol id="i-up" viewBox="0 0 24 24"><path d="M4 16l6-6 4 4 6-6M14 8h6v6" fill="none" stroke="currentColor" stroke-width="2" /></symbol>
    <symbol id="i-down" viewBox="0 0 24 24"><path d="M4 8l6 6 4-4 6 6M14 16h6v-6" fill="none" stroke="currentColor" stroke-width="2" /></symbol>
  </svg>
  <div class="app">
    <header class="top panel">
      <h1>IT Operations</h1>
      <span><span class="live" aria-hidden="true">●</span> Live</span>
    </header>
    <nav class="side panel" aria-label="Main">
      <ul>
        <li><a href="#" aria-current="page">Overview</a></li>
        <li><a href="#">Tickets</a></li>
        <li><a href="#">Servers</a></li>
        <li><a href="#">Reports</a></li>
      </ul>
    </nav>
    <section class="kpis" aria-label="Key figures">
      <div class="kpi"><p class="kpi__label">Open tickets</p><p class="kpi__value">5 <svg class="icon up" aria-hidden="true"><use href="#i-up" /></svg><span class="visually-hidden">rising</span></p></div>
      <div class="kpi"><p class="kpi__label">SLA met</p><p class="kpi__value">62%</p></div>
      <div class="kpi"><p class="kpi__label">Servers up</p><p class="kpi__value">11/12</p></div>
      <div class="kpi"><p class="kpi__label">Failed logins (24h)</p><p class="kpi__value">8 <svg class="icon down" aria-hidden="true"><use href="#i-down" /></svg><span class="visually-hidden">falling</span></p></div>
    </section>
    <section class="chart panel">
      <h2>Tickets per day</h2>
      <svg viewBox="0 0 220 120" role="img" aria-labelledby="ct cd">
        <title id="ct">Tickets per weekday</title>
        <desc id="cd">From 4 on Monday to 11 on Friday.</desc>
        <rect class="bar" x="10" y="68" width="30" height="32" />
        <rect class="bar" x="50" y="44" width="30" height="56" />
        <rect class="bar" x="90" y="60" width="30" height="40" />
        <rect class="bar" x="130" y="28" width="30" height="72" />
        <rect class="bar" x="170" y="12" width="30" height="88" />
        <text x="25" y="112" text-anchor="middle">Mon</text>
        <text x="65" y="112" text-anchor="middle">Tue</text>
        <text x="105" y="112" text-anchor="middle">Wed</text>
        <text x="145" y="112" text-anchor="middle">Thu</text>
        <text x="185" y="112" text-anchor="middle">Fri</text>
      </svg>
    </section>
    <section class="alerts panel">
      <h2>Alerts</h2>
      <ul>
        <li class="alert"><strong>09:12</strong><span>VPN gateway down</span><span class="alert__detail">vpn-gw-01 not responding to health checks.</span></li>
        <li class="alert"><strong>08:40</strong><span>Disk 97% on fs01</span><span class="alert__detail">D:\shares grew 40 GB overnight.</span></li>
        <li class="alert"><strong>07:05</strong><span>Backup slow</span><span class="alert__detail">Nightly job took 3h 10m (usual 1h 20m).</span></li>
      </ul>
    </section>
    <section class="tickets panel">
      <div class="table-wrap">
        <table>
          <caption>Open tickets</caption>
          <thead><tr><th scope="col">ID</th><th scope="col">Title</th><th scope="col">Priority</th><th scope="col">Assignee</th><th scope="col">Opened</th></tr></thead>
          <tbody>
            <tr><td>9</td><td>File server almost full</td><td>P1</td><td>Fatima Khan</td><td>2024-03-05</td></tr>
            <tr><td>11</td><td>Suspicious login from abroad</td><td>P1</td><td>Ana Silva</td><td>2024-03-06</td></tr>
            <tr><td>7</td><td>Badge reader offline</td><td>P3</td><td>unassigned</td><td>2024-03-04</td></tr>
            <tr><td>12</td><td>Teams calls dropping</td><td>P2</td><td>Kenji Sato</td><td>2024-03-06</td></tr>
            <tr><td>4</td><td>Printer jams on floor 2</td><td>P4</td><td>unassigned</td><td>2024-03-01</td></tr>
          </tbody>
        </table>
      </div>
    </section>
  </div>
</body>
</html>
