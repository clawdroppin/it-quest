@@lesson e6-ranges Ranges and Coroutines (C++20)
@@concepts lists, functions
@@text
Two C++20 features change how you write data-processing code: **ranges** (composable, lazy algorithms) and **coroutines** (functions that can pause).

## Ranges and views

Range algorithms take the whole container instead of a `begin`/`end` pair, and accept **projections**:

```cpp,norun
std::ranges::sort(servers, {}, &Server::cpu);                  // sort by a member
auto it = std::ranges::find(servers, "db01", &Server::name);   // find by a member
auto n = std::ranges::count(log, 500, &Entry::status);
```

**Views** are lazy adaptors combined with `|`. Nothing runs until you iterate, and no intermediate vectors are created:

```cpp
#include <iostream>
#include <ranges>
#include <vector>

int main() {
    std::vector<int> ports = {22, 80, 443, 8080, 8443, 3389};
    auto high = ports | std::views::filter([](int p) { return p > 1024; })
                      | std::views::transform([](int p) { return p + 10000; })
                      | std::views::take(2);
    for (int p : high) std::cout << p << ' ';      // 18080 18443
    std::cout << '\n';
    for (int i : std::views::iota(1, 4)) std::cout << i;   // 123
    std::cout << '\n';
}
```

Other useful views: `reverse`, `drop`, `take_while`, `keys`/`values` (on maps), `split`, `join`, and in C++23 `enumerate`, `zip`, `chunk` and `std::ranges::to<std::vector>()`.

Views hold references to their source: don't return a view over a local container.

## Coroutines

A function containing `co_yield`, `co_await` or `co_return` is a **coroutine**: it can suspend and later resume where it left off, with its local variables kept in a heap-allocated frame. C++20 provides the machinery, not ready-made types: the return type's `promise_type` decides the behaviour. C++23 adds `std::generator<T>`; libraries like cppcoro and Asio provide tasks for async I/O.

```cpp,norun
Generator<int> countdown(int n) {
    while (n > 0) co_yield n--;     // pause here, hand n to the caller
}
for (int x : countdown(3)) ...      // 3 2 1
```

Generators turn "callback-style" producers (a tokenizer, a file walker, a paginated API client) into something you can loop over. Async coroutines (`co_await socket.read()`) make asynchronous network code read like sequential code.

@@quiz
When does a view pipeline like `v | filter(...) | transform(...)` do its work?
- Immediately, creating new vectors
* Lazily, element by element, when you iterate it
- At compile time
- Only when you call .run()
? Views are lazy.

@@quiz
What does a projection like `&Server::cpu` do in `std::ranges::sort(servers, {}, &Server::cpu)`?
* Tells the algorithm to compare servers by their cpu member
- Sorts only the cpu values
- Creates a new vector
- Filters servers
? Projections replace hand-written comparator lambdas.

@@quiz
What makes a function a coroutine?
- The `async` keyword
* Using co_yield, co_await or co_return in its body
- Returning std::future
- Running on another thread
? Its return type's promise_type controls it.

@@drill
Which keyword suspends a coroutine and hands a value to the caller?
= co_yield
? co_await waits; co_return finishes.

@@flashcards
`std::ranges::sort(r, {}, proj)` :: sort by projection
`std::views::filter` :: lazy filter
`std::views::transform` :: lazy map
`std::views::take(n)` :: first n elements
`std::views::iota(a, b)` :: a, a+1, ..., b-1
Projection :: member or function applied before comparing
`co_yield` :: produce a value and suspend
`co_await` :: suspend until something is ready
`promise_type` :: controls a coroutine's behaviour
`std::coroutine_handle` :: resume/destroy a coroutine

@@exercise Log analysis with ranges
Answer questions about an access log using range algorithms and views, without writing raw loops for the filtering:

- the first 3 hosts that returned a 5xx status (`filter` → `transform` → `take`)
- the 3 slowest requests: copy, `std::ranges::sort` with `std::ranges::greater{}` and the projection `&Entry::ms`, then `take(3)`
- the number of 200 responses with `std::ranges::count` and a projection
- the average time of hosts starting with `web`
- the squares of 1..5 with `iota` and `transform`
@@starter
#include <algorithm>
#include <format>
#include <iostream>
#include <ranges>
#include <string>
#include <vector>

struct Entry {
    std::string host;
    int status;
    int ms;
};

int main() {
    std::vector<Entry> log = {{"web01", 200, 120}, {"db01", 500, 2300}, {"web02", 503, 1200}, {"web01", 200, 340},
                              {"db01", 502, 90},   {"cache01", 200, 15}, {"web02", 404, 30},  {"api01", 500, 60}};
    std::cout << "TODO\n";
}
@@expected
5xx hosts (first 3): db01 web02 db01
slowest: db01 2300ms, web02 1200ms, web01 340ms
status 200: 3
web average: 422.5ms
squares: 1 4 9 16 25
@@requires
std::views::filter
std::views::take
&Entry::ms
@@hint
`for (const auto& h : log | std::views::filter([](const Entry& e) { return e.status >= 500; }) | std::views::transform(&Entry::host) | std::views::take(3))`. For the average, filter with `e.host.starts_with("web")` and sum in a range-for. Format the average with `std::format("{}", avg)`.
@@solution
#include <algorithm>
#include <format>
#include <iostream>
#include <ranges>
#include <string>
#include <vector>

