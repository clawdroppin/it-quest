// app.js: the front end. One state object; render() draws the page from it; events change the
// state and call render() again. React, Vue and Svelte are this same loop, made faster and nicer.

const state = { links: [], tags: [], q: "", tag: null, sort: "new", message: "", error: false };
const $ = (id) => document.getElementById(id);

async function api(method, url, body) {
  const res = await fetch(url, {
    method,
    headers: body ? { "Content-Type": "application/json" } : {},
    body: body ? JSON.stringify(body) : undefined,
  });
  if (res.status === 204) return null;
  const data = await res.json();
  if (!res.ok) throw new Error(data.error ?? `HTTP ${res.status}`);
  return data;
}

async function load() {
  const params = new URLSearchParams({ sort: state.sort });
  if (state.q) params.set("q", state.q);
  if (state.tag) params.set("tag", state.tag);
  try {
    [state.links, state.tags] = await Promise.all([api("GET", `/api/links?${params}`), api("GET", "/api/tags")]);
  } catch (err) {
    show(err.message, true);
  }
  render();
}

function show(message, error = false) {
  state.message = message;
  state.error = error;
  render();
}

// Build elements with textContent, never innerHTML with user data: titles can't inject HTML.
function el(tag, { dataset = {}, ...props } = {}, ...children) {
  const node = Object.assign(document.createElement(tag), props);
  Object.assign(node.dataset, dataset); // node.dataset itself can't be replaced, only filled
  node.append(...children);
  return node;
}

function render() {
  $("message").textContent = state.message;
  $("message").className = state.error ? "error" : "";
  $("tags").replaceChildren(
    ...state.tags.map((t) =>
      el("button", { className: t.name === state.tag ? "active" : "", dataset: { tag: t.name } }, `#${t.name} ${t.count}`),
    ),
  );
  $("links").replaceChildren(
    ...state.links.map((l) =>
      el(
        "li",
        {},
        el("a", { href: l.url, target: "_blank", rel: "noopener noreferrer", dataset: { visit: l.id } }, l.title),
        el("small", {}, ...l.tags.map((t) => el("span", { className: "tag" }, `#${t}`)), `${l.url} · ${l.visits} visits`),
        el("span", { className: "actions" }, el("button", { dataset: { remove: l.id }, title: "Delete" }, "✕")),
      ),
    ),
  );
  $("empty").hidden = state.links.length > 0 || Boolean(state.q || state.tag);
}

$("add").addEventListener("submit", async (e) => {
  e.preventDefault();
  const form = new FormData(e.target);
  try {
    const link = await api("POST", "/api/links", {
      url: form.get("url"),
      title: form.get("title"),
      tags: String(form.get("tags")).split(",").map((t) => t.trim()).filter(Boolean),
    });
    e.target.reset();
    show(`Saved "${link.title}"`);
    load();
  } catch (err) {
    show(err.message, true);
  }
});

// Wait until the user stops typing for 250 ms before searching ("debouncing").
let timer;
$("search").addEventListener("input", (e) => {
  clearTimeout(timer);
  timer = setTimeout(() => {
    state.q = e.target.value.trim();
    load();
  }, 250);
});

$("sort").addEventListener("change", (e) => {
  state.sort = e.target.value;
  load();
});

// One listener for many buttons ("event delegation"): it works for elements render() creates later.
$("tags").addEventListener("click", (e) => {
  const tag = e.target.closest("button")?.dataset.tag;
  if (!tag) return;
  state.tag = state.tag === tag ? null : tag;
  load();
});

$("links").addEventListener("click", async (e) => {
  const remove = e.target.closest("[data-remove]");
  const visit = e.target.closest("[data-visit]");
  if (remove && confirm("Delete this link?")) {
    await api("DELETE", `/api/links/${remove.dataset.remove}`).catch((err) => show(err.message, true));
    load();
  } else if (visit) {
    api("POST", `/api/links/${visit.dataset.visit}/visit`).then(load, () => {});
  }
});

load();
