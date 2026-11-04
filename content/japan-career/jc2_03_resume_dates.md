@@lesson jc2-resume-dates 履歴書 and 職務経歴書 in Detail: 和暦, 学歴・職歴 and Career History
@@text
Japanese application documents are judged on correctness of form as much as content. The 履歴書 (rirekisho) has fixed sections and conventions: dates in a single consistent calendar (often 和暦, the Japanese era calendar), education and work history lines with set phrases, and 以上 at the end. The 職務経歴書 (shokumu keirekisho) is freer and is where you sell your skills. This lesson covers the conventions and has you build the date and history logic, which is where mistakes are most visible.

## 和暦 (Japanese era years)

| Era | Started | Year 1 |
|---|---|---|
| 昭和 (Shōwa) | 1926-12-25 | 1926 |
| 平成 (Heisei) | 1989-01-08 | 1989 |
| 令和 (Reiwa) | 2019-05-01 | 2019 |

- Era year = Western year − era start year + 1. The first year is written **元年** (gannen): 令和元年 = 2019 (from 1 May).
- The era changes on its start date: 2019-04-30 is 平成31年; 2019-05-01 is 令和元年. 1989-01-07 is 昭和64年.
- **Use one system consistently** across the whole document (all 和暦 or all 西暦), including the date at the top (the submission date) and in the 学歴・職歴 table. Many companies accept 西暦 now, especially for foreign applicants and IT jobs.

## The 学歴・職歴 table

```text
年      月  学歴・職歴
                学歴
令和2   4   ○○大学 工学部 情報工学科 入学
令和6   3   ○○大学 工学部 情報工学科 卒業
                職歴
令和6   4   株式会社△△ 入社
            運用保守部にてサーバー監視、障害対応に従事
令和8   3   一身上の都合により退職
令和8   4   株式会社□□ 入社
            現在に至る
                                    以上
```

Conventions:
- Write 学歴 first (usually from high school graduation; for foreign education, give the school name and country), then 職歴.
- Companies with full names (株式会社 not (株)); **入社** (joined) / **退社** or **退職** (left). For a voluntary departure write **一身上の都合により退職**; for a contract ending, **契約期間満了により退職**; for a company closure, **会社都合により退職**.
- Current job: **現在に至る** (up to the present). End the table with **以上** right-aligned.
- Age is stated as of the submission date (満○歳).
- Certifications (免許・資格) with the date obtained: 基本情報技術者試験 合格, AWS Certified Solutions Architect – Associate 取得, 日本語能力試験 N2 合格.

## The 職務経歴書

Usually A4, 1–3 pages, computer-written. Common structure: 職務要約 (summary), 職務経歴 (projects: period, company, role, team size, environment/technologies, responsibilities, results with numbers), 活かせる経験・スキル (skills), 資格, 自己PR. Formats: **編年体** (chronological), **逆編年体** (reverse chronological: most recent first, common in IT), **キャリア式** (grouped by skill area).

Write results with numbers ("監視アラートを40%削減", "問い合わせ対応 月120件") and technologies precisely (OS, cloud, languages, tools, versions).

@@quiz
How is the year 2019 from 1 May written in 和暦?
- 平成31年
* 令和元年
- 令和1年 only
- 平成元年
? 元年 is the first year of an era.

@@quiz
What era and year is 30 April 2019?
* 平成31年
- 令和元年
- 平成30年
- 令和0年
? The era changed on 1 May.

@@quiz
Which phrase ends the 学歴・職歴 table?
- 現在に至る
* 以上
- 終わり
- 以下余白
? 現在に至る marks the current job.

@@quiz
How do you write a voluntary resignation in the 職歴?
- 会社都合により退職
* 一身上の都合により退職
- 解雇
- 契約期間満了により退職
? It's the standard neutral phrase.

@@quiz
Which 職務経歴書 format lists the most recent job first?
- 編年体
* 逆編年体
- キャリア式
- 自由形式
? Common in IT.

@@quiz
What is 2026 in 令和?
- 令和7年
* 令和8年
- 令和6年
- 令和9年
? 2026 − 2019 + 1.

@@flashcards
和暦 :: Japanese era calendar
西暦 :: Western calendar
元年 :: first year of an era
昭和 start :: 1926-12-25
平成 start :: 1989-01-08
令和 start :: 2019-05-01
era year :: year − start year + 1
入学 / 卒業 :: entered / graduated
入社 :: joined a company
一身上の都合により退職 :: resigned for personal reasons
会社都合により退職 :: left due to the company
現在に至る :: up to the present
以上 :: end of the list
満○歳 :: full age
免許・資格 :: licences and certifications
職務要約 :: career summary
逆編年体 :: most recent first
キャリア式 :: grouped by skill
自己PR :: self-promotion

@@drill
What is 2020 in 令和 (write as 令和N年)?
= 令和2年
? 2020 − 2019 + 1 = 2.

@@drill
Which phrase follows your current job in the 職歴?
= 現在に至る
? Then 以上 at the end.

@@exercise Convert dates to 和暦
Convert dates to Japanese era dates, choosing the era by its exact start date and writing the first year as 元年. Print each date in the 履歴書 style `令和8年4月1日`.
@@lang python
@@starter
from datetime import date

ERAS = [("令和", date(2019, 5, 1)), ("平成", date(1989, 1, 8)), ("昭和", date(1926, 12, 25))]
dates = [date(2026, 11, 4), date(2019, 5, 1), date(2019, 4, 30), date(2020, 3, 31), date(1989, 1, 7), date(1989, 1, 8), date(1998, 7, 15), date(1926, 12, 25)]

