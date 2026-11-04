@@lesson ck1-storage-deep Storage in Depth: Volumes, PersistentVolumes, Claims and StorageClasses
@@text
Storage is 10% of the CKA, and the tasks are predictable: create a PersistentVolume, bind a claim to it, mount it in a Pod, pick or create a StorageClass, expand a volume. The marks are lost on details: access modes, capacity, storageClassName and reclaim policies. This lesson goes through them carefully.

## Volume types in a Pod

- **emptyDir**: scratch space that lives as long as the Pod (`medium: Memory` for tmpfs); shared between the Pod's containers.
- **hostPath**: a path on the node (node-specific, a security risk; fine for labs and DaemonSets that need node files).
- **configMap / secret / downwardAPI / projected**: config as files.
- **persistentVolumeClaim**: durable storage via the PV subsystem.
- **ephemeral** (generic ephemeral volumes): a PVC created and deleted with the Pod.

## PersistentVolumes and claims

A **PersistentVolume (PV)** is a cluster-scoped piece of storage; a **PersistentVolumeClaim (PVC)** is a namespaced request. The control plane **binds** a PVC to a PV that satisfies it, one-to-one.

```yaml,norun
apiVersion: v1
kind: PersistentVolume
metadata: {name: pv-logs}
spec:
  capacity: {storage: 5Gi}
  accessModes: [ReadWriteOnce]
  persistentVolumeReclaimPolicy: Retain
  storageClassName: manual
  hostPath: {path: /mnt/logs}
---
apiVersion: v1
kind: PersistentVolumeClaim
metadata: {name: logs, namespace: app}
spec:
  accessModes: [ReadWriteOnce]
  storageClassName: manual
  resources: {requests: {storage: 2Gi}}
```

A PVC binds to a PV when:
- the **storageClassName** is equal (both empty counts as equal; a PVC with no class field uses the **default StorageClass**, if there is one),
- the PV offers **all** requested **access modes**,
- the PV's **capacity ≥ the request** (the smallest suitable PV wins, and the PVC gets the whole PV),
- any **selector** (labels) or `volumeName` matches,
- the PV is **Available** (not already Bound or Released).

Otherwise the PVC stays **Pending**: `kubectl describe pvc` explains why.

## Access modes

| Mode | Short | Meaning |
|---|---|---|
| ReadWriteOnce | RWO | read-write by **one node** (several Pods on that node can share) |
| ReadOnlyMany | ROX | read-only by many nodes |
| ReadWriteMany | RWX | read-write by many nodes (NFS, CephFS, Azure Files, EFS...) |
| ReadWriteOncePod | RWOP | read-write by **one Pod** |

## Reclaim policies and phases

- **Delete** (default for dynamically provisioned volumes): deleting the PVC deletes the PV and the backing storage.
- **Retain**: the PV becomes **Released** and keeps its data; an admin must clean it (and remove `claimRef`) before reuse.
- Phases: **Available → Bound → Released** (→ **Failed**).

## StorageClasses and dynamic provisioning

```yaml,norun
apiVersion: storage.k8s.io/v1
kind: StorageClass
metadata:
  name: fast
  annotations: {storageclass.kubernetes.io/is-default-class: "true"}
provisioner: ebs.csi.aws.com
parameters: {type: gp3}
reclaimPolicy: Delete
volumeBindingMode: WaitForFirstConsumer
allowVolumeExpansion: true
```

- A PVC naming a class with a **provisioner** gets a new PV created automatically (via a **CSI driver**).
- **WaitForFirstConsumer** delays binding until a Pod uses the PVC, so the volume is created in the Pod's zone; **Immediate** binds at once. `kubernetes.io/no-provisioner` + WaitForFirstConsumer is used for local volumes.
- **Expansion**: with `allowVolumeExpansion: true`, edit the PVC's `resources.requests.storage` upwards (shrinking isn't allowed).
- **VolumeSnapshots** (CSI) for point-in-time copies.

## Using a claim

```yaml,norun
spec:
  volumes:
  - name: logs
    persistentVolumeClaim: {claimName: logs}
  containers:
  - name: app
    image: busybox:1.36
    volumeMounts: [{name: logs, mountPath: /var/log/app}]
```

StatefulSets use `volumeClaimTemplates` so each replica gets its own PVC (`data-db-0`, `data-db-1`); deleting the StatefulSet doesn't delete those PVCs by default.

@@quiz
A PVC requests 2Gi RWO with storageClassName manual. Which PV does it bind to?
- 1Gi RWO manual
* 5Gi RWO manual
- 10Gi RWO fast
- 5Gi ROX manual
? Same class, all modes, enough capacity.

@@quiz
A PVC asks for 2Gi and binds to a 5Gi PV. How much storage does it get?
- 2Gi
* 5Gi (the whole PV)
- 3Gi
- It fails to bind
? Binding is one-to-one.

@@quiz
A PV with reclaim policy Retain is released when its PVC is deleted. What happens to it?
- It's deleted
- It's immediately reused by the next PVC
* It stays Released with its data until an admin cleans it up
- It becomes Available automatically
? Delete removes the storage; Retain keeps it.

