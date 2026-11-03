@@lesson e7-functions Functions in Depth: Return Values, Scope and Libraries
@@concepts functions, modules
@@text
Bash functions look like those in other languages, but they behave like **small commands**: they take arguments, write to stdout and stderr, and end with an exit status. Understanding that model avoids most function bugs.

## Three ways to get a result out

```bash
is_even() { (( $1 % 2 == 0 )); }          # 1. exit status: for yes/no questions
is_even 4 && echo "4 is even"

double() { echo $(( $1 * 2 )); }          # 2. stdout: capture with $( )
x=$(double 21); echo "$x"

split_host() {                            # 3. write to caller variables (namerefs)
    local -n _name=$2 _port=$3
    _name=${1%:*} _port=${1##*:}
}
split_host db01:5432 h p; echo "$h $p"
```

`return` only takes a number 0–255: it's a status, not a value. `$( )` runs the function in a **subshell**, so any variables it sets are lost, and it costs a fork. Namerefs (option 3) avoid both.

## Scope: dynamic, not lexical

Variables are global unless declared `local`. And `local` variables are visible to **functions called from** the declaring function (dynamic scope):

```bash
show() { echo "level=$level"; }
outer() { local level=inner; show; }
level=global
outer      # level=inner: show sees outer's local
show       # level=global
```

Make every helper variable `local` (and `local -i n` for integers, `local -a` / `local -A` for arrays). `declare -g` inside a function creates a global on purpose.

## Arguments

```bash
args() {
    echo "count=$# first=$1 all=[$*]"
    shift                 # drop $1
    for a in "$@"; do echo "  - $a"; done
}
args deploy "web 01" db01
```

`"$@"` keeps each argument separate, the function equivalent of `"${arr[@]}"`. `${@:2}` gives arguments from the second on, and `${!#}` the last one.

## Recursion, FUNCNAME and the call stack

```bash
fact() { (( $1 <= 1 )) && { echo 1; return; }; echo $(( $1 * $(fact $(( $1 - 1 ))) )); }
fact 10
where() { echo "${FUNCNAME[0]} called from ${FUNCNAME[1]:-main} (line ${BASH_LINENO[0]})"; }
helper() { where; }
helper
```

`FUNCNAME`, `BASH_SOURCE` and `BASH_LINENO` let you print stack traces in error handlers.

## Libraries with source

Put reusable functions in a file and load them with `source lib.sh` (or `.`). Good libraries:

- only define functions and constants: no side effects when sourced
- prefix names (`log::info`, `str_trim`) to avoid clashes
- guard against double loading: `[[ -n ${_LIB_LOG_LOADED:-} ]] && return; _LIB_LOG_LOADED=1`
- find their own directory with `"${BASH_SOURCE[0]%/*}"` to source siblings

```bash,norun
#!/usr/bin/env bash
here=$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)
source "$here/lib/log.sh"
```

A script can tell whether it's being run or sourced: `[[ ${BASH_SOURCE[0]} == "$0" ]] && main "$@"`. That makes its functions testable.

@@quiz
How should a function return a string?
- `return "text"`
* Print it to stdout and capture it with `$(f)`, or assign it through a nameref
- Set $? to the string
- Strings can't be returned
? return only accepts a status 0–255.

@@quiz
`f() { local x=1; g; }` and `g() { echo "$x"; }`. What does `f` print?
* 1, because Bash uses dynamic scope and g sees f's local
- Nothing: x is local to f
- An error
- 0
? Locals are visible to callees.

@@quiz
Why does `count=0; inc() { ((count++)); }; v=$(inc); echo $count` print 0?
- ((count++)) returns 0
* `$( )` runs inc in a subshell, so the change happens in a copy
- inc wasn't called
- count is local
? Call it directly (`inc`) to change the global.

@@flashcards
`return n` :: exit status 0–255
`$(f)` :: capture stdout (subshell)
`local` :: function-scoped variable
dynamic scope :: callees see caller's locals
`declare -g` :: global from inside a function
`"$@"` :: all arguments, separately
`$#` :: argument count
`shift` :: drop the first argument
`FUNCNAME` :: call stack of function names
`source` / `.` :: load a file into this shell
`BASH_SOURCE[0]` :: path of the current file

@@exercise A small string library
Write and test these functions without external commands (`sed`, `tr`, `awk`):

