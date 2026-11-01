//! Runs the learner's code with the tools installed on their computer: `rustc` for Rust,
//! Python, Node (JavaScript and TypeScript), PowerShell, Bash, SQLite (through Python's
//! `sqlite3`), Go, C# (the .NET SDK's Roslyn compiler, called directly because it's fast),
//! and C++ (g++/clang++, or MSVC from Visual Studio). HTML needs nothing: the app previews it.
//!
//! Every run happens in its own temp folder, with a timeout and an output cap.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Mutex;
use std::time::{Duration, Instant};

const MAX_OUTPUT: u64 = 100_000;

#[derive(Serialize, Deserialize, Clone, Copy, PartialEq, Eq, Hash, Debug)]
#[serde(rename_all = "lowercase")]
pub enum Lang {
    Rust,
    Python,
    JavaScript,
    PowerShell,
    Bash,
    Sql,
    TypeScript,
    Go,
    CSharp,
    Cpp,
    Html,
}

impl Lang {
    pub const ALL: [Lang; 11] = [
        Lang::Rust,
        Lang::Python,
        Lang::JavaScript,
        Lang::PowerShell,
        Lang::Bash,
        Lang::Sql,
        Lang::TypeScript,
        Lang::Go,
        Lang::CSharp,
        Lang::Cpp,
        Lang::Html,
    ];

    /// The language a fenced code block is in, from its info string (```python).
    pub fn from_fence(name: &str) -> Option<Lang> {
        Some(match name {
            "rust" | "rs" => Lang::Rust,
            "python" | "py" => Lang::Python,
            "javascript" | "js" => Lang::JavaScript,
            "powershell" | "ps1" | "pwsh" => Lang::PowerShell,
            "bash" | "sh" => Lang::Bash,
            "sql" => Lang::Sql,
            "typescript" | "ts" => Lang::TypeScript,
            "go" | "golang" => Lang::Go,
            "csharp" | "cs" | "c#" => Lang::CSharp,
            "cpp" | "c++" | "cxx" => Lang::Cpp,
            "html" => Lang::Html,
            _ => return None,
        })
    }

    pub fn name(self) -> &'static str {
        match self {
            Lang::Rust => "Rust",
            Lang::Python => "Python",
            Lang::JavaScript => "JavaScript",
            Lang::PowerShell => "PowerShell",
            Lang::Bash => "Bash",
            Lang::Sql => "SQL",
            Lang::TypeScript => "TypeScript",
            Lang::Go => "Go",
            Lang::CSharp => "C#",
            Lang::Cpp => "C++",
            Lang::Html => "HTML",
        }
    }

    fn file_name(self) -> &'static str {
        match self {
            Lang::Rust => "main.rs",
            Lang::Python => "main.py",
            Lang::JavaScript => "main.js",
            Lang::PowerShell => "main.ps1",
            Lang::Bash => "main.sh",
            Lang::Sql => "main.sql",
            Lang::TypeScript => "main.ts",
            Lang::Go => "main.go",
            Lang::CSharp => "main.cs",
            Lang::Cpp => "main.cpp",
            Lang::Html => "index.html",
        }
    }

    /// Languages compiled to a program first (Rust has its own path, with test builds).
    fn compiled(self) -> bool {
        matches!(self, Lang::Go | Lang::CSharp | Lang::Cpp)
    }

    fn timeout(self) -> Duration {
        // Windows PowerShell alone takes most of a second to start.
        Duration::from_secs(if self == Lang::PowerShell { 10 } else { 6 })
    }

    /// What a starter's "not done yet" marker looks like when it runs.
    fn todo_marker(self) -> &'static str {
        match self {
            Lang::Rust => "not yet implemented",
            Lang::Python => "NotImplementedError",
            _ => "TODO: not done yet",
        }
    }

    fn install_hint(self) -> &'static str {
        match self {
            Lang::Rust => "Install Rust from https://rustup.rs and restart the app.",
            Lang::Python | Lang::Sql => "Install Python 3 from https://python.org (tick \"Add python.exe to PATH\") and restart the app.",
            Lang::JavaScript => "Install Node.js (LTS) from https://nodejs.org and restart the app.",
            Lang::PowerShell => "Windows PowerShell is part of Windows; it should always be there.",
            Lang::Bash if cfg!(windows) => "Install Git for Windows from https://git-scm.com (it includes Git Bash) and restart the app.",
            Lang::Bash => "Install bash with your package manager.",
            Lang::TypeScript => "TypeScript runs on Node.js 22.6 or newer (it strips the types). Install Node.js (LTS) from https://nodejs.org.",
            Lang::Go => "Install Go from https://go.dev/dl and restart the app.",
            Lang::CSharp => "Install the .NET SDK (8 or newer) from https://dotnet.microsoft.com/download and restart the app.",
            Lang::Cpp if cfg!(windows) => {
                "Install Visual Studio Community with the \"Desktop development with C++\" workload (or MinGW g++), then restart the app."
            }
            Lang::Cpp => "Install g++ (e.g. `sudo apt install g++`) or clang++.",
            Lang::Html => "",
        }
    }
}

/// An installed program that runs one language.
#[derive(Clone, Serialize)]
pub struct Tool {
    pub program: PathBuf,
    /// Arguments that always come first (`py -3`).
    #[serde(skip)]
    pub pre_args: Vec<String>,
    pub version: String,
    /// Extra environment for every run (MSVC's INCLUDE/LIB/PATH).
    #[serde(skip)]
    pub env: Vec<(String, String)>,
    /// Language-specific extras. C#: [csc.dll, runtime version, reference response file].
    #[serde(skip)]
    pub extra: Vec<String>,
}

