@@lesson cl9-containerize Containerize a Status Board
@@text
Every IT team wants one page that answers *"is it us or is it them?"* You'll build a **status board**: a tiny web service that checks whether internal services answer on their ports and shows **ok / degraded / down**. Then you'll take it all the way: **Docker image → Docker Compose → Kubernetes**.

```text
           ┌──────────────┐  TCP check   ┌─────┐
browser ──▶│ statusboard  │─────────────▶│ web │ nginx
           │  :8080       │─────────────▶│cache│ redis
           └──────────────┘─────────────▶ printer 10.0.0.250:9100 (doesn't exist: "degraded")
```

The app is written the cloud-native way:

- **configured only through environment variables** (`TARGETS`, `PORT`, `TIMEOUT`, `TITLE`), so the same image runs everywhere, configured by Compose's `environment:` or a Kubernetes **ConfigMap**;
- **logs to stdout**, has a **`/healthz`** endpoint for Docker's `HEALTHCHECK` and Kubernetes' probes, and a JSON API at `/api/status`;
- **escapes everything** it puts in HTML, because config values are input too;
- has its network call **injectable** (`connect=`), so the tests fake the network and run instantly.

No packages to install: only the Python standard library.
@@project statusboard Containerize a Status Board
Write `load_config`, `check_target`, `summarize` and `render_html` in `app.py` until `python -m unittest` passes. Then follow `LAB.md`: run it, build the image, run three containers with Compose, and deploy to a local Kubernetes cluster with kind.
@@file app.py
"""Status board: a tiny web service that shows whether internal services are reachable.

Configured only through environment variables (twelve-factor style), so the same image
runs on your laptop, in Docker Compose and in Kubernetes:

    TARGETS  "web=web:80,cache=cache:6379"   name=host:port, comma separated
    PORT     port to listen on (default 8080)
    TIMEOUT  seconds per check (default 1.0)
    TITLE    page title (default "Status Board")
"""
import html
import json
import os
import socket
import time
from http.server import BaseHTTPRequestHandler, ThreadingHTTPServer


def load_config(env):
    """Read the settings from a mapping like os.environ. Raise ValueError on bad input.

    Returns {"title": str, "port": int, "timeout": float, "targets": [(name, host, port), ...]}.
    Skip empty entries and surrounding spaces. Split host and port at the LAST colon.
    Bad: missing "=" or ":", empty name or host, non-numeric port, port outside 1-65535,
    duplicate names, PORT not an integer, TIMEOUT not a number or <= 0.
    """
    raise NotImplementedError


def check_target(host, port, timeout, connect=socket.create_connection):
    """Try a TCP connection. Return (ok, milliseconds or None, error message or None).

    Call connect((host, port), timeout=timeout) and close what it returns. On OSError return
    (False, None, the error's message, or its class name if the message is empty).
    """
    raise NotImplementedError


def summarize(results):
    """results: list of dicts {name, ok, ms, error}. Overall status: ok / degraded / down.

    Returns {"status": ..., "up": count, "total": count, "services": results}.
    "ok" when everything is up (also when there's nothing to check), "down" when nothing is.
    """
    raise NotImplementedError


def render_html(title, summary):
    """A small self-contained page. Every value from config is escaped (html.escape).

    Must contain "<title>: <status>" in an <h1>, "<up> of <total> services up", and a table row per
    service showing "<ms> ms" when up or the error when down.
    """
    raise NotImplementedError


def run_checks(config):
    results = []
    for name, host, port in config["targets"]:
        ok, ms, error = check_target(host, port, config["timeout"])
        results.append({"name": name, "ok": ok, "ms": ms, "error": error})
    return summarize(results)


def make_handler(config):
    class Handler(BaseHTTPRequestHandler):
        def _send(self, code, body, content_type):
            data = body.encode("utf-8")
            self.send_response(code)
            self.send_header("Content-Type", content_type)
            self.send_header("Content-Length", str(len(data)))
            self.end_headers()
            self.wfile.write(data)

        def do_GET(self):
            if self.path == "/healthz":          # liveness: the process answers
                self._send(200, "ok\n", "text/plain")
            elif self.path == "/api/status":
                summary = run_checks(config)
                self._send(200, json.dumps(summary, indent=2), "application/json")
            elif self.path == "/":
                self._send(200, render_html(config["title"], run_checks(config)), "text/html; charset=utf-8")
            else:
                self._send(404, "not found\n", "text/plain")

        def log_message(self, fmt, *args):     # one line per request to stdout: containers log to stdout
            print(f"{self.address_string()} {fmt % args}", flush=True)

    return Handler


