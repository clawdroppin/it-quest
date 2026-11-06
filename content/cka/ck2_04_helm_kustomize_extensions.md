@@lesson ck2-helm-kustomize-extensions Helm, Kustomize and Extending Kubernetes: Charts, Overlays, CRDs, Admission and Plugins
@@lang python
@@text
The CKA curriculum asks you to "use Helm and Kustomize to install cluster components", to "understand extension interfaces (CNI, CSI, CRI)" and to "understand CRDs, install and configure operators". None of these is hard, but each has a few rules that decide what ends up in the cluster. This lesson goes through them at the level the exam and real clusters need.

## Helm in practice

Helm installs a **chart** (a package of templated manifests) as a **release** with a name, in a namespace. Every install or upgrade creates a new **revision**, stored as a Secret in the release's namespace.

```
helm repo add bitnami https://charts.bitnami.com/bitnami
helm repo update
helm search repo nginx
helm show values bitnami/nginx > values.yaml        # see what you can set
helm install web bitnami/nginx -n web --create-namespace -f values.yaml --set replicaCount=3
helm list -A
helm get values web -n web                          # what you set
helm get manifest web -n web                        # what was applied
helm upgrade web bitnami/nginx -n web --set replicaCount=4
helm history web -n web
helm rollback web 1 -n web
helm uninstall web -n web
```

**Value precedence**, lowest to highest:

1. The chart's own `values.yaml`.
2. A parent chart's values for a subchart.
3. Each `-f` / `--values` file, **in the order given** (later files win).
4. `--set`, `--set-string` and `--set-file`, applied last (later ones win).

Maps are **merged** key by key, but lists are **replaced** whole. On `helm upgrade`, values you don't pass again are reset to the chart defaults unless you add `--reuse-values` (or `--reset-then-reuse-values`). That surprises people who set something with `--set` at install time and lose it on the next upgrade.

Useful flags: `--dry-run` with `--debug` renders without installing, `helm template` renders locally, `--version` pins the chart version, `--wait` waits for resources to be ready, and `--atomic` rolls back automatically if the upgrade fails.

## Kustomize in practice

Kustomize is built into kubectl (`kubectl apply -k dir/`, `kubectl kustomize dir/` to just render). It doesn't template. It takes plain YAML and transforms it.

```
base/
  kustomization.yaml     resources: [deployment.yaml, service.yaml]
  deployment.yaml
  service.yaml
overlays/prod/
  kustomization.yaml
```

```yaml
# overlays/prod/kustomization.yaml
resources:
  - ../../base
namespace: shop-prod
namePrefix: prod-
labels:
  - pairs: {env: prod}
    includeSelectors: false
images:
  - name: shop/web
    newTag: "2.4.1"
replicas:
  - name: web
    count: 4
configMapGenerator:
  - name: web-config
    literals: [LOG_LEVEL=warn]
patches:
  - path: resources-patch.yaml
```

What each field does:

- `resources` lists files or other kustomization directories (a **base**). An **overlay** points at the base and changes it.
- `namePrefix` / `nameSuffix` rename every resource **and update references** to them (a Deployment's `configMapRef`, a Service's name in an Ingress).
- `namespace` sets the namespace on everything.
- `labels` (or the older `commonLabels`, which also changes selectors, a problem on existing Deployments because selectors are immutable) adds labels.
- `images` changes image names and tags without touching the Deployment YAML.
- `configMapGenerator` / `secretGenerator` create ConfigMaps or Secrets with a **content hash suffix** (`web-config-7c9hm2kt5b`). When the content changes, the name changes, the reference in the Deployment changes, and so the Pods roll. That's the trick that makes config changes trigger rollouts.
- `patches` applies **strategic merge patches** (partial YAML merged by key; lists of containers are merged by `name`) or **JSON 6902 patches** (`op: replace, path: /spec/replicas`).

Helm and Kustomize combine: `helm template` output can be a Kustomize resource, and `kustomize build --enable-helm` can inflate charts.

## Custom resources and operators

A **CustomResourceDefinition** (CRD) adds a new kind to the API: `kubectl get crd`, then `kubectl get certificates -A` works like any built-in kind. The CRD holds an OpenAPI **schema** for validation, the **scope** (Namespaced or Cluster), versions, and optional `additionalPrinterColumns` for `kubectl get`.

A CRD only stores data. An **operator** is a controller (usually a Deployment) that watches those resources and acts on them: cert-manager issues certificates for `Certificate` objects, a Postgres operator creates StatefulSets, Services and backups for a `PostgresCluster`. Installing an operator usually means applying its CRDs, then its controller and RBAC (often with Helm), then creating custom resources.

On the exam you may be asked to list the CRDs of an installed operator, read a field from a custom resource (`kubectl get <kind> <name> -o jsonpath=...`), or use `kubectl explain <kind>.spec` to find a field.

