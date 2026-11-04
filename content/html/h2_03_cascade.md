@@lesson h2-cascade The Cascade, Specificity, Inheritance and Custom Properties
@@text
"Why isn't my CSS working?" is almost always answered by the **cascade**: the algorithm that decides which of several conflicting declarations wins. Learn it once and CSS stops feeling random.

## How a winner is chosen

When several rules set the same property on the same element, the browser compares them in this order, and the first difference decides:

1. **Origin and importance**: browser defaults < your styles < your `!important` styles (user and browser `!important` beat those; animations and transitions sit in between).
2. **Cascade layers** (`@layer`): later layers beat earlier ones; styles **not** in a layer beat all layered styles.
3. **Specificity** of the selector.
4. **Order**: the later declaration wins.

Inline `style=""` attributes beat any selector (think of them as the highest specificity).

## Specificity

Count three numbers (a, b, c) for a selector, and compare them left to right:

| Selector part | Adds to |
|---|---|
| `#id` | a |
| `.class`, `[attr]`, `:hover` and other pseudo-classes | b |
| `div`, `::before` and other element / pseudo-element names | c |
| `*`, combinators (`>`, `+`, `~`, space) | nothing |

```text
p                    (0, 0, 1)
.alert               (0, 1, 0)
nav a:hover          (0, 1, 2)
#sidebar .link       (1, 1, 0)
```

`(1, 0, 0)` beats `(0, 15, 0)`: a single id beats any number of classes. That's why many teams avoid ids in CSS: they're hard to override.

Special pseudo-classes:

- `:is(a, b)` and `:not(a, b)` take the specificity of their **most specific** argument.
- **`:where(...)` always has zero specificity**: perfect for defaults that anything can override.

## !important

`!important` jumps above all normal declarations. It's a sledgehammer: the only way to override it is another `!important` with higher specificity or a later position. Legitimate uses: utility classes that must always win (`.hidden { display: none !important; }`) and overriding third-party inline styles. If you find yourself adding it to fix a conflict, fix the specificity instead.

## Inheritance

Some properties **inherit** from the parent automatically: mostly text-related ones (`color`, `font-*`, `line-height`, `text-align`, `visibility`, `cursor`, `list-style`). Box properties don't (`margin`, `padding`, `border`, `background`, `width`, `display`). Control it with the keywords:

- `inherit`: take the parent's value
- `initial`: the property's default from the spec
- `unset`: inherit if the property normally inherits, else initial
- `revert`: go back to the browser's default style

`button { font: inherit; }` is a classic: form controls don't inherit fonts by default.

## Cascade layers

Layers let you declare the order of whole groups of styles up front, so a component library's specificity can't fight your page styles:

```html
<style>
  @layer reset, base, components, utilities;
  @layer base { a { color: #1d4ed8; } }
  @layer components { .card a { color: #0f766e; } }
  @layer utilities { .text-red { color: #b91c1c; } }
</style>
<div class="card"><a class="text-red" href="#">Red wins: utilities is the last layer</a></div>
```

Inside layers, specificity only matters **within** the same layer. `.text-red` (one class) beats `.card a` (a class and an element) because its layer comes later.

## Custom properties (CSS variables)

```html
<style>
  :root { --brand: #e11d48; --radius: 8px; --space: 1rem; }
  .btn { background: var(--brand); border-radius: var(--radius); padding: calc(var(--space) / 2) var(--space); color: white; border: 0; }
  .danger-zone { --brand: #7f1d1d; }       /* redefine for a subtree */
</style>
<button class="btn">Normal</button>
<div class="danger-zone"><button class="btn">Inside the danger zone</button></div>
```

- Names start with `--`; read them with `var(--name, fallback)`.
- They **inherit** and cascade like any property, so redefining one on an element changes it for everything inside. That's how themes (dark mode) and component variants work.
- They can be changed at runtime from JavaScript: `element.style.setProperty('--brand', 'teal')`.

## Debugging the cascade

DevTools (F12) → Elements → Styles shows every rule that matches, crossed out where it lost, and the **Computed** tab shows the final value and where it came from. When a style "doesn't work", look there first: is the rule matching at all (typo in the selector)? Is it overridden? Is the property even inherited?

@@quiz
Which selector wins: `#nav a` or `.menu .item a`?
* `#nav a` (1, 0, 1) beats `.menu .item a` (0, 2, 1)
- `.menu .item a`, because it has more parts
- Whichever comes later
- They tie
? Ids beat any number of classes.

@@quiz
What's special about `:where()`?
- It matches more elements
* It always adds zero specificity, so its rules are easy to override
- It only works in layers
- It's the same as `:is()`
? `:is()` takes its most specific argument's specificity.

@@quiz
Which of these properties is inherited by default?
- `margin`
- `border`
* `color`
- `background`
? Text-related properties inherit; box properties don't.

@@quiz
With `@layer base, components;`, a rule in `base` with an id selector vs a rule in `components` with a class: which wins?
- The id, because ids are more specific
* The class in `components`, because later layers win before specificity is compared
- Neither applies
- The one written later in the file
? Layer order is compared before specificity.

@@flashcards
cascade order :: origin/importance → layers → specificity → order
specificity :: (ids, classes/attributes/pseudo-classes, elements)
`:where()` :: zero specificity
`:is()` :: specificity of its most specific argument
`!important` :: beats normal declarations
inherited properties :: color, font, line-height, text-align...
`inherit` / `initial` / `unset` / `revert` :: cascade keywords
`@layer a, b;` :: later layers win
unlayered styles :: beat all layered styles
`--name` / `var(--name, fallback)` :: custom properties
`font: inherit` :: make form controls use the page font

