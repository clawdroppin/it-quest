@@lesson x3-memory Memory, Pointers and Smart Pointers
@@text
## Stack and heap

- **Stack**: local variables. Very fast; freed automatically when the function returns. Limited size (about 1 MB on Windows, 8 MB on Linux).
- **Heap** (free store): memory you ask for at run time, for big or long-lived data. It has to be released when you're done.

`std::vector` and `std::string` keep a small object on the stack, which owns a heap buffer and frees it in its destructor. Most of the time, that's all the heap management you need.

## Pointers

A pointer holds a memory address. `&x` takes the address of `x`; `*p` follows the pointer.

```cpp
#include <iostream>

int main() {
    int port = 443;
    int* p = &port;          // p points at port
    std::cout << *p << '\n'; // 443
    *p = 8443;               // change port through the pointer
    std::cout << port << '\n';

    int* nothing = nullptr;  // points nowhere
    if (nothing) std::cout << *nothing;   // always check before following
    std::cout << (nothing == nullptr) << '\n';
}
```

References (`int& r = port;`) are like pointers that can't be null and can't be re-pointed. Prefer references; use pointers when "nothing" is a valid value, or for C APIs.

## The classic memory bugs

These are why C and C++ have so many security holes (most CVEs in Windows and Chrome are memory-safety bugs):

| Bug | What happens |
|---|---|
| **Use after free** | using memory after it's been released |
| **Dangling pointer/reference** | pointing at a local variable that no longer exists |
| **Buffer overflow** | writing past the end of an array: the classic exploit |
| **Memory leak** | `new` without `delete`: memory is never returned |
| **Double free** | releasing the same memory twice |

```cpp,norun
int* danger() {
    int local = 42;
    return &local;           // local dies when the function returns: dangling!
}

char buf[8];
std::strcpy(buf, "this is much longer than 8 bytes");   // buffer overflow
```

Tools that find them: compile with `-fsanitize=address,undefined` (g++/clang) or `/fsanitize=address` (MSVC), and run **Valgrind** on Linux. Use them in your projects.

## Smart pointers: ownership in the type

Modern C++ almost never uses raw `new` and `delete`. Smart pointers from `<memory>` free the object automatically (RAII):

```cpp
#include <iostream>
#include <memory>
#include <string>

struct Connection {
    std::string host;
    explicit Connection(std::string h) : host(std::move(h)) { std::cout << "open " << host << '\n'; }
    ~Connection() { std::cout << "close " << host << '\n'; }
};

int main() {
    // unique_ptr: exactly one owner. Can be moved, not copied.
    auto a = std::make_unique<Connection>("db01");
    auto b = std::move(a);                    // ownership moves to b; a is now empty
    std::cout << (a == nullptr) << ' ' << b->host << '\n';

    // shared_ptr: several owners; freed when the last one goes away.
    auto s1 = std::make_shared<Connection>("cache01");
    {
        auto s2 = s1;
        std::cout << "owners: " << s1.use_count() << '\n';   // 2
    }
    std::cout << "owners: " << s1.use_count() << '\n';       // 1
}   // both connections close here, automatically
```

The rules modern C++ code follows:

1. Prefer values and containers (`std::vector<Server>`): no pointers at all.
2. Need a heap object with one owner? `std::unique_ptr` via `std::make_unique`.
3. Truly shared ownership? `std::shared_ptr` via `std::make_shared` (it costs more, so use it only when needed).
4. A raw pointer `T*` or reference `T&` means "I'm *using* this, I don't own it".
5. No naked `new`/`delete`.

## Move semantics

Copying a big vector copies every element. **Moving** it just hands over the heap buffer:

```cpp
#include <iostream>
#include <string>
#include <utility>
#include <vector>

int main() {
    std::vector<std::string> logs(100000, "a log line");
    std::vector<std::string> archive = std::move(logs);   // cheap: no 100000 copies
    std::cout << archive.size() << ' ' << logs.size() << '\n';   // 100000 0 (logs is left empty)
}
```

After a move, the source is valid but unspecified (usually empty): don't use it except to assign a new value. Returning a local vector from a function is automatically a move (or even free), so don't be afraid to return big objects by value.

@@quiz
What does `int* p = &x;` store in `p`?
* The memory address of `x`
- A copy of `x`
- The value of `x` times two
- A reference count
? `&` takes the address. `*p` reads the value at that address.

