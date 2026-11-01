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
@@project kvstore Mini Key-Value Database
Write the four functions that throw `TODO` in `kv.hpp` until `tests.cpp` passes, then build `main.cpp` and use your database.
@@file kv.hpp
// kv.hpp: a small persistent key-value database, in the style of Redis and the storage engines
// inside real databases: an in-memory map, plus an append-only log on disk that is replayed at
// start-up. Header-only, standard library only.
#pragma once

#include <cctype>
#include <cstdio>
#include <filesystem>
#include <fstream>
#include <istream>
#include <map>
#include <optional>
#include <ostream>
#include <sstream>
#include <stdexcept>
#include <string>
#include <vector>

namespace kv {

// ---------------------------------------------------------------- the log format
//
// One record per line:   SET <key> <value>   or   DEL <key>
// Keys and values may contain spaces, newlines and backslashes, so they're escaped:
//   backslash -> \\     newline -> \n     space -> \s
// After escaping, a key or value never contains a space or newline, so a line splits cleanly.

/// Escape text for the log: "a b\c" becomes "a\sb\\c". The empty string becomes "\0" (so it
/// still takes up a field in the line).
inline std::string escape(const std::string& s) {
    throw std::logic_error("TODO: not done yet");
}

/// Reverse escape(). Throws std::invalid_argument("bad escape") for a backslash followed by
/// anything other than \ n s 0, or a backslash at the very end.
inline std::string unescape(const std::string& s) {
    throw std::logic_error("TODO: not done yet");
}

// ---------------------------------------------------------------- the store

class Store {
public:
    /// An in-memory store. Give it a log stream to make every change durable.
    explicit Store(std::ostream* log = nullptr) : log_(log) {}

    /// Start (or stop, with nullptr) logging changes, e.g. after replaying the old log.
    void setLog(std::ostream* log) { log_ = log; }

    void set(const std::string& key, const std::string& value) {
        if (key.empty()) throw std::invalid_argument("key must not be empty");
        write("SET " + escape(key) + " " + escape(value));
        data_[key] = value;
    }

    std::optional<std::string> get(const std::string& key) const {
        auto it = data_.find(key);
        if (it == data_.end()) return std::nullopt;
        return it->second;
    }

    bool del(const std::string& key) {
        if (!data_.contains(key)) return false;
        write("DEL " + escape(key));
        data_.erase(key);
        return true;
    }

    /// Keys starting with prefix, in sorted order (std::map keeps them sorted for free).
    std::vector<std::string> keys(const std::string& prefix = "") const {
        std::vector<std::string> out;
        for (auto it = data_.lower_bound(prefix); it != data_.end() && it->first.starts_with(prefix); ++it)
            out.push_back(it->first);
        return out;
    }

    std::size_t size() const { return data_.size(); }

    /**
     * Rebuild the data from a log. Each COMPLETE line (ending in '\n') is applied in order:
     * "SET key value" sets, "DEL key" deletes. Returns how many records were applied.
     *
     * A crash can leave a half-written last line with no '\n': ignore it, it was never
     * acknowledged. Any other malformed line (unknown command, wrong number of fields, bad
     * escape) throws std::runtime_error("corrupt log at line N"), counting lines from 1.
     */
    std::size_t replay(std::istream& in) {
        throw std::logic_error("TODO: not done yet");
    }

    /// The smallest log that recreates the current data: one SET per key.
    std::string snapshot() const {
        std::string out;
        for (const auto& [k, v] : data_) out += "SET " + escape(k) + " " + escape(v) + "\n";
        return out;
    }

private:
    void write(const std::string& record) {
        if (!log_) return;
        *log_ << record << '\n';
        log_->flush();                              // durable before we say "OK"
        if (!*log_) throw std::runtime_error("could not write the log");
    }

