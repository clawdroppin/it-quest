@@lesson pm2-exam-strategy Exam Strategy: Situational Questions, Pacing and a Study Plan
@@text
Knowing the material is half of passing the PMP. The other half is answering long situational questions under time pressure, where two or three options sound right. This lesson collects the decision rules that resolve most of those questions, explains the exam's format and pacing, and builds a study plan from your practice-test results.

## Format

About **180 questions in 230 minutes**, with two optional 10-minute breaks (after roughly questions 60 and 120; you can't go back to a finished section). Question types: single choice, **multiple responses** ("choose two"), matching, hotspot and fill-in-the-blank calculations. Some questions are unscored pretest items. The result is reported per domain as *Above Target*, *Target*, *Below Target* or *Needs Improvement*.

Pacing: 230 / 180 ≈ **77 seconds per question**. Aim to finish each third in about 70 minutes, leaving time to review flagged questions.

## Decision rules for situational questions

1. **Read the last sentence first**: what is actually asked (first, next, best, most likely)?
2. Identify the **approach**: predictive, agile or hybrid. Agile answers favour the team self-organising and the product owner prioritising.
3. **Assess before acting**: gather information, review the plan, analyse impact, talk to the person, before escalating or changing anything.
4. **Servant leadership**: coach, empower, remove impediments; don't command or do the team's work.
5. **Follow the process** but don't hide behind it: change requests through change control; issues into the issue log; risks into the risk register.
6. **Deal with problems directly and privately**, then escalate only if needed (to the sponsor or functional manager).
7. **Collaborate / problem-solve** is the preferred conflict approach; avoid "withdraw" and "force" unless safety or urgency demands.
8. **Ethics**: honesty, transparency, follow laws and the PMI Code of Ethics; report violations; never accept gifts that create a conflict of interest.
9. Eliminate answers that **blame**, **ignore** the problem, **skip** stakeholders or **break** the contract.

## A study plan

- Base: the PMBOK Guide (8th edition, principles and performance domains), the *Process Groups: A Practice Guide*, and the *Agile Practice Guide*, plus a 35-contact-hour course (required to apply).
- Take a full timed practice exam early to find weak domains; then study weakest-first, weighted by domain size.
- Track scores by domain and by question type; review every wrong answer and write the rule it taught you.
- Aim for consistently 70%+ on reputable full-length practice exams before booking.

@@quiz
How long do you have per question on average?
- About 30 seconds
* About 77 seconds
- About 2 minutes
- Unlimited
? 230 minutes / 180 questions.

@@quiz
A team member says a task will be late. What should the project manager do first?
- Escalate to the sponsor
- Assign someone else
* Talk with the team member to understand the cause and the impact
- Crash the schedule
? Assess before acting.

@@quiz
In an agile project, the customer wants a new feature mid-sprint. What happens?
- The team adds it immediately
* The product owner evaluates it and orders it in the backlog
- The PM rejects it
- A change control board meets
? The sprint goal is protected.

@@quiz
Which answer type is usually wrong on PMP situational questions?
- Collaborating with the stakeholder
- Reviewing the risk register
* Ignoring the problem and hoping it resolves itself
- Coaching the team
? Eliminate ignore, blame and skip answers.

@@quiz
A vendor offers you an expensive gift during a contract negotiation. What do you do?
- Accept it politely
* Decline it and follow the organisation's policy
- Accept it and share it with the team
- Tell the vendor to wait until after signing
? Avoid conflicts of interest.

@@flashcards
PMP format :: about 180 questions, 230 minutes
breaks :: two optional 10-minute breaks
pace :: about 77 s per question
read first :: the last sentence
assess before acting :: gather information first
servant leader :: coach, empower, remove impediments
escalate :: only after addressing directly
preferred conflict mode :: collaborate / problem solve
agile change :: product owner reorders backlog
ethics :: honesty, responsibility, respect, fairness
eliminate :: blame, ignore, skip, break contract
35 contact hours :: required to apply
results :: per-domain performance ratings

@@drill
About how many seconds should you spend per PMP question?
= 77
= 76
= 76.7
= 77 seconds
? 230 × 60 / 180.

@@drill
What is the preferred conflict resolution technique on the PMP exam?
= collaborate
= problem solving
= collaborate/problem solve
= collaborating
= problem solve
? It addresses the root cause.

@@exercise Pace yourself through the exam
Given a log of question timings from a practice exam (seconds per question), compute the time used in each third, compare it with the target pace (77 s per question), flag the stretch where you fell furthest behind, and estimate how many questions would have been left unanswered if the clock ran out at 230 minutes.
@@lang python
@@starter
import random

random.seed(21)
timings = [random.randint(45, 110) for _ in range(60)] + [random.randint(60, 140) for _ in range(60)] + [random.randint(40, 90) for _ in range(60)]
TOTAL_MIN = 230
TARGET = TOTAL_MIN * 60 / 180

def by_section(timings):
    raise NotImplementedError
@@expected
target pace 76.7 s per question, 76.7 min per 60-question section
section 1: 79.3 min (+2.7 min vs target)
section 2: 100.1 min (+23.4 min vs target)
section 3: 64.7 min (-12.0 min vs target)
furthest behind: 26.1 min after question 120
total 244.1 min; answered 167 of 180 in 230 min, 13 unanswered
@@requires
def by_section
TARGET
TOTAL_MIN
sum(
@@hint
Split into three slices of 60. For each, the target is 60 × TARGET seconds; print minutes used and the difference. Walk a running total to find the question where you were most behind schedule (`elapsed − n × TARGET` largest). Unanswered = questions remaining when the running total passes `TOTAL_MIN × 60`.
@@solution
import random

random.seed(21)
timings = [random.randint(45, 110) for _ in range(60)] + [random.randint(60, 140) for _ in range(60)] + [random.randint(40, 90) for _ in range(60)]
TOTAL_MIN = 230
TARGET = TOTAL_MIN * 60 / 180

def by_section(timings):
    return [timings[i:i + 60] for i in range(0, len(timings), 60)]

print(f"target pace {TARGET:.1f} s per question, {60 * TARGET / 60:.1f} min per 60-question section")
for n, section in enumerate(by_section(timings), 1):
    used = sum(section)
    print(f"section {n}: {used / 60:.1f} min ({(used - 60 * TARGET) / 60:+.1f} min vs target)")
elapsed, worst, worst_at, answered = 0, 0.0, 0, 0
for q, t in enumerate(timings, 1):
    elapsed += t
    behind = elapsed - q * TARGET
    if behind > worst:
        worst, worst_at = behind, q
    if elapsed <= TOTAL_MIN * 60:
        answered = q
print(f"furthest behind: {worst / 60:.1f} min after question {worst_at}")
print(f"total {elapsed / 60:.1f} min; answered {answered} of 180 in {TOTAL_MIN} min, {180 - answered} unanswered")

@@exercise Build a study plan from practice results
From practice-test results by domain, compute each domain's score, then allocate study hours in proportion to `exam weight × (target − score)` (domains already at or above the target get a minimum maintenance share), and say whether you're ready to book (all domains at or above 70% and the overall score at least 72%).
@@lang python
@@starter
WEIGHTS = {"People": 0.33, "Process": 0.41, "Business Environment": 0.26}
results = {"People": (48, 62), "Process": (49, 76), "Business Environment": (22, 42)}  # (correct, asked)
TARGET = 0.75
HOURS = 30
MAINTENANCE = 0.1

def plan(results):
    raise NotImplementedError
@@expected
People                77%  study 3.0 h
Process               64%  study 11.4 h
Business Environment  52%  study 15.6 h
overall 66%: not yet: focus on the weakest domains first
@@requires
def plan
WEIGHTS
MAINTENANCE
max(
@@hint
Score = correct / asked. Need = `WEIGHTS[d] * max(TARGET - score, 0)`. Give domains with no need `MAINTENANCE` of the hours each, and split the rest of the hours by need. Overall = total correct / total asked.
@@solution
WEIGHTS = {"People": 0.33, "Process": 0.41, "Business Environment": 0.26}
results = {"People": (48, 62), "Process": (49, 76), "Business Environment": (22, 42)}  # (correct, asked)
TARGET = 0.75
HOURS = 30
MAINTENANCE = 0.1

def plan(results):
    scores = {d: c / a for d, (c, a) in results.items()}
    need = {d: WEIGHTS[d] * max(TARGET - s, 0) for d, s in scores.items()}
    fixed = [d for d in need if need[d] == 0]
    remaining = HOURS * (1 - MAINTENANCE * len(fixed))
    total_need = sum(need.values())
    hours = {d: (HOURS * MAINTENANCE if d in fixed else remaining * need[d] / total_need) for d in need}
    return scores, hours

scores, hours = plan(results)
for d in WEIGHTS:
    print(f"{d:<21} {scores[d]:.0%}  study {hours[d]:.1f} h")
overall = sum(c for c, _ in results.values()) / sum(a for _, a in results.values())
ready = overall >= 0.72 and all(s >= 0.70 for s in scores.values())
print(f"overall {overall:.0%}: {'ready to book' if ready else 'not yet: focus on the weakest domains first'}")
