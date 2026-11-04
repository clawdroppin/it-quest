@@lesson sp2-social Social Engineering, Phishing and Email Security
@@text
Most breaches start with a person being tricked. Domain 2 (threats, 22%) lists the social engineering techniques by name, and domain 4 asks how email security controls and awareness training reduce them. This lesson covers both, so you can recognise an attack and say which control stops it.

## Why it works: the principles

Attackers exploit **authority** ("This is the CEO"), **urgency** ("in the next 30 minutes"), **scarcity**, **social proof** ("everyone in finance has done this"), **familiarity/liking**, **intimidation** and **trust**. Training teaches people to notice those pressures.

## Techniques

| Technique | Description |
|---|---|
| **Phishing** | mass email pretending to be a trusted sender, with a malicious link or attachment |
| **Spear phishing** | targeted at a specific person using researched details |
| **Whaling** | spear phishing aimed at executives |
| **Vishing** | voice phishing by phone ("IT support here, I need your code") |
| **Smishing** | SMS phishing (fake delivery notices) |
| **Business email compromise (BEC)** | a real or spoofed executive/supplier mailbox asks for payments or bank detail changes |
| **Pretexting** | an invented scenario to justify the request |
| **Impersonation** / **brand impersonation** | pretending to be a person or a well-known company |
| **Watering hole** | compromise a website the targets visit |
| **Typosquatting** | registering `rnicrosoft.com`-style look-alike domains |
| **Misinformation / disinformation** | false information spread (by mistake / deliberately) |
| **Tailgating / piggybacking** | following someone through a secure door (piggybacking: with their consent) |
| **Shoulder surfing** | watching someone type |
| **Dumpster diving** | searching rubbish for information |
| **MFA fatigue / push bombing** | repeated MFA prompts until the user approves one |
| **QR code phishing (quishing)** | malicious QR codes in emails or on posters |

## Spotting a phishing email

- Sender's **display name** doesn't match the **actual address** or domain.
- **Reply-To** differs from From.
- Links whose real destination (hover) differs from the text; look-alike domains.
- Urgency, threats, unusual requests (gift cards, bank changes, password "verification").
- Unexpected attachments, especially macros, HTML, ISO/ZIP files.
- **Authentication results** in headers: `spf=fail`, `dkim=fail`, `dmarc=fail`.

## Email security controls

- **SPF**: a DNS TXT record listing which servers may send mail for the domain.
- **DKIM**: the sending server signs messages; the public key is in DNS. Proves the message wasn't altered and came from the domain.
- **DMARC**: tells receivers what to do when SPF/DKIM fail (`p=none`, `quarantine`, `reject`) and where to send reports. **Alignment** requires the From domain to match.
- **Secure email gateway**: filters spam, malware, malicious links (URL rewriting, sandboxing attachments).
- **External sender banners** ("This email came from outside the organisation").
- **S/MIME** and **PGP** for signing and encrypting messages end to end.

## Process controls

- **Verify payment and bank detail changes out of band** (call a known number, not the one in the email). This one control stops most BEC losses.
- **Number matching** for MFA push (defeats MFA fatigue) and phishing-resistant MFA (**FIDO2/passkeys**).
- **Report phishing** button and a clear, blame-free reporting process.
- **Least privilege** so a phished account can do less damage.

## Security awareness training

The objectives list: phishing **campaigns** (simulated phishing), recognising **anomalous behaviour** (risky, unexpected, unintentional), **user guidance** (policy/handbooks, situational awareness, insider threat, password management, removable media, social engineering, operational security, hybrid/remote work environments), **reporting and monitoring** (initial and recurring), and **development and execution** of the programme. Measure click rates and report rates over time: a rising report rate is the real success indicator.

@@quiz
The finance team gets an email from the CEO's real-looking address asking to urgently pay a new supplier. What's the best control?
- A stronger spam filter only
* Verify the request out of band using a known phone number before paying
- Reply to the email to confirm
- Forward it to the CEO's personal address
? This is business email compromise.

@@quiz
Which email authentication record tells receiving servers to reject mail that fails checks?
- SPF
- DKIM
* DMARC with p=reject
- MX
? DMARC sets the policy and reporting.

@@quiz
A user receives 40 MFA push prompts at 2 a.m. and finally taps Approve. What attack is this, and what helps?
* MFA fatigue; number matching or phishing-resistant MFA
- Vishing; caller ID
- Whaling; executive training
- Typosquatting; DNS filtering
? Push bombing.

