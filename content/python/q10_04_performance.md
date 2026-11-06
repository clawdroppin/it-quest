@@lesson q10-performance Making Python Fast: Measure, Then Optimise
@@concepts lists, maps, functions
@@text
Python is slower than Rust or C for raw computation, often 10–100× for tight loops. Yet most slow Python programs are slow for reasons that have nothing to do with the language: the wrong data structure, repeated work, or too many round trips to a database. This lesson is about finding and fixing those.

## Rule 1: measure first

Intuition about performance is usually wrong. Measure:

```python
import timeit

setup = "data = list(range(10_000)); s = set(data)"
print("list:", timeit.timeit("9_999 in data", setup=setup, number=1_000))
print("set: ", timeit.timeit("9_999 in s", setup=setup, number=1_000))
```

- `timeit` for small snippets (it runs them many times and avoids common traps). In IPython/Jupyter: `%timeit expr`.
- `time.perf_counter()` around a block for a quick measurement.
- **cProfile** for whole programs: `python -m cProfile -s cumtime app.py` lists where time goes by function. View it graphically with **snakeviz**.
- **py-spy** (`py-spy top --pid 1234`, `py-spy record -o flame.svg -- python app.py`) samples a running program without changing it: great for production.
- **scalene** and **memray** profile CPU *and* memory line by line.

Optimise the function at the top of the profile, re-measure, stop when it's fast enough.

## Rule 2: algorithms and data structures

| Operation | list | set / dict |
|---|---|---|
| `x in container` | O(n): scans everything | O(1) average: hash lookup |
| add/remove at the end | O(1) | O(1) |
| insert/remove at the front | O(n) | n/a (use `deque`: O(1)) |

The classic accidental O(n²):

```python
import time

old = [f"user{i}" for i in range(5000)]
new = [f"user{i}" for i in range(2500, 7500)]

t = time.perf_counter()
slow = [u for u in new if u not in old]            # each `in` scans the whole list
t_slow = time.perf_counter() - t

t = time.perf_counter()
old_set = set(old)
fast = [u for u in new if u not in old_set]        # each `in` is a hash lookup
t_fast = time.perf_counter() - t

print(slow == fast, f"{t_slow / t_fast:.0f}x faster")
```

Other classics: sorting inside a loop, building strings with `+=` in a loop (use `"".join(parts)`), recomputing the same value (cache it), reading a file line by line with `readlines()` into memory when you could stream.

## Rule 3: let C do the loop

Built-ins and the standard library are implemented in C. Pushing the loop into them is often 5–50× faster than a Python `for`:

- `sum`, `min`, `max`, `sorted`, `any`, `all`, `len`, `str.join`, `str.count`, `bytes.translate`
- `collections.Counter` for counting, `heapq.nlargest` for top-N, `bisect` for sorted searches
- comprehensions are faster than `.append` loops
- `itertools` and `operator.itemgetter`
- **NumPy** for numeric arrays: one vectorised expression replaces a loop over millions of numbers

## Rule 4: avoid repeated work

```python
from functools import cache

@cache
def route_cost(a, b):
    # imagine an expensive calculation or lookup here
    return abs(len(a) - len(b))

print(route_cost("tokyo", "osaka"), route_cost.cache_info())
```

- Cache pure functions with `functools.cache` / `lru_cache(maxsize=...)`.
- Hoist work out of loops (compile a regex once with `re.compile`, look up `obj.attr` once).
- Batch I/O: one SQL query with `IN (...)` instead of one per item; `executemany`; HTTP connection reuse with a session.

## Rule 5: when Python itself is the bottleneck

1. **PyPy**: a JIT interpreter; pure-Python loops often run 4–10× faster, no code changes.
2. **Processes**: use all cores (previous lesson).
3. **Cython** or **mypyc**: compile type-annotated Python to C extensions.
4. **Numba**: `@njit` compiles numeric functions to machine code.
5. **Rust with PyO3/maturin**: write the hot function in Rust and import it like a Python module. This is how Polars, Pydantic v2, ruff and uv are so fast.

## Memory performance

- Generators process streams in constant memory (`sum(int(l) for l in open(path))`).
- `__slots__` and `array.array` / NumPy shrink millions of small objects.
- `sys.getsizeof` measures one object; `tracemalloc` finds which lines allocate most.

@@quiz
What should you do before optimising?
* Measure with a profiler or timeit to find the real bottleneck
- Rewrite everything in C
- Remove all functions
- Add threads
? Optimising the wrong part wastes time.

@@quiz
Why is `x in some_set` faster than `x in some_list` for large collections?
- Sets are sorted
* Sets use hashing, so lookup is O(1) on average instead of scanning every item
- Lists are stored on disk
- It isn't
? Converting a list to a set once can turn O(n²) into O(n).

@@quiz
What's the efficient way to build a big string from many parts?
- `s += part` in a loop
* Collect parts in a list and use `"".join(parts)`
- Use `str.format` repeatedly
- `print` to a file
? Repeated `+=` can copy the string each time.

@@quiz
Which tool can sample a running production Python process without modifying it?
- timeit
- cProfile
* py-spy
- pdb
? It reads the process's stacks from outside.