struct Entry {
    std::string host;
    int status;
    int ms;
};

int main() {
    std::vector<Entry> log = {{"web01", 200, 120}, {"db01", 500, 2300}, {"web02", 503, 1200}, {"web01", 200, 340},
                              {"db01", 502, 90},   {"cache01", 200, 15}, {"web02", 404, 30},  {"api01", 500, 60}};

    std::cout << "5xx hosts (first 3):";
    auto errors = log | std::views::filter([](const Entry& e) { return e.status >= 500; })
                      | std::views::transform(&Entry::host)
                      | std::views::take(3);
    for (const auto& h : errors) std::cout << ' ' << h;

    auto by_time = log;
    std::ranges::sort(by_time, std::ranges::greater{}, &Entry::ms);
    std::cout << "\nslowest: ";
    bool first = true;
    for (const auto& e : by_time | std::views::take(3)) {
        std::cout << (first ? "" : ", ") << e.host << ' ' << e.ms << "ms";
        first = false;
    }

    std::cout << "\nstatus 200: " << std::ranges::count(log, 200, &Entry::status) << '\n';

    double total = 0;
    int count = 0;
    for (const auto& e : log | std::views::filter([](const Entry& e) { return e.host.starts_with("web"); })) {
        total += e.ms;
        ++count;
    }
    std::cout << std::format("web average: {}ms\n", total / count);

    std::cout << "squares:";
    for (int sq : std::views::iota(1, 6) | std::views::transform([](int x) { return x * x; })) std::cout << ' ' << sq;
    std::cout << '\n';
}

@@exercise A generator coroutine
Write a minimal `Generator<T>` coroutine type and use it:

- `promise_type` stores the current value; `initial_suspend` and `final_suspend` return `std::suspend_always`; `yield_value` stores the value and suspends; `return_void`; `unhandled_exception` rethrows
- the generator owns its `std::coroutine_handle` (destroy it in the destructor, move-only)
- support range-for: `begin()` resumes once and returns an iterator; the iterator's `++` resumes; it equals `std::default_sentinel` when the coroutine is done

Write `fibonacci()` (infinite) and `words(std::string_view)` (splits on spaces) as generators.
@@starter
#include <coroutine>
#include <exception>
#include <iostream>
#include <iterator>
#include <string_view>
#include <utility>

int main() {
    std::cout << "TODO\n";
}
@@expected
fib: 0 1 1 2 3 5 8 13 21 34
first over 1000: 1597
words: [deploy] [web01] [now]
@@requires
co_yield
promise_type
std::coroutine_handle
@@hint
`struct iterator { std::coroutine_handle<promise_type> h; iterator& operator++() { h.resume(); return *this; } const T& operator*() const { return h.promise().current; } bool operator==(std::default_sentinel_t) const { return h.done(); } };`. An infinite generator is fine as long as you `break` out of the loop; the destructor cleans up the suspended frame.
@@solution
#include <coroutine>
#include <exception>
#include <iostream>
#include <iterator>
#include <string_view>
#include <utility>

template <class T>
class Generator {
public:
    struct promise_type {
        T current{};
        Generator get_return_object() { return Generator(std::coroutine_handle<promise_type>::from_promise(*this)); }
        std::suspend_always initial_suspend() noexcept { return {}; }
        std::suspend_always final_suspend() noexcept { return {}; }
        std::suspend_always yield_value(T value) {
            current = std::move(value);
            return {};
        }
        void return_void() {}
        void unhandled_exception() { throw; }
    };

    struct iterator {
        std::coroutine_handle<promise_type> h;
        iterator& operator++() {
            h.resume();
            return *this;
        }
        const T& operator*() const { return h.promise().current; }
        bool operator==(std::default_sentinel_t) const { return h.done(); }
    };

    explicit Generator(std::coroutine_handle<promise_type> h) : handle_(h) {}
    Generator(Generator&& other) noexcept : handle_(std::exchange(other.handle_, {})) {}
    Generator(const Generator&) = delete;
    ~Generator() {
        if (handle_) handle_.destroy();
    }

    iterator begin() {
        handle_.resume();
        return iterator{handle_};
    }
    std::default_sentinel_t end() { return {}; }

private:
    std::coroutine_handle<promise_type> handle_;
};

Generator<long long> fibonacci() {
    long long a = 0, b = 1;
    while (true) {
        co_yield a;
        a = std::exchange(b, a + b);
    }
}

Generator<std::string_view> words(std::string_view text) {
    size_t i = 0;
    while (i < text.size()) {
        while (i < text.size() && text[i] == ' ') ++i;
        size_t start = i;
        while (i < text.size() && text[i] != ' ') ++i;
        if (i > start) co_yield text.substr(start, i - start);
    }
}

int main() {
    std::cout << "fib:";
    int n = 0;
    for (long long x : fibonacci()) {
        if (n++ == 10) break;
        std::cout << ' ' << x;
    }
    for (long long x : fibonacci()) {
        if (x > 1000) {
            std::cout << "\nfirst over 1000: " << x << '\n';
            break;
        }
    }
    std::cout << "words:";
    for (auto w : words("  deploy web01   now ")) std::cout << " [" << w << ']';
    std::cout << '\n';
}
