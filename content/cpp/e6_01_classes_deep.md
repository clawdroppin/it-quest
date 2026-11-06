@@lesson e6-classes Classes in Depth: Special Members and the Rule of Zero/Five
@@concepts objects
@@text
Every C++ class has up to six **special member functions** that the compiler may write for you:

| Member | Called when |
|---|---|
| default constructor `T()` | `T x;` |
| destructor `~T()` | the object's lifetime ends |
| copy constructor `T(const T&)` | `T b = a;`, pass by value |
| copy assignment `T& operator=(const T&)` | `b = a;` |
| move constructor `T(T&&)` | `T b = std::move(a);`, returning a local |
| move assignment `T& operator=(T&&)` | `b = std::move(a);`, `b = T(...)` |

## Rule of zero

If your members already manage themselves (`std::string`, `std::vector`, `std::unique_ptr`), write **none** of these: the compiler-generated versions do the right thing. That's the goal for almost every class.

## Rule of five

If you write a destructor because the class owns a raw resource (memory from `new[]`, a file handle, a socket), you almost certainly need all five, or the defaults will copy the raw handle and free it twice. Usually the better fix is to wrap the resource in its own small RAII class (or a `unique_ptr` with a custom deleter) and go back to the rule of zero.

```cpp,norun
class Socket {
public:
    explicit Socket(int fd) : fd_(fd) {}
    ~Socket() { if (fd_ >= 0) ::close(fd_); }
    Socket(const Socket&) = delete;                       // sockets can't be copied
    Socket& operator=(const Socket&) = delete;
    Socket(Socket&& o) noexcept : fd_(std::exchange(o.fd_, -1)) {}
    Socket& operator=(Socket&& o) noexcept {
        if (this != &o) { reset(); fd_ = std::exchange(o.fd_, -1); }
        return *this;
    }
private:
    int fd_ = -1;
};
```

## Other class essentials

- `explicit` on single-argument constructors stops surprise conversions (`Timeout t = 30;`).
- Members are initialised in **declaration order**, not the order in the initializer list.
- `= default` asks for the compiler version; `= delete` forbids an operation.
- C++20's `auto operator<=>(const T&) const = default;` generates `<`, `<=`, `>`, `>=` comparing members in order, and `= default` on `operator==` generates `==` and `!=`.
- Mark move operations `noexcept`, or `std::vector` will copy instead of move when it grows.

@@quiz
A class only has `std::string` and `std::vector` members. Which special members should you write?
* None: the compiler-generated ones are correct (rule of zero)
- All five
- Only the destructor
- Only the copy constructor
? The members manage their own resources.

@@quiz
What goes wrong if a class with a raw `new[]` buffer has a destructor but the default copy constructor?
- Nothing
* Copies share the same pointer and both destructors delete it: a double free
- The copy is empty
- It doesn't compile
? Write all five, or wrap the buffer.

