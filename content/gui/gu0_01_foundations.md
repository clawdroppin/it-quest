@@lesson gu0-foundations GUI Foundations: Windows, Widgets and the Event Loop
@@concepts functions, objects, input
@@lang python
@@text
A **GUI** (graphical user interface) is everything a person sees and clicks: windows, buttons, text boxes, lists, menus. Every GUI toolkit, in every language, is built on the same few ideas. Learn them once and WinForms, WPF, Qt, tkinter, Electron and the web all start to look alike.

## The four big ideas

| Idea | What it means |
|---|---|
| **Widgets** (controls, elements) | the building blocks: `Button`, `Label`, `Entry`, `Listbox`, `Checkbox`, `Slider` |
| **Layout** | how widgets are placed: rows, columns, grids, stretching when the window resizes |
| **Events** | things that happen: a click, a key press, the mouse moving, a timer firing, the window closing |
| **The event loop** | the program waits for an event, runs the code you attached to it (a **callback** or **handler**), then waits again |

A console program runs top to bottom and ends. A GUI program sets up the window and then hands control to the **event loop**, which runs until the window closes. Your code only runs *when something happens*.

```text
start → build the window → mainloop() ┐
                                      │  wait for an event
                                      │  click on "Save"  → run on_save()
                                      │  key Ctrl+Q       → run on_quit()
                                      └─ repeat until the window closes
```

## Your first window with tkinter

**tkinter** comes with Python on Windows and macOS (on Linux, `sudo apt install python3-tk`). It's plain-looking, but perfect for learning, and good enough for small internal tools.

```python,norun
import tkinter as tk
from tkinter import ttk          # ttk = the "themed" widgets, which look more native

root = tk.Tk()
root.title("Hello GUI")
root.geometry("320x160")

count = tk.IntVar(value=0)       # a variable the widgets can watch

def on_click():
    count.set(count.get() + 1)

ttk.Label(root, text="You clicked:").pack(pady=(16, 4))
ttk.Label(root, textvariable=count, font=("Segoe UI", 20, "bold")).pack()
ttk.Button(root, text="Click me", command=on_click).pack(pady=12)

root.mainloop()                  # the event loop: nothing after this runs until the window closes
```

Notice `command=on_click`, not `command=on_click()`. You hand the toolkit the function itself, and *it* calls it later. Calling it yourself would run it once, right now, and give the button its return value (`None`). This is the most common beginner GUI bug in every language.

## Layout: never place things with pixel coordinates

Absolute positions (`x=40, y=120`) break as soon as the window is resized, the font is bigger, or the text is translated. Use a **layout manager**:

- `pack`: stack widgets top-to-bottom or left-to-right. Simple.
- `grid`: rows and columns, like a table. The right choice for forms.
- `place`: absolute positions. Avoid.

```python,norun
form = ttk.Frame(root, padding=16)
form.grid(sticky="nsew")
root.columnconfigure(0, weight=1)        # the frame grows with the window
form.columnconfigure(1, weight=1)        # column 1 (the inputs) takes the extra width

ttk.Label(form, text="Hostname").grid(row=0, column=0, sticky="w", pady=4)
ttk.Entry(form).grid(row=0, column=1, sticky="ew", padx=(8, 0))
ttk.Label(form, text="Owner").grid(row=1, column=0, sticky="w", pady=4)
ttk.Entry(form).grid(row=1, column=1, sticky="ew", padx=(8, 0))
ttk.Button(form, text="Save").grid(row=2, column=1, sticky="e", pady=(12, 0))
```

`sticky` says which edges a widget sticks to (`n`, `s`, `e`, `w`); `weight` says which columns and rows get extra space. Every toolkit has the same ideas: CSS flexbox and grid, WPF `Grid` and `StackPanel`, Qt `QGridLayout` and `QVBoxLayout`.

## Events beyond clicks

