@@lesson t9-alert-router Alert Router
@@text
Monitoring systems produce a flood of alerts. Something has to decide **who gets woken up**: the network on-call engineer for a core switch outage, a Slack channel for a slow query, *nobody* for a dev box or a server that's down for planned patching. Prometheus **Alertmanager**, PagerDuty and Opsgenie all do this with the same few ideas, and you'll build them in TypeScript:

- **validation**: alerts arrive as JSON, so a type guard rejects anything malformed;
- **label matchers** like `team=network`, `env!=dev`, `host=~^web` (a regex);
- **routes**: the first match wins, with a minimum severity for paging;
- **silences**: maintenance windows that mute matching alerts;
- **de-duplication** by fingerprint, and **grouping** into one notification per receiver.

```text
skipped 2 invalid alert(s)
8 alert(s), 7 after de-duplication

→ network-oncall  critical LinkDown {"team":"network","env":"prod","device":"tky-core-sw1"}
→ network-slack   warning  HighLatency {"team":"network","env":"prod","device":"osk-vpn1"}
🔇 silenced (db02 patching, change CHG-1042) critical DiskFull {"team":"dba","env":"prod","host":"db02"}
→ dba-slack       warning  SlowQuery {"host":"db01","env":"prod","team":"dba"}
✖ no route         info     CpuHigh {"team":"web","env":"dev","host":"web-dev1"}
...
Notifications:
  network-oncall: 1 alert(s), worst critical: LinkDown
  web-slack: 2 alert(s), worst warning: CertExpiring, CpuHigh
```

The types do real work here. `Decision` is a discriminated union (`send` / `silenced` / `dropped`), so `main.ts` can't read a receiver from a silenced alert, and `Matcher["op"]` only allows the three operators.

Steps:

1. `node --test`: all 12 tests fail with "TODO".
2. Write the functions in `router.ts`. The doc comment above each one says exactly what it must do.
3. When the tests pass: `node main.ts alerts.json config.json 2026-10-31T09:30:00Z` (or `npm start`).
4. Optional but recommended: `npm install`, then `npx tsc` to type-check the whole project, like CI would.
@@project alert_router Alert Router
Write the eight functions in `router.ts` until `node --test` passes, then route the sample alerts with `node main.ts alerts.json config.json 2026-10-31T09:30:00Z`. Change `config.json` to see different routing.
@@file router.ts
// router.ts: decide where monitoring alerts go, like Prometheus Alertmanager or PagerDuty rules.

export const SEVERITIES = ["info", "warning", "critical"] as const;
export type Severity = (typeof SEVERITIES)[number];

export type Alert = {
  name: string;                    // e.g. "DiskFull"
  severity: Severity;
  labels: Record<string, string>;  // e.g. { host: "db01", env: "prod", team: "dba" }
  startsAt: string;                // ISO time, e.g. "2026-10-31T09:00:00Z"
};

/** A label matcher: `env=prod`, `env!=dev` or `host=~^web` (a regular expression). */
export type Matcher = { label: string; op: "=" | "!=" | "=~"; value: string };

export type Route = {
  receiver: string;                // where to send: "network-oncall", "dba-slack", ...
  match: Matcher[];                // all must match
  minSeverity?: Severity;          // ignore alerts below this
};

export type Silence = { match: Matcher[]; from: string; until: string; reason: string };

export type Decision =
  | { kind: "send"; receiver: string; alert: Alert }
  | { kind: "silenced"; reason: string; alert: Alert }
  | { kind: "dropped"; alert: Alert };

/**
 * Check that unknown JSON is an Alert: name a non-empty string, severity one of SEVERITIES,
 * labels an object whose values are all strings, startsAt a string that Date can parse.
 */
export function isAlert(x: unknown): x is Alert {
    throw new Error("TODO: not done yet");
}

/**
 * Parse "env=prod", "env!=dev" or "host=~^web". Spaces around the parts are trimmed.
 * Throw Error(`bad matcher: ${text}`) if there's no operator or the label is empty.
 * Check "!=" and "=~" before "=", or "env!=dev" splits at the wrong place.
 */
export function parseMatcher(text: string): Matcher {
    throw new Error("TODO: not done yet");
}

/** True if the alert's labels satisfy the matcher. A missing label counts as "". */
export function matches(m: Matcher, labels: Record<string, string>): boolean {
    throw new Error("TODO: not done yet");
}

/** -1, 0 or 1, comparing two severities by their position in SEVERITIES. */
export function compareSeverity(a: Severity, b: Severity): number {
    throw new Error("TODO: not done yet");
}

