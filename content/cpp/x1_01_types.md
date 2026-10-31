@@lesson x1-types Numbers, Strings and Input
@@concepts numbers, strings, input
@@text
## Number types

| Type | Typical size | Notes |
|---|---|---|
| `int` | 32 bits | about ±2.1 billion |
| `long long` | 64 bits | big whole numbers |
| `std::uint8_t`, `std::uint32_t`, `std::int64_t` ... | exactly 8, 32, 64 bits | from `<cstdint>`: use them for network data and file formats |
| `std::size_t` | 64 bits on 64-bit systems | sizes and indexes (unsigned) |
| `double` | 64 bits | decimals |

```cpp
#include <iostream>
#include <cstdint>
#include <cmath>

int main() {
    std::cout << 7 / 2 << ' ' << 7 / 2.0 << ' ' << 7 % 2 << '\n';   // 3 3.5 1
    std::cout << std::pow(2, 10) << ' ' << std::sqrt(81.0) << '\n';

    std::uint8_t ttl = 255;
    ttl = ttl + 1;                       // unsigned types wrap around: 0
    std::cout << +ttl << '\n';           // + prints it as a number, not a char

    std::uint32_t ip = (192u << 24) | (168u << 16) | (1u << 8) | 10u;
    std::cout << ip << '\n';             // 3232235786: an IPv4 address is just a 32-bit number
    std::cout << ((ip >> 24) & 0xFF) << '\n';   // 192
}
```

**Signed overflow** (`int` going past its maximum) is undefined behaviour in C++. Unsigned types wrap around, which is defined. Bit operators (`<<`, `>>`, `&`, `|`, `^`, `~`) are everyday tools for network and hardware code.

## Formatting output

```cpp
#include <iostream>
#include <iomanip>

int main() {
    double ratio = 0.8734;
    std::cout << std::fixed << std::setprecision(1) << ratio * 100 << "%\n";   // 87.3%
    std::cout << std::setw(8) << std::left << "web01" << '|' << std::setw(5) << std::right << 42 << "|\n";
    std::cout << std::hex << 255 << std::dec << ' ' << 255 << '\n';            // ff 255
}
```

These manipulators are "sticky": `std::fixed` stays on until you change it. C++20 also has `std::format("{:.1f}%", x)` in `<format>`, like Python's f-strings, if your compiler is new enough.

## std::string

```cpp
#include <iostream>
#include <string>
#include <cctype>

int main() {
    std::string fqdn = "web01.contoso.local";
    std::cout << fqdn.size() << '\n';
    std::cout << fqdn.substr(0, 5) << '\n';          // web01
    auto dot = fqdn.find('.');                       // position of the first '.'
    std::cout << fqdn.substr(dot + 1) << '\n';       // contoso.local
    std::cout << (fqdn.find("prod") == std::string::npos ? "not prod" : "prod") << '\n';
    std::cout << fqdn.starts_with("web") << fqdn.ends_with(".local") << '\n';   // C++20

    std::string path = "C:\\Logs";
    path += "\\app.log";                              // strings are mutable in C++
    std::cout << path << '\n';
    std::cout << R"(raw: C:\no\escapes\needed)" << '\n';

    std::string upper = fqdn;
    for (char& c : upper) c = static_cast<char>(std::toupper(static_cast<unsigned char>(c)));
    std::cout << upper << '\n';
}
```

`find` returns `std::string::npos` when nothing is found. Text to numbers: `std::stoi("443")` (throws on bad input), `std::to_string(443)` the other way.

## Reading input

```cpp,norun
std::string name;
int age = 0;
std::cin >> name >> age;            // reads words separated by spaces
std::string line;
std::getline(std::cin, line);       // reads a whole line
```

`std::cin >> x` stops at whitespace. Use `std::getline` for whole lines, and a loop `while (std::getline(std::cin, line))` to read everything.

@@quiz
`std::uint8_t x = 255; x = x + 1;` What's in `x`?
* `0`
- `256`
- undefined behaviour
- a compile error
? Unsigned integers wrap around modulo 2^bits. It's signed overflow that's undefined.

@@quiz
What does `s.find("x")` return when "x" isn't in the string?
* `std::string::npos`
- `-1` as an `int`
- `0`
- it throws
? `npos` is the largest `size_t` value. Compare with it, not with -1.

@@quiz
What is `(ip >> 24) & 0xFF` for a 32-bit IPv4 address?
* The first octet (e.g. 192 for 192.168.1.10)
- The last octet
- The subnet mask
- The whole address
? Shifting right by 24 moves the top 8 bits down; `& 0xFF` keeps only those 8 bits.

@@drill
Which header gives you exact-size types like `std::uint32_t`?
= <cstdint>
= cstdint
? `#include <cstdint>`.

@@exercise Dotted quad
Convert the 32-bit number `ip` into its dotted form by shifting and masking. Print `192.168.1.10`.
@@starter
#include <iostream>
#include <cstdint>

int main() {
    std::uint32_t ip = 3232235786u;
    // print the four octets separated by dots
}
@@expected
192.168.1.10
@@requires
>>
@@hint
The octets are `(ip >> 24) & 0xFF`, `(ip >> 16) & 0xFF`, `(ip >> 8) & 0xFF` and `ip & 0xFF`.
@@solution
#include <iostream>
#include <cstdint>

int main() {
    std::uint32_t ip = 3232235786u;
    std::cout << ((ip >> 24) & 0xFF) << '.' << ((ip >> 16) & 0xFF) << '.'
              << ((ip >> 8) & 0xFF) << '.' << (ip & 0xFF) << '\n';
}

@@exercise Sum the input
Read lines until the input ends. Each line is `name bytes`. Print the total in MB with one decimal (1 MB = 1048576 bytes), like `total: 3.5 MB`.
@@stdin
app.log 1048576
db.bak 2097152
notes.txt 524288
@@starter
#include <iostream>
#include <iomanip>
#include <string>

int main() {
    std::string name;
    long long bytes = 0, total = 0;
    // read pairs with std::cin >> name >> bytes
}
@@expected
total: 3.5 MB
@@hint
`while (std::cin >> name >> bytes) total += bytes;` then print `total / 1048576.0` with `std::fixed << std::setprecision(1)`.
@@solution
#include <iostream>
#include <iomanip>
#include <string>

int main() {
    std::string name;
    long long bytes = 0, total = 0;
    while (std::cin >> name >> bytes) total += bytes;
    std::cout << "total: " << std::fixed << std::setprecision(1) << total / 1048576.0 << " MB\n";
}
