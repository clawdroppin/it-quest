@@lesson h2-css-basics CSS: Selectors, Colours and the Box Model
@@text
## Adding CSS

```html
<!doctype html>
<html lang="en">
<head>
  <meta charset="utf-8">
  <title>Styled</title>
  <style>
    body { font-family: system-ui, sans-serif; margin: 2rem; color: #222; }
    h1 { color: #0b6e4f; }
    .warning { background: #fff3cd; border-left: 4px solid #e0a800; padding: 0.5rem 1rem; }
    #maintenance { font-weight: bold; }
  </style>
</head>
<body>
  <h1>Status</h1>
  <p class="warning">VPN is slow this morning.</p>
  <p id="maintenance">Maintenance on Saturday.</p>
</body>
</html>
```

A CSS **rule** is a **selector** plus **declarations** (`property: value;`) in braces. On real sites the CSS lives in a separate file, linked with `<link rel="stylesheet" href="style.css">`, so every page shares it. `style="color: red"` directly on an element also works, but it's hard to maintain.

## Selectors

| Selector | Matches |
|---|---|
| `p` | every `<p>` |
| `.warning` | elements with `class="warning"` (an element can have several: `class="card warning"`) |
| `#maintenance` | the element with `id="maintenance"` |
| `nav a` | `<a>` anywhere inside a `<nav>` |
| `ul > li` | `<li>` that are *direct* children of a `<ul>` |
| `h1, h2` | both |
| `a:hover`, `input:focus` | states: mouse over, focused |
| `li:nth-child(odd)`, `tr:first-child` | position |
| `input[type="email"]` | by attribute |

Style with **classes**. Ids are unique, so ids in CSS make styles hard to reuse.

## The cascade and specificity

When several rules set the same property, the browser decides:

1. **Specificity**: id (`#x`) beats class (`.x`) beats element (`p`).
2. If equal, the rule that comes **later** wins.
3. `!important` overrides both. Avoid it; it starts an arms race.

Some properties (`color`, `font-family`) are **inherited** by child elements; others (`border`, `padding`) aren't.

## Colours, fonts and units

```html
<style>
  .demo {
    color: #1a1a2e;                    /* hex */
    background: rgb(230 240 255);      /* red green blue */
    border: 2px solid hsl(210 80% 50%);/* hue saturation lightness */
    font: 1.1rem/1.6 "Noto Sans JP", system-ui, sans-serif;
    padding: 1em;
  }
</style>
<p class="demo">こんにちは: a box with colour, a border and a font stack.</p>
```

