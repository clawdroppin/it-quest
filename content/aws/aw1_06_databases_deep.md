@@lesson aw1-db-deep Databases in Depth: RDS, Aurora, DynamoDB, Caching and Analytics
@@text
"Which database?" is a staple of AWS exams and architecture reviews. This lesson compares the managed database services, explains the high-availability and read-scaling options of RDS and Aurora, shows how DynamoDB capacity and keys work, and where ElastiCache, Redshift and the purpose-built databases fit.

## Choosing a database

| Need | Service |
|---|---|
| Relational, existing engine (MySQL, PostgreSQL, MariaDB, Oracle, SQL Server, Db2) | **RDS** |
| Relational, cloud-native, high performance, fast failover | **Aurora** (MySQL/PostgreSQL compatible) |
| Key-value / document at any scale, single-digit-ms latency, serverless | **DynamoDB** |
| In-memory cache, sessions, leaderboards | **ElastiCache** (Redis OSS / Valkey / Memcached), **MemoryDB** (durable) |
| Data warehouse, analytics SQL over TBs–PBs | **Redshift** |
| Graph (social, fraud, knowledge graphs) | **Neptune** |
| MongoDB-compatible documents | **DocumentDB** |
| Time series (IoT, metrics) | **Timestream** |
| Wide-column, Cassandra-compatible | **Keyspaces** |
| Immutable, cryptographically verifiable ledger | QLDB was retired; use an append-only design or Aurora with auditing |
| SQL queries on S3 data | **Athena** |

## RDS

- Managed: patching, backups, monitoring, but **you choose instance size and storage**. No OS access (except RDS Custom).
- **Automated backups**: daily snapshot + transaction logs → **point-in-time restore** to any second within the retention (1–35 days). **Manual snapshots** are kept until deleted.
- **Multi-AZ (instance)**: **synchronous** replication to a standby in another AZ; automatic **failover** (DNS endpoint flips, typically 60–120 s); the standby **doesn't serve reads**. For **availability**.
- **Multi-AZ DB cluster** (MySQL/PostgreSQL): one writer and **two readable standbys**, faster failover.
- **Read replicas**: **asynchronous**, up to 15, can be cross-Region, can be **promoted** to standalone. For **read scaling** (and DR with cross-Region). Applications must send reads to the replica endpoint.
- **RDS Proxy**: pools connections (great for Lambda), faster failover handling, IAM auth.
- Encryption at rest with KMS must be chosen at creation: to encrypt an unencrypted DB, **snapshot → copy with encryption → restore**.
- **Storage auto scaling** grows storage when it runs low.

## Aurora

- Storage: **6 copies across 3 AZs**, auto-grows up to 128 TiB, self-healing; tolerates losing 2 copies for writes and 3 for reads.
- Up to **15 Aurora Replicas** sharing the same storage (low replica lag), failover usually under 30 s.
- Endpoints: **cluster (writer)**, **reader** (load-balances replicas), custom, instance.
- **Aurora Serverless v2**: scales capacity in fine-grained ACUs for variable workloads.
- **Global Database**: storage-level replication to up to 5 secondary Regions with typically < 1 s lag; promote a secondary in about a minute for DR.
- Backtrack (MySQL), cloning, zero-ETL integration with Redshift.

## DynamoDB

- Tables of **items** (max **400 KB**) with a **primary key**: a **partition key**, or partition + **sort key**. Choose a **high-cardinality** partition key that spreads traffic (avoid **hot partitions**).
- **Capacity modes**: **on-demand** (pay per request, instant scaling: unknown or spiky traffic) or **provisioned** (RCU/WCU with auto scaling: predictable traffic, cheaper at steady load).
  - **1 RCU** = one **strongly consistent** read per second of up to **4 KB**, or **two eventually consistent** reads.
  - **1 WCU** = one write per second of up to **1 KB**.
  - **Transactional** reads/writes cost **double**.
- **Indexes**: **GSI** (different partition/sort key, own capacity, eventually consistent, add any time) and **LSI** (same partition key, different sort key, created with the table only).
- **Streams** (item changes to Lambda), **TTL** (expire items for free), **Global Tables** (multi-Region, multi-active), **PITR** (35 days), **DAX** (in-memory cache, microsecond reads).
- Access patterns first: design keys around queries; scans are expensive.

## Caching

**ElastiCache** in front of a database reduces read load and latency. Strategies: **lazy loading / cache-aside** (read from cache, on a miss read DB and populate; data can be stale → use TTLs) and **write-through** (update cache on every write; no stale data, but caches unused items). Redis/Valkey offers replication, Multi-AZ failover, persistence, pub/sub and sorted sets; Memcached is simple, multi-threaded, no persistence.

