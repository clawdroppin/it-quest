@@lesson g2-templates-regexp Text Templates and Regular Expressions
@@concepts strings, text-processing
@@text
Two standard-library packages do a lot of the text work in real Go programs: `text/template` (and its safe sibling `html/template`) turns data into text, and `regexp` pulls structure out of text. Both have a few rules that differ from other languages, and this lesson covers them.

## text/template: data in, text out

A template is text with **actions** in `{{ }}`. You parse it once and execute it with data:

```go
package main

import (
	"os"
	"text/template"
)

type Server struct {
	Name   string
	CPU    float64
	Tags   []string
	Online bool
}

const report = `{{range .}}{{.Name}}: {{if .Online}}up{{else}}DOWN{{end}}, cpu {{printf "%.0f" .CPU}}%{{with .Tags}} [{{join . ", "}}]{{end}}
{{end}}`

func main() {
	funcs := template.FuncMap{"join": func(s []string, sep string) string {
		out := ""
		for i, v := range s {
			if i > 0 {
				out += sep
			}
			out += v
		}
		return out
	}}
	t := template.Must(template.New("report").Funcs(funcs).Parse(report))
	servers := []Server{{"web01", 42.5, []string{"prod", "eu"}, true}, {"db01", 91, nil, false}}
	if err := t.Execute(os.Stdout, servers); err != nil {
		panic(err)
	}
}
```

The parts:

- **`.` (dot)** is the current data. At the top it's what you passed to `Execute`; inside `range` it's each element; inside `with` it's the value you tested.
- **Fields and methods**: `{{.Name}}` reads an **exported** field (capitalised). Unexported fields can't be reached, the most common surprise. Map keys work too: `{{.Labels.env}}`.
- **`if` / `else if` / `else` / `end`**: a value is false if it's the zero value: `false`, 0, `""`, a nil pointer, or an empty slice or map.
- **`range`** loops over slices, maps (in sorted key order) and channels, with an optional `{{else}}` for the empty case. `{{range $i, $s := .}}` gives the index too.
- **`with`** runs its body only if the value is non-empty, and sets dot to it. Handy for optional sections.
- **Pipelines**: `{{.CPU | printf "%.1f"}}` passes the value as the *last* argument of the next function.
- **Built-in functions**: `printf`, `len`, `index`, `eq`, `ne`, `lt`, `le`, `gt`, `ge`, `and`, `or`, `not`, `slice`, `print`, `println`, `html`, `js`, `urlquery`, `call`.
- **Your own functions** go in a `template.FuncMap` and must be added with `.Funcs` **before** `Parse`, because the parser checks that every function exists.
- **Whitespace control**: `{{- ` trims whitespace before the action, ` -}}` after. Use it to keep generated config files tidy without squashing the template onto one line.
- **Variables**: `{{$total := 0}}`. Since Go 1.11 you can assign to them in loops: `{{$total = add $total .Count}}`.
- **Named templates**: `{{define "row"}}...{{end}}` and `{{template "row" .}}` let you reuse pieces; `template.ParseFS` with `embed` loads a directory of templates bundled into the binary.

`template.Must` panics if parsing fails, which is right for templates written in your source code: a broken template is a bug you want to find at startup. Errors at **execution** time (a missing field on a map, a function returning an error) come back from `Execute`, so check them.

Missing map keys print `<no value>` by default. `t.Option("missingkey=error")` makes them an error instead, which is much safer for generated config files.

## html/template: the same API, with escaping

`html/template` has exactly the same API but understands HTML. It escapes each value **according to where it appears**: element text, attribute values, URLs, JavaScript and CSS each get their own escaping. A user name like `<script>alert(1)</script>` becomes harmless text.

```go,norun
t := template.Must(template.New("p").Parse(`<a href="/u/{{.ID}}" title="{{.Name}}">{{.Name}}</a>`))
```

Rules to remember:

- Always use `html/template` for HTML. `text/template` does **no** escaping and makes it trivially easy to create cross-site scripting holes.
- `template.HTML("<b>ok</b>")` marks a string as already safe. Only use it for HTML you generated yourself, never for user input.
- URLs with dangerous schemes such as `javascript:` are replaced with `#ZgotmplZ`, a deliberate marker that tells you something unsafe was blocked.

## regexp: RE2, linear time

Go's `regexp` package uses the **RE2** syntax and engine. It guarantees matching in **time linear in the input**, so a hostile pattern or input can't hang your server (no catastrophic backtracking). The price is that it doesn't support **backreferences** (`\1`) or **lookaround** (`(?=...)`, `(?<!...)`).

