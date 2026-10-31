@@lesson xp-cloud_compare Cloud Provider Cost Comparison
@@text
Design one small architecture (two web VMs behind a load balancer, a managed PostgreSQL database, 500 GB of object storage, 1 TB of internet egress a month) and price it on all three clouds in the Tokyo region using their pricing calculators. Put the numbers into a script that compares on-demand, 1-year and 3-year commitments.

**You'll practise:** equivalent services across clouds · pricing models · egress costs · commitments and reservations

## Milestones

1. **Map**: the equivalent service on each cloud (EC2/VM/Compute Engine, RDS/Azure Database/Cloud SQL...).
2. **Price**: use each official calculator for the Tokyo region and record the numbers in prices.json.
3. **Compare**: monthly totals per cloud, per pricing model.
4. **Sensitivity**: how the ranking changes if egress triples.
5. **Recommend**: a one-page summary with the non-price factors too (skills, support, Japanese data residency).

## You're done when

- COMPARE.md has a table for all three clouds and a recommendation you can defend.

## Getting help

Open the project, then the 🤖 **AI Helper**: it sees the lesson and your files. Good questions to start with:

- *"Which services are equivalent for a managed PostgreSQL on each cloud?"*
- *"Why is egress often the surprise cost?"*

Ask for hints, reviews and explanations rather than finished code: you learn by writing it.
@@project cloud_compare Cloud Provider Cost Comparison
Create the folder, open it in VS Code, and work through `README.md` one milestone at a time.
@@file README.md
# Cloud Provider Cost Comparison

Design one small architecture (two web VMs behind a load balancer, a managed PostgreSQL database, 500 GB of object storage, 1 TB of internet egress a month) and price it on all three clouds in the Tokyo region using their pricing calculators. Put the numbers into a script that compares on-demand, 1-year and 3-year commitments.

## Milestones

- [ ] 1. Map: the equivalent service on each cloud (EC2/VM/Compute Engine, RDS/Azure Database/Cloud SQL...).
- [ ] 2. Price: use each official calculator for the Tokyo region and record the numbers in prices.json.
- [ ] 3. Compare: monthly totals per cloud, per pricing model.
- [ ] 4. Sensitivity: how the ranking changes if egress triples.
- [ ] 5. Recommend: a one-page summary with the non-price factors too (skills, support, Japanese data residency).

## Done when

- [ ] COMPARE.md has a table for all three clouds and a recommendation you can defend.
@@file compare.py
"""Compare cloud costs from prices.json"""
import json

prices = json.load(open("prices.json", encoding="utf-8"))
for cloud, items in prices.items():
    total = sum(items.values())
    print(f"{cloud:<6} ${total:>8,.2f} / month (on demand)")
# TODO: 1-year and 3-year commitments, egress sensitivity
@@file prices.json
{
  "aws":   { "vms": 0, "load_balancer": 0, "database": 0, "storage": 0, "egress": 0 },
  "azure": { "vms": 0, "load_balancer": 0, "database": 0, "storage": 0, "egress": 0 },
  "gcp":   { "vms": 0, "load_balancer": 0, "database": 0, "storage": 0, "egress": 0 }
}
