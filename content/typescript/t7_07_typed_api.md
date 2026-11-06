@@lesson t7-api Typed Backends: Routers, Middleware and Shared API Contracts
@@concepts networking, functions
@@text
TypeScript is at its best when **one type definition** describes an API for both the server and the client. Then a renamed field is a compile error in the frontend, not a production incident.

## Frameworks

| Framework | Style |
|---|---|
| Express | the classic; types from `@types/express`, loosely typed `req.body` |
| Fastify | fast, schema-based validation and serialization, good TS support |
| Hono | small, runs on Node/Bun/Deno/edge; typed routes and a typed client (`hc`) |
| NestJS | Angular-style decorators and dependency injection, popular in enterprises |
| tRPC | no REST at all: call server functions from the client with shared types |

```ts,norun
// Hono: the route string types c.req.param(), and the validator types the body
import { Hono } from "hono";
import { zValidator } from "@hono/zod-validator";
const app = new Hono()
  .get("/devices/:serial", (c) => c.json({ serial: c.req.param("serial") }))
  .post("/devices", zValidator("json", CreateDevice), (c) => {
    const body = c.req.valid("json");     // typed from the schema
    return c.json({ created: body.serial }, 201);
  });
export type AppType = typeof app;         // the client imports this type
```

## Routing is just a lookup

Underneath, a router matches method + path to a handler and extracts parameters. Testing handlers is easiest if they're **plain functions** from a request object to a response object, independent of the HTTP library:

```ts
type Req = { method: string; path: string; params: Record<string, string>; body: unknown };
type Res = { status: number; body: unknown };
type Handler = (req: Req) => Res | Promise<Res>;

const getHealth: Handler = () => ({ status: 200, body: { ok: true } });
Promise.resolve(getHealth({ method: "GET", path: "/health", params: {}, body: null })).then((r) => console.log(r));
```

The HTTP adapter (`node:http`, Express, Hono) only converts real requests into `Req` and `Res` back into a response. That's the ports-and-adapters idea from module 5.

## Middleware that adds typed context

Middleware runs before handlers: authentication, request IDs, logging, rate limiting. In a typed design, each middleware **adds fields to the context**, and the type accumulates:

```ts,norun
const app = createApp()
  .use(requestId)       // ctx: { requestId: string }
  .use(auth)            // ctx: { requestId: string; user: User }
  .get("/me", (ctx) => ({ status: 200, body: ctx.user }));   // ctx.user is typed
```

The exercise builds exactly this with an intersection type that grows with each `.use`.

## Sharing contracts with the client

Options, from simplest to most automatic:

1. A shared `types.ts` (or package in a monorepo) imported by both sides, plus runtime validation on the server.
2. Shared **schemas** (Zod): the server validates with them, the client infers types from them.
3. An **OpenAPI** spec, with server and client types generated from it (works across languages).
4. **tRPC** or Hono's RPC client: the client type is inferred directly from the server's router type.

## Production concerns

- Validate every input; return problem details consistently (`{ error, issues }`).
- Use `AbortSignal` timeouts for outbound calls, and graceful shutdown on SIGTERM.
- Structured JSON logs with a request id, and metrics per route.
- Never trust `Content-Type` or sizes: limit body size before parsing.

@@quiz
What's the main benefit of sharing API types between server and client?
* A breaking change becomes a compile error on the other side instead of a runtime bug
- Faster HTTP
- Smaller bundles
- No need for validation
? You still validate on the server: the client could be anything.

@@quiz
Why write handlers as plain `(req) => res` functions?
- HTTP libraries require it
* They can be tested without a server, and the HTTP library becomes a swappable adapter
- It's faster
- To avoid async code
? Ports and adapters again.

@@quiz
In a typed middleware chain, what does `.use(auth)` change?
* The context type gains the fields auth adds (like `user`), so later handlers can use them safely
- Nothing at the type level
- It removes fields
- It makes the context any
? The context type accumulates with each middleware.

@@drill
Which HTTP status code means the request is missing valid authentication?
= 401
? 403 means authenticated but not allowed.

