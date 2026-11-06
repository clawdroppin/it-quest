@@lesson b10-stats 🎓 Beyond IT: Statistics in SQL: Medians, Percentiles and Regression
@@concepts numbers
@@text
> **Beyond IT scope.** Statistical analysis belongs to data science and research. IT people do meet percentiles (p95 response time) and averages every day, which is a good reason to understand them properly, but regression and distributions go beyond the job.

## Mean, median and why both matter

The **mean** (`AVG`) is pulled by extreme values; the **median** (the middle value) isn't. Ticket resolution times are a classic skewed distribution: most tickets close in minutes, one waits a week. In our help desk data the mean is about 16 hours, while the median is under 2 hours (you'll compute both in the exercise). Report the median (or percentiles) for anything skewed: response times, salaries, file sizes.

SQLite has no `MEDIAN`; PostgreSQL has `percentile_cont(0.5) WITHIN GROUP (ORDER BY x)`. Portable version with window functions:

```sql
WITH r AS (
    SELECT salary, ROW_NUMBER() OVER (ORDER BY salary) AS rn, COUNT(*) OVER () AS n FROM employees
)
SELECT AVG(salary) AS median_salary FROM r WHERE rn IN ((n + 1) / 2, (n + 2) / 2);
```

For an odd n, both expressions give the middle row; for an even n, the two middle rows, averaged.

## Percentiles

The **p-th percentile** is the value below which p% of observations fall. Monitoring uses p50, p95 and p99 latency: "95% of requests finish within 300 ms". There are several definitions (nearest rank, linear interpolation); PostgreSQL's `percentile_disc` uses nearest rank, `percentile_cont` interpolates.

Nearest rank: the value at position `ceil(p × n)` in sorted order. Since SQLite's `ceil` needs the math functions, use `CAST(p * n AS INTEGER) + (p * n > CAST(p * n AS INTEGER))`.

## Spread: variance and standard deviation

Variance is the average squared distance from the mean: `AVG(x * x) - AVG(x) * AVG(x)` (the population variance; multiply by n / (n − 1) for the sample variance). The standard deviation is its square root (`sqrt()`, available in SQLite builds with math functions, `stddev_samp` in PostgreSQL). A **z-score** `(x − mean) / sd` says how unusual a value is; |z| > 3 is a common outlier rule, for example for "this server's CPU is unusually high".

## Histograms

Bucket values with integer division (`CAST(x / 100 AS INTEGER) * 100`) or `NTILE`, count per bucket, and draw bars with `substr('##########...', 1, count)`. A histogram shows the shape (skew, two humps) that averages hide.

## Linear regression

The least-squares line y = a + b·x through points (x, y):

```text
b = (n·Σxy − Σx·Σy) / (n·Σx² − (Σx)²)
a = (Σy − b·Σx) / n
r² = (n·Σxy − Σx·Σy)² / ((n·Σx² − (Σx)²) · (n·Σy² − (Σy)²))
```

All of it is plain aggregates. PostgreSQL has them built in: `regr_slope(y, x)`, `regr_intercept`, `regr_r2`, `corr`. `r²` (0..1) says how much of the variation in y the line explains. Correlation is not causation: tickets and ice cream sales both rise in summer.

Watch for integer division in these formulas: multiply by `1.0` first.

@@quiz
Resolution times are mostly a few minutes, with a few multi-day tickets. Which statistic best describes a "typical" ticket?
- The mean
* The median
- The maximum
- The sum
? Skewed data drags the mean towards the tail.

@@quiz
What does "p95 latency = 300 ms" mean?
* 95% of requests completed in 300 ms or less
- The average is 300 ms
- 5% of requests took 300 ms
- The slowest request took 300 ms
? Percentiles describe the distribution's tail.

@@quiz
What does r² = 0.9 tell you about a regression line?
- The slope is 0.9
* The line explains about 90% of the variation in y
- 90% of points are on the line
- x causes y
? It measures fit, not causation.

