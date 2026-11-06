@@lesson b1-sentinel-rules Log Sentinel 2: Detection Rules and the CLI
@@text
You can turn a log into events. Now the interesting part: **detection rules**, each a small function that takes the list of events and returns findings.

| Rule | Question it answers |
|---|---|
| `brute_force` | Which IPs failed to log in at least N times? |
| `compromised` | Did an IP that failed many times then **succeed**? (A guessed password: the most urgent finding.) |
| `root_attempts` | Who tried to log in as `root` directly? (Should be disabled: `PermitRootLogin no`.) |
| `targeted_users` | Which usernames are attacked most? |

Then `main.py` becomes a real CLI with `argparse`:

```text
python main.py                       # the sample log, readable report
python main.py /var/log/auth.log --threshold 10
python main.py --json > report.json  # for other tools (a SIEM) to read
```

It exits with code **1** when it finds a compromise, so a scheduled job or monitoring system can raise an alarm.

**Create** the new files with the card below: your `logparse.py` from step 1 is kept as it is. If you didn't finish step 1, the finished version of the whole project is the second card.

@@project sentinel Log Sentinel, step 2: rules and CLI
Write the four functions in `rules.py` until `test_rules.py` passes, then try the CLI: `python main.py` and `python main.py --json`.
@@file rules.py
"""Detection rules. Each takes a list of logparse.Event and returns findings."""

from collections import Counter


def brute_force(events, threshold=5):
    """IPs with at least `threshold` failed logins, as (ip, count) pairs,
    most failures first (ties: by ip)."""
    raise NotImplementedError("TODO: brute_force")


def compromised(events, threshold=3):
    """(user, ip) pairs for every accepted login from an IP that had at least
    `threshold` failed logins *before* it (events are in time order)."""
    raise NotImplementedError("TODO: compromised")


def root_attempts(events):
    """Sorted list of the unique IPs that tried to log in as root (failed or not)."""
    raise NotImplementedError("TODO: root_attempts")


def targeted_users(events, top=3):
    """The `top` usernames with the most failed logins, as (user, count) pairs,
    most failures first (ties: by user name)."""
    raise NotImplementedError("TODO: targeted_users")
@@file main.py
"""Log Sentinel: find attacks in SSH authentication logs.

    python main.py [logfile] [--threshold N] [--json]

Exit code: 0 = nothing urgent, 1 = a probable compromise was found, 2 = the log couldn't be read.
"""

import argparse
import json
import sys

from logparse import parse_file
from rules import brute_force, compromised, root_attempts, targeted_users


def build_report(events, threshold):
    return {
        "events": len(events),
        "failed": sum(1 for e in events if e.kind == "failed"),
        "brute_force": brute_force(events, threshold),
        "compromised": compromised(events),
        "root_attempts": root_attempts(events),
        "targeted_users": targeted_users(events),
    }


def print_report(r):
    print(f"Log Sentinel: {r['events']} login events, {r['failed']} failed")
    print()
    if r["compromised"]:
        print("!!! PROBABLE COMPROMISE: success after repeated failures")
        for user, ip in r["compromised"]:
            print(f"    {user} from {ip}")
        print()
    print("Brute-force sources:")
    for ip, n in r["brute_force"] or [("(none)", "")]:
        print(f"    {ip:<16} {n}")
    print("Root login attempts from:", ", ".join(r["root_attempts"]) or "(none)")
    print("Most targeted users:", ", ".join(f"{u} ({n})" for u, n in r["targeted_users"]))


def main(argv=None):
    parser = argparse.ArgumentParser(description="Find attacks in SSH authentication logs.")
    parser.add_argument("logfile", nargs="?", default="sample_auth.log")
    parser.add_argument("--threshold", type=int, default=5, help="failed logins that count as brute force")
    parser.add_argument("--json", action="store_true", help="print the report as JSON")
    args = parser.parse_args(argv)

    try:
        events = parse_file(args.logfile)
    except OSError as e:
        print(f"Can't read {args.logfile}: {e}", file=sys.stderr)
        return 2
    report = build_report(events, args.threshold)
    if args.json:
        print(json.dumps(report, indent=2))
    else:
        print_report(report)
    return 1 if report["compromised"] else 0


if __name__ == "__main__":
    sys.exit(main())
@@file test_rules.py
import unittest

from logparse import Event, parse_file
from rules import brute_force, compromised, root_attempts, targeted_users

SAMPLE = parse_file("sample_auth.log")


