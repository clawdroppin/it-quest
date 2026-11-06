@@lesson p7-cli Command-Line Tools: flag, Subcommands, Exit Codes and Config
@@concepts automation
@@text
Go is a favourite for CLI tools (`kubectl`, `terraform`, `gh`, `hugo`): one static binary, fast start-up, easy cross-compilation.

## The flag package

```go,norun
port := flag.Int("port", 8080, "port to listen on")
verbose := flag.Bool("v", false, "verbose output")
flag.Parse()
args := flag.Args() // positional arguments after the flags
```

Flags go **before** positional arguments (`tool -v file.txt`); `flag` stops at the first non-flag. `-h` prints generated usage.

## Testable CLIs: run(args, stdout, stderr) int

Don't put logic in `main`. Put it in a function that takes arguments and writers and returns an exit code, so tests can call it directly:

```go,norun
func main() {
	os.Exit(run(os.Args[1:], os.Stdout, os.Stderr))
}
```

Note that `os.Exit` skips deferred functions, which is another reason to keep `main` tiny.

## Subcommands

Each subcommand gets its own `flag.FlagSet`. With `flag.ContinueOnError` parsing returns an error instead of exiting, which is what you want inside `run`:

```go,norun
fs := flag.NewFlagSet("ping", flag.ContinueOnError)
fs.SetOutput(stderr)
count := fs.Int("count", 4, "number of pings")
if err := fs.Parse(args); err != nil {
	return 2
}
```

Exit code conventions: `0` success, `1` failure, `2` usage error.

## Configuration precedence

Real tools merge settings from several places. The usual order, lowest to highest: **defaults → config file → environment variables → flags**. `fs.Visit` iterates only over flags the user actually set, which tells you whether a flag value came from the user or is a default.

## Signals and cancellation

```go,norun
ctx, stop := signal.NotifyContext(context.Background(), os.Interrupt, syscall.SIGTERM)
defer stop()
err := doWork(ctx) // Ctrl+C cancels ctx; clean up and exit
```

## Bigger frameworks

`github.com/spf13/cobra` (used by kubectl, gh and hugo) adds nested commands, shell completion and generated docs, and pairs with `viper` for config. `urfave/cli` and `alecthomas/kong` are popular alternatives:

```go,norun
var rootCmd = &cobra.Command{Use: "netkit", Short: "Network helpers"}
var pingCmd = &cobra.Command{
	Use:  "ping HOST",
	Args: cobra.ExactArgs(1),
	RunE: func(cmd *cobra.Command, args []string) error { return ping(cmd.Context(), args[0]) },
}

func init() {
	pingCmd.Flags().IntP("count", "c", 4, "number of pings")
	rootCmd.AddCommand(pingCmd)
}
```

## Building and shipping

```text
go build -ldflags "-s -w -X main.version=1.4.0" -o netkit .
GOOS=linux GOARCH=arm64 go build .     # cross-compile (PowerShell: $env:GOOS="linux")
go install example.com/netkit@latest   # users install from source
```

Tools like GoReleaser build archives for every platform, plus checksums and Homebrew/Scoop manifests, from a git tag.

@@quiz
Why use `flag.ContinueOnError` in a `run` function?
* Parse errors are returned instead of the program exiting, so run can return an exit code and stay testable
- It ignores bad flags
- It's faster
- It enables subcommands
? ExitOnError calls os.Exit(2) itself.

@@quiz
What's a common precedence order for configuration (lowest first)?
- flags, env, file, defaults
* defaults, config file, environment, flags
- env, defaults, flags, file
- Whatever was read last
? The most specific, most immediate source wins.

@@quiz
What does `-ldflags "-X main.version=1.4.0"` do?
- Sets an environment variable
* Sets the string variable main.version at link time
- Adds a build tag
- Strips the binary
? A common way to embed version info.

@@drill
Which package parses command-line flags?
= flag
? flag.String("addr", ":8080", "listen address").

@@flashcards
`flag.Parse()` :: parse os.Args
`flag.Args()` :: positional arguments
`flag.NewFlagSet` :: flags for one subcommand
`ContinueOnError` :: return parse errors
`fs.Visit` :: only flags that were set
Exit code 2 :: usage error
`signal.NotifyContext` :: context cancelled by Ctrl+C
`-ldflags -X` :: set a variable at link time
cobra :: subcommand framework
GOOS / GOARCH :: cross-compilation targets

@@exercise A netkit CLI with subcommands
Write `run(args []string, stdout, stderr io.Writer) int` for a tool with two subcommands:

- `cidr <prefix>`: print `network <addr> broadcast <addr> hosts <n>` for an IPv4 prefix (use `net/netip`; hosts = 2^(32-bits) − 2). Invalid prefix: `cidr: invalid prefix "<arg>"` on stderr, exit 1.
- `ping [-count N] [-timeout D] <host>`: (simulated) print `pinging <host> <N> times, timeout <D>`; defaults 4 and 1s. Missing host: `ping: missing host`, exit 2. Bad flags: exit 2 (the flag package writes its own message).
- no command: `usage: netkit <cidr|ping> [args]`, exit 2; unknown command: `unknown command "<name>"`, exit 2.

