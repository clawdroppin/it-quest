@@lesson gc2-migration-dr Migration and Disaster Recovery Planning: Waves, Strategies, RTO/RPO and Cost
@@text
Case-study questions on the Professional Cloud Architect exam often start with "the company wants to migrate..." or "the company must recover from a regional outage within...". Both are planning problems with a method behind them, and this lesson works through each method with code.

## Migration journey

Google's framework has four phases: **Assess** (inventory, dependencies, TCO; Migration Center does discovery), **Plan** (landing zone: resource hierarchy, identity, network, security baselines), **Deploy** (move workloads in waves), and **Optimize** (right-size, managed services, autoscaling).

### Strategies (the "Rs")

| Strategy | What changes | Google Cloud example |
|---|---|---|
| **Rehost** (lift and shift) | nothing in the app | Migrate to Virtual Machines onto Compute Engine |
| **Replatform** (lift and optimise) | small platform swaps | MySQL to Cloud SQL; app to GKE or Cloud Run |
| **Refactor / re-architect** | the code changes | monolith to microservices, Pub/Sub, Spanner |
| **Repurchase** | replace with SaaS | in-house mail to Workspace |
| **Retire** / **Retain** | switch off / keep on-premises for now | licence-bound mainframe kept with Interconnect |

### Waves

Group workloads into **waves** so that each wave only depends on workloads already in the cloud (or reachable over hybrid connectivity). Start with low-risk, low-dependency apps to build skills, and move tightly coupled, chatty systems together, since latency between on-premises and the cloud breaks them.

## Disaster recovery

- **RTO** (recovery time objective): the maximum acceptable downtime. **RPO** (recovery point objective): the maximum acceptable data loss, measured in time.
- Patterns, from cheapest to most expensive:

| Pattern | Typical RTO | Typical RPO | What runs in the DR region |
|---|---|---|---|
| **Backup and restore (cold)** | hours to days | hours (backup interval) | nothing; backups in a multi-region or dual-region bucket |
| **Warm standby (pilot light)** | tens of minutes | minutes (async replica) | a scaled-down copy, plus a cross-region read replica |
| **Hot (active-active / multi-region)** | ~0 | ~0 | full capacity, with Spanner or multi-region services |

- Data services decide the RPO: Cloud SQL cross-region replicas (async, minutes) or promotion; Spanner multi-region (synchronous, RPO 0); dual-region Cloud Storage with turbo replication (RPO 15 minutes); BigQuery dataset replication or cross-region copies.
- **Test** failover regularly (game days). An untested DR plan should be treated as a guess.

@@quiz
A company moves its PostgreSQL database to Cloud SQL without code changes and containerises its app for Cloud Run. Which strategy is this?
- Rehost
* Replatform
- Refactor
- Repurchase
? Small platform swaps, same code.

@@quiz
The requirement is RTO 4 hours and RPO 24 hours at the lowest cost. Which DR pattern fits?
* Backup and restore
- Warm standby
- Active-active multi-region
- No DR needed
? Daily backups in another region meet a 24-hour RPO.

@@quiz
Which database gives a near-zero RPO across regions?
- Cloud SQL with cross-region replicas
* Spanner with a multi-region configuration
- Firestore in a single region
- Memorystore
? Spanner replicates synchronously.

@@quiz
Why move tightly coupled, chatty applications in the same wave?
- They are cheaper to move
* Latency across the hybrid link between them would hurt performance
- Waves must contain exactly two apps
- To avoid IAM changes
? Split them only if the interfaces tolerate latency.

@@quiz
Which phase sets up the resource hierarchy, identity and network before workloads move?
- Assess
* Plan (the landing zone)
- Deploy
- Optimize
? Build the landing zone first.

@@flashcards
Assess / Plan / Deploy / Optimize :: migration phases
landing zone :: hierarchy, identity, network, security baseline
rehost :: lift and shift
replatform :: lift and optimise
refactor :: change the code
repurchase :: move to SaaS
retire / retain :: switch off / keep
migration wave :: group with satisfied dependencies
RTO :: maximum downtime
RPO :: maximum data loss
backup and restore :: cheapest, slowest
warm standby :: scaled-down copy running
active-active :: near-zero RTO and RPO
Spanner multi-region :: synchronous, RPO 0
turbo replication :: dual-region bucket, RPO 15 min
game day :: practise failover

