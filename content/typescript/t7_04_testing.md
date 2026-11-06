@@lesson t7-testing Testing TypeScript: Typed Fakes, Spies, Fake Clocks and Type Tests
@@concepts testing, functions
@@text
Types catch a whole class of bugs, but not wrong logic. Tests catch logic bugs, and TypeScript makes **test doubles** (fakes, stubs, spies) safer: if the real interface changes, the fake stops compiling.

## Test runners

- **Vitest**: Jest-compatible API, runs TS via esbuild, fast watch mode. The default for Vite projects and a good default overall.
- **node:test**: built into Node; with type stripping you can run `node --test` on `.ts` files with no dependencies.
- **Jest** (with ts-jest or swc): still common in older codebases.
- **Playwright**: end-to-end browser tests, written in TS.

```ts,norun
import { describe, it } from "node:test";
import assert from "node:assert/strict";
import { parsePort } from "./config.ts";

describe("parsePort", () => {
  it("accepts valid ports", () => assert.equal(parsePort("443"), 443));
  it("rejects text", () => assert.throws(() => parsePort("https"), /not a port/));
});
```

## Design for testability: depend on interfaces

Code that creates its own dependencies is hard to test. Pass them in, typed as small interfaces:

```ts
interface Clock { now(): number }
interface Mailer { send(to: string, subject: string): Promise<void> }

class CertMonitor {
  private readonly clock: Clock;
  private readonly mailer: Mailer;
  constructor(clock: Clock, mailer: Mailer) { this.clock = clock; this.mailer = mailer; }
  async check(host: string, expiresAt: number): Promise<boolean> {
    const days = (expiresAt - this.clock.now()) / 86_400_000;
    if (days < 14) { await this.mailer.send("ops@corp.example", `${host} cert expires in ${Math.floor(days)} days`); return true; }
    return false;
  }
}

const sent: string[] = [];
const monitor = new CertMonitor({ now: () => 0 }, { send: async (_to, subject) => { sent.push(subject); } });
monitor.check("web01", 5 * 86_400_000).then((warned) => console.log(warned, sent));
```

The fakes are plain objects. TypeScript checks they match `Clock` and `Mailer`, so no mocking library is needed.

## Spies

A **spy** wraps a function and records its calls. Typed spies keep the original signature:

```ts,norun
import { vi, expect } from "vitest";
const send = vi.fn<Mailer["send"]>(async () => {});
await monitor.check("web01", soon);
expect(send).toHaveBeenCalledWith("ops@corp.example", expect.stringContaining("web01"));
```

## Fake time

Code with timers and dates is flaky unless you control time. Vitest/Jest have `vi.useFakeTimers()`; `node:test` has `mock.timers`. Even better, inject a `Clock` as above.

## Type tests

For libraries and type-heavy code, test the **types** too:

- `expectTypeOf(fn).returns.toEqualTypeOf<number>()` in Vitest
- `tsd` or `tstyche` for `.d.ts` packages
- `// @ts-expect-error` lines prove that wrong usage **doesn't** compile (if it starts compiling, the directive itself errors)
- the `Expect<Equal<A, B>>` helpers you've been using

## What to test

- Pure functions: many small input/output cases (table-driven tests: `for (const [input, expected] of cases)`).
- Boundaries: parsing, validation, serialization, error paths.
- Use cases with fakes for I/O.
- A few end-to-end tests for critical flows. Don't chase 100% coverage of trivial code.

@@quiz
Why are typed fakes better than untyped mocks?
* If the real interface changes, the fake stops compiling, so tests can't silently drift
- They're faster
- They need a library
- They hide errors
? The compiler keeps test doubles honest.

@@quiz
What happens if code under a `// @ts-expect-error` line starts compiling cleanly?
- Nothing
* The directive itself becomes an error ("unused @ts-expect-error")
- The test is skipped
- The code is deleted
? That's what makes it useful for negative type tests.

@@quiz
What's the most robust way to test code that depends on the current time?
- Run tests at a fixed time of day
* Inject a clock (or use fake timers) so tests control time
- Add sleeps
- Avoid testing it
? Real time makes tests flaky.

@@drill
What's the name of a test double that records how it was called?
= spy
= a spy
? Check the recorded calls after the code runs.

@@flashcards
Vitest :: fast Jest-compatible test runner for TS
`node:test` :: Node's built-in test runner
Fake :: working lightweight implementation of an interface
Stub :: returns canned answers
Spy :: records calls to a function
Fake clock :: controllable time for timer/date code
Dependency injection :: pass dependencies in, typed as interfaces
`expectTypeOf` :: Vitest type assertions
`@ts-expect-error` :: asserts the next line has a type error
Table-driven test :: one loop over many input/expected pairs

