@@lesson l2-redirection Redirection in Depth: File Descriptors, Here-Docs and Process Substitution
@@concepts files, processes
@@text
Every process starts with three open **file descriptors**: 0 (stdin), 1 (stdout) and 2 (stderr). Redirection rewires them, and once you think in descriptors, the confusing cases make sense.

## The basics, precisely

```bash,norun
cmd > out.txt          # fd 1 → file (truncate)
cmd >> out.txt         # fd 1 → file (append)
cmd 2> err.txt         # fd 2 → file
cmd &> all.txt         # both (Bash shortcut for > all.txt 2>&1)
cmd 2>/dev/null        # throw errors away
echo "oops" >&2        # write to stderr: do this for errors and logs in your scripts
cmd < input.txt        # fd 0 ← file
```

## Order matters

Redirections are applied **left to right**, and `2>&1` means "make fd 2 point where fd 1 points **right now**":

```bash,norun
cmd > file 2>&1        # 1 → file, then 2 → (where 1 points) file: both in the file
cmd 2>&1 > file        # 2 → terminal (where 1 pointed), then 1 → file: errors still on screen
cmd 2>&1 >/dev/null | grep ERROR   # only stderr goes into the pipe
```

## Here-documents and here-strings

```bash
host=web01
cat <<EOF
server_name $host.example.jp;    # variables and $(commands) are expanded
EOF
cat <<'EOF'
echo "$HOME"                     # quoted delimiter: nothing is expanded
EOF
grep -c web <<< "web01 web02"    # here-string: a single string as stdin
```

`<<-EOF` strips leading **tabs** (not spaces), so here-docs can be indented inside functions.

## Process substitution

`<(cmd)` runs a command and gives you a file name (like `/dev/fd/63`) to read its output. Perfect for commands that want files, not pipes:

```bash
diff <(printf 'a\nb\nc\n') <(printf 'a\nB\nc\n')
comm -3 <(printf 'x\ny\n') <(printf 'y\nz\n')
```

It also avoids the "variables set inside a pipeline loop are lost" problem, because the loop doesn't run in a subshell:

```bash
count=0
while read -r line; do ((count++)); done < <(printf '1\n2\n3\n')
echo "$count"     # 3 (with `printf ... | while`, it would print 0)
```

## exec: redirect the whole script, or open your own descriptors

```bash,norun
exec > >(tee -a /var/log/deploy.log) 2>&1     # log everything from here on
exec 3< hosts.txt                              # open fd 3 for reading
while read -r host <&3; do ssh "$host" uptime; done   # ssh can't eat the host list from stdin
exec 3<&-                                      # close it
exec {fd}> /tmp/lock                           # let Bash pick a free fd number, stored in $fd
```

@@quiz
Which command sends both stdout and stderr to `log.txt`?
* `cmd > log.txt 2>&1`
- `cmd 2>&1 > log.txt`
- `cmd 2> log.txt 1>&2`
- `cmd | log.txt`
? Left to right: point 1 at the file first, then copy it to 2.

@@quiz
What's the difference between `<<EOF` and `<<'EOF'`?
- None
* With the quoted delimiter, nothing in the here-doc is expanded
- The quoted form appends
- The quoted form strips tabs
? Use the quoted form to write scripts that contain `$`.

@@quiz
Why does `printf '1\n2\n' | while read -r x; do ((n++)); done; echo $n` print nothing useful?
- read fails on numbers
* Each part of a pipeline runs in a subshell, so n is changed in a copy
- printf doesn't output newlines
- ((n++)) is invalid
? Use `done < <(printf ...)` instead.

@@drill
Write the redirection that sends both stdout and stderr to `log.txt` (portable order).
= > log.txt 2>&1
= >log.txt 2>&1
? Order matters: 2>&1 copies wherever stdout points now.

@@flashcards
fd 0 / 1 / 2 :: stdin / stdout / stderr
`>&2` :: write to stderr
`2>&1` :: fd 2 → wherever fd 1 points now
`&>` :: both outputs to a file
`<<EOF` :: here-doc with expansion
`<<'EOF'` :: literal here-doc
`<<-EOF` :: strip leading tabs
`<<<` :: here-string
`<(cmd)` :: process substitution (read)
`exec 3< file` :: open a descriptor
`exec > >(tee log)` :: log the rest of the script

