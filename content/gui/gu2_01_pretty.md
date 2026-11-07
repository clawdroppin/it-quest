@@lesson gu2-pretty Making It Pretty: Colour, Spacing, Type and Themes
@@concepts variables
@@text
Most "ugly" interfaces aren't missing decoration. They're inconsistent: twelve slightly different greys, random gaps, five font sizes, buttons that don't line up. Good-looking GUIs come from a few **rules applied everywhere**, a small **design system**. This lesson uses CSS because the preview shows results instantly, but every idea works in WPF, Qt, tkinter and Figma too.

## Design tokens: decide once, reuse everywhere

A **design token** is a named design decision: `--accent`, `--space-3`, `--radius`. Put them in one place and use them everywhere, and changing the whole look (or adding a dark theme) becomes a one-minute job.

```html
<style>
  :root {
    --bg: #fdfcff;  --surface: #ffffff;  --text: #1e1b2e;  --muted: #6b6880;
    --accent: #7c3aed;  --accent-text: #ffffff;  --border: #e7e3f3;
    --space-1: 4px; --space-2: 8px; --space-3: 16px; --space-4: 24px; --space-5: 40px;
    --radius: 12px;
    --shadow: 0 1px 2px rgb(30 27 46 / .06), 0 8px 24px rgb(30 27 46 / .08);
  }
  body { background: var(--bg); color: var(--text); font-family: system-ui, sans-serif; padding: var(--space-4); }
  .card { background: var(--surface); border: 1px solid var(--border); border-radius: var(--radius);
          box-shadow: var(--shadow); padding: var(--space-4); max-width: 320px; }
  .card h2 { margin: 0 0 var(--space-2); font-size: 1.25rem; }
  .card p { margin: 0 0 var(--space-3); color: var(--muted); line-height: 1.5; }
  .btn { background: var(--accent); color: var(--accent-text); border: 0; border-radius: 999px;
         padding: var(--space-2) var(--space-3); font: inherit; font-weight: 600; }
</style>
<div class="card">
  <h2>Backup finished</h2>
  <p>214 files copied to the NAS in 3 minutes.</p>
  <button class="btn">View report</button>
</div>
```

## Spacing: use a scale

Pick a short **spacing scale** (4, 8, 16, 24, 40, 64) and use *only* those values. Things that belong together get small gaps; separate groups get big ones. This is the **law of proximity**: people see things that are close as related.

- Inside a button: 8 × 16. Between a label and its input: 4 to 8. Between form fields: 16. Between sections: 40+.
- Align everything to a few invisible lines. Ragged left edges look messy even when nobody can say why.
- **Whitespace is not wasted space.** Cramped screens feel stressful; give things room.

## Colour: fewer colours, used on purpose

- **One accent colour** for the main action and the current selection. If everything is purple, nothing stands out.
- **Neutrals** (greys, ideally slightly tinted towards your accent) for text, borders and backgrounds: 3 to 5 shades is plenty.
- **Semantic colours** for meaning only: green success, amber warning, red danger, blue info. Never rely on colour alone: add an icon or text too, because about 1 in 12 men is colour-blind.
- **Contrast**: body text needs a contrast ratio of at least **4.5 : 1** against its background (WCAG AA), large text and icons 3 : 1. Light grey on white looks elegant and is unreadable for many people. Browser dev tools show the ratio when you inspect a colour.
- **HSL** makes colour scales easy: `hsl(265 83% 58%)` is hue, saturation, lightness. Keep the hue, change the lightness, and you get a family of tints that match. Modern CSS also has `oklch()`, where equal lightness steps *look* equal.

## Typography

