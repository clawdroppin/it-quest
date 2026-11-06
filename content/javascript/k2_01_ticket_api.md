@@lesson k2-ticket-api Help Desk Ticket API
@@text
Every IT department runs on tickets: ServiceNow, Jira Service Management, Zendesk, Freshservice. Under the dashboards they're all the same thing: a **REST API** over a database of tickets, with priorities, **SLA** deadlines and status changes. In this build you'll write one.

```text
$ node main.js
Help desk API on http://localhost:3000
  curl http://localhost:3000/api/tickets
  curl -X POST http://localhost:3000/api/tickets -H "Content-Type: application/json" -d '{"title":"VPN down","priority":1,"requester":"ana"}'
  curl -X PATCH http://localhost:3000/api/tickets/1 -H "Content-Type: application/json" -d '{"status":"resolved","note":"restarted the VPN gateway"}'
  curl http://localhost:3000/api/overdue
```

(On Windows PowerShell, `curl` is an alias for something else: use `curl.exe`, or `Invoke-RestMethod`.)

## The rules

| Priority | Name | SLA: resolve within |
|---|---|---|
| 1 | Critical | 4 hours |
| 2 | High | 8 hours |
| 3 | Normal (default) | 24 hours |
| 4 | Low | 72 hours |

Statuses: `open` → `in-progress` → `resolved` → `closed`. A ticket is **overdue** when it's `open` or `in-progress` and its `due` time has passed. ITIL calls this *incident management*, and SLA tracking is exactly what managers look at on Monday mornings.

## The files

| File | Job |
|---|---|
| `tickets.js` | the rules, as pure functions (you write these) |
| `server.js` | the HTTP API: routes, JSON in and out, status codes (given) |
| `main.js` | starts the server and saves tickets to `tickets.json` |
| `tickets.test.js` | tests for the rules |
| `api.test.js` | tests that start the real server on a free port and call it with `fetch` |

@@project ticket_api Help Desk Ticket API
Write the functions in `tickets.js` until **🧪 run tests** passes (both test files), then start it with **▶ node main.js** and try the `curl` commands.
@@file tickets.js
// Help desk rules: pure functions. Times are ISO strings like "2026-10-31T09:00:00.000Z".

const SLA_HOURS = { 1: 4, 2: 8, 3: 24, 4: 72 };
const STATUSES = ["open", "in-progress", "resolved", "closed"];

/** Error messages for a new ticket's body (empty = valid):
 * "title is required" (missing or blank), "title is too long" (over 100 chars),
 * "priority must be 1-4" (when given and not a whole number 1-4). */
function validateTicket(body) {
  throw new Error("TODO: not done yet");
}

/** The ISO time `hours` hours after the ISO time `iso`. */
function addHours(iso, hours) {
  return new Date(new Date(iso).getTime() + hours * 3600 * 1000).toISOString();
}

/** A new ticket: { id, title (trimmed), requester (default "unknown"), priority (default 3),
 * status "open", created (= now), due (now + SLA hours), notes [] }. */
function createTicket(body, id, now) {
  throw new Error("TODO: not done yet");
}

/** A NEW ticket with the patch applied (don't change the original). The patch may have:
 * - status: one of STATUSES, else throw Error("invalid status")
 * - priority: 1-4, else throw Error("priority must be 1-4"); the due time is recalculated from `created`
 * - note: text added to notes as { time: now, text }
 * Other fields are ignored. */
function applyUpdate(ticket, patch, now) {
  throw new Error("TODO: not done yet");
}

/** Tickets matching the query (all fields optional): status, priority (a number or numeric string),
 * q (text that must appear in the title, any case). Sorted by priority, then by created time. */
function filterTickets(tickets, query) {
  throw new Error("TODO: not done yet");
}

/** Open or in-progress tickets whose due time is before now, most overdue first. */
function overdue(tickets, now) {
  throw new Error("TODO: not done yet");
}

