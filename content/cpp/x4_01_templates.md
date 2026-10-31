@@lesson x4-templates Templates and Generic Code
@@text
## Function templates

A **template** is code with a type left open. The compiler generates a version for each type you use it with: no runtime cost.

```cpp
#include <iostream>
#include <string>
#include <vector>

template <typename T>
T clampTo(T value, T low, T high) {
    return value < low ? low : (value > high ? high : value);
}

template <typename Container>
void printAll(const Container& c) {
    for (const auto& x : c) std::cout << x << ' ';
    std::cout << '\n';
}

int main() {
    std::cout << clampTo(150, 0, 100) << ' ' << clampTo(2.5, 0.0, 1.0) << '\n';
    printAll(std::vector<int>{1, 2, 3});
    printAll(std::vector<std::string>{"dns", "dhcp"});
}
```

That's how `std::vector<T>`, `std::sort` and `std::max` work: the whole standard library is templates.

## Concepts: saying what a type must support (C++20)

Without constraints, using a template with the wrong type gives a huge error message from deep inside the template. **Concepts** check requirements up front:

```cpp
#include <concepts>
#include <iostream>

template <std::integral T>             // only integer types
T percentOf(T part, T whole) {
    return whole == 0 ? 0 : part * 100 / whole;
}

int main() {
    std::cout << percentOf(45, 60) << '\n';
    // percentOf(4.5, 6.0);  error: 'double' does not satisfy 'integral'. Short and clear.
}
```

Standard concepts include `std::integral`, `std::floating_point`, `std::same_as<T, U>`, `std::convertible_to`, and `std::ranges::range`.

## Class templates

```cpp
#include <iostream>
#include <string>
#include <vector>

// Keep the last N items, e.g. recent log lines or latency samples.
template <typename T, std::size_t N>
class RingBuffer {
public:
    void push(const T& item) {
        data_[next_] = item;
        next_ = (next_ + 1) % N;
        if (size_ < N) ++size_;
    }
    std::size_t size() const { return size_; }
    std::vector<T> items() const {           // oldest first
        std::vector<T> out;
        std::size_t start = size_ < N ? 0 : next_;
        for (std::size_t i = 0; i < size_; ++i) out.push_back(data_[(start + i) % N]);
        return out;
    }
private:
    T data_[N]{};
    std::size_t next_ = 0, size_ = 0;
};

int main() {
    RingBuffer<int, 3> latency;
    for (int ms : {12, 15, 40, 11, 9}) latency.push(ms);
    for (int ms : latency.items()) std::cout << ms << ' ';   // 40 11 9
    std::cout << '\n';
}
```

Templates must be visible where they're used, so they live in **header files** (`.hpp`), not `.cpp` files. That's why "header-only libraries" are common in C++.

## Lambdas as parameters

A template can take any callable, which is how `std::sort` accepts your comparison:

```cpp
#include <iostream>
#include <vector>

template <typename T, typename Pred>
std::vector<T> filter(const std::vector<T>& v, Pred keep) {
    std::vector<T> out;
    for (const auto& x : v) if (keep(x)) out.push_back(x);
    return out;
}

int main() {
    std::vector<int> ports{22, 80, 443, 8080, 3389};
    for (int p : filter(ports, [](int p) { return p >= 1024; })) std::cout << p << ' ';
    std::cout << '\n';
}
```

`std::function<bool(int)>` can store any callable too (for callbacks kept in a member), at a small runtime cost.

@@quiz
When is the code for `clampTo<int>` generated?
* At compile time, when the compiler sees it used with `int`
- At run time, the first time it's called
- Never: templates are interpreted
- When the program is installed
? Templates are instantiated by the compiler, so there's no runtime overhead.

@@quiz
Why do templates usually live in header files?
* The compiler needs the full template code wherever it's used, to generate each version
- Headers compile faster
- `.cpp` files can't contain templates
- It's only a convention
? Put template definitions in headers so every file that uses them can instantiate them.

@@quiz
What does `template <std::integral T>` add over `template <typename T>`?
* The compiler rejects non-integer types with a clear error, before compiling the body
- It makes the function faster
- It allows `T` to be any type
- Nothing
? Concepts state the requirements of a template and give readable errors.

@@exercise Generic maximum by key
Write a template `maxBy(const std::vector<T>& items, KeyFn key)` that returns the item with the largest `key(item)`. Assume the vector isn't empty. It must work for both calls in `main`.
@@starter
#include <iostream>
#include <string>
#include <vector>

struct Server { std::string name; int cpu; };

// write maxBy here

int main() {
    std::vector<Server> servers{{"web01", 35}, {"web02", 92}, {"db01", 71}};
    std::cout << maxBy(servers, [](const Server& s) { return s.cpu; }).name << '\n';

    std::vector<std::string> words{"dns", "loadbalancer", "vpn"};
    std::cout << maxBy(words, [](const std::string& w) { return w.size(); }) << '\n';
}
@@expected
web02
loadbalancer
@@requires
template
@@hint
`template <typename T, typename KeyFn> T maxBy(const std::vector<T>& items, KeyFn key) { T best = items[0]; for (const auto& x : items) if (key(x) > key(best)) best = x; return best; }`
@@solution
#include <iostream>
#include <string>
#include <vector>

struct Server { std::string name; int cpu; };

template <typename T, typename KeyFn>
T maxBy(const std::vector<T>& items, KeyFn key) {
    T best = items.front();
    for (const auto& x : items)
        if (key(x) > key(best)) best = x;
    return best;
}

int main() {
    std::vector<Server> servers{{"web01", 35}, {"web02", 92}, {"db01", 71}};
    std::cout << maxBy(servers, [](const Server& s) { return s.cpu; }).name << '\n';

    std::vector<std::string> words{"dns", "loadbalancer", "vpn"};
    std::cout << maxBy(words, [](const std::string& w) { return w.size(); }) << '\n';
}

@@exercise Moving average
Complete the class template `MovingAverage<N>`: `add(double)` keeps only the last `N` values, and `value()` returns their average (0 when empty).
@@starter
#include <array>
#include <iostream>

template <std::size_t N>
class MovingAverage {
public:
    void add(double x) { /* TODO: not done yet */ }
    double value() const { return 0; }
private:
    std::array<double, N> window_{};
    std::size_t next_ = 0, count_ = 0;
};

int main() {
    MovingAverage<3> ma;
    std::cout << ma.value() << '\n';
    for (double ms : {10.0, 20.0, 30.0, 40.0}) {
        ma.add(ms);
        std::cout << ma.value() << '\n';
    }
}
@@expected
0
10
15
20
30
@@hint
In `add`: `window_[next_] = x; next_ = (next_ + 1) % N; if (count_ < N) ++count_;`. In `value`: add up the first `count_` entries (the order doesn't matter for an average) and divide.
@@solution
#include <array>
#include <iostream>

template <std::size_t N>
class MovingAverage {
public:
    void add(double x) {
        window_[next_] = x;
        next_ = (next_ + 1) % N;
        if (count_ < N) ++count_;
    }
    double value() const {
        if (count_ == 0) return 0;
        double sum = 0;
        for (std::size_t i = 0; i < count_; ++i) sum += window_[i];
        return sum / count_;
    }
private:
    std::array<double, N> window_{};
    std::size_t next_ = 0, count_ = 0;
};

int main() {
    MovingAverage<3> ma;
    std::cout << ma.value() << '\n';
    for (double ms : {10.0, 20.0, 30.0, 40.0}) {
        ma.add(ms);
        std::cout << ma.value() << '\n';
    }
}
