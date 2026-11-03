@@lesson p9-functional Functional Tools: lambda, itertools, functools and operator
@@concepts functions, lists
@@text
Python isn't a purely functional language, but functions are values and the standard library has excellent tools for working with them. Rust's iterator adapters (`map`, `filter`, `take_while`, `chunks`, `fold`) all have Python equivalents here.

## lambda, map and filter

A `lambda` is a small anonymous function: one expression, no statements.

```python
by_load = sorted([("web", 35), ("db", 92)], key=lambda s: s[1])
print(by_load)

print(list(map(str.upper, ["web", "db"])))
print(list(filter(lambda p: p < 1024, [22, 8080, 443])))
```

`map`/`filter` return **lazy iterators**. Comprehensions are usually clearer than `map`+`lambda`; `map` shines with an existing function (`map(int, parts)`).

## Closures

A function defined inside another **remembers** the variables of the outer one:

```python
def make_threshold(limit):
    def is_over(value):
        return value > limit          # limit is captured
    return is_over

over_90 = make_threshold(90)
print(over_90(95), over_90(50))
```

To **change** a captured variable, declare it `nonlocal`:

```python
def make_counter():
    count = 0
    def next_id():
        nonlocal count
        count += 1
        return count
    return next_id

nid = make_counter()
print(nid(), nid(), nid())
```

Watch out for **late binding** in loops: `[lambda: i for i in range(3)]` all return 2, because they look up `i` when called. Bind it with a default: `lambda i=i: i`.

## functools

```python
import functools, operator

# partial: pre-fill some arguments
ping = functools.partial(print, "PING", sep=" -> ")
ping("web01")

# reduce: fold a sequence into one value
print(functools.reduce(operator.mul, [1, 2, 3, 4], 1))

# cache / lru_cache: remember results of pure functions
@functools.cache
def fib(n):
    return n if n < 2 else fib(n - 1) + fib(n - 2)
print(fib(80))

# singledispatch: choose an implementation by argument type
@functools.singledispatch
def to_text(value):
    return str(value)

@to_text.register
def _(value: list):
    return ", ".join(map(to_text, value))

@to_text.register
def _(value: bool):
    return "yes" if value else "no"

print(to_text([1, True, "x"]))
```

`cmp_to_key` converts old-style comparison functions for `sorted`; `wraps` is used in decorators; `cached_property` computes an attribute once.

## operator

`operator` has the built-in operators as functions: `add`, `mul`, `itemgetter`, `attrgetter`, `methodcaller`. They're faster and often clearer than lambdas:

```python
from operator import itemgetter
rows = [{"host": "db", "load": 92}, {"host": "web", "load": 35}]
print(sorted(rows, key=itemgetter("load")))
print(list(map(itemgetter("host"), rows)))
```

## itertools: the iterator toolbox

```python
import itertools as it

print(list(it.islice(it.count(100), 3)))                # 100, 101, 102 (count is infinite)
print(list(it.chain(["a", "b"], ["c"])))                # concatenate iterables
print(list(it.batched(range(7), 3)))                     # chunks of 3 (3.12+)
print(list(it.pairwise([10, 12, 19])))                  # neighbours: (10,12), (12,19)
print(list(it.accumulate([3, 1, 4])))                   # running totals
print(list(it.takewhile(lambda x: x < 5, [1, 3, 7, 2]))) # stop at first failure
print(list(it.product("ab", [1, 2])))                    # every combination
print(list(it.combinations(["web", "db", "cache"], 2)))  # unordered pairs
print(list(it.zip_longest([1, 2, 3], ["x"], fillvalue="-")))
```

### groupby

`groupby` groups **consecutive** items with the same key, so sort first:

```python
from itertools import groupby
events = [("web", "ok"), ("db", "fail"), ("web", "fail"), ("db", "ok")]
events.sort(key=lambda e: e[0])
for host, group in groupby(events, key=lambda e: e[0]):
    print(host, [status for _, status in group])
```

## Pure functions

A **pure** function's result depends only on its arguments and it changes nothing outside itself. Pure functions are trivially testable, cacheable and safe to run in parallel. A good architecture keeps a pure core (parsing, rules, calculations) and pushes side effects (files, network, printing) to the edges.

@@quiz
What does a closure remember?
- Nothing
* Variables from the enclosing function where it was defined
- Global variables only
- Its own return value
? `make_threshold(90)` returns a function that remembers `limit`.

@@quiz
Why must you sort before `itertools.groupby`?
* It only groups consecutive items with equal keys
- groupby requires sorted input or it raises
- Sorting makes it faster
- It doesn't matter
? Unsorted input creates several groups for the same key.

@@quiz
What does `functools.partial(f, 1)` return?
- f(1)
* A new function like f with the first argument already set to 1
- A list
- A decorator
? Calling it with `x` calls `f(1, x)`.

@@quiz
Which keyword lets an inner function rebind a variable of the enclosing function?
- global
* nonlocal
- outer
- static
? `global` refers to module-level names.

