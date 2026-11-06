@@lesson q10-binary Bytes, Binary Formats and Calling C or Rust
@@concepts files, networking
@@text
Network packets, file headers, device protocols and C libraries all speak **binary**, not text. This lesson covers Python's tools for raw bytes and for calling compiled code: the same ground as Rust's unsafe/FFI lesson, from the Python side.

## bytes, bytearray and memoryview

```python
data = b"\x45\x00\x00\x3c"            # bytes: immutable
print(data[0], hex(data[0]), data[:2], len(data))

buf = bytearray(b"hello")             # mutable
buf[0] = ord("J")
print(buf, bytes(buf).decode())

big = bytearray(1_000_000)
view = memoryview(big)[10:20]         # a window, no copy
view[:] = b"0123456789"
print(big[10:20])
```

- `str.encode("utf-8")` → bytes; `bytes.decode("utf-8")` → str. Text is characters, bytes are numbers 0–255; always know which encoding you're using.
- `bytes.fromhex("dead beef")`, `data.hex(" ")` for hex dumps.
- `int.from_bytes(b, "big")` / `n.to_bytes(4, "big")` for single integers.

## Byte order

Multi-byte numbers can be stored **big-endian** (most significant byte first: network order, used by IP/TCP headers) or **little-endian** (x86 and ARM CPUs, most file formats on Windows):

```python
n = 0x12345678
print(n.to_bytes(4, "big").hex(" "), "|", n.to_bytes(4, "little").hex(" "))
```

## struct: packing and unpacking records

`struct` converts between Python values and C-style binary records using a format string:

| Code | Type | Size |
|---|---|---|
| `B` / `b` | unsigned/signed byte | 1 |
| `H` / `h` | unsigned/signed short | 2 |
| `I` / `i` | unsigned/signed int | 4 |
| `Q` / `q` | unsigned/signed long long | 8 |
| `f` / `d` | float / double | 4 / 8 |
| `4s` | 4 raw bytes | 4 |
| `?` | bool | 1 |

The first character sets byte order: `!` or `>` big-endian (network), `<` little-endian, `=` native without padding.

```python
import struct

# a sensor record: id (ushort), temperature (float), flags (byte), little-endian
record = struct.pack("<Hf B", 7, 21.5, 0b101)
print(record.hex(" "), struct.calcsize("<HfB"))
sensor_id, temp, flags = struct.unpack("<HfB", record)
print(sensor_id, temp, bool(flags & 0b100))

# the first 4 bytes of an IPv4 header
version_ihl, tos, total_len = struct.unpack("!BBH", bytes.fromhex("45 00 00 3c"))
print(version_ihl >> 4, (version_ihl & 0x0F) * 4, total_len)
```

`struct.iter_unpack` walks repeated records; `Struct("<HfB")` precompiles a format for speed. For real network protocols, **Scapy** builds and dissects packets.

## Calling C with ctypes

`ctypes` (standard library) loads shared libraries (`.dll`, `.so`, `.dylib`) and calls their functions:

```python,norun
import ctypes, sys

if sys.platform == "win32":
    user32 = ctypes.windll.user32
    user32.MessageBoxW(None, "Backup finished", "IT Quest", 0)
    kernel32 = ctypes.windll.kernel32
    print(kernel32.GetTickCount64.restype)      # set restype/argtypes for correct conversions
else:
    libc = ctypes.CDLL(None)
    libc.printf(b"pid %d\n", libc.getpid())
```

- Declare `argtypes` and `restype`; wrong declarations crash the interpreter (no safety net, just like `unsafe` in Rust).
- `ctypes.Structure` subclasses describe C structs with `_fields_`.
- On Windows, ctypes is a quick way to reach Win32 APIs not wrapped by any module; `pywin32` wraps most of them properly.
- **cffi** is a friendlier alternative that reads C declarations.

## Writing extensions

When a hot loop needs native speed, write that part as an **extension module**:

| Tool | Write in |
|---|---|
| **PyO3 + maturin** | Rust (`#[pyfunction]`, `maturin develop`); safe and increasingly popular |
| **Cython** | Python with C types, compiled to C |
| **pybind11 / nanobind** | C++ |
| **C API** | plain C, most work |

```rust,norun
// Rust side (src/lib.rs), built with `maturin develop`
use pyo3::prelude::*;

#[pyfunction]
fn checksum(data: &[u8]) -> u32 {
    data.iter().fold(0u32, |acc, &b| acc.rotate_left(5) ^ b as u32)
}

#[pymodule]
fn fastcheck(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(checksum, m)?)
}
// Python: import fastcheck; fastcheck.checksum(b"data")
```

## Other binary tools

- `array.array("H", ...)`: compact typed arrays.
- `binascii`, `base64`, `zlib`, `gzip`, `hashlib`: encodings, compression and hashes.
- `mmap`: map a huge file into memory and slice it like bytes.
- `pickle`: Python's own object serialisation; **never unpickle untrusted data**: it can execute code.

@@quiz
Which byte order do IP and TCP headers use?
* Big-endian (network order)
- Little-endian
- Whatever the CPU uses
- None
? In struct format strings use `!` or `>`.

@@quiz
What does `struct.unpack("!BBH", data)` expect?
- 3 bytes
* 4 bytes: two unsigned bytes and one big-endian unsigned short
- 6 bytes
- A string
? B is 1 byte, H is 2 bytes.

@@quiz
Why is unpickling data from the internet dangerous?
- It's slow
* A crafted pickle can execute arbitrary code when loaded
- It loses precision
- It only works on Linux
? Use JSON or another data-only format for untrusted input.

@@quiz
What happens if you declare wrong argtypes for a C function in ctypes?
- A Python exception explains it
* Memory can be corrupted and the interpreter may crash
- ctypes fixes it automatically
- Nothing
? ctypes has no safety checks; it's Python's `unsafe`.

@@drill
Which `struct` format prefix means network (big-endian) byte order?
= !
? struct.pack("!H", 443).

@@flashcards
bytes :: immutable sequence of numbers 0–255
bytearray :: mutable bytes
memoryview :: zero-copy window into a buffer
Big-endian :: most significant byte first; network order
Little-endian :: least significant byte first; x86/ARM order
struct :: pack/unpack binary records with format strings
ctypes :: call functions in C shared libraries
PyO3 / maturin :: build Python extension modules in Rust
pickle :: Python object serialisation; unsafe with untrusted data

@@exercise Parse an IPv4 header
Write `parse_ipv4(header: bytes) -> dict` for the first 20 bytes of an IPv4 header (use `struct.unpack("!BBHHHBBH4s4s", ...)`):

- `version` (high 4 bits of byte 0), `header_len` (low 4 bits × 4)
- `total_length`, `ttl`, `protocol` (6 → `"TCP"`, 17 → `"UDP"`, 1 → `"ICMP"`, otherwise the number as a string)
- `src` and `dst` as dotted strings like `"192.168.1.10"`
- raise `ValueError("too short")` if fewer than 20 bytes
@@starter
import struct


def parse_ipv4(header):
    pass


h = bytes.fromhex("4500003c1c4640004006b1e6c0a8010ac0a80101")
print(parse_ipv4(h))
@@tests
h = bytes.fromhex("4500003c1c4640004006b1e6c0a8010ac0a80101")
r = parse_ipv4(h)
check((r["version"], r["header_len"], r["total_length"]), (4, 20, 60), "header fields")
check((r["ttl"], r["protocol"]), (64, "TCP"), "ttl and protocol")
check((r["src"], r["dst"]), ("192.168.1.10", "192.168.1.1"), "addresses")
check(parse_ipv4(bytes.fromhex("4500001c00000000ff11000008080808c0a80001"))["protocol"], "UDP", "udp")
check(parse_ipv4(bytes.fromhex("4500001c00000000ff2f000008080808c0a80001"))["protocol"], "47", "other protocol")
try:
    parse_ipv4(b"\x45\x00")
    check("ok", "ValueError", "too short")