@@quiz
What does `auto operator<=>(const Version&) const = default;` give you?
- Only ==
* All relational operators, comparing members in declaration order
- Hashing
- String conversion
? Add `operator== = default` for equality too (it's implied when <=> is defaulted).

@@drill
How many special member functions does the rule of five cover?
= 5
= five
? Destructor, copy constructor, copy assignment, move constructor, move assignment.

@@flashcards
Rule of zero :: let members manage resources; write no special members
Rule of five :: destructor + copy ctor/assign + move ctor/assign
`= delete` :: forbid an operation
`explicit` :: no implicit conversions
`noexcept` move :: lets vector move when growing
`std::exchange(x, v)` :: set x to v, return the old value
`operator<=>` :: three-way comparison (spaceship)
Initialisation order :: declaration order of members

@@exercise A buffer with the rule of five
Write `class Buffer` that owns a `char*` from `new char[size]`, with all five special members, each logging what it does:

- `explicit Buffer(size_t n)`: `alloc n`
- copy constructor: `copy n` (allocate and copy the bytes)
- move constructor (`noexcept`): `move n`, leaves the source empty (size 0, null data)
- copy assignment: `self-assign ignored` when assigning to itself, otherwise `copy-assign <new> over <old>`
- move assignment: `move-assign <new> over <old>`, freeing the old buffer
- destructor: `free n`, or `free empty` for an empty buffer
@@starter
#include <algorithm>
#include <cstddef>
#include <iostream>
#include <utility>

int main() {
    std::cout << "TODO\n";
}
@@expected
alloc 16
copy 16
move 16
a after move: size 0
copy-assign 16 over 16
self-assign ignored
alloc 8
move-assign 8 over 16
free empty
sizes: a=0 b=16 c=8
free 8
free 16
free empty
@@requires
noexcept
operator=(Buffer&&
@@hint
Use `std::exchange(other.data_, nullptr)` and `std::exchange(other.size_, 0)` to steal from the source. The main program: `Buffer a(16); Buffer b = a; Buffer c = std::move(a);`, print `a.size()`, then `b = c;`, `Buffer& same = b; b = same;`, `c = Buffer(8);`, and print the sizes. The temporary `Buffer(8)` is destroyed (empty) at the end of its statement.
@@solution
#include <algorithm>
#include <cstddef>
#include <iostream>
#include <utility>

class Buffer {
public:
    explicit Buffer(size_t n) : data_(new char[n]{}), size_(n) { std::cout << "alloc " << n << '\n'; }

    Buffer(const Buffer& other) : data_(new char[other.size_]), size_(other.size_) {
        std::copy(other.data_, other.data_ + size_, data_);
        std::cout << "copy " << size_ << '\n';
    }

    Buffer(Buffer&& other) noexcept
        : data_(std::exchange(other.data_, nullptr)), size_(std::exchange(other.size_, 0)) {
        std::cout << "move " << size_ << '\n';
    }

    Buffer& operator=(const Buffer& other) {
        if (this == &other) {
            std::cout << "self-assign ignored\n";
            return *this;
        }
        char* fresh = new char[other.size_];
        std::copy(other.data_, other.data_ + other.size_, fresh);
        std::cout << "copy-assign " << other.size_ << " over " << size_ << '\n';
        delete[] data_;
        data_ = fresh;
        size_ = other.size_;
        return *this;
    }

    Buffer& operator=(Buffer&& other) noexcept {
        if (this != &other) {
            std::cout << "move-assign " << other.size_ << " over " << size_ << '\n';
            delete[] data_;
            data_ = std::exchange(other.data_, nullptr);
            size_ = std::exchange(other.size_, 0);
        }
        return *this;
    }

    ~Buffer() {
        if (data_) std::cout << "free " << size_ << '\n';
        else std::cout << "free empty\n";
        delete[] data_;
    }

    size_t size() const { return size_; }

private:
    char* data_;
    size_t size_;
};

int main() {
    Buffer a(16);
    Buffer b = a;
    Buffer c = std::move(a);
    std::cout << "a after move: size " << a.size() << '\n';
    b = c;
    Buffer& same = b;
    b = same;
    c = Buffer(8);
    std::cout << "sizes: a=" << a.size() << " b=" << b.size() << " c=" << c.size() << '\n';
}

@@exercise Versions with the spaceship operator
Write `struct Version { int major, minor, patch; }` that sorts correctly:

- a defaulted `operator<=>` (and `operator==`)
- `static std::optional<Version> parse(std::string_view)` for `x.y.z` (use `std::from_chars`; anything else is invalid)
- `std::string str() const`

Parse the list (reporting invalid entries), sort it, and show why string comparison is wrong for versions. Then print the versions in the range [1.2.4, 2.0.0).
@@starter
#include <algorithm>
#include <charconv>
#include <compare>
#include <iostream>
#include <optional>
#include <string>
#include <string_view>
#include <vector>

int main() {
    std::vector<std::string> raw = {"1.10.0", "1.2.3", "1.2.10", "0.9.9", "2.0.0", "bad"};
    std::cout << "TODO\n";
}
@@expected
invalid: bad
sorted: 0.9.9 1.2.3 1.2.10 1.10.0 2.0.0
1.2.10 > 1.2.3: true
as strings: false
in [1.2.4, 2.0.0): 1.2.10 1.10.0
@@requires
operator<=>
std::optional<Version>
@@hint
A helper that parses one number and advances a `const char*`: `auto [p, ec] = std::from_chars(first, last, out); if (ec != std::errc{}) return false; first = p;`. After each of the first two numbers, require a `.`; after the third, require the end. Print booleans with `std::boolalpha`.
@@solution
#include <algorithm>
#include <charconv>
#include <compare>
#include <iostream>
#include <optional>
#include <string>
#include <string_view>
#include <vector>

struct Version {
    int major = 0, minor = 0, patch = 0;

    auto operator<=>(const Version&) const = default;
    bool operator==(const Version&) const = default;

    static std::optional<Version> parse(std::string_view s) {
        Version v;
        const char* p = s.data();
        const char* end = s.data() + s.size();
        int* parts[] = {&v.major, &v.minor, &v.patch};
        for (int i = 0; i < 3; ++i) {
            auto [next, ec] = std::from_chars(p, end, *parts[i]);
            if (ec != std::errc{}) return std::nullopt;
            p = next;
            if (i < 2) {
                if (p == end || *p != '.') return std::nullopt;
                ++p;
            }
        }
        if (p != end) return std::nullopt;
        return v;
    }

    std::string str() const {
        return std::to_string(major) + '.' + std::to_string(minor) + '.' + std::to_string(patch);
    }
};

int main() {
    std::vector<std::string> raw = {"1.10.0", "1.2.3", "1.2.10", "0.9.9", "2.0.0", "bad"};
    std::vector<Version> versions;
    for (const auto& r : raw) {
        if (auto v = Version::parse(r)) versions.push_back(*v);
        else std::cout << "invalid: " << r << '\n';
    }
    std::sort(versions.begin(), versions.end());
    std::cout << "sorted:";
    for (const auto& v : versions) std::cout << ' ' << v.str();
    std::cout << std::boolalpha << "\n1.2.10 > 1.2.3: " << (Version{1, 2, 10} > Version{1, 2, 3}) << '\n';
    std::cout << "as strings: " << (std::string("1.2.10") > std::string("1.2.3")) << '\n';
    Version low{1, 2, 4}, high{2, 0, 0};
    std::cout << "in [1.2.4, 2.0.0):";
    for (const auto& v : versions)
        if (v >= low && v < high) std::cout << ' ' << v.str();
    std::cout << '\n';
}