/**
 * What happens to one alert at time `now`:
 * 1. If a silence matches all its matchers and from <= now < until: silenced, with its reason.
 * 2. Otherwise the FIRST route whose matchers all match, and whose minSeverity (if any) the
 *    alert reaches: send to that route's receiver.
 * 3. Otherwise: dropped.
 */
export function route(alert: Alert, routes: Route[], silences: Silence[], now: Date): Decision {
    throw new Error("TODO: not done yet");
}

/**
 * Alerts often fire again and again. Keep one per (name + labels) fingerprint: the one with the
 * earliest startsAt. Labels count regardless of key order. Keep the order of first appearance.
 */
export function dedupe(alerts: Alert[]): Alert[] {
    throw new Error("TODO: not done yet");
}

/** name + sorted labels, e.g. "DiskFull{env=prod,host=db01}". */
export function fingerprint(a: Alert): string {
    throw new Error("TODO: not done yet");
}

/**
 * One message per receiver for the "send" decisions: receivers in alphabetical order, each
 * line "<receiver>: <n> alert(s), worst <severity>: <names sorted, comma separated>".
 */
export function summarize(decisions: Decision[]): string[] {
    throw new Error("TODO: not done yet");
}
@@file router.test.ts
import { test } from "node:test";
import assert from "node:assert/strict";
import { isAlert, parseMatcher, matches, compareSeverity, route, dedupe, fingerprint, summarize } from "./router.ts";
import type { Alert, Route, Silence } from "./router.ts";

const alert = (name: string, severity: Alert["severity"], labels: Record<string, string>, startsAt = "2026-10-31T09:00:00Z"): Alert => ({
  name,
  severity,
  labels,
  startsAt,
});

test("isAlert accepts a valid alert", () => {
  assert.equal(isAlert({ name: "DiskFull", severity: "critical", labels: { host: "db01" }, startsAt: "2026-10-31T09:00:00Z" }), true);
});

test("isAlert rejects bad data", () => {
  const good = { name: "X", severity: "info", labels: {}, startsAt: "2026-10-31T09:00:00Z" };
  assert.equal(isAlert(null), false);
  assert.equal(isAlert({ ...good, name: "" }), false);
  assert.equal(isAlert({ ...good, severity: "fatal" }), false);
  assert.equal(isAlert({ ...good, labels: { port: 22 } }), false);
  assert.equal(isAlert({ ...good, labels: ["a"] }), false);
  assert.equal(isAlert({ ...good, startsAt: "yesterday" }), false);
});

test("parseMatcher handles the three operators", () => {
  assert.deepEqual(parseMatcher("env=prod"), { label: "env", op: "=", value: "prod" });
  assert.deepEqual(parseMatcher(" env != dev "), { label: "env", op: "!=", value: "dev" });
  assert.deepEqual(parseMatcher("host=~^web\\d+"), { label: "host", op: "=~", value: "^web\\d+" });
});

test("parseMatcher rejects text without an operator", () => {
  assert.throws(() => parseMatcher("prod"), /bad matcher: prod/);
  assert.throws(() => parseMatcher("=prod"), /bad matcher/);
});

test("matches", () => {
  const labels = { env: "prod", host: "web07" };
  assert.equal(matches(parseMatcher("env=prod"), labels), true);
  assert.equal(matches(parseMatcher("env!=prod"), labels), false);
  assert.equal(matches(parseMatcher("host=~^web"), labels), true);
  assert.equal(matches(parseMatcher("team="), labels), true, "a missing label is the empty string");
  assert.equal(matches(parseMatcher("team!=dba"), labels), true);
});

test("compareSeverity", () => {
  assert.equal(compareSeverity("critical", "warning"), 1);
  assert.equal(compareSeverity("info", "info"), 0);
  assert.equal(compareSeverity("info", "critical"), -1);
});

// Built inside each test, so an unfinished parseMatcher fails the tests instead of the whole file.
const config = (): { routes: Route[]; silences: Silence[] } => ({
  routes: [
    { receiver: "network-oncall", match: [parseMatcher("team=network")], minSeverity: "critical" },
    { receiver: "network-slack", match: [parseMatcher("team=network")] },
    { receiver: "dba-oncall", match: [parseMatcher("team=dba"), parseMatcher("env=prod")] },
  ],
  silences: [
    { match: [parseMatcher("host=db02")], from: "2026-10-31T08:00:00Z", until: "2026-10-31T10:00:00Z", reason: "patching db02" },
  ],
});
const now = new Date("2026-10-31T09:30:00Z");

