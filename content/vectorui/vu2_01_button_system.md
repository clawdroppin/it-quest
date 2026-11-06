@@lesson vu2-buttons Designing a Button System: Variants, Sizes, States and Motion
@@concepts hello, files
@@lang html
@@text
Buttons are the most-used component in any app, and the clearest sign of whether a design system is good. This lesson designs a complete button family in Penpot (variants, sizes, states, icons) and builds it as real, interactive, animated CSS.

## Anatomy of a button

```text
 ┌──────────────────────────────┐
 │  [icon]  8px  Label text      │   height 40 (M), padding 0 16, radius 10
 └──────────────────────────────┘
```

- **Container**: fill, radius, border, shadow, min width.
- **Label**: one line, 14–16 px, weight 600, sentence case ("Save changes", not "SAVE CHANGES").
- **Icon** (optional): leading or trailing, 16–20 px, same colour as the label.
- **Hit area**: at least 40 × 40 on desktop, 44 × 44 on touch screens, even if the visible button is smaller.

## Variants by importance

| Variant | Use | Look |
|---|---|---|
| **Primary** | the one main action of a screen ("Save", "Buy") | solid accent fill, white label |
| **Secondary** | other actions | neutral fill or outline |
| **Ghost / tertiary** | low-importance actions, toolbars | no fill, accent or text colour; fill appears on hover |
| **Destructive** | delete, remove | red fill (or red ghost with confirmation/undo) |
| **Icon-only** | toolbars, compact UIs | square or circle, needs a tooltip and an accessible label |

One primary button per view. If everything is primary, nothing is.

## Sizes

| Size | Height | Padding | Font | Icon |
|---|---|---|---|---|
| S | 32 | 0 12 | 13 | 16 |
| M | 40 | 0 16 | 14 | 18 |
| L | 48 | 0 20 | 16 | 20 |

Keep the radius proportional or constant across sizes, but consistent with inputs of the same height so buttons and fields line up in forms.

## States

Every variant needs: **default, hover, pressed (active), focus-visible, disabled, loading**. In Penpot, build them as variants of one component: `Type × Size × State`. Design hover as a small step (accent 8–12% darker or lighter), pressed as a bigger step plus a slight shrink, focus as a 2–3 px ring with a gap (offset), disabled as reduced opacity with no shadow.

## Motion

Micro-interactions make buttons feel responsive:

