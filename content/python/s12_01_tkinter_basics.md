@@lesson s12-tk-basics Your First Desktop App with Tkinter
@@concepts objects, functions
@@text
This module builds real desktop applications with **Tkinter**, the GUI toolkit that ships with Python on Windows and macOS (on Linux: `sudo apt install python3-tk`). No installs, a single file, and it runs everywhere: perfect for internal IT tools like a password generator, a log viewer or an asset lookup app.

> The GUI examples open windows, so they're shown without a Run button: copy them into a `.py` file and run them on your computer. The exercises test the logic behind the windows, which is where most bugs live anyway.

## The smallest app

```python,norun
import tkinter as tk
from tkinter import ttk

root = tk.Tk()                         # the main window
root.title("Ping Tool")
root.geometry("320x120")

label = ttk.Label(root, text="Host:")
entry = ttk.Entry(root, width=25)
result = ttk.Label(root, text="")

def on_ping():
    host = entry.get().strip()
    result.config(text=f"Pinging {host}..." if host else "Enter a host")

button = ttk.Button(root, text="Ping", command=on_ping)

label.pack(pady=(10, 0))
entry.pack()
button.pack(pady=5)
result.pack()

entry.focus()
root.bind("<Return>", lambda event: on_ping())
root.mainloop()                        # hand control to Tk until the window closes
```

## The three ideas behind every GUI

1. **Widgets**: the building blocks (labels, entries, buttons, lists). Each is created with its **parent** as the first argument, forming a tree: window → frames → widgets.
2. **Layout**: a geometry manager (`pack`, `grid` or `place`) decides where each widget goes. A widget that is never packed/gridded is invisible.
3. **The event loop**: `mainloop()` waits for events (clicks, key presses, timers, redraw requests) and calls your **callbacks**. Your code runs only inside callbacks, and while a callback runs, the window can't redraw. A slow callback freezes the app (the background-work lesson fixes this).

This is the same model as Rust's egui/iced, WinForms, WPF, Qt and the browser's DOM.

## tk vs ttk

`tkinter` has two widget sets:

- classic `tk.Button`, `tk.Label`... look dated but allow direct colours.
- **themed** `ttk.Button`, `ttk.Label`, `ttk.Entry`, `ttk.Treeview`, `ttk.Notebook`, `ttk.Progressbar`... use the native look on Windows and macOS and are styled with themes.

Prefer `ttk` for everything it offers. Some widgets exist only in `tk` (`Text`, `Canvas`, `Listbox`, `Menu`).

## Common widgets

| Widget | Use |
|---|---|
| `ttk.Label` | text or an image |
| `ttk.Button` | an action (`command=`) |
| `ttk.Entry` | one line of input (`.get()`, `.insert()`, `.delete(0, "end")`) |
| `tk.Text` | multi-line text, logs, notes |
| `ttk.Checkbutton` / `ttk.Radiobutton` | options |
| `ttk.Combobox` | drop-down choice |
| `ttk.Spinbox` / `ttk.Scale` | numbers |
| `ttk.Treeview` | tables and trees with columns |
| `ttk.Notebook` | tabs |
| `ttk.Progressbar` | progress |
| `tk.Canvas` | drawing, charts, custom widgets |
| `tk.Menu` | menu bars and context menus |

## Configuring widgets

Options are set at creation (`ttk.Label(root, text="Hi")`) or later (`label.config(text="Bye")` or `label["text"] = "Bye"`). Read them with `label.cget("text")`.

## Organising the code as a class

As apps grow, wrap the window in a class: widgets become attributes, callbacks become methods:

```python,norun
import tkinter as tk
from tkinter import ttk

class PasswordApp(ttk.Frame):
    def __init__(self, master):
        super().__init__(master, padding=12)
        self.length = tk.IntVar(value=16)
        ttk.Label(self, text="Length").grid(row=0, column=0, sticky="w")
        ttk.Spinbox(self, from_=8, to=64, textvariable=self.length, width=5).grid(row=0, column=1)
        ttk.Button(self, text="Generate", command=self.generate).grid(row=1, column=0, columnspan=2, pady=6)
        self.output = ttk.Entry(self, width=40)
        self.output.grid(row=2, column=0, columnspan=2)

    def generate(self):
        import secrets, string
        alphabet = string.ascii_letters + string.digits + "-_!?"
        pw = "".join(secrets.choice(alphabet) for _ in range(self.length.get()))
        self.output.delete(0, "end")
        self.output.insert(0, pw)

root = tk.Tk()
root.title("Password Generator")
PasswordApp(root).pack(fill="both", expand=True)
root.mainloop()
```

