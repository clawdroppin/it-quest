@@lesson vu2-form-controls Designing Form Controls: Text Fields, Checkboxes, Switches and Their States
@@concepts strings, conditions
@@lang html
@@text
After buttons, form controls are the components every app needs, and the ones most often designed badly: placeholder text used as a label, error messages only in red, focus rings removed, custom checkboxes that can't be reached with the keyboard. This lesson designs a text field, a checkbox and a switch the right way, keeping the native elements underneath and styling them.

## Text fields

- A **visible label** above the field, always. Placeholder text disappears when you type, is low-contrast, and isn't a label.
- **Helper text** below the field explains the format. **Error text** replaces it, and is linked to the input with `aria-describedby` and announced with `aria-invalid="true"`.
- States: default, hover, **focus** (a clear ring: 2–3 px, contrast at least 3:1), filled, **error** (colour **plus** an icon or text, never colour alone), disabled.
- Size: at least 44 px tall for touch, with 16 px or larger text (smaller text makes iOS zoom in).

## Checkboxes and switches

- Keep the native `<input type="checkbox">`, and restyle it with `appearance: none` plus `:checked`, `:focus-visible` and `:disabled` styles. It stays keyboard- and screen-reader-accessible for free.
- **`accent-color`** is the one-line way to brand native checkboxes, radios and range sliders.
- **Checkbox** vs **switch**: a checkbox is a choice applied when you submit a form. A switch takes **effect immediately** (like Wi-Fi on/off). A switch is a checkbox with `role="switch"`. Screen readers then say "on/off" instead of "checked".
- Labels should be clickable: wrap the input in `<label>` or use `for`/`id`.

## Design tokens for controls

Keep control colours in the same token layers as buttons: `--control-border`, `--control-border-hover`, `--control-focus`, `--control-error`, `--control-bg-disabled`. Then dark mode and theming work for every control at once.

## Validation that helps

How and when a form reports errors matters as much as how its controls look:

- **Validate on blur, not on every keystroke.** Telling someone their email is invalid after they've typed the first letter is noise. Check when they leave the field, and then re-check as they type once an error is showing, so the message disappears the moment it's fixed.
- **Put the message next to the field**, below it, in text, with an icon and colour as extra cues. Link it to the input with `aria-describedby` so screen readers read it, and set `aria-invalid="true"` on the field.
- **Say how to fix it.** "Enter a date like 2026-11-06" beats "Invalid date".
- **On submit, move focus to the first error** (or to an error summary at the top that links to each field), so keyboard and screen-reader users aren't left guessing why nothing happened.
- **Don't clear what the user typed** when there's an error. Re-entering a long form because one field was wrong is the fastest way to lose people.

## Design states for every control

When you design a form control in Penpot, draw the full set of states before handing it off: **default, hover, focus, filled, disabled, read-only, error and success**. Each needs a clear difference, and the focus state needs a visible ring with at least 3:1 contrast against the background. Put the states in a variant matrix so developers can see them all at once and nothing is invented during implementation.

@@quiz
Why isn't placeholder text a substitute for a label?
* It disappears when the user types and is usually low contrast
- Screen readers can't read HTML
- Labels are deprecated
- Placeholders can't be styled
? Always show a visible label.

@@quiz
How should an error state be shown?
- Red border only
* Colour plus an icon or message text, linked with aria-describedby and aria-invalid
- A browser alert
- By disabling the field
? Never rely on colour alone.

@@quiz
When should you use a switch instead of a checkbox?
* When the setting takes effect immediately
- When there are more than two options
- When the form is long
- Always on mobile
? Checkboxes are applied on submit.

@@quiz
What is the simplest way to brand native checkboxes and radio buttons?
- Replace them with divs
* accent-color
- background-image
- A JavaScript library
? It keeps all the native behaviour.

@@quiz
Why use text of at least 16 px in inputs on mobile?
* Smaller text makes iOS Safari zoom in on focus
- Android requires it
- It improves contrast automatically
- It makes the field taller
? A common mobile annoyance.

@@flashcards
visible label :: always above or beside the field
placeholder :: an example, not a label
helper text :: format hint below the field
aria-describedby :: link the help or error text to the input
aria-invalid :: marks a field with an error
focus ring :: 2–3 px, at least 3:1 contrast
appearance: none :: restyle native controls
accent-color :: brand native checkboxes and radios
role="switch" :: checkbox announced as on/off
switch vs checkbox :: immediate vs on submit
44 px :: minimum touch target height
16 px input text :: avoids iOS zoom

@@drill
Which attribute links an input to its error message?
= aria-describedby
? The id of the message element goes in it.

@@drill
Which ARIA role makes a checkbox announce as on/off?
= switch
= role="switch"
? Use it for settings that apply immediately.

