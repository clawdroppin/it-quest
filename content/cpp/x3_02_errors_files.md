@@lesson x3-errors-files Errors, Files and Parsing
@@concepts errors, files, text-processing
@@text
## Exceptions

```cpp
#include <iostream>
#include <stdexcept>
#include <string>

int parsePort(const std::string& s) {
    std::size_t used = 0;
    int port = std::stoi(s, &used);                 // throws std::invalid_argument or std::out_of_range
    if (used != s.size()) throw std::invalid_argument("trailing characters in '" + s + "'");
    if (port < 1 || port > 65535) throw std::out_of_range("port out of range: " + s);
    return port;
}

int main() {
    for (std::string s : {"443", "8o8o", "70000", "22x"}) {
        try {
            std::cout << parsePort(s) << '\n';
        } catch (const std::exception& e) {          // catch by const reference
            std::cout << "bad: " << e.what() << '\n';
        }
    }
}
```

- Throw standard types (`std::invalid_argument`, `std::runtime_error`, `std::out_of_range`) or your own class derived from `std::exception`.
- **Catch by `const&`**, most specific first.
- RAII cleans up during the "unwinding", so exceptions don't leak.

## Errors without exceptions: std::optional

Some code (games, embedded, Google's style) avoids exceptions. `std::optional<T>` says "a T, or nothing":

```cpp
#include <iostream>
#include <optional>
#include <string>

std::optional<int> tryPort(const std::string& s) {
    try {
        int p = std::stoi(s);
        if (p >= 1 && p <= 65535) return p;
    } catch (...) {}
    return std::nullopt;
}

int main() {
    if (auto p = tryPort("8443")) std::cout << "port " << *p << '\n';
    std::cout << tryPort("abc").value_or(80) << '\n';
}
```

C++23 adds `std::expected<T, E>` for "a value or an error", like Rust's `Result`.

## Files with fstream

```cpp
#include <filesystem>
#include <fstream>
#include <iostream>
#include <string>

namespace fs = std::filesystem;

int main() {
    fs::path file = fs::temp_directory_path() / "itquest_demo.log";   // / joins paths portably

    {
        std::ofstream out(file);                    // open for writing (RAII: closes itself)
        out << "INFO boot\nERROR disk full\nINFO login\n";
    }

    std::ifstream in(file);
    if (!in) {
        std::cerr << "can't open " << file << '\n';
        return 1;
    }
    std::string line;
    int errors = 0;
    while (std::getline(in, line))
        if (line.starts_with("ERROR")) ++errors;
    std::cout << errors << " error(s), " << fs::file_size(file) << " bytes\n";
    in.close();
    fs::remove(file);
}
```

`std::filesystem` (C++17) lists directories (`fs::directory_iterator`), copies, renames, and checks space (`fs::space`): everything a cleanup or backup tool needs.

## Splitting text with stringstream

`std::istringstream` lets you use `>>` on a string, which is the easiest way to split on whitespace:

```cpp
#include <iostream>
#include <sstream>
#include <string>
#include <vector>

std::vector<std::string> split(const std::string& s, char sep) {
    std::vector<std::string> parts;
    std::istringstream in(s);
    std::string part;
    while (std::getline(in, part, sep)) parts.push_back(part);
    return parts;
}

int main() {
    std::istringstream line("2026-10-31 12:00:01 ERROR db01 timeout");
    std::string date, time, level, host;
    line >> date >> time >> level >> host;
    std::cout << level << " on " << host << '\n';

    for (const auto& p : split("10.0.0.1,web01,prod", ',')) std::cout << '[' << p << ']';
    std::cout << '\n';
}
```

For more complex patterns there's `<regex>`, but it's slow; for log parsing at scale, plain string functions win.

@@quiz
How should you catch exceptions in C++?
* By const reference: `catch (const std::exception& e)`
- By value: `catch (std::exception e)`
- By pointer: `catch (std::exception* e)`
- Only with `catch (...)`
? Catching by value copies and "slices" derived exceptions down to the base type.

@@quiz
What does `std::optional<int>` hold?
* Either an `int` or nothing (`std::nullopt`)
- An `int` that defaults to 0
- A pointer to an `int`
- Several `int`s
? Use `has_value()`, `*opt`, or `value_or(default)`.

@@quiz
`fs::path p = dir / "app.log";` What does `/` do here?
* Joins path parts with the right separator for the OS
- Divides numbers
- Nothing: it's a syntax error
- Makes the path absolute
? `std::filesystem::path` overloads `/` for joining.

@@exercise Parse key=value config
Read the config from input. Skip empty lines and lines starting with `#`. For each `key=value` line, store it in a `std::map` (trim spaces around key and value). Lines without `=` print `line <n>: missing '='` (counting all lines from 1). At the end, print each key and value as `key -> value`, sorted.
@@stdin
# web server
port = 8443

host=web01
loglevel
 timeout =30
@@starter
#include <iostream>
#include <map>
#include <string>

std::string trim(const std::string& s) {
    auto a = s.find_first_not_of(' ');
    if (a == std::string::npos) return "";
    return s.substr(a, s.find_last_not_of(' ') - a + 1);
}

int main() {
    std::map<std::string, std::string> cfg;
    std::string line;
    int n = 0;
    // read lines with std::getline
}
@@expected
line 5: missing '='
host -> web01
port -> 8443
timeout -> 30
@@hint
`while (std::getline(std::cin, line)) { ++n; if (trim(line).empty() || line[0] == '#') continue; auto eq = line.find('='); ... cfg[trim(line.substr(0, eq))] = trim(line.substr(eq + 1)); }`
@@solution
#include <iostream>
#include <map>
#include <string>

std::string trim(const std::string& s) {
    auto a = s.find_first_not_of(' ');
    if (a == std::string::npos) return "";
    return s.substr(a, s.find_last_not_of(' ') - a + 1);
}

int main() {
    std::map<std::string, std::string> cfg;
    std::string line;
    int n = 0;
    while (std::getline(std::cin, line)) {
        ++n;
        if (trim(line).empty() || line[0] == '#') continue;
        auto eq = line.find('=');
        if (eq == std::string::npos) {
            std::cout << "line " << n << ": missing '='\n";
            continue;
        }
        cfg[trim(line.substr(0, eq))] = trim(line.substr(eq + 1));
    }
    for (const auto& [k, v] : cfg) std::cout << k << " -> " << v << '\n';
}

@@exercise Optional lookup
Write `std::optional<std::string> ownerOf(const std::map<std::string, std::string>& owners, const std::string& host)` that returns the owner, or `std::nullopt` if the host isn't in the map. Don't use `operator[]` (it can't be used on a const map anyway).
@@starter
#include <iostream>
#include <map>
#include <optional>
#include <string>

std::optional<std::string> ownerOf(const std::map<std::string, std::string>& owners, const std::string& host) {
    return std::nullopt; // TODO: not done yet
}

int main() {
    const std::map<std::string, std::string> owners{{"web01", "aiko"}, {"db01", "kenji"}};
    for (std::string h : {"db01", "dns01", "web01"})
        std::cout << h << ": " << ownerOf(owners, h).value_or("(nobody)") << '\n';
}
@@expected
db01: kenji
dns01: (nobody)
web01: aiko
@@requires
find
@@hint
`auto it = owners.find(host); if (it == owners.end()) return std::nullopt; return it->second;`
@@solution
#include <iostream>
#include <map>
#include <optional>
#include <string>

std::optional<std::string> ownerOf(const std::map<std::string, std::string>& owners, const std::string& host) {
    auto it = owners.find(host);
    if (it == owners.end()) return std::nullopt;
    return it->second;
}

int main() {
    const std::map<std::string, std::string> owners{{"web01", "aiko"}, {"db01", "kenji"}};
    for (std::string h : {"db01", "dns01", "web01"})
        std::cout << h << ": " << ownerOf(owners, h).value_or("(nobody)") << '\n';
}
