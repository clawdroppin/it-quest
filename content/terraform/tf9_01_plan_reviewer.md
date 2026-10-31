@@lesson tf9-plan-reviewer Terraform Plan Reviewer
@@text
A plan with 300 lines of `~` updates can hide one `-/+` on the production database, because someone bumped `engine_version` and the provider can't do that in place. People miss it. A program doesn't. You'll write the reviewer that runs in the pipeline between `plan` and `apply`:

```text
Plan: -/+ 2 to replace, ~ 2 to update, + 2 to create, - 1 to delete
  CRITICAL  aws_db_instance.main                             destroys aws_db_instance (data loss)
  HIGH      aws_security_group.web                           open to the internet on port 22
  HIGH      aws_s3_bucket_public_access_block.assets         allows public S3 access
  MEDIUM    aws_instance.web[0]                              replaces aws_instance
  MEDIUM    aws_iam_role.legacy_ci                           deletes aws_iam_role
BLOCK: needs a second reviewer and a backup check
```

It reads the **machine-readable plan**: `terraform show -json plan.tfplan`. Every entry in `resource_changes` has an `address`, a `type`, and a `change` with `actions`, `before` and `after`:

| `actions` | Meaning | In the plan |
|---|---|---|
| `["create"]` | new | `+` |
| `["update"]` | changed in place | `~` |
| `["delete"]` | destroyed | `-` |
| `["delete", "create"]` | replaced, old one first | `-/+` |
| `["create", "delete"]` | replaced with `create_before_destroy` | `+/-` |
| `["read"]`, `["no-op"]` | data source read, nothing to do | |

`plan.json` here is a trimmed example from an AWS project (in a real one, `before` and `after` hold every attribute). The CI job exits with **2** on a BLOCK verdict, so the apply step never runs without a human.

You'll write `action_of`, `summarize` and `risks_of` in `review.py`. `open_ingress` (which ports a security group opens to `0.0.0.0/0` or `::/0`) is already written.
@@project plan_reviewer Terraform Plan Reviewer
Write `action_of`, `summarize` and `risks_of` in `review.py` until `python -m unittest` passes, then `python main.py`. If you have Terraform and a test AWS account (or any provider), run it on a real plan: `terraform show -json plan.tfplan > plan.json`.
@@file review.py
"""Review a Terraform plan: `terraform show -json plan.tfplan > plan.json`, then read it here."""
from dataclasses import dataclass

# Resource types whose loss means lost data. Destroying or replacing one is CRITICAL.
STATEFUL = {
    "aws_db_instance", "aws_rds_cluster", "aws_dynamodb_table", "aws_s3_bucket", "aws_ebs_volume", "aws_efs_file_system",
    "azurerm_mssql_database", "azurerm_storage_account", "azurerm_postgresql_flexible_server",
    "google_sql_database_instance", "google_storage_bucket", "google_bigquery_dataset",
}
SAFE_PUBLIC_PORTS = {80, 443}
OPEN = {"0.0.0.0/0", "::/0"}
ORDER = ["CRITICAL", "HIGH", "MEDIUM"]


@dataclass(frozen=True)
class Risk:
    severity: str
    address: str
    message: str


# HINT: ["delete", "create"] and ["create", "delete"] are both replacements (the order is
# HINT: create_before_destroy). ["no-op"] and ["read"] change nothing.
def action_of(actions: list[str]) -> str:
    """Terraform's action list → one word: "create", "update", "delete", "replace", "read" or "no-op"."""
    raise NotImplementedError


# HINT: a dict comprehension or collections.Counter; skip "no-op" and "read".
def summarize(changes: list[dict]) -> dict[str, int]:
    """Count the resource changes by action, like the last line of `terraform plan`.
    Returns only the actions that occur, among create, update, replace and delete."""
    raise NotImplementedError


