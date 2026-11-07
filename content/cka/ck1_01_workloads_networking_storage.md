@@lesson ck1-workloads-networking-storage Scheduling, Networking and Storage
@@text
## A production-ready Deployment

```yaml
apiVersion: apps/v1
kind: Deployment
metadata: { name: api, namespace: shop, labels: { app: api } }
spec:
  replicas: 3
  selector: { matchLabels: { app: api } }          # must match the template's labels
  strategy: { type: RollingUpdate, rollingUpdate: { maxSurge: 1, maxUnavailable: 0 } }
  template:
    metadata: { labels: { app: api } }
    spec:
      securityContext: { runAsNonRoot: true, runAsUser: 10001 }
      containers:
        - name: api
          image: ghcr.io/acme/api:2.1.0               # a fixed tag, never :latest
          ports: [{ containerPort: 8080 }]
          resources:
            requests: { cpu: 100m, memory: 128Mi }    # what the scheduler reserves
            limits: { memory: 256Mi }                 # above this: OOMKilled
          readinessProbe: { httpGet: { path: /ready, port: 8080 } }   # gets traffic only when ready
          livenessProbe: { httpGet: { path: /health, port: 8080 }, initialDelaySeconds: 10 }  # restarted if it fails
          envFrom: [{ configMapRef: { name: api-config } }]
          securityContext: { allowPrivilegeEscalation: false, readOnlyRootFilesystem: true }
```

- **requests** decide scheduling; **limits** cap usage. CPU over the limit is *throttled*; memory over the limit is **OOMKilled**. Pods with requests equal to limits get the **Guaranteed** QoS class; with only some, **Burstable**; with none, **BestEffort** (evicted first).
- **Probes**: *readiness* removes a pod from Service endpoints while it isn't ready; *liveness* restarts it; *startup* protects slow starters from the liveness probe.
- A **LimitRange** sets default requests and limits in a namespace; a **ResourceQuota** caps a namespace's total.

## Scheduling

| Tool | Effect |
|---|---|
| `nodeSelector: { disktype: ssd }` | only nodes with that label |
| **node affinity** | the same, with required/preferred rules and operators |
| **pod (anti-)affinity** | near or away from other pods (spread replicas across zones) |
| **taints** on nodes + **tolerations** on pods | a tainted node repels pods that don't tolerate the taint |
| `nodeName` | skip the scheduler entirely |
| **topologySpreadConstraints** | spread pods evenly across zones or nodes |

```bash,norun
k label node w1 disktype=ssd
k taint node w2 gpu=true:NoSchedule          # remove: k taint node w2 gpu=true:NoSchedule-
k cordon w3; k drain w3 --ignore-daemonsets --delete-emptydir-data; k uncordon w3
```

The control-plane nodes carry the taint `node-role.kubernetes.io/control-plane:NoSchedule`, which is why normal pods don't land there. A pod stuck in **Pending**: `k describe pod` shows why ("0/3 nodes are available: insufficient memory", "untolerated taint", "didn't match node affinity").

## Services and networking

- Every pod gets its own IP; all pods can reach all pods (the **CNI** plugin, such as Calico, Cilium or Flannel, makes this happen).
- **Service types**: **ClusterIP** (internal, the default), **NodePort** (a port 30000–32767 on every node), **LoadBalancer** (a cloud load balancer), **ExternalName** (a DNS alias). A *headless* Service (`clusterIP: None`) gives StatefulSet pods their own DNS names.
- A Service forwards to the **endpoints** that match its selector. `k get endpoints api` (or `endpointslices`) empty? The selector doesn't match any *ready* pod.
- **DNS** (CoreDNS): `api.shop.svc.cluster.local`, or just `api` from inside the same namespace, `api.shop` from another one.
- **Ingress** routes HTTP by host and path to Services, through an **ingress controller** (ingress-nginx, Traefik). The newer **Gateway API** (Gateway, HTTPRoute) is replacing it and is in the current CKA curriculum.
- **NetworkPolicy**: once any policy selects a pod, only allowed traffic reaches it. Needs a CNI that enforces policies.

```yaml
apiVersion: networking.k8s.io/v1
kind: NetworkPolicy
metadata: { name: db-from-api, namespace: shop }
spec:
  podSelector: { matchLabels: { app: db } }        # protects the db pods
  policyTypes: [Ingress]
  ingress:
    - from: [{ podSelector: { matchLabels: { app: api } } }]
      ports: [{ port: 5432 }]
```

