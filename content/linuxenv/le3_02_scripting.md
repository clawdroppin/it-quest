@@lesson le3-scripting Writing Reliable Shell Scripts
@@concepts functions, strings, errors, automation
@@lang bash
@@text
Once you repeat a command sequence three times, it's worth a script. Shell scripts are easy to start and easy to get subtly wrong: unquoted variables, ignored errors, scripts that break on file names with spaces. This lesson covers the habits that make Bash scripts reliable, plus arguments, arrays, exit codes and traps.

## A script skeleton

```bash,norun
#!/usr/bin/env bash
set -euo pipefail          # exit on error, on unset variables, and on failures inside pipes
IFS=$'\n\t'                # optional: safer word splitting

usage() { echo "usage: $0 [-n] [-d DIR] FILE..." >&2; exit 2; }

main() {
    local dry_run=0 dir=.
    while getopts ":nd:" opt; do
        case $opt in
            n) dry_run=1 ;;
            d) dir=$OPTARG ;;
            *) usage ;;
        esac
    done
    shift $((OPTIND - 1))
    (( $# > 0 )) || usage
    for f in "$@"; do echo "processing $f into $dir"; done
}
main "$@"
```

Make it executable with `chmod +x script.sh` and run it as `./script.sh`. Put personal scripts in `~/.local/bin` (usually on `PATH`).

## The rules that prevent most bugs

1. **Quote variables**: `"$file"`, `"$@"`. Unquoted `$file` splits on spaces and expands `*`.
2. `set -euo pipefail` makes failures stop the script instead of continuing silently. Know the caveats: commands in `if`, `&&`, `||` don't trigger `-e`.
3. Use `[[ ]]` for tests in Bash (`[[ $a == b* ]]`, `[[ -f $f ]]`) and `(( ))` for arithmetic.
4. `local` variables in functions.
5. Use `$(command)` instead of backticks.
6. Check scripts with **ShellCheck** (`shellcheck script.sh`): it catches most of the above.

## Exit codes

Every command returns 0 for success and 1–255 for failure; `$?` holds the last one. `exit 2` is a common convention for usage errors. `cmd || { echo "failed" >&2; exit 1; }` handles a failure explicitly. Messages for humans go to **stderr** (`>&2`) so that stdout stays clean for pipes.

## Arrays and parameter expansion

- Arrays: `files=(a.txt "b c.txt")`, `"${files[@]}"` (each element, quoted), `${#files[@]}` (count), `files+=(d.txt)`. Associative: `declare -A size; size[tokyo]=14`.
- Expansion: `${var:-default}`, `${var:?message}` (fail if unset), `${f%.txt}` (remove suffix), `${f##*/}` (basename), `${f%/*}` (dirname), `${s//a/b}` (replace all), `${s^^}` (upper case), `${#s}` (length).

## Cleanup with trap

`tmp=$(mktemp -d); trap 'rm -rf "$tmp"' EXIT` deletes temporary files however the script ends. `trap '...' INT TERM` handles Ctrl+C and `kill`.

## Here documents

`cat <<EOF > file` writes multi-line text with variables expanded; `<<'EOF'` (quoted) keeps `$` literal.

@@quiz
What does `set -o pipefail` change?
- Pipes run in parallel
* A pipeline fails if any command in it fails, not just the last
- Pipes are disabled
- Output is buffered
? Otherwise `false | true` succeeds.

@@quiz
Why write `"$@"` instead of `$*` or `$@` unquoted?
* It passes each argument exactly as given, even with spaces
- It's faster
- It joins all arguments with commas
- It's required by getopts
? Each argument stays one word.

@@quiz
`f=/home/aiko/photos/cat.jpg`. What is `${f##*/}`?
- /home/aiko/photos
* cat.jpg
- cat
- jpg
? Remove the longest prefix ending in /.

@@quiz
Where should error messages go?
- stdout
* stderr, with `>&2`
- A file in /tmp
- The exit code only
? Keep stdout clean for pipes.

@@quiz
Which tool catches common shell script mistakes automatically?
- bash -x
* ShellCheck
- chmod
- shfmt only
? bash -x traces execution, which helps debugging.

