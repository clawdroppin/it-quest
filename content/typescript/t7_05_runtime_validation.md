@@lesson t7-validation Runtime Validation with Inferred Types (How Zod Works)
@@concepts json, objects
@@text
Types are erased, so data from the outside world (HTTP bodies, files, env vars, `localStorage`, message queues) must be checked at runtime. Writing a type **and** a validator separately means they drift apart. Schema libraries solve this: you write the schema once, and the **type is inferred** from it.

## Using Zod (the most popular one)

```ts,norun
import { z } from "zod";

const Ticket = z.object({
  id: z.number().int().positive(),
  title: z.string().min(3),
  priority: z.enum(["low", "normal", "high"]),
  tags: z.array(z.string()).default([]),
  assignee: z.string().email().optional(),
});
type Ticket = z.infer<typeof Ticket>;      // the static type, derived from the schema

const result = Ticket.safeParse(JSON.parse(body));
if (!result.success) return badRequest(result.error.issues);   // [{ path: ["priority"], message: ... }]
const ticket: Ticket = result.data;
```

Alternatives: Valibot (tiny, tree-shakable), ArkType (TS-like syntax), TypeBox (produces JSON Schema), and the **Standard Schema** interface that lets frameworks accept any of them.

## How it works inside

A schema is an object with a `parse` method and a **phantom type**: a type parameter that's carried along only for inference. `Infer` reads it back:

```ts
interface Schema<T> {
  parse(value: unknown, path?: string): T;     // throws on invalid input
}
type Infer<S> = S extends Schema<infer T> ? T : never;

const str: Schema<string> = {
  parse(v, path = "value") {
    if (typeof v !== "string") throw new Error(`${path}: expected string`);
    return v;
  },
};

function object<Shape extends Record<string, Schema<unknown>>>(shape: Shape): Schema<{ [K in keyof Shape]: Infer<Shape[K]> }> {
  return {
    parse(v, path = "value") {
      if (typeof v !== "object" || v === null) throw new Error(`${path}: expected object`);
      const input: Record<string, unknown> = { ...v };
      const out: Record<string, unknown> = {};
      for (const key of Object.keys(shape)) out[key] = shape[key]!.parse(input[key], `${path}.${key}`);
      return out as { [K in keyof Shape]: Infer<Shape[K]> };
    },
  };
}

const User = object({ name: str, team: str });
type User = Infer<typeof User>;               // { name: string; team: string }
console.log(User.parse({ name: "ana", team: "ops", extra: 1 }));
```

The `object` function's return type is a **mapped type over the shape**, so each property's type comes from its schema. That's the whole trick; real libraries add many more schema kinds, error collection, transforms and better error messages.

## Parse, don't validate

A validator that returns `boolean` throws away what it learned. A **parser** returns the data in a more precise type (or fails). After parsing at the boundary, the rest of the program never re-checks:

- `isEmail(s): boolean` → you still have a plain `string` afterwards.
- `parseEmail(s): Email` (branded) → the type proves it was checked.

## Where to validate

- Every **external input**: request bodies, query strings, headers, env vars at startup, files, third-party API responses (they change without warning).
- Not between your own typed modules: that's what the compiler is for.
- Report **all** problems with **paths** (`items[2].port`), so users can fix everything at once.

## JSON Schema and OpenAPI

Schemas can also generate documentation and clients: TypeBox and `zod-to-json-schema` produce JSON Schema; OpenAPI generators produce typed clients from an API spec. One source of truth for validation, types and docs.

@@quiz
Why use a schema library instead of writing an interface plus a separate validator?
* The type is inferred from the schema, so the two can't drift apart
- Interfaces don't work with JSON
- Validators are slower
- The compiler requires it
? `type T = z.infer<typeof Schema>`.

@@quiz
What does "parse, don't validate" mean?
- Never check input
* Turn unchecked input into a precise type once, at the boundary, instead of returning a boolean
- Use JSON.parse everywhere
- Validate twice
? The output type proves the check happened.

@@quiz
Which data needs runtime validation in a typed backend?
- Values passed between your own typed functions
* Request bodies, env vars, files and third-party API responses
- Constants
- Nothing; TypeScript checks it all
? Types end where the outside world begins.

@@flashcards
Schema :: runtime description of data that also yields a static type
`z.infer<typeof S>` :: the static type of a Zod schema
`safeParse` :: returns success/data or error/issues instead of throwing
Phantom type :: type parameter used only for inference
Parse, don't validate :: return a refined type, not a boolean
Issue path :: location of an error, like `items[2].port`
Standard Schema :: shared interface for validation libraries
JSON Schema / OpenAPI :: language-neutral schema and API descriptions

