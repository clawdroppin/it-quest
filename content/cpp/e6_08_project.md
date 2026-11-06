@@lesson e6-project Project: A Thread Pool with Futures
@@concepts concurrency, objects
@@text
A **thread pool** keeps a fixed set of worker threads and feeds them tasks from a queue. Creating threads is expensive, and running thousands at once thrashes the CPU, so servers, build systems, game engines and backup tools all use pools.

## The design

```text,norun
submit(f) ──▶ [ task queue (mutex + condition variable) ] ──▶ worker 1
   │                                                       ──▶ worker 2
   └──▶ returns std::future<R>  ◀── result or exception ──── worker N
```

- `submit` wraps the callable in a `std::packaged_task<R()>`, which connects it to a `std::future<R>`. The task goes into the queue as a type-erased `std::function<void()>`. A `packaged_task` is move-only, so it's held in a `shared_ptr` that the `std::function` copies.
- Workers loop: wait on the condition variable until there's a task or the pool is stopping, pop, run (outside the lock!).
- An exception thrown by a task is stored in the future and rethrown by `future.get()` in the caller's thread.
- Shutdown: set `stopping`, `notify_all`, join every worker. Decide whether queued tasks still run (here: yes, the queue is drained first).

## Why not std::async?

`std::async(std::launch::async, f)` starts a new thread per call (on most implementations), and the future it returns **blocks in its destructor**. That's handy for a few tasks, but not for thousands.

## Beyond this project

Production pools add work stealing (each worker has its own deque and steals from others when idle), priorities, and back-pressure (a bounded queue, as in the previous lesson). Libraries: Intel oneTBB, BS::thread_pool, Asio's `thread_pool`, and the C++26 `std::execution` (senders/receivers).

@@quiz
What does `std::packaged_task` provide?
* A callable whose result (or exception) is delivered to an associated std::future
- A thread
- A lock
- A queue
? `task.get_future()` before queuing it.

@@quiz
Why run the task outside the queue's lock?
- It's required by the standard
* Otherwise only one task could run at a time, and a task that submits more work would deadlock
- To make tasks faster to compile
- Locks can't be held during function calls
? Hold locks only around the shared queue.

@@quiz
How does a caller get the result of a task submitted to the pool?
* Through the std::future returned by submit, using .get()
- By reading a global variable
- The pool prints it
- By polling the queue
? .get() waits for the result and rethrows any exception from the task.

@@drill
Which class wraps a callable so its result can be collected through a `std::future`?
= std::packaged_task
= packaged_task
? get_future() gives the matching future.

@@flashcards
Thread pool :: fixed workers + task queue
`std::packaged_task<R()>` :: task linked to a future
`std::future<R>::get()` :: wait for result, rethrow exceptions
`std::function<void()>` :: type-erased task in the queue
Work stealing :: idle workers take others' tasks
`std::async` future :: blocks in its destructor
Back-pressure :: bounded queue slows producers

@@exercise A thread pool
Write `ThreadPool`:

- constructor starts `n` workers; `shutdown()` (also called by the destructor) drains the queue, stops and joins them
- `template <class F> auto submit(F f) -> std::future<std::invoke_result_t<F>>` using a `std::packaged_task` in a `shared_ptr`; submitting after shutdown throws `std::runtime_error("pool is stopped")`
- record the ids of the threads that ran tasks (under a mutex)

Submit "sum 1..n" tasks for n = 10, 100, 1000000, -1 (throws `std::invalid_argument("negative n: -1")`) and 5000 to a pool of 4, and print results in submission order.
@@starter
#include <condition_variable>
#include <deque>
#include <functional>
#include <future>
#include <iostream>
#include <memory>
#include <mutex>
#include <set>
#include <stdexcept>
#include <string>
#include <thread>
#include <type_traits>
#include <vector>

int main() {
    std::cout << "TODO\n";
}
@@expected
sum(10) = 55
sum(100) = 5050
sum(1000000) = 500000500000
sum(-1) failed: negative n: -1
sum(5000) = 12502500
worker threads used <= 4: true
submit after shutdown throws: true
@@requires
std::packaged_task
std::future
notify_all
@@hint
Worker loop: `std::function<void()> job; { std::unique_lock lock(m_); cv_.wait(lock, [&] { return stopping_ || !queue_.empty(); }); if (stopping_ && queue_.empty()) return; job = std::move(queue_.front()); queue_.pop_front(); } job();`. In `submit`: `auto task = std::make_shared<std::packaged_task<R()>>(std::move(f)); auto fut = task->get_future(); queue_.emplace_back([task] { (*task)(); });`. Make `shutdown` safe to call twice.
@@solution
#include <condition_variable>
#include <deque>
#include <functional>
#include <future>
#include <iostream>
#include <memory>
#include <mutex>
#include <set>
#include <stdexcept>
#include <string>
#include <thread>
#include <type_traits>
#include <vector>

class ThreadPool {
public:
    explicit ThreadPool(size_t n) {
        for (size_t i = 0; i < n; ++i) workers_.emplace_back([this] { loop(); });
    }

    ~ThreadPool() { shutdown(); }

