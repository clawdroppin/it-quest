@@lesson n7-interop Interop: C APIs, Shared Libraries, Python and Rust
@@concepts modules, functions
@@text
C++ code rarely lives alone. It's called from Python (NumPy, PyTorch, OpenCV), C#, Go, Rust and plugins, and it calls into C libraries and the OS. The common language between all of them is the **C ABI**.

## Why C?

C++ has no stable ABI: name mangling, class layouts, exceptions and the standard library differ between compilers and even versions. C functions have simple, stable calling conventions, so every language can call them. The standard pattern is a **C++ library behind a C API**:

```cpp,norun
// inventory.h: usable from C, C#, Python ctypes, Rust, Go cgo...
#ifdef __cplusplus
extern "C" {                                   // no name mangling
#endif

typedef struct inv_db inv_db;                  // opaque handle: callers never see the C++ class
typedef enum { INV_OK, INV_NOT_FOUND, INV_BUFFER_TOO_SMALL, INV_INVALID_ARG, INV_INTERNAL } inv_status;

inv_db* inv_open(void);
void inv_close(inv_db* db);                     // the library allocates AND frees its objects
inv_status inv_add(inv_db* db, const char* host, int cores);
inv_status inv_list(const inv_db* db, char* buf, size_t* len);   // caller-provided buffer

#ifdef __cplusplus
}
#endif
```

Rules for a C boundary:

- **no exceptions across it**: catch everything in each function and return an error code
- opaque pointers instead of classes; plain structs, integers and `const char*` (UTF-8) in signatures
- whoever allocates frees: provide `*_free`/`*_close` functions, because the caller may use a different heap (very real on Windows with different CRTs)
- for output strings, let the caller pass a buffer and its size, and report the required size when it's too small
- callbacks take a `void* user` pointer so callers can pass context

## Exporting from shared libraries

On Windows, mark exported functions with `__declspec(dllexport)` (and `dllimport` for users); on Linux/macOS, compile with `-fvisibility=hidden` and mark exports with `__attribute__((visibility("default")))`. CMake's `GenerateExportHeader` writes these macros for you.

## Language bindings

- **Python**: pybind11 or nanobind wrap C++ classes directly (`py::class_<Inventory>(m, "Inventory").def("add", &Inventory::add)`); ctypes or cffi call a C API with no compilation step.
- **Rust**: the `cxx` crate for safe bidirectional C++ interop, or `bindgen` for C headers.
- **C#**: P/Invoke (`[LibraryImport("inventory")]`) on the C API.
- **Go**: cgo on the C API.

@@quiz
Why put a C API in front of a C++ library?
* The C ABI is stable and callable from nearly every language; C++ ABIs differ between compilers
- C is faster than C++
- C++ functions can't be exported
- To avoid using classes internally
? The C++ inside stays free to change.

@@quiz
What must every function of a C API built in C++ do with exceptions?
- Let them propagate to the caller
* Catch them all and convert them to error codes
- Rethrow them as C exceptions
- Nothing: C handles them
? An exception crossing a C boundary is undefined behaviour.

@@quiz
Why does a C API take a `void* user` argument next to a callback?
- For performance
* So the caller can pass its own context (like a C++ object) to the callback
- It's required by the C standard
- To return errors
? The callback casts it back to the real type.

@@flashcards
`extern "C"` :: C linkage: no name mangling
Opaque handle :: pointer to an undefined struct type
Error codes at the boundary :: no exceptions cross a C API
Caller-provided buffer :: buf + size, report the needed size
`void* user` :: callback context
`__declspec(dllexport)` :: export from a Windows DLL
`-fvisibility=hidden` :: hide symbols by default
pybind11 / nanobind :: Python bindings for C++
`cxx` crate :: Rust/C++ interop
P/Invoke :: call C from C#

@@exercise A C API around a C++ class
Implement the inventory C API (the header is in the lesson) on top of a C++ class with a `std::map<std::string, int>`:

- `inv_open` / `inv_close` create and destroy the C++ object behind the opaque `inv_db`
- `inv_add` rejects a null `db` or host and cores ≤ 0 with `INV_INVALID_ARG`
- `inv_get_cores(db, host, &out)` returns `INV_NOT_FOUND` for unknown hosts
- `inv_list(db, buf, &len)` writes `host,host,...` (sorted) plus a terminating `\0`; if `buf` is null or `*len` is too small, set `*len` to the required size and return `INV_BUFFER_TOO_SMALL`
- every function catches all exceptions and returns `INV_INTERNAL`
- `inv_status_str` returns `ok`, `not found`, `buffer too small`, `invalid argument` or `internal error`

