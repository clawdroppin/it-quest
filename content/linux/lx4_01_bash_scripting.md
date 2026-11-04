@@lesson lx4-scripting Shell Scripting for Administrators
@@lang bash
@@text
Linux+ domain 3 (scripting, containers and automation, about 19%) expects you to read and write Bash scripts: variables, tests, loops, functions, exit codes and safe habits. Most admin automation starts as a script, and a careless one can delete the wrong directory, so the safety habits matter as much as the syntax.

## Script skeleton

```bash,norun
#!/usr/bin/env bash
# backup-etc.sh: archive /etc to /backup with a date stamp
set -euo pipefail                 # stop on errors, unset variables, failed pipes

readonly DEST=/backup
stamp=$(date +%F)                 # command substitution: 2026-11-04

main() {
    mkdir -p "$DEST"
    tar -czf "$DEST/etc-$stamp.tar.gz" /etc
    echo "saved $DEST/etc-$stamp.tar.gz"
}

main "$@"
```

Make it executable: `chmod +x backup-etc.sh`, run `./backup-etc.sh`. The **shebang** picks the interpreter.

## Variables and quoting

- `name=value` (no spaces around `=`), use `"$name"`.
- **Always double-quote** expansions: `rm -rf "$dir"`; unquoted, a space or an empty variable can turn into `rm -rf /`.
- `'single quotes'` are literal; `"double quotes"` expand variables.
- `${var:-default}` (default if unset/empty), `${var:?message}` (fail if unset), `${#var}` length, `${var%.log}` remove suffix, `${var#*/}` remove prefix, `${var//a/b}` replace all, `${var,,}` lowercase.
- Arrays: `files=(a b c)`, `"${files[@]}"`, `${#files[@]}`; associative: `declare -A count`.
- Special: `$0` script name, `$1..$9` arguments, `$#` count, `"$@"` all arguments (quoted individually), `$?` last exit status, `$$` PID.
- `export VAR` passes it to child processes; `source file` / `. file` runs a file in the current shell.

## Tests and conditionals

```bash,norun
if [[ -f /etc/nginx/nginx.conf ]]; then echo "exists"; fi
[[ -d $dir ]]  [[ -e $path ]]  [[ -r $f ]]  [[ -x $f ]]  [[ -s $f ]] (non-empty)
[[ $a == "$b" ]]  [[ $a != b* ]]  [[ $s =~ ^[0-9]+$ ]]  [[ -z $s ]] (empty)  [[ -n $s ]]
(( count > 5 ))   [[ $n -gt 5 ]]   -eq -ne -lt -le -ge
case $1 in
    start|up) start_service ;;
    stop)     stop_service ;;
    *)        echo "usage: $0 {start|stop}" >&2; exit 2 ;;
esac
```

## Loops

```bash,norun
for host in web01 web02 db01; do ping -c1 -W1 "$host" >/dev/null && echo "$host up"; done
for f in /var/log/*.log; do gzip "$f"; done
for ((i = 1; i <= 5; i++)); do echo "$i"; done
while read -r user uid; do echo "$user $uid"; done < users.txt
until systemctl is-active --quiet nginx; do sleep 2; done
```

`break` and `continue` work as usual. Read files line by line with `while read -r`, not `for line in $(cat file)` (which splits on spaces).

## Functions, exit codes and errors

- `name() { ...; }`, local variables with `local`, return a status with `return N` (0–255), output with `echo`.
- `exit 0` success, non-zero failure. Callers and cron check it.
- `trap cleanup EXIT` runs cleanup however the script ends; `trap 'echo "failed at line $LINENO"' ERR`.
- Send errors to stderr: `echo "error" >&2`.
- `shellcheck script.sh` finds common bugs.

## Arithmetic and text

`$(( a + b ))`, `(( i++ ))`, `bc` for decimals; `printf "%-10s %5d\n" "$name" "$n"` for aligned output; `date +%F_%H%M` for timestamps; `mktemp` for safe temp files.

## Good practice

- `set -euo pipefail` and quoting.
- Check required commands and arguments at the top; print usage.
- Use absolute paths in cron jobs.
- Log what the script does; make it **idempotent** (safe to run twice).
- Never hard-code passwords; read them from a protected file or a secrets manager.
- Test on a non-production system; use `echo` dry-run modes for destructive steps.

@@quiz
What does `set -u` do in a script?
- Runs the script as root
* Treats use of an unset variable as an error
- Prints every command before running it
- Ignores errors in pipes
? `-e` exits on errors; `-o pipefail` catches failures inside pipes; `-x` traces.

@@quiz
Why quote `rm -rf "$target/"`?
* If $target is empty or contains spaces, the unquoted version can delete the wrong path (even /)
- Quotes make rm faster
- rm requires quotes
- To allow wildcards
? Combine with `${target:?}` to fail on empty.