@@flashcards
lambda :: anonymous one-expression function
Closure :: inner function capturing variables from its enclosing scope
`nonlocal` :: rebind a variable from the enclosing function
`functools.partial` :: pre-fill some arguments of a function
`functools.reduce` :: fold a sequence into one value
`functools.cache` :: memoize a function's results
`functools.singledispatch` :: pick an implementation by argument type
`itertools.groupby` :: group consecutive items with the same key
`itertools.batched` :: split an iterable into chunks of n
Pure function :: result depends only on arguments, no side effects

@@exercise Rate limiter closure
Write `make_limiter(max_calls)` returning a function `allow(user)` that returns True the first `max_calls` times it's called for each user and False afterwards. Different users have separate counts; different limiters are independent. Use a closure (a dict in the enclosing function), not globals.

Also write `make_multipliers()` returning a list of 3 functions where the i-th multiplies its argument by i (0, 1, 2), avoiding the late-binding trap.
@@starter
def make_limiter(max_calls):
    pass


def make_multipliers():
    pass


allow = make_limiter(2)
print([allow("maya") for _ in range(3)], allow("li"))
print([f(10) for f in make_multipliers()])
@@tests
a = make_limiter(2)
b = make_limiter(1)
check([a("u") for _ in range(3)], [True, True, False], "limit per user")
check(a("v"), True, "separate users")
check(b("u"), True, "independent limiters")
check(b("u"), False, "limit 1")
check([f(10) for f in make_multipliers()], [0, 10, 20], "no late binding")
@@hint
Inside `make_limiter`: `counts = {}` then `def allow(user): counts[user] = counts.get(user, 0) + 1; return counts[user] <= max_calls`. Mutating the dict doesn't need `nonlocal`. Multipliers: `[lambda x, i=i: x * i for i in range(3)]`.
@@solution
def make_limiter(max_calls):
    counts = {}

    def allow(user):
        counts[user] = counts.get(user, 0) + 1
        return counts[user] <= max_calls

    return allow


def make_multipliers():
    return [lambda x, i=i: x * i for i in range(3)]


allow = make_limiter(2)
print([allow("maya") for _ in range(3)], allow("li"))
print([f(10) for f in make_multipliers()])

@@exercise Log analysis with itertools
Given log entries as `(minute, host, level)` tuples, sorted by minute:

- `errors_per_host(entries)`: dict of host → number of `"ERROR"` entries, using `sorted` + `itertools.groupby` (hosts with no errors are left out).
- `bursts(entries)`: list of `(minute_a, minute_b)` for consecutive entries (by position) that are both `"ERROR"` and at most 1 minute apart, using `itertools.pairwise`.
- `batches(entries, n)`: the hosts in groups of `n` as lists (use `itertools.batched`), for sending in chunks.
@@starter
import itertools


def errors_per_host(entries):
    pass


def bursts(entries):
    pass


def batches(entries, n):
    pass


LOG = [(1, "web", "INFO"), (2, "db", "ERROR"), (3, "db", "ERROR"), (7, "web", "ERROR"), (9, "web", "ERROR")]
print(errors_per_host(LOG), bursts(LOG), batches(LOG, 2))
@@tests
LOG = [(1, "web", "INFO"), (2, "db", "ERROR"), (3, "db", "ERROR"), (7, "web", "ERROR"), (9, "web", "ERROR")]
check(errors_per_host(LOG), {"db": 2, "web": 2}, "errors per host")
check(errors_per_host([(1, "a", "INFO")]), {}, "no errors")
check(bursts(LOG), [(2, 3)], "bursts")
check(batches(LOG, 2), [["web", "db"], ["db", "web"], ["web"]], "batches")
@@requires
groupby
@@hint
`errs = sorted(h for _, h, lvl in entries if lvl == "ERROR")` then `{h: len(list(g)) for h, g in itertools.groupby(errs)}`. Bursts: `for a, b in itertools.pairwise(entries): if a[2] == b[2] == "ERROR" and b[0] - a[0] <= 1`. Batches: `[list(b) for b in itertools.batched((h for _, h, _ in entries), n)]`.
@@solution
import itertools


def errors_per_host(entries):
    errs = sorted(h for _, h, level in entries if level == "ERROR")
    return {h: len(list(g)) for h, g in itertools.groupby(errs)}


def bursts(entries):
    return [
        (a[0], b[0])
        for a, b in itertools.pairwise(entries)
        if a[2] == b[2] == "ERROR" and b[0] - a[0] <= 1
    ]


def batches(entries, n):
    return [list(b) for b in itertools.batched((h for _, h, _ in entries), n)]


LOG = [(1, "web", "INFO"), (2, "db", "ERROR"), (3, "db", "ERROR"), (7, "web", "ERROR"), (9, "web", "ERROR")]
print(errors_per_host(LOG), bursts(LOG), batches(LOG, 2))
