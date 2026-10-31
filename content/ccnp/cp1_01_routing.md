@@lesson cp1-routing Advanced Routing: EIGRP, OSPF and BGP
@@text
## Administrative distance

When two routing sources offer the same prefix, the router trusts the lower **administrative distance**:

| Source | AD |
|---|---|
| connected | 0 |
| static | 1 |
| eBGP | 20 |
| EIGRP (internal) | 90 |
| OSPF | 110 |
| IS-IS | 115 |
| RIP | 120 |
| EIGRP external | 170 |
| iBGP | 200 |

The **longest prefix match** always comes first: a /28 beats a /24 whatever the AD. AD only decides between identical prefixes. A **floating static route** has a higher AD on purpose (`ip route 0.0.0.0 0.0.0.0 10.9.9.1 250`) so it's only used when the dynamic route disappears.

## EIGRP

Cisco's advanced distance-vector protocol (now an open RFC 7868).

- Metric from **bandwidth** and **delay** by default (K1 and K3). Wide metrics (64-bit) in named mode.
- **DUAL**: the best path is the **successor**. A **feasible successor** is a backup whose *reported distance* is lower than the current *feasible distance* (the **feasibility condition**), which guarantees it's loop-free, so it's used instantly.
- Unequal-cost load balancing with `variance`.
- **Stub** routers (`eigrp stub`) don't get queries, which keeps big hub-and-spoke networks stable.

## OSPF in depth

- **Areas**: everything connects to **area 0** (the backbone). **ABRs** sit between areas; **ASBRs** bring in external routes.
- **LSA types**: 1 router, 2 network (from the DR), 3 summary (from ABRs, between areas), 4 ASBR summary, 5 external, 7 NSSA external.
- **Area types**: **stub** (no type 5; a default route instead), **totally stubby** (no type 3 or 5), **NSSA** (externals allowed as type 7, converted to type 5 at the ABR).
- **Network types**: broadcast (DR/BDR election, hello 10 s, dead 40 s), point-to-point (no DR), non-broadcast (neighbors configured by hand, hello 30 s).
- Neighbors only form if **area, hello/dead timers, subnet, authentication, MTU** and the stub flags match. `show ip ospf neighbor` stuck in **EXSTART/EXCHANGE** usually means an MTU mismatch.
- Path preference: intra-area (O) > inter-area (O IA) > external type 1 (E1, adds internal cost) > external type 2 (E2, the default: external cost only).
- **Summarization** only on ABRs (`area 1 range`) and ASBRs (`summary-address`).

## BGP: the internet's routing protocol

BGP is a **path-vector** protocol. Neighbors (**peers**) are configured by hand and talk over **TCP port 179**.

- **eBGP**: between different autonomous systems (AS numbers). TTL 1 by default: peers are usually directly connected.
- **iBGP**: inside one AS. Routes learned from one iBGP peer are **not passed to another iBGP peer** (split horizon), so iBGP needs a full mesh, **route reflectors**, or confederations.
- Neighbor states: Idle → Connect → Active → OpenSent → OpenConfirm → **Established**. Stuck in **Active** means the TCP session isn't coming up (reachability, wrong address, ACL).

```text
router bgp 65010
 neighbor 203.0.113.1 remote-as 64500          ! eBGP to the ISP
 neighbor 10.0.0.2 remote-as 65010             ! iBGP inside our AS
 neighbor 10.0.0.2 update-source Loopback0
 address-family ipv4
  network 198.51.100.0 mask 255.255.255.0      ! advertise our prefix (must be in the routing table)
  neighbor 203.0.113.1 activate
  neighbor 203.0.113.1 route-map ISP-IN in
```

## BGP best-path selection

When BGP has several paths to the same prefix, it compares them **in this order** and stops at the first difference. Memorize it; it's on every BGP exam (and you'll implement it in this track's build):

| # | Attribute | Prefer |
|---|---|---|
| 1 | **Weight** (Cisco only, local to the router) | highest |
| 2 | **Local preference** (shared across the AS) | highest (default 100) |
| 3 | **Locally originated** (network/aggregate/redistribute) | yes |
| 4 | **AS path** length | shortest |
| 5 | **Origin** | IGP (i) < EGP (e) < incomplete (?) |
| 6 | **MED** (multi-exit discriminator) | lowest |
| 7 | **eBGP over iBGP** | eBGP |
| 8 | **IGP metric** to the next hop | lowest |
| 9 | oldest eBGP path, then lowest **router ID**, then lowest neighbor address | |

