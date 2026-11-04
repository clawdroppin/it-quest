@@lesson ck1-workloads-deep Workloads in Depth: Deployments, Rollouts, Config, Jobs, QoS and Autoscaling
@@text
"Workloads and scheduling" is 15% of the CKA, but workload objects appear in almost every task. This lesson covers how Deployments roll out and roll back, the other controllers (DaemonSet, StatefulSet, Job, CronJob), configuration with ConfigMaps and Secrets, resource requests and QoS classes, probes and the Horizontal Pod Autoscaler.

## Deployments and rollouts

A Deployment manages **ReplicaSets**, which manage Pods. Changing the Pod template creates a new ReplicaSet and scales old and new according to the strategy:

- **RollingUpdate** (default) with `maxSurge` (extra Pods above `replicas`, default **25%**, rounded **up**) and `maxUnavailable` (Pods that may be unavailable, default **25%**, rounded **down**).
- **Recreate**: kill all old Pods, then start new ones (downtime; for apps that can't run two versions at once).

```bash,norun
kubectl create deployment web --image=nginx:1.27 --replicas=4
kubectl set image deployment/web nginx=nginx:1.28 --record=false
kubectl rollout status deployment/web
kubectl rollout history deployment/web
kubectl rollout undo deployment/web --to-revision=1
kubectl rollout pause deployment/web     # batch several changes
kubectl rollout restart deployment/web   # new Pods, same spec (e.g. after a Secret change)
kubectl scale deployment/web --replicas=6
```

`minReadySeconds` and readiness probes control when a new Pod counts as available. `revisionHistoryLimit` (default 10) keeps old ReplicaSets for rollback.

## Other controllers

| Controller | Use | Notes |
|---|---|---|
| **DaemonSet** | one Pod per (matching) node | log/metrics agents, CNI; tolerations to run on control plane nodes |
| **StatefulSet** | stable identity and storage | Pods `db-0`, `db-1`; ordered start/stop; **headless Service**; `volumeClaimTemplates` give each Pod its own PVC |
| **Job** | run to completion | `completions`, `parallelism`, `backoffLimit` (default 6), `activeDeadlineSeconds`, `ttlSecondsAfterFinished` |
| **CronJob** | scheduled Jobs | cron syntax, `concurrencyPolicy` (Allow/Forbid/Replace), history limits, `suspend` |

```bash,norun
kubectl create job backup --image=busybox -- sh -c 'echo done'
kubectl create cronjob report --image=busybox --schedule="0 2 * * *" -- sh -c 'date'
```

## ConfigMaps and Secrets

- Create: `kubectl create configmap app-cfg --from-literal=MODE=prod --from-file=app.conf`; `kubectl create secret generic db --from-literal=PASSWORD=...`.
- Consume as **environment variables** (`env.valueFrom.configMapKeyRef`, `envFrom`) or **volume files** (updated automatically, except with `subPath`; env vars need a Pod restart).
- Secrets are **base64-encoded, not encrypted** by default: enable **encryption at rest** (EncryptionConfiguration on the API server), restrict with RBAC, consider external secret stores. `immutable: true` protects against accidental changes.

## Resources and QoS

- **Requests** are what the scheduler reserves; **limits** cap usage. CPU over the limit is **throttled**; memory over the limit is **OOMKilled**.
- Units: CPU `500m` = half a core; memory `256Mi`.
- **QoS classes** decide eviction order under node memory pressure:
  - **Guaranteed**: every container has CPU and memory requests **equal** to limits (evicted last).
  - **Burstable**: at least one request or limit set, but not Guaranteed.
  - **BestEffort**: no requests or limits at all (evicted first).
- **LimitRange** sets defaults/min/max per container in a namespace; **ResourceQuota** caps namespace totals (and requires requests/limits on Pods once set for those resources).

## Probes

- **livenessProbe**: restart the container when it fails.
- **readinessProbe**: remove the Pod from Service endpoints while it fails.
- **startupProbe**: hold off the other probes until a slow app has started.
Types: `httpGet`, `tcpSocket`, `exec`, `grpc`; tune `initialDelaySeconds`, `periodSeconds`, `failureThreshold`.

## Horizontal Pod Autoscaler

Needs **metrics-server** and CPU/memory **requests** on the Pods. `kubectl autoscale deployment web --cpu-percent=60 --min=2 --max=10`.

**desiredReplicas = ceil(currentReplicas × currentMetric / targetMetric)**, ignored when the ratio is within a **10% tolerance**, clamped to min/max. Scale-down uses a stabilisation window (default 5 minutes) to avoid flapping. VPA adjusts requests instead; Cluster Autoscaler adds nodes.

@@quiz
A Deployment has 4 replicas and the defaults maxSurge 25% and maxUnavailable 25%. During a rollout, what is the maximum number of Pods?
- 4
* 5
- 6
- 8
? 25% of 4 = 1 extra Pod.

@@quiz
Which command rolls a Deployment back to its previous revision?
* kubectl rollout undo deployment/web
- kubectl rollback deployment/web
- kubectl set image --previous
- kubectl delete rs
? Use --to-revision for a specific one.

@@quiz
Each database replica needs its own persistent volume and a stable name. Which controller?
- Deployment
- DaemonSet
* StatefulSet
- ReplicaSet
? volumeClaimTemplates and ordinal names.

@@quiz
A Pod has containers with CPU and memory requests equal to their limits. What QoS class does it get?
* Guaranteed
- Burstable
- BestEffort
- Critical
? Evicted last under memory pressure.

@@quiz
A container exceeds its memory limit. What happens?
- It's throttled
* It's OOMKilled and restarted according to restartPolicy
- The node reboots
- Nothing until the request is exceeded
? CPU is throttled; memory is killed.

@@quiz
An HPA targets 50% CPU. 4 Pods average 90%. How many replicas will it want?
- 5
- 6
* 8
- 10
? ceil(4 × 90 / 50) = ceil(7.2) = 8.

@@flashcards
Deployment :: manages ReplicaSets
maxSurge default :: 25%, rounded up
maxUnavailable default :: 25%, rounded down
Recreate :: kill all, then start new
rollout undo :: roll back
rollout restart :: new Pods, same spec
DaemonSet :: one Pod per node
StatefulSet :: stable names and storage
Job backoffLimit :: default 6
CronJob concurrencyPolicy :: Allow, Forbid, Replace
Secret encoding :: base64, not encryption
env from ConfigMap :: needs Pod restart to update
CPU over limit :: throttled
memory over limit :: OOMKilled
Guaranteed :: requests = limits for all
BestEffort :: no requests or limits
LimitRange :: per-container defaults
ResourceQuota :: namespace totals
readinessProbe :: remove from endpoints
livenessProbe :: restart container
startupProbe :: protect slow starts
HPA formula :: ceil(current × metric / target)
HPA needs :: metrics-server + requests

@@drill
Which kubectl command shows a Deployment's rollout progress?
= kubectl rollout status deployment/web
= kubectl rollout status deploy/web
? history lists revisions.

@@drill
Which QoS class is evicted first under memory pressure?
= BestEffort
= besteffort
? No requests or limits.

@@exercise Simulate a rolling update
Simulate a RollingUpdate from version v1 to v2. Compute `maxSurge` (round up) and `maxUnavailable` (round down) from percentages. Each step: create as many v2 Pods as the surge allows (total ≤ replicas + surge), then (new Pods become ready instantly) remove v1 Pods while available Pods stay ≥ replicas − maxUnavailable. Print the counts after each step until all `replicas` Pods are v2. If both values are 0, print an error (Kubernetes rejects that).
@@lang python
@@starter
import math

scenarios = [(4, "25%", "25%"), (10, "25%", "0%"), (3, "0%", "1"), (5, "0%", "0%")]

def resolve(value, replicas, round_up):
    raise NotImplementedError
@@expected
replicas=4 maxSurge=1 maxUnavailable=1
  step 1: v1=2 v2=1 total=3
  step 2: v1=0 v2=3 total=3
  step 3: v1=0 v2=4 total=4
replicas=10 maxSurge=3 maxUnavailable=0
  step 1: v1=7 v2=3 total=10
  step 2: v1=4 v2=6 total=10
  step 3: v1=1 v2=9 total=10
  step 4: v1=0 v2=10 total=10
replicas=3 maxSurge=0 maxUnavailable=1
  step 1: v1=2 v2=0 total=2
  step 2: v1=1 v2=1 total=2
  step 3: v1=0 v2=2 total=2
  step 4: v1=0 v2=3 total=3
replicas=5 maxSurge=0 maxUnavailable=0
  error: maxSurge and maxUnavailable cannot both be 0
@@requires
math.ceil(
math.floor(
def resolve
@@hint
`resolve("25%", 10, True)` → `math.ceil(10 * 25 / 100)` = 3; an absolute value like `"1"` is just `int`. In each step, `new_count += min(replicas - new, replicas + surge - old - new)`; then `remove = min(old, old + new - (replicas - unavailable))`.
@@solution
import math

scenarios = [(4, "25%", "25%"), (10, "25%", "0%"), (3, "0%", "1"), (5, "0%", "0%")]

def resolve(value, replicas, round_up):
    if value.endswith("%"):
        raw = replicas * int(value[:-1]) / 100
        return math.ceil(raw) if round_up else math.floor(raw)
    return int(value)

for replicas, surge_s, unavail_s in scenarios:
    surge = resolve(surge_s, replicas, True)
    unavailable = resolve(unavail_s, replicas, False)
    print(f"replicas={replicas} maxSurge={surge} maxUnavailable={unavailable}")
    if surge == 0 and unavailable == 0:
        print("  error: maxSurge and maxUnavailable cannot both be 0")
        continue
    old, new, step = replicas, 0, 0
    while old > 0 or new < replicas:
        step += 1
        new += min(replicas - new, replicas + surge - old - new)
        remove = max(0, min(old, old + new - (replicas - unavailable)))
        old -= remove
        print(f"  step {step}: v1={old} v2={new} total={old + new}")

@@exercise QoS classes and HPA decisions
For each Pod spec, classify its QoS class from its containers' requests and limits (a limit without a request means the request equals the limit). Then compute the HPA decision for each Deployment: desired = ceil(current × metric / target), no change if the ratio is within 10% of 1.0, clamped to min/max.
@@lang python
@@starter
import math

pods = {
    "api": [{"requests": {"cpu": "500m", "memory": "256Mi"}, "limits": {"cpu": "500m", "memory": "256Mi"}}],
    "web": [{"requests": {"cpu": "100m"}, "limits": {"cpu": "500m", "memory": "128Mi"}}],
    "batch": [{"requests": {}, "limits": {}}],
    "cache": [{"requests": {}, "limits": {"cpu": "1", "memory": "1Gi"}}],
    "mixed": [{"requests": {"cpu": "1", "memory": "1Gi"}, "limits": {"cpu": "1", "memory": "1Gi"}}, {"requests": {}, "limits": {}}],
}
hpas = [("web", 4, 90, 50, 2, 10), ("api", 6, 52, 50, 2, 10), ("worker", 10, 10, 60, 3, 20), ("search", 8, 200, 50, 2, 20)]

def qos(containers):
    raise NotImplementedError
@@expected
api: Guaranteed
web: Burstable
batch: BestEffort
cache: Guaranteed
mixed: Burstable
HPA web: 4 -> 8 (ratio 1.80)
HPA api: ratio 1.04 within tolerance, stays at 6
HPA worker: 10 -> 3 (ratio 0.17)
HPA search: 8 -> 20 (ratio 4.00)
@@requires
def qos
math.ceil(
0.1
@@hint
For each container, `req = {**limits, **requests}` fills missing requests from limits. Guaranteed needs every container to have both cpu and memory with `req == limits`; BestEffort means no container has any request or limit.
@@solution
import math

pods = {
    "api": [{"requests": {"cpu": "500m", "memory": "256Mi"}, "limits": {"cpu": "500m", "memory": "256Mi"}}],
    "web": [{"requests": {"cpu": "100m"}, "limits": {"cpu": "500m", "memory": "128Mi"}}],
    "batch": [{"requests": {}, "limits": {}}],
    "cache": [{"requests": {}, "limits": {"cpu": "1", "memory": "1Gi"}}],
    "mixed": [{"requests": {"cpu": "1", "memory": "1Gi"}, "limits": {"cpu": "1", "memory": "1Gi"}}, {"requests": {}, "limits": {}}],
}
hpas = [("web", 4, 90, 50, 2, 10), ("api", 6, 52, 50, 2, 10), ("worker", 10, 10, 60, 3, 20), ("search", 8, 200, 50, 2, 20)]

def qos(containers):
    if all(not c["requests"] and not c["limits"] for c in containers):
        return "BestEffort"
    for c in containers:
        req = {**c["limits"], **c["requests"]}
        if set(c["limits"]) != {"cpu", "memory"} or req != c["limits"]:
            return "Burstable"
    return "Guaranteed"

for name, containers in pods.items():
    print(f"{name}: {qos(containers)}")

for name, current, metric, target, low, high in hpas:
    ratio = metric / target
    if abs(ratio - 1) <= 0.1:
        print(f"HPA {name}: ratio {ratio:.2f} within tolerance, stays at {current}")
        continue
    desired = max(low, min(high, math.ceil(current * ratio)))
    print(f"HPA {name}: {current} -> {desired} (ratio {ratio:.2f})")
