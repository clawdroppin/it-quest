@@lesson d5-docker Docker in Depth: Images, Layers, Compose and Container Security
@@concepts processes, files, strings
@@lang python
@@text
The Cloud track introduced containers. This lesson is the working knowledge a DevOps engineer uses every day: writing small, fast, safe images, running multi-container apps with Compose, and debugging containers when they misbehave. (Install Docker Desktop on Windows/macOS, or Docker Engine on Linux, to try the commands. The exercises are plain Python.)

## Images are layers

An image is a stack of read-only **layers**, one per filesystem-changing instruction (`RUN`, `COPY`, `ADD`). A container adds one thin writable layer on top. Layers are **cached**: when you rebuild, Docker reuses every layer until the first instruction (or copied file) that changed, and rebuilds everything after it.

That gives the most important Dockerfile rule: **put what changes rarely first, and what changes often last.**

```dockerfile
# Slow: any code change invalidates the pip install layer
FROM python:3.12-slim
WORKDIR /app
COPY . .
RUN pip install -r requirements.txt
CMD ["python", "app.py"]
```

```dockerfile
# Fast: dependencies are only reinstalled when requirements.txt changes
FROM python:3.12-slim
WORKDIR /app
COPY requirements.txt .
RUN pip install --no-cache-dir -r requirements.txt
COPY . .
CMD ["python", "app.py"]
```

## The instructions

