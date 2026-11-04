@@lesson lx4-automation Git, Ansible, Infrastructure as Code and CI/CD
@@lang bash
@@text
Modern Linux administration treats configuration as code: changes are written in files, reviewed in Git, and applied by tools like Ansible, Terraform and CI/CD pipelines. Linux+ covers Git basics, configuration management concepts, and the data formats (YAML, JSON) these tools use.

## Git essentials

```bash,norun
git init / git clone https://git.example.com/ops/infra.git
git status                       # what changed
git add roles/nginx/             # stage
git commit -m "nginx: enable HTTP/2"
git log --oneline --graph
git diff / git diff --staged
git branch feature/tls; git switch feature/tls      # (or git checkout -b)
git merge feature/tls            # bring a branch in
git rebase main                  # replay your commits on top of main
git pull / git push origin feature/tls
git tag v1.4.0
git revert <commit>              # undo with a new commit (safe on shared branches)
git reset --hard <commit>        # move the branch back (destroys work: careful)
git stash / git stash pop        # shelve uncommitted changes
```

- `.gitignore` lists files Git should never track (`*.log`, `.env`, secrets, build output).
- **Never commit secrets**; if you do, rotate them (history keeps them).
- **Merge conflicts**: Git marks `<<<<<<<`, `=======`, `>>>>>>>` sections; edit, `git add`, commit.
- Workflow: branch → commit → push → **pull/merge request** → review → merge → pipeline deploys.

## Configuration management

| Tool | Model | Language |
|---|---|---|
| **Ansible** | **agentless**, push over SSH | YAML playbooks |
| **Puppet** | agent, pull from a server | Puppet DSL |
| **Chef** | agent, pull | Ruby DSL |
| **SaltStack** | agent (or SSH), push/pull | YAML |
| **Terraform/OpenTofu** | provisions infrastructure (cloud, VMs, DNS) via APIs | HCL |

Key ideas: **idempotence** (running twice gives the same result: "ensure nginx is installed" rather than "install nginx"), **desired state**, **inventory**, **drift** detection, **immutable infrastructure** (replace servers instead of changing them).

## Ansible in brief

```yaml
# inventory.ini
[web]
web01 ansible_host=192.168.10.11
web02 ansible_host=192.168.10.12

# site.yml
- name: Configure web servers
  hosts: web
  become: true
  tasks:
    - name: Install nginx
      ansible.builtin.package:
        name: nginx
        state: present
    - name: Deploy config
      ansible.builtin.template:
        src: nginx.conf.j2
        dest: /etc/nginx/nginx.conf
        mode: "0644"
      notify: Reload nginx
  handlers:
    - name: Reload nginx
      ansible.builtin.service:
        name: nginx
        state: reloaded
```

Run: `ansible-playbook -i inventory.ini site.yml --check --diff` (dry run showing changes), then without `--check`. Ad hoc: `ansible web -m ping`. **Roles** organise tasks, templates (Jinja2), handlers and variables; **Ansible Vault** encrypts secrets.

## Data formats

- **YAML**: indentation-based (spaces, never tabs), `key: value`, lists with `- `. Indentation errors are the #1 Ansible/Kubernetes mistake.
- **JSON**: `{"key": "value", "list": [1, 2]}`; process with `jq` (`jq '.items[].name' file.json`).
- **Jinja2** templates: `{{ variable }}`, `{% for host in groups['web'] %}`.

## CI/CD

**Continuous integration** runs tests on every push (lint YAML, `shellcheck` scripts, `ansible-lint`, unit tests, build images). **Continuous delivery/deployment** ships passing changes to staging/production automatically or with an approval step. Tools: GitLab CI, GitHub Actions, Jenkins. Pipelines are defined as code (`.gitlab-ci.yml`, `.github/workflows/*.yml`). **GitOps**: the Git repository is the source of truth and an agent (Argo CD, Flux) makes the cluster match it.

## Other automation building blocks

`cloud-init` (first-boot configuration of cloud VMs: users, SSH keys, packages), **systemd** units and timers, `cron`, shell scripts, REST APIs with `curl`, and **Python** for anything complex.

@@quiz
What does it mean that an Ansible task is idempotent?
* Running it again doesn't change anything once the system is in the desired state
- It runs on only one host at a time
- It can't fail
- It runs without SSH
? "Ensure present", not "install again".

@@quiz
Which Git command safely undoes a commit that was already pushed to a shared branch?
- `git reset --hard`
* `git revert <commit>`
- `git stash`
- `git rebase -i`
? revert adds a new commit; reset rewrites history.

@@quiz
How does Ansible connect to managed Linux hosts by default?
- An installed agent polling a server
* Over SSH, agentless
- Through SNMP
- Using RDP
? Puppet and Chef use agents.

@@quiz
Which Ansible option shows what would change without changing anything?
- `--verbose`
* `--check --diff`
- `--limit`
- `--tags`
? A dry run.

