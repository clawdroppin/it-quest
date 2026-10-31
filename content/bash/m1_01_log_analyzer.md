@@lesson m1-log-analyzer Access Log Analyzer
@@text
A web server writes one line per request to its **access log**. When a site is slow, under attack, or "has errors", the access log answers *who, what and when*, and the fastest way to read it on a server is a few lines of shell.

In this build you'll write **analyze.sh**, a report tool for nginx/Apache logs:

```text
$ ./analyze.sh -n 3 access.log
== access.log ==
Requests: 18    Data: 39983 bytes

-- Top 3 clients --
5 203.0.113.7
4 10.0.0.4
4 198.51.100.2
...
-- Suspicious requests (scanners) --
203.0.113.7 5
```

## The log format

This is the **combined log format**, the default for nginx and Apache:

```text
203.0.113.7 - - [31/Oct/2026:08:15:02 +0000] "GET /wp-login.php HTTP/1.1" 404 153 "-" "curl/8.0"
```

Split on spaces, awk sees: `$1` the client IP, `$4` `[31/Oct/2026:08:15:02`, `$6` `"GET`, `$7` the path, `$9` the status code, `$10` the bytes sent.

## Design

All the logic is in **loglib.sh**, a library of functions that each take the log file as `$1` and print plain lines. `analyze.sh` only parses options and prints the sections, and `tests.sh` checks every function against the sample log. Functions you can test on their own, plus a thin main script: the same design as the Python and PowerShell builds.

One detail: `export LC_ALL=C` makes `sort` compare plain bytes, so results are the same on every machine, whatever its language settings.

@@project log_analyzer Access Log Analyzer
Write the functions in `loglib.sh` until **▶ tests.sh** prints `All tests passed`, then run **▶ analyze.sh**.
@@file loglib.sh
# loglib.sh: functions that analyze a web server access log (combined log format).
# Every function takes the log file as $1 and prints plain lines.
# Fields (awk): $1 IP, $4 [day/month/year:HH:MM:SS, $7 path, $9 status, $10 bytes.

todo() { echo "TODO: not done yet: $1" >&2; return 1; }

# Number of requests (lines).
total_requests() {
    todo total_requests
}

# Sum of the bytes field.
bytes_total() {
    todo bytes_total
}

# The $2 (default 10) IPs with the most requests, as "count ip".
# Most requests first; ties sorted by IP.
top_ips() {
    todo top_ips
}

# Requests per status code, as "status count", sorted by status.
status_counts() {
    todo status_counts
}

# The $2 (default 10) most requested paths that got a 404, as "count path".
# Most first; ties sorted by path.
not_found() {
    todo not_found
}

# IPs that requested typical scanner targets, as "ip count" (most first, ties by IP).
# Targets: wp-login, .env, .git, ../ (path traversal), phpmyadmin.
suspicious() {
    todo suspicious
}

# Requests per hour of the day, as "HH count", sorted by hour.
requests_per_hour() {
    todo requests_per_hour
}
@@file analyze.sh
#!/usr/bin/env bash
# usage: ./analyze.sh [-n TOP] [LOGFILE]
set -euo pipefail
export LC_ALL=C
cd "$(dirname "$0")"
source ./loglib.sh

top=5
while getopts "n:" opt; do
    case $opt in
        n) top=$OPTARG ;;
        *) echo "usage: $0 [-n TOP] [LOGFILE]" >&2; exit 2 ;;
    esac
done
shift $(( OPTIND - 1 ))
log="${1:-access.log}"
[[ -r $log ]] || { echo "can't read $log" >&2; exit 2; }

echo "== $log =="
printf 'Requests: %s    Data: %s bytes\n' "$(total_requests "$log")" "$(bytes_total "$log")"
echo
echo "-- Top $top clients --"
top_ips "$log" "$top"
echo
echo "-- Status codes --"
status_counts "$log"
echo
echo "-- Most requested missing pages --"
not_found "$log" "$top"
echo
echo "-- Requests per hour --"
requests_per_hour "$log" | while read -r hour count; do
    printf '%s:00 %-4s %s\n' "$hour" "$count" "$(printf '%*s' "$count" '' | tr ' ' '#')"
done
echo
echo "-- Suspicious requests (scanners) --"
found=$(suspicious "$log")
echo "${found:-none}"
@@file tests.sh
#!/usr/bin/env bash
# Run: bash tests.sh
cd "$(dirname "$0")"
export LC_ALL=C
source ./loglib.sh
failed=0