## Analytics

**Redshift** (columnar MPP warehouse; Serverless option; Spectrum queries S3), **Athena** (serverless SQL on S3, pay per TB scanned: use Parquet and partitions), **Glue** (ETL and Data Catalog), **EMR** (Spark/Hadoop), **Kinesis** (streaming), **OpenSearch** (search and log analytics), **QuickSight** (dashboards), **Lake Formation** (data lake permissions).

@@quiz
An RDS database must survive an AZ failure with automatic failover. What do you enable?
* Multi-AZ deployment
- A read replica in the same AZ
- Automated backups only
- RDS Proxy
? Multi-AZ keeps a synchronous standby.

@@quiz
A reporting workload slows down the production RDS database. What helps most?
- Multi-AZ
* A read replica for the reports
- Bigger backups
- Storage auto scaling
? Replicas offload reads asynchronously.

@@quiz
How many RCUs are needed for 100 strongly consistent reads per second of 6 KB items?
- 100
- 150
* 200
- 600
? 6 KB rounds up to 2 × 4 KB = 2 RCUs per read.

@@quiz
Traffic to a new DynamoDB table is unpredictable and spiky. Which capacity mode?
* On-demand
- Provisioned without auto scaling
- Reserved capacity
- Provisioned with LSIs
? Pay per request, no capacity planning.

@@quiz
How do you encrypt an existing unencrypted RDS instance?
- Enable encryption in the console
* Snapshot it, copy the snapshot with encryption, restore from the copy
- Create an encrypted read replica of it
- Enable SSL
? Encryption at rest is set at creation.

@@quiz
A company needs a relational database replicated to another Region with under a second of lag and fast regional failover. What fits?
- RDS Multi-AZ
* Aurora Global Database
- DynamoDB Global Tables
- ElastiCache Global Datastore
? Storage-level replication to secondary Regions.

@@flashcards
RDS :: managed relational engines
Multi-AZ :: synchronous standby, availability
read replica :: asynchronous, read scaling, up to 15
point-in-time restore :: backups + logs, 1–35 days
RDS Proxy :: connection pooling (Lambda)
encrypt existing RDS :: snapshot, encrypted copy, restore
Aurora storage :: 6 copies across 3 AZs
Aurora replicas :: up to 15, shared storage
Aurora Serverless v2 :: scales in ACUs
Aurora Global Database :: < 1 s cross-Region lag
DynamoDB item max :: 400 KB
partition key :: high cardinality, avoid hot keys
1 RCU :: 1 strong / 2 eventual reads of 4 KB
1 WCU :: 1 write of 1 KB
transactional :: double capacity
GSI / LSI :: any time / at creation only
DAX :: DynamoDB microsecond cache
lazy loading :: populate cache on miss
write-through :: update cache on write
Redshift :: columnar data warehouse
Athena :: SQL on S3, pay per TB scanned

@@drill
What is the maximum size of a DynamoDB item?
= 400 KB
= 400KB
= 400
? Store large blobs in S3 with a pointer.

@@drill
How many KB does one WCU write per second?
= 1
= 1 KB
? RCUs cover 4 KB.

@@exercise DynamoDB capacity calculator
For each access pattern compute the RCUs or WCUs needed: reads round the item size up to 4 KB units (strong: 1 per unit, eventual: half, transactional: double), writes round up to 1 KB units (transactional: double). Print each pattern's capacity, the table totals, and the estimated monthly provisioned cost with the example prices (illustrative) for 730 hours.
@@lang python
@@starter
import math

PRICE_RCU_HOUR = 0.00013
PRICE_WCU_HOUR = 0.00065
patterns = [
    ("get order", "read", 3.5, 200, "strong"),
    ("list orders", "read", 12, 50, "eventual"),
    ("checkout", "read", 2, 20, "transactional"),
    ("put order", "write", 2.5, 40, "standard"),
    ("update stock", "write", 0.5, 100, "transactional"),
]

def capacity(kind, size_kb, per_second, mode):
    raise NotImplementedError
