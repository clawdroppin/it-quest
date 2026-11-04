@@lesson n7-a11y Accessibility in Depth: ARIA Patterns, Focus and Live Regions
@@text
Module 3 covered the basics: semantic HTML, alt text, labels, contrast. Real apps also have custom widgets (tabs, dialogs, menus, comboboxes) that HTML doesn't fully provide, and content that changes without a page load. This lesson covers how to make those work for keyboard and screen-reader users, following the **WAI-ARIA Authoring Practices**.

Accessibility is also a legal requirement in many places: the EU's European Accessibility Act (in force from 2025), the US ADA and Section 508, and Japan's 障害者差別解消法 all point to **WCAG 2.2 level AA** as the standard.

## The first rule of ARIA

> Don't use ARIA if a native element does the job.

`<button>` is focusable, activates with Enter and Space, and is announced as a button. `<div role="button">` is none of those until you add `tabindex="0"`, key handlers and styles. Native elements first: `<button>`, `<a href>`, `<dialog>`, `<details>`, `<select>`, `<input type="checkbox">`.

ARIA only changes what assistive technology **announces**; it adds no behaviour. Three kinds of attribute:

- **Roles**: what it is (`role="tab"`, `role="tablist"`, `role="alert"`).
- **States**: current condition (`aria-selected`, `aria-expanded`, `aria-checked`, `aria-disabled`, `aria-invalid`).
- **Properties**: relationships and names (`aria-controls`, `aria-labelledby`, `aria-describedby`, `aria-label`).

Keep states in sync with reality in JavaScript: an `aria-expanded="false"` on an open menu is worse than nothing.

## The disclosure (show/hide)

The simplest pattern: a button that toggles a region.

```html
<button aria-expanded="false" aria-controls="filters" id="filters-btn">Filters</button>
<div id="filters" hidden>…</div>
<script>
  const b = document.getElementById("filters-btn");
  b.addEventListener("click", () => {
    const open = b.getAttribute("aria-expanded") === "true";
    b.setAttribute("aria-expanded", String(!open));
    document.getElementById("filters").hidden = open;
  });
</script>
```

