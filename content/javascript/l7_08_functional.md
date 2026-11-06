@@lesson l7-functional Functional Patterns: Composition, Currying, Memoisation, Debounce
@@concepts functions
@@text
Functions are values in JavaScript, so you can build small, reusable tools out of them. These patterns appear in every codebase and library: React hooks, lodash, RxJS, Express middleware.

## Pure functions

A **pure** function's result depends only on its arguments, and it has no side effects (no mutation, I/O or global state). Pure functions are trivial to test, cache and run in parallel. Keep the core logic pure and push I/O to the edges.

## Higher-order functions and composition

```javascript
const pipe = (...fns) => (x) => fns.reduce((acc, fn) => fn(acc), x);
const compose = (...fns) => (x) => fns.reduceRight((acc, fn) => fn(acc), x);

const normaliseHost = pipe(
  (s) => s.trim(),
  (s) => s.toLowerCase(),
  (s) => s.replace(/\.$/, ""),             // trailing dot from DNS names
);
console.log(normaliseHost("  WEB01.Corp.Local. "));
console.log(compose((x) => x + 1, (x) => x * 2)(5));   // (5*2)+1
```

`pipe` runs left to right (easier to read); `compose` right to left (the maths convention).

## Currying and partial application

**Currying** turns `f(a, b, c)` into `f(a)(b)(c)`. It's useful for building specialised functions from general ones:

```javascript
const curry = (fn) => function curried(...args) {
  return args.length >= fn.length ? fn(...args) : (...more) => curried(...args, ...more);
};
const log = curry((level, service, message) => `[${level}] ${service}: ${message}`);
const warnDb = log("WARN")("db");
console.log(warnDb("slow query"), log("INFO", "web", "started"));
```

## Memoisation

Cache results of an expensive pure function by its arguments:

```javascript
function memoize(fn, keyFn = (...args) => JSON.stringify(args)) {
  const cache = new Map();
  return (...args) => {
    const key = keyFn(...args);
    if (!cache.has(key)) cache.set(key, fn(...args));
    return cache.get(key);
  };
}
let calls = 0;
const slowSquare = (n) => { calls++; return n * n; };
const fastSquare = memoize(slowSquare);
console.log(fastSquare(9), fastSquare(9), "calls:", calls);
```

Unbounded caches grow forever: real ones limit size (an **LRU** cache evicts the least recently used entry) or expire entries after a **TTL**. Memoising an async function caches the **promise**, which also deduplicates concurrent requests.

## Debounce and throttle

Two ways to tame functions called too often (keystrokes, scroll, resize, file-watch events):

```text
events:    x x x x x       x x        x
debounce:            ──▶ call   ──▶ call  ──▶ call      (after a quiet period)
throttle:  call    call    call       call             (at most once per interval)
```

- **Debounce**: wait until calls stop for `ms`, then run once. Search-as-you-type, saving a draft, re-running a build after file changes.
- **Throttle**: run at most once per `ms`. Scroll position updates, progress reporting, rate-limited API calls.

```javascript
function debounce(fn, ms) {
  let timer;
  return (...args) => {
    clearTimeout(timer);
    timer = setTimeout(() => fn(...args), ms);
  };
}
const save = debounce((text) => console.log("saved:", text), 20);
save("h"); save("he"); save("hello");      // only "hello" is saved
```

## Immutability helpers

Functional style avoids mutating data (see the destructuring lesson): `map`/`filter`/`reduce`, spread, `toSorted`, `structuredClone`, and `Object.freeze` in development to catch accidental mutation.

## Point-free and readability

You can write `hosts.filter(isUp).map(toLabel)` instead of `hosts.filter((h) => isUp(h))`, which is often clearer. But beware passing functions that take extra arguments: `["1", "2", "3"].map(parseInt)` gives `[1, NaN, NaN]`, because `map` passes `(value, index)` and `parseInt` treats the index as the radix.

@@quiz
What does `pipe(f, g, h)(x)` compute?
* `h(g(f(x)))`
- `f(g(h(x)))`
- `[f(x), g(x), h(x)]`
- `f(x) + g(x) + h(x)`
? pipe applies left to right.

