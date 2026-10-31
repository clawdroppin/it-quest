@@lesson b3-backup-rotator Backup Rotator
@@text
"Do we have backups?" is the first question after every disaster: a dead disk, a deleted folder, ransomware. A backup you can't **verify** is a hope, not a backup. And a backup job that never deletes anything eventually fills the disk and stops working, quietly.

In this build you'll write **rotator**, a small tool you could schedule with Task Scheduler or cron:

```text
python main.py backup C:\Users\me\Documents D:\Backups --keep-last 3 --keep-daily 7
python main.py verify D:\Backups
python main.py backup ~/projects /mnt/backup --dry-run
```

It will:

1. **zip** the source folder into `D:\Backups\Documents-20261031-081502.zip` (the name holds the time),
2. write a **SHA-256 checksum** next to it (`...zip.sha256`), the same format `sha256sum` uses on Linux,
3. apply a **retention policy**: keep the newest few backups, plus the newest backup of each of the last few days, and delete the rest,
4. and with `--dry-run`, only *print* what it would do. Always offer this in tools that delete things.

## Retention, by example

With `keep_last=2, keep_daily=3` and these backups (newest first):

| Backup | Kept? | Why |
|---|---|---|
| Oct 31 18:00 | ✅ | one of the 2 newest |
| Oct 31 09:00 | ✅ | one of the 2 newest |
| Oct 31 08:00 | ❌ | Oct 31 already has a newer one |
| Oct 30 22:00 | ✅ | newest of Oct 30 (day 2 of 3) |
| Oct 30 07:00 | ❌ | |
| Oct 28 12:00 | ✅ | newest of Oct 28 (day 3 of 3) |
| Oct 27 12:00 | ❌ | only 3 days are kept |

Notice that the policy works on **names only**. That makes it easy to test, and safe: the function that decides is separate from the function that deletes.

@@project rotator Backup Rotator
Write the functions in `rotator.py` until `test_rotator.py` passes, then try a **dry run** on a real folder.
@@file rotator.py
"""Timestamped zip backups with checksums and a retention policy."""

import hashlib
import zipfile
from datetime import datetime
from pathlib import Path

STAMP = "%Y%m%d-%H%M%S"


def backup_name(source_name, when):
    """"Documents", datetime(2026, 10, 31, 8, 15, 2) -> "Documents-20261031-081502.zip"."""
    raise NotImplementedError("TODO: backup_name")


def parse_time(name):
    """The datetime in a backup file name, or None if the name isn't one of ours.
    "Documents-20261031-081502.zip" -> datetime(2026, 10, 31, 8, 15, 2).
    Source names may contain dashes themselves: "my-docs-20261031-081502.zip" works too."""
    raise NotImplementedError("TODO: parse_time")


def to_delete(names, keep_last, keep_daily):
    """Which backup names the retention policy deletes, oldest first.

    Keep the `keep_last` newest backups, and the newest backup of each of the
    `keep_daily` most recent days that have backups. Names that aren't backups are ignored."""
    raise NotImplementedError("TODO: to_delete")


def sha256_of(path):
    """The SHA-256 hex digest of a file, read in chunks (backups can be huge)."""
    raise NotImplementedError("TODO: sha256_of")


def make_backup(source, dest_dir, when):
    """Zip every file under `source` into dest_dir/backup_name(...), with paths
    relative to `source`. Also write "<zip>.sha256" containing "<digest>  <zip file name>\\n".
    Returns the zip's Path."""
    raise NotImplementedError("TODO: make_backup")


def verify(zip_path):
    """True if the zip's .sha256 file exists and matches, and the zip itself is intact
    (zipfile's testzip() returns None)."""
    raise NotImplementedError("TODO: verify")
@@file test_rotator.py
import tempfile
import unittest
from datetime import datetime
from pathlib import Path

from rotator import backup_name, make_backup, parse_time, sha256_of, to_delete, verify


