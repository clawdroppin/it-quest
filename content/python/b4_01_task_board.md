@@lesson b4-task-board Team Task Board (a full application)
@@text
Time to build a real **application**, not a script: a Kanban board for an IT team, with a database, business rules, a JSON API, a web page and a command line. Everything uses only Python's standard library, so it runs anywhere Python does.

```text
python main.py add "Renew TLS cert" --priority 1 --assignee yuki --tag web
python main.py move 1 doing
python main.py list
python main.py serve          # then open http://localhost:8000
```

## How it's built

It uses the layers from the *Application Development* lessons. Each file has one job, and the arrows only point one way:

| File | Layer | Knows about |
|---|---|---|
| `models.py` | domain | what a Task is; rules that always hold (a title is required, priorities are 1-3) |
| `repo.py` | storage | SQLite, migrations, SQL. Nothing else touches the database |
| `service.py` | use cases | the board's rules: allowed moves, "assign before you start", a WIP limit |
| `web.py` | interface | HTTP: routes, JSON, status codes, the HTML page |
| `main.py` | interface | the command line, and wiring everything together |

`web.py` and `main.py` are both thin: they translate a request into a call to `TaskBoard` and translate the result back. That's why the same app can have a web page *and* a CLI, and why a desktop GUI could be added without touching the rules.

## The rules you'll implement

```text
todo ──► doing ──► review ──► done
  ▲        │  ▲       │
  └────────┘  └───────┘
```

- Only the moves in the diagram are allowed (`can't move from todo to done`).
- A task must have an assignee before it can start.
- **WIP limit**: nobody may have more than 3 tasks in *doing*. Limiting work in progress is how Kanban teams finish things instead of starting things.
- Moving to *done* records the time, which feeds the weekly **cycle time** statistic.

## Your tasks

The starter has everything except four pieces, marked `raise NotImplementedError`:

1. `TaskRepo.find` in `repo.py`: build a query from optional filters, **with ? parameters**. One test tries SQL injection.
2. `TaskBoard.move` in `service.py`: the rules above.
3. `TaskBoard.board` and `TaskBoard.stats`: group tasks into columns; count, and average the cycle time.
4. `match` in `web.py`: the router. Turn `/api/tasks/{id}/move` into a regular expression and return `("move_task", {"id": "12"})`, or raise 404/405.

Run the tests after each piece: `python -m unittest -v`. When they all pass, run `python main.py serve` and use your app in the browser.
@@project task_board Team Task Board
Fill in the four `NotImplementedError` pieces until `python -m unittest -v` passes, then run `python main.py serve`.
@@file models.py
"""The domain: what a task is and which rules always hold. No database, no web, no printing."""
from __future__ import annotations

from dataclasses import dataclass, field
from datetime import datetime

STATUSES = ("todo", "doing", "review", "done")
# The moves a task may make. "done" is final.
TRANSITIONS = {
    "todo": {"doing"},
    "doing": {"todo", "review"},
    "review": {"doing", "done"},
    "done": set(),
}
PRIORITIES = (1, 2, 3)  # 1 = urgent, 3 = when there's time


class ValidationError(ValueError):
    """Bad input from a user. The message is shown to them as it is."""


class NotFound(LookupError):
    """There's no task with that id."""


@dataclass
class Task:
    title: str
    priority: int = 2
    assignee: str | None = None
    tags: list[str] = field(default_factory=list)
    status: str = "todo"
    created: datetime | None = None
    done_at: datetime | None = None
    id: int | None = None

    def __post_init__(self) -> None:
        # Every Task that exists is valid: the checks run whenever one is made.
        self.title = self.title.strip()
        if not self.title:
            raise ValidationError("title is required")
        if len(self.title) > 120:
            raise ValidationError("title is too long (max 120)")
        if self.priority not in PRIORITIES:
            raise ValidationError("priority must be 1, 2 or 3")
        if self.status not in STATUSES:
            raise ValidationError(f"unknown status: {self.status}")
        self.tags = sorted({t.strip().lower() for t in self.tags if t.strip()})

    def to_dict(self) -> dict:
        return {
            "id": self.id,
            "title": self.title,
            "priority": self.priority,
            "assignee": self.assignee,
            "tags": self.tags,
            "status": self.status,
            "created": self.created.isoformat() if self.created else None,
            "done_at": self.done_at.isoformat() if self.done_at else None,
        }
@@file repo.py
"""Stores tasks in SQLite. This is the only file that knows SQL."""
from __future__ import annotations

import sqlite3
from datetime import datetime

from models import NotFound, Task

# Each entry upgrades the database by one version. Never edit an old entry: add a new one,
# because databases out in the world already ran the old ones.
MIGRATIONS = [
    """
    CREATE TABLE tasks (
        id       INTEGER PRIMARY KEY,
        title    TEXT NOT NULL,
        priority INTEGER NOT NULL DEFAULT 2,
        assignee TEXT,
        status   TEXT NOT NULL DEFAULT 'todo',
        created  TEXT NOT NULL
    );
    """,
    """
    CREATE TABLE task_tags (
        task_id INTEGER NOT NULL REFERENCES tasks(id) ON DELETE CASCADE,
        tag     TEXT NOT NULL,
        PRIMARY KEY (task_id, tag)
    );
    CREATE INDEX idx_tasks_status ON tasks(status, assignee);
    """,
    "ALTER TABLE tasks ADD COLUMN done_at TEXT;",
]


def connect(path: str = ":memory:") -> sqlite3.Connection:
    """Open (or create) the database and bring its schema up to date."""
    conn = sqlite3.connect(path)
    conn.row_factory = sqlite3.Row
    conn.execute("PRAGMA foreign_keys = ON")
    migrate(conn)
    return conn


def migrate(conn: sqlite3.Connection) -> int:
    """Run the migrations this database hasn't had yet, and return its new version.

    SQLite keeps a free integer in every database file, `PRAGMA user_version`: we use it as
    "how many migrations have run". Each migration and its version bump commit together.
    """
    version = conn.execute("PRAGMA user_version").fetchone()[0]
    for number, sql in enumerate(MIGRATIONS[version:], start=version + 1):
        conn.executescript(f"BEGIN; {sql} PRAGMA user_version = {number}; COMMIT;")
    return len(MIGRATIONS)


