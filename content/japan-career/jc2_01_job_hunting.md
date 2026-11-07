@@lesson jc2-job-hunting Job Hunting: Documents, Sites, Interviews and Offers
@@text
## Where to look

| Kind | Examples | Notes |
|---|---|---|
| Japanese job sites | **doda**, **リクナビNEXT**, **マイナビ転職**, **Green** (IT/startups), **Wantedly** | huge choice; Japanese required |
| Scouting | **ビズリーチ**, LinkedIn, doda X | companies and recruiters contact you |
| English-friendly | **TokyoDev**, **Japan Dev**, **GaijinPot Jobs**, **Daijob**, **CareerCross**, LinkedIn | roles open to foreigners, often with English |
| Recruiters | エージェント (Robert Walters, JAC, Hays, en world, doda/リクルートエージェント) | **free for you** (the company pays); they coach you and negotiate |
| Direct | company career pages (採用情報), meetups, connpass events | networking works in Japan too |

## The documents

- **履歴書** (rirekisho): a standard form: personal details, photo, education, work history, qualifications, motivation.
- **職務経歴書** (shokumu keirekisho): your real CV, free format: summary, each job with projects, tools, achievements (with numbers), skills, qualifications, 自己PR. For IT roles this is the one that matters.
- A **GitHub** profile with the projects from this app is a strong addition. Japanese recruiters do look.

Both templates are in this track's project.

## The process (選考)

**応募** (apply) → **書類選考** (document screening, ~1 week) → **適性検査** (aptitude test like **SPI**, at some companies) → **一次面接** (first interview, often with HR or the team) → **二次面接** → **最終面接** (final, often with an executive) → **内定** (offer) → **内定承諾** (you accept) → **入社** (start). Two to six weeks overall is typical for mid-career hiring (中途採用). New-graduate hiring (新卒採用) is a separate, year-long system.

## Reading an offer (労働条件通知書)

