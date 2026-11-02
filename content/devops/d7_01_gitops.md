@@lesson d7-gitops Configuration Management and GitOps: Ansible, Helm, Kustomize and Argo CD
@@concepts maps, automation, files
@@lang python
@@text
Terraform creates infrastructure. Something else has to **configure** servers and **deploy** applications onto them, again and again, without drift. This lesson covers the tools for that: **Ansible** for servers, **Helm** and **Kustomize** for Kubernetes manifests, and **GitOps** with Argo CD or Flux, where Git becomes the single source of truth and a controller keeps the cluster matching it.

## Idempotency: the idea behind all of it

An **idempotent** operation gives the same result no matter how many times you run it. `echo "PermitRootLogin no" >> sshd_config` is not idempotent: run it three times and you get three lines. "Make sure this line is present" is idempotent: the first run adds it, later runs change nothing and report *ok*. Every tool in this lesson is built from idempotent, **declarative** steps: you describe the desired state, and the tool works out what (if anything) to change.

## Ansible

Ansible connects over **SSH** (WinRM for Windows), needs no agent on the servers, and runs **modules** (small idempotent programs) described in YAML **playbooks**.

```ini
# inventory.ini
[web]
web1.example.com
web2.example.com

[db]
db1.example.com ansible_user=admin

[all:vars]
ansible_python_interpreter=/usr/bin/python3
```

```yaml
# site.yml
- name: Configure web servers
  hosts: web
  become: true                              # sudo
  vars:
    app_port: 8080
  tasks:
    - name: Install nginx
      ansible.builtin.apt:
        name: nginx
        state: present
        update_cache: true

    - name: Disable root SSH login
      ansible.builtin.lineinfile:
        path: /etc/ssh/sshd_config
        regexp: '^#?PermitRootLogin'
        line: PermitRootLogin no
      notify: Restart sshd                  # only if this task changed something

    - name: Site config from a template
      ansible.builtin.template:
        src: templates/site.conf.j2         # Jinja2: {{ app_port }}
        dest: /etc/nginx/conf.d/site.conf
      notify: Reload nginx

    - name: nginx is running and starts at boot
      ansible.builtin.service:
        name: nginx
        state: started
        enabled: true

  handlers:
    - name: Restart sshd
      ansible.builtin.service: { name: ssh, state: restarted }
    - name: Reload nginx
      ansible.builtin.service: { name: nginx, state: reloaded }
```

```text
ansible all -i inventory.ini -m ping                  # can I reach everything?
ansible-playbook -i inventory.ini site.yml --check --diff   # dry run: what would change
ansible-playbook -i inventory.ini site.yml --limit web1     # just one host
ansible-vault encrypt group_vars/db/secrets.yml       # encrypt secrets in the repo
ansible-galaxy init roles/nginx                       # a reusable role skeleton
```

- **Roles** package tasks, handlers, templates and default variables for one job (`roles/nginx`), reusable across playbooks. **Collections** on Ansible Galaxy share them (`community.general`, `amazon.aws`).
- **Variables** layer from many places (role defaults, `group_vars/web.yml`, `host_vars/web1.yml`, `-e` on the command line, which wins).
- The output shows each task as `ok` (nothing to do), `changed` or `failed`. Running the same playbook twice should give **zero changes** the second time: that's the idempotency test.
- **Dynamic inventory** plugins ask AWS, Azure or VMware which servers exist, so you never maintain lists by hand.

## Helm: packaging Kubernetes apps

Real applications need many manifests (Deployment, Service, Ingress, ConfigMap, HPA…) that differ only slightly per environment. A **Helm chart** is a package of templated manifests plus default **values**:

```text
mychart/
  Chart.yaml            # name, version, appVersion, dependencies
  values.yaml           # defaults
  templates/
    deployment.yaml     # uses {{ .Values.replicas }}, {{ .Values.image.tag }}
    service.yaml
    _helpers.tpl
```

```yaml
# templates/deployment.yaml (a fragment)
spec:
  replicas: {{ .Values.replicas }}
  template:
    spec:
      containers:
        - name: web
          image: "{{ .Values.image.repository }}:{{ .Values.image.tag }}"
          resources: {{- toYaml .Values.resources | nindent 12 }}
```

```text
helm repo add bitnami https://charts.bitnami.com/bitnami
helm install web ./mychart -n web --create-namespace -f values-prod.yaml --set image.tag=1.4.2
helm upgrade --install web ./mychart -f values-prod.yaml      # install or upgrade: idempotent
helm diff upgrade web ./mychart -f values-prod.yaml           # (plugin) what would change
helm rollback web 3                                           # back to revision 3
helm template ./mychart -f values-prod.yaml                   # render to plain YAML
```

