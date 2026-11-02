@@lesson ck3-exam-tasks CKA Exam Tasks, Hands On
@@concepts maps, strings, automation
@@lang python
@@text
The CKA is two hours of **real tasks on real clusters**, about 15–20 of them, and you need 66%. Nobody asks you to define a Pod; they ask you to *fix* a broken node, *back up* etcd, *let* one namespace talk to another. This lesson walks through the task types that come up again and again, with the fastest commands for each, then the exercises rebuild the rules Kubernetes applies (resource units, label selectors, RBAC and NetworkPolicy) so you can predict what will happen before you apply.

Practise on a local cluster: **kind** (`kind create cluster --config` with one control plane and two workers) or **minikube**, and for the kubeadm tasks two or three VMs (Multipass, Vagrant). Killer.sh, included with the exam, is the closest simulator.

## Exam technique

```text
alias k=kubectl                                  # already set on the exam
export do="--dry-run=client -o yaml"             # k create deploy web --image=nginx $do > web.yaml
export now="--force --grace-period 0"            # k delete pod x $now
k config use-context <ctx>                       # FIRST, in every task: each one names its cluster
k explain pod.spec.containers.livenessProbe --recursive | less
```

- Generate YAML with `kubectl create ... $do` or `kubectl run`, never type it from scratch.
- Read the **whole task**: namespace, names and file paths must be exact.
- Skip anything that takes more than ~7 minutes; flag it and come back.
- The docs (kubernetes.io/docs) are allowed: bookmark the pages for NetworkPolicy, PV/PVC, etcd backup and kubeadm upgrade.

## Task: scale, update and roll back a Deployment

```text
k -n shop create deploy web --image=nginx:1.27 --replicas=3
k -n shop set image deploy/web nginx=nginx:1.28
k -n shop rollout status deploy/web
k -n shop rollout history deploy/web
k -n shop rollout undo deploy/web --to-revision=1
k -n shop scale deploy/web --replicas=5
k -n shop autoscale deploy/web --min=2 --max=8 --cpu-percent=70      # needs metrics-server
```

## Task: schedule on specific nodes

```text
k label node worker-2 disk=ssd
k taint node worker-1 dedicated=gpu:NoSchedule
```

```yaml
spec:
  nodeSelector: { disk: ssd }                     # simplest
  tolerations:
  - { key: dedicated, operator: Equal, value: gpu, effect: NoSchedule }
  affinity:
    podAntiAffinity:                              # spread replicas across nodes
      requiredDuringSchedulingIgnoredDuringExecution:
      - labelSelector: { matchLabels: { app: web } }
        topologyKey: kubernetes.io/hostname
```

A pod stuck **Pending**: `k describe pod` → Events tell you why (insufficient cpu, untolerated taint, no matching node, unbound PVC). The scheduler compares **requests**, not usage, against what's left on each node; the first exercise does that arithmetic.

## Task: a Service and Ingress or Gateway

```text
k -n shop expose deploy web --port=80 --target-port=8080 --name=web-svc
k -n shop get endpointslices -l kubernetes.io/service-name=web-svc   # empty = selector mismatch
k -n shop run tmp --rm -it --image=busybox:1.36 -- wget -qO- web-svc
```

A Service with no endpoints almost always means the **selector doesn't match** the pod labels (the second exercise), or the pods aren't Ready. Recent exams also use the **Gateway API** (`Gateway` + `HTTPRoute`) next to Ingress.

## Task: RBAC

```text
k -n shop create serviceaccount deployer
k -n shop create role deploy-role --verb=get,list,create,update --resource=deployments,pods
k -n shop create rolebinding deploy-rb --role=deploy-role --serviceaccount=shop:deployer
k -n shop auth can-i create deployments --as=system:serviceaccount:shop:deployer   # yes
k -n shop auth can-i delete pods --as=system:serviceaccount:shop:deployer          # no
```

**Role/RoleBinding** are namespaced; **ClusterRole/ClusterRoleBinding** cover the whole cluster; a RoleBinding can also point at a ClusterRole to reuse it in one namespace. Permissions only add up: there are no deny rules. The third exercise evaluates `can-i`.

## Task: NetworkPolicy

