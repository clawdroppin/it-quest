@@lesson tf3-hands-on Hands-on HCL: Expressions, Functions and Refactoring
@@concepts maps, lists, strings, automation
@@lang python
@@text
The Associate exam (003/004) asks you to **read** HCL and predict what Terraform will do: which addresses a `for_each` creates, what a function returns, why a plan wants to destroy something. This lesson works through the expressions you'll meet most, and each exercise rebuilds one of them in Python so you understand exactly how it behaves. Install Terraform and try every snippet in `terraform console` as you go: it evaluates expressions without creating anything.

```text
$ terraform console
> cidrsubnet("10.0.0.0/16", 8, 2)
"10.0.2.0/24"
> merge({a = 1, b = 2}, {b = 3})
{ "a" = 1, "b" = 3 }
> [for s in ["web", "api"] : upper(s)]
[ "WEB", "API" ]
```

## Types and variables

```hcl
variable "environment" {
  type    = string
  default = "dev"
  validation {
    condition     = contains(["dev", "staging", "prod"], var.environment)
    error_message = "environment must be dev, staging or prod."
  }
}

variable "instances" {
  type = map(object({
    size  = string
    disk  = optional(number, 20)
  }))
}

variable "db_password" {
  type      = string
  sensitive = true          # hidden in plan output, but still stored in state!
}
```

Primitive types: `string`, `number`, `bool`. Collections: `list(...)`, `set(...)`, `map(...)`. Structural: `object({...})`, `tuple([...])`. `any` lets Terraform infer. Values come from (lowest to highest precedence) defaults, `terraform.tfvars`, `*.auto.tfvars`, `-var-file`, `-var`, plus `TF_VAR_name` environment variables (just above defaults).

## count versus for_each

```hcl
resource "aws_iam_user" "by_count" {
  count = length(var.users)            # aws_iam_user.by_count[0], [1], [2]
  name  = var.users[count.index]
}

resource "aws_iam_user" "by_key" {
  for_each = toset(var.users)          # aws_iam_user.by_key["aiko"], ["ken"], ...
  name     = each.key
}
```

The difference matters when the list changes. Remove the **first** user and `count` shifts every index: Terraform plans to *rename* the users at `[0]` and `[1]` and destroy `[2]`. With `for_each` only the removed key is destroyed. Rule: use `count` for "zero or one" (`count = var.enabled ? 1 : 0`) and for truly identical copies, `for_each` for anything with an identity. The first exercise computes both plans.

## Expressions

```hcl
locals {
  name_prefix = "${var.project}-${var.environment}"            # interpolation
  is_prod     = var.environment == "prod"
  size        = local.is_prod ? "m7i.large" : "t3.micro"         # conditional
  upper_names = [for n in var.names : upper(n)]                  # list comprehension
  by_name     = {for u in var.users : u.name => u.role}          # map comprehension
  admins      = [for u in var.users : u.name if u.role == "admin"]
  all_ids     = aws_instance.web[*].id                           # splat
  common_tags = merge(var.default_tags, { Environment = var.environment })
}
```

## Functions worth knowing cold

| Group | Functions |
|---|---|
| strings | `format`, `join`, `split`, `replace`, `lower`/`upper`, `trimspace`, `substr`, `regex` |
| collections | `length`, `element`, `index`, `contains`, `concat`, `flatten`, `distinct`, `keys`, `values`, `lookup`, `merge`, `zipmap`, `setproduct`, `toset`, `tomap` |
| numbers | `min`, `max`, `ceil`, `floor` |
| networking | `cidrsubnet`, `cidrhost`, `cidrnetmask` |
| encoding/files | `jsonencode`, `yamlencode`, `base64encode`, `file`, `templatefile` |
| type/safety | `try`, `can`, `coalesce`, `tostring`, `tonumber` |

You can't write your own functions (until provider-defined functions in 1.8+, which come from providers, not your code). The second exercise implements `cidrsubnet` and `cidrhost`; the third `lookup`, `merge` and `flatten`.

## Nested loops with flatten and setproduct

A common real task: one resource per (environment, region) or per (VPC, subnet).

```hcl
locals {
  subnet_list = flatten([
    for vpc_name, vpc in var.vpcs : [
      for subnet_name, cidr in vpc.subnets : {
        key  = "${vpc_name}.${subnet_name}"
        vpc  = vpc_name
        cidr = cidr
      }
    ]
  ])
}

resource "aws_subnet" "all" {
  for_each   = { for s in local.subnet_list : s.key => s }   # for_each needs a map or set
  vpc_id     = aws_vpc.this[each.value.vpc].id
  cidr_block = each.value.cidr
}
```

