@@lesson n1-if Decisions: if, elif, else
@@concepts conditions
@@text
Programs need to make decisions: *if* the disk is almost full, send an alert; *otherwise*, do nothing.

## Comparisons and booleans

A comparison gives a **boolean**: `True` or `False`.

```python
disk_used = 91
print(disk_used > 90)
print(disk_used == 91)    # == compares; = assigns!
print(disk_used != 50)    # not equal
print(disk_used <= 100)
print("web" in "web01")
```

## if / elif / else

```python
disk_used = 91

if disk_used >= 95:
    print("CRITICAL: disk almost full")
elif disk_used >= 80:
    print("WARNING: disk filling up")
else:
    print("OK")
print("check done")
```

Three things to notice:

1. Each condition line ends with a **colon** `:`.
2. The lines that belong to it are **indented** (4 spaces). Indentation isn't decoration in Python, it *is* the structure. The last `print` isn't indented, so it always runs.
3. Python tests the conditions **top to bottom** and runs only the **first** block whose condition is true. That's why the order matters: test the most severe case first.

## Combining conditions: and, or, not

```python
cpu = 85
ram = 40
maintenance = False

if cpu > 80 and ram > 80:
    print("server overloaded")
if cpu > 80 or ram > 80:
    print("something is busy")
if not maintenance:
    print("alerts are on")
```

- `a and b` is True only if **both** are.
- `a or b` is True if **at least one** is.
- `not a` flips True and False.

Python even lets you chain comparisons: `if 18 <= age <= 65:`.

## Truthiness

In an `if`, empty things count as False: `0`, `""` (empty string), empty lists. Everything else is True:

```python
username = ""
if username:
    print(f"Hi {username}")
else:
    print("No username given")
```

@@quiz
`x = 50`. What does this print?

```text
if x > 10:
    print("A")
elif x > 40:
    print("B")
else:
    print("C")
```
* A
- B
- A and B
- C
? Only the **first** true branch runs. `x > 10` is already true, so `elif` is never checked.

@@quiz
Which line has a bug?
- `if count == 0:`
* `if count = 0:`
- `if not count:`
- `if count != 0:`
? `=` assigns, `==` compares. Python gives a SyntaxError for `if count = 0:`.

@@quiz
What does `bool("")` return?
* False
- True
- None
- an error
? Empty strings, 0, empty lists and None are "falsy".

@@drill
Which keyword adds another condition after an `if`?
= elif
? Short for "else if".

@@flashcards
== :: equal to (one = assigns, two compare)
!= :: not equal to
if / elif / else :: run the first block whose condition is true
and / or / not :: combine or flip conditions
truthy / falsy :: values treated as True / False in an if
indentation :: the spaces that mark which lines belong to the if

@@exercise Password policy
Read a password from the input. Print `strong` if it's **at least 12 characters** long **and** contains **no spaces**, `weak` if it's at least 8 characters (and no spaces), otherwise `rejected`. A password with a space is always `rejected`.

For the input `CorrectHorse42` the output is `strong`.
@@stdin
CorrectHorse42
@@starter
password = input()

@@expected
strong
@@hint
Check the "has a space" case first: `if " " in password:`. Then `elif len(password) >= 12:` and so on.
@@solution
password = input()

if " " in password:
    print("rejected")
elif len(password) >= 12:
    print("strong")
elif len(password) >= 8:
    print("weak")
else:
    print("rejected")

@@exercise Severity levels
Monitoring tools give each event a numeric severity. Read the number and print its name: `0` is `emergency`, `1`–`3` is `error`, `4` is `warning`, `5`–`6` is `info`, `7` is `debug`. Anything else prints `invalid`.

For the input `4` the output is `warning`.
@@stdin
4
@@starter
level = int(input())

@@expected
warning
@@hint
Use a chain of `if` / `elif`. For ranges use `1 <= level <= 3`.
@@solution
level = int(input())

if level == 0:
    print("emergency")
elif 1 <= level <= 3:
    print("error")
elif level == 4:
    print("warning")
elif 5 <= level <= 6:
    print("info")
elif level == 7:
    print("debug")
else:
    print("invalid")
@@text
> These numbers loosely follow **syslog severity levels**, the standard on Linux and network devices (the real list has 8 named levels: emergency, alert, critical, error, warning, notice, informational, debug). You'll meet syslog again in Linux+ and Network+.
