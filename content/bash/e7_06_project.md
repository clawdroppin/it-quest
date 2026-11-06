@@lesson e7-project Project: A Task Runner with Dependencies
@@concepts functions, maps, automation
@@text
Time to combine the module: you'll build `run.sh`, a tiny task runner in the spirit of `make` and `just`. A project defines tasks as Bash functions and declares which tasks depend on which; the runner works out the order, runs each task once, times it, and stops cleanly on failure.

## Design

```bash,norun
# tasks.sh: sourced by the runner
task_clean()   { rm -rf build; }
task_compile() { mkdir -p build && gcc -o build/app src/*.c; }
task_test()    { ./build/app --self-test; }
task_package() { tar czf app.tgz -C build .; }

deps[compile]="clean"
deps[test]="compile"
deps[package]="compile test"
```

The pieces you've learned map directly onto the problem:

| Need | Tool |
|---|---|
| tasks as named units | functions, discovered with `declare -F` |
| dependency graph | associative array `deps[task]="a b"` |
| run each task once, in order | depth-first **topological sort** with `visiting` / `done` marks |
| detect cycles | a task seen again while still `visiting` |
| call a task by name | `"task_$name"`, never `eval` |
| report | `printf` columns, `$SECONDS` or `EPOCHREALTIME` for timing |
| clean exit | `trap` on EXIT, exit statuses from each task |

## Topological sort in Bash

```bash,norun
visit() {
    local t=$1 d
    [[ ${state[$t]:-} == done ]] && return 0
    [[ ${state[$t]:-} == visiting ]] && { echo "cycle at $t" >&2; return 1; }
    state[$t]=visiting
    for d in ${deps[$t]:-}; do visit "$d" || return 1; done
    state[$t]=done
    order+=("$t")
}
```

Depth-first: visit the dependencies before appending the task itself. A task met again while it's still "visiting" means the graph has a cycle.

## Discovering tasks

`declare -F` lists all functions as `declare -f name`. Filter by the `task_` prefix to get the available tasks for `--list` and to reject unknown names with a helpful message.

## Timing

