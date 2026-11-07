@@lesson jc0-listings-cities Reading Job Listings and Comparing Cities
@@lang python
@@text
Job listings in Japan follow a fairly fixed layout (募集要項), and once you know the fields you can compare dozens of them quickly. Some fields hide the information that matters most: whether the role is really SES (you'd be placed at a client site), how much Japanese you'll actually use, and what the salary means once you've paid rent in that city. This lesson covers the fields, the signals, and a way to compare cities. All amounts are **example values for 2026** to show the method; check current listings and rents yourself.

## The fields of a 募集要項

| Field | Meaning | What to look for |
|---|---|---|
| 職種 / 仕事内容 | role and duties | specific product names vs vague 「システム開発全般」 |
| 必須スキル / 歓迎スキル | required / nice-to-have skills | 必須 is the real bar; 歓迎 is negotiable |
| 日本語レベル | Japanese level | 「ビジネスレベル」≈ N2+ in practice; 「日常会話」≈ N3 |
| 雇用形態 | employment type | 正社員 (permanent), 契約社員 (fixed-term), 派遣 (dispatched) |
| 勤務地 | workplace | 「客先常駐」 or 「プロジェクトにより異なる」 suggests SES |
| 想定年収 | expected annual pay | a range; offers usually land in the lower half unless you negotiate |
| 待遇・福利厚生 | benefits | 通勤手当, 住宅手当, リモート, フレックス, 賞与 (bonus count) |
| 休日休暇 | holidays | 完全週休2日制 (every weekend off) vs 週休2日制 (at least one week a month with two days off) |

**SES signals**: 客先常駐, 「大手SIerのプロジェクトに参画」, 勤務地 listed as "Tokyo 23 wards, depending on project", and many identical listings from one company. SES isn't always bad (it's a common entry point), but know what you're signing.

## Comparing cities

Tokyo pays the most, but rent takes much of the difference. A fair comparison looks at **money left after rent**, plus things like how common remote work is and how many English-friendly employers there are.

- **通勤手当** (commuting allowance) is usually paid in full and is **tax-free up to ¥150,000/month**, so the commute doesn't come out of your salary.
- Osaka, Fukuoka and Nagoya: lower rents, fewer international employers, more 社内SE and manufacturing IT roles.
- Remote-first companies sometimes pay Tokyo rates anywhere, which is the best of both.

@@quiz
A listing says 「客先常駐」. What does that usually mean?
* You will work at a client's site, typical of SES
- Remote work from home
- A permanent position in the company's own office
- Part-time work
? Ask which client and for how long.

@@quiz
What is the difference between 完全週休2日制 and 週休2日制?
* The first means two days off every week; the second only guarantees it at least one week per month
- They mean the same
- The second has more holidays
- The first is only for managers
? Read holiday fields carefully.

@@quiz
A listing asks for 「ビジネスレベルの日本語」. What does that usually mean in practice?
- N5
- N4
- N3
* N2 or above, plus meeting and email ability
? 日常会話レベル is closer to N3.

@@quiz
Why compare cities by money left after rent rather than by salary?
* Higher Tokyo salaries are partly absorbed by higher rent
- Rent is tax-deductible
- Salaries are the same everywhere
- Commuting costs are always deducted from salary
? 通勤手当 usually covers commuting separately.

@@flashcards
募集要項 :: job listing details
必須スキル :: required skills
歓迎スキル :: nice-to-have skills
雇用形態 :: employment type
正社員 :: permanent employee
契約社員 :: fixed-term contract employee
派遣 :: dispatched worker
客先常駐 :: stationed at a client site (SES)
想定年収 :: expected annual salary range
通勤手当 :: commuting allowance, tax-free to ¥150k/month
住宅手当 :: housing allowance
完全週休2日制 :: two days off every week
賞与 :: bonus

@@drill
What is the Japanese term for a permanent full-time employee?
= 正社員
= seishain
? 契約社員 is fixed-term.

@@drill
Up to how many yen per month is the commuting allowance tax-free?
= 150000
= 150,000
= ¥150,000
? It's paid on top of salary in most companies.

