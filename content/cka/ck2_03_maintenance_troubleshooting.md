@@lesson ck2-maintenance Cluster Maintenance and Troubleshooting: Upgrades, etcd, Nodes and Pods
@@text
Troubleshooting is the largest CKA domain (**30%**), and cluster maintenance tasks (kubeadm upgrades, etcd backup and restore) are near-certain exam questions. This lesson gives the exact procedures and a systematic way to diagnose broken Pods, nodes, Services and control planes.

## Upgrading a kubeadm cluster

Rules: upgrade **one minor version at a time** (1.33 → 1.34, not 1.32 → 1.34); control plane first, then workers; the kubelet may be up to **three minor versions older** than the API server but **never newer**; kubectl within ±1 minor.

Control plane node:

```bash,norun
# switch the package repository to the new minor version first (pkgs.k8s.io is per minor)
apt-mark unhold kubeadm && apt-get update && apt-get install -y kubeadm=1.34.1-1.1 && apt-mark hold kubeadm
kubeadm upgrade plan
kubeadm upgrade apply v1.34.1            # first control plane node; others: kubeadm upgrade node
kubectl drain cp1 --ignore-daemonsets
apt-mark unhold kubelet kubectl && apt-get install -y kubelet=1.34.1-1.1 kubectl=1.34.1-1.1 && apt-mark hold kubelet kubectl
systemctl daemon-reload && systemctl restart kubelet
kubectl uncordon cp1
```

Each worker: upgrade kubeadm → `kubeadm upgrade node` → drain (from a machine with kubectl) → upgrade kubelet/kubectl → restart kubelet → uncordon.

## etcd backup and restore

```bash,norun
ETCDCTL_API=3 etcdctl snapshot save /opt/backup/etcd.db \
  --endpoints=https://127.0.0.1:2379 \
  --cacert=/etc/kubernetes/pki/etcd/ca.crt \
  --cert=/etc/kubernetes/pki/etcd/server.crt \
  --key=/etc/kubernetes/pki/etcd/server.key
etcdutl snapshot status /opt/backup/etcd.db -w table
etcdutl snapshot restore /opt/backup/etcd.db --data-dir=/var/lib/etcd-restored
# then edit /etc/kubernetes/manifests/etcd.yaml: hostPath of the etcd-data volume -> /var/lib/etcd-restored
```

The certificate paths come from the etcd static Pod manifest (`--cert-file`, `--key-file`, `--trusted-ca-file`). After a restore, the etcd static Pod restarts with the old state; wait for the API server to come back.

## Troubleshooting Pods

| Symptom | Typical cause | Look at |
|---|---|---|
| **Pending** | no node fits (resources, taints, affinity), PVC unbound | `describe pod` events |
| **ImagePullBackOff / ErrImagePull** | wrong image name/tag, private registry without `imagePullSecrets` | events |
| **CrashLoopBackOff** | app exits: bad command, missing config, failing liveness probe | `logs --previous`, exit code |
| **OOMKilled** (exit 137) | memory limit too low | `describe`, last state |
| **CreateContainerConfigError** | missing ConfigMap/Secret key | events |
| **Running but not Ready** | readiness probe failing | `describe`, endpoints |
| **ContainerCreating** stuck | volume mount or CNI problem | events |

Exit codes: **0** success, **1** app error, **126/127** command not executable/not found, **137** SIGKILL (OOM), **143** SIGTERM.

```bash,norun
kubectl get pods -A -o wide
kubectl describe pod web-7d9c -n shop
kubectl logs web-7d9c -c app --previous
kubectl get events -n shop --sort-by=.lastTimestamp
kubectl exec -it web-7d9c -- sh
kubectl debug node/node2 -it --image=busybox     # node shell
```

## Troubleshooting nodes

**NotReady**: check `kubectl describe node` conditions (MemoryPressure, DiskPressure, PIDPressure, NetworkUnavailable), then on the node: `systemctl status kubelet`, `journalctl -u kubelet -f`, container runtime (`systemctl status containerd`, `crictl ps`), certificates, kubelet config (`/var/lib/kubelet/config.yaml`), CNI.

## Troubleshooting the control plane

On kubeadm clusters the components are **static Pods** in `/etc/kubernetes/manifests`. If `kubectl` fails ("connection refused" on 6443), the API server is down: check the manifest for typos, `crictl ps -a | grep kube-apiserver`, `crictl logs <id>`, `/var/log/pods/`. A broken scheduler leaves new Pods Pending with no node; a broken controller manager stops Deployments from creating ReplicaSets/Pods.

## Troubleshooting Services and DNS

Endpoints empty → selector/labels/readiness; wrong `targetPort`; DNS: `nslookup kubernetes.default` from a busybox Pod, CoreDNS Pods running, kube-dns Service; NetworkPolicies blocking; kube-proxy Pods healthy.

## Monitoring

`kubectl top nodes` / `kubectl top pods --containers` need metrics-server.

@@quiz
A cluster runs 1.32. What's the correct upgrade path to 1.34?
- 1.32 → 1.34 directly
* 1.32 → 1.33 → 1.34, control plane before workers
- Workers first, then control plane
- Upgrade kubelets first
? One minor version at a time.

