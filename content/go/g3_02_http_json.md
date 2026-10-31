@@lesson g3-http-json HTTP, JSON and Files
@@concepts networking, json, files
@@text
## JSON with struct tags

`encoding/json` maps JSON to structs. **Struct tags** give the JSON names:

```go
package main

import (
	"encoding/json"
	"fmt"
)

type Server struct {
	Name    string   `json:"name"`
	IP      string   `json:"ip"`
	Port    int      `json:"port,omitempty"` // left out of the JSON when 0
	Tags    []string `json:"tags"`
	secret  string   // lower-case fields are never encoded
}

func main() {
	s := Server{Name: "web01", IP: "10.0.0.11", Tags: []string{"prod"}, secret: "x"}
	b, _ := json.Marshal(s)
	fmt.Println(string(b))

	var back []Server
	err := json.Unmarshal([]byte(`[{"name":"db01","ip":"10.0.0.21","port":5432,"tags":[]}]`), &back)
	fmt.Println(back[0].Name, back[0].Port, err)

	pretty, _ := json.MarshalIndent(map[string]int{"up": 3, "down": 1}, "", "  ")
	fmt.Println(string(pretty))
}
```

Only **exported** (capitalized) fields are encoded. Unmarshal ignores JSON fields that your struct doesn't have, and leaves missing ones at their zero value. For JSON of unknown shape, decode into `map[string]any`.

## Files

```go
package main

import (
	"bufio"
	"fmt"
	"os"
	"path/filepath"
	"strings"
)

func main() {
	path := filepath.Join(os.TempDir(), "itquest_demo.log") // the right separator for the OS
	err := os.WriteFile(path, []byte("INFO boot\nERROR disk full\nINFO login\n"), 0o644)
	if err != nil {
		fmt.Println(err)
		return
	}
	defer os.Remove(path)

	f, err := os.Open(path)
	if err != nil {
		fmt.Println(err)
		return
	}
	defer f.Close()

	errors := 0
	scanner := bufio.NewScanner(f) // line by line: works for huge files
	for scanner.Scan() {
		if strings.HasPrefix(scanner.Text(), "ERROR") {
			errors++
		}
	}
	fmt.Println(errors, "error line(s)")
}
```

`os.ReadFile` reads a whole small file; `bufio.Scanner` reads line by line. `0o644` is the Unix permission (rw-r--r--).

## An HTTP client

```go,norun
client := &http.Client{Timeout: 5 * time.Second} // always set a timeout
resp, err := client.Get("https://status.example.jp/api/health")
if err != nil {
	return err
}
defer resp.Body.Close() // always close the body
if resp.StatusCode != http.StatusOK {
	return fmt.Errorf("health check: HTTP %d", resp.StatusCode)
}
var health struct {
	Status  string `json:"status"`
	Version string `json:"version"`
}
if err := json.NewDecoder(resp.Body).Decode(&health); err != nil {
	return err
}
```

The default `http.Get` has **no timeout**: a hung server hangs your program forever. Create a client with one.

## An HTTP server

The standard library has a production-quality web server. This is a complete health-check API:

```go,norun
package main

import (
	"encoding/json"
	"log"
	"net/http"
	"time"
)

func main() {
	started := time.Now()
	mux := http.NewServeMux()
	mux.HandleFunc("GET /health", func(w http.ResponseWriter, r *http.Request) {
		w.Header().Set("Content-Type", "application/json")
		json.NewEncoder(w).Encode(map[string]any{
			"status": "ok",
			"uptime": time.Since(started).String(),
		})
	})
	mux.HandleFunc("GET /hosts/{name}", func(w http.ResponseWriter, r *http.Request) {
		name := r.PathValue("name") // Go 1.22+ route patterns
		if name != "web01" {
			http.Error(w, "unknown host", http.StatusNotFound)
			return
		}
		w.Write([]byte(`{"name":"web01","ip":"10.0.0.11"}`))
	})
	log.Println("listening on :8080")
	log.Fatal(http.ListenAndServe(":8080", mux))
}
```

Every request runs in its own goroutine, so the server handles many clients at once with no extra code. Prometheus exporters, webhooks, and small internal APIs look exactly like this.

@@quiz
Why is `secret` missing from the JSON output of `json.Marshal`?
* Lower-case (unexported) fields are never encoded
- Because it's empty
- Because of a missing tag
- JSON can't hold strings called secret
? encoding/json only sees exported fields.

@@quiz
What's the danger of using `http.Get` directly in a monitoring tool?
* The default client has no timeout, so a hung server can block forever
- It doesn't support HTTPS
- It can't read JSON
- It's deprecated
? Create `&http.Client{Timeout: ...}`, or pass a context with a deadline.

