@@lesson n7-builds Builds and Dependencies: Modern CMake, vcpkg and Conan
@@concepts modules, automation
@@text
C++ has no single package manager or build tool, so every professional project needs a deliberate setup. The de facto standard is **CMake** plus **vcpkg** or **Conan**.

## Modern CMake: targets, not variables

```cmake,norun
cmake_minimum_required(VERSION 3.25)
project(inventory VERSION 1.4.0 LANGUAGES CXX)

set(CMAKE_CXX_STANDARD 20)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

find_package(fmt CONFIG REQUIRED)            # provided by vcpkg or Conan
find_package(GTest CONFIG REQUIRED)

add_library(core src/inventory.cpp src/parser.cpp)
target_include_directories(core PUBLIC include)
target_link_libraries(core PUBLIC fmt::fmt)  # PUBLIC: users of core also get fmt
target_compile_options(core PRIVATE
    $<$<CXX_COMPILER_ID:MSVC>:/W4 /permissive->
    $<$<NOT:$<CXX_COMPILER_ID:MSVC>>:-Wall -Wextra -Wpedantic>)

add_executable(inventory src/main.cpp)
target_link_libraries(inventory PRIVATE core)

enable_testing()
add_executable(core_tests tests/parser_test.cpp)
target_link_libraries(core_tests PRIVATE core GTest::gtest_main)
include(GoogleTest)
gtest_discover_tests(core_tests)
```

- `PRIVATE`: used only to build this target. `PUBLIC`: used here and by anyone linking to it. `INTERFACE`: only by users (header-only libraries).
- Build out of source: `cmake -S . -B build -G Ninja -DCMAKE_BUILD_TYPE=Release`, then `cmake --build build` and `ctest --test-dir build`.
- **CMakePresets.json** stores configurations (debug, release, asan, ci) so everyone builds the same way.

## Dependency managers

**vcpkg** (Microsoft) builds libraries from source with a manifest:

```json,norun
{ "name": "inventory", "dependencies": [ "fmt", "gtest", { "name": "asio", "version>=": "1.28.0" } ] }
```

Point CMake at it with `-DCMAKE_TOOLCHAIN_FILE=<vcpkg>/scripts/buildsystems/vcpkg.cmake`. **Conan** (JFrog) does the same with a `conanfile.txt`/`conanfile.py`, binary caching and version ranges like `fmt/[>=10 <11]`.

Both resolve a **dependency graph**: build order is a topological sort, independent targets build in parallel, and cycles are errors. Version ranges are resolved to the newest version that satisfies every constraint.

## Static vs shared libraries and ABI

A static library (`.lib`/`.a`) is copied into your executable. A shared one (`.dll`/`.so`/`.dylib`) is loaded at runtime and can be updated separately, but only if its **ABI** (binary interface: class layouts, name mangling, calling conventions) stays compatible. C++ ABIs break easily (add a member, change a compiler), which is why stable plugin interfaces often use a C API (see the interop lesson).

@@quiz
What does `target_link_libraries(core PUBLIC fmt::fmt)` mean?
* core uses fmt, and anything that links core also gets fmt's include paths and link settings
- fmt is linked only into core
- fmt becomes part of the public API documentation
- It downloads fmt
? PRIVATE would keep it internal to core.

@@quiz
In what order must a build system compile targets?
- Alphabetical
- The order they appear in CMakeLists.txt
* A topological order of the dependency graph: every target after its dependencies
- Random
? Targets with no path between them can build in parallel.

@@quiz
What does `^10.0.0` usually mean in a version constraint?
- Exactly 10.0.0
* At least 10.0.0 and below 11.0.0 (compatible with 10.x)
- Any version above 10
- 10.0.x only
? `~10.0` would mean at least 10.0.0 and below 10.1.0.

