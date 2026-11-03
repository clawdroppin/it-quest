@@lesson f8-project Project: A Server Status Dashboard App
@@concepts objects, json
@@text
The capstone: a desktop dashboard that pings your servers, shows each one's status with a latency sparkline, raises alerts when something changes, and remembers its settings between runs.

## Architecture

```text
probe goroutines ──samples──► Dashboard (view-model) ──rows/alerts──► fyne.Do ──► widgets
                                    ▲
Settings (JSON, versioned) ─────────┘   app.Preferences / a config file
```

- **Probes** run on a `time.Ticker` in goroutines with a context, and send `Sample{Host, Latency, OK}` values on a channel.
- The **Dashboard** view-model keeps a sliding window of samples per host, computes the status, sparkline and alerts. Pure Go, fully tested.
- The **view** is a `widget.Table` or `List` refreshed through `fyne.Do`, plus `a.SendNotification(...)` for alerts.

```go,norun
func (v *View) consume(ctx context.Context, samples <-chan Sample) {
	for {
		select {
		case <-ctx.Done():
			return
		case s := <-samples:
			if alert := v.dash.Add(s); alert != "" {
				v.app.SendNotification(fyne.NewNotification("Status change", alert))
			}
			fyne.Do(func() { v.table.Refresh() })
		}
	}
}
```

