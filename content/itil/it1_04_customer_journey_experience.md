@@lesson it1-customer-journey The Customer Journey and Experience: Touchpoints, Moments of Truth, XLAs and Feedback
@@concepts maps, numbers, functions
@@lang python
@@text
ITIL 4 goes beyond the Foundation's value chain in its **Drive Stakeholder Value (DSV)** module, which follows the whole **customer journey**: from the first contact with a provider to ongoing use and renewal. Even at Foundation level you'll see its ideas: value is **co-created**, and what users experience matters as much as what the SLA says. This lesson covers the journey, how to measure experience, and how to act on feedback.

## The customer journey

DSV describes the journey in steps:

1. **Explore**: understand the market, customers and their needs, and how to reach them.
2. **Engage**: build relationships and trust, and communicate clearly.
3. **Offer**: shape demand and design service offerings that match needs.
4. **Agree**: agree on the service, the expected value, and how it will be measured.
5. **Onboard**: get users started: accounts, training, data migration.
6. **Co-create**: deliver and use the service together, day to day.
7. **Realise**: confirm the value has been achieved, and improve.

- **Touchpoints** are every interaction between the customer and the provider: the portal, a phone call, an email notification, a technician's visit.
- **Moments of truth** are the touchpoints that shape the customer's opinion most: the first day with a new laptop, how a major outage is communicated, how a complaint is handled.
- A **customer journey map** shows the steps, touchpoints, what the user feels at each one, and the pain points. It's made with real users, not guessed in a meeting room.

## Experience vs service levels

- **SLAs** measure what the provider controls: response time, availability, resolution time. They can all be met while users are frustrated: the **watermelon effect** (green outside, red inside).
- **XLAs** (experience level agreements, a term used in industry rather than in the ITIL core) add experience measures: satisfaction, **effort** (how hard it was to get help), lost productive time, sentiment.
- Common measures:
  - **CSAT**: satisfaction with a single interaction, e.g. 1–5 after each ticket.
  - **NPS** (net promoter score): "How likely are you to recommend us?" on a scale of 0–10. Promoters score 9–10, passives 7–8, detractors 0–6, and NPS = % promoters − % detractors, from −100 to +100.
  - **CES** (customer effort score): "How easy was it to get your issue resolved?" Low effort predicts loyalty better than delight does.
  - **Lost productive time**: hours users couldn't work, which is often the measure executives care about most.
- Combine **operational data** (O-data: tickets, uptime) with **experience data** (X-data: surveys, sentiment). The gap between them is where improvement work belongs.

## Acting on feedback

- **Close the loop**: tell users what changed because of their feedback, or they'll stop giving it. Contact detractors personally when you can.
- Look for patterns: low scores clustered at one touchpoint (e.g. the first day for new joiners) point to the moment of truth to fix.
- Beware **survey fatigue**: ask briefly, at the right moment, and not after every single interaction.
- Low response rates bias results: angry and very happy users answer more than the middle.

## Relationship management

- The **relationship management** practice establishes and nurtures links between the organisation and its stakeholders at strategic and tactical levels.
- Regular **service reviews** with customers discuss SLA results, experience data, upcoming changes and improvement plans, which is much more useful than emailing a report.
- In Japan, regular face-to-face 定例会 (regular review meetings) with clients are expected. Bring data, an agenda and clear actions.

@@quiz
What is a moment of truth in the customer journey?
* A touchpoint that strongly shapes the customer's opinion of the service
- The moment a contract is signed
- The end of an SLA period
- An audit finding
? E.g. a new joiner's first day.

@@quiz
All SLA targets are green, but users are frustrated. What is this called?
* The watermelon effect
- Shift-left
- Value leakage
- Moment of truth
? Green outside, red inside.

@@quiz
In NPS, which scores count as detractors?
* 0–6
- 0–4
- 7–8
- 9–10
? Promoters 9–10, passives 7–8.

@@quiz
What does a customer effort score measure?
* How easy it was for the customer to get what they needed
- How many hours the provider worked
- How much the service costs
- How many tickets were closed
? Low effort predicts loyalty.

@@quiz
In which customer journey step do users get accounts, training and their data migrated?
- Explore
- Agree
* Onboard
- Realise
? Getting started.

@@quiz
Why "close the loop" on feedback?
* Users who see their feedback acted on keep giving it, and trust grows
- To reduce the number of surveys
- Because the SLA requires it
- To raise NPS automatically
? Tell them what changed.

