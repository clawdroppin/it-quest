@@lesson i2-habits Habit Tracker: an Offline Web App
@@text
Time to build a real web application with no framework at all: a daily habit tracker that remembers your data, works offline, can be **installed** on a phone or a PC like a native app, and is fully usable with a keyboard and a screen reader.

```text
 Habits                          Sunday, November 1
 ┌──────────────────────────────────────────┬─────┐
 │ A new habit, e.g. Stretch for 5 minutes  │ Add │
 └──────────────────────────────────────────┴─────┘
   Mon  Tue  Wed  Thu  Fri  Sat  Sun
 ┌─────────────────────────────────────────────────┐
 │ Stretch   2 days in a row · 7% of last 30 days  │
 │ [ ]  [ ]  [ ]  [ ]  [ ]  [✓]  [✓]               │
 └─────────────────────────────────────────────────┘
```

## How it's built

| File | Job |
|---|---|
| `index.html` | the page structure, a `<template>` for one habit, and a Content Security Policy |
| `style.css` | layout with grid and flexbox, dark mode, visible focus, reduced motion |
| `logic.js` | **all the rules**, as pure functions: add, toggle, streaks, completion rate, loading and upgrading saved data. No DOM, no storage |
| `app.js` | the page: state → `render()`, event delegation, `localStorage`, export/import, the service worker |
| `logic.test.js` | 8 tests for `logic.js`, run with Node: `node --test` |
| `sw.js` + `manifest.webmanifest` + `icon.svg` | make it an installable, offline Progressive Web App |

The most important design decision is the split between `logic.js` and `app.js`. Everything that can be wrong in a subtle way (dates, streaks, data from old versions) is in plain functions that take a state and **return a new state**, which you can test in a second without a browser. The page code only calls them and draws the result.

## Your tasks

Four functions in `logic.js` throw `TODO`. Their comments are the specification, and `logic.test.js` checks them:

1. `toggleDay`: return a new state where one habit's `done` list gains or loses a day, kept sorted. Use `map` and spread (`{ ...h, done }`), never `push` on the old array.
2. `lastNDays`: use the `addDays` helper (it already handles month ends and daylight saving).
3. `streak`: count back from today, or from yesterday if today isn't done yet. A `Set` makes each lookup instant.
4. `completionRate`: `lastNDays`, a filter, and `Math.round`.

```text
node --test
```

