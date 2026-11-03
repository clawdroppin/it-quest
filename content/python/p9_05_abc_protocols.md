@@lesson p9-protocols Duck Typing, Abstract Base Classes and Protocols
@@concepts objects
@@text
Rust has traits, Go and C# have interfaces. Python's answer is layered: at runtime it uses **duck typing**, and for documentation and tooling it offers **abstract base classes** and **protocols**.

## Duck typing

*"If it walks like a duck and quacks like a duck, it's a duck."* Python doesn't check an object's type before calling a method; it just calls it. Anything with the right methods works:

```python
class FileSink:
    def write(self, text):
        return f"file <- {text}"

class SlackSink:
    def write(self, text):
        return f"slack <- {text}"

def report(sink, text):
    return sink.write(text)          # any object with write() works

print(report(FileSink(), "backup ok"))
print(report(SlackSink(), "backup ok"))
```

This is why `json.dump` accepts any object with `write`, and `for` accepts anything iterable. It's flexible, but nothing tells you *what methods* are needed until something breaks at runtime.

**EAFP vs LBYL**: Python style prefers *"easier to ask forgiveness than permission"* (try it, catch the exception) over *"look before you leap"* (`if hasattr(...)`), because checking first is often racy or incomplete.

## Abstract base classes (ABCs)

An ABC defines a required interface. Subclasses **must** implement every `@abstractmethod`, or they can't be instantiated:

```python
from abc import ABC, abstractmethod

class Storage(ABC):
    @abstractmethod
    def save(self, key, data): ...

    @abstractmethod
    def load(self, key): ...

    def exists(self, key):                 # concrete helper built on the abstract ones
        try:
            self.load(key)
            return True
        except KeyError:
            return False

class MemoryStorage(Storage):
    def __init__(self):
        self._d = {}
    def save(self, key, data):
        self._d[key] = data
    def load(self, key):
        return self._d[key]

s = MemoryStorage()
s.save("a", b"1")
print(s.exists("a"), s.exists("b"))

class Broken(Storage):
    def save(self, key, data): ...

try:
    Broken()
except TypeError as e:
    print(e)
```

The error comes **when creating the object**, not deep inside some later call. ABCs also let you share helper methods (`exists`) among implementations: a "template method".

The standard library's `collections.abc` has ABCs like `Iterable`, `Mapping`, `Sequence`: inherit from `Mapping` and implement 3 methods, and you get `get`, `keys`, `items`, `==`, `in` for free.

## Protocols: static duck typing

ABCs need explicit inheritance. A `typing.Protocol` describes the shape an object must have, and **any** class with matching methods counts, without inheriting. Type checkers (mypy, Pyright) verify it before you run:

```python
from typing import Protocol, runtime_checkable

@runtime_checkable
class Notifier(Protocol):
    def send(self, to: str, msg: str) -> bool: ...

class Email:                       # doesn't mention Notifier at all
    def send(self, to: str, msg: str) -> bool:
        return "@" in to

def alert(n: Notifier, msg: str) -> bool:
    return n.send("ops@corp.example", msg)

print(alert(Email(), "disk full"))
print(isinstance(Email(), Notifier))    # works thanks to @runtime_checkable (checks method names only)
```

This is the closest thing Python has to Go interfaces or Rust traits.

## Which one to use?

| Need | Use |
|---|---|
| Quick scripts, flexible APIs | plain duck typing |
| A family of classes you control, with shared helper code, and errors at construction time | `ABC` |
| Describe what *callers* need, accept classes you don't control (third-party, tests) | `Protocol` + a type checker |

## Dependency injection

Combining protocols with composition gives **dependency injection**: business code receives its storage, clock or notifier as parameters typed by protocol. Production passes real implementations; tests pass fakes. No mocking library needed:

```python,norun
class FakeNotifier:
    def __init__(self):
        self.sent = []
    def send(self, to: str, msg: str) -> bool:
        self.sent.append((to, msg))
        return True

fake = FakeNotifier()
alert(fake, "test")
print(fake.sent)
```

@@quiz
What is duck typing?
* Using any object that has the needed methods, without checking its declared type
- Typing with type hints
- A Python package
- Inheriting from a Duck class
? Python just calls the method.

@@quiz
When does an ABC report a missing abstract method?
- Never
- When the method is called
* When you try to create an instance of the incomplete subclass
- At import time of the module
? `TypeError: Can't instantiate abstract class ...`.

@@quiz
How does a class satisfy a `typing.Protocol`?
- It must inherit from it
* By having methods with matching names and signatures; no inheritance needed
- By registering with abc.register
- By using a decorator
? Structural typing, checked by tools like mypy.

@@quiz
What does EAFP mean?
- Every attribute final, please
* Easier to ask forgiveness than permission: try it and handle the exception
- Explicit arguments for parameters
- Evaluate all functions positionally
? The opposite is LBYL, look before you leap.

