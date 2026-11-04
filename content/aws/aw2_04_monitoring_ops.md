@@lesson aw2-monitoring CloudWatch, CloudTrail, Config and Operations
@@text
Running workloads on AWS means knowing what's happening (metrics, logs, traces), who did what (API audit), whether resources stay compliant, and how to automate responses and patching. This lesson covers CloudWatch, CloudTrail, AWS Config, EventBridge and Systems Manager, which appear across all AWS associate exams.

## CloudWatch metrics and alarms

- **Metrics** are time series in **namespaces** (`AWS/EC2`, `AWS/ApplicationELB`) with **dimensions** (InstanceId, LoadBalancer). EC2 basic monitoring: 5-minute; **detailed**: 1-minute. Custom metrics down to 1-second (**high resolution**).
- EC2 **memory and disk usage are not built-in**: install the **CloudWatch agent**.
- **Alarms** watch one metric (or a metric math expression) with a **period**, **statistic** (Average, Sum, Maximum, p99), **threshold**, **evaluation periods** (N) and **datapoints to alarm** (M): "M out of N". States: **OK**, **ALARM**, **INSUFFICIENT_DATA**. Missing data can be treated as missing, not breaching, breaching or ignored.
- Actions: SNS notification, Auto Scaling policy, EC2 action (stop, reboot, **recover**), Systems Manager OpsItem.
- **Composite alarms** combine alarms with AND/OR to reduce noise.
- **Dashboards**, **anomaly detection**, **Synthetics canaries** (scripted checks of endpoints), **RUM**, **Application Signals**, **Container Insights**, **Lambda Insights**.

## CloudWatch Logs

Log groups → log streams → events. Set **retention** (default: never expire, which costs money). **Metric filters** turn log patterns into metrics ("count ERROR lines"); **Logs Insights** queries logs:

```text
fields @timestamp, @message
| filter @message like /ERROR/
| stats count() as errors by bin(5m)
```

**Subscription filters** stream logs to Lambda, Kinesis or OpenSearch; export to S3 for archive. **X-Ray** traces requests across services.

## CloudTrail

Records **API calls** (who, what, when, from where) across the account.

- **Management events** (control plane: `RunInstances`, `PutBucketPolicy`) are logged by default and kept **90 days** in **Event history**.
- **Data events** (`s3:GetObject`, Lambda `Invoke`, DynamoDB item operations) must be enabled and cost extra.
- **Insights events** detect unusual API activity.
- Create a **trail** (organisation-wide, all Regions) to deliver logs to S3 for long-term retention; enable **log file integrity validation**; send to CloudWatch Logs for alerting. **CloudTrail Lake** stores and queries events with SQL.

Typical investigation: who deleted the security group? → CloudTrail `DeleteSecurityGroup` event: `userIdentity`, `sourceIPAddress`, `eventTime`.

## AWS Config

Records **resource configurations and their history** and evaluates **Config rules** (managed, e.g. `s3-bucket-public-read-prohibited`, `encrypted-volumes`, `restricted-ssh`; or custom with Lambda/Guard). **Remediation** with SSM Automation documents. **Conformance packs** bundle rules (CIS, PCI); **aggregators** give a multi-account view.

CloudTrail answers "**who** changed it", Config answers "**what did it look like** and is it compliant", CloudWatch answers "**how is it performing**".

## EventBridge

An event bus that routes events (AWS service events, CloudTrail API calls, SaaS partners, your own apps) by **rules** with **event patterns** to targets (Lambda, SQS, SNS, Step Functions, SSM). **EventBridge Scheduler** runs cron/rate schedules. **Pipes** connect sources to targets with filtering and enrichment.

```json,norun
{"source": ["aws.ec2"], "detail-type": ["EC2 Instance State-change Notification"], "detail": {"state": ["stopped", "terminated"]}}
```

## Systems Manager (SSM)

