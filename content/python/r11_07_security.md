@@lesson r11-security Secure Python: Hashing, Passwords, Tokens and Untrusted Input
@@concepts files, text-processing
@@text
IT tools handle passwords, tokens, file paths and data from users and networks. A few standard-library modules and habits prevent the most common security bugs. This lesson is about writing defensive Python, not attacking anything.

## Randomness for security: `secrets`, not `random`

`random` is predictable (a deterministic generator meant for simulations and games). For anything security-related use `secrets`:

```python
import secrets, string

print(secrets.token_urlsafe(32))          # session IDs, reset links, API keys
print(secrets.token_hex(16))
alphabet = string.ascii_letters + string.digits
print("".join(secrets.choice(alphabet) for _ in range(16)))   # a random password
```

## Hashing: fingerprints of data

A **cryptographic hash** (SHA-256) turns any data into a fixed-size fingerprint. The same input always gives the same hash; any change gives a completely different one; you can't go backwards.

```python
import hashlib

print(hashlib.sha256(b"backup-2026-11-03.zip contents").hexdigest())

def file_sha256(path, chunk=1 << 20):
    h = hashlib.sha256()
    with open(path, "rb") as f:
        for block in iter(lambda: f.read(chunk), b""):
            h.update(block)
    return h.hexdigest()
```

Uses: verifying downloads and backups, detecting changed files, deduplication. Don't use MD5 or SHA-1 where an attacker could craft collisions.

## Passwords: never store them, store slow salted hashes

A fast hash like SHA-256 is **wrong** for passwords: attackers can try billions of guesses per second. Use a deliberately slow **key-derivation function** with a random **salt** per user:

```python
import hashlib, secrets, hmac

def hash_password(password, iterations=200_000):
    salt = secrets.token_bytes(16)
    digest = hashlib.pbkdf2_hmac("sha256", password.encode(), salt, iterations)
    return f"pbkdf2_sha256${iterations}${salt.hex()}${digest.hex()}"

def verify_password(password, stored):
    _, iterations, salt_hex, digest_hex = stored.split("$")
    digest = hashlib.pbkdf2_hmac("sha256", password.encode(), bytes.fromhex(salt_hex), int(iterations))
    return hmac.compare_digest(digest.hex(), digest_hex)

stored = hash_password("correct horse battery staple")
print(stored[:40] + "...")
print(verify_password("correct horse battery staple", stored), verify_password("guess", stored))
```

- The **salt** makes identical passwords hash differently and defeats precomputed tables.
- The stored string includes the algorithm and parameters so you can raise them later.
- In applications prefer **argon2** (`argon2-cffi`) or **bcrypt**; `hashlib.scrypt` is also in the standard library.
- Compare secrets with `hmac.compare_digest` (constant time), not `==`, so response timing doesn't leak how many characters matched.

## HMAC: proving a message wasn't forged

An **HMAC** is a hash keyed with a shared secret. Webhooks (GitHub, Stripe, Slack) sign each request body with HMAC so the receiver knows it's genuine and unchanged:

```python
import hmac, hashlib

secret = b"webhook-secret"
body = b'{"event": "push"}'
signature = hmac.new(secret, body, hashlib.sha256).hexdigest()
print(hmac.compare_digest(signature, hmac.new(secret, body, hashlib.sha256).hexdigest()))
```

For encryption (not just signing) use the `cryptography` package (`Fernet` for simple symmetric encryption). Never invent your own crypto.

## Untrusted input: the big four

1. **Command injection.** Never build shell commands from input: `os.system(f"ping {host}")` with `host = "x; rm -rf /"`. Use `subprocess.run(["ping", "-c", "1", host])` with a **list** and no `shell=True`, and validate `host` first.
2. **SQL injection.** Always use parameters: `cur.execute("SELECT * FROM users WHERE name = ?", (name,))`.
3. **Path traversal.** A user-supplied file name like `../../etc/passwd` escapes your folder. Resolve and check that the result stays inside the base directory (exercise).
4. **Unsafe deserialisation.** `pickle.loads`, `yaml.load` (not `safe_load`), `eval` and `exec` on untrusted data run attacker code. Use JSON or `yaml.safe_load`; parse, don't evaluate.