class NameTests(unittest.TestCase):
    def test_backup_name(self):
        self.assertEqual(backup_name("Documents", datetime(2026, 10, 31, 8, 15, 2)), "Documents-20261031-081502.zip")

    def test_parse_time(self):
        self.assertEqual(parse_time("Documents-20261031-081502.zip"), datetime(2026, 10, 31, 8, 15, 2))
        self.assertEqual(parse_time("my-docs-20261031-081502.zip"), datetime(2026, 10, 31, 8, 15, 2))

    def test_not_a_backup(self):
        for name in ["notes.txt", "Documents-20261031-081502.zip.sha256", "x-2026-10-31.zip", "x-20261399-081502.zip"]:
            self.assertIsNone(parse_time(name), name)


class RetentionTests(unittest.TestCase):
    NAMES = [
        "d-20261027-120000.zip", "d-20261031-080000.zip", "d-20261030-070000.zip", "readme.txt",
        "d-20261031-180000.zip", "d-20261028-120000.zip", "d-20261030-220000.zip", "d-20261031-090000.zip",
    ]

    def test_example_from_the_lesson(self):
        self.assertEqual(
            to_delete(self.NAMES, keep_last=2, keep_daily=3),
            ["d-20261027-120000.zip", "d-20261030-070000.zip", "d-20261031-080000.zip"],
        )

    def test_keep_everything(self):
        self.assertEqual(to_delete(self.NAMES, keep_last=10, keep_daily=0), [])

    def test_daily_only(self):
        self.assertEqual(len(to_delete(self.NAMES, keep_last=0, keep_daily=1)), 6)


class FileTests(unittest.TestCase):
    def setUp(self):
        self.tmp = tempfile.TemporaryDirectory()
        root = Path(self.tmp.name)
        self.src = root / "Documents"
        (self.src / "sub").mkdir(parents=True)
        (self.src / "a.txt").write_text("hello")
        (self.src / "sub" / "b.txt").write_text("world")
        self.dest = root / "backups"
        self.dest.mkdir()

    def tearDown(self):
        self.tmp.cleanup()

    def test_sha256(self):
        self.assertEqual(sha256_of(self.src / "a.txt"), "2cf24dba5fb0a30e26e83b2ac5b9e29e1b161e5c1fa7425e73043362938b9824")

    def test_backup_and_verify(self):
        import zipfile
        z = make_backup(self.src, self.dest, datetime(2026, 10, 31, 8, 15, 2))
        self.assertEqual(z.name, "Documents-20261031-081502.zip")
        with zipfile.ZipFile(z) as f:
            self.assertEqual(sorted(n.replace("\\", "/") for n in f.namelist()), ["a.txt", "sub/b.txt"])
        checksum = Path(str(z) + ".sha256").read_text()
        self.assertTrue(checksum.endswith("  Documents-20261031-081502.zip\n"))
        self.assertTrue(verify(z))

    def test_tampered_backup_fails(self):
        z = make_backup(self.src, self.dest, datetime(2026, 10, 31, 8, 15, 2))
        with open(z, "ab") as f:
            f.write(b"ransomware was here")
        self.assertFalse(verify(z))

    def test_missing_checksum_fails(self):
        z = make_backup(self.src, self.dest, datetime(2026, 10, 31, 8, 15, 2))
        Path(str(z) + ".sha256").unlink()
        self.assertFalse(verify(z))


if __name__ == "__main__":
    unittest.main()
@@file main.py
"""rotator: zip backups with checksums and a retention policy.

    python main.py backup SOURCE DEST [--keep-last 3] [--keep-daily 7] [--dry-run]
    python main.py verify DEST
"""

import argparse
import sys
from datetime import datetime
from pathlib import Path

from rotator import backup_name, make_backup, parse_time, to_delete, verify


