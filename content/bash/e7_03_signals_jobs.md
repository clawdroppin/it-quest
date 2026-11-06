@@lesson e7-signals Signals, traps and Job Control
@@concepts processes, concurrency
@@text
A **signal** is a small asynchronous message the kernel delivers to a process: "stop", "you lost your terminal", "please exit". Scripts that run for more than a moment should handle them, so that Ctrl+C or `systemctl stop` doesn't leave half-written files and orphaned children behind.

## The signals you'll meet

| Signal | Number | Sent by | Default |
|---|---|---|---|
| SIGINT | 2 | Ctrl+C | terminate |
| SIGTERM | 15 | `kill PID`, systemd, Docker stop | terminate |
| SIGKILL | 9 | `kill -9` | terminate, **cannot be caught** |
| SIGHUP | 1 | terminal closed; by convention "reload config" for daemons | terminate |
| SIGQUIT | 3 | Ctrl+\ | core dump |
| SIGTSTP / SIGCONT | 20 / 18 | Ctrl+Z / `fg`, `bg` | stop / continue |
| SIGCHLD | 17 | a child exited | ignore |
| SIGUSR1 / SIGUSR2 | 10 / 12 | your own use | terminate |

A process killed by signal N exits with status **128 + N**: 130 after Ctrl+C, 143 after SIGTERM, 137 after SIGKILL (often an out-of-memory kill in containers).

## trap

```bash
cleanup() { echo "cleaning up"; }
trap cleanup EXIT                     # runs however the script ends (except SIGKILL)
trap 'echo "reloading"' HUP
trap 'echo "interrupted"; exit 130' INT TERM
trap - INT                            # restore the default
trap '' QUIT                          # ignore a signal
trap -p                               # list active traps
```

Bash also has pseudo-signals: `EXIT` (script ends), `ERR` (a command fails, like `set -e` but you choose what happens), `DEBUG` (before every command) and `RETURN` (a function or sourced file returns).

The standard pattern for temporary files:

```bash,norun
tmp=$(mktemp -d)
trap 'rm -rf -- "$tmp"' EXIT
```

## When does the trap run?

Bash only runs a trap **between commands**. If the script is waiting for a foreground `sleep 300`, the trap runs after `sleep` ends. To react immediately, run the long command in the background and `wait` for it: `wait` returns as soon as a trapped signal arrives.

```bash,norun
sleep 300 & child=$!
trap 'kill "$child" 2>/dev/null; exit 143' TERM
wait "$child"
```

## Job control

```bash,norun
long_task &            # run in the background; $! is its PID
jobs -l                # list jobs of this shell
fg %1                  # bring job 1 to the foreground
# Ctrl+Z, then:
bg                     # continue it in the background
wait                   # wait for all children
wait -n                # wait for the NEXT child to finish (Bash 4.3+)
wait -n -p pid         # ...and store which one (Bash 5.1+)
disown %1              # don't send it SIGHUP when the shell exits
nohup cmd > out.log 2>&1 &   # survive logout
```

`wait PID` returns that child's exit status, so you can collect results from parallel jobs.

## Limiting parallelism

```bash,norun
max=4
for host in "${hosts[@]}"; do
    check "$host" &
    while (( $(jobs -rp | wc -l) >= max )); do wait -n; done
done
wait
```

@@quiz
Which signal can a script NOT trap?
- SIGTERM
- SIGINT
* SIGKILL
- SIGHUP
? SIGKILL (and SIGSTOP) always act immediately; that's why `kill -9` is a last resort.

@@quiz
A container's process exits with status 137. What happened?
* It was killed with signal 9 (128 + 9), often by the out-of-memory killer
- It ran out of disk space
- It exited normally
- It received Ctrl+C
? 130 = SIGINT, 143 = SIGTERM, 137 = SIGKILL.

@@quiz
Why use `trap ... EXIT` for cleanup rather than putting `rm` at the end of the script?
- EXIT traps are faster
* The trap also runs when the script exits early, through `exit`, `set -e` or a caught signal
- rm doesn't work at the end of scripts
- It's required by POSIX
? One cleanup point for every way out.

@@drill
Which signal does Ctrl+C send?
= SIGINT
= INT
= 2
? trap 'cleanup' INT handles it.

@@flashcards
SIGINT (2) :: Ctrl+C
SIGTERM (15) :: polite stop (kill default)
SIGKILL (9) :: forced, uncatchable
SIGHUP (1) :: hang-up / reload config
128 + N :: exit status after signal N
`trap cmd EXIT` :: run on any exit
`trap cmd ERR` :: run when a command fails
`trap - SIG` :: restore default
`$!` :: PID of the last background job
`wait PID` :: wait and get its status
`wait -n` :: wait for the next job
`kill -0 PID` :: is the process alive?

