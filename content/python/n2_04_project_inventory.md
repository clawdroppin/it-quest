@@lesson n2-project-inventory 🏆 Module Project: Asset Inventory
@@concepts lists, maps, functions
@@text
Time to put everything from the Novice level together. Every IT department keeps an **asset inventory**: which laptops and servers exist, who has them, when their warranty ends. You'll write the functions behind a small inventory report.

Each asset is a dictionary:

```python
assets = [
    {"tag": "LT-001", "type": "laptop", "owner": "ana", "year": 2019},
    {"tag": "LT-002", "type": "laptop", "owner": "ben", "year": 2023},
    {"tag": "SV-001", "type": "server", "owner": "it", "year": 2017},
    {"tag": "MN-004", "type": "monitor", "owner": "ana", "year": 2022},
]
for a in assets:
    print(a["tag"], a["type"], a["owner"])
```

Work through the exercises in order: each builds a piece of the final report. Plan before you type: what goes in, what comes out, and which pattern (loop and filter, counting, grouping) fits?

@@quiz
In an asset inventory, which structure fits "look up an asset by its tag"?
* A dictionary keyed by tag
- A list, searched each time
- A tuple
- A set of tags only
? Dictionaries give direct lookup by key.

@@quiz
Why keep each asset as a dictionary inside a list (or a dict of dicts) instead of separate lists for names, owners and locations?
* Each asset's fields stay together, so they can't get out of step
- It uses less memory
- Separate lists can't hold strings
- Python requires it
? Parallel lists break as soon as one of them is sorted or edited on its own.

@@quiz
What does `sorted(assets, key=lambda a: a["location"])` do?
* Returns a new list of assets ordered by location
- Sorts the assets in place
- Returns only the locations
- Groups assets by location
? key tells sorted which value to compare.

@@flashcards
list of dicts :: one dict per record, e.g. per asset
dict of dicts :: records keyed by an ID
sorted(x, key=...) :: new list ordered by a chosen value
lambda a: a["x"] :: a tiny inline function
len() :: how many items

@@exercise Count by type
Write `count_by_type(assets)` that returns a dict like `{"laptop": 2, "server": 1, "monitor": 1}`.
@@starter
def count_by_type(assets):
    pass

assets = [
    {"tag": "LT-001", "type": "laptop", "owner": "ana", "year": 2019},
    {"tag": "LT-002", "type": "laptop", "owner": "ben", "year": 2023},
    {"tag": "SV-001", "type": "server", "owner": "it", "year": 2017},
    {"tag": "MN-004", "type": "monitor", "owner": "ana", "year": 2022},
]
print(count_by_type(assets))
@@tests
assets = [
    {"tag": "LT-001", "type": "laptop", "owner": "ana", "year": 2019},
    {"tag": "LT-002", "type": "laptop", "owner": "ben", "year": 2023},
    {"tag": "SV-001", "type": "server", "owner": "it", "year": 2017},
    {"tag": "MN-004", "type": "monitor", "owner": "ana", "year": 2022},
]
check(count_by_type(assets), {"laptop": 2, "server": 1, "monitor": 1}, "count_by_type(assets)")
check(count_by_type([]), {}, "count_by_type([])")
@@hint
The counting pattern: `counts[a["type"]] = counts.get(a["type"], 0) + 1`.
@@solution
def count_by_type(assets):
    counts = {}
    for a in assets:
        counts[a["type"]] = counts.get(a["type"], 0) + 1
    return counts

assets = [
    {"tag": "LT-001", "type": "laptop", "owner": "ana", "year": 2019},
    {"tag": "LT-002", "type": "laptop", "owner": "ben", "year": 2023},
    {"tag": "SV-001", "type": "server", "owner": "it", "year": 2017},
    {"tag": "MN-004", "type": "monitor", "owner": "ana", "year": 2022},
]
print(count_by_type(assets))

@@exercise Due for replacement
Hardware is replaced after `max_age` years. Write `due_for_replacement(assets, this_year, max_age)` that returns a **sorted list of tags** of assets whose age (`this_year - year`) is **greater than or equal to** `max_age`.
@@starter
def due_for_replacement(assets, this_year, max_age):
    pass
