@@lesson tf1-modules-hcp-deep Modules, Registry Versions, Workspaces and HCP Terraform Policies
@@text
The exam devotes whole objectives to modules and to HCP Terraform. This lesson covers module sources and versions, how inputs and outputs cross module boundaries, CLI workspaces versus HCP Terraform workspaces, run workflows, and policy as code with Sentinel and OPA.

## Module sources

```hcl
module "network" {
  source  = "terraform-aws-modules/vpc/aws"     # public registry: <namespace>/<name>/<provider>
  version = "~> 5.16"                            # version only works for registry sources
  cidr    = "10.0.0.0/16"
}

module "app" {
  source = "./modules/app"                       # local path: no version, no download
  vpc_id = module.network.vpc_id                 # child outputs are module.<name>.<output>
}

module "shared" {
  source = "git::https://github.com/acme/tf-modules.git//dns?ref=v1.4.0"   # git with subdirectory and tag
}
```

- Other sources: private registry (`app.terraform.io/acme/vpc/aws`), S3/GCS buckets, HTTP archives.
- `terraform init` (or `terraform get`) downloads modules into `.terraform/modules`. Changing a module source or version needs `init` again.
- A module is any directory of `.tf` files; the working directory is the **root module**. Standard layout: `main.tf`, `variables.tf`, `outputs.tf`, `README.md`, `versions.tf`, `examples/`.

## Module boundaries

- A child module sees **only** the variables you pass in; the parent sees **only** the child's **outputs**.
- Providers are inherited from the parent by default; pass aliased providers with `providers = { aws = aws.us }`.
- `count`, `for_each` and `depends_on` work on module blocks.
- Keep modules small and composable; don't wrap a single resource; pin versions; publish with semantic versioning (registry modules need a repo named `terraform-<PROVIDER>-<NAME>` and version tags).

## Workspaces

- **CLI workspaces** (`terraform workspace new|select|list|show|delete`): several state files for one configuration and backend; `terraform.workspace` in expressions. Good for short-lived copies, not for strongly separated environments (same backend credentials, easy to apply to the wrong one).
- **HCP Terraform workspaces**: each has its own state, variables (Terraform and environment, sensitive), permissions, run history and VCS connection. Often one workspace per environment per component. **Projects** group workspaces.

## HCP Terraform runs

- Workflows: **VCS-driven** (PRs trigger speculative plans; merges trigger runs), **CLI-driven** (`terraform plan` locally, executed remotely: configure with the `cloud` block and `terraform login`), **API-driven**.
- Run stages: plan → **cost estimation** → **policy check** → apply (manual confirm or auto-apply).
- Other features: private registry, run tasks (third-party checks), agents for private networks, drift detection and continuous validation (health), dynamic provider credentials (OIDC), no-code provisioning, team permissions, audit trails.

```hcl
terraform {
  cloud {
    organization = "acme"
    workspaces { tags = ["app:shop"] }
  }
}
```

## Policy as code

**Sentinel** (HashiCorp) or **OPA/Rego** policies inspect the plan. Sentinel enforcement levels:
- **advisory**: failure is logged, run continues.
- **soft-mandatory**: failure stops the run unless a user with permission **overrides** it.
- **hard-mandatory**: failure stops the run; no override.

Typical policies: allowed instance types, mandatory tags, no public buckets, cost limit from cost estimation.

@@quiz
Which module source supports the `version` argument?
- A local path
* A Terraform registry module
- A git URL
- An S3 bucket
? Use ?ref= for git sources.

@@quiz
How does a parent module read the VPC ID created by a child module named `network`?
- var.network.vpc_id
* module.network.vpc_id, if the child declares that output
- data.network.vpc_id
- network.aws_vpc.main.id
? Only outputs cross the boundary.

@@quiz
A Sentinel policy fails at the soft-mandatory level. What happens?
- The run continues with a warning
* The run stops unless an authorised user overrides the failure
- The run stops and can't be overridden
- The workspace is locked permanently
? Advisory only logs; hard-mandatory blocks.

@@quiz
What does `terraform.workspace` return?
- The HCP Terraform organisation
* The name of the current CLI workspace
- The backend bucket
- The root module path
? "default" if you never created one.

@@quiz
In HCP Terraform's VCS-driven workflow, what happens when a pull request is opened?
- The change is applied
* A speculative plan runs and its result is shown on the PR
- Nothing until merge
- The workspace is destroyed
? Merges trigger real runs.

@@quiz
After changing a module's version constraint, what must you run?
* terraform init (to download the new version)
- terraform validate
- terraform state pull
- terraform workspace new
? init installs modules and providers.

@@flashcards
root module :: the working directory
registry source :: namespace/name/provider
version argument :: registry modules only
git source pin :: ?ref=tag
subdirectory :: //path in the source
module outputs :: module.NAME.OUTPUT
child module sees :: only passed variables
providers argument :: pass aliased providers
registry repo name :: terraform-PROVIDER-NAME
CLI workspace :: extra state for same config
terraform.workspace :: current workspace name
HCP workspace :: state, variables, runs, permissions
HCP project :: groups workspaces
cloud block :: connect CLI to HCP Terraform
VCS workflow :: PR = speculative plan
run stages :: plan, cost, policy, apply
Sentinel advisory :: log only
soft-mandatory :: stop unless overridden
hard-mandatory :: stop, no override
dynamic credentials :: OIDC, no static keys

