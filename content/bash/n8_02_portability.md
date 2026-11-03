@@lesson n8-portability Portability: POSIX sh, dash, BusyBox and macOS
@@concepts strings, processes
@@text
"Bash" isn't always available, and isn't always the same version:

- **Debian/Ubuntu** run `/bin/sh` as **dash**, a small, fast POSIX shell without Bash features. `#!/bin/sh` scripts with arrays fail there.
- **Alpine** containers ship **BusyBox ash**; Bash is an extra package.
- **macOS** ships Bash **3.2** (2007, for licensing reasons) and uses zsh as the default shell. No associative arrays, no `mapfile`, no `${var,,}`.
- Embedded devices and initramfs environments often have only BusyBox.

So decide per script: either `#!/usr/bin/env bash` and use Bash freely, or `#!/bin/sh` and stay strictly POSIX.

## Common bashisms and their POSIX replacements

| Bash | POSIX sh |
|---|---|
| `[[ $a == b* ]]` | `case $a in b*) ... ;; esac` |
| `[[ -n $x && -f $f ]]` | `[ -n "$x" ] && [ -f "$f" ]` (quote everything) |
| `[[ $s =~ re ]]` | `printf '%s\n' "$s" \| grep -Eq 're'` or `expr` |
| `arr=(a b c)` | `set -- a b c` (the one array POSIX has) or newline-separated strings |
| `${s,,}` / `${s^^}` | `printf '%s' "$s" \| tr '[:upper:]' '[:lower:]'` |
| `${s/a/b}` | `printf '%s' "$s" \| sed 's/a/b/'` |
| `${s:1:3}` | `printf '%s' "$s" \| cut -c2-4` |
| `function f { }` | `f() { }` |
| `local x` | not POSIX (but dash, ash and ksh support it) |
| `echo -e` / `echo -n` | `printf` |
| `$'\n'` | a literal newline inside quotes (`nl='` + Enter + `'`) |
| `source file` | `. ./file` |
| `&>` | `> file 2>&1` |
| `<(cmd)` / `<<<` | temp files / here-docs / pipes |
| `((i++))` | `i=$((i + 1))` (`$(( ))` **is** POSIX) |
| `read -a`, `mapfile` | `while IFS= read -r line` |
| `{1..5}` | `seq 1 5` (not POSIX either, but common) or a `while` loop |

## Testing for portability

- Run the script with `dash` or `busybox sh` (`docker run --rm -v "$PWD":/w -w /w alpine sh script.sh`).
- ShellCheck respects the shebang: with `#!/bin/sh` it flags every bashism (`SC3010: In POSIX sh, [[ ]] is undefined`).
- `bash --posix` or `set -o posix` makes Bash more POSIX-like but **still accepts** most Bash features, so it isn't a real test.
- `checkbashisms` (from Debian's devscripts) scans scripts too.

## External tool differences: GNU vs BSD

Portability isn't only the shell. macOS and the BSDs use BSD tools:

| Task | GNU (Linux) | BSD (macOS) |
|---|---|---|
| in-place sed | `sed -i 's/a/b/' f` | `sed -i '' 's/a/b/' f` |
| date arithmetic | `date -d '+1 day'` | `date -v+1d` |
| file size | `stat -c %s f` | `stat -f %z f` |
| extended regex sed | `sed -E` (also `-r`) | `sed -E` |
| readlink -f | yes | macOS 12.3+ only |

Portable choices: `sed -i.bak ... && rm f.bak`, `wc -c < f` for sizes, and `perl` or `awk` for anything complex.

## Checking versions at runtime

```bash
if (( BASH_VERSINFO[0] < 4 )); then
    echo "this script needs Bash 4+ (found $BASH_VERSION); on macOS: brew install bash" >&2
fi
echo "major version: ${BASH_VERSINFO[0]}"
```

@@quiz
A `#!/bin/sh` script using `[[ ]]` works on Fedora but fails on Ubuntu. Why?
* Ubuntu's /bin/sh is dash, which doesn't support `[[ ]]`; Fedora's /bin/sh is Bash
- Ubuntu has an older kernel
- `[[ ]]` needs root
- Fedora runs scripts in zsh
? Use `#!/usr/bin/env bash` if you use Bash features.

@@quiz
What is the only array available in POSIX sh?
- `declare -a`
* The positional parameters, set with `set -- a b c`
- `typeset`
- Arrays don't exist at all
? Iterate with `for x in "$@"`.

@@quiz
Why is testing with `bash --posix` not enough to prove a script is POSIX-compatible?
- It isn't installed by default
* Bash in POSIX mode still accepts most Bash extensions like `[[ ]]` and arrays
- It runs slower
- It requires a different shebang
? Test with dash or BusyBox, and run ShellCheck.

@@flashcards
dash :: Debian/Ubuntu /bin/sh
BusyBox ash :: Alpine's shell
macOS bash :: 3.2 (no assoc arrays)
`set -- a b c` :: POSIX "array"
`case` :: POSIX pattern matching
`[ "$x" = y ]` :: POSIX test (one =)
`$(( ))` :: POSIX arithmetic
`. ./file` :: POSIX source
`BASH_VERSINFO` :: version as an array
SC3xxx :: ShellCheck POSIX warnings
`sed -i ''` :: BSD in-place edit

@@exercise Rewrite in POSIX sh
Rewrite these Bash functions in strict POSIX sh, then run the file with `sh` (in Git Bash, `sh` is Bash in POSIX mode, so also obey the rules yourself: no `[[ ]]`, arrays, `local`, `${x,,}`, `${x/a/b}`, `$'...'`, `<<<` or `((`):

- `starts_with STRING PREFIX` (use `case`)
- `lower STRING` (use `tr`)
- `count_words` reads lines from stdin and prints how many words (use `set --` inside the loop)
- `join_args SEP ARGS...` (use `"$@"` and `shift`)
- `replace_all STRING FROM TO` (use `sed`, escaping is not needed for these inputs)

The starter writes your POSIX code to a file and runs it with `sh`.
@@starter
tmp=$(mktemp)
cat > "$tmp" <<'SH'
# write POSIX sh here
echo TODO
SH
sh "$tmp"
rm -f -- "$tmp"
@@expected
starts_with web01 web: yes
starts_with db01 web: no
lower: billing-api
words: 7
join: a, b, c
replace: web_01_tokyo
@@requires
case
set --
tr
shift
@@hint
`count_words() { n=0; while IFS= read -r line; do set -- $line; n=$((n + $#)); done; echo "$n"; }`: here the unquoted `$line` is intentional, to split into words. `join_args() { sep=$1; shift; out=$1; shift; for a in "$@"; do out="$out$sep$a"; done; printf '%s\n' "$out"; }`.
@@solution
tmp=$(mktemp)
cat > "$tmp" <<'SH'
starts_with() {
    case $1 in
        "$2"*) return 0 ;;
        *) return 1 ;;
    esac
}

