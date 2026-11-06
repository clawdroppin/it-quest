@@lesson x9-ecs 🎓 Beyond IT: A Typed Entity-Component-System Game Engine
@@concepts objects, loops, numbers
@@text
> **Beyond IT scope.** This lesson is about game engine architecture, which is outside IT and business software. It's included because the patterns (data-oriented design, typed queries, fixed time steps) are a great TypeScript workout. Skip it freely if your goal is an IT job.

Classic object-oriented games use deep inheritance (`Player extends Character extends Sprite`), which breaks down when you want a flying, burning, collectible enemy. Modern engines (Unity DOTS, Bevy, Flecs, and web engines like bitECS) use **Entity-Component-System**:

- **Entity**: just an id (a number).
- **Component**: plain data attached to an entity: `Position`, `Velocity`, `Health`, `Sprite`.
- **System**: a function that runs every frame over all entities that **have** certain components: "for every entity with Position and Velocity, move it".

Behaviour comes from combining components, not from class hierarchies.

## Typing components and queries

One type map lists every component:

```ts
type Components = {
  position: { x: number; y: number };
  velocity: { dx: number; dy: number };
  health: { hp: number; max: number };
  player: { name: string };
};
type Name = keyof Components;
type WithComponents<K extends Name> = { id: number } & { [P in K]: Components[P] };

const e: WithComponents<"position" | "health"> = { id: 1, position: { x: 0, y: 0 }, health: { hp: 3, max: 3 } };
console.log(e.health.hp);
```

A query like `world.query("position", "velocity")` can then return exactly `WithComponents<"position" | "velocity">[]`: systems get autocomplete and can't touch components they didn't ask for.

## The game loop with a fixed time step

Frame times vary (a 144 Hz monitor, a slow laptop, a tab in the background). Physics must not depend on the frame rate, so engines step the simulation in **fixed** increments and carry the remainder:

```ts
function makeLoop(step: number, update: (dt: number) => void) {
  let acc = 0;
  return (frameDt: number): number => {
    acc += Math.min(frameDt, 0.25);       // avoid the "spiral of death" after a long pause
    let steps = 0;
    while (acc >= step) { update(step); acc -= step; steps++; }
    return steps;
  };
}
const tick = makeLoop(0.02, () => {});
console.log(tick(0.07), tick(0.005));     // 3 steps, then 0 (the remainder carries over)
```

Rendering then **interpolates** between the last two physics states for smooth motion.

## Collision detection

The simplest shape is an **axis-aligned bounding box** (AABB): two boxes overlap when they overlap on both the x and y axes. Real engines first narrow candidates with a spatial grid or quadtree (the "broad phase") before exact checks (the "narrow phase").

## Storage layouts

Simple ECSs store a `Map<entity, component>` per component type (what the exercise does). High-performance ones use **structure of arrays** (one typed array per field, like `Float32Array` for all x positions) for cache-friendly loops: the same idea as columnar databases.

@@quiz
In ECS, where does behaviour live?
- In entity classes
- In components
* In systems that run over entities with certain components
- In the renderer
? Components are pure data; systems are functions.

@@quiz
Why use a fixed time step for physics?
* So the simulation behaves the same regardless of the frame rate
- To render faster
- Because browsers require it
- To save memory
? Accumulate frame time and step in fixed increments.

@@quiz
When do two AABBs overlap?
- When their centres are close
* When their intervals overlap on both the x and the y axis
- When one contains the other's centre
- When their areas are equal
? Check each axis separately.

@@drill
What's the name of updating a simulation in equal time slices regardless of frame rate?
= fixed time step
= fixed timestep
= fixed time-step
? Accumulate frame time and step in fixed amounts.

@@flashcards
Entity :: an id with no data or behaviour
Component :: plain data attached to an entity
System :: function over entities with given components
Query :: find entities that have a set of components
Fixed time step :: simulate in constant increments
Accumulator :: carries leftover frame time
AABB :: axis-aligned bounding box
Broad phase :: cheap filtering of collision candidates
Structure of arrays :: one array per field for fast loops

@@exercise A typed world with queries
Write `class World` for the `Components` map from the lesson:

- `spawn(components: Partial<Components>): number`: ids start at 1
- `add<K extends Name>(id: number, name: K, data: Components[K]): void` and `remove(id: number, name: Name): void`
- `get<K extends Name>(id: number, name: K): Components[K] | undefined`
- `despawn(id: number): void`
- `query<K extends Name>(...names: K[]): WithComponents<K>[]`: entities having **all** the components, in id order; the component objects are the stored ones (systems mutate them in place)
- `count(): number`: live entities
@@starter
type Components = {
  position: { x: number; y: number };
  velocity: { dx: number; dy: number };
  health: { hp: number; max: number };
  player: { name: string };
};
type Name = keyof Components;
type WithComponents<K extends Name> = { id: number } & { [P in K]: Components[P] };

