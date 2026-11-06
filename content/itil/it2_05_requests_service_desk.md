@@lesson it2-requests-desk Service Requests and the Service Desk: Catalogues, Fulfilment, Shift-Left and Desk Metrics
@@concepts maps, conditions, numbers
@@lang python
@@text
Most of what users ask IT for isn't broken: a new laptop, access to a shared drive, a software install, a question about how to do something. ITIL separates these **service requests** from incidents because they're handled differently: they're expected, standardised and often automatable. The **service desk** is where both arrive, and its design shapes how users feel about the whole IT organisation.

## Service request management

- A **service request** is a request from a user or their representative that starts a **predefined, agreed** service action, e.g. information, access, equipment, a standard change or feedback.
- Requests are **normal** parts of service delivery, not failures. They should be fulfilled by **standard procedures** (workflows) with known steps, times and approvals.
- **Approvals**: many requests are **pre-approved** (a standard mouse). Others need approval from a budget holder or an access owner. Keep approvals as few as you can: each adds waiting time.
- **Standard changes** (low risk, pre-authorised) are often requested through the service request process.
- Requests should be offered through a **service catalogue** (the customer-facing view of service offerings) with clear descriptions, prices, expected fulfilment times and who can order them.
- **Automation** pays off most here: requests are repetitive and predictable, e.g. automatic access provisioning after the owner approves.

## The service desk practice

- The **service desk** is the single point of contact between the provider and its users. It captures demand for incident resolution and service requests.
- ITIL 4 stresses that the desk should understand the **wider organisation and the business context**, show **empathy** and **emotional intelligence**, and be able to **prioritise** and **communicate** clearly. It's not just a call-logging function.
- **Channels**: phone, email, chat, a **self-service portal**, walk-up (a tech bar), chatbots and social media. Offer many, but steer users to the cheapest effective ones.
- Desk structures:
  - **local**: on site.
  - **centralised**: one location.
  - **virtual**: spread out, but acting as one desk.
  - **follow the sun**: handing off across time zones, which is common for global Japanese companies with teams in Tokyo, Manila and Europe.

## Shift-left

- **Shift-left** moves resolution closer to the user and earlier, where it's cheaper:
  - level 0 is self-service: knowledge articles, automated password reset.
  - level 1 is the desk.
  - level 2 is specialist teams.
  - level 3 is vendors and developers.
- Typical cost per contact rises sharply as tickets move right. A self-service resolution can cost a tenth of a level 2 fix.
- Shift-left needs **knowledge management** (articles that are good enough to use), **tools and permissions** for level 1, and training.

## Service desk metrics

- **First contact resolution (FCR)**: the share of contacts resolved during the first contact, without escalation or a callback. It's the metric most strongly linked to satisfaction.
- **Average speed of answer (ASA)** and **abandonment rate** (callers who hang up before answering).
- **Average handle time (AHT)**: don't push it down alone, or agents will rush and FCR drops.
- **Customer satisfaction (CSAT)** after each ticket and periodic **experience surveys**.
- **Request fulfilment time** against the catalogue promise, and the **cost per ticket**.
- **Backlog age**: tickets open longer than N days.
- Combine metrics, or you'll get the watermelon effect: green numbers and unhappy users.

@@quiz
A user asks for access to the finance shared drive. What is this in ITIL terms?
* A service request
- An incident
- A problem
- An emergency change
? Predefined and expected, not a failure.

@@quiz
Which statement about service requests is correct?
* They should be fulfilled with standard, predefined procedures
- Each one needs a CAB review
- They are a type of incident
- They can't be automated
? Repetitive and predictable.

@@quiz
What does shift-left mean?
* Resolving issues earlier and closer to the user, e.g. through self-service or level 1
- Moving the service desk to another office
- Escalating faster to level 3
- Working only during the morning shift
? It reduces cost and waiting.

@@quiz
Which metric is most closely linked to user satisfaction with the service desk?
* First contact resolution
- Number of tickets logged
- Average handle time alone
- Number of agents
? Users want it solved the first time.

@@quiz
Which desk structure hands work across time zones so that support is available around the clock?
- Local
- Centralised
* Follow the sun
- Walk-up
? Common for global companies.

@@quiz
Where should users find the services they can request, with times and prices?
* The service catalogue
- The CMDB
- The problem record
- The change schedule
? The customer-facing view of offerings.

@@flashcards
service request :: user request for a predefined, agreed action
standard procedure :: workflow for fulfilling a request
pre-approved request :: no approval step needed
service catalogue :: offerings users can order, with times and prices
service desk :: single point of contact with users
desk skills :: empathy, business context, prioritising, communication
channels :: phone, email, chat, portal, walk-up, bots
follow the sun :: support handed across time zones
shift-left :: resolve earlier, closer to the user
level 0 :: self-service
FCR :: resolved on the first contact
ASA :: average speed of answer
abandonment rate :: callers who hang up before an answer
AHT :: average handle time
watermelon effect :: green metrics, unhappy users

