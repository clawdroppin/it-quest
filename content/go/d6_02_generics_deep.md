@@lesson d6-generics Generics in Depth: Type Parameters, Constraints and Type Sets
@@concepts functions, lists
@@text
Go added generics in 1.18. They're deliberately simpler than in Rust, C# or TypeScript: type parameters on functions and types, constraints written as interfaces, and no methods with their own type parameters.

## Constraints are interfaces

A constraint says what a type parameter can do. Interfaces in constraints may list **type sets**:

```go
package main

import (
	"cmp"
	"fmt"
)

type Number interface {
	~int | ~int32 | ~int64 | ~float64 // ~int means "int or any type whose underlying type is int"
}

func Sum[T Number](xs []T) T {
	var total T
	for _, x := range xs {
		total += x
	}
	return total
}

func MaxOf[T cmp.Ordered](xs ...T) T { // cmp.Ordered: anything supporting < <= > >=
	m := xs[0]
	for _, x := range xs[1:] {
		m = max(m, x)
	}
	return m
}

type Millis int64

func main() {
	fmt.Println(Sum([]float64{1.5, 2.5}), Sum([]Millis{120, 80}))
	fmt.Println(MaxOf("web01", "db01", "mail01"), MaxOf(3, 9, 4))
}
```

Without the `~`, `Sum([]Millis{...})` wouldn't compile: `Millis` is a new named type whose **underlying** type is `int64`.

Useful built-in constraints: `any`, `comparable` (supports `==`, needed for map keys), `cmp.Ordered`.

## Generic types

```go
package main

import "fmt"

type Pair[K comparable, V any] struct {
	Key K
	Val V
}

type Set[T comparable] map[T]struct{}

func (s Set[T]) Add(v T)           { s[v] = struct{}{} }
func (s Set[T]) Has(v T) bool      { _, ok := s[v]; return ok }

func main() {
	s := Set[string]{}
	s.Add("ssh")
	p := Pair[string, int]{"https", 443}
	fmt.Println(s.Has("ssh"), s.Has("ftp"), p)
}
```

Methods can use the type's parameters, but a method **can't add new type parameters** of its own. Write a function instead: `func Map[T, U any](s []T, f func(T) U) []U`.

## Inference

Go infers type arguments from function arguments: `Sum(xs)` rather than `Sum[float64](xs)`. You must write them when there's nothing to infer from, like `NewRing[string](3)` or `Set[string]{}`.

## Constraints with methods

A constraint can require methods, and can combine methods and type sets:

```go
package main

import (
	"fmt"
	"strings"
)

type Named interface {
	Name() string
}

type Host struct{ H string }

func (h Host) Name() string { return h.H }

func Names[T Named](items []T) string {
	out := make([]string, len(items))
	for i, it := range items {
		out[i] = it.Name()
	}
	return strings.Join(out, ",")
}

func main() {
	fmt.Println(Names([]Host{{"web01"}, {"db01"}}))
}
```

Why not just take `[]Named`? Because a `[]Host` isn't a `[]Named` (different memory layout), so you'd have to convert the slice. With generics you pass `[]Host` directly, and the compiler can avoid interface boxing.

## When to use generics

Use them for **containers and algorithms** that work the same for every type: sets, caches, queues, `Map/Filter/Reduce`, sorting helpers, typed wrappers like `atomic.Pointer[T]`. Don't use them when an interface method expresses the behaviour (`io.Reader`), or when there's only one type in practice. The standard library's `slices`, `maps`, `cmp`, `sync.OnceValue` and `iter` packages are great examples.

@@quiz
What does `~int` mean in a constraint?
- Approximately an int
* int or any named type whose underlying type is int
- Any integer type
- A pointer to int
? It lets `type Millis int64` style types satisfy the constraint (with ~int64).

@@quiz
Which constraint is needed to use a type parameter as a map key?
- any
* comparable
- cmp.Ordered
- ~string
? Map keys must support ==.

@@quiz
Can a method declare its own type parameters in Go?
- Yes, like functions
* No; write a top-level generic function instead
- Only on pointer receivers
- Only in interfaces
? `func Map[T, U any](...)`, not `func (s List[T]) Map[U any]`.

