//! Optional AI helper. Talks to a model running on the learner's own computer through the
//! OpenAI-compatible HTTP API that LM Studio, Ollama, llama.cpp's server and Jan all offer.
//! Nothing is sent anywhere unless the learner sets it up, and only to the URL they choose.

use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use std::io::{BufRead, BufReader};
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Duration;
use tauri::ipc::Channel;

#[derive(Clone, Deserialize)]
pub struct AiSettings {
    pub base_url: String,
    /// Empty means "whatever the server lists first".
    #[serde(default)]
    pub model: String,
    /// Ask the model to guide instead of handing out full solutions.
    #[serde(default)]
    pub hints_only: bool,
    /// Let the model change the editor and project files, and run the code (see EDIT_RULES).
    #[serde(default)]
    pub can_edit: bool,
}

#[derive(Clone, Deserialize, Serialize)]
pub struct Message {
    /// "user" or "assistant"
    pub role: String,
    pub content: String,
}

#[derive(Clone, Serialize)]
#[serde(tag = "kind", content = "text", rename_all = "camelCase")]
pub enum Event {
    Delta(String),
    Done,
    Error(String),
}

/// `subject` is what the learner is studying right now: "Python", "CompTIA Network+"…
/// How the model edits and runs things. The app (AiPanel.tsx) looks for exactly these markers.
const EDIT_RULES: &str = " You are built into the app and can work in it directly. To replace ALL the code in the learner's editor, write a fenced block whose info string is the language followed by `editor`, for example ```python editor. To create or replace a file of the lesson's project on disk, use the language followed by `file=<relative path>`, for example ```python file=app/models.py. Such blocks must contain the complete code or file, never a fragment, and each one is applied as soon as you finish answering (the learner can undo it). Plain ```python blocks are only shown. To run the editor's code and see its output, end your answer with a line containing only [[RUN]]; to run the exercise's checks, use [[CHECK]]. The result comes back in the next message, so you can fix things and try again. Only change the learner's work when they ask you to write, fix or change code, and say in one sentence what you changed.";

fn system_prompt(subject: &str, hints_only: bool, can_edit: bool) -> String {
    let mut p = format!(
        "You are a patient, encouraging tutor built into IT Quest, an app that teaches complete beginners \
the skills for a career in IT: programming and scripting languages, networking, security, cloud, Linux and \
Windows administration, and IT certifications. The learner is studying {subject} right now. Explain things \
simply, in plain language, and connect them to what the learner is studying. Prefer short answers (under 250 \
words) unless asked for more. Use Markdown, with fenced code blocks tagged with the language for code. When \
explaining an error, say in one sentence what it means, point at the line, and explain how to fix it. For \
certification topics, be accurate, say when exam details may have changed, and point to the official exam objectives."
    );
    if subject == "Rust" {
        p.push_str(" The learner uses Rust edition 2024.");
    }
    if subject == "PowerShell" {
        p.push_str(" The app runs Windows PowerShell 5.1, so avoid PowerShell 7-only syntax like ?? and ternaries.");
    }
    if can_edit {
        p.push_str(EDIT_RULES);
        if hints_only {
            p.push_str(" Because the learner wants hints, only edit or run their code when they explicitly ask you to.");
        }
    }
    if hints_only {
        p.push_str(
            " The learner wants to learn by doing: do NOT write the complete solution to their exercise, \
project step or quiz. Give hints, ask guiding questions, point at the problem, and show small examples on \
different data. Only if they explicitly insist, give the solution, and explain it line by line.",
        );
    }
    p
}

fn agent() -> ureq::Agent {
    ureq::Agent::config_builder()
        .timeout_connect(Some(Duration::from_secs(4)))
        .timeout_per_read(Some(Duration::from_secs(180)))
        .http_status_as_error(false)
        .build()
        .into()
}

fn endpoint(base_url: &str, path: &str) -> String {
    format!("{}/{path}", base_url.trim().trim_end_matches('/'))
}

fn friendly_connect_error(e: &ureq::Error, base_url: &str) -> String {
    match e {
        ureq::Error::Io(_) | ureq::Error::ConnectionFailed | ureq::Error::HostNotFound => format!(
            "Couldn't reach an AI server at {base_url}. Is Ollama (or LM Studio) running, with its local server started?"
        ),
        ureq::Error::Timeout(_) => "The AI server took too long to answer.".to_string(),
        other => format!("Connection problem: {other}"),
    }
}

