// Side by side: one concept, every language. Built from the lessons' @@concepts tags.
import { LANG_NAMES, Lesson, Segment, Track } from "./api";
import { course, isLearnTrack } from "./course";
import { Segments } from "./LessonView";
import { update, useProgress } from "./progress";
import { openLesson } from "./nav";

/** The first runnable example of a lesson, or its first example of any kind. */
function firstExample(l: Lesson): Segment | undefined {
  const segs = l.blocks.flatMap((b) => (b.kind === "text" ? b.segments : [])).filter((s) => s.kind === "code");
  return segs.find((s) => s.kind === "code" && s.runnable) ?? segs[0];
}

export function Compare() {
  const p = useProgress();
  const { tracks, concepts } = course();
  const langTracks = tracks.filter((t) => isLearnTrack(t) && t.lang);
  const lessonsFor = (t: Track, c: string) => t.modules.filter((m) => !m.build).flatMap((m) => m.lessons.filter((l) => l.concepts.includes(c)));
  const counts = new Map(concepts.map(([id]) => [id, langTracks.filter((t) => lessonsFor(t, id).length > 0).length]));
  const concept = p.track.compare ?? concepts[0][0];
  const columns = langTracks.filter((t) => lessonsFor(t, concept).length > 0);

  return (
    <div className="compare">
      <nav className="sidebar">
        <div className="side-track">
          <b>⚖ Compare languages</b>
          <div className="small muted">One idea, every language, side by side. Learning a second language is much faster when you see what stays the same.</div>
        </div>
        {concepts.map(([id, name]) => (
          <button
            key={id}
            className={"side-lesson" + (id === concept ? " active" : "")}
            disabled={!counts.get(id)}
            onClick={() => update((q) => void (q.track.compare = id))}
          >
            <span className="side-title">{name}</span>
            <span className="side-ex">{counts.get(id) || "–"}</span>
          </button>
        ))}
      </nav>
      <div className="compare-cols">
        {columns.length === 0 && <p className="muted">No lessons are tagged with this concept yet.</p>}
        {columns.map((t) => (
          <div key={t.id} className="compare-col">
            <h3>
              {t.icon} {t.name}
            </h3>
            {lessonsFor(t, concept).map((l) => {
              const ex = firstExample(l);
              return (
                <div key={l.id} className="compare-item">
                  <button className="link" onClick={() => openLesson(l.id)}>
                    {p.completed.includes(l.id) ? "✔ " : ""}
                    {l.title} →
                  </button>
                  {ex && <Segments segments={[ex]} />}
                </div>
              );
            })}
            <div className="small muted">{t.lang && LANG_NAMES[t.lang]}</div>
          </div>
        ))}
      </div>
    </div>
  );
}
