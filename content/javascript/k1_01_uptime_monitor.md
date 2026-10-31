@@lesson k1-uptime-monitor Uptime Monitor
@@text
"Is the website up?" Every company wants to know before the customers do. Services like UptimeRobot or Pingdom do it for money; in this build you'll write your own **uptime monitor** in Node.js:

```text
$ node main.js
Checking 4 sites...
UP     312 ms  Example                 https://example.com
SLOW  1840 ms  Status page             https://www.githubstatus.com
DOWN     - ms  Broken on purpose       https://nonexistent.invalid   (fetch failed)
ALERT  DOWN: Broken on purpose
Uptime since first run: Example 100.0%, Status page 100.0%, Broken on purpose 0.0%
Wrote status.html
```

It checks every site **in parallel** with a timeout, classifies each as `up`, `slow` or `down`, keeps a **history** in `status.json`, raises **alerts** when a site changes state, and writes a public **status page**. `node main.js --watch 60` keeps checking every minute.

## The design

`lib.js` has the logic as small functions; `main.js` does the input and output (files, console). `lib.test.js` tests every function with `node:test`, including `checkSite`, which it points at a tiny local web server the test starts itself, so the tests never depend on the internet.

| Function | Job |
|---|---|
| `checkSite(site, timeoutMs)` | fetch the URL; return `{ name, ok, code, ms, error }` |
| `classify(result, slowMs)` | `"down"` if not ok, `"slow"` if slower than `slowMs`, else `"up"` |
| `addToHistory(history, results, time, max)` | a **new** history with each result appended, newest `max` kept |
| `uptimePercent(entries)` | share of entries that aren't down, 1 decimal |
| `alerts(history, results)` | `"DOWN: name"` / `"UP: name"` when the state changed |
| `renderPage(sites, history, time)` | the HTML status page (escape the names!) |

@@project uptime_monitor Uptime Monitor
Write the functions in `lib.js` until **🧪 run tests** passes, then run **▶ node main.js**. Edit `sites.json` to watch your own sites.
@@file lib.js
// Uptime monitor logic. Only checkSite touches the network; everything else is pure.

/**
 * Fetch site.url and measure how long it takes.
 * Returns { name, ok, code, ms, error }:
 * - ok: true when the response status is 200-399
 * - code: the HTTP status, or null if there was no response
 * - ms: whole milliseconds the request took
 * - error: null, "HTTP <code>", or the error message (timeouts included)
 * Give up after timeoutMs (tip: fetch(url, { signal: AbortSignal.timeout(timeoutMs) })).
 */
async function checkSite(site, timeoutMs = 5000) {
  throw new Error("TODO: not done yet");
}

/** "down" when !result.ok, "slow" when result.ms > slowMs, otherwise "up". */
function classify(result, slowMs = 1000) {
  throw new Error("TODO: not done yet");
}

/**
 * history: { [siteName]: [{ time, state, ms }, ...] } (oldest first).
 * results: [{ name, state, ms }]. Returns a NEW history object (don't change the input)
 * with one entry per result appended, keeping only the newest `max` entries per site.
 */
function addToHistory(history, results, time, max = 100) {
  throw new Error("TODO: not done yet");
}

/** Percentage (one decimal, as a number) of entries whose state isn't "down"; null for none. */
function uptimePercent(entries) {
  throw new Error("TODO: not done yet");
}

/**
 * Compare each result's state with the site's LAST state in history.
 * "DOWN: <name>" when it was up or slow and is now down, or when a new site starts out down.
 * "UP: <name>" when it was down and now isn't. Nothing otherwise. Same order as results.
 */
function alerts(history, results) {
  throw new Error("TODO: not done yet");
}