| Item | Watch for |
|---|---|
| **雇用形態** | **正社員** (regular, permanent), **契約社員** (fixed-term contract), **派遣社員** (dispatched by an agency), **業務委託** (freelance: no employee protections) |
| **給与** | **月給** (monthly) × 12 + **賞与** (bonus, often 2-4 months' salary a year, not guaranteed) vs **年俸制** (annual salary, divided by 12 or 14) |
| **みなし残業 / 固定残業代** | "includes 45 hours of overtime pay": that money is for overtime you'll be expected to do. Divide by the real hours! Overtime beyond it must be paid extra |
| **所定労働時間** | scheduled hours, usually 7.5–8 h/day |
| **試用期間** | probation, often 3–6 months, sometimes at lower pay |
| **交通費** | commuting costs: usually paid in full (up to a limit) |
| **社会保険** | health insurance, pension, employment insurance: must be enrolled for full-time employees |
| **休日** | 完全週休2日制 (every weekend off) vs 週休2日制 (at least one week a month with two days off: not the same!), 年間休日 (annual days off: 120+ is good) |
| **退職金** | retirement allowance, at some companies |

The project's offer checker turns this into numbers: **what does one hour of your work actually pay?**

## Interviews

Prepare in Japanese: 自己紹介 (1 minute), 志望動機 (why this company: specific!), 転職理由 (why you're changing: positive), strengths/weaknesses, a story of solving a problem, and **逆質問** (your questions for them: always have two). Online interviews are common for first rounds; final rounds are often in person. Wear a dark suit unless told otherwise (IT startups are relaxed, SIers are not).

@@quiz
Who usually pays a recruitment agency (エージェント) in Japan?
* The hiring company
- The candidate
- The government
- Both equally
? Agencies are free for job seekers.

@@quiz
Which document matters most for judging an IT candidate's skills?
* 職務経歴書 (career history)
- 履歴書 photo
- 在留カード
- 住民票
? It lists projects, tools and achievements.

@@quiz
An offer says 月給30万円 (固定残業代45時間分を含む). What does it mean?
* The ¥300,000 already includes pay for 45 hours of overtime per month
- You get ¥300,000 plus 45 hours of overtime pay
- Overtime is forbidden
- You work 45 hours a week
? Compare offers by pay per hour actually worked.

@@quiz
What's the difference between 完全週休2日制 and 週休2日制?
* 完全 means every week has two days off; without it, only at least one week a month must
- They're identical
- 週休2日制 means more holidays
- 完全 means no holidays
? Read the 年間休日 number too: 120+ is typical for good conditions.

@@quiz
Which employment type has the fewest employee protections?
* 業務委託 (freelance / outsourcing contract)
- 正社員
- 契約社員
- 派遣社員
? You're a business, not an employee: no paid leave or social insurance from the client.

@@quiz
What is 内定?
* A job offer
- A rejection
- The first interview
- A visa
? Then 内定承諾 (accepting) and 入社 (starting).

@@flashcards
doda / リクナビNEXT / マイナビ転職 :: Big Japanese job sites
TokyoDev / Japan Dev :: English-friendly developer job boards
エージェント :: Recruiter (free for candidates)
書類選考 :: Document screening
SPI / 適性検査 :: Aptitude test
一次面接 / 最終面接 :: First / final interview
内定 / 内定承諾 :: Job offer / accepting it
入社 :: Joining the company
中途採用 / 新卒採用 :: Mid-career / new-graduate hiring
労働条件通知書 :: Written employment conditions
正社員 / 契約社員 / 派遣社員 / 業務委託 :: Regular / contract / dispatched / freelance
賞与 :: Bonus
年俸制 :: Annual salary system
みなし残業 / 固定残業代 :: Fixed overtime included in salary
試用期間 :: Probation period
年間休日 :: Days off per year
逆質問 :: Your questions to the interviewer
転職理由 :: Reason for changing jobs

@@drill
What is the Japanese word for a job offer? (2 kanji)
= 内定
= ないてい
= naitei
? Then 内定承諾 (accepting the offer).

@@exercise Track applications through the 選考 stages
Track a mid-career job search through the lesson's stages: 応募 → 書類選考 → 一次面接 → 二次面接 → 最終面接 → 内定. Each application records the furthest stage it reached, whether it was rejected (`不採用`), and the date of the last contact.

Write `funnel(apps)` that counts how many applications **reached** each stage, and `follow_ups(apps, today)` that lists active applications with no contact for more than 7 days (polite follow-up time). Print the funnel with the percentage of applications that reached each stage, then the follow-ups.
@@lang python
@@starter
from datetime import date

STAGES = ["応募", "書類選考", "一次面接", "二次面接", "最終面接", "内定"]
apps = [
    {"company": "Sakura Systems", "stage": "最終面接", "rejected": False, "last": date(2026, 11, 5)},
    {"company": "Kansai Cloud", "stage": "書類選考", "rejected": True, "last": date(2026, 10, 20)},
    {"company": "Mirai Net", "stage": "一次面接", "rejected": False, "last": date(2026, 10, 25)},
    {"company": "Fuji Data", "stage": "内定", "rejected": False, "last": date(2026, 11, 6)},
    {"company": "Tokyo SecOps", "stage": "応募", "rejected": False, "last": date(2026, 10, 15)},
    {"company": "Nagoya Infra", "stage": "二次面接", "rejected": True, "last": date(2026, 11, 1)},
    {"company": "Hokkaido Web", "stage": "書類選考", "rejected": False, "last": date(2026, 11, 2)},
]

def funnel(apps):
    raise NotImplementedError


def follow_ups(apps, today):
    raise NotImplementedError


today = date(2026, 11, 7)
counts = funnel(apps)
for s in STAGES:
    print(f"{s}\t{counts[s]}  {counts[s] * 100 // len(apps)}%")
print("Follow up:")
for company, stage, days in follow_ups(apps, today):
    print(f"  {company} ({stage}): {days} days since last contact")
@@expected
応募	7  100%
書類選考	6  85%
一次面接	4  57%
二次面接	3  42%
最終面接	2  28%
内定	1  14%
Follow up:
  Mirai Net (一次面接): 13 days since last contact
  Tokyo SecOps (応募): 23 days since last contact
@@requires
def funnel
def follow_ups
STAGES.index
@@hint
An application at 二次面接 also passed every earlier stage, so count it in each stage up to its own: `STAGES[:STAGES.index(stage) + 1]`.
@@solution
from datetime import date

STAGES = ["応募", "書類選考", "一次面接", "二次面接", "最終面接", "内定"]
apps = [
    {"company": "Sakura Systems", "stage": "最終面接", "rejected": False, "last": date(2026, 11, 5)},
    {"company": "Kansai Cloud", "stage": "書類選考", "rejected": True, "last": date(2026, 10, 20)},
    {"company": "Mirai Net", "stage": "一次面接", "rejected": False, "last": date(2026, 10, 25)},
    {"company": "Fuji Data", "stage": "内定", "rejected": False, "last": date(2026, 11, 6)},
    {"company": "Tokyo SecOps", "stage": "応募", "rejected": False, "last": date(2026, 10, 15)},
    {"company": "Nagoya Infra", "stage": "二次面接", "rejected": True, "last": date(2026, 11, 1)},
    {"company": "Hokkaido Web", "stage": "書類選考", "rejected": False, "last": date(2026, 11, 2)},
]

def funnel(apps):
    counts = {s: 0 for s in STAGES}
    for a in apps:
        for s in STAGES[:STAGES.index(a["stage"]) + 1]:
            counts[s] += 1
    return counts

def follow_ups(apps, today):
    return [(a["company"], a["stage"], (today - a["last"]).days) for a in apps
            if not a["rejected"] and a["stage"] != "内定" and (today - a["last"]).days > 7]

today = date(2026, 11, 7)
counts = funnel(apps)
for s in STAGES:
    print(f"{s}\t{counts[s]}  {counts[s] * 100 // len(apps)}%")
print("Follow up:")
for company, stage, days in follow_ups(apps, today):
    print(f"  {company} ({stage}): {days} days since last contact")

@@exercise Find the red flags in a 労働条件通知書
Check offers against the lesson's "watch for" column. Write `red_flags(offer)` returning warnings:

- `雇用形態` is `業務委託`: no employee protections (no 社会保険, no paid leave);
- a full-time offer without `社会保険` enrolment;
- `休日` is `週休2日制` (not 完全週休2日制): some weeks have only one day off;
- `年間休日` under 120;
- `試用期間` longer than 6 months, or lower pay during it;
- no `交通費`.

Print each offer with its warnings, or `no red flags`, then the offers sorted by number of warnings.
@@lang python
@@starter
offers = [
    {"company": "A社", "雇用形態": "正社員", "社会保険": True, "休日": "完全週休2日制", "年間休日": 125,
     "試用期間": 3, "試用期間_lower_pay": False, "交通費": True},
    {"company": "B社", "雇用形態": "業務委託", "社会保険": False, "休日": "週休2日制", "年間休日": 105,
     "試用期間": 0, "試用期間_lower_pay": False, "交通費": False},
    {"company": "C社", "雇用形態": "契約社員", "社会保険": False, "休日": "完全週休2日制", "年間休日": 120,
     "試用期間": 6, "試用期間_lower_pay": True, "交通費": True},
    {"company": "D社", "雇用形態": "正社員", "社会保険": True, "休日": "週休2日制", "年間休日": 118,
     "試用期間": 12, "試用期間_lower_pay": False, "交通費": True},
]

def red_flags(o):
    raise NotImplementedError


results = {o["company"]: red_flags(o) for o in offers}
for company, flags in results.items():
    print(f"{company}: " + ("; ".join(flags) if flags else "no red flags"))
print("Ranked: " + " < ".join(sorted(results, key=lambda c: (len(results[c]), c))))
@@expected
A社: no red flags
B社: 業務委託: freelance, no employee protections; 週休2日制 is not every weekend off; only 105 年間休日; no 交通費
C社: no 社会保険 for a full-time employee; lower pay during 試用期間
D社: 週休2日制 is not every weekend off; only 118 年間休日; 12-month 試用期間
Ranked: A社 < C社 < D社 < B社
@@requires
def red_flags
< 120
@@hint
One `if` per row of the table, appending a warning each time. 業務委託 explains the missing 社会保険 already, so use `elif` to avoid saying it twice.
@@solution
offers = [
    {"company": "A社", "雇用形態": "正社員", "社会保険": True, "休日": "完全週休2日制", "年間休日": 125,
     "試用期間": 3, "試用期間_lower_pay": False, "交通費": True},
    {"company": "B社", "雇用形態": "業務委託", "社会保険": False, "休日": "週休2日制", "年間休日": 105,
     "試用期間": 0, "試用期間_lower_pay": False, "交通費": False},
    {"company": "C社", "雇用形態": "契約社員", "社会保険": False, "休日": "完全週休2日制", "年間休日": 120,
     "試用期間": 6, "試用期間_lower_pay": True, "交通費": True},
    {"company": "D社", "雇用形態": "正社員", "社会保険": True, "休日": "週休2日制", "年間休日": 118,
     "試用期間": 12, "試用期間_lower_pay": False, "交通費": True},
]

def red_flags(o):
    flags = []
    if o["雇用形態"] == "業務委託":
        flags.append("業務委託: freelance, no employee protections")
    elif not o["社会保険"]:
        flags.append("no 社会保険 for a full-time employee")
    if o["休日"] == "週休2日制":
        flags.append("週休2日制 is not every weekend off")
    if o["年間休日"] < 120:
        flags.append(f"only {o['年間休日']} 年間休日")
    if o["試用期間"] > 6:
        flags.append(f"{o['試用期間']}-month 試用期間")
    if o["試用期間_lower_pay"]:
        flags.append("lower pay during 試用期間")
    if not o["交通費"]:
        flags.append("no 交通費")
    return flags

results = {o["company"]: red_flags(o) for o in offers}
for company, flags in results.items():
    print(f"{company}: " + ("; ".join(flags) if flags else "no red flags"))
print("Ranked: " + " < ".join(sorted(results, key=lambda c: (len(results[c]), c))))
