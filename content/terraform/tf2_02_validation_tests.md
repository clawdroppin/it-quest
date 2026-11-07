@@lesson tf2-validation-tests Guard Rails in Code: Validation, Conditions, Checks and terraform test
@@text
Terraform gives you four layers of guard rails, and the Associate exam expects you to know which runs when:

| Feature | Where | When it runs | On failure |
|---|---|---|---|
| `validation` in a `variable` block | input variables | before plan, on the value given | error, nothing planned |
| `precondition` in `lifecycle` | resource, data source, output | before that object is planned/applied | error |
| `postcondition` in `lifecycle` | resource, data source | after the object is planned/applied | error, later dependents stop |
| `check` block (with `assert`) | top level | end of plan and apply | **warning only**, run continues |
| `terraform test` (`.tftest.hcl`) | test files | when you run the test command | test fails |

## Variable validation

```hcl
variable "environment" {
  type = string
  validation {
    condition     = contains(["dev", "staging", "prod"], var.environment)
    error_message = "environment must be dev, staging or prod."
  }
}

variable "bucket_name" {
  type = string
  validation {
    condition     = can(regex("^[a-z0-9][a-z0-9.-]{1,61}[a-z0-9]$", var.bucket_name))
    error_message = "Bucket names are 3-63 lowercase letters, digits, dots and hyphens."
  }
}
```

- `can(expr)` turns an error into `false`, which is handy around `regex`, `cidrhost` or type conversions.
- A variable can have several `validation` blocks. Since Terraform 1.9 they may also refer to other variables.
- Mark secrets with `sensitive = true`: they're redacted in plan output but **still stored in state**. Use `ephemeral` values (1.10+) to keep them out of state.

## Preconditions and postconditions

```hcl
data "aws_ami" "app" { ... }

resource "aws_instance" "app" {
  ami           = data.aws_ami.app.id
  instance_type = var.instance_type
  lifecycle {
    precondition {
      condition     = data.aws_ami.app.architecture == "x86_64"
      error_message = "The AMI must be x86_64 for this instance type."
    }
    postcondition {
      condition     = self.public_ip == ""
      error_message = "App servers must not get a public IP."
    }
  }
}
```

`self` is only available in postconditions. Preconditions document **assumptions**, and postconditions document **guarantees** that later resources rely on.

## check blocks

```hcl
check "site_up" {
  data "http" "home" { url = "https://${aws_cloudfront_distribution.site.domain_name}" }
  assert {
    condition     = data.http.home.status_code == 200
    error_message = "Home page did not return 200."
  }
}
```

Checks report problems **without blocking** the run. Use them for continuous validation, for example in HCP Terraform health assessments.

## terraform test

```hcl
# tests/naming.tftest.hcl
variables { environment = "dev" }

run "bucket_name_has_env_prefix" {
  command = plan                       # plan only: fast, no real resources
  assert {
    condition     = startswith(aws_s3_bucket.site.bucket, "dev-")
    error_message = "Bucket name should start with the environment."
  }
}

run "rejects_bad_environment" {
  command = plan
  variables { environment = "qa" }
  expect_failures = [var.environment]  # this run passes only if validation fails
}
```

- Each `run` block runs in order and shares state with the earlier ones. `command = apply` (the default) creates real resources and **destroys them at the end**.
- `mock_provider` (1.7+) fakes a provider so tests run without credentials.
- Runs: `terraform test`, or `terraform test -filter=tests/naming.tftest.hcl` for one file.

@@quiz
Which guard rail reports a failure as a warning without stopping the apply?
- Variable validation
- precondition
- postcondition
* check block
? Checks are for continuous validation.

@@quiz
Where can you use `self`?
- In variable validation
- In preconditions
* In postconditions
- In check blocks
? The object must already be planned or applied.

@@quiz
A `run` block in a .tftest.hcl file has no `command`. What does it do?
- Plan only
* Apply real resources and destroy them when the test file finishes
- Validate syntax only
- Fail
? The default command is apply.

@@quiz
What does `expect_failures = [var.environment]` mean?
* The run passes only if that variable's validation fails
- The variable is skipped
- Failures are ignored
- The variable becomes sensitive
? It's for testing negative cases.

@@quiz
A variable is marked `sensitive = true`. Where can its value still appear?
- Nowhere
- In plan output
* In the state file
- In the error messages only
? Sensitive values are redacted in output but stored in state; protect the state.

@@flashcards
validation block :: checks an input variable before plan
can(expr) :: false instead of an error
precondition :: assumption checked before the object
postcondition :: guarantee checked after, can use self
check block :: assert that only warns
terraform test :: runs .tftest.hcl files
run block :: one step of a test, in order
command = plan :: fast test, no resources
command = apply :: default, creates then destroys
expect_failures :: run passes if listed checks fail
mock_provider :: fake provider for tests
sensitive :: redacted in output, still in state
ephemeral :: kept out of plan and state