lower() { printf '%s\n' "$1" | tr '[:upper:]' '[:lower:]'; }

count_words() {
    n=0
    while IFS= read -r line; do
        set -- $line
        n=$((n + $#))
    done
    echo "$n"
}

join_args() {
    sep=$1
    shift
    out=$1
    shift
    for a in "$@"; do
        out="$out$sep$a"
    done
    printf '%s\n' "$out"
}

replace_all() { printf '%s\n' "$1" | sed "s/$2/$3/g"; }

for pair in "web01 web" "db01 web"; do
    set -- $pair
    if starts_with "$1" "$2"; then r=yes; else r=no; fi
    echo "starts_with $1 $2: $r"
done
echo "lower: $(lower 'Billing-API')"
printf 'one two three\n\nfour  five\n six seven\n' | { printf 'words: '; count_words; }
echo "join: $(join_args ', ' a b c)"
echo "replace: $(replace_all 'web-01-tokyo' '-' '_')"
SH
sh "$tmp"
rm -f -- "$tmp"

@@exercise GNU or BSD?
Scripts that must run on Linux and macOS often detect the tool flavour once and wrap the differences. Write:

- `detect_sed`: print `gnu` if `sed --version` succeeds, else `bsd`
- `sed_inplace EXPR FILE`: edits a file in place on both flavours (`-i.bak`, then remove the backup)
- `file_size FILE`: portable size with `wc -c <`
- `days_ago N`: prints the date N days before 2026-11-03 with GNU `date -d` if available, else BSD `date -v`

Show them working on a temporary file.
@@starter
echo TODO
@@expected
sed flavour: gnu
before: port=80
after: port=8080
backup left behind: no
size: 10 bytes
3 days before 2026-11-03: 2026-10-31
@@requires
-i.bak
wc -c <
@@hint
`days_ago() { if date -d '2026-11-03' +%F >/dev/null 2>&1; then date -d "2026-11-03 - $1 days" +%F; else date -j -v-"$1"d -f %F 2026-11-03 +%F; fi; }`. `wc -c < file` prints only the number (some systems pad it with spaces; strip them with `$(( ))`).
@@solution
detect_sed() {
    if sed --version >/dev/null 2>&1; then echo gnu; else echo bsd; fi
}

sed_inplace() {
    sed -i.bak "$1" "$2" && rm -f -- "$2.bak"
}

file_size() {
    local n
    n=$(wc -c < "$1")
    echo $(( n ))
}

days_ago() {
    if date -d '2026-11-03' +%F >/dev/null 2>&1; then
        date -d "2026-11-03 - $1 days" +%F
    else
        date -j -v-"$1"d -f %F 2026-11-03 +%F
    fi
}

echo "sed flavour: $(detect_sed)"
dir=$(mktemp -d)
printf 'port=80\n' > "$dir/app.conf"
echo "before: $(cat "$dir/app.conf")"
sed_inplace 's/^port=80$/port=8080/' "$dir/app.conf"
echo "after: $(cat "$dir/app.conf")"
if [[ -e $dir/app.conf.bak ]]; then echo "backup left behind: yes"; else echo "backup left behind: no"; fi
echo "size: $(file_size "$dir/app.conf") bytes"
echo "3 days before 2026-11-03: $(days_ago 3)"
rm -rf -- "$dir"
