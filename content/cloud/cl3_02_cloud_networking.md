@@lesson cl3-networking Cloud Networking and Reliable Architecture
@@concepts numbers, strings, loops, networking
@@lang python
@@text
Most cloud outages and most surprise bills come down to the network design. This lesson covers how to lay out a virtual network, connect it to other networks and the office, spread it across zones and regions, and route users to the healthy copy.

## Designing a VPC

A **VPC** (AWS, GCP) or **VNet** (Azure) is your private network in the cloud. Plan it like an office network:

- Pick a **CIDR block** that won't overlap anything you might ever connect to: other VPCs, the office, partners. `10.0.0.0/16` everywhere is the classic mistake that makes peering impossible later. Keep an IP address plan (IPAM) per region and environment.
- Split it into **subnets per tier and per availability zone**: public (load balancers, NAT gateways), private (applications), isolated (databases, no internet route at all). Three AZs × three tiers = nine subnets.
- AWS reserves **5 addresses** in every subnet (network, router, DNS, future, broadcast); Azure reserves 5 too; GCP 4. A `/24` gives 251 usable on AWS.
- **Route tables** decide where traffic goes: public subnets have `0.0.0.0/0 → internet gateway`; private subnets `0.0.0.0/0 → NAT gateway` (outbound only); isolated subnets have no default route.
- **Security groups** (stateful, per instance, allow rules only) are the main firewall; **network ACLs** (stateless, per subnet, allow and deny) are the coarse second layer. Azure: **NSGs**; GCP: **VPC firewall rules** with network tags or service accounts.
- **VPC endpoints / Private Link / Private Service Connect** reach S3, DynamoDB and other services without crossing the internet or paying for the NAT gateway.
- **Flow logs** record accepted and rejected connections: essential for troubleshooting and security.

The first exercise splits a VPC range into subnets.

## Connecting networks

| Need | AWS | Azure | Google Cloud |
|---|---|---|---|
| Two VPCs privately | VPC peering (not transitive) | VNet peering | VPC Network Peering |
| Many VPCs, hub and spoke | **Transit Gateway** | Virtual WAN / hub VNet | Network Connectivity Center, Shared VPC |
| Office over the internet | Site-to-Site VPN | VPN Gateway | Cloud VPN (HA) |
| Office over a private line | **Direct Connect** | **ExpressRoute** | **Cloud Interconnect** |
| Laptops | Client VPN | Point-to-site VPN | (third-party, or IAP) |

Peering is **not transitive**: if A peers with B and B with C, A still can't reach C. That's why big estates use a hub (Transit Gateway) instead of a mesh. A VPN is quick and cheap but goes over the internet; a private line gives steady bandwidth and latency, and production designs often use two lines, or a line plus a VPN as backup.

## Load balancers

| Type | Layer | Use it for |
|---|---|---|
| **Application LB** (AWS ALB, Azure Application Gateway, GCP HTTP(S) LB) | 7 (HTTP) | path and host routing, TLS termination, WAF, websockets |
| **Network LB** (AWS NLB, Azure Load Balancer, GCP TCP/UDP LB) | 4 (TCP/UDP) | very high throughput, static IPs, non-HTTP protocols |
| **Gateway LB** | 3 | putting firewalls and inspection appliances in the path |
| **Global** (GCP global LB, Azure Front Door, AWS Global Accelerator) | 4/7 | one anycast IP for users worldwide |

Load balancers run **health checks** and only send traffic to healthy targets. Put them in front of an **auto scaling group** (VM scale set, managed instance group) spread over at least two AZs.

## Availability zones and regions

A **region** is a geographic area; an **availability zone** is one or more separate data centres inside it, with independent power, cooling and network, a few kilometres apart and connected with low latency.

- **Multi-AZ** protects against a data centre failure and is the default for production: ALB plus instances in 2–3 AZs, RDS Multi-AZ, zone-redundant storage.
- **Multi-region** protects against a whole-region problem, and brings data closer to users far away; it costs much more and makes data consistency harder. Use it when the RTO/RPO or the users require it.

