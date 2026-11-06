@@lesson m10-lifetimes Lifetimes in Depth, Borrowed Data and Cow
@@text
Module 3 introduced lifetimes for functions like `longest`. Real programs go further: structs that borrow from a buffer, parsers that return slices of their input without copying, and functions that only sometimes need to allocate. This lesson makes lifetimes routine.

## What a lifetime annotation says

A lifetime parameter `'a` doesn't change how long anything lives. It **describes a relationship** the compiler checks: "the returned reference is valid as long as *this* input is". The borrow checker then makes sure no reference outlives the data it points to.

## The three elision rules

You rarely write lifetimes because the compiler fills them in using three rules:

1. Each reference parameter gets its own lifetime: `fn f(a: &str, b: &str)` becomes `fn f<'a, 'b>(a: &'a str, b: &'b str)`.
2. If there's **exactly one** input lifetime, it's used for all outputs: `fn first_word(s: &str) -> &str` returns something borrowed from `s`.
3. If there's a `&self` or `&mut self`, its lifetime is used for all outputs: methods return things borrowed from `self`.

When the rules don't decide (two reference inputs, a reference output, no `self`), you must annotate. That's the whole reason `longest(a, b)` needs `'a`.

## Structs that borrow

A struct holding a reference needs a lifetime parameter, which says "a `Token<'a>` can't outlive the text it points into":

```rust
#[derive(Debug)]
struct Token<'a> {
    kind: &'static str,
    text: &'a str,
}

fn tokenize(input: &str) -> Vec<Token<'_>> {
    let mut out = Vec::new();
    for word in input.split_whitespace() {
        let kind = if word.chars().all(|c| c.is_ascii_digit()) { "number" } else { "word" };
        out.push(Token { kind, text: word });
    }
    out
}

fn main() {
    let source = String::from("move 10 steps");
    let tokens = tokenize(&source);
    println!("{:?}", tokens);
    // drop(source);  // ❌ would fail: tokens still borrow from source
}
```

`'_` is the **anonymous lifetime**: "a lifetime is here, elision figures out which". The payoff of borrowing structs is **zero-copy parsing**: tokens are just pointers into the original text, so parsing a 100 MB log allocates almost nothing.

When should a struct own (`String`) instead of borrow (`&'a str`)? Own data that must outlive its source or be stored long-term (app state, things sent to other threads). Borrow for short-lived views: parsers, iterators, function-local helpers.

## Several lifetimes

Sometimes outputs relate to only one input:

```rust,norun
fn after_prefix<'a, 'b>(text: &'a str, prefix: &'b str) -> &'a str {
    text.strip_prefix(prefix).unwrap_or(text)
}
```

The result borrows from `text` only, so callers can drop `prefix` immediately. Using one `'a` for both would be needlessly restrictive.

## `'static`

