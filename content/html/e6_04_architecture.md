@@lesson e6-architecture CSS Architecture: Tokens, BEM, Utilities and Cascade Layers
@@text
A 50-line stylesheet needs no system. A 5,000-line one written by six people over three years does: without one, every change breaks something else, and people add `!important` until nothing can be overridden. This lesson covers the patterns teams use to keep CSS maintainable.

## Design tokens with custom properties

Tokens are the named decisions of a design: colours, spacing steps, radii, font sizes. Define them once, use them everywhere:

```css
:root {
  /* primitives */
  --blue-600: #2563eb;  --red-600: #dc2626;  --slate-50: #f8fafc;  --slate-900: #0f172a;
  /* semantic tokens: what they're FOR */
  --color-bg: var(--slate-50);
  --color-text: var(--slate-900);
  --color-accent: var(--blue-600);
  --color-danger: var(--red-600);
  /* scales */
  --space-1: 0.25rem; --space-2: 0.5rem; --space-3: 1rem; --space-4: 1.5rem; --space-5: 2.5rem;
  --radius: 8px;
  --step-0: 1rem; --step-1: 1.25rem; --step-2: 1.563rem;
}
@media (prefers-color-scheme: dark) {
  :root { --color-bg: var(--slate-900); --color-text: var(--slate-50); }
}
```

