OUT = "content/cpp/y2_01_kvstore.md"
BASE = "tools/appdev/cpp_kvstore"
FILES = ["kv.hpp", "tests.cpp", "main.cpp"]

PARTS = [
    ("text", r'''
@@lesson x9-kvstore Mini Key-Value Database
@@text
Redis, LevelDB, RocksDB and the storage engines inside PostgreSQL and SQLite are written in C and C++, and they all share one idea you'll build here: keep the data in memory for speed, and make every change **durable** by appending it to a **log** on disk *before* saying "OK". After a crash or a restart, replaying the log rebuilds the data.

```text
$ ./kv data.log
loaded 0 key(s) from 0 record(s) in data.log
kv> SET motd Welcome to the Tokyo office
OK
kv> SET user:1 yuki
OK
kv> KEYS user:
1) user:1
kv> GET motd
"Welcome to the Tokyo office"
kv> QUIT
$ ./kv data.log
loaded 2 key(s) from 2 record(s) in data.log
```

## The log format

One record per line, and keys and values are **escaped** so they can't break the line format:

```text
SET motd Welcome\sto\sthe\sTokyo\soffice
SET user:1 yuki
DEL user:1
```

| In the text | In the log |
|---|---|
| backslash `\` | `\\` |
| newline | `\n` |
| space | `\s` |
| the empty string | `\0` |

## Crash safety

Two real-world problems the design handles:

- **Torn writes**: the power fails in the middle of writing a line. The last line has no `\n`. It was never acknowledged to anyone, so replay **ignores** it.
- **The log grows forever**: setting one key a million times writes a million lines. **Compaction** writes a fresh log with one `SET` per key, to a temporary file, then **renames** it over the old one. A rename is atomic: after a crash there's either the old log or the new one, never half of each.

Any other damage (an unknown command, a bad escape) means the file is corrupt, and the program must stop with the line number rather than silently load wrong data.

## Your tasks

In `kv.hpp`, four functions throw `TODO`:

1. `escape` and `unescape`: the table above. `unescape` throws `std::invalid_argument("bad escape")` for anything else after a backslash.
2. `Store::replay`: read lines with `std::getline`; detect the torn last line with `in.eof()`; split with `std::istringstream`; report corruption with the line number.
3. `parseCommand`: case-insensitive command names; for `SET`, the value is the **rest of the line**; exact error messages are in the comment.

Build and run the tests:

```text
g++ -std=c++20 -Wall -o tests tests.cpp && ./tests
cl /EHsc /std:c++20 /utf-8 tests.cpp && tests.exe          (Visual Studio Developer Prompt)
```

Then build `main.cpp` the same way and try the shell.
'''),
    ("project", "kvstore", "Mini Key-Value Database",
     "Write the four functions that throw `TODO` in `kv.hpp` until `tests.cpp` passes, then build `main.cpp` and use your database.",
     BASE + "/starter", FILES),
    ("text", r'''
@@text
## Stretch goals

- **Expiry**: `SET session:abc xyz EX 60` and `TTL key`. Store the expiry time in the log (absolute time, not "60 seconds", or replay will extend it).
- **A network server**: accept connections with Asio (or plain sockets) and speak a line protocol, so several clients share one store. Then add a `std::shared_mutex`: many readers, one writer.
- **Benchmark it**: time a million `SET`s with `std::chrono`. Then try `flush()` only every 100 writes (faster, but what can you lose in a crash?). This trade-off is Redis's `appendfsync always / everysec / no` setting.
- **Checksums**: add a CRC32 to every line, so corruption in the middle of the file is detected even when the line still parses.
- **Binary format**: store `[length][bytes]` instead of escaped text. Compare file size and speed.
- **CMake**: a `CMakeLists.txt` with two targets (`kv` and `tests`) and `enable_testing()` + `add_test`, so `ctest` runs the tests.
'''),
    ("project", "kvstore_finished", "Mini Key-Value Database: the finished version",
     "A complete solution to compare with yours.",
     BASE + "/finished", FILES),
    ("text", r'''
@@text
🎉 Write-ahead logs, atomic renames, compaction and torn-write handling are the foundations of every database, file system and message queue. You now understand what "durable" means and what it costs.
'''),
]