def open_ingress(change: dict) -> list[tuple[int, int]]:
    """(from_port, to_port) ranges open to the whole internet in the planned `after` state, for
    aws_security_group (an `ingress` list with cidr_blocks / ipv6_cidr_blocks) and
    aws_vpc_security_group_ingress_rule (cidr_ipv4 / cidr_ipv6)."""
    after = change["change"].get("after") or {}
    if change["type"] == "aws_security_group":
        rules = after.get("ingress") or []
        return [(r["from_port"], r["to_port"]) for r in rules
                if OPEN & set((r.get("cidr_blocks") or []) + (r.get("ipv6_cidr_blocks") or []))]
    if change["type"] == "aws_vpc_security_group_ingress_rule":
        if after.get("cidr_ipv4") in OPEN or after.get("cidr_ipv6") in OPEN:
            return [(after.get("from_port", 0), after.get("to_port", 65535))]
    return []


# HINT: a port range (a, b) is fine only when a == b and that port is in SAFE_PUBLIC_PORTS.
# HINT: "-1" or protocol "all" shows up as from_port 0 / to_port 0 or 65535: treat it like any other range.
def risks_of(change: dict) -> list[Risk]:
    """The risks of one resource change:

    - replace or delete of a STATEFUL type → CRITICAL "destroys <type> (data loss)"
    - delete of anything else → MEDIUM "deletes <type>"
    - replace of anything else → MEDIUM "replaces <type>"
    - each internet-open ingress range other than exactly 80 or 443 (open_ingress), for creates,
      updates and replaces → HIGH "open to the internet on port <a>" or "on ports <a>-<b>"
    - an aws_s3_bucket_public_access_block whose after has any of block_public_acls,
      block_public_policy, ignore_public_acls, restrict_public_buckets set to false → HIGH "allows public S3 access"
    """
    raise NotImplementedError


def review(plan: dict) -> tuple[dict[str, int], list[Risk]]:
    """The summary and every risk, most severe first (in plan order within a severity)."""
    changes = plan.get("resource_changes", [])
    risks = [r for c in changes for r in risks_of(c)]
    return summarize(changes), sorted(risks, key=lambda r: ORDER.index(r.severity))


def verdict(risks: list[Risk]) -> str:
    if any(r.severity == "CRITICAL" for r in risks):
        return "BLOCK: needs a second reviewer and a backup check"
    if any(r.severity == "HIGH" for r in risks):
        return "REVIEW: security-relevant changes"
    return "OK to apply"
@@file test_review.py
import unittest

from review import Risk, action_of, review, risks_of, summarize, verdict


def rc(address, actions, after=None):
    return {"address": address, "type": address.split(".")[0], "change": {"actions": actions, "before": {}, "after": after}}


def sg(address, actions, *rules):
    ingress = [{"from_port": a, "to_port": b, "cidr_blocks": cidrs, "ipv6_cidr_blocks": []} for a, b, cidrs in rules]
    return rc(address, actions, {"ingress": ingress})


class Actions(unittest.TestCase):
    def test_words(self):
        self.assertEqual(action_of(["create"]), "create")
        self.assertEqual(action_of(["update"]), "update")
        self.assertEqual(action_of(["delete"]), "delete")
        self.assertEqual(action_of(["delete", "create"]), "replace")
        self.assertEqual(action_of(["create", "delete"]), "replace")
        self.assertEqual(action_of(["no-op"]), "no-op")
        self.assertEqual(action_of(["read"]), "read")

    def test_summary(self):
        changes = [rc("a.x", ["create"]), rc("a.y", ["create"]), rc("b.z", ["no-op"]),
                   rc("c.w", ["delete", "create"]), rc("d.v", ["read"]), rc("e.u", ["delete"])]
        self.assertEqual(summarize(changes), {"create": 2, "replace": 1, "delete": 1})


