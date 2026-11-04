@@lesson az2-monitor-backup Monitoring, Backup and Disaster Recovery on Azure
@@text
The last AZ-104 domain, "monitor and maintain Azure resources" (10–15%), covers Azure Monitor (metrics, logs, alerts), Log Analytics with KQL, Application Insights, Network Watcher, Azure Backup and Site Recovery. These are also the tools you'll use daily as an Azure administrator.

## Azure Monitor

- **Metrics**: numeric time series, collected automatically for most resources (VM CPU, disk, network), retained 93 days; guest OS metrics need the **Azure Monitor Agent** (AMA) with a **data collection rule** (DCR).
- **Logs**: stored in a **Log Analytics workspace**: activity log, resource logs (via **diagnostic settings**), VM event/syslog and performance counters (via AMA + DCR), Entra sign-in/audit logs, Defender alerts.
- **Activity log**: subscription-level control-plane events (who created, deleted, changed what), kept 90 days; send it to a workspace for longer.
- **Insights**: VM insights, Container insights, Network insights, Storage insights.

## KQL basics

```text
// Failed sign-ins per user in the last day
SigninLogs
| where TimeGenerated > ago(1d)
| where ResultType != "0"
| summarize failures = count() by UserPrincipalName
| order by failures desc
| take 10

// VMs with average CPU above 80% in 5-minute bins
Perf
| where ObjectName == "Processor" and CounterName == "% Processor Time"
| summarize avg_cpu = avg(CounterValue) by Computer, bin(TimeGenerated, 5m)
| where avg_cpu > 80
```

Operators to know: `where`, `project`, `extend`, `summarize ... by`, `bin()`, `order by`, `take`/`limit`, `join`, `render timechart`, `ago()`.

## Alerts

An **alert rule** = **scope** (resource) + **condition** (metric threshold, log query result, activity log event, resource health, or dynamic threshold) + **action group** (email, SMS, push, voice, webhook, Logic App, Azure Function, Automation runbook, ITSM). **Alert processing rules** suppress or route alerts (maintenance windows). Severities 0 (critical) to 4 (verbose). Stateful metric alerts resolve automatically.

## Application Insights

APM for apps: requests, dependencies, exceptions, page views, **availability tests**, **Application Map**, live metrics, distributed tracing (OpenTelemetry). Workspace-based (data in Log Analytics).

## Network Watcher

IP flow verify, next hop, effective security rules, **connection monitor**, **packet capture**, **VPN troubleshoot**, topology, **flow logs** (VNet flow logs replace NSG flow logs) with **traffic analytics**.

## Azure Backup

- **Recovery Services vault** (VMs, SQL in VM, Azure Files, on-premises via MARS agent/MABS) and **Backup vault** (newer workloads: Azure Disks, Blobs, PostgreSQL, AKS). The vault must be in the **same region** as the protected resources (for most workloads).
- **Backup policies**: schedule (daily/weekly, enhanced policy for hourly) and retention (daily, weekly, monthly, yearly: a **GFS** scheme). **Instant restore** snapshots kept 1–5 days (up to 30 with enhanced policy) for fast restores.
- Restore options for VMs: create a new VM, **restore disks**, replace existing, **file recovery** (mount a recovery point as a drive), cross-region restore (with GRS vaults).
- Security: **soft delete** (14 days, extendable), **immutable vaults**, **multi-user authorisation** (Resource Guard), encryption.
- Vault storage redundancy (LRS/ZRS/GRS) must be set **before** protecting the first item.

## Azure Site Recovery

Replicates VMs (Azure to Azure across regions, Hyper-V, VMware, physical) for **disaster recovery**: replication policy (recovery point retention, app-consistent snapshot frequency), **recovery plans** (ordered groups and scripts), **test failover** (in an isolated network, no impact), **failover**, **commit**, **re-protect** and failback.

**Backup vs Site Recovery**: Backup protects **data** over time (point-in-time restore, ransomware, accidental deletion); Site Recovery keeps **workloads running** after a regional outage (low RTO/RPO).

@@quiz
Where must you send resource logs so you can query them with KQL?
* A Log Analytics workspace via diagnostic settings
- The activity log
- Metrics explorer
- An action group
? Resource logs aren't collected until a diagnostic setting exists.

@@quiz
Which component defines who is notified and what automation runs when an alert fires?
- Alert processing rule
* Action group
- Data collection rule
- Diagnostic setting
? Email, SMS, webhooks, runbooks and more.

