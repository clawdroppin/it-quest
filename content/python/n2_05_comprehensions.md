@@lesson n2-comprehensions Comprehensions: Lists, Dicts and Sets in One Line
@@concepts lists, maps, loops
@@text
A huge share of loops do one of three things: transform every item, keep some items, or build a dict. **Comprehensions** say that in one readable line, and they're one of the most recognisably "Pythonic" features.

## List comprehensions

```python
sizes_mb = [120, 3400, 56, 990]

# the loop way
sizes_gb = []
for s in sizes_mb:
    sizes_gb.append(round(s / 1024, 2))

# the comprehension way
sizes_gb = [round(s / 1024, 2) for s in sizes_mb]
print(sizes_gb)
```

Read it as: *"the list of `round(s / 1024, 2)` for each `s` in `sizes_mb`"*.

## Filtering with if

```python
hosts = ["web01", "db01", "web02", "mail01"]
web = [h for h in hosts if h.startswith("web")]
upper_web = [h.upper() for h in hosts if h.startswith("web")]
print(web, upper_web)
```

An `if` **at the end** filters. A conditional **expression** at the start transforms every item one of two ways:

```python
loads = [35, 92, 51]
labels = ["HIGH" if x > 80 else "ok" for x in loads]
print(labels)
```

## Dict and set comprehensions

```python
users = ["maya", "li", "sam"]
lengths = {u: len(u) for u in users}               # dict
quota = {"maya": 50, "li": 120, "sam": 80}
over_100 = {u: gb for u, gb in quota.items() if gb > 100}
inverted = {gb: u for u, gb in quota.items()}       # swap keys and values
domains = {email.split("@")[1] for email in ["a@x.com", "b@y.org", "c@x.com"]}  # set
print(lengths, over_100, inverted, domains)
```

## Nested comprehensions

Two `for`s read **left to right, like nested loops**:

```python
racks = [["web01", "web02"], ["db01"], ["mail01", "mail02"]]
all_hosts = [host for rack in racks for host in rack]   # flatten
grid = [(r, c) for r in range(2) for c in range(3)]
print(all_hosts)
print(grid)
```

More than two levels gets hard to read: write a normal loop then.

## Generator expressions

Use parentheses instead of brackets and you get a **generator**: values are produced one at a time without building a list. Perfect when you only need to feed a function like `sum`, `max`, `any`:

```python
sizes = [120, 3400, 56, 990]
print(sum(s for s in sizes if s > 100))      # no temporary list
print(max(len(h) for h in ["web01", "mail01"]))
```

## When not to use one

- If the loop body has **side effects** (printing, writing files, sending requests), use a normal `for` loop. `[print(x) for x in items]` builds a useless list of `None`s.
- If it doesn't fit comfortably on one or two lines, a loop is clearer.
- If you need `try/except` per item, use a loop (or a helper function inside the comprehension).

## The walrus operator

`:=` assigns inside an expression, handy to avoid computing something twice:

```python
import re
lines = ["user=maya ok", "noise", "user=li fail"]
users = [m.group(1) for line in lines if (m := re.search(r"user=(\w+)", line))]
print(users)
```

@@quiz
What does `[x * 2 for x in range(4) if x % 2 == 0]` give?
- [0, 2, 4, 6]
* [0, 4]
- [2, 6]
- [0, 1, 2, 3]
? Only 0 and 2 pass the filter; then they're doubled.

@@quiz
Where does a filtering `if` go in a comprehension?
* At the end, after the `for`
- At the start, before the expression
- Inside square brackets around the expression
- Comprehensions can't filter
? `"a" if c else "b"` at the start is a conditional expression, not a filter.

@@quiz
Why use `sum(s for s in sizes)` instead of `sum([s for s in sizes])`?
- It's a different result
* The generator expression doesn't build a temporary list
- Lists can't be summed
- Generators are sorted
? For big inputs this saves memory.

@@quiz
When should you prefer a normal loop over a comprehension?
- Never
- When the result is a list
* When the body has side effects like printing or writing files, or the logic is complex
- When filtering
? Comprehensions are for building values.

