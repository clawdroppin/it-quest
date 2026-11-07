@@lesson gc9-iam-auditor IAM Policy Auditor
@@text
Most cloud breaches aren't clever hacks: they're a bucket readable by `allUsers`, a contractor's Gmail account with `roles/owner`, or a service account key with `roles/editor` sitting in a Git repository. A cloud architect's first job in a new organization is often an **IAM review**. You'll automate it:

```text
CRITICAL  projects/shop-dev            roles/storage.objectViewer         allAuthenticatedUsers  (public access)
CRITICAL  projects/shop-prod           roles/editor                       serviceAccount:123456789-compute@developer.gserviceaccount.com  (service account with a basic role)
CRITICAL  projects/shop-prod           roles/run.invoker                  allUsers  (public access)
HIGH      projects/data-prod           roles/bigquery.dataViewer          domain:partner.co.jp  (external member)
...
Score 0/100: 3 critical, 5 high, 4 low
```

The input is the JSON that `gcloud projects get-iam-policy PROJECT --format=json` prints, for several resources (an organization and its projects) in one file, `policies.json`. The program exits with code **2** when it finds anything critical, so it can run in a pipeline every night and fail loudly.

Look at that second line: `123456789-compute@developer.gserviceaccount.com` is the **Compute Engine default service account**, which older projects created with `roles/editor`. Every VM that uses it can change almost anything in the project. It's one of the most common findings in real Google Cloud reviews.

Not every finding means "remove it now". `roles/run.invoker` for `allUsers` is how you make a *public website* on Cloud Run, so it may be intentional. An auditor's job is to make every exception visible and deliberate.

You'll write `email_domain`, `check_binding` and `audit` in `audit.py`. The rules are in `check_binding`'s docstring.
@@project iam_auditor IAM Policy Auditor
Write `email_domain`, `check_binding` and `audit` in `audit.py` until `python -m unittest` passes. Then `python main.py` (and `python main.py --domain yourcompany.com` on real exported policies, if you have a Google Cloud project).
@@file audit.py
"""Audit Google Cloud IAM policies (the JSON from `gcloud ... get-iam-policy --format=json`)."""
from dataclasses import dataclass

SEVERITIES = ["CRITICAL", "HIGH", "MEDIUM", "LOW"]
WEIGHT = {"CRITICAL": 25, "HIGH": 10, "MEDIUM": 5, "LOW": 1}
BASIC_ROLES = {"roles/owner", "roles/editor"}
PUBLIC = {"allUsers", "allAuthenticatedUsers"}
GOOGLE_SA_DOMAINS = ("gserviceaccount.com",)


@dataclass(frozen=True)
class Finding:
    severity: str
    resource: str
    role: str
    member: str
    message: str


def member_kind(member: str) -> str:
    """'user:ann@example.com' → 'user', 'allUsers' → 'allUsers'."""
    return member.split(":", 1)[0]


# HINT: the email is everything after the first ":"; the domain is after the last "@".
# HINT: a service account's domain ends with gserviceaccount.com: it isn't "external" by its domain.
def email_domain(member: str) -> str | None:
    """The domain of a user:, group: or serviceAccount: member ('example.com'),
    the domain itself for domain: members, and None for allUsers / allAuthenticatedUsers."""
    raise NotImplementedError


# HINT: check each member of the binding and collect Findings. One member can produce several.
# HINT: a binding with a "condition" is still checked: conditions limit, they don't make a grant safe.
def check_binding(resource: str, binding: dict, domain: str) -> list[Finding]:
    """Findings for one binding {"role": ..., "members": [...]} on `resource`.
    `domain` is the company's own domain, like "example.com". Rules, for each member:

    - allUsers or allAuthenticatedUsers, any role → CRITICAL "public access"
    - a basic role (roles/owner, roles/editor):
        to a service account → CRITICAL "service account with a basic role"
        to anyone else → HIGH "basic role"
    - a user:, group: or domain: member whose domain isn't `domain` → HIGH "external member"
    - a user: member (not a group) with any role → LOW "grant to a group instead"

    Members are compared case-insensitively."""
    raise NotImplementedError


