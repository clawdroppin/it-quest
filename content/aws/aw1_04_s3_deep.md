@@lesson aw1-s3-deep S3 in Depth: Storage Classes, Lifecycle, Security and Performance
@@text
S3 appears in more AWS exam questions than any other service. You need to pick storage classes from access patterns, design lifecycle rules, secure buckets (and know why most data leaks were public buckets), replicate data, and understand S3's performance limits.

## Basics

- **Buckets** have globally unique names and live in one Region; **objects** are up to **5 TB** (multipart upload recommended over 100 MB, required over 5 GB).
- Durability **99.999999999%** (11 nines) for all classes; availability varies.
- Strong **read-after-write consistency** for all operations.
- A "folder" is just a **key prefix**: `logs/2026/11/04/app.log`.

## Storage classes

| Class | Use | Min. storage duration | Retrieval |
|---|---|---|---|
| **Standard** | frequently accessed | none | immediate |
| **Intelligent-Tiering** | unknown/changing patterns; moves objects between tiers automatically for a small monitoring fee | none | immediate (archive tiers optional) |
| **Standard-IA** | infrequent but needs immediate access | 30 days | immediate, per-GB retrieval fee |
| **One Zone-IA** | re-creatable infrequent data, one AZ | 30 days | immediate, retrieval fee |
| **Glacier Instant Retrieval** | archive accessed about quarterly | 90 days | milliseconds |
| **Glacier Flexible Retrieval** | archive, retrieval in minutes to hours | 90 days | expedited 1–5 min, standard 3–5 h, bulk 5–12 h |
| **Glacier Deep Archive** | long-term compliance archives | 180 days | standard within 12 h, bulk within 48 h |

IA classes also bill a **minimum object size of 128 KB**: tiny objects are cheaper in Standard.

## Lifecycle rules

Transition and expiration actions by prefix or tag, for example: Standard → **Standard-IA after 30 days** (the minimum before moving to IA) → **Glacier Flexible Retrieval after 90** → **Deep Archive after 365** → **expire after 7 years**. Also: expire **noncurrent versions** after N days and **abort incomplete multipart uploads** after 7 days (hidden storage costs). **S3 Storage Lens** and **Storage Class Analysis** help design rules.

## Versioning, replication and protection

- **Versioning** keeps every version; a delete adds a **delete marker**. Once enabled it can only be **suspended**.
- **MFA Delete** requires MFA to delete versions or change versioning.
- **Object Lock** (WORM): **governance** mode (privileged users can override) or **compliance** mode (nobody, not even root, can delete before the retention date); **legal hold** without a date.
- **Replication** (requires versioning on both sides): **CRR** (cross-Region: compliance, latency, DR) and **SRR** (same Region: log aggregation, prod → test). Only **new** objects replicate; use **Batch Replication** for existing ones. **Replication Time Control** gives a 15-minute SLA.

## Security

- **Block Public Access** (on by default for new buckets and recommended at account level) overrides any policy or ACL that would make data public.
- **Object Ownership: bucket owner enforced** (default) disables ACLs; use policies instead.
- Access = **IAM policies** (who in my account) + **bucket policies** (resource-based: cross-account access, require TLS, restrict to a VPC endpoint) evaluated together, explicit deny wins.
- **Encryption**: all new objects are encrypted by default with **SSE-S3**; **SSE-KMS** adds key policies, audit in CloudTrail, separation of duties (bucket keys reduce KMS cost); **DSSE-KMS** double layer; **SSE-C** customer-provided keys; client-side encryption.
- **Presigned URLs** grant time-limited access to one object without making it public.
- **Access Points** give per-application access policies on shared buckets.

```json,norun
{
  "Version": "2012-10-17",
  "Statement": [{
    "Sid": "DenyInsecureTransport",
    "Effect": "Deny",
    "Principal": "*",
    "Action": "s3:*",
    "Resource": ["arn:aws:s3:::reports-bucket", "arn:aws:s3:::reports-bucket/*"],
    "Condition": {"Bool": {"aws:SecureTransport": "false"}}
  }]
}
```

## Performance

- At least **3,500 PUT/COPY/POST/DELETE** and **5,500 GET/HEAD requests per second per prefix**; spread keys over more prefixes to scale.
- **Multipart upload** for parallel, resumable uploads; **byte-range fetches** for parallel downloads.
- **Transfer Acceleration** routes uploads over CloudFront edge locations; **CloudFront** caches downloads.
- **S3 Select** filters inside objects; **Athena** queries S3 data with SQL.

## Other features

Static website hosting (with CloudFront and OAC for HTTPS), **event notifications** (to Lambda, SQS, SNS or EventBridge), **requester pays**, **S3 Inventory**, **Batch Operations**, and **S3 Express One Zone** for single-digit-millisecond access.

@@quiz
Data is accessed unpredictably: some objects daily, others not for months. Which class minimises effort and cost?
- Standard
* Intelligent-Tiering
- One Zone-IA
- Glacier Deep Archive
? It moves objects between tiers automatically.

