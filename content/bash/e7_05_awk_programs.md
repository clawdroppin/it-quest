@@lesson e7-awk awk as a Programming Language
@@concepts text-processing, maps
@@text
Most people use awk for `{print $3}`. But awk is a complete little language with variables, associative arrays, functions and printf, built for one job: **turning lines of text into reports**. A 15-line awk program often replaces a 60-line Bash loop and runs 100× faster on big files.

## The model

An awk program is a list of `pattern { action }` rules. For each input line (a **record**), awk splits it into **fields** `$1..$NF` and runs every rule whose pattern matches.

```bash
printf '%s\n' "web01 200 512" "db01 500 80" "web02 200 1024" |
awk 'BEGIN { print "start" }
     $2 >= 500 { errors++ }
     { bytes += $3 }
     END { printf "errors=%d bytes=%d lines=%d\n", errors, bytes, NR }'
```

| Variable | Meaning |
|---|---|
| `NR` / `FNR` | record number overall / within the current file |
| `NF` | number of fields (`$NF` is the last field) |
| `FS` / `OFS` | input / output field separator (`-F,` sets FS) |
| `RS` / `ORS` | record separators (`RS=""` = paragraph mode) |
| `FILENAME` | current file |

Uninitialised variables are `0` or `""`, so `count[$1]++` just works.

## Patterns

```bash,norun
awk '/ERROR/'                    # regex on the whole line (default action: print)
awk '$3 ~ /^5[0-9][0-9]$/'       # regex on one field; !~ for "doesn't match"
awk 'NR > 1'                     # skip a header
awk '/BEGIN CERT/,/END CERT/'    # a range: from one match to the next
awk 'NR == FNR { seen[$1]; next } $1 in seen' allow.txt data.txt   # join two files
```

The last line is a classic: while reading the first file (`NR == FNR`), store keys; `next` skips the other rules. For the second file, print lines whose key was seen.

## Associative arrays

```bash
printf '%s\n' "web 3" "db 5" "web 4" |
awk '{ sum[$1] += $2; n[$1]++ }
     END { for (k in sum) printf "%s avg=%.1f\n", k, sum[k] / n[k] }' | LC_ALL=C sort
```