test("route: first matching route wins, respecting minSeverity", () => {
  const { routes, silences } = config();
  const d1 = route(alert("LinkDown", "critical", { team: "network" }), routes, silences, now);
  const d2 = route(alert("HighLatency", "warning", { team: "network" }), routes, silences, now);
  assert.deepEqual([d1.kind, d1.kind === "send" && d1.receiver], ["send", "network-oncall"]);
  assert.deepEqual([d2.kind, d2.kind === "send" && d2.receiver], ["send", "network-slack"]);
});

test("route: all matchers must match, otherwise dropped", () => {
  const { routes, silences } = config();
  assert.equal(route(alert("SlowQuery", "warning", { team: "dba", env: "dev" }), routes, silences, now).kind, "dropped");
  assert.equal(route(alert("SlowQuery", "warning", { team: "dba", env: "prod" }), routes, silences, now).kind, "send");
});

test("route: silences win while active", () => {
  const { routes, silences } = config();
  const a = alert("DiskFull", "critical", { team: "dba", env: "prod", host: "db02" });
  const d = route(a, routes, silences, now);
  assert.deepEqual(d.kind === "silenced" && d.reason, "patching db02");
  assert.equal(route(a, routes, silences, new Date("2026-10-31T10:00:00Z")).kind, "send", "until is exclusive");
});

test("fingerprint ignores label order", () => {
  assert.equal(fingerprint(alert("X", "info", { b: "2", a: "1" })), "X{a=1,b=2}");
  assert.equal(fingerprint(alert("X", "info", { a: "1", b: "2" })), fingerprint(alert("X", "info", { b: "2", a: "1" })));
});

test("dedupe keeps the earliest of each fingerprint, in first-seen order", () => {
  const list = [
    alert("DiskFull", "critical", { host: "db01" }, "2026-10-31T09:05:00Z"),
    alert("CpuHigh", "warning", { host: "web01" }),
    alert("DiskFull", "critical", { host: "db01" }, "2026-10-31T08:55:00Z"),
    alert("DiskFull", "critical", { host: "db02" }),
  ];
  const out = dedupe(list);
  assert.deepEqual(out.map((a) => `${a.name}@${a.labels.host}`), ["DiskFull@db01", "CpuHigh@web01", "DiskFull@db02"]);
  assert.equal(out[0].startsAt, "2026-10-31T08:55:00Z");
});

test("summarize groups sends per receiver", () => {
  const a = alert("LinkDown", "critical", {});
  const b = alert("HighLatency", "warning", {});
  const c = alert("Flapping", "info", {});
  const lines = summarize([
    { kind: "send", receiver: "network-slack", alert: b },
    { kind: "dropped", alert: c },
    { kind: "send", receiver: "dba-oncall", alert: c },
    { kind: "send", receiver: "network-slack", alert: a },
    { kind: "silenced", reason: "x", alert: a },
  ]);
  assert.deepEqual(lines, [
    "dba-oncall: 1 alert(s), worst info: Flapping",
    "network-slack: 2 alert(s), worst critical: HighLatency, LinkDown",
  ]);
});
@@file main.ts
// main.ts: route a batch of alerts.  Usage: node main.ts [alerts.json] [config.json] [now]
import { readFile } from "node:fs/promises";
import { isAlert, parseMatcher, route, dedupe, summarize } from "./router.ts";
import type { Route, Silence, Severity } from "./router.ts";

type ConfigFile = {
  routes: { receiver: string; match: string[]; minSeverity?: Severity }[];
  silences: { match: string[]; from: string; until: string; reason: string }[];
};

const [alertsPath = "alerts.json", configPath = "config.json", nowText] = process.argv.slice(2);
const now = nowText ? new Date(nowText) : new Date();

