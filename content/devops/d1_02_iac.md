@@lesson d1-iac Infrastructure as Code: Terraform and Ansible
@@text
Clicking through a console doesn't scale and can't be reviewed. **Infrastructure as code (IaC)** describes servers, networks and cloud resources in files: versioned in Git, reviewed in pull requests, applied by pipelines, and repeatable in every environment.

## Declarative vs imperative, provisioning vs configuration

| | Provisioning (create infrastructure) | Configuration management (set up what's inside) |
|---|---|---|
| Tools | **Terraform**/OpenTofu, CloudFormation, Bicep, Pulumi | **Ansible**, Puppet, Chef, Salt, DSC |
| Example | a VPC, subnets, VMs, a database, DNS records | packages, users, config files, services on the VMs |

**Declarative** tools (Terraform, Ansible modules, Kubernetes YAML) describe the *end state*; the tool works out the steps. **Idempotent**: running it twice changes nothing the second time. Imperative scripts describe steps, and must handle "already exists" themselves.

## Terraform

```hcl
terraform {
  required_providers {
    aws = { source = "hashicorp/aws", version = "~> 5.0" }
  }
  backend "s3" {                            # shared remote state, with locking
    bucket = "acme-tfstate"
    key    = "network/terraform.tfstate"
    region = "ap-northeast-1"
  }
}

provider "aws" { region = var.region }

variable "region" { default = "ap-northeast-1" }
variable "env"    { type = string }

resource "aws_vpc" "main" {
  cidr_block = "10.30.0.0/16"
  tags       = { Name = "vpc-${var.env}", env = var.env }
}

resource "aws_subnet" "app" {
  count             = 2
  vpc_id            = aws_vpc.main.id        # a reference: Terraform orders creation itself
  cidr_block        = cidrsubnet(aws_vpc.main.cidr_block, 8, count.index)
  availability_zone = ["ap-northeast-1a", "ap-northeast-1c"][count.index]
}

output "vpc_id" { value = aws_vpc.main.id }
```

```bash,norun
terraform init        # download providers, set up the backend
terraform fmt ; terraform validate
terraform plan -var env=dev     # show what WOULD change: review this every time
terraform apply -var env=dev
terraform destroy -var env=dev
```

- **State** (`terraform.tfstate`) maps your code to real resource IDs. Keep it in a **remote backend** with **locking** (S3 + DynamoDB lock / S3 native locking, Azure Storage, Terraform Cloud), never in Git, since it can contain secrets.
- **Drift**: someone changed a resource by hand; `plan` shows it, and `apply` puts it back.
- **Modules** package reusable pieces (a "standard VPC"); **workspaces** or separate folders per environment.
- **OpenTofu** is the open-source fork of Terraform with the same language.

## Ansible

**Agentless**: it connects over **SSH** (WinRM for Windows) and runs **modules** from **playbooks** in YAML against an **inventory** of hosts.

```yaml
# site.yml   ansible-playbook -i inventory.ini site.yml --check   (dry run), then without --check
- name: Web servers
  hosts: web
  become: true                      # sudo
  tasks:
    - name: Install nginx
      ansible.builtin.apt: { name: nginx, state: present, update_cache: true }
    - name: Deploy the site config
      ansible.builtin.template: { src: site.conf.j2, dest: /etc/nginx/sites-available/default }
      notify: Reload nginx
    - name: Nginx running and enabled
      ansible.builtin.service: { name: nginx, state: started, enabled: true }
  handlers:
    - name: Reload nginx
      ansible.builtin.service: { name: nginx, state: reloaded }
```

**Roles** organize tasks, templates and defaults for reuse; **Ansible Vault** encrypts secrets in the repository; **Ansible Galaxy** shares roles and collections. Ansible is also popular for **network automation** (Cisco IOS, Juniper modules).

## A common pairing

Terraform creates the VMs and network, Ansible configures them; or, more cloud-native, Terraform creates a Kubernetes cluster and the apps are deployed with Helm/GitOps. Golden images built with **Packer** reduce what has to be configured at boot.

@@quiz
What does `terraform plan` do?
* Shows what would be created, changed or destroyed, without changing anything
- Applies the changes
- Deletes the state
- Downloads the providers only
? Always read the plan, especially lines that destroy resources.

@@quiz
Where should Terraform state for a team be stored?
* In a remote backend with locking (e.g. S3, Azure Storage, Terraform Cloud)
- Committed to Git
- On one person's laptop
- In a Slack channel
? State can contain secrets, and locking stops two applies at once.

@@quiz
Someone changed a security group in the console. Terraform's plan now shows a change. What is this called?
* Drift
- Idempotency
- A module
- A provider
? IaC works best when all changes go through code.

@@quiz
What does it mean that Ansible is agentless?
* It connects over SSH/WinRM; nothing has to be installed on the managed hosts beforehand
- It has no playbooks
- It only runs locally
- It needs no inventory
? Puppet and Chef traditionally use agents.

@@quiz
Running the same Ansible playbook twice in a row: what should happen the second time?
* Nothing changes; every task reports "ok" (idempotency)
- Everything is reinstalled
- It fails because things exist
- The hosts reboot
? Well-written tasks describe a state, not an action.

@@quiz
Which is a configuration-management tool rather than a provisioning tool?
* Ansible
- Terraform
- CloudFormation
- Bicep
? Ansible can create cloud resources too, but it's mainly used to configure hosts.

@@flashcards
IaC :: Infrastructure described in versioned files
declarative :: Describe the end state; the tool finds the steps
idempotent :: Same result however many times it runs
Terraform :: Declarative multi-cloud provisioning (HCL)
OpenTofu :: Open-source fork of Terraform
provider :: Terraform plugin for a platform (aws, azurerm)
terraform init / plan / apply / destroy :: Set up / preview / change / remove
state file :: Maps code to real resources; keep remote + locked
drift :: Real infrastructure differs from code
module :: Reusable Terraform component
Ansible :: Agentless configuration management over SSH
playbook :: Ansible YAML file of plays and tasks
inventory :: List of hosts and groups
handler :: Task run only when notified by a change
--check :: Ansible dry run
Ansible Vault :: Encrypted secrets for Ansible
Packer :: Builds golden machine images

@@drill
Which Terraform command previews changes without applying them? (terraform ...)
= terraform plan
? Then terraform apply.
