@@lesson n7-memory Performance in Depth: Allocators, Caches and Data-Oriented Design
@@concepts lists, objects
@@text
Past the basics (good algorithms, no needless copies), C++ performance is mostly about **memory**: how often you allocate, and how your data is laid out for the CPU cache.

## The memory hierarchy

| Where | Rough latency |
|---|---|
| L1 cache | ~1 ns |
| L2 cache | ~4 ns |
| L3 cache | ~15 ns |
| main memory | ~80–100 ns |

The CPU loads memory in 64-byte **cache lines** and prefetches sequential access. Walking a `vector` is fast; chasing pointers through a `list` or a map of heap nodes is slow because each step may be a cache miss.

## Allocation costs

Every `new` is a call into a general-purpose allocator: locks or thread caches, bookkeeping, fragmentation. Hot code avoids it with:

- `reserve` on vectors whose size you know
- object pools and free lists (reuse instead of free)
- **arenas**: allocate from a big block by bumping a pointer, free everything at once
- small-buffer optimisation (`std::string` stores short strings inline; `boost::small_vector`)

C++17's **polymorphic memory resources** (`<memory_resource>`) make arenas easy for standard containers:

```cpp,norun
std::array<std::byte, 64 * 1024> buffer;
std::pmr::monotonic_buffer_resource arena(buffer.data(), buffer.size());
std::pmr::vector<std::pmr::string> names(&arena);   // vector AND its strings allocate from the arena
```

Per-request arenas (allocate while handling one request, release in one go) are a classic server and compiler technique.

## Data-oriented design

**Array of structs (AoS)** stores each object's fields together. **Struct of arrays (SoA)** stores each field in its own array. If a hot loop only touches two of eight fields, SoA reads only the bytes it needs, and the compiler can vectorise it with SIMD. Game engines (ECS), databases (columnar storage) and analytics engines are built on this idea.

Also watch for **false sharing**: two threads writing different variables on the same cache line slow each other down. Pad hot per-thread data to `std::hardware_destructive_interference_size` (usually 64).

## Measuring

- `perf stat` / `perf record` (Linux), VTune, AMD uProf, Visual Studio Profiler, Tracy for frame-based apps
- cache-miss counters, not just time
- heaptrack or the Visual Studio heap profiler for allocations
- Google Benchmark with `benchmark::DoNotOptimize`

@@quiz
Why is iterating a std::list usually slower than a std::vector of the same size?
* The list's nodes are scattered in memory, so each step may be a cache miss
- Lists use more CPU instructions per element
- Lists are always sorted
- Vectors use SIMD automatically
? Contiguous memory makes prefetching work.

@@quiz
What does a monotonic (arena) allocator do on deallocate?
- Returns memory to the OS
* Nothing: everything is released at once when the arena is destroyed or reset
- Compacts memory
- Throws
? That's why it's so fast.

@@quiz
When does struct-of-arrays beat array-of-structs?
- Always
* When hot loops touch only a few fields of many objects
- When objects have one field
- Never; it's just style
? Fewer bytes per element means fewer cache lines loaded.

@@flashcards
Cache line :: 64 bytes loaded at once
Cache miss :: data not in cache: ~100 ns trip to RAM
`reserve` :: allocate once up front
Arena / monotonic allocator :: bump pointer, free all at once
`std::pmr::memory_resource` :: pluggable allocator interface
`std::pmr::monotonic_buffer_resource` :: arena resource
AoS :: array of structs
SoA :: struct of arrays
False sharing :: threads contending on one cache line
SSO :: small string optimisation

@@exercise Count allocations with a pmr resource
Write `CountingResource`, a `std::pmr::memory_resource` that counts allocations and forwards them to an upstream resource. Then build 1,000 host names (`server-node-0000.example`, longer than any small-string buffer) in a `std::pmr::vector<std::pmr::string>` with `reserve(1000)`:

1. with the counter on top of `std::pmr::new_delete_resource()`: every string and the vector allocate from the heap
2. with a `std::pmr::monotonic_buffer_resource` over a 256 KiB static buffer, whose upstream is a counter over `std::pmr::null_memory_resource()` (which throws if used): zero upstream allocations
@@starter
#include <array>
#include <cstddef>
#include <format>
#include <iostream>
#include <memory_resource>
#include <string>
#include <vector>

int main() {
    std::cout << "TODO\n";
}
@@expected
default heap via pmr: 1001 allocations
arena: 0 upstream allocations for 1000 strings
first: server-node-0000.example, last: server-node-0999.example
@@requires
std::pmr::memory_resource
std::pmr::monotonic_buffer_resource
do_allocate
@@hint
Override `do_allocate(size_t bytes, size_t align)`, `do_deallocate(void*, size_t, size_t)` and `do_is_equal(const memory_resource& other) const noexcept`. `emplace_back(std::format("server-node-{:04}.example", i))` builds a `std::pmr::string` from the `std::string`, and the vector passes its allocator to it automatically.
@@solution
#include <array>
#include <cstddef>
#include <format>
#include <iostream>
#include <memory_resource>
#include <string>
#include <vector>

