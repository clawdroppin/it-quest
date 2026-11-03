@@lesson s12-tk-project 🏆 Module Project: An Asset Manager App, Packaged
@@concepts objects, files, json, testing
@@text
The capstone puts the whole module together: an **Asset Manager** desktop app for a small IT team. It lists hardware assets in a sortable, filterable table, edits them in a validated form, saves to a JSON file safely, supports undo, and ships as a single `.exe` a colleague can double-click.

## Architecture

```text
asset_manager/
├── model.py        AssetStore: data, validation, load/save, search   (no tkinter!)
├── presenter.py    AssetPresenter: reacts to view events, calls the model
├── view.py         MainWindow: ttk widgets, menus, Treeview, dialogs
├── app.py          wires them together, starts mainloop
└── tests/
    ├── test_model.py
    └── test_presenter.py   (with a FakeView)
```

Everything you can test without a window lives in `model.py` and `presenter.py`. The exercises below build the model; the view is the Tkinter code from earlier lessons (Treeview table, form with validators, menu with recent files, dirty flag, background export).

## The view, sketched

```python,norun
import tkinter as tk
from tkinter import ttk, messagebox


class MainWindow(ttk.Frame):
    def __init__(self, root, presenter):
        super().__init__(root, padding=8)
        self.presenter = presenter
        self.grid(sticky="nsew")
        root.columnconfigure(0, weight=1)
        root.rowconfigure(0, weight=1)

        self.search = tk.StringVar()
        ttk.Entry(self, textvariable=self.search).grid(row=0, column=0, sticky="ew")
        self.search.trace_add("write", lambda *_: presenter.on_search(self.search.get()))

        self.table = ttk.Treeview(self, columns=("tag", "type", "owner"), show="headings")
        for col in ("tag", "type", "owner"):
            self.table.heading(col, text=col.title(), command=lambda c=col: presenter.on_sort(c))
        self.table.grid(row=1, column=0, sticky="nsew", pady=(6, 0))
        self.columnconfigure(0, weight=1)
        self.rowconfigure(1, weight=1)

        root.protocol("WM_DELETE_WINDOW", presenter.on_close)

    # methods the presenter calls
    def show_rows(self, rows):
        self.table.delete(*self.table.get_children())
        for r in rows:
            self.table.insert("", "end", iid=r["tag"], values=(r["tag"], r["type"], r["owner"]))

    def show_error(self, text):
        messagebox.showerror("Asset Manager", text, parent=self)
```

## Packaging with PyInstaller

Your colleagues don't have Python installed. **PyInstaller** bundles the interpreter, your code and its dependencies:

```bash,norun
python -m venv .venv
.venv\Scripts\activate                     # source .venv/bin/activate on Linux/macOS
pip install pyinstaller
pyinstaller --onefile --windowed --name AssetManager --icon app.ico app.py
# result: dist/AssetManager.exe
```

| Option | Meaning |
|---|---|
| `--onefile` | one self-extracting executable (slower start) instead of a folder (`--onedir`, faster start) |
| `--windowed` / `-w` | no console window behind the GUI |
| `--icon app.ico` | the executable's icon |
| `--add-data "assets;assets"` | bundle extra files (`:` instead of `;` on Linux/macOS) |

Things that catch people out:

- **Bundled files**: at runtime they live under `sys._MEIPASS` in one-file mode. Use a helper: `base = Path(getattr(sys, "_MEIPASS", Path(__file__).parent))`.
- **User data** goes to `platformdirs.user_data_dir("AssetManager")`, never next to the `.exe`.
- PyInstaller builds for the OS it runs on; build the Windows `.exe` on Windows (or in CI on a Windows runner).
- Unsigned executables trigger SmartScreen / antivirus warnings. Companies sign them with a code-signing certificate (`signtool sign`), or distribute through an internal software portal (Intune, SCCM).
- Alternatives: **Nuitka** (compiles to C, faster), **Briefcase** (native installers, including macOS `.app` and Linux packages), or an MSI installer built with WiX for proper Start-menu entries and uninstall.

## Checklist before you ship

1. Model and presenter tests pass (`pytest`).
2. Every action works from the keyboard; Tab order is sensible.
3. Closing with unsaved changes asks first; saves are atomic.
4. Long operations run in a thread with progress and cancel.
5. Errors are shown in plain language, and logged to a file in the user's log folder.
6. The window remembers its size and the last opened file (`geometry()` saved in settings).
7. Tested at 150% display scaling and with a dark theme.
8. A version number in Help → About, matching the release.

