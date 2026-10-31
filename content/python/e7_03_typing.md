@@lesson e7-typing Type Hints and Dataclasses
@@concepts objects
@@text
As scripts grow into tools that teams share, two features keep them readable and catch bugs before they run.

## Type hints

You can annotate what types a function takes and returns:

```python
def to_gb(megabytes: float, decimals: int = 1) -> float:
    return round(megabytes / 1024, decimals)

def owners(assets: list[dict[str, str]]) -> set[str]:
    return {a["owner"] for a in assets}

print(to_gb(5000))
print(owners([{"owner": "ana"}, {"owner": "ben"}, {"owner": "ana"}]))
```

Python itself **ignores** hints when running. Their value is that editors (VS Code with Pylance) and checkers like **mypy** read them and warn you, as you type, when you pass a `str` where an `int` belongs. They're also documentation that can't go out of date unnoticed.

Common forms: `int`, `str`, `list[str]`, `dict[str, int]`, `tuple[str, int]`, `str | None` (a string or None), and `Callable`, `Iterator` from `collections.abc`.

## Dataclasses

Classes that mostly hold data need a lot of boilerplate: `__init__`, `__repr__`, `__eq__`. `@dataclass` writes it for you from type-hinted fields:

```python
from dataclasses import dataclass, field

@dataclass
class Host:
    name: str
    ip: str
    port: int = 22
    tags: list[str] = field(default_factory=list)

    def address(self) -> str:
        return f"{self.ip}:{self.port}"

h = Host("web01", "10.0.0.5", tags=["prod"])
print(h)
print(h.address())
print(h == Host("web01", "10.0.0.5", 22, ["prod"]))
```

- You get a constructor, a readable `print`, and `==` comparing all fields.
- Mutable defaults (lists, dicts) need `field(default_factory=list)`; otherwise every object would share one list.
- `@dataclass(frozen=True)` makes objects read-only (and usable as dict keys or in sets).
- `@dataclass(order=True)` adds `<`, `>` so you can `sorted()` them by their fields in order.

## Enums

For a fixed set of named values, use an `Enum` instead of loose strings, so typos become errors:

```python
from enum import Enum

class Status(Enum):
    UP = "up"
    DEGRADED = "degraded"
    DOWN = "down"

s = Status("down")
print(s, s.value, s is Status.DOWN)
```

@@quiz
What happens when you call `to_gb("5000")` with the hint `megabytes: float`?
- Python refuses to run the program
* Python runs it anyway (and fails at the division); an editor or mypy would have warned you
- The string is converted to a float automatically
- Nothing at all, it works
? Hints are for tools and people. The interpreter doesn't enforce them.

@@exercise Sort incidents
Write a dataclass `Incident` with fields `id: int`, `severity: int`, `title: str`, and a function `triage(incidents)` that returns them sorted by severity (**1 first**), then by id. Two incidents with the same fields must be equal (`==`).
@@starter
from dataclasses import dataclass


def triage(incidents):
    pass
@@tests
a = Incident(7, 2, "slow VPN")
b = Incident(3, 1, "site down")
c = Incident(5, 2, "printer")
check([i.id for i in triage([a, b, c])], [3, 5, 7], "triage order")
check(Incident(1, 1, "x") == Incident(1, 1, "x"), True, "dataclass equality")
@@hint
`@dataclass class Incident:` with the three annotated fields. Then `sorted(incidents, key=lambda i: (i.severity, i.id))` — a tuple key sorts by the first item, then the second.
@@solution
from dataclasses import dataclass


@dataclass
class Incident:
    id: int
    severity: int
    title: str


def triage(incidents):
    return sorted(incidents, key=lambda i: (i.severity, i.id))
