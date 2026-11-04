@@lesson ck1-services-gateway Services, DNS, Ingress and the Gateway API
@@text
"Services and networking" is 20% of the CKA. You need to expose workloads with the right Service type, know the DNS names Kubernetes creates, debug Services with no endpoints, and route HTTP traffic with Ingress and the newer **Gateway API** (part of the current curriculum). This lesson also covers the CNI and kube-proxy pieces underneath.

## The networking model

- Every Pod gets its own IP; all Pods can reach all Pods without NAT (unless **NetworkPolicies** restrict it).
- The **CNI plugin** (Calico, Cilium, Flannel...) implements Pod networking; its config is in `/etc/cni/net.d/`, binaries in `/opt/cni/bin/`. Without a CNI, nodes stay **NotReady** and Pods stay Pending/ContainerCreating.
- **kube-proxy** (iptables, IPVS or nftables mode) or an eBPF replacement (Cilium) implements Service virtual IPs.

## Services

| Type | Reachable from | Notes |
|---|---|---|
| **ClusterIP** (default) | inside the cluster | stable virtual IP and DNS name |
| **NodePort** | `<any node IP>:<30000–32767>` | also gets a ClusterIP |
| **LoadBalancer** | external load balancer (cloud, MetalLB) | also NodePort + ClusterIP |
| **ExternalName** | DNS CNAME to an external name | no proxying |
| **Headless** (`clusterIP: None`) | DNS returns Pod IPs directly | StatefulSets |

A Service selects Pods by **labels**; matching **ready** Pods become **EndpointSlices**. `port` (Service port), `targetPort` (container port, number or name), `nodePort`.

```bash,norun
kubectl expose deployment web --port=80 --target-port=8080 --type=NodePort
kubectl get svc,endpointslices -l app=web
kubectl run tmp --rm -it --image=busybox:1.36 --restart=Never -- wget -qO- http://web.default.svc.cluster.local
```

**No endpoints?** Check that the selector matches the Pod labels (`kubectl get pods --show-labels`), that Pods are Ready (readiness probe), and that `targetPort` matches the container port.

## DNS (CoreDNS)

| Name | Resolves to |
|---|---|
| `web` | Service in the same namespace (search domains) |
| `web.shop` | Service `web` in namespace `shop` |
| `web.shop.svc.cluster.local` | fully qualified Service name |
| `db-0.db.shop.svc.cluster.local` | Pod `db-0` behind headless Service `db` (StatefulSet) |
| `10-244-1-7.shop.pod.cluster.local` | a Pod by IP (dashes) |

CoreDNS runs as a Deployment in `kube-system` with the `kube-dns` Service; its config is the `coredns` ConfigMap (Corefile). Pods get `/etc/resolv.conf` pointing at the kube-dns ClusterIP with search domains `<ns>.svc.cluster.local svc.cluster.local cluster.local`.

## Ingress

An **Ingress** resource defines HTTP host/path rules to Services; an **Ingress controller** (ingress-nginx, Traefik, HAProxy...) must be installed to implement it. `pathType`: `Prefix`, `Exact`, `ImplementationSpecific`. TLS with a Secret of type `kubernetes.io/tls`. `ingressClassName` selects the controller.

```bash,norun
kubectl create ingress shop --class=nginx --rule="shop.example.com/api*=api:80" --rule="shop.example.com/*=web:80"
```

## Gateway API

The successor to Ingress: role-oriented and more expressive.

- **GatewayClass** (infrastructure provider: which controller), **Gateway** (listeners: ports, protocols, hostnames, TLS; owned by the platform team), **HTTPRoute** / GRPCRoute / TLSRoute / TCPRoute (owned by app teams, attached via `parentRefs`).
- HTTPRoute matches on path (`Exact`, `PathPrefix`, `RegularExpression`), headers, query params and method; actions include **weighted backendRefs** (canary), header modification, redirects and URL rewrites.

```yaml,norun
apiVersion: gateway.networking.k8s.io/v1
kind: HTTPRoute
metadata: {name: shop, namespace: shop}
spec:
  parentRefs: [{name: public-gw, namespace: infra}]
  hostnames: ["shop.example.com"]
  rules:
  - matches: [{path: {type: PathPrefix, value: /api}}]
    backendRefs:
    - {name: api-v1, port: 80, weight: 90}
    - {name: api-v2, port: 80, weight: 10}
  - backendRefs: [{name: web, port: 80}]
```

Precedence when several rules match: **Exact** path over prefix, then the **longest prefix**, then more header matches, and so on.

## NetworkPolicy (recap)

Default allow; once any policy selects a Pod for a direction, only allowed traffic passes. Remember to allow **DNS (UDP/TCP 53)** egress when restricting egress.

@@quiz
Which Service type exposes a port on every node in the 30000–32767 range?
- ClusterIP
* NodePort
- ExternalName
- Headless
? LoadBalancer builds on NodePort.

