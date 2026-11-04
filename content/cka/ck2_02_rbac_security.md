@@lesson ck2-rbac-security Access Control in Depth: RBAC, ServiceAccounts, kubeconfig and Pod Security
@@text
Cluster architecture, installation and configuration is 25% of the CKA, and RBAC is part of it. You will be asked to create a Role and RoleBinding, give a ServiceAccount specific permissions, check permissions with `kubectl auth can-i`, and work with kubeconfig contexts. This lesson covers how requests are authenticated, authorised and admitted, and the Pod-level security controls.

## The request path

Every API request goes through: **authentication** (who are you: client certificates, bearer tokens, OIDC; ServiceAccount tokens) → **authorisation** (may you: Node, **RBAC**, Webhook authorisers) → **admission control** (mutating, then validating: e.g. NamespaceLifecycle, LimitRanger, ResourceQuota, PodSecurity, webhooks, ValidatingAdmissionPolicy) → persisted to etcd.

Kubernetes has no User objects: users are names in certificates (CN = user, O = groups) or tokens from an identity provider.

## RBAC objects

| Object | Scope | Purpose |
|---|---|---|
| **Role** | namespace | rules: apiGroups, resources, verbs |
| **ClusterRole** | cluster | cluster-scoped resources (nodes, PVs), non-resource URLs, or reusable rules |
| **RoleBinding** | namespace | grants a Role **or a ClusterRole** within one namespace |
| **ClusterRoleBinding** | cluster | grants a ClusterRole everywhere |

Rules are **additive only** (no deny). Verbs: `get, list, watch, create, update, patch, delete, deletecollection`, plus special ones such as `impersonate`, `bind`, `escalate`. Core resources have apiGroup `""`; Deployments are in `apps`. Subresources: `pods/log`, `pods/exec`.

```bash,norun
kubectl create role pod-reader -n dev --verb=get,list,watch --resource=pods,pods/log
kubectl create rolebinding alice-pods -n dev --role=pod-reader --user=alice
kubectl create clusterrole node-viewer --verb=get,list --resource=nodes
kubectl create clusterrolebinding ops-nodes --clusterrole=node-viewer --group=ops
kubectl auth can-i list pods -n dev --as=alice
kubectl auth can-i --list -n dev --as=system:serviceaccount:dev:builder
```

Built-in ClusterRoles: `cluster-admin`, `admin`, `edit`, `view`. A RoleBinding to `view` gives read access in that one namespace.

## ServiceAccounts

- Identity for Pods: `system:serviceaccount:<ns>:<name>`; every namespace has `default`.
- Pods get a short-lived, auto-rotated **projected token** (TokenRequest API) mounted at `/var/run/secrets/kubernetes.io/serviceaccount/`. Set `automountServiceAccountToken: false` where the Pod doesn't call the API.
- `kubectl create serviceaccount builder -n dev`; `kubectl create token builder -n dev` for a temporary token; `spec.serviceAccountName` in the Pod.

## kubeconfig

`~/.kube/config` (or `$KUBECONFIG`) has **clusters** (server, CA), **users** (credentials) and **contexts** (cluster + user + namespace). In the exam, **switch context at the start of every task** as instructed.

```bash,norun
kubectl config get-contexts
kubectl config use-context k8s-prod
kubectl config set-context --current --namespace=dev
```

## Certificates

kubeadm clusters keep PKI in `/etc/kubernetes/pki` (CA, apiserver, etcd...). `kubeadm certs check-expiration`, `kubeadm certs renew all` (then restart control plane static Pods). To add a user: create a key and CSR, submit a **CertificateSigningRequest** object, `kubectl certificate approve`, retrieve the cert.

## Pod and container security

- **securityContext**: `runAsNonRoot`, `runAsUser`, `readOnlyRootFilesystem`, `allowPrivilegeEscalation: false`, `capabilities: {drop: [ALL]}`, `seccompProfile: RuntimeDefault`.
- **Pod Security Admission** (replaced PodSecurityPolicy): namespace labels `pod-security.kubernetes.io/enforce|audit|warn=privileged|baseline|restricted`.
- **NetworkPolicies** restrict traffic; **Secrets** encryption at rest; **audit logging** on the API server.

