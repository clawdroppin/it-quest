//! Projects that lessons create on disk, for work in VS Code and a real terminal.

use crate::course::Project;
use crate::runner::{Lang, find_tool};
use serde::Serialize;
use std::path::{Path, PathBuf};
use std::process::Command;

const ROOT_NAME: &str = "ITQuestProjects";

/// Next to the exe if that folder is writable, otherwise in Documents. Worked out once.
pub fn root() -> PathBuf {
    static ROOT: std::sync::OnceLock<PathBuf> = std::sync::OnceLock::new();
    ROOT.get_or_init(find_root).clone()
}

fn find_root() -> PathBuf {
    if let Some(dir) = crate::progress::exe_dir() {
        let root = dir.join(ROOT_NAME);
        let probe = root.join(".write_test");
        if std::fs::create_dir_all(&root).is_ok() && std::fs::write(&probe, b"ok").is_ok() {
            let _ = std::fs::remove_file(probe);
            return root;
        }
    }
    home_dir().join("Documents").join(ROOT_NAME)
}

/// The user's home folder: USERPROFILE on Windows, HOME elsewhere.
fn home_dir() -> PathBuf {
    std::env::var_os("USERPROFILE").or_else(|| std::env::var_os("HOME")).map(PathBuf::from).unwrap_or_default()
}

/// `track` keeps projects of different tracks apart (two tracks may both have a "todo" folder).
pub fn dir_of(track: &str, project: &Project) -> PathBuf {
    root().join(track).join(&project.folder)
}

/// The language a project is written in, from its files.
pub fn lang_of(project: &Project) -> Option<Lang> {
    let has = |ext: &str| project.files.iter().any(|(p, _)| p.ends_with(ext));
    if has("Cargo.toml") {
        Some(Lang::Rust)
    } else if has("go.mod") {
        Some(Lang::Go)
    } else if has(".csproj") {
        Some(Lang::CSharp)
    } else if has(".cpp") {
        Some(Lang::Cpp)
    } else if has(".ts") {
        Some(Lang::TypeScript)
    } else if has(".py") {
        Some(Lang::Python)
    } else if has(".ps1") {
        Some(Lang::PowerShell)
    } else if has(".sh") {
        Some(Lang::Bash)
    } else if has(".js") {
        Some(Lang::JavaScript)
    } else if has(".sql") {
        Some(Lang::Sql)
    } else if has(".html") {
        Some(Lang::Html)
    } else {
        None
    }
}

#[derive(Serialize)]
pub struct ProjectCommand {
    pub label: String,
    pub command: String,
}

