@@lesson tf0-iac-workflow Terraform: The Exam, HCL and the Core Workflow
@@text
## The Terraform Associate exam

**HashiCorp Certified: Terraform Associate (004)** is a 1-hour online exam, about 57 multiple-choice, multiple-select, true/false and fill-in questions, valid for two years. It covers infrastructure-as-code concepts, the Terraform workflow and CLI, configuration (HCL), modules, state, and **HCP Terraform** (formerly Terraform Cloud).

It's one of the cheapest and most practical certifications on the DevOps and cloud roadmap. Job ads in Japan for SRE, cloud and platform engineers (インフラエンジニア, SRE) very often list Terraform as a requirement.

> **OpenTofu** is the open-source fork of Terraform (after HashiCorp changed Terraform's license in 2023). It's nearly identical: `tofu init`, `tofu plan`. Everything here applies to both.

## Why infrastructure as code

- **Repeatable**: the same code builds dev, staging and production the same way.
- **Reviewable**: changes go through Git pull requests, and `terraform plan` shows exactly what will change *before* anything happens.
- **Declarative**: you describe the *end state* ("three VMs"); Terraform works out the steps. (Ansible is mostly procedural, configuring *inside* machines; Terraform creates the machines and the cloud resources around them.)
- **Multi-cloud**: thousands of **providers** (AWS, Azure, Google Cloud, Kubernetes, GitHub, Cloudflare, Datadog...).

## HCL: the configuration language

```hcl
terraform {
  required_version = ">= 1.6"
  required_providers {
    aws = { source = "hashicorp/aws", version = "~> 5.0" }   # any 5.x, not 6.0
  }
}

provider "aws" {
  region = var.region
}

variable "region" {
  type    = string
  default = "ap-northeast-1"              # Tokyo
}

variable "instance_count" {
  type = number
  validation {
    condition     = var.instance_count <= 5
    error_message = "At most 5 instances."
  }
}

locals {
  tags = { Project = "shop", Env = terraform.workspace }
}

data "aws_ami" "ubuntu" {                 # read something that already exists
  most_recent = true
  owners      = ["099720109477"]          # Canonical
  filter {
    name   = "name"
    values = ["ubuntu/images/hvm-ssd-gp3/ubuntu-noble-24.04-amd64-server-*"]
  }
}

resource "aws_instance" "web" {           # create and manage something
  count         = var.instance_count
  ami           = data.aws_ami.ubuntu.id  # an implicit dependency
  instance_type = "t3.micro"
  tags          = merge(local.tags, { Name = "web-${count.index}" })
}

output "web_ips" {
  value = aws_instance.web[*].public_ip   # a splat expression
}
```

- Block types: `terraform`, `provider`, `variable`, `locals`, `data`, `resource`, `output`, `module`.
- A resource's address is `TYPE.NAME` (`aws_instance.web`), with an index for `count` (`aws_instance.web[0]`) or a key for `for_each` (`aws_s3_bucket.logs["tokyo"]`).
- **References** create **implicit dependencies**; `depends_on` adds an explicit one when Terraform can't see it.
- **`count`** vs **`for_each`**: with count, removing the middle item shifts the indexes and recreates later resources; `for_each` over a map or set uses stable keys. Prefer for_each for anything that isn't identical.
- Version constraints: `= 1.2.0`, `>= 1.2`, `~> 1.2` (≥ 1.2 and < 2.0), `~> 1.2.0` (≥ 1.2.0 and < 1.3.0).
- Variable values come from (lowest to highest precedence): default, environment variables `TF_VAR_name`, `terraform.tfvars`, `*.auto.tfvars`, `-var-file`, and `-var` on the command line.
- Mark secrets with `sensitive = true` (hidden in output, *but still stored in the state in plain text*).
- Useful expressions: `for` (`[for s in var.subnets : s.cidr]`), conditionals (`var.prod ? 3 : 1`), `dynamic` blocks, functions like `lookup`, `merge`, `join`, `cidrsubnet`, `file`, `templatefile`. Try them in `terraform console`.

## The workflow

```bash,norun
terraform init        # download providers and modules, set up the backend; creates .terraform.lock.hcl
terraform fmt         # format the code (fmt -check in CI)
terraform validate    # syntax and internal consistency, no cloud calls
terraform plan -out=plan.tfplan   # compare code, state and reality; show the changes
terraform apply plan.tfplan       # do exactly what the saved plan said
terraform destroy     # remove everything this configuration manages
```

Plan symbols: `+` create, `~` update in place, `-` destroy, **`-/+` destroy and then create a replacement** (a change to an argument that can't be updated in place, like a VM's AMI or a database's engine). Always read the plan for `-/+` and `-`: that's where data is lost. (The project in this track reads the plan for you.)

- `.terraform.lock.hcl` records the exact provider versions and hashes: **commit it**. `terraform init -upgrade` updates it.
- `terraform apply -replace=aws_instance.web[0]` forces a recreate (it replaced the old `terraform taint`).
- `terraform plan -refresh-only` / `apply -refresh-only` updates the state to match reality without changing infrastructure (detecting **drift**).
- `TF_LOG=DEBUG` turns on detailed logs.

@@quiz
Which command downloads providers and configures the backend?
* terraform init
- terraform plan
- terraform get
- terraform validate
? init must run first in every new working directory, and again after adding providers or modules.

@@quiz
What does `version = "~> 5.0"` allow?
* Any version ≥ 5.0 and < 6.0
- Exactly 5.0
- Any version ≥ 5.0, including 6.x
- Only 5.0.x patch versions
? `~> 5.0.0` would allow only 5.0.x.

@@quiz
In a plan, what does `-/+` mean?
* The resource will be destroyed and a replacement created
- It will be updated in place
- It will be imported
- Nothing changes
? Changing an attribute that can't be updated in place forces replacement. Read these lines carefully.

@@quiz
You manage subnets with `count` over a list. Removing the second item from the list will…
* shift the indexes, so Terraform wants to change or recreate every subnet after it
- only remove the second subnet
- fail with an error
- do nothing until the next init
? `for_each` with stable keys avoids this.

@@quiz
Which file should be committed to Git?
* .terraform.lock.hcl
- terraform.tfstate
- the .terraform directory
- crash.log
? State contains secrets and belongs in a remote backend; .terraform is a local cache.

@@quiz
Which variable source has the highest precedence?
* `-var` on the command line
- terraform.tfvars
- the default in the variable block
- TF_VAR_ environment variables
? Later sources override earlier ones; command-line flags come last.

@@flashcards
terraform init :: providers, modules, backend
terraform validate :: checks the configuration without contacting providers
terraform plan -out :: save the exact plan to apply
-/+ in a plan :: destroy, then create a replacement
data block :: read existing infrastructure
implicit dependency :: created by a reference
for_each vs count :: stable keys vs shifting indexes
~> 1.2 :: ≥ 1.2 and < 2.0
TF_VAR_name :: set a variable from the environment
sensitive = true :: hidden in output, still in state
.terraform.lock.hcl :: provider versions and hashes: commit it
apply -replace :: force recreation (replaces taint)
-refresh-only :: update state to match reality (drift)

@@drill
Which environment variable turns on Terraform's detailed logs? (just the name)
= TF_LOG

@@exercise Check a command sequence
Check sequences of Terraform commands that people ran (or put in a pipeline) against the workflow rules: `init` must come before `validate`, `plan`, `apply` or `destroy`; `fmt -check` and `validate` should come before `plan` in CI; in CI, `apply` must use the saved plan file from an earlier `plan -out`; and `destroy` should never appear in a CI pipeline for production. Print the problems for each sequence, or "ok".
@@lang python
@@starter
sequences = {
    "laptop, first try": ["plan", "apply"],
    "laptop, good": ["init", "fmt", "validate", "plan", "apply"],
    "ci, missing checks": ["init", "plan -out=tfplan", "apply tfplan"],
    "ci, unsaved plan": ["init", "fmt -check", "validate", "plan", "apply -auto-approve"],
    "ci, good": ["init", "fmt -check", "validate", "plan -out=tfplan", "apply tfplan"],
    "ci prod, cleanup job": ["init", "fmt -check", "validate", "destroy -auto-approve"],
}

def check(name, commands):
    raise NotImplementedError

for name, commands in sequences.items():
    problems = check(name, commands)
    print(f"{name:<22} " + ("ok" if not problems else "; ".join(problems)))
@@expected
laptop, first try      'plan' before init; 'apply' before init
laptop, good           ok
ci, missing checks     run fmt -check and validate before plan
ci, unsaved plan       apply without the saved plan file
ci, good               ok
ci prod, cleanup job   destroy in a production pipeline
@@requires
def check
startswith("plan -out")
@@hint
Use the first word of each command (`c.split()[0]`). Track whether `init` has been seen. For names starting with "ci", require "fmt" and "validate" before the first "plan", require that every `apply` has an argument that matches a file saved by an earlier `c.startswith("plan -out")`, and flag `destroy` when "prod" is in the name.
@@solution
sequences = {
    "laptop, first try": ["plan", "apply"],
    "laptop, good": ["init", "fmt", "validate", "plan", "apply"],
    "ci, missing checks": ["init", "plan -out=tfplan", "apply tfplan"],
    "ci, unsaved plan": ["init", "fmt -check", "validate", "plan", "apply -auto-approve"],
    "ci, good": ["init", "fmt -check", "validate", "plan -out=tfplan", "apply tfplan"],
    "ci prod, cleanup job": ["init", "fmt -check", "validate", "destroy -auto-approve"],
}

def check(name, commands):
    problems, seen, saved = [], [], set()
    ci = name.startswith("ci")
    for c in commands:
        word = c.split()[0]
        if word in ("validate", "plan", "apply", "destroy") and "init" not in seen:
            problems.append(f"'{word}' before init")
        if ci and word == "plan" and not ("fmt" in seen and "validate" in seen):
            problems.append("run fmt -check and validate before plan")
        if c.startswith("plan -out"):
            saved.add(c.split("=", 1)[1])
        if ci and word == "apply":
            args = c.split()[1:]
            if not any(a in saved for a in args):
                problems.append("apply without the saved plan file")
        if ci and word == "destroy" and "prod" in name:
            problems.append("destroy in a production pipeline")
        seen.append(word)
    return problems

for name, commands in sequences.items():
    problems = check(name, commands)
    print(f"{name:<22} " + ("ok" if not problems else "; ".join(problems)))

@@exercise Resolve references in HCL expressions
Terraform strings can contain `${...}` references such as `${var.env}`, `${local.name}` or `${aws_s3_bucket.logs.arn}`. Write `resolve(text, values)` that replaces each reference with its value from `values` (a dict of dotted names), resolving locals that themselves contain references, and raising an error for an unknown name or a reference cycle. Print the resolved strings, then show the errors.
@@lang python
@@starter
import re

REF = re.compile(r"\$\{([\w.]+)\}")
values = {
    "var.env": "prod",
    "var.project": "shop",
    "local.prefix": "${var.project}-${var.env}",
    "local.bucket_name": "${local.prefix}-logs",
    "aws_s3_bucket.logs.arn": "arn:aws:s3:::shop-prod-logs",
    "local.a": "${local.b}",
    "local.b": "${local.a}",
}

def resolve(text, values, seen=()):
    raise NotImplementedError


for text in ["${local.bucket_name}", "Name tag: ${var.project} (${var.env})", "${aws_s3_bucket.logs.arn}/*"]:
    print(f"{text} -> {resolve(text, values)}")
for text in ["${var.region}", "${local.a}"]:
    try:
        resolve(text, values)
    except ValueError as e:
        print(f"{text} -> Error: {e}")
@@expected
${local.bucket_name} -> shop-prod-logs
Name tag: ${var.project} (${var.env}) -> Name tag: shop (prod)
${aws_s3_bucket.logs.arn}/* -> arn:aws:s3:::shop-prod-logs/*
${var.region} -> Error: reference to undeclared var.region
${local.a} -> Error: cycle: local.a -> local.b -> local.a
@@requires
def resolve
.sub(
@@hint
`re.sub` can take a function: for each `${name}`, look the name up and resolve its value recursively. Pass along the names you're already resolving to detect a cycle.
@@solution
import re

REF = re.compile(r"\$\{([\w.]+)\}")
values = {
    "var.env": "prod",
    "var.project": "shop",
    "local.prefix": "${var.project}-${var.env}",
    "local.bucket_name": "${local.prefix}-logs",
    "aws_s3_bucket.logs.arn": "arn:aws:s3:::shop-prod-logs",
    "local.a": "${local.b}",
    "local.b": "${local.a}",
}

def resolve(text, values, seen=()):
    def sub(m):
        name = m.group(1)
        if name in seen:
            raise ValueError(f"cycle: {' -> '.join(seen + (name,))}")
        if name not in values:
            raise ValueError(f"reference to undeclared {name}")
        return resolve(values[name], values, seen + (name,))
    return REF.sub(sub, text)

for text in ["${local.bucket_name}", "Name tag: ${var.project} (${var.env})", "${aws_s3_bucket.logs.arn}/*"]:
    print(f"{text} -> {resolve(text, values)}")
for text in ["${var.region}", "${local.a}"]:
    try:
        resolve(text, values)
    except ValueError as e:
        print(f"{text} -> Error: {e}")
