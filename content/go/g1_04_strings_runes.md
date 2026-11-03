@@lesson g1-strings Strings, Bytes, Runes and Unicode
@@concepts strings, text-processing
@@text
A Go `string` is an immutable sequence of **bytes**, almost always UTF-8 text. That's efficient and simple, but it means "length" and "character" need care as soon as text isn't plain English: Japanese, emoji, accented names.

## Bytes vs runes

```go
package main

import (
	"fmt"
	"unicode/utf8"
)

func main() {
	s := "東京 DC1"
	fmt.Println(len(s))                    // 10 bytes: each kanji is 3 bytes in UTF-8
	fmt.Println(utf8.RuneCountInString(s)) // 6 characters (runes)
	fmt.Println(s[0])                      // 230: indexing gives a byte, not a character
	for i, r := range s {                  // range decodes runes; i is the byte offset
		fmt.Printf("%d:%c ", i, r)
	}
	fmt.Println()
	runes := []rune(s)
	fmt.Println(string(runes[0:2])) // 東京
}
```

- `byte` is an alias for `uint8`; `rune` is an alias for `int32` (a Unicode code point).
- Slicing a string (`s[0:2]`) slices **bytes** and can cut a character in half. Convert to `[]rune` (or iterate with `range`) when you work with characters.
- Invalid UTF-8 bytes decode as `U+FFFD` (�) in `range`; check with `utf8.ValidString`.

## Building strings efficiently

Strings are immutable, so `s += x` in a loop copies everything each time. Use `strings.Builder`:

```go
package main

import (
	"fmt"
	"strings"
)

func main() {
	var b strings.Builder
	for i := 1; i <= 3; i++ {
		fmt.Fprintf(&b, "web%02d,", i)
	}
	out := strings.TrimSuffix(b.String(), ",")
	fmt.Println(out)
}
```

## The strings, strconv and unicode packages

| Need | Use |
|---|---|
| search | `strings.Contains`, `HasPrefix`, `Index`, `Cut` |
| split / join | `strings.Split`, `Fields` (on whitespace), `Join`, `strings.Lines` |
| case | `strings.ToUpper`, `EqualFold` (case-insensitive compare) |
| clean | `TrimSpace`, `Trim`, `TrimPrefix`, `ReplaceAll` |
| numbers ↔ text | `strconv.Atoi`, `ParseInt(s, 10, 64)`, `ParseFloat`, `ParseBool`, `Itoa`, `FormatInt`, `Quote` |
| character classes | `unicode.IsLetter`, `IsDigit`, `IsSpace`, `IsUpper`, `unicode.Is(unicode.Han, r)` |

`strings.Cut` is the idiomatic way to split once: `key, value, ok := strings.Cut(line, "=")`.

## Display width

In a terminal, CJK characters (kanji, kana, hangul) and most emoji take **two** columns. Tables that align columns by `len()` or rune count break with Japanese text. Libraries like `go-runewidth` handle the full rules; the exercise implements a simplified version.

## Byte slices

`[]byte` is the mutable counterpart; files and network connections deal in bytes. The `bytes` package mirrors `strings` (`bytes.Contains`, `bytes.Split`, `bytes.Buffer`). Converting `string(b)` and `[]byte(s)` copies the data.

@@quiz
What does `len("東京")` return?
- 2
* 6
- 4
- It doesn't compile
? len counts bytes; each of these kanji is 3 bytes in UTF-8.

@@quiz
What does `for i, r := range s` give you for a string?
* Byte offsets and decoded runes (characters)
- Indexes and bytes
- Only bytes
- Lines
? range over a string decodes UTF-8.

@@quiz
Why use `strings.Builder` instead of `s += x` in a loop?
- `+=` doesn't work on strings
* Strings are immutable, so `+=` copies the whole string every time
- Builder sorts the result
- Builder is required by gofmt
? Builder grows a buffer and copies once at the end.

@@flashcards
`byte` :: alias for uint8
`rune` :: alias for int32, a Unicode code point
`len(s)` :: number of bytes
`utf8.RuneCountInString` :: number of characters
`[]rune(s)` :: decode a string into code points
`strings.Builder` :: efficient string building
`strings.Cut` :: split once into before/after
`strconv.Atoi` :: text to int
`strings.EqualFold` :: case-insensitive equality
Display width :: CJK characters take 2 terminal columns

