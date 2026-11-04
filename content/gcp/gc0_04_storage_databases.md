@@lesson gc0-storage-db Storage and Databases in Depth: Cloud Storage, Cloud SQL, Spanner, Firestore, Bigtable, BigQuery
@@text
Picking the right storage and database service is one of the most common architect decisions, and the exams test it with short scenario clues ("global, strongly consistent, relational" → Spanner). This lesson covers Cloud Storage classes and controls, then each database with the clue words that point to it.

## Cloud Storage

- Buckets have a **location type**: **region**, **dual-region** (two specific regions, optional turbo replication with a 15-minute RPO) or **multi-region** (e.g. `EU`, `US`, `ASIA`).
- **Storage classes**: all have **millisecond access** (unlike AWS Glacier Flexible or Azure Archive). Colder classes are cheaper to store, but charge **retrieval fees** and have a **minimum storage duration**:

| Class | Minimum duration | Typical access |
|---|---|---|
| **Standard** | none | frequent / hot |
| **Nearline** | 30 days | about once a month |
| **Coldline** | 90 days | about once a quarter |
| **Archive** | 365 days | less than once a year, DR copies |

- **Autoclass** moves objects between classes automatically by access.
- **Object Lifecycle Management**: `SetStorageClass` and `Delete` actions with conditions (age, createdBefore, numberOfNewerVersions, isLive, matchesPrefix).
- **Object Versioning** keeps noncurrent versions; **soft delete** keeps deleted objects for a window (7 days default).
- **Retention policy** + **Bucket Lock** (irreversible) for WORM compliance; **object holds**.
- Access: **uniform bucket-level access** (IAM only, recommended) instead of ACLs; **public access prevention**; **signed URLs** (time-limited access without a Google account); CMEK with Cloud KMS.
- Transfer: `gcloud storage cp/rsync`, **Storage Transfer Service** (from S3, Azure, HTTP, on-premises), **Transfer Appliance** for offline bulk.

## Databases: clue words

| Service | Model | Clue words |
|---|---|---|
| **Cloud SQL** | MySQL, PostgreSQL, SQL Server | "lift and shift relational", up to ~64 TB, regional; HA with a standby in another zone; read replicas (incl. cross-region) |
| **AlloyDB** | PostgreSQL-compatible | "high-performance PostgreSQL", analytics on transactional data |
| **Spanner** | relational, horizontally scalable | "**global**, **strongly consistent**, relational, 99.999%, unlimited scale" |
| **Firestore** | document (NoSQL) | mobile/web apps, offline sync, real-time listeners, serverless |
| **Bigtable** | wide-column NoSQL | "**time series**, IoT, ad tech, **petabytes**, single-digit ms, HBase API"; design the **row key** to avoid hotspots |
| **BigQuery** | serverless data warehouse | "analytics", "SQL over TBs/PBs", "data warehouse", BI |
| **Memorystore** | Redis/Valkey/Memcached | cache, sessions |
| **Database Migration Service** | | migrate MySQL/PostgreSQL/SQL Server/Oracle to Cloud SQL/AlloyDB |

Cloud SQL details: **automated backups** and **point-in-time recovery** (binary logs/WAL), **HA configuration** (regional, synchronous standby, automatic failover), **read replicas** (asynchronous, not for HA), private IP via private services access, **Cloud SQL Auth Proxy** / connectors with IAM authentication.

## BigQuery essentials

- Storage and compute are separate. Pricing: **on-demand** (per TB scanned) or **editions** (slot capacity with autoscaling). Storage: active vs long-term (cheaper after 90 days without modification).
- **Partitioned tables** (by ingestion time, a date/timestamp column, or integer range) and **clustered** columns cut the data scanned; `SELECT *` on a big table is expensive; preview tables instead of querying them.
- Use `--dry_run` or the query validator to see bytes processed before running; set **maximum bytes billed**.
- Datasets are regional or multi-regional; authorised views, row-level and column-level security; BigQuery ML; external tables over Cloud Storage (BigLake).

@@quiz
A global financial app needs a relational database with strong consistency across continents and 99.999% availability. Which service?
- Cloud SQL with read replicas
* Spanner
- Bigtable
- Firestore
? Clue words: global, strongly consistent, relational.

