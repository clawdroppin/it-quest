@@lesson b9-tinygo 🎓 Beyond IT: Microcontrollers and WebAssembly with TinyGo
@@concepts loops, functions
@@text
> **Beyond IT scope.** This module goes past IT and business software into embedded systems, game development and generative art. They're great fun and deepen your understanding of how computers really work, but they're not needed for IT jobs. Skip them freely if that's your goal.

**TinyGo** is a Go compiler built on LLVM for small places: microcontrollers with a few KB of RAM, WebAssembly modules and WASI. It compiles a large subset of Go, including goroutines, channels and most of the standard library.

## Blinky: the "hello world" of hardware

```go,norun
package main

import (
	"machine"
	"time"
)

func main() {
	led := machine.LED
	led.Configure(machine.PinConfig{Mode: machine.PinOutput})
	button := machine.GP15
	button.Configure(machine.PinConfig{Mode: machine.PinInputPullup})
	for {
		if !button.Get() { // pull-up: pressed reads low
			led.High()
		} else {
			led.Low()
		}
		time.Sleep(10 * time.Millisecond)
	}
}
```

```text
tinygo flash -target=pico .        # Raspberry Pi Pico: build and flash over USB
tinygo flash -target=arduino-nano33 .
tinygo monitor                     # serial console for println debugging
```

Supported boards include the Raspberry Pi Pico/Pico 2 (RP2040/RP2350), Arduino Nano 33 IoT, Adafruit Feather boards, BBC micro:bit, ESP32-C3 and many more. `tinygo.org/x/drivers` has drivers for sensors, displays, LEDs and radios, over I²C, SPI, UART and PWM.

## Thinking small

- **Memory is tiny** (264 KB on a Pico, 2 KB on an Uno): avoid allocations in loops, prefer fixed-size arrays, keep the GC idle.
- **No floating-point unit** on many chips: use **fixed-point integers** (e.g. tenths of a degree) instead of floats.
- **Real signals are noisy**: mechanical buttons "bounce" for several milliseconds, and sensors spike. Debounce and filter in software.
- **Interrupts** (`pin.SetInterrupt`) react instantly to edges; keep handlers tiny and hand work off through channels or flags.

## WebAssembly

```text
tinygo build -o app.wasm -target=wasm ./web      # browser, much smaller than the standard Go wasm output
tinygo build -o plugin.wasm -target=wasip1 .     # WASI: runs in wasmtime, Spin, Envoy filters, databases
```

TinyGo's small WASM binaries make Go practical for browser modules and plugin systems.

@@quiz
Why use integers for tenths of a degree instead of float64 on a microcontroller?
* Many chips have no FPU, so floats are slow software emulation and use more flash
- Floats are forbidden in TinyGo
- Integers are more precise in every case
- Sensors only return integers
? Fixed-point maths is the embedded standard.

@@quiz
What is switch bounce?
- A software bug
* A mechanical switch rapidly flickering between on and off for a few milliseconds when pressed or released
- Electrical damage
- A TinyGo error
? One press can look like many presses without debouncing.

@@quiz
Why is TinyGo used instead of the standard Go compiler for microcontrollers?
* It produces much smaller binaries and supports bare-metal targets with no operating system
- It's faster to type
- Standard Go has no goroutines
- TinyGo is an interpreter
? A microcontroller may only have tens of kilobytes of RAM.

@@drill
What's the usual "hello world" program for a microcontroller called?
= blinky
= blink
? Toggle an LED in a loop.

@@flashcards
TinyGo :: LLVM-based Go compiler for small targets
`machine` package :: pins, I²C, SPI, UART on the board
`tinygo flash -target=pico` :: build and flash
Pull-up input :: reads high until pressed to ground
Debouncing :: ignore short flickers
Fixed-point :: integers scaled by a factor
ADC :: analog-to-digital converter
`-target=wasip1` :: WASI module

@@exercise Debouncing a button
A button is sampled every 1 ms. The raw samples bounce. Write:

- `type Debouncer struct` with `Update(sample bool) (changed bool, state bool)`: the debounced state only changes after **5 consecutive** samples that differ from the current state (any sample equal to the state resets the count); it starts released (`false`)
- on every debounced **press**, cycle an LED mode: `off → on → blink → off`

The samples are built from `(level, count)` segments (provided). Print each debounced change with its time in ms, then the number of raw transitions vs debounced changes. Use no allocations in the update loop.
@@starter
package main

import "fmt"

type Debouncer struct {
	state bool
	count int
}

func (d *Debouncer) Update(sample bool) (bool, bool) {
	return false, d.state
}

type segment struct {
	level bool
	n     int
}

