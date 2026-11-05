@@lesson m11-databases Databases in Rust: SQLite, Postgres, Repositories and Migrations
@@text
Almost every real application stores data in a database. Rust has excellent drivers, and its type system helps keep SQL and Rust structs in sync. This lesson covers the main crates, the patterns that keep database code testable, and migrations.

## The crate landscape

| Crate | Style | Notes |
|---|---|---|
| **rusqlite** | synchronous SQLite | simple and fast; perfect for desktop apps (Tauri), CLIs and tests; can bundle SQLite (`features = ["bundled"]`) |
| **sqlx** | async SQL for Postgres, MySQL, SQLite | write real SQL; `query!` / `query_as!` macros **check queries against the database at compile time**; built-in migrations and connection pools |
| **diesel** | synchronous ORM / query builder | type-safe query DSL, schema generated from migrations |
| **sea-orm** | async ORM built on sqlx | ActiveRecord-like entities, popular with Axum |
| **redb**, **sled**, **rocksdb** | embedded key-value stores | when you don't need SQL |

## rusqlite in practice

```rust,norun
use rusqlite::{Connection, params};

#[derive(Debug)]
struct Asset { id: i64, tag: String, owner: Option<String> }

fn main() -> rusqlite::Result<()> {
    let conn = Connection::open("assets.db")?;            // or Connection::open_in_memory()
    conn.execute_batch("
        PRAGMA foreign_keys = ON;
        CREATE TABLE IF NOT EXISTS assets (
            id    INTEGER PRIMARY KEY,
            tag   TEXT NOT NULL UNIQUE,
            owner TEXT
        );")?;
    conn.execute("INSERT INTO assets (tag, owner) VALUES (?1, ?2)", params!["LT-0042", "maya"])?;

    let mut stmt = conn.prepare("SELECT id, tag, owner FROM assets WHERE owner IS ?1")?;
    let rows = stmt.query_map(params!["maya"], |r| {
        Ok(Asset { id: r.get(0)?, tag: r.get(1)?, owner: r.get(2)? })
    })?;
    for a in rows { println!("{:?}", a?); }
    Ok(())
}
```

- **Always use parameters** (`?1`, `$1`), never `format!` user input into SQL: that's how SQL injection happens.
- Nullable columns map to `Option<T>`.
- **Transactions** group writes so they all succeed or none do: `let tx = conn.transaction()?; ...; tx.commit()?;` (dropping without commit rolls back: RAII again).
- Batch inserts inside one transaction are hundreds of times faster than separate autocommitted inserts.

## sqlx with Postgres

```rust,norun
let pool = sqlx::PgPool::connect(&std::env::var("DATABASE_URL")?).await?;
sqlx::migrate!("./migrations").run(&pool).await?;

let open: Vec<Ticket> = sqlx::query_as!(
    Ticket,
    "SELECT id, title, done FROM tickets WHERE done = $1 ORDER BY id",
    false
).fetch_all(&pool).await?;
```

`query_as!` connects to the database **at compile time** (or uses an offline cache with `cargo sqlx prepare`) and refuses to compile if a column name is wrong or a type doesn't match the struct. A **pool** reuses connections; put it in your Axum state.

## The repository pattern

Don't scatter SQL through your business logic. Define a **trait** for what the app needs, implement it for the real database, and implement it again **in memory** for tests:

```rust,norun
trait TicketRepo {
    fn add(&mut self, title: &str) -> Result<Ticket, RepoError>;
    fn get(&self, id: u32) -> Result<Option<Ticket>, RepoError>;
    fn close(&mut self, id: u32) -> Result<bool, RepoError>;
}

struct SqliteRepo { conn: rusqlite::Connection }   // used in production
struct MemoryRepo { tickets: Vec<Ticket> }          // used in unit tests
```

Business rules (services) take `&mut impl TicketRepo` (or `&mut dyn TicketRepo`) and never know which one they got. Tests run in microseconds without a database, and swapping SQLite for Postgres later touches only one file. The first exercise builds this.

