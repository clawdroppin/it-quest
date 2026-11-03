@@lesson g2-slices-deep Slices in Depth: Backing Arrays, append, and the slices and maps Packages
@@concepts lists, maps
@@text
Slices are Go's everyday collection, and most slice bugs come from not knowing what a slice actually is.

## A slice is a window onto an array

A slice value is a small header: a **pointer** to an underlying array, a **length** and a **capacity**.

```go
package main

import "fmt"

func main() {
	arr := [6]int{10, 20, 30, 40, 50, 60}
	s := arr[1:4]                     // [20 30 40], len 3, cap 5 (up to the end of arr)
	fmt.Println(s, len(s), cap(s))
	s[0] = 99                         // writes into arr!
	fmt.Println(arr)
	t := s[:cap(s)]                   // re-slice up to the capacity
	fmt.Println(t)
}
```

## append and aliasing

`append` writes into the existing array if there's capacity, otherwise it allocates a bigger one (roughly doubling) and copies. So two slices may or may not share memory after an `append`, which is a classic bug:

```go
package main

import "fmt"

func main() {
	base := make([]string, 2, 4)
	base[0], base[1] = "web01", "web02"
	a := append(base, "db01")   // fits: uses base's array
	b := append(base, "mail01") // also fits: overwrites a[2]!
	fmt.Println(a, b)

	safe := append(base[:len(base):len(base)], "db01") // full slice expression caps capacity: forces a copy
	fmt.Println(safe, base)
}
```

Rules of thumb:

- Always use the result of `append`: `s = append(s, x)`.
- Don't append to a slice you received and also keep using the original, unless you **own** it. To return an independent copy use `slices.Clone(s)`.
- Preallocate when you know the size: `make([]T, 0, n)` avoids repeated growth.

## copy, delete, insert

`copy(dst, src)` copies `min(len(dst), len(src))` elements. The `slices` package (Go 1.21+) handles the fiddly operations:

```go
package main

import (
	"fmt"
	"slices"
)

func main() {
	s := []int{5, 1, 4, 1, 3}
	slices.Sort(s)
	fmt.Println(s, slices.Contains(s, 4), slices.Index(s, 3))
	s = slices.Compact(s)        // remove consecutive duplicates
	i, found := slices.BinarySearch(s, 4)
	s = slices.Insert(s, 0, 0)
	s = slices.Delete(s, 1, 2)   // remove index 1
	fmt.Println(s, i, found, slices.Max(s))
}
```

Also: `slices.SortFunc(s, func(a, b T) int { return cmp.Compare(a.X, b.X) })`, `slices.SortStableFunc`, `slices.Equal`, `slices.IndexFunc`, `slices.ContainsFunc`, `slices.Chunk`.

## Maps and the maps package

Map iteration order is **randomized** on purpose, so never rely on it. To print a map deterministically, sort the keys:

```go
package main

import (
	"fmt"
	"maps"
	"slices"
)

func main() {
	owners := map[string]string{"web01": "ana", "db01": "kenji", "mail01": "bo"}
	for _, host := range slices.Sorted(maps.Keys(owners)) {
		fmt.Println(host, owners[host])
	}
	copyOf := maps.Clone(owners)
	delete(copyOf, "db01")
	fmt.Println(len(owners), len(copyOf))
}
```

- `v, ok := m[k]` distinguishes "missing" from "zero value".
- Maps are **not** safe for concurrent writes: use a mutex or `sync.Map`.
- A `map[string]struct{}` is the idiomatic set (the empty struct takes no memory).

## Arrays

Arrays (`[4]byte`) have a fixed size that's part of the type, and are **values** (assigning copies them). They're useful as map keys (`map[[16]byte]bool` for hashes, `[4]byte` for IPv4) and for fixed buffers. Most code uses slices.

@@quiz
`a := append(base, x)` and `b := append(base, y)` with spare capacity in base. What can happen?
* a and b share the same array, so writing y can overwrite a's x
- a and b are always independent
- The second append panics
- base grows in place
? Use slices.Clone or a full slice expression to force a copy.

@@quiz
How do you print a map in a stable order?
- Maps are always sorted by key
* Sort the keys (e.g. `slices.Sorted(maps.Keys(m))`) and iterate over them
- Use `fmt.Println(m)` and hope
- Convert to JSON
? Range over a map is randomized.

