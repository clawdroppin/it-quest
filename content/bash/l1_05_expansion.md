@@lesson l1-expansion Parameter Expansion: String Surgery Without sed
@@concepts variables, strings
@@text
Bash can cut, replace, default and transform variables by itself with **parameter expansion**. It's faster than calling `sed`, `basename` or `cut` (no extra process), and it's in every script you'll read.

## Defaults and checks

```bash
unset name; empty=""
echo "${name:-guest}"        # guest: use a default if unset OR empty
echo "${empty-literal}"      # (empty line): without the colon, only "unset" counts
echo "${name:=web01} $name"  # web01 web01: assign the default too
echo "${name:+--host=$name}" # --host=web01: alternative value only if set and non-empty
```

`${var:?message}` stops the script with an error if `var` is unset or empty, a cheap guard for required settings: `: "${API_TOKEN:?API_TOKEN must be set}"`.

## Cutting prefixes and suffixes

`#` removes from the front, `%` from the back; doubled, they remove the **longest** match:

```bash
path=/var/log/nginx/access.log.2.gz
echo "${path##*/}"   # access.log.2.gz   (basename)
echo "${path%/*}"    # /var/log/nginx    (dirname)
file=${path##*/}
echo "${file%%.*}"   # access            (before the first dot)
echo "${file%.*}"    # access.log.2      (drop the last extension)
echo "${file##*.}"   # gz                (last extension)
echo "${file#*.}"    # log.2.gz          (everything after the first dot)
```

Memory aid: on a US keyboard `#` is left of `%`, so `#` trims the left side.

## Replace, length, substrings, case

```bash
s="billing-api-v2"
echo "${#s}"            # 14
echo "${s/-/_}"         # billing_api-v2   (first match)
echo "${s//-/_}"        # billing_api_v2   (all matches)
echo "${s/#billing/pay}" # pay-api-v2      (only at the start; /% for the end)
echo "${s:0:7}"         # billing          (offset, length)
echo "${s: -2}"         # v2               (space before the minus!)
echo "${s^^} ${s^}"     # BILLING-API-V2 Billing-api-v2
up="WEB01"; echo "${up,,}"   # web01
```

## Indirection

`${!name}` reads the variable **whose name is stored in** `name`, and `${!prefix@}` lists variable names with a prefix:

```bash
APP_PORT=8080; APP_HOST=localhost
key=APP_PORT
echo "${!key}"          # 8080
echo ${!APP_@}          # APP_HOST APP_PORT
```

## Brace expansion (not a variable expansion, but related)

```bash
echo web{01..03}        # web01 web02 web03
echo backup.{tar,zip}   # backup.tar backup.zip
mkdir -p project/{src,tests,docs}
```

@@quiz
What's the difference between `${x:-d}` and `${x-d}`?
* `:-` uses the default when x is unset or empty; `-` only when x is unset
- They're identical
- `-` assigns the default
- `:-` only works with numbers
? The colon adds "or empty".

@@quiz
`f=report.tar.gz`. What does `${f%%.*}` give?
- report.tar
* report
- gz
- tar.gz
? `%%` removes the longest suffix matching `.*`.

@@quiz
What does `${!key}` do?
- Negates key
- Returns the length of key
* Returns the value of the variable whose name is stored in key
- Exports key
? Indirect expansion.

@@drill
Which expansion gives `file` from `file.txt` stored in `$f`?
= ${f%.txt}
= ${f%.*}
? % removes the shortest matching suffix.

@@flashcards
`${v:-d}` :: default if unset or empty
`${v:=d}` :: default and assign
`${v:?msg}` :: error if unset or empty
`${v:+alt}` :: alt if set and non-empty
`${v#p}` / `${v##p}` :: remove shortest/longest prefix
`${v%p}` / `${v%%p}` :: remove shortest/longest suffix
`${v/a/b}` / `${v//a/b}` :: replace first/all
`${#v}` :: length
`${v:o:l}` :: substring
`${v^^}` / `${v,,}` :: upper/lower case
`${!name}` :: indirect expansion

@@exercise Path surgery
For each path, print its directory, base name, stem (before the first dot) and last extension using only parameter expansion (no `basename`, `dirname`, `sed` or `cut`):

- a path without `/` has the directory `.`
- a base name without a dot has the extension `(none)`
@@starter
paths=(/var/log/nginx/access.log.2.gz /etc/hosts backup.tar.gz)
echo TODO
@@expected
/var/log/nginx/access.log.2.gz | dir=/var/log/nginx base=access.log.2.gz stem=access ext=gz
/etc/hosts | dir=/etc base=hosts stem=hosts ext=(none)
backup.tar.gz | dir=. base=backup.tar.gz stem=backup ext=gz
@@requires
##*/
%%.*
@@hint
`base=${p##*/}`; `dir=${p%/*}` but only if `[[ $p == */* ]]`, else `.`; `stem=${base%%.*}`; `ext=${base##*.}` only if `[[ $base == *.* ]]`.
@@solution
paths=(/var/log/nginx/access.log.2.gz /etc/hosts backup.tar.gz)
for p in "${paths[@]}"; do
    base=${p##*/}
    if [[ $p == */* ]]; then dir=${p%/*}; else dir=.; fi
    stem=${base%%.*}
    if [[ $base == *.* ]]; then ext=${base##*.}; else ext="(none)"; fi
    echo "$p | dir=$dir base=$base stem=$stem ext=$ext"
done

@@exercise Settings from prefixed variables
Services read their settings from environment variables named after the service: `billing-api` uses `BILLING_API_PORT`, `BILLING_API_HOST` and so on.

- build the prefix from the service name with case conversion and replacement (`billing-api` → `BILLING_API`)
- for each key `PORT`, `HOST`, `REPLICAS`, read `<PREFIX>_<KEY>` with indirect expansion and fall back to the defaults (80, localhost, 2)
- report whether a value came from the variable, the variable was **unset**, or it was set but **empty** (`[[ -v name ]]` tests whether a variable is set)
- finally print the title-cased name and the first three letters
@@starter
service="billing-api"
BILLING_API_PORT=8080
BILLING_API_REPLICAS=
declare -A defaults=([PORT]=80 [HOST]=localhost [REPLICAS]=2)
echo TODO
@@expected
prefix: BILLING_API
PORT=8080 (from BILLING_API_PORT)
HOST=localhost (default)
REPLICAS=2 (default, BILLING_API_REPLICAS was empty)
title: Billing-api, short: bil
@@requires
${!
^^
@@hint
`prefix=${service^^}; prefix=${prefix//-/_}`. In the loop: `var="${prefix}_$key"`; `if [[ -n ${!var:-} ]]` → from the variable; `elif [[ -v $var ]]` → empty; else unset.
@@solution
service="billing-api"
BILLING_API_PORT=8080
BILLING_API_REPLICAS=
declare -A defaults=([PORT]=80 [HOST]=localhost [REPLICAS]=2)

prefix=${service^^}
prefix=${prefix//-/_}
echo "prefix: $prefix"
for key in PORT HOST REPLICAS; do
    var="${prefix}_$key"
    if [[ -n ${!var:-} ]]; then
        echo "$key=${!var} (from $var)"
    elif [[ -v $var ]]; then
        echo "$key=${defaults[$key]} (default, $var was empty)"
    else
        echo "$key=${defaults[$key]} (default)"
    fi
done
echo "title: ${service^}, short: ${service:0:3}"