@@quiz
A Service has no endpoints. What's the first thing to check?
* Whether its selector matches the Pods' labels (and the Pods are Ready)
- The CoreDNS ConfigMap
- The node's kernel version
- The Ingress class
? Endpoints come from label selection of ready Pods.

@@quiz
What is the fully qualified DNS name of Service `api` in namespace `shop`?
- api.shop.cluster.local
* api.shop.svc.cluster.local
- shop.api.svc.cluster.local
- api.svc.shop.local
? <service>.<namespace>.svc.<cluster domain>.

@@quiz
An Ingress resource was created but nothing routes traffic. What's likely missing?
- A NetworkPolicy
* An Ingress controller (or the right ingressClassName)
- A headless Service
- kube-proxy
? Ingress resources need a controller.

@@quiz
In the Gateway API, which resource do application teams usually own to define routing to their Services?
- GatewayClass
- Gateway
* HTTPRoute
- IngressClass
? Gateways are owned by the platform team.

@@quiz
New nodes stay NotReady and Pods are stuck in ContainerCreating after kubeadm init. What's missing?
- metrics-server
* A CNI plugin
- An Ingress controller
- CoreDNS replicas
? Install Calico, Cilium, Flannel...

@@flashcards
Pod networking :: every Pod an IP, no NAT
CNI config :: /etc/cni/net.d
kube-proxy :: implements Service IPs
ClusterIP :: internal virtual IP
NodePort range :: 30000–32767
LoadBalancer :: external LB + NodePort
ExternalName :: DNS CNAME
headless Service :: clusterIP None, Pod IPs
targetPort :: container port
EndpointSlice :: ready Pod IPs for a Service
Service FQDN :: svc.ns.svc.cluster.local
StatefulSet Pod DNS :: pod-0.svc.ns.svc.cluster.local
CoreDNS config :: coredns ConfigMap (Corefile)
Ingress controller :: implements Ingress rules
pathType :: Prefix, Exact, ImplementationSpecific
TLS Secret type :: kubernetes.io/tls
GatewayClass :: which controller
Gateway :: listeners, owned by platform
HTTPRoute :: routing rules, owned by apps
weighted backendRefs :: canary traffic split
NetworkPolicy egress :: remember DNS port 53

@@drill
What is the default Service type?
= ClusterIP
= clusterip
? Internal only.

@@drill
Which Gateway API resource defines listeners (ports, protocols, TLS)?
= Gateway
? GatewayClass picks the controller.

@@exercise Resolve Kubernetes DNS names
Implement the resolver a Pod uses: try the name as-is if it ends with `.cluster.local`, otherwise append each search domain of the Pod's namespace in order (`<ns>.svc.cluster.local`, `svc.cluster.local`, `cluster.local`) and return the first that exists. Records: Services (ClusterIP, or the Pod IPs for headless Services) and StatefulSet Pods behind headless Services. Print what each lookup returns or `NXDOMAIN`.
@@lang python
@@starter
services = {("shop", "web"): "10.96.10.5", ("shop", "api"): "10.96.10.9", ("data", "db"): None, ("kube-system", "kube-dns"): "10.96.0.10"}
statefulset_pods = {("data", "db"): {"db-0": "10.244.1.7", "db-1": "10.244.2.4"}}
lookups = [("shop", "web"), ("shop", "api.shop"), ("shop", "db.data"), ("data", "db-1.db"), ("shop", "db"), ("shop", "kube-dns.kube-system.svc.cluster.local"), ("shop", "web.data")]

def records():
    raise NotImplementedError
@@expected
from shop: web -> web.shop.svc.cluster.local = 10.96.10.5
from shop: api.shop -> api.shop.svc.cluster.local = 10.96.10.9
from shop: db.data -> db.data.svc.cluster.local = 10.244.1.7, 10.244.2.4
from data: db-1.db -> db-1.db.data.svc.cluster.local = 10.244.2.4
from shop: db -> NXDOMAIN
from shop: kube-dns.kube-system.svc.cluster.local -> kube-dns.kube-system.svc.cluster.local = 10.96.0.10
from shop: web.data -> NXDOMAIN
@@requires
def records
svc.cluster.local
@@hint
Build a dict of FQDN → answer: `f"{svc}.{ns}.svc.cluster.local"` maps to the ClusterIP, or to the list of Pod IPs for headless Services; `f"{pod}.{svc}.{ns}.svc.cluster.local"` maps to each Pod IP.
@@solution
services = {("shop", "web"): "10.96.10.5", ("shop", "api"): "10.96.10.9", ("data", "db"): None, ("kube-system", "kube-dns"): "10.96.0.10"}
statefulset_pods = {("data", "db"): {"db-0": "10.244.1.7", "db-1": "10.244.2.4"}}
lookups = [("shop", "web"), ("shop", "api.shop"), ("shop", "db.data"), ("data", "db-1.db"), ("shop", "db"), ("shop", "kube-dns.kube-system.svc.cluster.local"), ("shop", "web.data")]

