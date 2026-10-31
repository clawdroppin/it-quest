@@lesson xp-qos_planner QoS Policy Planner
@@text
Voice breaks when a WAN link fills up. Plan a QoS policy for a 50 Mbps branch link: classes for voice, video, business apps, default and scavenger, with DSCP markings, bandwidth percentages and a priority queue for voice that never exceeds a third of the link. Then generate the Cisco MQC config (class-map, policy-map, service-policy) and check the plan's numbers.

**You'll practise:** DSCP and traffic classes · LLQ and CBWFQ · policing vs shaping · MQC configuration

## Milestones

1. **Classes**: classes.json with DSCP values and percentages.
2. **Validate**: total ≤ 100%, priority ≤ 33%, every DSCP used once.
3. **Calls**: how many G.711 calls (about 87 kbps each with overhead) fit in the priority queue.
4. **Generate**: class-maps, a policy-map and a shaper for the link.
5. **Review**: explain where in the network to mark and where to queue.

## You're done when

- The generated config is valid IOS syntax and the checks catch an over-allocated plan.

## Getting help

Open the project, then the 🤖 **AI Helper**: it sees the lesson and your files. Good questions to start with:

- *"Why limit the priority queue to about a third of the link?"*
- *"Where should traffic be marked, and why there?"*

Ask for hints, reviews and explanations rather than finished code: you learn by writing it.
@@project qos_planner QoS Policy Planner
Create the folder, open it in VS Code, and work through `README.md` one milestone at a time.
@@file README.md
# QoS Policy Planner

Voice breaks when a WAN link fills up. Plan a QoS policy for a 50 Mbps branch link: classes for voice, video, business apps, default and scavenger, with DSCP markings, bandwidth percentages and a priority queue for voice that never exceeds a third of the link. Then generate the Cisco MQC config (class-map, policy-map, service-policy) and check the plan's numbers.

## Milestones

- [ ] 1. Classes: classes.json with DSCP values and percentages.
- [ ] 2. Validate: total ≤ 100%, priority ≤ 33%, every DSCP used once.
- [ ] 3. Calls: how many G.711 calls (about 87 kbps each with overhead) fit in the priority queue.
- [ ] 4. Generate: class-maps, a policy-map and a shaper for the link.
- [ ] 5. Review: explain where in the network to mark and where to queue.

## Done when

- [ ] The generated config is valid IOS syntax and the checks catch an over-allocated plan.
@@file qos.py
"""Plan and generate a QoS policy: python qos.py classes.json 50"""
import json
import sys

classes = json.load(open(sys.argv[1] if len(sys.argv) > 1 else "classes.json", encoding="utf-8"))
link_mbps = float(sys.argv[2]) if len(sys.argv) > 2 else 50
total = sum(c["percent"] for c in classes)
print(f"allocated {total}% of {link_mbps} Mbps")
for c in classes:
    print(f"class-map match-any {c['name']}\n match dscp {c['dscp']}")
# TODO: validation rules, voice call capacity, policy-map with priority/bandwidth, shaper
@@file classes.json
[
  { "name": "VOICE", "dscp": "ef", "percent": 20, "priority": true },
  { "name": "VIDEO", "dscp": "af41", "percent": 20 },
  { "name": "BUSINESS", "dscp": "af21", "percent": 30 },
  { "name": "SCAVENGER", "dscp": "cs1", "percent": 5 }
]