**Composite availability**: components in **series** multiply (`0.999 × 0.9995 = 0.9985`); redundant copies in **parallel** fail only if all fail (`1 − (1 − 0.99)² = 0.9999`). That's why two AZs at 99% each beat one very good server. The second exercise computes this.

## DNS routing

Route 53, Azure DNS + Traffic Manager, and Cloud DNS routing policies decide which address a name resolves to:

| Policy | Behaviour |
|---|---|
| Simple | one answer |
| **Weighted** | split by percentage: canary releases, migrations (90/10) |
| **Latency** | the region with the lowest latency for this user |
| **Failover** | primary while its health check passes, otherwise secondary |
| **Geolocation / geoproximity** | by the user's country or distance: data residency, localised content |
| Multivalue | several healthy answers, client picks |

Keep **TTLs** short (30–60 s) on records you may need to fail over. The third exercise implements failover and weighted routing.

## CDNs and the edge

A **CDN** (CloudFront, Azure Front Door/CDN, Cloud CDN) caches content in edge locations near users: faster pages, less load and lower egress costs from the origin, plus DDoS protection (AWS Shield, Azure DDoS Protection, Cloud Armor) and a **WAF** in front. Use an **origin access control** so the bucket is only reachable through the CDN, set cache headers deliberately, and invalidate on deploy (or use versioned file names).

## Reliability patterns

- **Stateless** application servers; sessions in Redis/DynamoDB; files in object storage.
- **Queues** (SQS, Service Bus, Pub/Sub) between tiers to absorb spikes and survive a slow consumer.
- **Retries with exponential backoff and jitter**, **timeouts** on every call, **circuit breakers**, **idempotency keys** so a retried request isn't processed twice.
- **Read replicas** and caches for read-heavy loads; **sharding** or DynamoDB/Spanner/Cosmos DB when one database isn't enough.
- **Chaos engineering** (AWS Fault Injection Service, Azure Chaos Studio): break things on purpose, in a controlled way, to prove the design works.

@@quiz
VPC A is peered with VPC B, and B with C. Can A reach C?
- Yes, peering is transitive
* No, peering isn't transitive: peer A and C directly or use a hub such as Transit Gateway
- Only over IPv6
- Only if all three are in one region
? Hub-and-spoke designs exist because of this.

@@quiz
Why does a private subnet route `0.0.0.0/0` to a NAT gateway rather than an internet gateway?
* So instances can make outbound connections (updates, APIs) while nothing on the internet can connect in
- Because NAT gateways are free
- To make the subnet faster
- Because private subnets can't have route tables
? An internet gateway route plus a public IP would make the instance reachable from outside.

@@quiz
Which load balancer fits a game server that speaks UDP and needs static IP addresses?
- Application Load Balancer
* Network Load Balancer
- A CDN
- Weighted DNS
? Layer 7 load balancers only understand HTTP(S).

@@quiz
Two independent instances each have 99% availability behind a load balancer (ignore the load balancer). What's the availability of the pair?
- 98%
- 99%
* 99.99%
- 100%
? Parallel: 1 − (0.01 × 0.01) = 0.9999.

@@quiz
You're shifting 10% of users to a new region to test it. Which Route 53 routing policy?
- Failover
- Latency
* Weighted
- Geolocation
? Weights of 90 and 10 split the answers by percentage.

@@drill
What's the name of a private, isolated network in a cloud provider (AWS and GCP)?
= VPC
= virtual private cloud
? Azure calls it a virtual network (VNet).

