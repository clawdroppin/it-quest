@@lesson r11-logging Logging and Observability for Python Services
@@concepts automation, json
@@text
`print` is fine for scripts you watch. Services, scheduled jobs and tools that run on servers need **logging**: levels, timestamps, structured fields, and output that monitoring systems can search. This lesson covers the `logging` module properly, then metrics and traces.

## How logging is organised

```text
logger "assetctl.sync"  ──record──▶  handlers (console, file, syslog, HTTP)  ──▶  formatter (text or JSON)
        │ propagates up
logger "assetctl"  ──▶  root logger
```

- **Loggers** are named with dots, usually `logging.getLogger(__name__)`, forming a hierarchy that follows your packages.
- **Levels**: `DEBUG < INFO < WARNING < ERROR < CRITICAL`. A record is emitted if its level is at or above the logger's (and handler's) level.
- **Handlers** decide where records go; **formatters** decide how they look; **filters** can drop or enrich records.
- Records **propagate** to parent loggers, so configuring the root logger once (at program start) covers every module.

```python
import logging, sys

def setup(level="INFO"):
    handler = logging.StreamHandler(sys.stdout)
    handler.setFormatter(logging.Formatter("%(asctime)s %(levelname)-7s %(name)s: %(message)s", "%H:%M:%S"))
    root = logging.getLogger()
    root.handlers[:] = [handler]
    root.setLevel(level)

setup()
log = logging.getLogger("assetctl.sync")
log.debug("not shown at INFO")
log.info("synced %d assets from %s", 42, "intune")     # lazy %-formatting
try:
    1 / 0
except ZeroDivisionError:
    log.exception("sync failed")                        # ERROR + traceback
```

Rules:

- **Libraries** only create loggers (`getLogger(__name__)`) and never configure handlers. **Applications** configure logging once, in `main()`.
- Pass values as arguments (`log.info("synced %d", n)`), not f-strings: formatting is skipped when the level is disabled, and log tools can group messages by template.
- `log.exception(...)` inside `except` includes the traceback.
- Larger apps configure with `logging.config.dictConfig({...})` from a file.

## Files and rotation

```python,norun
from logging.handlers import RotatingFileHandler, TimedRotatingFileHandler
handler = TimedRotatingFileHandler("app.log", when="midnight", backupCount=14, encoding="utf-8")
```

On servers, prefer writing to **stdout/stderr** and letting the platform collect logs (systemd journal, Docker, Kubernetes, Azure App Service). On Windows services, the Event Log (`NTEventLogHandler`) or files.

## Structured (JSON) logs

Text logs are for humans; log platforms (Elastic, Loki, Datadog, Splunk, Azure Monitor) work best with **one JSON object per line**, where every field can be filtered and graphed:

```json
{"ts": "2026-11-03T09:00:00Z", "level": "ERROR", "logger": "assetctl.sync", "msg": "sync failed", "source": "intune", "request_id": "a1b2"}
```

Add fields with `extra={"source": "intune"}`, or use **structlog**, which makes structured logging the default.

## Correlation IDs with contextvars

When many requests run concurrently, you need to know which log lines belong to the same request. Store a request ID in a **context variable** and add it to every record with a filter:

```python
import contextvars, logging

request_id = contextvars.ContextVar("request_id", default="-")

class RequestIdFilter(logging.Filter):
    def filter(self, record):
        record.request_id = request_id.get()
        return True

h = logging.StreamHandler()
h.addFilter(RequestIdFilter())
h.setFormatter(logging.Formatter("[%(request_id)s] %(message)s"))
log = logging.getLogger("demo")
log.handlers[:] = [h]
log.propagate = False

request_id.set("req-42")
log.warning("payment declined")
```

`contextvars` work correctly with threads **and** asyncio tasks (each task gets its own copy).

## Metrics and traces

- **Metrics**: counters, gauges and histograms (`requests_total`, `queue_length`, `request_seconds`). `prometheus_client` exposes them at `/metrics` for Prometheus/Grafana.
- **Traces**: **OpenTelemetry** (`opentelemetry-sdk`, plus auto-instrumentation for FastAPI, requests, SQLAlchemy) follows a request across services and shows where time is spent.
- **Error tracking**: Sentry collects exceptions with context and groups them.
- **Health checks**: `/healthz` and `/readyz` endpoints for load balancers.

## What not to log

Passwords, tokens, API keys, session cookies, full card numbers, and personal data you don't need. Mask them (`token=abcd…`) with a filter, because one debug line can leak a secret into a log system many people can read.

@@quiz
How should a library get its logger?
* `logging.getLogger(__name__)`, without configuring handlers
- `logging.basicConfig()` at import
- print()
- A global logger from the app
? The application configures handlers once.

@@quiz
Why write `log.info("synced %d", n)` instead of an f-string?
- f-strings are not allowed
* Formatting is skipped when the level is disabled, and tools can group messages by their template
- It's shorter
- It encrypts the value
? Lazy formatting.

@@quiz
What does `log.exception("...")` add compared to `log.error`?
- Nothing
* The current exception's traceback
- An email alert
- It exits the program
? Use it inside an except block.

@@quiz
Why use `contextvars` for a request ID?
- It's faster than a global
* Each thread and asyncio task gets its own value, so concurrent requests don't mix up IDs
- It's required by logging
- It writes to disk
? A plain global would be overwritten by the next request.

@@drill
Which logging handler starts a new file when the current one gets too big?
= RotatingFileHandler
= logging.handlers.RotatingFileHandler
? TimedRotatingFileHandler rotates by time.

