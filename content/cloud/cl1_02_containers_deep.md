@@lesson cl1-containers-deep Containers in Depth: Images, Layers, Builds, Registries and Runtime Limits
@@text
Writing a Dockerfile that works is easy; writing one that builds fast, ships small and runs safely takes knowing how images are made. This lesson covers layers and the build cache, multi-stage builds, tags and digests, registries and scanning, and how containers are isolated and limited by the kernel.

## Images and layers

- An image is a stack of **read-only layers** plus metadata (entrypoint, env, user...). A running container adds a thin **writable layer** (lost when the container is removed: use volumes for data).
- Each `RUN`, `COPY` and `ADD` instruction creates a layer; metadata instructions (`ENV`, `CMD`, `EXPOSE`, `USER`, `WORKDIR`) don't add file content.
- Deleting a file in a later layer **doesn't shrink** the image: the bytes stay in the earlier layer. Clean up within the same `RUN`.
- Images follow the **OCI** image spec; layers are content-addressed by SHA-256, so identical layers are shared between images.

## The build cache

Docker reuses a cached layer if the instruction and its inputs are unchanged. **Once one layer is rebuilt, every later layer is rebuilt too.** For `COPY`/`ADD`, the inputs are the copied files' contents.

So order instructions from **least to most frequently changing**:

```dockerfile
FROM python:3.13-slim
WORKDIR /app
COPY requirements.txt .                 # changes rarely
RUN pip install --no-cache-dir -r requirements.txt
COPY . .                                # changes on every commit
USER 10001
CMD ["python", "-m", "app"]
```

Use a `.dockerignore` (exclude `.git`, `node_modules`, secrets, build output) so `COPY . .` doesn't invalidate the cache unnecessarily or leak files.

## Multi-stage builds

```dockerfile
FROM golang:1.24 AS build
WORKDIR /src
COPY . .
RUN CGO_ENABLED=0 go build -o /out/api ./cmd/api

FROM gcr.io/distroless/static-debian12
COPY --from=build /out/api /api
USER nonroot
ENTRYPOINT ["/api"]
```

Compilers and build tools stay in the first stage; the final image contains only the binary. Smaller images pull faster and have fewer vulnerabilities. Base image choices: full distro, `-slim`, Alpine (musl libc), distroless, `scratch`.

## Tags, digests and registries

- `registry/namespace/repo:tag@sha256:digest`. **Tags are mutable** (`latest` especially); **digests are immutable**. Pin production deployments by digest or immutable version tags; enable tag immutability in the registry.
- Registries: Docker Hub (pull rate limits), ECR, ACR, Artifact Registry, GHCR, Harbor. Use private registries with **vulnerability scanning**, **image signing** (Sigstore cosign, Notation) and admission policies that only allow signed images from trusted registries.
- `CMD` supplies default arguments; `ENTRYPOINT` sets the executable. Prefer the exec (JSON) form so signals reach the process.

## Isolation and limits

- **Namespaces** isolate what a process sees: pid, net, mnt, uts, ipc, user, cgroup.
- **cgroups** limit what it uses: `--cpus=1.5`, `--memory=512m` (OOM-killed when exceeded, exit code 137), `--pids-limit`.
- Hardening: run as **non-root** (`USER`), `--read-only` root filesystem, drop capabilities (`--cap-drop ALL`), no `--privileged`, seccomp/AppArmor profiles, rootless mode, never mount the Docker socket into containers.
- Containers share the host kernel: a kernel exploit escapes all of them; use VMs or sandboxed runtimes (gVisor, Kata, Firecracker) for hostile multi-tenancy.

## Operations

`docker build -t app:1.4.2 .`, `docker image ls`, `docker history app:1.4.2` (layers and sizes), `docker run -d --name app -p 8080:8000 app:1.4.2`, `docker logs -f app`, `docker exec -it app sh`, `docker stats`, `docker system prune`. Health: `HEALTHCHECK` in the Dockerfile, restart policies (`--restart unless-stopped`).

@@quiz
A Dockerfile runs `COPY . .` before `RUN pip install -r requirements.txt`. Why are builds slow?
- pip is slow
* Any source change invalidates the copy layer, so dependencies reinstall every build
- COPY can't be cached
- The base image is too big
? Copy requirements first, install, then copy the rest.

@@quiz
A layer downloads a 300 MB archive and a later RUN deletes it. How much does the image shrink?
* Not at all; the bytes remain in the earlier layer
- By 300 MB
- By half
- It depends on compression
? Download, use and delete in one RUN.

@@quiz
What does a multi-stage build mainly achieve?
- Faster container start-up
* A final image without build tools and source, so it's smaller with fewer vulnerabilities
- Running several processes in one container
- Automatic scanning
? COPY --from copies only the artefacts.

