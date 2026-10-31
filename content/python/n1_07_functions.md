@@lesson n1-functions Functions
@@concepts functions
@@text
A **function** is a named, reusable piece of code. You've used Python's: `print`, `len`, `int`. Now you'll write your own.

```python
def greet(name):
    print(f"Welcome to the help desk, {name}!")

greet("Ana")
greet("Ben")
```

- `def` starts a function definition, followed by its name and **parameters** in brackets, and a colon.
- The indented **body** runs only when the function is **called**: `greet("Ana")`.
- `"Ana"` is the **argument**: the value given to the parameter `name` for that call.

## Returning a value

Most useful functions **return** a result instead of printing it, so the caller can use it:

```python
def to_gb(megabytes):
    return megabytes / 1024

size = to_gb(5120)
print(size)
print(to_gb(2048) + to_gb(1024))
```

`return` ends the function immediately and hands the value back. A function without `return` gives back `None` (Python's "nothing").

**Print vs return** is the most common beginner confusion. `print` shows something to a human. `return` hands a value to the code that called the function. A function that calculates should return; let the caller decide whether to print.

## Several parameters, default values

```python
def disk_status(used, total, warn_at=80):
    percent = used / total * 100
    if percent >= warn_at:
        return "WARNING"
    return "OK"

print(disk_status(450, 500))
print(disk_status(450, 500, warn_at=95))
print(disk_status(total=1000, used=100))
```

- `warn_at=80` is a **default**: used if the caller doesn't give one.
- Callers can name arguments (`warn_at=95`), which makes calls readable and order-free.

## Local variables

Variables created inside a function exist only inside it. That's a good thing: functions don't accidentally mess with each other's variables.

```python
def double(x):
    result = x * 2
    return result

print(double(21))
# print(result) here would be a NameError: result only lived inside double
```

## Why functions matter

- **No repetition**: write the logic once, call it everywhere.
- **Names**: `is_valid_ip(text)` explains itself better than ten lines of checks.
- **Testing**: a function that returns a value can be checked automatically. From now on, many exercises check your *functions* rather than your output.

@@quiz
What does this print?

```text
def f(x):
    print(x * 2)

result = f(5)
print(result)
```
- `10` then `10`
* `10` then `None`
- `None` then `10`
- Only `10`
? `f` prints 10 but returns nothing, so `result` is `None`.

@@quiz
In `def connect(host, port=22):`, what is `port=22`?
- A variable that's always 22
* A default value used when the caller doesn't pass a port
- A required argument
- A syntax error
? Callers can write `connect("srv")` (port is 22) or `connect("srv", 2222)`.

@@exercise Convert units
Write a function `bytes_to_mb(n)` that returns `n` bytes in megabytes, **rounded to 2 decimals** (1 MB = 1024 × 1024 bytes).
@@starter
def bytes_to_mb(n):
    pass  # replace this line

print(bytes_to_mb(5242880))
@@tests
check(bytes_to_mb(5242880), 5.0, "bytes_to_mb(5242880)")
check(bytes_to_mb(1500000), 1.43, "bytes_to_mb(1500000)")
check(bytes_to_mb(0), 0.0, "bytes_to_mb(0)")
@@hint
`return round(n / (1024 * 1024), 2)`
@@solution
def bytes_to_mb(n):
    return round(n / (1024 * 1024), 2)

print(bytes_to_mb(5242880))

@@exercise Is it a valid port?
Write `is_valid_port(p)` that returns `True` if `p` is a whole number from **1 to 65535**, and `False` otherwise.
@@starter
def is_valid_port(p):
    pass

print(is_valid_port(443))
@@tests
check(is_valid_port(443), True, "is_valid_port(443)")
check(is_valid_port(0), False, "is_valid_port(0)")
check(is_valid_port(65535), True, "is_valid_port(65535)")
check(is_valid_port(65536), False, "is_valid_port(65536)")
check(is_valid_port(-1), False, "is_valid_port(-1)")
@@hint
A comparison already *is* a boolean: `return 1 <= p <= 65535`.
@@solution
def is_valid_port(p):
    return 1 <= p <= 65535

print(is_valid_port(443))

@@exercise Classify a port
Write `port_kind(p)` that returns `"well-known"` for ports 0–1023, `"registered"` for 1024–49151, and `"dynamic"` for 49152–65535.
@@starter
def port_kind(p):
    pass

print(port_kind(80))
@@tests
check(port_kind(80), "well-known", "port_kind(80)")
check(port_kind(1023), "well-known", "port_kind(1023)")
check(port_kind(3389), "registered", "port_kind(3389)")
check(port_kind(49152), "dynamic", "port_kind(49152)")
@@hint
`if p <= 1023: return "well-known"` and so on. After a `return`, the function is over, so you don't even need `elif`.
@@solution
def port_kind(p):
    if p <= 1023:
        return "well-known"
    if p <= 49151:
        return "registered"
    return "dynamic"

print(port_kind(80))
@@text
> These three port ranges are defined by IANA, and they're a classic **Network+** and **A+** exam question. Remember: 0–1023 well-known (HTTP 80, HTTPS 443, SSH 22, RDP is 3389, which is in the registered range).
