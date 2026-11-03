@@lesson e6-concurrency Concurrency in Depth: Locks, Condition Variables, Atomics and jthread
@@concepts concurrency
@@text
`std::thread` and `std::mutex` are the start. Real concurrent C++ needs a few more tools, and a clear idea of what can go wrong.

## Locks

```cpp,norun
std::mutex m;
{
    std::lock_guard lock(m);         // simplest: lock now, unlock at end of scope
}
std::unique_lock lock(m);            // can unlock early and works with condition variables
std::scoped_lock both(a.m, b.m);     // locks several mutexes without deadlock
std::shared_mutex rw;                // many readers (shared_lock) or one writer (unique_lock)
```

**Deadlock** happens when thread 1 holds A and waits for B while thread 2 holds B and waits for A. Fix it by always locking in the same order, or by locking both at once with `std::scoped_lock`, which uses a deadlock-avoidance algorithm.

## Condition variables

A condition variable lets a thread sleep until another thread changes some shared state:

```cpp,norun
std::unique_lock lock(m);
cv.wait(lock, [&] { return !queue.empty() || closed; });   // re-checks the condition: handles spurious wakeups
```

Always wait with a predicate, change the state while holding the mutex, and `notify_one`/`notify_all` after changing it.

## Atomics and memory order

`std::atomic<T>` makes single operations indivisible without a mutex: `counter.fetch_add(1)`. The **memory order** says what other memory the operation synchronises:

- `seq_cst` (the default): simplest to reason about; use it unless profiling says otherwise
- `acquire`/`release`: a release store "publishes" the writes before it to a thread that does an acquire load of the same atomic
- `relaxed`: only atomicity, no ordering; fine for statistics counters

Lock-free data structures beyond counters and flags are very hard to get right; prefer a mutex.

## jthread and stop tokens (C++20)

