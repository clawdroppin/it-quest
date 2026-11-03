@@lesson m10-threadpool Project: A Thread Pool and Job Queue
@@text
This project combines the whole module: trait objects (boxed closures), `Send` and `'static` bounds, `Arc<Mutex<...>>`, channels, `Drop` for clean shutdown, and generics. You'll build the kind of **thread pool** that sits inside web servers, build systems and image processors: a fixed number of worker threads pulling jobs from a shared queue.

## Why a pool?

Spawning a thread per task is fine for ten tasks but not for ten thousand: each thread costs memory and start-up time, and running thousands at once thrashes the CPU. A pool keeps **N workers** (usually the number of CPU cores, `std::thread::available_parallelism()`) and a **queue** of jobs. Workers loop: take a job, run it, repeat.

## The design

```text
              execute(job)                      ┌─► Worker 0 (thread): loop { lock rx, recv job, unlock, run job }
  caller ───► Sender<Job> ──► channel ──► Arc<Mutex<Receiver<Job>>> ─► Worker 1
                                                └─► Worker 2
```

- A **job** is any closure that can run once on another thread: `type Job = Box<dyn FnOnce() + Send + 'static>;`
  - `FnOnce`: run once; `Send`: may move to another thread; `'static`: owns its data.
- The pool holds the `Sender<Job>`; all workers share **one** `Receiver` behind `Arc<Mutex<...>>` (mpsc receivers can't be cloned).
- **Important detail**: a worker must release the lock **before** running the job, or only one job would run at a time:

```rust,norun
let job = receiver.lock().unwrap().recv();   // the guard is dropped at the end of this statement
match job {
    Ok(job) => job(),                          // run without holding the lock
    Err(_) => break,                           // channel closed: shut down
}
```

## Graceful shutdown with `Drop`

When the pool is dropped:

1. Drop the `Sender` (store it as `Option<Sender<Job>>` and `take()` it). The channel closes once the queue is empty, so each worker's `recv()` returns `Err` and its loop ends.
2. **Join** every worker thread so the program doesn't exit while jobs are still running. Store handles as `Option<JoinHandle<()>>` and `take()` them, because `join` consumes the handle.

The result: `drop(pool)` waits for every queued job to finish. That's RAII again: cleanup happens automatically when the owner goes away.

## Getting results back

`execute` returns nothing. To get results, jobs send them on a second channel, or, more conveniently, a generic helper wraps that pattern: `map_parallel(&pool, items, f)` sends each item's index with its result, then puts the results back in the original order.

## Where to go from here

Production pools add: work stealing (Rayon), per-worker queues, panics that don't kill workers (`catch_unwind`), bounded queues for backpressure, and metrics. Tokio's runtime is, at its heart, a thread pool that runs futures instead of closures.

@@quiz
Why must a worker drop the mutex guard before running the job?
* Otherwise it would hold the queue's lock while working, and no other worker could take a job: only one job would run at a time
- Mutexes can't be held for more than one second
- Jobs can't run while locked
- It doesn't matter
? `let job = rx.lock().unwrap().recv();` drops the temporary guard at the end of the statement.

@@quiz
Why does a `Job` need the `Send` bound?
- So it can be printed
* It's created on one thread and run on another
- So it can be cloned
- Because closures are always `Send`
? `'static` is needed too: the job must own its data.

@@quiz
How do workers know to stop when the pool is dropped?
- A global flag
* The pool drops its `Sender`; once the queue is empty, `recv()` returns an error and each worker's loop ends
- The OS kills the threads
- They check the time
? Then `Drop` joins every worker.

@@exercise Build the thread pool
Complete `ThreadPool`:

- `new(size)` creates the channel, wraps the receiver in `Arc<Mutex<_>>`, and spawns `size` workers (panic if `size == 0`).
- Each worker thread loops: lock, `recv()`, release the lock, run the job; stop when `recv()` returns `Err`.
- `execute(f)` boxes the closure and sends it.
- `Drop`: take and drop the sender, then take and join every handle.
@@starter
use std::sync::mpsc::{self, Receiver, Sender};
use std::sync::{Arc, Mutex};
use std::thread::{self, JoinHandle};

type Job = Box<dyn FnOnce() + Send + 'static>;

struct ThreadPool {
    workers: Vec<Option<JoinHandle<()>>>,
    sender: Option<Sender<Job>>,
}

impl ThreadPool {
    fn new(size: usize) -> ThreadPool {
        todo!()
    }

    fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        todo!()
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        todo!()
    }
}

