@@lesson c2-security-deep Security Fundamentals: ACLs, Layer 2 Protections, AAA and VPNs
@@text
Security fundamentals is 15% of CCNA. You need to write and place ACLs correctly, configure port security, DHCP snooping and Dynamic ARP Inspection, explain AAA with RADIUS and TACACS+, secure device access, and describe VPNs and wireless security. All of it is defensive: these are the controls you'll be asked to deploy at work.

## Security concepts

**Threat** (potential danger), **vulnerability** (weakness), **exploit** (a way to use it), **mitigation** (a control). Programme elements: user awareness, training, physical access control. Password policy: length, MFA, no reuse; certificates and PKI for device identity.

## ACLs: rules you must know

- Processed **top-down**, **first match wins**, and every ACL ends with an **implicit deny any**.
- **Standard** (1–99, 1300–1999): match **source** only → place **close to the destination**.
- **Extended** (100–199, 2000–2699): match protocol, source, destination and ports → place **close to the source**.
- One ACL **per interface, per direction, per protocol**.
- **Named** ACLs allow editing by sequence number.

```text
ip access-list extended WEB-ONLY
 10 permit tcp 192.168.10.0 0.0.0.255 host 10.0.50.10 eq 443
 20 permit udp 192.168.10.0 0.0.0.255 host 10.0.50.53 eq domain
 30 deny   ip 192.168.10.0 0.0.0.255 10.0.50.0 0.0.0.255 log
 40 permit ip any any
interface g0/0.10
 ip access-group WEB-ONLY in
ip access-list standard VTY-MGMT
 permit 10.0.99.0 0.0.0.255
line vty 0 15
 access-class VTY-MGMT in
```

`show access-lists` shows hit counts per line: a great troubleshooting aid. Common errors: wrong direction (`in` vs `out`), wrong placement of a standard ACL, a broad deny above a specific permit, forgetting the implicit deny blocks routing protocols.

## Layer 2 security

**Port security** limits the MAC addresses on an access port:

```text
interface g1/0/5
 switchport mode access
 switchport port-security
 switchport port-security maximum 2
 switchport port-security mac-address sticky
 switchport port-security violation restrict
```

| Violation mode | Drops traffic | Logs/counts | Shuts port |
|---|---|---|---|
| **protect** | yes | no | no |
| **restrict** | yes | yes | no |
| **shutdown** (default) | yes | yes | yes (err-disabled) |

**DHCP snooping** stops rogue DHCP servers: ports are **untrusted** by default (server messages like OFFER/ACK are dropped); mark the uplink to the real server `ip dhcp snooping trust`. It builds a **binding table** (MAC, IP, VLAN, port) and can rate-limit DHCP messages.

**Dynamic ARP Inspection (DAI)** uses that binding table to drop ARP replies whose IP–MAC pair doesn't match, stopping ARP spoofing/man-in-the-middle. Trust uplinks: `ip arp inspection trust`.

```text
ip dhcp snooping
ip dhcp snooping vlan 10,20
no ip dhcp snooping information option
ip arp inspection vlan 10,20
interface g1/0/48
 ip dhcp snooping trust
 ip arp inspection trust
```

Other layer 2 hardening: unused ports shut down and in a dead VLAN, `switchport nonegotiate` (no DTP), native VLAN not 1 and unused, BPDU Guard on access ports.

## Device access

```text
enable secret <strong-secret>        ! hashed; never "enable password"
service password-encryption          ! weak type 7 obfuscation of other passwords
username admin privilege 15 algorithm-type scrypt secret <strong-secret>
ip domain-name example.local
crypto key generate rsa modulus 2048
ip ssh version 2
line vty 0 15
 transport input ssh
 login local
 exec-timeout 10
login block-for 120 attempts 5 within 60
banner motd ^Authorised access only^
```

## AAA

**Authentication** (who are you), **Authorisation** (what may you do), **Accounting** (what did you do).

| | RADIUS | TACACS+ |
|---|---|---|
| Origin | open standard | Cisco |
| Transport | UDP 1812/1813 | TCP 49 |
| Encryption | password only | whole payload |
| AuthN/AuthZ | combined | separate (per-command authorisation) |
| Typical use | network access (802.1X, VPN, Wi-Fi) | device administration |

**802.1X**: supplicant (client) → authenticator (switch/AP) → authentication server (RADIUS). EAP carries the credentials.

## VPNs

- **Site-to-site IPsec**: gateways tunnel traffic between sites; users don't notice. ESP provides confidentiality and integrity; IKE negotiates keys.
- **Remote access**: users run a client (TLS/SSL or IPsec) to reach the company network.
- **GRE over IPsec**: GRE carries multicast/routing protocols; IPsec encrypts it.

