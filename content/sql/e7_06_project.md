@@lesson e7-project Project: A Help Desk SLA Report
@@concepts lists, automation
@@text
Time to combine everything from this module into the kind of report an IT manager actually asks for:

> "For last month: how many tickets did we get per priority, how fast did we respond and resolve them, which ones broke the SLA, and who's carrying the most load?"

## The SLA

Sakura Systems' help desk promises:

| Priority | First response within | Resolution within |
|---|---|---|
| 1 (critical) | 30 minutes | 4 hours |
| 2 (high) | 2 hours | 1 day |
| 3 (normal) | 4 hours | 3 days |
| 4 (low) | 1 business day | 5 days |

We'll keep the targets in a **table**, not in CASE expressions. When the SLA changes, you update rows, not every report.

## Building a report in layers

A good large query reads like a pipeline of small, named steps. Each CTE does one job, and you can test each one by selecting from it:

```text
sla            -- targets per priority
events         -- first response time per ticket (from the ticket_events table)
measured       -- tickets + minutes to respond and resolve
judged         -- measured + met/breached flags
summary        -- aggregates per priority
```

Tips that apply to every report:

- **Decide the population first**: which tickets count? Opened in the period? Closed in the period? Exclude test tickets? Write it down and put it in exactly one CTE.
- **Handle open items explicitly**: an open ticket has no resolution time yet, but if it's already older than its target, it has *already* breached. Measure open tickets against a fixed "as of" time.
- **Name your units**: `_min`, `_hours` in column names prevent mixing minutes with hours.
- **Check totals**: the per-priority counts should add up to the population count. Put a quick check query next to the report while developing.

## The data

The exercises add a `ticket_events` table to the company database: each ticket's events (`created`, `responded`, `assigned`, `resolved`) with a timestamp. The as-of time for open tickets is `2026-11-01 00:00`.

```sql,norun
CREATE TABLE ticket_events (ticket_id INTEGER, kind TEXT, at TEXT);
```

@@quiz
Why keep SLA targets in a table rather than in CASE expressions?
* The targets become data: change them in one place, join them anywhere, and report on them
- CASE is slower
- Tables are required for CTEs
- CASE can't return numbers
? Rules that change belong in data.

@@quiz
How should an SLA report treat a ticket that's still open?
- Ignore it
- Count it as met
* Measure it against an "as of" time: if it's already past the target, it has breached
- Count it as breached always
? Old open tickets are often the worst breaches.

@@quiz
Which join keeps tickets that have no matching SLA target, so they show up as a problem?
* LEFT JOIN from tickets to targets
- INNER JOIN
- CROSS JOIN
- A UNION
? Rows with a NULL target reveal missing configuration.

@@flashcards
population :: the set of rows a report counts
as-of time :: fixed "now" for measuring open items
layered CTEs :: one job per step, testable
SLA target table :: rules as data
breach :: actual time > target time

