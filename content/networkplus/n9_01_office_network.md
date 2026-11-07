@@lesson n9-office-network Office Network Planner
@@text
Sakura Systems is opening an **Osaka office**, and you're designing its network: VLANs for staff, phones, printers, servers, guests and management, each with its own subnet, gateway and DHCP range. On a real project, the plan is reviewed before anything is configured, because a mistake (an overlapping subnet, a gateway inside the DHCP pool) means outages on day one.

You'll write a tool that does that review automatically, plans subnets with **VLSM**, and then **writes the switch configuration** for you:

```text
python main.py check plan.json
python main.py check broken_plan.json
python main.py vlsm 192.168.50.0/24 Sales=100 IT=50 Printers=20 LinkA=2 LinkB=2
python main.py config plan.json
```

```text
vlan 10
 name Staff
!
interface Vlan10
 description Staff
 ip address 10.20.10.1 255.255.255.0
 ip helper-address 10.20.99.10
 no shutdown
!
```

That output is real Cisco IOS configuration for a layer 3 switch (the CCNA track explains every line): a VLAN, its **SVI** (the VLAN's gateway interface), and a **DHCP relay** pointing at the server in the management VLAN.

## The plan file

`plan.json` describes the site: a **supernet** the whole office must fit in, and one entry per VLAN with `subnet`, `gateway`, an optional `dhcp` range and `hosts_needed`. Data in, problems out: a format any colleague can edit.

Python's `ipaddress` module does the bit maths: `ip_network("10.20.10.0/24")` has `.network_address`, `.broadcast_address`, `.num_addresses`, `.netmask`, `.overlaps(other)` and `.subnet_of(other)`; `ip_address("10.20.10.1")` values compare with `<`. Use it here, now that you can subnet by hand.
@@project netplan Office Network Planner
Write the four functions in `netplan.py` until `python -m unittest` passes. Then run the checker on `broken_plan.json`: try to spot all the mistakes yourself first.
@@file netplan.py
"""Check an office network plan, plan subnets with VLSM, and generate switch configuration."""

import ipaddress


def smallest_prefix(hosts):
    """The longest IPv4 prefix whose usable addresses (2^host_bits - 2) hold `hosts`.
    2 -> 30, 30 -> 27, 31 -> 26. ValueError if not even a /1 is big enough."""
    raise NotImplementedError("TODO: smallest_prefix")


def vlsm(supernet, needs):
    """Allocate subnets for needs = [(name, hosts), ...] inside `supernet`.

    Largest first (equal sizes keep their order), one after another from the start of the supernet.
    Returns [(name, "a.b.c.d/prefix"), ...] in allocation order. ValueError if they don't fit."""
    raise NotImplementedError("TODO: vlsm")


def check_plan(plan):
    """Every problem in a plan, as a list of messages (empty when the plan is fine).

    plan = {"supernet": "10.20.0.0/16", "vlans": [{"id", "name", "subnet", "gateway",
            optional "dhcp": [start, end], optional "hosts_needed"}, ...]}

    Messages, checked in this order for each VLAN:
      VLAN 5000: id must be 1-4094
      VLAN 10: duplicate id
      VLAN 10: 10.20.10.5/24 is not a valid network address      (skip the VLAN's other checks)
      VLAN 10: subnet 10.30.0.0/24 is not inside 10.20.0.0/16
      VLAN 10 and VLAN 20: subnets overlap                         (earlier VLAN first)
      VLAN 10: gateway 10.20.11.1 is not a usable address in 10.20.10.0/24
      VLAN 10: DHCP range is not inside 10.20.10.0/24
      VLAN 10: DHCP range starts after it ends
      VLAN 10: gateway is inside the DHCP range
      VLAN 10: 10.20.10.0/27 has 30 usable addresses, 50 needed
    """
    raise NotImplementedError("TODO: check_plan")


def switch_config(plan, dhcp_server=None):
    """Cisco-style configuration for a layer 3 switch: for each VLAN

        vlan 10
         name Staff
        !
        interface Vlan10
         description Staff
         ip address 10.20.10.1 255.255.255.0
         ip helper-address 10.20.99.10      (only if dhcp_server is given and the VLAN has "dhcp")
         no shutdown
        !

    Returns the text, ending with a newline."""
    raise NotImplementedError("TODO: switch_config")
@@file plan.json
{
  "site": "Sakura Systems Osaka office",
  "supernet": "10.20.0.0/16",
  "dhcp_server": "10.20.99.10",
  "vlans": [
    {"id": 10, "name": "Staff", "subnet": "10.20.10.0/24", "gateway": "10.20.10.1", "dhcp": ["10.20.10.100", "10.20.10.250"], "hosts_needed": 120},
    {"id": 20, "name": "Voice", "subnet": "10.20.20.0/24", "gateway": "10.20.20.1", "dhcp": ["10.20.20.100", "10.20.20.250"], "hosts_needed": 100},
    {"id": 30, "name": "Printers", "subnet": "10.20.30.0/27", "gateway": "10.20.30.1", "hosts_needed": 15},
    {"id": 40, "name": "Servers", "subnet": "10.20.40.0/26", "gateway": "10.20.40.1", "hosts_needed": 20},
    {"id": 50, "name": "Guest", "subnet": "10.20.50.0/23", "gateway": "10.20.50.1", "dhcp": ["10.20.50.10", "10.20.51.250"], "hosts_needed": 300},
    {"id": 99, "name": "Management", "subnet": "10.20.99.0/28", "gateway": "10.20.99.1", "hosts_needed": 10}
  ]
}
@@file broken_plan.json
{
  "site": "A plan a colleague sent you for review. How many mistakes can you find before running the checker?",
  "supernet": "172.16.0.0/16",
  "dhcp_server": "172.16.99.10",
  "vlans": [
    {"id": 10, "name": "Staff", "subnet": "172.16.10.0/25", "gateway": "172.16.10.1", "dhcp": ["172.16.10.50", "172.16.10.200"], "hosts_needed": 150},
    {"id": 20, "name": "Voice", "subnet": "172.16.10.64/26", "gateway": "172.16.10.65", "hosts_needed": 40},
    {"id": 30, "name": "Printers", "subnet": "172.16.30.0/28", "gateway": "172.16.30.20", "hosts_needed": 10},
    {"id": 30, "name": "Cameras", "subnet": "172.16.31.0/24", "gateway": "172.16.31.1", "dhcp": ["172.16.31.1", "172.16.31.100"]},
    {"id": 50, "name": "Guest", "subnet": "192.168.50.0/24", "gateway": "192.168.50.1", "dhcp": ["192.168.50.200", "192.168.50.20"]},
    {"id": 4095, "name": "Management", "subnet": "172.16.99.0/28", "gateway": "172.16.99.1"}
  ]
}
@@file test_netplan.py
import json
import unittest
from pathlib import Path

from netplan import check_plan, smallest_prefix, switch_config, vlsm

HERE = Path(__file__).parent


def vlan(vid, subnet, gateway, **extra):
    return {"id": vid, "name": f"V{vid}", "subnet": subnet, "gateway": gateway, **extra}


class PrefixTests(unittest.TestCase):
    def test_sizes(self):
        for hosts, prefix in [(2, 30), (6, 29), (7, 28), (30, 27), (31, 26), (50, 26), (100, 25), (254, 24), (255, 23)]:
            self.assertEqual(smallest_prefix(hosts), prefix, hosts)


class VlsmTests(unittest.TestCase):
    def test_lesson_example_largest_first(self):
        needs = [("Printers", 20), ("LinkA", 2), ("Sales", 100), ("IT", 50), ("LinkB", 2)]
        self.assertEqual(vlsm("192.168.50.0/24", needs), [
            ("Sales", "192.168.50.0/25"),
            ("IT", "192.168.50.128/26"),
            ("Printers", "192.168.50.192/27"),
            ("LinkA", "192.168.50.224/30"),
            ("LinkB", "192.168.50.228/30"),
        ])

    def test_too_big(self):
        with self.assertRaises(ValueError):
            vlsm("10.0.0.0/28", [("A", 20)])
        with self.assertRaises(ValueError):
            vlsm("10.0.0.0/24", [("A", 120), ("B", 120), ("C", 10)])


class CheckTests(unittest.TestCase):
    def check(self, *vlans):
        return check_plan({"supernet": "10.20.0.0/16", "vlans": list(vlans)})

    def test_sample_plan_is_valid(self):
        plan = json.loads((HERE / "plan.json").read_text(encoding="utf-8"))
        self.assertEqual(check_plan(plan), [])

    def test_ids(self):
        self.assertEqual(self.check(vlan(5000, "10.20.1.0/24", "10.20.1.1"), vlan(10, "10.20.2.0/24", "10.20.2.1"),
                                    vlan(10, "10.20.3.0/24", "10.20.3.1")),
                         ["VLAN 5000: id must be 1-4094", "VLAN 10: duplicate id"])

    def test_bad_network_address(self):
        self.assertEqual(self.check(vlan(10, "10.20.10.5/24", "10.20.10.1")),
                         ["VLAN 10: 10.20.10.5/24 is not a valid network address"])

    def test_outside_and_overlap(self):
        self.assertEqual(self.check(vlan(10, "10.30.0.0/24", "10.30.0.1"), vlan(20, "10.20.0.0/23", "10.20.0.1"),
                                    vlan(30, "10.20.1.0/24", "10.20.1.1")),
                         ["VLAN 10: subnet 10.30.0.0/24 is not inside 10.20.0.0/16",
                          "VLAN 20 and VLAN 30: subnets overlap"])

    def test_gateway(self):
        self.assertEqual(self.check(vlan(10, "10.20.10.0/24", "10.20.11.1"), vlan(20, "10.20.20.0/24", "10.20.20.255")),
                         ["VLAN 10: gateway 10.20.11.1 is not a usable address in 10.20.10.0/24",
                          "VLAN 20: gateway 10.20.20.255 is not a usable address in 10.20.20.0/24"])

    def test_dhcp(self):
        self.assertEqual(self.check(
            vlan(10, "10.20.10.0/24", "10.20.10.1", dhcp=["10.20.10.100", "10.20.11.20"]),
            vlan(20, "10.20.20.0/24", "10.20.20.1", dhcp=["10.20.20.200", "10.20.20.100"]),
            vlan(30, "10.20.30.0/24", "10.20.30.1", dhcp=["10.20.30.1", "10.20.30.200"]),
            vlan(40, "10.20.40.0/24", "10.20.40.1", dhcp=["10.20.40.10", "10.20.40.200"]),
        ), ["VLAN 10: DHCP range is not inside 10.20.10.0/24",
            "VLAN 20: DHCP range starts after it ends",
            "VLAN 30: gateway is inside the DHCP range"])

    def test_capacity(self):
        self.assertEqual(self.check(vlan(10, "10.20.10.0/27", "10.20.10.1", hosts_needed=50),
                                    vlan(20, "10.20.20.0/26", "10.20.20.1", hosts_needed=62)),
                         ["VLAN 10: 10.20.10.0/27 has 30 usable addresses, 50 needed"])


class ConfigTests(unittest.TestCase):
    def test_config(self):
        plan = {"supernet": "10.20.0.0/16", "vlans": [
            {"id": 10, "name": "Staff", "subnet": "10.20.10.0/24", "gateway": "10.20.10.1", "dhcp": ["10.20.10.100", "10.20.10.250"]},
            {"id": 30, "name": "Printers", "subnet": "10.20.30.0/27", "gateway": "10.20.30.1"},
        ]}
        text = switch_config(plan, dhcp_server="10.20.99.10")
        self.assertTrue(text.startswith("vlan 10\n name Staff\n!\ninterface Vlan10\n description Staff\n"))
        self.assertIn(" ip address 10.20.10.1 255.255.255.0\n ip helper-address 10.20.99.10\n no shutdown\n!\n", text)
        self.assertIn("interface Vlan30\n description Printers\n ip address 10.20.30.1 255.255.255.224\n no shutdown\n", text)
        self.assertEqual(text.count("helper-address"), 1)
        self.assertTrue(text.endswith("!\n"))

    def test_no_helper_without_server(self):
        plan = {"supernet": "10.20.0.0/16", "vlans": [
            {"id": 10, "name": "Staff", "subnet": "10.20.10.0/24", "gateway": "10.20.10.1", "dhcp": ["10.20.10.100", "10.20.10.250"]}]}
        self.assertNotIn("helper", switch_config(plan))


if __name__ == "__main__":
    unittest.main()
@@file main.py
"""Network plan tools.

python main.py check plan.json
python main.py vlsm 192.168.50.0/24 Sales=100 IT=50 Printers=20 LinkA=2 LinkB=2
python main.py config plan.json > switch.txt
"""

import argparse
import json
import sys

from netplan import check_plan, switch_config, vlsm


def load(path):
    with open(path, encoding="utf-8") as f:
        return json.load(f)


def cmd_check(args):
    plan = load(args.plan)
    problems = check_plan(plan)
    for p in problems:
        print("  x", p)
    if problems:
        print(f"{len(problems)} problem(s) in {args.plan}")
        return 1
    print(f"OK {args.plan}: {len(plan['vlans'])} VLANs, no problems")
    return 0


def cmd_vlsm(args):
    needs = []
    for item in args.needs:
        name, _, hosts = item.partition("=")
        needs.append((name, int(hosts)))
    try:
        result = vlsm(args.supernet, needs)
    except ValueError as e:
        print("error:", e, file=sys.stderr)
        return 1
    print("| Name | Hosts | Subnet |")
    print("|---|---|---|")
    hosts = dict(needs)
    for name, subnet in result:
        print(f"| {name} | {hosts[name]} | {subnet} |")
    return 0


def cmd_config(args):
    plan = load(args.plan)
    problems = check_plan(plan)
    if problems:
        print("Fix the plan first: python main.py check", args.plan, file=sys.stderr)
        return 1
    sys.stdout.write(switch_config(plan, plan.get("dhcp_server")))
    return 0


def main(argv=None):
    parser = argparse.ArgumentParser(description="Office network planning tools")
    sub = parser.add_subparsers(required=True)
    p = sub.add_parser("check", help="find problems in a plan")
    p.add_argument("plan")
    p.set_defaults(func=cmd_check)
    p = sub.add_parser("vlsm", help="plan subnets: NAME=HOSTS ...")
    p.add_argument("supernet")
    p.add_argument("needs", nargs="+")
    p.set_defaults(func=cmd_vlsm)
    p = sub.add_parser("config", help="print switch configuration for a plan")
    p.add_argument("plan")
    p.set_defaults(func=cmd_config)
    args = parser.parse_args(argv)
    return args.func(args)


if __name__ == "__main__":
    sys.exit(main())
@@text
> Tips: `smallest_prefix` can just try prefixes from 30 downwards. In `vlsm`, keep the next free address as an integer (`int(ip_address)`), and round it up to a multiple of the block size before each allocation. `ip_network(...)` raises `ValueError` for `10.20.10.5/24` because host bits are set, which is exactly the "not a valid network address" check.
@@project netplan_finished Office Network Planner: the finished version
The complete tool. Take it further: export the plan as a Markdown table for the documentation, check that the DHCP server is in a VLAN that exists, or generate DHCP scopes for Windows Server with PowerShell (`Add-DhcpServerv4Scope`).
@@file netplan.py
"""Check an office network plan, plan subnets with VLSM, and generate switch configuration."""

import ipaddress


def smallest_prefix(hosts):
    """The longest IPv4 prefix whose usable addresses (2^host_bits - 2) hold `hosts`.
    2 -> 30, 30 -> 27, 31 -> 26. ValueError if not even a /1 is big enough."""
    for prefix in range(30, 0, -1):
        if 2 ** (32 - prefix) - 2 >= hosts:
            return prefix
    raise ValueError(f"{hosts} hosts don't fit in IPv4")


def vlsm(supernet, needs):
    """Allocate subnets for needs = [(name, hosts), ...] inside `supernet`.

    Largest first (equal sizes keep their order), one after another from the start of the supernet.
    Returns [(name, "a.b.c.d/prefix"), ...] in allocation order. ValueError if they don't fit."""
    net = ipaddress.ip_network(supernet)
    nxt = int(net.network_address)
    out = []
    for name, hosts in sorted(needs, key=lambda n: -n[1]):
        prefix = smallest_prefix(hosts)
        size = 2 ** (32 - prefix)
        nxt = (nxt + size - 1) // size * size
        sub = ipaddress.ip_network(f"{ipaddress.ip_address(nxt)}/{prefix}")
        if not sub.subnet_of(net):
            raise ValueError(f"{name} ({hosts} hosts) doesn't fit in {supernet}")
        out.append((name, str(sub)))
        nxt += size
    return out


def check_plan(plan):
    """Every problem in a plan, as a list of messages (empty when the plan is fine).

    plan = {"supernet": "10.20.0.0/16", "vlans": [{"id", "name", "subnet", "gateway",
            optional "dhcp": [start, end], optional "hosts_needed"}, ...]}

    Messages, checked in this order for each VLAN:
      VLAN 5000: id must be 1-4094
      VLAN 10: duplicate id
      VLAN 10: 10.20.10.5/24 is not a valid network address      (skip the VLAN's other checks)
      VLAN 10: subnet 10.30.0.0/24 is not inside 10.20.0.0/16
      VLAN 10 and VLAN 20: subnets overlap                         (earlier VLAN first)
      VLAN 10: gateway 10.20.11.1 is not a usable address in 10.20.10.0/24
      VLAN 10: DHCP range is not inside 10.20.10.0/24
      VLAN 10: DHCP range starts after it ends
      VLAN 10: gateway is inside the DHCP range
      VLAN 10: 10.20.10.0/27 has 30 usable addresses, 50 needed
    """
    problems = []
    supernet = ipaddress.ip_network(plan["supernet"])
    seen_ids = set()
    networks = []
    for v in plan["vlans"]:
        vid = v["id"]
        if not 1 <= vid <= 4094:
            problems.append(f"VLAN {vid}: id must be 1-4094")
        if vid in seen_ids:
            problems.append(f"VLAN {vid}: duplicate id")
        seen_ids.add(vid)
        try:
            net = ipaddress.ip_network(v["subnet"])
        except ValueError:
            problems.append(f"VLAN {vid}: {v['subnet']} is not a valid network address")
            continue
        if not net.subnet_of(supernet):
            problems.append(f"VLAN {vid}: subnet {net} is not inside {supernet}")
        for other_id, other in networks:
            if net.overlaps(other):
                problems.append(f"VLAN {other_id} and VLAN {vid}: subnets overlap")
        networks.append((vid, net))

        def usable(addr):
            return net.network_address < addr < net.broadcast_address

        gateway = ipaddress.ip_address(v["gateway"])
        if not usable(gateway):
            problems.append(f"VLAN {vid}: gateway {gateway} is not a usable address in {net}")
        if "dhcp" in v:
            start, end = (ipaddress.ip_address(a) for a in v["dhcp"])
            if not (usable(start) and usable(end)):
                problems.append(f"VLAN {vid}: DHCP range is not inside {net}")
            elif start > end:
                problems.append(f"VLAN {vid}: DHCP range starts after it ends")
            elif start <= gateway <= end:
                problems.append(f"VLAN {vid}: gateway is inside the DHCP range")
        capacity = net.num_addresses - 2
        needed = v.get("hosts_needed", 0)
        if needed > capacity:
            problems.append(f"VLAN {vid}: {net} has {capacity} usable addresses, {needed} needed")
    return problems


def switch_config(plan, dhcp_server=None):
    """Cisco-style configuration for a layer 3 switch: for each VLAN

        vlan 10
         name Staff
        !
        interface Vlan10
         description Staff
         ip address 10.20.10.1 255.255.255.0
         ip helper-address 10.20.99.10      (only if dhcp_server is given and the VLAN has "dhcp")
         no shutdown
        !

    Returns the text, ending with a newline."""
    lines = []
    for v in plan["vlans"]:
        net = ipaddress.ip_network(v["subnet"])
        lines += [f"vlan {v['id']}", f" name {v['name']}", "!"]
        lines += [f"interface Vlan{v['id']}", f" description {v['name']}", f" ip address {v['gateway']} {net.netmask}"]
        if dhcp_server and "dhcp" in v:
            lines.append(f" ip helper-address {dhcp_server}")
        lines += [" no shutdown", "!"]
    return "\n".join(lines) + "\n"
@@file plan.json
{
  "site": "Sakura Systems Osaka office",
  "supernet": "10.20.0.0/16",
  "dhcp_server": "10.20.99.10",
  "vlans": [
    {"id": 10, "name": "Staff", "subnet": "10.20.10.0/24", "gateway": "10.20.10.1", "dhcp": ["10.20.10.100", "10.20.10.250"], "hosts_needed": 120},
    {"id": 20, "name": "Voice", "subnet": "10.20.20.0/24", "gateway": "10.20.20.1", "dhcp": ["10.20.20.100", "10.20.20.250"], "hosts_needed": 100},
    {"id": 30, "name": "Printers", "subnet": "10.20.30.0/27", "gateway": "10.20.30.1", "hosts_needed": 15},
    {"id": 40, "name": "Servers", "subnet": "10.20.40.0/26", "gateway": "10.20.40.1", "hosts_needed": 20},
    {"id": 50, "name": "Guest", "subnet": "10.20.50.0/23", "gateway": "10.20.50.1", "dhcp": ["10.20.50.10", "10.20.51.250"], "hosts_needed": 300},
    {"id": 99, "name": "Management", "subnet": "10.20.99.0/28", "gateway": "10.20.99.1", "hosts_needed": 10}
  ]
}
@@file broken_plan.json
{
  "site": "A plan a colleague sent you for review. How many mistakes can you find before running the checker?",
  "supernet": "172.16.0.0/16",
  "dhcp_server": "172.16.99.10",
  "vlans": [
    {"id": 10, "name": "Staff", "subnet": "172.16.10.0/25", "gateway": "172.16.10.1", "dhcp": ["172.16.10.50", "172.16.10.200"], "hosts_needed": 150},
    {"id": 20, "name": "Voice", "subnet": "172.16.10.64/26", "gateway": "172.16.10.65", "hosts_needed": 40},
    {"id": 30, "name": "Printers", "subnet": "172.16.30.0/28", "gateway": "172.16.30.20", "hosts_needed": 10},
    {"id": 30, "name": "Cameras", "subnet": "172.16.31.0/24", "gateway": "172.16.31.1", "dhcp": ["172.16.31.1", "172.16.31.100"]},
    {"id": 50, "name": "Guest", "subnet": "192.168.50.0/24", "gateway": "192.168.50.1", "dhcp": ["192.168.50.200", "192.168.50.20"]},
    {"id": 4095, "name": "Management", "subnet": "172.16.99.0/28", "gateway": "172.16.99.1"}
  ]
}
@@file test_netplan.py
import json
import unittest
from pathlib import Path

from netplan import check_plan, smallest_prefix, switch_config, vlsm

HERE = Path(__file__).parent


def vlan(vid, subnet, gateway, **extra):
    return {"id": vid, "name": f"V{vid}", "subnet": subnet, "gateway": gateway, **extra}


class PrefixTests(unittest.TestCase):
    def test_sizes(self):
        for hosts, prefix in [(2, 30), (6, 29), (7, 28), (30, 27), (31, 26), (50, 26), (100, 25), (254, 24), (255, 23)]:
            self.assertEqual(smallest_prefix(hosts), prefix, hosts)


class VlsmTests(unittest.TestCase):
    def test_lesson_example_largest_first(self):
        needs = [("Printers", 20), ("LinkA", 2), ("Sales", 100), ("IT", 50), ("LinkB", 2)]
        self.assertEqual(vlsm("192.168.50.0/24", needs), [
            ("Sales", "192.168.50.0/25"),
            ("IT", "192.168.50.128/26"),
            ("Printers", "192.168.50.192/27"),
            ("LinkA", "192.168.50.224/30"),
            ("LinkB", "192.168.50.228/30"),
        ])

    def test_too_big(self):
        with self.assertRaises(ValueError):
            vlsm("10.0.0.0/28", [("A", 20)])
        with self.assertRaises(ValueError):
            vlsm("10.0.0.0/24", [("A", 120), ("B", 120), ("C", 10)])


class CheckTests(unittest.TestCase):
    def check(self, *vlans):
        return check_plan({"supernet": "10.20.0.0/16", "vlans": list(vlans)})

    def test_sample_plan_is_valid(self):
        plan = json.loads((HERE / "plan.json").read_text(encoding="utf-8"))
        self.assertEqual(check_plan(plan), [])

    def test_ids(self):
        self.assertEqual(self.check(vlan(5000, "10.20.1.0/24", "10.20.1.1"), vlan(10, "10.20.2.0/24", "10.20.2.1"),
                                    vlan(10, "10.20.3.0/24", "10.20.3.1")),
                         ["VLAN 5000: id must be 1-4094", "VLAN 10: duplicate id"])

    def test_bad_network_address(self):
        self.assertEqual(self.check(vlan(10, "10.20.10.5/24", "10.20.10.1")),
                         ["VLAN 10: 10.20.10.5/24 is not a valid network address"])

    def test_outside_and_overlap(self):
        self.assertEqual(self.check(vlan(10, "10.30.0.0/24", "10.30.0.1"), vlan(20, "10.20.0.0/23", "10.20.0.1"),
                                    vlan(30, "10.20.1.0/24", "10.20.1.1")),
                         ["VLAN 10: subnet 10.30.0.0/24 is not inside 10.20.0.0/16",
                          "VLAN 20 and VLAN 30: subnets overlap"])

    def test_gateway(self):
        self.assertEqual(self.check(vlan(10, "10.20.10.0/24", "10.20.11.1"), vlan(20, "10.20.20.0/24", "10.20.20.255")),
                         ["VLAN 10: gateway 10.20.11.1 is not a usable address in 10.20.10.0/24",
                          "VLAN 20: gateway 10.20.20.255 is not a usable address in 10.20.20.0/24"])

    def test_dhcp(self):
        self.assertEqual(self.check(
            vlan(10, "10.20.10.0/24", "10.20.10.1", dhcp=["10.20.10.100", "10.20.11.20"]),
            vlan(20, "10.20.20.0/24", "10.20.20.1", dhcp=["10.20.20.200", "10.20.20.100"]),
            vlan(30, "10.20.30.0/24", "10.20.30.1", dhcp=["10.20.30.1", "10.20.30.200"]),
            vlan(40, "10.20.40.0/24", "10.20.40.1", dhcp=["10.20.40.10", "10.20.40.200"]),
        ), ["VLAN 10: DHCP range is not inside 10.20.10.0/24",
            "VLAN 20: DHCP range starts after it ends",
            "VLAN 30: gateway is inside the DHCP range"])

    def test_capacity(self):
        self.assertEqual(self.check(vlan(10, "10.20.10.0/27", "10.20.10.1", hosts_needed=50),
                                    vlan(20, "10.20.20.0/26", "10.20.20.1", hosts_needed=62)),
                         ["VLAN 10: 10.20.10.0/27 has 30 usable addresses, 50 needed"])


class ConfigTests(unittest.TestCase):
    def test_config(self):
        plan = {"supernet": "10.20.0.0/16", "vlans": [
            {"id": 10, "name": "Staff", "subnet": "10.20.10.0/24", "gateway": "10.20.10.1", "dhcp": ["10.20.10.100", "10.20.10.250"]},
            {"id": 30, "name": "Printers", "subnet": "10.20.30.0/27", "gateway": "10.20.30.1"},
        ]}
        text = switch_config(plan, dhcp_server="10.20.99.10")
        self.assertTrue(text.startswith("vlan 10\n name Staff\n!\ninterface Vlan10\n description Staff\n"))
        self.assertIn(" ip address 10.20.10.1 255.255.255.0\n ip helper-address 10.20.99.10\n no shutdown\n!\n", text)
        self.assertIn("interface Vlan30\n description Printers\n ip address 10.20.30.1 255.255.255.224\n no shutdown\n", text)
        self.assertEqual(text.count("helper-address"), 1)
        self.assertTrue(text.endswith("!\n"))

    def test_no_helper_without_server(self):
        plan = {"supernet": "10.20.0.0/16", "vlans": [
            {"id": 10, "name": "Staff", "subnet": "10.20.10.0/24", "gateway": "10.20.10.1", "dhcp": ["10.20.10.100", "10.20.10.250"]}]}
        self.assertNotIn("helper", switch_config(plan))


if __name__ == "__main__":
    unittest.main()
@@file main.py
"""Network plan tools.

python main.py check plan.json
python main.py vlsm 192.168.50.0/24 Sales=100 IT=50 Printers=20 LinkA=2 LinkB=2
python main.py config plan.json > switch.txt
"""

import argparse
import json
import sys

from netplan import check_plan, switch_config, vlsm


def load(path):
    with open(path, encoding="utf-8") as f:
        return json.load(f)


def cmd_check(args):
    plan = load(args.plan)
    problems = check_plan(plan)
    for p in problems:
        print("  x", p)
    if problems:
        print(f"{len(problems)} problem(s) in {args.plan}")
        return 1
    print(f"OK {args.plan}: {len(plan['vlans'])} VLANs, no problems")
    return 0


def cmd_vlsm(args):
    needs = []
    for item in args.needs:
        name, _, hosts = item.partition("=")
        needs.append((name, int(hosts)))
    try:
        result = vlsm(args.supernet, needs)
    except ValueError as e:
        print("error:", e, file=sys.stderr)
        return 1
    print("| Name | Hosts | Subnet |")
    print("|---|---|---|")
    hosts = dict(needs)
    for name, subnet in result:
        print(f"| {name} | {hosts[name]} | {subnet} |")
    return 0


def cmd_config(args):
    plan = load(args.plan)
    problems = check_plan(plan)
    if problems:
        print("Fix the plan first: python main.py check", args.plan, file=sys.stderr)
        return 1
    sys.stdout.write(switch_config(plan, plan.get("dhcp_server")))
    return 0


def main(argv=None):
    parser = argparse.ArgumentParser(description="Office network planning tools")
    sub = parser.add_subparsers(required=True)
    p = sub.add_parser("check", help="find problems in a plan")
    p.add_argument("plan")
    p.set_defaults(func=cmd_check)
    p = sub.add_parser("vlsm", help="plan subnets: NAME=HOSTS ...")
    p.add_argument("supernet")
    p.add_argument("needs", nargs="+")
    p.set_defaults(func=cmd_vlsm)
    p = sub.add_parser("config", help="print switch configuration for a plan")
    p.add_argument("plan")
    p.set_defaults(func=cmd_config)
    args = parser.parse_args(argv)
    return args.func(args)


if __name__ == "__main__":
    sys.exit(main())

@@drill
How many usable hosts does a /27 subnet have?
= 30
? 32 addresses minus network and broadcast.

@@text
🎉 You now plan networks the way network engineers do: on paper first, checked by a machine, then configuration generated from the same data so it can't drift from the documentation. That idea is the heart of **network automation** (Ansible, Netmiko, Terraform), one of the best-paid directions in networking.
