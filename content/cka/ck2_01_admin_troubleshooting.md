@@lesson ck2-admin-troubleshooting Cluster Administration and Troubleshooting
@@text
## Installing and upgrading with kubeadm

```bash,norun
# control plane
kubeadm init --pod-network-cidr=10.244.0.0/16
mkdir -p ~/.kube && cp /etc/kubernetes/admin.conf ~/.kube/config
kubectl apply -f <CNI manifest>                 # nodes stay NotReady until a CNI is installed
kubeadm token create --print-join-command      # run the printed command on each worker
```

**Upgrades** go one minor version at a time (1.33 → 1.34), control plane first, then each worker:

```bash,norun
# on the control plane
apt-mark unhold kubeadm && apt-get install -y kubeadm=1.34.x-* && apt-mark hold kubeadm
kubeadm upgrade plan
kubeadm upgrade apply v1.34.x
kubectl drain cp1 --ignore-daemonsets
apt-get install -y kubelet=1.34.x-* kubectl=1.34.x-*
systemctl daemon-reload && systemctl restart kubelet
kubectl uncordon cp1
# on each worker: upgrade kubeadm, then `kubeadm upgrade node`, drain, upgrade kubelet, restart, uncordon
```

(Package repositories are per minor version on pkgs.k8s.io, so you may need to change the repository line first.)

Version skew: the kubelet may be up to **three** minor versions older than the API server, never newer.

## Backing up and restoring etcd

The single most important admin skill, and a classic exam task:

```bash,norun
ETCDCTL_API=3 etcdctl snapshot save /backup/etcd.db \
  --endpoints=https://127.0.0.1:2379 \
  --cacert=/etc/kubernetes/pki/etcd/ca.crt \
  --cert=/etc/kubernetes/pki/etcd/server.crt \
  --key=/etc/kubernetes/pki/etcd/server.key

etcdutl snapshot restore /backup/etcd.db --data-dir=/var/lib/etcd-restored
# then point the etcd static pod's hostPath volume (in /etc/kubernetes/manifests/etcd.yaml) to the new data dir
```

The certificate paths are in the etcd static pod manifest: `grep -E 'cert|key|trusted' /etc/kubernetes/manifests/etcd.yaml`.

## Security: RBAC, service accounts and certificates

- **Role** (namespaced) / **ClusterRole** (cluster-wide): lists of verbs on resources. **RoleBinding** / **ClusterRoleBinding** give them to users, groups or ServiceAccounts.
- A ClusterRole bound with a *RoleBinding* only applies in that RoleBinding's namespace: a handy way to reuse roles.

```bash,norun
k create role pod-reader -n shop --verb=get,list,watch --resource=pods
k create rolebinding ann-reads -n shop --role=pod-reader --user=ann
k create sa deployer -n shop
k create clusterrolebinding deployer-edit --clusterrole=edit --serviceaccount=shop:deployer
k auth can-i list secrets -n shop --as=ann                 # no
k auth can-i create deployments -n shop --as=system:serviceaccount:shop:deployer
```

- Users don't exist as objects: they're names in **client certificates** (CN = user, O = group) or come from OIDC. New user: create a key and CSR, submit a **CertificateSigningRequest**, `k certificate approve`, put the certificate in a kubeconfig.
- `kubeadm certs check-expiration` and `kubeadm certs renew all`: cluster certificates expire after a year.
- **Pod Security Admission**: label a namespace `pod-security.kubernetes.io/enforce=restricted` to reject privileged pods.

## Helm and Kustomize

Both are in the current CKA curriculum:

- **Helm** installs packaged applications (*charts*): `helm repo add`, `helm install web bitnami/nginx -n web --create-namespace --set replicaCount=2`, `helm upgrade`, `helm rollback`, `helm list -A`.
- **Kustomize** (built into kubectl) layers changes on plain YAML: a `kustomization.yaml` lists resources and patches; `k apply -k overlays/prod`.
- **CRDs** and **operators** extend the API with new kinds (`k get crd`).

## A troubleshooting method