@@quiz
Attackers compromise an industry forum that a company's engineers visit. What is this technique?
- Pretexting
* Watering hole attack
- Smishing
- Shoulder surfing
? They wait for the targets to come to them.

@@quiz
Which is the best measure of a phishing awareness programme's success over time?
- Number of emails sent
* Rising report rate and falling click rate in simulations
- Number of employees fired
- Length of the training video
? Reporting turns users into sensors.

@@flashcards
phishing :: mass fraudulent email
spear phishing :: targeted at a person
whaling :: targeted at executives
vishing / smishing :: voice / SMS phishing
BEC :: business email compromise; payment fraud
pretexting :: invented scenario to justify a request
watering hole :: compromise a site the target visits
typosquatting :: look-alike domains
MFA fatigue :: repeated prompts until approved
quishing :: QR code phishing
SPF :: allowed sending servers
DKIM :: message signature, key in DNS
DMARC :: policy for SPF/DKIM failures
number matching :: defeats MFA fatigue
FIDO2/passkeys :: phishing-resistant MFA
out-of-band verification :: call a known number

@@drill
Which DMARC policy value tells receivers to refuse failing mail?
= reject
= p=reject
? `quarantine` sends it to spam; `none` only reports.

@@drill
What is phishing by SMS called?
= smishing
? Vishing is by voice.

@@exercise Score a suspicious email
Score emails for phishing indicators: display name contains a known executive but the domain isn't `example.com` (+3), Reply-To domain differs from From domain (+2), any of SPF/DKIM/DMARC `fail` (+2 each), urgency words in the subject (+1 each, from `URGENCY`), a link whose host doesn't match its displayed text's host (+3), a risky attachment extension (+3). Print the score and a verdict: 6+ `quarantine`, 3–5 `warn user`, otherwise `deliver`.
@@lang python
@@starter
EXECUTIVES = {"Hana Mori", "David Chen"}
URGENCY = ["urgent", "immediately", "today", "final notice"]
RISKY = (".iso", ".zip", ".html", ".xlsm", ".exe")
emails = [
    {"from_name": "Hana Mori", "from": "hana.mori@examp1e-mail.com", "reply_to": "pay@quickmail.biz", "subject": "URGENT: wire needed today",
     "auth": {"spf": "fail", "dkim": "none", "dmarc": "fail"}, "links": [("https://portal.example.com", "https://portal.example.com.login-check.biz")], "attachments": []},
    {"from_name": "IT Service Desk", "from": "servicedesk@example.com", "reply_to": "servicedesk@example.com", "subject": "Planned maintenance Saturday",
     "auth": {"spf": "pass", "dkim": "pass", "dmarc": "pass"}, "links": [("https://status.example.com", "https://status.example.com")], "attachments": []},
    {"from_name": "Courier Japan", "from": "notice@courier-jp.example.net", "reply_to": "notice@courier-jp.example.net", "subject": "Final notice: parcel on hold",
     "auth": {"spf": "pass", "dkim": "pass", "dmarc": "pass"}, "links": [], "attachments": ["label.iso"]},
]
@@expected
URGENT: wire needed today: 14 -> quarantine
  - executive name from external domain
  - Reply-To differs
  - spf fail
  - dmarc fail
  - urgency 'urgent'
  - urgency 'today'
  - link goes to portal.example.com.login-check.biz
Planned maintenance Saturday: 0 -> deliver
Final notice: parcel on hold: 4 -> warn user
  - urgency 'final notice'
  - risky attachment label.iso
