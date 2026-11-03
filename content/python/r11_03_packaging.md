@@lesson r11-packaging Packaging and Dependencies: pyproject.toml, uv and Versions
@@concepts modules
@@text
Sooner or later your script becomes something others install: a CLI for the team, a shared library, a service in a container. This lesson covers how modern Python projects declare themselves, manage dependencies reproducibly, and publish releases.

## pyproject.toml: one file for the project

```toml
[project]
name = "assetctl"
version = "1.4.0"
description = "Manage the IT asset inventory from the command line"
readme = "README.md"
requires-python = ">=3.11"
license = "MIT"
authors = [{ name = "IT Team", email = "it@corp.example" }]
dependencies = [
    "httpx>=0.27,<1",
    "typer>=0.12",
    "rich",
]

[project.optional-dependencies]
excel = ["openpyxl>=3.1"]           # pip install "assetctl[excel]"

[project.scripts]
assetctl = "assetctl.cli:app"       # installs an `assetctl` command

[dependency-groups]
dev = ["pytest>=8", "ruff", "mypy"]

[build-system]
requires = ["hatchling"]
build-backend = "hatchling.build"

[tool.ruff]
line-length = 100

[tool.pytest.ini_options]
testpaths = ["tests"]
```

- `[project]` is standard metadata (PEP 621): name, version, dependencies, entry points.
- `[build-system]` names the **build backend** that turns the source into installable files: hatchling, setuptools, flit, pdm-backend, or maturin for Rust extensions.
- `[tool.*]` sections configure tools (ruff, pytest, mypy, coverage) so there's one config file instead of many.
- Use the **src layout** (`src/assetctl/__init__.py`): tests then run against the installed package, not accidentally against loose files.

## Wheels and sdists

`python -m build` (or `uv build`) produces:

- a **wheel** (`assetctl-1.4.0-py3-none-any.whl`): a ready-to-install zip; extensions with compiled code get platform-specific wheels (`cp313-win_amd64`).
- an **sdist** (`.tar.gz`): the source, for building elsewhere.

Publish with `uv publish` or `twine upload dist/*` to **PyPI**, or to a private index (Azure Artifacts, GitLab, Nexus, Artifactory) for company code. Use **trusted publishing** from CI instead of long-lived API tokens.

## Version numbers

Python uses **PEP 440** versions, compatible with SemVer for normal releases: `1.4.0`, `1.4.1`, `2.0.0`, plus pre-releases `2.0.0a1`, `2.0.0b2`, `2.0.0rc1` and post/dev releases.

Version **specifiers** in dependencies:

| Specifier | Meaning |
|---|---|
| `>=1.4` | 1.4 or newer |
| `>=1.4,<2` | 1.4 up to, not including, 2.0 (a common safe range) |
| `~=1.4.2` | compatible release: `>=1.4.2, <1.5` |
| `==1.4.*` | any 1.4.x |
| `==1.4.2` | exactly (only in lock files or for known problems) |
| `!=1.4.3` | exclude a broken release |

**Libraries** declare broad ranges (so they combine with other packages); **applications** pin exact versions in a **lock file** so every install is identical.

## uv: the modern all-in-one tool

**uv** (written in Rust, by Astral) replaces pip, pip-tools, virtualenv, pipx and pyenv, and is 10–100× faster:

```text
uv init assetctl            # new project with pyproject.toml
uv add httpx typer          # add dependencies (updates pyproject + uv.lock)
uv add --dev pytest
uv run pytest               # runs in the project's .venv, syncing it first
uv lock --upgrade-package httpx
uv python install 3.13      # manage Python versions
uvx ruff check .            # run a tool without installing it
uv tool install assetctl    # install a CLI globally in its own environment
```

`uv.lock` records the exact version and hash of every package for every platform. Commit it for applications.

The classic equivalents still work everywhere: `python -m venv .venv`, `pip install -e ".[dev]"`, `pip freeze > requirements.txt`, `pip install -r requirements.txt`. **Poetry** and **PDM** are other full project managers.

## Dependency hygiene

- **Fewer dependencies, fewer problems**: every package is code you trust and must update.
- Watch for vulnerabilities: `pip-audit`, GitHub Dependabot or Renovate pull requests.
- Check licences for company policy (`pip-licenses`).
- Beware **typosquatting** (`reqeusts` instead of `requests`): double-check names, prefer well-known packages.
- Private package names can be hijacked by public ones with the same name (**dependency confusion**): configure your index carefully.

