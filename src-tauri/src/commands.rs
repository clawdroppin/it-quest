//! The commands the web UI calls through Tauri's `invoke`.

use crate::course::{Block, CONCEPTS, Project, Track};
use crate::{ai, progress, projects, runner};
use serde::Serialize;
use serde_json::Value;
use std::collections::HashMap;
use std::sync::atomic::{AtomicBool, AtomicU32, Ordering};
use std::sync::{Arc, Mutex, OnceLock};
use tauri::ipc::Channel;

pub fn course() -> &'static Vec<Track> {
    static COURSE: OnceLock<Vec<Track>> = OnceLock::new();
    COURSE.get_or_init(crate::course::load_course)
}

#[derive(Serialize)]
pub struct CourseData {
    tracks: &'static Vec<Track>,
    concepts: Vec<(&'static str, &'static str)>,
    projects_root: String,
}

#[tauri::command]
pub fn get_course() -> CourseData {
    CourseData { tracks: course(), concepts: CONCEPTS.to_vec(), projects_root: projects::root().display().to_string() }
}

#[derive(Serialize)]
pub struct ToolStatus {
    lang: runner::Lang,
    name: &'static str,
    ok: bool,
    /// The version when found, or what to install when not.
    info: String,
}

#[tauri::command]
pub async fn tool_status() -> Vec<ToolStatus> {
    tauri::async_runtime::spawn_blocking(|| {
        let handles: Vec<_> = runner::Lang::ALL.into_iter().map(|l| (l, std::thread::spawn(move || runner::find_tool(l)))).collect();
        handles
            .into_iter()
            .map(|(lang, h)| {
                let found = h.join().unwrap_or_else(|_| Err("check crashed".into()));
                ToolStatus {
                    lang,
                    name: lang.name(),
                    ok: found.is_ok(),
                    info: match found {
                        Ok(t) => t.version,
                        Err(e) => e,
                    },
                }
            })
            .collect()
    })
    .await
    .unwrap_or_default()
}

#[tauri::command]
pub async fn run_code(job: runner::Job) -> runner::RunResult {
    tauri::async_runtime::spawn_blocking(move || runner::execute(&job))
        .await
        .unwrap_or_else(|e| runner::RunResult { internal_error: Some(e.to_string()), ..Default::default() })
}

// ---------------------------------------------------------------- progress

#[tauri::command]
pub fn load_progress() -> Option<Value> {
    progress::load()
}

#[tauri::command]
pub fn save_progress(progress: Value) -> Result<(), String> {
    progress::save(&progress)
}

#[derive(Serialize)]
pub struct OldProgress {
    path: String,
    data: Value,
}

#[tauri::command]
pub fn find_rust_quest_progress() -> Vec<OldProgress> {
    progress::find_rust_quest().into_iter().map(|(path, data)| OldProgress { path, data }).collect()
}

// ---------------------------------------------------------------- projects

/// The project card at `block` in lesson `lesson_id`, and the track it belongs to.
fn find_project(lesson_id: &str, block: usize) -> Result<(&'static str, &'static Project), String> {
    for track in course() {
        for module in &track.modules {
            if let Some(lesson) = module.lessons.iter().find(|l| l.id == lesson_id)
                && let Some(Block::Project(p)) = lesson.blocks.get(block)
            {
                return Ok((track.id, p));
            }
        }
    }
    Err(format!("no project at {lesson_id} block {block}"))
}

#[derive(Serialize)]
pub struct ProjectStatus {
    dir: String,
    existing: usize,
    total: usize,
    commands: Vec<projects::ProjectCommand>,
}

#[tauri::command]
pub fn project_status(lesson_id: String, block: usize) -> Result<ProjectStatus, String> {
    let (track, p) = find_project(&lesson_id, block)?;
    Ok(ProjectStatus {
        dir: projects::dir_of(track, p).display().to_string(),
        existing: projects::existing_files(track, p),
        total: p.files.len(),
        commands: projects::commands(p),
    })
}

