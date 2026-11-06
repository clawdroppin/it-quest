@@lesson tf2-testing-ci Terraform in a Team: Testing, CI/CD and Advanced HCL
@@text
## Advanced HCL you'll meet in real code

```hcl
variable "subnets" {
  type = map(object({
    cidr = string
    az   = string
    public = optional(bool, false)        # optional attributes with defaults
  }))
}

resource "aws_subnet" "this" {
  for_each          = var.subnets           # one subnet per map key
  vpc_id            = aws_vpc.main.id
  cidr_block        = each.value.cidr
  availability_zone = each.value.az
  map_public_ip_on_launch = each.value.public
  tags = { Name = each.key }
}

locals {
  public_ids = [for k, s in aws_subnet.this : s.id if var.subnets[k].public]
}

resource "aws_security_group" "web" {
  name   = "web"
  vpc_id = aws_vpc.main.id
  dynamic "ingress" {                       # generate repeated nested blocks
    for_each = [80, 443]
    content {
      from_port   = ingress.value
      to_port     = ingress.value
      protocol    = "tcp"
      cidr_blocks = ["0.0.0.0/0"]
    }
  }
}
```

- **Types**: `string`, `number`, `bool`, `list(...)`, `set(...)`, `map(...)`, `object({...})`, `tuple([...])`, `any`.
- **Conditions everywhere**: `precondition` and `postcondition` in a resource's `lifecycle`, and `check` blocks that warn without failing the apply.
- **Functions**: `cidrsubnet("10.0.0.0/16", 8, 2)` → `10.0.2.0/24`; `try()`, `coalesce()`, `flatten()`, `zipmap()`, `toset()`, `jsonencode()`, `templatefile()`.
- **Provisioners** (`local-exec`, `remote-exec`) are a last resort: prefer cloud-init, images built with Packer, or configuration management.

## Testing

| Level | Tool | Checks |
|---|---|---|
| format and syntax | `terraform fmt -check`, `terraform validate` | style, references, types |
| static analysis | tflint, Trivy (tfsec), Checkov | provider-specific mistakes, insecure settings |
| unit-style tests | **`terraform test`** with `command = plan` | a module's logic, without creating anything |
| integration tests | `terraform test` with `command = apply`, or Terratest (Go) | real resources in a sandbox account, then destroyed |
| policy | Sentinel, OPA/Conftest on the plan JSON | organization rules |

```hcl
# tests/site.tftest.hcl
variables {
  name        = "demo"
  environment = "dev"
}

run "bucket_is_private_by_default" {
  command = plan
  assert {
    condition     = aws_s3_bucket_public_access_block.this.block_public_acls == true
    error_message = "The bucket must block public ACLs by default."
  }
}

run "rejects_bad_environment" {
  command = plan
  variables { environment = "qa" }
  expect_failures = [var.environment]
}
```

## A CI/CD pipeline for infrastructure

```yaml
# .github/workflows/terraform.yml (the core of it)
on: { pull_request: {}, push: { branches: [main] } }
jobs:
  plan:
    runs-on: ubuntu-latest
    permissions: { id-token: write, contents: read, pull-requests: write }   # OIDC: no stored cloud keys
    steps:
      - uses: actions/checkout@v4
      - uses: hashicorp/setup-terraform@v3
      - run: terraform fmt -check -recursive
      - run: terraform init -input=false
      - run: terraform validate
      - run: terraform plan -input=false -out=plan.tfplan
      - run: terraform show -json plan.tfplan > plan.json   # review it (like this track's project)
  # apply: only on main, in a protected environment that needs an approval
```

Principles:

- **Plan on every pull request**, post the plan for review, **apply only after merge**, from the pipeline, never from laptops.
- Authenticate the pipeline with **OIDC federation** (GitHub → AWS IAM role, Azure workload identity, Google Workload Identity Federation) instead of long-lived keys.
- Separate state and credentials per environment; protect production with required reviewers.
- **Drift**: schedule `terraform plan -detailed-exitcode` nightly (exit code 2 = changes) and alert.

## Structuring a growing codebase

```text
infra/
  modules/             reusable modules (network, service, database)
  envs/
    dev/main.tf        calls the modules with dev values; its own backend
    prod/main.tf
```

Tools like **Terragrunt** or HCP Terraform workspaces reduce repetition between environments. Keep **blast radius** small: one state per environment *and* per area (network, data, apps), so a mistake can't destroy everything at once.

@@quiz
Which block generates several nested `ingress` blocks from a list?
* A `dynamic` block
- `for_each` on the resource
- `count`
- A `locals` block
? `for_each` on the resource creates several resources; `dynamic` creates several nested blocks inside one.

@@quiz
What does `terraform test` with `command = plan` do?
* Runs the module's plan with the test's variables and checks the assertions, creating nothing
- Applies the module to production
- Formats the code
- Downloads providers only
? With `command = apply` it creates real resources and destroys them afterwards.

