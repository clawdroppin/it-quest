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
