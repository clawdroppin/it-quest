@@lesson gu9-plant-buddy Plant Buddy: a Polished App with Its Own Icons and Animations
@@text
Time to put the whole track together in one small app that's a joy to use: **Plant Buddy** reminds you when to water your plants, and every part of it is designed: tokens, a hand-drawn icon set, an app icon, light and dark themes, button states, a dialog, toasts, and lots of little animations.

```text
 🪴 Plant Buddy                              ☾   (+ New plant)
    1 plant needs water
 ┌──────────────────────────┐ ┌──────────────────────────┐
 │ (🌿) Monty the Monstera  │ │ (🌻) Sunny               │
 │      1 day overdue       │ │      Tomorrow            │
 │      ▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁   │ │      ███▁▁▁▁▁▁▁▁▁▁▁▁▁   │
 │ (💧 Water)          🗑   │ │ (💧 Water)          🗑   │
 └──────────────────────────┘ └──────────────────────────┘
```

## What's in it

| Feature | Lesson it comes from |
|---|---|
| All logic in `plants.js`, pure functions with tests; `app.js` only draws and reacts | GUI Foundations |
| Design tokens for colour, spacing, radius, easing and duration | Making It Pretty |
| Light and dark themes (follows the OS, `T` toggles, remembered) | Making It Pretty |
| Buttons with hover, pressed, focus-visible and disabled states; icon buttons with names | Interactive Controls |
| A `<dialog>` form with a range slider, emoji radio buttons and a shaking error | Interactive Controls |
| Toasts in a `role="status"` region | Interactive Controls |
| Staggered card entrance, a swaying happy plant, a drooping thirsty one, a hop and falling droplets on watering, leaves bursting when everyone is happy | Cute Animations |
| `prefers-reduced-motion` turns the motion off | Cute Animations |
| An SVG icon sprite on one 24-unit grid, and an app icon that reads at 16 px | Icon Creation |

## How it's built

| File | Job |
|---|---|
| `index.html` | the structure, the **icon sprite** (`<symbol>`s), a `<template>` for one card, the dialog |
| `style.css` | tokens, both themes, components, every keyframe, reduced motion |
| `plants.js` | the rules: days until water, status, labels, watering, sorting, adding, loading saved data |
| `app.js` | state → `render()`, event delegation, the dialog, toasts, droplets, the celebration, the theme, keyboard shortcuts |
| `plants.test.js` | 8 tests for `plants.js`: `node --test` |
| `app-icon.svg` | the app icon (also the favicon) |

## Your tasks

Four functions in `plants.js` throw `TODO`. The comment above each is its specification, and the tests check them:

1. `status`: turn "days until water" into `"thirsty"`, `"today"`, `"soon"` or `"happy"`. The page uses it as `data-status`, which picks the colours and animations in the CSS.
2. `label`: the friendly text: `"3 days overdue"`, `"1 day overdue"`, `"Water today"`, `"Tomorrow"`, `"In 4 days"`.
3. `water`: return a **new** list (`map` and spread), with only the watered plant replaced.
4. `sortByNeed`: copy the list (`[...plants]`) before sorting, because `sort` changes the array it's called on.

```text
node --test
```

The page calls these functions as soon as it loads, so it shows nothing until they work. Then open `index.html` in a browser (double-click it, or `python -m http.server` in the folder), and try: watering every thirsty plant, the `N` and `T` keys, Tab through everything, an empty name in the dialog, your OS in dark mode, and "reduce motion" turned on.
@@project plant_buddy Plant Buddy
Write the four functions that throw `TODO` in `plants.js` until `node --test` passes, then open `index.html`.
@@file index.html
<!doctype html>
<html lang="en">
<head>
  <meta charset="utf-8">
  <meta name="viewport" content="width=device-width, initial-scale=1">
  <title>Plant Buddy</title>
  <meta http-equiv="Content-Security-Policy" content="default-src 'self'; style-src 'self'; img-src 'self' data:">
  <link rel="icon" href="app-icon.svg" type="image/svg+xml">
  <link rel="stylesheet" href="style.css">
</head>
<body>
  <!-- The icon set: one 24 x 24 grid, 2-unit stroke, round caps and joins, drawn inside 2..22.
       Each <symbol> is used with <svg class="icon"><use href="#i-name"/></svg> and takes the text colour. -->
  <svg class="sprite" aria-hidden="true">
    <symbol id="i-drop" viewBox="0 0 24 24"><path d="M12 3c3.5 4.4 6 7.8 6 11a6 6 0 0 1-12 0c0-3.2 2.5-6.6 6-11z"/><path d="M9.5 14.5a2.5 2.5 0 0 0 2.5 2.5"/></symbol>
    <symbol id="i-leaf" viewBox="0 0 24 24"><path d="M5 19c0-8 5-13 14-14 0 9-5 14-13 14z"/><path d="M5 19l7-7"/></symbol>
    <symbol id="i-sun" viewBox="0 0 24 24"><circle cx="12" cy="12" r="4"/><path d="M12 2v2M12 20v2M2 12h2M20 12h2M4.9 4.9l1.4 1.4M17.7 17.7l1.4 1.4M4.9 19.1l1.4-1.4M17.7 6.3l1.4-1.4"/></symbol>
    <symbol id="i-moon" viewBox="0 0 24 24"><path d="M20 14.5A8 8 0 1 1 9.5 4a6.5 6.5 0 0 0 10.5 10.5z"/></symbol>
    <symbol id="i-plus" viewBox="0 0 24 24"><path d="M12 5v14M5 12h14"/></symbol>
    <symbol id="i-trash" viewBox="0 0 24 24"><path d="M4 7h16M10 11v6M14 11v6M6 7l1 13h10l1-13M9 7V4h6v3"/></symbol>
    <symbol id="i-pot" viewBox="0 0 24 24"><path d="M5 13h14l-2 8H7z"/><path d="M12 13V8"/><path d="M12 9c-3 0-5-2-5-5 3 0 5 2 5 5zM12 8c0-3 2-5 5-5 0 3-2 5-5 5z"/></symbol>
    <symbol id="i-x" viewBox="0 0 24 24"><path d="M6 6l12 12M18 6L6 18"/></symbol>
  </svg>

  <header class="top">
    <div class="brand">
      <img src="app-icon.svg" alt="" width="40" height="40" class="logo">
      <div>
        <h1>Plant Buddy</h1>
        <p id="summary" class="muted"></p>
      </div>
    </div>
    <div class="actions">
      <button id="theme" class="icon-btn" aria-label="Switch to dark theme" title="Theme (T)">
        <svg class="icon"><use href="#i-moon"/></svg>
      </button>
      <button id="add" class="btn" title="New plant (N)">
        <svg class="icon" aria-hidden="true"><use href="#i-plus"/></svg> New plant
      </button>
    </div>
  </header>

  <main>
    <ul id="plants" class="grid" aria-label="Your plants"></ul>
    <div id="empty" class="empty" hidden>
      <svg class="icon big"><use href="#i-pot"/></svg>
      <p>No plants yet. Press <kbd>N</kbd> to add your first one.</p>
    </div>
  </main>

  <template id="card">
    <li class="card">
      <div class="face" aria-hidden="true"><span class="emoji"></span></div>
      <div class="info">
        <h2 class="name"></h2>
        <p class="when"></p>
        <div class="meter" aria-hidden="true"><span></span></div>
      </div>
      <div class="card-actions">
        <button class="btn water"><svg class="icon" aria-hidden="true"><use href="#i-drop"/></svg> Water</button>
        <button class="icon-btn remove"><svg class="icon"><use href="#i-trash"/></svg></button>
      </div>
    </li>
  </template>

  <dialog id="dialog" aria-labelledby="dialog-title">
    <form id="form" method="dialog">
      <header class="dialog-head">
        <h2 id="dialog-title">A new plant</h2>
        <button type="button" class="icon-btn" id="cancel" aria-label="Close"><svg class="icon"><use href="#i-x"/></svg></button>
      </header>
      <label>Name <input name="name" maxlength="30" required autocomplete="off" placeholder="e.g. Fernando the Fern"></label>
      <fieldset class="emojis">
        <legend>Pick a face</legend>
        <label><input type="radio" name="emoji" value="🪴" checked><span>🪴</span></label>
        <label><input type="radio" name="emoji" value="🌿"><span>🌿</span></label>
        <label><input type="radio" name="emoji" value="🌵"><span>🌵</span></label>
        <label><input type="radio" name="emoji" value="🌱"><span>🌱</span></label>
        <label><input type="radio" name="emoji" value="🌻"><span>🌻</span></label>
        <label><input type="radio" name="emoji" value="🌷"><span>🌷</span></label>
      </fieldset>
      <label>Water every <output id="every-out">3 days</output>
        <input name="everyDays" type="range" min="1" max="30" value="3">
      </label>
      <p id="error" class="error" role="alert"></p>
      <div class="dialog-actions">
        <button type="submit" class="btn">Add plant</button>
      </div>
    </form>
  </dialog>

  <div id="toasts" class="toasts" role="status"></div>

  <script src="plants.js"></script>
  <script src="app.js"></script>
</body>
</html>
@@file style.css
/* ---------- Design tokens: every colour, gap, radius and timing lives here ---------- */
:root {
  color-scheme: light;
  --bg: #f4fbf5;
  --surface: #ffffff;
  --surface-2: #ecf7ee;
  --text: #17301f;
  --muted: #5d7464;
  --border: #d8eadb;
  --accent: #16a34a;
  --accent-hover: #15803d;
  --accent-text: #ffffff;
  --ring: #86efac;
  --water: #0ea5e9;
  --thirsty: #e11d48;
  --today: #d97706;
  --soon: #ca8a04;
  --happy: #16a34a;

  --space-1: 4px; --space-2: 8px; --space-3: 16px; --space-4: 24px; --space-5: 40px;
  --radius: 18px;
  --radius-sm: 10px;
  --shadow: 0 1px 2px rgb(23 48 31 / .06), 0 10px 30px rgb(23 48 31 / .08);
  --shadow-lift: 0 2px 4px rgb(23 48 31 / .08), 0 18px 40px rgb(23 48 31 / .14);

  --ease-out: cubic-bezier(.2, .8, .3, 1);
  --ease-bounce: cubic-bezier(.34, 1.56, .64, 1);
  --dur-fast: 150ms;
  --dur: 300ms;
}

