@@lesson g9-project Project: Product Analytics with SQL (Funnels, Cohorts and Retention)
@@concepts lists, maps
@@text
Your company launched an internal self-service IT portal. Management asks the questions every product team asks:

1. **Funnel**: of the people who opened the portal, how many searched the knowledge base, opened a ticket form, and submitted a ticket?
2. **Cohorts and retention**: of the people who first used the portal in a given week, how many came back in later weeks?
3. **Engagement**: who are the power users, and how does daily activity trend?

All three are answered from one **event table**, the most common data shape in product analytics (web analytics, app telemetry, audit logs):

```text
events(user_id, event, at)       -- 'visit', 'search', 'form_open', 'submit', ...
```

## Funnels

A funnel counts users who reached each step **in order**. The simple version counts distinct users per event; a stricter version requires each step to happen **after** the previous one, which needs the time of each user's first occurrence of each step:

```text
first_visit  = MIN(at) WHERE event = 'visit'
first_search = MIN(at) WHERE event = 'search' AND at >= first_visit
...
```

Report counts and **conversion** from the previous step and from the top, which point at the step losing the most people.

## Cohorts and retention

A **cohort** is a group of users who started in the same period (here: the week of their first visit). **Retention** for week N is the share of the cohort active in week N after their start. The classic output is a triangle:

```text
cohort     size  w0    w1   w2
2026-W40     4  100%  50%  25%
2026-W41     3  100%  67%
```

Steps: find each user's first week; find every (user, active week) pair; compute `week_number = active_week − first_week`; count distinct users per cohort and week number; divide by cohort size.

## A note on weeks

Using `strftime('%W')` week numbers breaks across year boundaries (week 52 → week 0). A robust "week index" is the number of days since a fixed Monday divided by 7: `CAST((julianday(day) - julianday('2026-01-05')) / 7 AS INTEGER)`.

## Engagement

DAU/WAU/MAU (daily, weekly and monthly active users) and their ratio (DAU/MAU, "stickiness") summarise engagement. Power users: rank by active days, not by raw event counts (one user clicking 500 times in a day is not a power user).

The exercises use a small events table (included in each exercise) so you can check every number by hand.

@@quiz
What does a strict funnel require that a simple distinct-user count doesn't?
* Each step must happen after the previous step for the same user
- More users
- A date dimension
- A window function
? Otherwise users who searched before ever visiting would count.

@@quiz
What is a cohort in retention analysis?
- All users
* Users grouped by the period in which they started
- The most active users
- A type of index
? Retention compares each cohort with itself over time.

@@quiz
Why rank power users by active days rather than event count?
- Event counts are always zero
* A burst of clicks in one day says less about habitual use than activity on many days
- Days are easier to count
- SQL can't count events
? Engagement is about coming back.

@@drill
What's the name of a group of users who signed up in the same period?
= cohort
= a cohort
? Retention is tracked per cohort.

@@flashcards
event table :: user, event, timestamp
funnel :: users reaching each step in order
conversion :: share of users moving to the next step
cohort :: users who started in the same period
retention :: share of a cohort active N periods later
DAU / WAU / MAU :: daily / weekly / monthly active users
stickiness :: DAU / MAU
week index :: days since a fixed Monday / 7

@@exercise A strict funnel
For the portal events, show each funnel step (`visit`, `search`, `form_open`, `submit`), the number of `users` who reached it **in order** (each step's first occurrence at or after the previous step's), the `pct_of_prev` and `pct_of_top` (whole numbers). Keep the step order.
@@setup
CREATE TABLE events (user_id INTEGER, event TEXT, at TEXT);
INSERT INTO events VALUES
 (1,'visit','2026-10-05 09:00'),(1,'search','2026-10-05 09:01'),(1,'form_open','2026-10-05 09:03'),(1,'submit','2026-10-05 09:06'),
 (2,'visit','2026-10-05 10:00'),(2,'search','2026-10-05 10:02'),(2,'form_open','2026-10-05 10:05'),
 (3,'search','2026-10-06 08:00'),(3,'visit','2026-10-06 08:05'),(3,'form_open','2026-10-06 08:07'),(3,'submit','2026-10-06 08:09'),
 (4,'visit','2026-10-07 13:00'),(4,'search','2026-10-07 13:01'),
 (5,'visit','2026-10-12 09:00'),(5,'search','2026-10-12 09:02'),(5,'form_open','2026-10-12 09:04'),(5,'submit','2026-10-12 09:08'),
 (6,'visit','2026-10-13 11:00'),
 (7,'visit','2026-10-14 15:00'),(7,'search','2026-10-14 15:01'),(7,'submit','2026-10-14 15:02'),
 (1,'visit','2026-10-13 09:00'),(1,'search','2026-10-13 09:05'),
 (2,'visit','2026-10-20 10:00'),(5,'visit','2026-10-20 09:00'),(1,'visit','2026-10-21 09:00');
