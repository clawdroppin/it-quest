@@lesson l4-robust Robust Scripts: Strict Mode, Options, Traps
@@concepts errors, automation
@@text
A script that runs at 3 AM from cron, with nobody watching, has to fail **loudly and safely**. This lesson is the template professionals start every script from.

## The header

```bash,norun
#!/usr/bin/env bash
set -euo pipefail
```

- `-e`: exit as soon as a command fails (instead of carrying on with broken state),
- `-u`: using an unset variable is an error (catches typos like `$BAKUP_DIR`),
- `-o pipefail`: a pipeline fails if **any** part fails, not just the last.

Why it matters, in one line: without `-u`, a typo in `rm -rf "$BUILD_DIR/"*` with an empty variable becomes `rm -rf /*`.

```bash
set -euo pipefail
name="web01"
echo "checking $name"
false || echo "a failure handled with || doesn't stop the script"
if ! grep -q "nope" /dev/null; then echo "neither does one tested with if"; fi
echo "still running"
```

## Options with getopts

```bash
usage() { echo "usage: $0 [-v] [-n COUNT] TARGET" >&2; exit 2; }

parse() {
    local OPTIND opt verbose=0 count=1
    while getopts "vn:" opt; do
        case $opt in
            v) verbose=1 ;;
            n) count=$OPTARG ;;
            *) usage ;;
        esac
    done
    shift $(( OPTIND - 1 ))
    echo "verbose=$verbose count=$count target=${1:-none}"
}
parse -v -n 3 web01
parse db01
```

`"vn:"` means `-v` is a flag and `-n` takes a value (the colon). After the loop, `shift` drops the options so `$1` is the first real argument.

## Cleanup with trap

`trap` runs code when the script exits, even after an error, which is perfect for temporary files and lock files:

```bash
work() {
    local tmp
    tmp=$(mktemp -d)
    trap 'rm -rf "$tmp"; echo "cleaned up"; trap - RETURN' RETURN
    echo "working in a temp folder"
    touch "$tmp/data"
}
work
```

In scripts you'll usually write `trap cleanup EXIT`. `mktemp` makes unique temporary names, so two runs never collide.

## Logging and locking

```bash,norun
log() { echo "$(date '+%F %T') [$1] ${*:2}" | tee -a /var/log/myjob.log >&2; }
log INFO "backup started"

# Only one copy at a time (cron may start the next run before this one finishes):
exec 9> /var/lock/myjob.lock
flock -n 9 || { log WARN "already running"; exit 1; }
```

## Exit codes are your API

`exit 0` for success, non-zero for failure, and document them: monitoring, cron mail and CI pipelines all decide based on them.

## ShellCheck

[ShellCheck](https://www.shellcheck.net) finds the bugs this course warned you about (unquoted variables, `for f in $(ls)`, missing `local`...). Install it (`sudo apt install shellcheck`) and run `shellcheck myscript.sh`. The VS Code extension shows warnings as you type.

@@quiz
What does `set -u` do?
* Makes using an unset variable an error
- Makes all variables uppercase
- Unsets all variables
- Runs as an unprivileged user
? It catches typos in variable names before they do damage.

@@quiz
With `set -e`, a command fails inside `if ! cmd; then`. Does the script exit?
- yes, always
* no: commands tested by `if`, `&&` or `||` don't trigger `-e`
- only with pipefail
- only in functions
? `-e` ignores failures that the script is explicitly checking.

@@quiz
What is `trap cleanup EXIT` for?
* Running the `cleanup` function whenever the script ends, even after an error
- Catching errors so the script never stops
- Hiding error messages
- Exiting when cleanup fails
? Ideal for removing temporary files and locks.

@@drill
Which builtin parses short options like `-v` and `-o file`?
= getopts
? while getopts 'vo:' opt; do ... done.

@@flashcards
set -e :: Exit when a command fails
set -u :: Error on unset variables
set -o pipefail :: A pipeline fails if any part fails
getopts :: Parse short options like -v -n 3
shift :: Drop the first argument(s)
trap ... EXIT :: Run code when the script ends
mktemp :: Create a unique temporary file (-d: folder)
flock :: Make sure only one copy of a job runs
ShellCheck :: Linter that finds common shell script bugs

@@exercise Option parsing
Write `backup_opts` using **getopts**: `-d DIR` sets the destination (default `/backup`), `-k N` how many to keep (default `7`), `-z` turns on compression (default off). The remaining argument is the source. Print `src=<source> dest=<dir> keep=<n> zip=<0|1>`.
@@starter
backup_opts() {
    echo "TODO: not done yet" >&2
    return 1
}

@@tests
check "$(backup_opts /etc)" "src=/etc dest=/backup keep=7 zip=0" "backup_opts /etc"
check "$(backup_opts -z -k 3 /home)" "src=/home dest=/backup keep=3 zip=1" "backup_opts -z -k 3 /home"
check "$(backup_opts -d /mnt/nas -z /var/www)" "src=/var/www dest=/mnt/nas keep=7 zip=1" "backup_opts -d /mnt/nas -z /var/www"
@@requires
getopts
@@hint
Copy the shape of `parse` from the lesson: `local OPTIND opt dest=/backup keep=7 zip=0`, `while getopts "d:k:z" opt; do case ... esac; done`, then `shift $(( OPTIND - 1 ))`.
@@solution
backup_opts() {
    local OPTIND opt dest=/backup keep=7 zip=0
    while getopts "d:k:z" opt; do
        case $opt in
            d) dest=$OPTARG ;;
            k) keep=$OPTARG ;;
            z) zip=1 ;;
            *) return 2 ;;
        esac
    done
    shift $(( OPTIND - 1 ))
    echo "src=$1 dest=$dest keep=$keep zip=$zip"
}

@@exercise Log lines
Write `log_line` that takes a level and a message (the message can be several words) and prints `[LEVEL] message` with the level in **uppercase** and padded to 5 characters inside the brackets, like `[INFO ] backup started` and `[ERROR] disk full`.
@@starter
log_line() {
    echo "TODO: not done yet" >&2
    return 1
}

@@tests
check "$(log_line info backup started)" "[INFO ] backup started" "log_line info backup started"
check "$(log_line error disk full)" "[ERROR] disk full" "log_line error disk full"
check "$(log_line warn "two  spaces")" "[WARN ] two  spaces" "log_line warn \"two  spaces\""
@@hint
`local level="${1^^}"; shift` removes the level, then `printf '[%-5s] %s\n' "$level" "$*"`. `%-5s` left-aligns in 5 characters; `"$*"` joins the rest with spaces.
@@solution
log_line() {
    local level="${1^^}"
    shift
    printf '[%-5s] %s\n' "$level" "$*"
}
