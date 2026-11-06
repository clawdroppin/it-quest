@@lesson m11-observability Error Design and Observability: thiserror, anyhow, tracing
@@text
When a program runs on a server at 3 a.m., two things decide how fast someone can fix it: **errors** that explain what went wrong and why, and **logs, metrics and traces** that show what the program was doing. This lesson covers both the Rust way.

## Libraries vs applications

| | Libraries | Applications |
|---|---|---|
| Error type | a specific `enum` callers can `match` on | one catch-all type, add context, report |
| Crate | **thiserror** | **anyhow** (or `eyre` / `color-eyre`) |
| Goal | let the caller decide what to do | tell a human what happened |

```rust,norun
// library: thiserror generates Display, Error and From for you
#[derive(Debug, thiserror::Error)]
pub enum ConfigError {
    #[error("cannot read {path}")]
    Io { path: String, #[source] source: std::io::Error },
    #[error("line {line}: {msg}")]
    Parse { line: usize, msg: String },
    #[error("missing key `{0}`")]
    Missing(String),
}

// application: anyhow adds context and prints the whole chain
use anyhow::Context;
fn main() -> anyhow::Result<()> {
    let cfg = load("app.toml").context("starting server")?;
    Ok(())
}
// Error: starting server
// Caused by:
//     0: cannot read app.toml
//     1: The system cannot find the file specified. (os error 2)
```

## The source chain

`std::error::Error` has a `source()` method returning the **underlying** error. Good errors form a chain: *what we were doing* → *what failed* → *the OS reason*. Two rules:

1. Each layer's `Display` describes **only its own** level ("cannot read app.toml"), not the cause.
2. The cause is returned from `source()`, and the **reporter** walks the chain to print it all.

Printing the cause in both places produces messages like "cannot read app.toml: file not found: file not found". The first exercise implements a chain by hand, exactly what thiserror generates.

## Panics vs errors

- `Result` for anything that can go wrong at run time: bad input, missing files, network failures.
- `panic!` / `unwrap` / `expect` only for **bugs** (a broken invariant) or in tests and prototypes. `expect("config was validated above")` documents why it can't fail.
- In services, a panic in one request handler shouldn't kill the server: Tokio catches panics per task, and `std::panic::catch_unwind` exists for edges like FFI.
- `RUST_BACKTRACE=1` shows where a panic happened; `std::backtrace::Backtrace::capture()` attaches one to your errors.

## Logging with tracing

The **tracing** crate is the standard for logs in Rust. It records **events** (log lines) with **structured fields** and **spans** (periods of work like "handling request 42") so events inside know their context.

```rust,norun
use tracing::{info, warn, instrument};

#[instrument(skip(db), fields(user = %user_id))]
async fn delete_account(db: &Db, user_id: u64) -> Result<(), AppError> {
    info!(rows = 3, "deleting account data");
    if let Err(e) = db.purge(user_id).await {
        warn!(error = %e, "purge failed, will retry");
    }
    Ok(())
}

fn main() {
    tracing_subscriber::fmt()
        .with_env_filter("info,sqlx=warn,myapp::db=debug")   // or the RUST_LOG variable
        .json()                                                  // one JSON object per line
        .init();
}
```

- **Levels**: `error` > `warn` > `info` > `debug` > `trace`. Production usually runs at `info`.
- **Structured fields** (`rows = 3`, `user = %id`) can be searched and graphed in tools like Loki, Elasticsearch, Datadog or Azure Monitor: "all errors for user 42" becomes a query, not a grep.
- **Filters** like `info,sqlx=warn,myapp::db=debug` set the level per module (the second exercise implements this).
- Never log secrets: passwords, tokens, full card numbers, personal data you don't need.
- The older `log` crate (with `env_logger`) is simpler and still common in libraries; tracing can consume `log` records.

## Metrics and traces

