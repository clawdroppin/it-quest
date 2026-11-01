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