# HINT: sort with a key: (SEVERITIES.index(f.severity), f.resource, f.role, f.member).
def audit(policies: dict[str, dict], domain: str) -> list[Finding]:
    """Check every binding of every policy ({resource: policy}) and return all findings,
    the most severe first, then by resource, role and member."""
    raise NotImplementedError


def score(findings: list[Finding]) -> int:
    """100 minus the weight of every finding (CRITICAL 25, HIGH 10, MEDIUM 5, LOW 1), never below 0."""
    return max(0, 100 - sum(WEIGHT[f.severity] for f in findings))


def report(findings: list[Finding]) -> str:
    lines = []
    for f in findings:
        lines.append(f"{f.severity:<9} {f.resource:<28} {f.role:<34} {f.member}  ({f.message})")
    counts = {s: sum(f.severity == s for f in findings) for s in SEVERITIES}
    summary = ", ".join(f"{n} {s.lower()}" for s, n in counts.items() if n) or "no findings"
    lines.append(f"Score {score(findings)}/100: {summary}")
    return "\n".join(lines)
@@file test_audit.py
import unittest

from audit import Finding, audit, check_binding, email_domain, score

R = "projects/shop-prod"


def msgs(findings):
    return sorted((f.severity, f.member, f.message) for f in findings)


class Domains(unittest.TestCase):
    def test_domains(self):
        self.assertEqual(email_domain("user:Ann@Example.com"), "example.com")
        self.assertEqual(email_domain("group:ops@example.com"), "example.com")
        self.assertEqual(email_domain("serviceAccount:web@shop-prod.iam.gserviceaccount.com"),
                         "shop-prod.iam.gserviceaccount.com")
        self.assertEqual(email_domain("domain:example.com"), "example.com")
        self.assertIsNone(email_domain("allUsers"))


class Bindings(unittest.TestCase):
    def test_public(self):
        b = {"role": "roles/storage.objectViewer", "members": ["allUsers", "allAuthenticatedUsers"]}
        self.assertEqual(msgs(check_binding(R, b, "example.com")), [
            ("CRITICAL", "allAuthenticatedUsers", "public access"),
            ("CRITICAL", "allUsers", "public access"),
        ])

    def test_basic_roles(self):
        b = {"role": "roles/editor", "members": [
            "group:devs@example.com", "serviceAccount:ci@shop-prod.iam.gserviceaccount.com"]}
        self.assertEqual(msgs(check_binding(R, b, "example.com")), [
            ("CRITICAL", "serviceAccount:ci@shop-prod.iam.gserviceaccount.com", "service account with a basic role"),
            ("HIGH", "group:devs@example.com", "basic role"),
        ])

    def test_viewer_is_not_basic_finding(self):
        b = {"role": "roles/viewer", "members": ["group:audit@example.com"]}
        self.assertEqual(check_binding(R, b, "example.com"), [])

    def test_external_user_gets_three(self):
        b = {"role": "roles/owner", "members": ["user:bob@gmail.com"]}
        self.assertEqual(msgs(check_binding(R, b, "example.com")), [
            ("HIGH", "user:bob@gmail.com", "basic role"),
            ("HIGH", "user:bob@gmail.com", "external member"),
            ("LOW", "user:bob@gmail.com", "grant to a group instead"),
        ])

    def test_case_insensitive_domain(self):
        b = {"role": "roles/run.invoker", "members": ["group:Web@EXAMPLE.com", "domain:partner.co.jp"]}
        self.assertEqual(msgs(check_binding(R, b, "Example.com")), [
            ("HIGH", "domain:partner.co.jp", "external member"),
        ])

    def test_findings_carry_context(self):
        f = check_binding(R, {"role": "roles/run.invoker", "members": ["allUsers"]}, "example.com")[0]
        self.assertEqual(f, Finding("CRITICAL", R, "roles/run.invoker", "allUsers", "public access"))


