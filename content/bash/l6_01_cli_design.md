@@lesson l6-cli-design Designing Command-Line Tools
@@concepts functions, input, errors
@@text
A script you run yourself only has to work for you. A **command-line tool** is used by other people, called from other scripts, cron jobs and CI pipelines, and piped into `grep`. It has to behave the way Unix tools behave. This lesson covers those conventions.

## The conventions

| Convention | Why |
|---|---|
| results go to **stdout**, messages and errors to **stderr** | `tool | grep x` must not get error messages mixed into the data |
| exit code **0** on success, **1** on errors, **2** for wrong usage | scripts and CI check `$?` |
| `-h` / `--help` prints usage and exits 0 | everyone tries it first |
| short options (`-v`), and long ones (`--verbose`) for clarity | `-v` for typing, `--verbose` for scripts |
| `--` ends the options | `rm -- -weird-file-name` |
| read stdin when no file is given, or when the file is `-` | `cat log | tool` and `tool log` both work |
| quiet on success | "no news is good news": print only what was asked for |
| no colour when stdout isn't a terminal (`[[ -t 1 ]]`), or when `NO_COLOR` is set | colour codes break pipes and log files |

## Parsing options with getopts

```bash
#!/usr/bin/env bash
set -euo pipefail
set -- -v -k 14 /etc /home          # pretend the user typed: backup -v -k 14 /etc /home

usage() {
    echo "usage: backup [-v] [-d DEST] [-k DAYS] SOURCE..." >&2
}

verbose=0 dest=/backup keep=7
while getopts ":vd:k:h" opt; do       # leading ":" = we handle errors ourselves
    case $opt in
        v) verbose=1 ;;
        d) dest=$OPTARG ;;               # "d:" means -d takes a value
        k) keep=$OPTARG ;;
        h) usage; exit 0 ;;
        :) echo "backup: -$OPTARG needs a value" >&2; exit 2 ;;
        \?) echo "backup: unknown option -$OPTARG" >&2; usage; exit 2 ;;
    esac
done
shift $((OPTIND - 1))                   # what's left are the positional arguments

[[ $# -gt 0 ]] || { usage; exit 2; }
[[ $keep =~ ^[0-9]+$ ]] || { echo "backup: -k must be a number" >&2; exit 2; }
echo "verbose=$verbose dest=$dest keep=$keep sources=$*"
```

`getopts` handles `-v -d /mnt`, `-vd /mnt` and `-d/mnt` alike. It doesn't do long options; for those, a `while`/`case` loop over `"$1"` is common:

```bash,norun
while [[ $# -gt 0 ]]; do
    case $1 in
        -v | --verbose) verbose=1; shift ;;
        -d | --dest) dest=${2:?--dest needs a value}; shift 2 ;;
        --dest=*) dest=${1#*=}; shift ;;
        --) shift; break ;;
        -*) echo "unknown option: $1" >&2; exit 2 ;;
        *) break ;;
    esac
done
```

## Subcommands

Tools that do several related things (`git commit`, `docker run`, `systemctl restart`) dispatch on the first word. With one function per subcommand it stays tidy:

```bash
cmd_start()  { echo "starting $1"; }
cmd_stop()   { echo "stopping $1"; }
cmd_status() { echo "$1 is running"; }

main() {
    local cmd=${1:-}
    [[ -n $cmd ]] && shift
    case $cmd in
        start | stop | status) "cmd_$cmd" "$@" ;;       # call the function named cmd_<command>
        "" | help | -h | --help) echo "usage: svc start|stop|status NAME" ;;
        *) echo "svc: unknown command: $cmd" >&2; return 2 ;;
    esac
}

main status nginx
main restart nginx || echo "(exit code $?)"
```

## Reading from files or stdin

```bash,norun
# "$@" files, or stdin if there are none: exactly what cat, grep and sort do
count_errors() {
    grep -c ERROR -- "${@:--}"      # ${@:--} means "$@", or "-" (stdin) when empty
}
count_errors app.log
journalctl -u nginx | count_errors
```

## Configuration

Follow the **XDG** folders on Linux, so users know where things are and backups pick them up:

| What | Where |
|---|---|
| config | `${XDG_CONFIG_HOME:-$HOME/.config}/tool/config` |
| data | `${XDG_DATA_HOME:-$HOME/.local/share}/tool/` |
| cache | `${XDG_CACHE_HOME:-$HOME/.cache}/tool/` |
| system-wide config | `/etc/tool/config` |

Priority, from lowest to highest: built-in defaults, then `/etc`, then the user's config file, then **environment variables**, then **command-line options**. A simple `KEY=value` config file can be read safely without `source` (which would run any code in it):

```bash,norun
while IFS='=' read -r key value; do
    [[ $key =~ ^[A-Z_]+$ ]] || continue        # skip comments, blank lines, junk
    case $key in
        DEST) dest=$value ;;
        KEEP_DAYS) keep=$value ;;
    esac
done < "$config_file"
```

@@quiz
Your tool prints a warning about a missing optional file. Where should it go?
- stdout, so the user sees it
* stderr, so it doesn't end up in the data when the output is piped or redirected
- a log file only
- nowhere: tools should be silent
? `tool > report.csv` should create a clean CSV, with the warning still visible on screen.

@@quiz
What does the leading `:` in `getopts ":vd:k:h"` do?
- Makes every option take a value
* Puts getopts in silent mode: you handle unknown options (`?`) and missing values (`:`) yourself
- Allows long options
- Makes the options case-insensitive
? Without it, getopts prints its own generic error messages.

