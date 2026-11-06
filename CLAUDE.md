# CLAUDE.md

Read [AGENTS.md](AGENTS.md): it has the project layout, the lesson format, how to add content, and the commands to check your work.

Quick reminders:

- Never rename lesson ids or reorder exercises (they're progress keys).
- New quiz/drill/flashcard/text blocks go after a lesson's last quiz, drill or project: quiz and drill answers are keyed by block position (see `KEY_MIGRATIONS` in src/progress.ts).
- After content changes: `cd src-tauri && CARGO_TARGET_DIR=target/verify cargo run -q -- --verify --only <track>/ --projects` must end with "0 problem(s)".
- Project briefs (`content/*/zp_*.md`) are generated: edit `tools/project_ideas/spec_*.py` and rerun the generator with all specs.
- Releases: bump the version in package.json, src-tauri/tauri.conf.json and src-tauri/Cargo.toml, then push to the `release` branch.
