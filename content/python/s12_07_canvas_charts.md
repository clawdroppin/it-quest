@@lesson s12-tk-canvas Drawing on the Canvas: Charts and Custom Widgets
@@concepts objects, numbers
@@text
`tk.Canvas` is a drawing surface: lines, rectangles, ovals, polygons, text and images, each an object you can move, recolour or delete later. It's how you add charts to a monitoring tool, draw a rack diagram, or build a custom widget like a gauge.

## Drawing basics

```python,norun
import tkinter as tk

root = tk.Tk()
c = tk.Canvas(root, width=400, height=220, bg="white", highlightthickness=0)
c.pack(fill="both", expand=True)

c.create_line(20, 200, 380, 200, fill="#888")                       # x axis
bar = c.create_rectangle(40, 80, 90, 200, fill="#2f6fde", outline="")
c.create_text(65, 70, text="72%", font=("Segoe UI", 10, "bold"))
c.create_oval(300, 30, 340, 70, fill="#2e7d32", outline="", tags=("led",))
c.create_arc(150, 40, 270, 160, start=90, extent=-260, style="arc", width=10, outline="#e65100")

c.itemconfig(bar, fill="#c62828")              # change later by id
c.move("led", 10, 0)                           # or by tag
c.tag_bind("led", "<Button-1>", lambda e: print("LED clicked"))
root.mainloop()
```

- Coordinates are pixels from the **top-left** corner; **y grows downwards**.
- Every `create_*` returns an **item id**; **tags** name groups of items (`"bars"`, `"grid"`), so you can update or delete them together: `c.delete("bars")`.
- Items can have their own event bindings (`tag_bind`): clickable chart bars, draggable icons.
- Redraw on resize: bind `<Configure>` and use `event.width`/`event.height`.

## From data to pixels

Every chart maps **data values** to **pixel coordinates** with a linear scale:

```text
pixel = p0 + (value - v0) / (v1 - v0) * (p1 - p0)
```

For a y axis, `p0` is the **bottom** pixel and `p1` the top, which flips the direction so bigger values go up. The first exercise computes bar chart geometry this way; the second picks **nice axis ticks** (0, 25, 50, 75, 100 rather than 0, 23.7, 47.4...), the same algorithm charting libraries use.

## Animation

`root.after(ms, callback)` schedules a callback. Calling it again from inside the callback creates an animation loop without threads:

```python,norun
def pulse(step=0):
    size = 20 + 6 * abs((step % 20) - 10) / 10
    c.coords("led", 320 - size / 2, 50 - size / 2, 320 + size / 2, 50 + size / 2)
    root.after(50, pulse, step + 1)
pulse()
```

Keep each frame's work small. For smooth easing, see the animation lessons in the GUI Design track.

## Matplotlib inside Tkinter

For full charts, embed **matplotlib**:

```python,norun
from matplotlib.figure import Figure
from matplotlib.backends.backend_tkagg import FigureCanvasTkAgg

fig = Figure(figsize=(5, 3), dpi=100)
ax = fig.add_subplot()
ax.plot([1, 2, 3, 4], [35, 60, 42, 88], marker="o")
ax.set_title("CPU %")
canvas = FigureCanvasTkAgg(fig, master=root)
canvas.get_tk_widget().pack(fill="both", expand=True)
canvas.draw()
```

Use the Canvas for lightweight, interactive, live-updating visuals; matplotlib for proper scientific/business charts and exports to PNG/PDF.

@@quiz
Where is (0, 0) on a Tk Canvas?
- The centre
- The bottom-left
* The top-left; y grows downwards
- Wherever you set it
? So y axes must be flipped for charts.

@@quiz
What are canvas tags useful for?
* Naming groups of items so you can update, delete or bind events to all of them at once
- Styling text only
- Saving the canvas
- Setting the window title
? E.g. `c.delete("bars")` before redrawing.

@@quiz
How do you animate without threads in Tkinter?
- time.sleep in a loop
* Schedule the next frame with `root.after(ms, callback)` from inside the callback
- Use mainloop twice
- Call update() in a while loop forever
? The event loop stays responsive between frames.

@@flashcards
Canvas :: drawing surface with movable items
Item id :: number returned by `create_*`, used to change the item
Canvas tag :: a name for a group of items
`coords` / `move` / `itemconfig` :: change an item's geometry or options
`tag_bind` :: event bindings for canvas items
Linear scale :: maps a data range to a pixel range
Nice ticks :: round axis values like 0, 25, 50
`after(ms, f)` :: schedule f; used for animation and polling
FigureCanvasTkAgg :: embeds a matplotlib figure in Tkinter