@@drill
Which Sentinel enforcement level can never be overridden?
= hard-mandatory
= hard mandatory
? soft-mandatory can be overridden.

@@drill
Which command lists CLI workspaces?
= terraform workspace list
? new, select, show and delete are the others.

@@exercise Resolve version constraints
Implement Terraform's version constraint matching for module and provider versions. Constraints are comma-separated, with operators `=`, `!=`, `>`, `>=`, `<`, `<=` and `~>` (pessimistic: the last given component may increase, the ones before it are fixed). For each requirement, pick the newest available version that satisfies every constraint, or report none.
@@lang python
@@starter
available = {
    "hashicorp/aws": ["4.67.0", "5.0.0", "5.79.0", "5.80.0", "5.82.2", "5.99.1", "6.0.0", "6.2.0"],
    "terraform-aws-modules/vpc/aws": ["5.8.1", "5.16.0", "5.17.0", "5.21.0", "6.0.1"],
    "hashicorp/random": ["3.5.1", "3.6.0", "3.6.3", "3.7.1"],
}
requirements = [("hashicorp/aws", "~> 5.80"), ("hashicorp/aws", "~> 5.80.0"), ("hashicorp/aws", ">= 5.0, < 6.0, != 5.99.1"),
                ("terraform-aws-modules/vpc/aws", "~> 5.16"), ("hashicorp/random", "3.6.0"), ("hashicorp/random", ">= 4.0"), ("hashicorp/aws", "~> 6.0")]

def parse(version):
    raise NotImplementedError
@@expected
hashicorp/aws "~> 5.80" -> 5.99.1
hashicorp/aws "~> 5.80.0" -> 5.80.0
hashicorp/aws ">= 5.0, < 6.0, != 5.99.1" -> 5.82.2
terraform-aws-modules/vpc/aws "~> 5.16" -> 5.21.0
hashicorp/random "3.6.0" -> 3.6.0
hashicorp/random ">= 4.0" -> no matching version
hashicorp/aws "~> 6.0" -> 6.2.0
@@requires
def parse
def satisfies
"~>"
@@hint
`parse("5.80")` → `(5, 80)`; pad versions to three parts for comparison. For `~> X.Y` the version must be `>= X.Y.0` and have the same major; for `~> X.Y.Z`, `>= X.Y.Z` and the same major and minor. A bare version means `=`.
@@solution
available = {
    "hashicorp/aws": ["4.67.0", "5.0.0", "5.79.0", "5.80.0", "5.82.2", "5.99.1", "6.0.0", "6.2.0"],
    "terraform-aws-modules/vpc/aws": ["5.8.1", "5.16.0", "5.17.0", "5.21.0", "6.0.1"],
    "hashicorp/random": ["3.5.1", "3.6.0", "3.6.3", "3.7.1"],
}
requirements = [("hashicorp/aws", "~> 5.80"), ("hashicorp/aws", "~> 5.80.0"), ("hashicorp/aws", ">= 5.0, < 6.0, != 5.99.1"),
                ("terraform-aws-modules/vpc/aws", "~> 5.16"), ("hashicorp/random", "3.6.0"), ("hashicorp/random", ">= 4.0"), ("hashicorp/aws", "~> 6.0")]

def parse(version):
    return tuple(int(p) for p in version.split("."))

def pad(parts):
    return parts + (0,) * (3 - len(parts))

def satisfies(version, constraint):
    v = pad(parse(version))
    for part in constraint.split(","):
        part = part.strip()
        for op in ("~>", ">=", "<=", "!=", ">", "<", "="):
            if part.startswith(op):
                target = parse(part[len(op):].strip())
                break
        else:
            op, target = "=", parse(part)
        t = pad(target)
        if op == "~>":
            fixed = len(target) - 1
            ok = v >= t and v[:fixed] == t[:fixed]
        else:
            ok = {"=": v == t, "!=": v != t, ">": v > t, ">=": v >= t, "<": v < t, "<=": v <= t}[op]
        if not ok:
            return False
    return True

for name, constraint in requirements:
    matches = [v for v in available[name] if satisfies(v, constraint)]
    best = max(matches, key=parse) if matches else None
    print(f"{name} \"{constraint}\" -> {best or 'no matching version'}")

