@@lesson d4-github-actions GitHub Actions: Build, Test and Deploy Pipelines
@@concepts automation, testing, maps
@@lang python
@@text
**GitHub Actions** runs your pipeline on GitHub's machines (or your own) whenever something happens in the repository: a push, a pull request, a schedule, a button press. It's the most common CI/CD tool for new projects, and its ideas (triggers, jobs, steps, runners, secrets) carry over to GitLab CI, Azure Pipelines and Jenkins.

## Anatomy of a workflow

Workflows are YAML files in `.github/workflows/`. This one tests a Python project on every push and pull request:

```yaml
name: CI                                  # shown in the Actions tab

on:                                       # triggers
  push:
    branches: [main]
  pull_request:                           # every PR, from any branch
  workflow_dispatch:                      # a "Run workflow" button

permissions:
  contents: read                          # least privilege for the automatic GITHUB_TOKEN

jobs:
  test:
    runs-on: ubuntu-latest                # the runner: a fresh VM for every job
    steps:
      - uses: actions/checkout@v4         # an action: reusable code from another repo
      - uses: actions/setup-python@v5
        with:
          python-version: "3.12"
          cache: pip                      # cache downloaded packages between runs
      - run: pip install -r requirements.txt
      - run: ruff check .                 # lint
      - run: pytest -q --junitxml=report.xml
      - uses: actions/upload-artifact@v4  # keep files after the job ends
        if: always()                      # even when the tests failed
        with:
          name: test-report
          path: report.xml
```

| Word | Meaning |
|---|---|
| **workflow** | one YAML file: triggers + jobs |
| **event / trigger** | what starts it: `push`, `pull_request`, `schedule` (cron), `workflow_dispatch`, `release`, `workflow_call` |
| **job** | a set of steps on one runner; jobs run **in parallel** unless linked with `needs` |
| **step** | one command (`run:`) or one action (`uses:`) |
| **runner** | the machine: `ubuntu-latest`, `windows-latest`, `macos-latest`, or self-hosted |
| **action** | a reusable step from the Marketplace or your own repo |
| **artifact** | files saved from a job (reports, builds) to download or pass to later jobs |

## Jobs that depend on each other

```yaml
jobs:
  lint:     { runs-on: ubuntu-latest, steps: [...] }
  test:     { runs-on: ubuntu-latest, steps: [...] }
  build:
    needs: [lint, test]                   # waits for both; skipped if either fails
    runs-on: ubuntu-latest
    steps: [...]
  deploy:
    needs: build
    if: github.ref == 'refs/heads/main'   # only from main, never from a PR
    environment: production               # can require a human approval
    runs-on: ubuntu-latest
    steps: [...]
```

`lint` and `test` run at the same time; `build` waits for both; `deploy` waits for `build`. The first exercise computes exactly this order.

## Matrix builds

Test on several versions and systems with one job definition:

```yaml
  test:
    strategy:
      fail-fast: false                    # finish every combination even if one fails
      matrix:
        os: [ubuntu-latest, windows-latest]
        python: ["3.11", "3.12", "3.13"]
        exclude:
          - os: windows-latest
            python: "3.11"
    runs-on: ${{ matrix.os }}
    steps:
      - uses: actions/setup-python@v5
        with: { python-version: "${{ matrix.python }}" }
```

That's 2 × 3 = 6 combinations, minus the excluded one = 5 jobs in parallel.

## Expressions, contexts, variables and secrets

- `${{ ... }}` is an expression: `${{ github.sha }}`, `${{ github.event_name }}`, `${{ matrix.os }}`, `${{ secrets.DB_PASSWORD }}`, `${{ vars.REGION }}`.
- Conditions: `if: github.event_name == 'push'`, `if: failure()`, `if: always()`, `if: contains(github.event.head_commit.message, '[skip deploy]') == false`.
- `env:` at workflow, job or step level sets environment variables.
- **Secrets** (Settings → Secrets and variables → Actions) are encrypted and masked in logs (`***`). **Environment** secrets only unlock for jobs that use that environment, after its protection rules (required reviewers, wait timers, allowed branches) pass.
- A step can pass data on: `echo "tag=v1.2.3" >> "$GITHUB_OUTPUT"`, then `${{ steps.version.outputs.tag }}`. Jobs pass outputs with `outputs:` and `needs.<job>.outputs.<name>`.