@@quiz
Which component version may never be newer than the kube-apiserver?
- kubectl
* kubelet
- etcd
- CoreDNS
? Kubelets can lag up to three minor versions.

@@quiz
A Pod shows CrashLoopBackOff. What's the most useful first command?
- kubectl get nodes
* kubectl logs <pod> --previous
- kubectl rollout undo
- kubectl delete pod
? See why the last run exited.

@@quiz
A container's last state shows exit code 137 and reason OOMKilled. What should you change?
- The image tag
- The readiness probe
* Raise the memory limit (or fix the memory use)
- Add a toleration
? 137 = SIGKILL.

@@quiz
`kubectl` returns "connection refused" to port 6443 on a kubeadm cluster. Where do you look?
* The kube-apiserver static Pod manifest and its container logs via crictl
- The CoreDNS ConfigMap
- The Deployment's rollout history
- The Ingress controller
? The API server is down; kubectl can't help.

@@quiz
After restoring an etcd snapshot to /var/lib/etcd-restored, what else must you do?
- Nothing
* Point the etcd static Pod's data hostPath at the restored directory
- Run kubeadm init again
- Restart every node
? Otherwise etcd keeps using the old data.

@@flashcards
upgrade order :: control plane, then workers
version skew :: one minor at a time
kubelet skew :: up to 3 minors older, never newer
kubeadm upgrade plan :: show available versions
kubeadm upgrade apply :: first control plane
kubeadm upgrade node :: other nodes
drain flags :: --ignore-daemonsets --delete-emptydir-data
etcd snapshot save :: needs endpoints, cacert, cert, key
etcd restore :: new data dir + update manifest
static Pod manifests :: /etc/kubernetes/manifests
Pending :: scheduling or PVC
ImagePullBackOff :: image name or pull secret
CrashLoopBackOff :: logs --previous
exit 137 :: SIGKILL, often OOM
exit 127 :: command not found
CreateContainerConfigError :: missing ConfigMap/Secret
node NotReady :: kubelet, runtime, CNI
kubelet logs :: journalctl -u kubelet
crictl :: inspect containers on the node
kubectl top :: needs metrics-server

@@drill
Which command shows the logs of the previous (crashed) container instance?
= kubectl logs <pod> --previous
= kubectl logs --previous
= kubectl logs -p
? -p is short for --previous.

@@drill
Which exit code means the container was killed by SIGKILL (often an OOM)?
= 137
? 143 is SIGTERM.

@@exercise Plan a kubeadm upgrade
Given the current versions of each node and a target version, check the plan is legal (target at most one minor version above the control plane) and print the ordered steps: control plane nodes first (the first uses `kubeadm upgrade apply`, others `kubeadm upgrade node`), then workers, each with drain, upgrade, uncordon. Also flag any node whose kubelet is more than three minor versions behind the target or newer than it.
@@lang python
@@starter
nodes = [("cp1", "control-plane", "1.33.4"), ("cp2", "control-plane", "1.33.4"), ("w1", "worker", "1.33.4"), ("w2", "worker", "1.30.2"), ("w3", "worker", "1.33.4")]
targets = ["1.34.1", "1.35.0"]

def minor(version):
    raise NotImplementedError
@@expected
target v1.34.1:
  cp1: drain -> kubeadm upgrade apply v1.34.1 -> kubelet 1.33.4 to 1.34.1 -> uncordon
  cp2: drain -> kubeadm upgrade node -> kubelet 1.33.4 to 1.34.1 -> uncordon
  w1: drain -> kubeadm upgrade node -> kubelet 1.33.4 to 1.34.1 -> uncordon
  w2: drain -> kubeadm upgrade node -> kubelet 1.30.2 to 1.34.1 -> uncordon
  w3: drain -> kubeadm upgrade node -> kubelet 1.33.4 to 1.34.1 -> uncordon
  warning: w2 kubelet 1.30.2 is 4 minors behind v1.34.1; upgrade it in steps first
target v1.35.0:
  refused: control plane is 1.33; upgrade one minor version at a time
@@requires
def minor
kubeadm upgrade apply
kubeadm upgrade node
@@hint
`minor("1.33.4")` → 33. The control plane minor is the minimum minor among control plane nodes; refuse if `minor(target) - cp > 1`. Skew: `minor(target) - minor(kubelet) > 3`.
@@solution
nodes = [("cp1", "control-plane", "1.33.4"), ("cp2", "control-plane", "1.33.4"), ("w1", "worker", "1.33.4"), ("w2", "worker", "1.30.2"), ("w3", "worker", "1.33.4")]
targets = ["1.34.1", "1.35.0"]

def minor(version):
    return int(version.split(".")[1])

