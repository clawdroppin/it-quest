OUT = "content/html/i2_01_habits.md"
BASE = "tools/appdev/html_habits"
FILES = ["index.html", "style.css", "logic.js", "app.js", "logic.test.js", "package.json",
         "sw.js", "manifest.webmanifest", "icon.svg"]

PARTS = [
    ("text", r'''
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
'''),
    ("project", "habits", "Habit Tracker",
     "Write the four functions that throw `TODO` in `logic.js` until `node --test` passes, then open `index.html`.",
     BASE + "/starter", FILES),
    ("text", r'''
@@text
## Stretch goals

- **Weekly goals**: "3 times a week" habits, whose streak counts weeks instead of days.
- **A heat map** of the last year (like GitHub's contributions graph) with CSS grid, one small square per day.
- **Reminders**: the Notifications API (`Notification.requestPermission()`), shown when the page is open in the evening and today isn't done.
- **Sync between devices**: a tiny API (the Link Vault or Team Task Board projects show how to build one) and `fetch` to save to it, with the local copy still working offline.
- **Undo**: keep the previous states in an array. Since every change returns a new state, undo is just "go back one".
- **Rewrite the page with a framework** (React, Vue or Svelte) and keep `logic.js` unchanged. Notice how little of the work was in the page code.
'''),
    ("project", "habits_finished", "Habit Tracker: the finished version",
     "A complete solution to compare with yours.",
     BASE + "/finished", FILES),
    ("text", r'''
@@text
🎉 You've built an installable, offline, accessible web app with tested logic and no dependencies at all. Every framework is a more convenient way of doing what you just did by hand: state, render, events.
'''),
]
