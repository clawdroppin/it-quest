@@lesson sp2-appvulns Application Vulnerabilities and Secure Coding Defences
@@text
Security+ expects you to recognise common application vulnerabilities from a description or a log line, and to name the defence. This lesson explains each one from the defender's side: what the flaw is, how it shows up in logs, and how secure coding and controls prevent it.

## Injection

**SQL injection (SQLi)**: user input is pasted into a SQL statement, so input can change the query's meaning. Signs in logs: quotes, `OR 1=1`, `UNION SELECT`, `--` comments in parameters.

Defence: **parameterised queries / prepared statements** (the database treats input as data, never as code), stored procedures used safely, **input validation** (allow lists), least-privilege database accounts, a **WAF** as an extra layer.

```python,norun
# Vulnerable: string building
cursor.execute("SELECT * FROM users WHERE name = '" + name + "'")
# Safe: parameterised
cursor.execute("SELECT * FROM users WHERE name = ?", (name,))
```

Other injection types: **command injection** (input reaches a shell), **LDAP injection**, **XML injection/XXE**. Same principle: never let input become code; use safe APIs.

## Cross-site scripting (XSS)

Untrusted input is returned in a web page without encoding, so a browser runs it as script in the site's context (stealing session cookies, changing the page).

- **Reflected**: in the request, echoed back immediately.
- **Stored/persistent**: saved (a comment, profile field) and shown to every visitor.
- **DOM-based**: client-side JavaScript writes input into the page.

