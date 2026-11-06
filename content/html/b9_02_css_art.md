@@lesson b9-css-art 🎓 Beyond IT: CSS Art and Illustration
@@text
> **Beyond IT scope.** This lesson is optional. Drawing pictures with pure CSS is an art form rather than a job skill, but it's the most enjoyable way to master gradients, shapes, positioning and transforms, which you use every day in real interfaces.

CSS artists draw detailed illustrations with nothing but `<div>`s and styles. The tricks below are the same ones that make good-looking buttons, badges, backgrounds and loaders.

## Shapes from boxes

```html
<style>
  .shapes { display: flex; gap: 1rem; align-items: center; }
  .circle { width: 4rem; aspect-ratio: 1; border-radius: 50%; background: #f43f5e; }
  .pill { width: 6rem; height: 2.5rem; border-radius: 999px; background: #8b5cf6; }
  .leaf { width: 4rem; aspect-ratio: 1; border-radius: 0 70%; background: #22c55e; }
  .triangle { width: 0; height: 0; border: 2rem solid transparent; border-bottom: 3.5rem solid #f59e0b; border-top: 0; }
  .diamond { width: 3rem; aspect-ratio: 1; background: #06b6d4; rotate: 45deg; }
  .tri2 { width: 4rem; aspect-ratio: 1; background: #0ea5e9; clip-path: polygon(50% 0, 100% 100%, 0 100%); }
</style>
<div class="shapes"><div class="circle"></div><div class="pill"></div><div class="leaf"></div><div class="triangle"></div><div class="diamond"></div><div class="tri2"></div></div>
```

- `border-radius` with different values per corner (and the `/` syntax: `50% 50% 50% 50% / 60% 60% 40% 40%` for an egg) makes organic shapes.
- `clip-path: polygon(...)`, `circle()`, `ellipse()` cuts any shape.
- `aspect-ratio: 1` keeps squares square.

## Gradients are paint

```css
background: linear-gradient(to bottom, #0ea5e9, #f0f9ff);          /* sky */
background: radial-gradient(circle at 30% 30%, #fde68a, #f59e0b);   /* lit sphere */
background: conic-gradient(#ef4444 0 25%, #3b82f6 0 60%, #22c55e 0); /* pie chart */
background: repeating-linear-gradient(45deg, #000 0 10px, #facc15 0 20px);  /* hazard stripes */
```

Several gradients can be layered in one `background` (the first is on top), so one element can contain a whole scene: `background: radial-gradient(...) , linear-gradient(...)`. Hard colour stops (`#f00 0 50%, #00f 0`) make sharp edges instead of blends.

## Pseudo-elements: three shapes per element

Every element can have `::before` and `::after`, each a box you can position and shape. That's how one `<div class="cloud">` becomes a cloud:

```html
<style>
  .sky { background: linear-gradient(#38bdf8, #e0f2fe); padding: 2rem; }
  .cloud { position: relative; width: 8rem; height: 2.5rem; background: white; border-radius: 999px; margin-top: 2rem; }
  .cloud::before, .cloud::after { content: ""; position: absolute; background: white; border-radius: 50%; }
  .cloud::before { width: 3rem; height: 3rem; left: 1rem; top: -1.5rem; }
  .cloud::after { width: 4rem; height: 4rem; right: 1rem; top: -2.25rem; }
</style>
<div class="sky"><div class="cloud"></div></div>
```

## Shadows as drawing tools

`box-shadow` can be repeated with offsets and no blur to stamp copies of a shape (pixel art in one element!), and `inset` shadows add shading:

```css
.eye { box-shadow: inset -4px -4px 0 rgb(0 0 0 / 0.2), 30px 0 0 currentColor; }  /* shaded, plus a second eye 30px right */
```

## Composition tips

- Build on a container with `position: relative` and place parts with `position: absolute` using percentages so the art scales.
- Use **custom properties** for the palette and sizes, so recolouring is one line.
- Size everything relative to one variable (`--size`) or with `em`, then change the font size to scale the whole drawing.
- `mix-blend-mode` and `filter: blur()` add glow and soft light.

## Animating CSS art

Keyframes on `rotate`, `translate` and `scale` bring drawings to life: a bobbing boat, blinking eyes (`scale: 1 0.1` for a frame), drifting clouds. Wrap them in `@media (prefers-reduced-motion: no-preference)`.

## Accessibility

CSS art is decorative or a picture: wrap it in an element with `role="img"` and an `aria-label` that describes it, and make the inner `div`s `aria-hidden` (or rely on them being empty).

@@quiz
How do you make a sharp-edged two-colour background with one gradient?
* Use hard colour stops, e.g. `linear-gradient(#f00 0 50%, #00f 0)`
- Use two backgrounds
- It's impossible
- Use `border-image`
? A stop starting where the previous ended has no blend.

