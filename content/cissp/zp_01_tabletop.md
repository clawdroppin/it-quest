@@lesson xp-tabletop Incident Response Tabletop Exercise
@@text
Tabletop exercises test an incident response plan without breaking anything. Build a facilitator tool that runs a scenario as timed "injects" (new information arriving), asks the team decision questions, records their answers and decision times, and scores them against the plan: who was notified, when containment started, whether legal and management were involved.

**You'll practise:** incident response phases · business continuity testing · roles and escalation · after-action reports

## Milestones

1. **Scenario**: scenario.json with injects (minute, message, question, expected actions).
2. **Run**: print each inject at its time (or on Enter), record the team's answer.
3. **Score**: compare answers with expected actions (keywords), per phase.
4. **Report**: an after-action report with what went well and what to improve.
5. **Second scenario**: write one for a cloud outage or a lost laptop.

## You're done when

- You've run the exercise with at least one other person and produced the after-action report.

## Getting help

Open the project, then the 🤖 **AI Helper**: it sees the lesson and your files. Good questions to start with:

- *"What should the first hour of a ransomware response look like?"*
- *"Which decisions in an incident must go to management or legal?"*

Ask for hints, reviews and explanations rather than finished code: you learn by writing it.
@@project tabletop Incident Response Tabletop Exercise
Create the folder, open it in VS Code, and work through `README.md` one milestone at a time.
@@file README.md
# Incident Response Tabletop Exercise

Tabletop exercises test an incident response plan without breaking anything. Build a facilitator tool that runs a scenario as timed "injects" (new information arriving), asks the team decision questions, records their answers and decision times, and scores them against the plan: who was notified, when containment started, whether legal and management were involved.

## Milestones

- [ ] 1. Scenario: scenario.json with injects (minute, message, question, expected actions).
- [ ] 2. Run: print each inject at its time (or on Enter), record the team's answer.
- [ ] 3. Score: compare answers with expected actions (keywords), per phase.
- [ ] 4. Report: an after-action report with what went well and what to improve.
- [ ] 5. Second scenario: write one for a cloud outage or a lost laptop.

## Done when

- [ ] You've run the exercise with at least one other person and produced the after-action report.
@@file tabletop.py
"""Run a tabletop exercise: python tabletop.py scenario.json"""
import json
import sys
import time

scenario = json.load(open(sys.argv[1] if len(sys.argv) > 1 else "scenario.json", encoding="utf-8"))
log = []
print(f"=== {scenario['title']} ===")
for inject in scenario["injects"]:
    input(f"\n[T+{inject['minute']} min] press Enter for the next inject")
    print(inject["message"])
    start = time.time()
    answer = input(inject["question"] + "\n> ")
    log.append({"minute": inject["minute"], "answer": answer, "seconds": round(time.time() - start)})
# TODO: score against inject["expected"], write after_action.md
@@file scenario.json
{
  "title": "Ransomware on the file server",
  "injects": [
    { "minute": 0, "message": "The help desk gets five calls: files on the S: drive have the extension .locked.",
      "question": "What do you do first, and who do you call?", "expected": ["isolate", "incident manager", "do not pay"] },
    { "minute": 30, "message": "A ransom note asks for 5 BTC within 48 hours. Backups from last night exist.",
      "question": "Who decides about the ransom, and what do you check about the backups?", "expected": ["management", "legal", "backups offline", "test restore"] }
  ]
}
