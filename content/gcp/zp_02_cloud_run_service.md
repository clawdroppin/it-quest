@@lesson xp-cloud_run_service Deploy a Service to Cloud Run
@@text
Ship a real service on Google Cloud: containerize a small API, push it to Artifact Registry, deploy it to Cloud Run in asia-northeast1, give it its own service account with only the roles it needs, store a secret in Secret Manager, and set a budget alert so a mistake can't cost you much. Then delete it all.

**You'll practise:** Cloud Run · Artifact Registry · service accounts and least privilege · Secret Manager · budgets

## Milestones

1. **Project**: create a project, link billing, set a ¥1,000 budget alert first.
2. **Build**: `gcloud builds submit` (or docker push) into Artifact Registry.
3. **Deploy**: `gcloud run deploy` with `--service-account` and `--region asia-northeast1`.
4. **Secret**: create a secret and mount it as an environment variable.
5. **Clean up**: delete the service, the image and the project; note what each step cost.

## You're done when

- The service answers on its HTTPS URL, runs as its own service account, and everything is deleted afterwards.

## Getting help

Open the project, then the 🤖 **AI Helper**: it sees the lesson and your files. Good questions to start with:

- *"Which IAM roles does my Cloud Run service account actually need?"*
- *"Explain what happens when Cloud Run scales to zero."*

Ask for hints, reviews and explanations rather than finished code: you learn by writing it.
@@project cloud_run_service Deploy a Service to Cloud Run
Create the folder, open it in VS Code, and work through `README.md` one milestone at a time.
@@file README.md
# Deploy a Service to Cloud Run

Ship a real service on Google Cloud: containerize a small API, push it to Artifact Registry, deploy it to Cloud Run in asia-northeast1, give it its own service account with only the roles it needs, store a secret in Secret Manager, and set a budget alert so a mistake can't cost you much. Then delete it all.

## Milestones

- [ ] 1. Project: create a project, link billing, set a ¥1,000 budget alert first.
- [ ] 2. Build: `gcloud builds submit` (or docker push) into Artifact Registry.
- [ ] 3. Deploy: `gcloud run deploy` with `--service-account` and `--region asia-northeast1`.
- [ ] 4. Secret: create a secret and mount it as an environment variable.
- [ ] 5. Clean up: delete the service, the image and the project; note what each step cost.

## Done when

- [ ] The service answers on its HTTPS URL, runs as its own service account, and everything is deleted afterwards.
@@file LAB.md
# Cloud Run lab

```bash
gcloud config set project YOUR_PROJECT
gcloud services enable run.googleapis.com artifactregistry.googleapis.com secretmanager.googleapis.com
gcloud artifacts repositories create apps --repository-format=docker --location=asia-northeast1
gcloud iam service-accounts create hello-run
# TODO: build, deploy with --service-account, add a secret
```

## Clean up
```bash
gcloud run services delete hello --region asia-northeast1
```
@@file main.py
import os
from http.server import BaseHTTPRequestHandler, HTTPServer


class Handler(BaseHTTPRequestHandler):
    def do_GET(self):
        greeting = os.environ.get("GREETING", "Hello")
        self.send_response(200)
        self.end_headers()
        self.wfile.write(f"{greeting} from Cloud Run\n".encode())


HTTPServer(("0.0.0.0", int(os.environ.get("PORT", "8080"))), Handler).serve_forever()
@@file Dockerfile
FROM python:3.12-slim
WORKDIR /app
COPY main.py .
USER 1000
CMD ["python", "main.py"]
