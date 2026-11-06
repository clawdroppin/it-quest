@@lesson p9-inheritance Inheritance, super() and Composition
@@concepts objects
@@text
Classes can build on other classes. **Inheritance** lets a subclass reuse and specialise a parent's behaviour; **composition** builds objects out of other objects. Knowing when to use which is one of the most important design skills in object-oriented code.

## Inheritance basics

```python
class Device:
    def __init__(self, name, ip):
        self.name = name
        self.ip = ip

    def describe(self):
        return f"{self.name} ({self.ip})"

    def check(self):
        return "ping ok"

class Printer(Device):                       # Printer IS A Device
    def __init__(self, name, ip, toner):
        super().__init__(name, ip)           # let the parent set up its part
        self.toner = toner

    def check(self):                         # override
        base = super().check()               # reuse the parent's version
        return f"{base}, toner {self.toner}%"

p = Printer("prn-2f", "10.0.2.40", 12)
print(p.describe())                          # inherited
print(p.check())                             # overridden
print(isinstance(p, Device), issubclass(Printer, Device))
```

- `super()` returns a proxy for the parent class; always call `super().__init__(...)` when you override `__init__`.
- Every class ultimately inherits from `object`, which provides defaults like `__repr__` and `__eq__`.

## Polymorphism

Code that works with a `Device` works with **any** subclass, and each object runs its own version of a method:

```python,norun
class Switch(Device):
    def check(self):
        return "ping ok, 48 ports up"

for d in [Printer("prn", "10.0.2.40", 80), Switch("sw1", "10.0.0.2"), Device("nas", "10.0.0.9")]:
    print(f"{d.name}: {d.check()}")
```

## Method resolution order and multiple inheritance

Python allows a class to have several parents. The **MRO** (method resolution order) decides where attributes are looked up; see it with `Class.__mro__`. `super()` follows the MRO, which makes cooperative **mixins** possible:

```python,norun
class JsonMixin:
    def to_json(self):
        import json
        return json.dumps(vars(self))

class LoggedDevice(JsonMixin, Device):
    pass

d = LoggedDevice("ap-3", "10.0.3.1")
print(d.to_json())
print([c.__name__ for c in LoggedDevice.__mro__])
```

A **mixin** is a small class adding one capability, meant to be combined with others. Keep multiple inheritance to mixins; deep diamond hierarchies get confusing fast.

## Composition: HAS-A instead of IS-A

Inheritance couples the subclass tightly to every detail of the parent. Often it's better to **hold** an object and **delegate** to it:

```python
class Notifier:
    def send(self, msg):
        return f"email: {msg}"

class SmsNotifier:
    def send(self, msg):
        return f"sms: {msg}"

class Monitor:
    def __init__(self, notifier):
        self.notifier = notifier            # HAS A notifier

    def alert(self, host):
        return self.notifier.send(f"{host} is down")

print(Monitor(Notifier()).alert("web01"))
print(Monitor(SmsNotifier()).alert("web01"))
```

Swapping behaviour is now just passing a different object, and tests can pass a fake. The rule of thumb from *Design Patterns*: **favour composition over inheritance**. Use inheritance when there's a genuine "is a kind of" relationship and the subclass honours everything the parent promises (the **Liskov substitution principle**: anywhere a `Device` works, a `Printer` must work too).

## Signs inheritance is the wrong tool

- A subclass overrides a method just to raise "not supported".
- You inherit only to reuse one helper function.
- The hierarchy is more than 2–3 levels deep.
- You need combinations (`EncryptedCompressedRemoteBackup`): compose small objects instead.

## Inheriting from built-ins

You can subclass `dict`, `list` or `Exception`. Custom exceptions are the most common and most useful case:

```python
class BackupError(Exception):
    pass

class DiskFullError(BackupError):
    def __init__(self, path, free_mb):
        super().__init__(f"{path}: only {free_mb} MB free")
        self.free_mb = free_mb

try:
    raise DiskFullError("/backup", 12)
except BackupError as e:               # catches every kind of backup error
    print(type(e).__name__, e, e.free_mb)
```

For dict-like or list-like classes prefer `collections.UserDict` / `UserList`, or composition: built-in methods like `dict.update` don't call your overridden `__setitem__`.

@@quiz
What does `super().__init__(name, ip)` do in a subclass's `__init__`?
- Creates a second object
* Runs the parent class's initialiser on this object
- Deletes the parent
- Calls the subclass again
? Without it, the parent's attributes aren't set.

@@quiz
What does "favour composition over inheritance" mean?
* Prefer objects that hold and delegate to other objects, rather than deep class hierarchies
- Never use classes
- Always use multiple inheritance
- Put everything in one class
? Composition is looser coupling and easier to test.

@@quiz
What is the MRO?
- A memory limit
* The order in which Python searches classes for an attribute
- A type of mixin
- The module search path
? Inspect it with `Class.__mro__`.

@@quiz
Why define `class DiskFullError(BackupError)`?
- Exceptions can't be raised otherwise
* Callers can catch all backup errors with one `except BackupError`, or just this one specifically
- To make it faster
- To avoid a traceback
? Exception hierarchies let callers choose how precise to be.

@@drill
Which built-in calls the parent class's version of a method?
= super
= super()
? super().__init__(name).

