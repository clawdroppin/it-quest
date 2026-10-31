@@lesson l2-pipes Pipes, Redirection and the Text Toolbox
@@concepts text-processing
@@text
The Unix philosophy: small tools that each do one thing, connected with **pipes**. Text flows from one into the next.

```bash
printf "web01\ndb01\nweb02\nweb01\nmail01\nweb01\n" | sort | uniq -c | sort -rn
```

*Sort the names, count identical neighbours, sort by that count, biggest first.* That four-command pipeline answers "what appears most often?", one of the most common questions in log analysis.

## Redirection

| Syntax | Means |
|---|---|
| `cmd > file` | stdout to a file (overwrite) |
| `cmd >> file` | stdout, appended |
| `cmd 2> file` | stderr (error messages) to a file |
| `cmd > file 2>&1` | both, to the same file |
| `cmd &> file` | both (Bash shorthand) |
| `cmd < file` | read stdin from a file |
| `cmd > /dev/null` | throw output away |
| `cmd \| tee file` | show it **and** save it |

Every program has three streams: **stdin** (0), **stdout** (1), **stderr** (2). Keeping errors separate means `cmd > results.txt` still shows you the errors.

## The toolbox

```bash
cat > /tmp/itq-access.log <<'EOF'
10.0.0.4 GET /index.html 200
203.0.113.7 GET /wp-login.php 404
10.0.0.4 GET /about.html 200
203.0.113.7 POST /wp-login.php 404
198.51.100.2 GET /index.html 200
203.0.113.7 GET /.env 404
EOF

wc -l < /tmp/itq-access.log                      # how many lines
cut -d' ' -f1 /tmp/itq-access.log | sort -u      # unique IPs (field 1, space-separated)
grep ' 404$' /tmp/itq-access.log | wc -l         # how many 404s
cut -d' ' -f4 /tmp/itq-access.log | sort | uniq -c   # count per status code
head -n 2 /tmp/itq-access.log
tr 'a-z' 'A-Z' < /tmp/itq-access.log | tail -n 1
rm /tmp/itq-access.log
```

| Tool | Does |
|---|---|
| `sort` | sort lines; `-n` numbers, `-r` reverse, `-u` unique, `-k2` by field 2, `-t:` separator |
| `uniq` | merge identical **adjacent** lines (so sort first); `-c` count |
| `cut` | pick fields: `-d','` separator, `-f1,3` fields |
| `tr` | translate characters: `tr -d '\r'` removes Windows line endings |
| `wc` | count lines `-l`, words `-w`, bytes `-c` |
| `head`, `tail` | first / last lines |
| `tee` | copy the stream to a file too |
| `paste`, `column -t` | join lines, align columns |

@@quiz
Why does `uniq -c` usually come after `sort`?
* `uniq` only merges identical lines that are next to each other
- `uniq` sorts in reverse
- `sort` removes empty lines
- It doesn't matter
? Sort first, so all identical lines are neighbours.

@@quiz
Which command saves both normal output and errors of `backup.sh` into `log.txt`?
- `./backup.sh > log.txt`
* `./backup.sh > log.txt 2>&1`
- `./backup.sh 2> log.txt`
- `./backup.sh | log.txt`
? `2>&1` sends stderr to wherever stdout is going. Order matters: first `> log.txt`, then `2>&1`.

@@drill
Write a pipeline that prints the unique values of the **first** field of `ips.txt` (space-separated), sorted.
= cut -d' ' -f1 ips.txt | sort -u
= cut -d " " -f1 ips.txt | sort -u
= cut -d' ' -f1 ips.txt | sort | uniq
= awk '{print $1}' ips.txt | sort -u
? `cut` picks the field, `sort -u` sorts and removes duplicates.

@@exercise Top talkers
`log` holds access log lines (IP, method, path, status). Print the **IP with the most requests** and its count, as `count ip` with a single space (for example `3 203.0.113.7`).
@@starter
log='10.0.0.4 GET /index.html 200
203.0.113.7 GET /wp-login.php 404
10.0.0.4 GET /about.html 200
203.0.113.7 POST /wp-login.php 404
198.51.100.2 GET /index.html 200
203.0.113.7 GET /.env 404'

@@expected
3 203.0.113.7
@@hint
`echo "$log" | cut -d' ' -f1 | sort | uniq -c | sort -rn | head -n 1`. `uniq -c` pads the count with spaces; remove them with `awk '{print $1, $2}'` (or `sed 's/^ *//'`).
@@solution
log='10.0.0.4 GET /index.html 200
203.0.113.7 GET /wp-login.php 404
10.0.0.4 GET /about.html 200
203.0.113.7 POST /wp-login.php 404
198.51.100.2 GET /index.html 200
203.0.113.7 GET /.env 404'
echo "$log" | cut -d' ' -f1 | sort | uniq -c | sort -rn | head -n 1 | awk '{print $1, $2}'

@@exercise Status code report
Using the same kind of log, print how many requests got each status code, as `status: count`, sorted by status code.
@@starter
log='10.0.0.4 GET / 200
10.0.0.5 GET /a 404
10.0.0.4 GET /b 200
10.0.0.6 POST /login 500
10.0.0.7 GET / 200
10.0.0.5 GET /c 404'

@@expected
200: 3
404: 2
500: 1
@@hint
`cut -d' ' -f4 | sort | uniq -c`, then reformat with `while read -r count code; do echo "$code: $count"; done`.
@@solution
log='10.0.0.4 GET / 200
10.0.0.5 GET /a 404
10.0.0.4 GET /b 200
10.0.0.6 POST /login 500
10.0.0.7 GET / 200
10.0.0.5 GET /c 404'
echo "$log" | cut -d' ' -f4 | sort | uniq -c | while read -r count code; do
    echo "$code: $count"
done