try {
  const raw: unknown = JSON.parse(await readFile(alertsPath, "utf8"));
  if (!Array.isArray(raw)) throw new Error(`${alertsPath} must contain a JSON array`);
  const valid = raw.filter(isAlert);
  if (valid.length < raw.length) console.log(`skipped ${raw.length - valid.length} invalid alert(s)`);

  // Our own config file: trusted, so a type assertion is fine here.
  const config = JSON.parse(await readFile(configPath, "utf8")) as ConfigFile;
  const routes: Route[] = config.routes.map((r) => ({ ...r, match: r.match.map(parseMatcher) }));
  const silences: Silence[] = config.silences.map((s) => ({ ...s, match: s.match.map(parseMatcher) }));

  const alerts = dedupe(valid);
  console.log(`${valid.length} alert(s), ${alerts.length} after de-duplication\n`);
  const decisions = alerts.map((a) => route(a, routes, silences, now));
  for (const d of decisions) {
    const what = `${d.alert.severity.padEnd(8)} ${d.alert.name} ${JSON.stringify(d.alert.labels)}`;
    if (d.kind === "send") console.log(`→ ${d.receiver.padEnd(15)} ${what}`);
    else if (d.kind === "silenced") console.log(`🔇 silenced (${d.reason}) ${what}`);
    else console.log(`✖ no route         ${what}`);
  }
  console.log("\nNotifications:");
  for (const line of summarize(decisions)) console.log(`  ${line}`);
} catch (e) {
  console.error(`error: ${e instanceof Error ? e.message : String(e)}`);
  process.exitCode = 1;
}
@@file alerts.json
[
  { "name": "LinkDown", "severity": "critical", "labels": { "team": "network", "env": "prod", "device": "tky-core-sw1" }, "startsAt": "2026-10-31T09:12:00Z" },
  { "name": "HighLatency", "severity": "warning", "labels": { "team": "network", "env": "prod", "device": "osk-vpn1" }, "startsAt": "2026-10-31T09:10:00Z" },
  { "name": "DiskFull", "severity": "critical", "labels": { "team": "dba", "env": "prod", "host": "db02" }, "startsAt": "2026-10-31T09:01:00Z" },
  { "name": "SlowQuery", "severity": "warning", "labels": { "team": "dba", "env": "prod", "host": "db01" }, "startsAt": "2026-10-31T09:05:00Z" },
  { "name": "SlowQuery", "severity": "warning", "labels": { "host": "db01", "env": "prod", "team": "dba" }, "startsAt": "2026-10-31T08:59:00Z" },
  { "name": "CpuHigh", "severity": "warning", "labels": { "team": "web", "env": "prod", "host": "web07" }, "startsAt": "2026-10-31T09:15:00Z" },
  { "name": "CpuHigh", "severity": "info", "labels": { "team": "web", "env": "dev", "host": "web-dev1" }, "startsAt": "2026-10-31T09:16:00Z" },
  { "name": "CertExpiring", "severity": "warning", "labels": { "team": "web", "env": "prod", "host": "web01", "days": "9" }, "startsAt": "2026-10-31T00:00:00Z" },
  { "name": "", "severity": "critical", "labels": {}, "startsAt": "2026-10-31T09:00:00Z" },
  { "name": "PrinterJam", "severity": "panic", "labels": { "floor": "3F" }, "startsAt": "2026-10-31T09:00:00Z" }
]
@@file config.json
{
  "routes": [
    { "receiver": "network-oncall", "match": ["team=network", "env=prod"], "minSeverity": "critical" },
    { "receiver": "network-slack", "match": ["team=network"] },
    { "receiver": "dba-oncall", "match": ["team=dba", "env=prod"], "minSeverity": "critical" },
    { "receiver": "dba-slack", "match": ["team=dba"] },
    { "receiver": "web-slack", "match": ["team=web", "env!=dev"] }
  ],
  "silences": [
    { "match": ["host=db02"], "from": "2026-10-31T08:00:00Z", "until": "2026-10-31T12:00:00Z", "reason": "db02 patching, change CHG-1042" }
  ]
}
@@file package.json
{
  "name": "alert-router",
  "version": "1.0.0",
  "private": true,
  "type": "module",
  "scripts": {
    "start": "node main.ts alerts.json config.json 2026-10-31T09:30:00Z",
    "test": "node --test",
    "check": "tsc"
  },
  "devDependencies": {
    "@types/node": "^22.0.0",
    "typescript": "^5.8.0"
  }
}
@@file tsconfig.json
{
  "compilerOptions": {
    "target": "es2023",
    "module": "nodenext",
    "strict": true,
    "noUncheckedIndexedAccess": false,
    "noEmit": true,
    "allowImportingTsExtensions": true,
    "verbatimModuleSyntax": true,
    "erasableSyntaxOnly": true,
    "skipLibCheck": true,
    "types": ["node"]
  },
  "include": ["*.ts"]
}
@@text
> Tips: in `isAlert`, cast to `Record<string, unknown>` after the object check, then test each field's `typeof`. `parseMatcher` is easiest with one regular expression with the alternatives `!=|=~|=` in that order. In `route`, check the silences first, then `routes.find(...)`. For `fingerprint`, sort `Object.keys(labels)`. `dedupe` needs a `Map<string, Alert>`, which keeps insertion order.
@@project alert_router_finished Alert Router: the finished version
The complete router. Ideas: read alerts from an HTTP endpoint (`node:http`, POST `/alerts`) so Alertmanager can call it directly, add "inhibit" rules (a `LinkDown` hides the `HighLatency` alerts it causes), send real Slack webhook messages with `fetch`, or validate `config.json` with Zod.
@@file router.ts
// router.ts: decide where monitoring alerts go, like Prometheus Alertmanager or PagerDuty rules.

