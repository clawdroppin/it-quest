@@lesson n1-scope-arguments Scope, Arguments and Return Values in Depth
@@concepts functions, variables
@@text
You've written functions. This lesson covers the details that decide whether they behave the way you expect: where variables live (**scope**), the many ways to pass arguments, and returning several values.

## Scope: where a name is visible

Python looks up a name in four places, in order (the **LEGB** rule):

1. **L**ocal: inside the current function
2. **E**nclosing: inside any function that contains this one
3. **G**lobal: at the top level of the file (module)
4. **B**uilt-in: `print`, `len`, `range`...

```python
limit = 90                      # global

def check(load):
    status = "high" if load > limit else "ok"   # reads the global
    return status

print(check(95))
# print(status)  -> NameError: status only exists inside check
```

Assigning to a name inside a function makes it **local** for the whole function. That's why this fails:

```python
count = 0

def bump():
    count = count + 1   # UnboundLocalError: count is local here, and not yet set
```

You *can* write `global count` at the top of the function, but don't make it a habit: functions that change globals are hard to test and reason about. **Pass values in and return results out** instead.

Also avoid naming variables after built-ins: `list = [1, 2]` hides the real `list` for the rest of the file.

## Positional and keyword arguments

```python
def connect(host, port=22, timeout=5.0):
    return f"{host}:{port} (timeout {timeout}s)"

print(connect("web01"))                       # defaults
print(connect("web01", 2222))                 # positional
print(connect("web01", timeout=1.5))          # keyword: skip port, clear meaning
```

Keyword arguments make calls readable: `copy(src, dst, overwrite=True)` beats `copy(src, dst, True)`.

## Keyword-only and positional-only parameters

```python
def delete_user(name, *, force=False):     # after *, arguments MUST be named
    return f"delete {name} force={force}"

print(delete_user("old", force=True))
# delete_user("old", True)  -> TypeError: the dangerous flag must be spelled out

def clamp(value, low, high, /):              # before /, arguments can't be named
    return max(low, min(value, high))
```

## *args and **kwargs

```python
def log(level, *messages, **fields):
    text = " ".join(messages)
    extra = " ".join(f"{k}={v}" for k, v in fields.items())
    return f"[{level}] {text} {extra}".strip()

print(log("INFO", "backup", "done", files=120, secs=3.2))
```

- `*messages` collects extra positional arguments into a **tuple**.
- `**fields` collects extra keyword arguments into a **dict**.

The same stars **unpack** when calling: `connect(*("web01", 2222))`, `connect(**{"host": "db", "port": 5432})`.

## The mutable default trap

Default values are created **once**, when the function is defined, not on every call:

```python
def add_alert(msg, alerts=[]):      # BUG: the same list is reused across calls
    alerts.append(msg)
    return alerts

print(add_alert("a"))
print(add_alert("b"))               # ['a', 'b'] !
```

The fix is the `None` default:

```python
def add_alert(msg, alerts=None):
    if alerts is None:
        alerts = []
    alerts.append(msg)
    return alerts
```

## Returning several values

`return a, b` returns a **tuple**, which the caller can unpack:

```python
def min_max(values):
    return min(values), max(values)

low, high = min_max([31, 87, 54])
print(low, high)
```

## Docstrings and type hints

```python
def uptime_percent(up_minutes: int, total_minutes: int) -> float:
    """Return availability as a percentage rounded to 2 decimals."""
    return round(100 * up_minutes / total_minutes, 2)

help(uptime_percent)
```

The docstring shows in `help()` and editors; type hints (covered fully later) document what goes in and out.

@@quiz
In what order does Python look up a name?
- Global, Local, Built-in, Enclosing
* Local, Enclosing, Global, Built-in
- Built-in first
- Only Local and Global
? The LEGB rule.

@@quiz
What's wrong with `def f(items=[]):`?
- Lists can't be defaults
* The same list is shared by every call that uses the default
- It makes items a tuple
- Nothing
? Use `items=None` and create the list inside.

