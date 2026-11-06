@@lesson s12-tk-events Events, Variables and Input Validation
@@concepts objects, functions
@@text
GUIs are driven by events. This lesson covers how to react to them (`command=`, `bind`), how Tk **variables** keep widgets and data in sync, and how to validate input as the user types.

## command= vs bind

- `command=` is the simple path for a widget's main action: a button click, a checkbox toggle, a menu item.
- `widget.bind(sequence, handler)` reacts to **any** event, and the handler receives an **event object**:

```python,norun
import tkinter as tk
from tkinter import ttk

root = tk.Tk()
entry = ttk.Entry(root)
entry.pack(padx=10, pady=10)

def on_key(event):
    print("key:", event.keysym, "char:", repr(event.char))

entry.bind("<KeyRelease>", on_key)
entry.bind("<Return>", lambda e: print("submit", entry.get()))
root.bind("<Control-s>", lambda e: print("save"))           # window-wide shortcut
root.bind("<Escape>", lambda e: root.destroy())
entry.bind("<FocusOut>", lambda e: print("left the field"))
root.mainloop()
```

Common sequences: `<Button-1>` (left click), `<Double-Button-1>`, `<Button-3>` (right click, context menus), `<Motion>`, `<Enter>`/`<Leave>` (hover), `<KeyPress-a>`, `<Control-Shift-S>`, `<Configure>` (size changed), `<<TreeviewSelect>>` and `<<ComboboxSelected>>` (virtual events from widgets).

Return the string `"break"` from a handler to stop further processing of that event (for example to block a key).

## Tk variables

`tk.StringVar`, `IntVar`, `DoubleVar` and `BooleanVar` hold a value that widgets display and edit. Change the variable and every widget using it updates; change the widget and the variable updates:

```python,norun
import tkinter as tk
from tkinter import ttk

root = tk.Tk()
name = tk.StringVar(value="maya")
admin = tk.BooleanVar()

ttk.Entry(root, textvariable=name).pack()
ttk.Checkbutton(root, text="Administrator", variable=admin).pack()
greeting = ttk.Label(root)
greeting.pack()

def refresh(*_):
    greeting.config(text=f"{name.get()} ({'admin' if admin.get() else 'user'})")

name.trace_add("write", refresh)       # called on every change
admin.trace_add("write", refresh)
refresh()
root.mainloop()
```

`trace_add("write", callback)` turns variables into a tiny **reactive** system, the same idea as signals in modern web frameworks.

## Validating as the user types

Entries can reject keystrokes with `validatecommand`. Tk calls your function with the proposed new text (`%P`) and accepts the change only if it returns True:

```python,norun
def is_port_text(proposed):
    return proposed == "" or (proposed.isdigit() and int(proposed) <= 65535)

vcmd = (root.register(is_port_text), "%P")
port = ttk.Entry(root, validate="key", validatecommand=vcmd)
```

Keystroke validation keeps junk out; **submit-time** validation (on Save) checks the complete form and shows errors next to the fields. Use both: keystroke rules must allow incomplete-but-valid-so-far text (an empty field, a half-typed date).

## Showing errors well

- Put the message next to the field (a red label below it) rather than only in a pop-up.
- Disable the Save button until the form is valid: `button.state(["disabled"])` / `button.state(["!disabled"])`.
- Move focus to the first invalid field: `entry.focus_set()`.
- Never clear what the user typed because one character was wrong.

## Keyboard accessibility

- Every action reachable by keyboard: Tab order follows creation order; `Return` submits; `Escape` cancels; shortcuts like `Ctrl+S` shown in menus.
- `underline=0` on buttons and menu items marks Alt-key mnemonics.
- Don't rely on colour alone to show errors.

@@quiz
What's the difference between `command=` and `bind`?
* `command=` handles a widget's main action; `bind` handles any event sequence and passes an event object
- They are identical
- bind only works on buttons
- command= handles keyboard events only
? Use bind for keys, mouse movement, focus and virtual events.

@@quiz
What does `name.trace_add("write", refresh)` do?
- Logs to a file
* Calls `refresh` every time the variable's value changes
- Makes the entry read-only
- Writes the variable to disk
? Tk variables can notify you of changes.