func main() {
	segs := []segment{
		{false, 10}, {true, 1}, {false, 1}, {true, 1}, {false, 1}, {true, 20}, // press with bounce
		{false, 2}, {true, 2}, {false, 30}, // release with bounce
		{true, 8}, {false, 10}, // a quick clean press
		{true, 3}, {false, 20}, // a 3 ms glitch: must be ignored
		{true, 30}, {false, 5},
	}
	var samples []bool
	for _, s := range segs {
		for range s.n {
			samples = append(samples, s.level)
		}
	}
	modes := [3]string{"off", "on", "blink"}
	_ = modes
	fmt.Println(len(samples))
}
@@expected
t=18ms pressed -> on
t=42ms released
t=72ms pressed -> blink
t=80ms released
t=113ms pressed -> off
t=143ms released
raw transitions: 14, debounced changes: 6
@@requires
Update(
@@hint
In `Update`: `if sample == d.state { d.count = 0; return false, d.state }`, otherwise increment and flip the state when the count reaches 5 (reset the count). Keep `mode := 0` and advance it with `(mode + 1) % 3` on presses. Count raw transitions by comparing each sample with the previous one.
@@solution
package main

import "fmt"

type Debouncer struct {
	state bool
	count int
}

func (d *Debouncer) Update(sample bool) (bool, bool) {
	if sample == d.state {
		d.count = 0
		return false, d.state
	}
	d.count++
	if d.count >= 5 {
		d.state = sample
		d.count = 0
		return true, d.state
	}
	return false, d.state
}

type segment struct {
	level bool
	n     int
}

func main() {
	segs := []segment{
		{false, 10}, {true, 1}, {false, 1}, {true, 1}, {false, 1}, {true, 20}, // press with bounce
		{false, 2}, {true, 2}, {false, 30}, // release with bounce
		{true, 8}, {false, 10}, // a quick clean press
		{true, 3}, {false, 20}, // a 3 ms glitch: must be ignored
		{true, 30}, {false, 5},
	}
	var samples []bool
	for _, s := range segs {
		for range s.n {
			samples = append(samples, s.level)
		}
	}
	modes := [3]string{"off", "on", "blink"}
	mode := 0
	var d Debouncer
	raw, changes := 0, 0
	for t, s := range samples {
		if t > 0 && s != samples[t-1] {
			raw++
		}
		changed, pressed := d.Update(s)
		if !changed {
			continue
		}
		changes++
		if pressed {
			mode = (mode + 1) % 3
			fmt.Printf("t=%dms pressed -> %s\n", t, modes[mode])
		} else {
			fmt.Printf("t=%dms released\n", t)
		}
	}
	fmt.Printf("raw transitions: %d, debounced changes: %d\n", raw, changes)
}

@@exercise A temperature sensor in fixed point
A TMP36 sensor outputs 500 mV at 0 °C plus 10 mV per °C, read by a 12-bit ADC (0–4095) with a 3.3 V reference. Using **integers only**:

- `MilliVolts(raw int) int` = `raw * 3300 / 4095`
- `Tenths(mv int) int`: temperature in tenths of a degree (`mv - 500`, since 1 mV = 0.1 °C)
- `Format(t int) string`: tenths as `23.4`, handling negatives (`-0.5`, `-42.0`)
- a `Filter` with a fixed `[4]int` ring buffer: `Add(t int) (avg int, ok bool)` rejects a reading more than 20.0 °C (200 tenths) away from the current average (once there is one), otherwise adds it and returns the average of up to the last 4 accepted readings (integer division)
@@starter
package main

import "fmt"

type Filter struct {
	buf   [4]int
	n     int
	next  int
}

func main() {
	raws := []int{906, 910, 915, 2048, 908, 100}
	_ = raws
	fmt.Println("TODO")
}
@@expected
raw=906 mV=730 temp=23.0 avg=23.0
raw=910 mV=733 temp=23.3 avg=23.1
raw=915 mV=737 temp=23.7 avg=23.3
raw=2048 mV=1650 temp=115.0 rejected
raw=908 mV=731 temp=23.1 avg=23.2
raw=100 mV=80 temp=-42.0 rejected
format check: -0.5 0.0 12.3
@@hint
Keep the current average in the filter (or recompute it from `buf[:n]`). Format: `sign := ""; if t < 0 { sign = "-"; t = -t }; return fmt.Sprintf("%s%d.%d", sign, t/10, t%10)`.
@@solution
package main

import "fmt"

func MilliVolts(raw int) int { return raw * 3300 / 4095 }

func Tenths(mv int) int { return mv - 500 }

func Format(t int) string {
	sign := ""
	if t < 0 {
		sign, t = "-", -t
	}
	return fmt.Sprintf("%s%d.%d", sign, t/10, t%10)
}

type Filter struct {
	buf  [4]int
	n    int
	next int
}

func (f *Filter) avg() int {
	sum := 0
	for _, v := range f.buf[:f.n] {
		sum += v
	}
	return sum / f.n
}

func (f *Filter) Add(t int) (int, bool) {
	if f.n > 0 {
		d := t - f.avg()
		if d > 200 || d < -200 {
			return 0, false
		}
	}
	f.buf[f.next] = t
	f.next = (f.next + 1) % len(f.buf)
	f.n = min(f.n+1, len(f.buf))
	return f.avg(), true
}

func main() {
	raws := []int{906, 910, 915, 2048, 908, 100}
	var f Filter
	for _, raw := range raws {
		mv := MilliVolts(raw)
		t := Tenths(mv)
		avg, ok := f.Add(t)
		if !ok {
			fmt.Printf("raw=%d mV=%d temp=%s rejected\n", raw, mv, Format(t))
			continue
		}
		fmt.Printf("raw=%d mV=%d temp=%s avg=%s\n", raw, mv, Format(t), Format(avg))
	}
	fmt.Println("format check:", Format(-5), Format(0), Format(123))
}