The next lessons cover layout, events, separating logic from the window, styling, dialogs, drawing, background work, and packaging the app as an `.exe`.

## Other Python GUI options

| Toolkit | Notes |
|---|---|
| **PySide6 / PyQt6** | Qt: professional, huge widget set, Qt Designer; larger download |
| **Flet** | Flutter-based, modern look, also web and mobile |
| **NiceGUI** | GUI in the browser from Python |
| **CustomTkinter** | modern-looking Tkinter widgets |
| **Kivy** / **BeeWare Toga** | touch and mobile |
| **pywebview** | an HTML/JS front end with a Python back end (like Tauri) |

What you learn with Tkinter (widgets, layout, events, MVC) transfers to all of them.

@@quiz
What does `root.mainloop()` do?
* Runs the event loop: waits for events and calls your callbacks until the window closes
- Draws the window once and exits
- Starts a new thread
- Compiles the GUI
? Code after mainloop runs only after the window is closed.

@@quiz
Why does a long-running callback freeze the window?
- Tkinter has a bug
* While the callback runs, the event loop can't process redraws or clicks
- Windows limits callbacks
- Callbacks run in another process
? Move slow work to a thread or split it with `after`.

@@quiz
Why prefer `ttk` widgets?
- They are faster to type
* They use the platform's native look and support themes
- tk widgets are removed
- ttk works without a window
? Use tk only for widgets ttk doesn't have (Text, Canvas, Listbox, Menu).

@@drill
Which method starts Tkinter's event loop?
= mainloop
= mainloop()
= root.mainloop()
? Nothing appears until it runs.

@@flashcards
Tkinter :: Python's built-in GUI toolkit
Widget :: a GUI element such as a button or entry
Parent / master :: the container a widget belongs to
Geometry manager :: pack, grid or place; positions widgets
Event loop :: `mainloop()`, dispatches events to callbacks
Callback :: function the toolkit calls when an event happens
ttk :: themed Tk widgets with a native look
`config` :: change widget options after creation

@@exercise The logic behind the ping window
Separate the logic from the window: write `ping_message(text)` that the button callback would call. It returns the text for the result label:

- stripped empty input → `"Enter a host"`
- a valid IPv4 address (4 numbers 0–255) or a host name (letters, digits, hyphens and dots; doesn't start or end with `-` or `.`; max 253 characters) → `"Pinging <host>..."`
- anything else → `"Invalid host: <text>"` (stripped text)

Use the `ipaddress` module for IP checking.
@@starter
import ipaddress
import re


def ping_message(text):
    pass


for t in ["", "  web01.corp.example ", "10.0.0.300", "10.0.0.5", "-bad", "a b"]:
    print(repr(ping_message(t)))
@@expected
'Enter a host'
'Pinging web01.corp.example...'
'Invalid host: 10.0.0.300'
'Pinging 10.0.0.5...'
'Invalid host: -bad'
'Invalid host: a b'
@@tests
check(ping_message("localhost"), "Pinging localhost...", "single label")
check(ping_message("a" * 254), "Invalid host: " + "a" * 254, "too long")
check(ping_message("host."), "Invalid host: host.", "trailing dot")
@@hint
Try `ipaddress.IPv4Address(host)` in a `try`; on `ValueError`, fall back to a regex. But careful: `"10.0.0.300"` matches a host-name regex too, so treat anything made only of digits and dots as an IP attempt. A host-name regex: `r"[A-Za-z0-9]([A-Za-z0-9.-]*[A-Za-z0-9])?"` with `re.fullmatch`.
@@solution
import ipaddress
import re

HOST = re.compile(r"[A-Za-z0-9]([A-Za-z0-9.-]*[A-Za-z0-9])?")


def ping_message(text):
    host = text.strip()
    if not host:
        return "Enter a host"
    if re.fullmatch(r"[\d.]+", host):
        try:
            ipaddress.IPv4Address(host)
            return f"Pinging {host}..."
        except ValueError:
            return f"Invalid host: {host}"
    if len(host) <= 253 and HOST.fullmatch(host):
        return f"Pinging {host}..."
    return f"Invalid host: {host}"


for t in ["", "  web01.corp.example ", "10.0.0.300", "10.0.0.5", "-bad", "a b"]:
    print(repr(ping_message(t)))

@@exercise A password generator's model
Write the logic for the Password Generator window as a class the window can call:

- `PasswordOptions(length=16, digits=True, symbols=True, avoid_ambiguous=False)`
- `alphabet(options)`: ASCII letters, plus digits if enabled, plus `"-_!?@#"` if symbols are enabled; if `avoid_ambiguous`, remove the characters `"Il1O0o"`
- `generate(options, rng)`: `length` characters picked with `rng.choice(alphabet)`; when digits/symbols are enabled the result must contain **at least one** of each enabled kind: keep generating until it does. Raise `ValueError("length must be 8-64")` for other lengths.
- `strength(password)`: `"weak"` below 10 characters, `"strong"` for 14+ characters with at least 3 of the 4 classes (lower, upper, digit, other), otherwise `"medium"`

`rng` is passed in (a `random.Random(seed)` in tests, `secrets.SystemRandom()` in the real app) so the tests are reproducible.
@@starter
import random
import string
from dataclasses import dataclass


@dataclass
class PasswordOptions:
    length: int = 16
    digits: bool = True
    symbols: bool = True
    avoid_ambiguous: bool = False


def alphabet(options):
    pass


def generate(options, rng):
    pass


def strength(password):
    pass


pw = generate(PasswordOptions(length=12), random.Random(7))
print(len(pw), strength(pw), strength("abc"), strength("Tr0ub4dor&3xyz"))
@@expected
12 medium weak strong
@@tests
opts = PasswordOptions(length=20, avoid_ambiguous=True)
for seed in range(30):
    pw = generate(opts, random.Random(seed))
    check(len(pw), 20, "length")
    check(any(c.isdigit() for c in pw) and any(c in "-_!?@#" for c in pw), True, "has digit and symbol")
    check(any(c in "Il1O0o" for c in pw), False, "no ambiguous")
check(alphabet(PasswordOptions(digits=False, symbols=False)), string.ascii_letters, "letters only")
try:
    generate(PasswordOptions(length=4), random.Random(1))
    check("generated", "ValueError", "too short")
except ValueError:
    pass
check(strength("abcdefghijklmn"), "medium", "long but one class")
@@hint
Build the alphabet as a string, then `"".join(c for c in chars if c not in "Il1O0o")` if needed. In `generate`, loop: `pw = "".join(rng.choice(chars) for _ in range(options.length))`, and `return pw` once the required kinds are present. For strength, count classes with `sum([any(c.islower() ...), ...])`.
@@solution
import random
import string
from dataclasses import dataclass

SYMBOLS = "-_!?@#"


@dataclass
class PasswordOptions:
    length: int = 16
    digits: bool = True
    symbols: bool = True
    avoid_ambiguous: bool = False


def alphabet(options):
    chars = string.ascii_letters
    if options.digits:
        chars += string.digits
    if options.symbols:
        chars += SYMBOLS
    if options.avoid_ambiguous:
        chars = "".join(c for c in chars if c not in "Il1O0o")
    return chars


def generate(options, rng):
    if not 8 <= options.length <= 64:
        raise ValueError("length must be 8-64")
    chars = alphabet(options)
    while True:
        pw = "".join(rng.choice(chars) for _ in range(options.length))
        if options.digits and not any(c.isdigit() for c in pw):
            continue
        if options.symbols and not any(c in SYMBOLS for c in pw):
            continue
        return pw


def strength(password):
    if len(password) < 10:
        return "weak"
    classes = sum([
        any(c.islower() for c in password),
        any(c.isupper() for c in password),
        any(c.isdigit() for c in password),
        any(not c.isalnum() for c in password),
    ])
    if len(password) >= 14 and classes >= 3:
        return "strong"
    return "medium"


pw = generate(PasswordOptions(length=12), random.Random(7))
print(len(pw), strength(pw), strength("abc"), strength("Tr0ub4dor&3xyz"))
