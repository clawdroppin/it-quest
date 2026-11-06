@@lesson m10-macros Macros: macro_rules! and Procedural Macros
@@text
You've used macros since your first program: `println!`, `vec!`, `format!`, `assert_eq!`, `#[derive(Debug)]`. A **macro** is code that writes code. It runs at compile time and expands into ordinary Rust before type checking. Macros remove boilerplate that functions can't: a variable number of arguments, new syntax, or generating trait implementations.

## Functions vs macros

| Function | Macro |
|---|---|
| fixed number of typed arguments | any number of tokens in any pattern (`vec![1, 2, 3]`) |
| runs at runtime | expands at compile time |
| can't generate items (structs, impls) | can generate any code |
| easy to read and debug | harder to read; use when a function can't do the job |

Rule of thumb: write a function first. Reach for a macro when you'd otherwise copy and paste nearly identical code (implementing the same trait for ten types, test tables), or need variadic syntax.

## Declarative macros: `macro_rules!`

A `macro_rules!` macro is a list of **rules**: a pattern of tokens and the code it expands to. The first matching rule wins.

```rust
macro_rules! square {
    ($x:expr) => {
        $x * $x
    };
}

macro_rules! greet {
    () => {
        println!("Hello!")
    };
    ($name:expr) => {
        println!("Hello, {}!", $name)
    };
}

fn main() {
    println!("{}", square!(7));
    println!("{}", square!(2 + 1));   // expands to (2 + 1) * (2 + 1) as a whole expression: 9
    greet!();
    greet!("Ferris");
}
```

`$x:expr` is a **metavariable** with a **fragment specifier**: what kind of syntax it matches.

| Specifier | Matches |
|---|---|
| `expr` | an expression: `2 + 1`, `f(x)` |
| `ident` | an identifier: `count`, `MyType` |
| `ty` | a type: `Vec<String>` |
| `pat` | a pattern: `Some(x)`, `1..=5` |
| `literal` | a literal: `"hi"`, `42` |
| `block` | a `{ ... }` block |
| `tt` | a single token tree (anything; the most flexible) |
| `path`, `stmt`, `item`, `vis`, `lifetime`, `meta` | more specialised pieces |

An `expr` metavariable is kept as one unit, which is why `square!(2 + 1)` correctly gives 9 rather than `2 + 1 * 2 + 1`.

## Repetition

`$( ... ),*` repeats a pattern separated by commas (zero or more); `+` means one or more; `?` means optional. The same repetition in the expansion emits code once per match:

```rust
macro_rules! my_vec {
    ( $( $item:expr ),* $(,)? ) => {{
        let mut v = Vec::new();
        $( v.push($item); )*
        v
    }};
}

fn main() {
    let v = my_vec![1, 2, 3,];
    println!("{:?}", v);
}
```

`$(,)?` allows an optional trailing comma. The double braces `{{ ... }}` make the expansion a single block expression.

## Hygiene and scoping

Variables created inside a macro (like `v` above) don't clash with the caller's variables of the same name: macros are **hygienic** for local variables. Items such as functions and types are not, so refer to library items with full paths in exported macros (`$crate::helper()`, `::std::collections::HashMap`).

A macro must be defined **before** it's used in the file. Mark it `#[macro_export]` to make it available to other crates as `your_crate::my_macro!`.

## Generating code: implementing traits for many types

```rust
trait Describe {
    fn describe() -> String;
}

macro_rules! impl_describe {
    ($($t:ty => $text:literal),+ $(,)?) => {
        $( impl Describe for $t {
            fn describe() -> String { $text.to_string() }
        } )+
    };
}

impl_describe! {
    u8 => "a small unsigned number",
    i64 => "a big signed number",
    String => "owned text",
}

fn main() {
    println!("{} / {}", <u8 as Describe>::describe(), String::describe());
}
```

This is exactly how the standard library implements `Add`, `Display` and dozens of other traits for every number type without repeating itself.

## Debugging macros

- `cargo expand` (install with `cargo install cargo-expand`) shows the code after expansion.
- Start with one rule and one test call; add repetition after it works.
- Error messages point into the expansion; read the "in this macro invocation" note.

## Procedural macros

For more power, **procedural macros** are Rust functions, compiled as a separate crate, that receive the token stream and return new tokens. Three kinds:

| Kind | Looks like | Examples |
|---|---|---|
| **Derive** | `#[derive(Serialize)]` | serde, thiserror, clap's `Parser` |
| **Attribute** | `#[tokio::main]`, `#[get("/")]` | tokio, web frameworks, tracing's `#[instrument]` |
| **Function-like** | `sql!(SELECT ...)`, `html! { ... }` | sqlx's checked queries, UI templates |

They're written with the `syn` crate (parses Rust code into a syntax tree) and `quote` (turns templates back into tokens):

```rust,norun
// in a crate with `proc-macro = true` in [lib]
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(Hello)]
pub fn derive_hello(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    let name = &ast.ident;
    quote! {
        impl #name {
            pub fn hello() -> String { format!("Hello from {}", stringify!(#name)) }
        }
    }.into()
}
```

Procedural macros slow compile times and are hard to debug, so use them for library-level ergonomics, not everyday code.

## Built-in macros worth knowing

`dbg!` (print and return a value with file and line), `todo!` / `unimplemented!` / `unreachable!`, `matches!`, `assert!` / `debug_assert!`, `include_str!` / `include_bytes!` (embed a file in the binary), `env!` / `option_env!` (compile-time environment variables, e.g. `env!("CARGO_PKG_VERSION")`), `concat!`, `stringify!`, `cfg!`, `write!` / `writeln!`, `format_args!`.

@@quiz
Why does `square!(2 + 1)` give 9 with `$x:expr`?
* An `expr` fragment is substituted as a single expression, so it behaves like `(2 + 1) * (2 + 1)`
- Macros evaluate their arguments first like functions
- It doesn't; it gives 5
- Because of operator overloading
? Fragment specifiers keep the structure of what they matched.

@@quiz
What does `$( $item:expr ),*` match?
- Exactly one expression
* Zero or more expressions separated by commas
- One or more blocks
- A comma only
? `+` would require at least one.

@@quiz
Which kind of macro is `#[derive(Serialize)]`?
- A declarative `macro_rules!` macro
* A procedural derive macro
- A built-in compiler intrinsic
- An attribute on a function
? Derive macros generate trait implementations from a type definition.

@@quiz
When should you prefer a function over a macro?
* Whenever a function can do the job: functions are easier to read, type-check and debug
- Never; macros are always faster
- Only in tests
- When you need variable arguments
? Macros are for what functions can't express.

@@drill
Which macro defines a declarative macro?
= macro_rules!
= macro_rules
? macro_rules! square { ($x:expr) => { $x * $x }; }

@@exercise A hashmap! literal macro
Write a macro `hashmap!` that builds a `HashMap` from `key => value` pairs, with an optional trailing comma, and that also works with no pairs at all. Then write `max_of!`: given one or more expressions, it returns the largest (use recursion in the macro: one expression returns itself; otherwise compare the first with `max_of!` of the rest using `std::cmp::max`).
@@starter
use std::collections::HashMap;

// macro_rules! hashmap { ... }

// macro_rules! max_of { ... }

fn main() {
    let ports: HashMap<&str, u16> = hashmap! {
        "http" => 80,
        "https" => 443,
        "ssh" => 22,
    };
    let mut names: Vec<_> = ports.keys().collect();
    names.sort();
    println!("{:?} {}", names, ports["ssh"]);
    let empty: HashMap<String, i32> = hashmap! {};
    println!("{}", empty.len());
    println!("{}", max_of!(3, 9, 4));
    println!("{}", max_of!(-1));
}
@@expected
["http", "https", "ssh"] 22
0
9
-1
@@tests
#[test]
fn checks() {
    let m = hashmap! { 1 => "one" };
    assert_eq!(m.get(&1), Some(&"one"));
    let m2 = hashmap! { "a" => 1, "b" => 2 };
    assert_eq!(m2.len(), 2);
    assert_eq!(max_of!(1, 2), 2);
    assert_eq!(max_of!(5, 1, 3, 2), 5);
    assert_eq!(max_of!(2 + 2, 3), 4);
}
@@requires
macro_rules! hashmap
macro_rules! max_of
@@hint
`hashmap`: `( $( $k:expr => $v:expr ),* $(,)? ) => {{ let mut m = ::std::collections::HashMap::new(); $( m.insert($k, $v); )* m }};`. `max_of`: two rules, `($x:expr) => { $x };` and `($x:expr, $($rest:expr),+) => { ::std::cmp::max($x, max_of!($($rest),+)) };`.
@@solution
use std::collections::HashMap;

