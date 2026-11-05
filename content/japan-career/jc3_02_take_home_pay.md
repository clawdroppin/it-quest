@@lesson jc3-take-home From 年収 to 手取り: Social Insurance, Income Tax and Resident Tax
@@text
A ¥6,000,000 offer doesn't mean ¥500,000 a month in your bank account. Japanese payslips (給与明細) deduct social insurance, income tax and, from your second year, resident tax. Knowing how each is computed lets you compare offers, budget for rent, and avoid the second-year surprise. **All rates below are approximate 2026 example values for an employee under 40 in Tokyo; they change every year and by prefecture and insurer. Check your payslip, your 健康保険組合 and the National Tax Agency (国税庁) for current figures.** This is a simplified model for budgeting, not tax advice.

## Social insurance (社会保険), employee share

| Item | Approximate employee rate | Notes |
|---|---|---|
| **健康保険** (health) | about 5% | half of ~10%; varies by prefecture and 健保組合 |
| **介護保険** (long-term care) | about 0.8% | only from age 40 |
| **厚生年金** (pension) | **9.15%** | half of 18.3%; capped at a maximum monthly salary grade |
| **雇用保険** (employment) | about 0.5–0.6% | |

Contributions are calculated on a **standard monthly remuneration** (標準報酬月額) grade, set from April–June pay, so overtime in those months raises them for the whole year. Bonuses are also charged. Social insurance is **fully deductible** from taxable income.

## Income tax (所得税)

1. Start from gross salary (給与収入).
2. Subtract the **employment income deduction** (給与所得控除): at least ¥650,000; e.g. 20% + ¥440,000 for ¥3.6M–6.6M, capped at ¥1.95M.
3. Subtract social insurance paid and the **basic deduction** (基礎控除, ¥580,000 for most incomes after the 2025 reform; more at low incomes), plus dependants, iDeCo, life insurance etc.
4. Apply **progressive rates** to taxable income: 5% (to ¥1.95M), 10% (to ¥3.3M), 20% (to ¥6.95M), 23% (to ¥9M), 33% (to ¥18M), 40%, 45%, using the quick deduction table (e.g. 20% − ¥427,500).
5. Add the **2.1% reconstruction surtax** (復興特別所得税) on the tax.
Employers withhold monthly (源泉徴収) and settle the difference in **年末調整** (year-end adjustment) in December. Side income or more than ¥20M salary requires a **確定申告** (tax return) by 15 March.

## Resident tax (住民税)

About **10%** of taxable income (computed with a smaller basic deduction of ¥430,000) plus a flat 均等割 (about ¥5,000). It's based on **last year's** income and paid from **June of the following year** to May. So in your first year in Japan you pay none, and your take-home drops when it starts. Leaving Japan mid-year? You still owe the rest of that year's resident tax; arrange it before departure (a 納税管理人, tax agent, can handle it).

**ふるさと納税** (hometown tax donations) let you redirect part of your resident tax to other municipalities and receive gifts, paying ¥2,000 yourself.

@@quiz
Which deduction starts in your second year in Japan?
- Pension
- Health insurance
* Resident tax (住民税)
- Employment insurance
? It's based on the previous year's income.

@@quiz
About what share of salary is the employee's pension contribution?
- 2%
* 9.15%
- 18.3%
- 25%
? The employer pays the other half.

@@quiz
Why can heavy overtime in April to June raise your deductions all year?
* Social insurance grades are set from those months' pay
- Tax rates rise in summer
- Bonuses are paid then
- Resident tax is set in April
? 標準報酬月額.

@@quiz
What is 年末調整?
- A summer bonus
* The employer's year-end settlement of withheld income tax
- A pension refund
- A visa renewal
? Done in December.

@@quiz
Which surtax is added to national income tax?
- 1% consumption surtax
* 2.1% reconstruction special income tax
- 10% city tax
- None
? 復興特別所得税.

