@@lesson gu5-svg-icons Icon Creation: Drawing Icons by Hand with SVG
@@concepts strings
@@text
Icons are tiny drawings that replace words: 💾 save, 🔍 search, ⚙ settings. Most modern icons are **SVG** (Scalable Vector Graphics): text files that describe shapes, so they're sharp at any size, tiny to download, can be recoloured with CSS, and can be animated. You can write them by hand in any text editor, and that's exactly what this lesson does.

## The canvas: viewBox

```html
<svg width="96" height="96" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"
     stroke-linecap="round" stroke-linejoin="round" style="color:#7c3aed">
  <circle cx="12" cy="12" r="9" />
  <path d="M8 14s1.5 2 4 2 4-2 4-2" />
  <line x1="9" y1="9" x2="9.01" y2="9" />
  <line x1="15" y1="9" x2="15.01" y2="9" />
</svg>
```

- `viewBox="0 0 24 24"` defines your **drawing grid**: 24 × 24 units, the standard for UI icons. `width`/`height` only set how big it's shown: the same drawing works at 16, 24, 96 or 512 pixels.
- `stroke="currentColor"` makes the icon use the surrounding text colour, so `color: red` on a button recolours its icon. This is the single most useful icon trick.
- `fill="none"` + `stroke` = an **outline** icon. `fill="currentColor"` with no stroke = a **solid** icon.
- `stroke-linecap="round"` and `stroke-linejoin="round"` give the soft, friendly look of most modern icon sets.

## The basic shapes

```html
<svg width="300" height="100" viewBox="0 0 72 24" fill="none" stroke="#0ea5e9" stroke-width="1.5">
  <rect x="2" y="4" width="16" height="16" rx="4" />            <!-- rounded square: rx is the corner radius -->
  <circle cx="34" cy="12" r="8" />                             <!-- centre x, centre y, radius -->
  <ellipse cx="58" cy="12" rx="10" ry="6" />
  <line x1="2" y1="22" x2="70" y2="22" stroke-dasharray="2 3" />
  <polyline points="22,20 28,14 32,18" />                       <!-- connected straight lines -->
</svg>
```

## Paths: the shape that does everything

`<path d="...">` draws with commands. Upper case = absolute coordinates, lower case = relative to the current point.

| Command | Meaning |
|---|---|
| `M x y` | **move** the pen to x,y (without drawing) |
| `L x y` | a **line** to x,y |
| `H x` / `V y` | a horizontal / vertical line |
| `Q cx cy x y` | a **quadratic curve** to x,y, pulled towards the control point cx,cy |
| `C c1x c1y c2x c2y x y` | a **cubic curve** with two control points |
| `A rx ry rot large sweep x y` | an **arc** of an ellipse |
| `Z` | close the shape back to the start |

```html
<svg width="320" height="110" viewBox="0 0 96 32" fill="none" stroke="#f43f5e" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
  <!-- a heart: two curves meeting at the bottom point -->
  <path d="M16 27 C 6 20, 2 13, 6 8 C 9 4, 14 5, 16 9 C 18 5, 23 4, 26 8 C 30 13, 26 20, 16 27 Z" fill="#fecdd3" />
  <!-- a check mark -->
  <path d="M38 16 l5 5 l10 -11" />
  <!-- a speech bubble: rounded rect with a tail -->
  <path d="M62 6 h24 a4 4 0 0 1 4 4 v10 a4 4 0 0 1 -4 4 h-14 l-6 5 v-5 h-4 a4 4 0 0 1 -4 -4 v-10 a4 4 0 0 1 4 -4 z" />
</svg>
```

You don't need to design complex curves by hand: start from simple shapes, and when you need curves, draw in a vector editor (**Figma**, **Inkscape**, Illustrator, Affinity) and export SVG. But reading paths lets you fix, simplify and animate them.

## Building an icon from shapes

A bell, from four simple parts:

```html
<svg width="96" height="96" viewBox="0 0 24 24" fill="none" stroke="#f59e0b" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
  <path d="M6 16 V11 a6 6 0 0 1 12 0 v5" />     <!-- the dome: up, a half-circle arc, down -->
  <path d="M4 16 h16" />                         <!-- the rim -->
  <path d="M10 19 a2 2 0 0 0 4 0" />             <!-- the clapper -->
  <path d="M12 3 v2" />                          <!-- the hanger -->
</svg>
```

## Designing a consistent icon set

Icons look like a *family* when they share the same rules. Write your rules down:

1. **Grid**: 24 × 24 with **2 units of padding** (draw inside 2..22), so icons never touch the edges.
2. **Stroke**: the same width everywhere (1.5 or 2), the same caps and joins.
3. **Corner radius**: one value (e.g. 2) for rounded corners.
4. **Visual weight**: a circle looks smaller than a square of the same width, so make circles slightly bigger (about 20 units vs 18). Check the set side by side, small.
5. **Simplicity**: at 16 px, details vanish. One idea per icon. Test at the smallest size you'll use.
6. **Metaphors people know**: magnifier = search, gear = settings, trash = delete, house = home. Don't be clever.
7. **Pixel-snapping**: put horizontal and vertical lines on whole or half units (with a 2-unit stroke, on whole numbers) so they render crisp, not blurry.

Look at open-source sets to learn: **Lucide** (successor to Feather), **Heroicons**, **Tabler Icons**, **Phosphor**, **Material Symbols**. They're free to use, and studying their paths is the fastest way to get good.

## Using icons in an app

- **Inline SVG** (in the HTML): can be styled and animated with CSS. Best for UI icons.
- **A sprite**: define each icon once as a `<symbol>`, then reuse it anywhere with `<use>`:

```html
<svg width="0" height="0" style="position:absolute">
  <symbol id="i-star" viewBox="0 0 24 24"><path d="M12 3l2.7 5.6 6.1.9-4.4 4.3 1 6.1L12 17l-5.4 2.9 1-6.1-4.4-4.3 6.1-.9z" /></symbol>
  <symbol id="i-moon" viewBox="0 0 24 24"><path d="M20 14.5A8 8 0 1 1 9.5 4a6.5 6.5 0 0 0 10.5 10.5z" /></symbol>
</svg>
<style>
  .icon { width: 32px; height: 32px; fill: none; stroke: currentColor; stroke-width: 2; stroke-linejoin: round; }
  button { font: 16px system-ui; display: inline-flex; gap: 6px; align-items: center; padding: 8px 14px; border-radius: 10px; border: 1px solid #ddd; background: white; }
</style>
<button style="color:#eab308"><svg class="icon"><use href="#i-star" /></svg> Favourite</button>
<button style="color:#6366f1"><svg class="icon"><use href="#i-moon" /></svg> Dark mode</button>
```

- **`<img src="icon.svg">`**: simple, but CSS can't recolour the inside.
- Icon-only buttons need a name: `<button aria-label="Settings">`. Decorative icons next to text get `aria-hidden="true"`.

Desktop toolkits use SVG too: WPF draws `Path` data with the same syntax (`<Path Data="M8 14 L12 18 L20 6" Stroke="{DynamicResource Accent}" />`), Qt loads SVG with `QIcon("save.svg")`, and tkinter needs PNGs (`tk.PhotoImage(file="save.png")`).

## Animated SVG icons

Every part of an inline SVG can be animated with CSS. The famous **line-drawing** effect uses the dash pattern: make one dash as long as the whole line, offset it so the line is hidden, then animate the offset to 0.