#[derive(Clone, Deserialize)]
pub struct Job {
    pub lang: Lang,
    pub code: String,
    #[serde(default)]
    pub stdin: String,
    /// Hidden code run first (SQL: create and fill the tables).
    #[serde(default)]
    pub setup: String,
    #[serde(default)]
    pub check: Option<Check>,
}

#[derive(Clone, Deserialize, Default)]
pub struct Check {
    pub expected: Option<String>,
    pub tests: Option<String>,
    #[serde(default)]
    pub requires: Vec<String>,
}

#[derive(Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RunResult {
    /// For Rust: did it compile. Scripts always "compile".
    pub compile_ok: bool,
    pub compiler_output: String,
    pub stdout: String,
    pub stderr: String,
    pub exit_code: Option<i32>,
    pub timed_out: bool,
    /// Set when this was a check.
    pub check: Option<CheckResult>,
    pub internal_error: Option<String>,
}

#[derive(Serialize)]
pub struct CheckResult {
    pub passed: bool,
    pub problems: Vec<String>,
}

fn command(program: &Path) -> Command {
    let mut cmd = Command::new(program);
    #[cfg(windows)]
    {
        use std::os::windows::process::CommandExt;
        const CREATE_NO_WINDOW: u32 = 0x0800_0000;
        cmd.creation_flags(CREATE_NO_WINDOW);
    }
    cmd
}

fn home() -> Option<PathBuf> {
    std::env::var_os("USERPROFILE").or_else(|| std::env::var_os("HOME")).map(PathBuf::from)
}

/// Runs `program pre_args version_args` and returns the first line it prints, if it succeeds.
fn probe(program: &Path, pre_args: &[&str], version_args: &[&str]) -> Option<String> {
    let out = command(program).args(pre_args).args(version_args).output().ok()?;
    if !out.status.success() {
        return None;
    }
    let text = if out.stdout.is_empty() { out.stderr } else { out.stdout };
    Some(String::from_utf8_lossy(&text).lines().next().unwrap_or("").trim().to_string())
}

fn find_tool_uncached(lang: Lang) -> Result<Tool, String> {
    let missing = || format!("{} isn't installed (or wasn't found). {}", lang.name(), lang.install_hint());
    match lang {
        Lang::TypeScript => return find_typescript().ok_or_else(missing),
        Lang::CSharp => return find_csharp().ok_or_else(missing),
        Lang::Cpp => return find_cpp().ok_or_else(missing),
        Lang::Html => {
            return Ok(Tool { program: PathBuf::new(), pre_args: vec![], version: "built-in preview".into(), env: vec![], extra: vec![] });
        }
        _ => {}
    }
    let mut candidates: Vec<(PathBuf, Vec<&str>)> = Vec::new();
    let version_args: &[&str] = match lang {
        Lang::Rust => {
            candidates.push(("rustc".into(), vec![]));
            if let Some(h) = home() {
                candidates.push((h.join(".cargo").join("bin").join("rustc.exe"), vec![]));
                candidates.push((h.join(".cargo").join("bin").join("rustc"), vec![]));
            }
            &["--version"]
        }
        Lang::Python | Lang::Sql => {
            // `python` can be the Microsoft Store stub, which fails `--version`'s check below.
            candidates.push(("python".into(), vec![]));
            candidates.push(("py".into(), vec!["-3"]));
            candidates.push(("python3".into(), vec![]));
            &["--version"]
        }
        Lang::JavaScript => {
            candidates.push(("node".into(), vec![]));
            candidates.push((r"C:\Program Files\nodejs\node.exe".into(), vec![]));
            &["--version"]
        }
        Lang::PowerShell => {
            // Windows PowerShell 5.1 on Windows; PowerShell 7 (pwsh) elsewhere, or if that's all there is.
            candidates.push(("powershell".into(), vec![]));
            candidates.push(("pwsh".into(), vec![]));
            &["-NoProfile", "-NonInteractive", "-Command", "\"PowerShell $($PSVersionTable.PSVersion)\""]
        }
        Lang::Bash => {
            // Git Bash. Never C:\Windows\System32\bash.exe: that's WSL, a different system.
            for base in ["ProgramFiles", "ProgramW6432", "LOCALAPPDATA"] {
                if let Some(dir) = std::env::var_os(base) {
                    let dir = PathBuf::from(dir);
                    candidates.push((dir.join("Git").join("bin").join("bash.exe"), vec![]));
                    candidates.push((dir.join("Programs").join("Git").join("bin").join("bash.exe"), vec![]));
                }
            }
            if !cfg!(windows) {
                candidates.push(("bash".into(), vec![]));
            }
            &["--version"]
        }
        Lang::Go => {
            candidates.push(("go".into(), vec![]));
            candidates.push((r"C:\Program Files\Go\bin\go.exe".into(), vec![]));
            candidates.push(("/usr/local/go/bin/go".into(), vec![]));
            &["version"]
        }
        _ => unreachable!(),
    };
    for (program, pre) in candidates {
        let Some(version) = probe(&program, &pre, version_args) else { continue };
        let version = match lang {
            Lang::Python => {
                if !version.starts_with("Python 3") {
                    continue;
                }
                version
            }
            Lang::Sql => {
                let v = probe(&program, &pre, &["-c", "import sqlite3; print('SQLite', sqlite3.sqlite_version)"]);
                match v {
                    Some(v) => v,
                    None => continue,
                }
            }
            Lang::JavaScript => format!("Node.js {version}"),
            Lang::Bash => version
                .replace("GNU bash, version", if cfg!(windows) { "Git Bash" } else { "Bash" })
                .split(" (")
                .next()
                .unwrap_or("Bash")
                .to_string(),
            Lang::Go => version.replace("go version go", "Go ").split(' ').take(2).collect::<Vec<_>>().join(" "),
            _ => version,
        };
        return Ok(Tool { program, pre_args: pre.iter().map(|s| s.to_string()).collect(), version, env: vec![], extra: vec![] });
    }
    Err(missing())
}

