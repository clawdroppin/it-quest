@@lesson ck0-architecture-kubectl CKA: The Exam, Cluster Architecture and kubectl
@@text
## The Certified Kubernetes Administrator exam

The **CKA** (from the CNCF and the Linux Foundation) is **hands-on**: 2 hours in a remote desktop, about 15–20 tasks on real clusters, solved at a terminal. You need 66% to pass. You may open the official documentation (kubernetes.io/docs) in the exam browser, nothing else. One free retake is included.

Domains: **Troubleshooting** (30%), **Cluster Architecture, Installation and Configuration** (25%), **Services and Networking** (20%), **Workloads and Scheduling** (15%), **Storage** (10%).

Because it's practical, **speed with kubectl** is everything. The same goes for the job: Kubernetes skills are in demand at Japanese tech companies (Mercari, LINE Yahoo, CyberAgent, many SIers' cloud teams), and CKA is one of the most recognized proofs of them.

Practice on a real cluster: **kind** or **minikube** on your laptop, or `kubeadm` on two or three VMs (the exam asks about kubeadm). The exam environment comes with **killer.sh** simulator sessions, which are harder than the real thing.

## Architecture

```text
Control plane                                  Each worker node
┌──────────────────────────────────┐           ┌───────────────────────────────┐
│ kube-apiserver  ← everything     │◀─────────▶│ kubelet  (runs the pods)      │
│ etcd            (the state, k/v) │           │ kube-proxy (Service rules)    │
│ kube-scheduler  (picks nodes)    │           │ container runtime (containerd)│
│ kube-controller-manager          │           │ CNI plugin (pod network)      │
└──────────────────────────────────┘           └───────────────────────────────┘
```

- **kube-apiserver**: the only component that talks to etcd. kubectl, controllers and kubelets all go through it.
- **etcd**: a distributed key-value store holding the whole cluster state. Lose it without a backup and you lose the cluster.
- **kube-scheduler**: assigns new pods to nodes (by resources, affinity, taints).
- **kube-controller-manager**: the control loops (Deployments, ReplicaSets, nodes, endpoints...) that make the *actual* state match the *desired* state.
- **kubelet**: the agent on each node; it starts containers through the runtime (via **CRI**) and reports status. It's a systemd service, not a pod: `systemctl status kubelet`, `journalctl -u kubelet`.
- On kubeadm clusters, the control-plane components are **static pods**: manifests in `/etc/kubernetes/manifests/` that the kubelet runs directly. Edit the file and the kubelet restarts the component.

## kubectl at exam speed

```bash,norun
alias k=kubectl                      # already set in the exam
export do="--dry-run=client -o yaml" # generate YAML instead of writing it
k config use-context prod-cluster    # EVERY task says which context: switch first!

k run web --image=nginx:1.27 $do > pod.yaml
k create deployment api --image=ghcr.io/acme/api:2.1 --replicas=3 $do > deploy.yaml
k expose deployment api --port=80 --target-port=8080 --type=ClusterIP
k create configmap app-cfg --from-literal=MODE=prod
k create secret generic db --from-literal=password='s3cret'
k scale deployment api --replicas=5
k set image deployment/api api=ghcr.io/acme/api:2.2
k rollout status deployment/api;  k rollout undo deployment/api

k get pods -A -o wide                # all namespaces, with node and IP
k get pods -l app=api --show-labels
k describe pod web                   # events at the bottom: why it's failing
k logs web -c sidecar --previous     # logs of the crashed previous container
k exec -it web -- sh
k explain pod.spec.containers.livenessProbe   # field docs without the browser
k get pod web -o jsonpath='{.status.podIP}'
k top nodes; k top pods              # needs metrics-server
```

The **imperative** commands plus `--dry-run=client -o yaml` save minutes on every task: generate the skeleton, edit it with `vim`, `k apply -f`.

## Objects you must know cold

| Object | What it does |
|---|---|
| **Pod** | one or more containers sharing a network namespace and volumes |
| **ReplicaSet** | keeps N identical pods running (you rarely create these directly) |
| **Deployment** | rolling updates and rollbacks of ReplicaSets: stateless apps |
| **StatefulSet** | stable names (db-0, db-1) and a volume per pod: databases |
| **DaemonSet** | one pod on every (or selected) node: log agents, CNI, monitoring |
| **Job / CronJob** | run to completion / on a schedule |
| **Service** | a stable IP and DNS name in front of pods chosen by labels |
| **ConfigMap / Secret** | configuration as environment variables or files (Secrets are only base64, not encrypted, unless etcd encryption is configured) |
| **Namespace** | a scope for names, RBAC, quotas |

Labels and **selectors** connect everything: a Service finds pods by labels, a Deployment owns pods by labels. A typo in a label is one of the most common exam (and production) bugs.

@@quiz
Which component is the only one that reads and writes etcd directly?
* kube-apiserver
- kube-scheduler
- kubelet
- kube-controller-manager
? Everything else talks to the API server.

@@quiz
On a kubeadm cluster, where would you change the kube-apiserver's flags?
* In its static pod manifest under /etc/kubernetes/manifests/
- With kubectl edit deployment kube-apiserver
- In /etc/systemd/system/kube-apiserver.service
- In a ConfigMap in the default namespace
? The kubelet watches that folder and recreates the static pod when the file changes.

@@quiz
A node shows NotReady. What's a good first check on the node itself?
* `systemctl status kubelet` and `journalctl -u kubelet`
- `kubectl delete node`
- Restart etcd
- `kubectl rollout restart`
? The kubelet runs as a systemd service; if it's stopped or misconfigured, the node goes NotReady.

@@quiz
Which object runs one copy of a pod on every node?
* DaemonSet
- StatefulSet
- Deployment with replicas equal to the number of nodes
- ReplicaSet
? Deployments don't guarantee one per node; DaemonSets do.

@@quiz
What's the fastest way to create a correct Deployment YAML in the exam?
* `kubectl create deployment ... --dry-run=client -o yaml > file.yaml`, then edit it
- Type it from memory
- Copy it from a blog
- `kubectl get deployment -o yaml` from another cluster
? Imperative commands with dry-run produce valid skeletons in seconds.

@@flashcards
CKA format :: 2 hours, hands-on tasks, 66% to pass, docs allowed
biggest CKA domain :: Troubleshooting (30%)
etcd :: the cluster's state store
kube-scheduler :: assigns pods to nodes
kubelet :: node agent, a systemd service
static pods :: /etc/kubernetes/manifests/
CRI / CNI / CSI :: runtime / network / storage plugin interfaces
--dry-run=client -o yaml :: generate YAML without creating anything
kubectl logs --previous :: logs of the crashed container
kubectl explain :: field documentation in the terminal
StatefulSet :: stable names and per-pod volumes
Secret :: base64, not encrypted by default

@@drill
Which directory holds the static pod manifests on a kubeadm node?
= /etc/kubernetes/manifests
= /etc/kubernetes/manifests/
