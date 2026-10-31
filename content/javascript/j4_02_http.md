@@lesson j4-http HTTP and REST APIs with Node
@@concepts networking, json
@@text
## HTTP in one table

A **request** has a method, a path, headers and maybe a body; a **response** has a status code, headers and a body.

| Method | Means | Example |
|---|---|---|
| `GET` | read | `GET /api/tickets` |
| `POST` | create | `POST /api/tickets` with JSON |
| `PUT` / `PATCH` | replace / update | `PATCH /api/tickets/7` |
| `DELETE` | delete | `DELETE /api/tickets/7` |

| Status | Means |
|---|---|
| `200` OK, `201` Created, `204` No Content | success |
| `301` / `302` | redirect |
| `400` Bad Request, `401` Unauthorized, `403` Forbidden, `404` Not Found | the client's fault |
| `500` Internal Server Error, `502` Bad Gateway, `503` Unavailable | the server's fault |

**REST** is the convention of resources (nouns) at URLs, manipulated with these methods, usually with JSON bodies.

## A server in 20 lines

Node's built-in `http` module is enough for a small API. This one starts, answers a request to itself, and stops:

```javascript
const http = require("node:http");

const tickets = [{ id: 1, title: "Printer jam", status: "open" }];

const server = http.createServer((req, res) => {
  const url = new URL(req.url, "http://localhost");
  if (req.method === "GET" && url.pathname === "/api/tickets") {
    res.writeHead(200, { "Content-Type": "application/json" });
    res.end(JSON.stringify(tickets));
  } else {
    res.writeHead(404, { "Content-Type": "application/json" });
    res.end(JSON.stringify({ error: "not found" }));
  }
});

server.listen(0, async () => {                       // 0 = any free port
  const { port } = server.address();
  const ok = await fetch(`http://localhost:${port}/api/tickets`);
  console.log(ok.status, await ok.json());
  const missing = await fetch(`http://localhost:${port}/nope`);
  console.log(missing.status);
  server.close();
});
```

Real projects usually add a framework like **Express** or **Fastify** for routing and middleware, but underneath it's exactly this: a function from request to response.

## Reading a JSON body

The body arrives in chunks, so it's read asynchronously:

```javascript,norun
async function readJson(req) {
  let body = "";
  for await (const chunk of req) body += chunk;
  return JSON.parse(body || "{}");
}
```

## Things a real API needs

- **Validation**: never trust input; reply `400` with a clear message.
- **Authentication**: who is calling? (API keys, tokens, OAuth.) **Authorization**: are they allowed?
- **HTTPS**: in production, behind a reverse proxy like nginx that handles TLS.
- **Logging** and sensible error handling, so a bug returns `500` instead of crashing the process.

@@quiz
A client sends a ticket without a title to your API. Which status should you return?
* 400 Bad Request
- 404 Not Found
- 500 Internal Server Error
- 200 OK
? The request was invalid, which is the client's fault: a 4xx code, with a helpful message.

@@quiz
Which method creates a new resource in a REST API?
* POST
- GET
- DELETE
- HEAD
? POST creates; GET reads; PUT/PATCH update; DELETE removes.

@@flashcards
GET / POST / PATCH / DELETE :: Read / create / update / delete
200 / 201 / 204 :: OK / Created / No Content
400 / 401 / 403 / 404 :: Bad request / not authenticated / not allowed / not found
500 / 502 / 503 :: Server error / bad gateway (upstream failed) / unavailable
REST :: Resources at URLs, used with HTTP methods and JSON
Content-Type: application/json :: Header saying the body is JSON
reverse proxy :: A server (nginx) in front of apps, handling TLS and routing

@@exercise A tiny router
Write `route(method, path)` returning the name of the handler for an API with these routes, or `"notFound"`: `GET /api/tickets` → `"list"`, `POST /api/tickets` → `"create"`, `GET /api/tickets/<number>` → `"show"`, `PATCH /api/tickets/<number>` → `"update"`. Also return the id for the last two: return an object `{ handler, id }` where `id` is a **number**, or `null` when there is none.
@@starter
function route(method, path) {
  throw new Error("TODO: not done yet");
}

@@tests
check(route("GET", "/api/tickets"), { handler: "list", id: null }, "GET /api/tickets");
check(route("POST", "/api/tickets"), { handler: "create", id: null }, "POST /api/tickets");
check(route("GET", "/api/tickets/42"), { handler: "show", id: 42 }, "GET /api/tickets/42");
check(route("PATCH", "/api/tickets/7"), { handler: "update", id: 7 }, "PATCH /api/tickets/7");
check(route("DELETE", "/api/tickets/7"), { handler: "notFound", id: null }, "DELETE isn't supported");
check(route("GET", "/api/tickets/abc"), { handler: "notFound", id: null }, "a non-numeric id");
@@hint
Match the path with `/^\/api\/tickets\/(\d+)$/`. If it matches, the id is `Number(m[1])`, and the method decides between `show` and `update`.
@@solution
function route(method, path) {
  if (path === "/api/tickets") {
    if (method === "GET") return { handler: "list", id: null };
    if (method === "POST") return { handler: "create", id: null };
  }
  const m = path.match(/^\/api\/tickets\/(\d+)$/);
  if (m) {
    if (method === "GET") return { handler: "show", id: Number(m[1]) };
    if (method === "PATCH") return { handler: "update", id: Number(m[1]) };
  }
  return { handler: "notFound", id: null };
}

@@exercise Validate a new ticket
Write `validateTicket(body)` that returns a list of error messages (empty when valid): `title is required` if the title is missing or blank, `title is too long` over 100 characters, and `priority must be 1-4` unless `priority` is a whole number from 1 to 4. Priority is optional (missing is fine).
@@starter
function validateTicket(body) {
  throw new Error("TODO: not done yet");
}

@@tests
check(validateTicket({ title: "Printer jam", priority: 2 }), [], "a valid ticket");
check(validateTicket({ title: "VPN down" }), [], "priority is optional");
check(validateTicket({ title: "   " }), ["title is required"], "a blank title");
check(validateTicket({ priority: 9 }), ["title is required", "priority must be 1-4"], "two problems");
check(validateTicket({ title: "x".repeat(101), priority: 1.5 }), ["title is too long", "priority must be 1-4"], "too long, and a decimal priority");
@@hint
`const title = (body.title ?? "").trim();`. For the priority: `if (body.priority !== undefined && !(Number.isInteger(body.priority) && body.priority >= 1 && body.priority <= 4))`.
@@solution
function validateTicket(body) {
  const errors = [];
  const title = typeof body.title === "string" ? body.title.trim() : "";
  if (!title) errors.push("title is required");
  else if (title.length > 100) errors.push("title is too long");
  const p = body.priority;
  if (p !== undefined && !(Number.isInteger(p) && p >= 1 && p <= 4)) errors.push("priority must be 1-4");
  return errors;
}
