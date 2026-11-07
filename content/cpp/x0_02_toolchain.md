@@lesson x0-toolchain How C++ Is Built: Preprocessor, Compiler, Linker
@@concepts numbers, processes
@@text
Most C++ errors make sense once you know the three stages that turn source into a program.

## 1. The preprocessor

Lines starting with `#` are handled before compilation, as **text substitution**:

```cpp,norun
#pragma once                 // in a header: include this file only once per .cpp
#include <vector>            // paste the contents of the standard header here
#include "inventory.h"       // paste your own header (searched next to this file first)
#define LOG_LEVEL 2          // a textual macro: prefer constexpr variables instead
#ifdef _WIN32                // compile this part only on Windows
#include <windows.h>
#endif
```

## 2. The compiler

Each `.cpp` file (a **translation unit**) is compiled separately into an object file (`.obj`/`.o`). The compiler only sees declarations from the headers it included, so a function declared in a header but defined elsewhere is fine at this stage.

## 3. The linker

The linker joins the object files and libraries into an executable, matching each use of a function to exactly one definition. Its errors look different:

```text,norun
error LNK2019: unresolved external symbol "int parse_port(...)"   (MSVC)
undefined reference to `parse_port(...)'                           (GCC/Clang)
multiple definition of `config'                                     (defined in a header included twice)
```

"Unresolved" means you declared but never defined it (or forgot to add the .cpp or library to the build). "Multiple definition" breaks the **One Definition Rule**: put definitions in .cpp files, or mark header definitions `inline`.

## Compiler flags worth knowing

| GCC/Clang | MSVC | Meaning |
|---|---|---|
| `-std=c++20` | `/std:c++20` | language version |
| `-Wall -Wextra -Wpedantic` | `/W4 /permissive-` | more warnings, stricter standard |
| `-Werror` | `/WX` | warnings fail the build |
| `-O2` | `/O2` | optimise |
| `-g` | `/Zi` | debug info |
| `-fsanitize=address,undefined` | `/fsanitize=address` | catch memory bugs at runtime |

