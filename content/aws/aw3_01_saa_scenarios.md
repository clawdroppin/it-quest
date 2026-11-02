@@lesson aw3-saa Solutions Architect Associate: Exam Domains and Scenarios
@@concepts maps, numbers, conditions
@@lang python
@@text
The **AWS Certified Solutions Architect – Associate (SAA-C03)** is 65 questions in 130 minutes, scored 100–1000 with 720 to pass. Almost every question is a short scenario ending in "which solution meets these requirements **most cost-effectively** / with the **least operational overhead** / with the **highest availability**?". Several answers would work; you're choosing the one that fits the qualifier. This lesson maps the four domains, teaches the keywords that point to services, and the exercises rebuild three things the exam tests constantly: IAM policy evaluation, S3 storage class choice, and capacity sizing.

| Domain | Weight |
|---|---|
| 1. Design secure architectures | 30% |
| 2. Design resilient architectures | 26% |
| 3. Design high-performing architectures | 24% |
| 4. Design cost-optimized architectures | 20% |

## Reading the qualifier

| The question says | Lean towards |
|---|---|
| "least operational overhead" | managed and serverless: Lambda, Fargate, DynamoDB, Aurora Serverless, S3, managed services over EC2 you run |
| "most cost-effective" | the cheapest option that still meets *every* stated requirement: Spot for interruptible, S3 lifecycle tiers, Savings Plans for steady load, gp3 over io2 |
| "highly available" | Multi-AZ, Auto Scaling across AZs, ELB, Route 53 health checks |
| "fault tolerant" / "no data loss" | synchronous replication (Multi-AZ RDS, Aurora storage), SQS between tiers |
| "decouple" | SQS (queue, one consumer), SNS (fan-out), EventBridge (event routing), Kinesis (ordered streams) |
| "in near real time" | Kinesis Data Streams / Firehose, DynamoDB Streams, EventBridge |
| "global users, low latency" | CloudFront, Global Accelerator, Route 53 latency routing, DynamoDB global tables, Aurora Global Database |
| "without changing the application" | lift and shift: EC2, RDS for the same engine, Storage Gateway, Application Migration Service |

## Domain 1: secure architectures

- **IAM**: users, groups, roles, policies; identity vs resource policies; **an explicit Deny always wins**, then any Allow, otherwise implicit deny (the first exercise). **SCPs** limit what accounts can do, even admins. Permission boundaries cap delegated roles. Cross-account access with roles and `sts:AssumeRole`.
- **Federation**: IAM Identity Center for the workforce; **Cognito** user pools (sign-up/sign-in for app users) and identity pools (temporary AWS credentials for them).
- **Network**: security groups (stateful) vs NACLs (stateless, can deny); private subnets plus NAT; **VPC endpoints** (gateway endpoints for S3 and DynamoDB are free; interface endpoints for the rest); **WAF** on ALB/CloudFront/API Gateway; **Shield** Standard (free) and Advanced; **Network Firewall**.
- **Data protection**: **KMS** (customer managed keys, key policies, envelope encryption), **CloudHSM** for dedicated HSMs; **ACM** for TLS certificates; S3 encryption (SSE-S3, SSE-KMS, SSE-C, client-side), **bucket policies**, Block Public Access, Object Lock (WORM); **Secrets Manager** (rotation) vs **Parameter Store** (cheaper, no built-in rotation); **Macie** finds PII in S3.
- **Detection**: CloudTrail, Config, **GuardDuty** (threats), **Inspector** (vulnerabilities), **Security Hub** (one view), **Detective**.

## Domain 2: resilient architectures

- **Decoupling**: SQS standard (at-least-once, best-effort order) vs **FIFO** (exactly-once processing, ordered, 300 msg/s or 3,000 with batching, higher with high-throughput mode); visibility timeout; dead-letter queues. SNS + SQS fan-out. Step Functions for workflows.
- **Scaling**: Auto Scaling groups with target tracking, step and scheduled policies; launch templates; health checks from ELB; warm pools.
- **Databases**: RDS **Multi-AZ** (synchronous standby, automatic failover, *not* for reads) vs **read replicas** (asynchronous, for reads, can be cross-region and promoted). **Aurora**: storage replicated six ways across 3 AZs, up to 15 replicas, Global Database (<1 s replication). DynamoDB: multi-AZ by default, global tables, PITR, on-demand backup.
- **DR**: backup and restore, pilot light, warm standby, multi-site (see the CloudOps lesson in the Cloud track); AWS Backup; **Elastic Disaster Recovery**.
- **Route 53**: failover, weighted, latency, geolocation routing with health checks.