    std::map<std::string, std::string> data_;
    std::ostream* log_;
};

// ---------------------------------------------------------------- the command language

struct Command {
    std::string op;                 // SET GET DEL KEYS COUNT
    std::vector<std::string> args;
};

/**
 * Parse one line typed by a user. The command name is case-insensitive and returned in upper
 * case. SET takes a key and a value, where the value is the REST of the line (it may contain
 * spaces): "SET motd Welcome to Tokyo" has the value "Welcome to Tokyo". GET and DEL take one
 * key; KEYS takes an optional prefix; COUNT takes nothing. Errors throw std::invalid_argument:
 * "empty command", "unknown command: FOO", "wrong number of arguments for SET".
 */
inline Command parseCommand(const std::string& line) {
    throw std::logic_error("TODO: not done yet");
}

/// Run a command and return the reply, in the style of redis-cli.
inline std::string execute(Store& store, const Command& cmd) {
    if (cmd.op == "SET") {
        store.set(cmd.args[0], cmd.args[1]);
        return "OK";
    }
    if (cmd.op == "GET") {
        auto v = store.get(cmd.args[0]);
        return v ? "\"" + *v + "\"" : "(nil)";
    }
    if (cmd.op == "DEL") return store.del(cmd.args[0]) ? "(integer) 1" : "(integer) 0";
    if (cmd.op == "COUNT") return "(integer) " + std::to_string(store.size());
    auto keys = store.keys(cmd.args.empty() ? "" : cmd.args[0]);
    if (keys.empty()) return "(empty)";
    std::string out;
    for (std::size_t i = 0; i < keys.size(); ++i) out += (i ? "\n" : "") + std::to_string(i + 1) + ") " + keys[i];
    return out;
}

/// Replace the log file with a snapshot, safely: write a new file, then rename it over the old one.
inline void compactFile(const Store& store, const std::filesystem::path& path) {
    auto tmp = path;
    tmp += ".tmp";
    {
        std::ofstream out(tmp, std::ios::binary | std::ios::trunc);
        out << store.snapshot();
        out.flush();
        if (!out) throw std::runtime_error("could not write " + tmp.string());
    }
    std::filesystem::rename(tmp, path);
}

}  // namespace kv
@@file tests.cpp
// tests.cpp: a tiny test runner, so the project needs nothing installed.
//   g++ -std=c++20 -o tests tests.cpp && ./tests       (or: cl /EHsc /std:c++20 tests.cpp && tests.exe)
#include <filesystem>
#include <fstream>
#include <functional>
#include <iostream>
#include <sstream>
#include <string>

#include "kv.hpp"

using namespace kv;

static int passed = 0, failed = 0;

static void check(const std::string& name, const std::function<bool()>& test) {
    try {
        if (test()) {
            ++passed;
            std::cout << "  ok    " << name << '\n';
            return;
        }
        std::cout << "  FAIL  " << name << '\n';
    } catch (const std::exception& e) {
        if (std::string(e.what()).starts_with("TODO")) {
            std::cout << "  TODO  " << name << '\n';
            ++failed;
            return;
        }
        std::cout << "  FAIL  " << name << ": exception: " << e.what() << '\n';
    }
    ++failed;
}

template <typename E, typename F>
static std::string errorOf(F f) {
    try {
        f();
    } catch (const E& e) {
        return e.what();
    }
    return "no error";
}