## Wireless security

WPA2-Personal (PSK, AES-CCMP), WPA2-Enterprise (802.1X/RADIUS), **WPA3** (SAE replaces PSK, protecting against offline dictionary attacks; 192-bit mode for Enterprise; Protected Management Frames required). WEP and TKIP are broken.

@@quiz
Where should a standard ACL be placed?
- Close to the source
* Close to the destination
- On every interface
- Only on VTY lines
? It matches only the source, so placing it near the source could block too much.

@@quiz
An ACL has `permit tcp any host 10.0.50.10 eq 443` and nothing else. What happens to SSH traffic to 10.0.50.10?
- Permitted
* Denied by the implicit deny
- Logged and permitted
- Depends on the direction
? Every ACL ends with deny any.

@@quiz
Which port security violation mode drops traffic and logs it but keeps the port up?
- protect
* restrict
- shutdown
- sticky
? protect drops silently; shutdown err-disables.

@@quiz
A rogue DHCP server on an access port is handing out addresses. Which feature stops it?
* DHCP snooping
- Port security
- BPDU Guard
- PortFast
? Untrusted ports can't send OFFERs.

@@quiz
Which AAA protocol encrypts the whole payload and supports per-command authorisation?
- RADIUS
* TACACS+
- LDAP
- Kerberos
? TACACS+ uses TCP 49.

@@quiz
What does Dynamic ARP Inspection use to validate ARP packets?
- The MAC address table
* The DHCP snooping binding table
- The routing table
- CDP information
? IP–MAC pairs from DHCP.

@@flashcards
ACL processing :: top-down, first match
end of every ACL :: implicit deny any
standard ACL :: source only, near destination
extended ACL :: near source
one ACL per :: interface, direction, protocol
access-class :: ACL on VTY lines
port security default violation :: shutdown
protect / restrict :: drop silently / drop + log
sticky MAC :: learned and saved to config
DHCP snooping :: untrusted ports can't offer
DAI :: validates ARP against bindings
enable secret :: hashed privileged password
login block-for :: brute-force protection
transport input ssh :: SSH only on VTY
AAA :: authentication, authorisation, accounting
RADIUS :: UDP 1812/1813, combined
TACACS+ :: TCP 49, full encryption
802.1X roles :: supplicant, authenticator, server
site-to-site VPN :: IPsec between gateways
WPA3 :: SAE, PMF required

@@drill
Which command applies an ACL to VTY lines?
= access-class
? ip access-group is for interfaces.

@@drill
Which TCP port does TACACS+ use?
= 49
? RADIUS uses UDP 1812/1813.

@@exercise Evaluate an extended ACL
Implement first-match evaluation of the extended ACL in `acl` against each packet: match protocol (`ip` matches any), source and destination with wildcard masks (`any` and `host X` keywords) and an optional `eq` destination port. Print the matching line's sequence and action, or `implicit deny`. Then print the hit count per line, like `show access-lists`.
@@lang python
@@starter
acl = [
    "10 permit tcp 192.168.10.0 0.0.0.255 host 10.0.50.10 eq 443",
    "20 permit udp 192.168.10.0 0.0.0.255 host 10.0.50.53 eq 53",
    "30 deny ip 192.168.10.0 0.0.0.255 10.0.50.0 0.0.0.255",
    "40 permit tcp any any eq 80",
]
packets = [
    ("tcp", "192.168.10.25", "10.0.50.10", 443),
    ("tcp", "192.168.10.25", "10.0.50.10", 22),
    ("udp", "192.168.10.30", "10.0.50.53", 53),
    ("tcp", "192.168.20.5", "203.0.113.80", 80),
    ("udp", "192.168.20.5", "203.0.113.80", 123),
    ("tcp", "192.168.10.40", "198.51.100.1", 80),
]

def to_int(addr):
    raise NotImplementedError

def matches(addr, base, wildcard):
    raise NotImplementedError
@@expected
tcp 192.168.10.25 -> 10.0.50.10:443: 10 permit
tcp 192.168.10.25 -> 10.0.50.10:22: 30 deny
udp 192.168.10.30 -> 10.0.50.53:53: 20 permit
tcp 192.168.20.5 -> 203.0.113.80:80: 40 permit
udp 192.168.20.5 -> 203.0.113.80:123: implicit deny
tcp 192.168.10.40 -> 198.51.100.1:80: 40 permit
  10 permit tcp 192.168.10.0 0.0.0.255 host 10.0.50.10 eq 443 (1 matches)
  20 permit udp 192.168.10.0 0.0.0.255 host 10.0.50.53 eq 53 (1 matches)
  30 deny ip 192.168.10.0 0.0.0.255 10.0.50.0 0.0.0.255 (1 matches)
  40 permit tcp any any eq 80 (2 matches)