/// Node 22.6+ runs .ts files by stripping the types (no type checking; erasable syntax only).
fn find_typescript() -> Option<Tool> {
    for program in ["node", r"C:\Program Files\nodejs\node.exe"] {
        let Some(v) = probe(Path::new(program), &[], &["--version"]) else { continue };
        let nums: Vec<u32> = v.trim_start_matches('v').split('.').filter_map(|n| n.parse().ok()).collect();
        let (major, minor) = (nums.first().copied().unwrap_or(0), nums.get(1).copied().unwrap_or(0));
        if major < 22 || (major == 22 && minor < 6) {
            continue;
        }
        let mut pre_args = vec!["--disable-warning=ExperimentalWarning".to_string()];
        if major == 22 || (major == 23 && minor < 6) {
            pre_args.push("--experimental-strip-types".into());
        }
        return Some(Tool { program: program.into(), pre_args, version: format!("TypeScript on Node.js {v}"), env: vec![], extra: vec![] });
    }
    None
}

/// The newest .NET SDK, its Roslyn compiler and the matching reference assemblies.
fn find_csharp() -> Option<Tool> {
    let mut dotnets: Vec<PathBuf> =
        vec!["dotnet".into(), r"C:\Program Files\dotnet\dotnet.exe".into(), "/usr/share/dotnet/dotnet".into(), "/usr/lib/dotnet/dotnet".into()];
    if let Some(h) = home() {
        dotnets.push(h.join(".dotnet").join("dotnet"));
    }
    let version_key = |v: &str| v.split(['.', '-']).map(|n| n.parse::<u32>().unwrap_or(0)).collect::<Vec<_>>();
    for dotnet in dotnets {
        let Ok(out) = command(&dotnet).arg("--list-sdks").output() else { continue };
        // Lines like "8.0.425 [C:\Program Files\dotnet\sdk]"
        let sdks = String::from_utf8_lossy(&out.stdout).into_owned();
        let Some((sdk_version, sdk_dir)) = sdks
            .lines()
            .filter_map(|l| l.split_once(" [").map(|(v, d)| (v.trim().to_string(), PathBuf::from(d.trim().trim_end_matches(']')))))
            .max_by_key(|(v, _)| version_key(v))
        else {
            continue;
        };
        let major = sdk_version.split('.').next().unwrap_or("8").to_string();
        let csc = sdk_dir.join(&sdk_version).join("Roslyn").join("bincore").join("csc.dll");
        let Some(root) = sdk_dir.parent().map(Path::to_path_buf) else { continue };
        let newest = |dir: PathBuf| -> Option<String> {
            std::fs::read_dir(dir)
                .ok()?
                .filter_map(|e| e.ok()?.file_name().into_string().ok())
                .filter(|v| v.split('.').next() == Some(major.as_str()))
                .max_by_key(|v| version_key(v))
        };
        let (Some(ref_version), Some(runtime)) =
            (newest(root.join("packs").join("Microsoft.NETCore.App.Ref")), newest(root.join("shared").join("Microsoft.NETCore.App")))
        else {
            continue;
        };
        let ref_dir = root.join("packs").join("Microsoft.NETCore.App.Ref").join(&ref_version).join("ref").join(format!("net{major}.0"));
        let mut rsp = String::new();
        for entry in std::fs::read_dir(&ref_dir).into_iter().flatten().flatten() {
            if entry.path().extension().is_some_and(|e| e == "dll") {
                rsp.push_str(&format!("-r:\"{}\"\n", entry.path().display()));
            }
        }
        if !csc.exists() || rsp.is_empty() {
            continue;
        }
        return Some(Tool {
            program: dotnet,
            pre_args: vec![],
            version: format!("C# (.NET {runtime})"),
            env: vec![],
            extra: vec![csc.display().to_string(), runtime, rsp],
        });
    }
    None
}

/// g++ or clang++ on the PATH, or else MSVC from the newest Visual Studio with the C++ tools.
fn find_cpp() -> Option<Tool> {
    for program in ["g++", "clang++"] {
        if let Some(v) = probe(Path::new(program), &[], &["--version"]) {
            return Some(Tool { program: program.into(), pre_args: vec![], version: v, env: vec![], extra: vec![] });
        }
    }
    if !cfg!(windows) {
        return None;
    }
    let vswhere = PathBuf::from(std::env::var_os("ProgramFiles(x86)")?).join(r"Microsoft Visual Studio\Installer\vswhere.exe");
    let out = command(&vswhere)
        .args(["-latest", "-products", "*", "-requires", "Microsoft.VisualStudio.Component.VC.Tools.x86.x64", "-property", "installationPath"])
        .output()
        .ok()?;
    let install = PathBuf::from(String::from_utf8_lossy(&out.stdout).trim());
    let vcvars = install.join(r"VC\Auxiliary\Build\vcvars64.bat");
    if !vcvars.exists() {
        return None;
    }
    // Run vcvars64 once and keep the environment it sets up (it takes about a second).
    let mut cmd = command(Path::new("cmd"));
    #[cfg(windows)]
    {
        use std::os::windows::process::CommandExt;
        cmd.raw_arg(format!("/C \"call \"{}\" >nul && set\"", vcvars.display()));
    }
    let out = cmd.output().ok()?;
    let env: Vec<(String, String)> = String::from_utf8_lossy(&out.stdout)
        .lines()
        .filter_map(|l| l.split_once('='))
        .filter(|(k, _)| ["PATH", "INCLUDE", "LIB", "LIBPATH"].contains(&k.to_uppercase().as_str()))
        .map(|(k, v)| (k.to_uppercase(), v.to_string()))
        .collect();
    let path = env.iter().find(|(k, _)| k == "PATH")?.1.clone();
    let cl = std::env::split_paths(&path).map(|d| d.join("cl.exe")).find(|p| p.exists())?;
    // ...\VC\Tools\MSVC\14.51.36231\bin\Hostx64\x64\cl.exe
    let version = cl.ancestors().nth(4).and_then(|p| p.file_name()).map(|v| v.to_string_lossy().into_owned()).unwrap_or_default();
    Some(Tool { program: cl, pre_args: vec![], version: format!("MSVC {version} (Visual Studio)"), env, extra: vec![] })
}