class RuleTests(unittest.TestCase):
    def test_brute_force_default(self):
        self.assertEqual(brute_force(SAMPLE), [("203.0.113.7", 6)])

    def test_brute_force_lower_threshold(self):
        self.assertEqual(brute_force(SAMPLE, 3), [("203.0.113.7", 6), ("198.51.100.23", 4), ("192.0.2.99", 3)])

    def test_compromised(self):
        self.assertEqual(compromised(SAMPLE), [("ben", "198.51.100.23")])

    def test_compromised_needs_failures_before(self):
        events = [Event("t1", "accepted", "ana", "1.1.1.1")] + [Event("t", "failed", "ana", "1.1.1.1")] * 5
        self.assertEqual(compromised(events), [])

    def test_root_attempts(self):
        self.assertEqual(root_attempts(SAMPLE), ["192.0.2.99", "203.0.113.7"])

    def test_targeted_users(self):
        self.assertEqual(targeted_users(SAMPLE), [("ben", 4), ("root", 4), ("admin", 2)])

    def test_empty(self):
        self.assertEqual(brute_force([]), [])
        self.assertEqual(compromised([]), [])
        self.assertEqual(root_attempts([]), [])
        self.assertEqual(targeted_users([]), [])


if __name__ == "__main__":
    unittest.main()
@@project sentinel_finished Log Sentinel: the finished version
The complete project, with every test passing. Compare it with yours, or use it as a starting point for your own ideas: read `/var/log/secure` too, add a `--since` time filter, or write the IPs to block into a firewall rule file.
@@file logparse.py
"""Parse Linux SSH authentication log lines (like /var/log/auth.log) into events."""

import re
from dataclasses import dataclass

# "Oct 31 08:15:02 web01 sshd[1234]: <message>"
LINE = re.compile(
    r"^(?P<month>\w{3})\s+(?P<day>\d{1,2}) (?P<time>\d\d:\d\d:\d\d) (?P<host>\S+) sshd\[\d+\]: (?P<msg>.*)$"
)
FAILED = re.compile(r"Failed password for (?:invalid user )?(?P<user>\S+) from (?P<ip>[\d.]+)")
ACCEPTED = re.compile(r"Accepted (?:password|publickey) for (?P<user>\S+) from (?P<ip>[\d.]+)")


@dataclass
class Event:
    time: str  # "Oct 31 08:15:02"
    kind: str  # "failed" or "accepted"
    user: str
    ip: str


def parse_line(line):
    """Return an Event for a failed or accepted SSH login line, or None for any other line."""
    m = LINE.match(line.strip())
    if m is None:
        return None
    time = f"{m.group('month')} {m.group('day')} {m.group('time')}"
    for kind, pattern in (("failed", FAILED), ("accepted", ACCEPTED)):
        found = pattern.search(m.group("msg"))
        if found:
            return Event(time, kind, found.group("user"), found.group("ip"))
    return None


def parse_file(path):
    """Return the list of Events in the file at `path`, skipping lines that aren't logins."""
    with open(path, encoding="utf-8", errors="replace") as f:
        return [e for e in (parse_line(line) for line in f) if e is not None]
@@file rules.py
"""Detection rules. Each takes a list of logparse.Event and returns findings."""

from collections import Counter


def brute_force(events, threshold=5):
    """IPs with at least `threshold` failed logins, as (ip, count) pairs,
    most failures first (ties: by ip)."""
    counts = Counter(e.ip for e in events if e.kind == "failed")
    found = [(ip, n) for ip, n in counts.items() if n >= threshold]
    return sorted(found, key=lambda pair: (-pair[1], pair[0]))


def compromised(events, threshold=3):
    """(user, ip) pairs for every accepted login from an IP that had at least
    `threshold` failed logins *before* it (events are in time order)."""
    failures = Counter()
    found = []
    for e in events:
        if e.kind == "failed":
            failures[e.ip] += 1
        elif failures[e.ip] >= threshold:
            found.append((e.user, e.ip))
    return found


def root_attempts(events):
    """Sorted list of the unique IPs that tried to log in as root (failed or not)."""
    return sorted({e.ip for e in events if e.user == "root"})


def targeted_users(events, top=3):
    """The `top` usernames with the most failed logins, as (user, count) pairs,
    most failures first (ties: by user name)."""
    counts = Counter(e.user for e in events if e.kind == "failed")
    return sorted(counts.items(), key=lambda pair: (-pair[1], pair[0]))[:top]