@@requires
def matches
def to_int
~
@@hint
A wildcard match is `(to_int(addr) & ~wc) == (to_int(base) & ~wc)` using 32-bit values. Parse each line with a token pointer: `any` → base 0.0.0.0, wildcard 255.255.255.255; `host X` → wildcard 0.0.0.0; otherwise two tokens.
@@solution
acl = [
    "10 permit tcp 192.168.10.0 0.0.0.255 host 10.0.50.10 eq 443",
    "20 permit udp 192.168.10.0 0.0.0.255 host 10.0.50.53 eq 53",
    "30 deny ip 192.168.10.0 0.0.0.255 10.0.50.0 0.0.0.255",
    "40 permit tcp any any eq 80",
]
packets = [
    ("tcp", "192.168.10.25", "10.0.50.10", 443),
    ("tcp", "192.168.10.25", "10.0.50.10", 22),
    ("udp", "192.168.10.30", "10.0.50.53", 53),
    ("tcp", "192.168.20.5", "203.0.113.80", 80),
    ("udp", "192.168.20.5", "203.0.113.80", 123),
    ("tcp", "192.168.10.40", "198.51.100.1", 80),
]

def to_int(addr):
    a, b, c, d = (int(x) for x in addr.split("."))
    return (a << 24) | (b << 16) | (c << 8) | d

def matches(addr, base, wildcard):
    wc = to_int(wildcard)
    keep = ~wc & 0xFFFFFFFF
    return (to_int(addr) & keep) == (to_int(base) & keep)

def parse(line):
    tokens = line.split()
    rule = {"seq": int(tokens[0]), "action": tokens[1], "proto": tokens[2], "port": None}
    i = 3
    for side in ("src", "dst"):
        if tokens[i] == "any":
            rule[side] = ("0.0.0.0", "255.255.255.255")
            i += 1
        elif tokens[i] == "host":
            rule[side] = (tokens[i + 1], "0.0.0.0")
            i += 2
        else:
            rule[side] = (tokens[i], tokens[i + 1])
            i += 2
    if i < len(tokens) and tokens[i] == "eq":
        rule["port"] = int(tokens[i + 1])
    return rule

rules = [parse(line) for line in acl]
hits = {r["seq"]: 0 for r in rules}
for proto, src, dst, port in packets:
    result = "implicit deny"
    for r in rules:
        if r["proto"] != "ip" and r["proto"] != proto:
            continue
        if not matches(src, *r["src"]) or not matches(dst, *r["dst"]):
            continue
        if r["port"] is not None and r["port"] != port:
            continue
        hits[r["seq"]] += 1
        result = f"{r['seq']} {r['action']}"
        break
    print(f"{proto} {src} -> {dst}:{port}: {result}")

for line in acl:
    seq = int(line.split()[0])
    print(f"  {line} ({hits[seq]} matches)")