## Storage

- A **PersistentVolume** (PV) is a piece of storage; a **PersistentVolumeClaim** (PVC) is a pod's request for one. A **StorageClass** provisions PVs *dynamically* when a claim appears.
- **Access modes**: ReadWriteOnce (one node), ReadOnlyMany, ReadWriteMany (NFS, CephFS, cloud file storage), ReadWriteOncePod.
- **Reclaim policy**: Delete (the default for dynamic volumes: the data goes when the claim goes) or **Retain** (keep it for manual cleanup).
- A PVC stuck in **Pending**: no StorageClass, no matching PV (size, access mode, class name), or a `WaitForFirstConsumer` class waiting for a pod.
- **emptyDir** is scratch space that lives as long as the pod; **hostPath** mounts a node folder (dangerous outside of system pods).

@@quiz
A container keeps restarting with reason OOMKilled. What's happening?
* It uses more memory than its memory limit
- Its CPU limit is too low
- Its readiness probe fails
- The node is cordoned
? CPU over the limit is throttled; memory over the limit kills the container.

@@quiz
A Service has no endpoints, although the pods are running. What's the most likely cause?
* The Service's selector doesn't match the pods' labels (or the pods aren't ready)
- The Service type is ClusterIP
- CoreDNS is down
- The pods have no resource limits
? Compare `k get svc api -o yaml` selector with `k get pods --show-labels`.

@@quiz
You want no ordinary pods on node w2, which has GPUs, except those that explicitly allow it. What do you use?
* A taint on w2 and a toleration on the GPU pods
- A nodeSelector on every other pod
- `k cordon w2`
- A NetworkPolicy
? Taints repel; tolerations let specific pods through. Add node affinity to also *attract* them.

@@quiz
After creating a NetworkPolicy that allows ingress to the db pods only from app=api, what happens to traffic from other pods to db?
* It's denied, because a policy now selects the db pods
- It's still allowed
- Only TCP is denied
- Nothing changes until the pods restart
? Selected pods become default-deny for the listed policy types.

@@quiz
What's the safe way to take a node out for maintenance?
* `kubectl drain` (which cordons it and evicts the pods), then `kubectl uncordon` afterwards
- Shut it down and let Kubernetes notice
- `kubectl delete node` and re-join it
- Delete its pods one by one
? Drain respects PodDisruptionBudgets.

@@quiz
A PVC is Pending in a cluster with no StorageClass. What's needed?
* A matching PersistentVolume created by hand, or a StorageClass for dynamic provisioning
- A bigger node
- A Service
- A liveness probe
? Static PVs must match the claim's size, access mode and class.

@@flashcards
requests / limits :: scheduling reservation / hard cap
QoS classes :: Guaranteed, Burstable, BestEffort
readiness / liveness probe :: remove from Service / restart container
taint + toleration :: node repels pods unless they tolerate it
cordon / drain :: no new pods / evict the pods too
ClusterIP / NodePort / LoadBalancer :: internal / port on every node / cloud LB
NodePort range :: 30000–32767
service DNS :: name.namespace.svc.cluster.local
Ingress / Gateway API :: HTTP routing to Services
NetworkPolicy :: selected pods become default-deny
PV / PVC / StorageClass :: storage / claim / dynamic provisioner
reclaim policies :: Delete / Retain
RWX :: ReadWriteMany: many nodes read and write

@@drill
What's the full DNS name of Service `api` in namespace `shop` (default cluster domain)?
= api.shop.svc.cluster.local

@@exercise Is this Deployment production-ready?
Check Deployments (as parsed YAML dicts) against the lesson's production-ready checklist. Write `review(deploy)` that returns the problems:

