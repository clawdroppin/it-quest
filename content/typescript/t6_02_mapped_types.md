@@lesson t6-mapped Mapped Types, Modifiers and Key Remapping
@@concepts objects, maps
@@text
A **mapped type** builds a new object type by looping over keys. All the utility types you've used (`Partial`, `Readonly`, `Pick`, `Record`) are mapped types, and you can write your own.

## The basic loop

```ts
type Flags<T> = { [K in keyof T]: boolean };

interface Service { web: string; db: string; cache: string }
type Healthy = Flags<Service>;                 // { web: boolean; db: boolean; cache: boolean }
const now: Healthy = { web: true, db: true, cache: false };
console.log(now);
```

Read `[K in keyof T]` as "for each key K of T". The right side can use `K` and `T[K]`.

## Rebuilding the built-ins

```ts
type MyPartial<T> = { [K in keyof T]?: T[K] };
type MyReadonly<T> = { readonly [K in keyof T]: T[K] };
type MyPick<T, K extends keyof T> = { [P in K]: T[P] };
type MyRecord<K extends PropertyKey, V> = { [P in K]: V };

type Host = { name: string; ip: string; port: number };
const partial: MyPartial<Host> = { ip: "10.0.0.5" };
const picked: MyPick<Host, "name" | "port"> = { name: "web01", port: 443 };
console.log(partial, picked);
```

## Modifiers: adding and removing

Prefix `readonly` or `?` with `-` to **remove** them, or `+` (the default) to add them:

```ts
type Mutable<T> = { -readonly [K in keyof T]: T[K] };
type Complete<T> = { [K in keyof T]-?: T[K] };          // that's how Required<T> works

type Draft = { readonly id: number; title?: string };
const d: Mutable<Complete<Draft>> = { id: 1, title: "VPN" };
d.id = 2;                                                // allowed now
console.log(d);
```

Mapped types over `keyof T` are **homomorphic**: they keep the original modifiers unless you change them.

## Key remapping with `as`

Since TypeScript 4.1 you can rename keys, or filter them by remapping to `never`:

```ts
type Getters<T> = { [K in keyof T & string as `get${Capitalize<K>}`]: () => T[K] };
type OnlyStrings<T> = { [K in keyof T as T[K] extends string ? K : never]: T[K] };

type Config = { host: string; port: number; tls: boolean };
type ConfigGetters = Getters<Config>;      // { getHost: () => string; getPort: () => number; getTls: () => boolean }
type Texts = OnlyStrings<Config>;          // { host: string }
const t: Texts = { host: "db01" };
console.log(t);
```

`keyof T & string` keeps only string keys (keys can also be numbers or symbols), which template literals need.

## Mapping over unions and arrays

Mapped types also work on **tuples and arrays**: `{ [K in keyof T]: Promise<T[K]> }` with a tuple `T` gives a tuple of promises. That's how `Promise.all` keeps each element's type:

```ts
type Promised<T extends readonly unknown[]> = { [K in keyof T]: Promise<T[K]> };
type P = Promised<[string, number]>;     // [Promise<string>, Promise<number>]
const ps: P = [Promise.resolve("a"), Promise.resolve(1)];
Promise.all(ps).then(([s, n]) => console.log(s, n));
```

## The "map then index" trick

To get a **union** out of a mapped type, index it with `keyof`:

```ts
type Events = { login: { user: string }; logout: { user: string }; alert: { level: number } };
type EventMessage = { [K in keyof Events]: { type: K; payload: Events[K] } }[keyof Events];
// { type: "login"; payload: ... } | { type: "logout"; ... } | { type: "alert"; ... }
const m: EventMessage = { type: "alert", payload: { level: 3 } };
console.log(m.type);
```

This turns a lookup table into a discriminated union: a pattern you'll reuse constantly.

@@quiz
What does `{ [K in keyof T]-?: T[K] }` do?
- Makes every property optional
* Removes the optional modifier from every property (like `Required<T>`)
- Removes every property
- Makes properties readonly
? `-` removes a modifier; `+` (or nothing) adds it.

@@quiz
How do you drop some keys in a mapped type?
* Remap them to `never` with `as`
- Set their value type to `never`
- Use `delete`
- Use `Omit` only
? `[K in keyof T as Cond extends true ? K : never]` filters keys.

@@quiz
What does `{ [K in keyof E]: { type: K; payload: E[K] } }[keyof E]` produce?
- An object type
* A union of `{ type, payload }` variants, one per key of E
- An array type
- never
? Map, then index with keyof to collect the values as a union.

@@flashcards
Mapped type :: `{ [K in keyof T]: ... }`
Homomorphic mapped type :: maps over keyof T and keeps modifiers
`-readonly` :: remove readonly
`-?` :: remove optional
`as` in mapped types :: rename or filter keys
Filter keys :: remap to `never`
`keyof T & string` :: only the string keys
Map then index :: `{...}[keyof T]` turns a table into a union

@@exercise Getters and setters from a type
Write these types:

- `Getters<T>`: `getX(): T[X]` for every string key
- `Setters<T>`: `setX(value: T[X]): void` for every string key
- `Accessors<T> = Getters<T> & Setters<T>`

and a runtime function `accessors<T extends Record<string, unknown>>(initial: T): Accessors<T> & { snapshot(): T }` that keeps the state in a private copy of `initial`. `snapshot()` returns a copy, so changing it doesn't affect the state.
@@starter
type Getters<T> = unknown;
type Setters<T> = unknown;
type Accessors<T> = Getters<T> & Setters<T>;

