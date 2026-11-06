@@lesson l1-functions Functions and Script Arguments
@@concepts functions
@@text
```bash
greet() {
    local name="$1"
    local times="${2:-1}"      # default: 1
    for (( i = 0; i < times; i++ )); do
        echo "Hello, $name!"
    done
}

greet ana
greet ben 2
```

- Arguments are `$1`, `$2`, ...; `$@` is all of them, `$#` how many.
- `local` keeps a variable inside the function. Without it, every variable is **global** and functions overwrite each other's variables.
- Call a function like a command: `greet ben 2`, no parentheses.

## Returning results

A function's **output** is its result; capture it with `$( )`. `return` only sets the **exit code** (0–255), for success or failure:

```bash
to_upper() {
    echo "${1^^}"
}
is_even() {
    (( $1 % 2 == 0 ))      # the exit code of the last command is the function's
}

name=$(to_upper "web01")
echo "$name"
if is_even 42; then echo "42 is even"; fi
```

Error messages go to **stderr** with `>&2`, so they don't end up in the captured result:

```bash
safe_divide() {
    if [[ $2 -eq 0 ]]; then
        echo "error: division by zero" >&2
        return 1
    fi
    echo $(( $1 / $2 ))
}
safe_divide 10 2
safe_divide 1 0 || echo "(it failed, as expected)"
```

## Script arguments

A script gets arguments the same way. `$0` is the script's own name:

```bash,norun
#!/usr/bin/env bash
# usage: ./backup.sh SOURCE DEST
if [[ $# -ne 2 ]]; then
    echo "usage: $0 SOURCE DEST" >&2
    exit 1
fi
src="$1"
dest="$2"
echo "Backing up $src to $dest"
```

Save it, make it executable with `chmod +x backup.sh`, run it with `./backup.sh /etc /backup`. The first line, the **shebang**, tells Linux which interpreter runs the file.

## Writing functions you can trust

Bash functions behave differently from functions in most languages, so a few rules save a lot of debugging:

- **Use `local` for every variable** a function sets. Without it, the variable is global and silently overwrites a variable of the same name in the caller. `local` also works for arrays: `local -a items=()`.
- **Return data on standard output, status with `return`.** `return` only carries a number from 0 to 255 (0 means success). To hand back a string, `echo` it and capture it with `result=$(my_function)`. Keep diagnostic messages on **standard error** (`echo "warning" >&2`) so they don't end up mixed into the captured result.
- **Command substitution runs in a subshell.** Changes a function makes to variables are lost when you call it as `$(fn)`. If a function must set several values, either print them on separate lines and read them, or use a **nameref**: `local -n out=$1; out=value` writes into the variable whose name the caller passed.
- **Check arguments early.** `(( $# == 2 )) || { echo "usage: copy_to SRC DEST" >&2; return 2; }` makes misuse obvious instead of failing halfway through.
- **Quote `"$@"`** to pass all arguments on unchanged, spaces included. `$*` and unquoted `$@` split them apart.
- **Name functions with verbs** (`backup_db`, `is_port_open`), and prefix helper functions in a shared library with a short namespace (`log_info`, `log_warn`) so they don't collide with commands.

```bash
log() { printf '%s %s\n' "[$1]" "$2" >&2; }

max_of() {
  local best=$1 n
  shift
  for n in "$@"; do
    (( n > best )) && best=$n
  done
  echo "$best"
}

is_even() { (( $1 % 2 == 0 )); }

biggest=$(max_of 4 17 9 12)
log info "computing"
echo "max: $biggest"
if is_even "$biggest"; then echo even; else echo odd; fi
```

`is_even` shows a useful pattern: a function whose **exit status** is the answer, so it reads naturally in `if` and `&&`. The last command's status becomes the function's status, so no explicit `return` is needed.

Finally, keep functions short. When a function grows past a screen, or needs data structures more complex than arrays and associative arrays, it's often a sign the job has outgrown Bash and would be clearer in Python.

@@quiz
How does a Bash function give back a text result?
* It prints it (echo), and the caller captures it with `$( )`
- `return "text"`
- It assigns to `$result`
- It can't
? `return` only sets a number (the exit code). Text results are printed.

@@quiz
Why use `local` inside functions?
- It makes functions faster
* Without it, variables are global and can overwrite variables elsewhere in the script
- It's required for `$1`
- It exports the variable
? Bash variables are global by default.

@@quiz
How does a Bash function hand back a string to its caller?
* It prints it, and the caller captures it with $(function)
- With return "text"
- By setting $RESULT automatically
- Functions can't return strings
? return only sets the exit status (0-255).

@@flashcards
name() { ... } :: define a function
$1, $2, $# :: arguments and how many there are
"$@" :: all arguments, each kept intact
local :: a variable visible only inside the function
return n :: exit status of the function (0 = success)
result=$(fn) :: capture what the function prints
>&2 :: send messages to standard error

@@exercise Size in MB
Write `to_mb` that takes a number of **bytes** and prints it in whole **megabytes** (1 MB = 1048576 bytes), rounded down.
@@starter
to_mb() {
    echo "TODO: not done yet" >&2
    return 1
}

@@tests
check "$(to_mb 1048576)" "1" "to_mb 1048576"
check "$(to_mb 5242880)" "5" "to_mb 5242880"
check "$(to_mb 3000000)" "2" "to_mb 3000000"
check "$(to_mb 0)" "0" "to_mb 0"
@@hint
`echo $(( $1 / 1048576 ))`
@@solution
to_mb() {
    echo $(( $1 / 1048576 ))
}

@@exercise Validate an IP
Write `is_ipv4` that succeeds (exit code 0) for an IPv4 address with four numbers 0–255 separated by dots, and fails (non-zero) otherwise. Print nothing.
@@starter
is_ipv4() {
    echo "TODO: not done yet" >&2
    return 1
}

@@tests
for ip in 10.0.0.4 192.168.1.255 0.0.0.0; do
    is_ipv4 "$ip"; check "$?" "0" "is_ipv4 $ip should succeed"
done
for ip in 256.1.1.1 10.0.0 10.0.0.4.5 abc 1.2.3.x ""; do
    is_ipv4 "$ip" 2>/dev/null; check "$([[ $? -ne 0 ]] && echo fails)" "fails" "is_ipv4 '$ip' should fail"
done
@@hint
First the shape: `[[ $1 =~ ^([0-9]{1,3})\.([0-9]{1,3})\.([0-9]{1,3})\.([0-9]{1,3})$ ]] || return 1`. The groups land in `BASH_REMATCH[1]` to `[4]`; loop over them and `return 1` if one is over 255.
@@solution
is_ipv4() {
    [[ $1 =~ ^([0-9]{1,3})\.([0-9]{1,3})\.([0-9]{1,3})\.([0-9]{1,3})$ ]] || return 1
    local i
    for i in 1 2 3 4; do
        (( BASH_REMATCH[i] <= 255 )) || return 1
    done
    return 0
}