assert_eq() {
    if [[ "$1" == "$2" ]]; then
        echo "  ok    $3"
    else
        echo "  FAIL  $3"
        echo "        expected: $(printf '%s' "$2" | tr '\n' '|')"
        echo "        got:      $(printf '%s' "$1" | tr '\n' '|')"
        failed=$(( failed + 1 ))
    fi
}

assert_eq "$(total_requests access.log 2>&1)" "18" "total_requests"
assert_eq "$(bytes_total access.log 2>&1)" "39983" "bytes_total"
assert_eq "$(top_ips access.log 3 2>&1)" $'5 203.0.113.7\n4 10.0.0.4\n4 198.51.100.2' "top_ips (3)"
assert_eq "$(top_ips access.log 2>&1 | wc -l | tr -d ' ')" "6" "top_ips default shows up to 10"
assert_eq "$(status_counts access.log 2>&1)" $'200 7\n302 1\n400 1\n404 7\n500 2' "status_counts"
assert_eq "$(not_found access.log 3 2>&1)" $'2 /old-page\n2 /wp-login.php\n1 /.env' "not_found (3)"
assert_eq "$(suspicious access.log 2>&1)" $'203.0.113.7 5\n192.0.2.99 1' "suspicious"
assert_eq "$(requests_per_hour access.log 2>&1)" $'08 7\n09 6\n10 5' "requests_per_hour"

if (( failed > 0 )); then echo "$failed test(s) failed"; exit 1; fi
echo "All tests passed"
@@file access.log
10.0.0.4 - - [31/Oct/2026:08:01:10 +0000] "GET /index.html HTTP/1.1" 200 5120 "-" "Mozilla/5.0"
10.0.0.4 - - [31/Oct/2026:08:01:11 +0000] "GET /style.css HTTP/1.1" 200 2048 "http://shop.example/index.html" "Mozilla/5.0"
203.0.113.7 - - [31/Oct/2026:08:15:02 +0000] "GET /wp-login.php HTTP/1.1" 404 153 "-" "curl/8.0"
203.0.113.7 - - [31/Oct/2026:08:15:03 +0000] "POST /wp-login.php HTTP/1.1" 404 153 "-" "curl/8.0"
203.0.113.7 - - [31/Oct/2026:08:15:04 +0000] "GET /.env HTTP/1.1" 404 153 "-" "curl/8.0"
198.51.100.2 - - [31/Oct/2026:08:20:00 +0000] "GET /index.html HTTP/1.1" 200 5120 "-" "Mozilla/5.0"
198.51.100.2 - - [31/Oct/2026:08:20:05 +0000] "GET /products HTTP/1.1" 200 8192 "-" "Mozilla/5.0"
10.0.0.9 - - [31/Oct/2026:09:00:01 +0000] "GET /index.html HTTP/1.1" 200 5120 "-" "Mozilla/5.0"
192.0.2.50 - - [31/Oct/2026:09:05:10 +0000] "GET /old-page HTTP/1.1" 404 153 "-" "Mozilla/5.0"
203.0.113.7 - - [31/Oct/2026:09:10:00 +0000] "GET /.git/config HTTP/1.1" 404 153 "-" "curl/8.0"
10.0.0.4 - - [31/Oct/2026:09:30:00 +0000] "GET /products HTTP/1.1" 200 8192 "-" "Mozilla/5.0"
198.51.100.2 - - [31/Oct/2026:09:31:00 +0000] "POST /cart HTTP/1.1" 302 0 "-" "Mozilla/5.0"
192.0.2.99 - - [31/Oct/2026:09:45:00 +0000] "GET /../../etc/passwd HTTP/1.1" 400 0 "-" "python-requests/2.32"
10.0.0.9 - - [31/Oct/2026:10:02:00 +0000] "GET /api/stock HTTP/1.1" 500 512 "-" "Mozilla/5.0"
10.0.0.9 - - [31/Oct/2026:10:02:05 +0000] "GET /api/stock HTTP/1.1" 500 512 "-" "Mozilla/5.0"
10.0.0.4 - - [31/Oct/2026:10:10:00 +0000] "GET /old-page HTTP/1.1" 404 153 "-" "Mozilla/5.0"
198.51.100.2 - - [31/Oct/2026:10:15:00 +0000] "GET /checkout HTTP/1.1" 200 4096 "-" "Mozilla/5.0"
203.0.113.7 - - [31/Oct/2026:10:20:00 +0000] "GET /phpmyadmin/ HTTP/1.1" 404 153 "-" "curl/8.0"
@@text
> Sorting with ties: `sort -k1,1nr -k2,2` sorts by field 1 as a number, reversed, and then by field 2 as text. For counting, the `sort | uniq -c` pipeline or an awk array `c[$1]++` both work; awk prints its arrays in random order, so sort afterwards.