```yaml
apiVersion: networking.k8s.io/v1
kind: NetworkPolicy
metadata: { name: db-from-api, namespace: shop }
spec:
  podSelector: { matchLabels: { app: db } }       # the pods this protects
  policyTypes: [Ingress]
  ingress:
  - from:
    - podSelector: { matchLabels: { app: api } }  # same namespace
    - namespaceSelector: { matchLabels: { team: ops } }
    ports: [{ protocol: TCP, port: 5432 }]
```

Pods that **no** policy selects allow everything. Once any policy selects a pod for Ingress, only what some policy allows gets in. Items in the `from` list are OR-ed; a podSelector and namespaceSelector **in the same item** are AND-ed. The fourth exercise implements these rules. Remember: policies need a CNI that enforces them (Calico, Cilium); kind's default one doesn't.

## Task: storage

```text
k get sc                                         # which StorageClass is (default)?
```

```yaml
apiVersion: v1
kind: PersistentVolumeClaim
metadata: { name: data, namespace: shop }
spec:
  accessModes: [ReadWriteOnce]
  storageClassName: standard
  resources: { requests: { storage: 2Gi } }
```

Mount it with `volumes: [{name: data, persistentVolumeClaim: {claimName: data}}]` and a `volumeMounts` entry. A PVC stuck **Pending**: no matching PV (size, access mode, class) or no provisioner. Reclaim policy `Retain` keeps the data after the claim is deleted.

## Task: back up and restore etcd

```text
ETCDCTL_API=3 etcdctl snapshot save /opt/backup/etcd.db \
  --endpoints=https://127.0.0.1:2379 \
  --cacert=/etc/kubernetes/pki/etcd/ca.crt \
  --cert=/etc/kubernetes/pki/etcd/server.crt \
  --key=/etc/kubernetes/pki/etcd/server.key
etcdutl snapshot status /opt/backup/etcd.db -w table

etcdutl snapshot restore /opt/backup/etcd.db --data-dir /var/lib/etcd-restored
# then edit /etc/kubernetes/manifests/etcd.yaml: the etcd-data hostPath → /var/lib/etcd-restored
```

The certificate paths are in `/etc/kubernetes/manifests/etcd.yaml`; `grep` them rather than memorising.

## Task: upgrade a cluster with kubeadm

One minor version at a time, control plane first, then each worker:

```text
# control plane
apt-mark unhold kubeadm && apt-get update && apt-get install -y kubeadm='1.33.x-*' && apt-mark hold kubeadm
kubeadm upgrade plan
kubeadm upgrade apply v1.33.x
k drain cp-1 --ignore-daemonsets
apt-get install -y kubelet='1.33.x-*' kubectl='1.33.x-*' && systemctl daemon-reload && systemctl restart kubelet
k uncordon cp-1
# each worker: install kubeadm, then "kubeadm upgrade node", drain, kubelet, uncordon
```

## Task: fix a broken node or cluster

```text
k get nodes                                      # NotReady?
ssh worker-1
systemctl status kubelet                         # stopped? failed?
journalctl -u kubelet -e                         # the reason: wrong config path, bad cert, swap on...
cat /var/lib/kubelet/config.yaml
crictl ps -a                                     # containers on the node (no docker)
ls /etc/kubernetes/manifests                     # static pods: apiserver, scheduler, controller-manager, etcd
```

If `kubectl` itself fails, the API server static pod is broken: check its manifest for a typo, and `crictl logs` its container. Pods not starting: `k describe` events, `k logs --previous`, wrong image, failing probe, missing ConfigMap or Secret.

## Other things to have done once

- Static pods (a manifest in `/etc/kubernetes/manifests` on one node).
- A **multi-container** pod with a sidecar sharing an `emptyDir` volume; an **init container**.
- **ConfigMaps and Secrets** as env vars and as files.
- **Helm**: `helm repo add`, `helm install --namespace x --create-namespace --set key=value`, `helm upgrade`, `helm list -A`.
- **Kustomize**: `k apply -k dir/`.
- **CRDs and operators**: `k get crd`, describing a custom resource.
- `k top nodes/pods`, `k get events -A --sort-by=.lastTimestamp`, `k -n kube-system get pods`, CoreDNS problems.

@@quiz
A Service has no endpoints. What's the first thing to check?
- The node's kernel version
* Whether the Service's selector matches the pods' labels, and whether those pods are Ready
- The etcd snapshot
- The Ingress class
? `k get pods --show-labels` next to `k describe svc` shows mismatches at once.

