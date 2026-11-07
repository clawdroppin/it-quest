@@lesson d8-devsecops DevSecOps and Platform Engineering: Secrets, Supply Chain and Policy as Code
@@concepts strings, text-processing, maps
@@lang python
@@text
**DevSecOps** means security is built into the pipeline from the first commit ("shift left"), instead of being a review at the end. **Platform engineering** means a team builds paved roads (templates, pipelines, self-service tools) so every other team gets security, monitoring and deployment right by default. This lesson covers both.

## Secrets management

Secrets are passwords, API keys, tokens, certificates and private keys. The rules:

1. **Never in Git.** Not in code, not in config files, not in Dockerfiles. Add `.env` and `*.pem` to `.gitignore`, and run a **secret scanner** as a pre-commit hook and in CI (gitleaks, trufflehog, GitHub secret scanning with push protection).
2. **A secret store** holds them: HashiCorp **Vault** / OpenBao, **AWS Secrets Manager** / SSM Parameter Store, **Azure Key Vault**, **Google Secret Manager**. Apps fetch secrets at start-up with their own identity (IAM role, managed identity, Kubernetes service account), never with a hard-coded master password.
3. **Short-lived beats long-lived.** Vault's *dynamic secrets* create a database user for one hour; cloud workload identity and OIDC (as in GitHub Actions) replace stored keys entirely.
4. **Rotate** regularly and immediately after any leak. Plan for rotation from day one.
5. In Kubernetes, a plain `Secret` is only base64-encoded. Turn on encryption at rest, restrict who can read Secrets with RBAC, and sync from a real store with the **External Secrets Operator**, or keep encrypted secrets in Git with **SOPS** or **Sealed Secrets**.

## Security checks in the pipeline

| Check | Finds | Tools |
|---|---|---|
| Secret scanning | committed keys and tokens | gitleaks, trufflehog, GitHub push protection |
| **SAST** (static analysis) | dangerous code patterns: SQL injection, eval, weak crypto | Semgrep, CodeQL, Bandit (Python), SonarQube |
| **SCA** (dependency scanning) | libraries with known vulnerabilities (CVEs) | Dependabot, Renovate, `pip-audit`, `npm audit`, OSV-Scanner, Snyk |
| Container scanning | vulnerable OS packages in images | Trivy, Grype, Docker Scout |
| IaC scanning | open security groups, public buckets, no encryption | Checkov, tfsec/Trivy config, KICS |
| **DAST** (dynamic) | vulnerabilities in the running app | OWASP ZAP, Burp Suite |
| Policy as code | rule violations in manifests and plans | OPA/Conftest, Kyverno, Sentinel |

Run fast checks on every PR, slower ones nightly. **Fail the build on new critical findings** but don't let hundreds of old low ones block everyone: track them with a baseline and fix them over time. A scanner that cries wolf gets ignored.

## The software supply chain

Attackers increasingly target the *build*, not the app: a hijacked dependency (event-stream, ua-parser-js), a compromised build tool (SolarWinds, xz-utils, Codecov), a typo-squatted package name (`reqeusts`). Defences:

- **Lock files** (`package-lock.json`, `poetry.lock`, `go.sum`) and pinned versions; review dependency updates like code.
- **SBOM** (Software Bill of Materials): a list of every component in your build, in SPDX or CycloneDX format (`syft`, `trivy sbom`). When the next Log4Shell happens, you can answer "are we affected?" in minutes.
- **Signing**: sign images and artifacts with **Sigstore cosign** (keyless, using the CI's OIDC identity), and only allow signed images into the cluster.
- **Provenance and SLSA**: record *how* and *where* each artifact was built (`actions/attest-build-provenance`); SLSA levels describe how tamper-resistant the build is.
- **Pin CI actions by SHA**, give pipelines least-privilege tokens, and protect the release branch.

## Versions and vulnerabilities

Advisories say which versions are affected, usually as ranges: *introduced in 2.0.0, fixed in 2.17.1*. Versions follow **semantic versioning** (`MAJOR.MINOR.PATCH`) and must be compared **as numbers**: `2.10.0` is newer than `2.9.3`, although as text "2.10" < "2.9". The second exercise does this comparison.

## Policy as code

Write your rules as code, test them, and enforce them automatically:

```rego
# OPA / Conftest (Rego): no containers running as root, no :latest images
package main

deny contains msg if {
  some c in input.spec.template.spec.containers
  not c.securityContext.runAsNonRoot
  msg := sprintf("container %s must set runAsNonRoot", [c.name])
}

deny contains msg if {
  some c in input.spec.template.spec.containers
  endswith(c.image, ":latest")
  msg := sprintf("container %s uses a :latest image", [c.name])
}
```

`conftest test deployment.yaml` runs these in CI; in the cluster, **Kyverno** or **OPA Gatekeeper** reject non-compliant objects at admission time (and Kubernetes' built-in **Pod Security Admission** enforces the `restricted` profile). For Terraform, the same idea checks the plan: no public S3 buckets, required tags, allowed regions. The third exercise is a small policy engine in Python.

## Kubernetes hardening, briefly

`runAsNonRoot: true`, `readOnlyRootFilesystem: true`, `allowPrivilegeEscalation: false`, `capabilities: { drop: [ALL] }`, resource **limits** on every container, **NetworkPolicies** (default deny, then allow what's needed), least-privilege **RBAC**, no service-account token where it isn't needed, images from your own registry only, and audit logging on. CIS Benchmarks and `kube-bench` check a cluster against the standard.

## Platform engineering

As companies grow, every team reinventing pipelines, Terraform modules, dashboards and Kubernetes manifests is slow and inconsistent. A **platform team** builds an **Internal Developer Platform**:

- **Golden paths**: a template creates a new service with the repo, CI/CD, Dockerfile, Helm chart, dashboards, alerts and security scans already wired up.
- A **developer portal** such as **Backstage** shows a catalog of every service, its owner, docs, on-call and health, and offers self-service actions ("create a database", "new service").
- **Self-service infrastructure** through approved Terraform modules or Crossplane, with guardrails (policy as code, budgets) instead of tickets.
- The platform is treated as a **product**: its users are developers, and success is measured by adoption, lead time and satisfaction (and the DORA metrics from module 1).

## Where DevOps, SRE and CloudOps meet

| Role | Focus |
|---|---|
| **DevOps engineer** | pipelines, IaC, containers, automation, developer workflow |
| **SRE** | reliability: SLOs, error budgets, incident response, toil reduction, capacity |
| **CloudOps / cloud engineer** | running cloud infrastructure: accounts, networking, security, cost, backups |
| **Platform engineer** | building the internal platform the others use |
| **DevSecOps / cloud security** | security tooling and guardrails throughout the lifecycle |

The skills overlap heavily; job titles vary by company. Git, Linux, a scripting language, Docker, Kubernetes, Terraform, CI/CD, one cloud and observability are the shared core.

@@quiz
A developer pushed an AWS access key to a public repository and deleted it in the next commit. Is it safe now?
- Yes, the file is gone
* No: it's still in the history and bots scan GitHub within minutes; the key must be revoked and rotated
- Yes, if the repository is made private
- Only if the commit is amended
? Treat any committed secret as compromised.

@@quiz
What does an SBOM help you with?
- It makes builds faster
* It lists every component in your software, so you can quickly tell if a newly announced vulnerability affects you
- It encrypts your dependencies
- It replaces lock files
? Formats: SPDX and CycloneDX. Tools: syft, trivy.

@@quiz
Which comparison of versions is correct?
- "2.10.0" < "2.9.3" because 1 < 9
* 2.10.0 is newer than 2.9.3, because versions compare number by number
- They're equal
- Only the major version matters
? Split on dots and compare the parts as integers.

@@quiz
What does Kyverno or OPA Gatekeeper do in a cluster?
- Monitors CPU usage
* Checks objects against policies when they're created, and rejects those that break the rules
- Backs up etcd
- Signs container images
? Policy as code at admission time, with the same rules you test in CI.

@@quiz
What's a "golden path" in platform engineering?
- The fastest network route between regions
* A supported, ready-made way to build and run a service, with CI/CD, security and monitoring built in
- A list of senior engineers
- A Kubernetes ingress rule
? Teams can leave the path, but following it gives them everything for free.

@@drill
What does SBOM stand for?
= software bill of materials
? A list of every component in your software.

@@flashcards
shift left :: find security problems early, in the IDE and CI
secret scanner :: gitleaks, trufflehog, GitHub push protection
secret store :: Vault, AWS Secrets Manager, Azure Key Vault, Google Secret Manager
dynamic secrets :: credentials created on demand that expire soon
External Secrets Operator :: syncs secrets from a store into Kubernetes
SOPS / Sealed Secrets :: encrypted secrets that can live in Git
SAST :: static code analysis: Semgrep, CodeQL, Bandit
SCA :: known-vulnerable dependencies: Dependabot, pip-audit, OSV
DAST :: testing the running app: OWASP ZAP
IaC scanning :: Checkov, tfsec/Trivy for Terraform and manifests
SBOM :: bill of materials: every component, in SPDX or CycloneDX
cosign :: sign and verify container images (Sigstore)
SLSA :: levels of build integrity and provenance
semantic versioning :: MAJOR.MINOR.PATCH, compared numerically
policy as code :: rules as tested code: OPA/Rego, Conftest, Kyverno, Sentinel
Pod Security Admission :: built-in privileged / baseline / restricted profiles
NetworkPolicy :: firewall rules between pods
Internal Developer Platform :: self-service tools and golden paths for developers
Backstage :: an open-source developer portal and service catalog
CIS Benchmark / kube-bench :: hardening standards and their checker

@@exercise A secret scanner
Write `scan(text)` that returns a list of `(line_number, kind)` for every secret found, in line order (line numbers start at 1; at most one finding per line, using the first rule that matches):

- `"aws-access-key"`: `AKIA` followed by exactly 16 upper-case letters or digits
- `"github-token"`: `ghp_` followed by 36 letters or digits
- `"private-key"`: a line containing `-----BEGIN` and `PRIVATE KEY-----`
- `"password"`: `password` (any case) followed by optional spaces, `=` or `:`, optional spaces, and a quoted value of at least 8 characters

Lines containing `# scan:ignore` are skipped.
@@starter
import re

CODE = """import boto3
AWS_KEY = "AKIAIOSFODNN7EXAMPLE"
token = 'ghp_abcdefghijklmnopqrstuvwxyz0123456789'
db_password = "correct-horse-battery"
password = "short"
-----BEGIN RSA PRIVATE KEY-----
test_key = "AKIAABCDEFGHIJKLMNOP"  # scan:ignore
"""


def scan(text):
    pass


for finding in scan(CODE):
    print(finding)
@@tests
check(scan(CODE), [(2, "aws-access-key"), (3, "github-token"), (4, "password"), (6, "private-key")], "the sample file")
check(scan("PASSWORD: 'hunter22hunter'"), [(1, "password")], "upper case and a colon")
check(scan("x = 'AKIA1234'"), [], "too short to be a key")
check(scan("print('hello')\n"), [], "nothing to find")
@@hint
Keep a list of `(kind, compiled_regex)` in rule order, e.g. `re.compile(r"AKIA[A-Z0-9]{16}")`, `re.compile(r"-----BEGIN .*PRIVATE KEY-----")` and `re.compile(r"password\s*[=:]\s*['\"][^'\"]{8,}['\"]", re.I)`. For each line, take the first rule whose `search` matches.
@@solution
import re

CODE = """import boto3
AWS_KEY = "AKIAIOSFODNN7EXAMPLE"
token = 'ghp_abcdefghijklmnopqrstuvwxyz0123456789'
db_password = "correct-horse-battery"
password = "short"
-----BEGIN RSA PRIVATE KEY-----
test_key = "AKIAABCDEFGHIJKLMNOP"  # scan:ignore
"""

RULES = [
    ("aws-access-key", re.compile(r"AKIA[A-Z0-9]{16}")),
    ("github-token", re.compile(r"ghp_[A-Za-z0-9]{36}")),
    ("private-key", re.compile(r"-----BEGIN.*PRIVATE KEY-----")),
    ("password", re.compile(r"password\s*[=:]\s*['\"][^'\"]{8,}['\"]", re.IGNORECASE)),
]


def scan(text):
    found = []
    for n, line in enumerate(text.splitlines(), start=1):
        if "# scan:ignore" in line:
            continue
        for kind, rule in RULES:
            if rule.search(line):
                found.append((n, kind))
                break
    return found


for finding in scan(CODE):
    print(finding)

@@exercise Which installed packages are vulnerable?
Write `parse_version(v)` that turns `"2.17.1"` into the tuple `(2, 17, 1)` (any number of parts), and `affected(installed, advisories)`. `installed` maps package names to versions; `advisories` is a list of dicts with `package`, `id`, `introduced` and `fixed` (a version is affected when `introduced <= version < fixed`). Return a sorted list of strings `"package version: id (fixed in X)"`.
@@starter
def parse_version(v):
    pass


def affected(installed, advisories):
    pass


installed = {"log4j-core": "2.14.1", "requests": "2.32.3", "jinja2": "3.1.2"}
advisories = [
    {"package": "log4j-core", "id": "CVE-2021-44228", "introduced": "2.0.0", "fixed": "2.15.0"},
    {"package": "requests", "id": "CVE-2023-32681", "introduced": "2.3.0", "fixed": "2.31.0"},
    {"package": "jinja2", "id": "CVE-2024-22195", "introduced": "0.0.0", "fixed": "3.1.3"},
]
for line in affected(installed, advisories):
    print(line)
@@tests
check(parse_version("2.17.1"), (2, 17, 1), "three parts")
check(parse_version("10.0"), (10, 0), "two parts")
check(parse_version("2.10.0") > parse_version("2.9.3"), True, "numeric comparison")
installed = {"log4j-core": "2.14.1", "requests": "2.32.3", "jinja2": "3.1.2"}
advisories = [
    {"package": "log4j-core", "id": "CVE-2021-44228", "introduced": "2.0.0", "fixed": "2.15.0"},
    {"package": "requests", "id": "CVE-2023-32681", "introduced": "2.3.0", "fixed": "2.31.0"},
    {"package": "jinja2", "id": "CVE-2024-22195", "introduced": "0.0.0", "fixed": "3.1.3"},
]
check(affected(installed, advisories), ["jinja2 3.1.2: CVE-2024-22195 (fixed in 3.1.3)", "log4j-core 2.14.1: CVE-2021-44228 (fixed in 2.15.0)"], "two affected packages")
check(affected({"x": "2.15.0"}, [{"package": "x", "id": "A", "introduced": "2.0.0", "fixed": "2.15.0"}]), [], "the fixed version is safe")
check(affected({"x": "1.9.9"}, [{"package": "x", "id": "A", "introduced": "2.0.0", "fixed": "2.15.0"}]), [], "older than introduced")
check(affected({}, advisories), [], "nothing installed")
@@hint
`tuple(int(p) for p in v.split("."))`. For each advisory whose package is installed, compare `parse_version(introduced) <= parse_version(version) < parse_version(fixed)`.
@@solution
def parse_version(v):
    return tuple(int(p) for p in v.split("."))


def affected(installed, advisories):
    found = []
    for adv in advisories:
        version = installed.get(adv["package"])
        if version is None:
            continue
        if parse_version(adv["introduced"]) <= parse_version(version) < parse_version(adv["fixed"]):
            found.append(f"{adv['package']} {version}: {adv['id']} (fixed in {adv['fixed']})")
    return sorted(found)


installed = {"log4j-core": "2.14.1", "requests": "2.32.3", "jinja2": "3.1.2"}
advisories = [
    {"package": "log4j-core", "id": "CVE-2021-44228", "introduced": "2.0.0", "fixed": "2.15.0"},
    {"package": "requests", "id": "CVE-2023-32681", "introduced": "2.3.0", "fixed": "2.31.0"},
    {"package": "jinja2", "id": "CVE-2024-22195", "introduced": "0.0.0", "fixed": "3.1.3"},
]
for line in affected(installed, advisories):
    print(line)

@@exercise Policy as code for Deployments
Write `violations(deployment)` for a Kubernetes Deployment given as a dict. For each container in `spec.template.spec.containers`, in order, check these rules in this order and collect the messages:

1. the image has no tag or ends with `:latest` → `"<name>: pin the image version"`
2. `securityContext.runAsNonRoot` isn't `True` → `"<name>: must run as non-root"`
3. `securityContext.privileged` is `True` → `"<name>: privileged is not allowed"`
4. `resources.limits` has no `memory` → `"<name>: set a memory limit"`

Missing sections count as not set.
@@starter
def violations(deployment):
    pass


deployment = {
    "kind": "Deployment",
    "spec": {"template": {"spec": {"containers": [
        {"name": "web", "image": "ghcr.io/acme/web:1.4.2",
         "securityContext": {"runAsNonRoot": True},
         "resources": {"limits": {"memory": "256Mi", "cpu": "500m"}}},
        {"name": "sidecar", "image": "busybox", "securityContext": {"privileged": True}},
    ]}}},
}
for v in violations(deployment):
    print(v)
@@tests
deployment = {
    "kind": "Deployment",
    "spec": {"template": {"spec": {"containers": [
        {"name": "web", "image": "ghcr.io/acme/web:1.4.2",
         "securityContext": {"runAsNonRoot": True},
         "resources": {"limits": {"memory": "256Mi", "cpu": "500m"}}},
        {"name": "sidecar", "image": "busybox", "securityContext": {"privileged": True}},
    ]}}},
}
check(violations(deployment), ["sidecar: pin the image version", "sidecar: must run as non-root", "sidecar: privileged is not allowed", "sidecar: set a memory limit"], "a bad sidecar")
ok = {"spec": {"template": {"spec": {"containers": [{"name": "a", "image": "x:1", "securityContext": {"runAsNonRoot": True}, "resources": {"limits": {"memory": "1Gi"}}}]}}}}
check(violations(ok), [], "a compliant deployment")
check(violations({"spec": {"template": {"spec": {"containers": [{"name": "b", "image": "localhost:5000/b:latest"}]}}}}),
      ["b: pin the image version", "b: must run as non-root", "b: set a memory limit"], ":latest and nothing set")
@@hint
`containers = deployment["spec"]["template"]["spec"]["containers"]`. Use `c.get("securityContext", {})` and `c.get("resources", {}).get("limits", {})` so missing sections are empty dicts. For the tag, look only after the last `/`.
@@solution
def violations(deployment):
    messages = []
    for c in deployment["spec"]["template"]["spec"]["containers"]:
        name = c["name"]
        last = c["image"].rsplit("/", 1)[-1]
        sec = c.get("securityContext", {})
        limits = c.get("resources", {}).get("limits", {})
        if ":" not in last or last.endswith(":latest"):
            messages.append(f"{name}: pin the image version")
        if sec.get("runAsNonRoot") is not True:
            messages.append(f"{name}: must run as non-root")
        if sec.get("privileged") is True:
            messages.append(f"{name}: privileged is not allowed")
        if "memory" not in limits:
            messages.append(f"{name}: set a memory limit")
    return messages


deployment = {
    "kind": "Deployment",
    "spec": {"template": {"spec": {"containers": [
        {"name": "web", "image": "ghcr.io/acme/web:1.4.2",
         "securityContext": {"runAsNonRoot": True},
         "resources": {"limits": {"memory": "256Mi", "cpu": "500m"}}},
        {"name": "sidecar", "image": "busybox", "securityContext": {"privileged": True}},
    ]}}},
}
for v in violations(deployment):
    print(v)
