@@lesson p9-enums-match Enums and Structural Pattern Matching
@@concepts objects, conditions
@@text
Two features that let Python model data the way Rust does with `enum` and `match`: **enums** for fixed sets of named values, and **`match`/`case`** (Python 3.10+) for taking data apart by its shape.

## Enums

Instead of magic strings like `"hihg"` (typo!) scattered through the code, define the allowed values once:

```python
from enum import Enum, IntEnum, StrEnum, Flag, auto

class Severity(IntEnum):        # IntEnum: members compare like ints
    LOW = 1
    MEDIUM = 2
    HIGH = 3
    CRITICAL = 4

class Status(StrEnum):          # StrEnum (3.11+): members are also strings
    OPEN = auto()               # auto() -> "open"
    IN_PROGRESS = auto()
    CLOSED = auto()

print(Severity.HIGH, Severity.HIGH.value, Severity["LOW"], Severity(4))
print(Severity.HIGH > Severity.MEDIUM, max(Severity))
print(Status.OPEN == "open", [s.value for s in Status])
```

- Iterate over members, look up by name (`Severity["LOW"]`) or by value (`Severity(4)`).
- A wrong value fails loudly: `Severity(9)` raises `ValueError`.
- Plain `Enum` members only compare equal to themselves (safest); `IntEnum`/`StrEnum` mix in int/str behaviour, handy for JSON and databases.
- Enums can have methods and properties like any class.

### Flags: combinable options

```python
from enum import Flag, auto

class Perm(Flag):
    READ = auto()
    WRITE = auto()
    EXECUTE = auto()

p = Perm.READ | Perm.WRITE
print(p, Perm.WRITE in p, Perm.EXECUTE in p)
```

## match / case

`match` compares a value against **patterns**, top to bottom, and runs the first that fits:

```python
def http_meaning(code):
    match code:
        case 200 | 201 | 204:
            return "success"
        case 301 | 302:
            return "redirect"
        case 404:
            return "not found"
        case int(n) if 500 <= n < 600:     # class pattern + guard
            return "server error"
        case _:                            # wildcard: anything else
            return "other"

print([http_meaning(c) for c in (200, 302, 404, 503, 418)])
```

Its real power is **destructuring** sequences and dicts:

```python
def run(command):
    match command.split():
        case ["restart", service]:
            return f"restarting {service}"
        case ["restart", *services] if services:
            return f"restarting {len(services)} services"
        case ["logs", service, "--tail", n]:
            return f"last {n} lines of {service}"
        case ["help" | "?"]:
            return "commands: restart, logs"
        case []:
            return "empty command"
        case _:
            return f"unknown: {command}"

for c in ["restart nginx", "restart a b c", "logs web --tail 50", "?", "", "fly"]:
    print(run(c))
```

Patterns for dicts (JSON events) and classes:

```python
from dataclasses import dataclass

@dataclass
class Alert:
    host: str
    severity: int

def route(event):
    match event:
        case {"type": "login", "user": user, "ok": False}:
            return f"failed login: {user}"
        case {"type": "disk", "free": free} if free < 5:
            return f"disk almost full ({free}%)"
        case Alert(host=h, severity=4):
            return f"page on-call for {h}"
        case Alert(host=h):
            return f"ticket for {h}"
        case _:
            return "ignore"

print(route({"type": "login", "user": "maya", "ok": False, "ip": "1.2.3.4"}))
print(route({"type": "disk", "free": 3}))
print(route(Alert("db01", 4)), route(Alert("web01", 2)), route(42))
```

Rules to remember:

- A **bare name** in a pattern (`service`) **captures** the value; it doesn't compare against a variable. To compare against a constant, use a dotted name: `case Severity.HIGH:`.
- Dict patterns match if the listed keys exist; extra keys are ignored.
- Sequence patterns don't match strings (a string isn't treated as a list of characters).
- `match` doesn't check exhaustiveness like Rust; add `case _:`. (Type checkers can check enums with `assert_never`.)

@@quiz
What does `Severity(4)` return for an IntEnum with CRITICAL = 4?
* `Severity.CRITICAL`
- 4
- "CRITICAL"
- ValueError
? Calling the enum looks up a member by value.

@@quiz
In `case ["restart", service]:`, what does `service` do?
- Compares with a variable named service
* Captures the second word into the variable `service`
- Must be a string literal
- Matches only "service"
? Bare names capture; use dotted names like `Mode.X` for constants.

@@quiz
Does `case {"type": "disk", "free": free}` match `{"type": "disk", "free": 3, "host": "a"}`?
* Yes, extra keys are ignored
- No, keys must match exactly
- Only with `**rest`
- Only if host is captured
? Dict patterns check only the listed keys.

@@quiz
Which enum type lets you combine members with `|`?
- IntEnum
- StrEnum
* Flag
- Enum
? `Perm.READ | Perm.WRITE`.

@@drill
Which keyword starts a structural pattern-matching block?
= match
? Each pattern is a case.

@@flashcards
Enum :: a fixed set of named constant members
IntEnum / StrEnum :: enum members that also behave like ints / strings
`auto()` :: assigns member values automatically
Flag :: enum whose members combine with `|`
`match` / `case` :: structural pattern matching (Python 3.10+)
`case _:` :: wildcard pattern matching anything
Guard :: `case pattern if condition:`
Capture pattern :: a bare name in a pattern binds the matched value
`[first, *rest]` :: sequence pattern capturing the remainder

