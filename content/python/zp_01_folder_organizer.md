@@lesson xp-folder_organizer Downloads Folder Organizer
@@text
Point it at a messy Downloads folder and it moves every file into `Images/`, `Documents/`, `Installers/`, `Archives/` and so on, by extension, with a dry-run mode, a log, and an undo file that puts everything back.

**You'll practise:** pathlib · shutil.move · dicts as lookup tables · JSON undo log · argparse flags

## Milestones

1. **Rules**: a dict from extension to folder name, loaded from rules.json.
2. **Plan**: build a list of (source, destination) moves without touching anything.
3. **Dry run**: `--dry-run` prints the plan as a table.
4. **Move**: do the moves; if a name already exists, add ` (1)`, ` (2)`...
5. **Undo**: save the moves to undo.json; `--undo` reverses them.

## You're done when

- A dry run changes nothing, a real run sorts the test folder, and `--undo` restores it exactly.

## Getting help

Open the project, then the 🤖 **AI Helper**: it sees the lesson and your files. Good questions to start with:

- *"How do I make the planning step testable without real files?"*
- *"What happens with files that have no extension, and how should I handle them?"*

Ask for hints, reviews and explanations rather than finished code: you learn by writing it.
@@project folder_organizer Downloads Folder Organizer
Create the folder, open it in VS Code, and work through `README.md` one milestone at a time.
@@file README.md
# Downloads Folder Organizer

Point it at a messy Downloads folder and it moves every file into `Images/`, `Documents/`, `Installers/`, `Archives/` and so on, by extension, with a dry-run mode, a log, and an undo file that puts everything back.

## Milestones

- [ ] 1. Rules: a dict from extension to folder name, loaded from rules.json.
- [ ] 2. Plan: build a list of (source, destination) moves without touching anything.
- [ ] 3. Dry run: `--dry-run` prints the plan as a table.
- [ ] 4. Move: do the moves; if a name already exists, add ` (1)`, ` (2)`...
- [ ] 5. Undo: save the moves to undo.json; `--undo` reverses them.

## Done when

- [ ] A dry run changes nothing, a real run sorts the test folder, and `--undo` restores it exactly.
@@file organize.py
"""Sort a folder's files into subfolders by type: python organize.py FOLDER [--dry-run] [--undo]"""
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
@@file rules.json
{
  ".jpg": "Images", ".jpeg": "Images", ".png": "Images", ".gif": "Images", ".webp": "Images",
  ".pdf": "Documents", ".docx": "Documents", ".xlsx": "Documents", ".pptx": "Documents", ".txt": "Documents",
  ".zip": "Archives", ".7z": "Archives", ".tar": "Archives", ".gz": "Archives",
  ".exe": "Installers", ".msi": "Installers", ".deb": "Installers", ".AppImage": "Installers",
  ".mp4": "Video", ".mkv": "Video", ".mp3": "Audio"
}
