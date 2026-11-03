@@lesson t6-recursive Recursive Types and Type-Level Programming
@@concepts functions
@@text
Types can refer to themselves. That's needed for trees and JSON, and it turns the type system into a small (pure, functional) programming language. This lesson shows the useful parts and where to stop.

## Recursive data types

```ts
type Json = string | number | boolean | null | Json[] | { [key: string]: Json };

interface MenuItem { label: string; children?: MenuItem[] }
const menu: MenuItem = { label: "Admin", children: [{ label: "Users" }, { label: "Audit log" }] };

const countItems = (m: MenuItem): number => 1 + (m.children ?? []).reduce((n, c) => n + countItems(c), 0);
console.log(countItems(menu));
```

## Deep utility types

`Partial` and `Readonly` only affect the top level. Recursive versions go all the way down:

```ts
type DeepReadonly<T> =
  T extends (...args: never[]) => unknown ? T
  : T extends readonly (infer E)[] ? readonly DeepReadonly<E>[]
  : T extends object ? { readonly [K in keyof T]: DeepReadonly<T[K]> }
  : T;

type DeepPartial<T> =
  T extends readonly unknown[] ? T
  : T extends object ? { [K in keyof T]?: DeepPartial<T[K]> }
  : T;

type Cfg = { db: { host: string; pool: { min: number; max: number } }; tags: string[] };
const patch: DeepPartial<Cfg> = { db: { pool: { max: 20 } } };
const frozen: DeepReadonly<Cfg> = { db: { host: "x", pool: { min: 1, max: 5 } }, tags: [] };
// frozen.db.pool.max = 9;        // error: readonly
console.log(patch, frozen.tags.length);
```

Note the order of checks: functions first (leave them alone), then arrays, then objects, then primitives.

## Dotted paths

A classic: all the valid "a.b.c" paths into an object, so `get(config, "db.pool.max")` is checked and typed:

```ts
type Paths<T> = T extends readonly unknown[] ? never
  : T extends object ? { [K in keyof T & string]: K | `${K}.${Paths<T[K]>}` }[keyof T & string]
  : never;

type PathValue<T, P extends string> =
  P extends `${infer K}.${infer Rest}` ? (K extends keyof T ? PathValue<T[K], Rest> : never)
  : P extends keyof T ? T[P] : never;

type Cfg = { db: { host: string; pool: { max: number } } };
type P = Paths<Cfg>;                          // "db" | "db.host" | "db.pool" | "db.pool.max"
type V = PathValue<Cfg, "db.pool.max">;       // number
const v: V = 10;
console.log(v);
```

## Tuples as lists

Tuple types support spreading and `infer` on positions, so you can write list operations:

```ts
type Head<T extends readonly unknown[]> = T extends readonly [infer H, ...unknown[]] ? H : never;
type Tail<T extends readonly unknown[]> = T extends readonly [unknown, ...infer R] ? R : [];
type Reverse<T extends readonly unknown[]> = T extends readonly [infer H, ...infer R] ? [...Reverse<R>, H] : [];
type Length<T extends readonly unknown[]> = T["length"];

type R = Reverse<["a", "b", "c"]>;        // ["c", "b", "a"]
type L = Length<[1, 2, 3]>;               // 3
const r: R = ["c", "b", "a"];
console.log(r);
```

This is how typed `pipe`/`compose` functions, `zip`, and argument-currying helpers are built.

## Limits and good judgement

- TypeScript limits recursion depth (around 50 levels for non-tail recursion, 1000 for **tail-recursive** conditional types). Errors like "Type instantiation is excessively deep" mean you hit it.
- Complex types slow down the editor for everyone. Measure with `tsc --extendedDiagnostics` and `--generateTrace`.
- Every clever type needs **type tests**, and a plain-English comment explaining it.
- If a type is hard to read, consider a simpler API: two functions instead of one overloaded generic.

