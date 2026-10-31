@@lesson xp-containerize Containerize a Web App
@@text
Take a small Python web app (included), write a production-quality Dockerfile (slim base image, non-root user, layer caching, a health check), then a compose file that adds a Redis cache, and measure how much smaller and faster your image gets with each improvement.

**You'll practise:** Dockerfile instructions · image layers and caching · non-root containers · docker compose · health checks

## Milestones

1. **Run it locally**: `python app.py` and open http://localhost:8000.
2. **Dockerfile v1**: the simplest one that works; note the image size.
3. **Dockerfile v2**: slim base, requirements first for caching, non-root USER, HEALTHCHECK.
4. **Compose**: add Redis and count page visits in it.
5. **Compare**: a table of image size and rebuild time for v1 and v2 in NOTES.md.

## You're done when

- `docker compose up` runs the app with Redis, as a non-root user, and the health check passes.

## Getting help

Open the project, then the 🤖 **AI Helper**: it sees the lesson and your files. Good questions to start with:

- *"Why does copying requirements.txt before the code speed up rebuilds?"*
- *"Review my Dockerfile for security problems."*

Ask for hints, reviews and explanations rather than finished code: you learn by writing it.
@@project containerize Containerize a Web App
Create the folder, open it in VS Code, and work through `README.md` one milestone at a time.
@@file README.md
# Containerize a Web App

Take a small Python web app (included), write a production-quality Dockerfile (slim base image, non-root user, layer caching, a health check), then a compose file that adds a Redis cache, and measure how much smaller and faster your image gets with each improvement.

## Milestones

- [ ] 1. Run it locally: `python app.py` and open http://localhost:8000.
- [ ] 2. Dockerfile v1: the simplest one that works; note the image size.
- [ ] 3. Dockerfile v2: slim base, requirements first for caching, non-root USER, HEALTHCHECK.
- [ ] 4. Compose: add Redis and count page visits in it.
- [ ] 5. Compare: a table of image size and rebuild time for v1 and v2 in NOTES.md.

## Done when

- [ ] `docker compose up` runs the app with Redis, as a non-root user, and the health check passes.
@@file app.py
"""A tiny web app using only the standard library."""
from http.server import BaseHTTPRequestHandler, HTTPServer


class Handler(BaseHTTPRequestHandler):
    def do_GET(self):
        body = b"ok\n" if self.path == "/health" else b"Hello from a container!\n"
        self.send_response(200)
        self.send_header("Content-Type", "text/plain")
        self.end_headers()
        self.wfile.write(body)


if __name__ == "__main__":
    HTTPServer(("0.0.0.0", 8000), Handler).serve_forever()
@@file Dockerfile
# v1: works, but big, slow to rebuild, and runs as root. Improve it.
FROM python:3.12
COPY . /app
WORKDIR /app
CMD ["python", "app.py"]
@@file compose.yaml
services:
  web:
    build: .
    ports: ["8000:8000"]
  # TODO: redis
