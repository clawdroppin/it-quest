@@lesson b9-canvas 🎓 Beyond IT: Generative Art with Canvas
@@text
> **Beyond IT scope.** This lesson is optional. Generative art is a creative coding topic rather than an IT skill, but it's a fun way to practise loops, maths and the canvas API, which do show up in dashboards, charts and image tools.

The `<canvas>` element is a bitmap you draw on with JavaScript. Unlike SVG, it doesn't keep a tree of shapes: you paint pixels, and the canvas forgets how they got there. That makes it fast for thousands of shapes, animations and pixel effects.

## Drawing basics

```html
<canvas id="c" width="320" height="200" role="img" aria-label="A sun over the sea"></canvas>
<script>
  const ctx = document.getElementById("c").getContext("2d");
  ctx.fillStyle = "#0ea5e9"; ctx.fillRect(0, 0, 320, 200);          // sky
  ctx.fillStyle = "#1e3a8a"; ctx.fillRect(0, 130, 320, 70);         // sea
  ctx.beginPath(); ctx.arc(160, 130, 40, Math.PI, 0);               // half circle
  ctx.fillStyle = "#facc15"; ctx.fill();
  ctx.strokeStyle = "white"; ctx.lineWidth = 2;
  for (let y = 145; y < 200; y += 12) {
    ctx.beginPath(); ctx.moveTo(130 - (y - 130), y); ctx.lineTo(190 + (y - 130), y); ctx.stroke();
  }
</script>
```

- `width`/`height` **attributes** set the drawing resolution; CSS only stretches the result. For sharp drawing on high-DPI screens, multiply the attributes by `devicePixelRatio` and `ctx.scale()` by it.
- Paths: `beginPath()`, `moveTo`, `lineTo`, `arc(x, y, r, start, end)`, `bezierCurveTo`, `closePath()`, then `fill()` or `stroke()`.
- Colours: any CSS colour, including `hsl(200 80% 50% / 0.5)`, which is ideal for generative work (vary the hue number).
- Transforms: `translate`, `rotate`, `scale`, wrapped in `save()`/`restore()`.

## Randomness with a seed

`Math.random()` gives a different picture each time. Generative artists use a **seeded** random generator so a seed number always reproduces the same artwork:

```js,norun
function mulberry32(seed) {
  return function () {
    seed |= 0; seed = (seed + 0x6d2b79f5) | 0;
    let t = Math.imul(seed ^ (seed >>> 15), 1 | seed);
    t = (t + Math.imul(t ^ (t >>> 7), 61 | t)) ^ t;
    return ((t ^ (t >>> 14)) >>> 0) / 4294967296;
  };
}
const rand = mulberry32(42);   // rand() → same sequence for seed 42
```

## Classic techniques

- **Grids with variation**: loop over a grid, and in each cell draw something with a random rotation, size or colour (the "Truchet tiles" and "10 PRINT" mazes).
- **Recursion**: a tree that draws a branch, then two smaller branches at angles from its end, down to a depth limit.
- **Noise and flow fields**: particles follow angles from a smooth noise function, leaving trails.
- **Polar coordinates**: `x = cx + r * cos(θ)`, `y = cy + r * sin(θ)` for spirals, roses (`r = cos(kθ)`) and circular patterns.
- **Accumulation**: draw thousands of semi-transparent shapes; density emerges.

## Animation

```js,norun
function frame(t) {
  ctx.fillStyle = "rgb(15 23 42 / 0.1)";   // translucent clear leaves trails
  ctx.fillRect(0, 0, w, h);
  // ... draw based on t (milliseconds)
  requestAnimationFrame(frame);
}
requestAnimationFrame(frame);
```

`requestAnimationFrame` syncs with the screen refresh and pauses in background tabs. Respect `prefers-reduced-motion` by drawing a single still frame instead.

## Exporting

`canvas.toDataURL("image/png")` or `canvas.toBlob()` gives you an image file; put it in a download link's `href` with a `download` attribute.

## Accessibility

A canvas is a single image to assistive technology. Give it `role="img"` and an `aria-label` describing what it shows, or put fallback text between `<canvas>` and `</canvas>`.

@@quiz
What do the canvas `width` and `height` attributes control, compared with CSS width?
* The attributes set the drawing resolution; CSS only scales the displayed result
- They're the same thing
- CSS sets the resolution
- Attributes are ignored
? Mismatches cause blurry or stretched drawings.

@@quiz
Why use a seeded random generator in generative art?
- It's more random
* The same seed reproduces the same artwork exactly
- Math.random is unavailable in canvas
- It's faster
? You can share or re-render a piece by its seed.