@@flashcards
Router :: maps method + path to a handler
Route params :: values captured from `:name` segments
Middleware :: runs before handlers; can add context or stop the request
Typed context :: intersection type that grows with each middleware
tRPC :: call server procedures with inferred types, no REST layer
Hono RPC :: typed client from the server's route types
OpenAPI :: language-neutral API spec for codegen
Problem details :: consistent error responses

@@exercise A typed router
Write a `Router` class:

- `get(pattern, handler)`, `post(pattern, handler)`, `delete(pattern, handler)` register routes and return `this`; `pattern` uses `:name` segments
- each handler receives `{ params, query, body }` where `params` is typed from the pattern (`Params<P>` from the template literal lesson is given), `query` is a `Record<string, string>` parsed from `?a=1&b=2` (URL-decoded), and `body` is `unknown`
- `handle(method: string, url: string, body?: unknown): Promise<Res>`:
  - first matching route (registration order) wins; handlers may be async
  - path matches but method doesn't → `405` with `{ error: "method not allowed", allow: ["GET", ...] }` (methods registered for matching patterns, in registration order, no duplicates)
  - no match → `404` with `{ error: "not found" }`
  - a handler that throws → `500` with `{ error: "internal error" }`
@@starter
type ParamNames<P extends string> =
  P extends `${string}:${infer Name}/${infer Rest}` ? Name | ParamNames<`/${Rest}`>
  : P extends `${string}:${infer Name}` ? Name
  : never;
type Params<P extends string> = { [K in ParamNames<P>]: string };
type Res = { status: number; body: unknown };
type Ctx<P extends string> = { params: Params<P>; query: Record<string, string>; body: unknown };

class Router {
  get<P extends string>(pattern: P, handler: (ctx: Ctx<P>) => Res | Promise<Res>): this {
    throw new Error("TODO: not done yet");
  }
  post<P extends string>(pattern: P, handler: (ctx: Ctx<P>) => Res | Promise<Res>): this {
    throw new Error("TODO: not done yet");
  }
  delete<P extends string>(pattern: P, handler: (ctx: Ctx<P>) => Res | Promise<Res>): this {
    throw new Error("TODO: not done yet");
  }
  handle(method: string, url: string, body?: unknown): Promise<Res> {
    throw new Error("TODO: not done yet");
  }
}

@@tests
const devices = new Map<string, { serial: string; owner: string }>([["LT-1", { serial: "LT-1", owner: "ana" }]]);
const router = new Router()
  .get("/devices", ({ query }) => ({ status: 200, body: [...devices.values()].filter((d) => !query.owner || d.owner === query.owner) }))
  .get("/devices/:serial", ({ params }) => {
    const d = devices.get(params.serial);
    return d ? { status: 200, body: d } : { status: 404, body: { error: "no such device" } };
  })
  .post("/devices", async ({ body }) => {
    const d = body as { serial: string; owner: string };
    devices.set(d.serial, d);
    return { status: 201, body: d };
  })
  .delete("/devices/:serial", ({ params }) => ({ status: devices.delete(params.serial) ? 204 : 404, body: null }))
  .get("/crash", () => { throw new Error("bug"); });
(async () => {
  check(await router.handle("GET", "/devices/LT-1"), { status: 200, body: { serial: "LT-1", owner: "ana" } }, "param route");
  check((await router.handle("POST", "/devices", { serial: "PH-2", owner: "bo" })).status, 201, "post");
  check(await router.handle("GET", "/devices?owner=b%6F"), { status: 200, body: [{ serial: "PH-2", owner: "bo" }] }, "query decoded");
  check(await router.handle("PUT", "/devices/LT-1"), { status: 405, body: { error: "method not allowed", allow: ["GET", "DELETE"] } }, "405");
  check(await router.handle("GET", "/printers"), { status: 404, body: { error: "not found" } }, "404");
  check(await router.handle("GET", "/crash"), { status: 500, body: { error: "internal error" } }, "500");
  check((await router.handle("DELETE", "/devices/LT-1")).status, 204, "delete");
})();
@@hint
Store routes as `{ method, parts: string[], handler: (ctx: Ctx<string>) => Res | Promise<Res> }` (cast the typed handler once when registering). Split the URL with `const [path = "", qs = ""] = url.split("?")`; parse the query with `new URLSearchParams(qs)`. Write `match(parts, path): Record<string, string> | null`. Wrap the handler call in `try { return await route.handler(...) } catch { return 500 }`.
@@solution
type ParamNames<P extends string> =
  P extends `${string}:${infer Name}/${infer Rest}` ? Name | ParamNames<`/${Rest}`>
  : P extends `${string}:${infer Name}` ? Name
  : never;
