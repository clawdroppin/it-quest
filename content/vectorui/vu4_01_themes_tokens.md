@@lesson vu4-themes App Themes and Design Systems: Tokens from Penpot to Code
@@concepts hello, json
@@lang html
@@text
A **design system** is the shared language between design and code: tokens (colours, spacing, type, radius, shadows, motion), components built from them, and rules for using them. Done well, a whole app can switch from light to dark, or be rebranded, by changing a few values. This lesson builds that pipeline: tokens designed in Penpot, exported as JSON, turned into CSS variables and Rust constants, and switched live.

## Three layers of tokens

| Layer | Example | Meaning |
|---|---|---|
| **Primitive** (raw palette) | `indigo.500 = #6366f1`, `slate.900 = #0f172a`, `space.4 = 16px` | what exists |
| **Semantic** (role) | `color.accent = {indigo.500}`, `color.bg = {slate.50}`, `space.card = {space.6}` | what it's for |
| **Component** | `button.primary.bg = {color.accent}`, `button.radius = {radius.md}` | where it's used |

Themes change only the **semantic** layer: in the dark set, `color.bg = {slate.900}` and `color.accent = {indigo.400}`. Components never mention raw colours, so they follow automatically.

## Tokens in Penpot

1. Open the **Tokens** panel. Create a set `primitives` with the palette and scales.
2. Create sets `light` and `dark` with the semantic tokens referencing primitives (`{indigo.500}`).
3. Create **themes** (e.g. group *Mode* with themes *Light* and *Dark*), each enabling `primitives` plus one mode set. Add a *Brand* group if you have several brands.
4. Apply tokens to shapes (right-click a token → apply to fill, stroke, radius, gap, padding, font size...). Switching the active theme repaints the whole design.
5. **Export** the tokens as JSON (W3C Design Tokens format), or sync them to a Git repository with a plugin, so code uses the same values.

```text
{
  "color": {
    "bg":     { "$type": "color", "$value": "{slate.50}" },
    "accent": { "$type": "color", "$value": "{indigo.500}" }
  },
  "radius": { "md": { "$type": "borderRadius", "$value": "10" } }
}
```

## Tokens to code

