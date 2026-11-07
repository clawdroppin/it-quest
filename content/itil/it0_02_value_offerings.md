@@lesson it0-value-offerings Value in Practice: Utility, Warranty, Offerings and Relationships
@@text
ITIL 4 Foundation questions about key concepts look simple but use precise definitions: a service is a means of enabling **value co-creation** by facilitating **outcomes** customers want, without the customer having to manage specific **costs** and **risks**. This lesson applies those definitions: how to tell utility from warranty, what a service offering consists of, the roles around a service, and how service relationships work.

## Value, outcomes, costs and risks

- **Value**: the perceived benefits, usefulness and importance of something. Value is **co-created** by provider and consumer together.
- **Output**: a tangible or intangible deliverable of an activity (a report, a configured laptop). **Outcome**: a result for a stakeholder enabled by outputs (the sales team closes deals faster).
- **Costs**: removed from the consumer (they don't buy servers) or imposed (licence fees, staff time to learn the tool).
- **Risks**: removed (hardware failure handled by the provider) or imposed (dependency on the provider, data leaving the company).

## Utility and warranty

- **Utility**: the functionality offered to meet a particular need: **what the service does**, "fit for purpose". Example: the email service can send encrypted messages.
- **Warranty**: assurance that a product or service will meet agreed requirements: **how well it performs**, "fit for use". Typically **availability, capacity, security, continuity** and performance. Example: 99.9% availability, mailbox size 50 GB, recovery within 4 hours.
- A service needs **both** to create value.

## Service offerings

A **service offering** is a formal description of one or more services designed for a target consumer group, and may include:
1. **Goods**: ownership transferred to the consumer (a laptop, a phone); the consumer takes responsibility.
2. **Access to resources**: ownership stays with the provider (network access, storage space, a software licence).
3. **Service actions**: performed by the provider (user support, replacing equipment, installing software).
Different offerings of the same service: Basic (8×5 support) and Premium (24×7 support, faster response).

## Roles and relationships

- **Customer**: defines the requirements and takes responsibility for the outcomes. **User**: uses the services. **Sponsor**: authorises the budget. One person can hold all three (a freelancer buying a SaaS tool).
- **Service relationship**: cooperation between a provider and a consumer, including **service provision**, **service consumption** and **service relationship management**.
- **Service relationship model**: an organisation is often both a consumer (of cloud services) and a provider (to its own customers): value flows along a chain.

@@quiz
"The service must be available 99.9% of the time" describes what?
- Utility
* Warranty
- An output
- A service action
? How well the service performs.

@@quiz
What is the difference between an output and an outcome?
* An output is a deliverable; an outcome is the result it enables for a stakeholder
- They are the same
- Outcomes are tangible, outputs aren't
- Outputs are only for customers
? Outputs enable outcomes.

@@quiz
Which part of a service offering transfers ownership to the consumer?
* Goods
- Access to resources
- Service actions
- Warranty
? A laptop handed to the user.

@@quiz
Who authorises the budget for service consumption?
- User
- Customer
* Sponsor
- Supplier
? Customers define requirements.

@@quiz
The provider takes over server maintenance, but the consumer becomes dependent on the provider's security. What is that dependency?
- A removed cost
* An imposed risk
- A utility
- An output
? Services remove and impose both costs and risks.

@@flashcards
service :: enables value co-creation via outcomes
value :: perceived benefits, usefulness, importance
co-creation :: provider and consumer together
output :: deliverable of an activity
outcome :: result for a stakeholder
utility :: fit for purpose, what it does
warranty :: fit for use, how well
warranty aspects :: availability, capacity, security, continuity
service offering :: services for a consumer group
goods :: ownership transferred
access to resources :: ownership stays with provider
service actions :: performed by provider
customer :: defines requirements, owns outcomes
user :: uses the service
sponsor :: authorises budget
service relationship :: provision + consumption + relationship management

@@drill
Is "the service can export reports to PDF" utility or warranty?
= utility
? It's functionality.

@@drill
Which role defines requirements and takes responsibility for outcomes?
= customer
= the customer
? The sponsor pays; the user uses.

@@exercise Utility or warranty?
Classify service requirements as utility (functionality) or warranty (availability, capacity, security, continuity, performance) using keyword rules, and check that each service has at least one of each (a service needs both to create value).
@@lang python
@@starter
WARRANTY = {"availability": ["available", "uptime", "%"], "capacity": ["users", "gb", "concurrent", "storage"],
            "security": ["encrypt", "mfa", "access", "audit"], "continuity": ["recover", "rto", "rpo", "backup"],
            "performance": ["seconds", "response", "latency"]}
services = {
    "Email": ["send and receive mail on phones", "99.9% available during business hours", "50 GB mailbox per user",
              "recover service within 4 hours after a disaster"],
    "Expense app": ["submit receipts with a photo", "approve claims from a manager view", "export to accounting"],
    "VPN": ["99.95% uptime", "MFA required for every login", "support 2000 concurrent users"],
}

def classify(requirement):
    raise NotImplementedError
@@expected
Email
  utility                 send and receive mail on phones
  warranty (availability) 99.9% available during business hours
  warranty (capacity)     50 GB mailbox per user
  warranty (continuity)   recover service within 4 hours after a disaster
Expense app
  utility                 submit receipts with a photo
  utility                 approve claims from a manager view
  utility                 export to accounting
  MISSING: warranty (how well, how safely?)
VPN
  warranty (availability) 99.95% uptime
  warranty (security)     MFA required for every login
  warranty (capacity)     support 2000 concurrent users
  MISSING: utility (what does it do?)
@@requires
def classify
WARRANTY
utility
@@hint
Lower-case the requirement and return the first warranty aspect whose keywords appear in it, or "utility" if none match. Then group per service and report what's missing.
@@solution
WARRANTY = {"availability": ["available", "uptime", "%"], "capacity": ["users", "gb", "concurrent", "storage"],
            "security": ["encrypt", "mfa", "access", "audit"], "continuity": ["recover", "rto", "rpo", "backup"],
            "performance": ["seconds", "response", "latency"]}
services = {
    "Email": ["send and receive mail on phones", "99.9% available during business hours", "50 GB mailbox per user",
              "recover service within 4 hours after a disaster"],
    "Expense app": ["submit receipts with a photo", "approve claims from a manager view", "export to accounting"],
    "VPN": ["99.95% uptime", "MFA required for every login", "support 2000 concurrent users"],
}

def classify(requirement):
    text = requirement.lower()
    for aspect, words in WARRANTY.items():
        if any(w in text for w in words):
            return f"warranty ({aspect})"
    return "utility"

for name, reqs in services.items():
    print(name)
    kinds = []
    for r in reqs:
        kind = classify(r)
        kinds.append(kind)
        print(f"  {kind:<23} {r}")
    has_utility = "utility" in kinds
    has_warranty = any(k.startswith("warranty") for k in kinds)
    if not (has_utility and has_warranty):
        print(f"  MISSING: {'utility (what does it do?)' if not has_utility else 'warranty (how well, how safely?)'}")

@@exercise Compose service offerings
Describe offerings made of goods, access to resources and service actions, compute each offering's monthly cost per user (goods amortised over their life in months), and compare the costs and risks the consumer still carries (ownership of goods means the consumer carries loss and damage risk).
@@lang python
@@starter
offerings = {
    "Basic workplace": {"goods": [("laptop", 120_000, 48)], "access": [("M365 licence", 1_800), ("VPN", 300)],
                        "actions": [("8x5 service desk", 900)]},
    "Premium workplace": {"goods": [("laptop", 180_000, 36), ("phone", 90_000, 24)], "access": [("M365 licence", 3_200), ("VPN", 300)],
                          "actions": [("24x7 service desk", 2_400), ("next-day device swap", 1_000)]},
    "Contractor access": {"goods": [], "access": [("virtual desktop", 4_500), ("VPN", 300)], "actions": [("8x5 service desk", 900)]},
}

def monthly_cost(offering):
    raise NotImplementedError
@@expected
Basic workplace: ¥5,500/user/month (goods ¥2,500, access ¥2,100, actions ¥900); consumer owns laptop (loss/damage risk imposed)
Premium workplace: ¥15,650/user/month (goods ¥8,750, access ¥3,500, actions ¥3,400); consumer owns laptop, phone (loss/damage risk imposed)
Contractor access: ¥5,700/user/month (goods ¥0, access ¥4,800, actions ¥900); no goods: device risk stays with the provider
@@requires
def monthly_cost
goods
actions
sum(
@@hint
Goods are `(name, price, life_months)`: monthly share = price / life. Access and actions are `(name, monthly_cost)`. Return the total and the breakdown by part. A consumer owning goods carries the risk of loss and damage; offerings without goods leave that risk with the provider.
@@solution
offerings = {
    "Basic workplace": {"goods": [("laptop", 120_000, 48)], "access": [("M365 licence", 1_800), ("VPN", 300)],
                        "actions": [("8x5 service desk", 900)]},
    "Premium workplace": {"goods": [("laptop", 180_000, 36), ("phone", 90_000, 24)], "access": [("M365 licence", 3_200), ("VPN", 300)],
                          "actions": [("24x7 service desk", 2_400), ("next-day device swap", 1_000)]},
    "Contractor access": {"goods": [], "access": [("virtual desktop", 4_500), ("VPN", 300)], "actions": [("8x5 service desk", 900)]},
}

def monthly_cost(offering):
    parts = {
        "goods": sum(price / life for _, price, life in offering["goods"]),
        "access": sum(cost for _, cost in offering["access"]),
        "actions": sum(cost for _, cost in offering["actions"]),
    }
    return sum(parts.values()), parts

for name, offering in offerings.items():
    total, parts = monthly_cost(offering)
    breakdown = ", ".join(f"{k} ¥{v:,.0f}" for k, v in parts.items())
    risk = ("consumer owns " + ", ".join(g[0] for g in offering["goods"]) + " (loss/damage risk imposed)") if offering["goods"] else "no goods: device risk stays with the provider"
    print(f"{name}: ¥{total:,.0f}/user/month ({breakdown}); {risk}")
