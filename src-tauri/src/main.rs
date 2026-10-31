#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod ai;
mod commands;
mod course;
mod progress;
mod projects;
mod runner;
mod tracks;

fn main() {
    if std::env::args().any(|a| a == "--verify") {
        verify_course();
        return;
    }
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            commands::get_course,
            commands::tool_status,
            commands::run_code,
            commands::load_progress,
            commands::save_progress,
            commands::find_rust_quest_progress,
            commands::project_status,
            commands::project_action,
            commands::project_sources,
            commands::ai_models,
            commands::ai_chat,
            commands::ai_stop,
            commands::open_url,
        ])
        .run(tauri::generate_context!())
        .expect("error while running IT Quest");
}

/// Development helper (`cargo run -- --verify`): every solution must pass its own check,
/// every starter must not (so exercises can't be passed by doing nothing), and runnable
/// examples should run. `--only python/` limits it to lessons whose id starts with that,
/// `--projects` also builds or syntax-checks every project card, `-v` prints starter feedback.
fn verify_course() {
    use course::{Block, Segment};
    let args: Vec<String> = std::env::args().collect();
    let only = args.iter().position(|a| a == "--only").and_then(|i| args.get(i + 1)).cloned().unwrap_or_default();
    let verbose = args.iter().any(|a| a == "-v");
    let check_projects = args.iter().any(|a| a == "--projects");
    let project_root = std::env::temp_dir().join("it_quest_verify");
    let mut failures = 0;
    let mut checks = 0;
    let mut ids = std::collections::HashSet::new();

    for track in commands::course() {
        for module in &track.modules {
            for lesson in &module.lessons {
                if !ids.insert(lesson.id.clone()) {
                    failures += 1;
                    println!("[FAIL] duplicate lesson id {}", lesson.id);
                }
            }
        }
    }
    for lang in runner::Lang::ALL {
        match runner::find_tool(lang) {
            Ok(t) => println!("{:<11} {}", lang.name(), t.version),
            Err(e) => println!("{:<11} MISSING: {e}", lang.name()),
        }
    }

    for track in commands::course() {
        for module in &track.modules {
            for lesson in module.lessons.iter().filter(|l| l.id.starts_with(&only)) {
                let lang = lesson.lang.or(track.lang);
                for (bi, block) in lesson.blocks.iter().enumerate() {
                    match block {
                        Block::Quiz(q) if q.options.len() < 2 || q.correct >= q.options.len() => {
                            failures += 1;
                            println!("[FAIL] {} quiz {bi} needs at least two options and one * answer", lesson.id);
                        }
                        Block::Drill(d) if d.answers.is_empty() => {
                            failures += 1;
                            println!("[FAIL] {} drill {bi} has no = answer", lesson.id);
                        }
                        Block::Flashcards { cards } if cards.is_empty() => {
                            failures += 1;
                            println!("[FAIL] {} flashcards {bi} has no `front :: back` lines", lesson.id);
                        }
                        Block::Project(p) if check_projects => {
                            checks += 1;
                            let dir = project_root.join(lesson.id.replace('/', "__")).join(&p.folder);
                            let _ = std::fs::remove_dir_all(&dir);
                            for (path, contents) in &p.files {
                                let target = dir.join(path);
                                std::fs::create_dir_all(target.parent().unwrap()).unwrap();
                                std::fs::write(target, contents).unwrap();
                            }
                            let (ok, output) = verify_project(p, &dir, &project_root);
                            println!("[{}] {} project {}", if ok { "ok" } else { "FAIL" }, lesson.id, p.folder);
                            if !ok {
                                failures += 1;
                                println!("{output}");
                            }
                        }
                        _ => {}
                    }
                }
                // Runnable examples should run, apart from ones that demonstrate errors.
                let segments = lesson.blocks.iter().filter_map(|b| match b {
                    Block::Text { segments } => Some(segments),
                    _ => None,
                });
                for seg in segments.flatten().chain(&lesson.outro) {
                    if let Segment::Code { code, lang, runnable: true } = seg {
                        let setup = if Some(*lang) == lesson.lang.or(track.lang) { lesson.setup.clone() } else { String::new() };
                        let r = runner::execute(&runner::Job { lang: *lang, code: code.clone(), stdin: String::new(), setup, check: None });
                        if let Some(e) = r.internal_error {
                            println!("[note] {} example couldn't run: {e}", lesson.id);
                        } else if !r.compile_ok || r.exit_code != Some(0) {
                            let first = code.lines().find(|l| !l.trim().is_empty()).unwrap_or("").trim();
                            let reason = if r.compile_ok { "stops with an error" } else { "doesn't compile" };
                            println!("[note] {} {} example {reason} (intended?): {first}", lesson.id, lang.name());
                        }
                    }
                }
                for (i, ex) in lesson.exercises.iter().enumerate() {
                    checks += 1;
                    let Some(lang) = lang else {
                        failures += 1;
                        println!("[FAIL] {} #{} exercise in a track without a language (add @@lang)", lesson.id, i + 1);
                        continue;
                    };
                    if runner::find_tool(lang).is_err() {
                        println!("[skip] {} #{} {} ({} isn't installed here)", lesson.id, i + 1, ex.title, lang.name());
                        continue;
                    }
                    let check = runner::Check {
                        expected: ex.expected.clone(),
                        tests: ex.tests.clone(),
                        requires: ex.requires.clone(),
                    };
                    let run = |code: &str| {
                        runner::execute(&runner::Job {
                            lang,
                            code: code.to_string(),
                            stdin: ex.stdin.clone(),
                            setup: ex.setup.clone(),
                            check: Some(check.clone()),
                        })
                    };
                    let sol = run(&ex.solution);
                    let starter = run(&ex.starter);
                    let passed = |r: &runner::RunResult| r.check.as_ref().is_some_and(|c| c.passed);
                    let status = if passed(&sol) && !passed(&starter) { "ok" } else { "FAIL" };
                    println!("[{status}] {} #{} {}", lesson.id, i + 1, ex.title);
                    if !passed(&sol) {
                        failures += 1;
                        let problems = sol.check.as_ref().map(|c| c.problems.join("\n")).unwrap_or_default();
                        println!(
                            "  solution failed: {}{problems}\n{}\n{}\n{}",
                            sol.internal_error.unwrap_or_default(),
                            sol.compiler_output,
                            sol.stdout,
                            sol.stderr
                        );
                    }
                    if passed(&starter) {
                        failures += 1;
                        println!("  starter code already passes!");
                    } else if verbose && let Some(c) = &starter.check {
                        println!("  starter feedback: {}", c.problems.join("\n  ").replace('\n', "\n  "));
                    }
                }
            }
        }
    }
    println!("{checks} checks, {failures} problem(s)");
    if failures > 0 {
        std::process::exit(1);
    }
}