@@project log_analyzer_finished Access Log Analyzer: the finished version
The complete project. Point it at a real log: on a server, `sudo ./analyze.sh /var/log/nginx/access.log`. Extensions: a `-s HH` option for one hour only, a "top user agents" section (needs splitting on `"` instead of spaces: `awk -F'"' '{print $6}'`), reading `.gz` rotated logs with `zcat`, or printing ready-to-paste `ufw deny from <ip>` lines for the scanners.
@@file loglib.sh
# loglib.sh: functions that analyze a web server access log (combined log format).
# Every function takes the log file as $1 and prints plain lines.
# Fields (awk): $1 IP, $4 [day/month/year:HH:MM:SS, $7 path, $9 status, $10 bytes.

# Number of requests (lines).
total_requests() {
    awk 'END { print NR }' "$1"
}

# Sum of the bytes field.
bytes_total() {
    awk '{ s += $10 } END { print s + 0 }' "$1"
}

# The $2 (default 10) IPs with the most requests, as "count ip".
# Most requests first; ties sorted by IP.
top_ips() {
    awk '{ print $1 }' "$1" | sort | uniq -c | sort -k1,1nr -k2,2 | head -n "${2:-10}" | awk '{ print $1, $2 }'
}

# Requests per status code, as "status count", sorted by status.
status_counts() {
    awk '{ c[$9]++ } END { for (s in c) print s, c[s] }' "$1" | sort -n
}

# The $2 (default 10) most requested paths that got a 404, as "count path".
# Most first; ties sorted by path.
not_found() {
    awk '$9 == 404 { print $7 }' "$1" | sort | uniq -c | sort -k1,1nr -k2,2 | head -n "${2:-10}" | awk '{ print $1, $2 }'
}

# IPs that requested typical scanner targets, as "ip count" (most first, ties by IP).
# Targets: wp-login, .env, .git, ../ (path traversal), phpmyadmin.
suspicious() {
    awk '$7 ~ /wp-login|\.env|\.git|\.\.\/|phpmyadmin/ { c[$1]++ } END { for (ip in c) print ip, c[ip] }' "$1" |
        sort -k2,2nr -k1,1
}

# Requests per hour of the day, as "HH count", sorted by hour.
requests_per_hour() {
    awk '{ split($4, t, ":"); c[t[2]]++ } END { for (h in c) print h, c[h] }' "$1" | sort
}
@@file analyze.sh
#!/usr/bin/env bash
# usage: ./analyze.sh [-n TOP] [LOGFILE]
set -euo pipefail
export LC_ALL=C
cd "$(dirname "$0")"
source ./loglib.sh

top=5
while getopts "n:" opt; do
    case $opt in
        n) top=$OPTARG ;;
        *) echo "usage: $0 [-n TOP] [LOGFILE]" >&2; exit 2 ;;
    esac
done
shift $(( OPTIND - 1 ))
log="${1:-access.log}"
[[ -r $log ]] || { echo "can't read $log" >&2; exit 2; }

echo "== $log =="
printf 'Requests: %s    Data: %s bytes\n' "$(total_requests "$log")" "$(bytes_total "$log")"
echo
echo "-- Top $top clients --"
top_ips "$log" "$top"
echo
echo "-- Status codes --"
status_counts "$log"
echo
echo "-- Most requested missing pages --"
not_found "$log" "$top"
echo
echo "-- Requests per hour --"
requests_per_hour "$log" | while read -r hour count; do
    printf '%s:00 %-4s %s\n' "$hour" "$count" "$(printf '%*s' "$count" '' | tr ' ' '#')"
done
echo
echo "-- Suspicious requests (scanners) --"
found=$(suspicious "$log")
echo "${found:-none}"
@@file tests.sh
#!/usr/bin/env bash
# Run: bash tests.sh
cd "$(dirname "$0")"
export LC_ALL=C
source ./loglib.sh
failed=0

assert_eq() {
    if [[ "$1" == "$2" ]]; then
        echo "  ok    $3"
    else
        echo "  FAIL  $3"
        echo "        expected: $(printf '%s' "$2" | tr '\n' '|')"
        echo "        got:      $(printf '%s' "$1" | tr '\n' '|')"
        failed=$(( failed + 1 ))
    fi
}