class Risks(unittest.TestCase):
    def test_stateful(self):
        self.assertEqual(risks_of(rc("aws_db_instance.main", ["delete", "create"])),
                         [Risk("CRITICAL", "aws_db_instance.main", "destroys aws_db_instance (data loss)")])
        self.assertEqual(risks_of(rc("google_storage_bucket.logs", ["delete"]))[0].severity, "CRITICAL")

    def test_stateless(self):
        self.assertEqual(risks_of(rc("aws_instance.web[0]", ["create", "delete"])),
                         [Risk("MEDIUM", "aws_instance.web[0]", "replaces aws_instance")])
        self.assertEqual(risks_of(rc("aws_iam_role.ci", ["delete"])),
                         [Risk("MEDIUM", "aws_iam_role.ci", "deletes aws_iam_role")])
        self.assertEqual(risks_of(rc("aws_instance.web[1]", ["update"])), [])

    def test_security_group(self):
        c = sg("aws_security_group.web", ["create"],
               (443, 443, ["0.0.0.0/0"]), (22, 22, ["0.0.0.0/0"]), (5432, 5432, ["10.0.0.0/16"]), (8000, 8100, ["0.0.0.0/0"]))
        self.assertEqual([r.message for r in risks_of(c)],
                         ["open to the internet on port 22", "open to the internet on ports 8000-8100"])

    def test_ipv6_and_all_ports(self):
        c = rc("aws_security_group.x", ["update"],
               {"ingress": [{"from_port": 0, "to_port": 0, "cidr_blocks": [], "ipv6_cidr_blocks": ["::/0"]}]})
        self.assertEqual([r.message for r in risks_of(c)], ["open to the internet on port 0"])

    def test_ingress_rule_resource(self):
        c = rc("aws_vpc_security_group_ingress_rule.rdp", ["create"], {"cidr_ipv4": "0.0.0.0/0", "from_port": 3389, "to_port": 3389})
        self.assertEqual(risks_of(c), [Risk("HIGH", "aws_vpc_security_group_ingress_rule.rdp", "open to the internet on port 3389")])

    def test_deleted_open_group_is_not_open(self):
        c = sg("aws_security_group.old", ["delete"], (22, 22, ["0.0.0.0/0"]))
        c["change"]["after"] = None
        self.assertEqual([r.severity for r in risks_of(c)], ["MEDIUM"])

    def test_public_access_block(self):
        bad = rc("aws_s3_bucket_public_access_block.site", ["update"],
                 {"block_public_acls": True, "block_public_policy": False, "ignore_public_acls": True, "restrict_public_buckets": True})
        good = rc("aws_s3_bucket_public_access_block.logs", ["create"],
                  {"block_public_acls": True, "block_public_policy": True, "ignore_public_acls": True, "restrict_public_buckets": True})
        self.assertEqual([r.message for r in risks_of(bad)], ["allows public S3 access"])
        self.assertEqual(risks_of(good), [])


class Review(unittest.TestCase):
    def test_review_and_verdict(self):
        plan = {"resource_changes": [
            rc("aws_instance.web[0]", ["delete", "create"]),
            sg("aws_security_group.web", ["update"], (22, 22, ["0.0.0.0/0"])),
            rc("aws_s3_bucket.data", ["delete"]),
        ]}
        summary, risks = review(plan)
        self.assertEqual(summary, {"replace": 1, "update": 1, "delete": 1})
        self.assertEqual([r.severity for r in risks], ["CRITICAL", "HIGH", "MEDIUM"])
        self.assertTrue(verdict(risks).startswith("BLOCK"))
        self.assertTrue(verdict(risks[1:]).startswith("REVIEW"))
        self.assertEqual(verdict([]), "OK to apply")


if __name__ == "__main__":
    unittest.main()
@@file main.py
"""`python main.py [plan.json]`: print the plan review; exit 2 when it should be blocked.

Make the JSON from a real plan:
  terraform plan -out=plan.tfplan
  terraform show -json plan.tfplan > plan.json"""
import json
import sys
from pathlib import Path

from review import review, verdict

SYMBOL = {"create": "+", "update": "~", "replace": "-/+", "delete": "-"}


