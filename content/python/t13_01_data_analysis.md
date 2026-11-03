@@lesson t13-data 🎓 Beyond IT: Data Analysis with Python
@@concepts numbers, lists, maps, files
@@text
> **Beyond IT scope.** This module goes past typical IT and administration work into data science, machine learning, scientific computing and games: fields where Python is the leading language. It's here so the Python course covers what the language is famous for. IT staff do meet some of it (capacity reports, log analytics, anomaly alerts), but skip it freely if your goal is an IT job.

Data analysis is asking questions of a dataset: what's typical, what's unusual, how groups differ, how things change over time. Python dominates this field through **pandas**, **NumPy**, **matplotlib** and **Jupyter** notebooks. The ideas underneath are plain statistics you can compute with the standard library, which is what the exercises do.

## Describing data

| Measure | Meaning | Standard library |
|---|---|---|
| mean | the average; pulled by outliers | `statistics.mean` |
| median | the middle value; robust to outliers | `statistics.median` |
| mode | the most common value | `statistics.mode` |
| standard deviation | typical distance from the mean | `statistics.stdev` (sample), `pstdev` (population) |
| quartiles | values splitting sorted data into four equal parts | `statistics.quantiles(data, n=4)` |
| correlation | how two variables move together (−1..1) | `statistics.correlation` |

```python
import statistics as st

response_ms = [120, 135, 128, 140, 131, 125, 980, 133]
print("mean  ", round(st.mean(response_ms), 1))    # pulled up by the 980 outlier
print("median", st.median(response_ms))
print("stdev ", round(st.stdev(response_ms), 1))
print("quartiles", st.quantiles(response_ms, n=4))
```

When mean and median disagree a lot, the data is **skewed** or has outliers; report the median (and percentiles like p95) for things like response times.

## pandas in one screen

pandas' `DataFrame` is a table with named columns and fast, vectorised operations:

```python,norun
import pandas as pd

df = pd.read_csv("tickets.csv", parse_dates=["opened"])
df.head()                                   # first rows
df.describe()                               # count, mean, std, min, quartiles, max
df["team"].value_counts()                   # counts per category
df[df["priority"] == "high"]                # filter rows
df.groupby("team")["minutes"].agg(["count", "mean", "median"])
df.pivot_table(index="team", columns="priority", values="minutes", aggfunc="mean")
df.set_index("opened").resample("W")["id"].count()   # tickets per week
df.to_excel("report.xlsx")                  # needs openpyxl
```

Its newer rival **Polars** has a similar API and is much faster on big data. For a few thousand rows, the standard library (`csv`, `statistics`, `collections`) is plenty, as the exercises show.

## The analysis workflow

1. **Load** and look: shape, column types, a few rows.
2. **Clean**: missing values, wrong types, duplicates, inconsistent spellings (`"Net"`, `"network"`).
3. **Explore**: summaries, group comparisons, plots.
4. **Explain**: one clear chart and a sentence per finding. Correlation is not causation.

**Jupyter notebooks** (`pip install jupyterlab`) mix code, output, charts and notes in one document, which is how most analysis is done and shared.

@@quiz
Response times are `[120, 130, 125, 2000]`. Which summary best describes a typical request?
- The mean
* The median, because it isn't pulled up by the outlier
- The maximum
- The standard deviation
? Use percentiles (p50, p95, p99) for latency.

@@quiz
What does `df.groupby("team")["minutes"].mean()` compute?
- The mean of all minutes
* The average minutes for each team
- The number of teams
- Sorts the table by team
? Split into groups, apply a function, combine the results.

@@quiz
Two columns have a correlation of 0.9. What can you conclude?
- One causes the other
* They tend to rise and fall together, but that alone doesn't show causation
- They are identical
- The data is wrong
? A third factor may drive both.

@@flashcards
Mean :: average; sensitive to outliers
Median :: middle value; robust to outliers
Standard deviation :: typical spread around the mean
Quartiles :: values at 25%, 50%, 75% of sorted data
Skewed data :: mean and median differ a lot
DataFrame :: pandas table with named columns
groupby :: split rows into groups, aggregate each
Pivot table :: groups on rows and columns with an aggregate in each cell
Jupyter :: notebooks mixing code, output and notes

@@exercise Summarise a column
Write `summary(values)` returning a dict with `count`, `mean`, `median`, `stdev` (sample standard deviation), `min`, `max` and `outliers`: the values outside **1.5 × IQR** from the quartiles (IQR = Q3 − Q1, with Q1 and Q3 from `statistics.quantiles(values, n=4)`), in their original order. Round `mean` and `stdev` to 2 decimals. With fewer than 2 values, `stdev` is `0.0` and `outliers` is `[]`.
@@starter
import statistics