@@exercise Screen and rank job listings
Check listings against your profile (Japanese level, years of experience, skills), flag SES signals and fixed-term contracts, and rank the matches by the midpoint of the salary range, with a bonus for remote work.
@@concepts lists, conditions, strings, numbers, maps, functions
@@starter
profile = {"jlpt": 3, "years": 4, "skills": {"python", "aws", "docker", "sql"}}
listings = [
    {"company": "A社 (SaaS)", "jlpt": 2, "years": 3, "required": {"python", "aws"}, "salary": (6_000_000, 8_500_000),
     "type": "正社員", "place": "渋谷本社", "remote": True},
    {"company": "B社 (SIer)", "jlpt": 3, "years": 2, "required": {"java"}, "salary": (4_500_000, 6_000_000),
     "type": "正社員", "place": "客先常駐（都内）", "remote": False},
    {"company": "C社 (外資系)", "jlpt": 4, "years": 3, "required": {"aws", "docker"}, "salary": (8_000_000, 11_000_000),
     "type": "正社員", "place": "港区オフィス", "remote": True},
    {"company": "D社 (SES)", "jlpt": 3, "years": 1, "required": {"python"}, "salary": (4_000_000, 5_500_000),
     "type": "正社員", "place": "プロジェクトにより異なる", "remote": False},
    {"company": "E社 (startup)", "jlpt": 3, "years": 3, "required": {"python", "sql"}, "salary": (5_500_000, 7_500_000),
     "type": "契約社員", "place": "福岡オフィス", "remote": True},
]
SES_SIGNALS = ("客先常駐", "プロジェクトにより")

def check(listing):
    raise NotImplementedError
