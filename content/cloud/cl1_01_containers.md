@@lesson cl1-containers Docker and Containers
@@text
**Docker** made containers easy: package an app with everything it needs into an **image**, and run it the same way anywhere. "It works on my machine" stops being an excuse.

## Words

| Word | Meaning |
|---|---|
| **image** | a read-only template: a stack of **layers** (base OS files, runtime, your code) |
| **container** | a running (or stopped) instance of an image, with a thin writable layer on top |
| **Dockerfile** | the recipe that builds an image |
| **registry** | where images live: Docker Hub, GitHub Container Registry, AWS ECR, Azure ACR |
| **tag** | a version label: `python:3.12-slim`, `myapp:1.4.2`. `latest` is just a default name, not "newest" |
| **volume** | storage that outlives the container (databases!) |
| **OCI** | the open standard for images and runtimes, so Podman, containerd and Kubernetes all run the same images |

## The commands

```bash,norun
docker run -d --name web -p 8080:80 nginx:1.27-alpine   # detached, host port 8080 → container port 80
docker ps                     # running containers (-a: all)
docker logs -f web            # stdout/stderr of the container
docker exec -it web sh        # a shell inside it
docker stop web && docker rm web
docker images                 # local images
docker pull redis:7-alpine
docker run --rm -it ubuntu:24.04 bash    # throwaway container, removed on exit
docker run -d -v pgdata:/var/lib/postgresql/data -e POSTGRES_PASSWORD=dev postgres:16
docker system df ; docker system prune   # disk usage, clean up
```

## A Dockerfile

```text
FROM python:3.12-slim            # base image (small variant)
WORKDIR /app
COPY requirements.txt .
RUN pip install --no-cache-dir -r requirements.txt   # cached unless requirements change
COPY . .                         # your code changes most: copy it last
RUN useradd -u 10001 app
USER app                         # don't run as root
EXPOSE 8000                      # documentation: which port the app uses
CMD ["python", "app.py"]
```

Each instruction makes a **layer**, and Docker **caches** layers: order instructions from least to most frequently changing, so rebuilding after a code change takes seconds. A **`.dockerignore`** keeps junk (`.git`, `node_modules`, secrets!) out of the build. **Multi-stage builds** compile in one big image and copy only the result into a small one:

```text
FROM golang:1.23 AS build
WORKDIR /src
COPY . .
RUN go build -o /out/app

FROM gcr.io/distroless/static
COPY --from=build /out/app /app
ENTRYPOINT ["/app"]
```

## Networking and Compose

Containers on the same user-defined **network** reach each other **by name**. **Docker Compose** describes several containers in one `compose.yaml`:

```yaml
services:
  app:
    build: .
    ports: ["8000:8000"]
    environment:
      DATABASE_URL: postgres://app:dev@db:5432/app
    depends_on: [db]
  db:
    image: postgres:16
    environment:
      POSTGRES_USER: app
      POSTGRES_PASSWORD: dev
    volumes: [dbdata:/var/lib/postgresql/data]
volumes:
  dbdata:
```

`docker compose up -d`, `docker compose logs -f`, `docker compose down` (add `-v` to delete volumes too).

## Container security

- Use **small, official, pinned** base images (`-slim`, `-alpine`, distroless), and rebuild often for patches.
- **Don't run as root**; drop capabilities; read-only file system where possible.
- **Never bake secrets into images**: anyone who pulls the image can read every layer. Pass them at runtime (environment variables from a secret store, mounted files).
- **Scan** images for vulnerabilities (Docker Scout, Trivy, ECR/ACR scanning), and sign them.
- Containers are **ephemeral**: anything written inside is lost when the container is replaced. Logs go to **stdout**; data goes to **volumes** or external services.

@@quiz
What's the difference between an image and a container?
* An image is the read-only template; a container is a running instance of it
- They're the same thing
- A container is a compressed image
- An image runs; a container is stored
? One image can start any number of containers.

@@quiz
`docker run -p 8080:80 nginx`: what does `8080:80` mean?
* Host port 8080 is forwarded to port 80 inside the container
- The container uses ports 8080 to 80
- Port 80 on the host goes to 8080 in the container
- 8080 MB of RAM, 80% CPU
? host:container.

@@quiz
Why put `COPY requirements.txt` and `RUN pip install` BEFORE `COPY . .`?
* So the slow dependency layer stays cached when only your code changes
- Docker requires that order
- It makes the image run as non-root
- It encrypts the requirements
? Changing a layer invalidates the cache for every layer after it.

@@quiz
A developer put an API key in the Dockerfile with `ENV API_KEY=...` and pushed the image to a public registry. What should happen?
* Treat the key as leaked: revoke it, then pass secrets at runtime from a secret store
- Nothing, ENV values are encrypted
- Delete the tag; the key is gone
- Rename the variable
? Image layers can be inspected by anyone who pulls them.

@@quiz
A database container is recreated and all its data is gone. What was missing?
* A volume for the data directory
- A port mapping
- A Dockerfile
- A tag
? Container file systems are ephemeral.

@@quiz
In Docker Compose, how does the `app` service reach the `db` service?
* By its service name, `db`, on the Compose network
- Only through localhost
- By the host's public IP
- It can't; containers are isolated
? Compose creates a network with built-in DNS.

@@quiz
What does a multi-stage build achieve?
* A small final image with only the built result, not the compilers and build tools
- Faster networking
- Running two containers in one
- Automatic scaling
? Smaller images pull faster and have fewer vulnerabilities.

@@quiz
Where should a containerized app write its logs?
* To stdout/stderr, collected by the runtime or platform
- To a file inside the container
- To the image
- To /dev/null
? `docker logs` and Kubernetes read stdout; files inside disappear with the container.

@@flashcards
image :: Read-only layered template
container :: Running instance of an image
layer :: One Dockerfile instruction's file changes; cached
registry :: Image storage: Docker Hub, GHCR, ECR, ACR
tag :: Image version label (latest is just a name)
Dockerfile :: Recipe for an image
.dockerignore :: Files kept out of the build context
multi-stage build :: Build big, ship small
volume :: Persistent storage outside the container
-p host:container :: Port publishing
docker exec -it NAME sh :: Shell inside a running container
docker compose up -d :: Start all services in compose.yaml
OCI :: Open standard for container images and runtimes
distroless :: Images with just the app and its runtime, no shell
USER :: Dockerfile instruction to drop root

@@drill
Which Docker command shows the output (logs) of a container named web?
= docker logs web
? Add -f to follow.
