@@lesson p9-properties Properties, Class Methods, Static Methods and __slots__
@@concepts objects
@@text
Attributes in Python are public and simple: `server.port = 22`. This lesson shows how to keep that simple syntax while adding validation and computed values (**properties**), how to write alternative constructors (**class methods**), and how classes can share data.

## Properties: methods that look like attributes

Start with plain attributes. When you later need validation or a computed value, turn the attribute into a **property** without changing any calling code:

```python
class Disk:
    def __init__(self, total_gb, used_gb):
        self.total_gb = total_gb
        self.used_gb = used_gb          # goes through the setter below

    @property
    def used_gb(self):
        return self._used_gb

    @used_gb.setter
    def used_gb(self, value):
        if not 0 <= value <= self.total_gb:
            raise ValueError(f"used must be 0..{self.total_gb}, got {value}")
        self._used_gb = value

    @property
    def percent(self):                  # read-only, computed
        return round(100 * self.used_gb / self.total_gb, 1)

d = Disk(500, 120)
d.used_gb = 450
print(d.percent)
try:
    d.used_gb = 600
except ValueError as e:
    print(e)
```

- The real value lives in `_used_gb` (a leading underscore means "internal, don't touch").
- A property without a setter is **read-only**: `d.percent = 5` raises `AttributeError`.
- This is why Python code rarely has `get_x()`/`set_x()` methods like Java.

## Class attributes vs instance attributes

```python
class Ticket:
    prefix = "INC"                 # class attribute: shared by all instances
    next_number = 1

    def __init__(self, title):
        self.title = title         # instance attribute
        self.number = f"{Ticket.prefix}{Ticket.next_number:04d}"
        Ticket.next_number += 1

print(Ticket("printer").number, Ticket("vpn").number)
```

Reading `self.prefix` finds the class attribute if the instance has none. **Assigning** `self.prefix = ...` creates an instance attribute that hides it. Never use a mutable class attribute (like a list) as per-instance storage: it's shared.

## Class methods: alternative constructors

A `@classmethod` receives the **class** (`cls`) instead of an instance. Its main use is alternative ways to create objects:

```python
class Endpoint:
    def __init__(self, host, port):
        self.host, self.port = host, port

    @classmethod
    def parse(cls, text):                # "db01:5432" -> Endpoint
        host, _, port = text.rpartition(":")
        return cls(host, int(port))      # cls, so subclasses get their own type

    def __repr__(self):
        return f"{type(self).__name__}({self.host!r}, {self.port})"

print(Endpoint.parse("db01:5432"))
```

The standard library is full of these: `dict.fromkeys`, `datetime.fromisoformat`, `int.from_bytes`, `Path.home()`.

## Static methods

A `@staticmethod` gets neither `self` nor `cls`; it's a plain function grouped with the class because it belongs there logically:

```python
class Password:
    @staticmethod
    def is_strong(text):
        return len(text) >= 12 and any(c.isdigit() for c in text)

print(Password.is_strong("correct-horse-42"))
```

Often a module-level function is just as good. Use static methods when the function is clearly part of the class's API.

## __slots__: fixed attributes, less memory

Normally each instance stores attributes in a `__dict__`, so you can add any attribute (even by typo). `__slots__` lists the allowed attributes:

```python
class Point:
    __slots__ = ("x", "y")
    def __init__(self, x, y):
        self.x, self.y = x, y

p = Point(1, 2)
try:
    p.z = 3
except AttributeError as e:
    print("blocked:", e)
```

Slotted instances use noticeably less memory, which matters with millions of objects, and typos in attribute names become errors. `@dataclass(slots=True)` does it for you.

## Private by convention

- `_name`: internal; please don't use from outside (not enforced).
- `__name` (two leading underscores): **name mangling**, becomes `_ClassName__name` to avoid clashes in subclasses. Rarely needed.
- There's no `private` keyword; Python's culture is "we're all consenting adults".

@@quiz
What does a property without a setter give you?
- An error when reading
* A read-only attribute computed by a method
- A class attribute
- A static method
? Assigning to it raises AttributeError.

@@quiz
What does a classmethod receive as its first argument?
- The instance
* The class itself
- Nothing
- The module
? Conventionally named `cls`.

@@quiz
Why use `cls(...)` instead of `Endpoint(...)` inside a classmethod constructor?
* So subclasses calling the method get an instance of the subclass
- It's faster
- Endpoint isn't defined yet
- To avoid recursion
? `SecureEndpoint.parse(...)` should return a SecureEndpoint.

@@quiz
What does `__slots__` do?
- Makes attributes private
* Restricts instances to the listed attributes and saves memory
- Creates properties
- Makes the class immutable
? There's no per-instance `__dict__`.

@@drill
Which decorator makes a method behave like an attribute?
= @property
= property
? Add @x.setter to control assignment.

@@flashcards
`@property` :: a method accessed like an attribute
`@x.setter` :: method run when assigning to property x, used for validation
Class attribute :: defined in the class body, shared by all instances
`@classmethod` :: receives `cls`; used for alternative constructors
`@staticmethod` :: receives neither self nor cls
`__slots__` :: fixed list of allowed attributes, less memory
`_name` :: convention for "internal"
Name mangling :: `__name` becomes `_Class__name`

