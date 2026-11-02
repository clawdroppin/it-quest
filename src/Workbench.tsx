// The right-hand panel: exercise tabs and a playground, the code editor, Run/Check, and output.
import { useEffect, useRef, useState } from "react";
import { Exercise, LANG_NAMES, Lang, RunResult, api } from "./api";
import { CodeEditor } from "./Editor";
import { Markdown } from "./Markdown";
import { LessonRef, lessonLang } from "./course";
import { addUnique, getProgress, update, useProgress } from "./progress";
import { askAi, tryCode, workbenchContext, workbenchControl } from "./bus";

export type Tab = { kind: "exercise"; index: number } | { kind: "playground" };

const PLAYGROUND: Record<Lang, string> = {
  rust: 'fn main() {\n    println!("Playground: try anything here!");\n}',
  python: 'print("Playground: try anything here!")',
  javascript: 'console.log("Playground: try anything here!");',
  powershell: 'Write-Output "Playground: try anything here!"',
  bash: 'echo "Playground: try anything here!"',
  sql: "-- Playground: try anything here!\nSELECT 'hello' AS greeting, 6 * 7 AS answer;",
  typescript: 'const message: string = "Playground: try anything here!";\nconsole.log(message);',
  go: 'package main\n\nimport "fmt"\n\nfunc main() {\n\tfmt.Println("Playground: try anything here!")\n}',
  csharp: 'Console.WriteLine("Playground: try anything here!");',
  cpp: '#include <iostream>\n\nint main() {\n    std::cout << "Playground: try anything here!\\n";\n}',
  html: "<!doctype html>\n<html>\n<head>\n  <style>\n    body { font-family: sans-serif; padding: 1rem; }\n    h1 { color: teal; }\n  </style>\n</head>\n<body>\n  <h1>Playground</h1>\n  <p>Try anything here!</p>\n</body>\n</html>",
};

export const exerciseKey = (lessonId: string, i: number) => `${lessonId}#${i}`;

export function shorten(text: string, max: number) {
  text = text.trim();
  return text.length <= max ? text : text.slice(0, max) + "…";
}

interface Props {
  r: LessonRef;
  tab: Tab;
  setTab: (t: Tab) => void;
}

