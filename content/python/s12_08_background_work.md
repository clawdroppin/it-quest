@@lesson s12-tk-background Background Work: Threads, Queues and Progress
@@concepts concurrency, objects
@@text
Tkinter runs everything on **one thread**: the event loop. If a button handler pings 200 hosts or downloads a file, the window freezes ("Not Responding") until it finishes. The fix is always the same: do slow work **somewhere else** and send results back to the GUI thread.

## The golden rule

**Only the main thread may touch widgets.** Tk isn't thread-safe; calling `label.config()` from a worker thread works sometimes and crashes or corrupts state at other times.

```text
 worker thread                      main (Tk) thread
 ─────────────                      ────────────────
 do slow work  ──put──▶  queue  ◀──get_nowait──  root.after(100, poll)
                                                 update widgets
```

## Thread + queue + after

```python,norun
import queue
import threading
import time
import tkinter as tk
from tkinter import ttk

root = tk.Tk()
bar = ttk.Progressbar(root, length=300, maximum=100)
bar.pack(padx=10, pady=10)
status = ttk.Label(root, text="Idle")
status.pack()
messages = queue.Queue()
cancel = threading.Event()

def scan(hosts):                                  # runs in the worker thread
    for i, host in enumerate(hosts, 1):
        if cancel.is_set():
            messages.put(("done", "Cancelled"))
            return
        time.sleep(0.05)                          # pretend to ping
        messages.put(("progress", i * 100 // len(hosts), host))
    messages.put(("done", f"Scanned {len(hosts)} hosts"))

def poll():                                       # runs in the Tk thread
    try:
        while True:
            msg = messages.get_nowait()
            if msg[0] == "progress":
                bar["value"] = msg[1]
                status.config(text=f"Pinging {msg[2]}...")
            elif msg[0] == "done":
                status.config(text=msg[1])
                start.state(["!disabled"])
                return                            # stop polling
    except queue.Empty:
        pass
    root.after(100, poll)

def on_start():
    cancel.clear()
    start.state(["disabled"])
    hosts = [f"10.0.0.{i}" for i in range(1, 101)]
    threading.Thread(target=scan, args=(hosts,), daemon=True).start()
    poll()

start = ttk.Button(root, text="Scan", command=on_start)
start.pack(side="left", padx=10, pady=10)
ttk.Button(root, text="Cancel", command=cancel.set).pack(side="left")
root.mainloop()
```

Key points:

- `queue.Queue` is thread-safe; the worker only ever calls `put`.
- The GUI **polls** with `after`: `get_nowait()` never blocks, so the window stays responsive.
- **Cancellation** is cooperative: a `threading.Event` the worker checks between steps. You can't kill a thread from outside.
- `daemon=True` lets the program exit even if a worker is still running.
- Disable the Start button while running so the user can't start ten scans at once.

## Other options

- `concurrent.futures.ThreadPoolExecutor` for many parallel jobs (ping 200 hosts with 32 workers); attach `future.add_done_callback(lambda f: messages.put(...))`, never update widgets directly from the callback.
- **asyncio**: run an event loop in a background thread and submit coroutines with `asyncio.run_coroutine_threadsafe`; results still travel through the queue.
- **Processes** (`multiprocessing`, `ProcessPoolExecutor`) for CPU-heavy work (hashing large files, image processing) where the GIL would limit threads.
- `root.event_generate("<<ScanDone>>", when="tail")` is sometimes used as a wake-up from threads, but it isn't guaranteed safe on every platform; the queue + `after` pattern is.

## Good progress feedback

- Show **determinate** progress (`mode="determinate"`, 0–100) when you know the total, **indeterminate** (`mode="indeterminate"`, `bar.start()`) when you don't.
- Show what's happening ("Pinging 10.0.0.42...") and an ETA for long jobs.
- Always allow cancel for anything over a few seconds.
- Report errors from the worker through the queue too (`("error", str(exc))`): an exception in a thread otherwise just prints to the console and vanishes.

@@quiz
Why does a long-running button handler freeze the window?
* Tk handles events on one thread; while the handler runs, no events (redraws, clicks) are processed
- Python is slow
- Buttons have a timeout
- The GIL stops all threads
? Move slow work to a worker thread.

@@quiz
How should a worker thread update a label?
- Call `label.config()` directly
* Put a message on a queue; the main thread polls it with `after` and updates the label
- Use a global variable and hope
- Call `root.mainloop()` again
? Only the main thread touches widgets.

@@quiz
How do you cancel a running worker thread?
- `thread.kill()`
* Set a `threading.Event` that the worker checks between steps, and let it return
- Delete the thread object
- Close the window
? Python threads can't be killed from outside; cancellation is cooperative.

@@quiz
When should you use processes instead of threads for background work?
- For network I/O
* For CPU-heavy work, where the GIL limits threads
- Never in GUI apps
- For updating widgets
? Threads are fine for waiting on I/O.

@@drill
Which widget method schedules a function to run on the GUI thread after a delay?
= after
= after()
= root.after
? Use it to poll a queue filled by a worker thread.

@@flashcards
Event loop :: the single Tk thread that processes events and redraws
Golden rule :: only the main thread touches widgets
`queue.Queue` :: thread-safe channel from worker to GUI
`get_nowait` :: non-blocking read; raises `queue.Empty`
`threading.Event` :: flag used for cooperative cancellation
daemon thread :: doesn't keep the program alive at exit
Determinate progress :: known total, 0–100%
Indeterminate progress :: unknown total, moving bar
ThreadPoolExecutor :: pool of threads for many parallel I/O jobs

@@exercise Progress text and ETA
Write `progress_text(done, total, elapsed)` that returns the status line for a progress bar:

- `"<done>/<total> (<percent>%)"` with the percent as an integer rounded down
- then, when at least one item is done and some remain, `" · about <eta> left"` where the ETA is `elapsed / done * (total - done)` seconds formatted by `format_duration`
- when `done == total`: `" · finished in <elapsed>"`
- `total == 0` → `"0/0 (100%) · finished in 0s"`

and `format_duration(seconds)`: round to whole seconds; under 60 → `"42s"`, under an hour → `"3m 05s"`, otherwise `"1h 02m"`.
@@starter
def format_duration(seconds):
    pass


def progress_text(done, total, elapsed):
    pass


print(format_duration(42.4), format_duration(185), format_duration(3725))
print(progress_text(0, 200, 0.0))
print(progress_text(50, 200, 12.5))
print(progress_text(200, 200, 61.2))
@@expected
42s 3m 05s 1h 02m
0/200 (0%)
50/200 (25%) · about 38s left
200/200 (100%) · finished in 1m 01s
@@tests
check(progress_text(0, 0, 0), "0/0 (100%) · finished in 0s", "nothing to do")
check(progress_text(2, 3, 2), "2/3 (66%) · about 1s left", "rounded down percent")
check(format_duration(59.6), "1m 00s", "rounds to a minute")
check(format_duration(0), "0s", "zero")
@@hint
`s = round(seconds)`; `if s < 60: return f"{s}s"`; `if s < 3600: return f"{s // 60}m {s % 60:02d}s"`; else hours and minutes with `divmod`. Percent: `done * 100 // total`.
@@solution
def format_duration(seconds):
    s = round(seconds)
    if s < 60:
        return f"{s}s"
    if s < 3600:
        return f"{s // 60}m {s % 60:02d}s"
    hours, rest = divmod(s, 3600)
    return f"{hours}h {rest // 60:02d}m"


def progress_text(done, total, elapsed):
    if total == 0:
        return "0/0 (100%) · finished in 0s"
    text = f"{done}/{total} ({done * 100 // total}%)"
    if done == total:
        return text + f" · finished in {format_duration(elapsed)}"
    if done > 0:
        eta = elapsed / done * (total - done)
        text += f" · about {format_duration(eta)} left"
    return text


print(format_duration(42.4), format_duration(185), format_duration(3725))
print(progress_text(0, 200, 0.0))
print(progress_text(50, 200, 12.5))
print(progress_text(200, 200, 61.2))

@@exercise A worker that reports through a queue
Build the non-GUI half of the scan example:

- `scan(hosts, check_host, messages, cancel)`: for each host, stop early if `cancel.is_set()` (put `("done", "cancelled")` and return). Otherwise call `check_host(host)`; put `("result", host, value)`; if it raises, put `("error", host, str(exc))` and carry on. At the end put `("done", "ok")`.
- `drain(messages, max_items=50)`: what the GUI's `poll` does each tick: take up to `max_items` messages **without blocking** and return them as a list (empty list if none).

The test runs `scan` in a real thread and drains until it sees `"done"`.
@@starter
import queue
import threading


def scan(hosts, check_host, messages, cancel):
    pass


def drain(messages, max_items=50):
    pass


def fake_check(host):
    if host.endswith(".3"):
        raise TimeoutError("timed out")
    return int(host.split(".")[-1]) * 10


q = queue.Queue()
scan(["10.0.0.1", "10.0.0.2", "10.0.0.3"], fake_check, q, threading.Event())
print(drain(q, max_items=2))
print(drain(q))
print(drain(q))
@@expected
[('result', '10.0.0.1', 10), ('result', '10.0.0.2', 20)]
[('error', '10.0.0.3', 'timed out'), ('done', 'ok')]
[]
@@tests
import time
q = queue.Queue()
cancel = threading.Event()
hosts = [f"h{i}" for i in range(30)]
def slow(h):
    time.sleep(0.001)
    return len(h)
t = threading.Thread(target=scan, args=(hosts, slow, q, cancel), daemon=True)
t.start()
seen = []
deadline = time.monotonic() + 5
while time.monotonic() < deadline:
    seen.extend(drain(q))
    if seen and seen[-1][0] == "done":
        break
    time.sleep(0.005)
check(len([m for m in seen if m[0] == "result"]), 30, "all results arrive")
check(seen[-1], ("done", "ok"), "finished")
q2 = queue.Queue()
stop = threading.Event()
stop.set()
scan(hosts, slow, q2, stop)
check(drain(q2), [("done", "cancelled")], "cancelled before start")
@@hint
`scan`: loop, `if cancel.is_set(): messages.put(("done", "cancelled")); return`, then `try: value = check_host(host) except Exception as exc: ... else: ...`. `drain`: loop `max_items` times calling `messages.get_nowait()` and stop on `queue.Empty`.
@@solution
import queue
import threading


def scan(hosts, check_host, messages, cancel):
    for host in hosts:
        if cancel.is_set():
            messages.put(("done", "cancelled"))
            return
        try:
            value = check_host(host)
        except Exception as exc:
            messages.put(("error", host, str(exc)))
        else:
            messages.put(("result", host, value))
    messages.put(("done", "ok"))


def drain(messages, max_items=50):
    items = []
    for _ in range(max_items):
        try:
            items.append(messages.get_nowait())
        except queue.Empty:
            break
    return items


def fake_check(host):
    if host.endswith(".3"):
        raise TimeoutError("timed out")
    return int(host.split(".")[-1]) * 10


q = queue.Queue()
scan(["10.0.0.1", "10.0.0.2", "10.0.0.3"], fake_check, q, threading.Event())
print(drain(q, max_items=2))
print(drain(q))
print(drain(q))
