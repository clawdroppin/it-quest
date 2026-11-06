@@lesson t7-errors Typed Error Handling: Result Types and Error Unions
@@concepts errors, functions
@@text
In TypeScript, `throw` is invisible to the type system: a function's signature doesn't say what it can throw, and `catch (e)` gives you `unknown`. For **expected** failures (validation, "not found", conflicts, network timeouts) many teams make errors part of the return type instead.

## Two kinds of failure

- **Expected failures** are part of the domain: the user typed a bad port, the device is already assigned, the API returned 404. The caller should handle them, so put them in the type.
- **Bugs and disasters** (null dereference, out of memory, a broken invariant) can't be handled meaningfully near where they happen. Let them throw and catch them at the top level (log, return 500, restart).

## The Result type

```ts
type Result<T, E> = { ok: true; value: T } | { ok: false; error: E };
const ok = <T>(value: T): Result<T, never> => ({ ok: true, value });
const err = <E>(error: E): Result<never, E> => ({ ok: false, error });

type ParseError = { kind: "empty" } | { kind: "not-a-number"; input: string } | { kind: "out-of-range"; value: number };

function parsePort(input: string): Result<number, ParseError> {
  const t = input.trim();
  if (t === "") return err({ kind: "empty" });
  const n = Number(t);
  if (!Number.isInteger(n)) return err({ kind: "not-a-number", input: t });
  if (n < 1 || n > 65535) return err({ kind: "out-of-range", value: n });
  return ok(n);
}

const r = parsePort("70000");
if (r.ok) console.log("port", r.value);
else console.log("failed:", r.error.kind);
```

Using `never` for the unused side means `ok(5)` fits any `Result<number, E>`.

## Errors as discriminated unions

Each error case has a `kind`, so callers can handle each one **exhaustively**, and the compiler tells them when a new case is added:

```ts
type ParseError = { kind: "empty" } | { kind: "not-a-number"; input: string } | { kind: "out-of-range"; value: number };
function message(e: ParseError): string {
  switch (e.kind) {
    case "empty": return "Please enter a port.";
    case "not-a-number": return `"${e.input}" isn't a number.`;
    case "out-of-range": return `${e.value} is outside 1-65535.`;
  }
}
console.log(message({ kind: "out-of-range", value: 0 }));
```

## Combining results

Chaining many fallible steps with `if (!r.ok) return r;` is clear and is what most code does. Helpers make pipelines shorter:

```ts
type Result<T, E> = { ok: true; value: T } | { ok: false; error: E };
const map = <T, U, E>(r: Result<T, E>, f: (v: T) => U): Result<U, E> => (r.ok ? { ok: true, value: f(r.value) } : r);
const andThen = <T, U, E, F>(r: Result<T, E>, f: (v: T) => Result<U, F>): Result<U, E | F> => (r.ok ? f(r.value) : r);
const r1 = map<number, string, never>({ ok: true, value: 21 }, (n) => `port ${n * 2}`);
console.log(r1);
```

Note how `andThen` **unions** the error types: the result of a pipeline can fail in any of the ways its steps can. Libraries like `neverthrow` and Effect build on this idea.

## Typed exceptions when you do throw

If you throw, throw **Error subclasses** with a discriminating property, and narrow in `catch`:

```ts
class NotFoundError extends Error {
  readonly kind = "not-found";
  readonly resource: string;
  constructor(resource: string) { super(`${resource} not found`); this.name = "NotFoundError"; this.resource = resource; }
}
try {
  throw new NotFoundError("device IT-9");
} catch (e) {
  if (e instanceof NotFoundError) console.log("404:", e.resource);
  else throw e;                              // never swallow unknown errors
}
```

Also use `new Error("message", { cause: e })` when wrapping lower-level errors, so the original stack isn't lost.

## Guidelines

- Return `Result` for expected, recoverable failures in your domain and at boundaries (parsing, validation).
- Throw for bugs and truly exceptional situations.
- Convert at the edges: wrap throwing third-party calls in a function that returns `Result` (`tryCatch`), and turn `Result` errors into HTTP responses in one place.
- Don't mix styles randomly in one layer.

@@quiz
Why put expected failures in the return type instead of throwing?
* The type tells callers what can go wrong and the compiler makes them handle it
- Throwing is slower
- TypeScript can't throw
- It hides errors
? `throw` isn't visible in a function's signature.

@@quiz
`andThen(r: Result<T, E>, f: (v: T) => Result<U, F>)` returns `Result<U, E | F>`. Why the union?
- It's a mistake
* The pipeline can fail with either step's error
- Only F is possible
- Errors are merged into one object
? Error types accumulate along a chain.

@@quiz
What should you do with an unrecognized error in `catch (e)`?
- Ignore it
- Log and continue
* Rethrow it (or let it reach a top-level handler)
- Convert it to null
? Never silently swallow errors you don't understand.

@@drill
Which type holds either a success value or an error?
= Result
= Result<T, E>
? { ok: true, value } | { ok: false, error }.

@@flashcards
`Result<T, E>` :: `{ ok: true; value: T } | { ok: false; error: E }`
Expected failure :: domain error the caller should handle
Bug :: unexpected failure; let it throw
Error union :: errors as a discriminated union with `kind`
`map` :: transform the success value
`andThen` / `flatMap` :: chain a fallible step
`tryCatch` :: convert a throwing call into a Result
`Error` `cause` :: keep the original error when wrapping
Error subclass :: `class NotFoundError extends Error`

@@exercise Result helpers
Write:

- `ok`, `err` (as in the lesson)
- `map<T, U, E>(r, f)`, `mapErr<T, E, F>(r, f)`, `andThen<T, U, E, F>(r, f)`
- `tryCatch<T>(f: () => T): Result<T, Error>`: non-`Error` throws are wrapped as `new Error(String(thrown))`
- `all<T, E>(results: Result<T, E>[]): Result<T[], E[]>`: all values if every result is ok, otherwise **every** error
- `unwrapOr<T, E>(r: Result<T, E>, fallback: T): T`
@@starter
type Result<T, E> = { ok: true; value: T } | { ok: false; error: E };

function ok<T>(value: T): Result<T, never> {
  throw new Error("TODO: not done yet");
}
function err<E>(error: E): Result<never, E> {
  throw new Error("TODO: not done yet");
}
function map<T, U, E>(r: Result<T, E>, f: (v: T) => U): Result<U, E> {
  throw new Error("TODO: not done yet");
}
function mapErr<T, E, F>(r: Result<T, E>, f: (e: E) => F): Result<T, F> {
  throw new Error("TODO: not done yet");
}
function andThen<T, U, E, F>(r: Result<T, E>, f: (v: T) => Result<U, F>): Result<U, E | F> {
  throw new Error("TODO: not done yet");
}
function tryCatch<T>(f: () => T): Result<T, Error> {
  throw new Error("TODO: not done yet");
}
function all<T, E>(results: Result<T, E>[]): Result<T[], E[]> {
  throw new Error("TODO: not done yet");
}
function unwrapOr<T, E>(r: Result<T, E>, fallback: T): T {
  throw new Error("TODO: not done yet");
}

@@tests
const parse = (s: string): Result<number, string> => (/^\d+$/.test(s) ? ok(Number(s)) : err(`bad number: ${s}`));
const positive = (n: number): Result<number, "zero"> => (n > 0 ? ok(n) : err("zero"));
check(map(parse("21"), (n) => n * 2), { ok: true, value: 42 }, "map");
check(mapErr(parse("x"), (e) => e.toUpperCase()), { ok: false, error: "BAD NUMBER: X" }, "mapErr");
check([andThen(parse("0"), positive), andThen(parse("5"), positive)], [{ ok: false, error: "zero" }, { ok: true, value: 5 }], "andThen");
const j = tryCatch(() => JSON.parse("{") as unknown);
check(j.ok ? "" : j.error.name, "SyntaxError", "tryCatch Error");
const weird = tryCatch((): number => { throw "plain string"; });
check(weird.ok ? "" : weird.error.message, "plain string", "non-Error throw wrapped");
check(all(["1", "2"].map(parse)), { ok: true, value: [1, 2] }, "all ok");
check(all(["1", "a", "b"].map(parse)), { ok: false, error: ["bad number: a", "bad number: b"] }, "all errors");
check([unwrapOr(parse("7"), 0), unwrapOr(parse("?"), 0)], [7, 0], "unwrapOr");
@@hint
`map`: `r.ok ? ok(f(r.value)) : r`. Returning `r` unchanged works because in the error branch TypeScript narrows it to `{ ok: false; error: E }`, which fits `Result<U, E>`. In `tryCatch`, `catch (e) { return err(e instanceof Error ? e : new Error(String(e))); }`.
@@solution
type Result<T, E> = { ok: true; value: T } | { ok: false; error: E };

function ok<T>(value: T): Result<T, never> {
  return { ok: true, value };
}
function err<E>(error: E): Result<never, E> {
  return { ok: false, error };
}
function map<T, U, E>(r: Result<T, E>, f: (v: T) => U): Result<U, E> {
  return r.ok ? ok(f(r.value)) : r;
}
function mapErr<T, E, F>(r: Result<T, E>, f: (e: E) => F): Result<T, F> {
  return r.ok ? r : err(f(r.error));
}
function andThen<T, U, E, F>(r: Result<T, E>, f: (v: T) => Result<U, F>): Result<U, E | F> {
  return r.ok ? f(r.value) : r;
}
function tryCatch<T>(f: () => T): Result<T, Error> {
  try {
    return ok(f());
  } catch (e) {
    return err(e instanceof Error ? e : new Error(String(e)));
  }
}
function all<T, E>(results: Result<T, E>[]): Result<T[], E[]> {
  const values: T[] = [];
  const errors: E[] = [];
  for (const r of results) {
    if (r.ok) values.push(r.value);
    else errors.push(r.error);
  }
  return errors.length > 0 ? err(errors) : ok(values);
}
function unwrapOr<T, E>(r: Result<T, E>, fallback: T): T {
  return r.ok ? r.value : fallback;
}

@@exercise Assigning devices with typed errors
Write `assignDevice(store, serial, userId, today)` returning `Result<Assignment, AssignError>` where:

```ts,norun
type AssignError =
  | { kind: "device-not-found"; serial: string }
  | { kind: "user-not-found"; userId: string }
  | { kind: "already-assigned"; serial: string; to: string }
  | { kind: "limit-reached"; userId: string; limit: number };
