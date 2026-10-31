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