- **Metrics** are numbers over time: requests per second, error rate, latency percentiles, queue length. The `metrics` crate with a Prometheus exporter, scraped and graphed in Grafana.
- **Distributed tracing** follows one request across services: **OpenTelemetry** (`tracing-opentelemetry` + `opentelemetry-otlp`) exports spans to Jaeger, Tempo, Honeycomb or Azure Application Insights.
- **Health endpoints**: `/healthz` (process alive) and `/readyz` (dependencies reachable) for Kubernetes and load balancers.

Rule of thumb: logs tell you **what happened**, metrics tell you **that something is wrong**, traces tell you **where**.

@@quiz
Which crate is typically used for a library's error enum?
* thiserror
- anyhow
- tracing
- serde
? anyhow is for applications that just report errors.

@@quiz
Why shouldn't an error's Display message also include its source's message?
- It's not allowed
* The reporter walks the source chain and prints each cause, so it would appear twice
- Display can't take arguments
- It makes the binary larger
? Each layer describes only itself.

@@quiz
What do structured log fields like `user_id = 42` make possible?
- Faster compilation
* Searching, filtering and graphing logs by field in a log platform
- Encryption of logs
- Automatic error fixing
? A field is data, not just text inside a message.

@@quiz
When is `unwrap()`/`expect()` appropriate in production code?
- For network errors
- For user input
* When failure would mean a bug (a broken invariant), with `expect` explaining why it can't happen
- Never in any situation
? Recoverable failures belong in `Result`.

@@drill
Which crate derives error types for libraries?
= thiserror
? Applications often use anyhow instead.

@@flashcards
thiserror :: derive macro crate for precise error enums in libraries
anyhow :: catch-all error type with context for applications
source() :: the method on `Error` returning the underlying cause
Span (tracing) :: a named period of work that gives context to events inside it
Structured logging :: logs as key-value fields instead of only free text
RUST_LOG / EnvFilter :: per-module log level configuration like `info,sqlx=warn`
OpenTelemetry :: vendor-neutral standard for traces, metrics and logs
Metric :: a number measured over time, like requests per second

@@exercise An error chain by hand
Write what `thiserror` would generate, then a reporter.

- `ConfigError::Parse { line, source: ParseIntError }` displays as `line N: invalid number` and its `source()` is the `ParseIntError`. `ConfigError::Missing(key)` displays as `missing key `KEY`` (with backticks) and has no source.
- `AppError { context: String, source: ConfigError }` displays as its context; its `source()` is the `ConfigError`.
- `parse_port(text) -> Result<u16, ConfigError>`: find the line `port = VALUE` (lines numbered from 1, spaces around `=` optional) and parse the value; a missing line is `Missing("port")`.
- `report(e: &dyn Error) -> String`: the message and every source, joined with `: `.
@@starter
use std::error::Error;
use std::fmt;
use std::num::ParseIntError;

#[derive(Debug)]
enum ConfigError {
    Parse { line: usize, source: ParseIntError },
    Missing(String),
}

#[derive(Debug)]
struct AppError {
    context: String,
    source: ConfigError,
}

// impl fmt::Display and Error for both types

fn parse_port(text: &str) -> Result<u16, ConfigError> {
    todo!()
}

fn report(e: &dyn Error) -> String {
    todo!()
}

fn start(config: &str) -> Result<u16, AppError> {
    parse_port(config).map_err(|source| AppError { context: "starting server".into(), source })
}

fn main() {
    println!("{:?}", start("name = web\nport = 8080"));
    for cfg in ["name = web\nport=80x", "name = web"] {
        let e = start(cfg).unwrap_err();
        println!("{}", report(&e));
    }
}
@@expected
Ok(8080)
starting server: line 2: invalid number: invalid digit found in string
starting server: missing key `port`
@@tests
#[test]
fn checks() {
    let e = parse_port("port = 99999").unwrap_err();
    assert_eq!(e.to_string(), "line 1: invalid number");
    assert!(e.source().is_some());
    assert!(ConfigError::Missing("x".into()).source().is_none());
    assert_eq!(parse_port("  port=  443  ").unwrap(), 443);
}
@@requires
fn source(&self)
@@hint
`impl Error for ConfigError { fn source(&self) -> Option<&(dyn Error + 'static)> { match self { ConfigError::Parse { source, .. } => Some(source), ConfigError::Missing(_) => None } } }`. For `report`, start with `let mut out = e.to_string(); let mut cur = e.source(); while let Some(s) = cur { out += ": "; out += &s.to_string(); cur = s.source(); }`.
@@solution
use std::error::Error;
use std::fmt;
use std::num::ParseIntError;

