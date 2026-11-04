@@lesson d1-release-engineering Release Engineering and DORA Metrics
@@text
How do you know whether a team delivers well? The **DORA** research (DevOps Research and Assessment, the *Accelerate* book and yearly State of DevOps reports) found four delivery metrics that predict both speed and stability. This lesson explains them, how to compute them from deploy and incident records, and the release engineering practices behind good scores: trunk-based development, semantic versioning, conventional commits and automated changelogs.

## The DORA metrics

| Metric | Measures | Elite (roughly) |
|---|---|---|
| **Deployment frequency** | how often you deploy to production | on demand, several per day |
| **Lead time for changes** | commit → running in production | under a day |
| **Change failure rate** | % of deployments causing a failure needing remediation | about 0–15% |
| **Failed deployment recovery time** (formerly MTTR) | time to restore after a failed deployment | under an hour |

Speed and stability **go together**: small, frequent changes are easier to test, review and roll back. Recent reports also add **reliability** (meeting SLOs) and discuss rework rate. Use the metrics to improve a team over time, not to compare or rank teams (Goodhart's law: a measure that becomes a target stops being a good measure).

## Trunk-based development

Short-lived branches (hours to a day or two) merged to `main` frequently, behind feature flags when incomplete, with CI on every merge. Long-lived branches cause painful merges and late integration.

## Semantic versioning

**MAJOR.MINOR.PATCH**: breaking change → MAJOR; backwards-compatible feature → MINOR; bug fix → PATCH. Pre-releases: `2.0.0-rc.1` sorts before `2.0.0`. Before 1.0.0, anything may change.

## Conventional commits

`type(scope)!: description`, with types such as `feat`, `fix`, `docs`, `refactor`, `perf`, `test`, `build`, `ci`, `chore`. A `!` or a `BREAKING CHANGE:` footer marks a breaking change. Tools (semantic-release, release-please, changesets) derive the next version and changelog automatically: any breaking → major; else any `feat` → minor; else any `fix`/`perf` → patch; otherwise no release.

## Release practices

Build once, promote the same artefact through environments; tag releases; sign artefacts; keep release notes user-focused; automate the boring parts; have a rollback plan for every release.

@@quiz
Which DORA metric measures the time from commit to production?
- Deployment frequency
* Lead time for changes
- Change failure rate
- Recovery time
? Commit to running in production.

@@quiz
A team deployed 40 times; 6 deployments caused incidents needing a fix or rollback. What's the change failure rate?
- 6%
* 15%
- 40%
- 85%
? 6 / 40.

@@quiz
Current version 1.4.2. Commits since: `fix: null check`, `feat: export CSV`. What's the next version?
- 1.4.3
* 1.5.0
- 2.0.0
- 1.4.2-1
? A feature bumps minor and resets patch.

@@quiz
Which commit triggers a major release under conventional commits?
- `feat: add API`
- `fix(api): handle timeout`
* `feat(api)!: remove v1 endpoints`
- `chore: update deps`
? The ! marks a breaking change.

@@quiz
What did DORA research find about speed and stability?
- They trade off against each other
* High performers achieve both; small frequent changes help stability
- Stability requires fewer deployments
- Only speed matters
? Small batches are easier to fix.

@@flashcards
DORA four :: deploy frequency, lead time, change failure rate, recovery time
lead time for changes :: commit to production
change failure rate :: failed deploys / deploys
recovery time :: restore after failed deploy
Goodhart's law :: target ruins the measure
trunk-based development :: short-lived branches, merge often
semver MAJOR :: breaking change
semver MINOR :: compatible feature
semver PATCH :: bug fix
pre-release :: 2.0.0-rc.1 < 2.0.0
feat :: minor bump
fix :: patch bump
! or BREAKING CHANGE :: major bump
build once :: promote the same artefact

@@drill
What version follows 3.9.4 after a breaking change?
= 4.0.0
? Major bump resets minor and patch.

@@drill
Which conventional commit type marks a new feature?
= feat
? fix is for bug fixes.

@@exercise Compute DORA metrics
From deployment records (commit time, deploy time, whether it failed, and when service was restored), compute deployment frequency per week, median lead time in hours, change failure rate and median recovery time, and classify each metric with the thresholds given.
@@lang python
@@starter
from datetime import datetime
from statistics import median

DAYS = 14
deploys = [
    ("2026-10-19 09:10", "2026-10-19 11:00", None), ("2026-10-19 13:00", "2026-10-19 16:30", None),
    ("2026-10-20 10:00", "2026-10-21 09:00", "2026-10-21 09:40"), ("2026-10-22 08:00", "2026-10-22 10:00", None),
    ("2026-10-23 15:00", "2026-10-26 10:00", None), ("2026-10-26 11:00", "2026-10-26 12:30", None),
    ("2026-10-27 09:00", "2026-10-27 09:50", "2026-10-27 12:05"), ("2026-10-28 14:00", "2026-10-28 15:00", None),
    ("2026-10-29 10:00", "2026-10-29 10:45", None), ("2026-10-30 09:30", "2026-10-30 11:15", None),
]

def hours(a, b):
    raise NotImplementedError
@@expected
deployment frequency: 5.0/week (high)
lead time for changes: median 1.8 h (elite)
change failure rate: 20% (high)
failed deployment recovery time: median 1.46 h (high)
@@requires
def hours
median(
change failure rate
@@hint
Each record is `(commit, deploy, restored_or_None)`. Lead time = deploy − commit; recovery = restored − deploy for failed ones. Thresholds: frequency ≥ 7/week elite, ≥ 1 high; lead time < 24 h elite, < 168 h high; failure rate ≤ 15% elite, ≤ 30% high; recovery < 1 h elite, < 24 h high.
@@solution
from datetime import datetime
from statistics import median

DAYS = 14
deploys = [
    ("2026-10-19 09:10", "2026-10-19 11:00", None), ("2026-10-19 13:00", "2026-10-19 16:30", None),
    ("2026-10-20 10:00", "2026-10-21 09:00", "2026-10-21 09:40"), ("2026-10-22 08:00", "2026-10-22 10:00", None),
    ("2026-10-23 15:00", "2026-10-26 10:00", None), ("2026-10-26 11:00", "2026-10-26 12:30", None),
    ("2026-10-27 09:00", "2026-10-27 09:50", "2026-10-27 12:05"), ("2026-10-28 14:00", "2026-10-28 15:00", None),
    ("2026-10-29 10:00", "2026-10-29 10:45", None), ("2026-10-30 09:30", "2026-10-30 11:15", None),
]

def parse(text):
    return datetime.strptime(text, "%Y-%m-%d %H:%M")

def hours(a, b):
    return (parse(b) - parse(a)).total_seconds() / 3600

def level(value, elite, high, lower_is_better=True):
    if lower_is_better:
        return "elite" if value < elite else "high" if value < high else "medium/low"
    return "elite" if value >= elite else "high" if value >= high else "medium/low"

per_week = len(deploys) / DAYS * 7
lead = median(hours(c, d) for c, d, _ in deploys)
failed = [(d, r) for _, d, r in deploys if r]
cfr = len(failed) / len(deploys)
recovery = median(hours(d, r) for d, r in failed)
print(f"deployment frequency: {per_week:.1f}/week ({level(per_week, 7, 1, False)})")
print(f"lead time for changes: median {lead:.1f} h ({level(lead, 24, 168)})")
print(f"change failure rate: {cfr:.0%} ({'elite' if cfr <= 0.15 else 'high' if cfr <= 0.30 else 'medium/low'})")
print(f"failed deployment recovery time: median {recovery:.2f} h ({level(recovery, 1, 24)})")

@@exercise Next version from conventional commits
Parse conventional commit messages since the last tag and compute the next semantic version (breaking → major, feat → minor, fix/perf → patch, else no release; versions below 1.0.0 bump minor for breaking changes). Generate a changelog grouped into Breaking changes, Features and Fixes.
@@lang python
@@starter
import re

histories = {
    "1.4.2": ["fix(auth): refresh token race", "docs: update README", "feat(export): CSV export", "chore: bump deps"],
    "2.0.3": ["fix: off-by-one in pager", "perf(db): add index"],
    "3.1.0": ["feat(api)!: remove v1 endpoints", "feat: webhooks", "fix: retry on 503"],
    "0.9.1": ["refactor: split module", "feat!: new config format"],
    "1.0.0": ["docs: typo", "ci: cache deps"],
}
PATTERN = re.compile(r"^(\w+)(\([\w-]+\))?(!)?: (.+)$")

def next_version(current, commits):
    raise NotImplementedError
@@expected
1.4.2 -> 1.5.0
  Features: CSV export
  Fixes: refresh token race
2.0.3 -> 2.0.4
  Fixes: off-by-one in pager; add index
3.1.0 -> 4.0.0
  BREAKING CHANGES: remove v1 endpoints
  Features: webhooks
  Fixes: retry on 503
0.9.1 -> 0.10.0
  BREAKING CHANGES: new config format
1.0.0: no release (only ci, docs)
@@requires
def next_version
PATTERN.match(
BREAKING
@@hint
Classify each commit as breaking (`!` or a body containing "BREAKING CHANGE"), feat, fix (`fix` or `perf`) or other. For 0.x versions, a breaking change bumps the minor, as many tools do.
@@solution
import re

histories = {
    "1.4.2": ["fix(auth): refresh token race", "docs: update README", "feat(export): CSV export", "chore: bump deps"],
    "2.0.3": ["fix: off-by-one in pager", "perf(db): add index"],
    "3.1.0": ["feat(api)!: remove v1 endpoints", "feat: webhooks", "fix: retry on 503"],
    "0.9.1": ["refactor: split module", "feat!: new config format"],
    "1.0.0": ["docs: typo", "ci: cache deps"],
}
PATTERN = re.compile(r"^(\w+)(\([\w-]+\))?(!)?: (.+)$")

def classify(message):
    m = PATTERN.match(message)
    kind, scope, bang, text = m.groups()
    if bang or "BREAKING CHANGE" in message:
        return "breaking", text
    if kind == "feat":
        return "feat", text
    if kind in ("fix", "perf"):
        return "fix", text
    return "other", text

def next_version(current, commits):
    major, minor, patch = map(int, current.split("."))
    kinds = {classify(c)[0] for c in commits}
    if "breaking" in kinds:
        return f"0.{minor + 1}.0" if major == 0 else f"{major + 1}.0.0"
    if "feat" in kinds:
        return f"{major}.{minor + 1}.0"
    if "fix" in kinds:
        return f"{major}.{minor}.{patch + 1}"
    return None

for current, commits in histories.items():
    new = next_version(current, commits)
    if new is None:
        print(f"{current}: no release (only {', '.join(sorted({c.split(':')[0] for c in commits}))})")
        continue
    print(f"{current} -> {new}")
    for title, kind in (("BREAKING CHANGES", "breaking"), ("Features", "feat"), ("Fixes", "fix")):
        items = [text for c in commits for k, text in [classify(c)] if k == kind]
        if items:
            print(f"  {title}: " + "; ".join(items))