Also: `tarfile.extractall` on untrusted archives can write outside the target (use `filter="data"`, the default since 3.14); `xml` parsers can be abused (use `defusedxml`); `requests`/`httpx` verify TLS certificates; never disable it with `verify=False`.

## Tools

- **bandit**: scans code for risky patterns (`shell=True`, `pickle`, hard-coded passwords).
- **pip-audit**: known vulnerabilities in installed packages.
- **ruff** has many of bandit's checks (`S` rules).
- Secret scanners (gitleaks) in CI.

@@quiz
Which module should generate a password-reset token?
- random
* secrets
- uuid1
- time
? `random` is predictable.

@@quiz
Why is plain SHA-256 a bad way to store passwords?
* It's very fast, so attackers can test billions of guesses per second; use a slow salted KDF
- It's reversible
- It produces duplicates
- It's not in the standard library
? Use argon2, bcrypt, scrypt or PBKDF2 with a salt.

@@quiz
What does a webhook HMAC signature prove?
- The data is encrypted
* The body came from someone who knows the shared secret and wasn't changed
- The sender's IP address
- The request is recent
? Add a timestamp check to stop replays.

@@quiz
What's the safe way to run `ping` on a user-supplied host?
- `os.system("ping " + host)`
- `subprocess.run("ping " + host, shell=True)`
* `subprocess.run(["ping", "-c", "1", host])` after validating host
- `eval("ping " + host)`
? A list of arguments isn't interpreted by a shell.

@@flashcards
`secrets` :: cryptographically secure random tokens and choices
Cryptographic hash :: fixed-size fingerprint; one-way and collision-resistant (SHA-256)
Salt :: random per-user value mixed into a password hash
KDF :: slow key-derivation function for passwords (argon2, bcrypt, scrypt, PBKDF2)
`hmac.compare_digest` :: constant-time comparison of secrets
HMAC :: keyed hash proving authenticity and integrity
Command injection :: shell commands built from untrusted input
Path traversal :: `../` in a file name escaping the allowed folder
bandit :: security linter for Python code

@@exercise Safe file serving
A help-desk tool serves files from a base folder by name. Write `safe_path(base, name)` that returns the resolved `pathlib.Path` inside `base`, or raises `PermissionError("outside base folder")` if the name would escape it (via `..`, an absolute path, or anything else). Also reject names containing a NUL character (`ValueError("bad name")`) and empty names.

Use `Path.resolve()` and `Path.is_relative_to()`. The tests use a temporary folder.
@@starter
from pathlib import Path


def safe_path(base, name):
    pass
@@tests
import tempfile, os
with tempfile.TemporaryDirectory() as tmp:
    base = Path(tmp)
    (base / "docs").mkdir()
    (base / "docs" / "vpn.md").write_text("x")
    check(safe_path(base, "docs/vpn.md"), (base / "docs" / "vpn.md").resolve(), "normal file")
    check(safe_path(base, "docs/../docs/vpn.md"), (base / "docs" / "vpn.md").resolve(), "dots that stay inside")
    for bad in ["../secret.txt", "docs/../../x", os.path.abspath(os.sep + "etc"), "C:\\Windows\\win.ini" if os.name == "nt" else "/etc/passwd"]:
        try:
            safe_path(base, bad)
            check(bad, "PermissionError", "escape blocked")
        except PermissionError as e:
            check(str(e), "outside base folder", "message")
    for bad in ["", "a\x00b"]:
        try:
            safe_path(base, bad)
            check(bad, "ValueError", "bad name")
        except ValueError:
            pass
