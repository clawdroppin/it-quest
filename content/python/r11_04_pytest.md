@@lesson r11-pytest pytest in Depth: Fixtures, Parametrize and Mocking
@@concepts testing
@@text
The testing lesson introduced `assert` and `unittest`. Nearly every professional Python project uses **pytest**: tests are plain functions with plain `assert`, and its features (fixtures, parametrisation, plugins) keep large test suites short and readable. This lesson also covers mocking with the standard library's `unittest.mock`.

## pytest basics

```python,norun
# tests/test_quota.py      run with:  pytest -q   (or uv run pytest)
from assetctl.quota import percent_used

def test_percent_used():
    assert percent_used(50, 200) == 25.0

def test_zero_total_raises():
    import pytest
    with pytest.raises(ValueError, match="total must be positive"):
        percent_used(1, 0)
```

- Files `test_*.py`, functions `test_*`; pytest finds them automatically.
- When an `assert` fails, pytest shows the values on both sides (it rewrites asserts to explain them).
- `pytest -k quota` runs matching tests, `-x` stops at the first failure, `--lf` reruns last failures, `-vv` shows full diffs, `--pdb` drops into the debugger.

## Fixtures: reusable setup

A **fixture** is a function that provides something a test needs; tests ask for it by parameter name:

```python,norun
import sqlite3, pytest

@pytest.fixture
def db():
    conn = sqlite3.connect(":memory:")
    conn.execute("CREATE TABLE assets (tag TEXT PRIMARY KEY, owner TEXT)")
    yield conn                    # the test runs here
    conn.close()                  # teardown, even if the test failed

def test_insert(db):
    db.execute("INSERT INTO assets VALUES ('LT-1', 'maya')")
    assert db.execute("SELECT COUNT(*) FROM assets").fetchone()[0] == 1
```

- **Scope**: `@pytest.fixture(scope="session")` creates one for the whole run (expensive things like a Docker database).
- Fixtures can use other fixtures. Shared ones live in `conftest.py`.
- Built-in fixtures: `tmp_path` (a fresh temporary folder), `monkeypatch` (temporarily set attributes/env vars), `capsys` (captured output), `caplog` (captured logs).

```python,norun
def test_reads_config(tmp_path, monkeypatch):
    cfg = tmp_path / "app.toml"
    cfg.write_text('port = 8080')
    monkeypatch.setenv("APP_CONFIG", str(cfg))
    assert load_settings().port == 8080
```

## Parametrize: one test, many cases

```python,norun
@pytest.mark.parametrize(("text", "expected"), [
    ("22", 22),
    (" 443 ", 443),
    ("65535", 65535),
])
def test_parse_port(text, expected):
    assert parse_port(text) == expected

@pytest.mark.parametrize("bad", ["0", "70000", "abc", ""])
def test_parse_port_rejects(bad):
    with pytest.raises(ValueError):
        parse_port(bad)
```

Each case is reported separately, so you see exactly which input failed. Other marks: `skip`, `skipif(sys.platform != "win32", reason=...)`, `xfail` (known bug), custom marks like `slow` selected with `-m "not slow"`.

## Mocking with unittest.mock

When code calls something slow, external or random (an HTTP API, the clock, `subprocess`), replace it in tests with a **mock**:

```python
from unittest.mock import Mock, patch
import time

def wait_until_up(check, attempts, sleep=time.sleep):
    for i in range(attempts):
        if check():
            return i + 1
        sleep(2)
    return None

check = Mock(side_effect=[False, False, True])    # returns these values in turn
sleep = Mock()
print(wait_until_up(check, 5, sleep))
print(check.call_count, sleep.call_args_list)
sleep.assert_called_with(2)
```

- `Mock(return_value=...)` returns a fixed value; `side_effect` returns values in turn, or raises an exception (`side_effect=TimeoutError("x")`).
- Mocks **record calls**: `called`, `call_count`, `call_args`, `assert_called_once_with(...)`.
- `patch("module.name")` temporarily replaces an attribute **where it's looked up**: if `app.py` does `from requests import get`, patch `"app.get"`, not `"requests.get"`.
- `Mock(spec=RealClass)` / `create_autospec` make the mock reject methods the real class doesn't have, catching typos.

Prefer **passing dependencies in** (like `sleep=` above) over patching: it's explicit and needs no knowledge of import paths. Patch for code you don't control.

