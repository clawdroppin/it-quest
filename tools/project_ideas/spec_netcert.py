from gen_projects import P

PROJECTS = [
    P("ccna", "Intermediate", "config_diff", "Router Config Backup and Diff", "Keep config backups and see exactly what changed between them.",
      """Most outages start with a change. Save `show running-config` output from your lab routers (Packet Tracer, GNS3, CML or real gear) into dated files, and write a tool that compares two backups section by section: which interfaces, ACLs, routing statements and VLANs were added, removed or changed.""",
      ["IOS configuration structure", "parent/child sections", "difflib", "change management"],
      ["Collect: save two configs of the same device before and after a lab change.",
       "Parse: group lines into sections (a line without indentation starts a new one).",
       "Compare: added, removed and changed sections, ignoring timestamps and `!` lines.",
       "Report: a readable summary, then the line-level diff of changed sections.",
       "Process: write a CHANGE.md template (what, why, rollback plan) like a real change request."],
      ["Comparing the two sample configs reports exactly the three changes made."],
      ["Which lines in a running-config change on their own and should be ignored?",
       "How do I group IOS config lines into sections?"],
      {"config_diff.py": '''"""Compare two IOS config backups: python config_diff.py before.cfg after.cfg"""
import sys


def sections(text: str) -> dict[str, list[str]]:
    """Top-level line → its indented child lines."""
    out: dict[str, list[str]] = {}
    current = None
    for line in text.splitlines():
        if not line.strip() or line.startswith("!"):
            continue
        if not line.startswith(" "):
            current = line.strip()
            out[current] = []
        elif current:
            out[current].append(line.strip())
    return out


if __name__ == "__main__":
    a = sections(open(sys.argv[1], encoding="utf-8").read())
    b = sections(open(sys.argv[2], encoding="utf-8").read())
    # TODO: added, removed, changed
    print(len(a), "sections before,", len(b), "after")
''', "before.cfg": '''hostname R1
!
interface GigabitEthernet0/0
 ip address 10.0.0.1 255.255.255.0
 no shutdown
!
interface GigabitEthernet0/1
 shutdown
!
router ospf 1
 network 10.0.0.0 0.0.0.255 area 0
!
line vty 0 4
 transport input ssh
''', "after.cfg": '''hostname R1
!
interface GigabitEthernet0/0
 ip address 10.0.0.1 255.255.255.0
 ip access-group MGMT in
 no shutdown
!
interface GigabitEthernet0/1
 ip address 10.1.0.1 255.255.255.0
 no shutdown
!
router ospf 1
 network 10.0.0.0 0.0.0.255 area 0
 network 10.1.0.0 0.0.0.255 area 0
!
ip access-list standard MGMT
 permit 10.0.0.0 0.0.0.255
!
line vty 0 4
 transport input ssh
'''}),
    P("ccnp", "Expert", "sdwan_policy", "SD-WAN Path Selection Simulator", "Choose a WAN link per application from latency, loss and jitter.",
      """Application-aware routing in SD-WAN measures every tunnel continuously and moves traffic when a link breaks its SLA. Simulate it: links with measured latency, loss and jitter over time, SLA classes per application (voice, video, business, bulk), and a policy that picks the preferred link that meets the SLA, with fallback and hysteresis so traffic doesn't flap.""",
      ["SD-WAN application-aware routing", "SLA classes", "hysteresis", "simulation over time"],
      ["Data: measurements.csv with time, link (mpls, inet, lte) and latency/loss/jitter.",
       "SLA classes: voice ≤150 ms, ≤1% loss, ≤30 ms jitter; define the others.",
       "Choose: for each app and time step, the preferred link meeting the SLA, else the best available.",
       "Hysteresis: only move back to the preferred link after it has been good for 3 intervals.",
       "Report: a timeline of which link each app used, and how many times it switched."],
      ["The simulation moves voice off the internet link during the sample's loss spike and back afterwards, without flapping."],
      ["How do real SD-WAN products measure loss and jitter on tunnels?",
       "Why is hysteresis important in path selection?"],
      {"sdwan.py": '''"""Simulate SD-WAN application-aware routing: python sdwan.py measurements.csv"""
import csv
import sys

SLA = {
    "voice": {"latency": 150, "loss": 1.0, "jitter": 30},
    # TODO: video, business, bulk
}
PREFERENCE = {"voice": ["mpls", "inet", "lte"]}


def meets(m: dict, sla: dict) -> bool:
    return m["latency"] <= sla["latency"] and m["loss"] <= sla["loss"] and m["jitter"] <= sla["jitter"]


if __name__ == "__main__":
    rows = list(csv.DictReader(open(sys.argv[1] if len(sys.argv) > 1 else "measurements.csv", encoding="utf-8")))
    # TODO: group by time, choose per app with hysteresis, print a timeline
    print(len(rows), "measurements")
''', "measurements.csv": '''t,link,latency,loss,jitter
1,mpls,40,0.0,3
1,inet,35,0.2,8
2,mpls,41,0.0,3
2,inet,90,4.5,45
3,mpls,40,0.0,2
3,inet,95,5.0,50
4,mpls,42,0.1,3
4,inet,36,0.1,7
5,mpls,40,0.0,3
5,inet,34,0.0,6'''}),
    P("cissp", "Expert", "access_review", "Joiner-Mover-Leaver Access Review", "Find privilege creep and orphaned accounts by comparing HR data with system access.",
      """Access reviews are one of the most audited controls. Compare an HR export (who works here, in which role) with account exports from three systems, and report: accounts of people who left (orphaned), access that doesn't match the person's current role (privilege creep after a move), shared or generic accounts, and admins without a recent review.""",
      ["identity lifecycle", "least privilege", "role-based access", "audit evidence", "joining data sets"],
      ["Inputs: hr.csv (employee, status, role, department) and accounts.csv (system, user, entitlement, last_login).",
       "Role model: roles.json lists the entitlements each role should have.",
       "Leavers: active accounts of people with status left.",
       "Creep: entitlements not in the person's role model.",
       "Evidence: an output CSV a reviewer can sign off, with a column for their decision."],
      ["The sample data's planted problems (one leaver, one mover, one shared account) are all found."],
      ["What makes access review evidence acceptable to an auditor?",
       "How should I handle service accounts that don't belong to a person?"],
      {"review.py": '''"""Access review: python review.py hr.csv accounts.csv roles.json"""
import csv
import json
import sys


def main(hr_path: str, accounts_path: str, roles_path: str) -> None:
    hr = {r["employee"]: r for r in csv.DictReader(open(hr_path, encoding="utf-8"))}
    accounts = list(csv.DictReader(open(accounts_path, encoding="utf-8")))
    roles = json.load(open(roles_path, encoding="utf-8"))
    for a in accounts:
        person = hr.get(a["user"])
        # TODO: leavers, unknown users, entitlements outside the role
        print(a["system"], a["user"], a["entitlement"], person["status"] if person else "NOT IN HR")


if __name__ == "__main__":
    main(*(sys.argv[1:4] or ["hr.csv", "accounts.csv", "roles.json"]))
''', "hr.csv": '''employee,status,role,department
ann,active,developer,engineering
ken,active,accountant,finance
yui,left,developer,engineering
taro,active,support,it''', "accounts.csv": '''system,user,entitlement,last_login
github,ann,write,2026-10-29
github,yui,write,2026-09-01
erp,ken,ledger-edit,2026-10-28
erp,ken,payroll-admin,2026-10-02
github,ken,write,2026-03-11
helpdesk,taro,agent,2026-10-30
helpdesk,support,agent,2026-10-30''', "roles.json": '''{
  "developer": ["github:write"],
  "accountant": ["erp:ledger-edit"],
  "support": ["helpdesk:agent"]
}'''}),
    P("itil", "Intermediate", "change_calendar", "Change Calendar Conflict Checker", "Spot risky changes: overlaps, freeze periods and missing approvals.",
      """In ITIL change enablement, a change schedule shows planned changes. Write a checker that reads planned changes and flags: two changes to the same service in overlapping windows, changes during a freeze period (year-end, Golden Week), normal changes without CAB approval, and high-risk changes without a rollback plan.""",
      ["change types: standard, normal, emergency", "change schedule", "risk assessment", "datetime ranges"],
      ["Input: changes.csv with id, service, type, risk, start, end, approved, rollback.",
       "Overlaps: same service, overlapping time windows.",
       "Freezes: freeze.json date ranges; only emergency changes may happen then.",
       "Approvals: normal changes need approved=yes; high risk needs a rollback plan.",
       "Output: a list of issues, then the week's schedule as a simple text calendar."],
      ["Every planted issue in the sample is reported once, with the rule it breaks."],
      ["What's the difference between a standard, normal and emergency change?",
       "How do I check two time ranges for overlap?"],
      {"changes.py": '''"""Check a change schedule: python changes.py changes.csv freeze.json"""
import csv
import datetime as dt
import json
import sys


def parse(s: str) -> dt.datetime:
    return dt.datetime.fromisoformat(s)


def overlaps(a_start, a_end, b_start, b_end) -> bool:
    return a_start < b_end and b_start < a_end


if __name__ == "__main__":
    changes = list(csv.DictReader(open(sys.argv[1] if len(sys.argv) > 1 else "changes.csv", encoding="utf-8")))
    freezes = json.load(open(sys.argv[2] if len(sys.argv) > 2 else "freeze.json", encoding="utf-8"))
    # TODO: overlaps, freezes, approvals, rollback plans
    print(len(changes), "changes,", len(freezes), "freeze periods")
''', "changes.csv": '''id,service,type,risk,start,end,approved,rollback
C1,email,normal,low,2026-11-03T22:00,2026-11-03T23:00,yes,yes
C2,email,normal,medium,2026-11-03T22:30,2026-11-04T00:00,yes,yes
C3,erp,normal,high,2026-12-29T20:00,2026-12-29T23:00,yes,no
C4,vpn,standard,low,2026-11-05T12:00,2026-11-05T12:30,no,yes
C5,web,normal,medium,2026-11-06T21:00,2026-11-06T22:00,no,yes''', "freeze.json": '''[
  { "name": "year end", "start": "2026-12-25T00:00", "end": "2027-01-05T00:00" },
  { "name": "Golden Week", "start": "2027-04-29T00:00", "end": "2027-05-06T00:00" }
]'''}),
    P("ipa-fe", "Intermediate", "algorithm_tracer", "Algorithm Trace Trainer", "Practise the FE exam's trace tables by having a program check yours.",
      """The 基本情報技術者 exam's algorithm questions are about tracing: following variables through loops by hand. Implement the classic algorithms (linear and binary search, bubble, selection and insertion sort, stack and queue operations), make each one print a trace table, and use it to check your hand traces of past-exam style questions.""",
      ["trace tables", "searching and sorting", "array indexes (1-based in the exam!)", "stacks and queues"],
      ["Trace helper: a function that prints the variables you choose after every step.",
       "Searches: linear and binary search with traces.",
       "Sorts: bubble, selection and insertion sort, counting comparisons and swaps.",
       "Exam style: write versions with 1-based arrays like the exam's pseudo-code.",
       "Practice: trace three inputs by hand first, then check with the program."],
      ["Your hand traces match the program on at least five inputs."],
      ["Explain how binary search's low and high change on this input step by step.",
       "Why does insertion sort do fewer swaps on nearly sorted data?"],
      {"trace.py": '''"""Algorithm traces for FE practice: python trace.py"""


def trace(step: int, **vars) -> None:
    print(f"{step:>3}: " + "  ".join(f"{k}={v}" for k, v in vars.items()))


def binary_search(a: list[int], target: int) -> int:
    """1-based like the exam: returns the position of target in sorted a, or 0."""
    low, high, step = 1, len(a), 0
    while low <= high:
        mid = (low + high) // 2
        step += 1
        trace(step, low=low, high=high, mid=mid, value=a[mid - 1])
        if a[mid - 1] == target:
            return mid
        if a[mid - 1] < target:
            low = mid + 1
        else:
            high = mid - 1
    return 0


# TODO: linear search, bubble / selection / insertion sort with traces

if __name__ == "__main__":
    print("found at", binary_search([3, 8, 15, 21, 34, 55, 89], 34))
'''}),
    P("japanese", "Novice", "kana_quiz", "Kana and IT Vocabulary Quiz", "A terminal quiz with spaced repetition for kana and IT words.",
      """Build your own study tool: a quiz that shows hiragana, katakana or IT vocabulary (サーバー, 障害, 設定...), checks your answer, and schedules each card again sooner if you got it wrong (a simple Leitner box system), saving your progress between runs.""",
      ["Leitner spaced repetition", "Unicode text", "JSON save files", "random"],
      ["Cards: cards.csv with front, back and deck (hiragana, katakana, it).",
       "Quiz: ask 20 due cards, accept answers ignoring case and spaces.",
       "Boxes: right moves a card up a box, wrong back to box 1; box n is due every 2^n days.",
       "Save: progress.json keeps each card's box and next due date.",
       "Stats: how many cards per box, and your accuracy this week."],
      ["You've used it for a week and the stats show cards moving up the boxes."],
      ["How does the Leitner system decide when to show a card again?",
       "Give me 30 more IT words used in Japanese job ads, with readings."],
      {"quiz.py": '''"""Kana and IT vocabulary quiz: python quiz.py [deck]"""
import csv
import datetime as dt
import json
import random
import sys
from pathlib import Path

PROGRESS = Path("progress.json")


def load_cards(deck: str | None) -> list[dict]:
    cards = list(csv.DictReader(open("cards.csv", encoding="utf-8")))
    return [c for c in cards if deck is None or c["deck"] == deck]


if __name__ == "__main__":
    sys.stdout.reconfigure(encoding="utf-8")
    cards = load_cards(sys.argv[1] if len(sys.argv) > 1 else None)
    random.shuffle(cards)
    # TODO: only due cards, Leitner boxes, save progress
    for c in cards[:10]:
        answer = input(f"{c['front']} = ")
        print("✔" if answer.strip().lower() == c["back"].lower() else f"✘ {c['back']}")
''', "cards.csv": '''front,back,deck
あ,a,hiragana
し,shi,hiragana
つ,tsu,hiragana
ン,n,katakana
ソ,so,katakana
サーバー,server,it
障害,outage,it
設定,settings,it
本番環境,production environment,it
開発,development,it'''}),
    P("japan-career", "Intermediate", "job_tracker", "Japan Job Application Tracker", "Track applications, interviews and offers, and compare take-home pay.",
      """Job hunting in Japan has many steps: 書類選考, several interviews, sometimes an aptitude test (SPI), then 内定. Track every application in a CSV, see your pipeline at a glance, get reminders for follow-ups, and compare offers by estimated monthly take-home pay after tax and social insurance.""",
      ["the Japanese hiring process", "salary structures (bonuses, allowances)", "csv", "simple estimates"],
      ["Pipeline: applications.csv with company, role, stage, last_contact, salary_offer.",
       "Overview: counts per stage, and companies with no contact for 10+ days.",
       "Offers: annual salary → rough monthly take-home (income tax, resident tax, ~15% social insurance).",
       "Compare: offers side by side including bonus months and commuting allowance.",
       "Notes: a template for interview prep per company (自己紹介, 志望動機, questions to ask)."],
      ["Your real applications are in it and the overview tells you what to do next."],
      ["What is the difference between 年収 and 月給 in a Japanese offer?",
       "Help me prepare a 志望動機 for an infrastructure engineer role."],
      {"tracker.py": '''"""Job application tracker: python tracker.py"""
import csv
import datetime as dt
from collections import Counter

STAGES = ["applied", "document screening", "interview 1", "interview 2", "final interview", "offer", "rejected"]


def take_home_monthly(annual_yen: int, bonus_months: float = 2.0) -> int:
    """A rough estimate of monthly take-home pay: about 75-80% of the gross monthly salary."""
    monthly = annual_yen / (12 + bonus_months)
    # TODO: a better estimate with social insurance (~15%), income tax and resident tax
    return round(monthly * 0.78)


if __name__ == "__main__":
    apps = list(csv.DictReader(open("applications.csv", encoding="utf-8")))
    print(Counter(a["stage"] for a in apps))
    # TODO: follow-up reminders, offer comparison
''', "applications.csv": '''company,role,stage,last_contact,salary_offer
Example SIer,Infrastructure engineer,interview 1,2026-10-20,
Example Cloud KK,Cloud support engineer,offer,2026-10-28,5200000
Example Games,SRE,document screening,2026-10-10,'''}),
]