module.exports = { SLA_HOURS, STATUSES, validateTicket, addHours, createTicket, applyUpdate, filterTickets, overdue };
@@file server.js
// The HTTP API. createApp(db, onChange) returns an http.Server; db is { tickets: [], nextId: 1 }.
const http = require("node:http");
const t = require("./tickets");

function send(res, status, data) {
  res.writeHead(status, { "Content-Type": "application/json" });
  res.end(data === undefined ? "" : JSON.stringify(data));
}

async function readJson(req) {
  let body = "";
  for await (const chunk of req) {
    body += chunk;
    if (body.length > 100_000) throw new Error("body too large");
  }
  return body ? JSON.parse(body) : {};
}

function createApp(db, onChange = () => {}) {
  return http.createServer(async (req, res) => {
    const url = new URL(req.url, "http://localhost");
    const now = new Date().toISOString();
    try {
      if (url.pathname === "/api/tickets" && req.method === "GET") {
        return send(res, 200, t.filterTickets(db.tickets, Object.fromEntries(url.searchParams)));
      }
      if (url.pathname === "/api/tickets" && req.method === "POST") {
        let body;
        try { body = await readJson(req); } catch { return send(res, 400, { errors: ["body must be JSON"] }); }
        const errors = t.validateTicket(body);
        if (errors.length) return send(res, 400, { errors });
        const ticket = t.createTicket(body, db.nextId++, now);
        db.tickets.push(ticket);
        onChange(db);
        return send(res, 201, ticket);
      }
      if (url.pathname === "/api/overdue" && req.method === "GET") {
        return send(res, 200, t.overdue(db.tickets, now));
      }
      const m = url.pathname.match(/^\/api\/tickets\/(\d+)$/);
      if (m) {
        const index = db.tickets.findIndex((x) => x.id === Number(m[1]));
        if (index === -1) return send(res, 404, { error: "no such ticket" });
        if (req.method === "GET") return send(res, 200, db.tickets[index]);
        if (req.method === "PATCH") {
          let patch;
          try { patch = await readJson(req); } catch { return send(res, 400, { errors: ["body must be JSON"] }); }
          try {
            db.tickets[index] = t.applyUpdate(db.tickets[index], patch, now);
          } catch (err) {
            return send(res, 400, { errors: [err.message] });
          }
          onChange(db);
          return send(res, 200, db.tickets[index]);
        }
      }
      send(res, 404, { error: "not found" });
    } catch (err) {
      console.error(err);
      send(res, 500, { error: "internal error" });
    }
  });
}

module.exports = { createApp };
@@file main.js
// Starts the API and keeps the tickets in tickets.json.
const fs = require("node:fs");
const path = require("node:path");
const { createApp } = require("./server");

const FILE = path.join(__dirname, "tickets.json");
const PORT = Number(process.env.PORT) || 3000;

const db = fs.existsSync(FILE) ? JSON.parse(fs.readFileSync(FILE, "utf8")) : { tickets: [], nextId: 1 };
const save = (data) => {
  fs.writeFileSync(FILE + ".tmp", JSON.stringify(data, null, 2));
  fs.renameSync(FILE + ".tmp", FILE); // write-then-rename: a crash never leaves half a file
};

createApp(db, save).listen(PORT, () => {
  const base = `http://localhost:${PORT}`;
  console.log(`Help desk API on ${base}  (${db.tickets.length} tickets loaded, Ctrl+C to stop)`);
  console.log(`  curl ${base}/api/tickets`);
  console.log(`  curl -X POST ${base}/api/tickets -H "Content-Type: application/json" -d '{"title":"VPN down","priority":1,"requester":"ana"}'`);
  console.log(`  curl -X PATCH ${base}/api/tickets/1 -H "Content-Type: application/json" -d '{"status":"resolved","note":"restarted the gateway"}'`);
  console.log(`  curl ${base}/api/overdue`);
});
@@file tickets.test.js
const test = require("node:test");
const assert = require("node:assert/strict");
const t = require("./tickets");

const NOW = "2026-10-31T09:00:00.000Z";