@@hint
`if not name or "\x00" in name: raise ValueError("bad name")`. Then `root = Path(base).resolve()` and `target = (root / name).resolve()` (joining an absolute path replaces the base, which `is_relative_to` then catches). `if not target.is_relative_to(root): raise PermissionError(...)`.
@@solution
from pathlib import Path


def safe_path(base, name):
    if not name or "\x00" in name:
        raise ValueError("bad name")
    root = Path(base).resolve()
    target = (root / name).resolve()
    if not target.is_relative_to(root):
        raise PermissionError("outside base folder")
    return target

@@exercise Verify webhook signatures
A chat service sends webhooks with headers `X-Timestamp` (Unix seconds as text) and `X-Signature` (`"v1=" + hex HMAC-SHA256`). The signed message is `f"{timestamp}.".encode() + body`.

Write `sign(secret, timestamp, body)` returning the header value, and `verify(secret, headers, body, now, tolerance=300)` returning `(ok, reason)`:

- missing header → `(False, "missing header")`
- timestamp not an integer → `(False, "bad timestamp")`
- older or newer than `tolerance` seconds from `now` → `(False, "stale")` (stops replays)
- signature mismatch (compare with `hmac.compare_digest`) → `(False, "bad signature")`
- otherwise `(True, "ok")`
@@starter
import hmac
import hashlib


def sign(secret, timestamp, body):
    pass


def verify(secret, headers, body, now, tolerance=300):
    pass


secret = b"whsec_test"
body = b'{"event":"ticket.created","id":42}'
headers = {"X-Timestamp": "1762160000", "X-Signature": sign(secret, 1762160000, body)}
print(verify(secret, headers, body, now=1762160010))
print(verify(secret, headers, body + b" ", now=1762160010))
print(verify(secret, headers, body, now=1762170000))
@@expected
(True, 'ok')
(False, 'bad signature')
(False, 'stale')
@@tests
check(sign(b"k", 1, b"x").startswith("v1="), True, "prefix")
check(len(sign(b"k", 1, b"x")), 67, "hex sha256")
check(verify(b"k", {"X-Timestamp": "1"}, b"x", 1), (False, "missing header"), "missing")
check(verify(b"k", {"X-Timestamp": "soon", "X-Signature": "v1=00"}, b"x", 1), (False, "bad timestamp"), "bad ts")
check(verify(b"other", {"X-Timestamp": "5", "X-Signature": sign(b"k", 5, b"x")}, b"x", 5), (False, "bad signature"), "wrong secret")
check(verify(b"k", {"X-Timestamp": "5", "X-Signature": sign(b"k", 5, b"x")}, b"x", 5 - 300), (True, "ok"), "future within tolerance")
@@requires
compare_digest
@@hint
`mac = hmac.new(secret, f"{timestamp}.".encode() + body, hashlib.sha256).hexdigest(); return "v1=" + mac`. In verify, get both headers with `.get`, `int()` the timestamp inside try/except, check `abs(now - ts) > tolerance`, then compare `sign(secret, ts, body)` with the header.
@@solution
import hmac
import hashlib


def sign(secret, timestamp, body):
    mac = hmac.new(secret, f"{timestamp}.".encode() + body, hashlib.sha256).hexdigest()
    return "v1=" + mac


def verify(secret, headers, body, now, tolerance=300):
    ts_text = headers.get("X-Timestamp")
    given = headers.get("X-Signature")
    if ts_text is None or given is None:
        return False, "missing header"
    try:
        ts = int(ts_text)
    except ValueError:
        return False, "bad timestamp"
    if abs(now - ts) > tolerance:
        return False, "stale"
    if not hmac.compare_digest(sign(secret, ts, body), given):
        return False, "bad signature"
    return True, "ok"


secret = b"whsec_test"
body = b'{"event":"ticket.created","id":42}'
headers = {"X-Timestamp": "1762160000", "X-Signature": sign(secret, 1762160000, body)}
print(verify(secret, headers, body, now=1762160010))
print(verify(secret, headers, body + b" ", now=1762160010))
print(verify(secret, headers, body, now=1762170000))
