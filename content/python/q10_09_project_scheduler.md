@@lesson q10-scheduler 🏆 Module Project: A Job Runner with Dependencies and Retries
@@concepts concurrency, automation, errors
@@text
This project combines the module's ideas: asyncio tasks and limits, robust error handling, and clean data structures. You'll build the core of a **job runner**: the engine inside CI pipelines (GitHub Actions, GitLab CI), build tools (make, Bazel), workflow schedulers (Airflow, Prefect) and deployment tools (Ansible).

## The problem

A nightly maintenance run has jobs that depend on each other:

```text
snapshot_db ─┐
             ├─▶ backup_files ─▶ verify_backup ─▶ report
rotate_logs ─┘                                     ▲
patch_check ───────────────────────────────────────┘
```

- A job may start only when **all** its dependencies have succeeded.
- Independent jobs should run **in parallel** (but no more than N at once).
- Flaky jobs get **retries** with a delay.
- If a job fails for good, everything depending on it is **skipped**; unrelated jobs keep running.
- A dependency **cycle** (A needs B, B needs A) is a configuration error, detected before anything runs.

## Graphs and topological order

Jobs and dependencies form a **directed graph**. An order in which every job comes after its dependencies is a **topological order**. Kahn's algorithm finds one:

1. Count each job's unmet dependencies (its **in-degree**).
2. Start with the jobs that have none.
3. Repeatedly take a ready job, output it, and reduce the count of every job that depends on it; jobs reaching zero become ready.
4. If jobs remain that never became ready, there's a cycle.

Python's standard library even includes `graphlib.TopologicalSorter`, which implements exactly this, including a parallel-friendly `get_ready()` / `done()` API. The first exercise writes it by hand so you understand it; in production, use `graphlib`.

## Retries with backoff

Transient failures (network hiccups, locked files, rate limits) often succeed on a second try. Good retry logic:

- retries only errors that are likely temporary,
- waits between attempts with **exponential backoff** (0.5 s, 1 s, 2 s...) plus a little random **jitter** so many clients don't retry in lockstep,
- has a maximum number of attempts,
- records each attempt for the report.

The `tenacity` library offers this as a decorator (`@retry(stop=stop_after_attempt(3), wait=wait_exponential())`).

## Designing for testability

The runner takes jobs as **async functions** and gets its sleep function injected, so tests can run instantly and deterministically. That's the same dependency-injection idea from the protocols lesson.

## Extending it

- Timeouts per job (`asyncio.timeout`).
- Persist state so a re-run skips jobs that already succeeded today.
- Read the job graph from a YAML/TOML file.
- Expose progress as JSON for a dashboard.

@@quiz
What is a topological order?
* An ordering where every job comes after all of its dependencies
- Alphabetical order
- The order jobs were defined in
- The fastest order
? Only possible if the graph has no cycles.

@@quiz
In Kahn's algorithm, how do you detect a cycle?
- A job depends on itself only
* Some jobs never reach zero unmet dependencies, so they're never output
- The queue becomes infinite
- Python raises RecursionError
? Every job in or after a cycle stays blocked.

@@quiz
Why add jitter to retry delays?
- To make tests slower
* So many clients that failed together don't all retry at the same moment
- To encrypt requests
- It's required by HTTP
? Synchronised retries can overload a recovering service.

@@flashcards
Directed graph :: nodes connected by one-way edges
Topological order :: every node after its dependencies
Kahn's algorithm :: topological sort by repeatedly removing nodes with no unmet dependencies
In-degree :: number of unmet dependencies of a node
`graphlib.TopologicalSorter` :: standard library topological sorter
Exponential backoff :: doubling the wait after each failed attempt
Jitter :: random variation added to retry delays

@@exercise Order the jobs
Write `plan(deps)` where `deps` maps each job to a list of jobs it depends on. Every dependency is also a key. Return a list of **stages**: each stage is a sorted list of jobs whose dependencies are all in earlier stages (these could run in parallel). If there's a cycle, raise `ValueError("cycle: a, b")` naming the sorted jobs that could never be scheduled.
@@starter
def plan(deps):
    pass


DEPS = {
    "snapshot_db": [],
    "rotate_logs": [],
    "patch_check": [],
    "backup_files": ["snapshot_db", "rotate_logs"],
    "verify_backup": ["backup_files"],
    "report": ["verify_backup", "patch_check"],
}
for i, stage in enumerate(plan(DEPS), 1):
    print(i, stage)
@@expected
1 ['patch_check', 'rotate_logs', 'snapshot_db']
2 ['backup_files']
3 ['verify_backup']
4 ['report']
@@tests
check(plan({}), [], "empty")
check(plan({"a": [], "b": ["a"], "c": ["a"]}), [["a"], ["b", "c"]], "fan out")
try:
    plan({"a": ["c"], "b": ["a"], "c": ["b"], "d": []})
    check("planned", "ValueError", "cycle")
except ValueError as e:
    check(str(e), "cycle: a, b, c", "cycle message")
