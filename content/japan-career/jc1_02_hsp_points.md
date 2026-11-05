@@lesson jc1-hsp-points Highly Skilled Professional Points in Practice
@@text
For many engineers the **Highly Skilled Professional (高度専門職)** status is worth aiming for: a 5-year stay, more freedom in side activities, a working spouse, sometimes parents or a domestic helper, and a fast track to permanent residence. Whether you qualify comes down to a points table. This lesson explains the table for the **advanced specialised/technical** category (高度専門・技術活動, the one most IT engineers use), how to document each item, and how to plan to reach 70 or 80 points. **The figures below are a simplified summary of the official table as of 2026. Always check the current ポイント計算表 on the Immigration Services Agency (出入国在留管理庁) website before applying.** This is general information, not immigration advice; an 行政書士 (administrative scrivener) specialising in visas can check your case.

## The main point items (advanced specialised/technical)

| Item | Points (simplified) |
|---|---|
| **Academic**: doctorate / master's (or professional degree) / bachelor's | 30 / 20 / 10 (+5 for two or more of the higher degrees in different fields) |
| **Work experience** in the field: 10+ / 7+ / 5+ / 3+ years | 20 / 15 / 10 / 5 |
| **Annual salary** (from the Japanese employer) | ¥10M: 40, ¥9M: 35, ¥8M: 30, ¥7M: 25, ¥6M: 20, ¥5M: 15, ¥4M: 10, with age limits (below) |
| **Age** at application: under 30 / 30–34 / 35–39 | 15 / 10 / 5 |
| **Japanese**: JLPT N1 (or a Japanese-taught degree) / N2 | 15 / 10 |
| **Degree from a Japanese university** | 10 |
| **Qualifications** listed by the ministry (some national IT exams) | 5 for one, 10 for two or more |
| Employer bonuses (innovation support, research spending), research achievements, top-ranked universities, certain training | various |

**Salary age limits**: ¥7M and ¥6M count only up to age 39; ¥5M only up to 34; ¥4M only up to 29. ¥8M and above count at any age. Below **¥3M** annual salary you can't use this category at all.

The JLPT N1 and the Japanese-degree items don't stack in full: some combinations are capped, which the official form explains.

## Thresholds and benefits

- **70 points**: Highly Skilled Professional (i)(b); after 3 years you can apply for (ii) (no time limit), or apply for permanent residence after 3 years at 70+.
- **80 points**: permanent residence possible after **1 year**.
- Points are assessed at application **and** must be maintained for the permanent-residence route (salary drops matter).
- **J-Skip** (特別高度人材) bypasses points for very high earners with a master's degree or 10 years' experience (salary ¥20M+) or ¥40M+ alone.

## Documenting points

Degree certificates (with translation), employment certificates (在職証明書) showing roles and dates for experience, the employer's statement of salary, JLPT certificates, and qualification certificates. Experience must be in the relevant field; overlapping study and work periods may not both count.

@@quiz
How many points qualify for Highly Skilled Professional status?
- 50
* 70
- 80
- 100
? 80 opens the 1-year permanent residence route.

@@quiz
A 36-year-old engineer earns ¥5.5 million. How many salary points?
* 0, because ¥5M only counts up to age 34
- 15
- 20
- 10
? The salary age limits matter.

@@quiz
Which Japanese ability gives 15 points?
- N3
- N2
* N1 (or a degree taught in Japanese)
- Any JLPT pass
? N2 gives 10.

@@quiz
What does reaching 80 points allow?
- A 10-year visa
* Applying for permanent residence after 1 year
- Japanese citizenship
- Skipping tax
? 70 points means 3 years.

@@quiz
Below what annual salary can you not use the advanced specialised/technical category?
- ¥2M
* ¥3M
- ¥5M
- ¥10M
? A floor for the category.

@@flashcards
高度専門職 :: Highly Skilled Professional
高度専門・技術活動 :: category most IT engineers use
70 points :: HSP threshold
80 points :: PR after 1 year
master's degree :: 20 points
10+ years experience :: 20 points
¥10M salary :: 40 points
under 30 :: 15 age points
JLPT N1 :: 15 points
Japanese university degree :: 10 points
salary floor :: ¥3M
J-Skip :: 特別高度人材, very high earners
ポイント計算表 :: official points form
行政書士 :: visa paperwork specialist

@@drill
How many points does a master's degree give in the simplified table?
= 20
? Doctorate 30, bachelor's 10.

@@drill
How many points are needed to apply for permanent residence after one year as an HSP?
= 80
? 70 gives the 3-year route.

