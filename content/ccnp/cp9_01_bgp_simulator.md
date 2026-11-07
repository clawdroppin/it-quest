@@lesson cp9-bgp-simulator BGP Decision Simulator
@@text
"Why is traffic to Cloudflare going out of the backup ISP?" To answer that on a real router you read `show ip bgp`, look at the attributes of every path, and walk the best-path algorithm in your head. Here you'll write that algorithm, so you'll never have to guess again:

```text
Network             Next hop         LocPrf Weight  Path                  Chosen because
0.0.0.0/0           203.0.113.1         200      0  64500 i               local preference
104.16.0.0/13       203.0.113.5         100      0  64500 13335 i         MED
198.18.0.0/15       10.0.0.3            100      0  64900 i               origin
1.1.1.0/24          198.51.100.1        100      0  64700 13335 i         AS path length
172.16.50.0/24      10.0.0.3            100      0  i                     IGP metric
198.51.100.0/24     0.0.0.0             100  32768  i                     only path
8.8.8.0/24          203.0.113.1         100      0  64500 15169 i         AS path length
(1 prefix(es) filtered out completely by route maps)
```

The scenario: your company (AS 65010) has two ISPs. `paths.csv` holds every path the router has heard, with its attributes. `config.txt` holds the inbound **route maps**: prefer ISP A for the default route (local preference 200), and never accept private address space from either ISP. Your code applies the route maps, then picks the best path for every prefix and says **which attribute decided**.

You'll write four functions in `bgp.py`:

1. `prefix_matches`: IOS prefix-list matching with `ge` and `le`.
2. `apply_route_map`: first matching entry wins, `set` changes a copy of the path, and the **implicit deny** drops anything that matches nothing.
3. `compare`: the best-path order from the lesson (weight, local preference, locally originated, AS path, origin, MED, eBGP over iBGP, IGP metric, router ID), including the detail that **MED is only compared between paths from the same neighboring AS**.
4. `best_path`: the winner among many.

Start with `python -m unittest`: everything fails. Work through the tests one class at a time (`python -m unittest test_bgp.PrefixLists`), then run `python main.py`.
@@project bgp_simulator BGP Decision Simulator
Write `prefix_matches`, `apply_route_map`, `compare` and `best_path` in `bgp.py` until `python -m unittest` passes, then run `python main.py` and explain every line of the table.
@@file bgp.py
"""A small BGP decision simulator: inbound route maps, then best-path selection per prefix."""
import csv
import io
import ipaddress
from dataclasses import dataclass, field, replace

ORIGIN_RANK = {"i": 0, "e": 1, "?": 2}


@dataclass
class Path:
    prefix: str
    neighbor: str
    weight: int = 0
    local_pref: int = 100
    as_path: list[int] = field(default_factory=list)
    origin: str = "i"
    med: int = 0
    ebgp: bool = True
    igp_metric: int = 0
    router_id: str = "0.0.0.0"
    local: bool = False


@dataclass
class Entry:
    """One route-map entry: `permit`/`deny`, a sequence number, matches and sets."""
    action: str
    seq: int
    match_prefixes: list[str] = field(default_factory=list)
    match_as: list[int] = field(default_factory=list)
    sets: dict = field(default_factory=dict)


def load_paths(text: str) -> list[Path]:
    """Read paths.csv. `as_path` is space-separated AS numbers; `type` is ebgp, ibgp or local."""
    paths = []
    for row in csv.DictReader(io.StringIO(text)):
        kind = row["type"].strip()
        paths.append(Path(
            prefix=row["prefix"].strip(),
            neighbor=row["neighbor"].strip(),
            weight=int(row["weight"] or 0),
            local_pref=int(row["local_pref"] or 100),
            as_path=[int(a) for a in row["as_path"].split()],
            origin=row["origin"].strip() or "i",
            med=int(row["med"] or 0),
            ebgp=kind == "ebgp",
            igp_metric=int(row["igp_metric"] or 0),
            router_id=row["router_id"].strip(),
            local=kind == "local",
        ))
    return paths


