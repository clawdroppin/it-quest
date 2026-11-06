@@lesson b9-simd 🎓 Beyond IT: SIMD, SWAR and Bit-Level Parallelism
@@concepts numbers, text-processing
@@text
> **Beyond IT scope.** Low-level performance work for engines, codecs, databases and scientific code. Fascinating, but not needed for IT jobs.

**SIMD** (single instruction, multiple data) instructions process several values at once: SSE handles 16 bytes per instruction, AVX2 32 and AVX-512 64; ARM has NEON and SVE. simdjson parses JSON at gigabytes per second, and video codecs, game physics and database column scans all depend on it.

## Three ways to get SIMD

1. **Auto-vectorisation**: write simple loops over contiguous arrays (no aliasing, no early exits, no function calls) and compile with optimisation (`-O2 -march=native`, `/O2 /arch:AVX2`). Check the compiler's report (`-fopt-info-vec`, `/Qvec-report:2`).
2. **Intrinsics**: C functions that map to single instructions. Fast, but tied to one instruction set:

```cpp,norun
#include <immintrin.h>
// count bytes equal to '\n', 32 at a time (AVX2)
__m256i nl = _mm256_set1_epi8('\n');
for (; i + 32 <= n; i += 32) {
    __m256i chunk = _mm256_loadu_si256(reinterpret_cast<const __m256i*>(data + i));
    uint32_t mask = static_cast<uint32_t>(_mm256_movemask_epi8(_mm256_cmpeq_epi8(chunk, nl)));
    count += std::popcount(mask);
}
```

3. **Portable SIMD libraries**: Google Highway, xsimd, Eve, and `std::experimental::simd` (standardised as `std::simd` in C++26).

## SWAR: SIMD within a register

Even without vector instructions, a 64-bit register holds 8 bytes. With careful arithmetic you can test all 8 at once. The classic trick finds zero bytes: for each byte, `(x & 0x7F) + 0x7F` sets the high bit when the low 7 bits are non-zero, and OR-ing in `x` itself covers the high bit, so the bytes whose high bit is still clear are exactly the zero bytes. XOR with a repeated pattern first and "zero byte" becomes "byte equals the target". Masking with `0x7F` before adding means no carry can leak into the neighbouring byte, so the count is exact.

## Bitmaps

A bitmap stores one bit per item: 1,000 hosts × 3 attributes fit in 48 64-bit words. Queries like "Linux AND production AND NOT patched" become word-wise `&`, `|` and `~` on 64 items at a time, and `std::popcount` (C++20) counts the matches with a single instruction. Databases use exactly this in bitmap indexes (Roaring bitmaps).

@@quiz
What does SIMD stand for?
* Single instruction, multiple data: one instruction operating on several values at once
- Simple integer math device
- Synchronous instruction memory dispatch
- A GPU language
? AVX2 compares 32 bytes in one instruction.

@@quiz
Why mask the unused bits of the last word after a bitmap NOT?
- To save memory
* Otherwise the bits beyond the last item become 1 and are counted as matches
- popcount requires it
- It isn't necessary
? 1000 items use 1024 bits; 24 are padding.

@@quiz
How many 8-bit values does SWAR process at once in a 64-bit register?
* 8
- 4
- 16
- 64
? 64 bits divided into 8-bit lanes gives eight lanes.

@@flashcards
SIMD :: one instruction, many values
SSE / AVX2 / AVX-512 :: x86 SIMD extensions (16/32/64 bytes)
NEON :: ARM SIMD
Intrinsics :: C functions mapping to instructions
Auto-vectorisation :: compiler-generated SIMD
SWAR :: SIMD within a register
`std::popcount` :: count set bits (C++20)
Bitmap index :: one bit per row per attribute
`std::experimental::simd` :: portable SIMD types

@@exercise Count bytes with SWAR
Write `size_t count_byte_swar(std::string_view s, unsigned char target)` that processes 8 bytes at a time:

- load each 8-byte word with `std::memcpy` (never cast the char pointer to `uint64_t*`: that's undefined behaviour from misalignment and aliasing)
- `x = word ^ (0x0101010101010101 * target)` turns matching bytes into zero bytes
- `y = ~(((x & 0x7F7F...) + 0x7F7F...) | x | 0x7F7F...)` leaves exactly the high bit of each zero byte set; add `std::popcount(y)`
- handle the remaining 0–7 bytes with a plain loop, and count the words processed

Compare with `std::count` on a 10,000-line text plus some tricky bytes (`\n` next to `\x0b`, `\x01` and `\0`).
@@starter
#include <algorithm>
#include <bit>
#include <cstdint>
#include <cstring>
#include <iostream>
#include <string>
#include <string_view>

int main() {
    std::string text;
    for (int i = 0; i < 10000; ++i) text += "line " + std::to_string(i) + '\n';
    text += std::string("\n\x0b\n\x01\0\n", 6);
    std::cout << "TODO\n";
}
@@expected
newlines (scalar): 10003
newlines (swar): 10003
zero bytes (scalar): 1
zero bytes (swar): 1
text bytes: 98896, 8-byte words processed: 12362
@@requires
std::popcount
std::memcpy
0x7F7F7F7F7F7F7F7F
@@hint
Return the word count through a `size_t* words` out-parameter (or a small struct). `const uint64_t low7 = 0x7F7F7F7F7F7F7F7Full;`. Count with the scalar version via `std::count(s.begin(), s.end(), char(target))`.
@@solution
#include <algorithm>
#include <bit>
#include <cstdint>
#include <cstring>
#include <iostream>
#include <string>
#include <string_view>

size_t count_byte_swar(std::string_view s, unsigned char target, size_t* words = nullptr) {
    const uint64_t ones = 0x0101010101010101ull;
    const uint64_t low7 = 0x7F7F7F7F7F7F7F7Full;
    const uint64_t pattern = ones * target;
    size_t count = 0, i = 0, n_words = 0;
    for (; i + 8 <= s.size(); i += 8, ++n_words) {
        uint64_t word;
        std::memcpy(&word, s.data() + i, 8);
        uint64_t x = word ^ pattern;
        uint64_t y = ~(((x & low7) + low7) | x | low7);
        count += static_cast<size_t>(std::popcount(y));
    }
    for (; i < s.size(); ++i) count += static_cast<unsigned char>(s[i]) == target;
    if (words) *words = n_words;
    return count;
}

int main() {
    std::string text;
    for (int i = 0; i < 10000; ++i) text += "line " + std::to_string(i) + '\n';
    text += std::string("\n\x0b\n\x01\0\n", 6);

    size_t words = 0;
    std::cout << "newlines (scalar): " << std::count(text.begin(), text.end(), '\n') << '\n';
    std::cout << "newlines (swar): " << count_byte_swar(text, '\n', &words) << '\n';
    std::cout << "zero bytes (scalar): " << std::count(text.begin(), text.end(), '\0') << '\n';
    std::cout << "zero bytes (swar): " << count_byte_swar(text, 0) << '\n';
    std::cout << "text bytes: " << text.size() << ", 8-byte words processed: " << words << '\n';
}

@@exercise A bitmap index with popcount
Index 1,000 hosts by three attributes, one bit per host:

- `Bitmap(n)` stores `(n + 63) / 64` words; `set(i)`, `test(i)`, `count()` (sum of `std::popcount`), `operator&` and `operator~`
- `operator~` must clear the padding bits beyond `n` in the last word
- host `i` runs Linux if `i % 3 != 0`, is in production if `i % 4 == 0`, and is patched if `i % 5 != 0`

Answer "Linux AND production AND NOT patched" with bitmap operations, check it against a plain loop, and list the first 5 matching ids.
@@starter
#include <bit>
#include <cstdint>
#include <iostream>
#include <vector>

int main() {
    const size_t n = 1000;
    std::cout << "TODO\n";
}
@@expected
words per bitmap: 16
linux: 666, prod: 250, patched: 800
linux & prod & !patched: 33 (scalar check: 33)
first ids: 20 40 80 100 140
@@requires
std::popcount
operator~
@@hint
In `operator~`, after inverting every word: `if (size_t extra = n_ % 64) words.back() &= (uint64_t(1) << extra) - 1;`. Without that, `~patched` would count 24 phantom hosts.
@@solution
#include <bit>
#include <cstdint>
#include <iostream>
#include <vector>

class Bitmap {
public:
    explicit Bitmap(size_t n) : n_(n), words_((n + 63) / 64, 0) {}

    void set(size_t i) { words_[i / 64] |= uint64_t(1) << (i % 64); }
    bool test(size_t i) const { return (words_[i / 64] >> (i % 64)) & 1; }
    size_t words() const { return words_.size(); }

    size_t count() const {
        size_t total = 0;
        for (uint64_t w : words_) total += static_cast<size_t>(std::popcount(w));
        return total;
    }

    Bitmap operator&(const Bitmap& other) const {
        Bitmap out(n_);
        for (size_t i = 0; i < words_.size(); ++i) out.words_[i] = words_[i] & other.words_[i];
        return out;
    }

    Bitmap operator~() const {
        Bitmap out(n_);
        for (size_t i = 0; i < words_.size(); ++i) out.words_[i] = ~words_[i];
        if (size_t extra = n_ % 64) out.words_.back() &= (uint64_t(1) << extra) - 1;
        return out;
    }

private:
    size_t n_;
    std::vector<uint64_t> words_;
};

int main() {
    const size_t n = 1000;
    Bitmap linux_hosts(n), prod(n), patched(n);
    for (size_t i = 0; i < n; ++i) {
        if (i % 3 != 0) linux_hosts.set(i);
        if (i % 4 == 0) prod.set(i);
        if (i % 5 != 0) patched.set(i);
    }
    std::cout << "words per bitmap: " << linux_hosts.words() << '\n';
    std::cout << "linux: " << linux_hosts.count() << ", prod: " << prod.count() << ", patched: " << patched.count() << '\n';

    Bitmap risky = linux_hosts & prod & ~patched;
    size_t scalar = 0;
    for (size_t i = 0; i < n; ++i)
        if (i % 3 != 0 && i % 4 == 0 && i % 5 == 0) ++scalar;
    std::cout << "linux & prod & !patched: " << risky.count() << " (scalar check: " << scalar << ")\n";

    std::cout << "first ids:";
    int shown = 0;
    for (size_t i = 0; i < n && shown < 5; ++i)
        if (risky.test(i)) {
            std::cout << ' ' << i;
            ++shown;
        }
    std::cout << '\n';
}