@@quiz
Which expansion gives all script arguments, each as a separate quoted word?
- `$*`
* `"$@"`
- `$#`
- `$0`
? `$#` is the count.

@@quiz
How should a script read a file line by line, preserving spaces?
- `for line in $(cat file)`
* `while read -r line; do ...; done < file`
- `cat file | for line`
- `read file`
? for splits on all whitespace.

@@quiz
Which line makes a cleanup function run whenever the script exits?
- `exit cleanup`
* `trap cleanup EXIT`
- `cleanup &`
- `on_exit cleanup`
? trap also handles signals like INT and TERM.

@@flashcards
#!/usr/bin/env bash :: shebang
set -euo pipefail :: strict mode
"$var" :: always quote expansions
${var:-default} :: default value
${var:?msg} :: fail if unset or empty
${var%.log} :: remove suffix
${var//a/b} :: replace all
"$@" :: all arguments, separately quoted
$# :: number of arguments
$? :: last exit status
[[ -f file ]] :: regular file exists
[[ -z $s ]] :: string is empty
(( n > 5 )) :: arithmetic test
case ... esac :: pattern branching
while read -r :: line-by-line reading
local :: function-scoped variable
trap ... EXIT :: cleanup on exit
>&2 :: write to stderr
shellcheck :: script linter

@@drill
Which special variable holds the exit status of the last command?
= $?
? 0 means success.

@@drill
Which test checks that a path is a directory?
= -d
= [[ -d path ]]
? -f checks for a regular file.

@@exercise Health-check script with exit codes
Write a function `check_disk` that takes a mount point and a usage percentage and returns 0 (OK, below 80), 1 (WARNING, 80–89) or 2 (CRITICAL, 90+), printing a matching line. Run it for every line of `usage`, track the **worst** status, and finish with `overall: OK|WARNING|CRITICAL` and that status as the last line (`exit code N`). Don't actually call `exit`.
@@starter
usage='/ 72
/var 85
/data 41
/backup 93'

check_disk() {
    echo "TODO"
}
@@expected
OK / at 72%
WARNING /var at 85%
OK /data at 41%
CRITICAL /backup at 93%
overall: CRITICAL
exit code 2
@@requires
return 2
worst
@@hint
Inside the loop: `check_disk "$mount" "$pct"; rc=$?; (( rc > worst )) && worst=$rc`. Map numbers to names with an array `names=(OK WARNING CRITICAL)`.
@@solution
usage='/ 72
/var 85
/data 41
/backup 93'

check_disk() {
    local mount=$1 pct=$2
    if (( pct >= 90 )); then
        echo "CRITICAL $mount at $pct%"
        return 2
    elif (( pct >= 80 )); then
        echo "WARNING $mount at $pct%"
        return 1
    fi
    echo "OK $mount at $pct%"
    return 0
}

names=(OK WARNING CRITICAL)
worst=0
while read -r mount pct; do
    check_disk "$mount" "$pct" && rc=0 || rc=$?
    (( rc > worst )) && worst=$rc
done <<< "$usage"
echo "overall: ${names[worst]}"
echo "exit code $worst"

@@exercise Rename files safely with parameter expansion
The `files` variable lists file names from a camera export. Print the `mv` commands (dry run) that rename each file to lowercase, replace spaces with underscores, and change the `.jpeg` extension to `.jpg`. Skip files whose new name equals the old one, and print `conflict: name` instead of a command if two files would get the same new name.
@@starter
files='IMG 0001.JPEG
img_0002.jpg
Holiday Photo.jpeg
IMG 0001.jpeg
notes.txt'

echo "$files"
@@expected
mv -- "IMG 0001.JPEG" "img_0001.jpg"
mv -- "Holiday Photo.jpeg" "holiday_photo.jpg"
conflict: img_0001.jpg
@@requires
${new,,}
${new// /_}
declare -A
@@hint
Lowercase first: `new=${f,,}`, then `new=${new// /_}`, then `[[ $new == *.jpeg ]] && new=${new%.jpeg}.jpg`. Keep `declare -A seen` of new names.
@@solution
files='IMG 0001.JPEG
img_0002.jpg
Holiday Photo.jpeg
IMG 0001.jpeg
notes.txt'

declare -A seen
while IFS= read -r f; do
    new=$f
    new=${new,,}
    new=${new// /_}
    [[ $new == *.jpeg ]] && new=${new%.jpeg}.jpg
    if [[ -n ${seen[$new]+set} ]]; then
        echo "conflict: $new"
        continue
    fi
    seen[$new]=1
    [[ $new == "$f" ]] && continue
    echo "mv -- \"$f\" \"$new\""
done <<< "$files"