## Domain 3: high-performing architectures

- **Compute**: instance families (general **M/T**, compute **C**, memory **R/X**, storage **I/D**, accelerated **P/G/Inf**); Graviton for price-performance; **Lambda** (15 min max, up to 10 GB memory, provisioned concurrency for cold starts); **ECS/EKS** on Fargate or EC2; **Batch**.
- **Storage**: EBS **gp3** (baseline 3,000 IOPS, tune separately), **io2** Block Express for the highest IOPS, st1/sc1 HDD for throughput/cold; instance store (fast, ephemeral); **EFS** (NFS, many instances, Linux), **FSx** for Windows File Server, Lustre (HPC), NetApp ONTAP, OpenZFS; S3 with multipart upload and **Transfer Acceleration**.
- **Caching**: CloudFront at the edge; **ElastiCache** (Redis/Valkey for rich features and replication, Memcached for simple multithreaded caching); **DAX** for DynamoDB microsecond reads.
- **Data and analytics**: Kinesis, **Athena** (SQL on S3), **Glue** (ETL, catalog), **Redshift** (warehouse), **EMR**, **OpenSearch**, **QuickSight**, Lake Formation.
- **Networking**: placement groups (cluster for low latency, spread for isolation, partition for big distributed systems); enhanced networking/ENA; **Global Accelerator** for TCP/UDP with static anycast IPs.

## Domain 4: cost-optimized architectures

- Purchase options: On-Demand, **Savings Plans** (Compute: any family/region/Lambda/Fargate; EC2 Instance: one family in one region, bigger discount), **Reserved Instances**, **Spot** (up to 90% off, 2-minute warning), Dedicated Hosts (licensing).
- **S3 storage classes and lifecycle** (the second exercise): Standard, Intelligent-Tiering (unknown access patterns), Standard-IA, One Zone-IA (re-creatable data), Glacier Instant Retrieval (milliseconds, quarterly access), Glacier Flexible Retrieval (minutes to hours), Glacier Deep Archive (12–48 hours, cheapest).
- **Data transfer**: inbound free; out to the internet costs; between AZs costs; a **gateway endpoint** avoids NAT gateway processing for S3/DynamoDB; CloudFront often reduces egress.
- Tools: Cost Explorer, Budgets, Cost and Usage Report, **Compute Optimizer**, Trusted Advisor.
- Right-size (the third exercise sizes an Auto Scaling group), turn off what's idle, prefer serverless for spiky low-volume loads.

## Scenario patterns to recognise

| Scenario | Answer pattern |
|---|---|
| Users upload images; thumbnails must be made | S3 event → (SQS) → Lambda → S3 |
| Static website, global users, HTTPS, custom domain | S3 + CloudFront (OAC) + ACM + Route 53 |
| Session state lost when instances scale in | store sessions in ElastiCache or DynamoDB |
| Database overloaded by reads | read replicas, or ElastiCache in front |
| Share files between many Linux EC2 instances | EFS (Windows: FSx for Windows) |
| On-prem apps need to use S3 as file storage | Storage Gateway (File Gateway) |
| Move 80 TB with a slow internet link | Snowball Edge; ongoing sync: DataSync |
| Private EC2 instances must reach S3 cheaply | S3 gateway VPC endpoint |
| Rotate database credentials automatically | Secrets Manager |
| Audit who changed a security group | CloudTrail (and Config for the history) |

@@quiz
An identity policy allows `s3:*` on a bucket and an SCP for the account denies `s3:DeleteObject`. Can the user delete objects?
- Yes, the identity policy allows it
* No: an explicit deny anywhere (here in the SCP) overrides any allow
- Only if the bucket policy allows it
- Only the root user's SCP applies
? Evaluation: explicit deny → allows (all applicable layers must allow) → otherwise implicit deny.

