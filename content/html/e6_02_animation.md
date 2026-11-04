@@lesson e6-animation Transitions, Transforms and Keyframe Animations
@@text
Motion helps people understand what changed: a panel slides in from where it came from, a button reacts to a press, a spinner says "working". Done badly, motion is distracting, slow, or makes some people physically ill. This lesson covers both the techniques and the restraint.

## Transforms

`transform` moves, scales, rotates or skews an element **visually**, without affecting layout around it:

```css
transform: translateY(-4px);          /* move up 4px */
transform: scale(1.05);               /* 5% bigger */
transform: rotate(45deg);
transform: translate(-50%, -50%) rotate(10deg);   /* combined, applied right to left */
```

Individual properties exist too: `translate: 0 -4px; scale: 1.05; rotate: 45deg;`, which can be animated separately.

## Transitions

A transition animates a property **between two states** (hover, a class added by JavaScript, `:checked`):

```html
<style>
  .btn { padding: 0.6rem 1.2rem; border: 0; border-radius: 8px; background: #2563eb; color: white;
         transition: transform 150ms ease-out, box-shadow 150ms ease-out, background-color 200ms; }
  .btn:hover { transform: translateY(-2px); box-shadow: 0 6px 16px rgb(37 99 235 / 0.35); background: #1d4ed8; }
  .btn:active { transform: translateY(0); box-shadow: none; }
</style>
<button class="btn">Hover me</button>
```

`transition: property duration timing-function delay`. List properties explicitly; `transition: all` animates things you didn't mean to (and costs performance).

## Timing

- **Durations**: 100–200 ms for small feedback (hover, press), 200–400 ms for panels and dialogs. Longer feels sluggish.
- **Easing**: `ease-out` (fast start, gentle stop) for things entering; `ease-in` for leaving; `linear` only for continuous motion like spinners. `cubic-bezier(.2, .8, .2, 1)` for a custom curve; `linear()` can even approximate springs.

## Keyframe animations

For motion that isn't just A → B, or that runs by itself:

```html
<style>
  @keyframes spin { to { rotate: 360deg; } }
  @keyframes pulse { 0%, 100% { opacity: 1; } 50% { opacity: 0.4; } }
  .spinner { width: 1.5rem; height: 1.5rem; border: 3px solid #cbd5e1; border-top-color: #2563eb; border-radius: 50%;
             animation: spin 0.8s linear infinite; }
  .live { color: #15803d; animation: pulse 2s ease-in-out infinite; }
</style>
<div class="spinner" role="status" aria-label="Loading"></div>
<p class="live">● Live</p>
```

`animation: name duration timing iteration-count direction fill-mode`. Useful values: `infinite`, `alternate` (back and forth), `forwards` (keep the final state), `animation-delay` with negative values to start mid-animation.

## Performance: animate transform and opacity

Browsers draw a page in steps: **layout** (sizes and positions), **paint** (pixels), **composite** (layers). Animating `width`, `height`, `top` or `margin` forces layout on every frame, which stutters. Animating `transform` and `opacity` only needs compositing, often on the GPU, so it stays at 60 fps. Slide with `translate`, not `left`; grow with `scale`, not `width`.

`will-change: transform` hints the browser to prepare a layer, but use it sparingly (each layer costs memory).

## Animating to `height: auto`

Height transitions to `auto` used to be impossible. Modern options: `interpolate-size: allow-keywords` (Chromium), animating `grid-template-rows` from `0fr` to `1fr` on a wrapper (works everywhere), or animating `transform: scaleY()` for simple cases.

## Respect reduced motion

Some people get dizzy or nauseous from motion (vestibular disorders), and they turn on "reduce motion" in their OS. Honour it:

```css
@media (prefers-reduced-motion: reduce) {
  *, *::before, *::after {
    animation-duration: 0.01ms !important;
    animation-iteration-count: 1 !important;
    transition-duration: 0.01ms !important;
    scroll-behavior: auto !important;
  }
}
```

Or the other way round: only add motion inside `@media (prefers-reduced-motion: no-preference)`. Essential motion (a progress bar) can stay; decorative motion (parallax, bouncing) goes.

## View transitions

The View Transitions API animates between two states of a page (or two pages) automatically: `document.startViewTransition(() => updateTheDom())` cross-fades by default, and `view-transition-name` on elements makes them morph from their old position to their new one. Supported in Chromium and Safari; other browsers simply update without animation.

@@quiz
Which properties should you animate for smooth 60 fps motion?
- width and height
- top and left
* transform and opacity
- margin and padding
? They skip layout and paint, needing only compositing.

