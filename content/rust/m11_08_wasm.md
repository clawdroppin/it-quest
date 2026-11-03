@@lesson m11-wasm Rust and WebAssembly: the Browser, Front-End Frameworks and Tauri
@@text
**WebAssembly (Wasm)** is a compact binary instruction format that browsers (and servers) run at near-native speed in a sandbox. Rust is the leading language for it: no garbage collector, small binaries, and great tooling. This lesson covers how Rust reaches the browser and when it's worth it.

## What Wasm is (and isn't)

- A **stack-based virtual machine** with a few number types (`i32`, `i64`, `f32`, `f64`) and one big **linear memory**: a resizable array of bytes.
- Wasm code can't touch the DOM, network or files directly; it calls **imported** JavaScript functions, and JavaScript calls its **exported** functions.
- Only numbers cross the boundary. Strings and structs are passed as **(pointer, length)** into linear memory, which the tooling hides from you (the second exercise shows what happens underneath).
- Good for: heavy computation (image/video processing, compression, parsing, physics, crypto), porting existing Rust/C++ libraries to the web, plugins in sandboxes. Not automatically faster for ordinary DOM-heavy UI code.

## The toolchain

```text
rustup target add wasm32-unknown-unknown
cargo install wasm-pack              # builds, runs wasm-bindgen, makes an npm package
wasm-pack build --target web         # outputs pkg/ with .wasm + JS glue + .d.ts types
```

```rust,norun
// Cargo.toml: [lib] crate-type = ["cdylib"]; wasm-bindgen = "0.2"
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn checksum(data: &[u8]) -> u32 {
    data.iter().fold(0u32, |acc, &b| acc.rotate_left(5) ^ b as u32)
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}
```

```js,norun
import init, { checksum } from "./pkg/mylib.js";
await init();
console.log(checksum(new Uint8Array(await file.arrayBuffer())));
```

- **wasm-bindgen** generates the glue that copies strings, slices and structs between JS and linear memory.
- **web-sys** and **js-sys** expose browser APIs (DOM, fetch, canvas, WebGL/WebGPU) to Rust.
- **wasm-opt** (Binaryen) and `opt-level = "z"` + `lto = true` shrink the output.
- `console_error_panic_hook` turns Rust panics into readable browser console errors.

## Front-end frameworks in Rust

| Framework | Style |
|---|---|
| **Leptos** | fine-grained reactivity (signals), server functions, SSR + hydration |
| **Dioxus** | React-like components; targets web, desktop and mobile |
| **Yew** | the older React-like option with a large example base |
| **Sycamore** | signals, similar to SolidJS |

```rust,norun
#[component]
fn Counter() -> impl IntoView {
    let (count, set_count) = signal(0);
    view! { <button on:click=move |_| set_count.update(|n| *n += 1)>"Clicks: " {count}</button> }
}
```

Built with `trunk serve` (Yew/Leptos client-side) or `cargo leptos watch` (full-stack).

## Tauri: Rust back end, web front end

**Tauri** (the framework this app is built with) runs a web UI (any framework, or Leptos/Yew in Rust) in the operating system's own webview, with a Rust back end. The front end calls Rust **commands**:

```rust,norun
#[tauri::command]
fn read_note(name: String) -> Result<String, String> {
    std::fs::read_to_string(format!("notes/{name}.md")).map_err(|e| e.to_string())
}
// JS: const text = await invoke("read_note", { name: "todo" });
```

Apps are a few megabytes instead of Electron's 100+, and Tauri 2 also builds for Android and iOS. Capabilities files restrict which commands and APIs each window may use.

## Wasm outside the browser

- **WASI** gives Wasm modules a standard system interface (files, clocks, sockets) so they run in **Wasmtime**, **Wasmer** or **WasmEdge**.
- **Plugins**: apps embed Wasmtime to run untrusted extensions safely (Zed editor, Envoy proxy filters, Extism, Shopify functions).
- **Edge and serverless**: Cloudflare Workers, Fastly Compute and Fermyon Spin run Wasm with millisecond cold starts.
- **The Component Model** (WIT interface files, `cargo component`) lets Wasm modules written in different languages call each other with rich types.

The first exercise is the classic Rust + Wasm project: Conway's Game of Life stored as a flat array, the layout JavaScript can read directly from linear memory.

@@quiz
How are strings passed between JavaScript and Wasm?
- As native strings, Wasm understands UTF-16
* As a pointer and length into Wasm linear memory, copied by glue code like wasm-bindgen
- They can't be passed
- As JSON over HTTP
? Only numbers cross the boundary directly.

@@quiz
Which Rust crate exposes browser APIs like the DOM and fetch?
- tokio
* web-sys
- serde
- clap
? js-sys covers JavaScript built-ins; web-sys covers Web APIs.

