@@lesson x2-containers Vectors, Maps and Algorithms
@@concepts lists, maps
@@text
## std::vector

`std::vector<T>` is the default container: a growable array of `T`, stored in one block of memory (fast).

```cpp
#include <iostream>
#include <string>
#include <vector>

int main() {
    std::vector<std::string> hosts = {"web01", "web02"};
    hosts.push_back("db01");
    std::cout << hosts.size() << ' ' << hosts[0] << ' ' << hosts.back() << '\n';

    for (const auto& h : hosts)            // range-for: const auto& avoids copies
        std::cout << "- " << h << '\n';

    hosts.erase(hosts.begin() + 1);        // remove "web02"
    std::cout << hosts.size() << '\n';
    std::cout << hosts.at(1) << '\n';      // at() checks the index and throws if it's wrong
}
```

`hosts[10]` on a 3-item vector is **undefined behaviour**: no error, just a read of random memory. `hosts.at(10)` throws `std::out_of_range`. Use `at()` while learning, or whenever an index comes from outside.

## std::map and std::unordered_map

```cpp
#include <iostream>
#include <map>
#include <string>
#include <unordered_map>

int main() {
    std::map<std::string, std::string> owners{{"web01", "aiko"}, {"db01", "kenji"}};
    owners["mail01"] = "yui";
    for (const auto& [host, owner] : owners)       // a map is sorted by key
        std::cout << host << " -> " << owner << '\n';

    if (auto it = owners.find("dns01"); it != owners.end())
        std::cout << it->second << '\n';
    else
        std::cout << "dns01: no owner\n";
    std::cout << owners.contains("db01") << '\n';  // C++20

    std::unordered_map<std::string, int> counts;   // a hash table: faster, unsorted
    for (std::string level : {"INFO", "ERROR", "INFO", "WARN", "INFO"})
        counts[level]++;                           // a missing key starts at 0
    std::cout << counts["INFO"] << '\n';
}
```

Careful: reading `owners["dns01"]` **inserts** an empty entry if it's missing. Use `find` or `contains` when you only want to look.

`std::set` / `std::unordered_set` hold unique values.

## Algorithms and lambdas

`<algorithm>` has ready-made, well-tested loops. They work on **iterators**: `begin()` and `end()` mark a range. A **lambda** `[](int x) { return x > 2; }` is an inline function.

```cpp
#include <algorithm>
#include <iostream>
#include <numeric>
#include <vector>

int main() {
    std::vector<int> cpu = {35, 92, 88, 71, 15};

    std::sort(cpu.begin(), cpu.end());                               // 15 35 71 88 92
    std::sort(cpu.begin(), cpu.end(), [](int a, int b) { return a > b; });   // descending
    int total = std::accumulate(cpu.begin(), cpu.end(), 0);
    auto hot = std::count_if(cpu.begin(), cpu.end(), [](int c) { return c > 80; });
    auto maxIt = std::max_element(cpu.begin(), cpu.end());
    bool anyIdle = std::any_of(cpu.begin(), cpu.end(), [](int c) { return c < 20; });

    std::cout << total << ' ' << hot << ' ' << *maxIt << ' ' << anyIdle << '\n';

    int limit = 50;
    // [limit] captures a local variable so the lambda can use it
    std::erase_if(cpu, [limit](int c) { return c < limit; });       // C++20
    for (int c : cpu) std::cout << c << ' ';
    std::cout << '\n';

    // C++20 ranges: algorithms that take the whole container
    std::ranges::sort(cpu);
    std::cout << cpu.front() << '\n';
}
```

Prefer an algorithm to a hand-written loop: it says *what* you're doing (`count_if`), and it's already correct.

## structs

Group related values with a `struct`:

```cpp
#include <algorithm>
#include <iostream>
#include <string>
#include <vector>

struct Server {
    std::string name;
    int cpu = 0;
    std::string env = "prod";
};

int main() {
    std::vector<Server> servers = {{"web01", 35}, {"web02", 92}, {"test01", 88, "test"}};
    std::ranges::sort(servers, {}, &Server::cpu);          // sort by the cpu member
    for (const auto& s : servers) std::cout << s.name << ' ' << s.cpu << ' ' << s.env << '\n';
}
```

