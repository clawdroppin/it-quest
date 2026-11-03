@@lesson r11-config Configuration and Secrets Done Right
@@concepts files, automation
@@text
Hard-coded server names, passwords in source code and "change line 12 before running" are how scripts break and secrets leak. This lesson covers where configuration comes from, how to layer it, how to validate it, and how to handle secrets safely.

## The twelve-factor rule

*Store config in the environment* (from the **Twelve-Factor App** guidelines): anything that differs between your laptop, test and production (URLs, credentials, feature switches) must not be in the code. The same build runs everywhere, configured from outside.

## Sources, in order of priority

A typical precedence, highest first:

1. **Command-line flags**: `--port 8080` (explicit, one-off)
2. **Environment variables**: `ASSETCTL_PORT=8080` (containers, CI, services)
3. **A config file**: `assetctl.toml` in the project, `~/.config/assetctl/config.toml` per user, `/etc/assetctl/config.toml` per machine
4. **Defaults in code**

```python
import os, tomllib

DEFAULTS = {"port": 8080, "debug": False, "api_url": "https://inventory.corp.example"}

def load(file_text="", env=os.environ, flags=None):
    cfg = dict(DEFAULTS)
    cfg.update(tomllib.loads(file_text))                              # file
    for key in cfg:                                                   # env: ASSETCTL_<KEY>
        if (value := env.get(f"ASSETCTL_{key.upper()}")) is not None:
            cfg[key] = value
    cfg.update({k: v for k, v in (flags or {}).items() if v is not None})
    return cfg

print(load('port = 9000', env={"ASSETCTL_DEBUG": "true"}, flags={"port": None}))
```

Notice the problem in that output: environment variables are always **strings** (`"true"`), so config must be **converted and validated** (the second exercise).

## File formats

| Format | Module | Notes |
|---|---|---|
| TOML | `tomllib` (read, 3.11+), `tomli-w` (write) | Python's standard for config (`pyproject.toml`); typed values, comments |
| JSON | `json` | no comments; good for machine-written config |
| YAML | `PyYAML` / `ruamel.yaml` | popular in DevOps (Ansible, Kubernetes); always use `yaml.safe_load` |
| INI | `configparser` | old Windows-style, all values strings |
| `.env` | `python-dotenv` | `KEY=value` lines loaded into the environment for local development |

Find per-user folders portably with `platformdirs` (`user_config_dir("assetctl")`): `%APPDATA%` on Windows, `~/.config` on Linux, `~/Library/Application Support` on macOS.

## Validate at start-up

Read and validate **all** configuration when the program starts and fail immediately with a clear message ("ASSETCTL_PORT must be a number 1–65535, got 'eighty'"), rather than crashing an hour later. **pydantic-settings** does this from a typed class:

```python,norun
from pydantic_settings import BaseSettings, SettingsConfigDict

class Settings(BaseSettings):
    model_config = SettingsConfigDict(env_prefix="ASSETCTL_", env_file=".env")
    port: int = 8080
    debug: bool = False
    api_url: str
    api_token: str            # required: start-up fails if missing

settings = Settings()
```

## Secrets

A **secret** is anything that grants access: passwords, API tokens, private keys, connection strings with passwords.

- **Never commit secrets to Git**, not even in a "private" repo; history keeps them forever. Add `.env` to `.gitignore`. Scan with `gitleaks` or GitHub secret scanning. If one leaks: **rotate it** (revoke and replace), don't just delete the commit.
- Pass secrets via environment variables or files mounted by the platform (Docker/Kubernetes secrets), or fetch them at runtime from a **secrets manager**: Azure Key Vault, AWS Secrets Manager, HashiCorp Vault, 1Password CLI.
- On a workstation, use the OS credential store with `keyring` (Windows Credential Manager, macOS Keychain, Secret Service on Linux): `keyring.get_password("assetctl", "api")`.
- Ask interactively with `getpass.getpass()` (input isn't echoed).
- Don't log them, don't put them in URLs (they end up in proxy logs), and don't show them in `repr()`: wrap them in a class whose repr is masked (like Pydantic's `SecretStr`).
- Prefer **short-lived credentials** (managed identities, OIDC tokens in CI) over long-lived keys.

