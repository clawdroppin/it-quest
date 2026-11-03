@@lesson s12-tk-layout Layout: pack, grid, Frames and Resizing
@@concepts objects
@@text
A window that looks fine at one size and falls apart when maximised is the most common sign of a beginner GUI. Tkinter has three **geometry managers**; learning `grid` and `pack` well, and nesting **frames**, gives you layouts that resize properly.

## pack: stacking

`pack` places widgets one after another along a side:

```python,norun
import tkinter as tk
from tkinter import ttk

root = tk.Tk()
toolbar = ttk.Frame(root, padding=4)
toolbar.pack(side="top", fill="x")
ttk.Button(toolbar, text="Refresh").pack(side="left")
ttk.Button(toolbar, text="Export").pack(side="left", padx=4)

status = ttk.Label(root, text="Ready", anchor="w", padding=(6, 2))
status.pack(side="bottom", fill="x")

text = tk.Text(root)
text.pack(side="top", fill="both", expand=True)     # takes all remaining space
root.mainloop()
```

- `side`: `top` (default), `bottom`, `left`, `right`.
- `fill`: stretch to fill the allotted space in `x`, `y` or `both`.
- `expand=True`: claim extra space when the window grows.
- Order matters: pack the toolbar and status bar **before** the expanding text area.

Great for toolbars, status bars and simple stacks.

## grid: rows and columns

`grid` places widgets in a table, perfect for forms:

```python,norun
import tkinter as tk
from tkinter import ttk

root = tk.Tk()
form = ttk.Frame(root, padding=12)
form.grid(sticky="nsew")
root.columnconfigure(0, weight=1)
root.rowconfigure(0, weight=1)

fields = ["Asset tag", "Owner", "Location"]
for row, name in enumerate(fields):
    ttk.Label(form, text=name).grid(row=row, column=0, sticky="w", padx=(0, 8), pady=3)
    ttk.Entry(form).grid(row=row, column=1, sticky="ew", pady=3)

notes = tk.Text(form, height=5)
notes.grid(row=len(fields), column=0, columnspan=2, sticky="nsew", pady=(6, 0))

buttons = ttk.Frame(form)
buttons.grid(row=len(fields) + 1, column=0, columnspan=2, sticky="e", pady=(8, 0))
ttk.Button(buttons, text="Cancel").pack(side="left")
ttk.Button(buttons, text="Save").pack(side="left", padx=(6, 0))

form.columnconfigure(1, weight=1)            # the entry column stretches
form.rowconfigure(len(fields), weight=1)     # the notes row stretches
root.mainloop()
```

- `row`, `column`, `rowspan`, `columnspan` position the widget.
- `sticky` ("n", "s", "e", "w" combined) says which cell edges the widget sticks to; `"ew"` stretches horizontally, `"nsew"` fills the cell.
- **`columnconfigure(i, weight=1)` / `rowconfigure`** decide which columns and rows get extra space when the window grows. Without weights, nothing stretches. This is the line beginners forget.
- `padx`/`pady` add space outside the widget; a frame's `padding` adds space inside it.

**Don't mix `pack` and `grid` in the same container** (Tk will hang trying to satisfy both). Different frames may use different managers; that's how you combine them (above, the buttons frame uses pack inside a gridded form).

## place: absolute positions

