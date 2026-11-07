@@lesson jc9-application-kit Japan Application Kit
@@text
Everything for the job hunt in one folder: **templates** for the 履歴書 and 職務経歴書, a **tracker** that moves your applications through the 選考 stages and reminds you to follow up, and an **offer checker** that answers the question this whole app is about: *does this job really pay well above minimum wage?*

```text
A社 (自社開発, web): ¥4,480,000/year, ¥2,333 per hour worked (1.9x minimum)
C社 (社内SE): ¥4,350,000/year, ¥2,071 per hour worked (1.69x minimum)
   ! includes 20 hours of fixed overtime
B社 (SES): ¥3,360,000/year, ¥1,314 per hour worked (1.07x minimum)
   ! less than 20% above minimum wage
   ! includes 45 hours of fixed overtime
   ! no bonus
```

B社's ¥280,000 a month looks almost as good as the others, until you count the **45 hours of "included" overtime** (みなし残業) and the missing bonus. Per hour actually worked, it pays barely above Tokyo's minimum wage. That's the trap the checker exists to catch.

How the numbers work:

- **年収** = 月給 × (12 + bonus months);
- **hourly** = 年収 ÷ (12 × (scheduled hours + fixed overtime hours)), rounded down;
- warnings for below minimum, less than 20% above it, fixed overtime, and no bonus.
@@project application_kit Japan Application Kit
Write the functions in `jobkit.py` until `python -m unittest` passes. Run `python main.py offers` on the examples, then put in real offers (update `minimum_wage` for your prefecture). Track real applications with `python main.py add ...`, and fill in the two templates with your own history.
@@file jobkit.py
"""Japan job-hunting kit: track applications through the 選考 stages and compare offers honestly."""
from datetime import date, timedelta

STAGES = ["応募", "書類選考", "一次面接", "二次面接", "最終面接", "内定", "入社"]
CLOSED = ["不合格", "辞退"]          # rejected / you withdrew
FOLLOW_UP_DAYS = 7                   # chase the company (or your agent) if nothing happens in a week


def new_application(company, role, today, source=""):
    """A fresh application in stage 応募, with a follow-up date a week out."""
    return {"company": company, "role": role, "source": source, "stage": "応募",
            "history": [(today.isoformat(), "応募")], "follow_up": (today + timedelta(days=FOLLOW_UP_DAYS)).isoformat()}


def advance(app, today, skip_to=None):
    """Move to the next stage (or to `skip_to`, which must be later in STAGES). Records the history,
    and sets the next follow-up a week from today; 入社 (joined) has no follow-up (None).
    Raise ValueError if the application is closed, already at 入社, or skip_to isn't a later stage."""
    raise NotImplementedError


def close(app, outcome, today):
    """End an application as 不合格 or 辞退. No more follow-ups."""
    raise NotImplementedError


def follow_ups(apps, today):
    """Open applications whose follow-up date is today or earlier, oldest first (then by company)."""
    raise NotImplementedError


def funnel(apps):
    """How many applications REACHED each stage (from their history), as {stage: count} in STAGES order."""
    raise NotImplementedError


def annual_gross(monthly, bonus_months=0.0):
    """年収 (yen): 12 × 月給 + bonus (賞与), given as months of salary (e.g. 2.0)."""
    raise NotImplementedError


def effective_hourly(monthly, scheduled_hours, fixed_overtime_hours=0, bonus_months=0.0):
    """What one hour of work really pays, in yen, rounded down to a whole yen.

    みなし残業 / 固定残業代: a salary that "includes" N hours of overtime per month means those
    hours are already paid for, so they count as hours worked. Bonuses count as income.
        annual_gross / (12 × (scheduled_hours + fixed_overtime_hours))
    """
    raise NotImplementedError


def compare_offer(offer, minimum_wage):
    """offer: {"company", "monthly", "scheduled_hours", "fixed_overtime_hours", "bonus_months"}.
    Returns {"company", "annual", "hourly", "vs_minimum": hourly / minimum_wage rounded to 2 decimals,
    "warnings": [...]} with these warnings, in this order:
        "below minimum wage"                      if hourly < minimum_wage
        "less than 20% above minimum wage"       if minimum_wage <= hourly < 1.2 × minimum_wage
        "includes N hours of fixed overtime"     if fixed_overtime_hours > 0 (N as given)
        "no bonus"                                if bonus_months == 0
    """
    raise NotImplementedError
@@file test_jobkit.py
import unittest
from datetime import date

