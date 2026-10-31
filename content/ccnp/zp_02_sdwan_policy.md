@@lesson xp-sdwan_policy SD-WAN Path Selection Simulator
@@text
Application-aware routing in SD-WAN measures every tunnel continuously and moves traffic when a link breaks its SLA. Simulate it: links with measured latency, loss and jitter over time, SLA classes per application (voice, video, business, bulk), and a policy that picks the preferred link that meets the SLA, with fallback and hysteresis so traffic doesn't flap.

**You'll practise:** SD-WAN application-aware routing · SLA classes · hysteresis · simulation over time

## Milestones

1. **Data**: measurements.csv with time, link (mpls, inet, lte) and latency/loss/jitter.
2. **SLA classes**: voice ≤150 ms, ≤1% loss, ≤30 ms jitter; define the others.
3. **Choose**: for each app and time step, the preferred link meeting the SLA, else the best available.
4. **Hysteresis**: only move back to the preferred link after it has been good for 3 intervals.
5. **Report**: a timeline of which link each app used, and how many times it switched.

## You're done when

- The simulation moves voice off the internet link during the sample's loss spike and back afterwards, without flapping.

## Getting help

Open the project, then the 🤖 **AI Helper**: it sees the lesson and your files. Good questions to start with:

- *"How do real SD-WAN products measure loss and jitter on tunnels?"*
- *"Why is hysteresis important in path selection?"*

Ask for hints, reviews and explanations rather than finished code: you learn by writing it.
@@project sdwan_policy SD-WAN Path Selection Simulator
Create the folder, open it in VS Code, and work through `README.md` one milestone at a time.
@@file README.md
# SD-WAN Path Selection Simulator

Application-aware routing in SD-WAN measures every tunnel continuously and moves traffic when a link breaks its SLA. Simulate it: links with measured latency, loss and jitter over time, SLA classes per application (voice, video, business, bulk), and a policy that picks the preferred link that meets the SLA, with fallback and hysteresis so traffic doesn't flap.

## Milestones

- [ ] 1. Data: measurements.csv with time, link (mpls, inet, lte) and latency/loss/jitter.
- [ ] 2. SLA classes: voice ≤150 ms, ≤1% loss, ≤30 ms jitter; define the others.
- [ ] 3. Choose: for each app and time step, the preferred link meeting the SLA, else the best available.
- [ ] 4. Hysteresis: only move back to the preferred link after it has been good for 3 intervals.
- [ ] 5. Report: a timeline of which link each app used, and how many times it switched.

## Done when

- [ ] The simulation moves voice off the internet link during the sample's loss spike and back afterwards, without flapping.
@@file sdwan.py
"""Simulate SD-WAN application-aware routing: python sdwan.py measurements.csv"""
import csv
import sys

SLA = {
    "voice": {"latency": 150, "loss": 1.0, "jitter": 30},
    # TODO: video, business, bulk
}
PREFERENCE = {"voice": ["mpls", "inet", "lte"]}


def meets(m: dict, sla: dict) -> bool:
    return m["latency"] <= sla["latency"] and m["loss"] <= sla["loss"] and m["jitter"] <= sla["jitter"]


if __name__ == "__main__":
    rows = list(csv.DictReader(open(sys.argv[1] if len(sys.argv) > 1 else "measurements.csv", encoding="utf-8")))
    # TODO: group by time, choose per app with hysteresis, print a timeline
    print(len(rows), "measurements")
@@file measurements.csv
t,link,latency,loss,jitter
1,mpls,40,0.0,3
1,inet,35,0.2,8
2,mpls,41,0.0,3
2,inet,90,4.5,45
3,mpls,40,0.0,2
3,inet,95,5.0,50
4,mpls,42,0.1,3
4,inet,36,0.1,7
5,mpls,40,0.0,3
5,inet,34,0.0,6