@@drill
Which recovery objective measures how much data, in time, you can afford to lose?
= RPO
= recovery point objective
? RTO is about downtime.

@@drill
Which migration strategy replaces an application with a SaaS product?
= repurchase
? Like moving to Workspace.

@@exercise Plan migration waves
Given applications with dependencies, put each app in the earliest wave in which all its dependencies have already moved (or will stay reachable on-premises), keeping chatty pairs together. Then suggest a strategy for each app from its attributes.
@@concepts maps, loops, lists
@@lang python
@@starter
apps = {
    "intranet":   {"deps": [], "custom_code": False, "licence_bound": False, "saas_option": True},
    "auth":       {"deps": [], "custom_code": True, "licence_bound": False, "saas_option": False},
    "orders-db":  {"deps": [], "custom_code": False, "licence_bound": False, "saas_option": False},
    "orders-api": {"deps": ["auth", "orders-db"], "custom_code": True, "licence_bound": False, "saas_option": False},
    "reporting":  {"deps": ["orders-db"], "custom_code": True, "licence_bound": False, "saas_option": False},
    "web-shop":   {"deps": ["orders-api", "auth"], "custom_code": True, "licence_bound": False, "saas_option": False},
    "mainframe":  {"deps": [], "custom_code": True, "licence_bound": True, "saas_option": False},
    "billing":    {"deps": ["mainframe", "orders-api"], "custom_code": True, "licence_bound": False, "saas_option": False},
}
stays = {"mainframe"}                 # retained on-premises, reachable via Interconnect
chatty = [("orders-api", "orders-db")]  # must move together

def plan_waves():
    raise NotImplementedError
@@expected
wave 1: auth, intranet
wave 2: orders-api, orders-db, reporting
wave 3: billing, web-shop
retained: mainframe
  intranet    repurchase (move to SaaS)
  auth        refactor later; replatform first (many dependants)
  orders-db   rehost or managed service (e.g. Cloud SQL)
  orders-api  refactor later; replatform first (many dependants)
  reporting   replatform (containers on Cloud Run or GKE)
  web-shop    replatform (containers on Cloud Run or GKE)
  mainframe   retain (keep on-premises, connect with Interconnect)
  billing     replatform (containers on Cloud Run or GKE)
@@requires
def plan_waves
stays
chatty
@@hint
Treat apps in `stays` as already satisfied. Each round, an app is ready when every dependency has moved or stays. For each chatty pair, delay the earlier app to the later one's wave so they move together. Strategy: retain if licence-bound, repurchase if a SaaS option exists, replatform if it's custom code that depends on a database, refactor if it's a custom app others depend on heavily, otherwise rehost.
@@solution
apps = {
    "intranet":   {"deps": [], "custom_code": False, "licence_bound": False, "saas_option": True},
    "auth":       {"deps": [], "custom_code": True, "licence_bound": False, "saas_option": False},
    "orders-db":  {"deps": [], "custom_code": False, "licence_bound": False, "saas_option": False},
    "orders-api": {"deps": ["auth", "orders-db"], "custom_code": True, "licence_bound": False, "saas_option": False},
    "reporting":  {"deps": ["orders-db"], "custom_code": True, "licence_bound": False, "saas_option": False},
    "web-shop":   {"deps": ["orders-api", "auth"], "custom_code": True, "licence_bound": False, "saas_option": False},
    "mainframe":  {"deps": [], "custom_code": True, "licence_bound": True, "saas_option": False},
    "billing":    {"deps": ["mainframe", "orders-api"], "custom_code": True, "licence_bound": False, "saas_option": False},
}
stays = {"mainframe"}                 # retained on-premises, reachable via Interconnect
chatty = [("orders-api", "orders-db")]  # must move together