@@exercise A validated user account
Write `Account(username, quota_gb)`:

- `username` is a property: setting it strips whitespace and lower-cases it; empty or containing a space after stripping raises `ValueError`.
- `quota_gb` is a property that must be an int from 1 to 1000 (else `ValueError`).
- `used_gb` is a plain attribute starting at 0.
- `free_gb` is a read-only property: quota minus used, never below 0.
- `Account.from_line("maya,50")` builds one from a CSV line (class method).
@@starter
class Account:
    pass


a = Account.from_line("  Maya ,50")
a.used_gb = 60
print(a.username, a.quota_gb, a.free_gb)
@@tests
a = Account(" Li ", 10)
check(a.username, "li", "normalized")
a.used_gb = 3
check(a.free_gb, 7, "free")
a.used_gb = 30
check(a.free_gb, 0, "never negative")
for bad in ["", "   ", "two words"]:
    try:
        Account(bad, 10)
        check(bad, "ValueError", "bad username")
    except ValueError:
        pass
for bad in [0, 1001, "5", 2.5]:
    try:
        a.quota_gb = bad
        check(bad, "ValueError", "bad quota")
    except ValueError:
        pass
check(Account.from_line("maya,50").quota_gb, 50, "from_line")
try:
    a.free_gb = 5
    check("assigned", "AttributeError", "free_gb is read-only")
except AttributeError:
    pass
@@requires
@classmethod
@@hint
In the setters use `type(value) is not int` to reject floats, strings and bools. `from_line`: `name, quota = line.split(","); return cls(name, int(quota))`. `free_gb`: `max(0, self.quota_gb - self.used_gb)`.
@@solution
class Account:
    def __init__(self, username, quota_gb):
        self.username = username
        self.quota_gb = quota_gb
        self.used_gb = 0

    @property
    def username(self):
        return self._username

    @username.setter
    def username(self, value):
        value = value.strip().lower()
        if not value or " " in value:
            raise ValueError(f"invalid username: {value!r}")
        self._username = value

    @property
    def quota_gb(self):
        return self._quota_gb

    @quota_gb.setter
    def quota_gb(self, value):
        if type(value) is not int or not 1 <= value <= 1000:
            raise ValueError(f"quota must be an int from 1 to 1000, got {value!r}")
        self._quota_gb = value

    @property
    def free_gb(self):
        return max(0, self.quota_gb - self.used_gb)

    @classmethod
    def from_line(cls, line):
        name, quota = line.split(",")
        return cls(name, int(quota))


a = Account.from_line("  Maya ,50")
a.used_gb = 60
print(a.username, a.quota_gb, a.free_gb)

@@exercise Ticket numbering with class state
Write `Ticket`:

- class attributes `prefix = "INC"` and a counter
- each new ticket gets `number` like `"INC0001"`, increasing per ticket
- `Ticket.reset(start=1)` (class method) resets the counter
- `Ticket.is_valid_number(text)` (static method): True for the prefix followed by exactly 4 digits
- a subclass `Request(Ticket)` with `prefix = "REQ"` uses **its own** prefix but the shared counter; use `type(self).prefix` or `self.prefix`
@@starter
class Ticket:
    pass


class Request(Ticket):
    prefix = "REQ"


Ticket.reset()
print(Ticket("printer").number, Request("new laptop").number, Ticket("vpn").number)
@@tests
Ticket.reset()
check(Ticket("a").number, "INC0001", "first")
check(Request("b").number, "REQ0002", "subclass prefix, shared counter")
Ticket.reset(41)
check(Ticket("c").number, "INC0041", "reset start")
check(Ticket.is_valid_number("INC0042"), True, "valid")
check(Ticket.is_valid_number("INC42"), False, "too short")
check(Ticket.is_valid_number("REQ0001"), False, "wrong prefix")
check(Ticket("d").title, "d", "title stored")
@@requires
@staticmethod
@@hint
Keep the counter on `Ticket` explicitly (`Ticket._next`), so subclasses share it. `reset` is `@classmethod def reset(cls, start=1): Ticket._next = start`. The static check: `text.startswith(Ticket.prefix) and len(text) == len(Ticket.prefix) + 4 and text[len(Ticket.prefix):].isdigit()`.
@@solution
class Ticket:
    prefix = "INC"
    _next = 1

    def __init__(self, title):
        self.title = title
        self.number = f"{self.prefix}{Ticket._next:04d}"
        Ticket._next += 1

    @classmethod
    def reset(cls, start=1):
        Ticket._next = start

    @staticmethod
    def is_valid_number(text):
        p = Ticket.prefix
        return text.startswith(p) and len(text) == len(p) + 4 and text[len(p):].isdigit()


class Request(Ticket):
    prefix = "REQ"


Ticket.reset()
print(Ticket("printer").number, Request("new laptop").number, Ticket("vpn").number)