export const SEVERITIES = ["info", "warning", "critical"] as const;
export type Severity = (typeof SEVERITIES)[number];

export type Alert = {
  name: string;                    // e.g. "DiskFull"
  severity: Severity;
  labels: Record<string, string>;  // e.g. { host: "db01", env: "prod", team: "dba" }
  startsAt: string;                // ISO time, e.g. "2026-10-31T09:00:00Z"
};

/** A label matcher: `env=prod`, `env!=dev` or `host=~^web` (a regular expression). */
export type Matcher = { label: string; op: "=" | "!=" | "=~"; value: string };

export type Route = {
  receiver: string;                // where to send: "network-oncall", "dba-slack", ...
  match: Matcher[];                // all must match
  minSeverity?: Severity;          // ignore alerts below this
};

export type Silence = { match: Matcher[]; from: string; until: string; reason: string };

export type Decision =
  | { kind: "send"; receiver: string; alert: Alert }
  | { kind: "silenced"; reason: string; alert: Alert }
  | { kind: "dropped"; alert: Alert };

/**
 * Check that unknown JSON is an Alert: name a non-empty string, severity one of SEVERITIES,
 * labels an object whose values are all strings, startsAt a string that Date can parse.
 */
export function isAlert(x: unknown): x is Alert {
  if (typeof x !== "object" || x === null) return false;
  const o = x as Record<string, unknown>;
  if (typeof o.name !== "string" || o.name === "") return false;
  if (typeof o.severity !== "string" || !(SEVERITIES as readonly string[]).includes(o.severity)) return false;
  if (typeof o.labels !== "object" || o.labels === null || Array.isArray(o.labels)) return false;
  if (!Object.values(o.labels).every((v) => typeof v === "string")) return false;
  return typeof o.startsAt === "string" && !Number.isNaN(Date.parse(o.startsAt));
}

/**
 * Parse "env=prod", "env!=dev" or "host=~^web". Spaces around the parts are trimmed.
 * Throw Error(`bad matcher: ${text}`) if there's no operator or the label is empty.
 * Check "!=" and "=~" before "=", or "env!=dev" splits at the wrong place.
 */
export function parseMatcher(text: string): Matcher {
  const m = /^\s*([^=!~\s]+)\s*(!=|=~|=)\s*(.*?)\s*$/.exec(text);
  if (!m) throw new Error(`bad matcher: ${text}`);
  return { label: m[1], op: m[2] as Matcher["op"], value: m[3] };
}

/** True if the alert's labels satisfy the matcher. A missing label counts as "". */
export function matches(m: Matcher, labels: Record<string, string>): boolean {
  const actual = labels[m.label] ?? "";
  switch (m.op) {
    case "=":
      return actual === m.value;
    case "!=":
      return actual !== m.value;
    case "=~":
      return new RegExp(m.value).test(actual);
  }
}

/** -1, 0 or 1, comparing two severities by their position in SEVERITIES. */
export function compareSeverity(a: Severity, b: Severity): number {
  return Math.sign(SEVERITIES.indexOf(a) - SEVERITIES.indexOf(b));
}

/**
 * What happens to one alert at time `now`:
 * 1. If a silence matches all its matchers and from <= now < until: silenced, with its reason.
 * 2. Otherwise the FIRST route whose matchers all match, and whose minSeverity (if any) the
 *    alert reaches: send to that route's receiver.
 * 3. Otherwise: dropped.
 */
export function route(alert: Alert, routes: Route[], silences: Silence[], now: Date): Decision {
  const t = now.getTime();
  for (const s of silences) {
    if (t >= Date.parse(s.from) && t < Date.parse(s.until) && s.match.every((m) => matches(m, alert.labels)))
      return { kind: "silenced", reason: s.reason, alert };
  }
  for (const r of routes) {
    if (r.minSeverity && compareSeverity(alert.severity, r.minSeverity) < 0) continue;
    if (r.match.every((m) => matches(m, alert.labels))) return { kind: "send", receiver: r.receiver, alert };
  }
  return { kind: "dropped", alert };
}

/**
 * Alerts often fire again and again. Keep one per (name + labels) fingerprint: the one with the
 * earliest startsAt. Labels count regardless of key order. Keep the order of first appearance.
 */
