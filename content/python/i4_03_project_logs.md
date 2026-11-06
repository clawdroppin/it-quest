@@lesson i4-project-logs 🏆 Module Project: Web Log Analyzer
@@concepts text-processing, files, objects
@@text
Every web server writes an **access log**: one line per request. Here's the common format used by Apache and nginx:

```text
203.0.113.7 - - [31/Oct/2026:08:15:02 +0900] "GET /login HTTP/1.1" 401 512
```

That's: client IP, (two unused fields), time, the request (method, path, protocol), the **status code**, and the response size in bytes.

Status codes matter: **2xx** success, **3xx** redirect, **4xx** client error (401 unauthorized, 403 forbidden, 404 not found), **5xx** server error. A burst of 401s from one IP is someone guessing passwords; a burst of 5xx is your app breaking.

You'll build the analyzer in steps: parse a line into an object, then compute the statistics an on-call engineer would want.

@@quiz
In a log analyser, why read the file line by line instead of `f.read()`?
* Log files can be huge; line by line uses little memory
- read() can't open text files
- Lines are sorted automatically
- It's required by the re module
? `for line in f:` reads one line at a time.

@@quiz
Which tool counts how often each error message appears?
* `collections.Counter`
- `set`
- `sorted`
- `enumerate`
? Counter(messages).most_common(5) gives the top five.

@@quiz
Some lines in the log don't match your pattern. What should a robust analyser do?
* Skip them (and maybe count them) instead of crashing
- Stop with an exception
- Treat them as errors
- Delete them from the file
? Real logs always contain surprises: partial writes, stack traces, other formats.

@@flashcards
for line in f :: read a file one line at a time
re.match / re.search :: match at the start / anywhere
Counter :: count occurrences
defaultdict(list) :: group items without checking keys
datetime.strptime :: parse a timestamp

@@exercise Parse a line
Write a class `Request` with attributes `ip`, `method`, `path`, `status` (int) and `size` (int), and a function `parse(line)` that returns a `Request`, or `None` if the line doesn't match the format.
@@starter
import re

LINE = re.compile(r'(\S+) \S+ \S+ \[[^\]]+\] "(\S+) (\S+) [^"]*" (\d{3}) (\d+)')


class Request:
    pass


def parse(line):
    pass


r = parse('203.0.113.7 - - [31/Oct/2026:08:15:02 +0900] "GET /login HTTP/1.1" 401 512')
print(r.ip, r.method, r.path, r.status, r.size)
@@tests
r = parse('203.0.113.7 - - [31/Oct/2026:08:15:02 +0900] "GET /login HTTP/1.1" 401 512')
check((r.ip, r.method, r.path, r.status, r.size), ("203.0.113.7", "GET", "/login", 401, 512), "parse(sample line)")
check(parse("garbage"), None, "parse('garbage')")
@@hint
The regex is given: `m = LINE.match(line)`; if `m` is `None` return `None`. Otherwise `m.group(1)` is the IP, 2 the method, 3 the path, 4 the status, 5 the size. Give `Request.__init__` five parameters.
@@solution
import re

LINE = re.compile(r'(\S+) \S+ \S+ \[[^\]]+\] "(\S+) (\S+) [^"]*" (\d{3}) (\d+)')


class Request:
    def __init__(self, ip, method, path, status, size):
        self.ip = ip
        self.method = method
        self.path = path
        self.status = status
        self.size = size


def parse(line):
    m = LINE.match(line)
    if m is None:
        return None
    return Request(m.group(1), m.group(2), m.group(3), int(m.group(4)), int(m.group(5)))


r = parse('203.0.113.7 - - [31/Oct/2026:08:15:02 +0900] "GET /login HTTP/1.1" 401 512')
print(r.ip, r.method, r.path, r.status, r.size)

@@exercise The full report
Using your `Request` and `parse` (paste them in), write `report(lines)` that returns a dict with:

- `"requests"`: number of lines that parsed,
- `"errors"`: how many had a status of 500 or more,
- `"bytes"`: total size of all responses,
- `"top_path"`: the most requested path,
- `"suspects"`: sorted list of IPs with **3 or more** `401` responses.

Lines that don't parse are skipped.
@@starter
import re
from collections import Counter

LINE = re.compile(r'(\S+) \S+ \S+ \[[^\]]+\] "(\S+) (\S+) [^"]*" (\d{3}) (\d+)')

# paste Request and parse here


def report(lines):
    pass