@@starter
SELECT event, COUNT(DISTINCT user_id) AS users FROM events GROUP BY event;
@@expected
step      | users | pct_of_prev | pct_of_top
----------+-------+-------------+-----------
visit     | 7     | 100         | 100
search    | 5     | 71          | 71
form_open | 3     | 60          | 42
submit    | 2     | 66          | 28
@@requires
MIN(
WITH
@@hint
Build per-user first times step by step: `v AS (SELECT user_id, MIN(at) AS t FROM events WHERE event = 'visit' GROUP BY user_id)`, `s AS (SELECT e.user_id, MIN(e.at) AS t FROM events e JOIN v ON v.user_id = e.user_id AND e.at >= v.t WHERE e.event = 'search' GROUP BY e.user_id)`, and the same for form_open (joined to s) and submit (joined to f). Then UNION ALL the four counts with a step number, and use `LAG(users) OVER (ORDER BY n)` and `FIRST_VALUE(users) OVER (ORDER BY n)` for the percentages.
@@solution
WITH v AS (
    SELECT user_id, MIN(at) AS t FROM events WHERE event = 'visit' GROUP BY user_id
),
s AS (
    SELECT e.user_id, MIN(e.at) AS t FROM events e JOIN v ON v.user_id = e.user_id AND e.at >= v.t
    WHERE e.event = 'search' GROUP BY e.user_id
),
f AS (
    SELECT e.user_id, MIN(e.at) AS t FROM events e JOIN s ON s.user_id = e.user_id AND e.at >= s.t
    WHERE e.event = 'form_open' GROUP BY e.user_id
),
sub AS (
    SELECT e.user_id, MIN(e.at) AS t FROM events e JOIN f ON f.user_id = e.user_id AND e.at >= f.t
    WHERE e.event = 'submit' GROUP BY e.user_id
),
steps(n, step, users) AS (
    SELECT 1, 'visit', COUNT(*) FROM v
    UNION ALL SELECT 2, 'search', COUNT(*) FROM s
    UNION ALL SELECT 3, 'form_open', COUNT(*) FROM f
    UNION ALL SELECT 4, 'submit', COUNT(*) FROM sub
)
SELECT step, users,
       100 * users / COALESCE(LAG(users) OVER (ORDER BY n), users) AS pct_of_prev,
       100 * users / FIRST_VALUE(users) OVER (ORDER BY n) AS pct_of_top
FROM steps
ORDER BY n;

@@exercise A retention triangle
Using the same events, define each user's **week index** as `CAST((julianday(date(at)) - julianday('2026-10-05')) / 7 AS INTEGER)` (week 0 starts Monday 5 October). Each user's cohort is the week of their first event. Show `cohort` (week index), `size`, and the retention percentages `w0`, `w1`, `w2` (whole numbers; NULL when that week is in the future: the data ends in week 2). Sort by cohort.
@@setup
CREATE TABLE events (user_id INTEGER, event TEXT, at TEXT);
INSERT INTO events VALUES
 (1,'visit','2026-10-05 09:00'),(1,'search','2026-10-05 09:01'),(1,'form_open','2026-10-05 09:03'),(1,'submit','2026-10-05 09:06'),
 (2,'visit','2026-10-05 10:00'),(2,'search','2026-10-05 10:02'),(2,'form_open','2026-10-05 10:05'),
 (3,'search','2026-10-06 08:00'),(3,'visit','2026-10-06 08:05'),(3,'form_open','2026-10-06 08:07'),(3,'submit','2026-10-06 08:09'),
 (4,'visit','2026-10-07 13:00'),(4,'search','2026-10-07 13:01'),
 (5,'visit','2026-10-12 09:00'),(5,'search','2026-10-12 09:02'),(5,'form_open','2026-10-12 09:04'),(5,'submit','2026-10-12 09:08'),
 (6,'visit','2026-10-13 11:00'),
 (7,'visit','2026-10-14 15:00'),(7,'search','2026-10-14 15:01'),(7,'submit','2026-10-14 15:02'),
 (1,'visit','2026-10-13 09:00'),(1,'search','2026-10-13 09:05'),
 (2,'visit','2026-10-20 10:00'),(5,'visit','2026-10-20 09:00'),(1,'visit','2026-10-21 09:00');
@@starter
SELECT user_id, MIN(at) AS first_seen FROM events GROUP BY user_id ORDER BY user_id;
@@expected
cohort | size | w0  | w1 | w2
-------+------+-----+----+-----
0      | 4    | 100 | 25 | 50
1      | 3    | 100 | 33 | NULL
@@requires
COUNT(DISTINCT
julianday
@@hint
CTE `act` = distinct `(user_id, wk)`; CTE `coh` = `user_id, MIN(wk) AS cohort`; join them and compute `wk - cohort AS n`. Then per cohort: `COUNT(DISTINCT CASE WHEN n = 1 THEN user_id END)` etc. The size is the n = 0 count. Weeks in the future: `CASE WHEN cohort + 1 > 2 THEN NULL ELSE ... END` (2 = the last week with data).
@@solution
WITH act AS (
    SELECT DISTINCT user_id, CAST((julianday(date(at)) - julianday('2026-10-05')) / 7 AS INTEGER) AS wk FROM events
),
coh AS (
    SELECT user_id, MIN(wk) AS cohort FROM act GROUP BY user_id
),
grid AS (
    SELECT c.cohort, a.user_id, a.wk - c.cohort AS n FROM act a JOIN coh c ON c.user_id = a.user_id
),
counts AS (
    SELECT cohort,
           COUNT(DISTINCT CASE WHEN n = 0 THEN user_id END) AS size,
           COUNT(DISTINCT CASE WHEN n = 1 THEN user_id END) AS c1,
           COUNT(DISTINCT CASE WHEN n = 2 THEN user_id END) AS c2
    FROM grid GROUP BY cohort
)
SELECT cohort, size,
       100 AS w0,
       CASE WHEN cohort + 1 > 2 THEN NULL ELSE 100 * c1 / size END AS w1,
       CASE WHEN cohort + 2 > 2 THEN NULL ELSE 100 * c2 / size END AS w2
FROM counts
ORDER BY cohort;

@@exercise Power users
Rank users by **active days** (distinct dates with any event), then by total events, then by user id. Show `user_id`, `active_days`, `events`, `first_seen` (date), `last_seen` (date) and `rank` (`DENSE_RANK` on active days only). Only the top 4 rows.
@@setup
CREATE TABLE events (user_id INTEGER, event TEXT, at TEXT);
INSERT INTO events VALUES
 (1,'visit','2026-10-05 09:00'),(1,'search','2026-10-05 09:01'),(1,'form_open','2026-10-05 09:03'),(1,'submit','2026-10-05 09:06'),
 (2,'visit','2026-10-05 10:00'),(2,'search','2026-10-05 10:02'),(2,'form_open','2026-10-05 10:05'),
 (3,'search','2026-10-06 08:00'),(3,'visit','2026-10-06 08:05'),(3,'form_open','2026-10-06 08:07'),(3,'submit','2026-10-06 08:09'),
 (4,'visit','2026-10-07 13:00'),(4,'search','2026-10-07 13:01'),
 (5,'visit','2026-10-12 09:00'),(5,'search','2026-10-12 09:02'),(5,'form_open','2026-10-12 09:04'),(5,'submit','2026-10-12 09:08'),
 (6,'visit','2026-10-13 11:00'),
 (7,'visit','2026-10-14 15:00'),(7,'search','2026-10-14 15:01'),(7,'submit','2026-10-14 15:02'),
 (1,'visit','2026-10-13 09:00'),(1,'search','2026-10-13 09:05'),
 (2,'visit','2026-10-20 10:00'),(5,'visit','2026-10-20 09:00'),(1,'visit','2026-10-21 09:00');
@@starter
SELECT user_id, COUNT(*) AS events FROM events GROUP BY user_id ORDER BY events DESC LIMIT 4;
@@expected
user_id | active_days | events | first_seen | last_seen  | rank
--------+-------------+--------+------------+------------+-----
1       | 3           | 7      | 2026-10-05 | 2026-10-21 | 1
5       | 2           | 5      | 2026-10-12 | 2026-10-20 | 2
2       | 2           | 4      | 2026-10-05 | 2026-10-20 | 2
3       | 1           | 4      | 2026-10-06 | 2026-10-06 | 3
@@requires
COUNT(DISTINCT date(at))
DENSE_RANK()
@@hint
`GROUP BY user_id` with `COUNT(DISTINCT date(at)) AS active_days`, `COUNT(*) AS events`, `date(MIN(at))`, `date(MAX(at))`, `DENSE_RANK() OVER (ORDER BY COUNT(DISTINCT date(at)) DESC) AS rank`; order by active_days DESC, events DESC, user_id; LIMIT 4.
@@solution
SELECT user_id,
       COUNT(DISTINCT date(at)) AS active_days,
       COUNT(*) AS events,
       date(MIN(at)) AS first_seen,
       date(MAX(at)) AS last_seen,
       DENSE_RANK() OVER (ORDER BY COUNT(DISTINCT date(at)) DESC) AS rank
FROM events
GROUP BY user_id
ORDER BY active_days DESC, events DESC, user_id
LIMIT 4;