@@quiz
Several Pods on different nodes must write to the same volume. Which access mode is needed?
- ReadWriteOnce
- ReadOnlyMany
* ReadWriteMany
- ReadWriteOncePod
? Needs a shared filesystem backend.

@@quiz
Why use volumeBindingMode WaitForFirstConsumer?
* So the volume is provisioned in the zone where the Pod is scheduled
- To speed up binding
- To allow ReadWriteMany
- To retain data
? Avoids a volume in a zone the Pod can't use.

@@quiz
How do you grow a PVC from 10Gi to 20Gi?
- Delete and recreate it
* Edit the PVC's storage request, if its StorageClass allows volume expansion
- Edit the PV's capacity only
- Scale the StatefulSet
? Shrinking isn't supported.

@@flashcards
PV :: cluster-scoped storage
PVC :: namespaced request
binding :: one PVC to one PV
emptyDir :: lives with the Pod
hostPath :: node path, risky
RWO :: one node read-write
RWX :: many nodes read-write
ROX :: many nodes read-only
RWOP :: one Pod read-write
Retain :: PV kept as Released
Delete :: PV and storage removed
PV phases :: Available, Bound, Released, Failed
StorageClass :: provisioner + parameters
default StorageClass :: is-default-class annotation
WaitForFirstConsumer :: bind when a Pod uses it
allowVolumeExpansion :: grow PVCs
volumeClaimTemplates :: PVC per StatefulSet Pod
PVC Pending :: describe it for the reason

@@drill
What is the short name of the access mode that allows read-write by a single node?
= RWO
= ReadWriteOnce
? RWOP is per Pod.

@@drill
Which reclaim policy keeps the data after the claim is deleted?
= Retain
= retain
? Delete is the default for dynamic volumes.

@@exercise Bind claims to volumes
Implement PV binding. Process PVCs in order. For each, find Available PVs with the same storageClassName (a PVC with class `None` uses the default class `standard`), all requested access modes, and capacity ≥ request; bind to the smallest. If none match but the class has a provisioner, dynamically provision a PV of exactly the requested size (name `pvc-<claim>`). Otherwise the PVC stays Pending; print why: the first of `no PV with class`, `no PV with access modes`, `no PV large enough`.
@@lang python
@@starter
DEFAULT_CLASS = "standard"
provisioners = {"standard": "ebs.csi.aws.com", "manual": None}
pvs = [
    {"name": "pv-a", "class": "manual", "modes": {"RWO"}, "gi": 5, "phase": "Available"},
    {"name": "pv-b", "class": "manual", "modes": {"RWO", "ROX"}, "gi": 10, "phase": "Available"},
    {"name": "pv-c", "class": "manual", "modes": {"RWX"}, "gi": 20, "phase": "Released"},
    {"name": "pv-d", "class": "manual", "modes": {"RWX"}, "gi": 50, "phase": "Available"},
    {"name": "pv-e", "class": "manual", "modes": {"RWO"}, "gi": 20, "phase": "Available"},
]
pvcs = [("logs", "manual", {"RWO"}, 2), ("data", "manual", {"RWO"}, 8), ("shared", "manual", {"RWX"}, 30),
        ("cache", None, {"RWO"}, 15), ("big", "manual", {"RWO"}, 100), ("reports", "manual", {"ROX"}, 1), ("nfs", "nfs", {"RWX"}, 5)]

def candidates(cls, modes, size):
    raise NotImplementedError