@@hint
`remaining = {job: set(d) for job, d in deps.items()}`. Loop: `ready = sorted(j for j, d in remaining.items() if not d)`; if none ready but jobs remain, raise; otherwise append the stage, delete those jobs from `remaining`, and remove them from every other job's set.
@@solution
def plan(deps):
    remaining = {job: set(d) for job, d in deps.items()}
    stages = []
    while remaining:
        ready = sorted(j for j, d in remaining.items() if not d)
        if not ready:
            raise ValueError("cycle: " + ", ".join(sorted(remaining)))
        stages.append(ready)
        for j in ready:
            del remaining[j]
        for d in remaining.values():
            d.difference_update(ready)
    return stages


DEPS = {
    "snapshot_db": [],
    "rotate_logs": [],
    "patch_check": [],
    "backup_files": ["snapshot_db", "rotate_logs"],
    "verify_backup": ["backup_files"],
    "report": ["verify_backup", "patch_check"],
}
for i, stage in enumerate(plan(DEPS), 1):
    print(i, stage)

@@exercise Run them with retries
Write `async def run(jobs, deps, retries, sleep)`:

- `jobs` maps a name to an async function (no arguments) that returns nothing or raises.
- A job starts as soon as all its dependencies succeeded (run independent jobs concurrently; a simple approach is one task per job that first awaits its dependencies' results).
- A failing job is retried up to `retries` extra times, calling `await sleep(0.1 * 2 ** (attempt - 1))` before each retry (`attempt` is 1 for the first retry).
- If a dependency failed or was skipped, the job is `"skipped"` and never called.
- Return a dict name → `"ok"`, `"failed"` or `"skipped"`, plus record the delays passed to `sleep` (the test injects a fake sleep).
@@starter
import asyncio


async def run(jobs, deps, retries, sleep):
    pass


calls = {"flaky": 0}


async def ok():
    await asyncio.sleep(0)


async def flaky():
    calls["flaky"] += 1
    if calls["flaky"] < 3:
        raise ConnectionError("try again")


async def broken():
    raise RuntimeError("disk failure")


async def no_sleep(seconds):
    pass


JOBS = {"a": ok, "b": flaky, "c": broken, "d": ok, "e": ok}
DEPS = {"a": [], "b": ["a"], "c": [], "d": ["b"], "e": ["c", "a"]}
print(asyncio.run(run(JOBS, DEPS, 2, no_sleep)))
@@expected
{'a': 'ok', 'b': 'ok', 'c': 'failed', 'd': 'ok', 'e': 'skipped'}
@@tests
calls["flaky"] = 0
delays = []
async def fake_sleep(s):
    delays.append(s)
result = asyncio.run(run(JOBS, DEPS, 2, fake_sleep))
check(result, {"a": "ok", "b": "ok", "c": "failed", "d": "ok", "e": "skipped"}, "statuses")
check(sorted(delays), [0.1, 0.1, 0.2, 0.2], "backoff delays (flaky twice, broken twice)")
calls["flaky"] = 0
check(asyncio.run(run({"b": flaky}, {"b": []}, 1, fake_sleep)), {"b": "failed"}, "not enough retries")
order = []
async def rec_a():
    order.append("a")
async def rec_b():
    order.append("b")
asyncio.run(run({"b": rec_b, "a": rec_a}, {"b": ["a"], "a": []}, 0, fake_sleep))
check(order, ["a", "b"], "dependencies first")
@@hint
Create one task per job up front and store them in a dict, so each job can `await tasks[dep]` for its dependencies. Inside `async def one(name)`: `for dep in deps[name]: if await tasks[dep] != "ok": return "skipped"`, then loop `for attempt in range(retries + 1)`: if attempt > 0 sleep first; `try: await jobs[name](); return "ok" except Exception: pass`; after the loop return `"failed"`. Since tasks reference the `tasks` dict, fill it before any task runs (creating tasks doesn't run them until you await).
@@solution
import asyncio


async def run(jobs, deps, retries, sleep):
    tasks = {}

    async def one(name):
        for dep in deps[name]:
            if await tasks[dep] != "ok":
                return "skipped"
        for attempt in range(retries + 1):
            if attempt > 0:
                await sleep(0.1 * 2 ** (attempt - 1))
            try:
                await jobs[name]()
                return "ok"
            except Exception:
                pass
        return "failed"

    for name in jobs:
        tasks[name] = asyncio.create_task(one(name))
    results = await asyncio.gather(*tasks.values())
    return dict(zip(tasks, results))


calls = {"flaky": 0}


async def ok():
    await asyncio.sleep(0)


async def flaky():
    calls["flaky"] += 1
    if calls["flaky"] < 3:
        raise ConnectionError("try again")


async def broken():
    raise RuntimeError("disk failure")


async def no_sleep(seconds):
    pass


JOBS = {"a": ok, "b": flaky, "c": broken, "d": ok, "e": ok}
DEPS = {"a": [], "b": ["a"], "c": [], "d": ["b"], "e": ["c", "a"]}
print(asyncio.run(run(JOBS, DEPS, 2, no_sleep)))
