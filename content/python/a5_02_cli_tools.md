@@lesson a5-cli Building Command-Line Tools: argparse and logging
@@concepts automation
@@text
The scripts IT teams share are **command-line tools**: run them with options, they print or log what they do, and they return an exit code that other scripts and schedulers can check.

```text
python disk_check.py --path C:\ --warn 80 --verbose
```

## argparse: options and help for free

```python
import argparse

parser = argparse.ArgumentParser(description="Check free disk space.")
parser.add_argument("--path", default=".", help="folder or drive to check")
parser.add_argument("--warn", type=int, default=80, help="warn at this percent used")
parser.add_argument("-v", "--verbose", action="store_true", help="print more details")

# Normally: args = parser.parse_args()  (reads the real command line).
# Here we pass a list to show what it does:
args = parser.parse_args(["--warn", "90", "-v"])
print(args.path, args.warn, args.verbose)
```

`argparse` also generates `--help` output and friendly errors ("argument --warn: invalid int value") automatically.

## logging: better than print

```python
import logging, sys

logging.basicConfig(
    level=logging.INFO,
    format="%(asctime)s %(levelname)-7s %(message)s",
    datefmt="%H:%M:%S",
    stream=sys.stdout,
)
log = logging.getLogger("disk_check")

log.debug("only shown at DEBUG level")
log.info("checking C:")
log.warning("C: is 91% full")
log.error("D: not found")
```

- Levels, from least to most severe: `DEBUG`, `INFO`, `WARNING`, `ERROR`, `CRITICAL`. You choose the minimum level to show, e.g. DEBUG with `--verbose`.
- Every line gets a timestamp and level, in the same format, so logs can be searched and parsed.
- `basicConfig(filename="tool.log")` writes to a file instead, which is what scheduled tasks need.

## Exit codes and main()

A well-behaved tool puts its work in a `main()` function and returns an exit code:

```python,norun
import sys

def main(argv=None):
    args = build_parser().parse_args(argv)
    ...
    return 0 if all_ok else 1

if __name__ == "__main__":
    sys.exit(main())
```

`if __name__ == "__main__":` means "only when this file is run directly, not when it's imported". That lets tests import `main` and call it with fake arguments.

@@quiz
Your script is run every night by Task Scheduler / cron. How does the scheduler know it failed?
* The script exits with a non-zero exit code
- It prints "failed"
- It deletes its log file
- It can't know
? Schedulers, CI pipelines and other scripts all check the exit code.

@@quiz
What does argparse give you for free when you define your options?
* A `--help` message and error messages for wrong arguments
- A graphical window
- Automatic logging to a file
- A configuration file
? Users can run `script.py --help` to see every option.

@@quiz
Why use `logging` instead of `print` in a tool?
* Messages get levels and timestamps, and you can send them to files or silence them without changing the code
- print doesn't work in scripts
- logging is faster
- logging encrypts the output
? logging.info, warning and error can be filtered by level.

@@drill
Which standard-library module parses command-line options and builds `--help`?
= argparse
? argparse.ArgumentParser().

@@flashcards
argparse :: command-line options with --help for free
add_argument("--dry-run", action="store_true") :: an on/off flag
logging.basicConfig(level=...) :: set up log format and level
logger levels :: DEBUG, INFO, WARNING, ERROR, CRITICAL
sys.exit(1) :: end with a failure exit code
if __name__ == "__main__": :: run main only when executed directly

@@exercise A tiny CLI
Write `main(argv)` that parses these options from the list `argv`:

- `host` (a required **positional** argument),
- `--port` (int, default `22`),
- `--timeout` (float, default `2.0`).

It returns the string `"host:port timeout=Xs"`, e.g. `main(["db01", "--port", "5432"])` returns `"db01:5432 timeout=2.0s"`.
@@starter
import argparse


def main(argv):
    pass


print(main(["db01", "--port", "5432"]))
@@tests
check(main(["db01", "--port", "5432"]), "db01:5432 timeout=2.0s", "main(['db01', '--port', '5432'])")
check(main(["web01"]), "web01:22 timeout=2.0s", "defaults")
check(main(["x", "--timeout", "0.5"]), "x:22 timeout=0.5s", "--timeout 0.5")
@@hint
A positional argument has no dashes: `parser.add_argument("host")`. Then `args = parser.parse_args(argv)` and an f-string.
@@solution
import argparse


def main(argv):
    parser = argparse.ArgumentParser(description="Show a connection target.")
    parser.add_argument("host")
    parser.add_argument("--port", type=int, default=22)
    parser.add_argument("--timeout", type=float, default=2.0)
    args = parser.parse_args(argv)
    return f"{args.host}:{args.port} timeout={args.timeout}s"


print(main(["db01", "--port", "5432"]))

@@exercise Log level filter
Write `filter_log(lines, min_level)` that keeps only the log lines (format: `HH:MM:SS LEVEL message`) whose level is at least `min_level`, in the order DEBUG < INFO < WARNING < ERROR < CRITICAL.
@@starter
LEVELS = ["DEBUG", "INFO", "WARNING", "ERROR", "CRITICAL"]


def filter_log(lines, min_level):
    pass


log = ["08:00:01 INFO start", "08:00:02 DEBUG x=1", "08:00:03 ERROR disk", "08:00:04 WARNING slow"]
print(filter_log(log, "WARNING"))
@@tests
log = ["08:00:01 INFO start", "08:00:02 DEBUG x=1", "08:00:03 ERROR disk", "08:00:04 WARNING slow"]
check(filter_log(log, "WARNING"), ["08:00:03 ERROR disk", "08:00:04 WARNING slow"], "filter_log(log, 'WARNING')")
check(filter_log(log, "DEBUG"), log, "filter_log(log, 'DEBUG') keeps everything")
check(filter_log(log, "CRITICAL"), [], "nothing critical")
@@hint
`LEVELS.index(name)` turns a level into a number you can compare. The level is `line.split()[1]`.
@@solution
LEVELS = ["DEBUG", "INFO", "WARNING", "ERROR", "CRITICAL"]


def filter_log(lines, min_level):
    limit = LEVELS.index(min_level)
    return [line for line in lines if LEVELS.index(line.split()[1]) >= limit]


log = ["08:00:01 INFO start", "08:00:02 DEBUG x=1", "08:00:03 ERROR disk", "08:00:04 WARNING slow"]
print(filter_log(log, "WARNING"))
