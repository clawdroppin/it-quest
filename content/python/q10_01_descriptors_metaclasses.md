@@lesson q10-metaprogramming Descriptors, Class Hooks and Metaclasses
@@concepts objects
@@text
Frameworks like Django, SQLAlchemy, Pydantic and dataclasses let you write `name: str = Field(max_length=50)` and somehow get validation, database columns and JSON schemas. They use Python's **metaprogramming** hooks: code that customises how attributes and classes themselves behave. This is Python's counterpart to Rust's macros.

## How attribute lookup works

When you write `obj.x`, Python roughly does:

1. Look for `x` on the **class** (and its parents). If it's a **data descriptor** (defines `__set__` or `__delete__`), call its `__get__`.
2. Otherwise look in the **instance's** `__dict__`.
3. Otherwise, if the class attribute is a non-data descriptor (only `__get__`, like a function), call its `__get__`; or return the class attribute itself.
4. Otherwise call `__getattr__` if defined, else raise `AttributeError`.

Methods, `property`, `classmethod` and `staticmethod` are all descriptors.

## Writing a descriptor

A **descriptor** is an object stored as a class attribute that controls access to an attribute on instances:

```python
class Bounded:
    def __init__(self, low, high):
        self.low, self.high = low, high

    def __set_name__(self, owner, name):        # called when the class is created
        self.name = name
        self.storage = "_" + name

    def __get__(self, obj, objtype=None):
        if obj is None:                          # accessed on the class itself
            return self
        return getattr(obj, self.storage)

    def __set__(self, obj, value):
        if not self.low <= value <= self.high:
            raise ValueError(f"{self.name} must be {self.low}..{self.high}, got {value}")
        setattr(obj, self.storage, value)

class Server:
    cpu_percent = Bounded(0, 100)
    port = Bounded(1, 65535)

    def __init__(self, port):
        self.port = port
        self.cpu_percent = 0

s = Server(443)
s.cpu_percent = 75
try:
    s.port = 70000
except ValueError as e:
    print(e)
```

One `Bounded` class replaces a pile of near-identical properties. That's exactly how ORM columns and validation fields work.

## `__init_subclass__`: react when someone subclasses you

```python
class Plugin:
    registry = {}

    def __init_subclass__(cls, name=None, **kwargs):
        super().__init_subclass__(**kwargs)
        Plugin.registry[name or cls.__name__.lower()] = cls

class DiskCheck(Plugin, name="disk"):
    def run(self):
        return "disk ok"

class PingCheck(Plugin):
    def run(self):
        return "ping ok"

print(sorted(Plugin.registry))
print(Plugin.registry["disk"]().run())
```

Just defining a subclass registers it: no list to maintain. This hook covers most things people used metaclasses for.

## Class decorators

A function that receives a class and returns it (modified or registered) after the class body runs. `@dataclass` and `@functools.total_ordering` are class decorators:

```python
def with_repr(cls):
    fields = list(cls.__annotations__)
    def __repr__(self):
        inside = ", ".join(f"{f}={getattr(self, f)!r}" for f in fields)
        return f"{cls.__name__}({inside})"
    cls.__repr__ = __repr__
    return cls

@with_repr
class User:
    name: str
    role: str
    def __init__(self, name, role):
        self.name, self.role = name, role

print(User("maya", "admin"))
```

## Metaclasses

Classes are objects too, and their type is a **metaclass** (by default `type`). `type` can even create classes at runtime:

```python
Point = type("Point", (), {"x": 0, "describe": lambda self: f"x={self.x}"})
print(Point().describe(), type(Point))
```

A custom metaclass (`class Meta(type)` with `__new__`) can rewrite the class body as it's created. Today they're rarely needed: try, in order, a function, a decorator, a descriptor, `__init_subclass__`, a class decorator, and only then a metaclass. *"Metaclasses are deeper magic than 99% of users should ever worry about"* (Tim Peters).

## `__getattr__` for proxies

`__getattr__` is called only for **missing** attributes, which makes delegation easy:

```python
class ReadOnly:
    def __init__(self, target):
        self._target = target
    def __getattr__(self, name):
        return getattr(self._target, name)
    def __setattr__(self, name, value):
        if name == "_target":
            super().__setattr__(name, value)
        else:
            raise AttributeError("read-only view")

cfg = ReadOnly(type("Cfg", (), {"port": 22})())
print(cfg.port)
```

@@quiz
What makes an object a descriptor?
* It defines `__get__`, `__set__` or `__delete__` and is stored as a class attribute
- It inherits from Descriptor
- It's decorated with @descriptor
- It has `__slots__`
? Functions, properties and classmethods are all descriptors.

@@quiz
When is `__set_name__` called?
- On every attribute access
* When the owning class is created, telling the descriptor its attribute name
- When the instance is deleted
- Never automatically
? It saves you from repeating the name.

@@quiz
What does `__init_subclass__` let a base class do?
- Prevent subclassing
* Run code every time a subclass is defined, e.g. to register it
- Change instance attributes
- Create metaclasses
? Plugin registries are the classic use.

@@quiz
What's the default metaclass of every class?
- object
* type
- ABCMeta
- class
? `type(SomeClass)` is `type`.

@@drill
Which special method runs when a class is subclassed?
= __init_subclass__
? A simpler alternative to many metaclass uses.

