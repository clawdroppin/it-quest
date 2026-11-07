@@lesson m10-performance Performance: Measuring and Making Rust Fast
@@text
Rust is fast by default, but "fast by default" still loses to a better algorithm, and a few habits (needless cloning, allocating in loops, debug builds) can make Rust slower than Python. This lesson is a practical method: measure, find the hot spot, fix the biggest cost, measure again.

## Rule zero: build in release mode

`cargo run` builds a **debug** build: no optimisation, overflow checks, full debug info. It can be **10–100× slower** than `cargo run --release`. Never judge speed, or benchmark, in debug mode. Tune release builds in `Cargo.toml`:

```text
[profile.release]
lto = "fat"          # link-time optimisation across crates
codegen-units = 1    # better optimisation, slower compile
panic = "abort"      # smaller binaries, no unwinding
strip = true         # remove symbols
```

`RUSTFLAGS="-C target-cpu=native"` lets the compiler use every instruction your CPU supports (only for binaries that run on the same machine type).

## Measure first

- **Timing**: `let t = std::time::Instant::now(); ...; println!("{:?}", t.elapsed());` for rough numbers.
- **Benchmarks**: **criterion** (or the newer **divan**) runs code many times, handles warm-up and statistical noise, and reports changes between runs. Put benchmarks in `benches/` and run `cargo bench`. Use `std::hint::black_box(x)` so the optimiser can't delete the work you're measuring.
- **Profilers** show *where* time goes: `cargo flamegraph` (Linux perf / Windows ETW / macOS dtrace) produces a flame graph; Windows users also have Visual Studio's profiler and Superluminal; `samply` works across platforms and opens in the Firefox Profiler.
- **Memory**: `dhat` or `heaptrack` count allocations.

Optimise the function at the top of the profile, not the one you suspect.

## Algorithms beat micro-tuning

The biggest wins are almost always **algorithmic**:

| Task | Slow | Fast |
|---|---|---|
| "Is x in this list?" repeated n times | `Vec::contains`, O(n) each: O(n²) total | `HashSet`, O(1) each |
| Finding duplicates | nested loops, O(n²) | sort then scan, or a set: O(n log n) / O(n) |
| Repeated sub-problems (paths, prices, edit distance) | plain recursion: exponential | **memoisation** or dynamic programming |
| Range sums asked many times | re-summing: O(n) per query | **prefix sums**: O(1) per query |
| Top 10 of a million | sort everything | `BinaryHeap` of size 10, or `select_nth_unstable` |

## Allocation: the most common Rust slowdown

- **Pre-allocate**: `Vec::with_capacity(n)`, `String::with_capacity(n)`, `HashMap::with_capacity(n)`.
- **Reuse buffers** in loops: `buf.clear()` keeps the capacity; `read_line(&mut buf)` into the same `String`.
- **Borrow instead of cloning**: take `&str` / `&[T]` parameters, return iterators, use `Cow` (Lesson 3).
- Avoid `format!` / `to_string()` in hot loops; `write!` into an existing `String`.
- Collect only when you need a collection; chain iterators otherwise.
- Small strings and vectors on the stack: crates like `smallvec`, `arrayvec`, `compact_str`.
- A faster allocator (`mimalloc`) helps allocation-heavy programs.

## Iterators are zero-cost

`data.iter().filter(...).map(...).sum()` compiles to the same machine code as a hand-written loop, often better: bounds checks disappear and the compiler can **vectorise** (SIMD). Prefer iterators over index loops; `chunks_exact`, `zip` and `windows` help the optimiser further.

## Data layout and the CPU cache

Memory is slow compared to the CPU; caches are fast. Contiguous data (`Vec<T>`) is far faster to scan than pointer-chasing (`LinkedList`, `Vec<Box<T>>`, trees of `Rc`). For large datasets, **struct of arrays** (`xs: Vec<f32>, ys: Vec<f32>`) beats array of structs when you only read some fields. Smaller types (`u32` instead of `u64` IDs, enums instead of strings) mean more data fits in the cache.

## Faster hashing

Rust's default `HashMap` hasher (SipHash) resists denial-of-service attacks but is slower than necessary for trusted keys. `FxHashMap` (rustc-hash) or `ahash` can be 2–5× faster for integer and short-string keys in internal data structures.

