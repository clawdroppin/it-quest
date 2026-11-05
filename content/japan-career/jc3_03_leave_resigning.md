@@lesson jc3-leave-resigning Paid Leave and Changing Jobs: 有給休暇, Notice, Bonuses and Timing
@@lang python
@@text
Most IT careers in Japan include several job changes (転職). Doing it well means knowing your **paid leave rights**, the **notice rules**, and how **bonus dates** and **leave days** affect the best last day. The entitlements below come from the Labour Standards Act (労働基準法). Your company's rules (就業規則) can be more generous but not less. This is general information, not legal advice.

## Annual paid leave (年次有給休暇)

You qualify after **6 months** of continuous employment with **80% attendance**. Full-time days granted by length of service:

| Service | 0.5 y | 1.5 y | 2.5 y | 3.5 y | 4.5 y | 5.5 y | 6.5 y+ |
|---|---|---|---|---|---|---|---|
| Days | 10 | 11 | 12 | 14 | 16 | 18 | 20 |

- Unused days **carry over for one year**, then expire (a two-year claim period), so you can hold at most 40 days.
- Since 2019, employers must make sure anyone granted 10 or more days **takes at least 5** within the year.
- The employer can't refuse leave. It can only ask you to move it if your timing would seriously disrupt business (時季変更権), and that hardly applies when you're leaving and have no later dates to move it to.

## Resigning (退職)

- **Legal minimum notice** for permanent employees: **2 weeks** (Civil Code article 627). Company rules often ask for 1 month. Following them keeps things friendly, and handover (引き継ぎ) matters for references in a small industry.
- A common pattern: give notice, hand over, then use up your remaining leave (**有給消化**), so your last working day comes earlier than your official last day of employment (退職日).
- **Bonus timing**: many companies require you to be employed on the payment date (支給日在籍要件). Give notice so that your 退職日 falls after the summer (June/July) or winter (December) bonus date.
- **Retirement allowance** (退職金), if any, depends on years of service, and often only starts after 3 years.

## Between jobs

- Get your 源泉徴収票 (withholding slip) and 離職票 (for unemployment benefits) and your pension handbook number. Health insurance switches from the company's 健康保険 to 国民健康保険, or you can use 任意継続 for up to 2 years.
- If your residence status is work-based, report the change of employer to the Immigration Services Agency within **14 days** (所属機関に関する届出).

@@quiz
How many paid leave days does a full-time employee get after 6 months of service?
- 5
* 10
- 14
- 20
? Then 11 at 1.5 years, 12 at 2.5, and so on.

@@quiz
What is the legal minimum notice for a permanent employee to resign?
- 1 month
* 2 weeks
- 3 months
- No notice
? Company rules often ask for 1 month.

@@quiz
What does 支給日在籍要件 mean for your bonus?
* You must still be employed on the payment date to receive it
- The bonus is paid when you resign
- Bonuses are optional by law
- It is paid in proportion to the days worked
? Time your 退職日 after the bonus date.

@@quiz
Within how many days must a work-visa holder report a change of employer to immigration?
- 7
* 14
- 30
- 90
? The 所属機関に関する届出.

@@quiz
Unused paid leave from this year...
- expires immediately
* carries over one year, then expires
- carries over forever
- is always paid out in cash
? So you can hold at most 40 days.

@@flashcards
年次有給休暇 :: annual paid leave
6 months, 80% attendance :: when you qualify
10 → 20 days :: from 0.5 to 6.5 years of service
carry-over :: one year, then it expires
5-day rule :: employer must ensure you take 5
時季変更権 :: employer's right to move leave dates
退職日 :: official last day of employment
有給消化 :: using up leave before you leave
引き継ぎ :: handover
支給日在籍要件 :: must be employed on the bonus date
退職金 :: retirement allowance
離職票 :: separation notice for unemployment benefits
任意継続 :: keep the company health insurance, up to 2 years
所属機関に関する届出 :: report the employer change within 14 days

@@drill
How many paid leave days does a full-time employee get at 3.5 years of service?
= 14
? 10, 11, 12, 14, 16, 18, 20.

@@drill
What is the Japanese word for using up your remaining paid leave before leaving?
= 有給消化
= yuukyuu shouka
? Your last working day comes before your 退職日.

@@exercise Track paid leave balances
Grant leave on each anniversary according to the statutory table (only if attendance was 80% or more), take days from the oldest grant first, expire grants after two years, and flag years in which fewer than 5 days were taken from a grant of 10 or more.
@@concepts lists, loops, conditions
@@starter
GRANTS = {0.5: 10, 1.5: 11, 2.5: 12, 3.5: 14, 4.5: 16, 5.5: 18, 6.5: 20}
# service year at grant: (attendance rate, days taken in the following year)
history = {0.5: (0.95, 4), 1.5: (0.97, 8), 2.5: (0.78, 3), 3.5: (0.92, 20), 4.5: (0.99, 6)}

