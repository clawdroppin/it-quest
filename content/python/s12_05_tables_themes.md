@@lesson s12-tk-tables Tables with Treeview, Styles and Dark Mode
@@concepts objects, lists
@@text
Most IT tools show lists of things: assets, users, tickets, log lines. `ttk.Treeview` is Tkinter's table (and tree) widget. This lesson builds sortable, filterable tables and then makes the app look modern with **ttk styles** and a dark theme.

## A table

```python,norun
import tkinter as tk
from tkinter import ttk

ASSETS = [("LT-0042", "laptop", "maya", 1200), ("MN-0007", "monitor", "li", 240), ("LT-0050", "laptop", "sam", 1350)]

root = tk.Tk()
cols = ("tag", "kind", "owner", "price")
tree = ttk.Treeview(root, columns=cols, show="headings", selectmode="browse")
for c in cols:
    tree.heading(c, text=c.title(), command=lambda c=c: sort_by(c))
    tree.column(c, width=110, anchor="e" if c == "price" else "w")
tree.pack(fill="both", expand=True)

def fill(rows):
    tree.delete(*tree.get_children())
    for row in rows:
        tree.insert("", "end", iid=row[0], values=row)

state = {"col": "tag", "desc": False}
def sort_by(col):
    state["desc"] = not state["desc"] if state["col"] == col else False
    state["col"] = col
    i = cols.index(col)
    fill(sorted(ASSETS, key=lambda r: r[i], reverse=state["desc"]))

tree.bind("<<TreeviewSelect>>", lambda e: print("selected", tree.selection()))
fill(ASSETS)
root.mainloop()
```

