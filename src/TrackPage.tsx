// The Learn, Certs, Extras and Projects tabs: track picker, module sidebar, lesson, and the code workbench.
import { useEffect, useMemo, useRef, useState } from "react";
import { LEVELS, Lesson, Track, api } from "./api";
import { CERT_CATEGORIES, EXTRA_CATEGORIES, LEARN_CATEGORIES, LessonRef, course, lessonLang, lessonRef, trackProgress } from "./course";
import { LessonView } from "./LessonView";
import { Tab, Workbench, exerciseKey, shorten } from "./Workbench";
import { addUnique, update, useProgress } from "./progress";
import { aiContext, workbenchContext } from "./bus";

type View = "learn" | "certs" | "extras" | "projects";

export function TrackPage({ view }: { view: View }) {
  const p = useProgress();
  const tracks = course().tracks;
  const builds = view === "projects";
  const cats = view === "certs" ? CERT_CATEGORIES : view === "extras" ? EXTRA_CATEGORIES : LEARN_CATEGORIES;

  // Which tracks this tab shows, and which one is selected.
  const shown = useMemo(
    () => (builds ? tracks.filter((t) => t.modules.some((m) => m.build)) : tracks.filter((t) => cats.some((c) => c.id === t.category))),
    [view],
  );
  const filter = p.track[view] ?? (builds ? "all" : shown[0]?.id);
  const selected: Track[] = builds ? (filter === "all" ? shown : shown.filter((t) => t.id === filter)) : shown.filter((t) => t.id === filter);
  const lessons: LessonRef[] = selected.flatMap((t) =>
    t.modules.filter((m) => m.build === builds).flatMap((m) => m.lessons.map((lesson) => ({ track: t, module: m, lesson }))),
  );

  const lastKey = builds ? "projects" : filter;
  const current = lessonRef(p.last[lastKey]);
  const r = current && lessons.some((l) => l.lesson.id === current.lesson.id) ? current : lessons[0];
  const idx = r ? lessons.findIndex((l) => l.lesson.id === r.lesson.id) : -1;

  const [tab, setTab] = useState<Tab>({ kind: "playground" });
  const mainRef = useRef<HTMLDivElement>(null);

  const open = (id: string, t: Tab = { kind: "playground" }) => {
    update((q) => {
      q.last[lastKey] = id;
      addUnique(q.visited, id);
    });
    setTab(t);
    mainRef.current?.scrollTo({ top: 0 });
  };

  // A lesson opens on its first unfinished exercise, or the playground.
  useEffect(() => {
    if (!r) return;
    update((q) => addUnique(q.visited, r.lesson.id));
    const first = r.lesson.exercises.findIndex((_, i) => !p.passed.includes(exerciseKey(r.lesson.id, i)));
    setTab(r.lesson.exercises.length ? { kind: "exercise", index: first < 0 ? 0 : first } : { kind: "playground" });
  }, [r?.lesson.id]);

  const hasCode = !!r && (lessonLang(r) !== null || r.lesson.exercises.length > 0);
  const showEditor = !!r && hasCode && (!builds || p.showEditorInBuilds || r.lesson.exercises.length > 0);

  // What the AI helper is told: where the learner is, plus the editor and project files.
  useEffect(() => {
    if (!r) return;
    aiContext.subject = () => (lessonLang(r) && r.track.lang ? r.track.name.replace(" & Linux", "") : r.track.name);
    aiContext.current = async () => {
      let ctx = `Context from the app (the learner doesn't see this):\nI'm on "${r.track.name}" › "${r.module.title}" › "${r.lesson.title}".\n`;
      const text = r.lesson.blocks
        .flatMap((b) => (b.kind === "text" ? b.segments.map((s) => (s.kind === "md" ? s.text : "")) : []))
        .join("\n");
      ctx += `The lesson starts like this:\n${shorten(text, 1200)}\n`;
      if (showEditor && workbenchContext.current) ctx += workbenchContext.current();
      const pi = r.lesson.blocks.findIndex((b) => b.kind === "project");
      if (pi >= 0) {
        const files = await api.projectSources(r.lesson.id, pi).catch(() => "");
        if (files) ctx += `This step has a project. My files on disk right now:\n${files}`;
      }
      return ctx;
    };
  });

  const setFilter = (id: string) => update((q) => void (q.track[view] = id));

  return (
    <div className="track-page">
      <div className="track-tabs">
        {builds ? (
          <>
            <button className={"ttab" + (filter === "all" ? " active" : "")} onClick={() => setFilter("all")}>
              All projects
            </button>
            {shown.map((t) => (
              <TrackTab key={t.id} t={t} active={filter === t.id} onClick={() => setFilter(t.id)} builds />
            ))}
          </>
        ) : (
          cats.map((c) => {
            const ts = shown.filter((t) => t.category === c.id);
            if (!ts.length) return null;
            return (
              <div key={c.id} className="ttab-group">
                <span className="ttab-cat">{c.name}</span>
                {ts.map((t) => (
                  <TrackTab key={t.id} t={t} active={filter === t.id} onClick={() => setFilter(t.id)} />
                ))}
              </div>
            );
          })
        )}
      </div>

      <div className="track-body">
        <Sidebar view={view} selected={selected} current={r?.lesson.id} open={open} builds={builds} />
        <div className="main" ref={mainRef}>
          {r ? (
            <LessonView
              r={r}
              prev={lessons[idx - 1]}
              next={lessons[idx + 1]}
              open={(id) => open(id)}
              openExercise={(i) => setTab({ kind: "exercise", index: i })}
            />
          ) : (
            <div className="lesson">
              <h1>Coming soon</h1>
              <p className="muted">This track doesn't have lessons yet.</p>
            </div>
          )}
          {builds && r && hasCode && r.lesson.exercises.length === 0 && (
            <label className="check editor-toggle">
              <input type="checkbox" checked={p.showEditorInBuilds} onChange={(e) => update((q) => void (q.showEditorInBuilds = e.target.checked))} />
              Show the code editor here too (projects are written in VS Code)
            </label>
          )}
        </div>
        {showEditor && r && <Workbench r={r} tab={tab} setTab={setTab} />}
      </div>
    </div>
  );
}