@@quiz
In `def delete(name, *, force=False)`, how must `force` be passed?
* As a keyword: `force=True`
- Positionally only
- It can't be passed
- As `*force`
? Parameters after `*` are keyword-only.

@@quiz
What type does `*args` collect extra positional arguments into?
- list
* tuple
- dict
- set
? `**kwargs` collects keyword arguments into a dict.

@@flashcards
LEGB :: Local, Enclosing, Global, Built-in: the name lookup order
UnboundLocalError :: reading a local variable before it's assigned in that function
Keyword argument :: an argument passed by name: `port=22`
Keyword-only parameter :: a parameter after `*` that must be passed by name
`*args` :: collects extra positional arguments into a tuple
`**kwargs` :: collects extra keyword arguments into a dict
Mutable default trap :: default values are created once and shared between calls
Docstring :: the string right after `def`, shown by help()

@@exercise A flexible command builder
Write `build_command(program, *args, sudo=False, **options)` that builds a command line string:

- starts with `sudo ` if `sudo` is True
- then the program and every positional arg, separated by spaces
- then every option as `--name=value`, in the order given; underscores in names become dashes; an option whose value is `True` becomes just `--name`; options whose value is `False` or `None` are left out

`sudo` must be keyword-only.
@@starter
def build_command(program, *args, sudo=False, **options):
    pass


print(build_command("rsync", "src/", "dst/", sudo=True, dry_run=True, exclude="*.tmp", delete=False))
@@tests
check(build_command("ls"), "ls", "program only")
check(build_command("rsync", "src/", "dst/", sudo=True, dry_run=True, exclude="*.tmp", delete=False), "sudo rsync src/ dst/ --dry-run --exclude=*.tmp", "full")
check(build_command("ping", "web01", count=3, verbose=None), "ping web01 --count=3", "None skipped")
@@hint
Start with `parts = ["sudo"] if sudo else []`, then `parts += [program, *args]`. Loop over `options.items()`: `name = key.replace("_", "-")`; `if value is True: parts.append(f"--{name}")`; `elif value is not False and value is not None: parts.append(f"--{name}={value}")`. Finish with `" ".join(parts)`.
@@solution
def build_command(program, *args, sudo=False, **options):
    parts = ["sudo"] if sudo else []
    parts += [program, *args]
    for key, value in options.items():
        name = key.replace("_", "-")
        if value is True:
            parts.append(f"--{name}")
        elif value is not False and value is not None:
            parts.append(f"--{name}={value}")
    return " ".join(parts)


print(build_command("rsync", "src/", "dst/", sudo=True, dry_run=True, exclude="*.tmp", delete=False))

@@exercise Fix the shared-state bugs
These functions have scope and default-argument bugs. Rewrite them so that:

- `record(event, log=None)` returns a list containing all events recorded **into that log**; calling it without a log starts a fresh list each time.
- `make_stats(values)` returns a tuple `(count, total, average)` with average rounded to 1 decimal (0.0 for an empty list) and doesn't use any global variables.
@@starter
total = 0


def record(event, log=[]):
    log.append(event)
    return log


def make_stats(values):
    global total
    for v in values:
        total += v
    return len(values), total, round(total / len(values), 1)


print(record("boot"), record("login"))
@@tests
check(record("a"), ["a"], "fresh list 1")
check(record("b"), ["b"], "fresh list 2")
mine = []
record("x", mine)
check(record("y", mine), ["x", "y"], "shared when passed")
check(make_stats([1, 2, 4]), (3, 7, 2.3), "stats")
check(make_stats([1, 2, 4]), (3, 7, 2.3), "same result twice")
check(make_stats([]), (0, 0, 0.0), "empty")
@@requires
log=None
@@hint
Use `log=None` and `if log is None: log = []`. In `make_stats`, use a local `total = sum(values)`; guard the division with `if values else 0.0`.
@@solution
def record(event, log=None):
    if log is None:
        log = []
    log.append(event)
    return log


def make_stats(values):
    total = sum(values)
    average = round(total / len(values), 1) if values else 0.0
    return len(values), total, average


print(record("boot"), record("login"))
