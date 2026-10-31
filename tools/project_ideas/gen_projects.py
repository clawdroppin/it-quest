"""Turn compact project specs into lesson files and register them in tracks.rs.

usage: python tools/project_ideas/gen_projects.py tools/project_ideas/spec_*.py   (always pass ALL specs)
Each spec defines PROJECTS = [P(...), ...]. Re-running replaces earlier generated files (zp_*.md)
and their build! lines for the tracks it touches.
"""
import re
import runpy
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
TRACKS_RS = ROOT / "src-tauri/src/tracks.rs"
MARK = "// generated project ideas"


def lesson(p, n):
    out = [f"@@lesson xp-{p['slug']} {p['title']}", "@@text", p["pitch"].strip(), ""]
    out.append("**You'll practise:** " + " · ".join(p["skills"]))
    out += ["", "## Milestones", ""]
    for i, s in enumerate(p["steps"], 1):
        name, _, rest = s.partition(": ")
        out.append(f"{i}. **{name}**: {rest}" if rest else f"{i}. {name}")
    out += ["", "## You're done when", ""] + [f"- {d}" for d in p["done"]]
    if p.get("stretch"):
        out += ["", "## Stretch goals", ""] + [f"- {s}" for s in p["stretch"]]
    out += ["", "## Getting help", "",
            "Open the project, then the 🤖 **AI Helper**: it sees the lesson and your files. Good questions to start with:", ""]
    out += [f"- *\"{a}\"*" for a in p["ask"]]
    out += ["", "Ask for hints, reviews and explanations rather than finished code: you learn by writing it."]
    out.append(f"@@project {p['slug']} {p['title']}")
    out.append(p.get("card", "Create the folder, open it in VS Code, and work through `README.md` one milestone at a time."))
    readme = [f"# {p['title']}", "", p["pitch"].strip().split("\n\n")[0], "", "## Milestones", ""]
    readme += [f"- [ ] {i}. {s}" for i, s in enumerate(p["steps"], 1)]
    readme += ["", "## Done when", ""] + [f"- [ ] {d}" for d in p["done"]]
    if p.get("stretch"):
        readme += ["", "## Stretch goals", ""] + [f"- {s}" for s in p["stretch"]]
    out += ["@@file README.md"] + readme
    for path, body in p["files"].items():
        body = body.strip("\n")
        assert not any(l.startswith("@@") for l in body.splitlines()), path
        out += [f"@@file {path}", body]
    return "\n".join(out) + "\n"


def rust_str(s):
    return '"' + s.replace("\\", "\\\\").replace('"', '\\"') + '"'


def main():
    projects = []
    for spec in sys.argv[1:]:
        projects += runpy.run_path(spec)["PROJECTS"]
    by_track = {}
    for p in projects:
        by_track.setdefault(p["track"], []).append(p)
    rs = TRACKS_RS.read_text(encoding="utf-8")
    for track, ps in by_track.items():
        d = ROOT / "content" / track
        for old in d.glob("zp_*.md"):
            old.unlink()
        # remove earlier generated lines for this track
        rs = re.sub(rf"\n[ \t]*{re.escape(MARK)} {re.escape(track)}\n(?:[ \t]*build!\([^\n]*\n)+", "\n", rs)
        lines = []
        for n, p in enumerate(ps, 1):
            name = f"zp_{n:02d}_{p['slug']}.md"
            (d / name).write_text(lesson(p, n), encoding="utf-8", newline="\n")
            lines.append(f'build!({p["level"]}, {rust_str("🧩 " + p["title"])}, {rust_str(p["blurb"])}, "{track}"; "{name}"),')
        # insert after the last module!/build! of this track
        pat = re.compile(rf'(?:module|build)!\([^;]*?"{re.escape(track)}";[^)]*\),\n', re.S)
        matches = list(pat.finditer(rs))
        assert matches, track
        m = matches[-1]
        indent = re.search(r"\n([ \t]*)(?:module|build)!\([^\n]*$", rs[: m.start() + 8]).group(1)
        block = f"{indent}{MARK} {track}\n" + "".join(f"{indent}{l}\n" for l in lines)
        rs = rs[: m.end()] + block + rs[m.end():]
        print(f"{track}: {len(ps)} projects")
    TRACKS_RS.write_text(rs, encoding="utf-8", newline="\n")


def P(track, level, slug, title, blurb, pitch, skills, steps, done, ask, files, stretch=(), card=None):
    d = dict(track=track, level=level, slug=slug, title=title, blurb=blurb, pitch=pitch, skills=skills,
             steps=steps, done=done, ask=ask, files=files, stretch=list(stretch))
    if card:
        d["card"] = card
    return d


if __name__ == "__main__":
    main()
