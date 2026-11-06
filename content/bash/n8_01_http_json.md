@@lesson n8-http HTTP APIs from Bash: curl, Retries and JSON
@@concepts networking, json, errors
@@text
Admin scripts constantly talk to HTTP APIs: health endpoints, cloud APIs, chat webhooks, GitHub. `curl` does the talking, `jq` handles JSON, and your script handles **failure**: timeouts, rate limits and 5xx responses.

## curl options for scripts

```bash,norun
curl -fsS --max-time 10 https://api.example.jp/health
#  -f  fail on HTTP >= 400 (exit 22) instead of printing the error page
#  -s  silent (no progress bar)   -S  but still show errors
curl -sS -o body.json -w '%{http_code}' https://api.example.jp/users   # status code to stdout
curl -sS -X POST -H 'Content-Type: application/json' \
     -d '{"text":"deploy finished"}' "$WEBHOOK_URL"
curl -sS -H "Authorization: Bearer $TOKEN" https://api.example.jp/me
curl -sS --retry 3 --retry-delay 2 --retry-all-errors https://...
curl -sSL -o app.tgz https://.../app.tgz && sha256sum -c app.tgz.sha256
curl -sS --get --data-urlencode "q=disk full" https://api.example.jp/search
```

Secrets: `-H "Authorization: ..."` puts the token on the command line, visible in `ps` to other users on a shared machine. Read headers from a file instead: `curl -H @headers.txt` (curl 7.55+), or use a `.netrc` with `--netrc-file`.

## Retry with exponential backoff

`--retry` covers simple cases. When you need control (retry on 429 and 5xx but not on 4xx, respect a budget), write the loop yourself:

```bash,norun
for attempt in 1 2 3 4 5; do
    code=$(curl -sS -o "$out" -w '%{http_code}' "$url") || code=000
    case $code in
        2??) break ;;
        429|5??|000) sleep $(( 2 ** attempt )) ;;   # transient: back off and retry
        *) echo "giving up: HTTP $code" >&2; exit 1 ;; # 4xx: our request is wrong
    esac
done
```

## JSON with jq

`jq` is the standard tool (install it with your package manager). A quick tour:

```bash,norun
jq '.name' user.json                      # a field
jq -r '.items[].name' list.json           # -r: raw strings, one per line
jq -r '.items[] | select(.status == "down") | .host' hosts.json
jq -r '.items[] | [.host, .cpu] | @tsv' hosts.json    # tab-separated for read/awk
jq '{host: .name, ok: (.status == "up")}' x.json      # reshape
jq -n --arg t "$text" '{text: $t}'        # BUILD JSON safely from shell values
```

The last line matters: never build JSON by pasting variables into a string. A quote or newline in `$text` breaks it (or injects fields). `jq -n --arg` escapes properly.

## Without jq

On a minimal box you might not have `jq`. For **flat, known** JSON (a health check's `{"status":"ok","version":"1.4.0"}`), a Bash regex is acceptable; for anything nested, use jq or Python. This lesson's exercises use small helpers so they run anywhere, but in production, reach for jq.

To **produce** JSON without jq, escape `\` and `"` and control characters yourself; that's what the exercise does.

@@quiz
What does `curl -f` change?
* HTTP errors (status ≥ 400) make curl exit with a non-zero status instead of succeeding
- It follows redirects
- It forces HTTPS
- It saves to a file
? Without -f, a 500 page is a "successful" download.

@@quiz
Which responses should a retry loop retry?
- All non-200 responses
* Transient ones: timeouts/connection errors, 429 and 5xx; not other 4xx
- Only 404
- None: retrying is unsafe
? A 400 or 403 won't fix itself.

@@quiz
Why use `jq -n --arg msg "$msg" '{text: $msg}'` instead of `"{\"text\":\"$msg\"}"`?
- It's shorter
* jq escapes quotes, backslashes and newlines in the value, so the JSON is always valid and can't be injected into
- curl requires jq
- Bash can't print braces
? Build structured data with a tool that knows the format.

@@drill
Which `curl` option makes it fail with an error on HTTP 4xx and 5xx responses?
= -f
= --fail
= --fail-with-body
? Otherwise curl exits 0 for an error page.

@@flashcards
`curl -fsS` :: fail on HTTP errors, quiet, show errors
`-w '%{http_code}'` :: print the status code
`--max-time` :: overall timeout
`--retry N` :: built-in retries
`-H @file` :: headers from a file
`--data-urlencode` :: encode a query value
exponential backoff :: wait 2, 4, 8... seconds
`jq -r` :: raw string output
`select(...)` :: jq filter
`@tsv` :: jq tab-separated output
`jq -n --arg` :: build JSON safely

@@exercise A retrying API client
`fake_curl URL` stands in for `curl -sS -w '\n%{http_code}'`: it prints a body, a newline and the status code, failing a few times first. Write `fetch URL`:

- split the response into body and status code with parameter expansion (last line = code)
- 2xx: print the body and return 0
- 429 and 5xx: retry up to 4 attempts with exponential backoff (record the delays in a `delays` array instead of sleeping)
- other 4xx: give up immediately
- print a log line per attempt to stderr, and summarise the delays at the end
@@starter
declare -A calls=()
fake_curl() {
    local n=$(( ${calls[$1]:-0} + 1 )); calls[$1]=$n
    case $1 in
        /health) printf '{"status":"ok"}\n200' ;;
        /flaky) if (( n < 3 )); then printf 'Service Unavailable\n503'; else printf '{"items":3}\n200'; fi ;;
        /limited) printf 'Too Many Requests\n429' ;;
        /missing) printf 'Not Found\n404' ;;
    esac
}
echo TODO
@@expected
/health -> {"status":"ok"} (delays: none)
/flaky -> {"items":3} (delays: 2 4)
/limited failed with HTTP 429 after 4 attempts (delays: 2 4 8)
/missing failed with HTTP 404 after 1 attempt (delays: none)
@@requires
2??
429
delays+=
@@hint
`resp=$(fake_curl "$url"); code=${resp##*$'\n'}; body=${resp%$'\n'*}`. Note: `$( )` would run `fake_curl` in a subshell and lose its `calls` counter, so call it with output into a variable through a temp file or `printf -v`-style helper: `fake_curl "$url" > "$tmp"; resp=$(<"$tmp")`. Report with `2>/dev/null` or just keep the log on stderr.
@@solution
declare -A calls=()
fake_curl() {
    local n=$(( ${calls[$1]:-0} + 1 )); calls[$1]=$n
    case $1 in
        /health) printf '{"status":"ok"}\n200' ;;
        /flaky) if (( n < 3 )); then printf 'Service Unavailable\n503'; else printf '{"items":3}\n200'; fi ;;
        /limited) printf 'Too Many Requests\n429' ;;
        /missing) printf 'Not Found\n404' ;;
    esac
}

