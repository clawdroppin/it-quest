@@lesson xp-sprint_metrics Sprint Metrics Dashboard
@@text
Agile project managers forecast with data, not hope. From sprints.csv (planned and completed story points per sprint) and the remaining backlog, compute the velocity average and range, draw a text burndown for the current sprint, and forecast how many sprints the backlog needs, as a range ("6 to 9 sprints"), not a single date.

**You'll practise:** velocity · burndown and burnup · forecasting with ranges · agile metrics and their misuse

## Milestones

1. **Data**: sprints.csv and burndown.csv (day, points remaining).
2. **Velocity**: average of the last 3 sprints, and the lowest and highest.
3. **Burndown**: a text chart of remaining points against the ideal line.
4. **Forecast**: backlog ÷ high, average and low velocity → a range of sprints and dates.
5. **Explain**: in NOTES.md, why velocity must never be used to compare teams.

## You're done when

- The forecast range is correct for the sample and the burndown shows whether the sprint is on track.

## Getting help

Open the project, then the 🤖 **AI Helper**: it sees the lesson and your files. Good questions to start with:

- *"Why is a forecast range better than a single date?"*
- *"What are the risks of using velocity as a performance target?"*

Ask for hints, reviews and explanations rather than finished code: you learn by writing it.
@@project sprint_metrics Sprint Metrics Dashboard
Create the folder, open it in VS Code, and work through `README.md` one milestone at a time.
@@file README.md
# Sprint Metrics Dashboard

Agile project managers forecast with data, not hope. From sprints.csv (planned and completed story points per sprint) and the remaining backlog, compute the velocity average and range, draw a text burndown for the current sprint, and forecast how many sprints the backlog needs, as a range ("6 to 9 sprints"), not a single date.

## Milestones

- [ ] 1. Data: sprints.csv and burndown.csv (day, points remaining).
- [ ] 2. Velocity: average of the last 3 sprints, and the lowest and highest.
- [ ] 3. Burndown: a text chart of remaining points against the ideal line.
- [ ] 4. Forecast: backlog ÷ high, average and low velocity → a range of sprints and dates.
- [ ] 5. Explain: in NOTES.md, why velocity must never be used to compare teams.

## Done when

- [ ] The forecast range is correct for the sample and the burndown shows whether the sprint is on track.
@@file metrics.py
"""Sprint metrics: python metrics.py"""
import csv
import statistics

sprints = list(csv.DictReader(open("sprints.csv", encoding="utf-8")))
done = [int(s["completed"]) for s in sprints]
last3 = done[-3:]
print("velocity (last 3):", statistics.mean(last3), "range", min(last3), "-", max(last3))
# TODO: burndown chart, forecast range for the backlog
@@file sprints.csv
sprint,planned,completed
1,30,22
2,28,25
3,30,31
4,32,27
5,30,29
@@file burndown.csv
day,remaining
0,30
1,28
2,27
3,27
4,22
5,18
6,17
7,12
