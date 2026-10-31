from gen_projects import P


def py(doc, body):
    return f'"""{doc}"""\n{body.strip()}\n'


PROJECTS = [
    P("foundations", "Novice", "home_lab", "Home Lab Inventory and Network Map", "Document your own computers and home network like an IT admin.",
      """Treat your home as a tiny company: record every device (PC, phone, router, TV), its IP and MAC address, OS and owner, draw the network, and write a script that collects your own PC's details automatically. Documentation is the habit that separates good IT staff from the rest.""",
      ["IP and MAC addresses", "DHCP and the router", "documentation", "Python platform and socket modules"],
      ["Collect: find each device's IP and MAC (router admin page, `ipconfig /all`, phone settings).",
       "Record: devices.csv with name, type, IP, MAC, OS, owner, notes.",
       "Script: collect.py prints this PC's hostname, OS, CPU, RAM and IP addresses.",
       "Map: draw the network (router, Wi-Fi, devices) in NETWORK.md with a simple text diagram.",
       "Review: which devices haven't had updates in a year? Note them."],
      ["Someone else could understand your home network from your files alone."],
      ["How do I find which device has an unknown IP on my network?",
       "What's the difference between a private and a public IP address?"],
      {"collect.py": py("Print this computer's basic details for the inventory.", '''
import platform
import socket

print("hostname:", socket.gethostname())
print("os:", platform.system(), platform.release())
print("cpu:", platform.processor() or platform.machine())
# TODO: RAM (hint: on Windows use the wmic or PowerShell output; on Linux read /proc/meminfo), IP addresses
'''), "devices.csv": "name,type,ip,mac,os,owner,notes\nrouter,router,192.168.1.1,,,family,ISP-provided"}),
    P("aplus", "Intermediate", "pc_refresh", "PC Refresh Plan", "Plan replacing ten old office PCs: specs, data migration and disposal.",
      """A small office has ten PCs that are seven years old. Plan their replacement like an IT technician would: inventory the old machines, choose new specs, plan user data migration, schedule the swaps to minimize downtime, and dispose of the old drives securely. The deliverable is a plan document plus a checklist script.""",
      ["hardware specs", "Windows deployment and user migration", "secure data destruction", "project checklists"],
      ["Inventory: old-pcs.csv with CPU, RAM, disk, Windows version and user.",
       "Specs: choose a standard new PC and justify CPU, RAM, SSD and warranty.",
       "Migration: list what to move per user (Documents, browser bookmarks, Outlook signatures, printers).",
       "Schedule: two PCs per evening, with a rollback plan if a migration fails.",
       "Disposal: wipe or destroy drives and record certificates of destruction."],
      ["PLAN.md covers all five parts, and checklist.py prints a per-user checklist from the CSV."],
      ["Which Windows 10 PCs can't be upgraded to Windows 11, and why (TPM 2.0, CPU)?",
       "What's the safest way to wipe an SSD before disposal?"],
      {"checklist.py": py("Print a migration checklist for each user: python checklist.py old-pcs.csv", '''
import csv
import sys

STEPS = ["back up Documents and Desktop", "export browser bookmarks", "note mapped drives and printers",
         "copy Outlook signatures", "sign in on the new PC", "restore data and check with the user"]

for pc in csv.DictReader(open(sys.argv[1] if len(sys.argv) > 1 else "old-pcs.csv", encoding="utf-8")):
    print(f"## {pc['user']} ({pc['name']})")
    for s in STEPS:
        print(f"- [ ] {s}")
    # TODO: flag PCs that need extra steps (old Windows, small disk)
'''), "old-pcs.csv": "name,user,cpu,ram_gb,disk_gb,windows\npc-01,ann,i5-7500,8,256,10\npc-02,ken,i3-7100,4,500,10\npc-03,yui,i5-8400,8,256,11"}),
    P("networkplus", "Advanced", "wifi_survey", "Wi-Fi Channel Survey", "Parse nearby networks and recommend the least crowded channels.",
      """Use your laptop to survey Wi-Fi: on Windows `netsh wlan show networks mode=bssid`, on Linux `nmcli -f SSID,CHAN,SIGNAL dev wifi`. Parse the output, count networks per channel weighted by signal strength, account for 2.4 GHz channel overlap, and recommend the best channels for 2.4 and 5 GHz.""",
      ["2.4 vs 5 GHz", "channel overlap (1, 6, 11)", "signal strength (dBm / %)", "parsing command output"],
      ["Capture: save the command's output to survey.txt (a sample is included).",
       "Parse: SSID, BSSID, channel and signal for each access point.",
       "Weight: stronger neighbours count more; 2.4 GHz channels within 4 of each other overlap.",
       "Recommend: the best of 1/6/11, and the best 5 GHz channel.",
       "Walk test: survey from three rooms and compare."],
      ["The recommendation for the sample is 11 (2.4 GHz), and you understand why."],
      ["Why are only channels 1, 6 and 11 recommended in 2.4 GHz?",
       "How do signal percentage and dBm relate?"],
      {"survey.py": py("Recommend Wi-Fi channels from netsh output: python survey.py survey.txt", '''
import re
import sys

text = open(sys.argv[1] if len(sys.argv) > 1 else "survey.txt", encoding="utf-8").read()
aps = []
for m in re.finditer(r"Signal\\s*:\\s*(\\d+)%.*?Channel\\s*:\\s*(\\d+)", text, re.S):
    aps.append({"signal": int(m.group(1)), "channel": int(m.group(2))})
print(len(aps), "access points")
# TODO: weight per channel with overlap, recommend 2.4 and 5 GHz channels
'''), "survey.txt": '''SSID 1 : HomeNet
    BSSID 1                 : aa:bb:cc:00:00:01
         Signal             : 82%
         Channel            : 1
SSID 2 : Neighbour-2G
    BSSID 1                 : aa:bb:cc:00:00:02
         Signal             : 60%
         Channel            : 6
SSID 3 : Cafe
    BSSID 1                 : aa:bb:cc:00:00:03
         Signal             : 40%
         Channel            : 3
SSID 4 : Office-5G
    BSSID 1                 : aa:bb:cc:00:00:04
         Signal             : 70%
         Channel            : 36'''}),
    P("linux", "Advanced", "log_rotation", "Log Rotation and Disk Alerting", "Write logrotate rules and a disk-space alert that emails or logs.",
      """Full disks take servers down. Set up logrotate for an application that writes big logs, then write a script, run by a systemd timer, that checks every filesystem, writes a warning to the journal at 80% and a critical message at 90%, and lists the largest directories so the on-call person knows where to look.""",
      ["logrotate", "df and du", "systemd service and timer units", "logger and journalctl"],
      ["App: a script that writes lots of log lines to /var/log/myapp/app.log.",
       "logrotate: daily, keep 7, compress, copytruncate or a postrotate signal.",
       "Check: diskcheck.sh reads `df -P`, logs with `logger -p user.warning`.",
       "Largest: `du -xh --max-depth=2 / | sort -h | tail` for filesystems over the limit.",
       "Schedule: diskcheck.service and diskcheck.timer every 15 minutes."],
      ["`logrotate -d` shows your rules working and `journalctl -t diskcheck` shows alerts after you fill a test disk."],
      ["copytruncate or postrotate: which should I use, and what can go wrong?",
       "Review my systemd timer unit."],
      {"diskcheck.sh": '''#!/usr/bin/env bash
# Log a warning for filesystems above the thresholds.
set -euo pipefail
warn=80; crit=90
df -P -x tmpfs -x devtmpfs | awk 'NR > 1 { gsub("%", "", $5); print $5, $6 }' | while read -r use mount; do
  if (( use >= crit )); then
    logger -t diskcheck -p user.crit "$mount is ${use}% full"
  elif (( use >= warn )); then
    logger -t diskcheck -p user.warning "$mount is ${use}% full"
  fi
done
# TODO: list the largest directories on full filesystems
''', "myapp.logrotate": '''/var/log/myapp/*.log {
    daily
    rotate 7
    compress
    missingok
    notifempty
    # TODO: copytruncate or postrotate
}
''', "diskcheck.timer": '''[Unit]
Description=Check disk space every 15 minutes

[Timer]
OnCalendar=*:0/15
Persistent=true

[Install]
WantedBy=timers.target
'''}),
    P("securityplus", "Advanced", "email_headers", "Suspicious Email Header Analyzer", "Read an email's headers and explain the warning signs to a user.",
      """Users forward suspicious emails to IT every day. Write a tool that reads a saved .eml file and reports the defensive checks an analyst makes: SPF, DKIM and DMARC results from Authentication-Results, a From domain that differs from Reply-To or Return-Path, link text that doesn't match the real link target, and attachments with risky extensions. Then write the friendly reply you'd send the user.""",
      ["SPF, DKIM, DMARC", "email headers", "Python email package", "security awareness"],
      ["Parse: use `email.message_from_binary_file` with `policy.default`.",
       "Authentication: extract spf=, dkim= and dmarc= results.",
       "Mismatches: compare the domains of From, Reply-To and Return-Path.",
       "Links: find `<a href>` tags whose visible text shows a different domain.",
       "Verdict: a short risk summary and a template reply to the user who reported it."],
      ["The sample email is flagged for its failed DMARC, mismatched Reply-To and misleading link."],
      ["Explain how SPF, DKIM and DMARC work together.",
       "How should IT reply to a user who reported a phishing email, to encourage reporting?"],
      {"analyze.py": py("Analyze a saved email: python analyze.py sample.eml", '''
import re
import sys
from email import message_from_binary_file, policy
from email.utils import parseaddr


def domain(address: str) -> str:
    return parseaddr(address)[1].rpartition("@")[2].lower()


msg = message_from_binary_file(open(sys.argv[1] if len(sys.argv) > 1 else "sample.eml", "rb"), policy=policy.default)
auth = msg.get("Authentication-Results", "")
print("From:", msg["From"], "| Reply-To:", msg.get("Reply-To"))
print("results:", re.findall(r"(spf|dkim|dmarc)=(\\w+)", auth))
# TODO: domain mismatches, misleading links in the HTML part, risky attachments, a verdict
'''), "sample.eml": '''Authentication-Results: mx.example.com; spf=softfail smtp.mailfrom=billing-notice.example.net; dkim=none; dmarc=fail header.from=example-bank.co.jp
From: "Example Bank" <support@example-bank.co.jp>
Reply-To: help@billing-notice.example.net
Return-Path: <bounce@billing-notice.example.net>
Subject: Your account will be suspended
MIME-Version: 1.0
Content-Type: text/html; charset=utf-8

<p>Please confirm your details within 24 hours:</p>
<p><a href="http://billing-notice.example.net/confirm">https://www.example-bank.co.jp/login</a></p>
'''}),
    P("aws", "Advanced", "iam_policy_lint", "IAM Policy Linter", "Find wildcards and risky permissions in AWS IAM policy documents.",
      """Least privilege is the most tested idea in AWS security. Write a linter for IAM policy JSON: flag `"Action": "*"`, `"Resource": "*"` on write actions, `iam:PassRole` without a resource restriction, `NotAction` with Allow, and missing conditions on sensitive actions, then suggest a tighter policy.""",
      ["IAM policy grammar", "least privilege", "JSON", "severity rules"],
      ["Parse: handle Statement as an object or a list, Action as a string or a list.",
       "Rules: the five checks above, each with a severity and an explanation.",
       "Wildcards: `s3:*` and `s3:Get*` count as broad too.",
       "Suggest: for each finding, what the policy should say instead.",
       "Try it: lint the AWS managed policies you use, exported with the CLI."],
      ["Every planted problem in the sample policy is found and explained."],
      ["Why is iam:PassRole on Resource * dangerous?",
       "How would I write this policy so the Lambda can only write to one table?"],
      {"lint.py": py("Lint an IAM policy: python lint.py policy.json", '''
import json
import sys


def as_list(x):
    return x if isinstance(x, list) else [x]


policy = json.load(open(sys.argv[1] if len(sys.argv) > 1 else "policy.json", encoding="utf-8"))
for i, st in enumerate(as_list(policy["Statement"])):
    actions = as_list(st.get("Action", []))
    resources = as_list(st.get("Resource", []))
    if "*" in actions:
        print(f"statement {i}: HIGH Action is *")
    # TODO: Resource * on write actions, PassRole, NotAction with Allow, missing conditions
'''), "policy.json": '''{
  "Version": "2012-10-17",
  "Statement": [
    { "Effect": "Allow", "Action": "s3:*", "Resource": "*" },
    { "Effect": "Allow", "Action": ["iam:PassRole", "lambda:CreateFunction"], "Resource": "*" },
    { "Effect": "Allow", "Action": "dynamodb:PutItem", "Resource": "arn:aws:dynamodb:ap-northeast-1:123456789012:table/requests" }
  ]
}'''}),
    P("azure", "Advanced", "rightsizing", "VM Rightsizing Advisor", "Recommend smaller (or bigger) VM sizes from CPU and memory metrics.",
      """VMs sized "just in case" waste money. Read 30 days of CPU and memory metrics per VM (export from Azure Monitor, or use the sample), compute the 95th percentiles, and recommend a size from a small catalog with the monthly saving, never recommending a size that would be over 80% at the 95th percentile.""",
      ["Azure VM sizes and pricing", "percentiles", "cost optimization", "Azure Advisor concepts"],
      ["Catalog: sizes.json with vCPUs, memory and a monthly price.",
       "Metrics: metrics.csv with vm, time, cpu_percent, mem_used_gb.",
       "P95: CPU and memory 95th percentile per VM.",
       "Recommend: the cheapest size with headroom; show current vs recommended cost.",
       "Caveats: list cases where the recommendation is risky (batch jobs, month-end peaks)."],
      ["Recommendations make sense for the sample, with total monthly savings."],
      ["Why use the 95th percentile and not the average?",
       "How do B-series burstable VMs change this analysis?"],
      {"rightsize.py": py("VM rightsizing: python rightsize.py metrics.csv sizes.json", '''
import csv
import json
import math
import sys
from collections import defaultdict


def p95(values: list[float]) -> float:
    s = sorted(values)
    return s[max(0, math.ceil(0.95 * len(s)) - 1)]


rows = list(csv.DictReader(open(sys.argv[1] if len(sys.argv) > 1 else "metrics.csv", encoding="utf-8")))
sizes = json.load(open(sys.argv[2] if len(sys.argv) > 2 else "sizes.json", encoding="utf-8"))
cpu = defaultdict(list)
for r in rows:
    cpu[r["vm"]].append(float(r["cpu_percent"]))
for vm, values in cpu.items():
    print(vm, "cpu p95", p95(values))
# TODO: memory, current size, recommendation and savings
'''), "sizes.json": '''{
  "Standard_B2s":  { "vcpu": 2, "mem_gb": 4,  "monthly_usd": 30 },
  "Standard_D2s_v5": { "vcpu": 2, "mem_gb": 8,  "monthly_usd": 70 },
  "Standard_D4s_v5": { "vcpu": 4, "mem_gb": 16, "monthly_usd": 140 },
  "Standard_D8s_v5": { "vcpu": 8, "mem_gb": 32, "monthly_usd": 280 }
}''', "metrics.csv": "vm,size,time,cpu_percent,mem_used_gb\nweb-01,Standard_D8s_v5,2026-10-01T00:00,12,6.1\nweb-01,Standard_D8s_v5,2026-10-01T01:00,18,6.4\nweb-01,Standard_D8s_v5,2026-10-01T02:00,9,6.0\ndb-01,Standard_D4s_v5,2026-10-01T00:00,71,13.2\ndb-01,Standard_D4s_v5,2026-10-01T01:00,85,14.0"}),
    P("cloud", "Advanced", "cloud_compare", "Cloud Provider Cost Comparison", "Price the same small architecture on AWS, Azure and Google Cloud.",
      """Design one small architecture (two web VMs behind a load balancer, a managed PostgreSQL database, 500 GB of object storage, 1 TB of internet egress a month) and price it on all three clouds in the Tokyo region using their pricing calculators. Put the numbers into a script that compares on-demand, 1-year and 3-year commitments.""",
      ["equivalent services across clouds", "pricing models", "egress costs", "commitments and reservations"],
      ["Map: the equivalent service on each cloud (EC2/VM/Compute Engine, RDS/Azure Database/Cloud SQL...).",
       "Price: use each official calculator for the Tokyo region and record the numbers in prices.json.",
       "Compare: monthly totals per cloud, per pricing model.",
       "Sensitivity: how the ranking changes if egress triples.",
       "Recommend: a one-page summary with the non-price factors too (skills, support, Japanese data residency)."],
      ["COMPARE.md has a table for all three clouds and a recommendation you can defend."],
      ["Which services are equivalent for a managed PostgreSQL on each cloud?",
       "Why is egress often the surprise cost?"],
      {"compare.py": py("Compare cloud costs from prices.json", '''
import json

prices = json.load(open("prices.json", encoding="utf-8"))
for cloud, items in prices.items():
    total = sum(items.values())
    print(f"{cloud:<6} ${total:>8,.2f} / month (on demand)")
# TODO: 1-year and 3-year commitments, egress sensitivity
'''), "prices.json": '''{
  "aws":   { "vms": 0, "load_balancer": 0, "database": 0, "storage": 0, "egress": 0 },
  "azure": { "vms": 0, "load_balancer": 0, "database": 0, "storage": 0, "egress": 0 },
  "gcp":   { "vms": 0, "load_balancer": 0, "database": 0, "storage": 0, "egress": 0 }
}'''}),
    P("gcp", "Advanced", "sla_calculator", "Architecture Availability Calculator", "Compute the composite SLA of a design with series and parallel parts.",
      """Architects are asked "what availability will this design give us?" Describe an architecture as a tree of components in series and in parallel (zones, regions), with each Google Cloud SLA, and compute the composite availability and the allowed downtime per month. Then improve the design until it meets a 99.95% target.""",
      ["composite SLAs", "series and parallel availability", "zonal, regional and multi-regional services", "recursion"],
      ["Model: design.json with nested `series` and `parallel` lists and leaf components with SLAs.",
       "Compute: series multiplies; parallel is 1 − product of (1 − A).",
       "Downtime: availability → minutes per month and hours per year.",
       "Improve: add a second region or a regional database and compare.",
       "Explain: in DESIGN.md, which component limits availability most."],
      ["The calculator gives the right numbers for hand-checked designs and your final design meets 99.95%."],
      ["Why can't two services in series have higher availability than the weaker one?",
       "Which Google Cloud services are zonal, regional or multi-regional?"],
      {"sla.py": py("Composite availability: python sla.py design.json", '''
import json
import sys


def availability(node) -> float:
    if "sla" in node:
        return node["sla"]
    if "series" in node:
        a = 1.0
        for child in node["series"]:
            a *= availability(child)
        return a
    # TODO: parallel
    raise ValueError(node)


design = json.load(open(sys.argv[1] if len(sys.argv) > 1 else "design.json", encoding="utf-8"))
a = availability(design)
print(f"{a:.5%}  → {(1 - a) * 30 * 24 * 60:.1f} minutes of downtime per month")
'''), "design.json": '''{
  "series": [
    { "name": "global load balancer", "sla": 0.9999 },
    { "name": "Cloud Run (one region)", "sla": 0.9995 },
    { "name": "Cloud SQL HA", "sla": 0.9995 }
  ]
}'''}),
    P("terraform", "Intermediate", "tag_checker", "Tag Policy Checker for .tf Files", "Scan Terraform code for resources without required tags.",
      """Before a plan even runs, you can catch problems in the code. Scan all `.tf` files in a folder, find every `resource` block, and report the ones that have no `tags` (or are missing required keys like Owner and CostCenter), skipping resource types that don't support tags. A simple version of what tflint and Checkov do.""",
      ["HCL block structure", "regular expressions and brace matching", "tagging policies", "CI checks"],
      ["Find: each `resource \"TYPE\" \"NAME\" {` and its body (count braces).",
       "Tags: look for `tags = {` or `tags = merge(` in the body.",
       "Required keys: Owner and CostCenter, unless tags come from `default_tags` in the provider.",
       "Skip list: types that don't take tags (aws_iam_role_policy_attachment, aws_route...).",
       "CI: exit 1 when anything is missing, and add it to a pipeline."],
      ["The checker finds the two untagged resources in the sample and nothing else."],
      ["How do provider default_tags in AWS affect this check?",
       "Why is a regex-based HCL parser fragile, and what would be better?"],
      {"check_tags.py": py("Find resources without required tags: python check_tags.py FOLDER", '''
import re
import sys
from pathlib import Path

REQUIRED = {"Owner", "CostCenter"}
START = re.compile(r'resource\\s+"(\\w+)"\\s+"(\\w+)"\\s*\\{')


def blocks(text: str):
    """(type, name, body) for each resource block, by counting braces."""
    for m in START.finditer(text):
        depth, i = 1, m.end()
        while depth and i < len(text):
            depth += {"{": 1, "}": -1}.get(text[i], 0)
            i += 1
        yield m.group(1), m.group(2), text[m.end():i - 1]


folder = Path(sys.argv[1] if len(sys.argv) > 1 else ".")
for tf in folder.glob("*.tf"):
    for rtype, name, body in blocks(tf.read_text(encoding="utf-8")):
        print(rtype, name, "has tags" if "tags" in body else "NO TAGS")
# TODO: required keys, skip list, exit code
'''), "main.tf": '''resource "aws_s3_bucket" "logs" {
  bucket = "acme-logs"
  tags = {
    Owner      = "ops"
    CostCenter = "4100"
  }
}

resource "aws_instance" "web" {
  ami           = "ami-123"
  instance_type = "t3.micro"
}

resource "aws_sqs_queue" "jobs" {
  name = "jobs"
  tags = { Owner = "dev" }
}
'''}),
    P("cka", "Expert", "rbac_matrix", "RBAC Permission Matrix", "Turn Roles and RoleBindings into a who-can-do-what table.",
      """`kubectl auth can-i` answers one question at a time. Export the cluster's Roles, ClusterRoles and bindings as JSON and build the whole matrix: for each user, group and ServiceAccount, which verbs on which resources in which namespaces. Then highlight dangerous grants: `*` verbs, secrets access, and cluster-admin bindings.""",
      ["Role, ClusterRole, RoleBinding, ClusterRoleBinding", "aggregation of rules", "JSON from kubectl", "least privilege"],
      ["Export: `kubectl get roles,clusterroles,rolebindings,clusterrolebindings -A -o json > rbac.json`.",
       "Resolve: for each binding, find its role's rules.",
       "Matrix: subject → namespace → resource → verbs.",
       "Danger: wildcards, `get/list secrets`, `create pods/exec`, cluster-admin.",
       "Check: compare a few cells with `kubectl auth can-i --as`."],
      ["The matrix agrees with `kubectl auth can-i` on the cells you checked."],
      ["Why is `create pods/exec` as powerful as reading secrets?",
       "How does a ClusterRole bound with a RoleBinding behave?"],
      {"rbac.py": py("RBAC matrix: python rbac.py rbac.json", '''
import json
import sys

data = json.load(open(sys.argv[1] if len(sys.argv) > 1 else "rbac.json", encoding="utf-8"))
roles = {}
for item in data["items"]:
    if item["kind"] in ("Role", "ClusterRole"):
        key = (item["kind"], item["metadata"].get("namespace"), item["metadata"]["name"])
        roles[key] = item.get("rules", [])
print(len(roles), "roles")
# TODO: resolve bindings to rules, build the matrix, flag dangerous grants
'''), "rbac.json": '''{ "items": [
  { "kind": "Role", "metadata": { "name": "pod-reader", "namespace": "shop" },
    "rules": [ { "apiGroups": [""], "resources": ["pods"], "verbs": ["get", "list", "watch"] } ] },
  { "kind": "RoleBinding", "metadata": { "name": "ann-reads", "namespace": "shop" },
    "roleRef": { "kind": "Role", "name": "pod-reader" }, "subjects": [ { "kind": "User", "name": "ann" } ] },
  { "kind": "ClusterRoleBinding", "metadata": { "name": "ci-admin" },
    "roleRef": { "kind": "ClusterRole", "name": "cluster-admin" }, "subjects": [ { "kind": "ServiceAccount", "name": "ci", "namespace": "build" } ] }
] }'''}),
    P("ccna", "Advanced", "config_generator", "Branch Router Config Generator", "Generate consistent configs for many sites from one template.",
      """Typing the same config on twenty branch routers causes typos. Describe each site in sites.csv (name, site number, WAN IP, LAN subnet) and generate complete, consistent IOS configs from a template: hostname, interfaces, DHCP pool, OSPF, SSH-only management, NTP, logging and banners.""",
      ["IOS configuration", "templates", "subnet math", "network automation basics"],
      ["Template: a full branch config with `$placeholders` (string.Template).",
       "Data: sites.csv with five sites.",
       "Derive: gateway, DHCP range and OSPF network statements from the LAN subnet.",
       "Generate: one .cfg file per site in out/.",
       "Test: paste one config into Packet Tracer or CML and check it works."],
      ["Five valid configs are generated and one of them works in a lab."],
      ["How do I compute the wildcard mask for an OSPF network statement?",
       "What should every branch router config include for security?"],
      {"generate.py": py("Generate branch configs: python generate.py sites.csv", '''
import csv
import ipaddress
import sys
from pathlib import Path
from string import Template

template = Template(Path("branch.tpl").read_text(encoding="utf-8"))
out = Path("out")
out.mkdir(exist_ok=True)
for site in csv.DictReader(open(sys.argv[1] if len(sys.argv) > 1 else "sites.csv", encoding="utf-8")):
    lan = ipaddress.ip_network(site["lan"])
    values = dict(site, gateway=str(lan.network_address + 1), mask=str(lan.netmask), wildcard=str(lan.hostmask))
    (out / f"{site['name']}.cfg").write_text(template.substitute(values), encoding="utf-8")
    print("wrote", site["name"])
'''), "branch.tpl": '''hostname $name
!
interface GigabitEthernet0/0
 description WAN
 ip address $wan 255.255.255.252
 no shutdown
!
interface GigabitEthernet0/1
 description LAN
 ip address $gateway $mask
 no shutdown
!
router ospf 1
 network $lan_net $wildcard area $site
!
''', "sites.csv": "name,site,wan,lan,lan_net\nBR-OSAKA,1,203.0.113.2,10.1.1.0/24,10.1.1.0\nBR-NAGOYA,2,203.0.113.6,10.1.2.0/24,10.1.2.0"}),
    P("ccnp", "Expert", "qos_planner", "QoS Policy Planner", "Design queues for a WAN link and generate the MQC configuration.",
      """Voice breaks when a WAN link fills up. Plan a QoS policy for a 50 Mbps branch link: classes for voice, video, business apps, default and scavenger, with DSCP markings, bandwidth percentages and a priority queue for voice that never exceeds a third of the link. Then generate the Cisco MQC config (class-map, policy-map, service-policy) and check the plan's numbers.""",
      ["DSCP and traffic classes", "LLQ and CBWFQ", "policing vs shaping", "MQC configuration"],
      ["Classes: classes.json with DSCP values and percentages.",
       "Validate: total ≤ 100%, priority ≤ 33%, every DSCP used once.",
       "Calls: how many G.711 calls (about 87 kbps each with overhead) fit in the priority queue.",
       "Generate: class-maps, a policy-map and a shaper for the link.",
       "Review: explain where in the network to mark and where to queue."],
      ["The generated config is valid IOS syntax and the checks catch an over-allocated plan."],
      ["Why limit the priority queue to about a third of the link?",
       "Where should traffic be marked, and why there?"],
      {"qos.py": py("Plan and generate a QoS policy: python qos.py classes.json 50", '''
import json
import sys

classes = json.load(open(sys.argv[1] if len(sys.argv) > 1 else "classes.json", encoding="utf-8"))
link_mbps = float(sys.argv[2]) if len(sys.argv) > 2 else 50
total = sum(c["percent"] for c in classes)
print(f"allocated {total}% of {link_mbps} Mbps")
for c in classes:
    print(f"class-map match-any {c['name']}\\n match dscp {c['dscp']}")
# TODO: validation rules, voice call capacity, policy-map with priority/bandwidth, shaper
'''), "classes.json": '''[
  { "name": "VOICE", "dscp": "ef", "percent": 20, "priority": true },
  { "name": "VIDEO", "dscp": "af41", "percent": 20 },
  { "name": "BUSINESS", "dscp": "af21", "percent": 30 },
  { "name": "SCAVENGER", "dscp": "cs1", "percent": 5 }
]'''}),
    P("cissp", "Advanced", "tabletop", "Incident Response Tabletop Exercise", "Run a ransomware tabletop exercise with timed injects and scoring.",
      """Tabletop exercises test an incident response plan without breaking anything. Build a facilitator tool that runs a scenario as timed "injects" (new information arriving), asks the team decision questions, records their answers and decision times, and scores them against the plan: who was notified, when containment started, whether legal and management were involved.""",
      ["incident response phases", "business continuity testing", "roles and escalation", "after-action reports"],
      ["Scenario: scenario.json with injects (minute, message, question, expected actions).",
       "Run: print each inject at its time (or on Enter), record the team's answer.",
       "Score: compare answers with expected actions (keywords), per phase.",
       "Report: an after-action report with what went well and what to improve.",
       "Second scenario: write one for a cloud outage or a lost laptop."],
      ["You've run the exercise with at least one other person and produced the after-action report."],
      ["What should the first hour of a ransomware response look like?",
       "Which decisions in an incident must go to management or legal?"],
      {"tabletop.py": py("Run a tabletop exercise: python tabletop.py scenario.json", '''
import json
import sys
import time

scenario = json.load(open(sys.argv[1] if len(sys.argv) > 1 else "scenario.json", encoding="utf-8"))
log = []
print(f"=== {scenario['title']} ===")
for inject in scenario["injects"]:
    input(f"\\n[T+{inject['minute']} min] press Enter for the next inject")
    print(inject["message"])
    start = time.time()
    answer = input(inject["question"] + "\\n> ")
    log.append({"minute": inject["minute"], "answer": answer, "seconds": round(time.time() - start)})
# TODO: score against inject["expected"], write after_action.md
'''), "scenario.json": '''{
  "title": "Ransomware on the file server",
  "injects": [
    { "minute": 0, "message": "The help desk gets five calls: files on the S: drive have the extension .locked.",
      "question": "What do you do first, and who do you call?", "expected": ["isolate", "incident manager", "do not pay"] },
    { "minute": 30, "message": "A ransom note asks for 5 BTC within 48 hours. Backups from last night exist.",
      "question": "Who decides about the ransom, and what do you check about the backups?", "expected": ["management", "legal", "backups offline", "test restore"] }
  ]
}'''}),
    P("itil", "Advanced", "service_catalog", "IT Service Catalog Site", "Publish a service catalog with SLAs, owners and request forms.",
      """A service catalog tells users what IT offers and what to expect. Build it as a small website: each service (email, laptops, VPN, software requests) with a description, who can use it, the service owner, support hours, SLA targets and how to request it. Generate the pages from services.json with a Python script, so updating the catalog is editing one file.""",
      ["service catalog management", "service level management", "service owners", "generating HTML"],
      ["Define: services.json with at least six services and all the fields above.",
       "Generate: build.py writes index.html and one page per service.",
       "SLAs: show targets in plain language (\"we answer within 1 business hour\").",
       "Request: each page links to the right request form or email.",
       "Review: ask a non-IT person whether they can find how to get a new laptop."],
      ["The generated site covers six services and a non-IT tester found what they needed."],
      ["What's the difference between the business and technical service catalog?",
       "Help me write SLA targets users understand."],
      {"build.py": py("Generate the service catalog site: python build.py", '''
import html
import json
from pathlib import Path

services = json.load(open("services.json", encoding="utf-8"))
items = "\\n".join(f'<li><a href="{s["id"]}.html">{html.escape(s["name"])}</a> · {html.escape(s["summary"])}</li>' for s in services)
Path("index.html").write_text(f"<!doctype html><html lang=en><meta charset=utf-8><title>IT Services</title><h1>IT Services</h1><ul>{items}</ul></html>", encoding="utf-8")
# TODO: one page per service with owner, hours, SLA and how to request it
print("wrote index.html")
'''), "services.json": '''[
  { "id": "email", "name": "Email and calendar", "summary": "Outlook mailbox and shared calendars", "owner": "Ann (IT)", "hours": "24/7", "sla": "Restored within 4 hours" },
  { "id": "laptop", "name": "New laptop", "summary": "A standard laptop for new and existing staff", "owner": "Ken (IT)", "hours": "Weekdays 9–18", "sla": "Delivered within 5 business days" }
]'''}),
    P("ipa-fe", "Novice", "number_systems", "Number Systems and Logic Trainer", "Drill binary, hex, two's complement and logic gates like the exam.",
      """The FE exam always has questions on number conversion, two's complement, floating point and logic circuits. Build a drill program that generates random questions of each kind, checks your answers, explains the method when you're wrong, and tracks which kinds you miss most.""",
      ["binary, octal, hex", "two's complement", "bit shifts", "AND/OR/XOR/NAND truth tables"],
      ["Conversions: random numbers between decimal, binary and hex.",
       "Two's complement: 8-bit negative numbers both ways.",
       "Shifts: logical vs arithmetic shift results.",
       "Logic: random 2-input expressions and their truth tables.",
       "Stats: save results and show your weakest question type."],
      ["You can score 18/20 in each category without paper."],
      ["Explain two's complement of -45 in 8 bits step by step.",
       "What's the difference between an arithmetic and a logical right shift?"],
      {"drill.py": py("FE number system drills: python drill.py", '''
import random


def ask_binary() -> bool:
    n = random.randint(0, 255)
    answer = input(f"{n} in binary (8 bits)? ").strip()
    ok = answer == format(n, "08b")
    print("✔" if ok else f"✘ {format(n, '08b')}  (divide by 2, read the remainders upwards)")
    return ok


if __name__ == "__main__":
    score = sum(ask_binary() for _ in range(5))
    print(f"{score}/5")
    # TODO: hex, two's complement, shifts, logic gates, saved statistics
'''),}),
    P("japanese", "Intermediate", "keigo_mail", "Business Email Template Builder", "Assemble polite Japanese work emails from phrase blocks.",
      """Japanese business email follows fixed patterns: the opening (お世話になっております), self-introduction, the main request in 敬語, and the closing (よろしくお願いいたします). Build a tool that assembles emails for common IT situations (reporting an outage, asking for approval, apologizing for a delay) from phrase blocks, and quizzes you on choosing the right politeness level.""",
      ["business email structure", "敬語: 尊敬語, 謙譲語, 丁寧語", "IT situations in Japanese"],
      ["Phrases: phrases.json with openings, intros, requests and closings, with readings and meanings.",
       "Build: choose a situation and a recipient (customer, boss, colleague) and assemble an email.",
       "Situations: outage report, approval request, schedule change, apology for a delay.",
       "Quiz: show a plain sentence and pick the correct humble or respectful version.",
       "Check: ask a Japanese speaker (or the AI helper) to review three emails."],
      ["You can write an outage report email to a customer with the right structure and politeness."],
      ["Explain the difference between 申し上げる and おっしゃる with work examples.",
       "Review this email I wrote to my manager for politeness and naturalness."],
      {"mail.py": py("Assemble a Japanese business email: python mail.py outage customer", '''
import json
import sys

sys.stdout.reconfigure(encoding="utf-8")
phrases = json.load(open("phrases.json", encoding="utf-8"))
situation = sys.argv[1] if len(sys.argv) > 1 else "outage"
recipient = sys.argv[2] if len(sys.argv) > 2 else "customer"
print(phrases["opening"][recipient])
print(phrases["body"][situation])
print(phrases["closing"][recipient])
# TODO: more situations, the politeness quiz
'''), "phrases.json": '''{
  "opening": { "customer": "いつも大変お世話になっております。", "boss": "お疲れ様です。", "colleague": "お疲れ様です。" },
  "body": {
    "outage": "本日10時頃より、メールシステムに障害が発生しております。現在、復旧に向けて対応中でございます。",
    "approval": "下記の件につきまして、ご承認いただけますでしょうか。"
  },
  "closing": { "customer": "ご迷惑をおかけし、誠に申し訳ございません。何卒よろしくお願いいたします。", "boss": "よろしくお願いいたします。", "colleague": "よろしくお願いします。" }
}'''}),
    P("japan-career", "Intermediate", "resume_site", "Bilingual Résumé (履歴書 and 職務経歴書)", "Write and print a Japanese-format résumé and career history from one data file.",
      """Japanese employers expect a 履歴書 (a standard-format résumé) and a 職務経歴書 (a free-format career history). Keep your details in one JSON file and generate both as printable HTML pages (A4, with the expected sections), plus an English CV for foreign-capital companies, so updating one file updates all three.""",
      ["履歴書 and 職務経歴書 conventions", "print CSS for A4", "generating HTML from data", "presenting IT experience"],
      ["Data: me.json with education, work history, certifications (資格) and skills.",
       "履歴書: the standard sections in the usual table layout, with dates in 和暦 or 西暦 consistently.",
       "職務経歴書: a summary (職務要約), each job's projects with your role, environment and results.",
       "English CV: the same data, one page, achievement-focused.",
       "Print: A4 page size with `@page`, check page breaks in the print preview."],
      ["All three documents print cleanly on A4 and a reviewer finds no missing sections."],
      ["What does a good 職務要約 look like for an infrastructure engineer?",
       "Should I use 和暦 or 西暦 on my 履歴書?"],
      {"build.py": py("Generate résumé pages from me.json: python build.py", '''
import html
import json
from pathlib import Path

me = json.load(open("me.json", encoding="utf-8"))
rows = "".join(f"<tr><td>{c['date']}</td><td>{html.escape(c['name'])}</td></tr>" for c in me["certifications"])
page = f"""<!doctype html><html lang="ja"><meta charset="utf-8"><link rel="stylesheet" href="print.css">
<title>履歴書</title><h1>履歴書</h1><p>{html.escape(me['name'])}</p><h2>免許・資格</h2><table>{rows}</table></html>"""
Path("rirekisho.html").write_text(page, encoding="utf-8")
# TODO: education and work history, 職務経歴書, English CV
print("wrote rirekisho.html")
'''), "me.json": '''{
  "name": "Your Name",
  "certifications": [
    { "date": "2026年4月", "name": "CompTIA A+" },
    { "date": "2026年9月", "name": "AWS Certified Cloud Practitioner" }
  ],
  "education": [],
  "work": []
}''', "print.css": '''@page { size: A4; margin: 15mm; }
body { font-family: "Yu Gothic", "Noto Sans JP", sans-serif; font-size: 10.5pt; }
table { border-collapse: collapse; width: 100%; }
td { border: 1px solid #000; padding: 2mm; }
'''}),
]