def load_config(text: str) -> tuple[dict[str, list[Entry]], dict[str, str]]:
    """Read IOS-style route maps and `neighbor X route-map NAME in` lines.

    Returns (route maps by name with entries sorted by sequence, route-map name by neighbor)."""
    maps: dict[str, list[Entry]] = {}
    bound: dict[str, str] = {}
    current = None
    for raw in text.splitlines():
        line = raw.split("!")[0].strip()
        if not line:
            continue
        words = line.split()
        if words[0] == "route-map":
            current = Entry(action=words[2], seq=int(words[3]))
            maps.setdefault(words[1], []).append(current)
        elif words[0] == "neighbor" and words[2] == "route-map" and words[-1] == "in":
            bound[words[1]] = words[3]
        elif words[0] == "match" and current:
            if words[1] == "prefix":
                current.match_prefixes.append(" ".join(words[2:]))
            elif words[1] == "as-path":
                current.match_as.extend(int(a) for a in words[2:])
        elif words[0] == "set" and current:
            if words[1] == "as-path" and words[2] == "prepend":
                current.sets["prepend"] = [int(a) for a in words[3:]]
            else:
                current.sets[words[1]] = int(words[2])
    for entries in maps.values():
        entries.sort(key=lambda e: e.seq)
    return maps, bound


# HINT: split the rule into the network and optional "ge N" / "le N".
# HINT: without ge or le, the length must equal the network's length exactly.
# HINT: with only ge, the upper bound is 32; with only le, the lower bound is the network's length.
def prefix_matches(rule: str, prefix: str) -> bool:
    """Like an IOS prefix-list line: does `prefix` match `rule`?

    "10.0.0.0/8"            matches only 10.0.0.0/8 itself.
    "10.0.0.0/8 le 32"      matches 10.0.0.0/8 and anything inside it.
    "10.0.0.0/8 ge 24 le 28" matches prefixes inside 10/8 with lengths 24 to 28.
    The prefix must be inside the rule's network (its first bits equal)."""
    raise NotImplementedError


# HINT: go through the entries in order; an entry matches when ALL its match lists are satisfied
# HINT: (any of its prefixes matches, and any of its AS numbers is in the AS path). An empty list matches anything.
# HINT: use dataclasses.replace(path, ...) to return a changed copy; prepend goes in FRONT of the AS path.
def apply_route_map(entries: list[Entry], path: Path) -> Path | None:
    """Run a path through a route map. The first matching entry decides:
    deny → None (the path is dropped); permit → a copy with its `set`s applied
    (local-preference, weight, metric (MED), as-path prepend).
    No entry matches → None: the implicit deny at the end."""
    raise NotImplementedError


def router_id_key(rid: str) -> int:
    return int(ipaddress.ip_address(rid))


# HINT: build a list of (reason, value_for_a, value_for_b, prefer) and walk it in order,
# HINT: where prefer is "high" or "low". Compare MED only when both paths come from the same
# HINT: neighboring AS (the first AS in the AS path; a locally originated path has none).
def compare(a: Path, b: Path) -> tuple[Path, str]:
    """Return (the better path, the reason) using Cisco's order:
    "weight" (higher), "local preference" (higher), "locally originated" (local wins),
    "AS path length" (shorter), "origin" (i < e < ?), "MED" (lower, same neighbor AS only),
    "eBGP over iBGP", "IGP metric" (lower), "router ID" (lower).
    If everything ties, return (a, "tie")."""
    raise NotImplementedError


# HINT: start with the first path as the best and compare it with each of the others.
# HINT: keep the reason from the LAST comparison that changed or confirmed the winner.
def best_path(paths: list[Path]) -> tuple[Path, str]:
    """The best of several paths to the same prefix, and why it beat the runner-up it was last
    compared with. A single path wins with the reason "only path"."""
    raise NotImplementedError


def decide(paths: list[Path], maps: dict[str, list[Entry]], bound: dict[str, str]) -> dict[str, tuple[Path, str]]:
    """Apply each neighbor's inbound route map, then pick the best path for every prefix."""
    by_prefix: dict[str, list[Path]] = {}
    for p in paths:
        name = bound.get(p.neighbor)
        if name is not None:
            p = apply_route_map(maps.get(name, []), p)
            if p is None:
                continue
        by_prefix.setdefault(p.prefix, []).append(p)
    return {prefix: best_path(ps) for prefix, ps in by_prefix.items()}
@@file test_bgp.py
import unittest

from bgp import Entry, Path, apply_route_map, best_path, compare, decide, load_config, prefix_matches


def path(**kw):
    base = dict(prefix="0.0.0.0/0", neighbor="203.0.113.1", as_path=[64500], router_id="1.1.1.1")
    base.update(kw)
    return Path(**base)


