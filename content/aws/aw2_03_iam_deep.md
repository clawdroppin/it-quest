@@lesson aw2-iam-deep IAM in Depth: Roles, Policies, Organizations and Federation
@@text
Identity is the first line of defence in AWS, and the first domain of the Solutions Architect exam. This lesson goes beyond users and groups: roles and temporary credentials, the full policy evaluation logic (SCPs, permission boundaries, resource policies, session policies), multi-account governance with Organizations and Control Tower, and federation with IAM Identity Center.

## Principals and credentials

- **Root user**: full access to the account; lock it away (MFA, no access keys) and use it only for the few tasks that require it.
- **IAM users**: long-term credentials. Prefer federation for people and roles for workloads; if users exist, enforce MFA and rotate keys.
- **IAM roles**: assumed through **STS** to get **temporary credentials** (15 min – 12 h). Used by EC2 (instance profiles), Lambda (execution roles), ECS tasks, cross-account access, and federated users.
- **Trust policy** (who may assume the role) vs **permissions policy** (what the role may do).

```json,norun
{
  "Version": "2012-10-17",
  "Statement": [{
    "Effect": "Allow",
    "Principal": {"AWS": "arn:aws:iam::111122223333:root"},
    "Action": "sts:AssumeRole",
    "Condition": {"Bool": {"aws:MultiFactorAuthPresent": "true"}, "StringEquals": {"sts:ExternalId": "partner-7f3a"}}
  }]
}
```

The **external ID** protects against the **confused deputy** problem when a third party assumes a role in your account.

## Policy types

| Type | Attached to | Effect |
|---|---|---|
| **Identity-based** (managed, inline) | users, groups, roles | grants permissions |
| **Resource-based** (S3 bucket policy, KMS key policy, SQS, Lambda) | resources | grants access, including **cross-account**, names a Principal |
| **Permissions boundary** | users, roles | **maximum** permissions an identity policy can grant (delegated admin) |
| **SCP** (Organizations) | OUs, accounts | **maximum** permissions for every principal in the account, root included (not the management account) |
| **RCP** (resource control policy) | OUs, accounts | maximum permissions on resources in the account |
| **Session policy** | an assumed-role session | further limits that session |

Boundaries, SCPs, RCPs and session policies **never grant**; they only cap.

## Evaluation logic (single account)

1. Everything starts **implicitly denied**.
2. Any **explicit Deny** anywhere → **Deny** (final).
3. The **SCP/RCP** chain must allow the action (otherwise implicit deny).
4. A **resource-based policy** allow can grant access (for a same-account principal, even without an identity allow).
5. Otherwise the **identity-based policy** must allow it, **and** the permissions boundary (if set) **and** the session policy (if any) must allow it too.

**Cross-account**: the caller's account must allow it (identity policy) **and** the resource's account must allow it (resource policy), or the caller assumes a role in the resource account.

## Policy elements and conditions

`Effect`, `Action` (`s3:GetObject`, wildcards `s3:Get*`), `Resource` (ARNs: `arn:aws:s3:::bucket/prefix/*`), `Principal` (resource policies), `Condition`:

- `aws:SourceIp`, `aws:SourceVpce`, `aws:SecureTransport`
- `aws:MultiFactorAuthPresent`, `aws:PrincipalOrgID` (restrict to your organisation), `aws:RequestedRegion`
- **ABAC**: `aws:PrincipalTag/project` = `${aws:ResourceTag/project}` scales permissions with tags instead of one policy per team.

**NotAction** / **NotResource** invert the match; use carefully.

## Multi-account governance

- **AWS Organizations**: management account, **OUs**, **consolidated billing** (volume discounts, shared RIs/Savings Plans), SCPs, tag policies, backup policies.
- **Control Tower**: sets up a **landing zone** (log archive and audit accounts, guardrails as SCPs and Config rules, Account Factory).
- Typical SCPs: deny leaving the organisation, deny disabling CloudTrail/GuardDuty, deny Regions outside an approved list, deny root user actions.

## Federation

- **IAM Identity Center** (successor to AWS SSO): one place for workforce access to many accounts and apps; connects to an external IdP (Entra ID, Okta, Google) via **SAML 2.0/SCIM**; users get **permission sets** per account.
- **SAML 2.0 federation** directly to IAM roles; **web identity / Cognito** for app end users (user pools for sign-in, identity pools for AWS credentials).