function TrackTab({ t, active, onClick, builds = false }: { t: Track; active: boolean; onClick: () => void; builds?: boolean }) {
  const p = useProgress();
  const { done, total } = trackProgress(t, p.completed, builds);
  return (
    <button className={"ttab" + (active ? " active" : "")} onClick={onClick} title={t.blurb}>
      <span>{t.icon}</span> {t.name}
      {total > 0 && <span className="ttab-count">{Math.round((100 * done) / total)}%</span>}
      {total === 0 && <span className="ttab-count">soon</span>}
    </button>
  );
}

function Sidebar({
  view,
  selected,
  current,
  open,
  builds,
}: {
  view: View;
  selected: Track[];
  current: string | undefined;
  open: (id: string) => void;
  builds: boolean;
}) {
  const p = useProgress();
  const [collapsed, setCollapsed] = useState<Record<string, boolean>>({});
  const item = (l: Lesson) => {
    const done = p.completed.includes(l.id);
    const exDone = l.exercises.filter((_, i) => p.passed.includes(exerciseKey(l.id, i))).length;
    return (
      <button key={l.id} className={"side-lesson" + (l.id === current ? " active" : "")} onClick={() => open(l.id)}>
        <span className="side-mark">{done ? "✔" : l.id === current ? "▸" : "·"}</span>
        <span className="side-title">{l.title}</span>
        {l.exercises.length > 0 && (
          <span className="side-ex">
            {exDone}/{l.exercises.length}
          </span>
        )}
      </button>
    );
  };
  return (
    <nav className="sidebar">
      {selected.map((t) => {
        const { done, total } = trackProgress(t, p.completed, builds);
        const modules = t.modules.filter((m) => m.build === builds);
        return (
          <div key={t.id}>
            <div className="side-track">
              <div>
                {t.icon} <b>{t.name}</b>
              </div>
              {!builds && <div className="small muted">{t.blurb}</div>}
              {total > 0 && (
                <div className="bar" title={`${done} of ${total} lessons done`}>
                  <div style={{ width: `${(100 * done) / total}%` }} />
                </div>
              )}
            </div>
            {LEVELS.map((lv) => {
              const ms = modules.filter((m) => m.level === lv.id);
              if (!ms.length) return null;
              return (
                <div key={lv.id}>
                  {!builds && <div className={"level-head lv-" + lv.id}>{lv.name}</div>}
                  {ms.map((m) => {
                    const key = `${view}:${t.id}:${m.title}`;
                    const hasCurrent = m.lessons.some((l) => l.id === current);
                    const isCollapsed = collapsed[key] ?? !hasCurrent;
                    const mDone = m.lessons.filter((l) => p.completed.includes(l.id)).length;
                    return (
                      <div key={m.title} className="side-module">
                        <button className="side-module-head" onClick={() => setCollapsed({ ...collapsed, [key]: !isCollapsed })} title={m.blurb}>
                          <span>{isCollapsed ? "▸" : "▾"}</span>
                          <span className="side-title">{m.title}</span>
                          {builds && <span className={"lv-pill lv-" + m.level}>{m.level}</span>}
                          <span className="side-ex">
                            {mDone}/{m.lessons.length}
                          </span>
                        </button>
                        {!isCollapsed && m.lessons.map(item)}
                      </div>
                    );
                  })}
                </div>
              );
            })}
          </div>
        );
      })}
    </nav>
  );
}