fn main() {
    let (tx, rx) = mpsc::channel();
    let pool = ThreadPool::new(3);
    for i in 1..=6u64 {
        let tx = tx.clone();
        pool.execute(move || {
            tx.send(i * i).unwrap();
        });
    }
    drop(tx);
    drop(pool); // waits for every job
    let mut results: Vec<u64> = rx.iter().collect();
    results.sort();
    println!("{:?}", results);
}
@@expected
[1, 4, 9, 16, 25, 36]
@@tests
#[test]
fn checks() {
    use std::sync::atomic::{AtomicUsize, Ordering};
    let counter = Arc::new(AtomicUsize::new(0));
    {
        let pool = ThreadPool::new(4);
        assert_eq!(pool.workers.len(), 4);
        for _ in 0..100 {
            let c = Arc::clone(&counter);
            pool.execute(move || {
                c.fetch_add(1, Ordering::SeqCst);
            });
        }
    } // dropped here: must wait for all 100 jobs
    assert_eq!(counter.load(Ordering::SeqCst), 100);
}
@@requires
Arc<Mutex<Receiver<Job>>>
impl Drop for ThreadPool
@@hint
In `new`: `let (sender, receiver) = mpsc::channel::<Job>(); let receiver = Arc::new(Mutex::new(receiver));` then for each worker `let rx = Arc::clone(&receiver); thread::spawn(move || loop { let job = rx.lock().unwrap().recv(); match job { Ok(job) => job(), Err(_) => break } })`. In `drop`: `drop(self.sender.take());` then `for w in &mut self.workers { if let Some(h) = w.take() { h.join().unwrap(); } }`.
@@solution
use std::sync::mpsc::{self, Receiver, Sender};
use std::sync::{Arc, Mutex};
use std::thread::{self, JoinHandle};

type Job = Box<dyn FnOnce() + Send + 'static>;

struct ThreadPool {
    workers: Vec<Option<JoinHandle<()>>>,
    sender: Option<Sender<Job>>,
}

impl ThreadPool {
    fn new(size: usize) -> ThreadPool {
        assert!(size > 0, "a pool needs at least one worker");
        let (sender, receiver) = mpsc::channel::<Job>();
        let receiver: Arc<Mutex<Receiver<Job>>> = Arc::new(Mutex::new(receiver));
        let mut workers = Vec::with_capacity(size);
        for _ in 0..size {
            let rx = Arc::clone(&receiver);
            workers.push(Some(thread::spawn(move || loop {
                let job = rx.lock().unwrap().recv();
                match job {
                    Ok(job) => job(),
                    Err(_) => break,
                }
            })));
        }
        ThreadPool { workers, sender: Some(sender) }
    }

    fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        self.sender.as_ref().unwrap().send(Box::new(f)).unwrap();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        drop(self.sender.take());
        for w in &mut self.workers {
            if let Some(h) = w.take() {
                h.join().unwrap();
            }
        }
    }
}

fn main() {
    let (tx, rx) = mpsc::channel();
    let pool = ThreadPool::new(3);
    for i in 1..=6u64 {
        let tx = tx.clone();
        pool.execute(move || {
            tx.send(i * i).unwrap();
        });
    }
    drop(tx);
    drop(pool); // waits for every job
    let mut results: Vec<u64> = rx.iter().collect();
    results.sort();
    println!("{:?}", results);
}

@@exercise Ordered parallel map
Using a ready-made pool, write a generic `map_parallel<T, R, F>(pool: &ThreadPool, items: Vec<T>, f: F) -> Vec<R>` that runs `f` on every item in the pool and returns the results **in the original order**:

1. Wrap `f` in an `Arc` so every job can share it.
2. For each `(index, item)`, execute a job that sends `(index, f(item))` on a channel.
3. Drop your copy of the sender, collect all `(index, result)` pairs, sort by index, and keep the results.

Work out the bounds: `T: Send + 'static`, `R: Send + 'static`, `F: Fn(T) -> R + Send + Sync + 'static`.
@@starter
use std::sync::mpsc::{self, Receiver, Sender};
use std::sync::{Arc, Mutex};
use std::thread::{self, JoinHandle};

type Job = Box<dyn FnOnce() + Send + 'static>;

struct ThreadPool {
    workers: Vec<Option<JoinHandle<()>>>,
    sender: Option<Sender<Job>>,
}

impl ThreadPool {
    fn new(size: usize) -> ThreadPool {
        let (sender, receiver) = mpsc::channel::<Job>();
        let receiver: Arc<Mutex<Receiver<Job>>> = Arc::new(Mutex::new(receiver));
        let workers = (0..size)
            .map(|_| {
                let rx = Arc::clone(&receiver);
                Some(thread::spawn(move || loop {
                    let job = rx.lock().unwrap().recv();
                    match job {
                        Ok(job) => job(),
                        Err(_) => break,
                    }
                }))
            })
            .collect();
        ThreadPool { workers, sender: Some(sender) }
    }