@@quiz
Compliance requires that nobody, including the root user, can delete records for 7 years. What should you use?
- Versioning with MFA Delete
- Object Lock in governance mode
* Object Lock in compliance mode
- A bucket policy denying deletes
? Governance mode can be overridden by privileged users.

@@quiz
What must be enabled before configuring S3 Cross-Region Replication?
* Versioning on the source and destination buckets
- Transfer Acceleration
- SSE-KMS
- Static website hosting
? Replication requires versioning.

@@quiz
Which retrieval time fits Glacier Deep Archive standard retrieval?
- Milliseconds
- 1–5 minutes
* Within 12 hours
- Within 7 days
? Bulk is within 48 hours.

@@quiz
How can a user download one private object for 15 minutes without AWS credentials?
- Make the bucket public for 15 minutes
* Generate a presigned URL
- Use an S3 access point
- Enable requester pays
? The URL carries the signer's permissions until it expires.

@@flashcards
S3 durability :: 11 nines
max object size :: 5 TB
multipart upload :: recommended > 100 MB
Standard-IA minimum :: 30 days, 128 KB
Glacier Instant / Flexible minimum :: 90 days
Deep Archive minimum :: 180 days
Intelligent-Tiering :: automatic tiering for unknown patterns
One Zone-IA :: re-creatable data, one AZ
lifecycle :: transitions and expirations
abort incomplete multipart :: hidden cost cleanup
versioning delete :: adds a delete marker
Object Lock compliance :: nobody can delete
replication needs :: versioning
CRR / SRR :: cross-Region / same Region
Block Public Access :: overrides public policies
SSE-S3 :: default encryption
SSE-KMS :: key policies + CloudTrail audit
presigned URL :: temporary object access
prefix request rate :: 3,500 writes, 5,500 reads/s
Transfer Acceleration :: uploads via edge locations

@@drill
What is the minimum storage duration charged for S3 Standard-IA, in days?
= 30
= 30 days
? Also a 128 KB minimum object size.

@@drill
How many GET requests per second per prefix can S3 handle (at least)?
= 5500
= 5,500
? 3,500 for writes.

