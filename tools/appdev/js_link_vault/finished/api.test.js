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
