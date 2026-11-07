@@lesson d9-ship-service Ship a Service End to End: Code, Container, Pipeline, Kubernetes and Monitoring
@@text
This capstone puts the whole DevOps toolchain around one small service, the way a real team would run it: a ticket API that **exposes its own Prometheus metrics**, a hardened **container image**, a **Compose** stack with **Prometheus**, alert rules and a provisioned **Grafana** dashboard, hardened **Kubernetes** manifests with Kustomize, and a **GitHub Actions** pipeline that tests, builds, scans, smoke-tests and pushes the image.

```text
 git push ──▶ GitHub Actions: unit tests ─▶ docker build ─▶ Trivy scan ─▶ smoke test ─▶ push to GHCR
                                                                                          │
 kubectl apply -k k8s/  (or Argo CD watching the repo) ◀──────────────── image tag ──────┘
        │
        ▼
 ┌──────────────┐   /metrics    ┌────────────┐   PromQL   ┌─────────┐
 │  ticket-api  │ ◀──────────── │ Prometheus │ ◀───────── │ Grafana │
 │  2 replicas  │    scrape     │  + alerts  │            │  (RED)  │
 └──────────────┘               └────────────┘            └─────────┘
```

## The service

| Request | Response |
|---|---|
| `GET /healthz` | `{"status": "ok"}`, for probes and health checks |
| `POST /tickets` with `{"title": ..., "priority": ...}` | 201 with the ticket; 422 with every problem; 400 for broken JSON |
| `GET /tickets?priority=urgent` | the matching tickets |
| `GET /metrics` | Prometheus text format |

It uses only Python's standard library, so there's nothing to install: `python main.py`, then `curl localhost:8080/metrics`.

## How it's built

| File | Job |
|---|---|
| `metrics.py` | counters, histograms and the Prometheus text format, written from scratch |
| `service.py` | the store, validation, routing and instrumentation (request counts and durations per route) |
| `main.py` | configuration from environment variables and graceful shutdown on SIGTERM |
| `test_*.py` | 12 unit and HTTP tests |
| `Dockerfile` / `.dockerignore` | a slim, non-root image with a healthcheck and exec-form entrypoint |
| `compose.yaml` | the API, Prometheus and Grafana, with security options on the API |
| `monitoring/` | scrape config, three alert rules (down, error rate, p95 latency), Grafana data source and a RED dashboard, all provisioned as code |
| `k8s/` | Deployment with probes, resource limits, a restricted security context and zero-downtime rollouts; Service; Kustomize |
| `.github/workflows/ci.yml` | test → build → scan → smoke test → push |
| `Makefile` | `make test`, `make up`, `make load` |

## Your tasks

Four functions in `metrics.py` raise `NotImplementedError`. Their docstrings and the tests in `test_metrics.py` are the specification:

1. `format_labels`: build `{route="/tickets",status="200"}` and escape `\`, `"` and newlines.
2. `Counter.render`: the `# HELP` and `# TYPE` lines, then one line per label combination, **sorted**.
3. `Histogram.observe`: buckets are **cumulative**: an observation of 0.3 counts in the `0.5`, `1`, ... and `+Inf` buckets. Also add to the sum and the count.
4. `Histogram.render`: a `_bucket` line per bucket with an extra `le` label, then `_sum` and `_count`.

```text
python -m unittest -v
```

When the tests pass, `python main.py` and look at http://localhost:8080/metrics after a few requests.

## Then run it like production

With Docker installed:

```text
make up            # or: docker compose up -d --build
make load          # some traffic, including 404s
```

- Prometheus at http://localhost:9090: open **Status → Targets** (is `ticket-api` UP?), then try the queries from the lesson: `sum by (route) (rate(http_requests_total[5m]))`.
- **Alerts** at http://localhost:9090/alerts: `docker compose stop api` and watch `TicketApiDown` go pending, then firing after a minute.
- Grafana at http://localhost:3000: the **Services → Ticket API (RED)** dashboard is already there, because it's provisioned from the repo.