(Here `v.dash` must only be touched by this goroutine, or be protected by a mutex, since the table's callbacks read it on the UI thread.)

## Settings that survive upgrades

Fyne's `a.Preferences()` stores simple key/values per app ID. For structured settings, a JSON file in `os.UserConfigDir()` works everywhere. Real apps change their settings format over time, so store a **version** and migrate old files when loading instead of crashing or silently resetting the user's configuration.

## Status rules

Good dashboards avoid flapping. A single failed ping shouldn't page anyone:

- **DOWN** only after 2 consecutive failures
- **SLOW** when the average of the last 3 successful latencies is ≥ 200 ms
- **UP** otherwise

## Sparklines

A sparkline squeezes a time series into a word: `▁▂▄▇█▅▃`. Scale each value between the window's min and max into 8 levels using integer maths.

@@quiz
Why require two consecutive failures before showing DOWN?
* To avoid flapping alerts from a single lost packet
- Pings always fail once
- It's a Fyne requirement
- To save memory
? Hysteresis makes alerts trustworthy.

@@quiz
Why store a version number in the settings file?
- For display only
* So newer app versions can detect and migrate old formats
- JSON requires it
- To encrypt it
? Breaking users' configs on upgrade is a classic desktop app bug.

@@flashcards
Sliding window :: keep only the last N samples
Hysteresis :: require persistence before changing state
Sparkline :: tiny inline chart
`a.SendNotification` :: desktop notification in Fyne
`a.Preferences()` :: per-app key/value storage
`os.UserConfigDir()` :: where config files belong
Settings migration :: upgrade old formats on load

@@exercise The dashboard view-model
Implement `Dashboard`:

- `Add(s Sample) string`: append to the host's window (keep the last 8); recompute the status (rules in the lesson); if the host already had a status and it changed, return an alert `<host>: <OLD> -> <NEW>`, else `""`
- `Sparkline(host)`: one rune per sample: failures are `x`; successful latencies scale to `▁▂▃▄▅▆▇█` as `(v-min)*7/(max-min)` over the successful latencies in the window (all `▄` if max == min)
- `Rows()`: hosts sorted by severity (DOWN, SLOW, UP) then name, formatted `"%-6s %-4s last=%-6s %s"` with last as `<n>ms` or `fail` and the sparkline
- `Summary()`: `3 hosts: 1 down, 1 slow, 1 up`

`main` feeds 8 rounds of samples (provided) and prints alerts with their round number.
@@starter
package main

import (
	"fmt"
	"slices"
	"strings"
)

type Sample struct {
	Host      string
	LatencyMs int
	OK        bool
}

type Dashboard struct {
	window int
	hist   map[string][]Sample
	status map[string]string
}

func NewDashboard() *Dashboard {
	return &Dashboard{window: 8, hist: map[string][]Sample{}, status: map[string]string{}}
}

func main() {
	_ = slices.SortFunc[[]string]
	_ = strings.Builder{}
	web := []int{20, 25, 30, 22, 28, 35, 30, 26}
	db := []int{40, 60, 150, 220, 260, 300, 280, 310}
	api := []int{50, 55, -1, 60, 65, -1, -1, -1} // -1 = failed ping
	d := NewDashboard()
	_, _, _, _ = web, db, api, d
	fmt.Println("TODO")
}
@@expected
round 5: db01: UP -> SLOW
round 7: api01: UP -> DOWN
api01  DOWN last=fail   ▁▃x▅█xxx
db01   SLOW last=310ms  ▁▁▃▅▆▇▇█
web01  UP   last=26ms   ▁▃▅▁▄█▅▃
3 hosts: 1 down, 1 slow, 1 up
@@hint
Each round `i` adds `web01`, `db01`, `api01` samples in that order (a latency of -1 means `Sample{Host, 0, false}`). Status: look at the last two samples for DOWN; collect the last 3 OK latencies (walking backwards) for SLOW. Severity order: `map[string]int{"DOWN": 0, "SLOW": 1, "UP": 2}`. Build the sparkline with `[]rune("▁▂▃▄▅▆▇█")`.
@@solution
package main

import (
	"cmp"
	"fmt"
	"maps"
	"slices"
	"strings"
)

type Sample struct {
	Host      string
	LatencyMs int
	OK        bool
}

type Dashboard struct {
	window int
	hist   map[string][]Sample
	status map[string]string
}

func NewDashboard() *Dashboard {
	return &Dashboard{window: 8, hist: map[string][]Sample{}, status: map[string]string{}}
}

func compute(h []Sample) string {
	n := len(h)
	if n >= 2 && !h[n-1].OK && !h[n-2].OK {
		return "DOWN"
	}
	var lat []int
	for i := n - 1; i >= 0 && len(lat) < 3; i-- {
		if h[i].OK {
			lat = append(lat, h[i].LatencyMs)
		}
	}
	if len(lat) > 0 {
		sum := 0
		for _, l := range lat {
			sum += l
		}
		if sum/len(lat) >= 200 {
			return "SLOW"
		}
	}
	return "UP"
}

func (d *Dashboard) Add(s Sample) string {
	h := append(d.hist[s.Host], s)
	if len(h) > d.window {
		h = h[len(h)-d.window:]
	}
	d.hist[s.Host] = h
	old, had := d.status[s.Host]
	now := compute(h)
	d.status[s.Host] = now
	if had && old != now {
		return fmt.Sprintf("%s: %s -> %s", s.Host, old, now)
	}
	return ""
}

var levels = []rune("▁▂▃▄▅▆▇█")

func (d *Dashboard) Sparkline(host string) string {
	h := d.hist[host]
	lo, hi, seen := 0, 0, false
	for _, s := range h {
		if !s.OK {
			continue
		}
		if !seen {
			lo, hi, seen = s.LatencyMs, s.LatencyMs, true
		}
		lo, hi = min(lo, s.LatencyMs), max(hi, s.LatencyMs)
	}
	var b strings.Builder
	for _, s := range h {
		switch {
		case !s.OK:
			b.WriteRune('x')
		case hi == lo:
			b.WriteRune(levels[3])
		default:
			b.WriteRune(levels[(s.LatencyMs-lo)*7/(hi-lo)])
		}
	}
	return b.String()
}

var severity = map[string]int{"DOWN": 0, "SLOW": 1, "UP": 2}

func (d *Dashboard) Rows() []string {
	hosts := slices.Collect(maps.Keys(d.hist))
	slices.SortFunc(hosts, func(a, b string) int {
		if c := cmp.Compare(severity[d.status[a]], severity[d.status[b]]); c != 0 {
			return c
		}
		return strings.Compare(a, b)
	})
	var out []string
	for _, h := range hosts {
		s := d.hist[h][len(d.hist[h])-1]
		last := "fail"
		if s.OK {
			last = fmt.Sprintf("%dms", s.LatencyMs)
		}
		out = append(out, fmt.Sprintf("%-6s %-4s last=%-6s %s", h, d.status[h], last, d.Sparkline(h)))
	}
	return out
}

func (d *Dashboard) Summary() string {
	counts := map[string]int{}
	for _, s := range d.status {
		counts[s]++
	}
	return fmt.Sprintf("%d hosts: %d down, %d slow, %d up", len(d.status), counts["DOWN"], counts["SLOW"], counts["UP"])
}

func main() {
	web := []int{20, 25, 30, 22, 28, 35, 30, 26}
	db := []int{40, 60, 150, 220, 260, 300, 280, 310}
	api := []int{50, 55, -1, 60, 65, -1, -1, -1} // -1 = failed ping
	d := NewDashboard()
	sample := func(host string, v int) Sample {
		if v < 0 {
			return Sample{host, 0, false}
		}
		return Sample{host, v, true}
	}
	for i := range web {
		for _, s := range []Sample{sample("web01", web[i]), sample("db01", db[i]), sample("api01", api[i])} {
			if alert := d.Add(s); alert != "" {
				fmt.Printf("round %d: %s\n", i+1, alert)
			}
		}
	}
	for _, r := range d.Rows() {
		fmt.Println(r)
	}
	fmt.Println(d.Summary())
}

@@exercise Versioned settings with migration
Write `Load(raw []byte) (Settings, []string, error)` for the dashboard's settings file. The current format is version 2:

```json
{"version":2,"refresh_ms":5000,"theme":"system","hosts":["web01"]}
```

- empty input → defaults (version 2, 5000 ms, `system`, no hosts) and no warnings
- version 0 or 1 files used `"refresh"` in **seconds**: convert to `refresh_ms` and warn `migrated settings from v1`
- `refresh_ms` below 1000 → 1000 with warning `refresh_ms 200 too low, using 1000`
- a theme other than `system`, `light`, `dark` → `system` with warning `unknown theme "neon", using "system"`
- a version newer than 2 → error `settings version 3 is newer than this app (2)`
- invalid JSON → error `settings: <decoder error>`
- `Hosts` is never nil (so it saves as `[]`)

`main` prints the resulting JSON (`json.Marshal`) and warnings for several inputs.
@@starter
package main

import (
	"encoding/json"
	"fmt"
	"slices"
)

type Settings struct {
	Version   int      `json:"version"`
	RefreshMs int      `json:"refresh_ms"`
	Theme     string   `json:"theme"`
	Hosts     []string `json:"hosts"`
}

func Load(raw []byte) (Settings, []string, error) {
	_ = slices.Contains[[]string]
	return Settings{}, nil, nil
}

func main() {
	inputs := []string{
		``,
		`{"refresh": 10, "hosts": ["web01", "db01"]}`,
		`{"version":2,"refresh_ms":200,"theme":"neon"}`,
		`{"version":3}`,
		`{"version":`,
	}
	for _, in := range inputs {
		s, warnings, err := Load([]byte(in))
		if err != nil {
			fmt.Println("error:", err)
			continue
		}
		out, _ := json.Marshal(s)
		fmt.Println(string(out))
		for _, w := range warnings {
			fmt.Println("  warning:", w)
		}
	}
}
@@expected
{"version":2,"refresh_ms":5000,"theme":"system","hosts":[]}
{"version":2,"refresh_ms":10000,"theme":"system","hosts":["web01","db01"]}
  warning: migrated settings from v1
{"version":2,"refresh_ms":1000,"theme":"system","hosts":[]}
  warning: refresh_ms 200 too low, using 1000
  warning: unknown theme "neon", using "system"
error: settings version 3 is newer than this app (2)
error: settings: unexpected end of JSON input
@@hint
Decode into a "file" struct that has every field any version used, with pointers where you need to know whether a field was present: `Refresh *int \`json:"refresh"\``, `RefreshMs *int`, `Theme *string`. Start from the defaults and overwrite with whatever is present.
@@solution
package main

import (
	"encoding/json"
	"fmt"
	"slices"
)

type Settings struct {
	Version   int      `json:"version"`
	RefreshMs int      `json:"refresh_ms"`
	Theme     string   `json:"theme"`
	Hosts     []string `json:"hosts"`
}

const currentVersion = 2

type fileFormat struct {
	Version   int      `json:"version"`
	Refresh   *int     `json:"refresh"`
	RefreshMs *int     `json:"refresh_ms"`
	Theme     *string  `json:"theme"`
	Hosts     []string `json:"hosts"`
}

func Load(raw []byte) (Settings, []string, error) {
	s := Settings{Version: currentVersion, RefreshMs: 5000, Theme: "system", Hosts: []string{}}
	if len(raw) == 0 {
		return s, nil, nil
	}
	var f fileFormat
	if err := json.Unmarshal(raw, &f); err != nil {
		return s, nil, fmt.Errorf("settings: %w", err)
	}
	if f.Version > currentVersion {
		return s, nil, fmt.Errorf("settings version %d is newer than this app (%d)", f.Version, currentVersion)
	}
	var warnings []string
	if f.Version < 2 && f.Refresh != nil {
		s.RefreshMs = *f.Refresh * 1000
		warnings = append(warnings, "migrated settings from v1")
	}
	if f.RefreshMs != nil {
		s.RefreshMs = *f.RefreshMs
	}
	if s.RefreshMs < 1000 {
		warnings = append(warnings, fmt.Sprintf("refresh_ms %d too low, using 1000", s.RefreshMs))
		s.RefreshMs = 1000
	}
	if f.Theme != nil {
		if slices.Contains([]string{"system", "light", "dark"}, *f.Theme) {
			s.Theme = *f.Theme
		} else {
			warnings = append(warnings, fmt.Sprintf("unknown theme %q, using %q", *f.Theme, "system"))
		}
	}
	if f.Hosts != nil {
		s.Hosts = f.Hosts
	}
	return s, warnings, nil
}

func main() {
	inputs := []string{
		``,
		`{"refresh": 10, "hosts": ["web01", "db01"]}`,
		`{"version":2,"refresh_ms":200,"theme":"neon"}`,
		`{"version":3}`,
		`{"version":`,
	}
	for _, in := range inputs {
		s, warnings, err := Load([]byte(in))
		if err != nil {
			fmt.Println("error:", err)
			continue
		}
		out, _ := json.Marshal(s)
		fmt.Println(string(out))
		for _, w := range warnings {
			fmt.Println("  warning:", w)
		}
	}
}
