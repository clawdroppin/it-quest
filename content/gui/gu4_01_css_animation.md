@@lesson gu4-css-animation Cute Animations: Transitions, Keyframes and Micro-interactions
@@concepts functions
@@text
Good animation isn't decoration: it **explains** what happened. A toast slides in from where it lives, a deleted row shrinks away, a heart bounces to say "yes, liked". Done well, it makes an app feel friendly and alive. Done badly, it's slow and annoying. This lesson covers both the tools and the taste.

## Transitions: animate between two states

A **transition** smoothly animates a property when its value changes, for example on `:hover` or when JavaScript adds a class.

```html
<style>
  body { font-family: system-ui, sans-serif; padding: 32px; display: flex; gap: 24px; }
  .card { width: 140px; padding: 20px; border-radius: 16px; background: #fff7ed; text-align: center; font-size: 40px;
          box-shadow: 0 2px 6px rgb(0 0 0 / .08);
          transition: transform .25s cubic-bezier(.34, 1.56, .64, 1), box-shadow .25s; }
  .card:hover { transform: translateY(-6px) rotate(-2deg); box-shadow: 0 14px 28px rgb(0 0 0 / .14); }
</style>
<div class="card">🐶</div><div class="card">🐱</div><div class="card">🐰</div>
```

`transition: <property> <duration> <easing> <delay>`. List the properties you mean (`transition: all` animates things you didn't intend and is slower).

## Easing: the secret to "cute"

Real objects don't start and stop instantly. The **easing curve** is what makes motion feel natural or bouncy:

| Easing | Feels like | Use for |
|---|---|---|
| `linear` | robotic | spinners, progress, colour cycles |
| `ease-out` | fast start, gentle stop | things **entering** (menus, toasts) |
| `ease-in` | slow start, fast end | things **leaving** |
| `ease-in-out` | smooth both ends | things moving from A to B |
| `cubic-bezier(.34, 1.56, .64, 1)` | overshoots and settles: **bouncy** | playful pops, toggles, likes |

A `cubic-bezier` with a value above 1 overshoots the target and comes back, which is the core of "cute". Dev tools let you drag the curve and preview it.

## Duration

- Small things (buttons, toggles, hovers): **100 to 200 ms**.
- Medium (menus, toasts, cards): **200 to 350 ms**.
- Big (page transitions, dialogs): **300 to 500 ms**.

Anything over half a second starts to feel like waiting. Leaving is usually a bit faster than entering.

## Keyframes: multi-step animations

`@keyframes` describes an animation with several steps; `animation` plays it.

```html
<style>
  body { font-family: system-ui, sans-serif; padding: 32px; display: flex; gap: 32px; font-size: 44px; }
  .bounce { animation: bounce 1s ease-in-out infinite; }
  @keyframes bounce {
    0%, 100% { transform: translateY(0) scale(1, 1); }
    40%      { transform: translateY(-24px) scale(.95, 1.05); }    /* stretch going up */
    80%      { transform: translateY(0) scale(1.08, .92); }        /* squash on landing */
  }
  .wiggle { display: inline-block; animation: wiggle 1.6s ease-in-out infinite; transform-origin: 50% 0; }
  @keyframes wiggle {
    0%, 60%, 100% { transform: rotate(0); }
    10%, 30% { transform: rotate(14deg); }
    20%, 40% { transform: rotate(-14deg); }
  }
  .float { animation: float 3s ease-in-out infinite; }
  @keyframes float { 50% { transform: translateY(-8px); } }
  .pulse { animation: pulse 1.4s ease-out infinite; border-radius: 50%; }
  @keyframes pulse { 0% { box-shadow: 0 0 0 0 rgb(239 68 68 / .6); } 100% { box-shadow: 0 0 0 18px rgb(239 68 68 / 0); } }
</style>
<div class="bounce">🐣</div>
<div class="wiggle">🔔</div>
<div class="float">🎈</div>
<div class="pulse">🔴</div>
```

The bounce uses **squash and stretch**, one of Disney's 12 principles of animation: a soft thing stretches as it moves fast and squashes when it lands. It's the quickest way to make something feel cute.

The `animation` shorthand: `name duration easing delay iteration-count direction fill-mode`. Useful values: `infinite`, `alternate` (play back and forth), `forwards` (keep the last frame), and `animation-delay` to **stagger** a list so items appear one after another.

## Micro-interactions

A **micro-interaction** is a tiny animation that answers one action. The classic: a like button that pops.

```html
<style>
  body { font-family: system-ui, sans-serif; padding: 32px; }
  .like { font-size: 28px; background: none; border: 0; cursor: pointer; color: #9ca3af; position: relative; }
  .like.on { color: #ef4444; animation: pop .45s cubic-bezier(.34, 1.56, .64, 1); }
  @keyframes pop { 0% { transform: scale(1); } 30% { transform: scale(.7); } 60% { transform: scale(1.35); } 100% { transform: scale(1); } }
  .like.on::after {               /* a ring that bursts outward */
    content: ""; position: absolute; inset: -6px; border-radius: 50%; border: 3px solid #fca5a5;
    animation: burst .5s ease-out forwards;
  }
  @keyframes burst { from { transform: scale(.4); opacity: 1; } to { transform: scale(1.6); opacity: 0; } }
</style>
<button class="like" aria-pressed="false" aria-label="Like">♥</button>
<script>
  const like = document.querySelector(".like");
  like.addEventListener("click", () => {
    const on = like.classList.toggle("on");
    like.setAttribute("aria-pressed", on);
  });
</script>
```

To **replay** an animation on every click, remove the class, force a reflow (`void el.offsetWidth`), and add it again; or use the Web Animations API, which is simpler:

```js,norun
el.animate(
  [{ transform: "scale(1)" }, { transform: "scale(1.3)" }, { transform: "scale(1)" }],
  { duration: 300, easing: "cubic-bezier(.34, 1.56, .64, 1)" }
);
```

## Confetti in 15 lines

Celebrations (task done, streak reached) are where cute animation shines. Create small elements, give each a random direction with CSS variables, and let one keyframe animation do the rest:

```html
<style>
  body { font-family: system-ui, sans-serif; padding: 48px; text-align: center; overflow: hidden; }
  button { font: inherit; font-size: 18px; padding: 10px 20px; border-radius: 12px; border: 0; background: #22c55e; color: white; cursor: pointer; }
  .bit { position: fixed; width: 8px; height: 12px; border-radius: 2px; pointer-events: none;
         animation: fly 1s cubic-bezier(.2, .8, .3, 1) forwards; }
  @keyframes fly { to { transform: translate(var(--x), var(--y)) rotate(var(--r)); opacity: 0; } }
</style>
<button id="done">Mark done ✓</button>
<script>
  const colors = ["#f472b6", "#facc15", "#60a5fa", "#34d399", "#a78bfa"];
  document.getElementById("done").addEventListener("click", (e) => {
    const box = e.target.getBoundingClientRect();
    for (let i = 0; i < 40; i++) {
      const b = document.createElement("div");
      b.className = "bit";
      b.style.left = box.left + box.width / 2 + "px";
      b.style.top = box.top + "px";
      b.style.background = colors[i % colors.length];
      b.style.setProperty("--x", (Math.random() - .5) * 400 + "px");
      b.style.setProperty("--y", (Math.random() - .8) * 300 + "px");
      b.style.setProperty("--r", Math.random() * 720 + "deg");
      document.body.append(b);
      b.addEventListener("animationend", () => b.remove());     // clean up
    }
  });
</script>
```

## Performance: animate transform and opacity

Browsers can animate `transform` (move, scale, rotate) and `opacity` on the GPU without recalculating the layout. Animating `width`, `height`, `top`, `left` or `margin` makes the browser redo the layout on every frame, which stutters. So move things with `translate`, not `left`; grow them with `scale`, not `width`. Aim for **60 frames per second** (16 ms per frame).

## Respect reduced motion

Some people get dizzy or sick from motion (vestibular disorders), and they turn on "reduce motion" in their OS. Always honour it:

```css
@media (prefers-reduced-motion: reduce) {
  *, *::before, *::after {
    animation-duration: 0.01ms !important;
    animation-iteration-count: 1 !important;
    transition-duration: 0.01ms !important;
  }
}
```

Or flip it around: only add big motion inside `@media (prefers-reduced-motion: no-preference) { ... }`. Small fades are usually fine; bouncing, zooming, parallax and spinning are not.

## Animation taste

- **Purposeful**: every animation should explain a change (where did it come from, where did it go, did it work?).
- **Quick**: under 300 ms for most UI.
- **Consistent**: same easing and durations everywhere. Make them tokens: `--ease-bounce`, `--dur-fast`.
- **Not in the way**: people should never wait for an animation to finish before they can act.
- **Infinite animations** only for things that really are ongoing (loading, live recording), never on static content.

@@quiz
Which easing gives a playful, bouncy overshoot?
- `linear`
- `ease-in`
* `cubic-bezier(.34, 1.56, .64, 1)`
- `steps(4)`
? A control point above 1 makes the motion go past the target and settle back.

@@quiz
Why animate `transform: translateX()` instead of `left`?
- `left` doesn't work with transitions
* `transform` and `opacity` can be animated without recalculating layout, so they stay smooth
- `transform` works in older browsers only
- `left` is deprecated
? Layout properties (width, height, top, left, margin) cause stutter when animated.

@@quiz
Which easing should a menu use when it opens (enters)?
- `ease-in`
* `ease-out`
- `linear`
- No easing
? Fast start, gentle stop feels responsive for entering things; `ease-in` suits leaving.

@@quiz
What should happen to a bouncing animation when `prefers-reduced-motion: reduce` matches?
- Play it twice as fast
* Remove or tone it down, because the user asked their OS for less motion
- Nothing, it's only a suggestion
- Show a warning
? Motion can make some people physically ill. Simple fades are usually still okay.

@@quiz
What's "squash and stretch"?
- A CSS property
* An animation principle: soft objects stretch when moving fast and squash on impact, which makes motion feel alive
- A way to compress images
- A flexbox setting
? `scale(.95, 1.05)` going up and `scale(1.08, .92)` on landing make a bounce feel squishy and cute.

@@drill
Which CSS property animates smoothly between two states?
= transition
? transition: transform 200ms ease-out.

@@flashcards
`transition: prop dur easing` :: animate a property when its value changes
`@keyframes name { 0% {...} 100% {...} }` :: a multi-step animation
`animation: name 1s ease infinite` :: play keyframes
`ease-out` / `ease-in` :: for entering / leaving
`cubic-bezier(.34, 1.56, .64, 1)` :: bouncy overshoot
`animation-fill-mode: forwards` :: keep the last frame
`animation-delay` :: stagger items in a list
`alternate` :: play forwards then backwards
squash and stretch :: deform soft things as they move and land
micro-interaction :: a tiny animation answering one action
`el.animate(keyframes, options)` :: the Web Animations API
`animationend` event :: clean up after an animation
`transform` and `opacity` :: cheap to animate (GPU); avoid width/top/left
`prefers-reduced-motion: reduce` :: the user wants less motion
UI animation duration :: about 100 to 350 ms; over 500 ms feels slow

@@exercise A bouncy hover card
Make the card lift when hovered: give `.card` a `transition` on `transform` with the bouncy easing `cubic-bezier(.34, 1.56, .64, 1)`, and on `.card:hover` move it up with `translateY(` and grow it slightly with `scale(`. Then add a `@media (prefers-reduced-motion: reduce)` block that sets `transition: none` on `.card`.
@@starter
<!doctype html>
<html lang="en">
<head>
  <meta charset="utf-8">
  <title>Hover card</title>
  <style>
    body { font-family: system-ui, sans-serif; padding: 48px; background: #fdf4ff; }
    .card { width: 180px; padding: 24px; border-radius: 20px; background: white; text-align: center;
            box-shadow: 0 4px 12px rgb(0 0 0 / .08); }
    .card div { font-size: 48px; }
  </style>
</head>
<body>
  <div class="card"><div>🐼</div><p>Pandas sleep 10 hours a day.</p></div>
</body>
</html>
@@requires
transition: transform
cubic-bezier(.34, 1.56, .64, 1)
.card:hover
translateY(
scale(
@media (prefers-reduced-motion: reduce)
transition: none
@@hint
`.card { transition: transform .3s cubic-bezier(.34, 1.56, .64, 1); }` and `.card:hover { transform: translateY(-8px) scale(1.03); }`. Then `@media (prefers-reduced-motion: reduce) { .card { transition: none; } }`.
@@solution
<!doctype html>
<html lang="en">
<head>
  <meta charset="utf-8">
  <title>Hover card</title>
  <style>
    body { font-family: system-ui, sans-serif; padding: 48px; background: #fdf4ff; }
    .card { width: 180px; padding: 24px; border-radius: 20px; background: white; text-align: center;
            box-shadow: 0 4px 12px rgb(0 0 0 / .08);
            transition: transform .3s cubic-bezier(.34, 1.56, .64, 1); }
    .card:hover { transform: translateY(-8px) scale(1.03); }
    .card div { font-size: 48px; }
    @media (prefers-reduced-motion: reduce) {
      .card { transition: none; }
    }
  </style>
</head>
<body>
  <div class="card"><div>🐼</div><p>Pandas sleep 10 hours a day.</p></div>
</body>
</html>

@@exercise A wiggling notification bell
Make the bell wiggle when it has the class `ring`. Write `@keyframes wiggle` that rotates back and forth (use `rotate(` with positive and negative angles), set `transform-origin` to the top (`50% 0`) so it swings like a bell, and apply it with `.bell.ring { animation: wiggle ... }`. Make the button add the `ring` class on click, and remove it on `animationend` so it can ring again.
@@starter
<!doctype html>
<html lang="en">
<head>
  <meta charset="utf-8">
  <title>Bell</title>
  <style>
    body { font-family: system-ui, sans-serif; padding: 48px; }
    .bell { display: inline-block; font-size: 48px; }
    button { font: inherit; margin-left: 16px; padding: 8px 14px; border-radius: 8px; }
  </style>
</head>
<body>
  <span class="bell">🔔</span>
  <button id="notify">New message</button>
  <script>
    // TODO
  </script>
</body>
</html>
@@requires
@keyframes wiggle
rotate(
transform-origin: 50% 0
.bell.ring
animation: wiggle
classList.add("ring")
animationend
classList.remove("ring")
@@hint
`@keyframes wiggle { 0%, 100% { transform: rotate(0); } 20%, 60% { transform: rotate(15deg); } 40%, 80% { transform: rotate(-15deg); } }`. In the script: `notify.addEventListener("click", () => bell.classList.add("ring"))` and `bell.addEventListener("animationend", () => bell.classList.remove("ring"))`.
@@solution
<!doctype html>
<html lang="en">
<head>
  <meta charset="utf-8">
  <title>Bell</title>
  <style>
    body { font-family: system-ui, sans-serif; padding: 48px; }
    .bell { display: inline-block; font-size: 48px; transform-origin: 50% 0; }
    .bell.ring { animation: wiggle .8s ease-in-out; }
    @keyframes wiggle {
      0%, 100% { transform: rotate(0); }
      20%, 60% { transform: rotate(15deg); }
      40%, 80% { transform: rotate(-15deg); }
    }
    button { font: inherit; margin-left: 16px; padding: 8px 14px; border-radius: 8px; }
  </style>
</head>
<body>
  <span class="bell">🔔</span>
  <button id="notify">New message</button>
  <script>
    const bell = document.querySelector(".bell");
    document.getElementById("notify").addEventListener("click", () => bell.classList.add("ring"));
    bell.addEventListener("animationend", () => bell.classList.remove("ring"));
  </script>
</body>
</html>

@@exercise Staggered list entrance
Make the list items slide in one after another. Write `@keyframes slide-in` that starts `from` with `opacity: 0` and a `translateY(`, give `li` the animation with `animation-fill-mode: both` (so they're hidden before they start), and stagger them with `animation-delay` using `:nth-child(2)`, `:nth-child(3)` and `:nth-child(4)`.
@@starter
<!doctype html>
<html lang="en">
<head>
  <meta charset="utf-8">
  <title>Stagger</title>
  <style>
    body { font-family: system-ui, sans-serif; padding: 32px; }
    ul { list-style: none; padding: 0; max-width: 280px; }
    li { background: #ecfeff; margin: 8px 0; padding: 12px 16px; border-radius: 12px; }
  </style>
</head>
<body>
  <ul>
    <li>🌱 Water the plants</li>
    <li>📮 Post the letter</li>
    <li>🧁 Bake cupcakes</li>
    <li>📚 Return library books</li>
  </ul>
</body>
</html>
@@requires
@keyframes slide-in
opacity: 0
translateY(
animation-fill-mode: both
li:nth-child(2)
li:nth-child(3)
li:nth-child(4)
animation-delay
@@hint
`li { animation: slide-in .4s ease-out; animation-fill-mode: both; }`, `@keyframes slide-in { from { opacity: 0; transform: translateY(12px); } }`, then `li:nth-child(2) { animation-delay: .08s; }` and so on.
@@solution
<!doctype html>
<html lang="en">
<head>
  <meta charset="utf-8">
  <title>Stagger</title>
  <style>
    body { font-family: system-ui, sans-serif; padding: 32px; }
    ul { list-style: none; padding: 0; max-width: 280px; }
    li { background: #ecfeff; margin: 8px 0; padding: 12px 16px; border-radius: 12px;
         animation: slide-in .4s ease-out; animation-fill-mode: both; }
    li:nth-child(2) { animation-delay: .08s; }
    li:nth-child(3) { animation-delay: .16s; }
    li:nth-child(4) { animation-delay: .24s; }
    @keyframes slide-in { from { opacity: 0; transform: translateY(12px); } }
  </style>
</head>
<body>
  <ul>
    <li>🌱 Water the plants</li>
    <li>📮 Post the letter</li>
    <li>🧁 Bake cupcakes</li>
    <li>📚 Return library books</li>
  </ul>
</body>
</html>
