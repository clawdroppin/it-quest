@@lesson xp-job_tracker Japan Job Application Tracker
@@text
Job hunting in Japan has many steps: 書類選考, several interviews, sometimes an aptitude test (SPI), then 内定. Track every application in a CSV, see your pipeline at a glance, get reminders for follow-ups, and compare offers by estimated monthly take-home pay after tax and social insurance.

**You'll practise:** the Japanese hiring process · salary structures (bonuses, allowances) · csv · simple estimates

## Milestones

1. **Pipeline**: applications.csv with company, role, stage, last_contact, salary_offer.
2. **Overview**: counts per stage, and companies with no contact for 10+ days.
3. **Offers**: annual salary → rough monthly take-home (income tax, resident tax, ~15% social insurance).
4. **Compare**: offers side by side including bonus months and commuting allowance.
5. **Notes**: a template for interview prep per company (自己紹介, 志望動機, questions to ask).

## You're done when

- Your real applications are in it and the overview tells you what to do next.

## Getting help

Open the project, then the 🤖 **AI Helper**: it sees the lesson and your files. Good questions to start with:

- *"What is the difference between 年収 and 月給 in a Japanese offer?"*
- *"Help me prepare a 志望動機 for an infrastructure engineer role."*

Ask for hints, reviews and explanations rather than finished code: you learn by writing it.
@@project job_tracker Japan Job Application Tracker
Create the folder, open it in VS Code, and work through `README.md` one milestone at a time.
@@file README.md
# Japan Job Application Tracker

Job hunting in Japan has many steps: 書類選考, several interviews, sometimes an aptitude test (SPI), then 内定. Track every application in a CSV, see your pipeline at a glance, get reminders for follow-ups, and compare offers by estimated monthly take-home pay after tax and social insurance.

## Milestones

- [ ] 1. Pipeline: applications.csv with company, role, stage, last_contact, salary_offer.
- [ ] 2. Overview: counts per stage, and companies with no contact for 10+ days.
- [ ] 3. Offers: annual salary → rough monthly take-home (income tax, resident tax, ~15% social insurance).
- [ ] 4. Compare: offers side by side including bonus months and commuting allowance.
- [ ] 5. Notes: a template for interview prep per company (自己紹介, 志望動機, questions to ask).

## Done when

- [ ] Your real applications are in it and the overview tells you what to do next.
@@file tracker.py
"""Job application tracker: python tracker.py"""
import csv
import datetime as dt
from collections import Counter

STAGES = ["applied", "document screening", "interview 1", "interview 2", "final interview", "offer", "rejected"]


def take_home_monthly(annual_yen: int, bonus_months: float = 2.0) -> int:
    """A rough estimate of monthly take-home pay: about 75-80% of the gross monthly salary."""
    monthly = annual_yen / (12 + bonus_months)
    # TODO: a better estimate with social insurance (~15%), income tax and resident tax
    return round(monthly * 0.78)


if __name__ == "__main__":
    apps = list(csv.DictReader(open("applications.csv", encoding="utf-8")))
    print(Counter(a["stage"] for a in apps))
    # TODO: follow-up reminders, offer comparison
@@file applications.csv
company,role,stage,last_contact,salary_offer
Example SIer,Infrastructure engineer,interview 1,2026-10-20,
Example Cloud KK,Cloud support engineer,offer,2026-10-28,5200000
Example Games,SRE,document screening,2026-10-10,
