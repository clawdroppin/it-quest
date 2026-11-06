@@lesson q10-parallelism Threads, Processes and the GIL
@@concepts concurrency, processes
@@text
asyncio handles thousands of waiting network calls in one thread. For other workloads Python offers **threads** and **processes**. Which one helps depends on one famous detail of CPython: the **Global Interpreter Lock**.

## The GIL

In standard CPython, only **one thread executes Python bytecode at a time**. The GIL makes the interpreter simpler and single-threaded code fast, but:

| Workload | Threads help? | Why |
|---|---|---|
| **I/O-bound** (network, disk, database, subprocesses) | **Yes** | the GIL is released while waiting for I/O |
| **CPU-bound pure Python** (parsing, hashing in Python, loops) | **No** | threads take turns; often slower than one thread |
| **CPU-bound in C libraries** (NumPy, hashlib on big data, compression) | Often yes | many C extensions release the GIL |

For CPU-bound pure Python, use **processes**: each has its own interpreter and its own GIL.

### Free-threaded Python

Python 3.13 introduced an optional **free-threaded build** (`python3.13t`, PEP 703) without the GIL, and 3.14 made it officially supported. Threads can then run Python code truly in parallel. Some C extensions still need updating and single-threaded code is a little slower, so the default build keeps the GIL for now. Check with `sys._is_gil_enabled()`.

## concurrent.futures: one API for both

```python
from concurrent.futures import ThreadPoolExecutor, as_completed
import time

def ping(host):
    time.sleep(0.05)                  # stands in for network I/O
    return host, "up"

hosts = [f"10.0.0.{i}" for i in range(1, 9)]
start = time.perf_counter()
with ThreadPoolExecutor(max_workers=8) as pool:
    results = dict(pool.map(ping, hosts))           # map keeps input order
print(len(results), f"{time.perf_counter() - start:.2f}s")

with ThreadPoolExecutor(max_workers=4) as pool:
    futures = {pool.submit(ping, h): h for h in hosts[:3]}
    for fut in as_completed(futures):               # in completion order
        print(futures[fut], fut.result())
```

- `submit` returns a **Future**; `.result()` waits and returns the value or **re-raises** the worker's exception.
- `map` raises the first exception when you reach that result.
- The `with` block waits for all work to finish.

Swap `ThreadPoolExecutor` for `ProcessPoolExecutor` to use processes: same code.

```python,norun
from concurrent.futures import ProcessPoolExecutor
import hashlib

def slow_hash(data: bytes) -> str:
    for _ in range(200_000):
        data = hashlib.sha256(data).digest()
    return data.hex()[:12]

if __name__ == "__main__":                 # REQUIRED for processes on Windows and macOS
    with ProcessPoolExecutor() as pool:     # defaults to one worker per CPU core
        print(list(pool.map(slow_hash, [b"a", b"b", b"c", b"d"])))
```

Process rules:

- On Windows/macOS, child processes **re-import your main module**, so all start-up code must be under `if __name__ == "__main__":`.
- Arguments and results are **pickled** (serialised) to cross between processes: they must be picklable (no lambdas, open files or sockets), and big data is expensive to send. Send work in **chunks** (`map(..., chunksize=100)`).
- Starting processes takes time; only worth it for work that takes noticeably longer than that.

## Thread safety

Threads share memory. A compound operation like `counter += 1` (read, add, write) can interleave between threads and lose updates. Protect shared state:

```python
import threading

counter = 0
lock = threading.Lock()

def work():
    global counter
    for _ in range(100_000):
        with lock:
            counter += 1

threads = [threading.Thread(target=work) for _ in range(4)]
for t in threads: t.start()
for t in threads: t.join()
print(counter)
```

Better still, **don't share**: give each worker its own data and combine the results (as `pool.map` does), or pass messages through a `queue.Queue`, which is thread-safe.

Other tools: `threading.Event` (signal "stop"), `threading.local()` (per-thread data), `multiprocessing.Manager` / `shared_memory` (sharing between processes).

## Choosing

```text
Waiting on many network calls?           → asyncio (or threads if the libraries are blocking)
Waiting on a few blocking calls?          → ThreadPoolExecutor
Crunching numbers in pure Python?         → ProcessPoolExecutor (or NumPy, or a Rust/C extension)
Running other programs?                    → subprocess (they're separate processes already)
```

@@quiz
Why don't threads speed up CPU-bound pure Python code in standard CPython?
* The GIL lets only one thread run Python bytecode at a time
- Threads aren't supported on Windows
- Python has no threads
- Threads always run on one core by OS rule
? Use processes for CPU-bound work.

@@quiz
Why must process-pool code be under `if __name__ == "__main__":` on Windows?
- For speed
* Child processes re-import the main module, which would start new pools endlessly
- Processes can't see functions otherwise
- It's only a style rule
? The spawn start method imports the script in each child.

@@quiz
What does `future.result()` do if the worker raised an exception?
- Returns None
* Re-raises the exception in the caller
- Logs and continues
- Returns the exception object
? Errors aren't lost; they surface where you collect results.