@@quiz
Which suits search-as-you-type that calls an API?
- Throttle with 0 ms
* Debounce, so the search runs once the user pauses typing
- Memoize
- Currying
? Throttle is for steady-rate updates like scroll handlers.

@@quiz
Why does `["1", "2", "3"].map(parseInt)` return `[1, NaN, NaN]`?
- parseInt is broken
* map passes the index as a second argument, which parseInt uses as the radix
- The strings have spaces
- map only works with arrow functions
? Use `.map((s) => parseInt(s, 10))` or `.map(Number)`.

@@drill
What's the name of the technique that delays a function until calls stop for a while?
= debounce
= debouncing
? Throttle runs at most once per interval instead.

@@flashcards
Pure function :: output depends only on input; no side effects
Higher-order function :: takes or returns functions
pipe :: compose left to right
Currying :: f(a, b) becomes f(a)(b)
Partial application :: fixing some arguments in advance
Memoisation :: caching results by arguments
LRU cache :: evicts the least recently used entry
Debounce :: run once after calls stop for a while
Throttle :: run at most once per interval

@@exercise An LRU cache with TTL
Write `class LruCache` with `constructor(maxSize, ttlMs = Infinity, now = () => Date.now())`:

- `get(key)`: the value, or `undefined` if missing or expired (an expired entry is removed); a successful get makes the entry the most recently used
- `set(key, value)`: stores it as the most recently used; if over `maxSize`, evicts the least recently used entry
- `size` getter and `keys()` (least to most recently used, as an array)

Then `memoizeAsync(fn, cache)` returning a function that caches **promises** by `JSON.stringify(args)`, so concurrent calls share one request; if the promise rejects, remove it from the cache so the next call retries.
@@starter
class LruCache {
  constructor(maxSize, ttlMs = Infinity, now = () => Date.now()) {
    throw new Error("TODO: not done yet");
  }
}

function memoizeAsync(fn, cache) {
  throw new Error("TODO: not done yet");
}

@@tests
let t = 0;
const c = new LruCache(2, 100, () => t);
c.set("a", 1); c.set("b", 2);
c.get("a");
c.set("c", 3);
check([c.keys(), c.get("b"), c.size], [["a", "c"], undefined, 2], "evicts least recently used");
t = 150;
check([c.get("a"), c.size], [undefined, 1], "expired entries vanish");
(async () => {
  let calls = 0;
  const lookup = memoizeAsync(async (host) => { calls++; if (host === "bad") throw new Error("nx"); return `${host}-ip`; }, new LruCache(10));
  const [x, y] = await Promise.all([lookup("web01"), lookup("web01")]);
  check([x, y, calls], ["web01-ip", "web01-ip", 1], "concurrent calls share one request");
  for (let i = 0; i < 2; i++) { try { await lookup("bad"); } catch {} }
  check(calls, 3, "failures are not cached");
})();
@@hint
A `Map` remembers insertion order, so it can be the LRU list: to "touch" a key, `delete` it and `set` it again; the first key (`map.keys().next().value`) is the least recently used. Store `{ value, expires: now() + ttl }`.
@@solution
class LruCache {
  constructor(maxSize, ttlMs = Infinity, now = () => Date.now()) {
    this.maxSize = maxSize;
    this.ttlMs = ttlMs;
    this.now = now;
    this.map = new Map();
  }

  get(key) {
    const entry = this.map.get(key);
    if (!entry) return undefined;
    this.map.delete(key);
    if (entry.expires <= this.now()) return undefined;
    this.map.set(key, entry);
    return entry.value;
  }

  set(key, value) {
    this.map.delete(key);
    this.map.set(key, { value, expires: this.now() + this.ttlMs });
    if (this.map.size > this.maxSize) this.map.delete(this.map.keys().next().value);
  }

  delete(key) {
    this.map.delete(key);
  }

  get size() {
    return this.map.size;
  }

  keys() {
    return [...this.map.keys()];
  }
}

function memoizeAsync(fn, cache) {
  return (...args) => {
    const key = JSON.stringify(args);
    const cached = cache.get(key);
    if (cached) return cached;
    const promise = fn(...args).catch((error) => {
      cache.delete(key);
      throw error;
    });
    cache.set(key, promise);
    return promise;
  };
}