## Auditing permissions

**IAM Access Analyzer** (external access findings, unused access, policy validation and generation from CloudTrail activity), **credential report**, **last accessed** information, **CloudTrail** for every API call.

@@quiz
An EC2 application needs to read from S3. What is the recommended way to give it credentials?
- Store an access key in the application config
- Use the root user's keys
* Attach an IAM role through an instance profile
- Put the key in user data
? Roles provide automatically rotated temporary credentials.

@@quiz
An identity policy allows `s3:*`, but an SCP on the account allows only `ec2:*`. Can the user read S3 objects?
- Yes, the identity policy allows it
* No, the SCP sets the maximum permissions
- Only with MFA
- Only from the management account
? SCPs cap but never grant.

@@quiz
Which feature lets a team lead create roles for developers but never with more than a defined set of permissions?
- SCPs
* Permissions boundaries
- Session policies
- Resource-based policies
? Delegate administration safely.

@@quiz
A third-party monitoring vendor assumes a role in your account. What prevents the confused deputy problem?
- MFA
* An external ID condition in the trust policy
- A permissions boundary
- A resource-based policy
? The vendor passes a unique ID per customer.

@@quiz
A bucket policy should allow access only from accounts in your AWS Organization. Which condition key?
* aws:PrincipalOrgID
- aws:SourceIp
- aws:SourceVpce
- aws:RequestedRegion
? Avoids listing every account ID.

@@quiz
Which service gives employees single sign-on to many AWS accounts using the company IdP?
- Cognito user pools
* IAM Identity Center
- STS GetSessionToken
- AWS Directory Service only
? Permission sets per account.

@@flashcards
root user :: MFA, no access keys, rarely used
IAM role :: temporary credentials via STS
trust policy :: who can assume the role
instance profile :: role for EC2
external ID :: confused deputy protection
identity-based policy :: attached to principals
resource-based policy :: names a Principal, cross-account
permissions boundary :: max permissions for an identity
SCP :: max permissions for an account
explicit deny :: always wins
default :: implicit deny
cross-account :: both sides must allow
aws:PrincipalOrgID :: restrict to the organisation
ABAC :: permissions based on tags
Organizations :: OUs, consolidated billing, SCPs
Control Tower :: landing zone and guardrails
Identity Center :: workforce SSO, permission sets
Cognito :: app end-user identities
Access Analyzer :: external and unused access

@@drill
Which AWS service issues temporary credentials when a role is assumed?
= STS
= sts
= Security Token Service
? AssumeRole returns temporary keys.

@@drill
What always wins in IAM policy evaluation?
= explicit deny
= an explicit deny
= Explicit deny
? Then SCPs, then allows.

@@exercise Full policy evaluation with SCPs and boundaries
Evaluate each request with the single-account logic: explicit Deny in any policy → `DENY (explicit)`; the SCP must allow the action → otherwise `DENY (SCP)`; a resource-policy Allow → `ALLOW (resource policy)`; otherwise the identity policy must allow, and the boundary too if one is set → `ALLOW`, else `DENY (boundary)` or `DENY (implicit)`. Actions and resources support `*` wildcards (use `fnmatch`).
@@lang python
@@starter
from fnmatch import fnmatchcase

scp = [{"Effect": "Allow", "Action": ["s3:*", "ec2:Describe*", "dynamodb:*"], "Resource": ["*"]},
       {"Effect": "Deny", "Action": ["s3:DeleteBucket"], "Resource": ["*"]}]
identity = [{"Effect": "Allow", "Action": ["s3:*", "ec2:*"], "Resource": ["*"]},
            {"Effect": "Deny", "Action": ["s3:PutBucketPolicy"], "Resource": ["*"]}]
boundary = [{"Effect": "Allow", "Action": ["s3:Get*", "s3:List*", "ec2:Describe*"], "Resource": ["*"]}]
resource_policies = {"arn:aws:s3:::shared-data/report.csv": [{"Effect": "Allow", "Action": ["s3:PutObject"], "Resource": ["arn:aws:s3:::shared-data/*"]}]}
requests = [
    ("s3:GetObject", "arn:aws:s3:::app-data/a.json"),
    ("s3:PutObject", "arn:aws:s3:::app-data/a.json"),
    ("s3:PutObject", "arn:aws:s3:::shared-data/report.csv"),
    ("s3:DeleteBucket", "arn:aws:s3:::app-data"),
    ("s3:PutBucketPolicy", "arn:aws:s3:::app-data"),
    ("ec2:TerminateInstances", "arn:aws:ec2:eu-west-1:111122223333:instance/i-0abc"),
    ("ec2:DescribeInstances", "*"),
    ("dynamodb:GetItem", "arn:aws:dynamodb:eu-west-1:111122223333:table/orders"),
]

