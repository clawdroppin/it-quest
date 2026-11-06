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
## Checking types with a tool

Type hints don't change how Python runs, so to get value from them, run a **type checker** such as mypy or Pyright (the engine behind VS Code's Pylance) over your code:

```text
$ mypy inventory.py
inventory.py:12: error: Argument 1 to "to_gb" has incompatible type "str"; expected "float"
```

It finds mistakes before the code runs: passing a string where a number is expected, forgetting that a function can return `None`, or misspelling a dataclass field. Editors show the same errors as you type.

Useful hint syntax:

- `list[str]`, `dict[str, int]`, `tuple[str, int]` for collections.
- `str | None` for "a string or None" (older code writes `Optional[str]`).
- `Callable[[int], str]` from `collections.abc` for a function that takes an int and returns a str.
- `Literal["low", "high"]` (from `typing`) for a value that must be one of a few strings.

Start by hinting function signatures. You don't need to annotate every local variable, because the checker works out most of them itself.

@@quiz
What happens when you call `to_gb("5000")` with the hint `megabytes: float`?
- Python refuses to run the program
* Python runs it anyway (and fails at the division); an editor or mypy would have warned you
- The string is converted to a float automatically
- Nothing at all, it works
? Hints are for tools and people. The interpreter doesn't enforce them.

@@quiz
What does `@dataclass` generate for you?
* An `__init__`, a readable `__repr__` and `__eq__` from the class's fields
- A database table
- Automatic type checking at runtime
- A JSON file
? frozen=True also makes instances immutable.

@@quiz
How do you get type errors reported before running the code?
* Run a type checker such as mypy or Pyright
- Python checks hints automatically when it runs
- Add `import types`
- Use print statements
? Hints are for tools and readers; Python itself ignores them at runtime.

@@flashcards
type hint :: def f(x: int) -> str: documents expected types
str | None :: a string or None
list[str] :: a list of strings
@dataclass :: generates __init__, __repr__ and __eq__
frozen=True :: dataclass instances can't be changed
Enum :: a fixed set of named values
mypy / Pyright :: tools that check type hints

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