def cmd_backup(args):
    source, dest = Path(args.source), Path(args.dest)
    if not source.is_dir():
        print(f"error: {source} is not a folder", file=sys.stderr)
        return 2
    now = datetime.now()
    if args.dry_run:
        print(f"would create {dest / backup_name(source.name, now)}")
        names = [p.name for p in dest.glob("*.zip")] + [backup_name(source.name, now)]
    else:
        dest.mkdir(parents=True, exist_ok=True)
        z = make_backup(source, dest, now)
        print(f"created {z} ({z.stat().st_size:,} bytes)")
        names = [p.name for p in dest.glob("*.zip")]
    mine = [n for n in names if n.startswith(source.name + "-")]
    for name in to_delete(mine, args.keep_last, args.keep_daily):
        if args.dry_run:
            print(f"would delete {name}")
        else:
            (dest / name).unlink()
            Path(dest / (name + ".sha256")).unlink(missing_ok=True)
            print(f"deleted {name}")
    return 0


def cmd_verify(args):
    bad = 0
    for z in sorted(Path(args.dest).glob("*.zip")):
        if parse_time(z.name) is None:
            continue
        ok = verify(z)
        bad += not ok
        print(f"{'OK  ' if ok else 'BAD '} {z.name}")
    return 1 if bad else 0


def main(argv=None):
    parser = argparse.ArgumentParser(description="Zip backups with checksums and retention.")
    sub = parser.add_subparsers(dest="command", required=True)
    p = sub.add_parser("backup")
    p.add_argument("source")
    p.add_argument("dest")
    p.add_argument("--keep-last", type=int, default=3)
    p.add_argument("--keep-daily", type=int, default=7)
    p.add_argument("--dry-run", action="store_true", help="only print what would happen")
    p.set_defaults(func=cmd_backup)
    p = sub.add_parser("verify")
    p.add_argument("dest")
    p.set_defaults(func=cmd_verify)
    args = parser.parse_args(argv)
    return args.func(args)


if __name__ == "__main__":
    sys.exit(main())
@@text
> Tip for `parse_time`: split off the last two dash-separated parts with `name.removesuffix(".zip").rsplit("-", 2)`, then let `datetime.strptime(..., STAMP)` reject anything that isn't a real time (wrap it in `try/except ValueError`). For `to_delete`, sort the parsed backups newest first and build a set of names to **keep**; everything else goes.

@@project rotator_finished Backup Rotator: the finished version
The complete project. Schedule it for real: on Windows, **Task Scheduler** → Create Basic Task → run `python C:\path\main.py backup ...` daily. On Linux, a crontab line like `0 2 * * * python3 /opt/rotator/main.py backup /home/me /mnt/backup`. Then, once a month, *restore* a backup to a test folder. A restore you've never tried is the most common way backups fail.
@@file rotator.py
"""Timestamped zip backups with checksums and a retention policy."""

import hashlib
import zipfile
from datetime import datetime
from pathlib import Path

STAMP = "%Y%m%d-%H%M%S"


def backup_name(source_name, when):
    """"Documents", datetime(2026, 10, 31, 8, 15, 2) -> "Documents-20261031-081502.zip"."""
    return f"{source_name}-{when.strftime(STAMP)}.zip"


def parse_time(name):
    """The datetime in a backup file name, or None if the name isn't one of ours.
    "Documents-20261031-081502.zip" -> datetime(2026, 10, 31, 8, 15, 2).
    Source names may contain dashes themselves: "my-docs-20261031-081502.zip" works too."""
    if not name.endswith(".zip"):
        return None
    parts = name.removesuffix(".zip").rsplit("-", 2)
    if len(parts) != 3:
        return None
    try:
        return datetime.strptime(f"{parts[1]}-{parts[2]}", STAMP)
    except ValueError:
        return None


def to_delete(names, keep_last, keep_daily):
    """Which backup names the retention policy deletes, oldest first.

    Keep the `keep_last` newest backups, and the newest backup of each of the
    `keep_daily` most recent days that have backups. Names that aren't backups are ignored."""
    backups = sorted(((t, n) for n in names if (t := parse_time(n))), reverse=True)
    keep = {n for _, n in backups[:keep_last]}
    days = []
    for t, n in backups:
        if t.date() not in days:
            days.append(t.date())
            if len(days) > keep_daily:
                break
            keep.add(n)  # the first one we see for a day is its newest
    return [n for _, n in reversed(backups) if n not in keep]