@@quiz
Why keep `model.py` free of tkinter imports?
* So the core logic can be unit-tested without a window and reused by other front ends (CLI, web)
- tkinter is slow to import
- PyInstaller requires it
- Models can't use classes
? This is the MVP separation from earlier in the module.

@@quiz
What does `pyinstaller --windowed` do?
- Makes the window resizable
* Builds the app without a console window behind the GUI
- Builds for Windows only
- Creates an installer
? Use it for GUI apps; leave it off for command-line tools.

@@quiz
Where do files added with `--add-data` live at runtime in one-file mode?
- Next to the .exe
- In the user's Documents folder
* In a temporary extraction folder given by `sys._MEIPASS`
- In the registry
? Resolve paths relative to `sys._MEIPASS` when it exists.

@@flashcards
PyInstaller :: bundles Python, your code and dependencies into an executable
`--onefile` :: single executable that extracts itself at start
`--onedir` :: a folder with the executable and libraries; starts faster
`--windowed` :: no console window
`sys._MEIPASS` :: folder of bundled files at runtime (one-file mode)
Code signing :: certifies the publisher; avoids SmartScreen warnings
Nuitka :: compiles Python to C for a faster executable
Briefcase :: packages Python apps as native installers

@@exercise The asset store
Write `AssetStore`, the model behind the app. Assets are dicts with `tag`, `type`, `owner` and `purchased` (`"YYYY-MM-DD"`).

- `validate(asset)` (a static method): return a dict of field → error message, empty if valid:
  - `tag` must match `IT-` followed by exactly 5 digits → `"tag must look like IT-12345"`
  - `type` must be one of `laptop`, `desktop`, `monitor`, `phone` → `"unknown type"`
  - `owner` must not be blank → `"owner is required"`
  - `purchased` must be a real date (`date.fromisoformat`) not in the future compared to `today` (a parameter, default `date.today()`) → `"invalid date"`
- `add(asset)`: raise `ValueError` with the errors joined as `"field: message; field: message"` (fields in alphabetical order) if invalid; raise `ValueError("duplicate tag IT-00001")` if the tag exists; otherwise store a copy
- `search(text)`: assets whose tag, type or owner contains `text` (case-insensitive), sorted by tag
- `to_json()` / `from_json(text)` (a class method): save and load, with the assets sorted by tag and `indent=2`
@@starter
import json
import re
from datetime import date


class AssetStore:
    pass


store = AssetStore()
store.add({"tag": "IT-00042", "type": "laptop", "owner": "Maya", "purchased": "2025-03-01"})
store.add({"tag": "IT-00007", "type": "monitor", "owner": "Leo", "purchased": "2024-11-20"})
print([a["tag"] for a in store.search("")], [a["tag"] for a in store.search("MAY")])
try:
    store.add({"tag": "IT-7", "type": "toaster", "owner": " ", "purchased": "2025-02-30"})
except ValueError as e:
    print(e)
copy = AssetStore.from_json(store.to_json())
print([a["owner"] for a in copy.search("it-")])
@@expected
['IT-00007', 'IT-00042'] ['IT-00042']
owner: owner is required; purchased: invalid date; tag: tag must look like IT-12345; type: unknown type
['Leo', 'Maya']
@@tests
check(AssetStore.validate({"tag": "IT-12345", "type": "phone", "owner": "x", "purchased": "2030-01-01"}, today=date(2026, 1, 1)), {"purchased": "invalid date"}, "future date")
check(AssetStore.validate({"tag": "IT-123456", "type": "phone", "owner": "x", "purchased": "2020-01-01"}), {"tag": "tag must look like IT-12345"}, "six digits")
s = AssetStore()
a = {"tag": "IT-00001", "type": "phone", "owner": "Ana", "purchased": "2024-01-01"}
s.add(a)
a["owner"] = "changed"
check(s.search("ana")[0]["owner"], "Ana", "stores a copy")
try:
    s.add({"tag": "IT-00001", "type": "phone", "owner": "Bo", "purchased": "2024-01-01"})
    check("added", "ValueError", "duplicate")
except ValueError as e:
    check(str(e), "duplicate tag IT-00001", "duplicate message")