@@exercise A mini schema library
Build a small Zod-like library that **collects all issues**:

- `type Issue = { path: string; message: string }` and `type Result<T> = { ok: true; value: T } | { ok: false; issues: Issue[] }`
- `interface Schema<T> { check(value: unknown, path: string, issues: Issue[]): T | undefined; parse(value: unknown): Result<T> }` (`check` pushes issues and returns `undefined` on failure; `parse` starts with path `"$"`)
- `type Infer<S>`
- `s.string()`, `s.number()` (finite numbers only), `s.boolean()`; messages: `"expected string"` etc.
- `s.literal(...values)`: one of the given strings, message `"expected one of a|b"`
- `s.array(item)`: paths `$.tags[1]`; message `"expected array"`
- `s.object(shape)`: paths `$.owner.name`; message `"expected object"`; extra keys are dropped
- `s.optional(schema)`: `undefined` passes

Write a `makeSchema<T>(check)` helper so every schema gets the same `parse`.
@@starter
type Issue = { path: string; message: string };
type Result<T> = { ok: true; value: T } | { ok: false; issues: Issue[] };
interface Schema<T> {
  check(value: unknown, path: string, issues: Issue[]): T | undefined;
  parse(value: unknown): Result<T>;
}
type Infer<S> = unknown;

const s = {
  // TODO: string, number, boolean, literal, array, object, optional
};

@@tests
type Equal<X, Y> = (<T>() => T extends X ? 1 : 2) extends (<T>() => T extends Y ? 1 : 2) ? true : false;
type Expect<T extends true> = T;
const Server = s.object({
  name: s.string(),
  port: s.number(),
  env: s.literal("dev", "prod"),
  tags: s.array(s.string()),
  owner: s.optional(s.object({ name: s.string(), oncall: s.boolean() })),
});
type Server = Infer<typeof Server>;
type _t1 = Expect<Equal<Server["env"], "dev" | "prod">>;
type _t2 = Expect<Equal<Server["owner"], { name: string; oncall: boolean } | undefined>>;
check(Server.parse({ name: "web01", port: 443, env: "prod", tags: ["eu"], extra: true }), { ok: true, value: { name: "web01", port: 443, env: "prod", tags: ["eu"], owner: undefined } }, "valid");
const bad = Server.parse({ name: 5, port: Infinity, env: "test", tags: ["a", 2], owner: { name: "ana" } });
check(bad.ok ? [] : bad.issues, [
  { path: "$.name", message: "expected string" },
  { path: "$.port", message: "expected number" },
  { path: "$.env", message: "expected one of dev|prod" },
  { path: "$.tags[1]", message: "expected string" },
  { path: "$.owner.oncall", message: "expected boolean" },
], "all issues with paths");
check(Server.parse(null), { ok: false, issues: [{ path: "$", message: "expected object" }] }, "not an object");
@@hint
`function makeSchema<T>(check: Schema<T>["check"]): Schema<T> { return { check, parse(value) { const issues: Issue[] = []; const v = check(value, "$", issues); return issues.length ? { ok: false, issues } : { ok: true, value: v as T }; } }; }`. For objects, run every property's check (don't stop at the first failure) and return `undefined` if the issue count grew. `type Infer<S> = S extends Schema<infer T> ? T : never;`.
@@solution
type Issue = { path: string; message: string };
type Result<T> = { ok: true; value: T } | { ok: false; issues: Issue[] };
interface Schema<T> {
  check(value: unknown, path: string, issues: Issue[]): T | undefined;
  parse(value: unknown): Result<T>;
}
type Infer<S> = S extends Schema<infer T> ? T : never;

function makeSchema<T>(check: Schema<T>["check"]): Schema<T> {
  return {
    check,
    parse(value) {
      const issues: Issue[] = [];
      const v = check(value, "$", issues);
      return issues.length > 0 ? { ok: false, issues } : { ok: true, value: v as T };
    },
  };
}

function primitive<T>(name: string, test: (v: unknown) => v is T): Schema<T> {
  return makeSchema<T>((v, path, issues) => {
    if (test(v)) return v;
    issues.push({ path, message: `expected ${name}` });
    return undefined;
  });
}

