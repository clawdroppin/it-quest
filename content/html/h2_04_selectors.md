@@lesson h2-selectors Selectors in Depth: Combinators, Pseudo-classes and Pseudo-elements
@@text
Good selectors let you style a page without adding a class to every element. This lesson covers the full selector toolbox, including `:has()`, the "parent selector" CSS waited 20 years for.

## Combinators

| Selector | Matches |
|---|---|
| `nav a` | any `a` **inside** a `nav` (descendant, any depth) |
| `ul > li` | `li` that are **direct children** of `ul` |
| `h2 + p` | the `p` **immediately after** an `h2` (adjacent sibling) |
| `h2 ~ p` | every `p` after an `h2` with the same parent (general sibling) |

## Attribute selectors

| Selector | Matches |
|---|---|
| `[disabled]` | has the attribute |
| `[type="email"]` | exact value |
| `a[href^="https://"]` | value starts with |
| `a[href$=".pdf"]` | ends with |
| `a[href*="sakura"]` | contains |
| `[lang|="ja"]` | `ja` or `ja-...` |
| `[class~="card"]` | a space-separated word |
| `[data-status="down" i]` | case-insensitive |

Useful for marking external links, file types, or styling by `data-*` state without extra classes.

## Structural pseudo-classes

| Selector | Matches |
|---|---|
| `:first-child`, `:last-child`, `:only-child` | by position among siblings |
| `:nth-child(2)` | the 2nd child |
| `:nth-child(odd)` / `(even)` | zebra striping |
| `:nth-child(3n + 1)` | 1st, 4th, 7th... |
| `:nth-child(-n + 3)` | the first three |
| `:nth-last-child(1)` | counted from the end |
| `:nth-of-type(2)` | the 2nd element **of its type** |
| `:empty` | no children (not even text) |
| `:not(.done)` | doesn't match the list |

```html
<style>
  tbody tr:nth-child(even) { background: #f1f5f9; }
  li:not(:last-child) { border-bottom: 1px solid #e2e8f0; }
  li:nth-child(-n + 3) { font-weight: bold; }
</style>
<ul><li>First</li><li>Second</li><li>Third</li><li>Fourth</li></ul>
```

## State pseudo-classes

`:hover`, `:active`, `:focus`, **`:focus-visible`** (focus from the keyboard: show your focus ring here, so mouse users don't see it on every click but keyboard users always do), `:focus-within` (the element or something inside has focus), `:checked`, `:disabled`, `:required`, `:valid`, `:invalid`, `:user-invalid`, `:target` (the element whose id is in the URL's `#hash`), `:visited`.

## `:has()`: style based on what's inside or after

`A:has(B)` matches an A that contains (or is related to) a B. It's supported in all current browsers.

```html
<style>
  .card:has(img) { display: grid; grid-template-columns: 80px 1fr; gap: 1rem; }
  label:has(input:checked) { font-weight: bold; color: #15803d; }
  form:has(:user-invalid) button[type="submit"] { opacity: 0.5; }
  h2:has(+ p) { margin-bottom: 0.25rem; }   /* an h2 followed by a p */
</style>
<label><input type="checkbox" checked> Backups verified</label>
<label><input type="checkbox"> Patches installed</label>
```

`:has()` replaces a lot of JavaScript that only existed to toggle classes on parents.

## Pseudo-elements

| Pseudo-element | Styles |
|---|---|
| `::before`, `::after` | generated content before/after the element's content (needs `content: ""`) |
| `::marker` | the bullet or number of a list item |
| `::placeholder` | input placeholder text |
| `::selection` | selected text |
| `::first-line`, `::first-letter` | typographic effects |
| `::backdrop` | behind a modal `<dialog>` or fullscreen element |

```html
<style>
  a[href$=".pdf"]::after { content: " (PDF)"; color: #64748b; font-size: 0.85em; }
  a[href^="http"]:not([href*="sakura.example"])::after { content: " ↗"; }
  li::marker { color: #e11d48; }
</style>
<ul>
  <li><a href="/policies/security.pdf">Security policy</a></li>
  <li><a href="https://learn.microsoft.com">Microsoft Learn</a></li>
  <li><a href="https://wiki.sakura.example">Our wiki</a></li>
</ul>
```

Generated content is decorative: screen readers may or may not read it, and it can't be selected. Don't put essential information only in `::before`.

## Counters

CSS can number things itself: `counter-reset: step;` on the container, `counter-increment: step;` on each item, and `content: counter(step)` in a pseudo-element. Handy for numbered steps with custom styling.

