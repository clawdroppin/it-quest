@@lesson x1-text Text in Depth: string_view, format, from_chars and UTF-8
@@concepts strings, text-processing
@@text
C++ text handling is fast but low-level. These four tools cover most real work.

## std::string_view: look, don't copy

A `string_view` is a pointer plus a length pointing into someone else's characters. Taking substrings of a view is free, so parsers use them everywhere. It does **not** own the text: never return a view into a local string.

```cpp
#include <iostream>
#include <string_view>

int main() {
    std::string_view line = "GET /health HTTP/1.1";
    auto method = line.substr(0, line.find(' '));
    std::cout << method << ' ' << line.size() << ' ' << line.ends_with("1.1") << '\n';  // GET 20 1
}
```

## std::format: Python-style formatting (C++20)

```cpp
#include <format>
#include <iostream>

int main() {
    std::cout << std::format("{:<8}|{:>6}|{:^7}|\n", "web01", 42, "ok");     // align left/right/centre
    std::cout << std::format("{:.2f} {:08.3f} {:+d}\n", 3.14159, 2.5, 7);     // precision, padding, sign
    std::cout << std::format("{:x} {:#x} {:08b} {:#o}\n", 255, 255, 5, 8);     // hex, binary, octal
    std::cout << std::format("{0} and {0} again, then {1}\n", "a", "b");      // positional arguments
}
```

C++23 adds `std::print` and `std::println`, which print directly.

## from_chars and to_chars: fast, exact conversions

`std::stoi` throws and accepts leading spaces and trailing junk. `std::from_chars` (in `<charconv>`) never throws, never allocates, ignores locales, and tells you exactly what happened:

```cpp,norun
int value;
auto [ptr, ec] = std::from_chars(text.data(), text.data() + text.size(), value);
if (ec == std::errc::invalid_argument) ...      // no digits at all
if (ec == std::errc::result_out_of_range) ...   // too big for int
if (ptr != text.data() + text.size()) ...       // trailing characters
```

## UTF-8

`std::string` holds **bytes**. In UTF-8, ASCII characters are 1 byte and others 2–4 bytes, so `size()` is not the number of characters, and cutting at an arbitrary byte can split a character into invalid garbage. Continuation bytes always look like `10xxxxxx`, so counting the bytes that *aren't* continuation bytes counts code points. For full Unicode work (normalisation, grapheme clusters, case folding) use a library like ICU.

On MSVC, compile with `/utf-8` so string literals are UTF-8 (this app does).

@@quiz
Why is returning a `std::string_view` into a local `std::string` a bug?
* The view doesn't own the characters; they're destroyed when the function returns
- string_view can't be returned
- It copies the string twice
- It's only a warning
? A dangling view.

@@quiz
What does `std::from_chars` do with `"80x"` when parsing an int?
- Throws an exception
- Returns an error and no value
* Parses 80 and returns a pointer to the `x`, so you can detect the trailing text
- Ignores the x silently and you can't tell
? Check ptr against the end.

@@quiz
What does `"日本".size()` return for a UTF-8 string?
- 2
* 6
- 4
- It depends on the font
? Each of these characters takes 3 bytes.

@@flashcards
`std::string_view` :: non-owning view of characters
`std::format("{:>8}", x)` :: right-align in 8 columns
`{:.2f}` :: 2 decimals
`{:#x}` :: hex with 0x
`std::from_chars` :: parse numbers without exceptions
`std::errc::result_out_of_range` :: number too big
`std::to_chars` :: format numbers without allocation
UTF-8 continuation byte :: 10xxxxxx
`/utf-8` :: MSVC: source and literals are UTF-8

@@exercise A strict key=value parser
Write `std::string parse(std::string_view line)` for `key=number` settings, returning `key = value` or an error:

- no `=`: `error: missing '='`
- use `std::from_chars` on the value: `invalid_argument` → `error: not a number`, `result_out_of_range` → `error: out of range`, leftover characters → `error: trailing characters after <the parsed number>`
@@starter
#include <charconv>
#include <iostream>
#include <string>
#include <string_view>

int main() {
    std::string_view lines[] = {"timeout=30", "retries=99999999999", "port=80x", "mode=abc", "novalue"};
    std::cout << "TODO\n";
}
@@expected
timeout=30 -> timeout = 30
retries=99999999999 -> error: out of range
port=80x -> error: trailing characters after 80
mode=abc -> error: not a number
novalue -> error: missing '='
@@requires
std::from_chars
std::errc::result_out_of_range
@@hint
`auto eq = line.find('='); auto key = line.substr(0, eq); auto text = line.substr(eq + 1);` then `auto [ptr, ec] = std::from_chars(text.data(), text.data() + text.size(), value);`. Build the result with `std::string(key) + " = " + std::to_string(value)`.
@@solution
#include <charconv>
#include <iostream>
#include <string>
#include <string_view>

std::string parse(std::string_view line) {
    auto eq = line.find('=');
    if (eq == std::string_view::npos) return "error: missing '='";
    auto key = line.substr(0, eq);
    auto text = line.substr(eq + 1);
    int value = 0;
    auto [ptr, ec] = std::from_chars(text.data(), text.data() + text.size(), value);
    if (ec == std::errc::invalid_argument) return "error: not a number";
    if (ec == std::errc::result_out_of_range) return "error: out of range";
    if (ptr != text.data() + text.size()) return "error: trailing characters after " + std::to_string(value);
    return std::string(key) + " = " + std::to_string(value);
}

int main() {
    std::string_view lines[] = {"timeout=30", "retries=99999999999", "port=80x", "mode=abc", "novalue"};
    for (auto line : lines) std::cout << line << " -> " << parse(line) << '\n';
}

@@exercise Counting and truncating UTF-8
Write two functions for UTF-8 text:

- `size_t code_points(std::string_view s)`: count the bytes that are **not** continuation bytes (`(c & 0xC0) != 0x80`)
- `std::string_view first(std::string_view s, size_t n)`: the first `n` code points, never cutting a character in half

Print bytes, code points and the first 3 code points of each string.
@@starter
#include <iostream>
#include <string_view>

int main() {
    std::string_view words[] = {"café", "日本語テキスト", "ok"};
    std::cout << "TODO\n";
}
@@expected
café: 5 bytes, 4 code points, first 3: caf
日本語テキスト: 21 bytes, 7 code points, first 3: 日本語
ok: 2 bytes, 2 code points, first 3: ok
@@requires
0xC0
@@hint
In `first`, walk the bytes and count the start bytes; when you're about to start code point number `n + 1`, return `s.substr(0, i)`. If the loop finishes, return the whole string. Cast to `unsigned char` before masking.
@@solution
#include <iostream>
#include <string_view>

bool is_start(char c) { return (static_cast<unsigned char>(c) & 0xC0) != 0x80; }

size_t code_points(std::string_view s) {
    size_t n = 0;
    for (char c : s)
        if (is_start(c)) ++n;
    return n;
}

std::string_view first(std::string_view s, size_t n) {
    size_t seen = 0;
    for (size_t i = 0; i < s.size(); ++i) {
        if (is_start(s[i]) && seen++ == n) return s.substr(0, i);
    }
    return s;
}

int main() {
    std::string_view words[] = {"café", "日本語テキスト", "ok"};
    for (auto w : words)
        std::cout << w << ": " << w.size() << " bytes, " << code_points(w) << " code points, first 3: " << first(w, 3) << '\n';
}