## Admission control

Every create or update request goes through: **authentication → authorization (RBAC) → mutating admission → schema validation → validating admission → stored in etcd**.

- **Mutating** admission changes the object, for example adding default tolerations, injecting a sidecar, or setting a default StorageClass.
- **Validating** admission accepts or rejects it, for example Pod Security Admission, ResourceQuota and LimitRanger checks.
- Built-in plugins are enabled with the API server flag `--enable-admission-plugins` (on kubeadm clusters, in `/etc/kubernetes/manifests/kube-apiserver.yaml`).
- **Webhooks** (`MutatingWebhookConfiguration`, `ValidatingWebhookConfiguration`) call your own service. A webhook with `failurePolicy: Fail` whose service is down blocks every matching request, a classic cluster-wide outage, so scope webhooks narrowly with selectors.
- **ValidatingAdmissionPolicy** expresses checks in **CEL** inside the API server, with no webhook to run: `object.spec.replicas <= 5`.

## The extension interfaces

| Interface | Plugs in | Where to look |
|---|---|---|
| **CRI** (Container Runtime Interface) | containerd, CRI-O | `crictl ps`, `crictl logs`, `crictl pods`; kubelet's `--container-runtime-endpoint` (`unix:///run/containerd/containerd.sock`) |
| **CNI** (Container Network Interface) | Calico, Cilium, Flannel | config in `/etc/cni/net.d/`, binaries in `/opt/cni/bin/`; Pods stay `ContainerCreating` and nodes `NotReady` without one |
| **CSI** (Container Storage Interface) | cloud disks, Ceph, NFS drivers | `kubectl get csidrivers`, the driver's controller Deployment and node DaemonSet; a StorageClass names the CSI `provisioner` |

When a node is `NotReady` right after `kubeadm init`, the missing piece is almost always the CNI plugin. When Pods sit in `ContainerCreating` with "failed to create pod sandbox", look at CNI. When PVCs stay `Pending` with a CSI provisioner, look at the CSI controller's logs.

@@quiz
A chart's values.yaml sets replicaCount: 1. You run helm install with -f prod.yaml (replicaCount: 3) and --set replicaCount=5. What is applied?
* 5
- 1
- 3
- An error, because the value is set twice
? --set is applied after all -f files.

@@quiz
You installed a release with --set image.tag=2.0 and later run helm upgrade with only --set replicaCount=4. What happens to image.tag?
* It returns to the chart default, unless you add --reuse-values
- It stays at 2.0 because Helm remembers every value
- The upgrade fails
- Helm asks interactively
? Values not passed again are reset to the chart defaults on upgrade.

@@quiz
Why does a Kustomize configMapGenerator add a hash suffix to the ConfigMap name?
* So that a content change changes the name and the reference in the Deployment, which triggers a rollout
- To avoid name clashes with Secrets
- Because ConfigMap names must be unique cluster-wide
- To encrypt the data
? Pods don't restart when a ConfigMap they use changes; a new name forces a new ReplicaSet.

@@quiz
What does a CRD on its own do?
* Adds a new resource kind to the API; a controller (operator) is needed to act on it
- Runs a controller that manages applications
- Replaces the scheduler
- Validates Pods against Pod Security Standards
? CRDs store and validate data; operators reconcile it.

@@quiz
In which order are admission phases applied?
* Mutating admission, then schema validation, then validating admission
- Validating, then mutating
- Mutating admission runs after the object is stored in etcd
- RBAC runs after admission
? Authentication and authorization come first, and the object is stored last.

@@quiz
A freshly built kubeadm cluster has all nodes NotReady and CoreDNS Pods Pending. What is most likely missing?
* A CNI plugin
- A CSI driver
- metrics-server
- An Ingress controller
? Install Calico, Cilium or Flannel and the nodes become Ready.

@@flashcards
Helm release :: an installed instance of a chart, with numbered revisions
helm get values :: the values you supplied for a release
helm rollback NAME REV :: return a release to an earlier revision
Helm value precedence :: chart values < -f files in order < --set
--reuse-values :: keep the previous release's values on upgrade
kubectl kustomize dir :: render a kustomization without applying it
namePrefix :: renames resources and updates references
configMapGenerator :: ConfigMap with a content hash suffix that triggers rollouts
Strategic merge patch :: partial YAML merged by key (containers by name)
CRD :: adds a new API kind; an operator acts on it
Admission order :: mutating, schema validation, validating
failurePolicy Fail :: a down webhook blocks matching requests
crictl :: inspect containers and Pods through CRI on a node
/etc/cni/net.d :: CNI configuration on each node

@@drill
Which command shows the revisions of the Helm release "web" in namespace "web"?
= helm history web -n web
= helm history web --namespace web