@@flashcards
Logger :: named source of log records, `getLogger(__name__)`
Handler :: sends records somewhere (console, file, network)
Formatter :: turns a record into text or JSON
Filter :: drops or enriches records
Propagation :: records travel up to parent loggers
`log.exception` :: logs at ERROR with the traceback
Structured logging :: logs as JSON fields
Correlation ID :: an ID linking all log lines of one request
contextvars :: context-local variables safe for threads and asyncio
OpenTelemetry :: standard for traces, metrics and logs

@@exercise A JSON log formatter
Write `JsonFormatter(logging.Formatter)` whose `format(record)` returns one JSON line with keys in this order: `level`, `logger`, `msg` (the formatted message, `record.getMessage()`), then any **extra** fields passed with `extra={...}`, and finally `exc` with the exception type and message (`"ZeroDivisionError: division by zero"`) if `record.exc_info` is set. Values in extra fields whose key contains `password` or `token` must be replaced with `"***"`.

To find extra fields, compare `record.__dict__` with the attributes every record has (`STANDARD` is given).
@@starter
import json
import logging

STANDARD = set(vars(logging.LogRecord("x", 10, "", 0, "", None, None))) | {"message", "asctime", "taskName"}


class JsonFormatter(logging.Formatter):
    pass


def make_logger(stream):
    handler = logging.StreamHandler(stream)
    handler.setFormatter(JsonFormatter())
    log = logging.getLogger("assetctl")
    log.handlers[:] = [handler]
    log.setLevel(logging.INFO)
    log.propagate = False
    return log


import io
buf = io.StringIO()
log = make_logger(buf)
log.info("synced %d assets", 42, extra={"source": "intune", "api_token": "abc123"})
print(buf.getvalue().strip())
@@expected
{"level": "INFO", "logger": "assetctl", "msg": "synced 42 assets", "source": "intune", "api_token": "***"}
@@tests
buf = io.StringIO()
log = make_logger(buf)
try:
    1 / 0
except ZeroDivisionError:
    log.exception("sync failed", extra={"password": "hunter2"})
line = json.loads(buf.getvalue())
check(line["msg"], "sync failed", "message")
check(line["password"], "***", "masked")
check(line["exc"], "ZeroDivisionError: division by zero", "exception")
check(list(line)[:3], ["level", "logger", "msg"], "key order")
buf2 = io.StringIO()
log = make_logger(buf2)
log.warning("plain")
check(json.loads(buf2.getvalue()), {"level": "WARNING", "logger": "assetctl", "msg": "plain"}, "no extras")
@@hint
`data = {"level": record.levelname, "logger": record.name, "msg": record.getMessage()}`. Then `for key, value in record.__dict__.items(): if key not in STANDARD:` add it (masked if needed). For the exception: `etype, evalue, _ = record.exc_info; data["exc"] = f"{etype.__name__}: {evalue}"`. Return `json.dumps(data)`.
@@solution
import json
import logging

STANDARD = set(vars(logging.LogRecord("x", 10, "", 0, "", None, None))) | {"message", "asctime", "taskName"}


class JsonFormatter(logging.Formatter):
    def format(self, record):
        data = {"level": record.levelname, "logger": record.name, "msg": record.getMessage()}
        for key, value in record.__dict__.items():
            if key in STANDARD:
                continue
            if "password" in key or "token" in key:
                value = "***"
            data[key] = value
        if record.exc_info:
            etype, evalue, _ = record.exc_info
            data["exc"] = f"{etype.__name__}: {evalue}"
        return json.dumps(data)


def make_logger(stream):
    handler = logging.StreamHandler(stream)
    handler.setFormatter(JsonFormatter())
    log = logging.getLogger("assetctl")
    log.handlers[:] = [handler]
    log.setLevel(logging.INFO)
    log.propagate = False
    return log


import io
buf = io.StringIO()
log = make_logger(buf)
log.info("synced %d assets", 42, extra={"source": "intune", "api_token": "abc123"})
print(buf.getvalue().strip())

@@exercise Request IDs across async tasks
Write:

- `request_id = contextvars.ContextVar("request_id", default="-")`
- `async def handle(name, delay, lines)`: sets the request ID to `f"req-{name}"`, awaits `asyncio.sleep(delay)`, then appends `f"[{request_id.get()}] done {name}"` to `lines`
- `async def serve(requests)`: runs `handle` for every `(name, delay)` concurrently with `asyncio.gather` and returns the lines

Because each task has its own context, every line must show its own ID even though the tasks interleave. Also, after `serve` finishes, `request_id.get()` in the caller is still `"-"`.
@@starter
import asyncio
import contextvars


async def handle(name, delay, lines):
    pass


async def serve(requests):
    pass


print(asyncio.run(serve([("a", 0.03), ("b", 0.01), ("c", 0.02)])))
@@expected
['[req-b] done b', '[req-c] done c', '[req-a] done a']
@@tests
lines = asyncio.run(serve([("x", 0.02), ("y", 0.0)]))
check(lines, ["[req-y] done y", "[req-x] done x"], "own ids")
check(request_id.get(), "-", "caller context untouched")
@@requires
ContextVar
@@hint
`handle`: `request_id.set(f"req-{name}")`, `await asyncio.sleep(delay)`, append. `serve`: `lines = []`, `await asyncio.gather(*(handle(n, d, lines) for n, d in requests))`, return lines. `gather` wraps each coroutine in a task, and each task copies the context.
@@solution
import asyncio
import contextvars

request_id = contextvars.ContextVar("request_id", default="-")


async def handle(name, delay, lines):
    request_id.set(f"req-{name}")
    await asyncio.sleep(delay)
    lines.append(f"[{request_id.get()}] done {name}")


async def serve(requests):
    lines = []
    await asyncio.gather(*(handle(n, d, lines) for n, d in requests))
    return lines


print(asyncio.run(serve([("a", 0.03), ("b", 0.01), ("c", 0.02)])))
