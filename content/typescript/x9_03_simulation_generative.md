@@lesson x9-generative 🎓 Beyond IT: Simulations and Generative Art
@@concepts numbers, loops, strings
@@text
> **Beyond IT scope.** Procedural generation and simulation belong to games, art and science rather than IT. They're here because they're a fun way to practise algorithms and typed data structures. Skip them freely if your goal is an IT job.

Procedural content (game levels, terrain, textures, plants, music) and simulations (traffic, epidemics, ecosystems) are built from a handful of small, deterministic building blocks.

## Seeded randomness

`Math.random()` can't be seeded, so results can't be reproduced. Generators use a small **seeded PRNG** instead: the same seed always gives the same world, which makes worlds shareable ("seed 1234") and bugs reproducible. **Mulberry32** is a popular tiny one:

```ts
function mulberry32(seed: number): () => number {
  let a = seed >>> 0;
  return () => {
    a = (a + 0x6d2b79f5) >>> 0;
    let t = a;
    t = Math.imul(t ^ (t >>> 15), t | 1);
    t ^= t + Math.imul(t ^ (t >>> 7), t | 61);
    return ((t ^ (t >>> 14)) >>> 0) / 4294967296;     // [0, 1)
  };
}
const rand = mulberry32(42);
console.log(rand().toFixed(4), rand().toFixed(4));       // the same two numbers every run
```

These PRNGs are **not** cryptographically secure: use `crypto.getRandomValues` for tokens and passwords.

## Cellular automata

A grid of cells where each cell's next state depends only on its neighbours. Conway's Game of Life (a cell survives with 2–3 live neighbours and is born with exactly 3) produces gliders and even universal computers from those two rules. Variations generate cave maps for games: start with random noise, then repeatedly apply "become wall if 5+ of 9 cells are walls".

## L-systems

**Lindenmayer systems** rewrite strings with rules, then interpret the result as turtle-graphics commands. They model plant growth remarkably well:

```ts
const rules: Record<string, string> = { F: "F[+F]F[-F]F" };
let s = "F";
for (let i = 0; i < 2; i++) s = [...s].map((c) => rules[c] ?? c).join("");
console.log(s.length, s.slice(0, 20));
```

`F` = draw forward, `+`/`-` = turn, `[`/`]` = push/pop the turtle's state (a branch).

## Noise

Pure random values look like TV static. **Value noise**, **Perlin noise** and **Simplex noise** produce smooth randomness by interpolating between random values on a grid; layering several frequencies ("octaves", fractal Brownian motion) gives natural-looking terrain, clouds and wood grain.

## Agent-based simulation

Simulate many simple agents (boids flocking with three rules: separation, alignment, cohesion; an SIR epidemic model; traffic) and watch complex behaviour emerge. Keep the simulation state in typed arrays for performance, step it with a fixed time step, and render separately.

@@quiz
Why use a seeded PRNG for procedural generation?
* The same seed reproduces the same output, so worlds can be shared and bugs reproduced
- It's more random than Math.random
- It's cryptographically secure
- Browsers require it
? Determinism is the point.

@@quiz
What do `[` and `]` mean in a plant L-system's turtle commands?
- Comments
* Save and restore the turtle's position and heading (start and end a branch)
- Repeat the enclosed commands
- Draw a square
? A stack of turtle states makes branches.

@@quiz
In Conway's Game of Life, when is a dead cell born?
- With 2 live neighbours
* With exactly 3 live neighbours
- With 4 or more live neighbours
- Randomly
? Survival needs 2 or 3; birth needs exactly 3.

@@flashcards
PRNG :: pseudo-random number generator
Seed :: starting value that determines the whole sequence
Mulberry32 :: tiny fast 32-bit seeded PRNG
Cellular automaton :: grid where cells update from neighbours
Game of Life rules :: survive with 2-3, born with 3
L-system :: string rewriting rules, drawn with a turtle
Perlin / Simplex noise :: smooth gradient noise
Octaves :: layered noise frequencies for natural detail
Boids :: flocking from separation, alignment and cohesion

@@exercise Seeded generators
Using `mulberry32` from the lesson (given), write:

- `randInt(rand: () => number, min: number, max: number): number`: inclusive range
- `shuffle<T>(rand: () => number, items: readonly T[]): T[]`: a Fisher–Yates shuffle (loop `i` from the end down to 1, swap with `j = randInt(rand, 0, i)`) returning a new array
- `weightedPick<T>(rand: () => number, options: readonly { value: T; weight: number }[]): T`: pick proportionally to weight (walk the cumulative weights with `r = rand() * total`; the first option whose running total exceeds `r` wins)
- `dungeonRoom(seed: number): { width: number; height: number; loot: string[] }`: with `const rand = mulberry32(seed)`, `width = randInt(rand, 4, 9)`, `height = randInt(rand, 3, 7)`, then `randInt(rand, 0, 3)` loot items picked with weights coin 60, potion 25, gem 10, map 5. Calls must happen in exactly this order so the same seed gives the same room
@@starter
function mulberry32(seed: number): () => number {
  let a = seed >>> 0;
  return () => {
    a = (a + 0x6d2b79f5) >>> 0;
    let t = a;
    t = Math.imul(t ^ (t >>> 15), t | 1);
    t ^= t + Math.imul(t ^ (t >>> 7), t | 61);
    return ((t ^ (t >>> 14)) >>> 0) / 4294967296;
  };
}

function randInt(rand: () => number, min: number, max: number): number {
  throw new Error("TODO: not done yet");
}

function shuffle<T>(rand: () => number, items: readonly T[]): T[] {
  throw new Error("TODO: not done yet");
}

function weightedPick<T>(rand: () => number, options: readonly { value: T; weight: number }[]): T {
  throw new Error("TODO: not done yet");
}

function dungeonRoom(seed: number): { width: number; height: number; loot: string[] } {
  throw new Error("TODO: not done yet");
}

@@tests
const fixed = (values: number[]) => { let i = 0; return () => values[i++ % values.length] ?? 0; };
check([randInt(fixed([0]), 4, 9), randInt(fixed([0.999]), 4, 9), randInt(fixed([0.5]), 1, 6)], [4, 9, 4], "randInt bounds");
check(shuffle(fixed([0.1, 0.9, 0.5]), ["a", "b", "c", "d"]), ["d", "b", "c", "a"], "fisher-yates");
const opts = [{ value: "coin", weight: 60 }, { value: "gem", weight: 10 }, { value: "map", weight: 30 }];
check([0, 0.59, 0.6, 0.69, 0.7, 0.99].map((r) => weightedPick(fixed([r]), opts)), ["coin", "coin", "gem", "gem", "map", "map"], "weights");
check(JSON.stringify(dungeonRoom(7)) === JSON.stringify(dungeonRoom(7)), true, "deterministic");
const rooms = Array.from({ length: 200 }, (_, i) => dungeonRoom(i));
check([Math.min(...rooms.map((r) => r.width)), Math.max(...rooms.map((r) => r.width)), Math.max(...rooms.map((r) => r.loot.length))], [4, 9, 3], "ranges");
const allLoot = rooms.flatMap((r) => r.loot);
check(allLoot.filter((l) => l === "coin").length > allLoot.filter((l) => l === "gem").length * 3, true, "weights matter");
@@hint
`randInt`: `min + Math.floor(rand() * (max - min + 1))`. In `shuffle`, copy first (`const a = [...items]`) and swap with destructuring: `[a[i], a[j]] = [a[j] as T, a[i] as T]`. In `weightedPick`, return the last option as a fallback after the loop (rounding).
@@solution
function mulberry32(seed: number): () => number {
  let a = seed >>> 0;
  return () => {
    a = (a + 0x6d2b79f5) >>> 0;
    let t = a;
    t = Math.imul(t ^ (t >>> 15), t | 1);
    t ^= t + Math.imul(t ^ (t >>> 7), t | 61);
    return ((t ^ (t >>> 14)) >>> 0) / 4294967296;
  };
}

function randInt(rand: () => number, min: number, max: number): number {
  return min + Math.floor(rand() * (max - min + 1));
}

function shuffle<T>(rand: () => number, items: readonly T[]): T[] {
  const a = [...items];
  for (let i = a.length - 1; i >= 1; i--) {
    const j = randInt(rand, 0, i);
    [a[i], a[j]] = [a[j] as T, a[i] as T];
  }
  return a;
}

function weightedPick<T>(rand: () => number, options: readonly { value: T; weight: number }[]): T {
  const total = options.reduce((sum, o) => sum + o.weight, 0);
  const r = rand() * total;
  let running = 0;
  for (const o of options) {
    running += o.weight;
    if (running > r) return o.value;
  }
  const last = options[options.length - 1];
  if (!last) throw new Error("no options");
  return last.value;
}

const LOOT = [
  { value: "coin", weight: 60 },
  { value: "potion", weight: 25 },
  { value: "gem", weight: 10 },
  { value: "map", weight: 5 },
];

function dungeonRoom(seed: number): { width: number; height: number; loot: string[] } {
  const rand = mulberry32(seed);
  const width = randInt(rand, 4, 9);
  const height = randInt(rand, 3, 7);
  const count = randInt(rand, 0, 3);
  const loot = Array.from({ length: count }, () => weightedPick(rand, LOOT));
  return { width, height, loot };
}

@@exercise Life and L-systems
Write:

- `lifeStep(grid: readonly string[]): string[]`: one Game of Life generation on a grid of `#` (alive) and `.` (dead) rows; cells outside the grid count as dead
- `expand(axiom: string, rules: Record<string, string>, iterations: number): string`
- `turtle(commands: string, angleDeg: number): { segments: number; maxDepth: number; bounds: { minX: number; maxX: number; minY: number; maxY: number } }`: start at (0, 0) heading up (angle 90°); `F` moves 1 unit forward drawing a segment; `+` turns left by `angleDeg`, `-` right; `[` pushes the state (position, heading) and `]` pops it; other characters are ignored. `maxDepth` is the deepest bracket nesting; bounds are rounded to 2 decimals (avoid `-0`)
@@starter
function lifeStep(grid: readonly string[]): string[] {
  throw new Error("TODO: not done yet");
}

function expand(axiom: string, rules: Record<string, string>, iterations: number): string {
  throw new Error("TODO: not done yet");
}

function turtle(commands: string, angleDeg: number): { segments: number; maxDepth: number; bounds: { minX: number; maxX: number; minY: number; maxY: number } } {
  throw new Error("TODO: not done yet");
}

@@tests
const blinker = [".....", "..#..", "..#..", "..#..", "....."];
check(lifeStep(blinker), [".....", ".....", ".###.", ".....", "....."], "blinker flips");
check(lifeStep(lifeStep(blinker)), blinker, "period 2");
const glider = [".#....", "..#...", "###...", "......", "......"];
check(lifeStep(lifeStep(lifeStep(lifeStep(glider)))), ["......", "..#...", "...#..", ".###..", "......"], "glider moves diagonally");
check(expand("F", { F: "F[+F]F[-F]F" }, 2).length, 61, "expansion");
check(expand("A", { A: "AB", B: "A" }, 5), "ABAABABAABAAB", "algae");
check(turtle("FF+F", 90), { segments: 3, maxDepth: 0, bounds: { minX: -1, maxX: 0, minY: 0, maxY: 2 } }, "simple path");
check(turtle(expand("F", { F: "F[+F]F[-F]F" }, 2), 25.7), { segments: 25, maxDepth: 2, bounds: { minX: -1.3, maxX: 1.65, minY: 0, maxY: 9 } }, "plant");
@@hint
For Life, count neighbours with a helper `alive(r, c)` that returns false outside the grid. For the turtle, keep `x, y, angle` and a stack array; moving is `x += Math.cos(rad); y += Math.sin(rad)` with `rad = angle * Math.PI / 180`. Round bounds with `Math.round(v * 100) / 100 + 0` (adding 0 turns `-0` into `0`).
@@solution
function lifeStep(grid: readonly string[]): string[] {
  const alive = (r: number, c: number): boolean => grid[r]?.[c] === "#";
  return grid.map((row, r) =>
    [...row]
      .map((cell, c) => {
        let n = 0;
        for (let dr = -1; dr <= 1; dr++) for (let dc = -1; dc <= 1; dc++) if ((dr || dc) && alive(r + dr, c + dc)) n++;
        return n === 3 || (cell === "#" && n === 2) ? "#" : ".";
      })
      .join(""),
  );
}

function expand(axiom: string, rules: Record<string, string>, iterations: number): string {
  let s = axiom;
  for (let i = 0; i < iterations; i++) s = [...s].map((c) => rules[c] ?? c).join("");
  return s;
}

function turtle(commands: string, angleDeg: number): { segments: number; maxDepth: number; bounds: { minX: number; maxX: number; minY: number; maxY: number } } {
  let x = 0;
  let y = 0;
  let angle = 90;
  const stack: { x: number; y: number; angle: number }[] = [];
  let segments = 0;
  let maxDepth = 0;
  let minX = 0, maxX = 0, minY = 0, maxY = 0;
  for (const c of commands) {
    if (c === "F") {
      const rad = (angle * Math.PI) / 180;
      x += Math.cos(rad);
      y += Math.sin(rad);
      segments++;
      minX = Math.min(minX, x);
      maxX = Math.max(maxX, x);
      minY = Math.min(minY, y);
      maxY = Math.max(maxY, y);
    } else if (c === "+") angle += angleDeg;
    else if (c === "-") angle -= angleDeg;
    else if (c === "[") {
      stack.push({ x, y, angle });
      maxDepth = Math.max(maxDepth, stack.length);
    } else if (c === "]") {
      const s = stack.pop();
      if (s) ({ x, y, angle } = s);
    }
  }
  const r = (v: number): number => Math.round(v * 100) / 100 + 0;
  return { segments, maxDepth, bounds: { minX: r(minX), maxX: r(maxX), minY: r(minY), maxY: r(maxY) } };
}