:root[data-theme="dark"] {
  color-scheme: dark;
  --bg: #0f1a13;
  --surface: #172419;
  --surface-2: #1d2e21;
  --text: #e3f2e6;
  --muted: #9bb3a1;
  --border: #29402e;
  --accent: #4ade80;
  --accent-hover: #86efac;
  --accent-text: #0f1a13;
  --ring: #166534;
  --water: #38bdf8;
  --thirsty: #fb7185;
  --today: #fbbf24;
  --soon: #facc15;
  --happy: #4ade80;
  --shadow: 0 1px 2px rgb(0 0 0 / .3), 0 10px 30px rgb(0 0 0 / .3);
  --shadow-lift: 0 2px 4px rgb(0 0 0 / .35), 0 18px 40px rgb(0 0 0 / .45);
}

/* ---------- Base ---------- */
* { box-sizing: border-box; }
body {
  margin: 0;
  background: var(--bg);
  color: var(--text);
  font: 16px/1.5 "Nunito", system-ui, -apple-system, "Segoe UI", sans-serif;
  transition: background-color var(--dur), color var(--dur);
}
h1, h2 { margin: 0; line-height: 1.2; }
.muted { color: var(--muted); margin: 0; }
.sprite { position: absolute; width: 0; height: 0; }
.icon {
  width: 20px; height: 20px; flex: none;
  fill: none; stroke: currentColor; stroke-width: 2; stroke-linecap: round; stroke-linejoin: round;
}
.icon.big { width: 56px; height: 56px; stroke-width: 1.5; }
kbd { font: 600 .85em ui-monospace, monospace; background: var(--surface-2); border: 1px solid var(--border); border-radius: 6px; padding: 1px 6px; }

/* ---------- Buttons: default, hover, active, focus-visible, disabled ---------- */
.btn, .icon-btn {
  display: inline-flex; align-items: center; justify-content: center; gap: var(--space-2);
  border: 0; font: inherit; font-weight: 700; cursor: pointer;
  transition: background-color var(--dur-fast), transform var(--dur-fast) var(--ease-bounce), box-shadow var(--dur-fast), color var(--dur-fast);
}
.btn {
  background: var(--accent); color: var(--accent-text);
  border-radius: 999px; padding: var(--space-2) var(--space-3);
  box-shadow: 0 2px 0 rgb(0 0 0 / .12);
}
.btn:hover { background: var(--accent-hover); transform: translateY(-1px); }
.btn:active { transform: translateY(1px) scale(.97); box-shadow: none; }
.icon-btn {
  width: 40px; height: 40px; border-radius: 50%;
  background: transparent; color: var(--muted);
}
.icon-btn:hover { background: var(--surface-2); color: var(--text); }
.icon-btn:active { transform: scale(.9); }
.btn:focus-visible, .icon-btn:focus-visible, input:focus-visible + span, input:focus-visible {
  outline: 3px solid var(--ring); outline-offset: 2px;
}
.btn:disabled { opacity: .5; cursor: not-allowed; transform: none; }

/* ---------- Layout ---------- */
.top {
  display: flex; align-items: center; justify-content: space-between; gap: var(--space-3); flex-wrap: wrap;
  max-width: 960px; margin: 0 auto; padding: var(--space-4) var(--space-3);
}
.brand { display: flex; align-items: center; gap: var(--space-3); }
.logo { border-radius: 12px; animation: sway 4s ease-in-out infinite; transform-origin: 50% 100%; }
.actions { display: flex; align-items: center; gap: var(--space-2); }
main { max-width: 960px; margin: 0 auto; padding: 0 var(--space-3) var(--space-5); }
.grid {
  list-style: none; margin: 0; padding: 0;
  display: grid; gap: var(--space-3);
  grid-template-columns: repeat(auto-fill, minmax(260px, 1fr));
}

/* ---------- Plant cards ---------- */
.card {
  --status: var(--happy);
  background: var(--surface); border: 1px solid var(--border); border-radius: var(--radius);
  box-shadow: var(--shadow); padding: var(--space-3);
  display: grid; grid-template-columns: auto 1fr; gap: var(--space-3); align-items: center;
  transition: transform var(--dur) var(--ease-bounce), box-shadow var(--dur);
}
.card.enter {                                       /* the first render: a staggered entrance */
  animation: card-in var(--dur) var(--ease-out) both;
  animation-delay: calc(var(--i, 0) * 60ms);        /* app.js sets --i */
}
.card:hover { transform: translateY(-3px); box-shadow: var(--shadow-lift); }
.card[data-status="thirsty"] { --status: var(--thirsty); }
.card[data-status="today"] { --status: var(--today); }
.card[data-status="soon"] { --status: var(--soon); }

.face {
  width: 64px; height: 64px; border-radius: 50%;
  display: grid; place-items: center; font-size: 34px;
  background: color-mix(in srgb, var(--status) 16%, var(--surface));
  box-shadow: inset 0 0 0 3px color-mix(in srgb, var(--status) 45%, transparent);
}
.emoji { display: inline-block; transform-origin: 50% 90%; }
.card[data-status="happy"] .emoji { animation: sway 3.5s ease-in-out infinite; }
.card[data-status="thirsty"] .emoji { animation: droop 2.4s ease-in-out infinite; filter: saturate(.6); }
.name { font-size: 1.1rem; font-weight: 800; overflow-wrap: anywhere; }
.when { margin: 2px 0 var(--space-2); color: var(--status); font-weight: 700; font-size: .95rem; }
.meter { height: 6px; border-radius: 999px; background: var(--surface-2); overflow: hidden; }
.meter span {
  display: block; height: 100%; width: calc(var(--fill, 1) * 100%);
  background: var(--status); border-radius: inherit;
  transition: width 600ms var(--ease-bounce);
}
.card-actions { grid-column: 1 / -1; display: flex; justify-content: space-between; align-items: center; }
.water { background: var(--water); color: white; }
.water:hover { background: color-mix(in srgb, var(--water) 85%, black); }

/* Watering: the card does a little hop, and droplets fall from the button (made in app.js) */
.card.watered { animation: hop 500ms var(--ease-bounce); }
.drop {
  position: fixed; width: 10px; height: 14px; pointer-events: none;
  background: var(--water); border-radius: 50% 50% 50% 50% / 60% 60% 40% 40%;
  animation: fall 700ms var(--ease-out) forwards;
}
.card.removing { animation: card-out 250ms ease-in forwards; }

/* ---------- Empty state ---------- */
.empty { text-align: center; color: var(--muted); padding: var(--space-5) var(--space-3); }
.empty .icon { color: var(--accent); animation: float 3s ease-in-out infinite; }

/* ---------- Dialog and form ---------- */
dialog {
  border: 0; border-radius: var(--radius); padding: var(--space-4);
  width: min(420px, calc(100vw - 32px));
  background: var(--surface); color: var(--text); box-shadow: var(--shadow-lift);
}
dialog[open] { animation: pop-in var(--dur) var(--ease-bounce); }
dialog::backdrop { background: rgb(15 26 19 / .45); backdrop-filter: blur(3px); }
form { display: grid; gap: var(--space-3); }
.dialog-head { display: flex; justify-content: space-between; align-items: center; }
label { display: grid; gap: var(--space-1); font-weight: 700; }
input[name="name"] {
  font: inherit; padding: 10px 12px; border-radius: var(--radius-sm);
  border: 2px solid var(--border); background: var(--bg); color: var(--text);
  transition: border-color var(--dur-fast);
}
input[name="name"]:focus { border-color: var(--accent); }
input[type="range"] { accent-color: var(--accent); }
output { color: var(--accent); font-variant-numeric: tabular-nums; }
.emojis { border: 0; padding: 0; margin: 0; display: flex; flex-wrap: wrap; gap: var(--space-2); }
.emojis legend { font-weight: 700; margin-bottom: var(--space-1); }
.emojis label { cursor: pointer; }
.emojis input { position: absolute; opacity: 0; width: 1px; height: 1px; }
.emojis span {
  display: grid; place-items: center; width: 44px; height: 44px; font-size: 24px;
  border-radius: 12px; background: var(--surface-2); border: 2px solid transparent;
  transition: transform var(--dur-fast) var(--ease-bounce), border-color var(--dur-fast);
}
.emojis span:hover { transform: scale(1.1); }
.emojis input:checked + span { border-color: var(--accent); transform: scale(1.1); }
.error { color: var(--thirsty); margin: 0; min-height: 1.5em; font-weight: 700; }
.error:not(:empty) { animation: shake 300ms; }
.dialog-actions { display: flex; justify-content: flex-end; }

/* ---------- Toasts ---------- */
.toasts { position: fixed; bottom: var(--space-3); left: 50%; transform: translateX(-50%); display: grid; gap: var(--space-2); z-index: 10; }
.toast {
  background: var(--text); color: var(--bg); padding: 10px 16px; border-radius: 999px;
  box-shadow: var(--shadow-lift); font-weight: 700; display: flex; align-items: center; gap: var(--space-2);
  animation: toast-in var(--dur) var(--ease-bounce);
}
.toast.leaving { opacity: 0; transform: translateY(8px); transition: opacity 200ms, transform 200ms; }

/* ---------- Celebration: leaves burst out when every plant is watered ---------- */
.leaf {
  position: fixed; font-size: 20px; pointer-events: none;
  animation: burst 1100ms var(--ease-out) forwards;
}

