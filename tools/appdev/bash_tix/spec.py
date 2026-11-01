OUT = "content/bash/m3_01_tix.md"
BASE = "tools/appdev/bash_tix"
FILES = ["tix", "tixlib.sh", "tests.sh", "tix-completion.bash", "Makefile"]

PARTS = [
    ("text", r'''
@@lesson m3-tix tix: a Help-Desk Ticket Tracker for the Command Line
@@text
Git, Docker, kubectl and the AWS CLI all work the same way: one program, several **subcommands**, short options, helpful errors, clean exit codes, and tab completion. You'll build one of your own: `tix`, a ticket tracker for a small IT team or a home lab, written in Bash and installed with `make install`.

```text
$ tix add -p urgent -a yuki VPN down for Osaka office
Created ticket #2
$ tix list
#2    urgent  yuki       VPN down for Osaka office
#4    high    ken        Email bounces
#1    normal  -          Printer on 3F is offline
$ tix close 2
Closed ticket #2
$ tix close 2
tix: ticket #2 is already closed
$ echo $?
1
$ tix report
open: 2  closed: 1
open by priority: high=1 normal=1
open by assignee:
  -          1
  ken        1
```

## How it's built

| File | Job |
|---|---|
| `tix` | the program: `set -euo pipefail`, find and `source` the library, call `main "$@"` |
| `tixlib.sh` | every command as a function (`cmd_add`, `cmd_list`, ...) and `main`, which dispatches |
| `tests.sh` | 36 tests that run the real `tix` against a temporary data file |
| `tix-completion.bash` | Tab completion for commands, priorities and ticket numbers |
| `Makefile` | `make test`, `make lint`, `make install` and `make uninstall` |

Tickets are one **tab-separated** line each, in `~/.local/share/tix/tickets.tsv` (the XDG data folder), or in `$TIX_FILE` when it's set, which is how the tests use a temporary file:

```text
1	open	normal	-	2026-11-01	Printer on 3F is offline
2	closed	urgent	yuki	2026-11-01	VPN down for Osaka office
```

## Your tasks

Four functions in `tixlib.sh` print `TODO`. The comment above each one says exactly what to do:

1. `next_id`: one line of `awk` that finds the highest id.
2. `update_field`: `awk` with `-v` variables to change one column of one line, writing to a temp file and `mv`-ing it over the original.
3. `cmd_add`: `getopts ":p:a:"`, validation, and appending a line with `printf '%s\t%s...'`.
4. `cmd_list`: filter with `awk`, sort by priority rank and then id with `sort -t$'\t' -k1,1n -k2,2n`, then print the columns with `printf '%-5s %-7s %-10s %s\n'`.

`cmd_close`, `cmd_show` and the others are written for you. Read them first: they show the style.

```text
bash tests.sh            # or: make test
```

Then install it for yourself with `make install` (make sure `~/.local/bin` is on your `PATH`), and `source tix-completion.bash` to try Tab completion.
'''),
    ("project", "tix", "tix",
     "Write the four functions that print `TODO` in `tixlib.sh` until `bash tests.sh` passes, then install `tix` with `make install`.",
     BASE + "/starter", FILES),
    ("text", r'''
@@text
## Stretch goals

- **`tix edit ID`**: open the title in `$EDITOR` (write it to a temp file, run the editor, read it back).
- **Comments**: `tix note ID text` appending to a second file, `notes.tsv`, shown by `tix show`.
- **Locking**: two people running `tix add` at the same moment can both get the same id. Wrap changes in `flock` (`exec 9>"$TIX_FILE.lock"; flock 9`).
- **Colour**, only when the output is a terminal: `[[ -t 1 ]]`, and respect the `NO_COLOR` environment variable.
- **Due dates and `tix overdue`**: `date -d "$created + 3 days" +%F` and string comparison of ISO dates.
- **Package it**: build a `.deb` with `dpkg-deb --build` (a `DEBIAN/control` file plus the `make install` layout under `usr/`).
- **ShellCheck it**: `make lint`, and fix every warning.
'''),
    ("project", "tix_finished", "tix: the finished version",
     "A complete solution to compare with yours.",
     BASE + "/finished", FILES),
    ("text", r'''
@@text
🎉 You've built a real command-line application: subcommands, option parsing, safe file updates, exit codes, tests, completion and an installer. It's the same structure as tools used by millions of people.
'''),
]
