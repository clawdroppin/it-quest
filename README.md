# IT Quest

**Learn IT from zero: scripting and programming languages, networking, security, cloud, the certifications employers ask for, and the path to an IT job in Japan.**

IT Quest is a free, offline desktop app for Windows and Linux. Every lesson has examples you can run, quizzes and flashcards, and practice exercises that are checked for you. There are real projects for every skill, and a local AI helper that coaches you without giving away the answers.

| | |
|---|---|
| 39 tracks | 11 languages, 17 certifications and IT skills, Japanese, the Japan career path, and creative extras: GUI design, Rust UI themes, vector UI design, UI animation, Blender, DaVinci Resolve, the Linux desktop and GIMP |
| 500+ lesson pages | novice to expert, with 1,400+ quiz questions and 3,000+ flashcards |
| 600+ exercises | checked automatically: output, tests and required code |
| 112 projects | guided builds with starters, tests and finished versions, plus open-ended project briefs |

## Download

Go to **[Releases](../../releases)** and download the file for your computer. No installation is needed.

| System | File | How to start |
|---|---|---|
| Windows 10/11 | `IT-Quest-windows-portable.exe` | Double-click it. Windows SmartScreen may warn about an unknown app: choose **More info → Run anyway**. |
| Linux (x86-64) | `IT-Quest-linux-x86_64.AppImage` | `chmod +x IT-Quest-linux-x86_64.AppImage`, then run it (double-click it in most file managers). |
| Debian / Ubuntu | `IT-Quest-linux-amd64.deb` | `sudo apt install ./IT-Quest-linux-amd64.deb` if you'd rather install it. |

The app is **portable**: put it in a folder (or on a USB stick) and your progress is saved next to it in `it_quest_progress.json`. Copy that file to move your progress between computers, including between Windows and Linux. If the folder isn't writable, progress goes to `%APPDATA%\ITQuest` (Windows) or `~/.local/share/ITQuest` (Linux).

The Linux AppImage needs WebKitGTK, which most desktop distributions already have. If it doesn't start, install `libwebkit2gtk-4.1-0` (Debian/Ubuntu) or `webkit2gtk4.1` (Fedora).

## What's inside

**💻 Languages**: Python, PowerShell, Bash, JavaScript, TypeScript, HTML & CSS, Go, C#, C++, Rust and SQL, each from "hello world" to professional practice (testing, packaging, security).

**🎓 IT & certifications**:

- Foundations: IT fundamentals, CompTIA A+, Network+, Linux+, Security+
- Networking: Cisco CCNA, CCNP Enterprise
- Cloud and DevOps: AWS (to Solutions Architect Associate), Microsoft Azure, Google Cloud Professional Cloud Architect, Cloud and containers with CloudOps (landing zones, DR, cost, cloud networking, Python automation), Kubernetes (CKA, with hands-on exam tasks), Terraform Associate (with hands-on HCL), DevOps and SRE (Git and pull requests, GitHub Actions, Docker, Prometheus and Grafana, GitOps, DevSecOps, and a ship-a-service capstone)
- Management and security leadership: ITIL 4, CISSP, PMP
- Japan: Japanese for IT work, 基本情報技術者 (IPA FE), and the Japan career track (visas, résumés, interviews)

**🛠 Projects**: every track has projects. *Builds* come with a starter, tests and a finished version (a BGP route simulator, a Kubernetes manifest linter, a Terraform plan reviewer, a risk register and more). *Project briefs* (🧩) are open-ended: milestones, a starter folder and suggested questions for the AI helper.

