@@lesson xp-tag_checker Tag Policy Checker for .tf Files
@@text
Before a plan even runs, you can catch problems in the code. Scan all `.tf` files in a folder, find every `resource` block, and report the ones that have no `tags` (or are missing required keys like Owner and CostCenter), skipping resource types that don't support tags. A simple version of what tflint and Checkov do.

**You'll practise:** HCL block structure · regular expressions and brace matching · tagging policies · CI checks

## Milestones

1. **Find**: each `resource "TYPE" "NAME" {` and its body (count braces).
2. **Tags**: look for `tags = {` or `tags = merge(` in the body.
3. **Required keys**: Owner and CostCenter, unless tags come from `default_tags` in the provider.
4. **Skip list**: types that don't take tags (aws_iam_role_policy_attachment, aws_route...).
5. **CI**: exit 1 when anything is missing, and add it to a pipeline.

## You're done when

- The checker finds the two untagged resources in the sample and nothing else.

## Getting help

Open the project, then the 🤖 **AI Helper**: it sees the lesson and your files. Good questions to start with:

- *"How do provider default_tags in AWS affect this check?"*
- *"Why is a regex-based HCL parser fragile, and what would be better?"*

Ask for hints, reviews and explanations rather than finished code: you learn by writing it.
@@project tag_checker Tag Policy Checker for .tf Files
Create the folder, open it in VS Code, and work through `README.md` one milestone at a time.
@@file README.md
# Tag Policy Checker for .tf Files

Before a plan even runs, you can catch problems in the code. Scan all `.tf` files in a folder, find every `resource` block, and report the ones that have no `tags` (or are missing required keys like Owner and CostCenter), skipping resource types that don't support tags. A simple version of what tflint and Checkov do.

## Milestones

- [ ] 1. Find: each `resource "TYPE" "NAME" {` and its body (count braces).
- [ ] 2. Tags: look for `tags = {` or `tags = merge(` in the body.
- [ ] 3. Required keys: Owner and CostCenter, unless tags come from `default_tags` in the provider.
- [ ] 4. Skip list: types that don't take tags (aws_iam_role_policy_attachment, aws_route...).
- [ ] 5. CI: exit 1 when anything is missing, and add it to a pipeline.

## Done when

- [ ] The checker finds the two untagged resources in the sample and nothing else.
@@file check_tags.py
"""Find resources without required tags: python check_tags.py FOLDER"""
import re
import sys
from pathlib import Path

REQUIRED = {"Owner", "CostCenter"}
START = re.compile(r'resource\s+"(\w+)"\s+"(\w+)"\s*\{')


def blocks(text: str):
    """(type, name, body) for each resource block, by counting braces."""
    for m in START.finditer(text):
        depth, i = 1, m.end()
        while depth and i < len(text):
            depth += {"{": 1, "}": -1}.get(text[i], 0)
            i += 1
        yield m.group(1), m.group(2), text[m.end():i - 1]


folder = Path(sys.argv[1] if len(sys.argv) > 1 else ".")
for tf in folder.glob("*.tf"):
    for rtype, name, body in blocks(tf.read_text(encoding="utf-8")):
        print(rtype, name, "has tags" if "tags" in body else "NO TAGS")
# TODO: required keys, skip list, exit code
@@file main.tf
resource "aws_s3_bucket" "logs" {
  bucket = "acme-logs"
  tags = {
    Owner      = "ops"
    CostCenter = "4100"
  }
}

resource "aws_instance" "web" {
  ami           = "ami-123"
  instance_type = "t3.micro"
}

resource "aws_sqs_queue" "jobs" {
  name = "jobs"
  tags = { Owner = "dev" }
}