@@flashcards
DSV :: Drive Stakeholder Value module
journey steps :: explore, engage, offer, agree, onboard, co-create, realise
touchpoint :: any interaction between customer and provider
moment of truth :: touchpoint that shapes opinion most
journey map :: steps, touchpoints, feelings and pain points
SLA :: provider-controlled service levels
XLA :: experience level agreement
watermelon effect :: green SLAs, unhappy users
CSAT :: satisfaction with one interaction
NPS :: % promoters (9–10) − % detractors (0–6)
CES :: how easy it was
lost productive time :: hours users couldn't work
O-data / X-data :: operational / experience data
close the loop :: tell users what changed
survey fatigue :: too many surveys, fewer answers

@@drill
40% promoters, 35% passives and 25% detractors. What is the NPS?
= 15
= +15
? 40 − 25.

@@drill
Which customer journey step is about confirming the value was achieved?
= realise
= Realise
= realize
? The last step.

@@exercise Compute NPS, CSAT and CES by touchpoint
From survey responses, compute the NPS overall, and the average CSAT and CES for each touchpoint. Then name the moment of truth that needs attention: the touchpoint with the lowest average CSAT among those with at least 3 responses.
@@starter
responses = [
    {"touchpoint": "onboarding", "nps": 4, "csat": 2, "ces": 2},
    {"touchpoint": "onboarding", "nps": 6, "csat": 3, "ces": 3},
    {"touchpoint": "onboarding", "nps": 7, "csat": 2, "ces": 2},
    {"touchpoint": "portal", "nps": 9, "csat": 5, "ces": 6},
    {"touchpoint": "portal", "nps": 10, "csat": 4, "ces": 6},
    {"touchpoint": "portal", "nps": 8, "csat": 4, "ces": 5},
    {"touchpoint": "outage comms", "nps": 3, "csat": 1, "ces": 3},
    {"touchpoint": "outage comms", "nps": 9, "csat": 4, "ces": 5},
    {"touchpoint": "phone", "nps": 10, "csat": 5, "ces": 7},
    {"touchpoint": "phone", "nps": 9, "csat": 5, "ces": 6},
    {"touchpoint": "phone", "nps": 6, "csat": 3, "ces": 4},
]

def nps(scores):
    raise NotImplementedError

def by_touchpoint(responses):
    raise NotImplementedError

print(f"NPS {nps([r['nps'] for r in responses]):+d}")
stats = by_touchpoint(responses)
for tp, (n, csat, ces) in stats.items():
    print(f"  {tp:<13} n={n}  CSAT {csat:.1f}/5  CES {ces:.1f}/7")
eligible = {tp: s for tp, s in stats.items() if s[0] >= 3}
worst = min(eligible, key=lambda tp: eligible[tp][1])
print("moment of truth to fix:", worst)
@@expected
NPS +9
  onboarding    n=3  CSAT 2.3/5  CES 2.3/7
  portal        n=3  CSAT 4.3/5  CES 5.7/7
  outage comms  n=2  CSAT 2.5/5  CES 4.0/7
  phone         n=3  CSAT 4.3/5  CES 5.7/7
moment of truth to fix: onboarding
@@requires
def nps
def by_touchpoint
>= 9
@@hint
`nps`: promoters are scores `>= 9`, detractors `<= 6`; return `round((promoters - detractors) / len(scores) * 100)`. `by_touchpoint`: group the responses in a dict and return `{tp: (count, average csat, average ces)}`.
@@solution
responses = [
    {"touchpoint": "onboarding", "nps": 4, "csat": 2, "ces": 2},
    {"touchpoint": "onboarding", "nps": 6, "csat": 3, "ces": 3},
    {"touchpoint": "onboarding", "nps": 7, "csat": 2, "ces": 2},
    {"touchpoint": "portal", "nps": 9, "csat": 5, "ces": 6},
    {"touchpoint": "portal", "nps": 10, "csat": 4, "ces": 6},
    {"touchpoint": "portal", "nps": 8, "csat": 4, "ces": 5},
    {"touchpoint": "outage comms", "nps": 3, "csat": 1, "ces": 3},
    {"touchpoint": "outage comms", "nps": 9, "csat": 4, "ces": 5},
    {"touchpoint": "phone", "nps": 10, "csat": 5, "ces": 7},
    {"touchpoint": "phone", "nps": 9, "csat": 5, "ces": 6},
    {"touchpoint": "phone", "nps": 6, "csat": 3, "ces": 4},
]