```python,norun
entry.bind("<Return>", lambda e: on_save())          # Enter key in the entry
root.bind("<Control-q>", lambda e: root.destroy())    # a keyboard shortcut for the whole window
canvas.bind("<Motion>", lambda e: show_xy(e.x, e.y))  # the mouse moved
root.protocol("WM_DELETE_WINDOW", on_close)           # the user clicked the X: ask "save first?"
root.after(1000, tick)                                # a timer: run tick() in 1000 ms
```

## Never block the event loop

While your callback runs, the loop can't redraw the window or react to clicks, so the window **freezes** ("Not Responding"). Anything slow (downloading, reading a big file, pinging 200 hosts) must run in a **background thread**, and only the GUI thread may touch widgets:

```python,norun
import threading, queue

results = queue.Queue()

def scan_in_background():
    for host in hosts:
        results.put((host, ping(host)))         # never touch widgets from this thread

def poll_results():
    while not results.empty():
        host, ok = results.get()
        listbox.insert("end", f"{host}: {'up' if ok else 'down'}")
    root.after(100, poll_results)               # check again in 100 ms

threading.Thread(target=scan_in_background, daemon=True).start()
poll_results()
```

The same rule exists everywhere: WinForms has `Invoke`, WPF has `Dispatcher` and `async`/`await`, Qt has signals across threads, JavaScript has `async` functions and Web Workers.

## Keep the logic out of the window

The most important design habit in GUI programming: **the window should be thin.** Put the rules (validation, calculations, saving) in plain classes and functions that know nothing about buttons. The GUI only reads input, calls the logic, and shows the result.

```text
┌───────────── View (the window) ──────────────┐
│ reads widgets, calls the model, shows results │
└───────────────────────┬───────────────────────┘
                        │ plain method calls
┌───────────── Model (the logic) ───────────────┐
│ no tkinter imports: testable, reusable         │
└────────────────────────────────────────────────┘
```

