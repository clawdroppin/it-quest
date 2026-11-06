@@lesson t6-conditional Conditional Types and infer
@@concepts functions
@@text
A **conditional type** chooses between two types: `T extends U ? X : Y`. Combined with `infer`, it lets you take types apart: the element type of an array, the value inside a promise, a function's parameters.

## The basics

```ts
type IsString<T> = T extends string ? "yes" : "no";
type A = IsString<"web01">;      // "yes"
type B = IsString<443>;          // "no"

type ApiResult<T> = T extends Error ? { ok: false; error: string } : { ok: true; value: T };
const r: ApiResult<number> = { ok: true, value: 42 };
console.log(r);
```

`extends` here means "is assignable to", the same test as in constraints.

## Distribution over unions

When the checked type is a **bare type parameter** and you pass a union, the condition runs once per member and the results are joined:

```ts
type NonNull<T> = T extends null | undefined ? never : T;
type C = NonNull<string | null | undefined>;      // string

type ToArray<T> = T extends unknown ? T[] : never;
type D = ToArray<string | number>;                // string[] | number[]

type ToArrayNonDist<T> = [T] extends [unknown] ? T[] : never;
type E = ToArrayNonDist<string | number>;         // (string | number)[]
const e: E = ["a", 1];
console.log(e);
```

Distribution is how `Exclude<T, U>` and `Extract<T, U>` work. Wrap both sides in `[ ]` to turn it off.

## infer: pattern matching on types

`infer X` declares a type variable that TypeScript fills in when the pattern matches:

```ts
type ElementOf<T> = T extends readonly (infer E)[] ? E : never;
type Unwrap<T> = T extends Promise<infer V> ? Unwrap<V> : T;      // recursive, like Awaited<T>
type FirstArg<F> = F extends (first: infer A, ...rest: never[]) => unknown ? A : never;

type F = ElementOf<string[]>;                          // string
type G = Unwrap<Promise<Promise<number>>>;             // number
type H = FirstArg<(host: string, port: number) => void>;   // string
const g: G = 5;
console.log(g);
```

The standard library ships many of these: `ReturnType<F>`, `Parameters<F>`, `ConstructorParameters<C>`, `InstanceType<C>`, `Awaited<T>`, `NonNullable<T>`, `Exclude`, `Extract`.

```ts
async function fetchHost(id: number) { return { id, name: "web01", tags: ["prod"] }; }
type Host = Awaited<ReturnType<typeof fetchHost>>;   // { id: number; name: string; tags: string[] }
type Args = Parameters<typeof fetchHost>;            // [id: number]
const h: Host = { id: 1, name: "x", tags: [] };
console.log(h.name);
```

Deriving types from functions like this keeps them in sync automatically: change `fetchHost` and `Host` follows.

## infer with constraints

`infer X extends string` (TypeScript 4.7) only matches when X fits the constraint:

```ts
type NumericKey<T> = T extends `${infer N extends number}` ? N : never;
type I = NumericKey<"443">;      // 443 (a number literal type)
type J = NumericKey<"ssh">;      // never
const port: I = 443;
console.log(port);
```

## Conditional return types

A function can return a type that depends on its argument:

```ts
type Parsed<K extends "int" | "bool" | "text"> = K extends "int" ? number : K extends "bool" ? boolean : string;

function parseAs<K extends "int" | "bool" | "text">(kind: K, raw: string): Parsed<K> {
  const value = kind === "int" ? Number.parseInt(raw, 10) : kind === "bool" ? raw === "true" : raw;
  return value as Parsed<K>;                    // TS can't narrow K inside, so assert once
}
const n = parseAs("int", "42");                 // number
const b = parseAs("bool", "true");              // boolean
console.log(n + 1, b);
```

Inside the body TypeScript can't prove which branch applies for a generic `K`, so implementations of conditional return types usually need one assertion. Keep it in one well-tested place.

@@quiz
What is `Exclude<"a" | "b" | "c", "a">`?
- "a"
* `"b" | "c"`
- never
- string
? Exclude distributes: each member that extends "a" becomes never.

