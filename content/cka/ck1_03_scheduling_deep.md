@@lesson ck1-scheduling-deep Scheduling in Depth: Selectors, Affinity, Taints, Spread and Priority
@@text
When a Pod is stuck in Pending, the answer is almost always in the scheduler's rules. This lesson explains how kube-scheduler filters and scores nodes, and how you steer Pods with nodeSelector, node affinity, Pod (anti-)affinity, taints and tolerations, topology spread constraints and priority classes, plus static Pods and manual scheduling.

## How the scheduler decides

1. **Filter** out nodes that can't run the Pod: not enough allocatable CPU/memory for the **requests**, nodeSelector/affinity mismatch, **untolerated taints**, port conflicts, volume zone constraints, node unschedulable (cordoned).
2. **Score** the remaining nodes (spread, balanced resources, preferred affinity, image locality) and pick the highest.
3. **Bind** the Pod to the node. If no node passes filtering, the Pod stays **Pending** with an event such as `0/3 nodes are available: 1 node(s) had untolerated taint {node-role.kubernetes.io/control-plane: }, 2 Insufficient memory.`

## nodeSelector and node affinity

```yaml,norun
spec:
  nodeSelector:
    disktype: ssd
  affinity:
    nodeAffinity:
      requiredDuringSchedulingIgnoredDuringExecution:
        nodeSelectorTerms:
        - matchExpressions:
          - key: topology.kubernetes.io/zone
            operator: In
            values: [eu-west-1a, eu-west-1b]
      preferredDuringSchedulingIgnoredDuringExecution:
      - weight: 50
        preference:
          matchExpressions:
          - key: gpu
            operator: Exists
```

Operators: `In`, `NotIn`, `Exists`, `DoesNotExist`, `Gt`, `Lt`. **Required** = hard filter; **preferred** = score bonus. *IgnoredDuringExecution*: changing node labels later doesn't evict running Pods. Label nodes with `kubectl label node node1 disktype=ssd`.

## Pod affinity and anti-affinity

Place Pods relative to **other Pods** within a **topologyKey** (node `kubernetes.io/hostname`, zone `topology.kubernetes.io/zone`): co-locate a cache with its app (affinity) or spread replicas so no two share a node (required anti-affinity).

## Taints and tolerations

