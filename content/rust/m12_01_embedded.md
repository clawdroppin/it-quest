@@lesson m12-embedded 🎓 Beyond IT: Embedded Rust and Microcontrollers
@@text
> **Beyond IT scope.** This module goes past typical IT, administration and business software into electronics and firmware engineering. It's here so the Rust course covers the whole language, and because IoT devices increasingly end up on IT's network. Skip it freely if your goal is an IT job.

Rust runs on microcontrollers: tiny chips with kilobytes of RAM, no operating system and no heap, controlling sensors, motors and radios. Memory safety matters even more here, because a buffer overflow in a pacemaker or a car's brake controller is far worse than in a web page.

## `no_std`: Rust without the standard library

```rust,norun
#![no_std]          // no std: no heap, files, threads or println!
#![no_main]         // no normal main; the runtime crate provides the entry point

use cortex_m_rt::entry;
use panic_halt as _;   // what to do on panic: stop the chip

#[entry]
fn main() -> ! {       // `!` = never returns: firmware runs forever
    loop {}
}
```

- **core** is still available: `Option`, `Result`, iterators, slices, `str`, formatting traits, atomics.
- **alloc** (`Vec`, `String`, `Box`) only if you provide a global allocator; most firmware avoids the heap entirely and uses fixed-size buffers: the `heapless` crate offers `heapless::Vec<T, 16>` and `heapless::String<64>` with capacity in the type.
- **Const generics** (`[T; N]` with `N` a type parameter) make fixed-capacity data structures natural (the second exercise).

## The layers of embedded Rust

| Layer | Example crates | What it does |
|---|---|---|
| **PAC** (peripheral access crate) | `stm32f4`, `rp2040-pac` | generated from the chip's SVD file: every register and bit field, typed |
| **HAL** (hardware abstraction layer) | `stm32f4xx-hal`, `rp2040-hal`, `esp-hal`, `nrf-hal` | safe APIs: "set pin 5 as output", "start SPI at 1 MHz" |
| **embedded-hal** traits | `embedded-hal` 1.0 | standard traits (`OutputPin`, `SpiDevice`, `I2c`) so drivers work on any chip |
| **Drivers** | `ssd1306` display, `bme280` sensor | written once against embedded-hal |
| **Board support / frameworks** | `embassy` (async), RTIC (interrupt-driven) | scheduling, timers, networking |

```rust,norun
use embedded_hal::digital::OutputPin;

fn blink<P: OutputPin>(led: &mut P, delay: &mut impl embedded_hal::delay::DelayNs) {
    loop {
        led.set_high().ok();
        delay.delay_ms(500);
        led.set_low().ok();
        delay.delay_ms(500);
    }
}
```

**Embassy** brings `async`/`await` to microcontrollers: each task is a state machine, the executor sleeps the CPU until an interrupt wakes it, with no heap and no RTOS.

## Registers and bit manipulation

Hardware is controlled through **memory-mapped registers**: writing bits at a specific address turns on a clock, configures a pin or starts a transfer. A typical register packs several **fields**:

```text
GPIO MODER register (32 bits): 2 bits per pin, pins 0–15
bits 31..30  29..28  ...   3..2    1..0
     pin15   pin14         pin1    pin0       00 input, 01 output, 10 alternate, 11 analog
```

The core operations, all on integers:

- set bits: `reg |= mask`
- clear bits: `reg &= !mask`
- read a field: `(reg >> shift) & ((1 << width) - 1)`
- write a field: clear it, then OR in the new value shifted into place

Real register access uses `core::ptr::read_volatile`/`write_volatile` (the compiler must not optimise them away or reorder them); PACs wrap this in safe methods like `gpioa.moder.modify(|_, w| w.moder5().output())`. The first exercise implements field operations on a plain `u32`.

## Interrupts and shared state

An **interrupt** pauses `main` to run a handler when hardware needs attention (a byte arrived, a timer fired). Data shared between them needs protection: `critical_section::Mutex<RefCell<T>>`, atomics, or a framework like RTIC that proves at compile time that resources aren't shared unsafely. Handlers must be short; they usually push data into a **ring buffer** that `main` drains.

## Tooling

- `rustup target add thumbv7em-none-eabihf` (Cortex-M4F), `riscv32imac-unknown-none-elf`, and so on.
- **probe-rs** flashes and debugs over SWD/JTAG: `cargo run` flashes the board, `defmt` provides tiny, fast logging over the debug probe.
- Popular boards to start with: Raspberry Pi Pico (RP2040/RP2350), ESP32-C3/C6 (Wi-Fi, RISC-V, `esp-hal`), STM32 Nucleo, BBC micro:bit (used by the official *Discovery* book).
- Read *The Embedded Rust Book* and the Embassy book next.

@@quiz
What does `#![no_std]` remove?
- The core library
* The standard library: heap collections, files, threads, println! and other OS services
- Iterators
- The type system
? `core` remains available.

