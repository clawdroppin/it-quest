@@lesson jc1-visa-changes-pr Keeping Your Status: Job Changes, Renewals, Re-entry and Permanent Residence
@@concepts conditions, numbers, functions
@@lang python
@@text
Getting your first work visa is only the start. Over the following years you'll change jobs, renew your period of stay, travel abroad and perhaps apply for **permanent residence (永住許可)**. Each step has deadlines and paperwork, and missing one can hurt your next application. This lesson is general information, not legal advice: rules change, so check the Immigration Services Agency (出入国在留管理庁) website or ask an administrative scrivener (行政書士) for your own case.

## Changing jobs on a work visa

- Your status (e.g. 技術・人文知識・国際業務, "Engineer/Specialist in Humanities/International Services") is tied to the **kind of work**, not to one employer. You can usually change jobs without a new visa if the new work fits the same status.
- Within **14 days** of leaving and of joining, you must notify immigration of the change of contracting organisation (契約機関に関する届出). It can be done online through the immigration e-notification system, by post or at a regional office.
- To reduce risk at your next renewal, you can apply for a **certificate of authorised employment (就労資格証明書)**, which confirms that the new job fits your status.
- Unemployed for more than **3 months** without a good reason (actively job hunting counts) can be grounds for your status to be cancelled. Keep evidence of your job search.
- Side jobs outside your status need **permission to engage in an activity other than that permitted (資格外活動許可)**. HSP holders have different rules.

## Renewing your period of stay (在留期間更新)

- You can apply from about **3 months before** your period of stay expires.
- If you applied before expiry, you may stay legally while the decision is pending, until it's decided or **2 months after** the original expiry date, whichever comes first (the special period, 特例期間).
- Documents: the application form, a photo, your passport and residence card, and from the employer, documents about the company and your job. You'll also usually need your **resident tax certificates (課税証明書・納税証明書)**.
- Periods granted are 3 months, 1 year, 3 years or 5 years. Longer periods come with stable employment, paid taxes and a good record.

## Travelling: re-entry

- With a valid residence card, a **special re-entry permit (みなし再入国許可)** lets you leave and return within **1 year** (or before your period of stay ends, if sooner) without applying in advance. Tick the box on the embarkation card or at the automated gate.
- For longer absences, apply for a regular **re-entry permit (再入国許可)** before leaving (up to 5 years).
- If you leave without either, your status ends when you leave.

## Permanent residence (永住許可)

General requirements:
- **Good conduct**: no serious offences, traffic violations kept minimal.
- **Independent livelihood**: stable income enough to support yourself and family.
- **In Japan's interest**: normally **10 years** of continuous residence, including at least **5 years** on a work or residence status, and your current status granted for the **longest period available (in practice, 3 years counts)**.
- **Taxes, pension and health insurance paid properly and on time.** Late payment is one of the most common reasons for refusal. Keep the receipts and use payroll deduction where possible.

Shorter routes (exceptions to the 10-year rule):
- **HSP points 70+**: 3 years of residence at 70+ points. **80+ points**: 1 year at 80+ points. (These can be counted from points you held during that time, even before you had HSP status.)
- **Spouse of a Japanese national or a permanent resident**: married for 3 years and living in Japan for 1 year.
- **Long-term resident (定住者)**: 5 years.

Notes:
- Processing commonly takes many months, so keep your current status renewed while waiting.
- You need a guarantor (身元保証人), usually a Japanese national or permanent resident.
- A 2024 amendment to the immigration law allows permanent residence to be revoked for deliberate non-payment of taxes or social insurance, which makes paying them on time even more important.

@@quiz
You change from one IT company to another and the work is similar. What must you do?
* Notify immigration of the change of contracting organisation within 14 days
- Apply for a new visa before starting
- Nothing at all
- Leave Japan and re-enter
? You can also get a certificate of authorised employment.

@@quiz
How early can you usually apply to renew your period of stay?
* About 3 months before it expires
- 1 year before
- Only after it expires
- 1 week before
? Apply early.

@@quiz
You applied for renewal before expiry, and your card expires while immigration is still deciding. What happens?
* You can stay legally until the decision or 2 months after the expiry, whichever is first
- You must leave immediately
- You become a permanent resident
- You must reapply from abroad
? The special period (特例期間).

