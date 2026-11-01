// logic.js: everything the app knows about habits, with no DOM and no storage.
// The page uses it as window.HabitLogic; Node's test runner loads it with require().
// Every function returns a new state instead of changing the old one, so render() can
// trust that the state it was given never changes behind its back.
(function (root) {
  "use strict";

  const VERSION = 2;

  // The local calendar day of a Date, as "YYYY-MM-DD" (not toISOString(), which is UTC).
  function dayKey(date) {
    const y = date.getFullYear();
    const m = String(date.getMonth() + 1).padStart(2, "0");
    const d = String(date.getDate()).padStart(2, "0");
    return `${y}-${m}-${d}`;
  }

  // "YYYY-MM-DD" -> a Date at local midnight.
  function parseDay(key) {
    const [y, m, d] = key.split("-").map(Number);
    return new Date(y, m - 1, d);
  }

  // The day key `n` days after `key` (n may be negative). Uses the calendar, not 24-hour
  // steps, so a daylight-saving change can't skip or repeat a day.
  function addDays(key, n) {
    const date = parseDay(key);
    date.setDate(date.getDate() + n);
    return dayKey(date);
  }

  function emptyState() {
    return { version: VERSION, nextId: 1, habits: [] };
  }

  // Adds a habit called `name` (trimmed). Throws Error("Give the habit a name") for an empty
  // name and Error(`You already have "<name>"`) when one with the same name exists, ignoring case.
  function addHabit(state, name) {
    const clean = String(name).trim();
    if (!clean) throw new Error("Give the habit a name");
    if (state.habits.some((h) => h.name.toLowerCase() === clean.toLowerCase())) {
      throw new Error(`You already have "${clean}"`);
    }
    const habit = { id: state.nextId, name: clean, done: [] };
    return { ...state, nextId: state.nextId + 1, habits: [...state.habits, habit] };
  }

  function removeHabit(state, id) {
    return { ...state, habits: state.habits.filter((h) => h.id !== id) };
  }

  // Marks `day` done for habit `id`, or un-marks it if it was done. `done` stays sorted
  // (oldest first) and never has duplicates. Other habits are untouched. Unknown ids change nothing.
  function toggleDay(state, id, day) {
    throw new Error("TODO: not done yet");
  }

  // The `n` day keys ending with `today`, oldest first: lastNDays("2026-11-01", 3)
  // is ["2026-10-30", "2026-10-31", "2026-11-01"].
  function lastNDays(today, n) {
    throw new Error("TODO: not done yet");
  }

  // How many days in a row the habit has been done, counting back from today. If today
  // isn't done yet, the streak still counts up to yesterday (the day isn't over).
  function streak(habit, today) {
    throw new Error("TODO: not done yet");
  }

  // The percentage (0-100, rounded) of the last `days` days (ending today) that are done.
  function completionRate(habit, today, days) {
    throw new Error("TODO: not done yet");
  }

  // Reads saved text (from localStorage or an imported file) into a state.
  // null or "" -> an empty state. Version 1 stored { habits: [{ name, days: {"YYYY-MM-DD": true} }] }
  // without ids; it's upgraded. Anything else that doesn't look right throws Error("Not a habit file").
  function load(text) {
    if (!text) return emptyState();
    let data;
    try {
      data = JSON.parse(text);
    } catch {
      throw new Error("Not a habit file");
    }
    if (!data || !Array.isArray(data.habits)) throw new Error("Not a habit file");
    if (data.version === undefined || data.version === 1) {
      const habits = data.habits.map((h, i) => ({
        id: i + 1,
        name: String(h.name),
        done: Object.keys(h.days || {}).filter((d) => h.days[d]).sort(),
      }));
      return { version: VERSION, nextId: habits.length + 1, habits };
    }
    if (data.version !== VERSION) throw new Error("Not a habit file");
    const valid = data.habits.every(
      (h) => Number.isInteger(h.id) && typeof h.name === "string" && Array.isArray(h.done) &&
        h.done.every((d) => /^\d{4}-\d{2}-\d{2}$/.test(d)),
    );
    if (!valid || !Number.isInteger(data.nextId)) throw new Error("Not a habit file");
    return data;
  }

  function save(state) {
    return JSON.stringify(state);
  }

  const api = { VERSION, dayKey, parseDay, addDays, emptyState, addHabit, removeHabit, toggleDay,
    lastNDays, streak, completionRate, load, save };
  if (typeof module === "object" && module.exports) module.exports = api;
  else root.HabitLogic = api;
})(globalThis);