`EPOCHREALTIME` (Bash 5) holds the time with microseconds, e.g. `1762160000.123456`. Remove the dot to get an integer and subtract: `(( (${end/./} - ${start/./}) / 1000 ))` milliseconds. (Real timings vary between runs, so this lesson's exercises print fixed "durations" from a fake clock to keep the output testable.)

@@quiz
Why does depth-first visiting produce a valid run order?
* Each task is appended only after all of its dependencies have been appended
- Bash sorts arrays automatically
- Because tasks are alphabetical
- It doesn't; it needs a second pass
? Post-order traversal = topological order.

@@quiz
How does the runner detect a dependency cycle?
- It counts tasks
* A task is reached again while it's still marked "visiting"
- Bash raises an error
- Cycles can't happen
? Three states: unvisited, visiting, done.

@@quiz
Tasks A needs B, and B needs C. In what order must they run?
* C, then B, then A
- A, B, C
- B, C, A
- Any order
? A topological sort runs every dependency before the task that needs it.

@@drill
What's the name of an ordering where every task runs after all of its dependencies?
= topological order
= topological sort
= topological ordering
? It only exists if the dependencies have no cycle.

@@flashcards
topological sort :: order where dependencies come first
`declare -F` :: list function names
`"task_$name"` :: call a function by computed name
`EPOCHREALTIME` :: time with microseconds
`SECONDS` :: seconds since the shell started
visiting / done :: DFS marks for cycle detection

@@exercise Resolve the run order
Implement `resolve TASK...` that fills the global array `order` with the tasks to run (dependencies first, each once) or prints `error: cycle: a -> b -> a` and returns 1.

- keep a `stack` array of tasks being visited so the cycle message shows the path
- reject unknown tasks (no `task_NAME` function) with `error: unknown task: NAME`
- show the order for `package`, for `test docs` together, a cycle in a second graph, and an unknown task
@@starter
task_clean() { :; }; task_compile() { :; }; task_test() { :; }
task_docs() { :; }; task_package() { :; }; task_lint() { :; }
declare -A deps=([compile]="clean lint" [test]="compile" [package]="compile test docs" [docs]="clean")
echo TODO
@@expected
package: clean lint compile test docs package
test docs: clean lint compile test docs
error: cycle: a -> b -> c -> a
error: unknown task: deploy
@@requires
declare -F
local -n
@@hint
Check that a task exists with `declare -F "task_$t" >/dev/null`. In `visit`, push before visiting dependencies (`stack+=("$t")`) and pop afterwards (`unset 'stack[-1]'`). On a cycle, print the stack from the first occurrence of the task: loop over `stack` and start printing when you meet it. Use a nameref to the dependency map so `resolve` works with any graph: `resolve GRAPH_NAME TASK...`.
@@solution
task_clean() { :; }; task_compile() { :; }; task_test() { :; }
task_docs() { :; }; task_package() { :; }; task_lint() { :; }
declare -A deps=([compile]="clean lint" [test]="compile" [package]="compile test docs" [docs]="clean")

visit() {
    local t=$1 d s path="" found=""
    if ! declare -F "task_$t" >/dev/null; then
        echo "error: unknown task: $t"
        return 1
    fi
    [[ ${state[$t]:-} == done ]] && return 0
    if [[ ${state[$t]:-} == visiting ]]; then
        for s in "${stack[@]}"; do
            [[ $s == "$t" ]] && found=1
            [[ -n $found ]] && path+="$s -> "
        done
        echo "error: cycle: $path$t"
        return 1
    fi
    state[$t]=visiting
    stack+=("$t")
    for d in ${_graph[$t]:-}; do
        visit "$d" || return 1
    done
    unset 'stack[-1]'
    state[$t]=done
    order+=("$t")
}

resolve() {
    local -n _graph=$1
    shift
    declare -gA state=()
    stack=() order=()
    local t
    for t in "$@"; do visit "$t" || return 1; done
}

resolve deps package && echo "package: ${order[*]}"
resolve deps test docs && echo "test docs: ${order[*]}"

task_a() { :; }; task_b() { :; }; task_c() { :; }
declare -A cyclic=([a]="b" [b]="c" [c]="a")
resolve cyclic a || true
resolve deps deploy || true

@@exercise Run the tasks
Build the runner on top of `resolve` (included in the starter):

- `run_tasks TASK...` resolves the order, then calls each `task_NAME`, printing `==> NAME` before it and a result line after it with the duration from the fake clock `now_ms` (which advances by the task's cost)
- stop at the first failing task, mark the remaining ones as `skipped`, and return the failing task's status
- print a summary table with `printf` (task left-aligned in 9 columns, status, time right-aligned)
- `--list` prints the available tasks (from `declare -F`) with their dependencies, sorted
@@starter
clock=0
now_ms() { echo "$clock"; }
work() { (( clock += $1 )); }
task_clean() { work 30; }
task_lint() { work 120; echo "   2 warnings"; }
task_compile() { work 2400; }
task_test() { work 900; [[ ${FAIL_TEST:-} != 1 ]] || { echo "   1 test failed"; return 2; }; }
task_package() { work 300; }
declare -A deps=([compile]="clean lint" [test]="compile" [package]="compile test")
visit() {
    local t=$1 d
    [[ ${state[$t]:-} == done ]] && return 0
    [[ ${state[$t]:-} == visiting ]] && { echo "error: cycle at $t"; return 1; }
    state[$t]=visiting
    for d in ${deps[$t]:-}; do visit "$d" || return 1; done
    state[$t]=done
    order+=("$t")
}
resolve() { declare -gA state=(); order=(); local t; for t in "$@"; do visit "$t" || return 1; done; }
echo TODO
@@expected
available tasks:
  clean
  compile   <- clean lint
  lint
  package   <- compile test
  test      <- compile
==> clean
==> lint
   2 warnings
==> compile
==> test
==> package
TASK      STATUS      TIME
clean     ok          30ms
lint      ok         120ms
compile   ok        2400ms
test      ok         900ms
package   ok         300ms
total: 3750ms, result: success
==> clean
==> lint
   2 warnings
==> compile
==> test
   1 test failed
TASK      STATUS      TIME
clean     ok          30ms
lint      ok         120ms
compile   ok        2400ms
test      FAILED(2)  900ms
package   skipped        -
total: 3450ms, result: failed
@@requires
declare -F
"task_$
printf
@@hint
List tasks: `declare -F | awk '{print $3}' | grep '^task_' | sed 's/^task_//' | LC_ALL=C sort`. While running, store results in two associative arrays `result[$t]` and `took[$t]`. Once a task fails, set `failed=$rc` and give every later task `skipped`. Print the table rows with `printf '%-9s %-9s %6s\n'`, where the time column is `${took[$t]}ms` or `-`.
@@solution
clock=0
now_ms() { echo "$clock"; }
work() { (( clock += $1 )); }
task_clean() { work 30; }
task_lint() { work 120; echo "   2 warnings"; }
task_compile() { work 2400; }
task_test() { work 900; [[ ${FAIL_TEST:-} != 1 ]] || { echo "   1 test failed"; return 2; }; }
task_package() { work 300; }
declare -A deps=([compile]="clean lint" [test]="compile" [package]="compile test")
visit() {
    local t=$1 d
    [[ ${state[$t]:-} == done ]] && return 0
    [[ ${state[$t]:-} == visiting ]] && { echo "error: cycle at $t"; return 1; }
    state[$t]=visiting
    for d in ${deps[$t]:-}; do visit "$d" || return 1; done
    state[$t]=done
    order+=("$t")
}
resolve() { declare -gA state=(); order=(); local t; for t in "$@"; do visit "$t" || return 1; done; }

list_tasks() {
    local t
    echo "available tasks:"
    while read -r t; do
        if [[ -n ${deps[$t]:-} ]]; then
            printf '  %-9s <- %s\n' "$t" "${deps[$t]}"
        else
            printf '  %s\n' "$t"
        fi
    done < <(declare -F | awk '{print $3}' | grep '^task_' | sed 's/^task_//' | LC_ALL=C sort)
}

run_tasks() {
    resolve "$@" || return 1
    local -A result=() took=()
    local t start rc failed=0 total=0
    for t in "${order[@]}"; do
        if (( failed )); then
            result[$t]=skipped
            continue
        fi
        echo "==> $t"
        start=$(now_ms)
        "task_$t"
        rc=$?
        took[$t]=$(( $(now_ms) - start ))
        (( total += took[$t] ))
        if (( rc == 0 )); then
            result[$t]=ok
        else
            result[$t]="FAILED($rc)"
            failed=$rc
        fi
    done
    printf '%-9s %-9s %6s\n' TASK STATUS TIME
    local time
    for t in "${order[@]}"; do
        if [[ -v took[$t] ]]; then time="${took[$t]}ms"; else time=-; fi
        printf '%-9s %-9s %6s\n' "$t" "${result[$t]}" "$time"
    done
    if (( failed )); then
        echo "total: ${total}ms, result: failed"
    else
        echo "total: ${total}ms, result: success"
    fi
    return "$failed"
}

list_tasks
run_tasks package
clock=0
FAIL_TEST=1 run_tasks package || true