def main():
    config = load_config(os.environ)
    print(f"listening on :{config['port']}, checking {len(config['targets'])} target(s)", flush=True)
    ThreadingHTTPServer(("0.0.0.0", config["port"]), make_handler(config)).serve_forever()


if __name__ == "__main__":
    main()
@@file test_app.py
import unittest

from app import check_target, load_config, render_html, summarize


class FakeConn:
    closed = False

    def close(self):
        self.closed = True


class ConfigTests(unittest.TestCase):
    def test_defaults(self):
        c = load_config({})
        self.assertEqual(c, {"title": "Status Board", "port": 8080, "timeout": 1.0, "targets": []})

    def test_targets(self):
        c = load_config({"TARGETS": " web=web:80, cache = cache:6379 ,", "PORT": "9000", "TIMEOUT": "0.5"})
        self.assertEqual(c["targets"], [("web", "web", 80), ("cache", "cache", 6379)])
        self.assertEqual((c["port"], c["timeout"]), (9000, 0.5))

    def test_ipv6_style_host_keeps_colons(self):
        self.assertEqual(load_config({"TARGETS": "dns=::1:53"})["targets"], [("dns", "::1", 53)])

    def test_bad_targets(self):
        for bad in ["web", "web=web", "web=web:http", "=web:80", "web=:80", "web=web:0", "web=web:70000"]:
            with self.subTest(bad=bad), self.assertRaises(ValueError):
                load_config({"TARGETS": bad})

    def test_duplicate_names(self):
        with self.assertRaises(ValueError):
            load_config({"TARGETS": "a=x:1,a=y:2"})

    def test_bad_numbers(self):
        for env in [{"PORT": "eighty"}, {"TIMEOUT": "fast"}, {"TIMEOUT": "0"}]:
            with self.subTest(env=env), self.assertRaises(ValueError):
                load_config(env)


class CheckTests(unittest.TestCase):
    def test_up(self):
        conn = FakeConn()
        calls = []

        def connect(addr, timeout):
            calls.append((addr, timeout))
            return conn

        ok, ms, error = check_target("web", 80, 0.5, connect=connect)
        self.assertTrue(ok)
        self.assertIsInstance(ms, float)
        self.assertIsNone(error)
        self.assertTrue(conn.closed, "close the connection")
        self.assertEqual(calls, [(("web", 80), 0.5)])

    def test_down(self):
        def connect(addr, timeout):
            raise ConnectionRefusedError("connection refused")

        self.assertEqual(check_target("db", 5432, 1, connect=connect), (False, None, "connection refused"))

    def test_down_without_message(self):
        def connect(addr, timeout):
            raise TimeoutError()

        self.assertEqual(check_target("db", 5432, 1, connect=connect), (False, None, "TimeoutError"))


def r(name, ok):
    return {"name": name, "ok": ok, "ms": 1.0 if ok else None, "error": None if ok else "refused"}


class SummaryTests(unittest.TestCase):
    def test_states(self):
        self.assertEqual(summarize([r("a", True), r("b", True)])["status"], "ok")
        self.assertEqual(summarize([r("a", True), r("b", False)])["status"], "degraded")
        self.assertEqual(summarize([r("a", False)])["status"], "down")
        self.assertEqual(summarize([])["status"], "ok")

    def test_counts(self):
        s = summarize([r("a", True), r("b", False), r("c", True)])
        self.assertEqual((s["up"], s["total"]), (2, 3))
        self.assertEqual([x["name"] for x in s["services"]], ["a", "b", "c"])

    def test_html_escapes(self):
        page = render_html("Ops <Board>", summarize([r("<script>", False), r("web", True)]))
        self.assertIn("Ops &lt;Board&gt;: degraded", page)
        self.assertIn("&lt;script&gt;", page)
        self.assertNotIn("<script>", page)
        self.assertIn("1 of 2 services up", page)
        self.assertIn("1.0 ms", page)


if __name__ == "__main__":
    unittest.main()
@@file Dockerfile
# Small official base image, pinned to a minor version
FROM python:3.12-slim

# No .pyc files, and logs appear immediately (containers log to stdout)
ENV PYTHONDONTWRITEBYTECODE=1 PYTHONUNBUFFERED=1