@@quiz
What's the idiomatic set type in Go?
- `[]string`
* `map[string]struct{}`
- `set[string]`
- `map[string]bool` is the only option
? map[string]bool also works, but struct{} uses no memory per value.

@@flashcards
Slice header :: pointer, length, capacity
`cap(s)` :: room before append must reallocate
`s = append(s, x)` :: always use append's result
Full slice expression :: `s[lo:hi:max]` limits capacity
`copy(dst, src)` :: copies min(len) elements
`slices.Clone` :: independent copy
`slices.Sort` / `SortFunc` :: sorting
`slices.Sorted(maps.Keys(m))` :: sorted map keys
`v, ok := m[k]` :: check presence
`map[K]struct{}` :: set

@@exercise A ring buffer of log lines
Write a generic fixed-size ring buffer that keeps the **last** N items:

- `NewRing[T any](n int) *Ring[T]`
- `(r *Ring[T]) Push(v T)`: overwrites the oldest item when full
- `(r *Ring[T]) Items() []T`: oldest to newest, as a **new** slice (callers may modify it)
- `(r *Ring[T]) Len() int`

Allocate the storage once with `make([]T, n)`; don't use `append` in `Push`.
@@starter
package main

import (
	"fmt"
	"strings"
)

type Ring[T any] struct {
	buf []T
}

func NewRing[T any](n int) *Ring[T] { return &Ring[T]{} }

func (r *Ring[T]) Push(v T) {}

func (r *Ring[T]) Items() []T { return nil }

func (r *Ring[T]) Len() int { return 0 }

func main() {
	r := NewRing[string](3)
	fmt.Println(r.Len(), r.Items())
	for _, line := range strings.Fields("boot login error retry ok") {
		r.Push(line)
		fmt.Println(r.Len(), r.Items())
	}
	items := r.Items()
	items[0] = "CHANGED"
	fmt.Println(r.Items())
	nums := NewRing[int](2)
	nums.Push(1)
	fmt.Println(nums.Items())
}
@@expected
0 []
1 [boot]
2 [boot login]
3 [boot login error]
3 [login error retry]
3 [error retry ok]
[error retry ok]
[1]
@@requires
make([]T, n)
@@hint
Keep `start` (index of the oldest item) and `count`. `Push`: if `count < len(buf)`, write at `(start+count) % len(buf)` and increment `count`; otherwise write at `start` and advance `start`. `Items`: `out := make([]T, 0, r.count)` and append `buf[(start+i)%len(buf)]`.
@@solution
package main

import (
	"fmt"
	"strings"
)

type Ring[T any] struct {
	buf   []T
	start int
	count int
}

func NewRing[T any](n int) *Ring[T] { return &Ring[T]{buf: make([]T, n)} }

func (r *Ring[T]) Push(v T) {
	if r.count < len(r.buf) {
		r.buf[(r.start+r.count)%len(r.buf)] = v
		r.count++
		return
	}
	r.buf[r.start] = v
	r.start = (r.start + 1) % len(r.buf)
}

func (r *Ring[T]) Items() []T {
	out := make([]T, 0, r.count)
	for i := 0; i < r.count; i++ {
		out = append(out, r.buf[(r.start+i)%len(r.buf)])
	}
	return out
}

func (r *Ring[T]) Len() int { return r.count }

func main() {
	r := NewRing[string](3)
	fmt.Println(r.Len(), r.Items())
	for _, line := range strings.Fields("boot login error retry ok") {
		r.Push(line)
		fmt.Println(r.Len(), r.Items())
	}
	items := r.Items()
	items[0] = "CHANGED"
	fmt.Println(r.Items())
	nums := NewRing[int](2)
	nums.Push(1)
	fmt.Println(nums.Items())
}

@@exercise Software inventory report
Each line of the input is `host,package,version`. Using the `slices` and `maps` packages, print:

1. every package with the **sorted, de-duplicated** versions in use: `openssl: 3.0.13, 3.2.1`
2. the hosts that have **more than one** version of the same package installed: `host db01 has 2 versions of openssl`
3. the packages sorted by how many hosts use them (most first, ties by name): `openssl 3 hosts`

