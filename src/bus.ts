// Small cross-component signals: "try this code", "ask the AI", and what the AI should know.
import { Lang } from "./api";

type Handler<T> = (value: T) => void;

function signal<T>() {
  const handlers = new Set<Handler<T>>();
  return {
    emit: (v: T) => handlers.forEach((h) => h(v)),
    on: (h: Handler<T>) => {
      handlers.add(h);
      return () => {
        handlers.delete(h);
      };
    },
  };
}

/** A lesson's "Try it" button: run this example in the playground. */
export const tryCode = signal<{ lang: Lang; code: string }>();

/** Open the AI helper, optionally asking a question right away. */
export const askAi = signal<string | null>();

/** The open page tells the AI helper what the learner is looking at. */
export const aiContext: { current: () => Promise<string>; subject: () => string } = {
  current: async () => "",
  subject: () => "IT",
};

/** The code editor's part of that context (its code and last output), while it's shown. */
export const workbenchContext: { current: (() => string) | null } = { current: null };