@@flashcards
VPC / VNet :: your private network in the cloud
non-overlapping CIDR :: plan ranges so networks can be connected later
public / private / isolated subnet :: route to IGW / route to NAT / no internet route
AWS reserved addresses :: 5 per subnet
security group :: stateful, per instance, allow rules only
network ACL :: stateless, per subnet, allow and deny
VPC endpoint / Private Link :: reach services without the internet or NAT
flow logs :: records of accepted and rejected connections
peering :: private link between two VPCs, not transitive
Transit Gateway :: hub connecting many VPCs and VPNs
Direct Connect / ExpressRoute / Interconnect :: private line from the office or DC
ALB vs NLB :: layer 7 HTTP routing vs layer 4 TCP/UDP speed
multi-AZ :: survive a data-centre failure
series vs parallel availability :: multiply vs 1 − product of failures
weighted / latency / failover routing :: split traffic / nearest region / backup when unhealthy
CDN :: cache content at the edge, near users
exponential backoff with jitter :: retries that don't stampede a recovering service
idempotency key :: a retried request is processed only once

@@exercise Split a VPC into subnets
Write `subnets(cidr, new_prefix)` that returns every subnet of size `new_prefix` inside `cidr` as strings, in order, and `usable(cidr)` that returns how many addresses AWS lets you use in a subnet (the size minus 5). Use the `ipaddress` module. Then write `plan(cidr, tiers, azs)`: give each tier/AZ pair the next `/24` in order (tiers outer, AZs inner) and return a dict like `{"public-a": "10.20.0.0/24", ...}`.
@@starter
import ipaddress


def subnets(cidr, new_prefix):
    pass


def usable(cidr):
    pass


def plan(cidr, tiers, azs):
    pass


for name, net in plan("10.20.0.0/16", ["public", "app", "data"], ["a", "b", "c"]).items():
    print(f"{name:10} {net:16} {usable(net)} usable")
@@tests
check(subnets("10.0.0.0/22", 24), ["10.0.0.0/24", "10.0.1.0/24", "10.0.2.0/24", "10.0.3.0/24"], "a /22 holds four /24s")
check(len(subnets("10.0.0.0/16", 20)), 16, "sixteen /20s in a /16")
check(usable("10.0.0.0/24"), 251, "256 minus 5")
check(usable("10.0.0.0/28"), 11, "the smallest AWS subnet")
p = plan("10.20.0.0/16", ["public", "app", "data"], ["a", "b", "c"])
check(p["public-a"], "10.20.0.0/24", "first")
check(p["app-a"], "10.20.3.0/24", "tiers outer, AZs inner")
check(p["data-c"], "10.20.8.0/24", "ninth")
check(list(p)[:3], ["public-a", "public-b", "public-c"], "order")
@@hint
`ipaddress.ip_network(cidr).subnets(new_prefix=n)` yields the subnets; `str()` each. `ip_network(cidr).num_addresses - 5`. For the plan, take `subnets(cidr, 24)` and an index that goes up by one for each name.
@@solution
import ipaddress


def subnets(cidr, new_prefix):
    return [str(s) for s in ipaddress.ip_network(cidr).subnets(new_prefix=new_prefix)]


def usable(cidr):
    return ipaddress.ip_network(cidr).num_addresses - 5


def plan(cidr, tiers, azs):
    blocks = subnets(cidr, 24)
    out = {}
    i = 0
    for tier in tiers:
        for az in azs:
            out[f"{tier}-{az}"] = blocks[i]
            i += 1
    return out


for name, net in plan("10.20.0.0/16", ["public", "app", "data"], ["a", "b", "c"]).items():
    print(f"{name:10} {net:16} {usable(net)} usable")

@@exercise Composite availability
Write `series(*parts)` (all must work: multiply) and `parallel(*parts)` (works if any works: `1 - product of (1 - a)`). Then `downtime_minutes(availability, days=30)`: the expected downtime in a month, rounded to 1 decimal. Use them to compare a single-AZ design with a multi-AZ one.
@@starter
def series(*parts):
    pass


def parallel(*parts):
    pass


def downtime_minutes(availability, days=30):
    pass


