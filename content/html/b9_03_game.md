@@lesson b9-game 🎓 Beyond IT: Build a Canvas Game (Packet Catcher)
@@text
> **Beyond IT scope.** This lesson is optional. Game programming isn't an IT job skill, but a small game exercises everything at once: a loop, state, input, collision maths, timing, sound-free feedback and accessibility.

We'll build an original arcade game, **Packet Catcher**: data packets fall from the top of the screen and you move a router left and right to catch them. Green packets score points; red "malware" packets cost a life. It speeds up as you score.

## The game loop

Every real-time game has the same skeleton:

```js,norun
let last = performance.now();
function loop(now) {
  const dt = Math.min((now - last) / 1000, 0.05);   // seconds since last frame, capped
  last = now;
  update(dt);    // move things, check collisions, change state
  render();      // draw the current state
  if (state.running) requestAnimationFrame(loop);
}
requestAnimationFrame(loop);
```

**Delta time** (`dt`) makes movement speed independent of the frame rate: `x += speed * dt` moves `speed` pixels per second whether the screen runs at 60 Hz or 144 Hz. Capping `dt` stops objects teleporting after the tab was in the background.

## State

Keep everything the game needs in one object, separate from drawing:

```js,norun
const state = {
  running: false, score: 0, lives: 3, speed: 120,
  player: { x: 160, w: 60, h: 12 },
  packets: [],          // { x, y, r, bad }
  spawnTimer: 0,
};
```

`render()` reads the state and draws it; `update()` changes it. That separation makes the logic testable and the bugs easier to find.

## Input

```js,norun
const keys = new Set();
addEventListener("keydown", e => keys.add(e.key));
addEventListener("keyup", e => keys.delete(e.key));
// in update():
if (keys.has("ArrowLeft") || keys.has("a")) state.player.x -= 300 * dt;
```

Track held keys in a set rather than moving on each `keydown` (which repeats at the OS key-repeat rate). Add pointer support (`pointermove` over the canvas) for mice and touch screens.

## Collision

Circle (packet) against rectangle (player): find the closest point of the rectangle to the circle's centre and compare the distance with the radius:

```js,norun
function hits(c, r) {
  const cx = Math.max(r.x, Math.min(c.x, r.x + r.w));
  const cy = Math.max(r.y, Math.min(c.y, r.y + r.h));
  return (c.x - cx) ** 2 + (c.y - cy) ** 2 <= c.r ** 2;
}
```

## Difficulty and feel

- Spawn packets on a timer that shortens as the score grows; increase fall speed gradually.
- Give feedback: flash the screen edge when you lose a life, briefly enlarge the score when it changes.
- Pause on `Escape` or `P`, and automatically when the tab is hidden (`visibilitychange`).
- Store the high score in `localStorage`.

## Accessible games

Games can't always be fully accessible, but simple steps help a lot: a pause key; keyboard **and** pointer controls; a speed setting; no information by colour alone (malware packets are red **and** drawn with an ✕); a live region that announces score and lives at intervals and on game over; respecting `prefers-reduced-motion` by turning off screen shake and flashes.

@@quiz
Why multiply movement by delta time?
* So speed is the same in pixels per second regardless of frame rate
- To make the game slower
- Because canvas requires it
- To save battery
? 60 Hz and 144 Hz screens should play the same.

@@quiz
Why track held keys in a Set instead of moving on each keydown event?
- Sets are faster to draw
* keydown repeats at the OS repeat rate after a delay, which gives jerky movement; polling held keys each frame is smooth
- keydown doesn't work on canvas
- To support gamepads
? Update positions in the loop based on what's held.

@@quiz
How do you detect a circle overlapping a rectangle?
- Compare their x coordinates only
* Clamp the circle's centre to the rectangle to find the closest point, then compare that distance with the radius
- Check whether the centres are equal
- Use `getBoundingClientRect`
? Closest-point test.

@@drill
Which browser function should drive a canvas game loop?
= requestAnimationFrame
= window.requestAnimationFrame
? It syncs with the display's refresh.

