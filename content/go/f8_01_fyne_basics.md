@@lesson f8-fyne-basics Desktop GUIs with Fyne: Apps, Widgets, Layouts and Forms
@@concepts functions, objects
@@text
Go isn't only for servers. **Fyne** is a pure-Go GUI toolkit for desktop (Windows, macOS, Linux) and mobile apps that draws its own Material-style widgets with OpenGL. One codebase builds native apps for every platform. It's a natural choice for internal admin tools, monitoring dashboards and installers, where you want one binary, no web stack.

This module teaches the Fyne APIs with read-only examples (`fyne.io/fyne/v2` is a third-party module that needs cgo and a C compiler) and puts the testable **logic** behind the UI into exercises. That's also how you should structure real GUI apps: thin widgets, well-tested view models.

## Setup

```text
go get fyne.io/fyne/v2@latest
# Windows needs a C compiler for cgo, e.g. MSYS2 MinGW-w64 gcc; Linux needs gcc + libgl/xorg dev packages
go install fyne.io/tools/cmd/fyne@latest   # packaging tool
```

## Hello, window

```go,norun
package main

import (
	"fyne.io/fyne/v2"
	"fyne.io/fyne/v2/app"
	"fyne.io/fyne/v2/container"
	"fyne.io/fyne/v2/widget"
)

func main() {
	a := app.NewWithID("com.example.netkit") // the ID is used for preferences storage
	w := a.NewWindow("Netkit")

	status := widget.NewLabel("Ready")
	host := widget.NewEntry()
	host.SetPlaceHolder("hostname or IP")
	ping := widget.NewButton("Ping", func() {
		status.SetText("Pinging " + host.Text + "...")
	})

	w.SetContent(container.NewVBox(host, ping, status))
	w.Resize(fyne.NewSize(360, 160))
	w.ShowAndRun() // runs the event loop until the window closes
}
```

## Widgets

`Label`, `Entry` (single/multi-line, password), `Button`, `Check`, `RadioGroup`, `Select`, `Slider`, `ProgressBar` and `ProgressBarInfinite`, `List`, `Table`, `Tree`, `Card`, `Form`, `Toolbar`, `RichText` (Markdown). `dialog` provides message, confirm, file open/save and custom dialogs.

## Layouts and containers

| Container | Arranges children |
|---|---|
| `container.NewVBox` / `NewHBox` | stacked vertically / horizontally |
| `container.NewBorder(top, bottom, left, right, center)` | edges + a center that fills the rest |
| `container.NewGridWithColumns(n)` | equal cells, n columns |
| `container.NewGridWrap(size)` | fixed-size cells wrapped to the width |
| `container.NewAppTabs`, `NewHSplit`, `NewScroll` | tabs, resizable split, scrolling |

Sizes are in device-independent units, so layouts work on HiDPI screens. A custom layout implements `MinSize` and `Layout`, which is what the second exercise computes.

## Forms and validation

```go,norun
port := widget.NewEntry()
port.Validator = func(s string) error { // shows an error icon and blocks submit
	n, err := strconv.Atoi(s)
	if err != nil || n < 1 || n > 65535 {
		return errors.New("port must be 1-65535")
	}
	return nil
}
form := &widget.Form{
	Items:    []*widget.FormItem{{Text: "Host", Widget: host}, {Text: "Port", Widget: port}},
	OnSubmit: func() { save(host.Text, port.Text) },
}
```

A validator is just `func(string) error`, so validation logic is plain, testable Go.

## Themes, resources and packaging

- `a.Settings().SetTheme(...)` or a custom `fyne.Theme` for brand colours; light/dark follow the OS.
- `fyne bundle icon.png > bundled.go` embeds resources (or use `//go:embed`).
- `fyne package -os windows -icon icon.png` creates a proper `.exe` with icon and metadata; `fyne-cross` builds for other OSes using Docker.
- `fyne.io/fyne/v2/test` provides a headless test driver: `test.Type(entry, "web01")`, `test.Tap(button)`.

## Alternatives

- **Wails**: Go backend + web frontend (HTML/CSS/JS) in a system webview, like Tauri for Go.
- **Gio** (`gioui.org`): immediate-mode GUI, very flexible, lower-level.
- A local web UI served by `net/http` and opened in the browser is often the simplest "GUI" for ops tools.

@@quiz
What does `w.ShowAndRun()` do?
* Shows the window and runs the app's event loop until it quits
- Starts a web server
- Shows the window and returns immediately
- Compiles the UI
? Code after it runs when the app exits.

