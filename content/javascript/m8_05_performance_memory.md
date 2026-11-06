@@lesson m8-performance Performance, Profiling and Memory Leaks
@@concepts concurrency, objects
@@text
"Make it work, make it right, make it fast" — in that order, and only after **measuring**. This lesson covers how to find what's slow, how V8 runs JavaScript fast, and how to find memory leaks in long-running Node.js services.

## Measure first

```javascript
const { performance } = require("node:perf_hooks");
const items = Array.from({ length: 200000 }, (_, i) => `host${i}`);

function time(label, fn) {
  const start = performance.now();
  const result = fn();
  console.log(label.padEnd(18), (performance.now() - start).toFixed(1), "ms");
  return result;
}
time("array includes", () => { let n = 0; for (let i = 0; i < 300; i++) if (items.includes(`host${i * 600}`)) n++; return n; });
const set = new Set(items);
time("set has", () => { let n = 0; for (let i = 0; i < 300; i++) if (set.has(`host${i * 600}`)) n++; return n; });
```

The biggest wins almost always come from **algorithms and data structures** (an O(n) `includes` in a loop is O(n²); a `Set` makes it O(n)), and from **doing less I/O** (batch queries, cache, stream), not from micro-tweaks.

Tools:

- `console.time` / `performance.now()` for quick timing; **benchmark** libraries (`tinybench`, `mitata`) for fair comparisons (warm-up, many runs, statistics).
- `node --cpu-prof app.js` writes a `.cpuprofile`; open it in Chrome DevTools (Performance tab) to see a **flame graph** of where time goes.
- `node --inspect app.js`, then `chrome://inspect`, for live profiling and debugging.
- `clinic.js` and `0x` for flame graphs and event-loop analysis.
- In browsers: DevTools Performance and Lighthouse.

## How V8 makes JavaScript fast

V8 interprets bytecode first, then **JIT-compiles** hot functions to optimised machine code, based on the types it has seen. Code stays fast when it's predictable:

- Create objects with the **same shape** (same properties, same order); V8 gives them shared hidden classes. Adding properties later or `delete`-ing them makes objects slower.
- Keep arrays **homogeneous** (all numbers, or all objects of one shape) and without holes.
- Functions called with consistent argument types stay optimised ("monomorphic").

These matter in hot loops processing millions of items, not in typical business code.

## Memory: the garbage collector

V8 frees objects automatically once nothing can **reach** them. A **memory leak** in JavaScript means something is still holding references to data you no longer need. Typical causes in servers:

| Leak | Example | Fix |
|---|---|---|
| Unbounded cache | `cache[userId] = data` forever | LRU/TTL cache, `WeakMap` |
| Forgotten listeners | `emitter.on("x", handler)` per request, never removed | `off`/`once`, AbortSignal |
| Timers | `setInterval` never cleared | `clearInterval`, `unref()` |
| Closures | a callback capturing a huge object | capture only what you need |
| Global arrays | logging every request into an array | ship logs out, cap the size |

Node prints `MaxListenersExceededWarning` when more than 10 listeners are added to one event: often a leak.

```javascript
const before = process.memoryUsage().heapUsed;
const leak = [];
for (let i = 0; i < 1e5; i++) leak.push({ id: i, payload: "x".repeat(20) });
const after = process.memoryUsage().heapUsed;
console.log("heap grew by about", Math.round((after - before) / 1024 / 1024), "MB");
leak.length = 0;   // drop the references so the GC can free them
```

Finding leaks: take **heap snapshots** (DevTools Memory tab, or `v8.writeHeapSnapshot()`) a few minutes apart under load and compare which object types keep growing, then follow their **retainers** to the code that holds them.

## Node.js server performance checklist

1. Never block the event loop (sync I/O, huge JSON, CPU loops): see the event loop lesson.
2. Use connection pooling for databases and keep-alive for HTTP clients.
3. Stream large responses; compress (gzip/brotli) text responses.
4. Cache what's expensive and rarely changes, with an expiry.
5. Run one process per CPU core (`node:cluster`, PM2, or several containers behind a load balancer).
6. Set `NODE_ENV=production`; many libraries skip debug work then.
7. Watch memory and event-loop delay in production metrics.

@@quiz
What should you do before optimising code?
* Measure and profile to find where the time actually goes
- Rewrite it in a faster style everywhere
- Remove all comments
- Use var instead of let
? Most time is usually spent in a few hot spots, often I/O.

@@quiz
Checking `array.includes(x)` inside a loop over the same array is slow because?
- includes is buggy
* Each includes scans the array, making the loop O(n²); a Set lookup is O(1)
- Arrays can't be searched
- Strings are slow
? Data structure choice usually beats micro-optimisation.

@@quiz
What is a memory leak in JavaScript?
- Memory freed too early
* Objects you no longer need that are still referenced, so the GC can't free them
- Using too many variables
- Reading uninitialised memory
? Unbounded caches and forgotten listeners are common causes.

@@drill
Which Node.js flag starts the inspector so Chrome DevTools can profile the process?
= --inspect
? Then open chrome://inspect.

@@flashcards
Profiling :: measuring where a program spends time
Flame graph :: visualisation of time per call stack
`--cpu-prof` :: Node flag writing a CPU profile
`--inspect` :: attach Chrome DevTools to Node
JIT :: compiling hot code to machine code at runtime
Hidden class :: V8's internal shape of an object
Garbage collector :: frees unreachable objects
Heap snapshot :: record of all objects in memory, for leak hunting
Retainer :: the reference keeping an object alive

