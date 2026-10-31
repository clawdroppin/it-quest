@@lesson xp-rightsizing VM Rightsizing Advisor
@@text
VMs sized "just in case" waste money. Read 30 days of CPU and memory metrics per VM (export from Azure Monitor, or use the sample), compute the 95th percentiles, and recommend a size from a small catalog with the monthly saving, never recommending a size that would be over 80% at the 95th percentile.

**You'll practise:** Azure VM sizes and pricing · percentiles · cost optimization · Azure Advisor concepts

## Milestones

1. **Catalog**: sizes.json with vCPUs, memory and a monthly price.
2. **Metrics**: metrics.csv with vm, time, cpu_percent, mem_used_gb.
3. **P95**: CPU and memory 95th percentile per VM.
4. **Recommend**: the cheapest size with headroom; show current vs recommended cost.
5. **Caveats**: list cases where the recommendation is risky (batch jobs, month-end peaks).

## You're done when

- Recommendations make sense for the sample, with total monthly savings.

## Getting help

Open the project, then the 🤖 **AI Helper**: it sees the lesson and your files. Good questions to start with:

- *"Why use the 95th percentile and not the average?"*
- *"How do B-series burstable VMs change this analysis?"*

Ask for hints, reviews and explanations rather than finished code: you learn by writing it.
@@project rightsizing VM Rightsizing Advisor
Create the folder, open it in VS Code, and work through `README.md` one milestone at a time.
@@file README.md
# VM Rightsizing Advisor

VMs sized "just in case" waste money. Read 30 days of CPU and memory metrics per VM (export from Azure Monitor, or use the sample), compute the 95th percentiles, and recommend a size from a small catalog with the monthly saving, never recommending a size that would be over 80% at the 95th percentile.

## Milestones

- [ ] 1. Catalog: sizes.json with vCPUs, memory and a monthly price.
- [ ] 2. Metrics: metrics.csv with vm, time, cpu_percent, mem_used_gb.
- [ ] 3. P95: CPU and memory 95th percentile per VM.
- [ ] 4. Recommend: the cheapest size with headroom; show current vs recommended cost.
- [ ] 5. Caveats: list cases where the recommendation is risky (batch jobs, month-end peaks).

## Done when

- [ ] Recommendations make sense for the sample, with total monthly savings.
@@file rightsize.py
"""VM rightsizing: python rightsize.py metrics.csv sizes.json"""
import csv
import json
import math
import sys
from collections import defaultdict


def p95(values: list[float]) -> float:
    s = sorted(values)
    return s[max(0, math.ceil(0.95 * len(s)) - 1)]


rows = list(csv.DictReader(open(sys.argv[1] if len(sys.argv) > 1 else "metrics.csv", encoding="utf-8")))
sizes = json.load(open(sys.argv[2] if len(sys.argv) > 2 else "sizes.json", encoding="utf-8"))
cpu = defaultdict(list)
for r in rows:
    cpu[r["vm"]].append(float(r["cpu_percent"]))
for vm, values in cpu.items():
    print(vm, "cpu p95", p95(values))
# TODO: memory, current size, recommendation and savings
@@file sizes.json
{
  "Standard_B2s":  { "vcpu": 2, "mem_gb": 4,  "monthly_usd": 30 },
  "Standard_D2s_v5": { "vcpu": 2, "mem_gb": 8,  "monthly_usd": 70 },
  "Standard_D4s_v5": { "vcpu": 4, "mem_gb": 16, "monthly_usd": 140 },
  "Standard_D8s_v5": { "vcpu": 8, "mem_gb": 32, "monthly_usd": 280 }
}
@@file metrics.csv
vm,size,time,cpu_percent,mem_used_gb
web-01,Standard_D8s_v5,2026-10-01T00:00,12,6.1
web-01,Standard_D8s_v5,2026-10-01T01:00,18,6.4
web-01,Standard_D8s_v5,2026-10-01T02:00,9,6.0
db-01,Standard_D4s_v5,2026-10-01T00:00,71,13.2
db-01,Standard_D4s_v5,2026-10-01T01:00,85,14.0
