@@lesson n1-strings Text & f-strings
@@concepts strings
@@text
Text in Python is a **string** (`str`). You write it in quotes, single or double:

```python
a = "web01"
b = 'web01'
print(a == b)
```

Use the other kind of quote when the text contains one: `"It's down"` or `'She said "hi"'`.

## Joining strings

`+` joins (concatenates) strings, `*` repeats them:

```python
name = "db" + "01"
print(name)
print("-" * 20)
```

You can't `+` a string and a number: `"Disk " + 5` is a `TypeError`. Convert first with `str(5)`, or better, use an f-string.

## f-strings: the way to build text

Put an `f` before the quote, and put any variable or expression inside `{ }`:

```python
host = "web01"
cpu = 37.5
users = 120
print(f"{host}: CPU {cpu}%, {users} users, {users * 2} sessions")
```

Inside the braces you can also **format** the value after a colon:

```python
price = 1234.5678
print(f"{price:.2f}")       # 2 decimals
print(f"{price:,.2f}")      # thousands separator
print(f"{0.8734:.1%}")      # as a percent
print(f"[{'ok':>6}]")       # right-align in 6 characters
print(f"[{'ok':<6}]")       # left-align
print(f"[{42:05}]")         # pad with zeros
```

Alignment makes neat reports:

```python
print(f"{'Server':<10}{'CPU':>6}")
print(f"{'web01':<10}{37.5:>6.1f}")
print(f"{'db01':<10}{91.25:>6.1f}")
```

## String methods

A **method** is a function that belongs to a value; you call it with a dot. Strings have many:

```python
s = "  Web01.Example.COM  "
print(s.strip())          # remove spaces at both ends
print(s.lower())          # all lowercase
print(s.upper())
print(s.strip().replace(".", "-"))
print(s.strip().startswith("Web"))
print("example" in s.lower())   # is it contained?
print(len(s))             # length (a function, not a method)
```

Methods **return a new string**; the original doesn't change. To keep the result, assign it: `s = s.strip()`.

## Indexing and slicing

Each character has a position, starting at **0**. Negative positions count from the end:

```python
code = "ERR-404"
print(code[0])      # first
print(code[-1])     # last
print(code[0:3])    # positions 0, 1, 2 (stop is not included)
print(code[4:])     # from 4 to the end
```

## Special characters

`\n` is a new line and `\t` a tab. For Windows paths, use a **raw string** `r"..."`, where backslashes are just backslashes:

```python
print("line one\nline two")
print(r"C:\Users\ana\new_folder")
```

@@quiz
What does `f"{3.14159:.2f}"` produce?
- `3.14159`
* `3.14`
- `3.1`
- `{3.14159:.2f}`
? `.2f` means "float with 2 decimals".

@@quiz
`name = "  ana  "` then `name.strip()` is called on its own line. What is `name` now?
* Still `"  ana  "`: methods return a new string; you must assign it
- `"ana"`
- `"  ana"`
- An error
? Strings never change in place. Write `name = name.strip()` to keep the result.

@@quiz
What is `"server"[0:3]`?
* `"ser"`
- `"serv"`
- `"s"`
- `"ver"`
? A slice includes the start index and stops before the end index.

@@flashcards
`s.strip()` :: remove whitespace from both ends
`s.lower()` / `s.upper()` :: change case
`s.replace(a, b)` :: replace every a with b
`f"{x:.2f}"` :: format x with 2 decimals
`s[0:3]` :: characters at positions 0, 1, 2
`r"C:\path"` :: raw string: backslashes stay as they are

@@exercise Username generator
New employees get a username: the **first letter** of their first name plus their **last name**, all **lowercase**, followed by `@corp.local`. Spaces around the names must be removed. For the given variables, print:

```text
jtanaka@corp.local
```
@@starter
first = "  Jun "
last = "TANAKA  "

@@expected
jtanaka@corp.local
@@hint
Clean them first: `first = first.strip().lower()`. The first letter is `first[0]`. Then an f-string: `f"{...}{...}@corp.local"`.
@@solution
first = "  Jun "
last = "TANAKA  "

first = first.strip().lower()
last = last.strip().lower()
print(f"{first[0]}{last}@corp.local")

@@exercise Disk report
Print a neat two-line report from these variables. The name is left-aligned in 8 characters, the percentage is shown with **1 decimal** and a `%` sign:

```text
C:      used 73.4% of 512 GB
D:      used 12.0% of 2000 GB
```
@@starter
c_used, c_total = 375.8, 512
d_used, d_total = 240, 2000

@@expected
C:      used 73.4% of 512 GB
D:      used 12.0% of 2000 GB
@@hint
The percentage is `used / total * 100`, formatted with `:.1f`. The drive name is `{'C:':<8}`.
@@solution
c_used, c_total = 375.8, 512
d_used, d_total = 240, 2000

print(f"{'C:':<8}used {c_used / c_total * 100:.1f}% of {c_total} GB")
print(f"{'D:':<8}used {d_used / d_total * 100:.1f}% of {d_total} GB")
