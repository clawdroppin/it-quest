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

@@exercise Simulate a rolling deployment with health checks
A **rolling** deployment replaces instances a few at a time. Write `rolling_deploy(instances, new_version, batch_size, healthy)` that updates `batch_size` instances per step and runs the health check `healthy(name, version)` on each updated instance:

- if every instance in the batch passes, go on to the next batch;
- if any fails, **stop** and roll back every instance updated so far to its old version, then return `rolled back`.

Print each step and the final versions. Run it once with a good build and once with a build that fails its health check on the third instance.
@@lang python
@@starter
def rolling_deploy(instances, new_version, batch_size, healthy):
    raise NotImplementedError


def good_build(name, version):
    return True

def bad_build(name, version):
    return name != "web-3"

for label, check, version in [("v2.4.0 (good)", good_build, "v2.4.0"), ("v2.5.0 (bad)", bad_build, "v2.5.0")]:
    fleet = {f"web-{i}": "v2.3.1" for i in range(1, 7)}
    print(label + ":")
    result = rolling_deploy(fleet, version, 2, check)
    print(f"  {result}: " + ", ".join(f"{n}={v}" for n, v in fleet.items()))
@@expected
v2.4.0 (good):
  batch ['web-1', 'web-2']: healthy
  batch ['web-3', 'web-4']: healthy
  batch ['web-5', 'web-6']: healthy
  deployed: web-1=v2.4.0, web-2=v2.4.0, web-3=v2.4.0, web-4=v2.4.0, web-5=v2.4.0, web-6=v2.4.0
v2.5.0 (bad):
  batch ['web-1', 'web-2']: healthy
  batch ['web-3', 'web-4']: FAILED on ['web-3']
  rolling back ['web-1', 'web-2', 'web-3', 'web-4']
  rolled back: web-1=v2.3.1, web-2=v2.3.1, web-3=v2.3.1, web-4=v2.3.1, web-5=v2.3.1, web-6=v2.3.1
@@requires
def rolling_deploy
rolled back
@@hint
Remember the old versions before you start, loop over the names in slices of `batch_size`, and keep a list of everything updated so the rollback knows what to undo.
@@solution
def rolling_deploy(instances, new_version, batch_size, healthy):
    old = dict(instances)
    updated = []
    names = list(instances)
    for start in range(0, len(names), batch_size):
        batch = names[start:start + batch_size]
        for name in batch:
            instances[name] = new_version
            updated.append(name)
        failed = [n for n in batch if not healthy(n, new_version)]
        print(f"  batch {batch}: " + ("healthy" if not failed else f"FAILED on {failed}"))
        if failed:
            for name in updated:
                instances[name] = old[name]
            print(f"  rolling back {updated}")
            return "rolled back"
    return "deployed"

def good_build(name, version):
    return True

def bad_build(name, version):
    return name != "web-3"

for label, check, version in [("v2.4.0 (good)", good_build, "v2.4.0"), ("v2.5.0 (bad)", bad_build, "v2.5.0")]:
    fleet = {f"web-{i}": "v2.3.1" for i in range(1, 7)}
    print(label + ":")
    result = rolling_deploy(fleet, version, 2, check)
    print(f"  {result}: " + ", ".join(f"{n}={v}" for n, v in fleet.items()))
