@@lesson i3-regex Searching Text with Regular Expressions
@@concepts text-processing
@@text
Logs, configs and command output are text. To pull out the IP addresses, error codes or usernames hidden in them, you need **regular expressions** (regex): little patterns that describe text.

```python
import re

line = "2026-10-31 08:15:02 Failed password for ana from 203.0.113.7 port 52211"
m = re.search(r"from (\d+\.\d+\.\d+\.\d+)", line)
if m:
    print("attacker IP:", m.group(1))
```

Always write patterns as raw strings `r"..."` so backslashes reach the regex unchanged.

## The building blocks

| Pattern | Matches |
|---|---|
| `abc` | the exact text abc |
| `.` | any one character |
| `\d` / `\w` / `\s` | a digit / a word character (letter, digit, _) / whitespace |
| `[abc]` / `[0-9]` / `[^x]` | one of a, b, c / a digit / anything but x |
| `*` / `+` / `?` | 0 or more / 1 or more / 0 or 1 of the thing before |
| `{3}` / `{1,3}` | exactly 3 / between 1 and 3 |
| `^` / `$` | start / end of the text |
| `( )` | a group you can extract |
| `\.` | a literal dot (escape special characters with `\`) |

So `\d{1,3}\.\d{1,3}\.\d{1,3}\.\d{1,3}` is "four groups of 1–3 digits separated by dots": an IPv4-looking address.

## The main functions

```python
import re

log = "user=ana id=17; user=ben id=42; user=carl id=7"
print(re.findall(r"user=(\w+)", log))          # every match's group
print(re.findall(r"id=(\d+)", log))
print(re.sub(r"id=\d+", "id=***", log))        # search and replace
print(bool(re.fullmatch(r"[A-Z]{2}-\d{3}", "LT-001")))   # whole text must match
for m in re.finditer(r"user=(\w+) id=(\d+)", log):
    print(m.group(1), "->", int(m.group(2)))
```

- `re.search` finds the **first** match anywhere (or `None`).
- `re.fullmatch` requires the **whole** string to match: perfect for validation.
- `re.findall` returns all matches (the groups, if the pattern has groups).
- `re.sub` replaces matches.

## Named groups

For readability, name your groups:

```python
import re

m = re.search(r"(?P<date>\d{4}-\d{2}-\d{2}) (?P<level>[A-Z]+) (?P<msg>.*)", "2026-10-31 ERROR disk full")
print(m.group("level"), "|", m.group("msg"))
print(m.groupdict())
```

> Regex is powerful but easy to get wrong. Build patterns step by step, test them on real lines, and keep them simple. Sites like regex101.com show what each part matches.

@@quiz
Which pattern matches a whole string of exactly 3 digits, like `404`?
- `\d+`
* `^\d{3}$` (or `re.fullmatch(r"\d{3}", s)`)
- `[0-9]`
- `.{3}`
? `\d{3}` is three digits; `^` and `$` (or `fullmatch`) make sure nothing else is around them.

@@quiz
What does `re.findall(r"\d+", "cpu 91% mem 64%")` return?
* `["91", "64"]`
- `[91, 64]`
- `"91"`
- `["9", "1", "6", "4"]`
? findall returns every match as a string; + means one or more digits.

@@quiz
Why write regex patterns as raw strings like `r"\d+"`?
* So backslashes reach the regex engine unchanged
- Raw strings are faster
- Patterns must be raw strings or they fail
- To make the match case-insensitive
? Without r, Python would treat some backslash sequences as escapes first.

@@drill
Which regex shorthand matches any single digit?
= \d
? \d+ matches one or more digits.

@@flashcards
`\d` / `\w` / `\s` :: digit / word character / whitespace
`+` / `*` / `?` :: one or more / zero or more / optional
`re.search(p, s)` :: first match anywhere, or None
`re.findall(p, s)` :: list of all matches (their groups)
`re.sub(p, new, s)` :: replace matches
`re.fullmatch(p, s)` :: the whole string must match (validation)

@@exercise Extract failed-login IPs
SSH logs record failed logins like `Failed password for ana from 203.0.113.7 port 22`. Write `failed_ips(lines)` that returns a **sorted list of unique IPs** from the lines that contain `Failed password`.
@@starter
import re


def failed_ips(lines):
    pass


log = [
    "Oct 31 08:01 sshd: Accepted password for ben from 10.0.0.4 port 50000",
    "Oct 31 08:02 sshd: Failed password for ana from 203.0.113.7 port 52211",
    "Oct 31 08:02 sshd: Failed password for root from 198.51.100.23 port 40001",
    "Oct 31 08:03 sshd: Failed password for ana from 203.0.113.7 port 52212",
]
print(failed_ips(log))
@@tests
log = [
    "Oct 31 08:01 sshd: Accepted password for ben from 10.0.0.4 port 50000",
    "Oct 31 08:02 sshd: Failed password for ana from 203.0.113.7 port 52211",
    "Oct 31 08:02 sshd: Failed password for root from 198.51.100.23 port 40001",
    "Oct 31 08:03 sshd: Failed password for ana from 203.0.113.7 port 52212",
]
check(failed_ips(log), ["198.51.100.23", "203.0.113.7"], "failed_ips(sample log)")
check(failed_ips([]), [], "empty log")
@@hint
For each line with `"Failed password" in line`, `re.search(r"from (\d+\.\d+\.\d+\.\d+)", line)`; add `m.group(1)` to a set. Return `sorted(...)`.
@@solution
import re


def failed_ips(lines):
    ips = set()
    for line in lines:
        if "Failed password" in line:
            m = re.search(r"from (\d+\.\d+\.\d+\.\d+)", line)
            if m:
                ips.add(m.group(1))
    return sorted(ips)


log = [
    "Oct 31 08:01 sshd: Accepted password for ben from 10.0.0.4 port 50000",
    "Oct 31 08:02 sshd: Failed password for ana from 203.0.113.7 port 52211",
    "Oct 31 08:02 sshd: Failed password for root from 198.51.100.23 port 40001",
    "Oct 31 08:03 sshd: Failed password for ana from 203.0.113.7 port 52212",
]
print(failed_ips(log))

@@exercise Mask sensitive data
Before sharing logs with a vendor, you must hide email addresses. Write `mask_emails(text)` that replaces every email (letters, digits, dots, `_`, `-`, `+` before the `@`; letters, digits, dots and `-` after it) with `[email]`.
@@starter
import re


def mask_emails(text):
    pass


print(mask_emails("reset requested by ana.tanaka@corp.example for bob+test@mail.co.jp"))
@@tests
check(mask_emails("reset requested by ana.tanaka@corp.example for bob+test@mail.co.jp"), "reset requested by [email] for [email]", "two emails")
check(mask_emails("no emails here"), "no emails here", "no emails")
@@hint
`re.sub(r"[\w.+-]+@[\w.-]+", "[email]", text)` — inside `[ ]`, `.` and `+` are literal characters.
@@solution
import re


def mask_emails(text):
    return re.sub(r"[\w.+-]+@[\w.-]+", "[email]", text)


print(mask_emails("reset requested by ana.tanaka@corp.example for bob+test@mail.co.jp"))
@@text
> Hiding personal data (**PII**) in logs is required by privacy laws like the GDPR and Japan's APPI. It's called *data masking* on the Security+ exam.
