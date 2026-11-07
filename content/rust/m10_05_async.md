@@lesson m10-async Async Rust: Futures, async/await and Tokio
@@concepts concurrency, networking
@@text
A web server may hold 10,000 open connections, but each one spends most of its time **waiting**: for the network, a database, a timer. One thread per connection wastes memory (each thread has its own stack, often 2–8 MB). **Async** Rust lets a handful of threads juggle thousands of waiting tasks. It's what powers Tokio, Axum, reqwest, sqlx and most Rust network software.

## Futures: values that will be ready later

An `async fn` doesn't run when you call it. It returns a **future**: a value describing work that can make progress when **polled**:

```rust,norun
async fn fetch_user(id: u32) -> User { ... }

let fut = fetch_user(7);   // nothing has happened yet
let user = fut.await;      // now it runs, pausing at each .await inside
```

The `Future` trait has one method:

```rust,norun
trait Future {
    type Output;
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output>;
}
```

`poll` returns `Poll::Ready(value)` when done or `Poll::Pending` when it must wait. Before returning `Pending`, a future arranges for the **waker** in `cx` to be called when it can make progress (data arrived, timer expired). An **executor** (also called a runtime) keeps a list of tasks, polls the ones that were woken, and sleeps when none are ready.

The compiler turns each `async fn` into a **state machine**: one state per `.await` point, storing the local variables alive at that point. No threads, no stacks: a task can be a few hundred bytes.

## `.await` and cooperative scheduling

`.await` means "poll this future; if it's pending, pause **me** and let the executor run something else". Tasks give up control only at `.await` points: async is **cooperative**. Consequences:

- Never run long CPU work or blocking calls (`std::thread::sleep`, `std::fs::read` of a huge file, a heavy loop) inside async code without yielding: it stalls every other task on that thread. Use `tokio::task::spawn_blocking` or Rayon for CPU work, and async versions of I/O (`tokio::fs`, `tokio::time::sleep`).
- Holding a `std::sync::Mutex` guard across an `.await` can deadlock and makes the future non-`Send`; use `tokio::sync::Mutex` or finish with the lock before awaiting.

## `Pin` in one paragraph

A state machine may contain references to its own fields (a borrow that lives across an `.await`). Moving it in memory would break those references, so futures are **pinned**: `Pin<&mut F>` promises the future won't move again. In everyday code you only meet it as `Box::pin(fut)` (to store futures of different types in a list) or `std::pin::pin!(fut)` (to pin one on the stack).

## Tokio: the standard runtime

```rust,norun
// Cargo.toml: tokio = { version = "1", features = ["full"] }
use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() {
    let a = tokio::spawn(async { sleep(Duration::from_millis(200)).await; "a done" });
    let b = tokio::spawn(async { sleep(Duration::from_millis(100)).await; "b done" });
    println!("{} {}", a.await.unwrap(), b.await.unwrap());   // ~200 ms total, not 300
}
```

Essential tools:

| Tool | Use |
|---|---|
| `#[tokio::main]` | starts the runtime (multi-threaded by default) |
| `tokio::spawn(fut)` | run a task concurrently; returns a `JoinHandle` (the future must be `Send + 'static`) |
| `tokio::join!(a, b)` | run futures concurrently in the same task, wait for all |
| `tokio::select!` | wait for the **first** of several futures (timeouts, cancellation, shutdown signals) |
| `tokio::time::{sleep, timeout, interval}` | timers |
| `tokio::sync::{mpsc, oneshot, broadcast, watch, Mutex, Semaphore}` | async channels and locks; `Semaphore` limits concurrency (e.g. 10 downloads at once) |
| `tokio::net::{TcpListener, TcpStream}`, `tokio::fs` | async I/O |
| `spawn_blocking` | run blocking or CPU-heavy code on a separate thread pool |
| `futures::stream` / `StreamExt` | async iterators: `while let Some(item) = stream.next().await` |