@@quiz
After the getopts loop, why `shift $((OPTIND - 1))`?
* It removes the parsed options, so `$1`, `$2`... are the remaining arguments
- It resets getopts for the next call
- It's required to avoid an infinite loop
- It moves the options to the end
? OPTIND is the index of the next argument to process.

@@quiz
Why read a config file with a `while read` loop instead of `source config`?
- `source` is slower
* `source` runs the file as code, so anything in it would be executed
- `source` doesn't support `=`
- `while read` handles comments automatically
? Config files are often editable by more people than the script itself. Parse them as data.

@@flashcards
stdout / stderr :: data / messages and errors (`>&2`)
exit codes :: 0 success, 1 error, 2 wrong usage (by convention)
`getopts ":ab:"` :: parse short options; `b:` takes a value in `$OPTARG`
`shift $((OPTIND - 1))` :: drop the parsed options
`--` :: end of options
`[[ -t 1 ]]` :: is stdout a terminal?
`NO_COLOR` :: environment variable: the user doesn't want colour
`"${@:--}"` :: the arguments, or `-` (stdin) if there are none
`"cmd_$cmd" "$@"` :: dispatch a subcommand to a function
XDG folders :: `~/.config`, `~/.local/share`, `~/.cache`
`${2:?message}` :: fail with a message if the value is missing

@@exercise Parse the options
Write `parse_args` for a `ping-sweep` tool: `-c COUNT` (default 1), `-t TIMEOUT` (default 2), `-q` (quiet, default off), followed by one or more hosts. It prints `count=<c> timeout=<t> quiet=<0|1> hosts=<hosts separated by spaces>`. An unknown option prints `unknown option -X` to stderr and returns 2; no hosts prints `no hosts` to stderr and returns 2. Use `getopts`, and remember `local OPTIND`.
@@starter
parse_args() {
    local count=1 timeout=2 quiet=0 opt OPTIND=1
    echo "TODO" >&2; return 1
}

@@tests
check "$(parse_args web01)" "count=1 timeout=2 quiet=0 hosts=web01" "defaults"
check "$(parse_args -c 3 -q web01 db01)" "count=3 timeout=2 quiet=1 hosts=web01 db01" "options and two hosts"
check "$(parse_args -qc5 -t1 h)" "count=5 timeout=1 quiet=1 hosts=h" "bundled options"
check "$(parse_args -x h 2>&1)" "unknown option -x" "unknown option message"
check "$(parse_args -x h 2>/dev/null; echo $?)" "2" "unknown option exit code"
check "$(parse_args -c 2 2>&1; echo $?)" $'no hosts\n2' "no hosts"
@@hint
`while getopts ":c:t:q" opt; do case $opt in c) count=$OPTARG ;; t) timeout=$OPTARG ;; q) quiet=1 ;; *) echo "unknown option -$OPTARG" >&2; return 2 ;; esac; done`, then `shift $((OPTIND - 1))` and `[[ $# -gt 0 ]] || { echo "no hosts" >&2; return 2; }`.
@@solution
parse_args() {
    local count=1 timeout=2 quiet=0 opt OPTIND=1
    while getopts ":c:t:q" opt; do
        case $opt in
            c) count=$OPTARG ;;
            t) timeout=$OPTARG ;;
            q) quiet=1 ;;
            *) echo "unknown option -$OPTARG" >&2; return 2 ;;
        esac
    done
    shift $((OPTIND - 1))
    [[ $# -gt 0 ]] || { echo "no hosts" >&2; return 2; }
    echo "count=$count timeout=$timeout quiet=$quiet hosts=$*"
}

@@exercise Subcommands
Write `vault`, a tiny key-value tool that keeps its data in the associative array `store`. `vault set KEY VALUE` stores and prints nothing; `vault get KEY` prints the value, or `vault: no such key: KEY` to stderr and returns 1; `vault list` prints the keys sorted, one per line; anything else prints `vault: unknown command: X` to stderr and returns 2. Use one function per subcommand (`vault_set`, `vault_get`, `vault_list`).
@@starter
declare -A store

vault() {
    echo "TODO" >&2; return 1
}

@@tests
vault set motd "Welcome to Tokyo"
vault set backup_dir /mnt/nas
check "$(vault get motd)" "Welcome to Tokyo" "get a value with spaces"
check "$(vault list)" $'backup_dir\nmotd' "list, sorted"
check "$(vault get nope 2>&1; echo $?)" $'vault: no such key: nope\n1' "missing key"
check "$(vault frob 2>&1; echo $?)" $'vault: unknown command: frob\n2' "unknown command"
vault set motd "Closed on Monday"
check "$(vault get motd)" "Closed on Monday" "overwrite"
@@hint
`vault_set() { store[$1]=$2; }`, `vault_get() { [[ -v store[$1] ]] || { echo "vault: no such key: $1" >&2; return 1; }; echo "${store[$1]}"; }`, `vault_list() { printf '%s\n' "${!store[@]}" | sort; }`. In `vault`: `local cmd=$1; shift; case $cmd in set|get|list) "vault_$cmd" "$@" ;; *) ...; return 2 ;; esac`.
@@solution
declare -A store

vault_set() { store[$1]=$2; }
vault_get() {
    [[ -v store[$1] ]] || { echo "vault: no such key: $1" >&2; return 1; }
    echo "${store[$1]}"
}
vault_list() { printf '%s\n' "${!store[@]}" | sort; }

vault() {
    local cmd=${1:-}
    [[ $# -gt 0 ]] && shift
    case $cmd in
        set | get | list) "vault_$cmd" "$@" ;;
        *) echo "vault: unknown command: $cmd" >&2; return 2 ;;
    esac
}