@@quiz
Which container gives a toolbar at the top, a status bar at the bottom and content filling the middle?
- VBox
* Border
- GridWrap
- HBox
? `container.NewBorder(toolbar, statusBar, nil, nil, content)`.

@@quiz
Why does Fyne need a C compiler on Windows?
- For the Go compiler itself
* It uses cgo to talk to OpenGL and the windowing system
- For packaging only
- It doesn't
? MSYS2's MinGW gcc is the common choice.

@@drill
Which Fyne function creates a new application?
= app.New
= app.New()
? a := app.New(); w := a.NewWindow("Hi").

@@flashcards
`app.NewWithID` :: create a Fyne app with a preferences ID
`a.NewWindow` :: create a window
`w.SetContent` :: set the window's root object
`widget.NewEntry` :: text input
`Entry.Validator` :: `func(string) error`
`container.NewBorder` :: edges + filling center
`container.NewGridWrap` :: fixed-size wrapped cells
`dialog.ShowConfirm` :: yes/no dialog
`fyne package` :: build a distributable app
Wails :: Go + web frontend desktop apps

@@exercise Composable form validators
Fyne validators are `func(string) error`. Build a small library:

- `type Validator func(string) error`
- `Required()`, `MinLen(n)`, `Matches(re *regexp.Regexp, msg string)`, `IntRange(lo, hi)` (message `must be a number between lo and hi`)
- `All(vs ...Validator) Validator`: runs validators in order and returns the **first** error
- `Form` holds named fields (in order) with a value and a validator; `Errors()` returns `field: message` lines for invalid fields, and `CanSubmit()` is true when there are none

Messages: `is required`, `must be at least N characters`.
@@starter
package main

import (
	"errors"
	"fmt"
	"regexp"
	"strconv"
)

type Validator func(string) error

type Field struct {
	Name, Value string
	Validate    Validator
}

type Form struct{ Fields []*Field }

func main() {
	_ = errors.New
	_ = strconv.Atoi
	hostRe := regexp.MustCompile(`^[a-z0-9.-]+$`)
	_ = hostRe
	fmt.Println("TODO")
}
@@expected
host: is required
port: must be a number between 1 and 65535
user: must be at least 3 characters
can submit: false
host: must be lowercase letters, digits, dots or dashes
can submit: false
can submit: true
@@hint
`func Required() Validator { return func(s string) error { if strings.TrimSpace(s) == "" { return errors.New("is required") }; return nil } }`. Build the form with host `All(Required(), Matches(hostRe, "must be lowercase letters, digits, dots or dashes"))`, port `IntRange(1, 65535)`, user `All(Required(), MinLen(3))`. Start with values `""`, `"99999"`, `"ab"`; then set host `"Web_01"`, port `"8080"`, user `"ana"`; then host `"web01"`.
@@solution
package main

import (
	"errors"
	"fmt"
	"regexp"
	"strconv"
	"strings"
)

type Validator func(string) error

func Required() Validator {
	return func(s string) error {
		if strings.TrimSpace(s) == "" {
			return errors.New("is required")
		}
		return nil
	}
}

func MinLen(n int) Validator {
	return func(s string) error {
		if len([]rune(s)) < n {
			return fmt.Errorf("must be at least %d characters", n)
		}
		return nil
	}
}

func Matches(re *regexp.Regexp, msg string) Validator {
	return func(s string) error {
		if !re.MatchString(s) {
			return errors.New(msg)
		}
		return nil
	}
}

func IntRange(lo, hi int) Validator {
	return func(s string) error {
		n, err := strconv.Atoi(s)
		if err != nil || n < lo || n > hi {
			return fmt.Errorf("must be a number between %d and %d", lo, hi)
		}
		return nil
	}
}

func All(vs ...Validator) Validator {
	return func(s string) error {
		for _, v := range vs {
			if err := v(s); err != nil {
				return err
			}
		}
		return nil
	}
}

type Field struct {
	Name, Value string
	Validate    Validator
}

type Form struct{ Fields []*Field }

func (f *Form) Errors() []string {
	var out []string
	for _, fl := range f.Fields {
		if err := fl.Validate(fl.Value); err != nil {
			out = append(out, fl.Name+": "+err.Error())
		}
	}
	return out
}

func (f *Form) CanSubmit() bool { return len(f.Errors()) == 0 }

