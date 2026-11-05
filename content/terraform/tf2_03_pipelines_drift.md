@@lesson tf2-pipelines-drift Running Terraform as a Team: Pipelines, Approvals and Drift
@@text
Once more than one person runs Terraform, the questions change. You stop asking "does my config work?" and start asking "who can apply, what needs approval, and what happens when someone clicks in the console?" This lesson turns the CI/CD ideas from the previous lessons into concrete gate logic and drift handling.

## The pipeline shape

1. **On pull request**: `fmt -check`, `validate`, `tflint`/policy scans, then `plan -out=tfplan`. Post the plan summary as a PR comment.
2. **Review**: a human reads the plan, not just the code. A one-line change can mean `-/+ destroy and then create replacement`.
3. **On merge**: `apply tfplan`, the **saved plan**, so what was reviewed is exactly what runs. If state changed since the plan, Terraform refuses the stale plan.
4. **Credentials**: short-lived cloud credentials through OIDC federation, with plan roles kept read-only and apply roles able to write. No long-lived keys in CI secrets.

## Reading a plan for risk

`terraform show -json tfplan` gives `resource_changes`, each with `change.actions`:

| actions | Symbol | Risk |
|---|---|---|
| `["create"]` | `+` | low |
| `["update"]` | `~` | medium (in-place) |
| `["delete", "create"]` | `-/+` | **high**: replacement, data and IP addresses may be lost |
| `["create", "delete"]` | `+/-` | medium: `create_before_destroy` |
| `["delete"]` | `-` | **high** |

Typical gates: auto-apply plans that only create; require approval for updates; require a second approver, or block, for deletes and replacements of stateful resources (databases, buckets, volumes). HCP Terraform does this with run tasks and Sentinel/OPA policies, with `auto-apply` per workspace.

## Drift

**Drift** means the real infrastructure no longer matches state, usually because of manual changes.

- `terraform plan` refreshes first, then shows how to get back to the config. Drifted attributes appear under "Objects have changed outside of Terraform".
- `terraform plan -refresh-only` / `apply -refresh-only` **accepts** reality into state without changing infrastructure (it replaces the deprecated `terraform refresh`).
- `lifecycle { ignore_changes = [tags["LastScanned"], desired_count] }` tells Terraform to leave attributes that other systems manage, such as autoscalers or scanners.
- A resource **deleted** outside Terraform is planned for **create** again.
- Scheduled drift detection: run `plan -detailed-exitcode` nightly. Exit code 0 means no changes, 2 means changes, 1 means error. HCP Terraform's health assessments do the same.

## Locking and concurrency

Remote backends lock state during operations (S3 with a lock file via `use_lockfile`, HCP Terraform, Consul, azurerm). If a crashed job leaves a lock behind, use `terraform force-unlock LOCK_ID`, and only after confirming nothing is running. Split large estates into several root modules (network, data, apps) so plans stay fast and the blast radius stays small.

@@quiz
Why apply a saved plan file in CI instead of running `terraform apply` again?
* What runs is exactly what was reviewed, and a stale plan is rejected
- Saved plans skip state locking
- It is faster to download providers
- It disables drift detection
? Re-planning at apply time could pick up new changes.

@@quiz
Which command updates state to match real infrastructure without changing any resources?
- terraform apply -replace
* terraform apply -refresh-only
- terraform import
- terraform state rm
? It replaces the deprecated terraform refresh.

@@quiz
`plan -detailed-exitcode` returns 2. What does that mean?
- An error occurred
- No changes
* The plan succeeded and there are changes
- The state is locked
? 0 means no changes, 1 means error.

@@quiz
An autoscaler changes `desired_count` all day, and every plan wants to reset it. What is the cleanest fix?
- Run apply -refresh-only every hour
* Add desired_count to lifecycle ignore_changes
- Remove the resource from state
- Pin the provider version
? Let the other system own that attribute.

@@quiz
In plan JSON, what does `["delete", "create"]` mean?
- Create before destroy
* The resource will be replaced, destroyed first
- An in-place update
- A no-op
? -/+ is the risky one for stateful resources.

@@flashcards
saved plan :: plan -out=tfplan, then apply tfplan
stale plan :: rejected if state changed
OIDC in CI :: short-lived credentials, no stored keys
-/+ :: replace, delete then create
+/- :: create_before_destroy replacement
drift :: real infra differs from state
plan -refresh-only :: show drift only
apply -refresh-only :: accept drift into state
ignore_changes :: leave attributes managed elsewhere
deleted outside Terraform :: planned for create
-detailed-exitcode :: 0 none, 1 error, 2 changes
force-unlock :: remove a stuck lock carefully
blast radius :: split into several root modules