tmp=$(mktemp)
trap 'rm -f -- "$tmp"' EXIT

fetch() {
    local url=$1 attempt resp code body max=4
    delays=()
    for (( attempt = 1; attempt <= max; attempt++ )); do
        fake_curl "$url" > "$tmp"
        resp=$(<"$tmp")
        code=${resp##*$'\n'}
        body=${resp%$'\n'*}
        echo "attempt $attempt: $url -> $code" >&2
        case $code in
            2??) printf '%s' "$body"; return 0 ;;
            429|5??)
                (( attempt < max )) && delays+=($(( 2 ** attempt )))
                ;;
            *) break ;;
        esac
    done
    (( attempt > max )) && attempt=$max
    local word=attempts
    (( attempt == 1 )) && word=attempt
    printf 'failed with HTTP %s after %d %s' "$code" "$attempt" "$word"
    return 1
}

for url in /health /flaky /limited /missing; do
    fetch "$url" > "$tmp.out" 2>/dev/null
    result=$(<"$tmp.out")
    if [[ $result == failed* ]]; then
        echo "$url $result (delays: ${delays[*]:-none})"
    else
        echo "$url -> $result (delays: ${delays[*]:-none})"
    fi
done
rm -f -- "$tmp.out"

@@exercise Read and write flat JSON without jq
Write two helpers for **flat** JSON objects only:

- `json_get JSON KEY` prints the value of a string, number, boolean or null field using `[[ =~ ]]` (strip the quotes from strings); return 1 if the key is missing
- `json_escape STRING` escapes `\`, `"`, tab and newline
- `json_object KEY VALUE...` builds an object from pairs, escaping every value (all values as strings)

Then parse a health response and build a chat webhook payload from a message containing quotes and a newline.
@@starter
health='{"status": "ok", "version": "1.4.0", "uptime": 86400, "degraded": false}'
message='Deploy of "billing-api" finished
2 warnings'
echo TODO
@@expected
status=ok version=1.4.0 uptime=86400 degraded=false
missing key: region
{"text":"Deploy of \"billing-api\" finished\n2 warnings","channel":"#ops"}
@@requires
=~
BASH_REMATCH
@@hint
`re="\"$key\"[[:space:]]*:[[:space:]]*(\"([^\"]*)\"|[^,}[:space:]]+)"`: group 2 is a string's content, group 1 the raw value. Escape in order: backslashes first (`${s//\\/\\\\}`), then quotes, then `${s//$'\n'/\\n}` and tabs.
@@solution
health='{"status": "ok", "version": "1.4.0", "uptime": 86400, "degraded": false}'
message='Deploy of "billing-api" finished
2 warnings'

json_get() {
    local re="\"$2\"[[:space:]]*:[[:space:]]*(\"([^\"]*)\"|[^,}[:space:]]+)"
    [[ $1 =~ $re ]] || return 1
    if [[ ${BASH_REMATCH[1]} == \"* ]]; then
        printf '%s' "${BASH_REMATCH[2]}"
    else
        printf '%s' "${BASH_REMATCH[1]}"
    fi
}

json_escape() {
    local s=$1
    s=${s//\\/\\\\}
    s=${s//\"/\\\"}
    s=${s//$'\n'/\\n}
    s=${s//$'\t'/\\t}
    printf '%s' "$s"
}

json_object() {
    local out="{" sep=""
    while (( $# >= 2 )); do
        out+="$sep\"$(json_escape "$1")\":\"$(json_escape "$2")\""
        sep=","
        shift 2
    done
    printf '%s}' "$out"
}

line=""
for key in status version uptime degraded; do
    line+="$key=$(json_get "$health" "$key") "
done
echo "${line% }"
json_get "$health" region > /dev/null || echo "missing key: region"
json_object text "$message" channel "#ops"
echo