Units: `px` (fixed), `rem` (relative to the root font size, usually 16px; best for text and spacing), `em` (relative to this element's font size), `%` (of the parent), `vw`/`vh` (percent of the window). Prefer `rem` so that users who make the text bigger get a page that scales.

A **font stack** lists fallbacks: if "Noto Sans JP" isn't installed, the next one is used.

## The box model

Every element is a box: **content**, then **padding** (inside the border), **border**, and **margin** (outside, between boxes).

```html
<style>
  * { box-sizing: border-box; }    /* width includes padding and border: always use this */
  .card {
    width: 260px;
    padding: 16px;
    border: 1px solid #ccc;
    border-radius: 8px;
    margin: 12px 0;                /* top/bottom 12px, left/right 0 */
    box-shadow: 0 2px 6px rgb(0 0 0 / 0.15);
  }
</style>
<div class="card">A card: 260px wide in total, thanks to border-box.</div>
<div class="card">Another card, 12px below.</div>
```

Shorthand order is **top, right, bottom, left** (clockwise): `margin: 0 auto` centres a block with a width.

`display` decides how boxes flow: `block` (full width, new line: `div`, `p`, `h1`), `inline` (in the text: `a`, `strong`, `span`), `inline-block`, `none` (hidden), and `flex`/`grid` (next lesson). `<div>` and `<span>` are meaning-free boxes for styling.

@@quiz
Which selector matches `<p class="note">`?
* `.note`
- `#note`
- `note`
- `p#note`
? `.` selects classes, `#` selects ids, and a bare name selects element types.

@@quiz
`p { color: blue; }` and `.intro { color: green; }`. What colour is `<p class="intro">`?
* Green: a class selector is more specific than an element selector
- Blue: it comes first
- Blue: elements always win
- A mix
? Specificity: id > class > element. Order only decides between equally specific rules.

@@quiz
With `box-sizing: border-box`, a box has `width: 200px; padding: 20px; border: 5px`. How wide is it on screen?
* 200px
- 250px
- 240px
- 210px
? border-box includes padding and border in the width. With the default content-box, it would be 250px.

@@quiz
Why prefer `rem` to `px` for font sizes?
* It scales with the user's preferred text size
- It's more precise
- Browsers don't support px
- It's faster
? Accessibility: people with low vision set bigger default fonts. rem respects that.

@@drill
Which `box-sizing` value includes padding and border in the width?
= border-box
? *, *::before, *::after { box-sizing: border-box; }.

@@flashcards
`.class` :: class selector
`#id` :: id selector (avoid for styling)
`a:hover` :: a state (pseudo-class)
specificity :: id > class > element
`box-sizing: border-box` :: width includes padding and border
`margin: 10px 20px` :: top/bottom 10px, left/right 20px
`rem` :: relative to the root font size
`display: none` :: hide the element completely

@@exercise Alert boxes
Style the three paragraphs with a shared `.alert` class (padding, border-radius, a left border) and one extra class each: `.ok` (green), `.warn` (yellow) and `.error` (red), with a background colour each. Use `box-sizing: border-box` on everything.
@@starter
<!doctype html>
<html lang="en">
<head>
  <meta charset="utf-8">
  <title>Alerts</title>
  <style>
    body { font-family: system-ui, sans-serif; margin: 2rem; }
  </style>
</head>
<body>
  <p>Backups: all good</p>
  <p>Disk on db01 is 85% full</p>
  <p>Mail server is down</p>
</body>
</html>
@@requires
box-sizing: border-box
.alert {
.ok {
.warn {
.error {
class="alert ok"
class="alert warn"
class="alert error"
border-left
@@hint
`.alert { padding: 0.75rem 1rem; border-radius: 6px; border-left: 6px solid; }` then `.ok { background: #d1f2e0; border-color: #1e8a4c; }`, and so on. In the HTML: `<p class="alert ok">`.
@@solution
<!doctype html>
<html lang="en">
<head>
  <meta charset="utf-8">
  <title>Alerts</title>
  <style>
    * { box-sizing: border-box; }
    body { font-family: system-ui, sans-serif; margin: 2rem; }
    .alert { padding: 0.75rem 1rem; border-radius: 6px; border-left: 6px solid; margin: 0.5rem 0; }
    .ok { background: #d1f2e0; border-color: #1e8a4c; }
    .warn { background: #fff3cd; border-color: #e0a800; }
    .error { background: #fde2e1; border-color: #c62828; }
  </style>
</head>
<body>
  <p class="alert ok">Backups: all good</p>
  <p class="alert warn">Disk on db01 is 85% full</p>
  <p class="alert error">Mail server is down</p>
</body>
</html>

@@exercise Zebra table
Style the table: full width, collapsed borders (`border-collapse: collapse`), padding in the cells, a dark header row with white text, and every even body row light grey using `:nth-child(even)`. Highlight a row on `:hover`.
@@starter
<!doctype html>
<html lang="en">
<head>
  <meta charset="utf-8">
  <title>Servers</title>
  <style>
  </style>
</head>
<body>
  <table>
    <thead><tr><th>Host</th><th>CPU</th><th>Status</th></tr></thead>
    <tbody>
      <tr><td>web01</td><td>35%</td><td>OK</td></tr>
      <tr><td>web02</td><td>92%</td><td>Hot</td></tr>
      <tr><td>db01</td><td>71%</td><td>OK</td></tr>
      <tr><td>mail01</td><td>12%</td><td>OK</td></tr>
    </tbody>
  </table>
</body>
</html>
@@requires
border-collapse: collapse
:nth-child(even)
:hover
th
@@hint
`table { width: 100%; border-collapse: collapse; } th, td { padding: 8px 12px; text-align: left; } th { background: #263238; color: white; } tbody tr:nth-child(even) { background: #f0f0f0; } tbody tr:hover { background: #e3f2fd; }`
@@solution
<!doctype html>
<html lang="en">
<head>
  <meta charset="utf-8">
  <title>Servers</title>
  <style>
    body { font-family: system-ui, sans-serif; margin: 2rem; }
    table { width: 100%; border-collapse: collapse; }
    th, td { padding: 8px 12px; text-align: left; border-bottom: 1px solid #ddd; }
    th { background: #263238; color: white; }
    tbody tr:nth-child(even) { background: #f0f0f0; }
    tbody tr:hover { background: #e3f2fd; }
  </style>
</head>
<body>
  <table>
    <thead><tr><th>Host</th><th>CPU</th><th>Status</th></tr></thead>
    <tbody>
      <tr><td>web01</td><td>35%</td><td>OK</td></tr>
      <tr><td>web02</td><td>92%</td><td>Hot</td></tr>
      <tr><td>db01</td><td>71%</td><td>OK</td></tr>
      <tr><td>mail01</td><td>12%</td><td>OK</td></tr>
    </tbody>
  </table>
</body>
</html>