except ValueError:
    pass
@@requires
struct.unpack
@@hint
`vihl, tos, total, ident, frag, ttl, proto, csum, src, dst = struct.unpack("!BBHHHBBH4s4s", header[:20])`. Version is `vihl >> 4`, header length `(vihl & 0x0F) * 4`. Addresses: `".".join(str(b) for b in src)`.
@@solution
import struct

PROTOCOLS = {1: "ICMP", 6: "TCP", 17: "UDP"}


def parse_ipv4(header):
    if len(header) < 20:
        raise ValueError("too short")
    vihl, tos, total, ident, frag, ttl, proto, csum, src, dst = struct.unpack("!BBHHHBBH4s4s", header[:20])
    return {
        "version": vihl >> 4,
        "header_len": (vihl & 0x0F) * 4,
        "total_length": total,
        "ttl": ttl,
        "protocol": PROTOCOLS.get(proto, str(proto)),
        "src": ".".join(str(b) for b in src),
        "dst": ".".join(str(b) for b in dst),
    }


h = bytes.fromhex("4500003c1c4640004006b1e6c0a8010ac0a80101")
print(parse_ipv4(h))

@@exercise A binary log format
A device stores readings as fixed-size little-endian records: `timestamp` (uint32), `sensor` (uint16), `value` (float32), `ok` (bool). Write:

- `encode(readings)`: a list of `(timestamp, sensor, value, ok)` tuples → bytes, all records concatenated, preceded by a header: the 4 bytes `b"SLOG"` and the record count as uint16 little-endian
- `decode(data)`: back to a list of tuples; `value` rounded to 2 decimals; raise `ValueError("bad magic")` if the header doesn't start with `SLOG` and `ValueError("truncated")` if there are fewer bytes than the count says
@@starter
import struct

RECORD = struct.Struct("<IHf?")


def encode(readings):
    pass


def decode(data):
    pass


blob = encode([(1700000000, 7, 21.5, True), (1700000060, 7, 21.75, False)])
print(len(blob), decode(blob))
@@tests
rs = [(1700000000, 7, 21.5, True), (1700000060, 9, -3.3, False)]
blob = encode(rs)
check(len(blob), 6 + 2 * RECORD.size, "size")
check(blob[:4], b"SLOG", "magic")
check(decode(blob), rs, "round trip")
check(decode(encode([])), [], "empty")
try:
    decode(b"XLOG\x00\x00")
    check("ok", "ValueError", "bad magic")
except ValueError as e:
    check(str(e), "bad magic", "magic message")
try:
    decode(blob[:-1])
    check("ok", "ValueError", "truncated")
except ValueError as e:
    check(str(e), "truncated", "truncated message")
@@hint
`header = b"SLOG" + struct.pack("<H", len(readings))`, then `b"".join(RECORD.pack(*r) for r in readings)`. Decode: check `data[:4]`, `(count,) = struct.unpack_from("<H", data, 4)`, check `len(data) >= 6 + count * RECORD.size`, then `RECORD.unpack_from(data, 6 + i * RECORD.size)` for each i. float32 can't store 21.75 exactly in every case, hence the rounding.
@@solution
import struct

RECORD = struct.Struct("<IHf?")


def encode(readings):
    header = b"SLOG" + struct.pack("<H", len(readings))
    return header + b"".join(RECORD.pack(*r) for r in readings)


def decode(data):
    if data[:4] != b"SLOG":
        raise ValueError("bad magic")
    (count,) = struct.unpack_from("<H", data, 4)
    if len(data) < 6 + count * RECORD.size:
        raise ValueError("truncated")
    out = []
    for i in range(count):
        ts, sensor, value, ok = RECORD.unpack_from(data, 6 + i * RECORD.size)
        out.append((ts, sensor, round(value, 2), ok))
    return out


blob = encode([(1700000000, 7, 21.5, True), (1700000060, 7, 21.75, False)])
print(len(blob), decode(blob))
