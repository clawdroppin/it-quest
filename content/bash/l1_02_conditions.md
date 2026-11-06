@@lesson l1-conditions Exit Codes, if and case
@@concepts conditions, errors
@@text
## Every command answers yes or no

When a command finishes it returns an **exit code**: `0` means success, anything else is failure. `$?` holds the last one.

```bash
ls /etc > /dev/null
echo "ls /etc: $?"
ls /does-not-exist 2> /dev/null
echo "ls /does-not-exist: $?"
```

(`> /dev/null` throws output away; `2>` redirects error messages.)

`if` simply runs a command and checks its exit code:

```bash
if grep -q "root" /etc/passwd; then
    echo "root exists"
fi
```

## Tests with [[ ]]

`[[ ... ]]` is a command that tests things:

```bash
cpu=91
if [[ $cpu -ge 95 ]]; then
    echo "CRITICAL"
elif [[ $cpu -ge 80 ]]; then
    echo "WARNING"
else
    echo "OK"
fi
```

| Numbers | Text | Files |
|---|---|---|
| `-eq -ne` equal / not | `==` `!=` | `-e` exists |
| `-gt -ge` greater (or equal) | `-z` empty, `-n` not empty | `-f` regular file |
| `-lt -le` less (or equal) | `== web*` wildcard | `-d` directory |
| | `=~` regex | `-r -w -x` readable, writable, executable |

Combine with `&&` (and), `||` (or), `!` (not). Spaces inside the brackets are required: `[[$a]]` is an error.

(You'll also see the older `[ ... ]` in scripts; `[[ ]]` is safer and supports `&&`, `==` patterns and `=~` regex.)

## Short-circuit one-liners

```bash
mkdir -p /tmp/itq-x && echo "created"
[[ -d /nope ]] || echo "no such folder"
rm -r /tmp/itq-x
```

`a && b` runs b only if a succeeded; `a || b` only if a failed.

## case

```bash
answer="Yes"
case "${answer,,}" in
    y|yes) echo "continuing" ;;
    n|no)  echo "stopping" ;;
    *)     echo "please answer yes or no" ;;
esac
```

Each pattern ends with `)`, each branch with `;;`, and `esac` is `case` backwards (as `fi` is `if`).

## [ ], [[ ]] and (( )): which test to use

Bash has three ways to test things, and mixing them up causes subtle bugs:

- **`[ ... ]`** is the old POSIX `test` command. It's an ordinary command, so every variable must be quoted (`[ "$name" = "ana" ]`), `<` and `>` are redirections rather than comparisons, and `&&` can't go inside it. Use it only in scripts that must run under plain `sh`.
- **`[[ ... ]]`** is Bash's own test. Variables inside don't need quoting against word splitting, `&&`, `||` and `!` work inside, `==` matches **glob patterns** (`[[ $file == *.log ]]`), and `=~` matches **regular expressions** (`[[ $ip =~ ^[0-9]+\.[0-9]+ ]]`, with groups in `BASH_REMATCH`). Quote the right-hand side when you want a literal match instead of a pattern.
- **`(( ... ))`** is for **numbers**: `(( count > 10 ))`, `(( n % 2 == 0 ))`. Inside it, variables don't need `$`, and the comparison operators are the familiar `<`, `>`, `==`. Its exit status is success when the result is non-zero.

A classic mistake is comparing numbers as strings: `[[ 10 < 9 ]]` is **true**, because `<` inside `[[ ]]` compares text, and "1" sorts before "9". Use `(( 10 < 9 ))` or `[[ 10 -lt 9 ]]` for numbers.

```bash
file=report.log
[[ $file == *.log ]] && echo "a log file"
[[ 10 < 9 ]] && echo "as text, 10 sorts before 9"
(( 10 < 9 )) || echo "as numbers, 10 is not less than 9"
if [[ "192.168.1.20" =~ ^([0-9]+)\.([0-9]+) ]]; then echo "first octet ${BASH_REMATCH[1]}"; fi
```

@@quiz
What does an exit code of `0` mean?
* success
- failure
- the command was not found
- it depends on the command
? 0 is success; 1–255 are different kinds of failure. 127 means "command not found".

@@quiz
Which test checks whether `/etc/nginx` is a **directory**?
- `[[ -f /etc/nginx ]]`
* `[[ -d /etc/nginx ]]`
- `[[ -e /etc/nginx/ ]]` only
- `[[ /etc/nginx == dir ]]`
? `-d` is "is a directory", `-f` "is a regular file", `-e` "exists".

@@exercise Disk alert
`usage` holds a disk usage percentage. Print `CRITICAL` if it's 90 or more, `WARNING` if it's 75 or more, otherwise `OK`.
@@starter
usage=82

@@expected
WARNING
@@hint
`if [[ $usage -ge 90 ]]; then ... elif [[ $usage -ge 75 ]]; then ... else ... fi`
@@solution
usage=82
if [[ $usage -ge 90 ]]; then
    echo "CRITICAL"
elif [[ $usage -ge 75 ]]; then
    echo "WARNING"
else
    echo "OK"
fi

@@exercise Port to service
Write a function `service_for` that prints the service for a port: `22` ssh, `80` http, `443` https, `3306` mysql, anything else `unknown`. Use `case`.
@@starter
service_for() {
    echo "TODO: not done yet" >&2
    return 1
}

@@tests
check "$(service_for 22)" "ssh" "service_for 22"
check "$(service_for 443)" "https" "service_for 443"
check "$(service_for 3306)" "mysql" "service_for 3306"
check "$(service_for 8080)" "unknown" "service_for 8080"
@@hint
`case "$1" in 22) echo ssh ;; ... *) echo unknown ;; esac`. Inside a function, `$1` is its first argument.
@@solution
service_for() {
    case "$1" in
        22)   echo "ssh" ;;
        80)   echo "http" ;;
        443)  echo "https" ;;
        3306) echo "mysql" ;;
        *)    echo "unknown" ;;
    esac
}

@@exercise Does it exist?
Write `describe` that prints `dir` if its argument is a directory, `file` if it's a regular file, and `missing` otherwise.
@@starter
describe() {
    echo "TODO: not done yet" >&2
    return 1
}

@@tests
mkdir -p /tmp/itq-describe && touch /tmp/itq-describe/f.txt
check "$(describe /tmp/itq-describe)" "dir" "describe /tmp/itq-describe"
check "$(describe /tmp/itq-describe/f.txt)" "file" "describe a file"
check "$(describe /tmp/itq-describe/nope)" "missing" "describe a missing path"
rm -rf /tmp/itq-describe
@@hint
`if [[ -d "$1" ]]; then echo dir; elif [[ -f "$1" ]]; then echo file; else echo missing; fi`
@@solution
describe() {
    if [[ -d "$1" ]]; then
        echo "dir"
    elif [[ -f "$1" ]]; then
        echo "file"
    else
        echo "missing"
    fi
}
