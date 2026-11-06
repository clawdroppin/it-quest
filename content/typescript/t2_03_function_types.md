@@lesson t2-function-types Function Types in Depth: Overloads, Predicates, Assertions and this
@@concepts functions
@@text
Functions are where types pay off most: callers get autocomplete and errors, and implementations get checked. This lesson covers the function typing features you'll meet in real codebases and library definitions.

## Function type expressions

```ts
type Check = (host: string, timeoutMs?: number) => Promise<boolean>;
type Handler<E> = (event: E) => void;
interface Retryable {
  (task: () => Promise<void>): Promise<void>;   // call signature
  maxAttempts: number;                         // plus a property
}

const alwaysUp: Check = async (host) => host.length > 0;   // parameter types come from Check
alwaysUp("web01").then((up) => console.log("up?", up));
```

**Contextual typing**: when a function is assigned to a known type (or passed as a callback), its parameters get their types from it, so you don't need to annotate `host`.

## void vs undefined in callbacks

A callback typed to return `void` may return anything; the value is just ignored. That's why `items.forEach((x) => list.push(x))` is fine even though `push` returns a number. A function **declared** to return `void`, however, can't return a value.

## Optional, default and rest parameters

```ts
function connect(host: string, port = 443, ...flags: string[]): string {
  return `${host}:${port} ${flags.join(" ")}`.trim();
}
console.log(connect("db01"), connect("db01", 5432, "--tls", "--verbose"));
```

## Overloads

When the return type depends on the arguments in ways a single signature can't express, declare **overloads**: several signatures, then one implementation:

```ts
function parse(input: string): number;
function parse(input: string[]): number[];
function parse(input: string | string[]): number | number[] {
  return Array.isArray(input) ? input.map(Number) : Number(input);
}
const one = parse("42");            // number
const many = parse(["1", "2"]);     // number[]
console.log(one + 1, many.length);
```

Callers only see the overload signatures, not the implementation's. Prefer unions or generics when they work: overloads are harder to maintain. (Generics with conditional types, in the next module, can often replace them.)

## Type predicates

A function returning `x is T` is a **type guard**: when it returns true, TypeScript narrows the argument:

```ts
interface Incident { kind: "incident"; sev: number }
interface Request { kind: "request"; item: string }
type Ticket = Incident | Request;

const isIncident = (t: Ticket): t is Incident => t.kind === "incident";
const tickets: Ticket[] = [{ kind: "incident", sev: 2 }, { kind: "request", item: "laptop" }];
const incidents = tickets.filter(isIncident);     // Incident[]
console.log(incidents.map((i) => i.sev));
```

Since TypeScript 5.5, simple arrow functions like `(t) => t.kind === "incident"` infer the predicate automatically when passed to `filter`.

## Assertion functions

An **assertion function** throws if a condition is false; afterwards, TypeScript assumes it's true:

```ts
function assert(condition: unknown, message: string): asserts condition {
  if (!condition) throw new Error(message);
}
function assertString(x: unknown, name: string): asserts x is string {
  if (typeof x !== "string") throw new TypeError(`${name} must be a string`);
}
const env: Record<string, unknown> = { DB_HOST: "db01" };
const dbHost = env.DB_HOST;
assertString(dbHost, "DB_HOST");
console.log(dbHost.toUpperCase());       // dbHost: string from here on
```

Great for startup config checks and invariants.

## Typing `this`

