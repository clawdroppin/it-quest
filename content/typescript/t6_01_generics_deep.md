@@lesson t6-generics Generics in Depth: Constraints, Defaults, Inference and const Parameters
@@concepts functions, objects
@@text
Generics let one function or type work with many types **without losing information**. Module 2 introduced them; this module uses them to build the type-level tools that libraries like React, Zod, Prisma and tRPC are made of.

## Type tests

From now on, exercises include **type tests**: lines that are checked by the compiler, not at runtime. They use two tiny helpers:

```ts
type Equal<X, Y> = (<T>() => T extends X ? 1 : 2) extends (<T>() => T extends Y ? 1 : 2) ? true : false;
type Expect<T extends true> = T;

type _ok = Expect<Equal<1 | 2, 2 | 1>>;          // compiles
// type _bad = Expect<Equal<string, number>>;   // error: false doesn't satisfy true
console.log("type tests are erased at runtime");
```

The runner here strips types without checking them, so type tests always "pass" at runtime. Paste the code into an editor with TypeScript (VS Code, or `npx tsc --noEmit --strict file.ts`) to see type errors; the runtime tests still check the behaviour.

## Constraints

`T extends X` means "T can be any type that **is assignable to** X". Inside the function you can use everything X guarantees:

```ts
function longest<T extends { length: number }>(a: T, b: T): T {
  return a.length >= b.length ? a : b;
}
console.log(longest("ssh", "https"), longest([1, 2], [3]));

function prop<T, K extends keyof T>(obj: T, key: K): T[K] {
  return obj[key];
}
const host = { name: "web01", port: 443 };
const p = prop(host, "port");          // number, and "prot" would be a compile error
console.log(p);
```

`K extends keyof T` is the most important generic pattern: it ties one parameter to the keys of another and makes the return type `T[K]` exact.

## Inference and how to help it

TypeScript infers type arguments from the **values** you pass. When it can't (nothing to infer from), you write them: `new Map<string, number>()`, `useState<User | null>(null)`.

- Inference picks candidates from every argument and unifies them. `pair(1, "a")` with `pair<T>(a: T, b: T)` is an error, not `string | number`.
- `NoInfer<T>` (TypeScript 5.4) marks a parameter that shouldn't be used as an inference source:

```ts
function pickDefault<T extends string>(options: T[], fallback: NoInfer<T>): T {
  return options[0] ?? fallback;
}
pickDefault(["dev", "prod"], "dev");
// pickDefault(["dev", "prod"], "test");   // error: "test" isn't "dev" | "prod"
console.log("ok");
```

## Defaults

Type parameters can have defaults, like function parameters:

```ts
interface ApiResponse<TData = unknown, TError = string> {
  data?: TData;
  error?: TError;
}
const r1: ApiResponse = { error: "timeout" };                  // TData = unknown
const r2: ApiResponse<{ id: number }> = { data: { id: 1 } };
console.log(r1.error, r2.data?.id);
```

## const type parameters

A `const` modifier (TypeScript 5.0) infers literal types as if the caller wrote `as const`:

```ts
function defineRoles<const T extends readonly string[]>(roles: T): T {
  return roles;
}
const roles = defineRoles(["admin", "operator", "viewer"]);   // readonly ["admin", "operator", "viewer"]
type Role = (typeof roles)[number];                            // "admin" | "operator" | "viewer"
const me: Role = "operator";
console.log(me, roles.length);
```

Libraries use this so callers don't need `as const` everywhere.

## Generic classes

```ts
class Registry<T extends { id: string }> {
  private items = new Map<string, T>();
  add(item: T): this { this.items.set(item.id, item); return this; }
  get(id: string): T | undefined { return this.items.get(id); }
  get size(): number { return this.items.size; }
}
const reg = new Registry<{ id: string; ip: string }>().add({ id: "web01", ip: "10.0.0.5" });
console.log(reg.get("web01")?.ip, reg.size);
```

Returning `this` makes chaining work in subclasses too.

## Good generic hygiene

