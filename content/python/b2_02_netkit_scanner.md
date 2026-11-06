@@lesson b2-netkit-scanner Network Toolkit 2: Port Scanner and CLI
@@text
Now the **port scanner**: which TCP ports on a host accept connections? Admins use this to check that a firewall is doing its job and that no forgotten service (an old FTP, an open database port) is exposed. Attackers use it for the same reason, which is why you only scan machines you own or are allowed to test. The tests only scan your own computer.

A connect scan tries a full TCP connection to each port. Most ports don't answer, so each try waits for the timeout: scanning 1000 ports one by one at 0.5 s would take 8 minutes. With a **thread pool** of 100 workers, it takes seconds.

You'll also write `parse_ports`, so users can type ranges like `22,80,8000-8010`, and `main.py` ties everything together with **subcommands** (`argparse` sub-parsers, like `git commit` / `git push`).

@@project netkit Network Toolkit, step 2: scanner and CLI
Write `scan.py` until `test_scan.py` passes, then try `python main.py subnet 192.168.1.0/26` and `python main.py scan 127.0.0.1`.
@@file scan.py
"""A TCP connect port scanner. Only scan hosts you own or have permission to test."""

import socket
from concurrent.futures import ThreadPoolExecutor

COMMON_PORTS = {
    21: "ftp", 22: "ssh", 23: "telnet", 25: "smtp", 53: "dns", 80: "http", 110: "pop3",
    135: "msrpc", 139: "netbios", 143: "imap", 443: "https", 445: "smb", 993: "imaps",
    1433: "mssql", 3306: "mysql", 3389: "rdp", 5432: "postgresql", 5900: "vnc", 8080: "http-alt",
}


def parse_ports(spec):
    """"22,80,8000-8002" -> [22, 80, 8000, 8001, 8002]: sorted, no duplicates.
    Raise ValueError for anything that isn't a valid port or range (1-65535, start <= end)."""
    raise NotImplementedError("TODO: parse_ports")


def service_name(port):
    """The usual service on this port ("ssh"), or "unknown"."""
    raise NotImplementedError("TODO: service_name")


def port_open(host, port, timeout=0.5):
    """True if a TCP connection to host:port succeeds within `timeout` seconds."""
    raise NotImplementedError("TODO: port_open")


def scan(host, ports, workers=100, timeout=0.5):
    """The open ports among `ports`, sorted. Checks them in parallel with a thread pool."""
    raise NotImplementedError("TODO: scan")
@@file test_scan.py
import socket
import unittest

from scan import parse_ports, port_open, scan, service_name


class ParsePortsTests(unittest.TestCase):
    def test_list_and_ranges(self):
        self.assertEqual(parse_ports("80,22,8000-8002,22"), [22, 80, 8000, 8001, 8002])

    def test_spaces(self):
        self.assertEqual(parse_ports(" 443 , 1-2 "), [1, 2, 443])

    def test_invalid(self):
        for bad in ["", "abc", "0", "65536", "90-80", "1-2-3"]:
            with self.assertRaises(ValueError, msg=bad):
                parse_ports(bad)


class ScanTests(unittest.TestCase):
    def setUp(self):
        self.servers = []
        for _ in range(2):
            s = socket.socket()
            s.bind(("127.0.0.1", 0))
            s.listen()
            self.servers.append(s)
        self.open_ports = sorted(s.getsockname()[1] for s in self.servers)
        closed = socket.socket()
        closed.bind(("127.0.0.1", 0))
        self.closed_port = closed.getsockname()[1]
        closed.close()

    def tearDown(self):
        for s in self.servers:
            s.close()

    def test_port_open(self):
        self.assertTrue(port_open("127.0.0.1", self.open_ports[0]))
        self.assertFalse(port_open("127.0.0.1", self.closed_port))

    def test_scan(self):
        self.assertEqual(scan("127.0.0.1", [self.closed_port] + self.open_ports), self.open_ports)

    def test_service_name(self):
        self.assertEqual(service_name(22), "ssh")
        self.assertEqual(service_name(3389), "rdp")
        self.assertEqual(service_name(12345), "unknown")


if __name__ == "__main__":
    unittest.main()