## Migrations

Schemas change over time. **Migrations** are numbered SQL scripts applied in order, with a table recording which have run:

```text
migrations/
  0001_create_tickets.sql
  0002_add_priority.sql
  0003_create_comments.sql
```

Rules: never edit a migration that has already run in production; add a new one. Make each migration run in a transaction. Keep data migrations (moving values) separate from schema changes when they're big. `sqlx migrate`, `diesel migration` and `refinery` all implement this; the second exercise implements the core logic.

## Performance and correctness tips

- Add **indexes** for columns you filter or join on; check plans with `EXPLAIN QUERY PLAN` (SQLite) or `EXPLAIN ANALYZE` (Postgres).
- Avoid the **N+1 query** problem: one query with a `JOIN` or `WHERE id IN (...)` instead of a query per row.
- SQLite in apps: enable `PRAGMA journal_mode = WAL` for concurrent readers, and `busy_timeout`.
- Store money as integer cents, timestamps as ISO 8601 text or Unix seconds (the `time`/`chrono` crates integrate with the drivers).
- Use constraints (`NOT NULL`, `UNIQUE`, `CHECK`, foreign keys): the database is the last line of defence for data integrity.

@@quiz
Why must SQL use parameters (`?1`, `$1`) instead of `format!`?
* To prevent SQL injection: parameters are sent as data, never interpreted as SQL
- Parameters are required by Rust
- format! is slow
- To support Unicode
? A user typing `'; DROP TABLE users; --` is harmless with parameters.

@@quiz
What does sqlx's `query_as!` macro check?
- Nothing, it's just a shortcut
* At compile time, that the SQL is valid for the real schema and that columns match the Rust struct's fields and types
- That the database is running in production
- The query's speed
? It uses the database (or an offline cache) during compilation.

@@quiz
What is the main benefit of a repository trait?
- Faster queries
* Business logic doesn't depend on a specific database, so it can be tested with an in-memory implementation and the storage can change
- It writes SQL for you
- It replaces migrations
? Dependency inversion keeps the core of the app clean.

@@quiz
A migration already ran in production and has a typo in a column default. What do you do?
- Edit the old migration file
* Add a new migration that fixes it
- Delete the database
- Ignore it
? Old migrations must stay as they ran, or environments drift apart.

@@exercise A repository and a service
Implement an in-memory repository and a service with business rules on top of it.

- `MemoryRepo` implements `TicketRepo`: `insert` assigns ids starting at 1; `get` returns a clone; `update` replaces the ticket with the same id and returns `false` if it doesn't exist; `all` returns clones in id order.
- `TicketService<R: TicketRepo>` has:
  - `open(&mut self, title: &str) -> Result<u32, String>`: titles must not be blank (trimmed) and at most 60 characters; returns the new id.
  - `close(&mut self, id: u32) -> Result<(), String>`: error if the ticket doesn't exist or is already closed.
  - `open_titles(&self) -> Vec<String>`: titles of tickets not yet closed.
@@starter
#[derive(Debug, Clone, PartialEq)]
struct Ticket {
    id: u32,
    title: String,
    closed: bool,
}

trait TicketRepo {
    fn insert(&mut self, title: &str) -> u32;
    fn get(&self, id: u32) -> Option<Ticket>;
    fn update(&mut self, ticket: Ticket) -> bool;
    fn all(&self) -> Vec<Ticket>;
}

struct MemoryRepo {
    tickets: Vec<Ticket>,
}

// impl TicketRepo for MemoryRepo { ... }

struct TicketService<R: TicketRepo> {
    repo: R,
}

impl<R: TicketRepo> TicketService<R> {
    fn open(&mut self, title: &str) -> Result<u32, String> {
        todo!()
    }

    fn close(&mut self, id: u32) -> Result<(), String> {
        todo!()
    }

    fn open_titles(&self) -> Vec<String> {
        todo!()
    }
}

