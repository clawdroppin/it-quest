@@lesson a5-system Automating the System: os, shutil, subprocess
@@concepts processes, automation
@@text
This is where Python becomes a sysadmin's power tool: running commands, reading the environment, copying and archiving files.

## Environment and platform

```python
import os, sys, platform

print(platform.system())                 # Windows, Linux or Darwin (macOS)
print(sys.version_info.major)
print(os.environ.get("USERNAME") or os.environ.get("USER"))
print(os.cpu_count() > 0)
print(os.getcwd() != "")
```

`os.environ` is a dict of **environment variables**: `PATH`, `USERNAME`, `TEMP`… Scripts often read settings and secrets from them instead of hard-coding them (never put passwords in your code).

## Copying, moving, archiving: shutil

```python
import shutil
from pathlib import Path

Path("src").mkdir(exist_ok=True)
Path("src/a.txt").write_text("hello")
shutil.copy("src/a.txt", "src/b.txt")          # copy a file
shutil.copytree("src", "backup", dirs_exist_ok=True)
archive = shutil.make_archive("backup_2026", "zip", "backup")
print(Path(archive).name)
print(shutil.disk_usage(".").total > 0)        # total/used/free bytes
shutil.rmtree("backup")                        # delete a folder tree (careful!)
```

> `shutil.rmtree` and `Path.unlink` delete **permanently**: no Recycle Bin. Production scripts first print what they *would* delete (a "dry run") and only delete when asked.

## Running commands: subprocess

`subprocess.run` starts another program, waits for it, and gives you its output:

```python
import subprocess, sys

result = subprocess.run(
    [sys.executable, "--version"],     # the program and its arguments, as a list
    capture_output=True,
    text=True,
    timeout=10,
)
print(result.returncode)               # 0 means success
print(result.stdout.strip() or result.stderr.strip())
```

- Pass the command as a **list** of program and arguments. This avoids the shell, which is safer: user input can't sneak in extra commands (**command injection**, a classic security flaw).
- `capture_output=True, text=True` gives you `stdout` and `stderr` as strings.
- `returncode` is the **exit code**: `0` = success, anything else = failure. This convention is universal: Bash, PowerShell, Windows, Linux.
- `timeout=` stops commands that hang. `check=True` raises `CalledProcessError` on a non-zero exit code.

A typical real use: run a tool, then parse its output.

```python
import subprocess, sys

code = "import os; print(os.cpu_count()); print('ok')"
out = subprocess.run([sys.executable, "-c", code], capture_output=True, text=True).stdout
lines = out.splitlines()
print(f"cores={lines[0]} status={lines[1]}")
```

On Windows you'd run things like `["ipconfig", "/all"]` or `["powershell", "-Command", "Get-Service"]`; on Linux `["df", "-h"]` or `["systemctl", "is-active", "nginx"]`.

@@quiz
Why pass `["ping", "-n", "1", host]` instead of `f"ping -n 1 {host}"` with `shell=True`?
* With a list there's no shell, so a malicious `host` like `x & del *` can't run extra commands
- Lists are faster to type
- `shell=True` doesn't exist
- Strings can't contain spaces
? Building shell command strings from input is **command injection** waiting to happen.

@@quiz
A command finished with exit code `1`. What does that usually mean?
- Success, one result
* It failed
- It's still running
- It printed one line
? By convention, 0 is success and anything else is a failure code.

@@quiz
Which call copies a whole folder tree?
* `shutil.copytree(src, dst)`
- `shutil.copy(src, dst)`
- `os.rename(src, dst)`
- `pathlib.Path(src).copy(dst)`
? shutil.copy copies one file; copytree copies a directory and everything in it.

@@drill
Which module function runs an external command and waits for it to finish?
= subprocess.run
= subprocess.run()
= run
? Pass a list of arguments and check=True to raise on failure.

@@flashcards
`subprocess.run([...], capture_output=True, text=True)` :: run a command and get its output
exit code 0 :: success (any other number: failure)
`os.environ.get("NAME")` :: read an environment variable
`shutil.copytree(a, b)` :: copy a whole folder
`shutil.make_archive(name, "zip", folder)` :: zip a folder
command injection :: attacker input becomes part of a shell command

@@exercise Run and report
Write `run_check(args)` that runs the command `args` (a list), and returns `"OK"` if its exit code is 0, otherwise `"FAILED (exit N)"` with the code. If the program doesn't exist (`FileNotFoundError`), return `"MISSING"`.
@@starter
import subprocess, sys


def run_check(args):
    pass


print(run_check([sys.executable, "-c", "print('hi')"]))
@@tests
import sys as _s
check(run_check([_s.executable, "-c", "pass"]), "OK", "a command that succeeds")
check(run_check([_s.executable, "-c", "import sys; sys.exit(3)"]), "FAILED (exit 3)", "a command that exits with 3")
check(run_check(["this-program-does-not-exist-123"]), "MISSING", "a missing program")
@@hint
`try: r = subprocess.run(args, capture_output=True, text=True, timeout=30)` / `except FileNotFoundError: return "MISSING"`. Then check `r.returncode`.
@@solution
import subprocess, sys


def run_check(args):
    try:
        r = subprocess.run(args, capture_output=True, text=True, timeout=30)
    except FileNotFoundError:
        return "MISSING"
    if r.returncode == 0:
        return "OK"
    return f"FAILED (exit {r.returncode})"


print(run_check([sys.executable, "-c", "print('hi')"]))

@@exercise Clean old files (dry run first)
Write `old_files(folder, days, now)` that returns a sorted list of names of files in `folder` whose modification time (`p.stat().st_mtime`, seconds since 1970) is **more than** `days` days before `now` (also in seconds). Don't delete anything: this is the dry run.
@@starter
import os, time
from pathlib import Path

d = Path("tmpfiles")
d.mkdir(exist_ok=True)
now = time.time()
for name, age_days in [("old.log", 40), ("recent.log", 2), ("ancient.tmp", 400)]:
    p = d / name
    p.write_text("x")
    t = now - age_days * 86400
    os.utime(p, (t, t))     # pretend the file is older


def old_files(folder, days, now):
    pass


print(old_files("tmpfiles", 30, now))
@@tests
check(old_files("tmpfiles", 30, now), ["ancient.tmp", "old.log"], "old_files(..., 30 days)")
check(old_files("tmpfiles", 365, now), ["ancient.tmp"], "old_files(..., 365 days)")
check(old_files("tmpfiles", 1000, now), [], "nothing older than 1000 days")
@@hint
The age in seconds is `now - p.stat().st_mtime`; compare it with `days * 86400`.
@@solution
import os, time
from pathlib import Path

d = Path("tmpfiles")
d.mkdir(exist_ok=True)
now = time.time()
for name, age_days in [("old.log", 40), ("recent.log", 2), ("ancient.tmp", 400)]:
    p = d / name
    p.write_text("x")
    t = now - age_days * 86400
    os.utime(p, (t, t))     # pretend the file is older


def old_files(folder, days, now):
    limit = days * 86400
    return sorted(p.name for p in Path(folder).iterdir() if p.is_file() and now - p.stat().st_mtime > limit)


print(old_files("tmpfiles", 30, now))
