@@lesson m6-tools Documentation, Libraries and Tools
@@text
This lesson rounds off the professional toolkit: documenting your code, splitting a project into a library and a program, and the tools that keep code clean.

## Documentation comments

`///` comments document the item below them. They're written in Markdown, and tools turn them into a website:

```rust,norun
/// Calculates the area of a circle.
///
/// # Examples
///
/// ```
/// let a = geometry::circle_area(2.0);
/// assert!((a - 12.566).abs() < 0.001);
/// ```
pub fn circle_area(radius: f64) -> f64 {
    std::f64::consts::PI * radius * radius
}
```

- The first line is a one-sentence summary.
- Common sections: `# Examples`, `# Errors` (when it returns `Err`), `# Panics` (when it can panic).
- `//!` at the top of a file documents the **module itself** (you've seen this at the top of this app's own source files, if you've peeked).

Run **`cargo doc --open`** to build the documentation for your project — *and every crate it depends on* — and open it in your browser. It looks exactly like the official Rust docs, because they're made with the same tool.

In VS Code, hovering over a function shows its doc comment. Documenting your public functions helps you as much as anyone else.

## Doc tests

Code inside a doc comment's `# Examples` section is **compiled and run by `cargo test`**! These are called **doc tests**. They guarantee that your examples never go out of date. (Doc tests work for library crates; see below.)

## Library + binary

A Cargo project can contain:

- a **binary crate**: `src/main.rs`, a program you can run;
- a **library crate**: `src/lib.rs`, code meant to be used by other code;
- or **both**.

With both, put the logic in `lib.rs` (and its modules) and keep `main.rs` as a thin layer that handles input/output. In `main.rs`, you use the library by the project's name:

```text
src/
├── lib.rs      ← pub mod calc;
├── calc.rs     ← pub fn evaluate(...)
└── main.rs     ← use calculator::calc::evaluate;   (the package is called "calculator")
```

This is the same "separate logic from I/O" idea, made official. Libraries can be tested on their own, reused by another program (a GUI version, for example!), and get doc tests.

## Clippy: the friendly linter

**Clippy** checks your code for hundreds of common mistakes and non-idiomatic patterns:

```text
cargo clippy
```

For example, it would tell you:

```text
warning: this `if` has identical blocks
warning: you seem to be trying to use `match` for destructuring a single pattern. Consider using `if let`
warning: length comparison to zero: use `.is_empty()` instead
```

Each warning links to an explanation. Running Clippy regularly is one of the fastest ways to learn idiomatic Rust. Try it on the calculator project now!

## rustfmt: automatic formatting

```text
cargo fmt
```

This formats every file in the official style. In VS Code you can enable **Format on Save** (Settings → search "format on save"), so your code is always tidy.

## A professional's workflow

1. `cargo check` (or just watch rust-analyzer's squiggles) while writing.
2. `cargo test` after each change.
3. `cargo clippy` and `cargo fmt` before you consider something "done".
4. `cargo build --release` for the version you share.

@@quiz
What does `cargo doc --open` do?
- Opens the Rust book
* Builds HTML documentation for your project and its dependencies, then opens it in the browser
- Adds comments to your code
? It generates docs from your `///` comments, in the same style as the official documentation.

@@quiz
In a project with both `lib.rs` and `main.rs`, where should most of the logic go?
* In the library (`lib.rs` and its modules), with `main.rs` handling input/output
- In `main.rs`
- In `Cargo.toml`
? A library is easy to test and reuse; `main.rs` stays a thin layer.

@@quiz
Which command suggests more idiomatic ways to write your code?
- `cargo fmt`
* `cargo clippy`
- `cargo run`
? Clippy is a linter: it looks for common mistakes and non-idiomatic patterns. `cargo fmt` only changes formatting.

@@drill
Which comment syntax writes documentation for the item below it?
= ///
? cargo doc --open builds the docs.

@@flashcards
cargo doc --open :: build and open your documentation
rust-analyzer :: editor language server
clippy :: linter with hundreds of checks
rustfmt :: standard code formatter
rustup :: installs and updates toolchains

@@exercise Document and test a function
Write `percent(part, whole)`: what percentage `part` is of `whole`, rounded to one decimal place, or `None` when `whole` is zero.

Then do what a professional would do before publishing it:

1. a `///` doc comment that says what it returns, when it returns `None`, and has an `# Examples` section,
2. a `#[cfg(test)]` module with at least three tests (a normal case, rounding, and zero).

When you press ✔ Check, your tests and some hidden ones run.
@@starter
// Write the doc comment (with an examples section) and the body,
// then add a test module with at least three tests.
fn percent(part: u64, whole: u64) -> Option<f64> {
    let _ = (part, whole);
    todo!()
}

fn main() {
    println!("{:?}", percent(37, 120));
    println!("{:?}", percent(5, 0));
}
@@tests
#[test]
fn hidden_percent_values() {
    assert_eq!(percent(37, 120), Some(30.8));
    assert_eq!(percent(0, 9), Some(0.0));
    assert_eq!(percent(2, 0), None);
}
@@requires
///
# Examples
#[cfg(test)]
#[test]
assert_eq!
@@hint
Round to one decimal with `(x * 10.0).round() / 10.0`. In the tests module, `use super::*;` brings `percent` into scope.
@@solution
/// Returns what percentage `part` is of `whole`, rounded to one decimal place.
///
/// Returns `None` when `whole` is zero, because the percentage is undefined.
///
/// # Examples
///
/// ```
/// assert_eq!(percent(1, 4), Some(25.0));
/// assert_eq!(percent(1, 0), None);
/// ```
fn percent(part: u64, whole: u64) -> Option<f64> {
    if whole == 0 {
        return None;
    }
    let raw = part as f64 * 100.0 / whole as f64;
    Some((raw * 10.0).round() / 10.0)
}

fn main() {
    println!("{:?}", percent(37, 120));
    println!("{:?}", percent(5, 0));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn quarter() {
        assert_eq!(percent(1, 4), Some(25.0));
    }

    #[test]
    fn rounds_to_one_decimal() {
        assert_eq!(percent(1, 3), Some(33.3));
    }

    #[test]
    fn zero_whole_is_none() {
        assert_eq!(percent(7, 0), None);
    }
}
