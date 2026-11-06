@@lesson t5-fullstack Full-Stack TypeScript: Shared Types, Validation and Frameworks
@@text
TypeScript's superpower in a web application is that the **front end and the back end can share types**. Change a field in the API and the compiler shows every screen that breaks, before any user does. But types disappear at runtime, so the boundary where data enters needs real checks.

## Types vanish at runtime

```typescript
type User = { id: number; email: string };

const fromApi: unknown = JSON.parse('{"id": "7", "mail": "yuki@example.jp"}');
const user = fromApi as User;               // the compiler believes you...
console.log(typeof user.id, user.email);    // ...but id is a string and email is undefined
```

`as User` is a promise you make to the compiler, not a check. Every value from outside (JSON bodies, `fetch` responses, files, environment variables, `localStorage`) is `unknown` until you've checked it.

## Parse, don't validate

Write functions that take `unknown` and return either a fully typed value or the list of problems. After that point, the rest of the code can trust the type:

```typescript
type Result<T> = { ok: true; value: T } | { ok: false; errors: string[] };
type NewTicket = { title: string; priority: 1 | 2 | 3 | 4; tags: string[] };

function parseNewTicket(input: unknown): Result<NewTicket> {
  const errors: string[] = [];
  if (typeof input !== "object" || input === null) return { ok: false, errors: ["body must be an object"] };
  const o = input as Record<string, unknown>;
  const title = typeof o.title === "string" ? o.title.trim() : "";
  if (title.length < 3) errors.push("title must be at least 3 characters");
  const priority = o.priority ?? 3;
  if (priority !== 1 && priority !== 2 && priority !== 3 && priority !== 4) errors.push("priority must be 1-4");
  const tags = o.tags ?? [];
  if (!Array.isArray(tags) || !tags.every((t) => typeof t === "string")) errors.push("tags must be a list of strings");
  if (errors.length) return { ok: false, errors };
  return { ok: true, value: { title, priority: priority as NewTicket["priority"], tags: tags as string[] } };
}

console.log(parseNewTicket({ title: "VPN drops", priority: 2, tags: ["network"] }));
console.log(parseNewTicket({ title: "x", priority: "high", tags: "network" }));
```

Libraries do this declaratively and **derive the type from the schema**, so the two can never disagree. **zod** is the most popular:

```typescript,norun
import { z } from "zod";

const NewTicket = z.object({
  title: z.string().trim().min(3),
  priority: z.union([z.literal(1), z.literal(2), z.literal(3), z.literal(4)]).default(3),
  tags: z.array(z.string()).default([]),
});
type NewTicket = z.infer<typeof NewTicket>;   // the type comes from the schema

const result = NewTicket.safeParse(await request.json());
if (!result.success) return Response.json({ errors: result.error.issues }, { status: 400 });
```

(Alternatives: valibot, arktype, TypeBox.)

## Sharing types between front and back end

Put the API's types (and schemas) in one place both sides import. In a **monorepo** (one repository, several packages, managed by npm/pnpm workspaces, Turborepo or Nx):

```text
apps/
  web/        React front end   ─┐
  api/        Node back end     ─┼── both import @company/contracts
packages/
  contracts/  schemas + types for every endpoint
```

A typed API client then makes calls impossible to get wrong:

```typescript
type Api = {
  "GET /tickets": { query: { status?: "open" | "closed" }; response: { id: number; title: string }[] };
  "POST /tickets": { body: { title: string; priority: number }; response: { id: number } };
};

type Endpoint = keyof Api;

// A fake transport so this runs here; in a browser it would call fetch.
async function transport(method: string, path: string, payload: unknown): Promise<unknown> {
  return method === "GET" ? [{ id: 1, title: "VPN drops" }] : { id: 2 };
}

async function call<E extends Endpoint>(endpoint: E, payload: Omit<Api[E], "response">): Promise<Api[E]["response"]> {
  const [method, path] = endpoint.split(" ");
  return (await transport(method, path, payload)) as Api[E]["response"];
}

(async () => {
  const open = await call("GET /tickets", { query: { status: "open" } });
  console.log(open[0].title);                   // typed as string
  const created = await call("POST /tickets", { body: { title: "Printer", priority: 2 } });
  console.log(created.id);
  // call("POST /tickets", { body: { title: "x" } })   ← compile error: priority is missing
})();
```

Tools generate this for you: **tRPC** (call server functions directly, typed end to end), **OpenAPI** generators (types from an API spec: `openapi-typescript`), and **GraphQL** codegen.

## The frameworks

