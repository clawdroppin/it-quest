@@lesson x2-classes Classes and RAII
@@concepts objects
@@text
## Classes

A `class` is a `struct` whose members are private by default. Use classes when the object must keep its data valid (an **invariant**); keep plain data in structs.

```cpp
#include <iostream>
#include <stdexcept>
#include <string>

class Vlan {
public:
    Vlan(int id, std::string name) : id_(id), name_(std::move(name)) {   // member initializer list
        if (id < 1 || id > 4094) throw std::invalid_argument("VLAN id must be 1-4094");
    }

    int id() const { return id_; }                  // const: doesn't change the object
    const std::string& name() const { return name_; }
    void rename(const std::string& n) { name_ = n; }
    std::string describe() const { return "vlan " + std::to_string(id_) + " (" + name_ + ")"; }

private:
    int id_;
    std::string name_;
};

int main() {
    Vlan v(10, "users");
    v.rename("staff");
    std::cout << v.describe() << '\n';
    try {
        Vlan bad(5000, "oops");
    } catch (const std::invalid_argument& e) {
        std::cout << "error: " << e.what() << '\n';
    }
}
```

- The **constructor** has the class's name; its initializer list (`: id_(id), ...`) sets the members.
- Mark methods that don't modify the object **`const`**. You can then call them on `const` objects and references.
- `std::move(name)` moves the string's contents instead of copying them.

## RAII: the big idea of C++

**Resource Acquisition Is Initialization**: an object takes a resource (memory, a file, a lock, a socket) in its constructor and releases it in its **destructor** (`~ClassName()`), which runs automatically when the object goes out of scope, *even if an exception is thrown*.

```cpp
#include <iostream>
#include <string>

class Timer {
public:
    explicit Timer(std::string label) : label_(std::move(label)) { std::cout << "start " << label_ << '\n'; }
    ~Timer() { std::cout << "stop " << label_ << '\n'; }
private:
    std::string label_;
};

int main() {
    Timer whole("main");
    {
        Timer step("backup");
        std::cout << "  ...copying files\n";
    }   // step's destructor runs here
    std::cout << "  ...cleanup\n";
}   // whole's destructor runs here
```

This is why well-written C++ rarely leaks: `std::string`, `std::vector`, `std::ifstream`, `std::unique_ptr` and `std::lock_guard` all clean up after themselves. **Never write `new`/`delete` or open/close pairs by hand** when an RAII type exists.

## Operators

You can define what `==`, `<`, `<<` and others mean for your types:

```cpp
#include <iostream>
#include <string>

struct Endpoint {
    std::string host;
    int port;
    bool operator==(const Endpoint&) const = default;      // C++20: compare all members
    auto operator<=>(const Endpoint&) const = default;     // and <, >, <=, >=
};

std::ostream& operator<<(std::ostream& os, const Endpoint& e) {
    return os << e.host << ':' << e.port;
}

int main() {
    Endpoint a{"db01", 5432}, b{"db01", 5432}, c{"db02", 22};
    std::cout << a << ' ' << (a == b) << ' ' << (a < c) << '\n';
}
```

## Inheritance and virtual functions

```cpp
#include <iostream>
#include <memory>
#include <string>
#include <vector>

class Check {
public:
    virtual ~Check() = default;                 // a base class needs a virtual destructor
    virtual std::string name() const = 0;       // = 0: "pure virtual", subclasses must provide it
    virtual bool run() const = 0;
};

class DiskCheck : public Check {
public:
    explicit DiskCheck(int used) : used_(used) {}
    std::string name() const override { return "disk"; }
    bool run() const override { return used_ < 90; }
private:
    int used_;
};

class PingCheck : public Check {
public:
    std::string name() const override { return "ping"; }
    bool run() const override { return true; }
};

int main() {
    std::vector<std::unique_ptr<Check>> checks;
    checks.push_back(std::make_unique<DiskCheck>(95));
    checks.push_back(std::make_unique<PingCheck>());
    for (const auto& c : checks) std::cout << c->name() << ": " << (c->run() ? "PASS" : "FAIL") << '\n';
}
```

`override` makes the compiler check that you really are overriding something. Polymorphism works through pointers or references (`c->run()`), which is why the vector holds `unique_ptr`s, explained in the next lesson.