@@quiz
How many extra boxes can one element contribute through pseudo-elements?
- None
* Two: `::before` and `::after`
- Unlimited
- One
? Each needs `content: ""` to appear.

@@quiz
What does `conic-gradient(red 0 25%, blue 0 60%, green 0)` draw?
* A pie chart with a 25% red, 35% blue and 40% green slice
- Three stripes
- A rainbow ring
- Nothing; it's invalid
? Conic gradients sweep around a centre.

@@drill
Which two pseudo-elements give an element two extra boxes to draw with? Write the first one.
= ::before
= :before
? ::after is the other; both need content: "".

@@flashcards
`border-radius: 50%` :: circle
`border-radius: 0 70%` :: leaf shape
`clip-path: polygon()` :: cut any shape
`aspect-ratio: 1` :: keep it square
`radial-gradient(circle at 30% 30%, ...)` :: lit sphere
`conic-gradient` :: pie / colour wheel
hard colour stops :: sharp edges
layered backgrounds :: first is on top
`::before` / `::after` :: two extra shapes
`box-shadow` copies :: stamp a shape again
`role="img"` + `aria-label` :: describe CSS art

@@exercise A sunset scene in one element
Draw a sunset using **one** `<div class="scene">` and layered backgrounds only: a sun (a `radial-gradient` circle near the horizon), a sky (`linear-gradient` from deep purple at the top through orange to yellow), and a sea (a hard-stop gradient covering the bottom third). Then add stripes of reflected light with a `repeating-linear-gradient`. Give the scene `role="img"` and an `aria-label`.
@@starter
<!doctype html>
<html lang="en">
<head><meta charset="utf-8"><title>Sunset</title>
<style>
  .scene { width: 24rem; aspect-ratio: 4 / 3; }
</style>
</head>
<body>
  <div class="scene"></div>