@@exercise Calculate HSP points
Implement a simplified points calculator for the advanced specialised/technical category using the table in the lesson (academic, experience, salary with age limits, age, Japanese ability, Japanese degree, qualifications), check the ¥3M salary floor, and report the total and what it allows for each candidate.
@@lang python
@@starter
candidates = {
    "Aiko (28, master's, N2)": {"age": 28, "degree": "master", "years": 4, "salary": 5_200_000, "jlpt": "N2", "jp_degree": False, "quals": 1},
    "Ben (36, bachelor's)": {"age": 36, "degree": "bachelor", "years": 12, "salary": 6_800_000, "jlpt": None, "jp_degree": False, "quals": 0},
    "Chen (33, master's, N1, JP grad)": {"age": 33, "degree": "master", "years": 7, "salary": 8_200_000, "jlpt": "N1", "jp_degree": True, "quals": 2},
    "Dana (41, PhD)": {"age": 41, "degree": "doctor", "years": 15, "salary": 7_500_000, "jlpt": None, "jp_degree": False, "quals": 0},
    "Eli (25, bachelor's)": {"age": 25, "degree": "bachelor", "years": 2, "salary": 2_900_000, "jlpt": "N1", "jp_degree": True, "quals": 1},
}

def salary_points(salary, age):
    raise NotImplementedError
