@@lesson t7-project-incidents Project: A Typed Incident Service
@@concepts objects, errors, json
@@text
This project combines the module's techniques in one realistic backend core: an incident service like the ones behind PagerDuty or Opsgenie.

- **Commands** are a discriminated union: every possible action has its own shape.
- The service returns `Result<Incident, ServiceError>`, where `ServiceError` is itself a union with a `kind`.
- **Dependencies are injected** (a clock and an id generator), so tests are deterministic.
- Incoming JSON is **parsed** into a `Command` at the boundary, collecting every problem.
- **Metrics** (MTTA: mean time to acknowledge, MTTR: mean time to resolve) are computed from the history.

## The state machine

```text
          ack            resolve
 open ──────────► acknowledged ──────► resolved
   │                                     │
   └─────────────── resolve ────────────►│
   ▲                                     │
   └────────────────── reopen ───────────┘
```

Encoding the allowed transitions as data keeps the rules in one place:

```ts
type Status = "open" | "acknowledged" | "resolved";
type CommandType = "ack" | "resolve" | "reopen";
const ALLOWED_FROM: Record<CommandType, readonly Status[]> = {
  ack: ["open"],
  resolve: ["open", "acknowledged"],
  reopen: ["resolved"],
};
console.log(ALLOWED_FROM.resolve.includes("acknowledged"));
```

`Record<CommandType, ...>` guarantees that a new command type can't be added without deciding its rules.

## Why commands as data?

A command object can be logged, queued, sent over the network, replayed in tests and validated in one place. The same union type is used by the HTTP layer, the CLI and the tests, so they can't disagree about what a command looks like.

@@quiz
What does `Record<CommandType, readonly Status[]>` guarantee for the transition table?
* Every command type has an entry, so adding a command forces you to define its rules
- Commands can't be added
- The table is sorted
- Statuses are unique
? Union keys in Record must all be present.

@@quiz
Why inject a clock and an id generator into the service?
- To make it faster
* So tests can control time and ids and get deterministic results
- Because Date is deprecated
- To support multiple time zones
? Hidden dependencies on time and randomness make tests flaky.

@@quiz
Why model incident status as a union of string literals?
* The compiler checks every status value and can verify switch statements are exhaustive
- Strings are faster than numbers
- Unions are stored in the database
- It's required by Node
? Add a status and the compiler points to every place that must handle it.

@@drill
Which type makes the compiler complain if a switch misses a case?
= never
? Assign the leftover value to a never variable.

@@flashcards
MTTA :: mean time to acknowledge
MTTR :: mean time to resolve
Command :: an action as data (a discriminated union member)
Transition table :: allowed source states per command
Deterministic tests :: injected clock and id generator
Boundary parsing :: turn JSON into a typed Command once

@@exercise The incident service
Implement `IncidentService`:

- `constructor(clock: () => number, newId: () => string)`
- `execute(cmd: Command): Result<Incident, ServiceError>`:
  - `open`: title trimmed, at least 3 characters (else `{ kind: "invalid", message: "title too short" }`); creates an incident with status `open`, `openedAt = clock()`
  - `ack`, `resolve`, `reopen`: unknown id → `not-found`; a command not allowed from the current status → `{ kind: "invalid-transition", id, from, command }` (use the table from the lesson)
  - `ack` sets `ackAt`; `resolve` sets `resolvedAt`; `reopen` sets status `open` and clears `ackAt`/`resolvedAt`
  - every successful command appends `{ at, status, by }` (the new status) to `history`
- `get(id)` and `list(filter?: { status?: Status; maxSeverity?: Severity })`: sorted by severity (1 is most severe), then `openedAt`

Return **copies** of incidents (including the history array), so callers can't change the service's state.
@@starter
type Result<T, E> = { ok: true; value: T } | { ok: false; error: E };
type Status = "open" | "acknowledged" | "resolved";
type Severity = 1 | 2 | 3 | 4;
interface Incident {
  id: string;
  title: string;
  severity: Severity;
  status: Status;
  openedAt: number;
  ackAt?: number;
  resolvedAt?: number;
  history: { at: number; status: Status; by: string }[];
}
type Command =
  | { type: "open"; title: string; severity: Severity; by: string }
  | { type: "ack"; id: string; by: string }
  | { type: "resolve"; id: string; by: string }
  | { type: "reopen"; id: string; by: string };
type ServiceError =
  | { kind: "not-found"; id: string }
  | { kind: "invalid-transition"; id: string; from: Status; command: Command["type"] }
  | { kind: "invalid"; message: string };

