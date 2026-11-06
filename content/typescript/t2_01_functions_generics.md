@@lesson t2-functions-generics Functions and Generics
@@concepts functions
@@text
## Typed functions

```typescript
function percent(part: number, whole: number): number {
  return whole === 0 ? 0 : Math.round((part / whole) * 1000) / 10;
}

const toGb = (bytes: number): number => bytes / 1024 ** 3;

function ping(host: string, count = 4, timeoutMs?: number): string {
  return `ping ${host} x${count}${timeoutMs ? ` (${timeoutMs}ms)` : ""}`;
}

console.log(percent(45, 60), toGb(8 * 1024 ** 3), ping("10.0.0.1"), ping("10.0.0.1", 2, 500));
```

- A default value (`count = 4`) makes a parameter optional *and* gives its type.
- `timeoutMs?: number` is optional, with type `number | undefined` inside the function.
- `void` means the function returns nothing useful.

## Function types

Functions are values, so they have types too, which is useful for callbacks:

```typescript
type Check = (host: string) => boolean;
type Logger = (level: "info" | "error", msg: string) => void;

const isProd: Check = (host) => host.startsWith("prod-");   // host is inferred as string
const log: Logger = (level, msg) => console.log(`${level}: ${msg}`);

function runAll(hosts: string[], check: Check, logger: Logger): number {
  let ok = 0;
  for (const h of hosts) {
    if (check(h)) ok++;
    else logger("error", `${h} failed`);
  }
  return ok;
}
console.log(runAll(["prod-web", "dev-web"], isProd, log));
```

## Generics: types as parameters

A **generic** function works for many types while keeping them connected. `T` is a type parameter, filled in at each call:

```typescript
function first<T>(items: T[]): T | undefined {
  return items[0];
}

const h = first(["web01", "web02"]);   // T = string, so h: string | undefined
const p = first([22, 443]);            // T = number
console.log(h?.toUpperCase(), p);
```

Without generics you'd write `items: any[]` and lose all type information. You've already used generics: `Array<T>`, `Promise<T>`, `Record<K, V>`, `Map<K, V>`.

```typescript
function groupBy<T, K extends string>(items: T[], key: (item: T) => K): Record<K, T[]> {
  const out = {} as Record<K, T[]>;
  for (const item of items) {
    const k = key(item);
    (out[k] ??= []).push(item);
  }
  return out;
}

const tickets = [
  { id: 1, team: "network" },
  { id: 2, team: "desktop" },
  { id: 3, team: "network" },
] as const;
const byTeam = groupBy([...tickets], (t) => t.team);
console.log(byTeam.network.length, byTeam.desktop[0].id);
```

`K extends string` is a **constraint**: K can be any string type, but not a number or an object.

## Constraints with keyof

`keyof T` is the union of a type's property names, so you can write functions that only accept real keys:

```typescript
function pluck<T, K extends keyof T>(items: T[], key: K): T[K][] {
  return items.map((i) => i[key]);
}

const servers = [{ name: "web01", cpu: 35 }, { name: "db01", cpu: 71 }];
const names = pluck(servers, "name");   // string[]
const cpus = pluck(servers, "cpu");     // number[]
// pluck(servers, "ram");  ← error: '"ram"' is not assignable to '"name" | "cpu"'
console.log(names, cpus);
```

`T[K]` is an **indexed access type**: "the type of property K of T".

@@quiz
`function first<T>(items: T[]): T | undefined`. What's the return type of `first([1, 2])`?
* `number | undefined`
- `T`
- `any`
- `number[]`
? T is inferred from the argument as `number`.

@@quiz
What does `K extends keyof T` mean?
* K must be one of T's property names
- K is a subclass of T
- K can be any string
- K is optional
? `keyof T` is the union of T's keys; `extends` constrains K to it.

@@quiz
Why use a generic `<T>` instead of `any`?
* It keeps the connection between input and output types, so the result is still typed
- It's faster at run time
- `any` isn't allowed in functions
- Generics are checked at run time
? With `any`, the result is `any` too, and every check is lost.

@@drill
Which operator produces a union of an object type's keys?
= keyof
? keyof { a: 1; b: 2 } is "a" | "b".

@@flashcards
`(x: string) => boolean` :: a function type
`count = 4` :: optional parameter with a default
`x?: number` :: optional parameter (`number | undefined`)
`<T>` :: a type parameter (generic)
`T extends string` :: a constraint on T
`keyof T` :: union of T's property names
`T[K]` :: the type of property K of T

@@exercise Retry with types
Write a generic `retry<T>(fn: () => T, attempts: number): T`. Call `fn`; if it throws, try again, up to `attempts` times in total. If every attempt fails, throw the **last** error.
@@starter
function retry<T>(fn: () => T, attempts: number): T {
  throw new Error("TODO: not done yet");
}

@@tests
let calls = 0;
const flaky = () => { calls++; if (calls < 3) throw new Error(`fail ${calls}`); return "connected"; };
check(retry(flaky, 5), "connected", "succeeds on the third try");
check(calls, 3, "stopped after success");

let n = 0;
let message = "";
try { retry(() => { n++; throw new Error(`fail ${n}`); }, 2); } catch (e) { message = (e as Error).message; }
check(message, "fail 2", "throws the last error");
check(retry(() => 42, 1), 42, "returns the value (a number)");
@@hint
`let last: unknown; for (let i = 0; i < attempts; i++) { try { return fn(); } catch (e) { last = e; } } throw last;`
@@solution
function retry<T>(fn: () => T, attempts: number): T {
  let last: unknown;
  for (let i = 0; i < attempts; i++) {
    try {
      return fn();
    } catch (e) {
      last = e;
    }
  }
  throw last;
}

@@exercise Index by key
Write a generic `indexBy<T, K extends keyof T>(items: T[], key: K): Map<T[K], T>` returning a `Map` from each item's `key` value to the item. Later items replace earlier ones with the same key.
@@starter
function indexBy<T, K extends keyof T>(items: T[], key: K): Map<T[K], T> {
  throw new Error("TODO: not done yet");
}

@@tests
const users = [{ id: 7, name: "aiko" }, { id: 9, name: "kenji" }, { id: 7, name: "aiko2" }];
const byId = indexBy(users, "id");
check(byId.size, 2, "two distinct ids");
check(byId.get(9)?.name, "kenji", "lookup by id");
check(byId.get(7)?.name, "aiko2", "later items win");
check([...indexBy(users, "name").keys()], ["aiko", "kenji", "aiko2"], "by name");
@@hint
`const map = new Map<T[K], T>(); for (const item of items) map.set(item[key], item); return map;`
@@solution
function indexBy<T, K extends keyof T>(items: T[], key: K): Map<T[K], T> {
  const map = new Map<T[K], T>();
  for (const item of items) map.set(item[key], item);
  return map;
}