## Feature flags

Config can also switch behaviour on and off (`new_sync_engine = true`) so features ship dark and are enabled gradually. Keep the list small and remove flags once a feature is fully launched.

@@quiz
Which source should usually win when the same setting is given in several places?
* A command-line flag
- The default in code
- The config file
- The first one read
? Flags > environment > file > defaults.

@@quiz
A token was committed to Git and the commit was deleted. What must you still do?
- Nothing, it's gone
* Rotate the token: revoke it and issue a new one
- Make the repository private
- Rename the file
? Copies of history, forks and caches may still contain it.

@@quiz
Why validate configuration at start-up?
- It's faster
* Bad config fails immediately with a clear message instead of causing a crash much later
- Python requires it
- To encrypt it
? Fail fast.

@@quiz
What type are environment variable values?
- Whatever was set
* Always strings
- int or str
- bytes
? Convert and validate them.

@@flashcards
Twelve-factor config :: settings that vary between environments live outside the code, usually in env vars
Precedence :: flags > environment > config file > defaults
tomllib :: standard-library TOML parser (read-only)
`yaml.safe_load` :: the only safe way to load untrusted YAML
pydantic-settings :: typed settings loaded and validated from env vars and .env files
Secrets manager :: service storing secrets (Key Vault, Vault, Secrets Manager)
keyring :: access the OS credential store from Python
Rotate :: revoke a secret and replace it with a new one
SecretStr :: a wrapper that hides a secret in repr and logs

@@exercise Layered, typed settings
Write `load_settings(file_text, env, flags)`:

- defaults: `{"port": 8080, "debug": False, "workers": 4, "api_url": None}`
- then the TOML file (unknown keys → `ValueError("unknown setting: <key>")`)
- then environment variables `APP_<KEY>` (e.g. `APP_PORT`), converted to the type of the default (`api_url` is a string): ints with `int()`, bools from `"1"/"true"/"yes"/"on"` → True and `"0"/"false"/"no"/"off"` → False (case-insensitive), anything else → `ValueError("APP_DEBUG: not a boolean")`; an int that doesn't parse → `ValueError("APP_PORT: not an integer")`
- then flags (a dict), ignoring `None` values
- finally validate: `port` 1–65535 and `workers` ≥ 1 (`ValueError("port out of range")` / `"workers must be >= 1"`), and `api_url` is required (`ValueError("api_url is required")`)
@@starter
import tomllib


def load_settings(file_text, env, flags):
    pass


print(load_settings('workers = 8\napi_url = "https://x"', {"APP_DEBUG": "Yes", "APP_PORT": "9000"}, {"port": None, "workers": 2}))
@@expected
{'port': 9000, 'debug': True, 'workers': 2, 'api_url': 'https://x'}
@@tests
check(load_settings('api_url = "u"', {}, {}), {"port": 8080, "debug": False, "workers": 4, "api_url": "u"}, "defaults")
check(load_settings("", {"APP_API_URL": "env-url", "APP_DEBUG": "off"}, {})["api_url"], "env-url", "string env")
cases = [
    ('color = "red"', {}, {}, "unknown setting: color"),
    ('api_url = "u"', {"APP_DEBUG": "maybe"}, {}, "APP_DEBUG: not a boolean"),
    ('api_url = "u"', {"APP_PORT": "eighty"}, {}, "APP_PORT: not an integer"),
    ('api_url = "u"', {}, {"port": 70000}, "port out of range"),
    ('api_url = "u"', {"APP_WORKERS": "0"}, {}, "workers must be >= 1"),
    ("", {}, {}, "api_url is required"),
]
for text, env, flags, msg in cases:
    try:
        load_settings(text, env, flags)
        check("loaded", msg, "should fail")
    except ValueError as e:
        check(str(e), msg, msg)
@@hint
Keep `TYPES = {"port": int, "debug": bool, "workers": int, "api_url": str}` since `api_url`'s default is None. Convert env values with a helper that branches on the type. Check `bool` before `int` isn't an issue here because you look up the declared type, not the value's type.
@@solution
import tomllib

