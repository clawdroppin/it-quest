@@lesson l5-containers Containers: Docker in Practice
@@text
Containers package an application **with everything it needs** (libraries, runtime, config) into an image that runs the same on a laptop, a server or the cloud. Docker is in a huge share of IT job postings, and Kubernetes (which runs containers at scale) is the backbone of modern cloud platforms.

## Containers vs virtual machines

| | Virtual machine | Container |
|---|---|---|
| Contains | a whole OS with its own kernel | just the app and its files; shares the host's kernel |
| Starts in | minutes | seconds or less |
| Size | gigabytes | megabytes |
| Isolation | strong (hypervisor) | good (namespaces, cgroups) |

## Daily commands

```bash,norun
docker run -d --name web -p 8080:80 nginx       # run nginx in the background; host port 8080 -> container 80
docker ps                                       # running containers (-a: all)
docker logs -f web                              # its output
docker exec -it web sh                          # a shell inside it
docker stop web && docker rm web
docker images                                   # local images
docker pull python:3.12-slim
docker run --rm -it -v "$PWD":/app -w /app python:3.12-slim python script.py   # run a script in a throwaway container
```

`-v host:container` mounts a folder (**volume**) so data survives the container being deleted. Containers are **disposable**; data belongs in volumes.

## Building your own image

```dockerfile
# Dockerfile
FROM python:3.12-slim
WORKDIR /app
COPY requirements.txt .
RUN pip install --no-cache-dir -r requirements.txt
COPY . .
USER 1000
EXPOSE 8000
CMD ["python", "app.py"]
```

```bash,norun
docker build -t myapp:1.0 .
docker run -d -p 8000:8000 myapp:1.0
```

Each instruction is a **layer** that's cached, which is why `requirements.txt` is copied and installed *before* the rest of the code: changing code doesn't reinstall every package.

## Several containers: Compose

```yaml
# compose.yaml
services:
  web:
    build: .
    ports: ["8000:8000"]
    environment:
      DATABASE_URL: postgres://app:secret@db/app
    depends_on: [db]
  db:
    image: postgres:16
    environment:
      POSTGRES_PASSWORD: secret
    volumes: [dbdata:/var/lib/postgresql/data]
volumes:
  dbdata:
```

`docker compose up -d` starts both, on a private network where `web` reaches the database by the name `db`. (Real passwords go in `.env` files or secrets, not in Git.)

## Container security in five rules

1. Use small, official base images (`-slim`, `alpine`, distroless) and update them.
2. Don't run as root inside the container (`USER`).
3. Never bake secrets into images; anyone with the image can read them.
4. Scan images for known vulnerabilities (`docker scout cves`, Trivy).
5. Pin versions (`postgres:16`, not `latest`) so builds are repeatable.

@@quiz
What do containers share with the host that VMs don't?
* The kernel
- The file system
- Network ports by default
- Nothing
? That's why they're light and fast, and why a Linux container needs a Linux kernel (Docker Desktop on Windows runs one in a small VM).

@@quiz
`docker run -p 8080:80 nginx`. Which address shows the nginx page on your PC?
* `http://localhost:8080`
- `http://localhost:80`
- `http://nginx:8080`
- `http://localhost:8080:80`
? `-p host:container`: host port 8080 forwards to the container's port 80.

@@quiz
Why copy `requirements.txt` and install packages before copying the rest of the code in a Dockerfile?
* So the slow install layer stays cached when only the code changes
- Docker requires that order
- It makes the image smaller
- pip can't see other files
? Layers are rebuilt from the first changed instruction onward.

@@drill
Which command lists running containers?
= docker ps
= podman ps
? Add -a to include stopped ones.

@@flashcards
image :: A packaged, read-only template for containers
container :: A running (or stopped) instance of an image
docker run -d -p 8080:80 :: Run in the background, mapping host port 8080 to container port 80
docker exec -it name sh :: A shell inside a running container
volume (-v) :: Storage that outlives the container
Dockerfile :: Instructions to build an image (FROM, COPY, RUN, CMD)
docker compose up -d :: Start all services in compose.yaml
Kubernetes :: Runs and scales containers across many machines

@@exercise Read a Dockerfile
Write `base_image` that reads a Dockerfile from **stdin** and prints the image of its **last** `FROM` line (multi-stage builds have several; the last is what ships). Ignore `AS name`. Also print `WARNING: unpinned` on a second line if the image has no tag or uses `latest`.
@@starter
base_image() {
    echo "TODO: not done yet" >&2
    return 1
}

@@tests
check "$(base_image <<< $'FROM python:3.12-slim\nRUN pip install x')" "python:3.12-slim" "a simple Dockerfile"
check "$(base_image <<< $'FROM golang:1.23 AS build\nRUN go build\nFROM gcr.io/distroless/static:nonroot\nCOPY --from=build /app /app')" "gcr.io/distroless/static:nonroot" "multi-stage"
check "$(base_image <<< $'FROM nginx\nCOPY . /usr/share/nginx/html')" $'nginx\nWARNING: unpinned' "no tag"
check "$(base_image <<< $'from node:latest')" $'node:latest\nWARNING: unpinned' "latest (lowercase from)"
@@hint
`image=$(grep -i '^FROM' | tail -n 1 | awk '{print $2}')`. The tag is what follows the **last** `:` that isn't part of a registry with a port: for this exercise, `[[ $image != *:* || $image == *:latest ]]` is good enough.
@@solution
base_image() {
    local image
    image=$(grep -i '^FROM' | tail -n 1 | awk '{print $2}')
    echo "$image"
    if [[ $image != *:* || $image == *:latest ]]; then
        echo "WARNING: unpinned"
    fi
}