@@drill
Which exit code does `terraform plan -detailed-exitcode` return when there are changes?
= 2
? 0 means no changes.

@@drill
Which lifecycle argument makes Terraform leave attributes that are managed by another system?
= ignore_changes
? It's often used for tags added by scanners or autoscaled counts.

@@exercise Gate a pipeline on the plan
Read a simplified plan JSON (`resource_changes` with actions), work out each change's risk, and decide the gate for the run: auto-apply, one approval, or two approvals plus a block on prod for deleting or replacing stateful resources. Print a PR-comment style summary.
@@concepts json, conditions, maps
@@lang python
@@starter
import json
STATEFUL = ("aws_db_instance", "aws_s3_bucket", "aws_ebs_volume", "aws_dynamodb_table")
plans = {
    "feature/new-queue (dev)": '{"resource_changes": [{"address": "aws_sqs_queue.jobs", "type": "aws_sqs_queue", "change": {"actions": ["create"]}}, {"address": "aws_iam_policy.jobs", "type": "aws_iam_policy", "change": {"actions": ["create"]}}]}',
    "fix/instance-size (staging)": '{"resource_changes": [{"address": "aws_instance.web[0]", "type": "aws_instance", "change": {"actions": ["update"]}}, {"address": "aws_s3_bucket.logs", "type": "aws_s3_bucket", "change": {"actions": ["no-op"]}}]}',
    "chore/rename-db (prod)": '{"resource_changes": [{"address": "aws_db_instance.main", "type": "aws_db_instance", "change": {"actions": ["delete", "create"]}}, {"address": "aws_route53_record.db", "type": "aws_route53_record", "change": {"actions": ["update"]}}]}',
    "feat/blue-green (prod)": '{"resource_changes": [{"address": "aws_lb.app", "type": "aws_lb", "change": {"actions": ["create", "delete"]}}]}',
}

def risk(change):
    raise NotImplementedError
@@expected
feature/new-queue (dev): auto-apply
    + aws_sqs_queue.jobs       low
    + aws_iam_policy.jobs      low
fix/instance-size (staging): 1 approval
    ~ aws_instance.web[0]      medium
chore/rename-db (prod): BLOCKED: stateful replace/delete in prod needs a manual migration plan
  -/+ aws_db_instance.main     critical
    ~ aws_route53_record.db    medium
feat/blue-green (prod): 1 approval
  +/- aws_lb.app               medium
@@requires
def risk
json.loads
STATEFUL
@@hint
Map actions to symbols and risk levels: create → low, update → medium, create+delete → medium, delete or delete+create → high, and critical when the type is stateful. The gate is the worst risk: low means auto-apply, medium means 1 approval, high means 2 approvals, and critical on prod means blocked. Skip no-op changes in the summary.
@@solution
import json
STATEFUL = ("aws_db_instance", "aws_s3_bucket", "aws_ebs_volume", "aws_dynamodb_table")
plans = {
    "feature/new-queue (dev)": '{"resource_changes": [{"address": "aws_sqs_queue.jobs", "type": "aws_sqs_queue", "change": {"actions": ["create"]}}, {"address": "aws_iam_policy.jobs", "type": "aws_iam_policy", "change": {"actions": ["create"]}}]}',
    "fix/instance-size (staging)": '{"resource_changes": [{"address": "aws_instance.web[0]", "type": "aws_instance", "change": {"actions": ["update"]}}, {"address": "aws_s3_bucket.logs", "type": "aws_s3_bucket", "change": {"actions": ["no-op"]}}]}',
    "chore/rename-db (prod)": '{"resource_changes": [{"address": "aws_db_instance.main", "type": "aws_db_instance", "change": {"actions": ["delete", "create"]}}, {"address": "aws_route53_record.db", "type": "aws_route53_record", "change": {"actions": ["update"]}}]}',
    "feat/blue-green (prod)": '{"resource_changes": [{"address": "aws_lb.app", "type": "aws_lb", "change": {"actions": ["create", "delete"]}}]}',
}
SYMBOL = {("create",): "+", ("update",): "~", ("delete",): "-", ("delete", "create"): "-/+", ("create", "delete"): "+/-", ("no-op",): " "}
LEVELS = ["none", "low", "medium", "high", "critical"]

def risk(change):
    actions = tuple(change["change"]["actions"])
    if actions == ("no-op",):
        return "none"
    if actions == ("create",):
        return "low"
    if actions in (("update",), ("create", "delete")):
        return "medium"
    return "critical" if change["type"] in STATEFUL else "high"

