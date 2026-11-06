@@lesson n7-testing Testing C++: GoogleTest, Catch2 and Property Tests
@@concepts testing
@@text
The two dominant C++ test frameworks are **GoogleTest** (with GoogleMock) and **Catch2**. Both register tests with macros, run them all, and report failures with the expression that failed.

```cpp,norun
// GoogleTest
#include <gtest/gtest.h>

TEST(ParserTest, ParsesPort) {
    EXPECT_EQ(parse_port("443"), 443);      // EXPECT_*: record failure, keep going
    ASSERT_TRUE(parse_port("abc") < 0);     // ASSERT_*: record failure, stop this test
}

class InventoryTest : public ::testing::Test {      // fixture: shared setup per test
protected:
    void SetUp() override { inv.add("web01", 4); }
    Inventory inv;
};
TEST_F(InventoryTest, FindsHost) { EXPECT_EQ(inv.cores("web01"), 4); }

// parameterised tests, death tests (EXPECT_DEATH), and GoogleMock for mocks:
// MOCK_METHOD(bool, send, (const Batch&), (override));
```

```cpp,norun
// Catch2
#include <catch2/catch_test_macros.hpp>

TEST_CASE("ports parse", "[parser]") {
    REQUIRE(parse_port("443") == 443);      // expression decomposition prints both sides
    SECTION("garbage") { CHECK(parse_port("abc") == -1); }
}
```

## How the magic works

`TEST(...)` expands to a function definition plus a **static object** whose constructor registers the function in a global list before `main` runs. `EXPECT_EQ(a, b)` uses the preprocessor's `#` operator to turn the expressions into strings for the error message. You'll build exactly that in the first exercise.

## Beyond example-based tests

- **Property-based testing** (RapidCheck): generate hundreds of random inputs and check a property that must always hold (`parse(format(x)) == x`, "result is sorted", "agrees with a slow reference implementation"). Seed the generator so failures reproduce.
- **Fuzzing** (libFuzzer, AFL++): the fuzzer mutates inputs to explore new code paths, and the sanitizers turn bad memory access into crashes. Every parser that reads untrusted input should be fuzzed.
- **Coverage**: `--coverage` / llvm-cov / OpenCppCoverage show which lines tests never reached.
- **Benchmarks**: Google Benchmark, in CI on stable hardware.

@@quiz
Difference between `EXPECT_EQ` and `ASSERT_EQ` in GoogleTest?
* EXPECT records a failure and continues the test; ASSERT records it and returns from the test
- EXPECT is for numbers, ASSERT for strings
- ASSERT only works in debug builds
- No difference
? Use ASSERT when the rest of the test can't run meaningfully.

@@quiz
How does `TEST(name)` register a test before main runs?
- The linker scans for functions named TEST
* It defines a static object whose constructor adds the test to a global registry
- main calls each test by name
- With reflection
? Static initialisation runs before main.

@@quiz
Why seed the random generator in property-based tests?
- Seeds make tests faster
* So a failing case can be reproduced exactly
- Random numbers don't work without a seed
- To avoid duplicates
? Print the seed with every failure.

@@drill
Which GoogleTest macro checks equality and stops the test on failure?
= ASSERT_EQ
? EXPECT_EQ reports the failure but continues.

@@flashcards
`TEST(Suite, Name)` :: GoogleTest test
`TEST_F` :: test with a fixture
`EXPECT_EQ` :: soft check
`ASSERT_EQ` :: fatal check
`MOCK_METHOD` :: GoogleMock mock function
`TEST_CASE` / `SECTION` :: Catch2 test structure
`#x` (stringize) :: turn a macro argument into a string
Property-based test :: random inputs, invariant checks
libFuzzer :: coverage-guided fuzzing
Reference implementation :: slow but obviously correct oracle

@@exercise A mini test framework
Write the core of GoogleTest with macros:

- a global registry (a function returning a static `std::vector<TestCase>&`) and a `Registrar` struct whose constructor adds a test
- `TEST(name)`: declares `void name();`, defines a static `Registrar` for it, then starts the definition of `name`
- `EXPECT_EQ(a, b)`: if not equal, print `  FAILED: <a as text> == <b as text>: got [<a>] expected [<b>]` and mark the current test failed
- `run_all()` prints `[ RUN  ]`, then `[  OK  ]` or `[ FAIL ]`; an exception escaping a test prints `  unexpected exception: <what>` and fails it; finally a summary
@@starter
#include <iostream>
#include <sstream>
#include <stdexcept>
#include <string>
#include <vector>

int parse_port(const std::string& s) {
    try {
        size_t used = 0;
        int p = std::stoi(s, &used);
        return used == s.size() ? p : -1;
    } catch (const std::exception&) {
        return -1;
    }
}

std::string trim(const std::string& s) {   // bug: only trims the left side
    size_t start = s.find_first_not_of(' ');
    return start == std::string::npos ? "" : s.substr(start);
}

int parse_port_strict(const std::string& s) {
    if (s.empty()) throw std::invalid_argument("empty port");
    return parse_port(s);
}

int main() {
    std::cout << "TODO\n";
}
@@expected
[ RUN  ] parses_port
[  OK  ] parses_port
[ RUN  ] rejects_garbage
[  OK  ] rejects_garbage
[ RUN  ] trims_both_sides
  FAILED: trim("  x ") == std::string("x"): got [x ] expected [x]
[ FAIL ] trims_both_sides
[ RUN  ] strict_parser
  unexpected exception: empty port
