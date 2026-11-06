@@lesson l7-event-loop The Event Loop in Depth
@@concepts concurrency
@@text
JavaScript runs your code on **one thread**, yet a Node.js server handles thousands of connections at once. The **event loop** makes that possible, and knowing its exact order explains puzzling output, frozen pages and starved timers.

## The model

```text
          ┌───────────────────────────┐
          │      call stack           │  runs one function at a time, to completion
          └────────────┬──────────────┘
                       │ when the stack is empty:
                       ▼
   1. run ALL microtasks  (promise callbacks, await continuations, queueMicrotask)
   2. take ONE macrotask  (timer, I/O callback, UI event, setImmediate)
   3. (browser) maybe render a frame
   4. repeat
```

- Code runs **to completion**: nothing interrupts a function halfway. No locks are needed for ordinary variables.
- **Microtasks** run before anything else once the current code finishes, and new microtasks queued during that phase also run before the next macrotask.
- Slow operations (network, disk, timers) happen **outside** the JavaScript thread (in the OS or libuv's thread pool); only their callbacks come back through the queues.

## Predict the order

```javascript
console.log("1 sync");
setTimeout(() => console.log("6 timeout"), 0);
Promise.resolve().then(() => console.log("4 microtask"));
queueMicrotask(() => console.log("5 microtask 2"));
(async () => {
  console.log("2 async function starts synchronously");
  await null;
  console.log("4b after await (a microtask)");
})();
console.log("3 sync end");
```

Run it: all synchronous lines first, then the microtasks in the order they were queued, then the timer. `setTimeout(fn, 0)` means "at least 0 ms, after the current task and all microtasks".

## Node.js phases

Node's loop (libuv) has several macrotask phases per turn: **timers** (`setTimeout`/`setInterval`), **poll** (I/O callbacks), **check** (`setImmediate`), **close** callbacks. Node also has `process.nextTick`, which runs **before** promise microtasks:

```javascript
setImmediate(() => console.log("immediate"));
setTimeout(() => console.log("timeout"), 0);
Promise.resolve().then(() => console.log("promise"));
process.nextTick(() => console.log("nextTick"));
console.log("sync");
```

`nextTick` → promise → then timeout and immediate (their order is not guaranteed from the main module; inside an I/O callback, `setImmediate` always comes first).

## Blocking the loop

A long synchronous loop freezes **everything**: no requests answered, no timers fired, no clicks handled.

```javascript
const start = Date.now();
setTimeout(() => console.log(`timer fired after ${Date.now() - start} ms (asked for 10)`), 10);
let x = 0;
while (Date.now() - start < 100) x++;     // busy for 100 ms
console.log("busy loop done");
```

Fixes for CPU-heavy work:

1. **Chunk** it: process a slice, then `await new Promise(setImmediate)` (Node) or `setTimeout(0)` to let other work in.
2. Move it to a **worker thread** (`node:worker_threads`, Web Workers in browsers).
3. Avoid accidentally slow code: huge `JSON.parse` calls, synchronous `fs.readFileSync` in request handlers, catastrophic regex backtracking.

Measure with `perf_hooks.monitorEventLoopDelay()` in Node; a healthy server has delays of a few milliseconds.

## Microtask starvation

Because microtasks run until the queue is empty, a promise chain that keeps queueing more microtasks can starve timers and I/O forever. Recursive `process.nextTick` is the classic culprit.

## Async doesn't mean parallel

```javascript
const sleep = (ms) => new Promise((r) => setTimeout(r, ms));
async function main() {
  let t = Date.now();
  await sleep(50); await sleep(50);
  console.log("sequential ~", Math.round((Date.now() - t) / 50) * 50, "ms");
  t = Date.now();
  await Promise.all([sleep(50), sleep(50)]);
  console.log("concurrent ~", Math.round((Date.now() - t) / 50) * 50, "ms");
}
main();
```

Waiting overlaps (I/O happens in parallel outside JavaScript), but your JavaScript code itself never runs in parallel on one thread.

@@quiz
What runs first after the current synchronous code finishes?
- setTimeout callbacks
* All queued microtasks (promise callbacks, await continuations)
- setImmediate callbacks
- Rendering
? Then one macrotask, then microtasks again.

@@quiz
`setTimeout(f, 0)` is called, then a CPU loop runs for 200 ms. When does f run?
- After 0 ms
* After the loop finishes (about 200 ms), because callbacks only run when the stack is empty
- Never
- In parallel with the loop
? Timers give a minimum delay, not an exact one.

@@quiz
In Node.js, which runs first: `process.nextTick` or a resolved promise's `.then`?
* process.nextTick
- the promise callback
- whichever was queued first
- they run in parallel
? nextTick has its own queue, drained before promise microtasks.

@@quiz
How do you keep a CPU-heavy task from freezing a Node.js server?
- Make it async with the async keyword
* Split it into chunks that yield to the event loop, or run it in a worker thread
- Use more setTimeout calls inside the loop body
- Use var instead of let
? Marking a function async doesn't make its loops non-blocking.

@@drill
Which runs first: a resolved promise's `.then` callback or a `setTimeout(fn, 0)` callback?
= then
= .then
= the promise
= promise
= the then callback
? Microtasks run before the next macrotask.

@@flashcards
Event loop :: runs callbacks from queues when the call stack is empty
Run to completion :: a function is never interrupted by other JS code
Microtask :: promise callbacks, await continuations, queueMicrotask
Macrotask :: timers, I/O callbacks, UI events, setImmediate
`process.nextTick` :: Node queue drained before promise microtasks
`setImmediate` :: Node check-phase callback, after I/O
Blocking the loop :: long sync code freezes all other work
libuv :: Node's C library providing the loop and thread pool
Event loop delay :: how late callbacks run; a health metric

@@exercise Simulate the event loop
Write `simulate(program)` that models the queues. `program` is a list of steps for the main script; each step is one of:

- `["log", text]`: log immediately
- `["micro", steps]`: queue a microtask that runs `steps`
- `["timeout", ms, steps]`: queue a timer (macrotask) with a delay
- `["nextTick", steps]`: queue a nextTick callback (runs before microtasks)

Run the main steps, then loop: drain all nextTicks, then all microtasks (repeat both until both queues are empty), then run the timer with the smallest delay (ties in the order they were queued), and repeat. Steps inside callbacks can queue more work. Timer delays are relative to when the timer was queued: keep a virtual clock that jumps to each timer's due time. Return the list of logged texts.
@@starter
function simulate(program) {
  throw new Error("TODO: not done yet");
}

@@tests
check(simulate([
  ["log", "A"],
  ["timeout", 0, [["log", "T1"]]],
  ["micro", [["log", "M1"], ["micro", [["log", "M2"]]]]],
  ["nextTick", [["log", "N1"]]],
  ["log", "B"],
]), ["A", "B", "N1", "M1", "M2", "T1"], "basic order");
check(simulate([
  ["timeout", 10, [["log", "slow"]]],
  ["timeout", 5, [["log", "fast"], ["timeout", 1, [["log", "fast+1"]]], ["micro", [["log", "after fast"]]]]],
]), ["fast", "after fast", "fast+1", "slow"], "timers by due time");
check(simulate([["timeout", 0, [["log", "x"]]], ["timeout", 0, [["log", "y"]]]]), ["x", "y"], "ties keep order");
@@hint
Keep `ticks`, `micro`, `timers` (with `{ due, seq, steps }`), a `clock` and a `seq` counter. `run(steps)` loops over steps and pushes to the right queue (`due: clock + ms`). After the main program: `while (true) { while (ticks.length || micro.length) { if (ticks.length) run(ticks.shift()); else run(micro.shift()); } if (!timers.length) break; sort timers by due then seq; const t = timers.shift(); clock = t.due; run(t.steps); }`.
@@solution
function simulate(program) {
  const log = [];
  const ticks = [];
  const micro = [];
  const timers = [];
  let clock = 0;
  let seq = 0;

  function run(steps) {
    for (const step of steps) {
      const [kind] = step;
      if (kind === "log") log.push(step[1]);
      else if (kind === "micro") micro.push(step[1]);
      else if (kind === "nextTick") ticks.push(step[1]);
      else if (kind === "timeout") timers.push({ due: clock + step[1], seq: seq++, steps: step[2] });
    }
  }

  run(program);
  while (true) {
    while (ticks.length || micro.length) {
      if (ticks.length) run(ticks.shift());
      else run(micro.shift());
    }
    if (timers.length === 0) break;
    timers.sort((a, b) => a.due - b.due || a.seq - b.seq);
    const timer = timers.shift();
    clock = timer.due;
    run(timer.steps);
  }
  return log;
}

@@exercise Don't block the loop
`sumSquares(n)` below is CPU-heavy. Write `async function sumSquaresChunked(n, chunkSize, onProgress)` that computes the same result (the sum of i*i for i from 0 to n−1) but processes `chunkSize` numbers at a time and **yields to the event loop** between chunks (`await new Promise((r) => setImmediate(r))`), calling `onProgress(doneSoFar)` after each chunk. The test checks that a timer gets to run while the computation is in progress.
@@starter
function sumSquares(n) {
  let total = 0;
  for (let i = 0; i < n; i++) total += i * i;
  return total;
}

async function sumSquaresChunked(n, chunkSize, onProgress = () => {}) {
  throw new Error("TODO: not done yet");
}

@@tests
(async () => {
  const progress = [];
  let timerRanDuring = false;
  let finished = false;
  setTimeout(() => { if (!finished) timerRanDuring = true; }, 0);
  const result = await sumSquaresChunked(2_000_000, 100_000, (done) => progress.push(done));
  finished = true;
  check(result, sumSquares(2_000_000), "same result");
  check(progress.length, 20, "progress per chunk");
  check(progress.at(-1), 2_000_000, "last progress");
  check(timerRanDuring, true, "the event loop stayed responsive");
  check(await sumSquaresChunked(0, 10), 0, "empty");
})();
@@hint
`for (let start = 0; start < n; start += chunkSize) { const end = Math.min(n, start + chunkSize); for (let i = start; i < end; i++) total += i * i; onProgress(end); await new Promise((r) => setImmediate(r)); }`.
@@solution
function sumSquares(n) {
  let total = 0;
  for (let i = 0; i < n; i++) total += i * i;
  return total;
}

async function sumSquaresChunked(n, chunkSize, onProgress = () => {}) {
  let total = 0;
  for (let start = 0; start < n; start += chunkSize) {
    const end = Math.min(n, start + chunkSize);
    for (let i = start; i < end; i++) total += i * i;
    onProgress(end);
    await new Promise((resolve) => setImmediate(resolve));
  }
  return total;
}