fn main() {
    let mut svc = TicketService { repo: MemoryRepo { tickets: vec![] } };
    let a = svc.open("Printer jam").unwrap();
    svc.open("VPN down").unwrap();
    println!("{:?}", svc.open("   "));
    println!("{:?}", svc.close(a));
    println!("{:?}", svc.close(a));
    println!("{:?}", svc.close(99));
    println!("{:?}", svc.open_titles());
}
@@expected
Err("title is required")
Ok(())
Err("ticket 1 is already closed")
Err("ticket 99 not found")
["VPN down"]
@@tests
#[test]
fn checks() {
    let mut repo = MemoryRepo { tickets: vec![] };
    assert_eq!(repo.insert("a"), 1);
    assert_eq!(repo.insert("b"), 2);
    assert!(!repo.update(Ticket { id: 5, title: "x".into(), closed: false }));
    assert_eq!(repo.get(2).unwrap().title, "b");
    let mut svc = TicketService { repo };
    assert!(svc.open(&"x".repeat(61)).is_err());
    assert_eq!(svc.open(&"x".repeat(60)), Ok(3));
    assert_eq!(svc.open_titles().len(), 3);
}
@@requires
impl TicketRepo for MemoryRepo
@@hint
`insert`: `let id = self.tickets.len() as u32 + 1;` then push. `update`: `match self.tickets.iter_mut().find(|t| t.id == ticket.id) { Some(t) => { *t = ticket; true } None => false }`. In `close`, `let mut t = self.repo.get(id).ok_or(format!("ticket {id} not found"))?;` then check `t.closed`.
@@solution
#[derive(Debug, Clone, PartialEq)]
struct Ticket {
    id: u32,
    title: String,
    closed: bool,
}

trait TicketRepo {
    fn insert(&mut self, title: &str) -> u32;
    fn get(&self, id: u32) -> Option<Ticket>;
    fn update(&mut self, ticket: Ticket) -> bool;
    fn all(&self) -> Vec<Ticket>;
}

struct MemoryRepo {
    tickets: Vec<Ticket>,
}

impl TicketRepo for MemoryRepo {
    fn insert(&mut self, title: &str) -> u32 {
        let id = self.tickets.len() as u32 + 1;
        self.tickets.push(Ticket { id, title: title.to_string(), closed: false });
        id
    }

    fn get(&self, id: u32) -> Option<Ticket> {
        self.tickets.iter().find(|t| t.id == id).cloned()
    }

    fn update(&mut self, ticket: Ticket) -> bool {
        match self.tickets.iter_mut().find(|t| t.id == ticket.id) {
            Some(t) => {
                *t = ticket;
                true
            }
            None => false,
        }
    }

    fn all(&self) -> Vec<Ticket> {
        self.tickets.clone()
    }
}

struct TicketService<R: TicketRepo> {
    repo: R,
}

impl<R: TicketRepo> TicketService<R> {
    fn open(&mut self, title: &str) -> Result<u32, String> {
        let title = title.trim();
        if title.is_empty() {
            return Err("title is required".into());
        }
        if title.chars().count() > 60 {
            return Err("title is too long (max 60)".into());
        }
        Ok(self.repo.insert(title))
    }

    fn close(&mut self, id: u32) -> Result<(), String> {
        let mut t = self.repo.get(id).ok_or(format!("ticket {id} not found"))?;
        if t.closed {
            return Err(format!("ticket {id} is already closed"));
        }
        t.closed = true;
        self.repo.update(t);
        Ok(())
    }

    fn open_titles(&self) -> Vec<String> {
        self.repo.all().into_iter().filter(|t| !t.closed).map(|t| t.title).collect()
    }
}

fn main() {
    let mut svc = TicketService { repo: MemoryRepo { tickets: vec![] } };
    let a = svc.open("Printer jam").unwrap();
    svc.open("VPN down").unwrap();
    println!("{:?}", svc.open("   "));
    println!("{:?}", svc.close(a));
    println!("{:?}", svc.close(a));
    println!("{:?}", svc.close(99));
    println!("{:?}", svc.open_titles());
}

