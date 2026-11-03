@@lesson s12-tk-menus Menus, Dialogs, Files and Unsaved Changes
@@concepts files, objects
@@text
A document-style app (a notes tool, a config editor, an inventory file editor) needs the features every user expects: a menu bar with shortcuts, Open/Save dialogs, a recent-files list, and a warning before closing with unsaved changes.

## A menu bar

```python,norun
import tkinter as tk
from tkinter import filedialog, messagebox

root = tk.Tk()
menubar = tk.Menu(root)

file_menu = tk.Menu(menubar, tearoff=False)
file_menu.add_command(label="New", accelerator="Ctrl+N", command=lambda: print("new"))
file_menu.add_command(label="Open...", accelerator="Ctrl+O", command=lambda: open_file())
file_menu.add_command(label="Save", accelerator="Ctrl+S", command=lambda: print("save"))
file_menu.add_separator()
recent_menu = tk.Menu(file_menu, tearoff=False)
file_menu.add_cascade(label="Open Recent", menu=recent_menu)
file_menu.add_separator()
file_menu.add_command(label="Exit", command=root.destroy)
menubar.add_cascade(label="File", menu=file_menu, underline=0)

view_menu = tk.Menu(menubar, tearoff=False)
dark = tk.BooleanVar()
view_menu.add_checkbutton(label="Dark mode", variable=dark)
menubar.add_cascade(label="View", menu=view_menu, underline=0)

root.config(menu=menubar)
root.bind_all("<Control-o>", lambda e: open_file())     # accelerator text doesn't bind keys by itself!

def open_file():
    path = filedialog.askopenfilename(
        title="Open inventory",
        filetypes=[("Inventory files", "*.json"), ("All files", "*.*")],
    )
    if path:                                             # empty string if cancelled
        print("opening", path)

root.mainloop()
```

