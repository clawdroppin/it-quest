@@lesson m8-observability Logging, Metrics and Tracing in Node.js
@@concepts text-processing, json
@@text
When a service misbehaves at 3 a.m., you can't attach a debugger. You rely on what it recorded: **logs** (what happened), **metrics** (how much, how fast) and **traces** (where time went across services). Together they're called **observability**.

## Structured logging

Plain text logs are hard to search. **Structured** logs are one JSON object per line, with consistent fields:

```javascript
function createLogger(base = {}, minLevel = "info") {
  const levels = { debug: 10, info: 20, warn: 30, error: 40 };
  const write = (level, msg, fields = {}) => {
    if (levels[level] < levels[minLevel]) return;
    console.log(JSON.stringify({ time: new Date(0).toISOString(), level, msg, ...base, ...fields }));
  };
  return {
    debug: (m, f) => write("debug", m, f),
    info: (m, f) => write("info", m, f),
    warn: (m, f) => write("warn", m, f),
    error: (m, f) => write("error", m, f),
    child: (extra) => createLogger({ ...base, ...extra }, minLevel),
  };
}
const log = createLogger({ service: "ticket-api" });
const reqLog = log.child({ requestId: "r-81f2" });
reqLog.info("ticket created", { ticketId: 42, ms: 18 });
reqLog.debug("not shown at info level");
```

(The demo pins the time to keep output stable; real loggers use the current time.)

In production use **pino** (very fast) or winston. Good practice:

- **Levels**: `debug` for development detail, `info` for normal events, `warn` for unusual but handled situations, `error` for failures that need attention. Configure the level per environment.
- **Child loggers** carry context (request ID, user ID, job ID) into every line automatically.
- **Correlation IDs**: accept or generate an `X-Request-Id` per request and pass it to downstream services, so you can follow one request through every log. `AsyncLocalStorage` (from `node:async_hooks`) keeps it available without passing it through every function.
- **Redact** secrets: passwords, tokens, `Authorization` headers, personal data.
- Write to **stdout**; let the platform (systemd/journald, Docker, Kubernetes) collect and ship logs to Loki, Elasticsearch/OpenSearch, Datadog, CloudWatch...

## Metrics

Metrics are numbers over time, cheap to store and perfect for dashboards and alerts:

| Type | Example | Question it answers |
|---|---|---|
| Counter | `http_requests_total{status="500"}` | how many, how fast (rate) |
| Gauge | `queue_length`, `memory_bytes` | current value |
| Histogram | `http_request_duration_seconds` | distribution: p50, p95, p99 |

Prometheus scrapes a `/metrics` endpoint (via `prom-client`); Grafana graphs it. The **RED** method for services: **R**ate, **E**rrors, **D**uration. For resources, **USE**: Utilisation, Saturation, Errors.

Percentiles matter more than averages: if the average is 80 ms but p99 is 4 s, one request in a hundred is painfully slow.

## Tracing

A **trace** follows one request through many services as a tree of **spans** (name, start, duration, attributes). **OpenTelemetry** is the vendor-neutral standard: its Node SDK auto-instruments HTTP, Express, database drivers and more, and exports to Jaeger, Tempo, Honeycomb, Datadog...

```javascript,norun
const { trace } = require("@opentelemetry/api");
const tracer = trace.getTracer("ticket-api");
await tracer.startActiveSpan("assign-ticket", async (span) => {
  span.setAttribute("ticket.id", id);
  try { await assign(id); } finally { span.end(); }
});
```

## Health checks and alerts

- `/healthz` (liveness: the process is alive) and `/readyz` (readiness: dependencies like the database are reachable) for load balancers and Kubernetes.
- Alert on **symptoms users feel** (error rate, latency, SLO burn) rather than every cause (CPU 80%).
- Every alert should be actionable and link to a runbook.

@@quiz
Why log JSON objects instead of free text?
* Each field can be searched, filtered and aggregated by log tools
- JSON is smaller
- Free text can't contain numbers
- Node.js requires it
? E.g. find all lines where `level="error"` and `requestId="r-81f2"`.

@@quiz
Which metric type fits request latency?
- Counter
- Gauge
* Histogram
- Log line
? Histograms give percentiles like p95 and p99.

@@quiz
What does a correlation (request) ID let you do?
- Encrypt logs
* Follow a single request through every log line and service it touched
- Speed up requests
- Count users
? Pass it in a header to downstream services.

@@drill
Which endpoint path is the common convention for a health check?
= /health
= /healthz
? Return 200 when the service can work.

@@flashcards
Structured logging :: one JSON object per log line
Log levels :: debug, info, warn, error
Child logger :: logger with extra context fields
Correlation ID :: request ID carried through logs and services
AsyncLocalStorage :: per-request context without passing it around
Counter / gauge / histogram :: metric types
RED :: rate, errors, duration
Span :: one timed operation in a trace
OpenTelemetry :: open standard for traces, metrics and logs
Liveness vs readiness :: process alive vs able to serve

@@exercise A redacting logger
Write `createLogger({ level = "info", base = {}, clock, out })` where `clock()` returns an ISO time string and `out(line)` receives each log line (a JSON string). It returns an object with `debug/info/warn/error(msg, fields)` and `child(fields)`.

- each line: `{ time, level, msg, ...base, ...fields }` (key order as written)
- messages below `level` are dropped
- **redaction**: any key (at any depth) named `password`, `token`, `authorization` or `secret` (case-insensitive) has its value replaced by `"[REDACTED]"`
- an `Error` passed as `fields.err` is logged as `{ name, message }` (no stack)
@@starter
function createLogger({ level = "info", base = {}, clock = () => new Date().toISOString(), out = console.log } = {}) {
  throw new Error("TODO: not done yet");
}

