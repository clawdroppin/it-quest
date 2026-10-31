@@lesson xp-cost_report AWS Cost and Tagging Report
@@text
Download a Cost and Usage export (or use the sample costs.csv) and build the report a FinOps team wants every Monday: cost per service and per `Project` tag, the month-on-month change, the share of spend with no owner tag, and the five biggest single resources.

**You'll practise:** AWS billing and Cost Explorer concepts · tagging strategy · csv and grouping · percent change

## Milestones

1. **Load**: read costs.csv (date, service, resource_id, tags, cost).
2. **Group**: totals per service and per Project tag for each month.
3. **Change**: month-on-month change per service, flag anything up more than 20%.
4. **Untagged**: the share of cost with no Project or Owner tag.
5. **Act**: write three concrete savings ideas based on what you found (Savings Plans, rightsizing, deleting idle resources).

## You're done when

- The report's totals match the CSV and the untagged share is correct.

## Getting help

Open the project, then the 🤖 **AI Helper**: it sees the lesson and your files. Good questions to start with:

- *"Which AWS services usually hide surprise costs?"*
- *"What's a good tagging policy for a small company?"*

Ask for hints, reviews and explanations rather than finished code: you learn by writing it.
@@project cost_report AWS Cost and Tagging Report
Create the folder, open it in VS Code, and work through `README.md` one milestone at a time.
@@file README.md
# AWS Cost and Tagging Report

Download a Cost and Usage export (or use the sample costs.csv) and build the report a FinOps team wants every Monday: cost per service and per `Project` tag, the month-on-month change, the share of spend with no owner tag, and the five biggest single resources.

## Milestones

- [ ] 1. Load: read costs.csv (date, service, resource_id, tags, cost).
- [ ] 2. Group: totals per service and per Project tag for each month.
- [ ] 3. Change: month-on-month change per service, flag anything up more than 20%.
- [ ] 4. Untagged: the share of cost with no Project or Owner tag.
- [ ] 5. Act: write three concrete savings ideas based on what you found (Savings Plans, rightsizing, deleting idle resources).

## Done when

- [ ] The report's totals match the CSV and the untagged share is correct.
@@file cost_report.py
"""AWS cost report from a CSV export: python cost_report.py costs.csv"""
import csv
import sys
from collections import defaultdict


def load(path: str) -> list[dict]:
    rows = list(csv.DictReader(open(path, encoding="utf-8")))
    for r in rows:
        r["cost"] = float(r["cost"])
        r["month"] = r["date"][:7]
        r["tags"] = dict(t.split("=", 1) for t in r["tags"].split(";") if "=" in t)
    return rows


def by(rows: list[dict], key) -> dict:
    totals = defaultdict(float)
    for r in rows:
        totals[key(r)] += r["cost"]
    return dict(totals)


if __name__ == "__main__":
    rows = load(sys.argv[1] if len(sys.argv) > 1 else "costs.csv")
    for service, cost in sorted(by(rows, lambda r: r["service"]).items(), key=lambda x: -x[1]):
        print(f"{service:<28} ${cost:>10,.2f}")
    # TODO: per tag, month-on-month change, untagged share, top resources
@@file costs.csv
date,service,resource_id,tags,cost
2026-09-03,Amazon EC2,i-0a1,Project=shop;Owner=ann,412.20
2026-09-03,Amazon RDS,db-shop,Project=shop;Owner=ann,301.50
2026-09-04,Amazon S3,acme-logs,,88.10
2026-09-10,AWS Lambda,fn-thumbs,Project=media,12.40
2026-09-15,Amazon EC2,i-0b7,,233.00
2026-10-03,Amazon EC2,i-0a1,Project=shop;Owner=ann,455.90
2026-10-03,Amazon RDS,db-shop,Project=shop;Owner=ann,301.50
2026-10-04,Amazon S3,acme-logs,,141.70
2026-10-09,Amazon CloudWatch,logs,Project=shop,64.30
2026-10-15,Amazon EC2,i-0b7,,233.00
2026-10-20,NAT Gateway,nat-1,Project=shop,97.80