</body>
</html>
@@requires
radial-gradient(
linear-gradient(
repeating-linear-gradient(
role="img"
aria-label=
@@hint
Order matters (first on top): reflections, sun, sea, sky. Sea: `linear-gradient(transparent 0 66%, #1e1b4b 66%)`. Sun: `radial-gradient(circle at 50% 66%, #fde047 0 12%, transparent 12.5%)`. Reflections: `repeating-linear-gradient(transparent 0 6px, rgb(253 224 71 / 0.4) 6px 8px)` sized to a narrow strip with `background-size` and `background-position`, or let it span the sea.
@@solution
<!doctype html>
<html lang="en">
<head><meta charset="utf-8"><title>Sunset</title>
<style>
  .scene {
    width: 24rem; aspect-ratio: 4 / 3; border-radius: 12px;
    background:
      repeating-linear-gradient(transparent 0 6px, rgb(253 224 71 / 0.45) 6px 8px) 50% 100% / 30% 34% no-repeat,
      radial-gradient(circle at 50% 66%, #fde047 0 12%, rgb(253 224 71 / 0.4) 13%, transparent 25%),
      linear-gradient(transparent 0 66%, #1e1b4b 66%),
      linear-gradient(#3b0764, #c2410c 55%, #facc15 66%);
  }
</style>
</head>
<body>
  <div class="scene" role="img" aria-label="A sun setting over the sea with reflections on the water"></div>
</body>
</html>

@@exercise A cloud and a smiling sun with pseudo-elements
Draw a sun with a face: a yellow circle (`border-radius: 50%`, `radial-gradient` shading) whose eyes are `::before` with a `box-shadow` copy for the second eye, and whose smile is `::after` (a circle with only a bottom border). Next to it, a cloud made of one element plus `::before` and `::after`. Animate the cloud drifting left and right with `@keyframes`, only inside `@media (prefers-reduced-motion: no-preference)`.
@@starter
<!doctype html>
<html lang="en">
<head><meta charset="utf-8"><title>Sun and cloud</title>
<style>
  .sky { background: linear-gradient(#38bdf8, #e0f2fe); padding: 2rem; display: flex; gap: 3rem; align-items: center; }
</style>
</head>
<body>
  <div class="sky">
    <div class="sun"></div>
    <div class="cloud"></div>
  </div>
</body>
</html>
@@requires
.sun::before
.sun::after
box-shadow:
.cloud::before
.cloud::after
@keyframes
prefers-reduced-motion: no-preference
role="img"
@@hint
Eyes: `.sun::before { content: ""; position: absolute; width: 0.6rem; height: 0.6rem; border-radius: 50%; background: #0f172a; left: 1.8rem; top: 2rem; box-shadow: 2.2rem 0 0 #0f172a; }`. Smile: `.sun::after { content: ""; position: absolute; width: 2.4rem; height: 1.2rem; left: 1.8rem; top: 2.6rem; border: 0.3rem solid #0f172a; border-top: 0; border-radius: 0 0 2.4rem 2.4rem; }`.
@@solution
<!doctype html>
<html lang="en">
<head><meta charset="utf-8"><title>Sun and cloud</title>
<style>
  .sky { background: linear-gradient(#38bdf8, #e0f2fe); padding: 3rem 2rem 2rem; display: flex; gap: 3rem; align-items: center; overflow: hidden; }
  .sun { position: relative; width: 6rem; aspect-ratio: 1; border-radius: 50%;
         background: radial-gradient(circle at 35% 35%, #fef08a, #facc15 60%, #eab308);
         box-shadow: 0 0 2rem #fde047; }
  .sun::before { content: ""; position: absolute; width: 0.6rem; height: 0.6rem; border-radius: 50%; background: #0f172a;
                 left: 1.8rem; top: 2rem; box-shadow: 2.2rem 0 0 #0f172a; }
  .sun::after { content: ""; position: absolute; width: 2.4rem; height: 1.2rem; left: 1.8rem; top: 2.8rem;
                border: 0.3rem solid #0f172a; border-top: 0; border-radius: 0 0 2.4rem 2.4rem; box-sizing: border-box; }
  .cloud { position: relative; width: 8rem; height: 2.5rem; background: white; border-radius: 999px; }
  .cloud::before, .cloud::after { content: ""; position: absolute; background: white; border-radius: 50%; }
  .cloud::before { width: 3rem; height: 3rem; left: 1rem; top: -1.5rem; }
  .cloud::after { width: 4rem; height: 4rem; right: 1rem; top: -2.25rem; }
  @media (prefers-reduced-motion: no-preference) {
    .cloud { animation: drift 6s ease-in-out infinite alternate; }
    @keyframes drift { from { translate: -1.5rem 0; } to { translate: 2rem 0; } }
  }
</style>
</head>
<body>
  <div class="sky" role="img" aria-label="A smiling sun next to a drifting cloud">
    <div class="sun"></div>
    <div class="cloud"></div>
  </div>
</body>
</html>

@@exercise A conic-gradient colour wheel and pie chart
Make two circles: a full colour wheel with `conic-gradient` through hues (red, yellow, lime, cyan, blue, magenta, back to red) and a hole in the middle using a `radial-gradient` layered on top (a donut); and a pie chart of ticket categories (Network 40%, Accounts 25%, Hardware 20%, Software 15%) using hard stops, with a legend list next to it. The pie gets `role="img"` and an `aria-label` that states the percentages.
@@starter
<!doctype html>
<html lang="en">
<head><meta charset="utf-8"><title>Wheels</title>
<style>
  .round { width: 10rem; aspect-ratio: 1; border-radius: 50%; }
</style>
</head>
<body>
  <div class="round wheel"></div>
  <div class="round pie"></div>
</body>
</html>
@@requires
conic-gradient(red, yellow, lime, cyan, blue, magenta, red)
radial-gradient(
0 40%
0 65%
0 85%
role="img"
@@hint
Donut: `background: radial-gradient(circle, white 0 40%, transparent 41%), conic-gradient(red, yellow, lime, cyan, blue, magenta, red);`. Pie with hard stops: `conic-gradient(#2563eb 0 40%, #f59e0b 0 65%, #16a34a 0 85%, #e11d48 0)`.
@@solution
<!doctype html>
<html lang="en">
<head><meta charset="utf-8"><title>Wheels</title>
<style>
  body { font-family: system-ui; display: flex; gap: 2rem; align-items: center; flex-wrap: wrap; }
  .round { width: 10rem; aspect-ratio: 1; border-radius: 50%; }
  .wheel { background: radial-gradient(circle, white 0 40%, transparent 41%), conic-gradient(red, yellow, lime, cyan, blue, magenta, red); }
  .pie { background: conic-gradient(#2563eb 0 40%, #f59e0b 0 65%, #16a34a 0 85%, #e11d48 0); }
  .legend { list-style: none; padding: 0; }
  .legend span { display: inline-block; width: 0.8rem; height: 0.8rem; border-radius: 2px; margin-inline-end: 0.4rem; }
</style>
</head>
<body>
  <div class="round wheel" role="img" aria-label="A colour wheel"></div>
  <div class="round pie" role="img" aria-label="Tickets by category: Network 40%, Accounts 25%, Hardware 20%, Software 15%"></div>
  <ul class="legend" aria-hidden="true">
    <li><span style="background:#2563eb"></span>Network 40%</li>
    <li><span style="background:#f59e0b"></span>Accounts 25%</li>
    <li><span style="background:#16a34a"></span>Hardware 20%</li>
    <li><span style="background:#e11d48"></span>Software 15%</li>
  </ul>
</body>
</html>