@@quiz
`std::vector<int> v = {1, 2, 3}; v[5] = 9;` What happens?
* Undefined behaviour: it writes to memory it doesn't own
- It throws `std::out_of_range`
- The vector grows to 6 items
- A compile error
? `operator[]` doesn't check. `v.at(5)` throws instead.

@@quiz
`std::map<std::string, int> m; std::cout << m["x"];` What happens?
* It inserts `"x"` with value 0 and prints 0
- It throws
- Undefined behaviour
- It prints nothing
? `operator[]` creates missing entries. Use `find`/`contains` to only check.

@@quiz
What's the difference between `std::map` and `std::unordered_map`?
* `map` keeps keys sorted (a tree); `unordered_map` is a hash table (usually faster, no order)
- `unordered_map` allows duplicate keys
- `map` can only have string keys
- None: they're aliases
? Use `unordered_map` for fast lookups, `map` when you need keys in order.

@@drill
Which container keeps its keys sorted?
= std::map
= map
? std::unordered_map uses hashing and has no order.

@@flashcards
`std::vector<T>` :: growable array; the default container
`v.push_back(x)` :: add to the end
`v.at(i)` :: checked access (throws)
`std::unordered_map<K,V>` :: hash table
`m.contains(k)` :: check a key without inserting (C++20)
`[x](int a) { return a > x; }` :: lambda capturing x
`std::count_if(b, e, pred)` :: count matching items
`std::erase_if(v, pred)` :: remove matching items (C++20)

@@exercise Top talkers
Each line of input is `src_ip bytes`. Add up the bytes per IP and print the top 3 IPs, biggest first, as `ip bytes`.
@@stdin
10.0.0.5 1200
10.0.0.9 300
10.0.0.5 800
10.0.0.7 5000
10.0.0.9 2500
10.0.0.2 100
10.0.0.7 1
@@starter
#include <algorithm>
#include <iostream>
#include <string>
#include <unordered_map>
#include <vector>

int main() {
    std::unordered_map<std::string, long long> bytesByIp;
    std::string ip;
    long long bytes = 0;
    // read, total, sort, print the top 3
}
@@expected
10.0.0.7 5001
10.0.0.9 2800
10.0.0.5 2000
@@hint
After totalling, copy into a vector of pairs: `std::vector<std::pair<std::string, long long>> v(bytesByIp.begin(), bytesByIp.end());` Sort with a lambda comparing `.second`, then print the first `std::min<std::size_t>(3, v.size())`.
@@solution
#include <algorithm>
#include <iostream>
#include <string>
#include <unordered_map>
#include <vector>

int main() {
    std::unordered_map<std::string, long long> bytesByIp;
    std::string ip;
    long long bytes = 0;
    while (std::cin >> ip >> bytes) bytesByIp[ip] += bytes;

    std::vector<std::pair<std::string, long long>> v(bytesByIp.begin(), bytesByIp.end());
    std::sort(v.begin(), v.end(), [](const auto& a, const auto& b) { return a.second > b.second; });
    for (std::size_t i = 0; i < std::min<std::size_t>(3, v.size()); ++i)
        std::cout << v[i].first << ' ' << v[i].second << '\n';
}

@@exercise Unique, sorted
Read words (usernames) until input ends and print each distinct one once, in alphabetical order, then `<n> unique`.
@@stdin
tanaka suzuki tanaka abe
mori abe tanaka
@@starter
#include <iostream>
#include <set>
#include <string>

int main() {
    // read every word into a std::set
}
@@expected
abe
mori
suzuki
tanaka
4 unique
@@requires
std::set
.insert(
@@hint
`std::set<std::string> users; std::string w; while (std::cin >> w) users.insert(w);` A set is already sorted.
@@solution
#include <iostream>
#include <set>
#include <string>

int main() {
    std::set<std::string> users;
    std::string w;
    while (std::cin >> w) users.insert(w);
    for (const auto& u : users) std::cout << u << '\n';
    std::cout << users.size() << " unique\n";
}
