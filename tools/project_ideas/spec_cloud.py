from gen_projects import P

PROJECTS = [
    P("aws", "Intermediate", "cost_report", "AWS Cost and Tagging Report", "Find where the money goes, and which resources nobody owns.",
      """Download a Cost and Usage export (or use the sample costs.csv) and build the report a FinOps team wants every Monday: cost per service and per `Project` tag, the month-on-month change, the share of spend with no owner tag, and the five biggest single resources.""",
      ["AWS billing and Cost Explorer concepts", "tagging strategy", "csv and grouping", "percent change"],
      ["Load: read costs.csv (date, service, resource_id, tags, cost).",
       "Group: totals per service and per Project tag for each month.",
       "Change: month-on-month change per service, flag anything up more than 20%.",
       "Untagged: the share of cost with no Project or Owner tag.",
       "Act: write three concrete savings ideas based on what you found (Savings Plans, rightsizing, deleting idle resources)."],
      ["The report's totals match the CSV and the untagged share is correct."],
      ["Which AWS services usually hide surprise costs?",
       "What's a good tagging policy for a small company?"],
      {"cost_report.py": '''"""AWS cost report from a CSV export: python cost_report.py costs.csv"""
import csv
import sys
from collections import defaultdict


def load(path: str) -> list[dict]:
    rows = list(csv.DictReader(open(path, encoding="utf-8")))
    for r in rows:
        r["cost"] = float(r["cost"])
        r["month"] = r["date"][:7]
        r["tags"] = dict(t.split("=", 1) for t in r["tags"].split(";") if "=" in t)
    return rows


def by(rows: list[dict], key) -> dict:
    totals = defaultdict(float)
    for r in rows:
        totals[key(r)] += r["cost"]
    return dict(totals)


if __name__ == "__main__":
    rows = load(sys.argv[1] if len(sys.argv) > 1 else "costs.csv")
    for service, cost in sorted(by(rows, lambda r: r["service"]).items(), key=lambda x: -x[1]):
        print(f"{service:<28} ${cost:>10,.2f}")
    # TODO: per tag, month-on-month change, untagged share, top resources
''', "costs.csv": '''date,service,resource_id,tags,cost
2026-09-03,Amazon EC2,i-0a1,Project=shop;Owner=ann,412.20
2026-09-03,Amazon RDS,db-shop,Project=shop;Owner=ann,301.50
2026-09-04,Amazon S3,acme-logs,,88.10
2026-09-10,AWS Lambda,fn-thumbs,Project=media,12.40
2026-09-15,Amazon EC2,i-0b7,,233.00
2026-10-03,Amazon EC2,i-0a1,Project=shop;Owner=ann,455.90
2026-10-03,Amazon RDS,db-shop,Project=shop;Owner=ann,301.50
2026-10-04,Amazon S3,acme-logs,,141.70
2026-10-09,Amazon CloudWatch,logs,Project=shop,64.30
2026-10-15,Amazon EC2,i-0b7,,233.00
2026-10-20,NAT Gateway,nat-1,Project=shop,97.80'''}),
    P("azure", "Intermediate", "naming_audit", "Azure Naming and Tagging Audit", "Check resources against the Cloud Adoption Framework naming convention.",
      """Export your resources with `az resource list -o json > resources.json` (or use the sample) and check every name against a naming convention like `rg-shop-prod-jpe-001` (type, workload, environment, region, instance) and every resource for required tags (Environment, Owner, CostCenter). Produce a list of fixes.""",
      ["Azure resource types", "Cloud Adoption Framework naming", "tags and Azure Policy", "PowerShell JSON handling"],
      ["Prefixes: map resource types to CAF abbreviations (rg, vnet, vm, st, kv, app...).",
       "Parse: split names into parts and check each against allowed values.",
       "Tags: report missing required tags per resource.",
       "Report: a table and a CSV of resources to fix.",
       "Prevent: write the Azure Policy definition (JSON) that would deny untagged resources."],
      ["Every sample resource is classified correctly, and you've written the matching Azure Policy."],
      ["What are the CAF abbreviations for storage accounts and key vaults, and why do storage names have no dashes?",
       "Explain the difference between Audit and Deny effects in Azure Policy."],
      {"Test-AzureNaming.ps1": '''param([string]$Path = "resources.json")

$prefixes = @{
    "Microsoft.Resources/resourceGroups"   = "rg"
    "Microsoft.Network/virtualNetworks"    = "vnet"
    "Microsoft.Compute/virtualMachines"    = "vm"
    "Microsoft.KeyVault/vaults"            = "kv"
    "Microsoft.Web/sites"                  = "app"
}
$requiredTags = "Environment", "Owner", "CostCenter"

$resources = Get-Content $Path -Raw | ConvertFrom-Json
foreach ($r in $resources) {
    # TODO: check the name pattern  <prefix>-<workload>-<env>-<region>-<nnn>
    # TODO: check required tags
    Write-Output "$($r.type)  $($r.name)"
}
''', "resources.json": '''[
  { "name": "rg-shop-prod-jpe-001", "type": "Microsoft.Resources/resourceGroups", "tags": { "Environment": "prod", "Owner": "ann", "CostCenter": "4100" } },
  { "name": "vnet-shop-prod-jpe-001", "type": "Microsoft.Network/virtualNetworks", "tags": { "Environment": "prod", "Owner": "ann" } },
  { "name": "myVM1", "type": "Microsoft.Compute/virtualMachines", "tags": {} },
  { "name": "kv-shop-dev-jpe-001", "type": "Microsoft.KeyVault/vaults", "tags": { "Environment": "dev", "Owner": "ken", "CostCenter": "4100" } },
  { "name": "app-shop-prd-jpw-1", "type": "Microsoft.Web/sites", "tags": { "Environment": "prod", "Owner": "ann", "CostCenter": "4100" } }
]'''}),
    P("cloud", "Intermediate", "containerize", "Containerize a Web App", "Package a Python web app with Docker and docker compose, the right way.",
      """Take a small Python web app (included), write a production-quality Dockerfile (slim base image, non-root user, layer caching, a health check), then a compose file that adds a Redis cache, and measure how much smaller and faster your image gets with each improvement.""",
      ["Dockerfile instructions", "image layers and caching", "non-root containers", "docker compose", "health checks"],
      ["Run it locally: `python app.py` and open http://localhost:8000.",
       "Dockerfile v1: the simplest one that works; note the image size.",
       "Dockerfile v2: slim base, requirements first for caching, non-root USER, HEALTHCHECK.",
       "Compose: add Redis and count page visits in it.",
       "Compare: a table of image size and rebuild time for v1 and v2 in NOTES.md."],
      ["`docker compose up` runs the app with Redis, as a non-root user, and the health check passes."],
      ["Why does copying requirements.txt before the code speed up rebuilds?",
       "Review my Dockerfile for security problems."],
      {"app.py": '''"""A tiny web app using only the standard library."""
from http.server import BaseHTTPRequestHandler, HTTPServer


class Handler(BaseHTTPRequestHandler):
    def do_GET(self):
        body = b"ok\\n" if self.path == "/health" else b"Hello from a container!\\n"
        self.send_response(200)
        self.send_header("Content-Type", "text/plain")
        self.end_headers()
        self.wfile.write(body)


if __name__ == "__main__":
    HTTPServer(("0.0.0.0", 8000), Handler).serve_forever()
''', "Dockerfile": '''# v1: works, but big, slow to rebuild, and runs as root. Improve it.
FROM python:3.12
COPY . /app
WORKDIR /app
CMD ["python", "app.py"]
''', "compose.yaml": '''services:
  web:
    build: .
    ports: ["8000:8000"]
  # TODO: redis
'''}),
    P("devops", "Advanced", "ci_pipeline", "CI/CD Pipeline for a Project", "Lint, test, build and release a project automatically with GitHub Actions.",
      """Take one of your projects from this app and give it a professional pipeline: on every push, lint and test it on Windows and Linux; on a tag, build it and publish a release. Add a status badge to the README and branch protection so broken code can't be merged.""",
      ["GitHub Actions workflows", "matrix builds", "caching", "artifacts and releases", "branch protection"],
      ["Pick a project and push it to a new GitHub repository.",
       "Workflow: `on: [push, pull_request]`, a job that installs, lints and runs the tests.",
       "Matrix: run on ubuntu-latest and windows-latest.",
       "Release: on tags `v*`, build and upload the artifact to a GitHub Release.",
       "Protect: require the checks to pass before merging into main."],
      ["A pull request with a failing test is blocked, and pushing a tag creates a release."],
      ["Review my workflow YAML for mistakes.",
       "How do I cache pip or npm downloads in GitHub Actions?"],
      {".github/workflows/ci.yml": '''name: ci
on: [push, pull_request]

jobs:
  test:
    strategy:
      matrix:
        os: [ubuntu-latest, windows-latest]
    runs-on: ${{ matrix.os }}
    steps:
      - uses: actions/checkout@v4
      - uses: actions/setup-python@v5
        with:
          python-version: "3.12"
      - run: python -m unittest -v
      # TODO: lint step, release job on tags
''', "test_example.py": '''import unittest


class Example(unittest.TestCase):
    def test_truth(self):
        self.assertEqual(1 + 1, 2)


if __name__ == "__main__":
    unittest.main()
'''}),
    P("devops", "Advanced", "slo_budget", "SLO and Error Budget Tracker", "Turn request logs into SLIs, SLO compliance and burn-rate alerts.",
      """SREs run on numbers: what share of requests succeeded and were fast enough (the SLIs), whether that meets the SLO, how much error budget is left this month, and how fast it's burning. Compute all of it from an access log, including the multi-window burn-rate alert from Google's SRE workbook.""",
      ["SLI, SLO, error budget", "burn rate", "parsing logs", "time windows"],
      ["Parse: read requests.log (time, status, latency_ms).",
       "SLIs: availability (status < 500) and latency (under 300 ms) per hour.",
       "Budget: for a 99.5% SLO over 30 days, budget used and left.",
       "Burn rate: errors in the last 1 h and 6 h divided by the allowed rate; alert at 14.4× and 6×.",
       "Report: a short text report a manager understands."],
      ["The numbers are right on a log you made by hand, and an artificial outage triggers the alert."],
      ["Explain burn rate and why multi-window alerts reduce false alarms.",
       "Is 99.9% always better than 99.5%? What does it cost?"],
      {"slo.py": '''"""SLO report: python slo.py requests.log"""
import sys

SLO = 0.995
WINDOW_DAYS = 30


def parse(path: str) -> list[tuple[str, int, float]]:
    """(iso time, status, latency ms) per request; lines look like: 2026-10-30T10:00:01 200 123.4"""
    out = []
    for line in open(path, encoding="utf-8"):
        t, status, ms = line.split()
        out.append((t, int(status), float(ms)))
    return out


def availability(reqs) -> float:
    # TODO
    return 1.0


if __name__ == "__main__":
    reqs = parse(sys.argv[1] if len(sys.argv) > 1 else "requests.log")
    print(f"availability {availability(reqs):.4%} (SLO {SLO:.1%})")
    # TODO: error budget left, burn rates, latency SLI
''', "requests.log": '''2026-10-30T10:00:01 200 120.5
2026-10-30T10:00:02 200 98.0
2026-10-30T10:00:03 500 15.2
2026-10-30T10:00:04 200 410.9
2026-10-30T10:00:05 404 30.1
2026-10-30T10:00:06 200 101.7'''}),
    P("gcp", "Advanced", "cloud_run_service", "Deploy a Service to Cloud Run", "Build, deploy and secure a container on Cloud Run, with a budget alert.",
      """Ship a real service on Google Cloud: containerize a small API, push it to Artifact Registry, deploy it to Cloud Run in asia-northeast1, give it its own service account with only the roles it needs, store a secret in Secret Manager, and set a budget alert so a mistake can't cost you much. Then delete it all.""",
      ["Cloud Run", "Artifact Registry", "service accounts and least privilege", "Secret Manager", "budgets"],
      ["Project: create a project, link billing, set a ¥1,000 budget alert first.",
       "Build: `gcloud builds submit` (or docker push) into Artifact Registry.",
       "Deploy: `gcloud run deploy` with `--service-account` and `--region asia-northeast1`.",
       "Secret: create a secret and mount it as an environment variable.",
       "Clean up: delete the service, the image and the project; note what each step cost."],
      ["The service answers on its HTTPS URL, runs as its own service account, and everything is deleted afterwards."],
      ["Which IAM roles does my Cloud Run service account actually need?",
       "Explain what happens when Cloud Run scales to zero."],
      {"LAB.md": '''# Cloud Run lab

```bash
gcloud config set project YOUR_PROJECT
gcloud services enable run.googleapis.com artifactregistry.googleapis.com secretmanager.googleapis.com
gcloud artifacts repositories create apps --repository-format=docker --location=asia-northeast1
gcloud iam service-accounts create hello-run
# TODO: build, deploy with --service-account, add a secret
```

## Clean up
```bash
gcloud run services delete hello --region asia-northeast1
```
''', "main.py": '''import os
from http.server import BaseHTTPRequestHandler, HTTPServer


class Handler(BaseHTTPRequestHandler):
    def do_GET(self):
        greeting = os.environ.get("GREETING", "Hello")
        self.send_response(200)
        self.end_headers()
        self.wfile.write(f"{greeting} from Cloud Run\\n".encode())


HTTPServer(("0.0.0.0", int(os.environ.get("PORT", "8080"))), Handler).serve_forever()
''', "Dockerfile": '''FROM python:3.12-slim
WORKDIR /app
COPY main.py .
USER 1000
CMD ["python", "main.py"]
'''}),
    P("terraform", "Advanced", "static_site_module", "Reusable Terraform Module", "Write, document and test a module others can use.",
      """Write a Terraform module for a static website bucket (on AWS S3 or Google Cloud Storage): versioning, encryption, public access only if asked for, lifecycle rules and tags, with typed and validated variables, outputs, an example that uses it, and `terraform test` files that check it without guessing.""",
      ["module structure", "variables with validation", "outputs", "terraform test", "documentation"],
      ["Layout: modules/site/{main.tf,variables.tf,outputs.tf,README.md} and examples/basic.",
       "Variables: name, environment (validated: dev/staging/prod), tags, `public` (default false).",
       "Resources: bucket, versioning, encryption, lifecycle (delete old versions after 30 days).",
       "Tests: a `tests/site.tftest.hcl` with `command = plan` runs and asserts.",
       "Docs: a README with inputs, outputs and an example."],
      ["`terraform validate`, `terraform test` and a plan of the example all succeed."],
      ["How should I structure variables so the module is hard to misuse?",
       "Show me how terraform test assertions work with command = plan."],
      {"modules/site/variables.tf": '''variable "name" {
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
''', "modules/site/main.tf": '''# TODO: the bucket, versioning, encryption, public access block, lifecycle rule
''', "modules/site/outputs.tf": '''# TODO: bucket name, ARN or URL
'''}),
    P("cka", "Advanced", "three_tier_kind", "Three-tier App on a Local Cluster", "Run a web app, an API and a database on kind with everything the CKA asks for.",
      """On a local kind (or minikube) cluster, deploy a frontend, an API and PostgreSQL: Deployments with probes and resources, a StatefulSet with a PersistentVolumeClaim for the database, Services, an Ingress, a ConfigMap and a Secret, NetworkPolicies so only the API can reach the database, and RBAC for a deployer account. Then break things on purpose and fix them.""",
      ["Deployments and StatefulSets", "PVCs", "Services and Ingress", "NetworkPolicy", "RBAC", "troubleshooting"],
      ["Cluster: `kind create cluster --config kind.yaml` with an ingress-ready node.",
       "Database: StatefulSet + headless Service + PVC + Secret for the password.",
       "API and frontend: Deployments with probes, requests and limits, Services, an Ingress.",
       "Lock down: NetworkPolicies (default deny, then allow) and a deployer ServiceAccount with a Role.",
       "Break and fix: wrong image tag, wrong selector, failing probe; write down how you found each."],
      ["The app works through the Ingress, the DB survives pod deletion, and your break/fix notes cover three faults."],
      ["My NetworkPolicy blocks DNS too. Why, and how do I allow it?",
       "Check my StatefulSet: will the data survive if the pod is deleted?"],
      {"kind.yaml": '''kind: Cluster
apiVersion: kind.x-k8s.io/v1alpha4
nodes:
  - role: control-plane
    extraPortMappings:
      - containerPort: 80
        hostPort: 8080
  - role: worker
''', "db.yaml": '''apiVersion: v1
kind: Secret
metadata: { name: db, namespace: shop }
stringData: { password: change-me }
---
# TODO: headless Service and StatefulSet for postgres:16 with a volumeClaimTemplate
''', "NOTES.md": '''# Break and fix notes

| Fault | Symptom | How I found it | Fix |
|---|---|---|---|
'''}),
]