@@file main.py
"""Log Sentinel: find attacks in SSH authentication logs.

    python main.py [logfile] [--threshold N] [--json]

Exit code: 0 = nothing urgent, 1 = a probable compromise was found, 2 = the log couldn't be read.
"""

import argparse
import json
import sys

from logparse import parse_file
from rules import brute_force, compromised, root_attempts, targeted_users


def build_report(events, threshold):
    return {
        "events": len(events),
        "failed": sum(1 for e in events if e.kind == "failed"),
        "brute_force": brute_force(events, threshold),
        "compromised": compromised(events),
        "root_attempts": root_attempts(events),
        "targeted_users": targeted_users(events),
    }


def print_report(r):
    print(f"Log Sentinel: {r['events']} login events, {r['failed']} failed")
    print()
    if r["compromised"]:
        print("!!! PROBABLE COMPROMISE: success after repeated failures")
        for user, ip in r["compromised"]:
            print(f"    {user} from {ip}")
        print()
    print("Brute-force sources:")
    for ip, n in r["brute_force"] or [("(none)", "")]:
        print(f"    {ip:<16} {n}")
    print("Root login attempts from:", ", ".join(r["root_attempts"]) or "(none)")
    print("Most targeted users:", ", ".join(f"{u} ({n})" for u, n in r["targeted_users"]))


def main(argv=None):
    parser = argparse.ArgumentParser(description="Find attacks in SSH authentication logs.")
    parser.add_argument("logfile", nargs="?", default="sample_auth.log")
    parser.add_argument("--threshold", type=int, default=5, help="failed logins that count as brute force")
    parser.add_argument("--json", action="store_true", help="print the report as JSON")
    args = parser.parse_args(argv)

    try:
        events = parse_file(args.logfile)
    except OSError as e:
        print(f"Can't read {args.logfile}: {e}", file=sys.stderr)
        return 2
    report = build_report(events, args.threshold)
    if args.json:
        print(json.dumps(report, indent=2))
    else:
        print_report(report)
    return 1 if report["compromised"] else 0


if __name__ == "__main__":
    sys.exit(main())
@@file test_logparse.py
import unittest

from logparse import Event, parse_file, parse_line


class ParseLineTests(unittest.TestCase):
    def test_failed(self):
        e = parse_line("Oct 31 08:15:02 web01 sshd[1234]: Failed password for root from 203.0.113.7 port 52211 ssh2")
        self.assertEqual(e, Event("Oct 31 08:15:02", "failed", "root", "203.0.113.7"))

    def test_invalid_user(self):
        e = parse_line("Oct 31 08:15:09 web01 sshd[1234]: Failed password for invalid user admin from 203.0.113.7 port 52213 ssh2")
        self.assertEqual(e.user, "admin")
        self.assertEqual(e.kind, "failed")

    def test_accepted_publickey(self):
        e = parse_line("Oct 31 08:20:41 web01 sshd[1301]: Accepted publickey for ana from 10.0.0.4 port 50022 ssh2")
        self.assertEqual(e, Event("Oct 31 08:20:41", "accepted", "ana", "10.0.0.4"))

    def test_single_digit_day(self):
        e = parse_line("Nov  2 01:02:03 web01 sshd[9]: Accepted password for ben from 10.0.0.9 port 1 ssh2")
        self.assertEqual(e.time, "Nov 2 01:02:03")

    def test_other_lines_are_ignored(self):
        self.assertIsNone(parse_line("Oct 31 08:00:00 web01 sshd[1]: Connection closed by 10.0.0.4 port 50022"))
        self.assertIsNone(parse_line("Oct 31 08:00:00 web01 CRON[77]: Failed password for x from 1.2.3.4"))
        self.assertIsNone(parse_line(""))


class ParseFileTests(unittest.TestCase):
    def test_sample(self):
        events = parse_file("sample_auth.log")
        self.assertEqual(len(events), 17)
        self.assertEqual(sum(1 for e in events if e.kind == "accepted"), 3)


if __name__ == "__main__":
    unittest.main()
@@file test_rules.py
import unittest

from logparse import Event, parse_file
from rules import brute_force, compromised, root_attempts, targeted_users

SAMPLE = parse_file("sample_auth.log")


