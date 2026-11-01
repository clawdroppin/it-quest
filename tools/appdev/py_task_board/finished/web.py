"""The web interface: a JSON API and one HTML page, on Python's built-in WSGI.

WSGI is the standard every Python web framework (Flask, Django) is built on: a web app is
just a function app(environ, start_response) that returns the response body as bytes.
"""
from __future__ import annotations

import json
import re
from html import escape
from urllib.parse import parse_qs

from models import STATUSES, TRANSITIONS, NotFound, ValidationError

ROUTES = [
    ("GET", "/", "page"),
    ("GET", "/api/tasks", "list_tasks"),
    ("POST", "/api/tasks", "create_task"),
    ("GET", "/api/tasks/{id}", "get_task"),
    ("POST", "/api/tasks/{id}/move", "move_task"),
    ("POST", "/api/tasks/{id}/assign", "assign_task"),
    ("DELETE", "/api/tasks/{id}", "delete_task"),
    ("GET", "/api/stats", "stats"),
]

STATUS_LINES = {
    200: "200 OK", 201: "201 Created", 204: "204 No Content", 400: "400 Bad Request",
    404: "404 Not Found", 405: "405 Method Not Allowed", 500: "500 Internal Server Error",
}


class HttpError(Exception):
    def __init__(self, status: int, message: str):
        super().__init__(message)
        self.status = status


def match(method: str, path: str, routes=ROUTES) -> tuple[str, dict[str, str]]:
    """Find the handler for a request: ("get_task", {"id": "7"}) for GET /api/tasks/7.

    A {name} in a route's pattern matches one path segment (no "/"). Raise HttpError(404,
    "not found") when no pattern matches the path, and HttpError(405, "method not allowed")
    when a pattern matches but with a different method.
    """
    path_exists = False
    for route_method, pattern, name in routes:
        regex = "^" + re.sub(r"\{(\w+)\}", r"(?P<\1>[^/]+)", pattern) + "$"
        found = re.match(regex, path)
        if found:
            if route_method == method:
                return name, found.groupdict()
            path_exists = True
    if path_exists:
        raise HttpError(405, "method not allowed")
    raise HttpError(404, "not found")


def _task_id(params: dict) -> int:
    try:
        return int(params["id"])
    except ValueError:
        raise HttpError(404, "not found") from None


def _read_json(environ) -> dict:
    size = int(environ.get("CONTENT_LENGTH") or 0)
    if size == 0:
        return {}
    try:
        data = json.loads(environ["wsgi.input"].read(size))
    except (ValueError, UnicodeDecodeError):
        raise HttpError(400, "body must be JSON") from None
    if not isinstance(data, dict):
        raise HttpError(400, "body must be a JSON object")
    return data


def render_page(columns: dict) -> str:
    """The board as one HTML page. Everything that came from users goes through escape()."""
    cols = []
    for status, tasks in columns.items():
        cards = []
        for t in tasks:
            buttons = "".join(
                f'<button data-id="{t.id}" data-to="{to}">→ {to}</button>' for to in sorted(TRANSITIONS[status])
            )
            who = f"<small>@{escape(t.assignee)}</small>" if t.assignee else ""
            tags = " ".join(f"<span class=tag>#{escape(tag)}</span>" for tag in t.tags)
            cards.append(f'<li class="p{t.priority}"><b>#{t.id}</b> {escape(t.title)} {who} {tags}<div>{buttons}</div></li>')
        cols.append(f"<section><h2>{status} ({len(tasks)})</h2><ul>{''.join(cards)}</ul></section>")
    return PAGE.replace("{{columns}}", "".join(cols))


