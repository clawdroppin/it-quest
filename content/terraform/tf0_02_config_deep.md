@@lesson tf0-config-deep Configuration in Depth: Providers, Variables, Outputs, Data Sources and the Graph
@@text
The Terraform Associate exam (003/004) asks precise questions about how configuration is read: where providers come from and how they're pinned, which variable value wins, what `sensitive` really hides, and how Terraform decides the order in which to create things. This lesson covers those mechanics.

## Providers

```hcl
terraform {
  required_version = ">= 1.9.0"
  required_providers {
    aws = {
      source  = "hashicorp/aws"        # <namespace>/<type>, registry.terraform.io implied
      version = "~> 5.80"
    }
  }
}

provider "aws" {
  region = "eu-west-1"
}

provider "aws" {
  alias  = "us"
  region = "us-east-1"
}

resource "aws_acm_certificate" "cdn" {
  provider = aws.us          # use the aliased configuration
  domain_name = "example.com"
  validation_method = "DNS"
}
```

- `terraform init` downloads providers into `.terraform/` and records exact versions and checksums in **`.terraform.lock.hcl`**. **Commit the lock file**; `terraform init -upgrade` picks newer versions within the constraints.
- Version constraint operators: `=`, `!=`, `>`, `>=`, `<`, `<=`, and **`~>`** (pessimistic: `~> 5.80` means `>= 5.80, < 6.0`; `~> 5.80.1` means `>= 5.80.1, < 5.81.0`).
- Provider tiers: official (HashiCorp), partner, community.

## Input variables

```hcl
variable "instance_count" {
  type        = number
  default     = 2
  description = "Web servers per AZ"
  validation {
    condition     = var.instance_count >= 1 && var.instance_count <= 10
    error_message = "Use between 1 and 10 instances."
  }
}

variable "db_password" {
  type      = string
  sensitive = true
}
```

**Precedence** (later wins):
1. `default` in the variable block
2. environment variables `TF_VAR_name`
3. `terraform.tfvars`
4. `terraform.tfvars.json`
5. `*.auto.tfvars` / `*.auto.tfvars.json` (alphabetical order)
6. `-var` and `-var-file` on the command line (in the order given)

If no value is found and there's no default, Terraform **prompts** interactively (or fails with `-input=false`).

## Locals and outputs

- `locals { name_prefix = "${var.project}-${var.env}" }`: computed values, referenced as `local.name_prefix`.
- `output "web_ip" { value = aws_instance.web.public_ip }`: shown after apply, read with `terraform output` (`-json`, `-raw`), used by parent modules and `terraform_remote_state`.
- **`sensitive = true`** redacts values in plan/apply output, but they are still stored **in plain text in the state file**: protect state (encryption at rest, access control) and never commit it.

## Data sources

`data "aws_ami" "ubuntu" { most_recent = true ... }` **reads** existing infrastructure (referenced as `data.aws_ami.ubuntu.id`). Data sources are read during plan when their arguments are known.

## The dependency graph

Terraform builds a graph from **references** (implicit dependencies) plus **`depends_on`** (explicit, for hidden dependencies such as IAM policy propagation). It creates independent resources **in parallel** (default `-parallelism=10`), and **destroys in reverse** dependency order. `terraform graph` outputs the graph in DOT format.

## Meta-arguments and lifecycle

`count`, `for_each`, `provider`, `depends_on`, `lifecycle { create_before_destroy, prevent_destroy, ignore_changes, replace_triggered_by, precondition, postcondition }`.

## Useful commands

`terraform fmt` (style, `-check` in CI), `terraform validate` (syntax and internal consistency, no API calls), `terraform console` (try expressions), `terraform providers`, `terraform version`.

@@quiz
Which file records the exact provider versions and checksums selected by `terraform init`?
- terraform.tfstate
* .terraform.lock.hcl
- terraform.tfvars
- versions.tf
? Commit it to version control.

@@quiz
What does the constraint `~> 5.80` allow?
- Only 5.80.0
* Any version >= 5.80 and < 6.0
- Any version >= 5.80 and < 5.81
- Any 5.x or 6.x version
? The rightmost number can increase.