- **A type parameter should appear at least twice** (e.g. in a parameter and the return type). `function f<T>(x: T): void` is just `function f(x: unknown): void`.
- Constrain as little as needed, but as much as necessary.
- Name parameters meaningfully when there are several: `TKey`, `TValue`, `TEvent`.

@@quiz
In `function prop<T, K extends keyof T>(obj: T, key: K): T[K]`, what does `K extends keyof T` achieve?
* `key` must be one of obj's property names, and the return type is that property's type
- K becomes a string
- obj must have a property called K
- It allows any key
? This pattern powers type-safe getters, pickers and query builders.

@@quiz
What does a `const` type parameter do?
- Makes the argument immutable at runtime
* Infers literal (and readonly tuple) types as if the caller wrote `as const`
- Prevents the function from being called twice
- Makes T a constant number
? `defineRoles(["a", "b"])` infers `readonly ["a", "b"]`.

@@quiz
Why is `function log<T>(x: T): void` a poor use of generics?
* T appears only once, so it adds nothing over `unknown`
- Generics can't be used with void
- T must be constrained
- It's slower
? Type parameters should relate two or more positions.

@@drill
Which keyword constrains a type parameter?
= extends
? function f<T extends { id: number }>(x: T).

@@flashcards
Constraint :: `T extends X`: T must be assignable to X
`K extends keyof T` :: K is a key of T
Indexed access `T[K]` :: the property type for key K
Type argument inference :: TS works out T from the arguments
`NoInfer<T>` :: don't infer T from this position
Default type parameter :: `<T = unknown>`
`const` type parameter :: infer literal types
Return `this` :: chaining that works in subclasses
Type test :: `Expect<Equal<A, B>>`, checked by the compiler

@@exercise Typed collection helpers
Write these generic helpers. Each must keep exact types (see the type tests):

- `pluck<T, K extends keyof T>(items: readonly T[], key: K): T[K][]`
- `groupBy<T, K extends PropertyKey>(items: readonly T[], keyOf: (item: T) => K): Partial<Record<K, T[]>>`: preserves item order within groups
- `indexBy<T, K extends keyof T>(items: readonly T[], key: K): Map<T[K], T>`: later items win
- `sumBy<T>(items: readonly T[], value: (item: T) => number): number`
@@starter
function pluck<T, K extends keyof T>(items: readonly T[], key: K): T[K][] {
  throw new Error("TODO: not done yet");
}

function groupBy<T, K extends PropertyKey>(items: readonly T[], keyOf: (item: T) => K): Partial<Record<K, T[]>> {
  throw new Error("TODO: not done yet");
}

function indexBy<T, K extends keyof T>(items: readonly T[], key: K): Map<T[K], T> {
  throw new Error("TODO: not done yet");
}

function sumBy<T>(items: readonly T[], value: (item: T) => number): number {
  throw new Error("TODO: not done yet");
}

@@tests
type Equal<X, Y> = (<T>() => T extends X ? 1 : 2) extends (<T>() => T extends Y ? 1 : 2) ? true : false;
type Expect<T extends true> = T;
type Disk = { host: string; mount: string; usedGb: number; env: "prod" | "dev" };
const disks: Disk[] = [
  { host: "web01", mount: "/", usedGb: 40, env: "prod" },
  { host: "db01", mount: "/var", usedGb: 300, env: "prod" },
  { host: "dev01", mount: "/", usedGb: 12, env: "dev" },
  { host: "web01", mount: "/var", usedGb: 25, env: "prod" },
];
const used = pluck(disks, "usedGb");
type _t1 = Expect<Equal<typeof used, number[]>>;
check(used, [40, 300, 12, 25], "pluck");
const groups = groupBy(disks, (d) => d.env);
type _t2 = Expect<Equal<typeof groups, Partial<Record<"prod" | "dev", Disk[]>>>>;
check(groups.prod?.map((d) => d.host), ["web01", "db01", "web01"], "group prod");
check(groups.dev?.length, 1, "group dev");
const byHost = indexBy(disks, "host");
check(byHost.get("web01")?.mount, "/var", "later wins");
check(sumBy(disks, (d) => d.usedGb), 377, "sum");
@@hint
`groupBy`: `const out: Partial<Record<K, T[]>> = {}; for (const item of items) { const k = keyOf(item); (out[k] ??= []).push(item); }`.
@@solution
function pluck<T, K extends keyof T>(items: readonly T[], key: K): T[K][] {
  return items.map((item) => item[key]);
}