**Cancellation**: dropping a future cancels it at its current `.await`. `select!` drops the losers, and `timeout(dur, fut)` returns an error if the time runs out. Design so that stopping at any `.await` leaves data consistent.

## Async in traits and closures

Since Rust 1.75 you can write `async fn` in traits (`trait Repo { async fn get(&self, id: u32) -> Option<User>; }`), and async closures (`async || { ... }`) are stable since 1.85. Some libraries still use the `async-trait` crate for `dyn` compatibility.

## When to use async

| Workload | Choose |
|---|---|
| Many network connections, I/O-bound services, web servers, crawlers, chat | async (Tokio) |
| CPU-bound number crunching | threads / Rayon |
| A small CLI that makes a few requests | blocking code (`reqwest::blocking`, `ureq`) is simpler |
| GUI apps (egui) | a background thread or a runtime on a separate thread, sending results back with a channel (Module 9) |

## A runtime in 15 lines

To see that there's no magic, the exercises use a tiny executor written with only the standard library. `block_on` polls a future until it's ready, and `yield_now()` is a future that returns `Pending` once, simulating "waiting for I/O":

```rust
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll, Waker};

fn block_on<F: Future>(fut: F) -> F::Output {
    let mut fut = std::pin::pin!(fut);
    let mut cx = Context::from_waker(Waker::noop());
    loop {
        if let Poll::Ready(v) = fut.as_mut().poll(&mut cx) {
            return v;
        }
    }
}

struct YieldNow(bool);
impl Future for YieldNow {
    type Output = ();
    fn poll(mut self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<()> {
        if self.0 { Poll::Ready(()) } else { self.0 = true; Poll::Pending }
    }
}

async fn add_slowly(a: i32, b: i32) -> i32 {
    YieldNow(false).await;   // "wait for the network"
    a + b
}

fn main() {
    println!("{}", block_on(async { add_slowly(2, 3).await * 10 }));
}
```

Real executors don't spin in a loop: they park the thread until a waker fires. But the polling model is exactly the same.

@@quiz
What happens when you call an `async fn` without `.await`ing it?
- It runs immediately on another thread
* It returns a future; nothing runs until the future is polled (awaited or spawned)
- It panics
- It blocks the current thread
? Futures are lazy.

@@quiz
Why is `std::thread::sleep` inside an async task a problem?
* It blocks the whole executor thread, so other tasks on it can't run; use `tokio::time::sleep().await`
- It doesn't compile
- It sleeps for twice as long
- It cancels the task
? Async is cooperative: tasks yield only at `.await`.

@@quiz
What does `tokio::select!` do?
- Runs all futures and waits for every one
* Waits for the first of several futures to complete and cancels (drops) the others
- Chooses the fastest thread
- Selects rows from a database
? Typical uses: timeouts and shutdown signals.

@@quiz
Which workload suits async best?
- Compressing a video
* A server handling thousands of mostly-idle network connections
- Summing a big array
- A one-off script making one HTTP request
? CPU-bound work wants threads or Rayon.

@@drill
Which keyword waits for a future to finish inside an async function?
= .await
= await
? let body = fetch(url).await?;

@@exercise Async functions and errors
Using the provided `block_on` and `yield_now`, write:

- `async fn fetch(url: &str) -> Result<String, String>`: yields once (simulated network wait), then returns `Ok(format!("<page {url}>"))`, or `Err(format!("404 {url}"))` if the url contains `"missing"`.
- `async fn fetch_all(urls: &[&str]) -> Result<Vec<String>, String>`: fetches them one after another with `.await?` and collects the pages; the first error stops it.
- `async fn total_length(urls: &[&str]) -> usize`: fetches each and adds the lengths of successful pages, ignoring errors.
@@starter
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll, Waker};

fn block_on<F: Future>(fut: F) -> F::Output {
    let mut fut = std::pin::pin!(fut);
    let mut cx = Context::from_waker(Waker::noop());
    loop {
        if let Poll::Ready(v) = fut.as_mut().poll(&mut cx) {
            return v;
        }
    }
}