- `str_trim` prints its argument without leading and trailing whitespace (use `${s%%[![:space:]]*}` to find the leading part)
- `str_repeat TEXT N` prints TEXT N times
- `str_join SEP ITEMS...` joins items with a separator of any length
- `str_split STRING SEP ARRAY_NAME` fills an array through a nameref
- `str_contains HAYSTACK NEEDLE` returns a status only
@@starter
echo TODO
@@expected
trim: [hello world]
repeat: -=-=-=
join: web01, web02, db01
split: 3 parts: [a] [b c] [d]
contains: yes
not contains: no
@@requires
local -n
%%[![:space:]]*
@@hint
`str_trim() { local s=$1; s=${s#"${s%%[![:space:]]*}"}; s=${s%"${s##*[![:space:]]}"}; printf '%s' "$s"; }`. Join: print the first item, then `printf '%s%s' "$sep" "$item"` for the rest. Split: loop `while [[ $s == *"$sep"* ]]; do _arr+=("${s%%"$sep"*}"); s=${s#*"$sep"}; done; _arr+=("$s")`.
@@solution
str_trim() {
    local s=$1
    s=${s#"${s%%[![:space:]]*}"}
    s=${s%"${s##*[![:space:]]}"}
    printf '%s' "$s"
}

str_repeat() {
    local out="" i
    for (( i = 0; i < $2; i++ )); do out+=$1; done
    printf '%s' "$out"
}

str_join() {
    local sep=$1
    shift
    (( $# == 0 )) && return
    printf '%s' "$1"
    shift
    local item
    for item in "$@"; do printf '%s%s' "$sep" "$item"; done
}

str_split() {
    local s=$1 sep=$2
    local -n _parts=$3
    _parts=()
    while [[ $s == *"$sep"* ]]; do
        _parts+=("${s%%"$sep"*}")
        s=${s#*"$sep"}
    done
    _parts+=("$s")
}

str_contains() { [[ $1 == *"$2"* ]]; }

echo "trim: [$(str_trim $'  \t hello world \n ')]"
echo "repeat: $(str_repeat '-=' 3)"
echo "join: $(str_join ', ' web01 web02 db01)"
str_split "a::b c::d" "::" parts
printf 'split: %d parts:' "${#parts[@]}"
printf ' [%s]' "${parts[@]}"
echo
if str_contains "billing-api" "api"; then echo "contains: yes"; fi
if ! str_contains "billing-api" "web"; then echo "not contains: no"; fi

@@exercise Error handling with a stack trace
Build a tiny error-handling library:

- `die MESSAGE` prints `error: MESSAGE` and a stack trace (`  at FUNCTION (line N)` for each caller, using `FUNCNAME` and `BASH_LINENO`) to stdout, then returns 1 (a real script would `exit 1`)
- `require_number NAME VALUE` calls `die` unless VALUE is an integer
- `deploy` calls `parse_args`, which calls `require_number`

Show dynamic scope too: `deploy` sets `local env=prod`, and `banner` (called from `parse_args`) prints it.
@@starter
echo TODO
@@expected
banner: deploying to prod
replicas ok: 3
banner: deploying to prod
error: replicas must be a number, got 'three'
  at require_number
  at parse_args
  at deploy
  at main
deploy failed with status 1
@@requires
FUNCNAME
local env=prod
@@hint
In `die`, loop `for (( i = 1; i < ${#FUNCNAME[@]}; i++ ))` and print `${FUNCNAME[$i]}`. Code at the top level of a script shows up as `main` at the bottom of the stack. Line numbers depend on where your code sits, so this exercise only prints the names; in real scripts add `(line ${BASH_LINENO[$((i-1))]})`.
@@solution
die() {
    echo "error: $1"
    local i
    for (( i = 1; i < ${#FUNCNAME[@]}; i++ )); do
        echo "  at ${FUNCNAME[$i]}"
    done
    return 1
}

require_number() {
    [[ $2 =~ ^[0-9]+$ ]] || { die "$1 must be a number, got '$2'"; return; }
    echo "$1 ok: $2"
}

banner() { echo "banner: deploying to $env"; }

parse_args() {
    banner
    require_number replicas "$1"
}

deploy() {
    local env=prod
    parse_args "$@"
}

deploy 3
deploy three || echo "deploy failed with status $?"
