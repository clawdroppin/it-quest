@@lesson f3-automation Automation and Scripting for IT
@@text
Every exercise in this track has been a small Python program, and that's on purpose: scripting is what separates IT staff who spend all day clicking from those who solve a task once and let the computer repeat it. You don't need to become a software engineer; you need to recognise tasks worth automating and know enough to write, read and safely run a script.

## What's worth automating

A rough rule: if you do it **more than three times**, or it must be done **exactly the same way every time**, or it touches **many machines or users**, script it.

| Manual task | Script |
|---|---|
| Create a new starter's account, groups, mailbox, licence | read HR's CSV, create everything, email the manager |
| Check disk space on 40 servers each morning | collect it, flag anything over 85%, send a report |
| Find who has access to a share | export permissions to a spreadsheet |
| Clear old files from a temp folder | delete files older than 30 days, log what was removed |
| Install the same five apps on new laptops | a package list deployed automatically |

Automation saves time, but the bigger wins are **consistency** (no forgotten step) and **documentation** (the script *is* the procedure).

## Choosing a language

- **PowerShell**: Windows, Active Directory, Microsoft 365, Azure, Exchange, Intune. The default for Windows admins.
- **Bash**: Linux servers, quick command chains, CI pipelines.
- **Python**: cross-platform, data handling (CSV, JSON, Excel), APIs, network automation (Netmiko, NAPALM), cloud SDKs. The most versatile.
- Configuration management and infrastructure-as-code tools build on these: **Ansible** (YAML playbooks), **Terraform**, Intune/GPO for desktops.

## The building blocks (in any language)

Every script is made from the same few pieces, which you've been using all along:

- **Variables**: named values (`threshold = 85`).
- **Data structures**: lists of items, dictionaries of name → value.
- **Conditions**: `if usage > threshold:`.
- **Loops**: `for server in servers:`.
- **Functions**: named, reusable steps with inputs and outputs.
- **Input/output**: reading files, CSVs, APIs; writing reports and logs.
- **Error handling**: `try`/`except` (Python), `try`/`catch` (PowerShell), checking exit codes (Bash), so one unreachable server doesn't stop the run.

## APIs: automating cloud services

Modern services expose **REST APIs**: you send HTTPS requests with JSON and get JSON back. Microsoft Graph (Microsoft 365), the AWS/Azure/Google APIs, ServiceNow, Slack, GitHub: almost everything is scriptable. You authenticate with an **API key** or **OAuth token**, which are secrets: never put them in scripts or Git; use a secrets vault, environment variables or managed identities.

## Writing scripts safely

- **Dry run first**: print what would change (`-WhatIf` in PowerShell, a `--dry-run` flag in your own scripts), then run it for real.
- **Start small**: one test user or machine, then a pilot group, then everyone.
- **Log** what the script did, with timestamps.
- **Least privilege**: run with an account that can do only what the script needs.
- **Idempotent** scripts can run twice safely ("ensure the user is in the group", not "add the user to the group" and fail the second time).
- **Version control**: keep scripts in Git with a README; a script on one admin's desktop is a single point of failure.
- **Never run a script you don't understand**, especially one from the internet or an email.

## Scheduling

- Windows **Task Scheduler** (or `Register-ScheduledTask` in PowerShell).
- Linux **cron** (`0 7 * * 1-5 /opt/scripts/disk_report.sh` = 07:00 on weekdays) or systemd timers.
- Cloud schedulers: Azure Automation, AWS EventBridge + Lambda, GitHub Actions on a schedule.

