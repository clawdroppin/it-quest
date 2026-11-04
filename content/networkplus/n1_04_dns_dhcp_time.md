@@lesson n1-ipservices DNS, DHCP and Time Services in Depth
@@text
When "the network is down", the cause is very often DNS or DHCP. Network+ asks you to follow a DNS lookup step by step, read DHCP lease behaviour, and know why time synchronisation matters. This lesson goes beyond the record table in the services lesson.

## How a DNS lookup actually works

1. The app asks the OS **stub resolver** for `www.example.co.jp`.
2. The OS checks its **cache** and the **hosts file** (`C:\Windows\System32\drivers\etc\hosts`, `/etc/hosts`).
3. It asks the configured **recursive resolver** (company DNS server, ISP, 1.1.1.1, 8.8.8.8).
4. If the resolver hasn't cached the answer, it asks a **root server** (`.`) → referral to the **TLD** servers (`.jp`) → referral to `example.co.jp`'s **authoritative** servers → the answer.
5. The resolver caches the answer for its **TTL** and returns it.

The client's query is **recursive** ("give me the final answer"); the resolver's queries are **iterative** ("who should I ask next?").

**Negative caching**: "this name doesn't exist" (NXDOMAIN) is also cached, for the SOA's minimum TTL. That's why a record created a minute after someone looked it up still "doesn't work" for them.

## Zones and servers