| Instruction | What it does |
|---|---|
| `FROM image:tag` | the base image; `AS name` names a build stage |
| `WORKDIR /app` | set (and create) the working directory |
| `COPY src dst` | copy files from the build context (prefer it over `ADD`) |
| `ADD` | like COPY, plus URLs and auto-extracting tar files: surprising, so avoid |
| `RUN cmd` | run a command at **build** time, creating a layer |
| `ENV KEY=value` | an environment variable for build and run |
| `ARG NAME=default` | a build-time variable (`--build-arg`); not available at run time |
| `EXPOSE 8080` | documents the port; doesn't publish it |
| `USER app` | run as this user from here on (don't run as root) |
| `ENTRYPOINT ["prog"]` | the program the container runs |
| `CMD ["--flag"]` | default arguments (or the default command if there's no ENTRYPOINT); easy to override |
| `HEALTHCHECK CMD curl -f http://localhost:8080/healthz` | how Docker knows the app is healthy |

Use the **exec form** (`["python", "app.py"]`), not the shell form (`python app.py`): then your program is PID 1 and receives `SIGTERM` on `docker stop`, so it can shut down gracefully.

A `.dockerignore` file keeps junk and secrets out of the build context: `.git`, `node_modules`, `__pycache__`, `.env`, `*.log`, `.terraform`.

## Multi-stage builds: small final images

Build with the full toolchain, ship only the result:

```dockerfile
FROM node:22 AS build
WORKDIR /src
COPY package*.json ./
RUN npm ci
COPY . .
RUN npm run build                       # produces /src/dist

FROM nginx:1.27-alpine                  # final image: just a web server and the files
COPY --from=build /src/dist /usr/share/nginx/html
```

Base image choices: `-slim` (Debian, small, compatible), `-alpine` (tiny, but uses musl libc, which occasionally breaks Python wheels), **distroless** (no shell or package manager at all: smallest attack surface), `scratch` (empty: for static Go or Rust binaries).

## Tags, digests and registries

`ghcr.io/acme/web:1.4.2` is *registry* / *repository* : *tag*. Without a registry it's Docker Hub (`docker.io`), and official images live under `library/`, so `nginx` means `docker.io/library/nginx:latest`.

- **Tags move.** `:latest` and even `:1.4` can point to a different image tomorrow. For reproducible deploys use a specific version, or the **digest**: `nginx@sha256:4f0f...`, which can never change.
- Tag images with the Git commit (`web:3f9c2a1`) and the release (`web:1.4.2`), push both.
- Registries: Docker Hub, GitHub Container Registry (`ghcr.io`), Amazon ECR, Azure ACR, Google Artifact Registry, Harbor (self-hosted).

```text
docker build -t ghcr.io/acme/web:1.4.2 .
docker push ghcr.io/acme/web:1.4.2
docker buildx build --platform linux/amd64,linux/arm64 -t ghcr.io/acme/web:1.4.2 --push .   # multi-arch
```

## Running containers

```text
docker run -d --name web -p 8080:80 nginx:1.27            # detached, host port 8080 -> container 80
docker run --rm -it python:3.12-slim bash                 # throwaway interactive shell
docker run -d --restart unless-stopped \
  -e DB_HOST=db -v appdata:/var/lib/app \
  --memory 512m --cpus 1 --read-only --tmpfs /tmp \
  --cap-drop ALL --security-opt no-new-privileges \
  ghcr.io/acme/api:2.0.1
docker ps / docker ps -a                                  # running / all containers
docker logs -f --tail 100 web                             # follow the logs
docker exec -it web sh                                    # a shell inside a running container
docker inspect web                                        # everything, as JSON
docker stats                                              # live CPU and memory
docker system df / docker system prune                    # disk use / clean up
```

## Storage and networking

- **Volumes** (`-v appdata:/data`): managed by Docker, survive the container. Use them for databases.
- **Bind mounts** (`-v "$PWD":/app`): a host folder inside the container. Great for development.
- **tmpfs**: in memory, gone when the container stops.
- Containers on the same **user-defined network** reach each other **by name** (Docker's built-in DNS): `docker network create appnet`, then `--network appnet`, and the API connects to `db:5432`. The default `bridge` network has no name resolution.

## Docker Compose: a whole stack in one file

```yaml
# compose.yaml
services:
  api:
    build: .
    ports: ["8080:8080"]
    environment:
      DATABASE_URL: postgres://app:${DB_PASSWORD}@db:5432/app   # ${...} comes from .env
    depends_on:
      db:
        condition: service_healthy      # wait until the database really accepts connections
    restart: unless-stopped
  db:
    image: postgres:17
    environment:
      POSTGRES_USER: app
      POSTGRES_PASSWORD: ${DB_PASSWORD}
    volumes: ["dbdata:/var/lib/postgresql/data"]
    healthcheck:
      test: ["CMD-SHELL", "pg_isready -U app"]
      interval: 5s
      retries: 10
volumes:
  dbdata:
```

```text
docker compose up -d --build      # build and start everything
docker compose ps / logs -f api   # status / follow one service
docker compose exec db psql -U app
docker compose down               # stop and remove (add -v to delete volumes too)
```

Compose is perfect for development, CI test environments and small single-server deployments. For many servers and self-healing, you move to **Kubernetes**.

## Container security checklist

1. **Small, trusted base images**, pinned to a version, rebuilt regularly to pick up patches.
2. **Run as a non-root user** (`USER`), with a read-only root filesystem where possible.
3. **Drop capabilities** (`--cap-drop ALL`), never `--privileged`, never mount `/var/run/docker.sock` into an app container (that's root on the host).
4. **No secrets in images**: not in `ENV`, not in files, not in build args (they're visible in `docker history`). Pass them at run time, or use build secrets (`RUN --mount=type=secret,id=npmrc ...`).
5. **Scan images** for known vulnerabilities in CI: `trivy image`, `docker scout cves`, Grype; and generate an SBOM (`syft`).
6. **Sign images** (cosign) and only deploy signed ones.

## Debugging checklist

- Exits immediately? `docker logs <name>` and `docker ps -a` for the exit code (137 = killed, often out of memory; 1 = app error).
- Can't connect? Is the port published (`-p`)? Is the app listening on `0.0.0.0` inside the container, not `127.0.0.1`?
- Works on my machine? Check the architecture (`arm64` Mac vs `amd64` server) and environment variables.
- Image huge? `docker history <image>` shows each layer's size.

@@quiz
Why copy `requirements.txt` and install dependencies before copying the rest of the code?
- Docker requires that order
* So the slow install layer stays cached and is only rebuilt when the requirements change
- It makes the image run faster
- So the code isn't included in the image
? Layers rebuild from the first change onwards; code changes often, dependencies rarely.

@@quiz
What's the benefit of a multi-stage build?
- It runs several containers at once
* The final image contains only what's needed to run, not compilers and build tools: smaller and safer
- It builds for several CPU architectures
- It caches layers between machines
? `COPY --from=build` copies just the results from the build stage.

@@quiz
Your API container can't reach the database at `db:5432`. Both run with plain `docker run` on the default network. Why?
- Port 5432 is blocked by Docker
* The default bridge network has no DNS for container names; use a user-defined network or Compose
- Databases can't run in containers
- You need `--privileged`
? On a user-defined network (and in Compose), containers find each other by name.

@@quiz
Which is the safest way to give a container a database password?
- `ENV DB_PASSWORD=...` in the Dockerfile
- A build argument
* At run time: an environment variable from a secret store, or a mounted secret file
- Commit a `.env` file into the image
? Anything baked into an image can be read by anyone who can pull it.

@@quiz
A container exits with code 137. What's the most likely cause?
- A syntax error in the app
* It was killed (SIGKILL), often because it ran out of its memory limit
- The port was already in use
- The image wasn't found
? 128 + 9 (SIGKILL) = 137. Check `docker inspect` for `OOMKilled`.

@@drill
What's the name of a Dockerfile with a build stage and a smaller final stage?
= multi-stage build
= multi-stage
= multistage build
? The final image leaves the compilers behind.

@@flashcards
layer :: one filesystem change from RUN, COPY or ADD; cached and shared
build cache rule :: rarely changing steps first, often changing steps last
`.dockerignore` :: keep junk and secrets out of the build context
exec form `["prog", "arg"]` :: the app is PID 1 and receives SIGTERM
ENTRYPOINT vs CMD :: the program / its default (overridable) arguments
multi-stage build :: build in one stage, `COPY --from` the results into a small final stage
distroless :: images with no shell or package manager
tag vs digest :: movable name / immutable `@sha256:` fingerprint
`docker.io/library/` :: where unqualified official images come from
volume vs bind mount :: Docker-managed storage / a host folder
user-defined network :: containers resolve each other by name
`depends_on: condition: service_healthy` :: wait for a healthcheck in Compose
`docker exec -it c sh` :: a shell in a running container
`docker logs -f` :: follow a container's output
exit code 137 :: killed: usually out of memory
`USER app` :: don't run as root
`--cap-drop ALL` / `--read-only` :: remove privileges / read-only root filesystem
docker.sock :: mounting it gives root on the host
`RUN --mount=type=secret` :: use a secret at build time without storing it
trivy / docker scout / grype :: image vulnerability scanners

@@exercise Which layers rebuild?
Docker reuses cached layers until the first instruction that changed, then rebuilds that one **and every one after it**. Write `rebuilt(old, new, changed_files)`. `old` and `new` are lists of instruction lines. A step must be rebuilt if its text differs from the old step at the same position (or there was no old step there), or if it's a `COPY` whose source (the second word) is in `changed_files` or is `.` while anything changed. Return the **1-based numbers** of the steps that are rebuilt.
@@starter
def rebuilt(old, new, changed_files):
    pass


steps = ["FROM python:3.12-slim", "WORKDIR /app", "COPY requirements.txt .", "RUN pip install -r requirements.txt", "COPY . .", 'CMD ["python", "app.py"]']
print(rebuilt(steps, steps, {"app.py"}))
@@tests
steps = ["FROM python:3.12-slim", "WORKDIR /app", "COPY requirements.txt .", "RUN pip install -r requirements.txt", "COPY . .", 'CMD ["python", "app.py"]']
check(rebuilt(steps, steps, set()), [], "nothing changed")
check(rebuilt(steps, steps, {"app.py"}), [5, 6], "a code change: only the last copy and after")
check(rebuilt(steps, steps, {"requirements.txt"}), [3, 4, 5, 6], "new requirements: reinstall")
newer = steps[:1] + ["RUN apt-get update"] + steps[1:]
check(rebuilt(steps, newer, set()), [2, 3, 4, 5, 6, 7], "an inserted step rebuilds everything after it")
check(rebuilt(steps, ["FROM python:3.13-slim"] + steps[1:], set()), [1, 2, 3, 4, 5, 6], "a new base image rebuilds everything")
@@hint
Loop with `enumerate(new)`. The step changed if `i >= len(old) or old[i] != step`, or if it's a COPY (`step.split()[0] == "COPY"`) whose source `step.split()[1]` is in `changed_files`, or is `"."` and `changed_files` isn't empty. From the first changed step on, return every number up to `len(new)`.
@@solution
def rebuilt(old, new, changed_files):
    for i, step in enumerate(new):
        words = step.split()
        text_changed = i >= len(old) or old[i] != step
        copy_changed = words[0] == "COPY" and (words[1] in changed_files or (words[1] == "." and bool(changed_files)))
        if text_changed or copy_changed:
            return list(range(i + 1, len(new) + 1))
    return []


steps = ["FROM python:3.12-slim", "WORKDIR /app", "COPY requirements.txt .", "RUN pip install -r requirements.txt", "COPY . .", 'CMD ["python", "app.py"]']
print(rebuilt(steps, steps, {"app.py"}))

@@exercise Parse an image reference
Write `parse_image(ref)` returning a dict with `registry`, `repository`, `tag` and `digest`, filling in Docker's defaults:

- The first part is a **registry** only if it contains a `.` or a `:`, or is `localhost`; otherwise the registry is `docker.io`.
- On `docker.io`, a repository without a `/` gets `library/` in front.
- `@sha256:...` is the digest (else `None`); `:tag` after the last `/` is the tag. With no tag and no digest, the tag is `latest`; with a digest but no tag, the tag is `None`.
@@starter
def parse_image(ref):
    pass


print(parse_image("nginx"))
print(parse_image("ghcr.io/acme/web:1.4.2"))
@@tests
check(parse_image("nginx"), {"registry": "docker.io", "repository": "library/nginx", "tag": "latest", "digest": None}, "an official image")
check(parse_image("grafana/grafana:11.3.0"), {"registry": "docker.io", "repository": "grafana/grafana", "tag": "11.3.0", "digest": None}, "a Docker Hub user image")
check(parse_image("ghcr.io/acme/web:1.4.2"), {"registry": "ghcr.io", "repository": "acme/web", "tag": "1.4.2", "digest": None}, "GitHub's registry")
check(parse_image("localhost:5000/tools/backup"), {"registry": "localhost:5000", "repository": "tools/backup", "tag": "latest", "digest": None}, "a registry with a port")
check(parse_image("redis@sha256:abc123"), {"registry": "docker.io", "repository": "library/redis", "tag": None, "digest": "sha256:abc123"}, "a digest")
check(parse_image("123456789012.dkr.ecr.ap-northeast-1.amazonaws.com/api:2.0@sha256:ff"), {"registry": "123456789012.dkr.ecr.ap-northeast-1.amazonaws.com", "repository": "api", "tag": "2.0", "digest": "sha256:ff"}, "ECR with a tag and a digest")
@@hint
Split off the digest first with `ref.partition("@")`. Then split the rest at the first `/`: is that first part a registry? Then look for `:` only in the part after the **last** `/` (a registry port also has a colon).
@@solution
def parse_image(ref):
    name, at, digest = ref.partition("@")
    first, slash, rest = name.partition("/")
    if slash and ("." in first or ":" in first or first == "localhost"):
        registry, path = first, rest
    else:
        registry, path = "docker.io", name
    head, _, last = path.rpartition("/")
    tag = None
    if ":" in last:
        last, tag = last.split(":", 1)
    repository = f"{head}/{last}" if head else last
    if registry == "docker.io" and "/" not in repository:
        repository = "library/" + repository
    if tag is None and not at:
        tag = "latest"
    return {"registry": registry, "repository": repository, "tag": tag, "digest": digest if at else None}


print(parse_image("nginx"))
print(parse_image("ghcr.io/acme/web:1.4.2"))

@@exercise Lint a Dockerfile
Write `lint(dockerfile)` that returns a list of warnings, in this order:

1. For each `FROM` line whose image has no tag or uses `:latest` (ignore `scratch`, and stage names after `AS`): `"line N: pin the base image version"`.
2. For each `ADD` line: `"line N: use COPY instead of ADD"`.
3. If there is no `USER` instruction at all: `"no USER: the container runs as root"`.
4. If a line contains `apt-get install` without `--no-install-recommends`: `"line N: add --no-install-recommends"`.

Line numbers count from 1 and include blank lines. Instructions are case-insensitive. Report rules 1, 2 and 4 in line order, with rule 3 after all of them.
@@starter
DOCKERFILE = """FROM ubuntu
ADD app.tar.gz /opt/app
RUN apt-get update && apt-get install -y curl

FROM python:3.12-slim AS final
COPY --from=0 /opt/app /app
CMD ["python", "/app/main.py"]
"""


def lint(dockerfile):
    pass


for warning in lint(DOCKERFILE):
    print(warning)
@@tests
check(lint(DOCKERFILE), ["line 1: pin the base image version", "line 2: use COPY instead of ADD", "line 3: add --no-install-recommends", "no USER: the container runs as root"], "the sample")
good = "FROM python:3.12-slim\nRUN apt-get install -y --no-install-recommends curl\nUSER app\n"
check(lint(good), [], "a clean Dockerfile")
check(lint("from node:latest\nuser node\n"), ["line 1: pin the base image version"], "lower-case and :latest")
check(lint("FROM scratch\nCOPY app /app\nUSER 1000\n"), [], "scratch is fine")
check(lint("FROM localhost:5000/base\nUSER x\n"), ["line 1: pin the base image version"], "a registry port is not a tag")
@@hint
For each line: `words = line.split()`; the instruction is `words[0].upper()`. For FROM, the image is `words[1]`; it has a tag if `":" in image.rsplit("/", 1)[-1]`. Collect the per-line warnings, then add the USER warning at the end if no line's instruction was `USER`.
@@solution
DOCKERFILE = """FROM ubuntu
ADD app.tar.gz /opt/app
RUN apt-get update && apt-get install -y curl

FROM python:3.12-slim AS final
COPY --from=0 /opt/app /app
CMD ["python", "/app/main.py"]
"""


def lint(dockerfile):
    warnings = []
    has_user = False
    for n, line in enumerate(dockerfile.splitlines(), start=1):
        words = line.split()
        if not words:
            continue
        instruction = words[0].upper()
        if instruction == "FROM":
            image = words[1]
            last = image.rsplit("/", 1)[-1]
            if image != "scratch" and (":" not in last or last.endswith(":latest")):
                warnings.append(f"line {n}: pin the base image version")
        elif instruction == "ADD":
            warnings.append(f"line {n}: use COPY instead of ADD")
        elif instruction == "USER":
            has_user = True
        if "apt-get install" in line and "--no-install-recommends" not in line:
            warnings.append(f"line {n}: add --no-install-recommends")
    if not has_user:
        warnings.append("no USER: the container runs as root")
    return warnings


for warning in lint(DOCKERFILE):
    print(warning)
