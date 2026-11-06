@@lesson t1-tuples Tuples, Literal Types, as const and satisfies
@@concepts lists, variables
@@text
TypeScript can track **exact values**, not just "some string". That precision powers autocomplete, catches typos in configuration, and replaces enums.

## Literal types and widening

```ts
let mutable = "warn";          // type: string (it could change later)
const fixed = "warn";          // type: "warn" (a literal type)
type Level = "debug" | "info" | "warn" | "error";
function log(level: Level, msg: string): string { return `[${level}] ${msg}`; }
console.log(log(fixed, "disk 85%"));
// log(mutable, "x");          // error: string is not assignable to Level
```

TypeScript **widens** `let` variables and object properties to the general type, because they can change. That's often why a value "suddenly" isn't assignable to a union of literals.

## as const

`as const` says "this value will never change": everything becomes readonly and literal:

```ts
const LEVELS = ["debug", "info", "warn", "error"] as const;
type Level = (typeof LEVELS)[number];          // "debug" | "info" | "warn" | "error"
const isLevel = (x: string): x is Level => (LEVELS as readonly string[]).includes(x);

const HTTP = { ok: 200, notFound: 404, serverError: 500 } as const;
type HttpCode = (typeof HTTP)[keyof typeof HTTP];   // 200 | 404 | 500
console.log(isLevel("warn"), isLevel("loud"), HTTP.notFound);
```

This "const object/array + derived type" pattern is the modern replacement for `enum`: one source of truth, usable at runtime (to loop over, validate against) **and** as a type.

## Tuples

A **tuple** is an array with a fixed length and a type per position:

```ts
type Endpoint = [host: string, port: number];            // labelled elements
const db: Endpoint = ["db01", 5432];
const [host, port] = db;

type Range = readonly [start: number, end?: number];      // optional element
type Command = [name: string, ...args: string[]];         // rest element
const cmd: Command = ["ping", "-c", "3", "10.0.0.1"];

function minMax(values: number[]): [min: number, max: number] {
  return [Math.min(...values), Math.max(...values)];
}
const [lo, hi] = minMax([5, 2, 9]);
console.log(host, port, cmd.length, lo, hi);
```