## Useful plugins

| Plugin | Adds |
|---|---|
| `pytest-cov` | coverage reports (`--cov=assetctl --cov-report=term-missing`) |
| `pytest-xdist` | run tests in parallel (`-n auto`) |
| `pytest-asyncio` / `anyio` | `async def` tests |
| `pytest-mock` | `mocker` fixture around unittest.mock |
| `hypothesis` | property-based testing with shrinking |
| `pytest-httpx` / `respx` / `responses` | fake HTTP responses |
| `freezegun` / `time-machine` | control the current time |

## A healthy test suite

- Fast unit tests for logic (most tests), fewer integration tests against real databases/APIs (`testcontainers`), very few end-to-end tests.
- Tests run in CI on every pull request, on every supported Python version.
- A failing test is fixed or deleted, never ignored.

@@quiz
How does a pytest test get a fixture?
* By naming the fixture as a parameter of the test function
- By importing it
- By calling it inside the test
- With a decorator on the test
? pytest injects fixtures by name.

@@quiz
What does code after `yield` in a fixture do?
- Nothing
* Teardown: it runs after the test, even if the test failed
- It runs before the test
- It becomes another test
? Like `finally` for the test.

@@quiz
`app.py` does `from requests import get`. What should you patch?
- "requests.get"
* "app.get"
- "get"
- "requests"
? Patch where the name is looked up.

@@quiz
What does `Mock(side_effect=[1, 2])` do when called three times?
- Returns 1, 2, 1
* Returns 1, then 2, then raises StopIteration
- Always returns [1, 2]
- Raises immediately
? A list side effect is consumed one value per call.

@@drill
Which decorator runs one pytest test with many inputs?
= @pytest.mark.parametrize
= pytest.mark.parametrize
= parametrize
? Each case is reported separately.

@@flashcards
pytest :: the standard Python test framework, plain functions with assert
Fixture :: reusable setup/teardown injected by parameter name
`conftest.py` :: shared fixtures for a folder of tests
`tmp_path` :: built-in fixture: a fresh temporary directory
`monkeypatch` :: built-in fixture to temporarily change attributes and env vars
`@pytest.mark.parametrize` :: run one test with many inputs
Mock :: fake object that records calls and returns configured values
`side_effect` :: values to return in turn, or an exception to raise
`patch` :: temporarily replace a name where it's looked up
Coverage :: which lines tests execute (`pytest-cov`)

@@exercise Test with mocks
`notify_on_failures(results, mailer, clock)` sends one summary email when any check failed. It's given. Write `test_notify()` using `unittest.mock.Mock` objects for `mailer` and `clock` that verifies (with `assert` statements, so a failure raises `AssertionError`):

