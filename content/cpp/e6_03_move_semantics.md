@@lesson e6-move Move Semantics and Perfect Forwarding in Depth
@@concepts objects, functions
@@text
Move semantics let C++ transfer resources instead of copying them. Understanding exactly when moves happen is what separates "it works" from "it's fast".

## Value categories, simplified

- An **lvalue** has a name and an address: `x`, `v[0]`, `*p`. You can't steal from it implicitly, because it's still in use.
- An **rvalue** is a temporary or something you've declared you're done with: `std::string("a")`, `f()`, `std::move(x)`.
- `T&&` in a function parameter binds to rvalues: that's the move constructor's signature.

`std::move` doesn't move anything. It's a cast to an rvalue reference that *allows* the receiving overload to steal. A moved-from object is valid but unspecified: you may assign to it or destroy it, but don't read it.

## Where copies hide

```cpp,norun
std::vector<std::string> names;
std::string n = load();
names.push_back(n);               // copy: n is still used afterwards
names.push_back(std::move(n));    // move: n is now empty-ish
names.emplace_back("web01");      // constructs the string inside the vector: no temporary at all
for (auto s : names) ...          // copies every element! use const auto&
```

## Return values

`return local;` either constructs directly in the caller's storage (**NRVO**) or moves; it never copies a movable type. Don't write `return std::move(local);`: it disables NRVO. Returning a prvalue (`return Server{...};`) is guaranteed to be elided since C++17.

## noexcept and vector growth

When a `vector` grows it must move its elements to new storage. If an element's move constructor could throw, a failure halfway would lose data, so `vector` copies instead (`std::move_if_noexcept`). Mark your move operations `noexcept`; the compiler-generated ones already are when all members' are.

## Forwarding references and std::forward

In a template, `T&&` with deduced `T` is a **forwarding reference**: it binds to anything, and `T` remembers whether the argument was an lvalue. `std::forward<T>(arg)` passes it on unchanged: lvalues as lvalues, rvalues as rvalues. That's how `make_unique` and `emplace_back` construct objects without extra copies.

```cpp,norun
template <class T, class... Args>
std::unique_ptr<T> make_unique(Args&&... args) {
    return std::unique_ptr<T>(new T(std::forward<Args>(args)...));
}
```

@@quiz
What does `std::move(x)` do by itself?
- Moves x's contents somewhere else
* Casts x to an rvalue reference so a move overload can be chosen
- Frees x
- Copies x
? The actual move happens in the constructor or assignment it enables.

@@quiz
Why mark a move constructor `noexcept`?
* So std::vector moves elements when it reallocates instead of copying them
- So it can't be called
- It's required to compile
- To make it faster to compile
? vector uses move_if_noexcept for the strong exception guarantee.

@@quiz
In `template <class T> void f(T&& x)`, what is `T&&`?
- Always an rvalue reference
* A forwarding reference: binds to lvalues and rvalues, and std::forward preserves which
- A pointer
- An error
? Only with deduced T.

@@flashcards
lvalue :: named object with an address
rvalue :: temporary or std::move'd object
`T&&` (non-template) :: rvalue reference
`T&&` (deduced T) :: forwarding reference
`std::move` :: cast to rvalue
`std::forward<T>` :: preserve value category
`emplace_back` :: construct in place
NRVO :: named return value optimisation
`std::move_if_noexcept` :: move only if it can't throw
Moved-from state :: valid but unspecified

@@exercise Counting copies and moves
Write `Tracked` (noexcept move) and `Legacy` (move **not** noexcept); both count copies and moves in static counters. Then measure:

1. `push_back(lvalue)` and `push_back(std::move(x))` into a vector with spare capacity
2. `emplace_back()` into spare capacity
3. growth: `reserve(1)`, `emplace_back()`, reset counters, `emplace_back()` again (forcing a reallocation), for both types
4. returning a local `Tracked` from a function (only the copy count is guaranteed: NRVO may or may not happen)
@@starter
#include <iostream>
#include <vector>

struct Tracked {
    static inline int copies = 0, moves = 0;
    Tracked() = default;
    Tracked(const Tracked&) { ++copies; }
    Tracked(Tracked&&) noexcept { ++moves; }
    static void reset() { copies = moves = 0; }
};

int main() {
    std::cout << "TODO\n";
}
@@expected
push_back(lvalue): copies=1 moves=0
push_back(std::move): copies=0 moves=1
emplace_back: copies=0 moves=0
grow with noexcept move: copies=0 moves=1
grow without noexcept: copies=1 moves=0
return local: copies=0
@@requires
noexcept
std::move
emplace_back
@@hint
`Legacy` is the same as `Tracked` but with `Legacy(Legacy&&) { ++moves; }` (no `noexcept`). A template helper `template <class T> void report(const char* label)` can print `T::copies` and `T::moves`. Use `reserve(4)` for the first three measurements so nothing reallocates.
@@solution
#include <iostream>
#include <vector>