@@drill
Which kubectl command renders the kustomization in overlays/prod without applying it?
= kubectl kustomize overlays/prod
= k kustomize overlays/prod
= kubectl kustomize overlays/prod/

@@exercise Resolve Helm values
@@concepts maps, functions, loops
Work out the values Helm will use. Start from the chart defaults, deep-merge each `-f` file in order (maps merge key by key, lists and scalars replace), then apply each `--set` (a dotted path like `image.tag=2.4.1`; "true"/"false" and whole numbers become booleans and integers, as Helm does). Print the final values as flattened dotted paths, sorted.
@@starter
chart_defaults = {
    "replicaCount": 1,
    "image": {"repository": "shop/web", "tag": "1.0.0", "pullPolicy": "IfNotPresent"},
    "service": {"type": "ClusterIP", "port": 80},
    "ingress": {"enabled": False, "hosts": ["chart-example.local"]},
    "resources": {},
}
values_files = [
    {"image": {"tag": "2.3.0"}, "ingress": {"enabled": True, "hosts": ["shop.example.com"]}},
    {"replicaCount": 3, "resources": {"requests": {"cpu": "200m", "memory": "256Mi"}}, "service": {"type": "LoadBalancer"}},
]
set_flags = ["image.tag=2.4.1", "replicaCount=4", "ingress.enabled=false", "resources.limits.memory=512Mi"]

def deep_merge(base, override):
    raise NotImplementedError

def apply_set(values, flag):
    raise NotImplementedError

def flatten(d, prefix=""):
    out = {}
    for k, v in d.items():
        path = f"{prefix}{k}"
        if isinstance(v, dict) and v:
            out.update(flatten(v, path + "."))
        else:
            out[path] = v
    return out

values = chart_defaults
for f in values_files:
    values = deep_merge(values, f)
for flag in set_flags:
    apply_set(values, flag)
for path, value in sorted(flatten(values).items()):
    print(f"{path} = {value!r}")
@@expected
image.pullPolicy = 'IfNotPresent'
image.repository = 'shop/web'
image.tag = '2.4.1'
ingress.enabled = False
ingress.hosts = ['shop.example.com']
replicaCount = 4
resources.limits.memory = '512Mi'
resources.requests.cpu = '200m'
resources.requests.memory = '256Mi'
service.port = 80
service.type = 'LoadBalancer'
@@requires
def deep_merge
def apply_set
.setdefault(part, {})
@@hint
`deep_merge` copies `base`; for each key in `override`, if both sides are dicts recurse, otherwise take the override value. `apply_set` splits `key=value`, converts the value ("true"/"false" to bools, digits to int), walks the dotted path with `node = node.setdefault(part, {})` for all but the last part, and sets the last one.
@@solution
import copy

chart_defaults = {
    "replicaCount": 1,
    "image": {"repository": "shop/web", "tag": "1.0.0", "pullPolicy": "IfNotPresent"},
    "service": {"type": "ClusterIP", "port": 80},
    "ingress": {"enabled": False, "hosts": ["chart-example.local"]},
    "resources": {},
}
values_files = [
    {"image": {"tag": "2.3.0"}, "ingress": {"enabled": True, "hosts": ["shop.example.com"]}},
    {"replicaCount": 3, "resources": {"requests": {"cpu": "200m", "memory": "256Mi"}}, "service": {"type": "LoadBalancer"}},
]
set_flags = ["image.tag=2.4.1", "replicaCount=4", "ingress.enabled=false", "resources.limits.memory=512Mi"]

def deep_merge(base, override):
    result = copy.deepcopy(base)
    for key, value in override.items():
        if isinstance(result.get(key), dict) and isinstance(value, dict):
            result[key] = deep_merge(result[key], value)
        else:
            result[key] = copy.deepcopy(value)
    return result

def convert(text):
    if text in ("true", "false"):
        return text == "true"
    if text.isdigit():
        return int(text)
    return text

def apply_set(values, flag):
    path, raw = flag.split("=", 1)
    parts = path.split(".")
    node = values
    for part in parts[:-1]:
        node = node.setdefault(part, {})
    node[parts[-1]] = convert(raw)

def flatten(d, prefix=""):
    out = {}
    for k, v in d.items():
        path = f"{prefix}{k}"
        if isinstance(v, dict) and v:
            out.update(flatten(v, path + "."))
        else:
            out[path] = v
    return out

values = chart_defaults
for f in values_files:
    values = deep_merge(values, f)
for flag in set_flags:
    apply_set(values, flag)
for path, value in sorted(flatten(values).items()):
    print(f"{path} = {value!r}")