from jobkit import (advance, annual_gross, close, compare_offer, effective_hourly, follow_ups, funnel,
                    new_application)

D1 = date(2026, 10, 1)


class PipelineTests(unittest.TestCase):
    def test_new(self):
        a = new_application("A社", "インフラエンジニア", D1, "Doda")
        self.assertEqual(a["stage"], "応募")
        self.assertEqual(a["history"], [("2026-10-01", "応募")])
        self.assertEqual(a["follow_up"], "2026-10-08")
        self.assertEqual(a["source"], "Doda")

    def test_advance(self):
        a = new_application("A社", "ヘルプデスク", D1)
        advance(a, date(2026, 10, 5))
        self.assertEqual(a["stage"], "書類選考")
        self.assertEqual(a["follow_up"], "2026-10-12")
        advance(a, date(2026, 10, 10), skip_to="最終面接")
        self.assertEqual(a["stage"], "最終面接")
        self.assertEqual([s for _, s in a["history"]], ["応募", "書類選考", "最終面接"])

    def test_joined_has_no_follow_up(self):
        a = new_application("A社", "SRE", D1)
        advance(a, D1, skip_to="内定")
        advance(a, D1)
        self.assertEqual((a["stage"], a["follow_up"]), ("入社", None))
        with self.assertRaises(ValueError):
            advance(a, D1)

    def test_bad_advances(self):
        a = new_application("A社", "SRE", D1)
        advance(a, D1)
        for bad in ["応募", "書類選考", "内々定"]:
            with self.subTest(bad=bad), self.assertRaises(ValueError):
                advance(a, D1, skip_to=bad)

    def test_close(self):
        a = new_application("B社", "運用監視", D1)
        close(a, "不合格", date(2026, 10, 9))
        self.assertEqual((a["stage"], a["follow_up"]), ("不合格", None))
        with self.assertRaises(ValueError):
            advance(a, D1)
        with self.assertRaises(ValueError):
            close(new_application("C社", "x", D1), "lost", D1)

    def test_follow_ups(self):
        a = new_application("C社", "x", date(2026, 10, 3))   # due 10-10
        b = new_application("A社", "x", date(2026, 10, 1))   # due 10-08
        c = new_application("B社", "x", date(2026, 10, 1))   # due 10-08
        d = new_application("D社", "x", date(2026, 10, 1))
        close(d, "辞退", date(2026, 10, 2))
        apps = [a, b, c, d]
        self.assertEqual([x["company"] for x in follow_ups(apps, date(2026, 10, 7))], [])
        self.assertEqual([x["company"] for x in follow_ups(apps, date(2026, 10, 8))], ["A社", "B社"])
        self.assertEqual([x["company"] for x in follow_ups(apps, date(2026, 10, 20))], ["A社", "B社", "C社"])

    def test_funnel(self):
        a = new_application("A社", "x", D1)
        advance(a, D1)
        advance(a, D1)                       # 一次面接
        b = new_application("B社", "x", D1)
        advance(b, D1)
        close(b, "不合格", D1)
        c = new_application("C社", "x", D1)
        advance(c, D1, skip_to="内定")
        f = funnel([a, b, c])
        self.assertEqual(list(f), ["応募", "書類選考", "一次面接", "二次面接", "最終面接", "内定", "入社"])
        self.assertEqual(f["応募"], 3)
        self.assertEqual(f["書類選考"], 2)
        self.assertEqual(f["一次面接"], 1)
        self.assertEqual(f["内定"], 1)
        self.assertEqual(f["入社"], 0)