int main() {
    std::cout << "escape / unescape\n";
    check("plain text is unchanged", [] { return escape("hello") == "hello" && unescape("hello") == "hello"; });
    check("spaces, newlines and backslashes", [] { return escape("a b\nc\\d") == "a\\sb\\nc\\\\d"; });
    check("empty string", [] { return escape("") == "\\0" && unescape("\\0").empty(); });
    check("round trip", [] {
        for (std::string s : {"", " ", "\\", "\\s", "two  spaces", "line1\nline2", "東京 office", "\\0"})
            if (unescape(escape(s)) != s) return false;
        return true;
    });
    check("escaped text has no spaces or newlines", [] {
        auto e = escape("a b\nc");
        return e.find(' ') == std::string::npos && e.find('\n') == std::string::npos;
    });
    check("bad escapes throw", [] {
        return errorOf<std::invalid_argument>([] { unescape("a\\x"); }) == "bad escape"
            && errorOf<std::invalid_argument>([] { unescape("trailing\\"); }) == "bad escape";
    });

    std::cout << "Store\n";
    check("set, get, del, size", [] {
        Store s;
        s.set("user:1", "yuki");
        s.set("user:1", "ken");
        bool ok = s.get("user:1") == "ken" && !s.get("user:2") && s.size() == 1;
        return ok && s.del("user:1") && !s.del("user:1") && s.size() == 0;
    });
    check("keys by prefix, sorted", [] {
        Store s;
        for (std::string k : {"user:2", "host:web01", "user:10", "user:1", "users"}) s.set(k, "x");
        return s.keys("user:") == std::vector<std::string>{"user:1", "user:10", "user:2"} && s.keys().size() == 5
            && s.keys("zzz").empty();
    });
    check("every change is logged", [] {
        std::ostringstream log;
        Store s(&log);
        s.set("motd", "Welcome to Tokyo");
        s.del("motd");
        s.del("missing");                       // nothing deleted: nothing logged
        return log.str() == "SET motd Welcome\\sto\\sTokyo\nDEL motd\n";
    });
    check("empty keys are refused", [] { Store s; return errorOf<std::invalid_argument>([&] { s.set("", "x"); }) == "key must not be empty"; });

    std::cout << "replay\n";
    check("replays sets and deletes in order", [] {
        std::istringstream in("SET a 1\nSET b 2\nSET a 3\nDEL b\nSET c hello\\sworld\n");
        Store s;
        auto n = s.replay(in);
        return n == 5 && s.get("a") == "3" && !s.get("b") && s.get("c") == "hello world" && s.size() == 2;
    });
    check("a log written by one store rebuilds another", [] {
        std::ostringstream log;
        Store a(&log);
        a.set("note", "line1\nline2 with  spaces");
        a.set("empty", "");
        a.set("path", "C:\\Users\\yuki");
        std::istringstream in(log.str());
        Store b;
        b.replay(in);
        return b.get("note") == "line1\nline2 with  spaces" && b.get("empty") == "" && b.get("path") == "C:\\Users\\yuki";
    });
    check("a torn last line is ignored", [] {
        std::istringstream in("SET a 1\nSET b 2\nSET c 3");      // the crash happened mid-write
        Store s;
        return s.replay(in) == 2 && !s.get("c");
    });
    check("an empty log is fine", [] { std::istringstream in(""); Store s; return s.replay(in) == 0; });
    check("corruption is reported with the line number", [] {
        auto err = [](const std::string& text) {
            return errorOf<std::runtime_error>([&] { std::istringstream in(text); Store s; s.replay(in); });
        };
        return err("SET a 1\nPUT b 2\n") == "corrupt log at line 2"
            && err("SET a\n") == "corrupt log at line 1"
            && err("SET a 1 2\n") == "corrupt log at line 1"
            && err("SET a 1\nDEL\n") == "corrupt log at line 2"
            && err("SET a b\\q\n") == "corrupt log at line 1";
    });

    std::cout << "parseCommand / execute\n";
    check("SET keeps the rest of the line", [] {
        auto c = parseCommand("set motd   Welcome to  Tokyo");
        return c.op == "SET" && c.args == std::vector<std::string>{"motd", "Welcome to  Tokyo"};
    });
    check("other commands", [] {
        return parseCommand("GET a").args.size() == 1 && parseCommand("keys").args.empty()
            && parseCommand("KEYS user:").args[0] == "user:" && parseCommand("count").op == "COUNT";
    });
    check("errors", [] {
        auto err = [](const std::string& line) { return errorOf<std::invalid_argument>([&] { parseCommand(line); }); };
        return err("") == "empty command" && err("   ") == "empty command" && err("FLY away") == "unknown command: FLY"
            && err("SET onlykey") == "wrong number of arguments for SET" && err("GET a b") == "wrong number of arguments for GET"
            && err("DEL") == "wrong number of arguments for DEL" && err("COUNT x") == "wrong number of arguments for COUNT";
    });
    check("replies look like redis-cli", [] {
        Store s;
        auto run = [&](const std::string& line) { return execute(s, parseCommand(line)); };
        return run("SET a hello there") == "OK" && run("GET a") == "\"hello there\"" && run("GET b") == "(nil)"
            && run("SET b 2") == "OK" && run("KEYS") == "1) a\n2) b" && run("KEYS z") == "(empty)"
            && run("DEL a") == "(integer) 1" && run("DEL a") == "(integer) 0" && run("COUNT") == "(integer) 1";
    });

    std::cout << "compaction\n";
    check("compactFile writes one SET per key, and it replays", [] {
        auto path = std::filesystem::temp_directory_path() / "kv_compact_test.log";
        Store s;
        s.set("b", "2");
        s.set("a", "1 1");
        compactFile(s, path);
        std::ifstream in(path, std::ios::binary);
        std::stringstream text;
        text << in.rdbuf();
        in.close();
        std::istringstream again(text.str());
        Store t;
        bool ok = text.str() == "SET a 1\\s1\nSET b 2\n" && t.replay(again) == 2 && t.get("a") == "1 1";
        std::filesystem::remove(path);
        return ok;
    });

    std::cout << "\n" << passed << " passed, " << failed << " failed\n";
    return failed == 0 ? 0 : 1;
}
@@file main.cpp
// main.cpp: an interactive shell for the store, like redis-cli.
//   build: g++ -std=c++20 -O2 -o kv main.cpp     (or: cl /EHsc /std:c++20 /O2 main.cpp)
//   run:   ./kv data.log
// Type HELP for commands. Everything you SET survives a restart, because it's in data.log.
#include <fstream>
#include <iostream>