@@quiz
In which order do you upgrade a kubeadm cluster?
- Workers first, then the control plane, skipping minor versions if needed
* Control plane first (kubeadm, then upgrade apply, then kubelet), then each worker, one minor version at a time
- All nodes at once with apt upgrade
- Only kubelet; kubeadm upgrades itself
? Workers' kubelets may lag the API server, never lead it.

@@quiz
A NetworkPolicy's `from` has one item containing both a podSelector and a namespaceSelector. What does it allow?
- Pods matching the podSelector in any namespace, OR any pod in matching namespaces
* Only pods that match the podSelector AND live in a namespace matching the namespaceSelector
- Nothing; that's invalid
- Everything
? Separate list items (two dashes) would be OR.

@@quiz
`kubectl` returns "connection refused" on the control plane node. Where do you look?
- The Ingress controller logs
* The kube-apiserver static pod: its manifest in /etc/kubernetes/manifests, and crictl ps -a / crictl logs
- The Deployment's rollout history
- CoreDNS
? Static pods are run by the kubelet directly, so they don't need the API server to start.

@@quiz
Why is a pod Pending with "0/3 nodes are available: insufficient cpu" when the nodes' CPU usage is low?
* The scheduler compares the pod's CPU requests with the nodes' unrequested allocatable CPU, not actual usage
- metrics-server is broken
- CPU limits are too high
- The pod has no image
? Over-generous requests on other pods use up schedulable capacity.

@@flashcards
k config use-context :: first command in every exam task
--dry-run=client -o yaml :: generate a manifest to edit
k explain ... --recursive :: field reference offline
rollout undo --to-revision :: roll back a Deployment
taint + toleration :: keep pods off nodes unless they tolerate it
podAntiAffinity on hostname :: spread replicas across nodes
endpointslices :: the pods a Service actually sends to
auth can-i --as :: test someone's permissions
RoleBinding to a ClusterRole :: reuse a cluster-wide role in one namespace
NetworkPolicy default :: unselected pods allow all
from items OR, selectors in one item AND :: NetworkPolicy matching
etcdctl snapshot save with --cacert --cert --key :: back up etcd
etcdutl snapshot restore --data-dir :: restore, then point the etcd manifest at it
kubeadm upgrade plan / apply / node :: upgrade steps
drain --ignore-daemonsets / uncordon :: take a node out and back
journalctl -u kubelet :: why a node is NotReady
crictl ps -a / crictl logs :: containers on a node without docker
/etc/kubernetes/manifests :: static pod manifests

@@exercise Will the pod fit?
Write `cpu_millis(q)` (`"500m"` → 500, `"2"` → 2000, `"0.25"` → 250) and `mem_bytes(q)` (`"128Mi"` → 134217728, `"1Gi"`, `"512M"` → 512000000, `"1G"`, plain numbers are bytes; binary suffixes `Ki Mi Gi` use 1024, decimal `k M G` use 1000). Then `fits(node, pods, new_pod)`: `node` has `allocatable` cpu and memory, each pod has `requests`; return `True` if the new pod's requests fit in what the existing pods haven't requested.
@@starter
def cpu_millis(q):
    pass


def mem_bytes(q):
    pass


def fits(node, pods, new_pod):
    pass


node = {"allocatable": {"cpu": "2", "memory": "4Gi"}}
pods = [{"requests": {"cpu": "500m", "memory": "1Gi"}}, {"requests": {"cpu": "1", "memory": "512Mi"}}]
print(fits(node, pods, {"requests": {"cpu": "400m", "memory": "1Gi"}}))
print(fits(node, pods, {"requests": {"cpu": "600m", "memory": "256Mi"}}))
@@tests
check([cpu_millis(q) for q in ("500m", "2", "0.25", "1500m")], [500, 2000, 250, 1500], "cpu")
check([mem_bytes(q) for q in ("128Mi", "1Gi", "512M", "1G", "1024", "64Ki")], [134217728, 1073741824, 512000000, 1000000000, 1024, 65536], "memory")
node = {"allocatable": {"cpu": "2", "memory": "4Gi"}}
pods = [{"requests": {"cpu": "500m", "memory": "1Gi"}}, {"requests": {"cpu": "1", "memory": "512Mi"}}]
check(fits(node, pods, {"requests": {"cpu": "400m", "memory": "1Gi"}}), True, "fits")
check(fits(node, pods, {"requests": {"cpu": "600m", "memory": "256Mi"}}), False, "too much cpu")
check(fits(node, pods, {"requests": {"cpu": "500m", "memory": "2560Mi"}}), True, "exactly full")
check(fits(node, pods, {"requests": {"cpu": "100m", "memory": "3Gi"}}), False, "too much memory")
@@hint
For cpu: if it ends with `m`, `int(q[:-1])`, else `round(float(q) * 1000)`. For memory, check two-letter binary suffixes before one-letter decimal ones with a dict of multipliers. Sum the requests with a generator.
@@solution
def cpu_millis(q):
    if q.endswith("m"):
        return int(q[:-1])
    return round(float(q) * 1000)