Then use it from `main` the way a C program would.
@@starter
#include <cstddef>
#include <cstring>
#include <iostream>
#include <map>
#include <string>
#include <vector>

extern "C" {
typedef struct inv_db inv_db;
typedef enum { INV_OK, INV_NOT_FOUND, INV_BUFFER_TOO_SMALL, INV_INVALID_ARG, INV_INTERNAL } inv_status;
}

int main() {
    std::cout << "TODO\n";
}
@@expected
add web01: ok
add db01: ok
add null host: invalid argument
cores of db01: 16
cores of mail01: not found
list into 4 bytes: buffer too small, needs 11
list: db01,web01
closed
@@requires
extern "C"
catch (...)
INV_BUFFER_TOO_SMALL
@@hint
Define `struct inv_db { std::map<std::string, int> hosts; };` and create it with `new inv_db` inside `inv_open` (inside a try block). In `inv_list`, build the string first, compute `required = s.size() + 1`, compare with `*len`, then `std::memcpy(buf, s.c_str(), required)`. To query the size, call `inv_list(db, nullptr, &need)` first.
@@solution
#include <cstddef>
#include <cstring>
#include <iostream>
#include <map>
#include <string>
#include <vector>

extern "C" {
typedef struct inv_db inv_db;
typedef enum { INV_OK, INV_NOT_FOUND, INV_BUFFER_TOO_SMALL, INV_INVALID_ARG, INV_INTERNAL } inv_status;
}

struct inv_db {
    std::map<std::string, int> hosts;
};

extern "C" {

inv_db* inv_open(void) {
    try {
        return new inv_db;
    } catch (...) {
        return nullptr;
    }
}

void inv_close(inv_db* db) { delete db; }

inv_status inv_add(inv_db* db, const char* host, int cores) {
    if (!db || !host || cores <= 0) return INV_INVALID_ARG;
    try {
        db->hosts[host] = cores;
        return INV_OK;
    } catch (...) {
        return INV_INTERNAL;
    }
}

inv_status inv_get_cores(const inv_db* db, const char* host, int* out) {
    if (!db || !host || !out) return INV_INVALID_ARG;
    try {
        auto it = db->hosts.find(host);
        if (it == db->hosts.end()) return INV_NOT_FOUND;
        *out = it->second;
        return INV_OK;
    } catch (...) {
        return INV_INTERNAL;
    }
}

inv_status inv_list(const inv_db* db, char* buf, size_t* len) {
    if (!db || !len) return INV_INVALID_ARG;
    try {
        std::string s;
        for (const auto& [host, cores] : db->hosts) s += (s.empty() ? "" : ",") + host;
        size_t required = s.size() + 1;
        if (!buf || *len < required) {
            *len = required;
            return INV_BUFFER_TOO_SMALL;
        }
        std::memcpy(buf, s.c_str(), required);
        *len = required;
        return INV_OK;
    } catch (...) {
        return INV_INTERNAL;
    }
}

const char* inv_status_str(inv_status s) {
    switch (s) {
    case INV_OK: return "ok";
    case INV_NOT_FOUND: return "not found";
    case INV_BUFFER_TOO_SMALL: return "buffer too small";
    case INV_INVALID_ARG: return "invalid argument";
    default: return "internal error";
    }
}

}  // extern "C"

int main() {
    inv_db* db = inv_open();
    std::cout << "add web01: " << inv_status_str(inv_add(db, "web01", 4)) << '\n';
    std::cout << "add db01: " << inv_status_str(inv_add(db, "db01", 16)) << '\n';
    std::cout << "add null host: " << inv_status_str(inv_add(db, nullptr, 2)) << '\n';

    int cores = 0;
    if (inv_get_cores(db, "db01", &cores) == INV_OK) std::cout << "cores of db01: " << cores << '\n';
    std::cout << "cores of mail01: " << inv_status_str(inv_get_cores(db, "mail01", &cores)) << '\n';

    char small[4];
    size_t len = sizeof small;
    inv_status s = inv_list(db, small, &len);
    std::cout << "list into 4 bytes: " << inv_status_str(s) << ", needs " << len << '\n';

    size_t need = 0;
    inv_list(db, nullptr, &need);
    std::vector<char> buf(need);
    if (inv_list(db, buf.data(), &need) == INV_OK) std::cout << "list: " << buf.data() << '\n';

    inv_close(db);
    std::cout << "closed\n";
}

