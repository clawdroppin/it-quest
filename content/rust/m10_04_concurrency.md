@@lesson m10-concurrency Fearless Concurrency: Send, Sync, Arc, Mutex, Atomics and Scoped Threads
@@concepts concurrency
@@text
Module 8 spawned threads and sent messages on channels. Real concurrent programs also **share** data: a cache read by many threads, a counter of finished jobs, a work queue. Rust's type system turns most concurrency bugs (data races) into compile errors. This lesson shows how.

## Data races and why Rust prevents them

A **data race** happens when two threads access the same memory at the same time, at least one writes, and nothing synchronises them. The result is corrupted data that appears randomly. In C or C++ it compiles fine. In Rust it **doesn't compile**, thanks to two marker traits:

- **`Send`**: a value can be **moved** to another thread. Almost everything is `Send`; `Rc` is not (its counter isn't thread-safe).
- **`Sync`**: a value can be **shared by reference** between threads (`&T` is `Send`). `RefCell` and `Cell` are not `Sync`.

They're implemented automatically from a type's fields. `thread::spawn` requires its closure to be `Send + 'static`, so if you try to move an `Rc` into a thread, the compiler stops you and suggests `Arc`.

## Scoped threads: borrowing across threads

`thread::spawn` needs `'static` data because the thread might outlive the function. **Scoped threads** (`std::thread::scope`) guarantee every thread finishes before the scope ends, so threads may **borrow** local data:

```rust
use std::thread;

fn main() {
    let words = vec!["apple", "banana", "cherry", "date"];
    let (left, right) = words.split_at(2);
    let (a, b) = thread::scope(|s| {
        let h1 = s.spawn(|| left.iter().map(|w| w.len()).sum::<usize>());
        let h2 = s.spawn(|| right.iter().map(|w| w.len()).sum::<usize>());
        (h1.join().unwrap(), h2.join().unwrap())
    });
    println!("{a} + {b} letters, and words is still usable: {}", words.len());
}
```

No `Arc`, no cloning: the borrow checker knows the threads are done when `scope` returns. Prefer scoped threads for "split the work, wait for all of it" jobs.

## `Arc<T>`: shared ownership across threads

`Arc` is the thread-safe `Rc` (atomic reference count). `Arc::clone(&x)` hands another thread an owner of the same data. On its own it gives **read-only** access.

## `Mutex<T>` and `RwLock<T>`: shared mutable state

A `Mutex` (mutual exclusion) wraps data so only one thread at a time can access it. `lock()` returns a **guard**; the lock is released when the guard is dropped:

```rust
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let log = Arc::new(Mutex::new(Vec::new()));
    let mut handles = Vec::new();
    for id in 0..4 {
        let log = Arc::clone(&log);
        handles.push(thread::spawn(move || {
            log.lock().unwrap().push(format!("worker {id} done"));
        }));
    }
    for h in handles {
        h.join().unwrap();
    }
    let mut lines = log.lock().unwrap().clone();
    lines.sort();
    println!("{}", lines.join("\n"));
}
```

Notice that in Rust the data lives **inside** the mutex: you can't touch it without locking. In other languages the lock and the data are separate, and forgetting to lock is a classic bug.

- `lock()` returns `Err` only if another thread **panicked** while holding the lock (the mutex is "poisoned"); `.unwrap()` is common.
- **`RwLock`**: many readers *or* one writer. Good for data read far more than written (configuration, caches).
- Keep critical sections short: lock, copy or change, unlock. Never hold a lock while doing slow I/O.

**Deadlocks** happen when thread A holds lock 1 and waits for lock 2 while thread B holds lock 2 and waits for lock 1. Rust doesn't prevent them. Avoid them by always taking locks in the same order, holding one lock at a time, or using channels instead.

## Atomics: lock-free counters and flags

For a single number or flag, `std::sync::atomic` types (`AtomicUsize`, `AtomicBool`, `AtomicU64`...) are faster than a mutex:

```rust,norun
use std::sync::atomic::{AtomicUsize, Ordering};
static REQUESTS: AtomicUsize = AtomicUsize::new(0);
REQUESTS.fetch_add(1, Ordering::Relaxed);
```

`Ordering` controls how memory operations around the atomic are seen by other threads. `Relaxed` is fine for independent counters; `SeqCst` (the strictest) is a safe default when the atomic guards other data; `Acquire`/`Release` pairs are for experts building their own synchronisation. A shared **cancellation flag** (`AtomicBool`) that workers check in their loop is a common pattern.

## Channels revisited

`std::sync::mpsc` gives many producers and one consumer. Patterns:

- **Fan-in**: many workers send results to one collector (clone `tx` per worker; drop the original so the loop ends).
- **Work queue**: a channel of jobs shared by workers (`Arc<Mutex<Receiver<Job>>>`), the core of a thread pool: this module's project.
- `sync_channel(n)` is **bounded**: senders block when it's full, which gives **backpressure** so a fast producer can't use up all your memory.
- The `crossbeam-channel` crate adds multi-consumer channels and `select!`.

## `OnceLock` and `LazyLock`: initialise once

```rust,norun
use std::sync::LazyLock;
static CONFIG: LazyLock<Config> = LazyLock::new(|| load_config());
```

A global that's computed the first time it's used, safely, even if several threads race to it. `OnceLock` is the version you fill in yourself with `get_or_init`.

## Data parallelism with Rayon

For CPU-heavy work over collections, the **Rayon** crate turns `.iter()` into `.par_iter()` and spreads the work across all cores with a work-stealing scheduler:

```rust,norun
use rayon::prelude::*;
let total: u64 = files.par_iter().map(|f| hash_file(f)).sum();
```

It's usually the easiest big speed-up in Rust. Threads (or Rayon) are for **CPU-bound** work; for thousands of **waiting** tasks (network requests), use async (next lesson).

## Choosing a tool

| Situation | Tool |
|---|---|
| Split work, wait for all, borrow local data | `thread::scope` |
| Pass results or jobs between threads | channels |
| A shared counter or flag | atomics |
| Shared data many threads change | `Arc<Mutex<T>>` |
| Shared data mostly read | `Arc<RwLock<T>>` or `Arc<T>` if immutable |
| Parallel maps and sums over collections | Rayon |
| Thousands of concurrent I/O tasks | async + an executor like Tokio |

@@quiz
Why does `thread::spawn(move || rc.clone())` with an `Rc` fail to compile?
- Threads can't take closures
* `Rc` is not `Send`: its reference count isn't updated atomically, so Rust refuses to share it across threads. Use `Arc`
- `move` closures can't capture anything
- `Rc` is `'static`
? The compiler's message names the missing `Send` bound.

@@quiz
What do scoped threads allow that `thread::spawn` doesn't?
* Borrowing local variables, because all threads are guaranteed to finish before the scope ends
- Running faster
- Returning values
- Sharing an `Rc`
? `thread::spawn` requires `'static` data.

@@quiz
When is the lock of a `Mutex` released?
- When you call `unlock()`
* When the guard returned by `lock()` is dropped
- At the end of the program
- After one second
? Keep guards in small scopes so locks are held briefly.

@@quiz
Which tool fits a simple shared "requests served" counter best?
- `Arc<Mutex<u64>>`
* An `AtomicU64` with `fetch_add`
- A channel
- `RefCell<u64>`
? Atomics avoid locking for single numbers and flags.

@@quiz
What does Rust **not** prevent?
- Data races
* Deadlocks
- Sending an `Rc` to another thread
- Using a `RefCell` from two threads
? Take locks in a consistent order, or prefer channels.

@@drill
Which smart pointer shares ownership across threads?
= Arc
= Arc<T>
= std::sync::Arc
? Combine with Mutex for shared mutable state.

@@exercise Parallel word count
Count word frequencies across many lines using **scoped threads**. `count_words(lines: &[&str], workers: usize) -> BTreeMap<String, usize>`:

1. Split `lines` into `workers` chunks (use `lines.chunks(size)` with `size = lines.len().div_ceil(workers).max(1)`).
2. In a `thread::scope`, spawn one thread per chunk; each builds its own `BTreeMap` counting lowercase words (split on whitespace, trim `.,!?` from each word, skip empty words).
3. Merge all the partial maps into one and return it.

Each worker borrows its chunk; no `Arc` is needed.
@@starter
use std::collections::BTreeMap;
use std::thread;

fn count_words(lines: &[&str], workers: usize) -> BTreeMap<String, usize> {
    todo!()
}

fn main() {
    let lines = vec![
        "The cat sat.",
        "The dog sat!",
        "A cat, a dog?",
        "the end",
    ];
    let counts = count_words(&lines, 3);
    for (word, n) in &counts {
        println!("{word}: {n}");
    }
}
@@expected
a: 2
cat: 2
dog: 2
end: 1
sat: 2
the: 3
@@tests
#[test]
fn checks() {
    assert!(count_words(&[], 4).is_empty());
    let one = count_words(&["Hi hi HI"], 8);
    assert_eq!(one.get("hi"), Some(&3));
    let many: Vec<&str> = vec!["x y"; 100];
    let c = count_words(&many, 7);
    assert_eq!(c.get("x"), Some(&100));
    assert_eq!(c.len(), 2);
}
@@requires
thread::scope
@@hint
Inside `thread::scope(|s| { ... })`, collect handles: `let handles: Vec<_> = lines.chunks(size).map(|chunk| s.spawn(move || { let mut m = BTreeMap::new(); ...; m })).collect();`. Then join each handle and add its counts into the result with `*total.entry(w).or_insert(0) += n;`.
@@solution
use std::collections::BTreeMap;
use std::thread;

fn count_words(lines: &[&str], workers: usize) -> BTreeMap<String, usize> {
    let size = lines.len().div_ceil(workers.max(1)).max(1);
    let mut total = BTreeMap::new();
    thread::scope(|s| {
        let handles: Vec<_> = lines
            .chunks(size)
            .map(|chunk| {
                s.spawn(move || {
                    let mut m: BTreeMap<String, usize> = BTreeMap::new();
                    for line in chunk {
                        for w in line.split_whitespace() {
                            let w = w.trim_matches(|c| ".,!?".contains(c)).to_lowercase();
                            if !w.is_empty() {
                                *m.entry(w).or_insert(0) += 1;
                            }
                        }
                    }
                    m
                })
            })
            .collect();
        for h in handles {
            for (w, n) in h.join().unwrap() {
                *total.entry(w).or_insert(0) += n;
            }
        }
    });
    total
}

fn main() {
    let lines = vec![
        "The cat sat.",
        "The dog sat!",
        "A cat, a dog?",
        "the end",
    ];
    let counts = count_words(&lines, 3);
    for (word, n) in &counts {
        println!("{word}: {n}");
    }
}

@@exercise A shared bank
Several threads transfer money between accounts held in `Arc<Mutex<Vec<i64>>>`. Write:

- `transfer(bank: &Mutex<Vec<i64>>, from: usize, to: usize, amount: i64) -> bool`: lock **once**, and if `from` has at least `amount`, move it and return `true`; otherwise change nothing and return `false`. Doing the check and the update under the same lock is what makes it safe.
- `run(start: Vec<i64>, transfers: Vec<(usize, usize, i64)>) -> (Vec<i64>, usize)`: spawn one thread per transfer (`thread::spawn` with an `Arc` clone), count the successful ones with an `AtomicUsize`, join everything, and return the final balances and the success count.

Whatever order the threads run in, money is never created or destroyed.
@@starter
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};
use std::thread;

