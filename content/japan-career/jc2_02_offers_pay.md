@@lesson jc2-offers-pay Reading Offers in Depth: Salary, 固定残業代, Overtime Rules and Real Hourly Pay
@@text
A Japanese offer letter (内定通知書) and the working conditions notice (労働条件通知書) can make two jobs with the same 年収 (annual pay) very different. Bonuses may be "expected" rather than guaranteed, a high monthly salary may include 45 hours of overtime, and allowances may or may not count. This lesson shows how to break an offer down and compute what you're really paid per hour, and what the Labour Standards Act (労働基準法) guarantees. **Figures such as the minimum wage and insurance rates change every year: check the current values (厚生労働省, your prefecture) before relying on them.** This is general information, not legal advice; for a specific dispute, ask the 労働基準監督署 (labour standards inspection office) or a lawyer.

## The parts of the pay

| Term | Meaning |
|---|---|
| **基本給** | base salary: the basis for overtime, bonus and often retirement pay |
| **固定残業代 / みなし残業** | fixed overtime pay for a stated number of hours; must be shown separately with its hours |
| **手当** | allowances: 通勤手当 (commuting, usually non-taxable up to ¥150,000/month), 住宅手当 (housing), 資格手当 (certifications), 家族手当 |
| **賞与** | bonus, often "年2回 計4ヶ月分" (twice a year, 4 months of base salary in total); check whether it's 実績 (past result) or guaranteed |
| **年収** | annual pay; ask whether it includes bonus and fixed overtime |
| **試用期間** | probation (often 3–6 months), sometimes with lower pay |
| **退職金** | retirement allowance, depends on company rules |

## Overtime rules (労働基準法)

- Statutory hours: **8 hours a day, 40 a week**. Overtime requires a **36協定** (labour-management agreement).
- Limits: normally **45 hours/month and 360/year**; with a special clause up to 720/year, **under 100 hours/month** including holiday work, and an **average of 80** over any 2–6 months.
- Premiums: overtime **+25%**, over **60 hours/month +50%**, late night (22:00–5:00) **+25%** (stacking), statutory holiday **+35%**.
- **固定残業代 is legal only if** it's clearly separated from base pay, the covered hours are stated, and **overtime beyond those hours is paid extra**. If it doesn't cover the hours at the premium rate, the difference is owed.

## Checking an offer

1. **Hourly base rate** = (基本給 + allowances that count, excluding fixed overtime, commuting, family and similar) / monthly scheduled hours. It must be at least the **minimum wage** (最低賃金) of the prefecture.
2. **Does the fixed overtime cover its hours?** Fixed overtime ≥ hours × hourly base × 1.25.
3. **Real hourly pay** = total annual pay / (scheduled hours + expected overtime hours) per year. Compare offers on this, not on 年収 alone.
4. **Take-home pay (手取り)**: social insurance (health, pension, employment insurance) and income and resident taxes typically leave roughly **75–80%** of gross for a single person at typical IT salaries. This is a rough rule of thumb; use a current calculator for real numbers.

## Other things to check