Then open `index.html` in a browser. Everything except offline mode works from the file. For offline mode and "Install app", serve the folder over HTTP (service workers don't run on `file://` pages):

```text
python -m http.server 8000      then open http://localhost:8000
```

Try it with the keyboard only (Tab, Space, Enter), in dark mode, and narrow the window to phone width.
@@project habits Habit Tracker
Write the four functions that throw `TODO` in `logic.js` until `node --test` passes, then open `index.html`.
@@file index.html
<!doctype html>
<html lang="en">
<head>
  <meta charset="utf-8">
  <meta name="viewport" content="width=device-width, initial-scale=1">
  <meta http-equiv="Content-Security-Policy" content="default-src 'self'; img-src 'self' data:">
  <meta name="color-scheme" content="light dark">
  <meta name="theme-color" content="#2f6f5e">
  <title>Habits</title>
  <link rel="manifest" href="manifest.webmanifest">
  <link rel="icon" href="icon.svg" type="image/svg+xml">
  <link rel="stylesheet" href="style.css">
</head>
<body>
  <header class="top">
    <h1>Habits</h1>
    <p id="today" class="muted"></p>
  </header>

  <main>
    <form id="add-form" class="add" novalidate>
      <label for="habit-name" class="visually-hidden">New habit</label>
      <input id="habit-name" name="name" type="text" placeholder="A new habit, e.g. Stretch for 5 minutes"
             maxlength="60" autocomplete="off" required>
      <button type="submit">Add</button>
      <p id="form-error" class="error" role="alert" hidden></p>
    </form>

    <section aria-labelledby="list-title">
      <h2 id="list-title" class="visually-hidden">Your habits</h2>
      <div id="days-header" class="days-header" aria-hidden="true"></div>
      <ul id="habits" class="habits"></ul>
      <p id="empty" class="muted empty" hidden>No habits yet. Add one above. Small ones work best.</p>
    </section>

    <section class="data" aria-labelledby="data-title">
      <h2 id="data-title">Your data</h2>
      <p class="muted">Everything stays in this browser. Export a file to back it up or move it to another device.</p>
      <div class="buttons">
        <button type="button" id="export">Export</button>
        <label class="button" for="import">Import…</label>
        <input type="file" id="import" accept="application/json,.json" class="visually-hidden">
      </div>
    </section>
  </main>

  <p id="status" class="visually-hidden" aria-live="polite"></p>

  <template id="habit-template">
    <li class="habit">
      <div class="habit-head">
        <span class="name"></span>
        <span class="stats muted"></span>
        <button type="button" class="remove" data-action="remove">Remove</button>
      </div>
      <div class="days"></div>
    </li>
  </template>

  <script src="logic.js"></script>
  <script src="app.js"></script>
</body>
</html>
@@file style.css
:root {
  --bg: #f6f7f5;
  --card: #ffffff;
  --text: #1d2420;
  --muted: #5d6a63;
  --line: #d9dfdb;
  --accent: #2f6f5e;
  --accent-text: #ffffff;
  --error: #b3261e;
  --radius: 10px;
  font-family: system-ui, -apple-system, "Segoe UI", Roboto, sans-serif;
  color: var(--text);
  background: var(--bg);
}

@media (prefers-color-scheme: dark) {
  :root {
    --bg: #121614;
    --card: #1b211e;
    --text: #e4ebe7;
    --muted: #9aa8a0;
    --line: #2e3833;
    --accent: #6cc3a8;
    --accent-text: #0d1a15;
    --error: #ff8a80;
  }
}

* { box-sizing: border-box; }
body { margin: 0 auto; max-width: 720px; padding: 24px 16px 48px; line-height: 1.5; }
h1 { margin: 0; font-size: 1.8rem; }
h2 { font-size: 1.1rem; }
.muted { color: var(--muted); }
.top { margin-bottom: 20px; }
.top p { margin: 4px 0 0; }

.visually-hidden {
  position: absolute; width: 1px; height: 1px; overflow: hidden;
  clip: rect(0 0 0 0); white-space: nowrap;
}

button, .button {
  font: inherit; cursor: pointer; border-radius: var(--radius);
  border: 1px solid var(--line); background: var(--card); color: var(--text);
  padding: 8px 14px; display: inline-block;
}
button:hover, .button:hover { border-color: var(--accent); }
:focus-visible { outline: 3px solid var(--accent); outline-offset: 2px; }

.add { display: flex; flex-wrap: wrap; gap: 8px; margin-bottom: 24px; }
.add input {
  flex: 1 1 240px; font: inherit; padding: 8px 12px; border-radius: var(--radius);
  border: 1px solid var(--line); background: var(--card); color: var(--text);
}
.add input[aria-invalid="true"] { border-color: var(--error); }
.add button[type="submit"] { background: var(--accent); color: var(--accent-text); border-color: var(--accent); }
.error { color: var(--error); margin: 0; flex-basis: 100%; }

.days-header, .days { display: grid; grid-template-columns: repeat(7, 1fr); gap: 6px; }
.days-header { padding: 0 14px; font-size: 0.8rem; color: var(--muted); text-align: center; }
.habits { list-style: none; margin: 6px 0 0; padding: 0; display: grid; gap: 10px; }
.habit { background: var(--card); border: 1px solid var(--line); border-radius: var(--radius); padding: 12px 14px; }
.habit-head { display: flex; flex-wrap: wrap; align-items: baseline; gap: 4px 12px; margin-bottom: 10px; }
.name { font-weight: 600; }
.stats { font-size: 0.85rem; flex: 1; }
.remove { padding: 2px 10px; font-size: 0.85rem; }

.day { aspect-ratio: 1; padding: 0; font-size: 1.2rem; min-height: 36px; }
.day[aria-pressed="true"] { background: var(--accent); color: var(--accent-text); border-color: var(--accent); }
.day.is-today { border-width: 2px; border-color: var(--accent); }
.empty { text-align: center; padding: 24px; }

.data { margin-top: 40px; border-top: 1px solid var(--line); padding-top: 8px; }
.buttons { display: flex; gap: 8px; }

@media (prefers-reduced-motion: no-preference) {
  .day { transition: background-color 120ms ease; }
}
@@file logic.js
// logic.js: everything the app knows about habits, with no DOM and no storage.
// The page uses it as window.HabitLogic; Node's test runner loads it with require().
// Every function returns a new state instead of changing the old one, so render() can
// trust that the state it was given never changes behind its back.
(function (root) {
  "use strict";

  const VERSION = 2;

  // The local calendar day of a Date, as "YYYY-MM-DD" (not toISOString(), which is UTC).
  function dayKey(date) {
    const y = date.getFullYear();
    const m = String(date.getMonth() + 1).padStart(2, "0");
    const d = String(date.getDate()).padStart(2, "0");
    return `${y}-${m}-${d}`;
  }

  // "YYYY-MM-DD" -> a Date at local midnight.
  function parseDay(key) {
    const [y, m, d] = key.split("-").map(Number);
    return new Date(y, m - 1, d);
  }

  // The day key `n` days after `key` (n may be negative). Uses the calendar, not 24-hour
  // steps, so a daylight-saving change can't skip or repeat a day.
  function addDays(key, n) {
    const date = parseDay(key);
    date.setDate(date.getDate() + n);
    return dayKey(date);
  }

  function emptyState() {
    return { version: VERSION, nextId: 1, habits: [] };
  }

  // Adds a habit called `name` (trimmed). Throws Error("Give the habit a name") for an empty
  // name and Error(`You already have "<name>"`) when one with the same name exists, ignoring case.
  function addHabit(state, name) {
    const clean = String(name).trim();
    if (!clean) throw new Error("Give the habit a name");
    if (state.habits.some((h) => h.name.toLowerCase() === clean.toLowerCase())) {
      throw new Error(`You already have "${clean}"`);
    }
    const habit = { id: state.nextId, name: clean, done: [] };
    return { ...state, nextId: state.nextId + 1, habits: [...state.habits, habit] };
  }

  function removeHabit(state, id) {
    return { ...state, habits: state.habits.filter((h) => h.id !== id) };
  }

  // Marks `day` done for habit `id`, or un-marks it if it was done. `done` stays sorted
  // (oldest first) and never has duplicates. Other habits are untouched. Unknown ids change nothing.
  function toggleDay(state, id, day) {
    throw new Error("TODO: not done yet");
  }

  // The `n` day keys ending with `today`, oldest first: lastNDays("2026-11-01", 3)
  // is ["2026-10-30", "2026-10-31", "2026-11-01"].
  function lastNDays(today, n) {
    throw new Error("TODO: not done yet");
  }

  // How many days in a row the habit has been done, counting back from today. If today
  // isn't done yet, the streak still counts up to yesterday (the day isn't over).
  function streak(habit, today) {
    throw new Error("TODO: not done yet");
  }

  // The percentage (0-100, rounded) of the last `days` days (ending today) that are done.
  function completionRate(habit, today, days) {
    throw new Error("TODO: not done yet");
  }

  // Reads saved text (from localStorage or an imported file) into a state.
  // null or "" -> an empty state. Version 1 stored { habits: [{ name, days: {"YYYY-MM-DD": true} }] }
  // without ids; it's upgraded. Anything else that doesn't look right throws Error("Not a habit file").
  function load(text) {
    if (!text) return emptyState();
    let data;
    try {
      data = JSON.parse(text);
    } catch {
      throw new Error("Not a habit file");
    }
    if (!data || !Array.isArray(data.habits)) throw new Error("Not a habit file");
    if (data.version === undefined || data.version === 1) {
      const habits = data.habits.map((h, i) => ({
        id: i + 1,
        name: String(h.name),
        done: Object.keys(h.days || {}).filter((d) => h.days[d]).sort(),
      }));
      return { version: VERSION, nextId: habits.length + 1, habits };
    }
    if (data.version !== VERSION) throw new Error("Not a habit file");
    const valid = data.habits.every(
      (h) => Number.isInteger(h.id) && typeof h.name === "string" && Array.isArray(h.done) &&
        h.done.every((d) => /^\d{4}-\d{2}-\d{2}$/.test(d)),
    );
    if (!valid || !Number.isInteger(data.nextId)) throw new Error("Not a habit file");
    return data;
  }

  function save(state) {
    return JSON.stringify(state);
  }

  const api = { VERSION, dayKey, parseDay, addDays, emptyState, addHabit, removeHabit, toggleDay,
    lastNDays, streak, completionRate, load, save };
  if (typeof module === "object" && module.exports) module.exports = api;
  else root.HabitLogic = api;
})(globalThis);
@@file app.js
// app.js: the page. It owns the state, saves it, and draws it. All the rules are in logic.js.
(function () {
  "use strict";
  const L = window.HabitLogic;
  const KEY = "habits";
  const DAYS_SHOWN = 7;

  const $ = (id) => document.getElementById(id);
  const list = $("habits");
  const template = $("habit-template");

  let state = readSaved();

  function readSaved() {
    try {
      return L.load(localStorage.getItem(KEY));
    } catch (e) {
      // Storage blocked (private mode) or damaged data: start empty rather than crash.
      console.warn("Could not load saved habits:", e);
      return L.emptyState();
    }
  }

  // The only way the state changes: compute a new one, save it, draw it.
  function update(next, message) {
    state = next;
    try {
      localStorage.setItem(KEY, L.save(state));
    } catch {
      announce("Couldn't save: your browser is blocking storage.");
    }
    render();
    if (message) announce(message);
  }

  function announce(text) {
    $("status").textContent = text;   // read out by screen readers (aria-live)
  }

  const weekday = new Intl.DateTimeFormat(undefined, { weekday: "short" });
  const longDate = new Intl.DateTimeFormat(undefined, { weekday: "long", month: "long", day: "numeric" });

  function render() {
    const today = L.dayKey(new Date());
    const days = L.lastNDays(today, DAYS_SHOWN);
    $("today").textContent = longDate.format(new Date());

    $("days-header").replaceChildren(
      ...days.map((d) => {
        const span = document.createElement("span");
        span.textContent = weekday.format(L.parseDay(d));
        return span;
      }),
    );

    // Rebuilding the list is simple and fast for a few dozen items. textContent (never
    // innerHTML) puts the user's text in the page, so a habit called "<img onerror=...>" is just text.
    list.replaceChildren(
      ...state.habits.map((habit) => {
        const item = template.content.firstElementChild.cloneNode(true);
        item.dataset.id = habit.id;
        item.querySelector(".name").textContent = habit.name;
        const s = L.streak(habit, today);
        item.querySelector(".stats").textContent =
          `${s} day${s === 1 ? "" : "s"} in a row · ${L.completionRate(habit, today, 30)}% of the last 30 days`;
        item.querySelector(".remove").setAttribute("aria-label", `Remove ${habit.name}`);
        const done = new Set(habit.done);
        item.querySelector(".days").replaceChildren(
          ...days.map((d) => {
            const b = document.createElement("button");
            b.type = "button";
            b.className = "day" + (d === today ? " is-today" : "");
            b.dataset.action = "toggle";
            b.dataset.day = d;
            b.setAttribute("aria-pressed", String(done.has(d)));
            b.setAttribute("aria-label", `${habit.name}, ${longDate.format(L.parseDay(d))}`);
            b.textContent = done.has(d) ? "✓" : "";
            return b;
          }),
        );
        return item;
      }),
    );
    $("empty").hidden = state.habits.length > 0;
  }

  // One listener for the whole list (event delegation): it works for items added later too.
  list.addEventListener("click", (event) => {
    const button = event.target.closest("button[data-action]");
    if (!button) return;
    const id = Number(button.closest(".habit").dataset.id);
    const habit = state.habits.find((h) => h.id === id);
    if (button.dataset.action === "toggle") {
      const day = button.dataset.day;
      update(L.toggleDay(state, id, day));
      // render() replaced the button, so put the keyboard focus back on the new one.
      list.querySelector(`.habit[data-id="${id}"] [data-day="${day}"]`)?.focus();
    } else if (button.dataset.action === "remove" && confirm(`Remove "${habit.name}" and its history?`)) {
      update(L.removeHabit(state, id), `Removed ${habit.name}`);
      $("habit-name").focus();
    }
  });

  $("add-form").addEventListener("submit", (event) => {
    event.preventDefault();
    const input = $("habit-name");
    const error = $("form-error");
    try {
      update(L.addHabit(state, input.value), `Added ${input.value.trim()}`);
      input.value = "";
      error.hidden = true;
      input.removeAttribute("aria-invalid");
    } catch (e) {
      error.textContent = e.message;
      error.hidden = false;
      input.setAttribute("aria-invalid", "true");
    }
    input.focus();
  });

  $("export").addEventListener("click", () => {
    const blob = new Blob([L.save(state)], { type: "application/json" });
    const link = document.createElement("a");
    link.href = URL.createObjectURL(blob);
    link.download = `habits-${L.dayKey(new Date())}.json`;
    link.click();
    setTimeout(() => URL.revokeObjectURL(link.href), 1000);
  });

  $("import").addEventListener("change", async (event) => {
    const file = event.target.files[0];
    event.target.value = "";             // so choosing the same file again still fires "change"
    if (!file) return;
    try {
      const imported = L.load(await file.text());
      if (confirm(`Replace your ${state.habits.length} habit(s) with the ${imported.habits.length} in ${file.name}?`)) {
        update(imported, `Imported ${imported.habits.length} habit(s)`);
      }
    } catch (e) {
      alert(`${file.name}: ${e.message}`);
    }
  });

  // Another tab changed the data: show it here too.
  window.addEventListener("storage", (event) => {
    if (event.key === KEY) {
      state = readSaved();
      render();
    }
  });

  // A new day may have started while the page was open in the background.
  document.addEventListener("visibilitychange", () => {
    if (!document.hidden) render();
  });

  // Offline support. Service workers only run on http(s), not on pages opened as files.
  if ("serviceWorker" in navigator && location.protocol.startsWith("http")) {
    navigator.serviceWorker.register("sw.js").catch((e) => console.warn("No offline support:", e));
  }

  render();
})();
@@file logic.test.js
// Run: node --test     (needs Node.js 20 or newer)
const test = require("node:test");
const assert = require("node:assert/strict");
const L = require("./logic.js");

const T = "2026-11-01";

test("day keys use the local calendar", () => {
  assert.equal(L.dayKey(new Date(2026, 0, 5, 23, 59)), "2026-01-05");
  assert.equal(L.addDays("2026-03-01", -1), "2026-02-28");
  assert.equal(L.addDays("2024-02-28", 1), "2024-02-29");
  assert.equal(L.addDays("2026-12-31", 1), "2027-01-01");
});

test("adding habits", () => {
  let s = L.addHabit(L.emptyState(), "  Stretch ");
  s = L.addHabit(s, "Read 20 pages");
  assert.deepEqual(s.habits.map((h) => [h.id, h.name]), [[1, "Stretch"], [2, "Read 20 pages"]]);
  assert.equal(s.nextId, 3);
  assert.throws(() => L.addHabit(s, "   "), /Give the habit a name/);
  assert.throws(() => L.addHabit(s, "stretch"), /You already have "stretch"/);
});

test("adding does not change the old state", () => {
  const before = L.emptyState();
  L.addHabit(before, "Walk");
  assert.equal(before.habits.length, 0);
});

test("toggling days", () => {
  let s = L.addHabit(L.addHabit(L.emptyState(), "A"), "B");
  const original = s;
  s = L.toggleDay(s, 1, "2026-10-31");
  s = L.toggleDay(s, 1, "2026-10-29");
  s = L.toggleDay(s, 1, "2026-10-30");
  assert.deepEqual(s.habits[0].done, ["2026-10-29", "2026-10-30", "2026-10-31"], "kept sorted");
  s = L.toggleDay(s, 1, "2026-10-30");
  assert.deepEqual(s.habits[0].done, ["2026-10-29", "2026-10-31"], "toggled off");
  assert.deepEqual(s.habits[1].done, [], "other habits untouched");
  assert.deepEqual(original.habits[0].done, [], "old state untouched");
  assert.deepEqual(L.toggleDay(s, 99, T), s, "unknown id changes nothing");
});

test("the last n days", () => {
  assert.deepEqual(L.lastNDays(T, 3), ["2026-10-30", "2026-10-31", "2026-11-01"]);
  assert.deepEqual(L.lastNDays(T, 1), [T]);
  assert.equal(L.lastNDays(T, 7).length, 7);
  assert.equal(L.lastNDays("2026-03-02", 3)[0], "2026-02-28");
});

test("streaks", () => {
  const h = (done) => ({ id: 1, name: "x", done });
  assert.equal(L.streak(h([]), T), 0);
  assert.equal(L.streak(h(["2026-10-30", "2026-10-31", T]), T), 3, "including today");
  assert.equal(L.streak(h(["2026-10-30", "2026-10-31"]), T), 2, "today not done yet");
  assert.equal(L.streak(h(["2026-10-29", "2026-10-31", T]), T), 2, "a gap ends it");
  assert.equal(L.streak(h(["2026-10-29", "2026-10-30"]), T), 0, "missed yesterday");
  assert.equal(L.streak(h(["2026-02-27", "2026-02-28", "2026-03-01"]), "2026-03-01"), 3, "across a month end");
});

test("completion rate", () => {
  const h = { id: 1, name: "x", done: ["2026-10-20", "2026-10-28", "2026-10-30", T] };
  assert.equal(L.completionRate(h, T, 7), 43);
  assert.equal(L.completionRate(h, T, 1), 100);
  assert.equal(L.completionRate(h, T, 3), 67);
  assert.equal(L.completionRate({ ...h, done: [] }, T, 30), 0);
});

test("loading saved data", () => {
  assert.deepEqual(L.load(null), L.emptyState());
  let s = L.addHabit(L.emptyState(), "Walk");
  s = L.toggleDay(s, 1, T);
  assert.deepEqual(L.load(L.save(s)), s, "round trip");
  const v1 = JSON.stringify({ habits: [{ name: "Walk", days: { "2026-10-31": true, "2026-10-30": true, "2026-10-29": false } }] });
  assert.deepEqual(L.load(v1), { version: 2, nextId: 2, habits: [{ id: 1, name: "Walk", done: ["2026-10-30", "2026-10-31"] }] });
  for (const bad of ["{oops", "[]", '{"habits": 3}', '{"version": 9, "habits": []}',
    '{"version": 2, "nextId": 2, "habits": [{"id": 1, "name": "x", "done": ["yesterday"]}]}']) {
    assert.throws(() => L.load(bad), /Not a habit file/, bad);
  }
});
@@file package.json
{
  "name": "habit-tracker",
  "private": true,
  "type": "commonjs",
  "scripts": {
    "test": "node --test"
  }
}
@@file sw.js
// sw.js: the service worker. It keeps a copy of the app's files, so the app opens offline.
// Change VERSION whenever you change a file, so browsers fetch the new copies.
const VERSION = "habits-v1";
const FILES = ["./", "index.html", "style.css", "logic.js", "app.js", "manifest.webmanifest", "icon.svg"];

self.addEventListener("install", (event) => {
  event.waitUntil(caches.open(VERSION).then((cache) => cache.addAll(FILES)));
});

// Delete the caches of older versions.
self.addEventListener("activate", (event) => {
  event.waitUntil(
    caches.keys().then((keys) => Promise.all(keys.filter((k) => k !== VERSION).map((k) => caches.delete(k)))),
  );
});

// Network first, so you always get the newest files when online; the cache when offline.
self.addEventListener("fetch", (event) => {
  if (event.request.method !== "GET") return;
  event.respondWith(
    fetch(event.request)
      .then((response) => {
        const copy = response.clone();
        caches.open(VERSION).then((cache) => cache.put(event.request, copy));
        return response;
      })
      .catch(() => caches.match(event.request)),
  );
});
@@file manifest.webmanifest
{
  "name": "Habits",
  "short_name": "Habits",
  "description": "A small daily habit tracker that works offline.",
  "start_url": "./",
  "display": "standalone",
  "background_color": "#f6f7f5",
  "theme_color": "#2f6f5e",
  "icons": [{ "src": "icon.svg", "sizes": "any", "type": "image/svg+xml", "purpose": "any" }]
}
@@file icon.svg
<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 64 64">
  <rect width="64" height="64" rx="14" fill="#2f6f5e"/>
  <path d="M18 33l9 9 19-21" fill="none" stroke="#fff" stroke-width="7" stroke-linecap="round" stroke-linejoin="round"/>
</svg>
@@text
## Stretch goals

- **Weekly goals**: "3 times a week" habits, whose streak counts weeks instead of days.
- **A heat map** of the last year (like GitHub's contributions graph) with CSS grid, one small square per day.
- **Reminders**: the Notifications API (`Notification.requestPermission()`), shown when the page is open in the evening and today isn't done.
- **Sync between devices**: a tiny API (the Link Vault or Team Task Board projects show how to build one) and `fetch` to save to it, with the local copy still working offline.
- **Undo**: keep the previous states in an array. Since every change returns a new state, undo is just "go back one".
- **Rewrite the page with a framework** (React, Vue or Svelte) and keep `logic.js` unchanged. Notice how little of the work was in the page code.
@@project habits_finished Habit Tracker: the finished version
A complete solution to compare with yours.
@@file index.html
<!doctype html>
<html lang="en">
<head>
  <meta charset="utf-8">
  <meta name="viewport" content="width=device-width, initial-scale=1">
  <meta http-equiv="Content-Security-Policy" content="default-src 'self'; img-src 'self' data:">
  <meta name="color-scheme" content="light dark">
  <meta name="theme-color" content="#2f6f5e">
  <title>Habits</title>
  <link rel="manifest" href="manifest.webmanifest">
  <link rel="icon" href="icon.svg" type="image/svg+xml">
  <link rel="stylesheet" href="style.css">
</head>
<body>
  <header class="top">
    <h1>Habits</h1>
    <p id="today" class="muted"></p>
  </header>

  <main>
    <form id="add-form" class="add" novalidate>
      <label for="habit-name" class="visually-hidden">New habit</label>
      <input id="habit-name" name="name" type="text" placeholder="A new habit, e.g. Stretch for 5 minutes"
             maxlength="60" autocomplete="off" required>
      <button type="submit">Add</button>
      <p id="form-error" class="error" role="alert" hidden></p>
    </form>

    <section aria-labelledby="list-title">
      <h2 id="list-title" class="visually-hidden">Your habits</h2>
      <div id="days-header" class="days-header" aria-hidden="true"></div>
      <ul id="habits" class="habits"></ul>
      <p id="empty" class="muted empty" hidden>No habits yet. Add one above. Small ones work best.</p>
    </section>

    <section class="data" aria-labelledby="data-title">
      <h2 id="data-title">Your data</h2>
      <p class="muted">Everything stays in this browser. Export a file to back it up or move it to another device.</p>
      <div class="buttons">
        <button type="button" id="export">Export</button>
        <label class="button" for="import">Import…</label>
        <input type="file" id="import" accept="application/json,.json" class="visually-hidden">
      </div>
    </section>
  </main>

  <p id="status" class="visually-hidden" aria-live="polite"></p>

  <template id="habit-template">
    <li class="habit">
      <div class="habit-head">
        <span class="name"></span>
        <span class="stats muted"></span>
        <button type="button" class="remove" data-action="remove">Remove</button>
      </div>
      <div class="days"></div>
    </li>
  </template>

  <script src="logic.js"></script>
  <script src="app.js"></script>
</body>
</html>
@@file style.css
:root {
  --bg: #f6f7f5;
  --card: #ffffff;
  --text: #1d2420;
  --muted: #5d6a63;
  --line: #d9dfdb;
  --accent: #2f6f5e;
  --accent-text: #ffffff;
  --error: #b3261e;
  --radius: 10px;
  font-family: system-ui, -apple-system, "Segoe UI", Roboto, sans-serif;
  color: var(--text);
  background: var(--bg);
}

@media (prefers-color-scheme: dark) {
  :root {
    --bg: #121614;
    --card: #1b211e;
    --text: #e4ebe7;
    --muted: #9aa8a0;
    --line: #2e3833;
    --accent: #6cc3a8;
    --accent-text: #0d1a15;
    --error: #ff8a80;
  }
}

* { box-sizing: border-box; }
body { margin: 0 auto; max-width: 720px; padding: 24px 16px 48px; line-height: 1.5; }
h1 { margin: 0; font-size: 1.8rem; }
h2 { font-size: 1.1rem; }
.muted { color: var(--muted); }
.top { margin-bottom: 20px; }
.top p { margin: 4px 0 0; }

.visually-hidden {
  position: absolute; width: 1px; height: 1px; overflow: hidden;
  clip: rect(0 0 0 0); white-space: nowrap;
}

button, .button {
  font: inherit; cursor: pointer; border-radius: var(--radius);
  border: 1px solid var(--line); background: var(--card); color: var(--text);
  padding: 8px 14px; display: inline-block;
}
button:hover, .button:hover { border-color: var(--accent); }
:focus-visible { outline: 3px solid var(--accent); outline-offset: 2px; }

.add { display: flex; flex-wrap: wrap; gap: 8px; margin-bottom: 24px; }
.add input {
  flex: 1 1 240px; font: inherit; padding: 8px 12px; border-radius: var(--radius);
  border: 1px solid var(--line); background: var(--card); color: var(--text);
}
.add input[aria-invalid="true"] { border-color: var(--error); }
.add button[type="submit"] { background: var(--accent); color: var(--accent-text); border-color: var(--accent); }
.error { color: var(--error); margin: 0; flex-basis: 100%; }

.days-header, .days { display: grid; grid-template-columns: repeat(7, 1fr); gap: 6px; }
.days-header { padding: 0 14px; font-size: 0.8rem; color: var(--muted); text-align: center; }
.habits { list-style: none; margin: 6px 0 0; padding: 0; display: grid; gap: 10px; }
.habit { background: var(--card); border: 1px solid var(--line); border-radius: var(--radius); padding: 12px 14px; }
.habit-head { display: flex; flex-wrap: wrap; align-items: baseline; gap: 4px 12px; margin-bottom: 10px; }
.name { font-weight: 600; }
.stats { font-size: 0.85rem; flex: 1; }
.remove { padding: 2px 10px; font-size: 0.85rem; }

.day { aspect-ratio: 1; padding: 0; font-size: 1.2rem; min-height: 36px; }
.day[aria-pressed="true"] { background: var(--accent); color: var(--accent-text); border-color: var(--accent); }
.day.is-today { border-width: 2px; border-color: var(--accent); }
.empty { text-align: center; padding: 24px; }

.data { margin-top: 40px; border-top: 1px solid var(--line); padding-top: 8px; }
.buttons { display: flex; gap: 8px; }

@media (prefers-reduced-motion: no-preference) {
  .day { transition: background-color 120ms ease; }
}
@@file logic.js
// logic.js: everything the app knows about habits, with no DOM and no storage.
// The page uses it as window.HabitLogic; Node's test runner loads it with require().
// Every function returns a new state instead of changing the old one, so render() can
// trust that the state it was given never changes behind its back.
(function (root) {
  "use strict";

  const VERSION = 2;

  // The local calendar day of a Date, as "YYYY-MM-DD" (not toISOString(), which is UTC).
  function dayKey(date) {
    const y = date.getFullYear();
    const m = String(date.getMonth() + 1).padStart(2, "0");
    const d = String(date.getDate()).padStart(2, "0");
    return `${y}-${m}-${d}`;
  }

  // "YYYY-MM-DD" -> a Date at local midnight.
  function parseDay(key) {
    const [y, m, d] = key.split("-").map(Number);
    return new Date(y, m - 1, d);
  }

  // The day key `n` days after `key` (n may be negative). Uses the calendar, not 24-hour
  // steps, so a daylight-saving change can't skip or repeat a day.
  function addDays(key, n) {
    const date = parseDay(key);
    date.setDate(date.getDate() + n);
    return dayKey(date);
  }

  function emptyState() {
    return { version: VERSION, nextId: 1, habits: [] };
  }

  // Adds a habit called `name` (trimmed). Throws Error("Give the habit a name") for an empty
  // name and Error(`You already have "<name>"`) when one with the same name exists, ignoring case.
  function addHabit(state, name) {
    const clean = String(name).trim();
    if (!clean) throw new Error("Give the habit a name");
    if (state.habits.some((h) => h.name.toLowerCase() === clean.toLowerCase())) {
      throw new Error(`You already have "${clean}"`);
    }
    const habit = { id: state.nextId, name: clean, done: [] };
    return { ...state, nextId: state.nextId + 1, habits: [...state.habits, habit] };
  }

  function removeHabit(state, id) {
    return { ...state, habits: state.habits.filter((h) => h.id !== id) };
  }

  // Marks `day` done for habit `id`, or un-marks it if it was done. `done` stays sorted
  // (oldest first) and never has duplicates. Other habits are untouched. Unknown ids change nothing.
  function toggleDay(state, id, day) {
    return {
      ...state,
      habits: state.habits.map((h) => {
        if (h.id !== id) return h;
        const done = h.done.includes(day) ? h.done.filter((d) => d !== day) : [...h.done, day].sort();
        return { ...h, done };
      }),
    };
  }

  // The `n` day keys ending with `today`, oldest first: lastNDays("2026-11-01", 3)
  // is ["2026-10-30", "2026-10-31", "2026-11-01"].
  function lastNDays(today, n) {
    const days = [];
    for (let i = n - 1; i >= 0; i--) days.push(addDays(today, -i));
    return days;
  }

  // How many days in a row the habit has been done, counting back from today. If today
  // isn't done yet, the streak still counts up to yesterday (the day isn't over).
  function streak(habit, today) {
    const done = new Set(habit.done);
    let day = done.has(today) ? today : addDays(today, -1);
    let count = 0;
    while (done.has(day)) {
      count++;
      day = addDays(day, -1);
    }
    return count;
  }

  // The percentage (0-100, rounded) of the last `days` days (ending today) that are done.
  function completionRate(habit, today, days) {
    if (days <= 0) return 0;
    const done = new Set(habit.done);
    const hits = lastNDays(today, days).filter((d) => done.has(d)).length;
    return Math.round((hits / days) * 100);
  }

  // Reads saved text (from localStorage or an imported file) into a state.
  // null or "" -> an empty state. Version 1 stored { habits: [{ name, days: {"YYYY-MM-DD": true} }] }
  // without ids; it's upgraded. Anything else that doesn't look right throws Error("Not a habit file").
  function load(text) {
    if (!text) return emptyState();
    let data;
    try {
      data = JSON.parse(text);
    } catch {
      throw new Error("Not a habit file");
    }
    if (!data || !Array.isArray(data.habits)) throw new Error("Not a habit file");
    if (data.version === undefined || data.version === 1) {
      const habits = data.habits.map((h, i) => ({
        id: i + 1,
        name: String(h.name),
        done: Object.keys(h.days || {}).filter((d) => h.days[d]).sort(),
      }));
      return { version: VERSION, nextId: habits.length + 1, habits };
    }
    if (data.version !== VERSION) throw new Error("Not a habit file");
    const valid = data.habits.every(
      (h) => Number.isInteger(h.id) && typeof h.name === "string" && Array.isArray(h.done) &&
        h.done.every((d) => /^\d{4}-\d{2}-\d{2}$/.test(d)),
    );
    if (!valid || !Number.isInteger(data.nextId)) throw new Error("Not a habit file");
    return data;
  }

  function save(state) {
    return JSON.stringify(state);
  }

  const api = { VERSION, dayKey, parseDay, addDays, emptyState, addHabit, removeHabit, toggleDay,
    lastNDays, streak, completionRate, load, save };
  if (typeof module === "object" && module.exports) module.exports = api;
  else root.HabitLogic = api;
})(globalThis);
@@file app.js
// app.js: the page. It owns the state, saves it, and draws it. All the rules are in logic.js.
(function () {
  "use strict";
  const L = window.HabitLogic;
  const KEY = "habits";
  const DAYS_SHOWN = 7;

  const $ = (id) => document.getElementById(id);
  const list = $("habits");
  const template = $("habit-template");

  let state = readSaved();

  function readSaved() {
    try {
      return L.load(localStorage.getItem(KEY));
    } catch (e) {
      // Storage blocked (private mode) or damaged data: start empty rather than crash.
      console.warn("Could not load saved habits:", e);
      return L.emptyState();
    }
  }

  // The only way the state changes: compute a new one, save it, draw it.
  function update(next, message) {
    state = next;
    try {
      localStorage.setItem(KEY, L.save(state));
    } catch {
      announce("Couldn't save: your browser is blocking storage.");
    }
    render();
    if (message) announce(message);
  }

  function announce(text) {
    $("status").textContent = text;   // read out by screen readers (aria-live)
  }

  const weekday = new Intl.DateTimeFormat(undefined, { weekday: "short" });
  const longDate = new Intl.DateTimeFormat(undefined, { weekday: "long", month: "long", day: "numeric" });

  function render() {
    const today = L.dayKey(new Date());
    const days = L.lastNDays(today, DAYS_SHOWN);
    $("today").textContent = longDate.format(new Date());

    $("days-header").replaceChildren(
      ...days.map((d) => {
        const span = document.createElement("span");
        span.textContent = weekday.format(L.parseDay(d));
        return span;
      }),
    );

    // Rebuilding the list is simple and fast for a few dozen items. textContent (never
    // innerHTML) puts the user's text in the page, so a habit called "<img onerror=...>" is just text.
    list.replaceChildren(
      ...state.habits.map((habit) => {
        const item = template.content.firstElementChild.cloneNode(true);
        item.dataset.id = habit.id;
        item.querySelector(".name").textContent = habit.name;
        const s = L.streak(habit, today);
        item.querySelector(".stats").textContent =
          `${s} day${s === 1 ? "" : "s"} in a row · ${L.completionRate(habit, today, 30)}% of the last 30 days`;
        item.querySelector(".remove").setAttribute("aria-label", `Remove ${habit.name}`);
        const done = new Set(habit.done);
        item.querySelector(".days").replaceChildren(
          ...days.map((d) => {
            const b = document.createElement("button");
            b.type = "button";
            b.className = "day" + (d === today ? " is-today" : "");
            b.dataset.action = "toggle";
            b.dataset.day = d;
            b.setAttribute("aria-pressed", String(done.has(d)));
            b.setAttribute("aria-label", `${habit.name}, ${longDate.format(L.parseDay(d))}`);
            b.textContent = done.has(d) ? "✓" : "";
            return b;
          }),
        );
        return item;
      }),
    );
    $("empty").hidden = state.habits.length > 0;
  }

  // One listener for the whole list (event delegation): it works for items added later too.
  list.addEventListener("click", (event) => {
    const button = event.target.closest("button[data-action]");
    if (!button) return;
    const id = Number(button.closest(".habit").dataset.id);
    const habit = state.habits.find((h) => h.id === id);
    if (button.dataset.action === "toggle") {
      const day = button.dataset.day;
      update(L.toggleDay(state, id, day));
      // render() replaced the button, so put the keyboard focus back on the new one.
      list.querySelector(`.habit[data-id="${id}"] [data-day="${day}"]`)?.focus();
    } else if (button.dataset.action === "remove" && confirm(`Remove "${habit.name}" and its history?`)) {
      update(L.removeHabit(state, id), `Removed ${habit.name}`);
      $("habit-name").focus();
    }
  });

  $("add-form").addEventListener("submit", (event) => {
    event.preventDefault();
    const input = $("habit-name");
    const error = $("form-error");
    try {
      update(L.addHabit(state, input.value), `Added ${input.value.trim()}`);
      input.value = "";
      error.hidden = true;
      input.removeAttribute("aria-invalid");
    } catch (e) {
      error.textContent = e.message;
      error.hidden = false;
      input.setAttribute("aria-invalid", "true");
    }
    input.focus();
  });

  $("export").addEventListener("click", () => {
    const blob = new Blob([L.save(state)], { type: "application/json" });
    const link = document.createElement("a");
    link.href = URL.createObjectURL(blob);
    link.download = `habits-${L.dayKey(new Date())}.json`;
    link.click();
    setTimeout(() => URL.revokeObjectURL(link.href), 1000);
  });

  $("import").addEventListener("change", async (event) => {
    const file = event.target.files[0];
    event.target.value = "";             // so choosing the same file again still fires "change"
    if (!file) return;
    try {
      const imported = L.load(await file.text());
      if (confirm(`Replace your ${state.habits.length} habit(s) with the ${imported.habits.length} in ${file.name}?`)) {
        update(imported, `Imported ${imported.habits.length} habit(s)`);
      }
    } catch (e) {
      alert(`${file.name}: ${e.message}`);
    }
  });

  // Another tab changed the data: show it here too.
  window.addEventListener("storage", (event) => {
    if (event.key === KEY) {
      state = readSaved();
      render();
    }
  });

  // A new day may have started while the page was open in the background.
  document.addEventListener("visibilitychange", () => {
    if (!document.hidden) render();
  });

  // Offline support. Service workers only run on http(s), not on pages opened as files.
  if ("serviceWorker" in navigator && location.protocol.startsWith("http")) {
    navigator.serviceWorker.register("sw.js").catch((e) => console.warn("No offline support:", e));
  }

  render();
})();
@@file logic.test.js
// Run: node --test     (needs Node.js 20 or newer)
const test = require("node:test");
const assert = require("node:assert/strict");
const L = require("./logic.js");

const T = "2026-11-01";

test("day keys use the local calendar", () => {
  assert.equal(L.dayKey(new Date(2026, 0, 5, 23, 59)), "2026-01-05");
  assert.equal(L.addDays("2026-03-01", -1), "2026-02-28");
  assert.equal(L.addDays("2024-02-28", 1), "2024-02-29");
  assert.equal(L.addDays("2026-12-31", 1), "2027-01-01");
});

test("adding habits", () => {
  let s = L.addHabit(L.emptyState(), "  Stretch ");
  s = L.addHabit(s, "Read 20 pages");
  assert.deepEqual(s.habits.map((h) => [h.id, h.name]), [[1, "Stretch"], [2, "Read 20 pages"]]);
  assert.equal(s.nextId, 3);
  assert.throws(() => L.addHabit(s, "   "), /Give the habit a name/);
  assert.throws(() => L.addHabit(s, "stretch"), /You already have "stretch"/);
});

test("adding does not change the old state", () => {
  const before = L.emptyState();
  L.addHabit(before, "Walk");
  assert.equal(before.habits.length, 0);
});

test("toggling days", () => {
  let s = L.addHabit(L.addHabit(L.emptyState(), "A"), "B");
  const original = s;
  s = L.toggleDay(s, 1, "2026-10-31");
  s = L.toggleDay(s, 1, "2026-10-29");
  s = L.toggleDay(s, 1, "2026-10-30");
  assert.deepEqual(s.habits[0].done, ["2026-10-29", "2026-10-30", "2026-10-31"], "kept sorted");
  s = L.toggleDay(s, 1, "2026-10-30");
  assert.deepEqual(s.habits[0].done, ["2026-10-29", "2026-10-31"], "toggled off");
  assert.deepEqual(s.habits[1].done, [], "other habits untouched");
  assert.deepEqual(original.habits[0].done, [], "old state untouched");
  assert.deepEqual(L.toggleDay(s, 99, T), s, "unknown id changes nothing");
});

test("the last n days", () => {
  assert.deepEqual(L.lastNDays(T, 3), ["2026-10-30", "2026-10-31", "2026-11-01"]);
  assert.deepEqual(L.lastNDays(T, 1), [T]);
  assert.equal(L.lastNDays(T, 7).length, 7);
  assert.equal(L.lastNDays("2026-03-02", 3)[0], "2026-02-28");
});

test("streaks", () => {
  const h = (done) => ({ id: 1, name: "x", done });
  assert.equal(L.streak(h([]), T), 0);
  assert.equal(L.streak(h(["2026-10-30", "2026-10-31", T]), T), 3, "including today");
  assert.equal(L.streak(h(["2026-10-30", "2026-10-31"]), T), 2, "today not done yet");
  assert.equal(L.streak(h(["2026-10-29", "2026-10-31", T]), T), 2, "a gap ends it");
  assert.equal(L.streak(h(["2026-10-29", "2026-10-30"]), T), 0, "missed yesterday");
  assert.equal(L.streak(h(["2026-02-27", "2026-02-28", "2026-03-01"]), "2026-03-01"), 3, "across a month end");
});

test("completion rate", () => {
  const h = { id: 1, name: "x", done: ["2026-10-20", "2026-10-28", "2026-10-30", T] };
  assert.equal(L.completionRate(h, T, 7), 43);
  assert.equal(L.completionRate(h, T, 1), 100);
  assert.equal(L.completionRate(h, T, 3), 67);
  assert.equal(L.completionRate({ ...h, done: [] }, T, 30), 0);
});

test("loading saved data", () => {
  assert.deepEqual(L.load(null), L.emptyState());
  let s = L.addHabit(L.emptyState(), "Walk");
  s = L.toggleDay(s, 1, T);
  assert.deepEqual(L.load(L.save(s)), s, "round trip");
  const v1 = JSON.stringify({ habits: [{ name: "Walk", days: { "2026-10-31": true, "2026-10-30": true, "2026-10-29": false } }] });
  assert.deepEqual(L.load(v1), { version: 2, nextId: 2, habits: [{ id: 1, name: "Walk", done: ["2026-10-30", "2026-10-31"] }] });
  for (const bad of ["{oops", "[]", '{"habits": 3}', '{"version": 9, "habits": []}',
    '{"version": 2, "nextId": 2, "habits": [{"id": 1, "name": "x", "done": ["yesterday"]}]}']) {
    assert.throws(() => L.load(bad), /Not a habit file/, bad);
  }
});
@@file package.json
{
  "name": "habit-tracker",
  "private": true,
  "type": "commonjs",
  "scripts": {
    "test": "node --test"
  }
}
@@file sw.js
// sw.js: the service worker. It keeps a copy of the app's files, so the app opens offline.
// Change VERSION whenever you change a file, so browsers fetch the new copies.
const VERSION = "habits-v1";
const FILES = ["./", "index.html", "style.css", "logic.js", "app.js", "manifest.webmanifest", "icon.svg"];

self.addEventListener("install", (event) => {
  event.waitUntil(caches.open(VERSION).then((cache) => cache.addAll(FILES)));
});

// Delete the caches of older versions.
self.addEventListener("activate", (event) => {
  event.waitUntil(
    caches.keys().then((keys) => Promise.all(keys.filter((k) => k !== VERSION).map((k) => caches.delete(k)))),
  );
});

// Network first, so you always get the newest files when online; the cache when offline.
self.addEventListener("fetch", (event) => {
  if (event.request.method !== "GET") return;
  event.respondWith(
    fetch(event.request)
      .then((response) => {
        const copy = response.clone();
        caches.open(VERSION).then((cache) => cache.put(event.request, copy));
        return response;
      })
      .catch(() => caches.match(event.request)),
  );
});
@@file manifest.webmanifest
{
  "name": "Habits",
  "short_name": "Habits",
  "description": "A small daily habit tracker that works offline.",
  "start_url": "./",
  "display": "standalone",
  "background_color": "#f6f7f5",
  "theme_color": "#2f6f5e",
  "icons": [{ "src": "icon.svg", "sizes": "any", "type": "image/svg+xml", "purpose": "any" }]
}
@@file icon.svg
<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 64 64">
  <rect width="64" height="64" rx="14" fill="#2f6f5e"/>
  <path d="M18 33l9 9 19-21" fill="none" stroke="#fff" stroke-width="7" stroke-linecap="round" stroke-linejoin="round"/>
</svg>

@@drill
Which browser feature lets a web app work offline by intercepting requests?
= service worker
= a service worker
= service workers
? It serves cached files when there's no network.

@@text
🎉 You've built an installable, offline, accessible web app with tested logic and no dependencies at all. Every framework is a more convenient way of doing what you just did by hand: state, render, events.
