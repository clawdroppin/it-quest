@@lesson tf1-state-plan-deep State and Plans in Depth: Diffs, Replacement, Drift, Import and State Commands
@@text
Most Terraform incidents come from not reading the plan carefully or from mishandled state. This lesson explains how Terraform compares configuration, state and real infrastructure, what the plan symbols mean, when a change forces replacement, and how to fix state safely with `import`, `moved`, `removed` and the `terraform state` commands.

## Three views of the world

- **Configuration**: what you want (`.tf` files).
- **State**: what Terraform last knew about the resources it manages (IDs, attributes, dependencies), in `terraform.tfstate` or a remote backend.
- **Real infrastructure**: what actually exists now.

`terraform plan` **refreshes** state from the real world (unless `-refresh=false`), then compares it with the configuration and proposes actions.

## Plan symbols

| Symbol | Action |
|---|---|
| `+` | create |
| `-` | destroy |
| `~` | update in place |
| `-/+` | destroy then create (replace) |
| `+/-` | create then destroy (replace with `create_before_destroy`) |
| `<=` | read (data source) |

Plan summary: `Plan: 2 to add, 1 to change, 1 to destroy.` A replacement counts as one add **and** one destroy. Arguments marked **`# forces replacement`** are ones the provider can't change in place (e.g. an EC2 instance's `ami` or `availability_zone`, a subnet's `cidr_block`, an RDS instance's `engine`).

Save and apply exactly what you reviewed: `terraform plan -out=tfplan` then `terraform apply tfplan`. `terraform show -json tfplan` gives machine-readable output for policy checks.

## Drift

**Drift** = real infrastructure changed outside Terraform. Options:
- `terraform plan` shows it and proposes to **revert** to the configuration.
- `terraform apply -refresh-only` **accepts** the real values into state (replaces the deprecated `terraform refresh`).
- Change the configuration to match, or use `lifecycle { ignore_changes = [tags["LastScanned"]] }` for attributes managed elsewhere.

## Forcing and targeting

- `terraform apply -replace="aws_instance.web"` forces replacement (replaces the deprecated `terraform taint`).
- `-target=resource` limits the run to some resources: for exceptional recovery only.

## Bringing resources under management

```hcl
import {
  to = aws_s3_bucket.logs
  id = "acme-prod-logs"
}
```

`terraform plan -generate-config-out=generated.tf` writes configuration for import blocks. The older CLI form is `terraform import aws_s3_bucket.logs acme-prod-logs` (state only, you write the config).

## Refactoring

```hcl
moved {
  from = aws_instance.web
  to   = aws_instance.app["web"]
}

removed {
  from = aws_instance.legacy
  lifecycle { destroy = false }     # forget it, don't destroy it
}
```

Without `moved`, renaming a resource address means destroy + create.

## State commands

| Command | Use |
|---|---|
| `terraform state list` | list addresses |
| `terraform state show ADDR` | attributes of one resource |
| `terraform state mv SRC DST` | rename/move an address (prefer `moved` blocks) |
| `terraform state rm ADDR` | stop managing, don't destroy (prefer `removed` blocks) |
| `terraform state pull` / `push` | download/upload raw state (push is dangerous) |
| `terraform force-unlock LOCK_ID` | release a stuck lock |

## Remote state and locking

Backends (S3 with native lockfile or DynamoDB locking, azurerm, gcs, HCP Terraform) store state remotely, encrypt it, and **lock** it during operations so two applies can't run at once. Backend blocks **can't use variables**; supply values with `-backend-config` (partial configuration). Changing the backend requires `terraform init` (with `-migrate-state`). Share outputs with `terraform_remote_state` data sources or, better, purpose-built data sources/parameters.

@@quiz
A plan shows `-/+ resource "aws_instance" "web"` with `ami # forces replacement`. What happens on apply?
- The AMI is changed in place
* The instance is destroyed and a new one created
- Nothing, it's a read
- A new instance is created and the old one kept
? -/+ is replacement; +/- would mean create before destroy.

@@quiz
Someone changed a security group's description in the console. You want state to accept the change without modifying infrastructure. Which command?
- terraform apply
* terraform apply -refresh-only
- terraform state rm
- terraform import
? terraform refresh is deprecated.

@@quiz
You renamed `aws_instance.web` to `aws_instance.app` in the configuration. How do you avoid a destroy and create?
- Use -target
* Add a moved block from the old to the new address
- Run terraform taint
- Use ignore_changes
? Or terraform state mv.

@@quiz
Which command replaces the deprecated `terraform taint`?
* terraform apply -replace=ADDRESS
- terraform state rm
- terraform plan -destroy
- terraform untaint
? It plans a replacement for that resource.

@@quiz
How do you make sure `apply` performs exactly what was reviewed?
- Run plan twice
* Save the plan with -out and apply that file
- Use -auto-approve
- Use -refresh=false
? A saved plan is applied as-is (and fails if state changed).

