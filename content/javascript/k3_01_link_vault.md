@@lesson k3-link-vault Link Vault (a full-stack application)
@@text
A complete web application with **no dependencies at all**: a Node.js back end with its own small web framework, a JSON file database written safely, and a front end in plain JavaScript. You save links, tag them, search them, and see which ones you actually use.

```text
npm start          (or: node server.js)   then open http://localhost:3000
npm test           (or: node --test)
```

## The pieces

| File | Job |
|---|---|
| `links.js` | the business rules: clean URLs, refuse duplicates, search, sort, count tags |
| `store.js` | keeps the data in `links.json`, written atomically (temp file + rename) |
| `router.js` | a mini framework: route patterns, JSON bodies, errors, static files |
| `server.js` | wires it together: the routes, and starting/stopping the server |
| `public/` | the front end: `index.html`, `style.css`, `app.js` |

The API:

| Request | Does |
|---|---|
| `GET /api/links?q=docs&tag=k8s&sort=popular` | search |
| `POST /api/links` `{"url", "title", "tags"}` | save (201, or 409 if it's already saved) |
| `PATCH /api/links/:id` | rename or retag |
| `DELETE /api/links/:id` | delete (204) |
| `POST /api/links/:id/visit` | count a visit |
| `GET /api/tags` | tags with counts |

## Your tasks

Four functions in the starter throw `TODO: not done yet`. Each has a comment above it saying exactly what it must do.

1. **`normalizeUrl`** (`links.js`): `WWW.Example.com/docs/?utm_source=mail#top` and `example.com/docs` are the same page. Use the built-in `URL` class. Only allow `http` and `https`: a saved `javascript:` link would run code when clicked.
2. **`list`** (`links.js`): search words, a tag filter and three sort orders.
3. **`compile`** (`router.js`): turn `/api/links/:id` into a `RegExp` and return the params. Escape the other characters, because `.` means "any character" in a regex.
4. **`resolveStatic`** (`router.js`): map a URL path to a file in `public/` and **refuse anything outside it**. `/../server.js` and `/%2e%2e/server.js` are real attacks that leak source code and passwords from badly written servers.

Do them in this order and run `node --test` after each. The router tests need `compile`, so the API tests only pass once it's done.
@@project link_vault Link Vault
Write the four functions that throw `TODO`, until `node --test` passes. Then `npm start` and open http://localhost:3000.
@@file links.js
// links.js: the business rules for saved links. No HTTP, no files: it gets a store and a clock.

class AppError extends Error {
  constructor(status, message) {
    super(message);
    this.status = status;
  }
}

// Tracking parameters that make the same page look like a different URL.
const TRACKING = /^(utm_\w+|fbclid|gclid|mc_eid)$/i;

/**
 * The canonical form of a URL, so duplicates can be found:
 * - add "https://" when there's no scheme; only http and https are allowed (AppError 400 "url must be http or https"),
 * - lower-case the host, drop "www.", drop the #fragment and tracking parameters (TRACKING),
 * - remove a trailing "/" from the path (but keep "https://example.com/" as is).
 * Anything new URL() can't parse is AppError 400 "url is not valid".
 */
function normalizeUrl(input) {
  throw new Error("TODO: not done yet");
}

function cleanTags(tags) {
  if (tags === undefined) return [];
  if (!Array.isArray(tags)) throw new AppError(400, "tags must be a list");
  return [...new Set(tags.map((t) => String(t).trim().toLowerCase()).filter(Boolean))].sort();
}

function createLinkService(store, clock = () => new Date()) {
  function find(id) {
    const link = store.data.links.find((l) => l.id === Number(id));
    if (!link) throw new AppError(404, `no link #${id}`);
    return link;
  }

  return {
    add({ url, title, tags } = {}) {
      const normalized = normalizeUrl(url);
      if (store.data.links.some((l) => l.url === normalized)) throw new AppError(409, "already saved");
      const cleanTitle = String(title ?? "").trim() || new URL(normalized).hostname;
      if (cleanTitle.length > 200) throw new AppError(400, "title is too long");
      const link = {
        id: store.data.nextId++,
        url: normalized,
        title: cleanTitle,
        tags: cleanTags(tags),
        created: clock().toISOString(),
        visits: 0,
      };
      store.data.links.push(link);
      store.save();
      return link;
    },

    get: find,

    /**
     * Search the saved links.
     * - q: every word must appear in the title or the url (case-insensitive),
     * - tag: only links with this tag,
     * - sort: "new" (newest first, the default), "popular" (most visits first, then newest)
     *   or "title" (A to Z, case-insensitive). Anything else is AppError 400 "unknown sort".
     */
    list({ q, tag, sort = "new" } = {}) {
      throw new Error("TODO: not done yet");
    },

    update(id, { title, tags } = {}) {
      const link = find(id);
      if (title !== undefined) {
        const t = String(title).trim();
        if (!t) throw new AppError(400, "title is required");
        link.title = t;
      }
      if (tags !== undefined) link.tags = cleanTags(tags);
      store.save();
      return link;
    },

    remove(id) {
      const link = find(id);
      store.data.links = store.data.links.filter((l) => l !== link);
      store.save();
    },

    visit(id) {
      const link = find(id);
      link.visits++;
      store.save();
      return link;
    },

    tags() {
      const counts = {};
      for (const l of store.data.links) for (const t of l.tags) counts[t] = (counts[t] ?? 0) + 1;
      return Object.entries(counts)
        .map(([name, count]) => ({ name, count }))
        .sort((a, b) => b.count - a.count || a.name.localeCompare(b.name));
    },
  };
}

module.exports = { AppError, normalizeUrl, cleanTags, createLinkService };
@@file store.js
// store.js: keeps the data in one JSON file, written safely.
const fs = require("node:fs");

const EMPTY = () => ({ nextId: 1, links: [] });

// file = null keeps everything in memory (for tests).
function createStore(file = null) {
  let data = EMPTY();
  if (file && fs.existsSync(file)) data = { ...EMPTY(), ...JSON.parse(fs.readFileSync(file, "utf8")) };

  return {
    get data() {
      return data;
    },
    set data(value) {
      data = value;
    },
    save() {
      if (!file) return;
      // Write a temporary file, then rename it over the real one. A rename is atomic, so a crash
      // in the middle leaves either the old file or the new one, never half a file.
      const tmp = `${file}.tmp`;
      fs.writeFileSync(tmp, JSON.stringify(data, null, 2));
      fs.renameSync(tmp, file);
    },
  };
}

module.exports = { createStore };
@@file router.js
// router.js: a small web framework in one file: routes, JSON bodies, errors and static files.
// Express and Fastify do the same jobs; writing it once shows you there's no magic.
const fs = require("node:fs");
const path = require("node:path");
const { AppError } = require("./links");

const TYPES = { ".html": "text/html; charset=utf-8", ".js": "text/javascript", ".css": "text/css", ".svg": "image/svg+xml", ".png": "image/png" };

/**
 * Turn a route pattern into a matcher: "/api/links/:id" matches "/api/links/7" and gives { id: "7" }.
 * A ":name" segment matches exactly one non-empty path segment (no "/"). Returns a function
 * path => params object, or null when the path doesn't match. Params are URL-decoded.
 */
function compile(pattern) {
  throw new Error("TODO: not done yet");
}

/**
 * The file to serve for a URL path, or null. "/" means index.html. The result must stay inside
 * publicDir: "/../server.js" or "/%2e%2e/secret" must return null (a "path traversal" attack).
 */
function resolveStatic(publicDir, urlPath) {
  throw new Error("TODO: not done yet");
}

function send(res, status, body) {
  const data = body === undefined ? "" : JSON.stringify(body);
  res.writeHead(status, { "Content-Type": "application/json", "Content-Length": Buffer.byteLength(data) });
  res.end(data);
}

async function readJson(req, limit = 100_000) {
  let size = 0;
  const chunks = [];
  for await (const chunk of req) {
    size += chunk.length;
    if (size > limit) throw new AppError(413, "body is too large");
    chunks.push(chunk);
  }
  if (size === 0) return {};
  try {
    const body = JSON.parse(Buffer.concat(chunks).toString("utf8"));
    if (typeof body !== "object" || body === null || Array.isArray(body)) throw new Error();
    return body;
  } catch {
    throw new AppError(400, "body must be a JSON object");
  }
}

function createRouter({ publicDir = null, log = () => {} } = {}) {
  const routes = [];

  async function handle(req, res) {
    const started = Date.now();
    const url = new URL(req.url, "http://localhost");
    res.on("finish", () => log(`${req.method} ${url.pathname} ${res.statusCode} ${Date.now() - started}ms`));
    try {
      let pathKnown = false;
      for (const r of routes) {
        const params = r.match(url.pathname);
        if (!params) continue;
        if (r.method !== req.method) {
          pathKnown = true;
          continue;
        }
        const body = req.method === "POST" || req.method === "PATCH" ? await readJson(req) : {};
        const query = Object.fromEntries(url.searchParams);
        const [status, result] = await r.handler({ params, query, body, req });
        send(res, status, result);
        return;
      }
      if (pathKnown) throw new AppError(405, "method not allowed");
      if (req.method === "GET" && publicDir && !url.pathname.startsWith("/api/")) {
        const file = resolveStatic(publicDir, url.pathname);
        if (file && fs.existsSync(file) && fs.statSync(file).isFile()) {
          res.writeHead(200, {
            "Content-Type": TYPES[path.extname(file)] ?? "application/octet-stream",
            "Content-Security-Policy": "default-src 'self'",
            "X-Content-Type-Options": "nosniff",
          });
          fs.createReadStream(file).pipe(res);
          return;
        }
      }
      throw new AppError(404, "not found");
    } catch (err) {
      if (err instanceof AppError) send(res, err.status, { error: err.message });
      else {
        log(`ERROR ${err.stack}`);
        send(res, 500, { error: "internal error" });
      }
    }
  }

  return {
    add(method, pattern, handler) {
      routes.push({ method, match: compile(pattern), handler });
      return this;
    },
    handle,
  };
}

module.exports = { compile, resolveStatic, readJson, send, createRouter };
@@file server.js
// server.js: wires the pieces together and starts the web server.
//   node server.js              then open http://localhost:3000
//   PORT=8080 DATA_FILE=/data/links.json node server.js
const http = require("node:http");
const path = require("node:path");
const { createStore } = require("./store");
const { createLinkService } = require("./links");
const { createRouter } = require("./router");

function createApp(service, options = {}) {
  const router = createRouter(options)
    .add("GET", "/api/links", ({ query }) => [200, service.list(query)])
    .add("POST", "/api/links", ({ body }) => [201, service.add(body)])
    .add("GET", "/api/links/:id", ({ params }) => [200, service.get(params.id)])
    .add("PATCH", "/api/links/:id", ({ params, body }) => [200, service.update(params.id, body)])
    .add("DELETE", "/api/links/:id", ({ params }) => {
      service.remove(params.id);
      return [204];
    })
    .add("POST", "/api/links/:id/visit", ({ params }) => [200, service.visit(params.id)])
    .add("GET", "/api/tags", () => [200, service.tags()])
    .add("GET", "/api/health", () => [200, { status: "ok" }]);
  return http.createServer(router.handle);
}

module.exports = { createApp };

if (require.main === module) {
  const port = Number(process.env.PORT ?? 3000);
  const store = createStore(process.env.DATA_FILE ?? path.join(__dirname, "links.json"));
  const server = createApp(createLinkService(store), {
    publicDir: path.join(__dirname, "public"),
    log: (line) => console.log(new Date().toISOString(), line),
  });
  server.listen(port, "127.0.0.1", () => console.log(`Link Vault on http://localhost:${port}`));
  // Finish requests in progress before exiting (Ctrl+C, or "docker stop").
  for (const signal of ["SIGINT", "SIGTERM"]) {
    process.on(signal, () => server.close(() => process.exit(0)));
  }
}
@@file links.test.js
// Run: node --test
const test = require("node:test");
const assert = require("node:assert/strict");
const { normalizeUrl, createLinkService } = require("./links");
const { createStore } = require("./store");

const NOW = new Date("2026-11-02T09:00:00Z");
const service = () => createLinkService(createStore(), () => NOW);
const status = (fn) => {
  try {
    fn();
    return "no error";
  } catch (e) {
    return `${e.status} ${e.message}`;
  }
};

test("normalizeUrl cleans URLs so duplicates are found", () => {
  assert.equal(normalizeUrl("example.com"), "https://example.com/");
  assert.equal(normalizeUrl("  HTTPS://WWW.Example.COM/Docs/  "), "https://example.com/Docs");
  assert.equal(normalizeUrl("https://example.com/a?utm_source=x&id=7&fbclid=y#top"), "https://example.com/a?id=7");
  assert.equal(normalizeUrl("http://localhost:3000/"), "http://localhost:3000/");
});

test("normalizeUrl rejects what isn't a web link", () => {
  assert.equal(status(() => normalizeUrl("ftp://files.example.com")), "400 url must be http or https");
  assert.equal(status(() => normalizeUrl("javascript:alert(1)")), "400 url must be http or https");
  assert.equal(status(() => normalizeUrl("not a url")), "400 url is not valid");
  assert.equal(status(() => normalizeUrl("")), "400 url is not valid");
});

test("add saves a clean link and refuses duplicates", () => {
  const s = service();
  const link = s.add({ url: "www.ipa.go.jp/", tags: ["Exam", "exam", " IPA "] });
  assert.deepEqual(link, {
    id: 1, url: "https://ipa.go.jp/", title: "ipa.go.jp", tags: ["exam", "ipa"], created: NOW.toISOString(), visits: 0,
  });
  assert.equal(status(() => s.add({ url: "https://ipa.go.jp/#top" })), "409 already saved");
  assert.equal(status(() => s.add({ url: "x.com", tags: "a,b" })), "400 tags must be a list");
});

test("list searches, filters by tag and sorts", () => {
  const s = service();
  s.add({ url: "kubernetes.io/docs", title: "Kubernetes docs", tags: ["k8s", "docs"] });
  s.add({ url: "go.dev/doc", title: "Go documentation", tags: ["go", "docs"] });
  s.add({ url: "aws.amazon.com/jp", title: "AWS Japan", tags: ["cloud"] });
  s.visit(1);
  s.visit(1);
  s.visit(3);
  const titles = (opts) => s.list(opts).map((l) => l.title);
  assert.deepEqual(titles(), ["AWS Japan", "Go documentation", "Kubernetes docs"]);
  assert.deepEqual(titles({ q: "DOC" }), ["Go documentation", "Kubernetes docs"]);
  assert.deepEqual(titles({ q: "go dev" }), ["Go documentation"]);
  assert.deepEqual(titles({ tag: "Docs", sort: "title" }), ["Go documentation", "Kubernetes docs"]);
  assert.deepEqual(titles({ sort: "popular" }), ["Kubernetes docs", "AWS Japan", "Go documentation"]);
  assert.equal(status(() => s.list({ sort: "random" })), "400 unknown sort");
});

test("update, remove, tags and 404s", () => {
  const s = service();
  s.add({ url: "a.example.com", tags: ["x", "y"] });
  s.add({ url: "b.example.com", tags: ["y"] });
  assert.equal(s.update(1, { title: "Alpha", tags: ["Z"] }).title, "Alpha");
  assert.deepEqual(s.tags(), [{ name: "y", count: 1 }, { name: "z", count: 1 }]);
  s.remove(1);
  assert.equal(status(() => s.get(1)), "404 no link #1");
  assert.equal(status(() => s.update(2, { title: " " })), "400 title is required");
});
@@file api.test.js
// Run: node --test
// These tests start the real server on a free port and talk to it with fetch, like a browser would.
const test = require("node:test");
const assert = require("node:assert/strict");
const path = require("node:path");
const { compile, resolveStatic } = require("./router");
const { createApp } = require("./server");
const { createLinkService } = require("./links");
const { createStore } = require("./store");

test("compile matches route patterns", () => {
  const m = compile("/api/links/:id/visit");
  assert.deepEqual(m("/api/links/42/visit"), { id: "42" });
  assert.equal(m("/api/links/42"), null);
  assert.equal(m("/api/links//visit"), null);
  assert.deepEqual(compile("/api/tags/:name")("/api/tags/c%2B%2B"), { name: "c++" });
  assert.deepEqual(compile("/api/health")("/api/health"), {});
  assert.equal(compile("/api.json")("/apixjson"), null); // a "." in a pattern is a real dot
});

test("resolveStatic never leaves the public folder", () => {
  const pub = path.join(__dirname, "public");
  assert.equal(resolveStatic(pub, "/"), path.join(pub, "index.html"));
  assert.equal(resolveStatic(pub, "/app.js"), path.join(pub, "app.js"));
  assert.equal(resolveStatic(pub, "/../server.js"), null);
  assert.equal(resolveStatic(pub, "/%2e%2e/server.js"), null);
  assert.equal(resolveStatic(pub, "/..%5cserver.js"), path.sep === "\\" ? null : path.join(pub, "..\\server.js"));
  assert.equal(resolveStatic(pub, "/%E0%A4%A"), null); // broken encoding
});

async function withServer(fn) {
  const server = createApp(createLinkService(createStore()), { publicDir: path.join(__dirname, "public") });
  await new Promise((resolve) => server.listen(0, "127.0.0.1", resolve));
  const base = `http://127.0.0.1:${server.address().port}`;
  const call = async (method, url, body) => {
    const res = await fetch(base + url, {
      method,
      headers: body === undefined ? {} : { "Content-Type": "application/json" },
      body: body === undefined ? undefined : typeof body === "string" ? body : JSON.stringify(body),
    });
    const text = await res.text();
    const type = res.headers.get("content-type") ?? "";
    return [res.status, type.includes("json") && text ? JSON.parse(text) : text];
  };
  try {
    await fn(call);
  } finally {
    await new Promise((resolve) => server.close(resolve));
  }
}

test("the API works end to end", () =>
  withServer(async (call) => {
    const [created, link] = await call("POST", "/api/links", { url: "https://kubernetes.io/docs/", tags: ["k8s"] });
    assert.equal(created, 201);
    assert.equal(link.url, "https://kubernetes.io/docs");
    assert.deepEqual(await call("POST", "/api/links", { url: "kubernetes.io/docs" }), [409, { error: "already saved" }]);
    assert.equal((await call("POST", `/api/links/${link.id}/visit`))[1].visits, 1);
    assert.equal((await call("PATCH", `/api/links/${link.id}`, { title: "K8s" }))[1].title, "K8s");
    const [, list] = await call("GET", "/api/links?tag=k8s");
    assert.deepEqual(list.map((l) => l.title), ["K8s"]);
    assert.deepEqual(await call("GET", "/api/tags"), [200, [{ name: "k8s", count: 1 }]]);
    assert.deepEqual(await call("DELETE", `/api/links/${link.id}`), [204, ""]);
    assert.equal((await call("GET", `/api/links/${link.id}`))[0], 404);
  }));

test("errors are JSON with the right status", () =>
  withServer(async (call) => {
    assert.deepEqual(await call("POST", "/api/links", "{not json"), [400, { error: "body must be a JSON object" }]);
    assert.deepEqual(await call("POST", "/api/links", [1, 2]), [400, { error: "body must be a JSON object" }]);
    assert.deepEqual(await call("PUT", "/api/links"), [405, { error: "method not allowed" }]);
    assert.deepEqual(await call("GET", "/api/nothing"), [404, { error: "not found" }]);
    assert.deepEqual(await call("GET", "/api/links?sort=x"), [400, { error: "unknown sort" }]);
  }));

test("the front end is served, with a CSP", () =>
  withServer(async (call) => {
    const [status, html] = await call("GET", "/");
    assert.equal(status, 200);
    assert.match(html, /<title>Link Vault<\/title>/);
    assert.equal((await call("GET", "/../server.js"))[0], 404);
  }));
@@file package.json
{
  "name": "link-vault",
  "version": "1.0.0",
  "private": true,
  "type": "commonjs",
  "description": "Save, tag and search links: a full-stack app with no dependencies.",
  "scripts": {
    "start": "node server.js",
    "test": "node --test"
  },
  "engines": {
    "node": ">=20"
  }
}
@@file public/index.html
<!doctype html>
<html lang="en">
<head>
  <meta charset="utf-8">
  <meta name="viewport" content="width=device-width, initial-scale=1">
  <title>Link Vault</title>
  <link rel="stylesheet" href="/style.css">
  <script src="/app.js" defer></script>
</head>
<body>
  <header>
    <h1>🔖 Link Vault</h1>
    <input id="search" type="search" placeholder="Search links…" aria-label="Search links">
    <select id="sort" aria-label="Sort">
      <option value="new">Newest</option>
      <option value="popular">Most visited</option>
      <option value="title">Title</option>
    </select>
  </header>
  <main>
    <form id="add">
      <input name="url" placeholder="https://…" required aria-label="URL">
      <input name="title" placeholder="Title (optional)" aria-label="Title">
      <input name="tags" placeholder="tags, comma separated" aria-label="Tags">
      <button>Save link</button>
    </form>
    <p id="message" role="status"></p>
    <nav id="tags" aria-label="Tags"></nav>
    <ul id="links"></ul>
    <p id="empty" hidden>No links yet. Save your first one above.</p>
  </main>
</body>
</html>
@@file public/style.css
:root { --bg: #f5f6fa; --card: #fff; --text: #1d2230; --muted: #667085; --accent: #3b5bdb; --border: #e1e5ee; }
@media (prefers-color-scheme: dark) {
  :root { --bg: #11141c; --card: #1a1f2b; --text: #e6e9f0; --muted: #98a2b3; --accent: #8ea6ff; --border: #2a3040; }
}
* { box-sizing: border-box; }
body { margin: 0; font: 15px/1.5 system-ui, sans-serif; background: var(--bg); color: var(--text); }
header { display: flex; flex-wrap: wrap; gap: 12px; align-items: center; padding: 16px 24px; border-bottom: 1px solid var(--border); }
header h1 { margin: 0 auto 0 0; font-size: 22px; }
main { max-width: 860px; margin: 0 auto; padding: 16px; }
input, select, button { font: inherit; padding: 8px 10px; border: 1px solid var(--border); border-radius: 8px; background: var(--card); color: var(--text); }
button { background: var(--accent); color: #fff; border: 0; cursor: pointer; }
#add { display: grid; grid-template-columns: 2fr 1.5fr 1fr auto; gap: 8px; }
@media (max-width: 640px) { #add { grid-template-columns: 1fr; } }
#message { min-height: 1.5em; color: var(--muted); }
#message.error { color: #e03131; }
#tags { display: flex; flex-wrap: wrap; gap: 6px; margin-bottom: 12px; }
#tags button { background: transparent; color: var(--accent); border: 1px solid var(--border); padding: 2px 10px; }
#tags button.active { background: var(--accent); color: #fff; }
ul { list-style: none; padding: 0; margin: 0; }
li { background: var(--card); border: 1px solid var(--border); border-radius: 10px; padding: 12px 14px; margin-bottom: 8px; display: grid; grid-template-columns: 1fr auto; gap: 4px 12px; }
li a { color: var(--accent); font-weight: 600; text-decoration: none; word-break: break-word; }
li small { color: var(--muted); grid-column: 1; word-break: break-all; }
li .actions { grid-row: 1 / 3; grid-column: 2; display: flex; gap: 6px; align-items: center; }
li .actions button { background: transparent; color: var(--muted); border: 1px solid var(--border); padding: 4px 8px; }
.tag { color: var(--accent); margin-right: 6px; }
@@file public/app.js
// app.js: the front end. One state object; render() draws the page from it; events change the
// state and call render() again. React, Vue and Svelte are this same loop, made faster and nicer.

const state = { links: [], tags: [], q: "", tag: null, sort: "new", message: "", error: false };
const $ = (id) => document.getElementById(id);

async function api(method, url, body) {
  const res = await fetch(url, {
    method,
    headers: body ? { "Content-Type": "application/json" } : {},
    body: body ? JSON.stringify(body) : undefined,
  });
  if (res.status === 204) return null;
  const data = await res.json();
  if (!res.ok) throw new Error(data.error ?? `HTTP ${res.status}`);
  return data;
}

async function load() {
  const params = new URLSearchParams({ sort: state.sort });
  if (state.q) params.set("q", state.q);
  if (state.tag) params.set("tag", state.tag);
  try {
    [state.links, state.tags] = await Promise.all([api("GET", `/api/links?${params}`), api("GET", "/api/tags")]);
  } catch (err) {
    show(err.message, true);
  }
  render();
}

function show(message, error = false) {
  state.message = message;
  state.error = error;
  render();
}

// Build elements with textContent, never innerHTML with user data: titles can't inject HTML.
function el(tag, { dataset = {}, ...props } = {}, ...children) {
  const node = Object.assign(document.createElement(tag), props);
  Object.assign(node.dataset, dataset); // node.dataset itself can't be replaced, only filled
  node.append(...children);
  return node;
}

function render() {
  $("message").textContent = state.message;
  $("message").className = state.error ? "error" : "";
  $("tags").replaceChildren(
    ...state.tags.map((t) =>
      el("button", { className: t.name === state.tag ? "active" : "", dataset: { tag: t.name } }, `#${t.name} ${t.count}`),
    ),
  );
  $("links").replaceChildren(
    ...state.links.map((l) =>
      el(
        "li",
        {},
        el("a", { href: l.url, target: "_blank", rel: "noopener noreferrer", dataset: { visit: l.id } }, l.title),
        el("small", {}, ...l.tags.map((t) => el("span", { className: "tag" }, `#${t}`)), `${l.url} · ${l.visits} visits`),
        el("span", { className: "actions" }, el("button", { dataset: { remove: l.id }, title: "Delete" }, "✕")),
      ),
    ),
  );
  $("empty").hidden = state.links.length > 0 || Boolean(state.q || state.tag);
}

$("add").addEventListener("submit", async (e) => {
  e.preventDefault();
  const form = new FormData(e.target);
  try {
    const link = await api("POST", "/api/links", {
      url: form.get("url"),
      title: form.get("title"),
      tags: String(form.get("tags")).split(",").map((t) => t.trim()).filter(Boolean),
    });
    e.target.reset();
    show(`Saved "${link.title}"`);
    load();
  } catch (err) {
    show(err.message, true);
  }
});

// Wait until the user stops typing for 250 ms before searching ("debouncing").
let timer;
$("search").addEventListener("input", (e) => {
  clearTimeout(timer);
  timer = setTimeout(() => {
    state.q = e.target.value.trim();
    load();
  }, 250);
});

$("sort").addEventListener("change", (e) => {
  state.sort = e.target.value;
  load();
});

// One listener for many buttons ("event delegation"): it works for elements render() creates later.
$("tags").addEventListener("click", (e) => {
  const tag = e.target.closest("button")?.dataset.tag;
  if (!tag) return;
  state.tag = state.tag === tag ? null : tag;
  load();
});

$("links").addEventListener("click", async (e) => {
  const remove = e.target.closest("[data-remove]");
  const visit = e.target.closest("[data-visit]");
  if (remove && confirm("Delete this link?")) {
    await api("DELETE", `/api/links/${remove.dataset.remove}`).catch((err) => show(err.message, true));
    load();
  } else if (visit) {
    api("POST", `/api/links/${visit.dataset.visit}/visit`).then(load, () => {});
  }
});

load();
@@text
## Stretch goals

- **Fetch titles automatically**: when no title is given, `fetch()` the page on the server and read its `<title>`. Use a timeout (`AbortSignal.timeout(5000)`), a size limit, and refuse private addresses (`10.x`, `192.168.x`, `localhost`) so your server can't be used to probe the internal network (that attack is called **SSRF**).
- **Import and export**: read a browser bookmarks HTML export; download everything as JSON.
- **Accounts**: users, `crypto.scrypt` password hashes, a session cookie (`HttpOnly; SameSite=Lax`), and every link belongs to a user.
- **SQLite instead of JSON**: Node 22.5+ has `node:sqlite` built in. Only `store.js` and `links.js` should need to change.
- **Rewrite the front end in React or Svelte**, keeping the same API. Then rewrite the back end with Express or Fastify, keeping the same tests.
- **Ship it**: a `Dockerfile` (`FROM node:24-slim`, `USER node`, data in a volume), behind Caddy for HTTPS.
@@project link_vault_finished Link Vault: the finished version
A complete solution to compare with yours.
@@file links.js
// links.js: the business rules for saved links. No HTTP, no files: it gets a store and a clock.

class AppError extends Error {
  constructor(status, message) {
    super(message);
    this.status = status;
  }
}

// Tracking parameters that make the same page look like a different URL.
const TRACKING = /^(utm_\w+|fbclid|gclid|mc_eid)$/i;

/**
 * The canonical form of a URL, so duplicates can be found:
 * - add "https://" when there's no scheme; only http and https are allowed (AppError 400 "url must be http or https"),
 * - lower-case the host, drop "www.", drop the #fragment and tracking parameters (TRACKING),
 * - remove a trailing "/" from the path (but keep "https://example.com/" as is).
 * Anything new URL() can't parse is AppError 400 "url is not valid".
 */
function normalizeUrl(input) {
  let text = String(input ?? "").trim();
  if (!/^[a-z][a-z0-9+.-]*:/i.test(text)) text = "https://" + text;
  let url;
  try {
    url = new URL(text);
  } catch {
    throw new AppError(400, "url is not valid");
  }
  if (url.protocol !== "http:" && url.protocol !== "https:") throw new AppError(400, "url must be http or https");
  if (!url.hostname.includes(".") && url.hostname !== "localhost") throw new AppError(400, "url is not valid");
  url.hostname = url.hostname.toLowerCase().replace(/^www\./, "");
  url.hash = "";
  for (const key of [...url.searchParams.keys()]) {
    if (TRACKING.test(key)) url.searchParams.delete(key);
  }
  if (url.pathname.length > 1 && url.pathname.endsWith("/")) url.pathname = url.pathname.replace(/\/+$/, "");
  return url.toString();
}

function cleanTags(tags) {
  if (tags === undefined) return [];
  if (!Array.isArray(tags)) throw new AppError(400, "tags must be a list");
  return [...new Set(tags.map((t) => String(t).trim().toLowerCase()).filter(Boolean))].sort();
}

function createLinkService(store, clock = () => new Date()) {
  function find(id) {
    const link = store.data.links.find((l) => l.id === Number(id));
    if (!link) throw new AppError(404, `no link #${id}`);
    return link;
  }

  return {
    add({ url, title, tags } = {}) {
      const normalized = normalizeUrl(url);
      if (store.data.links.some((l) => l.url === normalized)) throw new AppError(409, "already saved");
      const cleanTitle = String(title ?? "").trim() || new URL(normalized).hostname;
      if (cleanTitle.length > 200) throw new AppError(400, "title is too long");
      const link = {
        id: store.data.nextId++,
        url: normalized,
        title: cleanTitle,
        tags: cleanTags(tags),
        created: clock().toISOString(),
        visits: 0,
      };
      store.data.links.push(link);
      store.save();
      return link;
    },

    get: find,

    /**
     * Search the saved links.
     * - q: every word must appear in the title or the url (case-insensitive),
     * - tag: only links with this tag,
     * - sort: "new" (newest first, the default), "popular" (most visits first, then newest)
     *   or "title" (A to Z, case-insensitive). Anything else is AppError 400 "unknown sort".
     */
    list({ q, tag, sort = "new" } = {}) {
      const sorters = {
        new: (a, b) => b.id - a.id,
        popular: (a, b) => b.visits - a.visits || b.id - a.id,
        title: (a, b) => a.title.localeCompare(b.title, "en", { sensitivity: "base" }),
      };
      if (!sorters[sort]) throw new AppError(400, "unknown sort");
      const words = String(q ?? "").toLowerCase().split(/\s+/).filter(Boolean);
      const wanted = tag ? String(tag).toLowerCase() : null;
      return store.data.links
        .filter((l) => !wanted || l.tags.includes(wanted))
        .filter((l) => words.every((w) => `${l.title} ${l.url}`.toLowerCase().includes(w)))
        .sort(sorters[sort]);
    },

    update(id, { title, tags } = {}) {
      const link = find(id);
      if (title !== undefined) {
        const t = String(title).trim();
        if (!t) throw new AppError(400, "title is required");
        link.title = t;
      }
      if (tags !== undefined) link.tags = cleanTags(tags);
      store.save();
      return link;
    },

    remove(id) {
      const link = find(id);
      store.data.links = store.data.links.filter((l) => l !== link);
      store.save();
    },

    visit(id) {
      const link = find(id);
      link.visits++;
      store.save();
      return link;
    },

    tags() {
      const counts = {};
      for (const l of store.data.links) for (const t of l.tags) counts[t] = (counts[t] ?? 0) + 1;
      return Object.entries(counts)
        .map(([name, count]) => ({ name, count }))
        .sort((a, b) => b.count - a.count || a.name.localeCompare(b.name));
    },
  };
}

module.exports = { AppError, normalizeUrl, cleanTags, createLinkService };
@@file store.js
// store.js: keeps the data in one JSON file, written safely.
const fs = require("node:fs");

const EMPTY = () => ({ nextId: 1, links: [] });

// file = null keeps everything in memory (for tests).
function createStore(file = null) {
  let data = EMPTY();
  if (file && fs.existsSync(file)) data = { ...EMPTY(), ...JSON.parse(fs.readFileSync(file, "utf8")) };

  return {
    get data() {
      return data;
    },
    set data(value) {
      data = value;
    },
    save() {
      if (!file) return;
      // Write a temporary file, then rename it over the real one. A rename is atomic, so a crash
      // in the middle leaves either the old file or the new one, never half a file.
      const tmp = `${file}.tmp`;
      fs.writeFileSync(tmp, JSON.stringify(data, null, 2));
      fs.renameSync(tmp, file);
    },
  };
}

module.exports = { createStore };
@@file router.js
// router.js: a small web framework in one file: routes, JSON bodies, errors and static files.
// Express and Fastify do the same jobs; writing it once shows you there's no magic.
const fs = require("node:fs");
const path = require("node:path");
const { AppError } = require("./links");

const TYPES = { ".html": "text/html; charset=utf-8", ".js": "text/javascript", ".css": "text/css", ".svg": "image/svg+xml", ".png": "image/png" };

/**
 * Turn a route pattern into a matcher: "/api/links/:id" matches "/api/links/7" and gives { id: "7" }.
 * A ":name" segment matches exactly one non-empty path segment (no "/"). Returns a function
 * path => params object, or null when the path doesn't match. Params are URL-decoded.
 */
function compile(pattern) {
  const names = [];
  const source = pattern
    .split("/")
    .map((part) => {
      if (part.startsWith(":")) {
        names.push(part.slice(1));
        return "([^/]+)";
      }
      return part.replace(/[.*+?^${}()|[\]\\]/g, "\\$&");
    })
    .join("/");
  const regex = new RegExp(`^${source}$`);
  return (p) => {
    const m = regex.exec(p);
    if (!m) return null;
    return Object.fromEntries(names.map((n, i) => [n, decodeURIComponent(m[i + 1])]));
  };
}

/**
 * The file to serve for a URL path, or null. "/" means index.html. The result must stay inside
 * publicDir: "/../server.js" or "/%2e%2e/secret" must return null (a "path traversal" attack).
 */
function resolveStatic(publicDir, urlPath) {
  let decoded;
  try {
    decoded = decodeURIComponent(urlPath);
  } catch {
    return null;
  }
  if (decoded.includes("\0")) return null;
  const root = path.resolve(publicDir);
  const file = path.resolve(root, "." + (decoded === "/" ? "/index.html" : decoded));
  if (file !== root && !file.startsWith(root + path.sep)) return null;
  return file;
}

function send(res, status, body) {
  const data = body === undefined ? "" : JSON.stringify(body);
  res.writeHead(status, { "Content-Type": "application/json", "Content-Length": Buffer.byteLength(data) });
  res.end(data);
}

async function readJson(req, limit = 100_000) {
  let size = 0;
  const chunks = [];
  for await (const chunk of req) {
    size += chunk.length;
    if (size > limit) throw new AppError(413, "body is too large");
    chunks.push(chunk);
  }
  if (size === 0) return {};
  try {
    const body = JSON.parse(Buffer.concat(chunks).toString("utf8"));
    if (typeof body !== "object" || body === null || Array.isArray(body)) throw new Error();
    return body;
  } catch {
    throw new AppError(400, "body must be a JSON object");
  }
}

function createRouter({ publicDir = null, log = () => {} } = {}) {
  const routes = [];

  async function handle(req, res) {
    const started = Date.now();
    const url = new URL(req.url, "http://localhost");
    res.on("finish", () => log(`${req.method} ${url.pathname} ${res.statusCode} ${Date.now() - started}ms`));
    try {
      let pathKnown = false;
      for (const r of routes) {
        const params = r.match(url.pathname);
        if (!params) continue;
        if (r.method !== req.method) {
          pathKnown = true;
          continue;
        }
        const body = req.method === "POST" || req.method === "PATCH" ? await readJson(req) : {};
        const query = Object.fromEntries(url.searchParams);
        const [status, result] = await r.handler({ params, query, body, req });
        send(res, status, result);
        return;
      }
      if (pathKnown) throw new AppError(405, "method not allowed");
      if (req.method === "GET" && publicDir && !url.pathname.startsWith("/api/")) {
        const file = resolveStatic(publicDir, url.pathname);
        if (file && fs.existsSync(file) && fs.statSync(file).isFile()) {
          res.writeHead(200, {
            "Content-Type": TYPES[path.extname(file)] ?? "application/octet-stream",
            "Content-Security-Policy": "default-src 'self'",
            "X-Content-Type-Options": "nosniff",
          });
          fs.createReadStream(file).pipe(res);
          return;
        }
      }
      throw new AppError(404, "not found");
    } catch (err) {
      if (err instanceof AppError) send(res, err.status, { error: err.message });
      else {
        log(`ERROR ${err.stack}`);
        send(res, 500, { error: "internal error" });
      }
    }
  }

  return {
    add(method, pattern, handler) {
      routes.push({ method, match: compile(pattern), handler });
      return this;
    },
    handle,
  };
}

module.exports = { compile, resolveStatic, readJson, send, createRouter };
@@file server.js
// server.js: wires the pieces together and starts the web server.
//   node server.js              then open http://localhost:3000
//   PORT=8080 DATA_FILE=/data/links.json node server.js
const http = require("node:http");
const path = require("node:path");
const { createStore } = require("./store");
const { createLinkService } = require("./links");
const { createRouter } = require("./router");

function createApp(service, options = {}) {
  const router = createRouter(options)
    .add("GET", "/api/links", ({ query }) => [200, service.list(query)])
    .add("POST", "/api/links", ({ body }) => [201, service.add(body)])
    .add("GET", "/api/links/:id", ({ params }) => [200, service.get(params.id)])
    .add("PATCH", "/api/links/:id", ({ params, body }) => [200, service.update(params.id, body)])
    .add("DELETE", "/api/links/:id", ({ params }) => {
      service.remove(params.id);
      return [204];
    })
    .add("POST", "/api/links/:id/visit", ({ params }) => [200, service.visit(params.id)])
    .add("GET", "/api/tags", () => [200, service.tags()])
    .add("GET", "/api/health", () => [200, { status: "ok" }]);
  return http.createServer(router.handle);
}

module.exports = { createApp };

if (require.main === module) {
  const port = Number(process.env.PORT ?? 3000);
  const store = createStore(process.env.DATA_FILE ?? path.join(__dirname, "links.json"));
  const server = createApp(createLinkService(store), {
    publicDir: path.join(__dirname, "public"),
    log: (line) => console.log(new Date().toISOString(), line),
  });
  server.listen(port, "127.0.0.1", () => console.log(`Link Vault on http://localhost:${port}`));
  // Finish requests in progress before exiting (Ctrl+C, or "docker stop").
  for (const signal of ["SIGINT", "SIGTERM"]) {
    process.on(signal, () => server.close(() => process.exit(0)));
  }
}
@@file links.test.js
// Run: node --test
const test = require("node:test");
const assert = require("node:assert/strict");
const { normalizeUrl, createLinkService } = require("./links");
const { createStore } = require("./store");

const NOW = new Date("2026-11-02T09:00:00Z");
const service = () => createLinkService(createStore(), () => NOW);
const status = (fn) => {
  try {
    fn();
    return "no error";
  } catch (e) {
    return `${e.status} ${e.message}`;
  }
};

test("normalizeUrl cleans URLs so duplicates are found", () => {
  assert.equal(normalizeUrl("example.com"), "https://example.com/");
  assert.equal(normalizeUrl("  HTTPS://WWW.Example.COM/Docs/  "), "https://example.com/Docs");
  assert.equal(normalizeUrl("https://example.com/a?utm_source=x&id=7&fbclid=y#top"), "https://example.com/a?id=7");
  assert.equal(normalizeUrl("http://localhost:3000/"), "http://localhost:3000/");
});

test("normalizeUrl rejects what isn't a web link", () => {
  assert.equal(status(() => normalizeUrl("ftp://files.example.com")), "400 url must be http or https");
  assert.equal(status(() => normalizeUrl("javascript:alert(1)")), "400 url must be http or https");
  assert.equal(status(() => normalizeUrl("not a url")), "400 url is not valid");
  assert.equal(status(() => normalizeUrl("")), "400 url is not valid");
});

test("add saves a clean link and refuses duplicates", () => {
  const s = service();
  const link = s.add({ url: "www.ipa.go.jp/", tags: ["Exam", "exam", " IPA "] });
  assert.deepEqual(link, {
    id: 1, url: "https://ipa.go.jp/", title: "ipa.go.jp", tags: ["exam", "ipa"], created: NOW.toISOString(), visits: 0,
  });
  assert.equal(status(() => s.add({ url: "https://ipa.go.jp/#top" })), "409 already saved");
  assert.equal(status(() => s.add({ url: "x.com", tags: "a,b" })), "400 tags must be a list");
});

test("list searches, filters by tag and sorts", () => {
  const s = service();
  s.add({ url: "kubernetes.io/docs", title: "Kubernetes docs", tags: ["k8s", "docs"] });
  s.add({ url: "go.dev/doc", title: "Go documentation", tags: ["go", "docs"] });
  s.add({ url: "aws.amazon.com/jp", title: "AWS Japan", tags: ["cloud"] });
  s.visit(1);
  s.visit(1);
  s.visit(3);
  const titles = (opts) => s.list(opts).map((l) => l.title);
  assert.deepEqual(titles(), ["AWS Japan", "Go documentation", "Kubernetes docs"]);
  assert.deepEqual(titles({ q: "DOC" }), ["Go documentation", "Kubernetes docs"]);
  assert.deepEqual(titles({ q: "go dev" }), ["Go documentation"]);
  assert.deepEqual(titles({ tag: "Docs", sort: "title" }), ["Go documentation", "Kubernetes docs"]);
  assert.deepEqual(titles({ sort: "popular" }), ["Kubernetes docs", "AWS Japan", "Go documentation"]);
  assert.equal(status(() => s.list({ sort: "random" })), "400 unknown sort");
});

test("update, remove, tags and 404s", () => {
  const s = service();
  s.add({ url: "a.example.com", tags: ["x", "y"] });
  s.add({ url: "b.example.com", tags: ["y"] });
  assert.equal(s.update(1, { title: "Alpha", tags: ["Z"] }).title, "Alpha");
  assert.deepEqual(s.tags(), [{ name: "y", count: 1 }, { name: "z", count: 1 }]);
  s.remove(1);
  assert.equal(status(() => s.get(1)), "404 no link #1");
  assert.equal(status(() => s.update(2, { title: " " })), "400 title is required");
});
@@file api.test.js
// Run: node --test
// These tests start the real server on a free port and talk to it with fetch, like a browser would.
const test = require("node:test");
const assert = require("node:assert/strict");
const path = require("node:path");
const { compile, resolveStatic } = require("./router");
const { createApp } = require("./server");
const { createLinkService } = require("./links");
const { createStore } = require("./store");

test("compile matches route patterns", () => {
  const m = compile("/api/links/:id/visit");
  assert.deepEqual(m("/api/links/42/visit"), { id: "42" });
  assert.equal(m("/api/links/42"), null);
  assert.equal(m("/api/links//visit"), null);
  assert.deepEqual(compile("/api/tags/:name")("/api/tags/c%2B%2B"), { name: "c++" });
  assert.deepEqual(compile("/api/health")("/api/health"), {});
  assert.equal(compile("/api.json")("/apixjson"), null); // a "." in a pattern is a real dot
});

test("resolveStatic never leaves the public folder", () => {
  const pub = path.join(__dirname, "public");
  assert.equal(resolveStatic(pub, "/"), path.join(pub, "index.html"));
  assert.equal(resolveStatic(pub, "/app.js"), path.join(pub, "app.js"));
  assert.equal(resolveStatic(pub, "/../server.js"), null);
  assert.equal(resolveStatic(pub, "/%2e%2e/server.js"), null);
  assert.equal(resolveStatic(pub, "/..%5cserver.js"), path.sep === "\\" ? null : path.join(pub, "..\\server.js"));
  assert.equal(resolveStatic(pub, "/%E0%A4%A"), null); // broken encoding
});

async function withServer(fn) {
  const server = createApp(createLinkService(createStore()), { publicDir: path.join(__dirname, "public") });
  await new Promise((resolve) => server.listen(0, "127.0.0.1", resolve));
  const base = `http://127.0.0.1:${server.address().port}`;
  const call = async (method, url, body) => {
    const res = await fetch(base + url, {
      method,
      headers: body === undefined ? {} : { "Content-Type": "application/json" },
      body: body === undefined ? undefined : typeof body === "string" ? body : JSON.stringify(body),
    });
    const text = await res.text();
    const type = res.headers.get("content-type") ?? "";
    return [res.status, type.includes("json") && text ? JSON.parse(text) : text];
  };
  try {
    await fn(call);
  } finally {
    await new Promise((resolve) => server.close(resolve));
  }
}

test("the API works end to end", () =>
  withServer(async (call) => {
    const [created, link] = await call("POST", "/api/links", { url: "https://kubernetes.io/docs/", tags: ["k8s"] });
    assert.equal(created, 201);
    assert.equal(link.url, "https://kubernetes.io/docs");
    assert.deepEqual(await call("POST", "/api/links", { url: "kubernetes.io/docs" }), [409, { error: "already saved" }]);
    assert.equal((await call("POST", `/api/links/${link.id}/visit`))[1].visits, 1);
    assert.equal((await call("PATCH", `/api/links/${link.id}`, { title: "K8s" }))[1].title, "K8s");
    const [, list] = await call("GET", "/api/links?tag=k8s");
    assert.deepEqual(list.map((l) => l.title), ["K8s"]);
    assert.deepEqual(await call("GET", "/api/tags"), [200, [{ name: "k8s", count: 1 }]]);
    assert.deepEqual(await call("DELETE", `/api/links/${link.id}`), [204, ""]);
    assert.equal((await call("GET", `/api/links/${link.id}`))[0], 404);
  }));

test("errors are JSON with the right status", () =>
  withServer(async (call) => {
    assert.deepEqual(await call("POST", "/api/links", "{not json"), [400, { error: "body must be a JSON object" }]);
    assert.deepEqual(await call("POST", "/api/links", [1, 2]), [400, { error: "body must be a JSON object" }]);
    assert.deepEqual(await call("PUT", "/api/links"), [405, { error: "method not allowed" }]);
    assert.deepEqual(await call("GET", "/api/nothing"), [404, { error: "not found" }]);
    assert.deepEqual(await call("GET", "/api/links?sort=x"), [400, { error: "unknown sort" }]);
  }));

test("the front end is served, with a CSP", () =>
  withServer(async (call) => {
    const [status, html] = await call("GET", "/");
    assert.equal(status, 200);
    assert.match(html, /<title>Link Vault<\/title>/);
    assert.equal((await call("GET", "/../server.js"))[0], 404);
  }));
@@file package.json
{
  "name": "link-vault",
  "version": "1.0.0",
  "private": true,
  "type": "commonjs",
  "description": "Save, tag and search links: a full-stack app with no dependencies.",
  "scripts": {
    "start": "node server.js",
    "test": "node --test"
  },
  "engines": {
    "node": ">=20"
  }
}
@@file public/index.html
<!doctype html>
<html lang="en">
<head>
  <meta charset="utf-8">
  <meta name="viewport" content="width=device-width, initial-scale=1">
  <title>Link Vault</title>
  <link rel="stylesheet" href="/style.css">
  <script src="/app.js" defer></script>
</head>
<body>
  <header>
    <h1>🔖 Link Vault</h1>
    <input id="search" type="search" placeholder="Search links…" aria-label="Search links">
    <select id="sort" aria-label="Sort">
      <option value="new">Newest</option>
      <option value="popular">Most visited</option>
      <option value="title">Title</option>
    </select>
  </header>
  <main>
    <form id="add">
      <input name="url" placeholder="https://…" required aria-label="URL">
      <input name="title" placeholder="Title (optional)" aria-label="Title">
      <input name="tags" placeholder="tags, comma separated" aria-label="Tags">
      <button>Save link</button>
    </form>
    <p id="message" role="status"></p>
    <nav id="tags" aria-label="Tags"></nav>
    <ul id="links"></ul>
    <p id="empty" hidden>No links yet. Save your first one above.</p>
  </main>
</body>
</html>
@@file public/style.css
:root { --bg: #f5f6fa; --card: #fff; --text: #1d2230; --muted: #667085; --accent: #3b5bdb; --border: #e1e5ee; }
@media (prefers-color-scheme: dark) {
  :root { --bg: #11141c; --card: #1a1f2b; --text: #e6e9f0; --muted: #98a2b3; --accent: #8ea6ff; --border: #2a3040; }
}
* { box-sizing: border-box; }
body { margin: 0; font: 15px/1.5 system-ui, sans-serif; background: var(--bg); color: var(--text); }
header { display: flex; flex-wrap: wrap; gap: 12px; align-items: center; padding: 16px 24px; border-bottom: 1px solid var(--border); }
header h1 { margin: 0 auto 0 0; font-size: 22px; }
main { max-width: 860px; margin: 0 auto; padding: 16px; }
input, select, button { font: inherit; padding: 8px 10px; border: 1px solid var(--border); border-radius: 8px; background: var(--card); color: var(--text); }
button { background: var(--accent); color: #fff; border: 0; cursor: pointer; }
#add { display: grid; grid-template-columns: 2fr 1.5fr 1fr auto; gap: 8px; }
@media (max-width: 640px) { #add { grid-template-columns: 1fr; } }
#message { min-height: 1.5em; color: var(--muted); }
#message.error { color: #e03131; }
#tags { display: flex; flex-wrap: wrap; gap: 6px; margin-bottom: 12px; }
#tags button { background: transparent; color: var(--accent); border: 1px solid var(--border); padding: 2px 10px; }
#tags button.active { background: var(--accent); color: #fff; }
ul { list-style: none; padding: 0; margin: 0; }
li { background: var(--card); border: 1px solid var(--border); border-radius: 10px; padding: 12px 14px; margin-bottom: 8px; display: grid; grid-template-columns: 1fr auto; gap: 4px 12px; }
li a { color: var(--accent); font-weight: 600; text-decoration: none; word-break: break-word; }
li small { color: var(--muted); grid-column: 1; word-break: break-all; }
li .actions { grid-row: 1 / 3; grid-column: 2; display: flex; gap: 6px; align-items: center; }
li .actions button { background: transparent; color: var(--muted); border: 1px solid var(--border); padding: 4px 8px; }
.tag { color: var(--accent); margin-right: 6px; }
@@file public/app.js
// app.js: the front end. One state object; render() draws the page from it; events change the
// state and call render() again. React, Vue and Svelte are this same loop, made faster and nicer.

const state = { links: [], tags: [], q: "", tag: null, sort: "new", message: "", error: false };
const $ = (id) => document.getElementById(id);

async function api(method, url, body) {
  const res = await fetch(url, {
    method,
    headers: body ? { "Content-Type": "application/json" } : {},
    body: body ? JSON.stringify(body) : undefined,
  });
  if (res.status === 204) return null;
  const data = await res.json();
  if (!res.ok) throw new Error(data.error ?? `HTTP ${res.status}`);
  return data;
}

async function load() {
  const params = new URLSearchParams({ sort: state.sort });
  if (state.q) params.set("q", state.q);
  if (state.tag) params.set("tag", state.tag);
  try {
    [state.links, state.tags] = await Promise.all([api("GET", `/api/links?${params}`), api("GET", "/api/tags")]);
  } catch (err) {
    show(err.message, true);
  }
  render();
}

function show(message, error = false) {
  state.message = message;
  state.error = error;
  render();
}

// Build elements with textContent, never innerHTML with user data: titles can't inject HTML.
function el(tag, { dataset = {}, ...props } = {}, ...children) {
  const node = Object.assign(document.createElement(tag), props);
  Object.assign(node.dataset, dataset); // node.dataset itself can't be replaced, only filled
  node.append(...children);
  return node;
}

function render() {
  $("message").textContent = state.message;
  $("message").className = state.error ? "error" : "";
  $("tags").replaceChildren(
    ...state.tags.map((t) =>
      el("button", { className: t.name === state.tag ? "active" : "", dataset: { tag: t.name } }, `#${t.name} ${t.count}`),
    ),
  );
  $("links").replaceChildren(
    ...state.links.map((l) =>
      el(
        "li",
        {},
        el("a", { href: l.url, target: "_blank", rel: "noopener noreferrer", dataset: { visit: l.id } }, l.title),
        el("small", {}, ...l.tags.map((t) => el("span", { className: "tag" }, `#${t}`)), `${l.url} · ${l.visits} visits`),
        el("span", { className: "actions" }, el("button", { dataset: { remove: l.id }, title: "Delete" }, "✕")),
      ),
    ),
  );
  $("empty").hidden = state.links.length > 0 || Boolean(state.q || state.tag);
}

$("add").addEventListener("submit", async (e) => {
  e.preventDefault();
  const form = new FormData(e.target);
  try {
    const link = await api("POST", "/api/links", {
      url: form.get("url"),
      title: form.get("title"),
      tags: String(form.get("tags")).split(",").map((t) => t.trim()).filter(Boolean),
    });
    e.target.reset();
    show(`Saved "${link.title}"`);
    load();
  } catch (err) {
    show(err.message, true);
  }
});

// Wait until the user stops typing for 250 ms before searching ("debouncing").
let timer;
$("search").addEventListener("input", (e) => {
  clearTimeout(timer);
  timer = setTimeout(() => {
    state.q = e.target.value.trim();
    load();
  }, 250);
});

$("sort").addEventListener("change", (e) => {
  state.sort = e.target.value;
  load();
});

// One listener for many buttons ("event delegation"): it works for elements render() creates later.
$("tags").addEventListener("click", (e) => {
  const tag = e.target.closest("button")?.dataset.tag;
  if (!tag) return;
  state.tag = state.tag === tag ? null : tag;
  load();
});

$("links").addEventListener("click", async (e) => {
  const remove = e.target.closest("[data-remove]");
  const visit = e.target.closest("[data-visit]");
  if (remove && confirm("Delete this link?")) {
    await api("DELETE", `/api/links/${remove.dataset.remove}`).catch((err) => show(err.message, true));
    load();
  } else if (visit) {
    api("POST", `/api/links/${visit.dataset.visit}/visit`).then(load, () => {});
  }
});

load();

@@drill
Which HTTP method is used to delete a resource?
= DELETE
? DELETE /links/42.

@@text
🎉 You've built every layer of a web application yourself: HTTP routing, input validation, safe file storage, security headers and a reactive front end. Frameworks will now feel like shortcuts you understand rather than magic you depend on.