const s = {
  string: () => primitive("string", (v): v is string => typeof v === "string"),
  number: () => primitive("number", (v): v is number => typeof v === "number" && Number.isFinite(v)),
  boolean: () => primitive("boolean", (v): v is boolean => typeof v === "boolean"),
  literal: <const L extends string[]>(...values: L): Schema<L[number]> =>
    makeSchema<L[number]>((v, path, issues) => {
      if (typeof v === "string" && values.includes(v)) return v;
      issues.push({ path, message: `expected one of ${values.join("|")}` });
      return undefined;
    }),
  array: <T>(item: Schema<T>): Schema<T[]> =>
    makeSchema<T[]>((v, path, issues) => {
      if (!Array.isArray(v)) {
        issues.push({ path, message: "expected array" });
        return undefined;
      }
      const before = issues.length;
      const out = v.map((x, i) => item.check(x, `${path}[${i}]`, issues));
      return issues.length === before ? (out as T[]) : undefined;
    }),
  object: <Shape extends Record<string, Schema<unknown>>>(shape: Shape): Schema<{ [K in keyof Shape]: Infer<Shape[K]> }> =>
    makeSchema<{ [K in keyof Shape]: Infer<Shape[K]> }>((v, path, issues) => {
      if (typeof v !== "object" || v === null || Array.isArray(v)) {
        issues.push({ path, message: "expected object" });
        return undefined;
      }
      const input: Record<string, unknown> = { ...v };
      const before = issues.length;
      const out: Record<string, unknown> = {};
      for (const [key, schema] of Object.entries(shape)) out[key] = schema.check(input[key], `${path}.${key}`, issues);
      return issues.length === before ? (out as { [K in keyof Shape]: Infer<Shape[K]> }) : undefined;
    }),
  optional: <T>(schema: Schema<T>): Schema<T | undefined> =>
    makeSchema<T | undefined>((v, path, issues) => (v === undefined ? undefined : schema.check(v, path, issues))),
};

@@exercise Validate an API request
The schema library from the previous exercise is given in the starter (with `parse` returning `Result`). Use it to write a request handler:

- `const CreateTicket = s.object({ title: s.string(), priority: s.literal("low", "normal", "high"), tags: s.optional(s.array(s.string())) })`
- `type CreateTicket = Infer<typeof CreateTicket>`
- `handleCreate(body: string, nextId: () => number): { status: number; body: unknown }`:
  - invalid JSON → `400` with `{ error: "invalid JSON" }`
  - schema issues → `422` with `{ error: "validation failed", issues }`
  - a title shorter than 3 characters after trimming → `422` with issue `{ path: "$.title", message: "too short" }` (checked after the schema passes)
  - success → `201` with `{ id, title (trimmed), priority, tags (default []) }`
@@starter
type Issue = { path: string; message: string };
type Result<T> = { ok: true; value: T } | { ok: false; issues: Issue[] };
interface Schema<T> {
  check(value: unknown, path: string, issues: Issue[]): T | undefined;
  parse(value: unknown): Result<T>;
}
type Infer<S> = S extends Schema<infer T> ? T : never;

function makeSchema<T>(check: Schema<T>["check"]): Schema<T> {
  return {
    check,
    parse(value) {
      const issues: Issue[] = [];
      const v = check(value, "$", issues);
      return issues.length > 0 ? { ok: false, issues } : { ok: true, value: v as T };
    },
  };
}

const s = {
  string: () => makeSchema<string>((v, path, issues) => (typeof v === "string" ? v : (issues.push({ path, message: "expected string" }), undefined))),
  literal: <const L extends string[]>(...values: L) =>
    makeSchema<L[number]>((v, path, issues) =>
      typeof v === "string" && values.includes(v) ? v : (issues.push({ path, message: `expected one of ${values.join("|")}` }), undefined)),
  array: <T>(item: Schema<T>) =>
    makeSchema<T[]>((v, path, issues) => {
      if (!Array.isArray(v)) return (issues.push({ path, message: "expected array" }), undefined);
      const before = issues.length;
      const out = v.map((x, i) => item.check(x, `${path}[${i}]`, issues));
      return issues.length === before ? (out as T[]) : undefined;
    }),
  object: <Shape extends Record<string, Schema<unknown>>>(shape: Shape) =>
    makeSchema<{ [K in keyof Shape]: Infer<Shape[K]> }>((v, path, issues) => {
      if (typeof v !== "object" || v === null || Array.isArray(v)) return (issues.push({ path, message: "expected object" }), undefined);
      const input: Record<string, unknown> = { ...v };
      const before = issues.length;
      const out: Record<string, unknown> = {};
      for (const [key, schema] of Object.entries(shape)) out[key] = schema.check(input[key], `${path}.${key}`, issues);
      return issues.length === before ? (out as { [K in keyof Shape]: Infer<Shape[K]> }) : undefined;
    }),
  optional: <T>(schema: Schema<T>) => makeSchema<T | undefined>((v, path, issues) => (v === undefined ? undefined : schema.check(v, path, issues))),
};

// TODO: CreateTicket schema and type