## Writing maintainable selectors

- Prefer **classes** for styling hooks; they're explicit and low-specificity.
- Keep selectors short. `.sidebar ul li a span` breaks as soon as the HTML changes.
- Don't style by ids, and don't use deep descendant chains to win specificity battles.
- Style **states** with attributes the browser or ARIA already sets (`[aria-expanded="true"]`, `:checked`, `[disabled]`) rather than duplicating them as classes.

@@quiz
What does `ul > li` match that `ul li` doesn't restrict?
* Only list items that are direct children of the ul, not items in nested lists
- Only the first li
- Items after the ul
- Nothing; they're identical
? `>` is the child combinator.

@@quiz
Which pseudo-class should show focus rings for keyboard users without showing them on mouse clicks?
- `:focus`
- `:hover`
* `:focus-visible`
- `:active`
? Never remove focus outlines without a replacement.

@@quiz
What does `label:has(input:checked)` match?
- Checked inputs
* Labels that contain a checked input
- Inputs inside labels
- Labels after a checked input
? `:has()` selects based on descendants or following siblings.

@@quiz
`li:nth-child(-n + 3)` matches...
- every third li
- the last three
* the first three
- none
? For n = 0, 1, 2: positions 3, 2, 1.

@@flashcards
`A B` :: B anywhere inside A
`A > B` :: direct child
`A + B` :: next sibling
`A ~ B` :: any later sibling
`[href^="https"]` :: attribute starts with
`[href$=".pdf"]` :: attribute ends with
`:nth-child(even)` :: zebra rows
`:not(...)` :: exclude
`:focus-visible` :: keyboard focus
`:focus-within` :: focus inside
`:has(...)` :: based on contents (parent selector)
`::before` / `::after` :: generated content (needs content)
`::marker` :: list bullets/numbers
`counter-increment` :: CSS numbering

@@exercise Style a server table without classes
Style the table using only structural and attribute selectors (no new classes or ids): zebra-stripe body rows (`tbody tr:nth-child(even)`), right-align the last column of every row (`td:last-child`, `th:last-child`), and colour the status cells red when `data-status="down"` and green when `data-status="up"`.
@@starter
<!doctype html>
<html lang="en">
<head><meta charset="utf-8"><title>Servers</title>
<style>
  table { border-collapse: collapse; }
  th, td { padding: 0.4rem 0.8rem; }
</style>
</head>
<body>
  <table>
    <thead><tr><th>Server</th><th>Status</th><th>Uptime (days)</th></tr></thead>
    <tbody>
      <tr><td>SV-0001</td><td data-status="up">up</td><td>142</td></tr>
      <tr><td>SV-0002</td><td data-status="down">down</td><td>0</td></tr>
      <tr><td>SV-0003</td><td data-status="up">up</td><td>37</td></tr>
      <tr><td>SV-0004</td><td data-status="up">up</td><td>9</td></tr>
    </tbody>
  </table>