@@quiz
When does a local object's destructor run?
* When it goes out of scope, including when an exception leaves the scope
- Only when you call `delete`
- When the program ends
- Never automatically
? That guarantee is what makes RAII work.

@@quiz
What does `const` after a method, as in `int id() const`, mean?
* The method doesn't modify the object, so it can be called on const objects
- It returns a constant
- The method can't be overridden
- It can only be called once
? Mark every method that only reads as `const`.

@@quiz
Why does a base class with virtual functions need `virtual ~Base() = default;`?
* So deleting a derived object through a base pointer runs the derived destructor too
- To make the class abstract
- It's only style
- To prevent copying
? Without it, `delete basePtr` on a derived object is undefined behaviour.

@@exercise Bandwidth counter
Complete the `Counter` class: `add(bytes)` adds to the total; `total()` returns it; `average()` returns the average per `add` call as a `double` (0 if none). Make the read-only methods `const`. Adding a negative number must throw `std::invalid_argument`.
@@starter
#include <iostream>
#include <stdexcept>

class Counter {
public:
    void add(long long bytes) { /* TODO: not done yet */ }
    long long total() { return 0; }
    double average() { return 0; }
private:
    long long total_ = 0;
    int calls_ = 0;
};

void report(const Counter& c) {   // needs const methods
    std::cout << "total=" << c.total() << " avg=" << c.average() << '\n';
}

int main() {
    Counter c;
    report(c);
    c.add(1000);
    c.add(500);
    c.add(1500);
    report(c);
    try { c.add(-1); } catch (const std::invalid_argument& e) { std::cout << "rejected: " << e.what() << '\n'; }
}
@@expected
total=0 avg=0
total=3000 avg=1000
rejected: bytes must not be negative
@@hint
Add `const` after `total()` and `average()`. In `add`, `if (bytes < 0) throw std::invalid_argument("bytes must not be negative");`. For the average: `calls_ == 0 ? 0.0 : static_cast<double>(total_) / calls_`.
@@solution
#include <iostream>
#include <stdexcept>

class Counter {
public:
    void add(long long bytes) {
        if (bytes < 0) throw std::invalid_argument("bytes must not be negative");
        total_ += bytes;
        ++calls_;
    }
    long long total() const { return total_; }
    double average() const { return calls_ == 0 ? 0.0 : static_cast<double>(total_) / calls_; }
private:
    long long total_ = 0;
    int calls_ = 0;
};

void report(const Counter& c) {
    std::cout << "total=" << c.total() << " avg=" << c.average() << '\n';
}

int main() {
    Counter c;
    report(c);
    c.add(1000);
    c.add(500);
    c.add(1500);
    report(c);
    try { c.add(-1); } catch (const std::invalid_argument& e) { std::cout << "rejected: " << e.what() << '\n'; }
}

@@exercise Scope guard
Write a class `Guard` whose constructor prints `lock <name>` and whose destructor prints `unlock <name>`. The given `main` throws in the middle; the output must show that the unlock still happens:

```text
lock config
editing
unlock config
caught: disk full
```
@@starter
#include <iostream>
#include <stdexcept>
#include <string>

// write class Guard here

int main() {
    try {
        Guard g("config");
        std::cout << "editing\n";
        throw std::runtime_error("disk full");
    } catch (const std::exception& e) {
        std::cout << "caught: " << e.what() << '\n';
    }
}
@@expected
lock config
editing
unlock config
caught: disk full
@@requires
~Guard
@@hint
`class Guard { public: explicit Guard(std::string n) : name_(std::move(n)) { ... } ~Guard() { ... } private: std::string name_; };`
@@solution
#include <iostream>
#include <stdexcept>
#include <string>

class Guard {
public:
    explicit Guard(std::string name) : name_(std::move(name)) { std::cout << "lock " << name_ << '\n'; }
    ~Guard() { std::cout << "unlock " << name_ << '\n'; }
private:
    std::string name_;
};

int main() {
    try {
        Guard g("config");
        std::cout << "editing\n";
        throw std::runtime_error("disk full");
    } catch (const std::exception& e) {
        std::cout << "caught: " << e.what() << '\n';
    }
}
