// The course, loaded once at start, with lookups the UI needs often.
import { Category, CourseData, Lang, Lesson, Module, Track } from "./api";

export interface LessonRef {
  track: Track;
  module: Module;
  lesson: Lesson;
}

let data: CourseData;
const byId = new Map<string, LessonRef>();

export function setCourse(d: CourseData) {
  data = d;
  for (const track of d.tracks)
    for (const module of track.modules) for (const lesson of module.lessons) byId.set(lesson.id, { track, module, lesson });
}

export const course = () => data;
export const lessonRef = (id: string | undefined) => (id ? byId.get(id) : undefined);
export const trackById = (id: string) => data.tracks.find((t) => t.id === id);

/** The language a lesson's code runs in. */
export const lessonLang = (r: LessonRef): Lang | null => r.lesson.lang ?? r.track.lang;

export const LEARN_CATEGORIES: { id: Category; name: string }[] = [
  { id: "scripting", name: "Scripting" },
  { id: "programming", name: "Programming" },
  { id: "data", name: "Data" },
];
export const CERT_CATEGORIES: { id: Category; name: string }[] = [
  { id: "foundations", name: "Foundations" },
  { id: "cert", name: "Certifications" },
  { id: "career", name: "Career & Japan" },
];

export const EXTRA_CATEGORIES: { id: Category; name: string }[] = [{ id: "extra", name: "Extras" }];

export const isLearnTrack = (t: Track) => LEARN_CATEGORIES.some((c) => c.id === t.category);

/** The tab a track's lessons are shown in. */
export const viewOf = (t: Track): "learn" | "certs" | "extras" =>
  isLearnTrack(t) ? "learn" : t.category === "extra" ? "extras" : "certs";

/** The lessons of a track in a view: course lessons, or the Projects tab's builds. */
export function viewLessons(track: Track, builds: boolean): Lesson[] {
  return track.modules.filter((m) => m.build === builds).flatMap((m) => m.lessons);
}

export function trackProgress(track: Track, completed: string[], builds = false) {
  const lessons = viewLessons(track, builds);
  return { done: lessons.filter((l) => completed.includes(l.id)).length, total: lessons.length };
}