class RuleTests(unittest.TestCase):
    def test_brute_force_default(self):
        self.assertEqual(brute_force(SAMPLE), [("203.0.113.7", 6)])

    def test_brute_force_lower_threshold(self):
        self.assertEqual(brute_force(SAMPLE, 3), [("203.0.113.7", 6), ("198.51.100.23", 4), ("192.0.2.99", 3)])

    def test_compromised(self):
        self.assertEqual(compromised(SAMPLE), [("ben", "198.51.100.23")])

    def test_compromised_needs_failures_before(self):
        events = [Event("t1", "accepted", "ana", "1.1.1.1")] + [Event("t", "failed", "ana", "1.1.1.1")] * 5
        self.assertEqual(compromised(events), [])

    def test_root_attempts(self):
        self.assertEqual(root_attempts(SAMPLE), ["192.0.2.99", "203.0.113.7"])

    def test_targeted_users(self):
        self.assertEqual(targeted_users(SAMPLE), [("ben", 4), ("root", 4), ("admin", 2)])

    def test_empty(self):
        self.assertEqual(brute_force([]), [])
        self.assertEqual(compromised([]), [])
        self.assertEqual(root_attempts([]), [])
        self.assertEqual(targeted_users([]), [])


if __name__ == "__main__":
    unittest.main()
@@file sample_auth.log
Oct 31 08:00:01 web01 sshd[1100]: Server listening on 0.0.0.0 port 22.
Oct 31 08:01:12 web01 sshd[1201]: Accepted publickey for ana from 10.0.0.4 port 50022 ssh2
Oct 31 08:15:02 web01 sshd[1234]: Failed password for root from 203.0.113.7 port 52211 ssh2
Oct 31 08:15:05 web01 sshd[1234]: Failed password for root from 203.0.113.7 port 52212 ssh2
Oct 31 08:15:09 web01 sshd[1234]: Failed password for invalid user admin from 203.0.113.7 port 52213 ssh2
Oct 31 08:15:12 web01 sshd[1234]: Failed password for invalid user admin from 203.0.113.7 port 52214 ssh2
Oct 31 08:15:16 web01 sshd[1234]: Failed password for invalid user test from 203.0.113.7 port 52215 ssh2
Oct 31 08:15:20 web01 sshd[1234]: Failed password for root from 203.0.113.7 port 52216 ssh2
Oct 31 08:15:23 web01 sshd[1234]: Connection closed by 203.0.113.7 port 52216 [preauth]
Oct 31 09:02:44 web01 sshd[1290]: Failed password for ben from 198.51.100.23 port 40001 ssh2
Oct 31 09:02:49 web01 sshd[1290]: Failed password for ben from 198.51.100.23 port 40002 ssh2
Oct 31 09:02:53 web01 sshd[1290]: Failed password for ben from 198.51.100.23 port 40003 ssh2
Oct 31 09:02:58 web01 sshd[1290]: Failed password for ben from 198.51.100.23 port 40004 ssh2
Oct 31 09:03:04 web01 sshd[1290]: Accepted password for ben from 198.51.100.23 port 40005 ssh2
Oct 31 09:30:00 web01 CRON[2001]: pam_unix(cron:session): session opened for user root
Oct 31 10:11:07 web01 sshd[1340]: Failed password for ana from 10.0.0.4 port 50101 ssh2
Oct 31 10:11:15 web01 sshd[1340]: Accepted password for ana from 10.0.0.4 port 50102 ssh2
Oct 31 11:45:30 web01 sshd[1402]: Failed password for invalid user oracle from 192.0.2.99 port 33001 ssh2
Oct 31 11:45:33 web01 sshd[1402]: Failed password for invalid user postgres from 192.0.2.99 port 33002 ssh2
Oct 31 11:45:37 web01 sshd[1402]: Failed password for root from 192.0.2.99 port 33003 ssh2
Oct 31 11:45:40 web01 sshd[1402]: Disconnected from invalid user postgres 192.0.2.99 port 33002 [preauth]

@@drill
Which `collections` class counts failed logins per IP in one line?
= Counter
= collections.Counter
? Counter(ips).most_common(5).

@@text
🎉 You've built a real security tool. Ideas to take it further:

- **Block attackers**: write the brute-force IPs as Windows Firewall or `ufw` rules (the PowerShell and Bash tracks show how).
- **Watch live**: read new lines as they're added (like `tail -f`), using a generator.
- **GeoIP**: show which country each attacking IP is in.

This project is also great **portfolio** material: put it on GitHub with a README showing sample output. Hiring managers for SOC and sysadmin roles love seeing it.
