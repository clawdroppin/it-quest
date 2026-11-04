@@lesson le1-text-tools Text Tools: grep, sed, awk, sort and Friends
@@concepts strings, text-processing
@@lang bash
@@text
Most of Linux is text: configuration files, logs, command output. A handful of small tools, joined with pipes, answer questions that would take a spreadsheet and ten minutes otherwise. This lesson covers the classic toolkit and regular expressions, with exercises that pull answers out of a log and a CSV.

## The toolkit

| Tool | Does | Example |
|---|---|---|
| `grep` | lines matching a pattern | `grep -i error app.log` |
| `cut` | columns by delimiter or position | `cut -d, -f2 data.csv` |
| `sort` | sort lines | `sort -t, -k3 -n` (by 3rd field, numerically) |
| `uniq` | collapse **adjacent** duplicates | `sort | uniq -c | sort -rn` (count) |
| `wc` | lines, words, bytes | `wc -l` |
| `head` / `tail` | first / last lines | `tail -f log` follows a growing file |
| `tr` | translate or delete characters | `tr 'a-z' 'A-Z'`, `tr -d '\r'` |
| `sed` | stream editor: substitute, delete, print | `sed 's/old/new/g'`, `sed -n '5,10p'` |
| `awk` | fields and small programs | `awk -F, '$3 > 100 {print $1}'` |
| `xargs` | turn input lines into arguments | `find . -name '*.tmp' -print0 \| xargs -0 rm` |
| `diff` | compare files | `diff -u old new` |
| `jq` | query JSON (install it) | `jq '.items[].name'` |

Useful grep flags: `-i` ignore case, `-v` invert, `-n` line numbers, `-c` count, `-r` recursive, `-l` file names only, `-o` only the match, `-E` extended regex, `-w` whole words, `-A/-B/-C N` context lines.

## Regular expressions

| Pattern | Matches |
|---|---|
| `.` | any character |
| `*` / `+` / `?` | 0+ / 1+ / 0–1 of the previous (`+ ?` need `-E`) |
| `^` / `$` | start / end of line |
| `[abc]`, `[0-9]`, `[^ ]` | one of, a range, not |
| `\b` or `-w` | word boundary |
| `(a\|b)` | alternatives (`-E`) |
| `{3}`, `{2,4}` | repetition counts (`-E`) |

Quote patterns in single quotes so the shell doesn't expand `*` or `$`.

## sed and awk essentials

- `sed 's/foo/bar/g' file` prints the result; `sed -i` edits in place (`sed -i.bak` keeps a backup). Use another delimiter for paths: `s|/old/path|/new|`. `sed '/^#/d'` deletes comment lines.
- awk runs `pattern { action }` per line; fields are `$1`, `$2`…, `$0` is the line, `NF` the field count, `NR` the line number. `-F` sets the separator. `END { … }` runs after the last line, so awk can sum and group: `awk '{sum[$1] += $2} END {for (k in sum) print k, sum[k]}'`.

## Habits

Build pipelines step by step, checking output at each stage. `sort` before `uniq`. Mind Windows line endings (`\r`): `tr -d '\r'` or `dos2unix`. Use `LC_ALL=C sort` for predictable byte order.

@@quiz
Why does `uniq -c` usually follow `sort`?
- uniq only works on numbers
* uniq collapses only adjacent duplicate lines
- sort adds the counts
- It doesn't matter
? Sorting brings duplicates together.

@@quiz
Which command prints lines 5 to 10 of a file?
- `grep 5,10 file`
* `sed -n '5,10p' file`
- `cut -f5-10 file`
- `head -5 -10 file`
? -n suppresses automatic printing.

@@quiz
In awk, what is `NF`?
- The file name
* The number of fields on the current line
- The line number
- The field separator
? `$NF` is the last field.

@@quiz
Which grep finds lines that do NOT contain "DEBUG"?
- `grep DEBUG -n`
* `grep -v DEBUG`
- `grep -i DEBUG`
- `grep ^DEBUG`
? -v inverts the match.

@@quiz
What does `^[0-9]+$` match with grep -E?
* Lines made only of digits
- Lines containing a digit
- Lines starting with a plus
- Empty lines
? Anchored at both ends.

@@flashcards
grep -i :: ignore case
grep -v :: invert match
grep -o :: only the matching part
grep -r :: search recursively
cut -d, -f2 :: second comma field
sort -n / -r / -k :: numeric / reverse / key
uniq -c :: count adjacent duplicates
tr -d '\r' :: strip Windows line endings
sed s///g :: substitute every match
sed -i :: edit in place
awk $1 / $NF :: first / last field
awk NR :: line number
awk END :: after the last line
xargs -0 :: null-separated arguments
^ and $ :: start and end of line

@@drill
What pipeline counts duplicate lines, most frequent first? (three commands)
= sort | uniq -c | sort -rn
= sort | uniq -c | sort -nr
? The classic frequency count.

@@drill
Which awk variable holds the current line number?
= NR
? NF is the field count.