@@quiz
A RoleBinding in namespace dev references the ClusterRole `view`. What access results?
- Read access in all namespaces
* Read access only in dev
- No access; RoleBindings can't reference ClusterRoles
- Cluster-admin in dev
? The binding's scope wins.

@@quiz
How do you check whether ServiceAccount `builder` in `dev` can create Deployments there?
* kubectl auth can-i create deployments -n dev --as=system:serviceaccount:dev:builder
- kubectl get rolebindings builder
- kubectl describe sa builder --verbs
- kubectl auth whoami builder
? --as impersonates.

@@quiz
Which object is needed to grant read access to Nodes?
- Role and RoleBinding
* ClusterRole (with a ClusterRoleBinding)
- ServiceAccount
- NetworkPolicy
? Nodes are cluster-scoped.

@@quiz
How does RBAC deny a permission granted by another binding?
- With a deny rule
- With a higher-priority Role
* It can't; RBAC is additive, so remove the granting binding
- With a NetworkPolicy
? There are no deny rules.

@@quiz
In which apiGroup are Deployments?
- "" (core)
* apps
- extensions
- batch
? Jobs are in batch.

@@quiz
Which namespace label enforces the restricted Pod Security Standard?
* pod-security.kubernetes.io/enforce=restricted
- podsecuritypolicy=restricted
- security.k8s.io/level=restricted
- psa=restricted
? Also audit and warn modes.

@@flashcards
request path :: authn, authz, admission
user identity :: cert CN; O = groups
Role :: namespaced permissions
ClusterRole :: cluster-wide or reusable
RoleBinding :: grant in one namespace
ClusterRoleBinding :: grant everywhere
RBAC rules :: additive only
core apiGroup :: empty string
Deployments apiGroup :: apps
logs subresource :: pods/log
auth can-i :: check permissions
--as :: impersonate a user
SA username :: system:serviceaccount:ns:name
SA token :: projected, short-lived
automountServiceAccountToken :: false if no API use
kubeconfig context :: cluster + user + namespace
PKI path :: /etc/kubernetes/pki
certs check-expiration :: kubeadm cert dates
CSR approve :: kubectl certificate approve
Pod Security levels :: privileged, baseline, restricted

@@drill
Which built-in ClusterRole gives read-only access to most namespaced objects?
= view
? edit and admin give more.

@@drill
Which kubectl command switches the current context to k8s-prod?
= kubectl config use-context k8s-prod
? Do it at the start of every exam task.

@@exercise Evaluate RBAC requests
Implement an RBAC authoriser. Bindings grant a Role (namespaced) or ClusterRole to users, groups or ServiceAccounts; a RoleBinding applies only in its namespace, a ClusterRoleBinding everywhere. A request `(user, groups, verb, resource, namespace)` is allowed if any rule from any applicable binding matches (`*` is a wildcard for verbs and resources). Print `yes`/`no` like `kubectl auth can-i`, with the granting binding.
@@lang python
@@starter
roles = {
    ("dev", "pod-reader"): [({"get", "list", "watch"}, {"pods", "pods/log"})],
    ("dev", "deployer"): [({"get", "list", "create", "update", "patch"}, {"deployments"}), ({"get", "list"}, {"pods"})],
}
cluster_roles = {
    "view": [({"get", "list", "watch"}, {"pods", "services", "deployments", "configmaps"})],
    "node-viewer": [({"get", "list"}, {"nodes"})],
    "cluster-admin": [({"*"}, {"*"})],
}
bindings = [
    {"name": "alice-pods", "ns": "dev", "role": ("Role", "pod-reader"), "subjects": [("User", "alice")]},
    {"name": "ci-deploy", "ns": "dev", "role": ("Role", "deployer"), "subjects": [("ServiceAccount", "dev:builder")]},
    {"name": "qa-view", "ns": "qa", "role": ("ClusterRole", "view"), "subjects": [("Group", "testers")]},
    {"name": "ops-nodes", "ns": None, "role": ("ClusterRole", "node-viewer"), "subjects": [("Group", "ops")]},
    {"name": "root", "ns": None, "role": ("ClusterRole", "cluster-admin"), "subjects": [("User", "admin")]},
]
requests = [("alice", [], "list", "pods", "dev"), ("alice", [], "delete", "pods", "dev"), ("alice", [], "list", "pods", "prod"),
            ("system:serviceaccount:dev:builder", [], "create", "deployments", "dev"), ("system:serviceaccount:dev:builder", [], "delete", "deployments", "dev"),
            ("bob", ["testers"], "list", "configmaps", "qa"), ("bob", ["testers"], "list", "secrets", "qa"),
            ("carol", ["ops"], "list", "nodes", None), ("admin", [], "delete", "nodes", None)]