/// The run and test buttons on a project card.
pub fn commands(project: &Project) -> Vec<ProjectCommand> {
    let has = |name: &str| project.files.iter().any(|(p, _)| p == name || p.ends_with(&format!("/{name}")));
    let mut v = Vec::new();
    let mut add = |label: &str, command: String| v.push(ProjectCommand { label: label.into(), command });
    let python = || match find_tool(Lang::Python) {
        Ok(t) if t.pre_args.is_empty() => "python".to_string(),
        Ok(_) => "py -3".to_string(),
        Err(_) => "python".to_string(),
    };
    match lang_of(project) {
        Some(Lang::Rust) => {
            add("▶ cargo run", "cargo run".into());
            add("🧪 cargo test", "cargo test".into());
        }
        Some(Lang::Python) | Some(Lang::Sql) => {
            let py_files: Vec<&String> = project.files.iter().map(|(p, _)| p).filter(|p| p.ends_with(".py") && !p.contains("test")).collect();
            let main = py_files.iter().find(|p| p.as_str() == "main.py").or(py_files.last()).map(|p| p.to_string());
            if let Some(main) = main {
                add(&format!("▶ python {main}"), format!("{} {main}", python()));
            }
            if project.files.iter().any(|(p, _)| p.contains("test") && p.ends_with(".py")) {
                add("🧪 run tests", format!("{} -m unittest -v", python()));
            }
        }
        Some(Lang::JavaScript) => {
            if has("main.js") {
                add("▶ node main.js", "node main.js".into());
            }
            if project.files.iter().any(|(p, _)| p.contains(".test.js")) {
                add("🧪 node --test", "node --test".into());
            }
        }
        Some(Lang::PowerShell) => {
            for (p, _) in &project.files {
                if p.ends_with(".ps1") && !p.contains('/') {
                    let label = if p.contains("test") || p.contains("Test") { "🧪" } else { "▶" };
                    // Windows PowerShell on Windows; PowerShell 7 (pwsh) is the one available on Linux.
                    let shell = if cfg!(windows) { "powershell" } else { "pwsh" };
                    add(&format!("{label} {p}"), format!("{shell} -NoProfile -ExecutionPolicy Bypass -File {p}"));
                }
            }
        }
        Some(Lang::Bash) => {
            let bash = find_tool(Lang::Bash).map(|t| t.program.display().to_string()).unwrap_or_else(|_| "bash".into());
            for (p, _) in &project.files {
                if p.ends_with(".sh") && !p.contains('/') {
                    let label = if p.contains("test") { "🧪" } else { "▶" };
                    add(&format!("{label} bash {p}"), format!("\"{bash}\" {p}"));
                }
            }
        }
        Some(Lang::TypeScript) => {
            if has("main.ts") {
                add("▶ node main.ts", "node main.ts".into());
            }
            if project.files.iter().any(|(p, _)| p.ends_with(".test.ts")) {
                add("🧪 node --test", "node --test".into());
            }
        }
        Some(Lang::Go) => {
            add("▶ go run .", "go run .".into());
            if project.files.iter().any(|(p, _)| p.ends_with("_test.go")) {
                add("🧪 go test", "go test -v ./...".into());
            }
        }
        Some(Lang::CSharp) => {
            add("▶ dotnet run", "dotnet run".into());
            add("🧪 dotnet run -- --test", "dotnet run -- --test".into());
        }
        Some(Lang::Cpp) => {
            for (target, label) in [("main.cpp", "▶ build and run"), ("tests.cpp", "🧪 build and run the tests")] {
                if has(target) {
                    add(label, cpp_command(target));
                }
            }
        }
        Some(Lang::Html) => {
            for (p, _) in project.files.iter().filter(|(p, _)| p.ends_with(".html") && !p.contains('/')) {
                let open = if cfg!(windows) { format!("start \"\" {p}") } else { format!("xdg-open {p}") };
                add(&format!("🌐 open {p} in the browser"), open);
            }
        }
        None => {}
    }
    v
}

/// Compile one C++ file (plus nothing else: projects keep their code in headers) and run it.
fn cpp_command(file: &str) -> String {
    let exe = file.trim_end_matches(".cpp");
    match find_tool(Lang::Cpp) {
        Ok(t) if t.program.file_name().is_some_and(|n| n.eq_ignore_ascii_case("cl.exe")) => {
            // A new terminal doesn't have MSVC's environment: set it up first.
            let vcvars = t.program.ancestors().nth(7).map(|vc| vc.join(r"Auxiliary\Build\vcvars64.bat"));
            let setup = vcvars.map(|p| format!("call \"{}\" >nul && ", p.display())).unwrap_or_default();
            format!("{setup}cl /nologo /EHsc /std:c++20 /utf-8 /W3 {file} /Fe:{exe}.exe && {exe}.exe")
        }
        Ok(t) => {
            let run = if cfg!(windows) { format!("{exe}.exe") } else { format!("./{exe}") };
            format!("{} -std=c++20 -Wall -o {exe} {file} && {run}", t.program.display())
        }
        Err(_) => format!("g++ -std=c++20 -Wall -o {exe} {file} && ./{exe}"),
    }
}

/// How many of the project's files already exist on disk.
pub fn existing_files(track: &str, project: &Project) -> usize {
    let dir = dir_of(track, project);
    project.files.iter().filter(|(path, _)| dir.join(path).exists()).count()
}

/// Write the files that don't exist yet. Never touches the learner's existing work.
pub fn create_missing(track: &str, project: &Project) -> std::io::Result<usize> {
    let dir = dir_of(track, project);
    let mut written = 0;
    for (path, contents) in &project.files {
        let target = dir.join(path);
        if !target.exists() {
            write_file(&target, contents)?;
            written += 1;
        }
    }
    Ok(written)
}

/// Overwrite every file with this lesson's version, backing up the old ones first.
pub fn reset(track: &str, project: &Project) -> std::io::Result<PathBuf> {
    let dir = dir_of(track, project);
    let stamp = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map_or(0, |d| d.as_secs());
    let backup = root().join("_backups").join(format!("{track}_{}_{stamp}", project.folder));
    for (path, contents) in &project.files {
        let target = dir.join(path);
        if target.exists() {
            let saved = backup.join(path);
            if let Some(parent) = saved.parent() {
                std::fs::create_dir_all(parent)?;
            }
            std::fs::copy(&target, saved)?;
        }
        write_file(&target, contents)?;
    }
    Ok(backup)
}

