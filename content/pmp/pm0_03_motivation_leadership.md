@@lesson pm0-motivation-leadership Leading People: Motivation Theories, Situational Leadership, Power and Distributed Teams
@@concepts conditions, maps, lists
@@lang python
@@text
The People domain is 42% of the PMP exam, and many of its questions hide a theory behind a short scenario: a team member who has lost interest, a new hire who is keen but inexperienced, a sponsor who controls everything through their title, a team spread over three time zones. Recognising the theory tells you the answer. This lesson covers the motivation and leadership models the exam uses, the kinds of power, and how to run a distributed team, which is the normal case for IT projects in Japan with offshore partners.

## Motivation theories

- **Maslow's hierarchy of needs**: physiological → safety → social (belonging) → esteem → self-actualisation. People are motivated by the lowest unmet need: someone worried about their contract renewal (safety) won't be moved by a "rock star of the month" award (esteem).
- **Herzberg's two-factor theory**:
  - **hygiene factors** (pay, working conditions, policies, job security) don't motivate when present, but cause **dissatisfaction** when missing.
  - **motivators** (achievement, recognition, responsibility, growth, the work itself) create satisfaction.
  - Fix the hygiene factors first, then use the motivators.
- **McGregor's Theory X and Theory Y**: Theory X managers assume people dislike work and need close control. Theory Y managers assume people are self-motivated and seek responsibility. Agile and servant leadership assume Theory Y. (**Theory Z**, from Ouchi, adds long-term employment and collective decisions, inspired by Japanese management.)
- **McClelland's acquired needs**: **achievement** (challenging goals, feedback), **affiliation** (good relationships, teamwork) and **power** (influence, responsibility). Match assignments to the dominant need.
- **Vroom's expectancy theory**: motivation = expectancy (effort leads to performance) × instrumentality (performance leads to a reward) × valence (the reward matters to me). If any factor is zero, motivation is zero. A bonus nobody believes they can reach doesn't motivate.
- **Daniel Pink's Drive**: for knowledge work, **autonomy, mastery and purpose** beat carrots and sticks.

## Situational leadership

Hersey and Blanchard's model (SLII, Blanchard's version) matches the leader's style to the follower's **development level** for a particular task:

| Development level | Competence | Commitment | Style |
|---|---|---|---|
| D1 enthusiastic beginner | Low | High | **S1 Directing**: clear instructions, close supervision |
| D2 disillusioned learner | Low to some | Low | **S2 Coaching**: direct and support, explain decisions |
| D3 capable but cautious | Moderate to high | Variable | **S3 Supporting**: encourage, involve in decisions |
| D4 self-reliant achiever | High | High | **S4 Delegating**: hand over responsibility |

The same person can be D4 at coding and D1 at presenting to the steering committee. The exam rewards matching the style to the situation, and moving towards delegation as people grow.

## Leadership styles and power

- Styles you'll see: **servant leadership** (removing impediments, serving the team, the default answer in agile scenarios), **transformational** (inspiring with a vision), **transactional** (rewards for results), **laissez-faire** (hands-off, which is fine only with experienced self-managing teams), **autocratic/directive** (needed in a crisis, harmful as a habit), and **democratic/participative**.
- French and Raven's **types of power** (plus later additions):
  - **formal / legitimate**: from the position.
  - **reward** and **coercive** (punishment).
  - **expert**: knowledge and skill.
  - **referent**: respect and charisma.
  - **informational**: controlling information.
  - **relational**: networks and alliances.
- Project managers usually have little formal power in functional and matrix organisations, so expert and referent power, built through trust, are what they rely on. Coercive power is almost never the right answer.
- **Emotional intelligence** (Goleman): self-awareness, self-regulation, motivation, empathy and social skills. Start with yourself: you can't manage others' emotions well if you can't manage your own.

## Distributed and virtual teams

- Benefits: access to talent anywhere, lower costs, follow-the-sun work. Risks: less trust, misunderstandings, time zone pain, people feeling isolated.
- Practices that work:
  - A **team charter** with working agreements: core hours, response times, which channel for what.
  - Rotate meeting times so the same people aren't always on calls at 22:00.
  - Use video for relationship-building conversations.
  - Keep a shared written record of decisions.
  - Meet in person for kick-off and key milestones when the budget allows.
- **Fishbowl windows** (a long-running open video link between sites) and **remote pairing** keep distributed agile teams connected.
- Mind cultural differences: in Japanese teams, disagreement is often indirect, and silence in a meeting doesn't mean agreement. Ask individually, and use written rounds or anonymous input for decisions.