@@flashcards
`add_library` / `add_executable` :: define targets
`target_link_libraries` :: dependencies with PUBLIC/PRIVATE/INTERFACE
`find_package(X CONFIG REQUIRED)` :: locate an installed package
CMakePresets.json :: shared configure/build presets
vcpkg manifest :: vcpkg.json with dependencies
Conan :: package manager with binary caching
Topological sort :: dependencies before dependents
ABI :: binary compatibility of compiled code
`^1.2.3` :: >=1.2.3 <2.0.0
`~1.2` :: >=1.2.0 <1.3.0

@@exercise Build order and parallel waves
Given each target's dependencies, compute:

- a build order with **Kahn's algorithm**, always picking the alphabetically first ready target (keep the ready targets in a `std::set`)
- the parallel build **waves**: a target's wave is 1 + the highest wave among its dependencies
- with a cycle, Kahn's algorithm can't finish: report the targets that never became ready, sorted
@@starter
#include <iostream>
#include <map>
#include <set>
#include <string>
#include <vector>

using Graph = std::map<std::string, std::vector<std::string>>;

int main() {
    Graph deps = {
        {"app", {"core", "net", "ui"}}, {"net", {"core", "ssl"}}, {"ui", {"core"}},
        {"tests", {"app", "gtest"}},    {"core", {}},             {"ssl", {}},  {"gtest", {}}};
    std::cout << "TODO\n";
}
@@expected
build order: core gtest ssl net ui app tests
wave 1: core gtest ssl
wave 2: net ui
wave 3: app
wave 4: tests
cycle detected among: app core net tests ui
@@requires
std::set<std::string>
@@hint
Build `remaining[target] = deps.size()` and a reverse map `dependents[dep]` → targets. Start `ready` with the targets whose count is 0; repeatedly take `*ready.begin()`, append it, and decrement its dependents, adding those that reach 0. Compute waves in build order with a `std::map<std::string, int>`. For the cycle, set `deps["core"] = {"app"}`.
@@solution
#include <algorithm>
#include <iostream>
#include <map>
#include <set>
#include <string>
#include <vector>

using Graph = std::map<std::string, std::vector<std::string>>;

struct Order {
    std::vector<std::string> order;
    std::set<std::string> stuck;
};

Order topo(const Graph& deps) {
    std::map<std::string, size_t> remaining;
    std::map<std::string, std::vector<std::string>> dependents;
    for (const auto& [target, ds] : deps) {
        remaining[target] = ds.size();
        for (const auto& d : ds) dependents[d].push_back(target);
    }
    std::set<std::string> ready;
    for (const auto& [target, n] : remaining)
        if (n == 0) ready.insert(target);

    Order result;
    while (!ready.empty()) {
        std::string next = *ready.begin();
        ready.erase(ready.begin());
        result.order.push_back(next);
        for (const auto& t : dependents[next])
            if (--remaining[t] == 0) ready.insert(t);
    }
    for (const auto& [target, n] : remaining)
        if (n > 0) result.stuck.insert(target);
    return result;
}

int main() {
    Graph deps = {
        {"app", {"core", "net", "ui"}}, {"net", {"core", "ssl"}}, {"ui", {"core"}},
        {"tests", {"app", "gtest"}},    {"core", {}},             {"ssl", {}},  {"gtest", {}}};

    Order o = topo(deps);
    std::cout << "build order:";
    for (const auto& t : o.order) std::cout << ' ' << t;
    std::cout << '\n';

    std::map<std::string, int> wave;
    int max_wave = 0;
    for (const auto& t : o.order) {
        int w = 1;
        for (const auto& d : deps.at(t)) w = std::max(w, wave[d] + 1);
        wave[t] = w;
        max_wave = std::max(max_wave, w);
    }
    for (int w = 1; w <= max_wave; ++w) {
        std::cout << "wave " << w << ':';
        for (const auto& t : o.order)
            if (wave[t] == w) std::cout << ' ' << t;
        std::cout << '\n';
    }

    deps["core"] = {"app"};
    Order cyclic = topo(deps);
    std::cout << "cycle detected among:";
    for (const auto& t : cyclic.stuck) std::cout << ' ' << t;
    std::cout << '\n';
}

@@exercise Resolve version constraints
Pick the newest available version that satisfies a constraint, like vcpkg and Conan do:

- versions have up to 3 numeric parts; missing parts are 0 (`11` is `11.0.0`)
- a constraint is space-separated clauses, all of which must hold: `>=`, `>`, `<=`, `<`, `=`, `^X.Y.Z` (≥ X.Y.Z and < (X+1).0.0) and `~X.Y[.Z]` (≥ X.Y.Z and < X.(Y+1).0)
- print the chosen version, or `no version of fmt satisfies <constraint>`
@@starter
#include <algorithm>
#include <charconv>
#include <compare>
#include <iostream>
#include <optional>
#include <sstream>
#include <string>
#include <string_view>
#include <vector>

int main() {
    std::vector<std::string> available = {"8.1.1", "9.1.0", "10.0.0", "10.2.1", "11.0.2"};
    std::vector<std::string> constraints = {">=9.0 <11", "^10.0.0", "~10.0", "=9.1.0", ">=12"};
    std::cout << "TODO\n";
}
@@expected
>=9.0 <11 -> 10.2.1
^10.0.0 -> 10.2.1
~10.0 -> 10.0.0
=9.1.0 -> 9.1.0
>=12 -> no version of fmt satisfies >=12
@@requires
operator<=>
@@hint
`struct Version { int major = 0, minor = 0, patch = 0; auto operator<=>(const Version&) const = default; };`. Split the clause into an operator (`>=`, `<=` first, then one character) and a version. Turn each clause into a predicate `bool satisfies(const Version& v, std::string_view clause)`, and split the constraint with `std::istringstream`. Check the available versions from newest to oldest.
@@solution
#include <algorithm>
#include <charconv>
#include <compare>
#include <functional>
#include <iostream>
#include <optional>
#include <sstream>
#include <string>
#include <string_view>
#include <vector>

struct Version {
    int major = 0, minor = 0, patch = 0;
    auto operator<=>(const Version&) const = default;
    std::string str() const { return std::to_string(major) + '.' + std::to_string(minor) + '.' + std::to_string(patch); }
};

Version parse(std::string_view s) {
    Version v;
    int* parts[] = {&v.major, &v.minor, &v.patch};
    const char* p = s.data();
    const char* end = s.data() + s.size();
    for (int i = 0; i < 3 && p < end; ++i) {
        p = std::from_chars(p, end, *parts[i]).ptr;
        if (p < end && *p == '.') ++p;
    }
    return v;
}

bool satisfies(const Version& v, std::string_view clause) {
    std::string_view op;
    for (std::string_view candidate : {">=", "<=", ">", "<", "=", "^", "~"}) {
        if (clause.starts_with(candidate)) {
            op = candidate;
            break;
        }
    }
    std::string_view text = clause.substr(op.size());
    Version c = parse(text);
    if (op == ">=") return v >= c;
    if (op == "<=") return v <= c;
    if (op == ">") return v > c;
    if (op == "<") return v < c;
    if (op == "=") return v == c;
    if (op == "^") return v >= c && v < Version{c.major + 1, 0, 0};
    if (op == "~") return v >= c && v < Version{c.major, c.minor + 1, 0};
    return v == parse(clause);
}

std::optional<Version> resolve(std::vector<Version> available, const std::string& constraint) {
    std::sort(available.begin(), available.end(), std::greater<>{});
    for (const auto& v : available) {
        std::istringstream clauses(constraint);
        std::string clause;
        bool ok = true;
        while (clauses >> clause) ok = ok && satisfies(v, clause);
        if (ok) return v;
    }
    return std::nullopt;
}

int main() {
    std::vector<std::string> available = {"8.1.1", "9.1.0", "10.0.0", "10.2.1", "11.0.2"};
    std::vector<std::string> constraints = {">=9.0 <11", "^10.0.0", "~10.0", "=9.1.0", ">=12"};
    std::vector<Version> versions;
    for (const auto& a : available) versions.push_back(parse(a));
    for (const auto& c : constraints) {
        auto v = resolve(versions, c);
        std::cout << c << " -> " << (v ? v->str() : "no version of fmt satisfies " + c) << '\n';
    }
}
