@@lesson p9-dunder Special Methods: Making Your Classes Feel Built-In
@@concepts objects
@@text
Why does `len(my_list)` work, `a + b` add numbers but join strings, and `for x in d` loop over a dict? Because built-in types implement **special methods** ("dunder" methods, for the **d**ouble **under**scores). Your classes can implement them too, and then they work with Python's syntax and built-in functions. This is Python's version of Rust's operator traits.

## Representation: __repr__ and __str__

```python
class Server:
    def __init__(self, name, load):
        self.name = name
        self.load = load

    def __repr__(self):                  # for developers: unambiguous
        return f"Server({self.name!r}, {self.load})"

    def __str__(self):                   # for users: readable
        return f"{self.name} ({self.load}%)"

s = Server("web01", 35)
print(s)            # uses __str__
print([s])          # containers use __repr__
print(f"{s!r}")
```

Always write a `__repr__`: debugging output and logs become useful instead of `<__main__.Server object at 0x...>`. A good `__repr__` looks like the code that would recreate the object.

## Equality and hashing

By default `==` compares identity. Define `__eq__` for value equality, and if objects should work as dict keys or in sets, a matching `__hash__` (equal objects must have equal hashes, so hash the same fields you compare):

```python
class Endpoint:
    def __init__(self, host, port):
        self.host, self.port = host, port

    def __eq__(self, other):
        if not isinstance(other, Endpoint):
            return NotImplemented          # let Python try the other side / fall back
        return (self.host, self.port) == (other.host, other.port)

    def __hash__(self):
        return hash((self.host, self.port))

print(Endpoint("db", 5432) == Endpoint("db", 5432))
print(len({Endpoint("db", 5432), Endpoint("db", 5432)}))
```