Compare versions **numerically** by their dot-separated parts (so `1.10.0` > `1.9.2`). Write `CompareVersions(a, b string) int` and use it with `slices.SortFunc`.
@@starter
package main

import (
	"cmp"
	"fmt"
	"maps"
	"slices"
	"strconv"
	"strings"
)

func CompareVersions(a, b string) int {
	_ = strconv.Atoi
	_ = cmp.Compare[int]
	return strings.Compare(a, b)
}

const data = `web01,openssl,3.0.13
web01,nginx,1.9.2
web02,openssl,3.2.1
web02,nginx,1.10.0
db01,openssl,3.0.13
db01,openssl,3.2.1
db01,postgres,16.2
web02,nginx,1.10.0`

func main() {
	_ = maps.Clone(map[string]int{})
	_ = slices.Contains([]string{}, "")
	fmt.Println(CompareVersions("1.10.0", "1.9.2"))
	_ = data
}
@@expected
nginx: 1.9.2, 1.10.0
openssl: 3.0.13, 3.2.1
postgres: 16.2
host db01 has 2 versions of openssl
openssl 3 hosts
nginx 2 hosts
postgres 1 hosts
@@requires
slices.SortFunc
maps.Keys
@@hint
Build `versions := map[string]map[string]struct{}{}` (package → set of versions), `perHost := map[string]map[string]map[string]struct{}{}` or simply a map keyed by `host+"|"+pkg`, and `hosts := map[string]map[string]struct{}{}` (package → set of hosts). Then `vs := slices.Collect(maps.Keys(versions[pkg])); slices.SortFunc(vs, CompareVersions)`. For `CompareVersions`, split on dots, compare part by part with `cmp.Compare`, and treat missing parts as 0. Remove the placeholder lines in `main`.
@@solution
package main

import (
	"cmp"
	"fmt"
	"maps"
	"slices"
	"strconv"
	"strings"
)

func CompareVersions(a, b string) int {
	pa, pb := strings.Split(a, "."), strings.Split(b, ".")
	for i := 0; i < max(len(pa), len(pb)); i++ {
		var x, y int
		if i < len(pa) {
			x, _ = strconv.Atoi(pa[i])
		}
		if i < len(pb) {
			y, _ = strconv.Atoi(pb[i])
		}
		if c := cmp.Compare(x, y); c != 0 {
			return c
		}
	}
	return 0
}

const data = `web01,openssl,3.0.13
web01,nginx,1.9.2
web02,openssl,3.2.1
web02,nginx,1.10.0
db01,openssl,3.0.13
db01,openssl,3.2.1
db01,postgres,16.2
web02,nginx,1.10.0`

func add(m map[string]map[string]struct{}, k, v string) {
	if m[k] == nil {
		m[k] = map[string]struct{}{}
	}
	m[k][v] = struct{}{}
}

func main() {
	versions := map[string]map[string]struct{}{}
	hosts := map[string]map[string]struct{}{}
	perHost := map[string]map[string]struct{}{}
	for _, line := range strings.Split(data, "\n") {
		parts := strings.Split(line, ",")
		host, pkg, ver := parts[0], parts[1], parts[2]
		add(versions, pkg, ver)
		add(hosts, pkg, host)
		add(perHost, host+"|"+pkg, ver)
	}
	for _, pkg := range slices.Sorted(maps.Keys(versions)) {
		vs := slices.Collect(maps.Keys(versions[pkg]))
		slices.SortFunc(vs, CompareVersions)
		fmt.Printf("%s: %s\n", pkg, strings.Join(vs, ", "))
	}
	for _, key := range slices.Sorted(maps.Keys(perHost)) {
		if n := len(perHost[key]); n > 1 {
			host, pkg, _ := strings.Cut(key, "|")
			fmt.Printf("host %s has %d versions of %s\n", host, n, pkg)
		}
	}
	pkgs := slices.Collect(maps.Keys(hosts))
	slices.SortFunc(pkgs, func(a, b string) int {
		return cmp.Or(cmp.Compare(len(hosts[b]), len(hosts[a])), cmp.Compare(a, b))
	})
	for _, pkg := range pkgs {
		fmt.Printf("%s %d hosts\n", pkg, len(hosts[pkg]))
	}
}