test("validateTicket", () => {
  assert.deepEqual(t.validateTicket({ title: "VPN down", priority: 1 }), []);
  assert.deepEqual(t.validateTicket({ title: "No priority" }), []);
  assert.deepEqual(t.validateTicket({ title: "  " }), ["title is required"]);
  assert.deepEqual(t.validateTicket({ priority: 0 }), ["title is required", "priority must be 1-4"]);
  assert.deepEqual(t.validateTicket({ title: "x".repeat(101), priority: "2" }), ["title is too long", "priority must be 1-4"]);
});

test("createTicket fills in defaults and the SLA", () => {
  assert.deepEqual(t.createTicket({ title: "  Printer jam ", requester: "ben" }, 7, NOW), {
    id: 7, title: "Printer jam", requester: "ben", priority: 3, status: "open",
    created: NOW, due: "2026-11-01T09:00:00.000Z", notes: [],
  });
  const p1 = t.createTicket({ title: "Site down", priority: 1 }, 8, NOW);
  assert.equal(p1.due, "2026-10-31T13:00:00.000Z");
  assert.equal(p1.requester, "unknown");
});

test("applyUpdate", () => {
  const ticket = t.createTicket({ title: "VPN down", priority: 3 }, 1, NOW);
  const later = "2026-10-31T10:00:00.000Z";
  const updated = t.applyUpdate(ticket, { status: "in-progress", priority: 1, note: "checking gateway", owner: "x" }, later);
  assert.equal(updated.status, "in-progress");
  assert.equal(updated.priority, 1);
  assert.equal(updated.due, "2026-10-31T13:00:00.000Z", "due is recalculated from created");
  assert.deepEqual(updated.notes, [{ time: later, text: "checking gateway" }]);
  assert.equal(updated.owner, undefined, "unknown fields are ignored");
  assert.equal(ticket.status, "open", "the original is unchanged");
  assert.deepEqual(ticket.notes, []);
  assert.throws(() => t.applyUpdate(ticket, { status: "done" }, later), /invalid status/);
  assert.throws(() => t.applyUpdate(ticket, { priority: 5 }, later), /priority must be 1-4/);
});

test("filterTickets", () => {
  const list = [
    t.createTicket({ title: "Printer jam", priority: 3 }, 1, "2026-10-31T08:00:00.000Z"),
    t.createTicket({ title: "VPN down", priority: 1 }, 2, "2026-10-31T09:00:00.000Z"),
    t.createTicket({ title: "New laptop", priority: 4 }, 3, "2026-10-31T07:00:00.000Z"),
    t.createTicket({ title: "VPN slow", priority: 1 }, 4, "2026-10-31T06:00:00.000Z"),
  ];
  list[0] = t.applyUpdate(list[0], { status: "resolved" }, NOW);
  assert.deepEqual(t.filterTickets(list, {}).map((x) => x.id), [4, 2, 1, 3]);
  assert.deepEqual(t.filterTickets(list, { status: "open" }).map((x) => x.id), [4, 2, 3]);
  assert.deepEqual(t.filterTickets(list, { priority: "1" }).map((x) => x.id), [4, 2]);
  assert.deepEqual(t.filterTickets(list, { q: "vpn" }).map((x) => x.id), [4, 2]);
});

test("overdue", () => {
  const list = [
    t.createTicket({ title: "a", priority: 1 }, 1, "2026-10-31T00:00:00.000Z"),   // due 04:00
    t.createTicket({ title: "b", priority: 1 }, 2, "2026-10-31T02:00:00.000Z"),   // due 06:00
    t.createTicket({ title: "c", priority: 3 }, 3, "2026-10-31T00:00:00.000Z"),   // due tomorrow
    t.createTicket({ title: "d", priority: 1 }, 4, "2026-10-30T00:00:00.000Z"),   // due yesterday, but resolved
  ];
  list[3] = t.applyUpdate(list[3], { status: "resolved" }, NOW);
  assert.deepEqual(t.overdue(list, NOW).map((x) => x.id), [1, 2]);
});
@@file api.test.js
const test = require("node:test");
const assert = require("node:assert/strict");
const { createApp } = require("./server");

