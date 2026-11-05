@@lesson ua2-flip-view-transitions The Web Animations API, FLIP and View Transitions
@@lang html
@@text
CSS transitions handle simple state changes. For layout changes (a list reordering, an item moving to another column, a thumbnail becoming a detail view), you need two more tools: the **FLIP** technique with the **Web Animations API**, and the newer **View Transitions API**, which animates between two DOM states for you. Both run in Tauri's webview (WebView2 on Windows, WebKitGTK on Linux), but check View Transitions support and keep a fallback.

## The Web Animations API (WAAPI)

```js
const anim = el.animate(
  [{ transform: "scale(0.9)", opacity: 0 }, { transform: "none", opacity: 1 }],
  { duration: 200, easing: "cubic-bezier(0.2, 0, 0, 1)", fill: "both" }
);
await anim.finished;      // a promise
anim.reverse(); anim.pause(); anim.playbackRate = 2;
```

Keyframes are like CSS @keyframes, but controllable from code: you can cancel, reverse and chain them, and read `currentTime`.

## FLIP: First, Last, Invert, Play

Animating `top`/`left`/`width` causes layout work every frame. FLIP animates only `transform`:
1. **First**: record each element's position (`getBoundingClientRect()`).
2. **Last**: make the DOM change (reorder, move), then record the new positions.
3. **Invert**: apply a transform that moves each element back to where it was (`translate(first − last)`).
4. **Play**: animate that transform to `none`.
The browser does the layout once, and the animation runs on the compositor.

## View Transitions

```js
if (!document.startViewTransition) { update(); return; }   // fallback
document.startViewTransition(() => update());
```

- The browser snapshots the old state, runs your update, snapshots the new state, and cross-fades between them.
- Give elements a `view-transition-name` (unique per page) to make them morph between their old and new positions and sizes, which is a shared-element transition for free.
- Style with `::view-transition-old(name)`, `::view-transition-new(name)` and `::view-transition-group(name)`.
- Respect `prefers-reduced-motion` by skipping the transition or shortening it.

@@quiz
In FLIP, what does the Invert step do?
* Applies a transform that puts each element back where it was before the change
- Reverses the DOM order
- Inverts colours
- Removes the animation
? Then Play animates the transform to none.

@@quiz
Why animate transform rather than top and left?
* transform can run on the compositor without layout every frame
- top and left aren't animatable
- transform uses less memory in all cases
- It's required by WAAPI
? Layout every frame causes jank.

@@quiz
What does document.startViewTransition need from you?
* A callback that updates the DOM to the new state
- Two separate HTML files
- A canvas element
- A Lottie file
? The browser snapshots before and after.

@@quiz
What does view-transition-name do?
* Marks an element so it morphs between its old and new position and size
- Names the CSS file
- Sets the animation duration
- Disables transitions
? Names must be unique on the page.

@@quiz
What does el.animate() return?
* An Animation object you can pause, reverse and await via .finished
- Nothing
- A CSS string
- A Promise only
? WAAPI gives code control over keyframes.

@@flashcards
WAAPI :: el.animate(keyframes, options)
anim.finished :: promise resolved at the end
fill: "both" :: keep the start and end styles
FLIP :: First, Last, Invert, Play
getBoundingClientRect :: read an element's position and size
invert transform :: translate(first − last)
startViewTransition :: animate between DOM states
view-transition-name :: shared element across states
::view-transition-old / new :: style the snapshots
feature check :: if (!document.startViewTransition)

@@drill
What does the L in FLIP stand for?
= last
? First, Last, Invert, Play.

@@drill
Which CSS property gives an element a shared identity in a view transition?
= view-transition-name
? Unique per page.

