@@lesson h3-responsive Responsive Design and Modern CSS
@@text
## One page, every screen

More than half of web traffic is from phones; in Japan the share is even higher. A **responsive** page adapts to the screen instead of having a separate "mobile site". The ingredients:

1. `<meta name="viewport" content="width=device-width, initial-scale=1">` in the head. Without it, phones pretend to be a 980px-wide desktop and shrink everything.
2. Flexible layouts: `%`, `fr`, `flex`, `minmax`, `max-width` instead of fixed widths.
3. **Media queries** for the changes that need them.
4. Images that shrink: `img { max-width: 100%; height: auto; }`.

## Media queries

```html
<style>
  body { font-family: system-ui, sans-serif; margin: 1rem; }
  .layout { display: grid; gap: 1rem; }               /* mobile first: one column */
  .layout > div { background: #f1f5f9; padding: 1rem; border-radius: 6px; }

  @media (min-width: 700px) {                         /* tablets and up: two columns */
    .layout { grid-template-columns: 220px 1fr; }
  }
  @media (min-width: 1100px) {                        /* wide screens: a third column */
    .layout { grid-template-columns: 220px 1fr 260px; }
  }
</style>
<div class="layout">
  <div>Menu</div>
  <div>Main content. Resize the window to see the layout change.</div>
  <div>Side panel</div>
</div>
```

**Mobile first**: write the simple one-column version as the default, then *add* complexity for bigger screens with `min-width` queries. It's simpler than starting from a desktop design and undoing it.

Other useful media features:

```css
@media (prefers-color-scheme: dark) { … }        /* the user chose dark mode */
@media (prefers-reduced-motion: reduce) { … }    /* turn animations off */
@media print { nav, .buttons { display: none; } } /* a clean printout */
@media (hover: none) { … }                       /* touch screens */
```

## CSS custom properties (variables)

Define values once and reuse them. Changing a theme becomes changing a few lines:

```html
<style>
  :root {
    --bg: #ffffff;
    --text: #1e293b;
    --accent: #0ea5e9;
    --radius: 8px;
  }
  @media (prefers-color-scheme: dark) {
    :root { --bg: #0f172a; --text: #e2e8f0; --accent: #38bdf8; }
  }
  body { background: var(--bg); color: var(--text); font-family: system-ui, sans-serif; padding: 1rem; }
  .button { background: var(--accent); color: white; border: 0; padding: 0.5rem 1rem; border-radius: var(--radius); }
</style>
<p>Switch your system to dark mode and this page follows.</p>
<button class="button">Restart service</button>
```

## Fluid sizes

```css
h1 { font-size: clamp(1.5rem, 4vw, 3rem); }      /* between 1.5rem and 3rem, growing with the window */
.content { width: min(100% - 2rem, 70ch); margin-inline: auto; }   /* readable line length, centred */
```

`ch` is the width of a "0": about 60–75 characters per line is the most readable width for text.

## Transitions and animations

```html
<style>
  .card { padding: 1rem; background: #e0f2fe; border-radius: 8px; transition: transform 0.2s, box-shadow 0.2s; font-family: sans-serif; }
  .card:hover { transform: translateY(-3px); box-shadow: 0 6px 16px rgb(0 0 0 / 0.15); }
  @keyframes pulse { 50% { opacity: 0.4; } }
  .live { color: #dc2626; animation: pulse 1.5s infinite; }
  @media (prefers-reduced-motion: reduce) { .live { animation: none; } }
</style>
<div class="card">Hover me <span class="live">● LIVE</span></div>
```

Keep motion small and purposeful, and respect `prefers-reduced-motion`.

@@quiz
What does `<meta name="viewport" content="width=device-width, initial-scale=1">` do?
* Tells phones to use their real width instead of pretending to be a desktop screen
- Makes the page load faster
- Sets the font size
- Hides the page on desktops
? Without it, media queries won't work as expected on phones.

@@quiz
"Mobile first" CSS means...
* The default styles are for small screens; `min-width` media queries add layout for bigger ones
- Making a separate mobile website
- Testing only on phones
- Using `max-width` queries for everything
? Start simple, then enhance.

@@quiz
What's the benefit of `--accent: #0ea5e9;` with `var(--accent)`?
* The colour is defined once, so themes and dark mode only change the variable
- It's faster to render
- It's required for colours
- Variables can't be changed by media queries
? Custom properties can be redefined in media queries and for parts of the page.

@@flashcards
`@media (min-width: 700px)` :: styles for screens at least 700px wide
mobile first :: default for small screens, add for larger
`prefers-color-scheme: dark` :: the user wants dark mode
`--name: value` / `var(--name)` :: CSS custom property
`clamp(min, preferred, max)` :: a value within limits
`max-width: 100%` on images :: never wider than their container
`@media print` :: styles for printing
`prefers-reduced-motion` :: the user wants less animation

