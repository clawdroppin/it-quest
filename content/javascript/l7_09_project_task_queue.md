@@lesson l7-project 🏆 Module Project: A Concurrent Task Queue
@@concepts concurrency, objects, errors
@@text
This project combines the module's ideas (classes with private state, promises and the event loop, events, error handling, functional options) into a small library you'd actually use: a **task queue** that runs async jobs with a concurrency limit, priorities, timeouts, retries and progress events. Libraries like `p-queue` and BullMQ, and every deployment or backup tool, contain one.

## What it does

```text
add(task, {priority})         ┌──────────── TaskQueue (concurrency = 2) ────────────┐
  ───────────────────────▶    │ waiting: [high#4] [normal#2] [normal#3] [low#5]      │
                              │ running: [#1] [#6]                                   │
                              └──────────────────────────────────────────────────────┘
                                  │ when a slot frees up: start the next waiting task
                                  ▼
                        events: "start", "done", "error", "idle"
```

- **Concurrency limit**: at most N tasks run at once. Pinging 500 hosts with 500 simultaneous connections can overwhelm the network or the target; 20 at a time is polite and fast.
- **Priorities**: higher-priority tasks start first; equal priorities run in the order they were added (FIFO).
- **Promises per task**: `add()` returns a promise for that task's result, so callers can `await` individual jobs.
- **Events**: progress reporting without coupling the queue to a UI.
- **`onIdle()`**: a promise that resolves when nothing is waiting or running.

## Design notes

- Keep the waiting list sorted by `(priority desc, sequence asc)`. A sorted array is fine for thousands of tasks; a **binary heap** scales to millions.
- After every task finishes (success **or** failure), call a private `#next()` that fills free slots. Forgetting the failure path is the classic bug that makes queues stall.
- Never let one task's rejection crash the queue: catch it, reject **that task's** promise, emit `"error"`, and carry on.
- `pause()` stops starting new tasks (running ones finish); `resume()` continues.
- Use the `EventEmitter` from `node:events` in real code; the exercise uses a tiny emitter so it runs anywhere.

```javascript
const { EventEmitter } = require("node:events");
const bus = new EventEmitter();
bus.on("done", ({ id, ms }) => console.log(`task ${id} finished in ${ms} ms`));
bus.emit("done", { id: 1, ms: 42 });
console.log("listeners:", bus.listenerCount("done"));
```

## Using it

```javascript,norun
const queue = new TaskQueue({ concurrency: 20 });
queue.on("done", ({ id }) => progress.tick());
const results = await Promise.allSettled(hosts.map((h) => queue.add(() => ping(h), { timeout: 2000 })));
await queue.onIdle();
```

@@quiz
Why should a task's failure still trigger starting the next task?
* Otherwise the free slot is never reused and the queue stalls
- Failures should stop the whole queue
- It doesn't matter
- To retry the same task forever
? Use `finally` so both paths call `#next()`.

@@quiz
Two tasks have the same priority. Which runs first?
- A random one
* The one added first (FIFO), using a sequence number as a tie-breaker
- The shorter one
- The one with the longer name
? Stable ordering makes behaviour predictable.

@@quiz
Why limit concurrency when checking 500 hosts?
- JavaScript can only run 2 promises at once
* To avoid overwhelming the network, the targets or local resources like file descriptors
- Because promises are slow
- It's required by fetch
? Unlimited parallelism can look like a denial-of-service attack.

@@drill
What's the name of the maximum number of tasks a queue runs at once?
= concurrency
= concurrency limit
? Extra tasks wait until a slot frees up.

@@flashcards
Concurrency limit :: maximum number of tasks running at once
Priority queue :: highest priority first; FIFO among equals
Binary heap :: efficient priority queue structure
`onIdle()` :: promise resolved when the queue is empty and nothing runs
Backpressure :: slowing producers when consumers can't keep up
EventEmitter :: Node.js class for on/emit events

@@exercise The task queue
Write `class TaskQueue` with `constructor({ concurrency = 1 } = {})`:

- `add(task, { priority = 0 } = {})`: `task` is a function returning a promise. Returns a promise resolving/rejecting with the task's result. Each task gets an `id` (1, 2, 3, ... in order added).
- runs at most `concurrency` tasks at once; picks the highest `priority` first, FIFO among equal priorities
- `on(event, fn)` and events: `"start"` `{ id }`, `"done"` `{ id, result }`, `"error"` `{ id, error }`, `"idle"` (no payload) when the last running task finishes and nothing is waiting
- getters `pending` (waiting count) and `running`
- `pause()` / `resume()`
- `onIdle()`: a promise that resolves when nothing is waiting or running (immediately if already idle)

