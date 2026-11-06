@@lesson e6-errors Error Handling in Depth: Exception Safety and Expected Values
@@concepts errors
@@text
C++ has two error-handling styles, and real codebases use both: **exceptions** for failures the caller usually can't handle locally, and **error values** (`std::optional`, error codes, `std::expected`) for failures that are part of normal operation, like parsing user input.

## Exception safety guarantees

When an exception passes through your function, what state are objects left in? Every function gives one of these guarantees:

| Guarantee | Promise |
|---|---|
| **nothrow** | never throws (`noexcept`): destructors, swaps, moves |
| **strong** | if it throws, nothing changed: commit or roll back |
| **basic** | if it throws, objects are valid and nothing leaks, but values may have changed |
| none | anything can happen: a bug |

RAII gives you the basic guarantee almost for free. The classic way to get the strong one is **copy and swap**: do all the work on a copy, then `swap` it in with an operation that can't throw.

```cpp,norun
void Config::apply(const Patch& p) {
    Config copy = *this;     // may throw: we're unchanged
    copy.merge(p);           // may throw: we're unchanged
    swap(copy);              // noexcept: commit
}
```

## Exceptions done right

- Throw by value, catch by `const&`: `catch (const std::exception& e)`.
- Derive your own types from `std::runtime_error` (or `std::logic_error` for bugs).
- Catch the most specific types first.
- Destructors must never throw (they're `noexcept` by default).
- A `noexcept` function that throws calls `std::terminate`.
- Many codebases (games, embedded, Google style) compile with exceptions **off**: there, error values are the only option.

## Error values

```cpp,norun
std::optional<int> find_port(std::string_view name);          // "maybe a value", no reason given

std::error_code ec;
std::filesystem::remove("old.log", ec);                         // OS-style error codes
if (ec) std::cerr << ec.message() << '\n';

// C++23
std::expected<Endpoint, std::string> parse_endpoint(std::string_view s);
auto r = parse_endpoint(text).and_then(check_policy).transform(to_url);
if (!r) std::cerr << r.error();
```

`std::expected<T, E>` holds either a value or an error and chains with `and_then` (next step may fail), `transform` (next step can't fail) and `or_else`. In C++20 you can write a small version yourself, which is what this lesson's second exercise does.

@@quiz
Which guarantee does copy-and-swap provide?
- nothrow
* strong: if anything throws, the original object is unchanged
- basic only
- none
? The swap at the end must be noexcept.

@@quiz
How should you catch exceptions?
- By value: `catch (std::exception e)`
* By const reference: `catch (const std::exception& e)`
- By pointer
- Only with catch (...)
? Catching by value slices derived exceptions.

@@quiz
What happens when a `noexcept` function throws?
- The exception propagates normally
* std::terminate is called
- It's silently ignored
- It becomes an error code
? noexcept is a promise the compiler relies on.

@@drill
Which exception-safety guarantee says an operation either succeeds or leaves the state unchanged?
= strong
= strong guarantee
= the strong guarantee
? The basic guarantee only promises no leaks and valid state.

@@flashcards
nothrow guarantee :: never throws
Strong guarantee :: commit or roll back
Basic guarantee :: valid state, no leaks
Copy and swap :: work on a copy, swap in
`catch (const std::exception& e)` :: catch by const reference
`std::runtime_error` :: base for runtime failures
`std::error_code` :: OS-style error value
`std::expected<T, E>` :: value or error (C++23)
`and_then` :: chain a step that may fail
`transform` :: map the value if present

@@exercise Inventory updates with the strong guarantee
Write `Inventory::apply(const std::vector<Change>& batch)` that applies all changes or none:

- each `Change { item, delta }` must name a known item (else throw `std::invalid_argument("unknown item <item>")`) and must not take stock below 0 (else throw `std::out_of_range("not enough <item>")`)
- work on a copy of the map and `swap` it in at the end
- for comparison, write `apply_in_place` that edits the map directly (only the basic guarantee) and show the partial update it leaves behind
@@starter
#include <iostream>
#include <map>
#include <stdexcept>
#include <string>
#include <vector>

struct Change {
    std::string item;
    int delta;
};

int main() {
    std::cout << "TODO\n";
}
@@expected
start: dock=2 laptop=5 monitor=3
batch 1 ok: dock=1 laptop=3 monitor=3
batch 2 failed (not enough laptop): dock=1 laptop=3 monitor=3
batch 3 failed (unknown item phone): dock=1 laptop=3 monitor=3
in-place version after batch 2: dock=1 laptop=3 monitor=2
@@requires
swap
catch (const std::exception& e)
@@hint
Put the checking-and-applying loop in a static helper `apply_to(std::map<std::string, int>& stock, const std::vector<Change>& batch)`. `apply` calls it on a copy and then `stock_.swap(copy)`; `apply_in_place` calls it on `stock_` directly. Batch 2 is `{monitor, -1}, {laptop, -10}`.
@@solution
#include <iostream>
#include <map>
#include <stdexcept>
#include <string>
#include <vector>

struct Change {
    std::string item;
    int delta;
};

class Inventory {
public:
    explicit Inventory(std::map<std::string, int> stock) : stock_(std::move(stock)) {}

    void apply(const std::vector<Change>& batch) {
        auto copy = stock_;
        apply_to(copy, batch);
        stock_.swap(copy);
    }

    void apply_in_place(const std::vector<Change>& batch) { apply_to(stock_, batch); }

    std::string str() const {
        std::string out;
        for (const auto& [item, n] : stock_) out += (out.empty() ? "" : " ") + item + '=' + std::to_string(n);
        return out;
    }

private:
    static void apply_to(std::map<std::string, int>& stock, const std::vector<Change>& batch) {
        for (const auto& c : batch) {
            auto it = stock.find(c.item);
            if (it == stock.end()) throw std::invalid_argument("unknown item " + c.item);
            if (it->second + c.delta < 0) throw std::out_of_range("not enough " + c.item);
            it->second += c.delta;
        }
    }

    std::map<std::string, int> stock_;
};

int main() {
    Inventory inv({{"laptop", 5}, {"monitor", 3}, {"dock", 2}});
    std::cout << "start: " << inv.str() << '\n';
    std::vector<std::vector<Change>> batches = {
        {{"laptop", -2}, {"dock", -1}},
        {{"monitor", -1}, {"laptop", -10}},
        {{"phone", 1}},
    };
    for (size_t i = 0; i < batches.size(); ++i) {
        try {
            inv.apply(batches[i]);
            std::cout << "batch " << i + 1 << " ok: " << inv.str() << '\n';
        } catch (const std::exception& e) {
            std::cout << "batch " << i + 1 << " failed (" << e.what() << "): " << inv.str() << '\n';
        }
    }
    try {
        inv.apply_in_place(batches[1]);
    } catch (const std::exception&) {
        std::cout << "in-place version after batch 2: " << inv.str() << '\n';
    }
}

@@exercise A small expected type
C++23 has `std::expected`; write a small version for C++20 and use it in a parsing pipeline:

- `struct Error { std::string message; };` and `template <class T> class Result` holding a `std::variant<T, Error>`, with `ok`, `fail`, `has_value`, `value`, `error`, `value_or`, `and_then(f)` (f returns a `Result`) and `transform(f)` (f returns a plain value)
- `split(s)` → host and port text (`missing ':'`, `empty host`); `parse_port` → int (`port is not a number`, `port N out of range` above 65535); `check_allowed` → only 22, 443 and 8443 (`port N not allowed by policy`)
- `parse_endpoint` chains them and produces an `Endpoint`
@@starter
#include <charconv>
#include <functional>
#include <iostream>
#include <string>
#include <string_view>
#include <type_traits>
#include <utility>
#include <variant>

struct Endpoint {
    std::string host;
    int port;
};

int main() {
    std::cout << "TODO\n";
}
@@expected
web01:8443 -> ok web01 port 8443
web01 -> error: missing ':'
db01:abc -> error: port is not a number
db01:70000 -> error: port 70000 out of range
db01:3389 -> error: port 3389 not allowed by policy
:443 -> error: empty host
fallback port for db01:abc: 443
@@requires
and_then
transform
std::variant<T, Error>
@@hint
`and_then`: `using R = std::invoke_result_t<F, const T&>; if (has_value()) return std::invoke(f, value()); return R::fail(error().message);`. `transform`: `using U = std::invoke_result_t<F, const T&>;` and wrap the result in `Result<U>::ok(...)`. In `parse_endpoint`, call `split(s).and_then([](const auto& parts) { return parse_port(parts.second).and_then(check_allowed).transform([&](int port) { return Endpoint{parts.first, port}; }); })`.
@@solution
#include <charconv>
#include <functional>
#include <iostream>
#include <string>
#include <string_view>
#include <type_traits>
#include <utility>
#include <variant>

struct Endpoint {
    std::string host;
    int port;
};

struct Error {
    std::string message;
};

template <class T>
class Result {
public:
    static Result ok(T value) { return Result(std::move(value)); }
    static Result fail(std::string message) { return Result(Error{std::move(message)}); }

    bool has_value() const { return std::holds_alternative<T>(v_); }
    const T& value() const { return std::get<T>(v_); }
    const Error& error() const { return std::get<Error>(v_); }
    T value_or(T fallback) const { return has_value() ? value() : fallback; }

    template <class F>
    auto and_then(F&& f) const {
        using R = std::invoke_result_t<F, const T&>;
        if (has_value()) return std::invoke(f, value());
        return R::fail(error().message);
    }

    template <class F>
    auto transform(F&& f) const {
        using U = std::invoke_result_t<F, const T&>;
        if (has_value()) return Result<U>::ok(std::invoke(f, value()));
        return Result<U>::fail(error().message);
    }

private:
    explicit Result(T value) : v_(std::move(value)) {}
    explicit Result(Error e) : v_(std::move(e)) {}
    std::variant<T, Error> v_;
};

using Parts = std::pair<std::string, std::string>;

Result<Parts> split(std::string_view s) {
    auto colon = s.find(':');
    if (colon == std::string_view::npos) return Result<Parts>::fail("missing ':'");
    if (colon == 0) return Result<Parts>::fail("empty host");
    return Result<Parts>::ok({std::string(s.substr(0, colon)), std::string(s.substr(colon + 1))});
}

Result<int> parse_port(std::string_view text) {
    int port = 0;
    auto [p, ec] = std::from_chars(text.data(), text.data() + text.size(), port);
    if (ec != std::errc{} || p != text.data() + text.size()) return Result<int>::fail("port is not a number");
    if (port < 1 || port > 65535) return Result<int>::fail("port " + std::to_string(port) + " out of range");
    return Result<int>::ok(port);
}

Result<int> check_allowed(int port) {
    if (port == 22 || port == 443 || port == 8443) return Result<int>::ok(port);
    return Result<int>::fail("port " + std::to_string(port) + " not allowed by policy");
}

Result<Endpoint> parse_endpoint(std::string_view s) {
    return split(s).and_then([](const Parts& parts) {
        return parse_port(parts.second).and_then(check_allowed).transform([&](int port) {
            return Endpoint{parts.first, port};
        });
    });
}

int main() {
    for (std::string_view s : {"web01:8443", "web01", "db01:abc", "db01:70000", "db01:3389", ":443"}) {
        auto r = parse_endpoint(s);
        std::cout << s << " -> ";
        if (r.has_value()) std::cout << "ok " << r.value().host << " port " << r.value().port << '\n';
        else std::cout << "error: " << r.error().message << '\n';
    }
    std::cout << "fallback port for db01:abc: " << parse_port("abc").value_or(443) << '\n';
}
