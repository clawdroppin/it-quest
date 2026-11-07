@@lesson q10-memory Memory: Garbage Collection, Weak References and Streaming
@@concepts objects, files
@@text
Python manages memory for you, but long-running services, scripts that process huge logs and tools that cache data can still run out of memory or leak it. This lesson explains how Python's memory works and the techniques that keep usage flat.

## How big are things?

Everything in Python is a full object with a header, so even small values cost more than in C or Rust:

```python
import sys

print(sys.getsizeof(0), sys.getsizeof(2**64), sys.getsizeof(""), sys.getsizeof("a" * 100))
print(sys.getsizeof([]), sys.getsizeof([0] * 1000), sys.getsizeof({}))
```

`getsizeof` counts only the container, not the objects inside it. A list of a million small ints costs about 8 MB for the pointers **plus** the int objects themselves. Compact alternatives:

- `array.array("i", ...)` stores raw C ints: 4 bytes each.
- NumPy arrays (same idea, plus vectorised maths).
- `__slots__` classes or tuples instead of dicts for millions of records.
- `bytes` instead of lists of small ints.

## Reference counting and the cycle collector

CPython frees an object the moment its **reference count** drops to zero. Objects that refer to each other in a **cycle** (a parent with a list of children that point back to the parent) never reach zero on their own, so a **cycle collector** (`gc` module) runs periodically to find and free them.

```python
import gc

class Node:
    def __init__(self):
        self.other = None

a, b = Node(), Node()
a.other, b.other = b, a          # a cycle
del a, b                          # refcounts don't reach zero...
print("collected:", gc.collect()) # ...but the cycle collector frees them
```

Real "leaks" in Python are almost always **objects that are still referenced**: a global cache that only grows, a list of every request ever handled, event handlers that are never removed.

## Weak references

A **weak reference** points to an object without keeping it alive. Perfect for caches and observer lists:

```python
import weakref

class Session:
    def __init__(self, user):
        self.user = user

cache = weakref.WeakValueDictionary()
s = Session("maya")
cache["maya"] = s
print("maya" in cache)
del s                            # the only strong reference is gone
print("maya" in cache)           # the cache entry disappeared automatically
```

