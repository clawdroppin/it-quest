@@lesson xp-helpdesk_db Help Desk Ticket Database
@@text
Design the database behind a help desk: users, agents, tickets, comments and categories. Then answer the questions managers ask every week with SQL: open tickets per agent, average time to resolve per category, tickets breaching their SLA, and the busiest hour of the day.

**You'll practise:** schema design and foreign keys · JOIN and GROUP BY · date functions · window functions · indexes

## Milestones

1. **Schema**: write schema.sql with tables, primary and foreign keys, and CHECK constraints for status and priority.
2. **Data**: add 30+ realistic rows in data.sql (or generate them).
3. **Reports**: write each question in reports.sql and check the answers by hand.
4. **SLA**: priority 1 must be resolved in 4 hours, 2 in 1 day, 3 in 3 days; list breaches.
5. **Speed**: add indexes for the reports and look at `EXPLAIN QUERY PLAN`.

## You're done when

- Every report in reports.sql runs and gives answers you've checked.

## Getting help

Open the project, then the 🤖 **AI Helper**: it sees the lesson and your files. Good questions to start with:

- *"Review my schema: are my foreign keys and constraints right?"*
- *"How do I compute hours between two timestamps in SQLite?"*

Ask for hints, reviews and explanations rather than finished code: you learn by writing it.
@@project helpdesk_db Help Desk Ticket Database
Create the folder, open it in VS Code, and work through `README.md` one milestone at a time.
@@file README.md
# Help Desk Ticket Database

Design the database behind a help desk: users, agents, tickets, comments and categories. Then answer the questions managers ask every week with SQL: open tickets per agent, average time to resolve per category, tickets breaching their SLA, and the busiest hour of the day.

## Milestones

- [ ] 1. Schema: write schema.sql with tables, primary and foreign keys, and CHECK constraints for status and priority.
- [ ] 2. Data: add 30+ realistic rows in data.sql (or generate them).
- [ ] 3. Reports: write each question in reports.sql and check the answers by hand.
- [ ] 4. SLA: priority 1 must be resolved in 4 hours, 2 in 1 day, 3 in 3 days; list breaches.
- [ ] 5. Speed: add indexes for the reports and look at `EXPLAIN QUERY PLAN`.

## Done when

- [ ] Every report in reports.sql runs and gives answers you've checked.
@@file schema.sql
-- Help desk schema (SQLite). Run: sqlite3 helpdesk.db < schema.sql
PRAGMA foreign_keys = ON;

CREATE TABLE agent (
    id    INTEGER PRIMARY KEY,
    name  TEXT NOT NULL,
    team  TEXT NOT NULL
);

CREATE TABLE ticket (
    id          INTEGER PRIMARY KEY,
    title       TEXT NOT NULL,
    priority    INTEGER NOT NULL CHECK (priority BETWEEN 1 AND 3),
    status      TEXT NOT NULL CHECK (status IN ('open', 'in_progress', 'resolved')),
    agent_id    INTEGER REFERENCES agent(id),
    opened_at   TEXT NOT NULL,
    resolved_at TEXT
);

-- TODO: users, categories, comments
@@file reports.sql
-- 1. Open tickets per agent
SELECT a.name, COUNT(*) AS open_tickets
FROM ticket t JOIN agent a ON a.id = t.agent_id
WHERE t.status <> 'resolved'
GROUP BY a.name
ORDER BY open_tickets DESC;

-- 2. TODO: average hours to resolve, per category
-- 3. TODO: SLA breaches
-- 4. TODO: busiest hour of the day
