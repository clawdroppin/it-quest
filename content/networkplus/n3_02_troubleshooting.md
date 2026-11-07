@@lesson n3-troubleshooting Network Troubleshooting and Tools
@@text
Network+ uses the same 7-step methodology as A+, with one useful addition: **approaches by layer**.

- **Top-down**: start at the application (does the website load?) and go down.
- **Bottom-up**: start at the cable and go up. Best when nothing works at all.
- **Divide and conquer**: start in the middle (can I ping the gateway?) and go up or down from the result.
- **Follow the path**: trace from the source to the destination, hop by hop.

## Tools

| Command / tool | Use |
|---|---|
| `ping`, `traceroute`/`tracert`, `pathping`, `mtr` | reachability, path, where packets are lost |
| `ipconfig` / `ip addr`, `ifconfig` | interface settings |
| `nslookup`, `dig` | DNS queries (`dig +short mx example.com`) |
| `arp -a` | the ARP cache |
| `netstat`, `ss` | connections and listening ports |
| `route print` / `ip route` | the routing table |
| `nmap` | port and host discovery (only on networks you're authorized to scan!) |
| `tcpdump`, **Wireshark** | packet capture and analysis |
| `iperf` | measure throughput between two hosts |
| `curl`, `openssl s_client` | test HTTP(S) and certificates |
| `show interface`, `show ip route`, `show vlan`, `show mac address-table` | on Cisco devices |
| cable tester, **TDR**/**OTDR**, toner, light meter, Wi-Fi analyzer | physical layer |

## Common problems

| Symptom | Likely cause |
|---|---|
| Link light off | cable, wrong port, disabled port, bad transceiver |
| Slow link at 100 Mb/s on a gigabit port | a damaged pair (gigabit needs all 4 pairs), **duplex/speed mismatch** |
| CRC errors and runts in interface counters | bad cable, interference, duplex mismatch |
| Works then stops every few minutes | IP **address conflict**, flapping link, STP topology changes |
| Can reach IPs but not names | DNS |
| Can reach the local subnet only | wrong **default gateway** or mask |
| One VLAN can't reach another | trunk missing the VLAN, routing, ACL |
| Fiber link down | dirty connector, wrong type (single-mode/multimode), TX/RX swapped |
| Voice choppy | **jitter**, **latency**, packet loss: check QoS |
| New site can't be reached from one network only | asymmetric routing, missing route back |
| Wi-Fi drops in one area | coverage gap, interference, channel overlap |

**Latency** is delay; **jitter** is variation in delay; **throughput** is real-world speed; **bandwidth** is capacity. **Attenuation** is signal loss over distance (why copper stops at 100 m).

@@quiz
Nothing works on a user's PC: no network at all. Which approach is usually fastest?
* Bottom-up: start with the physical layer
- Top-down: start with the application
- Reinstall the OS
- Replace the switch
? With total failure, check the cable and link light first.

@@quiz
A gigabit link keeps negotiating at 100 Mb/s. What's a likely cause?
* A damaged cable pair or a bad termination
- DNS misconfiguration
- A wrong default gateway
- An expired certificate
? 100 Mb/s Ethernet needs 2 pairs; gigabit needs all 4.

@@quiz
Which tool measures throughput between two hosts?
* iperf
- nslookup
- arp
- Wireshark only
? Run iperf as a server on one host and a client on the other.

@@quiz
A PC can ping hosts on its own subnet but nothing beyond it. What should you check?
* The default gateway (and subnet mask) setting
- The DNS server
- The MX record
- The Wi-Fi channel
? Traffic for other networks goes to the gateway.

@@quiz
Users report choppy VoIP calls, though data apps are fine. Which metrics matter most?
* Jitter, latency and packet loss
- Disk throughput
- DNS TTL
- Number of VLANs
? Voice needs consistent, low delay. QoS should prioritize it.

@@quiz
Which tool locates a break in a fiber cable, and how far away it is?
* An OTDR
- A toner probe
- A crimper
- A loopback plug
? An optical time-domain reflectometer measures reflections along the fiber (a TDR does the same for copper).

@@quiz
Interface counters on a switch port show many CRC errors. What's most likely?
* A physical problem: bad cable, interference or a duplex mismatch
- A DNS misconfiguration
- Too many VLANs
- A wrong DHCP lease time
? CRC errors mean frames arrived damaged.

@@flashcards
top-down / bottom-up :: Start at the application / at the physical layer
divide and conquer :: Start in the middle layer
traceroute / tracert :: Shows each hop to a destination
dig / nslookup :: DNS query tools
arp -a :: Show the ARP cache
nmap :: Port and host scanner (authorized use only)
Wireshark / tcpdump :: Packet capture and analysis
iperf :: Throughput test
OTDR / TDR :: Find breaks in fiber / copper
duplex mismatch :: One side full, the other half duplex: errors and slowness
latency / jitter :: Delay / variation in delay
attenuation :: Signal loss over distance
CRC errors :: Damaged frames: usually physical problems

@@drill
Which command shows the routing table on Windows?
= route print
= netstat -r
? On Linux: `ip route`.

@@exercise Read the ping and traceroute results
Apply the troubleshooting ladder to test results. Write `diagnose(results)`:

- loopback (127.0.0.1) fails → `TCP/IP stack problem`
- own IP fails → `NIC or driver problem`
- gateway fails → `local network problem (cable, switch, VLAN)`
- an internet IP fails → `routing or ISP problem beyond the gateway`
- a name fails but the IP works → `DNS problem`
- everything works → `network OK: check the application`

Check the tests in that order and report the first failure.
@@lang python
@@starter
cases = {
    "PC-1": {"loopback": True, "own_ip": True, "gateway": False, "internet_ip": False, "name": False},
    "PC-2": {"loopback": True, "own_ip": True, "gateway": True, "internet_ip": True, "name": False},
    "PC-3": {"loopback": True, "own_ip": True, "gateway": True, "internet_ip": True, "name": True},
    "PC-4": {"loopback": True, "own_ip": True, "gateway": True, "internet_ip": False, "name": False},
}
LADDER = [
    ("loopback", "TCP/IP stack problem"),
    ("own_ip", "NIC or driver problem"),
    ("gateway", "local network problem (cable, switch, VLAN)"),
    ("internet_ip", "routing or ISP problem beyond the gateway"),
    ("name", "DNS problem"),
]

def diagnose(results):
    raise NotImplementedError


for pc, results in cases.items():
    print(f"{pc}: {diagnose(results)}")
@@expected
PC-1: local network problem (cable, switch, VLAN)
PC-2: DNS problem
PC-3: network OK: check the application
PC-4: routing or ISP problem beyond the gateway
@@requires
def diagnose
results[
@@hint
Keep the tests in ladder order in a list and return at the first one that failed.
@@solution
cases = {
    "PC-1": {"loopback": True, "own_ip": True, "gateway": False, "internet_ip": False, "name": False},
    "PC-2": {"loopback": True, "own_ip": True, "gateway": True, "internet_ip": True, "name": False},
    "PC-3": {"loopback": True, "own_ip": True, "gateway": True, "internet_ip": True, "name": True},
    "PC-4": {"loopback": True, "own_ip": True, "gateway": True, "internet_ip": False, "name": False},
}
LADDER = [
    ("loopback", "TCP/IP stack problem"),
    ("own_ip", "NIC or driver problem"),
    ("gateway", "local network problem (cable, switch, VLAN)"),
    ("internet_ip", "routing or ISP problem beyond the gateway"),
    ("name", "DNS problem"),
]

def diagnose(results):
    for test, meaning in LADDER:
        if not results[test]:
            return meaning
    return "network OK: check the application"

for pc, results in cases.items():
    print(f"{pc}: {diagnose(results)}")

@@exercise Match symptoms to common network problems
Write `diagnose(case)` that turns measurements into the lesson's common problems, checking all of these:

- interface `duplex` differs between the two ends, or many `late_collisions` → **duplex mismatch**;
- `crc_errors` above 0.1% of frames → **bad cable or interference**;
- large pings fail (`big_ping_ok` false) while small ones work → **MTU / fragmentation**;
- `dhcp_address` starts with 169.254 → **DHCP failure (APIPA)**;
- name lookups fail while ping by IP works → **DNS**;
- two hosts report the same IP (`duplicate_ip`) → **IP conflict**;
- `signal_dbm` weaker than −70 → **weak Wi-Fi signal**.

Print each case with its problems and the first fix, or `no common problem found: escalate`.
@@lang python
@@starter
FIX = {
    "duplex mismatch": "set both ends to auto (or both to the same fixed speed/duplex)",
    "bad cable or interference": "replace the cable, keep it away from power lines",
    "MTU / fragmentation": "match the MTU along the path; check VPN/tunnel overhead",
    "DHCP failure (APIPA)": "check the DHCP server, scope and relay",
    "DNS": "check the DNS server settings with nslookup/dig",
    "IP conflict": "find the second host by its MAC and fix the static address or reservation",
    "weak Wi-Fi signal": "move closer to the AP or add an AP; check for interference",
}
cases = [
    {"name": "Printer port", "duplex": ("full", "half"), "late_collisions": 120, "frames": 50000, "crc_errors": 3},
    {"name": "Warehouse PC", "frames": 20000, "crc_errors": 900, "duplex": ("full", "full")},
    {"name": "VPN user", "big_ping_ok": False, "ping_ip_ok": True, "dns_ok": True},
    {"name": "New laptop", "dhcp_address": "169.254.33.7"},
    {"name": "Reception PC", "ping_ip_ok": True, "dns_ok": False},
    {"name": "Meeting room", "duplicate_ip": True, "signal_dbm": -78},
    {"name": "Server", "ping_ip_ok": True, "dns_ok": True, "frames": 1000000, "crc_errors": 2},
]

def diagnose(c):
    raise NotImplementedError


for c in cases:
    problems = diagnose(c)
    if not problems:
        print(f"{c['name']}: no common problem found: escalate")
    for p in problems:
        print(f"{c['name']}: {p} -> {FIX[p]}")
@@expected
Printer port: duplex mismatch -> set both ends to auto (or both to the same fixed speed/duplex)
Warehouse PC: bad cable or interference -> replace the cable, keep it away from power lines
VPN user: MTU / fragmentation -> match the MTU along the path; check VPN/tunnel overhead
New laptop: DHCP failure (APIPA) -> check the DHCP server, scope and relay
Reception PC: DNS -> check the DNS server settings with nslookup/dig
Meeting room: IP conflict -> find the second host by its MAC and fix the static address or reservation
Meeting room: weak Wi-Fi signal -> move closer to the AP or add an AP; check for interference
Server: no common problem found: escalate
@@requires
def diagnose
.get(
@@hint
Use `case.get(key, default)` for each measurement, since every case reports different ones. Check each rule independently and collect all the matches.
@@solution
FIX = {
    "duplex mismatch": "set both ends to auto (or both to the same fixed speed/duplex)",
    "bad cable or interference": "replace the cable, keep it away from power lines",
    "MTU / fragmentation": "match the MTU along the path; check VPN/tunnel overhead",
    "DHCP failure (APIPA)": "check the DHCP server, scope and relay",
    "DNS": "check the DNS server settings with nslookup/dig",
    "IP conflict": "find the second host by its MAC and fix the static address or reservation",
    "weak Wi-Fi signal": "move closer to the AP or add an AP; check for interference",
}
cases = [
    {"name": "Printer port", "duplex": ("full", "half"), "late_collisions": 120, "frames": 50000, "crc_errors": 3},
    {"name": "Warehouse PC", "frames": 20000, "crc_errors": 900, "duplex": ("full", "full")},
    {"name": "VPN user", "big_ping_ok": False, "ping_ip_ok": True, "dns_ok": True},
    {"name": "New laptop", "dhcp_address": "169.254.33.7"},
    {"name": "Reception PC", "ping_ip_ok": True, "dns_ok": False},
    {"name": "Meeting room", "duplicate_ip": True, "signal_dbm": -78},
    {"name": "Server", "ping_ip_ok": True, "dns_ok": True, "frames": 1000000, "crc_errors": 2},
]

def diagnose(c):
    found = []
    duplex = c.get("duplex")
    if (duplex and duplex[0] != duplex[1]) or c.get("late_collisions", 0) > 0:
        found.append("duplex mismatch")
    if c.get("frames") and c.get("crc_errors", 0) / c["frames"] > 0.001:
        found.append("bad cable or interference")
    if c.get("big_ping_ok") is False:
        found.append("MTU / fragmentation")
    if c.get("dhcp_address", "").startswith("169.254."):
        found.append("DHCP failure (APIPA)")
    if c.get("ping_ip_ok") and c.get("dns_ok") is False:
        found.append("DNS")
    if c.get("duplicate_ip"):
        found.append("IP conflict")
    if c.get("signal_dbm", 0) < -70:
        found.append("weak Wi-Fi signal")
    return found

for c in cases:
    problems = diagnose(c)
    if not problems:
        print(f"{c['name']}: no common problem found: escalate")
    for p in problems:
        print(f"{c['name']}: {p} -> {FIX[p]}")