test("the API end to end", async (t) => {
  const db = { tickets: [], nextId: 1 };
  let saves = 0;
  const server = createApp(db, () => saves++);
  await new Promise((resolve) => server.listen(0, "127.0.0.1", resolve));
  t.after(() => { server.closeAllConnections(); server.close(); });
  const base = `http://127.0.0.1:${server.address().port}`;
  const call = async (method, path, body) => {
    const res = await fetch(base + path, {
      method,
      headers: { "Content-Type": "application/json" },
      body: body === undefined ? undefined : JSON.stringify(body),
    });
    return { status: res.status, data: await res.json() };
  };

  let r = await call("POST", "/api/tickets", { title: "VPN down", priority: 1, requester: "ana" });
  assert.equal(r.status, 201);
  assert.equal(r.data.id, 1);
  assert.equal(r.data.status, "open");

  r = await call("POST", "/api/tickets", { priority: 9 });
  assert.equal(r.status, 400);
  assert.deepEqual(r.data.errors, ["title is required", "priority must be 1-4"]);

  r = await call("PATCH", "/api/tickets/1", { status: "resolved", note: "rebooted" });
  assert.equal(r.status, 200);
  assert.equal(r.data.status, "resolved");
  assert.equal(r.data.notes[0].text, "rebooted");

  r = await call("PATCH", "/api/tickets/1", { status: "finished" });
  assert.equal(r.status, 400);

  r = await call("GET", "/api/tickets/99");
  assert.equal(r.status, 404);

  r = await call("GET", "/api/tickets?status=resolved");
  assert.deepEqual(r.data.map((x) => x.id), [1]);

  assert.equal(saves, 2, "every successful change is saved");
});
@@text
> For the times, `addHours(created, SLA_HOURS[priority])` is given. ISO strings in the same format compare correctly as text, so `a.due < now` works, and so does `a.created.localeCompare(b.created)` for sorting. For `applyUpdate`, start from a copy: `const next = { ...ticket, notes: [...ticket.notes] };`.

@@project ticket_api_finished Help Desk Ticket API: the finished version
The complete project. Next steps a real help desk would want: **authentication** (an `Authorization: Bearer <token>` header checked in `createApp`), an **assignee** field with a `GET /api/tickets?assignee=me` filter, an **audit log** of every change, a small web page that lists tickets using the DOM lesson's techniques, or swapping the JSON file for SQLite (Node 22+ has `node:sqlite` built in) once you've done the SQL track.
@@file tickets.js
// Help desk rules: pure functions. Times are ISO strings like "2026-10-31T09:00:00.000Z".

const SLA_HOURS = { 1: 4, 2: 8, 3: 24, 4: 72 };
const STATUSES = ["open", "in-progress", "resolved", "closed"];

const isPriority = (p) => Number.isInteger(p) && p >= 1 && p <= 4;

/** Error messages for a new ticket's body (empty = valid):
 * "title is required" (missing or blank), "title is too long" (over 100 chars),
 * "priority must be 1-4" (when given and not a whole number 1-4). */
function validateTicket(body) {
  const errors = [];
  const title = typeof body.title === "string" ? body.title.trim() : "";
  if (!title) errors.push("title is required");
  else if (title.length > 100) errors.push("title is too long");
  if (body.priority !== undefined && !isPriority(body.priority)) errors.push("priority must be 1-4");
  return errors;
}

/** The ISO time `hours` hours after the ISO time `iso`. */
function addHours(iso, hours) {
  return new Date(new Date(iso).getTime() + hours * 3600 * 1000).toISOString();
}

/** A new ticket: { id, title (trimmed), requester (default "unknown"), priority (default 3),
 * status "open", created (= now), due (now + SLA hours), notes [] }. */
function createTicket(body, id, now) {
  const priority = body.priority ?? 3;
  return {
    id,
    title: body.title.trim(),
    requester: body.requester ?? "unknown",
    priority,
    status: "open",
    created: now,
    due: addHours(now, SLA_HOURS[priority]),
    notes: [],
  };
}