def _iso(value: datetime | None) -> str | None:
    return value.isoformat() if value else None


def _dt(value: str | None) -> datetime | None:
    return datetime.fromisoformat(value) if value else None


class TaskRepo:
    def __init__(self, conn: sqlite3.Connection):
        self.conn = conn

    def add(self, task: Task) -> Task:
        with self.conn:  # one transaction: the task and its tags, or nothing
            cur = self.conn.execute(
                "INSERT INTO tasks (title, priority, assignee, status, created, done_at) VALUES (?, ?, ?, ?, ?, ?)",
                (task.title, task.priority, task.assignee, task.status, _iso(task.created), _iso(task.done_at)),
            )
            task.id = cur.lastrowid
            self._save_tags(task)
        return task

    def get(self, task_id: int) -> Task:
        row = self.conn.execute("SELECT * FROM tasks WHERE id = ?", (task_id,)).fetchone()
        if row is None:
            raise NotFound(f"no task #{task_id}")
        return self._to_task(row)

    def find(self, status: str | None = None, assignee: str | None = None, tag: str | None = None) -> list[Task]:
        """Tasks matching every filter that isn't None, most urgent first, then oldest first.

        Build the SQL from fixed pieces and pass the values as ? parameters: never put user
        input into the SQL text itself.
        """
        raise NotImplementedError

    def update(self, task: Task) -> None:
        with self.conn:
            cur = self.conn.execute(
                "UPDATE tasks SET title = ?, priority = ?, assignee = ?, status = ?, done_at = ? WHERE id = ?",
                (task.title, task.priority, task.assignee, task.status, _iso(task.done_at), task.id),
            )
            if cur.rowcount == 0:
                raise NotFound(f"no task #{task.id}")
            self._save_tags(task)

    def delete(self, task_id: int) -> None:
        with self.conn:
            cur = self.conn.execute("DELETE FROM tasks WHERE id = ?", (task_id,))
            if cur.rowcount == 0:
                raise NotFound(f"no task #{task_id}")

    def _save_tags(self, task: Task) -> None:
        self.conn.execute("DELETE FROM task_tags WHERE task_id = ?", (task.id,))
        self.conn.executemany("INSERT INTO task_tags (task_id, tag) VALUES (?, ?)", [(task.id, t) for t in task.tags])

    def _to_task(self, row: sqlite3.Row) -> Task:
        tags = [r[0] for r in self.conn.execute("SELECT tag FROM task_tags WHERE task_id = ? ORDER BY tag", (row["id"],))]
        return Task(
            id=row["id"],
            title=row["title"],
            priority=row["priority"],
            assignee=row["assignee"],
            status=row["status"],
            tags=tags,
            created=_dt(row["created"]),
            done_at=_dt(row["done_at"]),
        )
@@file service.py
"""The use cases: what people can do with the board, and the business rules.

TaskBoard doesn't know about SQL or HTTP. It gets a repository and a clock from outside
("dependency injection"), so tests can give it an in-memory database and a fixed time.
"""
from __future__ import annotations

from datetime import datetime, timedelta
from typing import Callable

from models import STATUSES, TRANSITIONS, Task, ValidationError

WIP_LIMIT = 3  # how many tasks one person may have in "doing" at once


def _priority(value) -> int:
    try:
        return int(value)
    except (TypeError, ValueError):
        raise ValidationError("priority must be 1, 2 or 3") from None


class TaskBoard:
    def __init__(self, repo, clock: Callable[[], datetime] = datetime.now):
        self.repo = repo
        self.clock = clock

    def create_task(self, title: str, priority=2, assignee: str | None = None, tags=()) -> Task:
        task = Task(
            title=title,
            priority=_priority(priority),
            assignee=(assignee or "").strip() or None,
            tags=list(tags),
            created=self.clock(),
        )
        return self.repo.add(task)

    def tasks(self, status=None, assignee=None, tag=None) -> list[Task]:
        return self.repo.find(status=status, assignee=assignee, tag=tag)

    def assign(self, task_id: int, assignee: str | None) -> Task:
        task = self.repo.get(task_id)
        task.assignee = (assignee or "").strip() or None
        self.repo.update(task)
        return task

    def move(self, task_id: int, status: str) -> Task:
        """Move a task to a new column, enforcing the rules:

        - the status must exist, and the move must be in TRANSITIONS (ValidationError
          "can't move from todo to done" otherwise),
        - a task can only start ("doing") once someone is assigned ("assign the task before
          starting it"),
        - nobody may have more than WIP_LIMIT tasks in "doing" ("yuki already has 3 tasks
          in progress"),
        - moving to "done" records done_at = now; moving anywhere else clears it.
        """
        raise NotImplementedError

    def delete(self, task_id: int) -> None:
        self.repo.delete(task_id)

    def board(self) -> dict[str, list[Task]]:
        """Every task, grouped by status, with all four columns present even when empty."""
        raise NotImplementedError

    def stats(self, days: int = 7) -> dict:
        """Numbers for the team's weekly meeting:

        {"counts": {"todo": 2, "doing": 1, "review": 0, "done": 3},
         "done_recently": 2,          # finished in the last `days` days
         "avg_cycle_hours": 30.5}     # created -> done, for those tasks; None if there are none
        """
        raise NotImplementedError
@@file web.py
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
    raise NotImplementedError


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
@@file main.py
"""Team Task Board: a small Kanban app for an IT team.

python main.py serve                      open http://localhost:8000
python main.py add "Renew TLS cert" --priority 1 --assignee yuki --tag web
python main.py list
python main.py move 1 doing
python main.py stats
"""
from __future__ import annotations

import argparse
import json
import sys
from wsgiref.simple_server import make_server

from models import NotFound, ValidationError
from repo import TaskRepo, connect
from service import TaskBoard
from web import make_app


