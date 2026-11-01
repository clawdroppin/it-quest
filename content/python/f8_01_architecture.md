@@lesson f8-architecture From Scripts to Applications: Architecture
@@text
So far you've written **scripts**: run once, do a job, exit. An **application** is different. It runs for a long time, many people use it, it stores data, it grows new features every month, and other developers have to change it without breaking it. The code that makes this possible isn't harder Python. It's **structure**.

## What changes

| Script | Application |
|---|---|
| one file, top to bottom | many modules, each with one job |
| `print()` and `input()` everywhere | logic returns values; only the interface prints |
| data in variables, maybe a CSV | a database whose schema changes over time |
| "it worked when I ran it" | automated tests run on every change |
| one user: you | many users, bad input, errors that must not crash it |

## Layers

Almost every business application, in any language, is split into the same layers:

```text
  interface      web pages, JSON API, CLI, desktop GUI      (thin: translate, don't decide)
      │
  use cases      "move a task", "approve a leave request"   (the business rules)
      │
  domain         Task, Invoice, User: data + rules that always hold
      │
  storage        SQL, files, other APIs                     (swappable)
```

The golden rule: **dependencies point inward**. The domain doesn't import the database; the business rules don't know whether they were called from a web page or a test. That's what lets you add a mobile app, switch from SQLite to PostgreSQL, or test everything in milliseconds.

## The domain: dataclasses that can't be invalid

A `dataclass` writes `__init__`, `__repr__` and `__eq__` for you. Add `__post_init__` to check the data, so an invalid object **can't exist**: every other part of the program can then trust it.

```python
from dataclasses import dataclass, field
from enum import Enum


class Status(Enum):
    TODO = "todo"
    DOING = "doing"
    DONE = "done"


@dataclass
class Ticket:
    title: str
    priority: int = 3
    status: Status = Status.TODO
    tags: list[str] = field(default_factory=list)   # never use a bare [] as a default

    def __post_init__(self):
        self.title = self.title.strip()
        if not self.title:
            raise ValueError("title is required")
        if not 1 <= self.priority <= 4:
            raise ValueError("priority must be 1-4")


t = Ticket("  VPN drops every hour ", priority=2)
print(t)
print(t.status.value)
try:
    Ticket("", priority=9)
except ValueError as e:
    print("rejected:", e)
```

An `Enum` instead of plain strings means a typo like `Status.DOEN` fails immediately instead of silently creating a fourth status.

## Use cases: dependency injection

A service class holds the business rules. It **receives** what it needs (a repository, a clock, a mailer) instead of creating it. That's all "dependency injection" means.

```python
from datetime import datetime


class InMemoryRepo:
    def __init__(self):
        self.items = {}
        self.next_id = 1

    def add(self, item):
        item["id"] = self.next_id
        self.items[self.next_id] = item
        self.next_id += 1
        return item


class LeaveService:
    def __init__(self, repo, clock=datetime.now):
        self.repo = repo        # given to us: a real database, or a fake one in tests
        self.clock = clock      # given to us: the real time, or a fixed one in tests

    def request_leave(self, user, days):
        if days > 20:
            raise ValueError("more than 20 days needs HR approval")
        return self.repo.add({"user": user, "days": days, "at": self.clock().isoformat()})


fixed = lambda: datetime(2026, 11, 2, 9, 0)
service = LeaveService(InMemoryRepo(), clock=fixed)
print(service.request_leave("yuki", 3))
```

In production you'd write `LeaveService(SqlRepo(conn))`; in tests, `LeaveService(InMemoryRepo(), clock=fixed)`. The service code is identical. Python's "duck typing" means the two repos only need the same methods. To make the contract explicit, write a `typing.Protocol`:

```python,norun
from typing import Protocol

class Repo(Protocol):
    def add(self, item: dict) -> dict: ...
    def get(self, item_id: int) -> dict: ...
```

## Errors that mean something

Define your own exception classes for the problems users cause, so the interface can turn them into the right response (HTTP 400 vs 404 vs 500):

```python
class AppError(Exception):
    """Base class for errors we expect and can explain."""

class ValidationError(AppError):
    """The user sent bad input (HTTP 400)."""

class NotFound(AppError):
    """The thing they asked for doesn't exist (HTTP 404)."""


def handle(fn):
    try:
        return 200, fn()
    except ValidationError as e:
        return 400, str(e)
    except NotFound as e:
        return 404, str(e)


def get_user(user_id):
    raise NotFound(f"no user #{user_id}")

print(handle(lambda: get_user(7)))
```

Anything that *isn't* an `AppError` is a bug: log it with the full traceback and show the user a plain "something went wrong".

