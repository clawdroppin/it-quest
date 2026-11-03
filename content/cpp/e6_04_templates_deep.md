@@lesson e6-templates Templates in Depth: Variadics, if constexpr, Concepts and constexpr
@@concepts functions
@@text
Templates are C++'s compile-time programming system. Modern C++ makes them far more readable than their reputation suggests.

## Variadic templates and fold expressions

```cpp
#include <iostream>

template <class... Ts>
auto sum(Ts... values) {
    return (values + ... + 0);            // fold expression: v1 + (v2 + (v3 + 0))
}

template <class... Ts>
void print_all(const Ts&... values) {
    std::cout << sizeof...(values) << ": ";   // number of arguments
    ((std::cout << values << ' '), ...);      // comma fold: run once per argument
    std::cout << '\n';
}

int main() {
    std::cout << sum(1, 2, 3, 4) << '\n';  // 10
    print_all("web01", 443, 2.5);          // 3: web01 443 2.5
}
```

## if constexpr

`if constexpr` discards the branch that doesn't apply at compile time, so each branch only has to compile for the types that reach it:

```cpp,norun
template <class T>
std::string describe(const T& x) {
    if constexpr (std::is_same_v<T, bool>) return x ? "true" : "false";
    else if constexpr (std::is_arithmetic_v<T>) return std::to_string(x);
    else return std::string(x);
}
```

## Concepts: constraints with readable errors

```cpp,norun
template <class T>
concept Range = requires(const T& t) { t.begin(); t.end(); };

template <class T>
concept Numeric = std::integral<T> || std::floating_point<T>;

auto average(const Range auto& r) requires Numeric<std::ranges::range_value_t<decltype(r)>>;
```

Before C++20 the same thing needed SFINAE tricks like `std::enable_if_t`, which you'll still meet in older code.

## constexpr and consteval