def main(argv: list[str]) -> int:
    plan = json.loads(Path(argv[1] if len(argv) > 1 else "plan.json").read_text(encoding="utf-8"))
    summary, risks = review(plan)
    print("Plan: " + ", ".join(f"{SYMBOL[a]} {n} to {a}" for a, n in summary.items()) if summary else "Plan: no changes")
    for r in risks:
        print(f"  {r.severity:<9} {r.address:<48} {r.message}")
    v = verdict(risks)
    print(v)
    return 2 if v.startswith("BLOCK") else 0


if __name__ == "__main__":
    sys.exit(main(sys.argv))
@@file plan.json
{
  "format_version": "1.2",
  "terraform_version": "1.9.8",
  "resource_changes": [
    {
      "address": "aws_instance.web[0]",
      "type": "aws_instance",
      "name": "web",
      "index": 0,
      "change": {
        "actions": ["delete", "create"],
        "before": { "ami": "ami-0a1b2c3d4e5f60001", "instance_type": "t3.micro" },
        "after": { "ami": "ami-0f9e8d7c6b5a40002", "instance_type": "t3.micro" }
      },
      "action_reason": "replace_because_cannot_update"
    },
    {
      "address": "aws_instance.web[1]",
      "type": "aws_instance",
      "name": "web",
      "index": 1,
      "change": {
        "actions": ["update"],
        "before": { "instance_type": "t3.micro", "tags": { "Env": "prod" } },
        "after": { "instance_type": "t3.small", "tags": { "Env": "prod" } }
      }
    },
    {
      "address": "aws_security_group.web",
      "type": "aws_security_group",
      "name": "web",
      "change": {
        "actions": ["update"],
        "before": {
          "ingress": [{ "from_port": 443, "to_port": 443, "protocol": "tcp", "cidr_blocks": ["0.0.0.0/0"], "ipv6_cidr_blocks": [] }]
        },
        "after": {
          "ingress": [
            { "from_port": 443, "to_port": 443, "protocol": "tcp", "cidr_blocks": ["0.0.0.0/0"], "ipv6_cidr_blocks": [] },
            { "from_port": 22, "to_port": 22, "protocol": "tcp", "cidr_blocks": ["0.0.0.0/0"], "ipv6_cidr_blocks": [] }
          ]
        }
      }
    },
    {
      "address": "aws_db_instance.main",
      "type": "aws_db_instance",
      "name": "main",
      "change": {
        "actions": ["delete", "create"],
        "before": { "engine": "postgres", "engine_version": "15.8", "identifier": "shop-prod" },
        "after": { "engine": "postgres", "engine_version": "16.4", "identifier": "shop-prod-v2" }
      },
      "action_reason": "replace_because_cannot_update"
    },
    {
      "address": "aws_s3_bucket.assets",
      "type": "aws_s3_bucket",
      "name": "assets",
      "change": { "actions": ["create"], "before": null, "after": { "bucket": "acme-shop-assets" } }
    },
    {
      "address": "aws_s3_bucket_public_access_block.assets",
      "type": "aws_s3_bucket_public_access_block",
      "name": "assets",
      "change": {
        "actions": ["create"],
        "before": null,
        "after": { "block_public_acls": true, "block_public_policy": false, "ignore_public_acls": true, "restrict_public_buckets": false }
      }
    },
    {
      "address": "aws_iam_role.legacy_ci",
      "type": "aws_iam_role",
      "name": "legacy_ci",
      "change": { "actions": ["delete"], "before": { "name": "legacy-ci" }, "after": null }
    },
    {
      "address": "data.aws_ami.ubuntu",
      "mode": "data",
      "type": "aws_ami",
      "name": "ubuntu",
      "change": { "actions": ["read"], "before": null, "after": {} }
    },
    {
      "address": "aws_vpc.main",
      "type": "aws_vpc",
      "name": "main",
      "change": { "actions": ["no-op"], "before": { "cidr_block": "10.0.0.0/16" }, "after": { "cidr_block": "10.0.0.0/16" } }
    }
  ]
}
@@text
> Tips: `sorted(actions) == ["create", "delete"]` recognizes both replacement orders in one test. For deleted resources `after` is `null` (`None` in Python), which is why `open_ingress` uses `change.get("after") or {}`. Use `is False` for the public access flags: a missing flag isn't the same as `false`.