Tuples are ideal for small, positional return values (like React's `useState` returning `[value, setValue]`). For more than 2–3 elements, an object with named properties is clearer.

## satisfies

`satisfies` checks that a value matches a type **without changing** the value's inferred type:

```ts
type Route = { path: string; auth: boolean };

const routes = {
  home: { path: "/", auth: false },
  tickets: { path: "/tickets", auth: true },
} satisfies Record<string, Route>;

routes.tickets.path;          // OK, and autocomplete knows the keys "home" | "tickets"
// routes.reports;            // error: no such key (with a type annotation this would be allowed)
console.log(Object.keys(routes));
```

With an annotation (`const routes: Record<string, Route> = ...`), TypeScript would forget the specific keys. `satisfies` gives you both: validation **and** precise inference. Use it for configuration objects, lookup tables and theme definitions.

## Template literal types (preview)

Literal types can be built from other literals:

```ts
type Env = "dev" | "staging" | "prod";
type Region = "eu" | "us";
type ClusterName = `${Env}-${Region}`;           // "dev-eu" | "dev-us" | ... (6 combinations)
const c: ClusterName = "prod-eu";
console.log(c);
```

The type system in depth module uses these to type event names, CSS units, routes and more.

@@quiz
Why does `let level = "warn"` not fit a parameter of type `"debug" | "warn"`?
* `let` widens the type to `string`, since the value could change
- "warn" is misspelled
- let variables can't be passed to functions
- Literal types only work with numbers
? Use `const`, an annotation, or `as const`.

@@quiz
What does `(typeof LEVELS)[number]` give for `const LEVELS = ["a", "b"] as const`?
- number
- string[]
* `"a" | "b"`
- readonly ["a", "b"]
? Indexing a tuple type with `number` gives the union of its element types.

@@quiz
What's the advantage of `satisfies` over a type annotation?
* It checks the value against the type while keeping the more precise inferred type
- It's faster at runtime
- It converts the value
- It allows any value
? You keep exact keys and literal values for autocomplete.

@@drill
Which assertion makes an object literal deeply readonly with literal types?
= as const
? const roles = ["admin", "user"] as const.

@@flashcards
Literal type :: an exact value as a type, like `"warn"` or `443`
Widening :: `let` and properties get the general type
`as const` :: deeply readonly with literal types
Const object pattern :: `as const` object + derived type; replaces enum
Tuple :: fixed-length array with a type per position
Labelled tuple :: `[host: string, port: number]`
`satisfies` :: check against a type, keep the inferred type
Template literal type :: `` `${A}-${B}` `` builds string literal unions

@@exercise Severity levels without enums
Using a const array `SEVERITIES = ["info", "low", "medium", "high", "critical"] as const` and the derived `type Severity`, write:

- `isSeverity(x: string): x is Severity`
- `compareSeverity(a: Severity, b: Severity): number`: negative/0/positive by position in the list
- `atLeast(min: Severity): Severity[]`: all severities from `min` upwards
- `parseSeverity(text: string): Severity`: case-insensitive, trims, accepts the aliases `"crit"` → critical and `"warn"` → medium; throws `Error("unknown severity: <text>")` otherwise
@@starter
const SEVERITIES = ["info", "low", "medium", "high", "critical"] as const;
type Severity = (typeof SEVERITIES)[number];

function isSeverity(x: string): x is Severity {
  throw new Error("TODO: not done yet");
}

function compareSeverity(a: Severity, b: Severity): number {
  throw new Error("TODO: not done yet");
}

function atLeast(min: Severity): Severity[] {
  throw new Error("TODO: not done yet");
}

function parseSeverity(text: string): Severity {
  throw new Error("TODO: not done yet");
}

@@tests
check([isSeverity("high"), isSeverity("HIGH"), isSeverity("urgent")], [true, false, false], "guard");
const sorted: Severity[] = ["critical", "info", "high", "low"];
check(sorted.sort(compareSeverity), ["info", "low", "high", "critical"], "sort by severity");
check(atLeast("high"), ["high", "critical"], "at least");
check([" Crit ", "WARN", "Low"].map(parseSeverity), ["critical", "medium", "low"], "parse with aliases");
let msg = "";
try { parseSeverity("meh"); } catch (e) { msg = (e as Error).message; }
check(msg, "unknown severity: meh", "unknown");
@@hint
`(SEVERITIES as readonly string[]).includes(x)` avoids a type error from `includes` on a readonly literal tuple. Aliases: `const ALIASES: Record<string, Severity> = { crit: "critical", warn: "medium" };`.
@@solution
const SEVERITIES = ["info", "low", "medium", "high", "critical"] as const;
type Severity = (typeof SEVERITIES)[number];
const ALIASES: Record<string, Severity> = { crit: "critical", warn: "medium" };

function isSeverity(x: string): x is Severity {
  return (SEVERITIES as readonly string[]).includes(x);
}

function compareSeverity(a: Severity, b: Severity): number {
  return SEVERITIES.indexOf(a) - SEVERITIES.indexOf(b);
}

function atLeast(min: Severity): Severity[] {
  return SEVERITIES.slice(SEVERITIES.indexOf(min));
}

function parseSeverity(text: string): Severity {
  const t = text.trim().toLowerCase();
  if (isSeverity(t)) return t;
  const alias = ALIASES[t];
  if (alias) return alias;
  throw new Error(`unknown severity: ${text}`);
}

@@exercise Typed routes with satisfies
Define `const ROUTES = { ... } satisfies Record<string, RouteDef>` where `type RouteDef = { path: string; methods: readonly Method[]; auth: boolean }` and `type Method = "GET" | "POST" | "DELETE"`, with these routes:

- `health`: `/health`, GET, no auth
- `tickets`: `/tickets`, GET and POST, auth
- `ticket`: `/tickets/:id`, GET and DELETE, auth

Then write `type RouteName = keyof typeof ROUTES` and:

- `allowed(name: RouteName, method: Method): boolean`
- `buildPath(name: RouteName, params: Record<string, string | number> = {}): string`: replace `:param` segments (URL-encoded); throw `Error("missing param <p>")` if one is missing
- `table(): [name: RouteName, path: string, methods: string][]`: one tuple per route in definition order, methods joined with `,`
@@starter
type Method = "GET" | "POST" | "DELETE";
type RouteDef = { path: string; methods: readonly Method[]; auth: boolean };

const ROUTES = {} satisfies Record<string, RouteDef>;
type RouteName = keyof typeof ROUTES;

function allowed(name: RouteName, method: Method): boolean {
  throw new Error("TODO: not done yet");
}

function buildPath(name: RouteName, params: Record<string, string | number> = {}): string {
  throw new Error("TODO: not done yet");
}

function table(): [name: RouteName, path: string, methods: string][] {
  throw new Error("TODO: not done yet");
}

@@tests
check([allowed("tickets", "POST"), allowed("health", "DELETE")], [true, false], "allowed");
check(buildPath("ticket", { id: 42 }), "/tickets/42", "path with param");
check(buildPath("ticket", { id: "a b" }), "/tickets/a%20b", "encoded");
let msg = "";
try { buildPath("ticket"); } catch (e) { msg = (e as Error).message; }
check(msg, "missing param id", "missing");
check(table(), [["health", "/health", "GET"], ["tickets", "/tickets", "GET,POST"], ["ticket", "/tickets/:id", "GET,DELETE"]], "table");
@@hint
`ROUTES[name].methods.includes(method)`. For paths: `path.replace(/:(\w+)/g, (_, p: string) => { const v = params[p]; if (v === undefined) throw ...; return encodeURIComponent(String(v)); })`. For the table, `Object.keys(ROUTES) as RouteName[]` is a safe assertion here because the object is a constant you defined.
@@solution
type Method = "GET" | "POST" | "DELETE";
type RouteDef = { path: string; methods: readonly Method[]; auth: boolean };

const ROUTES = {
  health: { path: "/health", methods: ["GET"], auth: false },
  tickets: { path: "/tickets", methods: ["GET", "POST"], auth: true },
  ticket: { path: "/tickets/:id", methods: ["GET", "DELETE"], auth: true },
} satisfies Record<string, RouteDef>;
type RouteName = keyof typeof ROUTES;

function allowed(name: RouteName, method: Method): boolean {
  const methods: readonly Method[] = ROUTES[name].methods;
  return methods.includes(method);
}

function buildPath(name: RouteName, params: Record<string, string | number> = {}): string {
  return ROUTES[name].path.replace(/:(\w+)/g, (_match, p: string) => {
    const v = params[p];
    if (v === undefined) throw new Error(`missing param ${p}`);
    return encodeURIComponent(String(v));
  });
}

function table(): [name: RouteName, path: string, methods: string][] {
  return (Object.keys(ROUTES) as RouteName[]).map((name) => [name, ROUTES[name].path, ROUTES[name].methods.join(",")]);
}