@@exercise A text field with helper and error states
Build an email field with a visible label, helper text, and an error state shown on a second field. The error field needs `aria-invalid="true"`, its message linked with `aria-describedby`, an icon or text in addition to colour, and a visible `:focus-visible` ring on both inputs. Inputs must be at least 44 px tall with 16 px text.
@@starter
<!doctype html>
<html lang="en">
<head>
  <meta charset="utf-8">
  <title>Text fields</title>
  <style>
    :root { --control-border: #94a3b8; --control-focus: #2563eb; --control-error: #b91c1c; }
    body { font-family: system-ui, sans-serif; padding: 32px; max-width: 420px; color: #0f172a; }
  </style>
</head>
<body>
  <input type="email" placeholder="Email">
</body>
</html>
@@requires
<label for="email"
aria-describedby="email-help"
aria-invalid="true"
aria-describedby="email2-error"
:focus-visible
min-height: 44px
font-size: 16px
@@hint
Pair each `<label for="...">` with an input `id`. Give the helper `<p id="email-help">` and point the first input at it. The second field gets `aria-invalid="true" aria-describedby="email2-error"` and a `<p id="email2-error">` containing a warning symbol and text. Style `input:focus-visible { outline: 3px solid var(--control-focus); }`.
@@solution
<!doctype html>
<html lang="en">
<head>
  <meta charset="utf-8">
  <title>Text fields</title>
  <style>
    :root { --control-border: #94a3b8; --control-focus: #2563eb; --control-error: #b91c1c; }
    body { font-family: system-ui, sans-serif; padding: 32px; max-width: 420px; color: #0f172a; }
    .field { display: grid; gap: 6px; margin-bottom: 24px; }
    label { font-weight: 600; }
    input {
      min-height: 44px; font-size: 16px; padding: 0 12px;
      border: 1px solid var(--control-border); border-radius: 8px; background: #fff; color: inherit;
    }
    input:hover { border-color: #64748b; }
    input:focus-visible { outline: 3px solid var(--control-focus); outline-offset: 1px; border-color: var(--control-focus); }
    input[aria-invalid="true"] { border: 2px solid var(--control-error); }
    .help { margin: 0; font-size: 14px; color: #475569; }
    .error { margin: 0; font-size: 14px; color: var(--control-error); font-weight: 600; }
  </style>
</head>
<body>
  <div class="field">
    <label for="email">Email</label>
    <input id="email" type="email" autocomplete="email" placeholder="name@example.com" aria-describedby="email-help">
    <p class="help" id="email-help">We'll send the receipt here.</p>
  </div>
  <div class="field">
    <label for="email2">Work email</label>
    <input id="email2" type="email" value="kenji@" aria-invalid="true" aria-describedby="email2-error">
    <p class="error" id="email2-error"><span aria-hidden="true">⚠ </span>Enter a full address, like kenji@example.co.jp.</p>
  </div>
</body>
</html>

@@exercise Styled checkbox and an accessible switch
Restyle native checkboxes with `appearance: none` (with checked, focus and disabled states), and build a settings switch from a checkbox with `role="switch"`. The switch's track and thumb are drawn with CSS, the thumb moves when checked, the motion respects `prefers-reduced-motion`, and every control has a clickable label.
@@starter
<!doctype html>
<html lang="en">
<head>
  <meta charset="utf-8">
  <title>Checkboxes and switches</title>
  <style>
    :root { --accent: #4f46e5; --border: #94a3b8; }
    body { font-family: system-ui, sans-serif; padding: 32px; color: #0f172a; display: grid; gap: 16px; }
  </style>
</head>
<body>
  <label><input type="checkbox"> Email me about updates</label>
  <label><input type="checkbox"> Wi-Fi</label>
</body>
</html>
@@requires
appearance: none
:checked
:focus-visible
:disabled
role="switch"
prefers-reduced-motion
<label
@@hint
For the checkbox: `appearance: none; width: 20px; height: 20px; border: 2px solid var(--border); border-radius: 4px;`, and `:checked` gets the accent background with a check drawn by a `::before` or a background SVG. For the switch: `appearance: none; width: 44px; height: 24px; border-radius: 999px;` with a `::before` thumb that uses `transform: translateX(20px)` when `:checked`.
@@solution
<!doctype html>
<html lang="en">
<head>
  <meta charset="utf-8">
  <title>Checkboxes and switches</title>
  <style>
    :root { --accent: #4f46e5; --border: #94a3b8; }
    body { font-family: system-ui, sans-serif; padding: 32px; color: #0f172a; display: grid; gap: 16px; }
    label { display: inline-flex; align-items: center; gap: 10px; min-height: 44px; cursor: pointer; }
    input[type="checkbox"] { appearance: none; margin: 0; flex: none; cursor: pointer; }
    input.check { width: 20px; height: 20px; border: 2px solid var(--border); border-radius: 4px; display: grid; place-content: center; }
    input.check::before { content: ""; width: 10px; height: 6px; border: solid #fff; border-width: 0 0 2px 2px; transform: rotate(-45deg) translate(1px, -1px); opacity: 0; }
    input.check:checked { background: var(--accent); border-color: var(--accent); }
    input.check:checked::before { opacity: 1; }
    input.switch { width: 44px; height: 24px; border-radius: 999px; background: #cbd5e1; position: relative; transition: background 160ms ease; }
    input.switch::before { content: ""; position: absolute; top: 2px; left: 2px; width: 20px; height: 20px; border-radius: 50%; background: #fff; box-shadow: 0 1px 3px rgb(0 0 0 / 0.3); transition: transform 160ms ease; }
    input.switch:checked { background: var(--accent); }
    input.switch:checked::before { transform: translateX(20px); }
    input:focus-visible { outline: 3px solid #2563eb; outline-offset: 2px; }
    input:disabled { opacity: 0.5; cursor: not-allowed; }
    input:disabled + span { color: #64748b; }
    @media (prefers-reduced-motion: reduce) {
      input.switch, input.switch::before { transition: none; }
    }
  </style>
</head>
<body>
  <label><input class="check" type="checkbox" checked> <span>Email me about updates</span></label>
  <label><input class="check" type="checkbox" disabled> <span>Beta features (not available)</span></label>
  <label><input class="switch" type="checkbox" role="switch" checked> <span>Wi-Fi</span></label>
  <label><input class="switch" type="checkbox" role="switch"> <span>Bluetooth</span></label>
</body>
</html>
