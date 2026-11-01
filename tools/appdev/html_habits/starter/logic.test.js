// Run: node --test     (needs Node.js 20 or newer)
const test = require("node:test");
const assert = require("node:assert/strict");
const L = require("./logic.js");

const T = "2026-11-01";

test("day keys use the local calendar", () => {
  assert.equal(L.dayKey(new Date(2026, 0, 5, 23, 59)), "2026-01-05");
  assert.equal(L.addDays("2026-03-01", -1), "2026-02-28");
  assert.equal(L.addDays("2024-02-28", 1), "2024-02-29");
  assert.equal(L.addDays("2026-12-31", 1), "2027-01-01");
});

test("adding habits", () => {
  let s = L.addHabit(L.emptyState(), "  Stretch ");
  s = L.addHabit(s, "Read 20 pages");
  assert.deepEqual(s.habits.map((h) => [h.id, h.name]), [[1, "Stretch"], [2, "Read 20 pages"]]);
  assert.equal(s.nextId, 3);
  assert.throws(() => L.addHabit(s, "   "), /Give the habit a name/);
  assert.throws(() => L.addHabit(s, "stretch"), /You already have "stretch"/);
});

test("adding does not change the old state", () => {
  const before = L.emptyState();
  L.addHabit(before, "Walk");
  assert.equal(before.habits.length, 0);
});

test("toggling days", () => {
  let s = L.addHabit(L.addHabit(L.emptyState(), "A"), "B");
  const original = s;
  s = L.toggleDay(s, 1, "2026-10-31");
  s = L.toggleDay(s, 1, "2026-10-29");
  s = L.toggleDay(s, 1, "2026-10-30");
  assert.deepEqual(s.habits[0].done, ["2026-10-29", "2026-10-30", "2026-10-31"], "kept sorted");
  s = L.toggleDay(s, 1, "2026-10-30");
  assert.deepEqual(s.habits[0].done, ["2026-10-29", "2026-10-31"], "toggled off");
  assert.deepEqual(s.habits[1].done, [], "other habits untouched");
  assert.deepEqual(original.habits[0].done, [], "old state untouched");
  assert.deepEqual(L.toggleDay(s, 99, T), s, "unknown id changes nothing");
});

test("the last n days", () => {
  assert.deepEqual(L.lastNDays(T, 3), ["2026-10-30", "2026-10-31", "2026-11-01"]);
  assert.deepEqual(L.lastNDays(T, 1), [T]);
  assert.equal(L.lastNDays(T, 7).length, 7);
  assert.equal(L.lastNDays("2026-03-02", 3)[0], "2026-02-28");
});

test("streaks", () => {
  const h = (done) => ({ id: 1, name: "x", done });
  assert.equal(L.streak(h([]), T), 0);
  assert.equal(L.streak(h(["2026-10-30", "2026-10-31", T]), T), 3, "including today");
  assert.equal(L.streak(h(["2026-10-30", "2026-10-31"]), T), 2, "today not done yet");
  assert.equal(L.streak(h(["2026-10-29", "2026-10-31", T]), T), 2, "a gap ends it");
  assert.equal(L.streak(h(["2026-10-29", "2026-10-30"]), T), 0, "missed yesterday");
  assert.equal(L.streak(h(["2026-02-27", "2026-02-28", "2026-03-01"]), "2026-03-01"), 3, "across a month end");
});

test("completion rate", () => {
  const h = { id: 1, name: "x", done: ["2026-10-20", "2026-10-28", "2026-10-30", T] };
  assert.equal(L.completionRate(h, T, 7), 43);
  assert.equal(L.completionRate(h, T, 1), 100);
  assert.equal(L.completionRate(h, T, 3), 67);
  assert.equal(L.completionRate({ ...h, done: [] }, T, 30), 0);
});

test("loading saved data", () => {
  assert.deepEqual(L.load(null), L.emptyState());
  let s = L.addHabit(L.emptyState(), "Walk");
  s = L.toggleDay(s, 1, T);
  assert.deepEqual(L.load(L.save(s)), s, "round trip");
  const v1 = JSON.stringify({ habits: [{ name: "Walk", days: { "2026-10-31": true, "2026-10-30": true, "2026-10-29": false } }] });
  assert.deepEqual(L.load(v1), { version: 2, nextId: 2, habits: [{ id: 1, name: "Walk", done: ["2026-10-30", "2026-10-31"] }] });
  for (const bad of ["{oops", "[]", '{"habits": 3}', '{"version": 9, "habits": []}',
    '{"version": 2, "nextId": 2, "habits": [{"id": 1, "name": "x", "done": ["yesterday"]}]}']) {
    assert.throws(() => L.load(bad), /Not a habit file/, bad);
  }
});
