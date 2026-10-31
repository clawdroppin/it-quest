// Review: spaced-repetition flashcards from the lessons you've opened, and timed practice exams
// built from every quiz in a track.
import { useEffect, useMemo, useRef, useState } from "react";
import { Quiz, Track } from "./api";
import { course, lessonRef } from "./course";
import { Markdown } from "./Markdown";
import { CardState, update, useProgress } from "./progress";

const DAY = 24 * 60 * 60 * 1000;
/** Leitner boxes: how long until a card you knew comes back. */
const INTERVALS = [0, 1, 3, 7, 16, 35].map((d) => d * DAY);

interface DeckCard {
  key: string;
  front: string;
  back: string;
  source: string;
}

function shuffle<T>(a: T[]): T[] {
  const b = [...a];
  for (let i = b.length - 1; i > 0; i--) {
    const j = Math.floor(Math.random() * (i + 1));
    [b[i], b[j]] = [b[j], b[i]];
  }
  return b;
}

export function Review() {
  const [mode, setMode] = useState<"cards" | "exam">("cards");
  return (
    <div className="review">
      <div className="track-tabs">
        <button className={"ttab" + (mode === "cards" ? " active" : "")} onClick={() => setMode("cards")}>
          🃏 Flashcards
        </button>
        <button className={"ttab" + (mode === "exam" ? " active" : "")} onClick={() => setMode("exam")}>
          📝 Practice exams
        </button>
      </div>
      <div className="review-body">{mode === "cards" ? <Cards /> : <Exams />}</div>
    </div>
  );
}