class Audit(unittest.TestCase):
    def test_sorted_and_scored(self):
        policies = {
            "projects/b": {"bindings": [{"role": "roles/logging.viewer", "members": ["user:ann@example.com"]}]},
            "projects/a": {"bindings": [
                {"role": "roles/editor", "members": ["group:devs@example.com"]},
                {"role": "roles/storage.objectViewer", "members": ["allUsers"]},
            ]},
            "projects/empty": {},
        }
        found = audit(policies, "example.com")
        self.assertEqual([(f.severity, f.resource) for f in found],
                         [("CRITICAL", "projects/a"), ("HIGH", "projects/a"), ("LOW", "projects/b")])
        self.assertEqual(score(found), 100 - 25 - 10 - 1)

    def test_score_floor(self):
        self.assertEqual(score([Finding("CRITICAL", R, "r", "m", "x")] * 5), 0)


if __name__ == "__main__":
    unittest.main()
@@file main.py
"""`python main.py [policies.json] [--domain example.com]`: print the audit, exit 2 on critical findings.

policies.json maps resource names to IAM policies. Make one from your own projects with:
  gcloud projects get-iam-policy PROJECT_ID --format=json > p.json
and put it under the key "projects/PROJECT_ID"."""
import json
import sys
from pathlib import Path

from audit import audit, report


def main(argv: list[str]) -> int:
    args = argv[1:]
    domain = "example.com"
    if "--domain" in args:
        i = args.index("--domain")
        domain = args[i + 1]
        del args[i:i + 2]
    file = args[0] if args else "policies.json"
    policies = json.loads(Path(file).read_text(encoding="utf-8"))
    findings = audit(policies, domain)
    print(report(findings))
    return 2 if any(f.severity == "CRITICAL" for f in findings) else 0


if __name__ == "__main__":
    sys.exit(main(sys.argv))
@@file policies.json
{
  "organizations/123456789012": {
    "bindings": [
      { "role": "roles/resourcemanager.organizationAdmin", "members": ["group:cloud-admins@example.com"] },
      { "role": "roles/billing.admin", "members": ["user:cfo@example.com"] }
    ]
  },
  "projects/shop-prod": {
    "bindings": [
      { "role": "roles/owner", "members": ["user:founder@example.com"] },
      { "role": "roles/editor", "members": ["serviceAccount:123456789-compute@developer.gserviceaccount.com"] },
      { "role": "roles/run.invoker", "members": ["allUsers"] },
      { "role": "roles/cloudsql.client", "members": ["serviceAccount:shop-api@shop-prod.iam.gserviceaccount.com"] },
      { "role": "roles/logging.viewer", "members": ["group:sre@example.com", "user:contractor@outsource.co.jp"] }
    ]
  },
  "projects/data-prod": {
    "bindings": [
      { "role": "roles/bigquery.dataViewer", "members": ["group:analysts@example.com", "domain:partner.co.jp"] },
      { "role": "roles/editor", "members": ["group:data-eng@example.com"] },
      {
        "role": "roles/bigquery.admin",
        "members": ["user:tanaka@example.com"],
        "condition": { "title": "until-2026-12-31", "expression": "request.time < timestamp('2027-01-01T00:00:00Z')" }
      }
    ]
  },
  "projects/shop-dev": {
    "bindings": [
      { "role": "roles/editor", "members": ["group:developers@example.com"] },
      { "role": "roles/storage.objectViewer", "members": ["allAuthenticatedUsers"] }
    ]
  }
}
@@text
> Tips: `member.split(":", 1)` separates the kind from the rest; `rsplit("@", 1)[-1]` gets the domain whether or not there's an `@` (as in `domain:partner.co.jp`). Define a tiny `add(severity, message)` helper inside the loop to avoid repeating the `Finding(...)` constructor. `continue` after a public-access finding: that member has no domain to check.