@@quiz
How do you get a point on a circle of radius r around (cx, cy) at angle θ?
* `x = cx + r·cos θ`, `y = cy + r·sin θ`
- `x = cx · r`, `y = cy · θ`
- `x = r + θ`, `y = r − θ`
- `x = cos(cx)`, `y = sin(cy)`
? Polar to Cartesian coordinates.

@@drill
Which method gets the 2D drawing context from a canvas element?
= getContext
= getContext("2d")
= getContext('2d')
= canvas.getContext("2d")
? const ctx = canvas.getContext("2d").

@@flashcards
`getContext("2d")` :: canvas drawing API
`fillRect(x, y, w, h)` :: filled rectangle
`arc(x, y, r, a0, a1)` :: circle segment
`beginPath()` … `fill()` / `stroke()` :: draw a path
`save()` / `restore()` :: push/pop transforms and styles
`hsl(h s l / a)` :: easy colour variation
seeded PRNG :: reproducible randomness
`requestAnimationFrame` :: animation loop
`toDataURL()` :: export as image
`role="img"` + `aria-label` :: accessible canvas

@@exercise A Truchet tile maze
Fill a 400×400 canvas with a 20×20 grid of cells. In each cell draw a diagonal line from top-left to bottom-right **or** from top-right to bottom-left, choosing with a seeded random generator (`mulberry32` with seed 7) so the maze is the same each time. Add an input for the seed that redraws on change, and give the canvas `role="img"` with an `aria-label`.
@@starter
<!doctype html>
<html lang="en">
<head><meta charset="utf-8"><title>Maze</title></head>
<body>
  <canvas id="c" width="400" height="400"></canvas>