@@drill
200 calls were offered and 14 callers hung up before an answer. What is the abandonment rate in percent?
= 7
= 7%
? 14 ÷ 200.

@@drill
Which ITIL record type is a request for a new monitor?
= service request
= a service request
= request
? Not an incident.

@@exercise Route requests through the catalogue
Each catalogue item has an approval rule and a fulfilment target in business hours. For each incoming request, decide the approvals needed (none for pre-approved items, the user's manager for items over ¥50,000, the data owner for access requests, and both where both apply), whether it can be fulfilled automatically, and the due time in business hours once approved.
@@starter
CATALOGUE = {
    "mouse": {"price": 3000, "access": False, "auto": False, "hours": 8, "preapproved": True},
    "laptop": {"price": 180000, "access": False, "auto": False, "hours": 40, "preapproved": False},
    "shared drive access": {"price": 0, "access": True, "auto": True, "hours": 4, "preapproved": False},
    "software: diagram tool": {"price": 12000, "access": False, "auto": True, "hours": 2, "preapproved": True},
    "admin rights": {"price": 0, "access": True, "auto": False, "hours": 16, "preapproved": False},
    "4K monitor": {"price": 65000, "access": False, "auto": False, "hours": 24, "preapproved": False},
}
requests = [("R-101", "mouse"), ("R-102", "laptop"), ("R-103", "shared drive access"), ("R-104", "software: diagram tool"),
            ("R-105", "admin rights"), ("R-106", "4K monitor"), ("R-107", "standing desk")]

def approvals(item):
    raise NotImplementedError

def route(req_id, name):
    raise NotImplementedError

for req_id, name in requests:
    print(route(req_id, name))
@@expected
R-101 mouse: pre-approved, manual, due within 8 business hours
R-102 laptop: manager, manual, due within 40 business hours
R-103 shared drive access: data owner, automated, due within 4 business hours
R-104 software: diagram tool: pre-approved, automated, due within 2 business hours
R-105 admin rights: data owner, manual, due within 16 business hours
R-106 4K monitor: manager, manual, due within 24 business hours
R-107 standing desk: not in the catalogue: log as a request for a new offering
@@requires
def approvals
def route
"data owner"
@@hint
`approvals` returns a list: empty if pre-approved; otherwise "manager" when the price is over 50,000 and "data owner" for access items. In `route`, unknown items get "not in the catalogue: log as a request for a new offering". Otherwise format the approvals (or "pre-approved"), "automated" or "manual", and "due within N business hours".
@@solution
CATALOGUE = {
    "mouse": {"price": 3000, "access": False, "auto": False, "hours": 8, "preapproved": True},
    "laptop": {"price": 180000, "access": False, "auto": False, "hours": 40, "preapproved": False},
    "shared drive access": {"price": 0, "access": True, "auto": True, "hours": 4, "preapproved": False},
    "software: diagram tool": {"price": 12000, "access": False, "auto": True, "hours": 2, "preapproved": True},
    "admin rights": {"price": 0, "access": True, "auto": False, "hours": 16, "preapproved": False},
    "4K monitor": {"price": 65000, "access": False, "auto": False, "hours": 24, "preapproved": False},
}
requests = [("R-101", "mouse"), ("R-102", "laptop"), ("R-103", "shared drive access"), ("R-104", "software: diagram tool"),
            ("R-105", "admin rights"), ("R-106", "4K monitor"), ("R-107", "standing desk")]

def approvals(item):
    if item["preapproved"]:
        return []
    needed = []
    if item["price"] > 50000:
        needed.append("manager")
    if item["access"]:
        needed.append("data owner")
    return needed

def route(req_id, name):
    item = CATALOGUE.get(name)
    if item is None:
        return f"{req_id} {name}: not in the catalogue: log as a request for a new offering"
    needed = approvals(item)
    who = " + ".join(needed) if needed else "pre-approved"
    how = "automated" if item["auto"] else "manual"
    return f"{req_id} {name}: {who}, {how}, due within {item['hours']} business hours"

for req_id, name in requests:
    print(route(req_id, name))

@@exercise Service desk dashboard
From a day's contact log, compute the desk metrics: contacts by channel, FCR (resolved at first contact ÷ resolved contacts, excluding service requests that need fulfilment), the abandonment rate and ASA for phone calls, average handle time, and CSAT from the surveys returned. Then flag shift-left candidates: categories with at least 3 contacts where level 1 resolved none.
@@starter
log = [
    {"ch": "phone", "cat": "password", "wait": 30, "handle": 6, "first": True, "level": 1, "csat": 5},
    {"ch": "phone", "cat": "vpn", "wait": 95, "handle": 14, "first": False, "level": 2, "csat": 3},
    {"ch": "phone", "cat": None, "wait": 140, "handle": 0, "first": False, "level": 0, "csat": None},
    {"ch": "chat", "cat": "password", "wait": 10, "handle": 4, "first": True, "level": 1, "csat": None},
    {"ch": "portal", "cat": "printer", "wait": 0, "handle": 9, "first": False, "level": 2, "csat": 2},
    {"ch": "phone", "cat": "printer", "wait": 45, "handle": 11, "first": False, "level": 2, "csat": None},
    {"ch": "chat", "cat": "printer", "wait": 20, "handle": 7, "first": False, "level": 2, "csat": 3},
    {"ch": "phone", "cat": "vpn", "wait": 60, "handle": 12, "first": True, "level": 1, "csat": 4},
    {"ch": "email", "cat": "access", "wait": 0, "handle": 5, "first": False, "level": 1, "csat": None},
    {"ch": "phone", "cat": None, "wait": 200, "handle": 0, "first": False, "level": 0, "csat": None},
]

def dashboard(log):
    raise NotImplementedError

for name, value in dashboard(log).items():
    print(f"{name:<22} {value}")
@@expected
contacts by channel    phone 6, chat 2, portal 1, email 1
FCR                    43%
phone abandon rate     33%
phone ASA              58 s
AHT                    8.5 min
CSAT                   3.4 / 5 (5 surveys)
shift-left candidates  printer
@@requires
def dashboard
abandon
@@hint
Abandoned calls have `cat` of `None`. Answered contacts are the rest. FCR: among answered contacts except the "access" category (a request), the share with `first` true. Phone: abandonment = abandoned ÷ phone contacts; ASA = average `wait` of answered phone calls. AHT is the average `handle` of answered contacts. CSAT is the average of the non-None scores. Shift-left: group answered contacts by category, and flag categories with 3 or more contacts and no `level == 1`.
@@solution
log = [
    {"ch": "phone", "cat": "password", "wait": 30, "handle": 6, "first": True, "level": 1, "csat": 5},
    {"ch": "phone", "cat": "vpn", "wait": 95, "handle": 14, "first": False, "level": 2, "csat": 3},
    {"ch": "phone", "cat": None, "wait": 140, "handle": 0, "first": False, "level": 0, "csat": None},
    {"ch": "chat", "cat": "password", "wait": 10, "handle": 4, "first": True, "level": 1, "csat": None},
    {"ch": "portal", "cat": "printer", "wait": 0, "handle": 9, "first": False, "level": 2, "csat": 2},
    {"ch": "phone", "cat": "printer", "wait": 45, "handle": 11, "first": False, "level": 2, "csat": None},
    {"ch": "chat", "cat": "printer", "wait": 20, "handle": 7, "first": False, "level": 2, "csat": 3},
    {"ch": "phone", "cat": "vpn", "wait": 60, "handle": 12, "first": True, "level": 1, "csat": 4},
    {"ch": "email", "cat": "access", "wait": 0, "handle": 5, "first": False, "level": 1, "csat": None},
    {"ch": "phone", "cat": None, "wait": 200, "handle": 0, "first": False, "level": 0, "csat": None},
]

def dashboard(log):
    answered = [c for c in log if c["cat"] is not None]
    abandoned = [c for c in log if c["cat"] is None]
    phone = [c for c in log if c["ch"] == "phone"]
    phone_answered = [c for c in phone if c["cat"] is not None]
    fcr_pool = [c for c in answered if c["cat"] != "access"]
    scores = [c["csat"] for c in log if c["csat"] is not None]
    channels = {}
    for c in log:
        channels[c["ch"]] = channels.get(c["ch"], 0) + 1
    by_cat = {}
    for c in answered:
        by_cat.setdefault(c["cat"], []).append(c)
    shift = [cat for cat, cs in by_cat.items() if len(cs) >= 3 and not any(c["level"] == 1 for c in cs)]
    return {
        "contacts by channel": ", ".join(f"{k} {v}" for k, v in channels.items()),
        "FCR": f"{sum(c['first'] for c in fcr_pool) / len(fcr_pool):.0%}",
        "phone abandon rate": f"{len(abandoned) / len(phone):.0%}",
        "phone ASA": f"{sum(c['wait'] for c in phone_answered) / len(phone_answered):.0f} s",
        "AHT": f"{sum(c['handle'] for c in answered) / len(answered):.1f} min",
        "CSAT": f"{sum(scores) / len(scores):.1f} / 5 ({len(scores)} surveys)",
        "shift-left candidates": ", ".join(shift) or "none",
    }

for name, value in dashboard(log).items():
    print(f"{name:<22} {value}")