struct YieldNow(bool);
impl Future for YieldNow {
    type Output = ();
    fn poll(mut self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<()> {
        if self.0 { Poll::Ready(()) } else { self.0 = true; Poll::Pending }
    }
}
fn yield_now() -> YieldNow {
    YieldNow(false)
}

async fn fetch(url: &str) -> Result<String, String> {
    todo!()
}

async fn fetch_all(urls: &[&str]) -> Result<Vec<String>, String> {
    todo!()
}

async fn total_length(urls: &[&str]) -> usize {
    todo!()
}

fn main() {
    println!("{:?}", block_on(fetch_all(&["a.com", "b.com"])));
    println!("{:?}", block_on(fetch_all(&["a.com", "missing.com", "c.com"])));
    println!("{}", block_on(total_length(&["a.com", "missing.com", "bb.com"])));
}
@@expected
Ok(["<page a.com>", "<page b.com>"])
Err("404 missing.com")
25
@@tests
#[test]
fn checks() {
    assert_eq!(block_on(fetch("x")), Ok("<page x>".to_string()));
    assert_eq!(block_on(fetch("missing")), Err("404 missing".to_string()));
    assert_eq!(block_on(fetch_all(&[])), Ok(vec![]));
    assert_eq!(block_on(total_length(&[])), 0);
}
@@requires
yield_now().await
.await?
@@hint
`fetch`: `yield_now().await;` then an `if url.contains("missing")`. `fetch_all`: `let mut pages = Vec::new(); for u in urls { pages.push(fetch(u).await?); } Ok(pages)`. `total_length`: `if let Ok(p) = fetch(u).await { total += p.len(); }`.
@@solution
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll, Waker};

fn block_on<F: Future>(fut: F) -> F::Output {
    let mut fut = std::pin::pin!(fut);
    let mut cx = Context::from_waker(Waker::noop());
    loop {
        if let Poll::Ready(v) = fut.as_mut().poll(&mut cx) {
            return v;
        }
    }
}