UNITS = {"Ki": 1024, "Mi": 1024 ** 2, "Gi": 1024 ** 3, "k": 1000, "M": 1000 ** 2, "G": 1000 ** 3}


def mem_bytes(q):
    for suffix in ("Ki", "Mi", "Gi", "k", "M", "G"):
        if q.endswith(suffix):
            return int(float(q[:-len(suffix)]) * UNITS[suffix])
    return int(q)


def fits(node, pods, new_pod):
    free_cpu = cpu_millis(node["allocatable"]["cpu"]) - sum(cpu_millis(p["requests"]["cpu"]) for p in pods)
    free_mem = mem_bytes(node["allocatable"]["memory"]) - sum(mem_bytes(p["requests"]["memory"]) for p in pods)
    want = new_pod["requests"]
    return cpu_millis(want["cpu"]) <= free_cpu and mem_bytes(want["memory"]) <= free_mem


node = {"allocatable": {"cpu": "2", "memory": "4Gi"}}
pods = [{"requests": {"cpu": "500m", "memory": "1Gi"}}, {"requests": {"cpu": "1", "memory": "512Mi"}}]
print(fits(node, pods, {"requests": {"cpu": "400m", "memory": "1Gi"}}))
print(fits(node, pods, {"requests": {"cpu": "600m", "memory": "256Mi"}}))

@@exercise Label selectors and endpoints
A selector is a dict that may have `matchLabels` (every key must equal) and `matchExpressions` (a list of `{"key", "operator", "values"}` with operators `In`, `NotIn`, `Exists`, `DoesNotExist`). Write `matches(selector, labels)`, then `endpoints(service_selector, pods)`: a Service selector is a plain `matchLabels`-style dict; return the names of pods that match **and** have `ready` true, in order. An empty selector matches everything.
@@starter
def matches(selector, labels):
    pass


def endpoints(service_selector, pods):
    pass


pods = [
    {"name": "web-1", "labels": {"app": "web", "tier": "frontend"}, "ready": True},
    {"name": "web-2", "labels": {"app": "web", "tier": "frontend"}, "ready": False},
    {"name": "api-1", "labels": {"app": "api"}, "ready": True},
]
print(endpoints({"app": "web"}, pods))
print(endpoints({"app": "Web"}, pods), "<- why a Service has no endpoints")
@@tests
labels = {"app": "web", "env": "prod"}
check(matches({"matchLabels": {"app": "web"}}, labels), True, "matchLabels")
check(matches({"matchLabels": {"app": "web", "env": "dev"}}, labels), False, "every label must match")
check(matches({"matchExpressions": [{"key": "env", "operator": "In", "values": ["prod", "staging"]}]}, labels), True, "In")
check(matches({"matchExpressions": [{"key": "env", "operator": "NotIn", "values": ["prod"]}]}, labels), False, "NotIn")
check(matches({"matchExpressions": [{"key": "tier", "operator": "NotIn", "values": ["db"]}]}, labels), True, "NotIn with the key missing")
check(matches({"matchExpressions": [{"key": "tier", "operator": "Exists"}]}, labels), False, "Exists")
check(matches({"matchExpressions": [{"key": "tier", "operator": "DoesNotExist"}]}, labels), True, "DoesNotExist")
check(matches({}, labels), True, "empty selector")
pods = [
    {"name": "web-1", "labels": {"app": "web", "tier": "frontend"}, "ready": True},
    {"name": "web-2", "labels": {"app": "web", "tier": "frontend"}, "ready": False},
    {"name": "api-1", "labels": {"app": "api"}, "ready": True},
]
check(endpoints({"app": "web"}, pods), ["web-1"], "ready pods only")
check(endpoints({"app": "Web"}, pods), [], "labels are case-sensitive")
@@hint
Check each `matchLabels` pair with `labels.get(k) == v`. For expressions: `In` needs the key present and its value in `values`; `NotIn` is true when the key is missing or its value isn't in `values`. For `endpoints`, wrap the service selector as `{"matchLabels": sel}`.
@@solution
def matches(selector, labels):
    for k, v in selector.get("matchLabels", {}).items():
        if labels.get(k) != v:
            return False
    for e in selector.get("matchExpressions", []):
        key, op = e["key"], e["operator"]
        if op == "In" and not (key in labels and labels[key] in e["values"]):
            return False
        if op == "NotIn" and key in labels and labels[key] in e["values"]:
            return False
        if op == "Exists" and key not in labels:
            return False
        if op == "DoesNotExist" and key in labels:
            return False
    return True