@@expected
logs       2Gi manual   -> Bound to pv-a (5Gi)
data       8Gi manual   -> Bound to pv-b (10Gi)
shared    30Gi manual   -> Bound to pv-d (50Gi)
cache     15Gi standard -> Bound to pvc-cache (provisioned by ebs.csi.aws.com)
big      100Gi manual   -> Pending: no PV large enough
reports    1Gi manual   -> Pending: no PV with access modes
nfs        5Gi nfs      -> Pending: no PV with class
@@requires
def candidates
DEFAULT_CLASS
issubset(
@@hint
Filter step by step so you can report the reason: `same = [pv for pv in available if pv["class"] == cls]`, then `modes.issubset(pv["modes"])`, then capacity. `min(fits, key=lambda pv: pv["gi"])` picks the smallest.
@@solution
DEFAULT_CLASS = "standard"
provisioners = {"standard": "ebs.csi.aws.com", "manual": None}
pvs = [
    {"name": "pv-a", "class": "manual", "modes": {"RWO"}, "gi": 5, "phase": "Available"},
    {"name": "pv-b", "class": "manual", "modes": {"RWO", "ROX"}, "gi": 10, "phase": "Available"},
    {"name": "pv-c", "class": "manual", "modes": {"RWX"}, "gi": 20, "phase": "Released"},
    {"name": "pv-d", "class": "manual", "modes": {"RWX"}, "gi": 50, "phase": "Available"},
    {"name": "pv-e", "class": "manual", "modes": {"RWO"}, "gi": 20, "phase": "Available"},
]
pvcs = [("logs", "manual", {"RWO"}, 2), ("data", "manual", {"RWO"}, 8), ("shared", "manual", {"RWX"}, 30),
        ("cache", None, {"RWO"}, 15), ("big", "manual", {"RWO"}, 100), ("reports", "manual", {"ROX"}, 1), ("nfs", "nfs", {"RWX"}, 5)]

def candidates(cls, modes, size):
    available = [pv for pv in pvs if pv["phase"] == "Available"]
    same = [pv for pv in available if pv["class"] == cls]
    if not same:
        return [], "no PV with class"
    moded = [pv for pv in same if modes.issubset(pv["modes"])]
    if not moded:
        return [], "no PV with access modes"
    fits = [pv for pv in moded if pv["gi"] >= size]
    if not fits:
        return [], "no PV large enough"
    return fits, None

for claim, cls, modes, size in pvcs:
    cls = cls or DEFAULT_CLASS
    fits, why = candidates(cls, modes, size)
    if fits:
        pv = min(fits, key=lambda pv: pv["gi"])
        pv["phase"] = "Bound"
        print(f"{claim:<8} {size:>3}Gi {cls:<8} -> Bound to {pv['name']} ({pv['gi']}Gi)")
    elif provisioners.get(cls):
        pvs.append({"name": f"pvc-{claim}", "class": cls, "modes": set(modes), "gi": size, "phase": "Bound"})
        print(f"{claim:<8} {size:>3}Gi {cls:<8} -> Bound to pvc-{claim} (provisioned by {provisioners[cls]})")
    else:
        print(f"{claim:<8} {size:>3}Gi {cls:<8} -> Pending: {why}")

@@exercise Reclaim policies after deletions
Bound PVs have a reclaim policy. Process deletions of PVCs: `Delete` removes the PV and its storage; `Retain` leaves it `Released` (data kept). Then an admin "recycles" a Released PV by clearing its claimRef, which makes it Available again. Print each event and the final PV table, sorted by name.
@@lang python
@@starter
pvs = {
    "pv-db": {"phase": "Bound", "claim": "app/db", "policy": "Retain"},
    "pvc-7f3a": {"phase": "Bound", "claim": "app/cache", "policy": "Delete"},
    "pv-logs": {"phase": "Bound", "claim": "ops/logs", "policy": "Retain"},
    "pvc-91bc": {"phase": "Bound", "claim": "web/uploads", "policy": "Delete"},
}
events = [("delete-pvc", "app/db"), ("delete-pvc", "app/cache"), ("delete-pvc", "ops/logs"), ("clear-claimref", "pv-logs"), ("clear-claimref", "pvc-91bc")]

def delete_claim(claim):
    raise NotImplementedError
@@expected
PVC app/db deleted: pv-db Released, data kept
PVC app/cache deleted: pvc-7f3a and its storage deleted
PVC ops/logs deleted: pv-logs Released, data kept
pv-logs: claimRef cleared, Available again
pvc-91bc: error, phase is Bound, not Released
NAME      PHASE      CLAIM       POLICY
pv-db     Released   app/db      Retain
pv-logs   Available  -           Retain
pvc-91bc  Bound      web/uploads Delete
@@requires
def delete_claim
"Released"
"Retain"
@@hint
Find the PV whose `claim` equals the deleted claim. Only a Released PV can have its claimRef cleared; anything else is an error to print.
@@solution
pvs = {
    "pv-db": {"phase": "Bound", "claim": "app/db", "policy": "Retain"},
    "pvc-7f3a": {"phase": "Bound", "claim": "app/cache", "policy": "Delete"},
    "pv-logs": {"phase": "Bound", "claim": "ops/logs", "policy": "Retain"},
    "pvc-91bc": {"phase": "Bound", "claim": "web/uploads", "policy": "Delete"},
}
events = [("delete-pvc", "app/db"), ("delete-pvc", "app/cache"), ("delete-pvc", "ops/logs"), ("clear-claimref", "pv-logs"), ("clear-claimref", "pvc-91bc")]

def delete_claim(claim):
    name = next(n for n, pv in pvs.items() if pv["claim"] == claim)
    pv = pvs[name]
    if pv["policy"] == "Retain":
        pv["phase"] = "Released"
        print(f"PVC {claim} deleted: {name} Released, data kept")
    else:
        del pvs[name]
        print(f"PVC {claim} deleted: {name} and its storage deleted")

for action, target in events:
    if action == "delete-pvc":
        delete_claim(target)
    elif pvs[target]["phase"] == "Released":
        pvs[target].update(phase="Available", claim=None)
        print(f"{target}: claimRef cleared, Available again")
    else:
        print(f"{target}: error, phase is {pvs[target]['phase']}, not Released")

print("NAME      PHASE      CLAIM       POLICY")
for name, pv in sorted(pvs.items()):
    print(f"{name:<9} {pv['phase']:<10} {pv['claim'] or '-':<11} {pv['policy']}")
