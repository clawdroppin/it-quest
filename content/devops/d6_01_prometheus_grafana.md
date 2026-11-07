@@lesson d6-prometheus-grafana Monitoring with Prometheus and Grafana
@@concepts numbers, text-processing, maps
@@lang python
@@text
Module 2 explained *why* we monitor (SLOs, the three pillars). This lesson is the *how* with the most common open-source stack: **Prometheus** collects and stores metrics and fires alerts, **Alertmanager** routes the alerts to people, **Grafana** draws dashboards, and **Loki** stores logs. You'll understand the data format, PromQL and the maths behind `rate()` and percentiles well enough to build and debug real dashboards.

## How Prometheus works: pull

```text
 ┌────────────┐  scrape GET /metrics every 15 s   ┌───────────────┐
 │ Prometheus │ ────────────────────────────────▶ │ your service  │
 │  (TSDB)    │ ────────────────────────────────▶ │ node_exporter │  (host CPU, disk, memory)
 │            │ ────────────────────────────────▶ │ blackbox_exp. │  (probes URLs, ports, TLS expiry)
 └─────┬──────┘                                   └───────────────┘
       │ alert rules                    PromQL queries
       ▼                                     ▲
 ┌──────────────┐  Slack, email, PagerDuty  ┌┴──────────┐
 │ Alertmanager │ ───────────────────────▶  │  Grafana  │
 └──────────────┘                           └───────────┘
```

Prometheus **pulls** (scrapes) each target's `/metrics` page on a schedule and stores every number as a **time series**: a metric name plus **labels**, with timestamped values.

## The exposition format

What a service's `/metrics` page looks like:

```text
# HELP http_requests_total Requests handled, by route and status.
# TYPE http_requests_total counter
http_requests_total{route="/devices",status="200"} 1027
http_requests_total{route="/devices",status="500"} 3
# HELP process_resident_memory_bytes Memory in use.
# TYPE process_resident_memory_bytes gauge
process_resident_memory_bytes 4.1287e+07
# TYPE http_request_duration_seconds histogram
http_request_duration_seconds_bucket{le="0.1"} 900
http_request_duration_seconds_bucket{le="0.5"} 1010
http_request_duration_seconds_bucket{le="+Inf"} 1030
http_request_duration_seconds_sum 61.7
http_request_duration_seconds_count 1030
```

## The four metric types