function escapeHtml(text) {
  const map = { "&": "&amp;", "<": "&lt;", ">": "&gt;", '"': "&quot;", "'": "&#39;" };
  return String(text).replace(/[&<>"']/g, (c) => map[c]);
}

/** A status page: one table row per site with its name, current state and uptime. */
function renderPage(sites, history, time) {
  const rows = sites
    .map((site) => {
      const entries = history[site.name] ?? [];
      const last = entries.at(-1);
      const state = last ? last.state : "unknown";
      const uptime = uptimePercent(entries);
      return `<tr class="${state}"><td>${escapeHtml(site.name)}</td><td>${state.toUpperCase()}</td><td>${uptime === null ? "-" : uptime.toFixed(1) + "%"}</td></tr>`;
    })
    .join("\n");
  return `<!DOCTYPE html>
<html lang="en"><head><meta charset="utf-8"><title>Status</title>
<meta http-equiv="refresh" content="60">
<style>
body{font-family:system-ui,sans-serif;margin:2rem;background:#f6f7f9}
table{border-collapse:collapse;background:#fff}td,th{padding:.5rem 1rem;border-bottom:1px solid #ddd;text-align:left}
.up td:nth-child(2){color:#1e8449}.slow td:nth-child(2){color:#b9770e}.down td:nth-child(2){color:#c0392b;font-weight:bold}
</style></head><body>
<h1>Service status</h1>
<table><tr><th>Service</th><th>Status</th><th>Uptime</th></tr>
${rows}
</table>
<p>Last checked ${escapeHtml(new Date(time).toISOString())}</p>
</body></html>`;
}

module.exports = { checkSite, classify, addToHistory, uptimePercent, alerts, escapeHtml, renderPage };
@@file lib.test.js
const test = require("node:test");
const assert = require("node:assert/strict");
const http = require("node:http");
const lib = require("./lib");

test("classify", () => {
  assert.equal(lib.classify({ ok: true, ms: 200 }), "up");
  assert.equal(lib.classify({ ok: true, ms: 1500 }), "slow");
  assert.equal(lib.classify({ ok: false, ms: 10 }), "down");
  assert.equal(lib.classify({ ok: true, ms: 300 }, 250), "slow");
});

test("addToHistory appends and doesn't change its input", () => {
  const before = { a: [{ time: 1, state: "up", ms: 10 }] };
  const after = lib.addToHistory(before, [{ name: "a", state: "down", ms: 0 }, { name: "b", state: "up", ms: 5 }], 2);
  assert.deepEqual(after, {
    a: [{ time: 1, state: "up", ms: 10 }, { time: 2, state: "down", ms: 0 }],
    b: [{ time: 2, state: "up", ms: 5 }],
  });
  assert.equal(before.a.length, 1);
});

test("addToHistory keeps the newest entries", () => {
  let h = {};
  for (let t = 1; t <= 5; t++) h = lib.addToHistory(h, [{ name: "a", state: "up", ms: t }], t, 3);
  assert.deepEqual(h.a.map((e) => e.time), [3, 4, 5]);
});

test("uptimePercent", () => {
  const e = (state) => ({ time: 0, state, ms: 0 });
  assert.equal(lib.uptimePercent([e("up"), e("down"), e("slow"), e("up")]), 75);
  assert.equal(lib.uptimePercent([e("up"), e("up"), e("down")]), 66.7);
  assert.equal(lib.uptimePercent([]), null);
});

test("alerts on state changes", () => {
  const history = {
    a: [{ state: "down" }, { state: "up" }],
    b: [{ state: "down" }],
    c: [{ state: "slow" }],
  };
  const results = [
    { name: "a", state: "down" },
    { name: "b", state: "up" },
    { name: "c", state: "up" },
    { name: "d", state: "down" },
    { name: "e", state: "up" },
  ];
  assert.deepEqual(lib.alerts(history, results), ["DOWN: a", "UP: b", "DOWN: d"]);
});

test("renderPage escapes names", () => {
  const html = lib.renderPage([{ name: "Web & <API>" }], { "Web & <API>": [{ time: 0, state: "down", ms: 0 }] }, 0);
  assert.match(html, /<td>Web &amp; &lt;API&gt;<\/td>/);
  assert.match(html, /class="down"/);
  assert.match(html, /0\.0%/);
});

test("checkSite against a local server", async (t) => {
  const server = http.createServer((req, res) => {
    if (req.url === "/ok") { res.writeHead(200); res.end("fine"); }
    else if (req.url === "/slow") { setTimeout(() => { res.writeHead(200); res.end("late"); }, 500); }
    else { res.writeHead(500); res.end("broken"); }
  });
  await new Promise((resolve) => server.listen(0, "127.0.0.1", resolve));
  t.after(() => { server.closeAllConnections(); server.close(); });
  const base = `http://127.0.0.1:${server.address().port}`;

  const ok = await lib.checkSite({ name: "ok", url: `${base}/ok` });
  assert.equal(ok.name, "ok");
  assert.equal(ok.ok, true);
  assert.equal(ok.code, 200);
  assert.equal(ok.error, null);
  assert.equal(typeof ok.ms, "number");

  const broken = await lib.checkSite({ name: "broken", url: `${base}/fail` });
  assert.equal(broken.ok, false);
  assert.equal(broken.code, 500);
  assert.equal(broken.error, "HTTP 500");

  const slow = await lib.checkSite({ name: "slow", url: `${base}/slow` }, 100);
  assert.equal(slow.ok, false);
  assert.equal(slow.code, null);
  assert.match(slow.error, /timeout|abort/i);
});
@@file main.js
// Usage: node main.js [--watch SECONDS]
const fs = require("node:fs");
const path = require("node:path");
const lib = require("./lib");

const SITES = path.join(__dirname, "sites.json");
const HISTORY = path.join(__dirname, "status.json");
const PAGE = path.join(__dirname, "status.html");

async function runOnce() {
  const sites = JSON.parse(fs.readFileSync(SITES, "utf8"));
  const history = fs.existsSync(HISTORY) ? JSON.parse(fs.readFileSync(HISTORY, "utf8")) : {};
  console.log(`Checking ${sites.length} sites...`);

  const checked = await Promise.all(sites.map((s) => lib.checkSite(s, 5000)));
  const results = checked.map((r) => ({ name: r.name, state: lib.classify(r, 1000), ms: r.ms }));
  checked.forEach((r, i) => {
    const state = results[i].state.toUpperCase().padEnd(5);
    const ms = r.ok ? String(r.ms).padStart(5) : "    -";
    console.log(`${state} ${ms} ms  ${r.name.padEnd(22)}  ${sites[i].url}${r.error ? `   (${r.error})` : ""}`);
  });

  for (const alert of lib.alerts(history, results)) console.log(`ALERT  ${alert}`);

  const now = Date.now();
  const updated = lib.addToHistory(history, results, now, 1000);
  fs.writeFileSync(HISTORY, JSON.stringify(updated));
  fs.writeFileSync(PAGE, lib.renderPage(sites, updated, now));
  const summary = sites.map((s) => `${s.name} ${lib.uptimePercent(updated[s.name]).toFixed(1)}%`).join(", ");
  console.log(`Uptime since first run: ${summary}`);
  console.log(`Wrote ${path.basename(PAGE)}`);
}

async function main() {
  const i = process.argv.indexOf("--watch");
  const every = i > -1 ? Number(process.argv[i + 1]) : 0;
  await runOnce();
  if (every > 0) {
    console.log(`Watching: every ${every} s (Ctrl+C to stop)`);
    setInterval(() => runOnce().catch((e) => console.error(e)), every * 1000);
  }
}

main().catch((e) => {
  console.error(e.message);
  process.exit(1);
});
@@file sites.json
[
  { "name": "Example", "url": "https://example.com" },
  { "name": "Status page", "url": "https://www.githubstatus.com" },
  { "name": "Wikipedia", "url": "https://www.wikipedia.org" },
  { "name": "Broken on purpose", "url": "https://nonexistent.invalid" }
]
@@text
> For `checkSite`: wrap the `fetch` in `try/catch`. Measure with `const start = performance.now()` and `Math.round(performance.now() - start)`. Call `await res.arrayBuffer()` to finish reading the body, so the connection is released. A timeout throws an error whose `name` is `"TimeoutError"` (its message mentions the timeout), and a DNS failure throws `"fetch failed"`.

@@project uptime_monitor_finished Uptime Monitor: the finished version
The complete project. Make it useful for real: run it every minute with cron or Task Scheduler, publish `status.html` on a web server, send alerts to a Teams or Slack **webhook** (one `fetch` with `method: "POST"` and a JSON body), or add checks that the page **contains** an expected word, which catches "the server answers 200 but shows an error page".
@@file lib.js
// Uptime monitor logic. Only checkSite touches the network; everything else is pure.

/**
 * Fetch site.url and measure how long it takes.
 * Returns { name, ok, code, ms, error }:
 * - ok: true when the response status is 200-399
 * - code: the HTTP status, or null if there was no response
 * - ms: whole milliseconds the request took
 * - error: null, "HTTP <code>", or the error message (timeouts included)
 * Give up after timeoutMs (tip: fetch(url, { signal: AbortSignal.timeout(timeoutMs) })).
 */
async function checkSite(site, timeoutMs = 5000) {
  const start = performance.now();
  try {
    const res = await fetch(site.url, { signal: AbortSignal.timeout(timeoutMs) });
    await res.arrayBuffer();
    const ms = Math.round(performance.now() - start);
    const ok = res.status >= 200 && res.status < 400;
    return { name: site.name, ok, code: res.status, ms, error: ok ? null : `HTTP ${res.status}` };
  } catch (err) {
    const ms = Math.round(performance.now() - start);
    return { name: site.name, ok: false, code: null, ms, error: err.message };
  }
}

/** "down" when !result.ok, "slow" when result.ms > slowMs, otherwise "up". */
function classify(result, slowMs = 1000) {
  if (!result.ok) return "down";
  return result.ms > slowMs ? "slow" : "up";
}

/**
 * history: { [siteName]: [{ time, state, ms }, ...] } (oldest first).
 * results: [{ name, state, ms }]. Returns a NEW history object (don't change the input)
 * with one entry per result appended, keeping only the newest `max` entries per site.
 */
function addToHistory(history, results, time, max = 100) {
  const updated = { ...history };
  for (const r of results) {
    const entries = [...(updated[r.name] ?? []), { time, state: r.state, ms: r.ms }];
    updated[r.name] = entries.slice(-max);
  }
  return updated;
}

/** Percentage (one decimal, as a number) of entries whose state isn't "down"; null for none. */
function uptimePercent(entries) {
  if (!entries || entries.length === 0) return null;
  const good = entries.filter((e) => e.state !== "down").length;
  return Math.round((good / entries.length) * 1000) / 10;
}

/**
 * Compare each result's state with the site's LAST state in history.
 * "DOWN: <name>" when it was up or slow and is now down, or when a new site starts out down.
 * "UP: <name>" when it was down and now isn't. Nothing otherwise. Same order as results.
 */
function alerts(history, results) {
  const messages = [];
  for (const r of results) {
    const last = history[r.name]?.at(-1)?.state;
    const wasDown = last === "down";
    const isDown = r.state === "down";
    if (isDown && !wasDown) messages.push(`DOWN: ${r.name}`);
    else if (!isDown && wasDown) messages.push(`UP: ${r.name}`);
  }
  return messages;
}

function escapeHtml(text) {
  const map = { "&": "&amp;", "<": "&lt;", ">": "&gt;", '"': "&quot;", "'": "&#39;" };
  return String(text).replace(/[&<>"']/g, (c) => map[c]);
}

/** A status page: one table row per site with its name, current state and uptime. */
function renderPage(sites, history, time) {
  const rows = sites
    .map((site) => {
      const entries = history[site.name] ?? [];
      const last = entries.at(-1);
      const state = last ? last.state : "unknown";
      const uptime = uptimePercent(entries);
      return `<tr class="${state}"><td>${escapeHtml(site.name)}</td><td>${state.toUpperCase()}</td><td>${uptime === null ? "-" : uptime.toFixed(1) + "%"}</td></tr>`;
    })
    .join("\n");
  return `<!DOCTYPE html>
<html lang="en"><head><meta charset="utf-8"><title>Status</title>
<meta http-equiv="refresh" content="60">
<style>
body{font-family:system-ui,sans-serif;margin:2rem;background:#f6f7f9}
table{border-collapse:collapse;background:#fff}td,th{padding:.5rem 1rem;border-bottom:1px solid #ddd;text-align:left}
.up td:nth-child(2){color:#1e8449}.slow td:nth-child(2){color:#b9770e}.down td:nth-child(2){color:#c0392b;font-weight:bold}
</style></head><body>
<h1>Service status</h1>
<table><tr><th>Service</th><th>Status</th><th>Uptime</th></tr>
${rows}
</table>
<p>Last checked ${escapeHtml(new Date(time).toISOString())}</p>
</body></html>`;
}

module.exports = { checkSite, classify, addToHistory, uptimePercent, alerts, escapeHtml, renderPage };
@@file lib.test.js
const test = require("node:test");
const assert = require("node:assert/strict");
const http = require("node:http");
const lib = require("./lib");

test("classify", () => {
  assert.equal(lib.classify({ ok: true, ms: 200 }), "up");
  assert.equal(lib.classify({ ok: true, ms: 1500 }), "slow");
  assert.equal(lib.classify({ ok: false, ms: 10 }), "down");
  assert.equal(lib.classify({ ok: true, ms: 300 }, 250), "slow");
});

test("addToHistory appends and doesn't change its input", () => {
  const before = { a: [{ time: 1, state: "up", ms: 10 }] };
  const after = lib.addToHistory(before, [{ name: "a", state: "down", ms: 0 }, { name: "b", state: "up", ms: 5 }], 2);
  assert.deepEqual(after, {
    a: [{ time: 1, state: "up", ms: 10 }, { time: 2, state: "down", ms: 0 }],
    b: [{ time: 2, state: "up", ms: 5 }],
  });
  assert.equal(before.a.length, 1);
});

test("addToHistory keeps the newest entries", () => {
  let h = {};
  for (let t = 1; t <= 5; t++) h = lib.addToHistory(h, [{ name: "a", state: "up", ms: t }], t, 3);
  assert.deepEqual(h.a.map((e) => e.time), [3, 4, 5]);
});

test("uptimePercent", () => {
  const e = (state) => ({ time: 0, state, ms: 0 });
  assert.equal(lib.uptimePercent([e("up"), e("down"), e("slow"), e("up")]), 75);
  assert.equal(lib.uptimePercent([e("up"), e("up"), e("down")]), 66.7);
  assert.equal(lib.uptimePercent([]), null);
});

test("alerts on state changes", () => {
  const history = {
    a: [{ state: "down" }, { state: "up" }],
    b: [{ state: "down" }],
    c: [{ state: "slow" }],
  };
  const results = [
    { name: "a", state: "down" },
    { name: "b", state: "up" },
    { name: "c", state: "up" },
    { name: "d", state: "down" },
    { name: "e", state: "up" },
  ];
  assert.deepEqual(lib.alerts(history, results), ["DOWN: a", "UP: b", "DOWN: d"]);
});

test("renderPage escapes names", () => {
  const html = lib.renderPage([{ name: "Web & <API>" }], { "Web & <API>": [{ time: 0, state: "down", ms: 0 }] }, 0);
  assert.match(html, /<td>Web &amp; &lt;API&gt;<\/td>/);
  assert.match(html, /class="down"/);
  assert.match(html, /0\.0%/);
});

test("checkSite against a local server", async (t) => {
  const server = http.createServer((req, res) => {
    if (req.url === "/ok") { res.writeHead(200); res.end("fine"); }
    else if (req.url === "/slow") { setTimeout(() => { res.writeHead(200); res.end("late"); }, 500); }
    else { res.writeHead(500); res.end("broken"); }
  });
  await new Promise((resolve) => server.listen(0, "127.0.0.1", resolve));
  t.after(() => { server.closeAllConnections(); server.close(); });
  const base = `http://127.0.0.1:${server.address().port}`;

  const ok = await lib.checkSite({ name: "ok", url: `${base}/ok` });
  assert.equal(ok.name, "ok");
  assert.equal(ok.ok, true);
  assert.equal(ok.code, 200);
  assert.equal(ok.error, null);
  assert.equal(typeof ok.ms, "number");

  const broken = await lib.checkSite({ name: "broken", url: `${base}/fail` });
  assert.equal(broken.ok, false);
  assert.equal(broken.code, 500);
  assert.equal(broken.error, "HTTP 500");

  const slow = await lib.checkSite({ name: "slow", url: `${base}/slow` }, 100);
  assert.equal(slow.ok, false);
  assert.equal(slow.code, null);
  assert.match(slow.error, /timeout|abort/i);
});
@@file main.js
// Usage: node main.js [--watch SECONDS]
const fs = require("node:fs");
const path = require("node:path");
const lib = require("./lib");

const SITES = path.join(__dirname, "sites.json");
const HISTORY = path.join(__dirname, "status.json");
const PAGE = path.join(__dirname, "status.html");

async function runOnce() {
  const sites = JSON.parse(fs.readFileSync(SITES, "utf8"));
  const history = fs.existsSync(HISTORY) ? JSON.parse(fs.readFileSync(HISTORY, "utf8")) : {};
  console.log(`Checking ${sites.length} sites...`);

  const checked = await Promise.all(sites.map((s) => lib.checkSite(s, 5000)));
  const results = checked.map((r) => ({ name: r.name, state: lib.classify(r, 1000), ms: r.ms }));
  checked.forEach((r, i) => {
    const state = results[i].state.toUpperCase().padEnd(5);
    const ms = r.ok ? String(r.ms).padStart(5) : "    -";
    console.log(`${state} ${ms} ms  ${r.name.padEnd(22)}  ${sites[i].url}${r.error ? `   (${r.error})` : ""}`);
  });

  for (const alert of lib.alerts(history, results)) console.log(`ALERT  ${alert}`);

  const now = Date.now();
  const updated = lib.addToHistory(history, results, now, 1000);
  fs.writeFileSync(HISTORY, JSON.stringify(updated));
  fs.writeFileSync(PAGE, lib.renderPage(sites, updated, now));
  const summary = sites.map((s) => `${s.name} ${lib.uptimePercent(updated[s.name]).toFixed(1)}%`).join(", ");
  console.log(`Uptime since first run: ${summary}`);
  console.log(`Wrote ${path.basename(PAGE)}`);
}

async function main() {
  const i = process.argv.indexOf("--watch");
  const every = i > -1 ? Number(process.argv[i + 1]) : 0;
  await runOnce();
  if (every > 0) {
    console.log(`Watching: every ${every} s (Ctrl+C to stop)`);
    setInterval(() => runOnce().catch((e) => console.error(e)), every * 1000);
  }
}

main().catch((e) => {
  console.error(e.message);
  process.exit(1);
});
@@file sites.json
[
  { "name": "Example", "url": "https://example.com" },
  { "name": "Status page", "url": "https://www.githubstatus.com" },
  { "name": "Wikipedia", "url": "https://www.wikipedia.org" },
  { "name": "Broken on purpose", "url": "https://nonexistent.invalid" }
]
@@text
🎉 You've built the core of a commercial product: parallel checks with timeouts, state-change alerting, history and a status page, all tested without touching the internet. That last part, testing network code against a local server, is a technique many professionals never learn.
