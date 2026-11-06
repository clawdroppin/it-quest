@@lesson tf1-state-modules State, Modules and HCP Terraform
@@text
## State

Terraform records what it manages in the **state** (`terraform.tfstate`): a JSON map from each resource address to the real object's ID and attributes. Plan compares three things: **your code**, **the state**, and **reality** (refreshed from the provider).

- The state can contain **secrets** (database passwords, keys) in plain text. Never commit it; restrict who can read it.
- With a team, keep it in a **remote backend** with **locking**, so two people can't apply at once:

```hcl
terraform {
  backend "s3" {
    bucket       = "acme-tfstate"
    key          = "shop/prod/terraform.tfstate"
    region       = "ap-northeast-1"
    encrypt      = true
    use_lockfile = true     # S3-native locking (older setups used a DynamoDB table)
  }
}
```

Other backends: `azurerm` (blob storage), `gcs`, `consul`, `kubernetes`, and HCP Terraform (`cloud` block). Backends can't use variables; changing one needs `terraform init -migrate-state`.

State commands (never edit the file by hand):

| Command | Use |
|---|---|
| `terraform state list` | every address in the state |
| `terraform state show aws_instance.web[0]` | its recorded attributes |
| `terraform state mv` | rename an address (or use a `moved` block in code) |
| `terraform state rm` | stop managing something without destroying it (or a `removed` block) |
| `terraform import` / **`import` block** | bring an existing resource under management |
| `terraform force-unlock ID` | release a stuck lock (carefully) |

```hcl
import {                                 # Terraform 1.5+: import in code, reviewed in the plan
  to = aws_s3_bucket.logs
  id = "acme-legacy-logs"
}
moved {                                  # a rename that doesn't destroy anything
  from = aws_instance.web
  to   = aws_instance.app
}
```

`terraform plan -generate-config-out=generated.tf` even writes the resource block for an imported object.

## Modules

A **module** is any folder of `.tf` files. The folder you run Terraform in is the **root module**; others are **child modules** you call:

```hcl
module "network" {
  source  = "terraform-aws-modules/vpc/aws"   # from the public registry
  version = "~> 5.0"                          # pin versions for registry modules
  name    = "shop"
  cidr    = "10.0.0.0/16"
  azs     = ["ap-northeast-1a", "ap-northeast-1c"]
}

module "app" {
  source     = "./modules/app"                # a local path (no version argument)
  subnet_ids = module.network.private_subnets # a child module's output
}
```

- Inputs are the module's **variables**; it exposes only its **outputs**. A parent can't read a child's resources directly.
- Sources: local paths, the **Terraform Registry** (`namespace/name/provider`), Git (`git::https://...?ref=v1.2.0`), HTTP archives, S3/GCS buckets, a private registry in HCP Terraform.
- After adding or changing a module source, run `terraform init` again.
- Good modules are small and focused (a network, a service), have typed and validated variables, and sensible defaults.

## Workspaces and environments

- **CLI workspaces** (`terraform workspace new staging`, `select`, `list`) keep separate states for the same code; `terraform.workspace` gives the name. Fine for similar, short-lived copies.
- For real dev/staging/prod separation, many teams prefer **separate directories or root modules** per environment (often with separate accounts and backends), so a mistake in dev can't touch prod.

## HCP Terraform (formerly Terraform Cloud)

- **Remote runs**: plan and apply run on HashiCorp's infrastructure (or your own **agents**), with the state stored, encrypted and locked for you.
- **Workspaces** there are per configuration and environment (different from CLI workspaces): each has its own variables, state and permissions, and can be connected to a VCS repository (a pull request triggers a speculative plan).
- **Sentinel** and **OPA** policies (policy as code) can block a run: "no instance bigger than m5.large", "every resource must have a cost-center tag". Cost estimation and drift detection are also available on paid tiers.
- A private module and provider **registry**.
- `terraform login` stores an API token; the `cloud` block connects a configuration.

## Good practice

- Format, validate and plan in CI on every pull request; apply only from the main branch, with the reviewed plan.
- Pin provider and module versions.
- Use `prevent_destroy` on resources that must never disappear:

```hcl
resource "aws_db_instance" "main" {
  # ...
  lifecycle {
    prevent_destroy       = true   # plan fails if anything would destroy it
    create_before_destroy = false
    ignore_changes        = [tags["LastModifiedBy"]]
  }
}
```

