@@lesson f8-binding Data Binding, Lists and the View-Model Pattern
@@concepts objects, functions
@@text
In a small app, button callbacks can update labels directly. As soon as the same data appears in several places (a status label, a list, a title bar), that turns into spaghetti. Fyne's answer is **data binding**: widgets observe values and update themselves.

## Bindings in Fyne

```go,norun
import "fyne.io/fyne/v2/data/binding"

hostname := binding.NewString()
entry := widget.NewEntryWithData(hostname)            // two-way: typing updates the binding
title := widget.NewLabelWithData(hostname)            // one-way: label follows the binding

port := binding.NewInt()
portText := binding.IntToString(port)                 // derived binding with conversion
portEntry := widget.NewEntryWithData(portText)

hostname.AddListener(binding.NewDataListener(func() {
	v, _ := hostname.Get()
	w.SetTitle("Netkit - " + v)
}))
```

- `binding.NewString/Int/Float/Bool`, lists (`NewStringList`, `NewUntypedList`) and maps, plus `BindString(&myVar)` to bind an existing variable.
- Conversions (`IntToString`, `FloatToStringWithFormat`) are two-way: invalid text returns an error instead of changing the int.
- Listeners are called once when added, then on every change.

## Lists

`widget.List` is **virtualized**: it creates only enough row widgets to fill the screen and reuses them while scrolling. You supply three callbacks:

```go,norun
list := widget.NewList(
	func() int { return vm.Length() },                           // how many rows
	func() fyne.CanvasObject { return widget.NewLabel("template") }, // create a reusable row
	func(i widget.ListItemID, o fyne.CanvasObject) {             // fill row i
		h := vm.Item(i)
		o.(*widget.Label).SetText(h.Name + " (" + h.Env + ")")
	},
)
list.OnSelected = func(i widget.ListItemID) { vm.Select(i) }
search.OnChanged = func(s string) { vm.SetFilter(s); list.Refresh() }
```

## Model, View, View-Model

| Layer | Contains | Depends on Fyne? |
|---|---|---|
| Model | domain data and rules (hosts, checks) | no |
| View-model | UI state: filter text, selection, formatted strings, enabled/disabled | no |
| View | widgets, wiring callbacks to the view-model | yes |

Everything except the thin view is plain Go and can be unit-tested without a display, which is exactly what the exercises do. The same pattern works in Wails, Gio, web frontends and mobile apps.

@@quiz
What does `widget.List` do with row widgets when you scroll?
* Reuses a small set of row widgets, refilling them via the update callback
- Creates a widget for every item up front
- Destroys and recreates them on every frame
- Nothing; lists don't scroll
? That's why lists with 100,000 items stay fast.

@@quiz
Why keep filter and selection state in a view-model instead of the widgets?
- Widgets can't hold state
* It's plain Go that can be tested without a display, and the same state drives several widgets
- Fyne requires it
- It makes the binary smaller
? Thin views, testable view-models.

@@quiz
When is a Fyne data listener first called?
- Only on the first change
* Immediately when added, then on every change
- Only when the window opens
- Never automatically
? So the UI starts in sync with the data.

@@flashcards
`binding.NewString()` :: observable string
`NewEntryWithData` :: two-way bound entry
`NewLabelWithData` :: label that follows a binding
`binding.IntToString` :: two-way conversion binding
`AddListener` :: react to changes
`widget.NewList(length, create, update)` :: virtualized list
`list.Refresh()` :: redraw after data changes
View-model :: UI state without widgets
MVVM :: Model, View, View-Model

@@exercise Your own bindings
Implement a minimal version of Fyne's bindings with generics:

- `Binding[T comparable]` with `Get()`, `Set(v)` and `AddListener(f func(T))`
- `Set` does nothing if the value is unchanged; otherwise it stores the value, then calls listeners in registration order
- `AddListener` calls the new listener **immediately** with the current value
- `Map(src, f)` returns a new binding kept equal to `f(src.Get())`
- `BindPort(port *Binding[int], text, errMsg *Binding[string])` wires a two-way int↔string conversion: when `text` changes to a valid port (1–65535), set `port` and clear `errMsg`; otherwise set `errMsg` to `not a valid port` and leave `port` alone. When `port` changes, set `text`.

The equality check in `Set` is what stops the two-way binding from looping forever.
@@starter
package main

import (
	"fmt"
	"strconv"
)

type Binding[T comparable] struct {
	value T
}

func NewBinding[T comparable](v T) *Binding[T] { return &Binding[T]{value: v} }

func main() {
	_ = strconv.Atoi
	port := NewBinding(8080)
	text := NewBinding("")
	errMsg := NewBinding("")
	_, _, _ = port, text, errMsg
	fmt.Println("TODO")
}
@@expected
label: Listening on :8080
error: ""
entry: 8080
label: Listening on :9090
entry: 9090
error: "not a valid port"
entry: abc
label: Listening on :443
error: ""
entry: 443
@@hint
Order in `main`: `label := Map(port, func(p int) string { return fmt.Sprintf("Listening on :%d", p) })`, add a listener printing `label: ...`, add one to `errMsg` printing `error: %q`, call `BindPort(port, text, errMsg)` (register the `port` listener first, then the `text` one), add a listener to `text` printing `entry: ...`, then `text.Set("9090")`, `text.Set("abc")`, `port.Set(443)`.
@@solution
package main

import (
	"fmt"
	"strconv"
)

type Binding[T comparable] struct {
	value     T
	listeners []func(T)
}