@@exercise Ticket priority with enums
Write:

- `class Impact(IntEnum)`: `LOW=1, MEDIUM=2, HIGH=3`, and `class Urgency(IntEnum)` the same.
- `class Priority(StrEnum)`: `P1`, `P2`, `P3`, `P4` (values `"P1"` … `"P4"`).
- `priority(impact, urgency)`: score = impact + urgency; 6 → P1, 5 → P2, 4 → P3, anything lower → P4.
- `parse(text)`: `"high/low"` → `(Impact.HIGH, Urgency.LOW)`, case-insensitive; an unknown name raises `ValueError`.
@@starter
from enum import IntEnum, StrEnum


def priority(impact, urgency):
    pass


def parse(text):
    pass


i, u = parse("High/medium")
print(priority(i, u), priority(Impact.LOW, Urgency.LOW))
@@tests
check(priority(Impact.HIGH, Urgency.HIGH), Priority.P1, "P1")
check(priority(Impact.HIGH, Urgency.MEDIUM), "P2", "StrEnum equals its string")
check(priority(Impact.MEDIUM, Urgency.MEDIUM), Priority.P3, "P3")
check(priority(Impact.LOW, Urgency.MEDIUM), Priority.P4, "P4")
check(parse("High/medium"), (Impact.HIGH, Urgency.MEDIUM), "parse")
check(Impact.HIGH > Impact.LOW, True, "ordering")
try:
    parse("huge/low")
    check("parsed", "ValueError", "unknown impact")
except ValueError:
    pass
@@hint
`Impact[name.upper()]` looks up by name but raises `KeyError`; catch it and `raise ValueError(...) from None`. For the priority use `match impact + urgency:` with `case 6:`, `case 5:`, `case 4:` and `case _:`.
@@solution
from enum import IntEnum, StrEnum


class Impact(IntEnum):
    LOW = 1
    MEDIUM = 2
    HIGH = 3


class Urgency(IntEnum):
    LOW = 1
    MEDIUM = 2
    HIGH = 3


class Priority(StrEnum):
    P1 = "P1"
    P2 = "P2"
    P3 = "P3"
    P4 = "P4"


def priority(impact, urgency):
    match impact + urgency:
        case 6:
            return Priority.P1
        case 5:
            return Priority.P2
        case 4:
            return Priority.P3
        case _:
            return Priority.P4


def parse(text):
    left, _, right = text.partition("/")
    try:
        return Impact[left.strip().upper()], Urgency[right.strip().upper()]
    except KeyError:
        raise ValueError(f"cannot parse {text!r}") from None


i, u = parse("High/medium")
print(priority(i, u), priority(Impact.LOW, Urgency.LOW))

@@exercise A chat-ops command router
Write `handle(event)` using `match`. `event` is a dict from a chat bot:

- `{"cmd": "status", "args": []}` → `"all systems normal"`
- `{"cmd": "status", "args": [host]}` → `"status of <host>"`
- `{"cmd": "restart", "args": [service], "user": user}` where user is in `ADMINS` → `"restarting <service>"`; for other users → `"denied: <user> can't restart"`
- `{"cmd": "ack", "args": [ticket]}` where ticket is a string of digits → `"acknowledged #<ticket>"`
- anything else → `"unknown command"`
@@starter
ADMINS = {"maya", "li"}


def handle(event):
    pass


print(handle({"cmd": "restart", "args": ["nginx"], "user": "maya"}))
@@tests
check(handle({"cmd": "status", "args": []}), "all systems normal", "status")
check(handle({"cmd": "status", "args": ["db01"]}), "status of db01", "status host")
check(handle({"cmd": "restart", "args": ["nginx"], "user": "maya"}), "restarting nginx", "admin restart")
check(handle({"cmd": "restart", "args": ["nginx"], "user": "bob"}), "denied: bob can't restart", "denied")
check(handle({"cmd": "ack", "args": ["1042"]}), "acknowledged #1042", "ack")
check(handle({"cmd": "ack", "args": ["soon"]}), "unknown command", "bad ticket")
check(handle({"cmd": "status", "args": ["a", "b"]}), "unknown command", "too many args")
check(handle({"text": "hi"}), "unknown command", "no cmd")
@@requires
match event:
@@hint
`case {"cmd": "status", "args": []}:` matches an empty list. Guards do the rest: `case {"cmd": "restart", "args": [service], "user": user} if user in ADMINS:` before the denied case, and `case {"cmd": "ack", "args": [str(ticket)]} if ticket.isdigit():`.
@@solution
ADMINS = {"maya", "li"}


def handle(event):
    match event:
        case {"cmd": "status", "args": []}:
            return "all systems normal"
        case {"cmd": "status", "args": [host]}:
            return f"status of {host}"
        case {"cmd": "restart", "args": [service], "user": user} if user in ADMINS:
            return f"restarting {service}"
        case {"cmd": "restart", "args": [_], "user": user}:
            return f"denied: {user} can't restart"
        case {"cmd": "ack", "args": [str(ticket)]} if ticket.isdigit():
            return f"acknowledged #{ticket}"
        case _:
            return "unknown command"


print(handle({"cmd": "restart", "args": ["nginx"], "user": "maya"}))
