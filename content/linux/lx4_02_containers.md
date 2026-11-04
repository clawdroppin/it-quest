@@lesson lx4-containers Containers with Podman and Docker
@@lang bash
@@text
Containers package an application with its dependencies so it runs the same on a laptop, a server and in Kubernetes. Linux+ covers running and managing containers, images and registries, volumes and networking, writing a Containerfile/Dockerfile, and the security basics. Podman and Docker use almost identical commands.

## How containers work

A container is an ordinary Linux process isolated with **namespaces** (its own view of PIDs, network, mounts, hostname, users) and limited with **cgroups** (CPU, memory, I/O). It shares the **host kernel**, which is why containers start in milliseconds and why kernel vulnerabilities matter. An **image** is a read-only stack of layers; a **container** is a running instance with a thin writable layer on top.

**Podman** is daemonless and runs **rootless** by default (containers run as your user); **Docker** uses the `dockerd` daemon (rootless mode exists). Both follow the **OCI** standards.

## Everyday commands

```bash,norun
podman pull docker.io/library/nginx:1.27          # get an image from a registry
podman images                                     # local images
podman run -d --name web -p 8080:80 nginx:1.27    # detached, host 8080 -> container 80
podman ps / podman ps -a                          # running / all containers
podman logs -f web                                # follow output
podman exec -it web /bin/sh                       # shell inside
podman stop web; podman rm web                    # stop, remove
podman inspect web                                # JSON details (IP, mounts, env)
podman stats                                      # live CPU/memory
podman rmi nginx:1.27; podman image prune         # remove images
podman system df                                  # disk used by images/containers/volumes
```

Use **tags** or **digests** (`nginx@sha256:...`) rather than `latest` so deployments are reproducible.

## Persistent data

Container file systems are **ephemeral**: removing the container deletes its writable layer.

- **Volumes** (managed by the engine): `podman volume create pgdata`, `-v pgdata:/var/lib/postgresql/data`.
- **Bind mounts** (a host path): `-v /srv/site:/usr/share/nginx/html:ro,Z` (`:ro` read-only; `:Z` relabels for SELinux).
- **Environment variables**: `-e TZ=Asia/Tokyo`, `--env-file app.env` (keep secrets out of images; use secrets features).

## Networking

`-p host:container` publishes ports. Containers on the same user-defined network reach each other by name: `podman network create app`, `--network app`. Network modes: **bridge** (default, NAT), **host** (shares host networking), **none**.

## Building images

```text
# Containerfile / Dockerfile
FROM python:3.13-slim
WORKDIR /app
COPY requirements.txt .
RUN pip install --no-cache-dir -r requirements.txt
COPY . .
USER 1001
EXPOSE 8000
CMD ["python", "-m", "app"]
```

`podman build -t registry.example.com/team/app:1.4.0 .`, `podman push ...`. Each instruction creates a layer; order instructions so rarely-changing ones (dependencies) come first for caching. **Multi-stage builds** compile in one stage and copy only the result into a small runtime image. `ENTRYPOINT` sets the executable, `CMD` default arguments.

## Compose and pods

`podman-compose` / `docker compose` define multi-container apps in YAML (services, ports, volumes, networks). Podman **pods** group containers sharing a network namespace (like Kubernetes pods); `podman generate kube` exports YAML. **Quadlet** files (`.container` units) make systemd manage containers as services.

## Container security