Then try the real thing in a sandbox: write a small configuration with an S3 bucket and a security group, plan, review, change something that forces replacement (a bucket name), plan again and watch the reviewer catch it.
@@project plan_reviewer_finished Terraform Plan Reviewer: the finished version
The complete reviewer. Ideas: post the review as a pull-request comment from GitHub Actions, read `prevent_destroy`-style rules from a policy file instead of hard-coding `STATEFUL`, require tags on every created resource, or compare `before` and `after` to print exactly which attributes forced a replacement (`action_reason` and `replace_paths` in the plan JSON).
@@file review.py
"""Review a Terraform plan: `terraform show -json plan.tfplan > plan.json`, then read it here."""
from dataclasses import dataclass

# Resource types whose loss means lost data. Destroying or replacing one is CRITICAL.
STATEFUL = {
    "aws_db_instance", "aws_rds_cluster", "aws_dynamodb_table", "aws_s3_bucket", "aws_ebs_volume", "aws_efs_file_system",
    "azurerm_mssql_database", "azurerm_storage_account", "azurerm_postgresql_flexible_server",
    "google_sql_database_instance", "google_storage_bucket", "google_bigquery_dataset",
}
SAFE_PUBLIC_PORTS = {80, 443}
OPEN = {"0.0.0.0/0", "::/0"}
ORDER = ["CRITICAL", "HIGH", "MEDIUM"]


@dataclass(frozen=True)
class Risk:
    severity: str
    address: str
    message: str


# HINT: ["delete", "create"] and ["create", "delete"] are both replacements (the order is
# HINT: create_before_destroy). ["no-op"] and ["read"] change nothing.
def action_of(actions: list[str]) -> str:
    """Terraform's action list → one word: "create", "update", "delete", "replace", "read" or "no-op"."""
    if sorted(actions) == ["create", "delete"]:
        return "replace"
    return actions[0]


# HINT: a dict comprehension or collections.Counter; skip "no-op" and "read".
def summarize(changes: list[dict]) -> dict[str, int]:
    """Count the resource changes by action, like the last line of `terraform plan`.
    Returns only the actions that occur, among create, update, replace and delete."""
    counts: dict[str, int] = {}
    for c in changes:
        a = action_of(c["change"]["actions"])
        if a in ("create", "update", "replace", "delete"):
            counts[a] = counts.get(a, 0) + 1
    return counts


def open_ingress(change: dict) -> list[tuple[int, int]]:
    """(from_port, to_port) ranges open to the whole internet in the planned `after` state, for
    aws_security_group (an `ingress` list with cidr_blocks / ipv6_cidr_blocks) and
    aws_vpc_security_group_ingress_rule (cidr_ipv4 / cidr_ipv6)."""
    after = change["change"].get("after") or {}
    if change["type"] == "aws_security_group":
        rules = after.get("ingress") or []
        return [(r["from_port"], r["to_port"]) for r in rules
                if OPEN & set((r.get("cidr_blocks") or []) + (r.get("ipv6_cidr_blocks") or []))]
    if change["type"] == "aws_vpc_security_group_ingress_rule":
        if after.get("cidr_ipv4") in OPEN or after.get("cidr_ipv6") in OPEN:
            return [(after.get("from_port", 0), after.get("to_port", 65535))]
    return []


