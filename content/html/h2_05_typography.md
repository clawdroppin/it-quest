@@lesson h2-type Units, Typography, Colour and Backgrounds
@@text
Most of a web page is text. Good typography (readable size, comfortable line length, enough contrast) does more for a page than any decoration. This lesson covers the units, font and colour tools that make it happen.

## Units

| Unit | Relative to | Use for |
|---|---|---|
| `px` | a CSS pixel (not a screen pixel) | borders, shadows, small fixed details |
| `rem` | the root font size (16px by default) | font sizes, spacing: respects the user's browser setting |
| `em` | the element's own font size | padding that scales with the text (buttons), media queries |
| `%` | the parent (for widths) | fluid widths |
| `ch` | the width of "0" in the font | line length: `max-width: 65ch` |
| `vw`, `vh` | 1% of the viewport width / height | full-screen sections |
| `dvh`, `svh`, `lvh` | dynamic / small / large viewport height | mobile, where browser bars come and go |
| `fr` | a share of free space in a grid | grid columns |

Users who need bigger text change their browser's default font size. `rem` and `em` respect that; `px` font sizes ignore it. Use `rem` for type.

## Fluid sizes with clamp()

`clamp(min, preferred, max)` picks the preferred value, kept between the limits:

```html
<style>
  h1 { font-size: clamp(1.75rem, 1rem + 3vw, 3rem); }
  .wrap { width: min(100% - 2rem, 60rem); margin-inline: auto; }
</style>
<div class="wrap"><h1>Resize the window: this heading grows and shrinks</h1></div>
```

`min()` and `max()` work the same way: `width: min(100% - 2rem, 60rem)` is "full width minus margins, but never more than 60rem", the whole responsive container in one line.

## Readable text

- **Size**: 1rem (16px) body text minimum; larger on reading-heavy pages.
- **Line height**: 1.5–1.7 for body text (unitless: `line-height: 1.6`, so it scales with the font size), tighter (1.1–1.3) for big headings.
- **Line length**: 45–75 characters: `max-width: 65ch` on paragraphs or the content column.
- **Contrast**: at least 4.5:1 for normal text and 3:1 for large text (WCAG AA). Light grey on white fails more often than you'd think.

## Fonts

```css
body { font-family: system-ui, -apple-system, "Segoe UI", Roboto, "Hiragino Sans", "Noto Sans JP", sans-serif; }
code { font-family: ui-monospace, "Cascadia Code", Consolas, monospace; }
```

The **system font stack** is free, fast and looks native. For a brand font, use `@font-face` with WOFF2 files you host yourself, and `font-display: swap` so text appears immediately in a fallback font. Load only the weights you use; every weight is a separate download. Japanese fonts are huge (thousands of glyphs), which is why sites usually rely on system Japanese fonts or subset web fonts.

Variable fonts put all weights (and widths) in one file: `font-weight: 350` anything between 100 and 900.

## Text properties worth knowing

- `text-wrap: balance` evens out the lines of headings; `text-wrap: pretty` avoids single orphan words at the end of paragraphs.
- `text-overflow: ellipsis` with `overflow: hidden; white-space: nowrap;` cuts long text with "…".
- `overflow-wrap: anywhere` breaks long URLs and file paths instead of overflowing.
- `font-variant-numeric: tabular-nums` makes digits the same width: numbers in tables line up.
- `letter-spacing` for small uppercase labels; `text-transform: uppercase` instead of typing capitals.
- `hyphens: auto` with a correct `lang` attribute.

## Colour

```css
color: #1d4ed8;                     /* hex */
color: rgb(29 78 216 / 0.8);        /* with transparency */
color: hsl(221 83% 48%);            /* hue, saturation, lightness: easy to tweak */
color: oklch(0.5 0.2 264);          /* perceptual: equal steps of L look equally different */
color: color-mix(in oklch, var(--brand), white 30%);   /* a lighter tint */
```

`hsl()` is great for making a palette by hand: keep the hue, change the lightness. `oklch()` is better still: equal lightness values look equally light across hues, which keeps contrast predictable. `currentColor` means "this element's text colour": great for borders and SVG icons that follow the text.

## Backgrounds and gradients

```html
<style>
  .banner {
    padding: 2rem; color: white; border-radius: 12px;
    background:
      linear-gradient(135deg, rgb(225 29 72 / 0.9), rgb(124 58 237 / 0.9)),
      repeating-linear-gradient(45deg, #000 0 10px, #222 10px 20px);
  }
</style>
<div class="banner"><strong>Maintenance window:</strong> Saturday 22:00–24:00</div>
```

Layers are listed top first. `background-size: cover` fills a box with an image; `background-position` picks the focus. Text over images needs a darkening gradient layer for contrast.

