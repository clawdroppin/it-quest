OUT = "content/gui/gu9_01_plant_buddy.md"
BASE = "tools/appdev/gui_plant_buddy"
FILES = ["index.html", "style.css", "plants.js", "app.js", "plants.test.js", "package.json", "app-icon.svg"]

PARTS = [
    ("text", r'''
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
'''),
    ("project", "plant_buddy", "Plant Buddy",
     "Write the four functions that throw `TODO` in `plants.js` until `node --test` passes, then open `index.html`.",
     BASE + "/starter", FILES),
    ("text", r'''
@@text
## Make it yours

These are design tasks as much as code tasks:

- **Draw two new icons** for the sprite, on the same 24-unit grid with the same 2-unit stroke: a sun for a "needs light" note and a calendar for a "last watered" line. Check them next to the others at 20 px.
- **A "soon" animation**: a gentle wiggle on the emoji of plants that need water tomorrow, so they ask politely before they get thirsty.
- **An undo toast**: "Said goodbye to Spike. *Undo*". Keep the removed plant for 5 seconds and put it back if the button is pressed.
- **A third theme**: "sakura" pinks or "ocean" blues. With tokens it's only new values for the same names.
- **Ship it as a desktop app**: wrap the folder with Tauri (`npm create tauri-app`, then copy the files into the front end), and turn `app-icon.svg` into every icon size with `npx tauri icon`.
- **A tkinter or WPF version**: reuse the same rules (port `plants.js` to Python or C#), and recreate the look with `ttk.Style` or a WPF `ResourceDictionary`. The logic tests port almost line for line.
'''),
    ("project", "plant_buddy_finished", "Plant Buddy: the finished version",
     "A complete solution to compare with yours.",
     BASE + "/finished", FILES),
    ("text", r'''
@@text
🌱 Tokens, themes, real controls, feedback, motion with taste, and icons drawn by hand: that's what makes the difference between an app that works and an app people *like*. Every GUI you build from now on, in any language, can be made like this.
'''),
]
