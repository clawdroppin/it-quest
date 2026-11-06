@@lesson pm1-flow-forecasting Flow and Forecasting: Kanban, WIP Limits, Little's Law and Monte Carlo
@@concepts numbers, lists, loops
@@lang python
@@text
"When will it be done?" is the question every sponsor asks. Predictive projects answer with a critical path and PERT. Agile teams answer with velocity. Teams that work in a continuous flow (operations, support, platform teams, many maintenance projects) need a third way: **flow metrics** and **probabilistic forecasting**. The PMP exam expects you to know Kanban, WIP limits, cumulative flow diagrams and Little's law, and Monte Carlo simulation appears in the risk and schedule questions.

## Kanban essentials

- Kanban (from Toyota's production system, かんばん, "signboard") **visualises work** on a board with columns for each state (Ready → In progress → Review → Done).
- Its core practices:
  - visualise the workflow.
  - **limit work in progress (WIP)**.
  - manage flow.
  - make policies explicit (e.g. "definition of done" per column).
  - implement feedback loops.
  - improve collaboratively.
- Kanban is a **pull system**: a person pulls the next item when they have capacity, instead of work being pushed onto them.
- **WIP limits** cap how many items can be in a column. They expose bottlenecks (work piles up before the constrained step), reduce context switching, and finish things sooner. "Stop starting, start finishing."
- **Classes of service** handle different urgency: expedite (one at a time, jumps the queue), fixed date, standard and intangible (improvements).

## Flow metrics

- **Cycle time**: from when work starts on an item to when it's done. **Lead time**: from when it was requested to when it's done (the customer's view).
- **Throughput**: items finished per unit of time (per week).
- **WIP**: items started but not finished.
- **Little's law** links them for a stable system: **average cycle time = average WIP ÷ average throughput**. Halve the WIP at the same throughput and the cycle time halves too. That's why WIP limits work.
- **Cumulative flow diagram (CFD)**: a stacked area chart of how many items are in each state over time.
  - The vertical distance between bands is WIP.
  - The horizontal distance is approximately cycle time.
  - A widening band means a bottleneck.
  - Flat lines mean nothing is finishing.
- **Flow efficiency** = active work time ÷ total cycle time. Values of 15% or less are common: most time is spent waiting.

## Forecasting with probability

- Single-number estimates hide uncertainty. Better: "85% likely by 12 March."
- **Monte Carlo simulation**: use real historical data (e.g. weekly throughput samples), randomly sample many possible futures (thousands of runs), and look at the distribution of outcomes.
  - For "when will 60 items be done?", each run samples a past week's throughput repeatedly until the total reaches 60, and records the number of weeks.
  - The **percentiles** of the results give the forecast. Quote the 85th percentile for commitments (85% of simulations finished by then), and the 50th is a coin toss.
- The same method works for predictive schedules: sample each activity's duration from its three-point estimate and recompute the critical path each run. The result shows the probability of meeting a date and which activities most often end up critical (a **criticality index**).
- Monte Carlo is honest only if the inputs are: use recent data from the same team doing similar work, and rerun it as new weeks complete.

## Exam patterns

- A team is overwhelmed, many items are started, few finished → **introduce or lower WIP limits**.
- Work piles up in "Testing" → the **bottleneck** is testing. Swarm on it and don't start more development.
- The sponsor wants a firm date for a flow-based team → give a **probabilistic forecast** based on throughput, not a guess.

@@quiz
A team has 24 items in progress and finishes 8 items a week. What is the average cycle time according to Little's law?
* 3 weeks
- 8 weeks
- 0.33 weeks
- 32 weeks
? WIP ÷ throughput = 24 ÷ 8.

@@quiz
In a cumulative flow diagram, the "In testing" band keeps getting wider. What does it show?
* A bottleneck in testing: work arrives faster than it leaves
- Testing is finished
- Throughput is rising
- The team should start more work
? Swarm on testing.

@@quiz
What is the main purpose of WIP limits?
* To reduce multitasking and expose bottlenecks so work finishes sooner
- To keep everyone busy all the time
- To make the board look tidy
- To limit the backlog size
? Stop starting, start finishing.

@@quiz
A Monte Carlo forecast gives 9 weeks at the 50th percentile and 12 weeks at the 85th. What should you commit to?
* 12 weeks, stating the 85% confidence
- 9 weeks
- 10.5 weeks
- The best case, 6 weeks
? The 50th percentile is a coin toss.

@@quiz
What is the difference between lead time and cycle time?
* Lead time starts at the request, cycle time when work starts
- They are the same
- Cycle time includes waiting before work starts
- Lead time only counts weekends
? Lead time is the customer's view.

@@quiz
Which statement describes a pull system?
* Team members take new work when they have capacity
- Managers assign work as soon as it arrives
- Work is pushed to the next stage when it is done
- Work is assigned by seniority
? Kanban is a pull system.

@@quiz
Flow efficiency is 12%. What does that mean?
* Items are actively worked on for only 12% of their cycle time; the rest is waiting
- 12% of items fail
- The team works 12% of the time
- Throughput is 12 items a week
? Reduce queues and handoffs.

@@flashcards
Kanban :: visualise work, limit WIP, manage flow
pull system :: take work when you have capacity
WIP limit :: maximum items in a column
classes of service :: expedite, fixed date, standard, intangible
cycle time :: work started → done
lead time :: requested → done
throughput :: items finished per period
Little's law :: cycle time = WIP ÷ throughput
CFD :: stacked states over time
CFD vertical gap :: WIP
CFD horizontal gap :: cycle time
widening band :: bottleneck
flow efficiency :: active time ÷ cycle time
Monte Carlo :: sample history many times
85th percentile :: common commitment level
criticality index :: how often an activity is critical

@@drill
WIP is 30 items and the average cycle time is 5 days. What is the throughput in items per day?
= 6
? 30 ÷ 5.

@@drill
Which percentile is commonly used to commit to a Monte Carlo forecast? (number only)
= 85
? Some use 90 or 95 for critical dates.

@@exercise Flow metrics from a Kanban board
From the log of items (requested, started and finished days), compute lead time and cycle time per finished item, average cycle time, throughput per week over the period, average WIP (items in progress at the end of each day), and check Little's law. Expect the two cycle times to differ: two items are still unfinished, and a short period is never perfectly stable, which is why Little's law is a guide for averages over time rather than an exact formula for a three-week sample. Then find the column whose WIP limit is exceeded on the board snapshot.
@@starter
items = [
    ("A", 0, 1, 6), ("B", 0, 2, 9), ("C", 1, 3, 8), ("D", 2, 4, 12), ("E", 3, 6, 13),
    ("F", 4, 7, 15), ("G", 6, 9, 17), ("H", 7, 11, 20), ("I", 9, 12, None), ("J", 10, 14, None),
]
PERIOD = 21
board = {"Ready": 4, "In progress": 3, "Review": 5, "Testing": 2}
limits = {"In progress": 4, "Review": 3, "Testing": 3}

def flow_metrics(items, period):
    raise NotImplementedError

m = flow_metrics(items, PERIOD)
for name, value in m.items():
    print(f"{name:<26} {value}")
print("over WIP limit:", [col for col, n in board.items() if col in limits and n > limits[col]])
@@expected
finished items             8
average lead time (days)   9.6
average cycle time (days)  7.1
throughput per week        2.67
average WIP                3.48
Little's law cycle time    9.1
over WIP limit: ['Review']
@@requires
def flow_metrics
throughput
@@hint
Finished items have a `finished` day. Cycle time = finished − started; lead time = finished − requested. Throughput per week = finished count ÷ (period ÷ 7). WIP at the end of day d counts items with `started <= d` and (unfinished or `finished > d`); average it over days 0 to period − 1. Little's law predicts cycle time = WIP ÷ throughput per day.
@@solution
items = [
    ("A", 0, 1, 6), ("B", 0, 2, 9), ("C", 1, 3, 8), ("D", 2, 4, 12), ("E", 3, 6, 13),
    ("F", 4, 7, 15), ("G", 6, 9, 17), ("H", 7, 11, 20), ("I", 9, 12, None), ("J", 10, 14, None),
]
PERIOD = 21
board = {"Ready": 4, "In progress": 3, "Review": 5, "Testing": 2}
limits = {"In progress": 4, "Review": 3, "Testing": 3}

def flow_metrics(items, period):
    done = [i for i in items if i[3] is not None]
    cycle = [f - s for _, _, s, f in done]
    lead = [f - r for _, r, _, f in done]
    throughput_day = len(done) / period
    wip_days = [sum(1 for _, _, s, f in items if s <= d and (f is None or f > d)) for d in range(period)]
    avg_wip = sum(wip_days) / period
    return {
        "finished items": len(done),
        "average lead time (days)": round(sum(lead) / len(lead), 1),
        "average cycle time (days)": round(sum(cycle) / len(cycle), 1),
        "throughput per week": round(throughput_day * 7, 2),
        "average WIP": round(avg_wip, 2),
        "Little's law cycle time": round(avg_wip / throughput_day, 1),
    }

m = flow_metrics(items, PERIOD)
for name, value in m.items():
    print(f"{name:<26} {value}")
print("over WIP limit:", [col for col, n in board.items() if col in limits and n > limits[col]])

@@exercise Monte Carlo: when will the backlog be done?
Using the team's last 12 weeks of throughput, simulate 5,000 futures (with `random.Random(42)` so results are repeatable): each run samples a past week's throughput, with replacement, until the remaining 60 items are done, and records the number of weeks. Print the 50th, 85th and 95th percentiles and a small text histogram, and compare with the naive average-based estimate.
@@starter
import math
import random

history = [6, 9, 4, 7, 8, 3, 7, 10, 5, 6, 8, 2]
BACKLOG = 60
RUNS = 5000

def simulate(history, backlog, runs, seed=42):
    raise NotImplementedError

def percentile(sorted_values, p):
    raise NotImplementedError

results = sorted(simulate(history, BACKLOG, RUNS))
naive = math.ceil(BACKLOG / (sum(history) / len(history)))
print(f"naive estimate: {naive} weeks")
for p in (50, 85, 95):
    print(f"P{p}: {percentile(results, p)} weeks")
counts = {}
for w in results:
    counts[w] = counts.get(w, 0) + 1
for weeks in sorted(counts):
    print(f"{weeks:>3} weeks {'#' * round(counts[weeks] / RUNS * 100)}".rstrip())
@@expected
naive estimate: 10 weeks
P50: 10 weeks
P85: 11 weeks
P95: 12 weeks
  7 weeks
  8 weeks #######
  9 weeks #########################
 10 weeks ##################################
 11 weeks ######################
 12 weeks #########
 13 weeks ##
 14 weeks
 15 weeks
@@requires
def simulate
def percentile
rng.choice(
@@hint
`rng = random.Random(seed)`. For each run, set `done, weeks = 0, 0` and loop `while done < backlog: done += rng.choice(history); weeks += 1`, then append `weeks`. For a percentile of a sorted list, take the value at index `math.ceil(p / 100 * len(values)) - 1`.
@@solution
import math
import random

history = [6, 9, 4, 7, 8, 3, 7, 10, 5, 6, 8, 2]
BACKLOG = 60
RUNS = 5000

def simulate(history, backlog, runs, seed=42):
    rng = random.Random(seed)
    results = []
    for _ in range(runs):
        done, weeks = 0, 0
        while done < backlog:
            done += rng.choice(history)
            weeks += 1
        results.append(weeks)
    return results

def percentile(sorted_values, p):
    return sorted_values[math.ceil(p / 100 * len(sorted_values)) - 1]

results = sorted(simulate(history, BACKLOG, RUNS))
naive = math.ceil(BACKLOG / (sum(history) / len(history)))
print(f"naive estimate: {naive} weeks")
for p in (50, 85, 95):
    print(f"P{p}: {percentile(results, p)} weeks")
counts = {}
for w in results:
    counts[w] = counts.get(w, 0) + 1
for weeks in sorted(counts):
    print(f"{weeks:>3} weeks {'#' * round(counts[weeks] / RUNS * 100)}".rstrip())