class IncidentService {
  constructor(clock: () => number, newId: () => string) {}

  execute(cmd: Command): Result<Incident, ServiceError> {
    throw new Error("TODO: not done yet");
  }

  get(id: string): Incident | undefined {
    throw new Error("TODO: not done yet");
  }

  list(filter: { status?: Status; maxSeverity?: Severity } = {}): Incident[] {
    throw new Error("TODO: not done yet");
  }
}

@@tests
let now = 1000;
let n = 0;
const svc = new IncidentService(() => now, () => `INC-${++n}`);
const a = svc.execute({ type: "open", title: "  DB latency high ", severity: 2, by: "monitor" });
check(a.ok ? [a.value.id, a.value.title, a.value.status] : [], ["INC-1", "DB latency high", "open"], "opened");
now = 1300;
svc.execute({ type: "open", title: "Site down", severity: 1, by: "monitor" });
now = 1600;
check(svc.execute({ type: "ack", id: "INC-1", by: "ana" }).ok, true, "ack");
check(svc.execute({ type: "ack", id: "INC-1", by: "ana" }), { ok: false, error: { kind: "invalid-transition", id: "INC-1", from: "acknowledged", command: "ack" } }, "double ack");
now = 2500;
svc.execute({ type: "resolve", id: "INC-1", by: "ana" });
check(svc.execute({ type: "resolve", id: "INC-9", by: "x" }), { ok: false, error: { kind: "not-found", id: "INC-9" } }, "not found");
check(svc.execute({ type: "open", title: "x", severity: 3, by: "bo" }), { ok: false, error: { kind: "invalid", message: "title too short" } }, "invalid");
check(svc.get("INC-1")?.history, [
  { at: 1000, status: "open", by: "monitor" },
  { at: 1600, status: "acknowledged", by: "ana" },
  { at: 2500, status: "resolved", by: "ana" },
], "history");
check(svc.list().map((i) => i.id), ["INC-2", "INC-1"], "sorted by severity");
check(svc.list({ status: "open" }).map((i) => i.id), ["INC-2"], "filter status");
now = 3000;
const re = svc.execute({ type: "reopen", id: "INC-1", by: "bo" });
check(re.ok ? [re.value.status, re.value.ackAt, re.value.resolvedAt] : [], ["open", undefined, undefined], "reopen clears times");
const snapshot = svc.get("INC-1");
snapshot?.history.push({ at: 0, status: "open", by: "hacker" });
check(svc.get("INC-1")?.history.length, 4, "returns copies");
@@hint
Keep a `Map<string, Incident>`. A `copy(i)` helper: `({ ...i, history: [...i.history] })`. For transitions, `if (!ALLOWED_FROM[cmd.type].includes(incident.status)) return { ok: false, error: {...} }`. Since `open` is handled first and returns, TypeScript narrows `cmd` to the other three types afterwards, so `cmd.id` is available. To clear optional fields use `delete incident.ackAt`.
@@solution
type Result<T, E> = { ok: true; value: T } | { ok: false; error: E };
type Status = "open" | "acknowledged" | "resolved";
type Severity = 1 | 2 | 3 | 4;
interface Incident {
  id: string;
  title: string;
  severity: Severity;
  status: Status;
  openedAt: number;
  ackAt?: number;
  resolvedAt?: number;
  history: { at: number; status: Status; by: string }[];
}
type Command =
  | { type: "open"; title: string; severity: Severity; by: string }
  | { type: "ack"; id: string; by: string }
  | { type: "resolve"; id: string; by: string }
  | { type: "reopen"; id: string; by: string };
type ServiceError =
  | { kind: "not-found"; id: string }
  | { kind: "invalid-transition"; id: string; from: Status; command: Command["type"] }
  | { kind: "invalid"; message: string };

const ALLOWED_FROM: Record<Exclude<Command["type"], "open">, readonly Status[]> = {
  ack: ["open"],
  resolve: ["open", "acknowledged"],
  reopen: ["resolved"],
};
const NEXT: Record<Exclude<Command["type"], "open">, Status> = { ack: "acknowledged", resolve: "resolved", reopen: "open" };

const copy = (i: Incident): Incident => ({ ...i, history: [...i.history] });

class IncidentService {
  private readonly clock: () => number;
  private readonly newId: () => string;
  private readonly incidents = new Map<string, Incident>();

  constructor(clock: () => number, newId: () => string) {
    this.clock = clock;
    this.newId = newId;
  }