class PrefixLists(unittest.TestCase):
    def test_exact(self):
        self.assertTrue(prefix_matches("10.0.0.0/8", "10.0.0.0/8"))
        self.assertFalse(prefix_matches("10.0.0.0/8", "10.1.0.0/16"))

    def test_le(self):
        self.assertTrue(prefix_matches("10.0.0.0/8 le 32", "10.1.2.0/24"))
        self.assertTrue(prefix_matches("10.0.0.0/8 le 32", "10.0.0.0/8"))
        self.assertFalse(prefix_matches("10.0.0.0/8 le 32", "11.0.0.0/8"))

    def test_ge_le(self):
        self.assertTrue(prefix_matches("10.0.0.0/8 ge 24 le 28", "10.9.9.0/24"))
        self.assertFalse(prefix_matches("10.0.0.0/8 ge 24 le 28", "10.9.0.0/16"))
        self.assertFalse(prefix_matches("10.0.0.0/8 ge 24 le 28", "10.9.9.0/30"))

    def test_ge_only(self):
        self.assertTrue(prefix_matches("192.168.0.0/16 ge 24", "192.168.5.128/25"))
        self.assertFalse(prefix_matches("192.168.0.0/16 ge 24", "192.168.0.0/16"))


class RouteMaps(unittest.TestCase):
    def test_set_and_copy(self):
        rm = [Entry("permit", 10, match_prefixes=["0.0.0.0/0"], sets={"local-preference": 200})]
        p = path()
        out = apply_route_map(rm, p)
        self.assertEqual(out.local_pref, 200)
        self.assertEqual(p.local_pref, 100, "the original path must not change")

    def test_deny_and_implicit_deny(self):
        rm = [Entry("deny", 10, match_prefixes=["10.0.0.0/8 le 32"]),
              Entry("permit", 20, match_prefixes=["0.0.0.0/0"])]
        self.assertIsNone(apply_route_map(rm, path(prefix="10.1.0.0/16")))
        self.assertIsNotNone(apply_route_map(rm, path()))
        self.assertIsNone(apply_route_map(rm, path(prefix="198.51.100.0/24")), "implicit deny")

    def test_empty_permit_matches_all(self):
        rm = [Entry("permit", 10)]
        self.assertEqual(apply_route_map(rm, path(prefix="1.2.3.0/24")).prefix, "1.2.3.0/24")

    def test_prepend_and_as_match(self):
        rm = [Entry("permit", 10, match_as=[64999], sets={"prepend": [65010, 65010]}), Entry("permit", 20)]
        out = apply_route_map(rm, path(as_path=[64500, 64999]))
        self.assertEqual(out.as_path, [65010, 65010, 64500, 64999])
        self.assertEqual(apply_route_map(rm, path()).as_path, [64500])


class BestPath(unittest.TestCase):
    def test_order(self):
        cases = [
            (path(weight=100), path(local_pref=500), "weight"),
            (path(local_pref=200), path(as_path=[]), "local preference"),
            (path(local=True, as_path=[]), path(), "locally originated"),
            (path(as_path=[1]), path(as_path=[1, 2]), "AS path length"),
            (path(origin="i"), path(origin="?"), "origin"),
            (path(med=10), path(med=50), "MED"),
            (path(ebgp=True), path(ebgp=False), "eBGP over iBGP"),
            (path(igp_metric=5), path(igp_metric=20), "IGP metric"),
            (path(router_id="1.1.1.1"), path(router_id="2.2.2.2"), "router ID"),
        ]
        for good, bad, reason in cases:
            with self.subTest(reason):
                self.assertEqual(compare(good, bad), (good, reason))
                self.assertEqual(compare(bad, good), (good, reason))

    def test_med_only_same_neighbor_as(self):
        a = path(as_path=[64500], med=100, igp_metric=5)
        b = path(as_path=[64501], med=0, igp_metric=10)
        self.assertEqual(compare(a, b), (a, "IGP metric"))

    def test_router_id_numeric(self):
        a = path(router_id="9.9.9.9")
        b = path(router_id="10.0.0.1")
        self.assertEqual(compare(a, b)[0], a, "compare router IDs as numbers, not text")

    def test_best_of_many(self):
        ps = [path(as_path=[1, 2, 3]), path(as_path=[1, 2]), path(as_path=[1, 2, 3, 4])]
        self.assertIs(best_path(ps)[0], ps[1])
        self.assertEqual(best_path([ps[0]]), (ps[0], "only path"))


class EndToEnd(unittest.TestCase):
    def test_decide(self):
        maps, bound = load_config("""
route-map A-IN permit 10
 match prefix 0.0.0.0/0
 set local-preference 200
route-map A-IN permit 20
neighbor 203.0.113.1 route-map A-IN in
""")
        a = path(neighbor="203.0.113.1", as_path=[64500, 64600])
        b = path(neighbor="198.51.100.1", as_path=[64700], router_id="2.2.2.2")
        best, reason = decide([a, b], maps, bound)["0.0.0.0/0"]
        self.assertEqual((best.neighbor, reason), ("203.0.113.1", "local preference"))