export function dedupe(alerts: Alert[]): Alert[] {
  const byKey = new Map<string, Alert>();
  for (const a of alerts) {
    const key = fingerprint(a);
    const seen = byKey.get(key);
    if (!seen || Date.parse(a.startsAt) < Date.parse(seen.startsAt)) byKey.set(key, a);
  }
  return [...byKey.values()];
}

/** name + sorted labels, e.g. "DiskFull{env=prod,host=db01}". */
export function fingerprint(a: Alert): string {
  const labels = Object.keys(a.labels)
    .sort()
    .map((k) => `${k}=${a.labels[k]}`)
    .join(",");
  return `${a.name}{${labels}}`;
}

/**
 * One message per receiver for the "send" decisions: receivers in alphabetical order, each
 * line "<receiver>: <n> alert(s), worst <severity>: <names sorted, comma separated>".
 */
export function summarize(decisions: Decision[]): string[] {
  const groups = new Map<string, Alert[]>();
  for (const d of decisions) {
    if (d.kind !== "send") continue;
    const list = groups.get(d.receiver) ?? [];
    list.push(d.alert);
    groups.set(d.receiver, list);
  }
  return [...groups.keys()].sort().map((receiver) => {
    const list = groups.get(receiver) ?? [];
    const worst = list.reduce<Severity>((w, a) => (compareSeverity(a.severity, w) > 0 ? a.severity : w), "info");
    const names = list.map((a) => a.name).sort().join(", ");
    return `${receiver}: ${list.length} alert(s), worst ${worst}: ${names}`;
  });
}
@@file router.test.ts
import { test } from "node:test";
import assert from "node:assert/strict";
import { isAlert, parseMatcher, matches, compareSeverity, route, dedupe, fingerprint, summarize } from "./router.ts";
import type { Alert, Route, Silence } from "./router.ts";

const alert = (name: string, severity: Alert["severity"], labels: Record<string, string>, startsAt = "2026-10-31T09:00:00Z"): Alert => ({
  name,
  severity,
  labels,
  startsAt,
});

test("isAlert accepts a valid alert", () => {
  assert.equal(isAlert({ name: "DiskFull", severity: "critical", labels: { host: "db01" }, startsAt: "2026-10-31T09:00:00Z" }), true);
});

test("isAlert rejects bad data", () => {
  const good = { name: "X", severity: "info", labels: {}, startsAt: "2026-10-31T09:00:00Z" };
  assert.equal(isAlert(null), false);
  assert.equal(isAlert({ ...good, name: "" }), false);
  assert.equal(isAlert({ ...good, severity: "fatal" }), false);
  assert.equal(isAlert({ ...good, labels: { port: 22 } }), false);
  assert.equal(isAlert({ ...good, labels: ["a"] }), false);
  assert.equal(isAlert({ ...good, startsAt: "yesterday" }), false);
});

test("parseMatcher handles the three operators", () => {
  assert.deepEqual(parseMatcher("env=prod"), { label: "env", op: "=", value: "prod" });
  assert.deepEqual(parseMatcher(" env != dev "), { label: "env", op: "!=", value: "dev" });
  assert.deepEqual(parseMatcher("host=~^web\\d+"), { label: "host", op: "=~", value: "^web\\d+" });
});

test("parseMatcher rejects text without an operator", () => {
  assert.throws(() => parseMatcher("prod"), /bad matcher: prod/);
  assert.throws(() => parseMatcher("=prod"), /bad matcher/);
});

test("matches", () => {
  const labels = { env: "prod", host: "web07" };
  assert.equal(matches(parseMatcher("env=prod"), labels), true);
  assert.equal(matches(parseMatcher("env!=prod"), labels), false);
  assert.equal(matches(parseMatcher("host=~^web"), labels), true);
  assert.equal(matches(parseMatcher("team="), labels), true, "a missing label is the empty string");
  assert.equal(matches(parseMatcher("team!=dba"), labels), true);
});

test("compareSeverity", () => {
  assert.equal(compareSeverity("critical", "warning"), 1);
  assert.equal(compareSeverity("info", "info"), 0);
  assert.equal(compareSeverity("info", "critical"), -1);
});

// Built inside each test, so an unfinished parseMatcher fails the tests instead of the whole file.
const config = (): { routes: Route[]; silences: Silence[] } => ({
  routes: [
    { receiver: "network-oncall", match: [parseMatcher("team=network")], minSeverity: "critical" },
    { receiver: "network-slack", match: [parseMatcher("team=network")] },
    { receiver: "dba-oncall", match: [parseMatcher("team=dba"), parseMatcher("env=prod")] },
  ],
  silences: [
    { match: [parseMatcher("host=db02")], from: "2026-10-31T08:00:00Z", until: "2026-10-31T10:00:00Z", reason: "patching db02" },
  ],
});
const now = new Date("2026-10-31T09:30:00Z");

