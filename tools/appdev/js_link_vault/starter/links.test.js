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