    template <class F>
    auto submit(F f) -> std::future<std::invoke_result_t<F>> {
        using R = std::invoke_result_t<F>;
        auto task = std::make_shared<std::packaged_task<R()>>(std::move(f));
        auto fut = task->get_future();
        {
            std::lock_guard lock(m_);
            if (stopping_) throw std::runtime_error("pool is stopped");
            queue_.emplace_back([task] { (*task)(); });
        }
        cv_.notify_one();
        return fut;
    }

    void shutdown() {
        {
            std::lock_guard lock(m_);
            if (stopping_) return;
            stopping_ = true;
        }
        cv_.notify_all();
        for (auto& t : workers_) t.join();
    }

    size_t threads_used() {
        std::lock_guard lock(m_);
        return used_.size();
    }

private:
    void loop() {
        while (true) {
            std::function<void()> job;
            {
                std::unique_lock lock(m_);
                cv_.wait(lock, [&] { return stopping_ || !queue_.empty(); });
                if (stopping_ && queue_.empty()) return;
                job = std::move(queue_.front());
                queue_.pop_front();
                used_.insert(std::this_thread::get_id());
            }
            job();
        }
    }

    std::mutex m_;
    std::condition_variable cv_;
    std::deque<std::function<void()>> queue_;
    std::vector<std::thread> workers_;
    std::set<std::thread::id> used_;
    bool stopping_ = false;
};

int main() {
    ThreadPool pool(4);
    std::vector<long long> inputs = {10, 100, 1000000, -1, 5000};
    std::vector<std::future<long long>> results;
    for (long long n : inputs) {
        results.push_back(pool.submit([n] {
            if (n < 0) throw std::invalid_argument("negative n: " + std::to_string(n));
            long long sum = 0;
            for (long long i = 1; i <= n; ++i) sum += i;
            return sum;
        }));
    }
    for (size_t i = 0; i < inputs.size(); ++i) {
        try {
            long long value = results[i].get();   // get() first: it rethrows the task's exception
            std::cout << "sum(" << inputs[i] << ") = " << value << '\n';
        } catch (const std::exception& e) {
            std::cout << "sum(" << inputs[i] << ") failed: " << e.what() << '\n';
        }
    }
    pool.shutdown();
    std::cout << std::boolalpha << "worker threads used <= 4: " << (pool.threads_used() <= 4) << '\n';
    bool threw = false;
    try {
        pool.submit([] { return 1; });
    } catch (const std::runtime_error&) {
        threw = true;
    }
    std::cout << "submit after shutdown throws: " << threw << '\n';
}

@@exercise Parallel map-reduce over a log
Count HTTP status codes in 10,000 log lines in parallel:

- build the lines deterministically: line `i` is `GET /page<i> <status>` with status `codes[i % 7]` from `{200, 200, 200, 404, 500, 200, 301}`
- split the lines into 4 chunks; each chunk is a task that parses the last field with `std::from_chars` and returns a `std::map<int, int>`
- run the chunks with `std::async(std::launch::async, ...)` (fine for 4 tasks), merge the maps, and compare with a sequential count
@@starter
#include <charconv>
#include <future>
#include <iostream>
#include <map>
#include <string>
#include <string_view>
#include <vector>

int main() {
    std::cout << "TODO\n";
}
@@expected
200: 5715
301: 1428
404: 1429
500: 1428
chunks: 4, total: 10000, matches sequential: true
@@requires
std::async
std::from_chars
@@hint
A function `std::map<int, int> count(const std::vector<std::string>& lines, size_t begin, size_t end)`; capture `lines` by reference in the lambdas (they all finish before `lines` goes away). Chunk `c` covers `[c * size / 4, (c + 1) * size / 4)`. The status is after the last space: `line.rfind(' ')`.
@@solution
#include <charconv>
#include <future>
#include <iostream>
#include <map>
#include <string>
#include <string_view>
#include <vector>

std::map<int, int> count(const std::vector<std::string>& lines, size_t begin, size_t end) {
    std::map<int, int> counts;
    for (size_t i = begin; i < end; ++i) {
        std::string_view line = lines[i];
        auto space = line.rfind(' ');
        int status = 0;
        std::from_chars(line.data() + space + 1, line.data() + line.size(), status);
        ++counts[status];
    }
    return counts;
}

int main() {
    const int codes[] = {200, 200, 200, 404, 500, 200, 301};
    std::vector<std::string> lines;
    for (int i = 0; i < 10000; ++i) lines.push_back("GET /page" + std::to_string(i) + ' ' + std::to_string(codes[i % 7]));

    const size_t chunks = 4;
    std::vector<std::future<std::map<int, int>>> parts;
    for (size_t c = 0; c < chunks; ++c) {
        size_t begin = c * lines.size() / chunks, end = (c + 1) * lines.size() / chunks;
        parts.push_back(std::async(std::launch::async, [&lines, begin, end] { return count(lines, begin, end); }));
    }
    std::map<int, int> merged;
    for (auto& p : parts)
        for (const auto& [status, n] : p.get()) merged[status] += n;

    int total = 0;
    for (const auto& [status, n] : merged) {
        std::cout << status << ": " << n << '\n';
        total += n;
    }
    std::cout << std::boolalpha << "chunks: " << chunks << ", total: " << total
              << ", matches sequential: " << (merged == count(lines, 0, lines.size())) << '\n';
}