- One font family is usually enough (two at most: one for headings, one for text). `system-ui` uses each OS's own font and loads instantly; Inter, Nunito (round and friendly) and Poppins are popular free choices.
- A **type scale**: e.g. 12, 14, 16, 20, 24, 32. Body text 16px, line height around 1.5, lines at most about 70 characters wide (`max-width: 65ch`).
- Create hierarchy with **weight and colour** before size: a 600-weight title and a muted subtitle beat three different font sizes.
- Numbers that change (timers, prices, tables) look steadier with `font-variant-numeric: tabular-nums`.

## Depth and shape

- **Border radius** sets the personality: 4px feels serious and technical, 12 to 16px friendly, fully round (`999px`) playful. Pick one or two values.
- **Shadows** should be soft and layered (a small sharp one plus a large blurred one), with low opacity, and tinted, never pure black.
- Prefer **one** of border or shadow on a card, not heavy versions of both.

## Light and dark themes

With tokens, a dark theme is just new values for the same names:

```html
<style>
  :root { --bg: #f8f7fc; --surface: #fff; --text: #1e1b2e; --muted: #6b6880; --accent: #7c3aed; }
  @media (prefers-color-scheme: dark) {
    :root { --bg: #14121c; --surface: #1e1b2b; --text: #ece9f6; --muted: #a29fb5; --accent: #a78bfa; }
  }
  :root[data-theme="light"] { --bg: #f8f7fc; --surface: #fff; --text: #1e1b2e; --muted: #6b6880; --accent: #7c3aed; }
  :root[data-theme="dark"]  { --bg: #14121c; --surface: #1e1b2b; --text: #ece9f6; --muted: #a29fb5; --accent: #a78bfa; }
  body { background: var(--bg); color: var(--text); font-family: system-ui, sans-serif; padding: 24px; }
  .box { background: var(--surface); padding: 16px; border-radius: 12px; }
  .box small { color: var(--muted); }
  b { color: var(--accent); }
</style>
<div class="box">Theme follows your system. <b>Accent</b> <small>muted text</small></div>
```

`prefers-color-scheme` follows the operating system; the `data-theme` attribute (set by a toggle with `document.documentElement.dataset.theme = "dark"`) lets the user override it. Dark themes are **not** just inverted: use dark greys instead of pure black, slightly lighter surfaces for raised things, softer (less saturated, lighter) accents, and re-check contrast. Add `color-scheme: light dark;` so scrollbars and form controls match.

The same idea on the desktop: WPF puts colours in a `ResourceDictionary` and swaps it (`{DynamicResource Accent}`); Qt swaps a QSS file; tkinter's `ttk.Style().configure(...)` sets them per widget style.

## A pretty-checklist

1. Everything uses the spacing scale and the type scale.
2. One accent colour; neutrals for the rest.
3. Text contrast is at least 4.5 : 1.
4. Edges line up.
5. Similar things look the same, different things look clearly different.
6. Then, and only then, add delight: animation, illustration, icons.

@@quiz
What is a design token?
- A login token for a design tool
* A named design decision (colour, spacing, radius) defined once and reused everywhere
- A random CSS class
- An image file
? Tokens like `--accent` and `--space-3` make the look consistent and easy to change or theme.

@@quiz
What minimum contrast ratio does WCAG AA require for normal body text?
- 1.5 : 1
- 3 : 1
* 4.5 : 1
- 21 : 1
? Large text and icons need 3 : 1. 21 : 1 is black on white, the maximum.

@@quiz
Why use only one accent colour for actions?
- CSS only allows one colour variable
* So the main action and the current selection stand out; if everything is colourful, nothing is
- Accent colours are expensive
- Screens can't show two bright colours
? Neutrals for most things, the accent for what matters, semantic colours for meaning.

@@quiz
Which is the better dark theme background?
- Pure black `#000000` with pure white text
* A dark grey like `#14121c`, with slightly lighter surfaces for raised elements
- The light theme inverted
- Dark blue with dark purple text
? Pure black and white is harsh, and lighter surfaces show depth in the dark.