@@quiz
Which KQL operator aggregates rows, e.g. counting failures per user?
- where
- project
* summarize
- extend
? summarize count() by column.

@@quiz
A user deleted a file on a backed-up VM yesterday. What's the quickest restore?
- Restore the whole VM
* File recovery from a recovery point
- Site Recovery failover
- Restore disks to a new region
? Mount the recovery point and copy the file.

@@quiz
You need to rehearse a regional failover without affecting production. What do you run?
- A backup restore
* A Site Recovery test failover into an isolated VNet
- A planned failover
- Re-protect
? Test failover has no impact on replication.

@@quiz
When must you set a Recovery Services vault's storage redundancy?
- Any time
* Before protecting the first item
- After the first backup
- Only when restoring
? It can't be changed once items are protected.

@@flashcards
metrics retention :: 93 days
Azure Monitor Agent :: guest OS data via DCR
diagnostic settings :: send resource logs to a destination
Log Analytics workspace :: log store queried with KQL
activity log :: control-plane events, 90 days
KQL summarize :: aggregate by columns
KQL bin() :: time buckets
alert rule :: scope + condition + action group
action group :: notifications and automation
alert processing rule :: suppress or route alerts
Application Insights :: APM for apps
connection monitor :: end-to-end reachability
Recovery Services vault :: VMs, SQL, Files, MARS
Backup vault :: disks, blobs, PostgreSQL, AKS
GFS retention :: daily, weekly, monthly, yearly
instant restore :: local snapshots for fast restore
file recovery :: mount a recovery point
soft delete :: 14 days for backups
Site Recovery :: replicate VMs for DR
test failover :: isolated rehearsal
backup vs ASR :: data history vs keep running

@@drill
Which Azure tool replicates VMs to another region for disaster recovery?
= Azure Site Recovery
= Site Recovery
= ASR
? Backup protects data over time.

@@drill
How many days is the Azure activity log kept by default?
= 90
= 90 days
? Export it to a workspace for longer.

@@exercise Run a KQL-style query in Python
Reproduce this query over the `signins` rows: `where TimeGenerated > ago(1d) | where ResultType != "0" | summarize failures = count(), ips = dcount(IPAddress) by UserPrincipalName | where failures >= 3 | order by failures desc`. Use `NOW` as the current time. Then print a 1-hour `bin()` count of all failures (any user) in the last day.
@@lang python
@@starter
from datetime import datetime, timedelta
from collections import Counter

NOW = datetime(2026, 11, 4, 12, 0)
signins = [
    ("2026-11-04T09:01:00", "ana@contoso.com", "0", "203.0.113.5"),
    ("2026-11-04T09:15:00", "ben@contoso.com", "50126", "198.51.100.7"),
    ("2026-11-04T09:16:00", "ben@contoso.com", "50126", "198.51.100.7"),
    ("2026-11-04T09:17:00", "ben@contoso.com", "50126", "198.51.100.8"),
    ("2026-11-04T10:02:00", "chen@contoso.com", "50053", "192.0.2.44"),
    ("2026-11-04T10:30:00", "ben@contoso.com", "0", "203.0.113.9"),
    ("2026-11-04T11:05:00", "admin@contoso.com", "50126", "192.0.2.44"),
    ("2026-11-04T11:06:00", "admin@contoso.com", "50126", "192.0.2.45"),
    ("2026-11-04T11:07:00", "admin@contoso.com", "50126", "192.0.2.46"),
    ("2026-11-04T11:08:00", "admin@contoso.com", "50126", "192.0.2.47"),
    ("2026-11-02T08:00:00", "ana@contoso.com", "50126", "203.0.113.5"),
]
@@expected
UserPrincipalName      failures ips
admin@contoso.com             4   4
ben@contoso.com               3   2
Failures per hour:
  2026-11-04 09:00 ### 3
  2026-11-04 10:00 # 1
  2026-11-04 11:00 #### 4