@@quiz
Why does `DeepReadonly` check for functions first?
* Mapping over a function's keys would turn it into a plain object type and lose its call signature
- Functions can't be readonly
- For performance only
- It doesn't matter
? Leave functions (and usually arrays) for special handling.

@@quiz
What does `Reverse<[1, 2, 3]>` produce with the lesson's definition?
- [1, 2, 3]
* [3, 2, 1]
- number[]
- never
? It takes the head and appends it after the reversed tail.

@@quiz
What does "Type instantiation is excessively deep and possibly infinite" mean?
- A runtime stack overflow
* A recursive type exceeded the compiler's depth limit
- A missing import
- A syntax error
? Simplify, make it tail-recursive, or narrow the input types.

@@flashcards
Recursive type :: a type that refers to itself (trees, JSON)
`DeepPartial<T>` :: optional at every level
`DeepReadonly<T>` :: readonly at every level
`Paths<T>` :: union of dotted paths into T
`PathValue<T, P>` :: the type at a dotted path
`[infer H, ...infer R]` :: split a tuple into head and rest
`T["length"]` :: a tuple's length as a number literal type
Tail recursion :: deeper limit (1000) for conditional types
`--extendedDiagnostics` :: measure type-checking cost

@@exercise Deep merge for config overrides
Write `DeepPartial<T>` (arrays are replaced, not merged, so they're left as-is) and `deepMerge<T extends object>(base: T, patch: DeepPartial<T>): T`:

- plain objects merge recursively
- arrays and primitives in the patch replace the base value
- `undefined` in the patch means "keep the base value"
- neither argument is modified
@@starter
type DeepPartial<T> = Partial<T>;

function deepMerge<T extends object>(base: T, patch: DeepPartial<T>): T {
  throw new Error("TODO: not done yet");
}

@@tests
type Equal<X, Y> = (<T>() => T extends X ? 1 : 2) extends (<T>() => T extends Y ? 1 : 2) ? true : false;
type Expect<T extends true> = T;
type Cfg = { db: { host: string; pool: { min: number; max: number } }; tags: string[]; debug: boolean };
type _t1 = Expect<Equal<DeepPartial<{ a: { b: number }; c: string[] }>, { a?: { b?: number }; c?: string[] }>>;
const base: Cfg = { db: { host: "localhost", pool: { min: 1, max: 5 } }, tags: ["dev"], debug: false };
const merged = deepMerge(base, { db: { pool: { max: 20 } }, tags: ["prod", "eu"], debug: undefined });
check(merged, { db: { host: "localhost", pool: { min: 1, max: 20 } }, tags: ["prod", "eu"], debug: false }, "merged");
check(base.db.pool.max, 5, "base untouched");
check(deepMerge(base, {}) !== base, true, "returns a copy");
@@hint
A helper `isPlainObject(v: unknown): v is Record<string, unknown>` (`typeof v === "object" && v !== null && !Array.isArray(v)`) decides when to recurse. Work on `Record<string, unknown>` copies internally and return `out as T`.
@@solution
type DeepPartial<T> =
  T extends readonly unknown[] ? T
  : T extends object ? { [K in keyof T]?: DeepPartial<T[K]> }
  : T;

const isPlainObject = (v: unknown): v is Record<string, unknown> => typeof v === "object" && v !== null && !Array.isArray(v);

function mergeRecords(base: Record<string, unknown>, patch: Record<string, unknown>): Record<string, unknown> {
  const out: Record<string, unknown> = { ...base };
  for (const [key, value] of Object.entries(patch)) {
    if (value === undefined) continue;
    const current = out[key];
    out[key] = isPlainObject(current) && isPlainObject(value) ? mergeRecords(current, value) : value;
  }
  return out;
}

function deepMerge<T extends object>(base: T, patch: DeepPartial<T>): T {
  return mergeRecords({ ...(base as Record<string, unknown>) }, { ...(patch as Record<string, unknown>) }) as T;
}

@@exercise Typed dotted paths
Write `Paths<T>` and `PathValue<T, P>` as in the lesson, and:

- `getPath<T, P extends Paths<T> & string>(obj: T, path: P): PathValue<T, P>`
- `setPath<T, P extends Paths<T> & string>(obj: T, path: P, value: PathValue<T, P>): T`: returns a **new** object with the value replaced (copy each object along the path; leave the rest shared)
- `listPaths(obj: object): string[]`: all leaf paths (values that aren't plain objects) in depth-first order
@@starter
type Paths<T> = string;
type PathValue<T, P extends string> = unknown;

function getPath<T, P extends Paths<T> & string>(obj: T, path: P): PathValue<T, P> {
  throw new Error("TODO: not done yet");
}

function setPath<T, P extends Paths<T> & string>(obj: T, path: P, value: PathValue<T, P>): T {
  throw new Error("TODO: not done yet");
}

function listPaths(obj: object): string[] {
  throw new Error("TODO: not done yet");
}

@@tests
type Equal<X, Y> = (<T>() => T extends X ? 1 : 2) extends (<T>() => T extends Y ? 1 : 2) ? true : false;
type Expect<T extends true> = T;
type Cfg = { db: { host: string; pool: { max: number } }; tags: string[] };
type _t1 = Expect<Equal<Paths<Cfg>, "db" | "db.host" | "db.pool" | "db.pool.max" | "tags">>;
type _t2 = Expect<Equal<PathValue<Cfg, "db.pool">, { max: number }>>;
const cfg: Cfg = { db: { host: "db01", pool: { max: 5 } }, tags: ["prod"] };
const max = getPath(cfg, "db.pool.max");
type _t3 = Expect<Equal<typeof max, number>>;
check([max, getPath(cfg, "db.host")], [5, "db01"], "get");
const next = setPath(cfg, "db.pool.max", 50);
check([next.db.pool.max, cfg.db.pool.max, next.tags === cfg.tags], [50, 5, true], "set copies the path only");
check(listPaths(cfg), ["db.host", "db.pool.max", "tags"], "leaf paths");
@@hint
`getPath`: `path.split(".").reduce<unknown>((o, k) => (o as Record<string, unknown>)[k], obj) as PathValue<T, P>`. `setPath` recursion on `Record<string, unknown>`: `const [head, ...rest] = keys; return { ...o, [head]: rest.length ? setIn(o[head] as Record<string, unknown>, rest, v) : v };`.
@@solution
type Paths<T> = T extends readonly unknown[] ? never
  : T extends object ? { [K in keyof T & string]: K | `${K}.${Paths<T[K]>}` }[keyof T & string]
  : never;

type PathValue<T, P extends string> =
  P extends `${infer K}.${infer Rest}` ? (K extends keyof T ? PathValue<T[K], Rest> : never)
  : P extends keyof T ? T[P] : never;

function getPath<T, P extends Paths<T> & string>(obj: T, path: P): PathValue<T, P> {
  return path.split(".").reduce<unknown>((o, k) => (o as Record<string, unknown>)[k], obj) as PathValue<T, P>;
}

function setIn(o: Record<string, unknown>, keys: string[], value: unknown): Record<string, unknown> {
  const [head, ...rest] = keys;
  if (head === undefined) return o;
  return { ...o, [head]: rest.length > 0 ? setIn(o[head] as Record<string, unknown>, rest, value) : value };
}

function setPath<T, P extends Paths<T> & string>(obj: T, path: P, value: PathValue<T, P>): T {
  return setIn(obj as Record<string, unknown>, path.split("."), value) as T;
}

function listPaths(obj: object, prefix = ""): string[] {
  const out: string[] = [];
  for (const [key, value] of Object.entries(obj)) {
    const path = prefix ? `${prefix}.${key}` : key;
    if (typeof value === "object" && value !== null && !Array.isArray(value)) out.push(...listPaths(value, path));
    else out.push(path);
  }
  return out;
}