for target in targets:
    cp_minor = min(minor(v) for _, role, v in nodes if role == "control-plane")
    print(f"target v{target}:")
    if minor(target) - cp_minor > 1:
        print(f"  refused: control plane is 1.{cp_minor}; upgrade one minor version at a time")
        continue
    first = True
    ordered = [n for n in nodes if n[1] == "control-plane"] + [n for n in nodes if n[1] == "worker"]
    for name, role, version in ordered:
        step = "kubeadm upgrade apply v" + target if first else "kubeadm upgrade node"
        first = False
        print(f"  {name}: drain -> {step} -> kubelet {version} to {target} -> uncordon")
    for name, _, version in nodes:
        gap = minor(target) - minor(version)
        if gap > 3:
            print(f"  warning: {name} kubelet {version} is {gap} minors behind v{target}; upgrade it in steps first")

@@exercise Diagnose Pod states
Write a triage function for Pods described by their status fields. Return the likely cause and the next command for each Pod, following the lesson's table: check `phase`, waiting `reason`, last terminated `exit_code`, and readiness. Exit codes 137 with reason OOMKilled, 127 and 1 each get their own advice.
@@lang python
@@starter
pods = [
    {"name": "web-1", "phase": "Pending", "events": "0/3 nodes are available: 3 Insufficient memory."},
    {"name": "api-1", "phase": "Pending", "waiting": "ImagePullBackOff", "events": "manifest for shop/api:v9 not found"},
    {"name": "worker-1", "phase": "Running", "waiting": "CrashLoopBackOff", "exit_code": 137, "last_reason": "OOMKilled"},
    {"name": "job-1", "phase": "Running", "waiting": "CrashLoopBackOff", "exit_code": 127, "last_reason": "Error"},
    {"name": "app-1", "phase": "Running", "waiting": "CrashLoopBackOff", "exit_code": 1, "last_reason": "Error"},
    {"name": "cfg-1", "phase": "Pending", "waiting": "CreateContainerConfigError", "events": 'secret "db-pass" not found'},
    {"name": "front-1", "phase": "Running", "ready": False},
    {"name": "ok-1", "phase": "Running", "ready": True},
]

def triage(pod):
    raise NotImplementedError
@@expected
web-1: can't be scheduled (0/3 nodes are available: 3 Insufficient memory.)
  next: kubectl describe pod web-1; check requests, taints, affinity
api-1: image can't be pulled (manifest for shop/api:v9 not found)
  next: check the image name/tag and imagePullSecrets: kubectl describe pod api-1
worker-1: OOMKilled (exit 137)
  next: raise the memory limit or reduce memory use
job-1: command not found (exit 127)
  next: fix command/args or the image's entrypoint
app-1: application error (exit 1)
  next: kubectl logs app-1 --previous
cfg-1: missing config (secret "db-pass" not found)
  next: create the ConfigMap/Secret or fix the key reference
front-1: running but not ready
  next: check the readiness probe: kubectl describe pod front-1
ok-1: healthy
  next: -
@@requires
def triage
137
127
.get(
@@hint
Check `waiting` first (ImagePullBackOff, CreateContainerConfigError, CrashLoopBackOff with the exit code), then `phase == "Pending"` (scheduling), then readiness. Return a tuple `(cause, command)`.
@@solution
pods = [
    {"name": "web-1", "phase": "Pending", "events": "0/3 nodes are available: 3 Insufficient memory."},
    {"name": "api-1", "phase": "Pending", "waiting": "ImagePullBackOff", "events": "manifest for shop/api:v9 not found"},
    {"name": "worker-1", "phase": "Running", "waiting": "CrashLoopBackOff", "exit_code": 137, "last_reason": "OOMKilled"},
    {"name": "job-1", "phase": "Running", "waiting": "CrashLoopBackOff", "exit_code": 127, "last_reason": "Error"},
    {"name": "app-1", "phase": "Running", "waiting": "CrashLoopBackOff", "exit_code": 1, "last_reason": "Error"},
    {"name": "cfg-1", "phase": "Pending", "waiting": "CreateContainerConfigError", "events": 'secret "db-pass" not found'},
    {"name": "front-1", "phase": "Running", "ready": False},
    {"name": "ok-1", "phase": "Running", "ready": True},
]

def triage(pod):
    waiting = pod.get("waiting")
    name = pod["name"]
    if waiting in ("ImagePullBackOff", "ErrImagePull"):
        return f"image can't be pulled ({pod['events']})", f"check the image name/tag and imagePullSecrets: kubectl describe pod {name}"
    if waiting == "CreateContainerConfigError":
        return f"missing config ({pod['events']})", "create the ConfigMap/Secret or fix the key reference"
    if waiting == "CrashLoopBackOff":
        code = pod["exit_code"]
        if code == 137 and pod.get("last_reason") == "OOMKilled":
            return "OOMKilled (exit 137)", "raise the memory limit or reduce memory use"
        if code == 127:
            return "command not found (exit 127)", "fix command/args or the image's entrypoint"
        return f"application error (exit {code})", f"kubectl logs {name} --previous"
    if pod["phase"] == "Pending":
        return f"can't be scheduled ({pod['events']})", f"kubectl describe pod {name}; check requests, taints, affinity"
    if not pod.get("ready", True):
        return "running but not ready", f"check the readiness probe: kubectl describe pod {name}"
    return "healthy", "-"

for pod in pods:
    cause, command = triage(pod)
    print(f"{pod['name']}: {cause}\n  next: {command}")