@@quiz
A developer is worried that their contract won't be renewed and shows no interest in a new recognition programme. Which theory best explains this?
* Maslow: an unmet safety need comes before esteem
- Theory Y
- Expectancy theory
- Herzberg's motivators
? Address the lower need first.

@@quiz
In Herzberg's theory, what happens when hygiene factors such as fair pay are missing?
* People become dissatisfied
- People become motivated
- Nothing changes
- People seek power
? Present hygiene factors prevent dissatisfaction but don't motivate.

@@quiz
A new team member is highly enthusiastic but has never used the team's cloud platform. Which situational leadership style fits?
* Directing
- Delegating
- Supporting
- Laissez-faire
? D1 enthusiastic beginner → S1.

@@quiz
Which type of power does a respected project manager rely on when team members follow them because they admire them?
- Coercive
- Legitimate
* Referent
- Reward
? Built on respect and trust.

@@quiz
A bonus is offered, but the team believes the target is impossible. According to Vroom, what is the effect?
* Little or no motivation, because expectancy is near zero
- High motivation because of the reward
- Motivation rises over time
- It depends only on valence
? Motivation is the product of the three factors.

@@quiz
A distributed team always meets at 9:00 Tokyo time, which is late evening in Europe. What should the project manager do first?
* Agree working agreements, including rotating meeting times, in the team charter
- Tell the European members to adapt
- Cancel all meetings
- Replace the European members
? Share the inconvenience fairly.

@@quiz
What do Theory Y managers assume about people?
* They are self-motivated and seek responsibility
- They dislike work and need close control
- They only work for money
- They need coercive power
? The basis of servant leadership.

@@flashcards
Maslow :: physiological, safety, social, esteem, self-actualisation
Herzberg hygiene :: pay, conditions, policies: missing → dissatisfaction
Herzberg motivators :: achievement, recognition, growth, the work itself
Theory X / Y :: control / trust and responsibility
Theory Z :: long-term employment, collective decisions
McClelland :: achievement, affiliation, power
Vroom :: expectancy × instrumentality × valence
Pink's Drive :: autonomy, mastery, purpose
D1 → S1 :: enthusiastic beginner → directing
D2 → S2 :: disillusioned learner → coaching
D3 → S3 :: capable but cautious → supporting
D4 → S4 :: self-reliant achiever → delegating
referent power :: respect and charisma
expert power :: knowledge and skill
coercive power :: punishment, rarely the right answer
emotional intelligence :: self-awareness, self-regulation, motivation, empathy, social skills
team charter :: working agreements, including core hours

@@drill
Which situational leadership style suits a highly competent and committed team member?
= delegating
= Delegating
= S4
? D4 self-reliant achiever.

@@drill
In Vroom's model, expectancy 0.8, instrumentality 0.5, valence 0. What is the motivation score?
= 0
? Any zero factor makes it zero.

@@exercise Choose a leadership style
Classify each person's development level for a task from their competence and commitment (each "low", "some", "high" or "variable"), choose the matching style, and suggest what to do next. Competence low with high commitment is D1; low or some competence with low commitment is D2; some or high competence with variable commitment is D3; high with high is D4.
@@starter
STYLES = {
    "D1": ("Directing", "give clear steps and check in often"),
    "D2": ("Coaching", "explain the why, keep directing, and encourage"),
    "D3": ("Supporting", "involve them in decisions and build confidence"),
    "D4": ("Delegating", "hand over the task and agree how to report back"),
}
people = [
    ("Aiko", "Terraform modules", "high", "high"),
    ("Ben", "Terraform modules", "low", "high"),
    ("Chen", "presenting to the steering committee", "some", "low"),
    ("Dana", "on-call incident lead", "high", "variable"),
    ("Eiji", "estimating stories", "some", "variable"),
]

def development_level(competence, commitment):
    raise NotImplementedError

for name, task, comp, comm in people:
    level = development_level(comp, comm)
    style, action = STYLES[level]
    print(f"{name:<5} {task:<38} {level} -> {style}: {action}")
