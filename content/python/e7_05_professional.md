@@lesson e7-professional Professional Python: venv, pip, Packages and Quality
@@concepts modules
@@text
The last step from "I can write scripts" to "I can work on a team's Python code": environments, dependencies, project layout and tooling.

## Virtual environments

Different projects need different package versions. A **virtual environment** (venv) is a private folder of packages per project:

```text
python -m venv .venv              # create it, once per project
.venv\Scripts\activate            # Windows (PowerShell: .venv\Scripts\Activate.ps1)
source .venv/bin/activate         # Linux / macOS
python -m pip install requests    # installs into .venv only
python -m pip freeze > requirements.txt
python -m pip install -r requirements.txt   # recreate it elsewhere
deactivate
```

Never `pip install` into the system Python for project work; that's how machines end up with conflicting versions. Add `.venv/` to `.gitignore`. (Newer tools like **uv** do all of this faster, with the same ideas.)

## Packages worth knowing in IT

| Package | For |
|---|---|
| `requests` / `httpx` | HTTP and REST APIs |
| `paramiko`, `netmiko` | SSH into servers and network devices (Cisco, Juniper) |
| `boto3` | AWS automation |
| `azure-identity`, `azure-mgmt-*` | Azure automation |
| `pandas`, `openpyxl` | spreadsheets, reports, data analysis |
| `pyyaml` | YAML configs (Ansible, Kubernetes) |
| `psutil` | CPU, memory, disk, processes |
| `rich`, `typer` / `click` | nice terminal output and CLIs |
| `pytest` | testing |
| `ldap3` | Active Directory / LDAP queries |

## Project layout

```text
disk-guard/
├── pyproject.toml        # name, version, dependencies, tool settings
├── README.md
├── src/disk_guard/
│   ├── __init__.py
│   ├── cli.py            # argparse / main()
│   └── checks.py         # the logic, no printing
└── tests/
    └── test_checks.py
```

Keep logic in functions that **return** values, keep printing and argument parsing in a thin `cli.py`. That's what makes the logic testable.

## Quality tools

- **ruff**: a very fast linter *and* formatter: finds unused imports, likely bugs, style issues. `ruff check .` and `ruff format .`
- **mypy** or **pyright**: checks your type hints.
- **pytest**: runs tests; `pytest --cov` shows which lines tests cover.
- **pre-commit**: runs all of these automatically before every git commit.
- **git**: everything lives in version control. Small commits with clear messages.

## Security habits

