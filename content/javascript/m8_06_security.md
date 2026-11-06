@@lesson m8-security JavaScript Security: Injection, Prototype Pollution, ReDoS and the Supply Chain
@@concepts text-processing, objects
@@text
JavaScript runs in the most hostile environment there is: the browser, next to untrusted input, and on servers exposed to the internet, built from hundreds of third-party packages. This lesson covers the attacks specific to JavaScript and Node.js, on top of the general web security in the Professional JavaScript lesson.

## Cross-site scripting (XSS)

XSS happens when untrusted text is inserted into a page **as HTML**, so an attacker's `<script>` or `<img onerror=...>` runs in your users' browsers with their session.

```javascript,norun
element.innerHTML = `<b>${comment}</b>`;     // DANGER
element.textContent = comment;               // safe: always text
```

- Use `textContent`, or a framework that escapes by default (React, Vue, Svelte, lit). Be careful with their escape hatches: `dangerouslySetInnerHTML`, `v-html`.
- When you must render user HTML (rich text), sanitise it with **DOMPurify**.
- Add a **Content-Security-Policy** header so injected inline scripts don't run even if something slips through.
- URLs are another vector: `<a href="javascript:alert(1)">`. Allow only `http:`/`https:` for user-supplied links.

## Injection on the server

- **SQL**: always use parameters (`db.query("SELECT * FROM users WHERE id = $1", [id])`), never template strings.
- **Commands**: `execFile`/`spawn` with argument arrays, never `exec` with input (see the worker and process lesson).
- **Paths**: `path.join(base, userInput)` can escape with `../`; resolve and check the result stays inside `base`.
- **NoSQL**: `{ password: { $ne: null } }` sent as JSON can bypass a MongoDB login; validate that fields are the types you expect.
- `eval`, `new Function` and `vm` with user input are code execution. Node's `vm` module is **not** a security sandbox.

## Prototype pollution

Merging untrusted JSON into objects can change `Object.prototype`, which affects **every object** in the process:

```javascript
function naiveMerge(target, source) {
  for (const key in source) {
    if (typeof source[key] === "object" && source[key] !== null) {
      target[key] ??= {};
      naiveMerge(target[key], source[key]);
    } else {
      target[key] = source[key];
    }
  }
  return target;
}
const evil = JSON.parse('{"__proto__": {"isAdmin": true}}');
naiveMerge({}, evil);
console.log({}.isAdmin);                 // true: every object is now "admin"
delete Object.prototype.isAdmin;        // undo the damage for this demo
```

Defences: skip the keys `__proto__`, `constructor` and `prototype` when merging; use `Map` or `Object.create(null)` for dictionaries built from input; validate input against a schema; `Object.freeze(Object.prototype)` in hardened apps; Node's `--disable-proto=delete` flag.

## ReDoS: regular expression denial of service

Some regexes take **exponential** time on certain inputs, because of nested quantifiers that can match the same text in many ways:

```javascript
const evilPattern = /^(a+)+$/;
const start = Date.now();
evilPattern.test("a".repeat(22) + "!");   // each extra "a" doubles the time
console.log("took", Date.now() - start, "ms for 23 characters");
```

On a server, one request with a crafted string can freeze the event loop for minutes. Avoid nested quantifiers like `(a+)+`, `(a|aa)*`, `(.*)*`; limit input length before matching; lint with `eslint-plugin-regexp`; or use the `re2` package (linear-time engine) for user-supplied patterns.

## Secrets and data exposure

- Keep secrets in environment variables or a secret manager, never in the repository or in front-end code (anything shipped to the browser is public, including "hidden" API keys in a bundle).
- Don't send stack traces to clients in production; log them server-side.
- Don't log passwords, tokens or full card numbers; redact them in your logger.

## The supply chain

A typical Node.js app has hundreds of transitive dependencies; each is code you run with full permissions. Real incidents include hijacked maintainer accounts publishing malware (event-stream, ua-parser-js, the 2025 "Shai-Hulud" worm) and **typosquatting** (`crossenv` instead of `cross-env`).

