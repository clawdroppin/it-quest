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
