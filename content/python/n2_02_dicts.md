@@lesson n2-dicts Dictionaries
@@concepts maps
@@text
A **dictionary** (`dict`) stores **key → value** pairs. Instead of finding things by position, you find them by name. It's how you'd store "which user has which role", "how many errors per server", or any record with named fields.

```python
server = {"name": "web01", "ip": "10.0.0.5", "cpu": 8}
print(server["ip"])
server["cpu"] = 16          # change a value
server["os"] = "Ubuntu"     # add a new key
print(server)
print(len(server))
```

Keys are usually strings; each key appears only once.

## Missing keys

`server["ram"]` raises a `KeyError` if there's no such key. Check first with `in`, or use `.get`, which returns a default instead:

```python
server = {"name": "web01"}
print("ram" in server)
print(server.get("ram"))
print(server.get("ram", "unknown"))
```

## Looping over a dictionary

```python
ports = {"ssh": 22, "http": 80, "https": 443}
for name in ports:                   # the keys
    print(name)
for name, number in ports.items():  # key and value together
    print(f"{name:>6} -> {number}")
print(list(ports.values()))
```

## Counting with a dictionary

One of the most useful patterns in all of IT scripting: count how often each thing appears.

```python
events = ["login", "error", "login", "logout", "error", "error"]
counts = {}
for e in events:
    counts[e] = counts.get(e, 0) + 1
print(counts)
```

`counts.get(e, 0)` is "the count so far, or 0 the first time".

## Lists of dictionaries

Real data usually looks like a **list of records**, each a dictionary. This is exactly the shape of JSON from a web API or a row from a CSV file:

```python
users = [
    {"name": "ana", "dept": "IT", "active": True},
    {"name": "ben", "dept": "HR", "active": False},
    {"name": "carl", "dept": "IT", "active": True},
]
for u in users:
    if u["active"] and u["dept"] == "IT":
        print(u["name"])
```

## Grouping

```python
users = [{"name": "ana", "dept": "IT"}, {"name": "ben", "dept": "HR"}, {"name": "carl", "dept": "IT"}]
groups = {}
for u in users:
    groups.setdefault(u["dept"], []).append(u["name"])
print(groups)
```

`setdefault(key, [])` returns the list for that key, creating an empty one the first time.

@@quiz
`d = {"a": 1}`. What does `d.get("b", 0)` return?
- An error
- `None`
* `0`
- `"b"`
? `.get` returns the default you give it when the key is missing.

@@flashcards
`d[key]` :: the value for key; KeyError if missing
`d.get(key, default)` :: the value, or default if the key is missing
`d.items()` :: (key, value) pairs, for looping
`counts[x] = counts.get(x, 0) + 1` :: the counting pattern
`key in d` :: is this key in the dictionary?

@@exercise Failed logins per user
Write `count_failures(log)`: `log` is a list of `(user, ok)` pairs, where `ok` is `True` for a successful login. Return a dict mapping each user to their number of **failed** logins. Users with zero failures should not appear.
@@starter
def count_failures(log):
    pass

print(count_failures([("ana", True), ("ben", False), ("ben", False), ("carl", False)]))
@@tests
check(count_failures([("ana", True), ("ben", False), ("ben", False), ("carl", False)]), {"ben": 2, "carl": 1}, "count_failures(sample)")
check(count_failures([("ana", True)]), {}, "only successes")
check(count_failures([]), {}, "empty log")
@@hint
`for user, ok in log:` — if `not ok`, do the counting pattern.
@@solution
def count_failures(log):
    counts = {}
    for user, ok in log:
        if not ok:
            counts[user] = counts.get(user, 0) + 1
    return counts

print(count_failures([("ana", True), ("ben", False), ("ben", False), ("carl", False)]))

@@exercise Brute-force suspects
Using the same kind of log, write `suspects(log, limit)` that returns a **sorted list** of users with **at least** `limit` failed logins. You may call your own `count_failures` from the first exercise, so paste it in too.
@@starter
def suspects(log, limit):
    pass

log = [("ana", False), ("ben", False), ("ben", False), ("ben", False), ("ana", True), ("eve", False), ("eve", False), ("eve", False)]
print(suspects(log, 3))
@@tests
log = [("ana", False), ("ben", False), ("ben", False), ("ben", False), ("ana", True), ("eve", False), ("eve", False), ("eve", False)]
check(suspects(log, 3), ["ben", "eve"], "suspects(log, 3)")
check(suspects(log, 1), ["ana", "ben", "eve"], "suspects(log, 1)")
check(suspects(log, 5), [], "suspects(log, 5)")
@@hint
Count failures into a dict, then loop over `.items()` and keep the users whose count is `>= limit`. Return `sorted(...)`.
@@solution
def count_failures(log):
    counts = {}
    for user, ok in log:
        if not ok:
            counts[user] = counts.get(user, 0) + 1
    return counts


def suspects(log, limit):
    return sorted(user for user, n in count_failures(log).items() if n >= limit)

log = [("ana", False), ("ben", False), ("ben", False), ("ben", False), ("ana", True), ("eve", False), ("eve", False), ("eve", False)]
print(suspects(log, 3))
@@text
> Spotting many failed logins for one account is how **brute-force attacks** are detected. Security tools (and the Security+ exam) call this *account lockout threshold* and *anomaly detection*. You just wrote the core of it.
