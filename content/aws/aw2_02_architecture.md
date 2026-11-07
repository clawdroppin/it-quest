@@lesson aw2-architecture Designing Architectures (Solutions Architect)
@@text
The SAA-C03 exam is scenario questions: *"A company needs X with the LEAST operational overhead / MOST cost-effective / highest availability. What should a solutions architect do?"* The keywords in the question decide the answer.

| Keyword | Usually points to |
|---|---|
| "least operational overhead" | managed and serverless services (Lambda, Fargate, DynamoDB, Aurora Serverless, S3) |
| "highly available" | multiple AZs, Auto Scaling, ELB, RDS Multi-AZ |
| "decouple" | SQS, SNS, EventBridge |
| "cost-effective" + steady load | Savings Plans / Reserved; + interruptible: Spot; + rarely accessed data: S3 IA/Glacier |
| "real-time streaming" | Kinesis |
| "cache" / "reduce database load" | ElastiCache, DynamoDB DAX, CloudFront |
| "hybrid", "on-premises file access" | Storage Gateway, FSx, Direct Connect, DataSync |
| "encrypt", "keys" | KMS (customer managed keys for control and audit) |
| "users sign in to the app" | Cognito |

## Disaster recovery strategies

From cheapest/slowest to most expensive/fastest:

| Strategy | How | RTO/RPO |
|---|---|---|
| **Backup and restore** | back up to another region; rebuild when needed | hours |
| **Pilot light** | core pieces (the database) running in the DR region, the rest off | tens of minutes |
| **Warm standby** | a scaled-down but complete copy running | minutes |
| **Multi-site active/active** | full capacity in two regions, both serving | near zero |

## A classic three-tier design

1. **Route 53** → **CloudFront** (static content from **S3**, with WAF).
2. **ALB** in public subnets in **3 AZs**.
3. **EC2 Auto Scaling group** (or ECS on Fargate) in private subnets; sessions in **ElastiCache** or DynamoDB, so any instance can serve any user (**stateless** servers).
4. **Aurora** or **RDS Multi-AZ** in private database subnets, with read replicas for reports.
5. Secrets in **Secrets Manager**, logs in **CloudWatch**, audit in **CloudTrail**, everything built with **CloudFormation**/CDK/Terraform.

The serverless alternative: **API Gateway** → **Lambda** → **DynamoDB**, with S3 + CloudFront for the front end and Cognito for logins. No servers to patch, and costs close to zero when idle: the design in this track's build.

@@quiz
A company needs a disaster recovery plan with an RTO of minutes, at moderate cost. A smaller copy of the full environment can run all the time. Which strategy?
* Warm standby
- Backup and restore
- Pilot light
- Multi-site active/active
? Pilot light keeps only core components running; warm standby runs everything, scaled down.

@@quiz
A web app stores user sessions in memory on each EC2 instance, so users are logged out when Auto Scaling removes an instance. What's the best fix?
* Store sessions in ElastiCache or DynamoDB so the servers are stateless
- Turn off Auto Scaling
- Use bigger instances
- Use sticky sessions only
? Stateless servers can scale in and out freely.

@@quiz
An API with unpredictable, spiky traffic must be built with the LEAST operational overhead. Which design?
* API Gateway + Lambda + DynamoDB
- EC2 instances behind an ALB with a self-managed MySQL
- A single large EC2 instance
- On-premises servers with a VPN
? Serverless services scale automatically and need no patching.

@@quiz
Read-heavy reports are slowing down the production RDS database. What helps most?
* Add a read replica and send reports to it
- Enable Multi-AZ
- Increase the backup retention
- Move the database to a public subnet
? Multi-AZ standbys don't serve reads; replicas do.

@@quiz
Static images for a global audience load slowly far from the Tokyo region. What should be added?
* CloudFront in front of the S3 bucket
- A bigger S3 bucket
- A NAT gateway
- Direct Connect
? A CDN caches content at edge locations near users.

@@flashcards
backup and restore :: Cheapest DR; RTO hours
pilot light :: Core components always on in DR region
warm standby :: Scaled-down full copy running
multi-site active/active :: Full capacity in two regions
stateless servers :: Sessions stored outside the instance
least operational overhead :: Prefer managed/serverless services
decoupling :: SQS, SNS, EventBridge
read replica vs Multi-AZ :: Scale reads vs high availability
three-tier :: Web (ALB) / app (ASG, private) / database (Multi-AZ, private)
serverless web app :: S3 + CloudFront, API Gateway + Lambda + DynamoDB, Cognito