Ideas once it passes:

- An **allowlist file** of accepted exceptions (`projects/shop-prod roles/run.invoker allUsers  # public website`) that suppresses those findings but prints how many were suppressed.
- Flag a **condition** whose expiry date has passed (parse the `timestamp('...')` from the expression).
- Pull the policies live with the Cloud Resource Manager API instead of a file, for every project in a folder.
@@project iam_auditor_finished IAM Policy Auditor: the finished version
The complete auditor. Next steps: the allowlist, expired conditions, checking for service account keys (`gcloud iam service-accounts keys list`), or output as CSV for a spreadsheet the security team can sign off.
@@file audit.py
"""Audit Google Cloud IAM policies (the JSON from `gcloud ... get-iam-policy --format=json`)."""
from dataclasses import dataclass

SEVERITIES = ["CRITICAL", "HIGH", "MEDIUM", "LOW"]
WEIGHT = {"CRITICAL": 25, "HIGH": 10, "MEDIUM": 5, "LOW": 1}
BASIC_ROLES = {"roles/owner", "roles/editor"}
PUBLIC = {"allUsers", "allAuthenticatedUsers"}
GOOGLE_SA_DOMAINS = ("gserviceaccount.com",)


@dataclass(frozen=True)
class Finding:
    severity: str
    resource: str
    role: str
    member: str
    message: str


def member_kind(member: str) -> str:
    """'user:ann@example.com' → 'user', 'allUsers' → 'allUsers'."""
    return member.split(":", 1)[0]


# HINT: the email is everything after the first ":"; the domain is after the last "@".
# HINT: a service account's domain ends with gserviceaccount.com: it isn't "external" by its domain.
def email_domain(member: str) -> str | None:
    """The domain of a user:, group: or serviceAccount: member ('example.com'),
    the domain itself for domain: members, and None for allUsers / allAuthenticatedUsers."""
    if member in PUBLIC:
        return None
    value = member.split(":", 1)[1]
    return value.rsplit("@", 1)[-1].lower()


# HINT: check each member of the binding and collect Findings. One member can produce several.
# HINT: a binding with a "condition" is still checked: conditions limit, they don't make a grant safe.
def check_binding(resource: str, binding: dict, domain: str) -> list[Finding]:
    """Findings for one binding {"role": ..., "members": [...]} on `resource`.
    `domain` is the company's own domain, like "example.com". Rules, for each member:

    - allUsers or allAuthenticatedUsers, any role → CRITICAL "public access"
    - a basic role (roles/owner, roles/editor):
        to a service account → CRITICAL "service account with a basic role"
        to anyone else → HIGH "basic role"
    - a user:, group: or domain: member whose domain isn't `domain` → HIGH "external member"
    - a user: member (not a group) with any role → LOW "grant to a group instead"

    Members are compared case-insensitively."""
    role = binding["role"]
    findings = []
    for member in binding.get("members", []):
        kind = member_kind(member)

        def add(sev, msg):
            findings.append(Finding(sev, resource, role, member, msg))

        if member in PUBLIC:
            add("CRITICAL", "public access")
            continue
        if role in BASIC_ROLES:
            if kind == "serviceAccount":
                add("CRITICAL", "service account with a basic role")
            else:
                add("HIGH", "basic role")
        if kind in ("user", "group", "domain") and email_domain(member) != domain.lower():
            add("HIGH", "external member")
        if kind == "user":
            add("LOW", "grant to a group instead")
    return findings