## Parallelism

After the single-threaded code is good, use all your cores: **Rayon**'s `par_iter()` (Lesson 4). Parallelism multiplies a fast algorithm; it can't rescue a bad one.

## I/O

Wrap files and stdout in `BufReader` / `BufWriter`; lock stdout once (`let mut out = std::io::stdout().lock();`) when printing many lines. Printing a million lines unbuffered is the classic "why is my Rust program slow" question.

@@quiz
Why did a Rust program run 30× slower than expected?
* It was built in debug mode; benchmark with `--release`
- Rust is slower than Python
- The CPU was busy
- Iterators are slow
? Debug builds skip optimisation.

@@quiz
What usually gives the biggest speed-up?
- Adding `unsafe`
- Using `u8` everywhere
* A better algorithm or data structure (e.g. a `HashSet` instead of repeated `Vec::contains`)
- Removing comments
? O(n²) to O(n) beats any micro-optimisation.

@@quiz
What does `Vec::with_capacity(n)` avoid?
- Bounds checks
* Repeated reallocation and copying as the vector grows
- Cloning elements
- Panics
? Pre-allocating when you know the size is cheap and effective.

@@quiz
What does `std::hint::black_box` do in a benchmark?
* Prevents the optimiser from removing or precomputing the work being measured
- Hides the benchmark from the profiler
- Makes the code run in parallel
- Turns off optimisation for the whole program
? Otherwise the compiler may compute the answer at compile time.

@@drill
Which crate is the standard choice for statistically sound benchmarks?
= criterion
? cargo bench runs them.

@@exercise From O(n²) to O(n)
Two functions are written the slow way. Rewrite them to be fast while giving the same answers:

- `common_count(a: &[u32], b: &[u32]) -> usize`: how many elements of `b` also appear in `a` (counting duplicates in `b`). Use a `HashSet` built from `a`.
- `range_sums(data: &[i64], queries: &[(usize, usize)]) -> Vec<i64>`: for each `(start, end)` (end exclusive), the sum of `data[start..end]`. Build a **prefix sum** array once (`prefix[i]` = sum of the first `i` items) so each query is `prefix[end] - prefix[start]`.

The tests use big inputs: the slow versions would take too long.
@@starter
use std::collections::HashSet;

fn common_count(a: &[u32], b: &[u32]) -> usize {
    b.iter().filter(|x| a.contains(x)).count() // O(len(a) * len(b))
}

fn range_sums(data: &[i64], queries: &[(usize, usize)]) -> Vec<i64> {
    queries.iter().map(|&(s, e)| data[s..e].iter().sum()).collect() // O(n) per query
}

fn main() {
    println!("{}", common_count(&[1, 2, 3, 4], &[2, 4, 4, 9]));
    println!("{:?}", range_sums(&[3, 1, 4, 1, 5, 9], &[(0, 3), (2, 6), (5, 5)]));
}
@@expected
3
[8, 19, 0]
@@tests
#[test]
fn checks() {
    let a: Vec<u32> = (0..200_000).collect();
    let b: Vec<u32> = (100_000..300_000).collect();
    let start = std::time::Instant::now();
    assert_eq!(common_count(&a, &b), 100_000);
    let data: Vec<i64> = (1..=200_000).collect();
    let queries: Vec<(usize, usize)> = (0..200_000).map(|i| (0, 200_000 - i % 3)).collect();
    let sums = range_sums(&data, &queries);
    assert_eq!(sums[0], 200_000 * 200_001 / 2);
    assert_eq!(sums.len(), 200_000);
    assert!(start.elapsed().as_secs() < 3, "too slow: use a HashSet and prefix sums");
    assert_eq!(range_sums(&[], &[(0, 0)]), vec![0]);
}
@@requires
HashSet
prefix
@@hint
`let set: HashSet<&u32> = a.iter().collect(); b.iter().filter(|x| set.contains(x)).count()`. For prefix sums: `let mut prefix = Vec::with_capacity(data.len() + 1); prefix.push(0); for x in data { prefix.push(prefix.last().unwrap() + x); }`.
@@solution
use std::collections::HashSet;

