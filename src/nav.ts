// Moving around the app: everything is stored in progress, so the app reopens where you were.
import { lessonRef, trackById, viewOf } from "./course";
import { Page, addUnique, update } from "./progress";

export function goto(page: Page) {
  update((q) => void (q.page = page));
}

/** Opens a lesson in the tab it belongs to (Learn, Certs, Extras or Projects). */
export function openLesson(id: string) {
  const r = lessonRef(id);
  if (!r) return;
  update((q) => {
    addUnique(q.visited, id);
    if (r.module.build) {
      q.page = "projects";
      q.last.projects = id;
    } else {
      const view = viewOf(r.track);
      q.page = view;
      q.track[view] = r.track.id;
      q.last[r.track.id] = id;
    }
  });
}

/** Opens a track in its tab, at the lesson the learner was last on. */
export function openTrack(trackId: string) {
  const t = trackById(trackId);
  if (!t) return;
  update((q) => {
    const view = viewOf(t);
    q.page = view;
    q.track[view] = t.id;
  });
}