```go
package main

import (
	"fmt"
	"regexp"
)

var logLine = regexp.MustCompile(`^(?P<time>\S+) (?P<level>INFO|WARN|ERROR) (?P<msg>.*)$`)

func main() {
	m := logLine.FindStringSubmatch("2026-11-06T09:00:01Z ERROR disk /var at 97%")
	if m == nil {
		fmt.Println("no match")
		return
	}
	fmt.Println(m[logLine.SubexpIndex("level")], "|", m[logLine.SubexpIndex("msg")])

	pct := regexp.MustCompile(`(\d+)%`)
	fmt.Println(pct.FindAllString("cpu 91% mem 64% disk 97%", -1))
	fmt.Println(pct.ReplaceAllString("disk 97%", "${1} percent"))
}
```

The method names follow a pattern: `Find` + `All` (every match, with a limit, `-1` for no limit) + `String` (work on strings instead of `[]byte`) + `Submatch` (include capture groups) + `Index` (positions instead of text). So `FindAllStringSubmatch` returns `[][]string`: every match, each with its groups.

Other things worth knowing:

- **Compile once.** Put patterns in package-level `var`s with `regexp.MustCompile`. Compiling inside a loop is slow.
- **Use raw strings** (backticks) for patterns so `\d` doesn't need double escaping.
- `MatchString` checks whether there is a match **anywhere**; anchor with `^` and `$` to match the whole string.
- Named groups are written `(?P<name>...)` (Go 1.22 also accepts `(?<name>...)`); look up their index with `SubexpIndex`.
- Flags go at the start: `(?i)` case-insensitive, `(?m)` multi-line `^`/`$`, `(?s)` dot matches newline.
- In replacements, `$1` or `${name}` insert groups. Write `${1}x` rather than `$1x`, which would look for a group named `1x`. `ReplaceAllStringFunc` calls your function for each match, and `ReplaceAllLiteralString` disables `$` expansion.
- `regexp.QuoteMeta(userInput)` escapes text so it's matched literally.

## When not to use a regex

For fixed separators, `strings.Cut`, `strings.Fields`, `strings.Split` and `strings.HasPrefix` are faster and easier to read. Parse structured formats with real parsers: `encoding/json`, `encoding/csv`, `net/url`, `net/mail`, `time.Parse`. Reach for `regexp` when the text has a pattern but no parser exists.

@@quiz
A template uses `{{.password}}` on a struct with an unexported `password` field. What happens?
* Execution fails, because templates can only reach exported fields
- It prints the password
- It prints an empty string silently
- The template refuses to parse
? Only exported (capitalised) fields and methods are visible to templates.

@@quiz
Why must `.Funcs(funcMap)` be called before `.Parse`?
* The parser checks that every function used in the template exists
- Parse clears the function map
- Functions are only allowed in html/template
- It's a style rule, not required
? Calling Funcs after Parse leaves the parse with an unknown function error.

@@quiz
Why does Go's regexp package not support backreferences like `\1`?
* It uses the RE2 engine, which guarantees linear-time matching and can't support them
- They were removed for licensing reasons
- They are supported only with the (?b) flag
- Backreferences only work on []byte
? Linear time means no catastrophic backtracking on hostile input.

@@quiz
What is the difference between `text/template` and `html/template`?
* html/template escapes values according to their HTML context; text/template doesn't escape at all
- html/template is faster
- text/template can't use range
- They have different APIs
? Use html/template for anything that ends up in a browser.

@@flashcards
{{.Field}} :: read an exported field of the current data
{{range .}}...{{else}}...{{end}} :: loop, with a branch for empty
{{with .X}} :: run the body only if X is non-empty; dot becomes X
{{- and -}} :: trim whitespace around an action
template.FuncMap :: your own template functions (add before Parse)
missingkey=error :: fail on missing map keys instead of printing <no value>
#ZgotmplZ :: html/template blocked an unsafe value
RE2 :: linear-time regex: no backreferences or lookaround
FindAllStringSubmatch :: every match with its capture groups
SubexpIndex :: index of a named group
${1}x :: group 1 followed by x in a replacement
regexp.QuoteMeta :: escape text to match it literally

@@drill
Which template action loops over a slice?
= range
= {{range .}}
? End it with {{end}}; add {{else}} for the empty case.

@@drill
What limit argument tells FindAllString to return every match?
= -1
? Any non-negative number limits the number of matches.

