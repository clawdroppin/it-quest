// Run: node --test     (needs Node.js 20 or newer)
const test = require("node:test");
const assert = require("node:assert/strict");
const P = require("./plants.js");

const T = "2026-11-01";
const plant = (over) => ({ id: 1, name: "Fern", emoji: "🌿", everyDays: 3, lastWatered: T, waterings: 0, ...over });

test("days until water use calendar days", () => {
  assert.equal(P.daysUntilWater(plant({ lastWatered: "2026-10-29" }), T), 0);
  assert.equal(P.daysUntilWater(plant({ lastWatered: "2026-10-31", everyDays: 7 }), T), 6);
  assert.equal(P.daysUntilWater(plant({ lastWatered: "2026-02-27", everyDays: 3 }), "2026-03-03"), -1);
});

test("status", () => {
  assert.equal(P.status(plant({ lastWatered: "2026-10-25" }), T), "thirsty");
  assert.equal(P.status(plant({ lastWatered: "2026-10-28" }), T), "thirsty", "one day late is already thirsty");
  assert.equal(P.status(plant({ lastWatered: "2026-10-29", everyDays: 3 }), "2026-10-31"), "soon");
  assert.equal(P.status(plant({ lastWatered: "2026-10-29", everyDays: 3 }), "2026-11-01"), "today");
  assert.equal(P.status(plant({ lastWatered: "2026-10-29", everyDays: 3 }), "2026-10-30"), "happy");
});

test("labels", () => {
  assert.deepEqual([-3, -1, 0, 1, 2, 14].map(P.label),
    ["3 days overdue", "1 day overdue", "Water today", "Tomorrow", "In 2 days", "In 14 days"]);
});

test("watering returns a new list and leaves the others alone", () => {
  const a = plant({ id: 1, lastWatered: "2026-10-20", waterings: 5 });
  const b = plant({ id: 2, name: "Cactus" });
  const before = [a, b];
  const after = P.water(before, 1, T);
  assert.notEqual(after, before);
  assert.deepEqual(after[0], { ...a, lastWatered: T, waterings: 6 });
  assert.equal(after[1], b, "untouched plants are the same objects");
  assert.equal(a.lastWatered, "2026-10-20", "the old plant didn't change");
  assert.equal(P.water(before, 99, T).length, 2, "an unknown id changes nothing");
});

test("sorting by need", () => {
  const list = [
    plant({ id: 1, name: "zz plant", lastWatered: T, everyDays: 10 }),
    plant({ id: 2, name: "Basil", lastWatered: "2026-10-28", everyDays: 2 }),
    plant({ id: 3, name: "aloe", lastWatered: T, everyDays: 10 }),
    plant({ id: 4, name: "Mint", lastWatered: "2026-10-31", everyDays: 1 }),
  ];
  assert.deepEqual(P.sortByNeed(list, T).map((p) => p.id), [2, 4, 3, 1]);
  assert.deepEqual(list.map((p) => p.id), [1, 2, 3, 4], "the original order is kept");
});

test("adding plants", () => {
  let list = P.addPlant([], { name: "  Pothos ", emoji: "🍃", everyDays: "5" }, T);
  list = P.addPlant(list, { name: "Ivy", everyDays: 4 }, T);
  assert.deepEqual(list.map((p) => [p.id, p.name, p.emoji, p.everyDays]), [[1, "Pothos", "🍃", 5], [2, "Ivy", "🪴", 4]]);
  assert.throws(() => P.addPlant(list, { name: " ", everyDays: 3 }, T), /Give your plant a name/);
  assert.throws(() => P.addPlant(list, { name: "x".repeat(31), everyDays: 3 }, T), /30 characters/);
  assert.throws(() => P.addPlant(list, { name: "Ok", everyDays: 0 }, T), /1 to 60/);
  assert.throws(() => P.addPlant(list, { name: "Ok", everyDays: 2.5 }, T), /1 to 60/);
});

test("summary", () => {
  assert.match(P.summary([], T), /No plants yet/);
  assert.equal(P.summary([plant({ lastWatered: T })], T), "Everyone is happy 🌞");
  assert.equal(P.summary([plant({ lastWatered: "2026-10-01" }), plant({ lastWatered: "2026-10-29" })], T), "2 plants need water");
});

test("loading saved data", () => {
  assert.equal(P.load("not json", T).length, 4, "broken data gives the samples");
  assert.equal(P.load('[{"name": 5}]', T).length, 4, "wrong shapes give the samples");
  const saved = JSON.stringify([{ id: 7, name: "Old", everyDays: 3, lastWatered: T }]);
  assert.deepEqual(P.load(saved, T), [{ id: 7, name: "Old", emoji: "🪴", everyDays: 3, lastWatered: T, waterings: 0 }]);
});
