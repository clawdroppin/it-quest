@@lesson g1-pointers Pointers, Values and Escape Analysis
@@concepts variables, functions
@@text
Go passes **everything by value**: a function gets a copy of what you pass. Pointers let you share and modify a value instead of copying it. Unlike C, Go pointers are safe: no pointer arithmetic, and the garbage collector frees memory for you.

## & and *

```go
package main

import "fmt"

func main() {
	port := 80
	p := &port         // p is a *int: the address of port
	*p = 443           // write through the pointer
	fmt.Println(port)  // 443
	fmt.Println(p != nil, *p)

	q := new(int)      // a pointer to a new zero int
	*q++
	fmt.Println(*q)
}
```

## Copies vs sharing

```go
package main

import "fmt"

type Server struct {
	Name string
	Up   bool
}

func markUpCopy(s Server)     { s.Up = true } // changes the copy only
func markUpShared(s *Server)  { s.Up = true } // changes the caller's value

func main() {
	s := Server{Name: "web01"}
	markUpCopy(s)
	fmt.Println(s.Up) // false
	markUpShared(&s)
	fmt.Println(s.Up) // true
}
```

Slices, maps, channels, functions and interfaces are small **headers** that already refer to shared data, so passing them copies only the header: a function can change the elements of a slice you pass, but `append` growing it isn't visible to the caller unless you return the new slice.

## Value or pointer receivers?

- Use a **pointer receiver** (`func (s *Server) Restart()`) if the method changes the value, if the struct is large, or if it contains a `sync.Mutex` (which must never be copied).
- Use a **value receiver** for small, immutable types (`time.Time`, a `Point`).
- Be consistent: if some methods need pointers, give all of them pointer receivers.

## The loop variable trap (fixed in Go 1.22)

Since Go 1.22 each loop iteration has its **own** variable, so taking `&item` inside a `range` loop is safe. Older code copied values to work around the old shared variable; you'll still see `item := item` in older codebases.

## Escape analysis: stack or heap?

You never choose where memory lives. The compiler's **escape analysis** keeps values on the fast stack when it can prove they don't outlive the function, and moves ("escapes") them to the heap otherwise, e.g. when you return a pointer to a local variable:

```go,norun
func newServer(name string) *Server {
	s := Server{Name: name} // escapes to the heap: its address outlives the call
	return &s               // perfectly safe in Go (a classic bug in C)
}
```

See the decisions with `go build -gcflags=-m`. Fewer heap allocations means less garbage-collector work, which matters in hot paths, but write clear code first and measure before optimizing.

## nil pointers

Dereferencing a `nil` pointer panics: `invalid memory address or nil pointer dereference`. Check for `nil` at boundaries (results from lookups, optional fields), or return values instead of pointers when "missing" isn't a valid state. Methods with pointer receivers can be called on a nil pointer and can handle it themselves.

@@quiz
What does Go pass to a function when you call `f(s)` with a struct `s`?
* A copy of the struct
- A reference to the struct
- A pointer automatically
- Nothing until it's used
? Use `f(&s)` with a `*Server` parameter to share it.

@@quiz
When should a method have a pointer receiver?
- Never; Go doesn't have pointers in methods
* When it modifies the value, the struct is large, or it contains a mutex
- Only for exported methods
- Only for interfaces
? Be consistent across a type's methods.

@@quiz
Is it safe in Go to return a pointer to a local variable?
* Yes; escape analysis moves it to the heap
- No; it's a dangling pointer as in C
- Only with `new`
- Only for structs
? The garbage collector keeps it alive.

@@drill
Which operator takes the address of a variable?
= &
? * follows a pointer to its value.

@@flashcards
`&x` :: address of x (a pointer)
`*p` :: the value p points to
`new(T)` :: pointer to a new zero T
Pass by value :: functions get copies
Pointer receiver :: `func (s *T) M()` can modify s
Escape analysis :: decides stack vs heap
`-gcflags=-m` :: show escape decisions
nil pointer dereference :: runtime panic

