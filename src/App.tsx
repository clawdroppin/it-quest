// The window: top bar with the main tabs, the page, and the AI helper beside it.
import { useEffect, useState } from "react";
import { AiPanel } from "./AiPanel";
import { Compare } from "./Compare";
import { Home } from "./Home";
import { Review } from "./Review";
import { TrackPage } from "./TrackPage";
import { aiContext, askAi } from "./bus";
import { Page, update, useProgress } from "./progress";
import { goto } from "./nav";

const PAGES: { id: Page; label: string; title: string }[] = [
  { id: "home", label: "🏠 Home", title: "Roadmap, progress and setup" },
  { id: "learn", label: "💻 Languages", title: "Scripting and programming languages, novice to expert" },
  { id: "certs", label: "🎓 IT & Certs", title: "IT foundations, certifications, and the Japan career track" },
  { id: "extras", label: "✨ Extras", title: "Beyond IT: extra courses like GUI design, animation and icons" },
  { id: "projects", label: "🛠 Projects", title: "Real projects to build, for every skill and certification" },
  { id: "compare", label: "⚖ Compare", title: "One concept, every language, side by side" },
  { id: "review", label: "🃏 Review", title: "Flashcards and practice exams" },
];

export function App() {
  const p = useProgress();
  const [aiOpen, setAiOpen] = useState(false);

  // Pages without a lesson give the AI helper a general context.
  useEffect(() => {
    if (p.page === "home" || p.page === "compare" || p.page === "review") {
      aiContext.subject = () => "IT skills and certifications";
      aiContext.current = async () => `Context from the app: I'm on the ${p.page} page of IT Quest.`;
    }
  }, [p.page]);

  // Ctrl + / Ctrl - / Ctrl 0 zoom the whole app.
  useEffect(() => {
    const onKey = (e: KeyboardEvent) => {
      if (!e.ctrlKey) return;
      const z = (n: number) => update((q) => void (q.zoom = Math.min(1.8, Math.max(0.7, Math.round(n * 10) / 10))));
      if (e.key === "=" || e.key === "+") z(p.zoom + 0.1);
      else if (e.key === "-") z(p.zoom - 0.1);
      else if (e.key === "0") z(1);
      else return;
      e.preventDefault();
    };
    window.addEventListener("keydown", onKey);
    return () => window.removeEventListener("keydown", onKey);
  }, [p.zoom]);
  useEffect(() => {
    document.documentElement.style.fontSize = `${15 * p.zoom}px`;
  }, [p.zoom]);

  return (
    <div className={"app" + (aiOpen ? " with-ai" : "")}>
      <header className="topbar">
        <div className="brand">
          <span className="logo">▸_</span> IT Quest
        </div>
        <nav className="pages">
          {PAGES.map((pg) => (
            <button key={pg.id} className={"page-tab" + (p.page === pg.id ? " active" : "")} onClick={() => goto(pg.id)} title={pg.title}>
              {pg.label}
            </button>
          ))}
        </nav>
        <span className="spacer" />
        <button className={"btn" + (aiOpen ? " primary" : "")} onClick={() => (aiOpen ? setAiOpen(false) : askAi.emit(null))}>
          🤖 AI Helper
        </button>
      </header>
      <div className="content">
        <div className="page">
          {p.page === "home" && <Home />}
          {p.page === "learn" && <TrackPage key="learn" view="learn" />}
          {p.page === "certs" && <TrackPage key="certs" view="certs" />}
          {p.page === "extras" && <TrackPage key="extras" view="extras" />}
          {p.page === "projects" && <TrackPage key="projects" view="projects" />}
          {p.page === "compare" && <Compare />}
          {p.page === "review" && <Review />}
        </div>
        <AiPanel open={aiOpen} setOpen={setAiOpen} />
      </div>
    </div>
  );
}