A fake first parameter named `this` declares what `this` must be (it's erased at runtime):

```ts
interface Counter { count: number }
function increment(this: Counter, by = 1): number {
  this.count += by;
  return this.count;
}
const c: Counter = { count: 0 };
console.log(increment.call(c, 5));
// increment(1);                    // error: 'this' context of type 'void' isn't assignable
```

## Generic functions (recap)

```ts
function firstOr<T>(items: readonly T[], fallback: T): T {
  return items[0] ?? fallback;
}
console.log(firstOr([3, 4], 0), firstOr<string>([], "none"));
```

The next module goes much deeper into generics: constraints, defaults, inference and conditional types.

@@quiz
When should you use function overloads?
* When the return type depends on the argument types in a way a single signature can't express
- Always, for every function
- To make functions faster
- To allow any argument
? Prefer unions or generics when they suffice.

@@quiz
What does `function isX(v: T): v is X` let TypeScript do?
- Convert v to X
* Narrow v to X in code where the function returned true
- Check the type at compile time only
- Throw if v isn't X
? It's a user-defined type guard.

@@quiz
What does `asserts x is string` in a return type mean?
* If the function returns normally, x is a string afterwards; otherwise it throws
- The function returns a string
- x is converted to a string
- It's a comment
? Use assertion functions for invariants and config checks.

@@drill
Write the return type of a type predicate that says `x` is a string.
= x is string
? function isString(x: unknown): x is string.

@@flashcards
Function type :: `(a: A) => R`
Call signature :: `{ (a: A): R }` in an object type
Contextual typing :: callback parameters typed from where they're used
`void` return (callback) :: return value ignored
Overloads :: several signatures, one implementation
Type predicate :: `x is T` return type of a guard
Assertion function :: `asserts x is T` / `asserts cond`
`this` parameter :: declares the required `this` type

@@exercise Overloaded lookups
Write an overloaded `lookup` for a host table:

- `lookup(name: string): Host | undefined`
- `lookup(names: string[]): Host[]` (only found hosts, in the given order)
- `lookup(predicate: (h: Host) => boolean): Host[]`

`type Host = { name: string; ip: string; role: "web" | "db" | "mail" }` and the table `HOSTS` is given. Names compare case-insensitively.

Then write a type predicate `isRole<R extends Host["role"]>(role: R): (h: Host) => h is Host & { role: R }` that builds a guard for one role.
@@starter
type Host = { name: string; ip: string; role: "web" | "db" | "mail" };
const HOSTS: Host[] = [
  { name: "web01", ip: "10.0.0.5", role: "web" },
  { name: "db01", ip: "10.0.0.6", role: "db" },
  { name: "web02", ip: "10.0.0.7", role: "web" },
];

function lookup(arg: unknown): unknown {
  throw new Error("TODO: not done yet");
}

function isRole(role: string): (h: Host) => boolean {
  throw new Error("TODO: not done yet");
}

@@tests
check(lookup("DB01"), { name: "db01", ip: "10.0.0.6", role: "db" }, "single");
check(lookup("nope"), undefined, "missing");
check(lookup(["web02", "x", "web01"]), [HOSTS[2], HOSTS[0]], "many, in order");
check(lookup((h: Host) => h.ip.endsWith(".7")), [HOSTS[2]], "predicate");
const webs = HOSTS.filter(isRole("web"));
check(webs.map((h) => h.name), ["web01", "web02"], "role guard");
@@hint
Write the three overload signatures above one implementation `function lookup(arg: string | string[] | ((h: Host) => boolean)): Host | Host[] | undefined`. Inside, check `typeof arg === "function"`, then `Array.isArray(arg)`. For `isRole`, return `(h: Host): h is Host & { role: R } => h.role === role`.
@@solution
type Host = { name: string; ip: string; role: "web" | "db" | "mail" };
const HOSTS: Host[] = [
  { name: "web01", ip: "10.0.0.5", role: "web" },
  { name: "db01", ip: "10.0.0.6", role: "db" },
  { name: "web02", ip: "10.0.0.7", role: "web" },
];

const find = (name: string): Host | undefined => HOSTS.find((h) => h.name.toLowerCase() === name.toLowerCase());

function lookup(name: string): Host | undefined;
function lookup(names: string[]): Host[];
function lookup(predicate: (h: Host) => boolean): Host[];
function lookup(arg: string | string[] | ((h: Host) => boolean)): Host | Host[] | undefined {
  if (typeof arg === "function") return HOSTS.filter(arg);
  if (Array.isArray(arg)) return arg.map(find).filter((h): h is Host => h !== undefined);
  return find(arg);
}

function isRole<R extends Host["role"]>(role: R): (h: Host) => h is Host & { role: R } {
  return (h: Host): h is Host & { role: R } => h.role === role;
}

@@exercise Config assertions
Write assertion functions and use them to load config from an environment-like object:

- `assertDefined<T>(value: T | undefined | null, name: string): asserts value is T` → throws `Error("<name> is required")`
- `assertPort(value: unknown, name: string): asserts value is number` → must be an integer 1–65535, else `Error("<name> must be a port")`
- `loadConfig(env: Record<string, string | undefined>): { dbHost: string; dbPort: number; debug: boolean }`: `DB_HOST` required; `DB_PORT` optional (default `"5432"`) and converted with `Number` before `assertPort`; `DEBUG` is true only for `"1"` or `"true"`. Collect **all** problems and throw one `Error` whose message lists them joined with `"; "`.
@@starter
function assertDefined<T>(value: T | undefined | null, name: string): asserts value is T {
  throw new Error("TODO: not done yet");
}

function assertPort(value: unknown, name: string): asserts value is number {
  throw new Error("TODO: not done yet");
}

function loadConfig(env: Record<string, string | undefined>): { dbHost: string; dbPort: number; debug: boolean } {
  throw new Error("TODO: not done yet");
}

@@tests
check(loadConfig({ DB_HOST: "db01" }), { dbHost: "db01", dbPort: 5432, debug: false }, "defaults");
check(loadConfig({ DB_HOST: "db02", DB_PORT: "6543", DEBUG: "true" }), { dbHost: "db02", dbPort: 6543, debug: true }, "all set");
let msg = "";
try { loadConfig({ DB_PORT: "99999" }); } catch (e) { msg = (e as Error).message; }
check(msg, "DB_HOST is required; DB_PORT must be a port", "all problems listed");
try { assertPort(22.5, "SSH_PORT"); } catch (e) { msg = (e as Error).message; }
check(msg, "SSH_PORT must be a port", "assertPort");
@@hint
Run each check in its own `try/catch` and push `e.message` into a `problems` array. Assertion functions must be declared with an explicit type annotation when stored in a variable; plain `function` declarations are simplest.
@@solution
function assertDefined<T>(value: T | undefined | null, name: string): asserts value is T {
  if (value === undefined || value === null) throw new Error(`${name} is required`);
}

function assertPort(value: unknown, name: string): asserts value is number {
  if (typeof value !== "number" || !Number.isInteger(value) || value < 1 || value > 65535) {
    throw new Error(`${name} must be a port`);
  }
}

function loadConfig(env: Record<string, string | undefined>): { dbHost: string; dbPort: number; debug: boolean } {
  const problems: string[] = [];
  const dbHost = env.DB_HOST;
  const dbPort = Number(env.DB_PORT ?? "5432");
  try {
    assertDefined(dbHost, "DB_HOST");
  } catch (e) {
    problems.push((e as Error).message);
  }
  try {
    assertPort(dbPort, "DB_PORT");
  } catch (e) {
    problems.push((e as Error).message);
  }
  if (problems.length > 0 || dbHost === undefined) throw new Error(problems.join("; "));
  return { dbHost, dbPort, debug: env.DEBUG === "1" || env.DEBUG === "true" };
}