@@quiz
Log files are read often for 30 days, rarely for a year, then must be kept seven years for audits and are almost never read. The cheapest lifecycle?
- Keep everything in S3 Standard
* Standard, then Standard-IA (or Glacier Instant Retrieval) after 30 days, then Glacier Deep Archive after a year, expire after seven years
- One Zone-IA from day one
- EBS snapshots
? Audit data must survive an AZ loss, so not One Zone-IA.

@@quiz
Which RDS feature gives automatic failover but does NOT serve read traffic (in the classic single-standby deployment)?
* Multi-AZ
- Read replicas
- Aurora Serverless
- RDS Proxy
? Read replicas scale reads; Multi-AZ is for availability.

@@quiz
A nightly batch job can be interrupted and restarted. Cheapest compute?
- On-Demand
- Dedicated Hosts
* Spot Instances
- A 3-year Reserved Instance
? Interruptible plus flexible = Spot.

@@quiz
"Least operational overhead" for an API with spiky traffic and a key-value data model?
- EC2 Auto Scaling with MySQL on EC2
* API Gateway + Lambda + DynamoDB
- ECS on EC2 with Redis
- Lightsail
? Serverless and fully managed services remove servers to patch and scale.

@@flashcards
SAA-C03 :: 65 questions, 130 min, pass 720/1000
explicit deny :: always wins in IAM policy evaluation
SCP :: guardrail on what an account can do, grants nothing itself
Cognito user pool / identity pool :: app sign-in / temporary AWS credentials
gateway endpoint :: free private path to S3 and DynamoDB
SSE-KMS :: S3 encryption with KMS keys and audit
Secrets Manager vs Parameter Store :: rotation built in vs cheaper config
SQS FIFO :: ordered, exactly-once processing
SNS + SQS fan-out :: one message to many queues
RDS Multi-AZ vs read replica :: failover vs read scaling
Aurora :: 6 copies across 3 AZs, up to 15 replicas
DAX :: microsecond cache for DynamoDB
gp3 vs io2 :: general SSD vs highest IOPS
EFS / FSx for Windows / FSx for Lustre :: Linux NFS / SMB / HPC file systems
Global Accelerator :: static anycast IPs for TCP/UDP apps
Compute Savings Plan :: flexible commitment across families, regions, Lambda, Fargate
Glacier Instant / Flexible / Deep Archive :: ms / minutes–hours / 12–48 h retrieval
Snowball / DataSync / Storage Gateway :: offline bulk move / online sync / hybrid storage
placement groups :: cluster (latency), spread (isolation), partition (big data)

@@exercise IAM policy evaluation
Each policy is a list of statements `{"Effect": "Allow"|"Deny", "Action": [...], "Resource": [...]}`. Actions and resources may use `*` wildcards anywhere (`s3:Get*`, `arn:aws:s3:::logs/*`); use `fnmatch.fnmatchcase`. Write `decide(action, resource, identity, scps=None, boundary=None)` returning `"ExplicitDeny"`, `"Allow"` or `"ImplicitDeny"`: an explicit Deny in any of the given policies wins; otherwise the identity policy must allow it **and**, when given, the SCP and the permission boundary must each allow it too.
@@starter
from fnmatch import fnmatchcase


def matches(statement, action, resource):
    pass


def decide(action, resource, identity, scps=None, boundary=None):
    pass


identity = [{"Effect": "Allow", "Action": ["s3:*"], "Resource": ["arn:aws:s3:::logs", "arn:aws:s3:::logs/*"]}]
scp = [{"Effect": "Allow", "Action": ["*"], "Resource": ["*"]},
       {"Effect": "Deny", "Action": ["s3:DeleteObject"], "Resource": ["*"]}]
print(decide("s3:GetObject", "arn:aws:s3:::logs/2026/11/01.gz", identity, scp))
print(decide("s3:DeleteObject", "arn:aws:s3:::logs/old.gz", identity, scp))
print(decide("ec2:StartInstances", "*", identity, scp))
@@tests
identity = [{"Effect": "Allow", "Action": ["s3:*"], "Resource": ["arn:aws:s3:::logs", "arn:aws:s3:::logs/*"]}]
scp = [{"Effect": "Allow", "Action": ["*"], "Resource": ["*"]},
       {"Effect": "Deny", "Action": ["s3:DeleteObject"], "Resource": ["*"]}]
