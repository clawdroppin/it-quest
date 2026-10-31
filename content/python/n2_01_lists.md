@@lesson n2-lists Lists
@@concepts lists
@@text
A **list** holds many values in order. It's the workhorse of Python scripts: a list of servers, of users, of log lines.

```python
servers = ["web01", "web02", "db01"]
print(servers)
print(len(servers))
print(servers[0], servers[-1])
```

Indexing and slicing work exactly like with strings: `servers[0]` is the first, `servers[-1]` the last, `servers[1:]` everything from the second on.

## Changing a list

Unlike strings, lists **can be changed** in place:

```python
servers = ["web01", "web02"]
servers.append("db01")        # add to the end
servers.insert(0, "lb01")     # add at a position
servers[1] = "web01-new"      # replace
servers.remove("web02")       # remove by value
last = servers.pop()          # remove and return the last
print(servers, last)
```

## Looping over a list

```python
for host in ["web01", "web02", "db01"]:
    print(f"pinging {host}...")
```

Need the position too? `enumerate` gives you (index, item) pairs:

```python
for i, host in enumerate(["web01", "web02", "db01"], start=1):
    print(i, host)
```

## Building a list in a loop

```python
disks = [120, 980, 40, 512, 2000]
big = []
for size in disks:
    if size >= 500:
        big.append(size)
print(big)
```

## Useful tools

```python
nums = [42, 7, 19, 7, 88]
print(sum(nums), min(nums), max(nums))
print(sorted(nums))                 # a new sorted list
print(sorted(nums, reverse=True))
print(nums.count(7), 19 in nums)
print(", ".join(["a", "b", "c"]))   # list of strings -> one string
print("a,b,,c".split(","))          # one string -> list
```

`sorted(x)` returns a new list; `x.sort()` sorts the list itself and returns `None`.

## List comprehensions

A compact way to build a list from another, very common in real code:

```python
sizes_mb = [512, 2048, 128]
sizes_gb = [s / 1024 for s in sizes_mb]
print(sizes_gb)
hosts = ["web01", "db01", "web02"]
web = [h for h in hosts if h.startswith("web")]
print(web)
```

Read it as: "`s / 1024` **for** each `s` **in** `sizes_mb`" and "`h` for each `h` in hosts **if** it starts with web".

@@quiz
`a = [3, 1, 2]`, then `b = a.sort()`. What is `b`?
- `[1, 2, 3]`
- `[3, 1, 2]`
* `None`
- An error
? `.sort()` sorts `a` in place and returns `None`. Use `b = sorted(a)` for a new sorted list.

@@flashcards
`lst.append(x)` :: add x to the end of the list
`lst.pop()` :: remove and return the last item
`enumerate(lst)` :: gives (index, item) pairs in a for loop
`", ".join(lst)` :: join a list of strings into one string
`[x*2 for x in lst]` :: list comprehension: a new list built from another
`sorted(lst)` vs `lst.sort()` :: new sorted list vs sort in place (returns None)

@@exercise Average response time
Write `average_ms(times)` that returns the average of a list of response times, **rounded to 1 decimal**. For an empty list, return `0.0`.
@@starter
def average_ms(times):
    pass

print(average_ms([120, 80, 95, 210]))
@@tests
check(average_ms([120, 80, 95, 210]), 126.2, "average_ms([120, 80, 95, 210])")
check(average_ms([50]), 50.0, "average_ms([50])")
check(average_ms([]), 0.0, "average_ms([])")
@@hint
`sum(times) / len(times)`, but check `if not times:` first: dividing by zero is an error.
@@solution
def average_ms(times):
    if not times:
        return 0.0
    return round(sum(times) / len(times), 1)

print(average_ms([120, 80, 95, 210]))

@@exercise Slow servers
Write `slow_hosts(hosts, times, limit)`: `hosts` and `times` are two lists of the same length (the response time of each host). Return a list of the hosts whose time is **above** `limit`, in the original order.
@@starter
def slow_hosts(hosts, times, limit):
    pass

print(slow_hosts(["web01", "web02", "db01"], [120, 640, 900], 500))
@@tests
check(slow_hosts(["web01", "web02", "db01"], [120, 640, 900], 500), ["web02", "db01"], "slow_hosts(..., 500)")
check(slow_hosts(["a", "b"], [1, 2], 10), [], "nothing slow")
check(slow_hosts([], [], 10), [], "empty lists")
@@hint
Loop with `for i in range(len(hosts)):` and compare `times[i]`. (Or look up `zip(hosts, times)`, which pairs two lists up.)
@@solution
def slow_hosts(hosts, times, limit):
    result = []
    for host, t in zip(hosts, times):
        if t > limit:
            result.append(host)
    return result

print(slow_hosts(["web01", "web02", "db01"], [120, 640, 900], 500))

@@exercise Clean a user list
An export gave you messy usernames. Write `clean_users(names)` that returns them stripped of spaces, lowercase, with empty entries removed, **without duplicates**, **sorted** alphabetically.
@@starter
def clean_users(names):
    pass

print(clean_users([" Ana", "ben ", "", "ANA", "carl", "  "]))
@@tests
check(clean_users([" Ana", "ben ", "", "ANA", "carl", "  "]), ["ana", "ben", "carl"], "clean_users(messy list)")
check(clean_users([]), [], "clean_users([])")
@@hint
Build a new list: for each name, `n = name.strip().lower()`; skip it if it's empty or already in the result. Sort at the end.
@@solution
def clean_users(names):
    result = []
    for name in names:
        n = name.strip().lower()
        if n and n not in result:
            result.append(n)
    return sorted(result)

print(clean_users([" Ana", "ben ", "", "ANA", "carl", "  "]))
