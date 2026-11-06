@@lesson l1-variables Variables, Quoting and Arithmetic
@@concepts variables, strings, numbers
@@text
```bash
server="web01"          # NO spaces around =
port=8080
echo "$server listens on $port"
echo "${server}-backup"  # braces when text follows the name
```

`server = "web01"` (with spaces) would try to run a *command* called `server`. This is the most common Bash mistake.

## Quoting: the second most common mistake

```bash
file="my report.txt"
echo "$file"     # double quotes: variables expand, spaces are kept
echo '$file'     # single quotes: literally $file
echo $file       # no quotes: split into two words "my" and "report.txt"!
```

**Always put variables in double quotes**: `"$file"`. Unquoted, a name with a space becomes two arguments, and `rm $file` would try to delete `my` and `report.txt`.

## Command substitution

`$( ... )` runs a command and uses its output:

```bash
today=$(date +%F)
files=$(ls /etc | wc -l)
echo "Backup for $today: /etc has $files entries"
```

## Arithmetic

Bash variables are text, but `$(( ... ))` does whole-number math:

```bash
used=91
total=128
echo $(( total - used ))
echo $(( used * 100 / total ))    # integer division: 71, not 71.09
count=5
(( count++ ))
echo $count
```

For decimals, use `awk` or `bc`: `awk 'BEGIN { printf "%.1f\n", 91 / 128 * 100 }'`.

## String tricks

```bash
path="/var/log/nginx/access.log"
echo "${#path}"            # length
echo "${path##*/}"         # access.log      (remove the longest */ from the front)
echo "${path%/*}"          # /var/log/nginx  (remove the shortest /* from the end)
echo "${path%.log}.gz"     # replace the extension
name="WEB01"
echo "${name,,}"           # web01  (lowercase; ^^ for uppercase)
echo "${missing:-default}" # a default when the variable is empty
```

`basename` and `dirname` do the same as `##*/` and `%/*`, and are easier to read.

## Environment variables

```bash
echo "$HOME"
echo "$USER $SHELL"
export APP_ENV=production   # export: child programs see it too
```

`PATH` is the list of folders searched for commands: `echo "$PATH" | tr ':' '\n'`.

## Naming, scope and defaults

A few habits keep variables from biting you in longer scripts:

- **Lower case for your own variables**, upper case for environment variables and constants. `PATH`, `HOME` and `USER` are environment variables; if you name your own variable `PATH`, you've just broken every command after it.
- **No spaces around `=`**. `name = ana` runs a command called `name` with the arguments `=` and `ana`.
- **Variables are global by default**, even when set inside a function. Use `local` inside functions (covered in the functions lesson) so they don't leak.
- **`readonly`** stops a value being changed later: `readonly CONFIG_DIR=/etc/myapp`.
- **Default values** come from parameter expansion: `${name:-guest}` uses `guest` when `name` is unset or empty, `${name:=guest}` also assigns it, and `${name:?name is required}` stops the script with that message. These make scripts friendly to override from the environment: `LOG_LEVEL=${LOG_LEVEL:-info}`.
- **`set -u`** makes using an unset variable an error instead of an empty string, which catches typos like `$fiel` for `$file`. Combine it with the `:-` default for variables that are allowed to be missing.

```bash
unset nickname
echo "Hello, ${nickname:-friend}"
retries=${RETRIES:-3}
echo "Will retry $retries times"
readonly app=inventory
echo "App: $app, home: $HOME"
```

To see what's defined, `declare -p name` prints a variable with its attributes, `env` lists exported (environment) variables, and `set | less` shows everything including functions. When a script behaves differently from your terminal, check whether a variable was **exported**: child processes only see exported variables.

@@quiz
What's wrong with `name = "web01"`?
* The spaces: Bash runs a command called `name` with the arguments `=` and `web01`
- Nothing
- Strings need single quotes
- Variables must be uppercase
? Assignments must be written without spaces: `name="web01"`.

@@quiz
`f="old logs.txt"`, then `rm $f`. What does `rm` receive?
- one argument: `old logs.txt`
* two arguments: `old` and `logs.txt`
- nothing
- `$f` literally
? Unquoted variables are split on spaces. Use `rm "$f"`.

@@quiz
What does `echo $(( 7 / 2 ))` print?
- 3.5
* 3
- 4
- an error
? Bash arithmetic is integer-only and truncates.

@@drill
Write the syntax that captures a command's output into a variable called `now` from `date`.
= now=$(date)
= now="$(date)"
? No spaces around = in Bash assignments.

@@flashcards
name=value :: assign (no spaces around =)
"$name" :: use a variable, quoted
$(command) :: command substitution: capture output
$(( 2 + 3 )) :: arithmetic
${name:-default} :: default when unset or empty
export NAME :: pass a variable to child processes
readonly :: a variable that can't change

@@exercise Disk percentage
`used` and `total` are given in GB. Print `Disk is 71% full` using integer arithmetic (don't type 71).
@@starter
used=91
total=128

@@expected
Disk is 71% full
@@hint
`echo "Disk is $(( used * 100 / total ))% full"`
@@solution
used=91
total=128
echo "Disk is $(( used * 100 / total ))% full"

@@exercise Rename for backup
`file` holds a log path. Print the **file name** without its folder, then the backup path: the same path with `.log` replaced by `.1.gz`.
@@starter
file="/var/log/app/server.log"

@@expected
server.log
/var/log/app/server.1.gz
@@hint
`${file##*/}` for the name, and `${file%.log}.1.gz` for the backup path.
@@solution
file="/var/log/app/server.log"
echo "${file##*/}"
echo "${file%.log}.1.gz"