@@expected
get order      200/s x  3.5 KB strong        -> 200 RCU
list orders     50/s x   12 KB eventual      -> 75 RCU
checkout        20/s x    2 KB transactional -> 40 RCU
put order       40/s x  2.5 KB standard      -> 120 WCU
update stock   100/s x  0.5 KB transactional -> 200 WCU
Total: 315 RCU, 320 WCU, about $181.73/month provisioned
@@requires
def capacity
math.ceil(
@@hint
Reads: `units = math.ceil(size_kb / 4)`; eventual needs `math.ceil(units * per_second / 2)`. Writes: `units = math.ceil(size_kb)`.
@@solution
import math

PRICE_RCU_HOUR = 0.00013
PRICE_WCU_HOUR = 0.00065
patterns = [
    ("get order", "read", 3.5, 200, "strong"),
    ("list orders", "read", 12, 50, "eventual"),
    ("checkout", "read", 2, 20, "transactional"),
    ("put order", "write", 2.5, 40, "standard"),
    ("update stock", "write", 0.5, 100, "transactional"),
]

def capacity(kind, size_kb, per_second, mode):
    if kind == "read":
        units = math.ceil(size_kb / 4)
        if mode == "eventual":
            return math.ceil(units * per_second / 2)
        if mode == "transactional":
            return units * per_second * 2
        return units * per_second
    units = math.ceil(size_kb)
    return units * per_second * (2 if mode == "transactional" else 1)

rcu = wcu = 0
for name, kind, size, rate, mode in patterns:
    needed = capacity(kind, size, rate, mode)
    unit = "RCU" if kind == "read" else "WCU"
    if kind == "read":
        rcu += needed
    else:
        wcu += needed
    print(f"{name:<13} {rate:>4}/s x {size:>4} KB {mode:<13} -> {needed} {unit}")
cost = (rcu * PRICE_RCU_HOUR + wcu * PRICE_WCU_HOUR) * 730
print(f"Total: {rcu} RCU, {wcu} WCU, about ${cost:,.2f}/month provisioned")

@@exercise Recommend a database service
Recommend a service for each workload using rules checked in order: `graph` → Neptune; `analytics` over 1000 GB → Redshift; `cache` → ElastiCache; `key_value` → DynamoDB (add "Global Tables" when `multi_region`); `relational`: Aurora if `multi_region` or `high_performance`, otherwise RDS with the given `engine`. Add "+ Multi-AZ" for relational workloads with `ha`, and "+ read replicas" when `read_heavy`. Print each recommendation.
@@lang python
@@starter
workloads = [
    {"name": "legacy ERP", "model": "relational", "engine": "Oracle", "ha": True, "read_heavy": False},
    {"name": "shop catalogue", "model": "key_value", "multi_region": True},
    {"name": "fraud links", "model": "graph"},
    {"name": "BI warehouse", "model": "analytics", "size_gb": 8000},
    {"name": "session store", "model": "cache"},
    {"name": "booking app", "model": "relational", "engine": "PostgreSQL", "high_performance": True, "ha": True, "read_heavy": True},
    {"name": "intranet wiki", "model": "relational", "engine": "MySQL", "ha": False, "read_heavy": True},
]
@@expected
legacy ERP: RDS (Oracle) + Multi-AZ
shop catalogue: DynamoDB + Global Tables
fraud links: Neptune
BI warehouse: Redshift
session store: ElastiCache
booking app: Aurora (PostgreSQL) + Multi-AZ + read replicas
intranet wiki: RDS (MySQL) + read replicas
@@requires
def recommend
.get(
@@hint
Build a list of parts and join them with spaces; use `w.get("multi_region")` and friends for optional keys.
@@solution
workloads = [
    {"name": "legacy ERP", "model": "relational", "engine": "Oracle", "ha": True, "read_heavy": False},
    {"name": "shop catalogue", "model": "key_value", "multi_region": True},
    {"name": "fraud links", "model": "graph"},
    {"name": "BI warehouse", "model": "analytics", "size_gb": 8000},
    {"name": "session store", "model": "cache"},
    {"name": "booking app", "model": "relational", "engine": "PostgreSQL", "high_performance": True, "ha": True, "read_heavy": True},
    {"name": "intranet wiki", "model": "relational", "engine": "MySQL", "ha": False, "read_heavy": True},
]

def recommend(w):
    model = w["model"]
    if model == "graph":
        return "Neptune"
    if model == "analytics" and w.get("size_gb", 0) > 1000:
        return "Redshift"
    if model == "cache":
        return "ElastiCache"
    if model == "key_value":
        return "DynamoDB" + (" + Global Tables" if w.get("multi_region") else "")
    if w.get("multi_region") or w.get("high_performance"):
        parts = [f"Aurora ({w['engine']})"]
    else:
        parts = [f"RDS ({w['engine']})"]
    if w.get("ha"):
        parts.append("+ Multi-AZ")
    if w.get("read_heavy"):
        parts.append("+ read replicas")
    return " ".join(parts)

for w in workloads:
    print(f"{w['name']}: {recommend(w)}")