```html
<style>
  body { display: flex; gap: 32px; padding: 24px; }
  .draw path { stroke-dasharray: 30; stroke-dashoffset: 30; animation: draw .6s ease-out .2s forwards; }
  @keyframes draw { to { stroke-dashoffset: 0; } }
  .spin { animation: spin 1s linear infinite; transform-origin: 12px 12px; }
  @keyframes spin { to { transform: rotate(360deg); } }
  .gear:hover .cog { transform: rotate(90deg); }
  .cog { transition: transform .5s cubic-bezier(.34, 1.56, .64, 1); transform-origin: 12px 12px; }
</style>
<svg class="draw" width="72" height="72" viewBox="0 0 24 24" fill="none" stroke="#22c55e" stroke-width="2.5" stroke-linecap="round" stroke-linejoin="round">
  <circle cx="12" cy="12" r="10" stroke-opacity=".25" />
  <path d="M7 12.5l3.5 3.5 6.5-8" />
</svg>
<svg width="72" height="72" viewBox="0 0 24 24" fill="none" stroke="#3b82f6" stroke-width="2.5" stroke-linecap="round">
  <circle cx="12" cy="12" r="9" stroke-opacity=".2" />
  <path class="spin" d="M12 3a9 9 0 0 1 9 9" />
</svg>
<svg class="gear" width="72" height="72" viewBox="0 0 24 24" fill="none" stroke="#64748b" stroke-width="2" stroke-linecap="round">
  <g class="cog">
    <circle cx="12" cy="12" r="3" />
    <path d="M12 2v3M12 19v3M2 12h3M19 12h3M4.9 4.9l2.1 2.1M17 17l2.1 2.1M4.9 19.1L7 17M17 7l2.1-2.1" />
  </g>
</svg>
```

(Hover the gear.) In JavaScript, `path.getTotalLength()` gives the exact length to use for `stroke-dasharray`. For complex icon animations made in After Effects, designers export **Lottie** files (JSON) and apps play them with the Lottie player.

@@quiz
What does `viewBox="0 0 24 24"` do?
- Sets the icon to exactly 24 pixels
* Defines a 24 × 24 unit drawing grid; the icon can still be shown at any size
- Crops the icon to the top-left corner
- Adds 24 pixels of padding
? `width` and `height` set the displayed size; the viewBox is the coordinate system you draw in.

@@quiz
Why use `stroke="currentColor"` in icons?
- It makes them load faster
* The icon takes the text colour of its parent, so CSS `color` recolours it
- It's required by SVG
- It makes the stroke thicker
? One icon file, any colour: in buttons, in dark mode, on hover.

@@quiz
In a path, what's the difference between `L 10 5` and `l 10 5`?
- None
* `L` goes to the absolute point (10, 5); `l` moves 10 right and 5 down from the current point
- `l` draws a thinner line
- `L` is a curve
? Upper case is absolute, lower case is relative, for every path command.

@@quiz
Why should a circle icon be drawn slightly bigger than a square one?
- Circles have fewer pixels
* Circles look smaller than squares of the same width, so they need a bit more size to look balanced
- SVG shrinks circles
- It's not true, they should be identical
? Designers call this optical (visual) balance, not mathematical size.

@@quiz
How does the "line drawing" animation work?
* A dash as long as the line, offset by the same length (hidden), with the offset animated to 0
- Animating the `d` attribute letter by letter
- Animating `width` from 0
- Using a GIF
? `stroke-dasharray: L; stroke-dashoffset: L;` then animate `stroke-dashoffset` to 0.

@@drill
Which SVG attribute defines the coordinate system of an icon?
= viewBox
? viewBox="0 0 24 24".

@@flashcards
SVG :: a text format for vector drawings: sharp at any size, styleable, animatable
`viewBox="0 0 24 24"` :: the drawing grid; the standard size for UI icons
`currentColor` :: use the parent's text colour
outline vs solid icon :: `fill="none"` + stroke / `fill="currentColor"`
`stroke-linecap="round"` :: rounded line ends: a friendly look
`<rect rx>` :: a rectangle with rounded corners
`M` / `L` / `H` / `V` / `Z` :: move / line / horizontal / vertical / close
`C` / `Q` / `A` :: cubic curve / quadratic curve / arc
upper vs lower case path commands :: absolute vs relative
icon grid padding :: draw inside 2..22 on a 24 grid
`<symbol>` + `<use href>` :: an icon sprite: define once, reuse anywhere
`aria-label` / `aria-hidden="true"` :: name an icon-only button / hide a decorative icon
`stroke-dasharray` + `stroke-dashoffset` :: the line-drawing animation
`getTotalLength()` :: the exact length of a path, for dash animations
Lucide, Heroicons, Tabler, Phosphor :: free open-source icon sets to learn from
Lottie :: designer animations exported as JSON and played in apps