type Params<P extends string> = { [K in ParamNames<P>]: string };
type Res = { status: number; body: unknown };
type Ctx<P extends string> = { params: Params<P>; query: Record<string, string>; body: unknown };
type AnyHandler = (ctx: { params: Record<string, string>; query: Record<string, string>; body: unknown }) => Res | Promise<Res>;

function matchParts(parts: string[], path: string): Record<string, string> | null {
  const got = path.split("/");
  if (got.length !== parts.length) return null;
  const params: Record<string, string> = {};
  for (let i = 0; i < parts.length; i++) {
    const want = parts[i] ?? "";
    const actual = got[i] ?? "";
    if (want.startsWith(":")) params[want.slice(1)] = decodeURIComponent(actual);
    else if (want !== actual) return null;
  }
  return params;
}

class Router {
  private readonly routes: { method: string; parts: string[]; handler: AnyHandler }[] = [];

  private add(method: string, pattern: string, handler: unknown): this {
    this.routes.push({ method, parts: pattern.split("/"), handler: handler as AnyHandler });
    return this;
  }

  get<P extends string>(pattern: P, handler: (ctx: Ctx<P>) => Res | Promise<Res>): this {
    return this.add("GET", pattern, handler);
  }
  post<P extends string>(pattern: P, handler: (ctx: Ctx<P>) => Res | Promise<Res>): this {
    return this.add("POST", pattern, handler);
  }
  delete<P extends string>(pattern: P, handler: (ctx: Ctx<P>) => Res | Promise<Res>): this {
    return this.add("DELETE", pattern, handler);
  }

  async handle(method: string, url: string, body?: unknown): Promise<Res> {
    const [path = "", qs = ""] = url.split("?");
    const query: Record<string, string> = Object.fromEntries(new URLSearchParams(qs));
    const allow: string[] = [];
    for (const route of this.routes) {
      const params = matchParts(route.parts, path);
      if (params === null) continue;
      if (route.method !== method) {
        if (!allow.includes(route.method)) allow.push(route.method);
        continue;
      }
      try {
        return await route.handler({ params, query, body });
      } catch {
        return { status: 500, body: { error: "internal error" } };
      }
    }
    if (allow.length > 0) return { status: 405, body: { error: "method not allowed", allow } };
    return { status: 404, body: { error: "not found" } };
  }
}

@@exercise Middleware with accumulating context
Write `createApp()` returning an `App<{}>`, where `App<C>` has:

- `use<Added extends object>(mw: (ctx: C, req: Req) => Added | Res): App<C & Added>`: a middleware returns either fields to **add** to the context, or a `Res` (detected by a numeric `status` property) to **stop** the request
- `route(handler: (ctx: C, req: Req) => Res): (req: Req) => Res`: builds the final request function, which runs middleware in order, then the handler

with `type Req = { headers: Record<string, string>; path: string }` and `Res` as before. Then build the app in the tests' style: `requestId` (adds `requestId` from the `x-request-id` header or `"generated"`), `auth` (adds `user` from a token table or stops with 401), `requireRole("admin")` (stops with 403 unless `ctx.user.roles` includes it). `requireRole` must only be usable **after** auth: its context type requires `user`.
@@starter
type Req = { headers: Record<string, string>; path: string };
type Res = { status: number; body: unknown };
type User = { name: string; roles: string[] };

interface App<C> {
  use<Added extends object>(mw: (ctx: C, req: Req) => Added | Res): App<C & Added>;
  route(handler: (ctx: C, req: Req) => Res): (req: Req) => Res;
}

