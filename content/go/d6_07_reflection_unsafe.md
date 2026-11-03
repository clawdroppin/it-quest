@@lesson d6-reflection Reflection, Struct Tags, unsafe and cgo
@@concepts objects
@@text
`encoding/json`, `flag`-style config loaders, ORMs and validators all work on types they've never seen. They use **reflection**: inspecting and modifying values at run time through the `reflect` package.

## Type and Value

```go
package main

import (
	"fmt"
	"reflect"
)

type Host struct {
	Name string `json:"name" env:"HOST_NAME"`
	Port int    `json:"port,omitempty"`
	note string
}

func main() {
	h := Host{Name: "web01", Port: 443}
	t := reflect.TypeOf(h)
	fmt.Println(t.Name(), t.Kind(), t.NumField())
	for i := range t.NumField() {
		f := t.Field(i)
		fmt.Printf("%-5s %-6s exported=%-5v json=%q\n", f.Name, f.Type, f.IsExported(), f.Tag.Get("json"))
	}

	v := reflect.ValueOf(&h).Elem() // addressable: we can set fields
	v.FieldByName("Port").SetInt(8443)
	fmt.Println(h.Port, v.FieldByName("note").CanSet())
}
```

- `reflect.TypeOf(x)` describes the static type; `.Kind()` is the underlying category (`Struct`, `Int`, `Slice`, `Pointer`, `Map`...). Many types share a kind: `time.Duration` is kind `Int64`.
- `reflect.ValueOf(x)` holds a value. To **modify** it you need an addressable value: pass a pointer and call `.Elem()`.
- Unexported fields can be read via reflection, but not set, and calling `.Interface()` on them panics.

## Struct tags

Tags are raw string metadata after a field: `` `json:"port,omitempty" env:"PORT"` ``. `Tag.Get(key)` returns the value; `Tag.Lookup(key)` also tells you whether the key exists. `go vet` checks the `key:"value"` syntax.

## The laws of reflection, briefly

1. From interface value to reflection object: `reflect.ValueOf`.
2. From reflection object back to interface value: `v.Interface()`.
3. To modify a reflection object, the value must be settable (addressable and exported).

## When (not) to use it

Reflection is slow (often 10–100× a direct call), loses compile-time checking and panics on misuse. Prefer **generics** or interfaces when the set of types is known; reserve reflection for genuinely generic machinery: serializers, config loaders, test helpers (`reflect.DeepEqual`).

## unsafe

```go
package main

import (
	"fmt"
	"unsafe"
)

type Padded struct {
	A bool  // 1 byte + 7 padding
	B int64 // 8
	C bool  // 1 byte + 7 padding
}

type Packed struct {
	B int64
	A bool
	C bool // 6 bytes padding at the end
}

func main() {
	fmt.Println(unsafe.Sizeof(Padded{}), unsafe.Sizeof(Packed{}), unsafe.Alignof(int64(0)))
}
```

On 64-bit machines this prints `24 16 8`: ordering fields from largest to smallest can shrink structs, which matters for millions of them in memory.

`unsafe.Pointer` lets you convert between arbitrary pointer types, bypassing the type system. The standard library uses it for zero-copy conversions (`unsafe.String`, `unsafe.Slice`), but in application code it's almost never needed and breaks Go's memory safety guarantees. Code that imports `unsafe` deserves extra review.

## cgo

cgo calls C code from Go:

```go,norun
/*
#include <math.h>
*/
import "C"

func main() { println(float64(C.sqrt(2))) }
```

It's useful for wrapping existing C libraries (SQLite, system APIs), but has costs: a C compiler in the build, no easy cross-compilation, slower calls across the boundary and C's memory bugs. `CGO_ENABLED=0` builds a fully static binary, which is why many Go projects (like pure-Go SQLite drivers) avoid cgo.

@@quiz
Why does `reflect.ValueOf(cfg).Field(0).SetInt(1)` panic?
* The value isn't addressable; pass `&cfg` and call `.Elem()`
- SetInt doesn't exist
- Fields can't be set by index
- Reflection is read-only
? You get a copy otherwise, and setting a copy would be meaningless.

@@quiz
What is the `Kind` of a `time.Duration`?
- Duration
* Int64
- Struct
- Interface
? Kind is the underlying representation; Type is `time.Duration`.

@@quiz
When should you reach for reflection?
- For all generic code
* For machinery over unknown types (serializers, config loaders) when generics or interfaces can't do it
- Whenever performance matters
- Never
? It's slower and loses compile-time checks.

@@flashcards
`reflect.TypeOf` :: describe a value's type
`reflect.ValueOf` :: wrap a value for inspection
`.Kind()` :: underlying category (Struct, Int64, Slice...)
`.Elem()` :: value a pointer points to
`CanSet()` :: addressable and exported
`Tag.Lookup(key)` :: tag value and whether it exists
`reflect.DeepEqual` :: recursive equality
`unsafe.Sizeof` :: size in bytes, including padding
`unsafe.Pointer` :: escape hatch from the type system
cgo :: call C from Go; needs a C toolchain