single = series(0.9999, 0.99, 0.995)           # load balancer, one server, one database
multi = series(0.9999, parallel(0.99, 0.99), parallel(0.995, 0.995))
print(round(single, 5), downtime_minutes(single), "min/month")
print(round(multi, 5), downtime_minutes(multi), "min/month")
@@tests
check(round(series(0.99, 0.99), 4), 0.9801, "two in series")
check(round(parallel(0.99, 0.99), 4), 0.9999, "two in parallel")
check(round(parallel(0.9, 0.9, 0.9), 3), 0.999, "three in parallel")
check(downtime_minutes(0.999), 43.2, "three nines")
check(downtime_minutes(0.9999, days=365), 52.6, "four nines a year")
multi = series(0.9999, parallel(0.99, 0.99), parallel(0.995, 0.995))
check(round(multi, 5), 0.99978, "multi-AZ design")
@@hint
Start `result = 1` and multiply. For parallel, multiply the failure chances `(1 - a)` and return `1 - that`. A month has `days * 24 * 60` minutes; downtime is `(1 - availability)` of that.
@@solution
def series(*parts):
    result = 1
    for a in parts:
        result *= a
    return result


def parallel(*parts):
    fail = 1
    for a in parts:
        fail *= 1 - a
    return 1 - fail


def downtime_minutes(availability, days=30):
    return round((1 - availability) * days * 24 * 60, 1)


single = series(0.9999, 0.99, 0.995)           # load balancer, one server, one database
multi = series(0.9999, parallel(0.99, 0.99), parallel(0.995, 0.995))
print(round(single, 5), downtime_minutes(single), "min/month")
print(round(multi, 5), downtime_minutes(multi), "min/month")

@@exercise DNS failover and weighted routing
Records are dicts with `value`, `healthy`, and either `role` (`"primary"`/`"secondary"`) or `weight`. Write `failover(records)`: the primary's value if it's healthy, else the healthy secondary's value, else `None`. Write `weighted(records, n)`: only healthy records count; `n` is a number from 0 up to (not including) the total healthy weight, and the answer is the record whose cumulative weight range contains `n`, in list order (weights 90 and 10: 0–89 → first, 90–99 → second). Return `None` if nothing is healthy.
@@starter
def failover(records):
    pass


def weighted(records, n):
    pass


fo = [{"value": "198.51.100.10", "role": "primary", "healthy": False},
      {"value": "203.0.113.20", "role": "secondary", "healthy": True}]
print(failover(fo))
w = [{"value": "tokyo", "weight": 90, "healthy": True}, {"value": "osaka", "weight": 10, "healthy": True}]
print([weighted(w, n) for n in (0, 89, 90, 99)])
@@tests
fo = [{"value": "198.51.100.10", "role": "primary", "healthy": True},
      {"value": "203.0.113.20", "role": "secondary", "healthy": True}]
check(failover(fo), "198.51.100.10", "primary healthy")
fo[0]["healthy"] = False
check(failover(fo), "203.0.113.20", "fail over")
fo[1]["healthy"] = False
check(failover(fo), None, "both down")
w = [{"value": "tokyo", "weight": 90, "healthy": True}, {"value": "osaka", "weight": 10, "healthy": True}]
check([weighted(w, n) for n in (0, 89, 90, 99)], ["tokyo", "tokyo", "osaka", "osaka"], "90/10 split")
w[0]["healthy"] = False
check(weighted(w, 5), "osaka", "unhealthy records drop out")
check(weighted([{"value": "x", "weight": 5, "healthy": False}], 0), None, "nothing healthy")
@@hint
For failover, look for the record with each role in turn. For weighted, keep a running total of healthy weights and return the first record where `n < total`.
@@solution
def failover(records):
    for role in ("primary", "secondary"):
        for r in records:
            if r["role"] == role and r["healthy"]:
                return r["value"]
    return None


def weighted(records, n):
    total = 0
    for r in records:
        if not r["healthy"]:
            continue
        total += r["weight"]
        if n < total:
            return r["value"]
    return None


fo = [{"value": "198.51.100.10", "role": "primary", "healthy": False},
      {"value": "203.0.113.20", "role": "secondary", "healthy": True}]
print(failover(fo))
w = [{"value": "tokyo", "weight": 90, "healthy": True}, {"value": "osaka", "weight": 10, "healthy": True}]
print([weighted(w, n) for n in (0, 89, 90, 99)])