- **Hover**: colour and a 1–2 px lift with a bigger soft shadow, 120–150 ms ease-out.
- **Press**: scale to 0.96–0.98 and drop the shadow, 60–100 ms.
- **Ripple** or **glow** from the pointer (CSS custom properties updated from JavaScript with the pointer position).
- **Loading**: label fades, a spinner appears, the width stays the same (so the layout doesn't jump).
- **Success**: briefly turns green with a check mark.
- Always respect `prefers-reduced-motion`.

In Penpot's prototype mode you can show hover states with *mouse enter / leave* interactions that swap variants, and *smart animate*-like transitions with dissolve and easing. In code, CSS does it with `transition` and `:hover`/`:active`.

## From Penpot to code

Penpot's Inspect gives the CSS for each variant. Turn the repeated values into **CSS custom properties** (variables) so each variant only changes a few of them:

```text
.btn            { --bg: var(--accent); --fg: white; background: var(--bg); color: var(--fg); ... }
.btn.secondary  { --bg: var(--surface-alt); --fg: var(--text); }
.btn.ghost      { --bg: transparent; --fg: var(--accent); }
.btn:hover      { filter: brightness(1.08); }      /* or a --bg-hover variable */
```

The exercises build the family, then add the motion.

@@quiz
How many primary buttons should one screen normally have?
* One, for the main action
- At least three
- As many as there are actions
- None; use only ghost buttons
? Hierarchy tells users what to do next.

@@quiz
What minimum touch target should a button have on a phone?
- 16 × 16 px
- 24 × 24 px
* About 44 × 44 px
- 100 × 100 px
? On desktop 40 × 40 is a good minimum.

@@quiz
Why keep a loading button the same width?
* So the layout doesn't jump when the label is replaced by a spinner
- Because CSS can't change width
- To save memory
- Spinners need fixed widths
? Fade the label out but keep its space.

@@quiz
In Penpot, how do you organise Primary/Secondary × S/M/L × Default/Hover/Pressed/Disabled?
- 36 separate unrelated components
* One component with variants and three properties: Type, Size and State
- One group with hidden layers
- Separate pages per state
? Copies can then switch properties in the sidebar.

@@quiz
How should a disabled button differ from an enabled one?
* Visibly muted, not focusable or clickable, and ideally with nearby text explaining why it is disabled
- Hidden completely
- Exactly the same, so the layout doesn't shift
- Red, to signal an error
? Disabled states that don't explain themselves frustrate users; sometimes it's better to keep the button enabled and validate on click.

@@flashcards
primary / secondary / ghost / destructive :: button variants by importance
icon-only button :: needs a tooltip and an accessible label
S / M / L heights :: 32 / 40 / 48 px
hit area :: at least 40×40 desktop, 44×44 touch
button states :: default, hover, pressed, focus-visible, disabled, loading
hover motion :: colour plus small lift, ~120–150 ms ease-out
press motion :: scale 0.96–0.98, 60–100 ms
:focus-visible :: focus ring for keyboard users only
CSS custom properties :: variables variants override
prefers-reduced-motion :: turn motion down for users who ask

@@exercise A button family with CSS variables
Build `.btn` using custom properties: `--bg` and `--fg` set on `.btn` and used for `background` and `color`. Add variants `.btn.secondary`, `.btn.ghost` and `.btn.danger` that only change the variables, and sizes `.btn.sm` (height 32px) and `.btn.lg` (height 48px) on top of the default 40px. Disabled buttons (`.btn:disabled`) get `opacity` and `cursor: not-allowed`.
@@starter
<!doctype html>
<html lang="en">
<head>
  <meta charset="utf-8">
  <title>Button family</title>
  <style>
    :root { --accent: #6366f1; --danger: #dc2626; --surface-alt: #eef2ff; --text: #1e1b4b; }
    body { font-family: system-ui, sans-serif; padding: 32px; display: flex; flex-wrap: wrap; gap: 12px; align-items: center; }
    /* Your CSS here */
  </style>
</head>
<body>
  <button class="btn">Save</button>
  <button class="btn secondary">Cancel</button>
  <button class="btn ghost">Learn more</button>
  <button class="btn danger">Delete</button>
  <button class="btn sm">Small</button>
  <button class="btn lg">Large</button>
  <button class="btn" disabled>Disabled</button>
</body>
</html>
@@requires
.btn {
--bg:
--fg:
background: var(--bg)
color: var(--fg)
height: 40px
.btn.secondary
.btn.ghost
.btn.danger
.btn.sm
height: 32px
.btn.lg
height: 48px
.btn:disabled
cursor: not-allowed
@@hint
`.btn { --bg: var(--accent); --fg: #fff; background: var(--bg); color: var(--fg); height: 40px; padding: 0 16px; border: 0; border-radius: 10px; font: 600 14px system-ui; cursor: pointer; }`, then `.btn.secondary { --bg: var(--surface-alt); --fg: var(--text); }` and so on.
@@solution
<!doctype html>
<html lang="en">
<head>
  <meta charset="utf-8">
  <title>Button family</title>
  <style>
    :root { --accent: #6366f1; --danger: #dc2626; --surface-alt: #eef2ff; --text: #1e1b4b; }
    body { font-family: system-ui, sans-serif; padding: 32px; display: flex; flex-wrap: wrap; gap: 12px; align-items: center; }
    .btn { --bg: var(--accent); --fg: #fff; background: var(--bg); color: var(--fg); height: 40px; padding: 0 16px;
           border: 0; border-radius: 10px; font: 600 14px system-ui, sans-serif; cursor: pointer; }
    .btn.secondary { --bg: var(--surface-alt); --fg: var(--text); }
    .btn.ghost { --bg: transparent; --fg: var(--accent); }
    .btn.danger { --bg: var(--danger); --fg: #fff; }
    .btn.sm { height: 32px; padding: 0 12px; font-size: 13px; }
    .btn.lg { height: 48px; padding: 0 20px; font-size: 16px; }
    .btn:disabled { opacity: .45; cursor: not-allowed; }
  </style>
</head>
<body>
  <button class="btn">Save</button>
  <button class="btn secondary">Cancel</button>
  <button class="btn ghost">Learn more</button>
  <button class="btn danger">Delete</button>
  <button class="btn sm">Small</button>
  <button class="btn lg">Large</button>
  <button class="btn" disabled>Disabled</button>
</body>
</html>

@@exercise Animated states and a glow that follows the pointer
Bring the button to life. Give `.btn` a `transition` for `transform`, `box-shadow` and `background-color`. On `:hover` (but not when disabled: use `.btn:not(:disabled):hover`) lift it with `transform: translateY(-2px)` and a bigger `box-shadow`; on `:active` press it with `scale(0.97)`; add a `:focus-visible` outline with `outline-offset`. Add a radial glow: a `::after` pseudo-element whose `background` is a `radial-gradient` centred at `var(--x) var(--y)`, and a script that sets `--x` and `--y` from the pointer on `pointermove`. Turn the motion off inside `@media (prefers-reduced-motion: reduce)`.
@@starter
<!doctype html>
<html lang="en">
<head>
  <meta charset="utf-8">
  <title>Animated button</title>
  <style>
    body { font-family: system-ui, sans-serif; padding: 48px; background: #0f172a; display: flex; gap: 16px; }
    .btn { position: relative; overflow: hidden; background: #6366f1; color: #fff; height: 44px; padding: 0 22px; border: 0;
           border-radius: 12px; font: 600 15px system-ui, sans-serif; cursor: pointer; }
    /* Your CSS here */
  </style>
</head>
<body>
  <button class="btn">Get started ✨</button>
  <button class="btn" disabled>Not yet</button>
  <script>
    // Your script here
  </script>
</body>
</html>
@@requires
transition:
.btn:not(:disabled):hover
translateY(-2px)
box-shadow:
.btn:not(:disabled):active
scale(0.97)
.btn:focus-visible
outline-offset
::after
radial-gradient
var(--x) var(--y)
pointermove
setProperty('--x'
prefers-reduced-motion: reduce
@@hint
`.btn::after { content: ""; position: absolute; inset: 0; background: radial-gradient(120px circle at var(--x) var(--y), rgb(255 255 255 / .35), transparent 60%); opacity: 0; transition: opacity .2s; }` and `.btn:hover::after { opacity: 1; }`. In the script: `btn.addEventListener('pointermove', e => { const r = btn.getBoundingClientRect(); btn.style.setProperty('--x', (e.clientX - r.left) + 'px'); ... })` for each button.
@@solution
<!doctype html>
<html lang="en">
<head>
  <meta charset="utf-8">
  <title>Animated button</title>
  <style>
    body { font-family: system-ui, sans-serif; padding: 48px; background: #0f172a; display: flex; gap: 16px; }
    .btn { position: relative; overflow: hidden; background: #6366f1; color: #fff; height: 44px; padding: 0 22px; border: 0;
           border-radius: 12px; font: 600 15px system-ui, sans-serif; cursor: pointer;
           box-shadow: 0 2px 6px rgb(0 0 0 / .25);
           transition: transform .14s ease-out, box-shadow .14s ease-out, background-color .14s ease-out; }
    .btn:not(:disabled):hover { transform: translateY(-2px); box-shadow: 0 10px 24px rgb(99 102 241 / .45); background-color: #7275f5; }
    .btn:not(:disabled):active { transform: translateY(0) scale(0.97); box-shadow: 0 1px 3px rgb(0 0 0 / .3); transition-duration: .06s; }
    .btn:focus-visible { outline: 3px solid #a5b4fc; outline-offset: 3px; }
    .btn:disabled { opacity: .45; cursor: not-allowed; }
    .btn::after { content: ""; position: absolute; inset: 0; pointer-events: none;
                  background: radial-gradient(120px circle at var(--x) var(--y), rgb(255 255 255 / .35), transparent 60%);
                  opacity: 0; transition: opacity .2s; }
    .btn:not(:disabled):hover::after { opacity: 1; }
    @media (prefers-reduced-motion: reduce) {
      .btn, .btn::after { transition: none; }
      .btn:not(:disabled):hover, .btn:not(:disabled):active { transform: none; }
    }
  </style>
</head>
<body>
  <button class="btn">Get started ✨</button>
  <button class="btn" disabled>Not yet</button>
  <script>
    for (const btn of document.querySelectorAll('.btn')) {
      btn.addEventListener('pointermove', (e) => {
        const r = btn.getBoundingClientRect();
        btn.style.setProperty('--x', (e.clientX - r.left) + 'px');
        btn.style.setProperty('--y', (e.clientY - r.top) + 'px');
      });
    }
  </script>
</body>
</html>
