@@lesson g2-slices-maps Slices and Maps
@@concepts lists, maps
@@text
## Slices

An **array** has a fixed length that's part of its type (`[3]int`). You'll almost always use a **slice** instead: a growable view of an array.

```go
package main

import "fmt"

func main() {
	hosts := []string{"web01", "web02"}
	hosts = append(hosts, "db01")        // append returns the new slice: always assign it back
	fmt.Println(hosts, len(hosts))
	fmt.Println(hosts[0], hosts[len(hosts)-1])
	fmt.Println(hosts[1:3])              // a sub-slice: items 1 and 2

	for i, h := range hosts {
		fmt.Println(i, h)
	}

	ports := make([]int, 0, 10)          // length 0, room for 10 before it must grow
	for p := 8000; p < 8003; p++ {
		ports = append(ports, p)
	}
	fmt.Println(ports)
}
```

Reading `hosts[5]` on a 3-item slice **panics** (index out of range): Go checks every index.

A sub-slice **shares** memory with the original. Changing `sub[0]` changes the original too. Use `slices.Clone(s)` when you need an independent copy.

## The slices and sort packages

```go
package main

import (
	"fmt"
	"slices"
	"sort"
	"strings"
)

type Server struct {
	Name string
	CPU  int
}

func main() {
	ports := []int{443, 22, 8080, 80}
	slices.Sort(ports)
	fmt.Println(ports, slices.Contains(ports, 22), slices.Index(ports, 8080), slices.Max(ports))

	servers := []Server{{"web01", 35}, {"web02", 92}, {"db01", 71}}
	sort.Slice(servers, func(i, j int) bool { return servers[i].CPU > servers[j].CPU })
	fmt.Println(servers[0].Name)

	slices.SortFunc(servers, func(a, b Server) int { return strings.Compare(a.Name, b.Name) })
	fmt.Println(servers)
}
```

## Maps

```go
package main

import (
	"fmt"
	"maps"
	"slices"
)

func main() {
	owners := map[string]string{"web01": "aiko", "db01": "kenji"}
	owners["mail01"] = "yui"
	delete(owners, "web01")
	fmt.Println(len(owners), owners["db01"])

	who, ok := owners["dns01"] // the "comma ok" form: ok is false if the key is missing
	fmt.Printf("%q %t\n", who, ok)

	counts := map[string]int{}
	for _, level := range []string{"INFO", "ERROR", "INFO", "WARN", "INFO"} {
		counts[level]++ // a missing key reads as the zero value, 0
	}
	fmt.Println(counts["INFO"], counts["DEBUG"])

	// Map iteration order is random on purpose. Sort the keys for stable output.
	keys := slices.Sorted(maps.Keys(counts))
	for _, k := range keys {
		fmt.Printf("%s=%d\n", k, counts[k])
	}
}
```

- Reading a missing key gives the zero value, not an error. Use `v, ok := m[k]` to tell "missing" from "zero".
- **Map order is random.** Never rely on it; sort the keys.
- A `nil` map (declared with `var m map[string]int`) can be read but **panics on write**. Create maps with a literal or `make(map[string]int)`.
- A set is a `map[string]bool` or `map[string]struct{}`.

@@quiz
What's wrong with `append(hosts, "db01")` on its own line?
* The result is thrown away: append returns the new slice, which must be assigned back
- Nothing
- append only works on arrays
- It panics
? Write `hosts = append(hosts, "db01")`. The compiler reports the unused result.

@@quiz
`counts := map[string]int{}; fmt.Println(counts["x"])` prints...
* `0`
- an error
- `nil`
- it panics
? Missing keys read as the zero value. Use `v, ok := counts["x"]` to check presence.

@@quiz
You print a map with a `for range` loop twice. Is the order the same?
* Not necessarily: Go randomizes map iteration order
- Yes, insertion order
- Yes, sorted by key
- Only for string keys
? Sort the keys when the order matters (reports, tests).

@@drill
Which built-in adds elements to a slice?
= append
? s = append(s, x); always use the result.