- `accelerator=` only **displays** the shortcut; bind the key separately.
- `entryconfig(index, state="disabled")` greys out items (Undo when there's nothing to undo).
- Context menus: create a `tk.Menu` and show it with `menu.tk_popup(event.x_root, event.y_root)` on `<Button-3>`.
- On macOS the menu bar sits at the top of the screen and the app menu is special; Tk handles most of it.

## Standard dialogs

| Module | Functions |
|---|---|
| `tkinter.filedialog` | `askopenfilename`, `askopenfilenames`, `asksaveasfilename`, `askdirectory` |
| `tkinter.messagebox` | `showinfo`, `showwarning`, `showerror`, `askyesno`, `askyesnocancel`, `askokcancel` |
| `tkinter.simpledialog` | `askstring`, `askinteger`, `askfloat` |
| `tkinter.colorchooser` | `askcolor` |

They're **modal**: the function returns only when the user answers, and the main window is blocked meanwhile. For your own dialogs, create a `tk.Toplevel`, call `transient(root)`, `grab_set()` and `wait_window()`.

## Unsaved changes: the dirty flag

```text
          edit                      save ok
clean ───────────▶ dirty  ──────────────────▶ clean
                    │ close/open/new
                    ▼
        "Save changes to inventory.json?"  [Save] [Don't Save] [Cancel]
```

- Track a **dirty** flag: set on every edit, cleared after a successful save or load.
- Show it in the title: `"inventory.json • Asset Editor"` or a leading `*`.
- Intercept closing the window: `root.protocol("WM_DELETE_WINDOW", on_close)`, and ask with `messagebox.askyesnocancel` (Yes = save, No = discard, Cancel = stay).
- **Save safely**: write to a temporary file in the same folder, then `os.replace(tmp, path)` (atomic), so a crash during save never destroys the old file.

## Recent files

Keep the last 5–10 opened paths in a small JSON file in the user's config folder (`platformdirs.user_config_dir`). Most recent first, no duplicates (case-insensitive on Windows), drop files that no longer exist when building the menu. Exercise 2 builds this list.

## Where to put user files

- Settings: `%APPDATA%\YourApp` (Windows), `~/.config/yourapp` (Linux), `~/Library/Application Support/YourApp` (macOS): `platformdirs` gives the right one.
- Documents: let the user choose with a dialog; default to their Documents folder (`Path.home() / "Documents"`).
- Never write next to the program in `C:\Program Files`; it's read-only for normal users.

@@quiz
What does `accelerator="Ctrl+S"` on a menu item do?
- Binds Ctrl+S to the command
* Only displays the shortcut text; you must bind the key yourself
- Disables the item
- Saves the file
? Use `root.bind_all("<Control-s>", ...)`.

@@quiz
What does `filedialog.askopenfilename` return when the user cancels?
- None
* An empty string
- Raises an exception
- The last file
? Always check `if path:`.

@@quiz
How do you run code when the user clicks the window's close button?
- bind("<Close>")
* `root.protocol("WM_DELETE_WINDOW", handler)`
- Override mainloop
- atexit
? The handler decides whether to call `root.destroy()`.

@@quiz
How should a file be saved so a crash can't corrupt it?
- Open with "w" and write slowly
* Write to a temporary file in the same folder, then `os.replace` it over the original
- Save twice
- Use pickle
? `os.replace` is atomic on the same file system.

@@flashcards
`tk.Menu` :: menu bars, submenus and context menus
`add_cascade` :: attach a submenu
accelerator :: shortcut text shown in a menu (doesn't bind keys)
Modal dialog :: blocks the main window until answered
`askyesnocancel` :: returns True, False or None
Dirty flag :: marks unsaved changes
`WM_DELETE_WINDOW` :: protocol for the window close button
Atomic save :: write a temp file then `os.replace`
platformdirs :: finds per-user config and data folders

@@exercise Document state and closing
Write `Document` that tracks unsaved changes:

- `Document(path=None)`; `edit()` marks it dirty; `saved(path)` stores the path and clears dirty; `loaded(path)` the same
- `title(app="Asset Editor")`: `"<name> • <app>"` where name is the file's name (no folders) or `"Untitled"`, prefixed with `"*"` when dirty: `"*inventory.json • Asset Editor"`
- `on_close(ask, save)`: decides what happens when the user closes the window. If not dirty → return `"close"`. Otherwise call `ask(message)` with `"Save changes to <name>?"`, which returns True (save), False (discard) or None (cancel). True → call `save()` (returns True if saved, False if the user cancelled the Save dialog or it failed) and return `"close"` only if it saved, else `"stay"`; False → `"close"`; None → `"stay"`.
@@starter
from pathlib import PurePath


class Document:
    pass


d = Document()
print(d.title())
d.edit()
print(d.title(), d.on_close(ask=lambda m: None, save=lambda: True))
d.saved("C:/data/inventory.json")
print(d.title(), d.on_close(ask=lambda m: 1 / 0, save=lambda: True))
@@expected
Untitled • Asset Editor
*Untitled • Asset Editor stay
inventory.json • Asset Editor close
@@tests
d = Document("/srv/config.toml")
d.edit()
asked = []
def ask(msg):
    asked.append(msg)
    return True
check(d.on_close(ask, save=lambda: False), "stay", "save cancelled")
check(asked, ["Save changes to config.toml?"], "message")
check(d.on_close(lambda m: True, save=lambda: True), "close", "saved then close")
check(d.on_close(lambda m: False, save=lambda: 1 / 0), "close", "discard")
d.loaded("/srv/other.toml")
check(d.title("X"), "other.toml • X", "loaded clears dirty")
@@hint
Use `PurePath(self.path).name` for the name. In `on_close`, check `self.dirty` first; then `answer = ask(...)`; `if answer is None: return "stay"`; `if answer is False: return "close"`; else `return "close" if save() else "stay"`.
@@solution
from pathlib import PurePath


class Document:
    def __init__(self, path=None):
        self.path = path
        self.dirty = False

    def edit(self):
        self.dirty = True

    def saved(self, path):
        self.path = path
        self.dirty = False

    def loaded(self, path):
        self.saved(path)

    def name(self):
        return PurePath(self.path).name if self.path else "Untitled"

    def title(self, app="Asset Editor"):
        return f"{'*' if self.dirty else ''}{self.name()} • {app}"

    def on_close(self, ask, save):
        if not self.dirty:
            return "close"
        answer = ask(f"Save changes to {self.name()}?")
        if answer is None:
            return "stay"
        if answer is False:
            return "close"
        return "close" if save() else "stay"


d = Document()
print(d.title())
d.edit()
print(d.title(), d.on_close(ask=lambda m: None, save=lambda: True))
d.saved("C:/data/inventory.json")
print(d.title(), d.on_close(ask=lambda m: 1 / 0, save=lambda: True))

@@exercise Recent files and safe saving
- `add_recent(recent, path, limit=5)`: return a **new** list with `path` first, any earlier entry for the same file removed (compare case-insensitively and treating `\` and `/` as the same), and at most `limit` entries.
- `menu_items(recent, exists)`: the entries for the Open Recent menu: only paths where `exists(path)` is True, labelled `"1 inventory.json"`, `"2 notes.md"`... (number and file name). Return a list of `(label, path)`.
- `save_atomic(path, text)`: write `text` to `path + ".tmp"` (UTF-8), then `os.replace` it onto `path`. The test checks that no `.tmp` file is left.
@@starter
import os
from pathlib import PureWindowsPath


def add_recent(recent, path, limit=5):
    pass


def menu_items(recent, exists):
    pass


def save_atomic(path, text):
    pass


r = []
for p in ["C:/a/inv.json", "C:/b/notes.md", "c:\\A\\INV.json"]:
    r = add_recent(r, p)
print(r)
print(menu_items(r, exists=lambda p: "notes" not in p))
@@expected
['c:\\A\\INV.json', 'C:/b/notes.md']
[('1 INV.json', 'c:\\A\\INV.json')]
@@tests
r = []
for i in range(8):
    r = add_recent(r, f"C:/f{i}.txt", limit=3)
check(r, ["C:/f7.txt", "C:/f6.txt", "C:/f5.txt"], "limit")
original = ["x"]
add_recent(original, "y")
check(original, ["x"], "input not modified")
import tempfile, pathlib
with tempfile.TemporaryDirectory() as tmp:
    target = os.path.join(tmp, "inv.json")
    save_atomic(target, "first")
    save_atomic(target, "second ✓")
    check(pathlib.Path(target).read_text(encoding="utf-8"), "second ✓", "content")
    check(sorted(os.listdir(tmp)), ["inv.json"], "no temp file left")
@@hint
Normalise for comparison with `p.replace("\\", "/").lower()`. `add_recent`: `[path] + [p for p in recent if norm(p) != norm(path)]` then slice. File names: `PureWindowsPath(p).name` handles both slash styles. Save: `with open(tmp, "w", encoding="utf-8") as f: f.write(text)` then `os.replace(tmp, path)`.
@@solution
import os
from pathlib import PureWindowsPath


def _norm(p):
    return p.replace("\\", "/").lower()


def add_recent(recent, path, limit=5):
    rest = [p for p in recent if _norm(p) != _norm(path)]
    return ([path] + rest)[:limit]


def menu_items(recent, exists):
    existing = [p for p in recent if exists(p)]
    return [(f"{i} {PureWindowsPath(p).name}", p) for i, p in enumerate(existing, 1)]


def save_atomic(path, text):
    tmp = path + ".tmp"
    with open(tmp, "w", encoding="utf-8") as f:
        f.write(text)
    os.replace(tmp, path)


r = []
for p in ["C:/a/inv.json", "C:/b/notes.md", "c:\\A\\INV.json"]:
    r = add_recent(r, p)
print(r)
print(menu_items(r, exists=lambda p: "notes" not in p))