/* ---------- Keyframes ---------- */
@keyframes card-in { from { opacity: 0; transform: translateY(14px) scale(.97); } }
@keyframes card-out { to { opacity: 0; transform: scale(.85); } }
@keyframes pop-in { from { opacity: 0; transform: scale(.92); } }
@keyframes toast-in { from { opacity: 0; transform: translateY(14px) scale(.9); } }
@keyframes sway { 0%, 100% { transform: rotate(-4deg); } 50% { transform: rotate(4deg); } }
@keyframes droop { 0%, 100% { transform: rotate(0) translateY(0); } 50% { transform: rotate(-10deg) translateY(2px); } }
@keyframes float { 50% { transform: translateY(-8px); } }
@keyframes hop {
  0%, 100% { transform: translateY(0) scale(1, 1); }
  30% { transform: translateY(-10px) scale(.97, 1.04); }
  65% { transform: translateY(0) scale(1.03, .97); }
}
@keyframes fall { from { opacity: 1; } to { opacity: 0; transform: translate(var(--dx), 70px) scale(.5); } }
@keyframes burst { to { opacity: 0; transform: translate(var(--x), var(--y)) rotate(var(--r)); } }
@keyframes shake { 25% { transform: translateX(-4px); } 75% { transform: translateX(4px); } }

/* ---------- Less motion for people who asked their OS for it ---------- */
@media (prefers-reduced-motion: reduce) {
  *, *::before, *::after {
    animation-duration: 1ms !important;
    animation-iteration-count: 1 !important;
    transition-duration: 1ms !important;
  }
}
@@file plants.js
// plants.js: everything Plant Buddy knows about plants, with no DOM and no storage.
// The page uses it as window.Plants; Node's test runner loads it with require().
// Functions never change the list they're given; they return a new one.
(function (root) {
  "use strict";

  const MAX_NAME = 30;

  // "YYYY-MM-DD" -> a whole day number. Date.UTC ignores time zones and daylight saving,
  // so subtracting two day numbers always gives a whole number of calendar days.
  function dayNumber(key) {
    const [y, m, d] = key.split("-").map(Number);
    return Date.UTC(y, m - 1, d) / 86400000;
  }

  // The local calendar day of a Date, as "YYYY-MM-DD".
  function today(date = new Date()) {
    const m = String(date.getMonth() + 1).padStart(2, "0");
    const d = String(date.getDate()).padStart(2, "0");
    return `${date.getFullYear()}-${m}-${d}`;
  }

  // Days until the plant needs water: 0 = today, negative = overdue.
  function daysUntilWater(plant, todayKey) {
    return dayNumber(plant.lastWatered) + plant.everyDays - dayNumber(todayKey);
  }

  // The plant's status, which picks its colour, icon and animation:
  //   overdue (days < 0) -> "thirsty", 0 -> "today", 1 -> "soon", anything later -> "happy".
  function status(plant, todayKey) {
    throw new Error("TODO: not done yet");
  }

  // The text under the plant's name for a number of days (from daysUntilWater):
  //   -1 -> "1 day overdue", -3 -> "3 days overdue", 0 -> "Water today",
  //   1 -> "Tomorrow", 4 -> "In 4 days".
  function label(days) {
    throw new Error("TODO: not done yet");
  }

  // A new list where the plant with this id has lastWatered = todayKey and one more
  // in its `waterings` count. Every other plant is the same object as before.
  function water(plants, id, todayKey) {
    throw new Error("TODO: not done yet");
  }

  // A new, sorted list: the plant that needs water soonest first; ties by name, A to Z,
  // ignoring upper/lower case (use localeCompare with { sensitivity: "base" }).
  function sortByNeed(plants, todayKey) {
    throw new Error("TODO: not done yet");
  }

  // Adds a plant, or throws an Error whose message the form shows.
  function addPlant(plants, { name, emoji, everyDays }, todayKey) {
    name = String(name ?? "").trim();
    everyDays = Number(everyDays);
    if (!name) throw new Error("Give your plant a name");
    if (name.length > MAX_NAME) throw new Error(`Names can be ${MAX_NAME} characters at most`);
    if (!Number.isInteger(everyDays) || everyDays < 1 || everyDays > 60) {
      throw new Error("Water every 1 to 60 days");
    }
    const id = plants.reduce((max, p) => Math.max(max, p.id), 0) + 1;
    return [...plants, { id, name, emoji: emoji || "🪴", everyDays, lastWatered: todayKey, waterings: 0 }];
  }

  function removePlant(plants, id) {
    return plants.filter((p) => p.id !== id);
  }

  // A friendly summary for the header.
  function summary(plants, todayKey) {
    if (plants.length === 0) return "No plants yet. Add your first one!";
    const need = plants.filter((p) => daysUntilWater(p, todayKey) <= 0).length;
    if (need === 0) return "Everyone is happy 🌞";
    return `${need} plant${need === 1 ? " needs" : "s need"} water`;
  }

  function samplePlants(todayKey) {
    const back = (n) => {
      const [y, m, d] = todayKey.split("-").map(Number);
      return today(new Date(y, m - 1, d - n));
    };
    return [
      { id: 1, name: "Monty the Monstera", emoji: "🌿", everyDays: 7, lastWatered: back(8), waterings: 12 },
      { id: 2, name: "Spike", emoji: "🌵", everyDays: 21, lastWatered: back(3), waterings: 4 },
      { id: 3, name: "Basil", emoji: "🌱", everyDays: 2, lastWatered: back(2), waterings: 30 },
      { id: 4, name: "Sunny", emoji: "🌻", everyDays: 3, lastWatered: back(2), waterings: 9 },
    ];
  }

  // Reads saved JSON; anything broken or missing gives the sample plants instead.
  function load(json, todayKey) {
    try {
      const data = JSON.parse(json);
      if (Array.isArray(data) && data.every((p) => p && typeof p.name === "string" && typeof p.lastWatered === "string")) {
        return data.map((p) => ({ waterings: 0, emoji: "🪴", ...p }));
      }
    } catch { /* fall through */ }
    return samplePlants(todayKey);
  }

  const api = { MAX_NAME, dayNumber, today, daysUntilWater, status, label, water, sortByNeed,
    addPlant, removePlant, summary, samplePlants, load };
  if (typeof module === "object" && module.exports) module.exports = api;
  else root.Plants = api;
})(globalThis);
@@file app.js
// app.js: the window. It keeps the state, saves it, draws it, and plays the animations.
// All the rules live in plants.js (window.Plants), so this file stays about the GUI.
(function () {
  "use strict";
  const P = window.Plants;
  const KEY = "plant-buddy";
  const THEME_KEY = "plant-buddy-theme";
  const reduceMotion = matchMedia("(prefers-reduced-motion: reduce)");

  const $ = (sel) => document.querySelector(sel);
  const list = $("#plants");
  const template = $("#card");
  const dialog = $("#dialog");
  const form = $("#form");

  // ---------- State ----------
  let todayKey = P.today();
  let plants = P.load(readStorage(KEY), todayKey);
  let firstRender = true;

  function readStorage(key) {
    try { return localStorage.getItem(key); } catch { return null; }
  }
  function writeStorage(key, value) {
    try { localStorage.setItem(key, value); } catch { /* private mode: keep working without saving */ }
  }

  function setPlants(next) {
    plants = next;
    writeStorage(KEY, JSON.stringify(plants));
    render();
  }

  // ---------- Drawing ----------
  function render() {
    const sorted = P.sortByNeed(plants, todayKey);
    $("#summary").textContent = P.summary(plants, todayKey);
    $("#empty").hidden = plants.length > 0;

    list.replaceChildren(...sorted.map((plant, i) => {
      const card = template.content.firstElementChild.cloneNode(true);
      const days = P.daysUntilWater(plant, todayKey);
      card.dataset.id = plant.id;
      card.dataset.status = P.status(plant, todayKey);
      // Only the first render staggers in; later renders appear in place.
      if (firstRender) {
        card.classList.add("enter");
        card.style.setProperty("--i", i);
      }
      card.querySelector(".emoji").textContent = plant.emoji;
      card.querySelector(".name").textContent = plant.name;
      card.querySelector(".when").textContent = P.label(days);
      // The meter shows how much water is left: full right after watering, empty when due.
      const fill = Math.max(0, Math.min(1, days / plant.everyDays));
      card.querySelector(".meter span").style.setProperty("--fill", fill);
      card.querySelector(".water").setAttribute("aria-label", `Water ${plant.name}`);
      const remove = card.querySelector(".remove");
      remove.setAttribute("aria-label", `Remove ${plant.name}`);
      remove.title = `Remove ${plant.name}`;
      return card;
    }));
    firstRender = false;
  }

  // ---------- Feedback ----------
  function toast(text) {
    const t = document.createElement("div");
    t.className = "toast";
    t.textContent = text;
    $("#toasts").append(t);
    setTimeout(() => {
      t.classList.add("leaving");
      setTimeout(() => t.remove(), 250);
    }, 2400);
  }

  // Little droplets fall from the button that was pressed.
  function droplets(fromEl) {
    if (reduceMotion.matches) return;
    const box = fromEl.getBoundingClientRect();
    for (let i = 0; i < 6; i++) {
      const d = document.createElement("span");
      d.className = "drop";
      d.style.left = `${box.left + box.width / 2 - 5 + (i - 2.5) * 6}px`;
      d.style.top = `${box.top + box.height / 2}px`;
      d.style.setProperty("--dx", `${(i - 2.5) * 10}px`);
      d.style.animationDelay = `${i * 40}ms`;
      d.addEventListener("animationend", () => d.remove());
      document.body.append(d);
    }
  }

  // Leaves burst out of the header when nobody is thirsty any more.
  function celebrate() {
    if (reduceMotion.matches) return;
    const box = $(".logo").getBoundingClientRect();
    const leaves = ["🍃", "🌿", "✨", "🌸"];
    for (let i = 0; i < 24; i++) {
      const l = document.createElement("span");
      l.className = "leaf";
      l.textContent = leaves[i % leaves.length];
      l.style.left = `${box.left + box.width / 2}px`;
      l.style.top = `${box.top + box.height / 2}px`;
      const angle = (i / 24) * Math.PI * 2;
      const dist = 80 + Math.random() * 120;
      l.style.setProperty("--x", `${Math.cos(angle) * dist}px`);
      l.style.setProperty("--y", `${Math.sin(angle) * dist + 60}px`);
      l.style.setProperty("--r", `${Math.random() * 540 - 270}deg`);
      l.addEventListener("animationend", () => l.remove());
      document.body.append(l);
    }
  }

  // ---------- Actions ----------
  function waterPlant(id, button) {
    const plant = plants.find((p) => p.id === id);
    const neededWater = plants.some((p) => P.daysUntilWater(p, todayKey) <= 0);
    droplets(button);
    setPlants(P.water(plants, id, todayKey));
    const card = list.querySelector(`[data-id="${id}"]`);
    card.classList.add("watered");
    card.addEventListener("animationend", () => card.classList.remove("watered"), { once: true });
    card.querySelector(".water").focus();            // the card moved: keep the keyboard on it
    toast(`💧 ${plant.name} says thank you!`);
    if (neededWater && plants.every((p) => P.daysUntilWater(p, todayKey) > 0)) {
      celebrate();
      toast("🌞 Every plant is happy!");
    }
  }

  function removePlant(id) {
    const plant = plants.find((p) => p.id === id);
    const card = list.querySelector(`[data-id="${id}"]`);
    const finish = () => {
      setPlants(P.removePlant(plants, id));
      toast(`👋 Said goodbye to ${plant.name}`);
      $("#add").focus();
    };
    if (reduceMotion.matches) return finish();
    card.classList.add("removing");
    card.addEventListener("animationend", finish, { once: true });
  }

  // Event delegation: one listener for every card, even ones drawn later.
  list.addEventListener("click", (e) => {
    const card = e.target.closest(".card");
    if (!card) return;
    const id = Number(card.dataset.id);
    if (e.target.closest(".water")) waterPlant(id, e.target.closest(".water"));
    else if (e.target.closest(".remove")) removePlant(id);
  });

  // ---------- The add dialog ----------
  const everyInput = form.elements.everyDays;
  const everyOut = $("#every-out");
  const showEvery = () => {
    const n = Number(everyInput.value);
    everyOut.textContent = n === 1 ? "day" : `${n} days`;
  };
  everyInput.addEventListener("input", showEvery);

  function openDialog() {
    form.reset();
    showEvery();
    $("#error").textContent = "";
    dialog.showModal();                               // Esc closes it, focus is trapped inside
    form.elements.name.focus();
  }
  $("#add").addEventListener("click", openDialog);
  $("#cancel").addEventListener("click", () => dialog.close());
  dialog.addEventListener("close", () => $("#add").focus());

  form.addEventListener("submit", (e) => {
    e.preventDefault();
    const data = Object.fromEntries(new FormData(form));
    try {
      const next = P.addPlant(plants, data, todayKey);
      dialog.close();
      setPlants(next);
      toast(`🌱 Welcome home, ${next[next.length - 1].name}!`);
    } catch (err) {
      const error = $("#error");
      error.textContent = "";                          // restart the shake animation
      requestAnimationFrame(() => { error.textContent = err.message; });
      form.elements.name.focus();
    }
  });

  // ---------- Theme ----------
  const themeBtn = $("#theme");
  function applyTheme(theme) {
    document.documentElement.dataset.theme = theme;
    const next = theme === "dark" ? "light" : "dark";
    themeBtn.setAttribute("aria-label", `Switch to ${next} theme`);
    themeBtn.querySelector("use").setAttribute("href", theme === "dark" ? "#i-sun" : "#i-moon");
  }
  function toggleTheme() {
    const theme = document.documentElement.dataset.theme === "dark" ? "light" : "dark";
    writeStorage(THEME_KEY, theme);
    applyTheme(theme);
    if (!reduceMotion.matches) {
      themeBtn.animate([{ transform: "rotate(0) scale(1)" }, { transform: "rotate(200deg) scale(.7)" }, { transform: "rotate(360deg) scale(1)" }],
        { duration: 450, easing: "cubic-bezier(.34, 1.56, .64, 1)" });
    }
  }
  themeBtn.addEventListener("click", toggleTheme);
  applyTheme(readStorage(THEME_KEY) || (matchMedia("(prefers-color-scheme: dark)").matches ? "dark" : "light"));

  // ---------- Keyboard shortcuts (not while typing) ----------
  document.addEventListener("keydown", (e) => {
    if (e.ctrlKey || e.metaKey || e.altKey || dialog.open) return;
    if (e.target.closest("input, textarea, select")) return;
    if (e.key === "n" || e.key === "N") { e.preventDefault(); openDialog(); }
    if (e.key === "t" || e.key === "T") toggleTheme();
  });

  // A new day may start while the app is open: redraw when it's shown again.
  document.addEventListener("visibilitychange", () => {
    if (document.visibilityState === "visible" && P.today() !== todayKey) {
      todayKey = P.today();
      render();
    }
  });

  render();
})();
@@file plants.test.js
// Run: node --test     (needs Node.js 20 or newer)
const test = require("node:test");
const assert = require("node:assert/strict");
const P = require("./plants.js");