for run, raw in plans.items():
    env = run.split("(")[1].rstrip(")")
    changes = [c for c in json.loads(raw)["resource_changes"] if c["change"]["actions"] != ["no-op"]]
    worst = max((risk(c) for c in changes), key=LEVELS.index, default="none")
    if worst == "critical" and env == "prod":
        gate = "BLOCKED: stateful replace/delete in prod needs a manual migration plan"
    else:
        gate = {"none": "nothing to do", "low": "auto-apply", "medium": "1 approval", "high": "2 approvals", "critical": "2 approvals"}[worst]
    print(f"{run}: {gate}")
    for c in changes:
        print(f"  {SYMBOL[tuple(c['change']['actions'])]:>3} {c['address']:<24} {risk(c)}")

@@exercise Classify drift
Compare the config, the state and the real infrastructure for each resource, then say what `terraform plan` will do (revert drift, recreate deleted resources, or nothing because of `ignore_changes`) and which drift should instead be accepted with `-refresh-only` because it is the intended value.
@@concepts maps, conditions, loops
@@lang python
@@starter
resources = {
    "aws_instance.web": {"config": {"instance_type": "t3.small", "tags.Owner": "web"},
                         "state": {"instance_type": "t3.small", "tags.Owner": "web"},
                         "real": {"instance_type": "t3.large", "tags.Owner": "web"}, "ignore": []},
    "aws_ecs_service.api": {"config": {"desired_count": 2}, "state": {"desired_count": 2},
                            "real": {"desired_count": 7}, "ignore": ["desired_count"]},
    "aws_security_group_rule.ssh": {"config": {"cidr": "10.0.0.0/8"}, "state": {"cidr": "10.0.0.0/8"},
                                    "real": None, "ignore": []},
    "aws_s3_bucket.logs": {"config": {"tags.Team": "platform"}, "state": {"tags.Team": "platform"},
                           "real": {"tags.Team": "platform"}, "ignore": []},
}
intended = {("aws_instance.web", "instance_type")}  # emergency resize that should be kept

def diagnose(address, r):
    raise NotImplementedError
@@expected
aws_instance.web
  instance_type: state 't3.small', real 't3.large' -> intended: set instance_type = 't3.large' in config, then apply -refresh-only
aws_ecs_service.api
  desired_count: state 2, real 7 -> ignore_changes, plan: no change
aws_security_group_rule.ssh
  deleted outside Terraform -> plan: + create (it comes back)
aws_s3_bucket.logs
  in sync
@@requires
def diagnose
ignore
refresh-only
@@hint
If `real` is `None`, the object was deleted outside Terraform and will be created again. Otherwise compare each attribute of `real` with `state`. Ignored attributes produce no change. Intended drift should be accepted into the config (update the .tf, then plan shows no change), and any other drift will be reverted to the config by the next apply.
@@solution
resources = {
    "aws_instance.web": {"config": {"instance_type": "t3.small", "tags.Owner": "web"},
                         "state": {"instance_type": "t3.small", "tags.Owner": "web"},
                         "real": {"instance_type": "t3.large", "tags.Owner": "web"}, "ignore": []},
    "aws_ecs_service.api": {"config": {"desired_count": 2}, "state": {"desired_count": 2},
                            "real": {"desired_count": 7}, "ignore": ["desired_count"]},
    "aws_security_group_rule.ssh": {"config": {"cidr": "10.0.0.0/8"}, "state": {"cidr": "10.0.0.0/8"},
                                    "real": None, "ignore": []},
    "aws_s3_bucket.logs": {"config": {"tags.Team": "platform"}, "state": {"tags.Team": "platform"},
                           "real": {"tags.Team": "platform"}, "ignore": []},
}
intended = {("aws_instance.web", "instance_type")}  # emergency resize that should be kept

def diagnose(address, r):
    if r["real"] is None:
        return ["deleted outside Terraform -> plan: + create (it comes back)"]
    notes = []
    for attr, value in r["real"].items():
        if value == r["state"][attr]:
            continue
        drift = f"{attr}: state {r['state'][attr]!r}, real {value!r}"
        if attr in r["ignore"]:
            notes.append(f"{drift} -> ignore_changes, plan: no change")
        elif (address, attr) in intended:
            notes.append(f"{drift} -> intended: set {attr} = {value!r} in config, then apply -refresh-only")
        else:
            notes.append(f"{drift} -> plan: ~ update back to {r['config'][attr]!r}")
    return notes or ["in sync"]

for address, r in resources.items():
    print(address)
    for note in diagnose(address, r):
        print(f"  {note}")
