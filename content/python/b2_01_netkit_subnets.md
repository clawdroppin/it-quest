@@lesson b2-netkit-subnets Network Toolkit 1: The Subnet Calculator
@@text
Network engineers and sysadmins reach for the same small tools every day: a **subnet calculator**, a **port scanner**, a way to plan addresses for a new office. In this build you'll write **netkit**, one command-line tool that does all of it:

```text
python main.py subnet 192.168.10.64/27
python main.py split 10.20.0.0/22 24
python main.py vlsm 10.0.0.0/24 sales=50 it=20 guests=100 printers=5
python main.py scan 127.0.0.1 --ports 22,80,443,8000-8010
```

The subnetting part is pure math, so it's perfect for tests. It's also the single most feared topic on **Network+** and **CCNA**, and writing the code is one of the best ways to really understand it.

## VLSM in one paragraph

**Variable Length Subnet Masking**: you're given one block (say `10.0.0.0/24`) and a list of departments with how many hosts each needs. You give each one the *smallest* subnet that fits, starting with the **largest** need, placing them one after another. Each subnet must start on a multiple of its own size (a /26 holds 64 addresses, so it starts at .0, .64, .128 or .192).

| Need | Smallest fit | Size |
|---|---|---|
| 100 hosts | /25 (126 usable) | 128 |
| 50 hosts | /26 (62 usable) | 64 |
| 20 hosts | /27 (30 usable) | 32 |
| 5 hosts | /29 (6 usable) | 8 |

The `ipaddress` module from the Networking lesson does the heavy lifting.

@@project netkit Network Toolkit, step 1: subnets
Write the four functions in `subnet.py` until `test_subnet.py` passes (**🧪 run tests**).
@@file subnet.py
"""Subnet math: summaries, splitting, and VLSM planning. IPv4 only."""

import ipaddress


def summary(cidr):
    """A dict describing a network given as "a.b.c.d/n":
    network, broadcast, netmask, first, last (all strings), usable (int).
    For /31 and /32, first and last are the network's own addresses and usable is 2 or 1."""
    raise NotImplementedError("TODO: summary")


def split(cidr, new_prefix):
    """The subnets of `cidr` with prefix length `new_prefix`, as strings, in order.
    Raise ValueError if new_prefix is smaller than the network's prefix."""
    raise NotImplementedError("TODO: split")


def prefix_for_hosts(n):
    """The largest prefix length (the smallest network) with at least n usable hosts.
    1–2 hosts -> 30, 3–6 -> 29, 7–14 -> 28, ... Raise ValueError if n < 1."""
    raise NotImplementedError("TODO: prefix_for_hosts")


def vlsm(cidr, needs):
    """Plan subnets inside `cidr` for `needs`, a dict {name: hosts needed}.

    Biggest need first (ties: by name). Each gets the smallest subnet that fits,
    placed at the next free address that's a multiple of its size.
    Returns a list of (name, "a.b.c.d/n") in that order.
    Raise ValueError if they don't fit inside cidr."""
    raise NotImplementedError("TODO: vlsm")
@@file test_subnet.py
import unittest

from subnet import prefix_for_hosts, split, summary, vlsm


class SummaryTests(unittest.TestCase):
    def test_slash27(self):
        self.assertEqual(
            summary("192.168.10.64/27"),
            {"network": "192.168.10.64", "broadcast": "192.168.10.95", "netmask": "255.255.255.224",
             "first": "192.168.10.65", "last": "192.168.10.94", "usable": 30},
        )

    def test_host_bits_set(self):
        # People often type an address inside the network; accept it.
        self.assertEqual(summary("10.1.2.3/24")["network"], "10.1.2.0")

    def test_point_to_point(self):
        self.assertEqual(summary("10.0.0.0/31")["usable"], 2)


class SplitTests(unittest.TestCase):
    def test_split(self):
        self.assertEqual(split("10.20.0.0/22", 24), ["10.20.0.0/24", "10.20.1.0/24", "10.20.2.0/24", "10.20.3.0/24"])

    def test_bad_prefix(self):
        with self.assertRaises(ValueError):
            split("10.0.0.0/24", 16)


class PlanningTests(unittest.TestCase):
    def test_prefix_for_hosts(self):
        self.assertEqual([prefix_for_hosts(n) for n in (1, 2, 3, 6, 7, 30, 31, 100, 254, 255)], [30, 30, 29, 29, 28, 27, 26, 25, 24, 23])
        with self.assertRaises(ValueError):
            prefix_for_hosts(0)

    def test_vlsm(self):
        plan = vlsm("10.0.0.0/24", {"sales": 50, "it": 20, "guests": 100, "printers": 5})
        self.assertEqual(plan, [("guests", "10.0.0.0/25"), ("sales", "10.0.0.128/26"), ("it", "10.0.0.192/27"), ("printers", "10.0.0.224/29")])

    def test_vlsm_too_big(self):
        with self.assertRaises(ValueError):
            vlsm("192.168.1.0/26", {"a": 50, "b": 50})


if __name__ == "__main__":
    unittest.main()

@@drill
What is the subnet mask for a /24 network?
= 255.255.255.0
? 24 one-bits: three full octets.

@@text
> Tip: `ipaddress.ip_network(cidr, strict=False)` accepts addresses with host bits set, like `10.1.2.3/24`. `ip_network((int_address, prefix))` builds a network from a number, handy for VLSM: convert the starting address with `int(...)`, round it **up** to a multiple of the block size, and move along.
