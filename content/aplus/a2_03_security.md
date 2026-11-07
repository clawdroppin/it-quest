@@lesson a2-security A+ Security
@@text
## Malware

| Type | What it does |
|---|---|
| **Virus** | attaches to files, spreads when run |
| **Worm** | spreads by itself over the network |
| **Trojan** | pretends to be something useful |
| **Ransomware** | encrypts files, demands payment |
| **Spyware / keylogger** | watches and records what you do |
| **Rootkit** | hides deep in the OS, hard to detect |
| **Botnet** | infected machines controlled remotely |
| **Cryptominer** | steals CPU/GPU to mine cryptocurrency |

Symptoms: pop-ups, browser redirects, a new toolbar or search engine, slow performance, fans busy while idle, disabled antivirus, files renamed with odd extensions, certificate warnings.

## The malware removal process

CompTIA's order, worth memorizing:

1. **Investigate and verify** malware symptoms.
2. **Quarantine** the infected system (disconnect it from the network).
3. **Disable System Restore** (in Windows), so infected restore points don't come back.
4. **Remediate**: update the anti-malware software, then scan and remove (Safe Mode, a boot-time scan or recovery environment if needed).
5. **Schedule scans and run updates.**
6. **Enable System Restore** and create a new restore point.
7. **Educate the end user.**

## Windows security features

- **Microsoft Defender Antivirus** and **Defender Firewall** (separate rules for Domain, Private and Public networks).
- **BitLocker** encrypts whole drives (uses the TPM); **BitLocker To Go** encrypts USB drives. Store the **recovery key** safely (in AD or Entra ID). **EFS** encrypts individual files.
- **NTFS permissions** (Full control, Modify, Read & execute, List, Read, Write) vs **share permissions** (Full, Change, Read). Over the network, the **more restrictive** of the two applies. **Deny** overrides Allow. Copying a file gives it the new folder's permissions; moving it within the same volume keeps them.
- Account security: strong passwords, lockout after failed attempts, disable the **Guest** account, rename/disable the built-in Administrator, screen lock timeouts.

## Wireless and SOHO security

WPA3 (or WPA2-AES), change default router passwords, disable **WPS**, update firmware, guest networks, MAC filtering (weak), content filtering. **RADIUS/TACACS+** and **802.1X** give each user their own Wi-Fi login in companies (WPA2/WPA3-**Enterprise**).

## Social engineering and physical security

**Phishing** (email), **vishing** (voice), **smishing** (SMS), **whaling** (targeting executives), **tailgating** (following someone through a secure door), **shoulder surfing**, **impersonation**, **dumpster diving**. Physical controls: badge readers, **access control vestibules** (mantraps), security guards, cable locks, privacy screens, locked server rooms, video surveillance.

## Data destruction

- **Erasing / wiping** (overwriting) lets a drive be reused.
- **Degaussing** destroys magnetic data (HDDs, tapes), not SSDs.
- **Physical destruction**: shredding, drilling, incineration, with a **certificate of destruction** from third parties.
- A normal **quick format does not remove data.**

@@quiz
A user's browser homepage changed, and pop-ups appear constantly. You confirmed it's malware. What's the NEXT step?
* Quarantine the system by disconnecting it from the network
- Disable System Restore
- Educate the user
- Schedule regular scans
? After investigating and verifying, quarantine before anything else.

@@quiz
Why disable System Restore during malware removal?
* So the malware isn't restored later from infected restore points
- To make the scan faster
- Because antivirus can't run with it on
- To free space for the antivirus
? Re-enable it and create a clean restore point afterwards.

@@quiz
A folder is shared with share permission "Read" for Everyone, and NTFS permission "Modify" for Sales. What can a Sales user do when accessing it over the network?
* Only read
- Modify
- Full control
- Nothing
? Over the network the more restrictive of share and NTFS permissions applies.

@@quiz
A laptop containing customer data is stolen. Which feature would have kept the data unreadable?
* BitLocker full-disk encryption
- Windows Defender Firewall
- A strong Wi-Fi password
- System Restore
? Without the key, an encrypted disk is useless to a thief.

@@quiz
Someone without a badge follows an employee through a secured door. What is this?
* Tailgating
- Phishing
- Shoulder surfing
- Whaling
? An access control vestibule (mantrap) prevents it.

@@quiz
Old SSDs must be disposed of so data can't be recovered. Which method doesn't work for SSDs?
* Degaussing
- Shredding
- Secure erase by the manufacturer's tool
- Incineration
? SSDs store data in flash memory, not magnetically.

