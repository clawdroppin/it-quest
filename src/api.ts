// Types shared with the Rust side (src-tauri/src/course.rs, runner.rs) and wrappers around invoke.
import { Channel, invoke } from "@tauri-apps/api/core";

export type Lang = "rust" | "python" | "javascript" | "powershell" | "bash" | "sql" | "typescript" | "go" | "csharp" | "cpp" | "html";
export type Level = "novice" | "intermediate" | "advanced" | "expert";
export type Category = "scripting" | "programming" | "data" | "foundations" | "cert" | "career";

export type Segment = { kind: "md"; text: string } | { kind: "code"; code: string; lang: Lang; runnable: boolean };

export interface Quiz {
  question: string;
  options: string[];
  correct: number;
  explain: string;
}
export interface Card {
  front: string;
  back: string;
}
export interface Drill {
  question: string;
  answers: string[];
  explain: string;
}
export interface Project {
  folder: string;
  title: string;
  description: string;
  files: [string, string][];
}

export type Block =
  | { kind: "text"; segments: Segment[] }
  | ({ kind: "quiz" } & Quiz)
  | { kind: "flashcards"; cards: Card[] }
  | ({ kind: "drill" } & Drill)
  | ({ kind: "project" } & Project);

export interface Exercise {
  title: string;
  prompt: string;
  starter: string;
  expected: string | null;
  stdin: string;
  tests: string | null;
  requires: string[];
  setup: string;
  hint: string;
  solution: string;
}

export interface Lesson {
  id: string;
  title: string;
  concepts: string[];
  lang: Lang | null;
  blocks: Block[];
  exercises: Exercise[];
  outro: Segment[];
  /** Hidden code run before examples and the playground (SQL: the sample database). */
  setup: string;
}

export interface Module {
  title: string;
  blurb: string;
  level: Level;
  build: boolean;
  lessons: Lesson[];
}

export interface Track {
  id: string;
  name: string;
  icon: string;
  category: Category;
  lang: Lang | null;
  blurb: string;
  modules: Module[];
}

export interface CourseData {
  tracks: Track[];
  concepts: [string, string][];
  projects_root: string;
}

export interface CheckResult {
  passed: boolean;
  problems: string[];
}

export interface RunResult {
  compileOk: boolean;
  compilerOutput: string;
  stdout: string;
  stderr: string;
  exitCode: number | null;
  timedOut: boolean;
  check: CheckResult | null;
  internalError: string | null;
}

export interface ToolStatus {
  lang: Lang;
  name: string;
  ok: boolean;
  info: string;
}

export interface ProjectStatus {
  dir: string;
  existing: number;
  total: number;
  commands: { label: string; command: string }[];
}

export interface AiSettings {
  base_url: string;
  model: string;
  hints_only: boolean;
  include_context: boolean;
}

export interface ChatMessage {
  role: "user" | "assistant" | "error";
  /** What the chat shows. */
  content: string;
  /** What was actually sent (the question plus app context), if different. */
  sent?: string;
}

export type AiEvent = { kind: "delta"; text: string } | { kind: "done" } | { kind: "error"; text: string };

export const api = {
  course: () => invoke<CourseData>("get_course"),
  tools: () => invoke<ToolStatus[]>("tool_status"),
  run: (job: {
    lang: Lang;
    code: string;
    stdin: string;
    setup: string;
    check: { expected: string | null; tests: string | null; requires: string[] } | null;
  }) => invoke<RunResult>("run_code", { job }),
  loadProgress: () => invoke<unknown | null>("load_progress"),
  saveProgress: (progress: unknown) => invoke<void>("save_progress", { progress }),
  findRustQuest: () => invoke<{ path: string; data: any }[]>("find_rust_quest_progress"),
  projectStatus: (lessonId: string, block: number) => invoke<ProjectStatus>("project_status", { lessonId, block }),
  projectAction: (lessonId: string, block: number, action: string, command?: string) =>
    invoke<string>("project_action", { lessonId, block, action, command }),
  projectSources: (lessonId: string, block: number) => invoke<string>("project_sources", { lessonId, block }),
  aiModels: (baseUrl: string) => invoke<string[]>("ai_models", { baseUrl }),
  aiChat: (settings: AiSettings, subject: string, messages: { role: string; content: string }[], onEvent: (e: AiEvent) => void) => {
    const channel = new Channel<AiEvent>();
    channel.onmessage = onEvent;
    return invoke<number>("ai_chat", { settings, subject, messages, channel });
  },
  aiStop: (id: number) => invoke<void>("ai_stop", { id }),
  openUrl: (url: string) => invoke<void>("open_url", { url }),
};

export const LANG_NAMES: Record<Lang, string> = {
  rust: "Rust",
  python: "Python",
  javascript: "JavaScript",
  powershell: "PowerShell",
  bash: "Bash",
  sql: "SQL",
  typescript: "TypeScript",
  go: "Go",
  csharp: "C#",
  cpp: "C++",
  html: "HTML & CSS",
};

export const LEVELS: { id: Level; name: string }[] = [
  { id: "novice", name: "Novice" },
  { id: "intermediate", name: "Intermediate" },
  { id: "advanced", name: "Advanced" },
  { id: "expert", name: "Expert" },
];

export const AI_PRESETS: [string, string][] = [
  ["Ollama", "http://localhost:11434/v1"],
  ["LM Studio", "http://localhost:1234/v1"],
  ["llama.cpp", "http://localhost:8080/v1"],
  ["Jan", "http://localhost:1337/v1"],
];