- Commit `package-lock.json`; install with `npm ci` in CI.
- Run `npm audit` and enable Dependabot/Renovate; update regularly but review what changed.
- Prefer fewer, well-maintained dependencies; the standard library now covers a lot (fetch, test runner, `util.parseArgs`, `crypto.randomUUID`, `structuredClone`).
- Disable install scripts where possible (`npm config set ignore-scripts true`, or pnpm's default behaviour) and use npm's provenance and trusted publishing for your own packages.
- Node's **permission model** (`node --permission --allow-fs-read=./data app.js`) restricts what a process can touch.

@@quiz
Which is safe for displaying a user's comment in the DOM?
- `el.innerHTML = comment`
* `el.textContent = comment`
- `document.write(comment)`
- `el.outerHTML = comment`
? textContent never parses HTML.

@@quiz
What can `{"__proto__": {"isAdmin": true}}` do when deep-merged naively?
* Add `isAdmin` to Object.prototype, so every object in the process appears to have it
- Nothing; JSON can't contain __proto__
- Crash JSON.parse
- Only affect that one object
? Skip __proto__, constructor and prototype keys when merging.

@@quiz
Why is `/^(a+)+$/` dangerous on a server?
- It matches too much
* Nested quantifiers cause exponential backtracking, freezing the event loop on crafted input
- It's a syntax error
- It leaks memory
? Avoid nested quantifiers and limit input length.

@@quiz
Is Node's `vm` module a safe sandbox for untrusted code?
- Yes
* No; code can escape it, so never run untrusted code with it
- Only with strict mode
- Only on Linux
? Use separate processes, containers or isolates designed for it.

@@drill
Which property name is the classic way to pollute every object's prototype?
= __proto__
? Reject it when merging untrusted JSON.

@@flashcards
XSS :: injected script running in users' browsers
`textContent` :: sets text safely, no HTML parsing
DOMPurify :: sanitises HTML before rendering
CSP :: Content-Security-Policy header limiting script sources
Prototype pollution :: modifying Object.prototype via crafted keys
ReDoS :: regex with catastrophic backtracking used for denial of service
Typosquatting :: malicious package with a name like a popular one
`npm ci` :: clean install exactly from the lockfile
Permission model :: `node --permission` restricts file/network access

@@exercise A safe deep merge
Write `safeMerge(target, ...sources)` that deep-merges plain objects from untrusted input:

- nested plain objects merge recursively; arrays and other values replace
- keys `__proto__`, `constructor` and `prototype` are **ignored** at every level
- only own enumerable keys are copied
- returns `target`

The tests check that `Object.prototype` is never modified.
@@starter
function safeMerge(target, ...sources) {
  throw new Error("TODO: not done yet");
}

@@tests
const result = safeMerge({ db: { host: "a", port: 1 }, tags: ["x"] }, { db: { port: 2 }, tags: ["y"] }, { debug: true });
check(result, { db: { host: "a", port: 2 }, tags: ["y"], debug: true }, "merges");
safeMerge({}, JSON.parse('{"__proto__": {"polluted": 1}, "a": {"__proto__": {"polluted": 2}}}'));
safeMerge({}, JSON.parse('{"constructor": {"prototype": {"polluted": 3}}}'));
check(({}).polluted, undefined, "Object.prototype untouched");
check(Object.keys(safeMerge({}, JSON.parse('{"__proto__": {"x": 1}, "ok": 1}'))), ["ok"], "dangerous keys skipped");
@@hint
`const isPlain = (v) => typeof v === "object" && v !== null && !Array.isArray(v);` Loop `for (const key of Object.keys(source))`, `continue` on the three forbidden keys, and if both `target[key]` (checked with `Object.hasOwn`) and `source[key]` are plain objects, recurse; otherwise assign.
@@solution
const FORBIDDEN = new Set(["__proto__", "constructor", "prototype"]);
const isPlain = (v) => typeof v === "object" && v !== null && !Array.isArray(v);

function safeMerge(target, ...sources) {
  for (const source of sources) {
    for (const key of Object.keys(source)) {
      if (FORBIDDEN.has(key)) continue;
      const value = source[key];
      if (isPlain(value)) {
        const existing = Object.hasOwn(target, key) && isPlain(target[key]) ? target[key] : {};
        target[key] = safeMerge(existing, value);
      } else {
        target[key] = value;
      }
    }
  }
  return target;
}

@@exercise Contain user paths and links
Write:

- `resolveInside(base, userPath)`: join with `node:path` (`path.posix` so results are the same on every OS) and return the normalised absolute path if it is `base` itself or inside it; otherwise throw `Error("path escapes base")`. Reject paths containing a NUL character (`\0`) with the same error. `base` is an absolute POSIX path.
- `safeHref(url)`: return the URL string (normalised by `new URL`) if it parses and its protocol is `http:`, `https:` or `mailto:`; otherwise return `"#"`.
- `escapeHtml(text)`: replace `& < > " '` with `&amp; &lt; &gt; &quot; &#39;`.
@@starter
const path = require("node:path");

function resolveInside(base, userPath) {
  throw new Error("TODO: not done yet");
}

function safeHref(url) {
  throw new Error("TODO: not done yet");
}

function escapeHtml(text) {
  throw new Error("TODO: not done yet");
}

@@tests
check(resolveInside("/srv/files", "reports/q3.pdf"), "/srv/files/reports/q3.pdf", "inside");
check(resolveInside("/srv/files", "a/../b.txt"), "/srv/files/b.txt", "normalised");
const bad = ["../etc/passwd", "/etc/passwd", "a/../../x", "x\0.txt", "../files2/secret"];
check(bad.map((p) => { try { return resolveInside("/srv/files", p); } catch (e) { return e.message; } }), bad.map(() => "path escapes base"), "escapes refused");
check(["https://example.com/a b", "javascript:alert(1)", "JaVaScRiPt:alert(1)", "mailto:ops@corp.local", "data:text/html,x", "not a url"].map(safeHref), ["https://example.com/a%20b", "#", "#", "mailto:ops@corp.local", "#", "#"], "links");
check(escapeHtml(`<img src=x onerror="alert('x')"> & co`), "&lt;img src=x onerror=&quot;alert(&#39;x&#39;)&quot;&gt; &amp; co", "escape");
@@hint
`const full = path.posix.resolve(base, userPath);` then check `full === base || full.startsWith(base + "/")` (the `+ "/"` matters: `/srv/files2` must not count as inside `/srv/files`). For links, `try { const u = new URL(url); ... } catch { return "#"; }`. Escape `&` first.
@@solution
const path = require("node:path");

function resolveInside(base, userPath) {
  if (userPath.includes("\0")) throw new Error("path escapes base");
  const root = path.posix.resolve(base);
  const full = path.posix.resolve(root, userPath);
  if (full !== root && !full.startsWith(root + "/")) throw new Error("path escapes base");
  return full;
}

function safeHref(url) {
  try {
    const u = new URL(url);
    return ["http:", "https:", "mailto:"].includes(u.protocol) ? u.href : "#";
  } catch {
    return "#";
  }
}

function escapeHtml(text) {
  return text
    .replaceAll("&", "&amp;")
    .replaceAll("<", "&lt;")
    .replaceAll(">", "&gt;")
    .replaceAll('"', "&quot;")
    .replaceAll("'", "&#39;");
}
