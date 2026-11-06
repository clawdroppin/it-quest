@@lesson tf3-secrets-security Secrets and Security in Terraform: Sensitive Values, State, OIDC, Ephemeral Values and Scanners
@@concepts maps, text-processing, conditions
@@lang python
@@text
Terraform touches the most privileged credentials in an organisation: it can create and destroy whole environments. Getting security wrong usually happens in one of four places: **secrets in code**, **secrets in state**, **over-privileged credentials in CI**, and **insecure resources** that pass review because nobody reads 3,000 lines of plan. This lesson covers each, with the features Terraform added recently to help.

## Sensitive values

- Mark variables and outputs that hold secrets with `sensitive = true`. Terraform then hides them in plan and apply output (`(sensitive value)`).
- Sensitivity **propagates**: an expression that uses a sensitive value is sensitive too. An output that exposes one without `sensitive = true` is an **error**, which forces you to decide.
- `nonsensitive()` removes the marking. Use it only for values that are derived and safe, such as the length of a password.
- **Sensitive is not encrypted.** It only hides values in the CLI output. The values are still stored **in plain text in the state file**, and in plan files.

## State is a secret

- State contains every attribute of every resource: database passwords, private keys created by `tls_private_key`, access tokens. Treat the state file like a password vault.
- Use a **remote backend** with encryption at rest and strict access control: S3 with SSE-KMS and a bucket policy, an Azure Storage account with RBAC, GCS with IAM, or HCP Terraform. Never commit `terraform.tfstate` to Git (put `*.tfstate*` and `.terraform/` in `.gitignore`).
- Turn on versioning for the state bucket so you can recover from a bad apply, and use state locking.
- **OpenTofu** (the open source fork) can encrypt state client-side with its `encryption` block. Terraform relies on the backend's encryption.

## Keeping secrets out of state: ephemeral values

- **Ephemeral resources** (Terraform 1.10+) such as `ephemeral "aws_secretsmanager_secret_version" "db" {}` read a secret during the run and **never store it** in state or plan files.
- **Ephemeral variables and outputs** (`ephemeral = true`) can carry such values between modules.
- **Write-only arguments** (Terraform 1.11+, often named with a `_wo` suffix, e.g. `password_wo` with a `password_wo_version`) send a value to the provider without saving it in state. Bump the version argument to rotate the secret.
- Older approaches: let the cloud generate and manage the secret (e.g. `manage_master_user_password = true` on AWS RDS stores it in Secrets Manager), or read secrets from Vault at apply time.

## Credentials for Terraform itself

- Never put access keys in `.tf` files or in `provider` blocks. Use the provider's standard environment variables, a profile, or, best of all, **short-lived credentials**.
- **OIDC federation** from CI: GitHub Actions, GitLab CI and HCP Terraform can exchange a signed identity token for temporary cloud credentials (AWS `AssumeRoleWithWebIdentity`, Azure workload identity federation, GCP workload identity federation). There are no long-lived keys to leak.
- Separate the **plan** role (read-only) from the **apply** role (write), restrict the apply role to the protected branch, and scope roles per environment so a dev pipeline can't touch production.

## Scanning for insecure resources

- **Static analysis** of `.tf` files and plans:
  - **tflint** finds provider-specific mistakes and deprecated syntax.
  - **Checkov** and **Trivy** (which absorbed tfsec) check security policies such as public buckets, open security groups, unencrypted disks and missing logging.
- **Policy as code** in the pipeline: Sentinel or OPA (Open Policy Agent) policies run against the plan JSON and can block an apply.
- **Secret scanners** (gitleaks, trufflehog, GitHub secret scanning) catch keys committed to the repository. If a key is committed, **rotate it**: removing it from Git history isn't enough, because it's already been exposed.

@@quiz
Does `sensitive = true` encrypt a value in the state file?
* No: it only hides the value in CLI output; state still stores it in plain text
- Yes, with AES-256
- Yes, if the backend is local
- Only for outputs
? Protect the state itself.

@@quiz
An output uses a sensitive variable but isn't marked sensitive. What happens?
* Terraform reports an error until you mark it sensitive or use nonsensitive()
- The value is printed in plain text
- The output is silently dropped
- The variable becomes non-sensitive
? Sensitivity propagates.

@@quiz
Which feature reads a secret during a run without ever writing it to state?
* An ephemeral resource
- A data source
- A local value
- terraform_remote_state
? Terraform 1.10 and later.

@@quiz
What is the best way to give a CI pipeline cloud credentials for Terraform?
* OIDC federation for short-lived credentials, with separate plan and apply roles
- Long-lived access keys in the repository
- Access keys in the provider block
- The administrator's personal keys
? No long-lived secrets to leak.

@@quiz
An access key was committed to Git and removed in the next commit. What must you do?
* Rotate (revoke) the key, because it has already been exposed
- Nothing, it's been removed
- Rewrite history and keep the key
- Make the repository private
? Removal doesn't un-leak it.