`'static` means "can live for the whole program". String literals are `&'static str` (they're baked into the binary). As a **bound**, `T: 'static` means "`T` contains no non-static references", so it owns all its data. That's why `thread::spawn` requires `'static` closures: the thread might outlive the caller, so the closure can't borrow local variables (use `move`, or scoped threads in the next lesson).

`'static` doesn't mean "lives forever"; an owned `String` satisfies `T: 'static` and is still dropped normally.

## Lifetimes in `impl` blocks and methods

```rust,norun
impl<'a> Token<'a> {
    fn text(&self) -> &'a str { self.text }      // borrowed from the source, not from self
}
```

Returning `&'a str` (rather than relying on rule 3, which would tie it to `&self`) lets callers keep the text after the `Token` itself is gone.

## `Cow`: borrow when you can, own when you must

`std::borrow::Cow<'a, str>` ("clone on write") is either `Cow::Borrowed(&'a str)` or `Cow::Owned(String)`. Functions that **usually** return their input unchanged but **sometimes** modify it can avoid allocating in the common case:

```rust
use std::borrow::Cow;

fn escape_html(input: &str) -> Cow<'_, str> {
    if !input.contains(['<', '>', '&']) {
        return Cow::Borrowed(input);            // no allocation
    }
    let mut out = String::with_capacity(input.len() + 8);
    for c in input.chars() {
        match c {
            '<' => out.push_str("&lt;"),
            '>' => out.push_str("&gt;"),
            '&' => out.push_str("&amp;"),
            _ => out.push(c),
        }
    }
    Cow::Owned(out)
}

fn main() {
    for s in ["plain text", "a < b & c"] {
        let e = escape_html(s);
        let kind = if matches!(e, Cow::Borrowed(_)) { "borrowed" } else { "owned" };
        println!("{e} ({kind})");
    }
}
```

A `Cow<str>` derefs to `&str`, so callers use it like any string; `.into_owned()` gives a `String` when needed. `Cow` appears in serde, path handling (`to_string_lossy()` returns a `Cow`) and string normalisers.

## Common lifetime errors and fixes

| Error | Usual fix |
|---|---|
| "borrowed value does not live long enough" | keep the owner alive longer, or make the struct own its data |
| "missing lifetime specifier" | add `<'a>` to the struct/function and tie the output to the right input |
| "cannot return reference to local variable" | return an owned value (`String`, `Vec`) instead |
| "`x` does not live long enough... argument requires that `x` is borrowed for `'static`" | a thread or a boxed closure needs owned data: use `move`, clone, `Arc`, or scoped threads |
| "cannot borrow as mutable because it is also borrowed as immutable" | shorten the first borrow (finish using it before mutating), or split the struct's fields |

## Higher-ranked bounds (a glimpse)

`for<'a> Fn(&'a str) -> &'a str` reads "a closure that works for **any** lifetime the caller gives it". You'll see it in error messages and advanced APIs; the compiler usually infers it from `Fn(&str) -> &str`.

@@quiz
What does `fn first_word(s: &str) -> &str` return, according to elision?
* A reference that borrows from `s`, since there is exactly one input lifetime
- A `'static` reference
- A newly allocated string
- It doesn't compile without annotations
? Rule 2: one input lifetime is used for the output.

@@quiz
Why does a struct `Token<'a> { text: &'a str }` need a lifetime parameter?
- For speed
* To say a Token can't outlive the text it points into
- Because all structs need one
- So that it can be sent to threads
? The compiler then checks every use of Token against its source.

@@quiz
What does `T: 'static` require?
- That the value lives until the program ends
* That `T` holds no borrowed (non-static) references; owned data like `String` qualifies
- That `T` is a string literal
- That `T` is `Copy`
? Owned values satisfy `'static` and are still dropped normally.

@@quiz
When is `Cow<str>` a good return type?
- Always
* When a function usually returns its input unchanged and only sometimes needs to build a new string
- When the result must be shared between threads
- Never; use `String`
? It avoids allocating in the common case.

@@drill
Which type holds either borrowed or owned data, cloning only when needed?
= Cow
= Cow<str>
= std::borrow::Cow
? Clone on write.

@@exercise A zero-copy key=value parser
Parse lines like `name = Ferris` into `Pair<'a> { key: &'a str, value: &'a str }` that **borrow** from the input:

- `parse_line(line: &str) -> Option<Pair<'_>>`: split on the **first** `=`, trim both sides; return `None` if there's no `=`, the key is empty, or the line (trimmed) starts with `#` (a comment).
- `parse_all(text: &str) -> Vec<Pair<'_>>`: parse every line, skipping the ones that return `None`.
- `get<'a>(pairs: &[Pair<'a>], key: &str) -> Option<&'a str>`: the value for a key (the **last** one wins). Note the result borrows from the original text, not from `key`.
@@starter
#[derive(Debug, PartialEq)]
struct Pair<'a> {
    key: &'a str,
    value: &'a str,
}

fn parse_line(line: &str) -> Option<Pair<'_>> {
    todo!()
}

fn parse_all(text: &str) -> Vec<Pair<'_>> {
    todo!()
}

fn get<'a>(pairs: &[Pair<'a>], key: &str) -> Option<&'a str> {
    todo!()
}

fn main() {
    let config = String::from("# app settings\nname = Ferris\ntheme=dark\nbroken line\ntheme = light\nurl = http://x?a=b\n");
    let pairs = parse_all(&config);
    println!("{}", pairs.len());
    let wanted = String::from("theme");
    let theme = get(&pairs, &wanted);
    drop(wanted); // fine: the result borrows from config, not from the key
    println!("{:?} {:?} {:?}", theme, get(&pairs, "url"), get(&pairs, "missing"));
}
@@expected
4
Some("light") Some("http://x?a=b") None
@@tests
#[test]
fn checks() {
    assert_eq!(parse_line("a=1"), Some(Pair { key: "a", value: "1" }));
    assert_eq!(parse_line("  spaced   =  out  "), Some(Pair { key: "spaced", value: "out" }));
    assert_eq!(parse_line("empty ="), Some(Pair { key: "empty", value: "" }));
    assert_eq!(parse_line("= no key"), None);
    assert_eq!(parse_line("no equals"), None);
    assert_eq!(parse_line("  # a=1"), None);
    assert!(parse_all("").is_empty());
}
@@requires
split_once('=')
@@hint
`let (k, v) = line.split_once('=')?;` returns early with `None` if there's no `=`. Trim both, check the key isn't empty. For `get`, loop over the pairs (or use `.iter().rev().find(...)`) and return `Some(p.value)`.
@@solution
#[derive(Debug, PartialEq)]
struct Pair<'a> {
    key: &'a str,
    value: &'a str,
}

