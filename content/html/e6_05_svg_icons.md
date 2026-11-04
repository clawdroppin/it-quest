@@lesson e6-svg SVG: Icons, Shapes and Accessible Graphics
@@text
SVG (Scalable Vector Graphics) is an XML format for drawings described as shapes rather than pixels. It stays sharp at any size and zoom level, is tiny for icons and diagrams, can be styled with CSS, and can be written directly inside HTML. Every icon set you'll meet (Lucide, Heroicons, Material Symbols) is SVG.

## The coordinate system

```html
<svg viewBox="0 0 100 60" width="300" role="img" aria-label="Shapes demo">
  <rect x="5" y="5" width="30" height="20" rx="4" fill="#2563eb" />
  <circle cx="55" cy="15" r="10" fill="#e11d48" />
  <line x1="75" y1="5" x2="95" y2="25" stroke="#0f172a" stroke-width="2" />
  <polyline points="5,55 20,40 35,48 50,32 65,38 80,30 95,35" fill="none" stroke="#16a34a" stroke-width="2" />
  <text x="50" y="58" font-size="6" text-anchor="middle">viewBox 0 0 100 60</text>
</svg>
```

`viewBox="minX minY width height"` defines the drawing's internal coordinates; the `width`/`height` attributes (or CSS) define how big it's displayed. The drawing scales to fit, so you design once in convenient units. The y axis points **down**.

## Shapes and paths

| Element | Attributes |
|---|---|
| `<rect>` | `x y width height rx` (rounded corners) |
| `<circle>` / `<ellipse>` | `cx cy r` / `rx ry` |
| `<line>` | `x1 y1 x2 y2` |
| `<polyline>` / `<polygon>` | `points="x,y x,y ..."` (polygon closes) |
| `<path>` | `d="M10 10 L50 10 Q60 30 50 50 Z"` |
| `<text>` | `x y`, `text-anchor`, `dominant-baseline` |
| `<g>` | a group; `transform="translate(10 20)"` |

Path commands: `M` move, `L` line, `H`/`V` horizontal/vertical line, `C`/`Q` curves, `A` arc, `Z` close. Uppercase is absolute, lowercase relative. You rarely write paths by hand; design tools and icon sets produce them.

Styling: `fill`, `stroke`, `stroke-width`, `stroke-linecap="round"`, `stroke-linejoin`, `opacity`. All of these can also be set from CSS.

## Icons that follow the text colour

The key trick: `fill="currentColor"` (or `stroke="currentColor"`) makes the icon use the surrounding text `color`, so it changes with hover, dark mode and themes automatically:

```html
<style>
  .icon { width: 1.25em; height: 1.25em; vertical-align: -0.2em; }
  .link { color: #2563eb; } .link:hover { color: #1e3a8a; }
</style>
<a class="link" href="#"><svg class="icon" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" aria-hidden="true"><path d="M21 12a9 9 0 1 1-3-6.7L21 8" /><path d="M21 3v5h-5" /></svg> Refresh</a>
```

Sizing in `em` keeps icons in proportion to the text beside them.

## Accessible SVG

Two cases:

1. **Decorative** (an icon next to a text label): hide it from screen readers with `aria-hidden="true"` (and `focusable="false"` for old IE/Edge).
2. **Meaningful** (an icon-only button, a chart, a logo): give it a name.

```html
<!-- icon-only button: name the BUTTON, hide the icon -->
<button aria-label="Delete ticket"><svg aria-hidden="true" ...>...</svg></button>

<!-- a standalone meaningful graphic -->
<svg role="img" aria-labelledby="chart-title chart-desc" viewBox="0 0 100 50">
  <title id="chart-title">Tickets per day</title>
  <desc id="chart-desc">Rising from 4 on Monday to 11 on Friday.</desc>
  ...
</svg>
```

## Reusing icons with a sprite

Define each icon once in a hidden SVG as a `<symbol>`, then `<use>` it anywhere:

```html
<svg style="display:none"><symbol id="i-check" viewBox="0 0 24 24"><path d="M5 12l5 5L20 7" fill="none" stroke="currentColor" stroke-width="2" /></symbol></svg>
<svg class="icon" aria-hidden="true"><use href="#i-check" /></svg> Done
```

## SVG vs `<img>` vs icon fonts

- **Inline `<svg>`**: stylable with CSS (currentColor, hover), animatable, no extra request; best for icons and interactive graphics.
- **`<img src="logo.svg" alt="...">`**: cached, simple, but you can't style its inside from the page's CSS.
- **Icon fonts** (old Font Awesome): avoid; they render as missing boxes when fonts fail, are announced oddly by screen readers, and blur at some sizes.

## Security note

