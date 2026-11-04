@@lesson d2-logs-traces Logs, Traces and Incident Response in Practice
@@text
Metrics tell you **that** something is wrong; logs and traces tell you **where** and **why**. This lesson covers structured logging, log levels and correlation IDs, distributed tracing with OpenTelemetry (spans, context propagation, sampling), and the incident response routine: severity, roles, communication and the blameless postmortem. The exercises parse structured logs and find the critical path through a trace.

## Structured logging

- Log **events as JSON** (or logfmt) with consistent fields: `timestamp` (UTC, ISO 8601), `level`, `service`, `message`, **`trace_id`/`request_id`**, user or tenant ID (not personal data), and event-specific fields. Machines can filter and aggregate; humans can still read.
- Levels: **DEBUG** (development detail), **INFO** (normal events), **WARN** (unexpected but handled), **ERROR** (a request failed), **FATAL/CRITICAL** (the process can't continue). Production usually runs at INFO.
- Don't log secrets, tokens, passwords or full card numbers; mask personal data (GDPR, 個人情報保護法). Log sampling or rate limiting for very noisy events.
- Pipelines: application → stdout → agent (Fluent Bit, Vector, OpenTelemetry Collector) → store (Loki, Elasticsearch/OpenSearch, cloud logging) → queries, dashboards, alerts. Set retention by need and cost.

## Distributed tracing

- A **trace** is the whole journey of one request; a **span** is one operation (name, start, duration, attributes, status, parent span ID).
- **Context propagation**: the W3C **`traceparent`** header (`00-<trace-id>-<span-id>-<flags>`) carries the trace across services; libraries and service meshes inject it.
- **OpenTelemetry** (OTel): vendor-neutral APIs/SDKs and the Collector for traces, metrics and logs; export to Jaeger, Tempo, Zipkin, or commercial backends.
- **Sampling**: head-based (decide at the start, e.g. 10%) or **tail-based** (decide after the trace completes, so you keep all errors and slow traces).
- The **critical path** is the chain of spans that determined the end-to-end latency: speeding up spans off the critical path (running in parallel) doesn't help.
- **Exemplars** link a metric spike to example trace IDs.

## Incident response

1. **Detect** (alert on symptoms), **declare** early with a severity (SEV1 = major customer impact … SEV4 = minor).
2. Roles: **incident commander** (coordinates, decides), **operations lead** (hands on keyboard), **communications lead** (status page, stakeholders), scribe.
3. **Mitigate first**: roll back, fail over, disable a feature flag, scale up. Root cause can wait.
4. Communicate on a fixed cadence (e.g. every 30 minutes), even when there's no news.
5. Resolve, then a **blameless postmortem** within days: timeline, impact, detection, contributing factors (not a single "root cause" or a person), what went well, action items with owners and dates.

Useful measures: **MTTD** (detect), **MTTA** (acknowledge), **MTTR** (restore/recover).

@@quiz
Why include a trace_id in every log line?
- To encrypt logs
* To correlate log lines from all services for one request
- To reduce log volume
- It's required by JSON
? Jump from a trace to its logs and back.

@@quiz
Which header carries trace context between services in the W3C standard?
- X-Request-ID
* traceparent
- Authorization
- X-B3-Sampled only
? B3 is the older Zipkin format.

@@quiz
You want to keep every trace that had an error, but only a few normal ones. Which sampling?
- Head-based 1%
* Tail-based sampling
- No sampling
- Sampling by service name
? The decision happens after the trace completes.

@@quiz
A request takes 900 ms. Two parallel calls take 300 ms and 700 ms. Optimising the 300 ms call to 100 ms saves how much end-to-end?
* Nothing; it isn't on the critical path
- 200 ms
- 100 ms
- 300 ms
? The 700 ms call determines the latency.

@@quiz
During a major incident, what should the team do first?
- Find the root cause
* Mitigate the impact (rollback, failover, disable the feature)
- Write the postmortem
- Assign blame
? Restore service, then investigate.

@@quiz
Which role coordinates the incident and makes decisions?
- Scribe
- Operations lead
* Incident commander
- Communications lead
? They don't have to type commands themselves.

@@flashcards
structured log :: JSON with consistent fields
log level ERROR :: a request failed
production level :: usually INFO
never log :: secrets, tokens, passwords
trace :: one request's journey
span :: one operation with parent ID
traceparent :: W3C trace context header
OpenTelemetry :: vendor-neutral telemetry
OTel Collector :: receive, process, export
head sampling :: decide at start
tail sampling :: decide after completion
critical path :: spans that set total latency
exemplar :: metric point linked to a trace
incident commander :: coordinates and decides
mitigate first :: rollback, failover, flag off
status cadence :: regular updates
blameless postmortem :: contributing factors, actions
MTTD :: mean time to detect
MTTA :: mean time to acknowledge

@@drill
What is the W3C header name for trace context?
= traceparent
? tracestate carries vendor data.

@@drill
Which sampling strategy decides after the trace has completed?
= tail-based
= tail
= tail sampling
= tail-based sampling
? Head-based decides at the start.

@@exercise Query structured logs
Parse JSON log lines (skip and count malformed ones), count events per level and service, compute the error rate per service, list the slowest requests above `SLOW_MS`, and reconstruct every log line of the request with the most errors by its trace ID.
@@lang python
@@starter
import json
from collections import Counter

SLOW_MS = 1000
lines = [
    '{"ts":"2026-11-04T10:00:01Z","level":"INFO","service":"api","trace_id":"t1","msg":"request","path":"/orders","ms":120}',
    '{"ts":"2026-11-04T10:00:02Z","level":"INFO","service":"api","trace_id":"t2","msg":"request","path":"/orders","ms":1450}',
    '{"ts":"2026-11-04T10:00:02Z","level":"ERROR","service":"payments","trace_id":"t2","msg":"card provider timeout"}',
    '{"ts":"2026-11-04T10:00:03Z","level":"WARN","service":"payments","trace_id":"t2","msg":"retrying"}',
    'not json at all',
    '{"ts":"2026-11-04T10:00:04Z","level":"ERROR","service":"payments","trace_id":"t2","msg":"card provider timeout"}',
    '{"ts":"2026-11-04T10:00:05Z","level":"INFO","service":"api","trace_id":"t3","msg":"request","path":"/health","ms":3}',
    '{"ts":"2026-11-04T10:00:06Z","level":"ERROR","service":"api","trace_id":"t4","msg":"request","path":"/orders","ms":2100}',
    '{"ts":"2026-11-04T10:00:06Z","level":"INFO","service":"payments","trace_id":"t3","msg":"ok"}',
]

def parse(lines):
    raise NotImplementedError
@@expected
8 events, 1 malformed
levels: {'INFO': 4, 'ERROR': 3, 'WARN': 1}
api: 4 events, error rate 25%
payments: 4 events, error rate 50%
slow: t4 /orders 2100 ms, t2 /orders 1450 ms
trace t2 (2 errors):
  2026-11-04T10:00:02Z INFO  api      request
  2026-11-04T10:00:02Z ERROR payments card provider timeout
  2026-11-04T10:00:03Z WARN  payments retrying
  2026-11-04T10:00:04Z ERROR payments card provider timeout
@@requires
def parse
json.loads(
Counter(
@@hint
`parse` returns `(events, malformed_count)`, catching `json.JSONDecodeError`. Error rate per service = ERROR events / all events of that service. The noisiest trace is `Counter(e["trace_id"] for e in errors).most_common(1)`.
@@solution
import json
from collections import Counter

SLOW_MS = 1000
lines = [
    '{"ts":"2026-11-04T10:00:01Z","level":"INFO","service":"api","trace_id":"t1","msg":"request","path":"/orders","ms":120}',
    '{"ts":"2026-11-04T10:00:02Z","level":"INFO","service":"api","trace_id":"t2","msg":"request","path":"/orders","ms":1450}',
    '{"ts":"2026-11-04T10:00:02Z","level":"ERROR","service":"payments","trace_id":"t2","msg":"card provider timeout"}',
    '{"ts":"2026-11-04T10:00:03Z","level":"WARN","service":"payments","trace_id":"t2","msg":"retrying"}',
    'not json at all',
    '{"ts":"2026-11-04T10:00:04Z","level":"ERROR","service":"payments","trace_id":"t2","msg":"card provider timeout"}',
    '{"ts":"2026-11-04T10:00:05Z","level":"INFO","service":"api","trace_id":"t3","msg":"request","path":"/health","ms":3}',
    '{"ts":"2026-11-04T10:00:06Z","level":"ERROR","service":"api","trace_id":"t4","msg":"request","path":"/orders","ms":2100}',
    '{"ts":"2026-11-04T10:00:06Z","level":"INFO","service":"payments","trace_id":"t3","msg":"ok"}',
]

def parse(lines):
    events, bad = [], 0
    for line in lines:
        try:
            events.append(json.loads(line))
        except json.JSONDecodeError:
            bad += 1
    return events, bad

events, bad = parse(lines)
print(f"{len(events)} events, {bad} malformed")
print("levels:", dict(Counter(e["level"] for e in events)))
for service in sorted({e["service"] for e in events}):
    mine = [e for e in events if e["service"] == service]
    errors = sum(e["level"] == "ERROR" for e in mine)
    print(f"{service}: {len(mine)} events, error rate {errors / len(mine):.0%}")
slow = sorted((e for e in events if e.get("ms", 0) > SLOW_MS), key=lambda e: -e["ms"])
print("slow:", ", ".join(f"{e['trace_id']} {e['path']} {e['ms']} ms" for e in slow))
trace, count = Counter(e["trace_id"] for e in events if e["level"] == "ERROR").most_common(1)[0]
print(f"trace {trace} ({count} errors):")
for e in sorted((e for e in events if e["trace_id"] == trace), key=lambda e: e["ts"]):
    print(f"  {e['ts']} {e['level']:<5} {e['service']:<8} {e['msg']}")

@@exercise Find the critical path of a trace
Spans have an ID, parent, service, start and duration (ms). Build the tree, print it indented with timings, then compute the critical path: walking backwards from a span's end, take the child that finished last before the cursor, recurse into it, and continue from that child's start. Report each critical span's self time (its duration minus the time covered by its children) to show where to optimise.
@@lang python
@@starter
spans = [
    ("a", None, "gateway", "GET /checkout", 0, 920),
    ("b", "a", "auth", "verify token", 5, 40),
    ("c", "a", "cart", "load cart", 50, 180),
    ("d", "c", "db", "SELECT cart_items", 60, 120),
    ("e", "a", "pricing", "calculate", 50, 300),
    ("f", "a", "payments", "authorize", 240, 650),
    ("g", "f", "card-api", "POST /authorize", 260, 600),
    ("h", "a", "email", "queue receipt", 895, 15),
]

def children(span_id):
    raise NotImplementedError
@@expected
gateway: GET /checkout  [0-920 ms, 920 ms]
  auth: verify token  [5-45 ms, 40 ms]
  cart: load cart  [50-230 ms, 180 ms]
    db: SELECT cart_items  [60-180 ms, 120 ms]
  pricing: calculate  [50-350 ms, 300 ms]
  payments: authorize  [240-890 ms, 650 ms]
    card-api: POST /authorize  [260-860 ms, 600 ms]
  email: queue receipt  [895-910 ms, 15 ms]
critical path:
  gateway: GET /checkout [0-920 ms], self time 25 ms
  auth: verify token [5-45 ms], self time 40 ms
  cart: load cart [50-230 ms], self time 60 ms
  db: SELECT cart_items [60-180 ms], self time 120 ms
  payments: authorize [240-890 ms], self time 50 ms
  card-api: POST /authorize [260-860 ms], self time 600 ms
  email: queue receipt [895-910 ms], self time 15 ms
not on the critical path: pricing
@@requires
def children
def critical_path
end
@@hint
Each span is `(id, parent, service, name, start, duration)`; end = start + duration. Children overlapping a later critical child (like a parallel call) are skipped. Self time: merge the children's intervals (they may overlap) and subtract the covered length from the span's duration.
@@solution
spans = [
    ("a", None, "gateway", "GET /checkout", 0, 920),
    ("b", "a", "auth", "verify token", 5, 40),
    ("c", "a", "cart", "load cart", 50, 180),
    ("d", "c", "db", "SELECT cart_items", 60, 120),
    ("e", "a", "pricing", "calculate", 50, 300),
    ("f", "a", "payments", "authorize", 240, 650),
    ("g", "f", "card-api", "POST /authorize", 260, 600),
    ("h", "a", "email", "queue receipt", 895, 15),
]
by_id = {s[0]: s for s in spans}

def end(span):
    return span[4] + span[5]

def children(span_id):
    return sorted((s for s in spans if s[1] == span_id), key=lambda s: s[4])

def show(span, depth=0):
    print(f"{'  ' * depth}{span[2]}: {span[3]}  [{span[4]}-{end(span)} ms, {span[5]} ms]")
    for child in children(span[0]):
        show(child, depth + 1)

def self_time(span):
    covered, cur_start, cur_end = 0, None, None
    for c in children(span[0]):
        if cur_end is None or c[4] > cur_end:
            if cur_end is not None:
                covered += cur_end - cur_start
            cur_start, cur_end = c[4], end(c)
        else:
            cur_end = max(cur_end, end(c))
    if cur_end is not None:
        covered += cur_end - cur_start
    return span[5] - covered

def critical_path(span):
    # walk backwards from the span's end: take the child that finished last
    # before the cursor, recurse into it, then continue before its start
    path, cursor = [], end(span)
    while True:
        done = [c for c in children(span[0]) if end(c) <= cursor]
        if not done:
            break
        last = max(done, key=end)
        path = critical_path(last) + path
        cursor = last[4]
    return [span] + path

root = next(s for s in spans if s[1] is None)
show(root)
path = critical_path(root)
print("critical path:")
for s in path:
    print(f"  {s[2]}: {s[3]} [{s[4]}-{end(s)} ms], self time {self_time(s)} ms")
off = sorted(s[2] for s in spans if s not in path)
print("not on the critical path:", ", ".join(off))