@@exercise From O(n²) to O(n)
These functions are correct but slow on large inputs. Rewrite them to be linear (or n log n) time, keeping the same results:

- `findDuplicates(ids)`: values that appear more than once, each listed once, in order of their **second** appearance
- `joinUsers(users, tickets)`: for each ticket `{ id, userId }`, return `{ id, user: <name or null> }` where users is `[{ id, name }]`
- `firstUniqueChar(text)`: the first character that appears exactly once, or `null`

The tests use 100,000 items and must finish quickly.
@@starter
function findDuplicates(ids) {
  const out = [];
  for (let i = 0; i < ids.length; i++) {
    if (ids.indexOf(ids[i]) !== i && !out.includes(ids[i])) out.push(ids[i]);
  }
  return out;
}

function joinUsers(users, tickets) {
  return tickets.map((t) => ({ id: t.id, user: users.find((u) => u.id === t.userId)?.name ?? null }));
}

function firstUniqueChar(text) {
  for (const ch of text) if (text.indexOf(ch) === text.lastIndexOf(ch)) return ch;
  return null;
}
throw new Error("TODO: make these fast");

@@tests
check(findDuplicates([3, 1, 3, 2, 1, 3]), [3, 1], "duplicates");
check(joinUsers([{ id: 1, name: "Ana" }], [{ id: 10, userId: 1 }, { id: 11, userId: 9 }]), [{ id: 10, user: "Ana" }, { id: 11, user: null }], "join");
check([firstUniqueChar("aabbcdc"), firstUniqueChar("aa")], ["d", null], "unique char");
const n = 100000;
const big = Array.from({ length: n }, (_, i) => i % 50000);
const users = Array.from({ length: n }, (_, i) => ({ id: i, name: `u${i}` }));
const tickets = Array.from({ length: n }, (_, i) => ({ id: i, userId: n - 1 - i }));
const start = Date.now();
check(findDuplicates(big).length, 50000, "big duplicates");
check(joinUsers(users, tickets)[0].user, `u${n - 1}`, "big join");
check(firstUniqueChar("x".repeat(n) + "yz"), "y", "big text");
check(Date.now() - start < 1000, true, "fast enough");
@@hint
Duplicates: a `Set` of seen values and a `Set` of reported ones. Join: build `new Map(users.map((u) => [u.id, u.name]))` once. Unique char: count characters in a `Map` in one pass, then find the first with count 1 in a second pass.
@@solution
function findDuplicates(ids) {
  const seen = new Set();
  const reported = new Set();
  const out = [];
  for (const id of ids) {
    if (seen.has(id)) {
      if (!reported.has(id)) {
        reported.add(id);
        out.push(id);
      }
    } else {
      seen.add(id);
    }
  }
  return out;
}

function joinUsers(users, tickets) {
  const names = new Map(users.map((u) => [u.id, u.name]));
  return tickets.map((t) => ({ id: t.id, user: names.get(t.userId) ?? null }));
}

function firstUniqueChar(text) {
  const counts = new Map();
  for (const ch of text) counts.set(ch, (counts.get(ch) ?? 0) + 1);
  for (const ch of text) if (counts.get(ch) === 1) return ch;
  return null;
}

@@exercise Find the leak
`createServer()` simulates a request handler that leaks memory. Rewrite it so that it **doesn't grow without bound**, keeping its behaviour:

- `handle(userId)`: returns `"hello <name>"` where the name comes from `lookup(userId)`; results are cached, but the cache must hold at most **100** users (evict the oldest inserted)
- every request subscribes to config changes via `bus.on("config", ...)` so it can log; the subscription must be **removed** when the request finishes
- `stats()` returns `{ cached, listeners }` (cache size and `bus.listenerCount("config")`)
@@starter
const { EventEmitter } = require("node:events");

function createServer(lookup) {
  const bus = new EventEmitter();
  bus.setMaxListeners(0);
  const cache = {};
  function handle(userId) {
    bus.on("config", () => console.log("config changed during request", userId));
    if (!(userId in cache)) cache[userId] = lookup(userId);
    return `hello ${cache[userId]}`;
  }
  function stats() {
    return { cached: Object.keys(cache).length, listeners: bus.listenerCount("config") };
  }
  throw new Error("TODO: fix the leaks");
  return { handle, stats, bus };
}

@@tests
let lookups = 0;
const server = createServer((id) => { lookups++; return `user${id}`; });
for (let i = 0; i < 5000; i++) server.handle(i % 300);
check(server.handle(299), "hello user299", "behaviour");
check(server.stats(), { cached: 100, listeners: 0 }, "bounded");
const before = lookups;
server.handle(299); server.handle(299);
check(lookups, before, "recent entries are cached");
@@hint
Use a `Map` for the cache; after inserting, `if (cache.size > 100) cache.delete(cache.keys().next().value)`. Register the listener as a named function and remove it in a `finally` block with `bus.off("config", listener)`.
@@solution
const { EventEmitter } = require("node:events");

function createServer(lookup) {
  const bus = new EventEmitter();
  const cache = new Map();
  function handle(userId) {
    const listener = () => console.log("config changed during request", userId);
    bus.on("config", listener);
    try {
      if (!cache.has(userId)) {
        cache.set(userId, lookup(userId));
        if (cache.size > 100) cache.delete(cache.keys().next().value);
      }
      return `hello ${cache.get(userId)}`;
    } finally {
      bus.off("config", listener);
    }
  }
  function stats() {
    return { cached: cache.size, listeners: bus.listenerCount("config") };
  }
  return { handle, stats, bus };
}