@@quiz
What is the role of the embedded-hal crate?
- It flashes the chip
* It defines standard hardware traits so drivers can work across many microcontrollers
- It's an operating system
- It generates register code from SVD files
? PACs are generated from SVD; HALs implement embedded-hal.

@@quiz
How do you clear bits `mask` in a register value `reg`?
- `reg |= mask`
* `reg &= !mask`
- `reg ^= mask`
- `reg = mask`
? AND with the inverted mask keeps every other bit.

@@quiz
Why are volatile reads and writes needed for hardware registers?
* So the compiler doesn't remove, merge or reorder accesses that have side effects on hardware
- They are faster
- They allocate memory
- They're required for floats
? A register write can start a motor; it must happen exactly as written.

@@drill
Which crate-level attribute builds without the standard library?
= #![no_std]
= no_std
? Microcontrollers have no operating system.

@@flashcards
no_std :: Rust without the standard library, only `core` (and optionally `alloc`)
PAC :: peripheral access crate generated from a chip's SVD file
HAL :: hardware abstraction layer with safe peripheral APIs
embedded-hal :: standard traits for pins, SPI, I2C, delays
Embassy :: async/await framework for microcontrollers
Volatile access :: memory access the compiler must perform exactly as written
heapless :: fixed-capacity collections without an allocator
probe-rs :: tool to flash and debug microcontrollers

@@exercise Register fields
Write bit-field helpers and use them to configure a GPIO mode register (2 bits per pin).

- `read_field(reg, shift, width) -> u32`
- `write_field(reg, shift, width, value) -> u32`: returns the new register value with only that field changed; values too large for the field are masked to fit.
- `set_mode(reg, pin, mode) -> u32` and `mode_of(reg, pin) -> Mode`, where `Mode` is `Input` (0), `Output` (1), `Alternate` (2), `Analog` (3).
@@starter
#[derive(Debug, Clone, Copy, PartialEq)]
enum Mode {
    Input = 0,
    Output = 1,
    Alternate = 2,
    Analog = 3,
}

fn read_field(reg: u32, shift: u32, width: u32) -> u32 {
    todo!()
}

fn write_field(reg: u32, shift: u32, width: u32, value: u32) -> u32 {
    todo!()
}

fn set_mode(reg: u32, pin: u32, mode: Mode) -> u32 {
    todo!()
}

fn mode_of(reg: u32, pin: u32) -> Mode {
    todo!()
}

fn main() {
    let mut moder: u32 = 0xFFFF_FFFF;   // after reset: every pin analog
    moder = set_mode(moder, 5, Mode::Output);
    moder = set_mode(moder, 0, Mode::Input);
    moder = set_mode(moder, 15, Mode::Alternate);
    println!("{moder:#010x}");
    println!("{:?} {:?} {:?} {:?}", mode_of(moder, 5), mode_of(moder, 0), mode_of(moder, 15), mode_of(moder, 7));
}
@@expected
0xbffff7fc
Output Input Alternate Analog
@@tests
#[test]
fn checks() {
    assert_eq!(read_field(0b1011_0000, 4, 4), 0b1011);
    assert_eq!(write_field(0, 8, 4, 0xFF), 0xF00, "too-large values are masked");
    assert_eq!(write_field(0xFFFF_FFFF, 0, 32, 5), 5, "a full-width field works");
    assert_eq!(read_field(0x8000_0000, 31, 1), 1);
}
@@requires
fn write_field
match
@@hint
A mask of `width` ones: careful, `1u32 << 32` overflows, so use `if width == 32 { u32::MAX } else { (1 << width) - 1 }`. Then `(reg & !(mask << shift)) | ((value & mask) << shift)`. Pin `n` has `shift = n * 2`, width 2. Convert back with a `match` on the 2-bit value.
@@solution
#[derive(Debug, Clone, Copy, PartialEq)]
enum Mode {
    Input = 0,
    Output = 1,
    Alternate = 2,
    Analog = 3,
}

fn mask(width: u32) -> u32 {
    if width >= 32 { u32::MAX } else { (1 << width) - 1 }
}

fn read_field(reg: u32, shift: u32, width: u32) -> u32 {
    (reg >> shift) & mask(width)
}

fn write_field(reg: u32, shift: u32, width: u32, value: u32) -> u32 {
    let m = mask(width);
    (reg & !(m << shift)) | ((value & m) << shift)
}

fn set_mode(reg: u32, pin: u32, mode: Mode) -> u32 {
    write_field(reg, pin * 2, 2, mode as u32)
}

fn mode_of(reg: u32, pin: u32) -> Mode {
    match read_field(reg, pin * 2, 2) {
        0 => Mode::Input,
        1 => Mode::Output,
        2 => Mode::Alternate,
        _ => Mode::Analog,
    }
}

