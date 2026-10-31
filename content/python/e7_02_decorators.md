@@lesson e7-decorators Decorators and Context Managers
@@concepts functions
@@text
Two features that make professional Python code clean: **decorators** add behavior around functions, and **context managers** (the `with` statement) guarantee setup and cleanup.

## Functions are values

A function can be stored in a variable, passed to another function, and returned from one:

```python
def shout(text):
    return text.upper()

def apply(func, value):
    return func(value)

f = shout
print(apply(f, "disk full"))
print(sorted(["web10", "web9", "db1"], key=len))
```

## Decorators

A **decorator** takes a function and returns a new function that wraps it, adding something before and/or after:

```python
import functools, time

def timed(func):
    @functools.wraps(func)          # keep the original name and docstring
    def wrapper(*args, **kwargs):
        start = time.perf_counter()
        result = func(*args, **kwargs)
        ms = (time.perf_counter() - start) * 1000
        print(f"{func.__name__} took {ms:.0f} ms")
        return result
    return wrapper

@timed
def backup(n):
    time.sleep(0.05 * n)
    return f"{n} files"

print(backup(2))
```

`@timed` above `def backup` means `backup = timed(backup)`. `*args, **kwargs` collect any positional and keyword arguments, so the wrapper works for any function.

Real uses: timing, logging, caching, permission checks, and **retrying**:

```python
import functools

def retry(times):
    def decorate(func):
        @functools.wraps(func)
        def wrapper(*args, **kwargs):
            for attempt in range(1, times + 1):
                try:
                    return func(*args, **kwargs)
                except ConnectionError as e:
                    print(f"attempt {attempt} failed: {e}")
            raise ConnectionError(f"gave up after {times} attempts")
        return wrapper
    return decorate

calls = {"n": 0}

@retry(times=3)
def flaky_api():
    calls["n"] += 1
    if calls["n"] < 3:
        raise ConnectionError("timeout")
    return "data"

print(flaky_api())
```

A decorator **with arguments** (`@retry(times=3)`) is a function that returns a decorator: one more layer.

The standard library has some ready-made ones: `@functools.cache` remembers results, `@property` makes a method look like an attribute, `@staticmethod`, `@dataclass` (next lesson).

## Context managers

You know `with open(...) as f:`. Anything with setup and cleanup can be a context manager. The easiest way to write one is `contextlib.contextmanager` with a generator:

```python
from contextlib import contextmanager

@contextmanager
def maintenance_mode(server):
    print(f"[{server}] maintenance ON")
    try:
        yield server
    finally:
        print(f"[{server}] maintenance OFF")

with maintenance_mode("web01") as s:
    print(f"patching {s}...")
print("back to normal")
```

The code before `yield` is the setup, after it the cleanup. Because of `try/finally`, the cleanup runs **even if the block raises an error**: maintenance mode is never left on by accident. Locks, database transactions, temporary directories (`tempfile.TemporaryDirectory()`) and network connections all work this way.

@@quiz
What does `@timed` written above `def backup(...)` do?
- Runs backup immediately
* Replaces backup with `timed(backup)`, a wrapped version
- Makes backup faster
- Adds a comment
? Decorator syntax is just shorthand for `backup = timed(backup)`.

@@exercise Call counter
Write a decorator `counted` that counts how many times the decorated function is called, storing the number in an attribute `calls` on the wrapper (starting at 0). The wrapped function must still return its normal result.
@@starter
import functools


def counted(func):
    pass


@counted
def ping(host):
    return f"pong from {host}"


print(ping("a"), ping("b"), ping.calls)
@@tests
@counted
def _add(a, b):
    return a + b
check(_add.calls, 0, "calls before any call")
check(_add(2, 3), 5, "the wrapped function still returns its result")
_add(1, 1)
check(_add.calls, 2, "calls after two calls")
check(_add.__name__, "_add", "functools.wraps keeps the name")
@@hint
Inside `counted`, define `wrapper(*args, **kwargs)` that does `wrapper.calls += 1` and returns `func(*args, **kwargs)`. Set `wrapper.calls = 0` before returning `wrapper`, and decorate it with `@functools.wraps(func)`.
@@solution
import functools


def counted(func):
    @functools.wraps(func)
    def wrapper(*args, **kwargs):
        wrapper.calls += 1
        return func(*args, **kwargs)
    wrapper.calls = 0
    return wrapper


@counted
def ping(host):
    return f"pong from {host}"


print(ping("a"), ping("b"), ping.calls)

@@exercise A safe working directory
Write a context manager `in_folder(path)` (with `@contextmanager`) that creates the folder if needed, changes the current directory into it (`os.chdir`), and **always** changes back to the original directory afterwards, even if the block raises.
@@starter
import os
from contextlib import contextmanager


@contextmanager
def in_folder(path):
    yield


start = os.getcwd()
with in_folder("work"):
    print(os.path.basename(os.getcwd()))
print(os.getcwd() == start)
@@tests
import os as _os
_start = _os.getcwd()
with in_folder("w1"):
    _inside = _os.path.basename(_os.getcwd())
check(_inside, "w1", "inside the block, the current folder is w1")
check(_os.getcwd() == _start, True, "after the block, back in the original folder")
try:
    with in_folder("w2"):
        raise RuntimeError("boom")
except RuntimeError:
    pass
check(_os.getcwd() == _start, True, "back in the original folder even after an error")
@@hint
`old = os.getcwd()`, `os.makedirs(path, exist_ok=True)`, `os.chdir(path)`, then `try: yield` / `finally: os.chdir(old)`.
@@solution
import os
from contextlib import contextmanager


@contextmanager
def in_folder(path):
    old = os.getcwd()
    os.makedirs(path, exist_ok=True)
    os.chdir(path)
    try:
        yield
    finally:
        os.chdir(old)


start = os.getcwd()
with in_folder("work"):
    print(os.path.basename(os.getcwd()))
print(os.getcwd() == start)
