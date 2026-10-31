@@lesson xp-iam_policy_lint IAM Policy Linter
@@text
Least privilege is the most tested idea in AWS security. Write a linter for IAM policy JSON: flag `"Action": "*"`, `"Resource": "*"` on write actions, `iam:PassRole` without a resource restriction, `NotAction` with Allow, and missing conditions on sensitive actions, then suggest a tighter policy.

**You'll practise:** IAM policy grammar · least privilege · JSON · severity rules

## Milestones

1. **Parse**: handle Statement as an object or a list, Action as a string or a list.
2. **Rules**: the five checks above, each with a severity and an explanation.
3. **Wildcards**: `s3:*` and `s3:Get*` count as broad too.
4. **Suggest**: for each finding, what the policy should say instead.
5. **Try it**: lint the AWS managed policies you use, exported with the CLI.

## You're done when

- Every planted problem in the sample policy is found and explained.

## Getting help

Open the project, then the 🤖 **AI Helper**: it sees the lesson and your files. Good questions to start with:

- *"Why is iam:PassRole on Resource * dangerous?"*
- *"How would I write this policy so the Lambda can only write to one table?"*

Ask for hints, reviews and explanations rather than finished code: you learn by writing it.
@@project iam_policy_lint IAM Policy Linter
Create the folder, open it in VS Code, and work through `README.md` one milestone at a time.
@@file README.md
# IAM Policy Linter

Least privilege is the most tested idea in AWS security. Write a linter for IAM policy JSON: flag `"Action": "*"`, `"Resource": "*"` on write actions, `iam:PassRole` without a resource restriction, `NotAction` with Allow, and missing conditions on sensitive actions, then suggest a tighter policy.

## Milestones

- [ ] 1. Parse: handle Statement as an object or a list, Action as a string or a list.
- [ ] 2. Rules: the five checks above, each with a severity and an explanation.
- [ ] 3. Wildcards: `s3:*` and `s3:Get*` count as broad too.
- [ ] 4. Suggest: for each finding, what the policy should say instead.
- [ ] 5. Try it: lint the AWS managed policies you use, exported with the CLI.

## Done when

- [ ] Every planted problem in the sample policy is found and explained.
@@file lint.py
"""Lint an IAM policy: python lint.py policy.json"""
import json
import sys


def as_list(x):
    return x if isinstance(x, list) else [x]


policy = json.load(open(sys.argv[1] if len(sys.argv) > 1 else "policy.json", encoding="utf-8"))
for i, st in enumerate(as_list(policy["Statement"])):
    actions = as_list(st.get("Action", []))
    resources = as_list(st.get("Resource", []))
    if "*" in actions:
        print(f"statement {i}: HIGH Action is *")
    # TODO: Resource * on write actions, PassRole, NotAction with Allow, missing conditions
@@file policy.json
{
  "Version": "2012-10-17",
  "Statement": [
    { "Effect": "Allow", "Action": "s3:*", "Resource": "*" },
    { "Effect": "Allow", "Action": ["iam:PassRole", "lambda:CreateFunction"], "Resource": "*" },
    { "Effect": "Allow", "Action": "dynamodb:PutItem", "Resource": "arn:aws:dynamodb:ap-northeast-1:123456789012:table/requests" }
  ]
}
