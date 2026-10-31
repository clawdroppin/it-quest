from gen_projects import P

PROJECTS = [
    P("pmp", "Intermediate", "sprint_metrics", "Sprint Metrics Dashboard", "Velocity, burndown and forecast from a team's sprint data.",
      """Agile project managers forecast with data, not hope. From sprints.csv (planned and completed story points per sprint) and the remaining backlog, compute the velocity average and range, draw a text burndown for the current sprint, and forecast how many sprints the backlog needs, as a range ("6 to 9 sprints"), not a single date.""",
      ["velocity", "burndown and burnup", "forecasting with ranges", "agile metrics and their misuse"],
      ["Data: sprints.csv and burndown.csv (day, points remaining).",
       "Velocity: average of the last 3 sprints, and the lowest and highest.",
       "Burndown: a text chart of remaining points against the ideal line.",
       "Forecast: backlog ÷ high, average and low velocity → a range of sprints and dates.",
       "Explain: in NOTES.md, why velocity must never be used to compare teams."],
      ["The forecast range is correct for the sample and the burndown shows whether the sprint is on track."],
      ["Why is a forecast range better than a single date?",
       "What are the risks of using velocity as a performance target?"],
      {"metrics.py": '''"""Sprint metrics: python metrics.py"""
import csv
import statistics

sprints = list(csv.DictReader(open("sprints.csv", encoding="utf-8")))
done = [int(s["completed"]) for s in sprints]
last3 = done[-3:]
print("velocity (last 3):", statistics.mean(last3), "range", min(last3), "-", max(last3))
# TODO: burndown chart, forecast range for the backlog
''', "sprints.csv": "sprint,planned,completed\n1,30,22\n2,28,25\n3,30,31\n4,32,27\n5,30,29",
       "burndown.csv": "day,remaining\n0,30\n1,28\n2,27\n3,27\n4,22\n5,18\n6,17\n7,12"}),
    P("pmp", "Advanced", "project_plan", "Plan a Real IT Project", "Write the charter, WBS, risk register and schedule for a project you know.",
      """Pick a real or realistic IT project (a Microsoft 365 migration, a new office network, a website relaunch) and produce the core planning documents as a project manager would: a one-page charter, a stakeholder register with the power/interest grid, a WBS down to work packages, a schedule run through this track's critical path calculator, a risk register with responses, and a communication plan.""",
      ["project charter", "WBS", "stakeholder analysis", "risk register", "communication planning"],
      ["Charter: purpose, objectives (SMART), scope in and out, milestones, budget, sponsor.",
       "Stakeholders: at least eight, placed on the power/interest grid with an engagement plan.",
       "WBS: deliverables broken into work packages of 8–80 hours.",
       "Schedule: activities and dependencies in project.json, run through the Office Move calculator.",
       "Risks and communication: ten risks with owners and responses; who gets which report how often."],
      ["A colleague could start the project from your documents, and the AI helper's review found no major gaps."],
      ["Review my charter's objectives: are they SMART?",
       "What risks am I missing for a Microsoft 365 migration?"],
      {"CHARTER.md": '''# Project charter

| | |
|---|---|
| Project | |
| Sponsor | |
| Project manager | |

## Purpose

## Objectives (SMART)
1.

## In scope / out of scope

## Milestones

## Budget

## Approval
''', "risks.csv": "id,risk,probability,impact,owner,response,action\nR1,ISP line delivered late,0.5,high,PM,mitigate,order the line in week 1"}),
]