- **Session Manager**: shell access **without SSH, open ports or bastion hosts**, logged to S3/CloudWatch.
- **Run Command**: run scripts across fleets.
- **Patch Manager** with patch baselines and **maintenance windows**.
- **Parameter Store**: configuration and secrets (SecureString with KMS); **Secrets Manager** adds automatic rotation (RDS, etc.).
- **Automation** runbooks, **State Manager**, **Inventory**, **OpsCenter**, **Fleet Manager**.
- Requires the **SSM Agent** and an instance role with `AmazonSSMManagedInstanceCore`.

## Other operational services

**AWS Health Dashboard** (service events affecting you), **Trusted Advisor** (cost, performance, security, fault tolerance, service limits checks), **Service Quotas**, **AWS Backup** (central backup plans, vault lock), **CloudFormation** (infrastructure as code: stacks, change sets, drift detection, StackSets).

@@quiz
Which service shows who deleted an S3 bucket policy yesterday?
- CloudWatch Metrics
* CloudTrail
- AWS Config conformance packs
- Trusted Advisor
? Management events record API calls with the identity.

@@quiz
You need EC2 memory utilisation in CloudWatch. What do you do?
- Enable detailed monitoring
* Install and configure the CloudWatch agent
- Enable CloudTrail data events
- Turn on VPC Flow Logs
? Memory isn't visible to the hypervisor.

@@quiz
An alarm has period 1 minute, evaluation periods 5, datapoints to alarm 3. When does it go to ALARM?
- After 1 breaching datapoint
* When 3 of the last 5 datapoints breach
- After 5 consecutive breaches
- After 3 minutes regardless of values
? M out of N.

@@quiz
Which service records configuration history and flags unencrypted EBS volumes?
- CloudTrail
* AWS Config
- X-Ray
- EventBridge
? Config rules evaluate compliance.

@@quiz
Administrators need shell access to private instances without opening port 22 or running a bastion. What should they use?
- EC2 Instance Connect with a public IP
* Systems Manager Session Manager
- A NAT gateway
- Direct Connect
? Sessions are brokered by SSM and logged.

@@flashcards
basic / detailed monitoring :: 5-min / 1-min
CloudWatch agent :: memory, disk, logs from instances
alarm M of N :: datapoints to alarm / evaluation periods
alarm states :: OK, ALARM, INSUFFICIENT_DATA
composite alarm :: combine alarms with AND/OR
metric filter :: log pattern to metric
Logs Insights :: query language for logs
log retention default :: never expire
X-Ray :: distributed tracing
CloudTrail management events :: logged by default, 90 days
CloudTrail data events :: object-level, opt in
trail :: long-term delivery to S3
log file validation :: tamper detection
AWS Config :: configuration history + rules
CloudTrail vs Config :: who vs what/compliant
EventBridge rule :: event pattern to targets
Session Manager :: shell without SSH/bastion
Patch Manager :: baselines + maintenance windows
Parameter Store :: config and SecureString
Secrets Manager :: automatic rotation
Trusted Advisor :: best-practice checks

@@drill
How many days of management events does CloudTrail Event history keep?
= 90
= 90 days
? Use a trail for longer.

@@drill
Which SSM capability gives browser or CLI shell access without SSH?
= Session Manager
= session manager
? No inbound ports needed.

@@exercise Evaluate a CloudWatch alarm over time
Evaluate an alarm on a series of 1-minute datapoints (`None` = missing). At each minute, look at the last `EVAL_PERIODS` datapoints: treat missing data according to `MISSING` (`"notBreaching"` counts as OK, `"breaching"` as a breach). Go to ALARM when at least `DATAPOINTS` breach (value > threshold), back to OK otherwise; stay INSUFFICIENT_DATA until `EVAL_PERIODS` datapoints exist. Print each state change with its minute.
@@lang python
@@starter
THRESHOLD = 80
EVAL_PERIODS = 5
DATAPOINTS = 3
MISSING = "notBreaching"
cpu = [45, 50, 85, 90, 40, 88, 92, None, 70, 60, 55, None, None, 95, 97, 99]
@@expected
minute 0: INSUFFICIENT_DATA
minute 4: INSUFFICIENT_DATA -> OK (2/5 breaching: [45, 50, 85, 90, 40])
minute 5: OK -> ALARM (3/5 breaching: [50, 85, 90, 40, 88])
minute 8: ALARM -> OK (2/5 breaching: [40, 88, 92, None, 70])
minute 15: OK -> ALARM (3/5 breaching: [None, None, 95, 97, 99])
@@requires
EVAL_PERIODS
DATAPOINTS
def breaches
@@hint
`window = cpu[max(0, i - EVAL_PERIODS + 1): i + 1]`. `breaches(v)` returns `v > THRESHOLD`, or `MISSING == "breaching"` for `None`.
@@solution
THRESHOLD = 80
EVAL_PERIODS = 5
DATAPOINTS = 3
MISSING = "notBreaching"
cpu = [45, 50, 85, 90, 40, 88, 92, None, 70, 60, 55, None, None, 95, 97, 99]