@@expected
Aiko  Terraform modules                      D4 -> Delegating: hand over the task and agree how to report back
Ben   Terraform modules                      D1 -> Directing: give clear steps and check in often
Chen  presenting to the steering committee   D2 -> Coaching: explain the why, keep directing, and encourage
Dana  on-call incident lead                  D3 -> Supporting: involve them in decisions and build confidence
Eiji  estimating stories                     D3 -> Supporting: involve them in decisions and build confidence
@@requires
def development_level
commitment ==
@@hint
Check in this order: `commitment == "variable"` → "D3"; competence "high" and commitment "high" → "D4"; commitment "low" → "D2"; otherwise "D1".
@@solution
STYLES = {
    "D1": ("Directing", "give clear steps and check in often"),
    "D2": ("Coaching", "explain the why, keep directing, and encourage"),
    "D3": ("Supporting", "involve them in decisions and build confidence"),
    "D4": ("Delegating", "hand over the task and agree how to report back"),
}
people = [
    ("Aiko", "Terraform modules", "high", "high"),
    ("Ben", "Terraform modules", "low", "high"),
    ("Chen", "presenting to the steering committee", "some", "low"),
    ("Dana", "on-call incident lead", "high", "variable"),
    ("Eiji", "estimating stories", "some", "variable"),
]

def development_level(competence, commitment):
    if commitment == "variable":
        return "D3"
    if competence == "high" and commitment == "high":
        return "D4"
    if commitment == "low":
        return "D2"
    return "D1"

for name, task, comp, comm in people:
    level = development_level(comp, comm)
    style, action = STYLES[level]
    print(f"{name:<5} {task:<38} {level} -> {style}: {action}")

@@exercise Find meeting slots for a distributed team
Team members work 9:00–18:00 local time on weekdays, in Tokyo (UTC+9), Berlin (UTC+1 in winter) and New York (UTC−5 in winter). Find the UTC hours where all three are inside working hours, the hours where only two are, and propose a fair rotation: when no all-hands hour exists, rotate the meeting across the three pairwise-overlap slots, and show what local time it is for everyone. (In winter Tokyo and New York working hours don't overlap at all.)
@@starter
OFFSETS = {"Tokyo": 9, "Berlin": 1, "New York": -5}
START, END = 9, 18

def working(city, utc_hour):
    raise NotImplementedError

def overlaps():
    raise NotImplementedError

def local_times(utc_hour):
    return ", ".join(f"{city} {(utc_hour + off) % 24:02d}:00" for city, off in OFFSETS.items())

everyone, pairs = overlaps()
print("all three:", everyone or "none")
for pair, hours in pairs.items():
    print(f"{' + '.join(pair)}: UTC {hours}")
print("rotation:")
for pair, hours in pairs.items():
    if hours:
        print(f"  week for {' + '.join(pair)}: UTC {hours[0]:02d}:00 -> {local_times(hours[0])}")
@@expected
all three: none
Tokyo + Berlin: UTC [8]
Tokyo + New York: UTC []
Berlin + New York: UTC [14, 15, 16]
rotation:
  week for Tokyo + Berlin: UTC 08:00 -> Tokyo 17:00, Berlin 09:00, New York 03:00
  week for Berlin + New York: UTC 14:00 -> Tokyo 23:00, Berlin 15:00, New York 09:00
@@requires
def working
def overlaps
combinations(
@@hint
`working`: `local = (utc_hour + OFFSETS[city]) % 24` and check `START <= local < END`. In `overlaps`, loop over `range(24)`: hours where all three cities work go in `everyone`; for each pair of cities (use `itertools.combinations` or write them out), list the hours where both work.
@@solution
from itertools import combinations

OFFSETS = {"Tokyo": 9, "Berlin": 1, "New York": -5}
START, END = 9, 18

def working(city, utc_hour):
    local = (utc_hour + OFFSETS[city]) % 24
    return START <= local < END

def overlaps():
    everyone = [h for h in range(24) if all(working(c, h) for c in OFFSETS)]
    pairs = {}
    for pair in combinations(OFFSETS, 2):
        pairs[pair] = [h for h in range(24) if all(working(c, h) for c in pair)]
    return everyone, pairs

def local_times(utc_hour):
    return ", ".join(f"{city} {(utc_hour + off) % 24:02d}:00" for city, off in OFFSETS.items())

everyone, pairs = overlaps()
print("all three:", everyone or "none")
for pair, hours in pairs.items():
    print(f"{' + '.join(pair)}: UTC {hours}")
print("rotation:")
for pair, hours in pairs.items():
    if hours:
        print(f"  week for {' + '.join(pair)}: UTC {hours[0]:02d}:00 -> {local_times(hours[0])}")