@@drill
Which statistic is the middle value when the values are sorted?
= median
? Less affected by outliers than the mean.

@@flashcards
median :: middle value; robust to outliers
`percentile_cont(0.5) WITHIN GROUP (ORDER BY x)` :: PostgreSQL median
p95 :: 95% of values are at or below it
population variance :: AVG(x²) − AVG(x)²
z-score :: (x − mean) / sd
histogram :: counts per bucket
slope b :: (nΣxy − ΣxΣy) / (nΣx² − (Σx)²)
r² :: share of variation explained
`regr_slope(y, x)` :: PostgreSQL regression slope

@@exercise Mean vs median
For closed and resolved tickets (those with a `closed` time), compute the resolution time in minutes and show `tickets`, `mean_min` (whole minutes), `median_min` (the median, averaged when there are two middle values, rounded to a whole number) and `max_min`.
@@starter
SELECT AVG((julianday(closed) - julianday(opened)) * 1440) FROM tickets WHERE closed IS NOT NULL;
@@expected
tickets | mean_min | median_min | max_min
--------+----------+------------+--------
8       | 989      | 101        | 4800
@@requires
ROW_NUMBER()
COUNT(*) OVER ()
@@hint
CTE `m` with the minutes (`CAST(ROUND(...) AS INTEGER)`), CTE `r` adding `ROW_NUMBER() OVER (ORDER BY minutes)` and `COUNT(*) OVER ()`. Median: `AVG(minutes) FILTER (WHERE rn IN ((n + 1) / 2, (n + 2) / 2))`, or a scalar subquery.
@@solution
WITH m AS (
    SELECT CAST(ROUND((julianday(closed) - julianday(opened)) * 1440) AS INTEGER) AS minutes
    FROM tickets WHERE closed IS NOT NULL
),
r AS (
    SELECT minutes, ROW_NUMBER() OVER (ORDER BY minutes) AS rn, COUNT(*) OVER () AS n FROM m
)
SELECT COUNT(*) AS tickets,
       CAST(ROUND(AVG(minutes)) AS INTEGER) AS mean_min,
       (SELECT CAST(ROUND(AVG(minutes)) AS INTEGER) FROM r WHERE rn IN ((n + 1) / 2, (n + 2) / 2)) AS median_min,
       MAX(minutes) AS max_min
FROM r;

@@exercise Latency percentiles and a histogram
`requests(ms)` holds 40 response times. Show, in one row, `p50`, `p90`, `p95` and `p99` using **nearest rank** (the value at position `ceil(p × n)` in ascending order). Then show a histogram in 100 ms buckets: `bucket` (like `0-99`), `count` and a `bar` of `#` characters, one per request.
@@setup
CREATE TABLE requests (ms INTEGER);
INSERT INTO requests VALUES
 (42),(51),(55),(58),(61),(63),(66),(70),(72),(75),(78),(80),(84),(88),(91),(95),(99),(104),(110),(118),
 (121),(126),(133),(140),(152),(160),(171),(185),(196),(210),(228),(245),(260),(290),(310),(355),(420),(515),(780),(1240);
@@starter
SELECT AVG(ms) FROM requests;
@@expected
p50 | p90 | p95 | p99
----+-----+-----+-----
118 | 355 | 515 | 1240

