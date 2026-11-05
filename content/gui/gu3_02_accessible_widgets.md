@@lesson gu3-accessible-widgets Accessible Widgets: Dialogs, Tabs and Keyboard Patterns
@@text
Native controls (buttons, checkboxes, `<select>`, `<dialog>`) come with keyboard support and screen-reader semantics for free. Custom widgets don't: if you build tabs, menus or a modal from `<div>`s, you must add the roles, states and keyboard behaviour yourself. The **WAI-ARIA Authoring Practices Guide (APG)** documents the expected pattern for each widget. This lesson covers the rules and two of the most common patterns: modal dialogs and tabs.

## The rules of ARIA

1. **Use a native element** if one exists. `<button>` beats `<div role="button" tabindex="0">` with key handlers.
2. Don't change native semantics unnecessarily (`<h2 role="button">`).
3. Every interactive ARIA control must be **keyboard operable**.
4. Don't hide focusable elements (`aria-hidden="true"` on something focusable confuses users).
5. Every interactive element needs an **accessible name** (visible label, `aria-label`, or `aria-labelledby`).
"No ARIA is better than bad ARIA": wrong roles make things worse than none.

## Roles, states and properties

- **Role**: what it is (`dialog`, `tablist`, `tab`, `tabpanel`, `menu`, `switch`, `alert`).
- **States** change: `aria-expanded`, `aria-selected`, `aria-checked`, `aria-pressed`, `aria-disabled`, `aria-invalid`.
- **Properties** relate things: `aria-controls`, `aria-labelledby`, `aria-describedby`, `aria-modal`.
- **Live regions** announce changes: `role="status"` (polite) or `role="alert"` (assertive), or `aria-live`.

## Modal dialogs

- Use `<dialog>` with `showModal()`: the browser makes the rest of the page **inert**, traps focus, closes on Escape and returns focus to the opener in modern browsers. Give it an accessible name (`aria-labelledby` pointing at its heading).
- If you build one yourself: `role="dialog"`, `aria-modal="true"`, move focus into it on open, **trap** Tab and Shift+Tab inside, close on Escape, make the background `inert`, and **return focus** to the button that opened it.

## Tabs

`role="tablist"` containing `role="tab"` buttons, each with `aria-selected` and `aria-controls` pointing at a `role="tabpanel"` (labelled by its tab). Keyboard: **only the selected tab is in the Tab order** (`tabindex="0"`, others `-1`: a *roving tabindex*); **Left/Right arrows** move between tabs (wrapping), **Home/End** jump to first/last. Activation can be automatic (on focus) or manual (Enter/Space).

## Testing

Unplug the mouse and use the widget with Tab, Shift+Tab, arrows, Enter, Space and Escape. Then try a screen reader: NVDA (free, Windows), Narrator, VoiceOver (macOS/iOS), Orca (Linux). Automated checkers (axe DevTools, Lighthouse, Accessibility Insights) catch missing names and contrast, but not broken keyboard flows.

@@quiz
What's the first rule of ARIA?
* Use a native HTML element when one exists
- Add role attributes to every div
- Always use aria-label
- Avoid buttons
? Native elements include keyboard and semantics.

@@quiz
In an accessible tabs widget, how many tabs are in the Tab order?
- All of them
* Only the selected tab
- None
- Every other one
? Arrows move between tabs (roving tabindex).

@@quiz
What should happen when a modal dialog closes?
- Focus goes to the top of the page
* Focus returns to the element that opened it
- Focus stays nowhere
- The page reloads
? Users continue where they were.

@@quiz
Which attribute announces non-urgent status updates politely?
- role="alert"
* role="status"
- aria-hidden="true"
- tabindex="-1"
? alert interrupts; status waits.

@@quiz
What does `showModal()` on a `<dialog>` give you?
- Only a backdrop
* An inert background, focus handling and Escape to close
- A new window
- A tooltip
? Prefer it to a hand-built modal.

