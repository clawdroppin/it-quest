@@lesson m11-cargo The Cargo Ecosystem: Workspaces, Features, Versions and Publishing
@@text
Small programs live in one crate. Real projects grow into several crates, optional features, build scripts, CI and published releases. Cargo handles all of it; this lesson covers what you need to run a real Rust codebase.

## Workspaces

A **workspace** is a folder of related crates sharing one `Cargo.lock` and one `target/` directory:

```text
helpdesk/
  Cargo.toml          [workspace] members = ["core", "server", "cli"]
  core/               the business logic library (no I/O)
  server/             Axum binary depending on core
  cli/                command-line binary depending on core
```

```text
# helpdesk/Cargo.toml
[workspace]
members = ["core", "server", "cli"]
resolver = "3"

[workspace.dependencies]
serde = { version = "1", features = ["derive"] }

# server/Cargo.toml
[dependencies]
core = { path = "../core" }
serde = { workspace = true }
```

Benefits: one build, shared dependency versions, clear boundaries (the `core` crate can't accidentally depend on the web framework), and faster incremental builds. `cargo test --workspace`, `cargo build -p server`.

## Features: optional functionality

```text
[features]
default = ["json"]
json = ["dep:serde_json"]
sqlite = ["dep:rusqlite"]
full = ["json", "sqlite"]

[dependencies]
serde_json = { version = "1", optional = true }
rusqlite = { version = "0.37", optional = true }
```

```rust,norun
#[cfg(feature = "sqlite")]
pub mod sqlite_store;
```

Users opt in: `cargo build --features sqlite`, `--no-default-features`, `--all-features`. Features must be **additive**: enabling one must never break code that works without it, because Cargo unifies features across the whole dependency graph.

## Conditional compilation

`#[cfg(...)]` includes code only when a condition holds:

- `#[cfg(test)]`: test-only code.
- `#[cfg(target_os = "windows")]`, `#[cfg(unix)]`: platform-specific code (paths, services, the registry).
- `#[cfg(debug_assertions)]`: debug builds only.
- `cfg!(...)` is the expression form: `if cfg!(windows) { ... }`.

## Build scripts

A `build.rs` file runs **before** your crate compiles: compiling C code (`cc` crate), generating code (protobufs with `prost-build`), embedding a Windows icon and version info (`winresource`), or emitting `cargo:rustc-env=GIT_HASH=...` so `env!("GIT_HASH")` works in your code.

## Semantic versioning

Crates use **SemVer**: `MAJOR.MINOR.PATCH`.

- **PATCH** (1.4.2 → 1.4.3): bug fixes only.
- **MINOR** (1.4.3 → 1.5.0): new features, backwards compatible.
- **MAJOR** (1.5.0 → 2.0.0): breaking changes.
- Before 1.0, the **minor** number acts as the breaking one: 0.4 → 0.5 may break.

In `Cargo.toml`, `serde = "1.0.200"` means `^1.0.200`: any version `>= 1.0.200` and `< 2.0.0`. For `0.x`: `"0.4.3"` means `>= 0.4.3, < 0.5.0`. Other operators: `~1.2.3` (patch updates only), `=1.2.3` (exactly), `>=1.2, <1.5`. `Cargo.lock` pins exact versions; commit it for applications so builds are reproducible. `cargo update` moves within the allowed ranges; `cargo outdated` and `cargo upgrade` (cargo-edit) show and bump newer majors.

`cargo semver-checks` detects accidental breaking changes before you publish a library. The first exercise implements Cargo's version matching.

## Publishing

1. Fill in `description`, `license` (`MIT OR Apache-2.0` is the Rust convention), `repository`, `readme`, `keywords` in `Cargo.toml`.
2. Write docs with examples (they become doc tests and appear on docs.rs automatically).
3. `cargo publish --dry-run`, then `cargo publish` (needs a crates.io token). Published versions are permanent; you can only **yank** them (hide from new lockfiles).

For binaries, `cargo install your-tool` builds from source; `cargo-dist` or `cargo-binstall` provide pre-built downloads.

## Keeping a codebase healthy

| Tool | Use |
|---|---|
| `cargo fmt` | consistent formatting (`rustfmt.toml` for options) |
| `cargo clippy -- -D warnings` | lints that catch bugs and unidiomatic code |
| `cargo audit` / `cargo deny` | known vulnerabilities, licence policy, duplicate dependencies |
| `cargo machete` / `cargo udeps` | unused dependencies |
| `cargo tree -d` | why a dependency (or a duplicate version) is included |
| `cargo llvm-cov` | test coverage |
| `cargo nextest` | a faster, clearer test runner |
| MSRV (`rust-version = "1.85"`) | declare the oldest supported compiler |

A typical CI pipeline (GitHub Actions) runs fmt check, clippy, tests on Windows/macOS/Linux, and `cargo deny`, then builds release binaries on tags. The second exercise simulates feature resolution.

@@quiz
What does `serde = "1.0.200"` in Cargo.toml allow?
- Exactly 1.0.200
* Any version from 1.0.200 up to (but not including) 2.0.0
- Any version at all
- Only 1.0.x patch updates
? A bare version means the caret requirement `^1.0.200`.

@@quiz
Why must Cargo features be additive?
* Cargo unifies features across the dependency graph, so a feature enabled by any crate is enabled for everyone
- To make builds faster
- Because features can't be disabled
- crates.io rejects others
? A feature that removes functionality would break other crates that rely on it.

@@quiz
What is a workspace for?
- Storing editor settings
* Several related crates sharing one lockfile and target directory, built and tested together
- Publishing to crates.io
- Running build scripts
? E.g. a core library, a server and a CLI.

@@quiz
Can you delete a published crate version from crates.io?
- Yes, any time
* No; you can only yank it so new projects don't pick it, while existing lockfiles still work
- Only within 24 hours
- Only for 0.x versions
? Builds that depend on it must keep working.

@@drill
What do you call a set of crates that share one Cargo.lock and target folder?
= workspace
= a workspace
= cargo workspace
? [workspace] members = [...] in the root Cargo.toml.

@@exercise Cargo's version matching
Write SemVer parsing and Cargo's **caret** matching:

- `parse(v: &str) -> Option<(u64, u64, u64)>` for `MAJOR.MINOR.PATCH` (exactly three numeric parts).
- `caret_matches(req: &str, v: &str) -> bool`: `req` like `"1.2.3"` (caret implied). The version must be `>= req` and must not change the leftmost non-zero part: for `1.x.y` the major must be equal; for `0.x.y` with x > 0 the minor must be equal; for `0.0.z` the patch must be equal.
- `best_match(req: &str, available: &[&str]) -> Option<String>`: the highest available version that matches.
@@starter
fn parse(v: &str) -> Option<(u64, u64, u64)> {
    todo!()
}

fn caret_matches(req: &str, v: &str) -> bool {
    todo!()
}

fn best_match(req: &str, available: &[&str]) -> Option<String> {
    todo!()
}

fn main() {
    let versions = ["0.4.9", "1.0.150", "1.0.201", "1.2.0", "2.0.0", "0.5.1"];
    println!("{:?}", best_match("1.0.200", &versions));
    println!("{:?}", best_match("0.4.3", &versions));
    println!("{:?}", best_match("3.0.0", &versions));
}
@@expected
Some("1.2.0")
Some("0.4.9")
None
@@tests
#[test]
fn checks() {
    assert_eq!(parse("10.2.33"), Some((10, 2, 33)));
    assert_eq!(parse("1.2"), None);
    assert_eq!(parse("1.2.x"), None);
    assert!(caret_matches("1.2.3", "1.9.0"));
    assert!(!caret_matches("1.2.3", "1.2.2"));
    assert!(!caret_matches("1.2.3", "2.0.0"));
    assert!(caret_matches("0.4.3", "0.4.10"));
    assert!(!caret_matches("0.4.3", "0.5.0"));
    assert!(caret_matches("0.0.3", "0.0.3"));
    assert!(!caret_matches("0.0.3", "0.0.4"));
}
@@requires
fn caret_matches
.split(
@@hint
Parse with `let parts: Vec<&str> = v.split('.').collect();` and check there are three, then parse each with `.parse().ok()?`. Tuples compare lexicographically, so `ver >= req` works directly. Then `if req.0 > 0 { ver.0 == req.0 } else if req.1 > 0 { ver.0 == 0 && ver.1 == req.1 } else { ver == req }`.
@@solution
fn parse(v: &str) -> Option<(u64, u64, u64)> {
    let parts: Vec<&str> = v.split('.').collect();
    if parts.len() != 3 {
        return None;
    }
    Some((parts[0].parse().ok()?, parts[1].parse().ok()?, parts[2].parse().ok()?))
}

fn caret_matches(req: &str, v: &str) -> bool {
    let (Some(r), Some(ver)) = (parse(req), parse(v)) else {
        return false;
    };
    if ver < r {
        return false;
    }
    if r.0 > 0 {
        ver.0 == r.0
    } else if r.1 > 0 {
        ver.0 == 0 && ver.1 == r.1
    } else {
        ver == r
    }
}

fn best_match(req: &str, available: &[&str]) -> Option<String> {
    available
        .iter()
        .filter(|v| caret_matches(req, v))
        .max_by_key(|v| parse(v))
        .map(|v| v.to_string())
}

fn main() {
    let versions = ["0.4.9", "1.0.150", "1.0.201", "1.2.0", "2.0.0", "0.5.1"];
    println!("{:?}", best_match("1.0.200", &versions));
    println!("{:?}", best_match("0.4.3", &versions));
    println!("{:?}", best_match("3.0.0", &versions));
}

@@exercise Resolve features
Cargo features can enable other features. Given a feature table and the features requested on the command line, compute everything that ends up enabled:

- `resolve(table: &BTreeMap<&str, Vec<&str>>, requested: &[&str], default: bool) -> Result<BTreeSet<String>, String>`
- Start from the requested features, plus `"default"` if `default` is true.
- Repeatedly add every feature that an enabled feature lists (a feature may list `dep:name`, which enables the optional dependency: keep it in the result as `dep:name`).
- Requesting a feature that isn't in the table (and isn't a `dep:`) is an error: `unknown feature "x"`.
- Cycles must not loop forever.
@@starter
use std::collections::{BTreeMap, BTreeSet};

fn resolve(table: &BTreeMap<&str, Vec<&str>>, requested: &[&str], default: bool) -> Result<BTreeSet<String>, String> {
    todo!()
}

fn main() {
    let mut table = BTreeMap::new();
    table.insert("default", vec!["json"]);
    table.insert("json", vec!["dep:serde_json"]);
    table.insert("sqlite", vec!["dep:rusqlite", "storage"]);
    table.insert("storage", vec![]);
    table.insert("full", vec!["json", "sqlite"]);
    println!("{:?}", resolve(&table, &[], true));
    println!("{:?}", resolve(&table, &["full"], false));
    println!("{:?}", resolve(&table, &["postgres"], false));
}
@@expected
Ok({"default", "dep:serde_json", "json"})
Ok({"dep:rusqlite", "dep:serde_json", "full", "json", "sqlite", "storage"})
Err("unknown feature \"postgres\"")
@@tests
#[test]
fn checks() {
    let mut t = BTreeMap::new();
    t.insert("a", vec!["b"]);
    t.insert("b", vec!["a"]);
    let r = resolve(&t, &["a"], false).unwrap();
    assert_eq!(r.len(), 2, "a cycle resolves without looping");
    assert!(resolve(&t, &[], false).unwrap().is_empty());
    assert!(resolve(&t, &[], true).is_err(), "no default feature defined");
}
@@requires
BTreeSet
.iter(
@@hint
Use a work list: `let mut todo: Vec<String> = requested.iter().map(|s| s.to_string()).collect();` plus `"default"`. Pop items; skip ones already in the result; `dep:` items are inserted without lookup; otherwise look up the table (error if missing), insert, and push its children.
@@solution
use std::collections::{BTreeMap, BTreeSet};

fn resolve(table: &BTreeMap<&str, Vec<&str>>, requested: &[&str], default: bool) -> Result<BTreeSet<String>, String> {
    let mut todo: Vec<String> = requested.iter().map(|s| s.to_string()).collect();
    if default {
        todo.push("default".to_string());
    }
    let mut enabled = BTreeSet::new();
    while let Some(f) = todo.pop() {
        if enabled.contains(&f) {
            continue;
        }
        if f.starts_with("dep:") {
            enabled.insert(f);
            continue;
        }
        let children = table.get(f.as_str()).ok_or(format!("unknown feature \"{f}\""))?;
        todo.extend(children.iter().map(|c| c.to_string()));
        enabled.insert(f);
    }
    Ok(enabled)
}

fn main() {
    let mut table = BTreeMap::new();
    table.insert("default", vec!["json"]);
    table.insert("json", vec!["dep:serde_json"]);
    table.insert("sqlite", vec!["dep:rusqlite", "storage"]);
    table.insert("storage", vec![]);
    table.insert("full", vec!["json", "sqlite"]);
    println!("{:?}", resolve(&table, &[], true));
    println!("{:?}", resolve(&table, &["full"], false));
    println!("{:?}", resolve(&table, &["postgres"], false));
}

@@flashcards
workspace :: several crates sharing one Cargo.lock and target
cargo features :: optional functionality flags
cargo publish :: upload to crates.io
cargo audit :: check for known vulnerabilities
build.rs :: build script run before compiling
MSRV :: minimum supported Rust version