## Deploying to a cloud without stored keys: OIDC

Long-lived cloud keys in secrets get leaked. Instead, the workflow proves who it is with a short-lived **OIDC** token, and the cloud trusts *this repository and branch* only:

```yaml
permissions:
  id-token: write                         # allow requesting the OIDC token
  contents: read
steps:
  - uses: aws-actions/configure-aws-credentials@v4
    with:
      role-to-assume: arn:aws:iam::123456789012:role/github-deploy
      aws-region: ap-northeast-1
  - run: aws s3 sync ./site s3://my-site-bucket --delete
```

On the AWS side, an IAM role trusts `token.actions.githubusercontent.com` with a condition like `repo:acme/site:ref:refs/heads/main`. Azure (`azure/login` with federated credentials) and Google Cloud (Workload Identity Federation) work the same way.

## A complete delivery pipeline

```yaml
name: Deliver
on:
  push: { branches: [main] }
  pull_request:

concurrency:                              # one deploy at a time; newer runs cancel older ones
  group: deliver-${{ github.ref }}
  cancel-in-progress: true

jobs:
  test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - run: make test

  image:
    needs: test
    runs-on: ubuntu-latest
    permissions: { contents: read, packages: write }
    outputs:
      tag: ${{ steps.meta.outputs.tag }}
    steps:
      - uses: actions/checkout@v4
      - id: meta
        run: echo "tag=ghcr.io/${{ github.repository }}:${{ github.sha }}" >> "$GITHUB_OUTPUT"
      - uses: docker/login-action@v3
        with: { registry: ghcr.io, username: "${{ github.actor }}", password: "${{ secrets.GITHUB_TOKEN }}" }
      - uses: docker/build-push-action@v6
        with:
          push: ${{ github.event_name == 'push' }}     # build on PRs, push only from main
          tags: ${{ steps.meta.outputs.tag }}
          cache-from: type=gha
          cache-to: type=gha,mode=max

  plan:
    if: github.event_name == 'pull_request'
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: hashicorp/setup-terraform@v3
      - run: terraform init -input=false && terraform plan -input=false -no-color
        working-directory: infra

  deploy:
    if: github.event_name == 'push'
    needs: image
    environment: production
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - run: kubectl set image deployment/web web=${{ needs.image.outputs.tag }}
      - run: kubectl rollout status deployment/web --timeout=120s
```

## Reuse: composite actions and reusable workflows

- A **composite action** (`action.yml` with `runs: using: composite`) bundles steps you repeat, like "set up our toolchain".
- A **reusable workflow** (`on: workflow_call`) is a whole pipeline other repositories call with `uses: acme/ci/.github/workflows/python.yml@v2`. Platform teams use these so every service gets the same tested pipeline.

## Pipeline security

- **Pin third-party actions to a full commit SHA** (`uses: some/action@8f4b7f84...`), not a tag that can be moved. A compromised tag once leaked secrets from thousands of repositories. Dependabot can keep SHA pins updated.
- Set `permissions:` explicitly and as small as possible.
- Be careful with `pull_request_target` and with `${{ github.event.pull_request.title }}` inside `run:` (script injection): pass untrusted input through `env:` instead.
- Forks' pull requests don't get your secrets, by design.
- Self-hosted runners on public repositories can run strangers' code: keep them for private repositories.

## When it fails

Read the failing step's log; re-run with **debug logging** (Re-run jobs → Enable debug logging); reproduce locally (run the same `make test`, or the `act` tool); and keep pipeline logic in scripts (`make`, `./ci/test.sh`) so it's the same on your laptop and in CI.

@@quiz
Two jobs have no `needs`. How do they run?
- One after the other, in file order
* In parallel, on separate runners
- Only the first one runs
- They share one runner
? Use `needs` to order jobs; everything else runs at the same time.

@@quiz
Why is OIDC better than storing cloud access keys as secrets?
- It's faster
* The workflow gets short-lived credentials, and the cloud only trusts this repository and branch, so there's no long-lived key to leak
- Secrets don't work with AWS
- It doesn't need any IAM configuration
? Requires `permissions: id-token: write` and a trust policy on the cloud side.

