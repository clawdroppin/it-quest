@@lesson m11-kvstore Project: A Key-Value Store with Expiry and Persistence
@@text
This project ties Module 11 together: a small **key-value store** like a mini Redis, the kind of component behind caches, session stores and feature-flag services. You build its two hardest parts as std-only code; the lesson shows how the networking, persistence and testing lessons wrap around it.

## The design

```text
          TCP (lesson 2)                 snapshot file (this lesson)
client ──"SET k v EX 60"──▶ protocol ──▶ Store ──────────────▶ data.kv
       ◀──────"OK"────────            (HashMap + expiry)   ◀── on start-up
```

- **Store**: a `HashMap<String, Entry>` where `Entry { value, expires_at: Option<u64> }`.
- **Time is a parameter** (`now: u64`) instead of calling the system clock inside. That makes expiry testable in microseconds: a test can jump an hour ahead by passing a bigger number (the test-doubles idea from lesson 6).
- **Expiry** works two ways, like Redis: **lazily** (a `get` of an expired key treats it as missing and deletes it) and **actively** (`purge` sweeps everything expired, called periodically).
- **Persistence**: a **snapshot** writes every live entry to a text file; start-up restores it. Writing goes to `data.kv.tmp` first, then `rename` over the old file, so a crash mid-write never leaves a half-written snapshot (rename is atomic on the same file system).
- **Concurrency** (lesson 10.4): the server wraps the store in `Arc<Mutex<Store>>`, or shards it into several mutexes by key hash to reduce contention.

## Escaping: why the snapshot needs it

The snapshot format is one entry per line with tab-separated fields:

```text
session:42\tmaya\t1700000060
greeting\thello world\t-
```