@@flashcards
APG :: ARIA Authoring Practices Guide
first rule of ARIA :: use native elements
accessible name :: label, aria-label, aria-labelledby
role :: what the element is
aria-expanded :: open or closed state
aria-selected :: selected tab or option
aria-controls :: points at controlled element
aria-modal="true" :: content behind is inert
inert :: not focusable or clickable
showModal() :: native modal dialog
focus trap :: Tab stays inside the modal
return focus :: back to the opener
roving tabindex :: one 0, the rest -1
tab keys :: arrows, Home, End
role="status" :: polite live region
role="alert" :: assertive live region

@@drill
Which attribute value marks the content outside a custom dialog as inert for assistive technology? (attribute="value")
= aria-modal="true"
= aria-modal=true
= aria-modal
? Use the inert attribute on the background as well.

@@drill
Which keys move between tabs in a horizontal tablist?
= arrow keys
= left and right arrows
= left/right arrows
= arrows
? Home and End jump to the ends.

@@exercise An accessible modal dialog
Open a native `<dialog>` with `showModal()` from a button, label it with `aria-labelledby`, close it with a Cancel button and on Escape (native), and return focus to the opening button when it closes (listen for the `close` event). Confirm sets a polite `role="status"` message on the page.
@@starter
<!doctype html>
<html lang="en">
<head>
  <meta charset="utf-8">
  <title>Dialog</title>
  <style>
    body { font-family: system-ui, sans-serif; padding: 32px; }
    button { font: inherit; padding: 0.5rem 1rem; border-radius: 8px; }
  </style>
</head>
<body>
  <button id="delete">Delete project…</button>
  <!-- dialog and status message here -->