@@quiz
What is WASI?
- A JavaScript framework
* A standard system interface so Wasm modules can run outside the browser with files, clocks and sockets
- A Rust compiler flag
- A browser extension
? Runtimes like Wasmtime implement it.

@@quiz
How does Tauri differ from Electron?
* It uses the operating system's webview and a Rust back end, so apps are much smaller
- It bundles Chromium
- It only works on Linux
- It can't use JavaScript
? Electron ships its own Chromium and Node.js.

@@flashcards
WebAssembly :: portable, sandboxed binary instruction format run by browsers and runtimes
Linear memory :: the single resizable byte array a Wasm module uses
wasm-bindgen :: generates glue between Rust and JavaScript
wasm-pack :: builds a Rust crate into an npm-ready Wasm package
web-sys :: Rust bindings to browser Web APIs
WASI :: system interface for Wasm outside the browser
Leptos :: Rust web framework with signals and server functions
Tauri :: desktop/mobile apps with a web UI and Rust back end

@@exercise The Game of Life universe
Store the grid as one flat `Vec<u8>` (0 = dead, 1 = alive), row by row, the layout a Wasm module exposes to JavaScript.

- `Universe::from_rows(rows: &[&str])`: `#` alive, `.` dead; all rows have the same width.
- `index(row, col) -> usize`.
- `live_neighbours(row, col) -> u8`: the 8 surrounding cells, **wrapping** around the edges (a torus).
- `tick(&mut self)`: the next generation: a live cell with 2 or 3 live neighbours lives; a dead cell with exactly 3 becomes alive; everything else dies or stays dead.
- `render(&self) -> String`: rows of `#`/`.` joined by `\n`.
@@starter
struct Universe {
    width: usize,
    height: usize,
    cells: Vec<u8>,
}

impl Universe {
    fn from_rows(rows: &[&str]) -> Universe {
        todo!()
    }

    fn index(&self, row: usize, col: usize) -> usize {
        todo!()
    }

    fn live_neighbours(&self, row: usize, col: usize) -> u8 {
        todo!()
    }

    fn tick(&mut self) {
        todo!()
    }

    fn render(&self) -> String {
        todo!()
    }
}

fn main() {
    let mut u = Universe::from_rows(&[".....", "..#..", "..#..", "..#..", "....."]);
    u.tick();
    println!("{}\n", u.render());
    u.tick();
    println!("{}", u.render());
}
@@expected
.....
.....
.###.
.....
.....

.....
..#..
..#..
..#..
.....
@@tests
#[test]
fn checks() {
    let u = Universe::from_rows(&["#...", "....", "....", "...#"]);
    assert_eq!(u.index(3, 3), 15);
    assert_eq!(u.live_neighbours(0, 0), 1, "wraps to the opposite corner");
    let mut block = Universe::from_rows(&["....", ".##.", ".##.", "...."]);
    let before = block.render();
    block.tick();
    assert_eq!(block.render(), before, "a block is stable");
}
@@requires
fn tick
@@hint
For wrapping neighbours, add `height - 1`, `0` or `1` instead of subtracting: `for dr in [self.height - 1, 0, 1] { for dc in [self.width - 1, 0, 1] { if dr == 0 && dc == 0 { continue; } let r = (row + dr) % self.height; let c = (col + dc) % self.width; n += self.cells[self.index(r, c)]; } }`. Build the next generation in a new `Vec` and assign it at the end.
@@solution
struct Universe {
    width: usize,
    height: usize,
    cells: Vec<u8>,
}

impl Universe {
    fn from_rows(rows: &[&str]) -> Universe {
        let width = rows.first().map_or(0, |r| r.len());
        let cells = rows.iter().flat_map(|r| r.bytes().map(|b| (b == b'#') as u8)).collect();
        Universe { width, height: rows.len(), cells }
    }

    fn index(&self, row: usize, col: usize) -> usize {
        row * self.width + col
    }

    fn live_neighbours(&self, row: usize, col: usize) -> u8 {
        let mut n = 0;
        for dr in [self.height - 1, 0, 1] {
            for dc in [self.width - 1, 0, 1] {
                if dr == 0 && dc == 0 {
                    continue;
                }
                let r = (row + dr) % self.height;
                let c = (col + dc) % self.width;
                n += self.cells[self.index(r, c)];
            }
        }
        n
    }

    fn tick(&mut self) {
        let mut next = self.cells.clone();
        for row in 0..self.height {
            for col in 0..self.width {
                let i = self.index(row, col);
                let n = self.live_neighbours(row, col);
                next[i] = match (self.cells[i], n) {
                    (1, 2) | (1, 3) => 1,
                    (0, 3) => 1,
                    _ => 0,
                };
            }
        }
        self.cells = next;
    }

    fn render(&self) -> String {
        self.cells
            .chunks(self.width)
            .map(|row| row.iter().map(|&c| if c == 1 { '#' } else { '.' }).collect::<String>())
            .collect::<Vec<_>>()
            .join("\n")
    }
}