const T = "2026-11-01";
const plant = (over) => ({ id: 1, name: "Fern", emoji: "🌿", everyDays: 3, lastWatered: T, waterings: 0, ...over });

test("days until water use calendar days", () => {
  assert.equal(P.daysUntilWater(plant({ lastWatered: "2026-10-29" }), T), 0);
  assert.equal(P.daysUntilWater(plant({ lastWatered: "2026-10-31", everyDays: 7 }), T), 6);
  assert.equal(P.daysUntilWater(plant({ lastWatered: "2026-02-27", everyDays: 3 }), "2026-03-03"), -1);
});

test("status", () => {
  assert.equal(P.status(plant({ lastWatered: "2026-10-25" }), T), "thirsty");
  assert.equal(P.status(plant({ lastWatered: "2026-10-28" }), T), "thirsty", "one day late is already thirsty");
  assert.equal(P.status(plant({ lastWatered: "2026-10-29", everyDays: 3 }), "2026-10-31"), "soon");
  assert.equal(P.status(plant({ lastWatered: "2026-10-29", everyDays: 3 }), "2026-11-01"), "today");
  assert.equal(P.status(plant({ lastWatered: "2026-10-29", everyDays: 3 }), "2026-10-30"), "happy");
});

test("labels", () => {
  assert.deepEqual([-3, -1, 0, 1, 2, 14].map(P.label),
    ["3 days overdue", "1 day overdue", "Water today", "Tomorrow", "In 2 days", "In 14 days"]);
});

test("watering returns a new list and leaves the others alone", () => {
  const a = plant({ id: 1, lastWatered: "2026-10-20", waterings: 5 });
  const b = plant({ id: 2, name: "Cactus" });
  const before = [a, b];
  const after = P.water(before, 1, T);
  assert.notEqual(after, before);
  assert.deepEqual(after[0], { ...a, lastWatered: T, waterings: 6 });
  assert.equal(after[1], b, "untouched plants are the same objects");
  assert.equal(a.lastWatered, "2026-10-20", "the old plant didn't change");
  assert.equal(P.water(before, 99, T).length, 2, "an unknown id changes nothing");
});

test("sorting by need", () => {
  const list = [
    plant({ id: 1, name: "zz plant", lastWatered: T, everyDays: 10 }),
    plant({ id: 2, name: "Basil", lastWatered: "2026-10-28", everyDays: 2 }),
    plant({ id: 3, name: "aloe", lastWatered: T, everyDays: 10 }),
    plant({ id: 4, name: "Mint", lastWatered: "2026-10-31", everyDays: 1 }),
  ];
  assert.deepEqual(P.sortByNeed(list, T).map((p) => p.id), [2, 4, 3, 1]);
  assert.deepEqual(list.map((p) => p.id), [1, 2, 3, 4], "the original order is kept");
});

test("adding plants", () => {
  let list = P.addPlant([], { name: "  Pothos ", emoji: "🍃", everyDays: "5" }, T);
  list = P.addPlant(list, { name: "Ivy", everyDays: 4 }, T);
  assert.deepEqual(list.map((p) => [p.id, p.name, p.emoji, p.everyDays]), [[1, "Pothos", "🍃", 5], [2, "Ivy", "🪴", 4]]);
  assert.throws(() => P.addPlant(list, { name: " ", everyDays: 3 }, T), /Give your plant a name/);
  assert.throws(() => P.addPlant(list, { name: "x".repeat(31), everyDays: 3 }, T), /30 characters/);
  assert.throws(() => P.addPlant(list, { name: "Ok", everyDays: 0 }, T), /1 to 60/);
  assert.throws(() => P.addPlant(list, { name: "Ok", everyDays: 2.5 }, T), /1 to 60/);
});

test("summary", () => {
  assert.match(P.summary([], T), /No plants yet/);
  assert.equal(P.summary([plant({ lastWatered: T })], T), "Everyone is happy 🌞");
  assert.equal(P.summary([plant({ lastWatered: "2026-10-01" }), plant({ lastWatered: "2026-10-29" })], T), "2 plants need water");
});

