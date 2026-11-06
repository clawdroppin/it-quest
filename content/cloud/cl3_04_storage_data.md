@@lesson cl3-storage-data Cloud Storage and Data: Block, File, Object, Databases, Sharding and Lifecycle
@@lang python
@@text
Compute is easy to replace in the cloud: terminate an instance and start another. Data is the part you can't throw away, and it drives most of the cost and most of the hard decisions. This lesson covers the kinds of storage, how managed databases differ, how data is spread over many machines, and how lifecycle rules keep the bill under control.

## Three kinds of storage

| Kind | What you get | Examples | Typical use |
|---|---|---|---|
| **Block** | a raw disk attached to one VM; the OS puts a file system on it | AWS EBS, Azure Managed Disks, GCP Persistent Disk | boot disks, databases on VMs |
| **File** | a shared file system over NFS or SMB | AWS EFS/FSx, Azure Files, GCP Filestore | shared home directories, lift-and-shift apps |
| **Object** | an HTTP API storing whole objects with metadata in buckets | S3, Azure Blob Storage, Cloud Storage | backups, media, data lakes, static websites |

Key differences:

- Block storage is **low latency** and supports in-place writes, but usually attaches to **one VM** at a time (multi-attach exists with limits). Performance is sold as **IOPS** and **throughput**, often tied to disk size or a provisioned tier.
- File storage is **shared** across many clients, with POSIX or SMB semantics (locks, permissions, renames).
- Object storage is **massively scalable** and cheap per GB, but you replace whole objects (no in-place edits), and you access it over HTTP. There are no real directories: `logs/2026/11/app.log` is just a key with slashes in it.
- **Instance (ephemeral) storage** is a local disk on the host: very fast, and **gone when the instance stops**. Use it for caches and scratch space only.

## Durability and availability

These are different promises:

- **Durability**: the chance your data survives. S3 Standard is designed for eleven nines (99.999999999%) by storing copies across several facilities.
- **Availability**: the chance you can reach it right now, typically 99.9–99.99% for object storage.

Replication options climb in cost and protection: **locally redundant** (copies in one data centre), **zone-redundant** (across availability zones in a region), and **geo-redundant** (to a second region, often asynchronously, so a regional failover can lose the last few minutes of writes).

**Snapshots** of block volumes are point-in-time and incremental (only changed blocks are stored after the first). A snapshot of a running database disk is only **crash-consistent** unless you flush or freeze the application first; for databases, prefer the database's own backups.

## Object storage classes and lifecycle

Providers price object storage by how often you'll read it:

| Class (AWS / Azure / GCP names) | Storage price | Retrieval | Minimum storage duration |
|---|---|---|---|
| Standard / Hot / Standard | highest | free | none |
| Infrequent Access / Cool / Nearline | lower | per-GB fee | about 30 days |
| Archive-ish (Glacier Instant/Flexible, Cold, Coldline) | lower still | higher fee | 90 days |
| Deep archive (Glacier Deep Archive / Archive / Archive) | lowest | hours to retrieve, highest fee | 180–365 days |

**Lifecycle rules** move objects between classes and delete them by age, prefix or tags: "move `logs/` to infrequent access after 30 days, to archive after 90, delete after 365". Watch the **minimum storage duration**: deleting or moving an object early still bills the remaining days. Many small objects are a trap too, because some classes add per-object overhead and every transition is a billed request.

Other object storage features worth knowing:

- **Versioning** keeps old versions on overwrite and delete (and costs storage for them; pair it with a lifecycle rule for noncurrent versions).
- **Object lock / immutability** (WORM) protects backups from ransomware and from your own admins for a retention period.
- **Pre-signed URLs** give time-limited access to one object without making the bucket public.
- **Block public access** settings at the account level are the safety net against leaky buckets.

## Managed databases

Running a database on a VM means you own patching, backups, replication and failover. A **managed** database service does those for you:

- **Relational** (Amazon RDS/Aurora, Azure SQL, Cloud SQL/AlloyDB): SQL, transactions, joins. Scale **up** (bigger instance) and add **read replicas** for read-heavy work. High availability uses a **synchronous standby** in another zone with automatic failover; read replicas are usually **asynchronous** and can lag.
- **Key-value / wide-column** (DynamoDB, Bigtable, Cassandra): huge scale and predictable latency for access by key; you design tables around your queries.
- **Document** (MongoDB/Atlas, Cosmos DB, Firestore): JSON documents with flexible schemas.
- **In-memory caches** (ElastiCache/Memorystore/Azure Cache for Redis): microsecond reads; use **cache-aside** (read the cache, on a miss read the database and fill the cache) with a TTL.
- **Data warehouses** (Redshift, BigQuery, Synapse/Fabric): columnar storage for analytics over billions of rows, not for transactions.

The **CAP theorem** says that during a network partition a distributed store must choose between **consistency** and **availability**. Many cloud databases let you choose per request: **strong** reads (always the latest) cost more latency than **eventual** reads.

## Spreading data: partitioning and sharding

One machine eventually runs out of disk, memory or write capacity, so large systems split data into **partitions** (shards), each owned by a node.

- **Range partitioning** (by date, by ID range) makes range scans easy, but new writes can all hit the newest partition (a **hot partition**).
- **Hash partitioning** spreads keys evenly. The naive version, `hash(key) % N`, has a nasty property: change N from 4 to 5 and roughly **80% of keys move** to a different node.
- **Consistent hashing** puts nodes and keys on a ring; each key belongs to the next node clockwise. Adding a node only moves the keys between it and its neighbour, about **1/N** of them. **Virtual nodes** (many points per physical node) keep the spread even. DynamoDB, Cassandra and many caches use this idea.
- Choose a **partition key** with many distinct values and even traffic. A key like `country` puts most users in a few partitions; `user_id` spreads well.

## Moving data and what it costs

- **Ingress** (data into the cloud) is usually free; **egress** (out to the internet, and often between regions or zones) is billed per GB. Architectures that bounce data across regions or out to on-premises analytics can cost more in egress than in compute.
- Bulk transfers use offline appliances (AWS Snowball, Azure Data Box, Google Transfer Appliance) when the network would take weeks: 100 TB over a dedicated 1 Gb/s link takes more than nine days at full speed.
- **Private connectivity** (Direct Connect, ExpressRoute, Cloud Interconnect) gives predictable bandwidth and usually lower egress rates.

## Protecting data

- Encrypt at rest (on by default nearly everywhere) and in transit (TLS). Use **customer-managed keys** when policy requires control over rotation and revocation.
- Back up with the **3-2-1** rule, and keep at least one copy **immutable** or in a separate account, so one compromised account can't delete everything.
- **Test restores.** A backup you haven't restored is a hope, not a plan. Measure the actual **RTO** and **RPO** against the targets.

@@quiz
Which storage type is the best fit for millions of user-uploaded images served over HTTP?
* Object storage
- Block storage
- Instance store
- A file share
? Object storage scales without limits, is cheap per GB and is accessed over HTTP.

@@quiz
What happens to data on an instance (ephemeral) store when the instance is stopped?
* It is lost
- It is snapshotted automatically
- It moves to object storage
- It is kept until the instance is terminated
? Use it only for caches and scratch data.

@@quiz
An object is moved to an archive class with a 90-day minimum and deleted after 20 days. How is it billed?
* For the full 90 days of archive storage
- For 20 days only
- Nothing, because it was deleted
- Double, as a penalty
? Early deletion is billed up to the minimum storage duration.

@@quiz
A cluster uses hash(key) % N to choose a node and grows from 4 to 5 nodes. Roughly what share of keys move?
* About 80%
- About 20%
- About 50%
- None
? Only keys whose hash gives the same result mod 4 and mod 5 stay put; consistent hashing moves only about 1/N.

@@quiz
How do read replicas usually differ from a high-availability standby?
* Replicas are asynchronous and serve reads; the standby is synchronous and exists for failover
- Replicas are synchronous; the standby is asynchronous
- Both serve writes
- Replicas only exist in other regions
? Reads from a replica may be slightly stale.

@@quiz
What does object lock (WORM) protect against?
* Deletion or overwriting of backups during the retention period, even by admins or ransomware
- Slow retrieval from archive classes
- Egress charges
- Unencrypted uploads
? Combine it with a separate backup account for strong protection.