# HINT: sort with a key: (SEVERITIES.index(f.severity), f.resource, f.role, f.member).
def audit(policies: dict[str, dict], domain: str) -> list[Finding]:
    """Check every binding of every policy ({resource: policy}) and return all findings,
    the most severe first, then by resource, role and member."""
    findings = []
    for resource, policy in policies.items():
        for binding in policy.get("bindings", []):
            findings += check_binding(resource, binding, domain)
    return sorted(findings, key=lambda f: (SEVERITIES.index(f.severity), f.resource, f.role, f.member))


def score(findings: list[Finding]) -> int:
    """100 minus the weight of every finding (CRITICAL 25, HIGH 10, MEDIUM 5, LOW 1), never below 0."""
    return max(0, 100 - sum(WEIGHT[f.severity] for f in findings))


def report(findings: list[Finding]) -> str:
    lines = []
    for f in findings:
        lines.append(f"{f.severity:<9} {f.resource:<28} {f.role:<34} {f.member}  ({f.message})")
    counts = {s: sum(f.severity == s for f in findings) for s in SEVERITIES}
    summary = ", ".join(f"{n} {s.lower()}" for s, n in counts.items() if n) or "no findings"
    lines.append(f"Score {score(findings)}/100: {summary}")
    return "\n".join(lines)
@@file test_audit.py
import unittest

from audit import Finding, audit, check_binding, email_domain, score

R = "projects/shop-prod"


def msgs(findings):
    return sorted((f.severity, f.member, f.message) for f in findings)


class Domains(unittest.TestCase):
    def test_domains(self):
        self.assertEqual(email_domain("user:Ann@Example.com"), "example.com")
        self.assertEqual(email_domain("group:ops@example.com"), "example.com")
        self.assertEqual(email_domain("serviceAccount:web@shop-prod.iam.gserviceaccount.com"),
                         "shop-prod.iam.gserviceaccount.com")
        self.assertEqual(email_domain("domain:example.com"), "example.com")
        self.assertIsNone(email_domain("allUsers"))


class Bindings(unittest.TestCase):
    def test_public(self):
        b = {"role": "roles/storage.objectViewer", "members": ["allUsers", "allAuthenticatedUsers"]}
        self.assertEqual(msgs(check_binding(R, b, "example.com")), [
            ("CRITICAL", "allAuthenticatedUsers", "public access"),
            ("CRITICAL", "allUsers", "public access"),
        ])

    def test_basic_roles(self):
        b = {"role": "roles/editor", "members": [
            "group:devs@example.com", "serviceAccount:ci@shop-prod.iam.gserviceaccount.com"]}
        self.assertEqual(msgs(check_binding(R, b, "example.com")), [
            ("CRITICAL", "serviceAccount:ci@shop-prod.iam.gserviceaccount.com", "service account with a basic role"),
            ("HIGH", "group:devs@example.com", "basic role"),
        ])

    def test_viewer_is_not_basic_finding(self):
        b = {"role": "roles/viewer", "members": ["group:audit@example.com"]}
        self.assertEqual(check_binding(R, b, "example.com"), [])

    def test_external_user_gets_three(self):
        b = {"role": "roles/owner", "members": ["user:bob@gmail.com"]}
        self.assertEqual(msgs(check_binding(R, b, "example.com")), [
            ("HIGH", "user:bob@gmail.com", "basic role"),
            ("HIGH", "user:bob@gmail.com", "external member"),
            ("LOW", "user:bob@gmail.com", "grant to a group instead"),
        ])

    def test_case_insensitive_domain(self):
        b = {"role": "roles/run.invoker", "members": ["group:Web@EXAMPLE.com", "domain:partner.co.jp"]}
        self.assertEqual(msgs(check_binding(R, b, "Example.com")), [
            ("HIGH", "domain:partner.co.jp", "external member"),
        ])

    def test_findings_carry_context(self):
        f = check_binding(R, {"role": "roles/run.invoker", "members": ["allUsers"]}, "example.com")[0]
        self.assertEqual(f, Finding("CRITICAL", R, "roles/run.invoker", "allUsers", "public access"))