@@exercise Who is hammering the server?
The variable `log` holds web server access lines (`ip - - [time] "METHOD path" status bytes`). Using a pipeline, print the top 3 client IPs by request count, then with awk print the number of 5xx errors per path and the total bytes served.
@@starter
log='203.0.113.5 - - [04/Nov/2026:10:00:01] "GET /index.html" 200 5120
198.51.100.7 - - [04/Nov/2026:10:00:02] "GET /api/orders" 500 120
203.0.113.5 - - [04/Nov/2026:10:00:03] "GET /style.css" 200 2048
192.0.2.44 - - [04/Nov/2026:10:00:04] "POST /api/orders" 201 340
203.0.113.5 - - [04/Nov/2026:10:00:05] "GET /api/orders" 503 90
198.51.100.7 - - [04/Nov/2026:10:00:06] "GET /api/orders" 500 120
203.0.113.5 - - [04/Nov/2026:10:00:07] "GET /index.html" 304 0
192.0.2.44 - - [04/Nov/2026:10:00:08] "GET /api/users" 502 80
198.51.100.7 - - [04/Nov/2026:10:00:09] "GET /index.html" 200 5120'

echo "top clients:"
# Your pipeline here
echo "5xx by path:"
# Your awk here
@@expected
top clients:
      4 203.0.113.5
      3 198.51.100.7
      2 192.0.2.44
5xx by path:
  /api/orders 3
  /api/users 1
total bytes: 13038
@@requires
uniq -c
awk
@@hint
`printf '%s\n' "$log" | cut -d' ' -f1 | sort | uniq -c | sort -rn | head -3`. With awk's default whitespace splitting the path is `$6` (it still has the closing quote: strip it with `gsub(/"/, "", $6)`), the status is `$7` and bytes `$8`; test `$7 >= 500`. Print the per-path counts sorted.
@@solution
log='203.0.113.5 - - [04/Nov/2026:10:00:01] "GET /index.html" 200 5120
198.51.100.7 - - [04/Nov/2026:10:00:02] "GET /api/orders" 500 120
203.0.113.5 - - [04/Nov/2026:10:00:03] "GET /style.css" 200 2048
192.0.2.44 - - [04/Nov/2026:10:00:04] "POST /api/orders" 201 340
203.0.113.5 - - [04/Nov/2026:10:00:05] "GET /api/orders" 503 90
198.51.100.7 - - [04/Nov/2026:10:00:06] "GET /api/orders" 500 120
203.0.113.5 - - [04/Nov/2026:10:00:07] "GET /index.html" 304 0
192.0.2.44 - - [04/Nov/2026:10:00:08] "GET /api/users" 502 80
198.51.100.7 - - [04/Nov/2026:10:00:09] "GET /index.html" 200 5120'

echo "top clients:"
printf '%s\n' "$log" | cut -d' ' -f1 | sort | uniq -c | sort -rn | head -3
echo "5xx by path:"
printf '%s\n' "$log" | awk '{ gsub(/"/, "", $6); bytes += $8 }
    $7 >= 500 { errors[$6]++ }
    END { for (p in errors) print "  " p, errors[p]; print "total bytes:", bytes }' | sort

@@exercise Clean up a CSV
`data` is a messy CSV exported on Windows: it has `\r` line endings, a header, comment lines starting with `#`, blank lines and inconsistent case in the city column. Clean it with `tr`, `sed` and `awk`: strip `\r`, drop comments and blank lines, lower-case the city, then print the total sales per city sorted by total (highest first) and the number of data rows.
@@starter
data=$'name,city,sales\r\n# exported 2026-11-04\r\nAiko,Tokyo,1200\r\nBen,OSAKA,800\r\n\r\nChika,tokyo,450\r\n# test row below\r\nDai,Osaka,300\r\nEmi,Fukuoka,990\r\n'

# Your pipeline here
@@expected
tokyo 1650
osaka 1100
fukuoka 990
rows: 5
@@requires
tr -d
sed
awk -F,
@@hint
`printf '%s' "$data" | tr -d '\r' | sed -e '/^#/d' -e '/^$/d' | awk -F, 'NR > 1 { total[tolower($2)] += $3; rows++ } END { ... }'`. Save the cleaned text in a variable first, print the per-city totals through `sort -k2 -rn`, then count the data rows with a second `awk 'NR > 1'` piped to `wc -l`.
@@solution
data=$'name,city,sales\r\n# exported 2026-11-04\r\nAiko,Tokyo,1200\r\nBen,OSAKA,800\r\n\r\nChika,tokyo,450\r\n# test row below\r\nDai,Osaka,300\r\nEmi,Fukuoka,990\r\n'

clean=$(printf '%s' "$data" | tr -d '\r' | sed -e '/^#/d' -e '/^$/d')
printf '%s\n' "$clean" | awk -F, 'NR > 1 { total[tolower($2)] += $3 } END { for (c in total) print c, total[c] }' | sort -k2 -rn
echo "rows: $(printf '%s\n' "$clean" | awk 'NR > 1' | wc -l)"