@@tests
assets = [
    {"tag": "LT-001", "type": "laptop", "owner": "ana", "year": 2019},
    {"tag": "LT-002", "type": "laptop", "owner": "ben", "year": 2023},
    {"tag": "SV-001", "type": "server", "owner": "it", "year": 2017},
    {"tag": "MN-004", "type": "monitor", "owner": "ana", "year": 2022},
]
check(due_for_replacement(assets, 2026, 5), ["LT-001", "SV-001"], "due_for_replacement(assets, 2026, 5)")
check(due_for_replacement(assets, 2026, 20), [], "nothing that old")
@@hint
A list comprehension with a condition, then `sorted`.
@@solution
def due_for_replacement(assets, this_year, max_age):
    return sorted(a["tag"] for a in assets if this_year - a["year"] >= max_age)

@@exercise Assets per person
Write `by_owner(assets)` that returns a dict from owner to a **sorted list of their tags**.
@@starter
def by_owner(assets):
    pass
@@tests
assets = [
    {"tag": "LT-001", "type": "laptop", "owner": "ana", "year": 2019},
    {"tag": "LT-002", "type": "laptop", "owner": "ben", "year": 2023},
    {"tag": "SV-001", "type": "server", "owner": "it", "year": 2017},
    {"tag": "MN-004", "type": "monitor", "owner": "ana", "year": 2022},
]
check(by_owner(assets), {"ana": ["LT-001", "MN-004"], "ben": ["LT-002"], "it": ["SV-001"]}, "by_owner(assets)")
@@hint
Group with `setdefault(owner, []).append(tag)`, then sort each list: `for owner in groups: groups[owner].sort()`.
@@solution
def by_owner(assets):
    groups = {}
    for a in assets:
        groups.setdefault(a["owner"], []).append(a["tag"])
    for owner in groups:
        groups[owner].sort()
    return groups

@@exercise The report
Put it together. Using your functions (paste them in), print this report for the sample assets in 2026 with a 5-year limit. The type lines are in **alphabetical order**:

```text
=== Asset report 2026 ===
laptop: 2
monitor: 1
server: 1
Replace soon: LT-001, SV-001
ana has 2 asset(s)
ben has 1 asset(s)
it has 1 asset(s)
```
@@starter
assets = [
    {"tag": "LT-001", "type": "laptop", "owner": "ana", "year": 2019},
    {"tag": "LT-002", "type": "laptop", "owner": "ben", "year": 2023},
    {"tag": "SV-001", "type": "server", "owner": "it", "year": 2017},
    {"tag": "MN-004", "type": "monitor", "owner": "ana", "year": 2022},
]

print("=== Asset report 2026 ===")
@@expected
=== Asset report 2026 ===
laptop: 2
monitor: 1
server: 1
Replace soon: LT-001, SV-001
ana has 2 asset(s)
ben has 1 asset(s)
it has 1 asset(s)
@@hint
`for t, n in sorted(count_by_type(assets).items()):` gives the types alphabetically. Use `", ".join(...)` for the replacement line.
@@solution
def count_by_type(assets):
    counts = {}
    for a in assets:
        counts[a["type"]] = counts.get(a["type"], 0) + 1
    return counts


def due_for_replacement(assets, this_year, max_age):
    return sorted(a["tag"] for a in assets if this_year - a["year"] >= max_age)


def by_owner(assets):
    groups = {}
    for a in assets:
        groups.setdefault(a["owner"], []).append(a["tag"])
    for owner in groups:
        groups[owner].sort()
    return groups


assets = [
    {"tag": "LT-001", "type": "laptop", "owner": "ana", "year": 2019},
    {"tag": "LT-002", "type": "laptop", "owner": "ben", "year": 2023},
    {"tag": "SV-001", "type": "server", "owner": "it", "year": 2017},
    {"tag": "MN-004", "type": "monitor", "owner": "ana", "year": 2022},
]

print("=== Asset report 2026 ===")
for t, n in sorted(count_by_type(assets).items()):
    print(f"{t}: {n}")
print("Replace soon:", ", ".join(due_for_replacement(assets, 2026, 5)))
for owner, tags in sorted(by_owner(assets).items()):
    print(f"{owner} has {len(tags)} asset(s)")
@@text
🎉 **That's the Novice level of Python done.** You can now write real little tools: variables, decisions, loops, functions, lists and dictionaries are 80% of everyday scripting.

Next, in **Intermediate**, your programs start to touch the real world: files, errors, the standard library, and data formats like JSON and CSV.
