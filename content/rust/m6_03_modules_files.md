@@lesson m6-files Splitting Code Into Files
@@concepts modules
@@text
In a Cargo project, each module can live in **its own file**. This is how every real project is organized. In this lesson you'll take the calculator from Module 5, turn it into a Cargo project, and split it into files.

## The rule: `mod name;` loads `name.rs`

If `src/main.rs` contains

```rust,norun
mod calc;
```

(with a semicolon instead of a `{ }` block), Cargo looks for the module's code in **`src/calc.rs`**. The file's contents are exactly what you'd have written between the braces. The visibility rules are the same: items in `calc.rs` must be `pub` to be used from `main.rs`.

```text
src/
├── main.rs     ← mod calc;   use calc::evaluate;
└── calc.rs     ← pub fn evaluate(...) { ... }
```

## Nested modules: folders

A module can have its own sub-modules in a folder with the same name:

```text
src/
├── main.rs          ← mod game;
├── game.rs          ← pub mod player;  pub mod world;
└── game/
    ├── player.rs
    └── world.rs
```

(An older style uses `game/mod.rs` instead of `game.rs`. You'll see both in the wild, and they mean the same thing.)

## How should code be split?

There's no single right answer, but good rules of thumb:

- **Separate logic from input/output.** The calculator's `evaluate` (logic) goes in one file; reading lines and printing (I/O) stays in `main.rs`.
- **One topic per module.** `storage.rs` saves and loads, `model.rs` defines the data types, `ui.rs` draws things.
- **Keep `main.rs` short.** Ideally it just wires the parts together.
- Don't split *too* early. A 100-line program is fine in one file. Split when a file gets hard to navigate.

@@project calculator Calculator, Cargo edition
This project contains the finished calculator from Module 5, all in `main.rs`. Your job: split it into two files.
@@file Cargo.toml
[package]
name = "calculator"
version = "0.1.0"
edition = "2024"

[dependencies]
@@file src/main.rs
use std::fmt;
use std::io;

#[derive(Debug, PartialEq)]
enum CalcError {
    Empty,
    BadFormat,
    BadNumber(String),
    UnknownOperator(String),
    DivideByZero,
    NegativeSqrt,
}

impl fmt::Display for CalcError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CalcError::Empty => write!(f, "nothing to calculate"),
            CalcError::BadFormat => write!(f, "use: <number> <op> <number>, or sqrt <number>"),
            CalcError::BadNumber(t) => write!(f, "not a number: {t}"),
            CalcError::UnknownOperator(op) => write!(f, "unknown operator: {op}"),
            CalcError::DivideByZero => write!(f, "can't divide by zero"),
            CalcError::NegativeSqrt => write!(f, "can't take the square root of a negative number"),
        }
    }
}

/// Turns a token into a number. The word `mem` means the last result.
fn parse_number(token: &str, memory: f64) -> Result<f64, CalcError> {
    if token == "mem" {
        return Ok(memory);
    }
    token.parse::<f64>().map_err(|_| CalcError::BadNumber(token.to_string()))
}

fn evaluate(line: &str, memory: f64) -> Result<f64, CalcError> {
    let tokens: Vec<&str> = line.split_whitespace().collect();
    match tokens.as_slice() {
        [] => Err(CalcError::Empty),
        ["sqrt", x] => {
            let n = parse_number(x, memory)?;
            if n < 0.0 {
                return Err(CalcError::NegativeSqrt);
            }
            Ok(n.sqrt())
        }
        [a, op, b] => {
            let x = parse_number(a, memory)?;
            let y = parse_number(b, memory)?;
            match *op {
                "+" => Ok(x + y),
                "-" => Ok(x - y),
                "*" => Ok(x * y),
                "/" => {
                    if y == 0.0 {
                        Err(CalcError::DivideByZero)
                    } else {
                        Ok(x / y)
                    }
                }
                other => Err(CalcError::UnknownOperator(other.to_string())),
            }
        }
        _ => Err(CalcError::BadFormat),
    }
}

fn main() {
    println!("Calculator (type quit to exit)");
    let mut memory = 0.0;
    for line in io::stdin().lines() {
        let line = line.expect("failed to read input");
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        if line == "quit" {
            break;
        }
        match evaluate(line, memory) {
            Ok(result) => {
                println!("= {result}");
                memory = result;
            }
            Err(e) => println!("error: {e}"),
        }
    }
    println!("bye!");
}

@@text
## Step by step

1. Create the project and open it in VS Code. Run it with `cargo run` and try a few calculations **interactively**. (Type `quit` to exit. The `> line` echo is gone, since you can now see what you type.)
2. In VS Code's file explorer, right-click the `src` folder → **New File** → `calc.rs`.
3. **Cut** `CalcError`, its `impl fmt::Display`, `parse_number` and `evaluate` from `main.rs`, and paste them into `calc.rs`. Move `use std::fmt;` along with them.
4. In `calc.rs`, make the things `main.rs` needs **public**: `pub enum CalcError` and `pub fn evaluate`. Leave `parse_number` private: it's an internal helper.
5. At the top of `main.rs`, add:

```rust,norun
mod calc;

use calc::evaluate;
use std::io;
```

6. Run `cargo run` again. It should work exactly as before.

Your finished `calc.rs` starts like this:

```rust,norun
use std::fmt;

#[derive(Debug, PartialEq)]
pub enum CalcError {
    // ... the variants ...
}

impl fmt::Display for CalcError {
    // ... unchanged ...
}

fn parse_number(token: &str, memory: f64) -> Result<f64, CalcError> {
    // ... unchanged ...
}

pub fn evaluate(line: &str, memory: f64) -> Result<f64, CalcError> {
    // ... unchanged ...
}
```

And `main.rs` is now short and only about input/output. If you get errors like `function evaluate is private`, check your `pub`s. rust-analyzer often offers a quick fix: click the 💡 lightbulb, or press `Ctrl+.`.

> **Stuck?** The project's files on disk are yours. If you break something badly, you can use **↺ Reset project** (in "This step's starting files") to go back to the starting point. Your old files are backed up first.

@@quiz
`main.rs` contains `mod storage;`. Where does Cargo look for the module's code?
- In `storage.toml`
* In `src/storage.rs` (or `src/storage/mod.rs`)
- On crates.io
? `mod name;` with a semicolon loads the module from `name.rs` next to the file that declares it.

@@quiz
After moving `evaluate` into `calc.rs`, `main.rs` says "function `evaluate` is private". What's the fix?
* Add `pub` before `fn evaluate` in `calc.rs`
- Rename the file
- Move `main` into `calc.rs`
? Items in a module are private unless marked `pub`.

@@drill
Which line in `main.rs` loads the module in `shop.rs`?
= mod shop;
= mod shop
? use only creates shortcuts; mod loads the file.

@@flashcards
mod utils; :: loads src/utils.rs (or src/utils/mod.rs)
nested module file :: src/utils/math.rs for mod math inside utils
pub use :: re-export an item
lib.rs + main.rs :: a library crate and a binary in one package

@@exercise Organise IP helpers into modules
The Cargo project above splits code into files. The same `mod` rules work inside one file, which is how you'll practise them here: `mod name { ... }` instead of `mod name;`.

Build a `network` module with two submodules:

- `network::ip` with `pub fn parse(text: &str) -> Option<[u8; 4]>` (four dot-separated numbers 0-255, otherwise `None`) and `pub fn is_private(o: [u8; 4]) -> bool` (10.0.0.0/8, 172.16.0.0/12, 192.168.0.0/16)
- `network::report` with `pub fn describe(text: &str) -> String` returning `"<text>: private"`, `"<text>: public"` or `"<text>: invalid"`. It reaches the other submodule with `use super::ip;`.

`main` imports `describe` with `use network::report::describe;` and prints one line per address.
@@starter
// Put the IP helpers in a module called `network`, with two submodules:
//   network::ip     -> pub fn parse(text: &str) -> Option<[u8; 4]>
//                      pub fn is_private(o: [u8; 4]) -> bool
//   network::report -> pub fn describe(text: &str) -> String
// report reaches its sibling module through `super`. main imports describe with `use`.

fn main() {
    for text in ["10.1.2.3", "172.20.0.9", "172.32.0.1", "192.168.1.10", "8.8.8.8", "300.1.1.1", "1.2.3"] {
        // println!("{}", describe(text));
        let _ = text;
    }
}
@@expected
10.1.2.3: private
172.20.0.9: private
172.32.0.1: public
192.168.1.10: private
8.8.8.8: public
300.1.1.1: invalid
1.2.3: invalid
@@requires
mod network
pub mod ip
pub mod report
use super::ip
use network::report::describe
@@hint
Everything inside a module is private until you add `pub`, including the submodules themselves. 172.16.0.0/12 means the second octet is 16 to 31: `(16..=31).contains(&o[1])`.
@@solution
mod network {
    pub mod ip {
        /// Parses "a.b.c.d" into four octets.
        pub fn parse(text: &str) -> Option<[u8; 4]> {
            let parts: Vec<&str> = text.split('.').collect();
            if parts.len() != 4 {
                return None;
            }
            let mut octets = [0u8; 4];
            for (i, part) in parts.iter().enumerate() {
                octets[i] = part.parse().ok()?;
            }
            Some(octets)
        }

        /// True for 10.0.0.0/8, 172.16.0.0/12 and 192.168.0.0/16.
        pub fn is_private(o: [u8; 4]) -> bool {
            o[0] == 10 || (o[0] == 172 && (16..=31).contains(&o[1])) || (o[0] == 192 && o[1] == 168)
        }
    }

    pub mod report {
        use super::ip;

        pub fn describe(text: &str) -> String {
            match ip::parse(text) {
                None => format!("{text}: invalid"),
                Some(o) if ip::is_private(o) => format!("{text}: private"),
                Some(_) => format!("{text}: public"),
            }
        }
    }
}

use network::report::describe;

fn main() {
    for text in ["10.1.2.3", "172.20.0.9", "172.32.0.1", "192.168.1.10", "8.8.8.8", "300.1.1.1", "1.2.3"] {
        println!("{}", describe(text));
    }
}