Components use **semantic** tokens (`--color-danger`), never primitives (`--red-600`). Re-theming (dark mode, a client's brand) then means changing a few semantic tokens. A spacing **scale** stops the "13px here, 15px there" drift.

Custom properties inherit and can be overridden locally: `.alert { --color-accent: var(--color-danger); }` turns everything inside the alert that uses the accent red. That makes component **variants** cheap.

## BEM naming

Block, Element, Modifier: a naming convention that keeps selectors flat (one class each) and says what belongs to what:

```html
<article class="card card--urgent">
  <h2 class="card__title">VPN down</h2>
  <p class="card__body">Since 09:12.</p>
  <button class="card__action card__action--primary">Acknowledge</button>
</article>
```

```css
.card { ... }                       /* block */
.card__title { ... }                /* element: a part of the block */
.card--urgent { border-color: var(--color-danger); }   /* modifier: a variant */
```

Benefits: every selector has the same low specificity (one class), so order decides conflicts predictably; you can tell from the HTML which CSS applies; components don't leak into each other. The cost is verbose class names.

## Utility classes

The opposite approach: many tiny single-purpose classes composed in the HTML (the idea behind Tailwind):

```html
<div class="flex gap-2 p-3 rounded bg-surface">...</div>
```

```css
.flex { display: flex; } .gap-2 { gap: var(--space-2); } .p-3 { padding: var(--space-3); }
```

Fast to build with and the CSS stops growing, but the HTML gets noisy and repeated patterns need extracting into components (in a framework, a component file). Most real codebases mix: components for structure, a handful of utilities (`.visually-hidden`, `.stack`, `.flow`) for common needs.

## Layout primitives

A few reusable layout classes cover most needs:

```css
.stack > * + * { margin-block-start: var(--stack-space, var(--space-3)); }  /* vertical rhythm */
.cluster { display: flex; flex-wrap: wrap; gap: var(--space-2); align-items: center; }  /* tags, buttons */
.center { max-inline-size: 65ch; margin-inline: auto; padding-inline: var(--space-3); }
.grid-auto { display: grid; gap: var(--space-3); grid-template-columns: repeat(auto-fit, minmax(min(15rem, 100%), 1fr)); }
```

## Cascade layers

`@layer` lets you declare **which groups of styles win**, regardless of specificity:

```css
@layer reset, base, components, utilities;   /* order: later layers win */

@layer reset { *, *::before, *::after { box-sizing: border-box; } body { margin: 0; } }
@layer base { a { color: var(--color-accent); } }
@layer components { .card a { color: inherit; } }
@layer utilities { .text-danger { color: var(--color-danger); } }
```

A utility with one class in the `utilities` layer beats a component selector with three classes in `components`, because layer order is checked **before** specificity. Styles not in any layer beat all layers. Third-party CSS can be imported into a low layer: `@import url(vendor.css) layer(vendor);` so your own styles always win without specificity wars.

## Other organising tools

- **`:where()`** has zero specificity: `:where(.prose) h2 { }` is easy to override.
- A small **reset** (box-sizing, margin 0, `img { max-inline-size: 100%; display: block; }`) at the start.
- **Order within a file**: tokens → reset → base elements → layout primitives → components → utilities.
- **Linting**: Stylelint catches invalid properties, duplicate selectors and disallowed patterns (no IDs, max nesting depth).

@@quiz
Why should components use semantic tokens like `--color-danger` instead of `--red-600`?
* So re-theming changes a few semantic tokens instead of every component
- Semantic tokens are faster
- Primitives don't work in dark mode
- Browsers require it
? Components describe purpose; themes map purpose to colours.

@@quiz
In BEM, what does `card__title--large` mean?
- A card inside a title
* The `title` element of the `card` block, in its `large` variant
- Two separate classes
- An ID
? Block__element--modifier.

@@quiz
With `@layer base, components, utilities;`, a one-class rule in `utilities` vs a three-class rule in `components`: which wins?
- The three-class rule (higher specificity)
* The utilities rule, because later layers win before specificity is compared
- Whichever is written last in the file
- Neither applies
? Layer order beats specificity.

@@quiz
What is the specificity of `:where(.card .title)`?
* Zero
- Two classes
- One class
- Same as an ID
? `:where()` is designed to be easily overridden.

@@flashcards
design token :: named design decision (colour, space, radius)
semantic token :: named by purpose (`--color-danger`)
`.block__element--modifier` :: BEM
utility class :: one small purpose (`.p-3`)
`.stack > * + *` :: vertical spacing between children
`@layer a, b;` :: b beats a regardless of specificity
unlayered styles :: beat all layers
`@import url(x.css) layer(vendor)` :: put third-party CSS in a low layer
`:where()` :: zero specificity
Stylelint :: CSS linter

@@exercise Tokens and a themed alert
Refactor the hard-coded values into tokens on `:root`: `--color-text`, `--color-bg`, `--color-accent` (#2563eb), `--color-danger` (#dc2626), `--space-2` (0.5rem), `--space-3` (1rem) and `--radius` (8px). Use them in `.btn` and `.alert`. Then make `.alert--danger` a variant that only re-points `--color-accent` to `var(--color-danger)`, so the alert's border **and** its button turn red automatically. Add a dark theme that changes `--color-text` and `--color-bg` under `prefers-color-scheme: dark`.
@@starter
<!doctype html>
<html lang="en">
<head><meta charset="utf-8"><title>Tokens</title>
<style>
  body { color: #0f172a; background: #f8fafc; font-family: system-ui; }
  .alert { border: 2px solid #2563eb; padding: 1rem; border-radius: 8px; margin-block: 1rem; }
  .btn { background: #2563eb; color: white; border: 0; padding: 0.5rem 1rem; border-radius: 8px; }
  .alert--danger { border-color: #dc2626; }
  .alert--danger .btn { background: #dc2626; }
</style>
</head>
<body>
  <div class="alert"><p>Backups finished.</p><button class="btn">View report</button></div>
  <div class="alert alert--danger"><p>Disk C: is 97% full.</p><button class="btn">Clean up</button></div>
</body>
</html>
@@requires
--color-accent: #2563eb
--color-danger: #dc2626
--space-3: 1rem
--radius: 8px
background: var(--color-accent)
.alert--danger { --color-accent: var(--color-danger)
prefers-color-scheme: dark
@@hint
`.alert { border: 2px solid var(--color-accent); padding: var(--space-3); border-radius: var(--radius); }` and `.btn { background: var(--color-accent); ... }`. The variant is just `.alert--danger { --color-accent: var(--color-danger); }`: custom properties inherit, so the button inside picks it up.
@@solution
<!doctype html>
<html lang="en">
<head><meta charset="utf-8"><title>Tokens</title>
<style>
  :root {
    --color-text: #0f172a; --color-bg: #f8fafc;
    --color-accent: #2563eb; --color-danger: #dc2626;
    --space-2: 0.5rem; --space-3: 1rem; --radius: 8px;
  }
  @media (prefers-color-scheme: dark) {
    :root { --color-text: #f8fafc; --color-bg: #0f172a; }
  }
  body { color: var(--color-text); background: var(--color-bg); font-family: system-ui; }
  .alert { border: 2px solid var(--color-accent); padding: var(--space-3); border-radius: var(--radius); margin-block: var(--space-3); }
  .btn { background: var(--color-accent); color: white; border: 0; padding: var(--space-2) var(--space-3); border-radius: var(--radius); }
  .alert--danger { --color-accent: var(--color-danger); }
</style>
</head>
<body>
  <div class="alert"><p>Backups finished.</p><button class="btn">View report</button></div>
  <div class="alert alert--danger"><p>Disk C: is 97% full.</p><button class="btn">Clean up</button></div>
</body>
</html>

@@exercise BEM a messy component
The ticket component uses nested descendant selectors and an ID, so nothing can be overridden. Rename it with BEM: block `ticket`, elements `ticket__id`, `ticket__title`, `ticket__meta`, and modifier `ticket--p1` for the urgent one. Every selector in the stylesheet must be a single class (no IDs, no descendant chains).
@@starter
<!doctype html>
<html lang="en">
<head><meta charset="utf-8"><title>BEM</title>
<style>
  div.tickets article { border: 1px solid #cbd5e1; padding: 0.75rem; border-radius: 8px; margin-block: 0.5rem; }
  div.tickets article b { color: #64748b; }
  div.tickets article h3 { margin: 0; }
  div.tickets article small { color: #64748b; }
  #urgent { border-color: #dc2626; background: #fef2f2; }
</style>
</head>
<body>
  <div class="tickets">
    <article id="urgent"><b>#9</b><h3>File server almost full</h3><small>P1 · Fatima Khan</small></article>
    <article><b>#7</b><h3>Badge reader offline</h3><small>P3 · unassigned</small></article>
  </div>
</body>
</html>
@@requires
class="ticket ticket--p1"
class="ticket__id"
class="ticket__title"
class="ticket__meta"
.ticket--p1 {
.ticket__title {
@@hint
`<article class="ticket ticket--p1"><b class="ticket__id">#9</b><h3 class="ticket__title">...</h3><small class="ticket__meta">...</small></article>`, and CSS `.ticket { } .ticket__id { } .ticket__title { } .ticket__meta { } .ticket--p1 { }`.
@@solution
<!doctype html>
<html lang="en">
<head><meta charset="utf-8"><title>BEM</title>
<style>
  .ticket { border: 1px solid #cbd5e1; padding: 0.75rem; border-radius: 8px; margin-block: 0.5rem; }
  .ticket__id { color: #64748b; }
  .ticket__title { margin: 0; }
  .ticket__meta { color: #64748b; }
  .ticket--p1 { border-color: #dc2626; background: #fef2f2; }
</style>
</head>
<body>
  <div class="tickets">
    <article class="ticket ticket--p1"><b class="ticket__id">#9</b><h3 class="ticket__title">File server almost full</h3><small class="ticket__meta">P1 · Fatima Khan</small></article>
    <article class="ticket"><b class="ticket__id">#7</b><h3 class="ticket__title">Badge reader offline</h3><small class="ticket__meta">P3 · unassigned</small></article>
  </div>
</body>
</html>

@@exercise Cascade layers end a specificity war
A vendor stylesheet (simulated in the first `<style>` block) uses `#app .content p.note` to colour notes grey, and the team's `.text-danger` utility can't win without `!important`. Put the vendor rules in `@layer vendor`, the team's components in `@layer components` and the utility in `@layer utilities`, declared in that order with `@layer vendor, components, utilities;`. Remove the `!important`.
@@starter
<!doctype html>
<html lang="en">
<head><meta charset="utf-8"><title>Layers</title>
<style>
  #app .content p.note { color: #64748b; font-style: italic; }
</style>
<style>
  .note { padding-inline-start: 0.5rem; border-inline-start: 3px solid #cbd5e1; }
  .text-danger { color: #dc2626 !important; }
</style>
</head>
<body>
  <div id="app"><div class="content">
    <p class="note">Maintenance window: Saturday 02:00.</p>
    <p class="note text-danger">Backups failed last night.</p>
  </div></div>
</body>
</html>
@@requires
@layer vendor, components, utilities;
@layer vendor {
@layer components {
@layer utilities {
.text-danger { color: #dc2626; }
@@hint
Add `@layer vendor, components, utilities;` at the top of the first style block and wrap each group: `@layer vendor { #app .content p.note { ... } }`. In the utilities layer, `.text-danger { color: #dc2626; }` now wins because `utilities` comes after `vendor`.
@@solution
<!doctype html>
<html lang="en">
<head><meta charset="utf-8"><title>Layers</title>
<style>
  @layer vendor, components, utilities;
  @layer vendor {
    #app .content p.note { color: #64748b; font-style: italic; }
  }
</style>
<style>
  @layer components {
    .note { padding-inline-start: 0.5rem; border-inline-start: 3px solid #cbd5e1; }
  }
  @layer utilities {
    .text-danger { color: #dc2626; }
  }
</style>
</head>
<body>
  <div id="app"><div class="content">
    <p class="note">Maintenance window: Saturday 02:00.</p>
    <p class="note text-danger">Backups failed last night.</p>
  </div></div>
</body>
</html>