@@quiz
Why must keystroke validation accept an empty string?
- Tk requires it
* The user needs to be able to delete everything and retype; partial input must be allowed while typing
- Empty strings are valid ports
- It doesn't matter
? Check completeness on submit.

@@drill
Which Tk variable class holds text bound to an Entry?
= StringVar
= tk.StringVar
? IntVar and BooleanVar hold other types.

@@flashcards
`bind(sequence, handler)` :: react to any event; handler gets an event object
`<Control-s>` :: event sequence for Ctrl+S
`<<TreeviewSelect>>` :: virtual event sent by a widget
`"break"` :: returned from a handler to stop further event processing
StringVar / IntVar / BooleanVar :: Tk variables shared between widgets and code
`trace_add("write", cb)` :: callback on variable change
validatecommand :: function that accepts or rejects proposed entry text
`%P` :: the proposed new text passed to a validator

@@exercise Keystroke validators
Write validators for `validatecommand`, each taking the **proposed** text and returning True to accept it. They must accept every prefix of a valid value (so the user can type it) and reject everything else:

- `port_ok(p)`: empty, or digits without a leading zero forming a number 1–65535
- `ipv4_ok(p)`: up to four dot-separated parts, each empty (still typing) or a number 0–255 without leading zeros (`"0"` itself is fine); at most 3 dots
- `date_ok(p)`: a prefix of `YYYY-MM-DD`: digits where digits go, `-` at positions 4 and 7, max 10 characters; month (when two digits are present) 01–12, day 01–31
@@starter
def port_ok(p):
    pass


def ipv4_ok(p):
    pass


def date_ok(p):
    pass


print([port_ok(x) for x in ["", "8", "80", "08", "65536", "8a"]])
print([ipv4_ok(x) for x in ["", "10.", "10.0.0.255", "10.0.0.256", "1.2.3.4.5", "01.2"]])
print([date_ok(x) for x in ["2026-1", "2026-11-03", "2026-13", "2026/11", "2026-11-032"]])
@@expected
[True, True, True, False, False, False]
[True, True, True, False, False, False]
[True, True, False, False, False]
@@tests
check(port_ok("65535"), True, "max port")
check(port_ok("0"), False, "zero port")
check(ipv4_ok("0.0.0.0"), True, "zeros")
check(ipv4_ok("..."), True, "dots while typing")
check(date_ok(""), True, "empty date")
check(date_ok("2026-11-3"), True, "day half typed")
check(date_ok("2026-02-00"), False, "day 00")
check(date_ok("2026-11-32"), False, "day 32")
@@hint
For IPv4: `parts = p.split(".")`; `len(parts) <= 4`; each part `== ""` or (`part.isdigit()` and `int(part) <= 255` and not (`len(part) > 1 and part[0] == "0"`)). For dates compare against a pattern `"dddd-dd-dd"` character by character, then check the month and day only when complete (`len(p) >= 7` / `len(p) == 10`).
@@solution
def port_ok(p):
    if p == "":
        return True
    return p.isdigit() and p[0] != "0" and int(p) <= 65535


def ipv4_ok(p):
    parts = p.split(".")
    if len(parts) > 4:
        return False
    for part in parts:
        if part == "":
            continue
        if not part.isdigit() or int(part) > 255 or (len(part) > 1 and part[0] == "0"):
            return False
    return True


def date_ok(p):
    pattern = "dddd-dd-dd"
    if len(p) > len(pattern):
        return False
    for c, kind in zip(p, pattern):
        if kind == "d" and not c.isdigit():
            return False
        if kind == "-" and c != "-":
            return False
    if len(p) >= 7 and not 1 <= int(p[5:7]) <= 12:
        return False
    if len(p) == 10 and not 1 <= int(p[8:10]) <= 31:
        return False
    return True


print([port_ok(x) for x in ["", "8", "80", "08", "65536", "8a"]])
print([ipv4_ok(x) for x in ["", "10.", "10.0.0.255", "10.0.0.256", "1.2.3.4.5", "01.2"]])
print([date_ok(x) for x in ["2026-1", "2026-11-03", "2026-13", "2026/11", "2026-11-032"]])