fn parse_line(line: &str) -> Option<Pair<'_>> {
    if line.trim_start().starts_with('#') {
        return None;
    }
    let (k, v) = line.split_once('=')?;
    let key = k.trim();
    if key.is_empty() {
        return None;
    }
    Some(Pair { key, value: v.trim() })
}

fn parse_all(text: &str) -> Vec<Pair<'_>> {
    text.lines().filter_map(parse_line).collect()
}

fn get<'a>(pairs: &[Pair<'a>], key: &str) -> Option<&'a str> {
    pairs.iter().rev().find(|p| p.key == key).map(|p| p.value)
}

fn main() {
    let config = String::from("# app settings\nname = Ferris\ntheme=dark\nbroken line\ntheme = light\nurl = http://x?a=b\n");
    let pairs = parse_all(&config);
    println!("{}", pairs.len());
    let wanted = String::from("theme");
    let theme = get(&pairs, &wanted);
    drop(wanted); // fine: the result borrows from config, not from the key
    println!("{:?} {:?} {:?}", theme, get(&pairs, "url"), get(&pairs, "missing"));
}

@@exercise Normalise without allocating
Write `normalize(name: &str) -> Cow<'_, str>` for file names: if the name contains no spaces and no uppercase letters, return it **borrowed**; otherwise return an owned copy that is lowercase with spaces replaced by `-`. Then `count_allocations(names: &[&str]) -> usize` returns how many of the names needed an owned copy.
@@starter
use std::borrow::Cow;

fn normalize(name: &str) -> Cow<'_, str> {
    todo!()
}

fn count_allocations(names: &[&str]) -> usize {
    todo!()
}

fn main() {
    let names = ["report.pdf", "My Photo.JPG", "notes-2026.txt", "Read Me"];
    for n in names {
        println!("{}", normalize(n));
    }
    println!("{} of {} allocated", count_allocations(&names), names.len());
}
@@expected
report.pdf
my-photo.jpg
notes-2026.txt
read-me
2 of 4 allocated
@@tests
#[test]
fn checks() {
    assert!(matches!(normalize("ok.txt"), Cow::Borrowed("ok.txt")));
    assert!(matches!(normalize("A.txt"), Cow::Owned(_)));
    assert_eq!(normalize("A B  C"), "a-b--c");
    assert_eq!(count_allocations(&[]), 0);
    assert_eq!(count_allocations(&["x", "y z"]), 1);
}
@@requires
Cow::Borrowed
Cow::Owned
@@hint
Check with `name.contains(' ') || name.chars().any(|c| c.is_uppercase())`. Build the owned version with `name.to_lowercase().replace(' ', "-")`. Count with `.filter(|n| matches!(normalize(n), Cow::Owned(_))).count()`.
@@solution
use std::borrow::Cow;

fn normalize(name: &str) -> Cow<'_, str> {
    if name.contains(' ') || name.chars().any(|c| c.is_uppercase()) {
        Cow::Owned(name.to_lowercase().replace(' ', "-"))
    } else {
        Cow::Borrowed(name)
    }
}

fn count_allocations(names: &[&str]) -> usize {
    names.iter().filter(|n| matches!(normalize(n), Cow::Owned(_))).count()
}

fn main() {
    let names = ["report.pdf", "My Photo.JPG", "notes-2026.txt", "Read Me"];
    for n in names {
        println!("{}", normalize(n));
    }
    println!("{} of {} allocated", count_allocations(&names), names.len());
}

@@flashcards
struct with a reference :: needs a lifetime parameter
elision rule 1 :: each input reference gets its own lifetime
elision rule 2 :: one input lifetime is used for all outputs
elision rule 3 :: with &self, the output gets self's lifetime
T: 'a :: T outlives 'a
'static bound :: owns its data or only holds 'static references