@@exercise Bar chart geometry
Write `scale(value, v0, v1, p0, p1)` (the linear mapping from the lesson, returning a float) and `bars(values, width, height, pad=20, gap=10)` returning the rectangle coordinates `(x0, y0, x1, y1)` (rounded to ints) for a bar chart:

- the plot area is from `pad` to `width - pad` horizontally and from `pad` (top) to `height - pad` (bottom)
- bars share the plot width equally: each slot is `(plot_width) / n` wide; the bar is the slot minus `gap` (half the gap on each side)
- the value axis runs from 0 (bottom) to the **maximum value** (top of the plot area); a bar goes from the bottom up to its value
- empty `values` → `[]`; if every value is 0, all bars have zero height
@@starter
def scale(value, v0, v1, p0, p1):
    pass


def bars(values, width, height, pad=20, gap=10):
    pass


print(scale(50, 0, 100, 200, 0), scale(75, 0, 100, 0, 400))
print(bars([30, 60, 45], width=340, height=220))
@@expected
100.0 300.0
[(25, 110, 115, 200), (125, 20, 215, 200), (225, 65, 315, 200)]
@@tests
check(bars([], 300, 200), [], "empty")
check(bars([0, 0], 240, 120, pad=20, gap=0), [(20, 100, 120, 100), (120, 100, 220, 100)], "all zeros")
check(bars([5], 100, 100, pad=0, gap=0), [(0, 0, 100, 100)], "single full bar")
@@hint
`scale`: `p0 + (value - v0) / (v1 - v0) * (p1 - p0)`. In `bars`: `slot = (width - 2 * pad) / n`; `x0 = pad + i * slot + gap / 2`; `x1 = pad + (i + 1) * slot - gap / 2`; `top = max(values) or 1` avoids dividing by zero; `y0 = scale(v, 0, top, height - pad, pad)`.
@@solution
def scale(value, v0, v1, p0, p1):
    return p0 + (value - v0) / (v1 - v0) * (p1 - p0)


def bars(values, width, height, pad=20, gap=10):
    if not values:
        return []
    n = len(values)
    slot = (width - 2 * pad) / n
    top = max(values) or 1
    bottom = height - pad
    out = []
    for i, v in enumerate(values):
        x0 = pad + i * slot + gap / 2
        x1 = pad + (i + 1) * slot - gap / 2
        y0 = scale(v, 0, top, bottom, pad)
        out.append((round(x0), round(y0), round(x1), round(bottom)))
    return out


print(scale(50, 0, 100, 200, 0), scale(75, 0, 100, 0, 400))
print(bars([30, 60, 45], width=340, height=220))

@@exercise Nice axis ticks
Write `nice_ticks(max_value, target=5)` returning tick values from 0 up to a "nice" maximum that is ≥ `max_value`:

1. `raw = max_value / target`
2. `magnitude = 10 ** floor(log10(raw))`
3. the step is the smallest of `1, 2, 2.5, 5, 10` times `magnitude` that is ≥ `raw`
4. ticks are `0, step, 2*step, ...` up to the first tick ≥ `max_value`
5. return ints when the step is a whole number, otherwise floats rounded to 10 decimals

`max_value <= 0` → `[0, 1]`.
@@starter
import math


def nice_ticks(max_value, target=5):
    pass


print(nice_ticks(87), nice_ticks(1234), nice_ticks(0.37), nice_ticks(100))
@@expected
[0, 20, 40, 60, 80, 100] [0, 250, 500, 750, 1000, 1250] [0.0, 0.1, 0.2, 0.3, 0.4] [0, 20, 40, 60, 80, 100]
@@tests
check(nice_ticks(0), [0, 1], "zero")
check(nice_ticks(7, target=7), [0, 1, 2, 3, 4, 5, 6, 7], "small integers")
check(nice_ticks(9.5, target=2), [0, 5, 10], "target 2")
check(nice_ticks(23), [0, 5, 10, 15, 20, 25], "23")
@@hint
`for m in (1, 2, 2.5, 5, 10): if m * magnitude >= raw - 1e-12: step = m * magnitude; break`. Build ticks with `i * step` while the previous tick is below `max_value`. Decide the number type with `float(step).is_integer()`.
@@solution
import math


def nice_ticks(max_value, target=5):
    if max_value <= 0:
        return [0, 1]
    raw = max_value / target
    magnitude = 10 ** math.floor(math.log10(raw))
    for m in (1, 2, 2.5, 5, 10):
        if m * magnitude >= raw - 1e-12:
            step = m * magnitude
            break
    whole = float(step).is_integer()
    ticks = []
    i = 0
    while True:
        value = i * step
        ticks.append(int(round(value)) if whole else round(value, 10))
        if value >= max_value - 1e-12:
            return ticks
        i += 1


print(nice_ticks(87), nice_ticks(1234), nice_ticks(0.37), nice_ticks(100))