@@requires
timedelta(days=1)
fromisoformat(
@@hint
Filter with `datetime.fromisoformat(t) > NOW - timedelta(days=1)`. For dcount use a set of IPs per user. A 1-hour bin is `t.replace(minute=0, second=0)`.
@@solution
from datetime import datetime, timedelta
from collections import Counter

NOW = datetime(2026, 11, 4, 12, 0)
signins = [
    ("2026-11-04T09:01:00", "ana@contoso.com", "0", "203.0.113.5"),
    ("2026-11-04T09:15:00", "ben@contoso.com", "50126", "198.51.100.7"),
    ("2026-11-04T09:16:00", "ben@contoso.com", "50126", "198.51.100.7"),
    ("2026-11-04T09:17:00", "ben@contoso.com", "50126", "198.51.100.8"),
    ("2026-11-04T10:02:00", "chen@contoso.com", "50053", "192.0.2.44"),
    ("2026-11-04T10:30:00", "ben@contoso.com", "0", "203.0.113.9"),
    ("2026-11-04T11:05:00", "admin@contoso.com", "50126", "192.0.2.44"),
    ("2026-11-04T11:06:00", "admin@contoso.com", "50126", "192.0.2.45"),
    ("2026-11-04T11:07:00", "admin@contoso.com", "50126", "192.0.2.46"),
    ("2026-11-04T11:08:00", "admin@contoso.com", "50126", "192.0.2.47"),
    ("2026-11-02T08:00:00", "ana@contoso.com", "50126", "203.0.113.5"),
]

since = NOW - timedelta(days=1)
failed = [(datetime.fromisoformat(t), upn, ip) for t, upn, result, ip in signins
          if datetime.fromisoformat(t) > since and result != "0"]

counts = Counter(upn for _, upn, _ in failed)
ips = {}
for _, upn, ip in failed:
    ips.setdefault(upn, set()).add(ip)
print("UserPrincipalName      failures ips")
for upn, n in counts.most_common():
    if n >= 3:
        print(f"{upn:<22} {n:>8} {len(ips[upn]):>3}")

print("Failures per hour:")
bins = Counter(t.replace(minute=0, second=0) for t, _, _ in failed)
for hour in sorted(bins):
    print(f"  {hour:%Y-%m-%d %H:00} {'#' * bins[hour]} {bins[hour]}")

@@exercise Which recovery points does a backup policy keep?
A daily backup runs at 23:00. The policy keeps daily points for `DAILY` days, weekly points (Sunday) for `WEEKLY` weeks, monthly points (first Sunday of the month) for `MONTHLY` months, and yearly points (first Sunday of January) for `YEARLY` years. Given today's date, list how many recovery points exist in each category for the last 400 days of backups, the total (a point kept by several rules counts once), and the oldest point kept.
@@lang python
@@starter
from datetime import date, timedelta

TODAY = date(2026, 11, 4)
DAILY, WEEKLY, MONTHLY, YEARLY = 30, 12, 12, 3

def kept_by(point):
    raise NotImplementedError
@@expected
daily    30
weekly   12
monthly  12
yearly   1
Total recovery points: 47
Oldest kept: 2025-12-07 kept by monthly
@@requires
def kept_by
weekday() == 6
timedelta(days=
@@hint
For a point `p`, `age = (TODAY - p).days`. Daily if `age < DAILY`; weekly if Sunday and `age < WEEKLY * 7`; monthly if it's the first Sunday (`p.day <= 7`) and fewer than `MONTHLY` calendar months back (`(TODAY.year - p.year) * 12 + TODAY.month - p.month`); yearly if also January and `TODAY.year - p.year < YEARLY`.
@@solution
from datetime import date, timedelta

TODAY = date(2026, 11, 4)
DAILY, WEEKLY, MONTHLY, YEARLY = 30, 12, 12, 3

def kept_by(point):
    age = (TODAY - point).days
    months = (TODAY.year - point.year) * 12 + TODAY.month - point.month
    rules = []
    if age < DAILY:
        rules.append("daily")
    if point.weekday() == 6:
        if age < WEEKLY * 7:
            rules.append("weekly")
        if point.day <= 7 and months < MONTHLY:
            rules.append("monthly")
        if point.day <= 7 and point.month == 1 and TODAY.year - point.year < YEARLY:
            rules.append("yearly")
    return rules

counts = {"daily": 0, "weekly": 0, "monthly": 0, "yearly": 0}
kept = []
for days_ago in range(400):
    point = TODAY - timedelta(days=days_ago)
    rules = kept_by(point)
    for r in rules:
        counts[r] += 1
    if rules:
        kept.append(point)

for rule, n in counts.items():
    print(f"{rule:<8} {n}")
print("Total recovery points:", len(kept))
print("Oldest kept:", min(kept), "kept by", ", ".join(kept_by(min(kept))))
