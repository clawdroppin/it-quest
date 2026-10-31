@@lesson xp-pc_refresh PC Refresh Plan
@@text
A small office has ten PCs that are seven years old. Plan their replacement like an IT technician would: inventory the old machines, choose new specs, plan user data migration, schedule the swaps to minimize downtime, and dispose of the old drives securely. The deliverable is a plan document plus a checklist script.

**You'll practise:** hardware specs · Windows deployment and user migration · secure data destruction · project checklists

## Milestones

1. **Inventory**: old-pcs.csv with CPU, RAM, disk, Windows version and user.
2. **Specs**: choose a standard new PC and justify CPU, RAM, SSD and warranty.
3. **Migration**: list what to move per user (Documents, browser bookmarks, Outlook signatures, printers).
4. **Schedule**: two PCs per evening, with a rollback plan if a migration fails.
5. **Disposal**: wipe or destroy drives and record certificates of destruction.

## You're done when

- PLAN.md covers all five parts, and checklist.py prints a per-user checklist from the CSV.

## Getting help

Open the project, then the 🤖 **AI Helper**: it sees the lesson and your files. Good questions to start with:

- *"Which Windows 10 PCs can't be upgraded to Windows 11, and why (TPM 2.0, CPU)?"*
- *"What's the safest way to wipe an SSD before disposal?"*

Ask for hints, reviews and explanations rather than finished code: you learn by writing it.
@@project pc_refresh PC Refresh Plan
Create the folder, open it in VS Code, and work through `README.md` one milestone at a time.
@@file README.md
# PC Refresh Plan

A small office has ten PCs that are seven years old. Plan their replacement like an IT technician would: inventory the old machines, choose new specs, plan user data migration, schedule the swaps to minimize downtime, and dispose of the old drives securely. The deliverable is a plan document plus a checklist script.

## Milestones

- [ ] 1. Inventory: old-pcs.csv with CPU, RAM, disk, Windows version and user.
- [ ] 2. Specs: choose a standard new PC and justify CPU, RAM, SSD and warranty.
- [ ] 3. Migration: list what to move per user (Documents, browser bookmarks, Outlook signatures, printers).
- [ ] 4. Schedule: two PCs per evening, with a rollback plan if a migration fails.
- [ ] 5. Disposal: wipe or destroy drives and record certificates of destruction.

## Done when

- [ ] PLAN.md covers all five parts, and checklist.py prints a per-user checklist from the CSV.
@@file checklist.py
"""Print a migration checklist for each user: python checklist.py old-pcs.csv"""
import csv
import sys

STEPS = ["back up Documents and Desktop", "export browser bookmarks", "note mapped drives and printers",
         "copy Outlook signatures", "sign in on the new PC", "restore data and check with the user"]

for pc in csv.DictReader(open(sys.argv[1] if len(sys.argv) > 1 else "old-pcs.csv", encoding="utf-8")):
    print(f"## {pc['user']} ({pc['name']})")
    for s in STEPS:
        print(f"- [ ] {s}")
    # TODO: flag PCs that need extra steps (old Windows, small disk)
@@file old-pcs.csv
name,user,cpu,ram_gb,disk_gb,windows
pc-01,ann,i5-7500,8,256,10
pc-02,ken,i3-7100,4,500,10
pc-03,yui,i5-8400,8,256,11