if __name__ == "__main__":
    unittest.main()
@@file main.py
"""Show the BGP table the router would build: `python main.py [paths.csv] [config.txt]`."""
import sys
from pathlib import Path as File

from bgp import decide, load_config, load_paths


def main(argv: list[str]) -> int:
    paths_file = argv[1] if len(argv) > 1 else "paths.csv"
    config_file = argv[2] if len(argv) > 2 else "config.txt"
    paths = load_paths(File(paths_file).read_text(encoding="utf-8"))
    maps, bound = load_config(File(config_file).read_text(encoding="utf-8"))
    table = decide(paths, maps, bound)
    print(f"{'Network':<20}{'Next hop':<16}{'LocPrf':>7}{'Weight':>7}  {'Path':<22}Chosen because")
    for prefix in sorted(table, key=lambda p: (int(p.split("/")[1]), p)):
        best, reason = table[prefix]
        as_path = " ".join(map(str, best.as_path)) + " " + best.origin
        print(f"{prefix:<20}{best.neighbor:<16}{best.local_pref:>7}{best.weight:>7}  {as_path.strip():<22}{reason}")
    dropped = len({p.prefix for p in paths}) - len(table)
    if dropped:
        print(f"({dropped} prefix(es) filtered out completely by route maps)")
    return 0


if __name__ == "__main__":
    sys.exit(main(sys.argv))
@@file paths.csv
prefix,neighbor,type,weight,local_pref,as_path,origin,med,igp_metric,router_id
0.0.0.0/0,203.0.113.1,ebgp,0,100,64500,i,0,0,203.0.113.1
0.0.0.0/0,198.51.100.1,ebgp,0,100,64700,i,0,0,198.51.100.1
8.8.8.0/24,203.0.113.1,ebgp,0,100,64500 15169,i,0,0,203.0.113.1
8.8.8.0/24,198.51.100.1,ebgp,0,100,64700 3356 15169,i,0,0,198.51.100.1
1.1.1.0/24,203.0.113.1,ebgp,0,100,64500 6939 13335,i,0,0,203.0.113.1
1.1.1.0/24,198.51.100.1,ebgp,0,100,64700 13335,i,0,0,198.51.100.1
104.16.0.0/13,203.0.113.1,ebgp,0,100,64500 13335,i,50,0,203.0.113.1
104.16.0.0/13,203.0.113.5,ebgp,0,100,64500 13335,i,10,0,203.0.113.5
198.18.0.0/15,10.0.0.2,ibgp,0,100,64900,?,0,20,10.0.0.2
198.18.0.0/15,10.0.0.3,ibgp,0,100,64900,i,0,10,10.0.0.3
10.200.0.0/16,203.0.113.1,ebgp,0,100,64500,i,0,0,203.0.113.1
172.16.50.0/24,10.0.0.2,ibgp,0,100,,i,0,20,10.0.0.2
172.16.50.0/24,10.0.0.3,ibgp,0,100,,i,0,10,10.0.0.3
198.51.100.0/24,0.0.0.0,local,32768,100,,i,0,0,10.0.0.1
@@file config.txt
! Two ISPs: A (AS 64500, primary) and B (AS 64700, backup).
! Prefer ISP A for the default route, never accept private space from an ISP.

route-map ISP-A-IN deny 5
 match prefix 10.0.0.0/8 le 32
 match prefix 172.16.0.0/12 le 32
 match prefix 192.168.0.0/16 le 32
route-map ISP-A-IN permit 10
 match prefix 0.0.0.0/0
 set local-preference 200
route-map ISP-A-IN permit 20

route-map ISP-B-IN deny 5
 match prefix 10.0.0.0/8 le 32
 match prefix 172.16.0.0/12 le 32
 match prefix 192.168.0.0/16 le 32
route-map ISP-B-IN permit 20

neighbor 203.0.113.1 route-map ISP-A-IN in
neighbor 203.0.113.5 route-map ISP-A-IN in
neighbor 198.51.100.1 route-map ISP-B-IN in
@@text
> Tips: Python's `ipaddress` module does the bit work: `ip_network("10.1.2.0/24").subnet_of(ip_network("10.0.0.0/8"))` and `.prefixlen`. Compare router IDs as **numbers** (`int(ipaddress.ip_address(rid))`): as text, "9.9.9.9" sorts after "10.0.0.1". `dataclasses.replace(path, as_path=list(path.as_path))` gives you a safe copy to change.

Then experiment, the way you would in a lab:

- Make ISP B the primary instead. Which single line in `config.txt` do you change?
- Add `set as-path prepend 65010 65010 65010` somewhere. Why doesn't it change *your* router's choice for outbound traffic? (Prepending is for influencing *other* networks.)
- Delete the final empty `permit 20` from `ISP-A-IN` and run it again. That's the outage from the lesson.
@@project bgp_simulator_finished BGP Decision Simulator: the finished version
The complete simulator. Ideas: multipath (`maximum-paths`: install every path that ties down to the IGP metric), communities (`set community 65010:100` and matching on them), weight set per neighbor, or reading real `show ip bgp` output from a lab router or a public route server.
@@file bgp.py
"""A small BGP decision simulator: inbound route maps, then best-path selection per prefix."""
import csv
import io
import ipaddress
from dataclasses import dataclass, field, replace

ORIGIN_RANK = {"i": 0, "e": 1, "?": 2}


@dataclass
class Path:
    prefix: str
    neighbor: str
    weight: int = 0
    local_pref: int = 100
    as_path: list[int] = field(default_factory=list)
    origin: str = "i"
    med: int = 0
    ebgp: bool = True
    igp_metric: int = 0
    router_id: str = "0.0.0.0"
    local: bool = False


@dataclass
class Entry:
    """One route-map entry: `permit`/`deny`, a sequence number, matches and sets."""
    action: str
    seq: int
    match_prefixes: list[str] = field(default_factory=list)
    match_as: list[int] = field(default_factory=list)
    sets: dict = field(default_factory=dict)


def load_paths(text: str) -> list[Path]:
    """Read paths.csv. `as_path` is space-separated AS numbers; `type` is ebgp, ibgp or local."""
    paths = []
    for row in csv.DictReader(io.StringIO(text)):
        kind = row["type"].strip()
        paths.append(Path(
            prefix=row["prefix"].strip(),
            neighbor=row["neighbor"].strip(),
            weight=int(row["weight"] or 0),
            local_pref=int(row["local_pref"] or 100),
            as_path=[int(a) for a in row["as_path"].split()],
            origin=row["origin"].strip() or "i",
            med=int(row["med"] or 0),
            ebgp=kind == "ebgp",
            igp_metric=int(row["igp_metric"] or 0),
            router_id=row["router_id"].strip(),
            local=kind == "local",
        ))
    return paths


def load_config(text: str) -> tuple[dict[str, list[Entry]], dict[str, str]]:
    """Read IOS-style route maps and `neighbor X route-map NAME in` lines.

    Returns (route maps by name with entries sorted by sequence, route-map name by neighbor)."""
    maps: dict[str, list[Entry]] = {}
    bound: dict[str, str] = {}
    current = None
    for raw in text.splitlines():
        line = raw.split("!")[0].strip()
        if not line:
            continue
        words = line.split()
        if words[0] == "route-map":
            current = Entry(action=words[2], seq=int(words[3]))
            maps.setdefault(words[1], []).append(current)
        elif words[0] == "neighbor" and words[2] == "route-map" and words[-1] == "in":
            bound[words[1]] = words[3]
        elif words[0] == "match" and current:
            if words[1] == "prefix":
                current.match_prefixes.append(" ".join(words[2:]))
            elif words[1] == "as-path":
                current.match_as.extend(int(a) for a in words[2:])
        elif words[0] == "set" and current:
            if words[1] == "as-path" and words[2] == "prepend":
                current.sets["prepend"] = [int(a) for a in words[3:]]
            else:
                current.sets[words[1]] = int(words[2])
    for entries in maps.values():
        entries.sort(key=lambda e: e.seq)
    return maps, bound


# HINT: split the rule into the network and optional "ge N" / "le N".
# HINT: without ge or le, the length must equal the network's length exactly.
# HINT: with only ge, the upper bound is 32; with only le, the lower bound is the network's length.
def prefix_matches(rule: str, prefix: str) -> bool:
    """Like an IOS prefix-list line: does `prefix` match `rule`?

    "10.0.0.0/8"            matches only 10.0.0.0/8 itself.
    "10.0.0.0/8 le 32"      matches 10.0.0.0/8 and anything inside it.
    "10.0.0.0/8 ge 24 le 28" matches prefixes inside 10/8 with lengths 24 to 28.
    The prefix must be inside the rule's network (its first bits equal)."""
    words = rule.split()
    net = ipaddress.ip_network(words[0])
    candidate = ipaddress.ip_network(prefix)
    opts = dict(zip(words[1::2], (int(w) for w in words[2::2])))
    if "ge" in opts or "le" in opts:
        lo, hi = opts.get("ge", net.prefixlen), opts.get("le", 32)
    else:
        lo = hi = net.prefixlen
    return candidate.subnet_of(net) and lo <= candidate.prefixlen <= hi