@@quiz
A variable is set in terraform.tfvars, in a TF_VAR_ environment variable and with -var. Which value is used?
- terraform.tfvars
- The environment variable
* The -var value
- The default
? Command-line values have the highest precedence.

@@quiz
An output is marked sensitive. Where can the value still be read in plain text?
- Nowhere
* In the state file
- In the plan output
- In the lock file
? Protect the state.

@@quiz
A Lambda function needs an IAM policy attached before it's invoked, but it doesn't reference the attachment. What do you add?
- count
* depends_on
- a data source
- prevent_destroy
? Explicit dependency for a hidden relationship.

@@quiz
Which command checks syntax and internal consistency without contacting providers' APIs?
- terraform plan
* terraform validate
- terraform refresh
- terraform apply -check
? fmt checks style only.

@@flashcards
required_providers :: source and version per provider
provider source :: namespace/type
lock file :: .terraform.lock.hcl, commit it
init -upgrade :: newer versions within constraints
~> 5.80 :: >= 5.80, < 6.0
~> 5.80.1 :: >= 5.80.1, < 5.81.0
provider alias :: second configuration, provider = aws.us
variable precedence :: default < env < tfvars < auto.tfvars < -var
TF_VAR_name :: env var for a variable
validation block :: condition + error_message
sensitive :: hidden in output, plain in state
locals :: computed values
data source :: reads existing infrastructure
implicit dependency :: via references
depends_on :: explicit dependency
parallelism default :: 10
destroy order :: reverse dependencies
terraform fmt :: canonical style
terraform validate :: syntax and consistency
terraform console :: evaluate expressions

@@drill
Which environment variable prefix sets Terraform input variables?
= TF_VAR_
= TF_VAR
? TF_VAR_region sets var.region.

@@drill
What is Terraform's default parallelism?
= 10
? Change with -parallelism=n.

@@exercise Resolve variable precedence
Determine the final value of each variable from its sources, following Terraform's precedence: default, `TF_VAR_` environment variables, `terraform.tfvars`, `terraform.tfvars.json`, `*.auto.tfvars` files in alphabetical order, then `-var`/`-var-file` flags in command-line order. Print the winning value and its source; a variable with no value and no default is an error (with `-input=false`). Sensitive values are printed as `(sensitive)`.
@@lang python
@@starter
variables = {"region": {"default": "eu-west-1"}, "env": {}, "instance_count": {"default": 2}, "db_password": {"sensitive": True}, "owner": {}}
env = {"TF_VAR_env": "dev", "TF_VAR_db_password": "env-secret"}
tfvars = {"env": "staging", "instance_count": 3}
tfvars_json = {}
auto_files = {"zz.auto.tfvars": {"instance_count": 5}, "prod.auto.tfvars": {"env": "prod", "instance_count": 4}}
cli = [("-var-file", {"region": "us-east-1"}), ("-var", {"instance_count": 6})]

def sources():
    raise NotImplementedError
@@expected
region = 'us-east-1'  [from -var-file]
env = 'prod'  [from prod.auto.tfvars]
instance_count = 6  [from -var]
db_password = (sensitive)  [from environment]
owner: error, no value given and no default (input disabled)
@@requires
def sources
sorted(auto_files)
TF_VAR_
@@hint
Build an ordered list of `(source_name, values)` from lowest to highest precedence, then for each variable keep the last source that sets it. Strip `TF_VAR_` from the environment names.
@@solution
variables = {"region": {"default": "eu-west-1"}, "env": {}, "instance_count": {"default": 2}, "db_password": {"sensitive": True}, "owner": {}}
env = {"TF_VAR_env": "dev", "TF_VAR_db_password": "env-secret"}
tfvars = {"env": "staging", "instance_count": 3}
tfvars_json = {}
auto_files = {"zz.auto.tfvars": {"instance_count": 5}, "prod.auto.tfvars": {"env": "prod", "instance_count": 4}}
cli = [("-var-file", {"region": "us-east-1"}), ("-var", {"instance_count": 6})]

def sources():
    ordered = [("environment", {k[len("TF_VAR_"):]: v for k, v in env.items() if k.startswith("TF_VAR_")})]
    ordered.append(("terraform.tfvars", tfvars))
    ordered.append(("terraform.tfvars.json", tfvars_json))
    ordered += [(name, auto_files[name]) for name in sorted(auto_files)]
    ordered += [(flag, values) for flag, values in cli]
    return ordered