@@quiz
Why pin actions to a full commit SHA instead of `@v4`?
- SHAs run faster
* A tag can be moved to malicious code; a commit SHA can't change
- GitHub requires it
- Tags don't support inputs
? It's a supply-chain protection. Dependabot can update the pins for you.

@@quiz
How do you make a deploy job wait for a human approval?
- `if: approved()`
* Give it an `environment` whose protection rules require reviewers
- Add `sleep 3600`
- Use `workflow_dispatch` only
? Environments can also restrict which branches may deploy and hold their own secrets.

@@quiz
A matrix has `os: [ubuntu, windows]` and `python: ["3.11", "3.12", "3.13"]` with one exclude. How many jobs run?
- 3
* 5
- 6
- 7
? 2 × 3 = 6 combinations, minus one excluded.

@@flashcards
`.github/workflows/*.yml` :: where workflows live
`on:` :: the triggers: push, pull_request, schedule, workflow_dispatch, workflow_call
job / step / runner :: a unit on one machine / one command or action / the machine
`needs:` :: run this job after others succeed
`strategy.matrix` :: one job definition, many combinations
`fail-fast: false` :: let every matrix job finish
`${{ secrets.X }}` :: an encrypted secret, masked in logs
`$GITHUB_OUTPUT` :: pass values from a step to later steps and jobs
`environment:` :: protection rules (approvals) and its own secrets
`permissions:` :: scopes of the automatic GITHUB_TOKEN; keep minimal
OIDC (`id-token: write`) :: short-lived cloud credentials, no stored keys
`concurrency:` :: one run at a time per group; cancel older runs
`actions/cache` / `cache: pip` :: reuse downloads between runs
artifact :: files kept after a job (`upload-artifact`)
reusable workflow :: `on: workflow_call`, used with `uses: org/repo/.github/workflows/x.yml@ref`
composite action :: several steps packaged as one action
pin by SHA :: protects against moved or hijacked tags
script injection :: untrusted `${{ }}` text inside `run:`; pass it via `env:`

@@exercise In which order do the jobs run?
Write `stages(jobs)`. `jobs` maps each job name to the list of jobs it `needs`. Return a list of **stages**: each stage is a sorted list of the jobs that can start once all earlier stages are finished (jobs in the same stage run in parallel). If the jobs depend on each other in a circle, raise `ValueError("cycle")`.
@@starter
def stages(jobs):
    pass


print(stages({"lint": [], "test": [], "build": ["lint", "test"], "deploy": ["build"]}))
@@tests
check(stages({"lint": [], "test": [], "build": ["lint", "test"], "deploy": ["build"]}), [["lint", "test"], ["build"], ["deploy"]], "the pipeline from the lesson")
check(stages({"a": []}), [["a"]], "a single job")
check(stages({"docs": [], "unit": [], "e2e": ["unit"], "release": ["docs", "e2e"]}), [["docs", "unit"], ["e2e"], ["release"]], "independent docs job")
check(stages({}), [], "no jobs")
try:
    stages({"a": ["b"], "b": ["a"]})
    check("no error", "ValueError", "a cycle")
except ValueError as e:
    check(str(e), "cycle", "a cycle")
@@hint
Keep a set `done`. Repeat: `ready = sorted(j for j, needs in jobs.items() if j not in done and all(n in done for n in needs))`. If nothing is ready but jobs are left, it's a cycle. Otherwise append `ready` and add it to `done`.
@@solution
def stages(jobs):
    done = set()
    result = []
    while len(done) < len(jobs):
        ready = sorted(j for j, needs in jobs.items() if j not in done and all(n in done for n in needs))
        if not ready:
            raise ValueError("cycle")
        result.append(ready)
        done.update(ready)
    return result


print(stages({"lint": [], "test": [], "build": ["lint", "test"], "deploy": ["build"]}))

@@exercise Expand a build matrix
Write `expand(matrix)` the way GitHub does it (simplified): every combination of the list-valued keys, **in key order**, with the **last key changing fastest**. Then remove every combination that matches an entry of `exclude` (an exclude matches when all of its keys have the same values), and finally append each entry of `include` as an extra combination. `exclude` and `include` are optional. Return a list of dicts.
@@starter
def expand(matrix):
    pass


for combo in expand({"os": ["ubuntu", "windows"], "python": ["3.12", "3.13"]}):
    print(combo)