A scheduled script needs monitoring too: if it fails silently, nobody notices until the problem it prevented happens. Send an alert when a job fails (and, ideally, when it *didn't run*).

## Where to go next in this app

The **Python**, **PowerShell** and **Bash** tracks teach scripting properly, from "hello world" to tested, packaged tools. A good first goal: automate one boring task in your own job or home lab, then put it on GitHub with a README.

@@quiz
Which is the strongest reason to automate a new-starter process?
- It's more fun
* Every account is set up the same way, with no forgotten steps, and the script documents the process
- Scripts never have bugs
- It removes the need for HR
? Consistency and documentation, plus time.

@@quiz
Where should an API key used by a scheduled script be stored?
- In the script, so it's easy to find
- In the Git repository's README
* In a secrets vault, environment variable or managed identity, never in code
- In an email to yourself
? Secrets in code get leaked.

@@quiz
What does "idempotent" mean for a script?
* Running it again leaves the same end state without errors or duplicate changes
- It runs faster the second time
- It can only run once
- It runs on every OS
? "Ensure X" instead of "do X".

@@quiz
What does the cron schedule `0 7 * * 1-5` mean?
* 07:00 every Monday to Friday
- Every 7 minutes
- At midnight on the 7th of each month
- 01:00–05:00 every day
? Minute, hour, day of month, month, day of week.

@@flashcards
rule of three :: automate tasks you repeat
PowerShell :: Windows/Microsoft automation
Bash :: Linux shell scripting
Python :: cross-platform scripting, data and APIs
Ansible :: configuration management with YAML playbooks
REST API :: HTTPS + JSON interface to a service
API key / OAuth token :: secrets for API access
dry run :: show what would change without changing it
idempotent :: safe to run repeatedly
Task Scheduler :: Windows job scheduling
cron :: Linux job scheduling
cron fields :: minute hour day-of-month month day-of-week

@@drill
In a cron line, which field comes first?
= minute
? minute hour day month weekday.

@@drill
What PowerShell parameter shows what a command would do without doing it?
= -WhatIf
= whatif
? A dry run.

@@exercise A morning disk report with a dry-run cleanup
For each server, compute the percentage used (one decimal) and print a report with `WARN` at 85% or more and `CRIT` at 95% or more. Then write `cleanup(files, days, dry_run=True)` that selects temp files older than `days` and either prints `would delete` (dry run) or `deleted` for each, returning the space freed in MB. Run the cleanup as a dry run first, then for real.
@@lang python
@@starter
servers = {"fs01": (1800, 2000), "web01": (120, 500), "db01": (930, 1000), "app01": (420, 500)}  # used GB, size GB
temp_files = [("report_old.tmp", 45, 120), ("cache_a.tmp", 2, 300), ("dump.tmp", 90, 2048), ("upload.tmp", 31, 15)]  # name, age days, MB

def cleanup(files, days, dry_run=True):
    raise NotImplementedError
@@expected
fs01: 90.0% WARN
web01: 24.0% OK
db01: 93.0% WARN
app01: 84.0% OK
Dry run:
  would delete report_old.tmp (45 days, 120 MB)
  would delete dump.tmp (90 days, 2048 MB)
  would delete upload.tmp (31 days, 15 MB)
Would free 2183 MB
Real run:
  deleted report_old.tmp (45 days, 120 MB)
  deleted dump.tmp (90 days, 2048 MB)
  deleted upload.tmp (31 days, 15 MB)
Freed 2183 MB
@@requires
def cleanup
dry_run
>= 95
@@hint
Status: `"CRIT" if pct >= 95 else "WARN" if pct >= 85 else "OK"`. In `cleanup`, loop over files with `age > days`, print the right verb, and add up the MB.
@@solution
servers = {"fs01": (1800, 2000), "web01": (120, 500), "db01": (930, 1000), "app01": (420, 500)}
temp_files = [("report_old.tmp", 45, 120), ("cache_a.tmp", 2, 300), ("dump.tmp", 90, 2048), ("upload.tmp", 31, 15)]

for name, (used, size) in servers.items():
    pct = used / size * 100
    status = "CRIT" if pct >= 95 else "WARN" if pct >= 85 else "OK"
    print(f"{name}: {pct:.1f}% {status}")

def cleanup(files, days, dry_run=True):
    freed = 0
    for name, age, mb in files:
        if age > days:
            print(f"  {'would delete' if dry_run else 'deleted'} {name} ({age} days, {mb} MB)")
            freed += mb
    return freed

print("Dry run:")
print("Would free", cleanup(temp_files, 30), "MB")
print("Real run:")
print("Freed", cleanup(temp_files, 30, dry_run=False), "MB")

@@exercise New starters from an HR CSV, idempotently
Read the HR CSV and "create" accounts in a dictionary `directory`: the username is the first letter of the given name plus the family name, lower-case, with non-letters removed (`Ana Silva` → `asilva`); if it's taken by **someone else**, add 2, 3, ... Each user gets the groups for their department from `DEPARTMENT_GROUPS`. Running the import **twice** must not create duplicates: if a person with the same employee ID already exists, print `exists` instead. Print what happens on both runs and the final directory.
@@lang python
@@starter
import csv, io

hr_csv = """employee_id,given,family,department
E1001,Ana,Silva,Finance
E1002,Kenji,Sato,Sales
E1003,Aisha,Silva,Finance
E1004,Anna,Silva,Sales
"""
DEPARTMENT_GROUPS = {"Finance": ["GG-All-Staff", "GG-Finance-Staff"], "Sales": ["GG-All-Staff", "GG-Sales-Staff"]}
directory = {}   # username -> {"id": ..., "name": ..., "groups": [...]}

def import_starters(text):
    raise NotImplementedError

print("First run:")
import_starters(hr_csv)
print("Second run:")
import_starters(hr_csv)
@@expected
First run:
  created asilva for Ana Silva
  created ksato for Kenji Sato
  created asilva2 for Aisha Silva
  created asilva3 for Anna Silva
Second run:
  E1001 exists
  E1002 exists
  E1003 exists
  E1004 exists
Directory:
  asilva: E1001 Ana Silva GG-All-Staff, GG-Finance-Staff
  ksato: E1002 Kenji Sato GG-All-Staff, GG-Sales-Staff
  asilva2: E1003 Aisha Silva GG-All-Staff, GG-Finance-Staff
  asilva3: E1004 Anna Silva GG-All-Staff, GG-Sales-Staff
@@requires
def import_starters
csv.DictReader(
isalpha()
@@hint
Check for an existing employee ID with `any(u["id"] == row["employee_id"] for u in directory.values())`. Build the base name with `"".join(ch for ch in (given[0] + family).lower() if ch.isalpha())`, then loop with a counter while the name is taken.
@@solution
import csv, io

hr_csv = """employee_id,given,family,department
E1001,Ana,Silva,Finance
E1002,Kenji,Sato,Sales
E1003,Aisha,Silva,Finance
E1004,Anna,Silva,Sales
"""
DEPARTMENT_GROUPS = {"Finance": ["GG-All-Staff", "GG-Finance-Staff"], "Sales": ["GG-All-Staff", "GG-Sales-Staff"]}
directory = {}

def import_starters(text):
    for row in csv.DictReader(io.StringIO(text)):
        if any(u["id"] == row["employee_id"] for u in directory.values()):
            print(f"  {row['employee_id']} exists")
            continue
        base = "".join(ch for ch in (row["given"][0] + row["family"]).lower() if ch.isalpha())
        username, n = base, 1
        while username in directory:
            n += 1
            username = f"{base}{n}"
        directory[username] = {
            "id": row["employee_id"],
            "name": f"{row['given']} {row['family']}",
            "groups": DEPARTMENT_GROUPS[row["department"]],
        }
        print(f"  created {username} for {row['given']} {row['family']}")

print("First run:")
import_starters(hr_csv)
print("Second run:")
import_starters(hr_csv)
print("Directory:")
for username, user in directory.items():
    print(f"  {username}: {user['id']} {user['name']} {', '.join(user['groups'])}")