@@exercise Port security and DAI simulator
Simulate an access switch. For port security, each port has a `maximum` and violation `mode`; process frames `(port, mac)` in order, learning MACs until the maximum, then applying the mode (`protect`: drop silently, `restrict`: drop and count, `shutdown`: err-disable; later frames on a shut port are dropped). Then check `arps` (`(port, ip, mac)`) against the DHCP snooping `bindings`: ARPs on trusted ports pass; others must match a binding's IP, MAC and port. Print each decision and final port states.
@@lang python
@@starter
ports = {"Gi1/0/5": {"maximum": 1, "mode": "shutdown"}, "Gi1/0/6": {"maximum": 2, "mode": "restrict"}, "Gi1/0/7": {"maximum": 1, "mode": "protect"}}
frames = [
    ("Gi1/0/5", "aaaa.0000.0001"), ("Gi1/0/5", "aaaa.0000.0002"), ("Gi1/0/5", "aaaa.0000.0001"),
    ("Gi1/0/6", "bbbb.0000.0001"), ("Gi1/0/6", "bbbb.0000.0002"), ("Gi1/0/6", "bbbb.0000.0003"), ("Gi1/0/6", "bbbb.0000.0003"),
    ("Gi1/0/7", "cccc.0000.0001"), ("Gi1/0/7", "cccc.0000.0002"),
]
bindings = [("10.0.10.21", "bbbb.0000.0001", "Gi1/0/6"), ("10.0.10.22", "bbbb.0000.0002", "Gi1/0/6"), ("10.0.10.31", "cccc.0000.0001", "Gi1/0/7")]
trusted = {"Gi1/0/48"}
arps = [("Gi1/0/6", "10.0.10.21", "bbbb.0000.0001"), ("Gi1/0/7", "10.0.10.1", "cccc.0000.0001"), ("Gi1/0/48", "10.0.10.1", "0011.2233.4455"), ("Gi1/0/6", "10.0.10.22", "bbbb.0000.0001")]
@@expected
Gi1/0/5 aaaa.0000.0001: learned, forwarded
Gi1/0/5 aaaa.0000.0002: violation: port err-disabled
Gi1/0/5 aaaa.0000.0001: dropped (port err-disabled)
Gi1/0/6 bbbb.0000.0001: learned, forwarded
Gi1/0/6 bbbb.0000.0002: learned, forwarded
Gi1/0/6 bbbb.0000.0003: dropped, violation 1 logged
Gi1/0/6 bbbb.0000.0003: dropped, violation 2 logged
Gi1/0/7 cccc.0000.0001: learned, forwarded
Gi1/0/7 cccc.0000.0002: dropped silently
ARP 10.0.10.21 is-at bbbb.0000.0001 on Gi1/0/6: matches binding, forwarded
ARP 10.0.10.1 is-at cccc.0000.0001 on Gi1/0/7: DAI drop (no matching binding)
ARP 10.0.10.1 is-at 0011.2233.4455 on Gi1/0/48: trusted port, forwarded
ARP 10.0.10.22 is-at bbbb.0000.0001 on Gi1/0/6: DAI drop (no matching binding)
Gi1/0/5: err-disabled, 1 MAC(s), 1 violation(s)
Gi1/0/6: up, 2 MAC(s), 2 violation(s)
Gi1/0/7: up, 1 MAC(s), 0 violation(s)
@@requires
violations
err-disabled
@@hint
Keep `learned[port] = set()`, `violations[port] = 0` and `state[port] = "up"`. A frame from a MAC already learned is always forwarded. For DAI, a binding matches when `(ip, mac, port)` is in the set of bindings.
@@solution
ports = {"Gi1/0/5": {"maximum": 1, "mode": "shutdown"}, "Gi1/0/6": {"maximum": 2, "mode": "restrict"}, "Gi1/0/7": {"maximum": 1, "mode": "protect"}}
frames = [
    ("Gi1/0/5", "aaaa.0000.0001"), ("Gi1/0/5", "aaaa.0000.0002"), ("Gi1/0/5", "aaaa.0000.0001"),
    ("Gi1/0/6", "bbbb.0000.0001"), ("Gi1/0/6", "bbbb.0000.0002"), ("Gi1/0/6", "bbbb.0000.0003"), ("Gi1/0/6", "bbbb.0000.0003"),
    ("Gi1/0/7", "cccc.0000.0001"), ("Gi1/0/7", "cccc.0000.0002"),
]
bindings = [("10.0.10.21", "bbbb.0000.0001", "Gi1/0/6"), ("10.0.10.22", "bbbb.0000.0002", "Gi1/0/6"), ("10.0.10.31", "cccc.0000.0001", "Gi1/0/7")]
trusted = {"Gi1/0/48"}
arps = [("Gi1/0/6", "10.0.10.21", "bbbb.0000.0001"), ("Gi1/0/7", "10.0.10.1", "cccc.0000.0001"), ("Gi1/0/48", "10.0.10.1", "0011.2233.4455"), ("Gi1/0/6", "10.0.10.22", "bbbb.0000.0001")]

learned = {p: set() for p in ports}
violations = {p: 0 for p in ports}
state = {p: "up" for p in ports}

for port, mac in frames:
    cfg = ports[port]
    if state[port] == "err-disabled":
        decision = "dropped (port err-disabled)"
    elif mac in learned[port]:
        decision = "forwarded"
    elif len(learned[port]) < cfg["maximum"]:
        learned[port].add(mac)
        decision = "learned, forwarded"
    elif cfg["mode"] == "protect":
        decision = "dropped silently"
    elif cfg["mode"] == "restrict":
        violations[port] += 1
        decision = f"dropped, violation {violations[port]} logged"
    else:
        violations[port] += 1
        state[port] = "err-disabled"
        decision = "violation: port err-disabled"
    print(f"{port} {mac}: {decision}")

valid = set(bindings)
for port, ip, mac in arps:
    if port in trusted:
        verdict = "trusted port, forwarded"
    elif (ip, mac, port) in valid:
        verdict = "matches binding, forwarded"
    else:
        verdict = "DAI drop (no matching binding)"
    print(f"ARP {ip} is-at {mac} on {port}: {verdict}")

for port in ports:
    print(f"{port}: {state[port]}, {len(learned[port])} MAC(s), {violations[port]} violation(s)")