@@file main.py
"""netkit: subnet calculator, VLSM planner and port scanner.

    python main.py subnet 192.168.10.64/27
    python main.py split 10.20.0.0/22 24
    python main.py vlsm 10.0.0.0/24 sales=50 it=20 guests=100
    python main.py scan 127.0.0.1 --ports 22,80,443,8000-8010
"""

import argparse
import sys

from scan import COMMON_PORTS, parse_ports, scan, service_name
from subnet import split, summary, vlsm


def cmd_subnet(args):
    for key, value in summary(args.cidr).items():
        print(f"{key:<10} {value}")


def cmd_split(args):
    for net in split(args.cidr, args.prefix):
        print(net)


def cmd_vlsm(args):
    needs = {}
    for item in args.needs:
        name, _, hosts = item.partition("=")
        needs[name] = int(hosts)
    for name, net in vlsm(args.cidr, needs):
        info = summary(net)
        print(f"{name:<12} {net:<18} {info['first']} - {info['last']} ({info['usable']} usable, needs {needs[name]})")


def cmd_scan(args):
    ports = parse_ports(args.ports) if args.ports else sorted(COMMON_PORTS)
    print(f"Scanning {len(ports)} port(s) on {args.host}...")
    found = scan(args.host, ports, timeout=args.timeout)
    for port in found:
        print(f"{port:>5}/tcp open  {service_name(port)}")
    print(f"{len(found)} open")


def main(argv=None):
    parser = argparse.ArgumentParser(description="Subnet calculator, VLSM planner and port scanner.")
    sub = parser.add_subparsers(dest="command", required=True)

    p = sub.add_parser("subnet", help="describe a network")
    p.add_argument("cidr")
    p.set_defaults(func=cmd_subnet)

    p = sub.add_parser("split", help="split a network into smaller ones")
    p.add_argument("cidr")
    p.add_argument("prefix", type=int)
    p.set_defaults(func=cmd_split)

    p = sub.add_parser("vlsm", help="plan subnets for departments: name=hosts ...")
    p.add_argument("cidr")
    p.add_argument("needs", nargs="+")
    p.set_defaults(func=cmd_vlsm)

    p = sub.add_parser("scan", help="scan TCP ports (only hosts you're allowed to scan!)")
    p.add_argument("host")
    p.add_argument("--ports", help="e.g. 22,80,8000-8010 (default: common ports)")
    p.add_argument("--timeout", type=float, default=0.5)
    p.set_defaults(func=cmd_scan)

    args = parser.parse_args(argv)
    try:
        args.func(args)
    except ValueError as e:
        print(f"error: {e}", file=sys.stderr)
        return 2
    return 0


if __name__ == "__main__":
    sys.exit(main())