@@drill
Which function turns an expression error into false, often used in validation?
= can
= can()
? try() returns a fallback value instead.

@@drill
Which test `command` value runs without creating real resources?
= plan
? Apply is the default.

@@exercise Validate input variables
Implement validation rules like the HCL ones in the lesson (allowed values, bucket-name regex, CIDR prefix range, and a cross-variable rule) and run them against several tfvars sets, printing each variable's error message the way `terraform plan` would.
@@concepts conditions, functions, text-processing, testing, maps
@@lang python
@@starter
import re, ipaddress

def can(fn):
    try:
        return bool(fn())
    except Exception:
        return False

RULES = [
    ("environment", lambda v: v["environment"] in ("dev", "staging", "prod"), "environment must be dev, staging or prod."),
    ("bucket_name", lambda v: can(lambda: re.fullmatch(r"[a-z0-9][a-z0-9.-]{1,61}[a-z0-9]", v["bucket_name"])),
     "Bucket names are 3-63 lowercase letters, digits, dots and hyphens."),
    ("vpc_cidr", lambda v: can(lambda: 16 <= ipaddress.ip_network(v["vpc_cidr"]).prefixlen <= 24),
     "vpc_cidr must be a valid network between /16 and /24."),
    ("instance_count", lambda v: v["environment"] != "prod" or v["instance_count"] >= 2,
     "prod needs at least 2 instances."),
]
tfvars = {
    "dev.tfvars": {"environment": "dev", "bucket_name": "dev-site-assets", "vpc_cidr": "10.0.0.0/16", "instance_count": 1},
    "prod.tfvars": {"environment": "prod", "bucket_name": "Prod_Site", "vpc_cidr": "10.1.0.0/26", "instance_count": 1},
    "qa.tfvars": {"environment": "qa", "bucket_name": "qa-site", "vpc_cidr": "10.2.0.300/16", "instance_count": 3},
}

def validate(values):
    raise NotImplementedError
@@expected
dev.tfvars: valid, plan can continue
prod.tfvars: 3 error(s)
  Error: Invalid value for variable
    on variables.tf, var.bucket_name: Bucket names are 3-63 lowercase letters, digits, dots and hyphens.
  Error: Invalid value for variable
    on variables.tf, var.vpc_cidr: vpc_cidr must be a valid network between /16 and /24.
  Error: Invalid value for variable
    on variables.tf, var.instance_count: prod needs at least 2 instances.
qa.tfvars: 2 error(s)
  Error: Invalid value for variable
    on variables.tf, var.environment: environment must be dev, staging or prod.
  Error: Invalid value for variable
    on variables.tf, var.vpc_cidr: vpc_cidr must be a valid network between /16 and /24.
