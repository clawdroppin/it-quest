@@lesson j1-loops Loop Patterns and Iteration
@@concepts loops, lists
@@text
JavaScript has more loop forms than most languages. Picking the right one makes code shorter and avoids classic bugs.

| Loop | Use it for |
|---|---|
| `for (const x of iterable)` | values of arrays, strings, maps, sets: the everyday loop |
| `for (const key in object)` | an object's enumerable keys (including inherited ones!) |
| `array.forEach(fn)` | side effects per element; can't `break` |
| `for (let i = 0; i < n; i++)` | when you need the index or a custom step |
| `while (cond)` / `do { } while (cond)` | repeat until something changes (retries, polling) |

```javascript
const hosts = ["web01", "db01"];
for (const h of hosts) console.log("of:", h);
for (const [i, h] of hosts.entries()) console.log("entries:", i, h);
for (const ch of "IT✓") console.log("char:", ch);
const ports = { http: 80, https: 443 };
for (const key in ports) console.log("in:", key, ports[key]);
for (const [name, port] of Object.entries(ports)) console.log("entries:", name, port);
```

- **Don't use `for...in` on arrays**: it gives string indexes, in an order that isn't guaranteed in old engines, and includes extra properties.
- `Object.keys`, `Object.values` and `Object.entries` turn an object into arrays you can loop with `for...of`.

## break, continue and labels

```javascript
const racks = [["ok", "ok"], ["ok", "FAIL", "ok"], ["ok"]];
search: for (const [r, rack] of racks.entries()) {
  for (const [s, slot] of rack.entries()) {
    if (slot === "FAIL") {
      console.log(`first failure: rack ${r} slot ${s}`);
      break search;                       // leaves both loops
    }
  }
}
```

A **label** lets `break` or `continue` target an outer loop. Use it sparingly; extracting a function and returning is often clearer.

## Loops that wait

`forEach` ignores `await`; use `for...of` when each step must finish first:

```javascript
const sleep = (ms) => new Promise((r) => setTimeout(r, ms));
async function main() {
  for (const step of ["stop service", "copy files", "start service"]) {
    await sleep(10);
    console.log("done:", step);
  }
}
main();
```

To run steps **in parallel** instead, use `await Promise.all(items.map(async (x) => ...))`.

## Common patterns

```javascript
// retry with a while loop
let attempt = 0;
let ok = false;
while (!ok && attempt < 5) {
  attempt++;
  ok = attempt === 3;                    // pretend the 3rd try works
}
console.log("succeeded after", attempt, "attempts");

// build ranges
const range = (start, end, step = 1) => Array.from({ length: Math.ceil((end - start) / step) }, (_, i) => start + i * step);
console.log(range(0, 10, 3), range(5, 8));

// chunk a list into batches
const batch = (items, size) => Array.from({ length: Math.ceil(items.length / size) }, (_, i) => items.slice(i * size, i * size + size));
console.log(batch(["a", "b", "c", "d", "e"], 2));
```

`Array.from({ length: n }, (_, i) => ...)` is the idiomatic way to build an array of `n` computed values.

@@quiz
Which loop gives you the values of an array?
* `for (const x of arr)`
- `for (const x in arr)`
- `for (x : arr)`
- `arr.each(x)`
? `for...in` gives keys (as strings), not values.

@@quiz
Why does `items.forEach(async (x) => { await save(x); })` not wait for the saves?
* forEach ignores the promises its callback returns, so the loop finishes immediately
- async functions can't be used as callbacks
- save is never called
- It does wait
? Use `for...of` with `await`, or `await Promise.all(items.map(save))`.

@@quiz
What does `break outer;` do?
- Stops the program
* Leaves the loop labelled `outer`, even from inside a nested loop
- Skips to the next iteration of the inner loop
- Throws an error
? Labels let break and continue target an outer loop.