class OfferTests(unittest.TestCase):
    def test_annual(self):
        self.assertEqual(annual_gross(300_000), 3_600_000)
        self.assertEqual(annual_gross(300_000, 2.0), 4_200_000)
        self.assertEqual(annual_gross(250_000, 4.5), 4_125_000)

    def test_hourly(self):
        self.assertEqual(effective_hourly(300_000, 160), 1875)
        self.assertEqual(effective_hourly(300_000, 160, 45), 1463)          # 300000*12 / (12*205)
        self.assertEqual(effective_hourly(300_000, 160, 0, 2.0), 2187)      # 4.2M / 1920 = 2187.5
        with self.assertRaises(ValueError):
            effective_hourly(300_000, 0)

    def test_compare_good_offer(self):
        r = compare_offer({"company": "A社", "monthly": 320_000, "scheduled_hours": 160,
                           "fixed_overtime_hours": 0, "bonus_months": 2.0}, 1226)
        self.assertEqual(r, {"company": "A社", "annual": 4_480_000, "hourly": 2333, "vs_minimum": 1.9, "warnings": []})

    def test_compare_fixed_overtime_trap(self):
        # looks like more money per month, but 45 h of overtime are "included" and there is no bonus
        r = compare_offer({"company": "B社", "monthly": 280_000, "scheduled_hours": 168,
                           "fixed_overtime_hours": 45, "bonus_months": 0}, 1226)
        self.assertEqual(r["hourly"], 1314)
        self.assertEqual(r["vs_minimum"], 1.07)
        self.assertEqual(r["warnings"], ["less than 20% above minimum wage", "includes 45 hours of fixed overtime",
                                         "no bonus"])

    def test_below_minimum(self):
        r = compare_offer({"company": "C社", "monthly": 220_000, "scheduled_hours": 168,
                           "fixed_overtime_hours": 30}, 1226)
        self.assertEqual(r["hourly"], 1111)      # 220000 / 198
        self.assertEqual(r["warnings"][0], "below minimum wage")
        self.assertEqual(r["warnings"][-1], "no bonus", "missing bonus_months means 0")


if __name__ == "__main__":
    unittest.main()
@@file main.py
"""Job hunt tracker for Japan.

    python main.py                       today's follow-ups and the funnel
    python main.py add "A社" "インフラエンジニア" Doda
    python main.py next "A社"            move to the next stage
    python main.py next "A社" 最終面接   jump to a stage
    python main.py close "A社" 辞退      (or 不合格)
    python main.py offers                compare the offers in offers.json

Data is saved in applications.json next to this file.
"""
import json
import os
import sys
from datetime import date

import jobkit as jk

HERE = os.path.dirname(os.path.abspath(__file__))
APPS = os.path.join(HERE, "applications.json")
OFFERS = os.path.join(HERE, "offers.json")


def load(path, default):
    if not os.path.exists(path):
        return default
    with open(path, encoding="utf-8") as f:
        return json.load(f)


def find(apps, company):
    for a in apps:
        if a["company"] == company:
            return a
    raise SystemExit(f"no application for {company!r}")


def main(argv):
    sys.stdout.reconfigure(errors="replace")
    today = date.today()
    apps = load(APPS, [])
    cmd = argv[0] if argv else "status"

    if cmd == "add":
        apps.append(jk.new_application(argv[1], argv[2], today, argv[3] if len(argv) > 3 else ""))
    elif cmd == "next":
        jk.advance(find(apps, argv[1]), today, argv[2] if len(argv) > 2 else None)
    elif cmd == "close":
        jk.close(find(apps, argv[1]), argv[2], today)
    elif cmd == "offers":
        data = load(OFFERS, {"minimum_wage": 0, "offers": []})
        minimum = data["minimum_wage"]
        print(f"Minimum wage used: ¥{minimum:,}/hour (check the current figure for the prefecture!)\n")
        for r in sorted((jk.compare_offer(o, minimum) for o in data["offers"]), key=lambda r: -r["hourly"]):
            print(f"{r['company']}: ¥{r['annual']:,}/year, ¥{r['hourly']:,} per hour worked "
                  f"({r['vs_minimum']}x minimum)")
            for w in r["warnings"]:
                print(f"   ! {w}")
        return 0
    elif cmd != "status":
        print(__doc__)
        return 1

    if cmd != "status":
        with open(APPS, "w", encoding="utf-8") as f:
            json.dump(apps, f, ensure_ascii=False, indent=1)

    print(f"{len(apps)} application(s)")
    for a in apps:
        print(f"  {a['company']:12} {a['role']:20} {a['stage']}")
    due = jk.follow_ups(apps, today)
    if due:
        print("\nFollow up today (メールかエージェントに確認):")
        for a in due:
            print(f"  {a['company']} ({a['stage']}, since {a['history'][-1][0]})")
    print("\nFunnel:")
    for stage, n in jk.funnel(apps).items():
        print(f"  {stage:6} {'#' * n} {n}")
    return 0


if __name__ == "__main__":
    sys.exit(main(sys.argv[1:]))
@@file offers.json
{
  "_note": "Example offers. minimum_wage: Tokyo's from October 2025 (¥1,226/hour). Update it: 最低賃金 changes every October and differs by prefecture.",
  "minimum_wage": 1226,
  "offers": [
    {"company": "A社 (自社開発, web)", "monthly": 320000, "scheduled_hours": 160, "fixed_overtime_hours": 0, "bonus_months": 2.0},
    {"company": "B社 (SES)", "monthly": 280000, "scheduled_hours": 168, "fixed_overtime_hours": 45, "bonus_months": 0},
    {"company": "C社 (社内SE)", "monthly": 290000, "scheduled_hours": 155, "fixed_overtime_hours": 20, "bonus_months": 3.0}
  ]
}
@@file rirekisho.md
# 履歴書 (rirekisho) checklist

