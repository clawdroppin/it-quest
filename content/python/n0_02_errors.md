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
## The errors you'll meet first

| Error | What it usually means |
|---|---|
| `SyntaxError` | the code isn't valid Python: a missing bracket, quote or colon |
| `IndentationError` | the spaces at the start of a line don't line up |
| `NameError` | a name that doesn't exist: a typo, or a variable used before it was set |
| `TypeError` | an operation on the wrong kind of value, like `"5" + 3` |
| `ValueError` | the right type but an impossible value, like `int("abc")` |
| `ZeroDivisionError` | dividing by zero |
| `IndexError` / `KeyError` | asking a list for a position, or a dictionary for a key, that isn't there |

Learn to recognise these names and you'll know where to look before you've even read the rest of the message.

## Asking for help well

When you're really stuck, explain the problem out loud (or to a rubber duck on your desk); saying it often reveals the mistake. If you ask a person or search online, include the **exact error message**, the line it points to, what you expected and what happened instead. "It doesn't work" can't be answered; "TypeError: can only concatenate str (not "int") to str on line 4" usually can be answered in seconds.

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

@@quiz
`int("abc")` fails. Which error does Python raise?
* ValueError
- NameError
- SyntaxError
- KeyError
? The type (a string) is fine for int(), but this particular value can't be converted.

@@drill
Which kind of error do you get when you forget a closing parenthesis?
= SyntaxError
= syntax error
? Python can't even start running the file.

@@flashcards
SyntaxError :: invalid Python: missing bracket, quote or colon
IndentationError :: line indentation doesn't line up
NameError :: a name that doesn't exist (often a typo)
TypeError :: an operation on the wrong kind of value
ValueError :: right type, impossible value
traceback :: the error report; read it from the bottom up

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