`std::jthread` joins automatically in its destructor (a plain `std::thread` that's still joinable calls `std::terminate`) and supports **cooperative cancellation**:

```cpp,norun
std::jthread worker([](std::stop_token st) {
    while (!st.stop_requested()) poll_queue();
});
worker.request_stop();   // or just let it go out of scope: request_stop + join
```

C++20 also adds `std::latch` (wait until N things happened), `std::barrier` (phases) and `std::counting_semaphore`.

## Tools

Compile with `-fsanitize=thread` (ThreadSanitizer) to find data races. A data race (two threads accessing the same non-atomic memory, at least one writing, with no synchronisation) is undefined behaviour, even if the program "seems to work".

@@quiz
How does `std::scoped_lock lock(a.m, b.m)` prevent deadlock?
* It acquires both mutexes with a deadlock-avoidance algorithm, regardless of argument order
- It locks them one at a time in argument order
- It uses only one of them
- It times out
? Two threads locking (a, b) and (b, a) won't deadlock.

@@quiz
Why pass a predicate to `cv.wait`?
- It's faster
* The thread may wake spuriously or after another consumer took the item, so the condition must be re-checked
- It's required to compile
- To avoid locking the mutex
? `wait(lock, pred)` loops until pred is true.

@@quiz
What does a `std::jthread` do in its destructor?
- Detaches the thread
- Calls std::terminate
* Requests stop and joins
- Nothing
? A joinable std::thread would terminate the program.

@@flashcards
`std::lock_guard` :: lock for a scope
`std::unique_lock` :: movable lock, works with condition variables
`std::scoped_lock` :: lock several mutexes deadlock-free
`std::shared_mutex` :: readers-writer lock
`std::condition_variable` :: sleep until notified
Spurious wakeup :: wait returns without notify
`std::atomic<T>` :: indivisible operations
`memory_order_relaxed` :: atomicity only
`std::jthread` :: auto-joining thread with stop_token
`std::latch` :: one-shot countdown
Data race :: unsynchronised concurrent access: UB

@@exercise A bounded blocking queue
Write `BlockingQueue<T>` with a capacity:

- `bool push(T)` blocks while full; returns false once the queue is closed
- `std::optional<T> pop()` blocks while empty; returns `std::nullopt` when the queue is closed **and** drained
- `close()` wakes everyone up
- track the peak size seen (under the lock)

Run 2 producers pushing 1..500 each and 3 consumers summing into an atomic, with a capacity of 8. Join the producers, close the queue, join the consumers. Print only deterministic facts.
@@starter
#include <atomic>
#include <condition_variable>
#include <deque>
#include <iostream>
#include <mutex>
#include <optional>
#include <thread>
#include <vector>

int main() {
    std::cout << "TODO\n";
}
@@expected
produced: 1000
consumed: 1000
sum: 250500
peak queue size <= 8: true
every consumer exited: true
@@requires
std::condition_variable
wait(lock
@@hint
Two condition variables: `not_full` (producers wait on it) and `not_empty` (consumers wait on it). `push`: `not_full.wait(lock, [&] { return q.size() < capacity || closed; });`. `pop`: `not_empty.wait(lock, [&] { return !q.empty() || closed; }); if (q.empty()) return std::nullopt;`. Notify the other side after each change.
@@solution
#include <atomic>
#include <condition_variable>
#include <deque>
#include <iostream>
#include <mutex>
#include <optional>
#include <thread>
#include <vector>

template <class T>
class BlockingQueue {
public:
    explicit BlockingQueue(size_t capacity) : capacity_(capacity) {}

    bool push(T value) {
        std::unique_lock lock(m_);
        not_full_.wait(lock, [&] { return q_.size() < capacity_ || closed_; });
        if (closed_) return false;
        q_.push_back(std::move(value));
        if (q_.size() > peak_) peak_ = q_.size();
        not_empty_.notify_one();
        return true;
    }

    std::optional<T> pop() {
        std::unique_lock lock(m_);
        not_empty_.wait(lock, [&] { return !q_.empty() || closed_; });
        if (q_.empty()) return std::nullopt;
        T value = std::move(q_.front());
        q_.pop_front();
        not_full_.notify_one();
        return value;
    }

    void close() {
        {
            std::lock_guard lock(m_);
            closed_ = true;
        }
        not_empty_.notify_all();
        not_full_.notify_all();
    }

    size_t peak() {
        std::lock_guard lock(m_);
        return peak_;
    }

private:
    std::mutex m_;
    std::condition_variable not_empty_, not_full_;
    std::deque<T> q_;
    size_t capacity_;
    size_t peak_ = 0;
    bool closed_ = false;
};

int main() {
    BlockingQueue<int> queue(8);
    std::atomic<int> produced{0}, consumed{0}, exited{0};
    std::atomic<long long> sum{0};

    std::vector<std::thread> producers, consumers;
    for (int p = 0; p < 2; ++p)
        producers.emplace_back([&] {
            for (int i = 1; i <= 500; ++i)
                if (queue.push(i)) ++produced;
        });
    for (int c = 0; c < 3; ++c)
        consumers.emplace_back([&] {
            while (auto item = queue.pop()) {
                sum += *item;
                ++consumed;
            }
            ++exited;
        });

    for (auto& t : producers) t.join();
    queue.close();
    for (auto& t : consumers) t.join();

    std::cout << "produced: " << produced << '\n';
    std::cout << "consumed: " << consumed << '\n';
    std::cout << "sum: " << sum << '\n';
    std::cout << std::boolalpha << "peak queue size <= 8: " << (queue.peak() <= 8) << '\n';
    std::cout << "every consumer exited: " << (exited == 3) << '\n';
}

@@exercise Transfers without deadlock, and a cancellable monitor
Two threads move money between the same two accounts in **opposite** directions, 100,000 times each. Locking `from` then `to` one by one would deadlock; use `std::scoped_lock`.

- `struct Account { std::mutex m; long long balance; }` with `transfer(from, to, amount)` where amount is `i % 7 + 1`
- a monitor `std::jthread` loops until `stop_requested()`, counting how often it saw a consistent total (lock both accounts, check `a + b == 2000`) in an atomic, and sets a flag when it exits
- after the transfer threads finish, request the monitor to stop and join it
@@starter
#include <atomic>
#include <iostream>
#include <mutex>
#include <thread>

struct Account {
    std::mutex m;
    long long balance = 1000;
};

int main() {
    std::cout << "TODO\n";
}
@@expected
A=1000 B=1000
transfers: 200000
monitor checks all consistent: true
monitor stopped cleanly: true
@@requires
std::scoped_lock
std::jthread
stop_requested
@@hint
Both threads transfer the same amounts in opposite directions, so the final balances are back at 1000 regardless of the interleaving. Monitor: `std::jthread monitor([&](std::stop_token st) { while (!st.stop_requested()) { std::scoped_lock lock(a.m, b.m); if (a.balance + b.balance != 2000) bad = true; ++checks; } stopped = true; });`. Then `monitor.request_stop(); monitor.join();`.
@@solution
#include <atomic>
#include <iostream>
#include <mutex>
#include <thread>

struct Account {
    std::mutex m;
    long long balance = 1000;
};

void transfer(Account& from, Account& to, long long amount) {
    std::scoped_lock lock(from.m, to.m);
    from.balance -= amount;
    to.balance += amount;
}

int main() {
    Account a, b;
    std::atomic<int> transfers{0};
    std::atomic<long long> checks{0};
    std::atomic<bool> bad{false}, stopped{false};

    std::jthread monitor([&](std::stop_token st) {
        while (!st.stop_requested()) {
            std::scoped_lock lock(a.m, b.m);
            if (a.balance + b.balance != 2000) bad = true;
            ++checks;
        }
        stopped = true;
    });

    {
        std::jthread t1([&] {
            for (int i = 0; i < 100000; ++i) {
                transfer(a, b, i % 7 + 1);
                ++transfers;
            }
        });
        std::jthread t2([&] {
            for (int i = 0; i < 100000; ++i) {
                transfer(b, a, i % 7 + 1);
                ++transfers;
            }
        });
    }

    monitor.request_stop();
    monitor.join();
    std::cout << "A=" << a.balance << " B=" << b.balance << '\n';
    std::cout << "transfers: " << transfers << '\n';
    std::cout << std::boolalpha << "monitor checks all consistent: " << !bad << '\n';
    std::cout << "monitor stopped cleanly: " << stopped.load() << '\n';
}