def matches(statements, effect, action, resource):
    raise NotImplementedError
@@expected
s3:GetObject             arn:aws:s3:::app-data/a.json: ALLOW
s3:PutObject             arn:aws:s3:::app-data/a.json: DENY (boundary)
s3:PutObject             arn:aws:s3:::shared-data/report.csv: ALLOW (resource policy)
s3:DeleteBucket          arn:aws:s3:::app-data: DENY (explicit)
s3:PutBucketPolicy       arn:aws:s3:::app-data: DENY (explicit)
ec2:TerminateInstances   arn:aws:ec2:eu-west-1:111122223333:instance/i-0abc: DENY (SCP)
ec2:DescribeInstances    *: ALLOW
dynamodb:GetItem         arn:aws:dynamodb:eu-west-1:111122223333:table/orders: DENY (implicit)
@@requires
def matches
fnmatchcase(
def evaluate
@@hint
`matches` returns True if any statement with that Effect has an Action pattern matching the action and a Resource pattern matching the resource. Collect all policies (SCP, identity, boundary, the resource policy) when checking for explicit denies.
@@solution
from fnmatch import fnmatchcase

scp = [{"Effect": "Allow", "Action": ["s3:*", "ec2:Describe*", "dynamodb:*"], "Resource": ["*"]},
       {"Effect": "Deny", "Action": ["s3:DeleteBucket"], "Resource": ["*"]}]
identity = [{"Effect": "Allow", "Action": ["s3:*", "ec2:*"], "Resource": ["*"]},
            {"Effect": "Deny", "Action": ["s3:PutBucketPolicy"], "Resource": ["*"]}]
boundary = [{"Effect": "Allow", "Action": ["s3:Get*", "s3:List*", "ec2:Describe*"], "Resource": ["*"]}]
resource_policies = {"arn:aws:s3:::shared-data/report.csv": [{"Effect": "Allow", "Action": ["s3:PutObject"], "Resource": ["arn:aws:s3:::shared-data/*"]}]}
requests = [
    ("s3:GetObject", "arn:aws:s3:::app-data/a.json"),
    ("s3:PutObject", "arn:aws:s3:::app-data/a.json"),
    ("s3:PutObject", "arn:aws:s3:::shared-data/report.csv"),
    ("s3:DeleteBucket", "arn:aws:s3:::app-data"),
    ("s3:PutBucketPolicy", "arn:aws:s3:::app-data"),
    ("ec2:TerminateInstances", "arn:aws:ec2:eu-west-1:111122223333:instance/i-0abc"),
    ("ec2:DescribeInstances", "*"),
    ("dynamodb:GetItem", "arn:aws:dynamodb:eu-west-1:111122223333:table/orders"),
]

def matches(statements, effect, action, resource):
    for s in statements:
        if s["Effect"] != effect:
            continue
        if any(fnmatchcase(action, a) for a in s["Action"]) and any(fnmatchcase(resource, r) for r in s["Resource"]):
            return True
    return False

def evaluate(action, resource):
    res_policy = resource_policies.get(resource, [])
    for policy in (scp, identity, boundary, res_policy):
        if matches(policy, "Deny", action, resource):
            return "DENY (explicit)"
    if not matches(scp, "Allow", action, resource):
        return "DENY (SCP)"
    if matches(res_policy, "Allow", action, resource):
        return "ALLOW (resource policy)"
    if not matches(identity, "Allow", action, resource):
        return "DENY (implicit)"
    if boundary and not matches(boundary, "Allow", action, resource):
        return "DENY (boundary)"
    return "ALLOW"

for action, resource in requests:
    print(f"{action:<24} {resource}: {evaluate(action, resource)}")

@@exercise Review IAM roles for risky trust and permissions
Review each role and print its findings: trust policy principal `*` (anyone); a third-party account (not in `OUR_ACCOUNTS`) without an `sts:ExternalId` condition; `Action` `*` with `Resource` `*` (full admin) on a role not named in `ADMIN_ROLES`; `iam:PassRole` on `Resource` `*`; unused for more than 90 days (`last_used_days`). Print `OK` when nothing is found.
@@lang python
@@starter
OUR_ACCOUNTS = {"111122223333", "444455556666"}
ADMIN_ROLES = {"BreakGlassAdmin"}
roles = [
    {"name": "BreakGlassAdmin", "trust": {"principal": "arn:aws:iam::111122223333:root", "conditions": {"aws:MultiFactorAuthPresent": "true"}},
     "statements": [{"Action": "*", "Resource": "*"}], "last_used_days": 200},
    {"name": "VendorMonitoring", "trust": {"principal": "arn:aws:iam::999988887777:root", "conditions": {}},
     "statements": [{"Action": "cloudwatch:Get*", "Resource": "*"}], "last_used_days": 1},
    {"name": "CiDeployer", "trust": {"principal": "arn:aws:iam::444455556666:role/ci", "conditions": {}},
     "statements": [{"Action": "*", "Resource": "*"}, {"Action": "iam:PassRole", "Resource": "*"}], "last_used_days": 3},
    {"name": "AppReader", "trust": {"principal": "ec2.amazonaws.com", "conditions": {}},
     "statements": [{"Action": "s3:GetObject", "Resource": "arn:aws:s3:::app-data/*"}], "last_used_days": 0},
    {"name": "OldPartner", "trust": {"principal": "*", "conditions": {}},
     "statements": [{"Action": "s3:*", "Resource": "*"}], "last_used_days": 400},
]
@@expected
BreakGlassAdmin: unused for 200 days
VendorMonitoring: third-party account 999988887777 without external ID
CiDeployer: full admin (*:*); iam:PassRole on *
AppReader: OK
OldPartner: trusts anyone (*); unused for 400 days
@@requires
def review
split(":")
@@hint
The account ID is the fifth field of an IAM ARN: `principal.split(":")[4]`. Service principals (ending in `.amazonaws.com`) have no account.
@@solution
OUR_ACCOUNTS = {"111122223333", "444455556666"}
ADMIN_ROLES = {"BreakGlassAdmin"}
roles = [
    {"name": "BreakGlassAdmin", "trust": {"principal": "arn:aws:iam::111122223333:root", "conditions": {"aws:MultiFactorAuthPresent": "true"}},
     "statements": [{"Action": "*", "Resource": "*"}], "last_used_days": 200},
    {"name": "VendorMonitoring", "trust": {"principal": "arn:aws:iam::999988887777:root", "conditions": {}},
     "statements": [{"Action": "cloudwatch:Get*", "Resource": "*"}], "last_used_days": 1},
    {"name": "CiDeployer", "trust": {"principal": "arn:aws:iam::444455556666:role/ci", "conditions": {}},
     "statements": [{"Action": "*", "Resource": "*"}, {"Action": "iam:PassRole", "Resource": "*"}], "last_used_days": 3},
    {"name": "AppReader", "trust": {"principal": "ec2.amazonaws.com", "conditions": {}},
     "statements": [{"Action": "s3:GetObject", "Resource": "arn:aws:s3:::app-data/*"}], "last_used_days": 0},
    {"name": "OldPartner", "trust": {"principal": "*", "conditions": {}},
     "statements": [{"Action": "s3:*", "Resource": "*"}], "last_used_days": 400},
]

def review(role):
    findings = []
    principal = role["trust"]["principal"]
    if principal == "*":
        findings.append("trusts anyone (*)")
    elif principal.startswith("arn:aws:iam::"):
        account = principal.split(":")[4]
        if account not in OUR_ACCOUNTS and "sts:ExternalId" not in role["trust"]["conditions"]:
            findings.append(f"third-party account {account} without external ID")
    for s in role["statements"]:
        if s["Action"] == "*" and s["Resource"] == "*" and role["name"] not in ADMIN_ROLES:
            findings.append("full admin (*:*)")
        if s["Action"] == "iam:PassRole" and s["Resource"] == "*":
            findings.append("iam:PassRole on *")
    if role["last_used_days"] > 90:
        findings.append(f"unused for {role['last_used_days']} days")
    return findings

for role in roles:
    findings = review(role)
    print(f"{role['name']}: " + ("OK" if not findings else "; ".join(findings)))