Most companies accept the standard **JIS-style** form (download a template from Doda, マイナビ or
the 厚生労働省 sample), typed, as a PDF. Handwriting is rarely required now.

| 欄 (field) | What to write | Tips |
|---|---|---|
| 日付 | the date you send it | not the date you wrote it |
| 氏名 / ふりがな | your name; katakana reading for foreign names | e.g. スミス ジョン |
| 写真 | 3 × 4 cm, recent, suit, plain background | photo booths in stations print the right size |
| 生年月日 / 住所 / 電話 / メール | as they are | a professional email address |
| 学歴 (education) | from high school or university, 年 月 入学 / 卒業 | Japanese era (令和) or western years: be consistent |
| 職歴 (work history) | 年 月 company 入社 / 退社; last line 「現在に至る」, then 「以上」 right-aligned | |
| 免許・資格 | licences and qualifications, oldest first, with dates | 2026年 6月 基本情報技術者試験 合格 |
| 志望動機 | why this company, 3-5 lines | specific; link your skills to their business |
| 本人希望記入欄 | 「貴社の規定に従います。」 unless you have a real condition | |

Also note 在留資格 and its expiry, and your Japanese level (JLPT), in the 資格 or notes section.
@@file shokumu_keirekisho.md
# 職務経歴書 (career history) template

The 職務経歴書 matters more than the 履歴書 for IT roles: it's where your skills are judged.
A4, 1-3 pages, free format. Write it in Japanese for Japanese companies; an English CV too for 外資系.

```
職務経歴書
2026年11月1日現在
氏名 〇〇 〇〇

■ 職務要約 (summary, 3-5 lines)
ITサポートおよびインフラ運用に〇年間従事してまいりました。
Windows/Microsoft 365 環境の運用、ネットワーク障害の一次対応、PowerShell による業務自動化を得意としております。

■ 職務経歴
〇〇株式会社 (20XX年X月～現在)
事業内容: 〇〇 / 従業員数: 〇〇名 / 雇用形態: 正社員

期間          | 業務内容                                         | 環境・ツール
20XX年X月～   | 社内ヘルプデスク (約300名を担当)                  | Windows 11, Microsoft 365,
              | ・PCキッティング、アカウント管理                  | Entra ID, Intune,
              | ・ネットワーク障害の一次切り分け                  | Cisco スイッチ
              | 【実績】PowerShell でアカウント作成を自動化し、   |
              |  作業時間を月20時間削減                          |
              | 【役割】メンバー (チーム4名)                     |

■ 活かせる経験・知識・技術 (skills)
・OS: Windows Server 2022 (2年), Linux (Ubuntu, 1年)
・ネットワーク: VLAN, TCP/IP, DNS/DHCP の設計・運用
・クラウド: AWS (EC2, S3, IAM), Azure (Entra ID)
・言語: PowerShell (業務利用), Python (自動化スクリプト), SQL

■ 資格
・CompTIA A+ (20XX年X月)
・基本情報技術者試験 (20XX年X月)
・日本語能力試験 N2 (20XX年X月)

■ 自己PR
(One or two concrete stories: the problem, what you did, the result with a number.)

以上
```

Tips
- Numbers everywhere: users supported, tickets per month, hours saved, uptime.
- 【実績】(results) and 【役割】(your role) under each job.
- Link your GitHub with the projects from this app (status board, SLA clock, audit script...).
@@text
> Tips: `STAGES.index(stage)` gives a stage's position, so "later" means a bigger index. `today + timedelta(days=7)` then `.isoformat()` for the follow-up date. In `funnel`, collect the set of stages in each application's history, then count. Use `//` for the rounded-down hourly figure.
@@project application_kit_finished Japan Application Kit: the finished version
The complete kit. Next steps: add notes and interview dates per application, export the tracker to CSV, add commuting time to the offer comparison, or estimate take-home pay (roughly 75–80% of gross, but look up the current social insurance rates).
@@file jobkit.py
"""Japan job-hunting kit: track applications through the 選考 stages and compare offers honestly."""
from datetime import date, timedelta

