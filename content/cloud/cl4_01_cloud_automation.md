@@lesson cl4-automation Automating the Cloud with Python
@@concepts functions, maps, loops, errors, automation
@@lang python
@@text
Every cloud is an API. The console, the CLI and Terraform all call the same HTTPS endpoints, and Python scripts can too. CloudOps engineers write small scripts every week: find unused volumes, stop dev servers at night, rotate keys, report costs, react to events. This lesson teaches the patterns that make those scripts **correct** (they see every resource), **polite** (they back off when throttled) and **safe** (they don't delete production by accident).

## The SDKs

| Cloud | Python SDK | Credentials it finds automatically |
|---|---|---|
| AWS | `boto3` | env vars, `~/.aws/config` profiles and SSO, instance/Lambda roles |
| Azure | `azure-identity` + `azure-mgmt-*` | `DefaultAzureCredential`: env, managed identity, `az login` |
| Google Cloud | `google-cloud-*` | Application Default Credentials: `gcloud auth application-default login`, service accounts |

Never put keys in the script. Let the SDK find credentials from the environment, so the same code runs on your laptop (SSO) and in Lambda or a pipeline (a role via OIDC).

```python,norun
import boto3

session = boto3.Session(profile_name="dev", region_name="ap-northeast-1")
ec2 = session.client("ec2")                     # low-level client: one method per API call
resp = ec2.describe_instances(Filters=[{"Name": "tag:Environment", "Values": ["dev"]}])
for reservation in resp["Reservations"]:
    for inst in reservation["Instances"]:
        print(inst["InstanceId"], inst["State"]["Name"])
```

Responses are plain dicts and lists, exactly the JSON the API returns. `aws ec2 describe-instances --output json` shows the same shape, which is the fastest way to explore it.

## Pagination: seeing every resource

APIs return results a page at a time (often 50 to 1000 items) with a **token** for the next page. A script that only reads the first page silently misses resources: the classic automation bug.

```python,norun
paginator = ec2.get_paginator("describe_volumes")
for page in paginator.paginate(Filters=[{"Name": "status", "Values": ["available"]}]):
    for vol in page["Volumes"]:
        print(vol["VolumeId"], vol["Size"], "GiB, unattached")
```

boto3 paginators follow `NextToken` (or `Marker`, `ContinuationToken`...) for you. Azure SDK list methods return iterators that page automatically; Google's return pagers. The first exercise writes the loop by hand so you know what's happening underneath.

## Throttling, retries and backoff

Every API has rate limits. Too many calls and you get `ThrottlingException`, `TooManyRequests` or HTTP 429/503. The fix is **retry with exponential backoff and jitter**: wait 1 s, 2 s, 4 s, 8 s (capped), each with some randomness so a thousand scripts don't retry at the same instant. Only retry errors that are **transient** (throttling, timeouts, 5xx), never `AccessDenied` or `ValidationError`.

```python,norun
from botocore.config import Config
ec2 = boto3.client("ec2", config=Config(retries={"max_attempts": 10, "mode": "adaptive"}))
```

The SDKs do this for you when configured; the second exercise builds it yourself, because you'll need it for your own HTTP calls and for operations that are "eventually consistent" (a resource that was just created may not be visible yet).

## Waiters

Many operations are asynchronous: `run_instances` returns at once while the server is still starting. **Waiters** poll until the state is reached:

```python,norun
ec2.get_waiter("instance_running").wait(InstanceIds=[instance_id])
```

## Safe destructive scripts

Cleanup scripts are where automation hurts people. Build them like this:

1. **Dry run by default.** Print what *would* happen; delete only with `--apply`. (Many AWS calls also accept `DryRun=True`.)
2. **Filter positively.** Act only on resources that match explicit rules (`Environment=dev`, older than 30 days, unattached) and **skip anything tagged** `DoNotDelete` or `Environment=prod`.
3. **Log everything** (the resource, why it matched, what was done), and keep the output.
4. **Snapshot or tag before deleting**, or mark first (`ScheduledForDeletion=2026-12-01`) and delete in a later run.
5. **Least privilege** for the role that runs it: it can only touch what it's meant to.

The third exercise is a dry-run cleanup planner.

## Event-driven automation

Instead of polling on a schedule, react to events:

- **EventBridge** (AWS), **Event Grid** (Azure), **Eventarc** (GCP) deliver events like "an instance changed state", "a bucket policy changed", "a CloudTrail API call happened".
- A **rule** matches the event pattern and invokes a **Lambda / Azure Function / Cloud Run function**.
- Examples: tag new resources with the creator's name; remove a security group rule that opens 22 to the world; page someone when the root account signs in; start a pipeline when a file lands in a bucket.

```python,norun
def handler(event, context):                       # AWS Lambda entry point
    detail = event["detail"]
    if detail["eventName"] == "AuthorizeSecurityGroupIngress":
        for perm in detail["requestParameters"]["ipPermissions"]["items"]:
            ...                                     # revoke it if it opens 0.0.0.0/0 on port 22
```

Event handlers must be **idempotent** (events can arrive twice) and fast; put slow work on a queue.

## Scheduled jobs and where to run them

| Job | Where |
|---|---|
| A few seconds, on a schedule or event | Lambda + EventBridge Scheduler, Azure Functions timer, Cloud Scheduler + Cloud Run |
| Minutes to hours | ECS/Fargate scheduled task, Azure Container Apps job, Cloud Run jobs, Kubernetes CronJob |
| Runbooks operators trigger | Systems Manager Automation, Azure Automation |
| Reports from many accounts | a central "automation" account that **assumes a role** in each member account (`sts.assume_role`) |

## Querying instead of scripting

Often you don't need a loop at all: **AWS Config advanced queries** and **Steampipe** (SQL over cloud APIs), **Azure Resource Graph** (KQL across subscriptions), and **Cloud Asset Inventory** (GCP) answer "which buckets are public in every account?" in one query. Cost data comes from the **Cost Explorer API**, Cost Management exports, or the billing export to BigQuery, and the fourth exercise summarizes such an export.

@@quiz
A script lists S3 objects with one `list_objects_v2` call and reports 1,000 objects, but the bucket has 40,000. What's wrong?
- The bucket is in another region
* It read only the first page; it must follow the continuation token (or use a paginator)
- The role is missing s3:GetObject
- S3 limits buckets to 1,000 objects
? 1,000 is the page size of ListObjectsV2.

@@quiz
Which error should a script retry with backoff?
- AccessDenied
- ValidationError: invalid instance type
* ThrottlingException
- NoSuchBucket
? Only transient errors can succeed on retry.

@@quiz
Why add random jitter to the backoff delay?
* So many clients that failed together don't all retry at the same moment and overload the service again
- To make the script slower on purpose
- Because the APIs require it
- To hide the script from rate limits
? Without jitter, retries arrive in synchronised waves.

@@quiz
What should a cleanup script do by default?
- Delete everything that matches a name pattern
* A dry run that lists what it would do, acting only with an explicit flag, and skipping protected resources
- Ask for root credentials
- Run only in the console
? Make the safe path the default one.

@@quiz
How does a central automation account read resources in 40 member accounts?
- With 40 access keys stored in the script
* By calling sts:AssumeRole into a read role deployed in each member account
- It can't; scripts must run in each account
- With VPC peering
? Temporary credentials per account, from one trusted identity.

@@flashcards
boto3 client :: low-level AWS API calls, responses as dicts
DefaultAzureCredential :: Azure SDK credential chain
Application Default Credentials :: Google Cloud SDK credential chain
pagination token :: NextToken / Marker / ContinuationToken to fetch the next page
paginator :: SDK helper that follows the tokens for you
throttling :: HTTP 429/503, ThrottlingException
exponential backoff with jitter :: 1, 2, 4, 8 s... capped, plus randomness
transient vs permanent errors :: retry the first, never the second
waiter :: polls until a resource reaches a state
DryRun / --apply :: safe default for destructive scripts
EventBridge / Event Grid / Eventarc :: event routing to functions
idempotent handler :: processing the same event twice does no harm
sts.assume_role :: temporary credentials in another account
Resource Graph / Config queries / Steampipe :: query resources instead of looping

@@exercise Follow the pages
`client.list_volumes(token)` returns one page as a dict: `{"Volumes": [...], "NextToken": "..."}`, and the last page has no `NextToken`. Call it with `None` for the first page. Write `all_volumes(client)` that returns every volume across all pages, then `unattached_gib(volumes)` that returns the total `Size` of volumes whose `State` is `"available"`.
@@starter
class FakeEc2:
    """Stands in for boto3: three pages of volumes."""
    PAGES = {
        None: {"Volumes": [{"VolumeId": "vol-1", "Size": 8, "State": "in-use"},
                           {"VolumeId": "vol-2", "Size": 100, "State": "available"}], "NextToken": "p2"},
        "p2": {"Volumes": [{"VolumeId": "vol-3", "Size": 50, "State": "in-use"}], "NextToken": "p3"},
        "p3": {"Volumes": [{"VolumeId": "vol-4", "Size": 20, "State": "available"}]},
    }

    def __init__(self):
        self.calls = 0

    def list_volumes(self, token):
        self.calls += 1
        return self.PAGES[token]


def all_volumes(client):
    pass


def unattached_gib(volumes):
    pass


vols = all_volumes(FakeEc2())
print(len(vols), "volumes,", unattached_gib(vols), "GiB unattached")
@@tests
client = FakeEc2()
vols = all_volumes(client)
check([v["VolumeId"] for v in vols], ["vol-1", "vol-2", "vol-3", "vol-4"], "every page")
check(client.calls, 3, "one call per page")
check(unattached_gib(vols), 120, "100 + 20")
check(unattached_gib([]), 0, "nothing")
@@hint
`token = None`, then loop: `page = client.list_volumes(token)`, extend the result with `page["Volumes"]`, `token = page.get("NextToken")`, and stop when it's `None`.
@@solution
class FakeEc2:
    """Stands in for boto3: three pages of volumes."""
    PAGES = {
        None: {"Volumes": [{"VolumeId": "vol-1", "Size": 8, "State": "in-use"},
                           {"VolumeId": "vol-2", "Size": 100, "State": "available"}], "NextToken": "p2"},
        "p2": {"Volumes": [{"VolumeId": "vol-3", "Size": 50, "State": "in-use"}], "NextToken": "p3"},
        "p3": {"Volumes": [{"VolumeId": "vol-4", "Size": 20, "State": "available"}]},
    }

    def __init__(self):
        self.calls = 0

    def list_volumes(self, token):
        self.calls += 1
        return self.PAGES[token]


def all_volumes(client):
    volumes = []
    token = None
    while True:
        page = client.list_volumes(token)
        volumes.extend(page["Volumes"])
        token = page.get("NextToken")
        if token is None:
            return volumes


def unattached_gib(volumes):
    return sum(v["Size"] for v in volumes if v["State"] == "available")


vols = all_volumes(FakeEc2())
print(len(vols), "volumes,", unattached_gib(vols), "GiB unattached")

@@exercise Retry with exponential backoff
Write `delays(attempts, base=1, cap=20)`: the waits before each retry without jitter, `base * 2**i` capped at `cap`, for `i` from 0 to `attempts - 2` (the first attempt has no wait). Then `call_with_retry(fn, attempts=5, sleep=None)`: call `fn()`; if it raises `Throttled`, wait the next delay (call `sleep(seconds)`) and try again; any other exception goes straight up. If every attempt is throttled, raise the last `Throttled`. Return `fn()`'s result.
@@starter
class Throttled(Exception):
    pass


def delays(attempts, base=1, cap=20):
    pass


def call_with_retry(fn, attempts=5, sleep=None):
    pass


answers = iter([Throttled(), Throttled(), "ok"])


def flaky():
    a = next(answers)
    if isinstance(a, Exception):
        raise a
    return a


waited = []
print(call_with_retry(flaky, sleep=waited.append), "after waiting", waited)
@@tests
check(delays(5), [1, 2, 4, 8], "four waits for five attempts")
check(delays(8, base=1, cap=20), [1, 2, 4, 8, 16, 20, 20], "capped")
check(delays(1), [], "one attempt, no waits")


def make(seq):
    it = iter(seq)

    def fn():
        a = next(it)
        if isinstance(a, Exception):
            raise a
        return a
    return fn


waited = []
check(call_with_retry(make([Throttled(), Throttled(), "ok"]), sleep=waited.append), "ok", "succeeds on the third try")
check(waited, [1, 2], "waited 1 then 2")
waited = []
try:
    call_with_retry(make([Throttled()] * 3), attempts=3, sleep=waited.append)
    check("no error", "Throttled", "gives up")
except Throttled:
    check(waited, [1, 2], "gives up after three attempts")
try:
    call_with_retry(make([PermissionError("AccessDenied")]), sleep=waited.append)
    check("no error", "PermissionError", "not retried")
except PermissionError:
    check(True, True, "other errors are not retried")
@@hint
`delays`: `[min(cap, base * 2 ** i) for i in range(attempts - 1)]`. In `call_with_retry`, loop `for i in range(attempts)`: `try: return fn()`, `except Throttled as e:` remember it, and if this isn't the last attempt, `sleep(waits[i])`. After the loop, `raise` the remembered error.
@@solution
class Throttled(Exception):
    pass


def delays(attempts, base=1, cap=20):
    return [min(cap, base * 2 ** i) for i in range(attempts - 1)]


def call_with_retry(fn, attempts=5, sleep=None):
    waits = delays(attempts)
    last = None
    for i in range(attempts):
        try:
            return fn()
        except Throttled as e:
            last = e
            if i < attempts - 1 and sleep is not None:
                sleep(waits[i])
    raise last


answers = iter([Throttled(), Throttled(), "ok"])


def flaky():
    a = next(answers)
    if isinstance(a, Exception):
        raise a
    return a


waited = []
print(call_with_retry(flaky, sleep=waited.append), "after waiting", waited)

@@exercise A dry-run cleanup planner
Snapshots are dicts with `id`, `age_days` and `tags`. Write `plan_cleanup(snapshots, max_age=30)` that returns a list of `(id, action, reason)`: `("skip", "protected")` when tags have `DoNotDelete` (any value) or `Environment` is `"prod"` (any case); otherwise `("delete", "older than 30 days")` when `age_days > max_age`; otherwise leave it out. Then `run(plan, apply=False)` returns the lines to print: `"WOULD DELETE snap-1 (older than 30 days)"` in a dry run, `"DELETED snap-1 (...)"` with `apply=True`, and `"SKIP snap-2 (protected)"` either way.
@@starter
def plan_cleanup(snapshots, max_age=30):
    pass


def run(plan, apply=False):
    pass


snaps = [
    {"id": "snap-1", "age_days": 90, "tags": {"Environment": "dev"}},
    {"id": "snap-2", "age_days": 400, "tags": {"Environment": "Prod"}},
    {"id": "snap-3", "age_days": 5, "tags": {}},
    {"id": "snap-4", "age_days": 45, "tags": {"DoNotDelete": ""}},
]
print("\n".join(run(plan_cleanup(snaps))))
@@tests
snaps = [
    {"id": "snap-1", "age_days": 90, "tags": {"Environment": "dev"}},
    {"id": "snap-2", "age_days": 400, "tags": {"Environment": "Prod"}},
    {"id": "snap-3", "age_days": 5, "tags": {}},
    {"id": "snap-4", "age_days": 45, "tags": {"DoNotDelete": ""}},
]
p = plan_cleanup(snaps)
check(p, [("snap-1", "delete", "older than 30 days"), ("snap-2", "skip", "protected"), ("snap-4", "skip", "protected")], "the plan")
check(run(p), ["WOULD DELETE snap-1 (older than 30 days)", "SKIP snap-2 (protected)", "SKIP snap-4 (protected)"], "dry run by default")
check(run(p, apply=True)[0], "DELETED snap-1 (older than 30 days)", "apply")
check(plan_cleanup(snaps, max_age=100), [("snap-2", "skip", "protected"), ("snap-4", "skip", "protected")], "a longer retention")
check(plan_cleanup([{"id": "s", "age_days": 30, "tags": {}}]), [], "exactly 30 days is kept")
@@hint
Check protection first, then age. The reason text uses `max_age`: `f"older than {max_age} days"`. In `run`, pick the verb from the action and `apply`.
@@solution
def plan_cleanup(snapshots, max_age=30):
    plan = []
    for s in snapshots:
        tags = s["tags"]
        if "DoNotDelete" in tags or tags.get("Environment", "").lower() == "prod":
            plan.append((s["id"], "skip", "protected"))
        elif s["age_days"] > max_age:
            plan.append((s["id"], "delete", f"older than {max_age} days"))
    return plan


def run(plan, apply=False):
    lines = []
    for sid, action, reason in plan:
        if action == "skip":
            verb = "SKIP"
        else:
            verb = "DELETED" if apply else "WOULD DELETE"
        lines.append(f"{verb} {sid} ({reason})")
    return lines


snaps = [
    {"id": "snap-1", "age_days": 90, "tags": {"Environment": "dev"}},
    {"id": "snap-2", "age_days": 400, "tags": {"Environment": "Prod"}},
    {"id": "snap-3", "age_days": 5, "tags": {}},
    {"id": "snap-4", "age_days": 45, "tags": {"DoNotDelete": ""}},
]
print("\n".join(run(plan_cleanup(snaps))))

@@exercise Summarize a billing export
A billing export is CSV text with the columns `date,service,account,cost`. Write `by_service(csv_text)` that returns a list of `(service, total)` sorted by total, highest first, with totals rounded to 2 decimals, and `daily_spike(csv_text, factor=2)` that returns the dates whose total cost is more than `factor` times the average daily total (rounded comparisons aren't needed), in date order.
@@starter
import csv
import io

EXPORT = """date,service,account,cost
2026-11-01,EC2,prod,120.50
2026-11-01,S3,prod,10.25
2026-11-02,EC2,prod,118.00
2026-11-02,RDS,prod,60.00
2026-11-03,EC2,dev,450.75
2026-11-03,S3,prod,9.80
"""


def by_service(csv_text):
    pass


def daily_spike(csv_text, factor=2):
    pass


for service, total in by_service(EXPORT):
    print(f"{service:5} {total:>8.2f}")
print("spikes:", daily_spike(EXPORT, factor=1.5))
@@tests
check(by_service(EXPORT), [("EC2", 689.25), ("RDS", 60.0), ("S3", 20.05)], "totals per service")
check(daily_spike(EXPORT, factor=1.5), ["2026-11-03"], "the expensive day")
check(daily_spike(EXPORT, factor=2), [], "nothing over double")
check(by_service("date,service,account,cost\n"), [], "empty export")
@@hint
`csv.DictReader(io.StringIO(csv_text))` gives one dict per row; `float(row["cost"])`. Add into dicts keyed by service and by date. Sort with `key=lambda item: item[1], reverse=True`.
@@solution
import csv
import io

EXPORT = """date,service,account,cost
2026-11-01,EC2,prod,120.50
2026-11-01,S3,prod,10.25
2026-11-02,EC2,prod,118.00
2026-11-02,RDS,prod,60.00
2026-11-03,EC2,dev,450.75
2026-11-03,S3,prod,9.80
"""


def by_service(csv_text):
    totals = {}
    for row in csv.DictReader(io.StringIO(csv_text)):
        totals[row["service"]] = totals.get(row["service"], 0) + float(row["cost"])
    return sorted(((s, round(t, 2)) for s, t in totals.items()), key=lambda item: item[1], reverse=True)


def daily_spike(csv_text, factor=2):
    days = {}
    for row in csv.DictReader(io.StringIO(csv_text)):
        days[row["date"]] = days.get(row["date"], 0) + float(row["cost"])
    if not days:
        return []
    average = sum(days.values()) / len(days)
    return sorted(d for d, total in days.items() if total > factor * average)


for service, total in by_service(EXPORT):
    print(f"{service:5} {total:>8.2f}")
print("spikes:", daily_spike(EXPORT, factor=1.5))