macro_rules! hashmap {
    ( $( $k:expr => $v:expr ),* $(,)? ) => {{
        #[allow(unused_mut)]
        let mut m = ::std::collections::HashMap::new();
        $( m.insert($k, $v); )*
        m
    }};
}

macro_rules! max_of {
    ($x:expr) => { $x };
    ($x:expr, $($rest:expr),+) => { ::std::cmp::max($x, max_of!($($rest),+)) };
}

fn main() {
    let ports: HashMap<&str, u16> = hashmap! {
        "http" => 80,
        "https" => 443,
        "ssh" => 22,
    };
    let mut names: Vec<_> = ports.keys().collect();
    names.sort();
    println!("{:?} {}", names, ports["ssh"]);
    let empty: HashMap<String, i32> = hashmap! {};
    println!("{}", empty.len());
    println!("{}", max_of!(3, 9, 4));
    println!("{}", max_of!(-1));
}

@@exercise Generate newtypes
Units are easy to mix up: a function expecting metres happily accepts seconds when both are `f64`. Write a macro `unit!` that, given a name and a suffix, generates a newtype struct with:

- `#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]` and one `pub f64` field
- `impl std::ops::Add` (same unit + same unit)
- `impl std::fmt::Display` printing the number followed by the suffix, e.g. `12.5 m`

Then call it to create `Meters` (suffix `"m"`), `Seconds` (`"s"`) and `Kilograms` (`"kg"`), and write `speed(d: Meters, t: Seconds) -> f64`.
@@starter
// macro_rules! unit { ... }

// unit!(Meters, "m");  and so on

fn speed(d: Meters, t: Seconds) -> f64 {
    todo!()
}

fn main() {
    let run = Meters(400.0) + Meters(100.0);
    let time = Seconds(50.0);
    println!("{} in {} = {} m/s", run, time, speed(run, time));
    println!("{}", Kilograms(2.5) + Kilograms(0.5));
    // speed(time, run) would not compile: the types are different
}
@@expected
500 m in 50 s = 10 m/s
3 kg
@@tests
#[test]
fn checks() {
    assert_eq!(Meters(1.5) + Meters(2.0), Meters(3.5));
    assert_eq!(format!("{}", Seconds(0.25)), "0.25 s");
    assert!(Kilograms(1.0) < Kilograms(2.0));
    assert_eq!(speed(Meters(9.0), Seconds(3.0)), 3.0);
}
@@requires
macro_rules! unit
$name:ident
@@hint
`macro_rules! unit { ($name:ident, $suffix:literal) => { #[derive(...)] struct $name(pub f64); impl std::ops::Add for $name { type Output = $name; fn add(self, o: $name) -> $name { $name(self.0 + o.0) } } impl std::fmt::Display for $name { fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result { write!(f, "{} {}", self.0, $suffix) } } }; }`.
@@solution
macro_rules! unit {
    ($name:ident, $suffix:literal) => {
        #[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
        struct $name(pub f64);

        impl std::ops::Add for $name {
            type Output = $name;
            fn add(self, other: $name) -> $name {
                $name(self.0 + other.0)
            }
        }

        impl std::fmt::Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(f, "{} {}", self.0, $suffix)
            }
        }
    };
}

unit!(Meters, "m");
unit!(Seconds, "s");
unit!(Kilograms, "kg");

fn speed(d: Meters, t: Seconds) -> f64 {
    d.0 / t.0
}

fn main() {
    let run = Meters(400.0) + Meters(100.0);
    let time = Seconds(50.0);
    println!("{} in {} = {} m/s", run, time, speed(run, time));
    println!("{}", Kilograms(2.5) + Kilograms(0.5));
    // speed(time, run) would not compile: the types are different
}

@@flashcards
macro_rules! :: declarative macros by pattern
$x:expr :: a fragment matching an expression
$(...),* :: repetition
procedural macros :: derive, attribute and function-like
macros expand before type checking :: errors can be confusing
cargo expand :: show macro-expanded code