def main(argv: list[str] | None = None) -> int:
    parser = argparse.ArgumentParser(description="Team Task Board")
    parser.add_argument("--db", default="tasks.db", help="database file (default: tasks.db)")
    sub = parser.add_subparsers(dest="cmd", required=True)
    serve = sub.add_parser("serve", help="run the web app")
    serve.add_argument("--port", type=int, default=8000)
    add = sub.add_parser("add", help="add a task")
    add.add_argument("title")
    add.add_argument("--priority", type=int, default=2, choices=[1, 2, 3])
    add.add_argument("--assignee")
    add.add_argument("--tag", action="append", default=[])
    sub.add_parser("list", help="show the board")
    move = sub.add_parser("move", help="move a task to another column")
    move.add_argument("id", type=int)
    move.add_argument("status")
    assign = sub.add_parser("assign", help="assign a task to someone")
    assign.add_argument("id", type=int)
    assign.add_argument("assignee")
    sub.add_parser("stats", help="numbers for the weekly meeting")
    args = parser.parse_args(argv)

    board = TaskBoard(TaskRepo(connect(args.db)))
    try:
        if args.cmd == "serve":
            print(f"Task board on http://localhost:{args.port}  (Ctrl+C to stop)")
            with make_server("127.0.0.1", args.port, make_app(board)) as server:
                server.serve_forever()
        elif args.cmd == "add":
            task = board.create_task(args.title, args.priority, args.assignee, args.tag)
            print(f"added #{task.id}: {task.title}")
        elif args.cmd == "list":
            for status, tasks in board.board().items():
                print(f"== {status} ({len(tasks)})")
                for t in tasks:
                    who = f" @{t.assignee}" if t.assignee else ""
                    tags = "".join(f" #{tag}" for tag in t.tags)
                    print(f"  #{t.id} P{t.priority} {t.title}{who}{tags}")
        elif args.cmd == "move":
            task = board.move(args.id, args.status)
            print(f"#{task.id} is now {task.status}")
        elif args.cmd == "assign":
            task = board.assign(args.id, args.assignee)
            print(f"#{task.id} is assigned to {task.assignee}")
        elif args.cmd == "stats":
            print(json.dumps(board.stats(), indent=2))
    except (ValidationError, NotFound) as e:
        print(f"error: {e}", file=sys.stderr)
        return 1
    except KeyboardInterrupt:
        pass
    return 0


if __name__ == "__main__":
    sys.exit(main())
@@file test_board.py
"""Run: python -m unittest -v"""
import io
import json
import unittest
from datetime import datetime, timedelta
from wsgiref.util import setup_testing_defaults

from models import NotFound, Task, ValidationError
from repo import MIGRATIONS, TaskRepo, connect, migrate
from service import WIP_LIMIT, TaskBoard
from web import HttpError, make_app, match, render_page


class FakeClock:
    """A clock the test controls: time only moves when we say so."""

    def __init__(self):
        self.now = datetime(2026, 11, 2, 9, 0)

    def __call__(self):
        return self.now

    def advance(self, **kwargs):
        self.now += timedelta(**kwargs)


def new_board():
    clock = FakeClock()
    return TaskBoard(TaskRepo(connect(":memory:")), clock), clock


class ModelTests(unittest.TestCase):
    def test_valid_task_is_cleaned(self):
        t = Task(title="  Renew cert ", tags=["Web", "web", " TLS ", ""])
        self.assertEqual(t.title, "Renew cert")
        self.assertEqual(t.tags, ["tls", "web"])

    def test_invalid_tasks_cannot_exist(self):
        for kwargs in ({"title": "  "}, {"title": "x" * 121}, {"title": "ok", "priority": 5}, {"title": "ok", "status": "lost"}):
            with self.assertRaises(ValidationError):
                Task(**kwargs)


class RepoTests(unittest.TestCase):
    def setUp(self):
        self.conn = connect(":memory:")
        self.repo = TaskRepo(self.conn)
        when = datetime(2026, 11, 2, 9, 0)
        self.repo.add(Task(title="Patch web01", priority=1, assignee="yuki", tags=["linux", "patching"], created=when))
        self.repo.add(Task(title="Order toner", priority=3, created=when))
        self.repo.add(Task(title="Patch db01", priority=1, assignee="ken", tags=["linux"], status="doing", created=when))

    def test_migrations_bring_schema_to_latest(self):
        self.assertEqual(self.conn.execute("PRAGMA user_version").fetchone()[0], len(MIGRATIONS))
        self.assertEqual(migrate(self.conn), len(MIGRATIONS))  # running again changes nothing

    def test_round_trip(self):
        t = self.repo.get(1)
        self.assertEqual((t.title, t.priority, t.assignee, t.tags), ("Patch web01", 1, "yuki", ["linux", "patching"]))
        self.assertEqual(t.created, datetime(2026, 11, 2, 9, 0))

    def test_find_filters_and_order(self):
        self.assertEqual([t.id for t in self.repo.find()], [1, 3, 2])  # priority, then id
        self.assertEqual([t.id for t in self.repo.find(tag="LINUX")], [1, 3])
        self.assertEqual([t.id for t in self.repo.find(status="doing")], [3])
        self.assertEqual([t.id for t in self.repo.find(assignee="yuki", tag="linux")], [1])
        self.assertEqual(self.repo.find(tag="windows"), [])

    def test_find_is_safe_from_sql_injection(self):
        self.assertEqual(self.repo.find(assignee="x' OR '1'='1"), [])

    def test_update_and_delete(self):
        t = self.repo.get(2)
        t.tags = ["printer"]
        self.repo.update(t)
        self.assertEqual(self.repo.get(2).tags, ["printer"])
        self.repo.delete(2)
        with self.assertRaises(NotFound):
            self.repo.get(2)
        with self.assertRaises(NotFound):
            self.repo.delete(2)
        self.assertEqual(self.conn.execute("SELECT COUNT(*) FROM task_tags WHERE task_id = 2").fetchone()[0], 0)


