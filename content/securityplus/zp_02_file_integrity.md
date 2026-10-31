@@lesson xp-file_integrity File Integrity Monitor
@@text
Integrity monitoring is a core detective control: record a SHA-256 hash of every file in important folders, then later report anything that changed, appeared or disappeared. You'll build a small version of tools like Tripwire or OSSEC's syscheck.

**You'll practise:** hashing (SHA-256) · baselines · detective controls · hashlib and os.walk · JSON

## Milestones

1. **Baseline**: `fim.py init FOLDER` writes baseline.json with a hash, size and modified time per file.
2. **Check**: `fim.py check FOLDER` reports modified, added and deleted files.
3. **Large files**: hash in chunks so a 4 GB file doesn't fill memory.
4. **Ignore list**: patterns like `*.log` or `cache/` that are expected to change.
5. **Protect the baseline**: store its own hash separately and explain why it matters.

## You're done when

- Changing, adding and deleting test files are each reported correctly, and nothing else is.

## Getting help

Open the project, then the 🤖 **AI Helper**: it sees the lesson and your files. Good questions to start with:

- *"Why is SHA-256 suitable here but MD5 is not?"*
- *"How could an attacker defeat this, and what would stop them?"*

Ask for hints, reviews and explanations rather than finished code: you learn by writing it.
@@project file_integrity File Integrity Monitor
Create the folder, open it in VS Code, and work through `README.md` one milestone at a time.
@@file README.md
# File Integrity Monitor

Integrity monitoring is a core detective control: record a SHA-256 hash of every file in important folders, then later report anything that changed, appeared or disappeared. You'll build a small version of tools like Tripwire or OSSEC's syscheck.

## Milestones

- [ ] 1. Baseline: `fim.py init FOLDER` writes baseline.json with a hash, size and modified time per file.
- [ ] 2. Check: `fim.py check FOLDER` reports modified, added and deleted files.
- [ ] 3. Large files: hash in chunks so a 4 GB file doesn't fill memory.
- [ ] 4. Ignore list: patterns like `*.log` or `cache/` that are expected to change.
- [ ] 5. Protect the baseline: store its own hash separately and explain why it matters.

## Done when

- [ ] Changing, adding and deleting test files are each reported correctly, and nothing else is.
@@file fim.py
"""File integrity monitor: python fim.py init|check FOLDER"""
import hashlib
import json
import sys
from pathlib import Path


def sha256(path: Path) -> str:
    h = hashlib.sha256()
    with path.open("rb") as f:
        for chunk in iter(lambda: f.read(1 << 20), b""):
            h.update(chunk)
    return h.hexdigest()


def snapshot(folder: Path) -> dict[str, str]:
    """Relative path → hash for every file under folder."""
    # TODO
    return {}


def compare(old: dict[str, str], new: dict[str, str]) -> dict[str, list[str]]:
    """{"modified": [...], "added": [...], "deleted": [...]}"""
    # TODO
    return {"modified": [], "added": [], "deleted": []}


if __name__ == "__main__":
    command, folder = sys.argv[1], Path(sys.argv[2])
    if command == "init":
        Path("baseline.json").write_text(json.dumps(snapshot(folder), indent=2))
    else:
        print(compare(json.loads(Path("baseline.json").read_text()), snapshot(folder)))