## Dependencies, lifecycle and meta-arguments

- **Implicit dependencies** come from references (`aws_vpc.main.id`); `depends_on` is only for hidden ones (an IAM policy that must exist before a Lambda runs).
- `lifecycle { create_before_destroy = true }` for zero-downtime replacement; `prevent_destroy = true` on databases and state buckets; `ignore_changes = [tags["LastScanned"]]` for attributes something else manages; `replace_triggered_by = [...]` to force replacement.
- `precondition` and `postcondition` blocks check assumptions with clear error messages; `check` blocks run assertions on every plan without blocking.
- `provider` meta-argument with **aliases** for multi-region: `provider = aws.osaka`.

## Refactoring without destroying

| Situation | Tool |
|---|---|
| Renamed a resource or moved it into a module | `moved { from = aws_instance.web  to = module.web.aws_instance.this }` (or `terraform state mv`) |
| Something exists already and should be managed | `import { to = aws_s3_bucket.logs  id = "acme-logs" }` then `terraform plan -generate-config-out=gen.tf` (or `terraform import`) |
| Stop managing it but keep it | `removed { from = aws_instance.old  lifecycle { destroy = false } }` (or `terraform state rm`) |
| Force recreation | `terraform apply -replace=aws_instance.web` (`taint` is deprecated) |
| Accept out-of-band changes into state | `terraform apply -refresh-only` |

## Exam prep checklist

- The workflow: `init` (providers, modules, backend) → `validate` → `fmt` → `plan` → `apply` → `destroy`; `plan -out=tfplan` then `apply tfplan`.
- `.terraform.lock.hcl` pins provider **versions and hashes**: commit it; `init -upgrade` updates it.
- Version constraints: `~> 5.0` means `>= 5.0, < 6.0`; `~> 5.4.1` means `>= 5.4.1, < 5.5.0`.
- State: what it holds (including secrets in plain text), remote backends with locking, `terraform state list/show`, workspaces (CLI) vs HCP Terraform workspaces.
- Modules: sources (registry, Git, local path), `version` only for registry modules, inputs/outputs, `terraform get`.
- HCP Terraform: remote runs, VCS-driven workflow, variable sets, Sentinel/OPA policies, private registry, run triggers, cost estimation.
- Debugging: `TF_LOG=DEBUG`, `TF_LOG_PATH`.

@@quiz
`var.users = ["aiko", "ken", "mei"]` is used with `count`. You remove `"aiko"`. What does the plan show?
- Only aiko is destroyed
* Index 0 and 1 change (aiko→ken, ken→mei) and index 2 is destroyed
- Nothing: count ignores list changes
- Every user is destroyed and recreated with new names
? Indexes are the identity with count; for_each uses keys instead.

@@quiz
What does `cidrsubnet("10.0.0.0/16", 8, 3)` return?
- "10.0.0.3/16"
* "10.0.3.0/24"
- "10.3.0.0/8"
- "10.0.0.0/19"
? Add 8 bits to the prefix (/24) and take network number 3.

@@quiz
Which constraint allows 5.4.1 and 5.4.9 but not 5.5.0?
* ~> 5.4.1
- ~> 5.4
- >= 5.4.1
- = 5.4.1
? The pessimistic operator lets only the rightmost given part increase.

@@quiz
You renamed `aws_instance.web` to `aws_instance.frontend`. How do you avoid a destroy and create?
- Run terraform apply -replace
* Add a moved block from the old address to the new one (or terraform state mv)
- Delete the state file
- Use -refresh-only
? moved blocks record refactors in code, so everyone's state updates on the next apply.

@@quiz
Is a variable marked `sensitive = true` safe in the state file?
- Yes, it's encrypted automatically
* No: it's hidden in CLI output, but stored in plain text in state, so protect the state backend
- Yes, it isn't saved at all
- Only in HCP Terraform
? Use an encrypted backend with tight access, or ephemeral values and external secret stores.