@@flashcards
shebang :: #!/usr/bin/env bash
set -e :: exit on error
set -u :: error on unset variable
pipefail :: failing pipe fails
"$@" :: all arguments, each quoted
$# :: number of arguments
$? :: last exit code
exit 2 :: usage error convention
>&2 :: write to stderr
getopts :: parse -x options
${var:-def} :: default value
${f%.txt} :: remove suffix
${f##*/} :: basename
${s^^} :: upper case
"${arr[@]}" :: every array element
trap ... EXIT :: cleanup on exit
mktemp -d :: safe temporary folder
ShellCheck :: shell linter
<<'EOF' :: heredoc without expansion

@@drill
Which variable holds the number of arguments passed to a script?
= $#
? $@ is the arguments themselves.

@@drill
What parameter expansion removes the `.txt` suffix from `$f`?
= ${f%.txt}
? %% removes the longest match.

@@exercise A getopts command line
Write `process` that accepts `-n` (dry run), `-o DIR` (output folder, default `out`), `-v` (verbose, may repeat), then one or more file names. It prints the settings and, for each file, either `would convert` or `convert` with the output path (`DIR/name.png` from `name.jpg`). Bad options or no files print a usage line to stderr and return 2. The test captures both outputs.
@@starter
process() {
    # Your code here
    :
}

process -v -o thumbs "beach day.jpg" cat.jpg
process -n -vv dog.jpg
process -x cat.jpg 2>&1; echo "exit $?"
process -n 2>&1; echo "exit $?"
@@expected
dry_run=0 verbose=1 out=thumbs files=2
  convert 'beach day.jpg' -> thumbs/beach day.png
  convert 'cat.jpg' -> thumbs/cat.png
dry_run=1 verbose=2 out=out files=1
  would convert 'dog.jpg' -> out/dog.png
usage: process [-n] [-v] [-o DIR] FILE...
exit 2
usage: process [-n] [-v] [-o DIR] FILE...
exit 2
@@requires
getopts
OPTIND
"$@"
>&2
@@hint
Inside a function, set `local OPTIND=1` before the `getopts ":nvo:" opt` loop. For `-vv`, getopts sees `v` twice; count it with `verbose=$((verbose + 1))`. After the loop `shift $((OPTIND - 1))`, then check `$# -gt 0`. Build the output name with `${f%.*}.png`.
@@solution
usage() { echo "usage: process [-n] [-v] [-o DIR] FILE..." >&2; }

process() {
    local OPTIND=1 opt dry=0 verbose=0 out=out f
    while getopts ":nvo:" opt; do
        case $opt in
            n) dry=1 ;;
            v) verbose=$((verbose + 1)) ;;
            o) out=$OPTARG ;;
            *) usage; return 2 ;;
        esac
    done
    shift $((OPTIND - 1))
    if [[ $# -eq 0 ]]; then usage; return 2; fi
    echo "dry_run=$dry verbose=$verbose out=$out files=$#"
    for f in "$@"; do
        if (( dry )); then
            echo "  would convert '$f' -> $out/${f%.*}.png"
        else
            echo "  convert '$f' -> $out/${f%.*}.png"
        fi
    done
}

process -v -o thumbs "beach day.jpg" cat.jpg
process -n -vv dog.jpg
process -x cat.jpg 2>&1; echo "exit $?"
process -n 2>&1; echo "exit $?"

@@exercise Read a config file safely
`config` holds `key = value` lines with comments, blank lines, quoted values and spaces around `=`. Parse it into an associative array **without** `source` or `eval` (which would run code hidden in the file), report lines that aren't valid, apply defaults with `${...:-}`, and print the result sorted. A line like `name=$(rm -rf ~)` must stay a harmless string.
@@starter
config='# backup settings
source_dir = /home/aiko/Documents
dest = "/media/aiko/Backup Drive"

keep_days=14
compress = yes
name=$(rm -rf ~)
this line is broken
'

declare -A conf
# Your code here
@@expected
line 8 ignored: this line is broken
compress   = [yes]
dest       = [/media/aiko/Backup Drive]
keep_days  = [14]
log_level  = [info]
name       = [$(rm -rf ~)]
source_dir = [/home/aiko/Documents]
@@requires
declare -A
while IFS= read -r
${
@@hint
Loop with `while IFS= read -r line; do ... done <<< "$config"`. Skip `[[ -z $line || $line == \#* ]]`. Match `[[ $line =~ ^[[:space:]]*([A-Za-z_]+)[[:space:]]*=[[:space:]]*(.*)$ ]]` and use `BASH_REMATCH[1]` and `[2]`; strip surrounding double quotes with `${v#\"}` and `${v%\"}`.
@@solution
config='# backup settings
source_dir = /home/aiko/Documents
dest = "/media/aiko/Backup Drive"

keep_days=14
compress = yes
name=$(rm -rf ~)
this line is broken
'

declare -A conf
n=0
while IFS= read -r line; do
    n=$((n + 1))
    [[ -z $line || $line == \#* ]] && continue
    if [[ $line =~ ^[[:space:]]*([A-Za-z_]+)[[:space:]]*=[[:space:]]*(.*)$ ]]; then
        value=${BASH_REMATCH[2]}
        value=${value#\"}
        value=${value%\"}
        conf[${BASH_REMATCH[1]}]=$value
    else
        echo "line $n ignored: $line"
    fi
done <<< "$config"

conf[keep_days]=${conf[keep_days]:-30}
conf[log_level]=${conf[log_level]:-info}
for key in $(printf '%s\n' "${!conf[@]}" | sort); do
    printf '%-10s = [%s]\n' "$key" "${conf[$key]}"
done