#include "kv.hpp"

int main(int argc, char** argv) {
    const std::string path = argc > 1 ? argv[1] : "data.log";
    kv::Store store;
    try {
        std::ifstream existing(path, std::ios::binary);
        std::size_t records = existing ? store.replay(existing) : 0;
        std::cout << "loaded " << store.size() << " key(s) from " << records << " record(s) in " << path << "\n";
    } catch (const std::exception& e) {
        std::cerr << "error: " << e.what() << "\n";
        return 1;
    }

    // Start each session from a compacted log (one SET per key), then append every change to it.
    kv::compactFile(store, path);
    std::ofstream log(path, std::ios::binary | std::ios::app);
    store.setLog(&log);

    std::string line;
    std::cout << "kv> " << std::flush;
    while (std::getline(std::cin, line)) {
        if (line == "QUIT" || line == "quit" || line == "EXIT" || line == "exit") break;
        if (line == "HELP" || line == "help") {
            std::cout << "SET key value | GET key | DEL key | KEYS [prefix] | COUNT | QUIT\n";
        } else if (!line.empty()) {
            try {
                std::cout << kv::execute(store, kv::parseCommand(line)) << "\n";
            } catch (const std::exception& e) {
                std::cout << "(error) " << e.what() << "\n";
            }
        }
        std::cout << "kv> " << std::flush;
    }
    return 0;
}
@@text
## Stretch goals

- **Expiry**: `SET session:abc xyz EX 60` and `TTL key`. Store the expiry time in the log (absolute time, not "60 seconds", or replay will extend it).
- **A network server**: accept connections with Asio (or plain sockets) and speak a line protocol, so several clients share one store. Then add a `std::shared_mutex`: many readers, one writer.
- **Benchmark it**: time a million `SET`s with `std::chrono`. Then try `flush()` only every 100 writes (faster, but what can you lose in a crash?). This trade-off is Redis's `appendfsync always / everysec / no` setting.
- **Checksums**: add a CRC32 to every line, so corruption in the middle of the file is detected even when the line still parses.
- **Binary format**: store `[length][bytes]` instead of escaped text. Compare file size and speed.
- **CMake**: a `CMakeLists.txt` with two targets (`kv` and `tests`) and `enable_testing()` + `add_test`, so `ctest` runs the tests.
@@project kvstore_finished Mini Key-Value Database: the finished version
A complete solution to compare with yours.
@@file kv.hpp
// kv.hpp: a small persistent key-value database, in the style of Redis and the storage engines
// inside real databases: an in-memory map, plus an append-only log on disk that is replayed at
// start-up. Header-only, standard library only.
#pragma once

#include <cctype>
#include <cstdio>
#include <filesystem>
#include <fstream>
#include <istream>
#include <map>
#include <optional>
#include <ostream>
#include <sstream>
#include <stdexcept>
#include <string>
#include <vector>

