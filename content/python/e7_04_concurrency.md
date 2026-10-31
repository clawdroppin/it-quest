@@lesson e7-concurrency Doing Many Things at Once
@@concepts concurrency
@@text
Checking 200 servers one after another, at 1 second each, takes over 3 minutes. Most of that time the script is just **waiting** for the network. Concurrency lets it wait for many at once.

## Threads with a pool

`concurrent.futures.ThreadPoolExecutor` runs a function on many inputs using a pool of threads:

```python
import time
from concurrent.futures import ThreadPoolExecutor

def check(host):
    time.sleep(0.2)                  # pretend this is a network call
    return host, "up"

hosts = [f"srv{i:02}" for i in range(10)]

start = time.perf_counter()
results = [check(h) for h in hosts]
print(f"one by one: {time.perf_counter() - start:.1f}s")

start = time.perf_counter()
with ThreadPoolExecutor(max_workers=10) as pool:
    results = list(pool.map(check, hosts))
print(f"10 threads: {time.perf_counter() - start:.1f}s")
print(results[:2])
```

`pool.map` keeps the input order. When you want results as soon as each finishes, use `submit` and `as_completed`:

```python
import random, time
from concurrent.futures import ThreadPoolExecutor, as_completed

def ping(host):
    time.sleep(random.random() / 5)
    return host

with ThreadPoolExecutor(max_workers=5) as pool:
    futures = [pool.submit(ping, h) for h in ["a", "b", "c"]]
    for f in as_completed(futures):
        print("finished:", f.result())
```

## Threads vs processes

- **Threads** are ideal for **I/O-bound** work: waiting on networks, disks, APIs. (Python's *GIL* means plain Python code doesn't run on two CPU cores at once, but waiting doesn't need a core.)
- **Processes** (`ProcessPoolExecutor`) are for **CPU-bound** work, like hashing thousands of files: each process has its own interpreter and core.

## Shared data needs a lock

If threads update the same variable, use a `threading.Lock` so updates don't collide:

```python
import threading
from concurrent.futures import ThreadPoolExecutor

total = 0
lock = threading.Lock()

def add_bytes(n):
    global total
    with lock:
        total += n

with ThreadPoolExecutor(8) as pool:
    pool.map(add_bytes, range(1000))
print(total)
```

Better still: have each task **return** its result and combine them at the end, so nothing is shared.

## asyncio

`asyncio` handles thousands of concurrent network operations in a single thread, using `async def` and `await`. Modern libraries (`httpx`, `aiohttp`, `asyncssh`) are built on it:

```python
import asyncio

async def check(host, delay):
    await asyncio.sleep(delay)       # while waiting, other checks run
    return f"{host} ok"

async def main():
    results = await asyncio.gather(check("web01", 0.2), check("db01", 0.1), check("dns01", 0.15))
    print(results)

asyncio.run(main())
```

Rule of thumb: start with `ThreadPoolExecutor`. It's simple and fast enough for nearly all IT automation.

@@quiz
Which job benefits most from a thread pool?
* Checking whether 500 websites respond
- Calculating a big prime number
- Reading one small config file
- Sorting a list of 10 items
? Website checks are I/O-bound: almost all the time is spent waiting on the network, so many can wait at once.

@@flashcards
I/O-bound :: mostly waiting (network, disk): use threads or asyncio
CPU-bound :: mostly computing: use processes (ProcessPoolExecutor)
`pool.map(f, items)` :: run f on every item, results in input order
`as_completed(futures)` :: results in the order they finish
`threading.Lock()` :: protect data that several threads change

@@exercise Parallel checks
Write `check_all(hosts, checker, workers)` that runs `checker(host)` for every host using a `ThreadPoolExecutor` with `workers` threads, and returns a dict from host to the checker's result. The check gives you a slow fake checker: doing it one by one would be too slow.
@@starter
from concurrent.futures import ThreadPoolExecutor


def check_all(hosts, checker, workers):
    pass
@@tests
import time as _time
def _slow(h):
    _time.sleep(0.3)
    return h.upper()
_t0 = _time.perf_counter()
_r = check_all([f"h{i}" for i in range(10)], _slow, 10)
_dt = _time.perf_counter() - _t0
check(_r, {f"h{i}": f"H{i}" for i in range(10)}, "results for 10 hosts")
check(_dt < 1.5, True, "10 checks of 0.3s finish in well under 3s (they run in parallel)")
@@hint
`with ThreadPoolExecutor(max_workers=workers) as pool: results = pool.map(checker, hosts)`, then `dict(zip(hosts, results))`.
@@solution
from concurrent.futures import ThreadPoolExecutor


def check_all(hosts, checker, workers):
    with ThreadPoolExecutor(max_workers=workers) as pool:
        return dict(zip(hosts, pool.map(checker, hosts)))