1. With failures `{"disk": False, "ping": True, "dns": False}` and a clock whose `now()` returns `"09:00"`, `mailer.send` is called **once** with `("ops@corp.example", "09:00: 2 checks failed: disk, dns")`, and the function returns True.
2. With no failures, `mailer.send` is **not** called and it returns False.
3. If `mailer.send` raises `ConnectionError`, the function returns False (doesn't crash).

Return `"passed"` at the end of `test_notify()`.
@@starter
from unittest.mock import Mock


def notify_on_failures(results, mailer, clock):
    failed = [name for name, ok in results.items() if not ok]
    if not failed:
        return False
    try:
        mailer.send("ops@corp.example", f"{clock.now()}: {len(failed)} checks failed: {', '.join(failed)}")
    except ConnectionError:
        return False
    return True


def test_notify():
    pass


print(test_notify())
@@tests
check(test_notify(), "passed", "your test passes against the correct code")
original = notify_on_failures
def broken_once(results, mailer, clock):
    original(results, mailer, clock)
    return original(results, mailer, clock)
notify_on_failures = broken_once
try:
    test_notify()
    check("passed", "AssertionError", "your test catches a double send")
except AssertionError:
    pass
def broken_silent(results, mailer, clock):
    return any(not ok for ok in results.values())
notify_on_failures = broken_silent
try:
    test_notify()
    check("passed", "AssertionError", "your test catches a missing send")
except AssertionError:
    pass
notify_on_failures = original
@@hint
`mailer = Mock(); clock = Mock(); clock.now.return_value = "09:00"`. Then `assert notify_on_failures({...}, mailer, clock) is True` and `mailer.send.assert_called_once_with("ops@corp.example", "...")` (that method raises AssertionError itself). For case 2 use `mailer.send.assert_not_called()`. For case 3: `mailer.send.side_effect = ConnectionError("smtp down")`.
@@solution
from unittest.mock import Mock


def notify_on_failures(results, mailer, clock):
    failed = [name for name, ok in results.items() if not ok]
    if not failed:
        return False
    try:
        mailer.send("ops@corp.example", f"{clock.now()}: {len(failed)} checks failed: {', '.join(failed)}")
    except ConnectionError:
        return False
    return True


def test_notify():
    mailer, clock = Mock(), Mock()
    clock.now.return_value = "09:00"
    assert notify_on_failures({"disk": False, "ping": True, "dns": False}, mailer, clock) is True
    mailer.send.assert_called_once_with("ops@corp.example", "09:00: 2 checks failed: disk, dns")

    mailer = Mock()
    assert notify_on_failures({"disk": True}, mailer, clock) is False
    mailer.send.assert_not_called()

    mailer = Mock()
    mailer.send.side_effect = ConnectionError("smtp down")
    assert notify_on_failures({"disk": False}, mailer, clock) is False
    return "passed"


print(test_notify())

@@exercise A mini test runner
Build the core of a pytest-like runner to see how fixtures and parametrisation work:

- `fixture(func)`: a decorator that registers `func` in a `FIXTURES` dict under its name and returns it unchanged.
- `parametrize(name, values)`: a decorator that stores `(name, values)` on the test function as attribute `params` (a list, so it can be applied more than once... one level is enough here).
- `run(tests)`: for each test function, find its parameter names with `inspect.signature`; build arguments from fixtures (call the fixture function); if the test has `params`, run it once per value with that parameter set. Record results as strings: `"test_name PASSED"`, `"test_name[value] FAILED: <message>"` for an `AssertionError`. Return the list.
@@starter
import inspect

FIXTURES = {}


def fixture(func):
    pass


def parametrize(name, values):
    pass


def run(tests):
    pass


@fixture
def limits():
    return {"disk": 90}


@parametrize("used", [50, 95])
def test_disk(limits, used):
    assert used < limits["disk"], f"{used} is over the limit"


def test_plain():
    assert 1 + 1 == 2


for line in run([test_disk, test_plain]):
    print(line)
@@expected
test_disk[50] PASSED
test_disk[95] FAILED: 95 is over the limit
test_plain PASSED
@@tests
check("limits" in FIXTURES, True, "fixture registered")
check(limits(), {"disk": 90}, "fixture returns function unchanged")
@fixture
def name():
    return "maya"
def test_name(name):
    assert name == "li", "wrong name"
check(run([test_name]), ["test_name FAILED: wrong name"], "fixture injection and failure")
@@hint
`parametrize` returns `def decorate(func): func.params = (name, values); return func`. In `run`, `names = list(inspect.signature(test).parameters)`; `kwargs = {n: FIXTURES[n]() for n in names if n in FIXTURES}`. Build a list of `(label, kwargs)` cases: one per value if `hasattr(test, "params")`. Wrap each call in `try/except AssertionError as e`.
@@solution
import inspect

FIXTURES = {}


def fixture(func):
    FIXTURES[func.__name__] = func
    return func


def parametrize(name, values):
    def decorate(func):
        func.params = (name, values)
        return func
    return decorate


def run(tests):
    results = []
    for test in tests:
        names = list(inspect.signature(test).parameters)
        base = {n: FIXTURES[n]() for n in names if n in FIXTURES}
        if hasattr(test, "params"):
            pname, values = test.params
            cases = [(f"{test.__name__}[{v}]", {**base, pname: v}) for v in values]
        else:
            cases = [(test.__name__, base)]
        for label, kwargs in cases:
            try:
                test(**kwargs)
                results.append(f"{label} PASSED")
            except AssertionError as e:
                results.append(f"{label} FAILED: {e}")
    return results


@fixture
def limits():
    return {"disk": 90}


@parametrize("used", [50, 95])
def test_disk(limits, used):
    assert used < limits["disk"], f"{used} is over the limit"


def test_plain():
    assert 1 + 1 == 2


for line in run([test_disk, test_plain]):
    print(line)
