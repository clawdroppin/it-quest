@@lesson xp-ring_logger Ring Buffer Logger
@@text
Embedded devices and high-performance services often keep the last N log messages in a **ring buffer**: a fixed array where new entries overwrite the oldest. Write a templated `RingBuffer<T, N>` and a logger on top of it with levels and timestamps, then dump the buffer to a file when something goes wrong.

**You'll practise:** templates with non-type parameters · std::array · modular arithmetic · RAII and file streams · std::chrono

## Milestones

1. **RingBuffer**: `push`, `size`, `full`, and iteration from oldest to newest.
2. **Overwrite**: pushing into a full buffer drops the oldest entry.
3. **Logger**: `log(Level, std::string_view)` stores time, level and message.
4. **Dump**: write all entries to a file, oldest first.
5. **Tests**: a small tests.cpp that checks wrap-around with N = 3.

## You're done when

- The tests pass, including pushing 10 items into a buffer of 3 and reading back the last 3 in order.

## Getting help

Open the project, then the 🤖 **AI Helper**: it sees the lesson and your files. Good questions to start with:

- *"How do I write an iterator for my ring buffer?"*
- *"Why does a fixed-size buffer help in real-time systems?"*

Ask for hints, reviews and explanations rather than finished code: you learn by writing it.
@@project ring_logger Ring Buffer Logger
Create the folder, open it in VS Code, and work through `README.md` one milestone at a time.
@@file README.md
# Ring Buffer Logger

Embedded devices and high-performance services often keep the last N log messages in a **ring buffer**: a fixed array where new entries overwrite the oldest. Write a templated `RingBuffer<T, N>` and a logger on top of it with levels and timestamps, then dump the buffer to a file when something goes wrong.

## Milestones

- [ ] 1. RingBuffer: `push`, `size`, `full`, and iteration from oldest to newest.
- [ ] 2. Overwrite: pushing into a full buffer drops the oldest entry.
- [ ] 3. Logger: `log(Level, std::string_view)` stores time, level and message.
- [ ] 4. Dump: write all entries to a file, oldest first.
- [ ] 5. Tests: a small tests.cpp that checks wrap-around with N = 3.

## Done when

- [ ] The tests pass, including pushing 10 items into a buffer of 3 and reading back the last 3 in order.
@@file main.cpp
// A fixed-size ring buffer and a logger built on it.
#include <array>
#include <cstddef>
#include <iostream>
#include <string>

template <typename T, std::size_t N>
class RingBuffer {
public:
    void push(const T& value) {
        // TODO: write at head_, advance head_ modulo N, grow size_ up to N
        (void)value;
    }
    std::size_t size() const { return size_; }
    bool full() const { return size_ == N; }

private:
    std::array<T, N> items_{};
    std::size_t head_ = 0;
    std::size_t size_ = 0;
};

int main() {
    RingBuffer<std::string, 3> log;
    for (int i = 1; i <= 5; ++i) log.push("message " + std::to_string(i));
    std::cout << "stored " << log.size() << " messages\n";  // should be 3
}
