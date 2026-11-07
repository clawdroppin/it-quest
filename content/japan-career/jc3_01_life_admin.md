@@lesson jc3-life-admin Living and Working in Japan: The Admin
@@text
The paperwork in your first month decides how smooth your first year is. Do these in order.

## First two weeks

1. **在留カード**: issued at the airport (major airports) or mailed later.
2. **住民登録** at the city/ward office (市役所/区役所) within 14 days of moving in: you'll get a **住民票** (certificate of residence) and your **マイナンバー** (My Number). Apply for the **マイナンバーカード** (photo ID, used for many online procedures and as your health insurance card).
3. **Phone**: a SIM needs a residence card and often a bank account or credit card. Budget carriers (楽天モバイル, ahamo, povo, LINEMO) are cheap.
4. **Bank account** (ゆうちょ銀行, or a big bank or online bank): some banks require 6 months' residence; ゆうちょ and some online banks are easier. Your salary goes here.

## Insurance, pension, tax

| | Employee (会社員) | Self-employed / between jobs |
|---|---|---|
| Health insurance | **社会保険 / 健康保険** through the company (about half paid by the employer) | **国民健康保険** at the city office |
| Pension | **厚生年金** through the company | **国民年金** |
| Income tax (所得税) | deducted monthly; corrected in **年末調整** (year-end adjustment) by the company | **確定申告** (tax return, Feb–Mar) |
| Resident tax (住民税) | based on **last year's** income, deducted from salary from June | paid by bill |

- **The first-year surprise**: 住民税 starts in your *second* year, because it's based on the previous year's income. In year two your take-home pay drops a little. Plan for it.
- Take-home pay is roughly **75–80%** of gross for a typical salary after social insurance and taxes.
- Leaving Japan for good? You can claim back part of the pension contributions (**脱退一時金**, lump-sum withdrawal payment) within 2 years.

## Housing

Expect **敷金** (deposit, mostly returned), **礼金** (key money, not returned), agent fee (仲介手数料), first month's rent, fire insurance, and a **guarantor company** (保証会社), often 4–6 months' rent up front. Foreigner-friendly agents and **share houses** or **monthly mansions** are easier first steps. Your company may offer 住宅手当 (housing allowance) or a dorm (社宅/寮).

## At work

- **Paid leave** (有給休暇): 10 days after 6 months, rising with years of service; employers must make sure you take at least 5 a year.
- **Overtime** needs a **36協定** (overtime agreement) and is legally capped (generally 45 h/month, 360 h/year, with exceptions). Unpaid overtime (サービス残業) is illegal.
- **Health check** (健康診断) yearly, paid by the employer.
- **Resigning**: by law two weeks' notice is enough for permanent employees, but **one month** (or what the employment rules, 就業規則, say) is the norm. Hand in a 退職届, hand over your work (引き継ぎ), and collect your **源泉徴収票** (withholding slip) and **離職票** (if you'll claim unemployment benefits).
- Problems with an employer: the **Labour Standards Inspection Office** (労働基準監督署) handles violations; there are consultation lines in English for foreign workers.

@@quiz
Why does take-home pay often drop in your second year in Japan?
* Resident tax (住民税) starts, based on the previous year's income
- Salaries always fall in year two
- Pension stops
- The bonus is removed by law
? Plan for it in your budget.

@@quiz
Which health insurance does a full-time company employee normally have?
* 社会保険 (health insurance through the company)
- 国民健康保険
- None
- Travel insurance
? Employers pay about half.

@@quiz
Who corrects an employee's income tax at the end of the year?
* The company, through 年末調整
- Nobody
- The employee only, through a tax return
- The bank
? Employees with extra income may still need 確定申告.

@@quiz
What is 礼金 when renting an apartment?
* "Key money" paid to the landlord, not returned
- A refundable deposit
- Monthly rent
- A guarantor
? 敷金 is the deposit, mostly returned at move-out.

@@quiz
How many paid leave days does a full-time employee get after 6 months?
* 10
- 0
- 5
- 20
? It increases with years of service, up to 20.

@@quiz
What's a typical notice period when resigning in Japan?
* About one month, following the company's 就業規則
- One day
- Six months by law
- No notice is needed
? Two weeks is the legal minimum for permanent employees.