/// Finds the tool for a language, once per app run.
pub fn find_tool(lang: Lang) -> Result<Tool, String> {
    static CACHE: Mutex<Option<HashMap<Lang, Result<Tool, String>>>> = Mutex::new(None);
    if let Some(found) = CACHE.lock().unwrap().as_ref().and_then(|m| m.get(&lang)) {
        return found.clone();
    }
    let found = find_tool_uncached(lang);
    CACHE.lock().unwrap().get_or_insert_with(HashMap::new).insert(lang, found.clone());
    found
}

pub fn normalize(s: &str) -> String {
    s.replace("\r\n", "\n").lines().map(str::trim_end).collect::<Vec<_>>().join("\n").trim_end().to_string()
}

fn work_dir() -> std::io::Result<PathBuf> {
    static COUNTER: AtomicU64 = AtomicU64::new(0);
    let n = COUNTER.fetch_add(1, Ordering::Relaxed);
    let dir = std::env::temp_dir().join("it_quest").join(format!("{}_{}", std::process::id(), n));
    std::fs::create_dir_all(&dir)?;
    Ok(dir)
}

struct Ran {
    stdout: String,
    stderr: String,
    exit_code: Option<i32>,
    timed_out: bool,
}

fn run_process(mut cmd: Command, stdin: &str, timeout: Duration) -> std::io::Result<Ran> {
    // On Unix, give the child its own process group so a timeout can kill everything it started.
    #[cfg(unix)]
    {
        use std::os::unix::process::CommandExt;
        cmd.process_group(0);
    }
    let mut child = cmd.stdin(Stdio::piped()).stdout(Stdio::piped()).stderr(Stdio::piped()).spawn()?;

    let mut input = child.stdin.take().unwrap();
    let stdin = stdin.to_string();
    std::thread::spawn(move || {
        let _ = input.write_all(stdin.as_bytes());
        let _ = input.write_all(b"\n");
    });
    let reader = |pipe: Box<dyn Read + Send>| {
        let (tx, rx) = std::sync::mpsc::channel();
        std::thread::spawn(move || {
            let mut buf = Vec::new();
            let _ = pipe.take(MAX_OUTPUT).read_to_end(&mut buf);
            let _ = tx.send(String::from_utf8_lossy(&buf).into_owned());
        });
        rx
    };
    let out_rx = reader(Box::new(child.stdout.take().unwrap()));
    let err_rx = reader(Box::new(child.stderr.take().unwrap()));

    let start = Instant::now();
    let mut timed_out = false;
    let status = loop {
        if let Some(s) = child.try_wait()? {
            break Some(s);
        }
        if start.elapsed() > timeout {
            kill_tree(&mut child);
            timed_out = true;
            break None;
        }
        std::thread::sleep(Duration::from_millis(15));
    };
    // A grandchild that survived (or inherited the pipes) must not hang the app: wait a little, then give up.
    let collect = |rx: std::sync::mpsc::Receiver<String>| rx.recv_timeout(Duration::from_secs(2)).unwrap_or_default();
    Ok(Ran { stdout: collect(out_rx), stderr: collect(err_rx), exit_code: status.and_then(|s| s.code()), timed_out })
}

/// Kill a process and everything it started (launchers like `py -3` or `cmd /C` start children).
fn kill_tree(child: &mut std::process::Child) {
    #[cfg(windows)]
    {
        let _ = command(Path::new("taskkill")).args(["/PID", &child.id().to_string(), "/T", "/F"]).output();
    }
    #[cfg(unix)]
    {
        // `--` so the negative id is read as a process group, not as an option.
        let _ = Command::new("kill").args(["-s", "KILL", "--", &format!("-{}", child.id())]).output();
    }
    let _ = child.kill();
    let _ = child.wait();
}

// ---------------------------------------------------------------- Rust (compiled)

struct Compiled {
    ok: bool,
    output: String,
    exe: PathBuf,
}

fn compile_rust(rustc: &Tool, dir: &Path, source: &str, test: bool) -> std::io::Result<Compiled> {
    // Always compile a file called main.rs by relative path, so that error and panic
    // messages say `main.rs:line:col` instead of a long temp path.
    std::fs::write(dir.join("main.rs"), source)?;
    let exe = dir.join(if test { "check.exe" } else { "main.exe" });
    let mut cmd = command(&rustc.program);
    cmd.args(["--edition=2024", "--color=never", "main.rs", "-o"]).arg(&exe).current_dir(dir);
    if test {
        cmd.args(["--test", "-A", "warnings"]);
    }
    let out = cmd.output()?;
    Ok(Compiled { ok: out.status.success(), output: String::from_utf8_lossy(&out.stderr).into_owned(), exe })
}

const TODO_MESSAGE: &str = "Your code still has a placeholder (\"not done yet\") in it. Replace it with your own code.";