def wareki(d):
    raise NotImplementedError
@@expected
2026-11-04 -> 令和8年11月4日
2019-05-01 -> 令和元年5月1日
2019-04-30 -> 平成31年4月30日
2020-03-31 -> 令和2年3月31日
1989-01-07 -> 昭和64年1月7日
1989-01-08 -> 平成元年1月8日
1998-07-15 -> 平成10年7月15日
1926-12-25 -> 昭和元年12月25日
@@requires
def wareki
'元'
ERAS
@@hint
Loop over ERAS (newest first) and take the first era whose start date is ≤ d. Year = `d.year - start.year + 1`; write 元 when it's 1.
@@solution
from datetime import date

ERAS = [("令和", date(2019, 5, 1)), ("平成", date(1989, 1, 8)), ("昭和", date(1926, 12, 25))]
dates = [date(2026, 11, 4), date(2019, 5, 1), date(2019, 4, 30), date(2020, 3, 31), date(1989, 1, 7), date(1989, 1, 8), date(1998, 7, 15), date(1926, 12, 25)]

def wareki(d):
    for name, start in ERAS:
        if d >= start:
            year = d.year - start.year + 1
            return f"{name}{'元' if year == 1 else year}年{d.month}月{d.day}日"
    raise ValueError("before 昭和")

for d in dates:
    print(f"{d.isoformat()} -> {wareki(d)}")

@@exercise Build the 学歴・職歴 table
Generate the 学歴・職歴 rows from structured data, with the year in 和暦 (no day) and the standard phrases: 入学/卒業 for school, 入社 for joining, the right departure phrase by reason, an indented detail line for each job's duties, 現在に至る for the current job, and a right-aligned 以上. Also print the applicant's full age (満○歳) on the submission date.
@@lang python
@@starter
from datetime import date

ERAS = [("令和", date(2019, 5, 1)), ("平成", date(1989, 1, 8)), ("昭和", date(1926, 12, 25))]
LEAVE = {"personal": "一身上の都合により退職", "contract": "契約期間満了により退職", "company": "会社都合により退職"}
birthday = date(1999, 8, 20)
submitted = date(2026, 11, 4)
education = [("Warsaw University of Technology (Poland), Faculty of Electronics", date(2018, 10, 1), date(2022, 9, 30))]
jobs = [
    ("株式会社サンプル情報システム", date(2022, 11, 1), date(2024, 3, 31), "personal", "ヘルプデスクにて社内問い合わせ対応 (月120件) に従事"),
    ("株式会社テストクラウド", date(2024, 4, 1), None, None, "AWS環境の運用保守、監視設計を担当"),
]

def era_year_month(d):
    raise NotImplementedError
@@expected
令和8年11月4日現在  満27歳
            学歴
平成30   10  Warsaw University of Technology (Poland), Faculty of Electronics 入学
令和4     9  Warsaw University of Technology (Poland), Faculty of Electronics 卒業
            職歴
令和4    11  株式会社サンプル情報システム 入社
           ヘルプデスクにて社内問い合わせ対応 (月120件) に従事
令和6     3  一身上の都合により退職
令和6     4  株式会社テストクラウド 入社
           AWS環境の運用保守、監視設計を担当
           現在に至る
                                        以上
@@requires
def era_year_month
現在に至る
以上
@@hint
`era_year_month(d)` returns `(era_and_year, month)` such as `("令和6", 4)`, with 元 for the first year. Full age: `submitted.year - birthday.year - ((submitted.month, submitted.day) < (birthday.month, birthday.day))`.
@@solution
from datetime import date

ERAS = [("令和", date(2019, 5, 1)), ("平成", date(1989, 1, 8)), ("昭和", date(1926, 12, 25))]
LEAVE = {"personal": "一身上の都合により退職", "contract": "契約期間満了により退職", "company": "会社都合により退職"}
birthday = date(1999, 8, 20)
submitted = date(2026, 11, 4)
education = [("Warsaw University of Technology (Poland), Faculty of Electronics", date(2018, 10, 1), date(2022, 9, 30))]
jobs = [
    ("株式会社サンプル情報システム", date(2022, 11, 1), date(2024, 3, 31), "personal", "ヘルプデスクにて社内問い合わせ対応 (月120件) に従事"),
    ("株式会社テストクラウド", date(2024, 4, 1), None, None, "AWS環境の運用保守、監視設計を担当"),
]

def era_year_month(d):
    for name, start in ERAS:
        if d >= start:
            year = d.year - start.year + 1
            return f"{name}{'元' if year == 1 else year}", d.month
    raise ValueError("before 昭和")

def row(d, text):
    era, month = era_year_month(d)
    return f"{era:<6} {month:>2}  {text}"

age = submitted.year - birthday.year - ((submitted.month, submitted.day) < (birthday.month, birthday.day))
era, month = era_year_month(submitted)
print(f"{era}年{month}月{submitted.day}日現在  満{age}歳")
print(" " * 12 + "学歴")
for school, start, end in education:
    print(row(start, school + " 入学"))
    print(row(end, school + " 卒業"))
print(" " * 12 + "職歴")
for company, start, end, reason, duty in jobs:
    print(row(start, company + " 入社"))
    print(" " * 11 + duty)
    if end:
        print(row(end, LEAVE[reason]))
    else:
        print(" " * 11 + "現在に至る")
print(" " * 40 + "以上")
