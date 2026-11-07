@@lesson m11-testing Advanced Testing: Integration Tests, Test Doubles, Properties and Fuzzing
@@concepts testing
@@text
You already know `#[test]`, `assert_eq!` and `#[cfg(test)] mod tests`. Professional Rust projects go further: tests at several levels, fakes for the outside world, tests that generate their own inputs, and tools that hunt for crashes. This lesson covers that toolbox.

## The three kinds of tests Cargo knows

| Kind | Where | Sees | Run with |
|---|---|---|---|
| **Unit tests** | `#[cfg(test)] mod tests` inside `src/` | private items too | `cargo test` |
| **Integration tests** | each file in `tests/` is its own crate | only your **public** API | `cargo test --test api` |
| **Doc tests** | ```` ``` ```` code blocks in `///` comments | public API, like a user | `cargo test --doc` |

```rust,norun
/// Adds VAT to a price in cents.
///
/// ```
/// assert_eq!(shop::with_vat(1000, 21), 1210);
/// ```
pub fn with_vat(cents: u64, percent: u64) -> u64 { cents + cents * percent / 100 }
```

Doc tests keep examples honest: if the API changes, the docs fail to compile. Shared helpers for integration tests go in `tests/common/mod.rs`. Binary crates are hard to integration-test, so keep a thin `main.rs` and put logic in `lib.rs`.

Useful attributes: `#[should_panic(expected = "overflow")]`, `#[ignore]` (slow tests, run with `--ignored`), tests returning `Result<(), Box<dyn Error>>` so you can use `?`.

## Test doubles: faking the outside world

Code that talks to the clock, the network, the file system or randomness is hard to test. Hide each behind a **trait** and pass in a fake during tests:

| Double | What it does |
|---|---|
| **Stub** | returns canned answers (`FixedClock` always says 09:00) |
| **Fake** | a working lightweight version (in-memory repository) |
| **Spy** | records the calls it received so the test can inspect them |
| **Mock** | has expectations set up front and fails if they aren't met (`mockall` crate) |

Hand-written fakes are usually clearer than mock libraries. The second exercise builds a stub clock and a spy mailer.

## Property-based testing

Example-based tests check the inputs you thought of. **Property tests** state a rule that must hold for *every* input, then generate hundreds of random inputs:

- sorting returns the same length and an ordered result
- `decode(encode(x)) == x` (round trip)
- the fast implementation equals the simple one (oracle)
- adding an item then removing it leaves the collection unchanged

```rust,norun
use proptest::prelude::*;

proptest! {
    #[test]
    fn roundtrip(s in ".*") {
        prop_assert_eq!(decode(&encode(&s)), s);
    }
}
```

When a property fails, proptest and quickcheck **shrink** the input: they keep simplifying it while it still fails, so you get `[3, 3]` instead of a 40-element random vector. The first exercise builds a tiny property tester with a deterministic random number generator and shrinking.

## Snapshot testing

For large outputs (formatted reports, generated code, JSON), **insta** stores the expected output in a file the first time and diffs against it later: `insta::assert_snapshot!(render(&report));`. Review changes with `cargo insta review`.

## Fuzzing

A **fuzzer** feeds random, mutated bytes to a function for hours, guided by code coverage, looking for panics, overflows and hangs. It's how most parser bugs are found.

```rust,norun
// fuzz/fuzz_targets/parse.rs  (cargo install cargo-fuzz; cargo +nightly fuzz run parse)
#![no_main]
libfuzzer_sys::fuzz_target!(|data: &[u8]| {
    if let Ok(s) = std::str::from_utf8(data) {
        let _ = mylib::parse_config(s);   // must never panic
    }
});
```

Anything that parses untrusted input (files, network packets, user text) deserves a fuzz target.

## More tools

- **cargo nextest**: faster parallel runner, clear output, retries for flaky tests.
- **cargo llvm-cov**: line and branch coverage; aim for confidence, not 100%.
- **Miri**: runs tests in an interpreter that detects undefined behaviour in `unsafe` code.
- **loom**: explores every interleaving of a concurrent algorithm.
- **criterion / divan**: benchmarks (see the performance lesson).
- **testcontainers**: start a real Postgres in Docker for integration tests.

