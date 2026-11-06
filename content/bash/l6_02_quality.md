@@lesson l6-quality Testing, Linting and Safe Scripts
@@concepts testing, errors, files
@@text
Bash is easy to write and easy to get wrong: a missing pair of quotes, an unset variable or a failed `cd` can delete the wrong files. Professionals use three things to stop that: **ShellCheck**, **tests**, and a few patterns that make scripts safe by default.

## ShellCheck: run it on everything

```text
$ shellcheck backup.sh

In backup.sh line 7:
rm -rf $dir/*
       ^--^ SC2086: Double quote to prevent globbing and word splitting.
       ^--^ SC2115: Use "${var:?}" to ensure this never expands to /* .
```

ShellCheck knows hundreds of Bash mistakes and explains each one (search for the code, e.g. "SC2086"). Install it with `apt install shellcheck`, `brew install shellcheck` or `winget install koalaman.shellcheck`, and add it to your editor (VS Code has an extension) and to CI. When a warning is really wrong for your case, silence that one line with a comment: `# shellcheck disable=SC2034`.

## Write scripts as functions, then test the functions

A script whose logic lives in functions, with a `main` at the bottom that only runs when the file is executed, can be loaded and tested:

```bash,norun
#!/usr/bin/env bash
# rotate.sh
set -euo pipefail

keep_newest() {          # prints the files to delete: everything but the newest N
    local n=$1; shift
    printf '%s\n' "$@" | sort -r | tail -n +"$((n + 1))"
}

main() {
    keep_newest 7 /backup/db-*.sql.gz | xargs -r rm --
}

# Run main only when executed, not when sourced by the tests.
if [[ "${BASH_SOURCE[0]}" == "$0" ]]; then main "$@"; fi
```

## Testing with Bats

**Bats** (Bash Automated Testing System) is the standard test framework (`apt install bats`, or add `bats-core` to the repo as a Git submodule):

```bash,norun
# test/rotate.bats
setup() {
    source "$BATS_TEST_DIRNAME/../rotate.sh"
}

@test "keeps the newest 2" {
    run keep_newest 2 db-2026-10-01 db-2026-10-03 db-2026-10-02
    [ "$status" -eq 0 ]
    [ "$output" = "db-2026-10-01" ]
}

@test "nothing to delete when there are fewer files" {
    run keep_newest 5 a b
    [ -z "$output" ]
}
```

Run with `bats test/`. `run` captures the exit code in `$status` and the output in `$output`. Without Bats, a plain `assert_eq` function (like the one in the Server Security Audit and tix projects) does the same job.

**Test against a fake system, not the real one**: point the script at a temporary folder (`mktemp -d`) with sample files, through a variable like `ROOT=/` that tests override. Never test `rm` logic against your real `/backup`.

## Safe-by-default patterns

```bash,norun
#!/usr/bin/env bash
set -euo pipefail                  # stop on errors, unset variables and failed pipes
IFS=$'\n\t'                        # word splitting only on newlines and tabs

work=$(mktemp -d)                  # a private temporary folder...
trap 'rm -rf "$work"' EXIT         # ...always removed, even on errors or Ctrl+C

cd "$target" || exit 1             # never carry on in the wrong folder
rm -rf -- "${dir:?}/"*             # :? stops the script if dir is empty, so this can't become rm -rf /*

if [[ ${DRY_RUN:-0} == 1 ]]; then  # a dry-run mode for anything destructive
    run() { echo "would run: $*"; }
else
    run() { "$@"; }
fi
run rm -- "$old_file"
```

| Pattern | Protects against |
|---|---|
| `set -u` | typos in variable names silently becoming empty |
| `"$var"` everywhere | file names with spaces breaking into several arguments |
| `--` before file arguments | file names starting with `-` being read as options |
| `${var:?}` | dangerous commands with an empty variable |
| `mktemp` + `trap ... EXIT` | predictable temp names (a security hole) and leftover files |
| write to a temp file, then `mv` | half-written files after a crash (a rename is atomic) |
| `flock` | two copies of a cron job running at once |
| dry-run mode | finding out what it *would* do before it does it |

## set -e has blind spots

`set -e` doesn't stop the script when the failing command is part of an `if`, `&&`, `||`, or `while` test, or inside `$(...)` assigned with `local`. So `local x=$(false)` carries on, because `local` itself succeeded. Declare first, then assign: `local x; x=$(false)`. Check important commands explicitly anyway.

## Locking: one copy at a time

```bash,norun
exec 9> /var/lock/nightly-backup.lock    # open file descriptor 9 on the lock file
flock -n 9 || { echo "already running" >&2; exit 1; }
# ... the lock is held until the script exits
```

@@quiz
What does ShellCheck do?
- Runs your script in a sandbox
* Analyzes the script's code and warns about common bugs, like missing quotes, with an explanation for each
- Formats the script
- Checks that the commands it uses are installed
? It's a linter: static analysis, nothing is executed.

