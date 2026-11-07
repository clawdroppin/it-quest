//! The course: lesson file format, parser, and the list of tracks.
//!
//! Lesson files live in `content/<track>/` and are compiled into the exe with `include_str!`.
//! A lesson file is split into sections by lines that start with `@@`:
//!
//! ```text
//! @@lesson <id> <Title of the lesson>     id is local to the track; the full id is "<track>/<id>"
//! @@concepts loops, functions             optional: tags for the Compare tab (see CONCEPTS)
//! @@lang bash                            optional: the language of this lesson's code, if not the track's
//! @@text            markdown; ```<lang> blocks of a runnable language get "Try it" buttons
//!                   (```<lang>,norun blocks do not)
//! @@quiz            question lines, then `- wrong`, `* correct`, `? explanation`
//! @@flashcards      one card per line: `front :: back`
//! @@drill           a typed answer: question lines, `= accepted answer` lines, `? explanation`
//! @@exercise <Title>  markdown task description
//! @@starter         code the editor starts with
//! @@expected        exact program output needed to pass
//! @@stdin           input fed to the program when checking
//! @@tests           checks run against the code (Rust: #[test] fns; scripts: check(actual, expected, label) calls)
//! @@requires        one snippet per line the code must contain
//! @@setup           hidden code run before the learner's code (SQL: create and fill the tables)
//! @@hint / @@solution
//! @@text           (after an exercise) closing text shown below the practice section
//! @@project <folder> <Title>   markdown; a project card the learner can create on disk
//! @@file <path>     a file of the project above (the lesson's starting point for that file)
//! ```

use crate::runner::Lang;
use serde::Serialize;

#[derive(Serialize, Clone)]
#[serde(tag = "kind", rename_all = "camelCase")]
pub enum Segment {
    Md { text: String },
    Code { code: String, lang: Lang, runnable: bool },
}

#[derive(Serialize, Clone)]
pub struct Quiz {
    pub question: String,
    pub options: Vec<String>,
    pub correct: usize,
    pub explain: String,
}

#[derive(Serialize, Clone)]
pub struct Card {
    pub front: String,
    pub back: String,
}

#[derive(Serialize, Clone)]
pub struct Drill {
    pub question: String,
    pub answers: Vec<String>,
    pub explain: String,
}

/// A project the learner works on in VS Code and a real terminal.
#[derive(Serialize, Clone)]
pub struct Project {
    pub folder: String,
    pub title: String,
    pub description: String,
    /// (relative path, contents)
    pub files: Vec<(String, String)>,
}

#[derive(Serialize, Clone)]
#[serde(tag = "kind", rename_all = "camelCase")]
pub enum Block {
    Text { segments: Vec<Segment> },
    Quiz(Quiz),
    Flashcards { cards: Vec<Card> },
    Drill(Drill),
    Project(Project),
}

#[derive(Serialize, Clone, Default)]
pub struct Exercise {
    pub title: String,
    pub prompt: String,
    pub starter: String,
    pub expected: Option<String>,
    pub stdin: String,
    pub tests: Option<String>,
    pub requires: Vec<String>,
    pub setup: String,
    pub hint: String,
    pub solution: String,
}

#[derive(Serialize, Clone)]
pub struct Lesson {
    /// "<track>/<local id>". Progress is keyed by it, so never rename one.
    pub id: String,
    pub title: String,
    pub concepts: Vec<String>,
    /// Overrides the track's language (a Linux lesson in a cert track uses Bash).
    pub lang: Option<Lang>,
    pub blocks: Vec<Block>,
    pub exercises: Vec<Exercise>,
    /// Text written after the exercises, shown below the practice section.
    pub outro: Vec<Segment>,
    /// Hidden code run before the examples and the playground (SQL: the sample database).
    pub setup: String,
}

impl Lesson {
    /// Gives the lesson and every exercise without its own @@setup this sample data.
    pub fn with_setup(mut self, setup: &str) -> Lesson {
        self.setup = setup.to_string();
        for ex in self.exercises.iter_mut().filter(|e| e.setup.is_empty()) {
            ex.setup = setup.to_string();
        }
        self
    }
}

#[derive(Serialize, Clone, Copy, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum Level {
    Novice,
    Intermediate,
    Advanced,
    Expert,
}

#[derive(Serialize, Clone)]
pub struct Module {
    pub title: &'static str,
    pub blurb: &'static str,
    pub level: Level,
    /// Part of the Projects tab instead of the track's lessons.
    pub build: bool,
    pub lessons: Vec<Lesson>,
}

/// Where a track is shown in the app.
#[derive(Serialize, Clone, Copy, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum Category {
    Scripting,
    Programming,
    Data,
    Foundations,
    Cert,
    Career,
    /// Outside IT: the Extras tab.
    Extra,
}

#[derive(Serialize, Clone)]
pub struct Track {
    pub id: &'static str,
    pub name: &'static str,
    pub icon: &'static str,
    pub category: Category,
    /// The language this track's code runs in, if any.
    pub lang: Option<Lang>,
    pub blurb: &'static str,
    pub modules: Vec<Module>,
}