@@expected
skip  A社 (SaaS): needs N2 (you have N3)
skip  B社 (SIer): missing java
match C社 (外資系): ¥800万-¥1,100万, score ¥1,000万
match E社 (startup): ¥550万-¥750万, score ¥700万 [契約社員: ask about renewal and conversion]
match D社 (SES): ¥400万-¥550万, score ¥475万 [SES signal: プロジェクトにより異なる]
@@requires
def check
SES_SIGNALS
required
sorted(
@@hint
JLPT levels: a lower number is harder, so you qualify when `profile["jlpt"] <= listing["jlpt"]`. Collect the reasons a listing doesn't match (Japanese level, years, missing required skills). Separately collect warnings (SES signals in `place`, 契約社員). Score matches by the salary midpoint, plus ¥500,000 for remote.
@@solution
profile = {"jlpt": 3, "years": 4, "skills": {"python", "aws", "docker", "sql"}}
listings = [
    {"company": "A社 (SaaS)", "jlpt": 2, "years": 3, "required": {"python", "aws"}, "salary": (6_000_000, 8_500_000),
     "type": "正社員", "place": "渋谷本社", "remote": True},
    {"company": "B社 (SIer)", "jlpt": 3, "years": 2, "required": {"java"}, "salary": (4_500_000, 6_000_000),
     "type": "正社員", "place": "客先常駐（都内）", "remote": False},
    {"company": "C社 (外資系)", "jlpt": 4, "years": 3, "required": {"aws", "docker"}, "salary": (8_000_000, 11_000_000),
     "type": "正社員", "place": "港区オフィス", "remote": True},
    {"company": "D社 (SES)", "jlpt": 3, "years": 1, "required": {"python"}, "salary": (4_000_000, 5_500_000),
     "type": "正社員", "place": "プロジェクトにより異なる", "remote": False},
    {"company": "E社 (startup)", "jlpt": 3, "years": 3, "required": {"python", "sql"}, "salary": (5_500_000, 7_500_000),
     "type": "契約社員", "place": "福岡オフィス", "remote": True},
]
SES_SIGNALS = ("客先常駐", "プロジェクトにより")

def check(listing):
    gaps, warnings = [], []
    if profile["jlpt"] > listing["jlpt"]:
        gaps.append(f"needs N{listing['jlpt']} (you have N{profile['jlpt']})")
    if profile["years"] < listing["years"]:
        gaps.append(f"needs {listing['years']} years")
    missing = listing["required"] - profile["skills"]
    if missing:
        gaps.append("missing " + ", ".join(sorted(missing)))
    if any(s in listing["place"] for s in SES_SIGNALS):
        warnings.append("SES signal: " + listing["place"])
    if listing["type"] != "正社員":
        warnings.append(f"{listing['type']}: ask about renewal and conversion")
    return gaps, warnings

matches = []
for listing in listings:
    gaps, warnings = check(listing)
    if gaps:
        print(f"skip  {listing['company']}: {'; '.join(gaps)}")
        continue
    low, high = listing["salary"]
    score = (low + high) / 2 + (500_000 if listing["remote"] else 0)
    matches.append((score, listing, warnings))
for score, listing, warnings in sorted(matches, key=lambda m: -m[0]):
    low, high = listing["salary"]
    note = f" [{'; '.join(warnings)}]" if warnings else ""
    print(f"match {listing['company']}: ¥{low / 10_000:,.0f}万-¥{high / 10_000:,.0f}万, score ¥{score / 10_000:,.0f}万{note}")

@@exercise Compare cities after rent
For offers in different cities, estimate monthly take-home pay (using a simple 77% of gross, a rough figure for this salary band), subtract rent for a 1LDK, and compare what's left. Commuting costs are covered by 通勤手当, so they're ignored. Also show the gross salary Fukuoka would need to match Tokyo after rent.
@@starter
TAKE_HOME = 0.77   # rough example ratio, not tax advice
offers = {"Tokyo (渋谷)": 7_500_000, "Osaka (梅田)": 6_400_000, "Fukuoka (天神)": 5_800_000, "Remote, Tokyo rate, living in Fukuoka": 7_500_000}
rent_1ldk = {"Tokyo (渋谷)": 210_000, "Osaka (梅田)": 130_000, "Fukuoka (天神)": 95_000, "Remote, Tokyo rate, living in Fukuoka": 95_000}

def left_after_rent(city):
    raise NotImplementedError
@@expected
Remote, Tokyo rate, living in Fukuoka  take-home ¥ 481,250/month, rent ¥ 95,000 (20%), left ¥ 386,250
Osaka (梅田)                             take-home ¥ 410,667/month, rent ¥130,000 (32%), left ¥ 280,667
Fukuoka (天神)                           take-home ¥ 372,167/month, rent ¥ 95,000 (26%), left ¥ 277,167
Tokyo (渋谷)                             take-home ¥ 481,250/month, rent ¥210,000 (44%), left ¥ 271,250
Fukuoka matches Tokyo after rent at a gross salary of about ¥571万
@@requires
def left_after_rent
TAKE_HOME
rent_1ldk
sorted(
@@hint
Monthly take-home = annual × TAKE_HOME ÷ 12. Subtract the rent. To match Tokyo, Fukuoka needs (Tokyo's amount left + Fukuoka rent) × 12 ÷ TAKE_HOME in gross salary.
@@solution
TAKE_HOME = 0.77   # rough example ratio, not tax advice
offers = {"Tokyo (渋谷)": 7_500_000, "Osaka (梅田)": 6_400_000, "Fukuoka (天神)": 5_800_000, "Remote, Tokyo rate, living in Fukuoka": 7_500_000}
rent_1ldk = {"Tokyo (渋谷)": 210_000, "Osaka (梅田)": 130_000, "Fukuoka (天神)": 95_000, "Remote, Tokyo rate, living in Fukuoka": 95_000}

def left_after_rent(city):
    monthly = offers[city] * TAKE_HOME / 12
    return monthly, monthly - rent_1ldk[city]

results = {city: left_after_rent(city) for city in offers}
for city, (monthly, left) in sorted(results.items(), key=lambda kv: -kv[1][1]):
    share = rent_1ldk[city] / monthly
    print(f"{city:<38} take-home ¥{monthly:>8,.0f}/month, rent ¥{rent_1ldk[city]:>7,} ({share:.0%}), left ¥{left:>8,.0f}")
tokyo_left = results["Tokyo (渋谷)"][1]
needed = (tokyo_left + rent_1ldk["Fukuoka (天神)"]) * 12 / TAKE_HOME
print(f"Fukuoka matches Tokyo after rent at a gross salary of about ¥{needed / 10_000:,.0f}万")
