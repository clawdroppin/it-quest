OUT = "content/python/b4_01_task_board.md"
BASE = "tools/appdev/py_task_board"
FILES = ["models.py", "repo.py", "service.py", "web.py", "main.py", "test_board.py"]

PARTS = [
    ("text", r'''
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
'''),
    ("project", "task_board", "Team Task Board",
     "Fill in the four `NotImplementedError` pieces until `python -m unittest -v` passes, then run `python main.py serve`.",
     BASE + "/starter", FILES),
    ("text", r'''
@@text
## Stretch goals

Each of these is a real feature request you'd get at work:

- **Due dates**: add a migration (`ALTER TABLE tasks ADD COLUMN due TEXT`), a field, and highlight overdue cards. Notice you add a *new* migration and never edit an old one.
- **Comments and history**: a `task_events` table recording every move (who, when, from, to). Show the history in `GET /api/tasks/{id}`.
- **Users and login**: a `users` table with passwords hashed by `hashlib.scrypt`, a login form, and a session cookie (`HttpOnly`, `SameSite=Strict`). Only assignees may move their own tasks.
- **A desktop version**: a `tkinter` window that uses the same `TaskBoard`. If you need to change `service.py` to do it, the layers weren't clean.
- **Move to Flask or FastAPI**: rewrite only `web.py`. Count how many lines of `service.py` you had to change (the answer should be zero).
- **Ship it**: a `pyproject.toml` with a `task-board = "main:main"` script entry point, then a `Dockerfile`.
'''),
    ("project", "task_board_finished", "Team Task Board: the finished version",
     "A complete solution to compare with yours.",
     BASE + "/finished", FILES),
    ("text", r'''
@@text
🎉 You've written a layered application with a schema that can evolve, business rules with tests, an API and two user interfaces. That's the shape of most business software, and you'll recognise it in every Django, Rails, Spring or .NET codebase you open.
'''),
]
