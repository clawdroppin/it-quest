// The start page: continue where you left off, the career roadmap, every track, and setup status.
import { useEffect, useState } from "react";
import { ToolStatus, api } from "./api";
import { CERT_CATEGORIES, LEARN_CATEGORIES, course, lessonRef, trackById, trackProgress } from "./course";
import { importRustQuest, update, useProgress } from "./progress";
import { openLesson, openTrack } from "./nav";

/** The path from zero to an IT job, as stages of tracks. Tracks that don't exist are skipped. */
const ROADMAP: { stage: string; role: string; tracks: string[]; note: string }[] = [
  {
    stage: "1 · Foundations",
    role: "IT Support / Help Desk",
    tracks: ["foundations", "python", "powershell", "html", "aplus"],
    note: "How computers, operating systems and networks work, troubleshooting, your first scripts and web pages. Target: CompTIA A+.",
  },
  {
    stage: "2 · Systems & Networks",
    role: "Junior SysAdmin / NOC Technician",
    tracks: ["bash", "networkplus", "linux", "sql"],
    note: "Linux and the command line, networking in depth, databases. Target: CompTIA Network+ and Linux+.",
  },
  {
    stage: "3 · Security & Cloud",
    role: "SysAdmin / Cloud Support / SOC Analyst",
    tracks: ["securityplus", "cloud", "aws", "azure"],
    note: "Securing systems, and running them in AWS and Azure. Target: Security+, AWS Cloud Practitioner, AZ-900.",
  },
  {
    stage: "4 · Specialize",
    role: "Network / Cloud / DevOps Engineer",
    tracks: ["ccna", "devops", "itil", "javascript", "typescript", "go", "csharp", "cpp", "rust"],
    note: "Pick a direction: networking (CCNA), cloud and DevOps (Go is their language), or development (TypeScript, C#, C++, Rust). ITIL helps in any IT service job.",
  },
  {
    stage: "5 · Senior",
    role: "Senior Engineer / Architect / Security Lead",
    tracks: ["ccnp", "gcp", "cka", "terraform", "cissp", "pmp"],
    note: "Professional-level certifications: enterprise networks, cloud architecture, Kubernetes, infrastructure as code, security leadership and project management.",
  },
  {
    stage: "🇯🇵 Japan",
    role: "IT job in Japan",
    tracks: ["japanese", "ipa-fe", "japan-career"],
    note: "Japanese for the workplace (aim for JLPT N3–N2), Japan's own IT exam (基本情報技術者), visas, résumés and interviews.",
  },
];