@@quiz
What does `infer` do in `T extends Promise<infer V> ? V : T`?
* Captures the type inside the Promise as V when T matches
- Converts T to a Promise
- Infers the runtime value
- Makes V optional
? infer is pattern matching for types.

@@quiz
How do you stop a conditional type from distributing over a union?
- Use `infer`
* Wrap both sides in square brackets: `[T] extends [U]`
- Use `never`
- It can't be stopped
? Distribution only happens for a bare type parameter.

@@quiz
`type Host = Awaited<ReturnType<typeof fetchHost>>`. What's the benefit?
* The type stays in sync with the function automatically
- It's faster at runtime
- It validates the data
- It makes fetchHost synchronous
? Derive types from a single source of truth.

@@drill
Which keyword captures a type inside a conditional type pattern?
= infer
? T extends Promise<infer U> ? U : T.

@@flashcards
Conditional type :: `T extends U ? X : Y`
Distributive conditional :: applies per union member for a bare type parameter
`[T] extends [U]` :: non-distributive check
`infer X` :: capture part of a matched type
`ReturnType<F>` / `Parameters<F>` :: a function's result / argument tuple
`Awaited<T>` :: unwraps promises recursively
`Exclude<T, U>` / `Extract<T, U>` :: remove / keep union members
`infer N extends number` :: match only if it fits the constraint
Conditional return type :: result type depends on the argument

@@exercise Calling methods by name
Write these types:

- `FunctionKeys<T>`: the keys of T whose values are functions
- `ArgsOf<F>` and `ResultOf<F>`: a function's parameter tuple and return type (write them with `infer`, not with `Parameters`/`ReturnType`)

and the functions:

- `methodsOf<T extends object>(obj: T): string[]`: the sorted names of own properties that are functions
- `invoke<T, K extends FunctionKeys<T>>(obj: T, key: K, ...args: ArgsOf<T[K]>): ResultOf<T[K]>`: calls the method with `this` set to `obj`
@@starter
type FunctionKeys<T> = keyof T;
type ArgsOf<F> = unknown[];
type ResultOf<F> = unknown;

function methodsOf<T extends object>(obj: T): string[] {
  throw new Error("TODO: not done yet");
}

function invoke<T, K extends FunctionKeys<T>>(obj: T, key: K, ...args: ArgsOf<T[K]>): ResultOf<T[K]> {
  throw new Error("TODO: not done yet");
}

@@tests
type Equal<X, Y> = (<T>() => T extends X ? 1 : 2) extends (<T>() => T extends Y ? 1 : 2) ? true : false;
type Expect<T extends true> = T;
const server = {
  name: "web01",
  load: 0.4,
  restart(reason: string): string { return `${this.name} restarted: ${reason}`; },
  scale(by: number, max = 10): number { return Math.min(max, 2 * by); },
};
type _t1 = Expect<Equal<FunctionKeys<typeof server>, "restart" | "scale">>;
type _t2 = Expect<Equal<ArgsOf<typeof server.scale>, [by: number, max?: number]>>;
type _t3 = Expect<Equal<ResultOf<typeof server.restart>, string>>;
check(methodsOf(server), ["restart", "scale"], "methods");
check(invoke(server, "restart", "patch"), "web01 restarted: patch", "invoke with this");
check([invoke(server, "scale", 3), invoke(server, "scale", 8, 12)], [6, 12], "optional args");
@@hint
`type FunctionKeys<T> = { [K in keyof T]-?: T[K] extends (...args: any[]) => unknown ? K : never }[keyof T];` (map then index). For `invoke`, TypeScript can't call a generic `T[K]` directly: get it with `const fn = obj[key] as unknown as (...a: unknown[]) => ResultOf<T[K]>` and call `fn.apply(obj, args as unknown as unknown[])`.
@@solution
type FunctionKeys<T> = { [K in keyof T]-?: T[K] extends (...args: any[]) => unknown ? K : never }[keyof T];
type ArgsOf<F> = F extends (...args: infer A) => unknown ? A : never;
type ResultOf<F> = F extends (...args: any[]) => infer R ? R : never;

function methodsOf<T extends object>(obj: T): string[] {
  return Object.entries(obj)
    .filter(([, value]) => typeof value === "function")
    .map(([key]) => key)
    .sort();
}