`constexpr` functions can run at compile time when their inputs are known, so you can precompute lookup tables, parse constants and check invariants with `static_assert` at build time. `consteval` functions *must* run at compile time. C++20 allows `std::vector` and `std::string` inside constexpr functions (as long as they don't escape), and `std::array` works everywhere.

```cpp,norun
constexpr int square(int x) { return x * x; }
static_assert(square(12) == 144);       // checked by the compiler, costs nothing at runtime
```

@@quiz
What does `(values + ... + 0)` do?
* Adds all the arguments in a pack, starting from 0
- Creates an array
- Repeats the first value
- It's invalid syntax
? A binary right fold.

@@quiz
Why use `if constexpr` instead of `if` in a template?
- It's faster at runtime
* The discarded branch isn't compiled for that type, so it may contain code that wouldn't compile
- It allows runtime polymorphism
- It's required in all templates
? E.g. calling std::to_string only for numbers.

@@quiz
What does `static_assert(crc32("123456789") == 0xCBF43926)` cost at runtime?
- One CRC computation
* Nothing: it's evaluated by the compiler
- A table lookup
- A function call
? A failing static_assert stops the build.

@@flashcards
`typename... Ts` :: template parameter pack
`sizeof...(args)` :: number of pack elements
Fold expression :: `(args op ...)`
`if constexpr` :: compile-time branch
`concept` :: named requirement on types
`requires` :: constraint or requirement expression
`constexpr` :: may run at compile time
`consteval` :: must run at compile time
`static_assert` :: compile-time check
SFINAE :: pre-C++20 overload constraint trick

@@exercise A variadic join
Write `template <class... Ts> std::string join(std::string_view sep, const Ts&... values)` that converts each value with `to_text` and joins them:

- `to_text` uses `if constexpr`: `bool` → `true`/`false`; arithmetic → `std::format("{}", x)`; anything convertible to `std::string_view` → the text; anything matching your `Range` concept → `[a, b, c]` (recursively); otherwise fail with a `static_assert`
- `join` must work with zero arguments
@@starter
#include <concepts>
#include <format>
#include <iostream>
#include <map>
#include <string>
#include <string_view>
#include <type_traits>
#include <vector>

int main() {
    std::cout << "TODO\n";
}
@@expected
42, web01, true, 3.5, [1, 2, 3]
web01:443
[[a, b], [c]]
empty: ""
@@requires
if constexpr
concept Range
@@hint
`template <class T> concept Range = requires(const T& t) { t.begin(); t.end(); };`. Check `std::convertible_to<const T&, std::string_view>` before `Range`, because strings are ranges too. In `join`, build the result with a comma fold: `((out += (first ? "" : std::string(sep)) + to_text(values), first = false), ...);`. For the impossible branch use `static_assert(sizeof(T) == 0, "unsupported type")`.
@@solution
#include <concepts>
#include <format>
#include <iostream>
#include <map>
#include <string>
#include <string_view>
#include <type_traits>
#include <vector>

template <class T>
concept Range = requires(const T& t) {
    t.begin();
    t.end();
};

template <class T>
std::string to_text(const T& x) {
    if constexpr (std::is_same_v<T, bool>) {
        return x ? "true" : "false";
    } else if constexpr (std::is_arithmetic_v<T>) {
        return std::format("{}", x);
    } else if constexpr (std::convertible_to<const T&, std::string_view>) {
        return std::string(std::string_view(x));
    } else if constexpr (Range<T>) {
        std::string out = "[";
        bool first = true;
        for (const auto& item : x) {
            out += (first ? "" : ", ") + to_text(item);
            first = false;
        }
        return out + "]";
    } else {
        static_assert(sizeof(T) == 0, "unsupported type");
    }
}

template <class... Ts>
std::string join(std::string_view sep, const Ts&... values) {
    std::string out;
    bool first = true;
    ((out += (first ? "" : std::string(sep)) + to_text(values), first = false), ...);
    return out;
}

int main() {
    std::cout << join(", ", 42, "web01", true, 3.5, std::vector{1, 2, 3}) << '\n';
    std::cout << join(":", std::string("web01"), 443) << '\n';
    std::vector<std::vector<std::string>> nested = {{"a", "b"}, {"c"}};
    std::cout << join("", nested) << '\n';
    std::cout << "empty: \"" << join(", ") << "\"\n";
}

@@exercise A compile-time CRC-32 table
CRC-32 (used by zip, PNG and Ethernet) is fastest with a 256-entry lookup table. Build the table at compile time:

- `constexpr std::array<uint32_t, 256> make_table()`: for each `n`, run 8 rounds of `c = (c & 1) ? 0xEDB88320 ^ (c >> 1) : c >> 1`
- `constexpr uint32_t crc32(std::string_view data)`: start with `0xFFFFFFFF`, for each byte `c = table[(c ^ byte) & 0xFF] ^ (c >> 8)`, finish with `~c`
- prove it with `static_assert(crc32("123456789") == 0xCBF43926)` (the standard check value)
@@starter
#include <array>
#include <cstdint>
#include <format>
#include <iostream>
#include <string_view>

int main() {
    std::cout << "TODO\n";
}
@@expected
table[1] = 0x77073096
crc32("hello") = 0x3610a686
crc32("") = 0x00000000
check value verified at compile time
@@requires
static_assert(crc32("123456789") == 0xCBF43926)
constexpr std::array<uint32_t, 256>
@@hint
Store the table in a `constexpr auto table = make_table();` variable at namespace scope so `crc32` can use it in constant expressions. Cast each character with `static_cast<unsigned char>` before XORing. Print with `std::format("{:#010x}", value)`.
@@solution
#include <array>
#include <cstdint>
#include <format>
#include <iostream>
#include <string_view>

constexpr std::array<uint32_t, 256> make_table() {
    std::array<uint32_t, 256> table{};
    for (uint32_t n = 0; n < 256; ++n) {
        uint32_t c = n;
        for (int k = 0; k < 8; ++k) c = (c & 1) ? 0xEDB88320u ^ (c >> 1) : c >> 1;
        table[n] = c;
    }
    return table;
}

constexpr auto table = make_table();

constexpr uint32_t crc32(std::string_view data) {
    uint32_t c = 0xFFFFFFFFu;
    for (char ch : data) c = table[(c ^ static_cast<unsigned char>(ch)) & 0xFF] ^ (c >> 8);
    return ~c;
}

static_assert(crc32("123456789") == 0xCBF43926);

int main() {
    std::cout << std::format("table[1] = {:#010x}\n", table[1]);
    std::cout << std::format("crc32(\"hello\") = {:#010x}\n", crc32("hello"));
    std::cout << std::format("crc32(\"\") = {:#010x}\n", crc32(""));
    std::cout << "check value verified at compile time\n";
}