class ServiceTests(unittest.TestCase):
    def setUp(self):
        self.board, self.clock = new_board()

    def test_create_validates(self):
        t = self.board.create_task("Renew TLS cert", "1", " yuki ", ["web"])
        self.assertEqual((t.id, t.priority, t.assignee, t.status), (1, 1, "yuki", "todo"))
        with self.assertRaises(ValidationError):
            self.board.create_task("x", priority="urgent")

    def test_happy_path_to_done(self):
        t = self.board.create_task("Renew TLS cert", assignee="yuki")
        self.board.move(t.id, "doing")
        self.board.move(t.id, "review")
        self.clock.advance(hours=5)
        done = self.board.move(t.id, "done")
        self.assertEqual(done.status, "done")
        self.assertEqual(done.done_at, datetime(2026, 11, 2, 14, 0))

    def test_illegal_moves(self):
        t = self.board.create_task("Renew TLS cert", assignee="yuki")
        with self.assertRaisesRegex(ValidationError, "can't move from todo to done"):
            self.board.move(t.id, "done")
        with self.assertRaisesRegex(ValidationError, "unknown status"):
            self.board.move(t.id, "archived")
        with self.assertRaises(NotFound):
            self.board.move(99, "doing")

    def test_must_be_assigned_to_start(self):
        t = self.board.create_task("Nobody's task")
        with self.assertRaisesRegex(ValidationError, "assign the task"):
            self.board.move(t.id, "doing")
        self.board.assign(t.id, "ken")
        self.assertEqual(self.board.move(t.id, "doing").status, "doing")

    def test_wip_limit(self):
        ids = [self.board.create_task(f"Task {i}", assignee="yuki").id for i in range(WIP_LIMIT + 1)]
        for task_id in ids[:WIP_LIMIT]:
            self.board.move(task_id, "doing")
        with self.assertRaisesRegex(ValidationError, f"yuki already has {WIP_LIMIT} tasks in progress"):
            self.board.move(ids[-1], "doing")
        other = self.board.create_task("Ken's task", assignee="ken")
        self.board.move(other.id, "doing")  # the limit is per person

    def test_moving_back_clears_done_at(self):
        t = self.board.create_task("Review firewall rules", assignee="ken")
        self.board.move(t.id, "doing")
        self.board.move(t.id, "review")
        back = self.board.move(t.id, "doing")
        self.assertIsNone(back.done_at)

    def test_board_has_every_column(self):
        self.board.create_task("A")
        cols = self.board.board()
        self.assertEqual(list(cols), ["todo", "doing", "review", "done"])
        self.assertEqual([t.title for t in cols["todo"]], ["A"])
        self.assertEqual(cols["done"], [])

    def test_stats(self):
        self.assertEqual(self.board.stats(), {"counts": {"todo": 0, "doing": 0, "review": 0, "done": 0},
                                              "done_recently": 0, "avg_cycle_hours": None})
        a = self.board.create_task("A", assignee="yuki")
        b = self.board.create_task("B", assignee="yuki")
        self.board.create_task("C")
        for task, hours in ((a, 10), (b, 20)):
            self.board.move(task.id, "doing")
            self.board.move(task.id, "review")
            self.clock.advance(hours=hours)
            self.board.move(task.id, "done")
        stats = self.board.stats()
        self.assertEqual(stats["counts"], {"todo": 1, "doing": 0, "review": 0, "done": 2})
        self.assertEqual(stats["done_recently"], 2)
        self.assertEqual(stats["avg_cycle_hours"], 20.0)  # a: 10 h, b: 30 h (created at the start)
        self.clock.advance(days=8)
        self.assertEqual(self.board.stats()["done_recently"], 0)


def request(app, method, path, body=None, query=""):
    environ = {}
    setup_testing_defaults(environ)
    data = json.dumps(body).encode() if body is not None else b""
    environ.update(REQUEST_METHOD=method, PATH_INFO=path, QUERY_STRING=query,
                   CONTENT_LENGTH=str(len(data)), **{"wsgi.input": io.BytesIO(data)})
    seen = {}

    def start_response(status, headers):
        seen["status"], seen["headers"] = status, dict(headers)

    raw = b"".join(app(environ, start_response))
    is_json = seen["headers"]["Content-Type"] == "application/json"
    return int(seen["status"][:3]), (json.loads(raw) if is_json and raw else raw.decode())


class WebTests(unittest.TestCase):
    def setUp(self):
        self.board, _ = new_board()
        self.app = make_app(self.board)

    def test_match(self):
        self.assertEqual(match("GET", "/api/tasks"), ("list_tasks", {}))
        self.assertEqual(match("POST", "/api/tasks/12/move"), ("move_task", {"id": "12"}))
        with self.assertRaises(HttpError) as e:
            match("GET", "/api/nothing")
        self.assertEqual(e.exception.status, 404)
        with self.assertRaises(HttpError) as e:
            match("PUT", "/api/tasks")
        self.assertEqual(e.exception.status, 405)
        with self.assertRaises(HttpError):
            match("GET", "/api/tasks/1/2")  # {id} never matches a "/"

    def test_api_flow(self):
        status, task = request(self.app, "POST", "/api/tasks", {"title": "Renew TLS cert", "assignee": "yuki", "tags": ["web"]})
        self.assertEqual((status, task["id"], task["status"]), (201, 1, "todo"))
        status, moved = request(self.app, "POST", "/api/tasks/1/move", {"status": "doing"})
        self.assertEqual((status, moved["status"]), (200, "doing"))
        status, tasks = request(self.app, "GET", "/api/tasks", query="tag=web")
        self.assertEqual((status, [t["title"] for t in tasks]), (200, ["Renew TLS cert"]))
        self.assertEqual(request(self.app, "DELETE", "/api/tasks/1"), (204, ""))
        self.assertEqual(request(self.app, "GET", "/api/tasks/1")[0], 404)

    def test_errors_are_json(self):
        self.assertEqual(request(self.app, "POST", "/api/tasks", {"title": ""}), (400, {"error": "title is required"}))
        self.assertEqual(request(self.app, "POST", "/api/tasks/abc/move", {"status": "doing"})[0], 404)
        self.assertEqual(request(self.app, "GET", "/nope"), (404, {"error": "not found"}))

    def test_page_escapes_user_text(self):
        self.board.create_task("<script>alert(1)</script>", assignee="<b>x</b>")
        html = render_page(self.board.board())
        self.assertNotIn("<script>alert", html)
        self.assertIn("&lt;script&gt;", html)
        status, page = request(self.app, "GET", "/")
        self.assertEqual(status, 200)
        self.assertIn("Team Task Board", page)


if __name__ == "__main__":
    unittest.main()
@@text
## Stretch goals

Each of these is a real feature request you'd get at work:

- **Due dates**: add a migration (`ALTER TABLE tasks ADD COLUMN due TEXT`), a field, and highlight overdue cards. Notice you add a *new* migration and never edit an old one.
- **Comments and history**: a `task_events` table recording every move (who, when, from, to). Show the history in `GET /api/tasks/{id}`.
- **Users and login**: a `users` table with passwords hashed by `hashlib.scrypt`, a login form, and a session cookie (`HttpOnly`, `SameSite=Strict`). Only assignees may move their own tasks.
- **A desktop version**: a `tkinter` window that uses the same `TaskBoard`. If you need to change `service.py` to do it, the layers weren't clean.
- **Move to Flask or FastAPI**: rewrite only `web.py`. Count how many lines of `service.py` you had to change (the answer should be zero).
- **Ship it**: a `pyproject.toml` with a `task-board = "main:main"` script entry point, then a `Dockerfile`.
@@project task_board_finished Team Task Board: the finished version
A complete solution to compare with yours.
@@file models.py
"""The domain: what a task is and which rules always hold. No database, no web, no printing."""
from __future__ import annotations

from dataclasses import dataclass, field
from datetime import datetime

STATUSES = ("todo", "doing", "review", "done")
# The moves a task may make. "done" is final.
TRANSITIONS = {
    "todo": {"doing"},
    "doing": {"todo", "review"},
    "review": {"doing", "done"},
    "done": set(),
}
PRIORITIES = (1, 2, 3)  # 1 = urgent, 3 = when there's time


class ValidationError(ValueError):
    """Bad input from a user. The message is shown to them as it is."""


class NotFound(LookupError):
    """There's no task with that id."""


@dataclass
class Task:
    title: str
    priority: int = 2
    assignee: str | None = None
    tags: list[str] = field(default_factory=list)
    status: str = "todo"
    created: datetime | None = None
    done_at: datetime | None = None
    id: int | None = None

    def __post_init__(self) -> None:
        # Every Task that exists is valid: the checks run whenever one is made.
        self.title = self.title.strip()
        if not self.title:
            raise ValidationError("title is required")
        if len(self.title) > 120:
            raise ValidationError("title is too long (max 120)")
        if self.priority not in PRIORITIES:
            raise ValidationError("priority must be 1, 2 or 3")
        if self.status not in STATUSES:
            raise ValidationError(f"unknown status: {self.status}")
        self.tags = sorted({t.strip().lower() for t in self.tags if t.strip()})

    def to_dict(self) -> dict:
        return {
            "id": self.id,
            "title": self.title,
            "priority": self.priority,
            "assignee": self.assignee,
            "tags": self.tags,
            "status": self.status,
            "created": self.created.isoformat() if self.created else None,
            "done_at": self.done_at.isoformat() if self.done_at else None,
        }
@@file repo.py
"""Stores tasks in SQLite. This is the only file that knows SQL."""
from __future__ import annotations

import sqlite3
from datetime import datetime

from models import NotFound, Task

# Each entry upgrades the database by one version. Never edit an old entry: add a new one,
# because databases out in the world already ran the old ones.
MIGRATIONS = [
    """
    CREATE TABLE tasks (
        id       INTEGER PRIMARY KEY,
        title    TEXT NOT NULL,
        priority INTEGER NOT NULL DEFAULT 2,
        assignee TEXT,
        status   TEXT NOT NULL DEFAULT 'todo',
        created  TEXT NOT NULL
    );
    """,
    """
    CREATE TABLE task_tags (
        task_id INTEGER NOT NULL REFERENCES tasks(id) ON DELETE CASCADE,
        tag     TEXT NOT NULL,
        PRIMARY KEY (task_id, tag)
    );
    CREATE INDEX idx_tasks_status ON tasks(status, assignee);
    """,
    "ALTER TABLE tasks ADD COLUMN done_at TEXT;",
]


def connect(path: str = ":memory:") -> sqlite3.Connection:
    """Open (or create) the database and bring its schema up to date."""
    conn = sqlite3.connect(path)
    conn.row_factory = sqlite3.Row
    conn.execute("PRAGMA foreign_keys = ON")
    migrate(conn)
    return conn


def migrate(conn: sqlite3.Connection) -> int:
    """Run the migrations this database hasn't had yet, and return its new version.

    SQLite keeps a free integer in every database file, `PRAGMA user_version`: we use it as
    "how many migrations have run". Each migration and its version bump commit together.
    """
    version = conn.execute("PRAGMA user_version").fetchone()[0]
    for number, sql in enumerate(MIGRATIONS[version:], start=version + 1):
        conn.executescript(f"BEGIN; {sql} PRAGMA user_version = {number}; COMMIT;")
    return len(MIGRATIONS)


def _iso(value: datetime | None) -> str | None:
    return value.isoformat() if value else None


def _dt(value: str | None) -> datetime | None:
    return datetime.fromisoformat(value) if value else None


