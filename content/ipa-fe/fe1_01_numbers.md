@@lesson fe1-numbers 基礎理論: Numbers, Logic and Bits
@@lang python
@@text
## 基数変換 (base conversion)

| 10進数 | 2進数 | 8進数 | 16進数 |
|---|---|---|---|
| 10 | 1010 | 12 | A |
| 15 | 1111 | 17 | F |
| 255 | 1111 1111 | 377 | FF |
| 0.5 | 0.1 | 0.4 | 0.8 |
| 0.625 | 0.101 | 0.5 | 0.A |

- Decimal → binary: divide by 2 and read the remainders bottom-up. Fractions: multiply by 2 and read the integer parts top-down. **0.1 (decimal) can't be represented exactly in binary**: that's why floating point has rounding errors (丸め誤差).
- Binary ↔ hex: group 4 bits (1 nibble). Binary ↔ octal: group 3 bits.

## 負の数: 2の補数 (two's complement)

To negate: **invert all bits and add 1** (ビット反転して1を足す). With n bits, the range is **−2ⁿ⁻¹ … 2ⁿ⁻¹−1**: 8 bits holds −128…127. The top bit is the sign. −1 = 1111 1111. Adding past the top is **overflow** (あふれ).

## Shifts

- **Logical shift** (論理シフト): fill with 0. Left shift by n = ×2ⁿ, right shift = ÷2ⁿ (unsigned).
- **Arithmetic shift** (算術シフト): keep the sign bit when shifting right, so negative numbers stay negative.
- Exam trick: x × 10 = (x << 3) + (x << 1), because 8x + 2x.

## Logic (論理演算)

| A | B | AND (論理積) | OR (論理和) | XOR (排他的論理和) | NAND |
|---|---|---|---|---|---|
| 0 | 0 | 0 | 0 | 0 | 1 |
| 0 | 1 | 0 | 1 | 1 | 1 |
| 1 | 0 | 0 | 1 | 1 | 1 |
| 1 | 1 | 1 | 1 | 0 | 0 |

- **Masking**: `x AND 0000 1111` keeps the low 4 bits; `x OR 1000 0000` sets the top bit; `x XOR 1111 1111` flips all bits.
- **De Morgan** (ド・モルガンの法則): NOT(A AND B) = NOT A OR NOT B; NOT(A OR B) = NOT A AND NOT B.

## Units and other favourites

- k=10³, M=10⁶, G=10⁹, T=10¹²; m=10⁻³, μ=10⁻⁶, n=10⁻⁹, p=10⁻¹². (Memory often uses 2¹⁰ = 1024.)
- n bits represent 2ⁿ values: to number 1,000 items you need 10 bits.
- **Probability and statistics**: average (平均), median (中央値), mode (最頻値), standard deviation (標準偏差), and normal distribution (正規分布): ±1σ ≈ 68%, ±2σ ≈ 95%.
- **Information theory**: Huffman coding, parity bits (パリティ: detect 1-bit errors), Hamming codes (correct 1-bit errors), CRC.
- **Automata and formal languages**: state transition diagrams (状態遷移図), regular expressions (正規表現), BNF.
- **Reverse Polish notation** (逆ポーランド記法): `(A + B) × C` → `A B + C ×`. It's evaluated with a stack.

@@exercise Count the 1 bits
Without `bin()` or `.count()`, write `popcount(n)` that counts the 1 bits of a non-negative integer using `& 1` and `>> 1` (AND with 1 and shift right), the way the exam's pseudo-code does. Print the counts for 0, 7, 255 and 1234.
@@starter
def popcount(n):
    count = 0
    # while n is not 0: add (n AND 1) to count, then shift n right by 1
    return count


for n in [0, 7, 255, 1234]:
    print(n, popcount(n))
@@expected
0 0
7 3
255 8
1234 5
@@hint
`count += n & 1` then `n >>= 1`, in a `while n:` loop. 1234 is 100 1101 0010 in binary.
@@solution
def popcount(n):
    count = 0
    while n:
        count += n & 1
        n >>= 1
    return count


for n in [0, 7, 255, 1234]:
    print(n, popcount(n))

@@quiz
What is the 8-bit two's complement representation of −5?
* 1111 1011
- 1000 0101
- 1111 1010
- 0000 0101
? 5 = 0000 0101 → invert: 1111 1010 → +1: 1111 1011.

@@quiz
What is 16進数 2F in decimal?
* 47
- 32
- 215
- 62
? 2×16 + 15 = 47.

@@quiz
What is decimal 0.375 in binary?
* 0.011
- 0.101
- 0.111
- 0.0111
? 0.375×2 = 0.75 → 0; 0.75×2 = 1.5 → 1; 0.5×2 = 1.0 → 1.

@@quiz
Shifting an unsigned binary number 3 bits to the left (logical shift) does what?
* Multiplies it by 8 (if nothing overflows)
- Divides it by 3
- Multiplies it by 3
- Adds 3
? 2³ = 8.

@@quiz
To keep only the lower 4 bits of an 8-bit value, AND it with...
* 0000 1111
- 1111 0000
- 1111 1111
- 0000 0000
? AND with 1 keeps the bit, AND with 0 clears it.

@@quiz
What is the range of an 8-bit two's complement integer?
* −128 to 127
- −127 to 127
- 0 to 255
- −256 to 255
? −2⁷ to 2⁷−1.

@@quiz
What is the reverse Polish notation for (A + B) × (C − D)?
* A B + C D − ×
- A B C D + − ×
- + A B × − C D
- A + B C − D ×
? Operands first, then the operator, inside out.

@@quiz
What's the minimum number of bits needed to give each of 300 items a unique number?
* 9
- 8
- 10
- 300
? 2⁸ = 256 is too few; 2⁹ = 512 is enough.

@@quiz
Which equals NOT (A AND B)?
* (NOT A) OR (NOT B)
- (NOT A) AND (NOT B)
- A OR B
- A XOR B
? De Morgan's law.

@@flashcards
基数変換 :: Base conversion
2進数 / 8進数 / 10進数 / 16進数 :: Binary / octal / decimal / hexadecimal
2の補数 :: Two's complement: invert bits, add 1
論理シフト / 算術シフト :: Logical shift (fill 0) / arithmetic shift (keep sign)
論理積 / 論理和 / 排他的論理和 :: AND / OR / XOR
否定 :: NOT
ド・モルガンの法則 :: NOT(A AND B) = NOT A OR NOT B
丸め誤差 :: Rounding error
桁あふれ / オーバーフロー :: Overflow
逆ポーランド記法 :: Reverse Polish notation (postfix)
状態遷移図 :: State transition diagram
正規表現 :: Regular expression
パリティビット :: Parity bit: detects 1-bit errors
ハミング符号 :: Hamming code: corrects 1-bit errors
標準偏差 :: Standard deviation
中央値 / 最頻値 :: Median / mode

@@drill
Write decimal 200 in hexadecimal.
= C8
? 200 = 12×16 + 8.