## Shipping applications to non-Python users

- `uv tool install` / `pipx install` for colleagues who have Python.
- **PyInstaller** or **Nuitka** to build a standalone `.exe`.
- A **Docker image** for services (`FROM python:3.13-slim`, copy the lock file, `uv sync --frozen`).
- **Briefcase** (BeeWare) for desktop and mobile apps.

@@quiz
What does `~=1.4.2` allow?
- Exactly 1.4.2
* At least 1.4.2 but below 1.5
- Anything 1.x
- Anything newer than 1.4.2
? The "compatible release" operator.

@@quiz
Why should an application commit a lock file?
* So every install (CI, servers, colleagues) gets exactly the same package versions
- It makes the package smaller
- PyPI requires it
- Lock files replace pyproject.toml
? Reproducible builds.

@@quiz
Should a library pin `requests==2.32.3` in its dependencies?
- Yes, always pin
* No; libraries should use ranges so they can be combined with other packages
- Only on Windows
- Only for dev dependencies
? Exact pins in libraries cause unsolvable conflicts.

@@quiz
What is a wheel?
- A source tarball
* A built, ready-to-install package archive
- A virtual environment
- A lock file
? `.whl` files install without a build step.

@@flashcards
pyproject.toml :: standard project configuration and metadata file
Build backend :: tool that builds wheels and sdists (hatchling, setuptools...)
Wheel :: built package archive (.whl)
sdist :: source distribution (.tar.gz)
PEP 440 :: Python's version numbering standard
`~=1.4.2` :: compatible release: >=1.4.2, <1.5
Lock file :: exact versions and hashes of every dependency
uv :: fast Rust-based Python package and project manager
Entry point / `[project.scripts]` :: installs a command that runs a function
Typosquatting :: malicious packages with names similar to popular ones

@@exercise Check version specifiers
Implement simplified PEP 440 matching for versions made only of numbers (`1.4`, `1.4.2`, `2`):

- `parse(v)` → a tuple of ints, with trailing zeros removed so `1.4` and `1.4.0` are equal
- `matches(version, spec)`: `spec` is comma-separated clauses, all must hold. Operators: `>=`, `<=`, `>`, `<`, `==`, `!=` (and `==X.Y.*` meaning the version starts with X.Y), and `~=X.Y.Z` meaning `>=X.Y.Z` and the same `X.Y` prefix (for `~=X.Y`: `>=X.Y` and same `X`).
- `best(available, spec)`: the highest matching version string, or `None`.
@@starter
def parse(v):
    pass


def matches(version, spec):
    pass


def best(available, spec):
    pass


AVAILABLE = ["1.3.9", "1.4.0", "1.4.2", "1.4.10", "1.5.0", "2.0.0"]
print(best(AVAILABLE, ">=1.4,<2"), best(AVAILABLE, "~=1.4.2"), best(AVAILABLE, "==1.4.*,!=1.4.10"), best(AVAILABLE, ">3"))
@@expected
1.5.0 1.4.10 1.4.2 None
@@tests
check(parse("1.4.0"), (1, 4), "trailing zeros")
check(parse("2"), (2,), "single")
check(matches("1.4", "==1.4.0"), True, "equal with zeros")
check(matches("1.4.10", ">1.4.9"), True, "numeric compare")
check(matches("2.0", "~=1.4"), False, "~=1.4 excludes 2.0")
check(matches("1.9", "~=1.4"), True, "~=1.4 allows 1.9")
check(matches("1.5.0", "~=1.4.2"), False, "~=1.4.2 excludes 1.5")
check(matches("1.4", "!=1.4.0"), False, "not equal")
check(matches("1.40.1", "==1.4.*"), False, "prefix is by parts")
@@hint
`parse`: `parts = [int(p) for p in v.split(".")]`, then pop trailing zeros while more than one part. For `==X.Y.*` compare the first parts of the **unstripped** version list with the prefix. For `~=`: `prefix = parts_of_spec[:-1]`; require `version >= spec` and `version_parts[:len(prefix)] == prefix` (pad the version with zeros to enough length first). Check operators longest first (`>=` before `>`).
@@solution
def parse(v):
    parts = [int(p) for p in v.split(".")]
    while len(parts) > 1 and parts[-1] == 0:
        parts.pop()
    return tuple(parts)


def _padded(v, n):
    parts = [int(p) for p in v.split(".")]
    return parts + [0] * (n - len(parts))