@@exercise An env-based config loader
Write `Load(dst any, env map[string]string) error` that fills a struct from environment variables using tags:

- `env:"NAME"` selects the variable; `default:"..."` is used when the variable is missing; fields without an `env` tag are skipped
- supported field types: `string`, `int`, `bool`, `[]string` (comma separated, trimmed) and `time.Duration` (check the **type** before the kind, since Duration's kind is Int64)
- bad values return `NAME: invalid int "abc"` (or `invalid bool`, `invalid duration`)
- `dst` must be a pointer to a struct; otherwise return `Load: need pointer to struct, got <type>` (use `%T`)
@@starter
package main

import (
	"fmt"
	"reflect"
	"strconv"
	"strings"
	"time"
)

type Config struct {
	Host    string        `env:"APP_HOST" default:"localhost"`
	Port    int           `env:"APP_PORT" default:"8080"`
	Debug   bool          `env:"APP_DEBUG"`
	Tags    []string      `env:"APP_TAGS"`
	Timeout time.Duration `env:"APP_TIMEOUT" default:"5s"`
	secret  string
}

func Load(dst any, env map[string]string) error {
	_ = reflect.TypeOf
	_ = strconv.Atoi
	_ = strings.Split
	return nil
}

func main() {
	var c Config
	err := Load(&c, map[string]string{"APP_PORT": "9090", "APP_DEBUG": "true", "APP_TAGS": "web, prod"})
	fmt.Printf("%+v %v\n", c, err)
	var bad Config
	fmt.Println("error:", Load(&bad, map[string]string{"APP_PORT": "abc"}))
	fmt.Println("error:", Load(&bad, map[string]string{"APP_TIMEOUT": "soon"}))
	fmt.Println("error:", Load(c, nil))
}
@@expected
{Host:localhost Port:9090 Debug:true Tags:[web prod] Timeout:5s secret:} <nil>
error: APP_PORT: invalid int "abc"
error: APP_TIMEOUT: invalid duration "soon"
error: Load: need pointer to struct, got main.Config
@@requires
reflect.ValueOf
Tag.Lookup
@@hint
`v := reflect.ValueOf(dst)`; check `v.Kind() != reflect.Pointer || v.Elem().Kind() != reflect.Struct`. Loop over `t.NumField()`, get `sf := t.Field(i)`, `name, ok := sf.Tag.Lookup("env")`. For a value: `raw, found := env[name]`, falling back to `sf.Tag.Lookup("default")`. In a `set(f reflect.Value, raw string) error` helper: `if f.Type() == reflect.TypeOf(time.Duration(0))` first, then `switch f.Kind()` with `SetString`, `SetInt`, `SetBool`, and for slices `f.Set(reflect.ValueOf(parts))`.
@@solution
package main

import (
	"fmt"
	"reflect"
	"strconv"
	"strings"
	"time"
)

type Config struct {
	Host    string        `env:"APP_HOST" default:"localhost"`
	Port    int           `env:"APP_PORT" default:"8080"`
	Debug   bool          `env:"APP_DEBUG"`
	Tags    []string      `env:"APP_TAGS"`
	Timeout time.Duration `env:"APP_TIMEOUT" default:"5s"`
	secret  string
}

var durationType = reflect.TypeOf(time.Duration(0))

func set(f reflect.Value, raw string) error {
	if f.Type() == durationType {
		d, err := time.ParseDuration(raw)
		if err != nil {
			return fmt.Errorf("invalid duration %q", raw)
		}
		f.SetInt(int64(d))
		return nil
	}
	switch f.Kind() {
	case reflect.String:
		f.SetString(raw)
	case reflect.Int, reflect.Int64, reflect.Int32:
		n, err := strconv.Atoi(raw)
		if err != nil {
			return fmt.Errorf("invalid int %q", raw)
		}
		f.SetInt(int64(n))
	case reflect.Bool:
		b, err := strconv.ParseBool(raw)
		if err != nil {
			return fmt.Errorf("invalid bool %q", raw)
		}
		f.SetBool(b)
	case reflect.Slice:
		if f.Type().Elem().Kind() != reflect.String {
			return fmt.Errorf("unsupported slice type %s", f.Type())
		}
		var parts []string
		for p := range strings.SplitSeq(raw, ",") {
			parts = append(parts, strings.TrimSpace(p))
		}
		f.Set(reflect.ValueOf(parts))
	default:
		return fmt.Errorf("unsupported kind %s", f.Kind())
	}
	return nil
}

func Load(dst any, env map[string]string) error {
	v := reflect.ValueOf(dst)
	if v.Kind() != reflect.Pointer || v.Elem().Kind() != reflect.Struct {
		return fmt.Errorf("Load: need pointer to struct, got %T", dst)
	}
	v = v.Elem()
	t := v.Type()
	for i := range t.NumField() {
		sf := t.Field(i)
		name, ok := sf.Tag.Lookup("env")
		if !ok || !sf.IsExported() {
			continue
		}
		raw, found := env[name]
		if !found {
			raw, found = sf.Tag.Lookup("default")
		}
		if !found {
			continue
		}
		if err := set(v.Field(i), raw); err != nil {
			return fmt.Errorf("%s: %w", name, err)
		}
	}
	return nil
}

func main() {
	var c Config
	err := Load(&c, map[string]string{"APP_PORT": "9090", "APP_DEBUG": "true", "APP_TAGS": "web, prod"})
	fmt.Printf("%+v %v\n", c, err)
	var bad Config
	fmt.Println("error:", Load(&bad, map[string]string{"APP_PORT": "abc"}))
	fmt.Println("error:", Load(&bad, map[string]string{"APP_TIMEOUT": "soon"}))
	fmt.Println("error:", Load(c, nil))
}

@@exercise A config diff for deployments
Before rolling out a change, show what differs. Write `Diff(a, b any) ([]string, error)`:

- `a` and `b` must have the same struct type, otherwise return `Diff: type mismatch <A> vs <B>` (use `%T`)
- walk the exported fields in declaration order; recurse into nested structs, joining paths with dots (`Limits.MemoryMB`)
- for other fields, compare with `reflect.DeepEqual` and record `<path>: <old> -> <new>` (formatted with `%v`)
- skip unexported fields (calling `.Interface()` on them would panic)
@@starter
package main

import (
	"fmt"
	"reflect"
)

type Limits struct {
	CPU      int
	MemoryMB int
}

type Service struct {
	Name     string
	Replicas int
	Image    string
	Ports    []int
	Limits   Limits
	token    string
}

func Diff(a, b any) ([]string, error) {
	_ = reflect.DeepEqual
	return nil, nil
}

func main() {
	old := Service{"api", 2, "api:1.4", []int{80, 443}, Limits{500, 256}, "x"}
	next := Service{"api", 3, "api:1.5", []int{80, 8443}, Limits{500, 512}, "y"}
	changes, err := Diff(old, next)
	for _, c := range changes {
		fmt.Println(c)
	}
	fmt.Println("changes:", len(changes), err)
	_, err = Diff(old, old.Limits)
	fmt.Println(err)
}
@@expected
Replicas: 2 -> 3
Image: api:1.4 -> api:1.5
Ports: [80 443] -> [80 8443]
Limits.MemoryMB: 256 -> 512
changes: 4 <nil>
Diff: type mismatch main.Service vs main.Limits
@@requires
reflect.ValueOf
NumField
@@hint
Check `reflect.TypeOf(a) != reflect.TypeOf(b)` first. A recursive helper `walk(prefix string, va, vb reflect.Value, out *[]string)`: loop over `va.NumField()`, skip `!t.Field(i).IsExported()`, build `path` (`prefix + "." + name` when prefix isn't empty), recurse when `fa.Kind() == reflect.Struct`, else compare `reflect.DeepEqual(fa.Interface(), fb.Interface())`.
@@solution
package main

import (
	"fmt"
	"reflect"
)

type Limits struct {
	CPU      int
	MemoryMB int
}

type Service struct {
	Name     string
	Replicas int
	Image    string
	Ports    []int
	Limits   Limits
	token    string
}

func walk(prefix string, va, vb reflect.Value, out *[]string) {
	t := va.Type()
	for i := range va.NumField() {
		sf := t.Field(i)
		if !sf.IsExported() {
			continue
		}
		path := sf.Name
		if prefix != "" {
			path = prefix + "." + sf.Name
		}
		fa, fb := va.Field(i), vb.Field(i)
		if fa.Kind() == reflect.Struct {
			walk(path, fa, fb, out)
			continue
		}
		if !reflect.DeepEqual(fa.Interface(), fb.Interface()) {
			*out = append(*out, fmt.Sprintf("%s: %v -> %v", path, fa.Interface(), fb.Interface()))
		}
	}
}

func Diff(a, b any) ([]string, error) {
	if reflect.TypeOf(a) != reflect.TypeOf(b) {
		return nil, fmt.Errorf("Diff: type mismatch %T vs %T", a, b)
	}
	va, vb := reflect.ValueOf(a), reflect.ValueOf(b)
	if va.Kind() != reflect.Struct {
		return nil, fmt.Errorf("Diff: need structs, got %T", a)
	}
	var out []string
	walk("", va, vb, &out)
	return out, nil
}

func main() {
	old := Service{"api", 2, "api:1.4", []int{80, 443}, Limits{500, 256}, "x"}
	next := Service{"api", 3, "api:1.5", []int{80, 8443}, Limits{500, 512}, "y"}
	changes, err := Diff(old, next)
	for _, c := range changes {
		fmt.Println(c)
	}
	fmt.Println("changes:", len(changes), err)
	_, err = Diff(old, old.Limits)
	fmt.Println(err)
}