@@quiz
Which reference always resolves to exactly the same image content?
- app:latest
- app:1.4
* app@sha256:<digest>
- app:stable
? Tags can be moved.

@@quiz
Which kernel feature limits a container's memory?
- Namespaces
* cgroups
- seccomp
- SELinux
? Namespaces isolate what it can see.

@@quiz
A container exits with code 137 under load. What's the likely cause?
- A syntax error
* The memory limit was exceeded and the kernel killed it (OOM)
- The image wasn't found
- The port was in use
? 137 = 128 + SIGKILL(9).

@@flashcards
image :: read-only layers + metadata
container layer :: thin, writable, ephemeral
layer-creating instructions :: RUN, COPY, ADD
cache rule :: one change rebuilds all later layers
Dockerfile order :: least to most changing
.dockerignore :: exclude files from the context
deleting in a later layer :: doesn't shrink image
multi-stage build :: build tools stay behind
distroless :: no shell or package manager
tag :: mutable pointer
digest :: immutable content hash
image signing :: cosign, Notation
ENTRYPOINT :: the executable
CMD :: default arguments
exec form :: JSON array, signals work
namespaces :: isolate what you see
cgroups :: limit what you use
exit 137 :: OOM kill
non-root :: USER instruction
Docker socket mount :: root on the host

@@drill
Which Dockerfile instruction sets the user the container runs as?
= USER
= user
? Use a numeric UID for portability.

@@drill
Which kernel feature isolates a container's view of processes, network and mounts?
= namespaces
= namespace
= Linux namespaces
? cgroups limit resources.

@@exercise Simulate the build cache
Given a Dockerfile and the files changed since the last build, decide for each instruction whether it's a cache hit or rebuilt. An instruction is rebuilt if the instruction text changed, if it's a `COPY` whose sources include a changed file (`.` matches everything not in `.dockerignore`), or if any earlier layer was rebuilt. Print each step and the number of rebuilt steps for both Dockerfile orderings.
@@lang python
@@starter
import fnmatch

dockerignore = [".git/*", "*.md", "tests/*"]
changed = ["app/main.py", "README.md"]
dockerfiles = {
    "naive": ["FROM python:3.13-slim", "WORKDIR /app", "COPY . .", "RUN pip install -r requirements.txt", "CMD python -m app"],
    "cache-friendly": ["FROM python:3.13-slim", "WORKDIR /app", "COPY requirements.txt .", "RUN pip install -r requirements.txt", "COPY . .", "CMD python -m app"],
}
previous = {name: list(lines) for name, lines in dockerfiles.items()}

def copy_invalidated(instruction):
    raise NotImplementedError
@@expected
== naive
  CACHED  FROM python:3.13-slim
  CACHED  WORKDIR /app
  REBUILD COPY . .
  REBUILD RUN pip install -r requirements.txt
  REBUILD CMD python -m app
  3 of 5 steps rebuilt
== cache-friendly
  CACHED  FROM python:3.13-slim
  CACHED  WORKDIR /app
  CACHED  COPY requirements.txt .
  CACHED  RUN pip install -r requirements.txt
  REBUILD COPY . .
  REBUILD CMD python -m app
  2 of 6 steps rebuilt
