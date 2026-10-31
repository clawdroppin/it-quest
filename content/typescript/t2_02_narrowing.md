@@lesson t2-narrowing Narrowing and Discriminated Unions
@@concepts conditions, errors
@@text
## Narrowing

With a union type, TypeScript only lets you use what's valid for *every* member, until you check which one you have. Each check **narrows** the type:

```typescript
function describe(x: string | number | string[] | null): string {
  if (x === null) return "nothing";
  if (typeof x === "string") return `text "${x}"`;          // x: string
  if (typeof x === "number") return `number ${x.toFixed(1)}`;// x: number
  return `list of ${x.length}`;                              // x: string[] (all that's left)
}
console.log(describe("a"), describe(2), describe(["a", "b"]), describe(null));
```

Checks TypeScript understands: `typeof`, `===`/`!==`, truthiness (`if (x)`), `Array.isArray(x)`, `x instanceof Date`, and `"prop" in x`.

## Discriminated unions

The most useful pattern in TypeScript. Give each variant a **tag** property with a literal type; checking the tag narrows to that variant:

```typescript
type CheckResult =
  | { kind: "ok"; latencyMs: number }
  | { kind: "timeout"; afterMs: number }
  | { kind: "error"; code: number; message: string };

function report(r: CheckResult): string {
  switch (r.kind) {
    case "ok":
      return `up (${r.latencyMs} ms)`;
    case "timeout":
      return `no answer after ${r.afterMs} ms`;
    case "error":
      return `HTTP ${r.code}: ${r.message}`;   // only "error" has code and message
  }
}

console.log(report({ kind: "ok", latencyMs: 42 }));
console.log(report({ kind: "error", code: 503, message: "Service Unavailable" }));
```

Impossible combinations, like a timeout with an HTTP code, can't even be written. This is how you model API responses, events, messages and state machines.

## Exhaustiveness checking with never

What if someone adds a new `kind` later? Make the compiler find every `switch` that doesn't handle it:

```typescript
type Shape = { kind: "vm"; vcpus: number } | { kind: "container"; image: string } | { kind: "function"; runtime: string };

function assertNever(x: never): never {
  throw new Error(`unhandled: ${JSON.stringify(x)}`);
}

function cost(s: Shape): number {
  switch (s.kind) {
    case "vm": return s.vcpus * 30;
    case "container": return 10;
    case "function": return 2;
    default: return assertNever(s);   // if a case is missing, s isn't `never` here: compile error
  }
}
console.log(cost({ kind: "vm", vcpus: 4 }));
```

`never` is the type with no values. After all cases are handled, nothing is left, so `s` is `never`.

## Type guards: your own checks

A function returning `x is T` tells TypeScript what a `true` result means. This is how you safely turn `unknown` data (JSON, API responses) into typed values:

```typescript
type User = { name: string; enabled: boolean };

function isUser(x: unknown): x is User {
  return (
    typeof x === "object" && x !== null &&
    typeof (x as Record<string, unknown>).name === "string" &&
    typeof (x as Record<string, unknown>).enabled === "boolean"
  );
}

const data: unknown = JSON.parse('[{"name":"aiko","enabled":true},{"name":5}]');
const users = Array.isArray(data) ? data.filter(isUser) : [];
console.log(users.map((u) => u.name));   // u: User
```

In real projects, a validation library like **Zod** writes these guards for you from a schema: `const User = z.object({ name: z.string(), enabled: z.boolean() })`.

## Errors are unknown

In `catch (e)`, `e` is `unknown`, because anything can be thrown in JavaScript (even a string). Narrow it before use:

```typescript
try {
  JSON.parse("{bad json");
} catch (e) {
  const message = e instanceof Error ? e.message : String(e);
  console.log("parse failed:", message.split(" ")[0]);
}
```

@@quiz
After `if (typeof x === "string") return;`, what is `x` if its type was `string | number`?
* `number`
- `string`
- `string | number`
- `never`
? The `string` case returned, so only `number` is left.