check(json.loads(s.to_json()), [{"tag": "IT-00001", "type": "phone", "owner": "Ana", "purchased": "2024-01-01"}], "json shape")
@@hint
Use `re.fullmatch(r"IT-\d{5}", tag)`. For the date: `try: d = date.fromisoformat(value) except (ValueError, TypeError): ...` and then compare with `today`. Keep assets in a dict keyed by tag; `search` filters `self.assets.values()` and sorts with `key=lambda a: a["tag"]`. Join errors with `"; ".join(f"{k}: {v}" for k, v in sorted(errors.items()))`.
@@solution
import json
import re
from datetime import date


class AssetStore:
    TYPES = {"laptop", "desktop", "monitor", "phone"}

    def __init__(self):
        self.assets = {}

    @staticmethod
    def validate(asset, today=None):
        today = today or date.today()
        errors = {}
        if not re.fullmatch(r"IT-\d{5}", str(asset.get("tag", ""))):
            errors["tag"] = "tag must look like IT-12345"
        if asset.get("type") not in AssetStore.TYPES:
            errors["type"] = "unknown type"
        if not str(asset.get("owner", "")).strip():
            errors["owner"] = "owner is required"
        try:
            bought = date.fromisoformat(asset.get("purchased", ""))
            if bought > today:
                errors["purchased"] = "invalid date"
        except (ValueError, TypeError):
            errors["purchased"] = "invalid date"
        return errors

    def add(self, asset):
        errors = self.validate(asset)
        if errors:
            raise ValueError("; ".join(f"{k}: {v}" for k, v in sorted(errors.items())))
        if asset["tag"] in self.assets:
            raise ValueError(f"duplicate tag {asset['tag']}")
        self.assets[asset["tag"]] = dict(asset)

    def search(self, text):
        needle = text.lower()
        found = [
            a for a in self.assets.values()
            if any(needle in str(a[f]).lower() for f in ("tag", "type", "owner"))
        ]
        return sorted(found, key=lambda a: a["tag"])

    def to_json(self):
        return json.dumps(sorted(self.assets.values(), key=lambda a: a["tag"]), indent=2)

    @classmethod
    def from_json(cls, text):
        store = cls()
        for asset in json.loads(text):
            store.add(asset)
        return store


store = AssetStore()
store.add({"tag": "IT-00042", "type": "laptop", "owner": "Maya", "purchased": "2025-03-01"})
store.add({"tag": "IT-00007", "type": "monitor", "owner": "Leo", "purchased": "2024-11-20"})
print([a["tag"] for a in store.search("")], [a["tag"] for a in store.search("MAY")])
try:
    store.add({"tag": "IT-7", "type": "toaster", "owner": " ", "purchased": "2025-02-30"})
except ValueError as e:
    print(e)
copy = AssetStore.from_json(store.to_json())
print([a["owner"] for a in copy.search("it-")])

@@exercise The presenter
Write `AssetPresenter(store, view)` connecting the store to a view. The view has `show_rows(rows)`, `show_error(text)`, `set_title(text)`. The presenter keeps a search text, a sort column (default `"tag"`) and direction, and a dirty flag.

- `refresh()`: `view.show_rows(...)` with `store.search(self.query)` sorted by the sort column (ties broken by tag), reversed when descending; then `view.set_title(...)` with `"Asset Manager"`, prefixed by `"*"` when dirty
- `on_search(text)`: store the text and refresh
- `on_sort(column)`: same column again toggles descending; a new column sorts ascending; refresh
- `on_add(asset)`: try `store.add`; on `ValueError` call `view.show_error(str(e))` and return False; on success mark dirty, refresh, return True
- `on_saved()`: clear dirty and refresh

Use the `AssetStore` from the previous exercise (a minimal version is included in the starter).
@@starter
import re


class AssetStore:                       # minimal version so this exercise stands alone
    def __init__(self):
        self.assets = {}

    def add(self, asset):
        if not re.fullmatch(r"IT-\d{5}", asset.get("tag", "")):
            raise ValueError("tag: tag must look like IT-12345")
        if asset["tag"] in self.assets:
            raise ValueError(f"duplicate tag {asset['tag']}")
        self.assets[asset["tag"]] = dict(asset)

    def search(self, text):
        t = text.lower()
        found = [a for a in self.assets.values() if any(t in str(a[f]).lower() for f in ("tag", "type", "owner"))]
        return sorted(found, key=lambda a: a["tag"])


class FakeView:
    def __init__(self):
        self.rows, self.errors, self.title = [], [], None

    def show_rows(self, rows):
        self.rows = [r["tag"] for r in rows]

    def show_error(self, text):
        self.errors.append(text)

    def set_title(self, text):
        self.title = text


class AssetPresenter:
    pass