@@quiz
Which smart pointer should you use by default for one owner?
* `std::unique_ptr` (made with `std::make_unique`)
- `std::shared_ptr`
- a raw pointer with `new`
- `std::weak_ptr`
? `unique_ptr` costs nothing extra over a raw pointer and frees the object automatically.

@@quiz
A function returns `&local` where `local` is a variable inside it. What's the problem?
* The pointer dangles: `local` no longer exists after the return
- Nothing, the value is copied
- A compile error, always
- A memory leak
? Compilers usually warn about this. Reading through the pointer is undefined behaviour.

@@quiz
Why are memory-safety bugs so important in security?
* Buffer overflows and use-after-free let attackers corrupt memory and run their own code
- They only make programs slower
- They only affect games
- They can't be exploited, only crash
? A large share of serious CVEs in big C/C++ projects are memory-safety bugs. That's why Rust, Go and C# are popular for new security-sensitive code.

@@flashcards
`&x` :: the address of x
`*p` :: the value p points at
`nullptr` :: a pointer to nothing
`std::make_unique<T>(...)` :: create a heap object with one owner
`std::make_shared<T>(...)` :: create a heap object with shared, counted ownership
`std::move(x)` :: allow x's contents to be moved instead of copied
`-fsanitize=address` :: compile with AddressSanitizer to catch memory bugs
use after free :: using memory after it's been released

@@exercise Swap with pointers
Write `void swapPorts(int* a, int* b)` that swaps the two values through the pointers. If either pointer is `nullptr`, do nothing.
@@starter
#include <iostream>

void swapPorts(int* a, int* b) {
    // TODO: not done yet
}

int main() {
    int x = 80, y = 443;
    swapPorts(&x, &y);
    std::cout << x << ' ' << y << '\n';
    swapPorts(&x, nullptr);
    std::cout << x << ' ' << y << '\n';
}
@@expected
443 80
443 80
@@requires
*a
@@hint
`if (!a || !b) return; int tmp = *a; *a = *b; *b = tmp;`
@@solution
#include <iostream>

void swapPorts(int* a, int* b) {
    if (!a || !b) return;
    int tmp = *a;
    *a = *b;
    *b = tmp;
}

int main() {
    int x = 80, y = 443;
    swapPorts(&x, &y);
    std::cout << x << ' ' << y << '\n';
    swapPorts(&x, nullptr);
    std::cout << x << ' ' << y << '\n';
}

@@exercise Connection pool
Fill the `pool` vector with three `std::unique_ptr<Conn>` (for hosts `db01`, `db02`, `db03`) using `std::make_unique`. Then **move** the second one out of the pool into `active`. The destructors print `close <host>`; the output must be exactly:

```text
active: db02
pool[1] empty: 1
close db02
close db01
close db03
```
@@starter
#include <iostream>
#include <memory>
#include <string>
#include <vector>

struct Conn {
    std::string host;
    explicit Conn(std::string h) : host(std::move(h)) {}
    ~Conn() { std::cout << "close " << host << '\n'; }
};

int main() {
    std::vector<std::unique_ptr<Conn>> pool;
    // add three connections

    std::unique_ptr<Conn> active; // move pool[1] into this
    // ...
    std::cout << "active: " << active->host << '\n';
    std::cout << "pool[1] empty: " << (pool[1] == nullptr) << '\n';
}
@@expected
active: db02
pool[1] empty: 1
close db02
close db01
close db03
@@requires
std::make_unique
std::move
@@hint
`pool.push_back(std::make_unique<Conn>("db01"));` three times, then `active = std::move(pool[1]);`. Locals are destroyed in reverse order of creation: `active` (declared last) goes first, then the vector, which destroys its elements in order.
@@solution
#include <iostream>
#include <memory>
#include <string>
#include <vector>

struct Conn {
    std::string host;
    explicit Conn(std::string h) : host(std::move(h)) {}
    ~Conn() { std::cout << "close " << host << '\n'; }
};

int main() {
    std::vector<std::unique_ptr<Conn>> pool;
    for (const char* h : {"db01", "db02", "db03"}) pool.push_back(std::make_unique<Conn>(h));

    std::unique_ptr<Conn> active = std::move(pool[1]);
    std::cout << "active: " << active->host << '\n';
    std::cout << "pool[1] empty: " << (pool[1] == nullptr) << '\n';
}