@@quiz
Why end a script with `if [[ "${BASH_SOURCE[0]}" == "$0" ]]; then main "$@"; fi`?
* So tests can `source` the file to get its functions without running the script
- It makes the script faster
- It's required by `set -e`
- It prevents the script from running as root
? When sourced, BASH_SOURCE[0] is the file but $0 is the caller.

@@quiz
What does `rm -rf -- "${dir:?}/"*` protect against?
- Deleting hidden files
* An empty or unset `dir` turning the command into `rm -rf /*`, and file names starting with `-`
- Deleting files owned by root
- Running out of disk space
? A real bug like this once deleted users' whole home folders in a popular program's installer.

@@quiz
A cron job runs every 5 minutes but sometimes takes 7. How do you stop two copies running at once?
- `sleep 120` at the start
- `set -e`
* Take a lock with `flock -n` at the start and exit if it's already held
- Use `nohup`
? The lock is released automatically when the first copy exits, even if it crashes.

@@drill
Which tool finds common bugs in shell scripts?
= shellcheck
? Run it on every script.

@@flashcards
ShellCheck :: Bash linter; explains each warning (SC2086, ...)
`# shellcheck disable=SC2034` :: silence one warning on the next line
Bats :: Bash Automated Testing System (`@test`, `run`, `$status`, `$output`)
`[[ "${BASH_SOURCE[0]}" == "$0" ]]` :: true only when the file is executed, not sourced
`set -euo pipefail` :: exit on error, unset variables, and failed pipes
`${var:?}` :: stop if the variable is empty or unset
`mktemp -d` + `trap 'rm -rf "$work"' EXIT` :: safe temp folder, always cleaned up
`--` :: everything after is an argument, not an option
`flock -n 9` :: take a lock or fail immediately
dry-run :: print what would happen instead of doing it
`local x; x=$(cmd)` :: keeps the exit code that `local x=$(cmd)` hides

@@exercise Keep the newest
`to_delete N FILE...` prints the files that a cleanup should **delete** so that only the newest `N` stay. The names contain an ISO date (`db-2026-10-30.sql.gz`), so sorting the names sorts by date. Print them oldest first. If there are N or fewer files, print nothing. `N` must be a whole number: otherwise print `to_delete: N must be a number` to stderr and return 2.
@@starter
to_delete() {
    echo "TODO" >&2; return 1
}

@@tests
files=(db-2026-10-28.sql.gz db-2026-10-31.sql.gz db-2026-10-29.sql.gz db-2026-10-30.sql.gz)
check "$(to_delete 2 "${files[@]}")" $'db-2026-10-28.sql.gz\ndb-2026-10-29.sql.gz' "keep 2 of 4"
check "$(to_delete 4 "${files[@]}")" "" "keep all"
check "$(to_delete 9 "${files[@]}")" "" "more than there are"
check "$(to_delete 0 a b)" $'a\nb' "keep none"
check "$(to_delete x a 2>&1; echo $?)" $'to_delete: N must be a number\n2' "bad N"
check "$(to_delete 1 'my backup 2026-10-01' 'my backup 2026-10-02')" "my backup 2026-10-01" "names with spaces"
@@hint
Check with `[[ $1 =~ ^[0-9]+$ ]]`, then `local n=$1; shift`. If `(( $# <= n ))` return. Otherwise `printf '%s\n' "$@" | sort | head -n $(( $# - n ))`.
@@solution
to_delete() {
    [[ ${1:-} =~ ^[0-9]+$ ]] || { echo "to_delete: N must be a number" >&2; return 2; }
    local n=$1
    shift
    (( $# > n )) || return 0
    printf '%s\n' "$@" | sort | head -n $(( $# - n ))
}

@@exercise A dry-run mode
Write `run`: when `DRY_RUN` is `1` it prints `would run: ` followed by the command (each argument quoted with `printf '%q'`, separated by spaces) and doesn't run it; otherwise it runs the command with its arguments exactly as given and returns its exit code.
@@starter
run() {
    echo "TODO" >&2; return 1
}

@@tests
work=$(mktemp -d)
touch "$work/old file.log"
DRY_RUN=1
check "$(run rm -- "$work/old file.log" | sed "s|$work|W|")" 'would run: rm -- W/old\ file.log' "dry run prints"
check "$(ls "$work")" "old file.log" "dry run deletes nothing"
DRY_RUN=0
run rm -- "$work/old file.log"
check "$(ls "$work")" "" "real run deletes"
check "$(run false; echo $?)" "1" "exit code passed on"
rm -rf "$work"
@@hint
`if [[ ${DRY_RUN:-0} == 1 ]]; then echo "would run: $(printf '%q ' "$@")"` (that leaves a trailing space, so build the string first and trim it with `${s% }`), `else "$@"; fi`.
@@solution
run() {
    if [[ ${DRY_RUN:-0} == 1 ]]; then
        local s
        s=$(printf '%q ' "$@")
        echo "would run: ${s% }"
    else
        "$@"
    fi
}