```

Rules: the device and user must exist; the device must be unassigned; a user may hold at most `user.limit` devices. On success, set `device.assignedTo` and return `{ serial, userId, date: today }`. Check in this order: device, user, already assigned, limit.

Then write `httpStatus(e: AssignError): number` (404 / 404 / 409 / 422) and `describeError(e: AssignError): string` using an exhaustive `switch` with `assertNever`.
@@starter
type Result<T, E> = { ok: true; value: T } | { ok: false; error: E };
type AssignError =
  | { kind: "device-not-found"; serial: string }
  | { kind: "user-not-found"; userId: string }
  | { kind: "already-assigned"; serial: string; to: string }
  | { kind: "limit-reached"; userId: string; limit: number };
interface Device { serial: string; assignedTo: string | null }
interface User { id: string; limit: number }
interface Store { devices: Device[]; users: User[] }
interface Assignment { serial: string; userId: string; date: string }

function assignDevice(store: Store, serial: string, userId: string, today: string): Result<Assignment, AssignError> {
  throw new Error("TODO: not done yet");
}

function httpStatus(e: AssignError): number {
  throw new Error("TODO: not done yet");
}

function describeError(e: AssignError): string {
  throw new Error("TODO: not done yet");
}

@@tests
const store: Store = {
  devices: [{ serial: "LT-1", assignedTo: null }, { serial: "LT-2", assignedTo: "bo" }, { serial: "PH-1", assignedTo: null }],
  users: [{ id: "ana", limit: 1 }, { id: "bo", limit: 2 }],
};
check(assignDevice(store, "LT-1", "ana", "2026-11-03"), { ok: true, value: { serial: "LT-1", userId: "ana", date: "2026-11-03" } }, "assigned");
check(store.devices[0]?.assignedTo, "ana", "store updated");
const results = [
  assignDevice(store, "XX-9", "ana", "d"),
  assignDevice(store, "PH-1", "zed", "d"),
  assignDevice(store, "LT-2", "ana", "d"),
  assignDevice(store, "PH-1", "ana", "d"),
];
const errors = results.map((r) => (r.ok ? null : r.error));
check(errors.map((e) => e?.kind), ["device-not-found", "user-not-found", "already-assigned", "limit-reached"], "error kinds");
check(errors.map((e) => (e ? httpStatus(e) : 0)), [404, 404, 409, 422], "statuses");
check(errors.map((e) => (e ? describeError(e) : "")), [
  "No device with serial XX-9",
  "No user zed",
  "LT-2 is already assigned to bo",
  "ana already has 1 device(s), the limit",
], "messages");
@@hint
Count a user's devices with `store.devices.filter((d) => d.assignedTo === userId).length`. For `describeError`, end the switch with `default: return assertNever(e);` where `function assertNever(x: never): never { throw new Error("unexpected " + JSON.stringify(x)); }`.
@@solution
type Result<T, E> = { ok: true; value: T } | { ok: false; error: E };
type AssignError =
  | { kind: "device-not-found"; serial: string }
  | { kind: "user-not-found"; userId: string }
  | { kind: "already-assigned"; serial: string; to: string }
  | { kind: "limit-reached"; userId: string; limit: number };
interface Device { serial: string; assignedTo: string | null }
interface User { id: string; limit: number }
interface Store { devices: Device[]; users: User[] }
interface Assignment { serial: string; userId: string; date: string }

function assertNever(x: never): never {
  throw new Error(`unexpected ${JSON.stringify(x)}`);
}

function assignDevice(store: Store, serial: string, userId: string, today: string): Result<Assignment, AssignError> {
  const device = store.devices.find((d) => d.serial === serial);
  if (!device) return { ok: false, error: { kind: "device-not-found", serial } };
  const user = store.users.find((u) => u.id === userId);
  if (!user) return { ok: false, error: { kind: "user-not-found", userId } };
  if (device.assignedTo !== null) return { ok: false, error: { kind: "already-assigned", serial, to: device.assignedTo } };
  const held = store.devices.filter((d) => d.assignedTo === userId).length;
  if (held >= user.limit) return { ok: false, error: { kind: "limit-reached", userId, limit: user.limit } };
  device.assignedTo = userId;
  return { ok: true, value: { serial, userId, date: today } };
}

function httpStatus(e: AssignError): number {
  switch (e.kind) {
    case "device-not-found":
    case "user-not-found":
      return 404;
    case "already-assigned":
      return 409;
    case "limit-reached":
      return 422;
    default:
      return assertNever(e);
  }
}

function describeError(e: AssignError): string {
  switch (e.kind) {
    case "device-not-found":
      return `No device with serial ${e.serial}`;
    case "user-not-found":
      return `No user ${e.userId}`;
    case "already-assigned":
      return `${e.serial} is already assigned to ${e.to}`;
    case "limit-reached":
      return `${e.userId} already has ${e.limit} device(s), the limit`;
    default:
      return assertNever(e);
  }
}