- Use **minimal, trusted base images**; scan them (Trivy, Grype); rebuild to pick up patches.
- **Don't run as root** inside the container (`USER`); prefer rootless engines.
- **Read-only** root file system (`--read-only`), drop capabilities (`--cap-drop=ALL`, add back only what's needed), no `--privileged`.
- Limit resources: `--memory 512m --cpus 1.5`.
- Never bake secrets into images (they're visible in layers and history).
- Sign images and pull from trusted registries.
- Mounting the Docker socket into a container gives it control of the host.

## Orchestration (awareness)

Kubernetes schedules containers across many nodes: pods, deployments, services, ingress, ConfigMaps and Secrets, persistent volumes. Linux+ only expects the concepts.

@@quiz
What isolates a container's view of processes, network and mounts?
* Kernel namespaces
- cgroups
- The hypervisor
- SELinux
? cgroups limit resource usage.

@@quiz
A database container was removed and recreated, and all its data is gone. What should have been used?
- A bigger image
* A named volume or bind mount for the data directory
- The `latest` tag
- Host networking
? Container writable layers are ephemeral.

@@quiz
What does `-p 8080:80` mean in `podman run -p 8080:80 nginx`?
* Host port 8080 forwards to container port 80
- Container port 8080 forwards to host port 80
- Both ports are opened on the host
- Limits the container to 80% CPU
? host:container.

@@quiz
Which Containerfile instruction makes the app run as a non-root user?
- RUN
- EXPOSE
* USER
- WORKDIR
? Combine with rootless engines.

@@quiz
Why avoid the `latest` tag in production deployments?
- It's slower to download
* It isn't reproducible: the image it points to can change between deployments
- Registries delete it daily
- Podman doesn't support it
? Pin a version tag or digest.

@@flashcards
namespaces :: isolation of PIDs, network, mounts
cgroups :: resource limits
image vs container :: read-only template vs running instance
rootless :: containers run as an unprivileged user
podman run -d -p 8080:80 :: detached with a published port
podman exec -it :: shell in a running container
podman logs -f :: follow container output
volume :: engine-managed persistent storage
bind mount :: host directory in the container
:Z :: SELinux relabel for a bind mount
FROM / RUN / COPY / CMD :: base, build step, add files, default command
multi-stage build :: small final image
USER :: run as non-root
--read-only :: immutable container root file system
--cap-drop=ALL :: remove Linux capabilities
digest :: immutable image reference
pod :: containers sharing a network namespace
Quadlet :: systemd-managed containers

@@drill
Which kernel feature limits a container's CPU and memory?
= cgroups
= cgroup
? Namespaces isolate; cgroups limit.

@@drill
Which Containerfile instruction sets the base image?
= FROM
= from
? It must come first (after ARG).

@@exercise Lint a Containerfile
Check the `containerfile` text for common problems, printing each finding with its line number: a base image with `:latest` or no tag; `apt-get install` without `-y` (it will wait for input and hang the build); `ADD` used for local files (prefer `COPY`); secrets in `ENV` (names containing `PASSWORD`, `SECRET` or `TOKEN`); and no `USER` instruction anywhere (report at the end). Print `findings: N`.
@@starter
containerfile='FROM ubuntu:latest
ENV DB_PASSWORD=hunter2
RUN apt-get update && apt-get install curl
ADD ./app /app
WORKDIR /app
EXPOSE 8080
CMD ["./server"]'

echo "$containerfile"
@@expected
1: pin the base image version (ubuntu:latest)
2: secret in ENV is baked into the image
3: apt-get install without -y will hang
4: use COPY for local files
no USER: container runs as root
findings: 5
@@requires
awk
END
@@hint
One awk program: `NR` is the line number. `$1 == "FROM" && ($2 ~ /:latest$/ || $2 !~ /:/)`. Track `user = 1` when `$1 == "USER"` and report in `END`.
@@solution
containerfile='FROM ubuntu:latest
ENV DB_PASSWORD=hunter2
RUN apt-get update && apt-get install curl
ADD ./app /app
WORKDIR /app
EXPOSE 8080
CMD ["./server"]'

echo "$containerfile" | awk '
$1 == "FROM" && ($2 ~ /:latest$/ || $2 !~ /:/) { print NR ": pin the base image version (" $2 ")"; n++ }
$1 == "RUN" && /apt-get install/ && !/ -y/ { print NR ": apt-get install without -y will hang"; n++ }
$1 == "ADD" && $2 !~ /^https?:/ { print NR ": use COPY for local files"; n++ }
$1 == "ENV" && toupper($2) ~ /(PASSWORD|SECRET|TOKEN)/ { print NR ": secret in ENV is baked into the image"; n++ }
$1 == "USER" { user = 1 }
END {
    if (!user) { print "no USER: container runs as root"; n++ }
    print "findings: " n
}'

@@exercise Summarise container resource usage
`podman stats --no-stream` output (simplified) is in `stats`. Print containers using more than 80% of their memory limit as `MEMORY name used/limit (pct%)`, then the total CPU percentage across all containers with one decimal. Memory values are in MiB (`MiB`) or GiB (`GiB`): convert to MiB first.
@@starter
stats='NAME   CPU%   MEM_USAGE  MEM_LIMIT
web    3.5%   210MiB     512MiB
api    41.2%  1.8GiB     2GiB
db     12.0%  3.1GiB     4GiB
cache  0.8%   460MiB     512MiB
worker 22.4%  300MiB     1GiB'

echo "$stats"
@@expected
MEMORY api 1.8GiB/2GiB (90%)
MEMORY cache 460MiB/512MiB (90%)
total CPU: 79.9%
@@requires
function mib(
GiB
@@hint
In awk, define `function mib(v) { if (v ~ /GiB$/) return v * 1024; return v + 0 }` (awk converts the leading number). CPU: `$2 + 0`.
@@solution
stats='NAME   CPU%   MEM_USAGE  MEM_LIMIT
web    3.5%   210MiB     512MiB
api    41.2%  1.8GiB     2GiB
db     12.0%  3.1GiB     4GiB
cache  0.8%   460MiB     512MiB
worker 22.4%  300MiB     1GiB'

echo "$stats" | awk '
function mib(v) { if (v ~ /GiB$/) return v * 1024; return v + 0 }
NR > 1 {
    cpu += $2 + 0
    used = mib($3); limit = mib($4)
    pct = used / limit * 100
    if (pct > 80) printf "MEMORY %s %s/%s (%.0f%%)\n", $1, $3, $4, pct
}
END { printf "total CPU: %.1f%%\n", cpu }'
