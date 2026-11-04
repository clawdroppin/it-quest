@@lesson c2-ipservices-deep IP Services in Depth: NAT, DHCP Relay, QoS, SNMP and Syslog
@@text
IP services are 10% of CCNA but appear in many simulation questions. This lesson goes deeper than the configuration summary: NAT terminology and translation tables, DHCP relay, how QoS classifies and queues traffic, SNMP versions, syslog severity levels and TFTP/FTP for IOS file management.

## NAT terminology

| Term | Meaning | Example |
|---|---|---|
| **Inside local** | inside host's address as seen inside | 192.168.1.10 |
| **Inside global** | inside host's address as seen outside | 203.0.113.5 |
| **Outside global** | outside host's real address | 8.8.8.8 |
| **Outside local** | outside host as seen inside (usually = outside global) | 8.8.8.8 |

- **Static NAT**: one-to-one, permanent (`ip nat inside source static 192.168.1.10 203.0.113.10`): publishes a server.
- **Dynamic NAT**: from a **pool** of public addresses; runs out when the pool is exhausted.
- **PAT / NAT overload**: many inside hosts share one public IP, distinguished by **port** (`ip nat inside source list 1 interface g0/1 overload`).

```text
R1(config)# access-list 1 permit 192.168.1.0 0.0.0.255
R1(config)# ip nat inside source list 1 interface g0/1 overload
R1(config)# interface g0/0
R1(config-if)# ip nat inside
R1(config)# interface g0/1
R1(config-if)# ip nat outside
R1# show ip nat translations
Pro Inside global        Inside local          Outside local        Outside global
tcp 203.0.113.5:1024     192.168.1.10:51000    8.8.8.8:443          8.8.8.8:443
tcp 203.0.113.5:1025     192.168.1.11:51000    8.8.8.8:443          8.8.8.8:443
--- 203.0.113.10         192.168.1.20          ---                  ---
```

Common faults: `ip nat inside/outside` on the wrong interfaces, the ACL not matching the inside subnet, missing `overload`. `show ip nat statistics`, `clear ip nat translation *`.

## DHCP and DHCP relay

DORA: **Discover** (broadcast) → **Offer** → **Request** → **Ack**. Broadcasts don't cross routers, so for a central DHCP server the router on the client's subnet needs a **relay agent**:

```text
R1(config)# interface g0/0.10
R1(config-subif)# ip helper-address 10.0.50.5
```

The router converts the broadcast to a unicast to the server and sets **giaddr** (its interface address) so the server picks the right pool. A router can also be a DHCP **client** (`ip address dhcp`, typical on an ISP-facing interface) or **server** (`ip dhcp pool`, `ip dhcp excluded-address`). `show ip dhcp binding`.

## QoS concepts

Voice and video need low **delay** (one-way < 150 ms for voice), **jitter** (< 30 ms) and **loss** (< 1%). QoS tools:

1. **Classification and marking**: identify traffic and mark it. Layer 2 **CoS** (3 bits in the 802.1Q tag, 0–7); layer 3 **DSCP** (6 bits, 0–63). Key values: **EF (46)** voice, **AF41 (34)** video, **CS3 (24)** signalling, **0** best effort. The **trust boundary** is where markings are accepted (typically the IP phone or access switch).
2. **Queuing**: **LLQ** gives voice a strict priority queue (with a policer so it can't starve others); **CBWFQ** guarantees bandwidth per class.
3. **Congestion avoidance**: **WRED** drops some TCP packets early as a queue fills, avoiding tail drop and TCP global synchronisation.
4. **Policing vs shaping**: policing drops/remarks excess traffic (no delay); shaping buffers it and sends later (adds delay, smooths bursts; used toward a slower WAN provider rate).

## SNMP

Managers poll agents (UDP **161**); agents send **traps/informs** (UDP **162**; informs are acknowledged). Data lives in the **MIB** with **OIDs**.

| Version | Security |
|---|---|
| v1, v2c | community strings in clear text (`ro`/`rw`) |
| **v3** | users, **authentication** (SHA) and **encryption** (AES): `noAuthNoPriv`, `authNoPriv`, `authPriv` |

Use v3 `authPriv` and restrict with an ACL.

## Syslog

```text
*Mar  4 10:15:22.123: %LINK-3-UPDOWN: Interface GigabitEthernet0/1, changed state to down
          timestamp    facility-severity-mnemonic: description
```

| Level | Name | Mnemonic to remember |
|---|---|---|
| 0 | Emergency | **E**very |
| 1 | Alert | **A**wesome |
| 2 | Critical | **C**isco |
| 3 | Error | **E**ngineer |
| 4 | Warning | **W**ill |
| 5 | Notification | **N**eed |
| 6 | Informational | **I**ce |
| 7 | Debugging | **D**aily |

`logging host 10.0.50.20`, `logging trap warnings` (sends levels 0–4: the chosen level **and more severe**), `service timestamps log datetime msec`, `logging buffered`. Accurate **NTP** time is essential for correlating logs (`ntp server`, stratum, `show ntp associations`).

## TFTP and FTP for IOS files

TFTP (UDP 69, no authentication) and FTP (TCP 20/21, username/password) copy IOS images and configs: `copy tftp: flash:`, `copy running-config tftp:`, `show flash:`, then `boot system flash:image.bin`. SCP is the secure choice where available.

@@quiz
In PAT, what lets many inside hosts share one public address?
- Different MAC addresses
* Different source port numbers in the translations
- Different TTLs
- One VLAN per host
? The translation table maps address:port pairs.

@@quiz
What is the inside global address?
- The private address of an inside host
* The public address representing an inside host
- The real address of an outside server
- The router's loopback
? Inside local is the private one.

@@quiz
Clients on VLAN 20 can't get addresses from a DHCP server in another subnet. What is usually missing?
- `ip nat inside`
* `ip helper-address` on the VLAN 20 gateway interface
- `ip dhcp snooping`
- A static route to the clients
? DHCP discovers are broadcasts.

@@quiz
Which DSCP marking is standard for voice traffic?
- AF41
- CS3
* EF (46)
- 0
? Expedited Forwarding.

@@quiz
`logging trap warnings` is configured. Which severities are sent to the syslog server?
- Only level 4
* Levels 0 through 4
- Levels 4 through 7
- All levels
? The level and everything more severe.

@@quiz
Which SNMP version provides authentication and encryption?
- v1
- v2c
* v3
- None
? Use authPriv.

@@flashcards
inside local :: private address of inside host
inside global :: public address of inside host
outside global :: real address of outside host
static NAT :: one-to-one permanent
PAT / overload :: many-to-one using ports
show ip nat translations :: NAT table
DORA :: discover, offer, request, ack
ip helper-address :: DHCP relay
giaddr :: relay's interface address
CoS :: 3-bit L2 marking
DSCP :: 6-bit L3 marking
EF :: DSCP 46, voice
AF41 :: DSCP 34, video
LLQ :: strict priority queue
WRED :: early random drops
policing vs shaping :: drop vs buffer
SNMP ports :: 161 poll, 162 traps
SNMPv3 authPriv :: auth + encryption
syslog 0 / 7 :: emergency / debugging
logging trap 4 :: sends 0–4
TFTP :: UDP 69, no auth

@@drill
What syslog severity number is "Error"?
= 3
? Every Awesome Cisco Engineer.

@@drill
Which interface command configures a DHCP relay?
= ip helper-address
? Followed by the server's IP.

@@exercise Simulate PAT translations
Simulate a PAT router with public address `203.0.113.5` and source ports allocated from 1024 upward. For each outgoing flow `(inside_ip, inside_port, dest_ip, dest_port)`, reuse an existing translation for the same inside socket, otherwise allocate the next port. Print the translation table like `show ip nat translations`, then translate the return packets in `replies` (`(dest_port_on_public_ip, from_ip)`) back to the inside socket, printing `no translation, dropped` for unknown ports.
@@lang python
@@starter
PUBLIC = "203.0.113.5"
flows = [
    ("192.168.1.10", 51000, "8.8.8.8", 443),
    ("192.168.1.11", 51000, "8.8.8.8", 443),
    ("192.168.1.10", 51000, "8.8.8.8", 443),
    ("192.168.1.12", 40000, "1.1.1.1", 53),
    ("192.168.1.10", 51001, "93.184.216.34", 80),
]
replies = [(1025, "8.8.8.8"), (1026, "1.1.1.1"), (2000, "198.51.100.9")]
@@expected
Inside global        Inside local         Outside global
203.0.113.5:1024     192.168.1.10:51000   8.8.8.8:443
203.0.113.5:1025     192.168.1.11:51000   8.8.8.8:443
203.0.113.5:1026     192.168.1.12:40000   1.1.1.1:53
203.0.113.5:1027     192.168.1.10:51001   93.184.216.34:80
reply from 8.8.8.8 to 203.0.113.5:1025 -> 192.168.1.11:51000
reply from 1.1.1.1 to 203.0.113.5:1026 -> 192.168.1.12:40000
reply from 198.51.100.9 to 203.0.113.5:2000 -> no translation, dropped
@@requires
next_port
reverse
@@hint
Keep `table[(inside_ip, inside_port)] = public_port` and `reverse[public_port] = (inside_ip, inside_port)`.
@@solution
PUBLIC = "203.0.113.5"
flows = [
    ("192.168.1.10", 51000, "8.8.8.8", 443),
    ("192.168.1.11", 51000, "8.8.8.8", 443),
    ("192.168.1.10", 51000, "8.8.8.8", 443),
    ("192.168.1.12", 40000, "1.1.1.1", 53),
    ("192.168.1.10", 51001, "93.184.216.34", 80),
]
replies = [(1025, "8.8.8.8"), (1026, "1.1.1.1"), (2000, "198.51.100.9")]

table = {}
reverse = {}
outside = {}
next_port = 1024
for ip, port, dst, dport in flows:
    key = (ip, port)
    if key not in table:
        table[key] = next_port
        reverse[next_port] = key
        outside[next_port] = f"{dst}:{dport}"
        next_port += 1

print(f"{'Inside global':<21}{'Inside local':<21}Outside global")
for public_port, (ip, port) in sorted(reverse.items()):
    print(f"{PUBLIC + ':' + str(public_port):<21}{ip + ':' + str(port):<21}{outside[public_port]}")

for public_port, src in replies:
    if public_port in reverse:
        ip, port = reverse[public_port]
        print(f"reply from {src} to {PUBLIC}:{public_port} -> {ip}:{port}")
    else:
        print(f"reply from {src} to {PUBLIC}:{public_port} -> no translation, dropped")

@@exercise Filter and summarise syslog messages
Parse the IOS syslog lines (`%FACILITY-SEVERITY-MNEMONIC: text`). Given the configured `logging trap` level, print only the messages that would be sent to the server (severity ≤ trap level) with the severity name, then a count per facility of all messages, sorted by count descending then name.
@@lang python
@@starter
import re
from collections import Counter

TRAP = 4
NAMES = ["emergency", "alert", "critical", "error", "warning", "notification", "informational", "debugging"]
log = """*Mar  4 10:15:22.123: %LINK-3-UPDOWN: Interface GigabitEthernet0/1, changed state to down
*Mar  4 10:15:23.124: %LINEPROTO-5-UPDOWN: Line protocol on Interface GigabitEthernet0/1, changed state to down
*Mar  4 10:16:01.900: %OSPF-5-ADJCHG: Process 1, Nbr 2.2.2.2 on GigabitEthernet0/1 from FULL to DOWN
*Mar  4 10:16:40.002: %SYS-5-CONFIG_I: Configured from console by admin on vty0
*Mar  4 10:17:12.441: %SEC_LOGIN-4-LOGIN_FAILED: Login failed [user: root] [Source: 198.51.100.7]
*Mar  4 10:18:30.010: %PM-4-ERR_DISABLE: bpduguard error detected on Gi1/0/7, putting Gi1/0/7 in err-disable state
*Mar  4 10:19:02.777: %SYS-2-MALLOCFAIL: Memory allocation of 65536 bytes failed
*Mar  4 10:20:00.000: %SEC_LOGIN-4-LOGIN_FAILED: Login failed [user: admin] [Source: 198.51.100.7]"""
@@expected
Sent to server (logging trap 4 = warning):
  [3 error] LINK-UPDOWN: Interface GigabitEthernet0/1, changed state to down
  [4 warning] SEC_LOGIN-LOGIN_FAILED: Login failed [user: root] [Source: 198.51.100.7]
  [4 warning] PM-ERR_DISABLE: bpduguard error detected on Gi1/0/7, putting Gi1/0/7 in err-disable state
  [2 critical] SYS-MALLOCFAIL: Memory allocation of 65536 bytes failed
  [4 warning] SEC_LOGIN-LOGIN_FAILED: Login failed [user: admin] [Source: 198.51.100.7]
Messages per facility:
  SEC_LOGIN: 2
  SYS: 2
  LINEPROTO: 1
  LINK: 1
  OSPF: 1
  PM: 1
@@requires
re.search(
Counter(
@@hint
`re.search(r"%(\w+)-(\d)-(\w+): (.*)", line)` gives facility, severity, mnemonic and text.
@@solution
import re
from collections import Counter

TRAP = 4
NAMES = ["emergency", "alert", "critical", "error", "warning", "notification", "informational", "debugging"]
log = """*Mar  4 10:15:22.123: %LINK-3-UPDOWN: Interface GigabitEthernet0/1, changed state to down
*Mar  4 10:15:23.124: %LINEPROTO-5-UPDOWN: Line protocol on Interface GigabitEthernet0/1, changed state to down
*Mar  4 10:16:01.900: %OSPF-5-ADJCHG: Process 1, Nbr 2.2.2.2 on GigabitEthernet0/1 from FULL to DOWN
*Mar  4 10:16:40.002: %SYS-5-CONFIG_I: Configured from console by admin on vty0
*Mar  4 10:17:12.441: %SEC_LOGIN-4-LOGIN_FAILED: Login failed [user: root] [Source: 198.51.100.7]
*Mar  4 10:18:30.010: %PM-4-ERR_DISABLE: bpduguard error detected on Gi1/0/7, putting Gi1/0/7 in err-disable state
*Mar  4 10:19:02.777: %SYS-2-MALLOCFAIL: Memory allocation of 65536 bytes failed
*Mar  4 10:20:00.000: %SEC_LOGIN-4-LOGIN_FAILED: Login failed [user: admin] [Source: 198.51.100.7]"""

facilities = Counter()
print(f"Sent to server (logging trap {TRAP} = {NAMES[TRAP]}):")
for line in log.splitlines():
    m = re.search(r"%(\w+)-(\d)-(\w+): (.*)", line)
    facility, severity, mnemonic, text = m.group(1), int(m.group(2)), m.group(3), m.group(4)
    facilities[facility] += 1
    if severity <= TRAP:
        print(f"  [{severity} {NAMES[severity]}] {facility}-{mnemonic}: {text}")

print("Messages per facility:")
for facility, count in sorted(facilities.items(), key=lambda fc: (-fc[1], fc[0])):
    print(f"  {facility}: {count}")
