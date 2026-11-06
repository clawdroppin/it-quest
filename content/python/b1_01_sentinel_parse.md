@@lesson b1-sentinel-parse Log Sentinel 1: Parsing the Auth Log
@@text
Every Linux server exposed to the internet is attacked within minutes: bots try thousands of usernames and passwords over SSH. Every attempt is written to the **authentication log** (`/var/log/auth.log` on Ubuntu, `/var/log/secure` on Red Hat). In this build you'll write **Log Sentinel**, a command-line tool that reads that log and reports:

- **brute-force** sources: IPs with many failed logins,
- the **most targeted** usernames,
- who tried to log in as **root**,
- and the scariest case: an IP that failed many times and then **succeeded**, meaning a password was probably guessed.

This is exactly what a SOC analyst (Security Operations Center) does on day one, and what tools like **fail2ban** automate.

## How builds work

Each step has a **project card**. Press **📁 Create project** to write the starting files into a folder, **💻 Open in VS Code** to edit them, and **🧪 run tests** to see which parts still fail. The tests describe exactly what each function must do. When every test passes, the step is done. Stuck? The **🤖 AI Helper** can see your project files.

## The log format

```text
Oct 31 08:15:02 web01 sshd[1234]: Failed password for root from 203.0.113.7 port 52211 ssh2
Oct 31 08:15:09 web01 sshd[1234]: Failed password for invalid user admin from 203.0.113.7 port 52213 ssh2
Oct 31 08:20:41 web01 sshd[1301]: Accepted publickey for ana from 10.0.0.4 port 50022 ssh2
```

Date and time, the host name, the program (`sshd` with its process id), and the message. We care about two kinds of messages: `Failed password for ...` and `Accepted password/publickey for ...`. Note the `invalid user` variant: the attacker tried a username that doesn't even exist.

## Step 1: parse lines into events

In `logparse.py` you'll turn each line into an `Event` (a dataclass with `time`, `kind`, `user` and `ip`), and ignore everything else. The regular expressions are given; your job is to use them.

@@project sentinel Log Sentinel, step 1: the parser
Write `parse_line` and `parse_file` in `logparse.py` until `test_logparse.py` passes. Run the tests with **🧪 run tests**; take a first look at the events with **▶ python peek.py**.
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
    """Return an Event for a failed or accepted SSH login line, or None for any other line.

    Steps:
    1. Match LINE against the stripped line. No match -> None.
    2. Search the message (group "msg") with FAILED, then with ACCEPTED.
    3. Build the time as "month day time", e.g. "Oct 31 08:15:02".
    """
    raise NotImplementedError("TODO: parse_line")


def parse_file(path):
    """Return the list of Events in the file at `path`, skipping lines that aren't logins."""
    raise NotImplementedError("TODO: parse_file")
@@file peek.py
"""A first look at the parsed events. (Step 2 adds the real tool, main.py.)"""

from logparse import parse_file

events = parse_file("sample_auth.log")
print(f"{len(events)} login events")
for e in events[:5]:
    print(e)
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
Which `re` function tries to match a pattern at the start of a string?
= re.match
= match
? re.search looks anywhere in the string.

@@text
When all six tests pass, `python peek.py` prints the first events. On to step 2: the detection rules.