Working-hour system (fixed, フレックスタイム制, 裁量労働制 discretionary work, where overtime isn't paid by the hour), remote work rules, transfer possibility (転勤), annual paid leave (10 days after 6 months, increasing with service; at least 5 must be taken), side jobs (副業), and the non-compete/confidentiality clauses.

@@quiz
An offer says 月給30万円 (固定残業代45時間分 7万円を含む). What is the base pay used for the hourly rate?
- ¥300,000
* ¥230,000
- ¥370,000
- ¥70,000
? The fixed overtime part is excluded.

@@quiz
What is the normal monthly overtime limit under a standard 36協定?
- 30 hours
* 45 hours
- 60 hours
- 100 hours
? 360 hours a year.

@@quiz
What premium applies to overtime beyond 60 hours in a month?
- 25%
- 35%
* 50%
- 100%
? Since 2023 for all companies.

@@quiz
Fixed overtime covers 30 hours, but you work 42 hours of overtime. What should happen?
- Nothing; it's fixed
* The extra 12 hours must be paid on top
- You get time off instead, automatically
- The fixed amount is reduced
? Overtime beyond the covered hours must be paid.

@@quiz
Which allowance is usually non-taxable up to a monthly limit?
- 住宅手当
* 通勤手当
- 資格手当
- 家族手当
? Commuting allowance.

@@quiz
How many days of annual paid leave must an employer make sure a worker with 10+ days actually takes?
- 3
* 5
- 10
- 20
? Mandatory since 2019.

@@flashcards
基本給 :: base salary
固定残業代 :: fixed overtime pay for stated hours
みなし残業 :: same idea: deemed overtime
通勤手当 :: commuting allowance
賞与 :: bonus, months of base pay
試用期間 :: probation
労働条件通知書 :: working conditions notice
statutory hours :: 8/day, 40/week
36協定 :: overtime agreement
overtime limit :: 45/month, 360/year
special clause max :: 720/year, < 100/month
overtime premium :: +25%
over 60 h/month :: +50%
late night :: 22–5, +25%
holiday work :: +35%
hourly base :: base pay / scheduled hours
最低賃金 :: prefectural minimum wage
裁量労働制 :: discretionary work system
有給休暇 :: 10 days after 6 months
手取り :: take-home pay, roughly 75–80%

@@drill
What premium (percent) applies to statutory holiday work?
= 35
= 35%
? Overtime is 25%.

@@drill
What is the Japanese term for take-home pay?
= 手取り
= てどり
? 額面 is gross.

@@exercise Check fixed overtime and the minimum wage
For each offer, compute the hourly base rate (base pay excluding fixed overtime and excluded allowances, divided by monthly scheduled hours), check it against `MIN_WAGE`, check that the fixed overtime covers its stated hours at 1.25× the hourly rate, and compute the overtime pay owed if the expected overtime exceeds the covered hours. Print each offer's checks.
@@lang python
@@starter
MIN_WAGE = 1_226   # example: Tokyo from October 2025; use the current figure for your prefecture
offers = [
    {"name": "SIer A", "monthly": 320_000, "fixed_ot": 90_000, "ot_hours": 45, "allowances": {"住宅手当": 10_000, "通勤手当": 15_000}, "scheduled": 160, "expected_ot": 40},
    {"name": "SES B", "monthly": 260_000, "fixed_ot": 50_000, "ot_hours": 40, "allowances": {"通勤手当": 12_000}, "scheduled": 168, "expected_ot": 50},
    {"name": "Web C", "monthly": 350_000, "fixed_ot": 0, "ot_hours": 0, "allowances": {"資格手当": 10_000}, "scheduled": 160, "expected_ot": 20},
]
EXCLUDED = {"通勤手当", "家族手当"}

def hourly_base(offer):
    raise NotImplementedError
@@expected
SIer A: hourly base ¥1,500 -> OK (minimum ¥1,226)
  fixed overtime ¥90,000 for 45 h needs ¥84,375: covers its hours
  expected overtime is within the covered hours
SES B: hourly base ¥1,250 -> OK (minimum ¥1,226)
  fixed overtime ¥50,000 for 40 h needs ¥62,500: short by ¥12,500
  10 h beyond the covered hours must be paid extra: ¥15,625/month
Web C: hourly base ¥2,250 -> OK (minimum ¥1,226)
  no fixed overtime: all 20 h paid at 1.25x = ¥56,250/month
@@requires
def hourly_base
1.25
MIN_WAGE
@@hint
`monthly` is the advertised monthly pay including fixed overtime but excluding allowances listed separately. Hourly base = (monthly − fixed_ot + allowances not in EXCLUDED) / scheduled. Required fixed overtime = ot_hours × hourly × 1.25. Extra owed = (expected_ot − ot_hours) × hourly × 1.25 (× 1.5 for hours over 60 is not needed here).
@@solution
MIN_WAGE = 1_226   # example: Tokyo from October 2025; use the current figure for your prefecture
offers = [
    {"name": "SIer A", "monthly": 320_000, "fixed_ot": 90_000, "ot_hours": 45, "allowances": {"住宅手当": 10_000, "通勤手当": 15_000}, "scheduled": 160, "expected_ot": 40},
    {"name": "SES B", "monthly": 260_000, "fixed_ot": 50_000, "ot_hours": 40, "allowances": {"通勤手当": 12_000}, "scheduled": 168, "expected_ot": 50},
    {"name": "Web C", "monthly": 350_000, "fixed_ot": 0, "ot_hours": 0, "allowances": {"資格手当": 10_000}, "scheduled": 160, "expected_ot": 20},
]
EXCLUDED = {"通勤手当", "家族手当"}

def hourly_base(offer):
    counted = sum(v for k, v in offer["allowances"].items() if k not in EXCLUDED)
    return (offer["monthly"] - offer["fixed_ot"] + counted) / offer["scheduled"]

for o in offers:
    rate = hourly_base(o)
    print(f"{o['name']}: hourly base ¥{rate:,.0f} -> {'OK' if rate >= MIN_WAGE else 'BELOW MINIMUM WAGE'} (minimum ¥{MIN_WAGE:,})")
    if o["fixed_ot"]:
        needed = o["ot_hours"] * rate * 1.25
        status = "covers its hours" if o["fixed_ot"] >= needed else f"short by ¥{needed - o['fixed_ot']:,.0f}"
        print(f"  fixed overtime ¥{o['fixed_ot']:,} for {o['ot_hours']} h needs ¥{needed:,.0f}: {status}")
    extra_hours = max(0, o["expected_ot"] - o["ot_hours"])
    if not o["fixed_ot"]:
        print(f"  no fixed overtime: all {extra_hours} h paid at 1.25x = ¥{extra_hours * rate * 1.25:,.0f}/month")
    elif extra_hours:
        print(f"  {extra_hours} h beyond the covered hours must be paid extra: ¥{extra_hours * rate * 1.25:,.0f}/month")
    else:
        print("  expected overtime is within the covered hours")

@@exercise Compare offers by real hourly pay
Compute each offer's annual pay (monthly × 12, plus bonus as months of base pay, plus any overtime paid beyond fixed overtime at 1.25× the hourly base, excluding the non-taxable commuting allowance), annual hours worked (scheduled + expected overtime, × 12), real hourly pay, and a rough take-home estimate at `TAKE_HOME_RATIO` (labelled as a rule of thumb). Rank offers by real hourly pay.
@@lang python
@@starter
TAKE_HOME_RATIO = 0.78   # rough rule of thumb, not a tax calculation
offers = [
    {"name": "SIer A", "monthly": 320_000, "base": 230_000, "fixed_ot_hours": 45, "bonus_months": 4.0, "scheduled": 160, "expected_ot": 40},
    {"name": "SES B", "monthly": 260_000, "base": 210_000, "fixed_ot_hours": 40, "bonus_months": 2.0, "scheduled": 168, "expected_ot": 50},
    {"name": "Web C", "monthly": 350_000, "base": 350_000, "fixed_ot_hours": 0, "bonus_months": 0.0, "scheduled": 160, "expected_ot": 20},
    {"name": "外資 D", "monthly": 520_000, "base": 520_000, "fixed_ot_hours": 0, "bonus_months": 0.0, "scheduled": 160, "expected_ot": 0, "discretionary": True},
]

def annual_pay(o):
    raise NotImplementedError
@@expected
外資 D    年収 ¥ 6,240,000  1,920 h/year  real ¥3,250/h  take-home ≈ ¥4,867,200
Web C   年収 ¥ 4,856,250  2,160 h/year  real ¥2,248/h  take-home ≈ ¥3,787,875
SIer A  年収 ¥ 4,760,000  2,400 h/year  real ¥1,983/h  take-home ≈ ¥3,712,800
SES B   年収 ¥ 3,727,500  2,616 h/year  real ¥1,425/h  take-home ≈ ¥2,907,450
@@requires
def annual_pay
TAKE_HOME_RATIO
sorted(
@@hint
Hourly base = base / scheduled. Overtime paid beyond fixed hours = max(0, expected − fixed) × hourly × 1.25 per month, unless the offer is under a discretionary system (`discretionary`), where it's 0. Bonus = base × bonus_months.
@@solution
TAKE_HOME_RATIO = 0.78   # rough rule of thumb, not a tax calculation
offers = [
    {"name": "SIer A", "monthly": 320_000, "base": 230_000, "fixed_ot_hours": 45, "bonus_months": 4.0, "scheduled": 160, "expected_ot": 40},
    {"name": "SES B", "monthly": 260_000, "base": 210_000, "fixed_ot_hours": 40, "bonus_months": 2.0, "scheduled": 168, "expected_ot": 50},
    {"name": "Web C", "monthly": 350_000, "base": 350_000, "fixed_ot_hours": 0, "bonus_months": 0.0, "scheduled": 160, "expected_ot": 20},
    {"name": "外資 D", "monthly": 520_000, "base": 520_000, "fixed_ot_hours": 0, "bonus_months": 0.0, "scheduled": 160, "expected_ot": 0, "discretionary": True},
]

def annual_pay(o):
    hourly = o["base"] / o["scheduled"]
    extra_hours = 0 if o.get("discretionary") else max(0, o["expected_ot"] - o["fixed_ot_hours"])
    overtime = extra_hours * hourly * 1.25 * 12
    return o["monthly"] * 12 + o["base"] * o["bonus_months"] + overtime

rows = []
for o in offers:
    pay = annual_pay(o)
    hours = (o["scheduled"] + o["expected_ot"]) * 12
    rows.append((pay / hours, o["name"], pay, hours))

for real, name, pay, hours in sorted(rows, reverse=True):
    print(f"{name:<7} 年収 ¥{pay:>10,.0f}  {hours:,} h/year  real ¥{real:,.0f}/h  take-home ≈ ¥{pay * TAKE_HOME_RATIO:,.0f}")
