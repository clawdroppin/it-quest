@@lesson n7-safety Undefined Behaviour, Sanitizers and Safer C++
@@concepts errors, testing
@@text
**Undefined behaviour (UB)** means the C++ standard places no requirements on what happens. The program might work in debug and crash in release, or work on your PC and leak data on the server. Optimisers assume UB never happens and transform code accordingly, so "it worked when I tried it" proves nothing.

## The usual suspects

| UB | Example | Safer alternative |
|---|---|---|
| out-of-bounds access | `v[v.size()]`, `for (i = 0; i <= n; ...)` | range-for, `.at()`, `std::span` |
| use after free / dangling | returning `string_view` to a temporary | owning types, clear lifetimes |
| iterator invalidation | `erase` inside a range-for | `std::erase_if`, erase-returns-iterator |
| signed overflow | `int total = 2'000'000'000 * 2;` | wider types, checked arithmetic |
| uninitialised reads | `int max; if (x > max)` | always initialise |
| data races | two threads `++counter` | atomics, mutexes |
| null dereference | `*find(...)` without checking | `std::optional`, check pointers |
| invalid casts | `reinterpret_cast` type punning | `std::bit_cast`, `memcpy` |

## Tools that find it

- **Warnings**: `-Wall -Wextra -Wconversion` / `/W4`, treated as errors in CI.
- **Sanitizers** add runtime checks: AddressSanitizer (`-fsanitize=address`, also in MSVC) for out-of-bounds and use-after-free; UndefinedBehaviorSanitizer (`-fsanitize=undefined`) for overflow and bad shifts; ThreadSanitizer for races; MemorySanitizer for uninitialised reads. Run your test suite under them.
- **Debug iterators**: `_GLIBCXX_DEBUG` / `_GLIBCXX_ASSERTIONS` and MSVC's `_ITERATOR_DEBUG_LEVEL` check bounds and detect invalidated iterators in debug builds. libc++ and libstdc++ offer "hardened" modes for production.
- **Static analysis**: clang-tidy (with `cppcoreguidelines-*`, `bugprone-*`), the MSVC analyzer (`/analyze`), cppcheck, PVS-Studio, CodeQL.

## Memory safety and the future

Governments and large companies now push memory-safe languages for new code, because most serious security bugs in C and C++ are memory-safety bugs. C++'s answer is safety profiles, hardened standard libraries, and the **C++ Core Guidelines** (with the GSL library): owning types, spans instead of pointer+length, no naked `new`/`delete`, `gsl::narrow` for conversions. Rust interop (the next lessons and the Rust track) is the other common answer.

@@quiz
Why is "it works on my machine" meaningless for code with UB?
* The compiler may assume UB never happens and produce code that behaves differently with other flags, compilers or inputs
- UB only happens on servers
- UB is always caught by the compiler
- It isn't; UB is deterministic
? Optimisers exploit the assumption.

@@quiz
Which tool catches a heap buffer overflow at runtime?
- clang-format
* AddressSanitizer
- ThreadSanitizer
- CMake
? Compile with -fsanitize=address (or /fsanitize=address).

@@quiz
Which of these is undefined behaviour in C++?
* Signed integer overflow
- Unsigned integer wrap-around
- Dividing doubles by zero to get infinity
- Comparing two pointers to the same array
? Unsigned arithmetic wraps by definition; signed overflow lets the compiler assume it never happens.

@@flashcards
UB :: no requirements on behaviour
AddressSanitizer :: out-of-bounds, use-after-free
UBSan :: overflow, bad shifts, misaligned access
ThreadSanitizer :: data races
MemorySanitizer :: uninitialised reads
`_GLIBCXX_DEBUG` / `_ITERATOR_DEBUG_LEVEL` :: checked iterators
clang-tidy :: static analysis and modernisation
C++ Core Guidelines :: safety-oriented style rules
`std::bit_cast` :: safe type punning (C++20)

@@exercise Fix five bugs
The starter compiles but has five instances of undefined behaviour. Fix each one so the program prints the expected output:

1. an off-by-one loop
2. erasing from a vector inside a range-for
3. a `std::string_view` to a destroyed temporary
4. signed overflow when adding byte counts
5. reading an uninitialised variable
@@starter
#include <algorithm>
#include <cstdint>
#include <iostream>
#include <string>
#include <string_view>
#include <vector>

std::string host_name() { return "web01.example.jp"; }