- Secrets (passwords, API tokens) come from environment variables or a vault, **never** from the code or git.
- Validate input; use parameterized queries for SQL (you'll see why in the SQL track); never build shell commands from strings.
- Keep dependencies updated: `pip list --outdated`, and tools like `pip-audit` find known vulnerabilities.

## Where to go next

You now know Python well enough for real IT work. Practice by building: the 🛠 **Projects** tab has bigger Python projects (log analyzer, network scanner, system health monitor, backup tool), and the cert tracks have projects that use Python too.

@@quiz
Why use a virtual environment per project?
- It makes Python run faster
* Each project gets its own package versions without conflicting with others
- It's required to run any Python script
- It encrypts your code
? Project A can use requests 2.28 and project B 2.32, side by side.

@@quiz
Where should a script's API token come from?
- A variable at the top of the script
- A comment in the code
* An environment variable or a secrets manager
- The git commit message
? Code ends up in git, in backups, in screenshots. Secrets must live outside it.

@@quiz
What is `requirements.txt` (or the dependencies in pyproject.toml) for?
* Recording the packages a project needs so anyone can install the same set
- Storing passwords
- Listing tests to run
- Configuring the editor
? pip install -r requirements.txt recreates the environment.

@@drill
Write the command that creates a virtual environment in a folder called `.venv`.
= python -m venv .venv
= python3 -m venv .venv
= py -m venv .venv
? Then activate it before installing packages.

@@flashcards
`python -m venv .venv` :: create a virtual environment
`pip freeze > requirements.txt` :: record exact package versions
`pip install -r requirements.txt` :: install recorded packages
`pyproject.toml` :: modern project config: metadata, dependencies, tools
ruff :: fast Python linter and formatter
paramiko / netmiko :: SSH automation for servers / network devices
boto3 :: the AWS SDK for Python

@@exercise Review a project's layout
Check a repository's file list against the lesson's project layout and quality habits. Write `review(files, gitignore)` that returns the problems:

- no `pyproject.toml` (or only a `setup.py`);
- no `README.md`, or no `tests/` folder with at least one `test_*.py`;
- code not in a package folder (a `.py` file at the top level other than a small `main.py`);
- committed files that should be ignored: anything in `.venv/` or `__pycache__/`, `.env` files (secrets!), `*.pyc`;
- a `.gitignore` that doesn't list `.venv` and `__pycache__`.

Print each project with `looks professional` or its problems.
@@lang python
@@starter
import fnmatch

projects = {
    "netcheck": ([
        "pyproject.toml", "README.md", ".gitignore", "src/netcheck/__init__.py", "src/netcheck/ping.py",
        "tests/test_ping.py",
    ], [".venv/", "__pycache__/", ".env"]),
    "old-scripts": ([
        "setup.py", "backup.py", "report.py", "helpers.py", ".env", "__pycache__/helpers.cpython-313.pyc",
        ".venv/bin/python",
    ], ["*.log"]),
    "inventory": ([
        "pyproject.toml", "README.md", "main.py", "inventory/__init__.py", "inventory/load.py", "tests/conftest.py",
    ], [".venv", "__pycache__"]),
}

def review(files, gitignore):
    raise NotImplementedError


for name, (files, gitignore) in projects.items():
    problems = review(files, gitignore)
    print(name + ":")
    for p in problems or ["looks professional"]:
        print("  " + p)
@@expected
netcheck:
  looks professional
old-scripts:
  no pyproject.toml (setup.py only)
  no README.md
  no tests/test_*.py
  code outside a package: backup.py, report.py, helpers.py
  committed by mistake: .env
  committed by mistake: __pycache__/helpers.cpython-313.pyc
  committed by mistake: .venv/bin/python
  .gitignore misses .venv
  .gitignore misses __pycache__
inventory:
  no tests/test_*.py
@@requires
def review
fnmatch.fnmatch(
@@hint
`fnmatch.fnmatch(path, "tests/test_*.py")` does shell-style matching. Strip a trailing `/` from the .gitignore lines before comparing.
@@solution
import fnmatch

projects = {
    "netcheck": ([
        "pyproject.toml", "README.md", ".gitignore", "src/netcheck/__init__.py", "src/netcheck/ping.py",
        "tests/test_ping.py",
    ], [".venv/", "__pycache__/", ".env"]),
    "old-scripts": ([
        "setup.py", "backup.py", "report.py", "helpers.py", ".env", "__pycache__/helpers.cpython-313.pyc",
        ".venv/bin/python",
    ], ["*.log"]),
    "inventory": ([
        "pyproject.toml", "README.md", "main.py", "inventory/__init__.py", "inventory/load.py", "tests/conftest.py",
    ], [".venv", "__pycache__"]),
}

def review(files, gitignore):
    problems = []
    if "pyproject.toml" not in files:
        problems.append("no pyproject.toml" + (" (setup.py only)" if "setup.py" in files else ""))
    if "README.md" not in files:
        problems.append("no README.md")
    if not any(fnmatch.fnmatch(f, "tests/test_*.py") for f in files):
        problems.append("no tests/test_*.py")
    loose = [f for f in files if "/" not in f and f.endswith(".py") and f not in ("main.py", "setup.py")]
    if loose:
        problems.append("code outside a package: " + ", ".join(loose))
    for f in files:
        if f.startswith((".venv/", "__pycache__/")) or "/__pycache__/" in f or f.endswith(".pyc") or f.split("/")[-1] == ".env":
            problems.append(f"committed by mistake: {f}")
    ignored = {g.rstrip("/") for g in gitignore}
    for needed in (".venv", "__pycache__"):
        if needed not in ignored:
            problems.append(f".gitignore misses {needed}")
    return problems

for name, (files, gitignore) in projects.items():
    problems = review(files, gitignore)
    print(name + ":")
    for p in problems or ["looks professional"]:
        print("  " + p)
