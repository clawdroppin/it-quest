@@lesson x1-references References, Pointers, const and Spans
@@concepts variables, functions
@@text
C++ gives you several ways to refer to a value without copying it. Choosing the right one documents your intent and prevents bugs.

## References: another name for an object

```cpp
#include <iostream>

int main() {
    int cpu = 40;
    int& alias = cpu;      // must be initialised, can never be re-bound, never null
    alias += 10;
    std::cout << cpu << '\n';   // 50
}
```

## Pointers: an address that can change or be null

```cpp
#include <iostream>

int main() {
    int a = 1, b = 2;
    int* p = &a;           // & takes the address
    *p = 10;               // * follows the pointer
    p = &b;                // can be re-pointed
    p = nullptr;           // or point at nothing
    std::cout << a << ' ' << (p == nullptr) << '\n';   // 10 1
}
```

**Rule of thumb:** use a reference when something must exist; use a pointer when "nothing" is a valid answer (like the result of a search). Don't use raw pointers for ownership: that's what `std::unique_ptr` is for.

## const in all the places

```cpp,norun
void print(const Server& s);         // read-only access, no copy: the default for passing objects
const Server* find(...);             // pointer to a const Server: can't modify through it
Server* const p = &s;                // const pointer: can't re-point, can modify *p
int Server::load() const;            // member function that doesn't modify the object
```

Make everything `const` that doesn't need to change. A `const` object can only call `const` member functions, so "const correctness" spreads through a codebase; start it early.

## Arrays, std::array and std::span

C arrays (`int a[4]`) decay to a pointer when passed to functions and lose their size. Prefer `std::array<int, 4>` (fixed size, knows its length) or `std::vector`. To accept "any contiguous sequence" use **`std::span<const T>`**: a pointer plus a length, with no copy, which works with arrays, vectors and raw buffers alike.

```cpp
#include <array>
#include <iostream>
#include <span>
#include <vector>

int sum(std::span<const int> values) {
    int total = 0;
    for (int v : values) total += v;
    return total;
}

int main() {
    std::array<int, 3> a = {1, 2, 3};
    std::vector<int> v = {10, 20, 30, 40};
    std::cout << sum(a) << ' ' << sum(v) << ' ' << sum(std::span(v).subspan(1, 2)) << '\n';  // 6 100 50
}
```

## Dangling: the classic mistake

A reference or pointer must never outlive what it refers to:

```cpp,norun
const std::string& name() { std::string s = "web01"; return s; }  // s dies: dangling reference
std::string_view v = std::string("temp");                        // the temporary dies at the ;
Server* p = &servers[0]; servers.push_back(x);                   // the vector may reallocate: p dangles
```

@@quiz
When should a function take a `const T&` parameter?
* To read an object without copying it and without modifying it
- To modify the caller's object
- When the argument may be null
- Only for numbers
? It's the default for passing non-trivial objects.

@@quiz
A search that may find nothing should return...
- A reference
* A pointer (nullptr for not found) or std::optional
- A copy of a default object
- An exception
? References can't be null.

@@quiz
Why can `Server* p = &servers[0]` break after `servers.push_back(x)`?
- push_back clears the vector
* The vector may move its elements to a larger buffer, leaving p pointing at freed memory
- Pointers expire after one statement
- It can't break
? Iterators, pointers and references into a vector are invalidated by reallocation.

@@drill
Which keyword is the null pointer value in modern C++?
= nullptr
? Don't use NULL or 0.

@@flashcards
`T&` :: reference: alias, never null, can't re-bind
`T*` :: pointer: address, may be null, can re-point
`&x` :: address of x
`*p` :: object p points to
`const T&` :: read-only access without a copy
`const T*` :: pointer to const
`T* const` :: const pointer
`std::array<T, N>` :: fixed-size array that knows its size
`std::span<const T>` :: view over contiguous data
Dangling :: refers to an object that no longer exists

@@exercise Find and update servers
Write const-correct helpers over `std::vector<Server>`:

- `Server* find(std::vector<Server>& servers, std::string_view name)` returning `nullptr` when not found, and a `const` overload for read-only vectors
- `void bump(Server& s, int by)` that raises the CPU value, capped at 100
- `const Server& busiest(const std::vector<Server>& servers)` (assume non-empty)
@@starter
#include <iostream>
#include <string>
#include <string_view>
#include <vector>

struct Server {
    std::string name;
    int cpu;
};

int main() {
    std::vector<Server> servers = {{"web01", 40}, {"db01", 75}, {"web02", 12}};
    std::cout << "TODO\n";
}
@@expected
found web01 (cpu 40)
after bump: web01 cpu 50
db09: not found
after big bump: web02 cpu 100
busiest: web02 (100)
@@requires
const Server*
Server& s
@@hint
Loop with `for (auto& s : servers)` (or `const auto&` in the const overload) and `return &s;` on a match. The const overload's signature is `const Server* find(const std::vector<Server>& servers, std::string_view name)`.
@@solution
#include <iostream>
#include <string>
#include <string_view>
#include <vector>

