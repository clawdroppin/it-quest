@@lesson fe1-numbers-deep 基礎理論 in Depth: 2の補数, Shifts, 浮動小数点 and Errors
@@text
Every 科目A paper opens with 基礎理論 questions that are pure calculation: negative numbers in 2の補数 (two's complement), シフト演算 (shifts) used as multiplication, 浮動小数点数 (floating point) formats, and the named types of 誤差 (numerical error). They are quick marks if you know the rules. This lesson gives the rules with their Japanese terms, then checks them in code.

## 2の補数 (two's complement)

- n bits represent **−2ⁿ⁻¹ … 2ⁿ⁻¹ − 1**. 8 bits: −128 … 127; 16 bits: −32768 … 32767.
- Negate: **invert all bits (1の補数) and add 1**. −5 in 8 bits: 00000101 → 11111010 → **11111011**.
- The top bit is the sign (符号ビット). Adding two numbers with the same sign that gives a different sign is **overflow** (あふれ).

## シフト演算 (shifts)

| Shift | Rule | Use |
|---|---|---|
| 論理左シフト (logical left) | move bits left, fill 0 | × 2ⁿ for unsigned |
| 論理右シフト (logical right) | move right, fill 0 | ÷ 2ⁿ for unsigned |
| 算術左シフト (arithmetic left) | sign bit stays, others shift | × 2ⁿ for signed (overflow if sign would change) |
| 算術右シフト (arithmetic right) | fill with the **sign bit** | ÷ 2ⁿ for signed (rounds toward −∞) |

Classic question: "x を 10 倍するには?" → (x を 2 ビット左シフト + x) を 1 ビット左シフト = (4x + x) × 2 = 10x.

## 浮動小数点数 (floating point)

IEEE 754 単精度 (single precision, 32 bits): **符号 1 bit | 指数部 8 bits (bias 127) | 仮数部 23 bits** with a hidden leading 1 (正規化: 1.xxx × 2ᵉ). Value = (−1)ˢ × 1.fraction × 2^(exponent − 127). 倍精度 (double) is 1 | 11 (bias 1023) | 52.

Example: 0.75 = 1.1₂ × 2⁻¹ → sign 0, exponent −1 + 127 = 126 = 01111110, fraction 1000…0 → **0x3F400000**.

Decimal fractions like 0.1 can't be represented exactly in binary (循環小数), which is why 0.1 + 0.2 ≠ 0.3 in most languages.

## 誤差 (errors)

| Term | Meaning |
|---|---|
| **丸め誤差** (rounding error) | digits beyond the precision are rounded off |
| **打切り誤差** (truncation error) | an infinite calculation (series, iteration) is stopped early |
| **桁落ち** (cancellation) | subtracting nearly equal values loses significant digits |
| **情報落ち** (loss of information) | adding a very small number to a very large one: the small one disappears |
| **オーバーフロー / アンダーフロー** | result too large / too close to zero for the format |

Avoid 情報落ち by adding small numbers first (sort ascending); avoid 桁落ち by rearranging formulas.

## Information quantities

- Bits needed for n distinct values: **⌈log₂ n⌉** (26 letters → 5 bits; 1000 values → 10 bits).
- 1 KiB = 2¹⁰ bytes; the exam usually uses k = 10³ for transmission and K = 2¹⁰ for memory, and states it.
- De Morgan: ¬(A ∧ B) = ¬A ∨ ¬B; ¬(A ∨ B) = ¬A ∧ ¬B. XOR is 1 when the inputs differ; x XOR x = 0 (used for clearing and parity).

@@quiz
What is −20 in 8-bit two's complement?
- 00010100
- 11101011
* 11101100
- 10010100
? 00010100 → invert 11101011 → +1 = 11101100.

@@quiz
What range does 16-bit two's complement represent?
- 0 to 65535
* −32768 to 32767
- −32767 to 32767
- −65536 to 65535
? −2¹⁵ to 2¹⁵ − 1.

@@quiz
An 8-bit signed value 11110000 (−16) is arithmetically shifted right by 2. What is the result?
- 00111100 (60)
* 11111100 (−4)
- 11000000 (−64)
- 00001111 (15)
? Fill with the sign bit; −16 ÷ 4.

@@quiz
Adding a very small value to a very large one makes the small value disappear. Which error is this?
- 桁落ち
* 情報落ち
- 打切り誤差
- アンダーフロー
? Add small numbers first.

@@quiz
How many bits are needed to give a unique code to 300 products?
- 8
* 9
- 10
- 300
? 2⁸ = 256 < 300 ≤ 512 = 2⁹.

@@quiz
In IEEE 754 single precision, what is the exponent bias?
- 63
* 127
- 128
- 1023
? 1023 is for double precision.

@@flashcards
2の補数 :: invert bits and add 1
8-bit signed range :: −128 to 127
符号ビット :: top bit
論理シフト :: fill with 0
算術右シフト :: fill with sign bit
左シフト n :: × 2ⁿ
x × 10 :: (x << 2 + x) << 1
単精度 layout :: 1 | 8 | 23
単精度 bias :: 127
倍精度 layout :: 1 | 11 | 52
正規化 :: 1.xxx × 2ᵉ, hidden 1
0.1 in binary :: repeating, inexact
丸め誤差 :: rounding off digits
打切り誤差 :: stopping an infinite process
桁落ち :: subtracting near-equal values
情報落ち :: small + huge loses the small
bits for n values :: ⌈log₂ n⌉
De Morgan :: ¬(A∧B) = ¬A ∨ ¬B
x XOR x :: 0

@@drill
What is −1 in 8-bit two's complement (binary)?
= 11111111
= 1111 1111
? All ones.

@@drill
What is the Japanese term for cancellation error when subtracting nearly equal values?
= 桁落ち
= けたおち
? 情報落ち is loss of a small addend.

@@exercise Two's complement and shifts
Implement 8-bit two's complement encoding and decoding, logical and arithmetic right shifts, and signed addition with overflow detection. Print each value's bit pattern, the shift results, and the sums, including the overflow cases.
@@lang python
@@starter
BITS = 8
values = [5, -5, -20, 127, -128]
shifts = [(-16, 2), (100, 3), (-1, 4)]
sums = [(100, 27), (100, 28), (-100, -28), (-100, -29), (50, -80)]

def encode(n):
    raise NotImplementedError
@@expected
    5 -> 00000101
   -5 -> 11111011
  -20 -> 11101100
  127 -> 01111111
 -128 -> 10000000
  -16 >> 2: logical 00111100 (60), arithmetic 11111100 (-4)
  100 >> 3: logical 00001100 (12), arithmetic 00001100 (12)
   -1 >> 4: logical 00001111 (15), arithmetic 11111111 (-1)
  100 +   27 = 01111111 =   127
  100 +   28 = 10000000 =  -128  overflow (true sum 128)
 -100 +  -28 = 10000000 =  -128
 -100 +  -29 = 01111111 =   127  overflow (true sum -129)
   50 +  -80 = 11100010 =   -30
@@requires
def encode
def decode
1 << BITS
@@hint
`encode(n)` is `n & ((1 << BITS) - 1)` formatted with `f"{x:08b}"`. `decode(x)` subtracts `1 << BITS` when the top bit is set. An arithmetic right shift on the decoded value is just Python's `>>`; a logical one shifts the unsigned pattern.
@@solution
BITS = 8
values = [5, -5, -20, 127, -128]
shifts = [(-16, 2), (100, 3), (-1, 4)]
sums = [(100, 27), (100, 28), (-100, -28), (-100, -29), (50, -80)]
MASK = (1 << BITS) - 1

def encode(n):
    return n & MASK

def decode(x):
    return x - (1 << BITS) if x & (1 << (BITS - 1)) else x

def bits(x):
    return f"{x:0{BITS}b}"

for v in values:
    print(f"{v:>5} -> {bits(encode(v))}")

for v, n in shifts:
    x = encode(v)
    logical = x >> n
    arithmetic = encode(decode(x) >> n)
    print(f"{v:>5} >> {n}: logical {bits(logical)} ({logical}), arithmetic {bits(arithmetic)} ({decode(arithmetic)})")

for a, b in sums:
    raw = encode(encode(a) + encode(b))
    result = decode(raw)
    overflow = (a >= 0) == (b >= 0) and (result >= 0) != (a >= 0)
    print(f"{a:>5} + {b:>4} = {bits(raw)} = {result:>5}{'  overflow (true sum ' + str(a + b) + ')' if overflow else ''}")

@@exercise Encode IEEE 754 single precision by hand
Without using `struct` in your encoder, convert decimal numbers to the 32-bit single-precision pattern: sign, exponent with bias 127, and a 23-bit fraction (truncate, don't round, for simplicity). Print the fields and the hex value, then use `struct` only to check your result for numbers that are exactly representable.
@@lang python
@@starter
import struct

numbers = [0.75, -2.5, 10.0, 0.15625, 1.0, -0.375, 0.1]

def encode(x):
    raise NotImplementedError
@@expected
    0.75: sign 0 exp 01111110 (-1) frac 10000000000000000000000 = 0x3F400000  matches struct
    -2.5: sign 1 exp 10000000 (+1) frac 01000000000000000000000 = 0xC0200000  matches struct
    10.0: sign 0 exp 10000010 (+3) frac 01000000000000000000000 = 0x41200000  matches struct
 0.15625: sign 0 exp 01111100 (-3) frac 01000000000000000000000 = 0x3E200000  matches struct
     1.0: sign 0 exp 01111111 (+0) frac 00000000000000000000000 = 0x3F800000  matches struct
  -0.375: sign 1 exp 01111101 (-2) frac 10000000000000000000000 = 0xBEC00000  matches struct
     0.1: sign 0 exp 01111011 (-4) frac 10011001100110011001100 = 0x3DCCCCCC  struct rounds to 0x3DCCCCCD (0.1 is not exact)
@@requires
def encode
127
struct.pack(
@@hint
Normalise: start with exponent 0, then halve or double `abs(x)` until `1 <= m < 2`, adjusting the exponent. The fraction bits come from repeatedly doubling `m - 1`: each time the value reaches 1 or more, emit a 1 and subtract 1.
@@solution
import struct

numbers = [0.75, -2.5, 10.0, 0.15625, 1.0, -0.375, 0.1]

def encode(x):
    sign = 1 if x < 0 else 0
    m, e = abs(x), 0
    while m >= 2:
        m /= 2
        e += 1
    while m < 1:
        m *= 2
        e -= 1
    frac, f = 0, m - 1
    for _ in range(23):
        f *= 2
        bit = 1 if f >= 1 else 0
        frac = frac << 1 | bit
        f -= bit
    return sign, e + 127, frac

for x in numbers:
    sign, exponent, frac = encode(x)
    word = sign << 31 | exponent << 23 | frac
    check = struct.unpack(">I", struct.pack(">f", x))[0]
    note = "matches struct" if word == check else f"struct rounds to 0x{check:08X} (0.1 is not exact)"
    print(f"{x:>8}: sign {sign} exp {exponent:08b} ({exponent - 127:+d}) frac {frac:023b} = 0x{word:08X}  {note}")
