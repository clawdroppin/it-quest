@@lesson e7-generators Iterators and Generators
@@concepts loops
@@text
A 20 GB log file doesn't fit in memory. **Generators** let you process data one item at a time, as a stream, which is how professional tools handle huge inputs.

## Iterables and iterators

Anything you can loop over is **iterable**: lists, strings, files, dicts. Under the hood, `for` asks it for an **iterator** with `iter()` and calls `next()` until it runs out:

```python
it = iter(["web01", "web02"])
print(next(it))
print(next(it))
try:
    next(it)
except StopIteration:
    print("done")
```

## Generator functions: yield

A function with `yield` becomes a **generator**: calling it doesn't run it yet. Each `next` runs it until the next `yield`, which hands out one value and **pauses** right there.

```python
def countdown(n):
    print("starting")
    while n > 0:
        yield n
        n -= 1
    print("finished")

for x in countdown(3):
    print(x)
```

Generators produce values **lazily**: only when asked. That's how you chain processing steps without ever holding the whole data in memory:

```python
def read_lines(lines):
    for line in lines:
        yield line.rstrip("\n")

def only_errors(lines):
    for line in lines:
        if "ERROR" in line:
            yield line

def fields(lines):
    for line in lines:
        yield line.split(maxsplit=2)

raw = ["08:00 INFO ok\n", "08:01 ERROR disk full\n", "08:02 ERROR fan\n"]
for time, level, msg in fields(only_errors(read_lines(raw))):
    print(time, msg)
```

Replace `raw` with an open file and this pipeline handles a file of any size with almost no memory.

## Generator expressions

Like list comprehensions, with round brackets, and lazy:

```python
sizes = (len(word) for word in ["ssh", "https", "dns"])
print(sum(sizes))
print(sum(x * x for x in range(1_000_000)))   # never builds a million-item list
```

## itertools

The standard library's toolbox for streams:

```python
import itertools

print(list(itertools.islice(itertools.count(1), 5)))        # first 5 of an endless stream
print(list(itertools.chain([1, 2], [3])))                    # glue iterables together
for key, group in itertools.groupby(["a1", "a2", "b1"], key=lambda s: s[0]):
    print(key, list(group))
print(list(itertools.batched(range(7), 3)))                  # chunks of 3
```

`lambda s: s[0]` is a tiny unnamed function: "given s, return s[0]".

@@quiz
What does calling a generator function, like `countdown(3)`, do?
- Runs the whole function and returns a list
* Returns a generator object; the body runs only as values are requested
- Prints "starting" immediately
- Raises StopIteration
? Nothing runs until the first `next()` (which a for loop does for you).

@@exercise Batches for an API
APIs often accept at most N items per call. Write a generator `batches(items, size)` that yields lists of up to `size` items, in order, without using `itertools.batched`.
@@starter
def batches(items, size):
    pass


for b in batches(["u1", "u2", "u3", "u4", "u5"], 2):
    print(b)
@@tests
check(list(batches(["u1", "u2", "u3", "u4", "u5"], 2)), [["u1", "u2"], ["u3", "u4"], ["u5"]], "batches(5 items, 2)")
check(list(batches([], 3)), [], "batches([], 3)")
check(list(batches(range(3), 3)), [[0, 1, 2]], "an exact batch")
import types as _t
check(isinstance(batches([1], 1), _t.GeneratorType), True, "batches is a generator (uses yield)")
@@hint
Keep a `current` list; append each item; when it reaches `size`, `yield current` and start a new list. After the loop, yield what's left if it isn't empty.
@@solution
def batches(items, size):
    current = []
    for item in items:
        current.append(item)
        if len(current) == size:
            yield current
            current = []
    if current:
        yield current


for b in batches(["u1", "u2", "u3", "u4", "u5"], 2):
    print(b)

@@exercise Follow the log
Write a generator `alerts(lines, threshold)`: lines look like `"cpu=93"`. Yield `"ALERT cpu=N"` each time the value goes **above** `threshold` after having been at or below it (not on every high line, only when it **crosses** the threshold upward). Lines that don't start with `cpu=` are ignored.
@@starter
def alerts(lines, threshold):
    pass


print(list(alerts(["cpu=40", "cpu=95", "cpu=97", "noise", "cpu=50", "cpu=91"], 90)))
@@tests
check(list(alerts(["cpu=40", "cpu=95", "cpu=97", "noise", "cpu=50", "cpu=91"], 90)), ["ALERT cpu=95", "ALERT cpu=91"], "two crossings")
check(list(alerts(["cpu=99"], 90)), ["ALERT cpu=99"], "high from the start")
check(list(alerts(["cpu=10", "cpu=20"], 90)), [], "never high")
@@hint
Keep a boolean `high = False`. For each value: if it's above the threshold and not `high`, yield and set `high = True`; if it's at or below, set `high = False`.
@@solution
def alerts(lines, threshold):
    high = False
    for line in lines:
        if not line.startswith("cpu="):
            continue
        value = int(line[4:])
        if value > threshold:
            if not high:
                yield f"ALERT cpu={value}"
            high = True
        else:
            high = False


print(list(alerts(["cpu=40", "cpu=95", "cpu=97", "noise", "cpu=50", "cpu=91"], 90)))
@@text
> Alerting only on the *crossing* (not on every sample) avoids **alert fatigue**: the on-call engineer gets one page, not a hundred. Monitoring systems like Prometheus and Zabbix work this way.