def subject_matches(subject, user, groups):
    raise NotImplementedError
@@expected
alice list pods -n dev: yes (via alice-pods)
alice delete pods -n dev: no
alice list pods -n prod: no
system:serviceaccount:dev:builder create deployments -n dev: yes (via ci-deploy)
system:serviceaccount:dev:builder delete deployments -n dev: no
bob list configmaps -n qa: yes (via qa-view)
bob list secrets -n qa: no
carol list nodes (cluster): yes (via ops-nodes)
admin delete nodes (cluster): yes (via root)
@@requires
def subject_matches
"system:serviceaccount:"
"*"
@@hint
A ServiceAccount subject `"dev:builder"` matches the user `"system:serviceaccount:dev:builder"`. A binding applies if it's a ClusterRoleBinding (`ns is None`) or its namespace equals the request's. Look rules up in `roles[(ns, name)]` or `cluster_roles[name]`.
@@solution
roles = {
    ("dev", "pod-reader"): [({"get", "list", "watch"}, {"pods", "pods/log"})],
    ("dev", "deployer"): [({"get", "list", "create", "update", "patch"}, {"deployments"}), ({"get", "list"}, {"pods"})],
}
cluster_roles = {
    "view": [({"get", "list", "watch"}, {"pods", "services", "deployments", "configmaps"})],
    "node-viewer": [({"get", "list"}, {"nodes"})],
    "cluster-admin": [({"*"}, {"*"})],
}
bindings = [
    {"name": "alice-pods", "ns": "dev", "role": ("Role", "pod-reader"), "subjects": [("User", "alice")]},
    {"name": "ci-deploy", "ns": "dev", "role": ("Role", "deployer"), "subjects": [("ServiceAccount", "dev:builder")]},
    {"name": "qa-view", "ns": "qa", "role": ("ClusterRole", "view"), "subjects": [("Group", "testers")]},
    {"name": "ops-nodes", "ns": None, "role": ("ClusterRole", "node-viewer"), "subjects": [("Group", "ops")]},
    {"name": "root", "ns": None, "role": ("ClusterRole", "cluster-admin"), "subjects": [("User", "admin")]},
]
requests = [("alice", [], "list", "pods", "dev"), ("alice", [], "delete", "pods", "dev"), ("alice", [], "list", "pods", "prod"),
            ("system:serviceaccount:dev:builder", [], "create", "deployments", "dev"), ("system:serviceaccount:dev:builder", [], "delete", "deployments", "dev"),
            ("bob", ["testers"], "list", "configmaps", "qa"), ("bob", ["testers"], "list", "secrets", "qa"),
            ("carol", ["ops"], "list", "nodes", None), ("admin", [], "delete", "nodes", None)]

def subject_matches(subject, user, groups):
    kind, name = subject
    if kind == "User":
        return name == user
    if kind == "Group":
        return name in groups
    return user == "system:serviceaccount:" + name

def allowed(user, groups, verb, resource, ns):
    for b in bindings:
        if b["ns"] is not None and b["ns"] != ns:
            continue
        if not any(subject_matches(s, user, groups) for s in b["subjects"]):
            continue
        kind, role = b["role"]
        rules = roles.get((b["ns"], role), []) if kind == "Role" else cluster_roles[role]
        for verbs, resources in rules:
            if (verb in verbs or "*" in verbs) and (resource in resources or "*" in resources):
                return b["name"]
    return None