class TaskRepo:
    def __init__(self, conn: sqlite3.Connection):
        self.conn = conn

    def add(self, task: Task) -> Task:
        with self.conn:  # one transaction: the task and its tags, or nothing
            cur = self.conn.execute(
                "INSERT INTO tasks (title, priority, assignee, status, created, done_at) VALUES (?, ?, ?, ?, ?, ?)",
                (task.title, task.priority, task.assignee, task.status, _iso(task.created), _iso(task.done_at)),
            )
            task.id = cur.lastrowid
            self._save_tags(task)
        return task

    def get(self, task_id: int) -> Task:
        row = self.conn.execute("SELECT * FROM tasks WHERE id = ?", (task_id,)).fetchone()
        if row is None:
            raise NotFound(f"no task #{task_id}")
        return self._to_task(row)

    def find(self, status: str | None = None, assignee: str | None = None, tag: str | None = None) -> list[Task]:
        """Tasks matching every filter that isn't None, most urgent first, then oldest first.

        Build the SQL from fixed pieces and pass the values as ? parameters: never put user
        input into the SQL text itself.
        """
        sql = "SELECT * FROM tasks WHERE 1 = 1"
        params: list = []
        if status:
            sql += " AND status = ?"
            params.append(status)
        if assignee:
            sql += " AND assignee = ?"
            params.append(assignee)
        if tag:
            sql += " AND id IN (SELECT task_id FROM task_tags WHERE tag = ?)"
            params.append(tag.strip().lower())
        sql += " ORDER BY priority, id"
        return [self._to_task(row) for row in self.conn.execute(sql, params)]

    def update(self, task: Task) -> None:
        with self.conn:
            cur = self.conn.execute(
                "UPDATE tasks SET title = ?, priority = ?, assignee = ?, status = ?, done_at = ? WHERE id = ?",
                (task.title, task.priority, task.assignee, task.status, _iso(task.done_at), task.id),
            )
            if cur.rowcount == 0:
                raise NotFound(f"no task #{task.id}")
            self._save_tags(task)

    def delete(self, task_id: int) -> None:
        with self.conn:
            cur = self.conn.execute("DELETE FROM tasks WHERE id = ?", (task_id,))
            if cur.rowcount == 0:
                raise NotFound(f"no task #{task_id}")

    def _save_tags(self, task: Task) -> None:
        self.conn.execute("DELETE FROM task_tags WHERE task_id = ?", (task.id,))
        self.conn.executemany("INSERT INTO task_tags (task_id, tag) VALUES (?, ?)", [(task.id, t) for t in task.tags])

    def _to_task(self, row: sqlite3.Row) -> Task:
        tags = [r[0] for r in self.conn.execute("SELECT tag FROM task_tags WHERE task_id = ? ORDER BY tag", (row["id"],))]
        return Task(
            id=row["id"],
            title=row["title"],
            priority=row["priority"],
            assignee=row["assignee"],
            status=row["status"],
            tags=tags,
            created=_dt(row["created"]),
            done_at=_dt(row["done_at"]),
        )
@@file service.py
"""The use cases: what people can do with the board, and the business rules.

TaskBoard doesn't know about SQL or HTTP. It gets a repository and a clock from outside
("dependency injection"), so tests can give it an in-memory database and a fixed time.
"""
from __future__ import annotations

from datetime import datetime, timedelta
from typing import Callable

from models import STATUSES, TRANSITIONS, Task, ValidationError

WIP_LIMIT = 3  # how many tasks one person may have in "doing" at once


def _priority(value) -> int:
    try:
        return int(value)
    except (TypeError, ValueError):
        raise ValidationError("priority must be 1, 2 or 3") from None


class TaskBoard:
    def __init__(self, repo, clock: Callable[[], datetime] = datetime.now):
        self.repo = repo
        self.clock = clock

    def create_task(self, title: str, priority=2, assignee: str | None = None, tags=()) -> Task:
        task = Task(
            title=title,
            priority=_priority(priority),
            assignee=(assignee or "").strip() or None,
            tags=list(tags),
            created=self.clock(),
        )
        return self.repo.add(task)

    def tasks(self, status=None, assignee=None, tag=None) -> list[Task]:
        return self.repo.find(status=status, assignee=assignee, tag=tag)

    def assign(self, task_id: int, assignee: str | None) -> Task:
        task = self.repo.get(task_id)
        task.assignee = (assignee or "").strip() or None
        self.repo.update(task)
        return task

    def move(self, task_id: int, status: str) -> Task:
        """Move a task to a new column, enforcing the rules:

        - the status must exist, and the move must be in TRANSITIONS (ValidationError
          "can't move from todo to done" otherwise),
        - a task can only start ("doing") once someone is assigned ("assign the task before
          starting it"),
        - nobody may have more than WIP_LIMIT tasks in "doing" ("yuki already has 3 tasks
          in progress"),
        - moving to "done" records done_at = now; moving anywhere else clears it.
        """
        task = self.repo.get(task_id)
        if status not in STATUSES:
            raise ValidationError(f"unknown status: {status}")
        if status not in TRANSITIONS[task.status]:
            raise ValidationError(f"can't move from {task.status} to {status}")
        if status == "doing":
            if not task.assignee:
                raise ValidationError("assign the task before starting it")
            busy = self.repo.find(status="doing", assignee=task.assignee)
            if len(busy) >= WIP_LIMIT:
                raise ValidationError(f"{task.assignee} already has {WIP_LIMIT} tasks in progress")
        task.status = status
        task.done_at = self.clock() if status == "done" else None
        self.repo.update(task)
        return task

    def delete(self, task_id: int) -> None:
        self.repo.delete(task_id)

    def board(self) -> dict[str, list[Task]]:
        """Every task, grouped by status, with all four columns present even when empty."""
        columns: dict[str, list[Task]] = {s: [] for s in STATUSES}
        for task in self.repo.find():
            columns[task.status].append(task)
        return columns

    def stats(self, days: int = 7) -> dict:
        """Numbers for the team's weekly meeting:

        {"counts": {"todo": 2, "doing": 1, "review": 0, "done": 3},
         "done_recently": 2,          # finished in the last `days` days
         "avg_cycle_hours": 30.5}     # created -> done, for those tasks; None if there are none
        """
        now = self.clock()
        tasks = self.repo.find()
        counts = {s: 0 for s in STATUSES}
        for task in tasks:
            counts[task.status] += 1
        since = now - timedelta(days=days)
        recent = [t for t in tasks if t.done_at and t.done_at >= since]
        hours = [(t.done_at - t.created).total_seconds() / 3600 for t in recent]
        return {
            "counts": counts,
            "done_recently": len(recent),
            "avg_cycle_hours": round(sum(hours) / len(hours), 1) if hours else None,
        }
@@file web.py
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
@@file main.py
"""Team Task Board: a small Kanban app for an IT team.

python main.py serve                      open http://localhost:8000
python main.py add "Renew TLS cert" --priority 1 --assignee yuki --tag web
python main.py list
python main.py move 1 doing
python main.py stats
"""
from __future__ import annotations

import argparse
import json
import sys
from wsgiref.simple_server import make_server

from models import NotFound, ValidationError
from repo import TaskRepo, connect
from service import TaskBoard
from web import make_app