Defining `__eq__` without `__hash__` makes instances unhashable (on purpose: mutable value objects shouldn't be keys). `@dataclass(frozen=True)` (later lesson) writes all of this for you.

## Ordering

Implement `__lt__` (and `__eq__`) and decorate with `functools.total_ordering` to get `<=`, `>`, `>=` for free; then `sorted()`, `min()` and `max()` work:

```python
from functools import total_ordering

@total_ordering
class Version:
    def __init__(self, text):
        self.parts = tuple(int(p) for p in text.split("."))
    def __eq__(self, other):
        return self.parts == other.parts
    def __lt__(self, other):
        return self.parts < other.parts
    def __repr__(self):
        return ".".join(map(str, self.parts))

print(sorted([Version("1.10.0"), Version("1.9.2"), Version("1.2")]))
```

## Containers: __len__, __getitem__, __contains__, __iter__

```python
class Inventory:
    def __init__(self):
        self._items = {}
    def add(self, tag, name):
        self._items[tag] = name
    def __len__(self):
        return len(self._items)
    def __getitem__(self, tag):
        return self._items[tag]
    def __contains__(self, tag):
        return tag in self._items
    def __iter__(self):
        return iter(sorted(self._items))

inv = Inventory()
inv.add("LT-02", "laptop"); inv.add("LT-01", "laptop")
print(len(inv), inv["LT-01"], "LT-09" in inv, list(inv))
```

`__len__` also makes empty instances **falsy**; `__bool__` controls truthiness directly.

## Arithmetic operators

| Operator | Method | Reflected (right side) |
|---|---|---|
| `a + b` | `__add__` | `__radd__` |
| `a - b` | `__sub__` | `__rsub__` |
| `a * b` | `__mul__` | `__rmul__` |
| `a / b` | `__truediv__` | `__rtruediv__` |
| `-a` | `__neg__` | |
| `a += b` | `__iadd__` (falls back to `__add__`) | |

If `a.__add__(b)` returns `NotImplemented`, Python tries `b.__radd__(a)`. Implementing `__radd__` makes `sum()` work, because `sum` starts from `0 + first_item`.

## Other useful ones

- `__call__`: instances can be called like functions.
- `__enter__` / `__exit__`: the object works with `with` (context managers).
- `__format__`: custom format specs in f-strings.
- `__getattr__`: called for missing attributes (proxies, lazy loading).

@@quiz
Which method do containers like lists use to display their items?
- __str__
* __repr__
- __format__
- __print__
? `print([s])` shows each item's repr.

@@quiz
What should `__eq__` return when compared with an unrelated type?
- False
- None
* NotImplemented
- raise TypeError
? Python then tries the other operand or falls back to identity.

@@quiz
Why implement `__radd__` on a Money class?
* So `sum(list_of_money)` works, since sum starts with `0 + item`
- To make `+=` faster
- It's required for `__add__`
- To support subtraction
? `int.__add__(Money)` returns NotImplemented, so Python calls `Money.__radd__(0)`.

@@quiz
What does `functools.total_ordering` need from you?
- All six comparison methods
* `__eq__` and one of `__lt__`, `__le__`, `__gt__`, `__ge__`
- Only `__hash__`
- `__cmp__`
? It fills in the rest.

@@drill
Which special method makes `len(obj)` work?
= __len__
? Return an int.

@@flashcards
Dunder method :: a special method like `__len__` that Python calls for syntax and built-ins
`__repr__` :: developer-facing representation, used in containers and the REPL
`__str__` :: user-facing text, used by print and str()
`__eq__` / `__hash__` :: value equality and hashing; equal objects need equal hashes
`NotImplemented` :: return value meaning "I don't support this operand"
`__lt__` + `total_ordering` :: full ordering from one comparison
`__iter__` :: makes an object usable in for loops
`__radd__` :: reflected addition, used when the left operand doesn't know the type

@@exercise A Money type
Write `Money` storing integer cents and a currency:

- `Money(1250, "EUR")`; `repr` is `Money(12.50 EUR)`, `str` is `12.50 EUR`
- `+` and `-` between Money of the same currency; a different currency raises `ValueError`
- `* int` (and `int *`) multiplies
- equality and hashing by (cents, currency); `<` for sorting (same currency)
- `sum()` of a list of Money works (handle `0 + Money` via `__radd__`)
@@starter
from functools import total_ordering


@total_ordering
class Money:
    def __init__(self, cents, currency):
        self.cents = cents
        self.currency = currency


items = [Money(1250, "EUR"), Money(399, "EUR"), Money(2000, "EUR")]
print(sum(items), sorted(items), 3 * Money(100, "EUR"))
@@tests
a, b = Money(1250, "EUR"), Money(399, "EUR")
check(str(a + b), "16.49 EUR", "add")
check(repr(a - b), "Money(8.51 EUR)", "sub repr")
check(a * 2 == Money(2500, "EUR"), True, "mul")
check(2 * a == Money(2500, "EUR"), True, "rmul")
check(str(sum([a, b, Money(1, "EUR")])), "16.50 EUR", "sum")
check(len({Money(5, "EUR"), Money(5, "EUR"), Money(5, "USD")}), 2, "hash")
check(sorted([a, b])[0] == b, True, "sorting")
check(Money(5, "EUR") == 5, False, "unrelated type")
try:
    a + Money(1, "USD")
    check("no error", "ValueError", "currency mismatch")
except ValueError:
    pass
@@hint
`__add__`: if `other` isn't Money return `NotImplemented`; check currencies; return `Money(self.cents + other.cents, self.currency)`. `__radd__(self, other)`: `if other == 0: return self` else `return self.__add__(other)`. Format cents with `f"{self.cents / 100:.2f} {self.currency}"` (or `divmod` for exactness).
@@solution
from functools import total_ordering


@total_ordering
class Money:
    def __init__(self, cents, currency):
        self.cents = cents
        self.currency = currency

    def _amount(self):
        sign = "-" if self.cents < 0 else ""
        whole, rest = divmod(abs(self.cents), 100)
        return f"{sign}{whole}.{rest:02d} {self.currency}"

    def __repr__(self):
        return f"Money({self._amount()})"

    def __str__(self):
        return self._amount()

    def _check(self, other):
        if other.currency != self.currency:
            raise ValueError(f"currency mismatch: {self.currency} vs {other.currency}")

    def __add__(self, other):
        if not isinstance(other, Money):
            return NotImplemented
        self._check(other)
        return Money(self.cents + other.cents, self.currency)

    def __radd__(self, other):
        if other == 0:
            return self
        return self.__add__(other)

    def __sub__(self, other):
        if not isinstance(other, Money):
            return NotImplemented
        self._check(other)
        return Money(self.cents - other.cents, self.currency)

    def __mul__(self, factor):
        if not isinstance(factor, int):
            return NotImplemented
        return Money(self.cents * factor, self.currency)

    __rmul__ = __mul__

    def __eq__(self, other):
        if not isinstance(other, Money):
            return NotImplemented
        return (self.cents, self.currency) == (other.cents, other.currency)

    def __hash__(self):
        return hash((self.cents, self.currency))

    def __lt__(self, other):
        if not isinstance(other, Money):
            return NotImplemented
        self._check(other)
        return self.cents < other.cents


items = [Money(1250, "EUR"), Money(399, "EUR"), Money(2000, "EUR")]
print(sum(items), sorted(items), 3 * Money(100, "EUR"))

@@exercise A ring of recent alerts
Write `AlertLog(capacity)`, a container of the most recent alerts:

- `add(msg)`: append; when over capacity drop the oldest
- `len(log)`, `msg in log`, `log[0]` (oldest), `log[-1]` (newest), and `for msg in log` (oldest to newest)
- truthiness: an empty log is falsy
- `repr(log)` is `AlertLog(2/3)` (current count / capacity)
@@starter
class AlertLog:
    def __init__(self, capacity):
        pass


log = AlertLog(3)
for m in ["disk 91%", "cpu 99%", "ping lost", "disk 95%"]:
    log.add(m)
print(log, list(log), log[-1])
@@tests
log = AlertLog(3)
check(bool(log), False, "empty is falsy")
for m in ["disk 91%", "cpu 99%", "ping lost", "disk 95%"]:
    log.add(m)
check(len(log), 3, "len")
check(list(log), ["cpu 99%", "ping lost", "disk 95%"], "iteration order")
check(log[0], "cpu 99%", "oldest")
check(log[-1], "disk 95%", "newest")
check("disk 91%" in log, False, "dropped")
check("ping lost" in log, True, "contains")
check(repr(log), "AlertLog(3/3)", "repr")
check(bool(log), True, "non-empty is truthy")
@@hint
Store a `collections.deque(maxlen=capacity)` and delegate: `__len__` returns `len(self._items)`, `__getitem__(self, i)` returns `self._items[i]`, `__iter__` returns `iter(self._items)`, `__contains__` uses `in`. `__len__` alone already makes empty logs falsy.
@@solution
from collections import deque


class AlertLog:
    def __init__(self, capacity):
        self.capacity = capacity
        self._items = deque(maxlen=capacity)

    def add(self, msg):
        self._items.append(msg)

    def __len__(self):
        return len(self._items)

    def __getitem__(self, index):
        return self._items[index]

    def __contains__(self, msg):
        return msg in self._items

    def __iter__(self):
        return iter(self._items)

    def __repr__(self):
        return f"AlertLog({len(self)}/{self.capacity})"


log = AlertLog(3)
for m in ["disk 91%", "cpu 99%", "ping lost", "disk 95%"]:
    log.add(m)
print(log, list(log), log[-1])