#[derive(Debug)]
enum ConfigError {
    Parse { line: usize, source: ParseIntError },
    Missing(String),
}

#[derive(Debug)]
struct AppError {
    context: String,
    source: ConfigError,
}

impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ConfigError::Parse { line, .. } => write!(f, "line {line}: invalid number"),
            ConfigError::Missing(key) => write!(f, "missing key `{key}`"),
        }
    }
}

impl Error for ConfigError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            ConfigError::Parse { source, .. } => Some(source),
            ConfigError::Missing(_) => None,
        }
    }
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.context)
    }
}

impl Error for AppError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(&self.source)
    }
}

fn parse_port(text: &str) -> Result<u16, ConfigError> {
    for (i, line) in text.lines().enumerate() {
        if let Some((key, value)) = line.split_once('=') {
            if key.trim() == "port" {
                return value.trim().parse().map_err(|source| ConfigError::Parse { line: i + 1, source });
            }
        }
    }
    Err(ConfigError::Missing("port".into()))
}

fn report(e: &dyn Error) -> String {
    let mut out = e.to_string();
    let mut cur = e.source();
    while let Some(s) = cur {
        out += ": ";
        out += &s.to_string();
        cur = s.source();
    }
    out
}

fn start(config: &str) -> Result<u16, AppError> {
    parse_port(config).map_err(|source| AppError { context: "starting server".into(), source })
}

fn main() {
    println!("{:?}", start("name = web\nport = 8080"));
    for cfg in ["name = web\nport=80x", "name = web"] {
        let e = start(cfg).unwrap_err();
        println!("{}", report(&e));
    }
}

@@exercise Structured logs and level filters
Build two pieces of a logging system.

- `logfmt(level, msg, fields) -> String` formats one event in **logfmt**: `level=info msg="user logged in" user=maya`. A value is wrapped in double quotes if it is empty or contains a space, `=` or `"`; inside quotes, `"` becomes `\"`.
- `Filter::parse("warn,db=debug,http::auth=trace")`: entries without `=` set the default level (default is `Info` if none given); `target=level` entries set a level for a module path. Unknown level names are an error: `unknown level "loud"`.
- `Filter::enabled(&self, target, level) -> bool`: use the rule for the **longest** module path that equals `target` or is a prefix of it followed by `::`; otherwise the default. An event is enabled if its level is at least as severe as the rule.

`Level` is ordered from least to most severe: `Trace < Debug < Info < Warn < Error`.
@@starter
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
enum Level {
    Trace,
    Debug,
    Info,
    Warn,
    Error,
}

fn logfmt(level: Level, msg: &str, fields: &[(&str, &str)]) -> String {
    todo!()
}

#[derive(Debug)]
struct Filter {
    default: Level,
    rules: Vec<(String, Level)>,
}

impl Filter {
    fn parse(spec: &str) -> Result<Filter, String> {
        todo!()
    }

    fn enabled(&self, target: &str, level: Level) -> bool {
        todo!()
    }
}

