//! Learner progress, saved as JSON next to the exe so the app stays portable.
//! The UI owns its shape; this side only loads, saves and finds old Rust Quest progress.

use serde_json::Value;
use std::path::PathBuf;

const FILE_NAME: &str = "it_quest_progress.json";

/// The folder the app "lives" in. For a Linux AppImage the exe runs from a read-only mount,
/// so that's the folder holding the .AppImage file (the APPIMAGE variable) instead.
pub fn exe_dir() -> Option<PathBuf> {
    if let Some(image) = std::env::var_os("APPIMAGE") {
        return PathBuf::from(image).parent().map(PathBuf::from);
    }
    std::env::current_exe().ok()?.parent().map(PathBuf::from)
}

/// Next to the exe if that folder is writable, otherwise in %APPDATA% (Windows)
/// or $XDG_DATA_HOME, by default ~/.local/share (Linux).
fn candidate_paths() -> Vec<PathBuf> {
    let mut v = Vec::new();
    if let Some(dir) = exe_dir() {
        v.push(dir.join(FILE_NAME));
    }
    if let Some(appdata) = std::env::var_os("APPDATA") {
        v.push(PathBuf::from(appdata).join("ITQuest").join(FILE_NAME));
    } else if let Some(data) = std::env::var_os("XDG_DATA_HOME")
        .map(PathBuf::from)
        .or_else(|| std::env::var_os("HOME").map(|h| PathBuf::from(h).join(".local").join("share")))
    {
        v.push(data.join("ITQuest").join(FILE_NAME));
    }
    v
}

pub fn load() -> Option<Value> {
    for path in candidate_paths() {
        let Ok(text) = std::fs::read_to_string(&path) else { continue };
        match serde_json::from_str(&text) {
            Ok(progress) => return Some(progress),
            Err(_) => {
                // Never silently overwrite a damaged progress file: keep a copy.
                let _ = std::fs::copy(&path, path.with_extension("damaged.json"));
                break;
            }
        }
    }
    None
}

/// Writes to a temporary file and renames it, so a crash mid-save can't corrupt progress.
pub fn save(progress: &Value) -> Result<(), String> {
    let json = serde_json::to_string_pretty(progress).map_err(|e| e.to_string())?;
    let mut last_err = String::from("no place to save progress");
    for p in candidate_paths() {
        if let Some(dir) = p.parent() {
            let _ = std::fs::create_dir_all(dir);
        }
        let temp = p.with_extension("json.tmp");
        match std::fs::write(&temp, &json).and_then(|_| std::fs::rename(&temp, &p)) {
            Ok(()) => return Ok(()),
            Err(e) => last_err = e.to_string(),
        }
        let _ = std::fs::remove_file(&temp);
    }
    Err(last_err)
}

/// Rust Quest progress files on this computer, newest first: (path, contents).
pub fn find_rust_quest() -> Vec<(String, Value)> {
    const OLD: &str = "rust_quest_progress.json";
    let mut paths = Vec::new();
    if let Some(dir) = exe_dir() {
        paths.push(dir.join(OLD));
    }
    if let Some(appdata) = std::env::var_os("APPDATA") {
        paths.push(PathBuf::from(appdata).join("RustQuest").join(OLD));
    }
    if let Some(home) = std::env::var_os("USERPROFILE").or_else(|| std::env::var_os("HOME")).map(PathBuf::from) {
        for desktop in [home.join("Desktop"), home.join("OneDrive").join("Desktop")] {
            for folder in ["RustLearningAppPortable", "RustQuest", "RustLearningApp/dist", "Rust Quest"] {
                paths.push(desktop.join(folder).join(OLD));
            }
        }
        paths.push(home.join("Downloads").join(OLD));
    }
    let mut found: Vec<(std::time::SystemTime, String, Value)> = paths
        .into_iter()
        .filter_map(|p| {
            let modified = std::fs::metadata(&p).and_then(|m| m.modified()).ok()?;
            let value = serde_json::from_str(&std::fs::read_to_string(&p).ok()?).ok()?;
            Some((modified, p.display().to_string(), value))
        })
        .collect();
    found.sort_by_key(|f| std::cmp::Reverse(f.0));
    found.dedup_by(|a, b| a.1.eq_ignore_ascii_case(&b.1));
    found.into_iter().map(|(_, p, v)| (p, v)).collect()
}