def endpoints(service_selector, pods):
    sel = {"matchLabels": service_selector}
    return [p["name"] for p in pods if p["ready"] and matches(sel, p["labels"])]


pods = [
    {"name": "web-1", "labels": {"app": "web", "tier": "frontend"}, "ready": True},
    {"name": "web-2", "labels": {"app": "web", "tier": "frontend"}, "ready": False},
    {"name": "api-1", "labels": {"app": "api"}, "ready": True},
]
print(endpoints({"app": "web"}, pods))
print(endpoints({"app": "Web"}, pods), "<- why a Service has no endpoints")

@@exercise kubectl auth can-i
Roles are dicts `{"kind": "Role" or "ClusterRole", "name", "namespace" (Roles only), "rules": [{"verbs", "resources"}]}` where `"*"` matches anything. Bindings are `{"kind": "RoleBinding" or "ClusterRoleBinding", "namespace" (RoleBindings only), "roleRef": name, "roleKind", "subjects": [...]}`. Write `can_i(subject, verb, resource, namespace, roles, bindings)`: a ClusterRoleBinding grants its ClusterRole everywhere; a RoleBinding grants its Role or ClusterRole only in the binding's namespace.
@@starter
def can_i(subject, verb, resource, namespace, roles, bindings):
    pass


SA = "system:serviceaccount:shop:deployer"
roles = [
    {"kind": "Role", "name": "deploy-role", "namespace": "shop", "rules": [{"verbs": ["get", "list", "create", "update"], "resources": ["deployments", "pods"]}]},
    {"kind": "ClusterRole", "name": "view-nodes", "rules": [{"verbs": ["get", "list"], "resources": ["nodes"]}]},
    {"kind": "ClusterRole", "name": "secret-reader", "rules": [{"verbs": ["get"], "resources": ["secrets"]}]},
]
bindings = [
    {"kind": "RoleBinding", "namespace": "shop", "roleRef": "deploy-role", "roleKind": "Role", "subjects": [SA]},
    {"kind": "ClusterRoleBinding", "roleRef": "view-nodes", "roleKind": "ClusterRole", "subjects": [SA]},
    {"kind": "RoleBinding", "namespace": "shop", "roleRef": "secret-reader", "roleKind": "ClusterRole", "subjects": [SA]},
]
for verb, res, ns in [("create", "deployments", "shop"), ("delete", "pods", "shop"), ("get", "secrets", "kube-system")]:
    print(verb, res, ns, "->", "yes" if can_i(SA, verb, res, ns, roles, bindings) else "no")