@@exercise Evaluate policy checks
Simulate an HCP Terraform policy check stage. Each policy inspects the planned resources and has an enforcement level. Print each policy's result, then the run outcome: any hard-mandatory failure stops the run; a soft-mandatory failure stops it unless `override_by` is a user in `can_override`; advisory failures only warn.
@@lang python
@@starter
ALLOWED_TYPES = {"t3.micro", "t3.small", "t3.medium"}
REQUIRED_TAGS = {"owner", "cost-center"}
plan = [
    {"address": "aws_instance.web", "type": "aws_instance", "instance_type": "t3.small", "tags": {"owner": "web", "cost-center": "42"}},
    {"address": "aws_instance.ml", "type": "aws_instance", "instance_type": "p4d.24xlarge", "tags": {"owner": "ml"}},
    {"address": "aws_s3_bucket.public", "type": "aws_s3_bucket", "public": True, "tags": {"owner": "web", "cost-center": "42"}},
]
monthly_cost_delta = 23_800
policies = [("allowed-instance-types", "soft-mandatory"), ("mandatory-tags", "advisory"), ("no-public-buckets", "hard-mandatory"), ("cost-limit-5000", "soft-mandatory")]
runs = [{"name": "run-1", "plan": plan, "override_by": None}, {"name": "run-2", "plan": plan[:2], "override_by": "lead"}, {"name": "run-3", "plan": plan[:2], "override_by": "intern"}]
can_override = {"lead", "platform-admin"}

def check(policy, resources):
    raise NotImplementedError
@@expected
== run-1
  allowed-instance-types [soft-mandatory]: FAILED: aws_instance.ml uses p4d.24xlarge
  mandatory-tags [advisory]: FAILED: aws_instance.ml missing cost-center
  no-public-buckets [hard-mandatory]: FAILED: aws_s3_bucket.public is public
  cost-limit-5000 [soft-mandatory]: FAILED: cost +$23,800/month exceeds $5,000
  outcome: stopped (hard-mandatory failure, no override)
== run-2
  allowed-instance-types [soft-mandatory]: FAILED: aws_instance.ml uses p4d.24xlarge
  mandatory-tags [advisory]: FAILED: aws_instance.ml missing cost-center
  no-public-buckets [hard-mandatory]: passed
  cost-limit-5000 [soft-mandatory]: FAILED: cost +$23,800/month exceeds $5,000
  outcome: apply allowed (soft-mandatory overridden by lead)
== run-3
  allowed-instance-types [soft-mandatory]: FAILED: aws_instance.ml uses p4d.24xlarge
  mandatory-tags [advisory]: FAILED: aws_instance.ml missing cost-center
  no-public-buckets [hard-mandatory]: passed
  cost-limit-5000 [soft-mandatory]: FAILED: cost +$23,800/month exceeds $5,000
  outcome: stopped (soft-mandatory failure; intern can't override)
@@requires
def check
"hard-mandatory"
can_override
@@hint
`check` returns a list of failure messages (empty = pass). For run-2 and run-3 the public bucket isn't in the plan, so only the soft-mandatory and advisory policies fail.
@@solution
ALLOWED_TYPES = {"t3.micro", "t3.small", "t3.medium"}
REQUIRED_TAGS = {"owner", "cost-center"}
plan = [
    {"address": "aws_instance.web", "type": "aws_instance", "instance_type": "t3.small", "tags": {"owner": "web", "cost-center": "42"}},
    {"address": "aws_instance.ml", "type": "aws_instance", "instance_type": "p4d.24xlarge", "tags": {"owner": "ml"}},
    {"address": "aws_s3_bucket.public", "type": "aws_s3_bucket", "public": True, "tags": {"owner": "web", "cost-center": "42"}},
]
monthly_cost_delta = 23_800
policies = [("allowed-instance-types", "soft-mandatory"), ("mandatory-tags", "advisory"), ("no-public-buckets", "hard-mandatory"), ("cost-limit-5000", "soft-mandatory")]
runs = [{"name": "run-1", "plan": plan, "override_by": None}, {"name": "run-2", "plan": plan[:2], "override_by": "lead"}, {"name": "run-3", "plan": plan[:2], "override_by": "intern"}]
can_override = {"lead", "platform-admin"}

def check(policy, resources):
    if policy == "allowed-instance-types":
        return [f"{r['address']} uses {r['instance_type']}" for r in resources if r["type"] == "aws_instance" and r["instance_type"] not in ALLOWED_TYPES]
    if policy == "mandatory-tags":
        return [f"{r['address']} missing {', '.join(sorted(REQUIRED_TAGS - set(r['tags'])))}" for r in resources if REQUIRED_TAGS - set(r["tags"])]
    if policy == "no-public-buckets":
        return [f"{r['address']} is public" for r in resources if r.get("public")]
    if policy == "cost-limit-5000":
        return [f"cost +${monthly_cost_delta:,}/month exceeds $5,000"] if monthly_cost_delta > 5000 else []
    return []

for run in runs:
    print(f"== {run['name']}")
    outcome = "apply allowed"
    soft_failed = False
    for policy, level in policies:
        failures = check(policy, run["plan"])
        print(f"  {policy} [{level}]: {'passed' if not failures else 'FAILED: ' + '; '.join(failures)}")
        if failures and level == "hard-mandatory":
            outcome = "stopped (hard-mandatory failure, no override)"
        elif failures and level == "soft-mandatory":
            soft_failed = True
    if outcome == "apply allowed" and soft_failed:
        who = run["override_by"]
        if who in can_override:
            outcome = f"apply allowed (soft-mandatory overridden by {who})"
        else:
            outcome = "stopped (soft-mandatory failure" + (f"; {who} can't override)" if who else ")")
    print("  outcome:", outcome)