@@exercise Reorder a list with FLIP
Build a list with a "Shuffle" button. When it's clicked, record each item's position (First), reorder the DOM (Last), apply an inverting `translate` (Invert), and animate it to `none` with `el.animate` (Play). Skip the animation when the user prefers reduced motion.
@@starter
<!doctype html>
<html lang="en">
<head>
  <meta charset="utf-8">
  <title>FLIP list</title>
  <style>
    body { font-family: system-ui, sans-serif; padding: 32px; }
    ul { list-style: none; padding: 0; display: grid; gap: 8px; max-width: 280px; }
    li { padding: 12px 16px; border-radius: 10px; background: #eef2ff; color: #3730a3; }
  </style>
</head>
<body>
  <button id="shuffle">Shuffle</button>
  <ul id="list"><li>Rust</li><li>Python</li><li>Go</li><li>TypeScript</li><li>C#</li></ul>
  <script>
    document.getElementById("shuffle").onclick = () => {
      const list = document.getElementById("list");
      const items = [...list.children];
      items.reverse().forEach((li) => list.appendChild(li));
    };
  </script>
</body>
</html>
@@requires
getBoundingClientRect
.animate(
translate(
prefers-reduced-motion
@@hint
Before reordering: `const first = new Map(items.map(li => [li, li.getBoundingClientRect()]))`. After reordering: for each item, `const last = li.getBoundingClientRect(); const dx = first.get(li).left - last.left; const dy = first.get(li).top - last.top;` then `li.animate([{ transform: \`translate(${dx}px, ${dy}px)\` }, { transform: "none" }], { duration: 250, easing: "ease-out" })`.
@@solution
<!doctype html>
<html lang="en">
<head>
  <meta charset="utf-8">
  <title>FLIP list</title>
  <style>
    body { font-family: system-ui, sans-serif; padding: 32px; }
    ul { list-style: none; padding: 0; display: grid; gap: 8px; max-width: 280px; }
    li { padding: 12px 16px; border-radius: 10px; background: #eef2ff; color: #3730a3; }
  </style>
</head>
<body>
  <button id="shuffle">Shuffle</button>
  <ul id="list"><li>Rust</li><li>Python</li><li>Go</li><li>TypeScript</li><li>C#</li></ul>
  <script>
    const reduce = matchMedia("(prefers-reduced-motion: reduce)");
    document.getElementById("shuffle").onclick = () => {
      const list = document.getElementById("list");
      const items = [...list.children];
      const first = new Map(items.map((li) => [li, li.getBoundingClientRect()]));   // First
      items.sort(() => Math.random() - 0.5).forEach((li) => list.appendChild(li)); // Last
      if (reduce.matches) return;
      for (const li of items) {
        const last = li.getBoundingClientRect();
        const dx = first.get(li).left - last.left;
        const dy = first.get(li).top - last.top;
        if (!dx && !dy) continue;
        li.animate(                                                                 // Invert + Play
          [{ transform: `translate(${dx}px, ${dy}px)` }, { transform: "none" }],
          { duration: 260, easing: "cubic-bezier(0.2, 0, 0, 1)" }
        );
      }
    };
  </script>
</body>
</html>

@@exercise A gallery with a view transition
Build a grid of coloured cards. Clicking a card opens it as a large detail panel using `document.startViewTransition`, with the card and panel sharing a `view-transition-name` so it morphs. Include a fallback when the API isn't supported, a close button, and a reduced-motion rule.
@@starter
<!doctype html>
<html lang="en">
<head>
  <meta charset="utf-8">
  <title>Gallery</title>
  <style>
    body { font-family: system-ui, sans-serif; padding: 24px; }
    .grid { display: grid; grid-template-columns: repeat(3, 120px); gap: 12px; }
    .card { height: 90px; border-radius: 12px; border: 0; cursor: pointer; }
  </style>
</head>
<body>
  <div class="grid">
    <button class="card" style="background:#f97316">Sunset</button>
    <button class="card" style="background:#0ea5e9">Ocean</button>
    <button class="card" style="background:#22c55e">Forest</button>
  </div>
</body>
</html>
@@requires
startViewTransition
view-transition-name
::view-transition
prefers-reduced-motion
@@hint
On click: `const go = () => { card.style.removeProperty("view-transition-name"); detail.style.setProperty("view-transition-name", "hero"); ... show the detail ... }`, with the card holding `view-transition-name: hero` before the change. Wrap it as `document.startViewTransition ? document.startViewTransition(go) : go()`. Closing does the reverse.
@@solution
<!doctype html>
<html lang="en">
<head>
  <meta charset="utf-8">
  <title>Gallery</title>
  <style>
    body { font-family: system-ui, sans-serif; padding: 24px; }
    .grid { display: grid; grid-template-columns: repeat(3, 120px); gap: 12px; }
    .card { height: 90px; border-radius: 12px; border: 0; cursor: pointer; color: #fff; font: 600 15px system-ui; }
    .detail { position: fixed; inset: 40px; border-radius: 20px; padding: 24px; color: #fff; display: none; }
    .detail.open { display: block; }
    .detail h2 { margin-top: 0; }
    .detail button { background: rgb(255 255 255 / 0.25); color: #fff; border: 0; border-radius: 8px; padding: 8px 14px; }
    ::view-transition-group(hero) { animation-duration: 320ms; animation-timing-function: cubic-bezier(0.2, 0, 0, 1); }
    @media (prefers-reduced-motion: reduce) {
      ::view-transition-group(*), ::view-transition-old(*), ::view-transition-new(*) { animation: none; }
    }
  </style>
</head>
<body>
  <div class="grid">
    <button class="card" style="background:#f97316">Sunset</button>
    <button class="card" style="background:#0ea5e9">Ocean</button>
    <button class="card" style="background:#22c55e">Forest</button>
  </div>
  <section class="detail" aria-live="polite">
    <h2></h2>
    <button class="close">Close</button>
  </section>
  <script>
    const detail = document.querySelector(".detail");
    let current = null;
    const transition = (update) => document.startViewTransition ? document.startViewTransition(update) : update();
    document.querySelectorAll(".card").forEach((card) => {
      card.addEventListener("click", () => {
        card.style.setProperty("view-transition-name", "hero");
        transition(() => {
          card.style.removeProperty("view-transition-name");
          detail.style.setProperty("view-transition-name", "hero");
          detail.style.background = card.style.background;
          detail.querySelector("h2").textContent = card.textContent;
          detail.classList.add("open");
          current = card;
        });
      });
    });
    detail.querySelector(".close").addEventListener("click", () => {
      transition(() => {
        detail.style.removeProperty("view-transition-name");
        detail.classList.remove("open");
        current.style.setProperty("view-transition-name", "hero");
      });
    });
  </script>
</body>
</html>