- **Primary** (writable) and **secondary** zones (read-only copies via **zone transfer**, AXFR/IXFR on TCP 53; restrict transfers to known secondaries).
- **Forward zone** (names → IPs) and **reverse zone** (`in-addr.arpa` / `ip6.arpa`, PTR records).
- **Active Directory-integrated** zones replicate with AD.
- **Forwarders**: send unresolved queries to another resolver (for example the ISP or a security DNS service).
- **Conditional forwarders**: send queries for one domain (a partner's) to specific servers.
- **Split-horizon (split-brain) DNS**: internal clients get internal IPs for `app.example.com`; the internet gets the public IP.

## DNS security

- **DNSSEC**: signs records (RRSIG, DNSKEY, DS records) so resolvers can detect forgery. It doesn't encrypt.
- **DoH** (DNS over HTTPS, 443) and **DoT** (DNS over TLS, **853**): encrypt queries between client and resolver.
- **DNS filtering**: block known-bad domains at the resolver.
- Attacks: **cache poisoning**, **DNS hijacking** (changing the resolver setting or the registrar record), **typosquatting**, **DNS tunnelling** (data exfiltration in queries; look for long, random subdomains).

Tools: `nslookup example.com 8.8.8.8`, `dig example.com MX +short`, `dig +trace example.com`, `Resolve-DnsName`, `ipconfig /displaydns` and `/flushdns`.

## DHCP in depth

- **DORA** over UDP **67** (server) and **68** (client), broadcast first.
- **Lease timers**: at **T1 = 50%** of the lease the client renews **unicast** with its server; at **T2 = 87.5%** it **rebinds** by broadcast to any server; at 100% it must stop using the address.
- **Scope options**: 003 router (gateway), 006 DNS servers, 015 domain name, 042 NTP, 066/067 TFTP server and boot file (PXE), 043 vendor-specific (VoIP phones, AP controllers).
- **Reservations** (fixed IP by MAC) vs **static addresses** (configured on the device, excluded from the scope).
- **Relay / IP helper** on each router interface whose subnet has no DHCP server.
- **Scope exhaustion**: too many clients or a long lease on a busy guest Wi-Fi → shorten leases, enlarge the subnet. Attack version: **DHCP starvation**.
- **Rogue DHCP** server handing out the wrong gateway → **DHCP snooping** marks only the uplink to the real server as trusted.
- **DHCPv6** and **SLAAC** for IPv6 (routers send Router Advertisements; clients create their own address).
- **APIPA** 169.254.x.x when no DHCP server answers.

## Time: NTP and PTP

- **NTP** (UDP **123**): **stratum 0** is the reference clock (GPS, atomic), **stratum 1** servers attach to it, each hop adds one. Clients typically use stratum 2–3 servers or the domain controller hierarchy.
- Why it matters: **Kerberos** fails beyond ~5 minutes of skew; log correlation across devices; certificates (validity periods); MFA codes (TOTP).
- **PTP** (Precision Time Protocol, IEEE 1588): sub-microsecond accuracy for finance, telecom and broadcast.
- **NTS**: Network Time Security, authenticated NTP.

## IPAM

**IP address management** tracks subnets, assignments, DHCP scopes and DNS records in one place, so you don't hand out an address that a printer statically holds.

@@quiz
A user's PC can reach `10.20.1.15` but not `files.corp.local`. Other users are fine. What's the quickest check?
* The PC's DNS server setting and DNS cache (`ipconfig /all`, `/flushdns`)
- Replace the network cable
- Reboot the core switch
- Check the PoE budget
? IP works, names don't: DNS.

@@quiz
At what point in the lease does a DHCP client first try to renew with its own server?
- 25%
* 50% (T1)
- 87.5% (T2)
- 100%
? At T2 it broadcasts to any server.

@@quiz
Internal users should reach `portal.example.com` at 10.0.5.20, while internet users get 203.0.113.20. What is this?
- DNSSEC
* Split-horizon DNS
- A conditional forwarder
- Round-robin DNS
? Different answers depending on who asks.

@@quiz
Which port does DNS over TLS use?
- 53
- 443
* 853
- 123
? DoH uses 443.

@@quiz
Logs from firewalls and servers can't be lined up during an incident because timestamps differ by minutes. What fixes this?
- DHCP reservations
* Point every device at the same NTP source
- Increase log retention
- Enable DNSSEC
? Time sync is a security control.

@@flashcards
stub resolver :: the OS's DNS client
recursive query :: client asks for a final answer
iterative query :: resolver follows referrals
root → TLD → authoritative :: lookup path
negative caching :: NXDOMAIN answers are cached too
zone transfer :: AXFR/IXFR, TCP 53
conditional forwarder :: send one domain's queries elsewhere
split-horizon DNS :: different answers inside and outside
DNSSEC :: signs records, no encryption
DoT / DoH :: 853 / 443
DNS tunnelling :: data hidden in queries
T1 / T2 :: 50% renew / 87.5% rebind
option 003 / 006 :: router / DNS servers
option 066/067 :: PXE boot server and file
DHCP snooping :: blocks rogue DHCP servers
NTP stratum 1 :: directly attached to a reference clock
PTP :: precision time, IEEE 1588

@@drill
What port does NTP use?
= 123
? UDP.

@@drill
Which DHCP option number carries the default gateway?
= 3
= 003
? 006 carries DNS servers.

@@exercise Simulate a DNS lookup with caching
Simulate a recursive resolver. Each query either hits the cache (if the name was cached and its TTL hasn't expired at that time) or walks root → TLD → authoritative, counting 3 upstream queries and caching the answer with its TTL. Names not in `ZONES` return NXDOMAIN and are negatively cached for 60 seconds. Print each query's result and the total upstream queries.
@@lang python
@@starter
ZONES = {
    "www.example.co.jp": ("203.0.113.10", 300),
    "mail.example.co.jp": ("203.0.113.25", 3600),
}
queries = [
    (0, "www.example.co.jp"),
    (10, "www.example.co.jp"),
    (20, "new.example.co.jp"),
    (50, "new.example.co.jp"),
    (100, "mail.example.co.jp"),
    (320, "www.example.co.jp"),
    (400, "new.example.co.jp"),
]
@@expected
t=  0 www.example.co.jp: 203.0.113.10 (root -> TLD -> authoritative)
t= 10 www.example.co.jp: 203.0.113.10 (cache)
t= 20 new.example.co.jp: NXDOMAIN (root -> TLD -> authoritative)
t= 50 new.example.co.jp: NXDOMAIN (cache)
t=100 mail.example.co.jp: 203.0.113.25 (root -> TLD -> authoritative)
t=320 www.example.co.jp: 203.0.113.10 (root -> TLD -> authoritative)
t=400 new.example.co.jp: NXDOMAIN (root -> TLD -> authoritative)
Upstream queries: 15
@@requires
cache[
NXDOMAIN
@@hint
Store `cache[name] = (answer, expires_at)`. A hit means `name in cache and t < cache[name][1]`.
@@solution
ZONES = {
    "www.example.co.jp": ("203.0.113.10", 300),
    "mail.example.co.jp": ("203.0.113.25", 3600),
}
queries = [
    (0, "www.example.co.jp"),
    (10, "www.example.co.jp"),
    (20, "new.example.co.jp"),
    (50, "new.example.co.jp"),
    (100, "mail.example.co.jp"),
    (320, "www.example.co.jp"),
    (400, "new.example.co.jp"),
]

cache = {}
upstream = 0
for t, name in queries:
    if name in cache and t < cache[name][1]:
        answer = cache[name][0]
        source = "cache"
    else:
        upstream += 3
        if name in ZONES:
            answer, ttl = ZONES[name]
        else:
            answer, ttl = "NXDOMAIN", 60
        cache[name] = (answer, t + ttl)
        source = "root -> TLD -> authoritative"
    print(f"t={t:>3} {name}: {answer} ({source})")
print("Upstream queries:", upstream)

@@exercise DHCP scope health
For each scope, compute its pool size (usable hosts minus exclusions; reservations stay in the pool), the utilisation from active leases, and warn above 80% (`WARN`) or 95% (`CRITICAL`). For scopes over 80%, suggest halving the lease time if it's longer than 8 hours, otherwise enlarging the subnet. Use the `ipaddress` module.
@@lang python
@@starter
import ipaddress

scopes = [
    {"name": "Office-Wired", "subnet": "10.10.0.0/24", "excluded": 20, "leases": 150, "lease_hours": 192},
    {"name": "Guest-WiFi", "subnet": "10.20.0.0/24", "excluded": 4, "leases": 247, "lease_hours": 24},
    {"name": "Voice", "subnet": "10.30.0.0/23", "excluded": 10, "leases": 300, "lease_hours": 192},
    {"name": "Lab", "subnet": "10.40.0.0/26", "excluded": 2, "leases": 60, "lease_hours": 4},
]
@@expected
Office-Wired 10.10.0.0/24: 150/234 (64%) OK
Guest-WiFi 10.20.0.0/24: 247/250 (99%) CRITICAL -> shorten lease to 12 h
Voice 10.30.0.0/23: 300/500 (60%) OK
Lab 10.40.0.0/26: 60/60 (100%) CRITICAL -> enlarge subnet beyond /26
@@requires
ipaddress.ip_network(
num_addresses
@@hint
Usable hosts = `net.num_addresses - 2`, pool = that minus `excluded`. Utilisation = leases / pool.
@@solution
import ipaddress

scopes = [
    {"name": "Office-Wired", "subnet": "10.10.0.0/24", "excluded": 20, "leases": 150, "lease_hours": 192},
    {"name": "Guest-WiFi", "subnet": "10.20.0.0/24", "excluded": 4, "leases": 247, "lease_hours": 24},
    {"name": "Voice", "subnet": "10.30.0.0/23", "excluded": 10, "leases": 300, "lease_hours": 192},
    {"name": "Lab", "subnet": "10.40.0.0/26", "excluded": 2, "leases": 60, "lease_hours": 4},
]

for s in scopes:
    net = ipaddress.ip_network(s["subnet"])
    pool = net.num_addresses - 2 - s["excluded"]
    used = s["leases"] / pool
    if used > 0.95:
        level = "CRITICAL"
    elif used > 0.8:
        level = "WARN"
    else:
        level = "OK"
    line = f"{s['name']} {net}: {s['leases']}/{pool} ({used:.0%}) {level}"
    if level != "OK":
        if s["lease_hours"] > 8:
            line += f" -> shorten lease to {s['lease_hours'] // 2} h"
        else:
            line += f" -> enlarge subnet beyond /{net.prefixlen}"
    print(line)