bucket    | count | bar
----------+-------+------------------
0-99      | 17    | #################
100-199   | 12    | ############
200-299   | 5     | #####
300-399   | 2     | ##
400-499   | 1     | #
500-599   | 1     | #
700-799   | 1     | #
1200-1299 | 1     | #
@@requires
ROW_NUMBER()
GROUP BY
@@hint
`r AS (SELECT ms, ROW_NUMBER() OVER (ORDER BY ms) AS rn, COUNT(*) OVER () AS n FROM requests)`. A percentile: `(SELECT ms FROM r WHERE rn = CAST(0.95 * n AS INTEGER) + (0.95 * n > CAST(0.95 * n AS INTEGER)))`. For p50 with n = 40, that's position 20. Bucket: `CAST(ms / 100 AS INTEGER) * 100`; bar: `substr('########################################', 1, COUNT(*))`.
@@solution
WITH r AS (
    SELECT ms, ROW_NUMBER() OVER (ORDER BY ms) AS rn, COUNT(*) OVER () AS n FROM requests
)
SELECT (SELECT ms FROM r WHERE rn = CAST(0.50 * n AS INTEGER) + (0.50 * n > CAST(0.50 * n AS INTEGER))) AS p50,
       (SELECT ms FROM r WHERE rn = CAST(0.90 * n AS INTEGER) + (0.90 * n > CAST(0.90 * n AS INTEGER))) AS p90,
       (SELECT ms FROM r WHERE rn = CAST(0.95 * n AS INTEGER) + (0.95 * n > CAST(0.95 * n AS INTEGER))) AS p95,
       (SELECT ms FROM r WHERE rn = CAST(0.99 * n AS INTEGER) + (0.99 * n > CAST(0.99 * n AS INTEGER))) AS p99
FROM r LIMIT 1;

SELECT b || '-' || (b + 99) AS bucket, COUNT(*) AS count,
       substr('########################################', 1, COUNT(*)) AS bar
FROM (SELECT CAST(ms / 100 AS INTEGER) * 100 AS b FROM requests)
GROUP BY b
ORDER BY b;

@@exercise Does disk usage predict tickets?
`servers(name, disk_pct, tickets)` has each server's disk usage and the tickets it caused last quarter. Fit the least-squares line `tickets = a + b · disk_pct` with aggregates and show `n`, `slope` (3 decimals), `intercept` (2 decimals), `r2` (3 decimals) and the `predicted` tickets for a server at 95% (1 decimal).
@@setup
CREATE TABLE servers (name TEXT, disk_pct INTEGER, tickets INTEGER);
INSERT INTO servers VALUES ('SV-01', 35, 1), ('SV-02', 48, 2), ('SV-03', 52, 1), ('SV-04', 60, 3), ('SV-05', 71, 4),
  ('SV-06', 77, 4), ('SV-07', 83, 6), ('SV-08', 88, 5), ('SV-09', 91, 8), ('SV-10', 97, 9);
@@starter
SELECT AVG(disk_pct), AVG(tickets) FROM servers;
@@expected
n  | slope | intercept | r2    | predicted
---+-------+-----------+-------+----------
10 | 0.124 | -4.38     | 0.863 | 7.4
@@requires
SUM(
1.0
@@hint
CTE `s` with `COUNT(*) AS n, SUM(disk_pct) * 1.0 AS sx, SUM(tickets) * 1.0 AS sy, SUM(disk_pct * tickets) * 1.0 AS sxy, SUM(disk_pct * disk_pct) * 1.0 AS sxx, SUM(tickets * tickets) * 1.0 AS syy`. CTE `fit` computes `b` and `a` from the formulas; r² from the third formula. Round at the end.
@@solution
WITH s AS (
    SELECT COUNT(*) AS n, SUM(disk_pct) * 1.0 AS sx, SUM(tickets) * 1.0 AS sy,
           SUM(disk_pct * tickets) * 1.0 AS sxy, SUM(disk_pct * disk_pct) * 1.0 AS sxx, SUM(tickets * tickets) * 1.0 AS syy
    FROM servers
),
fit AS (
    SELECT n, (n * sxy - sx * sy) / (n * sxx - sx * sx) AS b,
           (n * sxy - sx * sy) * (n * sxy - sx * sy) / ((n * sxx - sx * sx) * (n * syy - sy * sy)) AS r2,
           sx, sy
    FROM s
)
SELECT n, ROUND(b, 3) AS slope, ROUND((sy - b * sx) / n, 2) AS intercept, ROUND(r2, 3) AS r2,
       ROUND((sy - b * sx) / n + b * 95, 1) AS predicted
FROM fit;