@@exercise Debounce and throttle with a fake clock
Write `debounce(fn, ms, timers)` and `throttle(fn, ms, timers)` where `timers` provides `setTimeout`, `clearTimeout` and `now()` (so tests can control time).

- `debounce`: each call restarts the wait; `fn` runs `ms` after the **last** call, with the last call's arguments. The returned function also has `flush()` (run a pending call now) and `cancel()`.
- `throttle`: the first call runs immediately; further calls within `ms` of the last run are dropped, except that the **last** dropped call runs at the end of the interval (a "trailing" call) with its arguments.
@@starter
function debounce(fn, ms, timers) {
  throw new Error("TODO: not done yet");
}

function throttle(fn, ms, timers) {
  throw new Error("TODO: not done yet");
}

@@tests
function fakeTimers() {
  let now = 0, id = 0;
  const pending = new Map();
  return {
    now: () => now,
    setTimeout(fn, ms) { pending.set(++id, { at: now + ms, fn }); return id; },
    clearTimeout(i) { pending.delete(i); },
    advance(ms) {
      const end = now + ms;
      while (true) {
        const due = [...pending].filter(([, t]) => t.at <= end).sort((a, b) => a[1].at - b[1].at)[0];
        if (!due) break;
        pending.delete(due[0]); now = due[1].at; due[1].fn();
      }
      now = end;
    },
  };
}
let tm = fakeTimers();
const saved = [];
const save = debounce((v) => saved.push(v), 100, tm);
save("a"); tm.advance(50); save("ab"); tm.advance(50); save("abc"); tm.advance(99);
check(saved, [], "nothing yet");
tm.advance(1);
check(saved, ["abc"], "last value after quiet period");
save("x"); save.flush();
check(saved, ["abc", "x"], "flush");
save("y"); save.cancel(); tm.advance(500);
check(saved, ["abc", "x"], "cancel");
tm = fakeTimers();
const sent = [];
const report = throttle((p) => sent.push([p, tm.now()]), 100, tm);
report(1); tm.advance(10); report(2); tm.advance(10); report(3);
check(sent, [[1, 0]], "first runs immediately");
tm.advance(80);
check(sent, [[1, 0], [3, 100]], "trailing call with latest args");
tm.advance(200); report(4);
check(sent.at(-1), [4, 300], "runs immediately after a quiet period");
@@hint
Debounce: keep `timer` and `lastArgs`; each call clears the timer and sets a new one. `flush` clears it and runs `fn(...lastArgs)` if one was pending. Throttle: keep `lastRun = -Infinity`, `trailingArgs` and `timer`. On a call, if `now - lastRun >= ms` and no timer is pending, run now; otherwise store the args and, if no timer, schedule one for `lastRun + ms - now` that runs the stored args.
@@solution
function debounce(fn, ms, timers) {
  let timer = null;
  let lastArgs = null;
  const run = () => {
    timer = null;
    const args = lastArgs;
    lastArgs = null;
    fn(...args);
  };
  const debounced = (...args) => {
    lastArgs = args;
    if (timer !== null) timers.clearTimeout(timer);
    timer = timers.setTimeout(run, ms);
  };
  debounced.flush = () => {
    if (timer === null) return;
    timers.clearTimeout(timer);
    run();
  };
  debounced.cancel = () => {
    if (timer !== null) timers.clearTimeout(timer);
    timer = null;
    lastArgs = null;
  };
  return debounced;
}

function throttle(fn, ms, timers) {
  let lastRun = -Infinity;
  let timer = null;
  let trailingArgs = null;
  const invoke = (args) => {
    lastRun = timers.now();
    fn(...args);
  };
  return (...args) => {
    const now = timers.now();
    if (timer === null && now - lastRun >= ms) {
      invoke(args);
      return;
    }
    trailingArgs = args;
    if (timer === null) {
      timer = timers.setTimeout(() => {
        timer = null;
        const pending = trailingArgs;
        trailingArgs = null;
        invoke(pending);
      }, lastRun + ms - now);
    }
  };
}
