@@lesson t6-template-types Template Literal Types: Typed Strings, Routes and Events
@@concepts strings
@@text
Template literal types bring string manipulation into the type system. They power typed routes in web frameworks, typed event names, CSS-in-TS units and environment variable names.

## Building strings from unions

```ts
type Env = "dev" | "prod";
type Service = "api" | "db";
type Host = `${Service}-${Env}.corp.local`;      // 4 combinations
type Size = `${number}px` | `${number}%`;        // any number followed by px or %

const h: Host = "db-prod.corp.local";
const width: Size = "50%";
console.log(h, width);
```

A union in a placeholder multiplies out: every combination is generated. Large unions crossed with each other can get slow; keep them reasonable.

## Intrinsic string types

`Uppercase<S>`, `Lowercase<S>`, `Capitalize<S>` and `Uncapitalize<S>` transform literal types:

```ts
type EventName = "connect" | "disconnect";
type HandlerName = `on${Capitalize<EventName>}`;          // "onConnect" | "onDisconnect"
type EnvVar = `APP_${Uppercase<"port" | "host">}`;       // "APP_PORT" | "APP_HOST"
const handler: HandlerName = "onConnect";
const v: EnvVar = "APP_HOST";
console.log(handler, v);
```

## Parsing strings with infer

`infer` works inside template literal types, so you can split strings at the type level:

```ts
type Split<S extends string, D extends string> =
  S extends `${infer Head}${D}${infer Tail}` ? [Head, ...Split<Tail, D>] : [S];

type Parts = Split<"web01.prod.corp", ".">;        // ["web01", "prod", "corp"]
type Domain<S> = S extends `${string}@${infer D}` ? D : never;
type D1 = Domain<"ana@corp.example">;            // "corp.example"
const parts: Parts = ["web01", "prod", "corp"];
console.log(parts.length);
```

`infer Head` matches as little as possible (up to the first delimiter), `Tail` takes the rest.

## Example: typed route parameters

Web frameworks (Express types, Hono, tRPC, React Router) use this trick to give `req.params` exact keys from the route string:

```ts
type ParamNames<P extends string> =
  P extends `${string}:${infer Name}/${infer Rest}` ? Name | ParamNames<`/${Rest}`>
  : P extends `${string}:${infer Name}` ? Name
  : never;
type Params<P extends string> = { [K in ParamNames<P>]: string };

type T = Params<"/sites/:site/racks/:rack">;     // { site: string; rack: string }
const params: T = { site: "tokyo", rack: "r12" };
console.log(params);
```

## Example: typed event maps

Combine mapped types, key remapping and `Capitalize`:

```ts
type Events = { connect: { host: string }; error: { code: number } };
type Handlers<E> = { [K in keyof E & string as `on${Capitalize<K>}`]?: (payload: E[K]) => void };

const h: Handlers<Events> = {
  onConnect: ({ host }) => console.log("connected to", host),   // payload typed automatically
  onError: ({ code }) => console.log("error", code),
};
h.onConnect?.({ host: "db01" });
```

## When to stop

Type-level string parsing is powerful but costs compile time and readability. Use it at **boundaries** where a typo would be expensive (routes, event names, config keys), keep the types in a small module with type tests, and don't parse whole languages in the type system.

@@quiz
What is `` `${"a" | "b"}-${"x" | "y"}` ``?
- `"a-x"`
* `"a-x" | "a-y" | "b-x" | "b-y"`
- string
- An error
? Unions in placeholders produce every combination.

@@quiz
What does `S extends `${infer A}.${infer B}` ? A : never` give for `"web01.prod.corp"`?
* `"web01"` (A matches up to the first dot)
- `"web01.prod"`
- `"corp"`
- never
? The leftmost infer matches as little as possible.

@@quiz
Which type turns `"error"` into `"onError"`?
- `` `on${Uppercase<"error">}` ``
* `` `on${Capitalize<"error">}` ``
- `` `on${"error"}` ``
- `Capitalize<"onerror">`
? Capitalize upper-cases the first character only.

@@flashcards
Template literal type :: `` `prefix-${T}` ``
Union in a placeholder :: produces every combination
`${number}` :: any numeric string
`Uppercase` / `Lowercase` :: change the case of a literal type
`Capitalize` / `Uncapitalize` :: change the first character
infer in template literals :: split strings at the type level
Typed route params :: extract `:name` segments from the path
Event handler names :: `` `on${Capitalize<K>}` ``

@@exercise Typed route matching
Write `ParamNames<P>` and `Params<P>` (as in the lesson), and:

- `matchRoute<P extends string>(pattern: P, path: string): Params<P> | null`: segments split on `/` must match one to one; `:name` segments capture the (URL-decoded) value; other segments must be equal
- `fillRoute<P extends string>(pattern: P, params: Params<P>): string`: the reverse, URL-encoding each value
@@starter
type ParamNames<P extends string> = string;
type Params<P extends string> = { [K in ParamNames<P>]: string };

function matchRoute<P extends string>(pattern: P, path: string): Params<P> | null {
  throw new Error("TODO: not done yet");
}

function fillRoute<P extends string>(pattern: P, params: Params<P>): string {
  throw new Error("TODO: not done yet");
}

