@@lesson j6-server Back End: Building a Web Server with Node.js
@@text
Express, Fastify, NestJS, Next.js: every Node.js back end is built on the same module, `node:http`. Learning it directly teaches you what the frameworks do for you, and how to debug them when they don't.

## The smallest server

```javascript,norun
const http = require("node:http");

const server = http.createServer((req, res) => {
  // req: method, url, headers, and a stream of the body
  // res: status, headers, and the body you write
  res.writeHead(200, { "Content-Type": "application/json" });
  res.end(JSON.stringify({ hello: "world", path: req.url }));
});

server.listen(3000, "127.0.0.1", () => console.log("http://localhost:3000"));
```

Run it on your computer with `node server.js` and open the address. `"127.0.0.1"` means only your own computer can connect; a server in a container listens on `"0.0.0.0"`.

## Routing

A **router** picks a handler from the method and the path. Path parameters like `/api/users/:id` become a regular expression:

```javascript
function compile(pattern) {
  const names = [];
  const source = pattern.replace(/:(\w+)/g, (_, name) => {
    names.push(name);
    return "([^/]+)";
  });
  const regex = new RegExp(`^${source}$`);
  return (path) => {
    const m = regex.exec(path);
    return m ? Object.fromEntries(names.map((n, i) => [n, decodeURIComponent(m[i + 1])])) : null;
  };
}

const userTicket = compile("/api/users/:user/tickets/:id");
console.log(userTicket("/api/users/yuki/tickets/42"));
console.log(userTicket("/api/users/yuki"));
```

## Reading the request body

The body arrives as a **stream** of chunks: it might be large, or arrive slowly. Collect it with a size limit, then parse:

```javascript,norun
async function readJson(req, limit = 100_000) {
  const chunks = [];
  let size = 0;
  for await (const chunk of req) {
    size += chunk.length;
    if (size > limit) throw new HttpError(413, "body is too large");
    chunks.push(chunk);
  }
  try {
    return JSON.parse(Buffer.concat(chunks).toString("utf8") || "{}");
  } catch {
    throw new HttpError(400, "body must be JSON");
  }
}
```

Without the limit, one client sending a 10 GB body could fill your server's memory.

## Middleware

**Middleware** is code that runs around every request: logging, authentication, CORS headers, timing. In Express it's `app.use(fn)`. The idea is just functions that wrap a handler:

```javascript
// Each middleware takes the next handler and returns a new handler.
const logger = (next) => async (req) => {
  const started = Date.now();
  const res = await next(req);
  console.log(`${req.method} ${req.url} ${res.status} ${Date.now() - started}ms`);
  return res;
};

const requireToken = (next) => async (req) => {
  if (req.headers.authorization !== "Bearer s3cret") return { status: 401, body: { error: "login required" } };
  return next(req);
};

const handler = async (req) => ({ status: 200, body: { tickets: 3 } });
const compose = (...mws) => (h) => mws.reduceRight((acc, mw) => mw(acc), h);
const app = compose(logger, requireToken)(handler);

(async () => {
  console.log((await app({ method: "GET", url: "/api/tickets", headers: {} })).body);
  console.log((await app({ method: "GET", url: "/api/tickets", headers: { authorization: "Bearer s3cret" } })).body);
})();
```

The order matters: here the logger runs first, so it logs the 401s too.

## Errors: one place, the right status

Throw errors with an HTTP status from anywhere, and catch them all in one place:

```javascript
class HttpError extends Error {
  constructor(status, message) {
    super(message);
    this.status = status;
  }
}

function toResponse(err) {
  if (err instanceof HttpError) return { status: err.status, body: { error: err.message } };
  console.error(err);                                        // a bug: log everything...
  return { status: 500, body: { error: "internal error" } }; // ...tell the client nothing
}

console.log(toResponse(new HttpError(404, "no ticket #7")));
console.log(toResponse(new TypeError("cannot read properties of undefined")).body);
```

## Serving files safely

To serve your front end, map the URL path to a file in a `public` folder. The classic mistake is trusting the path: `GET /../.env` would send your secrets. Resolve the full path, then check it's still inside the folder:

```javascript
const path = require("node:path");

function safeFile(publicDir, urlPath) {
  const root = path.resolve(publicDir);
  const file = path.resolve(root, "." + decodeURIComponent(urlPath));
  return file.startsWith(root + path.sep) ? file : null;
}

console.log(safeFile("/srv/app/public", "/css/site.css"));
console.log(safeFile("/srv/app/public", "/../.env"));
console.log(safeFile("/srv/app/public", "/%2e%2e/.env"));
```