def grant_days(service, attendance):
    raise NotImplementedError
@@expected
year 0.5: +10 granted,  4 taken, balance  6  (fewer than 5 days taken: employer breaches the 5-day rule)
year 1.5: +11 granted,  8 taken, balance  9
year 2.5: + 0 granted,  3 taken, balance  6  (no grant (attendance 78% < 80%))
year 3.5: +14 granted, 14 taken, balance  0  (6 day(s) expired; wanted 20 but only 14 available)
year 4.5: +16 granted,  6 taken, balance 10
@@requires
def grant_days
GRANTS
0.8
@@hint
Keep a list of `[service, days_left]` lots. Each year: drop lots older than 2 years (their remaining days expire), add the new grant if attendance ≥ 0.8, then take days from the oldest lot first. Flag when a grant of 10 or more days was given but fewer than 5 days were taken.
@@solution
GRANTS = {0.5: 10, 1.5: 11, 2.5: 12, 3.5: 14, 4.5: 16, 5.5: 18, 6.5: 20}
# service year at grant: (attendance rate, days taken in the following year)
history = {0.5: (0.95, 4), 1.5: (0.97, 8), 2.5: (0.78, 3), 3.5: (0.92, 20), 4.5: (0.99, 6)}

def grant_days(service, attendance):
    return GRANTS[service] if attendance >= 0.8 else 0

lots = []
for service, (attendance, taken) in history.items():
    expired = sum(days for granted, days in lots if service - granted >= 2)
    lots = [[g, d] for g, d in lots if service - g < 2]
    new = grant_days(service, attendance)
    if new:
        lots.append([service, new])
    want, used = taken, 0
    for lot in lots:
        use = min(lot[1], want - used)
        lot[1] -= use
        used += use
    balance = sum(d for _, d in lots)
    notes = []
    if expired:
        notes.append(f"{expired} day(s) expired")
    if not new:
        notes.append(f"no grant (attendance {attendance:.0%} < 80%)")
    if new >= 10 and used < 5:
        notes.append("fewer than 5 days taken: employer breaches the 5-day rule")
    if used < taken:
        notes.append(f"wanted {taken} but only {used} available")
    print(f"year {service}: +{new:>2} granted, {used:>2} taken, balance {balance:>2}" + (f"  ({'; '.join(notes)})" if notes else ""))

@@exercise Plan your last day
Given the day you give notice, the company's notice period, your remaining leave days and the bonus payment date, compute the official last day (退職日), the last working day (after 有給消化, counting weekdays only), and whether you keep the bonus. Then find the earliest notice date that still keeps the bonus.
@@concepts functions, loops, conditions
@@starter
from datetime import date, timedelta

NOTICE_DAYS = 30          # company rule; the legal minimum is 14
BONUS_DATE = date(2026, 12, 10)
LEAVE_LEFT = 12

def plan(notice_given, handover_days):
    raise NotImplementedError
@@expected
notice 2026-10-15: last working day 2026-10-29, 退職日 2026-11-16, bonus lost
notice 2026-11-02: last working day 2026-11-16, 退職日 2026-12-02, bonus lost
notice 2026-11-20: last working day 2026-11-27, 退職日 2026-12-20, bonus kept
earliest notice date with 10 handover days that keeps the bonus: 2026-11-10
@@requires
def plan
timedelta
weekday()
@@hint
退職日 = the later of (notice + NOTICE_DAYS) and (the date after the handover's working days plus all leave days, counting weekdays only). The last working day is the end of the handover. You keep the bonus if 退職日 ≥ BONUS_DATE. Write a helper that adds n weekdays to a date (skipping `weekday() >= 5`).
@@solution
from datetime import date, timedelta

NOTICE_DAYS = 30          # company rule; the legal minimum is 14
BONUS_DATE = date(2026, 12, 10)
LEAVE_LEFT = 12

def add_weekdays(start, n):
    day = start
    while n > 0:
        day += timedelta(days=1)
        if day.weekday() < 5:
            n -= 1
    return day

def plan(notice_given, handover_days):
    last_working = add_weekdays(notice_given, handover_days)
    leave_end = add_weekdays(last_working, LEAVE_LEFT)
    taishoku = max(notice_given + timedelta(days=NOTICE_DAYS), leave_end)
    return last_working, taishoku, taishoku >= BONUS_DATE

for notice, handover in ((date(2026, 10, 15), 10), (date(2026, 11, 2), 10), (date(2026, 11, 20), 5)):
    last_working, taishoku, bonus = plan(notice, handover)
    print(f"notice {notice}: last working day {last_working}, 退職日 {taishoku}, bonus {'kept' if bonus else 'lost'}")

day = date(2026, 10, 1)
while not plan(day, 10)[2]:
    day += timedelta(days=1)
print(f"earliest notice date with 10 handover days that keeps the bonus: {day}")