Values are **deep-merged**: `values-prod.yaml` only needs the keys that differ from `values.yaml`. Nested maps merge key by key; lists and plain values are replaced. The first exercise implements this merge.

## Kustomize: patching plain YAML

Kustomize (built into `kubectl apply -k`) uses no templates: a **base** of ordinary manifests plus **overlays** that patch it per environment.

```text
base/          deployment.yaml  service.yaml  kustomization.yaml
overlays/
  staging/     kustomization.yaml   (namePrefix: staging-, replicas: 1)
  prod/        kustomization.yaml   (replicas: 4, image tag, a resource-limits patch)
```

```yaml
# overlays/prod/kustomization.yaml
resources: [../../base]
namespace: web-prod
images:
  - name: ghcr.io/acme/web
    newTag: "1.4.2"
replicas:
  - name: web
    count: 4
patches:
  - path: limits.yaml
```

Helm suits distributing software to others (and third-party apps); Kustomize suits your own apps. Many teams use both (Helm for Prometheus and ingress controllers, Kustomize for their services).

## GitOps

In a classic pipeline, CI **pushes** changes to the cluster with `kubectl`/`helm` and cluster credentials. With **GitOps**:

1. The desired state of every environment lives in a **Git repository** (manifests, Helm values, Kustomize overlays).
2. A controller **inside the cluster** (Argo CD or Flux) **pulls** that repository and applies it.
3. It continuously compares the live cluster to Git (**drift detection**) and reverts manual changes (**self-healing**).
4. Deploying = merging a pull request (e.g. bumping an image tag). Rolling back = `git revert`.

```yaml
# An Argo CD Application
apiVersion: argoproj.io/v1alpha1
kind: Application
metadata: { name: web-prod, namespace: argocd }
spec:
  project: default
  source:
    repoURL: https://github.com/acme/deploy.git
    path: apps/web/overlays/prod
    targetRevision: main
  destination: { server: https://kubernetes.default.svc, namespace: web-prod }
  syncPolicy:
    automated: { prune: true, selfHeal: true }   # delete removed objects, undo manual edits
```

Benefits: every change is reviewed and audited in Git, CI never needs cluster credentials, and disaster recovery is "point a new cluster at the repo". A typical split: the **app repo** (code + Dockerfile; CI builds and pushes the image) and the **config repo** (CI or a bot opens a PR there with the new image tag).

## Progressive delivery

- **Blue/green**: run the new version beside the old, switch all traffic at once, switch back if it fails.
- **Canary**: send 5%, then 25%, 50%, 100% of traffic to the new version, checking metrics (error rate, latency from Prometheus) at each step and rolling back automatically if they get worse. Tools: **Argo Rollouts**, **Flagger**, service meshes (Istio, Linkerd).
- **Feature flags** (LaunchDarkly, Unleash, OpenFeature): ship code switched off, and turn it on per user group without a deploy.

@@quiz
What makes a task idempotent?
- It runs very fast
* Running it once or ten times leaves the system in the same state
- It never fails
- It only runs on one server
? "Ensure the line is present" is idempotent; "append the line" isn't.

@@quiz
What does `notify` do in an Ansible task?
- Sends an email to the admin
* Triggers a handler (e.g. restart a service) at the end, only if the task changed something
- Logs the task output
- Runs the task on every host
? Handlers run once even if several tasks notified them.

@@quiz
In GitOps, how does a change reach the production cluster?
- An engineer runs `kubectl apply` from their laptop
- CI pushes it with cluster admin credentials
* A pull request is merged into the config repository, and a controller in the cluster pulls and applies it
- The cluster emails the change for approval
? Argo CD or Flux sync the cluster to Git and revert drift.

@@quiz
When should you prefer Helm over Kustomize?
* To package an app for others to install with configurable values, or to install third-party software
- Never: Helm is deprecated
- When you don't want any templating
- Only for Windows containers
? Kustomize patches plain YAML for your own environments; Helm templates and versions packages.

@@quiz
What does a canary release do?
- Deploys at night only
* Sends a small, growing share of traffic to the new version while watching metrics, rolling back if they get worse
- Runs two versions forever
- Tests the release in a separate cluster
? Argo Rollouts and Flagger automate the steps and the metric checks.

