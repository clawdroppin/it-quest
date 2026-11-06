@@lesson it2-assets-finance IT Assets, Money and Suppliers: Asset Management, Licences, Showback and Supplier Management
@@concepts maps, numbers, lists
@@lang python
@@text
Services cost money, run on assets someone has to track, and depend on suppliers who must deliver. Three ITIL practices cover this: **IT asset management**, **service financial management** and **supplier management**. They're less glamorous than incident handling, but they are where audits, budget fights and outsourcing failures come from.

## IT asset management (ITAM)

- An **IT asset** is any financially valuable component that can contribute to delivering an IT product or service: hardware, software licences, cloud subscriptions, even some data.
- The **asset life cycle**: plan → acquire (procure, receive, tag) → deploy → operate and maintain → retire (sanitise, dispose, or resell).
- **Asset register**: what you own, where it is, who has it, its cost and its contract. It's related to the CMDB but not the same. The CMDB tracks configuration items for service management (relationships, impact), while the asset register tracks financial and contractual facts. Many items are both.
- **Software asset management (SAM)**: compare **entitlements** (licences you bought) with **deployments** (what's installed or assigned). Licence models include per user, per device, per core, concurrent users, subscription and site licences.
  - **Over-deployment** means a compliance risk: an audit by the vendor can bring a large true-up bill.
  - **Under-use** means wasted money: harvest unused licences and reassign them.
  - Watch for **downgrade rights** (a higher edition licence may cover a lower edition install) and cloud subscriptions that keep billing after staff leave.

## Service financial management

- Its purpose is to support decisions about services with financial information: **budgeting**, **costing** and **accounting**, and **charging** where the organisation does it.
- **CapEx** (buying servers, owning assets that depreciate) vs **OpEx** (cloud and subscriptions, paid as you use). Cloud moves spending from CapEx to OpEx, and can surprise budgets.
- **Direct costs** belong to one service (its dedicated licences). **Indirect** or **shared** costs (data centre power, the service desk) must be **allocated** with a fair driver: users, tickets, CPU hours or storage.
- **Showback** reports each department's share of costs without billing them, which builds awareness. **Chargeback** actually bills internal customers. Chargeback changes behaviour more, but costs effort and can cause arguments.
- **Total cost of ownership (TCO)** includes purchase, implementation, operation, support, training and disposal over the asset's life, not just the price tag.

## Supplier management

- Its purpose is to ensure the organisation's suppliers and their performance are managed appropriately to support seamless delivery of quality products and services.
- Activities:
  - creating a **supplier strategy**: what to source, from whom, and the sourcing model (insourcing, outsourcing, single, multi-sourcing).
  - selecting suppliers, negotiating **contracts** with measurable service levels.
  - managing the relationship and performance, with regular reviews and scorecards.
  - managing contract renewal and termination, including **exit plans**.
- **Supplier categories** by importance and risk: **strategic** (critical, long-term partners), **tactical**, **operational** and **commodity**. Spend relationship effort where the risk is.
- Watch **underpinning contracts**: your SLA with customers is only as strong as the supplier contracts underneath it. A 99.9% SLA on top of a supplier promising 99.5% is a promise you can't keep.
- Japanese companies often work through multi-layer subcontracting (元請け, 下請け). Make sure contracts and accountability are clear at every layer.

@@quiz
What is the main difference between the asset register and the CMDB?
* The register tracks financial and contractual facts, the CMDB tracks configuration items and their relationships for service management
- They are always the same database
- The CMDB only contains software
- The register only contains cloud services
? Many items appear in both.

@@quiz
A vendor audit finds 120 installs and 100 licences. What is this?
* Over-deployment: a compliance risk that can lead to a true-up bill
- Under-use
- A service request
- A downgrade right
? Reconcile entitlements and deployments regularly.

@@quiz
What is showback?
* Reporting each department's share of IT costs without billing them
- Billing departments for IT
- Returning unused hardware
- Showing a backup to the auditor
? Chargeback actually bills.

@@quiz
Which cost is indirect and needs allocation?
* Data centre power shared by all services
- A licence bought only for the HR system
- A contractor working only on the CRM
- A dedicated server for one service
? Allocate with a fair driver.

@@quiz
Your customer SLA promises 99.9% availability, but the hosting supplier's contract says 99.5%. What is the problem?
* The underpinning contract can't support the SLA
- Nothing; SLAs are always higher
- The CMDB is out of date
- It's a problem only for the supplier
? Align supplier contracts with customer commitments.

@@quiz
Which supplier category deserves the most relationship effort?
* Strategic suppliers
- Commodity suppliers
- Operational suppliers
- All suppliers equally
? Critical and hard to replace.

@@flashcards
IT asset :: financially valuable component contributing to a service
asset life cycle :: plan, acquire, deploy, operate, retire
SAM :: software asset management
entitlement / deployment :: licences bought / installed or assigned
over-deployment :: compliance risk
harvesting :: reclaiming unused licences
downgrade rights :: higher edition licence covers a lower edition
CapEx / OpEx :: owning assets / paying as you use
direct / indirect cost :: one service / shared, allocated
showback / chargeback :: report costs / bill costs
TCO :: total cost of ownership over the life
supplier categories :: strategic, tactical, operational, commodity
underpinning contract :: supplier contract supporting your SLA
exit plan :: how to leave a supplier safely

@@drill
You own 80 licences and 65 are assigned. How many can be harvested for new staff?
= 15
? 80 − 65.

@@drill
A shared ¥900,000 cost is allocated by users: department A has 30 of 90 users. What is A's share in yen? (number only)
= 300000
= 300,000
? 900,000 × 30 / 90.

@@exercise Licence compliance position
Compare entitlements with deployments per product. A licence for a higher edition may cover a lower-edition install (downgrade rights) when the product allows it. Report, per product family, the shortfall (a compliance risk, with the cost to true up) or the surplus (licences to harvest, with the annual saving).
@@starter
EDITIONS = {"Office": ["Standard", "Pro"], "Diagram": ["Basic", "Plus"]}
DOWNGRADE = {"Office": True, "Diagram": False}
PRICE = {("Office", "Standard"): 30000, ("Office", "Pro"): 48000, ("Diagram", "Basic"): 9000, ("Diagram", "Plus"): 15000}

entitlements = {("Office", "Standard"): 120, ("Office", "Pro"): 40, ("Diagram", "Basic"): 25, ("Diagram", "Plus"): 10}
deployments = {("Office", "Standard"): 135, ("Office", "Pro"): 18, ("Diagram", "Basic"): 31, ("Diagram", "Plus"): 4}

def position(family):
    raise NotImplementedError

for family in EDITIONS:
    for line in position(family):
        print(line)
@@expected
Office Standard: 15 installs covered by higher-edition licences
Office: 7 spare licences, harvest to save up to ¥336,000
Diagram Plus: 6 spare, harvest to save ¥90,000
Diagram Basic: short 6, true-up ¥54,000
@@requires
def position
DOWNGRADE[
@@hint
Go from the highest edition down. For each edition, `spare = owned - used`. With downgrade rights, carry positive spare licences down to cover shortfalls in lower editions. Without them, each edition stands alone. Report shortfalls with the true-up cost (`shortfall * price`) and spare licences with the saving.
@@solution
EDITIONS = {"Office": ["Standard", "Pro"], "Diagram": ["Basic", "Plus"]}
DOWNGRADE = {"Office": True, "Diagram": False}
PRICE = {("Office", "Standard"): 30000, ("Office", "Pro"): 48000, ("Diagram", "Basic"): 9000, ("Diagram", "Plus"): 15000}

entitlements = {("Office", "Standard"): 120, ("Office", "Pro"): 40, ("Diagram", "Basic"): 25, ("Diagram", "Plus"): 10}
deployments = {("Office", "Standard"): 135, ("Office", "Pro"): 18, ("Diagram", "Basic"): 31, ("Diagram", "Plus"): 4}

def position(family):
    lines, carry = [], 0
    for edition in reversed(EDITIONS[family]):
        key = (family, edition)
        owned, used = entitlements[key], deployments[key]
        spare = owned - used
        if DOWNGRADE[family] and spare < 0 and carry > 0:
            covered = min(carry, -spare)
            lines.append(f"{family} {edition}: {covered} installs covered by higher-edition licences")
            spare += covered
            carry -= covered
        if spare < 0:
            lines.append(f"{family} {edition}: short {-spare}, true-up ¥{-spare * PRICE[key]:,}")
        elif DOWNGRADE[family]:
            carry += spare
        elif spare > 0:
            lines.append(f"{family} {edition}: {spare} spare, harvest to save ¥{spare * PRICE[key]:,}")
    if carry > 0:
        lines.append(f"{family}: {carry} spare licences, harvest to save up to ¥{carry * PRICE[(family, EDITIONS[family][-1])]:,}")
    return lines

for family in EDITIONS:
    for line in position(family):
        print(line)

@@exercise Showback by usage
Allocate shared IT costs to departments with a fair driver for each cost (service desk by tickets, storage by terabytes, the network by users), add each department's direct costs, and print a showback report with each department's total and share of the whole.
@@starter
departments = {
    "Sales": {"users": 120, "tickets": 900, "tb": 12, "direct": 2_400_000},
    "Engineering": {"users": 200, "tickets": 600, "tb": 80, "direct": 6_100_000},
    "Finance": {"users": 40, "tickets": 300, "tb": 8, "direct": 1_500_000},
}
shared = {"service desk": ("tickets", 3_600_000), "storage": ("tb", 2_000_000), "network": ("users", 1_800_000)}

def allocate(departments, shared):
    raise NotImplementedError

report = allocate(departments, shared)
grand = sum(r["total"] for r in report.values())
for name, r in report.items():
    parts = "  ".join(f"{k} ¥{v:>9,}" for k, v in r["shared"].items())
    print(f"{name:<12} direct ¥{r['direct']:>9,}  {parts}  total ¥{r['total']:>10,} ({r['total'] / grand:.0%})")
print(f"{'all':<12} total ¥{grand:,}")
@@expected
Sales        direct ¥2,400,000  service desk ¥1,800,000  storage ¥  240,000  network ¥  600,000  total ¥ 5,040,000 (29%)
Engineering  direct ¥6,100,000  service desk ¥1,200,000  storage ¥1,600,000  network ¥1,000,000  total ¥ 9,900,000 (57%)
Finance      direct ¥1,500,000  service desk ¥  600,000  storage ¥  160,000  network ¥  200,000  total ¥ 2,460,000 (14%)
all          total ¥17,400,000
@@requires
def allocate
round(
@@hint
For each shared cost, compute the driver total across departments, then each department's share: `round(amount * dept[driver] / driver_total)`. Store `{"direct": ..., "shared": {cost name: share}, "total": direct + sum(shares)}` per department.
@@solution
departments = {
    "Sales": {"users": 120, "tickets": 900, "tb": 12, "direct": 2_400_000},
    "Engineering": {"users": 200, "tickets": 600, "tb": 80, "direct": 6_100_000},
    "Finance": {"users": 40, "tickets": 300, "tb": 8, "direct": 1_500_000},
}
shared = {"service desk": ("tickets", 3_600_000), "storage": ("tb", 2_000_000), "network": ("users", 1_800_000)}

def allocate(departments, shared):
    report = {}
    for name, d in departments.items():
        parts = {}
        for cost, (driver, amount) in shared.items():
            driver_total = sum(x[driver] for x in departments.values())
            parts[cost] = round(amount * d[driver] / driver_total)
        report[name] = {"direct": d["direct"], "shared": parts, "total": d["direct"] + sum(parts.values())}
    return report

report = allocate(departments, shared)
grand = sum(r["total"] for r in report.values())
for name, r in report.items():
    parts = "  ".join(f"{k} ¥{v:>9,}" for k, v in r["shared"].items())
    print(f"{name:<12} direct ¥{r['direct']:>9,}  {parts}  total ¥{r['total']:>10,} ({r['total'] / grand:.0%})")
print(f"{'all':<12} total ¥{grand:,}")
