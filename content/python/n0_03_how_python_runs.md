@@lesson n0-how-python-runs How Python Runs Your Code
@@concepts hello, modules
@@text
Before writing bigger programs it helps to know what actually happens when you press Run, and the different ways people run Python at work.

## The interpreter

Python is an **interpreted** language. The program called `python` (or `python3` on Linux and macOS, `py` on Windows) reads your `.py` file and runs it:

1. It **compiles** your source code to **bytecode**, a simpler instruction list (saved in `__pycache__/` folders as `.pyc` files so the next start is faster).
2. The **Python virtual machine** executes the bytecode one instruction at a time.

There's no separate "build" step like in Rust, C# or Go. That makes Python quick to try things in, but it also means many mistakes (a typo in a variable name inside an `if` branch) are only found **when that line runs**. Tests and type checkers (later lessons) close that gap.

You can see the bytecode yourself:

```python
import dis

def area(w, h):
    return w * h

dis.dis(area)
```

## Three ways to run Python

| Way | Command | Good for |
|---|---|---|
| **Script** | `python backup.py` | real programs and automation |
| **REPL** (interactive prompt) | `python`, then type lines at `>>>` | trying an idea, checking what a function returns |
| **Module** | `python -m http.server 8000` | running a tool that ships with Python or a package |

In the REPL, the value of each expression is printed automatically, `_` holds the last result, `help(str.split)` shows documentation and `dir(obj)` lists what an object can do. Exit with `exit()` or Ctrl+Z, Enter (Windows) / Ctrl+D (Linux, macOS).

## Which Python?

- **CPython** is the standard interpreter from python.org (written in C): this is what "Python" usually means.
- Versions matter: new syntax like `match` (3.10) or `except*` (3.11) fails on older Pythons. Check with `python --version`. Servers often run an older version than your laptop.
- On Windows, the **py launcher** picks versions: `py -3.13 script.py`, `py --list`.
- On Linux the system Python belongs to the OS; don't `sudo pip install` into it. Use a **virtual environment** (`python -m venv .venv`), covered in the professional Python lesson, or **uv**, a fast all-in-one tool for Python versions, environments and packages.
- Other interpreters exist: **PyPy** (a JIT compiler, often much faster for pure-Python loops), **MicroPython** (microcontrollers).

## The `if __name__ == "__main__":` line

Every file is also a **module** that other files can `import`. When Python runs a file directly, it sets the variable `__name__` to `"__main__"`; when the file is imported, `__name__` is the module's name. So this pattern means "only run this part when started directly":

```python
def check_disk(path):
    return f"checking {path}"

def main():
    print(check_disk("C:\\"))

if __name__ == "__main__":
    main()
```

Now another script can `import` this file to reuse `check_disk` **without** running the check. Professional scripts almost always use this layout.

## Shebang lines and running scripts directly

On Linux and macOS, a first line like `#!/usr/bin/env python3` plus `chmod +x tool.py` lets you run `./tool.py` directly. On Windows the py launcher reads the same line. It's harmless to include.

## Editors

You can write Python in any text editor, but a good editor shows mistakes as you type. **VS Code** with the Python extension (and Pylance) and **PyCharm** are the common choices: they run your code, debug it step by step, and point out undefined names before you run anything.

@@quiz
What does CPython do with your .py file before running it?
- Translates it to machine code like a C compiler
* Compiles it to bytecode, which the Python virtual machine executes
- Sends it to a server
- Nothing, it reads the text line by line each time
? The bytecode is cached in `__pycache__`.

@@quiz
When is `__name__` equal to `"__main__"`?
* When the file is run directly, not imported
- Always
- When the file is imported by another module
- Only in the REPL
? That's why `if __name__ == "__main__":` guards the "run" part of a script.

@@quiz
Why might a typo inside an `if` branch go unnoticed until later?
- Python ignores typos
* Python reports most errors only when the line actually runs
- The REPL fixes it
- Bytecode hides it
? Tests and type checkers find these earlier.

@@drill
What value does `__name__` have when a file is run directly?
= __main__
= "__main__"
= '__main__'
? When imported, it's the module's name instead.

@@flashcards
CPython :: the standard Python interpreter, written in C
Bytecode :: the simpler instructions Python compiles source code into
REPL :: Read-Eval-Print Loop, the interactive `>>>` prompt
`python -m module` :: run a module as a program, e.g. `python -m http.server`
`__name__ == "__main__"` :: true only when the file is run directly
py launcher :: Windows tool to pick a Python version: `py -3.13`
Virtual environment :: an isolated folder of packages for one project

@@exercise A script that can also be imported
Finish this script so it can be both imported and run:

- `describe(host, up)` returns `"web01 is UP"` or `"web01 is DOWN"`.
- `main()` returns a list with one description per host in `HOSTS` (it doesn't print).
- The bottom of the file prints every line of `main()` **only** when run directly.
@@starter
HOSTS = {"web01": True, "db01": False, "mail01": True}


def describe(host, up):
    pass


def main():
    pass


# add the guard here
@@expected
web01 is UP
db01 is DOWN
mail01 is UP
@@tests
check(describe("x", True), "x is UP", "describe up")
check(describe("x", False), "x is DOWN", "describe down")
check(main(), ["web01 is UP", "db01 is DOWN", "mail01 is UP"], "main returns the lines")
@@requires
if __name__ == "__main__":
@@hint
`return f"{host} is {'UP' if up else 'DOWN'}"`. `main` can be `return [describe(h, up) for h, up in HOSTS.items()]` (or a loop that appends). At the bottom: `if __name__ == "__main__":` then `for line in main(): print(line)`.
@@solution
HOSTS = {"web01": True, "db01": False, "mail01": True}


def describe(host, up):
    return f"{host} is {'UP' if up else 'DOWN'}"


def main():
    lines = []
    for host, up in HOSTS.items():
        lines.append(describe(host, up))
    return lines


if __name__ == "__main__":
    for line in main():
        print(line)

@@exercise Look inside with dir()
Python can tell you about its own objects. Write `public_methods(obj)` returning a **sorted** list of the names from `dir(obj)` that don't start with an underscore. Then write `python_version_ok(version_info, minimum)` that returns True when the version tuple is at least the minimum, e.g. `python_version_ok((3, 12, 1), (3, 10))` is True. (Tuples compare element by element; compare only as many parts as the minimum has.)
@@starter
def public_methods(obj):
    pass


def python_version_ok(version_info, minimum):
    pass


print(public_methods(True)[:3])
@@tests
check(public_methods([])[:4], ["append", "clear", "copy", "count"], "list methods")
check("upper" in public_methods(""), True, "str has upper")
check(any(n.startswith("_") for n in public_methods(5)), False, "no underscore names")
check(python_version_ok((3, 12, 1), (3, 10)), True, "3.12 >= 3.10")
check(python_version_ok((3, 9, 18), (3, 10)), False, "3.9 < 3.10")
check(python_version_ok((3, 10, 0), (3, 10)), True, "equal is ok")
@@hint
`sorted(name for name in dir(obj) if not name.startswith("_"))`. For the version: `tuple(version_info[:len(minimum)]) >= tuple(minimum)`. In real code you'd pass `sys.version_info`.
@@solution
def public_methods(obj):
    return sorted(name for name in dir(obj) if not name.startswith("_"))


def python_version_ok(version_info, minimum):
    return tuple(version_info[:len(minimum)]) >= tuple(minimum)


print(public_methods(True)[:3])
