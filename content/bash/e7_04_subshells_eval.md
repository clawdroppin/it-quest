@@lesson e7-subshells Subshells, Exit Codes, pipefail and the Dangers of eval
@@concepts processes, errors
@@text
Many "Bash is weird" moments come from not knowing **which process** a piece of code runs in, and **which exit status** you're looking at.

## Subshells vs groups

```bash
x=1
( x=2; cd /tmp )          # subshell: a copy of the shell; changes vanish
echo "$x $PWD"
{ x=3; }                  # group: same shell; note the spaces and the ;
echo "$x"
```

Code runs in a subshell in: `( ... )`, `$( ... )`, every part of a pipeline (except the last with `shopt -s lastpipe`), `&` background jobs and `<( ... )`. Use a subshell on purpose to isolate `cd`, `set` options, traps or `IFS` changes.

`$$` is always the main script's PID; `$BASHPID` is the PID of the current (sub)shell; `$BASH_SUBSHELL` is the nesting depth.

## Exit statuses

- `$?` is the status of the **last** command, so read it immediately (or save it: `rc=$?`).
- `!` inverts a status. `cmd || true` swallows a failure on purpose.
- A pipeline's status is that of its **last** command, so `false | true` succeeds. With `set -o pipefail`, it is the last **non-zero** status.
- `PIPESTATUS` holds every status of the last pipeline: `"${PIPESTATUS[@]}"`.

```bash
false | true; echo "status=$? all=${PIPESTATUS[*]}"
set -o pipefail
false | true; echo "status=$? with pipefail"
set +o pipefail
```

## The set -e traps (a reminder)

`set -e` (exit on error) is **disabled** inside conditions (`if f`, `f && g`, `f || g`) and doesn't see failures inside `$( )` used as an argument (`echo "$(false)"`). `local x=$(cmd)` hides `cmd`'s status behind `local`'s own success: declare first, assign second:

```bash
f() { local out; out=$(false); echo "status of the assignment: $?"; }
f
```

## eval: building code from strings

`eval` joins its arguments into a string and runs it **as Bash code**. With any untrusted data in that string, an attacker can run commands:

```bash,norun
name='x; rm -rf ~'     # data from a file, a form, a web hook...
eval "echo $name"      # runs rm -rf ~  !!!
```

Almost every `eval` has a safer replacement:

| Instead of | Use |
|---|---|
| `eval "$var=value"` | `printf -v "$var" '%s' value` or `declare -g "$var=value"` |
| `eval "echo \$$name"` | `${!name}` |
| `eval "arr=($list)"` | `read -ra arr <<< "$list"` or `mapfile` |
| building a command string | building an **array**: `cmd=(rsync -a); [[ $dry ]] && cmd+=(--dry-run); "${cmd[@]}"` |

If you really must generate code (for example, to save variables to a file and source them later), quote each value with `printf '%q'` or `declare -p`, which produce shell-safe text.

@@quiz
Why does `echo hi | read word; echo "$word"` print an empty line?
- read can't read from pipes
* `read` runs in a subshell (part of the pipeline), so `word` is set in a copy
- echo doesn't add a newline
- word is a reserved name
? Use `read word <<< "hi"`.

@@quiz
With `set -o pipefail`, what is the status of `grep x missing.txt | sort`?
- 0, because sort succeeded
* grep's non-zero status (2), because pipefail reports the last failing command
- Always 1
- 141
? Without pipefail, sort's 0 would hide the error.

@@quiz
What's the safe replacement for `eval "$name=\$value"`?
- `$name=$value`
* `printf -v "$name" '%s' "$value"`
- `export $name`
- `let $name=$value`
? printf -v assigns without running the value as code (still validate the name itself).

@@flashcards
`( ... )` :: subshell: changes don't escape
`{ ...; }` :: group: same shell
`$$` vs `$BASHPID` :: script PID vs current subshell PID
`$?` :: last command's status
`PIPESTATUS` :: statuses of the last pipeline
`set -o pipefail` :: pipeline fails if any part fails
`shopt -s lastpipe` :: last pipeline part runs in the current shell
`local x; x=$(cmd)` :: keep cmd's exit status
`eval` :: run a string as code (dangerous)
`printf -v name` :: safe dynamic assignment
`declare -p var` :: shell-safe dump of a variable
`"${cmd[@]}"` :: run a command built as an array

@@exercise Where did that variable go?
Demonstrate each case and print what happens:

1. a change inside `( )` vs `{ }`
2. a counter incremented in `cmd | while read` vs `while read ... done < <(cmd)` vs with `shopt -s lastpipe` (lastpipe needs job control off, which is the case in scripts)
3. `$$` vs `$BASHPID` inside a subshell (print only whether they are equal)
4. `PIPESTATUS` of `true | false | true`, and the status with and without `pipefail`
5. `local out=$(false)` vs `local out; out=$(false)`
@@starter
lines() { printf '%s\n' a b c; }
echo TODO
@@expected
subshell: x=1, group: x=3
pipe: 0, process substitution: 3, lastpipe: 3
in subshell, $$ == $BASHPID: no; BASH_SUBSHELL=1
PIPESTATUS: 0 1 0
status without pipefail: 0, with pipefail: 1
local out=$(false): 0
local out; out=$(false): 1
@@requires
PIPESTATUS
pipefail
lastpipe
$BASHPID
@@hint
Save `PIPESTATUS` immediately: `true | false | true; ps=("${PIPESTATUS[@]}")`. For lastpipe, enable it, run the pipeline, then `shopt -u lastpipe`. In 5, print `$?` straight after each assignment inside a function.
@@solution
lines() { printf '%s\n' a b c; }

x=1
( x=2 )
sub=$x
{ x=3; }
echo "subshell: x=$sub, group: x=$x"

n=0; lines | while read -r _; do ((n++)); done; pipe=$n
n=0; while read -r _; do ((n++)); done < <(lines); procsub=$n
shopt -s lastpipe
n=0; lines | while read -r _; do ((n++)); done; last=$n
shopt -u lastpipe
echo "pipe: $pipe, process substitution: $procsub, lastpipe: $last"

( if [[ $$ == "$BASHPID" ]]; then same=yes; else same=no; fi
  echo "in subshell, \$\$ == \$BASHPID: $same; BASH_SUBSHELL=$BASH_SUBSHELL" )

true | false | true; ps=("${PIPESTATUS[@]}")
echo "PIPESTATUS: ${ps[*]}"
true | false | true; without=$?
set -o pipefail
true | false | true; with=$?
set +o pipefail
echo "status without pipefail: $without, with pipefail: $with"

f1() { local out=$(false); echo "local out=\$(false): $?"; }
f2() { local out; out=$(false); echo "local out; out=\$(false): $?"; }
f1
f2

@@exercise Replace eval with safe code
A deployment script used `eval` to load `key=value` settings and to build a command. Rewrite it safely:

- `load_settings` reads lines, skips comments and blanks, only accepts keys matching `^[A-Z_][A-Z0-9_]*$`, and assigns with `printf -v` (prefixing the keys with `CFG_`)
- show that the malicious value `$(echo HACKED)` is stored as plain text, not executed
- build the rsync command as an **array** (add `--dry-run` when `CFG_DRY_RUN` is `1`), and print it with `printf '%q '`
- save the settings to a file with `declare -p`, unset them, `source` the file back and print one value
@@starter
settings='# deploy settings
TARGET=web01:/srv/app
DRY_RUN=1
GREETING=$(echo HACKED)
bad-key=ignored
EXCLUDES=.git node_modules'
echo TODO
@@expected
skipped invalid key: bad-key
CFG_TARGET=web01:/srv/app
CFG_GREETING=$(echo HACKED)
command: rsync -a --delete --exclude .git --exclude node_modules --dry-run ./build/ web01:/srv/app
restored: web01:/srv/app
@@requires
printf -v
declare -p
"${cmd[@]}"
@@hint
`[[ -z $line || $line == \#* ]] && continue; key=${line%%=*}; value=${line#*=}`. For the excludes: `read -ra ex <<< "$CFG_EXCLUDES"; for e in "${ex[@]}"; do cmd+=(--exclude "$e"); done`. Since there's no rsync here, show the command with `printf '%q ' "${cmd[@]}"` and define `run() { "$@"; }` to show how you'd execute it (`run "${cmd[@]}"` stays in the code but is guarded by `[[ -n ${REALLY_RUN:-} ]]`).
@@solution
settings='# deploy settings
TARGET=web01:/srv/app
DRY_RUN=1
GREETING=$(echo HACKED)
bad-key=ignored
EXCLUDES=.git node_modules'

load_settings() {
    local line key value
    while IFS= read -r line; do
        [[ -z $line || $line == \#* ]] && continue
        key=${line%%=*}
        value=${line#*=}
        if [[ ! $key =~ ^[A-Z_][A-Z0-9_]*$ ]]; then
            echo "skipped invalid key: $key"
            continue
        fi
        printf -v "CFG_$key" '%s' "$value"
    done <<< "$1"
}

load_settings "$settings"
echo "CFG_TARGET=$CFG_TARGET"
echo "CFG_GREETING=$CFG_GREETING"

cmd=(rsync -a --delete)
read -ra excludes <<< "$CFG_EXCLUDES"
for e in "${excludes[@]}"; do cmd+=(--exclude "$e"); done
[[ $CFG_DRY_RUN == 1 ]] && cmd+=(--dry-run)
cmd+=(./build/ "$CFG_TARGET")
printf -v shown '%q ' "${cmd[@]}"
echo "command: ${shown% }"
[[ -n ${REALLY_RUN:-} ]] && "${cmd[@]}"

tmp=$(mktemp)
declare -p CFG_TARGET CFG_GREETING > "$tmp"
unset CFG_TARGET CFG_GREETING
source "$tmp"
rm -f -- "$tmp"
echo "restored: $CFG_TARGET"
