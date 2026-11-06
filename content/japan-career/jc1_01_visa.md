@@lesson jc1-visa Visas and Residence Status
@@text
> **Important**: immigration rules change, and individual cases differ. This lesson explains the system so you can ask good questions. Always check the **Immigration Services Agency of Japan** (出入国在留管理庁, isa.go.jp), your employer's HR, or a licensed immigration specialist (**行政書士**, gyōseishoshi) before acting.

## The usual work visa for IT: 技術・人文知識・国際業務

**Engineer / Specialist in Humanities / International Services** (often called the "gijinkoku" visa) covers most IT jobs. Typical requirements for the **技術** (engineering) part:

- a **university degree** (or a Japanese vocational school diploma, 専門士) in a **related field**, **or**
- around **10 years** of relevant work experience, **or**
- for IT work specifically, passing certain **IT qualifications listed in a Ministry of Justice notice** (the "IT告示"), which includes Japan's IPA exams such as 基本情報技術者 and some exams of other Asian countries, can substitute for the degree/experience requirement;
- a **job offer** from a Japanese employer, with work that matches your background and pay **equal to or higher than a Japanese employee** doing the same work.

The employer usually applies for a **Certificate of Eligibility** (在留資格認定証明書, COE); with it, you get the visa at a Japanese embassy. Periods: 3 months, 1, 3 or 5 years, renewable.

## Highly Skilled Professional (高度専門職)

A **points system**: academic background, career, annual salary, age, Japanese ability (JLPT N1/N2), research, certain qualifications and more. **70 points** or more qualifies; benefits include a 5-year period, a working spouse, and a much faster route to **permanent residence** (永住権): possible after **3 years at 70 points, or 1 year at 80 points**. Calculate your points with the official points table (ポイント計算表) on the ISA site.

## Other routes you may hear about

| Route | For |
|---|---|
| **J-Skip** (特別高度人材) | very high salaries/degrees: faster benefits than regular HSP |
| **J-Find** (未来創造人材) | graduates of top-ranked universities: up to 2 years to look for work or start a business |
| **Working holiday** | young people from partner countries; short-term, can be a first step |
| **Student → work** | study at a Japanese language school or university, then change status after a job offer (very common) |
| **Spouse / long-term resident** | no work restrictions |

## Once you're in Japan

- **在留カード** (residence card): carry it always. Your status and expiry date are on it.
- **住民登録**: register your address at the city office (市役所/区役所) within **14 days** of moving in. You'll get your **マイナンバー** (My Number) there.
- **Changing jobs** (転職): notify immigration within **14 days** (所属機関に関する届出, online). Make sure the new job fits your status; you can apply for a **就労資格証明書** to confirm it.
- **Renewing**: apply up to 3 months before expiry; keep tax and pension payment records clean, as they're checked.
- **Side jobs** outside your status need **資格外活動許可** (permission).

@@quiz
Which residence status covers most IT engineering jobs?
* 技術・人文知識・国際業務 (Engineer/Specialist in Humanities/International Services)
- 特定技能
- 短期滞在 (temporary visitor)
- 文化活動
? Usually called gijinkoku.

@@quiz
What can sometimes substitute for the degree requirement for IT work under that status?
* Passing an IT qualification listed in the Ministry of Justice notice, such as certain IPA exams
- Any online course certificate
- A driving licence
- A JLPT N5 certificate
? Check the current list: it's specific.

@@quiz
How many points does the Highly Skilled Professional system require?
* 70 or more
- 50 or more
- 100
- 30
? 80+ points can lead to permanent residence after 1 year.

@@quiz
You change jobs while on a work visa. What must you do?
* Notify immigration within 14 days, and make sure the new job fits your status
- Nothing
- Leave Japan and re-enter
- Tell only your new employer
? Late or missing notifications can cause problems at renewal.

@@quiz
Who is a licensed specialist who can handle visa applications?
* A 行政書士 (gyōseishoshi)
- A 社内SE
- A 部長
- A recruiter only
? Many specialize in immigration.

@@flashcards
在留資格 :: Residence status (visa category)
技術・人文知識・国際業務 :: The common IT/office work status
在留資格認定証明書 (COE) :: Certificate of Eligibility, applied for by the employer
高度専門職 :: Highly Skilled Professional (points-based)
70 / 80 points :: HSP threshold / fastest permanent residence route
永住権 :: Permanent residence
在留カード :: Residence card: always carry it
住民登録 :: Address registration within 14 days
マイナンバー :: My Number (personal number)
所属機関に関する届出 :: Notification when changing employer (14 days)
就労資格証明書 :: Certificate confirming your new job fits your status
資格外活動許可 :: Permission for work outside your status
行政書士 :: Licensed administrative scrivener (visa specialist)
出入国在留管理庁 :: Immigration Services Agency