@@flashcards
Block storage :: a raw disk attached to a VM (EBS, Managed Disks, Persistent Disk)
File storage :: a shared file system over NFS or SMB
Object storage :: HTTP API for whole objects in buckets
Durability :: the chance data survives (eleven nines for S3 Standard)
Lifecycle rule :: moves or deletes objects by age, prefix or tag
Minimum storage duration :: early deletion is still billed to this length
Read replica :: asynchronous copy for reads; may lag
Cache-aside :: read cache, on a miss read the DB and fill the cache
Consistent hashing :: adding a node moves only about 1/N of keys
Hot partition :: one shard gets most of the traffic
Egress :: data leaving the cloud or region, billed per GB
Object lock :: write-once retention against deletion

@@drill
How many days does 100 TB take over a 1 Gb/s link at full speed? (round up to a whole day)
= 10
= 10 days
? 100 × 8,000 Gb ÷ 1 Gb/s = 800,000 s ≈ 9.3 days.

@@drill
Which caching pattern reads the cache first and fills it from the database on a miss?
= cache-aside
= cache aside
= lazy loading

@@exercise Simulate a lifecycle policy
@@concepts numbers, lists, loops
A bucket of logs gets 500 GB of new objects every month. A lifecycle rule keeps objects in **standard** for 30 days, moves them to **infrequent access** until day 90, then to **archive** until day 365, and deletes them after that. Compute the steady-state monthly storage bill (after a year of running, with 12 monthly batches aged 0, 30, 60, ... 330 days) with and without the rule, plus the one-off monthly transition request fees.
@@starter
PRICE_GB = {"standard": 0.023, "infrequent": 0.0125, "archive": 0.004}
MONTHLY_GB = 500
OBJECTS_PER_BATCH = 2_000_000
TRANSITION_FEE_PER_1000 = {"infrequent": 0.01, "archive": 0.03}

def storage_class(age_days):
    raise NotImplementedError

def monthly_bill(with_rule):
    raise NotImplementedError

for with_rule in (False, True):
    total, by_class = monthly_bill(with_rule)
    label = "with lifecycle rule" if with_rule else "all standard       "
    detail = ", ".join(f"{c} {gb:,} GB" for c, gb in by_class.items())
    print(f"{label}: ${total:,.2f}/month ({detail})")
fees = sum(OBJECTS_PER_BATCH / 1000 * fee for fee in TRANSITION_FEE_PER_1000.values())
print(f"transition requests: ${fees:,.2f}/month (each batch moves twice)")
@@expected
all standard       : $138.00/month (standard 6,000 GB)
with lifecycle rule: $42.00/month (standard 500 GB, infrequent 1,000 GB, archive 4,500 GB)
transition requests: $80.00/month (each batch moves twice)
@@requires
def storage_class
def monthly_bill
range(0, 360, 30)
@@hint
`storage_class` returns "standard" below 30 days, "infrequent" below 90, "archive" below 365, otherwise None (deleted). `monthly_bill` walks the batch ages `range(0, 360, 30)`, puts each 500 GB batch into its class (or "standard" when the rule is off), and returns the total cost and a dict of GB per class.
@@solution
PRICE_GB = {"standard": 0.023, "infrequent": 0.0125, "archive": 0.004}
MONTHLY_GB = 500
OBJECTS_PER_BATCH = 2_000_000
TRANSITION_FEE_PER_1000 = {"infrequent": 0.01, "archive": 0.03}

def storage_class(age_days):
    if age_days < 30:
        return "standard"
    if age_days < 90:
        return "infrequent"
    if age_days < 365:
        return "archive"
    return None

def monthly_bill(with_rule):
    by_class = {}
    for age in range(0, 360, 30):
        cls = storage_class(age) if with_rule else "standard"
        if cls is None:
            continue
        by_class[cls] = by_class.get(cls, 0) + MONTHLY_GB
    total = sum(PRICE_GB[c] * gb for c, gb in by_class.items())
    return total, by_class

for with_rule in (False, True):
    total, by_class = monthly_bill(with_rule)
    label = "with lifecycle rule" if with_rule else "all standard       "
    detail = ", ".join(f"{c} {gb:,} GB" for c, gb in by_class.items())
    print(f"{label}: ${total:,.2f}/month ({detail})")