## Configuration and logging

- **Config** comes from environment variables (and maybe a config file), read in *one* place at startup, checked, and passed in. Never `os.environ[...]` deep in the business logic.
- **Logging** instead of `print`: levels, timestamps, and output that can go to a file or a log system.

```python
import logging
import os

logging.basicConfig(level=os.environ.get("LOG_LEVEL", "INFO"), format="%(asctime)s %(levelname)s %(name)s: %(message)s")
log = logging.getLogger("tasks")

log.debug("only shown with LOG_LEVEL=DEBUG")
log.info("task %s moved to %s", 42, "done")
log.warning("WIP limit reached for %s", "yuki")
```

Pass values as arguments (`log.info("task %s", task_id)`), not f-strings: the message is only built if that level is enabled, and log systems can group messages by their template.

## A project layout that scales

```text
task-board/
├── pyproject.toml
├── src/task_board/
│   ├── models.py        domain
│   ├── service.py       use cases
│   ├── repo.py          storage
│   ├── web.py           interface (HTTP)
│   └── cli.py           interface (command line)
└── tests/
    ├── test_service.py  fast: fake repo, fake clock
    └── test_repo.py     uses a real SQLite :memory: database
```

You'll build exactly this in the **Team Task Board** project.

@@quiz
In a layered application, which layer should contain the rule "a task needs an assignee before it can start"?
- The web layer, because that's where the button is
- The database, as a trigger
* The use-case (service) layer
- The CLI
? Business rules live in one place, below every interface. Then the web page, the CLI and the API all obey the same rule.

@@quiz
Why does `LeaveService` take `clock` as a parameter instead of calling `datetime.now()` itself?
- It's faster
* Tests can pass a fixed time, so time-based rules can be tested reliably
- `datetime.now()` doesn't work inside classes
- To support time zones automatically
? Anything the code can't control (time, randomness, the network, the database) is passed in, so tests can control it.

@@quiz
What does `__post_init__` in a dataclass let you do?
- Run code after the program exits
* Check and clean the fields right after the object is created
- Define the order of the fields
- Make the object immutable
? `__post_init__` runs at the end of the generated `__init__`. Raising there means an invalid object is never created.

@@quiz
Which error should become HTTP 500 "internal error"?
- A `ValidationError` for an empty title
- A `NotFound` for task #99
* A `KeyError` nobody expected, from a bug in the code
- A wrong password
? Expected problems get specific answers (400, 404, 401). An unexpected exception is a bug: log it, and don't show the details to users.

@@flashcards
layered architecture :: interface → use cases → domain, with storage swappable; dependencies point inward
dependency injection :: give an object what it needs (repo, clock) instead of letting it create them
`@dataclass` :: generates `__init__`, `__repr__`, `__eq__` from the fields
`__post_init__` :: dataclass hook to validate fields after `__init__`
`field(default_factory=list)` :: a fresh list per object (never a shared `[]` default)
`Enum` :: a fixed set of named values; typos fail immediately
`typing.Protocol` :: describes the methods an object must have (structural typing)
custom exceptions :: `ValidationError` → 400, `NotFound` → 404, anything else → bug, 500
`logging.getLogger(__name__)` :: a named logger; configure levels and format once at startup

@@exercise A service with injected dependencies
Write `ShiftService`, which books on-call shifts. Its constructor takes a `repo` (any object with `add(shift)` and `for_person(name)` methods) and a `clock` (a function returning a `datetime`). `book(name, hours)` must:

- raise `ValueError("hours must be 1-12")` unless `1 <= hours <= 12`,
- raise `ValueError("weekly limit is 40 hours")` if the person's existing shifts plus this one would be over 40 hours,
- otherwise add and return the dict `{"name": name, "hours": hours, "booked_at": <clock() as ISO text>}`.
@@starter
from datetime import datetime


class FakeRepo:
    def __init__(self):
        self.shifts = []

    def add(self, shift):
        self.shifts.append(shift)
        return shift

    def for_person(self, name):
        return [s for s in self.shifts if s["name"] == name]


class ShiftService:
    def __init__(self, repo, clock=datetime.now):
        pass

    def book(self, name, hours):
        pass


service = ShiftService(FakeRepo(), clock=lambda: datetime(2026, 11, 2, 9, 0))
print(service.book("yuki", 8))
@@tests
_repo = FakeRepo()
_svc = ShiftService(_repo, clock=lambda: datetime(2026, 11, 2, 9, 0))
check(_svc.book("yuki", 8), {"name": "yuki", "hours": 8, "booked_at": "2026-11-02T09:00:00"}, "book a shift")
for _ in range(3):
    _svc.book("yuki", 10)
