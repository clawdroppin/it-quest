@@lesson i4-json-csv JSON and CSV
@@concepts json, files
@@text
Two text formats carry most of the data you'll handle in IT:

- **JSON**: what web APIs, cloud tools (AWS, Azure CLI), and modern config files speak.
- **CSV**: what spreadsheets, user exports and reports use.

## JSON

JSON looks almost exactly like Python dicts and lists. The `json` module converts between them:

```python
import json

text = '{"host": "web01", "ports": [80, 443], "healthy": true, "owner": null}'
data = json.loads(text)          # JSON text -> Python
print(data["ports"][1], data["healthy"], data["owner"])

report = {"checked": 3, "failed": ["db01"]}
print(json.dumps(report))                 # Python -> JSON text
print(json.dumps(report, indent=2))       # pretty-printed
```

| JSON | Python |
|---|---|
| object `{}` | dict |
| array `[]` | list |
| `"text"` | str |
| `42`, `3.5` | int, float |
| `true` / `false` | `True` / `False` |
| `null` | `None` |

For files: `json.load(f)` reads, `json.dump(obj, f, indent=2)` writes.

```python
import json

config = {"servers": ["web01", "web02"], "retries": 3}
with open("config.json", "w", encoding="utf-8") as f:
    json.dump(config, f, indent=2)
with open("config.json", encoding="utf-8") as f:
    loaded = json.load(f)
print(loaded["retries"], loaded == config)
```

Real API data is **nested**. Walk into it step by step:

```python
import json

resp = json.loads('{"items": [{"name": "vm1", "state": {"code": 16, "name": "running"}}, {"name": "vm2", "state": {"code": 80, "name": "stopped"}}]}')
for vm in resp["items"]:
    print(vm["name"], vm["state"]["name"])
```

## CSV

`csv.DictReader` reads each row as a dictionary, using the header row as keys. Everything comes in as **text**; convert numbers yourself.

```python
import csv

with open("users.csv", "w", encoding="utf-8", newline="") as f:
    f.write("name,dept,storage_gb\nana,IT,120\nben,HR,15\ncarl,IT,300\n")

with open("users.csv", encoding="utf-8", newline="") as f:
    for row in csv.DictReader(f):
        print(row["name"], row["dept"], int(row["storage_gb"]) * 2)
```

Writing works the same way with `csv.DictWriter`:

```python
import csv

rows = [{"host": "web01", "status": "ok"}, {"host": "db01", "status": "down"}]
with open("status.csv", "w", encoding="utf-8", newline="") as f:
    w = csv.DictWriter(f, fieldnames=["host", "status"])
    w.writeheader()
    w.writerows(rows)
print(open("status.csv", encoding="utf-8").read())
```

Pass `newline=""` when opening CSV files; the `csv` module handles line endings itself. Never split CSV lines with `.split(",")`: values can contain commas inside quotes, and the `csv` module handles that correctly.

@@quiz
What does `json.loads('{"ok": true}')["ok"]` give in Python?
- `"true"`
- `true`
* `True`
- `1`
? JSON `true` becomes Python's `True`.

@@quiz
Why open a CSV file with `newline=""`?
* So the csv module handles line endings itself and doesn't add blank rows on Windows
- To skip the header row
- To read it faster
- To remove commas
? The csv docs recommend newline="" for both reading and writing.

@@quiz
Which gives you each CSV row as a dictionary keyed by the header names?
* `csv.DictReader(f)`
- `csv.reader(f)`
- `json.load(f)`
- `f.readlines()`
? DictWriter does the reverse for writing.

@@flashcards
`json.loads(text)` / `json.dumps(obj)` :: JSON text → Python / Python → JSON text
`json.load(f)` / `json.dump(obj, f, indent=2)` :: the same, with files
JSON `null` :: Python `None`
`csv.DictReader(f)` :: read CSV rows as dicts (values are strings!)
`newline=""` :: always pass it when opening CSV files

@@exercise Stopped VMs
Write `stopped_vms(json_text)` that takes JSON shaped like the cloud example (an object with an `"items"` list, each with a `"name"` and `"state": {"name": ...}`) and returns a sorted list of the names of VMs whose state name is `"stopped"`.
@@starter
import json


def stopped_vms(json_text):
    pass


sample = '{"items": [{"name": "vm1", "state": {"name": "running"}}, {"name": "vm3", "state": {"name": "stopped"}}, {"name": "vm2", "state": {"name": "stopped"}}]}'
print(stopped_vms(sample))
@@tests
sample = '{"items": [{"name": "vm1", "state": {"name": "running"}}, {"name": "vm3", "state": {"name": "stopped"}}, {"name": "vm2", "state": {"name": "stopped"}}]}'
check(stopped_vms(sample), ["vm2", "vm3"], "stopped_vms(sample)")
check(stopped_vms('{"items": []}'), [], "no VMs")
@@hint
`data = json.loads(json_text)`, then loop over `data["items"]` and check `vm["state"]["name"]`.
@@solution
import json


def stopped_vms(json_text):
    data = json.loads(json_text)
    return sorted(vm["name"] for vm in data["items"] if vm["state"]["name"] == "stopped")


sample = '{"items": [{"name": "vm1", "state": {"name": "running"}}, {"name": "vm3", "state": {"name": "stopped"}}, {"name": "vm2", "state": {"name": "stopped"}}]}'
print(stopped_vms(sample))

@@exercise Storage per department
The starter writes a CSV of users. Write `storage_by_dept(path)` that returns a dict from department to the **total** storage in GB (as int).
@@starter
import csv

with open("users.csv", "w", encoding="utf-8", newline="") as f:
    f.write('name,dept,storage_gb\nana,IT,120\nben,HR,15\ncarl,IT,300\n"Doe, Jane",HR,40\n')


def storage_by_dept(path):
    pass


print(storage_by_dept("users.csv"))
@@tests
check(storage_by_dept("users.csv"), {"IT": 420, "HR": 55}, "storage_by_dept('users.csv')")
@@hint
Open with `newline=""`, loop over `csv.DictReader(f)`, and add `int(row["storage_gb"])` to `totals[row["dept"]]` using `.get(..., 0)`. Notice the name with a comma inside quotes: `.split(",")` would break on it, `csv` won't.
@@solution
import csv

with open("users.csv", "w", encoding="utf-8", newline="") as f:
    f.write('name,dept,storage_gb\nana,IT,120\nben,HR,15\ncarl,IT,300\n"Doe, Jane",HR,40\n')


def storage_by_dept(path):
    totals = {}
    with open(path, encoding="utf-8", newline="") as f:
        for row in csv.DictReader(f):
            totals[row["dept"]] = totals.get(row["dept"], 0) + int(row["storage_gb"])
    return totals


print(storage_by_dept("users.csv"))