Work from the outside in, and read what Kubernetes tells you:

| Symptom | Look at | Typical causes |
|---|---|---|
| Pod **Pending** | `k describe pod` events | not enough CPU/memory, taints, affinity, unbound PVC |
| **ImagePullBackOff / ErrImagePull** | events | a typo in the image name or tag, a private registry without `imagePullSecrets` |
| **CrashLoopBackOff** | `k logs --previous` | the app crashes: bad config, missing env var or Secret, wrong command |
| **CreateContainerConfigError** | events | a referenced ConfigMap or Secret key doesn't exist |
| Running but **0/1 READY** | `k describe`, probe settings | readiness probe fails (wrong port or path) |
| Service unreachable | `k get endpoints`, `k exec ... -- wget -qO- svc:port` | selector/label mismatch, wrong `targetPort`, NetworkPolicy |
| DNS fails | `k exec -it pod -- nslookup kubernetes.default` | CoreDNS pods not running |
| Node **NotReady** | `systemctl status kubelet`, `journalctl -u kubelet` | kubelet stopped, wrong config, certificate, runtime down, no CNI |
| kubectl can't connect | `crictl ps` on the control plane, `/var/log/pods` | the API server static pod is broken (a typo in its manifest) |

`crictl` talks to the container runtime directly: `crictl ps -a`, `crictl logs <id>`. It works even when the API server is down.

`k get events -A --sort-by=.lastTimestamp` shows what happened recently across the whole cluster.

@@quiz
What's the correct order to upgrade a kubeadm cluster from 1.33 to 1.34?
* kubeadm on the control plane, `kubeadm upgrade apply`, then kubelet and kubectl there; then each worker in turn
- All kubelets first, then the control plane
- Skip straight to the newest version
- Reinstall the cluster
? Control plane first; kubelets must never be newer than the API server.

@@quiz
A pod shows CreateContainerConfigError. What's the most likely cause?
* It references a ConfigMap or Secret (or a key in one) that doesn't exist
- The image doesn't exist
- The node is out of memory
- The liveness probe fails
? ImagePullBackOff is the image; CrashLoopBackOff is the app itself.

@@quiz
How do you check whether the ServiceAccount `deployer` in namespace `shop` may delete pods there?
* `kubectl auth can-i delete pods -n shop --as=system:serviceaccount:shop:deployer`
- `kubectl get sa deployer -o yaml`
- `kubectl describe role`
- `kubectl logs deployer`
? `auth can-i` with `--as` impersonates any user or ServiceAccount.

@@quiz
kubectl suddenly can't reach the API server after you edited `/etc/kubernetes/manifests/kube-apiserver.yaml`. How do you investigate?
* Use `crictl ps -a` and `crictl logs` (or /var/log/pods) on the control-plane node to see why the static pod fails
- `kubectl describe pod kube-apiserver`
- Restore etcd
- Reboot every worker
? With the API server down, kubectl can't help; the runtime still can.

@@quiz
Which etcdctl subcommand takes a backup?
* `snapshot save`
- `backup create`
- `member add`
- `get / --prefix`
? Restore with `etcdutl snapshot restore` into a new data directory.

@@flashcards
kubeadm upgrade order :: control plane first, one minor version at a time
kubelet version skew :: up to three minor versions older, never newer
etcd backup :: etcdctl snapshot save with the CA, cert and key
etcd restore :: etcdutl snapshot restore --data-dir, then update the manifest
Role vs ClusterRole :: namespaced / cluster-wide
kubectl auth can-i --as :: check permissions by impersonation
user identity in certificates :: CN = user, O = group
cluster certificates :: expire after one year: kubeadm certs renew
Pending :: scheduling problem: describe the pod
ImagePullBackOff :: wrong image or no pull secret
CrashLoopBackOff :: the app crashes: logs --previous
crictl :: talks to the runtime even without the API server
helm rollback :: return a release to an earlier revision
kubectl apply -k :: apply a Kustomize directory

@@drill
Which port does etcd listen on for clients?
= 2379