/// Trim blank lines at the start and end, keep inner indentation.
fn trim_block(s: &str) -> String {
    let lines: Vec<&str> = s.lines().collect();
    let start = lines.iter().position(|l| !l.trim().is_empty()).unwrap_or(lines.len());
    let end = lines.iter().rposition(|l| !l.trim().is_empty()).map_or(start, |i| i + 1);
    lines[start..end].join("\n")
}

/// Splits markdown into text and runnable code blocks. Fences in other languages
/// (```text, ```toml…) stay part of the markdown.
fn split_segments(md: &str) -> Vec<Segment> {
    let mut out = Vec::new();
    let mut text = String::new();
    let mut code: Option<(String, Lang, bool)> = None;
    let mut in_other_fence = false;
    for line in md.lines() {
        let t = line.trim_start();
        match &mut code {
            None if in_other_fence => {
                in_other_fence = !t.starts_with("```");
                text.push_str(line);
                text.push('\n');
            }
            None if t.starts_with("```") => {
                let info = t.trim_start_matches('`');
                let name = info.split(',').next().unwrap_or("").trim();
                match Lang::from_fence(name) {
                    Some(lang) => {
                        if !text.trim().is_empty() {
                            out.push(Segment::Md { text: trim_block(&text) });
                        }
                        text.clear();
                        code = Some((String::new(), lang, !info.contains("norun")));
                    }
                    None => {
                        in_other_fence = true;
                        text.push_str(line);
                        text.push('\n');
                    }
                }
            }
            None => {
                text.push_str(line);
                text.push('\n');
            }
            Some(_) if t.starts_with("```") => {
                let (c, lang, runnable) = code.take().unwrap();
                out.push(Segment::Code { code: trim_block(&c), lang, runnable });
            }
            Some((c, _, _)) => {
                c.push_str(line);
                c.push('\n');
            }
        }
    }
    if !text.trim().is_empty() {
        out.push(Segment::Md { text: trim_block(&text) });
    }
    out
}

fn parse_quiz(body: &str) -> Quiz {
    let mut q = Quiz { question: String::new(), options: vec![], correct: 0, explain: String::new() };
    let mut in_explain = false;
    for line in body.lines() {
        if in_explain {
            q.explain.push_str(line);
            q.explain.push('\n');
        } else if let Some(o) = line.strip_prefix("- ") {
            q.options.push(o.trim().to_string());
        } else if let Some(o) = line.strip_prefix("* ") {
            q.correct = q.options.len();
            q.options.push(o.trim().to_string());
        } else if let Some(e) = line.strip_prefix("? ") {
            in_explain = true;
            q.explain.push_str(e);
            q.explain.push('\n');
        } else {
            q.question.push_str(line);
            q.question.push('\n');
        }
    }
    q.question = trim_block(&q.question);
    q.explain = trim_block(&q.explain);
    q
}

fn parse_drill(body: &str) -> Drill {
    let mut d = Drill { question: String::new(), answers: vec![], explain: String::new() };
    let mut in_explain = false;
    for line in body.lines() {
        if in_explain {
            d.explain.push_str(line);
            d.explain.push('\n');
        } else if let Some(a) = line.strip_prefix("= ") {
            d.answers.push(a.trim().to_string());
        } else if let Some(e) = line.strip_prefix("? ") {
            in_explain = true;
            d.explain.push_str(e);
            d.explain.push('\n');
        } else {
            d.question.push_str(line);
            d.question.push('\n');
        }
    }
    d.question = trim_block(&d.question);
    d.explain = trim_block(&d.explain);
    d
}

fn parse_cards(body: &str) -> Vec<Card> {
    body.lines()
        .filter_map(|l| l.split_once("::"))
        .map(|(f, b)| Card { front: f.trim().to_string(), back: b.trim().to_string() })
        .collect()
}

