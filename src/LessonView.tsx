// The lesson in the middle: text, runnable examples, quizzes, flashcards, drills and project cards.
import { useEffect, useState } from "react";
import { Card, Drill, LANG_NAMES, Project, ProjectStatus, Quiz, Segment, api } from "./api";
import { CodeEditor } from "./Editor";
import { Markdown } from "./Markdown";
import { LessonRef } from "./course";
import { addUnique, update, useProgress } from "./progress";
import { tryCode } from "./bus";
import { exerciseKey } from "./Workbench";

interface Props {
  r: LessonRef;
  next: LessonRef | undefined;
  prev: LessonRef | undefined;
  open: (id: string) => void;
  openExercise: (i: number) => void;
}

export function LessonView({ r, next, prev, open, openExercise }: Props) {
  const p = useProgress();
  const { lesson } = r;
  const done = p.completed.includes(lesson.id);
  const idx = r.module.lessons.indexOf(lesson);

  return (
    <article className="lesson">
      <div className="crumbs">
        {r.track.icon} {r.track.name} › {r.module.title} · lesson {idx + 1} of {r.module.lessons.length}
      </div>
      <h1>
        {lesson.title} {done && <span className="good small">✔ done</span>}
      </h1>

      {lesson.blocks.map((b, bi) => {
        switch (b.kind) {
          case "text":
            return <Segments key={bi} segments={b.segments} />;
          case "quiz":
            return <QuizCard key={bi} quiz={b} answerKey={`${lesson.id}?${bi}`} />;
          case "flashcards":
            return <Flashcards key={bi} cards={b.cards} />;
          case "drill":
            return <DrillCard key={bi} drill={b} drillKey={`${lesson.id}!${bi}`} />;
          case "project":
            return <ProjectCard key={bi} project={b} lessonId={lesson.id} block={bi} />;
        }
      })}

      {lesson.exercises.length > 0 && (
        <section className="practice">
          <h2>🏋 Practice</h2>
          {lesson.exercises.map((e, i) => {
            const ok = p.passed.includes(exerciseKey(lesson.id, i));
            return (
              <div key={i} className="practice-row">
                <span>
                  {ok ? "✅" : "⬜"} {i + 1}. {e.title}
                </span>
                <button className="btn small" onClick={() => openExercise(i)}>
                  {ok ? "Open again" : "Open in editor →"}
                </button>
              </div>
            );
          })}
        </section>
      )}
      {lesson.outro.length > 0 && <Segments segments={lesson.outro} />}

      <div className="lesson-nav">
        <button className="btn" disabled={!prev} onClick={() => prev && open(prev.lesson.id)}>
          ← Previous
        </button>
        <button
          className="btn primary"
          onClick={() => {
            update((q) => addUnique(q.completed, lesson.id));
            if (next) open(next.lesson.id);
          }}
        >
          {next ? (done ? "Next lesson →" : "✔ Complete & next →") : done ? "✔ Completed" : "✔ Mark complete"}
        </button>
        {done && (
          <button className="btn subtle small" onClick={() => update((q) => void (q.completed = q.completed.filter((c) => c !== lesson.id)))}>
            Mark not done
          </button>
        )}
      </div>
    </article>
  );
}

export function Segments({ segments }: { segments: Segment[] }) {
  return (
    <>
      {segments.map((s, i) =>
        s.kind === "md" ? (
          <Markdown key={i} text={s.text} />
        ) : (
          <div key={i} className="example">
            <CodeEditor value={s.code} lang={s.lang} readOnly />
            {s.runnable && (
              <button className="btn small try" onClick={() => tryCode.emit({ lang: s.lang, code: s.code })} title={`Runs this ${LANG_NAMES[s.lang]} example in the playground`}>
                ▶ Try it
              </button>
            )}
          </div>
        ),
      )}
    </>
  );
}

export function QuizCard({ quiz, answerKey }: { quiz: Quiz; answerKey: string }) {
  const p = useProgress();
  const answer = p.quiz[answerKey];
  const answered = answer !== undefined;
  return (
    <div className="card quiz">
      <div className="card-label">❓ Quick check</div>
      <Markdown text={quiz.question} />
      <div className="options">
        {quiz.options.map((o, i) => {
          let cls = "option";
          if (answered && i === quiz.correct) cls += " right";
          else if (answered && i === answer) cls += " wrong";
          return (
            <button key={i} className={cls} onClick={() => update((q) => void (q.quiz[answerKey] = i))}>
              <Markdown text={o} className="inline" />
            </button>
          );
        })}
      </div>
      {answered && (
        <div className={answer === quiz.correct ? "verdict good" : "verdict bad"}>
          {answer === quiz.correct ? "✔ Correct! " : "✘ Not quite. "}
          {quiz.explain && <Markdown text={quiz.explain} className="inline" />}
          {answer !== quiz.correct && (
            <button className="btn subtle small" onClick={() => update((q) => void delete q.quiz[answerKey])}>
              Try again
            </button>
          )}
        </div>
      )}
    </div>
  );
}

