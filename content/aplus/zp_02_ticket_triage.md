@@lesson xp-ticket_triage Help Desk Ticket Triage
@@text
A help desk gets dozens of emails a day. Read tickets from tickets.csv, guess each one's category from keywords (hardware, network, account, printer, software), set a priority from impact and urgency, compute its SLA deadline, and print today's queue in the order an agent should work it.

**You'll practise:** impact × urgency priority matrix · SLA deadlines · keyword matching · csv and datetime

## Milestones

1. **Categories**: keyword lists per category in rules.json.
2. **Priority**: impact (1 person / team / company) × urgency gives P1–P4.
3. **SLA**: P1 4h, P2 8h, P3 2 days, P4 5 days, from the time it was opened.
4. **Queue**: sort by deadline, show time left, mark breached tickets.
5. **Accuracy**: hand-label 20 tickets and measure how often your categories are right.

## You're done when

- The queue is in a sensible order and most categories are right on your labelled tickets.

## Getting help

Open the project, then the 🤖 **AI Helper**: it sees the lesson and your files. Good questions to start with:

- *"Explain the impact and urgency matrix used in ITIL."*
- *"How can I improve keyword matching without machine learning?"*

Ask for hints, reviews and explanations rather than finished code: you learn by writing it.
@@project ticket_triage Help Desk Ticket Triage
Create the folder, open it in VS Code, and work through `README.md` one milestone at a time.
@@file README.md
# Help Desk Ticket Triage

A help desk gets dozens of emails a day. Read tickets from tickets.csv, guess each one's category from keywords (hardware, network, account, printer, software), set a priority from impact and urgency, compute its SLA deadline, and print today's queue in the order an agent should work it.

## Milestones

- [ ] 1. Categories: keyword lists per category in rules.json.
- [ ] 2. Priority: impact (1 person / team / company) × urgency gives P1–P4.
- [ ] 3. SLA: P1 4h, P2 8h, P3 2 days, P4 5 days, from the time it was opened.
- [ ] 4. Queue: sort by deadline, show time left, mark breached tickets.
- [ ] 5. Accuracy: hand-label 20 tickets and measure how often your categories are right.

## Done when

- [ ] The queue is in a sensible order and most categories are right on your labelled tickets.
@@file triage.py
"""Triage help desk tickets: python triage.py tickets.csv"""
import csv
import datetime as dt
import json
import sys

RULES = json.load(open("rules.json", encoding="utf-8"))
SLA_HOURS = {"P1": 4, "P2": 8, "P3": 48, "P4": 120}


def category(text: str) -> str:
    """The category whose keywords appear most often in the text, or "other"."""
    # TODO
    return "other"


def priority(impact: str, urgency: str) -> str:
    """impact: person/team/company; urgency: low/medium/high → P1..P4."""
    # TODO
    return "P4"


if __name__ == "__main__":
    for row in csv.DictReader(open(sys.argv[1] if len(sys.argv) > 1 else "tickets.csv", encoding="utf-8")):
        print(category(row["text"]), priority(row["impact"], row["urgency"]), row["text"])
@@file rules.json
{
  "hardware": ["screen", "keyboard", "mouse", "laptop", "battery", "fan", "noise"],
  "network": ["wifi", "wi-fi", "internet", "vpn", "slow", "disconnect"],
  "account": ["password", "locked", "login", "mfa", "account"],
  "printer": ["printer", "print", "toner", "paper jam", "scan"],
  "software": ["excel", "outlook", "install", "update", "crash", "error"]
}
@@file tickets.csv
opened,impact,urgency,text
2026-10-30 08:55,person,high,My laptop screen flickers and goes black
2026-10-30 09:10,company,high,Internet is down for the whole office
2026-10-30 09:12,person,medium,I am locked out after too many password attempts
2026-10-30 09:30,team,medium,The 3rd floor printer says paper jam but there is no paper stuck
2026-10-30 10:05,person,low,Please install Python on my PC
2026-10-30 10:40,team,high,Outlook crashes when opening shared calendar