@@tests
check(expand({"os": ["ubuntu", "windows"], "python": ["3.12", "3.13"]}),
      [{"os": "ubuntu", "python": "3.12"}, {"os": "ubuntu", "python": "3.13"},
       {"os": "windows", "python": "3.12"}, {"os": "windows", "python": "3.13"}], "every combination")
check(len(expand({"os": ["u", "w"], "py": ["1", "2", "3"], "exclude": [{"os": "w", "py": "1"}]})), 5, "one excluded")
check(expand({"os": ["u", "w"], "py": ["1"], "exclude": [{"os": "w"}]}), [{"os": "u", "py": "1"}], "a partial exclude")
check(expand({"node": ["20"], "include": [{"node": "22", "experimental": True}]}),
      [{"node": "20"}, {"node": "22", "experimental": True}], "include adds a combination")
@@hint
`keys = [k for k in matrix if k not in ("include", "exclude")]`, then `itertools.product(*(matrix[k] for k in keys))` gives tuples in the right order; `dict(zip(keys, values))` makes each one a dict. An exclude `ex` matches `combo` when `all(combo.get(k) == v for k, v in ex.items())`.
@@solution
from itertools import product


def expand(matrix):
    keys = [k for k in matrix if k not in ("include", "exclude")]
    combos = [dict(zip(keys, values)) for values in product(*(matrix[k] for k in keys))]
    for ex in matrix.get("exclude", []):
        combos = [c for c in combos if not all(c.get(k) == v for k, v in ex.items())]
    combos.extend(dict(inc) for inc in matrix.get("include", []))
    return combos


for combo in expand({"os": ["ubuntu", "windows"], "python": ["3.12", "3.13"]}):
    print(combo)

@@exercise Find unpinned actions
Write `unpinned(workflow_text)` that returns, in order, every action reference from a `uses:` line that is **not pinned to a full 40-character commit SHA**. Ignore local actions (starting with `./`) and Docker images (starting with `docker://`). A reference is everything after `uses:` up to the first space or `#`, with quotes removed.
@@starter
import re

WORKFLOW = """
jobs:
  build:
    steps:
      - uses: actions/checkout@v4
      - uses: actions/setup-python@0b93645e9fea7318ecaed2b359559ac225c90a2b # v5.3.0
      - uses: ./.github/actions/setup-tools
      - uses: "some-org/deploy-action@main"
      - uses: docker://alpine:3.20
      - run: echo uses: not-an-action
"""


def unpinned(workflow_text):
    pass


print(unpinned(WORKFLOW))
@@tests
check(unpinned(WORKFLOW), ["actions/checkout@v4", "some-org/deploy-action@main"], "the sample workflow")
check(unpinned("steps:\n  - uses: a/b@" + "c" * 40), [], "pinned")
check(unpinned("steps:\n  - uses: a/b@" + "c" * 39), ["a/b@" + "c" * 39], "39 characters is not a full SHA")
check(unpinned("  - uses: a/b\n"), ["a/b"], "no ref at all")
check(unpinned("  - run: ls\n"), [], "no actions")
@@hint
Match lines with `re.match(r"\s*-?\s*uses:\s*(\S+)", line)` so `run: echo uses:` isn't matched. Strip quotes with `.strip("'\"")`, drop anything after `#`. Pinned means `re.fullmatch(r"[0-9a-f]{40}", ref.split("@")[1])` when there is an `@`.
@@solution
import re

WORKFLOW = """
jobs:
  build:
    steps:
      - uses: actions/checkout@v4
      - uses: actions/setup-python@0b93645e9fea7318ecaed2b359559ac225c90a2b # v5.3.0
      - uses: ./.github/actions/setup-tools
      - uses: "some-org/deploy-action@main"
      - uses: docker://alpine:3.20
      - run: echo uses: not-an-action
"""


def unpinned(workflow_text):
    found = []
    for line in workflow_text.splitlines():
        m = re.match(r"\s*-?\s*uses:\s*([^\s#]+)", line)
        if not m:
            continue
        ref = m.group(1).strip("'\"")
        if ref.startswith("./") or ref.startswith("docker://"):
            continue
        _, _, version = ref.partition("@")
        if not re.fullmatch(r"[0-9a-f]{40}", version):
            found.append(ref)
    return found


print(unpinned(WORKFLOW))