@@exercise Truncate and align Japanese text
Write:

- `Width(s string) int`: terminal display width; runes in the ranges U+1100–U+115F, U+2E80–U+A4CF, U+AC00–U+D7A3, U+F900–U+FAFF, U+FE30–U+FE4F, U+FF00–U+FF60, U+FFE0–U+FFE6 and U+1F300–U+1FAFF count as 2, everything else as 1
- `Truncate(s string, max int) string`: if wider than `max`, cut whole runes so the result **plus** `"…"` (width 1) fits in `max`
- `PadRight(s string, width int) string`: adds spaces up to the display width

`main` prints a small aligned table.
@@starter
package main

import (
	"fmt"
	"strings"
)

func Width(s string) int { return len(s) }

func Truncate(s string, max int) string { return s }

func PadRight(s string, width int) string { return s + strings.Repeat(" ", 0) }

func main() {
	rows := [][2]string{
		{"web01", "Tokyo office"},
		{"東京-db", "大阪バックアップセンター"},
		{"ｶﾅ", "half-width kana"},
		{"emoji🚀", "launch pad"},
	}
	for _, r := range rows {
		fmt.Printf("|%s|%s|\n", PadRight(r[0], 8), PadRight(Truncate(r[1], 12), 12))
	}
	fmt.Println(Width("日本語abc"), Truncate("abc", 5), Truncate("abcdef", 4))
}
@@expected
|web01   |Tokyo office|
|東京-db |大阪バック… |
|ｶﾅ      |half-width …|
|emoji🚀 |launch pad  |
9 abc abc…
@@hint
Write `func runeWidth(r rune) int` with a list of `[2]rune` ranges. In `Truncate`, return `s` unchanged if `Width(s) <= max`; otherwise add runes while `w + runeWidth(r) <= max-1`, then append `"…"`. `PadRight`: `s + strings.Repeat(" ", width-Width(s))` when positive. Note the half-width kana `ｶﾅ` (U+FF76, U+FF85) is outside U+FF00–U+FF60, so it's width 1.
@@solution
package main

import (
	"fmt"
	"strings"
)

var wide = [][2]rune{
	{0x1100, 0x115F}, {0x2E80, 0xA4CF}, {0xAC00, 0xD7A3}, {0xF900, 0xFAFF},
	{0xFE30, 0xFE4F}, {0xFF00, 0xFF60}, {0xFFE0, 0xFFE6}, {0x1F300, 0x1FAFF},
}

func runeWidth(r rune) int {
	for _, rg := range wide {
		if r >= rg[0] && r <= rg[1] {
			return 2
		}
	}
	return 1
}

func Width(s string) int {
	w := 0
	for _, r := range s {
		w += runeWidth(r)
	}
	return w
}

func Truncate(s string, max int) string {
	if Width(s) <= max {
		return s
	}
	var b strings.Builder
	w := 0
	for _, r := range s {
		if w+runeWidth(r) > max-1 {
			break
		}
		b.WriteRune(r)
		w += runeWidth(r)
	}
	return b.String() + "…"
}

func PadRight(s string, width int) string {
	if pad := width - Width(s); pad > 0 {
		return s + strings.Repeat(" ", pad)
	}
	return s
}

func main() {
	rows := [][2]string{
		{"web01", "Tokyo office"},
		{"東京-db", "大阪バックアップセンター"},
		{"ｶﾅ", "half-width kana"},
		{"emoji🚀", "launch pad"},
	}
	for _, r := range rows {
		fmt.Printf("|%s|%s|\n", PadRight(r[0], 8), PadRight(Truncate(r[1], 12), 12))
	}
	fmt.Println(Width("日本語abc"), Truncate("abc", 5), Truncate("abcdef", 4))
}

@@exercise Parse a config file
Write `ParseConfig(text string) (map[string]string, []string)` for INI-like files:

- lines are trimmed; empty lines and lines starting with `#` or `;` are skipped
- `[section]` lines set a section; keys become `section.key` (no prefix before any section)
- `key = value`: split on the **first** `=` with `strings.Cut`; key is trimmed and lower-cased; value is trimmed, and surrounding double quotes are removed with `strconv.Unquote` (which also handles `\n` escapes)
- problems are collected as `line <n>: <reason>` (`missing =`, `empty key`, `bad quoted value`) and the line is skipped

