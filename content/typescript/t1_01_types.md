@@lesson t1-types Arrays, Unions and Object Types
@@concepts lists, objects
@@text
## Arrays and tuples

```typescript
const hosts: string[] = ["web01", "web02"];
const ports: Array<number> = [22, 443];          // the same, written generically
const pair: [string, number] = ["db01", 5432];   // a tuple: fixed length, a type per position
const [host, port] = pair;
console.log(hosts.length, ports[1], host, port);

const readonlyList: readonly string[] = ["a", "b"];
// readonlyList.push("c");   ← error: push doesn't exist on readonly arrays
```

## Union types

A value that may be one of several types: `string | number`.

```typescript
function formatId(id: string | number): string {
  return typeof id === "number" ? `#${id.toString().padStart(5, "0")}` : id.toUpperCase();
}
console.log(formatId(42), formatId("inc-7"));
```

Inside each branch, TypeScript knows which type you have. That's **narrowing** (a full lesson later).

## Literal types

A type can be a specific value. Combined with unions, they replace enums:

```typescript
type Level = "debug" | "info" | "warn" | "error";
type HttpMethod = "GET" | "POST" | "PUT" | "DELETE";

function log(level: Level, message: string): void {
  console.log(`[${level.toUpperCase()}] ${message}`);
}
log("warn", "disk 85% full");
// log("critical", "x");   ← error: '"critical"' is not assignable to type 'Level'
```

Your editor autocompletes the allowed values, and a typo is a compile error. When you also need the list at run time, write it once as an array:

```typescript
const LEVELS = ["debug", "info", "warn", "error"] as const;
type Level = (typeof LEVELS)[number];          // "debug" | "info" | "warn" | "error"
console.log(LEVELS.includes("info"), LEVELS.indexOf("warn"));
```

`as const` makes the array readonly and keeps the exact strings as types.

## Object types

```typescript
type Server = {
  name: string;
  ip: string;
  cpu: number;
  tags?: string[];          // optional: may be missing
  readonly id: number;      // can't be changed after creation
};

const s: Server = { id: 1, name: "web01", ip: "10.0.0.11", cpu: 35 };
s.cpu = 40;
// s.id = 2;     ← error: id is read-only
console.log(s.tags?.length ?? 0);   // tags may be undefined
```

`interface Server { ... }` means almost the same thing as `type Server = { ... }`. Teams pick one style; interfaces can be extended (`interface Admin extends User`), and `type` can also name unions. This course uses `type` for data and `interface` for "contracts" that classes implement.

## Structural typing

TypeScript checks the **shape** of a value, not its name. Anything with the right properties fits:

```typescript
type HasName = { name: string };
function shout(x: HasName) { return x.name.toUpperCase(); }

const server = { name: "db01", ip: "10.0.0.21" };   // never declared as HasName
console.log(shout(server));                        // fine: it has a name
```

## Record and index signatures

For dictionaries keyed by strings:

```typescript
const owners: Record<string, string> = { web01: "aiko", db01: "kenji" };
owners["mail01"] = "yui";
const counts: { [level: string]: number } = {};
for (const l of ["INFO", "ERROR", "INFO"]) counts[l] = (counts[l] ?? 0) + 1;
console.log(owners, counts);
```

Turn on `noUncheckedIndexedAccess` in real projects: then `owners["x"]` has type `string | undefined`, which reminds you that the key may be missing.

@@quiz
`type Level = "info" | "warn";` What can a `Level` variable hold?
* Only the strings `"info"` or `"warn"`
- Any string
- The numbers 0 and 1
- `undefined`
? Literal union types are TypeScript's lightweight replacement for enums.

@@quiz
What does `tags?: string[]` mean in an object type?
* `tags` may be missing (undefined), or an array of strings
- `tags` is an array that may contain undefined
- `tags` is required
- `tags` is read-only
? Use `?.` and `??` when reading optional properties.

@@quiz
`function f(x: { name: string })`: can you pass `{ name: "a", ip: "b" }` stored in a variable?
* Yes: TypeScript checks the shape, and extra properties are fine
- No: the types have different names
- Only with `as`
- Only if `ip` is optional
? Structural typing. (An object literal written directly in the call *is* checked for extra properties, to catch typos.)

@@flashcards
`string[]` :: an array of strings
`[string, number]` :: a tuple
`string | number` :: a union: either type
`"GET" | "POST"` :: a union of literal types
`as const` :: keep exact values, make readonly
`prop?: T` :: optional property
`readonly prop: T` :: can't be reassigned
`Record<string, T>` :: an object used as a dictionary

@@exercise Severity order
`type Severity = "low" | "medium" | "high" | "critical"`. Write `worst(list: Severity[]): Severity | null` returning the most severe value in the list, or `null` for an empty list.
@@starter
const SEVERITIES = ["low", "medium", "high", "critical"] as const;
type Severity = (typeof SEVERITIES)[number];

function worst(list: Severity[]): Severity | null {
  throw new Error("TODO: not done yet");
}

@@tests
check(worst(["low", "high", "medium"]), "high", "high beats medium");
check(worst(["critical", "low"]), "critical", "critical first");
check(worst(["low"]), "low", "single");
check(worst([]), null, "empty list");
@@hint
Compare positions in `SEVERITIES`: `SEVERITIES.indexOf(s)` is bigger for worse values. Loop and keep the best so far, or use `reduce`.
@@solution
const SEVERITIES = ["low", "medium", "high", "critical"] as const;
type Severity = (typeof SEVERITIES)[number];

function worst(list: Severity[]): Severity | null {
  let result: Severity | null = null;
  for (const s of list) {
    if (result === null || SEVERITIES.indexOf(s) > SEVERITIES.indexOf(result)) result = s;
  }
  return result;
}

@@exercise Inventory summary
Write `summarize(servers: Server[]): Record<string, number>` that counts servers per environment, treating a missing `env` as `"prod"`.
@@starter
type Server = { name: string; env?: "prod" | "staging" | "dev" };

function summarize(servers: Server[]): Record<string, number> {
  throw new Error("TODO: not done yet");
}

@@tests
check(summarize([{ name: "a" }, { name: "b", env: "dev" }, { name: "c", env: "prod" }]), { prod: 2, dev: 1 }, "counts");
check(summarize([]), {}, "empty");
@@hint
`const out: Record<string, number> = {}; for (const s of servers) { const env = s.env ?? "prod"; out[env] = (out[env] ?? 0) + 1; }`
@@solution
type Server = { name: string; env?: "prod" | "staging" | "dev" };

function summarize(servers: Server[]): Record<string, number> {
  const out: Record<string, number> = {};
  for (const s of servers) {
    const env = s.env ?? "prod";
    out[env] = (out[env] ?? 0) + 1;
  }
  return out;
}
