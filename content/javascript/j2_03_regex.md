@@lesson j2-regex Text Processing and Regular Expressions
@@concepts text-processing
@@text
JavaScript regexes are written between slashes: `/pattern/flags`.

```javascript
const line = "2026-10-31 08:15:02 ERROR [web01] disk C: 3% free";
console.log(/ERROR/.test(line));                 // does it match?
const m = line.match(/^(\S+) (\S+) (\w+) \[(\w+)\]/);
console.log(m[3], m[4]);                         // groups
```

## Named groups

```javascript
const re = /user=(?<user>\w+) ip=(?<ip>[\d.]+)/;
const { user, ip } = "user=ana ip=10.0.0.4 action=login".match(re).groups;
console.log(user, ip);
```

## All matches: the g flag and matchAll

```javascript
const log = "from 10.0.0.4, from 203.0.113.7, from 10.0.0.4";
console.log(log.match(/\d+\.\d+\.\d+\.\d+/g));                     // every match
for (const m of log.matchAll(/from (?<ip>[\d.]+)/g)) console.log(m.groups.ip);
```

Flags: `g` all matches, `i` ignore case, `m` `^`/`$` per line, `s` `.` matches newlines too.

## Replace and split

```javascript
console.log("Server:  WEB01   Status: OK".replace(/\s+/g, " "));
console.log("2026-10-31".replace(/(\d+)-(\d+)-(\d+)/, "$3/$2/$1"));
console.log("password=hunter2&user=ana".replace(/password=[^&]*/, "password=***"));
console.log("a, b;c  d".split(/[,;\s]+/));
```

Masking secrets before writing logs, like the third line, is a real security practice.
## Testing and building patterns

- `regex.test(text)` returns true or false: the quickest check ("does this look like an IP?").
- Flags change how a pattern matches: `i` ignores case, `g` finds all matches, `m` makes `^` and `$` match at each line, `u` handles Unicode properly.
- Build a pattern from user input with care: characters like `.`, `*` and `(` have special meanings. Escape them first: `text.replace(/[.*+?^${}()|[\]\\]/g, "\\$&")`, or use plain string methods (`includes`, `startsWith`) when you don't need a pattern at all.
- Anchor patterns that must match the whole value: `/^\d{3}-\d{4}$/` checks a whole postcode, while `/\d{3}-\d{4}/` would also match inside "abc123-4567xyz".

```javascript
const isPostcode = (s) => /^\d{3}-\d{4}$/.test(s);
console.log(isPostcode("150-0002"), isPostcode("1500002"), isPostcode("x150-0002"));
console.log(/error/i.test("ERROR: disk full"));
```

Regular expressions are powerful but easy to make unreadable. Name the groups, keep each pattern small, and write a few test strings next to it, including ones that should **not** match.

@@quiz
Which call returns **all** IP-looking matches in a string?
* `text.match(/\d+\.\d+\.\d+\.\d+/g)`
- `text.match(/\d+\.\d+\.\d+\.\d+/)`
- `/\d+/.test(text)`
- `text.split(/\d/)`
? Without `g`, `match` returns only the first match (with its groups).

@@quiz
Which flag makes a regular expression ignore upper and lower case?
* `i`
- `g`
- `m`
- `c`
? /error/i matches "Error", "ERROR" and "error".

@@quiz
Why anchor a validation pattern with `^` and `$`?
* So the whole value must match, not just part of it
- To make it faster
- To ignore case
- To find all matches
? Without anchors, "abc150-0002xyz" would pass a postcode check.

@@drill
Which regex flag finds every match, not just the first?
= g
? Combine with matchAll to loop over matches.

@@flashcards
regex.test(s) :: true if the pattern matches
s.match(re) :: first match with groups (all matches with g)
s.matchAll(re) :: iterator of every match with groups (needs g)
(?<name>...) :: named group, read from match.groups.name
flag i :: ignore case
flag g :: global: all matches
^ and $ :: start and end of the string
\d \w \s :: digit, word character, whitespace

@@exercise Mask secrets
Write `maskSecrets(text)` that replaces the value of every `password=`, `token=` and `apikey=` (any case) with `***`. A value runs until `&`, a space, or the end.
@@starter
function maskSecrets(text) {
  throw new Error("TODO: not done yet");
}

@@tests
check(maskSecrets("user=ana&password=hunter2&x=1"), "user=ana&password=***&x=1", "password in a query string");
check(maskSecrets("TOKEN=abc123 apikey=XYZ"), "TOKEN=*** apikey=***", "two secrets, any case");
check(maskSecrets("nothing secret here"), "nothing secret here", "no secrets");
@@hint
`text.replace(/(password|token|apikey)=[^&\s]*/gi, "$1=***")`. The `$1` keeps the name exactly as it was written.
@@solution
function maskSecrets(text) {
  return text.replace(/(password|token|apikey)=[^&\s]*/gi, "$1=***");
}

@@exercise Parse a syslog line
Write `parseSyslog(line)` for lines like `Oct 31 08:15:02 web01 sshd[1234]: Failed password for root`. Return `{ time, host, program, pid, message }` with `pid` as a **number**, or `null` if the line doesn't match.
@@starter
function parseSyslog(line) {
  throw new Error("TODO: not done yet");
}

@@tests
check(
  parseSyslog("Oct 31 08:15:02 web01 sshd[1234]: Failed password for root"),
  { time: "Oct 31 08:15:02", host: "web01", program: "sshd", pid: 1234, message: "Failed password for root" },
  "an sshd line",
);
check(parseSyslog("Nov  2 01:00:00 db01 cron[7]: job done").time, "Nov  2 01:00:00", "a one-digit day keeps its spacing");
check(parseSyslog("garbage"), null, "not a syslog line");
@@hint
`/^(?<time>\w{3}\s+\d{1,2} \d\d:\d\d:\d\d) (?<host>\S+) (?<program>[^\[]+)\[(?<pid>\d+)\]: (?<message>.*)$/`. Then build the object from `m.groups`, with `Number(m.groups.pid)`.
@@solution
function parseSyslog(line) {
  const m = line.match(/^(?<time>\w{3}\s+\d{1,2} \d\d:\d\d:\d\d) (?<host>\S+) (?<program>[^\[]+)\[(?<pid>\d+)\]: (?<message>.*)$/);
  if (!m) return null;
  const { time, host, program, pid, message } = m.groups;
  return { time, host, program, pid: Number(pid), message };
}