@@quiz
Which is best for 500 HTTP requests using a blocking library like `requests`?
- ProcessPoolExecutor
* ThreadPoolExecutor
- A single loop
- multiprocessing.Manager
? The work is waiting on I/O; threads release the GIL while waiting.

@@drill
What does GIL stand for?
= global interpreter lock
? Only one thread runs Python bytecode at a time.

@@flashcards
GIL :: Global Interpreter Lock: one thread runs Python bytecode at a time
I/O-bound :: mostly waiting on network/disk; threads and asyncio help
CPU-bound :: mostly computing; processes help
Free-threaded Python :: optional GIL-free CPython build (3.13t, supported in 3.14)
Future :: placeholder for a result from an executor
`as_completed` :: yields futures as they finish
Pickling :: serialising objects to send between processes
Race condition :: result depends on thread timing, e.g. lost `+=` updates
`threading.Lock` :: lets only one thread into a block at a time

@@exercise Parallel checks with error handling
Write `check_hosts(hosts, check, workers)` using `ThreadPoolExecutor`. `check(host)` returns a string or raises. Return a list of `(host, result)` **in the original order**, where a raised exception becomes `"ERROR: <ExceptionType>: <message>"`. Use `submit` and keep the futures in order.
@@starter
from concurrent.futures import ThreadPoolExecutor
import time


def check_hosts(hosts, check, workers):
    pass


def fake(host):
    time.sleep(0.05)
    if host == "db02":
        raise TimeoutError("no reply")
    return "up"


print(check_hosts(["web01", "db02", "web02"], fake, 3))
@@tests
start = time.perf_counter()
r = check_hosts(["web01", "db02", "web02", "web03"], fake, 4)
elapsed = time.perf_counter() - start
check(r, [("web01", "up"), ("db02", "ERROR: TimeoutError: no reply"), ("web02", "up"), ("web03", "up")], "results in order")
check(elapsed < 0.15, True, "ran in parallel")
check(check_hosts([], fake, 2), [], "empty")
@@requires
ThreadPoolExecutor(
@@hint
`with ThreadPoolExecutor(max_workers=workers) as pool: futures = [(h, pool.submit(check, h)) for h in hosts]`. Then loop: `try: result = fut.result()` / `except Exception as e: result = f"ERROR: {type(e).__name__}: {e}"`.
@@solution
from concurrent.futures import ThreadPoolExecutor
import time


def check_hosts(hosts, check, workers):
    results = []
    with ThreadPoolExecutor(max_workers=workers) as pool:
        futures = [(h, pool.submit(check, h)) for h in hosts]
        for host, fut in futures:
            try:
                result = fut.result()
            except Exception as e:
                result = f"ERROR: {type(e).__name__}: {e}"
            results.append((host, result))
    return results


def fake(host):
    time.sleep(0.05)
    if host == "db02":
        raise TimeoutError("no reply")
    return "up"


print(check_hosts(["web01", "db02", "web02"], fake, 3))

@@exercise A thread-safe counter and work splitting
- `SafeCounter` with `increment(key)` and `snapshot()` (a copy of the counts as a dict), safe to call from many threads (use a `threading.Lock`).
- `chunk(items, n)`: split a list into `n` nearly equal consecutive parts (sizes differ by at most 1, bigger parts first, no empty parts unless `items` is shorter than `n`, in which case return only non-empty parts). This is how work is divided between processes.
@@starter
import threading


class SafeCounter:
    pass


def chunk(items, n):
    pass


print(chunk(list(range(10)), 3))
@@tests
c = SafeCounter()
def hammer():
    for _ in range(20000):
        c.increment("hits")
ts = [threading.Thread(target=hammer) for _ in range(4)]
for t in ts:
    t.start()
for t in ts:
    t.join()
check(c.snapshot(), {"hits": 80000}, "no lost updates")
snap = c.snapshot()
snap["hits"] = 0
check(c.snapshot()["hits"], 80000, "snapshot is a copy")
check(chunk(list(range(10)), 3), [[0, 1, 2, 3], [4, 5, 6], [7, 8, 9]], "uneven")
check(chunk([1, 2], 4), [[1], [2]], "fewer items than parts")
check(chunk([], 3), [], "empty")
@@requires
threading.Lock
@@hint
In `increment`: `with self._lock: self._counts[key] = self._counts.get(key, 0) + 1`; `snapshot` copies inside the lock. For `chunk`: `size, extra = divmod(len(items), n)`; part `i` has `size + (1 if i < extra else 0)` items; skip empty parts.
@@solution
import threading


class SafeCounter:
    def __init__(self):
        self._lock = threading.Lock()
        self._counts = {}

    def increment(self, key):
        with self._lock:
            self._counts[key] = self._counts.get(key, 0) + 1

    def snapshot(self):
        with self._lock:
            return dict(self._counts)


def chunk(items, n):
    size, extra = divmod(len(items), n)
    parts, start = [], 0
    for i in range(n):
        length = size + (1 if i < extra else 0)
        if length:
            parts.append(items[start:start + length])
        start += length
    return parts


print(chunk(list(range(10)), 3))
