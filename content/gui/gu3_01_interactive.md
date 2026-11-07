@@lesson gu3-interactive Interactive Controls: States, Toggles, Sliders and Toasts
@@concepts conditions, input
@@text
A control feels *alive* when it answers every touch: it lights up when the pointer is over it, sinks when pressed, shows clearly where the keyboard is, and looks unavailable when it can't be used. This lesson builds the controls every app needs, in HTML and CSS (with a little JavaScript), and shows the same states on the desktop.

## The five states of a button

| State | When | Typical look |
|---|---|---|
| default | resting | the base style |
| `:hover` | the pointer is over it | a bit lighter or darker, maybe lifted |
| `:active` | being pressed | darker, pushed down (`scale(.97)` or `translateY(1px)`) |
| `:focus-visible` | reached with the keyboard | a clear outline ring |
| `:disabled` | can't be used | faded, `cursor: not-allowed`, no hover effect |

```html
<style>
  body { font-family: system-ui, sans-serif; padding: 24px; display: flex; gap: 12px; }
  .btn {
    --c: #7c3aed;
    background: var(--c); color: white; border: 0; border-radius: 10px;
    padding: 10px 18px; font: inherit; font-weight: 600; cursor: pointer;
    box-shadow: 0 2px 0 rgb(0 0 0 / .15);
    transition: background-color .15s, transform .1s, box-shadow .15s;
  }
  .btn:hover { background: #6d28d9; box-shadow: 0 4px 12px rgb(124 58 237 / .35); }
  .btn:active { transform: translateY(1px) scale(.98); box-shadow: 0 1px 0 rgb(0 0 0 / .15); }
  .btn:focus-visible { outline: 3px solid #c4b5fd; outline-offset: 2px; }
  .btn:disabled { opacity: .45; cursor: not-allowed; box-shadow: none; transform: none; background: var(--c); }
  .btn.ghost { background: transparent; color: #7c3aed; box-shadow: inset 0 0 0 2px #c4b5fd; }
  .btn.ghost:hover { background: #f5f3ff; }
</style>
<button class="btn">Save</button>
<button class="btn ghost">Cancel</button>
<button class="btn" disabled>Deploy</button>
```

Press **Tab** in the preview: the ring appears for keyboard users but not for mouse clicks. That's the difference between `:focus` (always) and `:focus-visible` (only when it helps). **Never** write `outline: none` without a replacement: keyboard users get lost.

**Hit targets**: make clickable things at least about 24 to 44 px tall, and put the padding on the button itself, not around it.

## Use real controls

A `<div onclick>` looks like a button but can't be reached with Tab, doesn't react to Enter or Space, and screen readers don't announce it. Use `<button>`, `<input type="checkbox">`, `<input type="range">`, `<select>`, `<dialog>`, and style them. You get keyboard support and accessibility for free. Desktop toolkits are the same: use the real `Button`, `CheckBox` and `Slider` and restyle them with templates or QSS.

## A toggle switch from a checkbox