| Framework | What it is | Typical use |
|---|---|---|
| **Next.js** (React) | full-stack: pages, server rendering, API routes, server actions | most new React apps; huge job market |
| **Nuxt** (Vue), **SvelteKit**, **Remix / React Router** | the same idea for other UI libraries | |
| **Angular** | complete front-end framework: DI, routing, forms, RxJS | large enterprise apps (common in Japanese SIers and banks) |
| **NestJS** | structured back end: modules, controllers, providers, DI | enterprise APIs |
| **Express / Fastify / Hono** | small back-end frameworks | APIs, microservices |
| **Prisma / Drizzle** | type-safe database access | the "repository" layer |

A Next.js route handler, showing how the pieces from this module fit:

```typescript,norun
// app/api/tickets/route.ts
export async function POST(request: Request) {
  const parsed = NewTicket.safeParse(await request.json());          // boundary: parse
  if (!parsed.success) return Response.json({ errors: parsed.error.issues }, { status: 400 });
  const result = await createTicket(parsed.data, await currentUser()); // use case
  return result.ok ? Response.json(result.value, { status: 201 }) : Response.json({ error: result.error }, { status: 409 });
}
```

## tsconfig for applications

The settings that matter most in a real project:

```text
"strict": true                      all the safety checks
"noUncheckedIndexedAccess": true    arr[i] is T | undefined (catches off-by-one bugs)
"exactOptionalPropertyTypes": true  optional means "absent", not "may be undefined"
"verbatimModuleSyntax": true        imports of types use `import type`
```

Run `tsc --noEmit` in CI on every pull request, alongside the tests and a linter (**typescript-eslint**).

@@quiz
What does `const user = data as User` check at runtime?
- That data has every field of User
- That the field types are right
* Nothing: it only tells the compiler to trust you
- That data isn't null
? Type assertions are erased. Use a parser (or zod) at the boundary for real checks.

@@quiz
What's the benefit of deriving the TypeScript type from a zod schema (`z.infer`)?
* The runtime check and the static type can never disagree
- zod types are faster
- It removes the need for tests
- It works without TypeScript
? One source of truth: change the schema, and both the check and the type change.

@@quiz
Why put API types in a shared `contracts` package in a monorepo?
- To make the bundle smaller
* So changing an endpoint shows compile errors in both the front end and back end
- Because Node.js requires it
- To hide the types from users
? The compiler checks both sides against the same definition.

@@quiz
Which framework is typical for structured, enterprise-style Node.js back ends with dependency injection?
- Svelte
- Express
* NestJS
- Vite
? NestJS is built around modules, controllers and DI providers, much like Angular and Spring.

@@flashcards
`unknown` :: a value you must check before using; right type for outside data
`as T` :: a compile-time assertion; checks nothing at runtime
parse, don't validate :: turn `unknown` into a typed value or a list of errors, once, at the edge
zod :: schema library; `z.infer` gives the type, `safeParse` checks data
monorepo :: one repository with several packages (pnpm workspaces, Turborepo, Nx)
tRPC :: end-to-end typed calls from front end to back end
OpenAPI :: a standard API description; types and clients can be generated from it
Next.js :: full-stack React framework
Angular :: complete front-end framework, popular in enterprises
NestJS :: structured back-end framework with DI
Prisma / Drizzle :: type-safe database libraries
`noUncheckedIndexedAccess` :: makes `arr[i]` possibly undefined

@@exercise Parse an API response
A monitoring API returns JSON you don't control. Write `parseHosts(data: unknown)` returning `Result<Host[]>` where `Host` is `{ name: string; up: boolean; latencyMs: number | null }`. The data must be an array; for each item check that `name` is a non-empty string, `up` is a boolean and `latencyMs` is a number ≥ 0 or `null` (missing counts as `null`). Collect **every** error as `"item <index>: <field> is invalid"`, in order.
@@starter
type Host = { name: string; up: boolean; latencyMs: number | null };
type Result<T> = { ok: true; value: T } | { ok: false; errors: string[] };

function parseHosts(data: unknown): Result<Host[]> {
  throw new Error("TODO: not done yet");
}

console.log(parseHosts([{ name: "web01", up: true, latencyMs: 12 }, { name: "db01", up: false }]));
@@tests
check(parseHosts([{ name: "web01", up: true, latencyMs: 12 }, { name: "db01", up: false }]),
  { ok: true, value: [{ name: "web01", up: true, latencyMs: 12 }, { name: "db01", up: false, latencyMs: null }] }, "valid data; missing latency is null");
check(parseHosts({ hosts: [] }), { ok: false, errors: ["data must be an array"] }, "not an array");
check(parseHosts([{ name: "", up: "yes", latencyMs: -1 }, { name: "ok", up: true, latencyMs: null }, null]),
  { ok: false, errors: ["item 0: name is invalid", "item 0: up is invalid", "item 0: latencyMs is invalid", "item 2: item is invalid"] }, "every error, in order");