function groupBy<T, K extends PropertyKey>(items: readonly T[], keyOf: (item: T) => K): Partial<Record<K, T[]>> {
  const out: Partial<Record<K, T[]>> = {};
  for (const item of items) {
    const k = keyOf(item);
    (out[k] ??= []).push(item);
  }
  return out;
}

function indexBy<T, K extends keyof T>(items: readonly T[], key: K): Map<T[K], T> {
  return new Map(items.map((item) => [item[key], item] as const));
}

function sumBy<T>(items: readonly T[], value: (item: T) => number): number {
  return items.reduce((total, item) => total + value(item), 0);
}

@@exercise A generic LRU cache
Write `class LruCache<K, V>`, a least-recently-used cache:

- `constructor(capacity: number)`
- `get(key: K): V | undefined`: a hit makes the key most recently used
- `set(key: K, value: V): this`: inserting beyond capacity evicts the least recently used key
- `getOrCompute(key: K, compute: (key: K) => V): V`
- `keys(): K[]`: from least to most recently used
- `stats`: a getter returning `{ hits: number; misses: number }` (only `get` and `getOrCompute` count)

Hint: a `Map` remembers insertion order, so delete and re-insert to move a key to the end.
@@starter
class LruCache<K, V> {
  private readonly capacity: number;

  constructor(capacity: number) {
    this.capacity = capacity;
  }
}

@@tests
const cache = new LruCache<string, number>(2);
cache.set("a", 1).set("b", 2);
check(cache.get("a"), 1, "hit");
cache.set("c", 3);                                   // evicts b (a was used recently)
check(cache.keys(), ["a", "c"], "evicted LRU");
check(cache.get("b"), undefined, "miss");
let computed = 0;
const v = cache.getOrCompute("d", (k) => { computed++; return k.length * 10; });
const again = cache.getOrCompute("d", () => { computed++; return -1; });
check([v, again, computed], [10, 10, 1], "compute once");
check(cache.keys(), ["c", "d"], "order after compute");
check(cache.stats, { hits: 2, misses: 2 }, "stats");
@@hint
Keep fields declared and assigned in the constructor as in the starter: constructor parameter properties (`constructor(private capacity: number)`) aren't erasable syntax, so Node's type stripping can't run them. In `get`, `if (!this.map.has(key)) { this.misses++; return undefined; }`, then delete and re-set the key. In `set`, delete the key first, set it, and while `map.size > capacity` delete `this.map.keys().next().value`.
@@solution
class LruCache<K, V> {
  private readonly capacity: number;
  private readonly map = new Map<K, V>();
  private hits = 0;
  private misses = 0;

  constructor(capacity: number) {
    this.capacity = capacity;
  }

  get(key: K): V | undefined {
    if (!this.map.has(key)) {
      this.misses++;
      return undefined;
    }
    this.hits++;
    const value = this.map.get(key) as V;
    this.map.delete(key);
    this.map.set(key, value);
    return value;
  }

  set(key: K, value: V): this {
    this.map.delete(key);
    this.map.set(key, value);
    while (this.map.size > this.capacity) {
      const oldest = this.map.keys().next();
      if (oldest.done) break;
      this.map.delete(oldest.value);
    }
    return this;
  }

  getOrCompute(key: K, compute: (key: K) => V): V {
    if (this.map.has(key)) return this.get(key) as V;
    this.misses++;
    const value = compute(key);
    this.set(key, value);
    return value;
  }

  keys(): K[] {
    return [...this.map.keys()];
  }

  get stats(): { hits: number; misses: number } {
    return { hits: this.hits, misses: this.misses };
  }
}