@@flashcards
Inheritance :: a subclass reuses and specialises a parent class (IS-A)
Composition :: an object holds other objects and delegates to them (HAS-A)
`super()` :: proxy to the next class in the MRO, used to call parent methods
Override :: a subclass method replacing the parent's version
Polymorphism :: the same call runs each object's own implementation
Mixin :: small class adding one capability, combined via multiple inheritance
Liskov substitution :: a subclass must work anywhere its parent works
MRO :: method resolution order, `Class.__mro__`

@@exercise A device hierarchy
Build:

- `Device(name)` with `status()` returning `"name: ok"` and `kind()` returning `"device"`.
- `Printer(name, toner)` overriding `status()`: use the parent's status, and append `", low toner"` when toner is below 15. `kind()` is `"printer"`.
- `Camera(name, recording)`: status is the parent's if recording, otherwise `"name: NOT RECORDING"`. `kind()` is `"camera"`.
- `summary(devices)`: a dict counting devices by kind.
@@starter
class Device:
    pass


class Printer(Device):
    pass


class Camera(Device):
    pass


def summary(devices):
    pass


fleet = [Printer("prn-1", 10), Camera("cam-1", False), Device("nas"), Printer("prn-2", 80)]
for d in fleet:
    print(d.status())
print(summary(fleet))
@@expected
prn-1: ok, low toner
cam-1: NOT RECORDING
nas: ok
prn-2: ok
{'printer': 2, 'camera': 1, 'device': 1}
@@tests
check(Printer("p", 14).status(), "p: ok, low toner", "low toner")
check(Printer("p", 15).status(), "p: ok", "enough toner")
check(Camera("c", True).status(), "c: ok", "recording")
check(isinstance(Camera("c", True), Device), True, "camera is a device")
check(summary([]), {}, "empty summary")
@@requires
super()
@@hint
`Printer.__init__` calls `super().__init__(name)` then stores toner. `Printer.status`: `text = super().status()` then append. `summary`: `counts[d.kind()] = counts.get(d.kind(), 0) + 1`.
@@solution
class Device:
    def __init__(self, name):
        self.name = name

    def status(self):
        return f"{self.name}: ok"

    def kind(self):
        return "device"


class Printer(Device):
    def __init__(self, name, toner):
        super().__init__(name)
        self.toner = toner

    def status(self):
        text = super().status()
        if self.toner < 15:
            text += ", low toner"
        return text

    def kind(self):
        return "printer"


class Camera(Device):
    def __init__(self, name, recording):
        super().__init__(name)
        self.recording = recording

    def status(self):
        if self.recording:
            return super().status()
        return f"{self.name}: NOT RECORDING"

    def kind(self):
        return "camera"


def summary(devices):
    counts = {}
    for d in devices:
        counts[d.kind()] = counts.get(d.kind(), 0) + 1
    return counts


fleet = [Printer("prn-1", 10), Camera("cam-1", False), Device("nas"), Printer("prn-2", 80)]
for d in fleet:
    print(d.status())
print(summary(fleet))

@@exercise Compose a backup job
Instead of a class per combination (`CompressedEncryptedBackup`...), compose small steps.

- Each step is an object with `apply(data: str) -> str`. Write `Compress` (removes all spaces), `Encrypt(shift)` (shifts each lowercase letter a–z forward by `shift`, wrapping z→a; other characters unchanged) and `Tag(label)` (prepends `"[label]"`).
- `BackupJob(steps)` holds a list of steps; `run(data)` applies them in order; `describe()` returns the class names joined by `" -> "`.
@@starter
class Compress:
    pass


class Encrypt:
    pass


class Tag:
    pass


class BackupJob:
    pass


job = BackupJob([Compress(), Encrypt(1), Tag("daily")])
print(job.describe())
print(job.run("hello zoo"))
@@expected
Compress -> Encrypt -> Tag
[daily]ifmmpapp
@@tests
check(Encrypt(2).apply("xyz a!"), "zab c!", "wraps")
check(Compress().apply(" a b "), "ab", "compress")
check(BackupJob([]).run("same"), "same", "no steps")
check(BackupJob([Tag("x"), Compress()]).run("a b"), "[x]ab", "order matters")
check(BackupJob([Tag("x")]).describe(), "Tag", "describe")
@@hint
Encrypt: `chr((ord(c) - ord("a") + self.shift) % 26 + ord("a"))` for `"a" <= c <= "z"`. `run`: loop `data = step.apply(data)`. `describe`: `" -> ".join(type(s).__name__ for s in self.steps)`.
@@solution
class Compress:
    def apply(self, data):
        return data.replace(" ", "")


class Encrypt:
    def __init__(self, shift):
        self.shift = shift

    def apply(self, data):
        out = []
        for c in data:
            if "a" <= c <= "z":
                c = chr((ord(c) - ord("a") + self.shift) % 26 + ord("a"))
            out.append(c)
        return "".join(out)


class Tag:
    def __init__(self, label):
        self.label = label

    def apply(self, data):
        return f"[{self.label}]{data}"


class BackupJob:
    def __init__(self, steps):
        self.steps = steps

    def run(self, data):
        for step in self.steps:
            data = step.apply(data)
        return data

    def describe(self):
        return " -> ".join(type(s).__name__ for s in self.steps)


job = BackupJob([Compress(), Encrypt(1), Tag("daily")])
print(job.describe())
print(job.run("hello zoo"))