namespace kv {

// ---------------------------------------------------------------- the log format
//
// One record per line:   SET <key> <value>   or   DEL <key>
// Keys and values may contain spaces, newlines and backslashes, so they're escaped:
//   backslash -> \\     newline -> \n     space -> \s
// After escaping, a key or value never contains a space or newline, so a line splits cleanly.

/// Escape text for the log: "a b\c" becomes "a\sb\\c". The empty string becomes "\0" (so it
/// still takes up a field in the line).
inline std::string escape(const std::string& s) {
    if (s.empty()) return "\\0";
    std::string out;
    for (char c : s) {
        if (c == '\\') out += "\\\\";
        else if (c == '\n') out += "\\n";
        else if (c == ' ') out += "\\s";
        else out += c;
    }
    return out;
}

/// Reverse escape(). Throws std::invalid_argument("bad escape") for a backslash followed by
/// anything other than \ n s 0, or a backslash at the very end.
inline std::string unescape(const std::string& s) {
    if (s == "\\0") return "";
    std::string out;
    for (std::size_t i = 0; i < s.size(); ++i) {
        if (s[i] != '\\') {
            out += s[i];
            continue;
        }
        if (++i == s.size()) throw std::invalid_argument("bad escape");
        switch (s[i]) {
            case '\\': out += '\\'; break;
            case 'n': out += '\n'; break;
            case 's': out += ' '; break;
            default: throw std::invalid_argument("bad escape");
        }
    }
    return out;
}

// ---------------------------------------------------------------- the store

class Store {
public:
    /// An in-memory store. Give it a log stream to make every change durable.
    explicit Store(std::ostream* log = nullptr) : log_(log) {}

    /// Start (or stop, with nullptr) logging changes, e.g. after replaying the old log.
    void setLog(std::ostream* log) { log_ = log; }

    void set(const std::string& key, const std::string& value) {
        if (key.empty()) throw std::invalid_argument("key must not be empty");
        write("SET " + escape(key) + " " + escape(value));
        data_[key] = value;
    }

    std::optional<std::string> get(const std::string& key) const {
        auto it = data_.find(key);
        if (it == data_.end()) return std::nullopt;
        return it->second;
    }

    bool del(const std::string& key) {
        if (!data_.contains(key)) return false;
        write("DEL " + escape(key));
        data_.erase(key);
        return true;
    }

    /// Keys starting with prefix, in sorted order (std::map keeps them sorted for free).
    std::vector<std::string> keys(const std::string& prefix = "") const {
        std::vector<std::string> out;
        for (auto it = data_.lower_bound(prefix); it != data_.end() && it->first.starts_with(prefix); ++it)
            out.push_back(it->first);
        return out;
    }

    std::size_t size() const { return data_.size(); }

    /**
     * Rebuild the data from a log. Each COMPLETE line (ending in '\n') is applied in order:
     * "SET key value" sets, "DEL key" deletes. Returns how many records were applied.
     *
     * A crash can leave a half-written last line with no '\n': ignore it, it was never
     * acknowledged. Any other malformed line (unknown command, wrong number of fields, bad
     * escape) throws std::runtime_error("corrupt log at line N"), counting lines from 1.
     */
    std::size_t replay(std::istream& in) {
        std::size_t applied = 0, number = 0;
        std::string line;
        while (std::getline(in, line)) {
            ++number;
            if (in.eof()) break;                    // no '\n' after it: a torn write, skip it
            std::istringstream fields(line);
            std::string op, key, value, extra;
            fields >> op >> key;
            try {
                if (op == "SET" && (fields >> value) && !(fields >> extra)) {
                    data_[unescape(key)] = unescape(value);
                } else if (op == "DEL" && !key.empty() && !(fields >> extra)) {
                    data_.erase(unescape(key));
                } else {
                    throw std::invalid_argument("bad record");
                }
            } catch (const std::invalid_argument&) {
                throw std::runtime_error("corrupt log at line " + std::to_string(number));
            }
            ++applied;
        }
        return applied;
    }

    /// The smallest log that recreates the current data: one SET per key.
    std::string snapshot() const {
        std::string out;
        for (const auto& [k, v] : data_) out += "SET " + escape(k) + " " + escape(v) + "\n";
        return out;
    }

private:
    void write(const std::string& record) {
        if (!log_) return;
        *log_ << record << '\n';
        log_->flush();                              // durable before we say "OK"
        if (!*log_) throw std::runtime_error("could not write the log");
    }