function accessors<T extends Record<string, unknown>>(initial: T): Accessors<T> & { snapshot(): T } {
  throw new Error("TODO: not done yet");
}

@@tests
type Equal<X, Y> = (<T>() => T extends X ? 1 : 2) extends (<T>() => T extends Y ? 1 : 2) ? true : false;
type Expect<T extends true> = T;
type _t1 = Expect<Equal<Getters<{ host: string; port: number }>, { getHost: () => string; getPort: () => number }>>;
type _t2 = Expect<Equal<Setters<{ tls: boolean }>, { setTls: (value: boolean) => void }>>;
const cfg = accessors({ host: "db01", port: 5432 });
check([cfg.getHost(), cfg.getPort()], ["db01", 5432], "getters");
cfg.setPort(6543);
const snap = cfg.snapshot();
snap.host = "changed";
check(cfg.snapshot(), { host: "db01", port: 6543 }, "setter and snapshot copy");
@@hint
At runtime, loop over `Object.keys(initial)` and build `obj["get" + cap(key)] = () => state[key]` and `obj["set" + cap(key)] = (v) => { state[key] = v; }` on a `Record<string, unknown>`, then return it `as Accessors<T> & { snapshot(): T }`. The assertion is needed because TypeScript can't follow string building at runtime.
@@solution
type Getters<T> = { [K in keyof T & string as `get${Capitalize<K>}`]: () => T[K] };
type Setters<T> = { [K in keyof T & string as `set${Capitalize<K>}`]: (value: T[K]) => void };
type Accessors<T> = Getters<T> & Setters<T>;

function accessors<T extends Record<string, unknown>>(initial: T): Accessors<T> & { snapshot(): T } {
  const state: Record<string, unknown> = { ...initial };
  const cap = (s: string): string => s.charAt(0).toUpperCase() + s.slice(1);
  const obj: Record<string, unknown> = { snapshot: () => ({ ...state }) };
  for (const key of Object.keys(initial)) {
    obj[`get${cap(key)}`] = () => state[key];
    obj[`set${cap(key)}`] = (value: unknown) => {
      state[key] = value;
    };
  }
  return obj as Accessors<T> & { snapshot(): T };
}

@@exercise Typed config diff
Write:

- `type Change<T> = { [K in keyof T]: { key: K; from: T[K]; to: T[K] } }[keyof T]`: a union of possible changes
- `diff<T extends Record<string, unknown>>(before: T, after: T): Change<T>[]`: one entry per key whose value differs (compare with `JSON.stringify`, so arrays and objects compare by content), in the key order of `before`
- `type Mutable<T>` (removes readonly) and `applyChanges<T extends Record<string, unknown>>(base: Readonly<T>, changes: readonly Change<T>[]): T`: returns a new object with the changes applied (`to` values); `base` is not modified
@@starter
type Change<T> = unknown;
type Mutable<T> = T;

function diff<T extends Record<string, unknown>>(before: T, after: T): Change<T>[] {
  throw new Error("TODO: not done yet");
}

function applyChanges<T extends Record<string, unknown>>(base: Readonly<T>, changes: readonly Change<T>[]): T {
  throw new Error("TODO: not done yet");
}

@@tests
type Equal<X, Y> = (<T>() => T extends X ? 1 : 2) extends (<T>() => T extends Y ? 1 : 2) ? true : false;
type Expect<T extends true> = T;
type Cfg = { host: string; port: number; tags: string[] };
type _t1 = Expect<Equal<Change<{ a: number; b: string }>, { key: "a"; from: number; to: number } | { key: "b"; from: string; to: string }>>;
type _t2 = Expect<Equal<Mutable<{ readonly id: number }>, { id: number }>>;
const before: Cfg = { host: "db01", port: 5432, tags: ["prod"] };
const after: Cfg = { host: "db01", port: 6543, tags: ["prod", "eu"] };
const changes = diff(before, after);
check(changes, [{ key: "port", from: 5432, to: 6543 }, { key: "tags", from: ["prod"], to: ["prod", "eu"] }], "diff");
check(diff(before, { ...before, tags: ["prod"] }), [], "same content");
const applied = applyChanges(before, changes);
check([applied, before.port], [after, 5432], "apply without mutating");
@@hint
In `diff`, iterate `Object.keys(before) as (keyof T)[]` and push `{ key, from: before[key], to: after[key] }`. TypeScript can't see that this object fits the union for a generic key, so push it `as Change<T>`. In `applyChanges`, copy into `const out = { ...base } as Mutable<T>` and assign `out[c.key] = c.to`.
@@solution
type Change<T> = { [K in keyof T]: { key: K; from: T[K]; to: T[K] } }[keyof T];
type Mutable<T> = { -readonly [K in keyof T]: T[K] };

function diff<T extends Record<string, unknown>>(before: T, after: T): Change<T>[] {
  const out: Change<T>[] = [];
  for (const key of Object.keys(before) as (keyof T)[]) {
    if (JSON.stringify(before[key]) !== JSON.stringify(after[key])) {
      out.push({ key, from: before[key], to: after[key] } as Change<T>);
    }
  }
  return out;
}

function applyChanges<T extends Record<string, unknown>>(base: Readonly<T>, changes: readonly Change<T>[]): T {
  const out = { ...base } as Mutable<T>;
  for (const c of changes) {
    const { key, to } = c as { key: keyof T; to: T[keyof T] };
    out[key] = to;
  }
  return out as T;
}