def plan_waves():
    wave_of, done, n = {}, set(stays), 1
    remaining = [a for a in apps if a not in stays]
    while remaining:
        ready = [a for a in remaining if all(d in done for d in apps[a]["deps"])]
        if not ready:
            raise ValueError(f"circular dependencies: {remaining}")
        for a in ready:
            wave_of[a] = n
        done.update(ready)
        remaining = [a for a in remaining if a not in ready]
        n += 1
    for a, b in chatty:
        latest = max(wave_of[a], wave_of[b])
        wave_of[a] = wave_of[b] = latest
    return wave_of

def strategy(name):
    a = apps[name]
    dependants = sum(name in other["deps"] for other in apps.values())
    if a["licence_bound"]:
        return "retain (keep on-premises, connect with Interconnect)"
    if a["saas_option"]:
        return "repurchase (move to SaaS)"
    if not a["custom_code"]:
        return "rehost or managed service (e.g. Cloud SQL)"
    if dependants >= 2:
        return "refactor later; replatform first (many dependants)"
    return "replatform (containers on Cloud Run or GKE)"

waves = plan_waves()
for w in sorted(set(waves.values())):
    print(f"wave {w}: {', '.join(sorted(a for a in waves if waves[a] == w))}")
print(f"retained: {', '.join(sorted(stays))}")
for name in apps:
    print(f"  {name:<11} {strategy(name)}")

@@exercise Choose a DR pattern per workload
For each workload, pick the cheapest DR pattern whose typical RTO and RPO meet the requirement. Then estimate its monthly DR cost from the size of the primary environment, and flag requirements that only active-active can meet.
@@concepts conditions, numbers, lists
@@lang python
@@starter
# (name, typical RTO minutes, typical RPO minutes, DR cost as a fraction of primary)
PATTERNS = [("backup and restore", 480, 1440, 0.05), ("warm standby", 30, 5, 0.35), ("active-active", 1, 0, 1.0)]
workloads = [
    ("internal wiki", 2880, 1440, 800),          # RTO min, RPO min, primary monthly cost in $
    ("order processing", 60, 15, 12_000),
    ("payments ledger", 5, 0, 20_000),
    ("analytics warehouse", 720, 1440, 9_000),
    ("customer portal", 240, 60, 6_000),
]

def choose(rto, rpo):
    raise NotImplementedError
@@expected
internal wiki        RTO 2880 min, RPO 1440 min -> backup and restore $      40/month
order processing     RTO   60 min, RPO   15 min -> warm standby       $   4,200/month
payments ledger      RTO    5 min, RPO    0 min -> active-active      $  20,000/month  <- needs synchronous replication (e.g. Spanner multi-region)
analytics warehouse  RTO  720 min, RPO 1440 min -> backup and restore $     450/month
customer portal      RTO  240 min, RPO   60 min -> warm standby       $   2,100/month
total DR cost: $26,790/month
@@requires
def choose
PATTERNS
rpo
@@hint
Patterns are ordered from cheapest to most expensive, so return the first one whose RTO and RPO are both less than or equal to the requirement. The DR cost is the primary cost times the pattern's fraction. Sum the totals at the end.
@@solution
# (name, typical RTO minutes, typical RPO minutes, DR cost as a fraction of primary)
PATTERNS = [("backup and restore", 480, 1440, 0.05), ("warm standby", 30, 5, 0.35), ("active-active", 1, 0, 1.0)]
workloads = [
    ("internal wiki", 2880, 1440, 800),          # RTO min, RPO min, primary monthly cost in $
    ("order processing", 60, 15, 12_000),
    ("payments ledger", 5, 0, 20_000),
    ("analytics warehouse", 720, 1440, 9_000),
    ("customer portal", 240, 60, 6_000),
]

def choose(rto, rpo):
    for name, p_rto, p_rpo, fraction in PATTERNS:
        if p_rto <= rto and p_rpo <= rpo:
            return name, fraction
    return None, None

total = 0
for name, rto, rpo, primary in workloads:
    pattern, fraction = choose(rto, rpo)
    cost = primary * fraction
    total += cost
    note = "  <- needs synchronous replication (e.g. Spanner multi-region)" if pattern == "active-active" else ""
    print(f"{name:<20} RTO {rto:>4} min, RPO {rpo:>4} min -> {pattern:<18} ${cost:>8,.0f}/month{note}")
print(f"total DR cost: ${total:,.0f}/month")