Tasks must start **asynchronously**: `add()` schedules a start (e.g. with `queueMicrotask`) rather than running the task inside `add`, so several tasks added in the same tick are ordered by priority.
@@starter
class TaskQueue {
  constructor({ concurrency = 1 } = {}) {
    throw new Error("TODO: not done yet");
  }
}

@@tests
const sleep = (ms) => new Promise((r) => setTimeout(r, ms));
(async () => {
  const q = new TaskQueue({ concurrency: 2 });
  const order = [];
  let maxRunning = 0;
  q.on("start", ({ id }) => { order.push(id); maxRunning = Math.max(maxRunning, q.running); });
  const events = [];
  q.on("error", ({ id, error }) => events.push(`error ${id}: ${error.message}`));
  q.on("idle", () => events.push("idle"));
  const job = (ms, value) => async () => { await sleep(ms); return value; };
  const p1 = q.add(job(30, "a"));
  const p2 = q.add(job(30, "b"));
  const p3 = q.add(job(5, "c"), { priority: 0 });
  const p4 = q.add(job(5, "d"), { priority: 5 });
  const p5 = q.add(async () => { throw new Error("boom"); });
  check(q.pending, 5, "nothing starts synchronously");
  const results = await Promise.allSettled([p1, p2, p3, p4, p5]);
  check(results.map((r) => r.value ?? r.reason.message), ["a", "b", "c", "d", "boom"], "per-task results");
  check(order, [4, 1, 2, 3, 5], "priority first, then FIFO");
  check(maxRunning, 2, "concurrency limit");
  await q.onIdle();
  check(events, ["error 5: boom", "idle"], "events");
  q.pause();
  const p6 = q.add(job(1, "e"));
  await sleep(20);
  check([q.pending, q.running], [1, 0], "paused");
  q.resume();
  check(await p6, "e", "resumed");
  await q.onIdle();
  check([q.pending, q.running], [0, 0], "idle at the end");
})();
@@hint
Private fields: `#waiting = []`, `#running = 0`, `#seq = 0`, `#paused = false`, `#handlers = new Map()`, `#idleResolvers = []`. In `add`, return `new Promise((resolve, reject) => { this.#waiting.push({ id, priority, task, resolve, reject }); this.#waiting.sort((a, b) => b.priority - a.priority || a.id - b.id); queueMicrotask(() => this.#next()); })`. `#next()` starts tasks while `!this.#paused && this.#running < this.concurrency && this.#waiting.length`; each started task runs `Promise.resolve().then(item.task).then(...).catch(...).finally(() => { this.#running--; this.#next(); })`. When both counts are 0, emit "idle" and resolve the idle promises.
@@solution
class TaskQueue {
  #waiting = [];
  #running = 0;
  #seq = 0;
  #paused = false;
  #handlers = new Map();
  #idleResolvers = [];

  constructor({ concurrency = 1 } = {}) {
    this.concurrency = concurrency;
  }