/// Turn the Rust test harness's failure output into something a beginner can read.
fn friendly_rust_test_failure(stdout: &str) -> String {
    let mut lines = Vec::new();
    for l in stdout.lines().skip_while(|l| !l.starts_with("---- ")).take_while(|l| !l.starts_with("failures:")) {
        if l.trim().is_empty() || l.starts_with("note:") || l.starts_with("---- ") || l.starts_with("thread '") {
            continue;
        }
        let l = if l.contains("not yet implemented") {
            TODO_MESSAGE.to_string()
        } else if let Some(msg) = l.strip_prefix("assertion `left == right` failed: ") {
            msg.to_string()
        } else if l.starts_with("assertion `left == right` failed") {
            "A result was wrong:".to_string()
        } else if let Some(v) = l.strip_prefix("  left: ") {
            format!("  your function returned: {v}")
        } else if let Some(v) = l.strip_prefix(" right: ") {
            format!("  but it should be:       {v}")
        } else {
            l.to_string()
        };
        if !lines.contains(&l) {
            lines.push(l);
        }
    }
    lines.join("\n")
}

fn execute_rust(rustc: &Tool, job: &Job, dir: &Path, res: &mut RunResult) -> std::io::Result<Option<Vec<String>>> {
    // A check with tests needs a second, test-mode build. Start it right away on another
    // thread (in its own folder) so both builds run at the same time.
    let test_build = match job.check.as_ref().and_then(|c| c.tests.as_ref()) {
        Some(tests) => {
            let src = format!(
                "{}\n\n#[cfg(test)]\n#[allow(unused_imports)]\nmod it_quest_checks {{\n    use super::*;\n{}\n}}\n",
                job.code, tests
            );
            let test_dir = dir.join("test");
            std::fs::create_dir_all(&test_dir)?;
            let rustc = rustc.clone();
            Some(std::thread::spawn(move || compile_rust(&rustc, &test_dir, &src, true)))
        }
        None => None,
    };

    let compiled = compile_rust(rustc, dir, &job.code, false)?;
    res.compile_ok = compiled.ok;
    res.compiler_output = compiled.output;
    if !compiled.ok {
        if let Some(build) = test_build {
            let _ = build.join();
        }
        return Ok(job.check.is_some().then(|| vec!["Your code doesn't compile yet. Fix the errors first.".to_string()]));
    }

    let mut cmd = command(&compiled.exe);
    cmd.current_dir(dir);
    let ran = run_process(cmd, &job.stdin, Lang::Rust.timeout())?;
    res.stdout = ran.stdout;
    res.stderr = ran.stderr;
    res.exit_code = ran.exit_code;
    res.timed_out = ran.timed_out;

    let mut problems = Vec::new();
    if let Some(build) = test_build {
        let c = build.join().map_err(|_| std::io::Error::other("the test build crashed"))??;
        if !c.ok {
            problems.push(format!(
                "Your code runs, but the checker couldn't call your functions. Did you keep the names and parameter types from the task?\n\n{}",
                c.output
            ));
        } else {
            let mut cmd = command(&c.exe);
            cmd.args(["--test-threads=1", "-q"]).current_dir(dir);
            let t = run_process(cmd, "", Lang::Rust.timeout())?;
            if t.exit_code != Some(0) {
                problems.push(format!("Some checks failed:\n{}", friendly_rust_test_failure(&t.stdout)));
            }
        }
    }
    Ok(job.check.is_some().then_some(problems))
}

// ---------------------------------------------------------------- scripts (interpreted)

/// The `check(actual, expected, label)` helper each script language gets in test runs.
/// A failed check prints one marker line to stderr: \x01FAIL\x02label\x02got\x02expected
fn check_helper(lang: Lang) -> &'static str {
    match lang {
        Lang::Python => {
            "\n\n# ---- IT Quest checker ----\nimport sys as _iq_sys\ndef check(actual, expected, label=\"\"):\n    if actual != expected:\n        print(\"\\x01FAIL\\x02\" + str(label) + \"\\x02\" + repr(actual) + \"\\x02\" + repr(expected), file=_iq_sys.stderr)\n\n"
        }
        Lang::JavaScript => {
            "\n\n// ---- IT Quest checker ----\nfunction check(actual, expected, label = \"\") {\n  const a = JSON.stringify(actual), e = JSON.stringify(expected);\n  if (a !== e) process.stderr.write(\"\\x01FAIL\\x02\" + label + \"\\x02\" + a + \"\\x02\" + e + \"\\n\");\n}\n\n"
        }
        Lang::PowerShell => {
            "\n\n# ---- IT Quest checker ----\nfunction check($actual, $expected, $label = '') {\n  $a = ConvertTo-Json -Compress -Depth 5 -InputObject $actual\n  $e = ConvertTo-Json -Compress -Depth 5 -InputObject $expected\n  if (-not ($a -ceq $e)) { [Console]::Error.WriteLine([string][char]1 + 'FAIL' + [char]2 + $label + [char]2 + $a + [char]2 + $e) }\n}\n\n"
        }
        Lang::Bash => {
            "\n\n# ---- IT Quest checker ----\ncheck() {\n  if [[ \"$1\" != \"$2\" ]]; then printf '\\001FAIL\\002%s\\002%s\\002%s\\n' \"$3\" \"$1\" \"$2\" >&2; fi\n}\n\n"
        }
        Lang::TypeScript => {
            "\n\n// ---- IT Quest checker ----\nfunction check(actual: unknown, expected: unknown, label = \"\"): void {\n  const a = JSON.stringify(actual), e = JSON.stringify(expected);\n  if (a !== e) process.stderr.write(\"\\x01FAIL\\x02\" + label + \"\\x02\" + a + \"\\x02\" + e + \"\\n\");\n}\n\n"
        }
        _ => "",
    }
}

/// Prints the result of every statement that returns rows, as a small aligned table.
const SQL_HARNESS: &str = r#"import sqlite3, sys
con = sqlite3.connect(":memory:", isolation_level=None)  # autocommit, so BEGIN/COMMIT work as written