def main(argv: list[str] | None = None) -> int:
    parser = argparse.ArgumentParser(description="Team Task Board")
    parser.add_argument("--db", default="tasks.db", help="database file (default: tasks.db)")
    sub = parser.add_subparsers(dest="cmd", required=True)
    serve = sub.add_parser("serve", help="run the web app")
    serve.add_argument("--port", type=int, default=8000)
    add = sub.add_parser("add", help="add a task")
    add.add_argument("title")
    add.add_argument("--priority", type=int, default=2, choices=[1, 2, 3])
    add.add_argument("--assignee")
    add.add_argument("--tag", action="append", default=[])
    sub.add_parser("list", help="show the board")
    move = sub.add_parser("move", help="move a task to another column")
    move.add_argument("id", type=int)
    move.add_argument("status")
    assign = sub.add_parser("assign", help="assign a task to someone")
    assign.add_argument("id", type=int)
    assign.add_argument("assignee")
    sub.add_parser("stats", help="numbers for the weekly meeting")
    args = parser.parse_args(argv)

    board = TaskBoard(TaskRepo(connect(args.db)))
    try:
        if args.cmd == "serve":
            print(f"Task board on http://localhost:{args.port}  (Ctrl+C to stop)")
            with make_server("127.0.0.1", args.port, make_app(board)) as server:
                server.serve_forever()
        elif args.cmd == "add":
            task = board.create_task(args.title, args.priority, args.assignee, args.tag)
            print(f"added #{task.id}: {task.title}")
        elif args.cmd == "list":
            for status, tasks in board.board().items():
                print(f"== {status} ({len(tasks)})")
                for t in tasks:
                    who = f" @{t.assignee}" if t.assignee else ""
                    tags = "".join(f" #{tag}" for tag in t.tags)
                    print(f"  #{t.id} P{t.priority} {t.title}{who}{tags}")
        elif args.cmd == "move":
            task = board.move(args.id, args.status)
            print(f"#{task.id} is now {task.status}")
        elif args.cmd == "assign":
            task = board.assign(args.id, args.assignee)
            print(f"#{task.id} is assigned to {task.assignee}")
        elif args.cmd == "stats":
            print(json.dumps(board.stats(), indent=2))
    except (ValidationError, NotFound) as e:
        print(f"error: {e}", file=sys.stderr)
        return 1
    except KeyboardInterrupt:
        pass
    return 0


if __name__ == "__main__":
    sys.exit(main())
@@file test_board.py
"""Run: python -m unittest -v"""
import io
import json
import unittest
from datetime import datetime, timedelta
from wsgiref.util import setup_testing_defaults

from models import NotFound, Task, ValidationError
from repo import MIGRATIONS, TaskRepo, connect, migrate
from service import WIP_LIMIT, TaskBoard
from web import HttpError, make_app, match, render_page


class FakeClock:
    """A clock the test controls: time only moves when we say so."""

    def __init__(self):
        self.now = datetime(2026, 11, 2, 9, 0)

    def __call__(self):
        return self.now

    def advance(self, **kwargs):
        self.now += timedelta(**kwargs)


def new_board():
    clock = FakeClock()
    return TaskBoard(TaskRepo(connect(":memory:")), clock), clock


class ModelTests(unittest.TestCase):
    def test_valid_task_is_cleaned(self):
        t = Task(title="  Renew cert ", tags=["Web", "web", " TLS ", ""])
        self.assertEqual(t.title, "Renew cert")
        self.assertEqual(t.tags, ["tls", "web"])

    def test_invalid_tasks_cannot_exist(self):
        for kwargs in ({"title": "  "}, {"title": "x" * 121}, {"title": "ok", "priority": 5}, {"title": "ok", "status": "lost"}):
            with self.assertRaises(ValidationError):
                Task(**kwargs)


class RepoTests(unittest.TestCase):
    def setUp(self):
        self.conn = connect(":memory:")
        self.repo = TaskRepo(self.conn)
        when = datetime(2026, 11, 2, 9, 0)
        self.repo.add(Task(title="Patch web01", priority=1, assignee="yuki", tags=["linux", "patching"], created=when))
        self.repo.add(Task(title="Order toner", priority=3, created=when))
        self.repo.add(Task(title="Patch db01", priority=1, assignee="ken", tags=["linux"], status="doing", created=when))

    def test_migrations_bring_schema_to_latest(self):
        self.assertEqual(self.conn.execute("PRAGMA user_version").fetchone()[0], len(MIGRATIONS))
        self.assertEqual(migrate(self.conn), len(MIGRATIONS))  # running again changes nothing

    def test_round_trip(self):
        t = self.repo.get(1)
        self.assertEqual((t.title, t.priority, t.assignee, t.tags), ("Patch web01", 1, "yuki", ["linux", "patching"]))
        self.assertEqual(t.created, datetime(2026, 11, 2, 9, 0))

    def test_find_filters_and_order(self):
        self.assertEqual([t.id for t in self.repo.find()], [1, 3, 2])  # priority, then id
        self.assertEqual([t.id for t in self.repo.find(tag="LINUX")], [1, 3])
        self.assertEqual([t.id for t in self.repo.find(status="doing")], [3])
        self.assertEqual([t.id for t in self.repo.find(assignee="yuki", tag="linux")], [1])
        self.assertEqual(self.repo.find(tag="windows"), [])

    def test_find_is_safe_from_sql_injection(self):
        self.assertEqual(self.repo.find(assignee="x' OR '1'='1"), [])

    def test_update_and_delete(self):
        t = self.repo.get(2)
        t.tags = ["printer"]
        self.repo.update(t)
        self.assertEqual(self.repo.get(2).tags, ["printer"])
        self.repo.delete(2)
        with self.assertRaises(NotFound):
            self.repo.get(2)
        with self.assertRaises(NotFound):
            self.repo.delete(2)
        self.assertEqual(self.conn.execute("SELECT COUNT(*) FROM task_tags WHERE task_id = 2").fetchone()[0], 0)