@@exercise Response and resolution times
Using the company tickets and the `ticket_events` table in the starter, show for each ticket: `id`, `priority`, `respond_min` (minutes from `opened` to the **first** `responded` event, NULL if none) and `resolve_min` (minutes from `opened` to `closed`, or to the as-of time `2026-11-01 00:00` for tickets that aren't closed). Only tickets opened from `2026-10-20`. Sort by id.
@@starter
CREATE TABLE ticket_events (ticket_id INTEGER, kind TEXT, at TEXT);
INSERT INTO ticket_events VALUES
  (7, 'responded', '2026-10-20 11:40'), (8, 'responded', '2026-10-21 09:33'),
  (9, 'responded', '2026-10-24 08:05'), (10, 'responded', '2026-10-28 16:00'),
  (11, 'responded', '2026-10-29 23:55'), (11, 'responded', '2026-10-30 08:00'),
  (13, 'responded', '2026-10-31 10:00'), (14, 'responded', '2026-10-31 08:07');

SELECT id, priority FROM tickets WHERE opened >= '2026-10-20' ORDER BY id;
@@expected
id | priority | respond_min | resolve_min
---+----------+-------------+------------
7  | 2        | 25          | 16605
8  | 4        | 3           | 11
9  | 1        | 15          | 11050
10 | 3        | 1495        | 6295
11 | 1        | 75          | 2960
12 | 4        | NULL        | 2280
13 | 4        | 1060        | 1900
14 | 3        | 2           | 15
15 | 3        | NULL        | 890
@@requires
MIN(
COALESCE(
@@hint
First response per ticket: `SELECT ticket_id, MIN(at) AS first_at FROM ticket_events WHERE kind = 'responded' GROUP BY ticket_id` as a CTE, LEFT JOIN it. Minutes: `CAST(ROUND((julianday(x) - julianday(t.opened)) * 1440) AS INTEGER)`, with `COALESCE(t.closed, '2026-11-01 00:00')` for resolution.
@@solution
CREATE TABLE ticket_events (ticket_id INTEGER, kind TEXT, at TEXT);
INSERT INTO ticket_events VALUES
  (7, 'responded', '2026-10-20 11:40'), (8, 'responded', '2026-10-21 09:33'),
  (9, 'responded', '2026-10-24 08:05'), (10, 'responded', '2026-10-28 16:00'),
  (11, 'responded', '2026-10-29 23:55'), (11, 'responded', '2026-10-30 08:00'),
  (13, 'responded', '2026-10-31 10:00'), (14, 'responded', '2026-10-31 08:07');

WITH first_response AS (
    SELECT ticket_id, MIN(at) AS first_at FROM ticket_events WHERE kind = 'responded' GROUP BY ticket_id
)
SELECT t.id, t.priority,
       CAST(ROUND((julianday(f.first_at) - julianday(t.opened)) * 1440) AS INTEGER) AS respond_min,
       CAST(ROUND((julianday(COALESCE(t.closed, '2026-11-01 00:00')) - julianday(t.opened)) * 1440) AS INTEGER) AS resolve_min
FROM tickets t
LEFT JOIN first_response f ON f.ticket_id = t.id
WHERE t.opened >= '2026-10-20'
ORDER BY t.id;

@@exercise The SLA report
Build the full report for **all** tickets. Put the targets in an `sla` table (minutes; treat "1 business day" as 1440 minutes). A ticket **breaches** if it has no response yet and is past the response target at the as-of time, if its response came late, or if its resolution time (closed, or as-of for open tickets) is past the resolution target. Show per `priority`: `tickets`, `breached`, `pct_met` (whole-number percentage of tickets not breached) and `worst_ticket` (the id of the breached ticket that overran its resolution target by the most minutes, or `-`). Sort by priority.
@@starter
CREATE TABLE ticket_events (ticket_id INTEGER, kind TEXT, at TEXT);
INSERT INTO ticket_events VALUES
  (1, 'responded', '2026-10-01 09:20'), (2, 'responded', '2026-10-03 15:10'),
  (3, 'responded', '2026-10-06 08:50'), (4, 'responded', '2026-10-08 13:30'),
  (5, 'responded', '2026-10-10 10:10'), (6, 'responded', '2026-10-12 15:30'),
  (7, 'responded', '2026-10-20 11:40'), (8, 'responded', '2026-10-21 09:33'),
  (9, 'responded', '2026-10-24 08:05'), (10, 'responded', '2026-10-28 16:00'),
  (11, 'responded', '2026-10-29 23:55'), (13, 'responded', '2026-10-31 10:00'),
  (14, 'responded', '2026-10-31 08:07');
-- CREATE TABLE sla (...);
SELECT priority, COUNT(*) AS tickets FROM tickets GROUP BY priority ORDER BY priority;
@@expected
priority | tickets | breached | pct_met | worst_ticket
---------+---------+----------+---------+-------------
1        | 3       | 2        | 33      | 9
2        | 3       | 1        | 66      | 7
3        | 6       | 3        | 50      | 10
4        | 3       | 1        | 66      | 12
@@requires
CREATE TABLE sla
WITH
JOIN sla
ROW_NUMBER()
@@hint
Layers: `first_response` (MIN responded per ticket), `measured` (tickets joined to first_response and sla, with `respond_min` = minutes to first response or, if none, minutes to the as-of time; `resolve_min` = minutes to COALESCE(closed, as-of)), `judged` (adds `breached` = `respond_min > respond_target OR resolve_min > resolve_target`, and `overrun` = `resolve_min - resolve_target`), then the summary. For worst_ticket, rank breached tickets per priority with `ROW_NUMBER() OVER (PARTITION BY priority ORDER BY overrun DESC, id)` in another CTE and LEFT JOIN rank 1.
@@solution
CREATE TABLE ticket_events (ticket_id INTEGER, kind TEXT, at TEXT);
INSERT INTO ticket_events VALUES
  (1, 'responded', '2026-10-01 09:20'), (2, 'responded', '2026-10-03 15:10'),
  (3, 'responded', '2026-10-06 08:50'), (4, 'responded', '2026-10-08 13:30'),
  (5, 'responded', '2026-10-10 10:10'), (6, 'responded', '2026-10-12 15:30'),
  (7, 'responded', '2026-10-20 11:40'), (8, 'responded', '2026-10-21 09:33'),
  (9, 'responded', '2026-10-24 08:05'), (10, 'responded', '2026-10-28 16:00'),
  (11, 'responded', '2026-10-29 23:55'), (13, 'responded', '2026-10-31 10:00'),
  (14, 'responded', '2026-10-31 08:07');
CREATE TABLE sla (priority INTEGER PRIMARY KEY, respond_target INTEGER, resolve_target INTEGER);
INSERT INTO sla VALUES (1, 30, 240), (2, 120, 1440), (3, 240, 4320), (4, 1440, 7200);

WITH first_response AS (
    SELECT ticket_id, MIN(at) AS first_at FROM ticket_events WHERE kind = 'responded' GROUP BY ticket_id
),
measured AS (
    SELECT t.id, t.priority, s.respond_target, s.resolve_target,
           CAST(ROUND((julianday(COALESCE(f.first_at, '2026-11-01 00:00')) - julianday(t.opened)) * 1440) AS INTEGER) AS respond_min,
           CAST(ROUND((julianday(COALESCE(t.closed, '2026-11-01 00:00')) - julianday(t.opened)) * 1440) AS INTEGER) AS resolve_min
    FROM tickets t
    JOIN sla s ON s.priority = t.priority
    LEFT JOIN first_response f ON f.ticket_id = t.id
),
judged AS (
    SELECT *, (respond_min > respond_target OR resolve_min > resolve_target) AS breached,
           resolve_min - resolve_target AS overrun
    FROM measured
),
worst AS (
    SELECT priority, id,
           ROW_NUMBER() OVER (PARTITION BY priority ORDER BY overrun DESC, id) AS rn
    FROM judged WHERE breached
)
SELECT j.priority,
       COUNT(*) AS tickets,
       SUM(j.breached) AS breached,
       100 * SUM(NOT j.breached) / COUNT(*) AS pct_met,
       COALESCE(CAST(w.id AS TEXT), '-') AS worst_ticket
FROM judged j
LEFT JOIN worst w ON w.priority = j.priority AND w.rn = 1
GROUP BY j.priority
ORDER BY j.priority;

@@exercise Who carries the load?
For each assignee, show `assignee` name, `open_now` (tickets not resolved or closed), `handled` (all tickets ever assigned), `share` (their percentage of all assigned tickets, whole number) and `rank` by handled (1 = most, ties share a rank). Unassigned tickets don't count. Sort by rank, then name.
@@starter
SELECT assignee_id, COUNT(*) FROM tickets GROUP BY assignee_id;
@@expected
assignee     | open_now | handled | share | rank
-------------+----------+---------+-------+-----
Mia Lee      | 1        | 7       | 58    | 1
Ana Silva    | 2        | 4       | 33    | 2
Kenji Tanaka | 1        | 1       | 8     | 3
@@requires
RANK()
SUM(COUNT(*)) OVER ()
@@hint
Group by assignee; `SUM(status IN ('open', 'in-progress'))` for open_now; `100 * COUNT(*) / SUM(COUNT(*)) OVER ()` for share (a window over grouped rows); `RANK() OVER (ORDER BY COUNT(*) DESC)`.
@@solution
SELECT e.name AS assignee,
       SUM(t.status IN ('open', 'in-progress')) AS open_now,
       COUNT(*) AS handled,
       100 * COUNT(*) / SUM(COUNT(*)) OVER () AS share,
       RANK() OVER (ORDER BY COUNT(*) DESC) AS rank
FROM tickets t
JOIN employees e ON e.id = t.assignee_id
GROUP BY e.id
ORDER BY rank, assignee;