    std::map<std::string, std::string> data_;
    std::ostream* log_;
};

// ---------------------------------------------------------------- the command language

struct Command {
    std::string op;                 // SET GET DEL KEYS COUNT
    std::vector<std::string> args;
};

/**
 * Parse one line typed by a user. The command name is case-insensitive and returned in upper
 * case. SET takes a key and a value, where the value is the REST of the line (it may contain
 * spaces): "SET motd Welcome to Tokyo" has the value "Welcome to Tokyo". GET and DEL take one
 * key; KEYS takes an optional prefix; COUNT takes nothing. Errors throw std::invalid_argument:
 * "empty command", "unknown command: FOO", "wrong number of arguments for SET".
 */
inline Command parseCommand(const std::string& line) {
    std::istringstream in(line);
    std::string op;
    if (!(in >> op)) throw std::invalid_argument("empty command");
    for (auto& c : op) c = static_cast<char>(std::toupper(static_cast<unsigned char>(c)));
    std::vector<std::string> args;
    if (op == "SET") {
        std::string key, value;
        in >> key;
        std::getline(in >> std::ws, value);
        if (key.empty() || value.empty()) throw std::invalid_argument("wrong number of arguments for SET");
        return {op, {key, value}};
    }
    for (std::string word; in >> word;) args.push_back(word);
    if (op == "GET" || op == "DEL") {
        if (args.size() != 1) throw std::invalid_argument("wrong number of arguments for " + op);
    } else if (op == "KEYS") {
        if (args.size() > 1) throw std::invalid_argument("wrong number of arguments for KEYS");
    } else if (op == "COUNT") {
        if (!args.empty()) throw std::invalid_argument("wrong number of arguments for COUNT");
    } else {
        throw std::invalid_argument("unknown command: " + op);
    }
    return {op, args};
}

/// Run a command and return the reply, in the style of redis-cli.
inline std::string execute(Store& store, const Command& cmd) {
    if (cmd.op == "SET") {
        store.set(cmd.args[0], cmd.args[1]);
        return "OK";
    }
    if (cmd.op == "GET") {
        auto v = store.get(cmd.args[0]);
        return v ? "\"" + *v + "\"" : "(nil)";
    }
    if (cmd.op == "DEL") return store.del(cmd.args[0]) ? "(integer) 1" : "(integer) 0";
    if (cmd.op == "COUNT") return "(integer) " + std::to_string(store.size());
    auto keys = store.keys(cmd.args.empty() ? "" : cmd.args[0]);
    if (keys.empty()) return "(empty)";
    std::string out;
    for (std::size_t i = 0; i < keys.size(); ++i) out += (i ? "\n" : "") + std::to_string(i + 1) + ") " + keys[i];
    return out;
}

/// Replace the log file with a snapshot, safely: write a new file, then rename it over the old one.
inline void compactFile(const Store& store, const std::filesystem::path& path) {
    auto tmp = path;
    tmp += ".tmp";
    {
        std::ofstream out(tmp, std::ios::binary | std::ios::trunc);
        out << store.snapshot();
        out.flush();
        if (!out) throw std::runtime_error("could not write " + tmp.string());
    }
    std::filesystem::rename(tmp, path);
}

}  // namespace kv
@@file tests.cpp
// tests.cpp: a tiny test runner, so the project needs nothing installed.
//   g++ -std=c++20 -o tests tests.cpp && ./tests       (or: cl /EHsc /std:c++20 tests.cpp && tests.exe)
#include <filesystem>
#include <fstream>
#include <functional>
#include <iostream>
#include <sstream>
#include <string>

#include "kv.hpp"

using namespace kv;

static int passed = 0, failed = 0;

static void check(const std::string& name, const std::function<bool()>& test) {
    try {
        if (test()) {
            ++passed;
            std::cout << "  ok    " << name << '\n';
            return;
        }
        std::cout << "  FAIL  " << name << '\n';
    } catch (const std::exception& e) {
        if (std::string(e.what()).starts_with("TODO")) {
            std::cout << "  TODO  " << name << '\n';
            ++failed;
            return;
        }
        std::cout << "  FAIL  " << name << ": exception: " << e.what() << '\n';
    }
    ++failed;
}