@@quiz
Why should a status never be shown by colour alone?
- Colours don't print
* Many people are colour-blind, so pair the colour with an icon or text
- Green is hard to see on screens
- It's slower
? "✓ Running" in green and "✕ Stopped" in red work for everyone.

@@quiz
Why use a spacing scale such as 4, 8, 12, 16, 24, 32 instead of any value that looks right?
* Consistent steps make layouts look deliberate and keep designers and code in agreement
- Browsers only support those values
- It makes pages load faster
- Screen readers need it
? A small set of tokens removes hundreds of one-off decisions.

@@drill
What do you call named values like `--space-2` or `--brand` that a design reuses everywhere?
= design tokens
= tokens
= design token
? Decide once, reuse everywhere.

@@flashcards
design token :: a named design decision: `--accent`, `--space-3`, `--radius`
spacing scale :: a short list of allowed gaps, e.g. 4, 8, 16, 24, 40
law of proximity :: things close together look related
accent colour :: one colour for the main action and selection
semantic colours :: success, warning, danger, info: meaning, not decoration
WCAG AA contrast :: 4.5 : 1 for text, 3 : 1 for large text and icons
`hsl(h s l)` :: hue, saturation, lightness: easy matching tints
`oklch()` :: a colour space where equal lightness steps look equal
type scale :: a short list of font sizes, e.g. 12, 14, 16, 20, 24, 32
`max-width: 65ch` :: keeps lines readable
`tabular-nums` :: digits of equal width for timers and tables
`prefers-color-scheme: dark` :: the OS is in dark mode
`[data-theme="dark"]` :: a user override for the theme
`color-scheme: light dark` :: native controls and scrollbars follow the theme