STAGES = ["応募", "書類選考", "一次面接", "二次面接", "最終面接", "内定", "入社"]
CLOSED = ["不合格", "辞退"]          # rejected / you withdrew
FOLLOW_UP_DAYS = 7                   # chase the company (or your agent) if nothing happens in a week


def new_application(company, role, today, source=""):
    """A fresh application in stage 応募, with a follow-up date a week out."""
    return {"company": company, "role": role, "source": source, "stage": "応募",
            "history": [(today.isoformat(), "応募")], "follow_up": (today + timedelta(days=FOLLOW_UP_DAYS)).isoformat()}


def advance(app, today, skip_to=None):
    """Move to the next stage (or to `skip_to`, which must be later in STAGES). Records the history,
    and sets the next follow-up a week from today; 入社 (joined) has no follow-up (None).
    Raise ValueError if the application is closed, already at 入社, or skip_to isn't a later stage."""
    if app["stage"] in CLOSED or app["stage"] == STAGES[-1]:
        raise ValueError(f"{app['company']}: can't advance from {app['stage']}")
    now = STAGES.index(app["stage"])
    target = now + 1 if skip_to is None else (STAGES.index(skip_to) if skip_to in STAGES else -1)
    if target <= now:
        raise ValueError(f"{skip_to!r} is not a later stage than {app['stage']}")
    app["stage"] = STAGES[target]
    app["history"].append((today.isoformat(), app["stage"]))
    app["follow_up"] = None if app["stage"] == STAGES[-1] else (today + timedelta(days=FOLLOW_UP_DAYS)).isoformat()
    return app


def close(app, outcome, today):
    """End an application as 不合格 or 辞退. No more follow-ups."""
    if outcome not in CLOSED:
        raise ValueError(f"outcome must be one of {CLOSED}")
    app["stage"] = outcome
    app["history"].append((today.isoformat(), outcome))
    app["follow_up"] = None
    return app


def follow_ups(apps, today):
    """Open applications whose follow-up date is today or earlier, oldest first (then by company)."""
    due = [a for a in apps if a["follow_up"] and date.fromisoformat(a["follow_up"]) <= today]
    return sorted(due, key=lambda a: (a["follow_up"], a["company"]))


def funnel(apps):
    """How many applications REACHED each stage (from their history), as {stage: count} in STAGES order."""
    reached = {s: 0 for s in STAGES}
    for a in apps:
        seen = {stage for _, stage in a["history"]}
        for s in STAGES:
            if s in seen:
                reached[s] += 1
    return reached


def annual_gross(monthly, bonus_months=0.0):
    """年収 (yen): 12 × 月給 + bonus (賞与), given as months of salary (e.g. 2.0)."""
    return round(monthly * (12 + bonus_months))


