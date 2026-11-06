@@lesson l1-loops Loops and Reading Files
@@concepts loops
@@text
## for

```bash
for server in web01 web02 db01; do
    echo "Checking $server"
done

for i in {1..3}; do echo "Attempt $i"; done

for (( i = 0; i < 3; i++ )); do echo "i=$i"; done
```

## Looping over files

```bash
mkdir -p /tmp/itq-loop && cd /tmp/itq-loop
touch a.log b.log c.txt
for f in *.log; do
    echo "Compressing $f"
done
cd / && rm -r /tmp/itq-loop
```

Use globs like `*.log`, never `for f in $(ls)`, which breaks on spaces in names.

## while, and reading a file line by line

```bash
printf "web01 10.0.0.11\nweb02 10.0.0.12\ndb01 10.0.0.21\n" > /tmp/itq-hosts.txt
while read -r name ip; do
    echo "$name is at $ip"
done < /tmp/itq-hosts.txt
rm /tmp/itq-hosts.txt
```

`read -r name ip` splits each line on whitespace into variables (the last one gets the rest of the line). `-r` keeps backslashes as they are. This pattern processes files of any size, one line at a time.

Output of a command works too:

```bash
printf "3\n1\n2\n" | sort | while read -r n; do
    echo "got $n"
done
```

## A retry loop

```bash,norun
for attempt in {1..5}; do
    if curl -sf https://example.com/health > /dev/null; then
        echo "service is up"
        break
    fi
    echo "attempt $attempt failed, retrying in 2s"
    sleep 2
done
```

`break` leaves the loop, `continue` skips to the next round.

## Arrays

```bash
servers=(web01 web02 db01)
echo "${servers[0]}"
echo "${#servers[@]} servers"
servers+=(mail01)
for s in "${servers[@]}"; do echo "- $s"; done
```

`"${servers[@]}"` (in quotes) gives each item as its own word, even ones with spaces.

## Loop control and common pitfalls

- **`break`** leaves the loop and **`continue`** skips to the next iteration. Both take a number for nested loops: `break 2` leaves two levels.
- **C-style loops** are handy for counters: `for ((i = 1; i <= 5; i++)); do ...; done`.
- **Brace expansion** builds lists: `{1..5}`, `{01..10}` (zero-padded), `{a..e}`, `{0..100..10}`. It happens before variables are expanded, so `{1..$n}` doesn't work; use `seq 1 "$n"` or a C-style loop.
- **Don't loop over `ls` output.** `for f in $(ls)` breaks on names with spaces and newlines. Use a glob, `for f in *.log`, and guard against no matches with `shopt -s nullglob` (or `[[ -e $f ]] || continue`).
- **Pipes run loops in a subshell.** In `cat file | while read -r line; do count=$((count + 1)); done`, the `count` you see after the loop is unchanged, because the loop ran in a child process. Redirect instead: `while read -r line; do ...; done < file`, or use process substitution `< <(command)`.
- **Read lines safely** with `while IFS= read -r line`: `IFS=` keeps leading spaces and `-r` keeps backslashes. If the last line has no trailing newline, add `|| [[ -n $line ]]` to the condition so it isn't skipped.

```bash
for i in {1..10}; do
  (( i % 2 == 0 )) && continue
  (( i > 7 )) && break
  echo "odd: $i"
done
count=0
while IFS= read -r line; do
  count=$((count + 1))
done < <(printf 'a\nb\nc\n')
echo "lines: $count"
```

When a loop does something slow per item (a network call, a compression), think about whether the items could run in parallel. `xargs -P 4` or GNU `parallel` run several at once, as the professional-scripting lesson shows.

@@quiz
Why is `for f in $(ls)` a bad idea?
* File names with spaces get split into several words
- `ls` is too slow
- for loops can't use `$( )`
- It only works on Linux
? Use a glob: `for f in *`. The shell handles names with spaces correctly.

@@quiz
In `while read -r user shell; do ...; done < users.txt`, a line is `ana /bin/bash`. What is `$shell`?
- ana
* /bin/bash
- the whole line
- empty
? `read` splits the line on whitespace: the first word goes into `user`, the rest into `shell`.

@@quiz
Why is `while IFS= read -r line; do ...; done < file` better than `for line in $(cat file)`?
* It reads whole lines, keeping spaces and backslashes, instead of splitting on every space
- It's the only loop that works on files
- for can't read files
- It sorts the lines
? for over $(cat ...) splits on whitespace and expands wildcards.

@@flashcards
for x in a b c :: loop over words
for f in *.log :: loop over matching files
for ((i=0; i<5; i++)) :: C-style counting loop
while cond; do ... done :: loop while a command succeeds
while IFS= read -r line :: read a file line by line
{1..10} :: brace expansion: 1 to 10
break / continue :: leave the loop / next round
shopt -s nullglob :: unmatched globs expand to nothing

@@exercise Server checklist
Print `- <name>` for each server in the array, then `Total: 4`. Use the array's length, don't type 4.
@@starter
servers=(web01 web02 db01 mail01)

@@expected
- web01
- web02
- db01
- mail01
Total: 4
@@hint
`for s in "${servers[@]}"; do echo "- $s"; done`, then `echo "Total: ${#servers[@]}"`.
@@solution
servers=(web01 web02 db01 mail01)
for s in "${servers[@]}"; do
    echo "- $s"
done
echo "Total: ${#servers[@]}"

@@exercise Users with a login shell
The here-document imitates `/etc/passwd` (fields separated by `:`; the 7th field is the shell). Print the names (field 1) of users whose shell is `/bin/bash`.
@@starter
passwd_data=$(cat <<'EOF'
root:x:0:0:root:/root:/bin/bash
daemon:x:1:1:daemon:/usr/sbin:/usr/sbin/nologin
ana:x:1000:1000:Ana Silva:/home/ana:/bin/bash
www-data:x:33:33:www-data:/var/www:/usr/sbin/nologin
ben:x:1001:1001:Ben Jones:/home/ben:/bin/bash
EOF
)

@@expected
root
ana
ben
@@hint
`IFS` sets the separator for `read`: `while IFS=: read -r user _ _ _ _ _ shell; do ...; done <<< "$passwd_data"`. (`_` is a common name for fields you don't need; `<<<` feeds a string as input.)
@@solution
passwd_data=$(cat <<'EOF'
root:x:0:0:root:/root:/bin/bash
daemon:x:1:1:daemon:/usr/sbin:/usr/sbin/nologin
ana:x:1000:1000:Ana Silva:/home/ana:/bin/bash
www-data:x:33:33:www-data:/var/www:/usr/sbin/nologin
ben:x:1001:1001:Ben Jones:/home/ben:/bin/bash
EOF
)
while IFS=: read -r user _ _ _ _ _ shell; do
    if [[ $shell == /bin/bash ]]; then
        echo "$user"
    fi
done <<< "$passwd_data"
@@text
> That's a real Linux+ question: *which accounts can log in?* `/usr/sbin/nologin` is how service accounts like `www-data` are prevented from logging in interactively.
