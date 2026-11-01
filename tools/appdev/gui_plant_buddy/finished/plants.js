// plants.js: everything Plant Buddy knows about plants, with no DOM and no storage.
// The page uses it as window.Plants; Node's test runner loads it with require().
// Functions never change the list they're given; they return a new one.
(function (root) {
  "use strict";

  const MAX_NAME = 30;

  // "YYYY-MM-DD" -> a whole day number. Date.UTC ignores time zones and daylight saving,
  // so subtracting two day numbers always gives a whole number of calendar days.
  function dayNumber(key) {
    const [y, m, d] = key.split("-").map(Number);
    return Date.UTC(y, m - 1, d) / 86400000;
  }

  // The local calendar day of a Date, as "YYYY-MM-DD".
  function today(date = new Date()) {
    const m = String(date.getMonth() + 1).padStart(2, "0");
    const d = String(date.getDate()).padStart(2, "0");
    return `${date.getFullYear()}-${m}-${d}`;
  }

  // Days until the plant needs water: 0 = today, negative = overdue.
  function daysUntilWater(plant, todayKey) {
    return dayNumber(plant.lastWatered) + plant.everyDays - dayNumber(todayKey);
  }

  // The plant's status, which picks its colour, icon and animation:
  //   overdue (days < 0) -> "thirsty", 0 -> "today", 1 -> "soon", anything later -> "happy".
  function status(plant, todayKey) {
    const days = daysUntilWater(plant, todayKey);
    if (days < 0) return "thirsty";
    if (days === 0) return "today";
    if (days === 1) return "soon";
    return "happy";
  }

  // The text under the plant's name for a number of days (from daysUntilWater):
  //   -1 -> "1 day overdue", -3 -> "3 days overdue", 0 -> "Water today",
  //   1 -> "Tomorrow", 4 -> "In 4 days".
  function label(days) {
    if (days < 0) return `${-days} day${days === -1 ? "" : "s"} overdue`;
    if (days === 0) return "Water today";
    if (days === 1) return "Tomorrow";
    return `In ${days} days`;
  }

  // A new list where the plant with this id has lastWatered = todayKey and one more
  // in its `waterings` count. Every other plant is the same object as before.
  function water(plants, id, todayKey) {
    return plants.map((p) => (p.id === id ? { ...p, lastWatered: todayKey, waterings: p.waterings + 1 } : p));
  }

  // A new, sorted list: the plant that needs water soonest first; ties by name, A to Z,
  // ignoring upper/lower case (use localeCompare with { sensitivity: "base" }).
  function sortByNeed(plants, todayKey) {
    return [...plants].sort((a, b) =>
      daysUntilWater(a, todayKey) - daysUntilWater(b, todayKey) ||
      a.name.localeCompare(b.name, undefined, { sensitivity: "base" }));
  }

  // Adds a plant, or throws an Error whose message the form shows.
  function addPlant(plants, { name, emoji, everyDays }, todayKey) {
    name = String(name ?? "").trim();
    everyDays = Number(everyDays);
    if (!name) throw new Error("Give your plant a name");
    if (name.length > MAX_NAME) throw new Error(`Names can be ${MAX_NAME} characters at most`);
    if (!Number.isInteger(everyDays) || everyDays < 1 || everyDays > 60) {
      throw new Error("Water every 1 to 60 days");
    }
    const id = plants.reduce((max, p) => Math.max(max, p.id), 0) + 1;
    return [...plants, { id, name, emoji: emoji || "🪴", everyDays, lastWatered: todayKey, waterings: 0 }];
  }

  function removePlant(plants, id) {
    return plants.filter((p) => p.id !== id);
  }

  // A friendly summary for the header.
  function summary(plants, todayKey) {
    if (plants.length === 0) return "No plants yet. Add your first one!";
    const need = plants.filter((p) => daysUntilWater(p, todayKey) <= 0).length;
    if (need === 0) return "Everyone is happy 🌞";
    return `${need} plant${need === 1 ? " needs" : "s need"} water`;
  }

  function samplePlants(todayKey) {
    const back = (n) => {
      const [y, m, d] = todayKey.split("-").map(Number);
      return today(new Date(y, m - 1, d - n));
    };
    return [
      { id: 1, name: "Monty the Monstera", emoji: "🌿", everyDays: 7, lastWatered: back(8), waterings: 12 },
      { id: 2, name: "Spike", emoji: "🌵", everyDays: 21, lastWatered: back(3), waterings: 4 },
      { id: 3, name: "Basil", emoji: "🌱", everyDays: 2, lastWatered: back(2), waterings: 30 },
      { id: 4, name: "Sunny", emoji: "🌻", everyDays: 3, lastWatered: back(2), waterings: 9 },
    ];
  }

  // Reads saved JSON; anything broken or missing gives the sample plants instead.
  function load(json, todayKey) {
    try {
      const data = JSON.parse(json);
      if (Array.isArray(data) && data.every((p) => p && typeof p.name === "string" && typeof p.lastWatered === "string")) {
        return data.map((p) => ({ waterings: 0, emoji: "🪴", ...p }));
      }
    } catch { /* fall through */ }
    return samplePlants(todayKey);
  }

  const api = { MAX_NAME, dayNumber, today, daysUntilWater, status, label, water, sortByNeed,
    addPlant, removePlant, summary, samplePlants, load };
  if (typeof module === "object" && module.exports) module.exports = api;
  else root.Plants = api;
})(globalThis);