@@tests
const lines = [];
const log = createLogger({ level: "info", base: { service: "api" }, clock: () => "T", out: (l) => lines.push(JSON.parse(l)) });
log.debug("hidden");
log.info("login", { user: "ana", password: "hunter2", headers: { Authorization: "Bearer x", accept: "json" } });
log.child({ requestId: "r1" }).error("failed", { err: new TypeError("bad input"), attempt: 2 });
check(lines, [
  { time: "T", level: "info", msg: "login", service: "api", user: "ana", password: "[REDACTED]", headers: { Authorization: "[REDACTED]", accept: "json" } },
  { time: "T", level: "error", msg: "failed", service: "api", requestId: "r1", err: { name: "TypeError", message: "bad input" }, attempt: 2 },
], "logged lines");
@@hint
Write `redact(value)`: arrays map over items; plain objects become `Object.fromEntries(Object.entries(v).map(([k, x]) => [k, SECRET.has(k.toLowerCase()) ? "[REDACTED]" : redact(x)]))`; anything else is returned as is. Convert `fields.err` before redacting.
@@solution
const LEVELS = { debug: 10, info: 20, warn: 30, error: 40 };
const SECRET = new Set(["password", "token", "authorization", "secret"]);

function redact(value) {
  if (Array.isArray(value)) return value.map(redact);
  if (value instanceof Error) return { name: value.name, message: value.message };
  if (typeof value === "object" && value !== null) {
    return Object.fromEntries(Object.entries(value).map(([k, v]) => [k, SECRET.has(k.toLowerCase()) ? "[REDACTED]" : redact(v)]));
  }
  return value;
}

function createLogger({ level = "info", base = {}, clock = () => new Date().toISOString(), out = console.log } = {}) {
  const write = (lvl, msg, fields = {}) => {
    if (LEVELS[lvl] < LEVELS[level]) return;
    out(JSON.stringify(redact({ time: clock(), level: lvl, msg, ...base, ...fields })));
  };
  return {
    debug: (m, f) => write("debug", m, f),
    info: (m, f) => write("info", m, f),
    warn: (m, f) => write("warn", m, f),
    error: (m, f) => write("error", m, f),
    child: (extra) => createLogger({ level, base: { ...base, ...extra }, clock, out }),
  };
}

@@exercise Latency histogram
Write `class Histogram` with `constructor(buckets)` (sorted upper bounds in ms, e.g. `[50, 100, 250, 500, 1000]`):

- `observe(ms)`: record a value
- `count`, `sum` getters
- `percentile(p)` (p from 0 to 100): estimate from the buckets: the upper bound of the first bucket where the cumulative count reaches `ceil(p / 100 × count)`; values above the last bound count in an `Infinity` bucket. Return `null` when empty.
- `toPrometheus(name)`: lines in Prometheus text format: one `name_bucket{le="<bound>"} <cumulative count>` per bucket, then `le="+Inf"`, then `name_sum <sum>` and `name_count <count>`, joined by `\n`
@@starter
class Histogram {
  constructor(buckets) {
    throw new Error("TODO: not done yet");
  }
}

@@tests
const h = new Histogram([50, 100, 250]);
for (const ms of [12, 40, 45, 80, 95, 120, 300, 30, 20, 60]) h.observe(ms);
check([h.count, h.sum], [10, 802], "count and sum");
check([h.percentile(50), h.percentile(90), h.percentile(100), h.percentile(0)], [50, 250, Infinity, 50], "percentiles");
check(h.toPrometheus("http_ms").split("\n"), ['http_ms_bucket{le="50"} 5', 'http_ms_bucket{le="100"} 8', 'http_ms_bucket{le="250"} 9', 'http_ms_bucket{le="+Inf"} 10', "http_ms_sum 802", "http_ms_count 10"], "prometheus");
check(new Histogram([1]).percentile(50), null, "empty");
@@hint
Keep `counts = new Array(buckets.length + 1).fill(0)`; in `observe`, find the first index where `ms <= buckets[i]` (or the last slot). For percentiles, `target = Math.max(1, Math.ceil(p / 100 * count))` and walk the cumulative counts.
@@solution
class Histogram {
  constructor(buckets) {
    this.buckets = buckets;
    this.counts = new Array(buckets.length + 1).fill(0);
    this.total = 0;
  }

  observe(ms) {
    const i = this.buckets.findIndex((b) => ms <= b);
    this.counts[i === -1 ? this.buckets.length : i]++;
    this.total += ms;
  }

  get count() {
    return this.counts.reduce((a, b) => a + b, 0);
  }

  get sum() {
    return this.total;
  }

  percentile(p) {
    const count = this.count;
    if (count === 0) return null;
    const target = Math.max(1, Math.ceil((p / 100) * count));
    let cumulative = 0;
    for (let i = 0; i < this.counts.length; i++) {
      cumulative += this.counts[i];
      if (cumulative >= target) return i < this.buckets.length ? this.buckets[i] : Infinity;
    }
    return Infinity;
  }

  toPrometheus(name) {
    const lines = [];
    let cumulative = 0;
    this.buckets.forEach((b, i) => {
      cumulative += this.counts[i];
      lines.push(`${name}_bucket{le="${b}"} ${cumulative}`);
    });
    lines.push(`${name}_bucket{le="+Inf"} ${this.count}`);
    lines.push(`${name}_sum ${this.sum}`, `${name}_count ${this.count}`);
    return lines.join("\n");
  }
}