@@flashcards
Type parameter :: `func F[T any](x T)`
Constraint :: interface restricting a type parameter
Type set :: `int | float64` inside a constraint
`~T` :: types with underlying type T
`comparable` :: supports == and !=
`cmp.Ordered` :: supports < and >
Generic type :: `type Set[T comparable] map[T]struct{}`
Instantiation :: `Set[string]{}`
No generic methods :: use functions instead

@@exercise Functional helpers
Write generic helpers:

- `Map[T, U any](xs []T, f func(T) U) []U`
- `Filter[T any](xs []T, keep func(T) bool) []T`
- `Reduce[T, A any](xs []T, init A, f func(A, T) A) A`
- `GroupBy[T any, K comparable](xs []T, key func(T) K) map[K][]T`
- `TopN[T any, K cmp.Ordered](xs []T, n int, key func(T) K) []T`: the n items with the largest key, largest first, stable for ties; the input isn't modified

`main` uses them on login events.
@@starter
package main

import (
	"cmp"
	"fmt"
	"maps"
	"slices"
)

type Login struct {
	User    string
	Country string
	Ms      int
	OK      bool
}

// TODO: Map, Filter, Reduce, GroupBy, TopN

func main() {
	logins := []Login{
		{"ana", "JP", 120, true}, {"bo", "US", 900, false}, {"kenji", "JP", 300, true},
		{"ana", "JP", 80, true}, {"eve", "RU", 1500, false}, {"bo", "US", 200, true},
	}
	_ = logins
	_ = cmp.Compare[int]
	_ = maps.Clone(map[string]int{})
	_ = slices.Clone([]int{})
	fmt.Println("TODO")
}
@@expected
failed: [bo eve]
total ms: 3100
JP: 3 logins
RU: 1 logins
US: 2 logins
slowest: [eve bo kenji]
first user still: ana
@@hint
In `main`: `failed := Map(Filter(logins, func(l Login) bool { return !l.OK }), func(l Login) string { return l.User })`; `Reduce(logins, 0, func(acc int, l Login) int { return acc + l.Ms })`; print the groups with `slices.Sorted(maps.Keys(groups))`; `TopN(logins, 3, func(l Login) int { return l.Ms })` then map to users. `TopN`: `c := slices.Clone(xs); slices.SortStableFunc(c, func(a, b T) int { return cmp.Compare(key(b), key(a)) })`, then cut to `min(n, len(c))`.
@@solution
package main

import (
	"cmp"
	"fmt"
	"maps"
	"slices"
)

type Login struct {
	User    string
	Country string
	Ms      int
	OK      bool
}

func Map[T, U any](xs []T, f func(T) U) []U {
	out := make([]U, 0, len(xs))
	for _, x := range xs {
		out = append(out, f(x))
	}
	return out
}

func Filter[T any](xs []T, keep func(T) bool) []T {
	var out []T
	for _, x := range xs {
		if keep(x) {
			out = append(out, x)
		}
	}
	return out
}

func Reduce[T, A any](xs []T, init A, f func(A, T) A) A {
	acc := init
	for _, x := range xs {
		acc = f(acc, x)
	}
	return acc
}

func GroupBy[T any, K comparable](xs []T, key func(T) K) map[K][]T {
	out := map[K][]T{}
	for _, x := range xs {
		k := key(x)
		out[k] = append(out[k], x)
	}
	return out
}

func TopN[T any, K cmp.Ordered](xs []T, n int, key func(T) K) []T {
	c := slices.Clone(xs)
	slices.SortStableFunc(c, func(a, b T) int { return cmp.Compare(key(b), key(a)) })
	return c[:min(n, len(c))]
}

func main() {
	logins := []Login{
		{"ana", "JP", 120, true}, {"bo", "US", 900, false}, {"kenji", "JP", 300, true},
		{"ana", "JP", 80, true}, {"eve", "RU", 1500, false}, {"bo", "US", 200, true},
	}
	user := func(l Login) string { return l.User }
	fmt.Println("failed:", Map(Filter(logins, func(l Login) bool { return !l.OK }), user))
	fmt.Println("total ms:", Reduce(logins, 0, func(acc int, l Login) int { return acc + l.Ms }))
	groups := GroupBy(logins, func(l Login) string { return l.Country })
	for _, c := range slices.Sorted(maps.Keys(groups)) {
		fmt.Printf("%s: %d logins\n", c, len(groups[c]))
	}
	fmt.Println("slowest:", Map(TopN(logins, 3, func(l Login) int { return l.Ms }), user))
	fmt.Println("first user still:", logins[0].User)
}

