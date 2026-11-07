@@lesson sp2-threats Threats, Attacks and Vulnerabilities
@@text
## Threat actors

| Actor | Motivation | Resources |
|---|---|---|
| **Nation-state** | espionage, disruption, war | very high; **APTs** (advanced persistent threats) |
| **Organized crime** | money (ransomware, fraud) | high |
| **Hacktivist** | a political or social cause | varies |
| **Insider threat** | revenge, money, carelessness | has access already |
| **Unskilled attacker** ("script kiddie") | thrills, reputation | uses others' tools |
| **Shadow IT** | convenience | staff using unapproved apps and services |

Attributes: internal/external, resources and funding, sophistication. Motivations: data exfiltration, espionage, service disruption, blackmail, financial gain, revenge, ethics, war.

## Attack vectors and surfaces

Email, SMS, voice calls, instant messages, images and files, removable media (USB drops), **vulnerable software**, **unsupported (end-of-life) systems**, open service ports, default credentials, unsecured networks (wireless, wired, Bluetooth), and the **supply chain** (managed service providers, vendors, compromised software updates).

**Social engineering**: phishing, vishing, smishing, **pretexting** (a made-up scenario), **watering hole** (infecting a site the targets visit), **brand impersonation**, **typosquatting** (`rnicrosoft.com`), **business email compromise**, misinformation/disinformation.

## Application attacks

| Attack | How it works | Defence |
|---|---|---|
| **SQL injection** | input changes a database query | parameterized queries |
| **Cross-site scripting (XSS)** | attacker's script runs in other users' browsers | output encoding, CSP |
| **CSRF** | tricks a logged-in browser into sending a request | anti-CSRF tokens, SameSite cookies |
| **Buffer overflow** | writes past a memory buffer to run code | memory-safe languages (Rust!), ASLR, DEP |
| **Directory traversal** | `../../etc/passwd` | validate paths |
| **Race condition / TOCTOU** | the state changes between check and use | locking, atomic operations |
| **Privilege escalation** | gaining higher rights | patching, least privilege |
| **Replay** | resending captured valid traffic | timestamps, nonces |
| **Forgery (SSRF)** | the server is tricked into making requests | allow-lists for outbound requests |

## Password and network attacks

