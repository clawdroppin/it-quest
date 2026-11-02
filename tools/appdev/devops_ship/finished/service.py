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