check(parseHosts([]), { ok: true, value: [] }, "an empty list is fine");
@@hint
`if (!Array.isArray(data)) return { ok: false, errors: ["data must be an array"] };`. Loop with `data.forEach((item, i) => ...)`: if `typeof item !== "object" || item === null`, push `item i: item is invalid` and return. Otherwise cast to `Record<string, unknown>` and check each field. Only build the typed host when that item had no errors.
@@solution
type Host = { name: string; up: boolean; latencyMs: number | null };
type Result<T> = { ok: true; value: T } | { ok: false; errors: string[] };

function parseHosts(data: unknown): Result<Host[]> {
  if (!Array.isArray(data)) return { ok: false, errors: ["data must be an array"] };
  const errors: string[] = [];
  const hosts: Host[] = [];
  data.forEach((item: unknown, i) => {
    if (typeof item !== "object" || item === null) {
      errors.push(`item ${i}: item is invalid`);
      return;
    }
    const o = item as Record<string, unknown>;
    const before = errors.length;
    if (typeof o.name !== "string" || o.name === "") errors.push(`item ${i}: name is invalid`);
    if (typeof o.up !== "boolean") errors.push(`item ${i}: up is invalid`);
    const latency = o.latencyMs ?? null;
    if (latency !== null && !(typeof latency === "number" && latency >= 0)) errors.push(`item ${i}: latencyMs is invalid`);
    if (errors.length === before) hosts.push({ name: o.name as string, up: o.up as boolean, latencyMs: latency as number | null });
  });
  return errors.length ? { ok: false, errors } : { ok: true, value: hosts };
}

console.log(parseHosts([{ name: "web01", up: true, latencyMs: 12 }, { name: "db01", up: false }]));

@@exercise A typed route table
Front end and back end can share one table of routes. Write the type `PathParams<P>`, which turns a path like `"/hosts/:id/checks/:checkId"` into `{ id: string; checkId: string }` using template literal types, and a function `buildUrl(path, params, query?)` that fills in the parameters (URL-encoded), appends a query string from the optional object (skipping `undefined` values, keys in the given order), and throws `Error("missing param: <name>")` if a parameter has no value at runtime.
@@starter
type PathParams<P extends string> = Record<string, string>; // TODO: derive the names from P

function buildUrl<P extends string>(path: P, params: PathParams<P>, query?: Record<string, string | number | undefined>): string {
  throw new Error("TODO: not done yet");
}

@@tests
check(buildUrl("/hosts/:id", { id: "web01" }), "/hosts/web01", "one param");
check(buildUrl("/hosts/:id/checks/:checkId", { id: "db 01", checkId: "disk/var" }), "/hosts/db%2001/checks/disk%2Fvar", "params are encoded");
check(buildUrl("/hosts", {}, { page: 2, q: "tokyo dc", sort: undefined }), "/hosts?page=2&q=tokyo+dc", "query string, undefined skipped");
let message = "";
try { buildUrl("/hosts/:id", {} as any); } catch (e) { message = (e as Error).message; }
check(message, "missing param: id", "missing param is an error");
const typed: PathParams<"/a/:x/b/:y"> = { x: "1", y: "2" };
check(Object.keys(typed), ["x", "y"], "the type accepts exactly the path's params");
@@hint
Recursive template literal type: `P extends `${string}:${infer Name}/${infer Rest}` ? { [K in Name]: string } & PathParams<`/${Rest}`> : P extends `${string}:${infer Name}` ? { [K in Name]: string } : {}`. At runtime, `path.replace(/:(\w+)/g, (_, name) => ...)` with `encodeURIComponent`, and `URLSearchParams` for the query.
@@solution
type PathParams<P extends string> =
  P extends `${string}:${infer Name}/${infer Rest}`
    ? { [K in Name]: string } & PathParams<`/${Rest}`>
    : P extends `${string}:${infer Name}`
      ? { [K in Name]: string }
      : {};

function buildUrl<P extends string>(path: P, params: PathParams<P>, query?: Record<string, string | number | undefined>): string {
  const values = params as Record<string, string | undefined>;
  const filled = path.replace(/:(\w+)/g, (_, name: string) => {
    const value = values[name];
    if (value === undefined) throw new Error(`missing param: ${name}`);
    return encodeURIComponent(value);
  });
  if (!query) return filled;
  const search = new URLSearchParams();
  for (const [key, value] of Object.entries(query)) {
    if (value !== undefined) search.append(key, String(value));
  }
  const qs = search.toString();
  return qs ? `${filled}?${qs}` : filled;
}