/** A NEW ticket with the patch applied (don't change the original). The patch may have:
 * - status: one of STATUSES, else throw Error("invalid status")
 * - priority: 1-4, else throw Error("priority must be 1-4"); the due time is recalculated from `created`
 * - note: text added to notes as { time: now, text }
 * Other fields are ignored. */
function applyUpdate(ticket, patch, now) {
  const next = { ...ticket, notes: [...ticket.notes] };
  if (patch.status !== undefined) {
    if (!STATUSES.includes(patch.status)) throw new Error("invalid status");
    next.status = patch.status;
  }
  if (patch.priority !== undefined) {
    if (!isPriority(patch.priority)) throw new Error("priority must be 1-4");
    next.priority = patch.priority;
    next.due = addHours(next.created, SLA_HOURS[patch.priority]);
  }
  if (patch.note) next.notes.push({ time: now, text: String(patch.note) });
  return next;
}

/** Tickets matching the query (all fields optional): status, priority (a number or numeric string),
 * q (text that must appear in the title, any case). Sorted by priority, then by created time. */
function filterTickets(tickets, query) {
  const q = query.q?.toLowerCase();
  return tickets
    .filter((x) => query.status === undefined || x.status === query.status)
    .filter((x) => query.priority === undefined || x.priority === Number(query.priority))
    .filter((x) => !q || x.title.toLowerCase().includes(q))
    .sort((a, b) => a.priority - b.priority || a.created.localeCompare(b.created));
}

/** Open or in-progress tickets whose due time is before now, most overdue first. */
function overdue(tickets, now) {
  return tickets
    .filter((x) => (x.status === "open" || x.status === "in-progress") && x.due < now)
    .sort((a, b) => a.due.localeCompare(b.due));
}

module.exports = { SLA_HOURS, STATUSES, validateTicket, addHours, createTicket, applyUpdate, filterTickets, overdue };
@@file server.js
// The HTTP API. createApp(db, onChange) returns an http.Server; db is { tickets: [], nextId: 1 }.
const http = require("node:http");
const t = require("./tickets");

function send(res, status, data) {
  res.writeHead(status, { "Content-Type": "application/json" });
  res.end(data === undefined ? "" : JSON.stringify(data));
}

async function readJson(req) {
  let body = "";
  for await (const chunk of req) {
    body += chunk;
    if (body.length > 100_000) throw new Error("body too large");
  }
  return body ? JSON.parse(body) : {};
}

function createApp(db, onChange = () => {}) {
  return http.createServer(async (req, res) => {
    const url = new URL(req.url, "http://localhost");
    const now = new Date().toISOString();
    try {
      if (url.pathname === "/api/tickets" && req.method === "GET") {
        return send(res, 200, t.filterTickets(db.tickets, Object.fromEntries(url.searchParams)));
      }
      if (url.pathname === "/api/tickets" && req.method === "POST") {
        let body;
        try { body = await readJson(req); } catch { return send(res, 400, { errors: ["body must be JSON"] }); }
        const errors = t.validateTicket(body);
        if (errors.length) return send(res, 400, { errors });
        const ticket = t.createTicket(body, db.nextId++, now);
        db.tickets.push(ticket);
        onChange(db);
        return send(res, 201, ticket);
      }
      if (url.pathname === "/api/overdue" && req.method === "GET") {
        return send(res, 200, t.overdue(db.tickets, now));
      }
      const m = url.pathname.match(/^\/api\/tickets\/(\d+)$/);
      if (m) {
        const index = db.tickets.findIndex((x) => x.id === Number(m[1]));
        if (index === -1) return send(res, 404, { error: "no such ticket" });
        if (req.method === "GET") return send(res, 200, db.tickets[index]);
        if (req.method === "PATCH") {
          let patch;
          try { patch = await readJson(req); } catch { return send(res, 400, { errors: ["body must be JSON"] }); }
          try {
            db.tickets[index] = t.applyUpdate(db.tickets[index], patch, now);
          } catch (err) {
            return send(res, 400, { errors: [err.message] });
          }
          onChange(db);
          return send(res, 200, db.tickets[index]);
        }
      }
      send(res, 404, { error: "not found" });
    } catch (err) {
      console.error(err);
      send(res, 500, { error: "internal error" });
    }
  });
}

