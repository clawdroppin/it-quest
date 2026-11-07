@@lesson aw1-network-db Networking and Databases
@@text
## VPC

A **VPC** is your private network in a region (for example `10.0.0.0/16`). **Subnets** live in one AZ each.

- A **public subnet** has a route to an **internet gateway**; instances there can have public IPs.
- A **private subnet** has no direct route in; for outbound internet (updates) it routes to a **NAT gateway** in a public subnet.
- **Route tables** decide where traffic goes.
- **Security groups**: **stateful** firewalls on instances/ENIs, **allow rules only**, return traffic automatically allowed.
- **Network ACLs**: **stateless**, on subnets, allow *and* deny rules, evaluated in number order: return traffic needs its own rule.
- Connectivity: **VPC peering** (two VPCs), **Transit Gateway** (hub for many VPCs and on-premises), **Site-to-Site VPN**, **Direct Connect** (private line), **Client VPN**, **PrivateLink / VPC endpoints** (reach S3 or services without the internet).

A typical web app: ALB in public subnets in two AZs, app servers in private subnets, database in private subnets with no internet route.

## DNS and content delivery

- **Route 53**: DNS and domain registration, with **routing policies**: simple, **weighted** (10% to the new version), **latency-based**, **failover** (with health checks), **geolocation**, geoproximity, multivalue.
- **CloudFront**: CDN caching content at edge locations; works with S3, ALB, any origin; HTTPS, WAF, signed URLs.
- **Global Accelerator**: fast, static anycast IPs into AWS's network.
- **API Gateway**: managed front door for REST/HTTP/WebSocket APIs (often with Lambda).

## Databases

| Need | Service |
|---|---|
| Relational (MySQL, PostgreSQL, MariaDB, Oracle, SQL Server) | **RDS**: managed backups, patching, **Multi-AZ** (synchronous standby for HA), **read replicas** (scale reads) |
| High-end relational, MySQL/PostgreSQL compatible | **Aurora** (also Aurora Serverless) |
| Key-value / document, single-digit ms at any scale | **DynamoDB** (serverless, NoSQL) |
| In-memory cache | **ElastiCache** (Redis/Valkey, Memcached), MemoryDB |
| Data warehouse / analytics | **Redshift**; **Athena** queries files in S3 with SQL |
| Graph | Neptune |
| MongoDB-compatible | DocumentDB |
| Migration | **DMS** (Database Migration Service), with SCT for engine changes |

Multi-AZ is for **availability** (automatic failover); read replicas are for **performance** (and can be promoted for DR).

## Decoupling and integration

**SQS** (queues: producers and consumers work at their own pace), **SNS** (publish/subscribe notifications: email, SMS, fan-out to queues), **EventBridge** (event bus and schedules), **Step Functions** (workflows), **Kinesis** (real-time streams).

@@quiz
A security group allows inbound HTTPS. Do you need a rule for the response traffic going back out?
* No: security groups are stateful
- Yes, an outbound rule for ephemeral ports
- Yes, a network ACL deny rule
- Only for IPv6
? Network ACLs, which are stateless, would need it.

@@quiz
Which component lets instances in a private subnet download updates without being reachable from the internet?
* A NAT gateway in a public subnet
- An internet gateway attached to the private subnet
- A VPC peering connection
- A security group with 0.0.0.0/0 inbound
? NAT allows outbound connections only.

@@quiz
You must block one specific malicious IP address from a whole subnet. Which feature can do it?
* A network ACL deny rule
- A security group deny rule
- Route 53
- An IAM policy
? Security groups only have allow rules.

@@quiz
An RDS database must fail over automatically if its AZ goes down. Which feature?
* Multi-AZ deployment
- Read replicas
- A bigger instance class
- S3 cross-region replication
? Multi-AZ keeps a synchronous standby in another AZ.

@@quiz
An app needs a serverless NoSQL database with millisecond latency at huge scale. Which service?
* DynamoDB
- RDS for MySQL
- Redshift
- Aurora
? DynamoDB is a fully managed key-value and document database.

@@quiz
A web shop wants to send 10% of users to a new version of the site using DNS. Which Route 53 routing policy?
* Weighted
- Failover
- Geolocation
- Simple
? Weights split traffic in proportions.

@@quiz
An order system must not lose orders when the processing servers are overloaded. What should sit between the web tier and the processors?
* An SQS queue
- An internet gateway
- CloudFront
- A NAT gateway
? Queues decouple components and buffer bursts.