@@flashcards
terraform console :: evaluate expressions interactively
variable validation :: condition + error_message on input values
optional(type, default) :: object attribute with a default
TF_VAR_name :: set a variable from the environment
count :: copies addressed by index, for 0/1 or identical things
for_each :: instances addressed by key, stable when the collection changes
[for x in list : f(x) if cond] :: list comprehension
{for x in list : k => v} :: map comprehension
splat [*] :: attribute from every element
flatten + map comprehension :: nested loops for for_each
cidrsubnet(prefix, newbits, netnum) :: carve a subnet
cidrhost(prefix, hostnum) :: nth address in a range
lookup(map, key, default) :: map value with a fallback
merge(a, b) :: later maps win
try / can :: tolerate failing expressions
create_before_destroy / prevent_destroy / ignore_changes :: lifecycle controls
moved / import / removed blocks :: refactor, adopt and forget resources in code
-replace / -refresh-only :: force recreation / accept real-world changes into state
~> 5.4.1 :: >= 5.4.1 and < 5.5.0
.terraform.lock.hcl :: pinned provider versions and hashes

@@exercise count versus for_each
Write `count_plan(old, new)` and `foreach_plan(old, new)`. Both get the old and new lists of names and return a sorted list of changes. With **count** the address is the index: compare position by position and emit `"update [i]: old -> new"` when the name differs, `"create [i]: name"` for new positions and `"destroy [i]: name"` for positions that disappear. With **for_each** the address is the name: `"create [\"name\"]"` and `"destroy [\"name\"]"` only.
@@starter
def count_plan(old, new):
    pass


def foreach_plan(old, new):
    pass


old = ["aiko", "ken", "mei"]
new = ["ken", "mei"]
print("count:   ", count_plan(old, new))
print("for_each:", foreach_plan(old, new))
@@tests
old = ["aiko", "ken", "mei"]
new = ["ken", "mei"]
check(count_plan(old, new), ["destroy [2]: mei", "update [0]: aiko -> ken", "update [1]: ken -> mei"], "count shifts everything")
check(foreach_plan(old, new), ['destroy ["aiko"]'], "for_each removes one")
check(count_plan(["a"], ["a", "b"]), ["create [1]: b"], "appending is fine with count")
check(foreach_plan(["a", "b"], ["b", "c"]), ['create ["c"]', 'destroy ["a"]'], "create and destroy")
check(count_plan(["a", "b"], ["a", "b"]), [], "no changes")
@@hint
For count, loop `i` over `range(max(len(old), len(new)))` and compare `old[i]` with `new[i]` when both exist. For for_each, use set differences `set(new) - set(old)` and `set(old) - set(new)`. Return `sorted(changes)`.
@@solution
def count_plan(old, new):
    changes = []
    for i in range(max(len(old), len(new))):
        if i >= len(old):
            changes.append(f"create [{i}]: {new[i]}")
        elif i >= len(new):
            changes.append(f"destroy [{i}]: {old[i]}")
        elif old[i] != new[i]:
            changes.append(f"update [{i}]: {old[i]} -> {new[i]}")
    return sorted(changes)


def foreach_plan(old, new):
    changes = [f'create ["{n}"]' for n in set(new) - set(old)]
    changes += [f'destroy ["{n}"]' for n in set(old) - set(new)]
    return sorted(changes)


old = ["aiko", "ken", "mei"]
new = ["ken", "mei"]
print("count:   ", count_plan(old, new))
print("for_each:", foreach_plan(old, new))

@@exercise cidrsubnet and cidrhost
Implement Terraform's `cidrsubnet(prefix, newbits, netnum)` (extend the prefix length by `newbits` and return network number `netnum` of that size; raise `ValueError` if `netnum` doesn't fit) and `cidrhost(prefix, hostnum)` (the address `hostnum` places after the network address; negative numbers count back from the end, so `-1` is the last address). Use `ipaddress`.
@@starter
import ipaddress


def cidrsubnet(prefix, newbits, netnum):
    pass


def cidrhost(prefix, hostnum):
    pass


print(cidrsubnet("10.0.0.0/16", 8, 2))
print(cidrsubnet("10.0.0.0/16", 4, 15))
print(cidrhost("10.0.2.0/24", 10))
@@tests
check(cidrsubnet("10.0.0.0/16", 8, 2), "10.0.2.0/24", "a /24")
check(cidrsubnet("10.0.0.0/16", 4, 15), "10.0.240.0/20", "the last /20")
check(cidrsubnet("172.16.0.0/12", 4, 1), "172.17.0.0/16", "a /16 from a /12")
try:
    cidrsubnet("10.0.0.0/16", 2, 4)
    check("no error", "ValueError", "netnum too big")
except ValueError:
    check(True, True, "netnum too big")