@@exercise Fix the specificity war
The "danger" button should be red, but the `#toolbar button` rule wins. Fix it **without** `!important` and without changing the HTML: lower the toolbar rule's specificity with `:where()` (`:where(#toolbar) button`) so that `.btn-danger` wins.
@@starter
<!doctype html>
<html lang="en">
<head><meta charset="utf-8"><title>Toolbar</title>
<style>
  #toolbar button { background: #334155; color: white; border: 0; padding: 0.5rem 1rem; }
  .btn-danger { background: #b91c1c; }
</style>
</head>
<body>
  <div id="toolbar">
    <button>Save</button>
    <button class="btn-danger">Delete</button>
  </div>
</body>
</html>
@@requires
:where(#toolbar) button
.btn-danger
@@hint
`:where(#toolbar) button` has specificity (0, 0, 1), so `.btn-danger` (0, 1, 0) beats it for the background.
@@solution
<!doctype html>
<html lang="en">
<head><meta charset="utf-8"><title>Toolbar</title>
<style>
  :where(#toolbar) button { background: #334155; color: white; border: 0; padding: 0.5rem 1rem; }
  .btn-danger { background: #b91c1c; }
</style>
</head>
<body>
  <div id="toolbar">
    <button>Save</button>
    <button class="btn-danger">Delete</button>
  </div>
</body>
</html>

@@exercise Theme with custom properties
Rewrite the hard-coded colours as custom properties on `:root`: `--bg`, `--text`, `--accent` and `--card`. Use them everywhere with `var()`. Then add a `.theme-dark` class that redefines the four variables (bg `#0f172a`, text `#e2e8f0`, accent `#38bdf8`, card `#1e293b`), and a button that toggles that class on `<html>` with JavaScript.
@@starter
<!doctype html>
<html lang="en">
<head><meta charset="utf-8"><title>Status</title>
<style>
  body { background: #f8fafc; color: #0f172a; font-family: system-ui, sans-serif; }
  .card { background: white; padding: 1rem; border-left: 4px solid #e11d48; }
  a { color: #e11d48; }
</style>
</head>
<body>
  <div class="card"><h1>All systems operational</h1><a href="#">History</a></div>
</body>
</html>
@@requires
--accent
var(--bg)
var(--card)
.theme-dark
classList.toggle
@@hint
`:root { --bg: #f8fafc; --text: #0f172a; --accent: #e11d48; --card: white; } .theme-dark { --bg: #0f172a; ... }` and `body { background: var(--bg); color: var(--text); }`. Button: `document.getElementById("theme").addEventListener("click", () => document.documentElement.classList.toggle("theme-dark"));`
@@solution
<!doctype html>
<html lang="en">
<head><meta charset="utf-8"><title>Status</title>
<style>
  :root { --bg: #f8fafc; --text: #0f172a; --accent: #e11d48; --card: white; }
  .theme-dark { --bg: #0f172a; --text: #e2e8f0; --accent: #38bdf8; --card: #1e293b; }
  body { background: var(--bg); color: var(--text); font-family: system-ui, sans-serif; }
  .card { background: var(--card); padding: 1rem; border-left: 4px solid var(--accent); }
  a { color: var(--accent); }
</style>
</head>
<body>
  <button type="button" id="theme">Toggle dark theme</button>
  <div class="card"><h1>All systems operational</h1><a href="#">History</a></div>
  <script>
    document.getElementById("theme").addEventListener("click", () => {
      document.documentElement.classList.toggle("theme-dark");
    });
  </script>
</body>
</html>

@@exercise Layers for a component library
The page uses a "vendor" stylesheet whose `.card h2` rule overrides your simple heading style. Put the vendor CSS in a layer `vendor` and your styles in a layer `site`, declared in that order with `@layer vendor, site;`, so your `h2 { color: #0f766e; }` wins even though it's less specific. Also make the form controls use the page font with `font: inherit`.
@@starter
<!doctype html>
<html lang="en">
<head><meta charset="utf-8"><title>Layers</title>
<style>
  /* vendor.css */
  .card h2 { color: #6b21a8; font-size: 1.5rem; }
  .card { border: 1px solid #ccc; padding: 1rem; }

  /* our styles */
  body { font-family: Georgia, serif; }
  h2 { color: #0f766e; }
</style>
</head>
<body>
  <div class="card"><h2>Our heading should be teal</h2><input value="Our font?"> <button>Go</button></div>
</body>
</html>
@@requires
@layer vendor, site;
@layer vendor {
@layer site {
font: inherit
@@hint
`@layer vendor, site; @layer vendor { .card h2 { ... } .card { ... } } @layer site { body { ... } h2 { color: #0f766e; } input, button { font: inherit; } }`.
@@solution
<!doctype html>
<html lang="en">
<head><meta charset="utf-8"><title>Layers</title>
<style>
  @layer vendor, site;

  @layer vendor {
    .card h2 { color: #6b21a8; font-size: 1.5rem; }
    .card { border: 1px solid #ccc; padding: 1rem; }
  }

  @layer site {
    body { font-family: Georgia, serif; }
    h2 { color: #0f766e; }
    input, button { font: inherit; }
  }
</style>
</head>
<body>
  <div class="card"><h2>Our heading should be teal</h2><input value="Our font?"> <button>Go</button></div>
</body>
</html>