class World {
  spawn(components: Partial<Components>): number {
    throw new Error("TODO: not done yet");
  }
  add<K extends Name>(id: number, name: K, data: Components[K]): void {
    throw new Error("TODO: not done yet");
  }
  remove(id: number, name: Name): void {
    throw new Error("TODO: not done yet");
  }
  get<K extends Name>(id: number, name: K): Components[K] | undefined {
    throw new Error("TODO: not done yet");
  }
  despawn(id: number): void {
    throw new Error("TODO: not done yet");
  }
  query<K extends Name>(...names: K[]): WithComponents<K>[] {
    throw new Error("TODO: not done yet");
  }
  count(): number {
    throw new Error("TODO: not done yet");
  }
}

@@tests
const world = new World();
const hero = world.spawn({ player: { name: "Aki" }, position: { x: 0, y: 0 }, velocity: { dx: 1, dy: 0 }, health: { hp: 5, max: 5 } });
const rock = world.spawn({ position: { x: 5, y: 5 } });
const bird = world.spawn({ position: { x: 2, y: 9 }, velocity: { dx: 0, dy: -2 } });
function movement(w: World, dt: number): void {
  for (const e of w.query("position", "velocity")) {
    e.position.x += e.velocity.dx * dt;
    e.position.y += e.velocity.dy * dt;
  }
}
movement(world, 2);
check([world.get(hero, "position"), world.get(rock, "position"), world.get(bird, "position")], [{ x: 2, y: 0 }, { x: 5, y: 5 }, { x: 2, y: 5 }], "movement system");
check(world.query("position").map((e) => e.id), [1, 2, 3], "query order");
world.remove(bird, "velocity");
world.add(rock, "velocity", { dx: -1, dy: 0 });
check(world.query("velocity").map((e) => e.id), [1, 2], "add/remove");
world.despawn(hero);
check([world.count(), world.query("player").length, world.get(hero, "position")], [2, 0, undefined], "despawn");
check(world.spawn({}), 4, "ids keep increasing");
function _typeTests(): void {
  for (const e of world.query("health")) {
    // @ts-expect-error position wasn't queried
    e.position.x = 0;
  }
}
@@hint
Store `private stores: { [K in Name]: Map<number, Components[K]> }` initialised with a `new Map()` per name, plus a `Set<number>` of live ids. In `query`, iterate the live ids (sorted), skip any missing a component, and build `{ id, [name]: store.get(id) }`; cast the built object to `WithComponents<K>`.
@@solution
type Components = {
  position: { x: number; y: number };
  velocity: { dx: number; dy: number };
  health: { hp: number; max: number };
  player: { name: string };
};
type Name = keyof Components;
type WithComponents<K extends Name> = { id: number } & { [P in K]: Components[P] };

class World {
  private nextId = 1;
  private readonly alive = new Set<number>();
  private readonly stores: { [K in Name]: Map<number, Components[K]> } = {
    position: new Map(),
    velocity: new Map(),
    health: new Map(),
    player: new Map(),
  };

  spawn(components: Partial<Components>): number {
    const id = this.nextId++;
    this.alive.add(id);
    for (const name of Object.keys(components) as Name[]) {
      const data = components[name];
      if (data !== undefined) (this.stores[name] as Map<number, unknown>).set(id, data);
    }
    return id;
  }

  add<K extends Name>(id: number, name: K, data: Components[K]): void {
    if (this.alive.has(id)) (this.stores[name] as Map<number, Components[K]>).set(id, data);
  }

  remove(id: number, name: Name): void {
    this.stores[name].delete(id);
  }

  get<K extends Name>(id: number, name: K): Components[K] | undefined {
    return (this.stores[name] as Map<number, Components[K]>).get(id);
  }

  despawn(id: number): void {
    this.alive.delete(id);
    for (const store of Object.values(this.stores)) store.delete(id);
  }

  query<K extends Name>(...names: K[]): WithComponents<K>[] {
    const out: WithComponents<K>[] = [];
    for (const id of [...this.alive].sort((a, b) => a - b)) {
      const row: Record<string, unknown> = { id };
      let complete = true;
      for (const name of names) {
        const data = (this.stores[name] as Map<number, unknown>).get(id);
        if (data === undefined) {
          complete = false;
          break;
        }
        row[name] = data;
      }
      if (complete) out.push(row as WithComponents<K>);
    }
    return out;
  }

  count(): number {
    return this.alive.size;
  }
}

@@exercise Fixed-step physics and collisions
Write:

- `makeLoop(step: number, update: (dt: number) => void): (frameDt: number) => { steps: number; alpha: number }`: as in the lesson (clamp each frame to 0.25 s); `alpha` is the leftover fraction `acc / step` (for interpolation), rounded to 3 decimals
- `overlaps(a: Box, b: Box): boolean` for `type Box = { x: number; y: number; w: number; h: number }` (x/y is the top-left corner; touching edges do **not** overlap)
- `collisions(boxes: { id: string; box: Box }[], cell: number): [string, string][]`: a broad phase with a uniform grid of `cell`-sized cells; put each box into every cell it covers, test only pairs that share a cell, and return each overlapping pair once as `[smaller id, larger id]`, sorted
@@starter
type Box = { x: number; y: number; w: number; h: number };

function makeLoop(step: number, update: (dt: number) => void): (frameDt: number) => { steps: number; alpha: number } {
  throw new Error("TODO: not done yet");
}

function overlaps(a: Box, b: Box): boolean {
  throw new Error("TODO: not done yet");
}

function collisions(boxes: { id: string; box: Box }[], cell: number): [string, string][] {
  throw new Error("TODO: not done yet");
}

@@tests
let simulated = 0;
const tick = makeLoop(0.02, (dt) => { simulated += dt; });
check([tick(0.05), tick(0.012), tick(5)], [{ steps: 2, alpha: 0.5 }, { steps: 1, alpha: 0.1 }, { steps: 12, alpha: 0.6 }], "fixed steps with carry and clamp");
check(Math.round(simulated * 1000) / 1000, 0.3, "simulated time");
check([overlaps({ x: 0, y: 0, w: 10, h: 10 }, { x: 5, y: 5, w: 10, h: 10 }), overlaps({ x: 0, y: 0, w: 10, h: 10 }, { x: 10, y: 0, w: 5, h: 5 })], [true, false], "overlap and touching");
const boxes = [
  { id: "player", box: { x: 8, y: 8, w: 4, h: 4 } },
  { id: "coin", box: { x: 11, y: 11, w: 2, h: 2 } },
  { id: "wall", box: { x: 0, y: 0, w: 40, h: 2 } },
  { id: "bat", box: { x: 30, y: 1, w: 3, h: 3 } },
  { id: "tree", box: { x: 50, y: 50, w: 5, h: 5 } },
];
check(collisions(boxes, 10), [["bat", "wall"], ["coin", "player"]], "broad + narrow phase");
@@hint
Floating point: compare with a tiny epsilon in the loop (`while (acc >= step - 1e-9)`) so 0.06 / 0.02 gives 3 steps, not 2. For the grid, a box covers cells from `Math.floor(x / cell)` to `Math.floor((x + w) / cell)` on each axis; key cells as `` `${cx},${cy}` ``. Collect candidate pairs in a `Set` of `"a|b"` strings (ids sorted) to dedupe.
@@solution
type Box = { x: number; y: number; w: number; h: number };

function makeLoop(step: number, update: (dt: number) => void): (frameDt: number) => { steps: number; alpha: number } {
  let acc = 0;
  return (frameDt) => {
    acc += Math.min(frameDt, 0.25);
    let steps = 0;
    while (acc >= step - 1e-9) {
      update(step);
      acc -= step;
      steps++;
    }
    if (acc < 0) acc = 0;
    return { steps, alpha: Math.round((acc / step) * 1000) / 1000 };
  };
}

function overlaps(a: Box, b: Box): boolean {
  return a.x < b.x + b.w && b.x < a.x + a.w && a.y < b.y + b.h && b.y < a.y + a.h;
}

function collisions(boxes: { id: string; box: Box }[], cell: number): [string, string][] {
  const grid = new Map<string, { id: string; box: Box }[]>();
  for (const item of boxes) {
    const { x, y, w, h } = item.box;
    for (let cx = Math.floor(x / cell); cx <= Math.floor((x + w) / cell); cx++) {
      for (let cy = Math.floor(y / cell); cy <= Math.floor((y + h) / cell); cy++) {
        const key = `${cx},${cy}`;
        const list = grid.get(key) ?? [];
        list.push(item);
        grid.set(key, list);
      }
    }
  }
  const seen = new Set<string>();
  const out: [string, string][] = [];
  for (const list of grid.values()) {
    for (let i = 0; i < list.length; i++) {
      for (let j = i + 1; j < list.length; j++) {
        const a = list[i];
        const b = list[j];
        if (!a || !b) continue;
        const pair: [string, string] = a.id < b.id ? [a.id, b.id] : [b.id, a.id];
        const key = pair.join("|");
        if (seen.has(key)) continue;
        seen.add(key);
        if (overlaps(a.box, b.box)) out.push(pair);
      }
    }
  }
  return out.sort((p, q) => p[0].localeCompare(q[0]) || p[1].localeCompare(q[1]));
}