# HINT: go through the entries in order; an entry matches when ALL its match lists are satisfied
# HINT: (any of its prefixes matches, and any of its AS numbers is in the AS path). An empty list matches anything.
# HINT: use dataclasses.replace(path, ...) to return a changed copy; prepend goes in FRONT of the AS path.
def apply_route_map(entries: list[Entry], path: Path) -> Path | None:
    """Run a path through a route map. The first matching entry decides:
    deny → None (the path is dropped); permit → a copy with its `set`s applied
    (local-preference, weight, metric (MED), as-path prepend).
    No entry matches → None: the implicit deny at the end."""
    for e in entries:
        if e.match_prefixes and not any(prefix_matches(r, path.prefix) for r in e.match_prefixes):
            continue
        if e.match_as and not any(a in path.as_path for a in e.match_as):
            continue
        if e.action == "deny":
            return None
        p = replace(path, as_path=list(path.as_path))
        if "local-preference" in e.sets:
            p.local_pref = e.sets["local-preference"]
        if "weight" in e.sets:
            p.weight = e.sets["weight"]
        if "metric" in e.sets:
            p.med = e.sets["metric"]
        if "prepend" in e.sets:
            p.as_path = e.sets["prepend"] + p.as_path
        return p
    return None


def router_id_key(rid: str) -> int:
    return int(ipaddress.ip_address(rid))


# HINT: build a list of (reason, value_for_a, value_for_b, prefer) and walk it in order,
# HINT: where prefer is "high" or "low". Compare MED only when both paths come from the same
# HINT: neighboring AS (the first AS in the AS path; a locally originated path has none).
def compare(a: Path, b: Path) -> tuple[Path, str]:
    """Return (the better path, the reason) using Cisco's order:
    "weight" (higher), "local preference" (higher), "locally originated" (local wins),
    "AS path length" (shorter), "origin" (i < e < ?), "MED" (lower, same neighbor AS only),
    "eBGP over iBGP", "IGP metric" (lower), "router ID" (lower).
    If everything ties, return (a, "tie")."""
    def first_as(p: Path):
        return p.as_path[0] if p.as_path else None

    steps = [
        ("weight", a.weight, b.weight, "high"),
        ("local preference", a.local_pref, b.local_pref, "high"),
        ("locally originated", a.local, b.local, "high"),
        ("AS path length", len(a.as_path), len(b.as_path), "low"),
        ("origin", ORIGIN_RANK[a.origin], ORIGIN_RANK[b.origin], "low"),
    ]
    if first_as(a) == first_as(b):
        steps.append(("MED", a.med, b.med, "low"))
    steps += [
        ("eBGP over iBGP", a.ebgp, b.ebgp, "high"),
        ("IGP metric", a.igp_metric, b.igp_metric, "low"),
        ("router ID", router_id_key(a.router_id), router_id_key(b.router_id), "low"),
    ]
    for reason, va, vb, prefer in steps:
        if va != vb:
            a_wins = va > vb if prefer == "high" else va < vb
            return (a if a_wins else b), reason
    return a, "tie"


# HINT: start with the first path as the best and compare it with each of the others.
# HINT: keep the reason from the LAST comparison that changed or confirmed the winner.
def best_path(paths: list[Path]) -> tuple[Path, str]:
    """The best of several paths to the same prefix, and why it beat the runner-up it was last
    compared with. A single path wins with the reason "only path"."""
    best, reason = paths[0], "only path"
    for other in paths[1:]:
        best, reason = compare(best, other)
    return best, reason


def decide(paths: list[Path], maps: dict[str, list[Entry]], bound: dict[str, str]) -> dict[str, tuple[Path, str]]:
    """Apply each neighbor's inbound route map, then pick the best path for every prefix."""
    by_prefix: dict[str, list[Path]] = {}
    for p in paths:
        name = bound.get(p.neighbor)
        if name is not None:
            p = apply_route_map(maps.get(name, []), p)
            if p is None:
                continue
        by_prefix.setdefault(p.prefix, []).append(p)
    return {prefix: best_path(ps) for prefix, ps in by_prefix.items()}
@@file test_bgp.py
import unittest

from bgp import Entry, Path, apply_route_map, best_path, compare, decide, load_config, prefix_matches


def path(**kw):
    base = dict(prefix="0.0.0.0/0", neighbor="203.0.113.1", as_path=[64500], router_id="1.1.1.1")
    base.update(kw)
    return Path(**base)