@@exercise Build a Kustomize overlay
@@concepts maps, strings, json
Apply an overlay to base resources the way Kustomize does (a simplified model): add `namePrefix` to every name, set the namespace, add labels, override the image tag, set replicas on the named Deployment, and generate a ConfigMap whose name gets a 10-character hash suffix of its data. Update the Deployment's reference to the ConfigMap and the Service's reference to the Deployment so they point at the new names. Print each resulting resource, then show that changing the ConfigMap data changes the name the Deployment refers to.
@@starter
import hashlib
import json

base = [
    {"kind": "Deployment", "name": "web", "labels": {"app": "web"}, "replicas": 1,
     "image": "shop/web:1.0.0", "configMapRef": "web-config"},
    {"kind": "Service", "name": "web", "labels": {"app": "web"}, "targets": "web"},
]
overlay = {"namePrefix": "prod-", "namespace": "shop-prod", "labels": {"env": "prod"},
           "images": {"shop/web": "2.4.1"}, "replicas": {"web": 4},
           "configMapGenerator": {"name": "web-config", "data": {"LOG_LEVEL": "warn"}}}

def config_hash(data):
    raise NotImplementedError

def build(base, overlay):
    raise NotImplementedError

for r in build(base, overlay):
    print(json.dumps(r, sort_keys=True))
changed = dict(overlay, configMapGenerator={"name": "web-config", "data": {"LOG_LEVEL": "debug"}})
dep = next(r for r in build(base, changed) if r["kind"] == "Deployment")
print("after changing LOG_LEVEL the Deployment refers to", dep["configMapRef"])
@@expected
{"data": {"LOG_LEVEL": "warn"}, "kind": "ConfigMap", "labels": {"env": "prod"}, "name": "prod-web-config-cf9c827539", "namespace": "shop-prod"}
{"configMapRef": "prod-web-config-cf9c827539", "image": "shop/web:2.4.1", "kind": "Deployment", "labels": {"app": "web", "env": "prod"}, "name": "prod-web", "namespace": "shop-prod", "replicas": 4}
{"kind": "Service", "labels": {"app": "web", "env": "prod"}, "name": "prod-web", "namespace": "shop-prod", "targets": "prod-web"}
after changing LOG_LEVEL the Deployment refers to prod-web-config-d3d3cb6552
@@requires
def config_hash
def build
hexdigest()[:10]
@@hint
`config_hash` hashes `json.dumps(data, sort_keys=True)` with SHA-256 and keeps `hexdigest()[:10]`. In `build`, make the ConfigMap named `prefix + name + "-" + hash`, then copy each base resource, prefix its name, set the namespace, merge the labels, change the image tag if its repository is in `images`, apply `replicas` by the original name, and point `configMapRef` and `targets` at the new names.
@@solution
import hashlib
import json

base = [
    {"kind": "Deployment", "name": "web", "labels": {"app": "web"}, "replicas": 1,
     "image": "shop/web:1.0.0", "configMapRef": "web-config"},
    {"kind": "Service", "name": "web", "labels": {"app": "web"}, "targets": "web"},
]
overlay = {"namePrefix": "prod-", "namespace": "shop-prod", "labels": {"env": "prod"},
           "images": {"shop/web": "2.4.1"}, "replicas": {"web": 4},
           "configMapGenerator": {"name": "web-config", "data": {"LOG_LEVEL": "warn"}}}

def config_hash(data):
    return hashlib.sha256(json.dumps(data, sort_keys=True).encode()).hexdigest()[:10]

def build(base, overlay):
    prefix = overlay.get("namePrefix", "")
    gen = overlay["configMapGenerator"]
    cm_name = f"{prefix}{gen['name']}-{config_hash(gen['data'])}"
    renamed = {gen["name"]: cm_name}
    out = [{"kind": "ConfigMap", "name": cm_name, "namespace": overlay["namespace"],
            "labels": dict(overlay["labels"]), "data": gen["data"]}]
    for r in base:
        renamed.setdefault(r["name"], prefix + r["name"])
    for r in base:
        new = dict(r, name=prefix + r["name"], namespace=overlay["namespace"],
                   labels={**r["labels"], **overlay["labels"]})
        if "image" in new:
            repo, _, _ = new["image"].partition(":")
            if repo in overlay["images"]:
                new["image"] = f"{repo}:{overlay['images'][repo]}"
        if r["kind"] == "Deployment" and r["name"] in overlay["replicas"]:
            new["replicas"] = overlay["replicas"][r["name"]]
        if "configMapRef" in new:
            new["configMapRef"] = renamed[new["configMapRef"]]
        if "targets" in new:
            new["targets"] = prefix + new["targets"]
        out.append(new)
    return out

for r in build(base, overlay):
    print(json.dumps(r, sort_keys=True))
changed = dict(overlay, configMapGenerator={"name": "web-config", "data": {"LOG_LEVEL": "debug"}})
dep = next(r for r in build(base, changed) if r["kind"] == "Deployment")
print("after changing LOG_LEVEL the Deployment refers to", dep["configMapRef"])