</body>
</html>
@@requires
tbody tr:nth-child(even)
td:last-child
[data-status="down"]
[data-status="up"]
@@hint
`tbody tr:nth-child(even) { background: #f1f5f9; } td:last-child, th:last-child { text-align: right; } [data-status="down"] { color: #b91c1c; font-weight: bold; } [data-status="up"] { color: #15803d; }`
@@solution
<!doctype html>
<html lang="en">
<head><meta charset="utf-8"><title>Servers</title>
<style>
  table { border-collapse: collapse; }
  th, td { padding: 0.4rem 0.8rem; }
  tbody tr:nth-child(even) { background: #f1f5f9; }
  td:last-child, th:last-child { text-align: right; }
  [data-status="down"] { color: #b91c1c; font-weight: bold; }
  [data-status="up"] { color: #15803d; }
</style>
</head>
<body>
  <table>
    <thead><tr><th>Server</th><th>Status</th><th>Uptime (days)</th></tr></thead>
    <tbody>
      <tr><td>SV-0001</td><td data-status="up">up</td><td>142</td></tr>
      <tr><td>SV-0002</td><td data-status="down">down</td><td>0</td></tr>
      <tr><td>SV-0003</td><td data-status="up">up</td><td>37</td></tr>
      <tr><td>SV-0004</td><td data-status="up">up</td><td>9</td></tr>
    </tbody>
  </table>
</body>
</html>

@@exercise A checklist with :has()
Make the checklist react to its checkboxes with **no JavaScript**: a label whose checkbox is checked gets a line-through and grey text, and the whole `.checklist` gets a green border when it has no unchecked box left (`.checklist:not(:has(input:not(:checked)))`). Show a keyboard focus ring with `:focus-visible` on the checkboxes.
@@starter
<!doctype html>
<html lang="en">
<head><meta charset="utf-8"><title>Offboarding</title>
<style>
  .checklist { border: 3px solid #cbd5e1; padding: 1rem; display: grid; gap: 0.5rem; }
</style>
</head>
<body>
  <div class="checklist">
    <label><input type="checkbox"> Disable the account</label>
    <label><input type="checkbox"> Collect the laptop</label>
    <label><input type="checkbox"> Remove building access</label>
  </div>
</body>
</html>
@@requires
label:has(input:checked)
.checklist:not(:has(input:not(:checked)))
:focus-visible
@@hint
`label:has(input:checked) { text-decoration: line-through; color: #64748b; }`, `.checklist:not(:has(input:not(:checked))) { border-color: #15803d; }`, `input:focus-visible { outline: 3px solid #2563eb; outline-offset: 2px; }`.
@@solution
<!doctype html>
<html lang="en">
<head><meta charset="utf-8"><title>Offboarding</title>
<style>
  .checklist { border: 3px solid #cbd5e1; padding: 1rem; display: grid; gap: 0.5rem; }
  label:has(input:checked) { text-decoration: line-through; color: #64748b; }
  .checklist:not(:has(input:not(:checked))) { border-color: #15803d; }
  input:focus-visible { outline: 3px solid #2563eb; outline-offset: 2px; }
</style>
</head>
<body>
  <div class="checklist">
    <label><input type="checkbox"> Disable the account</label>
    <label><input type="checkbox"> Collect the laptop</label>
    <label><input type="checkbox"> Remove building access</label>
  </div>
</body>
</html>

@@exercise Numbered steps and link hints
Style the setup guide: number the steps with a CSS **counter** shown in a round badge (`::before` with `counter(step)`), mark links to PDFs with ` (PDF)` after them, and mark external links (starting with `http` but not on `sakura.example`) with ` ↗`.
@@starter
<!doctype html>
<html lang="en">
<head><meta charset="utf-8"><title>Laptop setup</title>
<style>
  .steps { list-style: none; padding: 0; }
  .steps li { margin: 0.75rem 0; }
</style>
</head>
<body>
  <ol class="steps">
    <li>Read the <a href="/docs/acceptable-use.pdf">acceptable use policy</a>.</li>
    <li>Sign in and install updates from <a href="https://support.microsoft.com">Microsoft Support</a>.</li>
    <li>Enrol in MFA on the <a href="https://mfa.sakura.example">MFA portal</a>.</li>
  </ol>
</body>
</html>
@@requires
counter-reset: step
counter-increment: step
counter(step)
a[href$=".pdf"]::after
:not([href*="sakura.example"])
@@hint
`.steps { counter-reset: step; } .steps li { counter-increment: step; } .steps li::before { content: counter(step); display: inline-grid; place-items: center; width: 1.6em; height: 1.6em; border-radius: 50%; background: #e11d48; color: white; margin-right: 0.5rem; }`. Links: `a[href$=".pdf"]::after { content: " (PDF)"; }` and `a[href^="http"]:not([href*="sakura.example"])::after { content: " ↗"; }`.
@@solution
<!doctype html>
<html lang="en">
<head><meta charset="utf-8"><title>Laptop setup</title>
<style>
  .steps { list-style: none; padding: 0; counter-reset: step; }
  .steps li { margin: 0.75rem 0; counter-increment: step; }
  .steps li::before {
    content: counter(step);
    display: inline-grid; place-items: center;
    width: 1.6em; height: 1.6em; margin-right: 0.5rem;
    border-radius: 50%; background: #e11d48; color: white; font-weight: bold;
  }
  a[href$=".pdf"]::after { content: " (PDF)"; color: #64748b; font-size: 0.85em; }
  a[href^="http"]:not([href*="sakura.example"])::after { content: " ↗"; }
</style>
</head>
<body>
  <ol class="steps">
    <li>Read the <a href="/docs/acceptable-use.pdf">acceptable use policy</a>.</li>
    <li>Sign in and install updates from <a href="https://support.microsoft.com">Microsoft Support</a>.</li>
    <li>Enrol in MFA on the <a href="https://mfa.sakura.example">MFA portal</a>.</li>
  </ol>
</body>
</html>