WORKDIR /app
COPY app.py .

# Never run as root inside the container
RUN useradd --create-home --uid 10001 appuser
USER appuser

EXPOSE 8080
HEALTHCHECK --interval=30s --timeout=3s --retries=3 \
  CMD python -c "import urllib.request; urllib.request.urlopen('http://localhost:8080/healthz', timeout=2)"

CMD ["python", "app.py"]
@@file .dockerignore
__pycache__/
*.pyc
test_*.py
k8s/
LAB.md
.git/
@@file compose.yaml
# docker compose up --build    then open http://localhost:8080
services:
  statusboard:
    build: .
    ports:
      - "8080:8080"
    environment:
      TITLE: "Office Services"
      TARGETS: "web=web:80,cache=cache:6379,printer=10.0.0.250:9100"
      TIMEOUT: "0.5"
    depends_on: [web, cache]
    restart: unless-stopped

  web:
    image: nginx:1.27-alpine

  cache:
    image: redis:7-alpine
@@file k8s/configmap.yaml
apiVersion: v1
kind: ConfigMap
metadata:
  name: statusboard-config
data:
  TITLE: "Cluster Services"
  TARGETS: "kube-dns=kube-dns.kube-system:53,web=web:80"
  TIMEOUT: "0.5"
@@file k8s/deployment.yaml
apiVersion: apps/v1
kind: Deployment
metadata:
  name: statusboard
  labels: { app: statusboard }
spec:
  replicas: 2
  selector:
    matchLabels: { app: statusboard }
  template:
    metadata:
      labels: { app: statusboard }
    spec:
      securityContext:
        runAsNonRoot: true
      containers:
        - name: statusboard
          image: statusboard:1.0
          imagePullPolicy: IfNotPresent
          ports:
            - containerPort: 8080
          envFrom:
            - configMapRef: { name: statusboard-config }
          resources:
            requests: { cpu: 50m, memory: 32Mi }
            limits: { cpu: 200m, memory: 64Mi }
          livenessProbe:
            httpGet: { path: /healthz, port: 8080 }
            periodSeconds: 10
          readinessProbe:
            httpGet: { path: /healthz, port: 8080 }
            periodSeconds: 5
          securityContext:
            allowPrivilegeEscalation: false
            readOnlyRootFilesystem: true
@@file k8s/service.yaml
apiVersion: v1
kind: Service
metadata:
  name: statusboard
spec:
  type: NodePort
  selector: { app: statusboard }
  ports:
    - port: 80
      targetPort: 8080
      nodePort: 30080
---
# Something for the status board to check
apiVersion: apps/v1
kind: Deployment
metadata:
  name: web
spec:
  replicas: 1
  selector:
    matchLabels: { app: web }
  template:
    metadata:
      labels: { app: web }
    spec:
      containers:
        - name: nginx
          image: nginx:1.27-alpine
          ports:
            - containerPort: 80
---
apiVersion: v1
kind: Service
metadata:
  name: web
spec:
  selector: { app: web }
  ports:
    - port: 80
@@file LAB.md
# Containerize the Status Board

You need **Docker Desktop** (or Docker Engine on Linux / WSL 2). For part 3, also **kind**
(`winget install Kubernetes.kind`) or **minikube**, and **kubectl**.

## 1. Run it without Docker
```
python -m unittest
set TARGETS=google=8.8.8.8:53,nothing=127.0.0.1:1     (PowerShell: $env:TARGETS="...")
python app.py
```
Open http://localhost:8080 and http://localhost:8080/api/status. Stop with Ctrl+C.

## 2. Docker
```
docker build -t statusboard:1.0 .
docker images statusboard
docker run --rm -p 8080:8080 -e TARGETS=dns=8.8.8.8:53 statusboard:1.0
docker ps                       (in another terminal: STATUS shows "healthy" after ~30 s)
docker exec -it <id> whoami     (appuser, not root)
docker history statusboard:1.0  (the layers)
```
Change `app.py` (the title colour, say) and build again: only the layers after `COPY` rebuild.

## 3. Docker Compose: three containers and a network
```
docker compose up --build -d
docker compose ps
docker compose logs -f statusboard
```
The board shows **degraded**: the printer address doesn't exist. Now:
```
docker compose stop cache      (refresh the page: cache is down)
docker compose start cache
docker compose down
```
Containers find each other by **service name** (`web`, `cache`): that's Compose's built-in DNS.