@@drill
Within how many days must you register your address after moving in? (a number)
= 14
? At the city or ward office.

@@exercise Does the offer fit 技術・人文知識・国際業務?
Screen job offers against the basic requirements for the engineering part of the status: a related degree or 専門士 diploma, or about 10 years of relevant experience, or a qualification from the IT notice (IT告示); professional work (not simple labour); and pay at least equal to a Japanese employee in the same role. Print "likely fits" or the problems to raise with the employer and a 行政書士.
@@lang python
@@starter
IT_NOTICE = {"基本情報技術者", "応用情報技術者", "情報処理安全確保支援士"}

offers = [
    {"who": "Aiko", "degree": "computer science", "years": 2, "certs": set(), "duties": "backend development", "simple_labour": False, "pay": 4_200_000, "peer_pay": 4_000_000},
    {"who": "Ben", "degree": None, "years": 4, "certs": {"基本情報技術者"}, "duties": "network operations", "simple_labour": False, "pay": 3_800_000, "peer_pay": 3_800_000},
    {"who": "Chen", "degree": "history", "years": 1, "certs": set(), "duties": "infrastructure engineer", "simple_labour": False, "pay": 3_600_000, "peer_pay": 3_600_000},
    {"who": "Dana", "degree": "electronics", "years": 3, "certs": set(), "duties": "warehouse picking", "simple_labour": True, "pay": 2_600_000, "peer_pay": 3_000_000},
    {"who": "Eiji", "degree": None, "years": 11, "certs": set(), "duties": "SRE", "simple_labour": False, "pay": 6_000_000, "peer_pay": 5_500_000},
]
RELATED = {"computer science", "information engineering", "electronics", "mathematics"}

def problems(o):
    raise NotImplementedError

for o in offers:
    found = problems(o)
    print(f"{o['who']:<5} " + ("likely fits" if not found else "check: " + "; ".join(found)))
@@expected
Aiko  likely fits
Ben   likely fits
Chen  check: degree field doesn't match: show how the studies relate, or use experience or an IT notice exam
Dana  check: 'warehouse picking' is simple labour, not covered by this status; pay ¥2,600,000 is below a Japanese peer's ¥3,000,000
Eiji  likely fits
@@requires
def problems
& IT_NOTICE
@@hint
The background is fine if the degree is in `RELATED`, or `years >= 10`, or `o["certs"] & IT_NOTICE` isn't empty. Then add a problem if `simple_labour` is true, and if `pay < peer_pay`. A degree outside the field isn't automatically a no, so suggest showing how the studies relate, or using experience or an IT notice exam instead.
@@solution
IT_NOTICE = {"基本情報技術者", "応用情報技術者", "情報処理安全確保支援士"}

offers = [
    {"who": "Aiko", "degree": "computer science", "years": 2, "certs": set(), "duties": "backend development", "simple_labour": False, "pay": 4_200_000, "peer_pay": 4_000_000},
    {"who": "Ben", "degree": None, "years": 4, "certs": {"基本情報技術者"}, "duties": "network operations", "simple_labour": False, "pay": 3_800_000, "peer_pay": 3_800_000},
    {"who": "Chen", "degree": "history", "years": 1, "certs": set(), "duties": "infrastructure engineer", "simple_labour": False, "pay": 3_600_000, "peer_pay": 3_600_000},
    {"who": "Dana", "degree": "electronics", "years": 3, "certs": set(), "duties": "warehouse picking", "simple_labour": True, "pay": 2_600_000, "peer_pay": 3_000_000},
    {"who": "Eiji", "degree": None, "years": 11, "certs": set(), "duties": "SRE", "simple_labour": False, "pay": 6_000_000, "peer_pay": 5_500_000},
]
RELATED = {"computer science", "information engineering", "electronics", "mathematics"}

def problems(o):
    found = []
    background = o["degree"] in RELATED or o["years"] >= 10 or bool(o["certs"] & IT_NOTICE)
    if not background:
        if o["degree"]:
            found.append("degree field doesn't match: show how the studies relate, or use experience or an IT notice exam")
        else:
            found.append("no degree, under 10 years' experience and no IT notice exam")
    if o["simple_labour"]:
        found.append(f"'{o['duties']}' is simple labour, not covered by this status")
    if o["pay"] < o["peer_pay"]:
        found.append(f"pay ¥{o['pay']:,} is below a Japanese peer's ¥{o['peer_pay']:,}")
    return found

for o in offers:
    found = problems(o)
    print(f"{o['who']:<5} " + ("likely fits" if not found else "check: " + "; ".join(found)))