@@exercise A migration runner
Write the core of a migration tool. Migration files are named like `0002_add_priority.sql`.

- `parse_name(file: &str) -> Option<(u32, String)>`: the number and the name (`(2, "add_priority")`); `None` if it doesn't match `NNNN_name.sql` (digits, underscore, a non-empty name, `.sql`).
- `pending(files: &[&str], applied: &[u32]) -> Result<Vec<String>, String>`: valid migration files not yet applied, sorted by number, returned as file names. Errors: two files with the same number (`duplicate migration 2`), or an applied number that has no file anymore (`missing migration 3`). Invalid file names are ignored (README.md, notes.txt).
@@starter
fn parse_name(file: &str) -> Option<(u32, String)> {
    todo!()
}

fn pending(files: &[&str], applied: &[u32]) -> Result<Vec<String>, String> {
    todo!()
}

fn main() {
    let files = ["0003_create_comments.sql", "README.md", "0001_create_tickets.sql", "0002_add_priority.sql"];
    println!("{:?}", pending(&files, &[1]));
    println!("{:?}", pending(&files, &[1, 2, 3]));
    println!("{:?}", pending(&files, &[1, 4]));
    println!("{:?}", pending(&["0001_a.sql", "0001_b.sql"], &[]));
}
@@expected
Ok(["0002_add_priority.sql", "0003_create_comments.sql"])
Ok([])
Err("missing migration 4")
Err("duplicate migration 1")
@@tests
#[test]
fn checks() {
    assert_eq!(parse_name("0010_add_index.sql"), Some((10, "add_index".to_string())));
    assert_eq!(parse_name("0010_.sql"), None);
    assert_eq!(parse_name("abcd_x.sql"), None);
    assert_eq!(parse_name("0001_x.txt"), None);
    assert_eq!(parse_name("notes.sql"), None);
    assert_eq!(pending(&[], &[]), Ok(vec![]));
}
@@requires
strip_suffix(".sql")
@@hint
`let stem = file.strip_suffix(".sql")?; let (num, name) = stem.split_once('_')?;` then check `!num.is_empty() && num.chars().all(|c| c.is_ascii_digit()) && !name.is_empty()`. In `pending`, collect `(number, file)` pairs, sort them, check neighbours for duplicates with `windows(2)`, then check every applied number exists.
@@solution
fn parse_name(file: &str) -> Option<(u32, String)> {
    let stem = file.strip_suffix(".sql")?;
    let (num, name) = stem.split_once('_')?;
    if num.is_empty() || !num.chars().all(|c| c.is_ascii_digit()) || name.is_empty() {
        return None;
    }
    Some((num.parse().ok()?, name.to_string()))
}

fn pending(files: &[&str], applied: &[u32]) -> Result<Vec<String>, String> {
    let mut found: Vec<(u32, &str)> = files
        .iter()
        .filter_map(|f| parse_name(f).map(|(n, _)| (n, *f)))
        .collect();
    found.sort();
    for w in found.windows(2) {
        if w[0].0 == w[1].0 {
            return Err(format!("duplicate migration {}", w[0].0));
        }
    }
    for a in applied {
        if !found.iter().any(|(n, _)| n == a) {
            return Err(format!("missing migration {a}"));
        }
    }
    Ok(found.into_iter().filter(|(n, _)| !applied.contains(n)).map(|(_, f)| f.to_string()).collect())
}

fn main() {
    let files = ["0003_create_comments.sql", "README.md", "0001_create_tickets.sql", "0002_add_priority.sql"];
    println!("{:?}", pending(&files, &[1]));
    println!("{:?}", pending(&files, &[1, 2, 3]));
    println!("{:?}", pending(&files, &[1, 4]));
    println!("{:?}", pending(&["0001_a.sql", "0001_b.sql"], &[]));
}

@@flashcards
sqlx :: async SQL with compile-time checked queries
rusqlite :: SQLite bindings
parameterised queries :: prevent SQL injection
migrations :: versioned schema changes
connection pool :: reuse database connections