@@exercise A generic TTL cache
Write `Cache[K comparable, V any]` with an injectable clock:

- `NewCache[K comparable, V any](ttl time.Duration, now func() time.Time) *Cache[K, V]`
- `Set(k K, v V)`, `Get(k K) (V, bool)`: expired entries count as missing (and are deleted)
- `GetOrLoad(k K, load func(K) (V, error)) (V, error)`: calls `load` on a miss and caches successful results only
- `Len() int`: entries not yet expired

`main` drives the cache with a fake clock.
@@starter
package main

import (
	"errors"
	"fmt"
	"time"
)

type entry[V any] struct {
	val     V
	expires time.Time
}

type Cache[K comparable, V any] struct {
	items map[K]entry[V]
}

func NewCache[K comparable, V any](ttl time.Duration, now func() time.Time) *Cache[K, V] {
	return &Cache[K, V]{items: map[K]entry[V]{}}
}

func main() {
	clock := time.Date(2026, 11, 3, 9, 0, 0, 0, time.UTC)
	now := func() time.Time { return clock }
	c := NewCache[string, string](time.Minute, now)
	_ = c
	_ = errors.New
	fmt.Println("TODO")
}
@@expected
web01 10.0.0.5 true
lookup db01
db01 10.0.0.6 <nil>
db01 10.0.0.6 <nil>
lookup nas01
nas01  no such host
lookup nas01
len 2
web01  false
len 0
@@hint
Store `ttl` and `now` in the struct. `Get`: `e, ok := c.items[k]; if !ok || !c.now().Before(e.expires) { delete(c.items, k); var zero V; return zero, false }`. In `main`: set web01, get it; GetOrLoad db01 twice (the loader prints `lookup <host>` and fails for `nas01` with `no such host`); GetOrLoad nas01 twice (it isn't cached); print `len`; advance `clock` by 61 seconds; get web01; print `len`.
@@solution
package main

import (
	"errors"
	"fmt"
	"time"
)

type entry[V any] struct {
	val     V
	expires time.Time
}

type Cache[K comparable, V any] struct {
	items map[K]entry[V]
	ttl   time.Duration
	now   func() time.Time
}

func NewCache[K comparable, V any](ttl time.Duration, now func() time.Time) *Cache[K, V] {
	return &Cache[K, V]{items: map[K]entry[V]{}, ttl: ttl, now: now}
}

func (c *Cache[K, V]) Set(k K, v V) {
	c.items[k] = entry[V]{v, c.now().Add(c.ttl)}
}

func (c *Cache[K, V]) Get(k K) (V, bool) {
	e, ok := c.items[k]
	if !ok || !c.now().Before(e.expires) {
		delete(c.items, k)
		var zero V
		return zero, false
	}
	return e.val, true
}

func (c *Cache[K, V]) GetOrLoad(k K, load func(K) (V, error)) (V, error) {
	if v, ok := c.Get(k); ok {
		return v, nil
	}
	v, err := load(k)
	if err != nil {
		return v, err
	}
	c.Set(k, v)
	return v, nil
}

func (c *Cache[K, V]) Len() int {
	n := 0
	for k := range c.items {
		if _, ok := c.Get(k); ok {
			n++
		}
	}
	return n
}

func main() {
	clock := time.Date(2026, 11, 3, 9, 0, 0, 0, time.UTC)
	now := func() time.Time { return clock }
	c := NewCache[string, string](time.Minute, now)
	dns := map[string]string{"db01": "10.0.0.6"}
	resolve := func(host string) (string, error) {
		fmt.Println("lookup", host)
		if ip, ok := dns[host]; ok {
			return ip, nil
		}
		return "", errors.New("no such host")
	}
	c.Set("web01", "10.0.0.5")
	ip, ok := c.Get("web01")
	fmt.Println("web01", ip, ok)
	for range 2 {
		ip, err := c.GetOrLoad("db01", resolve)
		fmt.Println("db01", ip, err)
	}
	ip, err := c.GetOrLoad("nas01", resolve)
	fmt.Println("nas01", ip, err)
	_, _ = c.GetOrLoad("nas01", resolve)
	fmt.Println("len", c.Len())
	clock = clock.Add(61 * time.Second)
	ip, ok = c.Get("web01")
	fmt.Println("web01", ip, ok)
	fmt.Println("len", c.Len())
}