`place(x=10, y=20)` or relative `place(relx=0.5, rely=0.5, anchor="center")`. Rarely right for whole layouts (it doesn't adapt to fonts or languages), but handy for overlays like a centred "Loading..." label.

## Frames: layout as a tree

Real windows are nested frames, each with a simple layout:

```text
root (grid)
├── toolbar frame (pack, left to right)
├── main PanedWindow (resizable split)
│   ├── left: Treeview of assets
│   └── right: details form (grid)
└── status bar
```

`ttk.PanedWindow` gives a draggable divider; `ttk.Notebook` gives tabs; `ttk.LabelFrame` draws a titled border around a group.

## Sizing and DPI

- `root.minsize(600, 400)` prevents a window too small to use.
- Let widgets size themselves from their content and fonts; avoid fixed pixel sizes so the layout survives translations and larger fonts.
- On high-DPI Windows screens, call `ctypes.windll.shcore.SetProcessDpiAwareness(1)` before creating the window so text isn't blurry.
- `root.update_idletasks()` then `root.winfo_width()` gives actual sizes, e.g. to centre a window on screen.

## Scrolling

`Text`, `Listbox`, `Treeview` and `Canvas` scroll; connect a `ttk.Scrollbar`:

```python,norun
tree = ttk.Treeview(frame, columns=("owner", "location"), show="headings")
bar = ttk.Scrollbar(frame, orient="vertical", command=tree.yview)
tree.configure(yscrollcommand=bar.set)
tree.grid(row=0, column=0, sticky="nsew")
bar.grid(row=0, column=1, sticky="ns")
```

@@quiz
A gridded Entry doesn't grow when the window is resized. What's usually missing?
* `columnconfigure(col, weight=1)` on its container and `sticky="ew"` on the widget
- `expand=True` in grid
- A Scrollbar
- mainloop
? Weights decide who gets extra space; sticky makes the widget fill its cell.

@@quiz
Why should the status bar be packed before the expanding text area?
- Order doesn't matter
* pack allocates space in order; an earlier expanding widget can take the space the status bar needs
- Status bars must be first
- To make it bold
? Pack fixed-size edge widgets first.

@@quiz
Can you use pack and grid in the same container?
- Yes, freely
* No; it can hang Tk. Use different frames for different managers
- Only on Windows
- Only with place
? Nest frames instead.

@@flashcards
pack :: stacks widgets along a side; good for toolbars and status bars
grid :: rows and columns; good for forms
place :: absolute or relative coordinates; good for overlays
sticky :: which cell edges a gridded widget sticks to ("nsew" fills)
weight :: how much extra space a grid row/column receives
fill / expand :: pack options to stretch and claim extra space
Frame :: container widget used to nest layouts
PanedWindow :: container with draggable dividers

@@exercise Plan a form layout
Before building a form, compute where everything goes. Write `plan_form(fields, columns)`:

- `fields` is a list of `(label, kind)` where kind is `"entry"` or `"text"`.
- The form has `columns` field-groups side by side; each group uses two grid columns (label, widget). Entry fields fill groups left to right, row by row: with 2 groups, field 0 is at row 0 group 0, field 1 at row 0 group 1, field 2 at row 1 group 0...
- `"text"` fields always start on a **new row** at group 0 and span the full width (`columnspan = columns * 2`, label above it in its own row is not needed: put the label in column 0 and the widget in column 1 with `columnspan = columns * 2 - 1`), and the next entry field starts on the row after it.
- Return a list of dicts: `{"label": name, "row": r, "label_col": c, "widget_col": c + 1, "span": s}` in field order.
@@starter
def plan_form(fields, columns):
    pass


FIELDS = [("Tag", "entry"), ("Owner", "entry"), ("Model", "entry"), ("Notes", "text"), ("Location", "entry")]
for p in plan_form(FIELDS, 2):
    print(p)
@@expected
{'label': 'Tag', 'row': 0, 'label_col': 0, 'widget_col': 1, 'span': 1}
{'label': 'Owner', 'row': 0, 'label_col': 2, 'widget_col': 3, 'span': 1}
{'label': 'Model', 'row': 1, 'label_col': 0, 'widget_col': 1, 'span': 1}
{'label': 'Notes', 'row': 2, 'label_col': 0, 'widget_col': 1, 'span': 3}
{'label': 'Location', 'row': 3, 'label_col': 0, 'widget_col': 1, 'span': 1}
@@tests
check(plan_form([], 3), [], "empty")
check([(p["row"], p["label_col"]) for p in plan_form([("a", "entry")] * 4, 1)], [(0, 0), (1, 0), (2, 0), (3, 0)], "one column")
check(plan_form([("n", "text"), ("a", "entry")], 3), [
    {"label": "n", "row": 0, "label_col": 0, "widget_col": 1, "span": 5},
    {"label": "a", "row": 1, "label_col": 0, "widget_col": 1, "span": 1},
], "text first")
@@hint
Track `row` and `group` (0..columns-1). For an entry: place at `(row, group*2)`, then `group += 1`, and when `group == columns` reset to 0 and `row += 1`. For a text: if `group != 0`, move to the next row first; place it; then `row += 1` and `group = 0`.
@@solution
def plan_form(fields, columns):
    plan, row, group = [], 0, 0
    for name, kind in fields:
        if kind == "text":
            if group != 0:
                row += 1
                group = 0
            plan.append({"label": name, "row": row, "label_col": 0, "widget_col": 1, "span": columns * 2 - 1})
            row += 1
            continue
        plan.append({"label": name, "row": row, "label_col": group * 2, "widget_col": group * 2 + 1, "span": 1})
        group += 1
        if group == columns:
            group = 0
            row += 1
    return plan


FIELDS = [("Tag", "entry"), ("Owner", "entry"), ("Model", "entry"), ("Notes", "text"), ("Location", "entry")]
for p in plan_form(FIELDS, 2):
    print(p)

@@exercise Share out extra space
When a window grows, Tk shares the extra pixels between columns according to their **weights**. Write `distribute(sizes, weights, total)`:

- `sizes` are the columns' natural widths, `weights` their weights, `total` the window width.
- If `total <= sum(sizes)`, return the natural sizes unchanged.
- Otherwise give the extra space to columns in proportion to their weight (columns with weight 0 get nothing), using integer pixels: each weighted column first gets `extra * weight // total_weight`, then the leftover pixels go one each to weighted columns from the **last** to the first (Tk's behaviour isn't specified; this is our rule).
- If all weights are 0, the extra space is unused (return natural sizes).
@@starter
def distribute(sizes, weights, total):
    pass


print(distribute([80, 200, 60], [0, 1, 0], 500))
print(distribute([100, 100, 100], [1, 2, 1], 403))
@@expected
[80, 360, 60]
[125, 152, 126]
@@tests
check(distribute([100, 100], [1, 1], 150), [100, 100], "smaller than natural")
check(distribute([100, 100], [0, 0], 500), [100, 100], "no weights")
check(sum(distribute([10, 20, 30, 40], [1, 1, 1, 0], 1000)), 1000, "fills total")
check(distribute([0, 0, 0], [1, 1, 1], 10), [3, 3, 4], "leftover from the last")
@@hint
`extra = total - sum(sizes)`; `tw = sum(weights)`. `out = [s + extra * w // tw for s, w in zip(sizes, weights)]`. `left = total - sum(out)`; then walk indices in reverse, adding 1 to weighted columns while `left > 0` (loop around if needed).
@@solution
def distribute(sizes, weights, total):
    natural = sum(sizes)
    tw = sum(weights)
    if total <= natural or tw == 0:
        return list(sizes)
    extra = total - natural
    out = [s + extra * w // tw for s, w in zip(sizes, weights)]
    left = total - sum(out)
    weighted = [i for i, w in enumerate(weights) if w > 0]
    i = 0
    while left > 0:
        out[weighted[-1 - (i % len(weighted))]] += 1
        left -= 1
        i += 1
    return out


print(distribute([80, 200, 60], [0, 1, 0], 500))
print(distribute([100, 100, 100], [1, 2, 1], 403))