  execute(cmd: Command): Result<Incident, ServiceError> {
    const at = this.clock();
    if (cmd.type === "open") {
      const title = cmd.title.trim();
      if (title.length < 3) return { ok: false, error: { kind: "invalid", message: "title too short" } };
      const incident: Incident = {
        id: this.newId(),
        title,
        severity: cmd.severity,
        status: "open",
        openedAt: at,
        history: [{ at, status: "open", by: cmd.by }],
      };
      this.incidents.set(incident.id, incident);
      return { ok: true, value: copy(incident) };
    }
    const incident = this.incidents.get(cmd.id);
    if (!incident) return { ok: false, error: { kind: "not-found", id: cmd.id } };
    if (!ALLOWED_FROM[cmd.type].includes(incident.status)) {
      return { ok: false, error: { kind: "invalid-transition", id: cmd.id, from: incident.status, command: cmd.type } };
    }
    incident.status = NEXT[cmd.type];
    if (cmd.type === "ack") incident.ackAt = at;
    if (cmd.type === "resolve") incident.resolvedAt = at;
    if (cmd.type === "reopen") {
      delete incident.ackAt;
      delete incident.resolvedAt;
    }
    incident.history.push({ at, status: incident.status, by: cmd.by });
    return { ok: true, value: copy(incident) };
  }

  get(id: string): Incident | undefined {
    const i = this.incidents.get(id);
    return i ? copy(i) : undefined;
  }

  list(filter: { status?: Status; maxSeverity?: Severity } = {}): Incident[] {
    return [...this.incidents.values()]
      .filter((i) => (filter.status === undefined || i.status === filter.status) && (filter.maxSeverity === undefined || i.severity <= filter.maxSeverity))
      .sort((a, b) => a.severity - b.severity || a.openedAt - b.openedAt)
      .map(copy);
  }
}

@@exercise Parsing commands and computing metrics
Write:

- `parseCommand(json: string): Result<Command, string[]>`: invalid JSON → `["invalid JSON"]`; `type` must be one of the four; `by` a non-empty string; `open` needs a string `title` and `severity` 1–4; the others need a string `id`. Collect all problems in this order: `type`, then `by`, then the type-specific fields. Messages: `"type must be open|ack|resolve|reopen"`, `"by is required"`, `"title is required"`, `"severity must be 1-4"`, `"id is required"`. Unknown types stop after the type message (plus `by`). Extra fields are dropped.
- `metrics(incidents: Incident[]): { open: number; mttaMinutes: number | null; mttrMinutes: number | null; bySeverity: Record<Severity, number> }`: `open` counts incidents not resolved; MTTA averages `ackAt - openedAt` over incidents with `ackAt`; MTTR averages `resolvedAt - openedAt` over incidents with `resolvedAt`; times are in milliseconds, report minutes rounded to 1 decimal (`null` when there's no data); `bySeverity` counts all incidents, with every severity present.
@@starter
type Result<T, E> = { ok: true; value: T } | { ok: false; error: E };
type Status = "open" | "acknowledged" | "resolved";
type Severity = 1 | 2 | 3 | 4;
interface Incident {
  id: string;
  title: string;
  severity: Severity;
  status: Status;
  openedAt: number;
  ackAt?: number;
  resolvedAt?: number;
  history: { at: number; status: Status; by: string }[];
}
type Command =
  | { type: "open"; title: string; severity: Severity; by: string }
  | { type: "ack"; id: string; by: string }
  | { type: "resolve"; id: string; by: string }
  | { type: "reopen"; id: string; by: string };

function parseCommand(json: string): Result<Command, string[]> {
  throw new Error("TODO: not done yet");
}

function metrics(incidents: Incident[]): { open: number; mttaMinutes: number | null; mttrMinutes: number | null; bySeverity: Record<Severity, number> } {
  throw new Error("TODO: not done yet");
}

@@tests
check(parseCommand('{"type":"open","title":"Site down","severity":1,"by":"monitor","x":1}'), { ok: true, value: { type: "open", title: "Site down", severity: 1, by: "monitor" } }, "open");
check(parseCommand('{"type":"ack","id":"INC-1","by":"ana"}'), { ok: true, value: { type: "ack", id: "INC-1", by: "ana" } }, "ack");
check(parseCommand('{"type":"open","severity":7}'), { ok: false, error: ["by is required", "title is required", "severity must be 1-4"] }, "open problems");
check(parseCommand('{"type":"delete","by":""}'), { ok: false, error: ["type must be open|ack|resolve|reopen", "by is required"] }, "bad type");
check(parseCommand("nope"), { ok: false, error: ["invalid JSON"] }, "bad json");
const min = 60_000;
const incidents: Incident[] = [
  { id: "1", title: "a", severity: 1, status: "resolved", openedAt: 0, ackAt: 5 * min, resolvedAt: 60 * min, history: [] },
  { id: "2", title: "b", severity: 2, status: "acknowledged", openedAt: 0, ackAt: 12 * min, history: [] },
  { id: "3", title: "c", severity: 2, status: "open", openedAt: 0, history: [] },
  { id: "4", title: "d", severity: 4, status: "resolved", openedAt: 0, resolvedAt: 25 * min, history: [] },
];
check(metrics(incidents), { open: 2, mttaMinutes: 8.5, mttrMinutes: 42.5, bySeverity: { 1: 1, 2: 2, 3: 0, 4: 1 } }, "metrics");
check(metrics([]).mttaMinutes, null, "no data");
@@hint
Parse to `unknown`, check it's an object, then copy into `const o: Record<string, unknown> = { ...data }`. Build the command in each branch once all checks passed. For averages, write `avg(values: number[]): number | null` that returns `Math.round(sum / values.length / 60000 * 10) / 10`.
@@solution
type Result<T, E> = { ok: true; value: T } | { ok: false; error: E };
type Status = "open" | "acknowledged" | "resolved";
type Severity = 1 | 2 | 3 | 4;
interface Incident {
  id: string;
  title: string;
  severity: Severity;
  status: Status;
  openedAt: number;
  ackAt?: number;
  resolvedAt?: number;
  history: { at: number; status: Status; by: string }[];
}
type Command =
  | { type: "open"; title: string; severity: Severity; by: string }
  | { type: "ack"; id: string; by: string }
  | { type: "resolve"; id: string; by: string }
  | { type: "reopen"; id: string; by: string };

const TYPES = ["open", "ack", "resolve", "reopen"] as const;
const isType = (x: unknown): x is Command["type"] => typeof x === "string" && (TYPES as readonly string[]).includes(x);
const isSeverity = (x: unknown): x is Severity => x === 1 || x === 2 || x === 3 || x === 4;

function parseCommand(json: string): Result<Command, string[]> {
  let data: unknown;
  try {
    data = JSON.parse(json);
  } catch {
    return { ok: false, error: ["invalid JSON"] };
  }
  if (typeof data !== "object" || data === null) return { ok: false, error: ["type must be open|ack|resolve|reopen"] };
  const o: Record<string, unknown> = { ...data };
  const errors: string[] = [];
  const type = o.type;
  if (!isType(type)) errors.push("type must be open|ack|resolve|reopen");
  const by = o.by;
  if (typeof by !== "string" || by === "") errors.push("by is required");
  if (!isType(type)) return { ok: false, error: errors };
  if (type === "open") {
    const { title, severity } = o;
    if (typeof title !== "string") errors.push("title is required");
    if (!isSeverity(severity)) errors.push("severity must be 1-4");
    if (errors.length > 0 || typeof title !== "string" || !isSeverity(severity) || typeof by !== "string") return { ok: false, error: errors };
    return { ok: true, value: { type, title, severity, by } };
  }
  const id = o.id;
  if (typeof id !== "string") errors.push("id is required");
  if (errors.length > 0 || typeof id !== "string" || typeof by !== "string") return { ok: false, error: errors };
  return { ok: true, value: { type, id, by } };
}

function avg(values: number[]): number | null {
  if (values.length === 0) return null;
  const sum = values.reduce((a, b) => a + b, 0);
  return Math.round((sum / values.length / 60000) * 10) / 10;
}

function metrics(incidents: Incident[]): { open: number; mttaMinutes: number | null; mttrMinutes: number | null; bySeverity: Record<Severity, number> } {
  const bySeverity: Record<Severity, number> = { 1: 0, 2: 0, 3: 0, 4: 0 };
  for (const i of incidents) bySeverity[i.severity]++;
  return {
    open: incidents.filter((i) => i.status !== "resolved").length,
    mttaMinutes: avg(incidents.flatMap((i) => (i.ackAt === undefined ? [] : [i.ackAt - i.openedAt]))),
    mttrMinutes: avg(incidents.flatMap((i) => (i.resolvedAt === undefined ? [] : [i.resolvedAt - i.openedAt]))),
    bySeverity,
  };
}
