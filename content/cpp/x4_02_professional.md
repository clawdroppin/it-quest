@@lesson x4-professional Professional C++: Builds, Threads and Safety
@@concepts concurrency, modules, testing
@@text
## Headers and source files

Real projects split code into **headers** (`.hpp`: declarations, templates, small inline functions) and **source files** (`.cpp`: definitions). Each `.cpp` compiles separately into an object file; the **linker** joins them.

```cpp,norun
// subnet.hpp
#pragma once                       // include this header only once per file
#include <cstdint>
#include <string>
std::uint32_t parseIpv4(const std::string& text);

// subnet.cpp
#include "subnet.hpp"
std::uint32_t parseIpv4(const std::string& text) { /* ... */ }

// main.cpp
#include "subnet.hpp"              // quotes: your files; <angle brackets>: system/library
```

"undefined reference to `parseIpv4`" (g++) or "unresolved external symbol" (MSVC, LNK2019) are **linker** errors: the function was declared but its `.cpp` wasn't compiled in.

## CMake

Almost every C++ project uses **CMake** to generate build files for any compiler or IDE:

```cmake
cmake_minimum_required(VERSION 3.20)
project(netkit CXX)
set(CMAKE_CXX_STANDARD 20)
add_executable(netkit main.cpp subnet.cpp)
target_compile_options(netkit PRIVATE $<IF:$<CXX_COMPILER_ID:MSVC>,/W4,-Wall -Wextra -Wpedantic>)
```

```bash,norun
cmake -B build -DCMAKE_BUILD_TYPE=Release
cmake --build build
./build/netkit
```

On Linux you'll often build tools from source this way (`cmake`, then `make` or `ninja`). Packages come from your distro (`apt install libssl-dev`), **vcpkg** or **Conan**.

## Debug vs Release, and sanitizers

- Debug (`-O0 -g`): slow, but the debugger (gdb, lldb, Visual Studio) can step through it.
- Release (`-O2`): fast. Undefined behaviour that "worked" in Debug may break here.
- **Sanitizers** add checks while you test: `-fsanitize=address` (memory errors), `-fsanitize=undefined` (UB), `-fsanitize=thread` (data races). Run your tests with them in CI.
- Static analysis: `clang-tidy`, MSVC `/analyze`, `cppcheck`.

## Threads

```cpp
#include <atomic>
#include <iostream>
#include <mutex>
#include <thread>
#include <vector>

int main() {
    std::atomic<int> checked{0};
    std::mutex printLock;
    std::vector<std::thread> workers;

    for (int id = 0; id < 4; ++id) {
        workers.emplace_back([id, &checked, &printLock] {
            for (int i = 0; i < 250; ++i) checked++;            // atomic: safe without a lock
            std::lock_guard<std::mutex> lock(printLock);        // RAII lock: unlocks at the end of scope
            std::cout << "worker " << id << " done\n";
        });
    }
    for (auto& t : workers) t.join();                           // wait for all of them
    std::cout << checked << " hosts checked\n";
}
```

- A **data race** (two threads touching the same data, at least one writing, without synchronization) is undefined behaviour. Protect shared data with a `std::mutex` + `std::lock_guard`, or use `std::atomic`.
- Every `std::thread` must be `join()`ed (or detached). `std::jthread` (C++20) joins automatically.
- `std::async` returns a `std::future` for "run this in the background and give me the result".

(The order of the "done" lines can change between runs. That's normal for threads.)

## Testing

C++ test frameworks: **GoogleTest** (most common), **Catch2**, **doctest**. A GoogleTest case:

```cpp,norun
TEST(Subnet, ParsesDottedQuad) {
    EXPECT_EQ(parseIpv4("192.168.1.10"), 3232235786u);
    EXPECT_THROW(parseIpv4("300.1.1.1"), std::invalid_argument);
}
```

The build project at the end of this track uses a tiny hand-written test runner, so it needs nothing installed.

## Safe C++ habits

1. Use `std::vector`, `std::string`, `std::array` and smart pointers, never raw arrays and `new`.
2. Initialize every variable; prefer `const`.
3. Compile with warnings as errors (`-Werror`, `/WX`) and sanitizers in testing.
4. Use `.at()` or check indexes from outside input; never trust lengths in network data.
5. Avoid C functions like `strcpy`, `sprintf`, `gets`: use `std::string` and streams.
6. Read the **C++ Core Guidelines** (isocpp.github.io), the community's best-practice rule book.

@@quiz
"unresolved external symbol" / "undefined reference" means...
* The linker can't find the function's definition: a `.cpp` file or library wasn't linked in
- A syntax error in a header
- The program crashed
- A missing `#include`
? Declarations satisfy the compiler; definitions are needed by the linker.

@@quiz
Two threads do `counter++` on a plain `int` with no lock. What is it?
* A data race: undefined behaviour, often a wrong total
- Fine, `++` is a single instruction
- A compile error
- Automatically made safe by the compiler
? Use `std::atomic<int>` or a mutex.

@@quiz
What does `-fsanitize=address` do?
* Adds run-time checks that stop the program at memory errors like overflows and use-after-free
- Removes unused memory
- Encrypts memory
- Makes the program faster
? AddressSanitizer is one of the best tools for finding memory bugs. Use it in tests.

@@drill
Which `std::mutex` helper locks in its constructor and unlocks in its destructor?
= std::lock_guard
= lock_guard
= std::scoped_lock
= scoped_lock
? RAII for locks.

@@flashcards
`#pragma once` :: include this header at most once
linker error :: a declared function has no definition linked in
`cmake -B build && cmake --build build` :: configure and build a CMake project
`std::lock_guard` :: RAII mutex lock
`std::atomic<int>` :: an int that's safe to change from many threads
`t.join()` :: wait for a thread to finish
`-fsanitize=thread` :: detect data races
C++ Core Guidelines :: the community's best-practice rules

@@exercise Parallel sum
Split the vector into 4 parts and sum each part in its own `std::thread`, storing each partial sum in `partial[i]` (each thread writes only its own slot, so no lock is needed). Join them and print the total.
@@starter
#include <iostream>
#include <numeric>
#include <thread>
#include <vector>

int main() {
    std::vector<long long> bytes(1000);
    std::iota(bytes.begin(), bytes.end(), 1);   // 1, 2, ..., 1000
    std::vector<long long> partial(4, 0);
    std::vector<std::thread> threads;
    std::size_t chunk = bytes.size() / 4;

    // start 4 threads, join them, print the total
}
@@expected
total: 500500
@@requires
std::thread
join
.emplace_back(
@@hint
`threads.emplace_back([&, i] { partial[i] = std::accumulate(bytes.begin() + i * chunk, bytes.begin() + (i + 1) * chunk, 0LL); });` in a loop over `i`, then join, then sum `partial`.
@@solution
#include <iostream>
#include <numeric>
#include <thread>
#include <vector>

int main() {
    std::vector<long long> bytes(1000);
    std::iota(bytes.begin(), bytes.end(), 1);
    std::vector<long long> partial(4, 0);
    std::vector<std::thread> threads;
    std::size_t chunk = bytes.size() / 4;

    for (std::size_t i = 0; i < 4; ++i) {
        threads.emplace_back([&, i] {
            partial[i] = std::accumulate(bytes.begin() + i * chunk, bytes.begin() + (i + 1) * chunk, 0LL);
        });
    }
    for (auto& t : threads) t.join();
    std::cout << "total: " << std::accumulate(partial.begin(), partial.end(), 0LL) << '\n';
}
