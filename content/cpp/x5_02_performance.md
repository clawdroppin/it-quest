@@lesson x5-performance Performance: Measuring and Making C++ Fast
@@text
People choose C++ for speed, but C++ isn't automatically fast. A careless C++ program can be slower than good Python. Performance work has one rule above all others: **measure first**. Guesses about what's slow are usually wrong.

## Measuring with std::chrono

```cpp
#include <chrono>
#include <iostream>
#include <numeric>
#include <vector>

int main() {
    using clock = std::chrono::steady_clock;          // steady: never jumps when the PC's time changes
    std::vector<long long> v(5'000'000);
    std::iota(v.begin(), v.end(), 1);

    auto start = clock::now();
    long long sum = std::accumulate(v.begin(), v.end(), 0LL);
    auto ms = std::chrono::duration_cast<std::chrono::microseconds>(clock::now() - start).count() / 1000.0;

    std::cout << "sum = " << sum << '\n';
    std::cout << "took about " << (ms < 1000 ? "under a second" : "over a second") << '\n';
}
```

Rules for honest benchmarks: build in **Release** mode (`-O2` / `/O2`; Debug builds can be 10-100× slower), run several times, use realistic data, and make sure the result is used (or the compiler may delete the work). For real profiling use a **profiler**: Visual Studio's Performance Profiler, `perf` on Linux, Intel VTune; and **Google Benchmark** for micro-benchmarks.

## Big-O beats micro-tricks

The biggest wins come from the algorithm and the data structure:

| Need | Slow | Fast |
|---|---|---|
| "is this IP in the blocklist?" (100,000 IPs) | `std::find` in a vector: O(n) | `std::unordered_set`: O(1) |
| keep items sorted while inserting | sort after each insert: O(n log n) each | `std::map` / `std::set`: O(log n) |
| remove many items from a vector | `erase` one by one: O(n²) | `std::erase_if` (one pass): O(n) |
| build a long string | `s = s + piece` in a loop | `s += piece`, with `s.reserve(...)` |

## Avoiding copies

```cpp
#include <iostream>
#include <string>
#include <string_view>
#include <vector>

struct Big {
    std::vector<int> data = std::vector<int>(1000, 7);
    Big() = default;
    Big(const Big& other) : data(other.data) { ++copies; }   // count copies
    Big(Big&&) noexcept = default;
    static inline int copies = 0;
};

long sumByValue(Big b) { return static_cast<long>(b.data.size()); }         // copies every time
long sumByRef(const Big& b) { return static_cast<long>(b.data.size()); }    // no copy

bool isAdmin(std::string_view user) { return user.starts_with("admin"); }   // works for literals and strings, no copy

int main() {
    Big b;
    for (int i = 0; i < 3; ++i) sumByValue(b);
    std::cout << "by value: " << Big::copies << " copies\n";
    Big::copies = 0;
    for (int i = 0; i < 3; ++i) sumByRef(b);
    std::cout << "by const&: " << Big::copies << " copies\n";

    std::vector<Big> all;
    all.reserve(2);                 // no reallocation while adding
    all.push_back(std::move(b));    // moved: the vector steals b's buffer
    all.emplace_back();             // built directly inside the vector
    std::cout << "after moves: " << Big::copies << " copies\n";
    std::cout << isAdmin("admin-yuki") << isAdmin(std::string("ken")) << '\n';
}
```

## Memory layout matters

Modern CPUs are fast; **memory** is slow. Reading data that's next to each other in memory (a `std::vector`) is many times faster than chasing pointers (a `std::list`, a vector of `unique_ptr`s), because the CPU loads whole **cache lines** (64 bytes) and predicts sequential access.

- Prefer `std::vector` by default. Even for inserting in the middle, it often beats `std::list` up to surprisingly large sizes.
- `reserve()` when you know the size, to avoid repeated reallocation and copying.
- Store objects by value in containers when you can, not as pointers.
- For hot loops over millions of items, consider "structure of arrays" (`std::vector<float> x, y, z`) instead of "array of structures": the loop then reads only the fields it needs.

## Concurrency

Use more cores when the work splits cleanly: `std::thread`, `std::jthread` (joins automatically), `std::async`, or parallel algorithms (`std::sort(std::execution::par, ...)`). Shared data needs a `std::mutex` (lock with `std::scoped_lock`) or `std::atomic`. Data races are undefined behaviour, and **ThreadSanitizer** (`-fsanitize=thread`) finds them.

## Sanitizers and warnings

Turn these on in your Debug builds; they find bugs that would take days to track down:

```text
g++ -std=c++20 -g -Wall -Wextra -fsanitize=address,undefined main.cpp    # memory errors, UB
cl /std:c++20 /W4 /fsanitize=address main.cpp                            # MSVC
clang-tidy main.cpp --checks='bugprone-*,performance-*,modernize-*'
```

