@@lesson e7-printf-read printf and read: Precise Output and Safe Input
@@concepts strings, input
@@text
`echo` is fine for quick messages, but its behaviour with `-e`, `-n` and backslashes differs between shells. For anything formatted, use **`printf`**. And for reading input, `read` has a handful of options that separate robust scripts from fragile ones.

## printf: a format string plus arguments

```bash
printf '%s has %d CPUs\n' web01 8
printf '%-10s|%5s|\n' left right       # pad: - means left-align
printf '%05d %x %o %e\n' 42 255 8 12345.678
printf '%.2f%%\n' 93.456               # 93.46%  (%% is a literal %)
printf '%s\n' a b c                    # the format is REUSED for extra arguments
printf '%q\n' "it's a \$path"          # quote safely for re-use as shell input
printf -v line '%-8s %s' disk 91%      # store in a variable instead of printing
echo "[$line]"
```

Key rules:

- The format string should be a **literal**: `printf "$user_input"` lets `%` sequences in the data break your output. Use `printf '%s\n' "$data"`.
- `%b` expands backslash escapes in the **argument** (`printf '%b' 'a\tb'`); `%s` prints it literally.
- `printf '%(%F %T)T\n' -1` prints the current time without starting `date`.
- `printf '%*s' 20 ''` prints 20 spaces: a quick way to draw padding and bars, and `${bar// /#}` turns them into `#`.

## read: the options that matter

```bash,norun
read -r line                   # -r: keep backslashes (always use it)
read -rp "Host: " host         # prompt
read -rs -p "Password: " pw    # silent, for secrets
read -rn1 -p "Continue? [y/N] " answer   # one character, no Enter needed
read -rt 5 reply || echo "timed out"     # timeout in seconds
read -r -d '' blob             # read until NUL (or end of input)
IFS=: read -r user _ uid gid _ <<< "root:x:0:0:root:/root:/bin/bash"
```

## Reading files line by line, correctly

```bash,norun
while IFS= read -r line || [[ -n $line ]]; do
    printf '[%s]\n' "$line"
done < file.txt
```

- `IFS=` keeps leading and trailing spaces.
- `-r` keeps backslashes.
- `|| [[ -n $line ]]` handles a last line with no final newline (`read` returns false but still fills `line`).

## NUL-separated data

File names can contain spaces and even newlines. Only NUL (`\0`) can't appear in a path, so robust pipelines pass names NUL-separated:

```bash,norun
find . -name '*.log' -print0 | while IFS= read -r -d '' f; do gzip -- "$f"; done
mapfile -d '' -t files < <(find . -type f -print0)
```

@@quiz
Why is `printf "$msg"` risky?
* Any `%` or backslash in the data is interpreted as formatting
- printf can't print variables
- It adds a newline twice
- It is slower than echo
? Use `printf '%s\n' "$msg"`.

@@quiz
What does `read -r` change?
- Reads in reverse
* Backslashes are kept as-is instead of being treated as escapes
- Reads one character
- Reads from a remote host
? Without -r, `C:\new` becomes `C:new`.

@@quiz
Why does `while read -r line` sometimes skip the last line of a file?
- read ignores short lines
* The last line has no trailing newline, so read returns false even though it filled `line`
- The loop body runs before reading
- IFS removes it
? Add `|| [[ -n $line ]]`.

@@drill
Which `read` option stops backslashes being treated as escapes?
= -r
? Always use read -r.

@@flashcards
`%-10s` :: left-aligned in 10 columns
`%05d` :: zero-padded integer
`%.2f` :: two decimals
`%%` :: a literal percent sign
`%q` :: shell-quoted
`%b` :: expand escapes in the argument
`printf -v var` :: print into a variable
`printf '%(%F)T' -1` :: today's date (builtin)
`read -r` :: raw (keep backslashes)
`read -s` / `-n1` / `-t5` :: silent / one char / timeout
`read -d ''` :: read until NUL
`IFS= read -r line` :: exact line

@@exercise A formatted disk report
Print a fixed-width table of the mounts with `printf` only (no `column`):