try:
    _svc.book("yuki", 3)
    _msg = "no error"
except ValueError as e:
    _msg = str(e)
check(_msg, "weekly limit is 40 hours", "38 + 3 hours is over the limit")
check(_svc.book("yuki", 2)["hours"], 2, "38 + 2 = 40 is allowed")
check(_svc.book("ken", 12)["name"], "ken", "the limit is per person")
try:
    _svc.book("ken", 13)
    _msg = "no error"
except ValueError as e:
    _msg = str(e)
check(_msg, "hours must be 1-12", "a 13-hour shift")
check(len(_repo.shifts), 6, "rejected shifts are not saved")
@@hint
In `__init__`, store `self.repo = repo` and `self.clock = clock`. In `book`, check the hours first, then `used = sum(s["hours"] for s in self.repo.for_person(name))` and compare `used + hours > 40`. Build the dict with `self.clock().isoformat()` and `return self.repo.add(shift)`.
@@solution
from datetime import datetime


class FakeRepo:
    def __init__(self):
        self.shifts = []

    def add(self, shift):
        self.shifts.append(shift)
        return shift

    def for_person(self, name):
        return [s for s in self.shifts if s["name"] == name]


class ShiftService:
    def __init__(self, repo, clock=datetime.now):
        self.repo = repo
        self.clock = clock

    def book(self, name, hours):
        if not 1 <= hours <= 12:
            raise ValueError("hours must be 1-12")
        used = sum(s["hours"] for s in self.repo.for_person(name))
        if used + hours > 40:
            raise ValueError("weekly limit is 40 hours")
        shift = {"name": name, "hours": hours, "booked_at": self.clock().isoformat()}
        return self.repo.add(shift)


service = ShiftService(FakeRepo(), clock=lambda: datetime(2026, 11, 2, 9, 0))
print(service.book("yuki", 8))

@@exercise A domain class that can't be invalid
Write a dataclass `Server` with fields `hostname: str`, `ip: str`, `role: str = "web"` and `tags: list = field(default_factory=list)`. In `__post_init__`:

- lower-case and strip the hostname; it must match `^[a-z0-9-]{1,63}$`, otherwise raise `ValueError("bad hostname")`,
- the ip must be a valid IPv4 address (use the `ipaddress` module), otherwise raise `ValueError("bad ip")`,
- role must be one of `web`, `db`, `cache`, otherwise raise `ValueError("bad role")`,
- tags become a sorted list without duplicates.
@@starter
import ipaddress
import re
from dataclasses import dataclass, field


@dataclass
class Server:
    hostname: str
    ip: str
    role: str = "web"
    tags: list = field(default_factory=list)

    def __post_init__(self):
        pass


print(Server(" WEB01 ", "10.0.0.11", tags=["prod", "tokyo", "prod"]))
@@tests
_s = Server(" WEB01 ", "10.0.0.11", tags=["prod", "tokyo", "prod"])
check((_s.hostname, _s.tags), ("web01", ["prod", "tokyo"]), "cleaned fields")
def _err(*args, **kwargs):
    try:
        Server(*args, **kwargs)
        return "no error"
    except ValueError as e:
        return str(e)
check(_err("web_01", "10.0.0.1"), "bad hostname", "underscore in hostname")
check(_err("web01", "10.0.0.256"), "bad ip", "256 is not a valid octet")
check(_err("web01", "10.0.0.1", role="mail"), "bad role", "unknown role")
check(Server("db01", "192.168.1.5", "db").role, "db", "a valid db server")
@@hint
`self.hostname = self.hostname.strip().lower()` then `if not re.fullmatch(r"[a-z0-9-]{1,63}", self.hostname): raise ValueError("bad hostname")`. For the IP: `try: ipaddress.IPv4Address(self.ip) except ValueError: raise ValueError("bad ip")`. Tags: `sorted(set(self.tags))`.
@@solution
import ipaddress
import re
from dataclasses import dataclass, field


@dataclass
class Server:
    hostname: str
    ip: str
    role: str = "web"
    tags: list = field(default_factory=list)

    def __post_init__(self):
        self.hostname = self.hostname.strip().lower()
        if not re.fullmatch(r"[a-z0-9-]{1,63}", self.hostname):
            raise ValueError("bad hostname")
        try:
            ipaddress.IPv4Address(self.ip)
        except ValueError:
            raise ValueError("bad ip") from None
        if self.role not in ("web", "db", "cache"):
            raise ValueError("bad role")
        self.tags = sorted(set(self.tags))


print(Server(" WEB01 ", "10.0.0.11", tags=["prod", "tokyo", "prod"]))