@@exercise Draw a home icon
Draw a house icon by hand in a 24 × 24 grid. Use `viewBox="0 0 24 24"`, `fill="none"`, `stroke="currentColor"`, `stroke-width="2"`, `stroke-linecap="round"` and `stroke-linejoin="round"`. Draw the roof and walls with a `<path>` that uses `M`, `L` and closes with `Z`, and the door with a `<rect>` that has rounded corners (`rx`). Show it at `width="96"`, and give the `svg` a colour with `style="color: ..."`.
@@starter
<!doctype html>
<html lang="en">
<head>
  <meta charset="utf-8">
  <title>Home icon</title>
</head>
<body>
  <!-- your svg here -->
</body>
</html>
@@requires
<svg
viewBox="0 0 24 24"
width="96"
fill="none"
stroke="currentColor"
stroke-width="2"
stroke-linecap="round"
stroke-linejoin="round"
style="color:
<path d="M
Z"
<rect
rx="
</svg>
@@hint
A house: `<path d="M3 11 L12 3 L21 11 V20 a1 1 0 0 1 -1 1 H4 a1 1 0 0 1 -1 -1 Z" />` (or with only `L`s: `M3 11 L12 3 L21 11 L21 21 L3 21 Z`), and a door: `<rect x="9.5" y="14" width="5" height="7" rx="1" />`.
@@solution
<!doctype html>
<html lang="en">
<head>
  <meta charset="utf-8">
  <title>Home icon</title>
</head>
<body>
  <svg width="96" height="96" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"
       stroke-linecap="round" stroke-linejoin="round" style="color: #0ea5e9">
    <path d="M3 11 L12 3 L21 11 L21 21 L3 21 Z" />
    <rect x="9.5" y="14" width="5" height="7" rx="1" />
  </svg>
</body>
</html>