# HINT: a port range (a, b) is fine only when a == b and that port is in SAFE_PUBLIC_PORTS.
# HINT: "-1" or protocol "all" shows up as from_port 0 / to_port 0 or 65535: treat it like any other range.
def risks_of(change: dict) -> list[Risk]:
    """The risks of one resource change:

    - replace or delete of a STATEFUL type → CRITICAL "destroys <type> (data loss)"
    - delete of anything else → MEDIUM "deletes <type>"
    - replace of anything else → MEDIUM "replaces <type>"
    - each internet-open ingress range other than exactly 80 or 443 (open_ingress), for creates,
      updates and replaces → HIGH "open to the internet on port <a>" or "on ports <a>-<b>"
    - an aws_s3_bucket_public_access_block whose after has any of block_public_acls,
      block_public_policy, ignore_public_acls, restrict_public_buckets set to false → HIGH "allows public S3 access"
    """
    address, rtype = change["address"], change["type"]
    action = action_of(change["change"]["actions"])
    out = []
    if action in ("replace", "delete"):
        if rtype in STATEFUL:
            out.append(Risk("CRITICAL", address, f"destroys {rtype} (data loss)"))
        else:
            word = "deletes" if action == "delete" else "replaces"
            out.append(Risk("MEDIUM", address, f"{word} {rtype}"))
    if action in ("create", "update", "replace"):
        for a, b in open_ingress(change):
            if a == b and a in SAFE_PUBLIC_PORTS:
                continue
            ports = f"port {a}" if a == b else f"ports {a}-{b}"
            out.append(Risk("HIGH", address, f"open to the internet on {ports}"))
        if rtype == "aws_s3_bucket_public_access_block":
            after = change["change"].get("after") or {}
            flags = ["block_public_acls", "block_public_policy", "ignore_public_acls", "restrict_public_buckets"]
            if any(after.get(f) is False for f in flags):
                out.append(Risk("HIGH", address, "allows public S3 access"))
    return out


def review(plan: dict) -> tuple[dict[str, int], list[Risk]]:
    """The summary and every risk, most severe first (in plan order within a severity)."""
    changes = plan.get("resource_changes", [])
    risks = [r for c in changes for r in risks_of(c)]
    return summarize(changes), sorted(risks, key=lambda r: ORDER.index(r.severity))


def verdict(risks: list[Risk]) -> str:
    if any(r.severity == "CRITICAL" for r in risks):
        return "BLOCK: needs a second reviewer and a backup check"
    if any(r.severity == "HIGH" for r in risks):
        return "REVIEW: security-relevant changes"
    return "OK to apply"
@@file test_review.py
import unittest

from review import Risk, action_of, review, risks_of, summarize, verdict


def rc(address, actions, after=None):
    return {"address": address, "type": address.split(".")[0], "change": {"actions": actions, "before": {}, "after": after}}


def sg(address, actions, *rules):
    ingress = [{"from_port": a, "to_port": b, "cidr_blocks": cidrs, "ipv6_cidr_blocks": []} for a, b, cidrs in rules]
    return rc(address, actions, {"ingress": ingress})


class Actions(unittest.TestCase):
    def test_words(self):
        self.assertEqual(action_of(["create"]), "create")
        self.assertEqual(action_of(["update"]), "update")
        self.assertEqual(action_of(["delete"]), "delete")
        self.assertEqual(action_of(["delete", "create"]), "replace")
        self.assertEqual(action_of(["create", "delete"]), "replace")
        self.assertEqual(action_of(["no-op"]), "no-op")
        self.assertEqual(action_of(["read"]), "read")

    def test_summary(self):
        changes = [rc("a.x", ["create"]), rc("a.y", ["create"]), rc("b.z", ["no-op"]),
                   rc("c.w", ["delete", "create"]), rc("d.v", ["read"]), rc("e.u", ["delete"])]
        self.assertEqual(summarize(changes), {"create": 2, "replace": 1, "delete": 1})


