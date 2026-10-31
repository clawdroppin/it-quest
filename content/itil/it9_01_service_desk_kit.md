@@lesson it9-service-desk-kit Service Desk Process Kit
@@text
Every service desk lives by its SLAs, and every SLA hides a tricky question: **when is this ticket actually due?** "P3: resolve within 3 business days" sounds simple, until a ticket arrives on Friday at 17:30, the customer goes quiet for two days, and Monday is 文化の日.

You'll build the **SLA clock** a Japanese service desk needs:

| Priority | Clock | Respond within | Resolve within |
|---|---|---|---|
| P1 | 24×7 | 15 min | 4 h |
| P2 | business hours | 30 min | 1 business day (9 h) |
| P3 | business hours | 4 business hours | 3 business days |
| P4 | business hours | 1 business day | 5 business days |

Business hours are **09:00–18:00, Monday–Friday**, minus the dates in `holidays_jp.txt` (2026's 国民の祝日 including the 振替休日 on 6 May and the 国民の休日 on 22 September, plus the usual 年末年始 closure). Time spent **waiting for the customer** (`paused`) doesn't count against the resolve target. A ticket is **at risk** when 25% or less of its resolve time is left, counted in clock minutes, not wall time: Friday 17:00 with a Monday 10:00 deadline has only two business hours left.

Then the second half of the kit: in `LAB.md` you'll write the **process documents** (priority matrix, incident and major incident procedures, a problem record, a standard change and a knowledge article) that make a service desk work. That's ITIL in practice.
@@project service_desk_kit Service Desk Process Kit
Write `add_business_minutes`, `business_minutes_between`, `due_times`, `check` and `summary` in `sla.py` until `python -m unittest` passes, then run `python main.py` to see the SLA board. Then follow `LAB.md` and write the process documents.
@@file sla.py
"""SLA clock for a service desk: due times in business hours, with Japanese holidays and paused time.

All times are local (JST), as naive datetimes. Business hours: 09:00-18:00, Monday-Friday,
except the dates in the holiday file.
"""
from datetime import date, datetime, time, timedelta

WORK_START = time(9, 0)
WORK_END = time(18, 0)
DAY_MINUTES = 9 * 60

# minutes; "24x7" counts every minute, "business" only business hours
SLA = {
    "P1": {"clock": "24x7", "response": 15, "resolve": 4 * 60},
    "P2": {"clock": "business", "response": 30, "resolve": DAY_MINUTES},
    "P3": {"clock": "business", "response": 4 * 60, "resolve": 3 * DAY_MINUTES},
    "P4": {"clock": "business", "response": DAY_MINUTES, "resolve": 5 * DAY_MINUTES},
}
AT_RISK = 0.25   # "at risk" when 25% or less of the resolve time is left


def load_holidays(lines):
    """Dates from lines like "2026-05-06 振替休日". Ignore blank lines and # comments."""
    days = set()
    for line in lines:
        line = line.strip()
        if line and not line.startswith("#"):
            days.add(date.fromisoformat(line.split()[0]))
    return days


def is_business_day(d, holidays):
    return d.weekday() < 5 and d not in holidays


def add_business_minutes(start, minutes, holidays):
    """The moment `minutes` business minutes after `start`.

    A start outside business hours begins counting at the next opening (09:00 on the next
    business day, or today if it's a business day before 09:00). Adding 0 minutes at 18:00
    returns 18:00; the clock only moves forward when there are minutes left to count.
    """
    raise NotImplementedError


def business_minutes_between(start, end, holidays):
    """Business minutes from start to end (0 if end <= start). Whole minutes, rounded down."""
    raise NotImplementedError


def _add(clock, start, minutes, holidays):
    if clock == "24x7":
        return start + timedelta(minutes=minutes)
    return add_business_minutes(start, minutes, holidays)


def _elapsed(clock, start, end, holidays):
    if clock == "24x7":
        return max(0, int((end - start).total_seconds() // 60))
    return business_minutes_between(start, end, holidays)


def due_times(priority, opened, holidays, paused=()):
    """(response_due, resolve_due) for a ticket. `paused` is a list of (start, end) periods
    ("waiting for the customer"); their clock minutes push the resolve time back."""
    raise NotImplementedError


def check(ticket, now, holidays):
    """SLA state of one ticket dict:
        {"id", "priority", "opened", "responded" (or None), "resolved" (or None), "paused": [(start, end)]}
    Returns {"id", "priority", "response", "resolve", "resolve_due"} where
        response: "met" / "breached" (responded late, or not yet and now is past due) / "open"
        resolve:  "met" / "breached" / "at risk" (not breached, 25% or less of the clock left) / "open"
    """
    raise NotImplementedError


def summary(results):
    """Per priority: how many closed tickets met the resolve SLA.

    Returns {"P1": {"closed": n, "met": n, "pct": float or None}, ...} for priorities that appear,
    in P1-P4 order. pct is met/closed*100 rounded to 1 decimal; None when nothing is closed yet.
    Closed = resolve is "met" or "breached".
    """
    raise NotImplementedError
@@file holidays_jp.txt
# Japanese public holidays (国民の祝日) for 2026, plus a typical company year-end closure.
# Always check the Cabinet Office list (内閣府「国民の祝日について」) for the current year:
# equinox days are only fixed in February of the year before, and laws change.
2026-01-01 元日
2026-01-12 成人の日
2026-02-11 建国記念の日
2026-02-23 天皇誕生日
2026-03-20 春分の日
2026-04-29 昭和の日
2026-05-03 憲法記念日
2026-05-04 みどりの日
2026-05-05 こどもの日
2026-05-06 振替休日
2026-07-20 海の日
2026-08-11 山の日
2026-09-21 敬老の日
2026-09-22 国民の休日
2026-09-23 秋分の日
2026-10-12 スポーツの日
2026-11-03 文化の日
2026-11-23 勤労感謝の日
# Company closure (年末年始休暇): not national holidays, but most offices close
2026-12-29 年末年始
2026-12-30 年末年始
2026-12-31 年末年始
2027-01-01 元日
2027-01-11 成人の日
@@file test_sla.py
import os
import unittest
from datetime import date, datetime

from sla import add_business_minutes, business_minutes_between, check, due_times, load_holidays, summary

with open(os.path.join(os.path.dirname(os.path.abspath(__file__)), "holidays_jp.txt"), encoding="utf-8") as f:
    HOLIDAYS = load_holidays(f)


def dt(s):
    return datetime.fromisoformat(s)


class HolidayTests(unittest.TestCase):
    def test_loaded(self):
        self.assertIn(date(2026, 5, 6), HOLIDAYS)          # 振替休日
        self.assertIn(date(2026, 9, 22), HOLIDAYS)         # 国民の休日
        self.assertNotIn(date(2026, 10, 30), HOLIDAYS)

    def test_parsing(self):
        self.assertEqual(load_holidays(["# comment", "", "2026-01-01 元日", "  2026-01-12  成人の日 "]),
                         {date(2026, 1, 1), date(2026, 1, 12)})


class AddTests(unittest.TestCase):
    def test_same_day(self):
        self.assertEqual(add_business_minutes(dt("2026-10-30 10:00"), 90, HOLIDAYS), dt("2026-10-30 11:30"))

    def test_rolls_over_weekend(self):
        # Friday 17:30 + 60 min -> 30 min Friday, 30 min Monday
        self.assertEqual(add_business_minutes(dt("2026-10-30 17:30"), 60, HOLIDAYS), dt("2026-11-02 09:30"))

    def test_skips_holiday(self):
        # Monday 2 Nov 17:00 + 120 -> Tuesday 3 Nov is 文化の日 -> Wednesday 10:00
        self.assertEqual(add_business_minutes(dt("2026-11-02 17:00"), 120, HOLIDAYS), dt("2026-11-04 10:00"))

    def test_opened_outside_hours(self):
        self.assertEqual(add_business_minutes(dt("2026-10-31 14:00"), 30, HOLIDAYS), dt("2026-11-02 09:30"))   # Saturday
        self.assertEqual(add_business_minutes(dt("2026-10-30 07:15"), 30, HOLIDAYS), dt("2026-10-30 09:30"))   # early
        self.assertEqual(add_business_minutes(dt("2026-10-30 18:00"), 30, HOLIDAYS), dt("2026-11-02 09:30"))   # closing

    def test_exactly_to_closing(self):
        self.assertEqual(add_business_minutes(dt("2026-10-30 17:00"), 60, HOLIDAYS), dt("2026-10-30 18:00"))
        self.assertEqual(add_business_minutes(dt("2026-10-30 18:00"), 0, HOLIDAYS), dt("2026-10-30 18:00"))

    def test_golden_week(self):
        # Thursday 30 April 17:00 + 27 business hours (3 days):
        # 1 h Thu, 9 h Fri 1 May, weekend + 4-6 May holidays, 9 h Thu 7 May, 8 h Fri 8 May -> 17:00
        self.assertEqual(add_business_minutes(dt("2026-04-30 17:00"), 27 * 60, HOLIDAYS), dt("2026-05-08 17:00"))


class BetweenTests(unittest.TestCase):
    def test_counts_only_business_time(self):
        self.assertEqual(business_minutes_between(dt("2026-10-30 17:00"), dt("2026-11-02 10:00"), HOLIDAYS), 120)
        self.assertEqual(business_minutes_between(dt("2026-10-31 09:00"), dt("2026-11-01 18:00"), HOLIDAYS), 0)
        self.assertEqual(business_minutes_between(dt("2026-10-30 12:00"), dt("2026-10-30 11:00"), HOLIDAYS), 0)
        self.assertEqual(business_minutes_between(dt("2026-10-29 08:00"), dt("2026-10-30 20:00"), HOLIDAYS), 18 * 60)


class DueTests(unittest.TestCase):
    def test_p1_runs_24x7(self):
        self.assertEqual(due_times("P1", dt("2026-10-31 23:00"), HOLIDAYS),
                         (dt("2026-10-31 23:15"), dt("2026-11-01 03:00")))

    def test_p3_business(self):
        response, resolve = due_times("P3", dt("2026-10-30 16:00"), HOLIDAYS)
        self.assertEqual(response, dt("2026-11-02 11:00"))
        self.assertEqual(resolve, dt("2026-11-05 16:00"))       # 3 business days, skipping 文化の日

    def test_pause_pushes_resolve(self):
        paused = [(dt("2026-10-30 10:00"), dt("2026-10-30 12:00"))]
        self.assertEqual(due_times("P2", dt("2026-10-30 09:00"), HOLIDAYS, paused)[1], dt("2026-11-02 11:00"))


class CheckTests(unittest.TestCase):
    def ticket(self, **kw):
        t = {"id": "INC1", "priority": "P2", "opened": dt("2026-10-30 09:00"), "responded": None,
             "resolved": None, "paused": []}
        t.update(kw)
        return t

    def test_met(self):
        r = check(self.ticket(responded=dt("2026-10-30 09:20"), resolved=dt("2026-10-30 15:00")),
                  dt("2026-10-30 16:00"), HOLIDAYS)
        self.assertEqual((r["response"], r["resolve"]), ("met", "met"))
        self.assertEqual(r["resolve_due"], dt("2026-10-30 18:00"))

    def test_late_response_but_open(self):
        r = check(self.ticket(responded=dt("2026-10-30 09:45")), dt("2026-10-30 10:00"), HOLIDAYS)
        self.assertEqual((r["response"], r["resolve"]), ("breached", "open"))

    def test_no_response_yet(self):
        self.assertEqual(check(self.ticket(), dt("2026-10-30 09:10"), HOLIDAYS)["response"], "open")
        self.assertEqual(check(self.ticket(), dt("2026-10-30 09:31"), HOLIDAYS)["response"], "breached")

    def test_at_risk(self):
        # P2 resolve = 540 min; at risk with 135 min or less left: due 18:00 -> from 15:45
        self.assertEqual(check(self.ticket(), dt("2026-10-30 15:44"), HOLIDAYS)["resolve"], "open")
        self.assertEqual(check(self.ticket(), dt("2026-10-30 15:45"), HOLIDAYS)["resolve"], "at risk")
        self.assertEqual(check(self.ticket(), dt("2026-10-30 18:01"), HOLIDAYS)["resolve"], "breached")

    def test_at_risk_counts_business_time(self):
        # P2 opened Friday 10:00 is due Monday 10:00 (8 h Friday + 1 h Monday).
        # Friday 17:00 has 65 wall-clock hours left but only 120 business minutes: at risk.
        t = self.ticket(opened=dt("2026-10-30 10:00"))
        self.assertEqual(check(t, dt("2026-10-30 16:00"), HOLIDAYS)["resolve"], "open")
        self.assertEqual(check(t, dt("2026-10-30 17:00"), HOLIDAYS)["resolve"], "at risk")
        self.assertEqual(check(t, dt("2026-11-01 12:00"), HOLIDAYS)["resolve"], "at risk")   # Sunday: not breached
        self.assertEqual(check(t, dt("2026-11-02 10:01"), HOLIDAYS)["resolve"], "breached")

    def test_late_resolution(self):
        r = check(self.ticket(resolved=dt("2026-11-02 10:00")), dt("2026-11-02 12:00"), HOLIDAYS)
        self.assertEqual(r["resolve"], "breached")


class SummaryTests(unittest.TestCase):
    def test_summary(self):
        results = [
            {"priority": "P2", "resolve": "met"}, {"priority": "P2", "resolve": "breached"},
            {"priority": "P2", "resolve": "met"}, {"priority": "P1", "resolve": "open"},
            {"priority": "P4", "resolve": "met"},
        ]
        self.assertEqual(summary(results), {
            "P1": {"closed": 0, "met": 0, "pct": None},
            "P2": {"closed": 3, "met": 2, "pct": 66.7},
            "P4": {"closed": 1, "met": 1, "pct": 100.0},
        })
        self.assertEqual(list(summary(results)), ["P1", "P2", "P4"])


if __name__ == "__main__":
    unittest.main()
@@file tickets.json
[
  {"id": "INC0101", "priority": "P1", "summary": "Core switch down, whole 3F offline",
   "opened": "2026-10-29T08:40", "responded": "2026-10-29T08:47", "resolved": "2026-10-29T11:05", "paused": []},
  {"id": "INC0102", "priority": "P2", "summary": "Sales team can't reach the CRM",
   "opened": "2026-10-29T13:10", "responded": "2026-10-29T13:25", "resolved": "2026-10-30T09:50", "paused": []},
  {"id": "INC0103", "priority": "P3", "summary": "Printer on 2F jams every morning",
   "opened": "2026-10-27T10:00", "responded": "2026-10-27T11:30", "resolved": null,
   "paused": [["2026-10-28T09:00", "2026-10-29T12:00"]]},
  {"id": "INC0104", "priority": "P2", "summary": "VPN disconnects every 10 minutes for Osaka branch",
   "opened": "2026-10-30T11:00", "responded": "2026-10-30T11:50", "resolved": null, "paused": []},
  {"id": "INC0105", "priority": "P4", "summary": "Request: second monitor for new hire",
   "opened": "2026-10-26T15:00", "responded": "2026-10-27T09:10", "resolved": "2026-10-30T16:00", "paused": []},
  {"id": "INC0106", "priority": "P3", "summary": "Outlook search returns nothing",
   "opened": "2026-10-26T09:30", "responded": "2026-10-26T10:00", "resolved": "2026-10-30T17:00", "paused": []},
  {"id": "INC0107", "priority": "P1", "summary": "Ransomware note on file server",
   "opened": "2026-10-31T02:10", "responded": "2026-10-31T02:20", "resolved": null, "paused": []},
  {"id": "INC0108", "priority": "P4", "summary": "Update the Wi-Fi guest password poster",
   "opened": "2026-10-30T17:30", "responded": null, "resolved": null, "paused": []}
]
@@file main.py
"""Service desk SLA board: which tickets are breached, at risk or fine.

    python main.py                              (now = 2026-10-31 05:00, the sample data's "now")
    python main.py --now 2026-11-02T10:00
    python main.py --now now                    (the real current time)
"""
import argparse
import json
import os
from datetime import datetime

from sla import check, load_holidays, summary

HERE = os.path.dirname(os.path.abspath(__file__))


def parse(value):
    return datetime.fromisoformat(value) if value else None


def load_tickets(path):
    with open(path, encoding="utf-8") as f:
        raw = json.load(f)
    tickets = []
    for t in raw:
        tickets.append({**t, "opened": parse(t["opened"]), "responded": parse(t.get("responded")),
                        "resolved": parse(t.get("resolved")),
                        "paused": [(parse(s), parse(e)) for s, e in t.get("paused", [])]})
    return tickets


def main(argv=None):
    ap = argparse.ArgumentParser(description="Service desk SLA board")
    ap.add_argument("tickets", nargs="?", default=os.path.join(HERE, "tickets.json"))
    ap.add_argument("--now", default="2026-10-31T05:00")
    args = ap.parse_args(argv)
    now = datetime.now().replace(second=0, microsecond=0) if args.now == "now" else parse(args.now)

    with open(os.path.join(HERE, "holidays_jp.txt"), encoding="utf-8") as f:
        holidays = load_holidays(f)
    tickets = load_tickets(args.tickets)
    results = [check(t, now, holidays) for t in tickets]

    print(f"SLA board at {now:%Y-%m-%d %H:%M} (JST)\n")
    print(f"{'ticket':9} {'pri':3}  {'response':9} {'resolve':9} {'resolve due':16}  summary")
    order = {"breached": 0, "at risk": 1, "open": 2, "met": 3}
    for t, r in sorted(zip(tickets, results), key=lambda x: (order[x[1]["resolve"]], x[1]["resolve_due"])):
        print(f"{r['id']:9} {r['priority']:3}  {r['response']:9} {r['resolve']:9} "
              f"{r['resolve_due']:%Y-%m-%d %H:%M}  {t['summary']}")

    print("\nResolved within SLA:")
    for p, s in summary(results).items():
        pct = "-" if s["pct"] is None else f"{s['pct']}%"
        print(f"  {p}: {s['met']}/{s['closed']} ({pct})")


if __name__ == "__main__":
    main()
@@file LAB.md
# Service Desk Process Kit

The code is half of this project. The other half is the **process**, which is what ITIL is really about.

## 1. The SLA clock
```
python -m unittest -v
python main.py
python main.py --now 2026-11-02T10:00
```
Explain to yourself why INC0104 is *at risk* on Saturday morning even though it's due Monday:
only business minutes count, and very few are left.

## 2. Write the process (put these in a `process/` folder, as Markdown)
1. **Priority matrix**: impact (1 person / a department / the company) × urgency (low / medium / high) → P1-P4.
   Add two real examples for each priority.
2. **Incident procedure**: log → categorize → prioritize → diagnose → escalate (functional / hierarchical)
   → resolve → close (confirm with the user). Who does what? When do you call the P1 bridge?
3. **Major incident** checklist: incident commander, communication every 30 minutes, status page,
   management update, and a blameless review afterwards.
4. **Problem record** for INC0103 (the printer that jams every morning): symptoms, workaround,
   root cause analysis (5 whys), known error, the change you'd request.
5. **Standard change** template: e.g. "add a user to a shared mailbox": pre-approved steps, risk, rollback.
6. **Knowledge article** for the most common call at your workplace or lab, in the
   problem / cause / resolution format, written so a new colleague can follow it.

## 3. Extend the code
- Add `--csv` output for a weekly SLA report to management.
- Read tickets from a real export (Jira Service Management, ServiceNow, Zendesk, Redmine can all export CSV).
- Add 2027 holidays from the Cabinet Office list when they're published.
- Add an **OLA** check: the network team must answer escalations within 2 business hours.

## Interview tip (日本語)
障害対応 (incident response) and 運用保守 (operations and maintenance) jobs ask about exactly this.
Be ready to explain 一次対応 (first-line response), エスカレーション, SLA, and 再発防止策 (preventing recurrence),
which is problem management.
@@text
> Tips: walk forward one day at a time. For each day, skip it if it's not a business day; otherwise clamp the start to 09:00, see how many minutes are left before 18:00, and either finish today or carry the rest into tomorrow. `datetime.combine(d, WORK_START)` builds 09:00 on day `d`. For `business_minutes_between`, overlap each day's 09:00–18:00 window with `[start, end]` using `max` and `min`. The private helpers `_add` and `_elapsed` already choose between the 24×7 and business clocks for you.
@@project service_desk_kit_finished Service Desk Process Kit: the finished version
The complete SLA clock. Next steps: per-customer calendars (a client in Osaka with different holidays), OLAs for internal teams, CSV export for the weekly report, or a small web board using the status board from the Cloud track.
@@file sla.py
"""SLA clock for a service desk: due times in business hours, with Japanese holidays and paused time.

All times are local (JST), as naive datetimes. Business hours: 09:00-18:00, Monday-Friday,
except the dates in the holiday file.
"""
from datetime import date, datetime, time, timedelta

WORK_START = time(9, 0)
WORK_END = time(18, 0)
DAY_MINUTES = 9 * 60

# minutes; "24x7" counts every minute, "business" only business hours
SLA = {
    "P1": {"clock": "24x7", "response": 15, "resolve": 4 * 60},
    "P2": {"clock": "business", "response": 30, "resolve": DAY_MINUTES},
    "P3": {"clock": "business", "response": 4 * 60, "resolve": 3 * DAY_MINUTES},
    "P4": {"clock": "business", "response": DAY_MINUTES, "resolve": 5 * DAY_MINUTES},
}
AT_RISK = 0.25   # "at risk" when 25% or less of the resolve time is left


def load_holidays(lines):
    """Dates from lines like "2026-05-06 振替休日". Ignore blank lines and # comments."""
    days = set()
    for line in lines:
        line = line.strip()
        if line and not line.startswith("#"):
            days.add(date.fromisoformat(line.split()[0]))
    return days


def is_business_day(d, holidays):
    return d.weekday() < 5 and d not in holidays


def add_business_minutes(start, minutes, holidays):
    """The moment `minutes` business minutes after `start`.

    A start outside business hours begins counting at the next opening (09:00 on the next
    business day, or today if it's a business day before 09:00). Adding 0 minutes at 18:00
    returns 18:00; the clock only moves forward when there are minutes left to count.
    """
    current = start
    remaining = minutes
    while True:
        d = current.date()
        opening = datetime.combine(d, WORK_START)
        closing = datetime.combine(d, WORK_END)
        if not is_business_day(d, holidays) or current >= closing:
            if remaining == 0 and is_business_day(d, holidays) and current == closing:
                return current
            current = datetime.combine(d + timedelta(days=1), WORK_START)
            continue
        if current < opening:
            current = opening
        available = (closing - current).total_seconds() / 60
        if remaining <= available:
            return current + timedelta(minutes=remaining)
        remaining -= available
        current = closing


def business_minutes_between(start, end, holidays):
    """Business minutes from start to end (0 if end <= start). Whole minutes, rounded down."""
    if end <= start:
        return 0
    total = 0.0
    d = start.date()
    while d <= end.date():
        if is_business_day(d, holidays):
            lo = max(start, datetime.combine(d, WORK_START))
            hi = min(end, datetime.combine(d, WORK_END))
            if hi > lo:
                total += (hi - lo).total_seconds() / 60
        d += timedelta(days=1)
    return int(total)


def _add(clock, start, minutes, holidays):
    if clock == "24x7":
        return start + timedelta(minutes=minutes)
    return add_business_minutes(start, minutes, holidays)


def _elapsed(clock, start, end, holidays):
    if clock == "24x7":
        return max(0, int((end - start).total_seconds() // 60))
    return business_minutes_between(start, end, holidays)


def due_times(priority, opened, holidays, paused=()):
    """(response_due, resolve_due) for a ticket. `paused` is a list of (start, end) periods
    ("waiting for the customer"); their clock minutes push the resolve time back."""
    rule = SLA[priority]
    paused_minutes = sum(_elapsed(rule["clock"], s, e, holidays) for s, e in paused)
    response_due = _add(rule["clock"], opened, rule["response"], holidays)
    resolve_due = _add(rule["clock"], opened, rule["resolve"] + paused_minutes, holidays)
    return response_due, resolve_due


def check(ticket, now, holidays):
    """SLA state of one ticket dict:
        {"id", "priority", "opened", "responded" (or None), "resolved" (or None), "paused": [(start, end)]}
    Returns {"id", "priority", "response", "resolve", "resolve_due"} where
        response: "met" / "breached" (responded late, or not yet and now is past due) / "open"
        resolve:  "met" / "breached" / "at risk" (not breached, 25% or less of the clock left) / "open"
    """
    rule = SLA[ticket["priority"]]
    response_due, resolve_due = due_times(ticket["priority"], ticket["opened"], holidays, ticket.get("paused", ()))

    def judge(done, due):
        if done is not None:
            return "met" if done <= due else "breached"
        return "breached" if now > due else "open"

    response = judge(ticket.get("responded"), response_due)
    resolve = judge(ticket.get("resolved"), resolve_due)
    if resolve == "open":
        left = _elapsed(rule["clock"], now, resolve_due, holidays)
        if left <= rule["resolve"] * AT_RISK:
            resolve = "at risk"
    return {"id": ticket["id"], "priority": ticket["priority"], "response": response,
            "resolve": resolve, "resolve_due": resolve_due}


def summary(results):
    """Per priority: how many closed tickets met the resolve SLA.

    Returns {"P1": {"closed": n, "met": n, "pct": float or None}, ...} for priorities that appear,
    in P1-P4 order. pct is met/closed*100 rounded to 1 decimal; None when nothing is closed yet.
    Closed = resolve is "met" or "breached".
    """
    out = {}
    for p in SLA:
        mine = [r for r in results if r["priority"] == p]
        if not mine:
            continue
        closed = [r for r in mine if r["resolve"] in ("met", "breached")]
        met = sum(1 for r in closed if r["resolve"] == "met")
        out[p] = {"closed": len(closed), "met": met,
                  "pct": round(met / len(closed) * 100, 1) if closed else None}
    return out
@@file holidays_jp.txt
# Japanese public holidays (国民の祝日) for 2026, plus a typical company year-end closure.
# Always check the Cabinet Office list (内閣府「国民の祝日について」) for the current year:
# equinox days are only fixed in February of the year before, and laws change.
2026-01-01 元日
2026-01-12 成人の日
2026-02-11 建国記念の日
2026-02-23 天皇誕生日
2026-03-20 春分の日
2026-04-29 昭和の日
2026-05-03 憲法記念日
2026-05-04 みどりの日
2026-05-05 こどもの日
2026-05-06 振替休日
2026-07-20 海の日
2026-08-11 山の日
2026-09-21 敬老の日
2026-09-22 国民の休日
2026-09-23 秋分の日
2026-10-12 スポーツの日
2026-11-03 文化の日
2026-11-23 勤労感謝の日
# Company closure (年末年始休暇): not national holidays, but most offices close
2026-12-29 年末年始
2026-12-30 年末年始
2026-12-31 年末年始
2027-01-01 元日
2027-01-11 成人の日
@@file test_sla.py
import os
import unittest
from datetime import date, datetime

from sla import add_business_minutes, business_minutes_between, check, due_times, load_holidays, summary

with open(os.path.join(os.path.dirname(os.path.abspath(__file__)), "holidays_jp.txt"), encoding="utf-8") as f:
    HOLIDAYS = load_holidays(f)


def dt(s):
    return datetime.fromisoformat(s)


class HolidayTests(unittest.TestCase):
    def test_loaded(self):
        self.assertIn(date(2026, 5, 6), HOLIDAYS)          # 振替休日
        self.assertIn(date(2026, 9, 22), HOLIDAYS)         # 国民の休日
        self.assertNotIn(date(2026, 10, 30), HOLIDAYS)

    def test_parsing(self):
        self.assertEqual(load_holidays(["# comment", "", "2026-01-01 元日", "  2026-01-12  成人の日 "]),
                         {date(2026, 1, 1), date(2026, 1, 12)})


class AddTests(unittest.TestCase):
    def test_same_day(self):
        self.assertEqual(add_business_minutes(dt("2026-10-30 10:00"), 90, HOLIDAYS), dt("2026-10-30 11:30"))

    def test_rolls_over_weekend(self):
        # Friday 17:30 + 60 min -> 30 min Friday, 30 min Monday
        self.assertEqual(add_business_minutes(dt("2026-10-30 17:30"), 60, HOLIDAYS), dt("2026-11-02 09:30"))

    def test_skips_holiday(self):
        # Monday 2 Nov 17:00 + 120 -> Tuesday 3 Nov is 文化の日 -> Wednesday 10:00
        self.assertEqual(add_business_minutes(dt("2026-11-02 17:00"), 120, HOLIDAYS), dt("2026-11-04 10:00"))

    def test_opened_outside_hours(self):
        self.assertEqual(add_business_minutes(dt("2026-10-31 14:00"), 30, HOLIDAYS), dt("2026-11-02 09:30"))   # Saturday
        self.assertEqual(add_business_minutes(dt("2026-10-30 07:15"), 30, HOLIDAYS), dt("2026-10-30 09:30"))   # early
        self.assertEqual(add_business_minutes(dt("2026-10-30 18:00"), 30, HOLIDAYS), dt("2026-11-02 09:30"))   # closing

    def test_exactly_to_closing(self):
        self.assertEqual(add_business_minutes(dt("2026-10-30 17:00"), 60, HOLIDAYS), dt("2026-10-30 18:00"))
        self.assertEqual(add_business_minutes(dt("2026-10-30 18:00"), 0, HOLIDAYS), dt("2026-10-30 18:00"))

    def test_golden_week(self):
        # Thursday 30 April 17:00 + 27 business hours (3 days):
        # 1 h Thu, 9 h Fri 1 May, weekend + 4-6 May holidays, 9 h Thu 7 May, 8 h Fri 8 May -> 17:00
        self.assertEqual(add_business_minutes(dt("2026-04-30 17:00"), 27 * 60, HOLIDAYS), dt("2026-05-08 17:00"))


class BetweenTests(unittest.TestCase):
    def test_counts_only_business_time(self):
        self.assertEqual(business_minutes_between(dt("2026-10-30 17:00"), dt("2026-11-02 10:00"), HOLIDAYS), 120)
        self.assertEqual(business_minutes_between(dt("2026-10-31 09:00"), dt("2026-11-01 18:00"), HOLIDAYS), 0)
        self.assertEqual(business_minutes_between(dt("2026-10-30 12:00"), dt("2026-10-30 11:00"), HOLIDAYS), 0)
        self.assertEqual(business_minutes_between(dt("2026-10-29 08:00"), dt("2026-10-30 20:00"), HOLIDAYS), 18 * 60)


class DueTests(unittest.TestCase):
    def test_p1_runs_24x7(self):
        self.assertEqual(due_times("P1", dt("2026-10-31 23:00"), HOLIDAYS),
                         (dt("2026-10-31 23:15"), dt("2026-11-01 03:00")))

    def test_p3_business(self):
        response, resolve = due_times("P3", dt("2026-10-30 16:00"), HOLIDAYS)
        self.assertEqual(response, dt("2026-11-02 11:00"))
        self.assertEqual(resolve, dt("2026-11-05 16:00"))       # 3 business days, skipping 文化の日

    def test_pause_pushes_resolve(self):
        paused = [(dt("2026-10-30 10:00"), dt("2026-10-30 12:00"))]
        self.assertEqual(due_times("P2", dt("2026-10-30 09:00"), HOLIDAYS, paused)[1], dt("2026-11-02 11:00"))


class CheckTests(unittest.TestCase):
    def ticket(self, **kw):
        t = {"id": "INC1", "priority": "P2", "opened": dt("2026-10-30 09:00"), "responded": None,
             "resolved": None, "paused": []}
        t.update(kw)
        return t

    def test_met(self):
        r = check(self.ticket(responded=dt("2026-10-30 09:20"), resolved=dt("2026-10-30 15:00")),
                  dt("2026-10-30 16:00"), HOLIDAYS)
        self.assertEqual((r["response"], r["resolve"]), ("met", "met"))
        self.assertEqual(r["resolve_due"], dt("2026-10-30 18:00"))

    def test_late_response_but_open(self):
        r = check(self.ticket(responded=dt("2026-10-30 09:45")), dt("2026-10-30 10:00"), HOLIDAYS)
        self.assertEqual((r["response"], r["resolve"]), ("breached", "open"))

    def test_no_response_yet(self):
        self.assertEqual(check(self.ticket(), dt("2026-10-30 09:10"), HOLIDAYS)["response"], "open")
        self.assertEqual(check(self.ticket(), dt("2026-10-30 09:31"), HOLIDAYS)["response"], "breached")

    def test_at_risk(self):
        # P2 resolve = 540 min; at risk with 135 min or less left: due 18:00 -> from 15:45
        self.assertEqual(check(self.ticket(), dt("2026-10-30 15:44"), HOLIDAYS)["resolve"], "open")
        self.assertEqual(check(self.ticket(), dt("2026-10-30 15:45"), HOLIDAYS)["resolve"], "at risk")
        self.assertEqual(check(self.ticket(), dt("2026-10-30 18:01"), HOLIDAYS)["resolve"], "breached")

    def test_at_risk_counts_business_time(self):
        # P2 opened Friday 10:00 is due Monday 10:00 (8 h Friday + 1 h Monday).
        # Friday 17:00 has 65 wall-clock hours left but only 120 business minutes: at risk.
        t = self.ticket(opened=dt("2026-10-30 10:00"))
        self.assertEqual(check(t, dt("2026-10-30 16:00"), HOLIDAYS)["resolve"], "open")
        self.assertEqual(check(t, dt("2026-10-30 17:00"), HOLIDAYS)["resolve"], "at risk")
        self.assertEqual(check(t, dt("2026-11-01 12:00"), HOLIDAYS)["resolve"], "at risk")   # Sunday: not breached
        self.assertEqual(check(t, dt("2026-11-02 10:01"), HOLIDAYS)["resolve"], "breached")

    def test_late_resolution(self):
        r = check(self.ticket(resolved=dt("2026-11-02 10:00")), dt("2026-11-02 12:00"), HOLIDAYS)
        self.assertEqual(r["resolve"], "breached")


class SummaryTests(unittest.TestCase):
    def test_summary(self):
        results = [
            {"priority": "P2", "resolve": "met"}, {"priority": "P2", "resolve": "breached"},
            {"priority": "P2", "resolve": "met"}, {"priority": "P1", "resolve": "open"},
            {"priority": "P4", "resolve": "met"},
        ]
        self.assertEqual(summary(results), {
            "P1": {"closed": 0, "met": 0, "pct": None},
            "P2": {"closed": 3, "met": 2, "pct": 66.7},
            "P4": {"closed": 1, "met": 1, "pct": 100.0},
        })
        self.assertEqual(list(summary(results)), ["P1", "P2", "P4"])


if __name__ == "__main__":
    unittest.main()
@@file tickets.json
[
  {"id": "INC0101", "priority": "P1", "summary": "Core switch down, whole 3F offline",
   "opened": "2026-10-29T08:40", "responded": "2026-10-29T08:47", "resolved": "2026-10-29T11:05", "paused": []},
  {"id": "INC0102", "priority": "P2", "summary": "Sales team can't reach the CRM",
   "opened": "2026-10-29T13:10", "responded": "2026-10-29T13:25", "resolved": "2026-10-30T09:50", "paused": []},
  {"id": "INC0103", "priority": "P3", "summary": "Printer on 2F jams every morning",
   "opened": "2026-10-27T10:00", "responded": "2026-10-27T11:30", "resolved": null,
   "paused": [["2026-10-28T09:00", "2026-10-29T12:00"]]},
  {"id": "INC0104", "priority": "P2", "summary": "VPN disconnects every 10 minutes for Osaka branch",
   "opened": "2026-10-30T11:00", "responded": "2026-10-30T11:50", "resolved": null, "paused": []},
  {"id": "INC0105", "priority": "P4", "summary": "Request: second monitor for new hire",
   "opened": "2026-10-26T15:00", "responded": "2026-10-27T09:10", "resolved": "2026-10-30T16:00", "paused": []},
  {"id": "INC0106", "priority": "P3", "summary": "Outlook search returns nothing",
   "opened": "2026-10-26T09:30", "responded": "2026-10-26T10:00", "resolved": "2026-10-30T17:00", "paused": []},
  {"id": "INC0107", "priority": "P1", "summary": "Ransomware note on file server",
   "opened": "2026-10-31T02:10", "responded": "2026-10-31T02:20", "resolved": null, "paused": []},
  {"id": "INC0108", "priority": "P4", "summary": "Update the Wi-Fi guest password poster",
   "opened": "2026-10-30T17:30", "responded": null, "resolved": null, "paused": []}
]
@@file main.py
"""Service desk SLA board: which tickets are breached, at risk or fine.

    python main.py                              (now = 2026-10-31 05:00, the sample data's "now")
    python main.py --now 2026-11-02T10:00
    python main.py --now now                    (the real current time)
"""
import argparse
import json
import os
from datetime import datetime

from sla import check, load_holidays, summary

HERE = os.path.dirname(os.path.abspath(__file__))


def parse(value):
    return datetime.fromisoformat(value) if value else None


def load_tickets(path):
    with open(path, encoding="utf-8") as f:
        raw = json.load(f)
    tickets = []
    for t in raw:
        tickets.append({**t, "opened": parse(t["opened"]), "responded": parse(t.get("responded")),
                        "resolved": parse(t.get("resolved")),
                        "paused": [(parse(s), parse(e)) for s, e in t.get("paused", [])]})
    return tickets


def main(argv=None):
    ap = argparse.ArgumentParser(description="Service desk SLA board")
    ap.add_argument("tickets", nargs="?", default=os.path.join(HERE, "tickets.json"))
    ap.add_argument("--now", default="2026-10-31T05:00")
    args = ap.parse_args(argv)
    now = datetime.now().replace(second=0, microsecond=0) if args.now == "now" else parse(args.now)

    with open(os.path.join(HERE, "holidays_jp.txt"), encoding="utf-8") as f:
        holidays = load_holidays(f)
    tickets = load_tickets(args.tickets)
    results = [check(t, now, holidays) for t in tickets]

    print(f"SLA board at {now:%Y-%m-%d %H:%M} (JST)\n")
    print(f"{'ticket':9} {'pri':3}  {'response':9} {'resolve':9} {'resolve due':16}  summary")
    order = {"breached": 0, "at risk": 1, "open": 2, "met": 3}
    for t, r in sorted(zip(tickets, results), key=lambda x: (order[x[1]["resolve"]], x[1]["resolve_due"])):
        print(f"{r['id']:9} {r['priority']:3}  {r['response']:9} {r['resolve']:9} "
              f"{r['resolve_due']:%Y-%m-%d %H:%M}  {t['summary']}")

    print("\nResolved within SLA:")
    for p, s in summary(results).items():
        pct = "-" if s["pct"] is None else f"{s['pct']}%"
        print(f"  {p}: {s['met']}/{s['closed']} ({pct})")


if __name__ == "__main__":
    main()
@@file LAB.md
# Service Desk Process Kit

The code is half of this project. The other half is the **process**, which is what ITIL is really about.

## 1. The SLA clock
```
python -m unittest -v
python main.py
python main.py --now 2026-11-02T10:00
```
Explain to yourself why INC0104 is *at risk* on Saturday morning even though it's due Monday:
only business minutes count, and very few are left.

## 2. Write the process (put these in a `process/` folder, as Markdown)
1. **Priority matrix**: impact (1 person / a department / the company) × urgency (low / medium / high) → P1-P4.
   Add two real examples for each priority.
2. **Incident procedure**: log → categorize → prioritize → diagnose → escalate (functional / hierarchical)
   → resolve → close (confirm with the user). Who does what? When do you call the P1 bridge?
3. **Major incident** checklist: incident commander, communication every 30 minutes, status page,
   management update, and a blameless review afterwards.
4. **Problem record** for INC0103 (the printer that jams every morning): symptoms, workaround,
   root cause analysis (5 whys), known error, the change you'd request.
5. **Standard change** template: e.g. "add a user to a shared mailbox": pre-approved steps, risk, rollback.
6. **Knowledge article** for the most common call at your workplace or lab, in the
   problem / cause / resolution format, written so a new colleague can follow it.

## 3. Extend the code
- Add `--csv` output for a weekly SLA report to management.
- Read tickets from a real export (Jira Service Management, ServiceNow, Zendesk, Redmine can all export CSV).
- Add 2027 holidays from the Cabinet Office list when they're published.
- Add an **OLA** check: the network team must answer escalations within 2 business hours.

## Interview tip (日本語)
障害対応 (incident response) and 運用保守 (operations and maintenance) jobs ask about exactly this.
Be ready to explain 一次対応 (first-line response), エスカレーション, SLA, and 再発防止策 (preventing recurrence),
which is problem management.
@@text
🎉 Incidents, service requests, priorities, SLAs, OLAs, pausing the clock, problem records, standard changes, knowledge articles: you've now *done* the practices the ITIL Foundation exam asks about. In a 運用保守 interview, talking through this kit shows you understand the job, not just the vocabulary.