def nps(scores):
    promoters = sum(1 for s in scores if s >= 9)
    detractors = sum(1 for s in scores if s <= 6)
    return round((promoters - detractors) / len(scores) * 100)

def by_touchpoint(responses):
    groups = {}
    for r in responses:
        groups.setdefault(r["touchpoint"], []).append(r)
    return {tp: (len(rs), sum(r["csat"] for r in rs) / len(rs), sum(r["ces"] for r in rs) / len(rs)) for tp, rs in groups.items()}

print(f"NPS {nps([r['nps'] for r in responses]):+d}")
stats = by_touchpoint(responses)
for tp, (n, csat, ces) in stats.items():
    print(f"  {tp:<13} n={n}  CSAT {csat:.1f}/5  CES {ces:.1f}/7")
eligible = {tp: s for tp, s in stats.items() if s[0] >= 3}
worst = min(eligible, key=lambda tp: eligible[tp][1])
print("moment of truth to fix:", worst)

@@exercise Find the watermelons
For each service, compare SLA compliance (operational data) with the experience score (experience data). Classify it as "healthy" (SLA met and experience at least 3.5), "watermelon" (SLA met but experience below 3.5), "tolerated" (SLA missed but experience fine) or "failing". Also compute lost productive time in hours from the incidents (users affected × hours down) and sort the report by it.
@@starter
services = {
    "Email": {"sla_target": 99.9, "sla_actual": 99.95, "experience": 4.4, "incidents": [(1200, 0.5)]},
    "VPN": {"sla_target": 99.5, "sla_actual": 99.7, "experience": 2.6, "incidents": [(300, 2.0), (450, 1.5), (200, 3.0)]},
    "HR portal": {"sla_target": 99.0, "sla_actual": 98.2, "experience": 3.9, "incidents": [(80, 4.0)]},
    "Laptop fleet": {"sla_target": 98.0, "sla_actual": 96.5, "experience": 2.9, "incidents": [(60, 8.0), (25, 6.0)]},
}

def classify(s):
    raise NotImplementedError

def lost_hours(s):
    raise NotImplementedError

for name, s in sorted(services.items(), key=lambda kv: -lost_hours(kv[1])):
    sla = "met" if s["sla_actual"] >= s["sla_target"] else "missed"
    print(f"{name:<13} SLA {sla:<6} experience {s['experience']:.1f}  lost {lost_hours(s):>6.0f} h  {classify(s)}")
@@expected
VPN           SLA met    experience 2.6  lost   1875 h  watermelon
Laptop fleet  SLA missed experience 2.9  lost    630 h  failing
Email         SLA met    experience 4.4  lost    600 h  healthy
HR portal     SLA missed experience 3.9  lost    320 h  tolerated
@@requires
def classify
def lost_hours
"watermelon"
@@hint
`classify`: `met = sla_actual >= sla_target`, `good = experience >= 3.5`, then return one of the four labels. `lost_hours` is `sum(users * hours for users, hours in s["incidents"])`.
@@solution
services = {
    "Email": {"sla_target": 99.9, "sla_actual": 99.95, "experience": 4.4, "incidents": [(1200, 0.5)]},
    "VPN": {"sla_target": 99.5, "sla_actual": 99.7, "experience": 2.6, "incidents": [(300, 2.0), (450, 1.5), (200, 3.0)]},
    "HR portal": {"sla_target": 99.0, "sla_actual": 98.2, "experience": 3.9, "incidents": [(80, 4.0)]},
    "Laptop fleet": {"sla_target": 98.0, "sla_actual": 96.5, "experience": 2.9, "incidents": [(60, 8.0), (25, 6.0)]},
}

def classify(s):
    met = s["sla_actual"] >= s["sla_target"]
    good = s["experience"] >= 3.5
    if met and good:
        return "healthy"
    if met:
        return "watermelon"
    if good:
        return "tolerated"
    return "failing"

def lost_hours(s):
    return sum(users * hours for users, hours in s["incidents"])

for name, s in sorted(services.items(), key=lambda kv: -lost_hours(kv[1])):
    sla = "met" if s["sla_actual"] >= s["sla_target"] else "missed"
    print(f"{name:<13} SLA {sla:<6} experience {s['experience']:.1f}  lost {lost_hours(s):>6.0f} h  {classify(s)}")