@@quiz
Which tools scan Terraform code for insecure settings such as public buckets?
* Checkov and Trivy
- terraform fmt
- terraform graph
- terraform console
? tflint for provider-specific mistakes.

@@flashcards
sensitive = true :: hide in CLI output, not encrypted
propagation :: expressions using sensitive values are sensitive
nonsensitive() :: remove the marking for safe derived values
state :: stores all attributes in plain text
remote backend :: encryption, access control, locking, versioning
.gitignore :: *.tfstate*, .terraform/
OpenTofu state encryption :: client-side encryption block
ephemeral resource :: read secrets without storing them (1.10+)
write-only argument :: _wo, not stored in state (1.11+)
OIDC federation :: short-lived CI credentials
plan role / apply role :: read-only / write, protected branch
tflint :: provider-aware linter
Checkov / Trivy :: security policy scanners
Sentinel / OPA :: policy as code on the plan
leaked key :: rotate it

@@drill
In which file does Terraform store resource attributes, including secrets, by default? (file name)
= terraform.tfstate
? Use an encrypted remote backend.

@@drill
Which Terraform version introduced ephemeral resources? (for example 1.5)
= 1.10
? Write-only arguments followed in 1.11.

@@exercise Find secrets in a state file
Walk a state file (as parsed JSON) and report every attribute that looks like a secret: attribute names containing password, secret, token, private_key or plaintext, and string values that look like AWS access key IDs (starting with `AKIA` followed by 16 uppercase letters or digits) or PEM private keys. Print the resource address and attribute path, never the value itself.
@@starter
import json
import re

state = json.loads('''{
  "version": 4,
  "resources": [
    {"mode": "managed", "type": "aws_db_instance", "name": "main", "instances": [
      {"attributes": {"identifier": "prod-db", "username": "admin", "password": "S3cr3t-pass", "port": 5432}}]},
    {"mode": "managed", "type": "tls_private_key", "name": "deploy", "instances": [
      {"attributes": {"algorithm": "ED25519", "private_key_pem": "-----BEGIN PRIVATE KEY-----\\nMC4CAQ...\\n-----END PRIVATE KEY-----"}}]},
    {"mode": "managed", "type": "aws_iam_access_key", "name": "ci", "instances": [
      {"attributes": {"id": "AKIAIOSFODNN7EXAMPLE", "user": "ci-bot", "secret": "wJalrXUtnFEMI/K7MDENG"}}]},
    {"mode": "managed", "type": "aws_s3_bucket", "name": "logs", "instances": [
      {"attributes": {"bucket": "acme-logs", "tags": {"team": "platform"}}}]},
    {"mode": "managed", "type": "github_actions_secret", "name": "deploy", "instances": [
      {"attributes": {"secret_name": "DEPLOY_TOKEN", "plaintext_value": "ghp_abc", "settings": {"api_token": "xyz"}}}]}
  ]
}''')

SECRET_NAME = re.compile(r"password|secret|token|private_key|plaintext", re.I)
AWS_KEY = re.compile(r"^AKIA[0-9A-Z]{16}$")

def walk(value, path):
    raise NotImplementedError

for r in state["resources"]:
    address = f"{r['type']}.{r['name']}"
    for inst in r["instances"]:
        for path, reason in walk(inst["attributes"], ""):
            print(f"{address}: {path} ({reason})")