module.exports = { createApp };
@@file main.js
// Starts the API and keeps the tickets in tickets.json.
const fs = require("node:fs");
const path = require("node:path");
const { createApp } = require("./server");

const FILE = path.join(__dirname, "tickets.json");
const PORT = Number(process.env.PORT) || 3000;

const db = fs.existsSync(FILE) ? JSON.parse(fs.readFileSync(FILE, "utf8")) : { tickets: [], nextId: 1 };
const save = (data) => {
  fs.writeFileSync(FILE + ".tmp", JSON.stringify(data, null, 2));
  fs.renameSync(FILE + ".tmp", FILE); // write-then-rename: a crash never leaves half a file
};

createApp(db, save).listen(PORT, () => {
  const base = `http://localhost:${PORT}`;
  console.log(`Help desk API on ${base}  (${db.tickets.length} tickets loaded, Ctrl+C to stop)`);
  console.log(`  curl ${base}/api/tickets`);
  console.log(`  curl -X POST ${base}/api/tickets -H "Content-Type: application/json" -d '{"title":"VPN down","priority":1,"requester":"ana"}'`);
  console.log(`  curl -X PATCH ${base}/api/tickets/1 -H "Content-Type: application/json" -d '{"status":"resolved","note":"restarted the gateway"}'`);
  console.log(`  curl ${base}/api/overdue`);
});
@@file tickets.test.js
const test = require("node:test");
const assert = require("node:assert/strict");
const t = require("./tickets");

const NOW = "2026-10-31T09:00:00.000Z";

test("validateTicket", () => {
  assert.deepEqual(t.validateTicket({ title: "VPN down", priority: 1 }), []);
  assert.deepEqual(t.validateTicket({ title: "No priority" }), []);
  assert.deepEqual(t.validateTicket({ title: "  " }), ["title is required"]);
  assert.deepEqual(t.validateTicket({ priority: 0 }), ["title is required", "priority must be 1-4"]);
  assert.deepEqual(t.validateTicket({ title: "x".repeat(101), priority: "2" }), ["title is too long", "priority must be 1-4"]);
});

test("createTicket fills in defaults and the SLA", () => {
  assert.deepEqual(t.createTicket({ title: "  Printer jam ", requester: "ben" }, 7, NOW), {
    id: 7, title: "Printer jam", requester: "ben", priority: 3, status: "open",
    created: NOW, due: "2026-11-01T09:00:00.000Z", notes: [],
  });
  const p1 = t.createTicket({ title: "Site down", priority: 1 }, 8, NOW);
  assert.equal(p1.due, "2026-10-31T13:00:00.000Z");
  assert.equal(p1.requester, "unknown");
});

test("applyUpdate", () => {
  const ticket = t.createTicket({ title: "VPN down", priority: 3 }, 1, NOW);
  const later = "2026-10-31T10:00:00.000Z";
  const updated = t.applyUpdate(ticket, { status: "in-progress", priority: 1, note: "checking gateway", owner: "x" }, later);
  assert.equal(updated.status, "in-progress");
  assert.equal(updated.priority, 1);
  assert.equal(updated.due, "2026-10-31T13:00:00.000Z", "due is recalculated from created");
  assert.deepEqual(updated.notes, [{ time: later, text: "checking gateway" }]);
  assert.equal(updated.owner, undefined, "unknown fields are ignored");
  assert.equal(ticket.status, "open", "the original is unchanged");
  assert.deepEqual(ticket.notes, []);
  assert.throws(() => t.applyUpdate(ticket, { status: "done" }, later), /invalid status/);
  assert.throws(() => t.applyUpdate(ticket, { priority: 5 }, later), /priority must be 1-4/);
});