struct YieldNow(bool);
impl Future for YieldNow {
    type Output = ();
    fn poll(mut self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<()> {
        if self.0 { Poll::Ready(()) } else { self.0 = true; Poll::Pending }
    }
}
fn yield_now() -> YieldNow {
    YieldNow(false)
}

async fn fetch(url: &str) -> Result<String, String> {
    yield_now().await;
    if url.contains("missing") {
        Err(format!("404 {url}"))
    } else {
        Ok(format!("<page {url}>"))
    }
}

async fn fetch_all(urls: &[&str]) -> Result<Vec<String>, String> {
    let mut pages = Vec::new();
    for u in urls {
        pages.push(fetch(u).await?);
    }
    Ok(pages)
}

async fn total_length(urls: &[&str]) -> usize {
    let mut total = 0;
    for u in urls {
        if let Ok(p) = fetch(u).await {
            total += p.len();
        }
    }
    total
}

fn main() {
    println!("{:?}", block_on(fetch_all(&["a.com", "b.com"])));
    println!("{:?}", block_on(fetch_all(&["a.com", "missing.com", "c.com"])));
    println!("{}", block_on(total_length(&["a.com", "missing.com", "bb.com"])));
}

@@exercise A round-robin executor
Concurrency without threads: write `run_all(tasks: Vec<Pin<Box<dyn Future<Output = String>>>>) -> Vec<String>`. Keep the tasks in a list; repeatedly go through the list in order, polling each unfinished task once; when a task returns `Ready(name)`, record it and remove it. Return the names **in the order the tasks finished**.

Each task in `main` logs its steps to a shared log and yields between steps, so you can see the tasks interleave on one thread.
@@starter
use std::cell::RefCell;
use std::future::Future;
use std::pin::Pin;
use std::rc::Rc;
use std::task::{Context, Poll, Waker};

struct YieldNow(bool);
impl Future for YieldNow {
    type Output = ();
    fn poll(mut self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<()> {
        if self.0 { Poll::Ready(()) } else { self.0 = true; Poll::Pending }
    }
}
fn yield_now() -> YieldNow {
    YieldNow(false)
}

fn run_all(tasks: Vec<Pin<Box<dyn Future<Output = String>>>>) -> Vec<String> {
    todo!()
}

async fn worker(name: &'static str, steps: u32, log: Rc<RefCell<Vec<String>>>) -> String {
    for i in 1..=steps {
        log.borrow_mut().push(format!("{name}{i}"));
        yield_now().await;
    }
    name.to_string()
}

fn main() {
    let log = Rc::new(RefCell::new(Vec::new()));
    let tasks: Vec<Pin<Box<dyn Future<Output = String>>>> = vec![
        Box::pin(worker("a", 3, log.clone())),
        Box::pin(worker("b", 1, log.clone())),
        Box::pin(worker("c", 2, log.clone())),
    ];
    let finished = run_all(tasks);
    println!("{}", log.borrow().join(" "));
    println!("{:?}", finished);
}
@@expected
a1 b1 c1 a2 c2 a3
["b", "c", "a"]
@@tests
#[test]
fn checks() {
    assert!(run_all(vec![]).is_empty());
    let ready: Vec<Pin<Box<dyn Future<Output = String>>>> = vec![Box::pin(async { "x".to_string() }), Box::pin(async { "y".to_string() })];
    assert_eq!(run_all(ready), vec!["x".to_string(), "y".to_string()]);
}
@@requires
Context::from_waker
.poll(
@@hint
`let mut cx = Context::from_waker(Waker::noop());`. Loop `while !tasks.is_empty()`, and inside, walk the list with an index `i`: `match tasks[i].as_mut().poll(&mut cx) { Poll::Ready(name) => { done.push(name); tasks.remove(i); } Poll::Pending => i += 1 }`.
@@solution
use std::cell::RefCell;
use std::future::Future;
use std::pin::Pin;
use std::rc::Rc;
use std::task::{Context, Poll, Waker};

struct YieldNow(bool);
impl Future for YieldNow {
    type Output = ();
    fn poll(mut self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<()> {
        if self.0 { Poll::Ready(()) } else { self.0 = true; Poll::Pending }
    }
}
fn yield_now() -> YieldNow {
    YieldNow(false)
}

fn run_all(mut tasks: Vec<Pin<Box<dyn Future<Output = String>>>>) -> Vec<String> {
    let mut cx = Context::from_waker(Waker::noop());
    let mut done = Vec::new();
    while !tasks.is_empty() {
        let mut i = 0;
        while i < tasks.len() {
            match tasks[i].as_mut().poll(&mut cx) {
                Poll::Ready(name) => {
                    done.push(name);
                    tasks.remove(i);
                }
                Poll::Pending => i += 1,
            }
        }
    }
    done
}

async fn worker(name: &'static str, steps: u32, log: Rc<RefCell<Vec<String>>>) -> String {
    for i in 1..=steps {
        log.borrow_mut().push(format!("{name}{i}"));
        yield_now().await;
    }
    name.to_string()
}

fn main() {
    let log = Rc::new(RefCell::new(Vec::new()));
    let tasks: Vec<Pin<Box<dyn Future<Output = String>>>> = vec![
        Box::pin(worker("a", 3, log.clone())),
        Box::pin(worker("b", 1, log.clone())),
        Box::pin(worker("c", 2, log.clone())),
    ];
    let finished = run_all(tasks);
    println!("{}", log.borrow().join(" "));
    println!("{:?}", finished);
}

@@flashcards
async fn :: returns a Future
.await :: wait for a future without blocking the thread
runtime :: tokio executes futures
tokio main attribute :: #[tokio::main] starts a runtime for main
tokio spawn :: run a task concurrently
join! :: wait for several futures
async for I/O :: many network connections; threads for CPU work