@@expected
Aiko (28, master's, N2): 70 points (academic 20, experience 5, salary 15, age 15, japanese 10, qualifications 5) -> HSP + PR after 3 years
Ben (36, bachelor's): 55 points (academic 10, experience 20, salary 20, age 5) -> short by 15
Chen (33, master's, N1, JP grad): 110 points (academic 20, experience 15, salary 30, age 10, japanese 15, JP degree 10, qualifications 10) -> HSP + PR after 1 year
Dana (41, PhD): 50 points (academic 30, experience 20) -> short by 20
Eli (25, bachelor's): salary below ¥3M, this category is not available
@@requires
def salary_points
def total_points
3_000_000
@@hint
Salary table as `(threshold, points, max_age)` rows from ¥10M down: `(10_000_000, 40, None)`, …, `(7_000_000, 25, 39)`, `(6_000_000, 20, 39)`, `(5_000_000, 15, 34)`, `(4_000_000, 10, 29)`. Take the first row whose threshold is met **and** whose age limit allows it; if a higher row is blocked by age, lower rows have stricter limits, so the result is 0. Experience: 20/15/10/5 for 10/7/5/3 years.
@@solution
candidates = {
    "Aiko (28, master's, N2)": {"age": 28, "degree": "master", "years": 4, "salary": 5_200_000, "jlpt": "N2", "jp_degree": False, "quals": 1},
    "Ben (36, bachelor's)": {"age": 36, "degree": "bachelor", "years": 12, "salary": 6_800_000, "jlpt": None, "jp_degree": False, "quals": 0},
    "Chen (33, master's, N1, JP grad)": {"age": 33, "degree": "master", "years": 7, "salary": 8_200_000, "jlpt": "N1", "jp_degree": True, "quals": 2},
    "Dana (41, PhD)": {"age": 41, "degree": "doctor", "years": 15, "salary": 7_500_000, "jlpt": None, "jp_degree": False, "quals": 0},
    "Eli (25, bachelor's)": {"age": 25, "degree": "bachelor", "years": 2, "salary": 2_900_000, "jlpt": "N1", "jp_degree": True, "quals": 1},
}
SALARY = [(10_000_000, 40, None), (9_000_000, 35, None), (8_000_000, 30, None), (7_000_000, 25, 39),
          (6_000_000, 20, 39), (5_000_000, 15, 34), (4_000_000, 10, 29)]

def salary_points(salary, age):
    for threshold, points, max_age in SALARY:
        if salary >= threshold and (max_age is None or age <= max_age):
            return points
    return 0

def total_points(c):
    parts = {
        "academic": {"doctor": 30, "master": 20, "bachelor": 10}.get(c["degree"], 0),
        "experience": next((p for y, p in ((10, 20), (7, 15), (5, 10), (3, 5)) if c["years"] >= y), 0),
        "salary": salary_points(c["salary"], c["age"]),
        "age": 15 if c["age"] < 30 else 10 if c["age"] < 35 else 5 if c["age"] < 40 else 0,
        "japanese": {"N1": 15, "N2": 10}.get(c["jlpt"], 0),
        "JP degree": 10 if c["jp_degree"] else 0,
        "qualifications": min(c["quals"], 2) * 5,
    }
    return sum(parts.values()), parts

for name, c in candidates.items():
    if c["salary"] < 3_000_000:
        print(f"{name}: salary below ¥3M, this category is not available")
        continue
    total, parts = total_points(c)
    outcome = "HSP + PR after 1 year" if total >= 80 else "HSP + PR after 3 years" if total >= 70 else f"short by {70 - total}"
    detail = ", ".join(f"{k} {v}" for k, v in parts.items() if v)
    print(f"{name}: {total} points ({detail}) -> {outcome}")

@@exercise Plan the path to 80 points
For a candidate below 80 points, evaluate realistic improvements (pass JLPT N2 or N1, a raise to the next salary band, a qualification, another year of experience) and their timing. Ageing matters: by the time an improvement is achieved, age points (and salary age limits) may change. Print the points for each plan at its completion date and the fastest plan to 80.
@@lang python
@@starter
base = {"age": 29, "degree": "master", "years": 4, "salary": 6_300_000, "jlpt": None, "jp_degree": False, "quals": 0}
plans = {
    "pass JLPT N2 (12 months)": (12, {"jlpt": "N2"}),
    "pass JLPT N1 (24 months)": (24, {"jlpt": "N1"}),
    "IPA exam + N2 (12 months)": (12, {"quals": 1, "jlpt": "N2"}),
    "raise to ¥8M (18 months)": (18, {"salary": 8_000_000}),
    "N2 + raise to ¥7M (12 months)": (12, {"jlpt": "N2", "salary": 7_000_000}),
    "N1 + raise to ¥8M (24 months)": (24, {"jlpt": "N1", "salary": 8_000_000}),
}

def points(c):
    raise NotImplementedError
@@expected
today (age 29): 60 points
pass JLPT N2 (12 months): age 30, 70 points
pass JLPT N1 (24 months): age 31, 75 points
IPA exam + N2 (12 months): age 30, 75 points
raise to ¥8M (18 months): age 30, 70 points
N2 + raise to ¥7M (12 months): age 30, 75 points
N1 + raise to ¥8M (24 months): age 31, 85 points -> 80 reached
fastest to 80: N1 + raise to ¥8M (24 months)
@@requires
def points
months // 12
80
@@hint
Reuse the points rules from the previous exercise. For each plan, copy `base`, apply the changes, add `months // 12` to both age and years of experience, and compute the total. Sort the plans that reach 80 by months.
@@solution
base = {"age": 29, "degree": "master", "years": 4, "salary": 6_300_000, "jlpt": None, "jp_degree": False, "quals": 0}
plans = {
    "pass JLPT N2 (12 months)": (12, {"jlpt": "N2"}),
    "pass JLPT N1 (24 months)": (24, {"jlpt": "N1"}),
    "IPA exam + N2 (12 months)": (12, {"quals": 1, "jlpt": "N2"}),
    "raise to ¥8M (18 months)": (18, {"salary": 8_000_000}),
    "N2 + raise to ¥7M (12 months)": (12, {"jlpt": "N2", "salary": 7_000_000}),
    "N1 + raise to ¥8M (24 months)": (24, {"jlpt": "N1", "salary": 8_000_000}),
}
SALARY = [(10_000_000, 40, None), (9_000_000, 35, None), (8_000_000, 30, None), (7_000_000, 25, 39),
          (6_000_000, 20, 39), (5_000_000, 15, 34), (4_000_000, 10, 29)]

def points(c):
    salary = next((p for t, p, a in SALARY if c["salary"] >= t and (a is None or c["age"] <= a)), 0)
    return ({"doctor": 30, "master": 20, "bachelor": 10}.get(c["degree"], 0)
            + next((p for y, p in ((10, 20), (7, 15), (5, 10), (3, 5)) if c["years"] >= y), 0)
            + salary
            + (15 if c["age"] < 30 else 10 if c["age"] < 35 else 5 if c["age"] < 40 else 0)
            + {"N1": 15, "N2": 10}.get(c["jlpt"], 0)
            + (10 if c["jp_degree"] else 0)
            + min(c["quals"], 2) * 5)

print(f"today (age {base['age']}): {points(base)} points")
reached = []
for name, (months, changes) in plans.items():
    c = dict(base, **changes)
    c["age"] += months // 12
    c["years"] += months // 12
    total = points(c)
    print(f"{name}: age {c['age']}, {total} points{' -> 80 reached' if total >= 80 else ''}")
    if total >= 80:
        reached.append((months, name))
print("fastest to 80:", min(reached)[1] if reached else "none of these plans")