- `show="headings"` hides the tree column for a plain table; omit it to show expandable tree rows (`insert(parent_iid, ...)`).
- `iid` gives each row a stable id (use your record's key) so selection survives refreshes.
- `tree.selection()` returns selected iids; `tree.item(iid, "values")` the values.
- **Tags** style rows: `tree.tag_configure("overdue", foreground="#c62828")` then `insert(..., tags=("overdue",))`. Striped rows: alternate `"odd"`/`"even"` tags.
- Large data: insert only what's visible or page it; Treeview handles thousands of rows but not millions.

Like the previous lesson, sorting and filtering belong in a model; the view only displays the rows it's given (exercise 1).

## Styles and themes

ttk widgets are drawn by a **theme** (`clam`, `alt`, `default`, `classic`, plus `vista`/`winnative` on Windows and `aqua` on macOS) and customised with **styles**:

```python,norun
style = ttk.Style()
print(style.theme_names())
style.theme_use("clam")                       # the most customisable built-in theme

style.configure(".", font=("Segoe UI", 10))   # "." = every widget
style.configure("TButton", padding=(12, 6))
style.configure("Accent.TButton", background="#2f6fde", foreground="white")
style.map("Accent.TButton", background=[("active", "#2559b5"), ("disabled", "#9db8ea")])
style.configure("Treeview", rowheight=26)

ttk.Button(root, text="Save", style="Accent.TButton")
```

- Style names are `Name.TWidgetClass`: your `Accent.TButton` inherits everything from `TButton`.
- `configure` sets default options; `map` sets options per **state** (`active`, `pressed`, `disabled`, `focus`, `selected`).
- Classic `tk` widgets (`Text`, `Listbox`, `Canvas`) don't use styles; set their `bg`/`fg` directly.

## Light and dark themes

Define your colours once as a **palette** and apply it, so switching themes is one function call:

```python,norun
LIGHT = {"bg": "#f6f7f9", "fg": "#1d2129", "panel": "#ffffff", "accent": "#2f6fde", "muted": "#6b7280"}
DARK  = {"bg": "#16181d", "fg": "#e6e8eb", "panel": "#1f2229", "accent": "#5b8def", "muted": "#9aa1ab"}

def apply(root, p):
    s = ttk.Style(root)
    s.theme_use("clam")
    s.configure(".", background=p["bg"], foreground=p["fg"], fieldbackground=p["panel"])
    s.configure("Treeview", background=p["panel"], fieldbackground=p["panel"], foreground=p["fg"])
    s.map("Treeview", background=[("selected", p["accent"])])
    root.configure(bg=p["bg"])
```

- Check **contrast**: text must be readable on its background. WCAG asks for a contrast ratio of at least **4.5:1** for normal text (exercise 2).
- On Windows, the registry value `AppsUseLightTheme` tells you the user's preference; the `darkdetect` package wraps it for every OS.
- The GUI Design track covers colour, spacing and typography in depth; **sv-ttk** (Sun Valley) and **CustomTkinter** give Tkinter a Windows 11 look out of the box.

@@quiz
Why give Treeview rows an explicit `iid` from your data?
* Rows keep a stable identity, so selection and updates can find them after a refresh
- It's required
- It sorts the table
- It sets the colour
? Use your record's key, like the asset tag.

@@quiz
What does the style name `Accent.TButton` mean?
- A new widget class
* A custom button style that inherits from the standard TButton style
- A theme
- A font
? Use it with `ttk.Button(..., style="Accent.TButton")`.

@@quiz
What is `style.map` for?
- Mapping keys to actions
* Setting options per widget state, like the background when hovered or disabled
- Drawing maps
- Changing the theme
? `configure` sets defaults; `map` sets per-state values.

@@quiz
What minimum contrast ratio does WCAG require for normal text?
- 2:1
- 3:1
* 4.5:1
- 10:1
? Large text needs 3:1.

@@flashcards
Treeview :: ttk table/tree widget
`show="headings"` :: Treeview as a plain table without the tree column
iid :: a row's identifier in a Treeview
Row tags :: style specific rows (colours, fonts)
ttk theme :: the drawing style for all ttk widgets (clam, vista, aqua...)
`style.configure` :: default style options
`style.map` :: per-state style options
Palette :: a named set of colours applied as a theme
Contrast ratio :: readability measure; WCAG AA wants 4.5:1 for text

@@exercise A table model
Write `TableModel(rows, columns)` that the Treeview view displays:

- `rows` are dicts; `columns` is a list of keys
- `set_filter(text)`: keep rows where any column's value (as a lower-cased string) contains the lower-cased text
- `click_header(col)`: clicking a new column sorts ascending by it; clicking the same column again toggles descending/ascending. Ties keep the previous order (stable sort applied to the original row order).
- `visible()`: list of tuples of column values for the filtered, sorted rows
- `header_text(col)`: the column title, with `" ▲"` or `" ▼"` appended for the current sort column (title is the key with the first letter upper-cased)
@@starter
class TableModel:
    pass


m = TableModel(
    [
        {"tag": "LT-0042", "owner": "maya", "price": 1200},
        {"tag": "MN-0007", "owner": "li", "price": 240},
        {"tag": "LT-0050", "owner": "sam", "price": 1350},
    ],
    ["tag", "owner", "price"],
)
m.click_header("price")
m.click_header("price")
m.set_filter("lt")
print(m.visible(), m.header_text("price"), m.header_text("tag"))
@@expected
[('LT-0050', 'sam', 1350), ('LT-0042', 'maya', 1200)] Price ▼ Tag
@@tests
rows = [{"a": 2, "b": "x"}, {"a": 1, "b": "y"}, {"a": 2, "b": "z"}]
m = TableModel(rows, ["a", "b"])
check(m.visible(), [(2, "x"), (1, "y"), (2, "z")], "unsorted original order")
m.click_header("a")
check(m.visible(), [(1, "y"), (2, "x"), (2, "z")], "ascending, stable")
check(m.header_text("a"), "A ▲", "arrow up")
m.click_header("b")
check(m.header_text("b"), "B ▲", "new column starts ascending")
m.set_filter("Z")
check(m.visible(), [(2, "z")], "filter case-insensitive")
m.set_filter("")
check(len(m.visible()), 3, "clear filter")
@@hint
Keep `self.sort_col = None` and `self.desc = False`. In `click_header`: `self.desc = not self.desc if col == self.sort_col else False`. In `visible`, filter first, then `sorted(..., key=lambda r: r[self.sort_col], reverse=self.desc)` if a sort column is set (Python's sort stays stable with `reverse=True`).
@@solution
class TableModel:
    def __init__(self, rows, columns):
        self.rows = rows
        self.columns = columns
        self.filter = ""
        self.sort_col = None
        self.desc = False

    def set_filter(self, text):
        self.filter = text.lower()

    def click_header(self, col):
        self.desc = not self.desc if col == self.sort_col else False
        self.sort_col = col

    def visible(self):
        rows = [r for r in self.rows if any(self.filter in str(r[c]).lower() for c in self.columns)]
        if self.sort_col is not None:
            rows = sorted(rows, key=lambda r: r[self.sort_col], reverse=self.desc)
        return [tuple(r[c] for c in self.columns) for r in rows]

    def header_text(self, col):
        title = col[:1].upper() + col[1:]
        if col == self.sort_col:
            title += " ▼" if self.desc else " ▲"
        return title


m = TableModel(
    [
        {"tag": "LT-0042", "owner": "maya", "price": 1200},
        {"tag": "MN-0007", "owner": "li", "price": 240},
        {"tag": "LT-0050", "owner": "sam", "price": 1350},
    ],
    ["tag", "owner", "price"],
)
m.click_header("price")
m.click_header("price")
m.set_filter("lt")
print(m.visible(), m.header_text("price"), m.header_text("tag"))

@@exercise Check a palette's contrast
Write the WCAG contrast check used to validate light and dark themes:

- `luminance(hex_color)`: for `"#rrggbb"`, convert each channel to 0–1 (`c / 255`), linearise it (`c / 12.92` if `c <= 0.04045`, else `((c + 0.055) / 1.055) ** 2.4`), and return `0.2126 * R + 0.7152 * G + 0.0722 * B`
- `contrast(a, b)`: `(lighter + 0.05) / (darker + 0.05)`, rounded to 2 decimals
- `check_palette(palette)`: for the pairs `("fg", "bg")`, `("fg", "panel")`, `("muted", "panel")`, `("accent", "panel")`, return a list of failures as strings `"muted on panel: 3.12"` for ratios below 4.5 (in that pair order)
@@starter
def luminance(hex_color):
    pass


def contrast(a, b):
    pass


def check_palette(palette):
    pass


DARK = {"bg": "#16181d", "fg": "#e6e8eb", "panel": "#1f2229", "accent": "#5b8def", "muted": "#6b7280"}
print(contrast("#000000", "#ffffff"), contrast("#777777", "#ffffff"))
print(check_palette(DARK))
@@expected
21.0 4.48
['muted on panel: 3.29']
@@tests
check(contrast("#ffffff", "#ffffff"), 1.0, "same colour")
check(contrast("#ffffff", "#000000"), contrast("#000000", "#ffffff"), "order doesn't matter")
check(round(luminance("#ff0000"), 4), 0.2126, "pure red")
LIGHT = {"bg": "#f6f7f9", "fg": "#1d2129", "panel": "#ffffff", "accent": "#2f6fde", "muted": "#6b7280"}
check(check_palette(LIGHT), [], "light palette passes")
@@hint
Parse channels with `int(hex_color[i:i + 2], 16)` for `i` in 1, 3, 5. `contrast`: `hi, lo = max(la, lb), min(la, lb)`. Format failures with `f"{x} on {y}: {ratio}"`.
@@solution
def luminance(hex_color):
    def channel(i):
        c = int(hex_color[i:i + 2], 16) / 255
        return c / 12.92 if c <= 0.04045 else ((c + 0.055) / 1.055) ** 2.4

    r, g, b = channel(1), channel(3), channel(5)
    return 0.2126 * r + 0.7152 * g + 0.0722 * b


def contrast(a, b):
    la, lb = luminance(a), luminance(b)
    hi, lo = max(la, lb), min(la, lb)
    return round((hi + 0.05) / (lo + 0.05), 2)


def check_palette(palette):
    failures = []
    for fg, bg in [("fg", "bg"), ("fg", "panel"), ("muted", "panel"), ("accent", "panel")]:
        ratio = contrast(palette[fg], palette[bg])
        if ratio < 4.5:
            failures.append(f"{fg} on {bg}: {ratio}")
    return failures


DARK = {"bg": "#16181d", "fg": "#e6e8eb", "panel": "#1f2229", "accent": "#5b8def", "muted": "#6b7280"}
print(contrast("#000000", "#ffffff"), contrast("#777777", "#ffffff"))
print(check_palette(DARK))
