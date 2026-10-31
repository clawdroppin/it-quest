@@lesson i3-exceptions Handling Errors
@@concepts errors
@@text
Real scripts meet a messy world: users type letters where numbers belong, files are missing, servers don't answer. A good script **expects** these problems and handles them, instead of crashing with a traceback.

## try / except

```python
text = "12x"
try:
    n = int(text)
    print("number:", n)
except ValueError:
    print(f"'{text}' is not a number")
print("the script keeps going")
```

Python runs the `try` block. If a `ValueError` happens anywhere in it, Python jumps straight to the matching `except` block. If no error happens, `except` is skipped.

## Common exception types

| Exception | Typical cause |
|---|---|
| `ValueError` | right type, bad value: `int("abc")` |
| `TypeError` | wrong type: `"a" + 1` |
| `KeyError` | missing dict key |
| `IndexError` | list position that doesn't exist |
| `ZeroDivisionError` | dividing by zero |
| `FileNotFoundError` | opening a file that isn't there |
| `PermissionError` | no right to read/write it |
| `TimeoutError`, `ConnectionError` | network trouble |

## Several excepts, else, finally

```python
def safe_ratio(a, b):
    try:
        result = a / b
    except ZeroDivisionError:
        return None
    except TypeError as e:
        print("bad input:", e)
        return None
    else:
        return round(result, 2)       # only if no exception
    finally:
        print("ratio attempted")      # always, no matter what

print(safe_ratio(10, 4))
print(safe_ratio(1, 0))
print(safe_ratio("1", 2))
```

`except TypeError as e` gives you the exception object; printing it shows its message.

## Raising your own

When *your* function gets bad input, **raise** an exception with a clear message:

```python
def set_port(p):
    if not 1 <= p <= 65535:
        raise ValueError(f"port {p} is out of range 1-65535")
    return p

try:
    set_port(70000)
except ValueError as e:
    print("Error:", e)
```

## Good habits

- Catch **specific** exceptions. A bare `except:` also hides your own bugs (typos become silent).
- Keep the `try` block small: only the line that can fail.
- Don't hide errors you can't handle. Let them crash, or log them clearly.

@@quiz
Which is best practice?
- Wrap the whole program in `try: ... except: pass`
* Catch the specific exception you expect, around the line that can fail
- Never use try/except
- Always catch `Exception`
? Specific, narrow handlers fix the problems you expect and still show you the bugs you didn't.

@@quiz
When does the `finally` block run?
- Only if there was an error
- Only if there was no error
* Always, error or not
- Never, it's for documentation
? `finally` is for cleanup that must always happen, like closing a connection.

@@exercise Parse a number safely
Write `to_int(text, default)` that returns `text` converted to an `int`, or `default` if it can't be converted. Surrounding spaces are fine (`int` ignores them).
@@starter
def to_int(text, default):
    pass

print(to_int("42", 0), to_int("4x2", 0))
@@tests
check(to_int("42", 0), 42, "to_int('42', 0)")
check(to_int(" 7 ", 0), 7, "to_int(' 7 ', 0)")
check(to_int("4x2", -1), -1, "to_int('4x2', -1)")
check(to_int("", None), None, "to_int('', None)")
@@hint
`try: return int(text)` and `except ValueError: return default`.
@@solution
def to_int(text, default):
    try:
        return int(text)
    except ValueError:
        return default

print(to_int("42", 0), to_int("4x2", 0))

@@exercise Validate a config
Write `get_timeout(config)` that reads `config["timeout"]` and returns it as an int. Raise a `ValueError` with a helpful message if it's missing, not a number, or not between 1 and 300. The checks only test that the right exception is raised, so the message is up to you.
@@starter
def get_timeout(config):
    pass
@@tests
def raises_value_error(cfg):
    try:
        get_timeout(cfg)
    except ValueError:
        return True
    except Exception:
        return False
    return False

check(get_timeout({"timeout": "30"}), 30, "get_timeout({'timeout': '30'})")
check(get_timeout({"timeout": 300}), 300, "get_timeout({'timeout': 300})")
check(raises_value_error({}), True, "missing key raises ValueError")
check(raises_value_error({"timeout": "soon"}), True, "'soon' raises ValueError")
check(raises_value_error({"timeout": 0}), True, "0 raises ValueError")
@@hint
`if "timeout" not in config: raise ValueError("...")`. Then `int(config["timeout"])` raises a ValueError by itself for "soon". Then check the range.
@@solution
def get_timeout(config):
    if "timeout" not in config:
        raise ValueError("config needs a 'timeout'")
    value = int(config["timeout"])
    if not 1 <= value <= 300:
        raise ValueError(f"timeout {value} must be between 1 and 300")
    return value