@@flashcards
`[]string{"a"}` :: a slice literal
`s = append(s, x)` :: add to a slice (assign back!)
`s[1:3]` :: a sub-slice (shares memory)
`make([]int, 0, 10)` :: empty slice with capacity 10
`v, ok := m[k]` :: read and check presence
`delete(m, k)` :: remove a key
`slices.Sorted(maps.Keys(m))` :: map keys, sorted
`map[string]struct{}` :: a set

@@exercise Top talkers
Add up the bytes per source IP, then print the top 3 IPs by total, biggest first, as `ip bytes`.
@@starter
package main

import (
	"fmt"
	"sort"
)

type Flow struct {
	Src   string
	Bytes int
}

func main() {
	flows := []Flow{
		{"10.0.0.5", 1200}, {"10.0.0.9", 300}, {"10.0.0.5", 800}, {"10.0.0.7", 5000},
		{"10.0.0.9", 2500}, {"10.0.0.2", 100}, {"10.0.0.7", 1},
	}
	totals := map[string]int{}
	_ = sort.Slice // remove this line when you use sort
	fmt.Println(flows, totals) // TODO: total, sort, print the top 3
}
@@expected
10.0.0.7 5001
10.0.0.9 2800
10.0.0.5 2000
@@hint
Fill `totals[f.Src] += f.Bytes` in a loop. Collect the keys into a slice, sort it with `sort.Slice(ips, func(i, j int) bool { return totals[ips[i]] > totals[ips[j]] })`, and print the first three.
@@solution
package main

import (
	"fmt"
	"sort"
)

type Flow struct {
	Src   string
	Bytes int
}

func main() {
	flows := []Flow{
		{"10.0.0.5", 1200}, {"10.0.0.9", 300}, {"10.0.0.5", 800}, {"10.0.0.7", 5000},
		{"10.0.0.9", 2500}, {"10.0.0.2", 100}, {"10.0.0.7", 1},
	}
	totals := map[string]int{}
	for _, f := range flows {
		totals[f.Src] += f.Bytes
	}
	ips := make([]string, 0, len(totals))
	for ip := range totals {
		ips = append(ips, ip)
	}
	sort.Slice(ips, func(i, j int) bool { return totals[ips[i]] > totals[ips[j]] })
	for _, ip := range ips[:min(3, len(ips))] {
		fmt.Println(ip, totals[ip])
	}
}

@@exercise Group users by department
Build a `map[string][]string` from department to user names, then print each department in alphabetical order with its users sorted and comma-separated: `it: abe, suzuki`.
@@starter
package main

import (
	"fmt"
	"slices"
	"strings"
)

func main() {
	users := [][2]string{{"tanaka", "sales"}, {"suzuki", "it"}, {"abe", "it"}, {"mori", "hr"}, {"ito", "sales"}}
	byDept := map[string][]string{}
	names := []string{"(you'll need these)"}
	slices.Sort(names)
	fmt.Println(strings.Join(names, ", "), users, byDept) // TODO: replace all of this
}
@@expected
hr: mori
it: abe, suzuki
sales: ito, tanaka
@@hint
`byDept[u[1]] = append(byDept[u[1]], u[0])` works even for a new key: appending to a nil slice is fine. Then get the sorted keys, and for each `slices.Sort(byDept[d])` and `strings.Join(..., ", ")`.
@@solution
package main

import (
	"fmt"
	"slices"
	"strings"
)

func main() {
	users := [][2]string{{"tanaka", "sales"}, {"suzuki", "it"}, {"abe", "it"}, {"mori", "hr"}, {"ito", "sales"}}
	byDept := map[string][]string{}
	for _, u := range users {
		byDept[u[1]] = append(byDept[u[1]], u[0])
	}
	depts := make([]string, 0, len(byDept))
	for d := range byDept {
		depts = append(depts, d)
	}
	slices.Sort(depts)
	for _, d := range depts {
		names := byDept[d]
		slices.Sort(names)
		fmt.Printf("%s: %s\n", d, strings.Join(names, ", "))
	}
}