LOG = """10.0.0.4 - - [31/Oct/2026:08:00:01 +0900] "GET / HTTP/1.1" 200 5120
203.0.113.7 - - [31/Oct/2026:08:00:02 +0900] "POST /login HTTP/1.1" 401 128
203.0.113.7 - - [31/Oct/2026:08:00:03 +0900] "POST /login HTTP/1.1" 401 128
this line is corrupted
203.0.113.7 - - [31/Oct/2026:08:00:04 +0900] "POST /login HTTP/1.1" 401 128
10.0.0.9 - - [31/Oct/2026:08:00:05 +0900] "GET /api/orders HTTP/1.1" 500 64
10.0.0.4 - - [31/Oct/2026:08:00:06 +0900] "GET / HTTP/1.1" 200 5120
10.0.0.4 - - [31/Oct/2026:08:00:07 +0900] "GET / HTTP/1.1" 304 0"""
print(report(LOG.splitlines()))
@@tests
LOG = """10.0.0.4 - - [31/Oct/2026:08:00:01 +0900] "GET / HTTP/1.1" 200 5120
203.0.113.7 - - [31/Oct/2026:08:00:02 +0900] "POST /login HTTP/1.1" 401 128
203.0.113.7 - - [31/Oct/2026:08:00:03 +0900] "POST /login HTTP/1.1" 401 128
this line is corrupted
203.0.113.7 - - [31/Oct/2026:08:00:04 +0900] "POST /login HTTP/1.1" 401 128
10.0.0.9 - - [31/Oct/2026:08:00:05 +0900] "GET /api/orders HTTP/1.1" 500 64
10.0.0.4 - - [31/Oct/2026:08:00:06 +0900] "GET / HTTP/1.1" 200 5120
10.0.0.4 - - [31/Oct/2026:08:00:07 +0900] "GET / HTTP/1.1" 304 0"""
r = report(LOG.splitlines())
check(r["requests"], 7, "requests")
check(r["errors"], 1, "errors (status >= 500)")
check(r["bytes"], 10688, "bytes")
check(r["top_path"], "/", "top_path")
check(r["suspects"], ["203.0.113.7"], "suspects")
@@hint
Parse every line into a list, dropping `None`s. Then: `len(...)`, a count with a condition, `sum(r.size for r in reqs)`, `Counter(r.path for r in reqs).most_common(1)[0][0]`, and a `Counter` of the IPs of the 401 requests.
@@solution
import re
from collections import Counter

LINE = re.compile(r'(\S+) \S+ \S+ \[[^\]]+\] "(\S+) (\S+) [^"]*" (\d{3}) (\d+)')


class Request:
    def __init__(self, ip, method, path, status, size):
        self.ip = ip
        self.method = method
        self.path = path
        self.status = status
        self.size = size


def parse(line):
    m = LINE.match(line)
    if m is None:
        return None
    return Request(m.group(1), m.group(2), m.group(3), int(m.group(4)), int(m.group(5)))


def report(lines):
    reqs = [r for r in (parse(line) for line in lines) if r is not None]
    unauthorized = Counter(r.ip for r in reqs if r.status == 401)
    return {
        "requests": len(reqs),
        "errors": sum(1 for r in reqs if r.status >= 500),
        "bytes": sum(r.size for r in reqs),
        "top_path": Counter(r.path for r in reqs).most_common(1)[0][0] if reqs else None,
        "suspects": sorted(ip for ip, n in unauthorized.items() if n >= 3),
    }


LOG = """10.0.0.4 - - [31/Oct/2026:08:00:01 +0900] "GET / HTTP/1.1" 200 5120
203.0.113.7 - - [31/Oct/2026:08:00:02 +0900] "POST /login HTTP/1.1" 401 128
203.0.113.7 - - [31/Oct/2026:08:00:03 +0900] "POST /login HTTP/1.1" 401 128
this line is corrupted
203.0.113.7 - - [31/Oct/2026:08:00:04 +0900] "POST /login HTTP/1.1" 401 128
10.0.0.9 - - [31/Oct/2026:08:00:05 +0900] "GET /api/orders HTTP/1.1" 500 64
10.0.0.4 - - [31/Oct/2026:08:00:06 +0900] "GET / HTTP/1.1" 200 5120
10.0.0.4 - - [31/Oct/2026:08:00:07 +0900] "GET / HTTP/1.1" 304 0"""
print(report(LOG.splitlines()))
@@text
🎉 **Intermediate done.** You can now read and write files, survive bad input, search text with regex, handle JSON and CSV, and model things with classes. That's enough Python to automate a large part of a help desk or junior sysadmin job.

In **Advanced** you'll build real command-line tools: run system commands, work with networks and IP addresses, write tests, and log properly.