## What makes a good test

1. **Fast and deterministic**: no real clock, network or random seed without control.
2. **One reason to fail**, with a name that says what behaviour broke.
3. Tests behaviour through the public interface, not private details, so refactoring doesn't break it.
4. Arrange / Act / Assert structure.
5. Every bug fixed gets a regression test first.

@@quiz
Where do integration tests live and what can they access?
- In src/, with access to private functions
* In the tests/ folder, each file a separate crate using only the public API
- In Cargo.toml
- In doc comments only
? They test the crate the way a user would.

@@quiz
What does shrinking do in property-based testing?
- Compresses test files
* Simplifies a failing random input step by step while it still fails, to give a minimal counterexample
- Removes slow tests
- Reduces binary size
? A minimal input makes the bug obvious.

@@quiz
A test double that records the calls it receives so the test can check them is called a…
- Stub
* Spy
- Fixture
- Fuzzer
? Stubs return canned answers; spies record.

@@quiz
Which kind of code most needs a fuzz target?
- Getters and setters
* Parsers of untrusted input such as files and network data
- Constants
- Unit tests
? Fuzzers find inputs that make parsers panic or hang.

@@drill
Which folder holds integration tests in a Cargo project?
= tests
= tests/
? Each file there is compiled as its own crate.

@@flashcards
Unit test :: a test inside `src/` in `#[cfg(test)] mod tests`, can see private items
Integration test :: a file in `tests/`, compiled as a separate crate against the public API
Doc test :: a code example in a `///` comment that `cargo test` compiles and runs
Stub :: a test double that returns fixed answers
Spy :: a test double that records the calls it received
Property test :: a test that checks a rule over many generated inputs
Shrinking :: reducing a failing input to a minimal one
Fuzzing :: feeding mutated random input to find crashes and hangs

@@exercise A tiny property tester
Build the core of a property-testing library.

- `XorShift` is a deterministic random number generator. `next(&mut self) -> u64` updates the state with `x ^= x << 13; x ^= x >> 7; x ^= x << 17;` and returns the new state. `below(&mut self, n: u64) -> u64` returns `next() % n`.
- `random_vec(rng, max_len, max_value) -> Vec<u32>`: a length `below(max_len + 1)`, then that many values `below(max_value + 1)`.
- `shrink(input, fails) -> Vec<u32>`: repeatedly try removing one element at a time; whenever the shorter vector still fails, keep it and start again. Stop when no single removal still fails.
- `find_counterexample(seed, cases, prop) -> Option<Vec<u32>>`: generate `cases` vectors (max length 8, values 0..=9); for the first one where `prop` returns false, return the shrunk vector. `None` if all pass.

`main` tests a buggy "sort" that also removes duplicates.
@@starter
struct XorShift {
    state: u64,
}

impl XorShift {
    fn next(&mut self) -> u64 {
        todo!()
    }

    fn below(&mut self, n: u64) -> u64 {
        todo!()
    }
}

fn random_vec(rng: &mut XorShift, max_len: u64, max_value: u64) -> Vec<u32> {
    todo!()
}

fn shrink(input: Vec<u32>, fails: &dyn Fn(&[u32]) -> bool) -> Vec<u32> {
    todo!()
}

fn find_counterexample(seed: u64, cases: usize, prop: &dyn Fn(&[u32]) -> bool) -> Option<Vec<u32>> {
    todo!()
}

fn buggy_sort(v: &[u32]) -> Vec<u32> {
    let mut out = v.to_vec();
    out.sort();
    out.dedup();
    out
}