@@exercise Responsive cards
Make the `.cards` container one column by default, two columns from `600px` and four columns from `1000px`, using media queries with `min-width`. Images inside cards must never overflow (`max-width: 100%`).
@@starter
<!doctype html>
<html lang="en">
<head>
  <meta charset="utf-8">
  <meta name="viewport" content="width=device-width, initial-scale=1">
  <title>Services</title>
  <style>
    body { font-family: system-ui, sans-serif; margin: 1rem; }
    .card { background: #f8fafc; border: 1px solid #e2e8f0; border-radius: 8px; padding: 1rem; }
  </style>
</head>
<body>
  <div class="cards">
    <div class="card"><img src="https://placehold.co/400x120?text=Mail" alt="Mail service"><h2>Mail</h2><p>Running</p></div>
    <div class="card"><img src="https://placehold.co/400x120?text=VPN" alt="VPN service"><h2>VPN</h2><p>Running</p></div>
    <div class="card"><img src="https://placehold.co/400x120?text=Wiki" alt="Wiki service"><h2>Wiki</h2><p>Degraded</p></div>
    <div class="card"><img src="https://placehold.co/400x120?text=Files" alt="File service"><h2>Files</h2><p>Running</p></div>
  </div>
</body>
</html>
@@requires
display: grid
@media (min-width: 600px)
@media (min-width: 1000px)
repeat(2, 1fr)
repeat(4, 1fr)
max-width: 100%
@@hint
`.cards { display: grid; gap: 1rem; } @media (min-width: 600px) { .cards { grid-template-columns: repeat(2, 1fr); } }` and the same for 1000px with 4. Then `.card img { max-width: 100%; height: auto; }`.
@@solution
<!doctype html>
<html lang="en">
<head>
  <meta charset="utf-8">
  <meta name="viewport" content="width=device-width, initial-scale=1">
  <title>Services</title>
  <style>
    body { font-family: system-ui, sans-serif; margin: 1rem; }
    .cards { display: grid; gap: 1rem; }
    .card { background: #f8fafc; border: 1px solid #e2e8f0; border-radius: 8px; padding: 1rem; }
    .card img { max-width: 100%; height: auto; display: block; }
    @media (min-width: 600px) { .cards { grid-template-columns: repeat(2, 1fr); } }
    @media (min-width: 1000px) { .cards { grid-template-columns: repeat(4, 1fr); } }
  </style>
</head>
<body>
  <div class="cards">
    <div class="card"><img src="https://placehold.co/400x120?text=Mail" alt="Mail service"><h2>Mail</h2><p>Running</p></div>
    <div class="card"><img src="https://placehold.co/400x120?text=VPN" alt="VPN service"><h2>VPN</h2><p>Running</p></div>
    <div class="card"><img src="https://placehold.co/400x120?text=Wiki" alt="Wiki service"><h2>Wiki</h2><p>Degraded</p></div>
    <div class="card"><img src="https://placehold.co/400x120?text=Files" alt="File service"><h2>Files</h2><p>Running</p></div>
  </div>
</body>
</html>

@@exercise Dark mode theme
Define `--bg`, `--text` and `--accent` on `:root`, use them with `var()` for the body and the button, and redefine them inside `@media (prefers-color-scheme: dark)`.
@@starter
<!doctype html>
<html lang="en">
<head>
  <meta charset="utf-8">
  <title>Theme</title>
  <style>
    body { background: white; color: #111; font-family: system-ui, sans-serif; padding: 1rem; }
    button { background: #2563eb; color: white; border: 0; padding: 0.5rem 1rem; border-radius: 6px; }
  </style>
</head>
<body>
  <h1>Ticket #4821</h1>
  <p>Printer on 3F is jammed again.</p>
  <button>Close ticket</button>
</body>
</html>
@@requires
:root
--bg:
--text:
--accent:
var(--bg)
var(--text)
var(--accent)
@media (prefers-color-scheme: dark)
@@hint
`:root { --bg: white; --text: #111; --accent: #2563eb; } @media (prefers-color-scheme: dark) { :root { --bg: #0b1120; --text: #e5e7eb; --accent: #60a5fa; } } body { background: var(--bg); color: var(--text); }`
@@solution
<!doctype html>
<html lang="en">
<head>
  <meta charset="utf-8">
  <title>Theme</title>
  <style>
    :root { --bg: white; --text: #111; --accent: #2563eb; }
    @media (prefers-color-scheme: dark) {
      :root { --bg: #0b1120; --text: #e5e7eb; --accent: #60a5fa; }
    }
    body { background: var(--bg); color: var(--text); font-family: system-ui, sans-serif; padding: 1rem; }
    button { background: var(--accent); color: white; border: 0; padding: 0.5rem 1rem; border-radius: 6px; }
  </style>
</head>
<body>
  <h1>Ticket #4821</h1>
  <p>Printer on 3F is jammed again.</p>
  <button>Close ticket</button>
</body>
</html>