check(decide("s3:GetObject", "arn:aws:s3:::logs/2026/11/01.gz", identity, scp), "Allow", "allowed by both")
check(decide("s3:DeleteObject", "arn:aws:s3:::logs/old.gz", identity, scp), "ExplicitDeny", "the SCP denies")
check(decide("ec2:StartInstances", "*", identity, scp), "ImplicitDeny", "nothing allows it")
check(decide("s3:GetObject", "arn:aws:s3:::other/x", identity), "ImplicitDeny", "wrong bucket")
boundary = [{"Effect": "Allow", "Action": ["s3:Get*", "s3:List*"], "Resource": ["*"]}]
check(decide("s3:PutObject", "arn:aws:s3:::logs/a", identity, None, boundary), "ImplicitDeny", "outside the boundary")
check(decide("s3:ListBucket", "arn:aws:s3:::logs", identity, None, boundary), "Allow", "inside the boundary")
deny_id = identity + [{"Effect": "Deny", "Action": ["s3:*"], "Resource": ["arn:aws:s3:::logs/secret/*"]}]
check(decide("s3:GetObject", "arn:aws:s3:::logs/secret/key", deny_id), "ExplicitDeny", "deny in the identity policy")
@@hint
`matches` is true when any pattern in `Action` matches the action and any pattern in `Resource` matches the resource. Gather the policies that were given (`identity`, plus `scps` and `boundary` when not `None`). First look for a matching Deny in any of them; then require a matching Allow in each one.
@@solution
from fnmatch import fnmatchcase


def matches(statement, action, resource):
    return (any(fnmatchcase(action, a) for a in statement["Action"])
            and any(fnmatchcase(resource, r) for r in statement["Resource"]))


def decide(action, resource, identity, scps=None, boundary=None):
    policies = [p for p in (identity, scps, boundary) if p is not None]
    for policy in policies:
        for st in policy:
            if st["Effect"] == "Deny" and matches(st, action, resource):
                return "ExplicitDeny"
    for policy in policies:
        if not any(st["Effect"] == "Allow" and matches(st, action, resource) for st in policy):
            return "ImplicitDeny"
    return "Allow"


identity = [{"Effect": "Allow", "Action": ["s3:*"], "Resource": ["arn:aws:s3:::logs", "arn:aws:s3:::logs/*"]}]
scp = [{"Effect": "Allow", "Action": ["*"], "Resource": ["*"]},
       {"Effect": "Deny", "Action": ["s3:DeleteObject"], "Resource": ["*"]}]
print(decide("s3:GetObject", "arn:aws:s3:::logs/2026/11/01.gz", identity, scp))
print(decide("s3:DeleteObject", "arn:aws:s3:::logs/old.gz", identity, scp))
print(decide("ec2:StartInstances", "*", identity, scp))

@@exercise Choose an S3 storage class
Write `storage_class(reads_per_year, retrieval, recreatable=False, pattern_known=True)` using these exam rules, in order:
1. `pattern_known` false → `"INTELLIGENT_TIERING"`.
2. `reads_per_year >= 12` (monthly or more) → `"STANDARD"`.
3. `recreatable` true and `reads_per_year >= 4` → `"ONEZONE_IA"`.
4. `reads_per_year >= 4` → `"STANDARD_IA"`.
5. `retrieval` is `"ms"` → `"GLACIER_IR"`; `"hours"` → `"GLACIER"`; `"days"` → `"DEEP_ARCHIVE"`.

Then `monthly_cost(gb, storage_class)` with these per-GB monthly prices: STANDARD 0.025, INTELLIGENT_TIERING 0.025, STANDARD_IA 0.0138, ONEZONE_IA 0.011, GLACIER_IR 0.005, GLACIER 0.0045, DEEP_ARCHIVE 0.002, rounded to 2 decimals.
@@starter
PRICES = {"STANDARD": 0.025, "INTELLIGENT_TIERING": 0.025, "STANDARD_IA": 0.0138, "ONEZONE_IA": 0.011,
          "GLACIER_IR": 0.005, "GLACIER": 0.0045, "DEEP_ARCHIVE": 0.002}


def storage_class(reads_per_year, retrieval, recreatable=False, pattern_known=True):
    pass


def monthly_cost(gb, storage_class):
    pass


