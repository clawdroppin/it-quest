@@lesson i3-files Files and Folders
@@concepts files
@@text
Scripts in IT read config files and logs, write reports and CSVs, and move files around. Python makes this easy.

## Writing and reading a text file

```python
with open("report.txt", "w", encoding="utf-8") as f:
    f.write("web01 OK\n")
    f.write("db01 FAIL\n")

with open("report.txt", encoding="utf-8") as f:
    text = f.read()
print(text)
```

- `open(path, mode)`: `"r"` read (the default), `"w"` write (replaces the file!), `"a"` append.
- `with` closes the file automatically when the block ends, even if an error happens. Always use it.
- Always pass `encoding="utf-8"`, especially on Windows, where the default may be something older.
- `write` doesn't add new lines; you add `\n` yourself.

## Reading line by line

For logs, which can be huge, loop over the file: Python reads one line at a time.

```python
with open("app.log", "w", encoding="utf-8") as f:
    f.write("INFO start\nERROR disk full\nINFO retry\nERROR disk full\n")

errors = 0
with open("app.log", encoding="utf-8") as f:
    for line in f:
        line = line.rstrip("\n")
        if line.startswith("ERROR"):
            errors += 1
            print("found:", line)
print(errors, "errors")
```

Each line ends with `\n`; `rstrip("\n")` (or `strip()`) removes it.

## pathlib: paths the modern way

`pathlib.Path` represents a file or folder path and works on Windows, Linux and macOS alike:

```python
from pathlib import Path

folder = Path("logs")
folder.mkdir(exist_ok=True)              # create if missing
(folder / "a.log").write_text("one\n", encoding="utf-8")
(folder / "b.log").write_text("two\nthree\n", encoding="utf-8")
(folder / "notes.txt").write_text("x", encoding="utf-8")

for p in sorted(folder.glob("*.log")):   # wildcard search
    lines = p.read_text(encoding="utf-8").splitlines()
    print(p.name, p.suffix, p.stem, len(lines), "lines", p.stat().st_size, "bytes")

print(Path("C:/Users/ana/report.pdf").parent)
print((folder / "missing.txt").exists())
```

- `/` joins path parts: `folder / "a.log"`. No more backslash headaches.
- `.name`, `.stem` (name without extension), `.suffix` (extension), `.parent`.
- `.exists()`, `.is_file()`, `.is_dir()`, `.mkdir()`, `.glob("*.log")`, `.rglob("*.log")` (also in subfolders).
- `.read_text()` / `.write_text()` for quick whole-file work.

## Missing files

```python
try:
    with open("nope.txt", encoding="utf-8") as f:
        print(f.read())
except FileNotFoundError:
    print("config file missing, using defaults")
```

@@quiz
What happens to an existing file when you open it with mode `"w"`?
- New text is added at the end
* Its contents are erased and replaced
- Python raises an error
- Nothing until you call save()
? `"w"` truncates the file. Use `"a"` to append.

@@quiz
Why use `with open(path) as f:` instead of `f = open(path)`?
* The file is closed automatically when the block ends, even after an error
- It's the only way to read text
- It makes reading faster
- It creates the file if it's missing
? with handles the clean-up for you.

@@quiz
Which pathlib call gives the file name without the folder, from `Path("logs/app.log")`?
* `.name`, which is "app.log"
- `.parent`
- `.suffix`
- `.stem`, which is "app.log"
? .stem is "app", .suffix is ".log", .parent is "logs".

@@flashcards
`with open(p, encoding="utf-8") as f:` :: open a file; it's closed automatically after the block
mode `"w"` :: write: erases the existing file
mode `"a"` :: append to the end of the file
`Path("a") / "b.txt"` :: join path parts with pathlib
`p.glob("*.log")` :: all .log files in folder p (`rglob` includes subfolders)

@@exercise Count log levels
The setup code (already in the starter) writes a log file. Write `count_levels(path)` that returns a dict counting the **first word** of each non-empty line.
@@starter
from pathlib import Path

Path("server.log").write_text(
    "INFO boot\nWARN fan slow\nINFO login ana\n\nERROR disk\nINFO logout ana\n", encoding="utf-8"
)


def count_levels(path):
    pass


print(count_levels("server.log"))
@@tests
from pathlib import Path as _P
_P("t.log").write_text("A x\nB y\nA z\n\n", encoding="utf-8")
check(count_levels("server.log"), {"INFO": 3, "WARN": 1, "ERROR": 1}, "count_levels('server.log')")
check(count_levels("t.log"), {"A": 2, "B": 1}, "count_levels on another file")
@@hint
Open the file, loop over lines, `words = line.split()`; skip the line if `words` is empty; count `words[0]`.
@@solution
from pathlib import Path

Path("server.log").write_text(
    "INFO boot\nWARN fan slow\nINFO login ana\n\nERROR disk\nINFO logout ana\n", encoding="utf-8"
)


def count_levels(path):
    counts = {}
    with open(path, encoding="utf-8") as f:
        for line in f:
            words = line.split()
            if words:
                counts[words[0]] = counts.get(words[0], 0) + 1
    return counts


print(count_levels("server.log"))

@@exercise Find big files
Write `big_files(folder, min_bytes)` that returns a **sorted list of file names** (just the name, like `"b.iso"`) of the files directly in `folder` whose size is **at least** `min_bytes`.
@@starter
from pathlib import Path

d = Path("downloads")
d.mkdir(exist_ok=True)
(d / "a.txt").write_text("x" * 10)
(d / "b.iso").write_text("x" * 5000)
(d / "c.zip").write_text("x" * 2000)


def big_files(folder, min_bytes):
    pass


print(big_files("downloads", 1000))
@@tests
check(big_files("downloads", 1000), ["b.iso", "c.zip"], "big_files('downloads', 1000)")
check(big_files("downloads", 3000), ["b.iso"], "big_files('downloads', 3000)")
check(big_files("downloads", 10**9), [], "nothing that big")
@@hint
`for p in Path(folder).iterdir():` — keep it if `p.is_file()` and `p.stat().st_size >= min_bytes`.
@@solution
from pathlib import Path

d = Path("downloads")
d.mkdir(exist_ok=True)
(d / "a.txt").write_text("x" * 10)
(d / "b.iso").write_text("x" * 5000)
(d / "c.zip").write_text("x" * 2000)


def big_files(folder, min_bytes):
    return sorted(p.name for p in Path(folder).iterdir() if p.is_file() and p.stat().st_size >= min_bytes)


print(big_files("downloads", 1000))
