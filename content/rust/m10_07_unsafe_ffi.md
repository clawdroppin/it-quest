@@lesson m10-unsafe Unsafe Rust and FFI: Talking to C and the Operating System
@@text
Rust's guarantees come from rules the compiler can check. Some correct programs can't be proven correct by the compiler: talking to hardware, calling C libraries, writing the internals of `Vec` itself. For those, Rust has `unsafe`. It doesn't turn off the borrow checker; it unlocks **five extra abilities**, and makes **you** responsible for upholding the rules inside that block.

## The five unsafe superpowers

Inside `unsafe { ... }` (or an `unsafe fn`) you may:

1. **Dereference raw pointers** (`*const T`, `*mut T`).
2. **Call unsafe functions** (including C functions declared via FFI).
3. **Access or modify mutable statics** (`static mut`).
4. **Implement unsafe traits** (`unsafe impl Send for MyType {}`).
5. **Access fields of unions**.

Everything else (types, borrowing of references, bounds checks on slices) still applies.

## Raw pointers

```rust
fn main() {
    let mut x = 10;
    let p = &raw mut x;          // creating a raw pointer is safe
    unsafe {
        *p += 5;                 // dereferencing it is not
    }
    println!("{x}");

    let numbers = [1, 2, 3, 4];
    let ptr = numbers.as_ptr();
    let third = unsafe { *ptr.add(2) };   // pointer arithmetic
    println!("{third}");
}
```

Raw pointers can be null, dangling, unaligned, or alias a `&mut`. The compiler checks none of that: **you** must guarantee it whenever you dereference. Undefined behaviour (UB) in unsafe code doesn't just crash; it lets the optimiser assume impossible things and produce wildly wrong programs.

## Safe abstractions over unsafe code

The goal is to wrap a small unsafe core in a **safe API** that can't be misused. The standard library does this everywhere. `slice::split_at_mut` gives two `&mut` halves of one slice, which the borrow checker can't verify as non-overlapping:

```rust,norun
fn split_at_mut(v: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    assert!(mid <= v.len());                       // the safety check
    let len = v.len();
    let ptr = v.as_mut_ptr();
    // SAFETY: the two ranges [0, mid) and [mid, len) don't overlap,
    // and both are inside the original slice, which we borrow mutably.
    unsafe {
        (std::slice::from_raw_parts_mut(ptr, mid),
         std::slice::from_raw_parts_mut(ptr.add(mid), len - mid))
    }
}
```

Conventions every Rust codebase follows:

- Keep `unsafe` blocks as **small** as possible.
- Write a `// SAFETY:` comment explaining why each block is sound.
- Document an `unsafe fn` with a `# Safety` section listing what callers must guarantee.
- Since edition 2024, operations inside an `unsafe fn` still need their own `unsafe { }` block (`unsafe_op_in_unsafe_fn`), so each risky line is marked.

## FFI: calling C from Rust

C is the lingua franca of operating systems and libraries (SQLite, OpenSSL, libcurl, Win32, POSIX). Declare the foreign functions in an `unsafe extern` block:

```rust,norun
use std::ffi::{CStr, CString, c_char, c_int};

unsafe extern "C" {
    fn abs(x: c_int) -> c_int;
    fn strlen(s: *const c_char) -> usize;
}

fn main() {
    let n = unsafe { abs(-7) };
    let text = CString::new("hello").unwrap();        // adds the terminating NUL
    let len = unsafe { strlen(text.as_ptr()) };
    println!("{n} {len}");
}
```

Key types: `CString` (owned, NUL-terminated, for passing to C), `CStr` (borrowed, for reading C strings), `c_int`, `c_char`, `*const T`. Link to libraries with `#[link(name = "sqlite3")]` or a `build.rs` script. In practice you rarely write declarations by hand: **bindgen** generates them from C headers, and many `-sys` crates (`libsqlite3-sys`, `openssl-sys`) already wrap them, with safe crates (`rusqlite`, `openssl`) on top.

On Windows, the `windows` crate from Microsoft gives typed access to the entire Win32 and WinRT API; on Unix, the `libc` and `nix` crates cover POSIX.

## Exposing Rust to other languages

```rust,norun
#[unsafe(no_mangle)]
pub extern "C" fn add(a: i32, b: i32) -> i32 { a + b }
```

`extern "C"` uses the C calling convention; `#[unsafe(no_mangle)]` keeps the symbol name `add` so C, Python (`ctypes`, or better **PyO3**), C#, Go or Node (**napi-rs**) can call it. Build a `cdylib` (`crate-type = ["cdylib"]`) to produce a `.dll`/`.so`/`.dylib`. **cbindgen** writes the C header for you. This is how companies rewrite performance-critical parts of Python or C++ programs in Rust one function at a time.