fn common_count(a: &[u32], b: &[u32]) -> usize {
    let set: HashSet<&u32> = a.iter().collect();
    b.iter().filter(|x| set.contains(x)).count()
}

fn range_sums(data: &[i64], queries: &[(usize, usize)]) -> Vec<i64> {
    let mut prefix = Vec::with_capacity(data.len() + 1);
    prefix.push(0);
    for x in data {
        prefix.push(prefix.last().unwrap() + x);
    }
    queries.iter().map(|&(s, e)| prefix[e] - prefix[s]).collect()
}

fn main() {
    println!("{}", common_count(&[1, 2, 3, 4], &[2, 4, 4, 9]));
    println!("{:?}", range_sums(&[3, 1, 4, 1, 5, 9], &[(0, 3), (2, 6), (5, 5)]));
}

@@exercise Memoise an exponential recursion
A delivery robot moves on a grid from the top-left to the bottom-right, only right or down, and can't enter blocked cells (`true` in the grid). Count the paths.

The starter's plain recursion explores the same cells over and over (exponential time). Add **memoisation**: write `count_paths(grid: &[Vec<bool>]) -> u64` using a helper that stores results in a `HashMap<(usize, usize), u64>` (or a 2D `Vec<Option<u64>>`), so each cell is computed once.
@@starter
use std::collections::HashMap;

fn count_paths(grid: &[Vec<bool>]) -> u64 {
    fn go(grid: &[Vec<bool>], r: usize, c: usize) -> u64 {
        let (rows, cols) = (grid.len(), grid[0].len());
        if grid[r][c] {
            return 0;
        }
        if r == rows - 1 && c == cols - 1 {
            return 1;
        }
        let mut total = 0;
        if r + 1 < rows {
            total += go(grid, r + 1, c);
        }
        if c + 1 < cols {
            total += go(grid, r, c + 1);
        }
        total
    }
    go(grid, 0, 0)
}

fn main() {
    let mut grid = vec![vec![false; 4]; 3];
    grid[1][1] = true;
    println!("{}", count_paths(&grid));
    let open = vec![vec![false; 17]; 17];
    println!("{}", count_paths(&open));
}
@@expected
4
601080390
@@tests
#[test]
fn checks() {
    assert_eq!(count_paths(&[vec![false]]), 1);
    assert_eq!(count_paths(&[vec![true]]), 0);
    assert_eq!(count_paths(&[vec![false, false], vec![false, false]]), 2);
    let start = std::time::Instant::now();
    let big = vec![vec![false; 30]; 30];
    assert_eq!(count_paths(&big), 30067266499541040);
    assert!(start.elapsed().as_secs() < 2, "too slow: memoise the results");
}
@@requires
HashMap
.insert(
@@hint
Give `go` an extra parameter `memo: &mut HashMap<(usize, usize), u64>`. At the start: `if let Some(&v) = memo.get(&(r, c)) { return v; }`. Before returning, `memo.insert((r, c), total);`.
@@solution
use std::collections::HashMap;

fn count_paths(grid: &[Vec<bool>]) -> u64 {
    fn go(grid: &[Vec<bool>], r: usize, c: usize, memo: &mut HashMap<(usize, usize), u64>) -> u64 {
        if let Some(&v) = memo.get(&(r, c)) {
            return v;
        }
        let (rows, cols) = (grid.len(), grid[0].len());
        if grid[r][c] {
            return 0;
        }
        if r == rows - 1 && c == cols - 1 {
            return 1;
        }
        let mut total = 0;
        if r + 1 < rows {
            total += go(grid, r + 1, c, memo);
        }
        if c + 1 < cols {
            total += go(grid, r, c + 1, memo);
        }
        memo.insert((r, c), total);
        total
    }
    go(grid, 0, 0, &mut HashMap::new())
}

fn main() {
    let mut grid = vec![vec![false; 4]; 3];
    grid[1][1] = true;
    println!("{}", count_paths(&grid));
    let open = vec![vec![false; 17]; 17];
    println!("{}", count_paths(&open));
}

@@flashcards
measure first :: profile before optimising
criterion :: benchmarking crate
release builds :: always benchmark with --release
avoid allocations in hot loops :: reuse buffers, with_capacity
flamegraph :: visualise where time is spent
iterators :: compile to code as fast as manual loops