@@requires
def copy_invalidated
fnmatch.fnmatch(
@@hint
For `COPY <src> <dst>`, the source is `instruction.split()[1]`. If it's `.`, the relevant changed files are those not matching any `.dockerignore` pattern; otherwise only that exact file. Once something is rebuilt, set a flag so later steps rebuild too.
@@solution
import fnmatch

dockerignore = [".git/*", "*.md", "tests/*"]
changed = ["app/main.py", "README.md"]
dockerfiles = {
    "naive": ["FROM python:3.13-slim", "WORKDIR /app", "COPY . .", "RUN pip install -r requirements.txt", "CMD python -m app"],
    "cache-friendly": ["FROM python:3.13-slim", "WORKDIR /app", "COPY requirements.txt .", "RUN pip install -r requirements.txt", "COPY . .", "CMD python -m app"],
}
previous = {name: list(lines) for name, lines in dockerfiles.items()}

def copy_invalidated(instruction):
    if not instruction.startswith("COPY "):
        return False
    source = instruction.split()[1]
    if source == ".":
        relevant = [f for f in changed if not any(fnmatch.fnmatch(f, p) for p in dockerignore)]
    else:
        relevant = [f for f in changed if f == source]
    return bool(relevant)

for name, lines in dockerfiles.items():
    print(f"== {name}")
    dirty = False
    rebuilt = 0
    for i, instruction in enumerate(lines):
        text_changed = i >= len(previous[name]) or previous[name][i] != instruction
        dirty = dirty or text_changed or copy_invalidated(instruction)
        rebuilt += dirty
        print(f"  {'REBUILD' if dirty else 'CACHED ':<7} {instruction}")
    print(f"  {rebuilt} of {len(lines)} steps rebuilt")

@@exercise Lint a container configuration
Check container run configurations against hardening rules and print findings by severity: `HIGH` for privileged mode, Docker socket mounts, running as root (user missing or `0`/`root`) and images without a tag or with `latest`; `MEDIUM` for no memory limit, writable root filesystem, and capabilities added; `LOW` for images not pinned by digest. Print a score of 100 minus 20 per HIGH, 10 per MEDIUM, 2 per LOW (minimum 0).
@@lang python
@@starter
containers = {
    "api": {"image": "registry.acme.io/api:2.3.1@sha256:9f2c", "user": "10001", "memory": "512m", "read_only": True, "cap_add": [], "privileged": False, "mounts": []},
    "worker": {"image": "registry.acme.io/worker:1.8", "user": "app", "memory": None, "read_only": False, "cap_add": [], "privileged": False, "mounts": ["/data:/data"]},
    "ci-runner": {"image": "docker:latest", "user": None, "memory": None, "read_only": False, "cap_add": ["SYS_ADMIN"], "privileged": True, "mounts": ["/var/run/docker.sock:/var/run/docker.sock"]},
    "proxy": {"image": "nginx", "user": "root", "memory": "128m", "read_only": True, "cap_add": ["NET_BIND_SERVICE"], "privileged": False, "mounts": []},
}

def findings(c):
    raise NotImplementedError
@@expected
api: score 100
worker: score 78
  [MEDIUM] no memory limit
  [MEDIUM] writable root filesystem
  [LOW] image not pinned by digest
ci-runner: score 0
  [HIGH] runs privileged
  [HIGH] mounts the Docker socket
  [HIGH] runs as root
  [HIGH] image tag is latest
  [MEDIUM] no memory limit
  [MEDIUM] writable root filesystem
  [MEDIUM] adds capabilities SYS_ADMIN
  [LOW] image not pinned by digest
proxy: score 48
  [HIGH] runs as root
  [HIGH] image tag is missing
  [MEDIUM] adds capabilities NET_BIND_SERVICE
  [LOW] image not pinned by digest
@@requires
def findings
docker.sock
"latest"
@@hint
An image's tag is what follows the last `:` after the last `/`; strip any `@sha256:...` first. Return a list of `(severity, message)` and compute the score from the counts.
@@solution
containers = {
    "api": {"image": "registry.acme.io/api:2.3.1@sha256:9f2c", "user": "10001", "memory": "512m", "read_only": True, "cap_add": [], "privileged": False, "mounts": []},
    "worker": {"image": "registry.acme.io/worker:1.8", "user": "app", "memory": None, "read_only": False, "cap_add": [], "privileged": False, "mounts": ["/data:/data"]},
    "ci-runner": {"image": "docker:latest", "user": None, "memory": None, "read_only": False, "cap_add": ["SYS_ADMIN"], "privileged": True, "mounts": ["/var/run/docker.sock:/var/run/docker.sock"]},
    "proxy": {"image": "nginx", "user": "root", "memory": "128m", "read_only": True, "cap_add": ["NET_BIND_SERVICE"], "privileged": False, "mounts": []},
}
WEIGHTS = {"HIGH": 20, "MEDIUM": 10, "LOW": 2}

def findings(c):
    out = []
    if c["privileged"]:
        out.append(("HIGH", "runs privileged"))
    if any("docker.sock" in m for m in c["mounts"]):
        out.append(("HIGH", "mounts the Docker socket"))
    if c["user"] in (None, "0", "root"):
        out.append(("HIGH", "runs as root"))
    name, _, digest = c["image"].partition("@")
    last = name.rsplit("/", 1)[-1]
    tag = last.split(":", 1)[1] if ":" in last else None
    if tag in (None, "latest"):
        out.append(("HIGH", f"image tag is {tag or 'missing'}"))
    if not c["memory"]:
        out.append(("MEDIUM", "no memory limit"))
    if not c["read_only"]:
        out.append(("MEDIUM", "writable root filesystem"))
    if c["cap_add"]:
        out.append(("MEDIUM", "adds capabilities " + ", ".join(c["cap_add"])))
    if not digest:
        out.append(("LOW", "image not pinned by digest"))
    return out

for name, c in containers.items():
    found = findings(c)
    score = max(0, 100 - sum(WEIGHTS[s] for s, _ in found))
    print(f"{name}: score {score}")
    for severity, message in found:
        print(f"  [{severity}] {message}")
