@@lesson tf3-providers-multi-env Providers at Scale: Aliases, Regions and Accounts, the Lock File, Environments and Cost
@@concepts maps, functions, numbers
@@lang python
@@text
A first Terraform project has one provider, one region and one environment. Real ones deploy to several regions (for latency or disaster recovery), several accounts or subscriptions (dev, staging, prod, a shared networking account), and need to know what a change will cost before anyone approves it. This lesson covers the provider features that make that possible, the dependency lock file, the common ways to lay out environments, and cost estimation in the pipeline.

## Provider requirements and the lock file

- Declare every provider in `required_providers` with a **source** and a **version constraint**:

```hcl
terraform {
  required_version = ">= 1.6"
  required_providers {
    aws = { source = "hashicorp/aws", version = "~> 5.40" }
  }
}
```

- `terraform init` picks the newest version that matches and records it, with checksums, in **`.terraform.lock.hcl`**. **Commit the lock file**: everyone and every pipeline then use exactly the same provider build.
- `terraform init -upgrade` moves to the newest allowed versions and updates the lock file. Do it on purpose, in its own pull request.
- The lock file stores hashes per platform. If the team uses macOS, Windows and Linux CI, run `terraform providers lock -platform=linux_amd64 -platform=darwin_arm64 -platform=windows_amd64` so all of them verify.
- Constraint operators: `=` (exact), `!=`, `>`, `>=`, `<`, `<=`, and `~>` (pessimistic): `~> 5.40` allows `>= 5.40, < 6.0`, and `~> 5.40.1` allows `>= 5.40.1, < 5.41`.

## Aliases: several configurations of one provider

```hcl
provider "aws" { region = "ap-northeast-1" }            # default (Tokyo)
provider "aws" {
  alias  = "osaka"
  region = "ap-northeast-3"
}
provider "aws" {
  alias  = "prod"
  region = "ap-northeast-1"
  assume_role { role_arn = "arn:aws:iam::111122223333:role/terraform-apply" }
}

resource "aws_s3_bucket" "replica" {
  provider = aws.osaka
  bucket   = "acme-backup-osaka"
}
```

- Resources without `provider =` use the **default** (unaliased) configuration.
- **Modules inherit the default provider** implicitly. Aliased configurations must be passed explicitly: `providers = { aws = aws.osaka }`, or for a module that needs two, `providers = { aws.primary = aws, aws.dr = aws.osaka }`. The module declares them with `configuration_aliases = [aws.primary, aws.dr]` in its `required_providers`.
- Provider references in a module's `providers` map must be static, so a `for_each` module can't pick a different provider for each instance. Use one module block per region, or generate them with a wrapper tool.
- `assume_role` (AWS), separate `subscription_id` values (Azure), or separate `project` values (GCP) give each configuration its own account or project.

## Laying out environments

| Approach | How | Good for | Watch out |
|---|---|---|---|
| **Directory per environment** | `envs/dev`, `envs/prod`, each with its own backend and variables, calling shared modules | Clear separation, different settings per environment | Some duplication |
| **Workspaces** | One configuration, `terraform workspace select prod` | Identical, short-lived copies (feature environments) | Easy to apply to the wrong workspace, and one backend for all |
| **HCP Terraform workspaces / stacks** | One workspace per environment with its own variables and permissions | Teams on HCP Terraform | Licence cost |
| **Wrappers** (Terragrunt) | DRY configuration across many environments and regions | Large estates | Another tool to learn |

Most teams use **directories per environment** and shared, versioned modules. Production gets its own state, its own credentials and its own approval step.

## Cost before you apply

- **Infracost** reads the plan JSON, prices each resource from cloud price lists, and comments the monthly difference on the pull request ("+¥48,000/month"). HCP Terraform has built-in cost estimation for some resources.
- Put a **cost gate** in policy as code: require extra approval when the monthly increase exceeds a threshold.
- **Tag everything** (`default_tags` in the AWS provider, the `tags` argument elsewhere) with owner, environment and cost centre, so the bill can be split per team afterwards.
- Usage-based services (data transfer, requests, Lambda invocations) can't be priced from the plan alone. Give Infracost a usage file with estimates.

@@quiz
Should `.terraform.lock.hcl` be committed to version control?
* Yes, so everyone and every pipeline use the same provider versions and checksums
- No, it's generated on every run
- Only in production
- Only if it contains secrets
? Update it on purpose with init -upgrade.

@@quiz
What does `version = "~> 5.40"` allow?
* 5.40 and later 5.x versions, but not 6.0
- Only 5.40.0
- Any version above 5.40, including 6.x
- 5.40.x only
? The rightmost number can increase.

@@quiz
A module creates resources in two regions. How does it get the aliased provider configurations?
* The calling module passes them in a providers map, and the module declares configuration_aliases
- Modules automatically inherit every alias
- With a data source
- Using terraform workspace
? Only the default configuration is inherited implicitly.