## 4. Kubernetes (kind)
```
kind create cluster --name lab
kind load docker-image statusboard:1.0 --name lab
kubectl apply -f k8s/
kubectl get pods,svc,deploy -o wide
kubectl port-forward svc/statusboard 8080:80
```
Then break things and watch Kubernetes fix them:
```
kubectl delete pod -l app=statusboard      (new pods appear: the Deployment keeps 2 replicas)
kubectl scale deployment statusboard --replicas=4
kubectl scale deployment web --replicas=0  (board: web down)  then back to 1
kubectl edit configmap statusboard-config ; kubectl rollout restart deployment statusboard
kubectl rollout status deployment statusboard ; kubectl rollout history deployment statusboard
kubectl logs -l app=statusboard --tail=20
kubectl describe pod <name>                (events: scheduling, probes, restarts)
```
Clean up: `kind delete cluster --name lab`.

## 5. Ideas
- Push the image to a registry (Docker Hub, GitHub Container Registry, ECR, ACR).
- Run it on a managed service: AWS App Runner / ECS Fargate, Azure Container Apps, Google Cloud Run.
- Scan the image: `docker scout cves statusboard:1.0` or `trivy image statusboard:1.0`.
@@text
> Tips: `entry.partition("=")` and `address.rpartition(":")` split at the first `=` and the **last** `:` (so `::1:53` works). `str.isdigit()` checks the port. Time a call with `time.perf_counter()` before and after. `html.escape()` every string you put in the page.
@@project statusboard_finished Containerize a Status Board: the finished version
The complete project. Next steps: add HTTP checks (status code, not just an open port), keep a history and show uptime percentages, expose `/metrics` for Prometheus, or deploy it to Azure Container Apps / AWS App Runner from a registry.
@@file app.py
"""Status board: a tiny web service that shows whether internal services are reachable.

Configured only through environment variables (twelve-factor style), so the same image
runs on your laptop, in Docker Compose and in Kubernetes:

    TARGETS  "web=web:80,cache=cache:6379"   name=host:port, comma separated
    PORT     port to listen on (default 8080)
    TIMEOUT  seconds per check (default 1.0)
    TITLE    page title (default "Status Board")
"""
import html
import json
import os
import socket
import time
from http.server import BaseHTTPRequestHandler, ThreadingHTTPServer


def load_config(env):
    """Read the settings from a mapping like os.environ. Raise ValueError on bad input.

    Returns {"title": str, "port": int, "timeout": float, "targets": [(name, host, port), ...]}.
    Skip empty entries and surrounding spaces. Split host and port at the LAST colon.
    Bad: missing "=" or ":", empty name or host, non-numeric port, port outside 1-65535,
    duplicate names, PORT not an integer, TIMEOUT not a number or <= 0.
    """
    targets = []
    for entry in env.get("TARGETS", "").split(","):
        entry = entry.strip()
        if not entry:
            continue
        name, sep, address = entry.partition("=")
        host, colon, port = address.rpartition(":")
        if not sep or not colon or not name.strip() or not host.strip() or not port.isdigit():
            raise ValueError(f"bad target {entry!r}: expected name=host:port")
        if not 1 <= int(port) <= 65535:
            raise ValueError(f"bad port in {entry!r}")
        targets.append((name.strip(), host.strip(), int(port)))

    names = [t[0] for t in targets]
    if len(names) != len(set(names)):
        raise ValueError("target names must be unique")

    try:
        port = int(env.get("PORT", "8080"))
        timeout = float(env.get("TIMEOUT", "1.0"))
    except ValueError:
        raise ValueError("PORT must be an integer and TIMEOUT a number") from None
    if timeout <= 0:
        raise ValueError("TIMEOUT must be positive")

    return {
        "title": env.get("TITLE", "Status Board"),
        "port": port,
        "timeout": timeout,
        "targets": targets,
    }


def check_target(host, port, timeout, connect=socket.create_connection):
    """Try a TCP connection. Return (ok, milliseconds or None, error message or None).

    Call connect((host, port), timeout=timeout) and close what it returns. On OSError return
    (False, None, the error's message, or its class name if the message is empty).
    """
    start = time.perf_counter()
    try:
        conn = connect((host, port), timeout=timeout)
    except OSError as e:
        return False, None, str(e) or type(e).__name__
    conn.close()
    return True, round((time.perf_counter() - start) * 1000, 1), None