@@quiz
Why `defer resp.Body.Close()`?
* So the connection can be reused and doesn't leak
- To delete the response
- It's optional style
- To print the body
? Unclosed bodies leak connections and file descriptors.

@@flashcards
`json:"name,omitempty"` :: JSON name, left out when zero
`json.Marshal(v)` :: Go value to JSON bytes
`json.Unmarshal(b, &v)` :: JSON bytes into v
`bufio.NewScanner(f)` :: read a file line by line
`filepath.Join(a, b)` :: build a path for the OS
`&http.Client{Timeout: 5 * time.Second}` :: an HTTP client with a timeout
`mux.HandleFunc("GET /x", h)` :: register a route
`r.PathValue("name")` :: a value from the route pattern

@@exercise Users from JSON
Unmarshal the JSON into a slice of `User`, then print the disabled users' names in alphabetical order and `enabled: <n>`.
@@starter
package main

import (
	"encoding/json"
	"fmt"
	"sort"
)

type User struct {
	// TODO: fields with json tags: name, dept, enabled
}

func main() {
	data := []byte(`[
		{"name": "tanaka", "dept": "sales", "enabled": true},
		{"name": "suzuki", "dept": "it", "enabled": false},
		{"name": "abe", "dept": "it", "enabled": false},
		{"name": "mori", "dept": "hr", "enabled": true}
	]`)
	var users []User
	err := json.Unmarshal(data, &users)
	fmt.Println(len(users), err, sort.StringsAreSorted(nil)) // TODO
}
@@expected
abe
suzuki
enabled: 2
@@requires
json:"enabled"
@@hint
Give each field a tag:

```go,norun
type User struct {
	Name    string `json:"name"`
	Dept    string `json:"dept"`
	Enabled bool   `json:"enabled"`
}
```

Collect the disabled names in a `[]string`, `sort.Strings` them, and count the enabled ones.
@@solution
package main

import (
	"encoding/json"
	"fmt"
	"sort"
)

type User struct {
	Name    string `json:"name"`
	Dept    string `json:"dept"`
	Enabled bool   `json:"enabled"`
}

func main() {
	data := []byte(`[
		{"name": "tanaka", "dept": "sales", "enabled": true},
		{"name": "suzuki", "dept": "it", "enabled": false},
		{"name": "abe", "dept": "it", "enabled": false},
		{"name": "mori", "dept": "hr", "enabled": true}
	]`)
	var users []User
	if err := json.Unmarshal(data, &users); err != nil {
		fmt.Println(err)
		return
	}
	var disabled []string
	enabled := 0
	for _, u := range users {
		if u.Enabled {
			enabled++
		} else {
			disabled = append(disabled, u.Name)
		}
	}
	sort.Strings(disabled)
	for _, n := range disabled {
		fmt.Println(n)
	}
	fmt.Printf("enabled: %d\n", enabled)
}

@@exercise Health response
Build the JSON a health endpoint returns: marshal a `Health` struct so the output is exactly the line below. `Checks` must be a map, and the `Error` field must be left out when empty (`omitempty`). `json.Marshal` sorts map keys, so the output is stable.

```text
{"status":"degraded","checks":{"db":"ok","disk":"fail"},"version":"1.4.2"}
```
@@starter
package main

import (
	"encoding/json"
	"fmt"
)

type Health struct {
	Status string
	// TODO: Checks, Version and Error, with json tags
}

func main() {
	h := Health{Status: "degraded"}
	b, err := json.Marshal(h)
	fmt.Println(string(b), err)
}
@@expected
{"status":"degraded","checks":{"db":"ok","disk":"fail"},"version":"1.4.2"}
@@requires
omitempty
@@hint
The fields are encoded in the order they're declared:

```go,norun
Status  string            `json:"status"`
Checks  map[string]string `json:"checks"`
Version string            `json:"version"`
Error   string            `json:"error,omitempty"`
```

Fill them in when you create `h`, and print only `string(b)`.
@@solution
package main

import (
	"encoding/json"
	"fmt"
)

type Health struct {
	Status  string            `json:"status"`
	Checks  map[string]string `json:"checks"`
	Version string            `json:"version"`
	Error   string            `json:"error,omitempty"`
}

func main() {
	h := Health{Status: "degraded", Checks: map[string]string{"disk": "fail", "db": "ok"}, Version: "1.4.2"}
	b, err := json.Marshal(h)
	if err != nil {
		fmt.Println(err)
		return
	}
	fmt.Println(string(b))
}