def sha256_of(path):
    """The SHA-256 hex digest of a file, read in chunks (backups can be huge)."""
    h = hashlib.sha256()
    with open(path, "rb") as f:
        while chunk := f.read(1024 * 1024):
            h.update(chunk)
    return h.hexdigest()


def make_backup(source, dest_dir, when):
    """Zip every file under `source` into dest_dir/backup_name(...), with paths
    relative to `source`. Also write "<zip>.sha256" containing "<digest>  <zip file name>\\n".
    Returns the zip's Path."""
    source = Path(source)
    target = Path(dest_dir) / backup_name(source.name, when)
    with zipfile.ZipFile(target, "w", zipfile.ZIP_DEFLATED) as z:
        for path in sorted(source.rglob("*")):
            if path.is_file():
                z.write(path, path.relative_to(source).as_posix())
    Path(str(target) + ".sha256").write_text(f"{sha256_of(target)}  {target.name}\n")
    return target


def verify(zip_path):
    """True if the zip's .sha256 file exists and matches, and the zip itself is intact
    (zipfile's testzip() returns None)."""
    checksum = Path(str(zip_path) + ".sha256")
    if not checksum.exists():
        return False
    expected = checksum.read_text().split()[0]
    if sha256_of(zip_path) != expected:
        return False
    try:
        with zipfile.ZipFile(zip_path) as z:
            return z.testzip() is None
    except zipfile.BadZipFile:
        return False
@@file test_rotator.py
import tempfile
import unittest
from datetime import datetime
from pathlib import Path

from rotator import backup_name, make_backup, parse_time, sha256_of, to_delete, verify


class NameTests(unittest.TestCase):
    def test_backup_name(self):
        self.assertEqual(backup_name("Documents", datetime(2026, 10, 31, 8, 15, 2)), "Documents-20261031-081502.zip")

    def test_parse_time(self):
        self.assertEqual(parse_time("Documents-20261031-081502.zip"), datetime(2026, 10, 31, 8, 15, 2))
        self.assertEqual(parse_time("my-docs-20261031-081502.zip"), datetime(2026, 10, 31, 8, 15, 2))

    def test_not_a_backup(self):
        for name in ["notes.txt", "Documents-20261031-081502.zip.sha256", "x-2026-10-31.zip", "x-20261399-081502.zip"]:
            self.assertIsNone(parse_time(name), name)


class RetentionTests(unittest.TestCase):
    NAMES = [
        "d-20261027-120000.zip", "d-20261031-080000.zip", "d-20261030-070000.zip", "readme.txt",
        "d-20261031-180000.zip", "d-20261028-120000.zip", "d-20261030-220000.zip", "d-20261031-090000.zip",
    ]

    def test_example_from_the_lesson(self):
        self.assertEqual(
            to_delete(self.NAMES, keep_last=2, keep_daily=3),
            ["d-20261027-120000.zip", "d-20261030-070000.zip", "d-20261031-080000.zip"],
        )

    def test_keep_everything(self):
        self.assertEqual(to_delete(self.NAMES, keep_last=10, keep_daily=0), [])

    def test_daily_only(self):
        self.assertEqual(len(to_delete(self.NAMES, keep_last=0, keep_daily=1)), 6)


