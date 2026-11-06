@@lesson l5-professional Expert Bash: Associative Arrays, Parallelism, Git
@@concepts maps, concurrency
@@text
## Associative arrays (dictionaries)

```bash
declare -A owner=( [web01]=ana [db01]=kenji )
owner[mail01]=ben
echo "${owner[db01]}"
for host in "${!owner[@]}"; do        # ${!arr[@]} = the keys
    echo "$host -> ${owner[$host]}"
done | sort
[[ -v owner[web01] ]] && echo "web01 has an owner"
```

The key order is random, so `sort` when it matters. Counting works like awk:

```bash
declare -A count
for word in error warn error info error; do
    (( count[$word]++ ))
done
for k in "${!count[@]}"; do echo "$k ${count[$k]}"; done | sort -k2 -rn
```

## Running things in parallel

```bash
check() { sleep 0.2; echo "$1 ok"; }
for h in web01 web02 web03; do
    check "$h" &       # & = run in the background
done
wait                   # wait for all background jobs
echo "all checks done"
```

Three checks of 0.2 s each finish in about 0.2 s, not 0.6 s. For big lists, limit concurrency: `xargs -P 8` or GNU `parallel`.

## Process substitution and here-docs

```bash
diff <(printf "a\nb\nc\n") <(printf "a\nc\nd\n")  # compare two commands' output
cat <<EOF
Report for $(date +%F)
Host: ${HOSTNAME:-unknown}
EOF
```

`<( cmd )` behaves like a file containing the command's output. A here-doc with `<<EOF` expands variables; `<<'EOF'` keeps everything literal.

## Git, from the command line

Every script and config file belongs in version control.

```bash,norun
git init
git add backup.sh
git commit -m "Add backup script"
git log --oneline
git diff                        # what changed since the last commit
git switch -c fix-retention     # a branch for a change
git push origin fix-retention   # share it, then open a pull request
```

"Infrastructure as code" (Terraform, Ansible, Kubernetes YAML) is this same idea at scale: servers are described in files in Git, reviewed like code, and applied automatically.

## When to stop using Bash

Bash is perfect for gluing commands together. Switch to **Python** when a script needs real data structures, JSON handling, error handling across many steps, or grows past ~150 lines. Knowing when to switch is part of being senior.

@@quiz
How do you loop over the **keys** of an associative array `owner`?
* `for k in "${!owner[@]}"`
- `for k in "${owner[@]}"`
- `for k in owner.keys`
- `for k in "${#owner[@]}"`
? `${!arr[@]}` gives the keys, `${arr[@]}` the values, `${#arr[@]}` the count.

@@quiz
What does `wait` do after starting jobs with `&`?
* Waits until all background jobs have finished
- Pauses for one second
- Kills the background jobs
- Waits for user input
? Without `wait`, the script could end before the jobs finish.

@@quiz
Which declaration creates an associative array in Bash?
* declare -A owner
- declare -a owner
- owner=()
- array owner
? -a is an indexed array; -A is an associative array (Bash 4+).

@@flashcards
declare -A map :: associative array (dictionary)
"${!map[@]}" :: the keys of an associative array
"${map[@]}" :: the values
cmd & :: run in the background
wait :: wait for background jobs
xargs -P 4 :: run up to four commands in parallel
<(cmd) :: process substitution: output as a file
<<'EOF' :: here-doc without variable expansion

@@exercise Group by owner
`servers` lists `host owner` pairs. Using an **associative array**, print each owner followed by their hosts (comma-separated, in input order), owners sorted alphabetically.
@@starter
servers='web01 ana
db01 kenji
web02 ana
mail01 ben
db02 kenji'

@@expected
ana: web01,web02
ben: mail01
kenji: db01,db02
@@requires
declare -A
@@hint
`declare -A hosts`, then in a `while read -r host owner` loop: `hosts[$owner]+="${hosts[$owner]:+,}$host"` (`${var:+,}` adds a comma only if the value is already non-empty). Print with `for o in "${!hosts[@]}"; do ...; done | sort`.
@@solution
servers='web01 ana
db01 kenji
web02 ana
mail01 ben
db02 kenji'
declare -A hosts
while read -r host owner; do
    hosts[$owner]+="${hosts[$owner]:+,}$host"
done <<< "$servers"
for o in "${!hosts[@]}"; do
    echo "$o: ${hosts[$o]}"
done | sort

@@exercise Parallel checks
`check_host` takes 0.3 seconds. Check the 5 hosts **in parallel** so the whole script finishes in about 0.3 s instead of 1.5 s, then print `done`. The output lines of the checks must be **sorted** (parallel jobs finish in any order).
@@starter
check_host() { sleep 0.3; echo "$1 up"; }
hosts=(web01 web02 db01 db02 mail01)
start=$SECONDS

# check every host, in parallel, sorted output

echo "done"
(( SECONDS - start < 2 )) || echo "too slow!"
@@expected
db01 up
db02 up
mail01 up
web01 up
web02 up
done
@@requires
&
wait
@@hint
`for h in "${hosts[@]}"; do check_host "$h" & done; wait`. To sort, put the loop and the `wait` in a group and pipe it: `{ ...; wait; } | sort`.
@@solution
check_host() { sleep 0.3; echo "$1 up"; }
hosts=(web01 web02 db01 db02 mail01)
start=$SECONDS

{
    for h in "${hosts[@]}"; do
        check_host "$h" &
    done
    wait
} | sort

echo "done"
(( SECONDS - start < 2 )) || echo "too slow!"