def summarize(results):
    """results: list of dicts {name, ok, ms, error}. Overall status: ok / degraded / down.

    Returns {"status": ..., "up": count, "total": count, "services": results}.
    "ok" when everything is up (also when there's nothing to check), "down" when nothing is.
    """
    up = sum(1 for r in results if r["ok"])
    total = len(results)
    if up == total:
        status = "ok"
    elif up == 0:
        status = "down"
    else:
        status = "degraded"
    return {"status": status, "up": up, "total": total, "services": list(results)}


def render_html(title, summary):
    """A small self-contained page. Every value from config is escaped (html.escape).

    Must contain "<title>: <status>" in an <h1>, "<up> of <total> services up", and a table row per
    service showing "<ms> ms" when up or the error when down.
    """
    rows = []
    for s in summary["services"]:
        state = "up" if s["ok"] else "down"
        detail = f'{s["ms"]} ms' if s["ok"] else html.escape(s["error"] or "")
        rows.append(f'<tr class="{state}"><td>{html.escape(s["name"])}</td><td>{state}</td><td>{detail}</td></tr>')
    return (
        "<!doctype html><html><head><meta charset=\"utf-8\">"
        f"<title>{html.escape(title)}</title>"
        "<style>body{font-family:sans-serif;margin:2rem}.up td{color:green}.down td{color:#b00}"
        "td{padding:.3rem 1rem}</style></head><body>"
        f"<h1>{html.escape(title)}: {summary['status']}</h1>"
        f"<p>{summary['up']} of {summary['total']} services up</p>"
        f"<table>{''.join(rows)}</table></body></html>"
    )


def run_checks(config):
    results = []
    for name, host, port in config["targets"]:
        ok, ms, error = check_target(host, port, config["timeout"])
        results.append({"name": name, "ok": ok, "ms": ms, "error": error})
    return summarize(results)


def make_handler(config):
    class Handler(BaseHTTPRequestHandler):
        def _send(self, code, body, content_type):
            data = body.encode("utf-8")
            self.send_response(code)
            self.send_header("Content-Type", content_type)
            self.send_header("Content-Length", str(len(data)))
            self.end_headers()
            self.wfile.write(data)

        def do_GET(self):
            if self.path == "/healthz":          # liveness: the process answers
                self._send(200, "ok\n", "text/plain")
            elif self.path == "/api/status":
                summary = run_checks(config)
                self._send(200, json.dumps(summary, indent=2), "application/json")
            elif self.path == "/":
                self._send(200, render_html(config["title"], run_checks(config)), "text/html; charset=utf-8")
            else:
                self._send(404, "not found\n", "text/plain")

        def log_message(self, fmt, *args):     # one line per request to stdout: containers log to stdout
            print(f"{self.address_string()} {fmt % args}", flush=True)

    return Handler


def main():
    config = load_config(os.environ)
    print(f"listening on :{config['port']}, checking {len(config['targets'])} target(s)", flush=True)
    ThreadingHTTPServer(("0.0.0.0", config["port"]), make_handler(config)).serve_forever()


if __name__ == "__main__":
    main()
@@file test_app.py
import unittest

from app import check_target, load_config, render_html, summarize


class FakeConn:
    closed = False

    def close(self):
        self.closed = True


class ConfigTests(unittest.TestCase):
    def test_defaults(self):
        c = load_config({})
        self.assertEqual(c, {"title": "Status Board", "port": 8080, "timeout": 1.0, "targets": []})

    def test_targets(self):
        c = load_config({"TARGETS": " web=web:80, cache = cache:6379 ,", "PORT": "9000", "TIMEOUT": "0.5"})
        self.assertEqual(c["targets"], [("web", "web", 80), ("cache", "cache", 6379)])
        self.assertEqual((c["port"], c["timeout"]), (9000, 0.5))

    def test_ipv6_style_host_keeps_colons(self):
        self.assertEqual(load_config({"TARGETS": "dns=::1:53"})["targets"], [("dns", "::1", 53)])

    def test_bad_targets(self):
        for bad in ["web", "web=web", "web=web:http", "=web:80", "web=:80", "web=web:0", "web=web:70000"]:
            with self.subTest(bad=bad), self.assertRaises(ValueError):
                load_config({"TARGETS": bad})

    def test_duplicate_names(self):
        with self.assertRaises(ValueError):
            load_config({"TARGETS": "a=x:1,a=y:2"})

    def test_bad_numbers(self):
        for env in [{"PORT": "eighty"}, {"TIMEOUT": "fast"}, {"TIMEOUT": "0"}]:
            with self.subTest(env=env), self.assertRaises(ValueError):
                load_config(env)