template <typename E, typename F>
static std::string errorOf(F f) {
    try {
        f();
    } catch (const E& e) {
        return e.what();
    }
    return "no error";
}

int main() {
    std::cout << "escape / unescape\n";
    check("plain text is unchanged", [] { return escape("hello") == "hello" && unescape("hello") == "hello"; });
    check("spaces, newlines and backslashes", [] { return escape("a b\nc\\d") == "a\\sb\\nc\\\\d"; });
    check("empty string", [] { return escape("") == "\\0" && unescape("\\0").empty(); });
    check("round trip", [] {
        for (std::string s : {"", " ", "\\", "\\s", "two  spaces", "line1\nline2", "東京 office", "\\0"})
            if (unescape(escape(s)) != s) return false;
        return true;
    });
    check("escaped text has no spaces or newlines", [] {
        auto e = escape("a b\nc");
        return e.find(' ') == std::string::npos && e.find('\n') == std::string::npos;
    });
    check("bad escapes throw", [] {
        return errorOf<std::invalid_argument>([] { unescape("a\\x"); }) == "bad escape"
            && errorOf<std::invalid_argument>([] { unescape("trailing\\"); }) == "bad escape";
    });

    std::cout << "Store\n";
    check("set, get, del, size", [] {
        Store s;
        s.set("user:1", "yuki");
        s.set("user:1", "ken");
        bool ok = s.get("user:1") == "ken" && !s.get("user:2") && s.size() == 1;
        return ok && s.del("user:1") && !s.del("user:1") && s.size() == 0;
    });
    check("keys by prefix, sorted", [] {
        Store s;
        for (std::string k : {"user:2", "host:web01", "user:10", "user:1", "users"}) s.set(k, "x");
        return s.keys("user:") == std::vector<std::string>{"user:1", "user:10", "user:2"} && s.keys().size() == 5
            && s.keys("zzz").empty();
    });
    check("every change is logged", [] {
        std::ostringstream log;
        Store s(&log);
        s.set("motd", "Welcome to Tokyo");
        s.del("motd");
        s.del("missing");                       // nothing deleted: nothing logged
        return log.str() == "SET motd Welcome\\sto\\sTokyo\nDEL motd\n";
    });
    check("empty keys are refused", [] { Store s; return errorOf<std::invalid_argument>([&] { s.set("", "x"); }) == "key must not be empty"; });

    std::cout << "replay\n";
    check("replays sets and deletes in order", [] {
        std::istringstream in("SET a 1\nSET b 2\nSET a 3\nDEL b\nSET c hello\\sworld\n");
        Store s;
        auto n = s.replay(in);
        return n == 5 && s.get("a") == "3" && !s.get("b") && s.get("c") == "hello world" && s.size() == 2;
    });
    check("a log written by one store rebuilds another", [] {
        std::ostringstream log;
        Store a(&log);
        a.set("note", "line1\nline2 with  spaces");
        a.set("empty", "");
        a.set("path", "C:\\Users\\yuki");
        std::istringstream in(log.str());
        Store b;
        b.replay(in);
        return b.get("note") == "line1\nline2 with  spaces" && b.get("empty") == "" && b.get("path") == "C:\\Users\\yuki";
    });
    check("a torn last line is ignored", [] {
        std::istringstream in("SET a 1\nSET b 2\nSET c 3");      // the crash happened mid-write
        Store s;
        return s.replay(in) == 2 && !s.get("c");
    });
    check("an empty log is fine", [] { std::istringstream in(""); Store s; return s.replay(in) == 0; });
    check("corruption is reported with the line number", [] {
        auto err = [](const std::string& text) {
            return errorOf<std::runtime_error>([&] { std::istringstream in(text); Store s; s.replay(in); });
        };
        return err("SET a 1\nPUT b 2\n") == "corrupt log at line 2"
            && err("SET a\n") == "corrupt log at line 1"
            && err("SET a 1 2\n") == "corrupt log at line 1"
            && err("SET a 1\nDEL\n") == "corrupt log at line 2"
            && err("SET a b\\q\n") == "corrupt log at line 1";
    });

    std::cout << "parseCommand / execute\n";
    check("SET keeps the rest of the line", [] {
        auto c = parseCommand("set motd   Welcome to  Tokyo");
        return c.op == "SET" && c.args == std::vector<std::string>{"motd", "Welcome to  Tokyo"};
    });
    check("other commands", [] {
        return parseCommand("GET a").args.size() == 1 && parseCommand("keys").args.empty()
            && parseCommand("KEYS user:").args[0] == "user:" && parseCommand("count").op == "COUNT";
    });
    check("errors", [] {
        auto err = [](const std::string& line) { return errorOf<std::invalid_argument>([&] { parseCommand(line); }); };
        return err("") == "empty command" && err("   ") == "empty command" && err("FLY away") == "unknown command: FLY"
            && err("SET onlykey") == "wrong number of arguments for SET" && err("GET a b") == "wrong number of arguments for GET"
            && err("DEL") == "wrong number of arguments for DEL" && err("COUNT x") == "wrong number of arguments for COUNT";
    });
    check("replies look like redis-cli", [] {
        Store s;
        auto run = [&](const std::string& line) { return execute(s, parseCommand(line)); };
        return run("SET a hello there") == "OK" && run("GET a") == "\"hello there\"" && run("GET b") == "(nil)"
            && run("SET b 2") == "OK" && run("KEYS") == "1) a\n2) b" && run("KEYS z") == "(empty)"
            && run("DEL a") == "(integer) 1" && run("DEL a") == "(integer) 0" && run("COUNT") == "(integer) 1";
    });

    std::cout << "compaction\n";
    check("compactFile writes one SET per key, and it replays", [] {
        auto path = std::filesystem::temp_directory_path() / "kv_compact_test.log";
        Store s;
        s.set("b", "2");
        s.set("a", "1 1");
        compactFile(s, path);
        std::ifstream in(path, std::ios::binary);
        std::stringstream text;
        text << in.rdbuf();
        in.close();
        std::istringstream again(text.str());
        Store t;
        bool ok = text.str() == "SET a 1\\s1\nSET b 2\n" && t.replay(again) == 2 && t.get("a") == "1 1";
        std::filesystem::remove(path);
        return ok;
    });

    std::cout << "\n" << passed << " passed, " << failed << " failed\n";
    return failed == 0 ? 0 : 1;
}
@@file main.cpp
// main.cpp: an interactive shell for the store, like redis-cli.
//   build: g++ -std=c++20 -O2 -o kv main.cpp     (or: cl /EHsc /std:c++20 /O2 main.cpp)
//   run:   ./kv data.log
// Type HELP for commands. Everything you SET survives a restart, because it's in data.log.
#include <fstream>
#include <iostream>

