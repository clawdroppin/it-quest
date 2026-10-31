@@lesson xp-three_tier_kind Three-tier App on a Local Cluster
@@text
On a local kind (or minikube) cluster, deploy a frontend, an API and PostgreSQL: Deployments with probes and resources, a StatefulSet with a PersistentVolumeClaim for the database, Services, an Ingress, a ConfigMap and a Secret, NetworkPolicies so only the API can reach the database, and RBAC for a deployer account. Then break things on purpose and fix them.

**You'll practise:** Deployments and StatefulSets · PVCs · Services and Ingress · NetworkPolicy · RBAC · troubleshooting

## Milestones

1. **Cluster**: `kind create cluster --config kind.yaml` with an ingress-ready node.
2. **Database**: StatefulSet + headless Service + PVC + Secret for the password.
3. **API and frontend**: Deployments with probes, requests and limits, Services, an Ingress.
4. **Lock down**: NetworkPolicies (default deny, then allow) and a deployer ServiceAccount with a Role.
5. **Break and fix**: wrong image tag, wrong selector, failing probe; write down how you found each.

## You're done when

- The app works through the Ingress, the DB survives pod deletion, and your break/fix notes cover three faults.

## Getting help

Open the project, then the 🤖 **AI Helper**: it sees the lesson and your files. Good questions to start with:

- *"My NetworkPolicy blocks DNS too. Why, and how do I allow it?"*
- *"Check my StatefulSet: will the data survive if the pod is deleted?"*

Ask for hints, reviews and explanations rather than finished code: you learn by writing it.
@@project three_tier_kind Three-tier App on a Local Cluster
Create the folder, open it in VS Code, and work through `README.md` one milestone at a time.
@@file README.md
# Three-tier App on a Local Cluster

On a local kind (or minikube) cluster, deploy a frontend, an API and PostgreSQL: Deployments with probes and resources, a StatefulSet with a PersistentVolumeClaim for the database, Services, an Ingress, a ConfigMap and a Secret, NetworkPolicies so only the API can reach the database, and RBAC for a deployer account. Then break things on purpose and fix them.

## Milestones

- [ ] 1. Cluster: `kind create cluster --config kind.yaml` with an ingress-ready node.
- [ ] 2. Database: StatefulSet + headless Service + PVC + Secret for the password.
- [ ] 3. API and frontend: Deployments with probes, requests and limits, Services, an Ingress.
- [ ] 4. Lock down: NetworkPolicies (default deny, then allow) and a deployer ServiceAccount with a Role.
- [ ] 5. Break and fix: wrong image tag, wrong selector, failing probe; write down how you found each.

## Done when

- [ ] The app works through the Ingress, the DB survives pod deletion, and your break/fix notes cover three faults.
@@file kind.yaml
kind: Cluster
apiVersion: kind.x-k8s.io/v1alpha4
nodes:
  - role: control-plane
    extraPortMappings:
      - containerPort: 80
        hostPort: 8080
  - role: worker
@@file db.yaml
apiVersion: v1
kind: Secret
metadata: { name: db, namespace: shop }
stringData: { password: change-me }
---
# TODO: headless Service and StatefulSet for postgres:16 with a volumeClaimTemplate
@@file NOTES.md
# Break and fix notes

| Fault | Symptom | How I found it | Fix |
|---|---|---|---|