@@exercise A typed spy
Write `spy<F extends (...args: any[]) => any>(impl: F): Spy<F>` where:

```ts,norun
type Spy<F extends (...args: any[]) => any> = ((...args: Parameters<F>) => ReturnType<F>) & {
  calls: Parameters<F>[];
  results: ({ ok: true; value: ReturnType<F> } | { ok: false; error: unknown })[];
  calledWith(...args: Parameters<F>): boolean;     // deep comparison with JSON.stringify
  reset(): void;
};
```

The spy calls `impl`, records arguments and outcomes (rethrowing errors), and `reset` clears the records. Then write `spyOn<T, K extends MethodKeys<T>>(obj: T, key: K): Spy<...>` that replaces `obj[key]` with a spy wrapping the original method (preserving `this`), and returns it.
@@starter
type Spy<F extends (...args: any[]) => any> = ((...args: Parameters<F>) => ReturnType<F>) & {
  calls: Parameters<F>[];
  results: ({ ok: true; value: ReturnType<F> } | { ok: false; error: unknown })[];
  calledWith(...args: Parameters<F>): boolean;
  reset(): void;
};
type MethodKeys<T> = { [K in keyof T]-?: T[K] extends (...args: any[]) => any ? K : never }[keyof T];
type MethodOf<T, K extends keyof T> = T[K] extends (...args: any[]) => any ? T[K] : never;

function spy<F extends (...args: any[]) => any>(impl: F): Spy<F> {
  throw new Error("TODO: not done yet");
}

function spyOn<T, K extends MethodKeys<T>>(obj: T, key: K): Spy<MethodOf<T, K>> {
  throw new Error("TODO: not done yet");
}

@@tests
const add = spy((a: number, b: number) => a + b);
check([add(1, 2), add(5, 5)], [3, 10], "passes through");
check(add.calls, [[1, 2], [5, 5]], "records calls");
check([add.calledWith(1, 2), add.calledWith(2, 1)], [true, false], "calledWith");
const boom = spy((msg: string): number => { throw new Error(msg); });
try { boom("disk"); } catch { /* expected */ }
check([boom.results[0]?.ok, (boom.results[0] as { error: Error }).error.message], [false, "disk"], "records errors");
add.reset();
check(add.calls.length, 0, "reset");
const pager = { sent: 0, page(team: string, sev: number): string { this.sent++; return `${team}:${sev}`; } };
const pageSpy = spyOn(pager, "page");
check([pager.page("net", 1), pager.sent], ["net:1", 1], "spyOn keeps this");
check(pageSpy.calls, [["net", 1]], "spyOn records");
@@hint
Create `const fn = function (this: unknown, ...args: Parameters<F>) { ... } as Spy<F>;` then assign `fn.calls = []` etc. Inside, push `args`, then `try { const value = impl.apply(this, args); results.push({ ok: true, value }); return value; } catch (error) { results.push({ ok: false, error }); throw error; }`. For `spyOn`, `const original = obj[key] as MethodOf<T, K>; const s = spy(original); (obj as Record<K, unknown>)[key] = s; return s;`.
@@solution
type Spy<F extends (...args: any[]) => any> = ((...args: Parameters<F>) => ReturnType<F>) & {
  calls: Parameters<F>[];
  results: ({ ok: true; value: ReturnType<F> } | { ok: false; error: unknown })[];
  calledWith(...args: Parameters<F>): boolean;
  reset(): void;
};
type MethodKeys<T> = { [K in keyof T]-?: T[K] extends (...args: any[]) => any ? K : never }[keyof T];
type MethodOf<T, K extends keyof T> = T[K] extends (...args: any[]) => any ? T[K] : never;

function spy<F extends (...args: any[]) => any>(impl: F): Spy<F> {
  const fn = function (this: unknown, ...args: Parameters<F>): ReturnType<F> {
    fn.calls.push(args);
    try {
      const value: ReturnType<F> = impl.apply(this, args);
      fn.results.push({ ok: true, value });
      return value;
    } catch (error) {
      fn.results.push({ ok: false, error });
      throw error;
    }
  } as Spy<F>;
  fn.calls = [];
  fn.results = [];
  fn.calledWith = (...args) => fn.calls.some((c) => JSON.stringify(c) === JSON.stringify(args));
  fn.reset = () => {
    fn.calls = [];
    fn.results = [];
  };
  return fn;
}

