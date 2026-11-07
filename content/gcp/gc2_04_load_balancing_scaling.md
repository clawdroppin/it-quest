@@lesson gc2-lb-scaling Load Balancing and Autoscaling Decisions: Picking the Load Balancer and Sizing the Fleet
@@text
"Which load balancer?" is one of the most common Professional Cloud Architect questions, and the answer follows from four questions asked in order. Autoscaling questions likewise come down to one formula. This lesson turns both into decision procedures.

## The four questions

1. **Traffic type**: HTTP(S) → an **Application Load Balancer** (layer 7, proxy). TCP/UDP or other protocols → a **Network Load Balancer** (layer 4).
2. **Clients**: from the internet → **external**. From inside the VPC or on-premises → **internal**.
3. **Scope**: backends in several regions, or one anycast IP worldwide → **global**. One region, or data residency → **regional**.
4. For layer 4, **proxy or passthrough**: a proxy NLB terminates TCP (with optional SSL offload) and can be global. A **passthrough** NLB is regional, keeps the client source IP, and supports UDP and arbitrary ports.

| Need | Load balancer |
|---|---|
| Global website, Cloud CDN, Cloud Armor, managed certificates | Global external Application LB |
| Internal microservices over HTTP in one region | Regional internal Application LB |
| Game servers on UDP that need the client IP | External passthrough Network LB (regional) |
| TCP service for global users with SSL offload | Global external proxy Network LB |
| Internal TCP database endpoint, next hop for appliances | Internal passthrough Network LB |

Cloud Armor and Cloud CDN attach to **external Application LBs** (CDN to the global one). Serverless backends (Cloud Run, Cloud Functions) plug in through **serverless NEGs**.

## Managed instance group autoscaling

- **Target utilisation**: recommended size = ⌈current load ÷ (target × per-VM capacity)⌉. With CPU at 90% on 4 VMs and a 60% target, you need ⌈4 × 0.9 ÷ 0.6⌉ = **6** VMs.
- Signals: CPU, load-balancing serving capacity (requests per second per VM), Cloud Monitoring metrics, and **schedules** for predictable peaks. **Predictive** autoscaling starts VMs early for slow-booting apps.
- **Cool-down / initialisation period**: ignore new VMs' metrics until they've warmed up. **Scale-in controls** limit how fast the group shrinks.
- Min and max bounds always apply. A regional MIG spreads VMs across zones, for availability.
- Cloud Run scales on **concurrency** (requests per instance, 80 by default), with `min-instances` to avoid cold starts.

@@quiz
An HTTPS website for global users needs a WAF and edge caching. Which load balancer fits?
* Global external Application Load Balancer with Cloud Armor and Cloud CDN
- Regional internal Application Load Balancer
- External passthrough Network Load Balancer
- Internal passthrough Network Load Balancer
? Armor and CDN attach to the external Application LB.

@@quiz
A UDP game server must see the real client IP. Which load balancer fits?
- Global external proxy Network LB
* External passthrough Network LB
- Global external Application LB
- Internal Application LB
? Passthrough keeps the source IP and supports UDP.

@@quiz
4 VMs run at 90% CPU and the autoscaler's target is 60%. What size does it recommend?
- 4
- 5
* 6
- 8
? ⌈4 × 0.9 ÷ 0.6⌉ = 6.

@@quiz
New VMs report very high CPU while they start, which causes extra scale-out. What fixes this?
- Set max replicas to 1
* An initialisation (cool-down) period
- Turn off health checks
- Use a zonal MIG
? Ignore metrics until the VMs are warm.

@@quiz
How do Cloud Run services get behind a global external Application Load Balancer?
- Instance groups
* Serverless network endpoint groups
- Target pools
- VPC peering
? Serverless NEGs.