pub fn parse_lesson(track: &str, src: &str) -> Lesson {
    let src = src.replace("\r\n", "\n");
    let mut sections: Vec<(String, String, String)> = Vec::new();
    for line in src.lines() {
        if let Some(rest) = line.strip_prefix("@@") {
            let (name, arg) = rest.split_once(' ').unwrap_or((rest, ""));
            sections.push((name.trim().to_string(), arg.trim().to_string(), String::new()));
        } else if let Some(last) = sections.last_mut() {
            last.2.push_str(line);
            last.2.push('\n');
        }
    }

    let mut lesson =
        Lesson { id: String::new(), title: String::new(), concepts: vec![], lang: None, blocks: vec![], exercises: vec![], outro: vec![], setup: String::new() };
    for (name, arg, body) in sections {
        let ex = lesson.exercises.last_mut();
        match (name.as_str(), ex) {
            ("lesson", _) => {
                let (id, title) = arg.split_once(' ').unwrap_or((&arg, ""));
                lesson.id = format!("{track}/{id}");
                lesson.title = title.to_string();
            }
            ("concepts", _) => {
                assert!(lesson.concepts.is_empty(), "lesson {}: only one @@concepts line per lesson (it tags the whole lesson)", lesson.id);
                lesson.concepts = arg.split(',').map(|c| c.trim().to_string()).filter(|c| !c.is_empty()).collect();
                for c in &lesson.concepts {
                    assert!(CONCEPTS.iter().any(|(id, _)| id == c), "lesson {}: unknown concept `{c}`", lesson.id);
                }
            }
            ("lang", _) => {
                lesson.lang = Some(Lang::from_fence(&arg).unwrap_or_else(|| panic!("lesson {}: unknown @@lang {arg}", lesson.id)))
            }
            ("project", _) => {
                let (folder, title) = arg.split_once(' ').unwrap_or((&arg, ""));
                lesson.blocks.push(Block::Project(Project {
                    folder: folder.to_string(),
                    title: title.to_string(),
                    description: trim_block(&body),
                    files: vec![],
                }));
            }
            ("file", _) => match lesson.blocks.last_mut() {
                Some(Block::Project(p)) => p.files.push((arg, trim_block(&body) + "\n")),
                _ => panic!("lesson {}: @@file must follow @@project", lesson.id),
            },
            ("text", Some(_)) => lesson.outro.extend(split_segments(&body)),
            ("text", None) => lesson.blocks.push(Block::Text { segments: split_segments(&body) }),
            ("quiz", _) => lesson.blocks.push(Block::Quiz(parse_quiz(&body))),
            ("flashcards", _) => lesson.blocks.push(Block::Flashcards { cards: parse_cards(&body) }),
            ("drill", _) => lesson.blocks.push(Block::Drill(parse_drill(&body))),
            ("exercise", _) => lesson.exercises.push(Exercise { title: arg, prompt: trim_block(&body), ..Default::default() }),
            ("starter", Some(ex)) => ex.starter = trim_block(&body),
            ("expected", Some(ex)) => ex.expected = Some(trim_block(&body)),
            ("stdin", Some(ex)) => ex.stdin = trim_block(&body),
            ("tests", Some(ex)) => ex.tests = Some(trim_block(&body)),
            ("setup", Some(ex)) => ex.setup = trim_block(&body),
            ("requires", Some(ex)) => {
                ex.requires = body.lines().map(str::trim).filter(|l| !l.is_empty()).map(String::from).collect()
            }
            ("hint", Some(ex)) => ex.hint = trim_block(&body),
            ("solution", Some(ex)) => ex.solution = trim_block(&body),
            (other, _) => panic!("lesson {}: unknown or misplaced section @@{}", lesson.id, other),
        }
    }
    lesson
}

/// Concepts shared across languages, in teaching order. The Compare tab shows, for each one,
/// how every language does it. (id, display name)
pub const CONCEPTS: &[(&str, &str)] = &[
    ("hello", "Hello, world"),
    ("variables", "Variables"),
    ("numbers", "Numbers & math"),
    ("strings", "Text & formatting"),
    ("input", "Reading input"),
    ("conditions", "If / else"),
    ("loops", "Loops"),
    ("functions", "Functions"),
    ("lists", "Lists & arrays"),
    ("maps", "Dictionaries & maps"),
    ("errors", "Handling errors"),
    ("files", "Files"),
    ("objects", "Structs, classes & objects"),
    ("modules", "Modules & packages"),
    ("testing", "Testing"),
    ("json", "JSON & data formats"),
    ("text-processing", "Searching & processing text"),
    ("processes", "Running commands & processes"),
    ("networking", "Networking & HTTP"),
    ("concurrency", "Concurrency"),
    ("automation", "Automation scripts"),
];

macro_rules! lessons {
    ($track:literal; $($file:literal),* $(,)?) => {
        vec![$(parse_lesson($track, include_str!(concat!("../../content/", $track, "/", $file)))),*]
    };
}

macro_rules! module {
    ($level:ident, $title:literal, $blurb:literal, $track:literal; $($file:literal),* $(,)?) => {
        Module { title: $title, blurb: $blurb, level: Level::$level, build: false, lessons: lessons![$track; $($file),*] }
    };
}

macro_rules! build {
    ($level:ident, $title:literal, $blurb:literal, $track:literal; $($file:literal),* $(,)?) => {
        Module { title: $title, blurb: $blurb, level: Level::$level, build: true, lessons: lessons![$track; $($file),*] }
    };
}

pub fn load_course() -> Vec<Track> {
    let mut tracks = vec![
        crate::tracks::python(),
        crate::tracks::powershell(),
        crate::tracks::bash(),
        crate::tracks::javascript(),
        crate::tracks::typescript(),
        crate::tracks::html(),
        crate::tracks::go(),
        crate::tracks::csharp(),
        crate::tracks::cpp(),
        crate::tracks::rust(),
        crate::tracks::sql(),
    ];
    tracks.extend(crate::tracks::knowledge());
    tracks.push(crate::tracks::gui());
    tracks.push(crate::tracks::linux_env());
    tracks.push(crate::tracks::gimp());
    tracks.push(crate::tracks::rust_ui());
    tracks.push(crate::tracks::vector_ui());
    tracks.push(crate::tracks::ui_anim());
    tracks.push(crate::tracks::blender());
    tracks.push(crate::tracks::resolve());
    tracks
}

pub(crate) use {build, lessons, module};