view = FakeView()
p = AssetPresenter(AssetStore(), view)
p.on_add({"tag": "IT-00003", "type": "phone", "owner": "Zoe"})
p.on_add({"tag": "IT-00001", "type": "laptop", "owner": "Ana"})
p.on_add({"tag": "IT-00002", "type": "laptop", "owner": "Bo"})
print(view.rows, view.title)
p.on_sort("owner"); print(view.rows)
p.on_sort("owner"); print(view.rows)
print(p.on_add({"tag": "bad", "type": "laptop", "owner": "X"}), view.errors)
p.on_search("lap"); p.on_saved(); print(view.rows, view.title)
@@expected
['IT-00001', 'IT-00002', 'IT-00003'] *Asset Manager
['IT-00001', 'IT-00002', 'IT-00003']
['IT-00003', 'IT-00002', 'IT-00001']
False ['tag: tag must look like IT-12345']
['IT-00002', 'IT-00001'] Asset Manager
@@tests
view = FakeView()
p = AssetPresenter(AssetStore(), view)
p.refresh()
check((view.rows, view.title), ([], "Asset Manager"), "empty start")
p.on_add({"tag": "IT-00009", "type": "monitor", "owner": "Same"})
p.on_add({"tag": "IT-00004", "type": "monitor", "owner": "Same"})
p.on_sort("type")
check(view.rows, ["IT-00004", "IT-00009"], "ties broken by tag")
check(p.on_add({"tag": "IT-00004", "type": "x", "owner": "y"}), False, "duplicate rejected")
check(view.errors, ["duplicate tag IT-00004"], "duplicate error shown")
p.on_sort("tag")
check(view.rows, ["IT-00004", "IT-00009"], "new column sorts ascending")
@@hint
`store.search` already returns rows sorted by tag. Sort them again with `rows.sort(key=lambda a: str(a[self.sort_col]).lower(), reverse=self.descending)`: Python's sort is stable (even with `reverse=True`), so rows with equal values keep their tag order.
@@solution
import re


class AssetStore:                       # minimal version so this exercise stands alone
    def __init__(self):
        self.assets = {}

    def add(self, asset):
        if not re.fullmatch(r"IT-\d{5}", asset.get("tag", "")):
            raise ValueError("tag: tag must look like IT-12345")
        if asset["tag"] in self.assets:
            raise ValueError(f"duplicate tag {asset['tag']}")
        self.assets[asset["tag"]] = dict(asset)

    def search(self, text):
        t = text.lower()
        found = [a for a in self.assets.values() if any(t in str(a[f]).lower() for f in ("tag", "type", "owner"))]
        return sorted(found, key=lambda a: a["tag"])


class FakeView:
    def __init__(self):
        self.rows, self.errors, self.title = [], [], None

    def show_rows(self, rows):
        self.rows = [r["tag"] for r in rows]

    def show_error(self, text):
        self.errors.append(text)

    def set_title(self, text):
        self.title = text


class AssetPresenter:
    def __init__(self, store, view):
        self.store = store
        self.view = view
        self.query = ""
        self.sort_col = "tag"
        self.descending = False
        self.dirty = False

    def refresh(self):
        rows = self.store.search(self.query)          # already sorted by tag
        rows.sort(key=lambda a: str(a[self.sort_col]).lower(), reverse=self.descending)
        self.view.show_rows(rows)
        self.view.set_title(("*" if self.dirty else "") + "Asset Manager")

    def on_search(self, text):
        self.query = text
        self.refresh()

    def on_sort(self, column):
        if column == self.sort_col:
            self.descending = not self.descending
        else:
            self.sort_col = column
            self.descending = False
        self.refresh()

    def on_add(self, asset):
        try:
            self.store.add(asset)
        except ValueError as e:
            self.view.show_error(str(e))
            return False
        self.dirty = True
        self.refresh()
        return True

    def on_saved(self):
        self.dirty = False
        self.refresh()


view = FakeView()
p = AssetPresenter(AssetStore(), view)
p.on_add({"tag": "IT-00003", "type": "phone", "owner": "Zoe"})
p.on_add({"tag": "IT-00001", "type": "laptop", "owner": "Ana"})
p.on_add({"tag": "IT-00002", "type": "laptop", "owner": "Bo"})
print(view.rows, view.title)
p.on_sort("owner"); print(view.rows)
p.on_sort("owner"); print(view.rows)
print(p.on_add({"tag": "bad", "type": "laptop", "owner": "X"}), view.errors)
p.on_search("lap"); p.on_saved(); print(view.rows, view.title)
