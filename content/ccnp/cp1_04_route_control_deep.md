@@lesson cp1-route-control-deep Route Control in Depth: Prefix Lists, Route Maps, Redistribution and BGP Policy
@@text
Most ENARSI troubleshooting tickets come down to route control: a prefix list that doesn't match what you think, a route map with an implicit deny at the end, a redistribution loop, or a BGP attribute set on the wrong side. This lesson explains exactly how matching works, then redistribution rules and the BGP attributes you use for policy.

## Prefix lists

```text,norun
ip prefix-list LAN seq 5 permit 10.0.0.0/8 le 24
ip prefix-list LAN seq 10 deny 0.0.0.0/0
ip prefix-list LAN seq 15 permit 0.0.0.0/0 le 32
```

A route matches an entry when:
- its first **N bits** equal the entry's network (N = the entry's prefix length), **and**
- its **prefix length** is in range: without `ge`/`le`, **exactly N**; with `le L`, from N to L; with `ge G`, from G to 32; with both, from G to L (N < G ≤ L).

Entries are checked in sequence order; the first match decides (permit/deny); **no match = implicit deny**. `0.0.0.0/0` alone matches only the default route; `0.0.0.0/0 le 32` matches everything.

## Route maps

```text,norun
route-map FROM-ISP deny 10
 match ip address prefix-list BOGONS
route-map FROM-ISP permit 20
 match ip address prefix-list CUSTOMER
 set local-preference 200
route-map FROM-ISP permit 30
```

- Sequences are evaluated in order. Within one sequence, different match **types** are ANDed; multiple values in one match statement are ORed.
- `permit` + match → the route is accepted (and `set` applied); `deny` + match → the route is dropped. No match → next sequence.
- A sequence with **no match clause matches everything**. If nothing matches → **implicit deny**.
- A `deny` in the referenced ACL/prefix list just means "doesn't match this sequence".

## Redistribution

- Seed metrics: into OSPF, default cost 20 (type E2, BGP-learned routes 1); into EIGRP and RIP, **you must set a metric** (`default-metric` or `metric`), otherwise routes aren't redistributed (except from connected/static).
- OSPF E1 adds internal cost; E2 (default) doesn't; E1 is preferred over E2 for the same prefix.
- **Mutual redistribution** at two points can cause loops and suboptimal routing: **tag** routes on redistribution and deny tagged routes on the way back, or raise the AD of external routes.
- Administrative distance: connected 0, static 1, eBGP 20, EIGRP internal 90, OSPF 110, IS-IS 115, RIP 120, EIGRP external 170, iBGP 200.

## BGP policy attributes

| Attribute | Influences | Direction | Preference |
|---|---|---|---|
| **Weight** (Cisco) | outbound traffic, one router | local only | highest |
| **Local preference** | outbound traffic, whole AS | inbound updates, shared in iBGP | highest (default 100) |
| **AS-path prepending** | inbound traffic from others | outbound updates | shortest path preferred |
| **MED** | inbound traffic from a neighbouring AS | outbound updates | lowest |
| **Communities** | tag routes for policy (no-export, no-advertise, provider-defined) | both | — |

Rule of thumb: to control how **you leave** the AS, set **local preference** inbound; to influence how **others enter**, prepend or set MED outbound. Remember `send-community` to neighbours, and soft reset (`clear ip bgp * soft`) after changing policy.

Best-path order (simplified): weight → local pref → locally originated → AS-path length → origin (IGP < EGP < incomplete) → MED → eBGP over iBGP → lowest IGP metric to next hop → oldest eBGP → lowest router ID.

@@quiz
Which route does `ip prefix-list X permit 10.0.0.0/8 le 24` NOT match?
- 10.1.0.0/16
- 10.0.0.0/8
* 10.1.1.0/25
- 10.20.30.0/24
? /25 is longer than le 24.

@@quiz
Which entry matches every route?
- permit 0.0.0.0/0
* permit 0.0.0.0/0 le 32
- permit 0.0.0.0/32
- permit 0.0.0.0/0 ge 0
? Without le, only the default route matches.

@@quiz
A route map has `permit 10` with a match that doesn't match a route, and no other sequences. What happens to the route?
- It's permitted
* It's denied by the implicit deny
- It's permitted without set actions
- An error is logged
? Nothing matched.

@@quiz
Routes redistributed into EIGRP don't appear. What's most likely missing?
* A seed metric
- A route tag
- An AS number
- A prefix list
? EIGRP needs bandwidth/delay/etc. for external routes.