@@drill
Which standard-library module times small code snippets accurately?
= timeit
? python -m timeit "sum(range(1000))".

@@flashcards
timeit :: accurate timing of small snippets
cProfile :: deterministic profiler showing time per function
py-spy :: sampling profiler for running processes, makes flame graphs
O(n²) trap :: `in` on a list inside a loop over another list
`functools.cache` :: memoise a pure function
Vectorisation :: replacing Python loops with whole-array operations (NumPy)
PyPy :: alternative interpreter with a JIT compiler
PyO3 / maturin :: write Python extension modules in Rust
tracemalloc :: find which lines allocate the most memory

@@exercise Fix the slow reconciliation
`reconcile(hr_users, ad_users)` compares the HR system's employee list with Active Directory. It's correct but O(n²) and far too slow for 50,000 users. Rewrite it to run in roughly linear time, returning the same result: a dict with sorted lists `missing_in_ad` (in HR, not in AD), `orphaned` (in AD, not in HR), and `count_both`. Names compare case-insensitively; return them lower-cased.
@@starter
def reconcile(hr_users, ad_users):
    missing = []
    for u in hr_users:
        if u.lower() not in [a.lower() for a in ad_users]:
            missing.append(u.lower())
    orphaned = []
    for a in ad_users:
        if a.lower() not in [u.lower() for u in hr_users]:
            orphaned.append(a.lower())
    both = 0
    for u in hr_users:
        if u.lower() in [a.lower() for a in ad_users]:
            both += 1
    return {"missing_in_ad": sorted(set(missing)), "orphaned": sorted(set(orphaned)), "count_both": both}


print(reconcile(["Maya", "Li", "Sam"], ["maya", "sam", "olduser"]))
@@tests
check(reconcile(["Maya", "Li", "Sam"], ["maya", "sam", "olduser"]), {"missing_in_ad": ["li"], "orphaned": ["olduser"], "count_both": 2}, "small")
import time
hr = [f"User{i}" for i in range(40000)]
ad = [f"user{i}" for i in range(5000, 45000)]
start = time.perf_counter()
r = reconcile(hr, ad)
elapsed = time.perf_counter() - start
check((len(r["missing_in_ad"]), len(r["orphaned"]), r["count_both"]), (5000, 5000, 35000), "large result")
check(elapsed < 0.5, True, "fast enough")
@@hint
Lower-case each list once into sets: `hr = {u.lower() for u in hr_users}`, `ad = {...}`. Then `sorted(hr - ad)`, `sorted(ad - hr)` and `len(hr & ad)`.
@@solution
def reconcile(hr_users, ad_users):
    hr = {u.lower() for u in hr_users}
    ad = {a.lower() for a in ad_users}
    return {"missing_in_ad": sorted(hr - ad), "orphaned": sorted(ad - hr), "count_both": len(hr & ad)}


print(reconcile(["Maya", "Li", "Sam"], ["maya", "sam", "olduser"]))

@@exercise Cache the expensive part
`count_ways(amount, coins)` counts the ways to make `amount` from unlimited coins of the given values (order doesn't matter). The naive recursion is exponential. Make it fast with memoisation (`functools.cache` on an inner function, or a dynamic-programming table). `coins` is a tuple.

Also write `top_n(scores, n)` returning the `n` `(name, score)` pairs with the highest scores, highest first (ties by name), using `heapq.nsmallest` with a suitable key instead of sorting everything.
@@starter
from functools import cache
import heapq


def count_ways(amount, coins):
    def ways(remaining, index):
        if remaining == 0:
            return 1
        if remaining < 0 or index == len(coins):
            return 0
        return ways(remaining - coins[index], index) + ways(remaining, index + 1)
    return ways(amount, 0)


def top_n(scores, n):
    pass


print(count_ways(10, (1, 2, 5)))
@@tests
check(count_ways(10, (1, 2, 5)), 10, "small")
check(count_ways(0, (1, 2)), 1, "zero amount")
import time
start = time.perf_counter()
check(count_ways(1000, (1, 2, 5, 10, 20, 50, 100, 200)), 321335886, "large")
check(time.perf_counter() - start < 1.0, True, "fast")
check(top_n({"a": 5, "b": 9, "c": 9, "d": 1}, 3), [("b", 9), ("c", 9), ("a", 5)], "top n")
check(top_n({}, 3), [], "empty")
@@requires
heapq
@@hint
Decorate the inner `ways` with `@cache` (it's created per call, so caches don't leak between different coin sets). Python's recursion limit may be hit for large amounts with coin 1: add `import sys; sys.setrecursionlimit(10_000)` or use a DP table: `table = [1] + [0] * amount; for c in coins: for a in range(c, amount + 1): table[a] += table[a - c]`. For top_n: `heapq.nsmallest(n, scores.items(), key=lambda kv: (-kv[1], kv[0]))`.
@@solution
from functools import cache
import heapq


def count_ways(amount, coins):
    table = [1] + [0] * amount
    for c in coins:
        for a in range(c, amount + 1):
            table[a] += table[a - c]
    return table[amount]


def top_n(scores, n):
    return heapq.nsmallest(n, scores.items(), key=lambda kv: (-kv[1], kv[0]))


print(count_ways(10, (1, 2, 5)))