Also write `GetInt(cfg map[string]string, key string, def int) int` that returns the default if the key is missing or not a valid integer. `main` prints the keys sorted.
@@starter
package main

import (
	"fmt"
	"slices"
	"strconv"
	"strings"
)

func ParseConfig(text string) (map[string]string, []string) {
	_ = strings.Cut
	_ = strconv.Unquote
	return map[string]string{}, nil
}

func GetInt(cfg map[string]string, key string, def int) int { return def }

const file = `# server config
name = edge-proxy
[http]
Port = 8080
banner = "Welcome\nto Tokyo"
timeout
[db]
host = db01 ; not a comment here
pool = lots
= orphan
dsn = "postgres://db01/app
`

func main() {
	cfg, problems := ParseConfig(file)
	keys := make([]string, 0, len(cfg))
	for k := range cfg {
		keys = append(keys, k)
	}
	slices.Sort(keys)
	for _, k := range keys {
		fmt.Printf("%s=%q\n", k, cfg[k])
	}
	for _, p := range problems {
		fmt.Println(p)
	}
	fmt.Println(GetInt(cfg, "http.port", 80), GetInt(cfg, "db.pool", 5), GetInt(cfg, "db.max", 10))
}
@@expected
db.host="db01 ; not a comment here"
db.pool="lots"
http.banner="Welcome\nto Tokyo"
http.port="8080"
name="edge-proxy"
line 6: missing =
line 10: empty key
line 11: bad quoted value
8080 5 10
@@requires
strings.Cut
@@hint
Loop with `for i, line := range strings.Split(text, "\n")` and use `i+1` as the line number. A section line: `strings.HasPrefix(line, "[") && strings.HasSuffix(line, "]")`. Only try `strconv.Unquote` when the value starts with `"`; if it fails, record `bad quoted value`.
@@solution
package main

import (
	"fmt"
	"slices"
	"strconv"
	"strings"
)

func ParseConfig(text string) (map[string]string, []string) {
	cfg := map[string]string{}
	var problems []string
	section := ""
	for i, raw := range strings.Split(text, "\n") {
		line := strings.TrimSpace(raw)
		n := i + 1
		if line == "" || strings.HasPrefix(line, "#") || strings.HasPrefix(line, ";") {
			continue
		}
		if strings.HasPrefix(line, "[") && strings.HasSuffix(line, "]") {
			section = strings.TrimSpace(line[1 : len(line)-1])
			continue
		}
		key, value, ok := strings.Cut(line, "=")
		if !ok {
			problems = append(problems, fmt.Sprintf("line %d: missing =", n))
			continue
		}
		key = strings.ToLower(strings.TrimSpace(key))
		value = strings.TrimSpace(value)
		if key == "" {
			problems = append(problems, fmt.Sprintf("line %d: empty key", n))
			continue
		}
		if strings.HasPrefix(value, `"`) {
			unq, err := strconv.Unquote(value)
			if err != nil {
				problems = append(problems, fmt.Sprintf("line %d: bad quoted value", n))
				continue
			}
			value = unq
		}
		if section != "" {
			key = section + "." + key
		}
		cfg[key] = value
	}
	return cfg, problems
}

func GetInt(cfg map[string]string, key string, def int) int {
	v, ok := cfg[key]
	if !ok {
		return def
	}
	n, err := strconv.Atoi(v)
	if err != nil {
		return def
	}
	return n
}

const file = `# server config
name = edge-proxy
[http]
Port = 8080
banner = "Welcome\nto Tokyo"
timeout
[db]
host = db01 ; not a comment here
pool = lots
= orphan
dsn = "postgres://db01/app
`

func main() {
	cfg, problems := ParseConfig(file)
	keys := make([]string, 0, len(cfg))
	for k := range cfg {
		keys = append(keys, k)
	}
	slices.Sort(keys)
	for _, k := range keys {
		fmt.Printf("%s=%q\n", k, cfg[k])
	}
	for _, p := range problems {
		fmt.Println(p)
	}
	fmt.Println(GetInt(cfg, "http.port", 80), GetInt(cfg, "db.pool", 5), GetInt(cfg, "db.max", 10))
}