def breaches(v):
    if v is None:
        return MISSING == "breaching"
    return v > THRESHOLD

state = "INSUFFICIENT_DATA"
print(f"minute 0: {state}")
for i in range(len(cpu)):
    if i + 1 < EVAL_PERIODS:
        continue
    window = cpu[i - EVAL_PERIODS + 1: i + 1]
    count = sum(breaches(v) for v in window)
    new = "ALARM" if count >= DATAPOINTS else "OK"
    if new != state:
        print(f"minute {i}: {state} -> {new} ({count}/{EVAL_PERIODS} breaching: {window})")
        state = new

@@exercise Investigate CloudTrail events
`events` is CloudTrail JSON (one record per line). Parse it and report: every security-sensitive event (`SENSITIVE` names) with time, user and source IP; failed calls (`errorCode` present) counted per user; and any API calls made by the root user. Users are `userIdentity.userName` or, for assumed roles, the session name after the last `/` of `userIdentity.arn`.
@@lang python
@@starter
import json
from collections import Counter

SENSITIVE = {"DeleteTrail", "StopLogging", "PutBucketPolicy", "AuthorizeSecurityGroupIngress", "CreateAccessKey", "DeleteSecurityGroup"}
events = """{"eventTime": "2026-11-03T09:12:01Z", "eventName": "RunInstances", "sourceIPAddress": "10.0.5.4", "userIdentity": {"type": "AssumedRole", "arn": "arn:aws:sts::111122223333:assumed-role/CiDeployer/build-4411"}}
{"eventTime": "2026-11-03T09:40:17Z", "eventName": "AuthorizeSecurityGroupIngress", "sourceIPAddress": "198.51.100.23", "userIdentity": {"type": "IAMUser", "userName": "dana", "arn": "arn:aws:iam::111122223333:user/dana"}}
{"eventTime": "2026-11-03T10:02:44Z", "eventName": "PutBucketPolicy", "sourceIPAddress": "198.51.100.23", "errorCode": "AccessDenied", "userIdentity": {"type": "IAMUser", "userName": "dana", "arn": "arn:aws:iam::111122223333:user/dana"}}
{"eventTime": "2026-11-03T10:03:10Z", "eventName": "GetObject", "sourceIPAddress": "198.51.100.23", "errorCode": "AccessDenied", "userIdentity": {"type": "IAMUser", "userName": "dana", "arn": "arn:aws:iam::111122223333:user/dana"}}
{"eventTime": "2026-11-03T11:30:00Z", "eventName": "CreateAccessKey", "sourceIPAddress": "203.0.113.77", "userIdentity": {"type": "Root", "arn": "arn:aws:iam::111122223333:root"}}
{"eventTime": "2026-11-03T11:31:05Z", "eventName": "StopLogging", "sourceIPAddress": "203.0.113.77", "userIdentity": {"type": "Root", "arn": "arn:aws:iam::111122223333:root"}}
{"eventTime": "2026-11-03T12:00:00Z", "eventName": "DescribeInstances", "sourceIPAddress": "10.0.5.4", "errorCode": "ThrottlingException", "userIdentity": {"type": "AssumedRole", "arn": "arn:aws:sts::111122223333:assumed-role/CiDeployer/build-4412"}}"""