@@quiz
You want your whole AS to prefer ISP A for outbound traffic. Which attribute?
- MED
- Weight
* Local preference set on routes received from ISP A
- AS-path prepending toward ISP A
? Local preference is shared via iBGP.

@@quiz
Which administrative distance does an external EIGRP route have?
- 90
- 110
* 170
- 200
? Internal EIGRP is 90.

@@flashcards
prefix-list no ge/le :: exact length
le L :: lengths N..L
ge G :: lengths G..32
0.0.0.0/0 le 32 :: match all
implicit deny :: end of prefix list and route map
route map no match clause :: matches all
match types in a sequence :: ANDed
values in one match :: ORed
deny sequence :: drop matching routes
OSPF seed metric :: 20, E2
EIGRP redistribution :: needs a metric
E1 vs E2 :: E1 adds internal cost
mutual redistribution :: tag and filter
AD eBGP :: 20
AD iBGP :: 200
AD OSPF :: 110
AD EIGRP external :: 170
weight :: local router, highest wins
local preference :: AS-wide, highest wins
MED :: lowest wins
AS-path prepend :: influence inbound traffic
no-export :: don't advertise outside AS
send-community :: needed to pass communities

@@drill
What is the default BGP local preference?
= 100
? Higher is preferred.

@@drill
What is the administrative distance of iBGP?
= 200
? eBGP is 20.

@@exercise Evaluate a prefix list
Implement Cisco prefix-list matching with `ge` and `le`, evaluate entries in sequence order with an implicit deny, and print the result for each route with the entry that decided it.
@@lang python
@@starter
import ipaddress

prefix_list = [
    (5, "deny", "10.66.0.0/16", None, 32),
    (10, "permit", "10.0.0.0/8", None, 24),
    (15, "permit", "172.16.0.0/12", 20, 24),
    (20, "permit", "0.0.0.0/0", None, None),
    (25, "deny", "192.168.0.0/16", 25, None),
]
routes = ["10.1.0.0/16", "10.66.5.0/24", "10.1.1.0/25", "172.20.0.0/16", "172.20.16.0/20", "172.20.16.128/25", "0.0.0.0/0", "192.168.1.0/24", "192.168.1.128/26", "8.8.8.0/24"]

def entry_matches(route, network, ge, le):
    raise NotImplementedError