class CountingResource : public std::pmr::memory_resource {
public:
    explicit CountingResource(std::pmr::memory_resource* upstream) : upstream_(upstream) {}
    size_t allocations = 0;

private:
    void* do_allocate(size_t bytes, size_t align) override {
        ++allocations;
        return upstream_->allocate(bytes, align);
    }
    void do_deallocate(void* p, size_t bytes, size_t align) override { upstream_->deallocate(p, bytes, align); }
    bool do_is_equal(const std::pmr::memory_resource& other) const noexcept override { return this == &other; }

    std::pmr::memory_resource* upstream_;
};

std::pmr::vector<std::pmr::string> build(std::pmr::memory_resource* resource) {
    std::pmr::vector<std::pmr::string> names(resource);
    names.reserve(1000);
    for (int i = 0; i < 1000; ++i) names.emplace_back(std::format("server-node-{:04}.example", i));
    return names;
}

static std::array<std::byte, 256 * 1024> buffer;

int main() {
    CountingResource heap(std::pmr::new_delete_resource());
    {
        auto names = build(&heap);
        std::cout << "default heap via pmr: " << heap.allocations << " allocations\n";
    }

    CountingResource nothing(std::pmr::null_memory_resource());
    std::pmr::monotonic_buffer_resource arena(buffer.data(), buffer.size(), &nothing);
    auto names = build(&arena);
    std::cout << "arena: " << nothing.allocations << " upstream allocations for " << names.size() << " strings\n";
    std::cout << "first: " << names.front() << ", last: " << names.back() << '\n';
}

@@exercise Array of structs vs struct of arrays
Simulate 10,000 particles moving along x for 100 steps (`x += vx * dt`, `dt = 0.016f`) in both layouts:

- AoS: `struct Particle { float x, y, z, vx, vy, vz, mass; int id; };` in a vector
- SoA: `struct Particles { std::vector<float> x, y, z, vx, vy, vz, mass; std::vector<int> id; };`
- initial values: `x = i * 0.5f`, `vx = (i % 10) * 0.1f`, others 0, `id = i`

Print the bytes each pass has to read (AoS: whole structs; SoA: just the `x` and `vx` arrays), check both layouts end with identical positions, and count the moving particles. (Timings vary per machine, so they're not printed; try them with a benchmark at home.)
@@starter
#include <iostream>
#include <vector>

int main() {
    std::cout << "TODO\n";
}
@@expected
sizeof(Particle): 32 bytes
AoS pass reads 320000 bytes for 10000 particles
SoA pass reads 80000 bytes (x and vx only)
same positions: true
moving particles: 9000
@@requires
std::vector<float> x
@@hint
The AoS loop is `for (auto& p : aos) p.x += p.vx * dt;`; the SoA loop is `for (size_t i = 0; i < n; ++i) soa.x[i] += soa.vx[i] * dt;`. Both do the same float operations in the same order, so the results are bit-identical. Bytes per pass: `n * sizeof(Particle)` and `n * 2 * sizeof(float)`.
@@solution
#include <iostream>
#include <vector>

struct Particle {
    float x, y, z, vx, vy, vz, mass;
    int id;
};

struct Particles {
    std::vector<float> x, y, z, vx, vy, vz, mass;
    std::vector<int> id;
};

int main() {
    const size_t n = 10000;
    const float dt = 0.016f;

    std::vector<Particle> aos(n);
    Particles soa;
    for (auto* v : {&soa.x, &soa.y, &soa.z, &soa.vx, &soa.vy, &soa.vz, &soa.mass}) v->assign(n, 0.0f);
    soa.id.resize(n);
    for (size_t i = 0; i < n; ++i) {
        float x = static_cast<float>(i) * 0.5f, vx = static_cast<float>(i % 10) * 0.1f;
        aos[i] = Particle{x, 0, 0, vx, 0, 0, 0, static_cast<int>(i)};
        soa.x[i] = x;
        soa.vx[i] = vx;
        soa.id[i] = static_cast<int>(i);
    }

    for (int step = 0; step < 100; ++step) {
        for (auto& p : aos) p.x += p.vx * dt;
        for (size_t i = 0; i < n; ++i) soa.x[i] += soa.vx[i] * dt;
    }

    bool same = true;
    size_t moving = 0;
    for (size_t i = 0; i < n; ++i) {
        if (aos[i].x != soa.x[i]) same = false;
        if (soa.vx[i] > 0) ++moving;
    }
    std::cout << "sizeof(Particle): " << sizeof(Particle) << " bytes\n";
    std::cout << "AoS pass reads " << n * sizeof(Particle) << " bytes for " << n << " particles\n";
    std::cout << "SoA pass reads " << n * 2 * sizeof(float) << " bytes (x and vx only)\n";
    std::cout << std::boolalpha << "same positions: " << same << '\n';
    std::cout << "moving particles: " << moving << '\n';
}