@@exercise Cleanup that always runs
Write `run_job NAME MODE` that creates a temporary directory, sets `trap` to remove it and report on `EXIT`, and does its work in a **subshell** `( ... )` so each run has its own EXIT trap:

- `ok`: writes a file and exits 0
- `fail`: writes a file and exits 3 (as if a step failed)
- `term`: sends itself SIGTERM (`kill -TERM $BASHPID`) with a `TERM` trap that exits with 143

After each run, print the exit status and whether the temporary directory still exists.
@@starter
echo TODO
@@expected
[ok] working
[ok] cleanup: removing temp dir (status 0)
ok: status 0, temp dir exists: no
[fail] working
[fail] cleanup: removing temp dir (status 3)
fail: status 3, temp dir exists: no
[term] working
[term] caught SIGTERM
[term] cleanup: removing temp dir (status 143)
term: status 143, temp dir exists: no
@@requires
trap
EXIT
kill -TERM
@@hint
Create the directory in the parent and pass it in, so you can check it afterwards: `tmp=$(mktemp -d); ( trap 'echo "[$name] cleanup: removing temp dir (status $?)"; rm -rf -- "$tmp"' EXIT; ... ); status=$?`. Inside the EXIT trap, `$?` is the status the subshell is exiting with. `$BASHPID` is the subshell's own PID (`$$` would be the parent's).
@@solution
run_job() {
    local name=$1 mode=$2 tmp status
    tmp=$(mktemp -d)
    (
        trap 'echo "[$name] cleanup: removing temp dir (status $?)"; rm -rf -- "$tmp"' EXIT
        trap 'echo "[$name] caught SIGTERM"; exit 143' TERM
        echo "[$name] working"
        echo data > "$tmp/partial.txt"
        case $mode in
            ok) exit 0 ;;
            fail) exit 3 ;;
            term) kill -TERM "$BASHPID"; sleep 1 ;;
        esac
    )
    status=$?
    if [[ -d $tmp ]]; then exists=yes; else exists=no; fi
    echo "$name: status $status, temp dir exists: $exists"
}

run_job ok ok
run_job fail fail
run_job term term

@@exercise A parallel health check with a concurrency limit
Check eight hosts in parallel, at most three at a time:

- `check HOST` sleeps a moment and exits 0, or 1 for hosts containing `db02` or `cache02`
- start each check in the background, record `PID → host` in an associative array
- while three jobs are running, `wait -n -p pid` for one to finish and record its status
- at the end, wait for the rest the same way
- print results sorted by host name, the peak number of running jobs, and a summary
@@starter
hosts=(web01 web02 web03 db01 db02 cache01 cache02 mail01)
check() { sleep 0.2; [[ $1 != db02 && $1 != cache02 ]]; }
echo TODO
@@expected
cache01: up
cache02: DOWN
db01: up
db02: DOWN
mail01: up
web01: up
web02: up
web03: up
peak running: 3
summary: 6 up, 2 down
@@requires
wait -n -p
declare -A
&
@@hint
`reap() { local pid; wait -n -p pid; status[${host_of[$pid]}]=$?; unset 'host_of[$pid]'; }` (`$?` is read right after `wait`, so keep that line first). Track running jobs with `${#host_of[@]}`. Sort the result lines with `LC_ALL=C sort`.
@@solution
hosts=(web01 web02 web03 db01 db02 cache01 cache02 mail01)
check() { sleep 0.2; [[ $1 != db02 && $1 != cache02 ]]; }

declare -A host_of=() status=()
max=3 peak=0

reap() {
    local pid rc
    wait -n -p pid
    rc=$?
    status[${host_of[$pid]}]=$rc
    unset 'host_of[$pid]'
}

for h in "${hosts[@]}"; do
    while (( ${#host_of[@]} >= max )); do reap; done
    check "$h" &
    host_of[$!]=$h
    (( ${#host_of[@]} > peak )) && peak=${#host_of[@]}
done
while (( ${#host_of[@]} > 0 )); do reap; done

for h in "${!status[@]}"; do
    if (( status[$h] == 0 )); then echo "$h: up"; else echo "$h: DOWN"; fi
done | LC_ALL=C sort
up=0 down=0
for h in "${!status[@]}"; do
    if (( status[$h] == 0 )); then ((up++)); else ((down++)); fi
done
echo "peak running: $peak"
echo "summary: $up up, $down down"