@@flashcards
idempotent :: same result however many times it runs
declarative :: describe the desired state, the tool works out the steps
Ansible :: agentless configuration over SSH with YAML playbooks
inventory :: the list (or dynamic plugin) of hosts and groups
module :: an idempotent unit of work: apt, template, service, lineinfile
handler :: runs once at the end when notified by a changed task
role :: reusable bundle of tasks, handlers, templates and defaults
`--check --diff` :: Ansible dry run showing what would change
`ansible-vault` :: encrypt secrets in the repository
Helm chart :: templated Kubernetes manifests + values.yaml
`helm upgrade --install` :: install or upgrade in one idempotent command
`helm rollback web 3` :: go back to a previous release revision
values deep merge :: maps merge key by key; lists and scalars are replaced
Kustomize :: base manifests + per-environment overlays and patches
GitOps :: Git is the source of truth; a controller pulls and syncs
Argo CD / Flux :: GitOps controllers
`selfHeal: true` :: undo manual changes to the cluster
drift :: the live state no longer matches the declared state
blue/green :: switch all traffic between two full versions
canary :: shift traffic gradually while checking metrics
feature flag :: turn code on or off without deploying

@@exercise Merge Helm values
Write `merge(base, override)` that returns a **new** dict, merging like Helm values files: if both have a key whose values are both dicts, merge them recursively; otherwise the override's value wins (lists are replaced, not combined). A value of `None` in the override **removes** the key. Don't change either input.
@@starter
def merge(base, override):
    pass


defaults = {"replicas": 1, "image": {"repository": "ghcr.io/acme/web", "tag": "latest"}, "env": ["DEBUG=1"]}
prod = {"replicas": 4, "image": {"tag": "1.4.2"}, "env": ["LOG=info"]}
print(merge(defaults, prod))
@@tests
defaults = {"replicas": 1, "image": {"repository": "ghcr.io/acme/web", "tag": "latest"}, "env": ["DEBUG=1"]}
prod = {"replicas": 4, "image": {"tag": "1.4.2"}, "env": ["LOG=info"]}
check(merge(defaults, prod), {"replicas": 4, "image": {"repository": "ghcr.io/acme/web", "tag": "1.4.2"}, "env": ["LOG=info"]}, "prod values")
check(defaults["image"]["tag"], "latest", "the base is unchanged")
check(merge({"a": {"b": {"c": 1, "d": 2}}}, {"a": {"b": {"d": 3}}}), {"a": {"b": {"c": 1, "d": 3}}}, "deep nesting")
check(merge({"ingress": {"enabled": True, "host": "x"}}, {"ingress": None}), {}, "None removes a key")
check(merge({"a": 1}, {"b": {"c": 2}}), {"a": 1, "b": {"c": 2}}, "new keys are added")
check(merge({"a": {"x": 1}}, {"a": 5}), {"a": 5}, "a scalar replaces a dict")
@@hint
Start with `result = dict(base)`. For each `key, value` in the override: if `value is None`, `result.pop(key, None)`; elif both `result.get(key)` and `value` are dicts, `result[key] = merge(result[key], value)`; else `result[key] = value`. Recursion copies nested dicts as it goes.
@@solution
def merge(base, override):
    result = dict(base)
    for key, value in override.items():
        if value is None:
            result.pop(key, None)
        elif isinstance(value, dict) and isinstance(result.get(key), dict):
            result[key] = merge(result[key], value)
        else:
            result[key] = value
    return result


defaults = {"replicas": 1, "image": {"repository": "ghcr.io/acme/web", "tag": "latest"}, "env": ["DEBUG=1"]}
prod = {"replicas": 4, "image": {"tag": "1.4.2"}, "env": ["LOG=info"]}
print(merge(defaults, prod))

@@exercise Detect drift
A GitOps controller compares what Git declares with what's running. Write `flatten(d)`, which turns nested dicts into one level with dotted keys (`{"a": {"b": 1}}` → `{"a.b": 1}`; lists and other values stay as they are), and `drift(desired, live)`, which returns a sorted list of differences: `"+ key"` for keys only in `desired` (missing from the cluster), `"- key"` for keys only in `live` (added by hand), and `"~ key: live -> desired"` for different values (show both with `repr`).
@@starter
def flatten(d, prefix=""):
    pass


def drift(desired, live):
    pass


desired = {"spec": {"replicas": 3, "image": "web:1.4.2"}, "metadata": {"labels": {"app": "web"}}}
live = {"spec": {"replicas": 5, "image": "web:1.4.2"}, "metadata": {"labels": {"app": "web", "debug": "true"}}}
for line in drift(desired, live):
    print(line)