@@quiz
Which malware is designed to hide itself deep in the OS and evade detection?
* Rootkit
- Adware
- Worm
- Keylogger
? Rootkits often need offline or boot-time scanning, or a reinstall.

@@flashcards
malware removal 1-7 :: Investigate, quarantine, disable System Restore, remediate, schedule scans/updates, enable System Restore, educate
worm :: Self-spreading malware
trojan :: Malware disguised as legitimate software
rootkit :: Hides deep in the OS
BitLocker :: Full-disk encryption; BitLocker To Go for USB
EFS :: Encrypts individual files on NTFS
share + NTFS permissions :: The more restrictive applies over the network
Deny :: Overrides Allow
copy vs move permissions :: Copy inherits the new folder's; move on the same volume keeps them
WPA2/3-Enterprise :: Individual Wi-Fi logins via 802.1X and RADIUS
tailgating :: Following someone through a secure door
vishing / smishing :: Phishing by voice / SMS
degaussing :: Erases magnetic media (not SSDs)

@@drill
What is the last step of the CompTIA malware removal process? (two words)
= educate user
= educate the user
= educate the end user
= educate end user
? Most infections start with a click; teaching prevents the next one.

@@exercise Name the social engineering attack
Write `classify(report)` that names the attack described in each incident report, checking these clues in order (the report is lowercased first):

1. `ceo`, `cfo` or `executive` together with `email` → `whaling`
2. `email` → `phishing`
3. `text message` or `sms` → `smishing`
4. `phone` or `call` → `vishing`
5. `followed` or `held the door` → `tailgating`
6. `over my shoulder` or `watched me type` → `shoulder surfing`
7. `bin` or `trash` → `dumpster diving`
8. otherwise → `unknown: escalate to security`

Then print how many reports there were of each attack, most common first (ties alphabetically).
@@lang python
@@starter
reports = [
    "Email asking me to reset my password on a strange site",
    "Someone followed me through the badge door without scanning",
    "Text message said my parcel was held and asked for a card number",
    "Email to the CFO asking for an urgent wire transfer",
    "A caller said he was from IT and needed my password",
    "A visitor watched me type my PIN at the printer",
    "Old invoices with customer data were found in the trash outside",
    "Email with an invoice attachment from an unknown sender",
    "USB stick left in the car park",
]

def classify(report):
    raise NotImplementedError


counts = {}
for r in reports:
    kind = classify(r)
    counts[kind] = counts.get(kind, 0) + 1
    print(f"{kind}: {r}")
print("Totals:")
for kind, n in sorted(counts.items(), key=lambda kv: (-kv[1], kv[0])):
    print(f"  {n} {kind}")
@@expected
phishing: Email asking me to reset my password on a strange site
tailgating: Someone followed me through the badge door without scanning
smishing: Text message said my parcel was held and asked for a card number
whaling: Email to the CFO asking for an urgent wire transfer
vishing: A caller said he was from IT and needed my password
shoulder surfing: A visitor watched me type my PIN at the printer
dumpster diving: Old invoices with customer data were found in the trash outside
phishing: Email with an invoice attachment from an unknown sender
unknown: escalate to security: USB stick left in the car park
Totals:
  2 phishing
  1 dumpster diving
  1 shoulder surfing
  1 smishing
  1 tailgating
  1 unknown: escalate to security
  1 vishing
  1 whaling
@@requires
def classify
.lower()
@@hint
Order matters: check the whaling clues before plain email, or every whaling report becomes phishing.
@@solution
reports = [
    "Email asking me to reset my password on a strange site",
    "Someone followed me through the badge door without scanning",
    "Text message said my parcel was held and asked for a card number",
    "Email to the CFO asking for an urgent wire transfer",
    "A caller said he was from IT and needed my password",
    "A visitor watched me type my PIN at the printer",
    "Old invoices with customer data were found in the trash outside",
    "Email with an invoice attachment from an unknown sender",
    "USB stick left in the car park",
]

def classify(report):
    text = report.lower()
    if any(w in text for w in ("ceo", "cfo", "executive")) and "email" in text:
        return "whaling"
    if "email" in text:
        return "phishing"
    if "text message" in text or "sms" in text:
        return "smishing"
    if "phone" in text or "call" in text:
        return "vishing"
    if "followed" in text or "held the door" in text:
        return "tailgating"
    if "over my shoulder" in text or "watched me type" in text:
        return "shoulder surfing"
    if "bin" in text or "trash" in text:
        return "dumpster diving"
    return "unknown: escalate to security"

counts = {}
for r in reports:
    kind = classify(r)
    counts[kind] = counts.get(kind, 0) + 1
    print(f"{kind}: {r}")
