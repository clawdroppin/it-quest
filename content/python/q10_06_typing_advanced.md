@@lesson q10-typing Advanced Type Hints: Generics, TypedDict, Literal and Overloads
@@concepts functions, objects
@@text
Type hints don't change how Python runs, but with a checker (**mypy**, **Pyright**/Pylance, or the new Rust-based **ty** and **Pyrefly**) they find bugs before you run anything, power editor autocompletion, and document APIs. The basics (`int`, `list[str]`, `X | None`) were covered earlier. This lesson covers the tools for real codebases, roughly what Rust's generics and enums give you.

## Generics

A generic function or class works for any type while keeping the types connected. Python 3.12+ has compact syntax:

```python
def first[T](items: list[T]) -> T | None:
    return items[0] if items else None

class Stack[T]:
    def __init__(self) -> None:
        self._items: list[T] = []
    def push(self, item: T) -> None:
        self._items.append(item)
    def pop(self) -> T:
        return self._items.pop()

s = Stack[int]()
s.push(3)
print(first(["web", "db"]), s.pop())
# s.push("x")  -> the type checker reports: str is not int
```

Older code writes `T = TypeVar("T")` and `class Stack(Generic[T])`; it means the same. **Bounds** restrict what T may be: `def biggest[T: (int, float)](...)` or `[T: Comparable]`.

## Literal and Final

```python
from typing import Literal, Final

LogLevel = Literal["debug", "info", "warning", "error"]   # a type alias

def set_level(level: LogLevel) -> None: ...

set_level("info")
# set_level("verbose")  -> checker error
MAX_RETRIES: Final = 3      # checker error if reassigned
```

## TypedDict: typing JSON-shaped dicts

```python
from typing import TypedDict, NotRequired

class Alert(TypedDict):
    host: str
    severity: int
    tags: NotRequired[list[str]]

def page(alert: Alert) -> str:
    return f"{alert['host']} sev{alert['severity']}"

print(page({"host": "db01", "severity": 2}))
```

At runtime it's an ordinary dict; the checker verifies keys and value types. Great for API payloads before you parse them into classes.

## Overloads

When the return type depends on the arguments:

```python
from typing import overload

@overload
def parse_port(text: str) -> int: ...
@overload
def parse_port(text: None) -> None: ...
def parse_port(text: str | None) -> int | None:
    return None if text is None else int(text)

print(parse_port("22"), parse_port(None))
```

## Callable, ParamSpec and decorators

```python
from collections.abc import Callable
import functools

def logged[**P, R](func: Callable[P, R]) -> Callable[P, R]:
    @functools.wraps(func)
    def wrapper(*args: P.args, **kwargs: P.kwargs) -> R:
        print("calling", func.__name__)
        return func(*args, **kwargs)
    return wrapper

@logged
def add(a: int, b: int) -> int:
    return a + b

print(add(2, 3))     # the checker still knows add takes two ints
```

## Other useful types

| Type | Meaning |
|---|---|
| `Self` | the current class (fluent methods returning `self`) |
| `Never` / `assert_never(x)` | unreachable code; exhaustive checks over enums/unions |
| `Annotated[int, "metadata"]` | extra info for frameworks (Pydantic, FastAPI, Typer) |
| `NewType("UserId", int)` | a distinct type for the checker, plain int at runtime |
| `Protocol` | structural interfaces (earlier lesson) |
| `type Alias = ...` | 3.12 type alias statement |

## Runtime validation

Hints aren't enforced at runtime. At the boundaries (HTTP requests, config files) use a validator: **Pydantic** models (`class User(BaseModel): name: str; quota: int = 10`) parse and validate data from type hints, and FastAPI builds on it. Inside your program, trust the checker.

## Adopting type checking

1. Start with `mypy --strict` (or Pyright "strict") on **new** modules only.
2. Add hints to public functions first; locals are inferred.
3. Run the checker in CI so hints stay true.

@@quiz
Do type hints make Python check types at runtime?
- Yes, always
* No; checkers like mypy or Pyright check them, and runtime ignores them (unless you use a validator like Pydantic)
- Only for functions
- Only with `python -O`
? Hints are metadata.

@@quiz
What does `def first[T](items: list[T]) -> T | None` express?
* The return type is the same type as the list's items
- T must be an int
- It returns a list
- Nothing useful
? Generics connect input and output types.

@@quiz
What is a TypedDict?
- A dict that checks types at runtime
* A type for dicts with specific keys and value types, checked statically
- A dataclass
- A sorted dict
? At runtime it's a normal dict.

@@quiz
What's `assert_never` for?
- Ending the program
* Making the checker prove every case of a union or enum was handled
- Disabling assertions
- Raising warnings
? If a new case is added and unhandled, the checker reports it.

@@drill
Which `typing` construct restricts a value to a few exact strings?
= Literal
= typing.Literal
? Literal["low", "high"].

@@flashcards
Generic :: code parameterised over types, `def f[T](x: T) -> T`
TypeVar :: older syntax for a type parameter
Literal :: a type allowing only specific values
TypedDict :: typed shape for dict keys and values
`@overload` :: several signatures for one function
ParamSpec :: captures a callable's parameters, for typing decorators
Self :: the type of the current class
NewType :: a distinct named type for the checker
Pydantic :: runtime validation from type hints

@@exercise A generic result type
Write a generic `Result[T]` (Python 3.12 syntax) that holds either a value or an error message:

- `Result.ok(value)` and `Result.err(message)` class methods
- `is_ok()`; `unwrap()` returns the value or raises `ValueError(message)`; `unwrap_or(default)`
- `map(func)` returns a new Result with `func` applied to the value (errors pass through unchanged)
- `__repr__`: `Ok(5)` / `Err('disk full')`

Then write `parse_percent(text) -> Result[int]`: an int 0–100 is Ok; otherwise Err with `"not a number: <text>"` or `"out of range: <n>"`.
@@starter
class Result[T]:
    pass


def parse_percent(text: str) -> "Result[int]":
    pass


print(parse_percent("42").map(lambda p: p / 100), parse_percent("abc"), parse_percent("150"))
@@expected
Ok(0.42) Err('not a number: abc') Err('out of range: 150')
@@tests
check(parse_percent("7").unwrap(), 7, "unwrap")
check(parse_percent("x").unwrap_or(0), 0, "unwrap_or")
check(parse_percent("x").is_ok(), False, "is_ok")
check(repr(Result.ok("a").map(str.upper)), "Ok('A')", "map ok")
check(repr(Result.err("e").map(str.upper)), "Err('e')", "map err")
try:
    Result.err("boom").unwrap()
    check("ok", "ValueError", "unwrap error")
except ValueError as e:
    check(str(e), "boom", "message")
@@hint
Store `self._value` and `self._error` (one of them None) in `__init__(self, value=None, error=None)`. `ok` is `@classmethod def ok(cls, value: T) -> "Result[T]": return cls(value=value)`. `map`: `if self._error is not None: return self` else `Result.ok(func(self._value))`.
@@solution
from collections.abc import Callable


class Result[T]:
    def __init__(self, value: T | None = None, error: str | None = None) -> None:
        self._value = value
        self._error = error

    @classmethod
    def ok(cls, value: T) -> "Result[T]":
        return cls(value=value)

    @classmethod
    def err(cls, message: str) -> "Result[T]":
        return cls(error=message)

    def is_ok(self) -> bool:
        return self._error is None

    def unwrap(self) -> T:
        if self._error is not None:
            raise ValueError(self._error)
        return self._value  # type: ignore[return-value]

    def unwrap_or(self, default: T) -> T:
        return default if self._error is not None else self._value  # type: ignore[return-value]

    def map[U](self, func: Callable[[T], U]) -> "Result[U]":
        if self._error is not None:
            return Result.err(self._error)
        return Result.ok(func(self._value))  # type: ignore[arg-type]

    def __repr__(self) -> str:
        return f"Err({self._error!r})" if self._error is not None else f"Ok({self._value!r})"


def parse_percent(text: str) -> Result[int]:
    try:
        n = int(text)
    except ValueError:
        return Result.err(f"not a number: {text}")
    if not 0 <= n <= 100:
        return Result.err(f"out of range: {n}")
    return Result.ok(n)


print(parse_percent("42").map(lambda p: p / 100), parse_percent("abc"), parse_percent("150"))

@@exercise Validate a TypedDict at runtime
Type hints are metadata you can read. Write `validate(td_class, data)` that checks a dict against a `TypedDict` using `typing.get_type_hints(td_class)` and `td_class.__required_keys__`:

- return a list of error strings (empty if valid), in the order of the hints:
  - `"missing key: <k>"` for each required key that's absent
  - `"<k>: expected <type name>, got <actual type name>"` when a present value isn't an instance of a simple type (`str`, `int`, `bool`, `float`)
  - `"unknown key: <k>"` for extra keys (after the others, in the data's order)
- only simple types are used, but `NotRequired[...]` keys may be missing
@@starter
from typing import TypedDict, NotRequired, get_type_hints


class Alert(TypedDict):
    host: str
    severity: int
    acknowledged: NotRequired[bool]


def validate(td_class, data):
    pass


print(validate(Alert, {"host": "db01", "severity": "high", "color": "red"}))
@@tests
check(validate(Alert, {"host": "db01", "severity": 2}), [], "valid")
check(validate(Alert, {"host": "db01", "severity": 2, "acknowledged": True}), [], "optional present")
check(validate(Alert, {"host": "db01", "severity": "high", "color": "red"}), ["severity: expected int, got str", "unknown key: color"], "errors")
check(validate(Alert, {}), ["missing key: host", "missing key: severity"], "missing")
check(validate(Alert, {"host": 1, "severity": 2, "acknowledged": "yes"}), ["host: expected str, got int", "acknowledged: expected bool, got str"], "types")
@@requires
get_type_hints
get_type_hints(
@@hint
`hints = get_type_hints(td_class)` gives `{"host": str, "severity": int, "acknowledged": bool}` (NotRequired is stripped). Loop over `hints.items()`: if the key is missing and in `td_class.__required_keys__`, add a missing error; if present and `not isinstance(data[k], kind)`, add a type error with `kind.__name__` and `type(data[k]).__name__`. Then add unknown keys.
@@solution
from typing import TypedDict, NotRequired, get_type_hints


class Alert(TypedDict):
    host: str
    severity: int
    acknowledged: NotRequired[bool]


def validate(td_class, data):
    hints = get_type_hints(td_class)
    errors = []
    for key, kind in hints.items():
        if key not in data:
            if key in td_class.__required_keys__:
                errors.append(f"missing key: {key}")
        elif not isinstance(data[key], kind):
            errors.append(f"{key}: expected {kind.__name__}, got {type(data[key]).__name__}")
    for key in data:
        if key not in hints:
            errors.append(f"unknown key: {key}")
    return errors


print(validate(Alert, {"host": "db01", "severity": "high", "color": "red"}))