@@quiz
An IoT platform stores billions of sensor readings per day and needs single-digit-millisecond reads by device and time. Which service?
- BigQuery
* Bigtable
- Cloud SQL
- Firestore
? Time series at petabyte scale; design the row key carefully.

@@quiz
Backups must be kept for 7 years and nobody may delete them early, not even administrators. What do you configure on the bucket?
- Object Versioning
- Lifecycle delete after 7 years
* A retention policy locked with Bucket Lock
- Uniform bucket-level access
? Bucket Lock is irreversible.

@@quiz
How quickly can you read an object in the Archive storage class?
* In milliseconds, with a retrieval fee
- After rehydration of up to 12 hours
- Only after restoring to Standard
- Within 48 hours
? All Cloud Storage classes are online.

@@quiz
A BigQuery query over a 5 TB table filtered on one day is expensive. What reduces the cost most?
- Adding LIMIT 10
* Partitioning the table by date (and clustering)
- Using SELECT *
- Moving the table to another region
? LIMIT doesn't reduce bytes scanned.

@@quiz
Cloud SQL needs automatic failover if a zone fails. What do you enable?
* High availability (regional) configuration
- A read replica in the same zone
- Point-in-time recovery only
- Memorystore
? Read replicas are for scaling reads.

@@flashcards
bucket location types :: region, dual-region, multi-region
Nearline / Coldline / Archive minimum :: 30 / 90 / 365 days
storage class access :: all milliseconds
Autoclass :: automatic class transitions
lifecycle actions :: SetStorageClass, Delete
Bucket Lock :: irreversible retention
uniform bucket-level access :: IAM only, no ACLs
signed URL :: time-limited object access
Storage Transfer Service :: online transfers into GCS
Cloud SQL :: managed MySQL/PostgreSQL/SQL Server
Cloud SQL HA :: synchronous standby in another zone
AlloyDB :: high-performance PostgreSQL
Spanner :: global, strongly consistent relational
Firestore :: document DB for apps
Bigtable :: wide-column, time series, petabytes
Bigtable row key :: avoid hotspots
BigQuery :: serverless warehouse
BigQuery on-demand :: pay per TB scanned
partitioning + clustering :: scan less data
dry run :: bytes processed before running
Memorystore :: managed Redis/Valkey

@@drill
What is the minimum storage duration of Coldline, in days?
= 90
= 90 days
? Nearline 30, Archive 365.

@@drill
Which Google Cloud database is a global, horizontally scalable relational database?
= Spanner
= Cloud Spanner
? Clue: strong consistency worldwide.

@@exercise Choose a Cloud Storage class
For each data set compute the monthly cost in each class with the example prices (illustrative): storage per GB-month plus retrieval per GB read. If the data will be deleted before a class's minimum duration, add the remaining days' storage as an early deletion charge (pro-rated by 30-day months). Pick the cheapest class and print the costs.
@@lang python
@@starter
CLASSES = {
    "STANDARD": (0.020, 0.00, 0),
    "NEARLINE": (0.010, 0.01, 30),
    "COLDLINE": (0.004, 0.02, 90),
    "ARCHIVE": (0.0012, 0.05, 365),
}
datasets = [
    ("website images", 200, 400, 365),
    ("monthly reports", 1000, 100, 730),
    ("quarterly audits", 5000, 400, 1825),
    ("DR backups", 20000, 0, 2555),
    ("scratch files", 500, 50, 10),
]

def monthly_cost(cls, gb, read_gb, keep_days):
    raise NotImplementedError
