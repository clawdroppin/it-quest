@@lesson x5-design Designing C++ Applications
@@text
C++ runs the software that has to be fast or close to the hardware: databases, browsers, game engines, trading systems, network equipment (Cisco's IOS is C/C++), car and factory controllers. Writing one 50-line program is different from writing one of those. This lesson covers how C++ applications are organized.

## Headers and source files

Bigger programs split each component into a **header** (`.hpp`: what it offers) and a **source file** (`.cpp`: how it works):

```text
// inventory.hpp: the interface. Other files #include this.
#pragma once
#include <string>
#include <vector>

namespace app {
struct Device { std::string hostname; int ports; };

class Inventory {
public:
    void add(Device d);
    std::vector<Device> withAtLeast(int ports) const;
private:
    std::vector<Device> devices_;
};
}
```

```text
// inventory.cpp: the implementation. Compiled once, on its own.
#include "inventory.hpp"
#include <algorithm>

namespace app {
void Inventory::add(Device d) { devices_.push_back(std::move(d)); }

std::vector<Device> Inventory::withAtLeast(int ports) const {
    std::vector<Device> out;
    std::copy_if(devices_.begin(), devices_.end(), std::back_inserter(out),
                 [ports](const Device& d) { return d.ports >= ports; });
    return out;
}
}
```

Why split? Changing `inventory.cpp` only recompiles that file, not every file that includes the header. On big projects that's the difference between seconds and an hour. Small libraries are often **header-only** (everything `inline` in the `.hpp`), which is simpler to use.

## CMake: the standard build system

```text
# CMakeLists.txt
cmake_minimum_required(VERSION 3.25)
project(netinv VERSION 1.0 LANGUAGES CXX)

set(CMAKE_CXX_STANDARD 20)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

add_library(netinv_core src/inventory.cpp src/parser.cpp)       # the logic, as a library
target_include_directories(netinv_core PUBLIC include)
target_compile_options(netinv_core PRIVATE $<$<CXX_COMPILER_ID:MSVC>:/W4> $<$<NOT:$<CXX_COMPILER_ID:MSVC>>:-Wall -Wextra>)

add_executable(netinv src/main.cpp)                              # the program: a thin main()
target_link_libraries(netinv PRIVATE netinv_core)

enable_testing()
add_executable(netinv_tests tests/test_inventory.cpp)            # tests link the same library
target_link_libraries(netinv_tests PRIVATE netinv_core)
add_test(NAME unit COMMAND netinv_tests)
```

```text
cmake -S . -B build -DCMAKE_BUILD_TYPE=Release
cmake --build build
ctest --test-dir build
```

The same pattern as in every other language: logic in a library, a thin `main`, tests that link the library. **vcpkg** or **Conan** install third-party libraries (`fmt`, `spdlog`, `nlohmann-json`, `asio`, `Catch2`).

## Interfaces with abstract classes

```cpp
#include <iostream>
#include <memory>
#include <string>
#include <vector>

// The interface: what an alert sender must do.
class Notifier {
public:
    virtual ~Notifier() = default;              // always: deleting through a base pointer must be safe
    virtual void send(const std::string& msg) = 0;
};

class ConsoleNotifier : public Notifier {
public:
    void send(const std::string& msg) override { std::cout << "[console] " << msg << '\n'; }
};

class RecordingNotifier : public Notifier {      // a fake for tests
public:
    void send(const std::string& msg) override { sent.push_back(msg); }
    std::vector<std::string> sent;
};

// The logic depends on the interface only.
class DiskMonitor {
public:
    explicit DiskMonitor(Notifier& notifier) : notifier_(notifier) {}
    void check(const std::string& disk, int percent) {
        if (percent >= 90) notifier_.send(disk + " is " + std::to_string(percent) + "% full");
    }
private:
    Notifier& notifier_;
};

int main() {
    ConsoleNotifier console;
    DiskMonitor live(console);
    live.check("C:", 95);

    RecordingNotifier fake;
    DiskMonitor tested(fake);
    tested.check("D:", 40);
    tested.check("E:", 91);
    std::cout << "recorded " << fake.sent.size() << ": " << fake.sent[0] << '\n';
}
```

Passing the dependency by reference (or `std::unique_ptr` when the object should own it) is dependency injection in C++.

## Ownership: say it in the types

| Type | Means |
|---|---|
| `T` (a value member) | I own it; it lives inside me |
| `std::unique_ptr<T>` | I own it alone; it's freed with me (moveable, not copyable) |
| `std::shared_ptr<T>` | shared ownership; freed when the last owner goes (use rarely) |
| `T&` / `const T&` | I use it, someone else owns it and keeps it alive |
| `T*` | I use it, it may be null; I don't own it |
| `std::string_view` / `std::span<T>` | I look at someone else's text / array, briefly |

Never `new`/`delete` by hand in application code. Every resource (memory, files, sockets, locks) is owned by an object whose destructor releases it: **RAII**.

## Error handling strategies

| Situation | Use |
|---|---|
| a value may be absent ("no user with that id") | `std::optional<T>` |
| expected failure with a reason ("bad port") | `std::variant<T, Error>`, or `std::expected<T, E>` in C++23 |
| exceptional, can't continue here (disk full, corrupt file) | throw an exception derived from `std::runtime_error` |
| a bug (broken invariant) | `assert` in debug builds |

```cpp
#include <charconv>
#include <iostream>
#include <optional>
#include <string>
#include <string_view>
#include <variant>

struct ParseError { std::string message; };

std::variant<int, ParseError> parsePort(std::string_view text) {
    int value = 0;
    auto [end, ec] = std::from_chars(text.data(), text.data() + text.size(), value);
    if (ec != std::errc{} || end != text.data() + text.size()) return ParseError{"\"" + std::string(text) + "\" is not a number"};
    if (value < 1 || value > 65535) return ParseError{std::to_string(value) + " is out of range"};
    return value;
}

int main() {
    for (std::string_view input : {"443", "http", "70000"}) {
        auto result = parsePort(input);
        if (auto* port = std::get_if<int>(&result)) std::cout << "port " << *port << '\n';
        else std::cout << "error: " << std::get<ParseError>(result).message << '\n';
    }
}
```

Many teams (Google, game studios, embedded) turn exceptions off entirely and use return values everywhere; know both styles.

@@quiz
Why put a class's implementation in a `.cpp` file instead of the header?
- Headers can't contain functions
* Changing the implementation then only recompiles that one file, not everything that includes the header
- The linker requires it
- It makes the program faster at run time
? Build times matter on large codebases. (Templates and small inline functions still go in headers.)

@@quiz
Why does an interface class need `virtual ~Notifier() = default;`?
* So deleting a derived object through a `Notifier*` runs the derived destructor
- To make the class abstract
- So it can be copied
- Because CMake requires it
? Without a virtual destructor, `delete basePtr` only runs the base destructor: undefined behaviour and leaks.

@@quiz
A function needs to read a big `std::vector<Device>` that belongs to the caller. What parameter type?
- `std::vector<Device>` (by value)
* `const std::vector<Device>&` (or `std::span<const Device>`)
- `std::shared_ptr<std::vector<Device>>`
- `std::vector<Device>*`
? A const reference looks at the caller's data without copying or taking ownership.

@@quiz
In CMake, why build the logic as a library (`add_library`) and `main.cpp` as a separate executable?
- Executables can't contain classes
* The test executable can link the same library, testing exactly the code the program uses
- Libraries are always faster
- vcpkg requires it
? One library, two consumers: the program and the tests.

@@drill
What's the name of a class with only pure virtual functions, used as an interface?
= abstract class
= an abstract class
= interface
? virtual void send(const Msg&) = 0;

@@flashcards
header (`.hpp`) / source (`.cpp`) :: interface / implementation, compiled separately
`#pragma once` :: include this header only once per compilation
header-only library :: everything in headers, nothing to link
CMake :: the standard C++ build system generator (`cmake -S . -B build`)
`add_library` / `add_executable` / `target_link_libraries` :: CMake building blocks
vcpkg / Conan :: C++ package managers
abstract class :: has `virtual ... = 0`; can't be created; used as an interface
`virtual ~Base() = default;` :: required in any class used polymorphically
RAII :: resources released by destructors
`std::optional` / `std::variant` / `std::expected` :: maybe a value / one of several types / value or error (C++23)
`std::from_chars` :: fast, exception-free number parsing

@@exercise An interface and a fake
Write the abstract class `Clock` (a virtual `int hour() const` that returns 0-23, and a virtual destructor), a `FixedClock` whose hour is set in its constructor, and `greeting(const Clock&)` returning `"Good morning"` before 12, `"Good afternoon"` before 18, and `"Good evening"` otherwise.
@@starter
#include <iostream>
#include <string>

// write Clock, FixedClock and greeting here

int main() {
    for (int h : {7, 12, 17, 18, 23}) {
        FixedClock clock(h);
        std::cout << h << ": " << greeting(clock) << '\n';
    }
}
@@expected
7: Good morning
12: Good afternoon
17: Good afternoon
18: Good evening
23: Good evening
@@requires
virtual
override
@@hint
`class Clock { public: virtual ~Clock() = default; virtual int hour() const = 0; };` and `class FixedClock : public Clock { public: explicit FixedClock(int h) : h_(h) {} int hour() const override { return h_; } private: int h_; };`.
@@solution
#include <iostream>
#include <string>

class Clock {
public:
    virtual ~Clock() = default;
    virtual int hour() const = 0;
};

class FixedClock : public Clock {
public:
    explicit FixedClock(int h) : h_(h) {}
    int hour() const override { return h_; }
private:
    int h_;
};

std::string greeting(const Clock& clock) {
    int h = clock.hour();
    if (h < 12) return "Good morning";
    if (h < 18) return "Good afternoon";
    return "Good evening";
}

int main() {
    for (int h : {7, 12, 17, 18, 23}) {
        FixedClock clock(h);
        std::cout << h << ": " << greeting(clock) << '\n';
    }
}

@@exercise Optional lookups
Write `std::optional<std::string> ownerOf(const std::map<std::string, std::string>& assets, const std::string& tag)` that returns the owner or nothing, and use `value_or` in `main` to print `PC-0142: yuki`, `PC-9999: (unknown)` and `MAC-0007: ken`.
@@starter
#include <iostream>
#include <map>
#include <optional>
#include <string>

// write ownerOf here

int main() {
    std::map<std::string, std::string> assets{{"PC-0142", "yuki"}, {"MAC-0007", "ken"}};
    for (std::string tag : {"PC-0142", "PC-9999", "MAC-0007"}) {
        // print "<tag>: <owner or (unknown)>"
    }
}
@@expected
PC-0142: yuki
PC-9999: (unknown)
MAC-0007: ken
@@requires
std::optional<std::string> ownerOf
value_or
@@hint
`auto it = assets.find(tag); if (it == assets.end()) return std::nullopt; return it->second;`. Print with `ownerOf(assets, tag).value_or("(unknown)")`.
@@solution
#include <iostream>
#include <map>
#include <optional>
#include <string>

std::optional<std::string> ownerOf(const std::map<std::string, std::string>& assets, const std::string& tag) {
    auto it = assets.find(tag);
    if (it == assets.end()) return std::nullopt;
    return it->second;
}

int main() {
    std::map<std::string, std::string> assets{{"PC-0142", "yuki"}, {"MAC-0007", "ken"}};
    for (std::string tag : {"PC-0142", "PC-9999", "MAC-0007"}) {
        std::cout << tag << ": " << ownerOf(assets, tag).value_or("(unknown)") << '\n';
    }
}