**Style Dictionary** (or Tokens Studio's transformers, or a small script) reads the JSON and writes platform files:

- **CSS custom properties** for the web and Tauri: `--color-bg: #f8fafc;`
- **Rust constants** for egui/iced/Slint: `pub const ACCENT: Color32 = Color32::from_rgb(0x63, 0x66, 0xf1);` (the Rust UI course's `Theme` struct)
- Slint globals, Android XML, iOS Swift, Tailwind config...

The exercises use CSS variables with a theme attribute, the most common way to switch themes in a web or Tauri app:

```text
:root, [data-theme="light"] { --bg: #f8fafc; --text: #0f172a; --accent: #6366f1; }
[data-theme="dark"]          { --bg: #0f172a; --text: #e2e8f0; --accent: #818cf8; }
@media (prefers-color-scheme: dark) { :root:not([data-theme]) { /* dark values */ } }
```

## Making a theme pretty

- Pick an **accent** with personality and test it on both backgrounds (contrast ≥ 4.5:1 for text, ≥ 3:1 for buttons and focus rings).
- Build **neutrals with a hint of the accent's hue** (blue-greys for an indigo accent, warm greys for an orange one): it makes the whole UI feel designed.
- Add **semantic colours** (success, warning, danger, info) with both a strong and a subtle (background) version.
- **Elevation**: light theme uses shadows; dark theme uses lighter surfaces (surface 1, 2, 3).
- **Motion tokens**: `motion.fast = 120ms`, `motion.normal = 200ms`, `easing.out = cubic-bezier(.2, .8, .2, 1)`, used by every transition.
- Optional delights: a subtle background gradient or noise texture, a frosted-glass panel (`backdrop-filter: blur(12px)`), a coloured shadow under the primary button (`box-shadow` in the accent's hue). Use them in one or two places, not everywhere.

## Documenting the system

Keep a Penpot page (or a small web page) showing: palette swatches with token names, type scale, spacing scale, each component with all variants and states, do/don't examples, and the version and changelog. A design system people can't find doesn't get used.

@@quiz
Which token layer should a dark theme change?
- Primitive (the raw palette)
* Semantic (roles like color.bg and color.accent)
- Component tokens only
- None; dark mode inverts colours automatically
? Components reference semantic tokens, so they follow.

@@quiz
What does `{indigo.500}` mean as a token value?
* A reference to another token, so the value updates if indigo.500 changes
- A JavaScript template
- A CSS selector
- A comment
? References build the semantic and component layers.

@@quiz
What turns Penpot's exported token JSON into CSS variables and Rust constants?
- Inkscape
* A transformer such as Style Dictionary (or a small script)
- The browser automatically
- Penpot's prototype mode
? One source, many platforms.

@@quiz
In dark themes, how is elevation usually shown?
- With darker, stronger shadows
* With progressively lighter surfaces for higher layers
- With thicker borders only
- It isn't shown
? Shadows are hard to see on dark backgrounds.

@@quiz
Why should components reference semantic tokens such as `color.surface` rather than primitives like `gray.50`?
* Themes can then remap the semantic token to a different primitive without touching any component
- Primitive tokens can't be exported
- Semantic tokens render faster
- Penpot only supports semantic tokens
? The semantic layer is the switch a dark or high-contrast theme flips.

@@drill
What do you call tokens like `color.button.background` that describe a role rather than a raw value?
= semantic tokens
= semantic
= component tokens
? They point at primitive tokens like blue-500.

@@flashcards
design system :: tokens, components and rules shared by design and code
primitive token :: a raw value (indigo.500)
semantic token :: a role (color.accent)
component token :: a usage (button.primary.bg)
token reference :: {other.token} as a value
token set :: a group of tokens (light, dark, brand)
theme :: a combination of enabled token sets
W3C Design Tokens :: JSON format with $type and $value
Style Dictionary :: generates CSS, Rust, Swift... from tokens
data-theme attribute :: switch CSS variable sets
prefers-color-scheme :: the system's light or dark preference
motion tokens :: shared durations and easings

@@exercise A theme switcher
Define colour tokens as CSS custom properties: `--bg`, `--surface`, `--text` and `--accent` under `[data-theme="light"]` and different values under `[data-theme="dark"]`, and use them with `var(...)` in `body`, `.card` and `.btn`. Add a `transition` on `background-color` and `color` so the switch is smooth. Write a script so the button toggles `document.documentElement.dataset.theme` between `light` and `dark`, and remembers the choice in `localStorage`.
@@starter
<!doctype html>
<html lang="en" data-theme="light">
<head>
  <meta charset="utf-8">
  <title>Themes</title>
  <style>
    body { font-family: system-ui, sans-serif; margin: 0; padding: 40px; min-height: 100vh; box-sizing: border-box; }
    .card { max-width: 360px; padding: 24px; border-radius: 16px; }
    .btn { border: 0; border-radius: 10px; padding: 10px 18px; font: 600 14px system-ui, sans-serif; color: #fff; cursor: pointer; }
    /* Your tokens and styles here */
  </style>
</head>
<body>
  <div class="card">
    <h2>Good evening 🌙</h2>
    <p>Your theme follows the tokens.</p>
    <button class="btn" id="toggle">Switch theme</button>
  </div>
  <script>
    // Your script here
  </script>
</body>
</html>
@@requires
[data-theme="light"]
[data-theme="dark"]
--bg:
--surface:
--text:
--accent:
var(--bg)
var(--surface)
var(--accent)
transition:
dataset.theme
localStorage
@@hint
`[data-theme="dark"] { --bg: #0f172a; --surface: #1e293b; --text: #e2e8f0; --accent: #818cf8; }`, then `body { background: var(--bg); color: var(--text); transition: background-color .25s, color .25s; }`. Script: `const root = document.documentElement; root.dataset.theme = localStorage.getItem('theme') || 'light'; toggle.onclick = () => { root.dataset.theme = root.dataset.theme === 'dark' ? 'light' : 'dark'; localStorage.setItem('theme', root.dataset.theme); };`
@@solution
<!doctype html>
<html lang="en" data-theme="light">
<head>
  <meta charset="utf-8">
  <title>Themes</title>
  <style>
    body { font-family: system-ui, sans-serif; margin: 0; padding: 40px; min-height: 100vh; box-sizing: border-box; }
    .card { max-width: 360px; padding: 24px; border-radius: 16px; }
    .btn { border: 0; border-radius: 10px; padding: 10px 18px; font: 600 14px system-ui, sans-serif; color: #fff; cursor: pointer; }
    [data-theme="light"] { --bg: #f8fafc; --surface: #ffffff; --text: #0f172a; --accent: #6366f1; }
    [data-theme="dark"] { --bg: #0f172a; --surface: #1e293b; --text: #e2e8f0; --accent: #818cf8; }
    body { background: var(--bg); color: var(--text); transition: background-color .25s, color .25s; }
    .card { background: var(--surface); box-shadow: 0 8px 24px rgb(15 23 42 / .08); transition: background-color .25s; }
    .btn { background: var(--accent); }
  </style>
</head>
<body>
  <div class="card">
    <h2>Good evening 🌙</h2>
    <p>Your theme follows the tokens.</p>
    <button class="btn" id="toggle">Switch theme</button>
  </div>
  <script>
    const root = document.documentElement;
    try { root.dataset.theme = localStorage.getItem('theme') || 'light'; } catch (e) {}
    document.getElementById('toggle').addEventListener('click', () => {
      root.dataset.theme = root.dataset.theme === 'dark' ? 'light' : 'dark';
      try { localStorage.setItem('theme', root.dataset.theme); } catch (e) {}
    });
  </script>
</body>
</html>

@@exercise Three-layer tokens with a glass panel
Write the tokens in three layers in `:root`: primitives (`--indigo-400`, `--indigo-500`, `--slate-50`, `--slate-900`), semantic tokens that reference them with `var()` (`--color-accent: var(--indigo-500)`, `--color-bg: var(--slate-50)`), and component tokens (`--button-bg: var(--color-accent)`). Override only the semantic ones inside `@media (prefers-color-scheme: dark)`. Add motion tokens `--motion-fast` and `--ease-out` and use them in the button's `transition`. Finish with a frosted `.glass` panel using `backdrop-filter: blur(...)`.
@@starter
<!doctype html>
<html lang="en">
<head>
  <meta charset="utf-8">
  <title>Token layers</title>
  <style>
    body { margin: 0; min-height: 100vh; font-family: system-ui, sans-serif; display: grid; place-items: center;
           background: radial-gradient(circle at 20% 20%, #a5b4fc, transparent 40%), radial-gradient(circle at 80% 70%, #f0abfc, transparent 40%), var(--color-bg, #f8fafc); }
    /* Your tokens and styles here */
  </style>
</head>
<body>
  <div class="glass">
    <h2>Design tokens</h2>
    <button class="btn">Primary action</button>
  </div>
</body>
</html>
@@requires
--indigo-500:
--slate-900:
--color-accent: var(--indigo-500)
--color-bg: var(--slate-50)
--button-bg: var(--color-accent)
@media (prefers-color-scheme: dark)
--motion-fast:
--ease-out:
var(--motion-fast)
var(--ease-out)
backdrop-filter: blur(
@@hint
`:root { --indigo-400: #818cf8; --indigo-500: #6366f1; --slate-50: #f8fafc; --slate-900: #0f172a; --color-accent: var(--indigo-500); --color-bg: var(--slate-50); --button-bg: var(--color-accent); --motion-fast: 120ms; --ease-out: cubic-bezier(.2, .8, .2, 1); }` and in the media query `:root { --color-accent: var(--indigo-400); --color-bg: var(--slate-900); }`.
@@solution
<!doctype html>
<html lang="en">
<head>
  <meta charset="utf-8">
  <title>Token layers</title>
  <style>
    body { margin: 0; min-height: 100vh; font-family: system-ui, sans-serif; display: grid; place-items: center;
           background: radial-gradient(circle at 20% 20%, #a5b4fc, transparent 40%), radial-gradient(circle at 80% 70%, #f0abfc, transparent 40%), var(--color-bg, #f8fafc); }
    :root {
      /* primitives */
      --indigo-400: #818cf8; --indigo-500: #6366f1; --slate-50: #f8fafc; --slate-900: #0f172a;
      /* semantic */
      --color-accent: var(--indigo-500); --color-bg: var(--slate-50); --color-text: var(--slate-900);
      /* component */
      --button-bg: var(--color-accent);
      /* motion */
      --motion-fast: 120ms; --ease-out: cubic-bezier(.2, .8, .2, 1);
    }
    @media (prefers-color-scheme: dark) {
      :root { --color-accent: var(--indigo-400); --color-bg: var(--slate-900); --color-text: var(--slate-50); }
    }
    .glass { padding: 32px; border-radius: 20px; color: var(--color-text);
             background: rgb(255 255 255 / .25); border: 1px solid rgb(255 255 255 / .4);
             backdrop-filter: blur(14px); box-shadow: 0 20px 40px rgb(15 23 42 / .15); }
    .btn { background: var(--button-bg); color: #fff; border: 0; border-radius: 10px; padding: 10px 18px; font: 600 14px system-ui, sans-serif;
           cursor: pointer; transition: transform var(--motion-fast) var(--ease-out), box-shadow var(--motion-fast) var(--ease-out); }
    .btn:hover { transform: translateY(-2px); box-shadow: 0 10px 20px rgb(99 102 241 / .4); }
  </style>
</head>
<body>
  <div class="glass">
    <h2>Design tokens</h2>
    <button class="btn">Primary action</button>
  </div>
</body>
</html>