@@tests
SA = "system:serviceaccount:shop:deployer"
roles = [
    {"kind": "Role", "name": "deploy-role", "namespace": "shop", "rules": [{"verbs": ["get", "list", "create", "update"], "resources": ["deployments", "pods"]}]},
    {"kind": "ClusterRole", "name": "view-nodes", "rules": [{"verbs": ["get", "list"], "resources": ["nodes"]}]},
    {"kind": "ClusterRole", "name": "secret-reader", "rules": [{"verbs": ["get"], "resources": ["secrets"]}]},
    {"kind": "ClusterRole", "name": "admin-all", "rules": [{"verbs": ["*"], "resources": ["*"]}]},
]
bindings = [
    {"kind": "RoleBinding", "namespace": "shop", "roleRef": "deploy-role", "roleKind": "Role", "subjects": [SA]},
    {"kind": "ClusterRoleBinding", "roleRef": "view-nodes", "roleKind": "ClusterRole", "subjects": [SA]},
    {"kind": "RoleBinding", "namespace": "shop", "roleRef": "secret-reader", "roleKind": "ClusterRole", "subjects": [SA]},
    {"kind": "ClusterRoleBinding", "roleRef": "admin-all", "roleKind": "ClusterRole", "subjects": ["aiko"]},
]
check(can_i(SA, "create", "deployments", "shop", roles, bindings), True, "the Role in its namespace")
check(can_i(SA, "create", "deployments", "default", roles, bindings), False, "not in another namespace")
check(can_i(SA, "delete", "pods", "shop", roles, bindings), False, "verb not granted")
check(can_i(SA, "list", "nodes", "anywhere", roles, bindings), True, "ClusterRoleBinding works everywhere")
check(can_i(SA, "get", "secrets", "shop", roles, bindings), True, "RoleBinding to a ClusterRole, in shop")
check(can_i(SA, "get", "secrets", "kube-system", roles, bindings), False, "but only in shop")
check(can_i("aiko", "delete", "nodes", "x", roles, bindings), True, "wildcards")
check(can_i("ken", "get", "pods", "shop", roles, bindings), False, "nobody bound ken")
@@hint
Loop over bindings that include the subject. A RoleBinding only counts when its namespace equals the requested one. Find the role with that name and kind (a Role must also be in the binding's namespace), then check whether any rule has the verb (or `*`) and the resource (or `*`).
@@solution
def can_i(subject, verb, resource, namespace, roles, bindings):
    for b in bindings:
        if subject not in b["subjects"]:
            continue
        if b["kind"] == "RoleBinding" and b["namespace"] != namespace:
            continue
        for r in roles:
            if r["name"] != b["roleRef"] or r["kind"] != b["roleKind"]:
                continue
            if r["kind"] == "Role" and r.get("namespace") != b.get("namespace"):
                continue
            for rule in r["rules"]:
                if (verb in rule["verbs"] or "*" in rule["verbs"]) and (resource in rule["resources"] or "*" in rule["resources"]):
                    return True
    return False


SA = "system:serviceaccount:shop:deployer"
roles = [
    {"kind": "Role", "name": "deploy-role", "namespace": "shop", "rules": [{"verbs": ["get", "list", "create", "update"], "resources": ["deployments", "pods"]}]},
    {"kind": "ClusterRole", "name": "view-nodes", "rules": [{"verbs": ["get", "list"], "resources": ["nodes"]}]},
    {"kind": "ClusterRole", "name": "secret-reader", "rules": [{"verbs": ["get"], "resources": ["secrets"]}]},
]
bindings = [
    {"kind": "RoleBinding", "namespace": "shop", "roleRef": "deploy-role", "roleKind": "Role", "subjects": [SA]},
    {"kind": "ClusterRoleBinding", "roleRef": "view-nodes", "roleKind": "ClusterRole", "subjects": [SA]},
    {"kind": "RoleBinding", "namespace": "shop", "roleRef": "secret-reader", "roleKind": "ClusterRole", "subjects": [SA]},
]
for verb, res, ns in [("create", "deployments", "shop"), ("delete", "pods", "shop"), ("get", "secrets", "kube-system")]:
    print(verb, res, ns, "->", "yes" if can_i(SA, verb, res, ns, roles, bindings) else "no")

@@exercise NetworkPolicy: is this connection allowed?
Pods are `{"name", "namespace", "labels"}`; namespaces are a dict of name → labels. A policy is `{"namespace", "podSelector" (matchLabels dict; empty selects every pod in the namespace), "ingress": [{"from": [peer, ...]}]}` and each peer may have `podSelector` and/or `namespaceSelector` (matchLabels dicts). Write `allowed(src, dst, policies, namespaces)`: if no policy in `dst`'s namespace selects `dst`, allow. Otherwise allow only if some selecting policy has a rule with a peer matching `src`. A peer with only `podSelector` means pods in the **policy's** namespace; only `namespaceSelector` means any pod in matching namespaces; both means both must match. A rule with an empty `from` list allows everyone.
@@starter
def selects(match_labels, labels):
    return all(labels.get(k) == v for k, v in match_labels.items())


def allowed(src, dst, policies, namespaces):
    pass


namespaces = {"shop": {"team": "shop"}, "ops": {"team": "ops"}, "dev": {"team": "dev"}}
db = {"name": "db", "namespace": "shop", "labels": {"app": "db"}}
api = {"name": "api", "namespace": "shop", "labels": {"app": "api"}}
web = {"name": "web", "namespace": "shop", "labels": {"app": "web"}}
policy = {"namespace": "shop", "podSelector": {"app": "db"},
          "ingress": [{"from": [{"podSelector": {"app": "api"}}, {"namespaceSelector": {"team": "ops"}}]}]}
print(allowed(api, db, [policy], namespaces), allowed(web, db, [policy], namespaces))
@@tests
namespaces = {"shop": {"team": "shop"}, "ops": {"team": "ops"}, "dev": {"team": "dev"}}
db = {"name": "db", "namespace": "shop", "labels": {"app": "db"}}
api = {"name": "api", "namespace": "shop", "labels": {"app": "api"}}
web = {"name": "web", "namespace": "shop", "labels": {"app": "web"}}
monitor = {"name": "prom", "namespace": "ops", "labels": {"app": "prometheus"}}
dev_api = {"name": "api", "namespace": "dev", "labels": {"app": "api"}}
policy = {"namespace": "shop", "podSelector": {"app": "db"},
          "ingress": [{"from": [{"podSelector": {"app": "api"}}, {"namespaceSelector": {"team": "ops"}}]}]}
check(allowed(api, db, [policy], namespaces), True, "api in the same namespace")
check(allowed(web, db, [policy], namespaces), False, "web is not allowed")
check(allowed(monitor, db, [policy], namespaces), True, "anything from ops")
check(allowed(dev_api, db, [policy], namespaces), False, "podSelector alone means the policy's namespace")
check(allowed(web, api, [policy], namespaces), True, "api isn't selected by any policy")
deny_all = {"namespace": "shop", "podSelector": {}, "ingress": []}
check(allowed(api, web, [deny_all], namespaces), False, "default deny")
both = {"namespace": "shop", "podSelector": {"app": "db"},
        "ingress": [{"from": [{"podSelector": {"app": "prometheus"}, "namespaceSelector": {"team": "ops"}}]}]}
check(allowed(monitor, db, [both], namespaces), True, "AND: prometheus in ops")
check(allowed({"name": "x", "namespace": "ops", "labels": {"app": "other"}}, db, [both], namespaces), False, "AND: wrong pod in ops")
open_rule = {"namespace": "shop", "podSelector": {"app": "db"}, "ingress": [{"from": []}]}
check(allowed(dev_api, db, [open_rule], namespaces), True, "empty from allows everyone")
@@hint
First collect the policies with `p["namespace"] == dst["namespace"]` and `selects(p["podSelector"], dst["labels"])`; none → `True`. For a peer: the namespace matches if it has a `namespaceSelector` that selects `namespaces[src["namespace"]]`, or, when it has none, if `src` is in the policy's namespace. The pod matches if there's no `podSelector` or it selects `src["labels"]`. Both must be true.
@@solution
def selects(match_labels, labels):
    return all(labels.get(k) == v for k, v in match_labels.items())


def peer_matches(peer, src, policy, namespaces):
    if "namespaceSelector" in peer:
        ns_ok = selects(peer["namespaceSelector"], namespaces[src["namespace"]])
    else:
        ns_ok = src["namespace"] == policy["namespace"]
    pod_ok = "podSelector" not in peer or selects(peer["podSelector"], src["labels"])
    return ns_ok and pod_ok


def allowed(src, dst, policies, namespaces):
    selecting = [p for p in policies
                 if p["namespace"] == dst["namespace"] and selects(p["podSelector"], dst["labels"])]
    if not selecting:
        return True
    for p in selecting:
        for rule in p["ingress"]:
            if not rule["from"]:
                return True
            if any(peer_matches(peer, src, p, namespaces) for peer in rule["from"]):
                return True
    return False


namespaces = {"shop": {"team": "shop"}, "ops": {"team": "ops"}, "dev": {"team": "dev"}}
db = {"name": "db", "namespace": "shop", "labels": {"app": "db"}}
api = {"name": "api", "namespace": "shop", "labels": {"app": "api"}}
web = {"name": "web", "namespace": "shop", "labels": {"app": "web"}}
policy = {"namespace": "shop", "podSelector": {"app": "db"},
          "ingress": [{"from": [{"podSelector": {"app": "api"}}, {"namespaceSelector": {"team": "ops"}}]}]}
print(allowed(api, db, [policy], namespaces), allowed(web, db, [policy], namespaces))