test("route: first matching route wins, respecting minSeverity", () => {
  const { routes, silences } = config();
  const d1 = route(alert("LinkDown", "critical", { team: "network" }), routes, silences, now);
  const d2 = route(alert("HighLatency", "warning", { team: "network" }), routes, silences, now);
  assert.deepEqual([d1.kind, d1.kind === "send" && d1.receiver], ["send", "network-oncall"]);
  assert.deepEqual([d2.kind, d2.kind === "send" && d2.receiver], ["send", "network-slack"]);
});

test("route: all matchers must match, otherwise dropped", () => {
  const { routes, silences } = config();
  assert.equal(route(alert("SlowQuery", "warning", { team: "dba", env: "dev" }), routes, silences, now).kind, "dropped");
  assert.equal(route(alert("SlowQuery", "warning", { team: "dba", env: "prod" }), routes, silences, now).kind, "send");
});

test("route: silences win while active", () => {
  const { routes, silences } = config();
  const a = alert("DiskFull", "critical", { team: "dba", env: "prod", host: "db02" });
  const d = route(a, routes, silences, now);
  assert.deepEqual(d.kind === "silenced" && d.reason, "patching db02");
  assert.equal(route(a, routes, silences, new Date("2026-10-31T10:00:00Z")).kind, "send", "until is exclusive");
});

test("fingerprint ignores label order", () => {
  assert.equal(fingerprint(alert("X", "info", { b: "2", a: "1" })), "X{a=1,b=2}");
  assert.equal(fingerprint(alert("X", "info", { a: "1", b: "2" })), fingerprint(alert("X", "info", { b: "2", a: "1" })));
});

test("dedupe keeps the earliest of each fingerprint, in first-seen order", () => {
  const list = [
    alert("DiskFull", "critical", { host: "db01" }, "2026-10-31T09:05:00Z"),
    alert("CpuHigh", "warning", { host: "web01" }),
    alert("DiskFull", "critical", { host: "db01" }, "2026-10-31T08:55:00Z"),
    alert("DiskFull", "critical", { host: "db02" }),
  ];
  const out = dedupe(list);
  assert.deepEqual(out.map((a) => `${a.name}@${a.labels.host}`), ["DiskFull@db01", "CpuHigh@web01", "DiskFull@db02"]);
  assert.equal(out[0].startsAt, "2026-10-31T08:55:00Z");
});

test("summarize groups sends per receiver", () => {
  const a = alert("LinkDown", "critical", {});
  const b = alert("HighLatency", "warning", {});
  const c = alert("Flapping", "info", {});
  const lines = summarize([
    { kind: "send", receiver: "network-slack", alert: b },
    { kind: "dropped", alert: c },
    { kind: "send", receiver: "dba-oncall", alert: c },
    { kind: "send", receiver: "network-slack", alert: a },
    { kind: "silenced", reason: "x", alert: a },
  ]);
  assert.deepEqual(lines, [
    "dba-oncall: 1 alert(s), worst info: Flapping",
    "network-slack: 2 alert(s), worst critical: HighLatency, LinkDown",
  ]);
});
@@file main.ts
// main.ts: route a batch of alerts.  Usage: node main.ts [alerts.json] [config.json] [now]
import { readFile } from "node:fs/promises";
import { isAlert, parseMatcher, route, dedupe, summarize } from "./router.ts";
import type { Route, Silence, Severity } from "./router.ts";

type ConfigFile = {
  routes: { receiver: string; match: string[]; minSeverity?: Severity }[];
  silences: { match: string[]; from: string; until: string; reason: string }[];
};

const [alertsPath = "alerts.json", configPath = "config.json", nowText] = process.argv.slice(2);
const now = nowText ? new Date(nowText) : new Date();