class PrefixLists(unittest.TestCase):
    def test_exact(self):
        self.assertTrue(prefix_matches("10.0.0.0/8", "10.0.0.0/8"))
        self.assertFalse(prefix_matches("10.0.0.0/8", "10.1.0.0/16"))

    def test_le(self):
        self.assertTrue(prefix_matches("10.0.0.0/8 le 32", "10.1.2.0/24"))
        self.assertTrue(prefix_matches("10.0.0.0/8 le 32", "10.0.0.0/8"))
        self.assertFalse(prefix_matches("10.0.0.0/8 le 32", "11.0.0.0/8"))

    def test_ge_le(self):
        self.assertTrue(prefix_matches("10.0.0.0/8 ge 24 le 28", "10.9.9.0/24"))
        self.assertFalse(prefix_matches("10.0.0.0/8 ge 24 le 28", "10.9.0.0/16"))
        self.assertFalse(prefix_matches("10.0.0.0/8 ge 24 le 28", "10.9.9.0/30"))

    def test_ge_only(self):
        self.assertTrue(prefix_matches("192.168.0.0/16 ge 24", "192.168.5.128/25"))
        self.assertFalse(prefix_matches("192.168.0.0/16 ge 24", "192.168.0.0/16"))


class RouteMaps(unittest.TestCase):
    def test_set_and_copy(self):
        rm = [Entry("permit", 10, match_prefixes=["0.0.0.0/0"], sets={"local-preference": 200})]
        p = path()
        out = apply_route_map(rm, p)
        self.assertEqual(out.local_pref, 200)
        self.assertEqual(p.local_pref, 100, "the original path must not change")

    def test_deny_and_implicit_deny(self):
        rm = [Entry("deny", 10, match_prefixes=["10.0.0.0/8 le 32"]),
              Entry("permit", 20, match_prefixes=["0.0.0.0/0"])]
        self.assertIsNone(apply_route_map(rm, path(prefix="10.1.0.0/16")))
        self.assertIsNotNone(apply_route_map(rm, path()))
        self.assertIsNone(apply_route_map(rm, path(prefix="198.51.100.0/24")), "implicit deny")

    def test_empty_permit_matches_all(self):
        rm = [Entry("permit", 10)]
        self.assertEqual(apply_route_map(rm, path(prefix="1.2.3.0/24")).prefix, "1.2.3.0/24")

    def test_prepend_and_as_match(self):
        rm = [Entry("permit", 10, match_as=[64999], sets={"prepend": [65010, 65010]}), Entry("permit", 20)]
        out = apply_route_map(rm, path(as_path=[64500, 64999]))
        self.assertEqual(out.as_path, [65010, 65010, 64500, 64999])
        self.assertEqual(apply_route_map(rm, path()).as_path, [64500])


class BestPath(unittest.TestCase):
    def test_order(self):
        cases = [
            (path(weight=100), path(local_pref=500), "weight"),
            (path(local_pref=200), path(as_path=[]), "local preference"),
            (path(local=True, as_path=[]), path(), "locally originated"),
            (path(as_path=[1]), path(as_path=[1, 2]), "AS path length"),
            (path(origin="i"), path(origin="?"), "origin"),
            (path(med=10), path(med=50), "MED"),
            (path(ebgp=True), path(ebgp=False), "eBGP over iBGP"),
            (path(igp_metric=5), path(igp_metric=20), "IGP metric"),
            (path(router_id="1.1.1.1"), path(router_id="2.2.2.2"), "router ID"),
        ]
        for good, bad, reason in cases:
            with self.subTest(reason):
                self.assertEqual(compare(good, bad), (good, reason))
                self.assertEqual(compare(bad, good), (good, reason))

    def test_med_only_same_neighbor_as(self):
        a = path(as_path=[64500], med=100, igp_metric=5)
        b = path(as_path=[64501], med=0, igp_metric=10)
        self.assertEqual(compare(a, b), (a, "IGP metric"))

    def test_router_id_numeric(self):
        a = path(router_id="9.9.9.9")
        b = path(router_id="10.0.0.1")
        self.assertEqual(compare(a, b)[0], a, "compare router IDs as numbers, not text")

    def test_best_of_many(self):
        ps = [path(as_path=[1, 2, 3]), path(as_path=[1, 2]), path(as_path=[1, 2, 3, 4])]
        self.assertIs(best_path(ps)[0], ps[1])
        self.assertEqual(best_path([ps[0]]), (ps[0], "only path"))


class EndToEnd(unittest.TestCase):
    def test_decide(self):
        maps, bound = load_config("""
route-map A-IN permit 10
 match prefix 0.0.0.0/0
 set local-preference 200
route-map A-IN permit 20
neighbor 203.0.113.1 route-map A-IN in
""")
        a = path(neighbor="203.0.113.1", as_path=[64500, 64600])
        b = path(neighbor="198.51.100.1", as_path=[64700], router_id="2.2.2.2")
        best, reason = decide([a, b], maps, bound)["0.0.0.0/0"]
        self.assertEqual((best.neighbor, reason), ("203.0.113.1", "local preference"))