    fn execute<F: FnOnce() + Send + 'static>(&self, f: F) {
        self.sender.as_ref().unwrap().send(Box::new(f)).unwrap();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        drop(self.sender.take());
        for w in &mut self.workers {
            if let Some(h) = w.take() {
                h.join().unwrap();
            }
        }
    }
}

fn map_parallel<T, R, F>(pool: &ThreadPool, items: Vec<T>, f: F) -> Vec<R>
where
    T: Send + 'static,
    R: Send + 'static,
    F: Fn(T) -> R + Send + Sync + 'static,
{
    todo!()
}

fn main() {
    let pool = ThreadPool::new(4);
    let words = vec!["thread", "pool", "map", "rust"];
    let lens = map_parallel(&pool, words, |w| format!("{w}:{}", w.len()));
    println!("{:?}", lens);
    let squares = map_parallel(&pool, (1..=10).collect(), |n: u64| n * n);
    println!("{:?}", squares);
}
@@expected
["thread:6", "pool:4", "map:3", "rust:4"]
[1, 4, 9, 16, 25, 36, 49, 64, 81, 100]
@@tests
#[test]
fn checks() {
    let pool = ThreadPool::new(3);
    let empty: Vec<i32> = map_parallel(&pool, Vec::<i32>::new(), |x| x);
    assert!(empty.is_empty());
    let slow_first = map_parallel(&pool, vec![30u64, 0, 10], |ms| {
        thread::sleep(std::time::Duration::from_millis(ms));
        ms
    });
    assert_eq!(slow_first, vec![30, 0, 10], "results must be in input order, not finishing order");
}
@@requires
Arc::new(f)
sort_by_key
@@hint
`let f = Arc::new(f); let (tx, rx) = mpsc::channel(); for (i, item) in items.into_iter().enumerate() { let tx = tx.clone(); let f = Arc::clone(&f); pool.execute(move || { tx.send((i, f(item))).unwrap(); }); } drop(tx); let mut out: Vec<(usize, R)> = rx.iter().collect(); out.sort_by_key(|(i, _)| *i); out.into_iter().map(|(_, r)| r).collect()`.
@@solution
use std::sync::mpsc::{self, Receiver, Sender};
use std::sync::{Arc, Mutex};
use std::thread::{self, JoinHandle};

type Job = Box<dyn FnOnce() + Send + 'static>;

struct ThreadPool {
    workers: Vec<Option<JoinHandle<()>>>,
    sender: Option<Sender<Job>>,
}

impl ThreadPool {
    fn new(size: usize) -> ThreadPool {
        let (sender, receiver) = mpsc::channel::<Job>();
        let receiver: Arc<Mutex<Receiver<Job>>> = Arc::new(Mutex::new(receiver));
        let workers = (0..size)
            .map(|_| {
                let rx = Arc::clone(&receiver);
                Some(thread::spawn(move || loop {
                    let job = rx.lock().unwrap().recv();
                    match job {
                        Ok(job) => job(),
                        Err(_) => break,
                    }
                }))
            })
            .collect();
        ThreadPool { workers, sender: Some(sender) }
    }

    fn execute<F: FnOnce() + Send + 'static>(&self, f: F) {
        self.sender.as_ref().unwrap().send(Box::new(f)).unwrap();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        drop(self.sender.take());
        for w in &mut self.workers {
            if let Some(h) = w.take() {
                h.join().unwrap();
            }
        }
    }
}

fn map_parallel<T, R, F>(pool: &ThreadPool, items: Vec<T>, f: F) -> Vec<R>
where
    T: Send + 'static,
    R: Send + 'static,
    F: Fn(T) -> R + Send + Sync + 'static,
{
    let f = Arc::new(f);
    let (tx, rx) = mpsc::channel();
    for (i, item) in items.into_iter().enumerate() {
        let tx = tx.clone();
        let f = Arc::clone(&f);
        pool.execute(move || {
            tx.send((i, f(item))).unwrap();
        });
    }
    drop(tx);
    let mut out: Vec<(usize, R)> = rx.iter().collect();
    out.sort_by_key(|(i, _)| *i);
    out.into_iter().map(|(_, r)| r).collect()
}

fn main() {
    let pool = ThreadPool::new(4);
    let words = vec!["thread", "pool", "map", "rust"];
    let lens = map_parallel(&pool, words, |w| format!("{w}:{}", w.len()));
    println!("{:?}", lens);
    let squares = map_parallel(&pool, (1..=10).collect(), |n: u64| n * n);
    println!("{:?}", squares);
}