def table(cur):
    cols = [d[0] for d in cur.description]
    rows = [["NULL" if v is None else str(v) for v in r] for r in cur.fetchall()]
    widths = [max([len(c)] + [len(r[i]) for r in rows]) for i, c in enumerate(cols)]
    line = lambda vals: " | ".join(v.ljust(w) for v, w in zip(vals, widths)).rstrip()
    print(line(cols))
    print("-+-".join("-" * w for w in widths))
    for r in rows:
        print(line(r))
    if not rows:
        print("(no rows)")
    print()

def run(path, show):
    src = open(path, encoding="utf-8").read()
    buf, line_no, start = "", 0, 1
    for line in src.splitlines(keepends=True):
        line_no += 1
        if not buf.strip():
            start = line_no
        buf += line
        if sqlite3.complete_statement(buf):
            stmt, buf = buf.strip(), ""
            try:
                cur = con.execute(stmt)
            except sqlite3.Error as e:
                print(f"Error in the statement starting on line {start}:\n{stmt}\n\n{e}", file=sys.stderr)
                sys.exit(1)
            if show and cur.description:
                table(cur)
    rest = "\n".join(l for l in buf.splitlines() if not l.strip().startswith("--")).strip()
    if rest:
        print(f"The statement starting on line {start} has no ; at the end, so it didn't run:\n{rest}", file=sys.stderr)
        sys.exit(1)

run("setup.sql", False)
run("main.sql", True)
"#;

fn script_command(tool: &Tool, lang: Lang, dir: &Path, file: &str) -> Command {
    let mut cmd = command(&tool.program);
    cmd.args(&tool.pre_args).current_dir(dir);
    match lang {
        Lang::Python | Lang::Sql => {
            cmd.args(["-X", "utf8", "-B", file]).env("PYTHONIOENCODING", "utf-8").env("PYTHONUNBUFFERED", "1");
        }
        Lang::JavaScript | Lang::TypeScript => {
            cmd.arg(file);
        }
        Lang::PowerShell => {
            // Run the file unchanged (so error line numbers match the editor), with UTF-8 output,
            // and a non-zero exit code for an uncaught terminating error. ($Error isn't used:
            // it also holds errors the script caught itself.)
            let script = format!(
                "[Console]::OutputEncoding = [Text.UTF8Encoding]::new($false); $OutputEncoding = [Console]::OutputEncoding; \
                 try {{ . .\\{file} }} catch {{ Write-Error $_; exit 1 }}"
            );
            cmd.args(["-NoProfile", "-NonInteractive", "-ExecutionPolicy", "Bypass", "-Command", &script]);
        }
        Lang::Bash => {
            cmd.args(["--noprofile", "--norc", file]).env("CHERE_INVOKING", "1").env("MSYS_NO_PATHCONV", "1");
        }
        _ => unreachable!(),
    }
    cmd
}

fn write_script(dir: &Path, lang: Lang, name: &str, code: &str) -> std::io::Result<()> {
    let code = code.replace("\r\n", "\n");
    match lang {
        // Windows PowerShell 5.1 reads files without a BOM in the old ANSI code page.
        Lang::PowerShell => std::fs::write(dir.join(name), format!("\u{feff}{}", code.replace('\n', "\r\n"))),
        _ => std::fs::write(dir.join(name), code),
    }
}

/// Pulls the checker's marker lines out of stderr and turns them into readable problems.
fn take_check_failures(stderr: &mut String) -> Vec<String> {
    let mut fails = Vec::new();
    let mut rest = String::new();
    for line in stderr.lines() {
        match line.split_once("\u{1}FAIL\u{2}") {
            Some((before, marker)) => {
                if !before.trim().is_empty() {
                    rest.push_str(before);
                    rest.push('\n');
                }
                let parts: Vec<&str> = marker.split('\u{2}').collect();
                let (label, got, want) = (parts.first().unwrap_or(&""), parts.get(1).unwrap_or(&""), parts.get(2).unwrap_or(&""));
                let label = if label.is_empty() { "A result was wrong:".to_string() } else { format!("{label}:") };
                fails.push(format!("{label}\n  your code gave:   {got}\n  but it should be: {want}"));
            }
            None => {
                rest.push_str(line);
                rest.push('\n');
            }
        }
    }
    *stderr = rest;
    fails
}

fn execute_script(tool: &Tool, job: &Job, dir: &Path, res: &mut RunResult) -> std::io::Result<Option<Vec<String>>> {
    let lang = job.lang;
    res.compile_ok = true;
    let file = lang.file_name();
    let setup = job.setup.as_str();
    // For scripts the setup simply runs first, in the same file. SQL has its own setup file.
    let prefix = if setup.is_empty() { String::new() } else { format!("{setup}\n") };
    let ran = if lang == Lang::Sql {
        write_script(dir, lang, "main.sql", &job.code)?;
        write_script(dir, lang, "setup.sql", setup)?;
        write_script(dir, Lang::Python, "run_sql.py", SQL_HARNESS)?;
        run_process(script_command(tool, lang, dir, "run_sql.py"), &job.stdin, lang.timeout())?
    } else {
        write_script(dir, lang, file, &format!("{prefix}{}", job.code))?;
        run_process(script_command(tool, lang, dir, file), &job.stdin, lang.timeout())?
    };
    res.stdout = ran.stdout;
    res.stderr = ran.stderr;
    res.exit_code = ran.exit_code;
    res.timed_out = ran.timed_out;
    let Some(check) = &job.check else { return Ok(None) };

    let mut problems = Vec::new();
    // SQL exercises are checked by their output only.
    if let Some(tests) = check.tests.as_ref().filter(|_| lang != Lang::Sql) {
        // Checks run in a second, separate run: the code, the check() helper, then the checks.
        let test_dir = dir.join("test");
        std::fs::create_dir_all(&test_dir)?;
        let src = format!("{prefix}{}{}{}\n", job.code, check_helper(lang), tests);
        write_script(&test_dir, lang, file, &src)?;
        let t = run_process(script_command(tool, lang, &test_dir, file), &job.stdin, lang.timeout())?;
        let mut stderr = t.stderr;
        let fails = take_check_failures(&mut stderr);
        if t.timed_out {
            problems.push("The checks never finished (more than a few seconds). Is there an endless loop?".to_string());
        } else if stderr.contains(lang.todo_marker()) {
            problems.push(TODO_MESSAGE.to_string());
        } else if t.exit_code != Some(0) && fails.is_empty() {
            problems.push(format!("Your code raised an error while being checked:\n{}", crate::ai::shorten(&stderr, 1500)));
        }
        if !fails.is_empty() {
            problems.push(format!("Some checks failed:\n{}", fails.join("\n")));
        }
    }
    Ok(Some(problems))
}