- **Brute force**, **dictionary**, **password spraying** (one common password against many accounts, avoiding lockouts), **credential stuffing** (leaked passwords tried on other sites).
- **DDoS** (volumetric, **amplified**/**reflected** via DNS or NTP), **on-path**, **DNS poisoning**, **wireless** (evil twin, deauth), **malicious code**.
- Cryptographic: **downgrade** attacks, **collisions**, **birthday** attacks.

## Malware

Ransomware, trojans, worms, spyware, **bloatware**, viruses, keyloggers, **logic bombs** (trigger on a condition, e.g. an employee's account being deleted), **rootkits**, **fileless** malware (lives in memory and PowerShell).

## Indicators of compromise

Account lockouts, impossible travel (logins from Tokyo and Brazil within an hour), concurrent sessions, blocked content, resource consumption, missing logs, out-of-cycle logging, published/documented leaks.

## Vulnerabilities

- **Zero-day**: unknown to the vendor; no patch exists yet.
- **Misconfiguration**: the most common cloud breach cause (public storage buckets, open security groups).
- Others: unpatched OS and firmware, **legacy/EOL** hardware, virtualization (**VM escape**), mobile (**jailbreaking**, **sideloading**), weak crypto, supply chain.

@@quiz
An attacker tries the password "Summer2026!" against every account in a company, once each. What attack is this?
* Password spraying
- Brute force
- Credential stuffing
- Rainbow table attack
? One password across many accounts avoids per-account lockouts.

@@quiz
Users of a popular forum report that viewing a certain post makes their browser send their session cookie to an unknown site. What vulnerability does the forum have?
* Cross-site scripting (XSS)
- SQL injection
- Buffer overflow
- Directory traversal
? The attacker's script runs in the victims' browsers. Fix by encoding output.

@@quiz
A disgruntled developer added code that deletes the payroll database if their account is ever disabled. What is this?
* A logic bomb
- A worm
- A rootkit
- Ransomware
? It waits for a condition, then runs.

@@quiz
A login form has the input `' OR 1=1 --`. What attack is being attempted?
* SQL injection
- XSS
- CSRF
- Replay
? Fix with parameterized queries (the SQL track shows how).

@@quiz
Which threat actor has the most resources and conducts long, stealthy campaigns?
* Nation-state (APT)
- Unskilled attacker
- Hacktivist
- Shadow IT
? Advanced persistent threats are typically state-sponsored.

@@quiz
A user logs in from Osaka at 09:00 and from São Paulo at 09:20. What indicator is this?
* Impossible travel
- Resource consumption
- Out-of-cycle logging
- Blocked content
? Nobody travels that fast: the account is likely compromised.

@@quiz
Attackers compromised a software vendor's update server, and customers installed malware with a normal update. What kind of attack is this?
* Supply chain attack
- Watering hole
- Typosquatting
- Pretexting
? Trusted software or services become the delivery vector.

@@quiz
Which vulnerability is, by definition, one for which no patch exists yet?
* Zero-day
- Misconfiguration
- Legacy system
- Weak cipher
? The vendor has had "zero days" to fix it.

@@flashcards
APT :: Advanced persistent threat: well-funded, long, stealthy (often nation-state)
insider threat :: Attack from someone with legitimate access
shadow IT :: Unapproved apps and services used by staff
pretexting :: A made-up scenario to manipulate a victim
watering hole :: Compromise a site the targets visit
typosquatting :: Look-alike domains
password spraying :: One password, many accounts
credential stuffing :: Leaked credentials tried on other sites
XSS :: Injected script runs in other users' browsers
CSRF :: Forces a logged-in browser to send a request
SSRF :: Server tricked into making requests
TOCTOU :: Time-of-check to time-of-use race condition
logic bomb :: Malware triggered by a condition
fileless malware :: Runs in memory/scripts, no file on disk
impossible travel :: Logins from distant places too close in time
zero-day :: Vulnerability with no patch yet
VM escape :: Breaking out of a VM to the hypervisor
amplification attack :: Small requests produce huge responses at the victim

@@drill
What is the name for malware that leaves no file on disk and runs in memory, often through PowerShell?
= fileless malware
= fileless
? EDR tools watch behaviour, not just files, to catch it.

@@exercise Brute force or password spraying?
The sign-in log lists failed and successful logins as `(time, source IP, account, result)`. Write `classify_sources(log)` that looks at each source IP's failures:

- 10 or more failures against **one** account → `brute force`
- failures against 5 or more **different** accounts with at most 2 tries each → `password spraying`
- otherwise → `noise`

For each brute force or spraying source, also report any account that later signed in **successfully** from that IP: that account is probably compromised. Print a line per source IP, sorted.
@@lang python
@@starter
log = []
for i in range(12):
    log.append((f"09:{i:02d}", "203.0.113.7", "admin", "fail"))
for i, user in enumerate(["aiko", "ben", "chika", "dev", "eri", "fumi"]):
    log.append((f"10:{i:02d}", "198.51.100.23", user, "fail"))
log.append(("10:07", "198.51.100.23", "fumi", "success"))
log.append(("11:00", "192.0.2.50", "ben", "fail"))
log.append(("11:01", "192.0.2.50", "ben", "success"))
log.append(("11:30", "203.0.113.7", "admin", "fail"))

def classify_sources(log):
    raise NotImplementedError


for ip, (kind, fails, accounts, hit) in sorted(classify_sources(log).items()):
    line = f"{ip}: {kind} ({fails} failures, {accounts} account(s))"
    if hit:
        line += "; then signed in as " + ", ".join(hit) + ": reset and investigate"
    print(line)
@@expected
192.0.2.50: noise (1 failures, 1 account(s))
198.51.100.23: password spraying (6 failures, 6 account(s)); then signed in as fumi: reset and investigate
203.0.113.7: brute force (13 failures, 1 account(s))
@@requires
def classify_sources
len(per_account)
@@hint
Count failures per IP **and** per account (`failures[ip][account]`). Brute force is many tries at one account; spraying is few tries at many accounts.
@@solution
log = []
for i in range(12):
    log.append((f"09:{i:02d}", "203.0.113.7", "admin", "fail"))
for i, user in enumerate(["aiko", "ben", "chika", "dev", "eri", "fumi"]):
    log.append((f"10:{i:02d}", "198.51.100.23", user, "fail"))
log.append(("10:07", "198.51.100.23", "fumi", "success"))
log.append(("11:00", "192.0.2.50", "ben", "fail"))
log.append(("11:01", "192.0.2.50", "ben", "success"))
log.append(("11:30", "203.0.113.7", "admin", "fail"))

def classify_sources(log):
    failures, successes = {}, {}
    for _, ip, account, result in log:
        if result == "fail":
            failures.setdefault(ip, {})
            failures[ip][account] = failures[ip].get(account, 0) + 1
        else:
            successes.setdefault(ip, set()).add(account)
    report = {}
    for ip, per_account in failures.items():
        if max(per_account.values()) >= 10 and len(per_account) == 1:
            kind = "brute force"
        elif len(per_account) >= 5 and max(per_account.values()) <= 2:
            kind = "password spraying"
        else:
            kind = "noise"
        hit = sorted(successes.get(ip, set()) & set(per_account)) if kind != "noise" else []
        report[ip] = (kind, sum(per_account.values()), len(per_account), hit)
    return report

for ip, (kind, fails, accounts, hit) in sorted(classify_sources(log).items()):
    line = f"{ip}: {kind} ({fails} failures, {accounts} account(s))"
    if hit:
        line += "; then signed in as " + ", ".join(hit) + ": reset and investigate"
    print(line)

@@exercise Match logs against indicators of compromise
A threat intelligence feed gives indicators of compromise: file hashes, IP addresses and domains. Write `load_iocs(feed)` that sorts them by type (a 64-character hex string is a SHA-256 hash, `ipaddress` can parse an IP, anything else with a dot is a domain), then `hunt(events, iocs)` that returns every event matching an IoC. A domain matches its subdomains too (`cdn.evil.example` matches `evil.example`). Print each hit with the host and the IoC, then the hosts to isolate.
@@lang python
@@starter
import ipaddress

feed = """203.0.113.66
evil.example
b5bb9d8014a0f9b1d61e21e796d78dccdf1352f23cd32812f4850b878ae4944c
198.51.100.9
update-check.example.net"""
events = [
    {"host": "pc-14", "type": "dns", "value": "cdn.evil.example"},
    {"host": "pc-07", "type": "dns", "value": "www.example.co.jp"},
    {"host": "srv-02", "type": "conn", "value": "203.0.113.66"},
    {"host": "pc-22", "type": "file", "value": "b5bb9d8014a0f9b1d61e21e796d78dccdf1352f23cd32812f4850b878ae4944c"},
    {"host": "pc-07", "type": "conn", "value": "192.0.2.10"},
    {"host": "pc-31", "type": "dns", "value": "notevil.example"},
    {"host": "pc-31", "type": "dns", "value": "update-check.example.net"},
]

def load_iocs(feed):
    raise NotImplementedError


def hunt(events, iocs):
    raise NotImplementedError


hits = hunt(events, load_iocs(feed))
for host, kind, ioc in hits:
    print(f"HIT {host}: {kind} {ioc}")
print("Isolate: " + ", ".join(sorted({h for h, _, _ in hits})))
@@expected
HIT pc-14: domain evil.example
HIT srv-02: ip 203.0.113.66
HIT pc-22: hash b5bb9d8014a0...
HIT pc-31: domain update-check.example.net
Isolate: pc-14, pc-22, pc-31, srv-02
@@requires
def load_iocs
def hunt
endswith("." + d)
@@hint
Match a domain exactly or as a suffix after a dot: `v.endswith("." + d)`. Without the dot, `notevil.example` would match `evil.example`.
@@solution
import ipaddress

feed = """203.0.113.66
evil.example
b5bb9d8014a0f9b1d61e21e796d78dccdf1352f23cd32812f4850b878ae4944c
198.51.100.9
update-check.example.net"""
events = [
    {"host": "pc-14", "type": "dns", "value": "cdn.evil.example"},
    {"host": "pc-07", "type": "dns", "value": "www.example.co.jp"},
    {"host": "srv-02", "type": "conn", "value": "203.0.113.66"},
    {"host": "pc-22", "type": "file", "value": "b5bb9d8014a0f9b1d61e21e796d78dccdf1352f23cd32812f4850b878ae4944c"},
    {"host": "pc-07", "type": "conn", "value": "192.0.2.10"},
    {"host": "pc-31", "type": "dns", "value": "notevil.example"},
    {"host": "pc-31", "type": "dns", "value": "update-check.example.net"},
]

def load_iocs(feed):
    iocs = {"hash": set(), "ip": set(), "domain": set()}
    for line in feed.split():
        if len(line) == 64 and all(c in "0123456789abcdef" for c in line.lower()):
            iocs["hash"].add(line.lower())
            continue
        try:
            ipaddress.ip_address(line)
            iocs["ip"].add(line)
        except ValueError:
            iocs["domain"].add(line.lower())
    return iocs

def hunt(events, iocs):
    hits = []
    for e in events:
        v = e["value"].lower()
        if e["type"] == "file" and v in iocs["hash"]:
            hits.append((e["host"], "hash", v[:12] + "..."))
        elif e["type"] == "conn" and v in iocs["ip"]:
            hits.append((e["host"], "ip", v))
        elif e["type"] == "dns":
            for d in iocs["domain"]:
                if v == d or v.endswith("." + d):
                    hits.append((e["host"], "domain", d))
    return hits

hits = hunt(events, load_iocs(feed))
for host, kind, ioc in hits:
    print(f"HIT {host}: {kind} {ioc}")
print("Isolate: " + ", ".join(sorted({h for h, _, _ in hits})))