@@quiz
You travel abroad for 8 months with a valid residence card. What do you need?
* Nothing in advance: the special re-entry permit covers up to 1 year
- A regular re-entry permit
- A new visa
- A tourist visa
? Tick the box when leaving.

@@quiz
Which HSP points total allows a permanent residence application after 1 year?
- 50
- 70
* 80
- 100
? 70 points: 3 years.

@@quiz
What is one of the most common reasons permanent residence applications are refused?
* Late or missing tax, pension or health insurance payments
- Not speaking Japanese fluently
- Changing jobs once
- Travelling abroad for a holiday
? Pay on time and keep the records.

@@flashcards
14-day notification :: change of contracting organisation (契約機関に関する届出)
就労資格証明書 :: certificate that new work fits your status
資格外活動許可 :: permission for work outside your status
在留期間更新 :: renewal of the period of stay
renewal timing :: from about 3 months before expiry
特例期間 :: stay legal up to 2 months after expiry while pending
みなし再入国許可 :: special re-entry, within 1 year
再入国許可 :: regular re-entry permit, up to 5 years
永住許可 :: permanent residence
standard PR rule :: 10 years, 5 on a work status
HSP 70 / 80 :: PR after 3 years / 1 year
spouse route :: married 3 years, 1 year in Japan
身元保証人 :: guarantor for the application
課税証明書 / 納税証明書 :: tax assessed / tax paid certificates
行政書士 :: administrative scrivener, visa paperwork specialist

@@drill
How many days do you have to notify immigration after changing employers?
= 14
? For leaving and for joining.

@@drill
With the special re-entry permit, within how many years must you return? (number only)
= 1
? Or before your status expires.

@@exercise Plan your immigration deadlines
Given your residence card expiry and the dates you leave and join employers, list the deadlines: the 14-day notifications, the earliest renewal application date (3 months before expiry, same day of the month), and the last day of the special period if you apply but are still waiting (2 months after expiry). Flag a job gap longer than 3 months.
@@starter
from datetime import date, timedelta

expiry = date(2027, 4, 15)
left_job = date(2026, 9, 30)
joined_job = date(2026, 11, 1)

def add_months(d, months):
    raise NotImplementedError

def deadlines(expiry, left, joined):
    raise NotImplementedError

for when, what in deadlines(expiry, left_job, joined_job):
    print(f"{when:%Y-%m-%d (%a)}  {what}")
@@expected
2026-10-14 (Wed)  notify immigration: left the previous employer
2026-11-15 (Sun)  notify immigration: joined the new employer
2027-01-15 (Fri)  earliest day to apply for renewal
2027-04-15 (Thu)  period of stay expires (apply before this)
2027-06-15 (Tue)  special period ends if the decision is still pending
@@requires
def add_months
def deadlines
timedelta(days=14)
@@hint
`add_months`: `m = d.month - 1 + months`, `year = d.year + m // 12`, `month = m % 12 + 1`, and clamp the day to the length of that month (use `calendar.monthrange`). Build a list of `(date, text)` with `left + timedelta(days=14)`, `joined + timedelta(days=14)`, `add_months(expiry, -3)`, the expiry itself and `add_months(expiry, 2)`. A gap is too long if `add_months(left, 3) < joined`. Sort the list.
@@solution
import calendar
from datetime import date, timedelta

expiry = date(2027, 4, 15)
left_job = date(2026, 9, 30)
joined_job = date(2026, 11, 1)

def add_months(d, months):
    m = d.month - 1 + months
    year, month = d.year + m // 12, m % 12 + 1
    day = min(d.day, calendar.monthrange(year, month)[1])
    return date(year, month, day)

def deadlines(expiry, left, joined):
    items = [
        (left + timedelta(days=14), "notify immigration: left the previous employer"),
        (joined + timedelta(days=14), "notify immigration: joined the new employer"),
        (add_months(expiry, -3), "earliest day to apply for renewal"),
        (expiry, "period of stay expires (apply before this)"),
        (add_months(expiry, 2), "special period ends if the decision is still pending"),
    ]
    if add_months(left, 3) < joined:
        items.append((add_months(left, 3), "WARNING: more than 3 months without work: keep job-search evidence"))
    return sorted(items)

for when, what in deadlines(expiry, left_job, joined_job):
    print(f"{when:%Y-%m-%d (%a)}  {what}")