class CheckTests(unittest.TestCase):
    def test_up(self):
        conn = FakeConn()
        calls = []

        def connect(addr, timeout):
            calls.append((addr, timeout))
            return conn

        ok, ms, error = check_target("web", 80, 0.5, connect=connect)
        self.assertTrue(ok)
        self.assertIsInstance(ms, float)
        self.assertIsNone(error)
        self.assertTrue(conn.closed, "close the connection")
        self.assertEqual(calls, [(("web", 80), 0.5)])

    def test_down(self):
        def connect(addr, timeout):
            raise ConnectionRefusedError("connection refused")

        self.assertEqual(check_target("db", 5432, 1, connect=connect), (False, None, "connection refused"))

    def test_down_without_message(self):
        def connect(addr, timeout):
            raise TimeoutError()

        self.assertEqual(check_target("db", 5432, 1, connect=connect), (False, None, "TimeoutError"))


def r(name, ok):
    return {"name": name, "ok": ok, "ms": 1.0 if ok else None, "error": None if ok else "refused"}


class SummaryTests(unittest.TestCase):
    def test_states(self):
        self.assertEqual(summarize([r("a", True), r("b", True)])["status"], "ok")
        self.assertEqual(summarize([r("a", True), r("b", False)])["status"], "degraded")
        self.assertEqual(summarize([r("a", False)])["status"], "down")
        self.assertEqual(summarize([])["status"], "ok")

    def test_counts(self):
        s = summarize([r("a", True), r("b", False), r("c", True)])
        self.assertEqual((s["up"], s["total"]), (2, 3))
        self.assertEqual([x["name"] for x in s["services"]], ["a", "b", "c"])

    def test_html_escapes(self):
        page = render_html("Ops <Board>", summarize([r("<script>", False), r("web", True)]))
        self.assertIn("Ops &lt;Board&gt;: degraded", page)
        self.assertIn("&lt;script&gt;", page)
        self.assertNotIn("<script>", page)
        self.assertIn("1 of 2 services up", page)
        self.assertIn("1.0 ms", page)


if __name__ == "__main__":
    unittest.main()
@@file Dockerfile
# Small official base image, pinned to a minor version
FROM python:3.12-slim

# No .pyc files, and logs appear immediately (containers log to stdout)
ENV PYTHONDONTWRITEBYTECODE=1 PYTHONUNBUFFERED=1

WORKDIR /app
COPY app.py .

# Never run as root inside the container
RUN useradd --create-home --uid 10001 appuser
USER appuser

EXPOSE 8080
HEALTHCHECK --interval=30s --timeout=3s --retries=3 \
  CMD python -c "import urllib.request; urllib.request.urlopen('http://localhost:8080/healthz', timeout=2)"

CMD ["python", "app.py"]
@@file .dockerignore
__pycache__/
*.pyc
test_*.py
k8s/
LAB.md
.git/
@@file compose.yaml
# docker compose up --build    then open http://localhost:8080
services:
  statusboard:
    build: .
    ports:
      - "8080:8080"
    environment:
      TITLE: "Office Services"
      TARGETS: "web=web:80,cache=cache:6379,printer=10.0.0.250:9100"
      TIMEOUT: "0.5"
    depends_on: [web, cache]
    restart: unless-stopped

  web:
    image: nginx:1.27-alpine

  cache:
    image: redis:7-alpine
@@file k8s/configmap.yaml
apiVersion: v1
kind: ConfigMap
metadata:
  name: statusboard-config
data:
  TITLE: "Cluster Services"
  TARGETS: "kube-dns=kube-dns.kube-system:53,web=web:80"
  TIMEOUT: "0.5"
@@file k8s/deployment.yaml
apiVersion: apps/v1
kind: Deployment
metadata:
  name: statusboard
  labels: { app: statusboard }
