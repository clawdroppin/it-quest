// app.js: the window. It keeps the state, saves it, draws it, and plays the animations.
// All the rules live in plants.js (window.Plants), so this file stays about the GUI.
(function () {
  "use strict";
  const P = window.Plants;
  const KEY = "plant-buddy";
  const THEME_KEY = "plant-buddy-theme";
  const reduceMotion = matchMedia("(prefers-reduced-motion: reduce)");

  const $ = (sel) => document.querySelector(sel);
  const list = $("#plants");
  const template = $("#card");
  const dialog = $("#dialog");
  const form = $("#form");

  // ---------- State ----------
  let todayKey = P.today();
  let plants = P.load(readStorage(KEY), todayKey);
  let firstRender = true;

  function readStorage(key) {
    try { return localStorage.getItem(key); } catch { return null; }
  }
  function writeStorage(key, value) {
    try { localStorage.setItem(key, value); } catch { /* private mode: keep working without saving */ }
  }

  function setPlants(next) {
    plants = next;
    writeStorage(KEY, JSON.stringify(plants));
    render();
  }

  // ---------- Drawing ----------
  function render() {
    const sorted = P.sortByNeed(plants, todayKey);
    $("#summary").textContent = P.summary(plants, todayKey);
    $("#empty").hidden = plants.length > 0;

    list.replaceChildren(...sorted.map((plant, i) => {
      const card = template.content.firstElementChild.cloneNode(true);
      const days = P.daysUntilWater(plant, todayKey);
      card.dataset.id = plant.id;
      card.dataset.status = P.status(plant, todayKey);
      // Only the first render staggers in; later renders appear in place.
      if (firstRender) {
        card.classList.add("enter");
        card.style.setProperty("--i", i);
      }
      card.querySelector(".emoji").textContent = plant.emoji;
      card.querySelector(".name").textContent = plant.name;
      card.querySelector(".when").textContent = P.label(days);
      // The meter shows how much water is left: full right after watering, empty when due.
      const fill = Math.max(0, Math.min(1, days / plant.everyDays));
      card.querySelector(".meter span").style.setProperty("--fill", fill);
      card.querySelector(".water").setAttribute("aria-label", `Water ${plant.name}`);
      const remove = card.querySelector(".remove");
      remove.setAttribute("aria-label", `Remove ${plant.name}`);
      remove.title = `Remove ${plant.name}`;
      return card;
    }));
    firstRender = false;
  }

  // ---------- Feedback ----------
  function toast(text) {
    const t = document.createElement("div");
    t.className = "toast";
    t.textContent = text;
    $("#toasts").append(t);
    setTimeout(() => {
      t.classList.add("leaving");
      setTimeout(() => t.remove(), 250);
    }, 2400);
  }

  // Little droplets fall from the button that was pressed.
  function droplets(fromEl) {
    if (reduceMotion.matches) return;
    const box = fromEl.getBoundingClientRect();
    for (let i = 0; i < 6; i++) {
      const d = document.createElement("span");
      d.className = "drop";
      d.style.left = `${box.left + box.width / 2 - 5 + (i - 2.5) * 6}px`;
      d.style.top = `${box.top + box.height / 2}px`;
      d.style.setProperty("--dx", `${(i - 2.5) * 10}px`);
      d.style.animationDelay = `${i * 40}ms`;
      d.addEventListener("animationend", () => d.remove());
      document.body.append(d);
    }
  }

  // Leaves burst out of the header when nobody is thirsty any more.
  function celebrate() {
    if (reduceMotion.matches) return;
    const box = $(".logo").getBoundingClientRect();
    const leaves = ["🍃", "🌿", "✨", "🌸"];
    for (let i = 0; i < 24; i++) {
      const l = document.createElement("span");
      l.className = "leaf";
      l.textContent = leaves[i % leaves.length];
      l.style.left = `${box.left + box.width / 2}px`;
      l.style.top = `${box.top + box.height / 2}px`;
      const angle = (i / 24) * Math.PI * 2;
      const dist = 80 + Math.random() * 120;
      l.style.setProperty("--x", `${Math.cos(angle) * dist}px`);
      l.style.setProperty("--y", `${Math.sin(angle) * dist + 60}px`);
      l.style.setProperty("--r", `${Math.random() * 540 - 270}deg`);
      l.addEventListener("animationend", () => l.remove());
      document.body.append(l);
    }
  }

  // ---------- Actions ----------
  function waterPlant(id, button) {
    const plant = plants.find((p) => p.id === id);
    const neededWater = plants.some((p) => P.daysUntilWater(p, todayKey) <= 0);
    droplets(button);
    setPlants(P.water(plants, id, todayKey));
    const card = list.querySelector(`[data-id="${id}"]`);
    card.classList.add("watered");
    card.addEventListener("animationend", () => card.classList.remove("watered"), { once: true });
    card.querySelector(".water").focus();            // the card moved: keep the keyboard on it
    toast(`💧 ${plant.name} says thank you!`);
    if (neededWater && plants.every((p) => P.daysUntilWater(p, todayKey) > 0)) {
      celebrate();
      toast("🌞 Every plant is happy!");
    }
  }

  function removePlant(id) {
    const plant = plants.find((p) => p.id === id);
    const card = list.querySelector(`[data-id="${id}"]`);
    const finish = () => {
      setPlants(P.removePlant(plants, id));
      toast(`👋 Said goodbye to ${plant.name}`);
      $("#add").focus();
    };
    if (reduceMotion.matches) return finish();
    card.classList.add("removing");
    card.addEventListener("animationend", finish, { once: true });
  }

  // Event delegation: one listener for every card, even ones drawn later.
  list.addEventListener("click", (e) => {
    const card = e.target.closest(".card");
    if (!card) return;
    const id = Number(card.dataset.id);
    if (e.target.closest(".water")) waterPlant(id, e.target.closest(".water"));
    else if (e.target.closest(".remove")) removePlant(id);
  });

  // ---------- The add dialog ----------
  const everyInput = form.elements.everyDays;
  const everyOut = $("#every-out");
  const showEvery = () => {
    const n = Number(everyInput.value);
    everyOut.textContent = n === 1 ? "day" : `${n} days`;
  };
  everyInput.addEventListener("input", showEvery);

  function openDialog() {
    form.reset();
    showEvery();
    $("#error").textContent = "";
    dialog.showModal();                               // Esc closes it, focus is trapped inside
    form.elements.name.focus();
  }
  $("#add").addEventListener("click", openDialog);
  $("#cancel").addEventListener("click", () => dialog.close());
  dialog.addEventListener("close", () => $("#add").focus());

  form.addEventListener("submit", (e) => {
    e.preventDefault();
    const data = Object.fromEntries(new FormData(form));
    try {
      const next = P.addPlant(plants, data, todayKey);
      dialog.close();
      setPlants(next);
      toast(`🌱 Welcome home, ${next[next.length - 1].name}!`);
    } catch (err) {
      const error = $("#error");
      error.textContent = "";                          // restart the shake animation
      requestAnimationFrame(() => { error.textContent = err.message; });
      form.elements.name.focus();
    }
  });

  // ---------- Theme ----------
  const themeBtn = $("#theme");
  function applyTheme(theme) {
    document.documentElement.dataset.theme = theme;
    const next = theme === "dark" ? "light" : "dark";
    themeBtn.setAttribute("aria-label", `Switch to ${next} theme`);
    themeBtn.querySelector("use").setAttribute("href", theme === "dark" ? "#i-sun" : "#i-moon");
  }
  function toggleTheme() {
    const theme = document.documentElement.dataset.theme === "dark" ? "light" : "dark";
    writeStorage(THEME_KEY, theme);
    applyTheme(theme);
    if (!reduceMotion.matches) {
      themeBtn.animate([{ transform: "rotate(0) scale(1)" }, { transform: "rotate(200deg) scale(.7)" }, { transform: "rotate(360deg) scale(1)" }],
        { duration: 450, easing: "cubic-bezier(.34, 1.56, .64, 1)" });
    }
  }
  themeBtn.addEventListener("click", toggleTheme);
  applyTheme(readStorage(THEME_KEY) || (matchMedia("(prefers-color-scheme: dark)").matches ? "dark" : "light"));

  // ---------- Keyboard shortcuts (not while typing) ----------
  document.addEventListener("keydown", (e) => {
    if (e.ctrlKey || e.metaKey || e.altKey || dialog.open) return;
    if (e.target.closest("input, textarea, select")) return;
    if (e.key === "n" || e.key === "N") { e.preventDefault(); openDialog(); }
    if (e.key === "t" || e.key === "T") toggleTheme();
  });

  // A new day may start while the app is open: redraw when it's shown again.
  document.addEventListener("visibilitychange", () => {
    if (document.visibilityState === "visible" && P.today() !== todayKey) {
      todayKey = P.today();
      render();
    }
  });

  render();
})();