fn main() {
    let mut moder: u32 = 0xFFFF_FFFF;
    moder = set_mode(moder, 5, Mode::Output);
    moder = set_mode(moder, 0, Mode::Input);
    moder = set_mode(moder, 15, Mode::Alternate);
    println!("{moder:#010x}");
    println!("{:?} {:?} {:?} {:?}", mode_of(moder, 5), mode_of(moder, 0), mode_of(moder, 15), mode_of(moder, 7));
}

@@exercise A fixed-capacity ring buffer
Interrupt handlers push received bytes into a ring buffer that the main loop drains. Build one that never allocates: storage is `[T; N]` with `N` a const generic.

- `RingBuffer::<T, N>::new()` (requires `T: Copy + Default`).
- `push(&mut self, v) -> Result<(), T>`: `Err(v)` (giving the value back) when full.
- `pop(&mut self) -> Option<T>`: oldest first.
- `len`, `is_full`.
- `push_overwrite(&mut self, v)`: when full, drop the oldest value to make room (for "latest N sensor readings").
@@starter
struct RingBuffer<T, const N: usize> {
    buf: [T; N],
    head: usize, // index of the oldest element
    len: usize,
}

impl<T: Copy + Default, const N: usize> RingBuffer<T, N> {
    fn new() -> Self {
        todo!()
    }

    fn push(&mut self, v: T) -> Result<(), T> {
        todo!()
    }

    fn pop(&mut self) -> Option<T> {
        todo!()
    }

    fn len(&self) -> usize {
        self.len
    }

    fn is_full(&self) -> bool {
        self.len == N
    }

    fn push_overwrite(&mut self, v: T) {
        todo!()
    }
}

fn main() {
    let mut uart: RingBuffer<u8, 4> = RingBuffer::new();
    for b in b"HELLO" {
        if let Err(lost) = uart.push(*b) {
            println!("overrun, lost {:?}", lost as char);
        }
    }
    let mut got = String::new();
    while let Some(b) = uart.pop() {
        got.push(b as char);
    }
    println!("{got}");

    let mut temps: RingBuffer<i16, 3> = RingBuffer::new();
    for t in [210, 215, 220, 230, 245] {
        temps.push_overwrite(t);
    }
    let mut last = vec![];
    while let Some(t) = temps.pop() {
        last.push(t);
    }
    println!("{last:?}");
}
@@expected
overrun, lost 'O'
HELL
[220, 230, 245]
@@tests
#[test]
fn checks() {
    let mut r: RingBuffer<u32, 2> = RingBuffer::new();
    assert_eq!(r.pop(), None);
    for round in 0..10 {
        assert_eq!(r.push(round), Ok(()));
        assert_eq!(r.push(round + 100), Ok(()));
        assert!(r.is_full());
        assert_eq!(r.pop(), Some(round), "wraps around correctly");
        assert_eq!(r.pop(), Some(round + 100));
        assert_eq!(r.len(), 0);
    }
    assert_eq!(std::mem::size_of::<RingBuffer<u8, 16>>(), 16 + 2 * std::mem::size_of::<usize>());
}
@@requires
const N: usize
T::default(
@@hint
`new`: `RingBuffer { buf: [T::default(); N], head: 0, len: 0 }`. The slot after the newest element is `(self.head + self.len) % N`. `pop` reads `self.buf[self.head]`, then `self.head = (self.head + 1) % N; self.len -= 1`. `push_overwrite`: if full, `self.pop();` first, then `push`.
@@solution
struct RingBuffer<T, const N: usize> {
    buf: [T; N],
    head: usize,
    len: usize,
}

impl<T: Copy + Default, const N: usize> RingBuffer<T, N> {
    fn new() -> Self {
        RingBuffer { buf: [T::default(); N], head: 0, len: 0 }
    }

    fn push(&mut self, v: T) -> Result<(), T> {
        if self.len == N {
            return Err(v);
        }
        self.buf[(self.head + self.len) % N] = v;
        self.len += 1;
        Ok(())
    }

    fn pop(&mut self) -> Option<T> {
        if self.len == 0 {
            return None;
        }
        let v = self.buf[self.head];
        self.head = (self.head + 1) % N;
        self.len -= 1;
        Some(v)
    }

    fn len(&self) -> usize {
        self.len
    }

    fn is_full(&self) -> bool {
        self.len == N
    }

    fn push_overwrite(&mut self, v: T) {
        if self.is_full() {
            self.pop();
        }
        let _ = self.push(v);
    }
}

fn main() {
    let mut uart: RingBuffer<u8, 4> = RingBuffer::new();
    for b in b"HELLO" {
        if let Err(lost) = uart.push(*b) {
            println!("overrun, lost {:?}", lost as char);
        }
    }
    let mut got = String::new();
    while let Some(b) = uart.pop() {
        got.push(b as char);
    }
    println!("{got}");

    let mut temps: RingBuffer<i16, 3> = RingBuffer::new();
    for t in [210, 215, 220, 230, 245] {
        temps.push_overwrite(t);
    }
    let mut last = vec![];
    while let Some(t) = temps.pop() {
        last.push(t);
    }
    println!("{last:?}");
}