@@flashcards
Application LB :: layer 7, HTTP(S), proxy
Network LB :: layer 4, TCP/UDP
proxy NLB :: terminates TCP, can be global
passthrough NLB :: regional, keeps client IP, UDP
external / internal :: internet clients / VPC clients
global :: one anycast IP, multi-region backends
Cloud Armor :: WAF on external Application LB
serverless NEG :: Cloud Run behind a LB
autoscaler size :: ⌈load ÷ target⌉
initialisation period :: ignore warming VMs
scale-in controls :: limit shrink rate
predictive autoscaling :: start early for slow boots
regional MIG :: VMs across zones
Cloud Run concurrency :: requests per instance

@@drill
Which kind of Network Load Balancer preserves the client's source IP address?
= passthrough
= a passthrough network load balancer
? Proxy load balancers terminate the connection.

@@drill
10 VMs run at 30% CPU with a target of 60%. To how many VMs does the autoscaler shrink, ignoring the minimum?
= 5
? ⌈10 × 0.3 ÷ 0.6⌉.

@@exercise Choose the load balancer
Encode the four questions (HTTP or not, external or internal, global or regional, proxy or passthrough) as a function, and run it on scenario requirements. Requirements such as UDP, keeping the client IP, Cloud Armor or a single region constrain the answer.
@@concepts conditions, functions, maps, numbers, loops, lists
@@lang python
@@starter
scenarios = {
    "global web shop with WAF": {"protocol": "https", "clients": "internet", "regions": 3, "needs": {"armor", "cdn"}},
    "internal REST APIs": {"protocol": "http", "clients": "vpc", "regions": 1, "needs": set()},
    "multiplayer game": {"protocol": "udp", "clients": "internet", "regions": 1, "needs": {"client_ip"}},
    "global TCP gateway": {"protocol": "tcp", "clients": "internet", "regions": 2, "needs": {"ssl_offload"}},
    "on-prem to database": {"protocol": "tcp", "clients": "vpc", "regions": 1, "needs": {"client_ip"}},
    "EU-only web app": {"protocol": "https", "clients": "internet", "regions": 1, "needs": {"residency"}},
}

def choose_lb(req):
    raise NotImplementedError
@@expected
global web shop with WAF  -> Global external Application LB + Cloud Armor + Cloud CDN
internal REST APIs        -> Regional internal Application LB
multiplayer game          -> Regional external passthrough Network LB
global TCP gateway        -> Global external proxy Network LB (SSL offload)
on-prem to database       -> Regional internal passthrough Network LB
EU-only web app           -> Regional external Application LB
@@requires
def choose_lb
passthrough
Application
@@hint
HTTP or HTTPS → Application LB: external or internal, and global unless it's one region with residency (or internal), then regional. Otherwise → Network LB: passthrough if it's UDP or needs the client IP (always regional), else proxy (global if there are several regions). Add Cloud Armor/CDN notes only for an external Application LB.
@@solution
scenarios = {
    "global web shop with WAF": {"protocol": "https", "clients": "internet", "regions": 3, "needs": {"armor", "cdn"}},
    "internal REST APIs": {"protocol": "http", "clients": "vpc", "regions": 1, "needs": set()},
    "multiplayer game": {"protocol": "udp", "clients": "internet", "regions": 1, "needs": {"client_ip"}},
    "global TCP gateway": {"protocol": "tcp", "clients": "internet", "regions": 2, "needs": {"ssl_offload"}},
    "on-prem to database": {"protocol": "tcp", "clients": "vpc", "regions": 1, "needs": {"client_ip"}},
    "EU-only web app": {"protocol": "https", "clients": "internet", "regions": 1, "needs": {"residency"}},
}

def choose_lb(req):
    side = "external" if req["clients"] == "internet" else "internal"
    if req["protocol"] in ("http", "https"):
        regional = req["regions"] == 1 and ("residency" in req["needs"] or side == "internal")
        name = f"{'Regional' if regional else 'Global'} {side} Application LB"
        extras = sorted(req["needs"] & {"armor", "cdn"})
        if extras and side == "external":
            name += " + " + " + ".join({"armor": "Cloud Armor", "cdn": "Cloud CDN"}[e] for e in extras)
        return name
    if req["protocol"] == "udp" or "client_ip" in req["needs"]:
        return f"Regional {side} passthrough Network LB"
    scope = "Global" if req["regions"] > 1 else "Regional"
    return f"{scope} {side} proxy Network LB" + (" (SSL offload)" if "ssl_offload" in req["needs"] else "")