[ FAIL ] strict_parser
4 tests, 2 passed, 2 failed
@@requires
#define TEST(name)
#define EXPECT_EQ(a, b)
@@hint
`#define TEST(name) void name(); static Registrar registrar_##name(#name, name); void name()`. In `EXPECT_EQ`, wrap the body in `do { ... } while (0)`, evaluate each argument once (`const auto& va = (a);`), and build the message with `std::ostringstream` and `#a " == " #b`. A global `bool current_failed` is reset before each test.
@@solution
#include <iostream>
#include <sstream>
#include <stdexcept>
#include <string>
#include <vector>

int parse_port(const std::string& s) {
    try {
        size_t used = 0;
        int p = std::stoi(s, &used);
        return used == s.size() ? p : -1;
    } catch (const std::exception&) {
        return -1;
    }
}

std::string trim(const std::string& s) {   // bug: only trims the left side
    size_t start = s.find_first_not_of(' ');
    return start == std::string::npos ? "" : s.substr(start);
}

int parse_port_strict(const std::string& s) {
    if (s.empty()) throw std::invalid_argument("empty port");
    return parse_port(s);
}

struct TestCase {
    const char* name;
    void (*fn)();
};

std::vector<TestCase>& registry() {
    static std::vector<TestCase> tests;
    return tests;
}

struct Registrar {
    Registrar(const char* name, void (*fn)()) { registry().push_back({name, fn}); }
};

bool current_failed = false;

#define TEST(name)                                       \
    void name();                                         \
    static Registrar registrar_##name(#name, name);      \
    void name()

#define EXPECT_EQ(a, b)                                                                     \
    do {                                                                                    \
        const auto& va = (a);                                                               \
        const auto& vb = (b);                                                               \
        if (!(va == vb)) {                                                                  \
            std::ostringstream msg;                                                         \
            msg << "  FAILED: " << #a " == " #b << ": got [" << va << "] expected [" << vb << ']'; \
            std::cout << msg.str() << '\n';                                                 \
            current_failed = true;                                                          \
        }                                                                                   \
    } while (0)

TEST(parses_port) { EXPECT_EQ(parse_port("443"), 443); }

TEST(rejects_garbage) { EXPECT_EQ(parse_port("abc"), -1); }

TEST(trims_both_sides) { EXPECT_EQ(trim("  x "), std::string("x")); }

TEST(strict_parser) { EXPECT_EQ(parse_port_strict(""), -1); }

int run_all() {
    int passed = 0, failed = 0;
    for (const auto& t : registry()) {
        std::cout << "[ RUN  ] " << t.name << '\n';
        current_failed = false;
        try {
            t.fn();
        } catch (const std::exception& e) {
            std::cout << "  unexpected exception: " << e.what() << '\n';
            current_failed = true;
        }
        std::cout << (current_failed ? "[ FAIL ] " : "[  OK  ] ") << t.name << '\n';
        (current_failed ? failed : passed)++;
    }
    std::cout << registry().size() << " tests, " << passed << " passed, " << failed << " failed\n";
    return failed == 0 ? 0 : 1;
}

int main() {
    run_all();
}

@@exercise A property-based test against a reference
`is_leap_buggy` only checks `year % 4 == 0`. Find its bug with a property test instead of guessing cases:

- property: for every year, `is_leap_buggy(y) == std::chrono::year(y).is_leap()` (the standard library is the reference)
- generate 1000 years in [1600, 2400] with `std::mt19937 rng(2026)` as `1600 + rng() % 801` (raw `mt19937` output is identical on every platform; the `*_distribution` classes are not)
- report the seed, the number of failures, the first counterexample, and the smallest failing year seen ("shrinking")
- then run the same property against a fixed `is_leap` and report that all cases pass
@@starter
#include <chrono>
#include <iostream>
#include <random>

bool is_leap_buggy(int y) { return y % 4 == 0; }

int main() {
    std::cout << "TODO\n";
}
@@expected
is_leap_buggy (seed 2026): 4 failures, first counterexample 1800, smallest 1800
is_leap (seed 2026): 1000/1000 passed
@@requires
std::mt19937
std::chrono::year
@@hint
A function `template <class F> void check(const char* name, F candidate)` that creates its own `std::mt19937 rng(2026)` so both runs see the same years. Track `failures`, `first` (0 until set) and `smallest`. The fix: `y % 4 == 0 && (y % 100 != 0 || y % 400 == 0)`.
@@solution
#include <chrono>
#include <iostream>
#include <random>

bool is_leap_buggy(int y) { return y % 4 == 0; }

bool is_leap(int y) { return y % 4 == 0 && (y % 100 != 0 || y % 400 == 0); }

template <class F>
void check(const char* name, F candidate) {
    const unsigned seed = 2026;
    std::mt19937 rng(seed);
    int failures = 0, first = 0, smallest = 0;
    for (int i = 0; i < 1000; ++i) {
        int y = 1600 + static_cast<int>(rng() % 801);
        bool expected = std::chrono::year(y).is_leap();
        if (candidate(y) != expected) {
            ++failures;
            if (first == 0) first = y;
            if (smallest == 0 || y < smallest) smallest = y;
        }
    }
    std::cout << name << " (seed " << seed << "): ";
    if (failures == 0) {
        std::cout << "1000/1000 passed\n";
    } else {
        std::cout << failures << " failures, first counterexample " << first << ", smallest " << smallest << '\n';
    }
}

int main() {
    check("is_leap_buggy", is_leap_buggy);
    check("is_leap", is_leap);
}