def user_of(identity):
    raise NotImplementedError
@@expected
Sensitive events:
  2026-11-03T09:40:17Z AuthorizeSecurityGroupIngress by dana from 198.51.100.23
  2026-11-03T10:02:44Z PutBucketPolicy by dana from 198.51.100.23 FAILED AccessDenied
  2026-11-03T11:30:00Z CreateAccessKey by root from 203.0.113.77
  2026-11-03T11:31:05Z StopLogging by root from 203.0.113.77
Failed calls: dana 2, build-4412 1
Root user activity: CreateAccessKey, StopLogging
@@requires
json.loads(
def user_of
Counter(
@@hint
Return `"root"` for type Root, `identity["userName"]` when present, otherwise `identity["arn"].rsplit("/", 1)[-1]`.
@@solution
import json
from collections import Counter

SENSITIVE = {"DeleteTrail", "StopLogging", "PutBucketPolicy", "AuthorizeSecurityGroupIngress", "CreateAccessKey", "DeleteSecurityGroup"}
events = """{"eventTime": "2026-11-03T09:12:01Z", "eventName": "RunInstances", "sourceIPAddress": "10.0.5.4", "userIdentity": {"type": "AssumedRole", "arn": "arn:aws:sts::111122223333:assumed-role/CiDeployer/build-4411"}}
{"eventTime": "2026-11-03T09:40:17Z", "eventName": "AuthorizeSecurityGroupIngress", "sourceIPAddress": "198.51.100.23", "userIdentity": {"type": "IAMUser", "userName": "dana", "arn": "arn:aws:iam::111122223333:user/dana"}}
{"eventTime": "2026-11-03T10:02:44Z", "eventName": "PutBucketPolicy", "sourceIPAddress": "198.51.100.23", "errorCode": "AccessDenied", "userIdentity": {"type": "IAMUser", "userName": "dana", "arn": "arn:aws:iam::111122223333:user/dana"}}
{"eventTime": "2026-11-03T10:03:10Z", "eventName": "GetObject", "sourceIPAddress": "198.51.100.23", "errorCode": "AccessDenied", "userIdentity": {"type": "IAMUser", "userName": "dana", "arn": "arn:aws:iam::111122223333:user/dana"}}
{"eventTime": "2026-11-03T11:30:00Z", "eventName": "CreateAccessKey", "sourceIPAddress": "203.0.113.77", "userIdentity": {"type": "Root", "arn": "arn:aws:iam::111122223333:root"}}
{"eventTime": "2026-11-03T11:31:05Z", "eventName": "StopLogging", "sourceIPAddress": "203.0.113.77", "userIdentity": {"type": "Root", "arn": "arn:aws:iam::111122223333:root"}}
{"eventTime": "2026-11-03T12:00:00Z", "eventName": "DescribeInstances", "sourceIPAddress": "10.0.5.4", "errorCode": "ThrottlingException", "userIdentity": {"type": "AssumedRole", "arn": "arn:aws:sts::111122223333:assumed-role/CiDeployer/build-4412"}}"""

def user_of(identity):
    if identity["type"] == "Root":
        return "root"
    if "userName" in identity:
        return identity["userName"]
    return identity["arn"].rsplit("/", 1)[-1]

records = [json.loads(line) for line in events.splitlines()]
failures = Counter()
root_calls = []
print("Sensitive events:")
for r in records:
    user = user_of(r["userIdentity"])
    status = f" FAILED {r['errorCode']}" if "errorCode" in r else ""
    if r["eventName"] in SENSITIVE:
        print(f"  {r['eventTime']} {r['eventName']} by {user} from {r['sourceIPAddress']}{status}")
    if "errorCode" in r:
        failures[user] += 1
    if user == "root":
        root_calls.append(r["eventName"])
print("Failed calls:", ", ".join(f"{u} {n}" for u, n in failures.most_common()))
print("Root user activity:", ", ".join(root_calls) if root_calls else "none")
