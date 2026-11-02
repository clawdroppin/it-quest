OUT = "content/devops/d9_02_ship_service.md"
BASE = "tools/appdev/devops_ship"
FILES = ["metrics.py", "service.py", "main.py", "test_metrics.py", "test_service.py",
         "Dockerfile", ".dockerignore", "compose.yaml", "Makefile",
         "monitoring/prometheus.yml", "monitoring/alerts.yml",
         "monitoring/grafana/datasources.yml", "monitoring/grafana/dashboards.yml",
         "monitoring/grafana/dashboards/ticket-api.json",
         "k8s/deployment.yaml", "k8s/service.yaml", "k8s/kustomization.yaml",
         ".github/workflows/ci.yml"]

PARTS = [
    ("text", r'''
@@lesson d9-ship-service Ship a Service End to End: Code, Container, Pipeline, Kubernetes and Monitoring
@@text
This capstone puts the whole DevOps toolchain around one small service, the way a real team would run it: a ticket API that **exposes its own Prometheus metrics**, a hardened **container image**, a **Compose** stack with **Prometheus**, alert rules and a provisioned **Grafana** dashboard, hardened **Kubernetes** manifests with Kustomize, and a **GitHub Actions** pipeline that tests, builds, scans, smoke-tests and pushes the image.

```text
 git push ──▶ GitHub Actions: unit tests ─▶ docker build ─▶ Trivy scan ─▶ smoke test ─▶ push to GHCR
                                                                                          │
 kubectl apply -k k8s/  (or Argo CD watching the repo) ◀──────────────── image tag ──────┘
        │
        ▼
 ┌──────────────┐   /metrics    ┌────────────┐   PromQL   ┌─────────┐
 │  ticket-api  │ ◀──────────── │ Prometheus │ ◀───────── │ Grafana │
 │  2 replicas  │    scrape     │  + alerts  │            │  (RED)  │
 └──────────────┘               └────────────┘            └─────────┘
```

## The service

| Request | Response |
|---|---|
| `GET /healthz` | `{"status": "ok"}`, for probes and health checks |
| `POST /tickets` with `{"title": ..., "priority": ...}` | 201 with the ticket; 422 with every problem; 400 for broken JSON |
| `GET /tickets?priority=urgent` | the matching tickets |
| `GET /metrics` | Prometheus text format |

It uses only Python's standard library, so there's nothing to install: `python main.py`, then `curl localhost:8080/metrics`.

## How it's built

| File | Job |
|---|---|
| `metrics.py` | counters, histograms and the Prometheus text format, written from scratch |
| `service.py` | the store, validation, routing and instrumentation (request counts and durations per route) |
| `main.py` | configuration from environment variables and graceful shutdown on SIGTERM |
| `test_*.py` | 12 unit and HTTP tests |
| `Dockerfile` / `.dockerignore` | a slim, non-root image with a healthcheck and exec-form entrypoint |
| `compose.yaml` | the API, Prometheus and Grafana, with security options on the API |
| `monitoring/` | scrape config, three alert rules (down, error rate, p95 latency), Grafana data source and a RED dashboard, all provisioned as code |
| `k8s/` | Deployment with probes, resource limits, a restricted security context and zero-downtime rollouts; Service; Kustomize |
| `.github/workflows/ci.yml` | test → build → scan → smoke test → push |
| `Makefile` | `make test`, `make up`, `make load` |

## Your tasks

Four functions in `metrics.py` raise `NotImplementedError`. Their docstrings and the tests in `test_metrics.py` are the specification:

1. `format_labels`: build `{route="/tickets",status="200"}` and escape `\`, `"` and newlines.
2. `Counter.render`: the `# HELP` and `# TYPE` lines, then one line per label combination, **sorted**.
3. `Histogram.observe`: buckets are **cumulative**: an observation of 0.3 counts in the `0.5`, `1`, ... and `+Inf` buckets. Also add to the sum and the count.
4. `Histogram.render`: a `_bucket` line per bucket with an extra `le` label, then `_sum` and `_count`.

```text
python -m unittest -v
```

When the tests pass, `python main.py` and look at http://localhost:8080/metrics after a few requests.

## Then run it like production

With Docker installed:

```text
make up            # or: docker compose up -d --build
make load          # some traffic, including 404s
```

- Prometheus at http://localhost:9090: open **Status → Targets** (is `ticket-api` UP?), then try the queries from the lesson: `sum by (route) (rate(http_requests_total[5m]))`.
- **Alerts** at http://localhost:9090/alerts: `docker compose stop api` and watch `TicketApiDown` go pending, then firing after a minute.
- Grafana at http://localhost:3000: the **Services → Ticket API (RED)** dashboard is already there, because it's provisioned from the repo.

With a local Kubernetes cluster (kind, minikube or Docker Desktop's):

```text
docker build -t ghcr.io/OWNER/ticket-api:0.1.0 .
kind load docker-image ghcr.io/OWNER/ticket-api:0.1.0     # kind only: copy the image into the cluster
kubectl create namespace tickets
kubectl apply -k k8s/
kubectl -n tickets rollout status deployment/ticket-api
kubectl -n tickets port-forward svc/ticket-api 8080:80
```

Push the repository to GitHub (replace `OWNER`) and the workflow runs on every push and pull request.
'''),
    ("project", "ship_service", "Ship a Service End to End",
     "Implement the four metric functions in `metrics.py` until `python -m unittest` passes, then run the stack with Docker Compose.",
     BASE + "/starter", FILES),
    ("text", r'''
@@text
## Stretch goals

- **Terraform** the registry and cluster: an ECR repository (or GHCR stays) and an EKS/AKS/GKE cluster with the modules from the Terraform track, planned on pull requests.
- **GitOps**: put `k8s/` in a separate config repository, install Argo CD, and let the pipeline open a PR that bumps `newTag` instead of deploying.
- **Alertmanager**: add it to Compose and route `severity: page` to a Slack webhook and `severity: ticket` to email.
- **An SLO**: 99.5% of requests succeed over 30 days. Add recording rules and multi-window burn-rate alerts (fast: 14.4× over 1 h, slow: 6× over 6 h).
- **A HorizontalPodAutoscaler** and a **PodDisruptionBudget**, then `kubectl drain` a node and watch the service stay up.
- **Signed images and an SBOM**: `cosign sign` and `syft` in the pipeline, and a Kyverno policy that only admits signed images.
- **Persistence**: swap `TicketStore` for PostgreSQL in Compose and a StatefulSet (or a managed database) in Kubernetes, without changing the API.
'''),
    ("project", "ship_service_finished", "Ship a Service End to End: the finished version",
     "A complete solution to compare with yours.",
     BASE + "/finished", FILES),
    ("text", r'''
@@text
🚀 Tests, an image, a pipeline, manifests, metrics, alerts and dashboards, all versioned in one repository: that's what "you build it, you run it" looks like, and it's exactly what DevOps interviews ask you to talk through.
'''),
]
