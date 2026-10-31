# Working on IT Quest (instructions for AI assistants and contributors)

IT Quest is a Tauri 2 desktop app: a **Rust backend** (`src-tauri/`) that embeds the whole course and runs learners' code, and a **React + TypeScript frontend** (`src/`). All lesson content is Markdown-like text in `content/`, compiled into the exe with `include_str!`, so the app is a single portable file.

The owner's goal: a beginner becomes a flexible IT generalist able to get an IT job in Japan paying well above minimum wage. Content should be practical, job-relevant and honest about exams (they change: say "check the current exam guide" where numbers may move).

## Layout

| Path | What |
|---|---|
| `content/<track>/*.md` | lessons, one file each (format below) |
| `src-tauri/src/tracks.rs` | every track and its modules: **a lesson file only appears if it's listed here** |
| `src-tauri/src/course.rs` | lesson parser, the `module!` / `build!` macros, `CONCEPTS`, `load_course()` (the order of language tracks) |
| `src-tauri/src/runner.rs` | finds installed languages and runs code with timeouts, output caps and checks |
| `src-tauri/src/projects.rs` | project folders, VS Code / folder / terminal buttons (Windows and Linux) |
| `src-tauri/src/progress.rs` | progress JSON next to the exe (or the AppImage), else `%APPDATA%` / `~/.local/share` |
| `src-tauri/src/ai.rs` | the local AI helper (OpenAI-compatible API, streaming) |
| `src-tauri/src/main.rs` | the app, plus the `--verify` content checker |
| `src/` | UI: `Home.tsx` (roadmap: track ids listed there must exist or they're skipped), `TrackPage.tsx`, `LessonView.tsx`, `Workbench.tsx`, `Review.tsx`, `AiPanel.tsx` |
| `tools/` | content tooling (below) |
| `docs/` | the GitHub Pages site |
| `.github/workflows/` | release builds (Windows exe, Linux AppImage/.deb) and Pages |

## Lesson format

Sections start with `@@` at the beginning of a line (full reference at the top of `course.rs`):

```text
@@lesson <id> <Title>          id is local to the track; the full id is "<track>/<id>" (stored in progress: never rename)
@@concepts loops, functions     optional, only ids from CONCEPTS in course.rs
@@lang python                   optional, when the lesson's code isn't in the track's language
@@text                          Markdown. ```python blocks get "Try it" buttons; ```python,norun don't run
@@quiz                          question, then `- wrong`, `* correct`, `? explanation`
@@flashcards                    `front :: back`, one per line
@@drill                         typed answer: question, `= accepted answer` lines, `? explanation`
@@exercise <Title>              then @@starter, @@expected / @@stdin / @@tests / @@requires, @@hint, @@solution
@@project <folder> <Title>      a project card, followed by @@file <path> blocks
```

Rules that keep things working:

- **Never change an existing lesson id or exercise order**: progress keys are `<lesson id>#<exercise index>`.
- Cert tracks have `lang: None`; their exercises need `@@lang python` (or another language).
- Projects whose folder ends in `_finished` must pass their own tests in `--verify` (Python: `python -m unittest`; C#: `dotnet run -- --test`; C++: `tests.cpp`; JS/TS: `node --test`; PowerShell: `tests.ps1`; Bash: `tests.sh`; Go: `go test`).
- Project language is detected from file names (`Cargo.toml`, `go.mod`, `.csproj`, `.cpp`, `.ts`, `.py`, `.ps1`, `.sh`, `.js`, `.sql`, `.html`, in that order).
- No line inside a `@@file` may start with `@@`.

## Adding content

- **A lesson**: write `content/<track>/<file>.md`, add it to a `module!(...)` in `tracks.rs`.
- **A guided build** (starter + tests + finished): write the finished project and its tests in a scratch folder, make the starter with `python tools/stub.py finished.py starter.py func1 func2` (replaces bodies with `raise NotImplementedError`, keeps docstrings and `# HINT` comments), describe the lesson in a spec and run `python tools/assemble.py spec.py` (see its docstring). Register with `build!(...)`.
- **Project briefs** (🧩, open-ended): add a `P(...)` entry to a spec in `tools/project_ideas/`, then run **all** specs: `python tools/project_ideas/gen_projects.py tools/project_ideas/spec_*.py`. It rewrites `content/*/zp_*.md` and the `// generated project ideas <track>` lines in `tracks.rs`. Don't edit those generated files by hand.
- **A new track**: a `Track { ... }` in `tracks.rs` (`knowledge()` for certs/careers, its own fn + `load_course()` for languages), and add its id to the roadmap in `src/Home.tsx`.

## Checking your work

```bash
npx tsc --noEmit -p .                                   # frontend types
cd src-tauri
CARGO_TARGET_DIR=target/verify cargo clippy --all-targets
CARGO_TARGET_DIR=target/verify cargo test
CARGO_TARGET_DIR=target/verify cargo run -q -- --verify --projects          # everything (several minutes)
CARGO_TARGET_DIR=target/verify cargo run -q -- --verify --only gcp/ --projects -v   # one track, with detail
```

The separate `CARGO_TARGET_DIR` keeps verify builds from fighting with a running `tauri dev`. Verify skips languages that aren't installed (it says so) and prints `[note]` for examples that intentionally fail (e.g. Rust compiler-error demos). "0 problem(s)" is the bar.

## Platform notes

- Windows is the main platform; Linux is supported (AppImage). Anything that spawns programs must work on both: see `projects.rs` (`open_target`, `run_in_terminal`) and `runner.rs` (`command()` hides console windows on Windows; `kill_tree` kills process trees on timeout).
- Progress and projects live next to the executable (`APPIMAGE` env var for AppImages, since those run from a read-only mount).
- Release builds: push to the `release` branch; `.github/workflows/build.yml` builds both platforms and publishes the GitHub Release for the version in `package.json` (bump it in `package.json`, `src-tauri/tauri.conf.json` and `src-tauri/Cargo.toml` together).

## Style

- Match the surrounding code: small functions, comments only where they explain why.
- Lessons: plain, friendly English for learners whose first language may not be English; short sentences; tables for comparisons; real-world IT examples (Japanese companies and yen where it fits); quizzes that test understanding, with an explanation for each.
- Keep security content defensive and educational.