@@quiz
How should a CI pipeline authenticate to the cloud for Terraform?
* With OIDC federation to a short-lived role or identity
- With an admin access key stored in the repository
- With the team lead's personal credentials
- It doesn't need to authenticate
? Federation gives temporary credentials and nothing to leak.

@@quiz
What does exit code 2 from `terraform plan -detailed-exitcode` mean?
* The plan succeeded and there are changes (possible drift)
- An error occurred
- There are no changes
- The state is locked
? 0 = no changes, 1 = error, 2 = changes present.

@@quiz
`cidrsubnet("10.0.0.0/16", 8, 2)` returns…
* 10.0.2.0/24
- 10.0.0.2/16
- 10.2.0.0/16
- 10.0.8.0/24
? Add 8 bits to the prefix (/24) and take network number 2.

@@flashcards
dynamic block :: generates repeated nested blocks
optional(type, default) :: optional object attributes with defaults
precondition / postcondition :: lifecycle checks that fail the run
check block :: warns without failing
terraform test :: .tftest.hcl files with run blocks and asserts
expect_failures :: a test that passes when validation fails
static analysis :: tflint, Trivy, Checkov
OIDC in CI :: short-lived cloud credentials, no stored keys
-detailed-exitcode :: 0 none, 1 error, 2 changes
blast radius :: split state so one mistake can't destroy everything

@@exercise Expand a dynamic block
Simulate what a `dynamic "ingress"` block produces: for each element of `var.rules` (iterated in order, like a list), write one nested `ingress` block with the `content` attributes, where `ingress.value.x` in the template (the iterator is named after the block) refers to the element's field. Elements with `enabled = false` are filtered out by the `for_each` expression `[for r in var.rules : r if r.enabled]`. Print the resulting HCL.
@@lang python
@@starter
rules = [
    {"port": 443, "cidr": "0.0.0.0/0", "description": "HTTPS", "enabled": True},
    {"port": 80, "cidr": "0.0.0.0/0", "description": "HTTP redirect", "enabled": True},
    {"port": 22, "cidr": "203.0.113.0/24", "description": "SSH from office", "enabled": False},
    {"port": 5432, "cidr": "10.0.0.0/16", "description": "Postgres from VPC", "enabled": True},
]
content = {"from_port": "ingress.value.port", "to_port": "ingress.value.port", "protocol": '"tcp"',
           "cidr_blocks": "[ingress.value.cidr]", "description": "ingress.value.description"}

def render_value(template, element):
    raise NotImplementedError

def expand(rules, content):
    raise NotImplementedError

print('resource "aws_security_group" "web" {')
print('  name = "web"')
print(expand(rules, content))
print("}")
@@expected
resource "aws_security_group" "web" {
  name = "web"
  ingress {
    from_port   = 443
    to_port     = 443
    protocol    = "tcp"
    cidr_blocks = ["0.0.0.0/0"]
    description = "HTTPS"
  }
  ingress {
    from_port   = 80
    to_port     = 80
    protocol    = "tcp"
    cidr_blocks = ["0.0.0.0/0"]
    description = "HTTP redirect"
  }
  ingress {
    from_port   = 5432
    to_port     = 5432
    protocol    = "tcp"
    cidr_blocks = ["10.0.0.0/16"]
    description = "Postgres from VPC"
  }
}
@@requires
def render_value
def expand
r["enabled"]
@@hint
In `render_value`, replace each `ingress.value.<field>` with the field's value: numbers as they are, strings in double quotes (use `re.sub(r"ingress\.value\.(\w+)", ...)`). `expand` keeps `[r for r in rules if r["enabled"]]` and builds `  ingress {`, one `    key = value` line per content item (pad the key to the longest key), and `  }` for each.
@@solution
import re

rules = [
    {"port": 443, "cidr": "0.0.0.0/0", "description": "HTTPS", "enabled": True},
    {"port": 80, "cidr": "0.0.0.0/0", "description": "HTTP redirect", "enabled": True},
    {"port": 22, "cidr": "203.0.113.0/24", "description": "SSH from office", "enabled": False},
    {"port": 5432, "cidr": "10.0.0.0/16", "description": "Postgres from VPC", "enabled": True},
]
content = {"from_port": "ingress.value.port", "to_port": "ingress.value.port", "protocol": '"tcp"',
           "cidr_blocks": "[ingress.value.cidr]", "description": "ingress.value.description"}

def render_value(template, element):
    def sub(m):
        v = element[m.group(1)]
        return str(v) if isinstance(v, (int, float)) else f'"{v}"'
    return re.sub(r"ingress\.value\.(\w+)", sub, template)

def expand(rules, content):
    width = max(len(k) for k in content)
    blocks = []
    for r in [r for r in rules if r["enabled"]]:
        lines = ["  ingress {"]
        lines += [f"    {k:<{width}} = {render_value(v, r)}" for k, v in content.items()]
        lines.append("  }")
        blocks.append("\n".join(lines))
    return "\n".join(blocks)

print('resource "aws_security_group" "web" {')
print('  name = "web"')
print(expand(rules, content))
print("}")
