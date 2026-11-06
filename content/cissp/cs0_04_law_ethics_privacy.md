@@lesson cs0-law-ethics-privacy Law, Ethics and Privacy: Canons, Regulations, Intellectual Property and Breach Notification (Domain 1)
@@concepts conditions, maps, functions
@@lang python
@@text
Domain 1 isn't only risk maths. A large share of exam questions (and of a security leader's real decisions) are about **law, ethics and privacy**: which canon wins when two duties conflict, which kind of intellectual property protects what, which regulation applies to which data, and how fast you must tell whom after a breach. This lesson covers each in the depth the exam expects, with a Japanese angle where it matters for work there.

## The ISC2 Code of Ethics

The **preamble** says the safety and welfare of society and the common good, duty to principals, and duty to each other require adherence to the highest ethical standards. The four **canons** are applied **in order**: when they conflict, the earlier one wins.

1. Protect society, the common good, necessary public trust and confidence, and the infrastructure.
2. Act honourably, honestly, justly, responsibly and legally.
3. Provide diligent and competent service to principals (employers, clients).
4. Advance and protect the profession.

A typical question: your employer asks you to keep quiet about a flaw that endangers the public. Canon 1 (society) outranks canon 3 (your employer). Members who see a breach of the code are expected to report it, and complaints must come from someone affected (for canons 1 and 2 anyone, for 3 principals, for 4 professionals).

## Kinds of law and investigations

- **Criminal** law: the state prosecutes, the burden is "beyond reasonable doubt", and the penalty can be prison.
- **Civil** (tort, contract) law: between parties, decided on the "preponderance of evidence", with damages as the remedy.
- **Administrative / regulatory** law: rules from agencies (financial regulators, privacy commissions). Fines and orders.
- Investigation types follow the same split: **administrative** (internal, policy violations), **criminal**, **civil** and **regulatory**. The evidence standard and chain-of-custody needs rise with the stakes, so treat every serious incident as if it could become criminal.

## Intellectual property

| Type | Protects | Duration (typical) | Notes |
|---|---|---|---|
| **Copyright** | Expression (code, text, images) | Author's life + 70 years | Automatic, no registration needed |
| **Trademark** | Names, logos, slogans | 10 years, renewable forever | Must be used and defended |
| **Patent** | Inventions (novel, useful, non-obvious) | 20 years from filing | Public disclosure in return for a monopoly |
| **Trade secret** | Confidential business information | As long as it stays secret | Protected by NDAs and controls, not registration |

Software licences: **EULAs**, open source licences (permissive like MIT, copyleft like GPL), and **click-wrap** / **shrink-wrap** agreements. Violating them is usually a civil matter, but large-scale piracy can be criminal.

## Privacy regulations