function Cards() {
  const p = useProgress();
  const deck: DeckCard[] = useMemo(() => {
    const out: DeckCard[] = [];
    for (const id of p.visited) {
      const r = lessonRef(id);
      if (!r) continue;
      r.lesson.blocks.forEach((b, bi) => {
        if (b.kind === "flashcards")
          b.cards.forEach((c, ci) => out.push({ key: `${id}~${bi}~${ci}`, ...c, source: `${r.track.icon} ${r.track.name} › ${r.lesson.title}` }));
      });
    }
    return out;
  }, [p.visited]);
  const now = Date.now();
  const due = deck.filter((c) => (p.cards[c.key]?.due ?? 0) <= now);
  const [queue, setQueue] = useState<DeckCard[] | null>(null);
  const [flipped, setFlipped] = useState(false);
  const [doneCount, setDoneCount] = useState(0);

  const answer = (card: DeckCard, knew: boolean) => {
    update((q) => {
      const old: CardState = q.cards[card.key] ?? { box: 0, due: 0 };
      const box = knew ? Math.min(old.box + 1, INTERVALS.length - 1) : 0;
      q.cards[card.key] = { box, due: Date.now() + INTERVALS[box] };
    });
    setFlipped(false);
    setDoneCount((n) => n + 1);
    // A card you didn't know comes back at the end of this session.
    setQueue((qu) => (qu ? [...qu.slice(1), ...(knew ? [] : [card])] : qu));
  };

  if (!queue || queue.length === 0) {
    const boxes = [0, 1, 2, 3, 4, 5].map((b) => deck.filter((c) => (p.cards[c.key]?.box ?? 0) === b).length);
    return (
      <div className="cards-home">
        <h1>Flashcards</h1>
        <p>
          Cards from every lesson you've opened land here. Cards you know come back less and less often (after 1, 3, 7, 16, then 35 days); cards
          you don't know come back right away. A few minutes a day beats hours the night before an exam.
        </p>
        {queue && doneCount > 0 && <div className="callout good-bg">🎉 Session done: {doneCount} cards reviewed.</div>}
        <div className="stats">
          <div>
            <b>{deck.length}</b> cards in your deck
          </div>
          <div>
            <b>{due.length}</b> due now
          </div>
          <div>
            <b>{boxes[4] + boxes[5]}</b> well learned
          </div>
        </div>
        <div className="btn-row">
          <button
            className="btn primary"
            disabled={!due.length}
            onClick={() => {
              setDoneCount(0);
              setQueue(shuffle(due).slice(0, 40));
            }}
          >
            Review {Math.min(due.length, 40)} due cards
          </button>
          <button
            className="btn"
            disabled={!deck.length}
            onClick={() => {
              setDoneCount(0);
              setQueue(shuffle(deck).slice(0, 20));
            }}
          >
            Practice 20 random cards
          </button>
        </div>
        {!deck.length && <p className="muted">Open lessons with flashcards (the cert tracks have lots) and they'll show up here.</p>}
      </div>
    );
  }

  const card = queue[0];
  return (
    <div className="card-session">
      <div className="small muted">
        {queue.length} left · {card.source}
      </div>
      <button className={"big-card" + (flipped ? " flipped" : "")} onClick={() => setFlipped(!flipped)}>
        <Markdown text={flipped ? card.back : card.front} />
        {!flipped && <div className="small muted">click to flip</div>}
      </button>
      {flipped ? (
        <div className="btn-row center">
          <button className="btn danger" onClick={() => answer(card, false)}>
            ✘ Didn't know
          </button>
          <button className="btn good" onClick={() => answer(card, true)}>
            ✔ Knew it
          </button>
        </div>
      ) : (
        <div className="btn-row center">
          <button className="btn primary" onClick={() => setFlipped(true)}>
            Show answer
          </button>
        </div>
      )}
      <button className="btn subtle small" onClick={() => setQueue(null)}>
        End session
      </button>
    </div>
  );
}

interface ExamQ extends Quiz {
  source: string;
}

function quizzesOf(t: Track): ExamQ[] {
  return t.modules.flatMap((m) =>
    m.lessons.flatMap((l) => l.blocks.flatMap((b) => (b.kind === "quiz" ? [{ ...b, source: l.title }] : []))),
  );
}

function Exams() {
  const p = useProgress();
  const tracks = course().tracks.filter((t) => quizzesOf(t).length >= 5);
  const [trackId, setTrackId] = useState(tracks.find((t) => t.category === "cert")?.id ?? tracks[0]?.id);
  const [count, setCount] = useState(20);
  const [timed, setTimed] = useState(true);
  const [exam, setExam] = useState<{ qs: ExamQ[]; answers: (number | null)[]; i: number; start: number; finished: boolean } | null>(null);
  const track = tracks.find((t) => t.id === trackId);

  if (exam) {
    const q = exam.qs[exam.i];
    const limit = exam.qs.length * 75 * 1000;
    const finish = () => {
      const score = exam.qs.filter((q, i) => exam.answers[i] === q.correct).length;
      update((x) => void x.exams.push({ track: trackId!, score, total: exam.qs.length, date: Date.now() }));
      setExam({ ...exam, finished: true });
    };
    if (exam.finished) {
      const score = exam.qs.filter((q, i) => exam.answers[i] === q.correct).length;
      const pct = Math.round((100 * score) / exam.qs.length);
      return (
        <div className="exam">
          <h1>
            Result: {score} / {exam.qs.length} ({pct}%)
          </h1>
          <p className={pct >= 80 ? "good" : pct >= 65 ? "warn" : "bad"}>
            {pct >= 80
              ? "Great: you're in passing territory. Keep it there across several exams before booking the real one."
              : pct >= 65
                ? "Close. Review the explanations below and the lessons they come from."
                : "Not yet. Go back to the lessons for the questions you missed, then try again."}
          </p>
          {exam.qs.map((q, i) => (
            <div key={i} className={"card quiz " + (exam.answers[i] === q.correct ? "right-card" : "wrong-card")}>
              <div className="card-label">
                {exam.answers[i] === q.correct ? "✔" : "✘"} Question {i + 1} · from “{q.source}”
              </div>
              <Markdown text={q.question} />
              <div className="small">
                Your answer: {exam.answers[i] === null ? "(none)" : <Markdown text={q.options[exam.answers[i]!]} className="inline" />}
              </div>
              {exam.answers[i] !== q.correct && (
                <div className="small good">
                  Correct: <Markdown text={q.options[q.correct]} className="inline" />
                </div>
              )}
              {q.explain && <Markdown text={q.explain} className="small muted" />}
            </div>
          ))}
          <button className="btn primary" onClick={() => setExam(null)}>
            Back
          </button>
        </div>
      );
    }
    return (
      <div className="exam">
        <div className="exam-head">
          <span>
            Question {exam.i + 1} of {exam.qs.length}
          </span>
          {timed && <Timer start={exam.start} limit={limit} onEnd={finish} />}
        </div>
        <div className="card quiz">
          <Markdown text={q.question} />
          <div className="options">
            {q.options.map((o, oi) => (
              <button
                key={oi}
                className={"option" + (exam.answers[exam.i] === oi ? " chosen" : "")}
                onClick={() => {
                  const answers = [...exam.answers];
                  answers[exam.i] = oi;
                  setExam({ ...exam, answers });
                }}
              >
                <Markdown text={o} className="inline" />
              </button>
            ))}
          </div>
        </div>
        <div className="btn-row">
          <button className="btn" disabled={exam.i === 0} onClick={() => setExam({ ...exam, i: exam.i - 1 })}>
            ← Back
          </button>
          {exam.i < exam.qs.length - 1 ? (
            <button className="btn primary" onClick={() => setExam({ ...exam, i: exam.i + 1 })}>
              Next →
            </button>
          ) : (
            <button className="btn good" onClick={finish}>
              Finish exam
            </button>
          )}
          <span className="small muted">{exam.answers.filter((a) => a !== null).length} answered</span>
        </div>
      </div>
    );
  }

  const history = p.exams.filter((e) => e.track === trackId).slice(-8).reverse();
  return (
    <div className="exam">
      <h1>Practice exams</h1>
      <p>
        A random mix of every quiz question in a track, without the answers shown until the end. Timed mode gives you 75 seconds per question,
        about the pace of the real CompTIA exams.
      </p>
      <div className="exam-setup">
        <label>
          Track
          <select value={trackId} onChange={(e) => setTrackId(e.target.value)}>
            {tracks.map((t) => (
              <option key={t.id} value={t.id}>
                {t.icon} {t.name} ({quizzesOf(t).length} questions)
              </option>
            ))}
          </select>
        </label>
        <label>
          Questions
          <select value={count} onChange={(e) => setCount(+e.target.value)}>
            {[10, 20, 30, 50, 90].map((n) => (
              <option key={n} value={n}>
                {n}
              </option>
            ))}
          </select>
        </label>
        <label className="check">
          <input type="checkbox" checked={timed} onChange={(e) => setTimed(e.target.checked)} /> Timed
        </label>
        <button
          className="btn primary"
          disabled={!track}
          onClick={() => {
            const qs = shuffle(quizzesOf(track!)).slice(0, count);
            setExam({ qs, answers: qs.map(() => null), i: 0, start: Date.now(), finished: false });
          }}
        >
          Start
        </button>
      </div>
      {history.length > 0 && (
        <>
          <h3>Your recent results</h3>
          <ul>
            {history.map((h, i) => (
              <li key={i}>
                {new Date(h.date).toLocaleDateString()}: {h.score}/{h.total} ({Math.round((100 * h.score) / h.total)}%)
              </li>
            ))}
          </ul>
        </>
      )}
    </div>
  );
}

function Timer({ start, limit, onEnd }: { start: number; limit: number; onEnd: () => void }) {
  const [now, setNow] = useState(Date.now());
  const ended = useRef(false);
  useEffect(() => {
    const t = setInterval(() => setNow(Date.now()), 1000);
    return () => clearInterval(t);
  }, []);
  const left = Math.max(0, start + limit - now);
  useEffect(() => {
    if (left === 0 && !ended.current) {
      ended.current = true;
      onEnd();
    }
  }, [left]);
  const m = Math.floor(left / 60000);
  const s = Math.floor((left % 60000) / 1000);
  return (
    <span className={left < 60000 ? "bad" : "muted"}>
      ⏱ {m}:{String(s).padStart(2, "0")}
    </span>
  );
}