A switch is just a checkbox in disguise: the real `<input>` stays (so Space toggles it and it's announced), it's made invisible, and a styled `<span>` follows its state with `:checked`.

```html
<style>
  body { font-family: system-ui, sans-serif; padding: 24px; }
  .switch { display: inline-flex; align-items: center; gap: 10px; cursor: pointer; }
  .switch input { position: absolute; opacity: 0; width: 1px; height: 1px; }
  .track { width: 44px; height: 26px; border-radius: 999px; background: #d4d0e0; position: relative; transition: background-color .2s; }
  .track::after {
    content: ""; position: absolute; top: 3px; left: 3px; width: 20px; height: 20px; border-radius: 50%;
    background: white; box-shadow: 0 1px 3px rgb(0 0 0 / .3);
    transition: transform .25s cubic-bezier(.34, 1.56, .64, 1);   /* a tiny overshoot: bouncy */
  }
  .switch input:checked + .track { background: #22c55e; }
  .switch input:checked + .track::after { transform: translateX(18px); }
  .switch input:focus-visible + .track { outline: 3px solid #bbf7d0; outline-offset: 2px; }
</style>
<label class="switch"><input type="checkbox" role="switch" checked><span class="track"></span> Auto-backup</label>
```

## Sliders, with a live value

```html
<style>
  body { font-family: system-ui, sans-serif; padding: 24px; }
  input[type=range] { accent-color: #7c3aed; width: 220px; }
  output { font-variant-numeric: tabular-nums; font-weight: 700; margin-left: 8px; }
</style>
<label>Volume <input id="vol" type="range" min="0" max="100" value="40"></label><output id="out">40</output>
<script>
  const vol = document.getElementById("vol"), out = document.getElementById("out");
  vol.addEventListener("input", () => out.textContent = vol.value);   // "input" fires while dragging
</script>
```

`accent-color` restyles native checkboxes, radios, ranges and progress bars in one line.

## Feedback: toasts, spinners and progress

People need to know their click did something, within about **100 ms**.

- **Instant change** (a toggle, a like): just update the control.
- **Short wait** (under ~1 s): disable the button and put a small **spinner** in it, so it can't be clicked twice.
- **Long wait**: a **progress bar** with real progress if you know it (`<progress value="40" max="100">`), otherwise an indeterminate one, and a Cancel button.
- **After it's done**: a **toast**, a small message that slides in, says "Saved ✓", and disappears after a few seconds. Important errors should stay until dismissed. Give the toast container `role="status"` (or `aria-live="polite"`) so screen readers announce it.

```html
<style>
  body { font-family: system-ui, sans-serif; padding: 24px; }
  .btn { background: #7c3aed; color: white; border: 0; border-radius: 10px; padding: 10px 18px; font: inherit; font-weight: 600; display: inline-flex; gap: 8px; align-items: center; }
  .btn:disabled { opacity: .7; }
  .spinner { width: 14px; height: 14px; border: 2px solid rgb(255 255 255 / .4); border-top-color: white; border-radius: 50%; animation: spin .7s linear infinite; }
  @keyframes spin { to { transform: rotate(360deg); } }
  #toasts { position: fixed; bottom: 16px; right: 16px; display: grid; gap: 8px; }
  .toast { background: #1e1b2e; color: white; padding: 10px 14px; border-radius: 10px; box-shadow: 0 8px 24px rgb(0 0 0 / .2);
           animation: toast-in .3s ease-out; }
  .toast.leaving { opacity: 0; transform: translateY(8px); transition: .3s; }
  @keyframes toast-in { from { opacity: 0; transform: translateY(12px) scale(.96); } }
</style>
<button class="btn" id="save">Save changes</button>
<div id="toasts" role="status"></div>
<script>
  const save = document.getElementById("save");
  save.addEventListener("click", () => {
    save.disabled = true;
    save.innerHTML = '<span class="spinner"></span> Saving…';
    setTimeout(() => {                                    // pretend to save
      save.disabled = false;
      save.textContent = "Save changes";
      toast("Saved ✓");
    }, 900);
  });
  function toast(text) {
    const t = document.createElement("div");
    t.className = "toast";
    t.textContent = text;
    document.getElementById("toasts").append(t);
    setTimeout(() => { t.classList.add("leaving"); setTimeout(() => t.remove(), 300); }, 2500);
  }
</script>
```

## Keyboard access checklist

- Everything clickable can be reached with **Tab** in a sensible order and used with **Enter** or **Space**.
- **Esc** closes dialogs and menus; focus goes back to the button that opened them.
- Shortcuts for frequent actions (Ctrl+S, Ctrl+F, Ctrl+N), shown in tooltips or menus.
- Arrow keys move inside groups (tabs, lists, radio buttons).
- Icon-only buttons have a text name: `<button aria-label="Delete ticket">🗑</button>` (WPF: `AutomationProperties.Name`).

## The same states on the desktop

```xml
<!-- WPF: a Style with triggers for hover, pressed and disabled -->
<Style TargetType="Button">
  <Setter Property="Background" Value="#7C3AED" />
  <Setter Property="Foreground" Value="White" />
  <Style.Triggers>
    <Trigger Property="IsMouseOver" Value="True"><Setter Property="Background" Value="#6D28D9" /></Trigger>
    <Trigger Property="IsPressed" Value="True"><Setter Property="Background" Value="#5B21B6" /></Trigger>
    <Trigger Property="IsEnabled" Value="False"><Setter Property="Opacity" Value="0.45" /></Trigger>
  </Style.Triggers>
</Style>
```

```text
/* Qt QSS: the same pseudo-states as CSS */
QPushButton         { background: #7c3aed; color: white; border-radius: 8px; padding: 8px 16px; }
QPushButton:hover   { background: #6d28d9; }
QPushButton:pressed { background: #5b21b6; }
QPushButton:disabled { background: #c4b5fd; }
```

In tkinter, `ttk.Style().map("Accent.TButton", background=[("pressed", "#5b21b6"), ("active", "#6d28d9")])` maps states to colours.

@@quiz
What's the difference between `:focus` and `:focus-visible`?
- None
* `:focus-visible` shows only when the browser thinks a focus ring helps (keyboard use), not on every mouse click
- `:focus-visible` is for images only
- `:focus` only works in dark mode
? Style `:focus-visible` with a clear outline, and never remove focus outlines without a replacement.

@@quiz
Why build a toggle switch from a hidden real `<input type="checkbox">` instead of a `<div>`?
- Divs can't be round
* The checkbox gives keyboard support (Space), form value and screen-reader announcements for free
- Checkboxes animate faster
- It uses less CSS
? `:checked + .track` lets the styled span follow the real control's state.

@@quiz
A "Save" request takes about 800 ms. What's the best feedback?
- Nothing, it's fast
* Disable the button with a small spinner, then show a "Saved ✓" toast
- A full-screen loading overlay
- An alert box saying "Please wait"
? Disabling prevents double submits; the toast confirms it worked without interrupting.

@@quiz
Which ARIA setting lets a screen reader announce a toast?
- `aria-hidden="true"`
* `role="status"` (or `aria-live="polite"`) on the container
- `tabindex="-1"`
- `role="presentation"`
? Live regions are read out when their content changes.

@@quiz
In the slider example, why listen to `input` rather than `change`?
- `change` doesn't exist for ranges
* `input` fires continuously while dragging; `change` only when the user lets go
- `input` is faster to type
- They're identical
? Use `input` for live previews and `change` for expensive work like saving.

@@drill
Which CSS pseudo-class styles a control only when it's focused from the keyboard?
= :focus-visible
= focus-visible
? Mouse clicks don't show the ring.

@@flashcards
`:hover` / `:active` :: pointer over it / being pressed
`:focus-visible` :: a focus ring only when the keyboard is used
`:disabled` :: the control can't be used
`outline: none` :: never without a visible replacement
`:checked + .track` :: style a sibling based on a checkbox's state
`role="switch"` :: tells assistive tech a checkbox is an on/off switch
`accent-color` :: recolour native checkboxes, radios, ranges and progress bars
`input` vs `change` event :: while dragging/typing vs when finished
toast :: a small message that appears briefly to confirm an action
`role="status"` / `aria-live="polite"` :: announce changing content to screen readers
`<progress value max>` :: a native progress bar
`aria-label` :: a text name for an icon-only button
WPF `Style.Triggers` :: change setters on IsMouseOver, IsPressed, IsEnabled
QSS `:hover` / `:pressed` :: Qt's button states

@@exercise A button with every state
Style `.btn` so it has all five states: a `transition`, a `:hover` background, an `:active` rule that uses `transform`, a `:focus-visible` rule with an `outline`, and a `:disabled` rule with `opacity` and `cursor: not-allowed`.
@@starter
<!doctype html>
<html lang="en">
<head>
  <meta charset="utf-8">
  <title>Buttons</title>
  <style>
    body { font-family: system-ui, sans-serif; padding: 32px; display: flex; gap: 12px; }
    .btn { background: #ec4899; color: white; border: 0; border-radius: 12px; padding: 10px 20px; font: inherit; font-weight: 600; cursor: pointer; }
  </style>
</head>
<body>
  <button class="btn">Adopt a cat 🐱</button>
  <button class="btn" disabled>Sold out</button>
</body>
</html>
@@requires
transition:
.btn:hover
.btn:active
transform:
.btn:focus-visible
outline:
.btn:disabled
opacity:
cursor: not-allowed
@@hint
`.btn:hover { background: #db2777; }`, `.btn:active { transform: scale(.97); }`, `.btn:focus-visible { outline: 3px solid #fbcfe8; outline-offset: 2px; }`, `.btn:disabled { opacity: .45; cursor: not-allowed; }`, and `transition: background-color .15s, transform .1s;` on `.btn`.
@@solution
<!doctype html>
<html lang="en">
<head>
  <meta charset="utf-8">
  <title>Buttons</title>
  <style>
    body { font-family: system-ui, sans-serif; padding: 32px; display: flex; gap: 12px; }
    .btn { background: #ec4899; color: white; border: 0; border-radius: 12px; padding: 10px 20px; font: inherit; font-weight: 600; cursor: pointer;
           transition: background-color .15s, transform .1s; }
    .btn:hover { background: #db2777; }
    .btn:active { transform: scale(.97); }
    .btn:focus-visible { outline: 3px solid #fbcfe8; outline-offset: 2px; }
    .btn:disabled { opacity: .45; cursor: not-allowed; background: #ec4899; transform: none; }
  </style>
</head>
<body>
  <button class="btn">Adopt a cat 🐱</button>
  <button class="btn" disabled>Sold out</button>
</body>
</html>

@@exercise A toggle switch
Turn the checkbox into a switch. Hide the real input visually (`opacity: 0`, keep it in the page), give `.track` a pill shape with `border-radius: 999px`, draw the knob with `.track::after`, and when the input is checked (`input:checked + .track`) change the track colour and move the knob with `translateX`. Add `role="switch"` to the input, and a `:focus-visible + .track` outline.
@@starter
<!doctype html>
<html lang="en">
<head>
  <meta charset="utf-8">
  <title>Switch</title>
  <style>
    body { font-family: system-ui, sans-serif; padding: 32px; }
    .switch { display: inline-flex; align-items: center; gap: 10px; cursor: pointer; }
  </style>
</head>
<body>
  <label class="switch">
    <input type="checkbox">
    <span class="track"></span>
    Night mode
  </label>
</body>
</html>
@@requires
role="switch"
opacity: 0
border-radius: 999px
.track::after
input:checked + .track
translateX(
input:focus-visible + .track
@@hint
`.track { width: 44px; height: 26px; border-radius: 999px; background: #d4d0e0; position: relative; }`, `.track::after { content: ""; position: absolute; top: 3px; left: 3px; width: 20px; height: 20px; border-radius: 50%; background: white; transition: transform .2s; }`, then `.switch input:checked + .track::after { transform: translateX(18px); }`.
@@solution
<!doctype html>
<html lang="en">
<head>
  <meta charset="utf-8">
  <title>Switch</title>
  <style>
    body { font-family: system-ui, sans-serif; padding: 32px; }
    .switch { display: inline-flex; align-items: center; gap: 10px; cursor: pointer; }
    .switch input { position: absolute; opacity: 0; width: 1px; height: 1px; }
    .track { width: 44px; height: 26px; border-radius: 999px; background: #d4d0e0; position: relative; transition: background-color .2s; }
    .track::after { content: ""; position: absolute; top: 3px; left: 3px; width: 20px; height: 20px; border-radius: 50%;
                    background: white; box-shadow: 0 1px 3px rgb(0 0 0 / .3); transition: transform .25s cubic-bezier(.34, 1.56, .64, 1); }
    .switch input:checked + .track { background: #6366f1; }
    .switch input:checked + .track::after { transform: translateX(18px); }
    .switch input:focus-visible + .track { outline: 3px solid #c7d2fe; outline-offset: 2px; }
  </style>
</head>
<body>
  <label class="switch">
    <input type="checkbox" role="switch">
    <span class="track"></span>
    Night mode
  </label>
</body>
</html>

@@exercise Toast notifications
Make the button show a toast. Add a container `<div id="toasts" role="status">`. In the script, on click, create a `div` with class `toast` and the text `Copied!`, `append` it to the container, and remove it with `setTimeout` after 2 seconds. Give `.toast` an entrance animation with `@keyframes`.
@@starter
<!doctype html>
<html lang="en">
<head>
  <meta charset="utf-8">
  <title>Toasts</title>
  <style>
    body { font-family: system-ui, sans-serif; padding: 32px; }
    button { font: inherit; padding: 8px 14px; border-radius: 8px; border: 1px solid #ccc; background: white; cursor: pointer; }
  </style>
</head>
<body>
  <button id="copy">Copy invite link</button>
  <script>
    // TODO
  </script>
</body>
</html>
@@requires
id="toasts"
role="status"
@keyframes
addEventListener("click"
createElement("div")
"toast"
append(
setTimeout(
@@hint
`document.getElementById("copy").addEventListener("click", () => { const t = document.createElement("div"); t.className = "toast"; t.textContent = "Copied!"; document.getElementById("toasts").append(t); setTimeout(() => t.remove(), 2000); });`
@@solution
<!doctype html>
<html lang="en">
<head>
  <meta charset="utf-8">
  <title>Toasts</title>
  <style>
    body { font-family: system-ui, sans-serif; padding: 32px; }
    button { font: inherit; padding: 8px 14px; border-radius: 8px; border: 1px solid #ccc; background: white; cursor: pointer; }
    #toasts { position: fixed; bottom: 16px; right: 16px; display: grid; gap: 8px; }
    .toast { background: #1e1b2e; color: white; padding: 10px 14px; border-radius: 10px; animation: pop-in .3s ease-out; }
    @keyframes pop-in { from { opacity: 0; transform: translateY(12px) scale(.95); } }
  </style>
</head>
<body>
  <button id="copy">Copy invite link</button>
  <div id="toasts" role="status"></div>
  <script>
    document.getElementById("copy").addEventListener("click", () => {
      const t = document.createElement("div");
      t.className = "toast";
      t.textContent = "Copied!";
      document.getElementById("toasts").append(t);
      setTimeout(() => t.remove(), 2000);
    });
  </script>
</body>
</html>