fn main() {
    let keeps_length = |v: &[u32]| buggy_sort(v).len() == v.len();
    let found = find_counterexample(42, 200, &keeps_length).unwrap();
    println!("counterexample has {} elements", found.len());
    println!("elements equal: {}", found[0] == found[1]);
    let is_sorted = |v: &[u32]| buggy_sort(v).windows(2).all(|w| w[0] <= w[1]);
    println!("{:?}", find_counterexample(42, 200, &is_sorted));
}
@@expected
counterexample has 2 elements
elements equal: true
None
@@tests
#[test]
fn checks() {
    let mut r = XorShift { state: 1 };
    assert_eq!(r.next(), 1082269761);
    let mut a = XorShift { state: 7 };
    let mut b = XorShift { state: 7 };
    assert_eq!(random_vec(&mut a, 8, 9), random_vec(&mut b, 8, 9), "same seed, same data");
    let v = random_vec(&mut a, 3, 5);
    assert!(v.len() <= 3 && v.iter().all(|&x| x <= 5));
    let has_big = |v: &[u32]| !v.iter().any(|&x| x >= 100);
    assert_eq!(shrink(vec![1, 150, 2, 300], &|v| !has_big(v)), vec![300]);
}
@@requires
fn shrink
cur.clone(
@@hint
`next`: `let mut x = self.state; x ^= x << 13; x ^= x >> 7; x ^= x << 17; self.state = x; x`. `shrink`: `let mut cur = input; 'outer: loop { for i in 0..cur.len() { let mut c = cur.clone(); c.remove(i); if fails(&c) { cur = c; continue 'outer; } } return cur; }`. In `find_counterexample`, call `shrink(v, &|x| !prop(x))`.
@@solution
struct XorShift {
    state: u64,
}

impl XorShift {
    fn next(&mut self) -> u64 {
        let mut x = self.state;
        x ^= x << 13;
        x ^= x >> 7;
        x ^= x << 17;
        self.state = x;
        x
    }

    fn below(&mut self, n: u64) -> u64 {
        self.next() % n
    }
}

fn random_vec(rng: &mut XorShift, max_len: u64, max_value: u64) -> Vec<u32> {
    let len = rng.below(max_len + 1);
    (0..len).map(|_| rng.below(max_value + 1) as u32).collect()
}

fn shrink(input: Vec<u32>, fails: &dyn Fn(&[u32]) -> bool) -> Vec<u32> {
    let mut cur = input;
    'outer: loop {
        for i in 0..cur.len() {
            let mut c = cur.clone();
            c.remove(i);
            if fails(&c) {
                cur = c;
                continue 'outer;
            }
        }
        return cur;
    }
}

fn find_counterexample(seed: u64, cases: usize, prop: &dyn Fn(&[u32]) -> bool) -> Option<Vec<u32>> {
    let mut rng = XorShift { state: seed };
    for _ in 0..cases {
        let v = random_vec(&mut rng, 8, 9);
        if !prop(&v) {
            return Some(shrink(v, &|x| !prop(x)));
        }
    }
    None
}

fn buggy_sort(v: &[u32]) -> Vec<u32> {
    let mut out = v.to_vec();
    out.sort();
    out.dedup();
    out
}

fn main() {
    let keeps_length = |v: &[u32]| buggy_sort(v).len() == v.len();
    let found = find_counterexample(42, 200, &keeps_length).unwrap();
    println!("counterexample has {} elements", found.len());
    println!("elements equal: {}", found[0] == found[1]);
    let is_sorted = |v: &[u32]| buggy_sort(v).windows(2).all(|w| w[0] <= w[1]);
    println!("{:?}", find_counterexample(42, 200, &is_sorted));
}

@@exercise Stubs and spies
`send_reminders` emails the owner of every task that is not done and is due within the next 24 hours (due time `>= now` and `<= now + 86400`). It depends on a `Clock` and a `Mailer`, so it can be tested without real time or real email.

- Implement `FixedClock(u64)` (a stub: `now` returns the stored value).
- Implement `SpyMailer` (a spy: `send` pushes `"to: subject"` into `sent` and returns `Ok(())`, except it returns `Err` for addresses without `@`).
- Implement `send_reminders(clock, mailer, tasks) -> (usize, usize)` returning (sent, failed). The subject is `Reminder: <title>`.
@@starter
trait Clock {
    fn now(&self) -> u64;
}

trait Mailer {
    fn send(&mut self, to: &str, subject: &str) -> Result<(), String>;
}

struct Task {
    title: &'static str,
    owner: &'static str,
    due: u64,
    done: bool,
}

struct FixedClock(u64);
struct SpyMailer {
    sent: Vec<String>,
}

