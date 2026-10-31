// The AI helper: a chat with a model running on the learner's own computer.
import { useEffect, useRef, useState } from "react";
import { AI_PRESETS, ChatMessage, api } from "./api";
import { Markdown } from "./Markdown";
import { update, useProgress } from "./progress";
import { aiContext, askAi } from "./bus";

const ASKS: [string, string][] = [
  ["Explain my error", "Can you explain the error or failed check I'm getting, and how I could fix it?"],
  ["Give me a hint", "I'm stuck. Can you give me a hint for the next step, without giving away the whole answer?"],
  ["Review my code", "Can you review my code? What's good, and what could be clearer or more idiomatic?"],
  ["Explain this lesson simply", "Can you explain the main idea of this lesson in simple words, with a small example?"],
  ["Quiz me", "Ask me 3 short questions about this lesson, one at a time, and tell me if my answers are right."],
];

/** Splits a reasoning model's <think>…</think> part from its answer. */
function splitThinking(text: string): { thinking: string | null; answer: string; still: boolean } {
  const t = text.trimStart();
  if (!t.startsWith("<think>")) return { thinking: null, answer: text, still: false };
  const rest = t.slice(7);
  const end = rest.indexOf("</think>");
  if (end < 0) return { thinking: rest.trim(), answer: "", still: true };
  return { thinking: rest.slice(0, end).trim(), answer: rest.slice(end + 8).trimStart(), still: false };
}

