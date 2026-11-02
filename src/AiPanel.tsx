// The AI helper: a chat with a model running on the learner's own computer.
import { useEffect, useRef, useState } from "react";
import { AI_PRESETS, ChatMessage, RunResult, api } from "./api";
import { Markdown } from "./Markdown";
import { update, useProgress } from "./progress";
import { aiContext, askAi, projectControl, workbenchControl } from "./bus";

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

/** How many times in a row the AI may run the code and answer again by itself. */
const MAX_AUTO_ROUNDS = 4;
const RUN_MARK = /^\s*\[\[(RUN|CHECK)\]\]\s*$/m;

type Block = { info: string; code: string };

/** The fenced code blocks of an answer, with their info strings ("python editor", "js file=a.js"). */
function codeBlocks(text: string): Block[] {
  const out: Block[] = [];
  const re = /^```([^\n`]*)\n([\s\S]*?)^```\s*$/gm;
  for (let m; (m = re.exec(text)); ) out.push({ info: m[1].trim(), code: m[2].replace(/\n$/, "") });
  return out;
}

/** What the AI should hear back after running the learner's code. */
function describeRun(r: RunResult | null, check: boolean): string {
  if (!r) return "The run was replaced by a newer one, so there's no result.";
  const fence = (t: string) => "```text\n" + (t.length > 2500 ? t.slice(0, 2500) + "…" : t) + "\n```";
  if (r.internalError) return `The app couldn't run it: ${r.internalError}`;
  if (!r.compileOk) return "It didn't compile:\n" + fence(r.compilerOutput);
  let out = "";
  if (check && r.check) out += r.check.passed ? "All checks passed. 🎉\n" : "The checks failed:\n" + fence(r.check.problems.join("\n")) + "\n";
  out += r.stdout ? "Output:\n" + fence(r.stdout) : "It printed nothing.";
  if (r.stderr.trim()) out += "\nError output:\n" + fence(r.stderr);
  if (r.timedOut) out += "\nIt was stopped because it took too long.";
  return out;
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

  // Braces matter: newer WebView2/Chromium returns a Promise from scrollIntoView, and React
  // would treat a returned value as the effect's cleanup function.
  useEffect(() => {
    bottom.current?.scrollIntoView({ block: "end" });
  }, [chat]);

  // Undo steps for the AI's changes, by id (kept outside the chat so the chat stays plain data).
  const stopped = useRef(false);
  const undos = useRef(new Map<number, () => Promise<unknown>>());
  const nextUndo = useRef(1);
  const [undone, setUndone] = useState<number[]>([]);

  function remember(undo: () => Promise<unknown>) {
    const undoId = nextUndo.current++;
    undos.current.set(undoId, undo);
    return undoId;
  }

  /** Applies the answer's `editor` and `file=` blocks, and says what changed. */
  async function applyChanges(answer: string): Promise<{ label: string; undoId: number }[]> {
    const changes: { label: string; undoId: number }[] = [];
    for (const b of codeBlocks(answer)) {
      const words = b.info.split(/\s+/);
      const file = words.find((w) => w.startsWith("file="))?.slice(5);
      try {
        if (words.includes("editor") && workbenchControl.current) {
          const wb = workbenchControl.current;
          const before = wb.getCode();
          wb.setCode(b.code);
          changes.push({ label: "✏ Replaced the code in the editor", undoId: remember(async () => wb.setCode(before)) });
        } else if (file && projectControl.current) {
          const pc = projectControl.current;
          const before = await pc.write(file, b.code + "\n");
          changes.push({ label: `📄 ${before === null ? "Created" : "Updated"} ${file}`, undoId: remember(() => pc.write(file, before)) });
        }
      } catch (e) {
        changes.push({ label: `⚠ Couldn't change ${file ?? "the editor"}: ${e}`, undoId: 0 });
      }
    }
    return changes;
  }

  async function send(question: string, auto?: { round: number; result: string }) {
    question = question.trim();
    if (!question || (streamId !== null && !auto)) return;
    if (!auto) stopped.current = false;
    const settings = p.ai;
    const sent = !auto && settings.include_context ? `${await aiContext.current()}\n\nMy question: ${question}` : undefined;
    const asked: ChatMessage = auto ? { role: "tool", content: auto.result } : { role: "user", content: question, sent };
    const history = [...chatRef.current.filter((m) => m.role !== "error"), asked];
    setChat([...history, { role: "assistant", content: "" }]);
    if (!auto) setInput("");
    // Older turns go as shown, to keep within small models' context; the newest with its context.
    const messages = history.map((m, i) => ({
      role: m.role === "tool" ? "user" : m.role,
      content: i === history.length - 1 ? (m.sent ?? m.content) : m.content,
    }));
    let answer = "";
    const id = await api.aiChat(settings, aiContext.subject(), messages, (e) => {
      if (e.kind === "delta") {
        answer += e.text;
        setChat((c) => {
          const last = c[c.length - 1];
          return [...c.slice(0, -1), { ...last, content: last.content + e.text }];
        });
      } else if (e.kind === "error") {
        setChat((c) => {
          const last = c[c.length - 1];
          const rest = last.role === "assistant" && !last.content ? c.slice(0, -1) : c;
          return [...rest, { role: "error", content: e.text }];
        });
        setStreamId(null);
      } else void finish(answer, auto?.round ?? 0);
    });
    setStreamId(id);
  }

  /** After an answer: apply its changes, then run or check the code if it asked to. */
  async function finish(answer: string, round: number) {
    const { answer: visible } = splitThinking(answer);
    if (p.ai.can_edit) {
      const changes = await applyChanges(visible);
      if (changes.length)
        setChat((c) => {
          const last = c[c.length - 1];
          return [...c.slice(0, -1), { ...last, changes }];
        });
      const mark = visible.match(RUN_MARK)?.[1];
      const wb = workbenchControl.current;
      if (mark && wb && round < MAX_AUTO_ROUNDS && !stopped.current) {
        const check = mark === "CHECK" && wb.canCheck;
        // Let React save the new code before running it.
        await new Promise((ok) => setTimeout(ok, 50));
        const result = await wb.run(check);
        const label = check ? "🛠 Checked the exercise" : "🛠 Ran the code";
        return sendRef.current(label, { round: round + 1, result: `${label} (round ${round + 1}). ${describeRun(result, check)}` });
      }
    }
    setStreamId(null);
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
            <input type="checkbox" checked={p.ai.can_edit} onChange={(e) => update((q) => void (q.ai.can_edit = e.target.checked))} />
            Let the AI change my code and project files, and run them (every change can be undone)
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
          if (m.role === "tool")
            return (
              <details key={i} className="msg tool small">
                <summary>{m.content.split(" (round")[0]}: the result went back to the AI</summary>
                <Markdown text={m.content} />
              </details>
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
              {answer ? <Markdown text={answer.replace(RUN_MARK, "")} /> : !still && <span className="muted">…</span>}
              {m.changes && (
                <div className="ai-changes">
                  {m.changes.map((ch, k) => (
                    <div key={k} className="small">
                      {ch.label}{" "}
                      {ch.undoId > 0 &&
                        (undone.includes(ch.undoId) ? (
                          <span className="muted">(undone)</span>
                        ) : (
                          <button
                            className="btn subtle small"
                            onClick={async () => {
                              await undos.current.get(ch.undoId)?.();
                              setUndone((u) => [...u, ch.undoId]);
                            }}
                          >
                            ↶ Undo
                          </button>
                        ))}
                    </div>
                  ))}
                </div>
              )}
              {!still && streamId === null && workbenchControl.current && (
                <div className="btn-row">
                  {codeBlocks(answer)
                    .filter((b) => !/\b(editor|file=)/.test(b.info) && !/^(text|console|output)?$/.test(b.info))
                    .map((b, k) => (
                      <button
                        key={k}
                        className="btn subtle small"
                        title="Replace the code in the editor with this block"
                        onClick={() => {
                          const wb = workbenchControl.current;
                          if (!wb) return;
                          const before = wb.getCode();
                          wb.setCode(b.code);
                          const undoId = remember(async () => wb.setCode(before));
                          setChat((c) =>
                            c.map((x, j) => (j === i ? { ...x, changes: [...(x.changes ?? []), { label: "✏ Put a code block in the editor", undoId }] } : x)),
                          );
                        }}
                      >
                        ⤵ Use block {k + 1} in the editor
                      </button>
                    ))}
                </div>
              )}
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
          <button
            className="btn danger"
            onClick={() => {
              stopped.current = true;
              api.aiStop(streamId);
            }}
          >
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