/// The project's files as they are on disk now, for the AI helper. Stops after roughly
/// `max_chars` so small local models aren't overwhelmed.
pub fn read_sources(track: &str, project: &Project, max_chars: usize) -> String {
    let dir = dir_of(track, project);
    let mut out = String::new();
    for (path, _) in &project.files {
        let Ok(text) = std::fs::read_to_string(dir.join(path)) else { continue };
        let fence = path.rsplit('.').next().unwrap_or("text");
        if out.len() + text.len() > max_chars {
            out.push_str(&format!("\n({path} and any further files were left out: too long)\n"));
            break;
        }
        out.push_str(&format!("\n{path}:\n```{fence}\n{}\n```\n", text.trim_end()));
    }
    out
}

fn write_file(target: &Path, contents: &str) -> std::io::Result<()> {
    if let Some(parent) = target.parent() {
        std::fs::create_dir_all(parent)?;
    }
    std::fs::write(target, contents)
}

/// PATH with ~/.cargo/bin added, in case cargo isn't on the system PATH.
fn path_with_cargo() -> std::ffi::OsString {
    let mut paths: Vec<PathBuf> = std::env::var_os("PATH").map(|p| std::env::split_paths(&p).collect()).unwrap_or_default();
    paths.push(home_dir().join(".cargo").join("bin"));
    std::env::join_paths(paths).unwrap_or_default()
}

#[cfg(windows)]
pub fn open_in_vscode(dir: &Path) -> std::io::Result<()> {
    use std::os::windows::process::CommandExt;
    let mut cmd = Command::new("cmd");
    cmd.args(["/C", "code", "."]).current_dir(dir);
    cmd.creation_flags(0x0800_0000); // CREATE_NO_WINDOW
    cmd.spawn().map(|_| ())
}

#[cfg(not(windows))]
pub fn open_in_vscode(dir: &Path) -> std::io::Result<()> {
    Command::new("code").arg(".").current_dir(dir).spawn().map(|_| ())
}

pub fn open_folder(dir: &Path) -> std::io::Result<()> {
    open_target(dir.as_os_str())
}

/// Open a folder or a web link with the system's default program.
pub fn open_target(target: &std::ffi::OsStr) -> std::io::Result<()> {
    let opener = if cfg!(windows) {
        "explorer"
    } else if cfg!(target_os = "macos") {
        "open"
    } else {
        "xdg-open"
    };
    Command::new(opener).arg(target).spawn().map(|_| ())
}

/// Run a command (like `cargo run`) in a new terminal window that stays open afterwards.
#[cfg(windows)]
pub fn run_in_terminal(dir: &Path, command_line: &str) -> std::io::Result<()> {
    use std::os::windows::process::CommandExt;
    let mut cmd = Command::new("cmd");
    cmd.arg("/K").arg(command_line).current_dir(dir).env("PATH", path_with_cargo());
    cmd.creation_flags(0x0000_0010); // CREATE_NEW_CONSOLE
    cmd.spawn().map(|_| ())
}

/// On Linux, try the common terminal emulators in turn; the shell stays open after the command.
#[cfg(not(windows))]
pub fn run_in_terminal(dir: &Path, command_line: &str) -> std::io::Result<()> {
    let script = format!("{command_line}; exec \"${{SHELL:-bash}}\"");
    let candidates: [(&str, &[&str]); 5] = [
        ("x-terminal-emulator", &["-e", "bash", "-c"]),
        ("gnome-terminal", &["--", "bash", "-c"]),
        ("konsole", &["-e", "bash", "-c"]),
        ("xfce4-terminal", &["-x", "bash", "-c"]),
        ("xterm", &["-e", "bash", "-c"]),
    ];
    let mut last = std::io::Error::new(std::io::ErrorKind::NotFound, "no terminal emulator found");
    for (program, args) in candidates {
        match Command::new(program).args(args).arg(&script).current_dir(dir).env("PATH", path_with_cargo()).spawn() {
            Ok(_) => return Ok(()),
            Err(e) => last = e,
        }
    }
    Err(last)
}