export function AiPanel({ open, setOpen }: { open: boolean; setOpen: (o: boolean) => void }) {
  const p = useProgress();
  const [chat, setChat] = useState<ChatMessage[]>([]);
  const [input, setInput] = useState("");
  const [streamId, setStreamId] = useState<number | null>(null);
  const [showSettings, setShowSettings] = useState(false);
  const [models, setModels] = useState<string[]>([]);
  const [status, setStatus] = useState<{ ok: boolean; text: string } | null>(null);
  const bottom = useRef<HTMLDivElement>(null);
  const chatRef = useRef(chat);
  chatRef.current = chat;

  useEffect(() => bottom.current?.scrollIntoView({ block: "end" }), [chat]);

  async function send(question: string) {
    question = question.trim();
    if (!question || streamId !== null) return;
    const settings = p.ai;
    const sent = settings.include_context ? `${await aiContext.current()}\n\nMy question: ${question}` : undefined;
    const history = [...chatRef.current.filter((m) => m.role !== "error"), { role: "user" as const, content: question, sent }];
    setChat([...history, { role: "assistant", content: "" }]);
    setInput("");
    // Older turns go as shown, to keep within small models' context; the newest with its context.
    const messages = history.map((m, i) => ({ role: m.role, content: i === history.length - 1 ? (m.sent ?? m.content) : m.content }));
    const id = await api.aiChat(settings, aiContext.subject(), messages, (e) => {
      if (e.kind === "delta")
        setChat((c) => {
          const last = c[c.length - 1];
          return [...c.slice(0, -1), { ...last, content: last.content + e.text }];
        });
      else {
        if (e.kind === "error")
          setChat((c) => {
            const last = c[c.length - 1];
            const rest = last.role === "assistant" && !last.content ? c.slice(0, -1) : c;
            return [...rest, { role: "error", content: e.text }];
          });
        setStreamId(null);
      }
    });
    setStreamId(id);
  }

  const sendRef = useRef(send);
  sendRef.current = send;
  useEffect(
    () =>
      askAi.on((q) => {
        setOpen(true);
        if (q) sendRef.current(q);
      }),
    [],
  );

  async function testConnection() {
    setStatus({ ok: true, text: "Connecting…" });
    try {
      const list = await api.aiModels(p.ai.base_url);
      setModels(list);
      setStatus({ ok: true, text: list.length ? `Connected. ${list.length} model(s) available.` : "Connected, but no model is loaded." });
    } catch (e) {
      setStatus({ ok: false, text: String(e) });
    }
  }

  if (!open) return null;
  return (
    <aside className="ai-panel">
      <div className="ai-head">
        <strong>🤖 AI Helper</strong>
        <span className="muted small">runs on your computer</span>
        <span className="spacer" />
        <button className="btn subtle small" onClick={() => setShowSettings(!showSettings)}>
          ⚙ Settings
        </button>
        <button className="btn subtle small" disabled={!chat.length || streamId !== null} onClick={() => setChat([])}>
          New chat
        </button>
        <button className="btn subtle small" onClick={() => setOpen(false)}>
          ✕
        </button>
      </div>

      {showSettings && (
        <div className="ai-settings">
          <p className="small muted">
            Works with any local server that speaks the OpenAI API: <b>Ollama</b> (<code>ollama serve</code>, then e.g.{" "}
            <code>ollama pull qwen2.5-coder:7b</code>), <b>LM Studio</b> (start the local server), llama.cpp or Jan. Nothing is sent anywhere else.
          </p>
          <div className="btn-row">
            {AI_PRESETS.map(([name, url]) => (
              <button key={name} className={"btn small" + (p.ai.base_url === url ? " primary" : "")} onClick={() => update((q) => void (q.ai.base_url = url))}>
                {name}
              </button>
            ))}
          </div>
          <label>
            Server address
            <input value={p.ai.base_url} onChange={(e) => update((q) => void (q.ai.base_url = e.target.value))} spellCheck={false} />
          </label>
          <label>
            Model
            <div className="row">
              <input
                list="ai-models"
                value={p.ai.model}
                placeholder="(the first model the server lists)"
                onChange={(e) => update((q) => void (q.ai.model = e.target.value))}
                spellCheck={false}
              />
              <datalist id="ai-models">
                {models.map((m) => (
                  <option key={m} value={m} />
                ))}
              </datalist>
              <button className="btn small" onClick={testConnection}>
                Test & list models
              </button>
            </div>
          </label>
          {status && <div className={status.ok ? "good small" : "bad small"}>{status.text}</div>}
          <label className="check">
            <input type="checkbox" checked={p.ai.hints_only} onChange={(e) => update((q) => void (q.ai.hints_only = e.target.checked))} />
            Hints, not full solutions (recommended: you learn more)
          </label>
          <label className="check">
            <input type="checkbox" checked={p.ai.include_context} onChange={(e) => update((q) => void (q.ai.include_context = e.target.checked))} />
            Include the lesson, my code and errors with each question
          </label>
        </div>
      )}

      <div className="ai-chat">
        {chat.length === 0 && (
          <div className="muted small ai-empty">
            Stuck? Ask anything about the lesson, your code, an error, a cert topic or a command. The helper can see what you're working on.
          </div>
        )}
        {chat.map((m, i) => {
          if (m.role === "user")
            return (
              <div key={i} className="msg user">
                {m.content}
              </div>
            );
          if (m.role === "error")
            return (
              <div key={i} className="msg error">
                ⚠ {m.content}
              </div>
            );
          const { thinking, answer, still } = splitThinking(m.content);
          return (
            <div key={i} className="msg assistant">
              {thinking !== null && (
                <details className="thinking" open={still}>
                  <summary>{still ? "Thinking…" : "Thought process"}</summary>
                  <div className="small muted">{thinking}</div>
                </details>
              )}
              {answer ? <Markdown text={answer} /> : !still && <span className="muted">…</span>}
            </div>
          );
        })}
        <div ref={bottom} />
      </div>

      <div className="ai-asks">
        {ASKS.map(([label, q]) => (
          <button key={label} className="chip" disabled={streamId !== null} onClick={() => send(q)}>
            {label}
          </button>
        ))}
      </div>
      <div className="ai-input">
        <textarea
          value={input}
          rows={3}
          placeholder="Ask a question… (Enter sends, Shift+Enter for a new line)"
          onChange={(e) => setInput(e.target.value)}
          onKeyDown={(e) => {
            if (e.key === "Enter" && !e.shiftKey) {
              e.preventDefault();
              send(input);
            }
          }}
        />
        {streamId !== null ? (
          <button className="btn danger" onClick={() => api.aiStop(streamId)}>
            ■ Stop
          </button>
        ) : (
          <button className="btn primary" disabled={!input.trim()} onClick={() => send(input)}>
            Send
          </button>
        )}
      </div>
    </aside>
  );
}