**🧑‍💻 Application development**: every programming language goes past IT scripting into building real software. An expert module covers architecture, testing, databases, APIs and shipping, and a capstone app ties it together: a team task board (Python), a full-stack link manager (JavaScript), a meeting-room booking system (TypeScript), an invoice manager (C#), a key-value database (C++), an asset-tracker module with a GUI (PowerShell), a ticket CLI (Bash), an offline habit-tracker PWA (HTML), an online shop database (SQL) and a REST API (Go).

**✨ Extras**: courses beyond IT. The first is **GUI Design & Creation**: windows and the event loop in tkinter, WinForms, WPF, PowerShell, Electron/Tauri, Qt and Fyne; colour, spacing, type and dark themes; interactive controls; cute animations (transitions, keyframes, easing, micro-interactions); drawing your own SVG icons and exporting app icons; and a capstone, the Plant Buddy app.

**🐧 Linux Desktop & Environment** teaches Linux as your everyday system: choosing a distro and desktop, installing and dual-booting, the terminal, files and permissions, apt/dnf/pacman, Flatpak and Snap, customising, dotfiles, drivers, gaming with Proton, backups and troubleshooting, with Bash exercises throughout.

**🎨 GIMP Image Editing** covers GIMP 3 from the interface to layers, selections and masks, colour correction, retouching, text, filters, exporting and Python automation; the exercises rebuild what the tools do to pixels (blend modes, Levels, convolutions).

**🦀 Rust UI & Theme Design** makes Rust apps beautiful: egui, iced, Slint and Tauri compared; colour palettes, design tokens and WCAG contrast; interactive animated buttons with easing and springs; icon fonts, SVG and app icons; layout, accessibility and a polish checklist, with Rust exercises that build the maths behind them.

**✒ Vector UI Design: Penpot & Inkscape** designs screens, buttons, icons and themes with Penpot (the open-source Figma, desktop or self-hosted with Docker) and Inkscape: flex and grid layout, components and variants, a full button system with animated states, a clean icon set and SVG sprites, and three-layer design tokens with light and dark themes.

**✨ UI Animation: Synfig & Glaxnimate** covers motion principles for interfaces, Synfig Studio (tweening, path morphing, bones, linked parameters), Glaxnimate and the Lottie JSON format inside out, and playing animations in web, Tauri and Rust apps with state-machine buttons and reduced-motion support.

**🧊 Blender: 3D, VFX & Motion Graphics** is a full Blender class: modeling and modifiers, shader nodes and Cycles/EEVEE rendering, animation and rigging, Geometry Nodes, particles, fire, smoke, liquids, rigid bodies and cloth, Grease Pencil, motion tracking and the node Compositor, the Video Sequence Editor and bpy scripting.

**🎬 DaVinci Resolve: Edit, Color, Fusion & Audio** goes from import to delivery: the Cut and Edit pages, Fusion VFX and motion graphics, colour grading, scopes, log, ACES and HDR, Fairlight mixing and loudness, codecs and export, and automating Resolve with Python.

**🃏 Review**: flashcards with spaced repetition (Leitner boxes) and practice exams for every track. **⚖ Compare**: one concept, every language, side by side.

## Running code: what to install

The app runs your code with the programs already installed on your computer. The 🏠 **Home** page shows what it found. Install only what you need:

| Language | Windows | Linux |
|---|---|---|
| Python | [python.org](https://www.python.org/downloads/) (tick "Add to PATH") | usually installed: `python3` |
| PowerShell | built in | [PowerShell 7](https://learn.microsoft.com/powershell/scripting/install/installing-powershell-on-linux) (`pwsh`) |
| Bash | [Git for Windows](https://git-scm.com/download/win) (Git Bash) | built in |
| JavaScript / TypeScript | [Node.js](https://nodejs.org) 22.6 or newer (TypeScript runs on Node directly) | the same, or your package manager |
| Go | [go.dev/dl](https://go.dev/dl) | the same |
| C# | [.NET SDK 8+](https://dotnet.microsoft.com/download) | the same |
| C++ | Visual Studio Community with "Desktop development with C++", or MinGW g++ | `sudo apt install g++` |
| Rust | [rustup.rs](https://rustup.rs) | the same |
| SQL | nothing extra: SQLite comes with Python | the same |
| HTML & CSS | nothing: previewed inside the app | the same |

Projects are created in an `ITQuestProjects` folder next to the app. Each project card has buttons to create the files, open them in VS Code, open the folder, and run the project's commands in a terminal.

## The AI helper (optional, local)

The 🤖 **AI Helper** talks to a language model running **on your own computer**, so nothing you write leaves your machine and it costs nothing.

1. Install [Ollama](https://ollama.com) (or [LM Studio](https://lmstudio.ai), or any server with an OpenAI-compatible API).
2. Download a model, for example `ollama pull qwen2.5-coder:7b` (a good balance on most laptops) or `llama3.1:8b`.
3. In IT Quest, open the AI Helper. The default address `http://localhost:11434/v1` works for Ollama (LM Studio uses `http://localhost:1234/v1`). Pick the model.

It knows which lesson, exercise or project you're on and can see your code. **Hints only** mode (on by default) makes it coach you instead of writing the solution. Turn it off when you want a full explanation.

The helper can also **work in the app for you**: when you ask, it rewrites the code in the editor or creates and edits files in your project folder, then runs or checks the code and reads the result, trying again a few times if needed. Every change shows an **↶ Undo** button. Turn this off in the helper's settings if you only want advice.

## Study tips

- Follow the roadmap on the Home page: Foundations → Systems and networks → Security and cloud → Specialize → Senior, with the Japan track alongside.
- Do the exercises without looking at the solution, then the build project of each track: projects are what you show in interviews.
- Use Review every day for ten minutes; practice exams tell you when you're ready to book the real one.

## Building it yourself

You need [Rust](https://rustup.rs), [Node.js](https://nodejs.org) 20+ and, on Linux, Tauri's system libraries.

```bash
npm ci
npx tauri dev       # run with hot reload
npx tauri build     # Windows: src-tauri/target/release/it_quest.exe
```

On Linux, `./scripts/build-linux.sh` installs the libraries (Debian/Ubuntu) and builds the AppImage and .deb into `dist-linux/`.

Releases are built by GitHub Actions (`.github/workflows/build.yml`) on every push to the `release` branch: both platforms are built and attached to a GitHub Release named after the version in `package.json`.

Check all course content (runs every example, exercise solution and project with the installed languages):

```bash
cd src-tauri
cargo run -- --verify --projects
```

Contributors and AI coding assistants: read [AGENTS.md](AGENTS.md) first.

## Licence and disclaimer

Course content is written for learning. Exam details (formats, domains, prices) change; always check the official exam guide before you book. Certification names are trademarks of their owners; this app isn't affiliated with or endorsed by them.