@@exercise A keyboard shortcut map
Apps register shortcuts like `"<Control-s>"`. Write a `Shortcuts` class:

- `add(sequence, action_name)`: parse sequences of the form `<Mod-Mod-key>` where modifiers are `Control`, `Shift`, `Alt` (any order) and the key is the last part. Store them normalised so `<Shift-Control-S>` and `<Control-Shift-s>` are the same shortcut (modifiers sorted, key lower-cased). Adding a shortcut that's already taken raises `ValueError("<Control-Shift-s> already used by save_as")` using the normalised form.
- `lookup(modifiers, key)`: `modifiers` is a set like `{"Control"}`; returns the action name or `None`.
- `menu_label(action)`: the accelerator text for menus, like `"Ctrl+Shift+S"` (modifiers in the order Ctrl, Alt, Shift; key upper-cased if it's a single character or a function key like `F5`, otherwise as stored).
@@starter
class Shortcuts:
    pass


s = Shortcuts()
s.add("<Control-s>", "save")
s.add("<Shift-Control-S>", "save_as")
s.add("<F5>", "refresh")
print(s.lookup({"Control"}, "S"), s.lookup({"Control", "Shift"}, "s"), s.lookup(set(), "F5"))
print(s.menu_label("save_as"), s.menu_label("refresh"))
@@expected
save save_as refresh
Ctrl+Shift+S F5
@@tests
s = Shortcuts()
s.add("<Alt-Control-x>", "cut_all")
check(s.menu_label("cut_all"), "Ctrl+Alt+X", "modifier order")
try:
    s.add("<Control-Alt-X>", "other")
    check("added", "ValueError", "duplicate")
except ValueError as e:
    check(str(e), "<Alt-Control-x> already used by cut_all", "duplicate message")
check(s.lookup({"Alt"}, "x"), None, "missing")
check(s.menu_label("nope"), None, "unknown action")
@@hint
Parse with `parts = sequence.strip("<>").split("-")`; the key is `parts[-1].lower()` and the modifiers `tuple(sorted(parts[:-1]))`. Store `self.by_combo[(mods, key)] = action` and `self.by_action[action] = (mods, key)`. The normalised text is `"<" + "-".join([*mods, key]) + ">"`. In `menu_label`, map names with `{"Control": "Ctrl", "Alt": "Alt", "Shift": "Shift"}` in a fixed order.
@@solution
class Shortcuts:
    ORDER = [("Control", "Ctrl"), ("Alt", "Alt"), ("Shift", "Shift")]

    def __init__(self):
        self.by_combo = {}
        self.by_action = {}

    @staticmethod
    def _parse(sequence):
        parts = sequence.strip("<>").split("-")
        return tuple(sorted(parts[:-1])), parts[-1].lower()

    def add(self, sequence, action):
        combo = self._parse(sequence)
        if combo in self.by_combo:
            text = "<" + "-".join([*combo[0], combo[1]]) + ">"
            raise ValueError(f"{text} already used by {self.by_combo[combo]}")
        self.by_combo[combo] = action
        self.by_action[action] = combo

    def lookup(self, modifiers, key):
        return self.by_combo.get((tuple(sorted(modifiers)), key.lower()))

    def menu_label(self, action):
        combo = self.by_action.get(action)
        if combo is None:
            return None
        mods, key = combo
        names = [label for name, label in self.ORDER if name in mods]
        is_function_key = key[0] == "f" and key[1:].isdigit()
        key_text = key.upper() if len(key) == 1 or is_function_key else key
        return "+".join([*names, key_text])


s = Shortcuts()
s.add("<Control-s>", "save")
s.add("<Shift-Control-S>", "save_as")
s.add("<F5>", "refresh")
print(s.lookup({"Control"}, "S"), s.lookup({"Control", "Shift"}, "s"), s.lookup(set(), "F5"))
print(s.menu_label("save_as"), s.menu_label("refresh"))