@@tests
type Equal<X, Y> = (<T>() => T extends X ? 1 : 2) extends (<T>() => T extends Y ? 1 : 2) ? true : false;
type Expect<T extends true> = T;
type _t1 = Expect<Equal<ParamNames<"/sites/:site/racks/:rack">, "site" | "rack">>;
type _t2 = Expect<Equal<ParamNames<"/health">, never>>;
const m = matchRoute("/sites/:site/racks/:rack", "/sites/osaka%201/racks/r7");
check(m, { site: "osaka 1", rack: "r7" }, "match");
check(matchRoute("/sites/:site", "/sites/a/b"), null, "segment count");
check(matchRoute("/sites/:site/racks", "/sites/a/rooms"), null, "literal mismatch");
check(matchRoute("/health", "/health"), {}, "no params");
check(fillRoute("/sites/:site/racks/:rack", { site: "kyoto dc", rack: "r1" }), "/sites/kyoto%20dc/racks/r1", "fill");
@@hint
`const a = pattern.split("/"), b = path.split("/"); if (a.length !== b.length) return null;` then walk both: for `seg.startsWith(":")` store `decodeURIComponent(b[i])` under `seg.slice(1)`. Build the result in a `Record<string, string>` and return it `as Params<P>`. For `fillRoute`, read values through `const values: Record<string, string> = params;`.
@@solution
type ParamNames<P extends string> =
  P extends `${string}:${infer Name}/${infer Rest}` ? Name | ParamNames<`/${Rest}`>
  : P extends `${string}:${infer Name}` ? Name
  : never;
type Params<P extends string> = { [K in ParamNames<P>]: string };

function matchRoute<P extends string>(pattern: P, path: string): Params<P> | null {
  const want = pattern.split("/");
  const got = path.split("/");
  if (want.length !== got.length) return null;
  const out: Record<string, string> = {};
  for (let i = 0; i < want.length; i++) {
    const seg = want[i] ?? "";
    const actual = got[i] ?? "";
    if (seg.startsWith(":")) out[seg.slice(1)] = decodeURIComponent(actual);
    else if (seg !== actual) return null;
  }
  return out as Params<P>;
}

function fillRoute<P extends string>(pattern: P, params: Params<P>): string {
  const values: Record<string, string> = params;
  return pattern
    .split("/")
    .map((seg) => (seg.startsWith(":") ? encodeURIComponent(values[seg.slice(1)] ?? "") : seg))
    .join("/");
}

@@exercise Environment variable names
Config objects use camelCase keys; environment variables use SCREAMING_SNAKE_CASE with a prefix. Write:

- `type ScreamingSnake<S extends string>`: `"dbHost"` → `"DB_HOST"`, `"maxRetries"` → `"MAX_RETRIES"`, `"port"` → `"PORT"` (an uppercase letter starts a new word)
- `type EnvKeys<T, Prefix extends string>`: a union of `` `${Prefix}${ScreamingSnake<K>}` `` for every string key K of T
- `envKey<const S extends string, const Prefix extends string>(prefix: Prefix, key: S): `${Prefix}${ScreamingSnake<S>}``
- `fromEnv<T extends Record<string, string>>(defaults: T, prefix: string, env: Record<string, string | undefined>): T`: each key takes `env[envKey(prefix, key)]` if set, else the default
@@starter
type ScreamingSnake<S extends string> = string;
type EnvKeys<T, Prefix extends string> = string;

function envKey<const S extends string, const Prefix extends string>(prefix: Prefix, key: S): `${Prefix}${ScreamingSnake<S>}` {
  throw new Error("TODO: not done yet");
}

function fromEnv<T extends Record<string, string>>(defaults: T, prefix: string, env: Record<string, string | undefined>): T {
  throw new Error("TODO: not done yet");
}

@@tests
type Equal<X, Y> = (<T>() => T extends X ? 1 : 2) extends (<T>() => T extends Y ? 1 : 2) ? true : false;
type Expect<T extends true> = T;
type _t1 = Expect<Equal<ScreamingSnake<"dbHost">, "DB_HOST">>;
type _t2 = Expect<Equal<ScreamingSnake<"maxRetries">, "MAX_RETRIES">>;
type _t3 = Expect<Equal<EnvKeys<{ dbHost: string; port: string }, "APP_">, "APP_DB_HOST" | "APP_PORT">>;
const k = envKey("APP_", "dbHost");
type _t4 = Expect<Equal<typeof k, "APP_DB_HOST">>;
check([k, envKey("X_", "maxRetries"), envKey("", "port")], ["APP_DB_HOST", "X_MAX_RETRIES", "PORT"], "env keys");
const cfg = fromEnv({ dbHost: "localhost", logLevel: "info" }, "APP_", { APP_DB_HOST: "db01", OTHER: "x" });
check(cfg, { dbHost: "db01", logLevel: "info" }, "from env");
@@hint
Recurse one character at a time: `S extends `${infer C}${infer Rest}` ? `${C extends Lowercase<C> ? Uppercase<C> : `_${C}`}${ScreamingSnake<Rest>}` : ""`. (A character that's already lowercase gets upper-cased; an uppercase one gets an underscore.) At runtime: `key.replace(/[A-Z]/g, (c) => "_" + c).toUpperCase()`.
@@solution
type ScreamingSnake<S extends string> =
  S extends `${infer C}${infer Rest}` ? `${C extends Lowercase<C> ? Uppercase<C> : `_${C}`}${ScreamingSnake<Rest>}` : "";
type EnvKeys<T, Prefix extends string> = { [K in keyof T & string]: `${Prefix}${ScreamingSnake<K>}` }[keyof T & string];

function envKey<const S extends string, const Prefix extends string>(prefix: Prefix, key: S): `${Prefix}${ScreamingSnake<S>}` {
  return `${prefix}${key.replace(/[A-Z]/g, (c) => `_${c}`).toUpperCase()}` as `${Prefix}${ScreamingSnake<S>}`;
}

function fromEnv<T extends Record<string, string>>(defaults: T, prefix: string, env: Record<string, string | undefined>): T {
  const out: Record<string, string> = { ...defaults };
  for (const key of Object.keys(defaults)) {
    const value = env[envKey(prefix, key)];
    if (value !== undefined) out[key] = value;
  }
  return out as T;
}