@@exercise Fix the inventory
The starter's `Inventory` loses changes because of value copies. Fix it so that:

- `Add(name string, qty int)` and `Take(name string, qty int) error` have pointer receivers and change the inventory (`Take` returns `not enough <name>: have <n>, want <q>` when there aren't enough)
- `Restock` (a plain function) takes a `*Inventory` and adds 10 of every item that has fewer than 5
- `Find(name string) *Item` returns a pointer **into** the inventory's slice (or nil), so callers can change the item

Keep `main` as given.
@@starter
package main

import (
	"fmt"
	"strings"
)

type Item struct {
	Name string
	Qty  int
}

type Inventory struct {
	Items []Item
}

func (inv Inventory) Add(name string, qty int) {
	for _, it := range inv.Items {
		if it.Name == name {
			it.Qty += qty
			return
		}
	}
	inv.Items = append(inv.Items, Item{name, qty})
}

func (inv Inventory) Take(name string, qty int) error {
	return nil
}

func Restock(inv *Inventory) {}

func (inv Inventory) Find(name string) *Item { return nil }

func (inv Inventory) String() string {
	parts := []string{}
	for _, it := range inv.Items {
		parts = append(parts, fmt.Sprintf("%s=%d", it.Name, it.Qty))
	}
	return strings.Join(parts, " ")
}

func main() {
	var inv Inventory
	inv.Add("cable", 3)
	inv.Add("mouse", 8)
	inv.Add("cable", 1)
	fmt.Println(inv)
	fmt.Println(inv.Take("mouse", 5), inv.Take("mouse", 5))
	Restock(&inv)
	fmt.Println(inv)
	if it := inv.Find("cable"); it != nil {
		it.Qty = 99
	}
	fmt.Println(inv, inv.Find("dock") == nil)
}
@@expected
cable=4 mouse=8
<nil> not enough mouse: have 3, want 5
cable=14 mouse=13
cable=99 mouse=13 true
@@requires
func (inv *Inventory) Add
func (inv *Inventory) Take
func Restock(inv *Inventory)
@@hint
Range over indexes to modify elements in place: `for i := range inv.Items { if inv.Items[i].Name == name { inv.Items[i].Qty += qty; return } }`. `Find` returns `&inv.Items[i]`, and `Add` and `Take` can use `Find`. Leave `String` with its value receiver so `fmt.Println(inv)` still uses it.
@@solution
package main

import (
	"fmt"
	"strings"
)

type Item struct {
	Name string
	Qty  int
}

type Inventory struct {
	Items []Item
}

func (inv *Inventory) Add(name string, qty int) {
	if it := inv.Find(name); it != nil {
		it.Qty += qty
		return
	}
	inv.Items = append(inv.Items, Item{name, qty})
}

func (inv *Inventory) Take(name string, qty int) error {
	it := inv.Find(name)
	have := 0
	if it != nil {
		have = it.Qty
	}
	if have < qty {
		return fmt.Errorf("not enough %s: have %d, want %d", name, have, qty)
	}
	it.Qty -= qty
	return nil
}

func Restock(inv *Inventory) {
	for i := range inv.Items {
		if inv.Items[i].Qty < 5 {
			inv.Items[i].Qty += 10
		}
	}
}

func (inv *Inventory) Find(name string) *Item {
	for i := range inv.Items {
		if inv.Items[i].Name == name {
			return &inv.Items[i]
		}
	}
	return nil
}

func (inv Inventory) String() string {
	parts := []string{}
	for _, it := range inv.Items {
		parts = append(parts, fmt.Sprintf("%s=%d", it.Name, it.Qty))
	}
	return strings.Join(parts, " ")
}

func main() {
	var inv Inventory
	inv.Add("cable", 3)
	inv.Add("mouse", 8)
	inv.Add("cable", 1)
	fmt.Println(inv)
	fmt.Println(inv.Take("mouse", 5), inv.Take("mouse", 5))
	Restock(&inv)
	fmt.Println(inv)
	if it := inv.Find("cable"); it != nil {
		it.Qty = 99
	}
	fmt.Println(inv, inv.Find("dock") == nil)
}

@@exercise A binary search tree of IPs
Build a binary search tree with pointers to keep IPv4 addresses sorted:

- `type Node struct { IP uint32; Left, Right *Node }` and `func Insert(root *Node, ip uint32) *Node` (returns the new root; duplicates are ignored)
- `func (n *Node) InOrder(visit func(uint32))`: must work on a nil `*Node` (an empty tree)
- `func (n *Node) Height() int`: 0 for nil
- `ParseIP(s string) uint32` and `FormatIP(ip uint32) string` (assume valid input)

`main` inserts the addresses and prints them sorted, then the height.
@@starter
package main

import (
	"fmt"
	"strconv"
	"strings"
)

type Node struct {
	IP          uint32
	Left, Right *Node
}

func Insert(root *Node, ip uint32) *Node {
	return root
}

func (n *Node) InOrder(visit func(uint32)) {}

func (n *Node) Height() int { return 0 }

func ParseIP(s string) uint32 {
	_ = strings.Split(s, ".")
	_ = strconv.Itoa
	return 0
}

func FormatIP(ip uint32) string { return "" }

func main() {
	var root *Node
	for _, s := range []string{"10.0.0.20", "10.0.0.3", "192.168.1.1", "10.0.0.3", "172.16.0.9", "10.0.0.100"} {
		root = Insert(root, ParseIP(s))
	}
	root.InOrder(func(ip uint32) { fmt.Println(FormatIP(ip)) })
	fmt.Println("height:", root.Height())
	var empty *Node
	empty.InOrder(func(uint32) { fmt.Println("never") })
	fmt.Println("empty height:", empty.Height())
}
@@expected
10.0.0.3
10.0.0.20
10.0.0.100
172.16.0.9
192.168.1.1
height: 4
empty height: 0
@@requires
func (n *Node) InOrder
@@hint
`Insert`: `if root == nil { return &Node{IP: ip} }`, then recurse left or right and assign: `root.Left = Insert(root.Left, ip)`. Methods on a nil pointer receiver are allowed: start `InOrder` and `Height` with `if n == nil { return ... }`. `ParseIP`: shift and OR the four octets: `ip = ip<<8 | uint32(n)`.
@@solution
package main

import (
	"fmt"
	"strconv"
	"strings"
)

type Node struct {
	IP          uint32
	Left, Right *Node
}

func Insert(root *Node, ip uint32) *Node {
	if root == nil {
		return &Node{IP: ip}
	}
	switch {
	case ip < root.IP:
		root.Left = Insert(root.Left, ip)
	case ip > root.IP:
		root.Right = Insert(root.Right, ip)
	}
	return root
}

func (n *Node) InOrder(visit func(uint32)) {
	if n == nil {
		return
	}
	n.Left.InOrder(visit)
	visit(n.IP)
	n.Right.InOrder(visit)
}

func (n *Node) Height() int {
	if n == nil {
		return 0
	}
	return 1 + max(n.Left.Height(), n.Right.Height())
}

func ParseIP(s string) uint32 {
	var ip uint32
	for _, part := range strings.Split(s, ".") {
		n, _ := strconv.Atoi(part)
		ip = ip<<8 | uint32(n)
	}
	return ip
}

func FormatIP(ip uint32) string {
	return fmt.Sprintf("%d.%d.%d.%d", ip>>24, ip>>16&0xff, ip>>8&0xff, ip&0xff)
}

func main() {
	var root *Node
	for _, s := range []string{"10.0.0.20", "10.0.0.3", "192.168.1.1", "10.0.0.3", "172.16.0.9", "10.0.0.100"} {
		root = Insert(root, ParseIP(s))
	}
	root.InOrder(func(ip uint32) { fmt.Println(FormatIP(ip)) })
	fmt.Println("height:", root.Height())
	var empty *Node
	empty.InOrder(func(uint32) { fmt.Println("never") })
	fmt.Println("empty height:", empty.Height())
}