@@flashcards
Descriptor :: class attribute object controlling access via `__get__`/`__set__`
Data descriptor :: defines `__set__` or `__delete__`; takes priority over the instance dict
`__set_name__(owner, name)` :: tells a descriptor which attribute name it was assigned to
`__init_subclass__` :: hook called when a subclass is defined
Class decorator :: function that receives and returns a class
Metaclass :: the class of a class; `type` by default
`type(name, bases, dict)` :: creates a class at runtime
`__getattr__` :: called only for missing attributes

@@exercise A typed field descriptor
Write a descriptor `Field(kind, required=True, default=None)`:

- `__set_name__` remembers the attribute name.
- Reading on an instance returns the stored value; if it was never set, return `default` if not required, else raise `AttributeError("<name> is required")`. Reading on the class returns the descriptor.
- Setting checks `isinstance(value, kind)` and raises `TypeError("<name> must be <kind name>")`.

Use it in the given `Asset` class.
@@starter
class Field:
    pass


class Asset:
    tag = Field(str)
    owner = Field(str, required=False, default="unassigned")
    price = Field(int, required=False, default=0)


a = Asset()
a.tag = "LT-0042"
print(a.tag, a.owner, a.price)
@@tests
a = Asset()
try:
    a.tag
    check("read", "AttributeError", "required field missing")
except AttributeError as e:
    check(str(e), "tag is required", "message")
a.tag = "LT-1"
a.price = 1200
check((a.tag, a.owner, a.price), ("LT-1", "unassigned", 1200), "values")
try:
    a.price = "cheap"
    check("set", "TypeError", "wrong type")
except TypeError as e:
    check(str(e), "price must be int", "type message")
b = Asset()
check(b.price, 0, "instances are independent")
check(isinstance(Asset.tag, Field), True, "class access returns descriptor")
@@hint
Store values in the instance's `__dict__` under the same name: data descriptors win over the instance dict, so there's no clash. `__get__`: `if obj is None: return self`, then `if self.name in obj.__dict__: return obj.__dict__[self.name]`. The kind's name is `self.kind.__name__`.
@@solution
class Field:
    def __init__(self, kind, required=True, default=None):
        self.kind = kind
        self.required = required
        self.default = default

    def __set_name__(self, owner, name):
        self.name = name

    def __get__(self, obj, objtype=None):
        if obj is None:
            return self
        if self.name in obj.__dict__:
            return obj.__dict__[self.name]
        if self.required:
            raise AttributeError(f"{self.name} is required")
        return self.default

    def __set__(self, obj, value):
        if not isinstance(value, self.kind):
            raise TypeError(f"{self.name} must be {self.kind.__name__}")
        obj.__dict__[self.name] = value


class Asset:
    tag = Field(str)
    owner = Field(str, required=False, default="unassigned")
    price = Field(int, required=False, default=0)


a = Asset()
a.tag = "LT-0042"
print(a.tag, a.owner, a.price)

@@exercise A self-registering command system
Write a base class `Command` that registers every subclass in `Command.registry` under a `name` class keyword (`class Restart(Command, name="restart")`). A subclass without a name raises `TypeError` **when it's defined**; a duplicate name also raises `TypeError`.

Then write `dispatch(line)`: split into words, find the command class by the first word, instantiate it and return `instance.run(*rest)`; unknown commands return `"unknown: <word>"`.
@@starter
class Command:
    registry = {}


def dispatch(line):
    pass


class Restart(Command, name="restart"):
    def run(self, service):
        return f"restarting {service}"


class Uptime(Command, name="uptime"):
    def run(self):
        return "up 12 days"


print(dispatch("restart nginx"), "|", dispatch("uptime"), "|", dispatch("reboot"))
@@tests
check(sorted(Command.registry), ["restart", "uptime"], "registered")
check(dispatch("restart nginx"), "restarting nginx", "dispatch with arg")
check(dispatch("fly"), "unknown: fly", "unknown")
try:
    class NoName(Command):
        pass
    check("defined", "TypeError", "missing name")
except TypeError:
    pass
try:
    class Again(Command, name="uptime"):
        pass
    check("defined", "TypeError", "duplicate name")
except TypeError:
    pass
check(Command.registry["uptime"].__name__, "Uptime", "duplicate didn't replace")
@@requires
__init_subclass__
@@hint
`def __init_subclass__(cls, name=None, **kwargs): super().__init_subclass__(**kwargs)`, then check `name is None` and `name in Command.registry` before `Command.registry[name] = cls`. In `dispatch`: `word, *rest = line.split()`.
@@solution
class Command:
    registry = {}

    def __init_subclass__(cls, name=None, **kwargs):
        super().__init_subclass__(**kwargs)
        if name is None:
            raise TypeError(f"{cls.__name__} needs a name")
        if name in Command.registry:
            raise TypeError(f"command {name!r} already exists")
        Command.registry[name] = cls


def dispatch(line):
    word, *rest = line.split()
    cls = Command.registry.get(word)
    if cls is None:
        return f"unknown: {word}"
    return cls().run(*rest)


class Restart(Command, name="restart"):
    def run(self, service):
        return f"restarting {service}"


class Uptime(Command, name="uptime"):
    def run(self):
        return "up 12 days"


print(dispatch("restart nginx"), "|", dispatch("uptime"), "|", dispatch("reboot"))
