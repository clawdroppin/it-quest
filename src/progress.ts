// The learner's progress: one plain object, kept in memory and saved (debounced) to
// it_quest_progress.json next to the exe through the Rust side.
import { useSyncExternalStore } from "react";
import { AiSettings, Lang, api } from "./api";

export type Page = "home" | "learn" | "certs" | "projects" | "compare" | "review";

/** Leitner box (0–5) and when the card is due again (ms since epoch). */
export interface CardState {
  box: number;
  due: number;
}

export interface ExamResult {
  track: string;
  score: number;
  total: number;
  date: number;
}

export interface Progress {
  version: 1;
  completed: string[];
  /** "<lesson id>#<exercise index>" */
  passed: string[];
  /** Learner code per "<lesson id>#<exercise index>". */
  code: Record<string, string>;
  /** Quiz answers per "<lesson id>?<block index>". */
  quiz: Record<string, number>;
  /** Drills answered correctly, per "<lesson id>!<block index>". */
  drills: Record<string, boolean>;
  playground: Partial<Record<Lang, string>>;
  /** Lessons the learner has opened; their flashcards join the Review deck. */
  visited: string[];
  cards: Record<string, CardState>;
  exams: ExamResult[];
  page: Page;
  /** Selected track per page ("learn", "certs"), and last lesson per track or "projects". */
  track: Record<string, string>;
  last: Record<string, string>;
  showEditorInBuilds: boolean;
  zoom: number;
  ai: AiSettings;
  /** The Rust Quest progress import offer was answered. */
  rustQuestAsked: boolean;
}

export function defaultProgress(): Progress {
  return {
    version: 1,
    completed: [],
    passed: [],
    code: {},
    quiz: {},
    drills: {},
    playground: {},
    visited: [],
    cards: {},
    exams: [],
    page: "home",
    track: { learn: "python", certs: "foundations" },
    last: {},
    showEditorInBuilds: false,
    zoom: 1,
    ai: { base_url: "http://localhost:11434/v1", model: "", hints_only: true, include_context: true },
    rustQuestAsked: false,
  };
}

let state: Progress = defaultProgress();
const listeners = new Set<() => void>();
let saveTimer: number | undefined;

export async function loadProgress() {
  const saved = (await api.loadProgress()) as Partial<Progress> | null;
  if (saved) {
    const d = defaultProgress();
    state = { ...d, ...saved, ai: { ...d.ai, ...(saved.ai ?? {}) }, track: { ...d.track, ...(saved.track ?? {}) } };
  }
  return saved != null;
}

export function getProgress() {
  return state;
}

export function update(fn: (p: Progress) => void) {
  const next = structuredClone(state);
  fn(next);
  state = next;
  listeners.forEach((l) => l());
  clearTimeout(saveTimer);
  saveTimer = window.setTimeout(saveNow, 600);
}

export function saveNow(): Promise<void> {
  clearTimeout(saveTimer);
  return api.saveProgress(state).catch((e) => console.error("saving progress failed", e));
}

export function useProgress(): Progress {
  return useSyncExternalStore(
    (l) => {
      listeners.add(l);
      return () => listeners.delete(l);
    },
    () => state,
  );
}

export function addUnique(list: string[], item: string) {
  if (!list.includes(item)) list.push(item);
}

/** Copies Rust Quest's progress into the Rust track. */
export function importRustQuest(old: any) {
  update((p) => {
    const pre = (id: string) => `rust/${id}`;
    for (const id of old.completed_lessons ?? []) addUnique(p.completed, pre(id));
    for (const id of old.passed_exercises ?? []) addUnique(p.passed, pre(id));
    for (const [k, v] of Object.entries(old.code ?? {})) p.code[pre(k)] ??= v as string;
    for (const [k, v] of Object.entries(old.quiz_answers ?? {})) p.quiz[pre(k)] ??= v as number;
    for (const id of old.completed_lessons ?? []) addUnique(p.visited, pre(id));
    if (old.playground && !p.playground.rust) p.playground.rust = old.playground;
    if (old.last_lesson) p.last.rust = pre(old.last_lesson);
    if (old.ai?.base_url) {
      p.ai = { ...p.ai, base_url: old.ai.base_url, model: old.ai.model ?? "", hints_only: old.ai.hints_only ?? true };
      if (old.ai.include_context != null) p.ai.include_context = old.ai.include_context;
    }
    p.rustQuestAsked = true;
  });
}