Taints **repel** Pods from nodes; tolerations **allow** (but don't force) a Pod onto tainted nodes.

```bash,norun
kubectl taint nodes node3 dedicated=gpu:NoSchedule
kubectl taint nodes node3 dedicated=gpu:NoSchedule-     # remove
```

| Effect | New Pods without toleration | Running Pods without toleration |
|---|---|---|
| **NoSchedule** | not scheduled | stay |
| **PreferNoSchedule** | avoided if possible | stay |
| **NoExecute** | not scheduled | **evicted** (after `tolerationSeconds` if set) |

```yaml,norun
tolerations:
- key: dedicated
  operator: Equal        # or Exists (matches any value)
  value: gpu
  effect: NoSchedule
```

Control plane nodes carry `node-role.kubernetes.io/control-plane:NoSchedule`. Nodes get automatic NoExecute taints when `not-ready` or `unreachable`. **Dedicated nodes** = taint (keep others off) **plus** node affinity (keep these Pods on).

## Topology spread constraints

```yaml,norun
topologySpreadConstraints:
- maxSkew: 1
  topologyKey: topology.kubernetes.io/zone
  whenUnsatisfiable: DoNotSchedule     # or ScheduleAnyway
  labelSelector:
    matchLabels: {app: web}
```

**Skew** = Pods in the fullest domain − Pods in the emptiest. A Pod can only go where the resulting skew stays ≤ `maxSkew`.

## Priority and preemption

A **PriorityClass** (`value`, `globalDefault`, `preemptionPolicy`) gives Pods priority; when a high-priority Pod can't fit, the scheduler **preempts** (evicts) lower-priority Pods. Built-in: `system-cluster-critical`, `system-node-critical`.

## Other placement tools

- **Cordon/drain**: `kubectl cordon node2` (unschedulable), `kubectl drain node2 --ignore-daemonsets --delete-emptydir-data` (evict Pods, respecting **PodDisruptionBudgets**), `kubectl uncordon node2`.
- **Manual scheduling**: set `spec.nodeName` (bypasses the scheduler; useful when the scheduler is down).
- **Static Pods**: manifests in `/etc/kubernetes/manifests` (kubelet `staticPodPath`) run by the kubelet directly; the control plane components are static Pods on kubeadm clusters; their mirror Pods have the node name appended.
- Multiple schedulers: `spec.schedulerName`.

@@quiz
A Pod is Pending with "1 node(s) had untolerated taint". What fixes it if the Pod should run there?
- Add a nodeSelector
* Add a matching toleration to the Pod (or remove the taint)
- Increase replicas
- Add a readiness probe
? Taints repel Pods without tolerations.

@@quiz
Which taint effect evicts already-running Pods that don't tolerate it?
- NoSchedule
- PreferNoSchedule
* NoExecute
- NoEvict
? NoSchedule only affects new Pods.

@@quiz
GPU nodes must run only GPU workloads, and GPU workloads must run only on GPU nodes. What's the design?
- A toleration only
- Node affinity only
* Taint the GPU nodes and give GPU Pods both a toleration and node affinity
- A PriorityClass
? Tolerations allow, they don't attract.

@@quiz
Which field places a Pod on a node without using the scheduler?
- nodeSelector
* spec.nodeName
- schedulerName
- topologyKey
? The kubelet on that node runs it directly.

@@quiz
Where does the kubelet look for static Pod manifests on a kubeadm cluster?
* /etc/kubernetes/manifests
- /var/lib/etcd
- /etc/kubelet.d/pods
- ~/.kube/manifests
? Set by staticPodPath.

@@quiz
Three zones have 3, 2 and 2 Pods of an app with maxSkew 1 and DoNotSchedule. Where can the next Pod go?
- Zone with 3 Pods
* Either zone with 2 Pods
- Anywhere
- Nowhere
? Putting it in the first zone would make the skew 2.

@@flashcards
scheduler phases :: filter, score, bind
filter uses :: requests, not limits
nodeSelector :: exact label match
required affinity :: hard filter
preferred affinity :: score bonus
IgnoredDuringExecution :: no eviction on label change
pod anti-affinity :: spread replicas apart
topologyKey hostname :: per node
taint :: repels Pods
toleration :: allows, doesn't attract
NoSchedule :: blocks new Pods
NoExecute :: evicts running Pods
remove taint :: append a minus
control-plane taint :: node-role.kubernetes.io/control-plane:NoSchedule
maxSkew :: allowed imbalance
PriorityClass :: priority and preemption
cordon :: mark unschedulable
drain :: evict Pods respecting PDBs
spec.nodeName :: bypass the scheduler
static Pods :: /etc/kubernetes/manifests

@@drill
Which command removes the taint dedicated=gpu:NoSchedule from node3?
= kubectl taint nodes node3 dedicated=gpu:NoSchedule-
= kubectl taint node node3 dedicated=gpu:NoSchedule-
? Note the trailing minus.

@@drill
Which kubectl command marks a node unschedulable without evicting Pods?
= kubectl cordon
= cordon
? drain also evicts.

@@exercise Explain why a Pod is Pending
Implement the scheduler's filter phase. For each Pod, check every node: cordoned, nodeSelector labels, untolerated `NoSchedule`/`NoExecute` taints (a toleration matches on key and effect, and on value unless its operator is `Exists`), and free CPU (millicores) and memory (Mi) against the requests. Print the feasible nodes, or a summary like the scheduler's event: `0/N nodes are available: <count> <reason>, ...`.
@@lang python
@@starter
from collections import Counter

nodes = {
    "cp1": {"labels": {"node-role": "control-plane"}, "taints": [("node-role.kubernetes.io/control-plane", "", "NoSchedule")], "free_cpu": 1500, "free_mem": 2048, "cordoned": False},
    "node1": {"labels": {"disktype": "ssd"}, "taints": [], "free_cpu": 500, "free_mem": 1024, "cordoned": False},
    "node2": {"labels": {"disktype": "hdd"}, "taints": [], "free_cpu": 3000, "free_mem": 8192, "cordoned": False},
    "gpu1": {"labels": {"disktype": "ssd", "gpu": "a100"}, "taints": [("dedicated", "gpu", "NoSchedule")], "free_cpu": 8000, "free_mem": 32768, "cordoned": False},
    "node3": {"labels": {"disktype": "ssd"}, "taints": [], "free_cpu": 4000, "free_mem": 8192, "cordoned": True},
}
pods = [
    {"name": "web", "cpu": 250, "mem": 256, "selector": {}, "tolerations": []},
    {"name": "db", "cpu": 1000, "mem": 2048, "selector": {"disktype": "ssd"}, "tolerations": []},
    {"name": "trainer", "cpu": 4000, "mem": 16384, "selector": {"gpu": "a100"}, "tolerations": [("dedicated", "Equal", "gpu", "NoSchedule")]},
    {"name": "monitor", "cpu": 100, "mem": 128, "selector": {}, "tolerations": [("node-role.kubernetes.io/control-plane", "Exists", "", "NoSchedule")]},
]

def tolerated(taint, tolerations):
    raise NotImplementedError
@@expected
web: feasible on node1, node2
db: Pending: 0/5 nodes are available: 1 Insufficient cpu, 2 node(s) didn't match Pod's node affinity/selector, 1 node(s) had untolerated taint {dedicated}, 1 node(s) were unschedulable.
trainer: feasible on gpu1
monitor: feasible on cp1, node1, node2
@@requires
def tolerated
Counter(
"Exists"
@@hint
Return the first failing reason per node (`node(s) were unschedulable`, `node(s) didn't match Pod's node affinity/selector`, `node(s) had untolerated taint {key}`, `Insufficient cpu`, `Insufficient memory`) and count them with a Counter.
@@solution
from collections import Counter

nodes = {
    "cp1": {"labels": {"node-role": "control-plane"}, "taints": [("node-role.kubernetes.io/control-plane", "", "NoSchedule")], "free_cpu": 1500, "free_mem": 2048, "cordoned": False},
    "node1": {"labels": {"disktype": "ssd"}, "taints": [], "free_cpu": 500, "free_mem": 1024, "cordoned": False},
    "node2": {"labels": {"disktype": "hdd"}, "taints": [], "free_cpu": 3000, "free_mem": 8192, "cordoned": False},
    "gpu1": {"labels": {"disktype": "ssd", "gpu": "a100"}, "taints": [("dedicated", "gpu", "NoSchedule")], "free_cpu": 8000, "free_mem": 32768, "cordoned": False},
    "node3": {"labels": {"disktype": "ssd"}, "taints": [], "free_cpu": 4000, "free_mem": 8192, "cordoned": True},
}
pods = [
    {"name": "web", "cpu": 250, "mem": 256, "selector": {}, "tolerations": []},
    {"name": "db", "cpu": 1000, "mem": 2048, "selector": {"disktype": "ssd"}, "tolerations": []},
    {"name": "trainer", "cpu": 4000, "mem": 16384, "selector": {"gpu": "a100"}, "tolerations": [("dedicated", "Equal", "gpu", "NoSchedule")]},
    {"name": "monitor", "cpu": 100, "mem": 128, "selector": {}, "tolerations": [("node-role.kubernetes.io/control-plane", "Exists", "", "NoSchedule")]},
]

def tolerated(taint, tolerations):
    key, value, effect = taint
    for t_key, op, t_value, t_effect in tolerations:
        if t_key == key and t_effect == effect and (op == "Exists" or t_value == value):
            return True
    return False

def reason(pod, node):
    if node["cordoned"]:
        return "node(s) were unschedulable"
    if any(node["labels"].get(k) != v for k, v in pod["selector"].items()):
        return "node(s) didn't match Pod's node affinity/selector"
    for taint in node["taints"]:
        if taint[2] in ("NoSchedule", "NoExecute") and not tolerated(taint, pod["tolerations"]):
            return f"node(s) had untolerated taint {{{taint[0]}}}"
    if pod["cpu"] > node["free_cpu"]:
        return "Insufficient cpu"
    if pod["mem"] > node["free_mem"]:
        return "Insufficient memory"
    return None

for pod in pods:
    reasons = {name: reason(pod, node) for name, node in nodes.items()}
    feasible = [n for n, r in reasons.items() if r is None]
    if feasible:
        print(f"{pod['name']}: feasible on {', '.join(feasible)}")
    else:
        counts = Counter(reasons.values())
        summary = ", ".join(f"{c} {r}" for r, c in sorted(counts.items()))
        print(f"{pod['name']}: Pending: 0/{len(nodes)} nodes are available: {summary}.")

@@exercise Place Pods with a topology spread constraint
Place new replicas of an app one at a time across zones with `maxSkew`. For each new Pod, a zone is allowed if, after adding the Pod there, max − min across all zones ≤ `MAX_SKEW`; among allowed zones pick the one with the fewest Pods (ties by zone name). With `DoNotSchedule`, a Pod with no allowed zone stays Pending; zones in `full` can't take Pods. Print each placement and the final distribution.
@@lang python
@@starter
MAX_SKEW = 1
zones = {"zone-a": 3, "zone-b": 1, "zone-c": 2}
full = {"zone-c"}
new_pods = 4

def allowed(zone, counts):
    raise NotImplementedError
@@expected
pod 1: zone-b -> {'zone-a': 3, 'zone-b': 2, 'zone-c': 2}
pod 2: zone-b -> {'zone-a': 3, 'zone-b': 3, 'zone-c': 2}
pod 3: Pending (no zone keeps skew <= 1; zone-c is full)
pod 4: Pending (no zone keeps skew <= 1; zone-c is full)
final: {'zone-a': 3, 'zone-b': 3, 'zone-c': 2}
@@requires
def allowed
MAX_SKEW
.values()
@@hint
`allowed` copies the counts, adds 1 to the zone and checks `max(c.values()) - min(c.values()) <= MAX_SKEW`. A zone in `full` is never a candidate, but still counts for the skew.
@@solution
MAX_SKEW = 1
zones = {"zone-a": 3, "zone-b": 1, "zone-c": 2}
full = {"zone-c"}
new_pods = 4

def allowed(zone, counts):
    c = dict(counts)
    c[zone] += 1
    return max(c.values()) - min(c.values()) <= MAX_SKEW

counts = dict(zones)
for i in range(1, new_pods + 1):
    candidates = [z for z in sorted(counts) if z not in full and allowed(z, counts)]
    if not candidates:
        print(f"pod {i}: Pending (no zone keeps skew <= {MAX_SKEW}; {', '.join(sorted(full))} is full)")
        continue
    zone = min(candidates, key=lambda z: (counts[z], z))
    counts[zone] += 1
    print(f"pod {i}: {zone} -> {counts}")
print("final:", counts)