@@expected
10.1.0.0/16        PERMIT (seq 10)
10.66.5.0/24       DENY   (seq 5)
10.1.1.0/25        DENY   (implicit deny)
172.20.0.0/16      DENY   (implicit deny)
172.20.16.0/20     PERMIT (seq 15)
172.20.16.128/25   DENY   (implicit deny)
0.0.0.0/0          PERMIT (seq 20)
192.168.1.0/24     DENY   (implicit deny)
192.168.1.128/26   DENY   (seq 25)
8.8.8.0/24         DENY   (implicit deny)
@@requires
def entry_matches
subnet_of(
implicit deny
@@hint
`route.subnet_of(net)` checks the first N bits. Length range: `low = ge or net.prefixlen`, `high = le or (32 if ge else net.prefixlen)`.
@@solution
import ipaddress

prefix_list = [
    (5, "deny", "10.66.0.0/16", None, 32),
    (10, "permit", "10.0.0.0/8", None, 24),
    (15, "permit", "172.16.0.0/12", 20, 24),
    (20, "permit", "0.0.0.0/0", None, None),
    (25, "deny", "192.168.0.0/16", 25, None),
]
routes = ["10.1.0.0/16", "10.66.5.0/24", "10.1.1.0/25", "172.20.0.0/16", "172.20.16.0/20", "172.20.16.128/25", "0.0.0.0/0", "192.168.1.0/24", "192.168.1.128/26", "8.8.8.0/24"]

def entry_matches(route, network, ge, le):
    net = ipaddress.ip_network(network)
    if not route.subnet_of(net):
        return False
    low = ge or net.prefixlen
    high = le or (32 if ge else net.prefixlen)
    return low <= route.prefixlen <= high

for r in routes:
    route = ipaddress.ip_network(r)
    for seq, action, network, ge, le in sorted(prefix_list):
        if entry_matches(route, network, ge, le):
            print(f"{r:<18} {action.upper():<6} (seq {seq})")
            break
    else:
        print(f"{r:<18} DENY   (implicit deny)")

@@exercise Apply a route map to BGP updates
Implement route-map evaluation for routes received from an ISP: sequences in order; a sequence matches when all its match conditions match (prefix in a list, AS path containing an ASN, community present), a sequence without conditions matches everything; permit applies the `set` actions (local preference, community, prepend), deny drops; no match drops. Then pick the best path per prefix by local preference, then shortest AS path.
@@lang python
@@starter
import ipaddress

BOGONS = ["10.0.0.0/8", "172.16.0.0/12", "192.168.0.0/16", "100.64.0.0/10"]
CUSTOMER = ["203.0.113.0/24", "198.51.100.0/24"]
route_map = [
    (10, "deny", {"prefix_in": BOGONS}, {}),
    (20, "deny", {"as_path_has": 64666}, {}),
    (30, "permit", {"prefix_in": CUSTOMER, "community": "65000:100"}, {"local_pref": 200}),
    (40, "permit", {}, {"local_pref": 120}),
]
updates = [
    ("ISP-A", "203.0.113.0/24", [65010, 65020], {"65000:100"}),
    ("ISP-B", "203.0.113.0/24", [65030], set()),
    ("ISP-A", "10.10.0.0/16", [65010], set()),
    ("ISP-B", "192.0.2.0/24", [65030, 64666], set()),
    ("ISP-A", "198.51.100.0/24", [65010], set()),
    ("ISP-B", "198.51.100.0/24", [65030, 65040, 65050], {"65000:100"}),
]

def matches(conditions, prefix, as_path, communities):
    raise NotImplementedError
@@expected
ISP-A 203.0.113.0/24: permitted by seq 30, local-pref 200
ISP-B 203.0.113.0/24: permitted by seq 40, local-pref 120
ISP-A 10.10.0.0/16: denied by seq 10
ISP-B 192.0.2.0/24: denied by seq 20
ISP-A 198.51.100.0/24: permitted by seq 40, local-pref 120
ISP-B 198.51.100.0/24: permitted by seq 30, local-pref 200
best 203.0.113.0/24: via ISP-A (local-pref 200, AS path 65010 65020)
best 198.51.100.0/24: via ISP-B (local-pref 200, AS path 65030 65040 65050)
@@requires
def matches
"local_pref"
route_map
len(
@@hint
`prefix_in` matches when the prefix is a subnet of any listed network. Default local preference is 100 if no set action changes it. For best paths group accepted routes by prefix and take `max` by `(local_pref, -len(as_path))`.
@@solution
import ipaddress

BOGONS = ["10.0.0.0/8", "172.16.0.0/12", "192.168.0.0/16", "100.64.0.0/10"]
CUSTOMER = ["203.0.113.0/24", "198.51.100.0/24"]
route_map = [
    (10, "deny", {"prefix_in": BOGONS}, {}),
    (20, "deny", {"as_path_has": 64666}, {}),
    (30, "permit", {"prefix_in": CUSTOMER, "community": "65000:100"}, {"local_pref": 200}),
    (40, "permit", {}, {"local_pref": 120}),
]
updates = [
    ("ISP-A", "203.0.113.0/24", [65010, 65020], {"65000:100"}),
    ("ISP-B", "203.0.113.0/24", [65030], set()),
    ("ISP-A", "10.10.0.0/16", [65010], set()),
    ("ISP-B", "192.0.2.0/24", [65030, 64666], set()),
    ("ISP-A", "198.51.100.0/24", [65010], set()),
    ("ISP-B", "198.51.100.0/24", [65030, 65040, 65050], {"65000:100"}),
]

def matches(conditions, prefix, as_path, communities):
    net = ipaddress.ip_network(prefix)
    if "prefix_in" in conditions and not any(net.subnet_of(ipaddress.ip_network(p)) for p in conditions["prefix_in"]):
        return False
    if "as_path_has" in conditions and conditions["as_path_has"] not in as_path:
        return False
    if "community" in conditions and conditions["community"] not in communities:
        return False
    return True

accepted = {}
for peer, prefix, as_path, communities in updates:
    for seq, action, conditions, sets in route_map:
        if matches(conditions, prefix, as_path, communities):
            if action == "deny":
                print(f"{peer} {prefix}: denied by seq {seq}")
            else:
                lp = sets.get("local_pref", 100)
                print(f"{peer} {prefix}: permitted by seq {seq}, local-pref {lp}")
                accepted.setdefault(prefix, []).append((lp, -len(as_path), peer, as_path))
            break
    else:
        print(f"{peer} {prefix}: implicit deny")

for prefix, candidates in accepted.items():
    lp, neg_len, peer, as_path = max(candidates)
    print(f"best {prefix}: via {peer} (local-pref {lp}, AS path {' '.join(map(str, as_path))})")