if __name__ == "__main__":
    unittest.main()
@@file main.py
"""Show the BGP table the router would build: `python main.py [paths.csv] [config.txt]`."""
import sys
from pathlib import Path as File

from bgp import decide, load_config, load_paths


def main(argv: list[str]) -> int:
    paths_file = argv[1] if len(argv) > 1 else "paths.csv"
    config_file = argv[2] if len(argv) > 2 else "config.txt"
    paths = load_paths(File(paths_file).read_text(encoding="utf-8"))
    maps, bound = load_config(File(config_file).read_text(encoding="utf-8"))
    table = decide(paths, maps, bound)
    print(f"{'Network':<20}{'Next hop':<16}{'LocPrf':>7}{'Weight':>7}  {'Path':<22}Chosen because")
    for prefix in sorted(table, key=lambda p: (int(p.split("/")[1]), p)):
        best, reason = table[prefix]
        as_path = " ".join(map(str, best.as_path)) + " " + best.origin
        print(f"{prefix:<20}{best.neighbor:<16}{best.local_pref:>7}{best.weight:>7}  {as_path.strip():<22}{reason}")
    dropped = len({p.prefix for p in paths}) - len(table)
    if dropped:
        print(f"({dropped} prefix(es) filtered out completely by route maps)")
    return 0


if __name__ == "__main__":
    sys.exit(main(sys.argv))
@@file paths.csv
prefix,neighbor,type,weight,local_pref,as_path,origin,med,igp_metric,router_id
0.0.0.0/0,203.0.113.1,ebgp,0,100,64500,i,0,0,203.0.113.1
0.0.0.0/0,198.51.100.1,ebgp,0,100,64700,i,0,0,198.51.100.1
8.8.8.0/24,203.0.113.1,ebgp,0,100,64500 15169,i,0,0,203.0.113.1
8.8.8.0/24,198.51.100.1,ebgp,0,100,64700 3356 15169,i,0,0,198.51.100.1
1.1.1.0/24,203.0.113.1,ebgp,0,100,64500 6939 13335,i,0,0,203.0.113.1
1.1.1.0/24,198.51.100.1,ebgp,0,100,64700 13335,i,0,0,198.51.100.1
104.16.0.0/13,203.0.113.1,ebgp,0,100,64500 13335,i,50,0,203.0.113.1
104.16.0.0/13,203.0.113.5,ebgp,0,100,64500 13335,i,10,0,203.0.113.5
198.18.0.0/15,10.0.0.2,ibgp,0,100,64900,?,0,20,10.0.0.2
198.18.0.0/15,10.0.0.3,ibgp,0,100,64900,i,0,10,10.0.0.3
10.200.0.0/16,203.0.113.1,ebgp,0,100,64500,i,0,0,203.0.113.1
172.16.50.0/24,10.0.0.2,ibgp,0,100,,i,0,20,10.0.0.2
172.16.50.0/24,10.0.0.3,ibgp,0,100,,i,0,10,10.0.0.3
198.51.100.0/24,0.0.0.0,local,32768,100,,i,0,0,10.0.0.1
@@file config.txt
! Two ISPs: A (AS 64500, primary) and B (AS 64700, backup).
! Prefer ISP A for the default route, never accept private space from an ISP.

route-map ISP-A-IN deny 5
 match prefix 10.0.0.0/8 le 32
 match prefix 172.16.0.0/12 le 32
 match prefix 192.168.0.0/16 le 32
route-map ISP-A-IN permit 10
 match prefix 0.0.0.0/0
 set local-preference 200
route-map ISP-A-IN permit 20

route-map ISP-B-IN deny 5
 match prefix 10.0.0.0/8 le 32
 match prefix 172.16.0.0/12 le 32
 match prefix 192.168.0.0/16 le 32
route-map ISP-B-IN permit 20

neighbor 203.0.113.1 route-map ISP-A-IN in
neighbor 203.0.113.5 route-map ISP-A-IN in
neighbor 198.51.100.1 route-map ISP-B-IN in

@@drill
Which BGP attribute, local to one router, is compared first in Cisco's best-path selection?
= Weight
= weight
? Then local preference, then locally originated, then AS path length.

@@text
🎉 You now know the BGP decision process better than many people who configure it daily. In an interview for a network engineer role, walking through *why* a path won, attribute by attribute, is exactly what they want to hear.