@@tests
check(flatten({"a": {"b": 1, "c": {"d": [1, 2]}}, "e": "x"}), {"a.b": 1, "a.c.d": [1, 2], "e": "x"}, "flatten")
desired = {"spec": {"replicas": 3, "image": "web:1.4.2"}, "metadata": {"labels": {"app": "web"}}}
live = {"spec": {"replicas": 5, "image": "web:1.4.2"}, "metadata": {"labels": {"app": "web", "debug": "true"}}}
check(drift(desired, live), ["- metadata.labels.debug", "~ spec.replicas: 5 -> 3"], "a manual scale and a label")
check(drift(desired, desired), [], "in sync")
check(drift({"a": "x", "b": 1}, {"a": "y"}), ["+ b", "~ a: 'y' -> 'x'"], "missing key and changed string")
@@hint
`flatten`: for each key, build `path = prefix + key`; if the value is a dict, `out.update(flatten(value, path + "."))`, else `out[path] = value`. In `drift`, flatten both and compare the key sets; sort the final list of strings.
@@solution
def flatten(d, prefix=""):
    out = {}
    for key, value in d.items():
        path = prefix + key
        if isinstance(value, dict):
            out.update(flatten(value, path + "."))
        else:
            out[path] = value
    return out


def drift(desired, live):
    want, have = flatten(desired), flatten(live)
    lines = [f"+ {k}" for k in want if k not in have]
    lines += [f"- {k}" for k in have if k not in want]
    lines += [f"~ {k}: {have[k]!r} -> {want[k]!r}" for k in want if k in have and want[k] != have[k]]
    return sorted(lines)


desired = {"spec": {"replicas": 3, "image": "web:1.4.2"}, "metadata": {"labels": {"app": "web"}}}
live = {"spec": {"replicas": 5, "image": "web:1.4.2"}, "metadata": {"labels": {"app": "web", "debug": "true"}}}
for line in drift(desired, live):
    print(line)

@@exercise An idempotent lineinfile
Write `ensure_line(text, pattern, line)`, a tiny version of Ansible's `lineinfile`. `text` is a file's contents (lines separated by `\n`, ending with `\n` if not empty). If some line matches the regular expression `pattern` (with `re.search`), replace the **last** matching line with `line`; otherwise append `line` at the end. Return `(new_text, changed)`, where `changed` is `False` when the result is identical to the input. Running it twice must change nothing the second time.
@@starter
import re


def ensure_line(text, pattern, line):
    pass


config = "Port 22\n#PermitRootLogin yes\nPasswordAuthentication yes\n"
config, changed = ensure_line(config, r"^#?PermitRootLogin", "PermitRootLogin no")
print(changed)
print(config, end="")
config, changed = ensure_line(config, r"^#?PermitRootLogin", "PermitRootLogin no")
print(changed)
@@tests
cfg = "Port 22\n#PermitRootLogin yes\nPasswordAuthentication yes\n"
new, changed = ensure_line(cfg, r"^#?PermitRootLogin", "PermitRootLogin no")
check(new, "Port 22\nPermitRootLogin no\nPasswordAuthentication yes\n", "replaces the commented line")
check(changed, True, "reports a change")
check(ensure_line(new, r"^#?PermitRootLogin", "PermitRootLogin no"), (new, False), "the second run changes nothing")
check(ensure_line("Port 22\n", r"^MaxAuthTries", "MaxAuthTries 3"), ("Port 22\nMaxAuthTries 3\n", True), "appends when missing")
check(ensure_line("", r"^x", "x=1"), ("x=1\n", True), "an empty file")
check(ensure_line("a=1\na=2\n", r"^a=", "a=9")[0], "a=1\na=9\n", "the last match is replaced")
@@hint
`lines = text.splitlines()`. Find the indexes where `re.search(pattern, l)`. If there are any, `lines[matches[-1]] = line`, else `lines.append(line)`. Join with `"\n"` and add a final `"\n"`. `changed = new_text != text`.
@@solution
import re


def ensure_line(text, pattern, line):
    lines = text.splitlines()
    matches = [i for i, l in enumerate(lines) if re.search(pattern, l)]
    if matches:
        lines[matches[-1]] = line
    else:
        lines.append(line)
    new_text = "\n".join(lines) + "\n"
    return new_text, new_text != text


config = "Port 22\n#PermitRootLogin yes\nPasswordAuthentication yes\n"
config, changed = ensure_line(config, r"^#?PermitRootLogin", "PermitRootLogin no")
print(changed)
print(config, end="")
config, changed = ensure_line(config, r"^#?PermitRootLogin", "PermitRootLogin no")
print(changed)