function handleCreate(body: string, nextId: () => number): { status: number; body: unknown } {
  throw new Error("TODO: not done yet");
}

@@tests
let id = 100;
const nextId = () => ++id;
check(handleCreate('{"title":"  VPN down ","priority":"high"}', nextId), { status: 201, body: { id: 101, title: "VPN down", priority: "high", tags: [] } }, "created");
check(handleCreate('{"title":"Disk","priority":"low","tags":["infra"]}', nextId).body, { id: 102, title: "Disk", priority: "low", tags: ["infra"] }, "with tags");
check(handleCreate("{nope", nextId), { status: 400, body: { error: "invalid JSON" } }, "bad JSON");
check(handleCreate('{"title":1,"priority":"urgent","tags":"x"}', nextId), { status: 422, body: { error: "validation failed", issues: [
  { path: "$.title", message: "expected string" },
  { path: "$.priority", message: "expected one of low|normal|high" },
  { path: "$.tags", message: "expected array" },
] } }, "issues");
check(handleCreate('{"title":" a ","priority":"low"}', nextId), { status: 422, body: { error: "validation failed", issues: [{ path: "$.title", message: "too short" }] } }, "too short");
check(id, 102, "ids only used on success");
@@hint
`let data: unknown; try { data = JSON.parse(body); } catch { return { status: 400, ... }; }`, then `const r = CreateTicket.parse(data); if (!r.ok) return ...;` After that, `r.value` is fully typed: `r.value.priority` is `"low" | "normal" | "high"`.
@@solution
type Issue = { path: string; message: string };
type Result<T> = { ok: true; value: T } | { ok: false; issues: Issue[] };
interface Schema<T> {
  check(value: unknown, path: string, issues: Issue[]): T | undefined;
  parse(value: unknown): Result<T>;
}
type Infer<S> = S extends Schema<infer T> ? T : never;

function makeSchema<T>(check: Schema<T>["check"]): Schema<T> {
  return {
    check,
    parse(value) {
      const issues: Issue[] = [];
      const v = check(value, "$", issues);
      return issues.length > 0 ? { ok: false, issues } : { ok: true, value: v as T };
    },
  };
}

const s = {
  string: () => makeSchema<string>((v, path, issues) => (typeof v === "string" ? v : (issues.push({ path, message: "expected string" }), undefined))),
  literal: <const L extends string[]>(...values: L) =>
    makeSchema<L[number]>((v, path, issues) =>
      typeof v === "string" && values.includes(v) ? v : (issues.push({ path, message: `expected one of ${values.join("|")}` }), undefined)),
  array: <T>(item: Schema<T>) =>
    makeSchema<T[]>((v, path, issues) => {
      if (!Array.isArray(v)) return (issues.push({ path, message: "expected array" }), undefined);
      const before = issues.length;
      const out = v.map((x, i) => item.check(x, `${path}[${i}]`, issues));
      return issues.length === before ? (out as T[]) : undefined;
    }),
  object: <Shape extends Record<string, Schema<unknown>>>(shape: Shape) =>
    makeSchema<{ [K in keyof Shape]: Infer<Shape[K]> }>((v, path, issues) => {
      if (typeof v !== "object" || v === null || Array.isArray(v)) return (issues.push({ path, message: "expected object" }), undefined);
      const input: Record<string, unknown> = { ...v };
      const before = issues.length;
      const out: Record<string, unknown> = {};
      for (const [key, schema] of Object.entries(shape)) out[key] = schema.check(input[key], `${path}.${key}`, issues);
      return issues.length === before ? (out as { [K in keyof Shape]: Infer<Shape[K]> }) : undefined;
    }),
  optional: <T>(schema: Schema<T>) => makeSchema<T | undefined>((v, path, issues) => (v === undefined ? undefined : schema.check(v, path, issues))),
};

const CreateTicket = s.object({
  title: s.string(),
  priority: s.literal("low", "normal", "high"),
  tags: s.optional(s.array(s.string())),
});
type CreateTicket = Infer<typeof CreateTicket>;

function handleCreate(body: string, nextId: () => number): { status: number; body: unknown } {
  let data: unknown;
  try {
    data = JSON.parse(body);
  } catch {
    return { status: 400, body: { error: "invalid JSON" } };
  }
  const r = CreateTicket.parse(data);
  if (!r.ok) return { status: 422, body: { error: "validation failed", issues: r.issues } };
  const input: CreateTicket = r.value;
  const title = input.title.trim();
  if (title.length < 3) return { status: 422, body: { error: "validation failed", issues: [{ path: "$.title", message: "too short" }] } };
  return { status: 201, body: { id: nextId(), title, priority: input.priority, tags: input.tags ?? [] } };
}