export function Home() {
  const p = useProgress();
  const [tools, setTools] = useState<ToolStatus[] | null>(null);
  const [rq, setRq] = useState<{ path: string; data: any } | null>(null);
  const tracks = course().tracks;

  useEffect(() => {
    api.tools().then(setTools);
    if (!p.rustQuestAsked) api.findRustQuest().then((found) => setRq(found[0] ?? null));
  }, []);

  const recent = Object.values(p.last)
    .map((id) => lessonRef(id))
    .filter((r) => !!r)
    .slice(0, 4);
  const totals = tracks.reduce(
    (a, t) => {
      const x = trackProgress(t, p.completed);
      return { done: a.done + x.done, total: a.total + x.total };
    },
    { done: 0, total: 0 },
  );
  const exercises = p.passed.length;

  return (
    <div className="home">
      <section className="hero">
        <h1>IT Quest</h1>
        <p>
          Learn the skills for a career in IT, from zero: scripting and programming languages, networking, security, cloud, the certifications
          employers ask for, and the path to an IT job in Japan. Every lesson has examples you can run, quizzes, and practice that's checked
          for you. A local AI helper is there when you're stuck.
        </p>
        <div className="stats">
          <div>
            <b>{totals.done}</b> of {totals.total} lessons done
          </div>
          <div>
            <b>{exercises}</b> exercises passed
          </div>
          <div>
            <b>{p.exams.length}</b> practice exams taken
          </div>
        </div>
      </section>

      {rq && !p.rustQuestAsked && (
        <section className="callout import">
          <b>🦀 Found your Rust Quest progress</b>
          <div className="small muted">{rq.path}</div>
          <p>
            Bring it into the Rust track here? Your completed lessons, passed exercises, code and AI helper settings come along. The old file is
            not changed.
          </p>
          <div className="btn-row">
            <button className="btn primary" onClick={() => importRustQuest(rq.data)}>
              Import it
            </button>
            <button className="btn subtle" onClick={() => update((q) => void (q.rustQuestAsked = true))}>
              No thanks
            </button>
          </div>
        </section>
      )}

      {recent.length > 0 && (
        <section>
          <h2>Continue</h2>
          <div className="continue">
            {recent.map((r) => (
              <button key={r!.lesson.id} className="cont-card" onClick={() => openLesson(r!.lesson.id)}>
                <span className="small muted">
                  {r!.track.icon} {r!.track.name} · {r!.module.title}
                </span>
                <b>{r!.lesson.title}</b>
              </button>
            ))}
          </div>
        </section>
      )}

      <section>
        <h2>Your roadmap to an IT job</h2>
        <div className="roadmap">
          {ROADMAP.map((s) => (
            <div key={s.stage} className="stage">
              <div className="stage-head">
                <b>{s.stage}</b>
                <span className="role">→ {s.role}</span>
              </div>
              <p className="small">{s.note}</p>
              <div className="chips">
                {s.tracks
                  .map((id) => trackById(id))
                  .filter((t) => !!t)
                  .map((t) => {
                    const x = trackProgress(t!, p.completed);
                    return (
                      <button key={t!.id} className="chip" onClick={() => openTrack(t!.id)}>
                        {t!.icon} {t!.name} {x.total > 0 && <span className="muted">{Math.round((100 * x.done) / x.total)}%</span>}
                      </button>
                    );
                  })}
              </div>
            </div>
          ))}
        </div>
      </section>

      {[...LEARN_CATEGORIES, ...CERT_CATEGORIES].map((c) => {
        const ts = tracks.filter((t) => t.category === c.id);
        if (!ts.length) return null;
        return (
          <section key={c.id}>
            <h2>{c.name}</h2>
            <div className="track-grid">
              {ts.map((t) => {
                const x = trackProgress(t, p.completed);
                const lessons = t.modules.filter((m) => !m.build).reduce((n, m) => n + m.lessons.length, 0);
                const builds = t.modules.filter((m) => m.build).length;
                return (
                  <button key={t.id} className="track-card" onClick={() => openTrack(t.id)}>
                    <div className="tc-head">
                      <span className="tc-icon">{t.icon}</span>
                      <b>{t.name}</b>
                    </div>
                    <div className="small muted">{t.blurb}</div>
                    <div className="small">
                      {lessons} lessons{builds > 0 && ` · ${builds} projects`}
                    </div>
                    <div className="bar">
                      <div style={{ width: x.total ? `${(100 * x.done) / x.total}%` : 0 }} />
                    </div>
                  </button>
                );
              })}
            </div>
          </section>
        );
      })}

      <section>
        <h2>Setup</h2>
        <div className="tools">
          {!tools && <span className="muted">Checking which languages are installed…</span>}
          {tools?.map((t) => (
            <div key={t.lang} className={"tool " + (t.ok ? "ok" : "missing")} title={t.info}>
              <b>
                {t.ok ? "✔" : "✘"} {t.name}
              </b>
              <span className="small">{t.info}</span>
            </div>
          ))}
        </div>
        <p className="small muted">
          Your code runs with the programs installed on this computer. Projects are created in <code>{course().projects_root}</code>. Progress is
          saved next to the app.
        </p>
      </section>
    </div>
  );
}