| Type | Behaves like | Example | Query it with |
|---|---|---|---|
| **counter** | only goes up (resets to 0 on restart) | `http_requests_total` | `rate()`, `increase()` |
| **gauge** | goes up and down | memory, queue length, temperature | the value, `avg_over_time()` |
| **histogram** | counts observations in buckets (`le` = "less or equal") | request duration | `histogram_quantile()` |
| **summary** | quantiles computed in the app | older libraries | (can't be aggregated across instances) |

Naming conventions: snake_case, a unit suffix (`_seconds`, `_bytes`), and `_total` for counters.

**Labels** split a metric into series: `route`, `status`, `method`, `instance`, `job`. Never put unbounded values in labels (user IDs, full URLs, IP addresses): every distinct value is a new series, and millions of series ("**high cardinality**") will exhaust Prometheus's memory.

## Instrumenting a service

```python,norun
# pip install prometheus-client
from prometheus_client import Counter, Histogram, start_http_server

REQUESTS = Counter("http_requests_total", "Requests handled", ["route", "status"])
LATENCY = Histogram("http_request_duration_seconds", "Request duration", ["route"],
                    buckets=[0.05, 0.1, 0.25, 0.5, 1, 2.5])

def handle(route):
    with LATENCY.labels(route).time():          # observes the duration
        status = do_work(route)
    REQUESTS.labels(route, str(status)).inc()

start_http_server(9100)                          # serves /metrics on :9100
```

Go has `prometheus/client_golang`, Node has `prom-client`, Java has Micrometer. Infrastructure comes with **exporters**: `node_exporter` (Linux hosts), `windows_exporter`, `blackbox_exporter` (HTTP/TCP/ICMP/TLS probes), `postgres_exporter`, `mysqld_exporter`, `kube-state-metrics` (Kubernetes objects), cAdvisor (containers).

## Configuring Prometheus

```yaml
# prometheus.yml
global:
  scrape_interval: 15s
  evaluation_interval: 15s
rule_files: ["alerts.yml"]
alerting:
  alertmanagers: [{ static_configs: [{ targets: ["alertmanager:9093"] }] }]
scrape_configs:
  - job_name: api
    static_configs:
      - targets: ["api:8080"]
        labels: { env: prod }
  - job_name: node
    static_configs:
      - targets: ["web1:9100", "web2:9100", "db1:9100"]
  - job_name: kubernetes-pods         # in Kubernetes, discover targets automatically
    kubernetes_sd_configs: [{ role: pod }]
```

Every scrape adds the labels `job` and `instance`, and the series `up` (1 if the scrape worked, 0 if not). `up == 0` is the first alert everyone writes.

## PromQL: asking questions

```text
up                                                    # is every target reachable?
http_requests_total{status=~"5.."}                    # regex match on a label
rate(http_requests_total[5m])                         # per-second rate over the last 5 minutes
sum by (route) (rate(http_requests_total[5m]))        # requests per second, per route
sum(rate(http_requests_total{status=~"5.."}[5m]))
  / sum(rate(http_requests_total[5m]))                # error ratio (an SLI!)
histogram_quantile(0.95,
  sum by (le) (rate(http_request_duration_seconds_bucket[5m])))   # p95 latency
100 * (1 - node_filesystem_avail_bytes{mountpoint="/"} / node_filesystem_size_bytes{mountpoint="/"})  # disk used %
predict_linear(node_filesystem_avail_bytes[6h], 4 * 3600) < 0      # disk full within 4 hours?
increase(kube_pod_container_status_restarts_total[1h]) > 3          # crash-looping pods
topk(5, sum by (instance) (rate(node_cpu_seconds_total{mode!="idle"}[5m])))  # busiest hosts
```

Two rules that avoid most PromQL mistakes:

1. **Never graph a raw counter**: always `rate()` (per second) or `increase()` (total over the window) it. `rate()` handles restarts: when a counter drops, Prometheus assumes it reset to zero and keeps counting. The exercise implements exactly that.
2. **`rate()` first, then `sum()`**: `sum(rate(x[5m]))`, never `rate(sum(x)[5m])`, which breaks on resets.

Use a range at least **4× the scrape interval** (`[1m]` with 15 s scrapes, usually `[5m]`).

## Alerting rules and Alertmanager

```yaml
# alerts.yml
groups:
  - name: api
    rules:
      - alert: HighErrorRate
        expr: |
          sum(rate(http_requests_total{status=~"5.."}[5m])) / sum(rate(http_requests_total[5m])) > 0.05
        for: 10m                         # must be true for 10 minutes: no flapping
        labels: { severity: page }
        annotations:
          summary: "More than 5% of API requests are failing"
          runbook_url: https://wiki.example.com/runbooks/api-errors
      - alert: DiskWillFillIn4Hours
        expr: predict_linear(node_filesystem_avail_bytes{fstype!="tmpfs"}[6h], 4 * 3600) < 0
        for: 30m
        labels: { severity: ticket }
```

**Alertmanager** groups related alerts (one message for 40 failing pods, not 40 messages), deduplicates, **routes** by label (`severity: page` → PagerDuty, `severity: ticket` → a Slack channel), and supports **silences** during maintenance and **inhibition** (don't page about every service when the whole datacenter is down). **Recording rules** precompute expensive queries (`record: job:http_errors:ratio5m`) so dashboards stay fast.

Alert on **symptoms users feel** (errors, latency, SLO burn rate), not on every cause (CPU at 80%). Every page should be urgent, actionable and have a runbook.

## Grafana

Grafana connects to **data sources** (Prometheus, Loki, Elasticsearch, CloudWatch, Azure Monitor, PostgreSQL…) and shows **dashboards** of **panels** (time series, stat, gauge, table, heatmap, logs).

- **Variables** make one dashboard work for every environment or host: a `$instance` dropdown filled by `label_values(up, instance)`, used as `node_load1{instance="$instance"}`.
- Good dashboards follow a method: **RED** for services (Rate, Errors, Duration) and **USE** for resources (Utilization, Saturation, Errors). Put the big picture at the top and details below.
- Import community dashboards by ID (Node Exporter Full is 1860) instead of starting from zero.
- **Provision as code**: data sources and dashboards as YAML/JSON files in Git, loaded at startup, so dashboards are reviewed and never lost.

```yaml
# grafana/provisioning/datasources/prometheus.yml
apiVersion: 1
datasources:
  - name: Prometheus
    type: prometheus
    url: http://prometheus:9090
    isDefault: true
```

## Logs with Loki, traces with Tempo

**Loki** stores logs indexed only by labels (like Prometheus), which makes it cheap. Promtail or Grafana Alloy ship logs to it, and you query with **LogQL** in Grafana: `{app="api", env="prod"} |= "error" | json | status >= 500`. **Tempo** (or Jaeger) stores traces sent with **OpenTelemetry**. In Grafana you can jump from a latency spike on a graph to the logs and traces of that moment.

## A whole monitoring stack in one Compose file

```yaml
services:
  prometheus:
    image: prom/prometheus:v3.0.1
    volumes: ["./prometheus.yml:/etc/prometheus/prometheus.yml:ro", "./alerts.yml:/etc/prometheus/alerts.yml:ro"]
    ports: ["9090:9090"]
  alertmanager:
    image: prom/alertmanager:v0.27.0
    ports: ["9093:9093"]
  node-exporter:
    image: prom/node-exporter:v1.8.2
  grafana:
    image: grafana/grafana:11.3.0
    ports: ["3000:3000"]                 # admin / admin on first login
    volumes: ["./grafana/provisioning:/etc/grafana/provisioning:ro"]
```

In Kubernetes the standard is the **kube-prometheus-stack** Helm chart (Prometheus Operator, Grafana, Alertmanager, exporters and ready-made dashboards), where `ServiceMonitor` objects tell Prometheus what to scrape. In the cloud, there are managed versions: Amazon Managed Prometheus + Amazon Managed Grafana, Azure Monitor managed Prometheus, Google Cloud Managed Prometheus, Grafana Cloud.

@@quiz
Why do you almost always wrap a counter in `rate()` or `increase()`?
- Counters are stored as strings
* A counter only grows (and resets on restart), so its raw value isn't meaningful; the rate of change is
- Grafana can't show counters
- `rate()` makes queries faster
? `rate()` also handles resets: a drop is treated as a restart from zero.

@@quiz
Why is a `user_id` label on a request counter a bad idea?
- Labels can't contain numbers
* Every distinct value creates a new time series; millions of users means millions of series (high cardinality)
- It leaks the user's password
- Prometheus sorts labels alphabetically
? Use labels with a small, fixed set of values: route, method, status.

@@quiz
What does `for: 10m` in an alerting rule do?
- Sends the alert every 10 minutes
* The condition must stay true for 10 minutes before the alert fires, which avoids flapping
- Deletes the alert after 10 minutes
- Evaluates the rule every 10 minutes
? Short spikes don't wake anyone up.

@@quiz
Which is correct for the total request rate across all instances?
* `sum(rate(http_requests_total[5m]))`
- `rate(sum(http_requests_total)[5m])`
- `sum(http_requests_total)`
- `rate(http_requests_total)`
? Take the rate of each series first (that's where resets are handled), then add them up.

@@quiz
What does Alertmanager do that Prometheus doesn't?
- Stores metrics
* Groups, deduplicates, silences and routes alerts to the right people and tools
- Scrapes targets
- Draws dashboards
? Prometheus decides *that* something is wrong; Alertmanager decides *who hears about it, and how*.

@@drill
Which Prometheus metric type only ever goes up (or resets to zero)?
= counter
? Use rate() to turn it into a per-second value.

@@flashcards
scrape :: Prometheus pulls `/metrics` from each target on a schedule
time series :: a metric name + a set of labels, with timestamped values
counter / gauge / histogram / summary :: only up / up and down / buckets / app-side quantiles
`_total`, `_seconds`, `_bytes` :: naming conventions for counters and units
high cardinality :: too many label values → too many series
exporter :: translates a system's stats into Prometheus metrics (node_exporter, blackbox)
`up` :: 1 if the last scrape worked, 0 if not
`rate(x[5m])` :: per-second rate of a counter, reset-safe
`increase(x[1h])` :: how much a counter grew over the window
`sum by (label) (...)` :: aggregate, keeping one label
`histogram_quantile(0.95, sum by (le) (rate(x_bucket[5m])))` :: p95 from a histogram
`predict_linear(x[6h], 4*3600)` :: extrapolate a gauge 4 hours ahead
`for:` in an alert rule :: how long the condition must hold before firing
recording rule :: precompute an expensive query as a new series
Alertmanager :: grouping, routing, silences, inhibition
RED / USE :: Rate-Errors-Duration for services / Utilization-Saturation-Errors for resources
Grafana variable :: `$instance` dropdown reused in every panel
provisioning :: data sources and dashboards as files in Git
Loki / LogQL :: label-indexed logs / their query language
kube-prometheus-stack :: the standard Helm chart for monitoring Kubernetes
ServiceMonitor :: tells the Prometheus Operator what to scrape

@@exercise Parse the metrics page
Write `parse_metrics(text)` that reads the Prometheus text format and returns a dict from **series** to value (a float). The series is the metric name, plus the label part in braces exactly as written if there is one (e.g. `http_requests_total{route="/devices",status="200"}`). Skip blank lines and lines starting with `#`. The value is the first number after the series (ignore an optional timestamp after it).
@@starter
PAGE = """# HELP http_requests_total Requests.
# TYPE http_requests_total counter
http_requests_total{route="/devices",status="200"} 1027
http_requests_total{route="/devices",status="500"} 3

process_resident_memory_bytes 4.1287e+07
up 1 1730419200000
"""


def parse_metrics(text):
    pass


for series, value in parse_metrics(PAGE).items():
    print(series, value)
@@tests
m = parse_metrics(PAGE)
check(len(m), 4, "four series")
check(m['http_requests_total{route="/devices",status="200"}'], 1027.0, "a labelled counter")
check(m["process_resident_memory_bytes"], 41287000.0, "scientific notation")
check(m["up"], 1.0, "the timestamp is ignored")
check(parse_metrics('x{path="/a b"} 5\n'), {'x{path="/a b"}': 5.0}, "a space inside a label value")
@@hint
If the line contains `{`, the series ends at the closing `}` (`line.index("}") + 1`) and the rest holds the value; otherwise split on whitespace. Then `float(rest.split()[0])`.
@@solution
PAGE = """# HELP http_requests_total Requests.
# TYPE http_requests_total counter
http_requests_total{route="/devices",status="200"} 1027
http_requests_total{route="/devices",status="500"} 3

process_resident_memory_bytes 4.1287e+07
up 1 1730419200000
"""


def parse_metrics(text):
    out = {}
    for line in text.splitlines():
        line = line.strip()
        if not line or line.startswith("#"):
            continue
        if "{" in line:
            end = line.index("}") + 1
            series, rest = line[:end], line[end:]
        else:
            series, rest = line.split(None, 1)
        out[series] = float(rest.split()[0])
    return out


for series, value in parse_metrics(PAGE).items():
    print(series, value)

@@exercise rate() with counter resets
Write `increase(samples)` and `rate(samples)` the way Prometheus does (simplified). `samples` is a list of `(seconds, value)` pairs of a counter, oldest first. The increase adds up every step: normally `value - previous`, but if the value **dropped** the process restarted and counted from zero, so that step adds the new `value` itself. `rate` is the increase divided by the time between the first and last sample, **rounded to 3 decimals**. With fewer than 2 samples, both return `0`.
@@starter
def increase(samples):
    pass


def rate(samples):
    pass


s = [(0, 100), (15, 160), (30, 220), (45, 10), (60, 70)]
print(increase(s), rate(s))
@@tests
check(increase([(0, 100), (15, 160), (30, 220)]), 120, "a steady counter")
check(increase([(0, 100), (15, 160), (30, 220), (45, 10), (60, 70)]), 190, "a restart in the middle")
check(rate([(0, 100), (15, 160), (30, 220), (45, 10), (60, 70)]), 3.167, "per second")
check(rate([(0, 5)]), 0, "one sample")
check(rate([(0, 0), (60, 30)]), 0.5, "half a request per second")
@@hint
Loop over `zip(samples, samples[1:])`: `delta = v2 - v1 if v2 >= v1 else v2`. Rate: `round(increase(samples) / (samples[-1][0] - samples[0][0]), 3)`.
@@solution
def increase(samples):
    if len(samples) < 2:
        return 0
    total = 0
    for (_, v1), (_, v2) in zip(samples, samples[1:]):
        total += v2 - v1 if v2 >= v1 else v2
    return total


def rate(samples):
    if len(samples) < 2:
        return 0
    return round(increase(samples) / (samples[-1][0] - samples[0][0]), 3)


s = [(0, 100), (15, 160), (30, 220), (45, 10), (60, 70)]
print(increase(s), rate(s))

@@exercise Percentiles from a histogram
Write `histogram_quantile(q, buckets)` like PromQL's. `buckets` is a list of `(le, count)` sorted by `le`, where counts are **cumulative** (the number of observations ≤ `le`) and the last bucket's `le` is `float("inf")`. Find the rank `q * total` (total = the last count). Find the first bucket whose count is ≥ the rank, then interpolate linearly inside it: `lower + (upper - lower) * (rank - count_below) / (count_in_bucket_and_below - count_below)`, where `lower` is the previous bucket's `le` (0 for the first bucket). If that bucket is the `+Inf` one, return the previous bucket's `le`. Round to 3 decimals. Return `None` if there are no observations.
@@starter
INF = float("inf")


def histogram_quantile(q, buckets):
    pass


latency = [(0.1, 900), (0.5, 1010), (1.0, 1025), (INF, 1030)]
print("p50:", histogram_quantile(0.5, latency))
print("p95:", histogram_quantile(0.95, latency))
@@tests
b = [(0.1, 900), (0.5, 1010), (1.0, 1025), (INF, 1030)]
check(histogram_quantile(0.5, b), 0.057, "the median is in the first bucket")
check(histogram_quantile(0.95, b), 0.385, "p95 in the second bucket")
check(histogram_quantile(0.999, b), 1.0, "in the +Inf bucket: the highest finite bound")
check(histogram_quantile(0.5, [(1, 0), (2, 10), (INF, 10)]), 1.5, "an empty first bucket")
check(histogram_quantile(0.9, [(1, 0), (INF, 0)]), None, "no observations")
@@hint
`total = buckets[-1][1]`; return `None` if it's 0. `rank = q * total`. Walk with the previous bound and count (start at 0, 0). When `count >= rank`: if `le` is infinite return the previous bound, else interpolate.
@@solution
INF = float("inf")


def histogram_quantile(q, buckets):
    total = buckets[-1][1]
    if total == 0:
        return None
    rank = q * total
    prev_le, prev_count = 0, 0
    for le, count in buckets:
        if count >= rank:
            if le == INF:
                return prev_le
            return round(prev_le + (le - prev_le) * (rank - prev_count) / (count - prev_count), 3)
        prev_le, prev_count = le, count
    return prev_le


latency = [(0.1, 900), (0.5, 1010), (1.0, 1025), (INF, 1030)]
print("p50:", histogram_quantile(0.5, latency))
print("p95:", histogram_quantile(0.95, latency))
