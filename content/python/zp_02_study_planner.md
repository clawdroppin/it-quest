@@lesson xp-study_planner Certification Study Planner
@@text
Enter your target certifications with their exam dates and estimated study hours, log what you study each day, and get a plan: hours per week needed, whether you're on track, and a streak counter. Data lives in a SQLite database (Python has `sqlite3` built in).

**You'll practise:** sqlite3 · SQL schema design · date arithmetic · functions returning data, printing separately

## Milestones

1. **Schema**: tables `goal(name, exam_date, hours_needed)` and `session(goal, day, hours)`.
2. **Commands**: `goal add`, `log 2.5 aws`, `status`.
3. **Status**: hours done, hours left, weeks left, hours per week needed, and on-track or behind.
4. **Streak**: count consecutive days with at least one session.
5. **Tests**: use an in-memory database (`sqlite3.connect(":memory:")`) in the tests.

## You're done when

- `status` shows every goal with correct numbers, and the tests pass using an in-memory database.

## Getting help

Open the project, then the 🤖 **AI Helper**: it sees the lesson and your files. Good questions to start with:

- *"Help me design the tables: should hours be REAL or INTEGER?"*
- *"How do I calculate the streak with SQL instead of Python?"*

Ask for hints, reviews and explanations rather than finished code: you learn by writing it.
@@project study_planner Certification Study Planner
Create the folder, open it in VS Code, and work through `README.md` one milestone at a time.
@@file README.md
# Certification Study Planner

Enter your target certifications with their exam dates and estimated study hours, log what you study each day, and get a plan: hours per week needed, whether you're on track, and a streak counter. Data lives in a SQLite database (Python has `sqlite3` built in).

## Milestones

- [ ] 1. Schema: tables `goal(name, exam_date, hours_needed)` and `session(goal, day, hours)`.
- [ ] 2. Commands: `goal add`, `log 2.5 aws`, `status`.
- [ ] 3. Status: hours done, hours left, weeks left, hours per week needed, and on-track or behind.
- [ ] 4. Streak: count consecutive days with at least one session.
- [ ] 5. Tests: use an in-memory database (`sqlite3.connect(":memory:")`) in the tests.

## Done when

- [ ] `status` shows every goal with correct numbers, and the tests pass using an in-memory database.
@@file planner.py
"""Study planner: python planner.py goal add NAME YYYY-MM-DD HOURS | log HOURS NAME | status"""
import datetime as dt
import sqlite3
import sys

SCHEMA = """
CREATE TABLE IF NOT EXISTS goal (name TEXT PRIMARY KEY, exam_date TEXT NOT NULL, hours_needed REAL NOT NULL);
CREATE TABLE IF NOT EXISTS session (goal TEXT NOT NULL REFERENCES goal(name), day TEXT NOT NULL, hours REAL NOT NULL);
"""


def connect(path: str = "study.db") -> sqlite3.Connection:
    db = sqlite3.connect(path)
    db.executescript(SCHEMA)
    return db


def status(db: sqlite3.Connection, today: dt.date) -> list[dict]:
    """One dict per goal: name, done, left, weeks_left, per_week, on_track."""
    # TODO
    return []


def main(argv: list[str]) -> None:
    db = connect()
    # TODO: goal add / log / status
    for g in status(db, dt.date.today()):
        print(g)


if __name__ == "__main__":
    main(sys.argv[1:])