@@flashcards
`for...of` :: loop over values of an iterable
`for...in` :: loop over enumerable keys of an object
`Object.entries(obj)` :: array of [key, value] pairs
`arr.entries()` :: iterator of [index, value]
Label :: a name for a loop, used with break/continue
`Array.from({ length: n }, fn)` :: build an array of n computed values
Sequential await :: `for...of` with `await` inside
Parallel await :: `await Promise.all(items.map(fn))`

@@exercise Batches with retries
Write `async function processInBatches(items, size, worker, maxAttempts = 3)`:

- split `items` into batches of `size`; run each batch **in parallel** (`Promise.all`), and the batches **one after another**
- each item is processed by `worker(item)` (returns a promise); if it rejects, retry that item up to `maxAttempts` attempts in total
- return `{ done: [...], failed: [...] }` with items in input order
@@starter
async function processInBatches(items, size, worker, maxAttempts = 3) {
  throw new Error("TODO: not done yet");
}

@@tests
(async () => {
  const tries = {};
  const running = { now: 0, max: 0 };
  const worker = async (x) => {
    running.now++; running.max = Math.max(running.max, running.now);
    await new Promise((r) => setTimeout(r, 5));
    running.now--;
    tries[x] = (tries[x] ?? 0) + 1;
    if (x === "flaky" && tries[x] < 2) throw new Error("try again");
    if (x === "dead") throw new Error("down");
    return x;
  };
  const result = await processInBatches(["a", "flaky", "dead", "b", "c"], 2, worker);
  check(result, { done: ["a", "flaky", "b", "c"], failed: ["dead"] }, "result");
  check(tries.dead, 3, "dead retried 3 times");
  check(running.max, 2, "at most one batch at a time");
})();
@@hint
Loop `for (let i = 0; i < items.length; i += size)` over `items.slice(i, i + size)`. Write a helper `async function attempt(item)` that loops up to `maxAttempts` with `try { await worker(item); return true; } catch {}` and returns false at the end. Then `const results = await Promise.all(batch.map(attempt));`.
@@solution
async function processInBatches(items, size, worker, maxAttempts = 3) {
  const done = [];
  const failed = [];
  async function attempt(item) {
    for (let n = 1; n <= maxAttempts; n++) {
      try {
        await worker(item);
        return true;
      } catch {
        // try again
      }
    }
    return false;
  }
  for (let i = 0; i < items.length; i += size) {
    const batch = items.slice(i, i + size);
    const results = await Promise.all(batch.map(attempt));
    batch.forEach((item, j) => (results[j] ? done : failed).push(item));
  }
  return { done, failed };
}

@@exercise Walk a nested structure
Config files nest objects and arrays. Write `flatten(obj)` that returns an object mapping **dotted paths** to leaf values: `{ db: { hosts: ["a", "b"] } }` → `{ "db.hosts.0": "a", "db.hosts.1": "b" }`. Empty objects and arrays are leaves too (kept as `{}` / `[]`). Use loops (or recursion), not a library.
@@starter
function flatten(obj) {
  throw new Error("TODO: not done yet");
}

@@tests
check(flatten({ db: { host: "db01", ports: [5432, 5433] }, debug: false }), { "db.host": "db01", "db.ports.0": 5432, "db.ports.1": 5433, debug: false }, "nested");
check(flatten({ a: {}, b: [], c: null }), { a: {}, b: [], c: null }, "empty containers and null");
check(flatten({}), {}, "empty");
@@hint
Write `walk(value, path)`: if `value` is a non-null object with at least one key, loop `for (const [k, v] of Object.entries(value)) walk(v, path ? `${path}.${k}` : k)`; otherwise `out[path] = value`.
@@solution
function flatten(obj) {
  const out = {};
  function walk(value, path) {
    const isContainer = typeof value === "object" && value !== null;
    if (isContainer && Object.keys(value).length > 0) {
      for (const [k, v] of Object.entries(value)) walk(v, path ? `${path}.${k}` : k);
    } else if (path) {
      out[path] = value;
    }
  }
  walk(obj, "");
  return out;
}
