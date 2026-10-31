@@lesson xp-access_review Joiner-Mover-Leaver Access Review
@@text
Access reviews are one of the most audited controls. Compare an HR export (who works here, in which role) with account exports from three systems, and report: accounts of people who left (orphaned), access that doesn't match the person's current role (privilege creep after a move), shared or generic accounts, and admins without a recent review.

**You'll practise:** identity lifecycle · least privilege · role-based access · audit evidence · joining data sets

## Milestones

1. **Inputs**: hr.csv (employee, status, role, department) and accounts.csv (system, user, entitlement, last_login).
2. **Role model**: roles.json lists the entitlements each role should have.
3. **Leavers**: active accounts of people with status left.
4. **Creep**: entitlements not in the person's role model.
5. **Evidence**: an output CSV a reviewer can sign off, with a column for their decision.

## You're done when

- The sample data's planted problems (one leaver, one mover, one shared account) are all found.

## Getting help

Open the project, then the 🤖 **AI Helper**: it sees the lesson and your files. Good questions to start with:

- *"What makes access review evidence acceptable to an auditor?"*
- *"How should I handle service accounts that don't belong to a person?"*

Ask for hints, reviews and explanations rather than finished code: you learn by writing it.
@@project access_review Joiner-Mover-Leaver Access Review
Create the folder, open it in VS Code, and work through `README.md` one milestone at a time.
@@file README.md
# Joiner-Mover-Leaver Access Review

Access reviews are one of the most audited controls. Compare an HR export (who works here, in which role) with account exports from three systems, and report: accounts of people who left (orphaned), access that doesn't match the person's current role (privilege creep after a move), shared or generic accounts, and admins without a recent review.

## Milestones

- [ ] 1. Inputs: hr.csv (employee, status, role, department) and accounts.csv (system, user, entitlement, last_login).
- [ ] 2. Role model: roles.json lists the entitlements each role should have.
- [ ] 3. Leavers: active accounts of people with status left.
- [ ] 4. Creep: entitlements not in the person's role model.
- [ ] 5. Evidence: an output CSV a reviewer can sign off, with a column for their decision.

## Done when

- [ ] The sample data's planted problems (one leaver, one mover, one shared account) are all found.
@@file review.py
"""Access review: python review.py hr.csv accounts.csv roles.json"""
import csv
import json
import sys


def main(hr_path: str, accounts_path: str, roles_path: str) -> None:
    hr = {r["employee"]: r for r in csv.DictReader(open(hr_path, encoding="utf-8"))}
    accounts = list(csv.DictReader(open(accounts_path, encoding="utf-8")))
    roles = json.load(open(roles_path, encoding="utf-8"))
    for a in accounts:
        person = hr.get(a["user"])
        # TODO: leavers, unknown users, entitlements outside the role
        print(a["system"], a["user"], a["entitlement"], person["status"] if person else "NOT IN HR")


if __name__ == "__main__":
    main(*(sys.argv[1:4] or ["hr.csv", "accounts.csv", "roles.json"]))
@@file hr.csv
employee,status,role,department
ann,active,developer,engineering
ken,active,accountant,finance
yui,left,developer,engineering
taro,active,support,it
@@file accounts.csv
system,user,entitlement,last_login
github,ann,write,2026-10-29
github,yui,write,2026-09-01
erp,ken,ledger-edit,2026-10-28
erp,ken,payroll-admin,2026-10-02
github,ken,write,2026-03-11
helpdesk,taro,agent,2026-10-30
helpdesk,support,agent,2026-10-30
@@file roles.json
{
  "developer": ["github:write"],
  "accountant": ["erp:ledger-edit"],
  "support": ["helpdesk:agent"]
}