@@flashcards
VPC :: Your private network in a region
public subnet :: Has a route to an internet gateway
NAT gateway :: Outbound-only internet for private subnets
security group :: Stateful, allow-only, instance level
network ACL :: Stateless, allow and deny, subnet level, numbered rules
Transit Gateway :: Hub connecting many VPCs and on-premises networks
Direct Connect :: Dedicated private line to AWS
VPC endpoint / PrivateLink :: Reach services privately without the internet
Route 53 :: DNS with routing policies and health checks
CloudFront :: CDN at edge locations
API Gateway :: Managed API front door
RDS Multi-AZ :: High availability with automatic failover
read replica :: Scales reads
Aurora :: AWS's cloud-native MySQL/PostgreSQL-compatible database
DynamoDB :: Serverless NoSQL key-value/document
ElastiCache :: Managed Redis/Memcached
Redshift :: Data warehouse
Athena :: SQL queries on files in S3
SQS / SNS :: Queues / pub-sub notifications
EventBridge :: Event bus and scheduler

@@drill
Is a network ACL stateful or stateless?
= stateless
? Return traffic must be explicitly allowed.

@@exercise Fan out SNS messages to SQS queues
An order service publishes to an **SNS** topic. Several **SQS** queues subscribe, and each subscription can have a filter policy: a dict of attribute → allowed values. A message goes to a queue when every attribute in the filter has one of the allowed values (no filter = every message).

Write `publish(message, subscriptions, queues)` that appends the message to each matching queue and returns how many queues got it. Then let each consumer drain its queue at its own pace (`batch` messages per poll) and print what is left after one poll.
@@lang python
@@starter
subscriptions = {
    "billing-queue": {"type": ["order_placed", "refund"]},
    "shipping-queue": {"type": ["order_placed"], "country": ["JP", "KR"]},
    "audit-queue": {},
    "fraud-queue": {"type": ["order_placed"], "amount_band": ["high"]},
}
consumers = {"billing-queue": 2, "shipping-queue": 1, "audit-queue": 10, "fraud-queue": 1}
messages = [
    {"id": 1, "type": "order_placed", "country": "JP", "amount_band": "low"},
    {"id": 2, "type": "order_placed", "country": "DE", "amount_band": "high"},
    {"id": 3, "type": "refund", "country": "JP", "amount_band": "low"},
    {"id": 4, "type": "order_placed", "country": "KR", "amount_band": "high"},
    {"id": 5, "type": "newsletter_signup", "country": "JP"},
]

def matches(message, policy):
    raise NotImplementedError


def publish(message, subscriptions, queues):
    raise NotImplementedError


queues = {q: [] for q in subscriptions}
for m in messages:
    print(f"message {m['id']} ({m['type']}) -> {publish(m, subscriptions, queues)} queue(s)")
for q, batch in consumers.items():
    got, queues[q] = queues[q][:batch], queues[q][batch:]
    print(f"{q}: processed {got}, waiting {queues[q]}")
