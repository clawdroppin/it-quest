from gen_projects import P

PROJECTS = [
    P("foundations", "Novice", "pc_build", "PC Build Compatibility Checker", "Check a list of PC parts for compatibility and power needs.",
      """Before you buy parts, check they fit together. Describe a build in build.json (CPU, motherboard, RAM, GPU, PSU, case) and the checker tells you: does the CPU socket match the board, is the RAM the right generation, is the PSU big enough with headroom, does the GPU fit in the case.""",
      ["hardware basics: sockets, RAM generations, form factors, power", "JSON", "Python dicts and if statements"],
      ["Parts: fill parts.json with a few real CPUs, boards, RAM kits, GPUs, PSUs and cases.",
       "Socket and RAM: compare the CPU's socket and RAM type with the board's.",
       "Power: add up typical watts and recommend a PSU with 30% headroom.",
       "Size: check the board's form factor and the GPU length against the case.",
       "Report: a clear list of ✔ and ✘ lines with the reason for each problem."],
      ["A compatible build gets all ✔ and each kind of mistake is explained in plain words."],
      ["What's the difference between ATX, micro-ATX and mini-ITX?",
       "How much PSU headroom is sensible, and why?"],
      {"check_build.py": '''"""Check a PC build for compatibility: python check_build.py build.json"""
import json
import sys


def check(build: dict) -> list[str]:
    """Problems with the build, as plain sentences. Empty means it's compatible."""
    problems = []
    if build["cpu"]["socket"] != build["motherboard"]["socket"]:
        problems.append(f"The {build['cpu']['name']} needs socket {build['cpu']['socket']}, "
                        f"but the board has {build['motherboard']['socket']}.")
    # TODO: RAM type, power with headroom, form factor, GPU length
    return problems


if __name__ == "__main__":
    build = json.load(open(sys.argv[1] if len(sys.argv) > 1 else "build.json", encoding="utf-8"))
    problems = check(build)
    print("\\n".join("✘ " + p for p in problems) or "✔ Everything fits together.")
''', "build.json": '''{
  "cpu": { "name": "Ryzen 5 7600", "socket": "AM5", "ram": "DDR5", "watts": 65 },
  "motherboard": { "name": "B650M board", "socket": "AM5", "ram": "DDR5", "form_factor": "micro-ATX" },
  "ram": { "name": "32 GB DDR5-6000", "type": "DDR5" },
  "gpu": { "name": "RTX 4060", "watts": 115, "length_mm": 240 },
  "psu": { "name": "550 W 80+ Gold", "watts": 550 },
  "case": { "name": "Compact case", "form_factors": ["micro-ATX", "mini-ITX"], "max_gpu_mm": 320 }
}'''}),
    P("aplus", "Intermediate", "ticket_triage", "Help Desk Ticket Triage", "Sort incoming tickets by category and priority, with SLA deadlines.",
      """A help desk gets dozens of emails a day. Read tickets from tickets.csv, guess each one's category from keywords (hardware, network, account, printer, software), set a priority from impact and urgency, compute its SLA deadline, and print today's queue in the order an agent should work it.""",
      ["impact × urgency priority matrix", "SLA deadlines", "keyword matching", "csv and datetime"],
      ["Categories: keyword lists per category in rules.json.",
       "Priority: impact (1 person / team / company) × urgency gives P1–P4.",
       "SLA: P1 4h, P2 8h, P3 2 days, P4 5 days, from the time it was opened.",
       "Queue: sort by deadline, show time left, mark breached tickets.",
       "Accuracy: hand-label 20 tickets and measure how often your categories are right."],
      ["The queue is in a sensible order and most categories are right on your labelled tickets."],
      ["Explain the impact and urgency matrix used in ITIL.",
       "How can I improve keyword matching without machine learning?"],
      {"triage.py": '''"""Triage help desk tickets: python triage.py tickets.csv"""
import csv
import datetime as dt
import json
import sys

RULES = json.load(open("rules.json", encoding="utf-8"))
SLA_HOURS = {"P1": 4, "P2": 8, "P3": 48, "P4": 120}


def category(text: str) -> str:
    """The category whose keywords appear most often in the text, or "other"."""
    # TODO
    return "other"


def priority(impact: str, urgency: str) -> str:
    """impact: person/team/company; urgency: low/medium/high → P1..P4."""
    # TODO
    return "P4"


if __name__ == "__main__":
    for row in csv.DictReader(open(sys.argv[1] if len(sys.argv) > 1 else "tickets.csv", encoding="utf-8")):
        print(category(row["text"]), priority(row["impact"], row["urgency"]), row["text"])
''', "rules.json": '''{
  "hardware": ["screen", "keyboard", "mouse", "laptop", "battery", "fan", "noise"],
  "network": ["wifi", "wi-fi", "internet", "vpn", "slow", "disconnect"],
  "account": ["password", "locked", "login", "mfa", "account"],
  "printer": ["printer", "print", "toner", "paper jam", "scan"],
  "software": ["excel", "outlook", "install", "update", "crash", "error"]
}''', "tickets.csv": '''opened,impact,urgency,text
2026-10-30 08:55,person,high,My laptop screen flickers and goes black
2026-10-30 09:10,company,high,Internet is down for the whole office
2026-10-30 09:12,person,medium,I am locked out after too many password attempts
2026-10-30 09:30,team,medium,The 3rd floor printer says paper jam but there is no paper stuck
2026-10-30 10:05,person,low,Please install Python on my PC
2026-10-30 10:40,team,high,Outlook crashes when opening shared calendar'''}),
    P("networkplus", "Intermediate", "vlsm_planner", "VLSM Subnet Planner", "Split an address block into right-sized subnets for each department.",
      """Given a block like 10.20.0.0/22 and a list of departments with host counts, allocate the smallest subnet that fits each (biggest first, so nothing overlaps), and print a plan with network, mask, first and last usable host, broadcast and spare capacity. Exactly the subnetting the Network+ and CCNA exams test.""",
      ["VLSM", "binary and masks", "Python ipaddress", "allocation order"],
      ["Sizes: for each department, the smallest prefix whose usable hosts ≥ needed hosts.",
       "Allocate: sort largest first, hand out subnets in order from the block.",
       "Report: a table with network, mask, usable range, broadcast and % used.",
       "Errors: say clearly when the block is too small.",
       "Check by hand: do two plans on paper first and compare with the program."],
      ["The plan matches your hand calculations and never overlaps."],
      ["Why allocate the biggest subnets first?",
       "Walk me through finding the broadcast address of 10.20.1.64/27 in binary."],
      {"vlsm.py": '''"""VLSM planner: python vlsm.py 10.20.0.0/22 sales=120 dev=60 hr=25 mgmt=10 p2p=2"""
import ipaddress
import math
import sys


def prefix_for(hosts: int) -> int:
    """The longest prefix (smallest subnet) with at least `hosts` usable addresses."""
    # usable = 2 ** (32 - prefix) - 2
    return 32 - math.ceil(math.log2(hosts + 2))


def plan(block: str, needs: dict[str, int]) -> list[tuple[str, ipaddress.IPv4Network]]:
    """(department, subnet) pairs, biggest first."""
    # TODO
    return []


if __name__ == "__main__":
    needs = dict((k, int(v)) for k, v in (a.split("=") for a in sys.argv[2:]))
    for name, net in plan(sys.argv[1], needs):
        print(f"{name:<8} {net}")
'''}),
    P("linux", "Intermediate", "lamp_lab", "Self-hosted Web Server Lab", "Set up and secure a Linux web server in a VM, documented as you go.",
      """In a VirtualBox, Hyper-V or WSL machine, install a web server, put a site on it, and secure it the way a junior Linux admin would: a non-root sudo user, SSH keys only, a firewall, automatic security updates, log rotation and a backup job. Keep a runbook (RUNBOOK.md) so anyone could rebuild it.""",
      ["users and sudo", "SSH keys and sshd_config", "ufw or firewalld", "nginx", "systemd and timers", "cron or timers for backups"],
      ["VM: install Ubuntu Server or Debian and note the version, IP and resources.",
       "Access: create an admin user, add your SSH key, disable password and root login.",
       "Firewall: allow only SSH and HTTP/HTTPS.",
       "Web: install nginx, serve your HTML project, check logs in /var/log/nginx.",
       "Maintenance: unattended-upgrades, a backup script on a systemd timer, and a check with `systemctl list-timers`."],
      ["You can rebuild the whole server from RUNBOOK.md and check.sh reports everything green."],
      ["Review my sshd_config changes before I restart sshd and lock myself out.",
       "How do I write a systemd timer instead of a cron job?"],
      {"RUNBOOK.md": '''# Web server runbook

## Machine
- OS:
- IP:

## Steps
1. Create the admin user: `sudo adduser admin && sudo usermod -aG sudo admin`
2. ...
''', "check.sh": '''#!/usr/bin/env bash
# Checks the server's security basics. Run on the server: bash check.sh
ok() { printf '\\e[32m✔\\e[0m %s\\n' "$1"; }
bad() { printf '\\e[31m✘\\e[0m %s\\n' "$1"; }

if grep -Eq '^\\s*PasswordAuthentication\\s+no' /etc/ssh/sshd_config /etc/ssh/sshd_config.d/*.conf 2>/dev/null; then
  ok "SSH password login disabled"
else
  bad "SSH password login is still allowed"
fi
# TODO: root login, firewall active, nginx running, unattended-upgrades, backup timer
'''}),
    P("securityplus", "Intermediate", "file_integrity", "File Integrity Monitor", "Detect changed, added and deleted files with SHA-256 baselines.",
      """Integrity monitoring is a core detective control: record a SHA-256 hash of every file in important folders, then later report anything that changed, appeared or disappeared. You'll build a small version of tools like Tripwire or OSSEC's syscheck.""",
      ["hashing (SHA-256)", "baselines", "detective controls", "hashlib and os.walk", "JSON"],
      ["Baseline: `fim.py init FOLDER` writes baseline.json with a hash, size and modified time per file.",
       "Check: `fim.py check FOLDER` reports modified, added and deleted files.",
       "Large files: hash in chunks so a 4 GB file doesn't fill memory.",
       "Ignore list: patterns like `*.log` or `cache/` that are expected to change.",
       "Protect the baseline: store its own hash separately and explain why it matters."],
      ["Changing, adding and deleting test files are each reported correctly, and nothing else is."],
      ["Why is SHA-256 suitable here but MD5 is not?",
       "How could an attacker defeat this, and what would stop them?"],
      {"fim.py": '''"""File integrity monitor: python fim.py init|check FOLDER"""
import hashlib
import json
import sys
from pathlib import Path


def sha256(path: Path) -> str:
    h = hashlib.sha256()
    with path.open("rb") as f:
        for chunk in iter(lambda: f.read(1 << 20), b""):
            h.update(chunk)
    return h.hexdigest()


def snapshot(folder: Path) -> dict[str, str]:
    """Relative path → hash for every file under folder."""
    # TODO
    return {}


def compare(old: dict[str, str], new: dict[str, str]) -> dict[str, list[str]]:
    """{"modified": [...], "added": [...], "deleted": [...]}"""
    # TODO
    return {"modified": [], "added": [], "deleted": []}


if __name__ == "__main__":
    command, folder = sys.argv[1], Path(sys.argv[2])
    if command == "init":
        Path("baseline.json").write_text(json.dumps(snapshot(folder), indent=2))
    else:
        print(compare(json.loads(Path("baseline.json").read_text()), snapshot(folder)))
'''}),
]
