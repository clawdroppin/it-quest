@@lesson x2-stl-deep The Standard Library in Depth: Choosing Containers
@@concepts lists, maps
@@text
The STL has a container for every job. Picking the right one is often the biggest performance decision you make.

| Container | Good at | Watch out |
|---|---|---|
| `vector` | the default: contiguous, cache-friendly, fast iteration and push_back | inserting in the middle is O(n); reallocation invalidates pointers |
| `deque` | push/pop at both ends | not contiguous |
| `list` / `forward_list` | splicing, stable iterators | slow iteration (pointer chasing) |
| `map` / `set` | sorted keys, range queries (`lower_bound`) | O(log n), node allocations |
| `unordered_map` / `unordered_set` | O(1) average lookup | no order; needs a hash for custom keys |
| `multimap` / `multiset` | duplicate keys | |
| `priority_queue` | always pop the largest | no iteration |
| `array` | fixed size, on the stack | size is compile-time |

When in doubt, use `vector`. Even for lookups, a sorted vector with `std::lower_bound` often beats `map` for small, mostly read-only data.

## Custom keys

`std::map` needs `operator<`; `unordered_map` needs `operator==` and a hash:

```cpp,norun
struct Endpoint {
    std::string host;
    uint16_t port;
    bool operator==(const Endpoint&) const = default;      // C++20: compiler-written ==
    auto operator<=>(const Endpoint&) const = default;     // and <, >, <=, >= for std::map
};

struct EndpointHash {
    size_t operator()(const Endpoint& e) const {
        return std::hash<std::string>{}(e.host) ^ (std::hash<uint16_t>{}(e.port) << 1);
    }
};

std::unordered_map<Endpoint, int, EndpointHash> connections;
```

## Iterator invalidation

Modifying a container can invalidate iterators, pointers and references into it:

- `vector`: any reallocation invalidates everything; `erase` invalidates from the erased point on
- `unordered_map`: rehashing invalidates iterators (but not references to elements)
- `map`/`list`: only the erased element

Erase while iterating with the returned iterator, or use C++20's `std::erase_if(container, pred)`.

## Algorithms you should know

```cpp
#include <algorithm>
#include <iostream>
#include <numeric>
#include <vector>

int main() {
    std::vector<int> v = {5, 1, 4, 2, 3, 9, 7};
    std::ranges::sort(v);
    std::cout << std::ranges::binary_search(v, 4) << ' '                 // 1
              << *std::ranges::lower_bound(v, 6) << ' '                  // 7: first >= 6
              << std::accumulate(v.begin(), v.end(), 0) << ' '           // 31
              << std::ranges::count_if(v, [](int x) { return x % 2; }) << '\n';  // 5
    std::erase_if(v, [](int x) { return x > 4; });
    std::cout << v.size() << '\n';                                      // 4
}
```

Also worth knowing: `partition`, `nth_element` (top-k without a full sort), `unique` after sorting, `transform`, `minmax_element`, `equal_range`, `set_intersection`.

@@quiz
Which container fits "always process the most urgent job next"?
- vector
- map
* priority_queue
- list
? It keeps the largest element on top.

@@quiz
What does `unordered_map` need for a custom key type?
- Only operator<
* operator== and a hash function
- A default constructor only
- Nothing
? `map` needs operator< instead.

@@quiz
Why might a sorted `vector` beat a `map` for lookups?
* Contiguous memory is cache-friendly, and binary search on it is fast; map nodes are scattered in memory
- vector lookups are O(1)
- map can't be searched
- vectors are always smaller
? Measure, but don't assume map is faster.

@@flashcards
`std::vector` :: default container, contiguous
`std::deque` :: fast at both ends
`std::map` :: sorted, O(log n)
`std::unordered_map` :: hashed, O(1) average
`std::priority_queue` :: max-heap
`= default` comparisons :: compiler-generated == and <=>
Iterator invalidation :: modification breaks iterators
`std::erase_if` :: remove matching elements (C++20)
`std::lower_bound` :: first element >= value
`std::nth_element` :: partial sort for top-k

@@exercise Connection counts with a custom hash
Count connections per endpoint with `std::unordered_map<Endpoint, int, EndpointHash>`:

- parse each `host:port` (split at the last `:`, port with `std::from_chars`)
- `Endpoint` uses a defaulted `operator==`; write `EndpointHash`
- unordered_map has no order, so copy the entries into a vector and sort: count descending, then host, then port
@@starter
#include <algorithm>
#include <charconv>
#include <cstdint>
#include <iostream>
#include <string>
#include <unordered_map>
#include <vector>

