@@lesson p9-references Names, Objects and Mutability: Python's Memory Model
@@concepts variables, lists, objects
@@text
Rust has ownership; Python has **references**. Understanding how Python's variables really work explains a whole family of confusing bugs: lists that change "by themselves", copies that aren't copies, and functions that modify their arguments.

## Variables are names, not boxes

In Python, every value is an **object** living somewhere in memory. A variable is a **name tag** attached to an object. Assignment never copies; it attaches another tag:

```python
servers = ["web01", "db01"]
backup = servers            # a second name for the SAME list
backup.append("mail01")
print(servers)              # ['web01', 'db01', 'mail01']
print(servers is backup, id(servers) == id(backup))
```

- `id(x)` is the object's identity (its address in CPython).
- `a is b` asks "same object?"; `a == b` asks "equal values?".

## Mutable and immutable types

| Immutable (can't change after creation) | Mutable (can change in place) |
|---|---|
| `int`, `float`, `bool`, `str`, `tuple`, `frozenset`, `bytes`, `None` | `list`, `dict`, `set`, `bytearray`, most class instances |

"Changing" an immutable value really creates a **new object** and moves the name:

```python
name = "web"
old_id = id(name)
name += "01"                 # a new string object
print(name, id(name) == old_id)

count = [0]
old = id(count)
count += [1]                 # list += extends IN PLACE
print(count, id(count) == old)
```

## Functions receive references

Python passes **the object reference** to a function (sometimes called "call by sharing"). The function can mutate a mutable argument, and the caller sees it; but **reassigning** the parameter only moves the local name:

```python
def add_tag(tags):
    tags.append("prod")       # mutates the caller's list

def replace(tags):
    tags = ["new"]            # only rebinds the local name

t = ["web"]
add_tag(t)
replace(t)
print(t)
```

Good style: either **mutate and return None** (like `list.sort()`) or **return a new object and leave the input alone** (like `sorted()`), and say which in the docstring.

## Copies: shallow and deep

```python
import copy

config = {"name": "web", "ports": [80, 443]}
shallow = config.copy()           # or dict(config), copy.copy(config)
deep = copy.deepcopy(config)

config["ports"].append(8080)
print(shallow["ports"])           # shares the inner list!
print(deep["ports"])              # fully independent
```

- **Shallow copy**: a new outer container whose items are the **same** objects. `list(x)`, `x[:]`, `x.copy()`, `dict(x)`.
- **Deep copy**: copies everything recursively. Slower; use when nested data must be independent.

## The `[[]] * n` trap

```python
grid = [[0] * 3] * 2              # two references to ONE row
grid[0][0] = 9
print(grid)                       # [[9, 0, 0], [9, 0, 0]]

grid = [[0] * 3 for _ in range(2)]  # a new row each time
grid[0][0] = 9
print(grid)
```

## Hashable means usable as a key

Dict keys and set items must be **hashable**: their hash can't change. Immutable built-ins are hashable; lists, dicts and sets are not. Use a `tuple` (of immutables) or `frozenset` as a compound key:

```python
seen = {("10.0.0.5", 22), ("10.0.0.5", 443)}
print(("10.0.0.5", 22) in seen)
```

## Garbage collection

Python frees objects automatically:

1. **Reference counting**: every object counts how many references point to it; at zero it's freed immediately (`sys.getrefcount` shows the count).
2. A **cycle collector** (`gc` module) periodically finds groups of objects that only reference each other.

So files and connections are usually closed when the last reference goes away, but don't rely on that: use `with`. `weakref` lets you refer to an object without keeping it alive (caches, observers).

## Small object caching

CPython reuses some immutable objects (small ints from -5 to 256, some strings). So `a is b` may be True for equal small numbers and False for big ones. That's an implementation detail: **use `==` for values and `is` only for `None` (and other singletons)**.

@@quiz
After `b = a` where `a` is a list, then `b.append(1)`, what happens to `a`?
* It also contains 1: both names refer to the same list
- Nothing, b is a copy
- An error
- a becomes a tuple
? Assignment attaches another name; it never copies.

@@quiz
What does a shallow copy of a dict containing lists share with the original?
- Nothing
* The inner lists
- Only the keys' hashes
- Everything including the outer dict
? Use `copy.deepcopy` for full independence.

@@quiz
Which of these can be a dict key?
- `["a", 1]`
- `{"a": 1}`
* `("a", 1)`
- `{"a"}`
? Keys must be hashable; tuples of immutables are.

@@quiz
When should you use `is` instead of `==`?
- To compare numbers
- To compare strings
* To check for singletons like `None`
- Always, it's faster
? `is` compares identity, not value.

@@drill
Which `copy` function copies nested lists all the way down?
= deepcopy
= copy.deepcopy
? copy.copy only copies the outer list.

@@flashcards
Reference :: a name pointing at an object; assignment copies the reference, not the object
`is` :: same object (identity)
`==` :: equal values
Mutable :: can be changed in place (list, dict, set)
Immutable :: can't change; "changes" create new objects (int, str, tuple)
Shallow copy :: new container, same inner objects
Deep copy :: recursive copy of everything (`copy.deepcopy`)
Hashable :: has a fixed hash, so it can be a dict key or set item
Reference counting :: CPython frees an object when no references remain

@@exercise Don't change my input
`normalize(servers)` receives a list of dicts like `{"name": "WEB01", "tags": ["Prod"]}`. Return a **new** list where every name and tag is lower-case, and **don't modify the input at all** (not the list, not the dicts, not the tag lists).

Also write `same_object(a, b)` and `same_value(a, b)`.
@@starter
def normalize(servers):
    pass


def same_object(a, b):
    pass


def same_value(a, b):
    pass


data = [{"name": "WEB01", "tags": ["Prod", "DMZ"]}]
print(normalize(data), data)
@@tests
data = [{"name": "WEB01", "tags": ["Prod", "DMZ"]}, {"name": "Db01", "tags": []}]
result = normalize(data)
check(result, [{"name": "web01", "tags": ["prod", "dmz"]}, {"name": "db01", "tags": []}], "normalized")
check(data, [{"name": "WEB01", "tags": ["Prod", "DMZ"]}, {"name": "Db01", "tags": []}], "input untouched")
check(result[1]["tags"] is data[1]["tags"], False, "tag lists are new objects")
x = [1]
check(same_object(x, x), True, "same object")
check(same_object(x, [1]), False, "equal but different")
check(same_value(x, [1]), True, "equal values")
@@hint
Build new objects: `[{"name": s["name"].lower(), "tags": [t.lower() for t in s["tags"]]} for s in servers]`. `same_object` uses `is`, `same_value` uses `==`.
@@solution
def normalize(servers):
    return [{"name": s["name"].lower(), "tags": [t.lower() for t in s["tags"]]} for s in servers]


def same_object(a, b):
    return a is b


def same_value(a, b):
    return a == b


data = [{"name": "WEB01", "tags": ["Prod", "DMZ"]}]
print(normalize(data), data)

@@exercise Snapshot and diff
A monitoring tool keeps a snapshot of the config so it can report changes later.

- `snapshot(config)`: return an independent copy (later changes to `config`, including nested lists and dicts, must not affect it).
- `changed_keys(old, new)`: a sorted list of top-level keys that were added, removed or whose values differ.
@@starter
import copy


def snapshot(config):
    pass


def changed_keys(old, new):
    pass


cfg = {"port": 22, "allow": ["10.0.0.0/8"], "limits": {"conn": 10}}
snap = snapshot(cfg)
cfg["allow"].append("0.0.0.0/0")
cfg["limits"]["conn"] = 99
cfg["banner"] = "hi"
print(changed_keys(snap, cfg))
@@tests
cfg = {"port": 22, "allow": ["10.0.0.0/8"], "limits": {"conn": 10}}
snap = snapshot(cfg)
cfg["allow"].append("0.0.0.0/0")
cfg["limits"]["conn"] = 99
cfg["banner"] = "hi"
check(snap, {"port": 22, "allow": ["10.0.0.0/8"], "limits": {"conn": 10}}, "snapshot is independent")
check(changed_keys(snap, cfg), ["allow", "banner", "limits"], "changes")
check(changed_keys({"a": 1, "b": 2}, {"a": 1}), ["b"], "removed key")
check(changed_keys({}, {}), [], "no changes")
@@requires
deepcopy
@@hint
`copy.deepcopy(config)`. For the diff: `keys = old.keys() | new.keys()` then keep keys where `old.get(k, missing) != new.get(k, missing)` with `missing = object()` (a unique sentinel, so a value of None still counts).
@@solution
import copy


def snapshot(config):
    return copy.deepcopy(config)


def changed_keys(old, new):
    missing = object()
    keys = old.keys() | new.keys()
    return sorted(k for k in keys if old.get(k, missing) != new.get(k, missing))


cfg = {"port": 22, "allow": ["10.0.0.0/8"], "limits": {"conn": 10}}
snap = snapshot(cfg)
cfg["allow"].append("0.0.0.0/0")
cfg["limits"]["conn"] = 99
cfg["banner"] = "hi"
print(changed_keys(snap, cfg))