Rules at the boundary: use `#[repr(C)]` structs so the layout matches C; never let a panic unwind into C (catch it with `std::panic::catch_unwind`, or set `panic = "abort"`); agree on who frees memory (memory allocated by Rust must be freed by Rust: provide a `free_thing` function).

## Tools for checking unsafe code

- **Miri** (`cargo +nightly miri test`) interprets your program and detects undefined behaviour: out-of-bounds pointers, use after free, invalid aliasing, data races.
- **Sanitizers** (AddressSanitizer, ThreadSanitizer) on nightly.
- `cargo geiger` counts unsafe code in your dependencies.
- **Clippy** lints such as `undocumented_unsafe_blocks`.

## Should you write unsafe?

Almost never in application code. Prefer a well-tested crate that already encapsulates it. Legitimate reasons: FFI, hand-optimised data structures after profiling proves the need, hardware access in embedded code, and implementing low-level primitives. Many large Rust applications contain zero lines of `unsafe`.

@@quiz
What does `unsafe` turn off?
- The borrow checker
- All type checking
* Nothing: it unlocks five extra abilities (raw pointer dereference, calling unsafe functions, mutable statics, unsafe traits, union fields), and you must uphold the rules there
- Bounds checks on every slice
? References inside unsafe blocks are still borrow-checked.

@@quiz
Is creating a raw pointer unsafe?
- Yes
* No; only dereferencing it is
- Only if it's null
- Only for `*mut`
? `&raw const x` and `as_ptr()` are safe operations.

@@quiz
What should accompany every `unsafe` block?
* A `// SAFETY:` comment explaining why the operations are sound
- A `#[test]`
- `#[allow(unsafe_code)]`
- A call to `catch_unwind`
? Reviewers and future you need the reasoning.

@@quiz
What is `CString` for?
- Faster strings
* An owned, NUL-terminated string that can be passed to C functions
- Reading C strings returned by a library
- Strings on the stack
? `CStr` is the borrowed version for reading.

@@quiz
Which tool detects undefined behaviour in unsafe code by interpreting it?
- rustfmt
- cargo doc
* Miri
- bindgen
? Run your tests under Miri when you write unsafe code.

@@drill
Which keyword marks a block where you may dereference raw pointers?
= unsafe
? You promise the compiler the code is sound.

@@exercise A safe wrapper over raw pointers
Implement two functions using raw pointers, each wrapped in a safe API:

- `sum_raw(data: &[i64]) -> i64`: get `data.as_ptr()`, then read each element with `unsafe { *ptr.add(i) }` for `i` in `0..data.len()` and add them up.
- `swap_ends(data: &mut [i32])`: swap the first and last elements using `data.as_mut_ptr()` and `std::ptr::swap`. Do nothing if the slice has fewer than 2 elements (that's the safety check).

Write a `// SAFETY:` comment above each unsafe block.
@@starter
fn sum_raw(data: &[i64]) -> i64 {
    todo!()
}

fn swap_ends(data: &mut [i32]) {
    todo!()
}

fn main() {
    println!("{}", sum_raw(&[5, 10, 20]));
    let mut v = vec![1, 2, 3, 4];
    swap_ends(&mut v);
    println!("{:?}", v);
}
@@expected
35
[4, 2, 3, 1]
@@tests
#[test]
fn checks() {
    assert_eq!(sum_raw(&[]), 0);
    assert_eq!(sum_raw(&[-3, 3, 7]), 7);
    let mut one = vec![9];
    swap_ends(&mut one);
    assert_eq!(one, vec![9]);
    let mut empty: Vec<i32> = vec![];
    swap_ends(&mut empty);
    let mut two = vec![1, 2];
    swap_ends(&mut two);
    assert_eq!(two, vec![2, 1]);
}
@@requires
as_ptr()
unsafe
SAFETY
ptr::swap
@@hint
`let ptr = data.as_ptr(); let mut total = 0; for i in 0..data.len() { total += unsafe { *ptr.add(i) }; }`. For the swap: `let p = data.as_mut_ptr(); unsafe { std::ptr::swap(p, p.add(data.len() - 1)); }`.
@@solution
fn sum_raw(data: &[i64]) -> i64 {
    let ptr = data.as_ptr();
    let mut total = 0;
    for i in 0..data.len() {
        // SAFETY: i < data.len(), so ptr.add(i) points inside the slice we borrow.
        total += unsafe { *ptr.add(i) };
    }
    total
}

fn swap_ends(data: &mut [i32]) {
    if data.len() < 2 {
        return;
    }
    let p = data.as_mut_ptr();
    // SAFETY: the slice has at least 2 elements, so index 0 and len-1 are valid and distinct,
    // and we hold the only mutable borrow of the slice.
    unsafe {
        std::ptr::swap(p, p.add(data.len() - 1));
    }
}

fn main() {
    println!("{}", sum_raw(&[5, 10, 20]));
    let mut v = vec![1, 2, 3, 4];
    swap_ends(&mut v);
    println!("{:?}", v);
}

@@exercise A C-compatible API
Write a small library API the way you would to expose Rust to C, Python or C#:

- `#[repr(C)] struct Stats { count: u32, min: i32, max: i32 }` (with `Debug, PartialEq` derived).
- `unsafe extern "C" fn stats(ptr: *const i32, len: usize, out: *mut Stats) -> i32`: returns `-1` if `ptr` or `out` is null or `len` is 0; otherwise builds a slice with `std::slice::from_raw_parts`, writes the stats into `*out`, and returns `0`.
- A safe Rust wrapper `fn stats_of(values: &[i32]) -> Option<Stats>` that calls it and converts the status code.

C callers can't use `Result`, so status codes and out-parameters are the convention at the boundary.
@@starter
#[repr(C)]
#[derive(Debug, PartialEq)]
struct Stats {
    count: u32,
    min: i32,
    max: i32,
}

/// # Safety
/// `ptr` must point to `len` valid i32 values and `out` to writable memory for one `Stats`.
unsafe extern "C" fn stats(ptr: *const i32, len: usize, out: *mut Stats) -> i32 {
    todo!()
}

fn stats_of(values: &[i32]) -> Option<Stats> {
    todo!()
}

fn main() {
    println!("{:?}", stats_of(&[4, -2, 9, 0]));
    println!("{:?}", stats_of(&[]));
    let status = unsafe { stats(std::ptr::null(), 3, std::ptr::null_mut()) };
    println!("{status}");
}
@@expected
Some(Stats { count: 4, min: -2, max: 9 })
None
-1
@@tests
#[test]
fn checks() {
    assert_eq!(stats_of(&[7]), Some(Stats { count: 1, min: 7, max: 7 }));
    let mut s = Stats { count: 0, min: 0, max: 0 };
    let data = [1, 2, 3];
    let code = unsafe { stats(data.as_ptr(), 3, &mut s) };
    assert_eq!((code, s), (0, Stats { count: 3, min: 1, max: 3 }));
    let code = unsafe { stats(data.as_ptr(), 3, std::ptr::null_mut()) };
    assert_eq!(code, -1);
}
@@requires
#[repr(C)]
from_raw_parts
is_null()
@@hint
Inside `stats`: `if ptr.is_null() || out.is_null() || len == 0 { return -1; }`, then `let values = unsafe { std::slice::from_raw_parts(ptr, len) };` and `unsafe { *out = Stats { ... }; }`. In the wrapper, create `let mut s = Stats { count: 0, min: 0, max: 0 };` and call `unsafe { stats(values.as_ptr(), values.len(), &mut s) }`.
@@solution
#[repr(C)]
#[derive(Debug, PartialEq)]
struct Stats {
    count: u32,
    min: i32,
    max: i32,
}

/// # Safety
/// `ptr` must point to `len` valid i32 values and `out` to writable memory for one `Stats`.
unsafe extern "C" fn stats(ptr: *const i32, len: usize, out: *mut Stats) -> i32 {
    if ptr.is_null() || out.is_null() || len == 0 {
        return -1;
    }
    // SAFETY: the caller promises ptr points to len valid values.
    let values = unsafe { std::slice::from_raw_parts(ptr, len) };
    let min = *values.iter().min().unwrap();
    let max = *values.iter().max().unwrap();
    // SAFETY: out is non-null and the caller promises it is writable.
    unsafe {
        *out = Stats { count: len as u32, min, max };
    }
    0
}

fn stats_of(values: &[i32]) -> Option<Stats> {
    let mut s = Stats { count: 0, min: 0, max: 0 };
    // SAFETY: the slice gives a valid pointer and length, and s is a valid Stats.
    let code = unsafe { stats(values.as_ptr(), values.len(), &mut s) };
    if code == 0 { Some(s) } else { None }
}

fn main() {
    println!("{:?}", stats_of(&[4, -2, 9, 0]));
    println!("{:?}", stats_of(&[]));
    let status = unsafe { stats(std::ptr::null(), 3, std::ptr::null_mut()) };
    println!("{status}");
}

@@flashcards
unsafe :: lets you do five extra things the compiler can't check
raw pointers :: *const T and *mut T
extern "C" :: call or expose C functions
safe wrapper :: hide unsafe code behind a safe API
SAFETY comment :: explain why an unsafe block is sound
Miri :: detects undefined behaviour in tests
