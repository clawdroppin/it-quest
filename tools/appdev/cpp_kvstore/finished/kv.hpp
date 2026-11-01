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
