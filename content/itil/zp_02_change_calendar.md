@@lesson xp-change_calendar Change Calendar Conflict Checker
@@text
In ITIL change enablement, a change schedule shows planned changes. Write a checker that reads planned changes and flags: two changes to the same service in overlapping windows, changes during a freeze period (year-end, Golden Week), normal changes without CAB approval, and high-risk changes without a rollback plan.

**You'll practise:** change types: standard, normal, emergency · change schedule · risk assessment · datetime ranges

## Milestones

1. **Input**: changes.csv with id, service, type, risk, start, end, approved, rollback.
2. **Overlaps**: same service, overlapping time windows.
3. **Freezes**: freeze.json date ranges; only emergency changes may happen then.
4. **Approvals**: normal changes need approved=yes; high risk needs a rollback plan.
5. **Output**: a list of issues, then the week's schedule as a simple text calendar.

## You're done when

- Every planted issue in the sample is reported once, with the rule it breaks.

## Getting help

Open the project, then the 🤖 **AI Helper**: it sees the lesson and your files. Good questions to start with:

- *"What's the difference between a standard, normal and emergency change?"*
- *"How do I check two time ranges for overlap?"*

Ask for hints, reviews and explanations rather than finished code: you learn by writing it.
@@project change_calendar Change Calendar Conflict Checker
Create the folder, open it in VS Code, and work through `README.md` one milestone at a time.
@@file README.md
# Change Calendar Conflict Checker

In ITIL change enablement, a change schedule shows planned changes. Write a checker that reads planned changes and flags: two changes to the same service in overlapping windows, changes during a freeze period (year-end, Golden Week), normal changes without CAB approval, and high-risk changes without a rollback plan.

## Milestones

- [ ] 1. Input: changes.csv with id, service, type, risk, start, end, approved, rollback.
- [ ] 2. Overlaps: same service, overlapping time windows.
- [ ] 3. Freezes: freeze.json date ranges; only emergency changes may happen then.
- [ ] 4. Approvals: normal changes need approved=yes; high risk needs a rollback plan.
- [ ] 5. Output: a list of issues, then the week's schedule as a simple text calendar.

## Done when

- [ ] Every planted issue in the sample is reported once, with the rule it breaks.
@@file changes.py
"""Check a change schedule: python changes.py changes.csv freeze.json"""
import csv
import datetime as dt
import json
import sys


def parse(s: str) -> dt.datetime:
    return dt.datetime.fromisoformat(s)


def overlaps(a_start, a_end, b_start, b_end) -> bool:
    return a_start < b_end and b_start < a_end


if __name__ == "__main__":
    changes = list(csv.DictReader(open(sys.argv[1] if len(sys.argv) > 1 else "changes.csv", encoding="utf-8")))
    freezes = json.load(open(sys.argv[2] if len(sys.argv) > 2 else "freeze.json", encoding="utf-8"))
    # TODO: overlaps, freezes, approvals, rollback plans
    print(len(changes), "changes,", len(freezes), "freeze periods")
@@file changes.csv
id,service,type,risk,start,end,approved,rollback
C1,email,normal,low,2026-11-03T22:00,2026-11-03T23:00,yes,yes
C2,email,normal,medium,2026-11-03T22:30,2026-11-04T00:00,yes,yes
C3,erp,normal,high,2026-12-29T20:00,2026-12-29T23:00,yes,no
C4,vpn,standard,low,2026-11-05T12:00,2026-11-05T12:30,no,yes
C5,web,normal,medium,2026-11-06T21:00,2026-11-06T22:00,no,yes
@@file freeze.json
[
  { "name": "year end", "start": "2026-12-25T00:00", "end": "2027-01-05T00:00" },
  { "name": "Golden Week", "start": "2027-04-29T00:00", "end": "2027-05-06T00:00" }
]
