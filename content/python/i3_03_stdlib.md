@@lesson i3-stdlib Modules & the Standard Library
@@concepts modules
@@text
Python comes with "batteries included": a huge **standard library** of ready-made modules. You use them with `import`.

```python
import math
import random
from datetime import datetime, timedelta

print(math.sqrt(144), math.ceil(4.1), math.floor(4.9))
print(random.randint(1, 6))
print(datetime.now().year >= 2024)
```

- `import math` → use it as `math.sqrt(...)`.
- `from datetime import datetime` → use `datetime` directly.
- `import something as short` → a nickname: `import datetime as dt`.

## The ones IT people use every day

| Module | For |
|---|---|
| `os`, `pathlib`, `shutil` | files, folders, environment variables, copying |
| `sys` | arguments, exit codes, Python version |
| `subprocess` | run other programs and commands |
| `datetime`, `time` | dates, times, durations, timestamps |
| `json`, `csv` | data formats |
| `re` | regular expressions (text patterns) |
| `collections` | `Counter`, `defaultdict`, `deque` |
| `ipaddress` | IPs, networks, subnets |
| `socket`, `urllib`, `http` | networking |
| `logging`, `argparse` | proper command-line tools |
| `hashlib`, `secrets` | hashes, secure random tokens |
| `unittest` | tests |

## datetime

```python
from datetime import datetime, timedelta

start = datetime(2026, 10, 1, 9, 30)
end = datetime(2026, 10, 3, 14, 0)
print(end - start)
print((end - start).total_seconds() / 3600, "hours")
print(start + timedelta(days=90))       # e.g. password expiry
print(start.strftime("%Y-%m-%d %H:%M"))
print(datetime.strptime("2026-10-31 08:15", "%Y-%m-%d %H:%M").hour)
```

`strftime` turns a date into text ("string format time"); `strptime` parses text into a date ("string parse time"). `%Y` year, `%m` month, `%d` day, `%H` hour, `%M` minute, `%S` second.

## collections.Counter

The counting pattern from the dictionary lesson, in one line:

```python
from collections import Counter

ips = ["10.0.0.1", "10.0.0.2", "10.0.0.1", "10.0.0.9", "10.0.0.1"]
c = Counter(ips)
print(c["10.0.0.1"])
print(c.most_common(2))
```

## hashlib and secrets

```python
import hashlib, secrets

print(hashlib.sha256(b"hello").hexdigest()[:16])
print(len(secrets.token_hex(16)))      # a secure random token
```

`secrets` is for passwords and tokens; `random` is *not* secure for that. (A favorite Security+ point.)

## Your own modules

Any `.py` file is a module. If you have `tools.py` with a function `ping`, another file in the same folder can `import tools` and call `tools.ping()`. You'll do this in the Projects tab.

@@quiz
You need a random password reset token. Which module?
- `random`
* `secrets`
- `math`
- `time`
? `secrets` uses the operating system's cryptographically secure randomness. `random` is predictable.

@@quiz
What does `Counter(["web", "db", "web"]).most_common(1)` return?
* `[("web", 2)]`
- `"web"`
- `{"web": 2, "db": 1}`
- `2`
? most_common(n) returns the n most frequent items with their counts.

@@quiz
How do you get the date 30 days from now?
* `datetime.now() + timedelta(days=30)`
- `datetime.now() + 30`
- `datetime.now().add(30)`
- `date.today(30)`
? Arithmetic on datetimes uses timedelta.

@@drill
Which module should you use to generate a secure random token?
= secrets
? secrets.token_hex(16); random is not for security.

@@flashcards
`from datetime import datetime, timedelta` :: dates/times, and durations to add to them
`strftime` / `strptime` :: date → text / text → date
`Counter(list).most_common(n)` :: the n most frequent items
`secrets.token_hex(16)` :: secure random token (not `random`!)
`hashlib.sha256(b"...").hexdigest()` :: SHA-256 hash as hex text

@@exercise Password expiry
Write `expires_on(changed, days)` that takes the date a password was changed as text `"YYYY-MM-DD"` and returns the expiry date (changed + `days` days) as text in the same format.
@@starter
from datetime import datetime, timedelta


def expires_on(changed, days):
    pass


print(expires_on("2026-10-01", 90))
@@tests
check(expires_on("2026-10-01", 90), "2026-12-30", "expires_on('2026-10-01', 90)")
check(expires_on("2024-02-28", 1), "2024-02-29", "leap year")
check(expires_on("2026-12-31", 1), "2027-01-01", "new year")
@@hint
`d = datetime.strptime(changed, "%Y-%m-%d")`, add `timedelta(days=days)`, then `.strftime("%Y-%m-%d")`.
@@solution
from datetime import datetime, timedelta


def expires_on(changed, days):
    d = datetime.strptime(changed, "%Y-%m-%d")
    return (d + timedelta(days=days)).strftime("%Y-%m-%d")


print(expires_on("2026-10-01", 90))

@@exercise Top talkers
Write `top_talkers(ips, n)` that returns the `n` most frequent IPs as a list of `(ip, count)` tuples, most frequent first. Use `Counter`.
@@starter
from collections import Counter


def top_talkers(ips, n):
    pass


print(top_talkers(["a", "b", "a", "c", "a", "b"], 2))
@@tests
check(top_talkers(["a", "b", "a", "c", "a", "b"], 2), [("a", 3), ("b", 2)], "top_talkers(sample, 2)")
check(top_talkers([], 3), [], "empty list")
@@requires
Counter
@@hint
`return Counter(ips).most_common(n)`
@@solution
from collections import Counter


def top_talkers(ips, n):
    return Counter(ips).most_common(n)


print(top_talkers(["a", "b", "a", "c", "a", "b"], 2))