@@quiz
What makes a union "discriminated"?
* Each variant has a common property with a different literal type (the tag), like `kind: "ok"`
- It has exactly two members
- It uses classes
- It's declared with `interface`
? Checking the tag narrows the union to one variant.

@@quiz
What does `function isUser(x: unknown): x is User` tell the compiler?
* When it returns true, `x` can be treated as a `User`
- It converts x into a User
- x is always a User
- It throws if x isn't a User
? A type predicate. The function's logic must actually check, or the type is a lie.

@@flashcards
narrowing :: a check makes a union type more specific
`typeof x === "string"` :: narrows to string
`"prop" in x` :: narrows to the members that have prop
discriminated union :: variants with a literal tag property
`never` :: a type with no values
exhaustiveness check :: `assertNever(x)` in `default:`
`x is T` :: a type predicate (custom type guard)
`catch (e)` :: e is `unknown`

@@exercise HTTP result
Write `describeResult(r: Result): string` for the discriminated union: `ok` → `200 OK (12 ms)`, `redirect` → `301 -> https://new.example.jp`, `fail` → `500 failed: db down`.
@@starter
type Result =
  | { kind: "ok"; status: number; ms: number }
  | { kind: "redirect"; status: number; location: string }
  | { kind: "fail"; status: number; reason: string };

function describeResult(r: Result): string {
  throw new Error("TODO: not done yet");
}

@@tests
check(describeResult({ kind: "ok", status: 200, ms: 12 }), "200 OK (12 ms)", "ok");
check(describeResult({ kind: "redirect", status: 301, location: "https://new.example.jp" }), "301 -> https://new.example.jp", "redirect");
check(describeResult({ kind: "fail", status: 500, reason: "db down" }), "500 failed: db down", "fail");
@@hint
`switch (r.kind) { case "ok": return \`${r.status} OK (${r.ms} ms)\`; ... }`
@@solution
type Result =
  | { kind: "ok"; status: number; ms: number }
  | { kind: "redirect"; status: number; location: string }
  | { kind: "fail"; status: number; reason: string };

function describeResult(r: Result): string {
  switch (r.kind) {
    case "ok":
      return `${r.status} OK (${r.ms} ms)`;
    case "redirect":
      return `${r.status} -> ${r.location}`;
    case "fail":
      return `${r.status} failed: ${r.reason}`;
  }
}

@@exercise Validate a config
Write a type guard `isConfig(x: unknown): x is Config` for `{ host: string; port: number; tls?: boolean }`. The port must be an integer from 1 to 65535, and `tls`, if present, a boolean.
@@starter
type Config = { host: string; port: number; tls?: boolean };

function isConfig(x: unknown): x is Config {
  throw new Error("TODO: not done yet");
}

@@tests
check(isConfig({ host: "db01", port: 5432 }), true, "minimal config");
check(isConfig({ host: "db01", port: 443, tls: true }), true, "with tls");
check(isConfig({ host: "db01", port: "443" }), false, "port as text");
check(isConfig({ host: "db01", port: 70000 }), false, "port out of range");
check(isConfig({ host: "db01", port: 22.5 }), false, "port not an integer");
check(isConfig({ host: "db01", port: 22, tls: "yes" }), false, "tls not boolean");
check(isConfig(null), false, "null");
check(isConfig("db01:22"), false, "a string");
@@hint
First `if (typeof x !== "object" || x === null) return false;`, then `const o = x as Record<string, unknown>;` and check each field. `Number.isInteger(o.port)` works for the integer test.
@@solution
type Config = { host: string; port: number; tls?: boolean };

function isConfig(x: unknown): x is Config {
  if (typeof x !== "object" || x === null) return false;
  const o = x as Record<string, unknown>;
  return (
    typeof o.host === "string" &&
    typeof o.port === "number" &&
    Number.isInteger(o.port) &&
    o.port >= 1 &&
    o.port <= 65535 &&
    (o.tls === undefined || typeof o.tls === "boolean")
  );
}