def effective_hourly(monthly, scheduled_hours, fixed_overtime_hours=0, bonus_months=0.0):
    """What one hour of work really pays, in yen, rounded down to a whole yen.

    みなし残業 / 固定残業代: a salary that "includes" N hours of overtime per month means those
    hours are already paid for, so they count as hours worked. Bonuses count as income.
        annual_gross / (12 × (scheduled_hours + fixed_overtime_hours))
    """
    hours = 12 * (scheduled_hours + fixed_overtime_hours)
    if hours <= 0:
        raise ValueError("hours must be positive")
    return int(annual_gross(monthly, bonus_months) // hours)


def compare_offer(offer, minimum_wage):
    """offer: {"company", "monthly", "scheduled_hours", "fixed_overtime_hours", "bonus_months"}.
    Returns {"company", "annual", "hourly", "vs_minimum": hourly / minimum_wage rounded to 2 decimals,
    "warnings": [...]} with these warnings, in this order:
        "below minimum wage"                      if hourly < minimum_wage
        "less than 20% above minimum wage"       if minimum_wage <= hourly < 1.2 × minimum_wage
        "includes N hours of fixed overtime"     if fixed_overtime_hours > 0 (N as given)
        "no bonus"                                if bonus_months == 0
    """
    fixed = offer.get("fixed_overtime_hours", 0)
    bonus = offer.get("bonus_months", 0.0)
    hourly = effective_hourly(offer["monthly"], offer["scheduled_hours"], fixed, bonus)
    warnings = []
    if hourly < minimum_wage:
        warnings.append("below minimum wage")
    elif hourly < 1.2 * minimum_wage:
        warnings.append("less than 20% above minimum wage")
    if fixed > 0:
        warnings.append(f"includes {fixed} hours of fixed overtime")
    if bonus == 0:
        warnings.append("no bonus")
    return {"company": offer["company"], "annual": annual_gross(offer["monthly"], bonus), "hourly": hourly,
            "vs_minimum": round(hourly / minimum_wage, 2), "warnings": warnings}
@@file test_jobkit.py
import unittest
from datetime import date

from jobkit import (advance, annual_gross, close, compare_offer, effective_hourly, follow_ups, funnel,
                    new_application)

D1 = date(2026, 10, 1)


class PipelineTests(unittest.TestCase):
    def test_new(self):
        a = new_application("A社", "インフラエンジニア", D1, "Doda")
        self.assertEqual(a["stage"], "応募")
        self.assertEqual(a["history"], [("2026-10-01", "応募")])
        self.assertEqual(a["follow_up"], "2026-10-08")
        self.assertEqual(a["source"], "Doda")

    def test_advance(self):
        a = new_application("A社", "ヘルプデスク", D1)
        advance(a, date(2026, 10, 5))
        self.assertEqual(a["stage"], "書類選考")
        self.assertEqual(a["follow_up"], "2026-10-12")
        advance(a, date(2026, 10, 10), skip_to="最終面接")
        self.assertEqual(a["stage"], "最終面接")
        self.assertEqual([s for _, s in a["history"]], ["応募", "書類選考", "最終面接"])

    def test_joined_has_no_follow_up(self):
        a = new_application("A社", "SRE", D1)
        advance(a, D1, skip_to="内定")
        advance(a, D1)
        self.assertEqual((a["stage"], a["follow_up"]), ("入社", None))
        with self.assertRaises(ValueError):
            advance(a, D1)

    def test_bad_advances(self):
        a = new_application("A社", "SRE", D1)
        advance(a, D1)
        for bad in ["応募", "書類選考", "内々定"]:
            with self.subTest(bad=bad), self.assertRaises(ValueError):
                advance(a, D1, skip_to=bad)

    def test_close(self):
        a = new_application("B社", "運用監視", D1)
        close(a, "不合格", date(2026, 10, 9))
        self.assertEqual((a["stage"], a["follow_up"]), ("不合格", None))
        with self.assertRaises(ValueError):
            advance(a, D1)
        with self.assertRaises(ValueError):
            close(new_application("C社", "x", D1), "lost", D1)

    def test_follow_ups(self):
        a = new_application("C社", "x", date(2026, 10, 3))   # due 10-10
        b = new_application("A社", "x", date(2026, 10, 1))   # due 10-08
        c = new_application("B社", "x", date(2026, 10, 1))   # due 10-08
        d = new_application("D社", "x", date(2026, 10, 1))
        close(d, "辞退", date(2026, 10, 2))
        apps = [a, b, c, d]
        self.assertEqual([x["company"] for x in follow_ups(apps, date(2026, 10, 7))], [])
        self.assertEqual([x["company"] for x in follow_ups(apps, date(2026, 10, 8))], ["A社", "B社"])
        self.assertEqual([x["company"] for x in follow_ups(apps, date(2026, 10, 20))], ["A社", "B社", "C社"])

    def test_funnel(self):
        a = new_application("A社", "x", D1)
        advance(a, D1)
        advance(a, D1)                       # 一次面接
        b = new_application("B社", "x", D1)
        advance(b, D1)
        close(b, "不合格", D1)
        c = new_application("C社", "x", D1)
        advance(c, D1, skip_to="内定")
        f = funnel([a, b, c])
        self.assertEqual(list(f), ["応募", "書類選考", "一次面接", "二次面接", "最終面接", "内定", "入社"])
        self.assertEqual(f["応募"], 3)
        self.assertEqual(f["書類選考"], 2)
        self.assertEqual(f["一次面接"], 1)
        self.assertEqual(f["内定"], 1)
        self.assertEqual(f["入社"], 0)


class OfferTests(unittest.TestCase):
    def test_annual(self):
        self.assertEqual(annual_gross(300_000), 3_600_000)
        self.assertEqual(annual_gross(300_000, 2.0), 4_200_000)
        self.assertEqual(annual_gross(250_000, 4.5), 4_125_000)

    def test_hourly(self):
        self.assertEqual(effective_hourly(300_000, 160), 1875)
        self.assertEqual(effective_hourly(300_000, 160, 45), 1463)          # 300000*12 / (12*205)
        self.assertEqual(effective_hourly(300_000, 160, 0, 2.0), 2187)      # 4.2M / 1920 = 2187.5
        with self.assertRaises(ValueError):
            effective_hourly(300_000, 0)

    def test_compare_good_offer(self):
        r = compare_offer({"company": "A社", "monthly": 320_000, "scheduled_hours": 160,
                           "fixed_overtime_hours": 0, "bonus_months": 2.0}, 1226)
        self.assertEqual(r, {"company": "A社", "annual": 4_480_000, "hourly": 2333, "vs_minimum": 1.9, "warnings": []})

    def test_compare_fixed_overtime_trap(self):
        # looks like more money per month, but 45 h of overtime are "included" and there is no bonus
        r = compare_offer({"company": "B社", "monthly": 280_000, "scheduled_hours": 168,
                           "fixed_overtime_hours": 45, "bonus_months": 0}, 1226)
        self.assertEqual(r["hourly"], 1314)
        self.assertEqual(r["vs_minimum"], 1.07)
        self.assertEqual(r["warnings"], ["less than 20% above minimum wage", "includes 45 hours of fixed overtime",
                                         "no bonus"])

    def test_below_minimum(self):
        r = compare_offer({"company": "C社", "monthly": 220_000, "scheduled_hours": 168,
                           "fixed_overtime_hours": 30}, 1226)
        self.assertEqual(r["hourly"], 1111)      # 220000 / 198
        self.assertEqual(r["warnings"][0], "below minimum wage")
        self.assertEqual(r["warnings"][-1], "no bonus", "missing bonus_months means 0")


if __name__ == "__main__":
    unittest.main()
@@file main.py
"""Job hunt tracker for Japan.

    python main.py                       today's follow-ups and the funnel
    python main.py add "A社" "インフラエンジニア" Doda
    python main.py next "A社"            move to the next stage
    python main.py next "A社" 最終面接   jump to a stage
    python main.py close "A社" 辞退      (or 不合格)
    python main.py offers                compare the offers in offers.json

Data is saved in applications.json next to this file.
"""
import json
import os
import sys
from datetime import date

import jobkit as jk

HERE = os.path.dirname(os.path.abspath(__file__))
APPS = os.path.join(HERE, "applications.json")
OFFERS = os.path.join(HERE, "offers.json")


def load(path, default):
    if not os.path.exists(path):
        return default
    with open(path, encoding="utf-8") as f:
        return json.load(f)


def find(apps, company):
    for a in apps:
        if a["company"] == company:
            return a
    raise SystemExit(f"no application for {company!r}")


def main(argv):
    sys.stdout.reconfigure(errors="replace")
    today = date.today()
    apps = load(APPS, [])
    cmd = argv[0] if argv else "status"

    if cmd == "add":
        apps.append(jk.new_application(argv[1], argv[2], today, argv[3] if len(argv) > 3 else ""))
    elif cmd == "next":
        jk.advance(find(apps, argv[1]), today, argv[2] if len(argv) > 2 else None)
    elif cmd == "close":
        jk.close(find(apps, argv[1]), argv[2], today)
    elif cmd == "offers":
        data = load(OFFERS, {"minimum_wage": 0, "offers": []})
        minimum = data["minimum_wage"]
        print(f"Minimum wage used: ¥{minimum:,}/hour (check the current figure for the prefecture!)\n")
        for r in sorted((jk.compare_offer(o, minimum) for o in data["offers"]), key=lambda r: -r["hourly"]):
            print(f"{r['company']}: ¥{r['annual']:,}/year, ¥{r['hourly']:,} per hour worked "
                  f"({r['vs_minimum']}x minimum)")
            for w in r["warnings"]:
                print(f"   ! {w}")
        return 0
    elif cmd != "status":
        print(__doc__)
        return 1

    if cmd != "status":
        with open(APPS, "w", encoding="utf-8") as f:
            json.dump(apps, f, ensure_ascii=False, indent=1)

    print(f"{len(apps)} application(s)")
    for a in apps:
        print(f"  {a['company']:12} {a['role']:20} {a['stage']}")
    due = jk.follow_ups(apps, today)
    if due:
        print("\nFollow up today (メールかエージェントに確認):")
        for a in due:
            print(f"  {a['company']} ({a['stage']}, since {a['history'][-1][0]})")
    print("\nFunnel:")
    for stage, n in jk.funnel(apps).items():
        print(f"  {stage:6} {'#' * n} {n}")
    return 0


if __name__ == "__main__":
    sys.exit(main(sys.argv[1:]))
@@file offers.json
{
  "_note": "Example offers. minimum_wage: Tokyo's from October 2025 (¥1,226/hour). Update it: 最低賃金 changes every October and differs by prefecture.",
  "minimum_wage": 1226,
  "offers": [
    {"company": "A社 (自社開発, web)", "monthly": 320000, "scheduled_hours": 160, "fixed_overtime_hours": 0, "bonus_months": 2.0},
    {"company": "B社 (SES)", "monthly": 280000, "scheduled_hours": 168, "fixed_overtime_hours": 45, "bonus_months": 0},
    {"company": "C社 (社内SE)", "monthly": 290000, "scheduled_hours": 155, "fixed_overtime_hours": 20, "bonus_months": 3.0}
  ]
}
@@file rirekisho.md
# 履歴書 (rirekisho) checklist

Most companies accept the standard **JIS-style** form (download a template from Doda, マイナビ or
the 厚生労働省 sample), typed, as a PDF. Handwriting is rarely required now.

| 欄 (field) | What to write | Tips |
|---|---|---|
| 日付 | the date you send it | not the date you wrote it |
| 氏名 / ふりがな | your name; katakana reading for foreign names | e.g. スミス ジョン |
| 写真 | 3 × 4 cm, recent, suit, plain background | photo booths in stations print the right size |
| 生年月日 / 住所 / 電話 / メール | as they are | a professional email address |
| 学歴 (education) | from high school or university, 年 月 入学 / 卒業 | Japanese era (令和) or western years: be consistent |
| 職歴 (work history) | 年 月 company 入社 / 退社; last line 「現在に至る」, then 「以上」 right-aligned | |
| 免許・資格 | licences and qualifications, oldest first, with dates | 2026年 6月 基本情報技術者試験 合格 |
| 志望動機 | why this company, 3-5 lines | specific; link your skills to their business |
| 本人希望記入欄 | 「貴社の規定に従います。」 unless you have a real condition | |

Also note 在留資格 and its expiry, and your Japanese level (JLPT), in the 資格 or notes section.
@@file shokumu_keirekisho.md
# 職務経歴書 (career history) template

The 職務経歴書 matters more than the 履歴書 for IT roles: it's where your skills are judged.
A4, 1-3 pages, free format. Write it in Japanese for Japanese companies; an English CV too for 外資系.

```
職務経歴書
2026年11月1日現在
氏名 〇〇 〇〇

■ 職務要約 (summary, 3-5 lines)
ITサポートおよびインフラ運用に〇年間従事してまいりました。
Windows/Microsoft 365 環境の運用、ネットワーク障害の一次対応、PowerShell による業務自動化を得意としております。

■ 職務経歴
〇〇株式会社 (20XX年X月～現在)
事業内容: 〇〇 / 従業員数: 〇〇名 / 雇用形態: 正社員

期間          | 業務内容                                         | 環境・ツール
20XX年X月～   | 社内ヘルプデスク (約300名を担当)                  | Windows 11, Microsoft 365,
              | ・PCキッティング、アカウント管理                  | Entra ID, Intune,
              | ・ネットワーク障害の一次切り分け                  | Cisco スイッチ
              | 【実績】PowerShell でアカウント作成を自動化し、   |
              |  作業時間を月20時間削減                          |
              | 【役割】メンバー (チーム4名)                     |

■ 活かせる経験・知識・技術 (skills)
・OS: Windows Server 2022 (2年), Linux (Ubuntu, 1年)
・ネットワーク: VLAN, TCP/IP, DNS/DHCP の設計・運用
・クラウド: AWS (EC2, S3, IAM), Azure (Entra ID)
・言語: PowerShell (業務利用), Python (自動化スクリプト), SQL

■ 資格
・CompTIA A+ (20XX年X月)
・基本情報技術者試験 (20XX年X月)
・日本語能力試験 N2 (20XX年X月)

■ 自己PR
(One or two concrete stories: the problem, what you did, the result with a number.)

以上
```

Tips
- Numbers everywhere: users supported, tickets per month, hours saved, uptime.
- 【実績】(results) and 【役割】(your role) under each job.
- Link your GitHub with the projects from this app (status board, SLA clock, audit script...).

@@drill
What is the Japanese word for a standard résumé?
= rirekisho
= 履歴書
? The 職務経歴書 (shokumu keirekisho) describes your work history in detail.

@@text
🎉 You've reached the end of the Japan path: language, the national exam, the market, visas, documents, offers and the admin of daily life. Combine it with the technical tracks and their projects (your GitHub portfolio), and you're ready to apply. 頑張ってください!