int main() {
    std::vector<int> ports = {22, 80, 443, 8080};
    std::cout << "ports:";
    for (size_t i = 0; i <= ports.size(); ++i) std::cout << ' ' << ports[i];
    std::cout << '\n';

    std::vector<int> unprivileged = ports;
    for (int p : unprivileged)
        if (p < 1024) unprivileged.erase(std::find(unprivileged.begin(), unprivileged.end(), p));
    std::cout << "unprivileged:";
    for (int p : unprivileged) std::cout << ' ' << p;
    std::cout << '\n';

    std::string_view host = host_name();
    std::cout << "host: " << host.substr(0, host.find('.')) << '\n';

    int sent = 2'000'000'000, received = 1'500'000'000;
    int total = sent + received;
    std::cout << "total bytes: " << total << '\n';

    std::vector<int> loads = {35, 72, 18};
    int peak;
    for (int l : loads)
        if (l > peak) peak = l;
    std::cout << "peak load: " << peak << '\n';
}
@@expected
ports: 22 80 443 8080
unprivileged: 8080
host: web01
total bytes: 3500000000
peak load: 72
@@requires
std::erase_if
int64_t
@@hint
1: `<` instead of `<=` (or a range-for). 2: `std::erase_if(unprivileged, [](int p) { return p < 1024; });`. 3: store the result in a `std::string`. 4: `int64_t total = int64_t(sent) + received;`. 5: `int peak = loads.front();`.
@@solution
#include <cstdint>
#include <iostream>
#include <string>
#include <string_view>
#include <vector>

std::string host_name() { return "web01.example.jp"; }

int main() {
    std::vector<int> ports = {22, 80, 443, 8080};
    std::cout << "ports:";
    for (int p : ports) std::cout << ' ' << p;
    std::cout << '\n';

    std::vector<int> unprivileged = ports;
    std::erase_if(unprivileged, [](int p) { return p < 1024; });
    std::cout << "unprivileged:";
    for (int p : unprivileged) std::cout << ' ' << p;
    std::cout << '\n';

    std::string host = host_name();
    std::cout << "host: " << host.substr(0, host.find('.')) << '\n';

    int sent = 2'000'000'000, received = 1'500'000'000;
    int64_t total = int64_t(sent) + received;
    std::cout << "total bytes: " << total << '\n';

    std::vector<int> loads = {35, 72, 18};
    int peak = loads.front();
    for (int l : loads)
        if (l > peak) peak = l;
    std::cout << "peak load: " << peak << '\n';
}

@@exercise A checked vector with stale-handle detection
Debug standard libraries detect invalidated iterators with a generation counter. Build that idea yourself:

- `CheckedVector<T>` wraps a `std::vector<T>` and a `uint64_t generation`
- `at(i)` throws `std::out_of_range("index <i> out of range (size <n>)")`
- `Handle handle(i)` remembers the index and the current generation; `get(handle)` throws `std::logic_error("stale handle (container modified)")` if the generation changed
- `push_back` and `erase(i)` increment the generation (conservatively: any of them may move elements)
@@starter
#include <cstdint>
#include <iostream>
#include <stdexcept>
#include <string>
#include <vector>

int main() {
    std::cout << "TODO\n";
}
@@expected
at(1) = 80
at(5): index 5 out of range (size 3)
handle before push_back: 443
after push_back: stale handle (container modified)
fresh handle: 443
after erase(0): stale handle (container modified)
@@requires
std::out_of_range
generation
@@hint
`struct Handle { size_t index; uint64_t generation; };`. A small helper `void attempt(const char* label, auto fn)` that prints `label: ` and either the value or `e.what()` keeps the main function short.
@@solution
#include <cstdint>
#include <iostream>
#include <stdexcept>
#include <string>
#include <vector>

template <class T>
class CheckedVector {
public:
    struct Handle {
        size_t index;
        uint64_t generation;
    };

    CheckedVector(std::initializer_list<T> items) : data_(items) {}

    T& at(size_t i) {
        if (i >= data_.size())
            throw std::out_of_range("index " + std::to_string(i) + " out of range (size " + std::to_string(data_.size()) + ")");
        return data_[i];
    }

    void push_back(T value) {
        ++generation_;
        data_.push_back(std::move(value));
    }

    void erase(size_t i) {
        at(i);
        ++generation_;
        data_.erase(data_.begin() + static_cast<std::ptrdiff_t>(i));
    }

    Handle handle(size_t i) {
        at(i);
        return {i, generation_};
    }

    T& get(Handle h) {
        if (h.generation != generation_) throw std::logic_error("stale handle (container modified)");
        return at(h.index);
    }

private:
    std::vector<T> data_;
    uint64_t generation_ = 0;
};

template <class F>
void attempt(const char* label, F fn) {
    std::cout << label << ": ";
    try {
        std::cout << fn() << '\n';
    } catch (const std::exception& e) {
        std::cout << e.what() << '\n';
    }
}

int main() {
    CheckedVector<int> ports = {22, 80, 443};
    std::cout << "at(1) = " << ports.at(1) << '\n';
    attempt("at(5)", [&] { return ports.at(5); });

    auto h = ports.handle(2);
    attempt("handle before push_back", [&] { return ports.get(h); });
    ports.push_back(8080);
    attempt("after push_back", [&] { return ports.get(h); });

    h = ports.handle(2);
    attempt("fresh handle", [&] { return ports.get(h); });
    ports.erase(0);
    attempt("after erase(0)", [&] { return ports.get(h); });
}
