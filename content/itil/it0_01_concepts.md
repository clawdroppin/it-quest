@@lesson it0-concepts ITIL 4: Service Management Concepts
@@text
**ITIL** is the most widely used framework for **IT service management (ITSM)**: how IT organizations deliver services that actually help the business. If you work on a service desk, in operations (**運用保守**), or with ServiceNow or Jira Service Management, you're already doing ITIL, whether anyone calls it that or not. Many job adverts, in Japan and elsewhere, list **ITIL 4 Foundation** as a plus.

**The exam**: ITIL 4 Foundation, run by **PeopleCert** (owner of AXELOS). It has 40 multiple-choice questions in 60 minutes, and you pass with **26 correct (65%)**. You need an accredited training course, which usually includes the voucher. It's available in Japanese too. PeopleCert updates ITIL from time to time, so check which version the current exam is based on. The core ideas below have stayed stable.

## Key concepts (the exam loves these definitions)

| Term | ITIL meaning |
|---|---|
| **Service** | a means of enabling **value co-creation** by facilitating **outcomes** customers want to achieve, without them having to manage specific **costs** and **risks** |
| **Value** | the perceived benefits, usefulness and importance of something |
| **Output** | a tangible or intangible deliverable of an activity (a new laptop) |
| **Outcome** | a result for a stakeholder, enabled by outputs (the new employee can work on day one) |
| **Utility** | **fit for purpose**: what the service does (functionality) |
| **Warranty** | **fit for use**: how well it does it (availability, capacity, security, continuity) |
| **Service offering** | goods, access to resources, and service actions, described for a target group |
| **Service relationship** | cooperation between provider and consumer: **service provision**, **service consumption**, **service relationship management** |

Roles on the consumer side: **customer** (defines requirements, takes responsibility for outcomes), **user** (uses the service), **sponsor** (authorizes the budget). One person can be all three.

**Organization**: a person or group with its own functions, responsibilities and authorities to achieve its objectives.

> Value needs both utility and warranty. Email that can do everything but is down half the day (no warranty) has no value; a perfectly available system that doesn't do what people need (no utility) has none either.

## Costs and risks

A good service **removes** some costs and risks from the consumer (you don't run your own mail server) and **imposes** some (subscription fees, dependence on the provider). Value is the balance.

@@quiz
In ITIL, which describes "fit for purpose"?
* Utility
- Warranty
- Outcome
- Output
? Utility is what the service does; warranty is how well (available, secure, enough capacity).

@@quiz
A new employee receives a configured laptop (A) and can start work on the first day (B). Which is which?
* A is an output, B is an outcome
- A is an outcome, B is an output
- Both are outputs
- Both are outcomes
? Outputs are deliverables; outcomes are the results stakeholders care about.

@@quiz
Who authorizes the budget for service consumption?
* The sponsor
- The user
- The customer
- The service desk
? The customer defines the requirements; the user uses the service.

@@quiz
Which is part of warranty?
* Availability of the service
- The features it offers
- The colour of the interface
- The number of users
? Availability, capacity, security and continuity are warranty.

@@quiz
Value in ITIL 4 is...
* Co-created by the provider and consumer, and perceived by stakeholders
- Created by the provider alone
- Measured only in money
- The same as output
? Hence "value co-creation".

@@flashcards
ITSM :: IT service management
ITIL 4 Foundation :: 40 questions, 60 minutes, pass 26 (65%)
service :: Enables value co-creation via outcomes, without the customer owning costs and risks
value :: Perceived benefits, usefulness and importance
output :: Deliverable of an activity
outcome :: Result for a stakeholder
utility :: Fit for purpose (what it does)
warranty :: Fit for use (how well: availability, capacity, security, continuity)
customer :: Defines requirements, responsible for outcomes
user :: Uses the service
sponsor :: Authorizes the budget
service offering :: Goods, access to resources, service actions
service relationship management :: Joint activities to ensure ongoing value co-creation

@@drill
Which ITIL term means "fit for use"?
= warranty
? Utility + warranty = value.

@@exercise Outputs, outcomes, costs and risks
ITIL defines value as the perceived benefits, usefulness and importance of something, and a service as enabling value co-creation by facilitating outcomes customers want to achieve without them having to manage specific costs and risks. Classify each statement about a cloud email service as an output (what the provider delivers), an outcome (a result the consumer wants), a cost removed, a cost imposed, a risk removed or a risk imposed, then summarise.
@@lang python
@@starter
statements = [
    ("Mailboxes are provisioned within an hour", "provider delivers"),
    ("Sales staff can reply to customers from their phones", "consumer result"),
    ("No need to buy and maintain mail servers", "consumer no longer pays"),
    ("Monthly per-user subscription fee", "consumer now pays"),
    ("Provider handles spam filtering and patching", "consumer no longer worries"),
    ("Customer data now sits with a third party", "consumer now worries"),
    ("Daily backups are taken", "provider delivers"),
    ("Contracts are signed faster", "consumer result"),
]

def classify(hint):
    raise NotImplementedError

summary = {}
for text, hint in statements:
    kind = classify(hint)
    summary[kind] = summary.get(kind, 0) + 1
    print(f"{kind:<14} {text}")
print(", ".join(f"{k}: {v}" for k, v in sorted(summary.items())))
@@expected
output         Mailboxes are provisioned within an hour
outcome        Sales staff can reply to customers from their phones
cost removed   No need to buy and maintain mail servers
cost imposed   Monthly per-user subscription fee
risk removed   Provider handles spam filtering and patching
risk imposed   Customer data now sits with a third party
output         Daily backups are taken
outcome        Contracts are signed faster
cost imposed: 1, cost removed: 1, outcome: 2, output: 2, risk imposed: 1, risk removed: 1
@@requires
def classify
"outcome"
@@hint
Map the hint text to the term with a dictionary: "provider delivers" is "output", "consumer result" is "outcome", "consumer no longer pays" is "cost removed", "consumer now pays" is "cost imposed", "consumer no longer worries" is "risk removed" and "consumer now worries" is "risk imposed".
@@solution
statements = [
    ("Mailboxes are provisioned within an hour", "provider delivers"),
    ("Sales staff can reply to customers from their phones", "consumer result"),
    ("No need to buy and maintain mail servers", "consumer no longer pays"),
    ("Monthly per-user subscription fee", "consumer now pays"),
    ("Provider handles spam filtering and patching", "consumer no longer worries"),
    ("Customer data now sits with a third party", "consumer now worries"),
    ("Daily backups are taken", "provider delivers"),
    ("Contracts are signed faster", "consumer result"),
]

TERMS = {
    "provider delivers": "output",
    "consumer result": "outcome",
    "consumer no longer pays": "cost removed",
    "consumer now pays": "cost imposed",
    "consumer no longer worries": "risk removed",
    "consumer now worries": "risk imposed",
}

def classify(hint):
    return TERMS[hint]

summary = {}
for text, hint in statements:
    kind = classify(hint)
    summary[kind] = summary.get(kind, 0) + 1
    print(f"{kind:<14} {text}")
print(", ".join(f"{k}: {v}" for k, v in sorted(summary.items())))