@@quiz
Why use `rem` instead of `px` for font sizes?
* `rem` scales with the user's browser font-size setting; px font sizes ignore it
- `px` doesn't work on phones
- `rem` is always larger
- `px` is deprecated
? Respecting user settings is an accessibility requirement.

@@quiz
What does `width: min(100% - 2rem, 60rem)` do?
- Always 60rem
* Takes the full width minus 2rem, but never more than 60rem
- Takes the larger value
- It's invalid CSS
? A responsive container in one line.

@@quiz
What's a comfortable line length for body text?
- 20–30 characters
* About 45–75 characters (e.g. `max-width: 65ch`)
- 120–150 characters
- The full screen width
? Long lines make it hard to find the next line.

@@quiz
What's the WCAG AA minimum contrast for normal body text?
- 2:1
- 3:1
* 4.5:1
- 7:1
? Large text needs 3:1; AAA asks for 7:1.

@@drill
Which CSS function sets a value with a minimum, a preferred size and a maximum?
= clamp()
= clamp
? font-size: clamp(1rem, 2.5vw, 1.5rem).

@@flashcards
`rem` :: relative to the root font size
`em` :: relative to the element's font size
`ch` :: width of "0": line lengths
`dvh` :: dynamic viewport height (mobile)
`clamp(min, pref, max)` :: fluid value with limits
unitless line-height :: 1.5–1.7 for body text
`max-width: 65ch` :: readable line length
system font stack :: native, fast fonts
`font-display: swap` :: show fallback text while web fonts load
`text-wrap: balance` :: even heading lines
`tabular-nums` :: aligned digits
`oklch()` :: perceptual colour space
`currentColor` :: the text colour
contrast AA :: 4.5:1 normal, 3:1 large text