function Flashcards({ cards }: { cards: Card[] }) {
  const [flipped, setFlipped] = useState<Set<number>>(new Set());
  return (
    <div className="card flash-block">
      <div className="card-label">🃏 Flashcards — click a card to flip it. They also join your Review deck.</div>
      <div className="flash-grid">
        {cards.map((c, i) => (
          <button
            key={i}
            className={"flash" + (flipped.has(i) ? " flipped" : "")}
            onClick={() => {
              const s = new Set(flipped);
              s.has(i) ? s.delete(i) : s.add(i);
              setFlipped(s);
            }}
          >
            <Markdown text={flipped.has(i) ? c.back : c.front} />
          </button>
        ))}
      </div>
    </div>
  );
}

/** Answers are compared without case, extra spaces or a trailing period. */
export const normalizeAnswer = (s: string) => s.trim().toLowerCase().replace(/\s+/g, " ").replace(/\.$/, "");

function DrillCard({ drill, drillKey }: { drill: Drill; drillKey: string }) {
  const p = useProgress();
  const [text, setText] = useState("");
  const [result, setResult] = useState<null | boolean>(null);
  const solved = p.drills[drillKey];
  const submit = () => {
    const ok = drill.answers.some((a) => normalizeAnswer(a) === normalizeAnswer(text));
    setResult(ok);
    if (ok) update((q) => void (q.drills[drillKey] = true));
  };
  return (
    <div className="card drill">
      <div className="card-label">⌨ Drill — type the answer {solved && <span className="good">✔</span>}</div>
      <Markdown text={drill.question} />
      <div className="drill-row">
        <input
          value={text}
          onChange={(e) => {
            setText(e.target.value);
            setResult(null);
          }}
          onKeyDown={(e) => e.key === "Enter" && submit()}
          spellCheck={false}
          placeholder="Type here and press Enter"
        />
        <button className="btn" onClick={submit}>
          Check
        </button>
        <button className="btn subtle" onClick={() => setResult(false)}>
          Show answer
        </button>
      </div>
      {result === true && <div className="verdict good">✔ Correct! {drill.explain && <Markdown text={drill.explain} className="inline" />}</div>}
      {result === false && (
        <div className="verdict bad">
          {text ? "✘ Not quite. " : ""}Answer: <code>{drill.answers[0]}</code>
          {drill.explain && <Markdown text={drill.explain} className="inline" />}
        </div>
      )}
    </div>
  );
}

function ProjectCard({ project, lessonId, block }: { project: Project; lessonId: string; block: number }) {
  const [status, setStatus] = useState<ProjectStatus | null>(null);
  const [msg, setMsg] = useState("");
  const [armed, setArmed] = useState(false);
  const refresh = () => api.projectStatus(lessonId, block).then(setStatus).catch((e) => setMsg(String(e)));
  useEffect(() => {
    refresh();
    setMsg("");
    setArmed(false);
  }, [lessonId, block]);

  const act = async (action: string, command?: string) => {
    try {
      const m = await api.projectAction(lessonId, block, action, command);
      if (m) setMsg(m);
    } catch (e) {
      setMsg("⚠ " + String(e));
    }
    refresh();
  };
  const ready = !!status && status.existing > 0;

  return (
    <div className="card project">
      <div className="card-label">🛠 Project</div>
      <h3>{project.title}</h3>
      <Markdown text={project.description} />
      {status && <div className="path">📁 {status.dir}</div>}
      <div className="btn-row">
        <button className="btn primary" disabled={!status || status.existing >= status.total} onClick={() => act("create")}>
          {!status || status.existing === 0
            ? "📁 Create project"
            : status.existing < status.total
              ? `➕ Add ${status.total - status.existing} new file(s)`
              : "✔ Files ready"}
        </button>
        <button className="btn" disabled={!ready} onClick={() => act("vscode")} title="Needs VS Code's `code` command">
          💻 Open in VS Code
        </button>
        <button className="btn" disabled={!ready} onClick={() => act("folder")}>
          📂 Open folder
        </button>
        {status?.commands.map((c) => (
          <button key={c.command} className="btn" disabled={!ready} onClick={() => act("run", c.command)} title={`Runs \`${c.command}\` in a new terminal window`}>
            {c.label}
          </button>
        ))}
      </div>
      <details>
        <summary>📄 This step's starting files ({project.files.length})</summary>
        {project.files.map(([path, contents]) => (
          <div key={path}>
            <div className="file-name">{path}</div>
            <CodeEditor value={contents} lang={langOfPath(path)} readOnly />
          </div>
        ))}
        <button
          className={"btn small" + (armed ? " danger" : "")}
          disabled={!ready}
          onClick={() => {
            if (armed) {
              setArmed(false);
              act("reset");
            } else setArmed(true);
          }}
        >
          {armed ? "⚠ Click again to overwrite (a backup is kept)" : "↺ Reset project to this step's files"}
        </button>
      </details>
      {msg && <div className="warn">{msg}</div>}
    </div>
  );
}

function langOfPath(path: string) {
  const ext = path.split(".").pop();
  return (({ rs: "rust", py: "python", js: "javascript", ps1: "powershell", psm1: "powershell", sh: "bash", sql: "sql", ts: "typescript", go: "go", cs: "csharp", cpp: "cpp", h: "cpp", hpp: "cpp", html: "html", css: "html" }) as const)[
    ext as "rs"
  ] ?? null;
}