fn main() {
    println!("{}", logfmt(Level::Info, "user logged in", &[("user", "maya"), ("ip", "10.0.0.7")]));
    println!("{}", logfmt(Level::Warn, "slow", &[("query", "SELECT \"x\""), ("note", "")]));
    let f = Filter::parse("warn,db=debug,http::auth=trace").unwrap();
    for (target, level) in [("app", Level::Info), ("db::pool", Level::Debug), ("dbx", Level::Debug), ("http::auth::jwt", Level::Trace), ("http", Level::Error)] {
        println!("{target} {level:?} -> {}", f.enabled(target, level));
    }
    println!("{:?}", Filter::parse("loud").map(|_| ()));
}
@@expected
level=info msg="user logged in" user=maya ip=10.0.0.7
level=warn msg=slow query="SELECT \"x\"" note=""
app Info -> false
db::pool Debug -> true
dbx Debug -> false
http::auth::jwt Trace -> true
http Error -> true
Err("unknown level \"loud\"")
@@tests
#[test]
fn checks() {
    let f = Filter::parse("").unwrap();
    assert!(f.enabled("x", Level::Info));
    assert!(!f.enabled("x", Level::Debug));
    let f = Filter::parse("a=error,a::b=debug").unwrap();
    assert!(f.enabled("a::b::c", Level::Debug));
    assert!(!f.enabled("a::c", Level::Warn));
    assert_eq!(logfmt(Level::Error, "a=b", &[]), "level=error msg=\"a=b\"");
}
@@requires
fn enabled
@@hint
A helper `fn quote(v: &str) -> String` decides on quoting. Parse a level with a `match` on the lower-case name. In `enabled`, filter rules where `target == path || target.starts_with(&format!("{path}::"))`, take `max_by_key(|(p, _)| p.len())`, and compare `level >= rule`.
@@solution
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
enum Level {
    Trace,
    Debug,
    Info,
    Warn,
    Error,
}

fn quote(v: &str) -> String {
    if v.is_empty() || v.contains([' ', '=', '"']) {
        format!("\"{}\"", v.replace('"', "\\\""))
    } else {
        v.to_string()
    }
}

fn logfmt(level: Level, msg: &str, fields: &[(&str, &str)]) -> String {
    let mut out = format!("level={} msg={}", format!("{level:?}").to_lowercase(), quote(msg));
    for (k, v) in fields {
        out += &format!(" {k}={}", quote(v));
    }
    out
}

fn parse_level(s: &str) -> Result<Level, String> {
    match s.trim().to_lowercase().as_str() {
        "trace" => Ok(Level::Trace),
        "debug" => Ok(Level::Debug),
        "info" => Ok(Level::Info),
        "warn" => Ok(Level::Warn),
        "error" => Ok(Level::Error),
        other => Err(format!("unknown level \"{other}\"")),
    }
}

#[derive(Debug)]
struct Filter {
    default: Level,
    rules: Vec<(String, Level)>,
}

impl Filter {
    fn parse(spec: &str) -> Result<Filter, String> {
        let mut f = Filter { default: Level::Info, rules: vec![] };
        for part in spec.split(',').map(str::trim).filter(|p| !p.is_empty()) {
            match part.split_once('=') {
                Some((path, lvl)) => f.rules.push((path.trim().to_string(), parse_level(lvl)?)),
                None => f.default = parse_level(part)?,
            }
        }
        Ok(f)
    }

    fn enabled(&self, target: &str, level: Level) -> bool {
        let rule = self
            .rules
            .iter()
            .filter(|(p, _)| target == p || target.starts_with(&format!("{p}::")))
            .max_by_key(|(p, _)| p.len())
            .map(|(_, l)| *l)
            .unwrap_or(self.default);
        level >= rule
    }
}

fn main() {
    println!("{}", logfmt(Level::Info, "user logged in", &[("user", "maya"), ("ip", "10.0.0.7")]));
    println!("{}", logfmt(Level::Warn, "slow", &[("query", "SELECT \"x\""), ("note", "")]));
    let f = Filter::parse("warn,db=debug,http::auth=trace").unwrap();
    for (target, level) in [("app", Level::Info), ("db::pool", Level::Debug), ("dbx", Level::Debug), ("http::auth::jwt", Level::Trace), ("http", Level::Error)] {
        println!("{target} {level:?} -> {}", f.enabled(target, level));
    }
    println!("{:?}", Filter::parse("loud").map(|_| ()));
}