fn main() {
    let mut u = Universe::from_rows(&[".....", "..#..", "..#..", "..#..", "....."]);
    u.tick();
    println!("{}\n", u.render());
    u.tick();
    println!("{}", u.render());
}

@@exercise Strings through linear memory
Simulate what wasm-bindgen does when JavaScript passes a string to Rust and back. `Memory` is a byte array with a **bump allocator** (`next` is the first free offset).

- `alloc(&mut self, len) -> Result<usize, String>`: returns the current `next` and advances it by `len`; if that would go past the end, `Err("out of memory")`.
- `write_str(&mut self, s) -> Result<(usize, usize), String>`: allocate, copy the UTF-8 bytes in, return `(ptr, len)`.
- `read_str(&self, ptr, len) -> Result<String, String>`: bounds-check (`Err("out of bounds")`) and decode (`Err("invalid utf-8")`).
- `shout(mem, ptr, len) -> Result<(usize, usize), String>`: the "exported Rust function": reads the string, upper-cases it, adds `!`, writes the result and returns its (ptr, len).
@@starter
struct Memory {
    bytes: Vec<u8>,
    next: usize,
}

impl Memory {
    fn new(size: usize) -> Memory {
        Memory { bytes: vec![0; size], next: 0 }
    }

    fn alloc(&mut self, len: usize) -> Result<usize, String> {
        todo!()
    }

    fn write_str(&mut self, s: &str) -> Result<(usize, usize), String> {
        todo!()
    }

    fn read_str(&self, ptr: usize, len: usize) -> Result<String, String> {
        todo!()
    }
}

fn shout(mem: &mut Memory, ptr: usize, len: usize) -> Result<(usize, usize), String> {
    todo!()
}

fn main() {
    let mut mem = Memory::new(32);
    let (p, l) = mem.write_str("héllo").unwrap();
    println!("js wrote at {p}, {l} bytes");
    let (p2, l2) = shout(&mut mem, p, l).unwrap();
    println!("rust returned {p2},{l2}: {}", mem.read_str(p2, l2).unwrap());
    println!("{:?}", mem.read_str(30, 5));
    println!("{:?}", mem.write_str("this is far too long to fit"));
}
@@expected
js wrote at 0, 6 bytes
rust returned 6,7: HÉLLO!
Err("out of bounds")
Err("out of memory")
@@tests
#[test]
fn checks() {
    let mut m = Memory::new(4);
    m.bytes[0] = 0xff;
    assert_eq!(m.read_str(0, 1), Err("invalid utf-8".to_string()));
    assert_eq!(m.alloc(4), Ok(0));
    assert_eq!(m.alloc(1), Err("out of memory".to_string()));
    assert_eq!(m.read_str(4, 0), Ok(String::new()));
}
@@requires
from_utf8
@@hint
`alloc`: `if self.next + len > self.bytes.len() { return Err(...) }`. Copy with `self.bytes[ptr..ptr + len].copy_from_slice(s.as_bytes())`. Read with `self.bytes.get(ptr..ptr + len).ok_or("out of bounds")?` then `std::str::from_utf8(slice)`. Note `É` is 2 bytes in UTF-8.
@@solution
struct Memory {
    bytes: Vec<u8>,
    next: usize,
}

impl Memory {
    fn new(size: usize) -> Memory {
        Memory { bytes: vec![0; size], next: 0 }
    }

    fn alloc(&mut self, len: usize) -> Result<usize, String> {
        if self.next + len > self.bytes.len() {
            return Err("out of memory".into());
        }
        let ptr = self.next;
        self.next += len;
        Ok(ptr)
    }

    fn write_str(&mut self, s: &str) -> Result<(usize, usize), String> {
        let len = s.len();
        let ptr = self.alloc(len)?;
        self.bytes[ptr..ptr + len].copy_from_slice(s.as_bytes());
        Ok((ptr, len))
    }

    fn read_str(&self, ptr: usize, len: usize) -> Result<String, String> {
        let slice = self.bytes.get(ptr..ptr + len).ok_or("out of bounds")?;
        std::str::from_utf8(slice).map(|s| s.to_string()).map_err(|_| "invalid utf-8".to_string())
    }
}

fn shout(mem: &mut Memory, ptr: usize, len: usize) -> Result<(usize, usize), String> {
    let s = mem.read_str(ptr, len)?;
    mem.write_str(&format!("{}!", s.to_uppercase()))
}

fn main() {
    let mut mem = Memory::new(32);
    let (p, l) = mem.write_str("héllo").unwrap();
    println!("js wrote at {p}, {l} bytes");
    let (p2, l2) = shout(&mut mem, p, l).unwrap();
    println!("rust returned {p2},{l2}: {}", mem.read_str(p2, l2).unwrap());
    println!("{:?}", mem.read_str(30, 5));
    println!("{:?}", mem.write_str("this is far too long to fit"));
}