/// Asks the server which models it offers. Blocking: call it from a background thread.
pub fn list_models(base_url: &str) -> Result<Vec<String>, String> {
    let mut resp = agent().get(&endpoint(base_url, "models")).call().map_err(|e| friendly_connect_error(&e, base_url))?;
    let status = resp.status().as_u16();
    let body = resp.body_mut().read_to_string().map_err(|e| e.to_string())?;
    if status != 200 {
        return Err(format!("The server answered with status {status}: {}", shorten(&body, 200)));
    }
    let value: Value = serde_json::from_str(&body).map_err(|_| "The server's answer wasn't the expected JSON.".to_string())?;
    let models: Vec<String> = value["data"]
        .as_array()
        .map(|list| list.iter().filter_map(|m| m["id"].as_str().map(String::from)).collect())
        .unwrap_or_default();
    // Embedding models can't chat; put them last.
    let (mut chat, embed): (Vec<String>, Vec<String>) = models.into_iter().partition(|m| !m.contains("embed"));
    chat.extend(embed);
    Ok(chat)
}

/// Sends the conversation and streams the answer back on `channel`, until done or `stop` is set.
/// Blocking: call it from a background thread.
pub fn chat(settings: AiSettings, subject: String, history: Vec<Message>, stop: Arc<AtomicBool>, channel: Channel<Event>) {
    let send = |event: Event| {
        let _ = channel.send(event);
    };
    let mut messages = vec![json!({ "role": "system", "content": system_prompt(&subject, settings.hints_only, settings.can_edit) })];
    // Keep within small models' context: only the last dozen turns.
    for m in history.iter().skip(history.len().saturating_sub(12)) {
        messages.push(json!({ "role": m.role, "content": m.content }));
    }
    let base_url = settings.base_url;
    let mut model = settings.model;
    if model.trim().is_empty() {
        match list_models(&base_url) {
            Ok(list) if !list.is_empty() => model = list[0].clone(),
            Ok(_) => return send(Event::Error("The server has no model loaded. Run `ollama pull <model>` (or load one in LM Studio) first.".into())),
            Err(e) => return send(Event::Error(e)),
        }
    }
    let body = json!({ "model": model, "messages": messages, "stream": true, "temperature": 0.4 });
    let resp = match agent().post(&endpoint(&base_url, "chat/completions")).send_json(&body) {
        Ok(r) => r,
        Err(e) => return send(Event::Error(friendly_connect_error(&e, &base_url))),
    };
    let status = resp.status().as_u16();
    let mut body = resp.into_body();
    if status != 200 {
        let text = body.read_to_string().unwrap_or_default();
        return send(Event::Error(format!("The AI server answered with status {status}: {}", shorten(&text, 300))));
    }

    let reader = BufReader::new(body.into_reader());
    for line in reader.lines() {
        if stop.load(Ordering::Relaxed) {
            return send(Event::Done);
        }
        let Ok(line) = line else {
            return send(Event::Error("The connection to the AI server was lost.".into()));
        };
        let data = line.trim().strip_prefix("data:").unwrap_or(line.trim()).trim();
        if data.is_empty() || data.starts_with(':') {
            continue;
        }
        if data == "[DONE]" {
            break;
        }
        let Ok(value) = serde_json::from_str::<Value>(data) else { continue };
        if let Some(err) = value.get("error") {
            let msg = err["message"].as_str().map(String::from).unwrap_or_else(|| err.to_string());
            return send(Event::Error(msg));
        }
        let choice = &value["choices"][0];
        // Streaming chunks carry `delta`; a server that ignores `stream` sends one `message`.
        let text = choice["delta"]["content"].as_str().or_else(|| choice["message"]["content"].as_str());
        if let Some(text) = text
            && !text.is_empty()
        {
            send(Event::Delta(text.to_string()));
        }
    }
    send(Event::Done);
}

pub fn shorten(text: &str, max_chars: usize) -> String {
    let text = text.trim();
    if text.chars().count() <= max_chars {
        return text.to_string();
    }
    let cut: String = text.chars().take(max_chars).collect();
    format!("{cut}…")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builds_endpoints() {
        assert_eq!(endpoint("http://localhost:1234/v1/", "models"), "http://localhost:1234/v1/models");
        assert_eq!(shorten("abcdef", 3), "abc…");
    }
}