@@exercise Make the article readable
The policy page is hard to read. Fix the typography: body text `1.125rem` with a unitless `line-height: 1.6`, the content column limited to `65ch` and centred with `margin-inline: auto`, a fluid heading with `clamp(1.75rem, 1rem + 3vw, 3rem)` and `text-wrap: balance`, the system font stack starting with `system-ui`, and the light grey text darkened to `#334155` (the old `#aaa` fails contrast).
@@starter
<!doctype html>
<html lang="en">
<head><meta charset="utf-8"><title>Password policy</title>
<style>
  body { font-family: "Times New Roman"; font-size: 13px; color: #aaa; line-height: 14px; }
  h1 { font-size: 40px; }
</style>
</head>
<body>
  <main>
    <h1>Password policy for all Sakura Systems staff and contractors</h1>
    <p>Passwords must be at least 14 characters long. Use a passphrase of several unrelated words, and never reuse a password from another service. Store passwords in the company password manager, not in documents or browser notes.</p>
    <p>Multi-factor authentication is required for email, VPN and every cloud service. If you lose your phone, contact the help desk immediately so we can revoke the old device.</p>
  </main>
</body>
</html>
@@requires
font-size: 1.125rem
line-height: 1.6
max-width: 65ch
margin-inline: auto
clamp(1.75rem, 1rem + 3vw, 3rem)
text-wrap: balance
system-ui
#334155
@@hint
`body { font-family: system-ui, "Segoe UI", sans-serif; font-size: 1.125rem; line-height: 1.6; color: #334155; } main { max-width: 65ch; margin-inline: auto; padding: 1rem; } h1 { font-size: clamp(1.75rem, 1rem + 3vw, 3rem); line-height: 1.2; text-wrap: balance; }`
@@solution
<!doctype html>
<html lang="en">
<head><meta charset="utf-8"><title>Password policy</title>
<style>
  body { font-family: system-ui, "Segoe UI", Roboto, "Hiragino Sans", sans-serif; font-size: 1.125rem; color: #334155; line-height: 1.6; }
  main { max-width: 65ch; margin-inline: auto; padding: 1rem; }
  h1 { font-size: clamp(1.75rem, 1rem + 3vw, 3rem); line-height: 1.2; text-wrap: balance; color: #0f172a; }
</style>
</head>
<body>
  <main>
    <h1>Password policy for all Sakura Systems staff and contractors</h1>
    <p>Passwords must be at least 14 characters long. Use a passphrase of several unrelated words, and never reuse a password from another service. Store passwords in the company password manager, not in documents or browser notes.</p>
    <p>Multi-factor authentication is required for email, VPN and every cloud service. If you lose your phone, contact the help desk immediately so we can revoke the old device.</p>
  </main>
</body>
</html>

@@exercise A numbers table that lines up
Make the cost table readable: digits with `font-variant-numeric: tabular-nums`, numbers right-aligned, long file paths in the first column wrapped with `overflow-wrap: anywhere`, and the long description cell cut to one line with an ellipsis (`white-space: nowrap; overflow: hidden; text-overflow: ellipsis;` plus a `max-width`).
@@starter
<!doctype html>
<html lang="en">
<head><meta charset="utf-8"><title>Storage costs</title>
<style>
  table { border-collapse: collapse; width: 100%; max-width: 40rem; }
  td, th { border-bottom: 1px solid #e2e8f0; padding: 0.4rem; }
</style>
</head>
<body>
  <table>
    <thead><tr><th>Share</th><th>Description</th><th>GB</th><th>Yen / month</th></tr></thead>
    <tbody>
      <tr><td class="path">\\fs01\departments\finance\archive\2019-2024</td><td class="desc">Closed fiscal years kept for the tax office audit requirement</td><td class="num">1,204</td><td class="num">12,040</td></tr>
      <tr><td class="path">\\fs01\it\images</td><td class="desc">Windows deployment images</td><td class="num">311</td><td class="num">3,110</td></tr>
      <tr><td class="path">\\fs02\sales</td><td class="desc">Sales team share</td><td class="num">88</td><td class="num">880</td></tr>
    </tbody>
  </table>
</body>
</html>
@@requires
tabular-nums
text-align: right
overflow-wrap: anywhere
text-overflow: ellipsis
white-space: nowrap
@@hint
`.num { text-align: right; font-variant-numeric: tabular-nums; } .path { overflow-wrap: anywhere; } .desc { max-width: 12rem; white-space: nowrap; overflow: hidden; text-overflow: ellipsis; }`
@@solution
<!doctype html>
<html lang="en">
<head><meta charset="utf-8"><title>Storage costs</title>
<style>
  table { border-collapse: collapse; width: 100%; max-width: 40rem; }
  td, th { border-bottom: 1px solid #e2e8f0; padding: 0.4rem; }
  .num { text-align: right; font-variant-numeric: tabular-nums; }
  .path { overflow-wrap: anywhere; }
  .desc { max-width: 12rem; white-space: nowrap; overflow: hidden; text-overflow: ellipsis; }
</style>
</head>
<body>
  <table>
    <thead><tr><th>Share</th><th>Description</th><th>GB</th><th>Yen / month</th></tr></thead>
    <tbody>
      <tr><td class="path">\\fs01\departments\finance\archive\2019-2024</td><td class="desc">Closed fiscal years kept for the tax office audit requirement</td><td class="num">1,204</td><td class="num">12,040</td></tr>
      <tr><td class="path">\\fs01\it\images</td><td class="desc">Windows deployment images</td><td class="num">311</td><td class="num">3,110</td></tr>
      <tr><td class="path">\\fs02\sales</td><td class="desc">Sales team share</td><td class="num">88</td><td class="num">880</td></tr>
    </tbody>
  </table>
</body>
</html>

@@exercise A palette from one hue
Build a status palette with `hsl()` custom properties from one brand hue: `--hue: 350`, then `--brand: hsl(var(--hue) 80% 45%)`, a light tint `--brand-soft: hsl(var(--hue) 80% 95%)` and a dark shade `--brand-strong: hsl(var(--hue) 80% 25%)`. Use them for the alert (soft background, strong text, brand border). Add a hero banner with a `linear-gradient` from `--brand` to `--brand-strong` and white text.
@@starter
<!doctype html>
<html lang="en">
<head><meta charset="utf-8"><title>Palette</title>
<style>
  .alert { padding: 1rem; border-left: 4px solid red; }
  .hero { padding: 2rem; }
</style>
</head>
<body>
  <div class="hero"><h1>IT Service Portal</h1></div>
  <div class="alert">Planned maintenance on Saturday 22:00–24:00.</div>
</body>
</html>
@@requires
--hue: 350
hsl(var(--hue) 80% 45%)
var(--brand-soft)
var(--brand-strong)
linear-gradient(
@@hint
`:root { --hue: 350; --brand: hsl(var(--hue) 80% 45%); --brand-soft: hsl(var(--hue) 80% 95%); --brand-strong: hsl(var(--hue) 80% 25%); } .alert { background: var(--brand-soft); color: var(--brand-strong); border-left: 4px solid var(--brand); } .hero { background: linear-gradient(135deg, var(--brand), var(--brand-strong)); color: white; }`
@@solution
<!doctype html>
<html lang="en">
<head><meta charset="utf-8"><title>Palette</title>
<style>
  :root {
    --hue: 350;
    --brand: hsl(var(--hue) 80% 45%);
    --brand-soft: hsl(var(--hue) 80% 95%);
    --brand-strong: hsl(var(--hue) 80% 25%);
  }
  .alert { padding: 1rem; border-left: 4px solid var(--brand); background: var(--brand-soft); color: var(--brand-strong); }
  .hero { padding: 2rem; color: white; background: linear-gradient(135deg, var(--brand), var(--brand-strong)); border-radius: 12px; }
</style>
</head>
<body>
  <div class="hero"><h1>IT Service Portal</h1></div>
  <div class="alert">Planned maintenance on Saturday 22:00–24:00.</div>
</body>
</html>