check(cidrhost("10.0.2.0/24", 10), "10.0.2.10", "tenth host")
check(cidrhost("10.0.2.0/24", -1), "10.0.2.255", "last address")
check(cidrhost("10.12.112.0/20", 268), "10.12.113.12", "crosses an octet")
@@hint
`net = ipaddress.ip_network(prefix)`; the new subnet starts at `int(net.network_address) + netnum * 2 ** (32 - new_len)`. Check `netnum < 2 ** newbits`. For hosts, a negative number is `net.num_addresses + hostnum`.
@@solution
import ipaddress


def cidrsubnet(prefix, newbits, netnum):
    net = ipaddress.ip_network(prefix)
    if not 0 <= netnum < 2 ** newbits:
        raise ValueError(f"netnum {netnum} doesn't fit in {newbits} bits")
    new_len = net.prefixlen + newbits
    start = int(net.network_address) + netnum * 2 ** (32 - new_len)
    return f"{ipaddress.ip_address(start)}/{new_len}"


def cidrhost(prefix, hostnum):
    net = ipaddress.ip_network(prefix)
    if hostnum < 0:
        hostnum = net.num_addresses + hostnum
    return str(net.network_address + hostnum)


print(cidrsubnet("10.0.0.0/16", 8, 2))
print(cidrsubnet("10.0.0.0/16", 4, 15))
print(cidrhost("10.0.2.0/24", 10))

@@exercise Collection functions and nested for_each
Implement `lookup(m, key, default)`, `merge(*maps)` (later maps win) and `flatten(value)` (nested lists of any depth become one flat list). Then `subnet_map(vpcs)`: given `{"prod": {"a": "10.0.1.0/24", "b": "10.0.2.0/24"}, "dev": {...}}`, return the map a `for_each` needs: keys `"prod.a"` with values `{"vpc": "prod", "cidr": "10.0.1.0/24"}`, built with `flatten` the way the lesson's HCL does.
@@starter
def lookup(m, key, default):
    pass


def merge(*maps):
    pass


def flatten(value):
    pass


def subnet_map(vpcs):
    pass


print(merge({"Owner": "platform", "Environment": "dev"}, {"Environment": "prod"}))
print(flatten([[1, 2], [3, [4, 5]], []]))
print(subnet_map({"prod": {"a": "10.0.1.0/24"}, "dev": {"a": "10.1.1.0/24"}}))
@@tests
check(lookup({"t3": 2}, "t3", 1), 2, "found")
check(lookup({"t3": 2}, "m7i", 1), 1, "default")
check(merge({"a": 1, "b": 2}, {"b": 3}, {"c": 4}), {"a": 1, "b": 3, "c": 4}, "later wins")
check(merge(), {}, "nothing")
check(flatten([[1, 2], [3, [4, 5]], []]), [1, 2, 3, 4, 5], "any depth")
check(flatten(["x"]), ["x"], "already flat")
vpcs = {"prod": {"a": "10.0.1.0/24", "b": "10.0.2.0/24"}, "dev": {"a": "10.1.1.0/24"}}
check(subnet_map(vpcs), {"prod.a": {"vpc": "prod", "cidr": "10.0.1.0/24"}, "prod.b": {"vpc": "prod", "cidr": "10.0.2.0/24"}, "dev.a": {"vpc": "dev", "cidr": "10.1.1.0/24"}}, "keys for for_each")
@@hint
`flatten` is recursive: for each item, if it's a list, extend with `flatten(item)`, else append it. For `subnet_map`, build a list of lists of dicts (one inner list per VPC), flatten it, then make `{s["key"]: {...} for s in flat}`.
@@solution
def lookup(m, key, default):
    return m[key] if key in m else default


def merge(*maps):
    out = {}
    for m in maps:
        out.update(m)
    return out


def flatten(value):
    flat = []
    for item in value:
        if isinstance(item, list):
            flat.extend(flatten(item))
        else:
            flat.append(item)
    return flat


def subnet_map(vpcs):
    nested = [[{"key": f"{vpc}.{name}", "vpc": vpc, "cidr": cidr} for name, cidr in subnets.items()]
              for vpc, subnets in vpcs.items()]
    return {s["key"]: {"vpc": s["vpc"], "cidr": s["cidr"]} for s in flatten(nested)}


print(merge({"Owner": "platform", "Environment": "dev"}, {"Environment": "prod"}))
print(flatten([[1, 2], [3, [4, 5]], []]))
print(subnet_map({"prod": {"a": "10.0.1.0/24"}, "dev": {"a": "10.1.1.0/24"}}))