struct Server {
    std::string name;
    int cpu;
};

Server* find(std::vector<Server>& servers, std::string_view name) {
    for (auto& s : servers)
        if (s.name == name) return &s;
    return nullptr;
}

const Server* find(const std::vector<Server>& servers, std::string_view name) {
    for (const auto& s : servers)
        if (s.name == name) return &s;
    return nullptr;
}

void bump(Server& s, int by) {
    s.cpu = s.cpu + by > 100 ? 100 : s.cpu + by;
}

const Server& busiest(const std::vector<Server>& servers) {
    const Server* best = &servers[0];
    for (const auto& s : servers)
        if (s.cpu > best->cpu) best = &s;
    return *best;
}

int main() {
    std::vector<Server> servers = {{"web01", 40}, {"db01", 75}, {"web02", 12}};
    if (Server* s = find(servers, "web01")) {
        std::cout << "found " << s->name << " (cpu " << s->cpu << ")\n";
        bump(*s, 10);
        std::cout << "after bump: " << s->name << " cpu " << s->cpu << '\n';
    }
    const auto& readonly = servers;
    if (const Server* s = find(readonly, "db09"); s == nullptr) std::cout << "db09: not found\n";
    if (Server* s = find(servers, "web02")) {
        bump(*s, 200);
        std::cout << "after big bump: " << s->name << " cpu " << s->cpu << '\n';
    }
    const Server& top = busiest(servers);
    std::cout << "busiest: " << top.name << " (" << top.cpu << ")\n";
}

@@exercise The IP header checksum with std::span
Write `uint16_t checksum(std::span<const uint8_t> data)`, the Internet checksum (RFC 1071) used by IPv4, ICMP, TCP and UDP:

- add the data as big-endian 16-bit words (`data[i] << 8 | data[i + 1]`) into a `uint32_t`
- fold the carries: `while (sum >> 16) sum = (sum & 0xFFFF) + (sum >> 16);`
- return the ones' complement `uint16_t(~sum)`

Compute it over the 20-byte header with the checksum field (bytes 10 and 11) zeroed, write it into the header, then verify that the checksum over the complete header is 0. Use `subspan` to read the first word.
@@starter
#include <array>
#include <cstdint>
#include <format>
#include <iostream>
#include <span>

int main() {
    std::array<uint8_t, 20> header = {
        0x45, 0x00, 0x00, 0x73, 0x00, 0x00, 0x40, 0x00, 0x40, 0x11,
        0x00, 0x00, 0xc0, 0xa8, 0x00, 0x01, 0xc0, 0xa8, 0x00, 0xc7};
    std::cout << "TODO\n";
}
@@expected
checksum: 0xb861
verify: 0x0000 (valid)
header bytes: 20, first word: 0x4500
@@requires
std::span<const uint8_t>
subspan
@@hint
`std::format("{:#06x}", value)` prints `0x` plus 4 hex digits. To store the result: `header[10] = uint8_t(sum >> 8); header[11] = uint8_t(sum & 0xFF);`. `auto first = std::span(header).subspan(0, 2);`.
@@solution
#include <array>
#include <cstdint>
#include <format>
#include <iostream>
#include <span>

uint16_t checksum(std::span<const uint8_t> data) {
    uint32_t sum = 0;
    for (size_t i = 0; i + 1 < data.size(); i += 2) sum += uint32_t(data[i]) << 8 | data[i + 1];
    if (data.size() % 2 == 1) sum += uint32_t(data.back()) << 8;
    while (sum >> 16) sum = (sum & 0xFFFF) + (sum >> 16);
    return uint16_t(~sum);
}

int main() {
    std::array<uint8_t, 20> header = {
        0x45, 0x00, 0x00, 0x73, 0x00, 0x00, 0x40, 0x00, 0x40, 0x11,
        0x00, 0x00, 0xc0, 0xa8, 0x00, 0x01, 0xc0, 0xa8, 0x00, 0xc7};
    uint16_t sum = checksum(header);
    std::cout << std::format("checksum: {:#06x}\n", sum);
    header[10] = uint8_t(sum >> 8);
    header[11] = uint8_t(sum & 0xFF);
    uint16_t verify = checksum(header);
    std::cout << std::format("verify: {:#06x} ({})\n", verify, verify == 0 ? "valid" : "corrupt");
    auto first = std::span<const uint8_t>(header).subspan(0, 2);
    std::cout << std::format("header bytes: {}, first word: {:#06x}\n", header.size(), first[0] << 8 | first[1]);
}
