@@lesson n0-errors Reading Error Messages
@@text
Every programmer, every day, writes code that doesn't work the first time. That's normal. The skill is **reading the error message**, which tells you what went wrong and where.

Try this broken program:

```python
print("Checking disk...")
print("Disk is full!"
```

Python answers with something like:

```text
  File "main.py", line 2
    print("Disk is full!"
         ^
SyntaxError: '(' was never closed
```

Read it from the **bottom up**:

1. **The last line** says *what* is wrong: a `SyntaxError` (Python couldn't understand the grammar), because a `(` was never closed.
2. **The line above** points at *where*: `main.py`, line 2.

The fix: add the missing `)`.

## Two kinds of errors

- **Syntax errors**: the code isn't valid Python, so *nothing* runs. Missing brackets, quotes or colons.
- **Runtime errors** (exceptions): the code is valid, starts running, then hits a problem. Python prints a **traceback**:

```python
print("Line 1 runs fine")
print(undefined_name)
print("This line never runs")
```

```text
Traceback (most recent call last):
  File "main.py", line 2, in <module>
    print(undefined_name)
          ^^^^^^^^^^^^^^
NameError: name 'undefined_name' is not defined
```

Notice that line 1 *did* run: the output shows before the error. A `NameError` means you used a name Python doesn't know, usually a typo.

## Your debugging checklist

1. Read the **last line** of the error: the type and message.
2. Go to the **line number** it mentions (and look at the line *before* it too).
3. Check for typos, missing quotes, brackets, colons.
4. Still stuck? Copy the error into the **AI Helper** and ask what it means.

> In IT support you'll read error messages all day: from Windows, from servers, from apps. Getting calm and methodical about them is one of the most valuable skills you can build.

@@quiz
Python shows: `NameError: name 'pirnt' is not defined`. What's the most likely problem?
- Python is broken
* A typo: `pirnt` should be `print`
- The line is too long
- You need to restart the computer
? A `NameError` means Python doesn't know that name. Very often it's a typo.

@@quiz
Where should you start reading a Python traceback?
- At the top
* At the bottom: the last line says what went wrong
- In the middle
? The last line has the error type and message. The lines above it show where it happened.

@@exercise Fix the program
This program has **two** mistakes. Run it, read the errors, and fix them so it prints:

```text
Server: web01
Status: online
```
@@starter
print("Server: web01)
prnt("Status: online")
@@expected
Server: web01
Status: online
@@hint
Fix one error at a time: run, read the last line of the error, fix, run again. The first line is missing a closing quote; the second has a typo in `print`.
@@solution
print("Server: web01")
print("Status: online")