class Audit(unittest.TestCase):
    def test_sorted_and_scored(self):
        policies = {
            "projects/b": {"bindings": [{"role": "roles/logging.viewer", "members": ["user:ann@example.com"]}]},
            "projects/a": {"bindings": [
                {"role": "roles/editor", "members": ["group:devs@example.com"]},
                {"role": "roles/storage.objectViewer", "members": ["allUsers"]},
            ]},
            "projects/empty": {},
        }
        found = audit(policies, "example.com")
        self.assertEqual([(f.severity, f.resource) for f in found],
                         [("CRITICAL", "projects/a"), ("HIGH", "projects/a"), ("LOW", "projects/b")])
        self.assertEqual(score(found), 100 - 25 - 10 - 1)

    def test_score_floor(self):
        self.assertEqual(score([Finding("CRITICAL", R, "r", "m", "x")] * 5), 0)


if __name__ == "__main__":
    unittest.main()
@@file main.py
"""`python main.py [policies.json] [--domain example.com]`: print the audit, exit 2 on critical findings.

policies.json maps resource names to IAM policies. Make one from your own projects with:
  gcloud projects get-iam-policy PROJECT_ID --format=json > p.json
and put it under the key "projects/PROJECT_ID"."""
import json
import sys
from pathlib import Path

from audit import audit, report


def main(argv: list[str]) -> int:
    args = argv[1:]
    domain = "example.com"
    if "--domain" in args:
        i = args.index("--domain")
        domain = args[i + 1]
        del args[i:i + 2]
    file = args[0] if args else "policies.json"
    policies = json.loads(Path(file).read_text(encoding="utf-8"))
    findings = audit(policies, domain)
    print(report(findings))
    return 2 if any(f.severity == "CRITICAL" for f in findings) else 0


if __name__ == "__main__":
    sys.exit(main(sys.argv))
@@file policies.json
{
  "organizations/123456789012": {
    "bindings": [
      { "role": "roles/resourcemanager.organizationAdmin", "members": ["group:cloud-admins@example.com"] },
      { "role": "roles/billing.admin", "members": ["user:cfo@example.com"] }
    ]
  },
  "projects/shop-prod": {
    "bindings": [
      { "role": "roles/owner", "members": ["user:founder@example.com"] },
      { "role": "roles/editor", "members": ["serviceAccount:123456789-compute@developer.gserviceaccount.com"] },
      { "role": "roles/run.invoker", "members": ["allUsers"] },
      { "role": "roles/cloudsql.client", "members": ["serviceAccount:shop-api@shop-prod.iam.gserviceaccount.com"] },
      { "role": "roles/logging.viewer", "members": ["group:sre@example.com", "user:contractor@outsource.co.jp"] }
    ]
  },
  "projects/data-prod": {
    "bindings": [
      { "role": "roles/bigquery.dataViewer", "members": ["group:analysts@example.com", "domain:partner.co.jp"] },
      { "role": "roles/editor", "members": ["group:data-eng@example.com"] },
      {
        "role": "roles/bigquery.admin",
        "members": ["user:tanaka@example.com"],
        "condition": { "title": "until-2026-12-31", "expression": "request.time < timestamp('2027-01-01T00:00:00Z')" }
      }
    ]
  },
  "projects/shop-dev": {
    "bindings": [
      { "role": "roles/editor", "members": ["group:developers@example.com"] },
      { "role": "roles/storage.objectViewer", "members": ["allAuthenticatedUsers"] }
    ]
  }
}

@@drill
Which basic role grants full control of a Google Cloud project and should be avoided?
= Owner
= roles/owner
? Prefer predefined roles with least privilege.

@@text
🎉 You've built a real security control. Least privilege, groups instead of people, no public access by accident, no keys: that's the IAM half of the security domain on the Professional Cloud Architect exam, and it's what you'll be asked to defend in architecture reviews.