@@exercise Render an on-call handover with text/template
@@concepts strings, text-processing
Write the template `handover` so that `main` prints a handover note. For each team, print its name and the on-call person; then list the open incidents with their severity padded to 4 characters, or "no open incidents" when there are none. Add a FuncMap function `upper` (with `strings.ToUpper`) and use it for team names. Finish with the total number of open incidents, computed in the template with `len` and a variable. Use `{{-` and `-}}` so there are no blank lines.
@@starter
package main

import (
	"os"
	"strings"
	"text/template"
)

type Incident struct {
	ID       string
	Severity string
	Title    string
}

type Team struct {
	Name     string
	OnCall   string
	Open     []Incident
}

const handover = `TODO
`

func main() {
	_ = strings.ToUpper
	funcs := template.FuncMap{}
	t := template.Must(template.New("handover").Funcs(funcs).Parse(handover))
	teams := []Team{
		{"platform", "Aiko", []Incident{{"INC-101", "SEV2", "Disk filling on db01"}, {"INC-104", "SEV3", "Slow builds"}}},
		{"payments", "Ben", nil},
		{"web", "Chen", []Incident{{"INC-099", "SEV1", "Checkout 500s"}}},
	}
	if err := t.Execute(os.Stdout, teams); err != nil {
		panic(err)
	}
}
@@expected
PLATFORM (on call: Aiko)
  SEV2 INC-101 Disk filling on db01
  SEV3 INC-104 Slow builds
PAYMENTS (on call: Ben)
  no open incidents
WEB (on call: Chen)
  SEV1 INC-099 Checkout 500s