@@exercise C callbacks with user data
A C library offers `void lib_visit(const host_rec* recs, size_t n, host_cb cb, void* user)`, which calls `cb` for each record until it returns non-zero. Use it from modern C++:

- sort the records by cores with `std::qsort` and a C-compatible comparator function
- write `template <class F> void visit_hosts(const host_rec* recs, size_t n, F&& f)` that passes **any** callable (including capturing lambdas) through `lib_visit`, using a captureless lambda as the trampoline and `&f` as the `void* user`
- use it to sum the cores, and to find the first host with at least 8 cores (stopping early)
@@starter
#include <cstddef>
#include <cstdlib>
#include <iostream>
#include <string>
#include <type_traits>

extern "C" {
struct host_rec {
    const char* name;
    int cores;
};
typedef int (*host_cb)(const struct host_rec* rec, void* user);

void lib_visit(const struct host_rec* recs, size_t n, host_cb cb, void* user) {
    for (size_t i = 0; i < n; ++i)
        if (cb(&recs[i], user) != 0) return;
}
}

int main() {
    host_rec hosts[] = {{"db01", 16}, {"web01", 4}, {"mail01", 8}, {"web02", 2}};
    std::cout << "TODO\n";
}
@@expected
sorted by cores: web02(2) web01(4) mail01(8) db01(16)
total cores: 30
first host with >= 8 cores: mail01 (stopped after 3 visits)
@@requires
std::qsort
void* user
static_cast<Fn*>(user)
@@hint
Comparator: `int by_cores(const void* a, const void* b) { int x = static_cast<const host_rec*>(a)->cores, y = static_cast<const host_rec*>(b)->cores; return (x > y) - (x < y); }`. Trampoline: `using Fn = std::remove_reference_t<F>; lib_visit(recs, n, [](const host_rec* r, void* user) -> int { return (*static_cast<Fn*>(user))(*r); }, &f);`. The callables return `int` (0 = continue).
@@solution
#include <cstddef>
#include <cstdlib>
#include <iostream>
#include <string>
#include <type_traits>

extern "C" {
struct host_rec {
    const char* name;
    int cores;
};
typedef int (*host_cb)(const struct host_rec* rec, void* user);

void lib_visit(const struct host_rec* recs, size_t n, host_cb cb, void* user) {
    for (size_t i = 0; i < n; ++i)
        if (cb(&recs[i], user) != 0) return;
}
}

int by_cores(const void* a, const void* b) {
    int x = static_cast<const host_rec*>(a)->cores;
    int y = static_cast<const host_rec*>(b)->cores;
    return (x > y) - (x < y);
}

template <class F>
void visit_hosts(const host_rec* recs, size_t n, F&& f) {
    using Fn = std::remove_reference_t<F>;
    lib_visit(recs, n, [](const host_rec* r, void* user) -> int { return (*static_cast<Fn*>(user))(*r); }, &f);
}

int main() {
    host_rec hosts[] = {{"db01", 16}, {"web01", 4}, {"mail01", 8}, {"web02", 2}};
    const size_t n = sizeof hosts / sizeof hosts[0];
    std::qsort(hosts, n, sizeof(host_rec), by_cores);
    std::cout << "sorted by cores:";
    for (const auto& h : hosts) std::cout << ' ' << h.name << '(' << h.cores << ')';
    std::cout << '\n';

    int total = 0;
    visit_hosts(hosts, n, [&](const host_rec& h) {
        total += h.cores;
        return 0;
    });
    std::cout << "total cores: " << total << '\n';

    std::string found;
    int visits = 0;
    visit_hosts(hosts, n, [&](const host_rec& h) {
        ++visits;
        if (h.cores >= 8) {
            found = h.name;
            return 1;
        }
        return 0;
    });
    std::cout << "first host with >= 8 cores: " << found << " (stopped after " << visits << " visits)\n";
}