function createApp(): App<{}> {
  throw new Error("TODO: not done yet");
}

const TOKENS: Record<string, User> = { t1: { name: "ana", roles: ["admin"] }, t2: { name: "bo", roles: [] } };

function requestId(_ctx: {}, req: Req): { requestId: string } | Res {
  throw new Error("TODO: not done yet");
}

function auth(_ctx: {}, req: Req): { user: User } | Res {
  throw new Error("TODO: not done yet");
}

function requireRole(role: string): (ctx: { user: User }, req: Req) => {} | Res {
  throw new Error("TODO: not done yet");
}

@@tests
const handle = createApp()
  .use(requestId)
  .use(auth)
  .use(requireRole("admin"))
  .route((ctx, req) => ({ status: 200, body: `${ctx.requestId} ${ctx.user.name} ${req.path}` }));
check(handle({ headers: { authorization: "Bearer t1", "x-request-id": "r-9" }, path: "/admin" }), { status: 200, body: "r-9 ana /admin" }, "allowed");
check(handle({ headers: { authorization: "Bearer t2" }, path: "/admin" }), { status: 403, body: { error: "forbidden" } }, "forbidden");
check(handle({ headers: {}, path: "/admin" }), { status: 401, body: { error: "unauthorized" } }, "no token");
const open = createApp().use(requestId).route((ctx) => ({ status: 200, body: ctx.requestId }));
check(open({ headers: {}, path: "/" }), { status: 200, body: "generated" }, "generated id");
function _typeTests(): void {
  // @ts-expect-error requireRole needs ctx.user, which only auth provides
  createApp().use(requestId).use(requireRole("admin"));
}
@@hint
Store middleware in an array of loosely typed functions, and make `use` return a **new** app with the longer list: `function build<C>(mws: AnyMw[]): App<C>`. When running: `let ctx: Record<string, unknown> = {}; for (const mw of mws) { const r = mw(ctx, req); if (isRes(r)) return r; ctx = { ...ctx, ...r }; }`. A `Res` check: `typeof (r as Res).status === "number"`.
@@solution
type Req = { headers: Record<string, string>; path: string };
type Res = { status: number; body: unknown };
type User = { name: string; roles: string[] };

interface App<C> {
  use<Added extends object>(mw: (ctx: C, req: Req) => Added | Res): App<C & Added>;
  route(handler: (ctx: C, req: Req) => Res): (req: Req) => Res;
}

type AnyMw = (ctx: Record<string, unknown>, req: Req) => object;

const isRes = (r: object): r is Res => typeof (r as Partial<Res>).status === "number";

function build<C>(mws: AnyMw[]): App<C> {
  return {
    use<Added extends object>(mw: (ctx: C, req: Req) => Added | Res): App<C & Added> {
      return build<C & Added>([...mws, mw as unknown as AnyMw]);
    },
    route(handler: (ctx: C, req: Req) => Res): (req: Req) => Res {
      return (req) => {
        let ctx: Record<string, unknown> = {};
        for (const mw of mws) {
          const r = mw(ctx, req);
          if (isRes(r)) return r;
          ctx = { ...ctx, ...r };
        }
        return handler(ctx as C, req);
      };
    },
  };
}

function createApp(): App<{}> {
  return build<{}>([]);
}

const TOKENS: Record<string, User> = { t1: { name: "ana", roles: ["admin"] }, t2: { name: "bo", roles: [] } };

function requestId(_ctx: {}, req: Req): { requestId: string } | Res {
  return { requestId: req.headers["x-request-id"] ?? "generated" };
}

function auth(_ctx: {}, req: Req): { user: User } | Res {
  const token = (req.headers.authorization ?? "").replace(/^Bearer /, "");
  const user = TOKENS[token];
  return user ? { user } : { status: 401, body: { error: "unauthorized" } };
}

function requireRole(role: string): (ctx: { user: User }, req: Req) => {} | Res {
  return (ctx) => (ctx.user.roles.includes(role) ? {} : { status: 403, body: { error: "forbidden" } });
}