@@flashcards
List comprehension :: `[expr for x in items if cond]`
Dict comprehension :: `{k: v for k, v in pairs}`
Set comprehension :: `{expr for x in items}`
Generator expression :: `(expr for x in items)`, produces values lazily
Conditional expression :: `a if cond else b`
Nested comprehension order :: the `for` clauses read left to right like nested loops
Walrus operator :: `:=` assigns inside an expression

@@exercise Clean up a user export
`rows` is a list of dicts like `{"name": " Maya ", "email": "MAYA@Corp.com", "active": True}`. Using comprehensions:

- `active_emails(rows)`: lower-cased emails of active users, in order
- `name_by_email(rows)`: a dict from lower-cased email to the stripped, title-cased name, for all users
- `domains(rows)`: the set of lower-cased domains (the part after `@`)
@@starter
def active_emails(rows):
    pass


def name_by_email(rows):
    pass


def domains(rows):
    pass


ROWS = [
    {"name": " maya ", "email": "MAYA@Corp.com", "active": True},
    {"name": "li wei", "email": "li@corp.com", "active": False},
    {"name": "sam", "email": "Sam@Partner.org", "active": True},
]
print(active_emails(ROWS))
@@tests
ROWS = [
    {"name": " maya ", "email": "MAYA@Corp.com", "active": True},
    {"name": "li wei", "email": "li@corp.com", "active": False},
    {"name": "sam", "email": "Sam@Partner.org", "active": True},
]
check(active_emails(ROWS), ["maya@corp.com", "sam@partner.org"], "active emails")
check(name_by_email(ROWS), {"maya@corp.com": "Maya", "li@corp.com": "Li Wei", "sam@partner.org": "Sam"}, "names")
check(domains(ROWS), {"corp.com", "partner.org"}, "domains")
check(active_emails([]), [], "empty")
@@hint
`[r["email"].lower() for r in rows if r["active"]]`; `{r["email"].lower(): r["name"].strip().title() for r in rows}`; `{r["email"].lower().split("@")[1] for r in rows}`.
@@solution
def active_emails(rows):
    return [r["email"].lower() for r in rows if r["active"]]


def name_by_email(rows):
    return {r["email"].lower(): r["name"].strip().title() for r in rows}


def domains(rows):
    return {r["email"].lower().split("@")[1] for r in rows}


ROWS = [
    {"name": " maya ", "email": "MAYA@Corp.com", "active": True},
    {"name": "li wei", "email": "li@corp.com", "active": False},
    {"name": "sam", "email": "Sam@Partner.org", "active": True},
]
print(active_emails(ROWS))

@@exercise Matrix tricks
Write with comprehensions:

- `flatten(grid)`: all values row by row
- `transpose(grid)`: rows become columns (`[[1, 2, 3], [4, 5, 6]]` → `[[1, 4], [2, 5], [3, 6]]`); assume all rows have the same length and the grid isn't empty
- `hot_cells(grid, limit)`: a list of `(row, col)` positions whose value is greater than `limit`
@@starter
def flatten(grid):
    pass


def transpose(grid):
    pass


def hot_cells(grid, limit):
    pass


temps = [[21, 35, 22], [40, 19, 38]]
print(flatten(temps), transpose(temps), hot_cells(temps, 30))
@@tests
g = [[21, 35, 22], [40, 19, 38]]
check(flatten(g), [21, 35, 22, 40, 19, 38], "flatten")
check(transpose(g), [[21, 40], [35, 19], [22, 38]], "transpose")
check(hot_cells(g, 30), [(0, 1), (1, 0), (1, 2)], "hot cells")
check(flatten([]), [], "flatten empty")
@@hint
`[v for row in grid for v in row]`. Transpose: `[[row[c] for row in grid] for c in range(len(grid[0]))]` (or `[list(col) for col in zip(*grid)]`). Hot cells: `[(r, c) for r, row in enumerate(grid) for c, v in enumerate(row) if v > limit]`.
@@solution
def flatten(grid):
    return [v for row in grid for v in row]


def transpose(grid):
    return [[row[c] for row in grid] for c in range(len(grid[0]))]


def hot_cells(grid, limit):
    return [(r, c) for r, row in enumerate(grid) for c, v in enumerate(row) if v > limit]


temps = [[21, 35, 22], [40, 19, 38]]
print(flatten(temps), transpose(temps), hot_cells(temps, 30))