@@project netkit_finished Network Toolkit: the finished version
The complete project with every test passing. Ideas to extend it: a **ping sweep** (`subprocess` running `ping -n 1` on every host of a subnet, in a thread pool), **banner grabbing** (read the first line a service sends, like SSH's version), or saving scan results as JSON to compare over time.
@@file subnet.py
"""Subnet math: summaries, splitting, and VLSM planning. IPv4 only."""

import ipaddress


def summary(cidr):
    """A dict describing a network given as "a.b.c.d/n":
    network, broadcast, netmask, first, last (all strings), usable (int).
    For /31 and /32, first and last are the network's own addresses and usable is 2 or 1."""
    net = ipaddress.ip_network(cidr, strict=False)
    hosts = list(net.hosts()) or [net.network_address]
    return {
        "network": str(net.network_address),
        "broadcast": str(net.broadcast_address),
        "netmask": str(net.netmask),
        "first": str(hosts[0]),
        "last": str(hosts[-1]),
        "usable": len(hosts),
    }


def split(cidr, new_prefix):
    """The subnets of `cidr` with prefix length `new_prefix`, as strings, in order.
    Raise ValueError if new_prefix is smaller than the network's prefix."""
    net = ipaddress.ip_network(cidr, strict=False)
    return [str(s) for s in net.subnets(new_prefix=new_prefix)]


def prefix_for_hosts(n):
    """The largest prefix length (the smallest network) with at least n usable hosts.
    1–2 hosts -> 30, 3–6 -> 29, 7–14 -> 28, ... Raise ValueError if n < 1."""
    if n < 1:
        raise ValueError("need at least one host")
    for prefix in range(30, 0, -1):
        if 2 ** (32 - prefix) - 2 >= n:
            return prefix
    raise ValueError(f"{n} hosts don't fit in IPv4")


def vlsm(cidr, needs):
    """Plan subnets inside `cidr` for `needs`, a dict {name: hosts needed}.

    Biggest need first (ties: by name). Each gets the smallest subnet that fits,
    placed at the next free address that's a multiple of its size.
    Returns a list of (name, "a.b.c.d/n") in that order.
    Raise ValueError if they don't fit inside cidr."""
    block = ipaddress.ip_network(cidr, strict=False)
    cursor = int(block.network_address)
    plan = []
    for name, hosts in sorted(needs.items(), key=lambda kv: (-kv[1], kv[0])):
        prefix = prefix_for_hosts(hosts)
        size = 2 ** (32 - prefix)
        cursor = (cursor + size - 1) // size * size  # round up to a multiple of size
        net = ipaddress.ip_network((cursor, prefix))
        if not net.subnet_of(block):
            raise ValueError(f"{name} ({hosts} hosts) doesn't fit in {block}")
        plan.append((name, str(net)))
        cursor += size
    return plan
@@file scan.py
"""A TCP connect port scanner. Only scan hosts you own or have permission to test."""

import socket
from concurrent.futures import ThreadPoolExecutor

COMMON_PORTS = {
    21: "ftp", 22: "ssh", 23: "telnet", 25: "smtp", 53: "dns", 80: "http", 110: "pop3",
    135: "msrpc", 139: "netbios", 143: "imap", 443: "https", 445: "smb", 993: "imaps",
    1433: "mssql", 3306: "mysql", 3389: "rdp", 5432: "postgresql", 5900: "vnc", 8080: "http-alt",
}


def _port(text):
    port = int(text)
    if not 1 <= port <= 65535:
        raise ValueError(f"port {port} is out of range 1-65535")
    return port


def parse_ports(spec):
    """"22,80,8000-8002" -> [22, 80, 8000, 8001, 8002]: sorted, no duplicates.
    Raise ValueError for anything that isn't a valid port or range (1-65535, start <= end)."""
    ports = set()
    for part in spec.split(","):
        part = part.strip()
        if "-" in part:
            start, end = part.split("-")  # more than one "-" raises ValueError here
            start, end = _port(start), _port(end)
            if start > end:
                raise ValueError(f"bad range {part}")
            ports.update(range(start, end + 1))
        else:
            ports.add(_port(part))  # int("") raises ValueError
    return sorted(ports)


def service_name(port):
    """The usual service on this port ("ssh"), or "unknown"."""
    return COMMON_PORTS.get(port, "unknown")


def port_open(host, port, timeout=0.5):
    """True if a TCP connection to host:port succeeds within `timeout` seconds."""
    try:
        with socket.create_connection((host, port), timeout=timeout):
            return True
    except OSError:
        return False


def scan(host, ports, workers=100, timeout=0.5):
    """The open ports among `ports`, sorted. Checks them in parallel with a thread pool."""
    with ThreadPoolExecutor(max_workers=workers) as pool:
        results = pool.map(lambda p: (p, port_open(host, p, timeout)), ports)
        return sorted(p for p, is_open in results if is_open)
@@file main.py
"""netkit: subnet calculator, VLSM planner and port scanner.

    python main.py subnet 192.168.10.64/27
    python main.py split 10.20.0.0/22 24
    python main.py vlsm 10.0.0.0/24 sales=50 it=20 guests=100
    python main.py scan 127.0.0.1 --ports 22,80,443,8000-8010
"""

import argparse
import sys

from scan import COMMON_PORTS, parse_ports, scan, service_name
from subnet import split, summary, vlsm


def cmd_subnet(args):
    for key, value in summary(args.cidr).items():
        print(f"{key:<10} {value}")


def cmd_split(args):
    for net in split(args.cidr, args.prefix):
        print(net)


def cmd_vlsm(args):
    needs = {}
    for item in args.needs:
        name, _, hosts = item.partition("=")
        needs[name] = int(hosts)
    for name, net in vlsm(args.cidr, needs):
        info = summary(net)
        print(f"{name:<12} {net:<18} {info['first']} - {info['last']} ({info['usable']} usable, needs {needs[name]})")


def cmd_scan(args):
    ports = parse_ports(args.ports) if args.ports else sorted(COMMON_PORTS)
    print(f"Scanning {len(ports)} port(s) on {args.host}...")
    found = scan(args.host, ports, timeout=args.timeout)
    for port in found:
        print(f"{port:>5}/tcp open  {service_name(port)}")
    print(f"{len(found)} open")


def main(argv=None):
    parser = argparse.ArgumentParser(description="Subnet calculator, VLSM planner and port scanner.")
    sub = parser.add_subparsers(dest="command", required=True)

    p = sub.add_parser("subnet", help="describe a network")
    p.add_argument("cidr")
    p.set_defaults(func=cmd_subnet)

    p = sub.add_parser("split", help="split a network into smaller ones")
    p.add_argument("cidr")
    p.add_argument("prefix", type=int)
    p.set_defaults(func=cmd_split)

    p = sub.add_parser("vlsm", help="plan subnets for departments: name=hosts ...")
    p.add_argument("cidr")
    p.add_argument("needs", nargs="+")
    p.set_defaults(func=cmd_vlsm)

    p = sub.add_parser("scan", help="scan TCP ports (only hosts you're allowed to scan!)")
    p.add_argument("host")
    p.add_argument("--ports", help="e.g. 22,80,8000-8010 (default: common ports)")
    p.add_argument("--timeout", type=float, default=0.5)
    p.set_defaults(func=cmd_scan)

    args = parser.parse_args(argv)
    try:
        args.func(args)
    except ValueError as e:
        print(f"error: {e}", file=sys.stderr)
        return 2
    return 0


if __name__ == "__main__":
    sys.exit(main())
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
@@file test_scan.py
import socket
import unittest

from scan import parse_ports, port_open, scan, service_name


class ParsePortsTests(unittest.TestCase):
    def test_list_and_ranges(self):
        self.assertEqual(parse_ports("80,22,8000-8002,22"), [22, 80, 8000, 8001, 8002])

    def test_spaces(self):
        self.assertEqual(parse_ports(" 443 , 1-2 "), [1, 2, 443])

    def test_invalid(self):
        for bad in ["", "abc", "0", "65536", "90-80", "1-2-3"]:
            with self.assertRaises(ValueError, msg=bad):
                parse_ports(bad)


class ScanTests(unittest.TestCase):
    def setUp(self):
        self.servers = []
        for _ in range(2):
            s = socket.socket()
            s.bind(("127.0.0.1", 0))
            s.listen()
            self.servers.append(s)
        self.open_ports = sorted(s.getsockname()[1] for s in self.servers)
        closed = socket.socket()
        closed.bind(("127.0.0.1", 0))
        self.closed_port = closed.getsockname()[1]
        closed.close()

    def tearDown(self):
        for s in self.servers:
            s.close()

    def test_port_open(self):
        self.assertTrue(port_open("127.0.0.1", self.open_ports[0]))
        self.assertFalse(port_open("127.0.0.1", self.closed_port))

    def test_scan(self):
        self.assertEqual(scan("127.0.0.1", [self.closed_port] + self.open_ports), self.open_ports)

    def test_service_name(self):
        self.assertEqual(service_name(22), "ssh")
        self.assertEqual(service_name(3389), "rdp")
        self.assertEqual(service_name(12345), "unknown")


if __name__ == "__main__":
    unittest.main()

@@drill
Which socket method returns an error code instead of raising when a connection fails?
= connect_ex
= connect_ex()
= socket.connect_ex
? 0 means the port accepted the connection.

@@text
🎉 You now own a toolkit you'll actually use. Keep it in your GitHub portfolio, and keep using `python main.py vlsm ...` to check your answers when you practice subnetting for Network+ and CCNA.