func main() {
	hostRe := regexp.MustCompile(`^[a-z0-9.-]+$`)
	host := &Field{"host", "", All(Required(), Matches(hostRe, "must be lowercase letters, digits, dots or dashes"))}
	port := &Field{"port", "99999", IntRange(1, 65535)}
	user := &Field{"user", "ab", All(Required(), MinLen(3))}
	form := &Form{Fields: []*Field{host, port, user}}
	show := func() {
		for _, e := range form.Errors() {
			fmt.Println(e)
		}
		fmt.Println("can submit:", form.CanSubmit())
	}
	show()
	host.Value, port.Value, user.Value = "Web_01", "8080", "ana"
	show()
	host.Value = "web01"
	show()
}

@@exercise A grid-wrap layout
Implement the maths of a layout like `container.NewGridWrap`: fixed-size cells placed left to right, wrapping to the next row when the next cell wouldn't fit.

- `GridWrap(n int, cell Size, pad int, width int) []Rect`: positions for `n` cells; cells are separated by `pad` horizontally and vertically, with no padding at the edges; at least one cell per row even if it's wider than `width`
- `MinHeight(n int, cell Size, pad int, width int) int`: total height needed
- `Render(rects []Rect, width, height, scale int) []string`: an ASCII preview where each character is `scale` units; cell `i` is drawn with the digit `i % 10`, empty space with `.`

All values are integers (device-independent units).
@@starter
package main

import (
	"fmt"
	"strings"
)

type Size struct{ W, H int }

type Rect struct{ X, Y, W, H int }

func main() {
	_ = strings.Repeat
	fmt.Println("TODO")
}
@@expected
[{0 0 40 20} {50 0 40 20} {100 0 40 20} {0 30 40 20} {50 30 40 20}]
min height: 50
0000.1111.2222
0000.1111.2222
..............
3333.4444.....
3333.4444.....
min height 3 columns -> 1 column: 50 -> 140
@@hint
Cells per row: `perRow := max(1, (width+pad)/(cell.W+pad))`. Cell i: `col, row := i%perRow, i/perRow`, `X = col*(cell.W+pad)`, `Y = row*(cell.H+pad)`. Rows: `(n + perRow - 1) / perRow`; height `rows*cell.H + (rows-1)*pad`. For the preview, build a `[][]byte` of `width/scale` × `height/scale` dots and fill each rect's cells. `main`: `rects := GridWrap(5, Size{40, 20}, 10, 150)`, print them and `MinHeight` with the same arguments, print `Render(rects, 140, height, 10)` line by line, then compare with `MinHeight(5, Size{40, 20}, 10, 45)` (one column).
@@solution
package main

import (
	"fmt"
	"strings"
)

type Size struct{ W, H int }

type Rect struct{ X, Y, W, H int }

func perRow(cell Size, pad, width int) int {
	return max(1, (width+pad)/(cell.W+pad))
}

func GridWrap(n int, cell Size, pad int, width int) []Rect {
	pr := perRow(cell, pad, width)
	out := make([]Rect, 0, n)
	for i := range n {
		col, row := i%pr, i/pr
		out = append(out, Rect{col * (cell.W + pad), row * (cell.H + pad), cell.W, cell.H})
	}
	return out
}

func MinHeight(n int, cell Size, pad int, width int) int {
	if n == 0 {
		return 0
	}
	pr := perRow(cell, pad, width)
	rows := (n + pr - 1) / pr
	return rows*cell.H + (rows-1)*pad
}

func Render(rects []Rect, width, height, scale int) []string {
	grid := make([][]byte, height/scale)
	for y := range grid {
		grid[y] = []byte(strings.Repeat(".", width/scale))
	}
	for i, r := range rects {
		for y := r.Y / scale; y < (r.Y+r.H)/scale && y < len(grid); y++ {
			for x := r.X / scale; x < (r.X+r.W)/scale && x < len(grid[y]); x++ {
				grid[y][x] = byte('0' + i%10)
			}
		}
	}
	out := make([]string, len(grid))
	for i, row := range grid {
		out[i] = string(row)
	}
	return out
}

func main() {
	cell := Size{40, 20}
	rects := GridWrap(5, cell, 10, 150)
	fmt.Println(rects)
	h := MinHeight(5, cell, 10, 150)
	fmt.Println("min height:", h)
	for _, line := range Render(rects, 140, h, 10) {
		fmt.Println(line)
	}
	fmt.Printf("min height 3 columns -> 1 column: %d -> %d\n", h, MinHeight(5, cell, 10, 45))
}
