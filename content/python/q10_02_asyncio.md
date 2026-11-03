@@lesson q10-asyncio asyncio in Depth: Tasks, Timeouts, Limits and Queues
@@concepts concurrency, networking
@@text
The concurrency lesson introduced `async`/`await`. This lesson goes deeper into how asyncio works and the patterns real network tools use: running thousands of operations at once **without** overwhelming anything, timing out the slow ones, and cancelling cleanly.

## The event loop in one picture

```text
event loop: ready queue → run task A until it awaits I/O → run task B → ... → I/O finished? → A is ready again
```

- A coroutine (`async def`) runs until it hits an `await` on something not ready yet (a socket, a timer), then **yields** control to the loop.
- One thread, many tasks: no locks needed for ordinary data, but **one blocking call** (`time.sleep`, `requests.get`, a heavy calculation) freezes **everything**. Use `await asyncio.sleep`, async libraries (`httpx`, `aiohttp`, `asyncpg`), or `await asyncio.to_thread(blocking_fn)`.
- asyncio is ideal for **I/O-bound** work with many connections: port scanners, API clients, chat servers, crawlers.

## Tasks: start now, await later

```python
import asyncio, time

async def check(host, delay):
    await asyncio.sleep(delay)            # stands in for a network call
    return f"{host} ok"

async def main():
    start = time.perf_counter()
    t1 = asyncio.create_task(check("web", 0.2))   # starts running immediately
    t2 = asyncio.create_task(check("db", 0.1))
    print(await t1, await t2)
    print(f"took {time.perf_counter() - start:.1f}s (not 0.3)")

asyncio.run(main())
```

Keep a reference to every task you create; a task nobody references can be garbage-collected mid-run.

## gather vs TaskGroup

```python
import asyncio

async def check(host):
    await asyncio.sleep(0.01)
    if host == "bad":
        raise ConnectionError(host)
    return host

async def main():
    # gather: results in input order; return_exceptions=True collects errors instead of raising
    print(await asyncio.gather(*(check(h) for h in ["a", "bad", "c"]), return_exceptions=True))

    # TaskGroup (3.11+): structured concurrency; if one fails, the others are cancelled
    try:
        async with asyncio.TaskGroup() as tg:
            tasks = [tg.create_task(check(h)) for h in ["a", "bad", "c"]]
    except* ConnectionError as eg:
        print("failed:", [str(e) for e in eg.exceptions])

asyncio.run(main())
```

`except*` handles an **ExceptionGroup**, which TaskGroup raises because several tasks may fail at once. Prefer TaskGroup in new code: no task can leak or be forgotten.

## Timeouts and cancellation

```python
import asyncio

async def slow():
    try:
        await asyncio.sleep(10)
    except asyncio.CancelledError:
        print("cleaning up")          # close sockets, release locks...
        raise                         # always re-raise CancelledError

async def main():
    try:
        async with asyncio.timeout(0.05):
            await slow()
    except TimeoutError:
        print("timed out")

asyncio.run(main())
```

Cancellation is delivered as a `CancelledError` **at an await point**. Catch it only to clean up, then re-raise. `asyncio.wait_for(coro, timeout)` is the older one-call equivalent.

## Limiting concurrency with a Semaphore

Launching 10,000 requests at once will exhaust sockets or get you rate-limited. A semaphore allows at most N inside a block:

```python
import asyncio

async def fetch(sem, i):
    async with sem:                     # waits if 3 are already running
        await asyncio.sleep(0.01)
        return i

async def main():
    sem = asyncio.Semaphore(3)
    results = await asyncio.gather(*(fetch(sem, i) for i in range(10)))
    print(results)

asyncio.run(main())
```

## Queues: producers and consumers

`asyncio.Queue` connects stages: producers put work in, a fixed pool of workers takes it out. It gives natural back-pressure with `maxsize`:

```python
import asyncio

async def worker(name, queue, results):
    while True:
        host = await queue.get()
        try:
            await asyncio.sleep(0.01)
            results.append(f"{name}:{host}")
        finally:
            queue.task_done()

async def main():
    queue, results = asyncio.Queue(), []
    for h in ["a", "b", "c", "d", "e"]:
        queue.put_nowait(h)
    workers = [asyncio.create_task(worker(f"w{i}", queue, results)) for i in range(2)]
    await queue.join()                  # wait until every item is processed
    for w in workers:
        w.cancel()
    print(len(results), "done")

asyncio.run(main())
```

## Synchronisation and streams

- `asyncio.Lock`, `Event`, `Condition`: like threading's, but awaitable.
- `asyncio.open_connection(host, port)` gives a `(reader, writer)` pair for TCP clients; `asyncio.start_server` builds servers.
- Debug mode (`asyncio.run(main(), debug=True)` or `PYTHONASYNCIODEBUG=1`) warns about slow callbacks and never-awaited coroutines.

@@quiz
What happens if a coroutine calls `time.sleep(5)`?
- Only that task waits
* The whole event loop is blocked, so every task freezes for 5 seconds
- asyncio converts it automatically
- It raises an error
? Use `await asyncio.sleep` or `asyncio.to_thread`.

@@quiz
What does an `asyncio.Semaphore(3)` do?
* Lets at most 3 tasks into the `async with` block at a time
- Runs 3 event loops
- Retries 3 times
- Times out after 3 seconds
? It limits concurrency.

@@quiz
Why prefer `asyncio.TaskGroup` over loose `create_task` calls?
- It's faster
* It guarantees all tasks finish or are cancelled before the block exits, and errors aren't lost
- It runs tasks in threads
- It removes the need for await
? Structured concurrency.