@@expected
website images    -> STANDARD  (STAN $4.00, NEAR $6.00, COLD $8.80, ARCH $20.24)
monthly reports   -> COLDLINE  (STAN $20.00, NEAR $11.00, COLD $6.00, ARCH $6.20)
quarterly audits  -> ARCHIVE   (STAN $100.00, NEAR $54.00, COLD $28.00, ARCH $26.00)
DR backups        -> ARCHIVE   (STAN $400.00, NEAR $200.00, COLD $80.00, ARCH $24.00)
scratch files     -> COLDLINE  (STAN $10.00, NEAR $8.83, COLD $8.33, ARCH $10.20)
@@requires
def monthly_cost
min(
@@hint
Storage: `gb * price`. Retrieval: `read_gb * fee`. Early deletion: if `keep_days < min_days`, add `gb * price * (min_days - keep_days) / 30`.
@@solution
CLASSES = {
    "STANDARD": (0.020, 0.00, 0),
    "NEARLINE": (0.010, 0.01, 30),
    "COLDLINE": (0.004, 0.02, 90),
    "ARCHIVE": (0.0012, 0.05, 365),
}
datasets = [
    ("website images", 200, 400, 365),
    ("monthly reports", 1000, 100, 730),
    ("quarterly audits", 5000, 400, 1825),
    ("DR backups", 20000, 0, 2555),
    ("scratch files", 500, 50, 10),
]

def monthly_cost(cls, gb, read_gb, keep_days):
    price, fee, min_days = CLASSES[cls]
    cost = gb * price + read_gb * fee
    if keep_days < min_days:
        cost += gb * price * (min_days - keep_days) / 30
    return cost

for name, gb, read_gb, keep in datasets:
    costs = {cls: monthly_cost(cls, gb, read_gb, keep) for cls in CLASSES}
    best = min(costs, key=costs.get)
    detail = ", ".join(f"{c[:4]} ${v:,.2f}" for c, v in costs.items())
    print(f"{name:<17} -> {best:<9} ({detail})")

@@exercise Estimate BigQuery scan costs
Estimate bytes scanned for each query on a table partitioned by day and clustered by `country`: without a partition filter the query scans every partition; with `days` it scans only those partitions; a `country` filter on the clustered column scans about `CLUSTER_FRACTION` of each scanned partition; only the referenced columns count (column sizes are per day). Print the GB scanned and on-demand cost at the example price (illustrative), and flag queries over `MAX_GB`.
@@lang python
@@starter
PRICE_PER_TB = 6.25
MAX_GB = 500
CLUSTER_FRACTION = 0.1
TABLE_DAYS = 730
column_gb_per_day = {"event_time": 0.5, "user_id": 0.8, "country": 0.2, "url": 3.0, "payload": 12.0}
queries = [
    {"name": "select star, all time", "columns": list(column_gb_per_day), "days": None, "country": False},
    {"name": "daily active users", "columns": ["event_time", "user_id"], "days": 1, "country": False},
    {"name": "JP pages last week", "columns": ["event_time", "country", "url"], "days": 7, "country": True},
    {"name": "users per country, year", "columns": ["user_id", "country"], "days": 365, "country": False},
]
@@expected
select star, all time     12,045.0 GB  $  73.52  OVER LIMIT: add a partition filter or select fewer columns
daily active users             1.3 GB  $   0.01
JP pages last week             2.6 GB  $   0.02
users per country, year      365.0 GB  $   2.23
@@requires
def scanned_gb
TABLE_DAYS
@@hint
`per_day = sum(column_gb_per_day[c] for c in columns)`; multiply by the number of days scanned and by `CLUSTER_FRACTION` if filtering on country. Cost = GB / 1024 * price per TB.
@@solution
PRICE_PER_TB = 6.25
MAX_GB = 500
CLUSTER_FRACTION = 0.1
TABLE_DAYS = 730
column_gb_per_day = {"event_time": 0.5, "user_id": 0.8, "country": 0.2, "url": 3.0, "payload": 12.0}
queries = [
    {"name": "select star, all time", "columns": list(column_gb_per_day), "days": None, "country": False},
    {"name": "daily active users", "columns": ["event_time", "user_id"], "days": 1, "country": False},
    {"name": "JP pages last week", "columns": ["event_time", "country", "url"], "days": 7, "country": True},
    {"name": "users per country, year", "columns": ["user_id", "country"], "days": 365, "country": False},
]

def scanned_gb(q):
    per_day = sum(column_gb_per_day[c] for c in q["columns"])
    days = TABLE_DAYS if q["days"] is None else q["days"]
    gb = per_day * days
    if q["country"]:
        gb *= CLUSTER_FRACTION
    return gb

for q in queries:
    gb = scanned_gb(q)
    cost = gb / 1024 * PRICE_PER_TB
    flag = "  OVER LIMIT: add a partition filter or select fewer columns" if gb > MAX_GB else ""
    print(f"{q['name']:<24} {gb:>9,.1f} GB  ${cost:>7.2f}{flag}")