@@exercise Permanent residence eligibility check
Check each person against the main permanent residence routes: the standard route (10 years' residence with 5 on a work status), HSP 80 points held for 1 year, HSP 70 points held for 3 years, and spouse of a Japanese national (3 years married, 1 year in Japan). Every route also requires on-time tax and insurance payments and a current status granted for 3 years or more. Print the routes that work, or what's missing.
@@starter
people = [
    {"name": "Aiko", "years": 11, "work_years": 9, "hsp_points": 60, "hsp_years": 0, "married_years": 0, "period": 5, "on_time": True},
    {"name": "Ben", "years": 1.5, "work_years": 1.5, "hsp_points": 85, "hsp_years": 1.2, "married_years": 0, "period": 5, "on_time": True},
    {"name": "Chen", "years": 4, "work_years": 4, "hsp_points": 75, "hsp_years": 2.5, "married_years": 0, "period": 3, "on_time": True},
    {"name": "Dana", "years": 2, "work_years": 2, "hsp_points": 40, "hsp_years": 0, "married_years": 3.5, "period": 3, "on_time": True},
    {"name": "Eiji", "years": 12, "work_years": 12, "hsp_points": 50, "hsp_years": 0, "married_years": 0, "period": 1, "on_time": False},
]

def routes(p):
    raise NotImplementedError

def missing(p):
    raise NotImplementedError

for p in people:
    ok = routes(p)
    gaps = missing(p)
    if ok and not gaps:
        print(f"{p['name']}: eligible via {', '.join(ok)}")
    else:
        reasons = gaps + ([] if ok else ["no route's residence requirement met yet"])
        print(f"{p['name']}: not yet: {'; '.join(reasons)}")
@@expected
Aiko: eligible via standard
Ben: eligible via HSP 80
Chen: not yet: no route's residence requirement met yet
Dana: eligible via spouse
Eiji: not yet: taxes or insurance paid late: pay on time for the coming years first; current period of stay is 1 year(s); 3 or more is needed
@@requires
def routes
def missing
"HSP 80"
@@hint
`routes` returns the names whose conditions hold: "standard" (`years >= 10 and work_years >= 5`), "HSP 80" (`hsp_points >= 80 and hsp_years >= 1`), "HSP 70" (`hsp_points >= 70 and hsp_years >= 3`), "spouse" (`married_years >= 3 and years >= 1`). `missing` returns the general problems: late payments, and a period shorter than 3 years.
@@solution
people = [
    {"name": "Aiko", "years": 11, "work_years": 9, "hsp_points": 60, "hsp_years": 0, "married_years": 0, "period": 5, "on_time": True},
    {"name": "Ben", "years": 1.5, "work_years": 1.5, "hsp_points": 85, "hsp_years": 1.2, "married_years": 0, "period": 5, "on_time": True},
    {"name": "Chen", "years": 4, "work_years": 4, "hsp_points": 75, "hsp_years": 2.5, "married_years": 0, "period": 3, "on_time": True},
    {"name": "Dana", "years": 2, "work_years": 2, "hsp_points": 40, "hsp_years": 0, "married_years": 3.5, "period": 3, "on_time": True},
    {"name": "Eiji", "years": 12, "work_years": 12, "hsp_points": 50, "hsp_years": 0, "married_years": 0, "period": 1, "on_time": False},
]

def routes(p):
    found = []
    if p["years"] >= 10 and p["work_years"] >= 5:
        found.append("standard")
    if p["hsp_points"] >= 80 and p["hsp_years"] >= 1:
        found.append("HSP 80")
    if p["hsp_points"] >= 70 and p["hsp_years"] >= 3:
        found.append("HSP 70")
    if p["married_years"] >= 3 and p["years"] >= 1:
        found.append("spouse")
    return found

def missing(p):
    gaps = []
    if not p["on_time"]:
        gaps.append("taxes or insurance paid late: pay on time for the coming years first")
    if p["period"] < 3:
        gaps.append(f"current period of stay is {p['period']} year(s); 3 or more is needed")
    return gaps

for p in people:
    ok = routes(p)
    gaps = missing(p)
    if ok and not gaps:
        print(f"{p['name']}: eligible via {', '.join(ok)}")
    else:
        reasons = gaps + ([] if ok else ["no route's residence requirement met yet"])
        print(f"{p['name']}: not yet: {'; '.join(reasons)}")