DEFAULTS = {"port": 8080, "debug": False, "workers": 4, "api_url": None}
TYPES = {"port": int, "debug": bool, "workers": int, "api_url": str}
TRUE = {"1", "true", "yes", "on"}
FALSE = {"0", "false", "no", "off"}


def convert(name, kind, text):
    if kind is bool:
        t = text.strip().lower()
        if t in TRUE:
            return True
        if t in FALSE:
            return False
        raise ValueError(f"{name}: not a boolean")
    if kind is int:
        try:
            return int(text)
        except ValueError:
            raise ValueError(f"{name}: not an integer") from None
    return text


def load_settings(file_text, env, flags):
    cfg = dict(DEFAULTS)
    for key, value in tomllib.loads(file_text).items():
        if key not in DEFAULTS:
            raise ValueError(f"unknown setting: {key}")
        cfg[key] = value
    for key, kind in TYPES.items():
        name = f"APP_{key.upper()}"
        if name in env:
            cfg[key] = convert(name, kind, env[name])
    cfg.update({k: v for k, v in flags.items() if v is not None})
    if not 1 <= cfg["port"] <= 65535:
        raise ValueError("port out of range")
    if cfg["workers"] < 1:
        raise ValueError("workers must be >= 1")
    if not cfg["api_url"]:
        raise ValueError("api_url is required")
    return cfg


print(load_settings('workers = 8\napi_url = "https://x"', {"APP_DEBUG": "Yes", "APP_PORT": "9000"}, {"port": None, "workers": 2}))

@@exercise Keep secrets out of logs
Write:

- `class Secret`: wraps a string; `reveal()` returns it; `str()` and `repr()` return `"Secret('********')"` (always 8 stars, so the length doesn't leak); two Secrets are equal if their values are equal (compare with `hmac.compare_digest`, which takes the same time whether or not they match).
- `redact(text)`: replaces secret-looking values in a log line with `***`: anything after `password=`, `token=`, `secret=` or `apikey=` (case-insensitive key) up to the next space, `&` or end; and `Authorization: Bearer <value>` → `Authorization: Bearer ***`.
@@starter
import hmac
import re


class Secret:
    pass


def redact(text):
    pass


key = Secret("s3cr3t-token")
print(key, f"{key!r}", key.reveal())
print(redact("GET /sync?user=maya&token=abc123&page=2 Authorization: Bearer eyJhbGci"))
@@expected
Secret('********') Secret('********') s3cr3t-token
GET /sync?user=maya&token=***&page=2 Authorization: Bearer ***
@@tests
check(Secret("a") == Secret("a"), True, "equal")
check(Secret("a") == Secret("b"), False, "not equal")
check(str(Secret("x" * 40)), "Secret('********')", "length hidden")
check(redact("login PASSWORD=hunter2 ok"), "login PASSWORD=*** ok", "case-insensitive")
check(redact("apikey=k1&secret=s2"), "apikey=***&secret=***", "several")
check(redact("nothing here"), "nothing here", "unchanged")
@@hint
`__eq__`: `isinstance(other, Secret) and hmac.compare_digest(self._value.encode(), other._value.encode())`. Redact with `re.sub(r"(?i)\b(password|token|secret|apikey)=[^\s&]*", r"\1=***", text)` then `re.sub(r"(Authorization: Bearer )\S+", r"\1***", text)`.
@@solution
import hmac
import re


class Secret:
    def __init__(self, value):
        self._value = value

    def reveal(self):
        return self._value

    def __repr__(self):
        return "Secret('********')"

    __str__ = __repr__

    def __eq__(self, other):
        if not isinstance(other, Secret):
            return NotImplemented
        return hmac.compare_digest(self._value.encode(), other._value.encode())

    def __hash__(self):
        return hash(self._value)


def redact(text):
    text = re.sub(r"(?i)\b(password|token|secret|apikey)=[^\s&]*", r"\1=***", text)
    return re.sub(r"(Authorization: Bearer )\S+", r"\1***", text)


key = Secret("s3cr3t-token")
print(key, f"{key!r}", key.reveal())
print(redact("GET /sync?user=maya&token=abc123&page=2 Authorization: Bearer eyJhbGci"))