@@quiz
A secret API key was committed to Git and pushed. What's the most important step?
- Delete the file in a new commit
* Rotate (revoke and replace) the key, then clean history
- Add it to .gitignore
- Make the repository private
? History and clones still contain it.

@@flashcards
git add / commit / push :: stage / record / upload
git switch -c :: create and switch branch
git revert :: undo via a new commit
git reset --hard :: discard commits and changes
git stash :: shelve uncommitted work
.gitignore :: files Git won't track
merge request :: review before merging
idempotence :: same result when run again
desired state :: declare what, not how
configuration drift :: system differs from code
Ansible :: agentless, SSH, YAML playbooks
Puppet / Chef :: agent-based config management
Terraform :: provision infrastructure via APIs
handler :: runs when notified by a change
Ansible Vault :: encrypt secrets
YAML :: spaces, never tabs
jq :: query JSON on the command line
cloud-init :: first-boot VM configuration
GitOps :: Git is the source of truth

@@drill
Which Git command shows the commit history in one line per commit?
= git log --oneline
? Add --graph for branches.

@@drill
Which command-line tool queries and filters JSON?
= jq
? `jq '.key'`.

@@exercise Check YAML indentation
Simple YAML linting in bash/awk: for each line of `playbook`, report tabs (`tab character`), indentation that isn't a multiple of 2 spaces, and trailing spaces. Ignore blank lines. Print `line N: problem` for each finding, then `ok` or `N problem(s)`.
@@starter
playbook='- name: Configure web
  hosts: web
  become: true
  tasks:
    - name: Install nginx
	  ansible.builtin.package:
        name: nginx
       state: present
    - name: Start nginx  
      ansible.builtin.service:
        name: nginx
        state: started'

echo "$playbook"
@@expected
line 6: tab character
line 8: indent of 7 spaces
line 9: trailing spaces
3 problem(s)
@@requires
/\t/
match($0, /^ */)
@@hint
`match($0, /^ */)` sets `RLENGTH` to the number of leading spaces. Trailing spaces: `/ +$/`.
@@solution
playbook='- name: Configure web
  hosts: web
  become: true
  tasks:
    - name: Install nginx
	  ansible.builtin.package:
        name: nginx
       state: present
    - name: Start nginx  
      ansible.builtin.service:
        name: nginx
        state: started'

echo "$playbook" | awk '
NF == 0 { next }
/\t/ { print "line " NR ": tab character"; n++ }
{ match($0, /^ */); if (RLENGTH % 2) { print "line " NR ": indent of " RLENGTH " spaces"; n++ } }
/ +$/ { print "line " NR ": trailing spaces"; n++ }
END { print (n ? n " problem(s)" : "ok") }'

@@exercise Build an Ansible inventory from a host list
The `hosts` variable lists `hostname ip role` lines. Generate an INI-style Ansible inventory grouped by role, with groups in alphabetical order and hosts in input order, each as `hostname ansible_host=ip`, separated by blank lines. Skip lines whose IP isn't a valid IPv4 address (four numbers 0–255) and list them at the end as `# skipped: hostname`.
@@starter
hosts='web01 192.168.10.11 web
db01 192.168.10.21 db
web02 192.168.10.12 web
cache01 192.168.10.31 cache
web03 192.168.10.300 web
db02 192.168.10.22 db'

echo "$hosts"
@@expected
[cache]
cache01 ansible_host=192.168.10.31

[db]
db01 ansible_host=192.168.10.21
db02 ansible_host=192.168.10.22

[web]
web01 ansible_host=192.168.10.11
web02 ansible_host=192.168.10.12
# skipped: web03
@@requires
sort -u
valid_ip
@@hint
Write `valid_ip()` with `IFS=. read -r a b c d` and checks. Collect roles with `cut -d' ' -f3 | sort -u`, then loop over roles and over hosts again.
@@solution
hosts='web01 192.168.10.11 web
db01 192.168.10.21 db
web02 192.168.10.12 web
cache01 192.168.10.31 cache
web03 192.168.10.300 web
db02 192.168.10.22 db'

valid_ip() {
    local a b c d extra part
    IFS=. read -r a b c d extra <<< "$1"
    [[ -z $extra && -n $d ]] || return 1
    for part in "$a" "$b" "$c" "$d"; do
        [[ $part =~ ^[0-9]{1,3}$ ]] && (( part <= 255 )) || return 1
    done
}

skipped=()
first=1
for role in $(cut -d' ' -f3 <<< "$hosts" | sort -u); do
    (( first )) || echo
    first=0
    echo "[$role]"
    while read -r name ip r; do
        [[ $r == "$role" ]] || continue
        if valid_ip "$ip"; then
            echo "$name ansible_host=$ip"
        else
            skipped+=("$name")
        fi
    done <<< "$hosts"
done
for name in "${skipped[@]}"; do echo "# skipped: $name"; done