// impl Clock for FixedClock ...
// impl Mailer for SpyMailer ...

fn send_reminders(clock: &dyn Clock, mailer: &mut dyn Mailer, tasks: &[Task]) -> (usize, usize) {
    todo!()
}

fn main() {
    let tasks = [
        Task { title: "Renew TLS cert", owner: "ops@corp.example", due: 1_000_500, done: false },
        Task { title: "Patch servers", owner: "maya@corp.example", due: 1_090_000, done: false },
        Task { title: "Old ticket", owner: "li@corp.example", due: 999_000, done: false },
        Task { title: "Backups", owner: "broken-address", due: 1_010_000, done: false },
        Task { title: "Audit", owner: "sam@corp.example", due: 1_000_100, done: true },
    ];
    let mut spy = SpyMailer { sent: vec![] };
    let result = send_reminders(&FixedClock(1_000_000), &mut spy, &tasks);
    println!("{result:?}");
    for line in &spy.sent {
        println!("{line}");
    }
}
@@expected
(1, 1)
ops@corp.example: Reminder: Renew TLS cert
@@tests
#[test]
fn checks() {
    let t = [Task { title: "x", owner: "a@b", due: 86_400, done: false }];
    let mut spy = SpyMailer { sent: vec![] };
    assert_eq!(send_reminders(&FixedClock(0), &mut spy, &t), (1, 0), "exactly 24h is included");
    let mut spy = SpyMailer { sent: vec![] };
    assert_eq!(send_reminders(&FixedClock(1), &mut spy, &t), (1, 0));
    let mut spy = SpyMailer { sent: vec![] };
    assert_eq!(send_reminders(&FixedClock(86_401), &mut spy, &t), (0, 0), "overdue is skipped");
    assert!(spy.sent.is_empty());
}
@@requires
impl Mailer for SpyMailer
.send(
@@hint
Filter with `!t.done && t.due >= now && t.due <= now + 86_400`, then `match mailer.send(t.owner, &format!("Reminder: {}", t.title)) { Ok(()) => sent += 1, Err(_) => failed += 1 }`. The spy only records successful sends.
@@solution
trait Clock {
    fn now(&self) -> u64;
}

trait Mailer {
    fn send(&mut self, to: &str, subject: &str) -> Result<(), String>;
}

struct Task {
    title: &'static str,
    owner: &'static str,
    due: u64,
    done: bool,
}

struct FixedClock(u64);
struct SpyMailer {
    sent: Vec<String>,
}

impl Clock for FixedClock {
    fn now(&self) -> u64 {
        self.0
    }
}

impl Mailer for SpyMailer {
    fn send(&mut self, to: &str, subject: &str) -> Result<(), String> {
        if !to.contains('@') {
            return Err(format!("invalid address: {to}"));
        }
        self.sent.push(format!("{to}: {subject}"));
        Ok(())
    }
}

fn send_reminders(clock: &dyn Clock, mailer: &mut dyn Mailer, tasks: &[Task]) -> (usize, usize) {
    let now = clock.now();
    let (mut sent, mut failed) = (0, 0);
    for t in tasks.iter().filter(|t| !t.done && t.due >= now && t.due <= now + 86_400) {
        match mailer.send(t.owner, &format!("Reminder: {}", t.title)) {
            Ok(()) => sent += 1,
            Err(_) => failed += 1,
        }
    }
    (sent, failed)
}

fn main() {
    let tasks = [
        Task { title: "Renew TLS cert", owner: "ops@corp.example", due: 1_000_500, done: false },
        Task { title: "Patch servers", owner: "maya@corp.example", due: 1_090_000, done: false },
        Task { title: "Old ticket", owner: "li@corp.example", due: 999_000, done: false },
        Task { title: "Backups", owner: "broken-address", due: 1_010_000, done: false },
        Task { title: "Audit", owner: "sam@corp.example", due: 1_000_100, done: true },
    ];
    let mut spy = SpyMailer { sent: vec![] };
    let result = send_reminders(&FixedClock(1_000_000), &mut spy, &tasks);
    println!("{result:?}");
    for line in &spy.sent {
        println!("{line}");
    }
}
