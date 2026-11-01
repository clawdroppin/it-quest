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