@@quiz
What should you do after catching `asyncio.CancelledError`?
- Ignore it and continue
* Clean up and re-raise it
- Restart the task
- Call sys.exit
? Swallowing cancellation breaks timeouts and shutdown.

@@flashcards
Event loop :: runs coroutines, switching at await points
Task :: a scheduled coroutine, created with `asyncio.create_task`
`asyncio.gather` :: run awaitables concurrently, results in order
TaskGroup :: structured concurrency; cancels siblings on failure
ExceptionGroup / `except*` :: several exceptions raised together
`asyncio.timeout(s)` :: async context manager raising TimeoutError
Semaphore :: limits how many tasks run a section at once
`asyncio.Queue` :: async producer/consumer queue
`asyncio.to_thread` :: run blocking code in a thread without blocking the loop

@@exercise A polite concurrent checker
Write `async def check_all(hosts, limit)` that "checks" every host concurrently with at most `limit` running at once, using `fake_check` (given). Return a dict host → result, where result is `"ok"` or, if `fake_check` raised, `"error: <message>"`. Also record the highest number of checks that were running at the same time in the global-free way shown: return a tuple `(results, max_running)`.
@@starter
import asyncio


async def fake_check(host):
    await asyncio.sleep(0.01)
    if host.startswith("bad"):
        raise ConnectionError(f"{host} unreachable")
    return "ok"


async def check_all(hosts, limit):
    pass


results, peak = asyncio.run(check_all(["a", "bad1", "c", "d", "e", "f"], 2))
print(results, peak)
@@tests
results, peak = asyncio.run(check_all(["a", "bad1", "c", "d", "e", "f"], 2))
check(results["a"], "ok", "ok host")
check(results["bad1"], "error: bad1 unreachable", "error captured")
check(len(results), 6, "all hosts")
check(peak, 2, "limit respected and used")
results, peak = asyncio.run(check_all(["x"] * 1 + ["y", "z"], 5))
check(peak, 3, "fewer hosts than limit")
check(asyncio.run(check_all([], 3)), ({}, 0), "empty")
@@requires
Semaphore
@@hint
Use a small state dict `state = {"running": 0, "peak": 0}` local to `check_all`. Inside an inner `async def one(host):` do `async with sem:` then increase running, update peak, `try: return host, await fake_check(host)` / `except Exception as e: return host, f"error: {e}"` / `finally: state["running"] -= 1`. Gather all and build the dict.
@@solution
import asyncio


async def fake_check(host):
    await asyncio.sleep(0.01)
    if host.startswith("bad"):
        raise ConnectionError(f"{host} unreachable")
    return "ok"


async def check_all(hosts, limit):
    sem = asyncio.Semaphore(limit)
    state = {"running": 0, "peak": 0}

    async def one(host):
        async with sem:
            state["running"] += 1
            state["peak"] = max(state["peak"], state["running"])
            try:
                return host, await fake_check(host)
            except Exception as e:
                return host, f"error: {e}"
            finally:
                state["running"] -= 1

    pairs = await asyncio.gather(*(one(h) for h in hosts))
    return dict(pairs), state["peak"]


results, peak = asyncio.run(check_all(["a", "bad1", "c", "d", "e", "f"], 2))
print(results, peak)

@@exercise Timeouts and a worker pool
Write `async def process(jobs, workers, timeout)`. Each job is `(name, seconds)`; processing it means `await asyncio.sleep(seconds)`. Use an `asyncio.Queue` and `workers` worker tasks. A job that takes longer than `timeout` is abandoned (use `asyncio.timeout`). Return a dict with sorted lists: `{"done": [...], "timed_out": [...]}`. Cancel the workers when the queue is finished.
@@starter
import asyncio


async def process(jobs, workers, timeout):
    pass


print(asyncio.run(process([("a", 0.01), ("slow", 0.5), ("b", 0.02), ("c", 0.01)], 2, 0.1)))
@@tests
r = asyncio.run(process([("a", 0.01), ("slow", 0.5), ("b", 0.02), ("c", 0.01)], 2, 0.1))
check(r, {"done": ["a", "b", "c"], "timed_out": ["slow"]}, "result")
check(asyncio.run(process([], 3, 0.1)), {"done": [], "timed_out": []}, "no jobs")
import time
start = time.perf_counter()
asyncio.run(process([("x", 0.05)] * 4, 4, 1))
check(time.perf_counter() - start < 0.18, True, "workers run concurrently")
@@requires
asyncio.Queue
@@hint
Fill the queue with `put_nowait`. A worker: `while True: name, secs = await queue.get()`, then `try: async with asyncio.timeout(timeout): await asyncio.sleep(secs); done.append(name)` / `except TimeoutError: timed_out.append(name)` / `finally: queue.task_done()`. After `await queue.join()`, cancel the workers.
@@solution
import asyncio


async def process(jobs, workers, timeout):
    queue = asyncio.Queue()
    for job in jobs:
        queue.put_nowait(job)
    done, timed_out = [], []

    async def worker():
        while True:
            name, secs = await queue.get()
            try:
                async with asyncio.timeout(timeout):
                    await asyncio.sleep(secs)
                done.append(name)
            except TimeoutError:
                timed_out.append(name)
            finally:
                queue.task_done()

    tasks = [asyncio.create_task(worker()) for _ in range(workers)]
    await queue.join()
    for t in tasks:
        t.cancel()
    await asyncio.gather(*tasks, return_exceptions=True)
    return {"done": sorted(done), "timed_out": sorted(timed_out)}


print(asyncio.run(process([("a", 0.01), ("slow", 0.5), ("b", 0.02), ("c", 0.01)], 2, 0.1)))