`for (k in arr)` has no defined order: pipe to `sort` (or use gawk's `PROCINFO["sorted_in"]`). `delete arr[k]` removes a key; `(k in arr)` tests without creating it. Multi-dimensional keys use `arr[a, b]`, joined with `SUBSEP`.

## Functions and string tools

```bash
awk 'function pct(part, total) { return total ? 100 * part / total : 0 }
     BEGIN {
        s = "billing-api-v2"
        print length(s), toupper(substr(s, 1, 7)), index(s, "api")
        n = split(s, parts, "-"); print n, parts[3]
        gsub(/-/, "_", s); print s
        if (match("took 245ms", /[0-9]+/)) print substr("took 245ms", RSTART, RLENGTH)
        printf "%.1f%%\n", pct(3, 8)
     }'
```

Extra local variables in awk functions are declared as **extra parameters** by convention: `function f(a, b,    tmp, i)`.

## Passing shell values in

Never splice shell variables into the program text. Use `-v`:

```bash
limit=500
printf '%s\n' "a 200" "b 503" | awk -v limit="$limit" '$2 >= limit { print $1 }'
```

@@quiz
What does `NR == FNR` identify in a two-file awk command?
* Lines from the first file, where the overall and per-file record numbers are equal
- The last line of each file
- Lines that appear in both files
- Empty lines
? Used for lookups: load file 1, filter file 2.

@@quiz
How should you pass a shell variable into awk?
- `awk "{ if (\$2 > $limit) print }"`
* `awk -v limit="$limit" '$2 > limit'`
- export it and use $limit inside awk
- awk can't read shell variables
? -v avoids quoting bugs and code injection.

@@quiz
In what order does `for (k in arr)` visit keys?
- Insertion order
- Sorted order
* Unspecified: sort afterwards if order matters
- Reverse order
? gawk can set PROCINFO["sorted_in"].

@@flashcards
`pattern { action }` :: an awk rule
`BEGIN` / `END` :: before / after all input
`NR` / `FNR` :: record number overall / in this file
`NF` / `$NF` :: field count / last field
`-F,` :: comma field separator
`OFS` :: output field separator
`next` :: skip to the next record
`arr[k]++` :: count by key
`(k in arr)` :: key test
`split(s, a, sep)` :: split into an array
`gsub(re, repl, s)` :: replace all
`match(s, re)` :: sets RSTART, RLENGTH
`-v name=value` :: pass a value in

@@exercise An access log report in one awk program
Write a single awk program (run with `awk -f` on a here-doc, or inline) that reads the access log and prints:

- total requests and the error rate (status ≥ 500) with one decimal
- requests and average response time (last field, in ms) per path, sorted by path
- the slowest request (path and time)
- hits per hour (the hour is characters 13–14 of the timestamp field), sorted
@@starter
log='10.0.0.1 [03/Nov/2026:09:01:12] GET /api/users 200 120
10.0.0.2 [03/Nov/2026:09:15:40] GET /api/orders 500 900
10.0.0.1 [03/Nov/2026:09:31:02] POST /api/orders 201 340
10.0.0.3 [03/Nov/2026:10:02:55] GET /api/users 200 80
10.0.0.2 [03/Nov/2026:10:45:10] GET /health 200 5
10.0.0.4 [03/Nov/2026:11:12:01] GET /api/orders 503 1500
10.0.0.1 [03/Nov/2026:11:30:30] GET /api/users 200 100'
echo TODO
@@expected
requests: 7, errors: 2 (28.6%)
/api/orders 3 avg=913ms
/api/users 3 avg=100ms
/health 1 avg=5ms
slowest: /api/orders 1500ms
hour 09: 3
hour 10: 2
hour 11: 2
@@requires
awk
END
substr
@@hint
In the main rule: `n++; if ($5 >= 500) err++; hits[$4]++; ms[$4] += $6; if ($6 > max) { max = $6; slow = $4 }; hour[substr($2, 14, 2)]++` (the `[` is character 1 of `$2`). Print the per-path and per-hour lines from `END` with a sort for each, or print them with a sort key prefix and pipe everything through `LC_ALL=C sort`. Round the average with `%.0f`.
@@solution
log='10.0.0.1 [03/Nov/2026:09:01:12] GET /api/users 200 120
10.0.0.2 [03/Nov/2026:09:15:40] GET /api/orders 500 900
10.0.0.1 [03/Nov/2026:09:31:02] POST /api/orders 201 340
10.0.0.3 [03/Nov/2026:10:02:55] GET /api/users 200 80
10.0.0.2 [03/Nov/2026:10:45:10] GET /health 200 5
10.0.0.4 [03/Nov/2026:11:12:01] GET /api/orders 503 1500
10.0.0.1 [03/Nov/2026:11:30:30] GET /api/users 200 100'

awk '
{
    n++
    if ($5 >= 500) err++
    hits[$4]++
    ms[$4] += $6
    if ($6 > max) { max = $6; slow = $4 }
    hour[substr($2, 14, 2)]++
}
END {
    printf "1 requests: %d, errors: %d (%.1f%%)\n", n, err, 100 * err / n
    for (p in hits) printf "2 %s %d avg=%.0fms\n", p, hits[p], ms[p] / hits[p]
    printf "3 slowest: %s %dms\n", slow, max
    for (h in hour) printf "4 hour %s: %d\n", h, hour[h]
}' <<< "$log" | LC_ALL=C sort | cut -d' ' -f2-

@@exercise Join two files and reshape them
You have an inventory (`host,role,datacenter`) and a monitoring export (`host;metric;value`). With awk:

1. join them with the `NR == FNR` idiom (different separators per file: set `FS` per file with `FNR == 1`)
2. print a CSV with header `host,role,dc,cpu,mem`, one line per host from the inventory, `-` for missing metrics (hosts in the same order as the inventory)
3. print hosts in the monitoring data that aren't in the inventory
4. print the average CPU per datacenter with a helper function `avg(sum, n)`, sorted
@@starter
inventory='host,role,dc
web01,web,tokyo
web02,web,osaka
db01,db,tokyo
cache01,cache,osaka'
metrics='web01;cpu;41
web01;mem;63
db01;cpu;77
db01;mem;88
web02;cpu;12
ghost01;cpu;99'
echo TODO
@@expected
host,role,dc,cpu,mem
web01,web,tokyo,41,63
web02,web,osaka,12,-
db01,db,tokyo,77,88
cache01,cache,osaka,-,-
unknown host in metrics: ghost01
avg cpu osaka: 12.0
avg cpu tokyo: 59.0
@@requires
NR == FNR
function avg
@@hint
Read the metrics first: `awk 'FNR == 1 { FS = (NR == FNR) ? ";" : "," ; $0 = $0 } NR == FNR { m[$1, $2] = $3; seen[$1]; next } FNR > 1 { ... }' <(printf '%s\n' "$metrics") <(printf '%s\n' "$inventory")`. Setting `FS` takes effect for the next record, so re-split the current one with `$0 = $0`. Use `((h, "cpu") in m)` to test for a metric. Print the unknown hosts and averages from END; the averages need sorting, so print them last through `sort` in a second step, or prefix sort keys.
@@solution
inventory='host,role,dc
web01,web,tokyo
web02,web,osaka
db01,db,tokyo
cache01,cache,osaka'
metrics='web01;cpu;41
web01;mem;63
db01;cpu;77
db01;mem;88
web02;cpu;12
ghost01;cpu;99'

awk '
function avg(sum, n) { return n ? sum / n : 0 }
function get(h, metric) { return ((h, metric) in m) ? m[h, metric] : "-" }
FNR == 1 { FS = (NR == FNR) ? ";" : ","; $0 = $0 }
NR == FNR { m[$1, $2] = $3; inmetrics[$1]; next }
FNR == 1 { print "host,role,dc,cpu,mem"; next }
{
    known[$1]
    print $1 "," $2 "," $3 "," get($1, "cpu") "," get($1, "mem")
    if (($1, "cpu") in m) { cpu[$3] += m[$1, "cpu"]; cnt[$3]++ }
}
END {
    for (h in inmetrics) if (!(h in known)) print "unknown host in metrics: " h
    for (dc in cpu) printf "avg cpu %s: %.1f\n", dc, avg(cpu[dc], cnt[dc]) | "LC_ALL=C sort"
}' <(printf '%s\n' "$metrics") <(printf '%s\n' "$inventory")