export function Workbench({ r, tab, setTab }: Props) {
  const p = useProgress();
  const lang = lessonLang(r);
  const { lesson } = r;
  const [stdin, setStdin] = useState("");
  const [output, setOutput] = useState<{ result: RunResult; check: boolean } | null>(null);
  const [running, setRunning] = useState<null | "run" | "check">(null);
  const [show, setShow] = useState<null | "hint" | "solution">(null);
  const [armReset, setArmReset] = useState(false);
  const runId = useRef(0);

  const ex: Exercise | null = tab.kind === "exercise" ? (lesson.exercises[tab.index] ?? null) : null;
  // Exercises in a track without a language run in the lesson's language, or Python.
  const runLang: Lang = lang ?? "python";
  const key = ex ? exerciseKey(lesson.id, (tab as { index: number }).index) : null;
  const code = ex ? (p.code[key!] ?? ex.starter) : (p.playground[runLang] ?? PLAYGROUND[runLang]);

  useEffect(() => {
    setOutput(null);
    setShow(null);
    setStdin(ex?.stdin ?? "");
    runId.current++;
    setRunning(null);
  }, [lesson.id, tab.kind, (tab as { index?: number }).index]);

  const setCode = (c: string) =>
    update((q) => {
      if (key) q.code[key] = c;
      else q.playground[runLang] = c;
    });

  async function run(check: boolean, codeOverride?: string): Promise<RunResult | null> {
    const id = ++runId.current;
    setRunning(check ? "check" : "run");
    try {
      const result = await api.run({
        lang: runLang,
        code: codeOverride ?? code,
        stdin,
        // The playground gets the lesson's sample data too (the SQL track's database).
        setup: ex ? ex.setup : lesson.setup,
        check: check && ex ? { expected: ex.expected, tests: ex.tests, requires: ex.requires } : null,
      });
      if (id !== runId.current) return null;
      setOutput({ result, check });
      if (check && result.check?.passed && key) update((q) => addUnique(q.passed, key));
      return result;
    } finally {
      if (id === runId.current) setRunning(null);
    }
  }

  // "Try it" in the lesson: load the example into the playground and run it.
  const runRef = useRef(run);
  runRef.current = run;
  useEffect(
    () =>
      tryCode.on(({ lang: l, code: c }) => {
        update((q) => {
          q.playground[l] = c;
        });
        setTab({ kind: "playground" });
        // Wait for the tab switch before running.
        setTimeout(() => runRef.current(false, c), 0);
      }),
    [],
  );

  // What the AI helper is told about this panel.
  useEffect(() => {
    workbenchContext.current = () => {
      const q = getProgress();
      const fence = (lang: string, text: string, max: number) => "```" + lang + "\n" + shorten(text, max) + "\n```\n";
      let ctx = "";
      if (ex) ctx += `Exercise "${ex.title}":\n${shorten(ex.prompt, 1500)}\n`;
      const current = ex ? (q.code[key!] ?? ex.starter) : (q.playground[runLang] ?? "");
      ctx += `My ${LANG_NAMES[runLang]} code in the editor${ex ? "" : " (playground)"}:\n` + fence(runLang, current, 6000);
      const res = output?.result;
      if (res) {
        if (!res.compileOk) ctx += "The compiler said:\n" + fence("text", res.compilerOutput, 3000);
        else {
          if (res.check && !res.check.passed) ctx += `The checker said:\n${shorten(res.check.problems.join("\n"), 2000)}\n`;
          if (res.stdout) ctx += "My program printed:\n" + fence("text", res.stdout, 1500);
          if (res.stderr.trim()) ctx += "Error output:\n" + fence("text", res.stderr, 1500);
        }
      }
      return ctx;
    };
  });
  // What the AI helper may do here (it reads the code fresh, since its answer arrives later).
  useEffect(() => {
    workbenchControl.current = {
      lang: runLang,
      canCheck: !!ex,
      getCode: () => {
        const q = getProgress();
        return ex ? (q.code[key!] ?? ex.starter) : (q.playground[runLang] ?? PLAYGROUND[runLang]);
      },
      setCode,
      run: (check) => {
        const q = getProgress();
        return run(check && !!ex, ex ? (q.code[key!] ?? ex.starter) : (q.playground[runLang] ?? PLAYGROUND[runLang]));
      },
    };
  });
  useEffect(
    () => () => {
      workbenchContext.current = null;
      workbenchControl.current = null;
    },
    [],
  );

  const passed = (i: number) => p.passed.includes(exerciseKey(lesson.id, i));

  return (
    <div className="workbench">
      <div className="wb-tabs">
        {lesson.exercises.map((e, i) => (
          <button
            key={i}
            className={"wb-tab" + (tab.kind === "exercise" && tab.index === i ? " active" : "")}
            onClick={() => setTab({ kind: "exercise", index: i })}
            title={e.title}
          >
            {passed(i) ? "✅" : "📝"} {i + 1}. {e.title}
          </button>
        ))}
        <button className={"wb-tab" + (tab.kind === "playground" ? " active" : "")} onClick={() => setTab({ kind: "playground" })}>
          🧪 Playground
        </button>
      </div>

      {ex && (
        <div className="wb-task">
          <h3>
            {passed((tab as { index: number }).index) && <span className="good">✅ </span>}
            {ex.title}
          </h3>
          <Markdown text={ex.prompt} />
        </div>
      )}
      {!ex && (
        <div className="wb-task muted">
          A free space to experiment in {LANG_NAMES[runLang]}. Nothing here is checked. <kbd>Ctrl</kbd>+<kbd>Enter</kbd> runs it.
        </div>
      )}

      <div className="wb-editor">
        <CodeEditor value={code} lang={runLang} onChange={setCode} onRun={() => run(false)} onCheck={ex ? () => run(true) : undefined} />
      </div>

      {(ex?.stdin || tab.kind === "playground") && (
        <details className="wb-stdin" open={!!ex?.stdin}>
          <summary>Input for the program (what you'd type)</summary>
          <textarea value={stdin} onChange={(e) => setStdin(e.target.value)} rows={2} spellCheck={false} />
        </details>
      )}

      <div className="wb-controls">
        <button className="btn primary" disabled={!!running} onClick={() => run(false)} title="Ctrl+Enter or F5">
          {running === "run" ? "Running…" : "▶ Run"}
        </button>
        {ex && (
          <button className="btn good" disabled={!!running} onClick={() => run(true)} title="Ctrl+Shift+Enter">
            {running === "check" ? "Checking…" : "✔ Check"}
          </button>
        )}
        {ex?.hint && (
          <button className="btn" onClick={() => setShow(show === "hint" ? null : "hint")}>
            💡 Hint
          </button>
        )}
        {ex && (
          <button className="btn" onClick={() => setShow(show === "solution" ? null : "solution")}>
            👀 Solution
          </button>
        )}
        <button
          className={"btn subtle" + (armReset ? " danger" : "")}
          onBlur={() => setArmReset(false)}
          onClick={() => {
            if (armReset) {
              setCode(ex ? ex.starter : PLAYGROUND[runLang]);
              setArmReset(false);
            } else setArmReset(true);
          }}
          title="Put the starting code back"
        >
          {armReset ? "⚠ Click again: lose your changes" : "↺ Reset"}
        </button>
      </div>

      {show === "hint" && ex && (
        <div className="callout hint">
          <Markdown text={ex.hint} />
        </div>
      )}
      {show === "solution" && ex && (
        <div className="callout">
          <p className="muted">One possible solution. Try to understand it, then write it yourself.</p>
          <CodeEditor value={ex.solution} lang={runLang} readOnly />
        </div>
      )}

      <Output output={output} running={!!running} html={runLang === "html" ? code : null} />
    </div>
  );
}

function Output({ output, running, html }: { output: { result: RunResult; check: boolean } | null; running: boolean; html: string | null }) {
  if (!output) return <div className="output empty muted">{running ? "Working…" : "Output appears here."}</div>;
  const r = output.result;
  const failed = r.internalError || !r.compileOk || (r.check && !r.check.passed) || (r.exitCode !== 0 && !r.timedOut) || r.timedOut;
  return (
    <div className="output">
      {r.internalError && <div className="out-block bad">⚠ {r.internalError}</div>}
      {r.check &&
        (r.check.passed ? (
          <div className="out-block good-bg">🎉 Passed! Well done.</div>
        ) : (
          <div className="out-block bad">
            {r.check.problems.map((pr, i) => (
              <pre key={i}>{pr}</pre>
            ))}
          </div>
        ))}
      {!r.compileOk && r.compilerOutput && (
        <>
          <div className="out-label">Compiler</div>
          <pre className="out-pre bad-text">{r.compilerOutput}</pre>
        </>
      )}
      {r.compileOk && (
        <>
          {html !== null ? (
            <>
              <div className="out-label">Preview</div>
              {/* No scripts and no same-origin access: the page can't reach the app. */}
              <iframe className="html-preview" sandbox="" srcDoc={html} title="Preview" />
            </>
          ) : (
            <>
              <div className="out-label">Output {r.timedOut && <span className="warn">(stopped: took too long)</span>}</div>
              <pre className="out-pre">{r.stdout || <span className="muted">(nothing printed)</span>}</pre>
            </>
          )}
          {r.stderr.trim() && (
            <>
              <div className="out-label">{html !== null ? "Tag problems" : "Errors / warnings"}</div>
              <pre className="out-pre bad-text">{r.stderr}</pre>
            </>
          )}
        </>
      )}
      {failed && !r.internalError && (
        <button className="btn subtle small" onClick={() => askAi.emit("Can you explain the error or failed check I'm getting, and how I could fix it?")}>
          🤖 Ask the AI helper about this
        </button>
      )}
    </div>
  );
}