@@quiz
Why can't a backend block use `var.bucket_name`?
- It can
* Backends are initialised before variables are evaluated; use -backend-config instead
- Variables are sensitive
- Only HCP Terraform supports variables
? Partial backend configuration.

@@flashcards
+ :: create
- :: destroy
~ :: update in place
-/+ :: destroy then create
+/- :: create before destroy
<= :: read data source
forces replacement :: can't update in place
plan -out :: save the reviewed plan
show -json :: plan for policy tools
drift :: changes outside Terraform
apply -refresh-only :: accept real values into state
-replace :: force replacement (replaces taint)
-target :: exceptional recovery only
import block :: bring existing resources in
-generate-config-out :: write config for imports
moved block :: rename without destroy
removed block :: forget without destroy
state rm :: stop managing
force-unlock :: release a stuck lock
backend block :: no variables, use -backend-config
init -migrate-state :: move state to a new backend

@@drill
Which plan symbol means destroy and then create?
= -/+
? +/- means create first.

@@drill
Which lifecycle argument stops Terraform from reverting changes to certain attributes?
= ignore_changes
? E.g. tags managed by another tool.

@@exercise Compute a plan
Compare desired configuration with current state and produce Terraform-style plan lines and the summary. Resources only in config are created (`+`), only in state destroyed (`-`); for resources in both, any changed attribute in `FORCE_NEW[type]` means replacement (`-/+`, or `+/-` if `create_before_destroy`), otherwise `~` update. Unchanged resources are skipped. Count replacements as one add and one destroy.
@@lang python
@@starter
FORCE_NEW = {"aws_instance": {"ami", "availability_zone"}, "aws_subnet": {"cidr_block", "vpc_id"}, "aws_s3_bucket": {"bucket"}}
state = {
    "aws_instance.web": {"ami": "ami-111", "instance_type": "t3.micro", "availability_zone": "eu-west-1a"},
    "aws_instance.api": {"ami": "ami-111", "instance_type": "t3.small", "availability_zone": "eu-west-1a"},
    "aws_subnet.a": {"cidr_block": "10.0.1.0/24", "vpc_id": "vpc-1"},
    "aws_s3_bucket.logs": {"bucket": "acme-logs", "tags": {"env": "prod"}},
    "aws_s3_bucket.old": {"bucket": "acme-old", "tags": {}},
}
config = {
    "aws_instance.web": ({"ami": "ami-222", "instance_type": "t3.micro", "availability_zone": "eu-west-1a"}, {"create_before_destroy": True}),
    "aws_instance.api": ({"ami": "ami-111", "instance_type": "t3.medium", "availability_zone": "eu-west-1a"}, {}),
    "aws_subnet.a": ({"cidr_block": "10.0.1.0/24", "vpc_id": "vpc-1"}, {}),
    "aws_s3_bucket.logs": ({"bucket": "acme-logs", "tags": {"env": "prod", "owner": "ops"}}, {}),
    "aws_subnet.b": ({"cidr_block": "10.0.2.0/24", "vpc_id": "vpc-1"}, {}),
}

def action(address):
    raise NotImplementedError
