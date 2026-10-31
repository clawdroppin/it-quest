@@lesson xp-static_site_module Reusable Terraform Module
@@text
Write a Terraform module for a static website bucket (on AWS S3 or Google Cloud Storage): versioning, encryption, public access only if asked for, lifecycle rules and tags, with typed and validated variables, outputs, an example that uses it, and `terraform test` files that check it without guessing.

**You'll practise:** module structure · variables with validation · outputs · terraform test · documentation

## Milestones

1. **Layout**: modules/site/{main.tf,variables.tf,outputs.tf,README.md} and examples/basic.
2. **Variables**: name, environment (validated: dev/staging/prod), tags, `public` (default false).
3. **Resources**: bucket, versioning, encryption, lifecycle (delete old versions after 30 days).
4. **Tests**: a `tests/site.tftest.hcl` with `command = plan` runs and asserts.
5. **Docs**: a README with inputs, outputs and an example.

## You're done when

- `terraform validate`, `terraform test` and a plan of the example all succeed.

## Getting help

Open the project, then the 🤖 **AI Helper**: it sees the lesson and your files. Good questions to start with:

- *"How should I structure variables so the module is hard to misuse?"*
- *"Show me how terraform test assertions work with command = plan."*

Ask for hints, reviews and explanations rather than finished code: you learn by writing it.
@@project static_site_module Reusable Terraform Module
Create the folder, open it in VS Code, and work through `README.md` one milestone at a time.
@@file README.md
# Reusable Terraform Module

Write a Terraform module for a static website bucket (on AWS S3 or Google Cloud Storage): versioning, encryption, public access only if asked for, lifecycle rules and tags, with typed and validated variables, outputs, an example that uses it, and `terraform test` files that check it without guessing.

## Milestones

- [ ] 1. Layout: modules/site/{main.tf,variables.tf,outputs.tf,README.md} and examples/basic.
- [ ] 2. Variables: name, environment (validated: dev/staging/prod), tags, `public` (default false).
- [ ] 3. Resources: bucket, versioning, encryption, lifecycle (delete old versions after 30 days).
- [ ] 4. Tests: a `tests/site.tftest.hcl` with `command = plan` runs and asserts.
- [ ] 5. Docs: a README with inputs, outputs and an example.

## Done when

- [ ] `terraform validate`, `terraform test` and a plan of the example all succeed.
@@file modules/site/variables.tf
variable "name" {
  type        = string
  description = "Bucket name prefix."
}

variable "environment" {
  type = string
  validation {
    condition     = contains(["dev", "staging", "prod"], var.environment)
    error_message = "environment must be dev, staging or prod."
  }
}

variable "public" {
  type    = bool
  default = false
}

variable "tags" {
  type    = map(string)
  default = {}
}
@@file modules/site/main.tf
# TODO: the bucket, versioning, encryption, public access block, lifecycle rule
@@file modules/site/outputs.tf
# TODO: bucket name, ARN or URL
