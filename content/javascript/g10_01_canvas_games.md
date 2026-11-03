@@lesson g10-canvas 🎓 Beyond IT: Browser Games with Canvas
@@concepts objects, loops, numbers
@@text
> **Beyond IT scope.** This module goes past IT and business software into games, 3D graphics and creative coding. It's here because the browser is one of the best game and graphics platforms there is, and building games is a fun way to master JavaScript. Skip it freely if your goal is an IT job.

The HTML `<canvas>` element is a bitmap you draw on with JavaScript. With `requestAnimationFrame` it runs smooth 60–144 FPS games with no plugins, and the same code runs on desktops, phones and in Electron/Tauri wrappers.

## The game loop in the browser

```javascript,norun
const canvas = document.querySelector("canvas");
const ctx = canvas.getContext("2d");
const keys = new Set();
addEventListener("keydown", (e) => keys.add(e.key));
addEventListener("keyup", (e) => keys.delete(e.key));

const paddle = { x: 280, y: 440, w: 80, h: 12, speed: 420 };
const ball = { x: 320, y: 240, r: 6, vx: 200, vy: -260 };

let last = performance.now();
function frame(now) {
  const dt = Math.min((now - last) / 1000, 0.05);   // seconds; clamp after tab switches
  last = now;
  update(dt);
  draw();
  requestAnimationFrame(frame);
}

function update(dt) {
  if (keys.has("ArrowLeft")) paddle.x -= paddle.speed * dt;
  if (keys.has("ArrowRight")) paddle.x += paddle.speed * dt;
  paddle.x = Math.max(0, Math.min(canvas.width - paddle.w, paddle.x));
  ball.x += ball.vx * dt;
  ball.y += ball.vy * dt;
  if (ball.x < ball.r || ball.x > canvas.width - ball.r) ball.vx *= -1;
  if (ball.y < ball.r) ball.vy *= -1;
}

function draw() {
  ctx.fillStyle = "#111"; ctx.fillRect(0, 0, canvas.width, canvas.height);
  ctx.fillStyle = "#4fc3f7"; ctx.fillRect(paddle.x, paddle.y, paddle.w, paddle.h);
  ctx.beginPath(); ctx.arc(ball.x, ball.y, ball.r, 0, Math.PI * 2); ctx.fillStyle = "#ffb300"; ctx.fill();
}
requestAnimationFrame(frame);
```

- `requestAnimationFrame` syncs with the display's refresh and pauses in background tabs (saving battery).
- **Delta time** (`dt`) keeps speed independent of frame rate; clamp it so a long pause doesn't teleport objects through walls.
- Draw with paths (`arc`, `lineTo`), rectangles, text and images (`drawImage` with sprite sheets). Set `canvas.width/height` for the device pixel ratio to stay sharp on high-DPI screens.

## Collisions

- **Circle vs rectangle**: find the point of the rectangle closest to the circle centre (clamp the centre into the rectangle); it collides if that point is within the radius.
- **Tunnelling**: a fast object can skip over a thin wall between frames. Fixes: smaller fixed time steps, or sweeping (raycasting along the movement).
- For many objects, avoid checking every pair: a **spatial grid** only compares objects in nearby cells.

## Game state and juice

- A **state machine** for screens: title → playing → paused → game over.
- **Entity lists** for bricks, bullets and particles; remove dead entities after updating (iterate backwards or filter).
- **"Juice"**: small particle bursts, screen shake, easing and sound make a game feel good. Particles are tiny objects with position, velocity, lifetime and fade.
- Input: keyboard, mouse/pointer events, touch, and the **Gamepad API**.
- Audio: the Web Audio API (next lessons) or a library like Howler.

## Engines and libraries

