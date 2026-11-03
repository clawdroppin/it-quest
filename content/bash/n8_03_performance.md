@@lesson n8-performance Performance: Forks, Builtins and Choosing the Right Tool
@@concepts processes, text-processing
@@text
Bash is slow at computation but fast at **orchestration**. Most slow scripts are slow for one reason: they start thousands of processes. Starting a process (`fork` + `exec`) costs around 1 ms on Linux and much more on Windows/WSL1/Git Bash, while a builtin operation costs microseconds.

## Count the processes

```bash,norun
# 10,000 lines → 30,000 processes: echo | cut, echo | tr, and $( ) subshells
while read -r line; do
    user=$(echo "$line" | cut -d: -f1)
    upper=$(echo "$user" | tr a-z A-Z)
    echo "$upper"
done < /etc/passwd

# zero extra processes
while IFS=: read -r user _; do
    echo "${user^^}"
done < /etc/passwd

# or one process for everything
awk -F: '{ print toupper($1) }' /etc/passwd
```

## Builtin replacements

| Slow (new process) | Fast (builtin) |
|---|---|
| `$(basename "$f")` / `$(dirname "$f")` | `${f##*/}` / `${f%/*}` |
| `$(echo "$s" \| tr a-z A-Z)` | `${s^^}` |
| `$(echo "$s" \| sed 's/a/b/g')` | `${s//a/b}` |
| `$(expr $a + $b)` | `$(( a + b ))` |
| `$(cat file)` | `$(< file)` |
| `$(date +%s)` | `$EPOCHSECONDS` / `printf -v now '%(%s)T' -1` |
| `$(seq 1 100)` | `{1..100}` or `for ((i=1; i<=100; i++))` |
| `echo "$s" \| grep -q x` | `[[ $s == *x* ]]` |
| `$(wc -l < f)` for each line | count in the loop |
| `x=$(func)` in a hot loop | `func` writes to a nameref or `printf -v` |

## When to hand over to awk (or another language)

Bash loops over lines are fine for hundreds of lines. For **large files**, one `awk`, `sort`, `grep` or `sed` process beats any Bash loop, because the loop happens in C:

```bash,norun
# Bash: ~2 s for 100,000 lines          awk: ~0.05 s
while read -r _ _ size; do (( total += size )); done < big.txt
awk '{ total += $3 } END { print total }' big.txt
```

And when the problem has real data structures, floating point or JSON, write it in Python or Go. A good rule: if a Bash script is over ~200 lines of logic (not commands), consider switching.

## Measuring

```bash,norun
time ./script.sh                 # real (wall clock), user and sys CPU time
TIMEFORMAT='%3R s'; time { for i in {1..1000}; do x=$(echo hi); done; }
strace -c -f ./script.sh         # count system calls (Linux)
bash -x ./script.sh 2>&1 | wc -l # how many commands ran
```

On Linux you can count forks: `strace -f -e trace=clone,fork,vfork -c ./script.sh`.

## Other hot spots

- Reading a big file into a variable and then looping over `${content}` repeatedly: loop once.
- `grep` in a loop over patterns: use one `grep -F -f patterns.txt`.
- `sort | uniq -c | sort -rn` is fast; nested loops comparing two lists are O(n²): use `comm`, `join` or awk's arrays.
- `ssh` in a loop: open one connection with `ControlMaster`, or run one remote script.

@@quiz
Why is `x=$(echo "$s" | tr a-z A-Z)` slow inside a loop?
* It starts a subshell and two processes per iteration; `${s^^}` does it inside Bash
- tr is a slow algorithm
- echo is slow
- It isn't slow
? Process creation dominates.

@@quiz
When should you replace a Bash `while read` loop with awk?
- Never
* When processing large files: awk loops in compiled code, often 10–50× faster
- Only for CSV files
- When the file has fewer than 10 lines
? Bash for orchestration, awk for bulk text.

@@quiz
What's the fast replacement for `$(cat file)`?
- `$(read file)`
* `$(< file)`
- `${file}`
- `$(source file)`
? Bash reads the file without starting cat.

@@flashcards
fork + exec :: cost of starting a process
`$(< file)` :: read a file without cat
`${f##*/}` :: builtin basename
`$(( ))` :: builtin arithmetic
`EPOCHSECONDS` :: builtin current time
`time` :: measure real/user/sys
`TIMEFORMAT` :: format for time
`strace -c -f` :: count system calls
`grep -F -f file` :: many fixed patterns at once
awk :: bulk text processing