@@exercise Tokens for a profile card
The card below has hard-coded values everywhere. Define these tokens on `:root`: `--accent`, `--text`, `--muted`, `--surface`, `--radius`, `--space-2` (8px), `--space-3` (16px) and `--space-4` (24px). Then use them: the card uses `var(--surface)`, `var(--radius)` and `var(--space-4)` for its padding; the name uses `var(--text)`; the role uses `var(--muted)`; the button uses `var(--accent)` and padding `var(--space-2) var(--space-3)`.
@@starter
<!doctype html>
<html lang="en">
<head>
  <meta charset="utf-8">
  <title>Profile</title>
  <style>
    body { background: #f4f2fa; font-family: system-ui, sans-serif; padding: 40px; }
    .card { background: #ffffff; border-radius: 16px; padding: 24px; max-width: 260px; text-align: center;
            box-shadow: 0 8px 24px rgb(30 27 46 / .1); }
    .avatar { width: 72px; height: 72px; border-radius: 50%; background: #ddd6fe; margin: 0 auto 16px;
              display: grid; place-items: center; font-size: 32px; }
    .name { color: #1e1b2e; margin: 0; }
    .role { color: #6b6880; margin: 4px 0 16px; }
    .btn { background: #7c3aed; color: white; border: 0; border-radius: 999px; padding: 8px 16px; font: inherit; }
  </style>
</head>
<body>
  <div class="card">
    <div class="avatar">🦊</div>
    <h2 class="name">Mika Sato</h2>
    <p class="role">Help desk · Tokyo</p>
    <button class="btn">Send message</button>
  </div>
</body>
</html>
@@requires
:root
--accent:
--text:
--muted:
--surface:
--radius:
--space-2: 8px
--space-3: 16px
--space-4: 24px
background: var(--surface)
border-radius: var(--radius)
padding: var(--space-4)
color: var(--text)
color: var(--muted)
background: var(--accent)
padding: var(--space-2) var(--space-3)
@@hint
Start with `:root { --accent: #7c3aed; --text: #1e1b2e; --muted: #6b6880; --surface: #ffffff; --radius: 16px; --space-2: 8px; --space-3: 16px; --space-4: 24px; }` and then replace each hard-coded value with its `var(...)`.
@@solution
<!doctype html>
<html lang="en">
<head>
  <meta charset="utf-8">
  <title>Profile</title>
  <style>
    :root {
      --accent: #7c3aed; --text: #1e1b2e; --muted: #6b6880; --surface: #ffffff;
      --radius: 16px; --space-2: 8px; --space-3: 16px; --space-4: 24px;
    }
    body { background: #f4f2fa; font-family: system-ui, sans-serif; padding: 40px; }
    .card { background: var(--surface); border-radius: var(--radius); padding: var(--space-4); max-width: 260px; text-align: center;
            box-shadow: 0 8px 24px rgb(30 27 46 / .1); }
    .avatar { width: 72px; height: 72px; border-radius: 50%; background: #ddd6fe; margin: 0 auto var(--space-3);
              display: grid; place-items: center; font-size: 32px; }
    .name { color: var(--text); margin: 0; }
    .role { color: var(--muted); margin: 4px 0 var(--space-3); }
    .btn { background: var(--accent); color: white; border: 0; border-radius: 999px; padding: var(--space-2) var(--space-3); font: inherit; }
  </style>
</head>
<body>
  <div class="card">
    <div class="avatar">🦊</div>
    <h2 class="name">Mika Sato</h2>
    <p class="role">Help desk · Tokyo</p>
    <button class="btn">Send message</button>
  </div>
</body>
</html>

@@exercise Light, dark and a manual override
Give this page a dark theme. Keep the light values on `:root`. Add a `@media (prefers-color-scheme: dark)` block that sets dark values for `--bg`, `--surface`, `--text` and `--accent` on `:root`, and a `:root[data-theme="dark"]` rule with the same dark values so a toggle can force it. Add `color-scheme: light dark;` on `:root` too. (Try it: add `data-theme="dark"` to the `<html>` tag.)
@@starter
<!doctype html>
<html lang="en">
<head>
  <meta charset="utf-8">
  <title>Themes</title>
  <style>
    :root { --bg: #f8f7fc; --surface: #ffffff; --text: #1e1b2e; --accent: #7c3aed; }
    body { background: var(--bg); color: var(--text); font-family: system-ui, sans-serif; padding: 32px; }
    .panel { background: var(--surface); border-radius: 14px; padding: 20px; max-width: 320px; }
    a { color: var(--accent); }
  </style>
</head>
<body>
  <div class="panel">
    <h2>Good evening 🌙</h2>
    <p>3 tickets are waiting for you. <a href="#">Open queue</a></p>
  </div>
</body>
</html>
@@requires
color-scheme: light dark
@media (prefers-color-scheme: dark)
:root[data-theme="dark"]
@@hint
`@media (prefers-color-scheme: dark) { :root { --bg: #14121c; --surface: #1e1b2b; --text: #ece9f6; --accent: #a78bfa; } }` and then `:root[data-theme="dark"] { ...the same four... }`.
@@solution
<!doctype html>
<html lang="en">
<head>
  <meta charset="utf-8">
  <title>Themes</title>
  <style>
    :root { --bg: #f8f7fc; --surface: #ffffff; --text: #1e1b2e; --accent: #7c3aed; color-scheme: light dark; }
    @media (prefers-color-scheme: dark) {
      :root { --bg: #14121c; --surface: #1e1b2b; --text: #ece9f6; --accent: #a78bfa; }
    }
    :root[data-theme="dark"] { --bg: #14121c; --surface: #1e1b2b; --text: #ece9f6; --accent: #a78bfa; }
    body { background: var(--bg); color: var(--text); font-family: system-ui, sans-serif; padding: 32px; }
    .panel { background: var(--surface); border-radius: 14px; padding: 20px; max-width: 320px; }
    a { color: var(--accent); }
  </style>
</head>
<body>
  <div class="panel">
    <h2>Good evening 🌙</h2>
    <p>3 tickets are waiting for you. <a href="#">Open queue</a></p>
  </div>
</body>
</html>
