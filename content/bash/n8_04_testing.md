@@lesson n8-testing Testing Shell Scripts: Assertions, Mocks and Bats
@@concepts testing, functions
@@text
Shell scripts deploy software, delete files and rotate backups: exactly the code where a bug hurts most. They're also very testable once you structure them right.

## Make scripts testable

1. Put logic in **functions**, and the top-level flow in `main`.
2. Run `main` only when executed, not when sourced: `[[ ${BASH_SOURCE[0]} == "$0" ]] && main "$@"`.
3. Make side effects **injectable**: paths from variables (`: "${CONFIG_DIR:=/etc/app}"`), commands through wrappers (`run() { "$@"; }`) or functions you can override.
4. Separate **deciding** from **doing**: a function that computes which backups to delete is easy to test; the `rm` that follows is trivial.

## Bats: the standard framework

[Bats-core](https://github.com/bats-core/bats-core) runs `.bats` files where each `@test` is a function:

```bash,norun
#!/usr/bin/env bats
setup() {
    source ./backup.sh          # loads functions; main doesn't run
    TMP=$(mktemp -d)
}
teardown() { rm -rf "$TMP"; }

@test "keeps the 3 newest backups" {
    touch "$TMP"/backup-2026-11-0{1..5}.tgz
    run old_backups "$TMP" 3
    [ "$status" -eq 0 ]
    [ "${lines[0]}" = "$TMP/backup-2026-11-01.tgz" ]
    [ "${#lines[@]}" -eq 2 ]
}

@test "fails on a missing directory" {
    run old_backups /nope 3
    [ "$status" -eq 1 ]
    [[ $output == *"not a directory"* ]]
}
```

`run` captures `$status`, `$output` and `$lines` without aborting the test. Helper libraries `bats-assert` and `bats-file` add `assert_output`, `assert_success`, `assert_file_exists`. Run with `bats test/` (TAP output, works in CI).

## Mocking commands

Because functions take precedence over commands on `PATH`, a test can **replace** any command:

```bash
systemctl() { echo "systemctl $*" >> "$CALLS"; return 0; }   # record, don't act
CALLS=$(mktemp)
systemctl restart nginx
cat "$CALLS"; rm -f "$CALLS"
```

For scripts run as separate processes, put stub executables in a temp directory and prepend it: `PATH="$TMP/bin:$PATH"`.

## Golden files and snapshot tests

For report-style scripts, store the expected output in `test/expected/report.txt` and compare: `diff -u test/expected/report.txt <(./report.sh test/fixtures/input.log)`. When the output changes on purpose, regenerate the golden file and review the diff in Git.

## Beyond unit tests

- **ShellCheck** in CI catches quoting bugs before tests even run.
- Run tests in a **container** for the target distribution (`docker run --rm -v "$PWD":/w -w /w debian:13 bats test`).
- `set -x` with `PS4='+ ${BASH_SOURCE##*/}:${LINENO}: '` traces with file and line numbers when debugging a failure.

@@quiz
Why does `[[ ${BASH_SOURCE[0]} == "$0" ]] && main "$@"` help testing?
* Sourcing the script in a test loads its functions without running main
- It makes the script faster
- It's required by Bats
- It checks the script's permissions
? When sourced, BASH_SOURCE[0] is the script but $0 is the caller.

@@quiz
How can a test stop `systemctl restart nginx` from really running?
- It can't
* Define a `systemctl` function that records its arguments; functions take precedence over commands
- Run the test as a normal user
- Use `set -n`
? Or put a stub executable first on PATH.

@@quiz
What does Bats' `run` do?
- Runs the test file
* Runs a command and captures its exit status, output and lines without failing the test
- Executes in parallel
- Starts a container
? Then assert on $status and $output.

@@flashcards
Bats `@test` :: one test case
`run cmd` :: capture $status, $output, $lines
`setup` / `teardown` :: per-test hooks
bats-assert :: assert_output, assert_success
function mock :: override a command in tests
stub on PATH :: `PATH="$TMP/bin:$PATH"`
golden file :: stored expected output
`diff -u` :: readable comparison
`PS4` :: prefix for set -x traces

@@exercise A tiny test framework
Write a minimal framework in pure Bash:

- `assert_eq EXPECTED ACTUAL [MSG]` and `assert_status EXPECTED CMD...`: on failure, print `    expected: ...` / `    actual:   ...` and return 1
- `run_tests`: finds every function named `test_*` (via `declare -F`), runs each in a **subshell** (so tests can't affect each other), and prints `PASS name` or `FAIL name`
- end with `N passed, M failed`

Use it to test `version_gt A B` (true if semantic version A > B), including a deliberately wrong test to see a failure.
@@starter
version_gt() {
    local IFS=.
    local -a a=($1) b=($2)
    local i
    for i in 0 1 2; do
        (( ${a[i]:-0} > ${b[i]:-0} )) && return 0
        (( ${a[i]:-0} < ${b[i]:-0} )) && return 1
    done
    return 1
}
echo TODO
@@expected
PASS test_equal_is_not_greater
PASS test_major_wins
PASS test_numeric_not_lexical
FAIL test_wrong_on_purpose
    expected: 1.2
    actual:   1.10
    (comparing the larger of 1.2 and 1.10)
3 passed, 1 failed
@@requires
declare -F
test_
(
@@hint
`for t in $(declare -F | awk '{print $3}' | grep '^test_'); do if out=$( "$t" 2>&1 ); then ...`: `$( )` runs each test in a subshell and captures its messages. `declare -F` lists functions alphabetically, which gives a stable order. A helper `bigger A B` that prints the larger version is handy for the failing test.
@@solution
version_gt() {
    local IFS=.
    local -a a=($1) b=($2)
    local i
    for i in 0 1 2; do
        (( ${a[i]:-0} > ${b[i]:-0} )) && return 0
        (( ${a[i]:-0} < ${b[i]:-0} )) && return 1
    done
    return 1
}

assert_eq() {
    [[ $1 == "$2" ]] && return 0
    echo "    expected: $1"
    echo "    actual:   $2"
    [[ -n ${3:-} ]] && echo "    ($3)"
    return 1
}

assert_status() {
    local expected=$1
    shift
    "$@"
    local actual=$?
    assert_eq "$expected" "$actual" "exit status of: $*"
}

bigger() { if version_gt "$1" "$2"; then echo "$1"; else echo "$2"; fi; }

test_major_wins() { assert_status 0 version_gt 2.0.0 1.9.9 && assert_status 1 version_gt 1.9.9 2.0.0; }
test_equal_is_not_greater() { assert_status 1 version_gt 1.4.0 1.4.0; }
test_numeric_not_lexical() { assert_status 0 version_gt 1.10.0 1.9.0; }
test_wrong_on_purpose() { assert_eq 1.2 "$(bigger 1.2 1.10)" "comparing the larger of 1.2 and 1.10"; }

run_tests() {
    local t passed=0 failed=0 out
    for t in $(declare -F | awk '{print $3}' | grep '^test_'); do
        if out=$( "$t" 2>&1 ); then
            echo "PASS $t"
            ((passed++))
        else
            echo "FAIL $t"
            [[ -n $out ]] && echo "$out"
            ((failed++))
        fi
    done
    echo "$passed passed, $failed failed"
    (( failed == 0 ))
}

run_tests || true

@@exercise Mock the system
`restart_if_unhealthy SERVICE` checks a service with `systemctl is-active`, restarts it if it's not active, and checks again. Test it **without touching the real system**:

- mock `systemctl` as a function that records every call in a `calls` array and answers from a `state` associative array (`restart` sets the state to `active` unless the service is in `broken`)
- mock `logger` to record messages
- run three scenarios (already active, recovers after restart, stays failed) and print the recorded calls and the function's status for each
@@starter
restart_if_unhealthy() {
    local svc=$1
    if systemctl is-active --quiet "$svc"; then
        return 0
    fi
    logger -t watchdog "$svc is down, restarting"
    systemctl restart "$svc"
    if systemctl is-active --quiet "$svc"; then
        logger -t watchdog "$svc recovered"
        return 0
    fi
    logger -t watchdog "$svc failed to restart"
    return 2
}
echo TODO
@@expected
== nginx (active)
calls: is-active nginx
logs: (none)
status: 0
== redis (failed)
calls: is-active redis; restart redis; is-active redis
logs: redis is down, restarting | redis recovered
status: 0
== billing (failed, broken)
calls: is-active billing; restart billing; is-active billing
logs: billing is down, restarting | billing failed to restart
status: 2
@@requires
systemctl()
logger()
calls+=
@@hint
In the mock, skip option arguments: `systemctl() { local action=$1; shift; [[ $1 == --quiet ]] && shift; calls+=("$action $1"); case $action in is-active) [[ ${state[$1]} == active ]] ;; restart) [[ -n ${broken[$1]:-} ]] || state[$1]=active ;; esac; }`. Join arrays with a custom separator using a small loop or `printf` and trimming.
@@solution
restart_if_unhealthy() {
    local svc=$1
    if systemctl is-active --quiet "$svc"; then
        return 0
    fi
    logger -t watchdog "$svc is down, restarting"
    systemctl restart "$svc"
    if systemctl is-active --quiet "$svc"; then
        logger -t watchdog "$svc recovered"
        return 0
    fi
    logger -t watchdog "$svc failed to restart"
    return 2
}

declare -A state=() broken=()
calls=() logs=()

systemctl() {
    local action=$1
    shift
    [[ $1 == --quiet ]] && shift
    calls+=("$action $1")
    case $action in
        is-active) [[ ${state[$1]:-} == active ]] ;;
        restart) [[ -n ${broken[$1]:-} ]] || state[$1]=active ;;
    esac
}

logger() {
    [[ $1 == -t ]] && shift 2
    logs+=("$*")
}

join_by() {
    local sep=$1 out="" item
    shift
    for item in "$@"; do out+="${out:+$sep}$item"; done
    printf '%s' "$out"
}

scenario() {
    local svc=$1 initial=$2 is_broken=${3:-} rc
    calls=() logs=()
    state[$svc]=$initial
    [[ -n $is_broken ]] && broken[$svc]=1
    echo "== $svc ($initial${is_broken:+, broken})"
    restart_if_unhealthy "$svc"
    rc=$?
    echo "calls: $(join_by '; ' "${calls[@]}")"
    if (( ${#logs[@]} )); then echo "logs: $(join_by ' | ' "${logs[@]}")"; else echo "logs: (none)"; fi
    echo "status: $rc"
}

scenario nginx active
scenario redis failed
scenario billing failed yes