</body>
</html>
@@requires
<dialog
aria-labelledby=
showModal()
addEventListener("close"
.focus()
role="status"
@@hint
`<dialog id="confirm" aria-labelledby="confirm-title"><h2 id="confirm-title">Delete project?</h2>...<form method="dialog"><button value="cancel">Cancel</button><button value="delete">Delete</button></form></dialog>`. A `method="dialog"` form closes the dialog and sets `dialog.returnValue` to the clicked button's value.
@@solution
<!doctype html>
<html lang="en">
<head>
  <meta charset="utf-8">
  <title>Dialog</title>
  <style>
    body { font-family: system-ui, sans-serif; padding: 32px; }
    button { font: inherit; padding: 0.5rem 1rem; border-radius: 8px; border: 1px solid #cbd5e1; background: #fff; }
    button:focus-visible { outline: 3px solid #2563eb; outline-offset: 2px; }
    dialog { border: none; border-radius: 14px; padding: 1.5rem; max-width: 26rem; box-shadow: 0 20px 50px rgb(0 0 0 / 0.25); }
    dialog::backdrop { background: rgb(15 23 42 / 0.5); }
    .actions { display: flex; gap: 0.5rem; justify-content: flex-end; margin-top: 1rem; }
    .danger { background: #dc2626; color: #fff; border-color: #dc2626; }
  </style>
</head>
<body>
  <button id="delete">Delete project…</button>
  <p id="message" role="status"></p>

  <dialog id="confirm" aria-labelledby="confirm-title" aria-describedby="confirm-text">
    <h2 id="confirm-title">Delete project?</h2>
    <p id="confirm-text">This removes all files in “Plant Buddy”. You can't undo this.</p>
    <form method="dialog" class="actions">
      <button value="cancel" autofocus>Cancel</button>
      <button value="delete" class="danger">Delete</button>
    </form>
  </dialog>

  <script>
    const opener = document.getElementById("delete");
    const dialog = document.getElementById("confirm");
    const message = document.getElementById("message");
    opener.addEventListener("click", () => dialog.showModal());
    dialog.addEventListener("close", () => {
      if (dialog.returnValue === "delete") {
        message.textContent = "Project deleted.";
      }
      opener.focus();
    });
  </script>
</body>
</html>

@@exercise Keyboard-accessible tabs
Build a tabs widget following the APG pattern: a `role="tablist"` with an `aria-label`, three `role="tab"` buttons with `aria-selected` and `aria-controls`, and `role="tabpanel"` panels labelled by their tabs. Use a roving tabindex, and handle ArrowRight, ArrowLeft (wrapping), Home and End in a `keydown` listener, activating the newly focused tab.
@@starter
<!doctype html>
<html lang="en">
<head>
  <meta charset="utf-8">
  <title>Tabs</title>
  <style>
    body { font-family: system-ui, sans-serif; padding: 32px; }
  </style>
</head>
<body>
  <div class="tabs">
    <!-- tablist and panels here: Overview, Settings, History -->
  </div>
</body>
</html>
@@requires
role="tablist"
role="tab"
aria-selected=
aria-controls=
role="tabpanel"
tabindex
ArrowRight
ArrowLeft
Home
End
@@hint
Write `select(index)` that sets `aria-selected` and `tabindex` on every tab (`"0"` for the selected, `"-1"` for the others), shows the matching panel (`hidden = false`), hides the rest, and calls `.focus()` on the tab. In `keydown`, compute the new index with `(i + 1) % n`, `(i - 1 + n) % n`, `0` or `n - 1`, and call `preventDefault()`.
@@solution
<!doctype html>
<html lang="en">
<head>
  <meta charset="utf-8">
  <title>Tabs</title>
  <style>
    body { font-family: system-ui, sans-serif; padding: 32px; }
    [role="tablist"] { display: flex; gap: 4px; border-bottom: 2px solid #e2e8f0; }
    [role="tab"] { font: inherit; padding: 0.6rem 1rem; border: none; background: none; border-bottom: 3px solid transparent; margin-bottom: -2px; cursor: pointer; color: #475569; }
    [role="tab"][aria-selected="true"] { color: #1d4ed8; border-bottom-color: #1d4ed8; font-weight: 600; }
    [role="tab"]:focus-visible { outline: 3px solid #2563eb; outline-offset: 2px; }
    [role="tabpanel"] { padding: 1rem 0; }
  </style>
</head>
<body>
  <div class="tabs">
    <div role="tablist" aria-label="Project">
      <button role="tab" id="tab-overview" aria-selected="true" aria-controls="panel-overview" tabindex="0">Overview</button>
      <button role="tab" id="tab-settings" aria-selected="false" aria-controls="panel-settings" tabindex="-1">Settings</button>
      <button role="tab" id="tab-history" aria-selected="false" aria-controls="panel-history" tabindex="-1">History</button>
    </div>
    <div role="tabpanel" id="panel-overview" aria-labelledby="tab-overview" tabindex="0"><p>Three plants, all watered today.</p></div>
    <div role="tabpanel" id="panel-settings" aria-labelledby="tab-settings" tabindex="0" hidden><p>Reminders at 08:00.</p></div>
    <div role="tabpanel" id="panel-history" aria-labelledby="tab-history" tabindex="0" hidden><p>Monstera repotted on 2 November.</p></div>
  </div>

  <script>
    const tabs = [...document.querySelectorAll('[role="tab"]')];

    function select(index) {
      tabs.forEach((tab, i) => {
        const selected = i === index;
        tab.setAttribute("aria-selected", String(selected));
        tab.setAttribute("tabindex", selected ? "0" : "-1");
        document.getElementById(tab.getAttribute("aria-controls")).hidden = !selected;
      });
      tabs[index].focus();
    }

    tabs.forEach((tab, i) => {
      tab.addEventListener("click", () => select(i));
      tab.addEventListener("keydown", (event) => {
        const n = tabs.length;
        const next = { ArrowRight: (i + 1) % n, ArrowLeft: (i - 1 + n) % n, Home: 0, End: n - 1 }[event.key];
        if (next !== undefined) {
          event.preventDefault();
          select(next);
        }
      });
    });
  </script>
</body>
</html>