(`<details>`/`<summary>` does this natively when you don't need custom behaviour.)

## Tabs

Structure: a `tablist` containing `tab`s, each controlling a `tabpanel`.

```html
<div role="tablist" aria-label="Server details">
  <button role="tab" id="tab-1" aria-selected="true"  aria-controls="panel-1">Overview</button>
  <button role="tab" id="tab-2" aria-selected="false" aria-controls="panel-2" tabindex="-1">Logs</button>
</div>
<div role="tabpanel" id="panel-1" aria-labelledby="tab-1">…</div>
<div role="tabpanel" id="panel-2" aria-labelledby="tab-2" hidden>…</div>
```

Keyboard: **Tab** moves into the tab list (to the selected tab) and then **out** to the panel; **Left/Right arrows** move between tabs; **Home/End** go to the first/last. This is the **roving tabindex** technique: only the active tab has `tabindex="0"`, the others `-1`, so the whole tab list is one Tab stop.

## Dialogs

Use the native `<dialog>` with `showModal()`. It provides most of the hard parts: it traps focus inside, makes the rest of the page inert, closes on Escape and renders in the top layer.

```html
<dialog id="confirm" aria-labelledby="confirm-title">
  <h2 id="confirm-title">Delete ticket #9?</h2>
  <p>This can't be undone.</p>
  <form method="dialog"><button value="cancel" autofocus>Cancel</button> <button value="delete">Delete</button></form>
</dialog>
```

Your remaining jobs: give it a name (`aria-labelledby`), put initial focus somewhere sensible (`autofocus`, ideally the least destructive option), and **return focus** to the button that opened it when it closes (browsers now do this for `showModal()`, but check yours).

## Focus management

- **Visible focus**: never `outline: none` without a replacement; use `:focus-visible`.
- **Order**: DOM order should match visual order. Avoid positive `tabindex` values (`tabindex="3"`); they create a separate, confusing order.
- **`tabindex="-1"`** makes something focusable by script (`el.focus()`) but not by Tab, e.g. a heading you move focus to after a route change in a single-page app.
- **Skip link**: `<a class="skip" href="#main">Skip to content</a>` as the first element, visible on focus.
- **`inert`** attribute removes a whole region from focus and the accessibility tree (e.g. the page behind a custom drawer).

## Live regions

When content changes without focus moving there (a toast, a "saved" message, search results count), screen readers won't notice unless it's in a **live region**:

```html
<div role="status" aria-live="polite">3 results</div>        <!-- announced when the user is idle -->
<div role="alert">Session expires in 1 minute</div>           <!-- announced immediately, interrupting -->
```

The region must **exist in the page before** you change its text; inserting a new element that already contains text often isn't announced. Use `alert` sparingly: it interrupts whatever the user is listening to.

## Testing with real tools

- Keyboard only: unplug the mouse for 5 minutes.
- Screen readers: **NVDA** (free, Windows), **Narrator** (built in to Windows), **VoiceOver** (macOS/iOS, Cmd+F5), **TalkBack** (Android).
- Browser dev tools' accessibility tree shows the computed name and role of every element.
- Automated checkers (axe, Lighthouse) find maybe 30–40% of issues; the rest needs a human.

@@quiz
What does adding `role="button"` to a `<div>` give you?
- Focus, keyboard activation and button styles
* Only the announcement "button"; focus and key handling must be added by you
- Nothing at all
- A native button
? ARIA changes semantics, not behaviour. Use `<button>`.

@@quiz
In an accessible tab list, how do keyboard users move between tabs?
- Tab to each tab in turn
* Arrow keys between tabs; Tab moves to the panel
- Only with the mouse
- With Enter only
? Roving tabindex: the tab list is one Tab stop.

@@quiz
A "Saved" message appears after an autosave. How do screen-reader users find out?
- They don't need to
* Put the text into an existing `role="status"` (polite live region)
- Use `role="alert"` every time
- Move focus to the message
? Polite announcements don't interrupt.

@@quiz
What does `<dialog>` opened with `showModal()` give you for free?
* Focus kept inside, the page behind made inert, Escape to close
- Automatic translation
- A heading
- Form validation
? You still provide the name and sensible initial focus.

@@flashcards
first rule of ARIA :: use native HTML when you can
`aria-expanded` :: is the controlled region open
`aria-controls` :: which element it controls
`aria-selected` :: current tab
roving tabindex :: one item `0`, others `-1`, arrows move
`role="status"` :: polite live region
`role="alert"` :: assertive live region
`tabindex="-1"` :: focusable by script only
`inert` :: remove from focus and AT
`showModal()` :: modal dialog with focus trap
WCAG 2.2 AA :: the usual legal target
NVDA / Narrator / VoiceOver :: screen readers to test with

@@exercise Accessible tabs with arrow keys
Turn the three buttons into an accessible tab widget: a `role="tablist"` with an `aria-label`, three `role="tab"` buttons with `aria-selected`, `aria-controls` and roving `tabindex` (0 on the selected one, -1 on the rest), and three `role="tabpanel"`s with `aria-labelledby` (only the first visible). Write the script: clicking a tab selects it; `ArrowRight`/`ArrowLeft` move to the next/previous tab (wrapping around) and select it, moving focus.
@@starter
<!doctype html>
<html lang="en">
<head><meta charset="utf-8"><title>Tabs</title></head>
<body>
  <div>
    <button>Overview</button>
    <button>Logs</button>
    <button>Settings</button>
  </div>
  <div><p>web01 · Ubuntu 24.04 · up 41 days</p></div>
  <div><p>09:12 nginx reloaded</p></div>
  <div><p>Auto-patching: on</p></div>
</body>
</html>
@@requires
role="tablist"
role="tab"
aria-selected="true"
aria-controls="panel-1"
tabindex="-1"
role="tabpanel"
aria-labelledby="tab-1"
ArrowRight
ArrowLeft
.focus()
@@hint
Write a `select(i)` function that loops over the tabs: `tab.setAttribute("aria-selected", String(j === i)); tab.tabIndex = j === i ? 0 : -1; panels[j].hidden = j !== i;` then `tabs[i].focus()`. In a keydown listener on the tab list, compute the current index and call `select((i + 1) % n)` or `select((i - 1 + n) % n)`.
@@solution
<!doctype html>
<html lang="en">
<head><meta charset="utf-8"><title>Tabs</title>
<style>
  [role="tab"][aria-selected="true"] { border-block-end: 3px solid #2563eb; font-weight: 600; }
  [role="tab"]:focus-visible { outline: 3px solid #f59e0b; }
</style>
</head>
<body>
  <div role="tablist" aria-label="Server web01">
    <button role="tab" id="tab-1" aria-selected="true" aria-controls="panel-1">Overview</button>
    <button role="tab" id="tab-2" aria-selected="false" aria-controls="panel-2" tabindex="-1">Logs</button>
    <button role="tab" id="tab-3" aria-selected="false" aria-controls="panel-3" tabindex="-1">Settings</button>
  </div>
  <div role="tabpanel" id="panel-1" aria-labelledby="tab-1"><p>web01 · Ubuntu 24.04 · up 41 days</p></div>
  <div role="tabpanel" id="panel-2" aria-labelledby="tab-2" hidden><p>09:12 nginx reloaded</p></div>
  <div role="tabpanel" id="panel-3" aria-labelledby="tab-3" hidden><p>Auto-patching: on</p></div>
  <script>
    const tabs = [...document.querySelectorAll('[role="tab"]')];
    const panels = tabs.map(t => document.getElementById(t.getAttribute("aria-controls")));
    function select(i) {
      tabs.forEach((tab, j) => {
        tab.setAttribute("aria-selected", String(j === i));
        tab.tabIndex = j === i ? 0 : -1;
        panels[j].hidden = j !== i;
      });
      tabs[i].focus();
    }
    tabs.forEach((tab, i) => tab.addEventListener("click", () => select(i)));
    document.querySelector('[role="tablist"]').addEventListener("keydown", e => {
      const i = tabs.indexOf(document.activeElement);
      if (i < 0) return;
      const n = tabs.length;
      if (e.key === "ArrowRight") { select((i + 1) % n); e.preventDefault(); }
      if (e.key === "ArrowLeft") { select((i - 1 + n) % n); e.preventDefault(); }
      if (e.key === "Home") { select(0); e.preventDefault(); }
      if (e.key === "End") { select(n - 1); e.preventDefault(); }
    });
  </script>
</body>
</html>

@@exercise A confirmation dialog that returns focus
Replace the `confirm()` call with an accessible `<dialog id="confirm">`: named by its heading via `aria-labelledby`, opened with `showModal()`, closed with a `<form method="dialog">` whose Cancel button has `autofocus`. When it closes, read `dialog.returnValue`; if it's `"delete"`, remove the ticket row and announce "Ticket 9 deleted" in a `role="status"` region. Return focus to the delete button's row heading or, if the row is gone, to the table caption (give it `tabindex="-1"`).
@@starter
<!doctype html>
<html lang="en">
<head><meta charset="utf-8"><title>Delete</title></head>
<body>
  <table>
    <caption>Tickets</caption>
    <tr id="row-9"><td>#9 File server almost full</td><td><button id="del">Delete</button></td></tr>
  </table>
  <script>
    document.getElementById("del").addEventListener("click", () => {
      if (confirm("Delete ticket 9?")) document.getElementById("row-9").remove();
    });
  </script>
</body>
</html>
@@requires
<dialog id="confirm" aria-labelledby="confirm-title">
showModal()
<form method="dialog">
autofocus
returnValue
role="status"
tabindex="-1"
@@hint
`const dlg = document.getElementById("confirm"); del.onclick = () => dlg.showModal(); dlg.addEventListener("close", () => { if (dlg.returnValue === "delete") { row.remove(); status.textContent = "Ticket 9 deleted"; caption.focus(); } else del.focus(); });`
@@solution
<!doctype html>
<html lang="en">
<head><meta charset="utf-8"><title>Delete</title></head>
<body>
  <table>
    <caption id="cap" tabindex="-1">Tickets</caption>
    <tr id="row-9"><td>#9 File server almost full</td><td><button id="del">Delete</button></td></tr>
  </table>
  <div role="status" id="status"></div>
  <dialog id="confirm" aria-labelledby="confirm-title">
    <h2 id="confirm-title">Delete ticket 9?</h2>
    <p>This can't be undone.</p>
    <form method="dialog">
      <button value="cancel" autofocus>Cancel</button>
      <button value="delete">Delete</button>
    </form>
  </dialog>
  <script>
    const dlg = document.getElementById("confirm");
    const del = document.getElementById("del");
    del.addEventListener("click", () => { dlg.returnValue = ""; dlg.showModal(); });
    dlg.addEventListener("close", () => {
      if (dlg.returnValue === "delete") {
        document.getElementById("row-9").remove();
        document.getElementById("status").textContent = "Ticket 9 deleted";
        document.getElementById("cap").focus();
      } else {
        del.focus();
      }
    });
  </script>
</body>
</html>

@@exercise A disclosure and a live search count
Make the "Filters" button a disclosure: `aria-expanded` and `aria-controls="filters"`, the panel starts `hidden`, and the script toggles both. Then make the search box report the number of matching tickets in a polite live region (`role="status"`) that exists in the page from the start, updating it on every `input` event ("2 tickets match").
@@starter
<!doctype html>
<html lang="en">
<head><meta charset="utf-8"><title>Search</title></head>
<body>
  <button id="toggle">Filters</button>
  <div id="filters"><label><input type="checkbox"> Only P1</label></div>
  <label for="q">Search tickets</label> <input id="q" type="search">
  <ul id="list">
    <li>VPN drops every hour</li><li>VPN client won't install</li><li>Printer jams</li><li>Email quota full</li>
  </ul>
</body>
</html>
@@requires
aria-expanded="false"
aria-controls="filters"
hidden
role="status"
addEventListener("input"
setAttribute("aria-expanded"
@@hint
Toggle: `const open = btn.getAttribute("aria-expanded") === "true"; btn.setAttribute("aria-expanded", String(!open)); panel.hidden = open;`. Search: loop over the `li`s, set `li.hidden = !li.textContent.toLowerCase().includes(q)`, count the visible ones and write `${n} tickets match` into the status element.
@@solution
<!doctype html>
<html lang="en">
<head><meta charset="utf-8"><title>Search</title></head>
<body>
  <button id="toggle" aria-expanded="false" aria-controls="filters">Filters</button>
  <div id="filters" hidden><label><input type="checkbox"> Only P1</label></div>
  <label for="q">Search tickets</label> <input id="q" type="search">
  <p role="status" id="count">4 tickets</p>
  <ul id="list">
    <li>VPN drops every hour</li><li>VPN client won't install</li><li>Printer jams</li><li>Email quota full</li>
  </ul>
  <script>
    const btn = document.getElementById("toggle");
    const panel = document.getElementById("filters");
    btn.addEventListener("click", () => {
      const open = btn.getAttribute("aria-expanded") === "true";
      btn.setAttribute("aria-expanded", String(!open));
      panel.hidden = open;
    });
    const items = [...document.querySelectorAll("#list li")];
    document.getElementById("q").addEventListener("input", e => {
      const q = e.target.value.trim().toLowerCase();
      let n = 0;
      for (const li of items) {
        const match = li.textContent.toLowerCase().includes(q);
        li.hidden = !match;
        if (match) n++;
      }
      document.getElementById("count").textContent = `${n} ticket${n === 1 ? "" : "s"} match`;
    });
  </script>
</body>
</html>