fees = sum(OBJECTS_PER_BATCH / 1000 * fee for fee in TRANSITION_FEE_PER_1000.values())
print(f"transition requests: ${fees:,.2f}/month (each batch moves twice)")

@@exercise Modulo hashing vs consistent hashing
@@concepts maps, numbers, loops
Place 10,000 keys on 4 nodes, then add a fifth node, and count how many keys move. Compare **modulo hashing** (`hash % N`) with a **consistent hash ring** using 100 virtual nodes per physical node. Use MD5 so the hashes are the same on every run, and also print how evenly the ring spreads the keys over the five nodes.
@@starter
import bisect
import hashlib

def h(text):
    return int(hashlib.md5(text.encode()).hexdigest(), 16)

KEYS = [f"user-{i}" for i in range(10_000)]

def modulo_owner(key, nodes):
    return nodes[h(key) % len(nodes)]

def build_ring(nodes, vnodes=100):
    raise NotImplementedError

def ring_owner(ring, key):
    raise NotImplementedError

def moved(before, after):
    return sum(1 for k in KEYS if before[k] != after[k])

old_nodes = ["n1", "n2", "n3", "n4"]
new_nodes = old_nodes + ["n5"]

mod_before = {k: modulo_owner(k, old_nodes) for k in KEYS}
mod_after = {k: modulo_owner(k, new_nodes) for k in KEYS}
ring_old, ring_new = build_ring(old_nodes), build_ring(new_nodes)
ring_before = {k: ring_owner(ring_old, k) for k in KEYS}
ring_after = {k: ring_owner(ring_new, k) for k in KEYS}

print(f"modulo hashing:     {moved(mod_before, mod_after) / len(KEYS):.1%} of keys moved")
print(f"consistent hashing: {moved(ring_before, ring_after) / len(KEYS):.1%} of keys moved")
counts = {n: 0 for n in new_nodes}
for owner in ring_after.values():
    counts[owner] += 1
print("ring spread:", ", ".join(f"{n}={c}" for n, c in counts.items()))
@@expected
modulo hashing:     79.8% of keys moved
consistent hashing: 21.6% of keys moved
ring spread: n1=1711, n2=1795, n3=2186, n4=2148, n5=2160
@@requires
def build_ring
def ring_owner
bisect.bisect(
@@hint
`build_ring` returns a sorted list of `(h(f"{node}#{i}"), node)` for every node and `i` in `range(vnodes)`. `ring_owner` finds the first point clockwise from the key's hash: `i = bisect.bisect(ring, (h(key),))`, wrapping to 0 when `i == len(ring)`.
@@solution
import bisect
import hashlib

def h(text):
    return int(hashlib.md5(text.encode()).hexdigest(), 16)

KEYS = [f"user-{i}" for i in range(10_000)]

def modulo_owner(key, nodes):
    return nodes[h(key) % len(nodes)]

def build_ring(nodes, vnodes=100):
    return sorted((h(f"{node}#{i}"), node) for node in nodes for i in range(vnodes))

def ring_owner(ring, key):
    i = bisect.bisect(ring, (h(key),))
    if i == len(ring):
        i = 0
    return ring[i][1]

def moved(before, after):
    return sum(1 for k in KEYS if before[k] != after[k])

old_nodes = ["n1", "n2", "n3", "n4"]
new_nodes = old_nodes + ["n5"]

mod_before = {k: modulo_owner(k, old_nodes) for k in KEYS}
mod_after = {k: modulo_owner(k, new_nodes) for k in KEYS}
ring_old, ring_new = build_ring(old_nodes), build_ring(new_nodes)
ring_before = {k: ring_owner(ring_old, k) for k in KEYS}
ring_after = {k: ring_owner(ring_new, k) for k in KEYS}

print(f"modulo hashing:     {moved(mod_before, mod_after) / len(KEYS):.1%} of keys moved")
print(f"consistent hashing: {moved(ring_before, ring_after) / len(KEYS):.1%} of keys moved")
counts = {n: 0 for n in new_nodes}
for owner in ring_after.values():
    counts[owner] += 1
print("ring spread:", ", ".join(f"{n}={c}" for n, c in counts.items()))
