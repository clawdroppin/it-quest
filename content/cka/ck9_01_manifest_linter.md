@@lesson ck9-manifest-linter Kubernetes Manifest Linter
@@text
Half of all Kubernetes incidents start in a manifest: an image tagged `latest` that silently changed, a container with no memory limit that ate a node, a Service whose selector has a typo so it sends traffic nowhere, a privileged log agent that gives anyone who breaks into it the whole node. Tools like kube-linter, Polaris and kube-score catch these in CI before `kubectl apply`. You'll write your own:

```text
CRITICAL  DaemonSet/ops/log-agent              agent: privileged container
HIGH      DaemonSet/ops/log-agent              uses the host network
HIGH      DaemonSet/ops/log-agent              mounts hostPath /var/log
HIGH      Deployment/shop/frontend             web: image uses the latest tag
HIGH      Service/shop/frontend                selector app=front-end matches no pods
MEDIUM    Deployment/shop/frontend             web: no resource requests
...
6 objects in 2 files, 14 problems
```

It reads **JSON** manifests (Python has no YAML parser built in, and kubectl prints JSON with `-o json`), from files that hold one object, a list, or a `kind: List` (what `kubectl get ... -o json` gives you). It exits with **2** on anything critical or high, so it can block a pipeline.

The linter has to know where the pod spec lives in each kind: directly in a Pod, under `spec.template.spec` in Deployments, StatefulSets, DaemonSets and Jobs, and one level deeper (`spec.jobTemplate.spec.template.spec`) in a CronJob. `pod_spec` and `pod_labels` already handle that. You'll write:

1. `image_tag`: harder than it looks (registry ports, digests, no tag at all).
2. `lint_container`: the per-container rules.
3. `lint_object`: host access, replicas, and running the container checks (init containers and batch jobs don't need probes).
4. `check_services`: the cross-object check: does any pod in the namespace match each Service's selector? That's the `k get endpoints` is empty problem from the troubleshooting lesson, caught before deployment.

Not every finding is wrong. A log agent *needs* `/var/log` from the host; that's why real linters let you annotate exceptions. But each one should be a decision, not an accident.
@@project manifest_linter Kubernetes Manifest Linter
Write `image_tag`, `lint_container`, `lint_object` and `check_services` in `kubelint.py` until `python -m unittest` passes. Then `python main.py`, and on your own cluster: `kubectl get deploy,sts,ds,cronjob,svc -A -o json > cluster.json` and `python main.py cluster.json`.
@@file kubelint.py
"""Lint Kubernetes manifests (JSON, as `kubectl get -o json` or `kubectl create --dry-run=client -o json` prints)."""
from dataclasses import dataclass

SEVERITIES = ["CRITICAL", "HIGH", "MEDIUM", "LOW"]
WORKLOADS = {"Deployment", "StatefulSet", "DaemonSet", "ReplicaSet", "Job"}


@dataclass(frozen=True)
class Problem:
    severity: str
    object: str  # "Deployment/shop/api"
    message: str


def ref(obj: dict) -> str:
    meta = obj.get("metadata", {})
    return f"{obj.get('kind')}/{meta.get('namespace', 'default')}/{meta.get('name')}"


def flatten(doc) -> list[dict]:
    """A file can hold one object, a list of objects, or a `kind: List` with `items`."""
    if isinstance(doc, list):
        return [o for d in doc for o in flatten(d)]
    if doc.get("kind") == "List":
        return flatten(doc.get("items", []))
    return [doc]


def pod_spec(obj: dict) -> dict | None:
    """The pod spec inside a Pod, a workload, or a CronJob; None for other kinds."""
    kind = obj.get("kind")
    spec = obj.get("spec", {})
    if kind == "Pod":
        return spec
    if kind in WORKLOADS:
        return spec.get("template", {}).get("spec", {})
    if kind == "CronJob":
        return spec.get("jobTemplate", {}).get("spec", {}).get("template", {}).get("spec", {})
    return None


def pod_labels(obj: dict) -> dict:
    """The labels pods of this object get."""
    if obj.get("kind") == "Pod":
        return obj.get("metadata", {}).get("labels", {})
    return obj.get("spec", {}).get("template", {}).get("metadata", {}).get("labels", {})


# HINT: a registry can have a port ("registry.local:5000/app"), so only look for ":" after the last "/".
# HINT: a digest ("app@sha256:...") pins the image exactly: return the digest part, starting with "sha256:".
def image_tag(image: str) -> str:
    """The tag of an image reference: "nginx:1.27" → "1.27", "nginx" → "latest" (that's what
    Docker assumes), "registry.local:5000/team/app" → "latest", "app@sha256:ab12…" → "sha256:ab12…"."""
    raise NotImplementedError


# HINT: use .get() with defaults everywhere: any of these fields can be missing.
def lint_container(where: str, c: dict, pod_sc: dict) -> list[Problem]:
    """Problems for one container. `where` is the object's ref, `pod_sc` the pod-level securityContext.

    - securityContext.privileged is true → CRITICAL "<name>: privileged container"
    - the image tag is "latest" → HIGH "<name>: image uses the latest tag"
    - no resources.requests (or empty) → MEDIUM "<name>: no resource requests"
    - no memory limit (resources.limits.memory) → MEDIUM "<name>: no memory limit"
    - runAsNonRoot isn't true in the container's securityContext NOR in the pod's → MEDIUM "<name>: may run as root"
    - no readinessProbe → LOW "<name>: no readiness probe"
    - no livenessProbe → LOW "<name>: no liveness probe"
    Problems are returned in that order."""
    raise NotImplementedError


# HINT: init containers and batch jobs don't serve traffic: check them, but drop the probe problems.
def lint_object(obj: dict) -> list[Problem]:
    """Problems for one object. For anything with a pod spec:

    - spec.hostNetwork is true → HIGH "uses the host network"
    - any volume with a hostPath → HIGH "mounts hostPath <path>"
    - every container (lint_container) and init container (the same, without probe problems).
      Containers of a Job or CronJob don't get probe problems either.
    For a Deployment or StatefulSet, also: replicas (default 1) below 2 → LOW "only <n> replica(s)".
    Objects without a pod spec have no problems here."""
    raise NotImplementedError


# HINT: a selector matches labels when every key/value in the selector is in the labels.
# HINT: only compare with objects in the same namespace (missing namespace means "default").
def check_services(objects: list[dict]) -> list[Problem]:
    """A Service whose selector matches the pod labels of no Pod or workload in its namespace
    → HIGH "selector <k=v,...> matches no pods" (keys sorted). Services without a selector are skipped."""
    raise NotImplementedError


def lint(objects: list[dict]) -> list[Problem]:
    """Everything, the most severe first (a stable sort keeps the order within a severity)."""
    problems = [p for o in objects for p in lint_object(o)] + check_services(objects)
    return sorted(problems, key=lambda p: SEVERITIES.index(p.severity))
@@file test_kubelint.py
import unittest

from kubelint import Problem, check_services, image_tag, lint, lint_container, lint_object

GOOD = {
    "name": "api",
    "image": "ghcr.io/acme/api:2.1.0",
    "resources": {"requests": {"cpu": "100m", "memory": "128Mi"}, "limits": {"memory": "256Mi"}},
    "securityContext": {"runAsNonRoot": True},
    "readinessProbe": {"httpGet": {"path": "/ready", "port": 8080}},
    "livenessProbe": {"httpGet": {"path": "/health", "port": 8080}},
}


def deployment(containers, replicas=3, labels=None, ns="shop", **spec):
    return {
        "kind": "Deployment",
        "metadata": {"name": "api", "namespace": ns},
        "spec": {
            "replicas": replicas,
            "template": {"metadata": {"labels": labels or {"app": "api"}}, "spec": {"containers": containers, **spec}},
        },
    }


class Images(unittest.TestCase):
    def test_tags(self):
        self.assertEqual(image_tag("nginx:1.27"), "1.27")
        self.assertEqual(image_tag("nginx"), "latest")
        self.assertEqual(image_tag("nginx:latest"), "latest")
        self.assertEqual(image_tag("registry.local:5000/team/app"), "latest")
        self.assertEqual(image_tag("registry.local:5000/team/app:v3"), "v3")
        self.assertEqual(image_tag("app@sha256:ab12"), "sha256:ab12")


class Containers(unittest.TestCase):
    def test_good(self):
        self.assertEqual(lint_container("D/x/y", GOOD, {}), [])

    def test_bad(self):
        c = {"name": "web", "image": "nginx", "securityContext": {"privileged": True}}
        self.assertEqual([(p.severity, p.message) for p in lint_container("D/x/y", c, {})], [
            ("CRITICAL", "web: privileged container"),
            ("HIGH", "web: image uses the latest tag"),
            ("MEDIUM", "web: no resource requests"),
            ("MEDIUM", "web: no memory limit"),
            ("MEDIUM", "web: may run as root"),
            ("LOW", "web: no readiness probe"),
            ("LOW", "web: no liveness probe"),
        ])

    def test_pod_level_non_root(self):
        c = {k: v for k, v in GOOD.items() if k != "securityContext"}
        self.assertEqual(lint_container("D/x/y", c, {"runAsNonRoot": True}), [])
        self.assertEqual(len(lint_container("D/x/y", c, {})), 1)

    def test_cpu_limit_is_not_memory(self):
        c = dict(GOOD, resources={"requests": {"cpu": "1"}, "limits": {"cpu": "1"}})
        self.assertEqual([p.message for p in lint_container("D/x/y", c, {})], ["api: no memory limit"])


class Objects(unittest.TestCase):
    def test_good_deployment(self):
        self.assertEqual(lint_object(deployment([GOOD])), [])

    def test_host_access_and_replicas(self):
        d = deployment([GOOD], replicas=1, hostNetwork=True,
                       volumes=[{"name": "sock", "hostPath": {"path": "/var/run/docker.sock"}}])
        self.assertEqual(lint_object(d), [
            Problem("HIGH", "Deployment/shop/api", "uses the host network"),
            Problem("HIGH", "Deployment/shop/api", "mounts hostPath /var/run/docker.sock"),
            Problem("LOW", "Deployment/shop/api", "only 1 replica(s)"),
        ])

    def test_default_replicas(self):
        d = deployment([GOOD])
        del d["spec"]["replicas"]
        self.assertEqual([p.message for p in lint_object(d)], ["only 1 replica(s)"])

    def test_init_containers_skip_probes(self):
        init = {"name": "migrate", "image": "ghcr.io/acme/migrate:1", "securityContext": {"runAsNonRoot": True},
                "resources": {"requests": {"cpu": "50m"}, "limits": {"memory": "64Mi"}}}
        self.assertEqual(lint_object(deployment([GOOD], initContainers=[init])), [])

    def test_cronjob_and_daemonset(self):
        cj = {"kind": "CronJob", "metadata": {"name": "backup"},
              "spec": {"jobTemplate": {"spec": {"template": {"spec": {"containers": [dict(GOOD, image="busybox")]}}}}}}
        self.assertEqual([p.message for p in lint_object(cj)], ["api: image uses the latest tag"])
        job = {"kind": "Job", "metadata": {"name": "once"},
               "spec": {"template": {"spec": {"containers": [{k: v for k, v in GOOD.items() if "Probe" not in k}]}}}}
        self.assertEqual(lint_object(job), [], "batch jobs need no probes")
        ds = {"kind": "DaemonSet", "metadata": {"name": "agent", "namespace": "ops"},
              "spec": {"template": {"spec": {"containers": [GOOD]}}}}
        self.assertEqual(lint_object(ds), [], "DaemonSets have no replicas")

    def test_other_kinds(self):
        self.assertEqual(lint_object({"kind": "ConfigMap", "metadata": {"name": "c"}}), [])


class Services(unittest.TestCase):
    def svc(self, selector, ns="shop"):
        return {"kind": "Service", "metadata": {"name": "api", "namespace": ns}, "spec": {"selector": selector}}

    def test_match(self):
        objs = [deployment([GOOD], labels={"app": "api", "tier": "web"}), self.svc({"app": "api"})]
        self.assertEqual(check_services(objs), [])

    def test_typo(self):
        objs = [deployment([GOOD]), self.svc({"tier": "web", "app": "apl"})]
        self.assertEqual(check_services(objs),
                         [Problem("HIGH", "Service/shop/api", "selector app=apl,tier=web matches no pods")])

    def test_other_namespace(self):
        objs = [deployment([GOOD], ns="other"), self.svc({"app": "api"})]
        self.assertEqual(len(check_services(objs)), 1)

    def test_no_selector(self):
        self.assertEqual(check_services([{"kind": "Service", "metadata": {"name": "ext"}, "spec": {}}]), [])


class All(unittest.TestCase):
    def test_sorted(self):
        objs = [deployment([GOOD], replicas=1), deployment([dict(GOOD, securityContext={"privileged": True, "runAsNonRoot": True})])]
        self.assertEqual([p.severity for p in lint(objs)], ["CRITICAL", "LOW"])


if __name__ == "__main__":
    unittest.main()
@@file main.py
"""`python main.py [files or folders...]`: lint every .json manifest, exit 2 on CRITICAL or HIGH.

Export from a cluster:   kubectl get deploy,sts,ds,cronjob,svc -A -o json > cluster.json
Check before applying:   kubectl create deployment x --image=nginx --dry-run=client -o json > x.json"""
import json
import sys
from pathlib import Path

from kubelint import flatten, lint


def main(argv: list[str]) -> int:
    targets = [Path(a) for a in argv[1:]] or [Path("manifests")]
    files = [f for t in targets for f in (sorted(t.glob("*.json")) if t.is_dir() else [t])]
    objects = [o for f in files for o in flatten(json.loads(f.read_text(encoding="utf-8")))]
    problems = lint(objects)
    for p in problems:
        print(f"{p.severity:<9} {p.object:<36} {p.message}")
    print(f"{len(objects)} objects in {len(files)} files, {len(problems)} problems")
    return 2 if any(p.severity in ("CRITICAL", "HIGH") for p in problems) else 0


if __name__ == "__main__":
    sys.exit(main(sys.argv))
@@file manifests/shop.json
{
  "kind": "List",
  "items": [
    {
      "apiVersion": "apps/v1",
      "kind": "Deployment",
      "metadata": { "name": "api", "namespace": "shop" },
      "spec": {
        "replicas": 3,
        "selector": { "matchLabels": { "app": "api" } },
        "template": {
          "metadata": { "labels": { "app": "api" } },
          "spec": {
            "securityContext": { "runAsNonRoot": true, "runAsUser": 10001 },
            "initContainers": [
              {
                "name": "migrate",
                "image": "ghcr.io/acme/api:2.1.0",
                "command": ["/app/migrate"],
                "resources": { "requests": { "cpu": "50m", "memory": "64Mi" }, "limits": { "memory": "128Mi" } }
              }
            ],
            "containers": [
              {
                "name": "api",
                "image": "ghcr.io/acme/api:2.1.0",
                "ports": [{ "containerPort": 8080 }],
                "resources": { "requests": { "cpu": "100m", "memory": "128Mi" }, "limits": { "memory": "256Mi" } },
                "readinessProbe": { "httpGet": { "path": "/ready", "port": 8080 } },
                "livenessProbe": { "httpGet": { "path": "/health", "port": 8080 } }
              }
            ]
          }
        }
      }
    },
    {
      "apiVersion": "v1",
      "kind": "Service",
      "metadata": { "name": "api", "namespace": "shop" },
      "spec": { "selector": { "app": "api" }, "ports": [{ "port": 80, "targetPort": 8080 }] }
    },
    {
      "apiVersion": "apps/v1",
      "kind": "Deployment",
      "metadata": { "name": "frontend", "namespace": "shop" },
      "spec": {
        "replicas": 1,
        "selector": { "matchLabels": { "app": "frontend" } },
        "template": {
          "metadata": { "labels": { "app": "frontend" } },
          "spec": {
            "containers": [{ "name": "web", "image": "nginx", "ports": [{ "containerPort": 80 }] }]
          }
        }
      }
    },
    {
      "apiVersion": "v1",
      "kind": "Service",
      "metadata": { "name": "frontend", "namespace": "shop" },
      "spec": { "selector": { "app": "front-end" }, "ports": [{ "port": 80 }] }
    }
  ]
}
@@file manifests/ops.json
[
  {
    "apiVersion": "apps/v1",
    "kind": "DaemonSet",
    "metadata": { "name": "log-agent", "namespace": "ops" },
    "spec": {
      "selector": { "matchLabels": { "app": "log-agent" } },
      "template": {
        "metadata": { "labels": { "app": "log-agent" } },
        "spec": {
          "hostNetwork": true,
          "volumes": [{ "name": "varlog", "hostPath": { "path": "/var/log" } }],
          "containers": [
            {
              "name": "agent",
              "image": "fluent/fluent-bit:3.1",
              "securityContext": { "privileged": true },
              "resources": { "requests": { "cpu": "50m", "memory": "64Mi" }, "limits": { "memory": "200Mi" } },
              "volumeMounts": [{ "name": "varlog", "mountPath": "/var/log" }]
            }
          ]
        }
      }
    }
  },
  {
    "apiVersion": "batch/v1",
    "kind": "CronJob",
    "metadata": { "name": "db-backup", "namespace": "ops" },
    "spec": {
      "schedule": "0 3 * * *",
      "jobTemplate": {
        "spec": {
          "template": {
            "spec": {
              "restartPolicy": "OnFailure",
              "securityContext": { "runAsNonRoot": true, "runAsUser": 999 },
              "containers": [
                {
                  "name": "backup",
                  "image": "postgres:16",
                  "command": ["sh", "-c", "pg_dump -h db.shop -U backup shop > /backup/shop.sql"],
                  "resources": { "requests": { "cpu": "100m", "memory": "128Mi" }, "limits": { "memory": "512Mi" } }
                }
              ]
            }
          }
        }
      }
    }
  }
]
@@text
> Tips: for `image_tag`, split on `@` first (a digest wins), then take the part after the last `/` and look for `:` there. In `lint_container`, `c.get("resources", {}).get("limits", {})` never fails, however incomplete the manifest. For `check_services`, `all(labels.get(k) == v for k, v in selector.items())` is the whole matching rule.

Then fix the manifests: make `frontend` pass with no HIGH problems. You'll be writing exactly the YAML (here JSON) the CKA expects you to know: resources, probes, `securityContext`, matching labels.
@@project manifest_linter_finished Kubernetes Manifest Linter: the finished version
The complete linter. Ideas: an annotation like `kubelint/ignore: "hostPath"` to accept a finding on purpose, a check that every Deployment has a PodDisruptionBudget, that `selector.matchLabels` matches the template labels, or a YAML reader (`pip install pyyaml`) so it works on the files you actually write.
@@file kubelint.py
"""Lint Kubernetes manifests (JSON, as `kubectl get -o json` or `kubectl create --dry-run=client -o json` prints)."""
from dataclasses import dataclass

SEVERITIES = ["CRITICAL", "HIGH", "MEDIUM", "LOW"]
WORKLOADS = {"Deployment", "StatefulSet", "DaemonSet", "ReplicaSet", "Job"}


@dataclass(frozen=True)
class Problem:
    severity: str
    object: str  # "Deployment/shop/api"
    message: str


def ref(obj: dict) -> str:
    meta = obj.get("metadata", {})
    return f"{obj.get('kind')}/{meta.get('namespace', 'default')}/{meta.get('name')}"


def flatten(doc) -> list[dict]:
    """A file can hold one object, a list of objects, or a `kind: List` with `items`."""
    if isinstance(doc, list):
        return [o for d in doc for o in flatten(d)]
    if doc.get("kind") == "List":
        return flatten(doc.get("items", []))
    return [doc]


def pod_spec(obj: dict) -> dict | None:
    """The pod spec inside a Pod, a workload, or a CronJob; None for other kinds."""
    kind = obj.get("kind")
    spec = obj.get("spec", {})
    if kind == "Pod":
        return spec
    if kind in WORKLOADS:
        return spec.get("template", {}).get("spec", {})
    if kind == "CronJob":
        return spec.get("jobTemplate", {}).get("spec", {}).get("template", {}).get("spec", {})
    return None


def pod_labels(obj: dict) -> dict:
    """The labels pods of this object get."""
    if obj.get("kind") == "Pod":
        return obj.get("metadata", {}).get("labels", {})
    return obj.get("spec", {}).get("template", {}).get("metadata", {}).get("labels", {})


# HINT: a registry can have a port ("registry.local:5000/app"), so only look for ":" after the last "/".
# HINT: a digest ("app@sha256:...") pins the image exactly: return the digest part, starting with "sha256:".
def image_tag(image: str) -> str:
    """The tag of an image reference: "nginx:1.27" → "1.27", "nginx" → "latest" (that's what
    Docker assumes), "registry.local:5000/team/app" → "latest", "app@sha256:ab12…" → "sha256:ab12…"."""
    if "@" in image:
        return image.split("@", 1)[1]
    last = image.rsplit("/", 1)[-1]
    return last.split(":", 1)[1] if ":" in last else "latest"


# HINT: use .get() with defaults everywhere: any of these fields can be missing.
def lint_container(where: str, c: dict, pod_sc: dict) -> list[Problem]:
    """Problems for one container. `where` is the object's ref, `pod_sc` the pod-level securityContext.

    - securityContext.privileged is true → CRITICAL "<name>: privileged container"
    - the image tag is "latest" → HIGH "<name>: image uses the latest tag"
    - no resources.requests (or empty) → MEDIUM "<name>: no resource requests"
    - no memory limit (resources.limits.memory) → MEDIUM "<name>: no memory limit"
    - runAsNonRoot isn't true in the container's securityContext NOR in the pod's → MEDIUM "<name>: may run as root"
    - no readinessProbe → LOW "<name>: no readiness probe"
    - no livenessProbe → LOW "<name>: no liveness probe"
    Problems are returned in that order."""
    name = c.get("name", "?")
    sc = c.get("securityContext", {})
    res = c.get("resources", {})
    out = []

    def add(sev, msg):
        out.append(Problem(sev, where, f"{name}: {msg}"))

    if sc.get("privileged"):
        add("CRITICAL", "privileged container")
    if image_tag(c.get("image", "")) == "latest":
        add("HIGH", "image uses the latest tag")
    if not res.get("requests"):
        add("MEDIUM", "no resource requests")
    if "memory" not in res.get("limits", {}):
        add("MEDIUM", "no memory limit")
    if not (sc.get("runAsNonRoot") or pod_sc.get("runAsNonRoot")):
        add("MEDIUM", "may run as root")
    if "readinessProbe" not in c:
        add("LOW", "no readiness probe")
    if "livenessProbe" not in c:
        add("LOW", "no liveness probe")
    return out


# HINT: init containers and batch jobs don't serve traffic: check them, but drop the probe problems.
def lint_object(obj: dict) -> list[Problem]:
    """Problems for one object. For anything with a pod spec:

    - spec.hostNetwork is true → HIGH "uses the host network"
    - any volume with a hostPath → HIGH "mounts hostPath <path>"
    - every container (lint_container) and init container (the same, without probe problems).
      Containers of a Job or CronJob don't get probe problems either.
    For a Deployment or StatefulSet, also: replicas (default 1) below 2 → LOW "only <n> replica(s)".
    Objects without a pod spec have no problems here."""
    spec = pod_spec(obj)
    if spec is None:
        return []
    where = ref(obj)
    out = []
    if spec.get("hostNetwork"):
        out.append(Problem("HIGH", where, "uses the host network"))
    for v in spec.get("volumes", []):
        if "hostPath" in v:
            out.append(Problem("HIGH", where, f"mounts hostPath {v['hostPath'].get('path')}"))
    pod_sc = spec.get("securityContext", {})
    batch = obj.get("kind") in ("Job", "CronJob")
    for c in spec.get("containers", []):
        out += [p for p in lint_container(where, c, pod_sc) if not (batch and "probe" in p.message)]
    for c in spec.get("initContainers", []):
        out += [p for p in lint_container(where, c, pod_sc) if "probe" not in p.message]
    if obj.get("kind") in ("Deployment", "StatefulSet"):
        n = obj.get("spec", {}).get("replicas", 1)
        if n < 2:
            out.append(Problem("LOW", where, f"only {n} replica(s)"))
    return out


# HINT: a selector matches labels when every key/value in the selector is in the labels.
# HINT: only compare with objects in the same namespace (missing namespace means "default").
def check_services(objects: list[dict]) -> list[Problem]:
    """A Service whose selector matches the pod labels of no Pod or workload in its namespace
    → HIGH "selector <k=v,...> matches no pods" (keys sorted). Services without a selector are skipped."""
    out = []
    for svc in objects:
        if svc.get("kind") != "Service":
            continue
        selector = svc.get("spec", {}).get("selector")
        if not selector:
            continue
        ns = svc.get("metadata", {}).get("namespace", "default")
        found = False
        for o in objects:
            if pod_spec(o) is None or o.get("metadata", {}).get("namespace", "default") != ns:
                continue
            labels = pod_labels(o)
            if all(labels.get(k) == v for k, v in selector.items()):
                found = True
                break
        if not found:
            text = ",".join(f"{k}={selector[k]}" for k in sorted(selector))
            out.append(Problem("HIGH", ref(svc), f"selector {text} matches no pods"))
    return out


def lint(objects: list[dict]) -> list[Problem]:
    """Everything, the most severe first (a stable sort keeps the order within a severity)."""
    problems = [p for o in objects for p in lint_object(o)] + check_services(objects)
    return sorted(problems, key=lambda p: SEVERITIES.index(p.severity))
@@file test_kubelint.py
import unittest

from kubelint import Problem, check_services, image_tag, lint, lint_container, lint_object

GOOD = {
    "name": "api",
    "image": "ghcr.io/acme/api:2.1.0",
    "resources": {"requests": {"cpu": "100m", "memory": "128Mi"}, "limits": {"memory": "256Mi"}},
    "securityContext": {"runAsNonRoot": True},
    "readinessProbe": {"httpGet": {"path": "/ready", "port": 8080}},
    "livenessProbe": {"httpGet": {"path": "/health", "port": 8080}},
}


def deployment(containers, replicas=3, labels=None, ns="shop", **spec):
    return {
        "kind": "Deployment",
        "metadata": {"name": "api", "namespace": ns},
        "spec": {
            "replicas": replicas,
            "template": {"metadata": {"labels": labels or {"app": "api"}}, "spec": {"containers": containers, **spec}},
        },
    }


class Images(unittest.TestCase):
    def test_tags(self):
        self.assertEqual(image_tag("nginx:1.27"), "1.27")
        self.assertEqual(image_tag("nginx"), "latest")
        self.assertEqual(image_tag("nginx:latest"), "latest")
        self.assertEqual(image_tag("registry.local:5000/team/app"), "latest")
        self.assertEqual(image_tag("registry.local:5000/team/app:v3"), "v3")
        self.assertEqual(image_tag("app@sha256:ab12"), "sha256:ab12")


class Containers(unittest.TestCase):
    def test_good(self):
        self.assertEqual(lint_container("D/x/y", GOOD, {}), [])

    def test_bad(self):
        c = {"name": "web", "image": "nginx", "securityContext": {"privileged": True}}
        self.assertEqual([(p.severity, p.message) for p in lint_container("D/x/y", c, {})], [
            ("CRITICAL", "web: privileged container"),
            ("HIGH", "web: image uses the latest tag"),
            ("MEDIUM", "web: no resource requests"),
            ("MEDIUM", "web: no memory limit"),
            ("MEDIUM", "web: may run as root"),
            ("LOW", "web: no readiness probe"),
            ("LOW", "web: no liveness probe"),
        ])

    def test_pod_level_non_root(self):
        c = {k: v for k, v in GOOD.items() if k != "securityContext"}
        self.assertEqual(lint_container("D/x/y", c, {"runAsNonRoot": True}), [])
        self.assertEqual(len(lint_container("D/x/y", c, {})), 1)

    def test_cpu_limit_is_not_memory(self):
        c = dict(GOOD, resources={"requests": {"cpu": "1"}, "limits": {"cpu": "1"}})
        self.assertEqual([p.message for p in lint_container("D/x/y", c, {})], ["api: no memory limit"])


class Objects(unittest.TestCase):
    def test_good_deployment(self):
        self.assertEqual(lint_object(deployment([GOOD])), [])

    def test_host_access_and_replicas(self):
        d = deployment([GOOD], replicas=1, hostNetwork=True,
                       volumes=[{"name": "sock", "hostPath": {"path": "/var/run/docker.sock"}}])
        self.assertEqual(lint_object(d), [
            Problem("HIGH", "Deployment/shop/api", "uses the host network"),
            Problem("HIGH", "Deployment/shop/api", "mounts hostPath /var/run/docker.sock"),
            Problem("LOW", "Deployment/shop/api", "only 1 replica(s)"),
        ])

    def test_default_replicas(self):
        d = deployment([GOOD])
        del d["spec"]["replicas"]
        self.assertEqual([p.message for p in lint_object(d)], ["only 1 replica(s)"])

    def test_init_containers_skip_probes(self):
        init = {"name": "migrate", "image": "ghcr.io/acme/migrate:1", "securityContext": {"runAsNonRoot": True},
                "resources": {"requests": {"cpu": "50m"}, "limits": {"memory": "64Mi"}}}
        self.assertEqual(lint_object(deployment([GOOD], initContainers=[init])), [])

    def test_cronjob_and_daemonset(self):
        cj = {"kind": "CronJob", "metadata": {"name": "backup"},
              "spec": {"jobTemplate": {"spec": {"template": {"spec": {"containers": [dict(GOOD, image="busybox")]}}}}}}
        self.assertEqual([p.message for p in lint_object(cj)], ["api: image uses the latest tag"])
        job = {"kind": "Job", "metadata": {"name": "once"},
               "spec": {"template": {"spec": {"containers": [{k: v for k, v in GOOD.items() if "Probe" not in k}]}}}}
        self.assertEqual(lint_object(job), [], "batch jobs need no probes")
        ds = {"kind": "DaemonSet", "metadata": {"name": "agent", "namespace": "ops"},
              "spec": {"template": {"spec": {"containers": [GOOD]}}}}
        self.assertEqual(lint_object(ds), [], "DaemonSets have no replicas")

    def test_other_kinds(self):
        self.assertEqual(lint_object({"kind": "ConfigMap", "metadata": {"name": "c"}}), [])


class Services(unittest.TestCase):
    def svc(self, selector, ns="shop"):
        return {"kind": "Service", "metadata": {"name": "api", "namespace": ns}, "spec": {"selector": selector}}

    def test_match(self):
        objs = [deployment([GOOD], labels={"app": "api", "tier": "web"}), self.svc({"app": "api"})]
        self.assertEqual(check_services(objs), [])

    def test_typo(self):
        objs = [deployment([GOOD]), self.svc({"tier": "web", "app": "apl"})]
        self.assertEqual(check_services(objs),
                         [Problem("HIGH", "Service/shop/api", "selector app=apl,tier=web matches no pods")])

    def test_other_namespace(self):
        objs = [deployment([GOOD], ns="other"), self.svc({"app": "api"})]
        self.assertEqual(len(check_services(objs)), 1)

    def test_no_selector(self):
        self.assertEqual(check_services([{"kind": "Service", "metadata": {"name": "ext"}, "spec": {}}]), [])


class All(unittest.TestCase):
    def test_sorted(self):
        objs = [deployment([GOOD], replicas=1), deployment([dict(GOOD, securityContext={"privileged": True, "runAsNonRoot": True})])]
        self.assertEqual([p.severity for p in lint(objs)], ["CRITICAL", "LOW"])


if __name__ == "__main__":
    unittest.main()
@@file main.py
"""`python main.py [files or folders...]`: lint every .json manifest, exit 2 on CRITICAL or HIGH.

Export from a cluster:   kubectl get deploy,sts,ds,cronjob,svc -A -o json > cluster.json
Check before applying:   kubectl create deployment x --image=nginx --dry-run=client -o json > x.json"""
import json
import sys
from pathlib import Path

from kubelint import flatten, lint


def main(argv: list[str]) -> int:
    targets = [Path(a) for a in argv[1:]] or [Path("manifests")]
    files = [f for t in targets for f in (sorted(t.glob("*.json")) if t.is_dir() else [t])]
    objects = [o for f in files for o in flatten(json.loads(f.read_text(encoding="utf-8")))]
    problems = lint(objects)
    for p in problems:
        print(f"{p.severity:<9} {p.object:<36} {p.message}")
    print(f"{len(objects)} objects in {len(files)} files, {len(problems)} problems")
    return 2 if any(p.severity in ("CRITICAL", "HIGH") for p in problems) else 0


if __name__ == "__main__":
    sys.exit(main(sys.argv))
@@file manifests/shop.json
{
  "kind": "List",
  "items": [
    {
      "apiVersion": "apps/v1",
      "kind": "Deployment",
      "metadata": { "name": "api", "namespace": "shop" },
      "spec": {
        "replicas": 3,
        "selector": { "matchLabels": { "app": "api" } },
        "template": {
          "metadata": { "labels": { "app": "api" } },
          "spec": {
            "securityContext": { "runAsNonRoot": true, "runAsUser": 10001 },
            "initContainers": [
              {
                "name": "migrate",
                "image": "ghcr.io/acme/api:2.1.0",
                "command": ["/app/migrate"],
                "resources": { "requests": { "cpu": "50m", "memory": "64Mi" }, "limits": { "memory": "128Mi" } }
              }
            ],
            "containers": [
              {
                "name": "api",
                "image": "ghcr.io/acme/api:2.1.0",
                "ports": [{ "containerPort": 8080 }],
                "resources": { "requests": { "cpu": "100m", "memory": "128Mi" }, "limits": { "memory": "256Mi" } },
                "readinessProbe": { "httpGet": { "path": "/ready", "port": 8080 } },
                "livenessProbe": { "httpGet": { "path": "/health", "port": 8080 } }
              }
            ]
          }
        }
      }
    },
    {
      "apiVersion": "v1",
      "kind": "Service",
      "metadata": { "name": "api", "namespace": "shop" },
      "spec": { "selector": { "app": "api" }, "ports": [{ "port": 80, "targetPort": 8080 }] }
    },
    {
      "apiVersion": "apps/v1",
      "kind": "Deployment",
      "metadata": { "name": "frontend", "namespace": "shop" },
      "spec": {
        "replicas": 1,
        "selector": { "matchLabels": { "app": "frontend" } },
        "template": {
          "metadata": { "labels": { "app": "frontend" } },
          "spec": {
            "containers": [{ "name": "web", "image": "nginx", "ports": [{ "containerPort": 80 }] }]
          }
        }
      }
    },
    {
      "apiVersion": "v1",
      "kind": "Service",
      "metadata": { "name": "frontend", "namespace": "shop" },
      "spec": { "selector": { "app": "front-end" }, "ports": [{ "port": 80 }] }
    }
  ]
}
@@file manifests/ops.json
[
  {
    "apiVersion": "apps/v1",
    "kind": "DaemonSet",
    "metadata": { "name": "log-agent", "namespace": "ops" },
    "spec": {
      "selector": { "matchLabels": { "app": "log-agent" } },
      "template": {
        "metadata": { "labels": { "app": "log-agent" } },
        "spec": {
          "hostNetwork": true,
          "volumes": [{ "name": "varlog", "hostPath": { "path": "/var/log" } }],
          "containers": [
            {
              "name": "agent",
              "image": "fluent/fluent-bit:3.1",
              "securityContext": { "privileged": true },
              "resources": { "requests": { "cpu": "50m", "memory": "64Mi" }, "limits": { "memory": "200Mi" } },
              "volumeMounts": [{ "name": "varlog", "mountPath": "/var/log" }]
            }
          ]
        }
      }
    }
  },
  {
    "apiVersion": "batch/v1",
    "kind": "CronJob",
    "metadata": { "name": "db-backup", "namespace": "ops" },
    "spec": {
      "schedule": "0 3 * * *",
      "jobTemplate": {
        "spec": {
          "template": {
            "spec": {
              "restartPolicy": "OnFailure",
              "securityContext": { "runAsNonRoot": true, "runAsUser": 999 },
              "containers": [
                {
                  "name": "backup",
                  "image": "postgres:16",
                  "command": ["sh", "-c", "pg_dump -h db.shop -U backup shop > /backup/shop.sql"],
                  "resources": { "requests": { "cpu": "100m", "memory": "128Mi" }, "limits": { "memory": "512Mi" } }
                }
              ]
            }
          }
        }
      }
    }
  }
]
@@text
🎉 Run it in CI with `kubectl create ... --dry-run=client -o json` or on a whole live cluster. Knowing *why* each of these rules exists is exactly what a Kubernetes administrator is hired for.
