@@lesson n2-collections-sorting Sorting and the collections Module
@@concepts lists, maps
@@text
Lists, dicts, tuples and sets cover most needs. For the rest, Python's `collections` module has specialised containers, and sorting has a few tricks worth knowing well.

## Sorting with a key

`sorted(items)` returns a new sorted list; `items.sort()` sorts in place and returns `None`. Both take `key=` (a function that computes what to compare) and `reverse=True`:

```python
servers = [("web01", 35), ("db01", 92), ("mail01", 51)]
print(sorted(servers, key=lambda s: s[1], reverse=True))   # by load, highest first
print(sorted(["web10", "Web9", "db1"], key=str.lower))       # case-insensitive
```

**Sort by several things** by returning a tuple. To sort one part descending and another ascending, negate the number:

```python
users = [("li", "ops", 120), ("maya", "dev", 50), ("sam", "ops", 80)]
print(sorted(users, key=lambda u: (u[1], -u[2])))   # by team, then biggest quota first
```

Python's sort is **stable**: items that compare equal keep their original order, so you can also sort in several passes (least important key first).

`operator.itemgetter(1)` and `operator.attrgetter("load")` are fast, readable alternatives to lambdas. `min`/`max` take `key=` too: `max(servers, key=lambda s: s[1])`.

## Natural sort

Plain sorting puts `"web10"` before `"web9"` (character by character). A **natural sort** key splits out the numbers:

```python
import re

def natural_key(text):
    return [int(part) if part.isdigit() else part.lower() for part in re.split(r"(\d+)", text)]

print(sorted(["web10", "web9", "web1"], key=natural_key))
```

## Counter: counting things

```python
from collections import Counter

levels = ["INFO", "ERROR", "INFO", "WARN", "INFO", "ERROR"]
counts = Counter(levels)
print(counts)
print(counts["INFO"], counts["DEBUG"])     # missing keys count as 0
print(counts.most_common(2))
print(Counter("mississippi").most_common(1))
```

Counters can be added and subtracted, and `.update(more_items)` adds counts.

## defaultdict: grouping without checks

```python
from collections import defaultdict

logins = [("maya", "10.0.0.5"), ("li", "10.0.0.9"), ("maya", "10.0.0.7")]
ips_by_user = defaultdict(list)
for user, ip in logins:
    ips_by_user[user].append(ip)       # no "if user not in dict" needed
print(dict(ips_by_user))
```

`defaultdict(int)` counts, `defaultdict(set)` collects unique values.

## deque: a fast queue

A list is slow to remove from the **front** (every item shifts). `deque` is fast at both ends, and with `maxlen` it keeps only the last N items, perfect for "the last 100 log lines":

```python
from collections import deque

recent = deque(maxlen=3)
for line in ["a", "b", "c", "d", "e"]:
    recent.append(line)
print(list(recent))

queue = deque(["job1", "job2"])
queue.append("job3")
print(queue.popleft(), list(queue))
```

## namedtuple and others

```python
from collections import namedtuple

Server = namedtuple("Server", ["name", "ip", "load"])
s = Server("web01", "10.0.0.5", 35)
print(s.name, s[2], s)
```

`namedtuple` gives tuples field names (dataclasses, covered later, are the more flexible modern choice). `OrderedDict` is mostly historical: normal dicts keep insertion order since Python 3.7. `ChainMap` layers several dicts (user settings over defaults).

## heapq and bisect

- `heapq.nsmallest(3, items, key=...)` / `nlargest` find the top N without sorting everything. `heapq.heappush`/`heappop` make a **priority queue**.
- `bisect.bisect` finds where a value belongs in a sorted list, e.g. mapping a score to a grade band.

```python
import heapq, bisect

loads = {"web01": 35, "db01": 92, "mail01": 51, "app01": 77}
print(heapq.nlargest(2, loads, key=loads.get))

limits = [50, 80, 95]
bands = ["ok", "busy", "high", "critical"]
print([bands[bisect.bisect(limits, x)] for x in (10, 50, 85, 99)])
```

@@quiz
What does `items.sort()` return?
- The sorted list
* None; it sorts the list in place
- A new list
- True
? Use `sorted(items)` when you want a new list.

@@quiz
How do you sort by team ascending, then quota descending?
* `key=lambda u: (u.team, -u.quota)`
- `key=lambda u: (u.team, u.quota), reverse=True`
- Two separate lists
- It's impossible
? Negating a number reverses its order inside the tuple.

@@quiz
Which container keeps only the last N items automatically?
- Counter
- defaultdict
* deque(maxlen=N)
- namedtuple
? Old items fall off the other end.

@@quiz
What does `Counter(levels)["DEBUG"]` return if DEBUG never appeared?
- KeyError
- None
* 0
- -1
? Counters treat missing keys as zero.