for name, req in scenarios.items():
    print(f"{name:<25} -> {choose_lb(req)}")

@@exercise Simulate a MIG autoscaler
Simulate a target-utilisation autoscaler over a day of load: compute the recommended size each step with ⌈load ÷ (target × capacity)⌉, clamp it between min and max, scale out immediately, but only scale in after a 3-step stabilisation window, and count the VM-hours used.
@@lang python
@@starter
import math
TARGET, CAPACITY = 0.6, 100       # target CPU fraction; requests per second one VM handles at 100%
MIN, MAX, STABILISE = 2, 12, 3
load = [80, 90, 150, 420, 610, 580, 300, 120, 110, 100, 95, 700, 900, 400, 100, 90]  # req/s per hour

def recommend(rps):
    raise NotImplementedError
@@expected
h00   80 rps rec  2 size  2 util  40% hold
h01   90 rps rec  2 size  2 util  45% hold
h02  150 rps rec  3 size  3 util  50% scale out
h03  420 rps rec  7 size  7 util  60% scale out
h04  610 rps rec 11 size 11 util  55% scale out
h05  580 rps rec 10 size 11 util  53% hold (stabilising)
h06  300 rps rec  5 size 11 util  27% hold (stabilising)
h07  120 rps rec  2 size 10 util  12% scale in
h08  110 rps rec  2 size  5 util  22% scale in
h09  100 rps rec  2 size  2 util  50% scale in
h10   95 rps rec  2 size  2 util  48% hold
h11  700 rps rec 12 size 12 util  58% scale out
h12  900 rps rec 12 size 12 util  75% hold  <- at MAX, over target
h13  400 rps rec  7 size 12 util  33% hold (stabilising)
h14  100 rps rec  2 size 12 util   8% hold (stabilising)
h15   90 rps rec  2 size  7 util  13% scale in
VM-hours: 121 (a fixed fleet sized for peak would use 192)
@@requires
def recommend
math.ceil
STABILISE
@@hint
`recommend` returns `max(MIN, min(MAX, math.ceil(rps / (TARGET * CAPACITY))))`. Keep a list of recent recommendations: scale out as soon as the recommendation is above the current size, but scale in only to the largest recommendation of the last `STABILISE` steps. Report capped steps where load exceeded what MAX VMs can serve.
@@solution
import math
TARGET, CAPACITY = 0.6, 100       # target CPU fraction; requests per second one VM handles at 100%
MIN, MAX, STABILISE = 2, 12, 3
load = [80, 90, 150, 420, 610, 580, 300, 120, 110, 100, 95, 700, 900, 400, 100, 90]  # req/s per hour

def recommend(rps):
    return max(MIN, min(MAX, math.ceil(rps / (TARGET * CAPACITY))))

size, history, vm_hours = MIN, [], 0
for hour, rps in enumerate(load):
    rec = recommend(rps)
    history = (history + [rec])[-STABILISE:]
    if rec > size:
        size, action = rec, "scale out"
    elif max(history) < size:
        size, action = max(history), "scale in"
    else:
        action = "hold" if rec == size else "hold (stabilising)"
    vm_hours += size
    util = rps / (size * CAPACITY)
    warn = "  <- at MAX, over target" if rec == MAX and util > TARGET else ""
    print(f"h{hour:02} {rps:>4} rps rec {rec:>2} size {size:>2} util {util:>4.0%} {action}{warn}")
print(f"VM-hours: {vm_hours} (a fixed fleet sized for peak would use {MAX * len(load)})")