/// Rust projects must build with their tests; script projects must at least parse.
fn verify_project(p: &course::Project, dir: &std::path::Path, root: &std::path::Path) -> (bool, String) {
    use runner::Lang;
    let out = |cmd: &mut std::process::Command| match cmd.output() {
        Ok(o) => (o.status.success(), format!("{}{}", String::from_utf8_lossy(&o.stdout), String::from_utf8_lossy(&o.stderr))),
        Err(e) => (false, e.to_string()),
    };
    match projects::lang_of(p) {
        Some(Lang::Rust) => out(std::process::Command::new("cargo")
            .args(["test", "--no-run", "--quiet"])
            .current_dir(dir)
            .env("CARGO_TARGET_DIR", root.join("target"))),
        Some(Lang::Python) | Some(Lang::Sql) => {
            let Ok(py) = runner::find_tool(Lang::Python) else { return (false, "python missing".into()) };
            let compiled = out(std::process::Command::new(&py.program).args(&py.pre_args).args(["-m", "compileall", "-q", "."]).current_dir(dir));
            // A finished version must also pass its own tests.
            if compiled.0 && p.folder.ends_with("_finished") {
                return out(std::process::Command::new(&py.program).args(&py.pre_args).args(["-m", "unittest", "-q"]).current_dir(dir));
            }
            compiled
        }
        Some(Lang::JavaScript) => {
            let mut all = (true, String::new());
            for (path, _) in p.files.iter().filter(|(f, _)| f.ends_with(".js")) {
                let r = out(std::process::Command::new("node").args(["--check", path]).current_dir(dir));
                all = (all.0 && r.0, all.1 + &r.1);
            }
            if all.0 && p.folder.ends_with("_finished") && p.files.iter().any(|(f, _)| f.ends_with(".test.js")) {
                return out(std::process::Command::new("node").arg("--test").current_dir(dir));
            }
            all
        }
        Some(Lang::PowerShell) => {
            let script = "$bad = 0; Get-ChildItem -Recurse -Filter *.ps1 | ForEach-Object { $e = $null; \
                [void][System.Management.Automation.Language.Parser]::ParseFile($_.FullName, [ref]$null, [ref]$e); \
                foreach ($x in $e) { Write-Output \"$($_.Name): $($x.Message)\"; $bad++ } }; exit $bad";
            let shell = if cfg!(windows) { "powershell" } else { "pwsh" };
            let parsed = out(std::process::Command::new(shell).args(["-NoProfile", "-Command", script]).current_dir(dir));
            if parsed.0 && p.folder.ends_with("_finished") && dir.join("tests.ps1").exists() {
                return out(std::process::Command::new(shell)
                    .args(["-NoProfile", "-ExecutionPolicy", "Bypass", "-File", "tests.ps1"])
                    .current_dir(dir));
            }
            parsed
        }
        Some(Lang::Bash) => {
            let Ok(bash) = runner::find_tool(Lang::Bash) else { return (false, "bash missing".into()) };
            let mut all = (true, String::new());
            for (path, _) in p.files.iter().filter(|(f, _)| f.ends_with(".sh")) {
                let r = out(std::process::Command::new(&bash.program).args(["-n", path]).current_dir(dir));
                all = (all.0 && r.0, all.1 + &r.1);
            }
            if all.0 && p.folder.ends_with("_finished") && dir.join("tests.sh").exists() {
                return out(std::process::Command::new(&bash.program).arg("tests.sh").current_dir(dir));
            }
            all
        }
        Some(Lang::TypeScript) => {
            // Node can't --check a .ts file, so strip the types and check the JavaScript that's left.
            let strip = "const m = require('node:module'), fs = require('node:fs');\
                for (const f of process.argv.slice(1)) fs.writeFileSync(f + '.check.mjs', m.stripTypeScriptTypes(fs.readFileSync(f, 'utf8')));";
            let ts: Vec<&String> = p.files.iter().map(|(f, _)| f).filter(|f| f.ends_with(".ts")).collect();
            let mut all = out(std::process::Command::new("node").args(["-e", strip, "--"]).args(&ts).current_dir(dir));
            for f in &ts {
                let r = out(std::process::Command::new("node").args(["--check", &format!("{f}.check.mjs")]).current_dir(dir));
                all = (all.0 && r.0, all.1 + &r.1);
                let _ = std::fs::remove_file(dir.join(format!("{f}.check.mjs")));
            }
            if all.0 && p.folder.ends_with("_finished") && ts.iter().any(|f| f.ends_with(".test.ts")) {
                return out(std::process::Command::new("node").arg("--test").current_dir(dir));
            }
            all
        }
        Some(Lang::Go) => {
            let Ok(go) = runner::find_tool(Lang::Go) else { return skipped("Go") };
            let vetted = out(std::process::Command::new(&go.program).args(["vet", "./..."]).current_dir(dir));
            if vetted.0 && p.folder.ends_with("_finished") {
                return out(std::process::Command::new(&go.program).args(["test", "./..."]).current_dir(dir));
            }
            vetted
        }
        Some(Lang::CSharp) => {
            let Ok(dotnet) = runner::find_tool(Lang::CSharp) else { return skipped("the .NET SDK") };
            let built = out(std::process::Command::new(&dotnet.program).args(["build", "-nologo", "-v", "q"]).current_dir(dir));
            if built.0 && p.folder.ends_with("_finished") {
                return out(std::process::Command::new(&dotnet.program).args(["run", "--no-build", "--", "--test"]).current_dir(dir));
            }
            built
        }
        Some(Lang::Cpp) => {
            let Ok(cpp) = runner::find_tool(Lang::Cpp) else { return skipped("a C++ compiler") };
            let msvc = cpp.program.file_name().is_some_and(|n| n.eq_ignore_ascii_case("cl.exe"));
            let mut all = (true, String::new());
            for file in ["main.cpp", "tests.cpp"].into_iter().filter(|f| dir.join(f).exists()) {
                let exe = file.replace(".cpp", if cfg!(windows) { ".exe" } else { "" });
                let mut cmd = std::process::Command::new(&cpp.program);
                cmd.current_dir(dir).envs(cpp.env.iter().map(|(k, v)| (k, v)));
                if msvc {
                    cmd.args(["/nologo", "/EHsc", "/std:c++20", "/utf-8", "/W3", file, &format!("/Fe:{exe}")]);
                } else {
                    cmd.args(["-std=c++20", "-Wall", "-o", &exe, file]);
                }
                let mut r = out(&mut cmd);
                if r.0 && file == "tests.cpp" && p.folder.ends_with("_finished") {
                    r = out(std::process::Command::new(dir.join(&exe)).current_dir(dir));
                }
                all = (all.0 && r.0, all.1 + &r.1);
            }
            all
        }
        Some(Lang::Html) => {
            let mut all = (true, String::new());
            for (path, contents) in p.files.iter().filter(|(f, _)| f.ends_with(".html")) {
                let problems = runner::html_tag_problems(contents);
                if !problems.is_empty() {
                    all = (false, all.1 + &format!("{path}:\n{}\n", problems.join("\n")));
                }
            }
            all
        }
        None => (true, String::new()),
    }
}

/// A project whose language isn't installed here: say so, but don't count it as a failure.
fn skipped(what: &str) -> (bool, String) {
    println!("[note] {what} isn't installed, so this project was only written out, not built");
    (true, String::new())
}