@@flashcards
給与明細 :: payslip
手取り :: take-home pay
社会保険 :: social insurance
健康保険 :: health insurance, about 5%
介護保険 :: long-term care, from 40
厚生年金 :: employee pension, 9.15%
雇用保険 :: employment insurance
標準報酬月額 :: standard monthly remuneration grade
給与所得控除 :: employment income deduction
基礎控除 :: basic deduction
復興特別所得税 :: 2.1% surtax
源泉徴収 :: monthly withholding
年末調整 :: year-end adjustment
確定申告 :: tax return by 15 March
住民税 :: resident tax, about 10%, from June next year
ふるさと納税 :: hometown tax donations

@@drill
In which month does resident tax deduction usually start each year?
= June
= 6月
= june
? It runs from June to the following May.

@@drill
What is the Japanese term for take-home pay?
= 手取り
= tedori
= てどり
? 額面 is the gross amount.

@@exercise Estimate take-home pay
Estimate annual and monthly take-home pay for several salaries using the simplified model and the example rates in the starter: social insurance (with the pension's monthly cap), the employment income deduction, the basic deduction, progressive income tax with the 2.1% surtax, and resident tax (only from year two). Print year one and year two side by side.
@@lang python
@@starter
HEALTH, CARE, PENSION, EMPLOYMENT = 0.05, 0.008, 0.0915, 0.0055
PENSION_MONTHLY_CAP = 650_000
BRACKETS = [(1_950_000, 0.05, 0), (3_300_000, 0.10, 97_500), (6_950_000, 0.20, 427_500), (9_000_000, 0.23, 636_000),
            (18_000_000, 0.33, 1_536_000), (40_000_000, 0.40, 2_796_000), (float("inf"), 0.45, 4_796_000)]
BASIC_NATIONAL, BASIC_RESIDENT, RESIDENT_RATE, RESIDENT_FLAT = 580_000, 430_000, 0.10, 5_000

def employment_deduction(gross):
    raise NotImplementedError
@@expected
¥4,000,000 (age 28): social ¥588,000, income tax ¥81,272, resident ¥179,200
   take-home year 1 ¥3,330,728 (¥277,561/month), year 2 ¥3,151,528 (¥262,627/month, 79%)
¥6,000,000 (age 32): social ¥882,000, income tax ¥196,338, resident ¥309,800
   take-home year 1 ¥4,921,662 (¥410,138/month), year 2 ¥4,611,862 (¥384,322/month, 77%)
¥9,000,000 (age 41): social ¥1,285,200, income tax ¥622,259, resident ¥538,480
   take-home year 1 ¥7,092,541 (¥591,045/month), year 2 ¥6,554,061 (¥546,172/month, 73%)
¥12,000,000 (age 35): social ¥1,379,700, income tax ¥1,250,489, resident ¥829,030
   take-home year 1 ¥9,369,811 (¥780,818/month), year 2 ¥8,540,781 (¥711,732/month, 71%)
@@requires
def employment_deduction
def income_tax
1.021
@@hint
Employment deduction: ¥650,000 up to ¥1.9M; 30% + ¥80,000 to ¥3.6M; 20% + ¥440,000 to ¥6.6M; 10% + ¥1,100,000 to ¥8.5M; ¥1,950,000 above. Pension = `min(gross / 12, cap) * 12 * PENSION`. Taxable = gross − employment deduction − social insurance − basic deduction (not below 0). Income tax = `(taxable * rate - quick) * 1.021` for the first bracket that fits.
@@solution
HEALTH, CARE, PENSION, EMPLOYMENT = 0.05, 0.008, 0.0915, 0.0055
PENSION_MONTHLY_CAP = 650_000
BRACKETS = [(1_950_000, 0.05, 0), (3_300_000, 0.10, 97_500), (6_950_000, 0.20, 427_500), (9_000_000, 0.23, 636_000),
            (18_000_000, 0.33, 1_536_000), (40_000_000, 0.40, 2_796_000), (float("inf"), 0.45, 4_796_000)]
BASIC_NATIONAL, BASIC_RESIDENT, RESIDENT_RATE, RESIDENT_FLAT = 580_000, 430_000, 0.10, 5_000

def employment_deduction(gross):
    if gross <= 1_900_000:
        return 650_000
    if gross <= 3_600_000:
        return gross * 0.3 + 80_000
    if gross <= 6_600_000:
        return gross * 0.2 + 440_000
    if gross <= 8_500_000:
        return gross * 0.1 + 1_100_000
    return 1_950_000

def social(gross, age):
    pension = min(gross / 12, PENSION_MONTHLY_CAP) * 12 * PENSION
    return gross * (HEALTH + EMPLOYMENT + (CARE if age >= 40 else 0)) + pension

def income_tax(taxable):
    for limit, rate, quick in BRACKETS:
        if taxable <= limit:
            return max(0, taxable * rate - quick) * 1.021

for gross, age in ((4_000_000, 28), (6_000_000, 32), (9_000_000, 41), (12_000_000, 35)):
    si = social(gross, age)
    income = gross - employment_deduction(gross)
    tax = income_tax(max(0, income - si - BASIC_NATIONAL))
    resident = max(0, income - si - BASIC_RESIDENT) * RESIDENT_RATE + RESIDENT_FLAT
    year1 = gross - si - tax
    year2 = year1 - resident
    print(f"¥{gross:,} (age {age}): social ¥{si:,.0f}, income tax ¥{tax:,.0f}, resident ¥{resident:,.0f}")
    print(f"   take-home year 1 ¥{year1:,.0f} (¥{year1 / 12:,.0f}/month), year 2 ¥{year2:,.0f} (¥{year2 / 12:,.0f}/month, {year2 / gross:.0%})")

@@exercise When does resident tax hit?
Given arrival and income per calendar year, compute the resident tax bill for each fiscal period (June to May, based on the previous calendar year's income), and print a month-by-month deduction schedule for the first 30 months after arrival, highlighting the month when resident tax starts.
@@lang python
@@starter
ARRIVAL = (2026, 4)  # year, month
income_by_year = {2026: 4_500_000, 2027: 6_200_000, 2028: 6_500_000}  # taxable-for-resident-tax income, simplified
RATE, FLAT = 0.10, 5_000

def resident_tax_for(calendar_year):
    raise NotImplementedError
@@expected
Apr 2026: ¥0/month
Jun 2026: ¥0/month
Dec 2026: ¥0/month
Jun 2027: ¥37,916/month  <- resident tax starts (June)
Dec 2027: ¥37,916/month
Jun 2028: ¥52,083/month
income in 2026 -> bill ¥455,000 paid June 2027 to May 2028
income in 2027 -> bill ¥625,000 paid June 2028 to May 2029
income in 2028 -> bill ¥655,000 paid June 2029 to May 2030
@@requires
def resident_tax_for
June
// 12
@@hint
Resident tax for the period June Y+1 to May Y+2 is `income_by_year[Y] * RATE + FLAT`, or 0 if you had no Japanese income in Y. A month (y, m) falls in the period that started in June of `y` if `m >= 6`, else June of `y - 1`; that period uses the income of the year before it started. Monthly amount = bill / 12 (rounded down to yen). Step through months with `(month - 1) // 12`.
@@solution
ARRIVAL = (2026, 4)  # year, month
income_by_year = {2026: 4_500_000, 2027: 6_200_000, 2028: 6_500_000}  # taxable-for-resident-tax income, simplified
RATE, FLAT = 0.10, 5_000
MONTHS = ["Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec"]

def resident_tax_for(calendar_year):
    income = income_by_year.get(calendar_year, 0)
    return int(income * RATE + FLAT) if income else 0

start_index = ARRIVAL[0] * 12 + ARRIVAL[1] - 1
previous = 0
for i in range(start_index, start_index + 30):
    y, m = i // 12, i % 12 + 1
    period_start = y if m >= 6 else y - 1
    monthly = resident_tax_for(period_start - 1) // 12
    note = "  <- resident tax starts (June)" if monthly and not previous else ""
    if m in (6, 12) or note or i == start_index:
        print(f"{MONTHS[m - 1]} {y}: ¥{monthly:,}/month{note}")
    previous = monthly
for y in sorted(income_by_year):
    print(f"income in {y} -> bill ¥{resident_tax_for(y):,} paid June {y + 1} to May {y + 2}")