for desc, args in [("product images", (365, "ms")), ("quarterly reports", (4, "ms")),
                   ("audit logs", (0, "days")), ("thumbnails", (6, "ms", True))]:
    sc = storage_class(*args)
    print(f"{desc:18} {sc:14} {monthly_cost(5000, sc):>8.2f} per month for 5 TB")
@@tests
check(storage_class(365, "ms"), "STANDARD", "daily")
check(storage_class(12, "ms"), "STANDARD", "monthly")
check(storage_class(4, "ms"), "STANDARD_IA", "quarterly")
check(storage_class(6, "ms", True), "ONEZONE_IA", "re-creatable thumbnails")
check(storage_class(1, "ms"), "GLACIER_IR", "yearly, instant")
check(storage_class(1, "hours"), "GLACIER", "yearly, can wait hours")
check(storage_class(0, "days"), "DEEP_ARCHIVE", "compliance archive")
check(storage_class(100, "ms", pattern_known=False), "INTELLIGENT_TIERING", "unknown pattern")
check(monthly_cost(5000, "STANDARD"), 125.0, "5 TB standard")
check(monthly_cost(5000, "DEEP_ARCHIVE"), 10.0, "5 TB deep archive")
@@hint
Translate the rules into `if` statements in the given order. The cost is `round(gb * PRICES[storage_class], 2)`.
@@solution
PRICES = {"STANDARD": 0.025, "INTELLIGENT_TIERING": 0.025, "STANDARD_IA": 0.0138, "ONEZONE_IA": 0.011,
          "GLACIER_IR": 0.005, "GLACIER": 0.0045, "DEEP_ARCHIVE": 0.002}


def storage_class(reads_per_year, retrieval, recreatable=False, pattern_known=True):
    if not pattern_known:
        return "INTELLIGENT_TIERING"
    if reads_per_year >= 12:
        return "STANDARD"
    if recreatable and reads_per_year >= 4:
        return "ONEZONE_IA"
    if reads_per_year >= 4:
        return "STANDARD_IA"
    return {"ms": "GLACIER_IR", "hours": "GLACIER", "days": "DEEP_ARCHIVE"}[retrieval]


def monthly_cost(gb, storage_class):
    return round(gb * PRICES[storage_class], 2)


for desc, args in [("product images", (365, "ms")), ("quarterly reports", (4, "ms")),
                   ("audit logs", (0, "days")), ("thumbnails", (6, "ms", True))]:
    sc = storage_class(*args)
    print(f"{desc:18} {sc:14} {monthly_cost(5000, sc):>8.2f} per month for 5 TB")

@@exercise Size an Auto Scaling group across AZs
Each instance handles `per_instance` requests per second at the target utilisation. Write `needed(peak_rps, per_instance)` (round up). Then `asg(peak_rps, per_instance, azs, survive_az_loss=True)` returning `(min_size, per_az)`: when the group must survive losing one AZ, the remaining `azs - 1` zones must still carry the peak, so each AZ needs `ceil(needed / (azs - 1))` instances; otherwise `ceil(needed / azs)`. `min_size` is `per_az * azs`.
@@starter
import math


def needed(peak_rps, per_instance):
    pass


def asg(peak_rps, per_instance, azs, survive_az_loss=True):
    pass


print(asg(4500, 500, 3))
print(asg(4500, 500, 3, survive_az_loss=False))
@@tests
check(needed(4500, 500), 9, "exact")
check(needed(4501, 500), 10, "round up")
check(asg(4500, 500, 3), (15, 5), "two AZs must carry 9")
check(asg(4500, 500, 3, survive_az_loss=False), (9, 3), "no AZ loss")
check(asg(1000, 500, 2), (4, 2), "two AZs: each must carry all")
check(asg(100, 500, 3), (3, 1), "at least one per AZ")
@@hint
`math.ceil(peak_rps / per_instance)`. Divide by `azs - 1` or `azs`, round up, and multiply back by `azs`.
@@solution
import math


def needed(peak_rps, per_instance):
    return math.ceil(peak_rps / per_instance)


def asg(peak_rps, per_instance, azs, survive_az_loss=True):
    zones = azs - 1 if survive_az_loss else azs
    per_az = math.ceil(needed(peak_rps, per_instance) / zones)
    return per_az * azs, per_az


print(asg(4500, 500, 3))
print(asg(4500, 500, 3, survive_az_loss=False))
