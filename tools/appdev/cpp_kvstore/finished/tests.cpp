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