@@exercise Simulate a lifecycle policy
Apply the lifecycle rules to each object based on its age in days and prefix: a rule applies when the key starts with its prefix; within a rule, use the last transition whose day threshold the age has reached; `expire_days` deletes the object. Print each object's resulting class (or `expired`), then the estimated monthly cost per class using the example per-GB prices (illustrative).
@@lang python
@@starter
PRICE_GB = {"STANDARD": 0.023, "STANDARD_IA": 0.0125, "GLACIER_IR": 0.004, "GLACIER": 0.0036, "DEEP_ARCHIVE": 0.00099}
rules = [
    {"prefix": "logs/", "transitions": [(30, "STANDARD_IA"), (90, "GLACIER"), (365, "DEEP_ARCHIVE")], "expire_days": 2555},
    {"prefix": "reports/", "transitions": [(60, "GLACIER_IR")], "expire_days": None},
]
objects = [
    ("logs/2026/11/app.log", 3, 2.0),
    ("logs/2026/09/app.log", 60, 40.0),
    ("logs/2026/03/app.log", 240, 55.0),
    ("logs/2024/01/app.log", 1000, 70.0),
    ("logs/2018/06/app.log", 3000, 80.0),
    ("reports/q1.pdf", 200, 0.5),
    ("reports/q4.pdf", 20, 0.4),
    ("images/logo.png", 900, 0.1),
]
@@expected
logs/2026/11/app.log      3 days: STANDARD
logs/2026/09/app.log     60 days: STANDARD_IA
logs/2026/03/app.log    240 days: GLACIER
logs/2024/01/app.log   1000 days: DEEP_ARCHIVE
logs/2018/06/app.log   3000 days: expired
reports/q1.pdf          200 days: GLACIER_IR
reports/q4.pdf           20 days: STANDARD
images/logo.png         900 days: STANDARD
STANDARD         2.5 GB  $0.0575/month
STANDARD_IA     40.0 GB  $0.5000/month
GLACIER_IR       0.5 GB  $0.0020/month
GLACIER         55.0 GB  $0.1980/month
DEEP_ARCHIVE    70.0 GB  $0.0693/month
@@requires
startswith(
def apply
@@hint
`apply(key, age)` returns a class name or `None` for expired. Loop over `rule["transitions"]` and keep the class whose threshold `age >= days`.
@@solution
PRICE_GB = {"STANDARD": 0.023, "STANDARD_IA": 0.0125, "GLACIER_IR": 0.004, "GLACIER": 0.0036, "DEEP_ARCHIVE": 0.00099}
rules = [
    {"prefix": "logs/", "transitions": [(30, "STANDARD_IA"), (90, "GLACIER"), (365, "DEEP_ARCHIVE")], "expire_days": 2555},
    {"prefix": "reports/", "transitions": [(60, "GLACIER_IR")], "expire_days": None},
]
objects = [
    ("logs/2026/11/app.log", 3, 2.0),
    ("logs/2026/09/app.log", 60, 40.0),
    ("logs/2026/03/app.log", 240, 55.0),
    ("logs/2024/01/app.log", 1000, 70.0),
    ("logs/2018/06/app.log", 3000, 80.0),
    ("reports/q1.pdf", 200, 0.5),
    ("reports/q4.pdf", 20, 0.4),
    ("images/logo.png", 900, 0.1),
]

def apply(key, age):
    for rule in rules:
        if key.startswith(rule["prefix"]):
            if rule["expire_days"] is not None and age >= rule["expire_days"]:
                return None
            cls = "STANDARD"
            for days, target in rule["transitions"]:
                if age >= days:
                    cls = target
            return cls
    return "STANDARD"

totals = {}
for key, age, gb in objects:
    cls = apply(key, age)
    print(f"{key:<22} {age:>4} days: {cls or 'expired'}")
    if cls:
        totals[cls] = totals.get(cls, 0) + gb

for cls in PRICE_GB:
    if cls in totals:
        print(f"{cls:<13} {totals[cls]:>6.1f} GB  ${totals[cls] * PRICE_GB[cls]:.4f}/month")

@@exercise Audit bucket configurations
Check each bucket's settings and print its findings: Block Public Access off; a policy statement with `"Principal": "*"` and `"Effect": "Allow"` without a condition; no statement denying `aws:SecureTransport` false; versioning off for buckets tagged `critical`; replication configured while versioning is off (invalid); no lifecycle rule aborting incomplete multipart uploads. Parse the policies with `json`. Print `clean` for buckets without findings.
@@lang python
@@starter
import json

buckets = [
    {"name": "web-assets", "bpa": False, "versioning": False, "critical": False, "replication": False, "abort_mpu": True,
     "policy": '{"Statement": [{"Effect": "Allow", "Principal": "*", "Action": "s3:GetObject", "Resource": "arn:aws:s3:::web-assets/*"}]}'},
    {"name": "finance-records", "bpa": True, "versioning": True, "critical": True, "replication": True, "abort_mpu": True,
     "policy": '{"Statement": [{"Effect": "Deny", "Principal": "*", "Action": "s3:*", "Resource": "*", "Condition": {"Bool": {"aws:SecureTransport": "false"}}}]}'},
    {"name": "backups", "bpa": True, "versioning": False, "critical": True, "replication": True, "abort_mpu": False,
     "policy": '{"Statement": []}'},
]
@@expected
web-assets: Block Public Access is off; public Allow of s3:GetObject; no deny for non-TLS requests
finance-records: clean
backups: no deny for non-TLS requests; critical bucket without versioning; replication requires versioning; no rule to abort incomplete multipart uploads
@@requires
json.loads(
def audit
@@hint
A TLS-deny statement has `Effect` Deny and `"aws:SecureTransport"` inside `stmt.get("Condition", {}).get("Bool", {})`.
@@solution
import json

buckets = [
    {"name": "web-assets", "bpa": False, "versioning": False, "critical": False, "replication": False, "abort_mpu": True,
     "policy": '{"Statement": [{"Effect": "Allow", "Principal": "*", "Action": "s3:GetObject", "Resource": "arn:aws:s3:::web-assets/*"}]}'},
    {"name": "finance-records", "bpa": True, "versioning": True, "critical": True, "replication": True, "abort_mpu": True,
     "policy": '{"Statement": [{"Effect": "Deny", "Principal": "*", "Action": "s3:*", "Resource": "*", "Condition": {"Bool": {"aws:SecureTransport": "false"}}}]}'},
    {"name": "backups", "bpa": True, "versioning": False, "critical": True, "replication": True, "abort_mpu": False,
     "policy": '{"Statement": []}'},
]

def audit(b):
    findings = []
    statements = json.loads(b["policy"])["Statement"]
    if not b["bpa"]:
        findings.append("Block Public Access is off")
    for stmt in statements:
        if stmt["Effect"] == "Allow" and stmt.get("Principal") == "*" and "Condition" not in stmt:
            findings.append(f"public Allow of {stmt['Action']}")
    tls = any(s["Effect"] == "Deny" and "aws:SecureTransport" in s.get("Condition", {}).get("Bool", {}) for s in statements)
    if not tls:
        findings.append("no deny for non-TLS requests")
    if b["critical"] and not b["versioning"]:
        findings.append("critical bucket without versioning")
    if b["replication"] and not b["versioning"]:
        findings.append("replication requires versioning")
    if not b["abort_mpu"]:
        findings.append("no rule to abort incomplete multipart uploads")
    return findings

for b in buckets:
    findings = audit(b)
    print(f"{b['name']}: " + ("clean" if not findings else "; ".join(findings)))