class Risks(unittest.TestCase):
    def test_stateful(self):
        self.assertEqual(risks_of(rc("aws_db_instance.main", ["delete", "create"])),
                         [Risk("CRITICAL", "aws_db_instance.main", "destroys aws_db_instance (data loss)")])
        self.assertEqual(risks_of(rc("google_storage_bucket.logs", ["delete"]))[0].severity, "CRITICAL")

    def test_stateless(self):
        self.assertEqual(risks_of(rc("aws_instance.web[0]", ["create", "delete"])),
                         [Risk("MEDIUM", "aws_instance.web[0]", "replaces aws_instance")])
        self.assertEqual(risks_of(rc("aws_iam_role.ci", ["delete"])),
                         [Risk("MEDIUM", "aws_iam_role.ci", "deletes aws_iam_role")])
        self.assertEqual(risks_of(rc("aws_instance.web[1]", ["update"])), [])

    def test_security_group(self):
        c = sg("aws_security_group.web", ["create"],
               (443, 443, ["0.0.0.0/0"]), (22, 22, ["0.0.0.0/0"]), (5432, 5432, ["10.0.0.0/16"]), (8000, 8100, ["0.0.0.0/0"]))
        self.assertEqual([r.message for r in risks_of(c)],
                         ["open to the internet on port 22", "open to the internet on ports 8000-8100"])

    def test_ipv6_and_all_ports(self):
        c = rc("aws_security_group.x", ["update"],
               {"ingress": [{"from_port": 0, "to_port": 0, "cidr_blocks": [], "ipv6_cidr_blocks": ["::/0"]}]})
        self.assertEqual([r.message for r in risks_of(c)], ["open to the internet on port 0"])

    def test_ingress_rule_resource(self):
        c = rc("aws_vpc_security_group_ingress_rule.rdp", ["create"], {"cidr_ipv4": "0.0.0.0/0", "from_port": 3389, "to_port": 3389})
        self.assertEqual(risks_of(c), [Risk("HIGH", "aws_vpc_security_group_ingress_rule.rdp", "open to the internet on port 3389")])

    def test_deleted_open_group_is_not_open(self):
        c = sg("aws_security_group.old", ["delete"], (22, 22, ["0.0.0.0/0"]))
        c["change"]["after"] = None
        self.assertEqual([r.severity for r in risks_of(c)], ["MEDIUM"])

    def test_public_access_block(self):
        bad = rc("aws_s3_bucket_public_access_block.site", ["update"],
                 {"block_public_acls": True, "block_public_policy": False, "ignore_public_acls": True, "restrict_public_buckets": True})
        good = rc("aws_s3_bucket_public_access_block.logs", ["create"],
                  {"block_public_acls": True, "block_public_policy": True, "ignore_public_acls": True, "restrict_public_buckets": True})
        self.assertEqual([r.message for r in risks_of(bad)], ["allows public S3 access"])
        self.assertEqual(risks_of(good), [])


class Review(unittest.TestCase):
    def test_review_and_verdict(self):
        plan = {"resource_changes": [
            rc("aws_instance.web[0]", ["delete", "create"]),
            sg("aws_security_group.web", ["update"], (22, 22, ["0.0.0.0/0"])),
            rc("aws_s3_bucket.data", ["delete"]),
        ]}
        summary, risks = review(plan)
        self.assertEqual(summary, {"replace": 1, "update": 1, "delete": 1})
        self.assertEqual([r.severity for r in risks], ["CRITICAL", "HIGH", "MEDIUM"])
        self.assertTrue(verdict(risks).startswith("BLOCK"))
        self.assertTrue(verdict(risks[1:]).startswith("REVIEW"))
        self.assertEqual(verdict([]), "OK to apply")


if __name__ == "__main__":
    unittest.main()
@@file main.py
"""`python main.py [plan.json]`: print the plan review; exit 2 when it should be blocked.

Make the JSON from a real plan:
  terraform plan -out=plan.tfplan
  terraform show -json plan.tfplan > plan.json"""
import json
import sys
from pathlib import Path

from review import review, verdict

SYMBOL = {"create": "+", "update": "~", "replace": "-/+", "delete": "-"}


def main(argv: list[str]) -> int:
    plan = json.loads(Path(argv[1] if len(argv) > 1 else "plan.json").read_text(encoding="utf-8"))
    summary, risks = review(plan)
    print("Plan: " + ", ".join(f"{SYMBOL[a]} {n} to {a}" for a, n in summary.items()) if summary else "Plan: no changes")
    for r in risks:
        print(f"  {r.severity:<9} {r.address:<48} {r.message}")
    v = verdict(risks)
    print(v)
    return 2 if v.startswith("BLOCK") else 0