- columns: mount (left, 10 wide), size in GiB (right, 6 wide, 1 decimal), used % (right, 4 wide) and a 20-character bar of `#` and `.`
- compute GiB with integer arithmetic: `size_mb * 10 / 1024` gives tenths
- mark mounts at 90% or more with ` !`
- end with a total line built with `printf -v`
@@starter
mounts='/ 51200 41984
/home 204800 81920
/var 20480 19046
/boot 1024 220'
echo TODO
@@expected
MOUNT         GiB USE% USAGE
/            50.0  82% ################....
/home       200.0  40% ########............
/var         20.0  92% ##################.. !
/boot         1.0  21% ####................
total: 271.0 GiB, 139.8 GiB used
@@requires
printf -v
%-10s
@@hint
`pct=$(( used * 100 / size ))`, `filled=$(( pct * 20 / 100 ))`. Build the bar with `printf -v hashes '%*s' "$filled" ''` then `${hashes// /#}`, and the same for the dots. Tenths of GiB: `t=$(( size * 10 / 1024 ))`, printed as `$((t / 10)).$((t % 10))`.
@@solution
mounts='/ 51200 41984
/home 204800 81920
/var 20480 19046
/boot 1024 220'

gib() { local t=$(( $1 * 10 / 1024 )); printf '%d.%d' $((t / 10)) $((t % 10)); }

printf '%-10s %6s %4s %s\n' MOUNT GiB USE% USAGE
total=0 total_used=0
while read -r mount size used; do
    pct=$(( used * 100 / size ))
    filled=$(( pct * 20 / 100 ))
    printf -v hashes '%*s' "$filled" ''
    printf -v dots '%*s' $(( 20 - filled )) ''
    flag=""
    (( pct >= 90 )) && flag=" !"
    printf '%-10s %6s %3d%% %s%s%s\n' "$mount" "$(gib "$size")" "$pct" "${hashes// /#}" "${dots// /.}" "$flag"
    (( total += size, total_used += used ))
done <<< "$mounts"
printf -v summary 'total: %s GiB, %s GiB used' "$(gib "$total")" "$(gib "$total_used")"
echo "$summary"

@@exercise Parsing awkward input safely
1. Read `/etc/passwd`-style lines with `IFS=: read -r` and print users with a login shell (not ending in `nologin` or `false`), keeping the home directory
2. Read a here-string that has leading spaces, a backslash and no final newline, and show each line exactly with `[...]`
3. Read NUL-separated file names (one contains a space, one a newline) with `read -d ''` and print them with `%q`
@@starter
passwd='root:x:0:0:root:/root:/bin/bash
daemon:x:1:1:daemon:/usr/sbin:/usr/sbin/nologin
jakub:x:1000:1000:Jakub:/home/jakub:/bin/bash
backup:x:34:34:backup:/var/backups:/bin/false'
echo TODO
@@expected
root (uid 0) home=/root
jakub (uid 1000) home=/home/jakub
[  indented line]
[C:\temp\new]
[last line without newline]
names: 3
my\ file.txt
plain.txt
$'report\n2026.txt'
@@requires
IFS=: read -r
read -r -d ''
%q
@@hint
For part 2 use `printf '  indented line\nC:\\temp\\new\nlast line without newline'` as the input and `while IFS= read -r line || [[ -n $line ]]`. For part 3: `printf '%s\0' 'my file.txt' $'report\n2026.txt' plain.txt | LC_ALL=C sort -z` piped into a loop is lost to a subshell, so feed the loop with `< <(...)` and collect into an array.
@@solution
passwd='root:x:0:0:root:/root:/bin/bash
daemon:x:1:1:daemon:/usr/sbin:/usr/sbin/nologin
jakub:x:1000:1000:Jakub:/home/jakub:/bin/bash
backup:x:34:34:backup:/var/backups:/bin/false'

while IFS=: read -r user _ uid _ _ home shell; do
    case $shell in
        */nologin|*/false) continue ;;
    esac
    echo "$user (uid $uid) home=$home"
done <<< "$passwd"

while IFS= read -r line || [[ -n $line ]]; do
    printf '[%s]\n' "$line"
done < <(printf '  indented line\nC:\\temp\\new\nlast line without newline')

names=()
while IFS= read -r -d '' name; do
    names+=("$name")
done < <(printf '%s\0' 'my file.txt' $'report\n2026.txt' plain.txt | LC_ALL=C sort -z)
echo "names: ${#names[@]}"
printf '%q\n' "${names[@]}"