test("filterTickets", () => {
  const list = [
    t.createTicket({ title: "Printer jam", priority: 3 }, 1, "2026-10-31T08:00:00.000Z"),
    t.createTicket({ title: "VPN down", priority: 1 }, 2, "2026-10-31T09:00:00.000Z"),
    t.createTicket({ title: "New laptop", priority: 4 }, 3, "2026-10-31T07:00:00.000Z"),
    t.createTicket({ title: "VPN slow", priority: 1 }, 4, "2026-10-31T06:00:00.000Z"),
  ];
  list[0] = t.applyUpdate(list[0], { status: "resolved" }, NOW);
  assert.deepEqual(t.filterTickets(list, {}).map((x) => x.id), [4, 2, 1, 3]);
  assert.deepEqual(t.filterTickets(list, { status: "open" }).map((x) => x.id), [4, 2, 3]);
  assert.deepEqual(t.filterTickets(list, { priority: "1" }).map((x) => x.id), [4, 2]);
  assert.deepEqual(t.filterTickets(list, { q: "vpn" }).map((x) => x.id), [4, 2]);
});

test("overdue", () => {
  const list = [
    t.createTicket({ title: "a", priority: 1 }, 1, "2026-10-31T00:00:00.000Z"),   // due 04:00
    t.createTicket({ title: "b", priority: 1 }, 2, "2026-10-31T02:00:00.000Z"),   // due 06:00
    t.createTicket({ title: "c", priority: 3 }, 3, "2026-10-31T00:00:00.000Z"),   // due tomorrow
    t.createTicket({ title: "d", priority: 1 }, 4, "2026-10-30T00:00:00.000Z"),   // due yesterday, but resolved
  ];
  list[3] = t.applyUpdate(list[3], { status: "resolved" }, NOW);
  assert.deepEqual(t.overdue(list, NOW).map((x) => x.id), [1, 2]);
});
@@file api.test.js
const test = require("node:test");
const assert = require("node:assert/strict");
const { createApp } = require("./server");

test("the API end to end", async (t) => {
  const db = { tickets: [], nextId: 1 };
  let saves = 0;
  const server = createApp(db, () => saves++);
  await new Promise((resolve) => server.listen(0, "127.0.0.1", resolve));
  t.after(() => { server.closeAllConnections(); server.close(); });
  const base = `http://127.0.0.1:${server.address().port}`;
  const call = async (method, path, body) => {
    const res = await fetch(base + path, {
      method,
      headers: { "Content-Type": "application/json" },
      body: body === undefined ? undefined : JSON.stringify(body),
    });
    return { status: res.status, data: await res.json() };
  };

  let r = await call("POST", "/api/tickets", { title: "VPN down", priority: 1, requester: "ana" });
  assert.equal(r.status, 201);
  assert.equal(r.data.id, 1);
  assert.equal(r.data.status, "open");

  r = await call("POST", "/api/tickets", { priority: 9 });
  assert.equal(r.status, 400);
  assert.deepEqual(r.data.errors, ["title is required", "priority must be 1-4"]);

  r = await call("PATCH", "/api/tickets/1", { status: "resolved", note: "rebooted" });
  assert.equal(r.status, 200);
  assert.equal(r.data.status, "resolved");
  assert.equal(r.data.notes[0].text, "rebooted");

  r = await call("PATCH", "/api/tickets/1", { status: "finished" });
  assert.equal(r.status, 400);

  r = await call("GET", "/api/tickets/99");
  assert.equal(r.status, 404);

  r = await call("GET", "/api/tickets?status=resolved");
  assert.deepEqual(r.data.map((x) => x.id), [1]);

  assert.equal(saves, 2, "every successful change is saved");
});

@@drill
Which HTTP status code means the request body failed validation?
= 400
= 422
? 400 Bad Request, or 422 Unprocessable Content.

@@text
🎉 You've built the core of an ITSM tool: validation, SLAs, state changes, filtering and a tested REST API. When an interviewer asks "have you worked with APIs?", you can say you've *built* one, and explain what each status code you return means.