@@quiz
A resource has no `provider` argument. Which configuration does it use?
* The default (unaliased) configuration of that provider
- The first alias in the file
- None, which is an error
- A random alias
? Set provider = aws.alias to change it.

@@quiz
What is the main risk of using CLI workspaces for dev and prod?
* It's easy to apply to the wrong workspace, and they share one backend and its access rules
- Workspaces don't support variables
- Workspaces can't use modules
- Workspaces are deprecated
? Directories per environment separate credentials and state.

@@quiz
What does Infracost add to a pull request?
* An estimate of the monthly cost change from the plan
- A security scan
- A state backup
- A provider upgrade
? Usage-based costs need a usage file.

@@flashcards
required_providers :: source and version constraint
.terraform.lock.hcl :: exact provider versions and checksums
init -upgrade :: move to the newest allowed versions
providers lock -platform :: hashes for other operating systems
~> 5.40 :: >= 5.40, < 6.0
~> 5.40.1 :: >= 5.40.1, < 5.41
alias :: extra configuration of a provider
provider = aws.osaka :: use an aliased configuration
providers map :: pass aliases into a module
configuration_aliases :: module declares the aliases it needs
assume_role :: a provider configuration per AWS account
directory per environment :: separate state, credentials and variables
Terragrunt :: wrapper for DRY multi-environment setups
Infracost :: cost estimate from the plan
default_tags :: tags on every AWS resource

@@drill
Does `~> 5.40.1` allow version 5.41.0? (yes/no)
= no
= No
? Only 5.40.x from 5.40.1.

@@drill
Which file records the selected provider versions and their checksums? (file name)
= .terraform.lock.hcl
= terraform.lock.hcl
? Commit it.

@@exercise Which provider configuration does each resource use?
Resolve the provider configuration for every resource, in the root module and in child modules. Root resources use their `provider` argument or the default. A child module inherits the default configuration implicitly, and any aliases only through its `providers` map; a resource in a module that refers to an alias the module wasn't given is an error. Print the effective region of each resource.
@@starter
providers = {"aws": "ap-northeast-1", "aws.osaka": "ap-northeast-3", "aws.us": "us-east-1"}
root_resources = {
    "aws_s3_bucket.main": None,
    "aws_s3_bucket.replica": "aws.osaka",
    "aws_acm_certificate.cdn": "aws.us",
}
modules = {
    "network": {"providers": {}, "resources": {"aws_vpc.this": None}},
    "dr": {"providers": {"aws": "aws.osaka"}, "resources": {"aws_vpc.this": None, "aws_db_instance.replica": None}},
    "edge": {"providers": {"aws.primary": "aws", "aws.us": "aws.us"},
             "resources": {"aws_cloudfront_distribution.cdn": "aws.primary", "aws_acm_certificate.cert": "aws.us",
                           "aws_wafv2_web_acl.waf": "aws.global"}},
}

def resolve(module_providers, ref):
    raise NotImplementedError

for address, ref in root_resources.items():
    print(f"{address:<46} {providers[ref or 'aws']}")
for name, m in modules.items():
    for address, ref in m["resources"].items():
        print(f"module.{name}.{address:<{39 - len(name)}} {resolve(m['providers'], ref)}")