Mnemonic: "**W**e **L**ove **O**ranges **AS** **O**ranges **M**ean **P**ure **R**efreshment" (Weight, Local pref, Originated, AS path, Origin, MED, Paths eBGP, Router ID).

Using it to steer traffic:

- **Outbound** (which exit *you* use): raise **local preference** on routes from the preferred ISP.
- **Inbound** (which link others use to reach you): **AS-path prepending** on the backup link (`set as-path prepend 65010 65010`) makes that path look longer; **MED** works only towards the same neighboring AS.

## Route maps, prefix lists and redistribution

```text
ip prefix-list CUSTOMER seq 10 permit 198.51.100.0/24
ip prefix-list DEFAULT seq 5 permit 0.0.0.0/0
ip prefix-list SMALL seq 5 permit 10.0.0.0/8 ge 24 le 28   ! any /24 to /28 inside 10/8

route-map ISP-IN permit 10
 match ip address prefix-list DEFAULT
 set local-preference 200
route-map ISP-IN permit 20        ! everything else, unchanged
```

A route map is a list of `permit`/`deny` entries with `match` and `set`; there's an **implicit deny at the end**. Forgetting the final empty `permit` drops every other route: a classic outage.

**Redistribution** between protocols needs a **seed metric** (OSPF uses 20 by default for redistributed routes; EIGRP needs one set) and care with **mutual redistribution** at two points, which can create loops: tag routes and filter them on the way back.

**Policy-based routing** (PBR) forwards by source or application instead of the destination: `ip policy route-map` on the incoming interface.

@@quiz
A router learns 10.1.0.0/16 via OSPF and 10.1.1.0/24 via RIP. Which is used for traffic to 10.1.1.5?
* The RIP /24: the longest prefix match wins before administrative distance is considered
- The OSPF route, because 110 < 120
- Both, load-balanced
- Neither
? AD only decides between routes for the same prefix length.

@@quiz
Which is the first attribute BGP compares on a Cisco router?
* Weight
- Local preference
- AS path length
- MED
? Weight is Cisco-specific and local to the router. Local preference is the first attribute shared across an AS.

@@quiz
You have two ISPs and want *outbound* traffic to prefer ISP A. What's the usual tool?
* Higher local preference on routes learned from ISP A
- AS-path prepending towards ISP A
- A lower MED towards ISP B
- A higher router ID
? Local preference steers outbound traffic for the whole AS. Prepending influences inbound traffic.

@@quiz
Two OSPF routers are stuck in EXSTART/EXCHANGE. What's the most likely cause?
* An MTU mismatch on the link
- Different router IDs
- Different OSPF process IDs
- A missing default route
? Process IDs are local and don't need to match. MTU mismatches stall the database exchange.

@@quiz
A route map is applied to inbound BGP updates with one `permit 10` entry that matches the default route. What happens to all other routes?
* They're dropped by the implicit deny at the end of the route map
- They're accepted unchanged
- They get local preference 100
- The BGP session resets
? Add an empty `permit 20` entry to accept the rest.

@@quiz
In EIGRP, when is a backup path a feasible successor?
* When its reported distance is less than the feasible distance of the current best path
- When its metric is within the variance
- When it has the same next hop
- Always, if it exists
? The feasibility condition guarantees that the backup path is loop-free.

@@flashcards
AD: eBGP / EIGRP / OSPF / iBGP :: 20 / 90 / 110 / 200
floating static :: a static route with a higher AD, used as a backup
EIGRP successor / feasible successor :: best path / loop-free backup
LSA type 3 :: summary, generated by ABRs
LSA type 5 / 7 :: external / NSSA external
totally stubby area :: no type 3 or 5 LSAs, only a default route
OSPF E2 vs E1 :: external cost only / adds the internal cost
BGP port :: TCP 179
iBGP split horizon :: iBGP-learned routes aren't sent to iBGP peers
route reflector :: removes the need for an iBGP full mesh
BGP best path order :: weight, local pref, originated, AS path, origin, MED, eBGP>iBGP, IGP metric, router ID
AS-path prepending :: makes a path look longer: steers inbound traffic
`ge 24 le 28` :: prefix-list match on prefix lengths 24 to 28

@@drill
What is the default BGP local preference?
= 100

@@drill
A BGP neighbor is stuck in which state when the TCP session can't be established? (one word)
= Active
= active
? Idle → Connect → Active → OpenSent → OpenConfirm → Established.
