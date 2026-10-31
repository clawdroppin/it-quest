"""Builds a build-lesson file from a header, project parts and real files on disk.

The spec is a Python file defining:
  OUT = "content/.../x.md"
  PARTS = [("text", "markdown..."),
           ("project", "folder", "Title", "description", base_dir, ["file", ...]),
           ...]
A file entry may be (published_name, source_path) to publish a different source under that name.
"""
import runpy
import sys
from pathlib import Path

spec = runpy.run_path(sys.argv[1])
out = []
for part in spec["PARTS"]:
    if part[0] == "text":
        out.append(part[1].strip("\n"))
    elif part[0] == "raw":
        out.append(part[1].strip("\n"))
    else:
        _, folder, title, desc, base, files = part
        out.append(f"@@project {folder} {title}\n{desc.strip()}")
        for f in files:
            name, src = (f, f) if isinstance(f, str) else f
            body = (Path(base) / src).read_text(encoding="utf-8").rstrip("\n")
            out.append(f"@@file {name}\n{body}")
Path(spec["OUT"]).write_text("\n".join(out) + "\n", encoding="utf-8")
print("wrote", spec["OUT"])