@@expected
aws_s3_bucket.main                             ap-northeast-1
aws_s3_bucket.replica                          ap-northeast-3
aws_acm_certificate.cdn                        us-east-1
module.network.aws_vpc.this                     ap-northeast-1
module.dr.aws_vpc.this                          ap-northeast-3
module.dr.aws_db_instance.replica               ap-northeast-3
module.edge.aws_cloudfront_distribution.cdn     ap-northeast-1
module.edge.aws_acm_certificate.cert            us-east-1
module.edge.aws_wafv2_web_acl.waf               ERROR: aws.global not passed to the module
@@requires
def resolve
module_providers.get(
@@hint
Inside a module, the local name is `ref or "aws"`. If the module's `providers` map has that name, look up the parent's configuration it maps to. If it doesn't, the default `"aws"` is inherited from the root implicitly; any other name is an error such as `"ERROR: aws.global not passed to the module"`.
@@solution
providers = {"aws": "ap-northeast-1", "aws.osaka": "ap-northeast-3", "aws.us": "us-east-1"}
root_resources = {
    "aws_s3_bucket.main": None,
    "aws_s3_bucket.replica": "aws.osaka",
    "aws_acm_certificate.cdn": "aws.us",
}
modules = {
    "network": {"providers": {}, "resources": {"aws_vpc.this": None}},
    "dr": {"providers": {"aws": "aws.osaka"}, "resources": {"aws_vpc.this": None, "aws_db_instance.replica": None}},
    "edge": {"providers": {"aws.primary": "aws", "aws.us": "aws.us"},
             "resources": {"aws_cloudfront_distribution.cdn": "aws.primary", "aws_acm_certificate.cert": "aws.us",
                           "aws_wafv2_web_acl.waf": "aws.global"}},
}

def resolve(module_providers, ref):
    local = ref or "aws"
    parent = module_providers.get(local)
    if parent is None:
        if local == "aws":
            parent = "aws"
        else:
            return f"ERROR: {local} not passed to the module"
    return providers[parent]

for address, ref in root_resources.items():
    print(f"{address:<46} {providers[ref or 'aws']}")
for name, m in modules.items():
    for address, ref in m["resources"].items():
        print(f"module.{name}.{address:<{39 - len(name)}} {resolve(m['providers'], ref)}")

@@exercise Estimate the cost of a plan
From a simplified plan (actions and resource types with their sizes) and a monthly price table, compute the monthly cost before and after the change, the difference per resource, and whether the change needs extra approval (an increase above ¥50,000 a month, or any resource type without a price, which needs a human estimate).
@@starter
PRICES = {
    ("aws_instance", "t3.medium"): 6_000,
    ("aws_instance", "m6i.xlarge"): 28_000,
    ("aws_db_instance", "db.t4g.medium"): 11_000,
    ("aws_db_instance", "db.r6g.large"): 42_000,
    ("aws_nat_gateway", "-"): 7_000,
}
plan = [
    {"address": "aws_instance.web[0]", "action": "update", "type": "aws_instance", "before": "t3.medium", "after": "m6i.xlarge"},
    {"address": "aws_instance.web[1]", "action": "create", "type": "aws_instance", "before": None, "after": "m6i.xlarge"},
    {"address": "aws_db_instance.main", "action": "update", "type": "aws_db_instance", "before": "db.t4g.medium", "after": "db.r6g.large"},
    {"address": "aws_nat_gateway.b", "action": "delete", "type": "aws_nat_gateway", "before": "-", "after": None},
    {"address": "aws_opensearch_domain.logs", "action": "create", "type": "aws_opensearch_domain", "before": None, "after": "r6g.large.search"},
]
THRESHOLD = 50_000

def price(rtype, size):
    raise NotImplementedError

def estimate(plan):
    raise NotImplementedError

rows, unknown = estimate(plan)
total = 0
for address, before, after in rows:
    total += after - before
    print(f"{address:<28} ¥{before:>7,} -> ¥{after:>7,}  ({after - before:+,})")
print(f"monthly change: {total:+,} yen")
for address in unknown:
    print(f"no price for {address}: needs a manual estimate")
print("approval needed" if total > THRESHOLD or unknown else "within the threshold")
@@expected
aws_instance.web[0]          ¥  6,000 -> ¥ 28,000  (+22,000)
aws_instance.web[1]          ¥      0 -> ¥ 28,000  (+28,000)
aws_db_instance.main         ¥ 11,000 -> ¥ 42,000  (+31,000)
aws_nat_gateway.b            ¥  7,000 -> ¥      0  (-7,000)
monthly change: +74,000 yen
no price for aws_opensearch_domain.logs: needs a manual estimate
approval needed
@@requires
def price
def estimate
PRICES.get(
@@hint
`price(rtype, size)` returns 0 for `None` (nothing exists), otherwise `PRICES.get((rtype, size))`, which is `None` when unknown. In `estimate`, put resources with an unknown price in `unknown`; for the rest, append `(address, price(before), price(after))`.
@@solution
PRICES = {
    ("aws_instance", "t3.medium"): 6_000,
    ("aws_instance", "m6i.xlarge"): 28_000,
    ("aws_db_instance", "db.t4g.medium"): 11_000,
    ("aws_db_instance", "db.r6g.large"): 42_000,
    ("aws_nat_gateway", "-"): 7_000,
}
plan = [
    {"address": "aws_instance.web[0]", "action": "update", "type": "aws_instance", "before": "t3.medium", "after": "m6i.xlarge"},
    {"address": "aws_instance.web[1]", "action": "create", "type": "aws_instance", "before": None, "after": "m6i.xlarge"},
    {"address": "aws_db_instance.main", "action": "update", "type": "aws_db_instance", "before": "db.t4g.medium", "after": "db.r6g.large"},
    {"address": "aws_nat_gateway.b", "action": "delete", "type": "aws_nat_gateway", "before": "-", "after": None},
    {"address": "aws_opensearch_domain.logs", "action": "create", "type": "aws_opensearch_domain", "before": None, "after": "r6g.large.search"},
]
THRESHOLD = 50_000

def price(rtype, size):
    if size is None:
        return 0
    return PRICES.get((rtype, size))

def estimate(plan):
    rows, unknown = [], []
    for change in plan:
        before = price(change["type"], change["before"])
        after = price(change["type"], change["after"])
        if before is None or after is None:
            unknown.append(change["address"])
        else:
            rows.append((change["address"], before, after))
    return rows, unknown

rows, unknown = estimate(plan)
total = 0
for address, before, after in rows:
    total += after - before
    print(f"{address:<28} ¥{before:>7,} -> ¥{after:>7,}  ({after - before:+,})")
print(f"monthly change: {total:+,} yen")
for address in unknown:
    print(f"no price for {address}: needs a manual estimate")
print("approval needed" if total > THRESHOLD or unknown else "within the threshold")