def summary(values):
    pass


print(summary([120, 135, 128, 140, 131, 125, 980, 133]))
@@expected
{'count': 8, 'mean': 236.5, 'median': 132.0, 'stdev': 300.48, 'min': 120, 'max': 980, 'outliers': [980]}
@@tests
check(summary([5]), {"count": 1, "mean": 5, "median": 5, "stdev": 0.0, "min": 5, "max": 5, "outliers": []}, "one value")
check(summary([10, 11, 12, 13, 14, 15, 100, -50])["outliers"], [100, -50], "both sides, original order")
check(summary([10, 10, 10, 10])["outliers"], [], "no spread")
@@hint
`q1, _, q3 = statistics.quantiles(values, n=4)`; `iqr = q3 - q1`; an outlier is `v < q1 - 1.5 * iqr or v > q3 + 1.5 * iqr`.
@@solution
import statistics


def summary(values):
    result = {
        "count": len(values),
        "mean": round(statistics.mean(values), 2),
        "median": statistics.median(values),
        "stdev": 0.0,
        "min": min(values),
        "max": max(values),
        "outliers": [],
    }
    if len(values) >= 2:
        result["stdev"] = round(statistics.stdev(values), 2)
        q1, _, q3 = statistics.quantiles(values, n=4)
        iqr = q3 - q1
        low, high = q1 - 1.5 * iqr, q3 + 1.5 * iqr
        result["outliers"] = [v for v in values if v < low or v > high]
    return result


print(summary([120, 135, 128, 140, 131, 125, 980, 133]))

@@exercise Group by and pivot
Write `group_stats(csv_text, by, value)`: parse the CSV (with a header row), convert `value` to float, and return a dict mapping each group to `{"count": n, "mean": m}` (mean rounded to 1 decimal), groups sorted by name. Skip rows where `value` is empty or not a number.

Then `pivot(csv_text, rows, cols, value)`: a dict of dicts with the **sum** of `value` for each (row, column) pair, e.g. `{"net": {"high": 90.0, "low": 15.0}}`; both levels sorted by name; missing pairs are left out.
@@starter
import csv
import io


def group_stats(csv_text, by, value):
    pass


def pivot(csv_text, rows, cols, value):
    pass


data = """team,priority,minutes
net,high,60
desk,low,10
net,low,15
desk,low,
net,high,30
desk,high,45
"""
print(group_stats(data, "team", "minutes"))
print(pivot(data, "team", "priority", "minutes"))
@@expected
{'desk': {'count': 2, 'mean': 27.5}, 'net': {'count': 3, 'mean': 35.0}}
{'desk': {'high': 45.0, 'low': 10.0}, 'net': {'high': 90.0, 'low': 15.0}}
@@tests
check(group_stats("a,b\nx,1\nx,oops\ny,2.5\n", "a", "b"), {"x": {"count": 1, "mean": 1.0}, "y": {"count": 1, "mean": 2.5}}, "bad values skipped")
check(group_stats("a,b\n", "a", "b"), {}, "no rows")
check(list(pivot("r,c,v\nz,b,1\na,b,2\na,a,3\n", "r", "c", "v")["a"]), ["a", "b"], "inner keys sorted")
@@hint
Read rows with `csv.DictReader(io.StringIO(csv_text))`. Write a helper `_numbers(csv_text, value)` that yields `(row, number)` for valid rows using `try: float(row[value]) except ValueError: continue`. Collect lists with `collections.defaultdict(list)`.
@@solution
import csv
import io
from collections import defaultdict


def _numbers(csv_text, value):
    for row in csv.DictReader(io.StringIO(csv_text)):
        try:
            yield row, float(row[value])
        except (ValueError, TypeError):
            continue


def group_stats(csv_text, by, value):
    groups = defaultdict(list)
    for row, number in _numbers(csv_text, value):
        groups[row[by]].append(number)
    return {
        name: {"count": len(nums), "mean": round(sum(nums) / len(nums), 1)}
        for name, nums in sorted(groups.items())
    }


def pivot(csv_text, rows, cols, value):
    table = defaultdict(lambda: defaultdict(float))
    for row, number in _numbers(csv_text, value):
        table[row[rows]][row[cols]] += number
    return {r: dict(sorted(inner.items())) for r, inner in sorted(table.items())}


data = """team,priority,minutes
net,high,60
desk,low,10
net,low,15
desk,low,
net,high,30
desk,high,45
"""
print(group_stats(data, "team", "minutes"))
print(pivot(data, "team", "priority", "minutes"))
