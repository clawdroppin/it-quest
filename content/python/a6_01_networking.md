@@lesson a6-networking Networking: IP Addresses, Subnets and Sockets
@@concepts networking
@@text
Network knowledge plus Python is one of the most employable combinations in IT. Python's standard library covers the basics without installing anything.

## ipaddress: subnetting without the headache

```python
import ipaddress

net = ipaddress.ip_network("192.168.10.0/26")
print(net.network_address, net.broadcast_address)
print(net.netmask, net.num_addresses)
hosts = list(net.hosts())
print(hosts[0], "...", hosts[-1], f"({len(hosts)} usable)")
print(ipaddress.ip_address("192.168.10.70") in net)
```

It does the binary math for you: network and broadcast addresses, the mask, the usable host range, and membership checks.

```python
import ipaddress

for text in ["10.1.2.3", "172.16.5.4", "192.168.1.1", "8.8.8.8", "127.0.0.1", "fe80::1"]:
    ip = ipaddress.ip_address(text)
    print(f"{text:<12} private={ip.is_private!s:<5} loopback={ip.is_loopback!s:<5} v{ip.version}")
```

Splitting a network into smaller subnets:

```python
import ipaddress

office = ipaddress.ip_network("10.20.0.0/22")
for sub in office.subnets(new_prefix=24):
    print(sub)
```

## Sockets: the bottom of everything

Every network program (browsers, SSH, databases) talks through **sockets**. A TCP socket connects to an IP and port. This example starts a tiny server on your own machine and connects to it:

```python
import socket, threading

server = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
server.bind(("127.0.0.1", 0))          # port 0: let the OS pick a free port
server.listen()
port = server.getsockname()[1]

def serve_once():
    conn, addr = server.accept()
    with conn:
        data = conn.recv(1024)
        conn.sendall(b"ECHO: " + data)

threading.Thread(target=serve_once).start()

with socket.create_connection(("127.0.0.1", port), timeout=3) as client:
    client.sendall(b"hello")
    print(client.recv(1024).decode())
server.close()
```

## Checking if a port is open

The core of every port scanner and "is the service up?" check:

```python
import socket

def port_open(host, port, timeout=0.5):
    try:
        with socket.create_connection((host, port), timeout=timeout):
            return True
    except OSError:          # refused, timed out, unreachable...
        return False

print(port_open("127.0.0.1", 1))
```

> Only scan machines you own or have **written permission** to test. Scanning other people's networks can be illegal, and it's a firing offense at most companies.

## DNS

```python
import socket
print(socket.gethostbyname("localhost"))
```

`gethostbyname` asks the system's DNS resolver for an IPv4 address. `socket.getaddrinfo` handles IPv6 too.

@@quiz
How many usable host addresses does a /26 network have?
- 64
* 62
- 32
- 30
? 2^(32−26) = 64 addresses, minus the network and broadcast addresses = 62.

@@quiz
Which of these is a private (RFC 1918) address?
- 8.8.8.8
- 1.1.1.1
* 172.20.4.9
- 100.200.1.1
? The private ranges are 10.0.0.0/8, 172.16.0.0/12 (172.16–172.31) and 192.168.0.0/16.

@@quiz
What does `ipaddress.ip_address("10.0.0.5") in ipaddress.ip_network("10.0.0.0/24")` return?
* True
- False
- An error
- "10.0.0.0/24"
? The in operator checks whether an address belongs to a network.

@@flashcards
10.0.0.0/8, 172.16.0.0/12, 192.168.0.0/16 :: the private (RFC 1918) IPv4 ranges
127.0.0.1 :: loopback: this computer itself
usable hosts in a /n network :: 2^(32−n) − 2
`ipaddress.ip_network("10.0.0.0/24")` :: a network object: hosts(), netmask, broadcast_address
`socket.create_connection((host, port), timeout=1)` :: open a TCP connection (raises OSError if closed)