for name, spec in variables.items():
    value, source = spec.get("default"), "default" if "default" in spec else None
    for src, values in sources():
        if name in values:
            value, source = values[name], src
    if source is None:
        print(f"{name}: error, no value given and no default (input disabled)")
        continue
    shown = "(sensitive)" if spec.get("sensitive") else repr(value)
    print(f"{name} = {shown}  [from {source}]")

@@exercise Order applies and destroys from the dependency graph
Resources reference each other. Compute apply "waves": resources whose dependencies are all created can run in parallel in the same wave (limited to `PARALLELISM` per wave, alphabetical). Then print the destroy order, which reverses the dependencies. Detect a cycle and report it.
@@lang python
@@starter
PARALLELISM = 3
graphs = {
    "web stack": {
        "aws_vpc.main": [],
        "aws_subnet.a": ["aws_vpc.main"],
        "aws_subnet.b": ["aws_vpc.main"],
        "aws_security_group.web": ["aws_vpc.main"],
        "aws_iam_role.app": [],
        "aws_iam_role_policy.app": ["aws_iam_role.app"],
        "aws_lb.web": ["aws_subnet.a", "aws_subnet.b", "aws_security_group.web"],
        "aws_instance.web": ["aws_subnet.a", "aws_security_group.web", "aws_iam_role_policy.app"],
        "aws_route53_record.www": ["aws_lb.web"],
    },
    "broken": {"a.x": ["b.y"], "b.y": ["c.z"], "c.z": ["a.x"], "d.w": []},
}

def waves(deps):
    raise NotImplementedError
@@expected
== web stack
  apply wave 1: aws_iam_role.app, aws_vpc.main
  apply wave 2: aws_iam_role_policy.app, aws_security_group.web, aws_subnet.a
  apply wave 3: aws_instance.web, aws_subnet.b
  apply wave 4: aws_lb.web
  apply wave 5: aws_route53_record.www
  destroy order: aws_route53_record.www -> aws_lb.web -> aws_subnet.b -> aws_instance.web -> aws_subnet.a -> aws_security_group.web -> aws_iam_role_policy.app -> aws_vpc.main -> aws_iam_role.app
== broken
  apply wave 1: d.w
  error: Cycle: a.x, b.y, c.z
@@requires
def waves
PARALLELISM
reversed(
@@hint
Each round, `ready = sorted(r for r in remaining if all(d in done for d in deps[r]))[:PARALLELISM]`. If nothing is ready but resources remain, there's a cycle among them. Destroy order is the reverse of the flattened apply order.
@@solution
PARALLELISM = 3
graphs = {
    "web stack": {
        "aws_vpc.main": [],
        "aws_subnet.a": ["aws_vpc.main"],
        "aws_subnet.b": ["aws_vpc.main"],
        "aws_security_group.web": ["aws_vpc.main"],
        "aws_iam_role.app": [],
        "aws_iam_role_policy.app": ["aws_iam_role.app"],
        "aws_lb.web": ["aws_subnet.a", "aws_subnet.b", "aws_security_group.web"],
        "aws_instance.web": ["aws_subnet.a", "aws_security_group.web", "aws_iam_role_policy.app"],
        "aws_route53_record.www": ["aws_lb.web"],
    },
    "broken": {"a.x": ["b.y"], "b.y": ["c.z"], "c.z": ["a.x"], "d.w": []},
}

def waves(deps):
    done, remaining, result = set(), set(deps), []
    while remaining:
        ready = sorted(r for r in remaining if all(d in done for d in deps[r]))[:PARALLELISM]
        if not ready:
            return result, sorted(remaining)
        result.append(ready)
        done.update(ready)
        remaining.difference_update(ready)
    return result, None

for name, deps in graphs.items():
    print(f"== {name}")
    result, cycle = waves(deps)
    for i, wave in enumerate(result, 1):
        print(f"  apply wave {i}: {', '.join(wave)}")
    if cycle:
        print(f"  error: Cycle: {', '.join(cycle)}")
        continue
    order = [r for wave in result for r in wave]
    print("  destroy order:", " -> ".join(reversed(order)))
