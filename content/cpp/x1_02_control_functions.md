@@lesson x1-control-functions Decisions, Loops and Functions
@@concepts conditions, loops, functions
@@text
## if, switch and loops

```cpp
#include <iostream>

int main() {
    int cpu = 91;
    if (cpu >= 90) std::cout << "CRITICAL\n";
    else if (cpu >= 75) std::cout << "WARNING\n";
    else std::cout << "OK\n";

    int port = 443;
    switch (port) {
        case 22: std::cout << "SSH\n"; break;
        case 80:
        case 8080: std::cout << "HTTP\n"; break;
        case 443: std::cout << "HTTPS\n"; break;
        default: std::cout << "unknown\n";
    }

    for (int i = 1; i <= 3; ++i) std::cout << "attempt " << i << '\n';

    int n = 0;
    while (n < 10) n += 3;
    std::cout << n << '\n';
}
```

**Don't forget `break`** in a `switch`: without it, execution "falls through" into the next case. `switch` only works on integers, chars and enums, not strings.

`&&`, `||` and `!` combine conditions. `=` inside an `if` is a classic bug: `if (x = 5)` assigns and is always true. Warnings catch it.

## Functions

```cpp
#include <iostream>
#include <string>

double toGb(long long bytes) {
    return bytes / 1024.0 / 1024.0 / 1024.0;
}

std::string ping(const std::string& host, int count = 4) {     // default argument
    return "ping " + host + " x" + std::to_string(count);
}

int main() {
    std::cout << toGb(8'589'934'592) << " GB\n";      // ' separates digits
    std::cout << ping("10.0.0.1") << '\n' << ping("10.0.0.1", 2) << '\n';
}
```

A function must be **declared before** it's used: define it above `main`, or write a declaration like `double toGb(long long bytes);` first.

## Passing by value, reference and const reference

This is the most important idea in this lesson.

```cpp
#include <iostream>
#include <string>

void addOne(int n) { n++; }              // by value: works on a copy
void addOneRef(int& n) { n++; }          // by reference: changes the caller's variable
std::size_t len(const std::string& s) {  // const reference: no copy, can't change it
    return s.size();
}

int main() {
    int a = 1;
    addOne(a);
    std::cout << a << '\n';     // 1
    addOneRef(a);
    std::cout << a << '\n';     // 2
    std::cout << len("a fairly long string that we don't want to copy") << '\n';
}
```

Rules of thumb:

- small types (`int`, `double`, `char`, pointers): **by value**;
- big types you only read (`std::string`, `std::vector`, your classes): **`const T&`**;
- when the function must change the caller's object: **`T&`**, and make that obvious from the name.

## Overloading

Functions can share a name if their parameter types differ:

```cpp
#include <iostream>
#include <string>

std::string describe(int n) { return "int " + std::to_string(n); }
std::string describe(const std::string& s) { return "string " + s; }

int main() {
    std::cout << describe(5) << '\n' << describe(std::string("5")) << '\n';
}
```

## Returning several values

```cpp
#include <iostream>
#include <utility>

std::pair<int, int> divide(int a, int b) { return {a / b, a % b}; }

int main() {
    auto [q, r] = divide(17, 5);         // structured binding
    std::cout << q << " remainder " << r << '\n';
}
```

For more than two values, return a small `struct` with named fields (next lessons).

@@quiz
What does `void f(int& n) { n = 0; }` do to the caller's variable?
* Sets it to 0
- Nothing: n is a copy
- It doesn't compile
- Sets a copy to 0 and returns it
? `int&` is a reference: another name for the caller's variable.

@@quiz
A function only reads a large `std::vector<std::string>`. How should it take it?
* `const std::vector<std::string>&`
- `std::vector<std::string>` (by value)
- `std::vector<std::string>&`
- as a global variable
? A const reference avoids copying everything, and promises not to change it.

@@quiz
What happens in a `switch` case without `break`?
* Execution continues into the next case
- A compile error
- The switch ends
- The default case runs
? This "fall-through" is occasionally useful (several cases sharing code), but usually a bug.

@@exercise Normalize in place
Write `void normalize(std::string& host)` that removes spaces at both ends and makes the text lower case, changing the caller's string.
@@starter
#include <iostream>
#include <string>
#include <cctype>

void normalize(std::string& host) {
    // TODO: not done yet
}

int main() {
    std::string a = "  WEB01.Contoso.LOCAL ";
    std::string b = "db01";
    normalize(a);
    normalize(b);
    std::cout << '[' << a << "]\n[" << b << "]\n";
}
@@expected
[web01.contoso.local]
[db01]
@@requires
std::string& host
@@hint
`find_first_not_of(' ')` and `find_last_not_of(' ')` give the first and last non-space positions; then `host = host.substr(first, last - first + 1);`. Lower case with `std::tolower` on each `char&`.
@@solution
#include <iostream>
#include <string>
#include <cctype>

void normalize(std::string& host) {
    auto first = host.find_first_not_of(' ');
    if (first == std::string::npos) { host.clear(); return; }
    auto last = host.find_last_not_of(' ');
    host = host.substr(first, last - first + 1);
    for (char& c : host) c = static_cast<char>(std::tolower(static_cast<unsigned char>(c)));
}

int main() {
    std::string a = "  WEB01.Contoso.LOCAL ";
    std::string b = "db01";
    normalize(a);
    normalize(b);
    std::cout << '[' << a << "]\n[" << b << "]\n";
}

@@exercise Prefix to mask
Write `std::uint32_t maskFromPrefix(int prefix)` returning the subnet mask as a 32-bit number (`/24` → 0xFFFFFF00), and `std::string dotted(std::uint32_t)`. Careful: shifting a 32-bit value by 32 is undefined behaviour, so handle `/0` separately.
@@starter
#include <iostream>
#include <cstdint>
#include <string>

std::uint32_t maskFromPrefix(int prefix) {
    return 0; // TODO: not done yet
}

std::string dotted(std::uint32_t ip) {
    return ""; // TODO: not done yet
}

int main() {
    for (int p : {24, 26, 16, 32, 0})
        std::cout << '/' << p << " = " << dotted(maskFromPrefix(p)) << '\n';
}
@@expected
/24 = 255.255.255.0
/26 = 255.255.255.192
/16 = 255.255.0.0
/32 = 255.255.255.255
/0 = 0.0.0.0
@@hint
`prefix == 0 ? 0 : 0xFFFFFFFFu << (32 - prefix)`. For `dotted`, build the string with `std::to_string` of each octet.
@@solution
#include <iostream>
#include <cstdint>
#include <string>

std::uint32_t maskFromPrefix(int prefix) {
    return prefix == 0 ? 0u : 0xFFFFFFFFu << (32 - prefix);
}

std::string dotted(std::uint32_t ip) {
    return std::to_string((ip >> 24) & 0xFF) + "." + std::to_string((ip >> 16) & 0xFF) + "." +
           std::to_string((ip >> 8) & 0xFF) + "." + std::to_string(ip & 0xFF);
}

int main() {
    for (int p : {24, 26, 16, 32, 0})
        std::cout << '/' << p << " = " << dotted(maskFromPrefix(p)) << '\n';
}