@@expected
message 1 (order_placed) -> 3 queue(s)
message 2 (order_placed) -> 3 queue(s)
message 3 (refund) -> 2 queue(s)
message 4 (order_placed) -> 4 queue(s)
message 5 (newsletter_signup) -> 1 queue(s)
billing-queue: processed [1, 2], waiting [3, 4]
shipping-queue: processed [1], waiting [4]
audit-queue: processed [1, 2, 3, 4, 5], waiting []
fraud-queue: processed [2], waiting [4]
@@requires
def publish
all(
@@hint
A filter matches when `all(message.get(attr) in allowed ...)` holds over its items; `all` of an empty filter is `True`, which handles the no-filter case for free.
@@solution
subscriptions = {
    "billing-queue": {"type": ["order_placed", "refund"]},
    "shipping-queue": {"type": ["order_placed"], "country": ["JP", "KR"]},
    "audit-queue": {},
    "fraud-queue": {"type": ["order_placed"], "amount_band": ["high"]},
}
consumers = {"billing-queue": 2, "shipping-queue": 1, "audit-queue": 10, "fraud-queue": 1}
messages = [
    {"id": 1, "type": "order_placed", "country": "JP", "amount_band": "low"},
    {"id": 2, "type": "order_placed", "country": "DE", "amount_band": "high"},
    {"id": 3, "type": "refund", "country": "JP", "amount_band": "low"},
    {"id": 4, "type": "order_placed", "country": "KR", "amount_band": "high"},
    {"id": 5, "type": "newsletter_signup", "country": "JP"},
]

def matches(message, policy):
    return all(message.get(attr) in allowed for attr, allowed in policy.items())

def publish(message, subscriptions, queues):
    sent = 0
    for queue, policy in subscriptions.items():
        if matches(message, policy):
            queues[queue].append(message["id"])
            sent += 1
    return sent

queues = {q: [] for q in subscriptions}
for m in messages:
    print(f"message {m['id']} ({m['type']}) -> {publish(m, subscriptions, queues)} queue(s)")
for q, batch in consumers.items():
    got, queues[q] = queues[q][:batch], queues[q][batch:]
    print(f"{q}: processed {got}, waiting {queues[q]}")

@@exercise Follow a VPC route table
Each subnet has a route table. Write `next_hop(subnet, dest, tables)` that picks the most specific matching route (longest prefix) and returns its target: `local`, an internet gateway (`igw-…`), a NAT gateway (`nat-…`), a peering connection (`pcx-…`) or `blackhole`.

Then write `can_reach_internet(subnet, tables)` that says whether instances there can start connections to the internet (a `0.0.0.0/0` route to an IGW or a NAT gateway) and whether the internet can reach them (IGW only, and only for instances with a public IP). Print the hops for several destinations and the internet summary for each subnet.
@@lang python
@@starter
import ipaddress

tables = {
    "public-a": [("10.0.0.0/16", "local"), ("0.0.0.0/0", "igw-01")],
    "private-a": [("10.0.0.0/16", "local"), ("0.0.0.0/0", "nat-0a"), ("10.1.0.0/16", "pcx-77")],
    "db-a": [("10.0.0.0/16", "local")],
    "legacy": [("10.0.0.0/16", "local"), ("0.0.0.0/0", "nat-old"), ("10.1.5.0/24", "blackhole"), ("10.1.0.0/16", "pcx-77")],
}

def next_hop(subnet, dest, tables):
    raise NotImplementedError


def can_reach_internet(subnet, tables):
    raise NotImplementedError


for subnet, dest in [("public-a", "10.0.3.7"), ("public-a", "8.8.8.8"), ("private-a", "8.8.8.8"),
                     ("private-a", "10.1.2.3"), ("legacy", "10.1.5.9"), ("legacy", "10.1.9.9"), ("db-a", "1.1.1.1")]:
    print(f"{subnet} -> {dest}: {next_hop(subnet, dest, tables)}")
for subnet in tables:
    out, inb = can_reach_internet(subnet, tables)
    print(f"{subnet}: outbound {'yes' if out else 'no'}, inbound {'yes (public IPs only)' if inb else 'no'}")
@@expected
public-a -> 10.0.3.7: local
public-a -> 8.8.8.8: igw-01
private-a -> 8.8.8.8: nat-0a
private-a -> 10.1.2.3: pcx-77
legacy -> 10.1.5.9: blackhole
legacy -> 10.1.9.9: pcx-77
db-a -> 1.1.1.1: no route
public-a: outbound yes, inbound yes (public IPs only)
private-a: outbound yes, inbound no
db-a: outbound no, inbound no
legacy: outbound yes, inbound no
@@requires
def next_hop
def can_reach_internet
prefixlen
@@hint
Collect every route whose network contains the destination and take the one with the biggest `prefixlen`. A NAT gateway lets instances start connections out, but nothing can start a connection in through it.
@@solution
import ipaddress

tables = {
    "public-a": [("10.0.0.0/16", "local"), ("0.0.0.0/0", "igw-01")],
    "private-a": [("10.0.0.0/16", "local"), ("0.0.0.0/0", "nat-0a"), ("10.1.0.0/16", "pcx-77")],
    "db-a": [("10.0.0.0/16", "local")],
    "legacy": [("10.0.0.0/16", "local"), ("0.0.0.0/0", "nat-old"), ("10.1.5.0/24", "blackhole"), ("10.1.0.0/16", "pcx-77")],
}

def next_hop(subnet, dest, tables):
    addr = ipaddress.ip_address(dest)
    matches = [(ipaddress.ip_network(p), t) for p, t in tables[subnet] if addr in ipaddress.ip_network(p)]
    if not matches:
        return "no route"
    return max(matches, key=lambda m: m[0].prefixlen)[1]

def can_reach_internet(subnet, tables):
    default = dict(tables[subnet]).get("0.0.0.0/0", "")
    outbound = default.startswith(("igw-", "nat-"))
    inbound = default.startswith("igw-")
    return outbound, inbound

for subnet, dest in [("public-a", "10.0.3.7"), ("public-a", "8.8.8.8"), ("private-a", "8.8.8.8"),
                     ("private-a", "10.1.2.3"), ("legacy", "10.1.5.9"), ("legacy", "10.1.9.9"), ("db-a", "1.1.1.1")]:
    print(f"{subnet} -> {dest}: {next_hop(subnet, dest, tables)}")
for subnet in tables:
    out, inb = can_reach_internet(subnet, tables)
    print(f"{subnet}: outbound {'yes' if out else 'no'}, inbound {'yes (public IPs only)' if inb else 'no'}")