@@quiz
Your program is slow. What should you do first?
- Rewrite the slowest-looking loop in assembly
- Switch every `std::vector` to an array
* Measure with a profiler (in a Release build) to find where the time actually goes
- Add more threads
? Guesses are usually wrong. Profile, fix the biggest cost, measure again.

@@quiz
Checking 1 million IPs against a 100,000-entry blocklist is slow with `std::find` on a vector. The best fix?
* Put the blocklist in a `std::unordered_set` (or a sorted vector with binary search)
- Compile with -O3
- Use `std::list` instead
- Check the IPs in reverse order
? O(n) per lookup becomes O(1) (or O(log n)): from 10¹¹ comparisons to about 10⁶ lookups.

@@quiz
Why is iterating a `std::vector` usually much faster than a `std::list` of the same items?
- vectors use less CPU instructions per element
* The elements are next to each other in memory, so CPU caches and prefetching work well
- lists are always sorted
- vectors are multi-threaded
? Pointer-chasing causes cache misses, and each miss costs as much as hundreds of instructions.

@@quiz
What does `-fsanitize=address` do?
- Makes the program faster
* Adds checks that report memory errors (out-of-bounds, use-after-free) when they happen
- Removes unused memory
- Hides addresses from attackers
? AddressSanitizer turns silent memory corruption into an immediate, precise error report.

@@flashcards
`std::chrono::steady_clock` :: a monotonic clock for measuring durations
Release vs Debug :: optimized (-O2, /O2) vs debuggable; benchmark only Release
profiler :: VS Performance Profiler, perf, VTune: where does time go?
`std::unordered_set` / `unordered_map` :: hash containers, O(1) average lookup
`reserve(n)` :: allocate once up front
`const T&` / `std::string_view` :: look at data without copying
`std::move` :: let the target take the source's resources
`emplace_back` :: construct the element directly in the container
cache line :: 64 bytes loaded together; sequential data is fast
`std::jthread` :: a thread that joins automatically
AddressSanitizer / UBSan / TSan :: find memory errors / undefined behaviour / data races

@@exercise Faster lookups
`blocked` is a list of blocked IPs and `traffic` is a list of connections. Count how many connections come from blocked IPs, by building a `std::unordered_set<std::string>` from `blocked` **once** and using it for every lookup. Print `blocked connections: 4`.
@@starter
#include <iostream>
#include <string>
#include <unordered_set>
#include <vector>

int main() {
    std::vector<std::string> blocked{"203.0.113.7", "198.51.100.23", "192.0.2.99"};
    std::vector<std::string> traffic{"10.0.0.5", "203.0.113.7", "10.0.0.9", "192.0.2.99",
                                     "203.0.113.7", "172.16.4.2", "198.51.100.23", "10.0.0.5"};
    int count = 0;
    // TODO: build the set once, then count
    std::cout << "blocked connections: " << count << '\n';
}
@@expected
blocked connections: 4
@@requires
std::unordered_set<std::string>
@@hint
`std::unordered_set<std::string> set(blocked.begin(), blocked.end());` then `for (const auto& ip : traffic) if (set.contains(ip)) ++count;`.
@@solution
#include <iostream>
#include <string>
#include <unordered_set>
#include <vector>

int main() {
    std::vector<std::string> blocked{"203.0.113.7", "198.51.100.23", "192.0.2.99"};
    std::vector<std::string> traffic{"10.0.0.5", "203.0.113.7", "10.0.0.9", "192.0.2.99",
                                     "203.0.113.7", "172.16.4.2", "198.51.100.23", "10.0.0.5"};
    int count = 0;
    std::unordered_set<std::string> set(blocked.begin(), blocked.end());
    for (const auto& ip : traffic)
        if (set.contains(ip)) ++count;
    std::cout << "blocked connections: " << count << '\n';
}

@@exercise Remove in one pass
Remove every log line that contains `DEBUG` from the vector **in one pass** with `std::erase_if` (C++20), then print how many lines are left and the remaining lines.
@@starter
#include <iostream>
#include <string>
#include <vector>

int main() {
    std::vector<std::string> lines{
        "INFO server started", "DEBUG cache warm", "WARN disk 85%", "DEBUG gc pause 3ms",
        "ERROR db timeout", "DEBUG retry 1"};
    // TODO: erase_if, then print
}
@@expected
3 lines left
INFO server started
WARN disk 85%
ERROR db timeout
@@requires
std::erase_if
@@hint
`std::erase_if(lines, [](const std::string& l) { return l.find("DEBUG") != std::string::npos; });` then print `lines.size()` and each line.
@@solution
#include <iostream>
#include <string>
#include <vector>

int main() {
    std::vector<std::string> lines{
        "INFO server started", "DEBUG cache warm", "WARN disk 85%", "DEBUG gc pause 3ms",
        "ERROR db timeout", "DEBUG retry 1"};
    std::erase_if(lines, [](const std::string& l) { return l.find("DEBUG") != std::string::npos; });
    std::cout << lines.size() << " lines left\n";
    for (const auto& l : lines) std::cout << l << '\n';
}
