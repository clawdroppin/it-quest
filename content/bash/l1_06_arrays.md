@@lesson l1-arrays Arrays in Depth: Lists, Dictionaries and namerefs
@@concepts lists, maps
@@text
Arrays are how Bash scripts handle lists of hosts, files and arguments **safely**, even when items contain spaces.

## Indexed arrays

```bash
hosts=(web01 web02 db01)
hosts+=(cache01)                       # append
echo "${#hosts[@]}"                    # 4: number of items
echo "${hosts[0]} ${hosts[-1]}"        # web01 cache01
echo "${hosts[@]:1:2}"                 # web02 db01: slice (offset, count)
echo "${!hosts[@]}"                    # 0 1 2 3: the indices
unset 'hosts[1]'                       # leaves a hole: indices 0 2 3
hosts=("${hosts[@]}")                  # re-pack: 0 1 2
```

## "${a[@]}" vs "${a[*]}"

- `"${a[@]}"` expands to **one word per item**, keeping spaces inside items. Use it almost always.
- `"${a[*]}"` joins all items into **one** string, separated by the first character of `IFS`: handy for joining: `(IFS=,; echo "${a[*]}")`.
- Unquoted `${a[@]}` splits items on spaces and expands globs: a classic bug.

## Filling arrays

```bash
mapfile -t lines < config.txt              # one item per line (-t drops the newlines)
mapfile -t users < <(cut -d: -f1 /etc/passwd)
read -ra fields <<< "10.0.0.5   web01  prod" # split a string on whitespace
IFS=, read -ra cols <<< "a,b,c"            # split on commas
files=(*.log)                              # glob into an array
```

Don't use `arr=($(command))`: it splits on every space and expands `*`.

## Associative arrays (dictionaries)

```bash
declare -A port=([ssh]=22 [https]=443)
port[dns]=53
echo "${port[https]} ${#port[@]}"          # 443 3
for svc in "${!port[@]}"; do echo "$svc"; done   # keys: order is NOT defined
[[ -v port[ftp] ]] || echo "no ftp"
```

Sort keys when order matters: `printf '%s\n' "${!port[@]}" | sort`.

## Passing arrays to functions: namerefs

Bash can't pass arrays by value. A **nameref** (`local -n`) makes a local name an alias for the caller's variable:

```bash
add_host() {
    local -n _list=$1        # _list now IS the caller's array
    _list+=("$2")
}
servers=()
add_host servers web01
echo "${servers[@]}"         # web01
```

Give namerefs unusual names (`_list`): if the caller's array were also called `_list`, the reference would point at itself.

@@quiz
Why loop with `for h in "${hosts[@]}"` rather than `for h in ${hosts[@]}`?
* The quoted form keeps each item as one word, even if it contains spaces
- The unquoted form is a syntax error
- Quoting makes it faster
- They're identical
? Unquoted expansions are split and glob-expanded.

@@quiz
In what order does `for k in "${!dict[@]}"` visit the keys of an associative array?
- Insertion order
- Alphabetical
* No guaranteed order
- Reverse insertion order
? Sort the keys when output order matters.

@@quiz
What does `local -n ref=$1` create?
- A copy of the array
* A reference: reads and writes go to the variable named in $1
- A read-only variable
- A number variable
? Namerefs need Bash 4.3+.

@@flashcards
`a+=(x)` :: append to an array
`${#a[@]}` :: number of items
`${a[-1]}` :: last item
`${a[@]:o:n}` :: slice
`${!a[@]}` :: indices or keys
`"${a[@]}"` :: one word per item
`"${a[*]}"` :: items joined with IFS
`mapfile -t arr` :: read lines into an array
`read -ra arr <<< "$s"` :: split a string into an array
`declare -A` :: associative array
`local -n` :: nameref

@@exercise Working with a host list
Practise the core array operations:

- fill `hosts` from the command output with `mapfile -t` and process substitution
- print the count, the first and last items, a slice, and the items joined with commas
- remove the second item, show the resulting indices, then re-pack the array
- append `mail01`, and collect the hosts starting with `web` into a new array
- split a whitespace-separated string with `read -ra`
- show why quoting matters with two file names, one of which contains a space
@@starter
list_hosts() { printf '%s\n' web01 web02 db01 cache01 web03; }
echo TODO
@@expected
count: 5
first: web01, last: web03
slice 1-2: web02 db01
joined: web01,web02,db01,cache01,web03
after unset: 4 items, indices: 0 2 3 4
repacked indices: 0 1 2 3
web hosts: web01 web03
fields: 3, second: web01
unquoted loop: 3 items, quoted loop: 2 items
@@requires
mapfile -t
read -ra
${!hosts[*]}
@@hint
`mapfile -t hosts < <(list_hosts)`. Join inside a subshell so the IFS change doesn't leak: `(IFS=,; echo "joined: ${hosts[*]}")`. For the quoting demo, count iterations of `for f in ${files[@]}` and `for f in "${files[@]}"` with `files=("my report.txt" b.txt)`.
@@solution
list_hosts() { printf '%s\n' web01 web02 db01 cache01 web03; }

mapfile -t hosts < <(list_hosts)
echo "count: ${#hosts[@]}"
echo "first: ${hosts[0]}, last: ${hosts[-1]}"
echo "slice 1-2: ${hosts[*]:1:2}"
(IFS=,; echo "joined: ${hosts[*]}")

unset 'hosts[1]'
echo "after unset: ${#hosts[@]} items, indices: ${!hosts[*]}"
hosts=("${hosts[@]}")
echo "repacked indices: ${!hosts[*]}"

hosts+=(mail01)
web=()
for h in "${hosts[@]}"; do
    [[ $h == web* ]] && web+=("$h")
done
echo "web hosts: ${web[*]}"

read -ra parts <<< "10.0.0.5   web01  prod"
echo "fields: ${#parts[@]}, second: ${parts[1]}"

files=("my report.txt" b.txt)
unquoted=0 quoted=0
for f in ${files[@]}; do ((unquoted++)); done
for f in "${files[@]}"; do ((quoted++)); done
echo "unquoted loop: $unquoted items, quoted loop: $quoted items"

@@exercise Group hosts by role with namerefs
Write two helper functions that work on **any** array whose name you pass:

- `add_unique NAME VALUE` appends VALUE to the array NAME unless it's already there (return 1 for a duplicate)
- `sort_array NAME` sorts the array NAME in place (`LC_ALL=C sort`)

Read the `host role` lines, keep a list of unique hosts, group hosts by role in an associative array (space-separated values), and print the roles in sorted order.
@@starter
data='web01 web
db01 db
web02 web
cache01 cache
db02 db
web01 web'
echo TODO
@@expected
duplicate ignored: web01
unique hosts: cache01 db01 db02 web01 web02
cache: cache01
db: db01 db02
web: web01 web02
@@requires
local -n
declare -A
@@hint
`add_unique() { local -n _arr=$1; local x; for x in "${_arr[@]}"; do [[ $x == "$2" ]] && return 1; done; _arr+=("$2"); }`. `sort_array() { local -n _arr=$1; mapfile -t _arr < <(printf '%s\n' "${_arr[@]}" | LC_ALL=C sort); }`. Read the data with `while read -r host role; do ... done <<< "$data"` and only add to the role group when `add_unique` succeeds. Trim the trailing space when printing a group: `${by_role[$r]% }`.
@@solution
data='web01 web
db01 db
web02 web
cache01 cache
db02 db
web01 web'

add_unique() {
    local -n _arr=$1
    local x
    for x in "${_arr[@]}"; do
        [[ $x == "$2" ]] && return 1
    done
    _arr+=("$2")
}

sort_array() {
    local -n _arr=$1
    mapfile -t _arr < <(printf '%s\n' "${_arr[@]}" | LC_ALL=C sort)
}

hosts=()
declare -A by_role=()
while read -r host role; do
    if add_unique hosts "$host"; then
        by_role[$role]+="$host "
    else
        echo "duplicate ignored: $host"
    fi
done <<< "$data"

sort_array hosts
echo "unique hosts: ${hosts[*]}"
roles=("${!by_role[@]}")
sort_array roles
for r in "${roles[@]}"; do
    echo "$r: ${by_role[$r]% }"
done