class ServiceTests(unittest.TestCase):
    def setUp(self):
        self.board, self.clock = new_board()

    def test_create_validates(self):
        t = self.board.create_task("Renew TLS cert", "1", " yuki ", ["web"])
        self.assertEqual((t.id, t.priority, t.assignee, t.status), (1, 1, "yuki", "todo"))
        with self.assertRaises(ValidationError):
            self.board.create_task("x", priority="urgent")

    def test_happy_path_to_done(self):
        t = self.board.create_task("Renew TLS cert", assignee="yuki")
        self.board.move(t.id, "doing")
        self.board.move(t.id, "review")
        self.clock.advance(hours=5)
        done = self.board.move(t.id, "done")
        self.assertEqual(done.status, "done")
        self.assertEqual(done.done_at, datetime(2026, 11, 2, 14, 0))

    def test_illegal_moves(self):
        t = self.board.create_task("Renew TLS cert", assignee="yuki")
        with self.assertRaisesRegex(ValidationError, "can't move from todo to done"):
            self.board.move(t.id, "done")
        with self.assertRaisesRegex(ValidationError, "unknown status"):
            self.board.move(t.id, "archived")
        with self.assertRaises(NotFound):
            self.board.move(99, "doing")

    def test_must_be_assigned_to_start(self):
        t = self.board.create_task("Nobody's task")
        with self.assertRaisesRegex(ValidationError, "assign the task"):
            self.board.move(t.id, "doing")
        self.board.assign(t.id, "ken")
        self.assertEqual(self.board.move(t.id, "doing").status, "doing")

    def test_wip_limit(self):
        ids = [self.board.create_task(f"Task {i}", assignee="yuki").id for i in range(WIP_LIMIT + 1)]
        for task_id in ids[:WIP_LIMIT]:
            self.board.move(task_id, "doing")
        with self.assertRaisesRegex(ValidationError, f"yuki already has {WIP_LIMIT} tasks in progress"):
            self.board.move(ids[-1], "doing")
        other = self.board.create_task("Ken's task", assignee="ken")
        self.board.move(other.id, "doing")  # the limit is per person

    def test_moving_back_clears_done_at(self):
        t = self.board.create_task("Review firewall rules", assignee="ken")
        self.board.move(t.id, "doing")
        self.board.move(t.id, "review")
        back = self.board.move(t.id, "doing")
        self.assertIsNone(back.done_at)

    def test_board_has_every_column(self):
        self.board.create_task("A")
        cols = self.board.board()
        self.assertEqual(list(cols), ["todo", "doing", "review", "done"])
        self.assertEqual([t.title for t in cols["todo"]], ["A"])
        self.assertEqual(cols["done"], [])

    def test_stats(self):
        self.assertEqual(self.board.stats(), {"counts": {"todo": 0, "doing": 0, "review": 0, "done": 0},
                                              "done_recently": 0, "avg_cycle_hours": None})
        a = self.board.create_task("A", assignee="yuki")
        b = self.board.create_task("B", assignee="yuki")
        self.board.create_task("C")
        for task, hours in ((a, 10), (b, 20)):
            self.board.move(task.id, "doing")
            self.board.move(task.id, "review")
            self.clock.advance(hours=hours)
            self.board.move(task.id, "done")
        stats = self.board.stats()
        self.assertEqual(stats["counts"], {"todo": 1, "doing": 0, "review": 0, "done": 2})
        self.assertEqual(stats["done_recently"], 2)
        self.assertEqual(stats["avg_cycle_hours"], 20.0)  # a: 10 h, b: 30 h (created at the start)
        self.clock.advance(days=8)
        self.assertEqual(self.board.stats()["done_recently"], 0)


def request(app, method, path, body=None, query=""):
    environ = {}
    setup_testing_defaults(environ)
    data = json.dumps(body).encode() if body is not None else b""
    environ.update(REQUEST_METHOD=method, PATH_INFO=path, QUERY_STRING=query,
                   CONTENT_LENGTH=str(len(data)), **{"wsgi.input": io.BytesIO(data)})
    seen = {}

    def start_response(status, headers):
        seen["status"], seen["headers"] = status, dict(headers)

    raw = b"".join(app(environ, start_response))
    is_json = seen["headers"]["Content-Type"] == "application/json"
    return int(seen["status"][:3]), (json.loads(raw) if is_json and raw else raw.decode())


class WebTests(unittest.TestCase):
    def setUp(self):
        self.board, _ = new_board()
        self.app = make_app(self.board)

    def test_match(self):
        self.assertEqual(match("GET", "/api/tasks"), ("list_tasks", {}))
        self.assertEqual(match("POST", "/api/tasks/12/move"), ("move_task", {"id": "12"}))
        with self.assertRaises(HttpError) as e:
            match("GET", "/api/nothing")
        self.assertEqual(e.exception.status, 404)
        with self.assertRaises(HttpError) as e:
            match("PUT", "/api/tasks")
        self.assertEqual(e.exception.status, 405)
        with self.assertRaises(HttpError):
            match("GET", "/api/tasks/1/2")  # {id} never matches a "/"

    def test_api_flow(self):
        status, task = request(self.app, "POST", "/api/tasks", {"title": "Renew TLS cert", "assignee": "yuki", "tags": ["web"]})
        self.assertEqual((status, task["id"], task["status"]), (201, 1, "todo"))
        status, moved = request(self.app, "POST", "/api/tasks/1/move", {"status": "doing"})
        self.assertEqual((status, moved["status"]), (200, "doing"))
        status, tasks = request(self.app, "GET", "/api/tasks", query="tag=web")
        self.assertEqual((status, [t["title"] for t in tasks]), (200, ["Renew TLS cert"]))
        self.assertEqual(request(self.app, "DELETE", "/api/tasks/1"), (204, ""))
        self.assertEqual(request(self.app, "GET", "/api/tasks/1")[0], 404)

    def test_errors_are_json(self):
        self.assertEqual(request(self.app, "POST", "/api/tasks", {"title": ""}), (400, {"error": "title is required"}))
        self.assertEqual(request(self.app, "POST", "/api/tasks/abc/move", {"status": "doing"})[0], 404)
        self.assertEqual(request(self.app, "GET", "/nope"), (404, {"error": "not found"}))

    def test_page_escapes_user_text(self):
        self.board.create_task("<script>alert(1)</script>", assignee="<b>x</b>")
        html = render_page(self.board.board())
        self.assertNotIn("<script>alert", html)
        self.assertIn("&lt;script&gt;", html)
        status, page = request(self.app, "GET", "/")
        self.assertEqual(status, 200)
        self.assertIn("Team Task Board", page)


if __name__ == "__main__":
    unittest.main()
@@text
🎉 You've written a layered application with a schema that can evolve, business rules with tests, an API and two user interfaces. That's the shape of most business software, and you'll recognise it in every Django, Rails, Spring or .NET codebase you open.