@@exercise Remove the forks
The function `slow_report` produces the right output but starts several processes per line. Write `fast_report` that produces **identical** output with no external commands and no `$( )` inside the loop (only builtins and parameter expansion). The starter counts calls to the external-command wrappers to prove it.
@@starter
counter=$(mktemp)
ext() { echo x >> "$counter"; "$@"; }   # records every external command
data='/var/log/nginx/access.log 2048
/var/log/syslog 8192
/home/jakub/notes.TXT 12'
slow_report() {
    local path size base dir ext kb
    while read -r path size; do
        base=$(ext basename "$path")
        dir=$(ext dirname "$path")
        ext=$(echo "$base" | ext awk -F. '{print $NF}' | ext tr 'A-Z' 'a-z')
        kb=$(ext expr "$size" / 1024)
        echo "$dir | $base | $ext | ${kb}K"
    done <<< "$data"
}
count() { local n; n=$(wc -l < "$counter"); echo $(( n )); : > "$counter"; }
slow_report
echo "slow forks: $(count)"
echo TODO
rm -f -- "$counter"
@@expected
/var/log/nginx | access.log | log | 2K
/var/log | syslog | syslog | 8K
/home/jakub | notes.TXT | txt | 0K
slow forks: 15
/var/log/nginx | access.log | log | 2K
/var/log | syslog | syslog | 8K
/home/jakub | notes.TXT | txt | 0K
fast forks: 0
@@requires
##*/
,,
$((
@@hint
`ext` appends to a file instead of incrementing a variable because it runs inside `$( )`, a subshell, where a counter change would be lost. In `fast_report`: `base=${path##*/}; dir=${path%/*}; ext=${base##*.}; ext=${ext,,}; kb=$(( size / 1024 ))`.
@@solution
counter=$(mktemp)
ext() { echo x >> "$counter"; "$@"; }
data='/var/log/nginx/access.log 2048
/var/log/syslog 8192
/home/jakub/notes.TXT 12'
slow_report() {
    local path size base dir ext kb
    while read -r path size; do
        base=$(ext basename "$path")
        dir=$(ext dirname "$path")
        ext=$(echo "$base" | ext awk -F. '{print $NF}' | ext tr 'A-Z' 'a-z')
        kb=$(ext expr "$size" / 1024)
        echo "$dir | $base | $ext | ${kb}K"
    done <<< "$data"
}

fast_report() {
    local path size base dir ext kb
    while read -r path size; do
        base=${path##*/}
        dir=${path%/*}
        ext=${base##*.}
        ext=${ext,,}
        kb=$(( size / 1024 ))
        echo "$dir | $base | $ext | ${kb}K"
    done <<< "$data"
}

count() { local n; n=$(wc -l < "$counter"); echo $(( n )); : > "$counter"; }

slow_report
echo "slow forks: $(count)"
fast_report
echo "fast forks: $(count)"
rm -f -- "$counter"

@@exercise Bash loop vs awk on a big input
Generate 20,000 lines of `host bytes status` data with one awk command (`srand(42)`-free: derive values from the line number so the output is deterministic). Then compute the same summary twice, once with a Bash `while read` loop and once with a single awk program:

- total bytes, number of 5xx responses, and the host with the most bytes

Print both results and check they're identical. Print the timings to **stderr** (they vary between runs) and only a success message to stdout.
@@starter
gen() { awk 'BEGIN { for (i = 1; i <= 20000; i++) printf "web%02d %d %d\n", i % 7, (i * 37) % 5000, (i % 50 == 0) ? 503 : 200 }'; }
echo TODO
@@expected
bash: total=49990000 errors=400 top=web03
awk:  total=49990000 errors=400 top=web03
results match: yes
@@requires
awk
while read
EPOCHREALTIME
@@hint
Bash: `declare -A per=(); while read -r h b s; do (( total += b )); (( s >= 500 )) && (( errors++ )); (( per[$h] += b )); done < <(gen)` then find the max key. awk: the same with `per[$1] += $2` and a loop in END. Time each with `start=$EPOCHREALTIME` and `echo "... $(( (${EPOCHREALTIME/./} - ${start/./}) / 1000 )) ms" >&2`. If two hosts tie, pick the alphabetically first to stay deterministic.
@@solution
gen() { awk 'BEGIN { for (i = 1; i <= 20000; i++) printf "web%02d %d %d\n", i % 7, (i * 37) % 5000, (i % 50 == 0) ? 503 : 200 }'; }

data=$(mktemp)
gen > "$data"

start=$EPOCHREALTIME
declare -A per=()
total=0 errors=0
while read -r h b s; do
    (( total += b ))
    (( s >= 500 )) && (( errors++ ))
    (( per[$h] += b ))
done < "$data"
top="" max=-1
for h in $(printf '%s\n' "${!per[@]}" | LC_ALL=C sort); do
    (( per[$h] > max )) && { max=${per[$h]}; top=$h; }
done
bash_result="total=$total errors=$errors top=$top"
echo "bash loop: $(( (${EPOCHREALTIME/./} - ${start/./}) / 1000 )) ms" >&2

start=$EPOCHREALTIME
awk_result=$(awk '
{ total += $2; if ($3 >= 500) errors++; per[$1] += $2 }
END {
    max = -1
    for (h in per) if (per[h] > max || (per[h] == max && h < top)) { max = per[h]; top = h }
    printf "total=%d errors=%d top=%s", total, errors, top
}' "$data")
echo "awk: $(( (${EPOCHREALTIME/./} - ${start/./}) / 1000 )) ms" >&2

echo "bash: $bash_result"
echo "awk:  $awk_result"
if [[ $bash_result == "$awk_result" ]]; then echo "results match: yes"; else echo "results match: no"; fi
rm -f -- "$data"
