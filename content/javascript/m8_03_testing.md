@@lesson m8-testing Testing with node:test: Mocks, Timers and Coverage
@@concepts testing
@@text
Node.js has a complete test runner built in: `node:test` with `node:assert`. No Jest or Mocha needed for most projects, and it's fast.

## A test file

```javascript,norun
// test/slugify.test.js
const { test, describe, beforeEach } = require("node:test");
const assert = require("node:assert/strict");
const { slugify } = require("../src/slugify.js");

describe("slugify", () => {
  test("lower-cases and joins words", () => {
    assert.equal(slugify("VPN Down Again"), "vpn-down-again");
  });

  test("removes accents", () => {
    assert.equal(slugify("Café Réseau"), "cafe-reseau");
  });

  test("rejects empty input", () => {
    assert.throws(() => slugify(""), { name: "RangeError", message: /empty/ });
  });

  test.todo("handles emoji");
});
```

Run with `node --test` (finds `*.test.js`, `test/**` and similar), `node --test --watch` while working, and `node --test --experimental-test-coverage` for a coverage report.

## Assertions

| Assertion | Checks |
|---|---|
| `assert.equal(a, b)` | `a === b` (strict mode import) |
| `assert.deepEqual(a, b)` | same structure and values |
| `assert.ok(x)` | truthy |
| `assert.match(text, /re/)` | regex matches |
| `assert.throws(fn, expected)` | throws (expected: class, regex, or object of properties) |
| `await assert.rejects(promise, expected)` | promise rejects |

Always import `node:assert/strict`: the legacy loose versions use `==`.

## Async tests

Return a promise or use `async`; the test waits. A failing `await` fails the test:

```javascript,norun
test("fetches the status page", async () => {
  const status = await getStatus(fakeFetch);
  assert.deepEqual(status, { up: 3, down: 0 });
});
```

## Mocks and spies

`node:test` includes `mock`:

```javascript,norun
const { test, mock } = require("node:test");

test("sends an alert once", (t) => {
  const send = t.mock.fn(async () => ({ ok: true }));       // a spy with an implementation
  const monitor = new Monitor({ send });
  monitor.record("web01", "down");
  monitor.record("web01", "down");
  assert.equal(send.mock.callCount(), 1);
  assert.deepEqual(send.mock.calls[0].arguments, ["web01 is down"]);
});

test("uses the clock", (t) => {
  t.mock.timers.enable({ apis: ["setTimeout", "Date"], now: 0 });
  const done = t.mock.fn();
  setTimeout(done, 5000);
  t.mock.timers.tick(5000);                                   // no real waiting
  assert.equal(done.mock.callCount(), 1);
});

test("stubs a method", (t) => {
  t.mock.method(fs, "readFileSync", () => "host=db01");      // restored after the test
});
```

Mocks created through `t.mock` are restored automatically when the test ends.

## Design for testability

The best mocking strategy is needing fewer mocks:

- **Inject dependencies**: pass `fetch`, `now()`, `sleep()`, a logger or a DB object as parameters (as the exercises in this course do) instead of importing them deep inside.
- Keep **pure logic** separate from I/O, and test the logic directly.
- Test **behaviour**, not implementation details: assert on outputs and effects, not on which private helper was called.
- For HTTP code, start a real server on port `0` (a random free port) in `before()`, or use `undici`'s `MockAgent`.

## The test pyramid

```text
        /\        a few end-to-end tests (browser, whole system) — slow, brittle
       /  \
      /----\      some integration tests (real DB, real HTTP server)
     /      \
    /--------\    many unit tests (pure functions, classes) — fast, precise
```

Snapshot tests (`t.assert.snapshot`), property-based testing (`fast-check` generates hundreds of random inputs) and **mutation testing** (Stryker changes your code to see if tests notice) are useful additions for critical code.

@@quiz
Which import gives strict (===) assertions?
- `require("node:assert")`
* `require("node:assert/strict")`
- `require("assert-strict")`
- `require("node:test/assert")`
? The legacy non-strict functions use `==`.

@@quiz
How do you test code that waits 5 minutes with setTimeout, without waiting?
- Change the code to wait 0 ms
* Use mock timers (`t.mock.timers.enable()` and `tick()`) or inject a fake `sleep`
- Run the test overnight
- Skip it
? Controlling time keeps tests fast and deterministic.

@@quiz
What's the best way to make code easy to test?
* Pass dependencies (clock, fetch, logger) in as parameters and keep logic separate from I/O
- Use global variables
- Mock every function
- Write tests after release
? Dependency injection means fewer, simpler mocks.

@@flashcards
`node --test` :: run the built-in test runner
`describe` / `test` :: group and define tests
`assert.deepEqual` :: compare structure and values
`assert.rejects` :: assert that a promise rejects
`t.mock.fn()` :: create a spy function
`t.mock.timers` :: fake timers and Date
`t.mock.method(obj, name)` :: replace a method during a test
Dependency injection :: passing collaborators in instead of importing them
Test pyramid :: many unit, some integration, few end-to-end tests

@@exercise A tiny test runner
To understand test runners, build a small one. Write `createRunner()` returning `{ test, describe, run }`:

- `test(name, fn)` registers a test; `fn` may be async
- `describe(name, fn)` calls `fn` immediately; tests registered inside get the name prefix `"<describe name> > "` (nesting works)
- `run()` runs tests **in order**, one at a time, and resolves to `{ passed, failed, results }` where `results` is a list of `"✓ name"` or `"✗ name: <error message>"`
- a test fails if it throws or its promise rejects; a test that takes longer than 100 ms fails with the message `"timed out"`
@@starter
function createRunner() {
  throw new Error("TODO: not done yet");
}

@@tests
(async () => {
  const { test, describe, run } = createRunner();
  const order = [];
  test("adds", () => { order.push(1); if (1 + 1 !== 2) throw new Error("math"); });
  describe("parser", () => {
    test("fails", () => { order.push(2); throw new Error("bad token"); });
    describe("async", () => {
      test("resolves", async () => { await new Promise((r) => setTimeout(r, 5)); order.push(3); });
      test("hangs", () => new Promise(() => {}));
    });
  });
  test("rejects", async () => { throw new Error("nope"); });
  const summary = await run();
  check(summary, {
    passed: 2, failed: 3,
    results: ["✓ adds", "✗ parser > fails: bad token", "✓ parser > async > resolves", "✗ parser > async > hangs: timed out", "✗ rejects: nope"],
  }, "summary");
  check(order, [1, 2, 3], "ran in order");
})();
@@hint
Keep `const tests = []` and a `prefix` stack. `describe` pushes the name, calls `fn`, pops. `run` loops with `for (const t of tests)` and `await Promise.race([Promise.resolve().then(t.fn), timeout])`, where `timeout` rejects after 100 ms (clear the timer in `finally`).
@@solution
function createRunner() {
  const tests = [];
  const prefixes = [];

  function test(name, fn) {
    tests.push({ name: [...prefixes, name].join(" > "), fn });
  }

  function describe(name, fn) {
    prefixes.push(name);
    try {
      fn();
    } finally {
      prefixes.pop();
    }
  }

  async function run() {
    const results = [];
    let passed = 0;
    let failed = 0;
    for (const t of tests) {
      let timer;
      const timeout = new Promise((_, reject) => {
        timer = setTimeout(() => reject(new Error("timed out")), 100);
      });
      try {
        await Promise.race([Promise.resolve().then(t.fn), timeout]);
        passed++;
        results.push(`✓ ${t.name}`);
      } catch (error) {
        failed++;
        results.push(`✗ ${t.name}: ${error.message}`);
      } finally {
        clearTimeout(timer);
      }
    }
    return { passed, failed, results };
  }

  return { test, describe, run };
}

@@exercise Test it with injected dependencies
`createMonitor({ check, notify, now })` should track hosts and alert sensibly. Write it so it's fully testable through its parameters:

- `async poll(hosts)`: calls `check(host)` (a promise of true/false; a rejection counts as false) for every host in parallel
- a host is reported **down** only after **2 consecutive** failed polls; then `notify("<host> is down")` is awaited once (not again while it stays down)
- when a down host succeeds again, `notify("<host> recovered after <n> min")` where n is whole minutes since it went down (`now()` returns ms)
- `status()` returns an object mapping each host to `"up"` or `"down"`
@@starter
function createMonitor({ check, notify, now }) {
  throw new Error("TODO: not done yet");
}

@@tests
(async () => {
  let time = 0;
  const results = { web01: [true, false, false, false, true], db01: [false, true, true, true, true] };
  const sent = [];
  const monitor = createMonitor({
    check: async (h) => { const r = results[h].shift(); if (r === undefined) throw new Error("no data"); return r; },
    notify: async (msg) => { sent.push(msg); },
    now: () => time,
  });
  for (let i = 0; i < 5; i++) { await monitor.poll(["web01", "db01"]); time += 60000; }
  check(sent, ["web01 is down", "web01 recovered after 2 min"], "alerts");
  check(monitor.status(), { web01: "up", db01: "up" }, "status");
  await monitor.poll(["web01"]);
  await monitor.poll(["web01"]);
  check(monitor.status().web01, "down", "rejections count as failures");
})();
@@hint
Keep a `Map` of host → `{ failures, down, downSince }`. After `await Promise.all(hosts.map(async (h) => [h, await check(h).catch(() => false)]))`, update each host's state and collect messages; await `notify` for each.
@@solution
function createMonitor({ check, notify, now }) {
  const state = new Map();

  async function poll(hosts) {
    const results = await Promise.all(hosts.map(async (h) => [h, await check(h).catch(() => false)]));
    for (const [host, ok] of results) {
      const s = state.get(host) ?? { failures: 0, down: false, downSince: 0 };
      state.set(host, s);
      if (ok) {
        if (s.down) {
          const minutes = Math.floor((now() - s.downSince) / 60000);
          await notify(`${host} recovered after ${minutes} min`);
        }
        s.failures = 0;
        s.down = false;
      } else {
        s.failures++;
        if (s.failures >= 2 && !s.down) {
          s.down = true;
          s.downSince = now();
          await notify(`${host} is down`);
        }
      }
    }
  }

  function status() {
    return Object.fromEntries([...state].map(([h, s]) => [h, s.down ? "down" : "up"]));
  }

  return { poll, status };
}