try {
  const raw: unknown = JSON.parse(await readFile(alertsPath, "utf8"));
  if (!Array.isArray(raw)) throw new Error(`${alertsPath} must contain a JSON array`);
  const valid = raw.filter(isAlert);
  if (valid.length < raw.length) console.log(`skipped ${raw.length - valid.length} invalid alert(s)`);

  // Our own config file: trusted, so a type assertion is fine here.
  const config = JSON.parse(await readFile(configPath, "utf8")) as ConfigFile;
  const routes: Route[] = config.routes.map((r) => ({ ...r, match: r.match.map(parseMatcher) }));
  const silences: Silence[] = config.silences.map((s) => ({ ...s, match: s.match.map(parseMatcher) }));

  const alerts = dedupe(valid);
  console.log(`${valid.length} alert(s), ${alerts.length} after de-duplication\n`);
  const decisions = alerts.map((a) => route(a, routes, silences, now));
  for (const d of decisions) {
    const what = `${d.alert.severity.padEnd(8)} ${d.alert.name} ${JSON.stringify(d.alert.labels)}`;
    if (d.kind === "send") console.log(`→ ${d.receiver.padEnd(15)} ${what}`);
    else if (d.kind === "silenced") console.log(`🔇 silenced (${d.reason}) ${what}`);
    else console.log(`✖ no route         ${what}`);
  }
  console.log("\nNotifications:");
  for (const line of summarize(decisions)) console.log(`  ${line}`);
} catch (e) {
  console.error(`error: ${e instanceof Error ? e.message : String(e)}`);
  process.exitCode = 1;
}
@@file alerts.json
[
  { "name": "LinkDown", "severity": "critical", "labels": { "team": "network", "env": "prod", "device": "tky-core-sw1" }, "startsAt": "2026-10-31T09:12:00Z" },
  { "name": "HighLatency", "severity": "warning", "labels": { "team": "network", "env": "prod", "device": "osk-vpn1" }, "startsAt": "2026-10-31T09:10:00Z" },
  { "name": "DiskFull", "severity": "critical", "labels": { "team": "dba", "env": "prod", "host": "db02" }, "startsAt": "2026-10-31T09:01:00Z" },
  { "name": "SlowQuery", "severity": "warning", "labels": { "team": "dba", "env": "prod", "host": "db01" }, "startsAt": "2026-10-31T09:05:00Z" },
  { "name": "SlowQuery", "severity": "warning", "labels": { "host": "db01", "env": "prod", "team": "dba" }, "startsAt": "2026-10-31T08:59:00Z" },
  { "name": "CpuHigh", "severity": "warning", "labels": { "team": "web", "env": "prod", "host": "web07" }, "startsAt": "2026-10-31T09:15:00Z" },
  { "name": "CpuHigh", "severity": "info", "labels": { "team": "web", "env": "dev", "host": "web-dev1" }, "startsAt": "2026-10-31T09:16:00Z" },
  { "name": "CertExpiring", "severity": "warning", "labels": { "team": "web", "env": "prod", "host": "web01", "days": "9" }, "startsAt": "2026-10-31T00:00:00Z" },
  { "name": "", "severity": "critical", "labels": {}, "startsAt": "2026-10-31T09:00:00Z" },
  { "name": "PrinterJam", "severity": "panic", "labels": { "floor": "3F" }, "startsAt": "2026-10-31T09:00:00Z" }
]
@@file config.json
{
  "routes": [
    { "receiver": "network-oncall", "match": ["team=network", "env=prod"], "minSeverity": "critical" },
    { "receiver": "network-slack", "match": ["team=network"] },
    { "receiver": "dba-oncall", "match": ["team=dba", "env=prod"], "minSeverity": "critical" },
    { "receiver": "dba-slack", "match": ["team=dba"] },
    { "receiver": "web-slack", "match": ["team=web", "env!=dev"] }
  ],
  "silences": [
    { "match": ["host=db02"], "from": "2026-10-31T08:00:00Z", "until": "2026-10-31T12:00:00Z", "reason": "db02 patching, change CHG-1042" }
  ]
}
@@file package.json
{
  "name": "alert-router",
  "version": "1.0.0",
  "private": true,
  "type": "module",
  "scripts": {
    "start": "node main.ts alerts.json config.json 2026-10-31T09:30:00Z",
    "test": "node --test",
    "check": "tsc"
  },
  "devDependencies": {
    "@types/node": "^22.0.0",
    "typescript": "^5.8.0"
  }
}
@@file tsconfig.json
{
  "compilerOptions": {
    "target": "es2023",
    "module": "nodenext",
    "strict": true,
    "noUncheckedIndexedAccess": false,
    "noEmit": true,
    "allowImportingTsExtensions": true,
    "verbatimModuleSyntax": true,
    "erasableSyntaxOnly": true,
    "skipLibCheck": true,
    "types": ["node"]
  },
  "include": ["*.ts"]
}
@@text
🎉 Alert routing and fatigue are daily problems for SRE and operations teams. Having built a router yourself, you'll understand Alertmanager configs, PagerDuty rules and "why did nobody get paged?" incidents from the inside.
