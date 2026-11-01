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