- Keep secrets out of code: read them from a secret manager (a `data` source or ephemeral resources), or from environment variables in the pipeline.
- Scan the code (tfsec/Trivy, Checkov) and the plan (your own reviewer, in this track's project).

@@quiz
Why use a remote backend with locking for a team?
* So the state is shared and two applies can't run against it at the same time
- To make plans faster
- Because local state is not supported any more
- To avoid downloading providers
? Locking prevents corrupted state from concurrent runs.

@@quiz
A bucket was created by hand years ago. How do you bring it under Terraform management without recreating it?
* Write its resource block and use an `import` block (or `terraform import`), then plan
- `terraform apply` with the same name
- `terraform state rm`
- `terraform taint`
? The plan should then show no changes, or only the differences you accept.

@@quiz
You renamed `aws_instance.web` to `aws_instance.app` in the code. How do you avoid destroying and recreating it?
* Add a `moved` block (or run `terraform state mv`)
- Run `terraform refresh`
- Delete the state file
- Use `-replace`
? Without it, Terraform sees one resource removed and a new one added.

@@quiz
Which statement about sensitive values in state is true?
* They are stored in plain text in the state, so the state itself must be protected
- They are encrypted in the state automatically
- They are never written to the state
- Only outputs are stored
? `sensitive = true` only hides them in CLI output.

@@quiz
In HCP Terraform, what can block a run that creates untagged resources?
* A Sentinel or OPA policy
- A CLI workspace
- `terraform validate`
- A provider version constraint
? Policy as code runs between plan and apply.

@@quiz
Which module source needs a `version` argument to pin it?
* A Terraform Registry module
- A local path like ./modules/app
- Both
- Neither: modules can't be pinned
? Git sources are pinned with `?ref=` instead.

@@flashcards
state :: maps resource addresses to real objects
remote backend :: shared state with locking
init -migrate-state :: move state to a new backend
terraform state rm / removed block :: forget a resource without destroying it
import block :: adopt existing infrastructure, in code
moved block :: rename without recreating
root vs child module :: where you run / modules you call
module outputs :: the only way a parent reads a module's values
terraform.workspace :: the current CLI workspace's name
HCP Terraform workspace :: state, variables and permissions per configuration
Sentinel / OPA :: policy as code
prevent_destroy :: plan fails if the resource would be destroyed
create_before_destroy :: build the replacement first

@@drill
Which lifecycle argument makes a plan fail if a resource would be destroyed?
= prevent_destroy

@@exercise Where does each workspace's state live?
With the S3 backend, the `default` workspace stores its state at `key`, and every other workspace at `<workspace_key_prefix>/<workspace>/<key>` (the prefix defaults to `env:`). Given a backend configuration and the workspaces, print each state path, the resource names a configuration would produce with `"${var.app}-${terraform.workspace}"`, and flag workspace names that would make invalid S3 bucket names (bucket names allow only lowercase letters, digits, dots and hyphens, 3–63 characters).
@@lang python
@@starter
import re

backend = {"bucket": "acme-tf-state", "key": "network/terraform.tfstate"}
app = "shop"
workspaces = ["default", "dev", "staging", "Feature_Login", "prod"]

def state_path(backend, workspace):
    raise NotImplementedError

def bucket_name(app, workspace):
    raise NotImplementedError

for ws in workspaces:
    name, ok = bucket_name(app, ws)
    print(f"{ws:<14} s3://{backend['bucket']}/{state_path(backend, ws):<42} bucket {name}" + ("" if ok else "  INVALID"))
@@expected
default        s3://acme-tf-state/network/terraform.tfstate                  bucket shop-default
dev            s3://acme-tf-state/env:/dev/network/terraform.tfstate         bucket shop-dev
staging        s3://acme-tf-state/env:/staging/network/terraform.tfstate     bucket shop-staging
Feature_Login  s3://acme-tf-state/env:/Feature_Login/network/terraform.tfstate bucket shop-Feature_Login  INVALID
prod           s3://acme-tf-state/env:/prod/network/terraform.tfstate        bucket shop-prod
@@requires
def state_path
workspace_key_prefix
@@hint
`prefix = backend.get("workspace_key_prefix", "env:")`. The default workspace returns `backend["key"]`; others return `f"{prefix}/{workspace}/{backend['key']}"`. For the bucket name, build `f"{app}-{workspace}"` and test it with `re.fullmatch(r"[a-z0-9.-]{3,63}", name)`.
@@solution
import re

backend = {"bucket": "acme-tf-state", "key": "network/terraform.tfstate"}
app = "shop"
workspaces = ["default", "dev", "staging", "Feature_Login", "prod"]

def state_path(backend, workspace):
    if workspace == "default":
        return backend["key"]
    prefix = backend.get("workspace_key_prefix", "env:")
    return f"{prefix}/{workspace}/{backend['key']}"

def bucket_name(app, workspace):
    name = f"{app}-{workspace}"
    return name, bool(re.fullmatch(r"[a-z0-9.-]{3,63}", name))

for ws in workspaces:
    name, ok = bucket_name(app, ws)
    print(f"{ws:<14} s3://{backend['bucket']}/{state_path(backend, ws):<42} bucket {name}" + ("" if ok else "  INVALID"))