print("Totals:")
for kind, n in sorted(counts.items(), key=lambda kv: (-kv[1], kv[0])):
    print(f"  {n} {kind}")

@@exercise Identify the malware from its symptoms
Score each malware type in the lesson's table against a ticket's symptoms. Write `suspects(symptoms)` that adds up points for each type using `CLUES` (a symptom can point to several types) and returns the types with the highest score, sorted by name. Print the top suspects for each ticket and the first step of the removal process for that case: `disconnect from the network` if a worm, ransomware or botnet is among them, otherwise `quarantine the machine and disable System Restore`.
@@lang python
@@starter
CLUES = {
    "files renamed with odd extensions": {"ransomware": 3},
    "ransom note": {"ransomware": 3},
    "fans busy while idle": {"cryptominer": 2, "botnet": 1},
    "high GPU use": {"cryptominer": 2},
    "browser redirects": {"spyware": 2, "trojan": 1},
    "new toolbar": {"spyware": 1, "trojan": 2},
    "antivirus disabled": {"rootkit": 2, "trojan": 1},
    "files missing from task manager": {"rootkit": 3},
    "other PCs infected over the network": {"worm": 3},
    "unusual outbound traffic at night": {"botnet": 2, "worm": 1},
    "passwords stolen": {"spyware": 2},
    "installed a free game": {"trojan": 2},
}
tickets = {
    "T-201": ["files renamed with odd extensions", "ransom note"],
    "T-202": ["fans busy while idle", "high GPU use"],
    "T-203": ["installed a free game", "new toolbar", "browser redirects"],
    "T-204": ["antivirus disabled", "files missing from task manager"],
    "T-205": ["unusual outbound traffic at night", "fans busy while idle", "other PCs infected over the network"],
    "T-206": ["passwords stolen", "browser redirects"],
}
SPREADING = {"worm", "ransomware", "botnet"}

def suspects(symptoms):
    raise NotImplementedError


for ticket, symptoms in tickets.items():
    top = suspects(symptoms)
    first = "disconnect from the network" if SPREADING & set(top) else "quarantine the machine and disable System Restore"
    print(f"{ticket}: {', '.join(top)} -> first: {first}")
@@expected
T-201: ransomware -> first: disconnect from the network
T-202: cryptominer -> first: quarantine the machine and disable System Restore
T-203: trojan -> first: quarantine the machine and disable System Restore
T-204: rootkit -> first: quarantine the machine and disable System Restore
T-205: worm -> first: disconnect from the network
T-206: spyware -> first: quarantine the machine and disable System Restore
@@requires
def suspects
max(
@@hint
Add points into a dict per type, find the best score with `max(score.values())`, then keep every type with that score (there may be a tie).
@@solution
CLUES = {
    "files renamed with odd extensions": {"ransomware": 3},
    "ransom note": {"ransomware": 3},
    "fans busy while idle": {"cryptominer": 2, "botnet": 1},
    "high GPU use": {"cryptominer": 2},
    "browser redirects": {"spyware": 2, "trojan": 1},
    "new toolbar": {"spyware": 1, "trojan": 2},
    "antivirus disabled": {"rootkit": 2, "trojan": 1},
    "files missing from task manager": {"rootkit": 3},
    "other PCs infected over the network": {"worm": 3},
    "unusual outbound traffic at night": {"botnet": 2, "worm": 1},
    "passwords stolen": {"spyware": 2},
    "installed a free game": {"trojan": 2},
}
tickets = {
    "T-201": ["files renamed with odd extensions", "ransom note"],
    "T-202": ["fans busy while idle", "high GPU use"],
    "T-203": ["installed a free game", "new toolbar", "browser redirects"],
    "T-204": ["antivirus disabled", "files missing from task manager"],
    "T-205": ["unusual outbound traffic at night", "fans busy while idle", "other PCs infected over the network"],
    "T-206": ["passwords stolen", "browser redirects"],
}
SPREADING = {"worm", "ransomware", "botnet"}

def suspects(symptoms):
    score = {}
    for s in symptoms:
        for kind, pts in CLUES.get(s, {}).items():
            score[kind] = score.get(kind, 0) + pts
    if not score:
        return []
    top = max(score.values())
    return sorted(k for k, v in score.items() if v == top)

for ticket, symptoms in tickets.items():
    top = suspects(symptoms)
    first = "disconnect from the network" if SPREADING & set(top) else "quarantine the machine and disable System Restore"
    print(f"{ticket}: {', '.join(top)} -> first: {first}")
