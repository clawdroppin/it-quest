@@lesson n1-loop-patterns Loop Patterns: enumerate, zip, range, break and else
@@concepts loops, lists
@@text
Most loops in real programs follow a handful of patterns. Python has a tool for each, and using them makes loops shorter and harder to get wrong.

## range in detail

`range(start, stop, step)` produces numbers from `start` up to **but not including** `stop`:

```python
print(list(range(5)))           # 0..4
print(list(range(1, 11, 3)))    # 1, 4, 7, 10
print(list(range(10, 0, -2)))   # counting down: 10, 8, 6, 4, 2
```

`range` doesn't build a list in memory; `range(1_000_000_000)` is instant.

## enumerate: index and value together

Instead of `for i in range(len(items))`, use `enumerate`:

```python
servers = ["web01", "web02", "db01"]
for number, name in enumerate(servers, start=1):
    print(f"{number}. {name}")
```

## zip: walk several lists side by side

```python
names = ["web01", "db01", "mail01"]
loads = [35, 92, 51]
for name, load in zip(names, loads):
    print(f"{name:<8}{load:>4}%")

lookup = dict(zip(names, loads))      # build a dict from two lists
print(lookup)
```

`zip` stops at the **shortest** input. Use `zip(a, b, strict=True)` to get an error when lengths differ (usually a bug), or `itertools.zip_longest` to pad.

## Looping over dictionaries

```python
quota = {"maya": 50, "li": 120}
for user in quota:                     # keys
    print(user)
for user, gb in quota.items():         # pairs
    print(user, gb)
```

Don't add or remove keys while looping over a dict; loop over `list(quota)` if you must.

## break, continue and loop else

- `break` leaves the loop immediately.
- `continue` skips to the next item.
- A loop's `else:` block runs only if the loop finished **without** `break`: perfect for searches.

```python
ports = [22, 80, 8080, 443]
for port in ports:
    if port == 3389:
        print("RDP is open!")
        break
else:
    print("no RDP port found")
```

## Accumulator patterns

```python
sizes = [120, 3400, 56, 990]
total = 0
biggest = None
for s in sizes:
    total += s
    if biggest is None or s > biggest:
        biggest = s
print(total, biggest)
print(sum(sizes), max(sizes))     # the built-ins do the same
```

Counting into a dict:

```python
levels = ["INFO", "ERROR", "INFO", "WARN", "INFO"]
counts = {}
for level in levels:
    counts[level] = counts.get(level, 0) + 1
print(counts)
```

## while loops: when you don't know how many times

```python
attempt = 0
connected = False
while not connected and attempt < 3:
    attempt += 1
    connected = attempt == 2          # pretend the second try works
    print(f"attempt {attempt}: {'ok' if connected else 'failed'}")
```

`while True:` with a `break` inside is the standard "loop until something happens" shape. Always make sure something in the loop changes the condition, or you get an infinite loop (Ctrl+C stops it).

## Nested loops and reversing

```python
for row in range(1, 4):
    print(" ".join(f"{row * col:3}" for col in range(1, 4)))

for name in reversed(["a", "b", "c"]):
    print(name, end=" ")
print()
```

@@quiz
What does `list(range(2, 10, 3))` give?
- [2, 5, 8, 11]
* [2, 5, 8]
- [3, 6, 9]
- [2, 3, 4]
? Stop (10) is not included.

@@quiz
When does a `for` loop's `else:` block run?
- Always
- When the list is empty only
* When the loop ends without hitting `break`
- When an error happens
? It's the "not found" branch of a search.

@@quiz
What does `zip([1, 2, 3], ["a", "b"])` produce?
* (1, "a") and (2, "b")
- An error
- Three pairs with None
- ["1a", "2b", "3"]
? zip stops at the shortest input unless `strict=True`.

@@flashcards
`range(a, b, step)` :: numbers from a up to but not including b
`enumerate(items, start=1)` :: pairs of (index, item)
`zip(a, b)` :: pairs items from several iterables, stopping at the shortest
`zip(..., strict=True)` :: raises an error if lengths differ
`break` :: leave the loop now
`continue` :: skip to the next iteration
Loop `else` :: runs when the loop wasn't broken out of
`counts[k] = counts.get(k, 0) + 1` :: the counting-into-a-dict pattern

@@exercise Numbered report
Write `report(names, statuses)` that returns a list of lines like `"1. web01  UP"`: the number (starting at 1), a dot and space, the name padded to 7 characters, then the status. If the lists have different lengths, raise `ValueError` (use `zip(..., strict=True)`).
@@starter
def report(names, statuses):
    pass


for line in report(["web01", "db01"], ["UP", "DOWN"]):
    print(line)
@@expected
1. web01  UP
2. db01   DOWN
@@tests
check(report([], []), [], "empty")
check(report(["a"], ["UP"]), ["1. a      UP"], "padding")
try:
    report(["a", "b"], ["UP"])
    check("no error", "ValueError", "different lengths")
except ValueError:
    pass
@@requires
enumerate(
@@hint
`for i, (name, status) in enumerate(zip(names, statuses, strict=True), start=1):` then `lines.append(f"{i}. {name:<7}{status}")`. Note the pairs from `zip` are unpacked inside parentheses.
@@solution
def report(names, statuses):
    lines = []
    for i, (name, status) in enumerate(zip(names, statuses, strict=True), start=1):
        lines.append(f"{i}. {name:<7}{status}")
    return lines


for line in report(["web01", "db01"], ["UP", "DOWN"]):
    print(line)

@@exercise First match and running totals
Write two functions:

- `first_full(disks, limit)`: `disks` is a list of `(name, percent_used)` tuples. Return the name of the first disk at or above `limit` using a `for` loop with `break` and `else` (return `None` from the `else`).
- `running_totals(values)`: a list where each item is the sum of all values up to and including that position (`[3, 1, 4]` → `[3, 4, 8]`).
@@starter
def first_full(disks, limit):
    pass


def running_totals(values):
    pass


print(first_full([("C:", 70), ("D:", 95), ("E:", 99)], 90))
print(running_totals([3, 1, 4, 1, 5]))
@@tests
check(first_full([("C:", 70), ("D:", 95), ("E:", 99)], 90), "D:", "first one")
check(first_full([("C:", 70)], 90), None, "none full")
check(first_full([], 90), None, "empty")
check(first_full([("C:", 90)], 90), "C:", "equal counts")
check(running_totals([3, 1, 4, 1, 5]), [3, 4, 8, 9, 14], "totals")
check(running_totals([]), [], "empty totals")
@@requires
else:
@@hint
`for name, used in disks: if used >= limit: found = name; break` then `else: found = None` and `return found`. For totals keep `total = 0`, add each value and append `total`.
@@solution
def first_full(disks, limit):
    for name, used in disks:
        if used >= limit:
            found = name
            break
    else:
        found = None
    return found


def running_totals(values):
    totals = []
    total = 0
    for v in values:
        total += v
        totals.append(total)
    return totals


print(first_full([("C:", 70), ("D:", 95), ("E:", 99)], 90))
print(running_totals([3, 1, 4, 1, 5]))