def records():
    table = {}
    for (ns, svc), ip in services.items():
        fqdn = f"{svc}.{ns}.svc.cluster.local"
        if ip is None:
            pods = statefulset_pods.get((ns, svc), {})
            table[fqdn] = ", ".join(pods.values())
            for pod, pod_ip in pods.items():
                table[f"{pod}.{fqdn}"] = pod_ip
        else:
            table[fqdn] = ip
    return table

table = records()
for ns, name in lookups:
    if name.endswith(".cluster.local"):
        candidates = [name]
    else:
        candidates = [f"{name}.{d}" for d in (f"{ns}.svc.cluster.local", "svc.cluster.local", "cluster.local")]
    answer = next(((c, table[c]) for c in candidates if c in table), None)
    if answer:
        print(f"from {ns}: {name} -> {answer[0]} = {answer[1]}")
    else:
        print(f"from {ns}: {name} -> NXDOMAIN")

@@exercise Match HTTPRoute rules
Implement Gateway API matching for one hostname: each rule has optional path matches (`Exact` or `PathPrefix`, where a prefix matches whole path segments: `/api` matches `/api` and `/api/x` but not `/apix`) and optional header matches; a rule without matches matches everything. Among matching rules pick by precedence: Exact over PathPrefix, then longer prefix, then more header matches. Weighted backends choose by the request's `bucket` (0–99) against cumulative weights. Print the backend for each request.
@@lang python
@@starter
rules = [
    {"matches": [{"path": ("PathPrefix", "/api"), "headers": {}}], "backends": [("api-v1", 90), ("api-v2", 10)]},
    {"matches": [{"path": ("PathPrefix", "/api"), "headers": {"x-canary": "true"}}], "backends": [("api-v2", 100)]},
    {"matches": [{"path": ("Exact", "/api/health"), "headers": {}}], "backends": [("health", 100)]},
    {"matches": [{"path": ("PathPrefix", "/api/admin"), "headers": {}}], "backends": [("admin", 100)]},
    {"matches": [], "backends": [("web", 100)]},
]
requests = [("/", {}, 5), ("/api/orders", {}, 42), ("/api/orders", {}, 95), ("/api/orders", {"x-canary": "true"}, 3),
            ("/api/health", {}, 50), ("/api/admin/users", {}, 50), ("/apix", {}, 50)]

def path_matches(kind, value, path):
    raise NotImplementedError
@@expected
/                 {}                     bucket  5 -> web
/api/orders       {}                     bucket 42 -> api-v1
/api/orders       {}                     bucket 95 -> api-v2
/api/orders       {'x-canary': 'true'}   bucket  3 -> api-v2
/api/health       {}                     bucket 50 -> health
/api/admin/users  {}                     bucket 50 -> admin
/apix             {}                     bucket 50 -> web
@@requires
def path_matches
def precedence
@@hint
Prefix: `path == value or path.startswith(value.rstrip("/") + "/")`. Precedence key for a match: `(kind == "Exact", len(value), len(headers))`; a rule with no matches gets the lowest key. Pick `max` over all matching (rule, match) pairs.
@@solution
rules = [
    {"matches": [{"path": ("PathPrefix", "/api"), "headers": {}}], "backends": [("api-v1", 90), ("api-v2", 10)]},
    {"matches": [{"path": ("PathPrefix", "/api"), "headers": {"x-canary": "true"}}], "backends": [("api-v2", 100)]},
    {"matches": [{"path": ("Exact", "/api/health"), "headers": {}}], "backends": [("health", 100)]},
    {"matches": [{"path": ("PathPrefix", "/api/admin"), "headers": {}}], "backends": [("admin", 100)]},
    {"matches": [], "backends": [("web", 100)]},
]
requests = [("/", {}, 5), ("/api/orders", {}, 42), ("/api/orders", {}, 95), ("/api/orders", {"x-canary": "true"}, 3),
            ("/api/health", {}, 50), ("/api/admin/users", {}, 50), ("/apix", {}, 50)]

def path_matches(kind, value, path):
    if kind == "Exact":
        return path == value
    return path == value or path.startswith(value.rstrip("/") + "/")

def precedence(match):
    kind, value = match["path"]
    return (kind == "Exact", len(value), len(match["headers"]))

def route(path, headers, bucket):
    best = None
    for rule in rules:
        if not rule["matches"]:
            candidates = [((False, -1, -1), rule)]
        else:
            candidates = [(precedence(m), rule) for m in rule["matches"]
                          if path_matches(*m["path"], path) and all(headers.get(k) == v for k, v in m["headers"].items())]
        for key, r in candidates:
            if best is None or key > best[0]:
                best = (key, r)
    running = 0
    for name, weight in best[1]["backends"]:
        running += weight
        if bucket < running:
            return name
    return best[1]["backends"][-1][0]

for path, headers, bucket in requests:
    print(f"{path:<17} {str(headers):<22} bucket {bucket:>2} -> {route(path, headers, bucket)}")
