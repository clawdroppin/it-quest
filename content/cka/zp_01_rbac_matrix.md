@@lesson xp-rbac_matrix RBAC Permission Matrix
@@text
`kubectl auth can-i` answers one question at a time. Export the cluster's Roles, ClusterRoles and bindings as JSON and build the whole matrix: for each user, group and ServiceAccount, which verbs on which resources in which namespaces. Then highlight dangerous grants: `*` verbs, secrets access, and cluster-admin bindings.

**You'll practise:** Role, ClusterRole, RoleBinding, ClusterRoleBinding · aggregation of rules · JSON from kubectl · least privilege

## Milestones

1. **Export**: `kubectl get roles,clusterroles,rolebindings,clusterrolebindings -A -o json > rbac.json`.
2. **Resolve**: for each binding, find its role's rules.
3. **Matrix**: subject → namespace → resource → verbs.
4. **Danger**: wildcards, `get/list secrets`, `create pods/exec`, cluster-admin.
5. **Check**: compare a few cells with `kubectl auth can-i --as`.

## You're done when

- The matrix agrees with `kubectl auth can-i` on the cells you checked.

## Getting help

Open the project, then the 🤖 **AI Helper**: it sees the lesson and your files. Good questions to start with:

- *"Why is `create pods/exec` as powerful as reading secrets?"*
- *"How does a ClusterRole bound with a RoleBinding behave?"*

Ask for hints, reviews and explanations rather than finished code: you learn by writing it.
@@project rbac_matrix RBAC Permission Matrix
Create the folder, open it in VS Code, and work through `README.md` one milestone at a time.
@@file README.md
# RBAC Permission Matrix

`kubectl auth can-i` answers one question at a time. Export the cluster's Roles, ClusterRoles and bindings as JSON and build the whole matrix: for each user, group and ServiceAccount, which verbs on which resources in which namespaces. Then highlight dangerous grants: `*` verbs, secrets access, and cluster-admin bindings.

## Milestones

- [ ] 1. Export: `kubectl get roles,clusterroles,rolebindings,clusterrolebindings -A -o json > rbac.json`.
- [ ] 2. Resolve: for each binding, find its role's rules.
- [ ] 3. Matrix: subject → namespace → resource → verbs.
- [ ] 4. Danger: wildcards, `get/list secrets`, `create pods/exec`, cluster-admin.
- [ ] 5. Check: compare a few cells with `kubectl auth can-i --as`.

## Done when

- [ ] The matrix agrees with `kubectl auth can-i` on the cells you checked.
@@file rbac.py
"""RBAC matrix: python rbac.py rbac.json"""
import json
import sys

data = json.load(open(sys.argv[1] if len(sys.argv) > 1 else "rbac.json", encoding="utf-8"))
roles = {}
for item in data["items"]:
    if item["kind"] in ("Role", "ClusterRole"):
        key = (item["kind"], item["metadata"].get("namespace"), item["metadata"]["name"])
        roles[key] = item.get("rules", [])
print(len(roles), "roles")
# TODO: resolve bindings to rules, build the matrix, flag dangerous grants
@@file rbac.json
{ "items": [
  { "kind": "Role", "metadata": { "name": "pod-reader", "namespace": "shop" },
    "rules": [ { "apiGroups": [""], "resources": ["pods"], "verbs": ["get", "list", "watch"] } ] },
  { "kind": "RoleBinding", "metadata": { "name": "ann-reads", "namespace": "shop" },
    "roleRef": { "kind": "Role", "name": "pod-reader" }, "subjects": [ { "kind": "User", "name": "ann" } ] },
  { "kind": "ClusterRoleBinding", "metadata": { "name": "ci-admin" },
    "roleRef": { "kind": "ClusterRole", "name": "cluster-admin" }, "subjects": [ { "kind": "ServiceAccount", "name": "ci", "namespace": "build" } ] }
] }