@@flashcards
Duck typing :: any object with the right methods works
EAFP :: try the operation and catch the exception
LBYL :: check conditions before acting
ABC :: abstract base class; subclasses must implement its abstract methods
`@abstractmethod` :: marks a method subclasses must implement
`collections.abc` :: ABCs like Iterable, Mapping, Sequence with free mixin methods
Protocol :: structural interface; matching methods are enough
Dependency injection :: passing collaborators (storage, clock) in instead of creating them inside

@@exercise A storage interface
Define an ABC `KeyValueStore` with abstract methods `get(key)` (raises `KeyError` if missing) and `put(key, value)`, plus **concrete** methods built on them:

- `get_or(key, default)`: the value or the default
- `increment(key)`: treats a missing key as 0, stores and returns the new value

Implement `DictStore` with a dict. Make sure a subclass that forgets `put` can't be instantiated.
@@starter
from abc import ABC, abstractmethod


class KeyValueStore(ABC):
    pass


class DictStore(KeyValueStore):
    pass


s = DictStore()
s.increment("logins")
print(s.increment("logins"), s.get_or("errors", 0))
@@tests
s = DictStore()
check(s.increment("x"), 1, "first increment")
check(s.increment("x"), 2, "second increment")
check(s.get_or("y", "none"), "none", "default")
s.put("y", 5)
check(s.get("y"), 5, "put/get")
try:
    s.get("zzz")
    check("found", "KeyError", "missing key")
except KeyError:
    pass
class Partial(KeyValueStore):
    def get(self, key):
        return 1
try:
    Partial()
    check("created", "TypeError", "abstract put")
except TypeError:
    pass
@@requires
@abstractmethod
@@hint
Abstract methods can just have `...` as their body. `get_or`: `try: return self.get(key) except KeyError: return default`. `increment`: `value = self.get_or(key, 0) + 1; self.put(key, value); return value`.
@@solution
from abc import ABC, abstractmethod


class KeyValueStore(ABC):
    @abstractmethod
    def get(self, key): ...

    @abstractmethod
    def put(self, key, value): ...

    def get_or(self, key, default):
        try:
            return self.get(key)
        except KeyError:
            return default

    def increment(self, key):
        value = self.get_or(key, 0) + 1
        self.put(key, value)
        return value


class DictStore(KeyValueStore):
    def __init__(self):
        self._data = {}

    def get(self, key):
        return self._data[key]

    def put(self, key, value):
        self._data[key] = value


s = DictStore()
s.increment("logins")
print(s.increment("logins"), s.get_or("errors", 0))

@@exercise Inject the clock
`is_maintenance_window(clock)` decides whether changes are allowed now. It depends on a **clock protocol**: any object with a `now()` method returning `(weekday, hour)`, where weekday 0 is Monday.

- Define `class Clock(Protocol)` with `now(self) -> tuple[int, int]`.
- Changes are allowed on Saturday (5) and Sunday (6) all day, and on weekdays from 22:00 to 05:59 (hour >= 22 or hour < 6).
- Write `FixedClock(weekday, hour)` for tests, and `deploy(clock, change)` returning `"deployed: <change>"` or `"blocked: <change> (outside window)"`.
@@starter
from typing import Protocol


class Clock(Protocol):
    pass


class FixedClock:
    pass


def is_maintenance_window(clock):
    pass


def deploy(clock, change):
    pass


print(deploy(FixedClock(2, 23), "patch web01"))
print(deploy(FixedClock(2, 14), "patch web01"))
@@expected
deployed: patch web01
blocked: patch web01 (outside window)
@@tests
check(is_maintenance_window(FixedClock(5, 12)), True, "saturday noon")
check(is_maintenance_window(FixedClock(6, 0)), True, "sunday")
check(is_maintenance_window(FixedClock(0, 5)), True, "monday 5am")
check(is_maintenance_window(FixedClock(0, 6)), False, "monday 6am")
check(is_maintenance_window(FixedClock(4, 22)), True, "friday 22h")
check(is_maintenance_window(FixedClock(4, 21)), False, "friday 21h")
@@requires
Protocol
@@hint
The protocol body is just `def now(self) -> tuple[int, int]: ...`. `FixedClock.now` returns the stored tuple. Then `day, hour = clock.now(); return day >= 5 or hour >= 22 or hour < 6`.
@@solution
from typing import Protocol


class Clock(Protocol):
    def now(self) -> tuple[int, int]: ...


class FixedClock:
    def __init__(self, weekday, hour):
        self.weekday = weekday
        self.hour = hour

    def now(self) -> tuple[int, int]:
        return self.weekday, self.hour


def is_maintenance_window(clock: Clock) -> bool:
    day, hour = clock.now()
    return day >= 5 or hour >= 22 or hour < 6


def deploy(clock: Clock, change: str) -> str:
    if is_maintenance_window(clock):
        return f"deployed: {change}"
    return f"blocked: {change} (outside window)"


print(deploy(FixedClock(2, 23), "patch web01"))
print(deploy(FixedClock(2, 14), "patch web01"))