assert_eq "$(total_requests access.log 2>&1)" "18" "total_requests"
assert_eq "$(bytes_total access.log 2>&1)" "39983" "bytes_total"
assert_eq "$(top_ips access.log 3 2>&1)" $'5 203.0.113.7\n4 10.0.0.4\n4 198.51.100.2' "top_ips (3)"
assert_eq "$(top_ips access.log 2>&1 | wc -l | tr -d ' ')" "6" "top_ips default shows up to 10"
assert_eq "$(status_counts access.log 2>&1)" $'200 7\n302 1\n400 1\n404 7\n500 2' "status_counts"
assert_eq "$(not_found access.log 3 2>&1)" $'2 /old-page\n2 /wp-login.php\n1 /.env' "not_found (3)"
assert_eq "$(suspicious access.log 2>&1)" $'203.0.113.7 5\n192.0.2.99 1' "suspicious"
assert_eq "$(requests_per_hour access.log 2>&1)" $'08 7\n09 6\n10 5' "requests_per_hour"

if (( failed > 0 )); then echo "$failed test(s) failed"; exit 1; fi
echo "All tests passed"
@@file access.log
10.0.0.4 - - [31/Oct/2026:08:01:10 +0000] "GET /index.html HTTP/1.1" 200 5120 "-" "Mozilla/5.0"
10.0.0.4 - - [31/Oct/2026:08:01:11 +0000] "GET /style.css HTTP/1.1" 200 2048 "http://shop.example/index.html" "Mozilla/5.0"
203.0.113.7 - - [31/Oct/2026:08:15:02 +0000] "GET /wp-login.php HTTP/1.1" 404 153 "-" "curl/8.0"
203.0.113.7 - - [31/Oct/2026:08:15:03 +0000] "POST /wp-login.php HTTP/1.1" 404 153 "-" "curl/8.0"
203.0.113.7 - - [31/Oct/2026:08:15:04 +0000] "GET /.env HTTP/1.1" 404 153 "-" "curl/8.0"
198.51.100.2 - - [31/Oct/2026:08:20:00 +0000] "GET /index.html HTTP/1.1" 200 5120 "-" "Mozilla/5.0"
198.51.100.2 - - [31/Oct/2026:08:20:05 +0000] "GET /products HTTP/1.1" 200 8192 "-" "Mozilla/5.0"
10.0.0.9 - - [31/Oct/2026:09:00:01 +0000] "GET /index.html HTTP/1.1" 200 5120 "-" "Mozilla/5.0"
192.0.2.50 - - [31/Oct/2026:09:05:10 +0000] "GET /old-page HTTP/1.1" 404 153 "-" "Mozilla/5.0"
203.0.113.7 - - [31/Oct/2026:09:10:00 +0000] "GET /.git/config HTTP/1.1" 404 153 "-" "curl/8.0"
10.0.0.4 - - [31/Oct/2026:09:30:00 +0000] "GET /products HTTP/1.1" 200 8192 "-" "Mozilla/5.0"
198.51.100.2 - - [31/Oct/2026:09:31:00 +0000] "POST /cart HTTP/1.1" 302 0 "-" "Mozilla/5.0"
192.0.2.99 - - [31/Oct/2026:09:45:00 +0000] "GET /../../etc/passwd HTTP/1.1" 400 0 "-" "python-requests/2.32"
10.0.0.9 - - [31/Oct/2026:10:02:00 +0000] "GET /api/stock HTTP/1.1" 500 512 "-" "Mozilla/5.0"
10.0.0.9 - - [31/Oct/2026:10:02:05 +0000] "GET /api/stock HTTP/1.1" 500 512 "-" "Mozilla/5.0"
10.0.0.4 - - [31/Oct/2026:10:10:00 +0000] "GET /old-page HTTP/1.1" 404 153 "-" "Mozilla/5.0"
198.51.100.2 - - [31/Oct/2026:10:15:00 +0000] "GET /checkout HTTP/1.1" 200 4096 "-" "Mozilla/5.0"
203.0.113.7 - - [31/Oct/2026:10:20:00 +0000] "GET /phpmyadmin/ HTTP/1.1" 404 153 "-" "curl/8.0"
@@text
🎉 You can now answer "what's happening on this web server?" in seconds, on any Linux box, with no extra software. That's a skill people notice in SOC, sysadmin and DevOps interviews.
