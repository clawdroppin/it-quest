@@lesson cl2-kubernetes Kubernetes
@@text
One server with Docker Compose is fine. Fifty servers and two hundred containers need an **orchestrator** that decides where containers run, restarts them when they die, scales them, rolls out new versions and connects them. That's **Kubernetes** (K8s), originally from Google, now the standard everywhere: EKS, AKS, GKE, OpenShift, and on-premises.

## The idea: desired state

You don't say *"start 3 containers"*. You **declare** *"there should be 3 replicas of this"* in YAML, and Kubernetes' controllers keep working until reality matches, forever. Kill a pod and a new one appears; a node dies and its pods move elsewhere.

## Architecture

- **Control plane**: **API server** (everything talks to it, including `kubectl`), **etcd** (the cluster's database), **scheduler** (picks a node for each pod), **controller manager** (the reconcile loops).
- **Worker nodes**: **kubelet** (runs the pods it's told to), a container runtime (**containerd**), **kube-proxy** (service networking).

## Objects you'll use daily

| Object | What |
|---|---|
| **Pod** | one or more containers that share an IP and storage; the smallest unit. Pods are disposable |
| **Deployment** | keeps N replicas of a pod running; does **rolling updates** and rollbacks |
| **ReplicaSet** | what a Deployment uses under the hood |
| **Service** | a stable name and IP for a changing set of pods (selected by **labels**): **ClusterIP** (internal), **NodePort**, **LoadBalancer** (cloud LB) |
| **Ingress** / Gateway API | HTTP(S) routing by host and path into services, with TLS |
| **ConfigMap** / **Secret** | configuration / sensitive values, as env vars or files. Secrets are only base64-encoded by default: encrypt etcd and limit access |
| **PersistentVolumeClaim** | a request for storage that outlives pods |
| **Namespace** | a virtual cluster for a team or app; scope for RBAC and quotas |
| **StatefulSet**, **DaemonSet**, **Job**/**CronJob** | stable identities (databases) / one pod per node (log agents) / run-to-completion tasks |

```yaml
apiVersion: apps/v1
kind: Deployment
metadata: { name: web }
spec:
  replicas: 3
  selector: { matchLabels: { app: web } }
  template:
    metadata: { labels: { app: web } }
    spec:
      containers:
        - name: web
          image: nginx:1.27-alpine
          ports: [{ containerPort: 80 }]
          resources:
            requests: { cpu: 100m, memory: 64Mi }    # used for scheduling
            limits: { cpu: 500m, memory: 128Mi }     # hard ceiling (OOMKilled above memory limit)
          readinessProbe: { httpGet: { path: /, port: 80 } }   # only send traffic when ready
          livenessProbe: { httpGet: { path: /, port: 80 } }    # restart if it hangs
```

## kubectl

```bash,norun
kubectl get pods -A                     # all namespaces
kubectl get deploy,svc,pods -o wide
kubectl apply -f k8s/                   # create or update from files (declarative)
kubectl describe pod web-7d4b9-xk2lp    # details + EVENTS: the first place to look
kubectl logs web-7d4b9-xk2lp --previous # logs of the crashed previous container
kubectl exec -it web-7d4b9-xk2lp -- sh
kubectl scale deploy web --replicas=5
kubectl set image deploy/web web=nginx:1.28-alpine
kubectl rollout status deploy/web ; kubectl rollout undo deploy/web
kubectl port-forward svc/web 8080:80
kubectl config get-contexts             # which cluster am I talking to?! (check before deleting)
```

## Troubleshooting pod states

| Status | Usually |
|---|---|
| **Pending** | no node has enough CPU/memory for the requests, or a volume can't be attached: `describe` the pod |
| **ImagePullBackOff** / ErrImagePull | wrong image name/tag, or no credentials for a private registry |
| **CrashLoopBackOff** | the container starts and exits: read `logs --previous` |
| **OOMKilled** | went over its memory limit |
| Running but no traffic | readiness probe failing, or the Service selector doesn't match the pod labels |

## Around Kubernetes

**Helm** (packages of YAML, "charts"), **Kustomize** (overlays per environment), **GitOps** with **Argo CD** / Flux (the cluster syncs itself from Git), **HPA** (Horizontal Pod Autoscaler, scales on CPU or custom metrics), service meshes (Istio, Linkerd). Certifications: **KCNA** (associate, multiple choice), **CKA** (administrator) and **CKAD** (developer), both hands-on in a real terminal.

@@quiz
What does a Deployment do if you delete one of its pods?
* Creates a replacement to get back to the desired number of replicas
- Nothing, the pod is gone
- Deletes the whole Deployment
- Restarts the node
? Kubernetes reconciles towards the declared desired state.

@@quiz
Pods are replaced and their IPs change. How do other apps reach them reliably?
* Through a Service, which selects pods by label and has a stable name and IP
- By hard-coding pod IPs
- Through etcd
- With a ConfigMap
? Services also load-balance across the matching pods.

@@quiz
A pod shows `ImagePullBackOff`. What's the most likely cause?
* The image name or tag is wrong, or credentials for a private registry are missing
- The app crashed on start
- The node ran out of disk
- The readiness probe failed
? `kubectl describe pod` shows the exact pull error.

@@quiz
A pod shows `CrashLoopBackOff`. What should you check first?
* The container's logs, with `kubectl logs <pod> --previous`
- The Service type
- The Ingress
- etcd backups
? The app starts and exits; its last output usually says why.

@@quiz
A pod stays `Pending`. What's a likely reason?
* No node has enough free CPU or memory for its resource requests
- The image doesn't exist
- The liveness probe failed
- It's running fine
? The scheduler can't place it; events in `describe` say why.

@@quiz
Which component stores the entire cluster state?
* etcd
- kubelet
- kube-proxy
- the container runtime
? Back up etcd; lose it and you lose the cluster's configuration.

@@quiz
Which probe keeps traffic away from a pod that's still starting up?
* Readiness probe
- Liveness probe
- Resource limit
- Node selector
? A failing liveness probe restarts the container; a failing readiness probe just removes it from the Service.

@@quiz
Which Service type gets a cloud load balancer with an external IP?
* LoadBalancer
- ClusterIP
- Headless
- ExternalName
? ClusterIP is internal only; NodePort opens a port on every node.

@@quiz
A log-collection agent must run exactly one pod on every node. Which object?
* DaemonSet
- Deployment with 1 replica
- StatefulSet
- CronJob
? New nodes get the pod automatically.

@@flashcards
Kubernetes :: Container orchestrator: desired state, reconciled continuously
control plane :: API server, etcd, scheduler, controller manager
kubelet :: Node agent that runs pods
pod :: Smallest unit: containers sharing IP and storage
Deployment :: N replicas, rolling updates, rollbacks
Service :: Stable name/IP for pods selected by labels
ClusterIP / NodePort / LoadBalancer :: Internal / port on each node / cloud LB
Ingress :: HTTP routing by host/path, TLS
ConfigMap / Secret :: Config / sensitive values (base64, not encrypted by default)
PVC :: Persistent storage claim
namespace :: Virtual cluster for scoping
DaemonSet :: One pod per node
StatefulSet :: Stable identity and storage per pod
readiness vs liveness :: Ready for traffic? vs alive or restart?
requests vs limits :: Scheduling reservation vs hard ceiling
CrashLoopBackOff :: Container keeps exiting: check logs --previous
ImagePullBackOff :: Can't pull the image
Helm :: Package manager for Kubernetes YAML
GitOps :: Cluster state synced from Git (Argo CD, Flux)
HPA :: Horizontal Pod Autoscaler
CKA / CKAD / KCNA :: Admin (hands-on) / developer (hands-on) / associate

@@drill
Which kubectl command shows a pod's details and recent events? (kubectl ... pod NAME)
= kubectl describe pod NAME
? Events explain scheduling, pulling, probes and restarts.