if __name__ == "__main__":
    sys.exit(main(sys.argv))
@@file plan.json
{
  "format_version": "1.2",
  "terraform_version": "1.9.8",
  "resource_changes": [
    {
      "address": "aws_instance.web[0]",
      "type": "aws_instance",
      "name": "web",
      "index": 0,
      "change": {
        "actions": ["delete", "create"],
        "before": { "ami": "ami-0a1b2c3d4e5f60001", "instance_type": "t3.micro" },
        "after": { "ami": "ami-0f9e8d7c6b5a40002", "instance_type": "t3.micro" }
      },
      "action_reason": "replace_because_cannot_update"
    },
    {
      "address": "aws_instance.web[1]",
      "type": "aws_instance",
      "name": "web",
      "index": 1,
      "change": {
        "actions": ["update"],
        "before": { "instance_type": "t3.micro", "tags": { "Env": "prod" } },
        "after": { "instance_type": "t3.small", "tags": { "Env": "prod" } }
      }
    },
    {
      "address": "aws_security_group.web",
      "type": "aws_security_group",
      "name": "web",
      "change": {
        "actions": ["update"],
        "before": {
          "ingress": [{ "from_port": 443, "to_port": 443, "protocol": "tcp", "cidr_blocks": ["0.0.0.0/0"], "ipv6_cidr_blocks": [] }]
        },
        "after": {
          "ingress": [
            { "from_port": 443, "to_port": 443, "protocol": "tcp", "cidr_blocks": ["0.0.0.0/0"], "ipv6_cidr_blocks": [] },
            { "from_port": 22, "to_port": 22, "protocol": "tcp", "cidr_blocks": ["0.0.0.0/0"], "ipv6_cidr_blocks": [] }
          ]
        }
      }
    },
    {
      "address": "aws_db_instance.main",
      "type": "aws_db_instance",
      "name": "main",
      "change": {
        "actions": ["delete", "create"],
        "before": { "engine": "postgres", "engine_version": "15.8", "identifier": "shop-prod" },
        "after": { "engine": "postgres", "engine_version": "16.4", "identifier": "shop-prod-v2" }
      },
      "action_reason": "replace_because_cannot_update"
    },
    {
      "address": "aws_s3_bucket.assets",
      "type": "aws_s3_bucket",
      "name": "assets",
      "change": { "actions": ["create"], "before": null, "after": { "bucket": "acme-shop-assets" } }
    },
    {
      "address": "aws_s3_bucket_public_access_block.assets",
      "type": "aws_s3_bucket_public_access_block",
      "name": "assets",
      "change": {
        "actions": ["create"],
        "before": null,
        "after": { "block_public_acls": true, "block_public_policy": false, "ignore_public_acls": true, "restrict_public_buckets": false }
      }
    },
    {
      "address": "aws_iam_role.legacy_ci",
      "type": "aws_iam_role",
      "name": "legacy_ci",
      "change": { "actions": ["delete"], "before": { "name": "legacy-ci" }, "after": null }
    },
    {
      "address": "data.aws_ami.ubuntu",
      "mode": "data",
      "type": "aws_ami",
      "name": "ubuntu",
      "change": { "actions": ["read"], "before": null, "after": {} }
    },
    {
      "address": "aws_vpc.main",
      "type": "aws_vpc",
      "name": "main",
      "change": { "actions": ["no-op"], "before": { "cidr_block": "10.0.0.0/16" }, "after": { "cidr_block": "10.0.0.0/16" } }
    }
  ]
}
@@text
🎉 You've written policy as code, the same idea as Sentinel and OPA in HCP Terraform, in about a hundred lines of Python. Reading plans carefully, and building tools so others can too, is what makes a team trust its infrastructure as code.
