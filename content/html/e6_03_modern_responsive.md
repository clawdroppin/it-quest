@@lesson e6-modern Modern Responsive CSS: Container Queries, Logical Properties and Nesting
@@text
Media queries ask "how wide is the **screen**?". But a component doesn't care about the screen: a ticket card might sit in a narrow sidebar on a huge monitor, or fill a phone. Modern CSS lets components respond to **their own space**, write direction-independent layouts, and organise styles with nesting, all supported in current browsers.

## Container queries

Mark an element as a container, then style its children based on the **container's** size:

```html
<style>
  .slot { container-type: inline-size; border: 1px dashed #94a3b8; padding: 0.5rem; margin-bottom: 1rem; }
  .ticket { display: grid; gap: 0.25rem; background: #f8fafc; padding: 0.75rem; border-radius: 8px; }
  @container (min-width: 28rem) {
    .ticket { grid-template-columns: 6rem 1fr auto; align-items: center; }
  }
</style>
<div class="slot" style="width: 18rem"><div class="ticket"><b>#11</b><span>Suspicious login from abroad</span><span>P1</span></div></div>
<div class="slot"><div class="ticket"><b>#11</b><span>Suspicious login from abroad</span><span>P1</span></div></div>
```

- `container-type: inline-size` makes an element a size container for its width (the common case). It can't size itself from its children in that direction any more, so put it on a wrapper whose width comes from its parent.
- `container-name: sidebar` plus `@container sidebar (min-width: ...)` targets a specific container.
- **Container units**: `cqi` (1% of the container's inline size), `cqb`; `font-size: clamp(1rem, 4cqi, 1.5rem)` scales text with the component.

Use media queries for page-level layout, container queries for components.

## Style and user-preference queries

```css
@media (prefers-color-scheme: dark) { ... }      /* OS dark mode */
@media (prefers-contrast: more) { ... }          /* high contrast requested */
@media (hover: hover) and (pointer: fine) { ... } /* a real mouse: hover effects make sense */
@media (forced-colors: active) { ... }           /* Windows High Contrast */
@media (min-width: 40rem) and (max-width: 60rem) { ... }
@media (40rem <= width <= 60rem) { ... }         /* range syntax: same thing */
```

`(hover: hover)` is important on touch devices: hover-only menus are unusable on phones.

## Logical properties

Physical properties (`left`, `margin-right`, `width`) assume left-to-right horizontal text. **Logical** properties follow the writing direction, so layouts work for Arabic and Hebrew (right to left) and vertical Japanese (top to bottom) without rewriting:

| Physical | Logical |
|---|---|
| `width` / `height` | `inline-size` / `block-size` |
| `margin-left` / `margin-right` | `margin-inline-start` / `margin-inline-end` |
| `margin-top` / `margin-bottom` | `margin-block-start` / `margin-block-end` |
| `padding-left` + `right` | `padding-inline` |
| `left` / `top` (positioning) | `inset-inline-start` / `inset-block-start` |
| `text-align: left` | `text-align: start` |
| `border-left` | `border-inline-start` |

`margin-inline: auto` (centring) and `padding-block: 1rem` are handy shorthands even if you never translate your site.

```html
<style>
  .note { border-inline-start: 4px solid #e11d48; padding-inline: 1rem; margin-block: 0.5rem; }
</style>
<p class="note">English: the border is on the left.</p>
<p class="note" dir="rtl" lang="ar">العربية: الحد على اليمين.</p>
<p class="note" style="writing-mode: vertical-rl; block-size: 10rem" lang="ja">縦書きの例です</p>
```

## Native CSS nesting

```css
.card {
  padding: 1rem;
  & h2 { margin-block-start: 0; }
  &:hover { box-shadow: 0 4px 12px rgb(0 0 0 / 0.1); }
  .dark & { background: #1e293b; }              /* parent selector after */
  @media (min-width: 40rem) { padding: 2rem; }  /* media queries nest too */
}
```

`&` refers to the parent selector. Keep nesting shallow (two levels): deep nesting recreates the specificity problems of long selectors.

## Other modern features worth knowing

- `:has()` (module 2) for parent-aware styles.
- `accent-color: #e11d48` restyles checkboxes, radios and range sliders in one line.
- `color-scheme: light dark` makes form controls and scrollbars follow dark mode; `light-dark(#fff, #111)` picks a colour per scheme.
- `scroll-snap-type` / `scroll-snap-align` for carousels without JavaScript.
- `@supports (selector(:has(*)))` to feature-test before using something new.
- The **popover** attribute (`<div popover id="menu">` + `<button popovertarget="menu">`) gives you menus and tooltips that open in the top layer, close on Escape and outside click, no JavaScript needed.

@@quiz
When should you use a container query instead of a media query?
* When a component should adapt to the space it's placed in, regardless of the screen size
- For print styles
- Only on mobile
- Never; they're experimental
? Media queries for page layout, container queries for components.

@@quiz
What does `margin-inline-start` mean in a right-to-left page?
- The left margin
* The right margin, because inline start is on the right in RTL
- The top margin
- Nothing; it only works in LTR
? Logical properties follow the writing direction.

@@quiz
Why check `(hover: hover)` before adding hover-only behaviour?
- Hover is slow
* Touch devices can't hover, so hover-only menus are unusable there
- It's required for animations
- It detects dark mode
? Provide click/tap behaviour too.

@@quiz
In native CSS nesting, what does `&:hover` inside `.card { }` mean?
- `.card :hover` (a hovered descendant)
* `.card:hover`
- `:hover .card`
- It's invalid
? `&` is replaced by the parent selector.

@@flashcards
`container-type: inline-size` :: make a size container
`@container (min-width: 28rem)` :: style by container width
`cqi` :: 1% of the container's inline size
`(hover: hover)` :: a device that can hover
`(40rem <= width <= 60rem)` :: range syntax
`margin-inline` :: left+right in LTR, follows direction
`inline-size` :: logical width
`text-align: start` :: logical alignment
`&` :: parent selector in nesting
`accent-color` :: style native checkboxes/radios
`color-scheme: light dark` :: native controls follow the scheme
`popover` / `popovertarget` :: declarative popups
`scroll-snap-type` :: CSS carousels

@@exercise A ticket card that fits anywhere
Make `.slot` a container (`container-type: inline-size`) and give the ticket card two layouts: stacked by default, and a three-column row (`grid-template-columns: 5rem 1fr auto`) when its container is at least `30rem` wide. Scale the title with container units: `font-size: clamp(1rem, 3cqi, 1.4rem)`.
@@starter
<!doctype html>
<html lang="en">
<head><meta charset="utf-8"><title>Ticket cards</title>
<style>
  body { font-family: system-ui; display: grid; grid-template-columns: 16rem 1fr; gap: 1rem; }
  .ticket { display: grid; gap: 0.25rem; background: #f8fafc; border: 1px solid #e2e8f0; padding: 0.75rem; border-radius: 8px; }
</style>
</head>
<body>
  <aside class="slot"><div class="ticket"><b>#9</b><span class="title">File server almost full</span><span>P1</span></div></aside>
  <main class="slot"><div class="ticket"><b>#9</b><span class="title">File server almost full</span><span>P1</span></div></main>
</body>
</html>
@@requires
container-type: inline-size
@container (min-width: 30rem)
grid-template-columns: 5rem 1fr auto
clamp(1rem, 3cqi, 1.4rem)
@@hint
`.slot { container-type: inline-size; } @container (min-width: 30rem) { .ticket { grid-template-columns: 5rem 1fr auto; align-items: center; } } .title { font-size: clamp(1rem, 3cqi, 1.4rem); }`
@@solution
<!doctype html>
<html lang="en">
<head><meta charset="utf-8"><title>Ticket cards</title>
<style>
  body { font-family: system-ui; display: grid; grid-template-columns: 16rem 1fr; gap: 1rem; }
  .slot { container-type: inline-size; }
  .ticket { display: grid; gap: 0.25rem; background: #f8fafc; border: 1px solid #e2e8f0; padding: 0.75rem; border-radius: 8px; }
  .title { font-size: clamp(1rem, 3cqi, 1.4rem); }
  @container (min-width: 30rem) {
    .ticket { grid-template-columns: 5rem 1fr auto; align-items: center; }
  }
</style>
</head>
<body>
  <aside class="slot"><div class="ticket"><b>#9</b><span class="title">File server almost full</span><span>P1</span></div></aside>
  <main class="slot"><div class="ticket"><b>#9</b><span class="title">File server almost full</span><span>P1</span></div></main>
</body>
</html>

@@exercise Logical properties for an international page
The notice component must work in English, Arabic (right to left) and vertical Japanese. Replace every physical property with its logical equivalent: `border-left` → `border-inline-start`, `padding-left`/`padding-right` → `padding-inline`, `margin-top`/`margin-bottom` → `margin-block`, `text-align: left` → `text-align: start`, and `width` → `inline-size`.
@@starter
<!doctype html>
<html lang="en">
<head><meta charset="utf-8"><title>Notices</title>
<style>
  .notice { border-left: 4px solid #f59e0b; padding-left: 1rem; padding-right: 1rem; margin-top: 0.5rem; margin-bottom: 0.5rem; text-align: left; width: 20rem; background: #fffbeb; }
</style>
</head>
<body>
  <p class="notice">Maintenance tonight from 22:00.</p>
  <p class="notice" dir="rtl" lang="ar">صيانة الليلة من الساعة 22:00.</p>
  <p class="notice" lang="ja" style="writing-mode: vertical-rl">今夜二十二時からメンテナンスです。</p>
</body>
</html>
@@requires
border-inline-start: 4px solid #f59e0b
padding-inline: 1rem
margin-block: 0.5rem
text-align: start
inline-size: 20rem
@@hint
`.notice { border-inline-start: 4px solid #f59e0b; padding-inline: 1rem; margin-block: 0.5rem; text-align: start; inline-size: 20rem; background: #fffbeb; }`
@@solution
<!doctype html>
<html lang="en">
<head><meta charset="utf-8"><title>Notices</title>
<style>
  .notice { border-inline-start: 4px solid #f59e0b; padding-inline: 1rem; margin-block: 0.5rem; text-align: start; inline-size: 20rem; background: #fffbeb; }
</style>
</head>
<body>
  <p class="notice">Maintenance tonight from 22:00.</p>
  <p class="notice" dir="rtl" lang="ar">صيانة الليلة من الساعة 22:00.</p>
  <p class="notice" lang="ja" style="writing-mode: vertical-rl">今夜二十二時からメンテナンスです。</p>
</body>
</html>

@@exercise A popover menu and nested styles
Build an account menu with **no JavaScript**: a button with `popovertarget="account-menu"` opens a `<div popover id="account-menu">` containing three links. Write the menu's CSS with native nesting: `.menu { ... & a { ... } & a:hover { ... } }`. Make hover styles apply only on devices that can hover (`@media (hover: hover)`), and set `accent-color` for the "Remember me" checkbox.
@@starter
<!doctype html>
<html lang="en">
<head><meta charset="utf-8"><title>Account</title></head>
<body>
  <button>Ana Silva ▾</button>
  <div class="menu">
    <a href="/profile">Profile</a>
    <a href="/devices">My devices</a>
    <a href="/logout">Sign out</a>
  </div>
  <label><input type="checkbox"> Remember me</label>
</body>
</html>
@@requires
popovertarget="account-menu"
popover id="account-menu"
& a
@media (hover: hover)
accent-color
@@hint
`<button popovertarget="account-menu">Ana Silva ▾</button><div popover id="account-menu" class="menu">...</div>`. CSS: `.menu { padding: 0.5rem; border-radius: 8px; & a { display: block; padding: 0.4rem 0.75rem; } @media (hover: hover) { & a:hover { background: #f1f5f9; } } } input[type="checkbox"] { accent-color: #e11d48; }`
@@solution
<!doctype html>
<html lang="en">
<head><meta charset="utf-8"><title>Account</title>
<style>
  .menu {
    padding: 0.5rem; border: 1px solid #e2e8f0; border-radius: 8px; min-inline-size: 12rem;
    & a { display: block; padding: 0.4rem 0.75rem; color: inherit; text-decoration: none; border-radius: 6px; }
    & a:focus-visible { outline: 3px solid #2563eb; }
    @media (hover: hover) {
      & a:hover { background: #f1f5f9; }
    }
  }
  input[type="checkbox"] { accent-color: #e11d48; }
</style>
</head>
<body>
  <button popovertarget="account-menu">Ana Silva ▾</button>
  <div popover id="account-menu" class="menu">
    <a href="/profile">Profile</a>
    <a href="/devices">My devices</a>
    <a href="/logout">Sign out</a>
  </div>
  <label><input type="checkbox"> Remember me</label>
</body>
</html>