@@flashcards
市役所 / 区役所 :: City / ward office
住民票 :: Certificate of residence
マイナンバーカード :: My Number photo ID card
社会保険 / 国民健康保険 :: Company health insurance / national health insurance
厚生年金 / 国民年金 :: Employee pension / national pension
所得税 / 住民税 :: Income tax / resident tax (based on last year)
年末調整 :: Year-end tax adjustment by the employer
確定申告 :: Tax return (Feb–Mar)
脱退一時金 :: Pension lump-sum refund when leaving Japan
敷金 / 礼金 :: Deposit / key money
保証会社 :: Rent guarantor company
社宅 / 寮 :: Company housing / dorm
有給休暇 :: Paid leave
36協定 :: Overtime agreement
サービス残業 :: Unpaid overtime (illegal)
健康診断 :: Annual health check
退職届 :: Resignation letter
引き継ぎ :: Handover
源泉徴収票 :: Withholding tax slip
労働基準監督署 :: Labour Standards Inspection Office

@@drill
What's the Japanese term for the year-end tax adjustment done by your employer? (4 kanji)
= 年末調整
= ねんまつちょうせい
= nenmatsu chousei
? Bring your insurance and deduction certificates to HR in November.

@@exercise Plan the first two weeks' paperwork
Plan the lesson's first steps after moving in. You already have the 在留カード from the airport. Each task has prerequisites: 住民登録 needs the 在留カード; the bank account needs 住民登録 (for the 住民票 and address); the phone contract needs the 在留カード and a bank account; マイナンバーカード needs 住民登録.

Write `schedule(tasks, move_in)` that orders the tasks so every prerequisite is done or already in hand (`have`) (one task per working day, starting the day after moving in, skipping Saturdays and Sundays because the ward office is closed), and check that 住民登録 lands within 14 days of moving in. Print the plan.
@@lang python
@@starter
from datetime import date, timedelta

have = {"在留カード"}
tasks = {
    "住民登録": ["在留カード"],
    "マイナンバーカード申請": ["住民登録"],
    "銀行口座": ["住民登録"],
    "携帯電話": ["在留カード", "銀行口座"],
}

def order(tasks):
    raise NotImplementedError


def schedule(tasks, move_in):
    raise NotImplementedError


move_in = date(2026, 4, 2)
plan = schedule(tasks, move_in)
for day, t in plan:
    print(f"{day} ({'月火水木金土日'[day.weekday()]}) {t}")
registered = next(d for d, t in plan if t == "住民登録")
left = 14 - (registered - move_in).days
print(f"住民登録 on day {(registered - move_in).days} after moving in ({'OK' if left >= 0 else 'LATE'}, limit 14)")
@@expected
2026-04-03 (金) 住民登録
2026-04-06 (月) マイナンバーカード申請
2026-04-07 (火) 銀行口座
2026-04-08 (水) 携帯電話
住民登録 on day 1 after moving in (OK, limit 14)
@@requires
def schedule
weekday()
all(
@@hint
Repeatedly pick a task whose prerequisites are all done. For the dates, add a day and keep adding while `weekday()` is 5 or 6 (Saturday, Sunday).
@@solution
from datetime import date, timedelta

have = {"在留カード"}
tasks = {
    "住民登録": ["在留カード"],
    "マイナンバーカード申請": ["住民登録"],
    "銀行口座": ["住民登録"],
    "携帯電話": ["在留カード", "銀行口座"],
}

def order(tasks):
    done, result = set(have), []
    while len(result) < len(tasks):
        ready = [t for t in tasks if t not in done and all(p in done for p in tasks[t])]
        if not ready:
            raise ValueError("circular prerequisites")
        result.append(ready[0])
        done.add(ready[0])
    return result

def schedule(tasks, move_in):
    plan, day = [], move_in
    for t in order(tasks):
        day += timedelta(days=1)
        while day.weekday() >= 5:
            day += timedelta(days=1)
        plan.append((day, t))
    return plan

move_in = date(2026, 4, 2)
plan = schedule(tasks, move_in)
for day, t in plan:
    print(f"{day} ({'月火水木金土日'[day.weekday()]}) {t}")
registered = next(d for d, t in plan if t == "住民登録")
left = 14 - (registered - move_in).days
print(f"住民登録 on day {(registered - move_in).days} after moving in ({'OK' if left >= 0 else 'LATE'}, limit 14)")