- fewer than 2 `replicas`;
- an image with no tag or the `latest` tag;
- a container without `resources.requests` or without `resources.limits.memory`;
- no `readinessProbe` (traffic would reach Pods before they're ready) or no `livenessProbe`;
- a `RollingUpdate` strategy with `maxUnavailable` equal to the replica count (the rollout could take every Pod down).

Print each Deployment with `ready for production` or its problems.
@@lang python
@@starter
deployments = [
    {"name": "web", "replicas": 3, "strategy": {"type": "RollingUpdate", "maxUnavailable": 1},
     "containers": [{"name": "web", "image": "registry.example.com/web:1.8.2",
                     "resources": {"requests": {"cpu": "250m", "memory": "256Mi"}, "limits": {"memory": "512Mi"}},
                     "readinessProbe": {"httpGet": "/ready"}, "livenessProbe": {"httpGet": "/healthz"}}]},
    {"name": "worker", "replicas": 1, "strategy": {"type": "RollingUpdate", "maxUnavailable": 1},
     "containers": [{"name": "worker", "image": "registry.example.com/worker:latest",
                     "resources": {"requests": {"cpu": "100m"}},
                     "livenessProbe": {"exec": "check"}}]},
    {"name": "api", "replicas": 2, "strategy": {"type": "RollingUpdate", "maxUnavailable": 2},
     "containers": [{"name": "api", "image": "registry.example.com/api",
                     "resources": {}, "readinessProbe": {"httpGet": "/ready"}},
                    {"name": "proxy", "image": "envoyproxy/envoy:v1.31.0",
                     "resources": {"requests": {"cpu": "50m"}, "limits": {"memory": "128Mi"}},
                     "readinessProbe": {"tcpSocket": 9901}, "livenessProbe": {"tcpSocket": 9901}}]},
]

def image_tag(image):
    raise NotImplementedError


def review(deploy):
    raise NotImplementedError


for d in deployments:
    problems = review(d)
    print(d["name"] + ":")
    for p in problems or ["ready for production"]:
        print("  " + p)
@@expected
web:
  ready for production
worker:
  only 1 replica
  maxUnavailable can take every Pod down
  worker: pin the image tag
  worker: no memory limit
  worker: no readinessProbe
api:
  maxUnavailable can take every Pod down
  api: pin the image tag
  api: no resource requests
  api: no memory limit
  api: no livenessProbe
@@requires
def review
def image_tag
rsplit(
@@hint
For the tag, look only at the part after the last `/`: a registry like `registry:5000/app` has a colon that isn't a tag.
@@solution
deployments = [
    {"name": "web", "replicas": 3, "strategy": {"type": "RollingUpdate", "maxUnavailable": 1},
     "containers": [{"name": "web", "image": "registry.example.com/web:1.8.2",
                     "resources": {"requests": {"cpu": "250m", "memory": "256Mi"}, "limits": {"memory": "512Mi"}},
                     "readinessProbe": {"httpGet": "/ready"}, "livenessProbe": {"httpGet": "/healthz"}}]},
    {"name": "worker", "replicas": 1, "strategy": {"type": "RollingUpdate", "maxUnavailable": 1},
     "containers": [{"name": "worker", "image": "registry.example.com/worker:latest",
                     "resources": {"requests": {"cpu": "100m"}},
                     "livenessProbe": {"exec": "check"}}]},
    {"name": "api", "replicas": 2, "strategy": {"type": "RollingUpdate", "maxUnavailable": 2},
     "containers": [{"name": "api", "image": "registry.example.com/api",
                     "resources": {}, "readinessProbe": {"httpGet": "/ready"}},
                    {"name": "proxy", "image": "envoyproxy/envoy:v1.31.0",
                     "resources": {"requests": {"cpu": "50m"}, "limits": {"memory": "128Mi"}},
                     "readinessProbe": {"tcpSocket": 9901}, "livenessProbe": {"tcpSocket": 9901}}]},
]

def image_tag(image):
    last = image.rsplit("/", 1)[-1]
    return last.split(":", 1)[1] if ":" in last else None

def review(deploy):
    problems = []
    if deploy["replicas"] < 2:
        problems.append(f"only {deploy['replicas']} replica")
    strategy = deploy.get("strategy", {})
    if strategy.get("type") == "RollingUpdate" and strategy.get("maxUnavailable", 0) >= deploy["replicas"]:
        problems.append("maxUnavailable can take every Pod down")
    for c in deploy["containers"]:
        tag = image_tag(c["image"])
        if tag in (None, "latest"):
            problems.append(f"{c['name']}: pin the image tag")
        res = c.get("resources", {})
        if "requests" not in res:
            problems.append(f"{c['name']}: no resource requests")
        if "memory" not in res.get("limits", {}):
            problems.append(f"{c['name']}: no memory limit")
        for probe in ("readinessProbe", "livenessProbe"):
            if probe not in c:
                problems.append(f"{c['name']}: no {probe}")
    return problems

for d in deployments:
    problems = review(d)
    print(d["name"] + ":")
    for p in problems or ["ready for production"]:
        print("  " + p)