SVG files can contain `<script>`. Never inline user-uploaded SVGs into your page without sanitising them; serve uploads as `<img>` (which doesn't run scripts) or from a separate domain.

@@quiz
What does `viewBox="0 0 24 24"` define?
- The display size in pixels
* The internal coordinate system that the drawing scales from
- The position on the page
- The colour palette
? Display size comes from width/height or CSS.

@@quiz
How do you make an inline SVG icon match the text colour, including on hover?
* `fill="currentColor"` or `stroke="currentColor"`
- `fill="inherit-text"`
- Use a PNG
- Set `color` on the path only
? It follows the CSS `color` of the parent.

@@quiz
An icon-only "Delete" button: where does the accessible name go?
- On the `<path>`
* On the button (`aria-label="Delete ticket"`), with the SVG `aria-hidden="true"`
- In a `title` attribute on the svg only
- Nowhere; icons are self-explanatory
? Name the control, hide the decoration.

@@quiz
Why not inline a user-uploaded SVG into your page?
- It's too big
* SVG can contain scripts, which would run in your page (XSS)
- SVG doesn't work inline
- Browsers block it
? Sanitise, or display it as `<img>`.

@@flashcards
`viewBox` :: internal coordinates
`<path d="M L C Z">` :: arbitrary shape
`fill="currentColor"` :: use the text colour
`stroke-linecap="round"` :: rounded line ends
`aria-hidden="true"` :: decorative SVG
`role="img"` + `<title>` :: meaningful SVG
`<symbol>` + `<use href>` :: sprite reuse
`text-anchor="middle"` :: centre SVG text
y axis in SVG :: points down
icon size in `em` :: scales with the text

@@exercise An icon button set
Create three buttons with inline SVG icons that use `stroke="currentColor"`: "Restart" (with a visible text label), "Delete" (icon-only) and "Settings" (icon-only). The icons are decorative (`aria-hidden="true"`), the icon-only buttons get `aria-label`s, and a `.icon` class sizes the SVGs at `1.25em`. Turn the Delete button's colour red on hover and check the icon follows.
@@starter
<!doctype html>
<html lang="en">
<head><meta charset="utf-8"><title>Icons</title>
<style>
  button { font: inherit; display: inline-flex; gap: 0.4rem; align-items: center; padding: 0.4rem 0.7rem; }
</style>
</head>
<body>
  <button>Restart</button>
  <button>X</button>
  <button>⚙</button>
</body>
</html>
@@requires
stroke="currentColor"
aria-hidden="true"
aria-label="Delete ticket"
aria-label="Settings"
.icon { width: 1.25em; height: 1.25em
viewBox="0 0 24 24"
@@hint
Each icon: `<svg class="icon" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" aria-hidden="true"><path d="..." /></svg>`. A cross is `M6 6l12 12M18 6L6 18`; a circle with a dot works for settings: `<circle cx="12" cy="12" r="3" /><circle cx="12" cy="12" r="9" />`.
@@solution
<!doctype html>
<html lang="en">
<head><meta charset="utf-8"><title>Icons</title>
<style>
  button { font: inherit; display: inline-flex; gap: 0.4rem; align-items: center; padding: 0.4rem 0.7rem; color: #0f172a; }
  .icon { width: 1.25em; height: 1.25em; }
  .danger:hover { color: #dc2626; }
</style>
</head>
<body>
  <button>
    <svg class="icon" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" aria-hidden="true"><path d="M21 12a9 9 0 1 1-3-6.7L21 8" /><path d="M21 3v5h-5" /></svg>
    Restart
  </button>
  <button class="danger" aria-label="Delete ticket">
    <svg class="icon" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" aria-hidden="true"><path d="M6 6l12 12M18 6L6 18" /></svg>
  </button>
  <button aria-label="Settings">
    <svg class="icon" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" aria-hidden="true"><circle cx="12" cy="12" r="3" /><circle cx="12" cy="12" r="9" /></svg>
  </button>
</body>
</html>

@@exercise An accessible SVG bar chart
Draw a bar chart of tickets per weekday (Mon 4, Tue 7, Wed 5, Thu 9, Fri 11) in an SVG with `viewBox="0 0 220 120"`. Each bar is a `<rect>` 30 units wide, with height `value × 8`, standing on a baseline at y = 100 (so `y = 100 − height`). Label the days with `<text>` below the bars. Make it accessible: `role="img"`, `aria-labelledby` pointing to a `<title>` and a `<desc>` that summarises the trend.
@@starter
<!doctype html>
<html lang="en">
<head><meta charset="utf-8"><title>Chart</title></head>
<body>
  <h2>Tickets this week</h2>
  <svg viewBox="0 0 220 120" width="440">
  </svg>
</body>
</html>
@@requires
role="img"
aria-labelledby="t d"
<title id="t">
<desc id="d">
height="88"
y="12"
text-anchor="middle"
@@hint
Mon: `<rect x="10" y="68" width="30" height="32" />`, Tue: x=50, height 56, y 44; Wed: x=90, height 40, y 60; Thu: x=130, height 72, y 28; Fri: x=170, height 88, y 12. Labels: `<text x="25" y="114" text-anchor="middle">Mon</text>` and so on.
@@solution
<!doctype html>
<html lang="en">
<head><meta charset="utf-8"><title>Chart</title>
<style>
  .bar { fill: #2563eb; }
  .bar:hover { fill: #1e3a8a; }
  text { font: 9px system-ui; fill: #334155; }
</style>
</head>
<body>
  <h2>Tickets this week</h2>
  <svg viewBox="0 0 220 120" width="440" role="img" aria-labelledby="t d">
    <title id="t">Tickets per weekday</title>
    <desc id="d">Tickets rose from 4 on Monday to 11 on Friday, with a dip on Wednesday.</desc>
    <line x1="5" y1="100" x2="215" y2="100" stroke="#94a3b8" />
    <rect class="bar" x="10" y="68" width="30" height="32" />
    <rect class="bar" x="50" y="44" width="30" height="56" />
    <rect class="bar" x="90" y="60" width="30" height="40" />
    <rect class="bar" x="130" y="28" width="30" height="72" />
    <rect class="bar" x="170" y="12" width="30" height="88" />
    <text x="25" y="114" text-anchor="middle">Mon</text>
    <text x="65" y="114" text-anchor="middle">Tue</text>
    <text x="105" y="114" text-anchor="middle">Wed</text>
    <text x="145" y="114" text-anchor="middle">Thu</text>
    <text x="185" y="114" text-anchor="middle">Fri</text>
  </svg>
</body>
</html>

@@exercise An icon sprite and a status badge
Define two icons once as `<symbol>`s in a hidden SVG (`id="i-check"` and `id="i-alert"`) and reuse them with `<use href="#i-check" />` in three status badges: "Backups OK" and "VPN OK" (check icon, green) and "Disk 97%" (alert icon, red). The badge colours come from `color`, so the symbols use `currentColor`.
@@starter
<!doctype html>
<html lang="en">
<head><meta charset="utf-8"><title>Status</title>
<style>
  .badge { display: inline-flex; gap: 0.3rem; align-items: center; padding: 0.2rem 0.6rem; border-radius: 999px; background: #f1f5f9; }
</style>
</head>
<body>
  <span class="badge">Backups OK</span>
  <span class="badge">VPN OK</span>
  <span class="badge">Disk 97%</span>
</body>
</html>
@@requires
<symbol id="i-check"
<symbol id="i-alert"
<use href="#i-check" />
<use href="#i-alert" />
currentColor
@@hint
`<svg style="display:none"><symbol id="i-check" viewBox="0 0 24 24"><path d="M5 12l5 5L20 7" fill="none" stroke="currentColor" stroke-width="3" /></symbol>...</svg>`; badges: `<span class="badge ok"><svg class="icon" aria-hidden="true"><use href="#i-check" /></svg> Backups OK</span>` with `.ok { color: #15803d; }`.
@@solution
<!doctype html>
<html lang="en">
<head><meta charset="utf-8"><title>Status</title>
<style>
  .badge { display: inline-flex; gap: 0.3rem; align-items: center; padding: 0.2rem 0.6rem; border-radius: 999px; background: #f1f5f9; }
  .icon { width: 1em; height: 1em; }
  .ok { color: #15803d; }
  .bad { color: #b91c1c; }
</style>
</head>
<body>
  <svg style="display:none">
    <symbol id="i-check" viewBox="0 0 24 24"><path d="M5 12l5 5L20 7" fill="none" stroke="currentColor" stroke-width="3" stroke-linecap="round" /></symbol>
    <symbol id="i-alert" viewBox="0 0 24 24"><path d="M12 3L2 21h20Z" fill="none" stroke="currentColor" stroke-width="2" /><path d="M12 10v5M12 18v1" stroke="currentColor" stroke-width="2" /></symbol>
  </svg>
  <span class="badge ok"><svg class="icon" aria-hidden="true"><use href="#i-check" /></svg> Backups OK</span>
  <span class="badge ok"><svg class="icon" aria-hidden="true"><use href="#i-check" /></svg> VPN OK</span>
  <span class="badge bad"><svg class="icon" aria-hidden="true"><use href="#i-alert" /></svg> Disk 97%</span>
</body>
</html>