func NewBinding[T comparable](v T) *Binding[T] { return &Binding[T]{value: v} }

func (b *Binding[T]) Get() T { return b.value }

func (b *Binding[T]) Set(v T) {
	if v == b.value {
		return
	}
	b.value = v
	for _, l := range b.listeners {
		l(v)
	}
}

func (b *Binding[T]) AddListener(f func(T)) {
	b.listeners = append(b.listeners, f)
	f(b.value)
}

func Map[A, B comparable](src *Binding[A], f func(A) B) *Binding[B] {
	out := NewBinding(f(src.Get()))
	src.AddListener(func(a A) { out.Set(f(a)) })
	return out
}

func BindPort(port *Binding[int], text, errMsg *Binding[string]) {
	port.AddListener(func(p int) { text.Set(strconv.Itoa(p)) })
	text.AddListener(func(s string) {
		n, err := strconv.Atoi(s)
		if err != nil || n < 1 || n > 65535 {
			errMsg.Set("not a valid port")
			return
		}
		port.Set(n)
		errMsg.Set("")
	})
}

func main() {
	port := NewBinding(8080)
	text := NewBinding("")
	errMsg := NewBinding("")
	label := Map(port, func(p int) string { return fmt.Sprintf("Listening on :%d", p) })
	label.AddListener(func(s string) { fmt.Println("label:", s) })
	errMsg.AddListener(func(s string) { fmt.Printf("error: %q\n", s) })
	BindPort(port, text, errMsg)
	text.AddListener(func(s string) { fmt.Println("entry:", s) })
	text.Set("9090")
	text.Set("abc")
	port.Set(443)
}

@@exercise A filterable list view-model
Write the view-model behind a host list with a search box:

- `NewListModel(hosts []Host)`; `Host{Name, Env string}`
- `SetFilter(s)`: case-insensitive substring match on name **or** env; empty shows everything
- `Length()` and `Item(i)` over the **visible** hosts (what `widget.List` calls)
- `Select(i)` selects the visible item i; the selection is remembered **by name**, so it survives filtering
- `SelectedIndex()`: the selection's position among visible items, or -1 if it's filtered out
- `Rows()`: one line per visible host, `> name (env)` for the selected one, `  name (env)` otherwise
@@starter
package main

import (
	"fmt"
	"strings"
)

type Host struct{ Name, Env string }

type ListModel struct {
	all []Host
}

func NewListModel(hosts []Host) *ListModel { return &ListModel{all: hosts} }

func main() {
	_ = strings.ToLower
	vm := NewListModel([]Host{{"web01", "prod"}, {"web02", "staging"}, {"db01", "prod"}, {"cache01", "dev"}, {"db02", "staging"}})
	_ = vm
	fmt.Println("TODO")
}
@@expected
filter "": 5 items, selected 2
  web01 (prod)
  web02 (staging)
> db01 (prod)
  cache01 (dev)
  db02 (staging)
filter "prod": 2 items, selected 1
  web01 (prod)
> db01 (prod)
filter "web": 2 items, selected -1
  web01 (prod)
  web02 (staging)
filter "STAG": 2 items, selected -1
  web02 (staging)
  db02 (staging)
filter "": 5 items, selected 2
@@hint
Keep `visible []int` (indexes into `all`) and `selected string`. `SetFilter` rebuilds `visible`. In `main`: `vm.Select(2)`, then for each filter in `"", "prod", "web", "STAG"` print the header and rows; finally `SetFilter("")` and print only the header line.
@@solution
package main

import (
	"fmt"
	"strings"
)

type Host struct{ Name, Env string }

type ListModel struct {
	all      []Host
	visible  []int
	selected string
}

func NewListModel(hosts []Host) *ListModel {
	m := &ListModel{all: hosts}
	m.SetFilter("")
	return m
}

func (m *ListModel) SetFilter(s string) {
	s = strings.ToLower(s)
	m.visible = m.visible[:0]
	for i, h := range m.all {
		if s == "" || strings.Contains(strings.ToLower(h.Name), s) || strings.Contains(strings.ToLower(h.Env), s) {
			m.visible = append(m.visible, i)
		}
	}
}

func (m *ListModel) Length() int    { return len(m.visible) }
func (m *ListModel) Item(i int) Host { return m.all[m.visible[i]] }

func (m *ListModel) Select(i int) {
	if i >= 0 && i < len(m.visible) {
		m.selected = m.Item(i).Name
	}
}

func (m *ListModel) SelectedIndex() int {
	for i := range m.visible {
		if m.Item(i).Name == m.selected {
			return i
		}
	}
	return -1
}

func (m *ListModel) Rows() []string {
	var out []string
	for i := range m.visible {
		h := m.Item(i)
		prefix := "  "
		if h.Name == m.selected {
			prefix = "> "
		}
		out = append(out, fmt.Sprintf("%s%s (%s)", prefix, h.Name, h.Env))
	}
	return out
}

func main() {
	vm := NewListModel([]Host{{"web01", "prod"}, {"web02", "staging"}, {"db01", "prod"}, {"cache01", "dev"}, {"db02", "staging"}})
	vm.Select(2)
	header := func(f string) {
		fmt.Printf("filter %q: %d items, selected %d\n", f, vm.Length(), vm.SelectedIndex())
	}
	for _, f := range []string{"", "prod", "web", "STAG"} {
		vm.SetFilter(f)
		header(f)
		for _, r := range vm.Rows() {
			fmt.Println(r)
		}
	}
	vm.SetFilter("")
	header("")
}