**Phaser** (2D, batteries included), **PixiJS** (fast 2D WebGL rendering), **Kaplay**, **three.js**/**Babylon.js** (3D), and **Godot** exporting to the web. Writing a small game from scratch first teaches you what they do for you.

@@quiz
Why use `requestAnimationFrame` instead of `setInterval` for a game loop?
* It runs in sync with the display refresh and pauses in background tabs
- setInterval doesn't exist in browsers
- It's the only way to draw on a canvas
- It runs on the GPU
? Combine it with delta time for consistent speed.

@@quiz
What is "tunnelling" in game physics?
- A level design technique
* A fast object passing through a thin obstacle because it moved past it between frames
- Loading assets in the background
- Network lag
? Use smaller steps or sweep tests.

@@quiz
How do you test whether a circle overlaps a rectangle?
* Clamp the circle's centre into the rectangle to find the closest point, then check its distance to the centre against the radius
- Compare their areas
- Check if the centres are equal
- Draw both and compare pixels
? This handles edges and corners correctly.

@@flashcards
Canvas 2D :: immediate-mode drawing API on a bitmap
`requestAnimationFrame` :: schedule drawing for the next display frame
Delta time :: seconds since the last frame
Tunnelling :: fast objects skipping through walls
Spatial grid :: buckets objects by area to reduce collision checks
Sprite sheet :: many animation frames in one image
Particles :: short-lived small objects for effects
Juice :: feedback effects that make a game feel good
Phaser / PixiJS :: popular 2D web game libraries

@@exercise Ball physics for a brick breaker
Write `step(ball, dt, world)` that moves a ball and handles collisions, returning `{ ball, hitBricks, lost }`:

- `ball` is `{ x, y, r, vx, vy }`; `world` is `{ width, height, paddle: {x, y, w, h}, bricks: [{ id, x, y, w, h }] }`. Don't mutate the inputs.
- move by velocity × dt
- walls: bounce off left, right and top (reflect the velocity, and put the ball back inside: `x = r` or `width - r`, `y = r`)
- falling below `height` → `lost: true`
- paddle: if the ball overlaps it **while moving down**, set `vy = -|vy|`, place it just above (`y = paddle.y - r`), and set `vx` from where it hit: `vx = speed × hit`, where `hit` = (ball.x − paddle centre) / (paddle.w / 2) clamped to −1..1, and `speed` keeps the ball's total speed constant: `vx = hit × speed × 0.75` and `vy = -sqrt(speed² − vx²)`
- bricks: every brick the ball overlaps (circle vs rectangle) is hit (return their ids in order); reflect `vy` once if any brick was hit
@@starter
function step(ball, dt, world) {
  throw new Error("TODO: not done yet");
}

@@tests
const world = { width: 400, height: 300, paddle: { x: 150, y: 280, w: 100, h: 10 }, bricks: [{ id: "a", x: 0, y: 0, w: 50, h: 20 }, { id: "b", x: 50, y: 0, w: 50, h: 20 }] };
let r = step({ x: 395, y: 150, r: 5, vx: 100, vy: 0 }, 0.1, world);
check([r.ball.x, r.ball.vx, r.lost], [395, -100, false], "right wall");
r = step({ x: 200, y: 274, r: 5, vx: 0, vy: 100 }, 0.05, world);
check([r.ball.y, r.ball.vx, r.ball.vy], [275, 0, -100], "paddle centre: straight up");
r = step({ x: 250, y: 274, r: 5, vx: 0, vy: 100 }, 0.05, world);
check([Math.round(r.ball.vx), Math.round(r.ball.vy), Math.round(Math.hypot(r.ball.vx, r.ball.vy))], [75, -66, 100], "edge hit: angled, same speed");
r = step({ x: 50, y: 30, r: 5, vx: 0, vy: -100 }, 0.1, world);
check([r.hitBricks, r.ball.vy], [["a", "b"], 100], "two bricks at once, one bounce");
r = step({ x: 10, y: 299, r: 5, vx: 0, vy: 100 }, 0.1, world);
check(r.lost, true, "lost");
const original = { x: 395, y: 150, r: 5, vx: 100, vy: 0 };
step(original, 0.1, world);
check(original.x, 395, "input not mutated");
@@hint
Copy the ball: `const b = { ...ball }`. Circle–rectangle overlap: `const cx = Math.max(rect.x, Math.min(b.x, rect.x + rect.w))` (same for y) and compare `(b.x - cx) ** 2 + (b.y - cy) ** 2 < b.r ** 2`. Compute `speed = Math.hypot(b.vx, b.vy)` before changing the paddle velocity.
@@solution
function overlaps(b, rect) {
  const cx = Math.max(rect.x, Math.min(b.x, rect.x + rect.w));
  const cy = Math.max(rect.y, Math.min(b.y, rect.y + rect.h));
  return (b.x - cx) ** 2 + (b.y - cy) ** 2 < b.r ** 2;
}

function step(ball, dt, world) {
  const b = { ...ball };
  b.x += b.vx * dt;
  b.y += b.vy * dt;
  if (b.x < b.r) { b.x = b.r; b.vx = Math.abs(b.vx); }
  if (b.x > world.width - b.r) { b.x = world.width - b.r; b.vx = -Math.abs(b.vx); }
  if (b.y < b.r) { b.y = b.r; b.vy = Math.abs(b.vy); }
  if (b.y - b.r > world.height) return { ball: b, hitBricks: [], lost: true };

  const p = world.paddle;
  if (b.vy > 0 && overlaps(b, p)) {
    const speed = Math.hypot(b.vx, b.vy);
    const hit = Math.max(-1, Math.min(1, (b.x - (p.x + p.w / 2)) / (p.w / 2)));
    b.vx = hit * speed * 0.75;
    b.vy = -Math.sqrt(speed ** 2 - b.vx ** 2);
    b.y = p.y - b.r;
  }

  const hitBricks = world.bricks.filter((brick) => overlaps(b, brick)).map((brick) => brick.id);
  if (hitBricks.length > 0) b.vy = -b.vy;
  return { ball: b, hitBricks, lost: false };
}

@@exercise A particle system
Write `class Particles` with `constructor(random = Math.random)`:

- `burst(x, y, count, { speed = 100, life = 1 })`: add `count` particles at (x, y), each moving in a random direction: angle = `random() × 2π`, velocity = `speed × (0.5 + random() / 2)` in that direction (call `random` twice per particle, angle first); each has `life` seconds left and `maxLife = life`
- `update(dt, gravity = 0)`: move each particle by its velocity × dt, add `gravity × dt` to `vy`, reduce `life` by `dt`, and **remove** particles whose life is ≤ 0
- `alphaOf(p)`: `life / maxLife` (for fading)
- `count` getter
@@starter
class Particles {
  constructor(random = Math.random) {
    throw new Error("TODO: not done yet");
  }
}

@@tests
const seq = [0, 0.5, 0.25, 1];
let i = 0;
const ps = new Particles(() => seq[i++ % seq.length]);
ps.burst(100, 50, 2, { speed: 100, life: 1 });
check(ps.count, 2, "two particles");
ps.update(0.5, 0);
const [a, b] = ps.list;
check([Math.round(a.x), Math.round(a.y), Math.round(b.x), Math.round(b.y)], [138, 50, 100, 100], "moved (75 px/s right, then 100 px/s down)");
check(ps.alphaOf(a), 0.5, "fading");
ps.update(0.5, 0);
check(ps.count, 0, "expired particles removed");
const g = new Particles(() => 0);
g.burst(0, 0, 1, { speed: 0, life: 10 });
g.update(1, 10); g.update(1, 10);
check([g.list[0].vy, g.list[0].y], [20, 10], "gravity");
@@hint
Store particles in `this.list`. In `burst`: `const angle = this.random() * Math.PI * 2; const v = speed * (0.5 + this.random() / 2); push({ x, y, vx: Math.cos(angle) * v, vy: Math.sin(angle) * v, life, maxLife: life })`. In `update`, move first, then add gravity, then filter.
@@solution
class Particles {
  constructor(random = Math.random) {
    this.random = random;
    this.list = [];
  }

  burst(x, y, count, { speed = 100, life = 1 } = {}) {
    for (let n = 0; n < count; n++) {
      const angle = this.random() * Math.PI * 2;
      const v = speed * (0.5 + this.random() / 2);
      this.list.push({ x, y, vx: Math.cos(angle) * v, vy: Math.sin(angle) * v, life, maxLife: life });
    }
  }

  update(dt, gravity = 0) {
    for (const p of this.list) {
      p.x += p.vx * dt;
      p.y += p.vy * dt;
      p.vy += gravity * dt;
      p.life -= dt;
    }
    this.list = this.list.filter((p) => p.life > 0);
  }

  alphaOf(p) {
    return p.life / p.maxLife;
  }

  get count() {
    return this.list.length;
  }
}