def make_app(board):
    def page(**_):
        return 200, render_page(board.board())

    def list_tasks(query, **_):
        tasks = board.tasks(status=query.get("status"), assignee=query.get("assignee"), tag=query.get("tag"))
        return 200, [t.to_dict() for t in tasks]

    def create_task(body, **_):
        task = board.create_task(
            title=str(body.get("title", "")),
            priority=body.get("priority", 2),
            assignee=body.get("assignee"),
            tags=body.get("tags") or [],
        )
        return 201, task.to_dict()

    def get_task(params, **_):
        return 200, board.repo.get(_task_id(params)).to_dict()

    def move_task(params, body, **_):
        return 200, board.move(_task_id(params), str(body.get("status", ""))).to_dict()

    def assign_task(params, body, **_):
        return 200, board.assign(_task_id(params), body.get("assignee")).to_dict()

    def delete_task(params, **_):
        board.delete(_task_id(params))
        return 204, None

    def stats(**_):
        return 200, board.stats()

    handlers = {f.__name__: f for f in (page, list_tasks, create_task, get_task, move_task, assign_task, delete_task, stats)}

    def app(environ, start_response):
        method = environ.get("REQUEST_METHOD", "GET")
        try:
            name, params = match(method, environ.get("PATH_INFO") or "/")
            query = {k: v[0] for k, v in parse_qs(environ.get("QUERY_STRING", "")).items()}
            body = _read_json(environ) if method == "POST" else {}
            status, payload = handlers[name](params=params, query=query, body=body)
        except HttpError as e:
            status, payload = e.status, {"error": str(e)}
        except ValidationError as e:
            status, payload = 400, {"error": str(e)}
        except NotFound as e:
            status, payload = 404, {"error": str(e)}
        except Exception:  # never show a stack trace to a browser
            status, payload = 500, {"error": "internal error"}

        if isinstance(payload, str):
            data, kind = payload.encode(), "text/html; charset=utf-8"
        elif payload is None:
            data, kind = b"", "text/plain"
        else:
            data, kind = json.dumps(payload).encode(), "application/json"
        start_response(STATUS_LINES[status], [
            ("Content-Type", kind),
            ("Content-Length", str(len(data))),
            ("X-Content-Type-Options", "nosniff"),
            ("Content-Security-Policy", "default-src 'self'; script-src 'self' 'unsafe-inline'; style-src 'unsafe-inline'"),
        ])
        return [data]

    return app


PAGE = """<!doctype html>
<html lang="en">
<head>
<meta charset="utf-8">
<meta name="viewport" content="width=device-width, initial-scale=1">
<title>Team Task Board</title>
<style>
  body { font-family: system-ui, sans-serif; margin: 0; background: #f4f5f8; color: #1d2230; }
  header { padding: 16px 24px; background: #3b5bdb; color: white; }
  form { display: flex; gap: 8px; flex-wrap: wrap; padding: 16px 24px; }
  input, select, button { font: inherit; padding: 6px 10px; }
  main { display: grid; grid-template-columns: repeat(auto-fit, minmax(220px, 1fr)); gap: 16px; padding: 0 24px 24px; }
  section { background: white; border-radius: 10px; padding: 12px; }
  h2 { font-size: 16px; text-transform: uppercase; color: #5b6475; margin: 4px 0 12px; }
  ul { list-style: none; margin: 0; padding: 0; }
  li { border: 1px solid #e1e5ee; border-left: 4px solid #8a94a6; border-radius: 8px; padding: 8px; margin-bottom: 8px; }
  li.p1 { border-left-color: #e03131; } li.p2 { border-left-color: #f08c00; }
  .tag { background: #e7ecff; color: #3b5bdb; border-radius: 6px; padding: 0 6px; font-size: 12px; }
  li button { margin: 6px 6px 0 0; font-size: 12px; }
  #error { color: #e03131; padding: 0 24px; }
</style>
</head>
<body>
<header><h1>Team Task Board</h1></header>
<form id="add">
  <input name="title" placeholder="New task" required maxlength="120">
  <select name="priority"><option value="1">P1 urgent</option><option value="2" selected>P2</option><option value="3">P3</option></select>
  <input name="assignee" placeholder="Assignee">
  <input name="tags" placeholder="tags, comma separated">
  <button>Add</button>
</form>
<p id="error"></p>
<main>{{columns}}</main>
<script>
async function call(method, url, body) {
  const res = await fetch(url, { method, headers: { "Content-Type": "application/json" }, body: JSON.stringify(body) });
  if (!res.ok) { document.getElementById("error").textContent = (await res.json()).error; return; }
  location.reload();
}
document.getElementById("add").addEventListener("submit", (e) => {
  e.preventDefault();
  const f = new FormData(e.target);
  call("POST", "/api/tasks", {
    title: f.get("title"), priority: Number(f.get("priority")), assignee: f.get("assignee"),
    tags: f.get("tags").split(",").map((t) => t.trim()).filter(Boolean),
  });
});
document.querySelector("main").addEventListener("click", (e) => {
  const b = e.target.closest("button[data-id]");
  if (b) call("POST", `/api/tasks/${b.dataset.id}/move`, { status: b.dataset.to });
});
</script>
</body>
</html>
"""