[Compiler Explorer](https://godbolt.org) shows the assembly any compiler produces, which is great for learning what code really costs.

## Fixed-width integers and overflow

`int` is "at least 16 bits" (in practice 32). When size matters (file formats, network protocols) use `<cstdint>`: `int32_t`, `uint8_t`, `uint64_t`. Two rules every C++ programmer must know:

- **unsigned** arithmetic wraps around: `uint8_t(250 + 10) == 4`. That's well defined and useful for sequence numbers.
- **signed** overflow is **undefined behaviour**: the compiler may assume it never happens, and the program can do anything. Check *before* you add.

```cpp
#include <cstdint>
#include <iostream>
#include <limits>

int main() {
    uint8_t seq = 250;
    seq += 10;
    std::cout << int(seq) << '\n';                                  // 4
    std::cout << std::numeric_limits<int32_t>::max() << '\n';       // 2147483647
}
```

## main's arguments

```cpp,norun
int main(int argc, char* argv[]) {      // argv[0] is the program, argv[1..] the arguments
    for (int i = 1; i < argc; ++i) std::cout << argv[i] << '\n';
    return 0;                           // exit code: 0 = success
}
```

@@quiz
`error LNK2019: unresolved external symbol` comes from which stage?
- The preprocessor
- The compiler
* The linker
- The program at runtime
? A function was declared but its definition wasn't linked in.

@@quiz
What happens when a signed `int` overflows?
- It wraps around to the minimum
- It saturates at the maximum
* Undefined behaviour: the program has no defined meaning
- An exception is thrown
? Unsigned types wrap; signed types must never overflow.

@@quiz
Why use `#pragma once` in headers?
* So a header included several times in one .cpp is only processed once
- To speed up linking
- To make functions inline
- To compile the header separately
? Older code uses #ifndef include guards for the same thing.

@@drill
Which build stage combines object files into an executable?
= linker
= the linker
= linking
? "undefined reference" is a linker error.

@@flashcards
Preprocessor :: #include, #define, #ifdef: text substitution
Translation unit :: one .cpp after preprocessing
Linker :: joins object files, resolves symbols
ODR :: exactly one definition of each function/variable
`#pragma once` :: include guard
`int32_t` / `uint8_t` :: fixed-width integers from <cstdint>
Unsigned overflow :: wraps around (defined)
Signed overflow :: undefined behaviour
`-Wall -Wextra` / `/W4` :: enable warnings
`argc` / `argv` :: command-line arguments

@@exercise A command-line argument parser
Write `Args parse(const std::vector<std::string>& argv)` (argv[0] is the program name):

- `-v`, `--verbose` and `--dry-run` are **flags**
- any other `--name` is an **option** that needs a value, either `--name=value` or the next argument; if there is none, record `missing value for --name`
- any other argument starting with `-` is an error: `unknown flag -x`
- `--` ends option parsing: everything after it is positional
- everything else is positional

Print options sorted by name (use `std::map`), flags and positional arguments in order, and errors (`none` if empty).
@@starter
#include <iostream>
#include <map>
#include <string>
#include <vector>

struct Args {
    std::map<std::string, std::string> options;
    std::vector<std::string> flags, positional, errors;
};

int main() {
    std::cout << "TODO\n";
}
@@expected
options: name=web01 port=8080
flags: -v --dry-run
positional: inventory.txt --not-an-option
errors: none
errors: missing value for --port
errors: unknown flag -x
@@requires
std::map
.size()
@@hint
Loop with an index so you can consume the next argument: `for (size_t i = 1; i < argv.size(); ++i)`. For `--name=value`, `a.find('=')` splits it. A small `join(const std::vector<std::string>&)` helper prints lists separated by spaces.
@@solution
#include <iostream>
#include <map>
#include <string>
#include <vector>

struct Args {
    std::map<std::string, std::string> options;
    std::vector<std::string> flags, positional, errors;
};

Args parse(const std::vector<std::string>& argv) {
    Args args;
    bool options_done = false;
    for (size_t i = 1; i < argv.size(); ++i) {
        const std::string& a = argv[i];
        if (options_done || a.empty() || a[0] != '-') {
            args.positional.push_back(a);
        } else if (a == "--") {
            options_done = true;
        } else if (a == "-v" || a == "--verbose" || a == "--dry-run") {
            args.flags.push_back(a);
        } else if (a.rfind("--", 0) == 0) {
            auto eq = a.find('=');
            if (eq != std::string::npos) {
                args.options[a.substr(2, eq - 2)] = a.substr(eq + 1);
            } else if (i + 1 < argv.size()) {
                args.options[a.substr(2)] = argv[++i];
            } else {
                args.errors.push_back("missing value for " + a);
            }
        } else {
            args.errors.push_back("unknown flag " + a);
        }
    }
    return args;
}

std::string join(const std::vector<std::string>& items) {
    std::string out;
    for (const auto& s : items) out += (out.empty() ? "" : " ") + s;
    return out.empty() ? "none" : out;
}

int main() {
    Args a = parse({"deploy", "--port", "8080", "-v", "--name=web01", "--dry-run", "inventory.txt", "--", "--not-an-option"});
    std::cout << "options:";
    for (const auto& [k, v] : a.options) std::cout << ' ' << k << '=' << v;
    std::cout << "\nflags: " << join(a.flags) << '\n';
    std::cout << "positional: " << join(a.positional) << '\n';
    std::cout << "errors: " << join(a.errors) << '\n';
    std::cout << "errors: " << join(parse({"deploy", "--port"}).errors) << '\n';
    std::cout << "errors: " << join(parse({"deploy", "-x"}).errors) << '\n';
}

@@exercise Overflow-safe arithmetic and wrapping sequence numbers
Two halves of the integer story:

1. `bool checked_add(int32_t a, int32_t b, int32_t& out)`: returns false instead of overflowing. Check against `std::numeric_limits<int32_t>` **before** adding, so there's never undefined behaviour
2. a packet loss counter for 8-bit sequence numbers that wrap from 255 to 0: for each pair of consecutive numbers, `uint8_t(cur - prev)` is the distance even across the wrap; a distance above 1 means `distance - 1` lost packets
@@starter
#include <cstdint>
#include <iostream>
#include <limits>
#include <vector>

int main() {
    std::vector<uint8_t> seqs = {250, 251, 253, 254, 0, 1, 5};
    std::cout << "TODO\n";
}
@@expected
2000000000 + 100000000 = 2100000000
2000000000 + 200000000 = overflow
-2000000000 + -200000000 = overflow
max int32: 2147483647, max uint8: 255
gap after 251: 1 lost
gap after 254: 1 lost
gap after 1: 3 lost
total lost: 5
@@requires
std::numeric_limits<int32_t>
uint8_t(
@@hint
`if (b > 0 && a > max - b) return false; if (b < 0 && a < min - b) return false;` neither subtraction can overflow. Print `uint8_t` values through `int(...)`, because `std::cout << uint8_t` prints a character.
@@solution
#include <cstdint>
#include <iostream>
#include <limits>
#include <vector>

bool checked_add(int32_t a, int32_t b, int32_t& out) {
    constexpr int32_t max = std::numeric_limits<int32_t>::max();
    constexpr int32_t min = std::numeric_limits<int32_t>::min();
    if (b > 0 && a > max - b) return false;
    if (b < 0 && a < min - b) return false;
    out = a + b;
    return true;
}

void show(int32_t a, int32_t b) {
    int32_t sum;
    std::cout << a << " + " << b << " = ";
    if (checked_add(a, b, sum)) std::cout << sum << '\n';
    else std::cout << "overflow\n";
}

int main() {
    show(2000000000, 100000000);
    show(2000000000, 200000000);
    show(-2000000000, -200000000);
    std::cout << "max int32: " << std::numeric_limits<int32_t>::max()
              << ", max uint8: " << int(std::numeric_limits<uint8_t>::max()) << '\n';

    std::vector<uint8_t> seqs = {250, 251, 253, 254, 0, 1, 5};
    int total = 0;
    for (size_t i = 1; i < seqs.size(); ++i) {
        uint8_t distance = uint8_t(seqs[i] - seqs[i - 1]);
        if (distance > 1) {
            std::cout << "gap after " << int(seqs[i - 1]) << ": " << distance - 1 << " lost\n";
            total += distance - 1;
        }
    }
    std::cout << "total lost: " << total << '\n';
}