  on(event, fn) {
    if (!this.#handlers.has(event)) this.#handlers.set(event, []);
    this.#handlers.get(event).push(fn);
    return this;
  }

  #emit(event, payload) {
    for (const fn of this.#handlers.get(event) ?? []) fn(payload);
  }

  get pending() {
    return this.#waiting.length;
  }

  get running() {
    return this.#running;
  }

  add(task, { priority = 0 } = {}) {
    const id = ++this.#seq;
    return new Promise((resolve, reject) => {
      this.#waiting.push({ id, priority, task, resolve, reject });
      this.#waiting.sort((a, b) => b.priority - a.priority || a.id - b.id);
      queueMicrotask(() => this.#next());
    });
  }

  pause() {
    this.#paused = true;
  }

  resume() {
    this.#paused = false;
    this.#next();
  }

  onIdle() {
    if (this.#running === 0 && this.#waiting.length === 0) return Promise.resolve();
    return new Promise((resolve) => this.#idleResolvers.push(resolve));
  }

  #next() {
    while (!this.#paused && this.#running < this.concurrency && this.#waiting.length > 0) {
      const item = this.#waiting.shift();
      this.#running++;
      this.#emit("start", { id: item.id });
      Promise.resolve()
        .then(item.task)
        .then(
          (result) => {
            this.#emit("done", { id: item.id, result });
            item.resolve(result);
          },
          (error) => {
            this.#emit("error", { id: item.id, error });
            item.reject(error);
          },
        )
        .finally(() => {
          this.#running--;
          this.#next();
          if (this.#running === 0 && this.#waiting.length === 0) {
            this.#emit("idle");
            for (const resolve of this.#idleResolvers.splice(0)) resolve();
          }
        });
    }
  }
}

@@exercise Timeouts and retries on top
Extend the queue **by composition**: write `withPolicy(task, { timeout, retries = 0, sleep })` that returns a new task function (no arguments, returns a promise) which:

- rejects with `Error("timeout after <ms> ms")` if one attempt takes longer than `timeout` (when given)
- retries a failed attempt up to `retries` more times, waiting `sleep(attempt * 10)` between attempts (`attempt` = 1, 2, ...)
- resolves with the first successful result, or rejects with the **last** error

Also write `runAll(queue, tasks, policy)` that adds every task (wrapped with `withPolicy`) to the queue and resolves to `{ ok: [...results], failed: [...error messages] }` in input order.

The test includes a minimal queue so this exercise stands alone.
@@starter
function withPolicy(task, { timeout, retries = 0, sleep = (ms) => new Promise((r) => setTimeout(r, ms)) } = {}) {
  throw new Error("TODO: not done yet");
}

async function runAll(queue, tasks, policy) {
  throw new Error("TODO: not done yet");
}

@@tests
class MiniQueue { add(task) { return task(); } }
const sleepReal = (ms) => new Promise((r) => setTimeout(r, ms));
(async () => {
  let n = 0;
  const flaky = async () => { n++; if (n < 3) throw new Error(`fail ${n}`); return "ok"; };
  const waits = [];
  check(await withPolicy(flaky, { retries: 2, sleep: async (ms) => waits.push(ms) })(), "ok", "retried until success");
  check(waits, [10, 20], "waits between attempts");
  let msg = "";
  try { await withPolicy(() => sleepReal(200).then(() => "late"), { timeout: 20 })(); } catch (e) { msg = e.message; }
  check(msg, "timeout after 20 ms", "timeout");
  n = -10;
  try { await withPolicy(flaky, { retries: 1, sleep: async () => {} })(); } catch (e) { msg = e.message; }
  check(msg, "fail -8", "last error");
  const summary = await runAll(new MiniQueue(), [async () => 1, async () => { throw new Error("x"); }, async () => 3], {});
  check(summary, { ok: [1, 3], failed: ["x"] }, "runAll");
})();
@@hint
Inside the returned function, loop `for (let attempt = 0; attempt <= retries; attempt++)`. Run one attempt with `timeout ? Promise.race([task(), timer]) : task()`, where `timer` rejects after `timeout` ms (clear it afterwards with `clearTimeout` so it doesn't keep the process alive). For `runAll`, `Promise.allSettled(tasks.map((t) => queue.add(withPolicy(t, policy))))`.
@@solution
function withPolicy(task, { timeout, retries = 0, sleep = (ms) => new Promise((r) => setTimeout(r, ms)) } = {}) {
  async function attemptOnce() {
    if (!timeout) return task();
    let timer;
    const limit = new Promise((_, reject) => {
      timer = setTimeout(() => reject(new Error(`timeout after ${timeout} ms`)), timeout);
    });
    try {
      return await Promise.race([task(), limit]);
    } finally {
      clearTimeout(timer);
    }
  }
  return async () => {
    let lastError;
    for (let attempt = 0; attempt <= retries; attempt++) {
      if (attempt > 0) await sleep(attempt * 10);
      try {
        return await attemptOnce();
      } catch (error) {
        lastError = error;
      }
    }
    throw lastError;
  };
}

async function runAll(queue, tasks, policy) {
  const settled = await Promise.allSettled(tasks.map((t) => queue.add(withPolicy(t, policy))));
  return {
    ok: settled.filter((s) => s.status === "fulfilled").map((s) => s.value),
    failed: settled.filter((s) => s.status === "rejected").map((s) => s.reason.message),
  };
}
