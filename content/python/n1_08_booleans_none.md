@@lesson n1-booleans-none Booleans, Truthiness and None
@@concepts conditions, variables
@@text
Every `if` and `while` asks a yes/no question. This lesson looks closely at how Python answers it: the `bool` type, **truthiness**, the logical operators, and `None`, the value that means "nothing here".

## True and False

`True` and `False` are the two values of type `bool`. Comparisons produce them:

```python
disk_free = 12
print(disk_free < 10, disk_free == 12, type(True))
print(3 < disk_free <= 20)        # chained comparison: 3 < x and x <= 20
```

`bool` is actually a kind of `int`: `True + True` is `2`. That's handy for counting: `sum(x > 90 for x in loads)` counts how many loads are above 90.

## Truthiness

Python lets **any** value stand in a condition. Values that count as false ("falsy"):

| Falsy | Example |
|---|---|
| `False`, `None` | |
| zero numbers | `0`, `0.0` |
| empty collections and strings | `""`, `[]`, `{}`, `set()`, `()` |

Everything else is **truthy**. So `if errors:` means "if the errors list is not empty", the idiomatic way to write it:

```python
errors = []
if not errors:
    print("all checks passed")

name = input_name = ""
print(bool(name), bool("0"), bool([0]))   # "0" and [0] are NOT empty
```

## and, or, not, and short-circuiting

- `a and b`: true only if both are.
- `a or b`: true if at least one is.
- `not a`: flips it.

They **short-circuit**: `and` stops at the first falsy value, `or` stops at the first truthy one. That makes safe checks possible:

```python
user = None
if user is not None and user["admin"]:   # second part never runs when user is None
    print("admin")
```

`and`/`or` return **one of the operands**, not necessarily `True`/`False`. That gives a common default-value idiom:

```python
configured = ""
port_text = configured or "8080"     # "" is falsy, so the default is used
print(port_text)
```

Careful: `0 or 10` gives `10`, so this idiom is wrong when `0` is a valid value. Then write `x if x is not None else 10`.

## None

`None` is Python's "no value": a function without `return` returns `None`, a missing setting can be `None`, `dict.get` returns `None` for a missing key.

```python
def find_user(name):
    users = {"maya": "admin"}
    return users.get(name)        # None if missing

role = find_user("li")
if role is None:
    print("unknown user")
```

**Always compare with `is None` / `is not None`**, not `== None`. `is` checks "the very same object", and there's only one `None`.

## any() and all()

```python
ports = [22, 80, 443]
print(any(p < 1024 for p in ports))   # at least one privileged port?
print(all(p > 0 for p in ports))      # every port positive?
print(all([]))                        # True: nothing violates the rule
```

## Common mistakes

- `if x == True:` → just `if x:`.
- `if status == "up" or "ok":` is **always true** (`"ok"` is truthy). Write `if status in ("up", "ok"):`.
- Treating `0` or `""` as "missing" when they're valid values: use `is None`.

@@quiz
Which of these is truthy?
- `0`
- `""`
* `"0"`
- `[]`
? `"0"` is a non-empty string, so it's truthy.

@@quiz
What does `"" or "default"` evaluate to?
- `True`
- `""`
* `"default"`
- `False`
? `or` returns the first truthy operand (or the last one).

@@quiz
Why is `if status == "up" or "ok":` a bug?
* It's parsed as `(status == "up") or "ok"`, and `"ok"` is always truthy
- `or` can't be used with strings
- It raises a SyntaxError
- It compares the wrong types
? Use `status in ("up", "ok")`.

@@quiz
How should you check for None?
- `x == None`
* `x is None`
- `not x`
- `x.isnone()`
? `not x` is also true for 0, "" and [].

@@flashcards
Truthy :: any value that counts as true in a condition
Falsy values :: False, None, 0, 0.0, "", [], {}, set(), ()
Short-circuit :: `and`/`or` stop evaluating as soon as the answer is known
`x or default` :: returns x if x is truthy, otherwise default
`is None` :: the correct way to test for None
`any(iterable)` :: True if at least one item is truthy
`all(iterable)` :: True if every item is truthy (True for an empty iterable)

@@exercise Health check summary
Write `health(checks)` where `checks` is a dict of check name → result. A result is `True` (passed), `False` (failed) or `None` (not run).

Return a string:
- `"no checks"` if the dict is empty
- `"OK"` if every check passed
- `"FAILED: a, b"` listing the failed names in order if any failed
- otherwise (nothing failed but some not run) `"INCOMPLETE: c"` listing the not-run names
@@starter
def health(checks):
    pass


print(health({"disk": True, "dns": None, "ping": False, "cert": False}))
@@tests
check(health({}), "no checks", "empty")
check(health({"disk": True, "ping": True}), "OK", "all passed")
check(health({"disk": True, "dns": None, "ping": False, "cert": False}), "FAILED: ping, cert", "failures win")
check(health({"disk": True, "dns": None, "ntp": None}), "INCOMPLETE: dns, ntp", "not run")
@@hint
Build two lists: `failed = [n for n, r in checks.items() if r is False]` and `skipped = [n for n, r in checks.items() if r is None]`. Use `is False` / `is None` so the three states stay distinct. Then `", ".join(failed)`.
@@solution
def health(checks):
    if not checks:
        return "no checks"
    failed = [name for name, result in checks.items() if result is False]
    skipped = [name for name, result in checks.items() if result is None]
    if failed:
        return "FAILED: " + ", ".join(failed)
    if skipped:
        return "INCOMPLETE: " + ", ".join(skipped)
    return "OK"


print(health({"disk": True, "dns": None, "ping": False, "cert": False}))

@@exercise Settings with defaults
Write `setting(user, defaults, key)` that returns the user's value for `key` if the user set it (**even if it's `0`, `False` or `""`**), otherwise the default, otherwise `None`. A user value of `None` counts as "not set".

Then write `count_true(values)` that returns how many items are truthy, using `sum`.
@@starter
def setting(user, defaults, key):
    pass


def count_true(values):
    pass


print(setting({"retries": 0}, {"retries": 3, "port": 22}, "retries"))
@@tests
d = {"retries": 3, "port": 22, "verbose": True}
check(setting({"retries": 0}, d, "retries"), 0, "0 is a real value")
check(setting({"verbose": False}, d, "verbose"), False, "False is a real value")
check(setting({"port": None}, d, "port"), 22, "None means not set")
check(setting({}, d, "port"), 22, "missing uses default")
check(setting({}, d, "color"), None, "nowhere")
check(count_true([0, 1, "", "x", None, [0]]), 3, "truthy count")
@@hint
`value = user.get(key)` then `if value is not None: return value` and `return defaults.get(key)`. Don't use `user.get(key) or defaults.get(key)`: that loses 0 and False. `count_true`: `sum(bool(v) for v in values)`.
@@solution
def setting(user, defaults, key):
    value = user.get(key)
    if value is not None:
        return value
    return defaults.get(key)


def count_true(values):
    return sum(bool(v) for v in values)


print(setting({"retries": 0}, {"retries": 3, "port": 22}, "retries"))
