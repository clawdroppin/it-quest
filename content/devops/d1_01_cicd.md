@@lesson d1-cicd CI/CD Pipelines
@@text
- **Continuous Integration (CI)**: every push is automatically built and tested, so problems are found in minutes, not at release time.
- **Continuous Delivery**: every change that passes is *ready* to release, with a button press.
- **Continuous Deployment**: every change that passes goes to production automatically.

## A typical pipeline

```text
push / PR ─▶ build ─▶ lint + unit tests ─▶ security scans ─▶ package (image) ─▶ deploy to staging ─▶ integration tests ─▶ approval ─▶ production
              fail fast: the cheapest checks run first                                           ▲ monitoring decides: continue or roll back
```

Tools: **GitHub Actions**, **GitLab CI**, **Azure Pipelines**, **Jenkins**, CircleCI, AWS CodePipeline. They all run **jobs** made of **steps** on **runners** (agents), triggered by events, defined in YAML in the repository.

```yaml
name: ci
on: [push, pull_request]
jobs:
  test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4          # a reusable action
      - uses: actions/setup-python@v5
        with: { python-version: "3.12" }
      - run: pip install -r requirements.txt
      - run: python -m pytest
  deploy:
    needs: test                            # runs only after test succeeds
    if: github.ref == 'refs/heads/main'
    environment: production                # can require a reviewer
    runs-on: ubuntu-latest
    steps:
      - run: ./deploy.sh
        env:
          API_TOKEN: ${{ secrets.API_TOKEN }}   # from the encrypted secret store, masked in logs
```

## Good pipeline habits

- **Fast feedback**: keep CI under ~10 minutes; cache dependencies; run jobs in parallel (a **matrix** over versions/OSs).
- **Build once, deploy many**: the same artifact (image, package) goes to staging and then production; only config changes.
- **Secrets** live in the CI's secret store or a vault, never in the YAML. Prefer short-lived credentials (**OIDC** federation to AWS/Azure instead of long-lived keys). Give the pipeline token **least privilege**.
- **Shift left on security** (**DevSecOps**): dependency scanning (Dependabot, Snyk), **SAST** (static code analysis), secret scanning, image scanning, **IaC scanning**; **DAST** against staging.
- Pin third-party actions/plugins to versions (or commit SHAs): the pipeline is part of your **supply chain**.

## Deployment strategies

| Strategy | How | Rollback |
|---|---|---|
| **Recreate** | stop old, start new | downtime |
| **Rolling** | replace instances a few at a time | roll forward/back gradually |
| **Blue/green** | run the new version (green) beside the old (blue), switch traffic at once | switch back instantly |
| **Canary** | send a small % of traffic to the new version, watch metrics, increase | move the % back |
| **Feature flags** | deploy dark, turn on for some users | flip the flag |

## DevOps metrics: DORA

The four (now five) **DORA** metrics measure delivery performance: **deployment frequency**, **lead time for changes**, **change failure rate**, **time to restore service** (and reliability). Elite teams deploy on demand with low failure rates, because small changes are easy to test and easy to undo.

@@quiz
What's the difference between continuous delivery and continuous deployment?
* Delivery keeps every change ready to release with a manual step; deployment releases automatically
- They're the same
- Delivery has no tests
- Deployment needs no pipeline
? Many regulated companies stop at continuous delivery with an approval gate.

@@quiz
Where should a pipeline get its cloud deployment credentials?
* From the CI secret store or, better, short-lived OIDC federation; never from the YAML in the repository
- Hard-coded in the workflow file
- From a README
- From the developer's laptop
? Anything in the repository is visible to everyone with read access, forever.

@@quiz
A new version goes to 5% of users while error rates are watched, then to 25%, 50%, 100%. Which strategy?
* Canary
- Blue/green
- Recreate
- Big bang
? Named after canaries in coal mines: an early warning.

@@quiz
Two identical environments exist; traffic is switched from the old to the new in one step, and back if problems appear. Which strategy?
* Blue/green
- Canary
- Rolling
- Recreate
? It needs double capacity during the switch.

@@quiz
Why "build once, deploy many"?
* So production runs exactly the artifact that was tested in staging
- To save disk space
- Because rebuilding is impossible
- To skip testing
? Rebuilding per environment can pull different dependency versions.

@@quiz
Static analysis of source code for vulnerabilities in the pipeline is called...
* SAST
- DAST
- DORA
- SLA
? DAST tests the running app from the outside.

@@quiz
Which is NOT one of the DORA delivery metrics?
* Lines of code written per day
- Deployment frequency
- Lead time for changes
- Change failure rate
? Lines of code measure activity, not outcomes.

@@flashcards
CI :: Build and test every change automatically
continuous delivery :: Always releasable; manual release step
continuous deployment :: Automatic release when checks pass
runner / agent :: Machine that executes pipeline jobs
matrix :: Run a job over several versions/OSs
artifact :: Build output passed between jobs or stages
build once, deploy many :: Same artifact through every environment
OIDC federation :: Short-lived cloud credentials for pipelines
DevSecOps / shift left :: Security checks early in the pipeline
SAST / DAST :: Static code analysis / testing the running app
rolling deployment :: Replace instances gradually
blue/green :: Switch all traffic between two environments
canary :: Gradually increase traffic to the new version
DORA metrics :: Deploy frequency, lead time, change failure rate, time to restore

@@drill
In GitHub Actions, which key makes a job wait for another job to succeed?
= needs
? needs: test