Values can contain tabs, newlines and backslashes, which would break the format. **Escaping** fixes that: write `\` as `\\`, tab as `\t`, newline as `\n`; when reading, reverse it. This is the same idea as JSON strings, CSV quoting and SQL parameters: whenever data travels inside a format with special characters, encode it.

## Production touches (beyond the exercises)

- An **append-only log** of every write, replayed on start-up and compacted into a new snapshot when it grows: Redis AOF and database write-ahead logs work this way.
- `fsync` (`File::sync_all`) before rename when durability matters.
- Limits: maximum key/value size and total memory, with an eviction policy (LRU) when full.
- Metrics: hit rate, number of keys, expired keys per sweep (lesson 7).
- Property test: after any random sequence of SET/DEL, `restore(snapshot(store))` equals the store (lesson 6).

## Extend it yourself

After the exercises, combine the pieces in a Cargo project: the protocol from lesson 2 (add `SET key value EX seconds`), the store and snapshot from here, a background thread calling `purge` every second, and a snapshot on shutdown.

@@quiz
Why does the store take `now` as a parameter instead of reading the clock?
- It's faster
* Tests can control time precisely and check expiry without waiting
- Rust has no clock
- To save memory
? Injecting time is the same idea as a stub clock.

@@quiz
Why write a snapshot to a temporary file and then rename it?
- Renaming compresses the file
* Rename replaces the old file atomically, so a crash never leaves a half-written snapshot
- Temporary files are faster to write
- It's required by Windows
? Readers see either the old or the new snapshot, never a mix.

@@quiz
What is lazy expiry?
* Treating an expired key as missing (and deleting it) when it's accessed
- Never expiring keys
- Expiring keys at random
- Sweeping all keys every millisecond
? Active sweeps clean up keys nobody reads.

@@drill
After writing the snapshot to a temporary file, which `std::fs` function replaces the old file in one step?
= rename
= fs::rename
= std::fs::rename
? A crash leaves either the old file or the new one, never half of each.

@@flashcards
TTL :: time to live: how long a key stays valid
lazy expiry :: expired keys are removed when they're next read
injected clock :: pass now in so tests control time
snapshot :: the whole store written to disk
write then rename :: makes saving atomic: never a half-written file
escaping :: protect separator characters inside stored values

@@exercise The store with expiry
Implement the in-memory store.

- `set(key, value, ttl: Option<u64>, now)`: stores the value; with a TTL it expires at `now + ttl`. Setting a key again replaces its value **and** its expiry.
- `get(&mut self, key, now) -> Option<String>`: a key is expired when `now >= expires_at`; an expired key is removed and `None` returned.
- `ttl(&self, key, now) -> Option<i64>`: seconds left; `-1` for a key without expiry; `None` if missing or expired.
- `del(&mut self, key) -> bool`.
- `purge(&mut self, now) -> usize`: removes every expired entry and returns how many.
@@starter
use std::collections::HashMap;

struct Entry {
    value: String,
    expires_at: Option<u64>,
}

#[derive(Default)]
struct Store {
    data: HashMap<String, Entry>,
}

impl Store {
    fn set(&mut self, key: &str, value: &str, ttl: Option<u64>, now: u64) {
        todo!()
    }

    fn get(&mut self, key: &str, now: u64) -> Option<String> {
        todo!()
    }

    fn ttl(&self, key: &str, now: u64) -> Option<i64> {
        todo!()
    }

    fn del(&mut self, key: &str) -> bool {
        todo!()
    }

    fn purge(&mut self, now: u64) -> usize {
        todo!()
    }
}

fn main() {
    let mut s = Store::default();
    s.set("session:42", "maya", Some(60), 1000);
    s.set("greeting", "hello", None, 1000);
    s.set("otp", "123456", Some(30), 1000);
    println!("{:?} {:?}", s.get("session:42", 1059), s.ttl("session:42", 1059));
    println!("{:?} {:?}", s.get("session:42", 1060), s.ttl("greeting", 1060));
    println!("purged {}", s.purge(1100));
    println!("keys left: {}", s.data.len());
}
@@expected
Some("maya") Some(1)
None Some(-1)
purged 1
keys left: 1
@@tests
#[test]
fn checks() {
    let mut s = Store::default();
    s.set("k", "a", Some(10), 0);
    s.set("k", "b", None, 5);
    assert_eq!(s.get("k", 100), Some("b".into()), "re-set clears the expiry");
    assert!(s.del("k"));
    assert!(!s.del("k"));
    assert_eq!(s.ttl("k", 0), None);
    s.set("x", "1", Some(0), 7);
    assert_eq!(s.get("x", 7), None, "ttl 0 expires immediately");
}
@@requires
retain
@@hint
A helper `fn expired(e: &Entry, now: u64) -> bool { e.expires_at.is_some_and(|t| now >= t) }`. In `get`, check first and `self.data.remove(key)` if expired. `purge`: remember `len` before, `self.data.retain(|_, e| !expired(e, now))`, return the difference.
@@solution
use std::collections::HashMap;

struct Entry {
    value: String,
    expires_at: Option<u64>,
}

#[derive(Default)]
struct Store {
    data: HashMap<String, Entry>,
}

fn expired(e: &Entry, now: u64) -> bool {
    e.expires_at.is_some_and(|t| now >= t)
}

impl Store {
    fn set(&mut self, key: &str, value: &str, ttl: Option<u64>, now: u64) {
        let entry = Entry { value: value.to_string(), expires_at: ttl.map(|t| now + t) };
        self.data.insert(key.to_string(), entry);
    }

    fn get(&mut self, key: &str, now: u64) -> Option<String> {
        let e = self.data.get(key)?;
        if expired(e, now) {
            self.data.remove(key);
            return None;
        }
        Some(e.value.clone())
    }

    fn ttl(&self, key: &str, now: u64) -> Option<i64> {
        let e = self.data.get(key)?;
        if expired(e, now) {
            return None;
        }
        Some(e.expires_at.map_or(-1, |t| (t - now) as i64))
    }

    fn del(&mut self, key: &str) -> bool {
        self.data.remove(key).is_some()
    }

    fn purge(&mut self, now: u64) -> usize {
        let before = self.data.len();
        self.data.retain(|_, e| !expired(e, now));
        before - self.data.len()
    }
}

fn main() {
    let mut s = Store::default();
    s.set("session:42", "maya", Some(60), 1000);
    s.set("greeting", "hello", None, 1000);
    s.set("otp", "123456", Some(30), 1000);
    println!("{:?} {:?}", s.get("session:42", 1059), s.ttl("session:42", 1059));
    println!("{:?} {:?}", s.get("session:42", 1060), s.ttl("greeting", 1060));
    println!("purged {}", s.purge(1100));
    println!("keys left: {}", s.data.len());
}

@@exercise Snapshots with escaping
Write the snapshot format.

- `escape(s)`: `\` → `\\`, tab → `\t`, newline → `\n`. `unescape(s) -> Result<String, String>`: the reverse; a backslash followed by anything else, or at the very end, is `Err("bad escape")`.
- `snapshot(entries: &[(String, String, Option<u64>)]) -> String`: one line per entry, `key\tvalue\texpiry` with escaped key and value and the expiry as a number or `-`. Lines are sorted by key so snapshots are reproducible; every line ends with `\n`.
- `restore(text, now) -> Result<Vec<(String, String, Option<u64>)>, String>`: parse it back (in file order), **skipping** entries already expired at `now`. A line without exactly 3 tab-separated fields, or a bad expiry, is `Err("line N: ...")` with the line number from 1 and any message.
@@starter
fn escape(s: &str) -> String {
    todo!()
}

fn unescape(s: &str) -> Result<String, String> {
    todo!()
}

fn snapshot(entries: &[(String, String, Option<u64>)]) -> String {
    todo!()
}

fn restore(text: &str, now: u64) -> Result<Vec<(String, String, Option<u64>)>, String> {
    todo!()
}

fn main() {
    let entries = vec![
        ("note".to_string(), "line1\nline2\tend".to_string(), None),
        ("b\\path".to_string(), "C:\\temp".to_string(), Some(2000)),
        ("old".to_string(), "x".to_string(), Some(500)),
    ];
    let snap = snapshot(&entries);
    print!("{snap}");
    let back = restore(&snap, 1000).unwrap();
    println!("{} restored, note value {:?}", back.len(), back[1].1);
    println!("{:?}", restore("a\tb\n", 0));
    println!("{:?}", unescape("bad\\q"));
}
@@expected
b\\path	C:\\temp	2000
note	line1\nline2\tend	-
old	x	500
2 restored, note value "line1\nline2\tend"
Err("line 1: expected 3 fields")
Err("bad escape")
@@tests
#[test]
fn checks() {
    let tricky = "a\\b\tc\nd\\\\";
    assert_eq!(unescape(&escape(tricky)), Ok(tricky.to_string()));
    assert!(unescape("end\\").is_err());
    assert!(restore("k\tv\tsoon\n", 0).is_err());
    assert_eq!(restore("", 0), Ok(vec![]));
    let e = vec![("k".to_string(), "v".to_string(), Some(10))];
    assert_eq!(restore(&snapshot(&e), 10), Ok(vec![]), "expired at exactly now");
}
@@requires
fn unescape
.chars()
@@hint
`unescape`: loop with `let mut chars = s.chars();` and on `\\` match `chars.next()`: `Some('\\')`, `Some('t')`, `Some('n')`, otherwise error. Sort with `let mut sorted = entries.to_vec(); sorted.sort_by(|a, b| a.0.cmp(&b.0));`. In `restore`, split each line with `line.split('\t')` (escaped values contain no raw tabs), and parse `-` as `None`.
@@solution
fn escape(s: &str) -> String {
    let mut out = String::new();
    for c in s.chars() {
        match c {
            '\\' => out.push_str("\\\\"),
            '\t' => out.push_str("\\t"),
            '\n' => out.push_str("\\n"),
            c => out.push(c),
        }
    }
    out
}

fn unescape(s: &str) -> Result<String, String> {
    let mut out = String::new();
    let mut chars = s.chars();
    while let Some(c) = chars.next() {
        if c != '\\' {
            out.push(c);
            continue;
        }
        match chars.next() {
            Some('\\') => out.push('\\'),
            Some('t') => out.push('\t'),
            Some('n') => out.push('\n'),
            _ => return Err("bad escape".into()),
        }
    }
    Ok(out)
}

fn snapshot(entries: &[(String, String, Option<u64>)]) -> String {
    let mut sorted = entries.to_vec();
    sorted.sort_by(|a, b| a.0.cmp(&b.0));
    let mut out = String::new();
    for (k, v, exp) in sorted {
        let exp = exp.map_or("-".to_string(), |t| t.to_string());
        out += &format!("{}\t{}\t{}\n", escape(&k), escape(&v), exp);
    }
    out
}

fn restore(text: &str, now: u64) -> Result<Vec<(String, String, Option<u64>)>, String> {
    let mut out = Vec::new();
    for (i, line) in text.lines().enumerate() {
        let n = i + 1;
        let fields: Vec<&str> = line.split('\t').collect();
        if fields.len() != 3 {
            return Err(format!("line {n}: expected 3 fields"));
        }
        let key = unescape(fields[0]).map_err(|e| format!("line {n}: {e}"))?;
        let value = unescape(fields[1]).map_err(|e| format!("line {n}: {e}"))?;
        let exp = match fields[2] {
            "-" => None,
            t => Some(t.parse::<u64>().map_err(|_| format!("line {n}: bad expiry {t:?}"))?),
        };
        if exp.is_some_and(|t| now >= t) {
            continue;
        }
        out.push((key, value, exp));
    }
    Ok(out)
}

fn main() {
    let entries = vec![
        ("note".to_string(), "line1\nline2\tend".to_string(), None),
        ("b\\path".to_string(), "C:\\temp".to_string(), Some(2000)),
        ("old".to_string(), "x".to_string(), Some(500)),
    ];
    let snap = snapshot(&entries);
    print!("{snap}");
    let back = restore(&snap, 1000).unwrap();
    println!("{} restored, note value {:?}", back.len(), back[1].1);
    println!("{:?}", restore("a\tb\n", 0));
    println!("{:?}", unescape("bad\\q"));
}
