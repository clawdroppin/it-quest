@@lesson xp-sla_calculator Architecture Availability Calculator
@@text
Architects are asked "what availability will this design give us?" Describe an architecture as a tree of components in series and in parallel (zones, regions), with each Google Cloud SLA, and compute the composite availability and the allowed downtime per month. Then improve the design until it meets a 99.95% target.

**You'll practise:** composite SLAs · series and parallel availability · zonal, regional and multi-regional services · recursion

## Milestones

1. **Model**: design.json with nested `series` and `parallel` lists and leaf components with SLAs.
2. **Compute**: series multiplies; parallel is 1 − product of (1 − A).
3. **Downtime**: availability → minutes per month and hours per year.
4. **Improve**: add a second region or a regional database and compare.
5. **Explain**: in DESIGN.md, which component limits availability most.

## You're done when

- The calculator gives the right numbers for hand-checked designs and your final design meets 99.95%.

## Getting help

Open the project, then the 🤖 **AI Helper**: it sees the lesson and your files. Good questions to start with:

- *"Why can't two services in series have higher availability than the weaker one?"*
- *"Which Google Cloud services are zonal, regional or multi-regional?"*

Ask for hints, reviews and explanations rather than finished code: you learn by writing it.
@@project sla_calculator Architecture Availability Calculator
Create the folder, open it in VS Code, and work through `README.md` one milestone at a time.
@@file README.md
# Architecture Availability Calculator

Architects are asked "what availability will this design give us?" Describe an architecture as a tree of components in series and in parallel (zones, regions), with each Google Cloud SLA, and compute the composite availability and the allowed downtime per month. Then improve the design until it meets a 99.95% target.

## Milestones

- [ ] 1. Model: design.json with nested `series` and `parallel` lists and leaf components with SLAs.
- [ ] 2. Compute: series multiplies; parallel is 1 − product of (1 − A).
- [ ] 3. Downtime: availability → minutes per month and hours per year.
- [ ] 4. Improve: add a second region or a regional database and compare.
- [ ] 5. Explain: in DESIGN.md, which component limits availability most.

## Done when

- [ ] The calculator gives the right numbers for hand-checked designs and your final design meets 99.95%.
@@file sla.py
"""Composite availability: python sla.py design.json"""
import json
import sys


def availability(node) -> float:
    if "sla" in node:
        return node["sla"]
    if "series" in node:
        a = 1.0
        for child in node["series"]:
            a *= availability(child)
        return a
    # TODO: parallel
    raise ValueError(node)


design = json.load(open(sys.argv[1] if len(sys.argv) > 1 else "design.json", encoding="utf-8"))
a = availability(design)
print(f"{a:.5%}  → {(1 - a) * 30 * 24 * 60:.1f} minutes of downtime per month")
@@file design.json
{
  "series": [
    { "name": "global load balancer", "sla": 0.9999 },
    { "name": "Cloud Run (one region)", "sla": 0.9995 },
    { "name": "Cloud SQL HA", "sla": 0.9995 }
  ]
}