## Testing a server

Start it on port `0` (the operating system picks a free port), call it with `fetch`, close it. These are real HTTP tests that run in milliseconds:

```javascript,norun
const test = require("node:test");
const assert = require("node:assert/strict");

test("GET /api/health", async () => {
  const server = createApp();
  await new Promise((resolve) => server.listen(0, resolve));
  const res = await fetch(`http://127.0.0.1:${server.address().port}/api/health`);
  assert.equal(res.status, 200);
  server.close();
});
```

## The frameworks

| Framework | Notes |
|---|---|
| **Express** | the classic; huge ecosystem of middleware |
| **Fastify** | faster, with JSON schema validation built in |
| **Hono** | tiny; runs on Node, Deno, Bun and edge platforms |
| **NestJS** | structured, Angular-like (modules, decorators, DI); popular in enterprises |
| **Next.js / Nuxt / SvelteKit** | "full-stack" frameworks: front end and back end in one project |

```javascript,norun
// Express: the same ideas as above, with the plumbing done for you.
const express = require("express");
const app = express();
app.use(express.json({ limit: "100kb" }));
app.get("/api/tickets/:id", (req, res) => res.json(service.get(req.params.id)));
app.use((err, req, res, next) => res.status(err.status ?? 500).json({ error: err.message }));
app.listen(3000);
```

@@quiz
Why read a request body with a size limit?
- JSON can't be larger than 100 KB
* Without one, a client can send a huge body and exhaust the server's memory
- Node.js crashes on bodies over 1 MB
- It makes responses faster
? Never trust the client to be reasonable. Limits on size, time and rate protect the server.

@@quiz
A request asks for `/../config/.env`. What should a static file server do?
- Serve it: the file exists
- Remove the dots and serve `config/.env`
* Resolve the full path and refuse it, because it's outside the public folder
- Return 500
? This is a "path traversal" attack. Compare the resolved path with the public folder instead of trying to clean the string.

@@quiz
What is middleware?
- A database between the server and the client
* Functions that run around every request, such as logging, authentication and CORS
- The part of Node.js that parses JSON
- A load balancer
? Middleware wraps handlers. Each one can act before and after the next, or stop the request (e.g. 401).

@@quiz
Why start a test server on port `0`?
- Port 0 is faster
* The operating system picks a free port, so tests never clash with other servers
- Tests are only allowed on port 0
- It disables the network
? `server.address().port` tells you which port you got.

@@flashcards
`http.createServer((req, res) => …)` :: Node's built-in web server
`res.writeHead(status, headers)` + `res.end(body)` :: send a response
`for await (const chunk of req)` :: read a request body stream
middleware :: code wrapping every request: logging, auth, CORS
`HttpError` with a status :: throw anywhere, turn into a response in one place
path traversal :: `../` in a URL reaching files outside the public folder
`server.listen(0)` :: let the OS pick a free port (great in tests)
Express / Fastify / Hono :: Node.js web frameworks
NestJS :: structured, enterprise-style Node.js framework
413 Payload Too Large :: the status for a body over the limit

@@exercise A router with methods
Write `createRouter()`. It returns an object with `add(method, pattern, handler)` (patterns use `:name` params) and `handle(method, path)`, which calls the matching handler with the params object and returns its result. If no pattern matches the path, return `{ status: 404 }`. If a pattern matches but with another method, return `{ status: 405 }`.
@@starter
function createRouter() {
  // TODO
}

const r = createRouter();
r.add("GET", "/api/tickets/:id", (p) => ({ status: 200, body: `ticket ${p.id}` }));
r.add("DELETE", "/api/tickets/:id", (p) => ({ status: 204 }));
console.log(r.handle("GET", "/api/tickets/42"));
@@tests
const _r = createRouter();
_r.add("GET", "/api/tickets", () => ({ status: 200, body: "list" }));
_r.add("GET", "/api/tickets/:id", (p) => ({ status: 200, body: `ticket ${p.id}` }));
_r.add("POST", "/api/users/:user/tickets/:id/assign", (p) => ({ status: 200, body: `${p.id}->${p.user}` }));
check(_r.handle("GET", "/api/tickets"), { status: 200, body: "list" }, "a fixed path");
check(_r.handle("GET", "/api/tickets/42"), { status: 200, body: "ticket 42" }, "one param");
check(_r.handle("POST", "/api/users/ken/tickets/7/assign"), { status: 200, body: "7->ken" }, "two params");
check(_r.handle("DELETE", "/api/tickets/42"), { status: 405 }, "wrong method");
check(_r.handle("GET", "/api/tickets/42/x"), { status: 404 }, "unknown path");
@@hint
In `add`, push `{ method, regex, names, handler }`: build the regex with `pattern.replace(/:(\w+)/g, (_, n) => { names.push(n); return "([^/]+)"; })` and wrap it in `^…$`. In `handle`, loop over the routes; when `regex.exec(path)` matches, either call the handler with `Object.fromEntries(names.map((n, i) => [n, m[i + 1]]))` or remember that the path exists.
@@solution
function createRouter() {
  const routes = [];
  return {
    add(method, pattern, handler) {
      const names = [];
      const source = pattern.replace(/:(\w+)/g, (_, n) => {
        names.push(n);
        return "([^/]+)";
      });
      routes.push({ method, regex: new RegExp(`^${source}$`), names, handler });
    },
    handle(method, path) {
      let known = false;
      for (const r of routes) {
        const m = r.regex.exec(path);
        if (!m) continue;
        if (r.method !== method) {
          known = true;
          continue;
        }
        return r.handler(Object.fromEntries(r.names.map((n, i) => [n, m[i + 1]])));
      }
      return { status: known ? 405 : 404 };
    },
  };
}

const r = createRouter();
r.add("GET", "/api/tickets/:id", (p) => ({ status: 200, body: `ticket ${p.id}` }));
r.add("DELETE", "/api/tickets/:id", (p) => ({ status: 204 }));
console.log(r.handle("GET", "/api/tickets/42"));

@@exercise Rate limiting middleware
Write `rateLimit(limit, windowMs, now)`: a middleware (it takes `next` and returns a handler `(req) => response`). Each client, identified by `req.ip`, may make `limit` requests per window of `windowMs` milliseconds (a fixed window starting at its first request). Over the limit, return `{ status: 429, body: { error: "too many requests" } }` without calling `next`. `now` is a function returning the current time in ms (so tests can control time).
@@starter
function rateLimit(limit, windowMs, now = Date.now) {
  // TODO: return (next) => (req) => ...
}

let t = 0;
const app = rateLimit(2, 1000, () => t)(() => ({ status: 200 }));
console.log(app({ ip: "10.0.0.5" }).status, app({ ip: "10.0.0.5" }).status, app({ ip: "10.0.0.5" }).status);
@@tests
let _t = 0;
const _app = rateLimit(3, 1000, () => _t)((req) => ({ status: 200, body: req.ip }));
const _codes = (ip, n) => Array.from({ length: n }, () => _app({ ip }).status);
check(_codes("10.0.0.5", 4), [200, 200, 200, 429], "the 4th request in the window is refused");
check(_app({ ip: "10.0.0.5" }).body, { error: "too many requests" }, "the 429 body");
check(_codes("10.0.0.9", 1), [200], "another client has its own count");
_t = 999;
check(_codes("10.0.0.5", 1), [429], "still the same window");
_t = 1000;
check(_codes("10.0.0.5", 4), [200, 200, 200, 429], "a new window starts");
@@hint
Keep a `Map` from ip to `{ start, count }`. In the handler: get the entry; if there is none or `now() - entry.start >= windowMs`, set a new one `{ start: now(), count: 0 }`. Add 1 to `count`; if it's over `limit`, return the 429 response, else `return next(req)`.
@@solution
function rateLimit(limit, windowMs, now = Date.now) {
  const clients = new Map();
  return (next) => (req) => {
    let entry = clients.get(req.ip);
    if (!entry || now() - entry.start >= windowMs) {
      entry = { start: now(), count: 0 };
      clients.set(req.ip, entry);
    }
    entry.count++;
    if (entry.count > limit) return { status: 429, body: { error: "too many requests" } };
    return next(req);
  };
}

let t = 0;
const app = rateLimit(2, 1000, () => t)(() => ({ status: 200 }));
console.log(app({ ip: "10.0.0.5" }).status, app({ ip: "10.0.0.5" }).status, app({ ip: "10.0.0.5" }).status);