`main` (provided) calls `run` with several argument lists and prints stdout, the first line of stderr and the exit code.
@@starter
package main

import (
	"flag"
	"fmt"
	"io"
	"net/netip"
	"strings"
	"time"
)

func run(args []string, stdout, stderr io.Writer) int {
	_ = flag.NewFlagSet
	_ = netip.ParsePrefix
	_ = time.Second
	return 0
}

func main() {
	cases := [][]string{
		{"cidr", "192.168.10.0/24"},
		{"cidr", "10.0.0.5/30"},
		{"cidr", "nope"},
		{"ping", "-count", "3", "-timeout", "500ms", "web01"},
		{"ping", "web01"},
		{"ping", "-count", "x", "web01"},
		{"ping"},
		{},
		{"dns"},
	}
	for _, c := range cases {
		var out, errOut strings.Builder
		code := run(c, &out, &errOut)
		fmt.Printf("$ netkit %s\n", strings.Join(c, " "))
		fmt.Print(out.String())
		if e := errOut.String(); e != "" {
			fmt.Println("stderr:", strings.SplitN(e, "\n", 2)[0])
		}
		fmt.Println("exit", code)
	}
}
@@expected
$ netkit cidr 192.168.10.0/24
network 192.168.10.0 broadcast 192.168.10.255 hosts 254
exit 0
$ netkit cidr 10.0.0.5/30
network 10.0.0.4 broadcast 10.0.0.7 hosts 2
exit 0
$ netkit cidr nope
stderr: cidr: invalid prefix "nope"
exit 1
$ netkit ping -count 3 -timeout 500ms web01
pinging web01 3 times, timeout 500ms
exit 0
$ netkit ping web01
pinging web01 4 times, timeout 1s
exit 0
$ netkit ping -count x web01
stderr: invalid value "x" for flag -count: parse error
exit 2
$ netkit ping
stderr: ping: missing host
exit 2
$ netkit 
stderr: usage: netkit <cidr|ping> [args]
exit 2
$ netkit dns
stderr: unknown command "dns"
exit 2
@@requires
flag.ContinueOnError
@@hint
Broadcast: `a := p.Masked().Addr().As4()`, then set the host bits: `hostBits := 32 - p.Bits()`; as a uint32 `n := binary.BigEndian.Uint32(a[:]) | (1<<hostBits - 1)`, back with `binary.BigEndian.PutUint32`. Or loop over the 4 bytes. `netip.AddrFrom4(b)` makes an address from bytes. For ping: `fs.Int`, `fs.Duration`, `fs.Parse(args)`, `fs.Arg(0)`. Reject non-IPv4 prefixes too (`p.Addr().Is4()`).
@@solution
package main

import (
	"encoding/binary"
	"flag"
	"fmt"
	"io"
	"net/netip"
	"strings"
	"time"
)

func cidr(args []string, stdout, stderr io.Writer) int {
	if len(args) != 1 {
		fmt.Fprintln(stderr, "usage: netkit cidr <prefix>")
		return 2
	}
	p, err := netip.ParsePrefix(args[0])
	if err != nil || !p.Addr().Is4() {
		fmt.Fprintf(stderr, "cidr: invalid prefix %q\n", args[0])
		return 1
	}
	network := p.Masked().Addr()
	a := network.As4()
	hostBits := 32 - p.Bits()
	n := binary.BigEndian.Uint32(a[:]) | (1<<hostBits - 1)
	var b [4]byte
	binary.BigEndian.PutUint32(b[:], n)
	hosts := (1 << hostBits) - 2
	fmt.Fprintf(stdout, "network %s broadcast %s hosts %d\n", network, netip.AddrFrom4(b), max(hosts, 0))
	return 0
}

func ping(args []string, stdout, stderr io.Writer) int {
	fs := flag.NewFlagSet("ping", flag.ContinueOnError)
	fs.SetOutput(stderr)
	count := fs.Int("count", 4, "number of pings")
	timeout := fs.Duration("timeout", time.Second, "timeout per ping")
	if err := fs.Parse(args); err != nil {
		return 2
	}
	host := fs.Arg(0)
	if host == "" {
		fmt.Fprintln(stderr, "ping: missing host")
		return 2
	}
	fmt.Fprintf(stdout, "pinging %s %d times, timeout %s\n", host, *count, *timeout)
	return 0
}

func run(args []string, stdout, stderr io.Writer) int {
	if len(args) == 0 {
		fmt.Fprintln(stderr, "usage: netkit <cidr|ping> [args]")
		return 2
	}
	switch args[0] {
	case "cidr":
		return cidr(args[1:], stdout, stderr)
	case "ping":
		return ping(args[1:], stdout, stderr)
	default:
		fmt.Fprintf(stderr, "unknown command %q\n", args[0])
		return 2
	}
}