function spyOn<T, K extends MethodKeys<T>>(obj: T, key: K): Spy<MethodOf<T, K>> {
  const original = obj[key] as MethodOf<T, K>;
  const s = spy(original);
  (obj as Record<K, unknown>)[key] = s;
  return s;
}

@@exercise A fake clock for timer code
Write `class FakeClock implements Clock` with `interface Clock { now(): number; setTimeout(fn: () => void, ms: number): number; clearTimeout(id: number): void }`:

- time starts at 0; `setTimeout` returns increasing ids starting at 1
- `advance(ms: number)`: moves time forward, running due timers **in order of due time** (ties: in creation order); `now()` inside a callback returns that timer's due time; timers scheduled by callbacks also run if they fall due within the advanced window

Then test a `Debouncer` (given, using an injected `Clock`) through the fake clock.
@@starter
interface Clock {
  now(): number;
  setTimeout(fn: () => void, ms: number): number;
  clearTimeout(id: number): void;
}

class FakeClock implements Clock {
  now(): number {
    throw new Error("TODO: not done yet");
  }
  setTimeout(fn: () => void, ms: number): number {
    throw new Error("TODO: not done yet");
  }
  clearTimeout(id: number): void {
    throw new Error("TODO: not done yet");
  }
  advance(ms: number): void {
    throw new Error("TODO: not done yet");
  }
}

class Debouncer {
  private readonly clock: Clock;
  private readonly waitMs: number;
  private timer: number | null = null;
  constructor(clock: Clock, waitMs: number) { this.clock = clock; this.waitMs = waitMs; }
  call(fn: () => void): void {
    if (this.timer !== null) this.clock.clearTimeout(this.timer);
    this.timer = this.clock.setTimeout(() => { this.timer = null; fn(); }, this.waitMs);
  }
}

@@tests
const clock = new FakeClock();
const log: string[] = [];
clock.setTimeout(() => log.push(`b@${clock.now()}`), 200);
clock.setTimeout(() => log.push(`a@${clock.now()}`), 100);
const cancelled = clock.setTimeout(() => log.push("never"), 150);
clock.setTimeout(() => {
  log.push(`c@${clock.now()}`);
  clock.setTimeout(() => log.push(`d@${clock.now()}`), 50);
}, 100);
clock.clearTimeout(cancelled);
clock.advance(160);
check([log, clock.now()], [["a@100", "c@100", "d@150"], 160], "ordered, nested");
clock.advance(100);
check(log, ["a@100", "c@100", "d@150", "b@200"], "later timer");
const saves: number[] = [];
const debounced = new Debouncer(clock, 300);
debounced.call(() => saves.push(1));
clock.advance(200);
debounced.call(() => saves.push(2));
clock.advance(299);
check(saves, [], "not yet");
clock.advance(1);
check(saves, [2], "only the last call runs");
@@hint
Store timers as `{ id, due, fn }` in an array. In `advance`, compute `end = time + ms`, then loop: find the pending timer with the smallest `due` (then smallest `id`) where `due <= end`; if none, break; remove it, set `time = due`, run it. Finally set `time = end`.
@@solution
interface Clock {
  now(): number;
  setTimeout(fn: () => void, ms: number): number;
  clearTimeout(id: number): void;
}

class FakeClock implements Clock {
  private time = 0;
  private nextId = 1;
  private timers: { id: number; due: number; fn: () => void }[] = [];

  now(): number {
    return this.time;
  }

  setTimeout(fn: () => void, ms: number): number {
    const id = this.nextId++;
    this.timers.push({ id, due: this.time + ms, fn });
    return id;
  }

  clearTimeout(id: number): void {
    this.timers = this.timers.filter((t) => t.id !== id);
  }

  advance(ms: number): void {
    const end = this.time + ms;
    for (;;) {
      const due = this.timers
        .filter((t) => t.due <= end)
        .sort((a, b) => a.due - b.due || a.id - b.id)[0];
      if (!due) break;
      this.clearTimeout(due.id);
      this.time = due.due;
      due.fn();
    }
    this.time = end;
  }
}

class Debouncer {
  private readonly clock: Clock;
  private readonly waitMs: number;
  private timer: number | null = null;
  constructor(clock: Clock, waitMs: number) { this.clock = clock; this.waitMs = waitMs; }
  call(fn: () => void): void {
    if (this.timer !== null) this.clock.clearTimeout(this.timer);
    this.timer = this.clock.setTimeout(() => { this.timer = null; fn(); }, this.waitMs);
  }
}
