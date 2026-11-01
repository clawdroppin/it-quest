// app.js: the page. It owns the state, saves it, and draws it. All the rules are in logic.js.
(function () {
  "use strict";
  const L = window.HabitLogic;
  const KEY = "habits";
  const DAYS_SHOWN = 7;

  const $ = (id) => document.getElementById(id);
  const list = $("habits");
  const template = $("habit-template");

  let state = readSaved();

  function readSaved() {
    try {
      return L.load(localStorage.getItem(KEY));
    } catch (e) {
      // Storage blocked (private mode) or damaged data: start empty rather than crash.
      console.warn("Could not load saved habits:", e);
      return L.emptyState();
    }
  }

  // The only way the state changes: compute a new one, save it, draw it.
  function update(next, message) {
    state = next;
    try {
      localStorage.setItem(KEY, L.save(state));
    } catch {
      announce("Couldn't save: your browser is blocking storage.");
    }
    render();
    if (message) announce(message);
  }

  function announce(text) {
    $("status").textContent = text;   // read out by screen readers (aria-live)
  }

  const weekday = new Intl.DateTimeFormat(undefined, { weekday: "short" });
  const longDate = new Intl.DateTimeFormat(undefined, { weekday: "long", month: "long", day: "numeric" });

  function render() {
    const today = L.dayKey(new Date());
    const days = L.lastNDays(today, DAYS_SHOWN);
    $("today").textContent = longDate.format(new Date());

    $("days-header").replaceChildren(
      ...days.map((d) => {
        const span = document.createElement("span");
        span.textContent = weekday.format(L.parseDay(d));
        return span;
      }),
    );

    // Rebuilding the list is simple and fast for a few dozen items. textContent (never
    // innerHTML) puts the user's text in the page, so a habit called "<img onerror=...>" is just text.
    list.replaceChildren(
      ...state.habits.map((habit) => {
        const item = template.content.firstElementChild.cloneNode(true);
        item.dataset.id = habit.id;
        item.querySelector(".name").textContent = habit.name;
        const s = L.streak(habit, today);
        item.querySelector(".stats").textContent =
          `${s} day${s === 1 ? "" : "s"} in a row · ${L.completionRate(habit, today, 30)}% of the last 30 days`;
        item.querySelector(".remove").setAttribute("aria-label", `Remove ${habit.name}`);
        const done = new Set(habit.done);
        item.querySelector(".days").replaceChildren(
          ...days.map((d) => {
            const b = document.createElement("button");
            b.type = "button";
            b.className = "day" + (d === today ? " is-today" : "");
            b.dataset.action = "toggle";
            b.dataset.day = d;
            b.setAttribute("aria-pressed", String(done.has(d)));
            b.setAttribute("aria-label", `${habit.name}, ${longDate.format(L.parseDay(d))}`);
            b.textContent = done.has(d) ? "✓" : "";
            return b;
          }),
        );
        return item;
      }),
    );
    $("empty").hidden = state.habits.length > 0;
  }

  // One listener for the whole list (event delegation): it works for items added later too.
  list.addEventListener("click", (event) => {
    const button = event.target.closest("button[data-action]");
    if (!button) return;
    const id = Number(button.closest(".habit").dataset.id);
    const habit = state.habits.find((h) => h.id === id);
    if (button.dataset.action === "toggle") {
      const day = button.dataset.day;
      update(L.toggleDay(state, id, day));
      // render() replaced the button, so put the keyboard focus back on the new one.
      list.querySelector(`.habit[data-id="${id}"] [data-day="${day}"]`)?.focus();
    } else if (button.dataset.action === "remove" && confirm(`Remove "${habit.name}" and its history?`)) {
      update(L.removeHabit(state, id), `Removed ${habit.name}`);
      $("habit-name").focus();
    }
  });

  $("add-form").addEventListener("submit", (event) => {
    event.preventDefault();
    const input = $("habit-name");
    const error = $("form-error");
    try {
      update(L.addHabit(state, input.value), `Added ${input.value.trim()}`);
      input.value = "";
      error.hidden = true;
      input.removeAttribute("aria-invalid");
    } catch (e) {
      error.textContent = e.message;
      error.hidden = false;
      input.setAttribute("aria-invalid", "true");
    }
    input.focus();
  });

  $("export").addEventListener("click", () => {
    const blob = new Blob([L.save(state)], { type: "application/json" });
    const link = document.createElement("a");
    link.href = URL.createObjectURL(blob);
    link.download = `habits-${L.dayKey(new Date())}.json`;
    link.click();
    setTimeout(() => URL.revokeObjectURL(link.href), 1000);
  });

  $("import").addEventListener("change", async (event) => {
    const file = event.target.files[0];
    event.target.value = "";             // so choosing the same file again still fires "change"
    if (!file) return;
    try {
      const imported = L.load(await file.text());
      if (confirm(`Replace your ${state.habits.length} habit(s) with the ${imported.habits.length} in ${file.name}?`)) {
        update(imported, `Imported ${imported.habits.length} habit(s)`);
      }
    } catch (e) {
      alert(`${file.name}: ${e.message}`);
    }
  });

  // Another tab changed the data: show it here too.
  window.addEventListener("storage", (event) => {
    if (event.key === KEY) {
      state = readSaved();
      render();
    }
  });

  // A new day may have started while the page was open in the background.
  document.addEventListener("visibilitychange", () => {
    if (!document.hidden) render();
  });

  // Offline support. Service workers only run on http(s), not on pages opened as files.
  if ("serviceWorker" in navigator && location.protocol.startsWith("http")) {
    navigator.serviceWorker.register("sw.js").catch((e) => console.warn("No offline support:", e));
  }

  render();
})();