- **OECD privacy principles** (the basis for most laws): collection limitation, data quality, purpose specification, use limitation, security safeguards, openness, individual participation, accountability.
- **GDPR** (EU, also applies to organisations outside the EU offering services to people in the EU): lawful basis for processing, data subject rights (access, rectification, erasure, portability, objection), DPOs for large-scale processing, DPIAs for high-risk processing, fines up to 4% of global turnover or €20M. **Breach notification to the supervisory authority within 72 hours** of becoming aware, and to data subjects without undue delay if the risk to them is high.
- **APPI** (Japan's Act on the Protection of Personal Information, amended 2022): reports to the **Personal Information Protection Commission (PPC)** and notice to individuals are mandatory for breaches of sensitive data, data that could cause financial loss, breaches by unauthorised (malicious) acts, or more than 1,000 people. There is a **preliminary report promptly** (guidance: roughly within 3–5 days) and a **final report within 30 days**, or **60 days** when the breach came from an unauthorised act.
- Other names to recognise: **HIPAA** (US health data), **SOX** (US public company financial controls), **GLBA** (US financial institutions), **PCI DSS** (a contractual standard for card data, not a law), **CCPA/CPRA** (California).
- **Transborder data flows**: GDPR adequacy decisions (Japan has one, in both directions), standard contractual clauses, and the EU–US Data Privacy Framework.

## Import/export and licensing

- **Wassenaar Arrangement**: export controls on dual-use goods, including some cryptography and intrusion software.
- Some countries restrict import or use of encryption. Check before shipping products with strong crypto, or before staff travel with encrypted laptops.

@@quiz
Canon 3 (serve your principals) conflicts with canon 1 (protect society). Which wins?
* Canon 1, because the canons are applied in order
- Canon 3, because your employer pays you
- Whichever the manager chooses
- Neither; you must resign
? Earlier canons take precedence.

@@quiz
Which form of intellectual property protects a secret recipe for as long as it remains confidential?
- Patent
- Copyright
* Trade secret
- Trademark
? No registration, but you must protect it.

@@quiz
Under GDPR, how long does a controller have to notify the supervisory authority of a personal data breach?
* 72 hours after becoming aware of it
- 24 hours after it happened
- 30 days
- Only if more than 1,000 people are affected
? Data subjects too if the risk to them is high.

@@quiz
Under Japan's amended APPI, when is the final report due for a breach caused by an unauthorised (malicious) act?
- 72 hours
- 30 days
* 60 days
- There is no final report
? 30 days for other reportable breaches.

@@quiz
Which statement about PCI DSS is correct?
* It is a contractual industry standard, not a law
- It is an EU regulation
- It only applies to health data
- It replaces GDPR for card data
? Enforced through card brand contracts.

@@quiz
Which kind of case uses the "preponderance of evidence" standard?
- Criminal
* Civil
- Military
- None
? Criminal cases need proof beyond reasonable doubt.

@@quiz
A patent gives protection for roughly how long?
* 20 years from filing
- The inventor's life plus 70 years
- 10 years, renewable forever
- As long as it stays secret
? In return, the invention is published.

@@flashcards
canon 1 :: protect society, the common good, trust and infrastructure
canon 2 :: act honourably, honestly, justly, responsibly and legally
canon 3 :: diligent and competent service to principals
canon 4 :: advance and protect the profession
canon order :: earlier canons win conflicts
criminal / civil standard :: beyond reasonable doubt / preponderance of evidence
copyright :: expression, life + 70 years, automatic
trademark :: names and logos, renewable
patent :: inventions, 20 years
trade secret :: as long as it stays secret
OECD principles :: basis of most privacy laws
GDPR breach :: supervisory authority within 72 hours
APPI breach :: PPC preliminary promptly, final in 30 (60 if malicious) days
PCI DSS :: contractual standard for card data
Wassenaar :: export controls on dual-use tech, including crypto

@@drill
How many hours does GDPR allow for notifying the supervisory authority of a breach?
= 72
? From becoming aware of it.

@@drill
Which canon number is "provide diligent and competent service to principals"?
= 3
= three
? Society, honour, principals, profession.

@@exercise Breach notification deadlines
For each breach, work out the notification deadlines that apply. GDPR applies when EU residents are affected (72 hours after awareness). APPI applies to Japanese residents when the breach is reportable (sensitive data, possible financial loss, an unauthorised act, or more than 1,000 people): a preliminary report target of 3 days and a final report at 30 days, or 60 days for unauthorised acts. Print the deadlines sorted by date.
@@starter
from datetime import datetime, timedelta

breaches = [
    {"id": "B1", "aware": "2026-03-02 09:00", "regions": {"EU", "JP"}, "people": 2400, "sensitive": False, "financial": False, "malicious": True},
    {"id": "B2", "aware": "2026-03-10 18:30", "regions": {"JP"}, "people": 300, "sensitive": False, "financial": False, "malicious": False},
    {"id": "B3", "aware": "2026-04-01 14:00", "regions": {"JP"}, "people": 40, "sensitive": True, "financial": False, "malicious": False},
    {"id": "B4", "aware": "2026-04-20 08:15", "regions": {"EU"}, "people": 12, "sensitive": False, "financial": True, "malicious": False},
]

def appi_reportable(b):
    raise NotImplementedError

def deadlines(b):
    raise NotImplementedError

all_items = []
for b in breaches:
    items = deadlines(b)
    if not items:
        print(f"{b['id']}: no mandatory report (keep an internal record)")
    all_items += [(when, b["id"], what) for what, when in items]
for when, bid, what in sorted(all_items):
    print(f"{when:%Y-%m-%d %H:%M}  {bid}  {what}")
@@expected
B2: no mandatory report (keep an internal record)
2026-03-05 09:00  B1  APPI preliminary report to the PPC
2026-03-05 09:00  B1  GDPR supervisory authority
2026-04-04 14:00  B3  APPI preliminary report to the PPC
2026-04-23 08:15  B4  GDPR supervisory authority
2026-05-01 09:00  B1  APPI final report (60 days)
2026-05-01 14:00  B3  APPI final report (30 days)
@@requires
def appi_reportable
def deadlines
timedelta(hours=72)
@@hint
`appi_reportable` is `"JP" in regions and (sensitive or financial or malicious or people > 1000)`. In `deadlines`, parse `aware` with `datetime.strptime(s, "%Y-%m-%d %H:%M")`, then add `("GDPR supervisory authority", aware + timedelta(hours=72))` when "EU" is in the regions, and the APPI preliminary (3 days) and final (30 or 60 days) reports when reportable.
@@solution
from datetime import datetime, timedelta

breaches = [
    {"id": "B1", "aware": "2026-03-02 09:00", "regions": {"EU", "JP"}, "people": 2400, "sensitive": False, "financial": False, "malicious": True},
    {"id": "B2", "aware": "2026-03-10 18:30", "regions": {"JP"}, "people": 300, "sensitive": False, "financial": False, "malicious": False},
    {"id": "B3", "aware": "2026-04-01 14:00", "regions": {"JP"}, "people": 40, "sensitive": True, "financial": False, "malicious": False},
    {"id": "B4", "aware": "2026-04-20 08:15", "regions": {"EU"}, "people": 12, "sensitive": False, "financial": True, "malicious": False},
]

def appi_reportable(b):
    return "JP" in b["regions"] and (b["sensitive"] or b["financial"] or b["malicious"] or b["people"] > 1000)

def deadlines(b):
    aware = datetime.strptime(b["aware"], "%Y-%m-%d %H:%M")
    items = []
    if "EU" in b["regions"]:
        items.append(("GDPR supervisory authority", aware + timedelta(hours=72)))
    if appi_reportable(b):
        items.append(("APPI preliminary report to the PPC", aware + timedelta(days=3)))
        final = 60 if b["malicious"] else 30
        items.append((f"APPI final report ({final} days)", aware + timedelta(days=final)))
    return items

all_items = []
for b in breaches:
    items = deadlines(b)
    if not items:
        print(f"{b['id']}: no mandatory report (keep an internal record)")
    all_items += [(when, b["id"], what) for what, when in items]
for when, bid, what in sorted(all_items):
    print(f"{when:%Y-%m-%d %H:%M}  {bid}  {what}")

@@exercise Resolve an ethics dilemma by canon
Each option in a dilemma is tagged with the canon it upholds and whether it breaks the law. Pick the option that serves the highest-priority canon (lowest number) without breaking the law; break ties by preferring the option that also serves a second canon. Print the choice and why.
@@starter
CANONS = {1: "protect society and infrastructure", 2: "act honourably, honestly and legally", 3: "serve principals diligently", 4: "advance the profession"}

dilemmas = {
    "Unsafe flaw the client wants hidden": [
        {"text": "Stay quiet as the client asked", "canons": [3], "illegal": False},
        {"text": "Report it to the regulator through proper channels after telling the client", "canons": [1, 2], "illegal": False},
        {"text": "Post the exploit publicly today", "canons": [1], "illegal": True},
    ],
    "Colleague padding their CV with a fake cert": [
        {"text": "Ignore it", "canons": [3], "illegal": False},
        {"text": "File a complaint with ISC2 with the evidence", "canons": [4, 2], "illegal": False},
    ],
    "Manager asks you to skip a penetration test scope review": [
        {"text": "Skip it to meet the deadline", "canons": [3], "illegal": False},
        {"text": "Do the review and explain the delay honestly", "canons": [2, 3], "illegal": False},
        {"text": "Test systems outside the agreed scope to save time", "canons": [3], "illegal": True},
    ],
}

def best_option(options):
    raise NotImplementedError

for name, options in dilemmas.items():
    choice = best_option(options)
    top = min(choice["canons"])
    print(f"{name}\n  -> {choice['text']} (canon {top}: {CANONS[top]})")
@@expected
Unsafe flaw the client wants hidden
  -> Report it to the regulator through proper channels after telling the client (canon 1: protect society and infrastructure)
Colleague padding their CV with a fake cert
  -> File a complaint with ISC2 with the evidence (canon 2: act honourably, honestly and legally)
Manager asks you to skip a penetration test scope review
  -> Do the review and explain the delay honestly (canon 2: act honourably, honestly and legally)
@@requires
def best_option
min(
@@hint
Filter out illegal options, then sort by a key: `(min(o["canons"]), -len(o["canons"]))`. The first one wins.
@@solution
CANONS = {1: "protect society and infrastructure", 2: "act honourably, honestly and legally", 3: "serve principals diligently", 4: "advance the profession"}

dilemmas = {
    "Unsafe flaw the client wants hidden": [
        {"text": "Stay quiet as the client asked", "canons": [3], "illegal": False},
        {"text": "Report it to the regulator through proper channels after telling the client", "canons": [1, 2], "illegal": False},
        {"text": "Post the exploit publicly today", "canons": [1], "illegal": True},
    ],
    "Colleague padding their CV with a fake cert": [
        {"text": "Ignore it", "canons": [3], "illegal": False},
        {"text": "File a complaint with ISC2 with the evidence", "canons": [4, 2], "illegal": False},
    ],
    "Manager asks you to skip a penetration test scope review": [
        {"text": "Skip it to meet the deadline", "canons": [3], "illegal": False},
        {"text": "Do the review and explain the delay honestly", "canons": [2, 3], "illegal": False},
        {"text": "Test systems outside the agreed scope to save time", "canons": [3], "illegal": True},
    ],
}

def best_option(options):
    legal = [o for o in options if not o["illegal"]]
    return sorted(legal, key=lambda o: (min(o["canons"]), -len(o["canons"])))[0]

for name, options in dilemmas.items():
    choice = best_option(options)
    top = min(choice["canons"])
    print(f"{name}\n  -> {choice['text']} (canon {top}: {CANONS[top]})")
