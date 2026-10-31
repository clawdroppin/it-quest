@@lesson n2-tuples-sets Tuples & Sets
@@concepts lists
@@text
Two more collection types, each perfect for one job.

## Tuples: fixed groups

A **tuple** is like a list that can't be changed. Use it for a small, fixed group of values that belong together, like an (ip, port) pair:

```python
endpoint = ("10.0.0.5", 443)
print(endpoint[0], endpoint[1])
ip, port = endpoint          # "unpacking" into two variables
print(f"{ip}:{port}")
```

Functions can return several values as a tuple:

```python
def split_host(text):
    host, port = text.split(":")
    return host, int(port)

h, p = split_host("db01:5432")
print(h, p + 1)
```

That's also why `a, b = b, a` swaps two variables: the right side builds a tuple, the left side unpacks it.

## Sets: unique items, fast lookups

A **set** holds each value at most once, in no particular order:

```python
seen = {"10.0.0.1", "10.0.0.2", "10.0.0.1"}
print(len(seen))
seen.add("10.0.0.3")
print("10.0.0.2" in seen)
print(set(["a", "b", "a"]))
```

Sets can compare groups, which is great for audits:

```python
allowed = {"ana", "ben", "carl"}
logged_in = {"ben", "carl", "mallory"}
print(logged_in - allowed)      # in logged_in but not allowed
print(logged_in & allowed)      # in both (intersection)
print(logged_in | allowed)      # in either (union)
```

(`print` shows sets in any order; use `sorted(...)` when you need a fixed order.)

| Type | Ordered | Changeable | Duplicates | Use for |
|---|---|---|---|---|
| list `[ ]` | yes | yes | yes | sequences of items |
| tuple `( )` | yes | no | yes | fixed records, multiple return values |
| set `{ }` | no | yes | no | uniqueness, membership, comparing groups |
| dict `{k: v}` | yes | yes | unique keys | lookups by name |

@@quiz
You have two lists of usernames: accounts in Active Directory, and people in the HR system. You want the AD accounts of people who've left. What's the best tool?
- A loop with `break`
* Set difference: `set(ad) - set(hr)`
- A tuple
- `sorted()`
? Set difference gives everything in the first set that's not in the second, in one step. This is a real "orphaned account" audit.

@@exercise Orphaned accounts
Write `orphaned(ad_users, hr_staff)` that returns a **sorted list** of usernames that exist in `ad_users` but not in `hr_staff`. Compare them **case-insensitively** (return lowercase names).
@@starter
def orphaned(ad_users, hr_staff):
    pass

print(orphaned(["Ana", "ben", "Old.Admin", "carl"], ["ana", "BEN", "carl"]))
@@tests
check(orphaned(["Ana", "ben", "Old.Admin", "carl"], ["ana", "BEN", "carl"]), ["old.admin"], "orphaned(sample)")
check(orphaned(["x", "y"], []), ["x", "y"], "nobody in HR")
check(orphaned([], ["a"]), [], "no AD accounts")
@@hint
Build two sets of lowercase names with a comprehension: `{u.lower() for u in ad_users}`. Subtract, then `sorted(...)`.
@@solution
def orphaned(ad_users, hr_staff):
    ad = {u.lower() for u in ad_users}
    hr = {s.lower() for s in hr_staff}
    return sorted(ad - hr)

print(orphaned(["Ana", "ben", "Old.Admin", "carl"], ["ana", "BEN", "carl"]))

@@exercise Parse an endpoint
Write `parse_endpoint(text)` that takes text like `"10.0.0.5:8080"` and returns a tuple `(host, port)` with the port as an `int`. If there's no `:`, use port `80`.
@@starter
def parse_endpoint(text):
    pass

print(parse_endpoint("10.0.0.5:8080"))
@@tests
check(parse_endpoint("10.0.0.5:8080"), ("10.0.0.5", 8080), "parse_endpoint('10.0.0.5:8080')")
check(parse_endpoint("example.com"), ("example.com", 80), "parse_endpoint('example.com')")
@@hint
`if ":" in text:` then `host, port = text.split(":")` and return `host, int(port)`.
@@solution
def parse_endpoint(text):
    if ":" in text:
        host, port = text.split(":")
        return host, int(port)
    return text, 80

print(parse_endpoint("10.0.0.5:8080"))