test("loading saved data", () => {
  assert.equal(P.load("not json", T).length, 4, "broken data gives the samples");
  assert.equal(P.load('[{"name": 5}]', T).length, 4, "wrong shapes give the samples");
  const saved = JSON.stringify([{ id: 7, name: "Old", everyDays: 3, lastWatered: T }]);
  assert.deepEqual(P.load(saved, T), [{ id: 7, name: "Old", emoji: "🪴", everyDays: 3, lastWatered: T, waterings: 0 }]);
});
@@file package.json
{
  "name": "plant-buddy",
  "private": true,
  "type": "commonjs",
  "scripts": {
    "test": "node --test"
  }
}
@@file app-icon.svg
<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 64 64">
  <!-- The app icon: one bold silhouette (a sprout in a pot) on a rounded square, readable at 16 px. -->
  <rect width="64" height="64" rx="16" fill="#16a34a"/>
  <path d="M32 36V22" stroke="#dcfce7" stroke-width="4" stroke-linecap="round"/>
  <path d="M32 26c-7 0-11-4-11-11 7 0 11 4 11 11z" fill="#bbf7d0"/>
  <path d="M32 24c0-7 4-11 11-11 0 7-4 11-11 11z" fill="#dcfce7"/>
  <path d="M17 36h30l-4 16H21z" fill="#fb923c"/>
  <rect x="15" y="33" width="34" height="6" rx="3" fill="#fdba74"/>
</svg>
@@text
## Make it yours

These are design tasks as much as code tasks:

- **Draw two new icons** for the sprite, on the same 24-unit grid with the same 2-unit stroke: a sun for a "needs light" note and a calendar for a "last watered" line. Check them next to the others at 20 px.
- **A "soon" animation**: a gentle wiggle on the emoji of plants that need water tomorrow, so they ask politely before they get thirsty.
- **An undo toast**: "Said goodbye to Spike. *Undo*". Keep the removed plant for 5 seconds and put it back if the button is pressed.
- **A third theme**: "sakura" pinks or "ocean" blues. With tokens it's only new values for the same names.
- **Ship it as a desktop app**: wrap the folder with Tauri (`npm create tauri-app`, then copy the files into the front end), and turn `app-icon.svg` into every icon size with `npx tauri icon`.
- **A tkinter or WPF version**: reuse the same rules (port `plants.js` to Python or C#), and recreate the look with `ttk.Style` or a WPF `ResourceDictionary`. The logic tests port almost line for line.
@@project plant_buddy_finished Plant Buddy: the finished version
A complete solution to compare with yours.
@@file index.html
<!doctype html>
<html lang="en">
<head>
  <meta charset="utf-8">
  <meta name="viewport" content="width=device-width, initial-scale=1">
  <title>Plant Buddy</title>
  <meta http-equiv="Content-Security-Policy" content="default-src 'self'; style-src 'self'; img-src 'self' data:">
  <link rel="icon" href="app-icon.svg" type="image/svg+xml">
  <link rel="stylesheet" href="style.css">
</head>
<body>
  <!-- The icon set: one 24 x 24 grid, 2-unit stroke, round caps and joins, drawn inside 2..22.
       Each <symbol> is used with <svg class="icon"><use href="#i-name"/></svg> and takes the text colour. -->
  <svg class="sprite" aria-hidden="true">
    <symbol id="i-drop" viewBox="0 0 24 24"><path d="M12 3c3.5 4.4 6 7.8 6 11a6 6 0 0 1-12 0c0-3.2 2.5-6.6 6-11z"/><path d="M9.5 14.5a2.5 2.5 0 0 0 2.5 2.5"/></symbol>
    <symbol id="i-leaf" viewBox="0 0 24 24"><path d="M5 19c0-8 5-13 14-14 0 9-5 14-13 14z"/><path d="M5 19l7-7"/></symbol>
    <symbol id="i-sun" viewBox="0 0 24 24"><circle cx="12" cy="12" r="4"/><path d="M12 2v2M12 20v2M2 12h2M20 12h2M4.9 4.9l1.4 1.4M17.7 17.7l1.4 1.4M4.9 19.1l1.4-1.4M17.7 6.3l1.4-1.4"/></symbol>
    <symbol id="i-moon" viewBox="0 0 24 24"><path d="M20 14.5A8 8 0 1 1 9.5 4a6.5 6.5 0 0 0 10.5 10.5z"/></symbol>
    <symbol id="i-plus" viewBox="0 0 24 24"><path d="M12 5v14M5 12h14"/></symbol>
    <symbol id="i-trash" viewBox="0 0 24 24"><path d="M4 7h16M10 11v6M14 11v6M6 7l1 13h10l1-13M9 7V4h6v3"/></symbol>
    <symbol id="i-pot" viewBox="0 0 24 24"><path d="M5 13h14l-2 8H7z"/><path d="M12 13V8"/><path d="M12 9c-3 0-5-2-5-5 3 0 5 2 5 5zM12 8c0-3 2-5 5-5 0 3-2 5-5 5z"/></symbol>
    <symbol id="i-x" viewBox="0 0 24 24"><path d="M6 6l12 12M18 6L6 18"/></symbol>
  </svg>

  <header class="top">
    <div class="brand">
      <img src="app-icon.svg" alt="" width="40" height="40" class="logo">
      <div>
        <h1>Plant Buddy</h1>
        <p id="summary" class="muted"></p>
      </div>
    </div>
    <div class="actions">
      <button id="theme" class="icon-btn" aria-label="Switch to dark theme" title="Theme (T)">
        <svg class="icon"><use href="#i-moon"/></svg>
      </button>
      <button id="add" class="btn" title="New plant (N)">
        <svg class="icon" aria-hidden="true"><use href="#i-plus"/></svg> New plant
      </button>
    </div>
  </header>

  <main>
    <ul id="plants" class="grid" aria-label="Your plants"></ul>
    <div id="empty" class="empty" hidden>
      <svg class="icon big"><use href="#i-pot"/></svg>
      <p>No plants yet. Press <kbd>N</kbd> to add your first one.</p>
    </div>
  </main>

  <template id="card">
    <li class="card">
      <div class="face" aria-hidden="true"><span class="emoji"></span></div>
      <div class="info">
        <h2 class="name"></h2>
        <p class="when"></p>
        <div class="meter" aria-hidden="true"><span></span></div>
      </div>
      <div class="card-actions">
        <button class="btn water"><svg class="icon" aria-hidden="true"><use href="#i-drop"/></svg> Water</button>
        <button class="icon-btn remove"><svg class="icon"><use href="#i-trash"/></svg></button>
      </div>
    </li>
  </template>

  <dialog id="dialog" aria-labelledby="dialog-title">
    <form id="form" method="dialog">
      <header class="dialog-head">
        <h2 id="dialog-title">A new plant</h2>
        <button type="button" class="icon-btn" id="cancel" aria-label="Close"><svg class="icon"><use href="#i-x"/></svg></button>
      </header>
      <label>Name <input name="name" maxlength="30" required autocomplete="off" placeholder="e.g. Fernando the Fern"></label>
      <fieldset class="emojis">
        <legend>Pick a face</legend>
        <label><input type="radio" name="emoji" value="🪴" checked><span>🪴</span></label>
        <label><input type="radio" name="emoji" value="🌿"><span>🌿</span></label>
        <label><input type="radio" name="emoji" value="🌵"><span>🌵</span></label>
        <label><input type="radio" name="emoji" value="🌱"><span>🌱</span></label>
        <label><input type="radio" name="emoji" value="🌻"><span>🌻</span></label>
        <label><input type="radio" name="emoji" value="🌷"><span>🌷</span></label>
      </fieldset>
      <label>Water every <output id="every-out">3 days</output>
        <input name="everyDays" type="range" min="1" max="30" value="3">
      </label>
      <p id="error" class="error" role="alert"></p>
      <div class="dialog-actions">
        <button type="submit" class="btn">Add plant</button>
      </div>
    </form>
  </dialog>

  <div id="toasts" class="toasts" role="status"></div>

  <script src="plants.js"></script>
  <script src="app.js"></script>
</body>
</html>
@@file style.css
/* ---------- Design tokens: every colour, gap, radius and timing lives here ---------- */
:root {
  color-scheme: light;
  --bg: #f4fbf5;
  --surface: #ffffff;
  --surface-2: #ecf7ee;
  --text: #17301f;
  --muted: #5d7464;
  --border: #d8eadb;
  --accent: #16a34a;
  --accent-hover: #15803d;
  --accent-text: #ffffff;
  --ring: #86efac;
  --water: #0ea5e9;
  --thirsty: #e11d48;
  --today: #d97706;
  --soon: #ca8a04;
  --happy: #16a34a;

  --space-1: 4px; --space-2: 8px; --space-3: 16px; --space-4: 24px; --space-5: 40px;
  --radius: 18px;
  --radius-sm: 10px;
  --shadow: 0 1px 2px rgb(23 48 31 / .06), 0 10px 30px rgb(23 48 31 / .08);
  --shadow-lift: 0 2px 4px rgb(23 48 31 / .08), 0 18px 40px rgb(23 48 31 / .14);

  --ease-out: cubic-bezier(.2, .8, .3, 1);
  --ease-bounce: cubic-bezier(.34, 1.56, .64, 1);
  --dur-fast: 150ms;
  --dur: 300ms;
}

:root[data-theme="dark"] {
  color-scheme: dark;
  --bg: #0f1a13;
  --surface: #172419;
  --surface-2: #1d2e21;
  --text: #e3f2e6;
  --muted: #9bb3a1;
  --border: #29402e;
  --accent: #4ade80;
  --accent-hover: #86efac;
  --accent-text: #0f1a13;
  --ring: #166534;
  --water: #38bdf8;
  --thirsty: #fb7185;
  --today: #fbbf24;
  --soon: #facc15;
  --happy: #4ade80;
  --shadow: 0 1px 2px rgb(0 0 0 / .3), 0 10px 30px rgb(0 0 0 / .3);
  --shadow-lift: 0 2px 4px rgb(0 0 0 / .35), 0 18px 40px rgb(0 0 0 / .45);
}

/* ---------- Base ---------- */
* { box-sizing: border-box; }
body {
  margin: 0;
  background: var(--bg);
  color: var(--text);
  font: 16px/1.5 "Nunito", system-ui, -apple-system, "Segoe UI", sans-serif;
  transition: background-color var(--dur), color var(--dur);
}
h1, h2 { margin: 0; line-height: 1.2; }
.muted { color: var(--muted); margin: 0; }
.sprite { position: absolute; width: 0; height: 0; }
.icon {
  width: 20px; height: 20px; flex: none;
  fill: none; stroke: currentColor; stroke-width: 2; stroke-linecap: round; stroke-linejoin: round;
}
.icon.big { width: 56px; height: 56px; stroke-width: 1.5; }
kbd { font: 600 .85em ui-monospace, monospace; background: var(--surface-2); border: 1px solid var(--border); border-radius: 6px; padding: 1px 6px; }

/* ---------- Buttons: default, hover, active, focus-visible, disabled ---------- */
.btn, .icon-btn {
  display: inline-flex; align-items: center; justify-content: center; gap: var(--space-2);
  border: 0; font: inherit; font-weight: 700; cursor: pointer;
  transition: background-color var(--dur-fast), transform var(--dur-fast) var(--ease-bounce), box-shadow var(--dur-fast), color var(--dur-fast);
}
.btn {
  background: var(--accent); color: var(--accent-text);
  border-radius: 999px; padding: var(--space-2) var(--space-3);
  box-shadow: 0 2px 0 rgb(0 0 0 / .12);
}
.btn:hover { background: var(--accent-hover); transform: translateY(-1px); }
.btn:active { transform: translateY(1px) scale(.97); box-shadow: none; }
.icon-btn {
  width: 40px; height: 40px; border-radius: 50%;
  background: transparent; color: var(--muted);
}
.icon-btn:hover { background: var(--surface-2); color: var(--text); }
.icon-btn:active { transform: scale(.9); }
.btn:focus-visible, .icon-btn:focus-visible, input:focus-visible + span, input:focus-visible {
  outline: 3px solid var(--ring); outline-offset: 2px;
}
.btn:disabled { opacity: .5; cursor: not-allowed; transform: none; }

/* ---------- Layout ---------- */
.top {
  display: flex; align-items: center; justify-content: space-between; gap: var(--space-3); flex-wrap: wrap;
  max-width: 960px; margin: 0 auto; padding: var(--space-4) var(--space-3);
}
.brand { display: flex; align-items: center; gap: var(--space-3); }
.logo { border-radius: 12px; animation: sway 4s ease-in-out infinite; transform-origin: 50% 100%; }
.actions { display: flex; align-items: center; gap: var(--space-2); }
main { max-width: 960px; margin: 0 auto; padding: 0 var(--space-3) var(--space-5); }
.grid {
  list-style: none; margin: 0; padding: 0;
  display: grid; gap: var(--space-3);
  grid-template-columns: repeat(auto-fill, minmax(260px, 1fr));
}

/* ---------- Plant cards ---------- */
.card {
  --status: var(--happy);
  background: var(--surface); border: 1px solid var(--border); border-radius: var(--radius);
  box-shadow: var(--shadow); padding: var(--space-3);
  display: grid; grid-template-columns: auto 1fr; gap: var(--space-3); align-items: center;
  transition: transform var(--dur) var(--ease-bounce), box-shadow var(--dur);
}
.card.enter {                                       /* the first render: a staggered entrance */
  animation: card-in var(--dur) var(--ease-out) both;
  animation-delay: calc(var(--i, 0) * 60ms);        /* app.js sets --i */
}
.card:hover { transform: translateY(-3px); box-shadow: var(--shadow-lift); }
.card[data-status="thirsty"] { --status: var(--thirsty); }
.card[data-status="today"] { --status: var(--today); }
.card[data-status="soon"] { --status: var(--soon); }

.face {
  width: 64px; height: 64px; border-radius: 50%;
  display: grid; place-items: center; font-size: 34px;
  background: color-mix(in srgb, var(--status) 16%, var(--surface));
  box-shadow: inset 0 0 0 3px color-mix(in srgb, var(--status) 45%, transparent);
}
.emoji { display: inline-block; transform-origin: 50% 90%; }
.card[data-status="happy"] .emoji { animation: sway 3.5s ease-in-out infinite; }
.card[data-status="thirsty"] .emoji { animation: droop 2.4s ease-in-out infinite; filter: saturate(.6); }
.name { font-size: 1.1rem; font-weight: 800; overflow-wrap: anywhere; }
.when { margin: 2px 0 var(--space-2); color: var(--status); font-weight: 700; font-size: .95rem; }
.meter { height: 6px; border-radius: 999px; background: var(--surface-2); overflow: hidden; }
.meter span {
  display: block; height: 100%; width: calc(var(--fill, 1) * 100%);
  background: var(--status); border-radius: inherit;
  transition: width 600ms var(--ease-bounce);
}
.card-actions { grid-column: 1 / -1; display: flex; justify-content: space-between; align-items: center; }
.water { background: var(--water); color: white; }
.water:hover { background: color-mix(in srgb, var(--water) 85%, black); }

/* Watering: the card does a little hop, and droplets fall from the button (made in app.js) */
.card.watered { animation: hop 500ms var(--ease-bounce); }
.drop {
  position: fixed; width: 10px; height: 14px; pointer-events: none;
  background: var(--water); border-radius: 50% 50% 50% 50% / 60% 60% 40% 40%;
  animation: fall 700ms var(--ease-out) forwards;
}
.card.removing { animation: card-out 250ms ease-in forwards; }

/* ---------- Empty state ---------- */
.empty { text-align: center; color: var(--muted); padding: var(--space-5) var(--space-3); }
.empty .icon { color: var(--accent); animation: float 3s ease-in-out infinite; }

/* ---------- Dialog and form ---------- */
dialog {
  border: 0; border-radius: var(--radius); padding: var(--space-4);
  width: min(420px, calc(100vw - 32px));
  background: var(--surface); color: var(--text); box-shadow: var(--shadow-lift);
}
dialog[open] { animation: pop-in var(--dur) var(--ease-bounce); }
dialog::backdrop { background: rgb(15 26 19 / .45); backdrop-filter: blur(3px); }
form { display: grid; gap: var(--space-3); }
.dialog-head { display: flex; justify-content: space-between; align-items: center; }
label { display: grid; gap: var(--space-1); font-weight: 700; }
input[name="name"] {
  font: inherit; padding: 10px 12px; border-radius: var(--radius-sm);
  border: 2px solid var(--border); background: var(--bg); color: var(--text);
  transition: border-color var(--dur-fast);
}
input[name="name"]:focus { border-color: var(--accent); }
input[type="range"] { accent-color: var(--accent); }
output { color: var(--accent); font-variant-numeric: tabular-nums; }
.emojis { border: 0; padding: 0; margin: 0; display: flex; flex-wrap: wrap; gap: var(--space-2); }
.emojis legend { font-weight: 700; margin-bottom: var(--space-1); }
.emojis label { cursor: pointer; }
.emojis input { position: absolute; opacity: 0; width: 1px; height: 1px; }
.emojis span {
  display: grid; place-items: center; width: 44px; height: 44px; font-size: 24px;
  border-radius: 12px; background: var(--surface-2); border: 2px solid transparent;
  transition: transform var(--dur-fast) var(--ease-bounce), border-color var(--dur-fast);
}
.emojis span:hover { transform: scale(1.1); }
.emojis input:checked + span { border-color: var(--accent); transform: scale(1.1); }
.error { color: var(--thirsty); margin: 0; min-height: 1.5em; font-weight: 700; }
.error:not(:empty) { animation: shake 300ms; }
.dialog-actions { display: flex; justify-content: flex-end; }

/* ---------- Toasts ---------- */
.toasts { position: fixed; bottom: var(--space-3); left: 50%; transform: translateX(-50%); display: grid; gap: var(--space-2); z-index: 10; }
.toast {
  background: var(--text); color: var(--bg); padding: 10px 16px; border-radius: 999px;
  box-shadow: var(--shadow-lift); font-weight: 700; display: flex; align-items: center; gap: var(--space-2);
  animation: toast-in var(--dur) var(--ease-bounce);
}
.toast.leaving { opacity: 0; transform: translateY(8px); transition: opacity 200ms, transform 200ms; }

/* ---------- Celebration: leaves burst out when every plant is watered ---------- */
.leaf {
  position: fixed; font-size: 20px; pointer-events: none;
  animation: burst 1100ms var(--ease-out) forwards;
}

/* ---------- Keyframes ---------- */
@keyframes card-in { from { opacity: 0; transform: translateY(14px) scale(.97); } }
@keyframes card-out { to { opacity: 0; transform: scale(.85); } }
@keyframes pop-in { from { opacity: 0; transform: scale(.92); } }
@keyframes toast-in { from { opacity: 0; transform: translateY(14px) scale(.9); } }
@keyframes sway { 0%, 100% { transform: rotate(-4deg); } 50% { transform: rotate(4deg); } }
@keyframes droop { 0%, 100% { transform: rotate(0) translateY(0); } 50% { transform: rotate(-10deg) translateY(2px); } }
@keyframes float { 50% { transform: translateY(-8px); } }
@keyframes hop {
  0%, 100% { transform: translateY(0) scale(1, 1); }
  30% { transform: translateY(-10px) scale(.97, 1.04); }
  65% { transform: translateY(0) scale(1.03, .97); }
}
@keyframes fall { from { opacity: 1; } to { opacity: 0; transform: translate(var(--dx), 70px) scale(.5); } }
@keyframes burst { to { opacity: 0; transform: translate(var(--x), var(--y)) rotate(var(--r)); } }
@keyframes shake { 25% { transform: translateX(-4px); } 75% { transform: translateX(4px); } }

/* ---------- Less motion for people who asked their OS for it ---------- */
@media (prefers-reduced-motion: reduce) {
  *, *::before, *::after {
    animation-duration: 1ms !important;
    animation-iteration-count: 1 !important;
    transition-duration: 1ms !important;
  }
}
@@file plants.js
// plants.js: everything Plant Buddy knows about plants, with no DOM and no storage.
// The page uses it as window.Plants; Node's test runner loads it with require().
// Functions never change the list they're given; they return a new one.
(function (root) {
  "use strict";

  const MAX_NAME = 30;

  // "YYYY-MM-DD" -> a whole day number. Date.UTC ignores time zones and daylight saving,
  // so subtracting two day numbers always gives a whole number of calendar days.
  function dayNumber(key) {
    const [y, m, d] = key.split("-").map(Number);
    return Date.UTC(y, m - 1, d) / 86400000;
  }

  // The local calendar day of a Date, as "YYYY-MM-DD".
  function today(date = new Date()) {
    const m = String(date.getMonth() + 1).padStart(2, "0");
    const d = String(date.getDate()).padStart(2, "0");
    return `${date.getFullYear()}-${m}-${d}`;
  }

  // Days until the plant needs water: 0 = today, negative = overdue.
  function daysUntilWater(plant, todayKey) {
    return dayNumber(plant.lastWatered) + plant.everyDays - dayNumber(todayKey);
  }

  // The plant's status, which picks its colour, icon and animation:
  //   overdue (days < 0) -> "thirsty", 0 -> "today", 1 -> "soon", anything later -> "happy".
  function status(plant, todayKey) {
    const days = daysUntilWater(plant, todayKey);
    if (days < 0) return "thirsty";
    if (days === 0) return "today";
    if (days === 1) return "soon";
    return "happy";
  }

  // The text under the plant's name for a number of days (from daysUntilWater):
  //   -1 -> "1 day overdue", -3 -> "3 days overdue", 0 -> "Water today",
  //   1 -> "Tomorrow", 4 -> "In 4 days".
  function label(days) {
    if (days < 0) return `${-days} day${days === -1 ? "" : "s"} overdue`;
    if (days === 0) return "Water today";
    if (days === 1) return "Tomorrow";
    return `In ${days} days`;
  }

  // A new list where the plant with this id has lastWatered = todayKey and one more
  // in its `waterings` count. Every other plant is the same object as before.
  function water(plants, id, todayKey) {
    return plants.map((p) => (p.id === id ? { ...p, lastWatered: todayKey, waterings: p.waterings + 1 } : p));
  }

  // A new, sorted list: the plant that needs water soonest first; ties by name, A to Z,
  // ignoring upper/lower case (use localeCompare with { sensitivity: "base" }).
  function sortByNeed(plants, todayKey) {
    return [...plants].sort((a, b) =>
      daysUntilWater(a, todayKey) - daysUntilWater(b, todayKey) ||
      a.name.localeCompare(b.name, undefined, { sensitivity: "base" }));
  }

  // Adds a plant, or throws an Error whose message the form shows.
  function addPlant(plants, { name, emoji, everyDays }, todayKey) {
    name = String(name ?? "").trim();
    everyDays = Number(everyDays);
    if (!name) throw new Error("Give your plant a name");
    if (name.length > MAX_NAME) throw new Error(`Names can be ${MAX_NAME} characters at most`);
    if (!Number.isInteger(everyDays) || everyDays < 1 || everyDays > 60) {
      throw new Error("Water every 1 to 60 days");
    }
    const id = plants.reduce((max, p) => Math.max(max, p.id), 0) + 1;
    return [...plants, { id, name, emoji: emoji || "🪴", everyDays, lastWatered: todayKey, waterings: 0 }];
  }

  function removePlant(plants, id) {
    return plants.filter((p) => p.id !== id);
  }

  // A friendly summary for the header.
  function summary(plants, todayKey) {
    if (plants.length === 0) return "No plants yet. Add your first one!";
    const need = plants.filter((p) => daysUntilWater(p, todayKey) <= 0).length;
    if (need === 0) return "Everyone is happy 🌞";
    return `${need} plant${need === 1 ? " needs" : "s need"} water`;
  }

  function samplePlants(todayKey) {
    const back = (n) => {
      const [y, m, d] = todayKey.split("-").map(Number);
      return today(new Date(y, m - 1, d - n));
    };
    return [
      { id: 1, name: "Monty the Monstera", emoji: "🌿", everyDays: 7, lastWatered: back(8), waterings: 12 },
      { id: 2, name: "Spike", emoji: "🌵", everyDays: 21, lastWatered: back(3), waterings: 4 },
      { id: 3, name: "Basil", emoji: "🌱", everyDays: 2, lastWatered: back(2), waterings: 30 },
      { id: 4, name: "Sunny", emoji: "🌻", everyDays: 3, lastWatered: back(2), waterings: 9 },
    ];
  }

  // Reads saved JSON; anything broken or missing gives the sample plants instead.
  function load(json, todayKey) {
    try {
      const data = JSON.parse(json);
      if (Array.isArray(data) && data.every((p) => p && typeof p.name === "string" && typeof p.lastWatered === "string")) {
        return data.map((p) => ({ waterings: 0, emoji: "🪴", ...p }));
      }
    } catch { /* fall through */ }
    return samplePlants(todayKey);
  }

  const api = { MAX_NAME, dayNumber, today, daysUntilWater, status, label, water, sortByNeed,
    addPlant, removePlant, summary, samplePlants, load };
  if (typeof module === "object" && module.exports) module.exports = api;
  else root.Plants = api;
})(globalThis);
@@file app.js
// app.js: the window. It keeps the state, saves it, draws it, and plays the animations.
// All the rules live in plants.js (window.Plants), so this file stays about the GUI.
(function () {
  "use strict";
  const P = window.Plants;
  const KEY = "plant-buddy";
  const THEME_KEY = "plant-buddy-theme";
  const reduceMotion = matchMedia("(prefers-reduced-motion: reduce)");

  const $ = (sel) => document.querySelector(sel);
  const list = $("#plants");
  const template = $("#card");
  const dialog = $("#dialog");
  const form = $("#form");

  // ---------- State ----------
  let todayKey = P.today();
  let plants = P.load(readStorage(KEY), todayKey);
  let firstRender = true;

  function readStorage(key) {
    try { return localStorage.getItem(key); } catch { return null; }
  }
  function writeStorage(key, value) {
    try { localStorage.setItem(key, value); } catch { /* private mode: keep working without saving */ }
  }

  function setPlants(next) {
    plants = next;
    writeStorage(KEY, JSON.stringify(plants));
    render();
  }

  // ---------- Drawing ----------
  function render() {
    const sorted = P.sortByNeed(plants, todayKey);
    $("#summary").textContent = P.summary(plants, todayKey);
    $("#empty").hidden = plants.length > 0;

    list.replaceChildren(...sorted.map((plant, i) => {
      const card = template.content.firstElementChild.cloneNode(true);
      const days = P.daysUntilWater(plant, todayKey);
      card.dataset.id = plant.id;
      card.dataset.status = P.status(plant, todayKey);
      // Only the first render staggers in; later renders appear in place.
      if (firstRender) {
        card.classList.add("enter");
        card.style.setProperty("--i", i);
      }
      card.querySelector(".emoji").textContent = plant.emoji;
      card.querySelector(".name").textContent = plant.name;
      card.querySelector(".when").textContent = P.label(days);
      // The meter shows how much water is left: full right after watering, empty when due.
      const fill = Math.max(0, Math.min(1, days / plant.everyDays));
      card.querySelector(".meter span").style.setProperty("--fill", fill);
      card.querySelector(".water").setAttribute("aria-label", `Water ${plant.name}`);
      const remove = card.querySelector(".remove");
      remove.setAttribute("aria-label", `Remove ${plant.name}`);
      remove.title = `Remove ${plant.name}`;
      return card;
    }));
    firstRender = false;
  }

  // ---------- Feedback ----------
  function toast(text) {
    const t = document.createElement("div");
    t.className = "toast";
    t.textContent = text;
    $("#toasts").append(t);
    setTimeout(() => {
      t.classList.add("leaving");
      setTimeout(() => t.remove(), 250);
    }, 2400);
  }

  // Little droplets fall from the button that was pressed.
  function droplets(fromEl) {
    if (reduceMotion.matches) return;
    const box = fromEl.getBoundingClientRect();
    for (let i = 0; i < 6; i++) {
      const d = document.createElement("span");
      d.className = "drop";
      d.style.left = `${box.left + box.width / 2 - 5 + (i - 2.5) * 6}px`;
      d.style.top = `${box.top + box.height / 2}px`;
      d.style.setProperty("--dx", `${(i - 2.5) * 10}px`);
      d.style.animationDelay = `${i * 40}ms`;
      d.addEventListener("animationend", () => d.remove());
      document.body.append(d);
    }
  }

  // Leaves burst out of the header when nobody is thirsty any more.
  function celebrate() {
    if (reduceMotion.matches) return;
    const box = $(".logo").getBoundingClientRect();
    const leaves = ["🍃", "🌿", "✨", "🌸"];
    for (let i = 0; i < 24; i++) {
      const l = document.createElement("span");
      l.className = "leaf";
      l.textContent = leaves[i % leaves.length];
      l.style.left = `${box.left + box.width / 2}px`;
      l.style.top = `${box.top + box.height / 2}px`;
      const angle = (i / 24) * Math.PI * 2;
      const dist = 80 + Math.random() * 120;
      l.style.setProperty("--x", `${Math.cos(angle) * dist}px`);
      l.style.setProperty("--y", `${Math.sin(angle) * dist + 60}px`);
      l.style.setProperty("--r", `${Math.random() * 540 - 270}deg`);
      l.addEventListener("animationend", () => l.remove());
      document.body.append(l);
    }
  }

  // ---------- Actions ----------
  function waterPlant(id, button) {
    const plant = plants.find((p) => p.id === id);
    const neededWater = plants.some((p) => P.daysUntilWater(p, todayKey) <= 0);
    droplets(button);
    setPlants(P.water(plants, id, todayKey));
    const card = list.querySelector(`[data-id="${id}"]`);
    card.classList.add("watered");
    card.addEventListener("animationend", () => card.classList.remove("watered"), { once: true });
    card.querySelector(".water").focus();            // the card moved: keep the keyboard on it
    toast(`💧 ${plant.name} says thank you!`);
    if (neededWater && plants.every((p) => P.daysUntilWater(p, todayKey) > 0)) {
      celebrate();
      toast("🌞 Every plant is happy!");
    }
  }

  function removePlant(id) {
    const plant = plants.find((p) => p.id === id);
    const card = list.querySelector(`[data-id="${id}"]`);
    const finish = () => {
      setPlants(P.removePlant(plants, id));
      toast(`👋 Said goodbye to ${plant.name}`);
      $("#add").focus();
    };
    if (reduceMotion.matches) return finish();
    card.classList.add("removing");
    card.addEventListener("animationend", finish, { once: true });
  }

  // Event delegation: one listener for every card, even ones drawn later.
  list.addEventListener("click", (e) => {
    const card = e.target.closest(".card");
    if (!card) return;
    const id = Number(card.dataset.id);
    if (e.target.closest(".water")) waterPlant(id, e.target.closest(".water"));
    else if (e.target.closest(".remove")) removePlant(id);
  });

  // ---------- The add dialog ----------
  const everyInput = form.elements.everyDays;
  const everyOut = $("#every-out");
  const showEvery = () => {
    const n = Number(everyInput.value);
    everyOut.textContent = n === 1 ? "day" : `${n} days`;
  };
  everyInput.addEventListener("input", showEvery);

  function openDialog() {
    form.reset();
    showEvery();
    $("#error").textContent = "";
    dialog.showModal();                               // Esc closes it, focus is trapped inside
    form.elements.name.focus();
  }
  $("#add").addEventListener("click", openDialog);
  $("#cancel").addEventListener("click", () => dialog.close());
  dialog.addEventListener("close", () => $("#add").focus());

  form.addEventListener("submit", (e) => {
    e.preventDefault();
    const data = Object.fromEntries(new FormData(form));
    try {
      const next = P.addPlant(plants, data, todayKey);
      dialog.close();
      setPlants(next);
      toast(`🌱 Welcome home, ${next[next.length - 1].name}!`);
    } catch (err) {
      const error = $("#error");
      error.textContent = "";                          // restart the shake animation
      requestAnimationFrame(() => { error.textContent = err.message; });
      form.elements.name.focus();
    }
  });

  // ---------- Theme ----------
  const themeBtn = $("#theme");
  function applyTheme(theme) {
    document.documentElement.dataset.theme = theme;
    const next = theme === "dark" ? "light" : "dark";
    themeBtn.setAttribute("aria-label", `Switch to ${next} theme`);
    themeBtn.querySelector("use").setAttribute("href", theme === "dark" ? "#i-sun" : "#i-moon");
  }
  function toggleTheme() {
    const theme = document.documentElement.dataset.theme === "dark" ? "light" : "dark";
    writeStorage(THEME_KEY, theme);
    applyTheme(theme);
    if (!reduceMotion.matches) {
      themeBtn.animate([{ transform: "rotate(0) scale(1)" }, { transform: "rotate(200deg) scale(.7)" }, { transform: "rotate(360deg) scale(1)" }],
        { duration: 450, easing: "cubic-bezier(.34, 1.56, .64, 1)" });
    }
  }
  themeBtn.addEventListener("click", toggleTheme);
  applyTheme(readStorage(THEME_KEY) || (matchMedia("(prefers-color-scheme: dark)").matches ? "dark" : "light"));

  // ---------- Keyboard shortcuts (not while typing) ----------
  document.addEventListener("keydown", (e) => {
    if (e.ctrlKey || e.metaKey || e.altKey || dialog.open) return;
    if (e.target.closest("input, textarea, select")) return;
    if (e.key === "n" || e.key === "N") { e.preventDefault(); openDialog(); }
    if (e.key === "t" || e.key === "T") toggleTheme();
  });

  // A new day may start while the app is open: redraw when it's shown again.
  document.addEventListener("visibilitychange", () => {
    if (document.visibilityState === "visible" && P.today() !== todayKey) {
      todayKey = P.today();
      render();
    }
  });

  render();
})();
@@file plants.test.js
// Run: node --test     (needs Node.js 20 or newer)
const test = require("node:test");
const assert = require("node:assert/strict");
const P = require("./plants.js");

const T = "2026-11-01";
const plant = (over) => ({ id: 1, name: "Fern", emoji: "🌿", everyDays: 3, lastWatered: T, waterings: 0, ...over });

test("days until water use calendar days", () => {
  assert.equal(P.daysUntilWater(plant({ lastWatered: "2026-10-29" }), T), 0);
  assert.equal(P.daysUntilWater(plant({ lastWatered: "2026-10-31", everyDays: 7 }), T), 6);
  assert.equal(P.daysUntilWater(plant({ lastWatered: "2026-02-27", everyDays: 3 }), "2026-03-03"), -1);
});

test("status", () => {
  assert.equal(P.status(plant({ lastWatered: "2026-10-25" }), T), "thirsty");
  assert.equal(P.status(plant({ lastWatered: "2026-10-28" }), T), "thirsty", "one day late is already thirsty");
  assert.equal(P.status(plant({ lastWatered: "2026-10-29", everyDays: 3 }), "2026-10-31"), "soon");
  assert.equal(P.status(plant({ lastWatered: "2026-10-29", everyDays: 3 }), "2026-11-01"), "today");
  assert.equal(P.status(plant({ lastWatered: "2026-10-29", everyDays: 3 }), "2026-10-30"), "happy");
});

test("labels", () => {
  assert.deepEqual([-3, -1, 0, 1, 2, 14].map(P.label),
    ["3 days overdue", "1 day overdue", "Water today", "Tomorrow", "In 2 days", "In 14 days"]);
});

test("watering returns a new list and leaves the others alone", () => {
  const a = plant({ id: 1, lastWatered: "2026-10-20", waterings: 5 });
  const b = plant({ id: 2, name: "Cactus" });
  const before = [a, b];
  const after = P.water(before, 1, T);
  assert.notEqual(after, before);
  assert.deepEqual(after[0], { ...a, lastWatered: T, waterings: 6 });
  assert.equal(after[1], b, "untouched plants are the same objects");
  assert.equal(a.lastWatered, "2026-10-20", "the old plant didn't change");
  assert.equal(P.water(before, 99, T).length, 2, "an unknown id changes nothing");
});

test("sorting by need", () => {
  const list = [
    plant({ id: 1, name: "zz plant", lastWatered: T, everyDays: 10 }),
    plant({ id: 2, name: "Basil", lastWatered: "2026-10-28", everyDays: 2 }),
    plant({ id: 3, name: "aloe", lastWatered: T, everyDays: 10 }),
    plant({ id: 4, name: "Mint", lastWatered: "2026-10-31", everyDays: 1 }),
  ];
  assert.deepEqual(P.sortByNeed(list, T).map((p) => p.id), [2, 4, 3, 1]);
  assert.deepEqual(list.map((p) => p.id), [1, 2, 3, 4], "the original order is kept");
});

test("adding plants", () => {
  let list = P.addPlant([], { name: "  Pothos ", emoji: "🍃", everyDays: "5" }, T);
  list = P.addPlant(list, { name: "Ivy", everyDays: 4 }, T);
  assert.deepEqual(list.map((p) => [p.id, p.name, p.emoji, p.everyDays]), [[1, "Pothos", "🍃", 5], [2, "Ivy", "🪴", 4]]);
  assert.throws(() => P.addPlant(list, { name: " ", everyDays: 3 }, T), /Give your plant a name/);
  assert.throws(() => P.addPlant(list, { name: "x".repeat(31), everyDays: 3 }, T), /30 characters/);
  assert.throws(() => P.addPlant(list, { name: "Ok", everyDays: 0 }, T), /1 to 60/);
  assert.throws(() => P.addPlant(list, { name: "Ok", everyDays: 2.5 }, T), /1 to 60/);
});

test("summary", () => {
  assert.match(P.summary([], T), /No plants yet/);
  assert.equal(P.summary([plant({ lastWatered: T })], T), "Everyone is happy 🌞");
  assert.equal(P.summary([plant({ lastWatered: "2026-10-01" }), plant({ lastWatered: "2026-10-29" })], T), "2 plants need water");
});

test("loading saved data", () => {
  assert.equal(P.load("not json", T).length, 4, "broken data gives the samples");
  assert.equal(P.load('[{"name": 5}]', T).length, 4, "wrong shapes give the samples");
  const saved = JSON.stringify([{ id: 7, name: "Old", everyDays: 3, lastWatered: T }]);
  assert.deepEqual(P.load(saved, T), [{ id: 7, name: "Old", emoji: "🪴", everyDays: 3, lastWatered: T, waterings: 0 }]);
});
@@file package.json
{
  "name": "plant-buddy",
  "private": true,
  "type": "commonjs",
  "scripts": {
    "test": "node --test"
  }
}
@@file app-icon.svg
<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 64 64">
  <!-- The app icon: one bold silhouette (a sprout in a pot) on a rounded square, readable at 16 px. -->
  <rect width="64" height="64" rx="16" fill="#16a34a"/>
  <path d="M32 36V22" stroke="#dcfce7" stroke-width="4" stroke-linecap="round"/>
  <path d="M32 26c-7 0-11-4-11-11 7 0 11 4 11 11z" fill="#bbf7d0"/>
  <path d="M32 24c0-7 4-11 11-11 0 7-4 11-11 11z" fill="#dcfce7"/>
  <path d="M17 36h30l-4 16H21z" fill="#fb923c"/>
  <rect x="15" y="33" width="34" height="6" rx="3" fill="#fdba74"/>
</svg>
@@text
🌱 Tokens, themes, real controls, feedback, motion with taste, and icons drawn by hand: that's what makes the difference between an app that works and an app people *like*. Every GUI you build from now on, in any language, can be made like this.
