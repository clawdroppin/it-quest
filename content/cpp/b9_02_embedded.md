@@lesson b9-embedded 🎓 Beyond IT: Embedded C++ and Arduino
@@concepts numbers, loops
@@text
> **Beyond IT scope.** Microcontroller programming: great fun with real hardware, but not needed for IT jobs.

Microcontrollers (Arduino's AVR chips, STM32, ESP32, RP2040) run C and C++ with kilobytes of RAM, no operating system and no screen. The Arduino "language" is C++ with a small library:

```cpp,norun
const int LED = 13, BUTTON = 2;

void setup() {
    pinMode(LED, OUTPUT);
    pinMode(BUTTON, INPUT_PULLUP);
    Serial.begin(115200);
}

void loop() {                                  // called forever
    bool pressed = digitalRead(BUTTON) == LOW; // pull-up: pressed reads LOW
    digitalWrite(LED, pressed ? HIGH : LOW);
    delay(10);
}
```

## Talking to hardware: memory-mapped registers

Under `digitalWrite` the chip has **registers** at fixed addresses. Writing bits into them configures pins and drives outputs:

```cpp,norun
auto* GPIOA_ODR = reinterpret_cast<volatile uint32_t*>(0x40020014);   // STM32 output data register
*GPIOA_ODR = *GPIOA_ODR | (1u << 5);                                   // set pin 5 high
```

- `volatile` tells the compiler every read and write really matters (the hardware can change the value, and writes have side effects), so it must not cache or remove them.
- Bit manipulation is everyday work: set `x | mask`, clear `x & ~mask`, toggle `x ^ mask`, test `(x >> n) & 1`, multi-bit fields `(x >> shift) & 0b11`.

## Embedded C++ rules

- usually no heap after start-up (fragmentation in 32 KB of RAM is fatal), no exceptions, no RTTI: `-fno-exceptions -fno-rtti`
- `constexpr`, templates, `std::array`, RAII and `enum class` still work and cost nothing
- **interrupt service routines** (ISRs) run between any two instructions of your main loop: keep them tiny, share data through `volatile` or atomic variables, and use lock-free single-producer/single-consumer **ring buffers** (a UART ISR pushes bytes, the main loop pops them)
- mechanical buttons **bounce**: a press produces several on/off flickers for a few milliseconds, so software **debounces** by waiting for a stable reading
- RTOSes (FreeRTOS, Zephyr) add tasks and queues on bigger chips; PlatformIO is a popular build environment

@@quiz
Why must a hardware register be accessed through a `volatile` pointer?
* So the compiler performs every read and write exactly as written, instead of caching or removing them
- To make it thread-safe
- To make it faster
- Because registers are constant
? Hardware has side effects the compiler can't see.

@@quiz
What does `reg & ~(1u << 3)` do?
- Sets bit 3
* Clears bit 3, leaving the other bits unchanged
- Toggles bit 3
- Tests bit 3
? `~` inverts the mask.

@@quiz
Why debounce a button?
- To save power
* The contacts bounce, producing several fake transitions per press
- Buttons are digital already
- To prevent overheating
? Wait for the reading to be stable for a few samples.

@@flashcards
`setup()` / `loop()` :: Arduino program structure
`volatile` :: every access is real
Set bit :: `x | (1u << n)`
Clear bit :: `x & ~(1u << n)`
Toggle bit :: `x ^ (1u << n)`
Test bit :: `(x >> n) & 1u`
ISR :: interrupt service routine
SPSC ring buffer :: lock-free ISR → main loop queue
Debouncing :: ignore contact bounce
`-fno-exceptions` :: common embedded flag

@@exercise A GPIO driver on simulated registers
Simulate an STM32-style GPIO port as a struct of `volatile uint32_t` registers `MODER` (2 bits per pin: `01` = output), `ODR` (output data) and `IDR` (input data), and write a tiny driver:

- `set_mode(port, pin, mode)`: clear the pin's 2-bit field, then set it
- `write(port, pin, on)`, `toggle(port, pin)`, `read(port, pin)` and `is_output(port, pin)`
- use read-modify-write with explicit assignments (`p.ODR = p.ODR | mask;`), since compound assignment on `volatile` is deprecated in C++20

Make pins 5 and 13 outputs, set pin 5, toggle 13, toggle 5, simulate a high input on pin 0, and print the registers in hex.
@@starter
#include <cstdint>
#include <format>
#include <iostream>

struct GpioPort {
    volatile uint32_t MODER = 0;
    volatile uint32_t ODR = 0;
    volatile uint32_t IDR = 0;
};

int main() {
    GpioPort gpioa;
    std::cout << "TODO\n";
}
@@expected
MODER=0x04000400
ODR after set 5: 0x00000020
ODR after toggle 13: 0x00002020
ODR after toggle 5: 0x00002000
pin 0 reads: 1
pin 5 is output: true, pin 6 is output: false
@@requires
volatile uint32_t
~(
@@hint
`uint32_t v = p.MODER; v = v & ~(0b11u << (pin * 2)); v = v | ((mode & 0b11u) << (pin * 2)); p.MODER = v;`. Copy a volatile register into a plain `uint32_t` before passing it to `std::format("{:#010x}", value)`.
@@solution
#include <cstdint>
#include <format>
#include <iostream>

struct GpioPort {
    volatile uint32_t MODER = 0;
    volatile uint32_t ODR = 0;
    volatile uint32_t IDR = 0;
};

constexpr uint32_t OUTPUT = 0b01;

void set_mode(GpioPort& p, int pin, uint32_t mode) {
    uint32_t v = p.MODER;
    v = v & ~(0b11u << (pin * 2));
    v = v | ((mode & 0b11u) << (pin * 2));
    p.MODER = v;
}

void write(GpioPort& p, int pin, bool on) {
    uint32_t mask = 1u << pin;
    p.ODR = on ? (p.ODR | mask) : (p.ODR & ~mask);
}

void toggle(GpioPort& p, int pin) { p.ODR = p.ODR ^ (1u << pin); }

bool read(const GpioPort& p, int pin) { return (p.IDR >> pin) & 1u; }

bool is_output(const GpioPort& p, int pin) { return ((p.MODER >> (pin * 2)) & 0b11u) == OUTPUT; }

std::string hex(uint32_t v) { return std::format("{:#010x}", v); }

int main() {
    GpioPort gpioa;
    set_mode(gpioa, 5, OUTPUT);
    set_mode(gpioa, 13, OUTPUT);
    std::cout << "MODER=" << hex(gpioa.MODER) << '\n';
    write(gpioa, 5, true);
    std::cout << "ODR after set 5: " << hex(gpioa.ODR) << '\n';
    toggle(gpioa, 13);
    std::cout << "ODR after toggle 13: " << hex(gpioa.ODR) << '\n';
    toggle(gpioa, 5);
    std::cout << "ODR after toggle 5: " << hex(gpioa.ODR) << '\n';
    gpioa.IDR = 0x1;
    std::cout << "pin 0 reads: " << read(gpioa, 0) << '\n';
    std::cout << std::boolalpha << "pin 5 is output: " << is_output(gpioa, 5) << ", pin 6 is output: " << is_output(gpioa, 6) << '\n';
}

@@exercise An ISR ring buffer and a debouncer
Two classic interrupt-side tools:

1. `RingBuffer<T, N>`: a lock-free single-producer/single-consumer queue for an ISR. `N` must be a power of two (`static_assert`); `head` and `tail` are free-running `std::atomic<uint32_t>` counters, the index is `counter & (N - 1)`, and the size is `head - tail` (correct even when the counters wrap). `push` returns false when full (an ISR can't wait, so the byte is dropped).
2. `Debouncer(stable_samples)`: `sample(raw)` returns the new state when the raw input has differed from the current state for `stable_samples` consecutive samples; any sample equal to the state resets the count.

Simulate a UART burst `HELLO`, pop 3 bytes, a second burst ` WORLD\n` that overflows the 8-byte buffer, pop everything, then debounce a bouncy button.
@@starter
#include <atomic>
#include <cstdint>
#include <iostream>
#include <optional>
#include <string>
#include <vector>

int main() {
    std::vector<int> samples = {0, 1, 0, 1, 1, 1, 1, 0, 1, 1, 1, 1, 0, 0, 0, 0};
    std::cout << "TODO\n";
}
@@expected
popped: HEL
dropped in burst 2: 1
popped: LO WORLD
size now: 0, capacity: 8
pressed at sample 5
released at sample 14
@@requires
static_assert
std::atomic<uint32_t>
& (N - 1)
@@hint
`push`: `uint32_t h = head_.load(std::memory_order_relaxed), t = tail_.load(std::memory_order_acquire); if (h - t == N) return false; buf_[h & (N - 1)] = v; head_.store(h + 1, std::memory_order_release);`. `pop` mirrors it. The `\n` printed at the end of `LO WORLD` is dropped because the buffer is full, so there's no newline in the output.
@@solution
#include <array>
#include <atomic>
#include <cstdint>
#include <iostream>
#include <optional>
#include <string>
#include <vector>

template <class T, size_t N>
class RingBuffer {
    static_assert(N > 0 && (N & (N - 1)) == 0, "N must be a power of two");

public:
    bool push(T value) {
        uint32_t h = head_.load(std::memory_order_relaxed);
        uint32_t t = tail_.load(std::memory_order_acquire);
        if (h - t == N) return false;
        buf_[h & (N - 1)] = value;
        head_.store(h + 1, std::memory_order_release);
        return true;
    }

    std::optional<T> pop() {
        uint32_t t = tail_.load(std::memory_order_relaxed);
        uint32_t h = head_.load(std::memory_order_acquire);
        if (h == t) return std::nullopt;
        T value = buf_[t & (N - 1)];
        tail_.store(t + 1, std::memory_order_release);
        return value;
    }

    size_t size() const { return head_.load() - tail_.load(); }
    static constexpr size_t capacity() { return N; }

private:
    std::array<T, N> buf_{};
    std::atomic<uint32_t> head_{0}, tail_{0};
};

class Debouncer {
public:
    explicit Debouncer(int stable_samples) : needed_(stable_samples) {}

    std::optional<bool> sample(bool raw) {
        if (raw == state_) {
            count_ = 0;
            return std::nullopt;
        }
        if (++count_ >= needed_) {
            state_ = raw;
            count_ = 0;
            return state_;
        }
        return std::nullopt;
    }

private:
    int needed_;
    int count_ = 0;
    bool state_ = false;
};

int main() {
    RingBuffer<char, 8> uart;
    auto isr_burst = [&](const std::string& bytes) {
        int dropped = 0;
        for (char c : bytes)
            if (!uart.push(c)) ++dropped;
        return dropped;
    };
    auto pop = [&](size_t n) {
        std::string out;
        while (out.size() < n)
            if (auto c = uart.pop()) out += *c;
            else break;
        return out;
    };

    isr_burst("HELLO");
    std::cout << "popped: " << pop(3) << '\n';
    std::cout << "dropped in burst 2: " << isr_burst(" WORLD\n") << '\n';
    std::cout << "popped: " << pop(100) << '\n';
    std::cout << "size now: " << uart.size() << ", capacity: " << uart.capacity() << '\n';

    std::vector<int> samples = {0, 1, 0, 1, 1, 1, 1, 0, 1, 1, 1, 1, 0, 0, 0, 0};
    Debouncer button(3);
    for (size_t i = 0; i < samples.size(); ++i)
        if (auto state = button.sample(samples[i] != 0))
            std::cout << (*state ? "pressed" : "released") << " at sample " << i << '\n';
}