@@requires
def validate
RULES
can(
.items()
@@hint
Loop over `RULES`. Each rule is `(variable, condition, message)`. Collect `(variable, message)` for every condition that's false. Wrapping risky calls in `can` (as the CIDR and regex rules do) means bad input gives a validation message instead of a crash.
@@solution
import re, ipaddress

def can(fn):
    try:
        return bool(fn())
    except Exception:
        return False

RULES = [
    ("environment", lambda v: v["environment"] in ("dev", "staging", "prod"), "environment must be dev, staging or prod."),
    ("bucket_name", lambda v: can(lambda: re.fullmatch(r"[a-z0-9][a-z0-9.-]{1,61}[a-z0-9]", v["bucket_name"])),
     "Bucket names are 3-63 lowercase letters, digits, dots and hyphens."),
    ("vpc_cidr", lambda v: can(lambda: 16 <= ipaddress.ip_network(v["vpc_cidr"]).prefixlen <= 24),
     "vpc_cidr must be a valid network between /16 and /24."),
    ("instance_count", lambda v: v["environment"] != "prod" or v["instance_count"] >= 2,
     "prod needs at least 2 instances."),
]
tfvars = {
    "dev.tfvars": {"environment": "dev", "bucket_name": "dev-site-assets", "vpc_cidr": "10.0.0.0/16", "instance_count": 1},
    "prod.tfvars": {"environment": "prod", "bucket_name": "Prod_Site", "vpc_cidr": "10.1.0.0/26", "instance_count": 1},
    "qa.tfvars": {"environment": "qa", "bucket_name": "qa-site", "vpc_cidr": "10.2.0.300/16", "instance_count": 3},
}

def validate(values):
    return [(var, msg) for var, cond, msg in RULES if not cond(values)]

for name, values in tfvars.items():
    errors = validate(values)
    if not errors:
        print(f"{name}: valid, plan can continue")
        continue
    print(f"{name}: {len(errors)} error(s)")
    for var, msg in errors:
        print(f"  Error: Invalid value for variable\n    on variables.tf, var.{var}: {msg}")

@@exercise A tiny terraform test runner
Model a module as a function from variables to planned resources, then run `.tftest.hcl`-style `run` blocks in order: each run merges its variables over the file-level ones, either expects the plan to succeed and checks assertions, or uses `expect_failures` and passes only if validation fails.
@@lang python
@@starter
def plan(var):
    if var["environment"] not in ("dev", "staging", "prod"):
        raise ValueError("var.environment")
    count = 3 if var["environment"] == "prod" else 1
    return {
        "aws_s3_bucket.site": {"bucket": f"{var['environment']}-{var['name']}", "force_destroy": var["environment"] != "prod"},
        **{f"aws_instance.web[{i}]": {"instance_type": var.get("size", "t3.micro")} for i in range(count)},
    }

file_vars = {"environment": "dev", "name": "shop"}
runs = [
    {"name": "bucket_has_env_prefix", "asserts": [("bucket starts with env", lambda r: r["aws_s3_bucket.site"]["bucket"].startswith("dev-"))]},
    {"name": "prod_is_protected", "vars": {"environment": "prod"},
     "asserts": [("force_destroy off", lambda r: r["aws_s3_bucket.site"]["force_destroy"] is False),
                 ("three web servers", lambda r: sum(k.startswith("aws_instance.web") for k in r) == 3)]},
    {"name": "prod_uses_large_size", "vars": {"environment": "prod"},
     "asserts": [("size is t3.large", lambda r: r["aws_instance.web[0]"]["instance_type"] == "t3.large")]},
    {"name": "rejects_qa", "vars": {"environment": "qa"}, "expect_failures": ["var.environment"]},
    {"name": "rejects_typo", "vars": {"environment": "dev"}, "expect_failures": ["var.environment"]},
]

def run_test(run):
    raise NotImplementedError
@@expected
run "bucket_has_env_prefix"... pass
run "prod_is_protected"... pass
run "prod_uses_large_size"... fail
  assertion failed: size is t3.large
run "rejects_qa"... pass
  expected failure of var.environment
run "rejects_typo"... fail
  expected var.environment to fail, but the plan succeeded
3 passed, 2 failed.
@@requires
def run_test
expect_failures
asserts
run.get(
@@hint
Merge with `{**file_vars, **run.get("vars", {})}`. Call `plan` inside `try`. If `expect_failures` is set, the run passes only if `plan` raised with a matching name. Otherwise a raise is a failure, and every assert with a false condition is a failure. Return `(passed, messages)`.
@@solution
def plan(var):
    if var["environment"] not in ("dev", "staging", "prod"):
        raise ValueError("var.environment")
    count = 3 if var["environment"] == "prod" else 1
    return {
        "aws_s3_bucket.site": {"bucket": f"{var['environment']}-{var['name']}", "force_destroy": var["environment"] != "prod"},
        **{f"aws_instance.web[{i}]": {"instance_type": var.get("size", "t3.micro")} for i in range(count)},
    }

file_vars = {"environment": "dev", "name": "shop"}
runs = [
    {"name": "bucket_has_env_prefix", "asserts": [("bucket starts with env", lambda r: r["aws_s3_bucket.site"]["bucket"].startswith("dev-"))]},
    {"name": "prod_is_protected", "vars": {"environment": "prod"},
     "asserts": [("force_destroy off", lambda r: r["aws_s3_bucket.site"]["force_destroy"] is False),
                 ("three web servers", lambda r: sum(k.startswith("aws_instance.web") for k in r) == 3)]},
    {"name": "prod_uses_large_size", "vars": {"environment": "prod"},
     "asserts": [("size is t3.large", lambda r: r["aws_instance.web[0]"]["instance_type"] == "t3.large")]},
    {"name": "rejects_qa", "vars": {"environment": "qa"}, "expect_failures": ["var.environment"]},
    {"name": "rejects_typo", "vars": {"environment": "dev"}, "expect_failures": ["var.environment"]},
]

def run_test(run):
    var = {**file_vars, **run.get("vars", {})}
    expected = run.get("expect_failures")
    try:
        resources = plan(var)
    except ValueError as err:
        if expected and str(err) in expected:
            return True, [f"expected failure of {err}"]
        return False, [f"unexpected error: {err}"]
    if expected:
        return False, [f"expected {', '.join(expected)} to fail, but the plan succeeded"]
    failed = [name for name, cond in run.get("asserts", []) if not cond(resources)]
    return not failed, [f"assertion failed: {f}" for f in failed]

passed = 0
for run in runs:
    ok, messages = run_test(run)
    passed += ok
    print(f"run \"{run['name']}\"... {'pass' if ok else 'fail'}")
    for m in messages:
        print(f"  {m}")
print(f"{passed} passed, {len(runs) - passed} failed.")