fn transfer(bank: &Mutex<Vec<i64>>, from: usize, to: usize, amount: i64) -> bool {
    todo!()
}

fn run(start: Vec<i64>, transfers: Vec<(usize, usize, i64)>) -> (Vec<i64>, usize) {
    todo!()
}

fn main() {
    let transfers = vec![(0, 1, 30), (1, 2, 10), (2, 0, 5), (0, 2, 20)];
    let (balances, ok) = run(vec![100, 50, 10], transfers);
    println!("total {}", balances.iter().sum::<i64>());
    println!("{ok} transfers succeeded");
}
@@expected
total 160
4 transfers succeeded
@@tests
#[test]
fn checks() {
    let bank = Mutex::new(vec![10, 0]);
    assert!(transfer(&bank, 0, 1, 10));
    assert!(!transfer(&bank, 0, 1, 1));
    assert_eq!(*bank.lock().unwrap(), vec![0, 10]);
    let (b, ok) = run(vec![5, 5], vec![(0, 1, 100)]);
    assert_eq!((b, ok), (vec![5, 5], 0));
    let many: Vec<(usize, usize, i64)> = (0..200).map(|i| (i % 2, (i + 1) % 2, 1)).collect();
    let (b, _) = run(vec![50, 50], many);
    assert_eq!(b.iter().sum::<i64>(), 100);
}
@@requires
Arc::clone
AtomicUsize
fetch_add
@@hint
`let mut accounts = bank.lock().unwrap();` then check `accounts[from] >= amount`. In `run`, wrap with `Arc::new(Mutex::new(start))` and `Arc::new(AtomicUsize::new(0))`; in each thread call `transfer(&bank, ...)` and `ok.fetch_add(1, Ordering::Relaxed)` on success. At the end, `bank.lock().unwrap().clone()`.
@@solution
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};
use std::thread;