@@quiz
What's a sensible duration for a hover effect?
* 100–200 ms
- 1 second
- 2–3 seconds
- 10 ms
? Small feedback should be quick.

@@quiz
What should happen when `prefers-reduced-motion: reduce` is set?
- Nothing; it's only for games
* Remove or minimise decorative animation and transitions
- Make animations slower
- Disable all CSS
? Essential motion can stay; decoration goes.

@@quiz
Why avoid `transition: all`?
- It's invalid
* It animates properties you didn't intend and can hurt performance
- It only works on hover
- It disables keyframes
? List the properties you mean.

@@flashcards
`transform: translateY(-4px)` :: move visually, no layout change
`translate` / `scale` / `rotate` :: individual transform properties
`transition: transform 150ms ease-out` :: animate between states
`@keyframes name { ... }` :: define an animation
`animation: spin 1s linear infinite` :: run it forever
`ease-out` :: for things entering
`forwards` :: keep the last keyframe
composite-only properties :: transform, opacity
`will-change` :: prepare a layer (sparingly)
`prefers-reduced-motion` :: user wants less motion
`grid-template-rows: 0fr → 1fr` :: animate to auto height
`startViewTransition()` :: animate DOM changes

@@exercise Buttons that respond
Give the buttons responsive feedback: on hover they lift with `transform: translateY(-2px)` and a shadow, on `:active` they press back down, and on `:focus-visible` they show a 3px outline. Transition only `transform` and `box-shadow` (150 ms, ease-out). Remove all this motion for users who prefer reduced motion with a `prefers-reduced-motion: reduce` media query that sets `transition: none` and `transform: none` on hover.
@@starter
<!doctype html>
<html lang="en">
<head><meta charset="utf-8"><title>Buttons</title>
<style>
  .btn { padding: 0.6rem 1.2rem; border: 0; border-radius: 8px; background: #2563eb; color: white; font: inherit; }
</style>
</head>
<body>
  <button class="btn">Restart service</button>
  <button class="btn">View logs</button>
</body>
</html>
@@requires
translateY(-2px)
:active
:focus-visible
transition: transform 150ms ease-out, box-shadow 150ms ease-out
prefers-reduced-motion: reduce
transition: none
@@hint
`.btn { transition: transform 150ms ease-out, box-shadow 150ms ease-out; } .btn:hover { transform: translateY(-2px); box-shadow: 0 6px 16px rgb(37 99 235 / 0.35); } .btn:active { transform: translateY(0); box-shadow: none; } .btn:focus-visible { outline: 3px solid #f59e0b; outline-offset: 2px; } @media (prefers-reduced-motion: reduce) { .btn { transition: none; } .btn:hover { transform: none; } }`
@@solution
<!doctype html>
<html lang="en">
<head><meta charset="utf-8"><title>Buttons</title>
<style>
  .btn { padding: 0.6rem 1.2rem; border: 0; border-radius: 8px; background: #2563eb; color: white; font: inherit;
         transition: transform 150ms ease-out, box-shadow 150ms ease-out; }
  .btn:hover { transform: translateY(-2px); box-shadow: 0 6px 16px rgb(37 99 235 / 0.35); }
  .btn:active { transform: translateY(0); box-shadow: none; }
  .btn:focus-visible { outline: 3px solid #f59e0b; outline-offset: 2px; }
  @media (prefers-reduced-motion: reduce) {
    .btn { transition: none; }
    .btn:hover { transform: none; }
  }
</style>
</head>
<body>
  <button class="btn">Restart service</button>
  <button class="btn">View logs</button>
</body>
</html>

@@exercise A loading spinner and skeleton
Create a CSS-only spinner (`@keyframes spin` rotating to `360deg`, `0.8s linear infinite`) with `role="status"` and a visually hidden "Loading tickets" label, and two skeleton lines that shimmer: a `@keyframes shimmer` that moves a `linear-gradient` background with `background-position` from `-200% 0` to `200% 0` over 1.5 s. Turn both animations off under `prefers-reduced-motion: reduce`.
@@starter
<!doctype html>
<html lang="en">
<head><meta charset="utf-8"><title>Loading</title>
<style>
  .skeleton { height: 1rem; margin: 0.5rem 0; border-radius: 4px; background: #e2e8f0; }
</style>
</head>
<body>
  <div class="spinner"></div>
  <div class="skeleton" style="width: 80%"></div>
  <div class="skeleton" style="width: 60%"></div>
</body>
</html>
@@requires
@keyframes spin
360deg
infinite
role="status"
@keyframes shimmer
background-position
prefers-reduced-motion: reduce
animation: none
@@hint
Spinner: `.spinner { width: 2rem; height: 2rem; border: 4px solid #cbd5e1; border-top-color: #2563eb; border-radius: 50%; animation: spin 0.8s linear infinite; } @keyframes spin { to { rotate: 360deg; } }`. Skeleton: `background: linear-gradient(90deg, #e2e8f0 25%, #f8fafc 50%, #e2e8f0 75%); background-size: 200% 100%; animation: shimmer 1.5s linear infinite;` with `@keyframes shimmer { from { background-position: -200% 0; } to { background-position: 200% 0; } }`.
@@solution
<!doctype html>
<html lang="en">
<head><meta charset="utf-8"><title>Loading</title>
<style>
  .spinner { width: 2rem; height: 2rem; border: 4px solid #cbd5e1; border-top-color: #2563eb; border-radius: 50%;
             animation: spin 0.8s linear infinite; }
  @keyframes spin { to { rotate: 360deg; } }
  .skeleton { height: 1rem; margin: 0.5rem 0; border-radius: 4px;
              background: linear-gradient(90deg, #e2e8f0 25%, #f8fafc 50%, #e2e8f0 75%);
              background-size: 200% 100%; animation: shimmer 1.5s linear infinite; }
  @keyframes shimmer { from { background-position: -200% 0; } to { background-position: 200% 0; } }
  .visually-hidden { position: absolute; width: 1px; height: 1px; overflow: hidden; clip: rect(0 0 0 0); white-space: nowrap; }
  @media (prefers-reduced-motion: reduce) {
    .spinner, .skeleton { animation: none; }
  }
</style>
</head>
<body>
  <div class="spinner" role="status"><span class="visually-hidden">Loading tickets</span></div>
  <div class="skeleton" style="width: 80%"></div>
  <div class="skeleton" style="width: 60%"></div>
</body>
</html>

@@exercise An expanding panel without JavaScript height hacks
Animate the "Details" panel open and closed using the grid trick: the wrapper `.panel` is `display: grid` with `grid-template-rows: 0fr`, and `1fr` when the checkbox toggle is checked (`.toggle:checked + .panel`). Transition `grid-template-rows` over 250 ms. The inner element needs `overflow: hidden` (and `min-height: 0`) so the content is clipped while collapsed.
@@starter
<!doctype html>
<html lang="en">
<head><meta charset="utf-8"><title>Expand</title>
<style>
  .panel-inner { background: #f1f5f9; padding: 0 1rem; }
</style>
</head>
<body>
  <label><input type="checkbox" class="toggle"> Show details</label>
  <div class="panel">
    <div class="panel-inner">
      <p>Ticket #11 was opened at 22:40 after five failed logins from 203.0.113.7.</p>
      <p>The account was locked and the password reset. MFA was re-enrolled the next morning.</p>
    </div>
  </div>
</body>
</html>
@@requires
grid-template-rows: 0fr
grid-template-rows: 1fr
.toggle:checked + .panel
transition: grid-template-rows 250ms
overflow: hidden
@@hint
The checkbox must be the panel's previous sibling for `+` to work: move the input out of the label (`<input type="checkbox" class="toggle" id="t"> <label for="t">Show details</label>` would break `+`, so put the label **before** the input instead, or use `~`). `.panel { display: grid; grid-template-rows: 0fr; transition: grid-template-rows 250ms ease-out; } .toggle:checked + .panel { grid-template-rows: 1fr; } .panel-inner { overflow: hidden; min-height: 0; }`
@@solution
<!doctype html>
<html lang="en">
<head><meta charset="utf-8"><title>Expand</title>
<style>
  .panel { display: grid; grid-template-rows: 0fr; transition: grid-template-rows 250ms ease-out; }
  .toggle:checked + .panel { grid-template-rows: 1fr; }
  .panel-inner { overflow: hidden; min-height: 0; background: #f1f5f9; padding: 0 1rem; }
  @media (prefers-reduced-motion: reduce) { .panel { transition: none; } }
</style>
</head>
<body>
  <label for="details-toggle">Show details</label>
  <input type="checkbox" class="toggle" id="details-toggle">
  <div class="panel">
    <div class="panel-inner">
      <p>Ticket #11 was opened at 22:40 after five failed logins from 203.0.113.7.</p>
      <p>The account was locked and the password reset. MFA was re-enrolled the next morning.</p>
    </div>
  </div>
</body>
</html>