/// `action`: create, reset, vscode, folder, or run (with `command`).
#[tauri::command]
pub fn project_action(lesson_id: String, block: usize, action: String, command: Option<String>) -> Result<String, String> {
    let (track, p) = find_project(&lesson_id, block)?;
    let dir = projects::dir_of(track, p);
    let err = |what: &str, e: std::io::Error| format!("Couldn't {what}: {e}");
    match action.as_str() {
        "create" => projects::create_missing(track, p)
            .map(|n| format!("Created {n} file(s). Your existing files were not touched."))
            .map_err(|e| err("create the files", e)),
        "reset" => projects::reset(track, p)
            .map(|b| format!("Reset done. Your previous files were backed up to {}", b.display()))
            .map_err(|e| err("reset the project", e)),
        "vscode" => projects::open_in_vscode(&dir).map(|_| String::new()).map_err(|e| err("start VS Code", e)),
        "folder" => projects::open_folder(&dir).map(|_| String::new()).map_err(|e| err("open the folder", e)),
        "run" => {
            // Only the card's own commands, never an arbitrary command line from the UI.
            let command = command.unwrap_or_default();
            if !projects::commands(p).iter().any(|c| c.command == command) {
                return Err("unknown command".into());
            }
            projects::run_in_terminal(&dir, &command).map(|_| String::new()).map_err(|e| err("open a terminal", e))
        }
        other => Err(format!("unknown action {other}")),
    }
}

#[tauri::command]
pub fn project_sources(lesson_id: String, block: usize) -> Result<String, String> {
    let (track, p) = find_project(&lesson_id, block)?;
    Ok(if projects::existing_files(track, p) > 0 { projects::read_sources(track, p, 12_000) } else { String::new() })
}

/// The AI helper writes a project file (`contents: None` deletes it); returns the old contents.
#[tauri::command]
pub fn project_write(lesson_id: String, block: usize, path: String, contents: Option<String>) -> Result<Option<String>, String> {
    let (track, p) = find_project(&lesson_id, block)?;
    projects::write_source(track, p, &path, contents.as_deref())
}

// ---------------------------------------------------------------- AI helper

#[tauri::command]
pub async fn ai_models(base_url: String) -> Result<Vec<String>, String> {
    tauri::async_runtime::spawn_blocking(move || ai::list_models(&base_url)).await.map_err(|e| e.to_string())?
}

fn streams() -> &'static Mutex<HashMap<u32, Arc<AtomicBool>>> {
    static STREAMS: OnceLock<Mutex<HashMap<u32, Arc<AtomicBool>>>> = OnceLock::new();
    STREAMS.get_or_init(Default::default)
}

/// Starts streaming an answer on `channel` and returns an id for `ai_stop`.
#[tauri::command]
pub fn ai_chat(settings: ai::AiSettings, subject: String, messages: Vec<ai::Message>, channel: Channel<ai::Event>) -> u32 {
    static NEXT: AtomicU32 = AtomicU32::new(1);
    let id = NEXT.fetch_add(1, Ordering::Relaxed);
    let stop = Arc::new(AtomicBool::new(false));
    streams().lock().unwrap().insert(id, stop.clone());
    std::thread::spawn(move || {
        ai::chat(settings, subject, messages, stop, channel);
        streams().lock().unwrap().remove(&id);
    });
    id
}

#[tauri::command]
pub fn ai_stop(id: u32) {
    if let Some(stop) = streams().lock().unwrap().get(&id) {
        stop.store(true, Ordering::Relaxed);
    }
}

#[tauri::command]
pub fn open_url(url: String) -> Result<(), String> {
    // Links in lessons open in the learner's browser; only web links.
    if !(url.starts_with("https://") || url.starts_with("http://")) {
        return Err("not a web link".into());
    }
    projects::open_target(std::ffi::OsStr::new(&url)).map_err(|e| e.to_string())
}