fn transfer(bank: &Mutex<Vec<i64>>, from: usize, to: usize, amount: i64) -> bool {
    let mut accounts = bank.lock().unwrap();
    if accounts[from] < amount {
        return false;
    }
    accounts[from] -= amount;
    accounts[to] += amount;
    true
}

fn run(start: Vec<i64>, transfers: Vec<(usize, usize, i64)>) -> (Vec<i64>, usize) {
    let bank = Arc::new(Mutex::new(start));
    let ok = Arc::new(AtomicUsize::new(0));
    let mut handles = Vec::new();
    for (from, to, amount) in transfers {
        let bank = Arc::clone(&bank);
        let ok = Arc::clone(&ok);
        handles.push(thread::spawn(move || {
            if transfer(&bank, from, to, amount) {
                ok.fetch_add(1, Ordering::Relaxed);
            }
        }));
    }
    for h in handles {
        h.join().unwrap();
    }
    let balances = bank.lock().unwrap().clone();
    (balances, ok.load(Ordering::Relaxed))
}

fn main() {
    let transfers = vec![(0, 1, 30), (1, 2, 10), (2, 0, 5), (0, 2, 20)];
    let (balances, ok) = run(vec![100, 50, 10], transfers);
    println!("total {}", balances.iter().sum::<i64>());
    println!("{ok} transfers succeeded");
}

@@flashcards
Send :: safe to move to another thread
Sync :: safe to share references between threads
Arc<Mutex<T>> :: shared mutable data across threads
lock() :: returns a guard; unlocks when dropped
RwLock :: many readers or one writer
scoped threads :: thread scope lets threads borrow local data
atomics :: lock-free counters and flags
deadlock :: threads waiting on each other's locks