With a local Kubernetes cluster (kind, minikube or Docker Desktop's):

```text
docker build -t ghcr.io/OWNER/ticket-api:0.1.0 .
kind load docker-image ghcr.io/OWNER/ticket-api:0.1.0     # kind only: copy the image into the cluster
kubectl create namespace tickets
kubectl apply -k k8s/
kubectl -n tickets rollout status deployment/ticket-api
kubectl -n tickets port-forward svc/ticket-api 8080:80
```

Push the repository to GitHub (replace `OWNER`) and the workflow runs on every push and pull request.
@@project ship_service Ship a Service End to End
Implement the four metric functions in `metrics.py` until `python -m unittest` passes, then run the stack with Docker Compose.
@@file metrics.py
"""Prometheus metrics with only the standard library: counters, histograms and the text format.

In a real service you'd use the prometheus-client package; writing it yourself shows exactly
what Prometheus reads from /metrics.
"""
import threading

INF = float("inf")


def format_value(v):
    """Prometheus writes +Inf, and whole numbers without a trailing .0."""
    if v == INF:
        return "+Inf"
    if float(v).is_integer():
        return str(int(v))
    return repr(float(v))


def format_labels(names, values):
    """('route', 'status'), ('/tickets', '200') -> '{route="/tickets",status="200"}'.

    No labels gives "". Inside a value, a backslash becomes \\\\, a double quote \\" and a
    newline \\n, so the line can always be parsed again.
    """
    raise NotImplementedError


class Counter:
    """A number that only goes up, one per combination of label values."""

    def __init__(self, name, help_text, labelnames=()):
        self.name, self.help, self.labelnames = name, help_text, tuple(labelnames)
        self._values = {}
        self._lock = threading.Lock()

    def inc(self, labels=(), amount=1):
        if amount < 0:
            raise ValueError("counters can only go up")
        labels = tuple(str(v) for v in labels)
        if len(labels) != len(self.labelnames):
            raise ValueError(f"{self.name} needs labels {self.labelnames}")
        with self._lock:
            self._values[labels] = self._values.get(labels, 0) + amount

    def value(self, labels=()):
        return self._values.get(tuple(str(v) for v in labels), 0)

    def render(self):
        """The HELP and TYPE lines, then one line per label combination, sorted by labels."""
        raise NotImplementedError


class Histogram:
    """Counts observations into cumulative buckets, plus their sum and count."""

    DEFAULT_BUCKETS = (0.005, 0.01, 0.025, 0.05, 0.1, 0.25, 0.5, 1, 2.5, 5)

    def __init__(self, name, help_text, labelnames=(), buckets=DEFAULT_BUCKETS):
        self.name, self.help, self.labelnames = name, help_text, tuple(labelnames)
        self.buckets = tuple(sorted(buckets)) + (INF,)
        self._series = {}  # labels -> {"counts": [per bucket], "sum": float, "count": int}
        self._lock = threading.Lock()

    def observe(self, value, labels=()):
        """Adds one observation. Buckets are cumulative: a value counts in EVERY bucket whose
        upper bound (le) is >= the value, including +Inf."""
        raise NotImplementedError

    def render(self):
        """HELP and TYPE, then for each label combination (sorted): one _bucket line per bucket
        with an extra le label, then _sum and _count."""
        raise NotImplementedError


class Registry:
    def __init__(self):
        self.metrics = []

    def register(self, metric):
        self.metrics.append(metric)
        return metric

    def render(self):
        """The whole /metrics page: every metric, separated by newlines, ending with one."""
        return "\n".join(m.render() for m in self.metrics) + "\n"
@@file service.py
"""The ticket API: storage, validation, HTTP handling and instrumentation."""
import json
import threading
import time
from http.server import BaseHTTPRequestHandler, ThreadingHTTPServer

from metrics import Counter, Histogram, Registry

PRIORITIES = ("low", "normal", "high", "urgent")


class TicketStore:
    """Tickets in memory. A real service would use a database; the API wouldn't change."""

    def __init__(self):
        self._tickets = []
        self._lock = threading.Lock()

    @staticmethod
    def validate(data):
        """Every problem with a new ticket, as a list of messages ([] when it's fine)."""
        if not isinstance(data, dict):
            return ["the body must be a JSON object"]
        problems = []
        title = data.get("title")
        if not isinstance(title, str) or not title.strip():
            problems.append("title is required")
        elif len(title.strip()) > 120:
            problems.append("title must be 120 characters or fewer")
        if data.get("priority", "normal") not in PRIORITIES:
            problems.append(f"priority must be one of: {', '.join(PRIORITIES)}")
        return problems

    def add(self, data):
        with self._lock:
            ticket = {"id": len(self._tickets) + 1, "title": data["title"].strip(),
                      "priority": data.get("priority", "normal"), "status": "open"}
            self._tickets.append(ticket)
            return ticket

    def list(self, priority=None):
        with self._lock:
            return [t for t in self._tickets if priority in (None, t["priority"])]


def make_server(host="127.0.0.1", port=8080, store=None, log=None):
    """A ready-to-run HTTP server with its own store and metrics (port 0 = any free port).
    `log` receives one JSON line per request (default: print it)."""
    log = log or (lambda line: print(line, flush=True))
    store = store or TicketStore()
    registry = Registry()
    requests_total = registry.register(Counter(
        "http_requests_total", "HTTP requests handled.", ["method", "route", "status"]))
    duration = registry.register(Histogram(
        "http_request_duration_seconds", "Time to handle a request.", ["route"],
        buckets=(0.005, 0.025, 0.1, 0.5, 2.5)))
    tickets_created = registry.register(Counter(
        "tickets_created_total", "Tickets created, by priority.", ["priority"]))

    class Handler(BaseHTTPRequestHandler):
        server_version = "TicketAPI/1.0"

        def log_message(self, fmt, *args):      # one structured line per request instead
            pass

        def _send(self, status, body, content_type="application/json"):
            data = body.encode() if isinstance(body, str) else json.dumps(body).encode()
            self.send_response(status)
            self.send_header("Content-Type", content_type)
            self.send_header("Content-Length", str(len(data)))
            self.end_headers()
            self.wfile.write(data)
            return status

        def _route(self):
            path = self.path.split("?")[0]
            # Known routes only, so a scanner hitting random URLs can't create endless series.
            return path if path in ("/healthz", "/metrics", "/tickets") else "other"

        def _handle(self, method):
            start = time.perf_counter()
            route = self._route()
            try:
                status = self._dispatch(method, route)
            except Exception:                    # never crash the server on one bad request
                status = self._send(500, {"error": "internal error"})
            requests_total.inc((method, route, status))
            duration.observe(time.perf_counter() - start, (route,))
            log(json.dumps({"method": method, "route": route, "status": status}))

        def _dispatch(self, method, route):
            if route == "/healthz" and method == "GET":
                return self._send(200, {"status": "ok"})
            if route == "/metrics" and method == "GET":
                return self._send(200, registry.render(), "text/plain; version=0.0.4")
            if route == "/tickets" and method == "GET":
                query = self.path.partition("?")[2]
                params = dict(p.split("=", 1) for p in query.split("&") if "=" in p)
                return self._send(200, store.list(params.get("priority")))
            if route == "/tickets" and method == "POST":
                length = int(self.headers.get("Content-Length") or 0)
                try:
                    data = json.loads(self.rfile.read(length) or b"null")
                except json.JSONDecodeError:
                    return self._send(400, {"error": "the body isn't valid JSON"})
                problems = TicketStore.validate(data)
                if problems:
                    return self._send(422, {"error": "invalid ticket", "problems": problems})
                ticket = store.add(data)
                tickets_created.inc((ticket["priority"],))
                return self._send(201, ticket)
            if route == "other":
                return self._send(404, {"error": "not found"})
            return self._send(405, {"error": "method not allowed"})

        def do_GET(self):
            self._handle("GET")

        def do_POST(self):
            self._handle("POST")

    server = ThreadingHTTPServer((host, port), Handler)
    server.registry = registry
    return server
@@file main.py
"""Starts the ticket API. Configuration comes from environment variables (twelve-factor)."""
import os
import signal
import sys

from service import make_server


def main():
    host = os.environ.get("HOST", "127.0.0.1")   # the container sets HOST=0.0.0.0
    port = int(os.environ.get("PORT", "8080"))
    server = make_server(host, port)

    # docker stop and Kubernetes send SIGTERM: finish cleanly instead of being killed.
    def stop(signum, frame):
        print('{"msg": "shutting down"}', flush=True)
        sys.exit(0)

    signal.signal(signal.SIGTERM, stop)
    print(f'{{"msg": "listening", "url": "http://{host}:{port}"}}', flush=True)
    try:
        server.serve_forever()
    except KeyboardInterrupt:
        pass
    finally:
        server.server_close()


if __name__ == "__main__":
    main()
@@file test_metrics.py
import unittest

from metrics import Counter, Histogram, Registry, format_labels, format_value


class FormatTest(unittest.TestCase):
    def test_values(self):
        self.assertEqual(format_value(3), "3")
        self.assertEqual(format_value(2.0), "2")
        self.assertEqual(format_value(0.25), "0.25")
        self.assertEqual(format_value(float("inf")), "+Inf")

    def test_labels(self):
        self.assertEqual(format_labels((), ()), "")
        self.assertEqual(format_labels(("route", "status"), ("/tickets", "200")), '{route="/tickets",status="200"}')

    def test_label_escaping(self):
        self.assertEqual(format_labels(("msg",), ('say "hi"\\n',)), '{msg="say \\"hi\\"\\\\n"}')
        self.assertEqual(format_labels(("msg",), ("two\nlines",)), '{msg="two\\nlines"}')


class CounterTest(unittest.TestCase):
    def test_render(self):
        c = Counter("http_requests_total", "Requests.", ["route", "status"])
        c.inc(("/tickets", 201))
        c.inc(("/healthz", 200), 2)
        c.inc(("/tickets", 201))
        self.assertEqual(c.render(), "\n".join([
            "# HELP http_requests_total Requests.",
            "# TYPE http_requests_total counter",
            'http_requests_total{route="/healthz",status="200"} 2',
            'http_requests_total{route="/tickets",status="201"} 2',
        ]))

    def test_no_labels_and_no_data(self):
        c = Counter("jobs_total", "Jobs.")
        self.assertEqual(c.render(), "# HELP jobs_total Jobs.\n# TYPE jobs_total counter")
        c.inc()
        self.assertTrue(c.render().endswith("\njobs_total 1"))

    def test_only_up(self):
        c = Counter("x_total", "X.")
        with self.assertRaises(ValueError):
            c.inc(amount=-1)
        with self.assertRaises(ValueError):
            c.inc(("unexpected",))


class HistogramTest(unittest.TestCase):
    def test_cumulative_buckets(self):
        h = Histogram("req_seconds", "Latency.", ["route"], buckets=(0.1, 0.5))
        for v in (0.05, 0.2, 0.3, 0.7):
            h.observe(v, ("/tickets",))
        self.assertEqual(h.render(), "\n".join([
            "# HELP req_seconds Latency.",
            "# TYPE req_seconds histogram",
            'req_seconds_bucket{route="/tickets",le="0.1"} 1',
            'req_seconds_bucket{route="/tickets",le="0.5"} 3',
            'req_seconds_bucket{route="/tickets",le="+Inf"} 4',
            'req_seconds_sum{route="/tickets"} 1.25',
            'req_seconds_count{route="/tickets"} 4',
        ]))

    def test_boundary_counts_in_its_bucket(self):
        h = Histogram("x", "X.", buckets=(1,))
        h.observe(1)
        self.assertIn('x_bucket{le="1"} 1', h.render())


class RegistryTest(unittest.TestCase):
    def test_page(self):
        r = Registry()
        r.register(Counter("a_total", "A.")).inc()
        r.register(Counter("b_total", "B."))
        page = r.render()
        self.assertTrue(page.endswith("\n"))
        self.assertIn("a_total 1\n# HELP b_total", page)


if __name__ == "__main__":
    unittest.main()
@@file test_service.py
import json
import threading
import unittest
import urllib.error
import urllib.request

from service import TicketStore, make_server


class ValidateTest(unittest.TestCase):
    def test_good(self):
        self.assertEqual(TicketStore.validate({"title": "Printer jammed", "priority": "high"}), [])
        self.assertEqual(TicketStore.validate({"title": "No priority given"}), [])

    def test_every_problem(self):
        self.assertEqual(TicketStore.validate({"title": "  ", "priority": "asap"}),
                         ["title is required", "priority must be one of: low, normal, high, urgent"])
        self.assertEqual(TicketStore.validate([]), ["the body must be a JSON object"])
        self.assertEqual(TicketStore.validate({"title": "x" * 121}), ["title must be 120 characters or fewer"])


class ApiTest(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.server = make_server(port=0, log=lambda line: None)   # any free port, quiet
        cls.base = f"http://127.0.0.1:{cls.server.server_address[1]}"
        cls.thread = threading.Thread(target=cls.server.serve_forever, daemon=True)
        cls.thread.start()

    @classmethod
    def tearDownClass(cls):
        cls.server.shutdown()
        cls.server.server_close()

    def call(self, method, path, body=None):
        data = None if body is None else (body if isinstance(body, bytes) else json.dumps(body).encode())
        req = urllib.request.Request(self.base + path, data=data, method=method)
        try:
            with urllib.request.urlopen(req) as resp:
                return resp.status, resp.read().decode()
        except urllib.error.HTTPError as e:
            return e.code, e.read().decode()

    def test_flow(self):
        status, body = self.call("GET", "/healthz")
        self.assertEqual((status, json.loads(body)), (200, {"status": "ok"}))

        status, body = self.call("POST", "/tickets", {"title": "VPN down", "priority": "urgent"})
        self.assertEqual(status, 201)
        self.assertEqual(json.loads(body)["title"], "VPN down")
        self.call("POST", "/tickets", {"title": "New mouse", "priority": "low"})

        status, body = self.call("GET", "/tickets?priority=urgent")
        self.assertEqual([t["title"] for t in json.loads(body)], ["VPN down"])

        self.assertEqual(self.call("POST", "/tickets", {"title": ""})[0], 422)
        self.assertEqual(self.call("POST", "/tickets", b"{not json")[0], 400)
        self.assertEqual(self.call("GET", "/nope")[0], 404)
        self.assertEqual(self.call("POST", "/healthz", {})[0], 405)

        status, page = self.call("GET", "/metrics")
        self.assertEqual(status, 200)
        self.assertIn('http_requests_total{method="POST",route="/tickets",status="201"} 2', page)
        self.assertIn('http_requests_total{method="GET",route="other",status="404"} 1', page)
        self.assertIn('tickets_created_total{priority="urgent"} 1', page)
        self.assertIn('http_request_duration_seconds_bucket{route="/tickets",le="+Inf"}', page)


if __name__ == "__main__":
    unittest.main()
@@file Dockerfile
# syntax=docker/dockerfile:1
# A small, non-root image. The app has no dependencies, so there is no install step; with a
# requirements.txt you would COPY it and `pip install --no-cache-dir -r` it BEFORE copying the code.
FROM python:3.12-slim

ENV PYTHONDONTWRITEBYTECODE=1 \
    PYTHONUNBUFFERED=1 \
    HOST=0.0.0.0 \
    PORT=8080

RUN useradd --system --uid 10001 --no-create-home app
WORKDIR /app
COPY metrics.py service.py main.py ./

USER 10001
EXPOSE 8080
HEALTHCHECK --interval=15s --timeout=3s --retries=3 \
  CMD python -c "import urllib.request; urllib.request.urlopen('http://127.0.0.1:8080/healthz', timeout=2)"
ENTRYPOINT ["python", "main.py"]
@@file .dockerignore
.git
.github
__pycache__/
*.pyc
test_*.py
k8s/
monitoring/
compose.yaml
Makefile
.env
@@file compose.yaml
# The service and its monitoring stack on one machine:
#   docker compose up -d --build
#   API http://localhost:8080   Prometheus http://localhost:9090   Grafana http://localhost:3000 (admin/admin)
services:
  api:
    build: .
    image: ticket-api:dev
    ports: ["8080:8080"]
    restart: unless-stopped
    read_only: true
    cap_drop: [ALL]
    security_opt: ["no-new-privileges:true"]

  prometheus:
    image: prom/prometheus:v3.0.1
    command: ["--config.file=/etc/prometheus/prometheus.yml", "--storage.tsdb.retention.time=7d"]
    volumes:
      - ./monitoring/prometheus.yml:/etc/prometheus/prometheus.yml:ro
      - ./monitoring/alerts.yml:/etc/prometheus/alerts.yml:ro
      - promdata:/prometheus
    ports: ["9090:9090"]
    depends_on: [api]

  grafana:
    image: grafana/grafana:11.3.0
    environment:
      GF_AUTH_ANONYMOUS_ENABLED: "true"        # read-only viewing without logging in (local use only)
      GF_AUTH_ANONYMOUS_ORG_ROLE: Viewer
    volumes:
      - ./monitoring/grafana/datasources.yml:/etc/grafana/provisioning/datasources/datasources.yml:ro
      - ./monitoring/grafana/dashboards.yml:/etc/grafana/provisioning/dashboards/dashboards.yml:ro
      - ./monitoring/grafana/dashboards:/var/lib/grafana/dashboards:ro
    ports: ["3000:3000"]
    depends_on: [prometheus]

volumes:
  promdata:
@@file Makefile
.PHONY: test run image up down load

test:            ## run the unit tests
	python -m unittest -v

run:             ## run the API locally
	python main.py

image:           ## build the container image
	docker build -t ticket-api:dev .

up:              ## start the API with Prometheus and Grafana
	docker compose up -d --build

down:
	docker compose down

load:            ## send some traffic so the dashboards have something to show
	for i in $$(seq 1 50); do \
	  curl -s -X POST localhost:8080/tickets -d '{"title":"Load test","priority":"low"}' > /dev/null; \
	  curl -s localhost:8080/tickets > /dev/null; curl -s localhost:8080/nope > /dev/null; \
	done
@@file monitoring/prometheus.yml
global:
  scrape_interval: 15s
  evaluation_interval: 15s

rule_files: ["/etc/prometheus/alerts.yml"]

scrape_configs:
  - job_name: ticket-api
    static_configs:
      - targets: ["api:8080"]
        labels: { env: dev }
@@file monitoring/alerts.yml
groups:
  - name: ticket-api
    rules:
      - alert: TicketApiDown
        expr: up{job="ticket-api"} == 0
        for: 1m
        labels: { severity: page }
        annotations:
          summary: "The ticket API can't be scraped"

      - alert: TicketApiHighErrorRate
        expr: |
          sum(rate(http_requests_total{job="ticket-api",status=~"5.."}[5m]))
            / sum(rate(http_requests_total{job="ticket-api"}[5m])) > 0.05
        for: 10m
        labels: { severity: page }
        annotations:
          summary: "More than 5% of ticket API requests fail"

      - alert: TicketApiSlow
        expr: |
          histogram_quantile(0.95, sum by (le) (rate(http_request_duration_seconds_bucket{job="ticket-api"}[5m]))) > 0.5
        for: 10m
        labels: { severity: ticket }
        annotations:
          summary: "p95 latency is above 500 ms"
@@file monitoring/grafana/datasources.yml
apiVersion: 1
datasources:
  - name: Prometheus
    uid: prometheus
    type: prometheus
    url: http://prometheus:9090
    isDefault: true
@@file monitoring/grafana/dashboards.yml
apiVersion: 1
providers:
  - name: files
    folder: Services
    type: file
    options:
      path: /var/lib/grafana/dashboards
@@file monitoring/grafana/dashboards/ticket-api.json
{
  "title": "Ticket API (RED)",
  "uid": "ticket-api",
  "schemaVersion": 39,
  "time": { "from": "now-1h", "to": "now" },
  "refresh": "30s",
  "panels": [
    {
      "type": "stat",
      "title": "Up",
      "gridPos": { "x": 0, "y": 0, "w": 4, "h": 4 },
      "datasource": { "type": "prometheus", "uid": "prometheus" },
      "targets": [{ "expr": "up{job=\"ticket-api\"}" }]
    },
    {
      "type": "timeseries",
      "title": "Rate: requests per second by route",
      "gridPos": { "x": 4, "y": 0, "w": 20, "h": 8 },
      "datasource": { "type": "prometheus", "uid": "prometheus" },
      "targets": [{ "expr": "sum by (route) (rate(http_requests_total{job=\"ticket-api\"}[5m]))", "legendFormat": "{{route}}" }]
    },
    {
      "type": "timeseries",
      "title": "Errors: share of 4xx and 5xx",
      "gridPos": { "x": 0, "y": 8, "w": 12, "h": 8 },
      "datasource": { "type": "prometheus", "uid": "prometheus" },
      "fieldConfig": { "defaults": { "unit": "percentunit" } },
      "targets": [
        { "expr": "sum(rate(http_requests_total{job=\"ticket-api\",status=~\"5..\"}[5m])) / sum(rate(http_requests_total{job=\"ticket-api\"}[5m]))", "legendFormat": "5xx" },
        { "expr": "sum(rate(http_requests_total{job=\"ticket-api\",status=~\"4..\"}[5m])) / sum(rate(http_requests_total{job=\"ticket-api\"}[5m]))", "legendFormat": "4xx" }
      ]
    },
    {
      "type": "timeseries",
      "title": "Duration: p50 and p95",
      "gridPos": { "x": 12, "y": 8, "w": 12, "h": 8 },
      "datasource": { "type": "prometheus", "uid": "prometheus" },
      "fieldConfig": { "defaults": { "unit": "s" } },
      "targets": [
        { "expr": "histogram_quantile(0.5, sum by (le) (rate(http_request_duration_seconds_bucket{job=\"ticket-api\"}[5m])))", "legendFormat": "p50" },
        { "expr": "histogram_quantile(0.95, sum by (le) (rate(http_request_duration_seconds_bucket{job=\"ticket-api\"}[5m])))", "legendFormat": "p95" }
      ]
    },
    {
      "type": "barchart",
      "title": "Tickets created by priority (last 24 h)",
      "gridPos": { "x": 0, "y": 16, "w": 24, "h": 7 },
      "datasource": { "type": "prometheus", "uid": "prometheus" },
      "targets": [{ "expr": "sum by (priority) (increase(tickets_created_total{job=\"ticket-api\"}[24h]))", "legendFormat": "{{priority}}", "instant": true }]
    }
  ]
}
@@file k8s/deployment.yaml
apiVersion: apps/v1
kind: Deployment
metadata:
  name: ticket-api
  labels: { app: ticket-api }
spec:
  replicas: 2
  selector:
    matchLabels: { app: ticket-api }
  strategy:
    rollingUpdate: { maxUnavailable: 0, maxSurge: 1 }   # never drop below 2 pods while rolling out
  template:
    metadata:
      labels: { app: ticket-api }
      annotations:
        prometheus.io/scrape: "true"                     # for annotation-based scraping
        prometheus.io/port: "8080"
    spec:
      automountServiceAccountToken: false
      securityContext:
        runAsNonRoot: true
        runAsUser: 10001
        seccompProfile: { type: RuntimeDefault }
      containers:
        - name: api
          image: ghcr.io/OWNER/ticket-api        # kustomization.yaml sets the tag
          ports: [{ name: http, containerPort: 8080 }]
          env:
            - { name: HOST, value: "0.0.0.0" }
            - { name: PORT, value: "8080" }
          resources:
            requests: { cpu: 50m, memory: 64Mi }
            limits: { memory: 128Mi }
          readinessProbe:
            httpGet: { path: /healthz, port: http }
            periodSeconds: 5
          livenessProbe:
            httpGet: { path: /healthz, port: http }
            periodSeconds: 15
            failureThreshold: 3
          securityContext:
            allowPrivilegeEscalation: false
            readOnlyRootFilesystem: true
            capabilities: { drop: [ALL] }
@@file k8s/service.yaml
apiVersion: v1
kind: Service
metadata:
  name: ticket-api
  labels: { app: ticket-api }
spec:
  selector: { app: ticket-api }
  ports:
    - { name: http, port: 80, targetPort: http }
@@file k8s/kustomization.yaml
# kubectl apply -k k8s/     (the pipeline sets the image tag with: kustomize edit set image ...)
apiVersion: kustomize.config.k8s.io/v1beta1
kind: Kustomization
namespace: tickets
resources:
  - deployment.yaml
  - service.yaml
images:
  - name: ghcr.io/OWNER/ticket-api
    newName: ghcr.io/OWNER/ticket-api
    newTag: "0.1.0"
@@file .github/workflows/ci.yml
name: CI
on:
  push: { branches: [main] }
  pull_request:

permissions:
  contents: read

concurrency:
  group: ci-${{ github.ref }}
  cancel-in-progress: true

jobs:
  test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4            # pin to a full SHA in production
      - uses: actions/setup-python@v5
        with: { python-version: "3.12" }
      - run: python -m unittest -v

  image:
    needs: test
    runs-on: ubuntu-latest
    permissions: { contents: read, packages: write }
    env:
      IMAGE: ghcr.io/${{ github.repository_owner }}/ticket-api
    steps:
      - uses: actions/checkout@v4
      - run: docker build -t "$IMAGE:${{ github.sha }}" .
      - name: Scan for vulnerabilities
        uses: aquasecurity/trivy-action@0.28.0
        with:
          image-ref: ${{ env.IMAGE }}:${{ github.sha }}
          severity: CRITICAL,HIGH
          ignore-unfixed: true
          exit-code: "1"
      - name: Smoke test the image
        run: |
          docker run -d --name api -p 8080:8080 "$IMAGE:${{ github.sha }}"
          for i in $(seq 1 20); do curl -fsS localhost:8080/healthz && break; sleep 1; done
          curl -fsS localhost:8080/metrics | grep -q http_requests_total
      - name: Push (main only)
        if: github.event_name == 'push'
        run: |
          echo "${{ secrets.GITHUB_TOKEN }}" | docker login ghcr.io -u "${{ github.actor }}" --password-stdin
          docker push "$IMAGE:${{ github.sha }}"

  manifests:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - name: Render the Kubernetes manifests
        run: kubectl kustomize k8s/ > rendered.yaml && cat rendered.yaml
@@text
## Stretch goals

- **Terraform** the registry and cluster: an ECR repository (or GHCR stays) and an EKS/AKS/GKE cluster with the modules from the Terraform track, planned on pull requests.
- **GitOps**: put `k8s/` in a separate config repository, install Argo CD, and let the pipeline open a PR that bumps `newTag` instead of deploying.
- **Alertmanager**: add it to Compose and route `severity: page` to a Slack webhook and `severity: ticket` to email.
- **An SLO**: 99.5% of requests succeed over 30 days. Add recording rules and multi-window burn-rate alerts (fast: 14.4× over 1 h, slow: 6× over 6 h).
- **A HorizontalPodAutoscaler** and a **PodDisruptionBudget**, then `kubectl drain` a node and watch the service stay up.
- **Signed images and an SBOM**: `cosign sign` and `syft` in the pipeline, and a Kyverno policy that only admits signed images.
- **Persistence**: swap `TicketStore` for PostgreSQL in Compose and a StatefulSet (or a managed database) in Kubernetes, without changing the API.
@@project ship_service_finished Ship a Service End to End: the finished version
A complete solution to compare with yours.
@@file metrics.py
"""Prometheus metrics with only the standard library: counters, histograms and the text format.

In a real service you'd use the prometheus-client package; writing it yourself shows exactly
what Prometheus reads from /metrics.
"""
import threading

INF = float("inf")


def format_value(v):
    """Prometheus writes +Inf, and whole numbers without a trailing .0."""
    if v == INF:
        return "+Inf"
    if float(v).is_integer():
        return str(int(v))
    return repr(float(v))


def format_labels(names, values):
    """('route', 'status'), ('/tickets', '200') -> '{route="/tickets",status="200"}'.

    No labels gives "". Inside a value, a backslash becomes \\\\, a double quote \\" and a
    newline \\n, so the line can always be parsed again.
    """
    if not names:
        return ""
    parts = []
    for name, value in zip(names, values):
        escaped = str(value).replace("\\", "\\\\").replace('"', '\\"').replace("\n", "\\n")
        parts.append(f'{name}="{escaped}"')
    return "{" + ",".join(parts) + "}"


class Counter:
    """A number that only goes up, one per combination of label values."""

    def __init__(self, name, help_text, labelnames=()):
        self.name, self.help, self.labelnames = name, help_text, tuple(labelnames)
        self._values = {}
        self._lock = threading.Lock()

    def inc(self, labels=(), amount=1):
        if amount < 0:
            raise ValueError("counters can only go up")
        labels = tuple(str(v) for v in labels)
        if len(labels) != len(self.labelnames):
            raise ValueError(f"{self.name} needs labels {self.labelnames}")
        with self._lock:
            self._values[labels] = self._values.get(labels, 0) + amount

    def value(self, labels=()):
        return self._values.get(tuple(str(v) for v in labels), 0)

    def render(self):
        """The HELP and TYPE lines, then one line per label combination, sorted by labels."""
        lines = [f"# HELP {self.name} {self.help}", f"# TYPE {self.name} counter"]
        with self._lock:
            for labels in sorted(self._values):
                lines.append(f"{self.name}{format_labels(self.labelnames, labels)} {format_value(self._values[labels])}")
        return "\n".join(lines)


class Histogram:
    """Counts observations into cumulative buckets, plus their sum and count."""

    DEFAULT_BUCKETS = (0.005, 0.01, 0.025, 0.05, 0.1, 0.25, 0.5, 1, 2.5, 5)

    def __init__(self, name, help_text, labelnames=(), buckets=DEFAULT_BUCKETS):
        self.name, self.help, self.labelnames = name, help_text, tuple(labelnames)
        self.buckets = tuple(sorted(buckets)) + (INF,)
        self._series = {}  # labels -> {"counts": [per bucket], "sum": float, "count": int}
        self._lock = threading.Lock()

    def observe(self, value, labels=()):
        """Adds one observation. Buckets are cumulative: a value counts in EVERY bucket whose
        upper bound (le) is >= the value, including +Inf."""
        labels = tuple(str(v) for v in labels)
        with self._lock:
            s = self._series.setdefault(labels, {"counts": [0] * len(self.buckets), "sum": 0.0, "count": 0})
            for i, le in enumerate(self.buckets):
                if value <= le:
                    s["counts"][i] += 1
            s["sum"] += value
            s["count"] += 1

    def render(self):
        """HELP and TYPE, then for each label combination (sorted): one _bucket line per bucket
        with an extra le label, then _sum and _count."""
        lines = [f"# HELP {self.name} {self.help}", f"# TYPE {self.name} histogram"]
        with self._lock:
            for labels in sorted(self._series):
                s = self._series[labels]
                for le, count in zip(self.buckets, s["counts"]):
                    lbl = format_labels(self.labelnames + ("le",), labels + (format_value(le),))
                    lines.append(f"{self.name}_bucket{lbl} {count}")
                base = format_labels(self.labelnames, labels)
                lines.append(f"{self.name}_sum{base} {format_value(round(s['sum'], 6))}")
                lines.append(f"{self.name}_count{base} {s['count']}")
        return "\n".join(lines)


class Registry:
    def __init__(self):
        self.metrics = []

    def register(self, metric):
        self.metrics.append(metric)
        return metric

    def render(self):
        """The whole /metrics page: every metric, separated by newlines, ending with one."""
        return "\n".join(m.render() for m in self.metrics) + "\n"
@@file service.py
"""The ticket API: storage, validation, HTTP handling and instrumentation."""
import json
import threading
import time
from http.server import BaseHTTPRequestHandler, ThreadingHTTPServer

from metrics import Counter, Histogram, Registry

PRIORITIES = ("low", "normal", "high", "urgent")


class TicketStore:
    """Tickets in memory. A real service would use a database; the API wouldn't change."""

    def __init__(self):
        self._tickets = []
        self._lock = threading.Lock()

    @staticmethod
    def validate(data):
        """Every problem with a new ticket, as a list of messages ([] when it's fine)."""
        if not isinstance(data, dict):
            return ["the body must be a JSON object"]
        problems = []
        title = data.get("title")
        if not isinstance(title, str) or not title.strip():
            problems.append("title is required")
        elif len(title.strip()) > 120:
            problems.append("title must be 120 characters or fewer")
        if data.get("priority", "normal") not in PRIORITIES:
            problems.append(f"priority must be one of: {', '.join(PRIORITIES)}")
        return problems

    def add(self, data):
        with self._lock:
            ticket = {"id": len(self._tickets) + 1, "title": data["title"].strip(),
                      "priority": data.get("priority", "normal"), "status": "open"}
            self._tickets.append(ticket)
            return ticket

    def list(self, priority=None):
        with self._lock:
            return [t for t in self._tickets if priority in (None, t["priority"])]


def make_server(host="127.0.0.1", port=8080, store=None, log=None):
    """A ready-to-run HTTP server with its own store and metrics (port 0 = any free port).
    `log` receives one JSON line per request (default: print it)."""
    log = log or (lambda line: print(line, flush=True))
    store = store or TicketStore()
    registry = Registry()
    requests_total = registry.register(Counter(
        "http_requests_total", "HTTP requests handled.", ["method", "route", "status"]))
    duration = registry.register(Histogram(
        "http_request_duration_seconds", "Time to handle a request.", ["route"],
        buckets=(0.005, 0.025, 0.1, 0.5, 2.5)))
    tickets_created = registry.register(Counter(
        "tickets_created_total", "Tickets created, by priority.", ["priority"]))

    class Handler(BaseHTTPRequestHandler):
        server_version = "TicketAPI/1.0"

        def log_message(self, fmt, *args):      # one structured line per request instead
            pass

        def _send(self, status, body, content_type="application/json"):
            data = body.encode() if isinstance(body, str) else json.dumps(body).encode()
            self.send_response(status)
            self.send_header("Content-Type", content_type)
            self.send_header("Content-Length", str(len(data)))
            self.end_headers()
            self.wfile.write(data)
            return status

        def _route(self):
            path = self.path.split("?")[0]
            # Known routes only, so a scanner hitting random URLs can't create endless series.
            return path if path in ("/healthz", "/metrics", "/tickets") else "other"

        def _handle(self, method):
            start = time.perf_counter()
            route = self._route()
            try:
                status = self._dispatch(method, route)
            except Exception:                    # never crash the server on one bad request
                status = self._send(500, {"error": "internal error"})
            requests_total.inc((method, route, status))
            duration.observe(time.perf_counter() - start, (route,))
            log(json.dumps({"method": method, "route": route, "status": status}))

        def _dispatch(self, method, route):
            if route == "/healthz" and method == "GET":
                return self._send(200, {"status": "ok"})
            if route == "/metrics" and method == "GET":
                return self._send(200, registry.render(), "text/plain; version=0.0.4")
            if route == "/tickets" and method == "GET":
                query = self.path.partition("?")[2]
                params = dict(p.split("=", 1) for p in query.split("&") if "=" in p)
                return self._send(200, store.list(params.get("priority")))
            if route == "/tickets" and method == "POST":
                length = int(self.headers.get("Content-Length") or 0)
                try:
                    data = json.loads(self.rfile.read(length) or b"null")
                except json.JSONDecodeError:
                    return self._send(400, {"error": "the body isn't valid JSON"})
                problems = TicketStore.validate(data)
                if problems:
                    return self._send(422, {"error": "invalid ticket", "problems": problems})
                ticket = store.add(data)
                tickets_created.inc((ticket["priority"],))
                return self._send(201, ticket)
            if route == "other":
                return self._send(404, {"error": "not found"})
            return self._send(405, {"error": "method not allowed"})

        def do_GET(self):
            self._handle("GET")

        def do_POST(self):
            self._handle("POST")

    server = ThreadingHTTPServer((host, port), Handler)
    server.registry = registry
    return server
@@file main.py
"""Starts the ticket API. Configuration comes from environment variables (twelve-factor)."""
import os
import signal
import sys

from service import make_server


def main():
    host = os.environ.get("HOST", "127.0.0.1")   # the container sets HOST=0.0.0.0
    port = int(os.environ.get("PORT", "8080"))
    server = make_server(host, port)

    # docker stop and Kubernetes send SIGTERM: finish cleanly instead of being killed.
    def stop(signum, frame):
        print('{"msg": "shutting down"}', flush=True)
        sys.exit(0)

    signal.signal(signal.SIGTERM, stop)
    print(f'{{"msg": "listening", "url": "http://{host}:{port}"}}', flush=True)
    try:
        server.serve_forever()
    except KeyboardInterrupt:
        pass
    finally:
        server.server_close()


if __name__ == "__main__":
    main()
@@file test_metrics.py
import unittest

from metrics import Counter, Histogram, Registry, format_labels, format_value


class FormatTest(unittest.TestCase):
    def test_values(self):
        self.assertEqual(format_value(3), "3")
        self.assertEqual(format_value(2.0), "2")
        self.assertEqual(format_value(0.25), "0.25")
        self.assertEqual(format_value(float("inf")), "+Inf")

    def test_labels(self):
        self.assertEqual(format_labels((), ()), "")
        self.assertEqual(format_labels(("route", "status"), ("/tickets", "200")), '{route="/tickets",status="200"}')

    def test_label_escaping(self):
        self.assertEqual(format_labels(("msg",), ('say "hi"\\n',)), '{msg="say \\"hi\\"\\\\n"}')
        self.assertEqual(format_labels(("msg",), ("two\nlines",)), '{msg="two\\nlines"}')


class CounterTest(unittest.TestCase):
    def test_render(self):
        c = Counter("http_requests_total", "Requests.", ["route", "status"])
        c.inc(("/tickets", 201))
        c.inc(("/healthz", 200), 2)
        c.inc(("/tickets", 201))
        self.assertEqual(c.render(), "\n".join([
            "# HELP http_requests_total Requests.",
            "# TYPE http_requests_total counter",
            'http_requests_total{route="/healthz",status="200"} 2',
            'http_requests_total{route="/tickets",status="201"} 2',
        ]))

    def test_no_labels_and_no_data(self):
        c = Counter("jobs_total", "Jobs.")
        self.assertEqual(c.render(), "# HELP jobs_total Jobs.\n# TYPE jobs_total counter")
        c.inc()
        self.assertTrue(c.render().endswith("\njobs_total 1"))

    def test_only_up(self):
        c = Counter("x_total", "X.")
        with self.assertRaises(ValueError):
            c.inc(amount=-1)
        with self.assertRaises(ValueError):
            c.inc(("unexpected",))


class HistogramTest(unittest.TestCase):
    def test_cumulative_buckets(self):
        h = Histogram("req_seconds", "Latency.", ["route"], buckets=(0.1, 0.5))
        for v in (0.05, 0.2, 0.3, 0.7):
            h.observe(v, ("/tickets",))
        self.assertEqual(h.render(), "\n".join([
            "# HELP req_seconds Latency.",
            "# TYPE req_seconds histogram",
            'req_seconds_bucket{route="/tickets",le="0.1"} 1',
            'req_seconds_bucket{route="/tickets",le="0.5"} 3',
            'req_seconds_bucket{route="/tickets",le="+Inf"} 4',
            'req_seconds_sum{route="/tickets"} 1.25',
            'req_seconds_count{route="/tickets"} 4',
        ]))

    def test_boundary_counts_in_its_bucket(self):
        h = Histogram("x", "X.", buckets=(1,))
        h.observe(1)
        self.assertIn('x_bucket{le="1"} 1', h.render())


class RegistryTest(unittest.TestCase):
    def test_page(self):
        r = Registry()
        r.register(Counter("a_total", "A.")).inc()
        r.register(Counter("b_total", "B."))
        page = r.render()
        self.assertTrue(page.endswith("\n"))
        self.assertIn("a_total 1\n# HELP b_total", page)


if __name__ == "__main__":
    unittest.main()
@@file test_service.py
import json
import threading
import unittest
import urllib.error
import urllib.request

from service import TicketStore, make_server


class ValidateTest(unittest.TestCase):
    def test_good(self):
        self.assertEqual(TicketStore.validate({"title": "Printer jammed", "priority": "high"}), [])
        self.assertEqual(TicketStore.validate({"title": "No priority given"}), [])

    def test_every_problem(self):
        self.assertEqual(TicketStore.validate({"title": "  ", "priority": "asap"}),
                         ["title is required", "priority must be one of: low, normal, high, urgent"])
        self.assertEqual(TicketStore.validate([]), ["the body must be a JSON object"])
        self.assertEqual(TicketStore.validate({"title": "x" * 121}), ["title must be 120 characters or fewer"])


class ApiTest(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.server = make_server(port=0, log=lambda line: None)   # any free port, quiet
        cls.base = f"http://127.0.0.1:{cls.server.server_address[1]}"
        cls.thread = threading.Thread(target=cls.server.serve_forever, daemon=True)
        cls.thread.start()

    @classmethod
    def tearDownClass(cls):
        cls.server.shutdown()
        cls.server.server_close()

    def call(self, method, path, body=None):
        data = None if body is None else (body if isinstance(body, bytes) else json.dumps(body).encode())
        req = urllib.request.Request(self.base + path, data=data, method=method)
        try:
            with urllib.request.urlopen(req) as resp:
                return resp.status, resp.read().decode()
        except urllib.error.HTTPError as e:
            return e.code, e.read().decode()

    def test_flow(self):
        status, body = self.call("GET", "/healthz")
        self.assertEqual((status, json.loads(body)), (200, {"status": "ok"}))

        status, body = self.call("POST", "/tickets", {"title": "VPN down", "priority": "urgent"})
        self.assertEqual(status, 201)
        self.assertEqual(json.loads(body)["title"], "VPN down")
        self.call("POST", "/tickets", {"title": "New mouse", "priority": "low"})

        status, body = self.call("GET", "/tickets?priority=urgent")
        self.assertEqual([t["title"] for t in json.loads(body)], ["VPN down"])

        self.assertEqual(self.call("POST", "/tickets", {"title": ""})[0], 422)
        self.assertEqual(self.call("POST", "/tickets", b"{not json")[0], 400)
        self.assertEqual(self.call("GET", "/nope")[0], 404)
        self.assertEqual(self.call("POST", "/healthz", {})[0], 405)

        status, page = self.call("GET", "/metrics")
        self.assertEqual(status, 200)
        self.assertIn('http_requests_total{method="POST",route="/tickets",status="201"} 2', page)
        self.assertIn('http_requests_total{method="GET",route="other",status="404"} 1', page)
        self.assertIn('tickets_created_total{priority="urgent"} 1', page)
        self.assertIn('http_request_duration_seconds_bucket{route="/tickets",le="+Inf"}', page)


if __name__ == "__main__":
    unittest.main()
@@file Dockerfile
# syntax=docker/dockerfile:1
# A small, non-root image. The app has no dependencies, so there is no install step; with a
# requirements.txt you would COPY it and `pip install --no-cache-dir -r` it BEFORE copying the code.
FROM python:3.12-slim

ENV PYTHONDONTWRITEBYTECODE=1 \
    PYTHONUNBUFFERED=1 \
    HOST=0.0.0.0 \
    PORT=8080

RUN useradd --system --uid 10001 --no-create-home app
WORKDIR /app
COPY metrics.py service.py main.py ./

USER 10001
EXPOSE 8080
HEALTHCHECK --interval=15s --timeout=3s --retries=3 \
  CMD python -c "import urllib.request; urllib.request.urlopen('http://127.0.0.1:8080/healthz', timeout=2)"
ENTRYPOINT ["python", "main.py"]
@@file .dockerignore
.git
.github
__pycache__/
*.pyc
test_*.py
k8s/
monitoring/
compose.yaml
Makefile
.env
@@file compose.yaml
# The service and its monitoring stack on one machine:
#   docker compose up -d --build
#   API http://localhost:8080   Prometheus http://localhost:9090   Grafana http://localhost:3000 (admin/admin)
services:
  api:
    build: .
    image: ticket-api:dev
    ports: ["8080:8080"]
    restart: unless-stopped
    read_only: true
    cap_drop: [ALL]
    security_opt: ["no-new-privileges:true"]

  prometheus:
    image: prom/prometheus:v3.0.1
    command: ["--config.file=/etc/prometheus/prometheus.yml", "--storage.tsdb.retention.time=7d"]
    volumes:
      - ./monitoring/prometheus.yml:/etc/prometheus/prometheus.yml:ro
      - ./monitoring/alerts.yml:/etc/prometheus/alerts.yml:ro
      - promdata:/prometheus
    ports: ["9090:9090"]
    depends_on: [api]

  grafana:
    image: grafana/grafana:11.3.0
    environment:
      GF_AUTH_ANONYMOUS_ENABLED: "true"        # read-only viewing without logging in (local use only)
      GF_AUTH_ANONYMOUS_ORG_ROLE: Viewer
    volumes:
      - ./monitoring/grafana/datasources.yml:/etc/grafana/provisioning/datasources/datasources.yml:ro
      - ./monitoring/grafana/dashboards.yml:/etc/grafana/provisioning/dashboards/dashboards.yml:ro
      - ./monitoring/grafana/dashboards:/var/lib/grafana/dashboards:ro
    ports: ["3000:3000"]
    depends_on: [prometheus]

volumes:
  promdata:
@@file Makefile
.PHONY: test run image up down load

test:            ## run the unit tests
	python -m unittest -v

run:             ## run the API locally
	python main.py

image:           ## build the container image
	docker build -t ticket-api:dev .

up:              ## start the API with Prometheus and Grafana
	docker compose up -d --build

down:
	docker compose down

load:            ## send some traffic so the dashboards have something to show
	for i in $$(seq 1 50); do \
	  curl -s -X POST localhost:8080/tickets -d '{"title":"Load test","priority":"low"}' > /dev/null; \
	  curl -s localhost:8080/tickets > /dev/null; curl -s localhost:8080/nope > /dev/null; \
	done
@@file monitoring/prometheus.yml
global:
  scrape_interval: 15s
  evaluation_interval: 15s

rule_files: ["/etc/prometheus/alerts.yml"]

scrape_configs:
  - job_name: ticket-api
    static_configs:
      - targets: ["api:8080"]
        labels: { env: dev }
@@file monitoring/alerts.yml
groups:
  - name: ticket-api
    rules:
      - alert: TicketApiDown
        expr: up{job="ticket-api"} == 0
        for: 1m
        labels: { severity: page }
        annotations:
          summary: "The ticket API can't be scraped"

      - alert: TicketApiHighErrorRate
        expr: |
          sum(rate(http_requests_total{job="ticket-api",status=~"5.."}[5m]))
            / sum(rate(http_requests_total{job="ticket-api"}[5m])) > 0.05
        for: 10m
        labels: { severity: page }
        annotations:
          summary: "More than 5% of ticket API requests fail"

      - alert: TicketApiSlow
        expr: |
          histogram_quantile(0.95, sum by (le) (rate(http_request_duration_seconds_bucket{job="ticket-api"}[5m]))) > 0.5
        for: 10m
        labels: { severity: ticket }
        annotations:
          summary: "p95 latency is above 500 ms"
@@file monitoring/grafana/datasources.yml
apiVersion: 1
datasources:
  - name: Prometheus
    uid: prometheus
    type: prometheus
    url: http://prometheus:9090
    isDefault: true
@@file monitoring/grafana/dashboards.yml
apiVersion: 1
providers:
  - name: files
    folder: Services
    type: file
    options:
      path: /var/lib/grafana/dashboards
@@file monitoring/grafana/dashboards/ticket-api.json
{
  "title": "Ticket API (RED)",
  "uid": "ticket-api",
  "schemaVersion": 39,
  "time": { "from": "now-1h", "to": "now" },
  "refresh": "30s",
  "panels": [
    {
      "type": "stat",
      "title": "Up",
      "gridPos": { "x": 0, "y": 0, "w": 4, "h": 4 },
      "datasource": { "type": "prometheus", "uid": "prometheus" },
      "targets": [{ "expr": "up{job=\"ticket-api\"}" }]
    },
    {
      "type": "timeseries",
      "title": "Rate: requests per second by route",
      "gridPos": { "x": 4, "y": 0, "w": 20, "h": 8 },
      "datasource": { "type": "prometheus", "uid": "prometheus" },
      "targets": [{ "expr": "sum by (route) (rate(http_requests_total{job=\"ticket-api\"}[5m]))", "legendFormat": "{{route}}" }]
    },
    {
      "type": "timeseries",
      "title": "Errors: share of 4xx and 5xx",
      "gridPos": { "x": 0, "y": 8, "w": 12, "h": 8 },
      "datasource": { "type": "prometheus", "uid": "prometheus" },
      "fieldConfig": { "defaults": { "unit": "percentunit" } },
      "targets": [
        { "expr": "sum(rate(http_requests_total{job=\"ticket-api\",status=~\"5..\"}[5m])) / sum(rate(http_requests_total{job=\"ticket-api\"}[5m]))", "legendFormat": "5xx" },
        { "expr": "sum(rate(http_requests_total{job=\"ticket-api\",status=~\"4..\"}[5m])) / sum(rate(http_requests_total{job=\"ticket-api\"}[5m]))", "legendFormat": "4xx" }
      ]
    },
    {
      "type": "timeseries",
      "title": "Duration: p50 and p95",
      "gridPos": { "x": 12, "y": 8, "w": 12, "h": 8 },
      "datasource": { "type": "prometheus", "uid": "prometheus" },
      "fieldConfig": { "defaults": { "unit": "s" } },
      "targets": [
        { "expr": "histogram_quantile(0.5, sum by (le) (rate(http_request_duration_seconds_bucket{job=\"ticket-api\"}[5m])))", "legendFormat": "p50" },
        { "expr": "histogram_quantile(0.95, sum by (le) (rate(http_request_duration_seconds_bucket{job=\"ticket-api\"}[5m])))", "legendFormat": "p95" }
      ]
    },
    {
      "type": "barchart",
      "title": "Tickets created by priority (last 24 h)",
      "gridPos": { "x": 0, "y": 16, "w": 24, "h": 7 },
      "datasource": { "type": "prometheus", "uid": "prometheus" },
      "targets": [{ "expr": "sum by (priority) (increase(tickets_created_total{job=\"ticket-api\"}[24h]))", "legendFormat": "{{priority}}", "instant": true }]
    }
  ]
}
@@file k8s/deployment.yaml
apiVersion: apps/v1
kind: Deployment
metadata:
  name: ticket-api
  labels: { app: ticket-api }
spec:
  replicas: 2
  selector:
    matchLabels: { app: ticket-api }
  strategy:
    rollingUpdate: { maxUnavailable: 0, maxSurge: 1 }   # never drop below 2 pods while rolling out
  template:
    metadata:
      labels: { app: ticket-api }
      annotations:
        prometheus.io/scrape: "true"                     # for annotation-based scraping
        prometheus.io/port: "8080"
    spec:
      automountServiceAccountToken: false
      securityContext:
        runAsNonRoot: true
        runAsUser: 10001
        seccompProfile: { type: RuntimeDefault }
      containers:
        - name: api
          image: ghcr.io/OWNER/ticket-api        # kustomization.yaml sets the tag
          ports: [{ name: http, containerPort: 8080 }]
          env:
            - { name: HOST, value: "0.0.0.0" }
            - { name: PORT, value: "8080" }
          resources:
            requests: { cpu: 50m, memory: 64Mi }
            limits: { memory: 128Mi }
          readinessProbe:
            httpGet: { path: /healthz, port: http }
            periodSeconds: 5
          livenessProbe:
            httpGet: { path: /healthz, port: http }
            periodSeconds: 15
            failureThreshold: 3
          securityContext:
            allowPrivilegeEscalation: false
            readOnlyRootFilesystem: true
            capabilities: { drop: [ALL] }
@@file k8s/service.yaml
apiVersion: v1
kind: Service
metadata:
  name: ticket-api
  labels: { app: ticket-api }
spec:
  selector: { app: ticket-api }
  ports:
    - { name: http, port: 80, targetPort: http }
@@file k8s/kustomization.yaml
# kubectl apply -k k8s/     (the pipeline sets the image tag with: kustomize edit set image ...)
apiVersion: kustomize.config.k8s.io/v1beta1
kind: Kustomization
namespace: tickets
resources:
  - deployment.yaml
  - service.yaml
images:
  - name: ghcr.io/OWNER/ticket-api
    newName: ghcr.io/OWNER/ticket-api
    newTag: "0.1.0"
@@file .github/workflows/ci.yml
name: CI
on:
  push: { branches: [main] }
  pull_request:

permissions:
  contents: read

concurrency:
  group: ci-${{ github.ref }}
  cancel-in-progress: true

jobs:
  test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4            # pin to a full SHA in production
      - uses: actions/setup-python@v5
        with: { python-version: "3.12" }
      - run: python -m unittest -v

  image:
    needs: test
    runs-on: ubuntu-latest
    permissions: { contents: read, packages: write }
    env:
      IMAGE: ghcr.io/${{ github.repository_owner }}/ticket-api
    steps:
      - uses: actions/checkout@v4
      - run: docker build -t "$IMAGE:${{ github.sha }}" .
      - name: Scan for vulnerabilities
        uses: aquasecurity/trivy-action@0.28.0
        with:
          image-ref: ${{ env.IMAGE }}:${{ github.sha }}
          severity: CRITICAL,HIGH
          ignore-unfixed: true
          exit-code: "1"
      - name: Smoke test the image
        run: |
          docker run -d --name api -p 8080:8080 "$IMAGE:${{ github.sha }}"
          for i in $(seq 1 20); do curl -fsS localhost:8080/healthz && break; sleep 1; done
          curl -fsS localhost:8080/metrics | grep -q http_requests_total
      - name: Push (main only)
        if: github.event_name == 'push'
        run: |
          echo "${{ secrets.GITHUB_TOKEN }}" | docker login ghcr.io -u "${{ github.actor }}" --password-stdin
          docker push "$IMAGE:${{ github.sha }}"

  manifests:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - name: Render the Kubernetes manifests
        run: kubectl kustomize k8s/ > rendered.yaml && cat rendered.yaml

@@drill
Which endpoint does an orchestrator call to check a service is healthy, by convention?
= /health
= /healthz
? Return 200 when it can serve.

@@text
🚀 Tests, an image, a pipeline, manifests, metrics, alerts and dashboards, all versioned in one repository: that's what "you build it, you run it" looks like, and it's exactly what DevOps interviews ask you to talk through.