def _clause(version, clause):
    clause = clause.strip()
    for op in ("~=", ">=", "<=", "==", "!=", ">", "<"):
        if clause.startswith(op):
            target = clause[len(op):].strip()
            break
    else:
        raise ValueError(f"bad clause: {clause}")
    if op == "==" and target.endswith(".*"):
        prefix = [int(p) for p in target[:-2].split(".")]
        return _padded(version, len(prefix))[:len(prefix)] == prefix
    v, t = parse(version), parse(target)
    if op == "~=":
        spec_parts = [int(p) for p in target.split(".")]
        prefix = spec_parts[:-1]
        return v >= t and _padded(version, len(prefix))[:len(prefix)] == prefix
    return {">=": v >= t, "<=": v <= t, "==": v == t, "!=": v != t, ">": v > t, "<": v < t}[op]


def matches(version, spec):
    return all(_clause(version, c) for c in spec.split(","))


def best(available, spec):
    ok = [v for v in available if matches(v, spec)]
    return max(ok, key=parse) if ok else None


AVAILABLE = ["1.3.9", "1.4.0", "1.4.2", "1.4.10", "1.5.0", "2.0.0"]
print(best(AVAILABLE, ">=1.4,<2"), best(AVAILABLE, "~=1.4.2"), best(AVAILABLE, "==1.4.*,!=1.4.10"), best(AVAILABLE, ">3"))

@@exercise Read project metadata
Use `tomllib` (standard library) to analyse a `pyproject.toml` given as text. Write `summary(text)` returning a dict:

- `name`, `version`
- `python`: the `requires-python` value (or `None`)
- `deps`: sorted list of dependency **names** (the part before any of `<>=!~[ ;`), lower-cased
- `scripts`: sorted list of command names from `[project.scripts]` (empty if none)
- `unpinned`: sorted names of dependencies that have no version specifier at all

Raise `ValueError("missing [project] table")` if there's no `[project]`.
@@starter
import tomllib
import re

TEXT = '''
[project]
name = "assetctl"
version = "1.4.0"
requires-python = ">=3.11"
dependencies = ["httpx>=0.27,<1", "Typer[all] >=0.12", "rich", "tomli; python_version < '3.11'"]

[project.scripts]
assetctl = "assetctl.cli:app"
assetd = "assetctl.daemon:main"
'''


def summary(text):
    pass


print(summary(TEXT))
@@tests
s = summary(TEXT)
check((s["name"], s["version"], s["python"]), ("assetctl", "1.4.0", ">=3.11"), "basics")
check(s["deps"], ["httpx", "rich", "tomli", "typer"], "dep names")
check(s["scripts"], ["assetctl", "assetd"], "scripts")
check(s["unpinned"], ["rich", "tomli"], "unpinned")
check(summary('[project]\nname = "x"\nversion = "0.1"')["deps"], [], "no deps")
try:
    summary('[tool.ruff]\nline-length = 100')
    check("ok", "ValueError", "missing project")
except ValueError:
    pass
@@hint
`data = tomllib.loads(text)`; `project = data.get("project")`. For each dependency: `name = re.split(r"[<>=!~\[ ;]", dep, maxsplit=1)[0].lower()`. A dependency is unpinned if the part before any `;` (environment marker) contains none of `<>=!~`.
@@solution
import tomllib
import re

TEXT = '''
[project]
name = "assetctl"
version = "1.4.0"
requires-python = ">=3.11"
dependencies = ["httpx>=0.27,<1", "Typer[all] >=0.12", "rich", "tomli; python_version < '3.11'"]

[project.scripts]
assetctl = "assetctl.cli:app"
assetd = "assetctl.daemon:main"
'''


def summary(text):
    data = tomllib.loads(text)
    project = data.get("project")
    if project is None:
        raise ValueError("missing [project] table")
    deps = project.get("dependencies", [])
    names, unpinned = [], []
    for dep in deps:
        name = re.split(r"[<>=!~\[ ;]", dep, maxsplit=1)[0].lower()
        names.append(name)
        requirement = dep.split(";")[0]
        if not any(op in requirement for op in "<>=!~"):
            unpinned.append(name)
    return {
        "name": project["name"],
        "version": project["version"],
        "python": project.get("requires-python"),
        "deps": sorted(names),
        "scripts": sorted(project.get("scripts", {})),
        "unpinned": sorted(unpinned),
    }


print(summary(TEXT))