</body>
</html>
@@requires
function mulberry32(
getContext("2d")
moveTo(
lineTo(
role="img"
aria-label=
type="number"
@@hint
`for (let y = 0; y < 20; y++) for (let x = 0; x < 20; x++) { const px = x * 20, py = y * 20; ctx.beginPath(); if (rand() < 0.5) { ctx.moveTo(px, py); ctx.lineTo(px + 20, py + 20); } else { ctx.moveTo(px + 20, py); ctx.lineTo(px, py + 20); } ctx.stroke(); }`
@@solution
<!doctype html>
<html lang="en">
<head><meta charset="utf-8"><title>Maze</title></head>
<body>
  <label for="seed">Seed</label> <input id="seed" type="number" value="7">
  <canvas id="c" width="400" height="400" role="img" aria-label="A random diagonal-line maze pattern"></canvas>
  <script>
    function mulberry32(seed) {
      return function () {
        seed |= 0; seed = (seed + 0x6d2b79f5) | 0;
        let t = Math.imul(seed ^ (seed >>> 15), 1 | seed);
        t = (t + Math.imul(t ^ (t >>> 7), 61 | t)) ^ t;
        return ((t ^ (t >>> 14)) >>> 0) / 4294967296;
      };
    }
    const canvas = document.getElementById("c");
    const ctx = canvas.getContext("2d");
    function draw(seed) {
      const rand = mulberry32(seed);
      ctx.fillStyle = "#0f172a";
      ctx.fillRect(0, 0, 400, 400);
      ctx.strokeStyle = "#38bdf8";
      ctx.lineWidth = 3;
      ctx.lineCap = "round";
      const size = 20;
      for (let y = 0; y < 20; y++) {
        for (let x = 0; x < 20; x++) {
          const px = x * size, py = y * size;
          ctx.beginPath();
          if (rand() < 0.5) { ctx.moveTo(px, py); ctx.lineTo(px + size, py + size); }
          else { ctx.moveTo(px + size, py); ctx.lineTo(px, py + size); }
          ctx.stroke();
        }
      }
    }
    const seedInput = document.getElementById("seed");
    seedInput.addEventListener("change", () => draw(Number(seedInput.value)));
    draw(7);
  </script>
</body>
</html>

@@exercise A recursive tree
Draw a fractal tree: a function `branch(x, y, length, angle, depth)` draws a line from (x, y) in direction `angle` and, if `depth > 0`, calls itself twice from the end point with `length * 0.72` and the angle turned by ±0.45 radians. Start at the bottom centre of a 400×300 canvas pointing straight up (`-Math.PI / 2`) with length 70 and depth 9. Make the line width shrink with depth and the colour go from brown trunk to green tips (vary the `hsl` hue with depth).
@@starter
<!doctype html>
<html lang="en">
<head><meta charset="utf-8"><title>Tree</title></head>
<body>
  <canvas id="c" width="400" height="300" role="img" aria-label="A fractal tree"></canvas>
</body>
</html>
@@requires
function branch(x, y, length, angle, depth)
Math.cos(angle)
Math.sin(angle)
length * 0.72
0.45
-Math.PI / 2
hsl(
@@hint
End point: `const x2 = x + Math.cos(angle) * length, y2 = y + Math.sin(angle) * length;`. Colour: `ctx.strokeStyle = `hsl(${30 + (9 - depth) * 10} 60% 35%)`;` and `ctx.lineWidth = depth + 1;`.
@@solution
<!doctype html>
<html lang="en">
<head><meta charset="utf-8"><title>Tree</title></head>
<body>
  <canvas id="c" width="400" height="300" role="img" aria-label="A fractal tree"></canvas>
  <script>
    const ctx = document.getElementById("c").getContext("2d");
    ctx.lineCap = "round";
    const MAX = 9;
    function branch(x, y, length, angle, depth) {
      const x2 = x + Math.cos(angle) * length;
      const y2 = y + Math.sin(angle) * length;
      ctx.strokeStyle = `hsl(${30 + (MAX - depth) * 10} 60% ${30 + (MAX - depth) * 3}%)`;
      ctx.lineWidth = depth + 1;
      ctx.beginPath();
      ctx.moveTo(x, y);
      ctx.lineTo(x2, y2);
      ctx.stroke();
      if (depth > 0) {
        branch(x2, y2, length * 0.72, angle - 0.45, depth - 1);
        branch(x2, y2, length * 0.72, angle + 0.45, depth - 1);
      }
    }
    branch(200, 300, 70, -Math.PI / 2, MAX);
  </script>
</body>
</html>

@@exercise An animated polar rose
Animate a rose curve `r = R · cos(k·θ)` on a 300×300 canvas: each frame, plot points for θ from 0 to 2π (step 0.01) using `x = cx + r·cos θ`, `y = cy + r·sin θ`, with `k` slowly changing over time (`k = 2 + Math.sin(t / 2000) * 3`). Clear with a translucent fill so old frames leave trails. Use `requestAnimationFrame`, and under `prefers-reduced-motion: reduce` (checked with `matchMedia`) draw one still frame with k = 4 instead.
@@starter
<!doctype html>
<html lang="en">
<head><meta charset="utf-8"><title>Rose</title></head>
<body>
  <canvas id="c" width="300" height="300" role="img" aria-label="An animated flower-like rose curve"></canvas>
</body>
</html>
@@requires
requestAnimationFrame(
matchMedia("(prefers-reduced-motion: reduce)")
Math.cos(k * theta)
Math.sin(t / 2000)
2 * Math.PI
@@hint
`function draw(k) { ctx.fillStyle = "rgb(15 23 42 / 0.15)"; ctx.fillRect(0, 0, 300, 300); ctx.beginPath(); for (let theta = 0; theta <= 2 * Math.PI; theta += 0.01) { const r = 120 * Math.cos(k * theta); const x = 150 + r * Math.cos(theta), y = 150 + r * Math.sin(theta); theta === 0 ? ctx.moveTo(x, y) : ctx.lineTo(x, y); } ctx.stroke(); }`
@@solution
<!doctype html>
<html lang="en">
<head><meta charset="utf-8"><title>Rose</title></head>
<body>
  <canvas id="c" width="300" height="300" role="img" aria-label="An animated flower-like rose curve"></canvas>
  <script>
    const ctx = document.getElementById("c").getContext("2d");
    function draw(k, hue) {
      ctx.fillStyle = "rgb(15 23 42 / 0.15)";
      ctx.fillRect(0, 0, 300, 300);
      ctx.strokeStyle = `hsl(${hue} 80% 65%)`;
      ctx.lineWidth = 1.5;
      ctx.beginPath();
      for (let theta = 0; theta <= 2 * Math.PI; theta += 0.01) {
        const r = 120 * Math.cos(k * theta);
        const x = 150 + r * Math.cos(theta);
        const y = 150 + r * Math.sin(theta);
        if (theta === 0) ctx.moveTo(x, y); else ctx.lineTo(x, y);
      }
      ctx.stroke();
    }
    if (matchMedia("(prefers-reduced-motion: reduce)").matches) {
      draw(4, 320);
    } else {
      function frame(t) {
        const k = 2 + Math.sin(t / 2000) * 3;
        draw(k, (t / 30) % 360);
        requestAnimationFrame(frame);
      }
      requestAnimationFrame(frame);
    }
  </script>
</body>
</html>