@@exercise Subnet summary
Write `summary(cidr)` that returns a dict with the `"network"`, `"broadcast"`, `"netmask"` (as strings), `"first"` and `"last"` usable host (strings), and `"usable"` (int), for a network given in CIDR notation.
@@starter
import ipaddress


def summary(cidr):
    pass


print(summary("192.168.10.64/27"))
@@tests
check(summary("192.168.10.64/27"), {"network": "192.168.10.64", "broadcast": "192.168.10.95", "netmask": "255.255.255.224", "first": "192.168.10.65", "last": "192.168.10.94", "usable": 30}, "summary('192.168.10.64/27')")
check(summary("10.0.0.0/30")["usable"], 2, "a /30 has 2 usable hosts")
@@hint
`net = ipaddress.ip_network(cidr)`; `hosts = list(net.hosts())`; use `str(...)` to turn addresses into strings.
@@solution
import ipaddress


def summary(cidr):
    net = ipaddress.ip_network(cidr)
    hosts = list(net.hosts())
    return {
        "network": str(net.network_address),
        "broadcast": str(net.broadcast_address),
        "netmask": str(net.netmask),
        "first": str(hosts[0]),
        "last": str(hosts[-1]),
        "usable": len(hosts),
    }


print(summary("192.168.10.64/27"))

@@exercise Which subnet?
A company has these subnets: `10.0.1.0/24` is `"office"`, `10.0.2.0/24` is `"servers"`, `10.0.8.0/21` is `"wifi"`. Write `which_subnet(ip)` that returns the name of the subnet containing `ip`, or `"external"` if none does, or `"invalid"` if `ip` isn't a valid IP address.
@@starter
import ipaddress

SUBNETS = {"office": "10.0.1.0/24", "servers": "10.0.2.0/24", "wifi": "10.0.8.0/21"}


def which_subnet(ip):
    pass


print(which_subnet("10.0.13.200"))
@@tests
check(which_subnet("10.0.1.17"), "office", "10.0.1.17")
check(which_subnet("10.0.2.250"), "servers", "10.0.2.250")
check(which_subnet("10.0.13.200"), "wifi", "10.0.13.200 (inside 10.0.8.0/21)")
check(which_subnet("8.8.8.8"), "external", "8.8.8.8")
check(which_subnet("10.0.300.1"), "invalid", "10.0.300.1")
@@hint
`try: addr = ipaddress.ip_address(ip)` / `except ValueError: return "invalid"`. Then loop over `SUBNETS.items()` and test `addr in ipaddress.ip_network(cidr)`.
@@solution
import ipaddress

SUBNETS = {"office": "10.0.1.0/24", "servers": "10.0.2.0/24", "wifi": "10.0.8.0/21"}


def which_subnet(ip):
    try:
        addr = ipaddress.ip_address(ip)
    except ValueError:
        return "invalid"
    for name, cidr in SUBNETS.items():
        if addr in ipaddress.ip_network(cidr):
            return name
    return "external"


print(which_subnet("10.0.13.200"))

@@exercise Port checker
Write `scan(host, ports)` that returns a sorted list of the ports in `ports` that are **open** on `host`, using a short timeout. The check starts a few local servers for you to find.
@@starter
import socket


def scan(host, ports):
    pass
@@tests
import socket as _so
_servers = []
for _ in range(2):
    _s = _so.socket()
    _s.bind(("127.0.0.1", 0))
    _s.listen()
    _servers.append(_s)
_open = sorted(s.getsockname()[1] for s in _servers)
_closed = _so.socket()
_closed.bind(("127.0.0.1", 0))
_closed_port = _closed.getsockname()[1]
_closed.close()
check(scan("127.0.0.1", _open + [_closed_port]), _open, "scan finds the two open ports and not the closed one")
@@hint
For each port, try `socket.create_connection((host, port), timeout=0.5)` inside `try`/`except OSError`.
@@solution
import socket


def scan(host, ports):
    found = []
    for port in ports:
        try:
            with socket.create_connection((host, port), timeout=0.5):
                found.append(port)
        except OSError:
            pass
    return sorted(found)