@@expected
    ~ aws_instance.api (instance_type)
  +/- aws_instance.web (ami # forces replacement)
    ~ aws_s3_bucket.logs (tags)
    - aws_s3_bucket.old
    + aws_subnet.b
Plan: 2 to add, 2 to change, 2 to destroy.
@@requires
def action
FORCE_NEW
"+/-"
@@hint
The resource type is `address.split(".")[0]`. Changed keys: `{k for k in desired if desired[k] != current.get(k)}`. Sort addresses for a stable plan.
@@solution
FORCE_NEW = {"aws_instance": {"ami", "availability_zone"}, "aws_subnet": {"cidr_block", "vpc_id"}, "aws_s3_bucket": {"bucket"}}
state = {
    "aws_instance.web": {"ami": "ami-111", "instance_type": "t3.micro", "availability_zone": "eu-west-1a"},
    "aws_instance.api": {"ami": "ami-111", "instance_type": "t3.small", "availability_zone": "eu-west-1a"},
    "aws_subnet.a": {"cidr_block": "10.0.1.0/24", "vpc_id": "vpc-1"},
    "aws_s3_bucket.logs": {"bucket": "acme-logs", "tags": {"env": "prod"}},
    "aws_s3_bucket.old": {"bucket": "acme-old", "tags": {}},
}
config = {
    "aws_instance.web": ({"ami": "ami-222", "instance_type": "t3.micro", "availability_zone": "eu-west-1a"}, {"create_before_destroy": True}),
    "aws_instance.api": ({"ami": "ami-111", "instance_type": "t3.medium", "availability_zone": "eu-west-1a"}, {}),
    "aws_subnet.a": ({"cidr_block": "10.0.1.0/24", "vpc_id": "vpc-1"}, {}),
    "aws_s3_bucket.logs": ({"bucket": "acme-logs", "tags": {"env": "prod", "owner": "ops"}}, {}),
    "aws_subnet.b": ({"cidr_block": "10.0.2.0/24", "vpc_id": "vpc-1"}, {}),
}

def action(address):
    if address not in state:
        return "+", []
    if address not in config:
        return "-", []
    desired, lifecycle = config[address]
    current = state[address]
    changed = sorted(k for k in desired if desired[k] != current.get(k))
    if not changed:
        return None, []
    forced = [k for k in changed if k in FORCE_NEW[address.split(".")[0]]]
    if forced:
        return ("+/-" if lifecycle.get("create_before_destroy") else "-/+"), [f"{k} # forces replacement" if k in forced else k for k in changed]
    return "~", changed

add = change = destroy = 0
for address in sorted(set(state) | set(config)):
    symbol, details = action(address)
    if symbol is None:
        continue
    note = f" ({', '.join(details)})" if details else ""
    print(f"  {symbol:>3} {address}{note}")
    add += symbol in ("+", "-/+", "+/-")
    destroy += symbol in ("-", "-/+", "+/-")
    change += symbol == "~"
print(f"Plan: {add} to add, {change} to change, {destroy} to destroy.")

@@exercise Apply refactoring blocks to state
Process `moved` and `removed` blocks against state before planning. A `moved` block renames an address (error if the destination already exists; ignored if the source isn't in state, because it was already moved). A `removed` block with `destroy = False` drops the address from state without destroying; with `destroy = True` it's planned for destruction. Print what happens and the resulting state addresses.
@@lang python
@@starter
state = {"aws_instance.web", "aws_instance.api", "aws_s3_bucket.logs", "module.net.aws_vpc.main", "aws_instance.legacy", "aws_instance.batch"}
blocks = [
    ("moved", "aws_instance.web", 'aws_instance.app["web"]'),
    ("moved", "aws_instance.api", 'aws_instance.app["api"]'),
    ("moved", "module.net.aws_vpc.main", "module.network.aws_vpc.main"),
    ("moved", "aws_instance.cache", 'aws_instance.app["cache"]'),
    ("moved", "aws_s3_bucket.logs", 'aws_instance.app["web"]'),
    ("removed", "aws_instance.legacy", False),
    ("removed", "aws_instance.batch", True),
]

def apply_block(block):
    raise NotImplementedError
@@expected
moved aws_instance.web -> aws_instance.app["web"]
moved aws_instance.api -> aws_instance.app["api"]
moved module.net.aws_vpc.main -> module.network.aws_vpc.main
moved aws_instance.cache: not in state, nothing to do
moved aws_s3_bucket.logs: error, aws_instance.app["web"] already exists
removed aws_instance.legacy: forgotten, real resource kept
removed aws_instance.batch: will be destroyed
state:
  aws_instance.app["api"]
  aws_instance.app["web"]
  aws_instance.batch  (planned destroy)
  aws_s3_bucket.logs
  module.network.aws_vpc.main
@@requires
def apply_block
"moved"
"removed"
@@hint
Use a set for state and a list for planned destroys. For `moved`, check `src in state` and `dst in state` before renaming.
@@solution
state = {"aws_instance.web", "aws_instance.api", "aws_s3_bucket.logs", "module.net.aws_vpc.main", "aws_instance.legacy", "aws_instance.batch"}
blocks = [
    ("moved", "aws_instance.web", 'aws_instance.app["web"]'),
    ("moved", "aws_instance.api", 'aws_instance.app["api"]'),
    ("moved", "module.net.aws_vpc.main", "module.network.aws_vpc.main"),
    ("moved", "aws_instance.cache", 'aws_instance.app["cache"]'),
    ("moved", "aws_s3_bucket.logs", 'aws_instance.app["web"]'),
    ("removed", "aws_instance.legacy", False),
    ("removed", "aws_instance.batch", True),
]
to_destroy = []

def apply_block(block):
    kind, src, arg = block
    if kind == "moved":
        if src not in state:
            return f"moved {src}: not in state, nothing to do"
        if arg in state:
            return f"moved {src}: error, {arg} already exists"
        state.remove(src)
        state.add(arg)
        return f"moved {src} -> {arg}"
    if src not in state:
        return f"removed {src}: not in state"
    if arg:
        to_destroy.append(src)
        return f"removed {src}: will be destroyed"
    state.remove(src)
    return f"removed {src}: forgotten, real resource kept"

for block in blocks:
    print(apply_block(block))
print("state:")
for address in sorted(state):
    print("  " + address + ("  (planned destroy)" if address in to_destroy else ""))