for user, groups, verb, resource, ns in requests:
    grant = allowed(user, groups, verb, resource, ns)
    where = f"-n {ns}" if ns else "(cluster)"
    answer = f"yes (via {grant})" if grant else "no"
    print(f"{user} {verb} {resource} {where}: {answer}")

@@exercise Check a Pod against Pod Security Standards
Classify Pods against the `baseline` and `restricted` levels (simplified). Baseline forbids `privileged`, `hostNetwork`, `hostPID`, `hostPath` volumes and adding capabilities other than `NET_BIND_SERVICE`. Restricted additionally requires `runAsNonRoot: True`, `allowPrivilegeEscalation: False`, capabilities drop `ALL`, and `seccompProfile` `RuntimeDefault` or `Localhost`. For each Pod, print the highest level it meets and the violations of the next level up.
@@lang python
@@starter
pods = {
    "web": {"runAsNonRoot": True, "allowPrivilegeEscalation": False, "drop": ["ALL"], "add": [], "seccomp": "RuntimeDefault"},
    "legacy": {"runAsNonRoot": False, "allowPrivilegeEscalation": True, "drop": [], "add": [], "seccomp": None},
    "proxy": {"runAsNonRoot": True, "allowPrivilegeEscalation": False, "drop": ["ALL"], "add": ["NET_BIND_SERVICE"], "seccomp": "RuntimeDefault"},
    "agent": {"hostNetwork": True, "hostPath": True, "add": ["SYS_ADMIN"], "drop": [], "seccomp": None},
    "debug": {"privileged": True, "drop": [], "add": [], "seccomp": None},
}

def baseline_violations(pod):
    raise NotImplementedError
@@expected
web: restricted
legacy: baseline (restricted needs: runAsNonRoot, allowPrivilegeEscalation, drop ALL, seccompProfile)
proxy: restricted
agent: privileged (baseline violations: hostNetwork, hostPath, capability SYS_ADMIN)
debug: privileged (baseline violations: privileged)
@@requires
def baseline_violations
def restricted_violations
NET_BIND_SERVICE
@@hint
Use `pod.get("privileged")` so missing keys mean "not set". The level is `restricted` if both lists are empty, `baseline` if only the baseline list is empty, otherwise `privileged`.
@@solution
pods = {
    "web": {"runAsNonRoot": True, "allowPrivilegeEscalation": False, "drop": ["ALL"], "add": [], "seccomp": "RuntimeDefault"},
    "legacy": {"runAsNonRoot": False, "allowPrivilegeEscalation": True, "drop": [], "add": [], "seccomp": None},
    "proxy": {"runAsNonRoot": True, "allowPrivilegeEscalation": False, "drop": ["ALL"], "add": ["NET_BIND_SERVICE"], "seccomp": "RuntimeDefault"},
    "agent": {"hostNetwork": True, "hostPath": True, "add": ["SYS_ADMIN"], "drop": [], "seccomp": None},
    "debug": {"privileged": True, "drop": [], "add": [], "seccomp": None},
}

def baseline_violations(pod):
    v = [key for key in ("privileged", "hostNetwork", "hostPID", "hostPath") if pod.get(key)]
    v += [f"capability {c}" for c in pod.get("add", []) if c != "NET_BIND_SERVICE"]
    return v

def restricted_violations(pod):
    v = []
    if pod.get("runAsNonRoot") is not True:
        v.append("runAsNonRoot")
    if pod.get("allowPrivilegeEscalation") is not False:
        v.append("allowPrivilegeEscalation")
    if "ALL" not in pod.get("drop", []):
        v.append("drop ALL")
    if pod.get("seccomp") not in ("RuntimeDefault", "Localhost"):
        v.append("seccompProfile")
    return v

for name, pod in pods.items():
    base, restricted = baseline_violations(pod), restricted_violations(pod)
    if not base and not restricted:
        print(f"{name}: restricted")
    elif not base:
        print(f"{name}: baseline (restricted needs: {', '.join(restricted)})")
    else:
        print(f"{name}: privileged (baseline violations: {', '.join(base)})")