`weakref.WeakKeyDictionary`, `WeakSet`, `weakref.ref(obj)` and `weakref.finalize(obj, callback)` complete the toolbox. Not every object supports weak references (ints, strings, tuples don't; your own classes do unless they use `__slots__` without `__weakref__`).

## Bounded caches

An unbounded dict cache is a memory leak with good intentions. Bound it:

- `functools.lru_cache(maxsize=1024)` evicts the least recently used entries.
- An `OrderedDict` with `move_to_end` and `popitem(last=False)` builds your own LRU (the exercise).
- Time-based expiry for data that goes stale (see Rust's KV-store project for the same idea).

## Streaming: constant memory for any input size

The biggest win is usually not loading everything at once:

```python
def lines(path_lines):            # imagine: open(path) yields lines lazily
    for line in path_lines:
        yield line.rstrip("\n")

def errors(lines):
    return (l for l in lines if " ERROR " in l)

fake_file = (f"2026-11-03 {'ERROR' if i % 1000 == 0 else 'INFO'} event {i}\n" for i in range(100_000))
print(sum(1 for _ in errors(lines(fake_file))))
```

Each stage handles one line at a time; memory use is the same for 1 KB or 100 GB of logs. Also stream with `csv.reader` over a file, `json` lines (one JSON object per line), database cursors (`fetchmany`), and HTTP `iter_content`.

## Finding memory problems

- `tracemalloc`: snapshot allocations and compare (`snapshot.compare_to(old, "lineno")`) to see which lines grow.
- **memray** (Linux/macOS): flame graphs of allocations, including C extensions.
- `objgraph` / `gc.get_referrers(obj)`: who is keeping this object alive?
- Watch process memory over time in production (RSS in your metrics); a slow steady climb is a leak.

@@quiz
What does `sys.getsizeof(a_list)` measure?
- The list and everything in it
* Only the list object itself (its pointer array), not the items
- The memory of the whole program
- Disk usage
? Nested objects must be measured separately.

@@quiz
What is the cycle collector for?
* Freeing groups of objects that reference each other but are unreachable
- Collecting unused imports
- Compressing memory
- Closing files
? Reference counting alone can't free cycles.

@@quiz
Why use a WeakValueDictionary for a cache of sessions?
- It's faster
* Entries disappear automatically when nothing else uses the session, so the cache can't keep them alive forever
- It encrypts values
- It's thread-safe
? Weak references don't count toward keeping objects alive.

@@quiz
What's the most effective way to process a 50 GB log in Python?
- `f.readlines()`
- `f.read().split("\n")`
* Iterate over the file line by line with generators
- Load it into a dict
? Streaming keeps memory constant.

@@drill
Which module creates references that don't keep an object alive?
= weakref
? Useful for caches and observers.

@@flashcards
Reference count :: number of references to an object; freed at zero
Cycle collector :: `gc` module; frees unreachable reference cycles
Weak reference :: refers to an object without keeping it alive
WeakValueDictionary :: dict whose entries vanish when their values are collected
LRU cache :: evicts the least recently used entries when full
Streaming :: processing data piece by piece in constant memory
tracemalloc :: standard library tool to find which lines allocate memory
`array.array` :: compact typed array of numbers

@@exercise An LRU cache
Write `LRUCache(capacity)` with `get(key)` (returns the value or `None`, and marks the key as recently used) and `put(key, value)` (inserts or updates and marks recent; if over capacity, evicts the **least** recently used key). Also `keys()` returning the keys from least to most recently used, and `evictions` (a count). Use `collections.OrderedDict`.
@@starter
from collections import OrderedDict


class LRUCache:
    pass


c = LRUCache(2)
c.put("a", 1)
c.put("b", 2)
c.get("a")
c.put("c", 3)
print(c.keys(), c.get("b"), c.evictions)
@@tests
c = LRUCache(2)
c.put("a", 1)
c.put("b", 2)
check(c.get("a"), 1, "get")
c.put("c", 3)
check(c.keys(), ["a", "c"], "b evicted, a was recent")
check(c.get("b"), None, "evicted")
c.put("a", 10)
check(c.keys(), ["c", "a"], "update moves to end")
check(c.get("a"), 10, "updated value")
check(c.evictions, 1, "eviction count")
c.put("d", 4)
check((c.keys(), c.evictions), (["a", "d"], 2), "second eviction")
@@requires
OrderedDict
.move_to_end(
@@hint
`get`: `if key not in self._d: return None`; `self._d.move_to_end(key)`; return it. `put`: set the value, `move_to_end(key)`, then `if len(self._d) > self.capacity: self._d.popitem(last=False); self.evictions += 1`.
@@solution
from collections import OrderedDict


class LRUCache:
    def __init__(self, capacity):
        self.capacity = capacity
        self._d = OrderedDict()
        self.evictions = 0

    def get(self, key):
        if key not in self._d:
            return None
        self._d.move_to_end(key)
        return self._d[key]

    def put(self, key, value):
        self._d[key] = value
        self._d.move_to_end(key)
        if len(self._d) > self.capacity:
            self._d.popitem(last=False)
            self.evictions += 1

    def keys(self):
        return list(self._d)


c = LRUCache(2)
c.put("a", 1)
c.put("b", 2)
c.get("a")
c.put("c", 3)
print(c.keys(), c.get("b"), c.evictions)

@@exercise A streaming log pipeline
Build a pipeline of generator functions that never holds the whole log in memory:

- `parse(lines)`: yields `(level, service, ms)` from lines like `"INFO api 120ms"`; skip lines that don't have exactly 3 parts or whose last part isn't digits followed by `ms`
- `slow(records, limit)`: yields records with `ms > limit`
- `summary(records)`: consumes the stream and returns a dict service → `(count, max_ms)`

The tests feed a generator of 300,000 lines and check that `parse` and `slow` are generators.
@@starter
import types


def parse(lines):
    pass


def slow(records, limit):
    pass


def summary(records):
    pass


LOG = ["INFO api 120ms", "WARN db 950ms", "garbage", "INFO api 1300ms", "ERROR db 2000ms", "INFO web xms"]
print(summary(slow(parse(LOG), 500)))
@@tests
LOG = ["INFO api 120ms", "WARN db 950ms", "garbage", "INFO api 1300ms", "ERROR db 2000ms", "INFO web xms"]
check(summary(slow(parse(LOG), 500)), {"db": (2, 2000), "api": (1, 1300)}, "summary")
check(isinstance(parse([]), types.GeneratorType), True, "parse is a generator")
check(isinstance(slow([], 1), types.GeneratorType), True, "slow is a generator")
big = (f"INFO svc{i % 3} {i % 1000}ms" for i in range(300000))
r = summary(slow(parse(big), 990))
check(r["svc0"][1] >= 990, True, "big stream works")
check(sum(c for c, _ in r.values()), 2700, "big count")
@@hint
`parse`: `for line in lines: parts = line.split(); if len(parts) == 3 and parts[2].endswith("ms") and parts[2][:-2].isdigit(): yield parts[0], parts[1], int(parts[2][:-2])`. `summary`: update `count, top = out.get(svc, (0, 0))` per record.
@@solution
import types


def parse(lines):
    for line in lines:
        parts = line.split()
        if len(parts) == 3 and parts[2].endswith("ms") and parts[2][:-2].isdigit():
            yield parts[0], parts[1], int(parts[2][:-2])


def slow(records, limit):
    for record in records:
        if record[2] > limit:
            yield record


def summary(records):
    out = {}
    for _, service, ms in records:
        count, top = out.get(service, (0, 0))
        out[service] = (count + 1, max(top, ms))
    return out


LOG = ["INFO api 120ms", "WARN db 950ms", "garbage", "INFO api 1300ms", "ERROR db 2000ms", "INFO web xms"]
print(summary(slow(parse(LOG), 500)))