// ---------------------------------------------------------------- Go, C#, C++ (compiled)

const COMPILE_TIMEOUT: Duration = Duration::from_secs(60);

/// What .NET's implicit usings would give a new console project.
const CSHARP_USINGS: &str = "global using System;\nglobal using System.Collections.Generic;\nglobal using System.IO;\nglobal using System.Linq;\nglobal using System.Net.Http;\nglobal using System.Threading;\nglobal using System.Threading.Tasks;\n";

fn execute_compiled(tool: &Tool, job: &Job, dir: &Path, res: &mut RunResult) -> std::io::Result<Option<Vec<String>>> {
    let exe = dir.join(if cfg!(windows) { "main.exe" } else { "main" });
    let file = job.lang.file_name();
    std::fs::write(dir.join(file), job.code.replace("\r\n", "\n"))?;

    let mut compile = command(&tool.program);
    compile.current_dir(dir).envs(tool.env.iter().map(|(k, v)| (k, v)));
    let mut run = command(&exe);
    match job.lang {
        Lang::Go => {
            compile.args(["build", "-o"]).arg(&exe).arg(file).env("GOTOOLCHAIN", "local").env("GO111MODULE", "off");
        }
        Lang::CSharp => {
            std::fs::write(dir.join("usings.cs"), CSHARP_USINGS)?;
            std::fs::write(dir.join("refs.rsp"), &tool.extra[2])?;
            let major = tool.extra[1].split('.').next().unwrap_or("8");
            std::fs::write(
                dir.join("main.runtimeconfig.json"),
                format!(r#"{{"runtimeOptions":{{"tfm":"net{major}.0","framework":{{"name":"Microsoft.NETCore.App","version":"{}"}}}}}}"#, tool.extra[1]),
            )?;
            compile.arg(&tool.extra[0]).args([
                "-nologo",
                "-nostdlib",
                "-noconfig",
                "-langversion:latest",
                "-nullable:enable",
                "-out:main.dll",
                "@refs.rsp",
                "usings.cs",
                file,
            ]);
            run = command(&tool.program);
            // Same number and date formats on every PC (1,234.5 rather than 1 234,5).
            run.arg("main.dll").env("DOTNET_SYSTEM_GLOBALIZATION_INVARIANT", "1");
        }
        Lang::Cpp if tool.program.file_name().is_some_and(|n| n.eq_ignore_ascii_case("cl.exe")) => {
            compile.args(["/nologo", "/EHsc", "/std:c++20", "/utf-8", "/W3", file, "/Fe:main.exe"]);
        }
        Lang::Cpp => {
            compile.args(["-std=c++20", "-Wall", "-O1", file, "-o"]).arg(&exe);
        }
        _ => unreachable!(),
    }
    let built = run_process(compile, "", COMPILE_TIMEOUT)?;
    // csc and cl print errors on stdout; Go and g++ on stderr. Drop MSVC's echo of the file name.
    let output = format!("{}{}", built.stdout, built.stderr);
    res.compiler_output = output.lines().filter(|l| l.trim() != file).collect::<Vec<_>>().join("\n");
    res.compile_ok = built.exit_code == Some(0);
    if !res.compile_ok {
        if built.timed_out {
            res.compiler_output.push_str("\n(The compiler took too long and was stopped.)");
        }
        return Ok(job.check.is_some().then(|| vec!["Your code doesn't compile yet. Fix the errors first.".to_string()]));
    }
    run.current_dir(dir).envs(tool.env.iter().map(|(k, v)| (k, v)));
    let ran = run_process(run, &job.stdin, job.lang.timeout())?;
    res.stdout = ran.stdout;
    res.stderr = ran.stderr;
    res.exit_code = ran.exit_code;
    res.timed_out = ran.timed_out;
    // Compiled languages are checked by their output (and required snippets) only.
    Ok(job.check.as_ref().map(|_| Vec::new()))
}

// ---------------------------------------------------------------- HTML (previewed by the app)

/// Elements that never have a closing tag.
const VOID_TAGS: [&str; 14] = ["area", "base", "br", "col", "embed", "hr", "img", "input", "link", "meta", "source", "track", "wbr", "!doctype"];

/// Nothing to run: the app shows a live preview. Here we only look for unclosed or mismatched
/// tags, the mistake beginners make most, and report them on stderr like a linter would.
fn execute_html(job: &Job, res: &mut RunResult) -> Option<Vec<String>> {
    res.compile_ok = true;
    res.exit_code = Some(0);
    let problems = html_tag_problems(&job.code);
    res.stderr = problems.join("\n");
    res.stdout = "(see the preview)".into();
    job.check.as_ref().map(|_| problems)
}

pub fn html_tag_problems(src: &str) -> Vec<String> {
    let mut problems = Vec::new();
    let mut open: Vec<(String, usize)> = Vec::new();
    let line_of = |pos: usize| src[..pos].matches('\n').count() + 1;
    let lower = src.to_ascii_lowercase();
    let mut i = 0;
    while let Some(rel) = src[i..].find('<') {
        let start = i + rel;
        if lower[start..].starts_with("<!--") {
            i = lower[start..].find("-->").map_or(src.len(), |e| start + e + 3);
            continue;
        }
        let Some(end_rel) = src[start..].find('>') else { break };
        let end = start + end_rel;
        let inner = &src[start + 1..end];
        i = end + 1;
        let closing = inner.starts_with('/');
        let name: String = inner
            .trim_start_matches('/')
            .chars()
            .take_while(|c| c.is_ascii_alphanumeric() || *c == '!' || *c == '-')
            .collect::<String>()
            .to_ascii_lowercase();
        if name.is_empty() {
            continue;
        }
        // Skip the contents of <script> and <style>: they aren't HTML.
        if !closing && (name == "script" || name == "style") {
            let close = format!("</{name}");
            match lower[i..].find(&close) {
                Some(e) => i = lower[i + e..].find('>').map_or(src.len(), |g| i + e + g + 1),
                None => {
                    problems.push(format!("<{name}> on line {} is never closed.", line_of(start)));
                    i = src.len();
                }
            }
            continue;
        }
        if VOID_TAGS.contains(&name.as_str()) || inner.ends_with('/') {
            continue;
        }
        if !closing {
            open.push((name, line_of(start)));
        } else if let Some(pos) = open.iter().rposition(|(n, _)| *n == name) {
            for (n, line) in open.drain(pos + 1..).rev() {
                if !["p", "li", "td", "th", "tr", "option"].contains(&n.as_str()) {
                    problems.push(format!("<{n}> on line {line} is never closed (found </{name}> on line {} first).", line_of(start)));
                }
            }
            open.pop();
        } else {
            problems.push(format!("</{name}> on line {} closes a tag that isn't open.", line_of(start)));
        }
    }
    for (n, line) in open {
        if !["html", "head", "body", "p", "li", "td", "th", "tr", "option"].contains(&n.as_str()) {
            problems.push(format!("<{n}> on line {line} is never closed."));
        }
    }
    problems
}

pub fn execute(job: &Job) -> RunResult {
    let tool = match find_tool(job.lang) {
        Ok(t) => t,
        Err(e) => return RunResult { internal_error: Some(e), ..Default::default() },
    };
    match execute_inner(&tool, job) {
        Ok(r) => r,
        Err(e) => RunResult { internal_error: Some(e.to_string()), ..Default::default() },
    }
}

fn execute_inner(tool: &Tool, job: &Job) -> std::io::Result<RunResult> {
    let dir = work_dir()?;
    let mut res = RunResult::default();
    let problems = if job.lang == Lang::Rust {
        execute_rust(tool, job, &dir, &mut res)
    } else if job.lang.compiled() {
        execute_compiled(tool, job, &dir, &mut res)
    } else if job.lang == Lang::Html {
        Ok(execute_html(job, &mut res))
    } else {
        execute_script(tool, job, &dir, &mut res)
    };
    let _ = std::fs::remove_dir_all(&dir);
    let Some(mut problems) = problems? else { return Ok(res) };
    let check = job.check.as_ref().unwrap();

    // Problems every language shares, put before the language-specific ones.
    let mut first = Vec::new();
    if res.compile_ok {
        if res.timed_out {
            first.push("Your program never finished (it ran for more than a few seconds). Is there an endless loop, or is it waiting for input?".to_string());
        } else if res.stderr.contains(job.lang.todo_marker()) {
            first.push(TODO_MESSAGE.to_string());
        } else if res.exit_code != Some(0) {
            first.push(if job.lang == Lang::Rust {
                "Your program crashed (panicked) instead of finishing normally.".to_string()
            } else if job.lang.compiled() {
                "Your program crashed or exited with an error instead of finishing normally.".to_string()
            } else {
                "Your program stopped with an error instead of finishing normally. Look at the error output.".to_string()
            });
        }
        if let Some(expected) = &check.expected
            && normalize(&res.stdout) != normalize(expected)
        {
            first.push(format!(
                "The output doesn't match yet.\n\nExpected:\n{}\n\nYour program printed:\n{}",
                normalize(expected),
                if res.stdout.trim().is_empty() { "(nothing)".into() } else { normalize(&res.stdout) }
            ));
        }
        // HTML and CSS don't care about spacing (`<li> <a>` = `<li><a>`), so neither does the check.
        let squash = |s: &str| s.chars().filter(|c| !c.is_whitespace()).collect::<String>();
        let code = if job.lang == Lang::Html { squash(&job.code) } else { job.code.clone() };
        for r in &check.requires {
            let found = if job.lang == Lang::Html { code.contains(&squash(r)) } else { code.contains(r.as_str()) };
            if !found {
                first.push(format!("This exercise wants you to use `{r}` in your code."));
            }
        }
    }
    // Don't say "there's a placeholder" twice.
    if first.iter().any(|p| p == TODO_MESSAGE) {
        problems.retain(|p| p != TODO_MESSAGE && !p.ends_with(TODO_MESSAGE));
    }
    first.extend(problems);
    res.check = Some(CheckResult { passed: first.is_empty(), problems: first });
    Ok(res)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn timeout_kills_the_whole_tree_and_returns() {
        let Ok(py) = find_tool(Lang::Python) else { return };
        // The child starts a grandchild that would keep the pipes open for 30 s.
        let mut cmd = command(&py.program);
        cmd.args(&py.pre_args).args([
            "-c",
            "import subprocess, sys, time; subprocess.Popen([sys.executable, '-c', 'import time; time.sleep(30)']); time.sleep(30)",
        ]);
        let start = Instant::now();
        let ran = run_process(cmd, "", Duration::from_secs(1)).unwrap();
        assert!(ran.timed_out);
        assert!(start.elapsed() < Duration::from_secs(8), "took {:?}", start.elapsed());
    }
}