int main() {
    std::vector<std::string> log = {"10.0.0.5:443", "10.0.0.5:22", "10.0.0.7:443", "10.0.0.5:443",
                                    "10.0.0.7:443", "10.0.0.5:443", "10.0.0.9:3389"};
    std::cout << "TODO\n";
}
@@expected
10.0.0.5:443 3
10.0.0.7:443 2
10.0.0.5:22 1
10.0.0.9:3389 1
distinct endpoints: 4
@@requires
EndpointHash
operator==
@@hint
Sort a `std::vector<std::pair<Endpoint, int>>` with a lambda: `if (a.second != b.second) return a.second > b.second; if (a.first.host != b.first.host) return a.first.host < b.first.host; return a.first.port < b.first.port;`.
@@solution
#include <algorithm>
#include <charconv>
#include <cstdint>
#include <iostream>
#include <string>
#include <unordered_map>
#include <vector>

struct Endpoint {
    std::string host;
    uint16_t port = 0;
    bool operator==(const Endpoint&) const = default;
};

struct EndpointHash {
    size_t operator()(const Endpoint& e) const {
        return std::hash<std::string>{}(e.host) ^ (std::hash<uint16_t>{}(e.port) << 1);
    }
};

Endpoint parse(const std::string& s) {
    auto colon = s.rfind(':');
    Endpoint e{s.substr(0, colon)};
    std::from_chars(s.data() + colon + 1, s.data() + s.size(), e.port);
    return e;
}

int main() {
    std::vector<std::string> log = {"10.0.0.5:443", "10.0.0.5:22", "10.0.0.7:443", "10.0.0.5:443",
                                    "10.0.0.7:443", "10.0.0.5:443", "10.0.0.9:3389"};
    std::unordered_map<Endpoint, int, EndpointHash> counts;
    for (const auto& line : log) ++counts[parse(line)];

    std::vector<std::pair<Endpoint, int>> sorted(counts.begin(), counts.end());
    std::sort(sorted.begin(), sorted.end(), [](const auto& a, const auto& b) {
        if (a.second != b.second) return a.second > b.second;
        if (a.first.host != b.first.host) return a.first.host < b.first.host;
        return a.first.port < b.first.port;
    });
    for (const auto& [e, n] : sorted) std::cout << e.host << ':' << e.port << ' ' << n << '\n';
    std::cout << "distinct endpoints: " << counts.size() << '\n';
}

@@exercise A job queue with priorities
Use `std::priority_queue` to run jobs by priority:

- `struct Job { std::string name; int priority; int seq; }`, where `seq` is the submission order
- higher priority runs first; equal priorities run in submission order (FIFO)
- write the comparator `struct Later` so the queue's top is the job to run next (a priority_queue puts the **largest** element according to the comparator on top)
@@starter
#include <iostream>
#include <queue>
#include <string>
#include <vector>

struct Job {
    std::string name;
    int priority;
    int seq;
};

int main() {
    std::vector<std::pair<std::string, int>> submitted = {
        {"backup", 1}, {"patch", 5}, {"report", 1}, {"alert", 9}, {"rotate-logs", 5}, {"cleanup", 1}};
    std::cout << "TODO\n";
}
@@expected
queued: 6, next: alert
run alert (priority 9)
run patch (priority 5)
run rotate-logs (priority 5)
run backup (priority 1)
run report (priority 1)
run cleanup (priority 1)
@@requires
std::priority_queue<Job, std::vector<Job>, Later>
@@hint
`Later` returns true when `a` should run **after** `b`: `if (a.priority != b.priority) return a.priority < b.priority; return a.seq > b.seq;`.
@@solution
#include <iostream>
#include <queue>
#include <string>
#include <vector>

struct Job {
    std::string name;
    int priority;
    int seq;
};

struct Later {
    bool operator()(const Job& a, const Job& b) const {
        if (a.priority != b.priority) return a.priority < b.priority;
        return a.seq > b.seq;
    }
};

int main() {
    std::vector<std::pair<std::string, int>> submitted = {
        {"backup", 1}, {"patch", 5}, {"report", 1}, {"alert", 9}, {"rotate-logs", 5}, {"cleanup", 1}};
    std::priority_queue<Job, std::vector<Job>, Later> queue;
    int seq = 0;
    for (const auto& [name, priority] : submitted) queue.push({name, priority, seq++});
    std::cout << "queued: " << queue.size() << ", next: " << queue.top().name << '\n';
    while (!queue.empty()) {
        const Job& job = queue.top();
        std::cout << "run " << job.name << " (priority " << job.priority << ")\n";
        queue.pop();
    }
}