@@expected
aws_db_instance.main: password (name looks secret)
tls_private_key.deploy: private_key_pem (name looks secret)
aws_iam_access_key.ci: id (AWS access key ID)
aws_iam_access_key.ci: secret (name looks secret)
github_actions_secret.deploy: secret_name (name looks secret)
github_actions_secret.deploy: plaintext_value (name looks secret)
github_actions_secret.deploy: settings.api_token (name looks secret)
@@requires
def walk
isinstance(value, dict)
@@hint
`walk` yields `(path, reason)` pairs. If the value is a dict, loop over its items with `child = f"{path}.{key}" if path else key`; if the key matches SECRET_NAME and the value is a non-empty string, yield `(child, "name looks secret")`, otherwise recurse. For strings, yield "AWS access key ID" for AWS_KEY matches and "PEM private key" when `"PRIVATE KEY-----" in value`. (Note `secret_name` matches by name too; it's a label rather than a secret, which is why reviews still need a human.)
@@solution
import json
import re

state = json.loads('''{
  "version": 4,
  "resources": [
    {"mode": "managed", "type": "aws_db_instance", "name": "main", "instances": [
      {"attributes": {"identifier": "prod-db", "username": "admin", "password": "S3cr3t-pass", "port": 5432}}]},
    {"mode": "managed", "type": "tls_private_key", "name": "deploy", "instances": [
      {"attributes": {"algorithm": "ED25519", "private_key_pem": "-----BEGIN PRIVATE KEY-----\\nMC4CAQ...\\n-----END PRIVATE KEY-----"}}]},
    {"mode": "managed", "type": "aws_iam_access_key", "name": "ci", "instances": [
      {"attributes": {"id": "AKIAIOSFODNN7EXAMPLE", "user": "ci-bot", "secret": "wJalrXUtnFEMI/K7MDENG"}}]},
    {"mode": "managed", "type": "aws_s3_bucket", "name": "logs", "instances": [
      {"attributes": {"bucket": "acme-logs", "tags": {"team": "platform"}}}]},
    {"mode": "managed", "type": "github_actions_secret", "name": "deploy", "instances": [
      {"attributes": {"secret_name": "DEPLOY_TOKEN", "plaintext_value": "ghp_abc", "settings": {"api_token": "xyz"}}}]}
  ]
}''')

SECRET_NAME = re.compile(r"password|secret|token|private_key|plaintext", re.I)
AWS_KEY = re.compile(r"^AKIA[0-9A-Z]{16}$")

def walk(value, path):
    if isinstance(value, dict):
        for key, child_value in value.items():
            child = f"{path}.{key}" if path else key
            if SECRET_NAME.search(key) and isinstance(child_value, str) and child_value:
                yield child, "name looks secret"
            else:
                yield from walk(child_value, child)
    elif isinstance(value, list):
        for i, item in enumerate(value):
            yield from walk(item, f"{path}[{i}]")
    elif isinstance(value, str):
        if AWS_KEY.match(value):
            yield path, "AWS access key ID"
        elif "PRIVATE KEY-----" in value:
            yield path, "PEM private key"

for r in state["resources"]:
    address = f"{r['type']}.{r['name']}"
    for inst in r["instances"]:
        for path, reason in walk(inst["attributes"], ""):
            print(f"{address}: {path} ({reason})")

@@exercise Propagate sensitivity to outputs
Variables can be sensitive; locals and outputs are expressions that reference variables and other locals. Work out which locals and outputs are sensitive (anything that references a sensitive value, directly or through a chain, unless wrapped in `nonsensitive`), then report each output as OK, ERROR (sensitive but not marked), or "marked but not needed".
@@starter
variables = {"db_password": True, "api_key": True, "region": False, "env": False}
locals_ = {
    "conn": ["db_password", "region"],
    "name": ["env", "region"],
    "headers": ["api_key"],
    "pw_length": ["nonsensitive:db_password"],
}
outputs = {
    "connection_string": (["conn"], False),
    "bucket_name": (["name"], False),
    "api_headers": (["headers"], True),
    "password_length": (["pw_length"], False),
    "region": (["region"], True),
}

def is_sensitive(ref, seen=None):
    raise NotImplementedError

for name, (refs, marked) in outputs.items():
    sensitive = any(is_sensitive(r) for r in refs)
    if sensitive and not marked:
        status = "ERROR: output refers to sensitive values; add sensitive = true"
    elif marked and not sensitive:
        status = "marked sensitive but not needed"
    else:
        status = "OK" + (" (sensitive)" if sensitive else "")
    print(f"{name:<18} {status}")
@@expected
connection_string  ERROR: output refers to sensitive values; add sensitive = true
bucket_name        OK
api_headers        OK (sensitive)
password_length    OK
region             marked sensitive but not needed
@@requires
def is_sensitive
startswith("nonsensitive:")
@@hint
A reference starting with "nonsensitive:" is never sensitive. A variable is sensitive if `variables[ref]` is true. A local is sensitive if any of its own references is. Keep a `seen` set to avoid infinite loops on cycles.
@@solution
variables = {"db_password": True, "api_key": True, "region": False, "env": False}
locals_ = {
    "conn": ["db_password", "region"],
    "name": ["env", "region"],
    "headers": ["api_key"],
    "pw_length": ["nonsensitive:db_password"],
}
outputs = {
    "connection_string": (["conn"], False),
    "bucket_name": (["name"], False),
    "api_headers": (["headers"], True),
    "password_length": (["pw_length"], False),
    "region": (["region"], True),
}

def is_sensitive(ref, seen=None):
    seen = seen or set()
    if ref.startswith("nonsensitive:") or ref in seen:
        return False
    seen.add(ref)
    if ref in variables:
        return variables[ref]
    return any(is_sensitive(r, seen) for r in locals_.get(ref, []))

for name, (refs, marked) in outputs.items():
    sensitive = any(is_sensitive(r) for r in refs)
    if sensitive and not marked:
        status = "ERROR: output refers to sensitive values; add sensitive = true"
    elif marked and not sensitive:
        status = "marked sensitive but not needed"
    else:
        status = "OK" + (" (sensitive)" if sensitive else "")
    print(f"{name:<18} {status}")
