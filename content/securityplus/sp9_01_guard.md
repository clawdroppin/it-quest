@@lesson sp9-guard Security Guard: Integrity Monitoring and Password Audits
@@text
Two classic defensive tools, both on the Security+ exam and both used in real SOCs:

1. **File integrity monitoring (FIM)**: record a **baseline** of hashes for important files (a web server's folder, `C:\Windows\System32\drivers`, `/etc`), then check regularly. A changed `web.config`, a new `.php` file in the upload folder, or a replaced DLL is often the *first* sign of a breach. PCI DSS requires FIM on card systems.
2. **Password auditing** the modern way (NIST SP 800-63B): length matters most, and the password must not be a known breached one. Complexity rules ("one capital, one symbol") produce `Password1!`, which is in every attacker's wordlist.

```text
python main.py baseline C:\inetpub\wwwroot baseline.json
python main.py check C:\inetpub\wwwroot baseline.json
  ADDED     uploads/shell.aspx
  MODIFIED  web.config
ALERT: 2 change(s) since the baseline of 2026-10-31T09:00:14

python main.py password --user ytanaka --online
Password to check (not shown):
  x appears in 52,256,179 breached accounts (Have I Been Pwned)
```

## k-anonymity: checking breaches without sending the password

The **Have I Been Pwned** Pwned Passwords API knows billions of breached passwords. You never send it the password, or even its full hash: you send only the **first 5 characters** of the SHA-1 hash. It replies with every breached hash suffix starting with those 5 characters (hundreds of them) and you look for yours locally. The service never learns which password you checked. (SHA-1 is fine *here*: it's a lookup key, not protection.)

Schedule `check` with Task Scheduler or cron and send its output to your SIEM, and you have a real detective control.
@@project guard Security Guard
Write the functions in `guard.py` until `python -m unittest` passes. Then make a baseline of a test folder, change something, and check it. `--online` contacts the Have I Been Pwned API, sending only 5 characters of the hash.
@@file guard.py
"""File integrity monitoring and password auditing."""

import hashlib
from pathlib import Path


def hash_file(path):
    """SHA-256 hex digest of a file, read in 64 KB chunks."""
    raise NotImplementedError("TODO: hash_file")


def build_baseline(root):
    """{relative path with forward slashes: sha256} for every file under root."""
    raise NotImplementedError("TODO: build_baseline")


def compare(baseline, current):
    """{"added": [...], "removed": [...], "modified": [...]}, each sorted."""
    raise NotImplementedError("TODO: compare")


def password_problems(password, username="", banned=()):
    """Problems with a password, following NIST SP 800-63B ideas (length and breach checks, not
    composition rules). Messages, in this order:
      "shorter than 12 characters"
      "is a common or breached password"      (case-insensitive match in `banned`)
      "contains the username"                 (case-insensitive; only for usernames of 3+ characters)
      "is one character repeated"
    """
    raise NotImplementedError("TODO: password_problems")


def pwned_prefix(password):
    """(first 5, remaining 35) characters of the password's uppercase SHA-1 hex digest.
    Only the 5-character prefix is ever sent to the breach API: k-anonymity."""
    raise NotImplementedError("TODO: pwned_prefix")


def pwned_count(suffix, range_response):
    """How many times the password appeared in breaches, from the API's response for its prefix:
    lines of "SUFFIX:COUNT". 0 if the suffix isn't listed."""
    raise NotImplementedError("TODO: pwned_count")
@@file test_guard.py
import hashlib
import tempfile
import unittest
from pathlib import Path

from guard import build_baseline, compare, hash_file, password_problems, pwned_count, pwned_prefix


class IntegrityTests(unittest.TestCase):
    def setUp(self):
        self.tmp = tempfile.TemporaryDirectory()
        self.root = Path(self.tmp.name)
        (self.root / "conf").mkdir()
        (self.root / "app.exe").write_bytes(b"MZ program")
        (self.root / "conf" / "settings.ini").write_text("debug=false\n")

    def tearDown(self):
        self.tmp.cleanup()

    def test_hash_file(self):
        self.assertEqual(hash_file(self.root / "app.exe"), hashlib.sha256(b"MZ program").hexdigest())

    def test_baseline_uses_relative_forward_slash_paths(self):
        base = build_baseline(self.root)
        self.assertEqual(sorted(base), ["app.exe", "conf/settings.ini"])
        self.assertEqual(base["app.exe"], hashlib.sha256(b"MZ program").hexdigest())

    def test_no_changes(self):
        base = build_baseline(self.root)
        self.assertEqual(compare(base, build_baseline(self.root)), {"added": [], "removed": [], "modified": []})

    def test_changes(self):
        base = build_baseline(self.root)
        (self.root / "conf" / "settings.ini").write_text("debug=true\n")
        (self.root / "app.exe").unlink()
        (self.root / "conf" / "backdoor.dll").write_bytes(b"evil")
        (self.root / "a_new.txt").write_text("hi")
        self.assertEqual(compare(base, build_baseline(self.root)), {
            "added": ["a_new.txt", "conf/backdoor.dll"],
            "removed": ["app.exe"],
            "modified": ["conf/settings.ini"],
        })


class PasswordTests(unittest.TestCase):
    BANNED = ["password123", "Qwerty12345!", "letmein"]

    def test_good_password(self):
        self.assertEqual(password_problems("correct horse battery staple", "ytanaka", self.BANNED), [])

    def test_short(self):
        self.assertEqual(password_problems("Tr0ub4dor&3", "ytanaka"), ["shorter than 12 characters"])

    def test_banned_any_case(self):
        self.assertEqual(password_problems("QWERTY12345!", banned=self.BANNED),
                         ["is a common or breached password"])

    def test_username(self):
        self.assertEqual(password_problems("YTanaka-2026-spring", "ytanaka"), ["contains the username"])
        self.assertEqual(password_problems("bobsled-weather-42", "bo"), [])

    def test_repeated_and_several_problems(self):
        self.assertEqual(password_problems("aaaaaaaa"), ["shorter than 12 characters", "is one character repeated"])
        self.assertEqual(password_problems("letmein", "letme", self.BANNED),
                         ["shorter than 12 characters", "is a common or breached password", "contains the username"])


class PwnedTests(unittest.TestCase):
    def test_prefix(self):
        # SHA-1 of "password" is 5BAA61E4C9B93F3F0682250B6CF8331B7EE68FD8
        self.assertEqual(pwned_prefix("password"), ("5BAA6", "1E4C9B93F3F0682250B6CF8331B7EE68FD8"))

    def test_count(self):
        response = "003D68EB55068C33ACE09247EE4C639306B:3\r\n1E4C9B93F3F0682250B6CF8331B7EE68FD8:52256179\r\n"
        self.assertEqual(pwned_count("1E4C9B93F3F0682250B6CF8331B7EE68FD8", response), 52256179)
        self.assertEqual(pwned_count("1e4c9b93f3f0682250b6cf8331b7ee68fd8", response), 52256179)
        self.assertEqual(pwned_count("FFFFF", response), 0)


if __name__ == "__main__":
    unittest.main()
@@file main.py
"""Security toolkit.

python main.py baseline C:\\inetpub\\wwwroot baseline.json     save hashes of every file
python main.py check C:\\inetpub\\wwwroot baseline.json        exit code 1 if anything changed
python main.py password [--user ytanaka] [--online]          audit a password (typed hidden)
"""

import argparse
import getpass
import json
import sys
import urllib.request
from datetime import datetime
from pathlib import Path

from guard import build_baseline, compare, password_problems, pwned_count, pwned_prefix

HERE = Path(__file__).parent


def cmd_baseline(args):
    base = build_baseline(args.folder)
    doc = {"folder": str(Path(args.folder).resolve()), "created": datetime.now().isoformat(timespec="seconds"), "files": base}
    Path(args.baseline).write_text(json.dumps(doc, indent=2), encoding="utf-8")
    print(f"Saved {len(base)} file hashes to {args.baseline}")
    return 0


def cmd_check(args):
    doc = json.loads(Path(args.baseline).read_text(encoding="utf-8"))
    changes = compare(doc["files"], build_baseline(args.folder))
    total = sum(len(v) for v in changes.values())
    for kind, paths in changes.items():
        for p in paths:
            print(f"  {kind.upper():9} {p}")
    if total:
        print(f"ALERT: {total} change(s) since the baseline of {doc['created']}")
        return 1
    print(f"OK: no changes since {doc['created']}")
    return 0


def cmd_password(args):
    banned = (HERE / "banned.txt").read_text(encoding="utf-8").split()
    password = getpass.getpass("Password to check (not shown): ")
    problems = password_problems(password, args.user, banned)
    if args.online:
        prefix, suffix = pwned_prefix(password)
        req = urllib.request.Request(f"https://api.pwnedpasswords.com/range/{prefix}", headers={"User-Agent": "it-quest-guard"})
        with urllib.request.urlopen(req, timeout=10) as r:
            count = pwned_count(suffix, r.read().decode())
        if count:
            problems.append(f"appears in {count:,} breached accounts (Have I Been Pwned)")
    for p in problems:
        print("  x", p)
    print("Looks good." if not problems else f"{len(problems)} problem(s).")
    return 1 if problems else 0


def main(argv=None):
    parser = argparse.ArgumentParser(description="File integrity monitoring and password auditing")
    sub = parser.add_subparsers(required=True)
    p = sub.add_parser("baseline", help="record file hashes")
    p.add_argument("folder")
    p.add_argument("baseline")
    p.set_defaults(func=cmd_baseline)
    p = sub.add_parser("check", help="compare a folder with its baseline")
    p.add_argument("folder")
    p.add_argument("baseline")
    p.set_defaults(func=cmd_check)
    p = sub.add_parser("password", help="audit a password")
    p.add_argument("--user", default="")
    p.add_argument("--online", action="store_true", help="also check Have I Been Pwned (sends only 5 hash characters)")
    p.set_defaults(func=cmd_password)
    args = parser.parse_args(argv)
    return args.func(args)


if __name__ == "__main__":
    sys.exit(main())
@@file banned.txt
123456
123456789
12345678
password
password1
password123
qwerty
qwerty123
Qwerty12345!
111111
abc123
iloveyou
admin
admin123
welcome
welcome1
letmein
monkey
dragon
football
baseball
sunshine
princess
Passw0rd!
P@ssw0rd
P@ssword1
Summer2026!
Winter2026!
changeme
trustno1
@@text
> Tips: `hashlib.sha256()` has `.update(chunk)` for reading in pieces: `iter(lambda: f.read(65536), b"")` loops until the file ends. `Path.rglob("*")` walks a folder; `p.relative_to(root).as_posix()` gives `conf/settings.ini` on every OS. Sets make `compare` short: `set(current) - set(baseline)` are the added files.
@@project guard_finished Security Guard: the finished version
The complete toolkit. Ideas: store the baseline somewhere the attacker can't write (or sign it with an HMAC key), send alerts by email or to a Teams/Slack webhook, or check every password in a CSV export of a test system.
@@file guard.py
"""File integrity monitoring and password auditing."""

import hashlib
from pathlib import Path


def hash_file(path):
    """SHA-256 hex digest of a file, read in 64 KB chunks."""
    h = hashlib.sha256()
    with open(path, "rb") as f:
        for chunk in iter(lambda: f.read(65536), b""):
            h.update(chunk)
    return h.hexdigest()


def build_baseline(root):
    """{relative path with forward slashes: sha256} for every file under root."""
    root = Path(root)
    return {p.relative_to(root).as_posix(): hash_file(p) for p in sorted(root.rglob("*")) if p.is_file()}


def compare(baseline, current):
    """{"added": [...], "removed": [...], "modified": [...]}, each sorted."""
    return {
        "added": sorted(set(current) - set(baseline)),
        "removed": sorted(set(baseline) - set(current)),
        "modified": sorted(p for p in baseline.keys() & current.keys() if baseline[p] != current[p]),
    }


def password_problems(password, username="", banned=()):
    """Problems with a password, following NIST SP 800-63B ideas (length and breach checks, not
    composition rules). Messages, in this order:
      "shorter than 12 characters"
      "is a common or breached password"      (case-insensitive match in `banned`)
      "contains the username"                 (case-insensitive; only for usernames of 3+ characters)
      "is one character repeated"
    """
    problems = []
    if len(password) < 12:
        problems.append("shorter than 12 characters")
    if password.lower() in {b.lower() for b in banned}:
        problems.append("is a common or breached password")
    if len(username) >= 3 and username.lower() in password.lower():
        problems.append("contains the username")
    if password and len(set(password)) == 1:
        problems.append("is one character repeated")
    return problems


def pwned_prefix(password):
    """(first 5, remaining 35) characters of the password's uppercase SHA-1 hex digest.
    Only the 5-character prefix is ever sent to the breach API: k-anonymity."""
    digest = hashlib.sha1(password.encode("utf-8")).hexdigest().upper()
    return digest[:5], digest[5:]


def pwned_count(suffix, range_response):
    """How many times the password appeared in breaches, from the API's response for its prefix:
    lines of "SUFFIX:COUNT". 0 if the suffix isn't listed."""
    for line in range_response.splitlines():
        s, _, count = line.strip().partition(":")
        if s.upper() == suffix.upper():
            return int(count)
    return 0
@@file test_guard.py
import hashlib
import tempfile
import unittest
from pathlib import Path

from guard import build_baseline, compare, hash_file, password_problems, pwned_count, pwned_prefix


class IntegrityTests(unittest.TestCase):
    def setUp(self):
        self.tmp = tempfile.TemporaryDirectory()
        self.root = Path(self.tmp.name)
        (self.root / "conf").mkdir()
        (self.root / "app.exe").write_bytes(b"MZ program")
        (self.root / "conf" / "settings.ini").write_text("debug=false\n")

    def tearDown(self):
        self.tmp.cleanup()

    def test_hash_file(self):
        self.assertEqual(hash_file(self.root / "app.exe"), hashlib.sha256(b"MZ program").hexdigest())

    def test_baseline_uses_relative_forward_slash_paths(self):
        base = build_baseline(self.root)
        self.assertEqual(sorted(base), ["app.exe", "conf/settings.ini"])
        self.assertEqual(base["app.exe"], hashlib.sha256(b"MZ program").hexdigest())

    def test_no_changes(self):
        base = build_baseline(self.root)
        self.assertEqual(compare(base, build_baseline(self.root)), {"added": [], "removed": [], "modified": []})

    def test_changes(self):
        base = build_baseline(self.root)
        (self.root / "conf" / "settings.ini").write_text("debug=true\n")
        (self.root / "app.exe").unlink()
        (self.root / "conf" / "backdoor.dll").write_bytes(b"evil")
        (self.root / "a_new.txt").write_text("hi")
        self.assertEqual(compare(base, build_baseline(self.root)), {
            "added": ["a_new.txt", "conf/backdoor.dll"],
            "removed": ["app.exe"],
            "modified": ["conf/settings.ini"],
        })


class PasswordTests(unittest.TestCase):
    BANNED = ["password123", "Qwerty12345!", "letmein"]

    def test_good_password(self):
        self.assertEqual(password_problems("correct horse battery staple", "ytanaka", self.BANNED), [])

    def test_short(self):
        self.assertEqual(password_problems("Tr0ub4dor&3", "ytanaka"), ["shorter than 12 characters"])

    def test_banned_any_case(self):
        self.assertEqual(password_problems("QWERTY12345!", banned=self.BANNED),
                         ["is a common or breached password"])

    def test_username(self):
        self.assertEqual(password_problems("YTanaka-2026-spring", "ytanaka"), ["contains the username"])
        self.assertEqual(password_problems("bobsled-weather-42", "bo"), [])

    def test_repeated_and_several_problems(self):
        self.assertEqual(password_problems("aaaaaaaa"), ["shorter than 12 characters", "is one character repeated"])
        self.assertEqual(password_problems("letmein", "letme", self.BANNED),
                         ["shorter than 12 characters", "is a common or breached password", "contains the username"])


class PwnedTests(unittest.TestCase):
    def test_prefix(self):
        # SHA-1 of "password" is 5BAA61E4C9B93F3F0682250B6CF8331B7EE68FD8
        self.assertEqual(pwned_prefix("password"), ("5BAA6", "1E4C9B93F3F0682250B6CF8331B7EE68FD8"))

    def test_count(self):
        response = "003D68EB55068C33ACE09247EE4C639306B:3\r\n1E4C9B93F3F0682250B6CF8331B7EE68FD8:52256179\r\n"
        self.assertEqual(pwned_count("1E4C9B93F3F0682250B6CF8331B7EE68FD8", response), 52256179)
        self.assertEqual(pwned_count("1e4c9b93f3f0682250b6cf8331b7ee68fd8", response), 52256179)
        self.assertEqual(pwned_count("FFFFF", response), 0)


if __name__ == "__main__":
    unittest.main()
@@file main.py
"""Security toolkit.

python main.py baseline C:\\inetpub\\wwwroot baseline.json     save hashes of every file
python main.py check C:\\inetpub\\wwwroot baseline.json        exit code 1 if anything changed
python main.py password [--user ytanaka] [--online]          audit a password (typed hidden)
"""

import argparse
import getpass
import json
import sys
import urllib.request
from datetime import datetime
from pathlib import Path

from guard import build_baseline, compare, password_problems, pwned_count, pwned_prefix

HERE = Path(__file__).parent


def cmd_baseline(args):
    base = build_baseline(args.folder)
    doc = {"folder": str(Path(args.folder).resolve()), "created": datetime.now().isoformat(timespec="seconds"), "files": base}
    Path(args.baseline).write_text(json.dumps(doc, indent=2), encoding="utf-8")
    print(f"Saved {len(base)} file hashes to {args.baseline}")
    return 0


def cmd_check(args):
    doc = json.loads(Path(args.baseline).read_text(encoding="utf-8"))
    changes = compare(doc["files"], build_baseline(args.folder))
    total = sum(len(v) for v in changes.values())
    for kind, paths in changes.items():
        for p in paths:
            print(f"  {kind.upper():9} {p}")
    if total:
        print(f"ALERT: {total} change(s) since the baseline of {doc['created']}")
        return 1
    print(f"OK: no changes since {doc['created']}")
    return 0


def cmd_password(args):
    banned = (HERE / "banned.txt").read_text(encoding="utf-8").split()
    password = getpass.getpass("Password to check (not shown): ")
    problems = password_problems(password, args.user, banned)
    if args.online:
        prefix, suffix = pwned_prefix(password)
        req = urllib.request.Request(f"https://api.pwnedpasswords.com/range/{prefix}", headers={"User-Agent": "it-quest-guard"})
        with urllib.request.urlopen(req, timeout=10) as r:
            count = pwned_count(suffix, r.read().decode())
        if count:
            problems.append(f"appears in {count:,} breached accounts (Have I Been Pwned)")
    for p in problems:
        print("  x", p)
    print("Looks good." if not problems else f"{len(problems)} problem(s).")
    return 1 if problems else 0


def main(argv=None):
    parser = argparse.ArgumentParser(description="File integrity monitoring and password auditing")
    sub = parser.add_subparsers(required=True)
    p = sub.add_parser("baseline", help="record file hashes")
    p.add_argument("folder")
    p.add_argument("baseline")
    p.set_defaults(func=cmd_baseline)
    p = sub.add_parser("check", help="compare a folder with its baseline")
    p.add_argument("folder")
    p.add_argument("baseline")
    p.set_defaults(func=cmd_check)
    p = sub.add_parser("password", help="audit a password")
    p.add_argument("--user", default="")
    p.add_argument("--online", action="store_true", help="also check Have I Been Pwned (sends only 5 hash characters)")
    p.set_defaults(func=cmd_password)
    args = parser.parse_args(argv)
    return args.func(args)


if __name__ == "__main__":
    sys.exit(main())
@@file banned.txt
123456
123456789
12345678
password
password1
password123
qwerty
qwerty123
Qwerty12345!
111111
abc123
iloveyou
admin
admin123
welcome
welcome1
letmein
monkey
dragon
football
baseball
sunshine
princess
Passw0rd!
P@ssw0rd
P@ssword1
Summer2026!
Winter2026!
changeme
trustno1

@@drill
How many characters of the SHA-1 hash does the k-anonymity breach check send?
= 5
= five
? The service returns all matching suffixes; the full hash never leaves your machine.

@@text
🎉 Hashes, baselines, detective controls, NIST password guidance and k-anonymity: you've just used five Security+ topics in working code. On your CV: *"Built a file integrity monitor and a breach-aware password auditor in Python."*