@@flashcards
`sorted(items, key=f)` :: new list sorted by f(item)
Stable sort :: equal items keep their original order
`Counter` :: dict subclass that counts hashable items
`most_common(n)` :: the n most frequent items with counts
`defaultdict(list)` :: dict that creates an empty list for missing keys
`deque` :: double-ended queue, fast at both ends; `maxlen` keeps the last N
`heapq.nlargest(n, items)` :: top n items without a full sort
`bisect.bisect(sorted_list, x)` :: index where x would be inserted

@@exercise Top talkers
`connections` is a list of `(source_ip, bytes)` tuples from a firewall log. Write:

- `bytes_by_ip(connections)`: a dict of total bytes per IP (use `defaultdict(int)` or `Counter`)
- `top_talkers(connections, n)`: the `n` IPs with the most total bytes as `(ip, total)` tuples, biggest first; ties sorted by IP ascending
- `chatty(connections, min_count)`: a sorted list of IPs that appear at least `min_count` times
@@starter
from collections import Counter, defaultdict


def bytes_by_ip(connections):
    pass


def top_talkers(connections, n):
    pass


def chatty(connections, min_count):
    pass


LOG = [("10.0.0.5", 500), ("10.0.0.9", 1200), ("10.0.0.5", 900), ("10.0.0.7", 1400), ("10.0.0.9", 200)]
print(top_talkers(LOG, 2))
@@tests
LOG = [("10.0.0.5", 500), ("10.0.0.9", 1200), ("10.0.0.5", 900), ("10.0.0.7", 1400), ("10.0.0.9", 200)]
check(bytes_by_ip(LOG), {"10.0.0.5": 1400, "10.0.0.9": 1400, "10.0.0.7": 1400}, "totals")
check(top_talkers(LOG, 2), [("10.0.0.5", 1400), ("10.0.0.7", 1400)], "ties by ip")
check(top_talkers([("a", 1), ("b", 5)], 5), [("b", 5), ("a", 1)], "fewer than n")
check(chatty(LOG, 2), ["10.0.0.5", "10.0.0.9"], "chatty")
@@hint
`totals = defaultdict(int)`; add bytes; return `dict(totals)`. Sort `totals.items()` with `key=lambda kv: (-kv[1], kv[0])` and slice `[:n]`. For chatty: `counts = Counter(ip for ip, _ in connections)` then filter and sort.
@@solution
from collections import Counter, defaultdict


def bytes_by_ip(connections):
    totals = defaultdict(int)
    for ip, size in connections:
        totals[ip] += size
    return dict(totals)


def top_talkers(connections, n):
    totals = bytes_by_ip(connections)
    return sorted(totals.items(), key=lambda kv: (-kv[1], kv[0]))[:n]


def chatty(connections, min_count):
    counts = Counter(ip for ip, _ in connections)
    return sorted(ip for ip, c in counts.items() if c >= min_count)


LOG = [("10.0.0.5", 500), ("10.0.0.9", 1200), ("10.0.0.5", 900), ("10.0.0.7", 1400), ("10.0.0.9", 200)]
print(top_talkers(LOG, 2))

@@exercise A tail -f buffer and natural sort
- `last_lines(lines, n)`: the last `n` lines as a list, using a `deque(maxlen=n)` (don't slice).
- `natural_sorted(names)`: sort host names so numbers compare as numbers: `["web10", "web9", "Web1"]` → `["Web1", "web9", "web10"]` (case-insensitive for text parts).
@@starter
from collections import deque
import re


def last_lines(lines, n):
    pass


def natural_sorted(names):
    pass


print(natural_sorted(["web10", "web9", "Web1", "db2"]))
@@tests
check(last_lines(["a", "b", "c", "d"], 2), ["c", "d"], "last 2")
check(last_lines(["a"], 5), ["a"], "fewer lines")
check(natural_sorted(["web10", "web9", "Web1", "db2"]), ["db2", "Web1", "web9", "web10"], "natural")
check(natural_sorted(["a2b10", "a2b9", "a10b1"]), ["a2b9", "a2b10", "a10b1"], "several numbers")
@@requires
deque(
@@hint
`buf = deque(lines, maxlen=n)` then `list(buf)`. For the key: `[int(p) if p.isdigit() else p.lower() for p in re.split(r"(\d+)", name)]`. The capturing group in `re.split` keeps the numbers in the result.
@@solution
from collections import deque
import re


def last_lines(lines, n):
    buf = deque(maxlen=n)
    for line in lines:
        buf.append(line)
    return list(buf)


def natural_key(name):
    return [int(p) if p.isdigit() else p.lower() for p in re.split(r"(\d+)", name)]


def natural_sorted(names):
    return sorted(names, key=natural_key)


print(natural_sorted(["web10", "web9", "Web1", "db2"]))