@@exercise An icon sprite
Make a mini icon set. Inside one hidden `<svg>`, define two icons as `<symbol>` elements with `id="i-heart"` and `id="i-plus"`, each with `viewBox="0 0 24 24"`. Then show both in buttons with `<use href="#i-heart" />` and `<use href="#i-plus" />`. Style them with a class `.icon` that sets `stroke: currentColor` and `fill: none`, and give the heart button `aria-label="Like"` because it has no text.
@@starter
<!doctype html>
<html lang="en">
<head>
  <meta charset="utf-8">
  <title>Sprite</title>
  <style>
    button { display: inline-flex; gap: 6px; align-items: center; font: 16px system-ui; padding: 8px 12px; border-radius: 10px; border: 1px solid #ddd; background: white; }
  </style>
</head>
<body>
  <button style="color:#e11d48"></button>
  <button style="color:#16a34a">New task</button>
</body>
</html>
@@requires
<symbol id="i-heart" viewBox="0 0 24 24"
<symbol id="i-plus" viewBox="0 0 24 24"
<use href="#i-heart"
<use href="#i-plus"
stroke: currentColor
fill: none
aria-label="Like"
@@hint
`<svg width="0" height="0" style="position:absolute"><symbol id="i-heart" viewBox="0 0 24 24"><path d="M12 20s-7-4.5-7-10a4 4 0 0 1 7-2.6A4 4 0 0 1 19 10c0 5.5-7 10-7 10z" /></symbol><symbol id="i-plus" viewBox="0 0 24 24"><path d="M12 5v14M5 12h14" /></symbol></svg>`, then `<svg class="icon"><use href="#i-heart" /></svg>` inside the button.
@@solution
<!doctype html>
<html lang="en">
<head>
  <meta charset="utf-8">
  <title>Sprite</title>
  <style>
    button { display: inline-flex; gap: 6px; align-items: center; font: 16px system-ui; padding: 8px 12px; border-radius: 10px; border: 1px solid #ddd; background: white; }
    .icon { width: 24px; height: 24px; stroke: currentColor; fill: none; stroke-width: 2; stroke-linecap: round; stroke-linejoin: round; }
  </style>
</head>
<body>
  <svg width="0" height="0" style="position:absolute" aria-hidden="true">
    <symbol id="i-heart" viewBox="0 0 24 24"><path d="M12 20s-7-4.5-7-10a4 4 0 0 1 7-2.6A4 4 0 0 1 19 10c0 5.5-7 10-7 10z" /></symbol>
    <symbol id="i-plus" viewBox="0 0 24 24"><path d="M12 5v14M5 12h14" /></symbol>
  </svg>
  <button style="color:#e11d48" aria-label="Like"><svg class="icon"><use href="#i-heart" /></svg></button>
  <button style="color:#16a34a"><svg class="icon" aria-hidden="true"><use href="#i-plus" /></svg> New task</button>
</body>
</html>

@@exercise A self-drawing check mark
Animate a check mark so it draws itself. Give the check `<path>` a class `tick`, set `stroke-dasharray: 24` and `stroke-dashoffset: 24` on it, and write `@keyframes draw` that animates `stroke-dashoffset` to `0`. Play it with `animation: draw` and `forwards` so it stays drawn. Respect `prefers-reduced-motion: reduce` by turning the animation off and setting the offset to 0.
@@starter
<!doctype html>
<html lang="en">
<head>
  <meta charset="utf-8">
  <title>Done!</title>
  <style>
    body { font-family: system-ui, sans-serif; padding: 32px; display: flex; align-items: center; gap: 12px; }
  </style>
</head>
<body>
  <svg width="64" height="64" viewBox="0 0 24 24" fill="none" stroke="#22c55e" stroke-width="2.5" stroke-linecap="round" stroke-linejoin="round">
    <circle cx="12" cy="12" r="10" stroke-opacity=".3" />
    <path d="M7 12.5l3.5 3.5 6.5-8" />
  </svg>
  <strong>Backup complete</strong>
</body>
</html>
@@requires
class="tick"
stroke-dasharray: 24
stroke-dashoffset: 24
@keyframes draw
stroke-dashoffset: 0
animation: draw
forwards
@media (prefers-reduced-motion: reduce)
@@hint
`.tick { stroke-dasharray: 24; stroke-dashoffset: 24; animation: draw .6s ease-out .2s forwards; }` and `@keyframes draw { to { stroke-dashoffset: 0; } }`. Reduced motion: `@media (prefers-reduced-motion: reduce) { .tick { animation: none; stroke-dashoffset: 0; } }`.
@@solution
<!doctype html>
<html lang="en">
<head>
  <meta charset="utf-8">
  <title>Done!</title>
  <style>
    body { font-family: system-ui, sans-serif; padding: 32px; display: flex; align-items: center; gap: 12px; }
    .tick { stroke-dasharray: 24; stroke-dashoffset: 24; animation: draw .6s ease-out .2s forwards; }
    @keyframes draw { to { stroke-dashoffset: 0; } }
    @media (prefers-reduced-motion: reduce) {
      .tick { animation: none; stroke-dashoffset: 0; }
    }
  </style>
</head>
<body>
  <svg width="64" height="64" viewBox="0 0 24 24" fill="none" stroke="#22c55e" stroke-width="2.5" stroke-linecap="round" stroke-linejoin="round">
    <circle cx="12" cy="12" r="10" stroke-opacity=".3" />
    <path class="tick" d="M7 12.5l3.5 3.5 6.5-8" />
  </svg>
  <strong>Backup complete</strong>
</body>
</html>