Why: you can **test** the model with normal unit tests (you can't easily test clicks), you can swap tkinter for Qt or a web page without rewriting the rules, and bugs are easier to find. Bigger apps formalise this as **MVC** (model-view-controller), **MVVM** (model-view-viewmodel, used by WPF) or a **store** with actions (React, Redux). This track's exercises practise exactly this: you'll write the models, and the windows are shown as examples to run on your own machine.

## A peek at Qt (PySide6)

When tkinter isn't enough, **Qt** is the professional choice: native-looking, fast, with tables, charts, docking panels and a visual designer. `pip install pyside6`:

```python,norun
from PySide6.QtWidgets import QApplication, QWidget, QPushButton, QLabel, QVBoxLayout

app = QApplication([])
win = QWidget(windowTitle="Hello Qt")
label = QLabel("Clicked 0 times")
button = QPushButton("Click me")
layout = QVBoxLayout(win)
layout.addWidget(label)
layout.addWidget(button)

clicks = 0
def on_click():
    global clicks
    clicks += 1
    label.setText(f"Clicked {clicks} times")

button.clicked.connect(on_click)     # Qt calls events "signals" and handlers "slots"
win.show()
app.exec()                           # the event loop
```

Qt can be styled with **QSS**, a CSS-like language: `button.setStyleSheet("QPushButton { border-radius: 8px; padding: 8px 16px; background: #6d28d9; color: white; }")`.

## Choosing a toolkit

| Toolkit | Language | Good for |
|---|---|---|
| tkinter | Python | quick internal tools, learning |
| PySide6 / PyQt | Python | serious cross-platform desktop apps |
| WinForms | C#, PowerShell | quick Windows admin tools |
| WPF / WinUI 3 | C# + XAML | modern, beautiful Windows apps |
| .NET MAUI / Avalonia | C# + XAML | cross-platform C# apps |
| Qt | C++ | fast, native cross-platform apps |
| Electron | HTML/CSS/JS | apps like VS Code, Slack, Discord |
| Tauri | HTML/CSS/JS + Rust | small, fast desktop apps (this app is one!) |
| Fyne | Go | simple cross-platform Go apps |

@@quiz
What does `mainloop()` (or `app.exec()`, or `Application.Run`) do?
- Draws the window once and exits
* Runs the event loop: waits for events and calls your handlers until the window closes
- Runs every callback once
- Starts a background thread for each button
? After it starts, your code only runs in response to events.

@@quiz
Why is `Button(text="Save", command=save())` a bug?
- `command` must be a string
* It calls `save` immediately and gives the button its return value (`None`) instead of the function
- Buttons can't run functions
- `save` must be a lambda
? Pass the function itself: `command=save`. Use `lambda: save(x)` when you need to pass arguments.

@@quiz
A button handler downloads a 2 GB file. What happens to the window?
- Nothing, GUIs are multithreaded
* It freezes ("Not Responding") until the download finishes, because the event loop can't run
- The download is automatically moved to the background
- The window closes
? Slow work goes in a background thread or async task; only the GUI thread updates widgets.

@@quiz
Why keep validation and calculations out of the window code?
- Windows can't run if-statements
- It makes the program faster
* Plain logic can be unit-tested and reused with another toolkit, and the window stays simple
- tkinter doesn't allow functions
? This is the core of MVC, MVVM and every other GUI pattern.

@@quiz
Which tkinter layout manager is best for a form with labels and inputs?
- `place`
* `grid`
- `pack` with absolute coordinates
- None, use pixel positions
? `grid` puts labels in one column and inputs in the next, and `sticky` plus `weight` let inputs stretch.

@@flashcards
widget / control :: a building block of a GUI: button, label, text box, list
event :: something that happens: click, key press, timer, window closing
callback / handler :: the function the toolkit calls when an event happens
event loop :: waits for events and runs their handlers until the window closes
`command=save` vs `command=save()` :: pass the function vs call it right now (bug)
layout manager :: places widgets in rows, columns or grids so they adapt to resizing
`sticky` / `weight` (tkinter grid) :: which edges a widget sticks to / which column gets extra space
`root.after(ms, fn)` :: run a function later on the GUI thread (timers, animations, polling)
freezing GUI :: slow work in a handler blocks the event loop
background thread + queue :: run slow work without freezing; only the GUI thread touches widgets
MVC / MVVM :: patterns that separate the logic (model) from the window (view)
Qt signal / slot :: Qt's names for an event and its handler
QSS :: Qt's CSS-like styling language
ttk :: tkinter's themed, more native-looking widgets

@@exercise A model for a counter window
Write the **model** for the click-counter window above, with no GUI code at all. `Counter(step=1, maximum=10)` starts at 0. `click()` adds `step` but never goes above `maximum`. `reset()` sets it back to 0. `label()` returns the text the window shows: `"0 clicks"`, `"1 click"` (singular!), `"7 clicks"`, and `"10 clicks (max)"` when it's at the maximum. `can_click()` returns whether the button should be enabled.
@@starter
class Counter:
    def __init__(self, step=1, maximum=10):
        pass

    def click(self):
        pass

    def reset(self):
        pass

    def can_click(self):
        pass

    def label(self):
        pass


c = Counter()
c.click()
print(c.label())
@@tests
c = Counter()
check(c.label(), "0 clicks", "starts at zero")
c.click()
check(c.label(), "1 click", "one click is singular")
c.click()
check(c.label(), "2 clicks", "two clicks")
check(c.can_click(), True, "can click below the max")
d = Counter(step=4, maximum=10)
d.click(); d.click(); d.click()
check(d.label(), "10 clicks (max)", "stops at the maximum")
check(d.can_click(), False, "button disabled at the max")
d.reset()
check(d.label(), "0 clicks", "reset")
check(d.can_click(), True, "enabled again after reset")
@@hint
Store `self.value`, `self.step` and `self.maximum`. In `click`, use `self.value = min(self.value + self.step, self.maximum)`. In `label`, check the maximum first, then `1`.
@@solution
class Counter:
    def __init__(self, step=1, maximum=10):
        self.step = step
        self.maximum = maximum
        self.value = 0

    def click(self):
        self.value = min(self.value + self.step, self.maximum)

    def reset(self):
        self.value = 0

    def can_click(self):
        return self.value < self.maximum

    def label(self):
        if self.value >= self.maximum:
            return f"{self.value} clicks (max)"
        if self.value == 1:
            return "1 click"
        return f"{self.value} clicks"


c = Counter()
c.click()
print(c.label())

@@exercise Validate a form before saving
A "new device" window has three inputs. Write `validate(form)`, where `form` is a dict with `"hostname"`, `"ip"` and `"owner"` (strings, maybe with extra spaces). Return a dict of **field name → error message** for every problem, so the window can show a red message under each bad field. Return `{}` when everything is fine.

- `hostname`: required (`"Required"`); letters, digits and `-` only (`"Letters, digits and - only"`)
- `ip`: four numbers 0 to 255 separated by dots (`"Not a valid IPv4 address"`)
- `owner`: optional, but at most 30 characters (`"30 characters at most"`)

Spaces at the start and end don't count.
@@starter
def validate(form):
    errors = {}
    return errors


print(validate({"hostname": "tk-lt-01", "ip": "10.0.0.5", "owner": ""}))
print(validate({"hostname": " ", "ip": "10.0.0.300", "owner": "x" * 31}))
@@tests
check(validate({"hostname": "tk-lt-01", "ip": "10.0.0.5", "owner": ""}), {}, "a good form")
check(validate({"hostname": "  pc-7 ", "ip": " 192.168.1.1 ", "owner": "Aiko"}), {}, "spaces are trimmed")
check(validate({"hostname": " ", "ip": "10.0.0.5", "owner": ""}), {"hostname": "Required"}, "missing hostname")
check(validate({"hostname": "bad host!", "ip": "10.0.0.5", "owner": ""}), {"hostname": "Letters, digits and - only"}, "bad characters")
check(validate({"hostname": "a", "ip": "10.0.0.300", "owner": ""}), {"ip": "Not a valid IPv4 address"}, "number above 255")
check(validate({"hostname": "a", "ip": "10.0.5", "owner": ""}), {"ip": "Not a valid IPv4 address"}, "only three parts")
check(validate({"hostname": "a", "ip": "10.0.x.5", "owner": ""}), {"ip": "Not a valid IPv4 address"}, "not a number")
check(validate({"hostname": "", "ip": "", "owner": "x" * 31}), {"hostname": "Required", "ip": "Not a valid IPv4 address", "owner": "30 characters at most"}, "every problem at once")
@@hint
Trim with `.strip()`. For the hostname, `all(ch.isalnum() or ch == "-" for ch in name)`. For the IP, `parts = ip.split(".")`, then check `len(parts) == 4` and every part `p.isdigit() and int(p) <= 255`.
@@solution
def validate(form):
    errors = {}
    hostname = form["hostname"].strip()
    ip = form["ip"].strip()
    owner = form["owner"].strip()

    if not hostname:
        errors["hostname"] = "Required"
    elif not all(ch.isalnum() or ch == "-" for ch in hostname):
        errors["hostname"] = "Letters, digits and - only"

    parts = ip.split(".")
    if len(parts) != 4 or not all(p.isdigit() and int(p) <= 255 for p in parts):
        errors["ip"] = "Not a valid IPv4 address"

    if len(owner) > 30:
        errors["owner"] = "30 characters at most"
    return errors


print(validate({"hostname": "tk-lt-01", "ip": "10.0.0.5", "owner": ""}))
print(validate({"hostname": " ", "ip": "10.0.0.300", "owner": "x" * 31}))

@@text
**Try it on your own machine:** wire `Counter` into the tkinter window at the top of this lesson. Disable the button with `button.state(["disabled"])` when `can_click()` is false, and show `label()` in the label. Your model doesn't change at all.