Defence: **output encoding** (escape `<`, `>`, `&`, quotes for the context), templating engines that escape by default, **Content Security Policy** headers, `HttpOnly` cookies (script can't read them), input validation.

## Cross-site request forgery (CSRF)

A malicious site makes the victim's browser send a request to a site where they're logged in (the browser attaches their cookies). Defence: **anti-CSRF tokens**, `SameSite` cookies, re-authentication for sensitive actions.

## Server-side request forgery (SSRF)

The app fetches a URL supplied by the user, and an attacker points it at internal resources (for example a cloud metadata service). Defence: allow-list destinations, block internal ranges, use IMDSv2-style protections in the cloud.

## Memory and logic flaws

- **Buffer overflow**: writing past the end of a buffer can overwrite memory and hijack execution. Defences: memory-safe languages (Rust, Go, C#, Java), bounds checking, **ASLR** and **DEP/NX** in the OS, stack canaries.
- **Memory leak**: memory never freed → crashes, DoS.
- **Race condition / TOCTOU** (time of check to time of use): state changes between a check and its use (two withdrawals at once). Defence: locking, atomic operations, transactions.
- **Integer overflow**, **null pointer dereference**.
- **Insecure deserialisation**: untrusted data turned into objects runs unexpected code.

## Web app weaknesses

- **Broken access control** / **insecure direct object reference (IDOR)**: changing `?invoice=1001` to `1002` shows another customer's invoice. Check authorisation on **every** request, server-side.
- **Directory traversal**: `../../etc/passwd` in a file parameter. Canonicalise paths and check they stay inside the allowed folder.
- **Security misconfiguration**: default accounts, verbose error messages, directory listing, unnecessary features, missing security headers.
- **Sensitive data exposure**: secrets in code, logs or URLs; missing TLS.
- **Vulnerable components**: outdated libraries (track with an **SBOM**, software composition analysis).
- **Session issues**: predictable IDs, no expiry, session fixation. Use framework session management, regenerate IDs on login, `Secure` + `HttpOnly` + `SameSite` flags.

## Secure development practices

- **Input validation** (server-side; allow lists over deny lists), **output encoding**.
- **Secure cookies**, **code signing**, **sandboxing**.
- **Static analysis (SAST)** reads code; **dynamic analysis (DAST)** tests the running app; **fuzzing** sends random/malformed input; **code review**.
- **Secrets management** (vaults, not code).
- **Error handling** that logs details internally but shows users a generic message.
- **Least privilege** for app service accounts.
- OWASP **Top 10** and **ASVS** as references.

## Other vulnerability types in the objectives

**OS-based** (unpatched), **hardware** (firmware, end-of-life, legacy), **virtualisation** (**VM escape**, **resource reuse**), **cloud-specific** (misconfigured storage buckets), **supply chain** (service providers, hardware and software providers), **cryptographic** (weak algorithms, bad key management), **zero-day** (no patch yet), **mobile** (sideloading, jailbreaking), **misconfiguration**.

@@quiz
A login form's logs show the username `admin' OR '1'='1`. Which vulnerability is being probed, and what's the main fix?
* SQL injection; parameterised queries
- XSS; output encoding
- CSRF; anti-CSRF tokens
- Buffer overflow; ASLR
? Prepared statements keep input as data.

@@quiz
A comment field saves `<script>` content that runs for every visitor. What is this?
- Reflected XSS
* Stored XSS
- CSRF
- SSRF
? Stored/persistent XSS affects everyone who views it.

@@quiz
A customer changes `?order=5001` to `?order=5002` and sees someone else's order. What's the flaw?
- SQL injection
* Broken access control (IDOR)
- Race condition
- Directory traversal
? Check authorisation server-side on every object.

@@quiz
Which cookie flag stops JavaScript from reading a session cookie?
- Secure
* HttpOnly
- SameSite
- Domain
? Secure = HTTPS only; SameSite helps against CSRF.

@@quiz
Which testing method analyses source code without running it?
* SAST
- DAST
- Fuzzing
- Penetration testing
? DAST tests the running application.

@@flashcards
SQL injection :: input changes the query; use prepared statements
command injection :: input reaches a shell
XSS :: script runs in the victim's browser; encode output
reflected / stored / DOM XSS :: echoed / saved / client-side
CSRF :: forged request using the victim's session; tokens
SSRF :: server fetches attacker-chosen URLs
buffer overflow :: write past a buffer; ASLR, DEP, safe languages
TOCTOU :: race between check and use
IDOR :: guessable object references without auth checks
directory traversal :: ../ in file paths
HttpOnly :: script can't read the cookie
SameSite :: limits cross-site cookie sending
CSP :: Content Security Policy header
SAST / DAST :: static code / running app testing
fuzzing :: malformed input testing
SBOM :: software bill of materials
VM escape :: break out of a VM to the host

@@drill
Which kind of database query keeps user input from being executed as SQL code?
= parameterised
= parameterized
= prepared statement
= parameterised query
= parameterized query
= prepared statements
? Also called prepared statements.

@@drill
Which three-letter header lets a site restrict which scripts may run on its pages?
= CSP
= csp
? Content Security Policy.

@@exercise Flag suspicious web requests
Scan web log request paths and query strings for indicators of attack using the patterns in `SIGNATURES` (case-insensitive substring matches, after URL-decoding with `unquote_plus`). Print each flagged request with the attack types it matches, then a count per attack type.
@@lang python
@@starter
SIGNATURES = {
    "SQL injection": ["' or '1'='1", "union select", "--", "; drop table"],
    "XSS": ["<script", "javascript:", "onerror="],
    "Directory traversal": ["../", "..\\", "/etc/passwd"],
    "Command injection": ["; cat ", "| whoami", "&& id"],
}
requests = [
    "GET /search?q=blue+shoes",
    "GET /login?user=admin%27+OR+%271%27%3D%271",
    "GET /products?id=5+UNION+SELECT+username,password+FROM+users",
    "GET /comment?text=%3Cscript%3Ealert(1)%3C/script%3E",
    "GET /download?file=../../../../etc/passwd",
    "GET /ping?host=8.8.8.8%3B+cat+/etc/shadow",
    "GET /profile?name=Sato",
]
@@expected
FLAG GET /login?user=admin%27+OR+%271%27%3D%271 -> SQL injection
FLAG GET /products?id=5+UNION+SELECT+username,password+FROM+users -> SQL injection
FLAG GET /comment?text=%3Cscript%3Ealert(1)%3C/script%3E -> XSS
FLAG GET /download?file=../../../../etc/passwd -> Directory traversal
FLAG GET /ping?host=8.8.8.8%3B+cat+/etc/shadow -> Command injection
SQL injection: 2
XSS: 1
Directory traversal: 1
Command injection: 1
@@requires
unquote_plus(
Counter(
@@hint
`text = unquote_plus(request).lower()`; collect `[name for name, patterns in SIGNATURES.items() if any(p in text for p in patterns)]`.
@@solution
from collections import Counter
from urllib.parse import unquote_plus

SIGNATURES = {
    "SQL injection": ["' or '1'='1", "union select", "--", "; drop table"],
    "XSS": ["<script", "javascript:", "onerror="],
    "Directory traversal": ["../", "..\\", "/etc/passwd"],
    "Command injection": ["; cat ", "| whoami", "&& id"],
}
requests = [
    "GET /search?q=blue+shoes",
    "GET /login?user=admin%27+OR+%271%27%3D%271",
    "GET /products?id=5+UNION+SELECT+username,password+FROM+users",
    "GET /comment?text=%3Cscript%3Ealert(1)%3C/script%3E",
    "GET /download?file=../../../../etc/passwd",
    "GET /ping?host=8.8.8.8%3B+cat+/etc/shadow",
    "GET /profile?name=Sato",
]

counts = Counter()
for request in requests:
    text = unquote_plus(request).lower()
    hits = [name for name, patterns in SIGNATURES.items() if any(p in text for p in patterns)]
    if hits:
        counts.update(hits)
        print(f"FLAG {request} -> {', '.join(hits)}")
for name, n in counts.most_common():
    print(f"{name}: {n}")

@@exercise Fix an injectable query
The function `find_orders_unsafe` builds SQL by concatenating the customer name, so a crafted name returns every order. Write `find_orders_safe` using a parameterised query, then run both against the in-memory SQLite database with a normal name and a malicious one, printing how many rows each returns.
@@lang python
@@starter
import sqlite3

db = sqlite3.connect(":memory:")
db.execute("CREATE TABLE orders (id INTEGER, customer TEXT, total INTEGER)")
db.executemany("INSERT INTO orders VALUES (?, ?, ?)", [(1, "sato", 1200), (2, "lee", 800), (3, "sato", 300), (4, "kim", 5000)])

def find_orders_unsafe(name):
    return db.execute("SELECT id FROM orders WHERE customer = '" + name + "'").fetchall()

def find_orders_safe(name):
    raise NotImplementedError

for name in ["sato", "x' OR '1'='1"]:
    print(f"{name!r}: unsafe {len(find_orders_unsafe(name))} rows")
@@expected
'sato': unsafe 2 rows, safe 2 rows
"x' OR '1'='1": unsafe 4 rows, safe 0 rows
@@requires
WHERE customer = ?
(name,)
@@hint
`db.execute("SELECT id FROM orders WHERE customer = ?", (name,)).fetchall()`.
@@solution
import sqlite3

db = sqlite3.connect(":memory:")
db.execute("CREATE TABLE orders (id INTEGER, customer TEXT, total INTEGER)")
db.executemany("INSERT INTO orders VALUES (?, ?, ?)", [(1, "sato", 1200), (2, "lee", 800), (3, "sato", 300), (4, "kim", 5000)])

def find_orders_unsafe(name):
    return db.execute("SELECT id FROM orders WHERE customer = '" + name + "'").fetchall()

def find_orders_safe(name):
    return db.execute("SELECT id FROM orders WHERE customer = ?", (name,)).fetchall()

for name in ["sato", "x' OR '1'='1"]:
    print(f"{name!r}: unsafe {len(find_orders_unsafe(name))} rows, safe {len(find_orders_safe(name))} rows")
