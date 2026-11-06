@@lesson i4-classes Classes and Objects
@@concepts objects
@@text
So far, data (dicts) and behavior (functions) lived apart. A **class** bundles them: it describes a kind of thing, with its data (**attributes**) and what it can do (**methods**). Each thing made from a class is an **object** (an *instance*).

```python
class Server:
    def __init__(self, name, cpu_cores):
        self.name = name
        self.cpu_cores = cpu_cores
        self.online = False

    def start(self):
        self.online = True
        print(f"{self.name} is starting")

    def describe(self):
        state = "online" if self.online else "offline"
        return f"{self.name} ({self.cpu_cores} cores, {state})"


web = Server("web01", 8)
db = Server("db01", 32)
web.start()
print(web.describe())
print(db.describe())
```

- `class Server:` defines the class. Class names use **CapitalWords**.
- `__init__` is the **constructor**: it runs when you create an object with `Server("web01", 8)` and sets up its attributes.
- `self` is the object itself. Every method gets it as the first parameter; Python passes it automatically. `web.start()` is really `Server.start(web)`.
- `"online" if self.online else "offline"` is a one-line if (a *conditional expression*).

## Why classes?

Each object keeps its **own** state: `web.online` is `True`, `db.online` is still `False`. The methods that change that state live right next to it, so it's always clear what can change a server and how.

## __str__ and __repr__

Special "dunder" (double underscore) methods let your objects work with Python's built-ins. `__str__` controls what `print` shows:

```python
class Ticket:
    def __init__(self, number, title, priority=3):
        self.number = number
        self.title = title
        self.priority = priority

    def __str__(self):
        return f"#{self.number} [P{self.priority}] {self.title}"

    def escalate(self):
        if self.priority > 1:
            self.priority -= 1


t = Ticket(1042, "VPN not connecting")
t.escalate()
print(t)
```

## Inheritance

A class can **inherit** from another, reusing its code and adding or changing parts:

```python
class Device:
    def __init__(self, name):
        self.name = name

    def info(self):
        return f"{self.name}: generic device"


class Router(Device):
    def __init__(self, name, ports):
        super().__init__(name)      # run Device's constructor
        self.ports = ports

    def info(self):
        return f"{self.name}: router with {self.ports} ports"


for d in [Device("printer1"), Router("rt1", 24)]:
    print(d.info())
```

Use inheritance sparingly: when one thing really *is a* kind of another. Most of the time, plain classes and functions are clearer.

@@quiz
What is `self` in a method?
- The class itself
* The particular object the method was called on
- A keyword that must be the last parameter
- A global variable
? `web.start()` calls `start` with `self` = `web`.

@@quiz
What does `class Server(Device):` mean?
* Server inherits from Device: it gets Device's methods and can add or override its own
- Server and Device are the same class
- Device is created inside Server
- Server can only be used with Device objects
? Inheritance expresses an "is a" relationship.

@@quiz
What's the difference between `__str__` and `__repr__`?
* __str__ is the friendly text for users; __repr__ is an unambiguous one for developers
- They are the same thing
- __repr__ is only for numbers
- __str__ is only used in files
? print uses __str__; the interactive prompt and lists use __repr__.

@@drill
What is the name of the method that sets up a new object?
= __init__
? It runs when you call the class, like Server("web01").

@@flashcards
class :: a blueprint for objects
__init__ :: sets up a new object
self :: the object the method was called on
attribute :: a value stored on an object, like self.name
method :: a function defined in a class
__str__ / __repr__ :: friendly text / developer text
inheritance :: class Child(Parent): reuse and extend
super() :: call the parent class's method

@@exercise A disk class
Write a class `Disk` with:

- `__init__(self, name, size_gb)`: stores both, and `used_gb` starting at 0.
- `write(self, gb)`: adds to `used_gb`, but raises `ValueError` if it wouldn't fit.
- `free_gb(self)`: returns the free space.
- `percent_used(self)`: returns the used percentage rounded to 1 decimal.
@@starter
class Disk:
    pass


d = Disk("C:", 500)
d.write(125)
print(d.free_gb(), d.percent_used())
@@tests
d = Disk("C:", 500)
d.write(125)
check(d.free_gb(), 375, "after writing 125 GB to a 500 GB disk, free_gb()")
check(d.percent_used(), 25.0, "percent_used()")
d.write(375)
check(d.free_gb(), 0, "full disk free_gb()")
try:
    d.write(1)
    raised = False
except ValueError:
    raised = True
check(raised, True, "writing to a full disk raises ValueError")
check(Disk("D:", 3).percent_used(), 0.0, "a new disk is 0% used")
@@hint
In `write`: `if self.used_gb + gb > self.size_gb: raise ValueError("disk full")`, otherwise add it.
@@solution
class Disk:
    def __init__(self, name, size_gb):
        self.name = name
        self.size_gb = size_gb
        self.used_gb = 0

    def write(self, gb):
        if self.used_gb + gb > self.size_gb:
            raise ValueError(f"{self.name}: not enough space for {gb} GB")
        self.used_gb += gb

    def free_gb(self):
        return self.size_gb - self.used_gb

    def percent_used(self):
        return round(self.used_gb / self.size_gb * 100, 1)


d = Disk("C:", 500)
d.write(125)
print(d.free_gb(), d.percent_used())

@@exercise Ticket queue
Write a class `TicketQueue` that keeps tickets as `(priority, title)` pairs:

- `add(self, title, priority)`: adds a ticket.
- `next(self)`: removes and returns the **title** with the **lowest priority number** (1 is most urgent). If several share it, the one added first. Returns `None` if empty.
- `__len__(self)`: number of waiting tickets (so `len(queue)` works).
@@starter
class TicketQueue:
    pass


q = TicketQueue()
q.add("printer jam", 3)
q.add("server down", 1)
print(q.next(), len(q))
@@tests
q = TicketQueue()
q.add("printer jam", 3)
q.add("server down", 1)
q.add("email slow", 2)
q.add("site down", 1)
check(len(q), 4, "len after 4 adds")
check(q.next(), "server down", "first next()")
check(q.next(), "site down", "second next()")
check(q.next(), "email slow", "third next()")
check(q.next(), "printer jam", "fourth next()")
check(q.next(), None, "next() on an empty queue")
check(len(q), 0, "len when empty")
@@hint
Store a list. In `next`, find the position of the smallest priority with a loop (keep the first one you find on ties), then `pop` that position.
@@solution
class TicketQueue:
    def __init__(self):
        self.items = []

    def add(self, title, priority):
        self.items.append((priority, title))

    def next(self):
        if not self.items:
            return None
        best = 0
        for i in range(1, len(self.items)):
            if self.items[i][0] < self.items[best][0]:
                best = i
        return self.items.pop(best)[1]

    def __len__(self):
        return len(self.items)


q = TicketQueue()
q.add("printer jam", 3)
q.add("server down", 1)
print(q.next(), len(q))