struct Tracked {
    static inline int copies = 0, moves = 0;
    Tracked() = default;
    Tracked(const Tracked&) { ++copies; }
    Tracked(Tracked&&) noexcept { ++moves; }
    static void reset() { copies = moves = 0; }
};

struct Legacy {
    static inline int copies = 0, moves = 0;
    Legacy() = default;
    Legacy(const Legacy&) { ++copies; }
    Legacy(Legacy&&) { ++moves; }
    static void reset() { copies = moves = 0; }
};

template <class T>
void report(const char* label) {
    std::cout << label << ": copies=" << T::copies << " moves=" << T::moves << '\n';
    T::reset();
}

template <class T>
void grow(const char* label) {
    std::vector<T> v;
    v.reserve(1);
    v.emplace_back();
    T::reset();
    v.emplace_back();
    report<T>(label);
}

Tracked make() {
    Tracked t;
    return t;
}

int main() {
    std::vector<Tracked> v;
    v.reserve(4);
    Tracked x;
    v.push_back(x);
    report<Tracked>("push_back(lvalue)");
    v.push_back(std::move(x));
    report<Tracked>("push_back(std::move)");
    v.emplace_back();
    report<Tracked>("emplace_back");

    grow<Tracked>("grow with noexcept move");
    grow<Legacy>("grow without noexcept");

    Tracked r = make();
    (void)r;
    std::cout << "return local: copies=" << Tracked::copies << '\n';
}

@@exercise A perfect-forwarding factory
Write `template <class T, class... Args> T make_logged(Args&&... args)` that:

- prints `make(` followed by `lvalue` or `rvalue` for each argument (use `std::is_lvalue_reference_v<Args>` in a pack expansion), then `)`
- constructs `T` with `std::forward<Args>(args)...`

`Conn(std::string host, Tracked t)` takes its arguments by value and moves them into its members. Report the `Tracked` copies and moves for an lvalue call and an rvalue call, and show the lvalue string is still usable.
@@starter
#include <iostream>
#include <string>
#include <type_traits>
#include <utility>
#include <vector>

struct Tracked {
    static inline int copies = 0, moves = 0;
    Tracked() = default;
    Tracked(const Tracked&) { ++copies; }
    Tracked(Tracked&&) noexcept { ++moves; }
    static void reset() { copies = moves = 0; }
};

struct Conn {
    std::string host;
    Tracked tracked;
    Conn(std::string h, Tracked t) : host(std::move(h)), tracked(std::move(t)) {}
};

int main() {
    std::cout << "TODO\n";
}
@@expected
make(lvalue, lvalue): copies=1 moves=1
make(rvalue, rvalue): copies=0 moves=2
host still usable after lvalue call: web01
built: web01 db01
@@requires
std::forward<Args>(args)...
std::is_lvalue_reference_v<Args>
@@hint
`std::vector<std::string> kinds{(std::is_lvalue_reference_v<Args> ? "lvalue" : "rvalue")...};` then print them joined with `, `. For the lvalue call the `Tracked` argument is copied into the parameter, then moved into the member; for the rvalue call both steps are moves.
@@solution
#include <iostream>
#include <string>
#include <type_traits>
#include <utility>
#include <vector>

struct Tracked {
    static inline int copies = 0, moves = 0;
    Tracked() = default;
    Tracked(const Tracked&) { ++copies; }
    Tracked(Tracked&&) noexcept { ++moves; }
    static void reset() { copies = moves = 0; }
};

struct Conn {
    std::string host;
    Tracked tracked;
    Conn(std::string h, Tracked t) : host(std::move(h)), tracked(std::move(t)) {}
};

template <class T, class... Args>
T make_logged(Args&&... args) {
    std::vector<std::string> kinds{(std::is_lvalue_reference_v<Args> ? "lvalue" : "rvalue")...};
    std::cout << "make(";
    for (size_t i = 0; i < kinds.size(); ++i) std::cout << (i ? ", " : "") << kinds[i];
    std::cout << ")";
    return T(std::forward<Args>(args)...);
}

int main() {
    std::string host = "web01";
    Tracked t;
    Tracked::reset();
    Conn a = make_logged<Conn>(host, t);
    std::cout << ": copies=" << Tracked::copies << " moves=" << Tracked::moves << '\n';

    Tracked::reset();
    Conn b = make_logged<Conn>(std::string("db01"), Tracked{});
    std::cout << ": copies=" << Tracked::copies << " moves=" << Tracked::moves << '\n';

    std::cout << "host still usable after lvalue call: " << host << '\n';
    std::cout << "built: " << a.host << ' ' << b.host << '\n';
}