spec:
  replicas: 2
  selector:
    matchLabels: { app: statusboard }
  template:
    metadata:
      labels: { app: statusboard }
    spec:
      securityContext:
        runAsNonRoot: true
      containers:
        - name: statusboard
          image: statusboard:1.0
          imagePullPolicy: IfNotPresent
          ports:
            - containerPort: 8080
          envFrom:
            - configMapRef: { name: statusboard-config }
          resources:
            requests: { cpu: 50m, memory: 32Mi }
            limits: { cpu: 200m, memory: 64Mi }
          livenessProbe:
            httpGet: { path: /healthz, port: 8080 }
            periodSeconds: 10
          readinessProbe:
            httpGet: { path: /healthz, port: 8080 }
            periodSeconds: 5
          securityContext:
            allowPrivilegeEscalation: false
            readOnlyRootFilesystem: true
@@file k8s/service.yaml
apiVersion: v1
kind: Service
metadata:
  name: statusboard
spec:
  type: NodePort
  selector: { app: statusboard }
  ports:
    - port: 80
      targetPort: 8080
      nodePort: 30080
---
# Something for the status board to check
apiVersion: apps/v1
kind: Deployment
metadata:
  name: web
spec:
  replicas: 1
  selector:
    matchLabels: { app: web }
  template:
    metadata:
      labels: { app: web }
    spec:
      containers:
        - name: nginx
          image: nginx:1.27-alpine
          ports:
            - containerPort: 80
---
apiVersion: v1
kind: Service
metadata:
  name: web
spec:
  selector: { app: web }
  ports:
    - port: 80
@@file LAB.md
# Containerize the Status Board

You need **Docker Desktop** (or Docker Engine on Linux / WSL 2). For part 3, also **kind**
(`winget install Kubernetes.kind`) or **minikube**, and **kubectl**.

## 1. Run it without Docker
```
python -m unittest
set TARGETS=google=8.8.8.8:53,nothing=127.0.0.1:1     (PowerShell: $env:TARGETS="...")
python app.py
```
Open http://localhost:8080 and http://localhost:8080/api/status. Stop with Ctrl+C.

## 2. Docker
```
docker build -t statusboard:1.0 .
docker images statusboard
docker run --rm -p 8080:8080 -e TARGETS=dns=8.8.8.8:53 statusboard:1.0
docker ps                       (in another terminal: STATUS shows "healthy" after ~30 s)
docker exec -it <id> whoami     (appuser, not root)
docker history statusboard:1.0  (the layers)
```
Change `app.py` (the title colour, say) and build again: only the layers after `COPY` rebuild.

## 3. Docker Compose: three containers and a network
```
docker compose up --build -d
docker compose ps
docker compose logs -f statusboard
```
The board shows **degraded**: the printer address doesn't exist. Now:
```
docker compose stop cache      (refresh the page: cache is down)
docker compose start cache
docker compose down
```
Containers find each other by **service name** (`web`, `cache`): that's Compose's built-in DNS.

## 4. Kubernetes (kind)
```
kind create cluster --name lab
kind load docker-image statusboard:1.0 --name lab
kubectl apply -f k8s/
kubectl get pods,svc,deploy -o wide
kubectl port-forward svc/statusboard 8080:80
```
Then break things and watch Kubernetes fix them:
```
kubectl delete pod -l app=statusboard      (new pods appear: the Deployment keeps 2 replicas)
kubectl scale deployment statusboard --replicas=4
kubectl scale deployment web --replicas=0  (board: web down)  then back to 1
kubectl edit configmap statusboard-config ; kubectl rollout restart deployment statusboard
kubectl rollout status deployment statusboard ; kubectl rollout history deployment statusboard
kubectl logs -l app=statusboard --tail=20
kubectl describe pod <name>                (events: scheduling, probes, restarts)
```
Clean up: `kind delete cluster --name lab`.

## 5. Ideas
- Push the image to a registry (Docker Hub, GitHub Container Registry, ECR, ACR).
- Run it on a managed service: AWS App Runner / ECS Fargate, Azure Container Apps, Google Cloud Run.
- Scan the image: `docker scout cves statusboard:1.0` or `trivy image statusboard:1.0`.

@@drill
Which file describes the instructions for building a container image?
= Dockerfile
? docker build -t status-board .

@@text
🎉 You've built an image the right way (slim base, cached layers, non-root user, health check, `.dockerignore`), wired containers by service name, and watched Kubernetes self-heal, scale and roll out. That's exactly the demo to show in an interview for an infrastructure or SRE role.