@@requires
urlparse(
EXECUTIVES
@@hint
`urlparse(url).hostname` gives the host; split addresses with `addr.split("@")[1]`.
@@solution
from urllib.parse import urlparse

EXECUTIVES = {"Hana Mori", "David Chen"}
URGENCY = ["urgent", "immediately", "today", "final notice"]
RISKY = (".iso", ".zip", ".html", ".xlsm", ".exe")
emails = [
    {"from_name": "Hana Mori", "from": "hana.mori@examp1e-mail.com", "reply_to": "pay@quickmail.biz", "subject": "URGENT: wire needed today",
     "auth": {"spf": "fail", "dkim": "none", "dmarc": "fail"}, "links": [("https://portal.example.com", "https://portal.example.com.login-check.biz")], "attachments": []},
    {"from_name": "IT Service Desk", "from": "servicedesk@example.com", "reply_to": "servicedesk@example.com", "subject": "Planned maintenance Saturday",
     "auth": {"spf": "pass", "dkim": "pass", "dmarc": "pass"}, "links": [("https://status.example.com", "https://status.example.com")], "attachments": []},
    {"from_name": "Courier Japan", "from": "notice@courier-jp.example.net", "reply_to": "notice@courier-jp.example.net", "subject": "Final notice: parcel on hold",
     "auth": {"spf": "pass", "dkim": "pass", "dmarc": "pass"}, "links": [], "attachments": ["label.iso"]},
]

for e in emails:
    score, reasons = 0, []
    from_domain = e["from"].split("@")[1]
    if e["from_name"] in EXECUTIVES and from_domain != "example.com":
        score += 3
        reasons.append("executive name from external domain")
    if e["reply_to"].split("@")[1] != from_domain:
        score += 2
        reasons.append("Reply-To differs")
    for check, result in e["auth"].items():
        if result == "fail":
            score += 2
            reasons.append(f"{check} fail")
    for word in URGENCY:
        if word in e["subject"].lower():
            score += 1
            reasons.append(f"urgency '{word}'")
    for shown, actual in e["links"]:
        if urlparse(shown).hostname != urlparse(actual).hostname:
            score += 3
            reasons.append(f"link goes to {urlparse(actual).hostname}")
    for name in e["attachments"]:
        if name.lower().endswith(RISKY):
            score += 3
            reasons.append(f"risky attachment {name}")
    verdict = "quarantine" if score >= 6 else "warn user" if score >= 3 else "deliver"
    print(f"{e['subject']}: {score} -> {verdict}")
    for r in reasons:
        print(f"  - {r}")

@@exercise Track a phishing simulation campaign
Given results from three monthly simulations, compute each month's click rate and report rate (as percentages of emails delivered), print them, and say whether the programme is improving: report rate went up **and** click rate went down from the first to the last month. Also list the departments whose click rate in the last month was above the company's last-month rate.
@@lang python
@@starter
campaigns = {
    "2026-08": {"Finance": (40, 12, 5), "Sales": (60, 15, 6), "Engineering": (80, 6, 20), "HR": (20, 4, 3)},
    "2026-09": {"Finance": (40, 8, 9), "Sales": (60, 13, 8), "Engineering": (80, 5, 28), "HR": (20, 2, 6)},
    "2026-10": {"Finance": (40, 3, 15), "Sales": (60, 11, 12), "Engineering": (80, 3, 35), "HR": (20, 1, 9)},
}
@@expected
2026-08: click 18.5%, report 17.0%
2026-09: click 14.0%, report 25.5%
2026-10: click 9.0%, report 35.5%
Improving
Above average clicks: Sales
@@requires
:.1f
sorted(
@@hint
Each tuple is `(delivered, clicked, reported)`. Sum them across departments for the month totals.
@@solution
campaigns = {
    "2026-08": {"Finance": (40, 12, 5), "Sales": (60, 15, 6), "Engineering": (80, 6, 20), "HR": (20, 4, 3)},
    "2026-09": {"Finance": (40, 8, 9), "Sales": (60, 13, 8), "Engineering": (80, 5, 28), "HR": (20, 2, 6)},
    "2026-10": {"Finance": (40, 3, 15), "Sales": (60, 11, 12), "Engineering": (80, 3, 35), "HR": (20, 1, 9)},
}

rates = {}
for month in sorted(campaigns):
    depts = campaigns[month]
    delivered = sum(d for d, _, _ in depts.values())
    clicked = sum(c for _, c, _ in depts.values())
    reported = sum(r for _, _, r in depts.values())
    click, report = clicked / delivered * 100, reported / delivered * 100
    rates[month] = (click, report)
    print(f"{month}: click {click:.1f}%, report {report:.1f}%")

months = sorted(rates)
first, last = rates[months[0]], rates[months[-1]]
improving = last[1] > first[1] and last[0] < first[0]
print("Improving" if improving else "Not improving")

last_month = campaigns[months[-1]]
risky = [name for name, (d, c, _) in last_month.items() if c / d * 100 > last[0]]
print("Above average clicks:", ", ".join(risky) if risky else "none")