#include "kv.hpp"

int main(int argc, char** argv) {
    const std::string path = argc > 1 ? argv[1] : "data.log";
    kv::Store store;
    try {
        std::ifstream existing(path, std::ios::binary);
        std::size_t records = existing ? store.replay(existing) : 0;
        std::cout << "loaded " << store.size() << " key(s) from " << records << " record(s) in " << path << "\n";
    } catch (const std::exception& e) {
        std::cerr << "error: " << e.what() << "\n";
        return 1;
    }

    // Start each session from a compacted log (one SET per key), then append every change to it.
    kv::compactFile(store, path);
    std::ofstream log(path, std::ios::binary | std::ios::app);
    store.setLog(&log);

    std::string line;
    std::cout << "kv> " << std::flush;
    while (std::getline(std::cin, line)) {
        if (line == "QUIT" || line == "quit" || line == "EXIT" || line == "exit") break;
        if (line == "HELP" || line == "help") {
            std::cout << "SET key value | GET key | DEL key | KEYS [prefix] | COUNT | QUIT\n";
        } else if (!line.empty()) {
            try {
                std::cout << kv::execute(store, kv::parseCommand(line)) << "\n";
            } catch (const std::exception& e) {
                std::cout << "(error) " << e.what() << "\n";
            }
        }
        std::cout << "kv> " << std::flush;
    }
    return 0;
}
@@text
🎉 Write-ahead logs, atomic renames, compaction and torn-write handling are the foundations of every database, file system and message queue. You now understand what "durable" means and what it costs.
