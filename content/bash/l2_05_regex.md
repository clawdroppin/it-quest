@@lesson l2-regex Regular Expressions: grep -E, sed -E and [[ =~ ]]
@@concepts text-processing, strings
@@text
Regular expressions describe text patterns. Admins use them daily to filter logs, validate input and pull fields out of messy output.

## The building blocks (ERE: grep -E, sed -E, awk, [[ =~ ]])

| Pattern | Matches |
|---|---|
| `.` | any one character |
| `[abc]` `[a-z]` `[^0-9]` | one character from (or not from) a set |
| `[[:digit:]]` `[[:alpha:]]` `[[:space:]]` | POSIX classes (portable, locale-aware) |
| `^` `$` | start / end of line |
| `*` `+` `?` | 0+, 1+, 0 or 1 of the previous item |
| `{3}` `{1,3}` | exactly 3, between 1 and 3 |
| `(a\|b)` | group and alternation |
| `\.` | a literal dot |

Basic regex (BRE, plain `grep` and `sed`) needs backslashes for `\+ \? \{ \} \( \) \|`. Use `-E` and forget about it.

## grep options you'll use

```bash
log='GET /api 200
POST /login 401
GET /admin 403'
grep -E ' (401|403)$' <<< "$log"     # lines ending in 401 or 403
grep -oE '[0-9]{3}$' <<< "$log"      # print only the matched part
grep -cE '^GET' <<< "$log"           # count matching lines: 2
grep -vE '^GET' <<< "$log"           # invert
grep -wE 'api' <<< "$log"            # whole words only
```

## sed -E with capture groups

```bash
echo "2026-11-03" | sed -E 's/([0-9]{4})-([0-9]{2})-([0-9]{2})/\3.\2.\1/'   # 03.11.2026
```

## Bash's own matcher: [[ string =~ regex ]]

Matching inside Bash needs no extra process, and the groups land in `BASH_REMATCH`:

```bash
line="ERROR [db01] connection refused (code=111)"
re='^([A-Z]+) \[([a-z0-9]+)\] .*\(code=([0-9]+)\)$'
if [[ $line =~ $re ]]; then
    echo "level=${BASH_REMATCH[1]} host=${BASH_REMATCH[2]} code=${BASH_REMATCH[3]}"
fi
```

Store the regex in a variable and leave it **unquoted** on the right of `=~`: a quoted pattern is matched as a literal string.

## Regex limits

Regexes match text, not meaning: `^[0-9]{1,3}(\.[0-9]{1,3}){3}$` accepts `999.1.1.1`. Match the shape with a regex, then check values with arithmetic. And don't parse HTML or JSON with regexes; use a real parser (`jq`, `xmllint`).

@@quiz
Why put the regex in a variable for `[[ $s =~ $re ]]`?
* Quoting the pattern makes it a literal string, and writing it inline needs awkward escaping; an unquoted variable is treated as a regex
- Variables are faster
- =~ only accepts variables
- To make it case-insensitive
? `re='...'` then `[[ $s =~ $re ]]`.

@@quiz
Which grep option prints only the matching part of each line?
- -c
- -v
* -o
- -w
? Useful for extracting IPs, codes, IDs.

@@quiz
Does `^[0-9]{1,3}(\.[0-9]{1,3}){3}$` fully validate an IPv4 address?
- Yes
* No: it accepts octets above 255, so check the numbers separately
- No: it rejects 10.0.0.1
- Only with grep -P
? Shape by regex, range by arithmetic.

@@flashcards
`grep -E` :: extended regex
`grep -o` :: only the match
`grep -c` :: count matching lines
`grep -v` :: invert match
`grep -w` :: whole words
`sed -E 's/(a)(b)/\2\1/'` :: capture groups
`[[ $s =~ $re ]]` :: Bash regex match
`BASH_REMATCH[n]` :: nth capture group
`[[:digit:]]` :: POSIX digit class
`{m,n}` :: repetition count

@@exercise Parse log lines with BASH_REMATCH
Parse application log lines like `2026-11-03T09:12:44Z ERROR [db01] connection refused (code=111)`:

- one regex with capture groups for the date, time, level, host, message and optional `(code=N)` at the end
- print matching lines as `date time level host: message [code N]` (omit the code part when there is none)
- count lines that don't match and print the totals
@@starter
logs='2026-11-03T09:12:44Z ERROR [db01] connection refused (code=111)
2026-11-03T09:12:45Z INFO [web01] request served
garbage line without structure
2026-11-03T09:13:02Z WARN [cache01] memory at 91% (code=7)'
echo TODO
@@expected
2026-11-03 09:12:44 ERROR db01: connection refused [code 111]
2026-11-03 09:12:45 INFO web01: request served
2026-11-03 09:13:02 WARN cache01: memory at 91% [code 7]
parsed: 3, skipped: 1
@@requires
BASH_REMATCH
=~ $re
@@hint
`re='^([0-9-]+)T([0-9:]+)Z ([A-Z]+) \[([^]]+)\] (.*[^)])( \(code=([0-9]+)\))?$'`: group 5 is the message, group 7 the optional code. Alternatively match the message as `(.*)` and strip a ` (code=N)` suffix with a second match.
@@solution
logs='2026-11-03T09:12:44Z ERROR [db01] connection refused (code=111)
2026-11-03T09:12:45Z INFO [web01] request served
garbage line without structure
2026-11-03T09:13:02Z WARN [cache01] memory at 91% (code=7)'

re='^([0-9-]+)T([0-9:]+)Z ([A-Z]+) \[([^]]+)\] (.*[^)])( \(code=([0-9]+)\))?$'
parsed=0 skipped=0
while IFS= read -r line; do
    if [[ $line =~ $re ]]; then
        out="${BASH_REMATCH[1]} ${BASH_REMATCH[2]} ${BASH_REMATCH[3]} ${BASH_REMATCH[4]}: ${BASH_REMATCH[5]}"
        [[ -n ${BASH_REMATCH[7]} ]] && out+=" [code ${BASH_REMATCH[7]}]"
        echo "$out"
        ((parsed++))
    else
        ((skipped++))
    fi
done <<< "$logs"
echo "parsed: $parsed, skipped: $skipped"

@@exercise Input validators
Write validators that return 0 (valid) or 1 (invalid):

- `is_ipv4`: four dot-separated numbers of 1–3 digits (regex), each ≤ 255, and no leading zeros like `01` (arithmetic and string checks)
- `is_semver`: `MAJOR.MINOR.PATCH` with an optional `-prerelease` like `-rc.1`
- `is_hostname`: labels of letters, digits and hyphens (not starting or ending with a hyphen), separated by dots, total length ≤ 253

Print a table of results.
@@starter
check() { if "$1" "$2"; then r=valid; else r=invalid; fi; printf '%-12s %-22s %s\n' "$1" "$2" "$r"; }
echo TODO
@@expected
is_ipv4      10.0.0.5               valid
is_ipv4      192.168.1.256          invalid
is_ipv4      10.0.01.5              invalid
is_ipv4      10.0.0                 invalid
is_semver    1.4.0                  valid
is_semver    2.0.0-rc.1             valid
is_semver    1.4                    invalid
is_hostname  web01.example.jp       valid
is_hostname  -web01.example.jp      invalid
is_hostname  web_01                 invalid
@@requires
=~
BASH_REMATCH
@@hint
IPv4: `re='^([0-9]{1,3})\.([0-9]{1,3})\.([0-9]{1,3})\.([0-9]{1,3})$'`, then loop over `BASH_REMATCH[1..4]`: reject `> 255` (use `10#$o` to force decimal) and any octet that starts with `0` but isn't just `0`. Hostname label: `[A-Za-z0-9]([A-Za-z0-9-]*[A-Za-z0-9])?`.
@@solution
check() { if "$1" "$2"; then r=valid; else r=invalid; fi; printf '%-12s %-22s %s\n' "$1" "$2" "$r"; }

is_ipv4() {
    local re='^([0-9]{1,3})\.([0-9]{1,3})\.([0-9]{1,3})\.([0-9]{1,3})$' i o
    [[ $1 =~ $re ]] || return 1
    for i in 1 2 3 4; do
        o=${BASH_REMATCH[$i]}
        [[ $o == 0?* ]] && return 1
        (( 10#$o <= 255 )) || return 1
    done
    return 0
}

is_semver() {
    local re='^(0|[1-9][0-9]*)\.(0|[1-9][0-9]*)\.(0|[1-9][0-9]*)(-[0-9A-Za-z.-]+)?$'
    [[ $1 =~ $re ]]
}

is_hostname() {
    local label='[A-Za-z0-9]([A-Za-z0-9-]*[A-Za-z0-9])?'
    local re="^$label(\.$label)*$"
    (( ${#1} <= 253 )) && [[ $1 =~ $re ]]
}

for ip in 10.0.0.5 192.168.1.256 10.0.01.5 10.0.0; do check is_ipv4 "$ip"; done
for v in 1.4.0 2.0.0-rc.1 1.4; do check is_semver "$v"; done
for h in web01.example.jp -web01.example.jp web_01; do check is_hostname "$h"; done