Open incidents: 3
@@requires
"upper": strings.ToUpper
{{range
@@hint
Start with `{{- $total := 0}}`, loop with `{{range .}}`, and inside it loop over `.Open` with `{{range .Open}}...{{else}}  no open incidents{{end}}`. Count with `{{$total = add $total (len .Open)}}`, which needs an `add` function in the FuncMap too. Use `printf "%-4s" .Severity` for the padding.
@@solution
package main

import (
	"os"
	"strings"
	"text/template"
)

type Incident struct {
	ID       string
	Severity string
	Title    string
}

type Team struct {
	Name     string
	OnCall   string
	Open     []Incident
}

const handover = `{{- $total := 0 -}}
{{range . -}}
{{upper .Name}} (on call: {{.OnCall}})
{{range .Open}}  {{printf "%-4s" .Severity}} {{.ID}} {{.Title}}
{{else}}  no open incidents
{{end -}}
{{$total = add $total (len .Open) -}}
{{end -}}
Open incidents: {{$total}}
`

func main() {
	funcs := template.FuncMap{
		"upper": strings.ToUpper,
		"add":   func(a, b int) int { return a + b },
	}
	t := template.Must(template.New("handover").Funcs(funcs).Parse(handover))
	teams := []Team{
		{"platform", "Aiko", []Incident{{"INC-101", "SEV2", "Disk filling on db01"}, {"INC-104", "SEV3", "Slow builds"}}},
		{"payments", "Ben", nil},
		{"web", "Chen", []Incident{{"INC-099", "SEV1", "Checkout 500s"}}},
	}
	if err := t.Execute(os.Stdout, teams); err != nil {
		panic(err)
	}
}

@@exercise Parse access logs with named groups
@@concepts strings, text-processing, maps
Write `Parse(line string) (Entry, bool)` using one package-level regexp with **named groups** for the client IP, method, path, status and duration in milliseconds. Lines that don't match return `false`. Then `main` prints, per status class (2xx, 4xx, 5xx), the count and the slowest path, and masks the last octet of every IPv4 address in the bad lines with `ReplaceAllString` before printing them.
@@starter
package main

import (
	"fmt"
	"regexp"
	"sort"
	"strconv"
	"strings"
)

type Entry struct {
	IP     string
	Method string
	Path   string
	Status int
	Ms     int
}

var lineRe = regexp.MustCompile(`TODO`)
var lastOctet = regexp.MustCompile(`TODO`)

func Parse(line string) (Entry, bool) {
	_ = strconv.Atoi
	return Entry{}, false
}

const logs = `10.0.0.5 GET /api/orders 200 34ms
10.0.0.9 POST /api/orders 201 120ms
10.0.1.7 GET /api/orders/77 404 8ms
garbage line from a crashed writer
10.0.0.5 GET /api/report 500 2400ms
10.0.2.3 GET /health 200 1ms
10.0.0.9 DELETE /api/orders/12 403 11ms
10.0.3.4 GET /api/report 503 30012ms
malformed 10.0.0.1 GET`

func main() {
	type stat struct {
		count   int
		slowest Entry
	}
	stats := map[string]*stat{}
	for _, line := range strings.Split(logs, "\n") {
		e, ok := Parse(line)
		if !ok {
			fmt.Println("skipped:", lastOctet.ReplaceAllString(line, "${1}x"))
			continue
		}
		class := fmt.Sprintf("%dxx", e.Status/100)
		s := stats[class]
		if s == nil {
			s = &stat{}
			stats[class] = s
		}
		s.count++
		if e.Ms > s.slowest.Ms {
			s.slowest = e
		}
	}
	classes := make([]string, 0, len(stats))
	for c := range stats {
		classes = append(classes, c)
	}
	sort.Strings(classes)
	for _, c := range classes {
		s := stats[c]
		fmt.Printf("%s: %d requests, slowest %s %s (%dms)\n", c, s.count, s.slowest.Method, s.slowest.Path, s.slowest.Ms)
	}
}
@@expected
skipped: garbage line from a crashed writer
skipped: malformed 10.0.0.x GET
2xx: 3 requests, slowest POST /api/orders (120ms)
4xx: 2 requests, slowest DELETE /api/orders/12 (11ms)
5xx: 2 requests, slowest GET /api/report (30012ms)
@@requires
SubexpIndex(
(?P<status>
@@hint
A pattern like `^(?P<ip>\d+\.\d+\.\d+\.\d+) (?P<method>[A-Z]+) (?P<path>\S+) (?P<status>\d{3}) (?P<ms>\d+)ms$` works. Use `FindStringSubmatch`, return false on nil, and read groups with `m[lineRe.SubexpIndex("status")]`. For masking, `(\d+\.\d+\.\d+\.)\d+` with the replacement `${1}x` keeps the first three octets.
@@solution
package main

import (
	"fmt"
	"regexp"
	"sort"
	"strconv"
	"strings"
)

type Entry struct {
	IP     string
	Method string
	Path   string
	Status int
	Ms     int
}

var lineRe = regexp.MustCompile(`^(?P<ip>\d+\.\d+\.\d+\.\d+) (?P<method>[A-Z]+) (?P<path>\S+) (?P<status>\d{3}) (?P<ms>\d+)ms$`)
var lastOctet = regexp.MustCompile(`(\d+\.\d+\.\d+\.)\d+`)

func Parse(line string) (Entry, bool) {
	m := lineRe.FindStringSubmatch(line)
	if m == nil {
		return Entry{}, false
	}
	status, _ := strconv.Atoi(m[lineRe.SubexpIndex("status")])
	ms, _ := strconv.Atoi(m[lineRe.SubexpIndex("ms")])
	return Entry{
		IP:     m[lineRe.SubexpIndex("ip")],
		Method: m[lineRe.SubexpIndex("method")],
		Path:   m[lineRe.SubexpIndex("path")],
		Status: status,
		Ms:     ms,
	}, true
}

const logs = `10.0.0.5 GET /api/orders 200 34ms
10.0.0.9 POST /api/orders 201 120ms
10.0.1.7 GET /api/orders/77 404 8ms
garbage line from a crashed writer
10.0.0.5 GET /api/report 500 2400ms
10.0.2.3 GET /health 200 1ms
10.0.0.9 DELETE /api/orders/12 403 11ms
10.0.3.4 GET /api/report 503 30012ms
malformed 10.0.0.1 GET`

func main() {
	type stat struct {
		count   int
		slowest Entry
	}
	stats := map[string]*stat{}
	for _, line := range strings.Split(logs, "\n") {
		e, ok := Parse(line)
		if !ok {
			fmt.Println("skipped:", lastOctet.ReplaceAllString(line, "${1}x"))
			continue
		}
		class := fmt.Sprintf("%dxx", e.Status/100)
		s := stats[class]
		if s == nil {
			s = &stat{}
			stats[class] = s
		}
		s.count++
		if e.Ms > s.slowest.Ms {
			s.slowest = e
		}
	}
	classes := make([]string, 0, len(stats))
	for c := range stats {
		classes = append(classes, c)
	}
	sort.Strings(classes)
	for _, c := range classes {
		s := stats[c]
		fmt.Printf("%s: %d requests, slowest %s %s (%dms)\n", c, s.count, s.slowest.Method, s.slowest.Path, s.slowest.Ms)
	}
}
