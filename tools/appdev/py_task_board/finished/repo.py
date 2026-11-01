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