@@flashcards
game loop :: update(dt) then render()
delta time :: seconds since last frame
cap dt :: avoid jumps after background tabs
state object :: all game data in one place
held-keys Set :: smooth keyboard movement
`pointermove` :: mouse and touch input
circle–rect collision :: clamp then distance
`visibilitychange` :: auto-pause when hidden
`localStorage` :: high score
colour + shape :: don't rely on colour alone

@@exercise Build Packet Catcher
Build the game on a 360×480 canvas: a router (rectangle) at the bottom moved with arrow keys / A and D **and** with the pointer; packets spawning at random x positions, 20% of them malware (red with an ✕), falling faster as the score grows; +1 per good packet caught, −1 life for malware caught or a good packet missed; game over at 0 lives with a high score kept in `localStorage`. Use a `requestAnimationFrame` loop with delta time, a circle–rectangle `hits()` function, pause with `p`/Escape and on `visibilitychange`, a Start/Restart button, and a polite live region that announces "Game over. Score N."
@@starter
<!doctype html>
<html lang="en">
<head><meta charset="utf-8"><title>Packet Catcher</title></head>
<body>
  <h1>Packet Catcher</h1>
  <canvas id="game" width="360" height="480"></canvas>
</body>
</html>
@@requires
requestAnimationFrame(
performance.now()
function hits(
localStorage
visibilitychange
pointermove
ArrowLeft
role="status"
role="img"
@@hint
Write it in stages, running the page after each: draw a static player; move it with keys; spawn and fall packets; catch and score; lives and game over; pause; high score; announcements. Keep `update(dt)` and `render()` separate.
@@solution
<!doctype html>
<html lang="en">
<head><meta charset="utf-8">
<meta name="viewport" content="width=device-width, initial-scale=1">
<title>Packet Catcher</title>
<style>
  body { font-family: system-ui; background: #0f172a; color: #e2e8f0; display: grid; justify-items: center; gap: 0.5rem; }
  canvas { background: #020617; border-radius: 10px; max-inline-size: 100%; touch-action: none; }
  .hud { display: flex; gap: 1rem; align-items: center; }
  button { font: inherit; padding: 0.4rem 1rem; }
  button:focus-visible { outline: 3px solid #38bdf8; }
  .sr { position: absolute; width: 1px; height: 1px; overflow: hidden; clip: rect(0 0 0 0); }
</style>
</head>
<body>
  <h1>Packet Catcher</h1>
  <div class="hud">
    <button id="start">Start</button>
    <span>Arrows / A D or the pointer to move · P or Esc to pause</span>
  </div>
  <canvas id="game" width="360" height="480" role="img" aria-label="Packet Catcher game area. Catch green packets, avoid red malware packets marked with a cross."></canvas>
  <p id="announce" role="status" class="sr"></p>
  <script>
    const canvas = document.getElementById("game");
    const ctx = canvas.getContext("2d");
    const W = canvas.width, H = canvas.height;
    const announce = msg => { document.getElementById("announce").textContent = msg; };
    let best = 0;
    try { best = Number(localStorage.getItem("packet-best")) || 0; } catch {}

    let state;
    function reset() {
      state = {
        running: true, paused: false, over: false, score: 0, lives: 3,
        player: { x: W / 2 - 30, y: H - 30, w: 60, h: 12 },
        packets: [], spawnTimer: 0, flash: 0,
      };
    }

    const keys = new Set();
    addEventListener("keydown", e => {
      keys.add(e.key);
      if ((e.key === "p" || e.key === "Escape") && state && state.running) togglePause();
    });
    addEventListener("keyup", e => keys.delete(e.key));
    canvas.addEventListener("pointermove", e => {
      if (!state || !state.running) return;
      const rect = canvas.getBoundingClientRect();
      const x = (e.clientX - rect.left) * (W / rect.width);
      state.player.x = Math.max(0, Math.min(W - state.player.w, x - state.player.w / 2));
    });
    document.addEventListener("visibilitychange", () => {
      if (document.hidden && state && state.running && !state.paused) togglePause();
    });

    function togglePause() {
      state.paused = !state.paused;
      announce(state.paused ? "Paused" : "Resumed");
      if (!state.paused) { last = performance.now(); requestAnimationFrame(loop); }
      render();
    }

    function hits(c, r) {
      const cx = Math.max(r.x, Math.min(c.x, r.x + r.w));
      const cy = Math.max(r.y, Math.min(c.y, r.y + r.h));
      return (c.x - cx) ** 2 + (c.y - cy) ** 2 <= c.r ** 2;
    }

    function update(dt) {
      const p = state.player;
      if (keys.has("ArrowLeft") || keys.has("a")) p.x -= 320 * dt;
      if (keys.has("ArrowRight") || keys.has("d")) p.x += 320 * dt;
      p.x = Math.max(0, Math.min(W - p.w, p.x));

      const fall = 120 + state.score * 6;
      state.spawnTimer -= dt;
      if (state.spawnTimer <= 0) {
        state.spawnTimer = Math.max(0.35, 1.1 - state.score * 0.02);
        state.packets.push({ x: 15 + Math.random() * (W - 30), y: -10, r: 10, bad: Math.random() < 0.2 });
      }
      for (const pk of state.packets) pk.y += fall * dt;

      const remaining = [];
      for (const pk of state.packets) {
        if (hits(pk, p)) {
          if (pk.bad) loseLife(); else state.score++;
        } else if (pk.y - pk.r > H) {
          if (!pk.bad) loseLife();
        } else {
          remaining.push(pk);
        }
      }
      state.packets = remaining;
      state.flash = Math.max(0, state.flash - dt);
    }

    function loseLife() {
      state.lives--;
      state.flash = matchMedia("(prefers-reduced-motion: reduce)").matches ? 0 : 0.25;
      announce(`${state.lives} ${state.lives === 1 ? "life" : "lives"} left`);
      if (state.lives <= 0) gameOver();
    }

    function gameOver() {
      state.running = false;
      state.over = true;
      if (state.score > best) {
        best = state.score;
        try { localStorage.setItem("packet-best", String(best)); } catch {}
      }
      announce(`Game over. Score ${state.score}. Best ${best}.`);
      document.getElementById("start").textContent = "Restart";
      document.getElementById("start").focus();
    }

    function render() {
      ctx.fillStyle = "#020617";
      ctx.fillRect(0, 0, W, H);
      if (state.flash > 0) { ctx.strokeStyle = "#ef4444"; ctx.lineWidth = 8; ctx.strokeRect(0, 0, W, H); }
      for (const pk of state.packets) {
        ctx.beginPath();
        ctx.arc(pk.x, pk.y, pk.r, 0, Math.PI * 2);
        ctx.fillStyle = pk.bad ? "#ef4444" : "#22c55e";
        ctx.fill();
        if (pk.bad) {
          ctx.strokeStyle = "white"; ctx.lineWidth = 2; ctx.beginPath();
          ctx.moveTo(pk.x - 5, pk.y - 5); ctx.lineTo(pk.x + 5, pk.y + 5);
          ctx.moveTo(pk.x + 5, pk.y - 5); ctx.lineTo(pk.x - 5, pk.y + 5);
          ctx.stroke();
        }
      }
      const p = state.player;
      ctx.fillStyle = "#38bdf8";
      ctx.fillRect(p.x, p.y, p.w, p.h);
      ctx.fillStyle = "#e2e8f0";
      ctx.font = "16px system-ui";
      ctx.fillText(`Score ${state.score}   Lives ${state.lives}   Best ${best}`, 10, 22);
      if (state.paused) { ctx.font = "28px system-ui"; ctx.fillText("Paused", W / 2 - 45, H / 2); }
      if (state.over) { ctx.font = "28px system-ui"; ctx.fillText("Game over", W / 2 - 65, H / 2); }
    }

    let last = performance.now();
    function loop(now) {
      if (!state.running || state.paused) return;
      const dt = Math.min((now - last) / 1000, 0.05);
      last = now;
      update(dt);
      render();
      if (state.running && !state.paused) requestAnimationFrame(loop);
    }

    document.getElementById("start").addEventListener("click", () => {
      reset();
      announce("Game started. 3 lives.");
      canvas.focus();
      last = performance.now();
      requestAnimationFrame(loop);
    });

    state = { running: false, paused: false, over: false, score: 0, lives: 3, player: { x: W / 2 - 30, y: H - 30, w: 60, h: 12 }, packets: [], flash: 0 };
    render();
  </script>
</body>
</html>