function invoke<T, K extends FunctionKeys<T>>(obj: T, key: K, ...args: ArgsOf<T[K]>): ResultOf<T[K]> {
  const fn = obj[key] as unknown as (...a: unknown[]) => ResultOf<T[K]>;
  return fn.apply(obj, args as unknown as unknown[]);
}

@@exercise Settings with an inferred type
A schema maps setting names to kinds: `"int"`, `"bool"`, `"text"` or `"list"` (comma-separated text). Write:

- `type KindToType<K>`: `"int"` → number, `"bool"` → boolean, `"text"` → string, `"list"` → string[]
- `type SettingsOf<S extends Record<string, Kind>> = { [P in keyof S]: KindToType<S[P]> }`
- `parseSettings<const S extends Record<string, Kind>>(schema: S, raw: Record<string, string | undefined>): SettingsOf<S>`

Parsing rules: `int` must match `/^-?\d+$/`; `bool` accepts `true/false/yes/no/1/0` (case-insensitive); `list` splits on commas, trims and drops empty items; a missing value is an error. Collect all errors as `"<name>: <problem>"` (problems: `missing`, `not an int`, `not a bool`) and throw one `Error` joined with `"; "`, in schema order.
@@starter
type Kind = "int" | "bool" | "text" | "list";
type KindToType<K> = unknown;
type SettingsOf<S extends Record<string, Kind>> = { [P in keyof S]: KindToType<S[P]> };

function parseSettings<const S extends Record<string, Kind>>(schema: S, raw: Record<string, string | undefined>): SettingsOf<S> {
  throw new Error("TODO: not done yet");
}

@@tests
type Equal<X, Y> = (<T>() => T extends X ? 1 : 2) extends (<T>() => T extends Y ? 1 : 2) ? true : false;
type Expect<T extends true> = T;
const schema = { port: "int", debug: "bool", host: "text", peers: "list" } as const;
const s = parseSettings(schema, { port: "8080", debug: "YES", host: "db01", peers: "a, b,,c" });
type _t1 = Expect<Equal<typeof s.port, number>>;
type _t2 = Expect<Equal<typeof s.peers, string[]>>;
check(s, { port: 8080, debug: true, host: "db01", peers: ["a", "b", "c"] }, "parsed");
let msg = "";
try { parseSettings(schema, { port: "80x", debug: "maybe", peers: "" }); } catch (e) { msg = (e as Error).message; }
check(msg, "port: not an int; debug: not a bool; host: missing", "errors");
@@hint
Loop over `Object.entries(schema)`, fill a `Record<string, unknown>` and collect errors; at the end `return out as SettingsOf<S>`. Note an empty `peers` value is valid and gives `[]`; only `undefined` is missing.
@@solution
type Kind = "int" | "bool" | "text" | "list";
type KindToType<K> = K extends "int" ? number : K extends "bool" ? boolean : K extends "list" ? string[] : K extends "text" ? string : never;
type SettingsOf<S extends Record<string, Kind>> = { [P in keyof S]: KindToType<S[P]> };

const BOOLS: Record<string, boolean> = { true: true, yes: true, "1": true, false: false, no: false, "0": false };

function parseSettings<const S extends Record<string, Kind>>(schema: S, raw: Record<string, string | undefined>): SettingsOf<S> {
  const out: Record<string, unknown> = {};
  const errors: string[] = [];
  for (const [name, kind] of Object.entries(schema)) {
    const value = raw[name];
    if (value === undefined) {
      errors.push(`${name}: missing`);
      continue;
    }
    if (kind === "int") {
      if (/^-?\d+$/.test(value)) out[name] = Number(value);
      else errors.push(`${name}: not an int`);
    } else if (kind === "bool") {
      const b = BOOLS[value.toLowerCase()];
      if (b === undefined) errors.push(`${name}: not a bool`);
      else out[name] = b;
    } else if (kind === "list") {
      out[name] = value.split(",").map((x) => x.trim()).filter((x) => x !== "");
    } else {
      out[name] = value;
    }
  }
  if (errors.length > 0) throw new Error(errors.join("; "));
  return out as SettingsOf<S>;
}