@@exercise stdout, stderr and the order of redirections
`check_hosts` prints OK lines to stdout and errors to stderr. In a temporary directory (`mktemp -d`):

- capture stdout and stderr in separate files and count their lines
- send **only the errors** through a pipe to `grep` (the order of redirections matters)
- capture both into one file and count the lines
- write a `log` function that prints to stderr, and show its message disappears with `2>/dev/null`
@@starter
check_hosts() {
    echo "OK web01"
    echo "ERROR db02 unreachable" >&2
    echo "OK cache01"
}
echo TODO
@@expected
stdout lines: 2
stderr lines: 1
only errors via pipe: ERROR db02 unreachable
combined: 3 lines
log with stderr hidden: (nothing shown)
@@requires
2>&1 >/dev/null
>&2
@@hint
`check_hosts > "$tmp/out" 2> "$tmp/err"`, then `wc -l < "$tmp/out"`. Errors only: `check_hosts 2>&1 >/dev/null | grep ERROR`. Combined: `check_hosts > "$tmp/all" 2>&1`. For the last line, capture `out=$(log "hidden" 2>/dev/null)` and print `(nothing shown)` when it's empty. Clean up with `rm -rf "$tmp"`.
@@solution
check_hosts() {
    echo "OK web01"
    echo "ERROR db02 unreachable" >&2
    echo "OK cache01"
}

log() { echo "[$(date +%T)] $*" >&2; }

tmp=$(mktemp -d)
check_hosts > "$tmp/out" 2> "$tmp/err"
echo "stdout lines: $(wc -l < "$tmp/out")"
echo "stderr lines: $(wc -l < "$tmp/err")"
echo "only errors via pipe: $(check_hosts 2>&1 >/dev/null | grep ERROR)"
check_hosts > "$tmp/all" 2>&1
echo "combined: $(wc -l < "$tmp/all") lines"
out=$(log "this goes to stderr" 2>/dev/null)
echo "log with stderr hidden: ${out:-(nothing shown)}"
rm -rf "$tmp"

@@exercise Here-docs, process substitution and extra descriptors
1. Generate an nginx server block with a here-doc that expands `$host` and `$port`, and a literal script line with a quoted here-doc
2. Compare the old and new host lists with `diff` and `comm` using process substitution (no temporary files)
3. Count lines in a `while read` loop fed by process substitution, and show the count survives the loop
4. Read the host list through file descriptor 3 (`exec 3< <(...)`) and close it afterwards
@@starter
host=web01
port=8080
old_hosts() { printf '%s\n' cache01 db01 web01; }
new_hosts() { printf '%s\n' cache01 db02 web01; }
echo TODO
@@expected
server {
    listen 8080;
    server_name web01.example.jp;
}
literal: echo "Deploying to $host"
2c2
< db01
---
> db02
removed: db01
added: db02
lines counted: 3
fd 3: cache01 db02 web01
@@requires
<<'EOF'
<(
exec 3<
@@hint
`comm -23 <(old_hosts) <(new_hosts)` shows lines only in the first input (removed), `comm -13` lines only in the second (added); both inputs must be sorted. Read through fd 3 with `while read -r h <&3; do list+=("$h"); done`, then `exec 3<&-`.
@@solution
host=web01
port=8080
old_hosts() { printf '%s\n' cache01 db01 web01; }
new_hosts() { printf '%s\n' cache01 db02 web01; }

cat <<EOF
server {
    listen $port;
    server_name $host.example.jp;
}
EOF
printf 'literal: '
cat <<'EOF'
echo "Deploying to $host"
EOF

diff <(old_hosts) <(new_hosts)
echo "removed: $(comm -23 <(old_hosts) <(new_hosts))"
echo "added: $(comm -13 <(old_hosts) <(new_hosts))"

count=0
while read -r line; do
    ((count++))
done < <(new_hosts)
echo "lines counted: $count"

exec 3< <(new_hosts)
list=()
while read -r h <&3; do
    list+=("$h")
done
exec 3<&-
echo "fd 3: ${list[*]}"