func main() {
	cases := [][]string{
		{"cidr", "192.168.10.0/24"},
		{"cidr", "10.0.0.5/30"},
		{"cidr", "nope"},
		{"ping", "-count", "3", "-timeout", "500ms", "web01"},
		{"ping", "web01"},
		{"ping", "-count", "x", "web01"},
		{"ping"},
		{},
		{"dns"},
	}
	for _, c := range cases {
		var out, errOut strings.Builder
		code := run(c, &out, &errOut)
		fmt.Printf("$ netkit %s\n", strings.Join(c, " "))
		fmt.Print(out.String())
		if e := errOut.String(); e != "" {
			fmt.Println("stderr:", strings.SplitN(e, "\n", 2)[0])
		}
		fmt.Println("exit", code)
	}
}

@@exercise Config precedence: defaults, file, env, flags
Write `Resolve(args []string, file, env map[string]string) ([]Setting, error)` for three settings, in this order: `addr` (default `:8080`), `log-level` (default `info`), `workers` (default `4`).

- a value in `file` (keys as above) overrides the default
- an environment variable `APP_` + upper-case name with `-` → `_` (`APP_LOG_LEVEL`) overrides the file
- a flag the user **explicitly set** in `args` overrides everything; use `fs.Visit` to find them
- each `Setting` records `Name`, `Value` and `Source` (`default`, `file`, `env` or `flag`)
@@starter
package main

import (
	"flag"
	"fmt"
	"io"
	"strings"
)

type Setting struct {
	Name, Value, Source string
}

func Resolve(args []string, file, env map[string]string) ([]Setting, error) {
	_ = flag.NewFlagSet
	_ = io.Discard
	_ = strings.ToUpper
	return nil, nil
}

func main() {
	file := map[string]string{"addr": ":9000", "workers": "8"}
	env := map[string]string{"APP_LOG_LEVEL": "debug", "APP_WORKERS": "6"}
	for _, args := range [][]string{{"-workers", "2"}, {}, {"-addr", ":7000", "-log-level", "warn"}} {
		settings, err := Resolve(args, file, env)
		fmt.Println("args:", args, "err:", err)
		for _, s := range settings {
			fmt.Printf("  %s=%s (%s)\n", s.Name, s.Value, s.Source)
		}
	}
	_, err := Resolve([]string{"-threads", "3"}, nil, nil)
	fmt.Println("err:", err)
}
@@expected
args: [-workers 2] err: <nil>
  addr=:9000 (file)
  log-level=debug (env)
  workers=2 (flag)
args: [] err: <nil>
  addr=:9000 (file)
  log-level=debug (env)
  workers=6 (env)
args: [-addr :7000 -log-level warn] err: <nil>
  addr=:7000 (flag)
  log-level=warn (flag)
  workers=6 (env)
err: flag provided but not defined: -threads
@@requires
fs.Visit
@@hint
Define each setting as a string flag with its default: `fs.String(name, def, "")`, keep the pointers in a map. `fs.SetOutput(io.Discard)` keeps usage text out of the output. After parsing, `set := map[string]bool{}`, `fs.Visit(func(f *flag.Flag) { set[f.Name] = true })`. Then walk the settings in order, applying file, env and flag in turn.
@@solution
package main

import (
	"flag"
	"fmt"
	"io"
	"strings"
)

type Setting struct {
	Name, Value, Source string
}

var defaults = []Setting{
	{"addr", ":8080", "default"},
	{"log-level", "info", "default"},
	{"workers", "4", "default"},
}

func Resolve(args []string, file, env map[string]string) ([]Setting, error) {
	fs := flag.NewFlagSet("app", flag.ContinueOnError)
	fs.SetOutput(io.Discard)
	values := map[string]*string{}
	for _, d := range defaults {
		values[d.Name] = fs.String(d.Name, d.Value, d.Name)
	}
	if err := fs.Parse(args); err != nil {
		return nil, err
	}
	set := map[string]bool{}
	fs.Visit(func(f *flag.Flag) { set[f.Name] = true })
	var out []Setting
	for _, d := range defaults {
		s := d
		if v, ok := file[s.Name]; ok {
			s.Value, s.Source = v, "file"
		}
		envKey := "APP_" + strings.ToUpper(strings.ReplaceAll(s.Name, "-", "_"))
		if v, ok := env[envKey]; ok {
			s.Value, s.Source = v, "env"
		}
		if set[s.Name] {
			s.Value, s.Source = *values[s.Name], "flag"
		}
		out = append(out, s)
	}
	return out, nil
}

func main() {
	file := map[string]string{"addr": ":9000", "workers": "8"}
	env := map[string]string{"APP_LOG_LEVEL": "debug", "APP_WORKERS": "6"}
	for _, args := range [][]string{{"-workers", "2"}, {}, {"-addr", ":7000", "-log-level", "warn"}} {
		settings, err := Resolve(args, file, env)
		fmt.Println("args:", args, "err:", err)
		for _, s := range settings {
			fmt.Printf("  %s=%s (%s)\n", s.Name, s.Value, s.Source)
		}
	}
	_, err := Resolve([]string{"-threads", "3"}, nil, nil)
	fmt.Println("err:", err)
}