class FileTests(unittest.TestCase):
    def setUp(self):
        self.tmp = tempfile.TemporaryDirectory()
        root = Path(self.tmp.name)
        self.src = root / "Documents"
        (self.src / "sub").mkdir(parents=True)
        (self.src / "a.txt").write_text("hello")
        (self.src / "sub" / "b.txt").write_text("world")
        self.dest = root / "backups"
        self.dest.mkdir()

    def tearDown(self):
        self.tmp.cleanup()

    def test_sha256(self):
        self.assertEqual(sha256_of(self.src / "a.txt"), "2cf24dba5fb0a30e26e83b2ac5b9e29e1b161e5c1fa7425e73043362938b9824")

    def test_backup_and_verify(self):
        import zipfile
        z = make_backup(self.src, self.dest, datetime(2026, 10, 31, 8, 15, 2))
        self.assertEqual(z.name, "Documents-20261031-081502.zip")
        with zipfile.ZipFile(z) as f:
            self.assertEqual(sorted(n.replace("\\", "/") for n in f.namelist()), ["a.txt", "sub/b.txt"])
        checksum = Path(str(z) + ".sha256").read_text()
        self.assertTrue(checksum.endswith("  Documents-20261031-081502.zip\n"))
        self.assertTrue(verify(z))

    def test_tampered_backup_fails(self):
        z = make_backup(self.src, self.dest, datetime(2026, 10, 31, 8, 15, 2))
        with open(z, "ab") as f:
            f.write(b"ransomware was here")
        self.assertFalse(verify(z))

    def test_missing_checksum_fails(self):
        z = make_backup(self.src, self.dest, datetime(2026, 10, 31, 8, 15, 2))
        Path(str(z) + ".sha256").unlink()
        self.assertFalse(verify(z))


if __name__ == "__main__":
    unittest.main()
@@file main.py
"""rotator: zip backups with checksums and a retention policy.

    python main.py backup SOURCE DEST [--keep-last 3] [--keep-daily 7] [--dry-run]
    python main.py verify DEST
"""

import argparse
import sys
from datetime import datetime
from pathlib import Path

from rotator import backup_name, make_backup, parse_time, to_delete, verify


def cmd_backup(args):
    source, dest = Path(args.source), Path(args.dest)
    if not source.is_dir():
        print(f"error: {source} is not a folder", file=sys.stderr)
        return 2
    now = datetime.now()
    if args.dry_run:
        print(f"would create {dest / backup_name(source.name, now)}")
        names = [p.name for p in dest.glob("*.zip")] + [backup_name(source.name, now)]
    else:
        dest.mkdir(parents=True, exist_ok=True)
        z = make_backup(source, dest, now)
        print(f"created {z} ({z.stat().st_size:,} bytes)")
        names = [p.name for p in dest.glob("*.zip")]
    mine = [n for n in names if n.startswith(source.name + "-")]
    for name in to_delete(mine, args.keep_last, args.keep_daily):
        if args.dry_run:
            print(f"would delete {name}")
        else:
            (dest / name).unlink()
            Path(dest / (name + ".sha256")).unlink(missing_ok=True)
            print(f"deleted {name}")
    return 0


def cmd_verify(args):
    bad = 0
    for z in sorted(Path(args.dest).glob("*.zip")):
        if parse_time(z.name) is None:
            continue
        ok = verify(z)
        bad += not ok
        print(f"{'OK  ' if ok else 'BAD '} {z.name}")
    return 1 if bad else 0


def main(argv=None):
    parser = argparse.ArgumentParser(description="Zip backups with checksums and retention.")
    sub = parser.add_subparsers(dest="command", required=True)
    p = sub.add_parser("backup")
    p.add_argument("source")
    p.add_argument("dest")
    p.add_argument("--keep-last", type=int, default=3)
    p.add_argument("--keep-daily", type=int, default=7)
    p.add_argument("--dry-run", action="store_true", help="only print what would happen")
    p.set_defaults(func=cmd_backup)
    p = sub.add_parser("verify")
    p.add_argument("dest")
    p.set_defaults(func=cmd_verify)
    args = parser.parse_args(argv)
    return args.func(args)


if __name__ == "__main__":
    sys.exit(main())
@@text
🎉 Backups, checksums, retention, dry runs, scheduling: these are the habits that make people trust a sysadmin. Mention this project in interviews, and be ready to explain the **3-2-1 rule**: 3 copies of your data, on 2 different kinds of storage, 1 of them off-site.
