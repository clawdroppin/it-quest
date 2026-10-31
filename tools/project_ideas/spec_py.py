from gen_projects import P

PROJECTS = [
    P("python", "Intermediate", "folder_organizer", "Downloads Folder Organizer", "Sort a messy folder into subfolders by file type and date.",
      """Point it at a messy Downloads folder and it moves every file into `Images/`, `Documents/`, `Installers/`, `Archives/` and so on, by extension, with a dry-run mode, a log, and an undo file that puts everything back.""",
      ["pathlib", "shutil.move", "dicts as lookup tables", "JSON undo log", "argparse flags"],
      ["Rules: a dict from extension to folder name, loaded from rules.json.",
       "Plan: build a list of (source, destination) moves without touching anything.",
       "Dry run: `--dry-run` prints the plan as a table.",
       "Move: do the moves; if a name already exists, add ` (1)`, ` (2)`...",
       "Undo: save the moves to undo.json; `--undo` reverses them."],
      ["A dry run changes nothing, a real run sorts the test folder, and `--undo` restores it exactly."],
      ["How do I make the planning step testable without real files?",
       "What happens with files that have no extension, and how should I handle them?"],
      {"organize.py": '''"""Sort a folder's files into subfolders by type: python organize.py FOLDER [--dry-run] [--undo]"""
import argparse
import json
import shutil
from pathlib import Path

RULES = json.loads(Path(__file__).with_name("rules.json").read_text(encoding="utf-8"))


def plan(folder: Path) -> list[tuple[Path, Path]]:
    """(source, destination) for every file directly inside `folder`."""
    moves = []
    # TODO: look up each file's suffix (lower case) in RULES; unknown types go to "Other"
    return moves


def main() -> None:
    ap = argparse.ArgumentParser(description=__doc__)
    ap.add_argument("folder", type=Path)
    ap.add_argument("--dry-run", action="store_true")
    ap.add_argument("--undo", action="store_true")
    args = ap.parse_args()
    for src, dst in plan(args.folder):
        print(f"{src.name}  ->  {dst.parent.name}/")
    # TODO: move for real unless --dry-run; write undo.json; handle --undo


if __name__ == "__main__":
    main()
''', "rules.json": '''{
  ".jpg": "Images", ".jpeg": "Images", ".png": "Images", ".gif": "Images", ".webp": "Images",
  ".pdf": "Documents", ".docx": "Documents", ".xlsx": "Documents", ".pptx": "Documents", ".txt": "Documents",
  ".zip": "Archives", ".7z": "Archives", ".tar": "Archives", ".gz": "Archives",
  ".exe": "Installers", ".msi": "Installers", ".deb": "Installers", ".AppImage": "Installers",
  ".mp4": "Video", ".mkv": "Video", ".mp3": "Audio"
}'''}),
    P("python", "Advanced", "study_planner", "Certification Study Planner", "Plan your study hours to an exam date, stored in SQLite.",
      """Enter your target certifications with their exam dates and estimated study hours, log what you study each day, and get a plan: hours per week needed, whether you're on track, and a streak counter. Data lives in a SQLite database (Python has `sqlite3` built in).""",
      ["sqlite3", "SQL schema design", "date arithmetic", "functions returning data, printing separately"],
      ["Schema: tables `goal(name, exam_date, hours_needed)` and `session(goal, day, hours)`.",
       "Commands: `goal add`, `log 2.5 aws`, `status`.",
       "Status: hours done, hours left, weeks left, hours per week needed, and on-track or behind.",
       "Streak: count consecutive days with at least one session.",
       "Tests: use an in-memory database (`sqlite3.connect(\":memory:\")`) in the tests."],
      ["`status` shows every goal with correct numbers, and the tests pass using an in-memory database."],
      ["Help me design the tables: should hours be REAL or INTEGER?",
       "How do I calculate the streak with SQL instead of Python?"],
      {"planner.py": '''"""Study planner: python planner.py goal add NAME YYYY-MM-DD HOURS | log HOURS NAME | status"""
import datetime as dt
import sqlite3
import sys

SCHEMA = """
CREATE TABLE IF NOT EXISTS goal (name TEXT PRIMARY KEY, exam_date TEXT NOT NULL, hours_needed REAL NOT NULL);
CREATE TABLE IF NOT EXISTS session (goal TEXT NOT NULL REFERENCES goal(name), day TEXT NOT NULL, hours REAL NOT NULL);
"""


def connect(path: str = "study.db") -> sqlite3.Connection:
    db = sqlite3.connect(path)
    db.executescript(SCHEMA)
    return db


def status(db: sqlite3.Connection, today: dt.date) -> list[dict]:
    """One dict per goal: name, done, left, weeks_left, per_week, on_track."""
    # TODO
    return []


def main(argv: list[str]) -> None:
    db = connect()
    # TODO: goal add / log / status
    for g in status(db, dt.date.today()):
        print(g)


if __name__ == "__main__":
    main(sys.argv[1:])
'''}),
]