@@drill
Name the DR strategy where only core components (like the database) run in the recovery region. (two words)
= pilot light
? Like the small flame that can light the whole furnace.

@@exercise Review a three-tier design
Check a design against the lesson's three-tier pattern. Each resource has a `tier` (`web`, `app`, `db`), its `subnet` (`public` or `private`) and the `azs` it runs in. Write `review(resources)` that returns findings:

- Only the load balancer (tier `web`) may be in a public subnet; anything else public is `exposed`.
- Every tier must span at least two AZs, otherwise `single AZ`.
- Databases need `multi_az` set and `encrypted` set.
- Each tier must have at least one resource, otherwise `missing tier`.

Print the findings for each design, or `matches the pattern`.
@@lang python
@@starter
designs = {
    "first draft": [
        {"name": "alb", "tier": "web", "subnet": "public", "azs": {"a", "c"}},
        {"name": "app-asg", "tier": "app", "subnet": "public", "azs": {"a"}},
        {"name": "orders-db", "tier": "db", "subnet": "public", "azs": {"a"}, "multi_az": False, "encrypted": False},
    ],
    "reviewed": [
        {"name": "alb", "tier": "web", "subnet": "public", "azs": {"a", "c"}},
        {"name": "app-asg", "tier": "app", "subnet": "private", "azs": {"a", "c"}},
        {"name": "orders-db", "tier": "db", "subnet": "private", "azs": {"a", "c"}, "multi_az": True, "encrypted": True},
    ],
    "no database": [
        {"name": "alb", "tier": "web", "subnet": "public", "azs": {"a", "c", "d"}},
        {"name": "app-asg", "tier": "app", "subnet": "private", "azs": {"a", "c"}},
    ],
}

def review(resources):
    raise NotImplementedError


for name, resources in designs.items():
    findings = review(resources)
    print(name + ":")
    for f in findings or ["matches the pattern"]:
        print("  " + f)
@@expected
first draft:
  app-asg: exposed in a public subnet
  app-asg: single AZ
  orders-db: exposed in a public subnet
  orders-db: single AZ
  orders-db: enable Multi-AZ
  orders-db: enable encryption at rest
reviewed:
  matches the pattern
no database:
  missing tier: db
@@requires
def review
any(
@@hint
Do the per-design check (is each tier present?) with `any` first, then loop over the resources for the per-resource rules.
@@solution
designs = {
    "first draft": [
        {"name": "alb", "tier": "web", "subnet": "public", "azs": {"a", "c"}},
        {"name": "app-asg", "tier": "app", "subnet": "public", "azs": {"a"}},
        {"name": "orders-db", "tier": "db", "subnet": "public", "azs": {"a"}, "multi_az": False, "encrypted": False},
    ],
    "reviewed": [
        {"name": "alb", "tier": "web", "subnet": "public", "azs": {"a", "c"}},
        {"name": "app-asg", "tier": "app", "subnet": "private", "azs": {"a", "c"}},
        {"name": "orders-db", "tier": "db", "subnet": "private", "azs": {"a", "c"}, "multi_az": True, "encrypted": True},
    ],
    "no database": [
        {"name": "alb", "tier": "web", "subnet": "public", "azs": {"a", "c", "d"}},
        {"name": "app-asg", "tier": "app", "subnet": "private", "azs": {"a", "c"}},
    ],
}

def review(resources):
    findings = []
    for tier in ("web", "app", "db"):
        if not any(r["tier"] == tier for r in resources):
            findings.append(f"missing tier: {tier}")
    for r in resources:
        if r["subnet"] == "public" and r["tier"] != "web":
            findings.append(f"{r['name']}: exposed in a public subnet")
        if len(r["azs"]) < 2:
            findings.append(f"{r['name']}: single AZ")
        if r["tier"] == "db":
            if not r.get("multi_az"):
                findings.append(f"{r['name']}: enable Multi-AZ")
            if not r.get("encrypted"):
                findings.append(f"{r['name']}: enable encryption at rest")
    return findings

for name, resources in designs.items():
    findings = review(resources)
    print(name + ":")
    for f in findings or ["matches the pattern"]:
        print("  " + f)
