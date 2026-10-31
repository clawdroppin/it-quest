from gen_projects import P

PROJECTS = [
    P("javascript", "Intermediate", "kanban", "Kanban Board in the Browser", "A drag-and-drop task board saved in localStorage.",
      """A Trello-style board for your study tasks: columns To do, Doing and Done, cards you can add, edit, drag between columns and delete, saved in the browser so it survives a reload. Plain HTML, CSS and JavaScript, no framework.""",
      ["DOM creation", "events and event delegation", "drag and drop API", "localStorage and JSON", "state → render"],
      ["State: one array of cards `{id, title, column}` and a `render()` that draws everything from it.",
       "Add and delete: a form per column, a ✕ button per card.",
       "Drag and drop: `dragstart`, `dragover`, `drop` move a card to another column.",
       "Save: write the state to localStorage after every change; load it at start.",
       "Polish: edit a card's title by double-clicking; count cards per column."],
      ["Cards can be added, moved and deleted, and everything is still there after a reload."],
      ["Why should I re-render from state instead of moving DOM elements around?",
       "My drop event never fires. What am I missing?"],
      {"index.html": '''<!doctype html>
<html lang="en">
<head>
  <meta charset="utf-8">
  <title>Kanban</title>
  <link rel="stylesheet" href="style.css">
</head>
<body>
  <h1>Study board</h1>
  <main id="board"></main>
  <script src="app.js"></script>
</body>
</html>
''', "style.css": '''body { font-family: system-ui, sans-serif; margin: 2rem; background: #f4f5f7; }
#board { display: grid; grid-template-columns: repeat(3, 1fr); gap: 1rem; }
.column { background: #ebecf0; border-radius: 8px; padding: .75rem; min-height: 200px; }
.card { background: white; border-radius: 6px; padding: .5rem; margin: .5rem 0; box-shadow: 0 1px 2px #0002; cursor: grab; }
''', "app.js": '''// Kanban board: all state in one array, drawn by render().
const COLUMNS = ["To do", "Doing", "Done"];
let cards = JSON.parse(localStorage.getItem("cards") ?? "[]");

function save() {
  localStorage.setItem("cards", JSON.stringify(cards));
}

function render() {
  const board = document.getElementById("board");
  board.innerHTML = "";
  for (const name of COLUMNS) {
    const col = document.createElement("section");
    col.className = "column";
    col.innerHTML = `<h2>${name}</h2>`;
    // TODO: draw this column's cards, an add form, and drag-and-drop handlers
    board.append(col);
  }
}

render();
'''}),
    P("typescript", "Advanced", "inventory_api", "Typed Inventory REST API", "A small JSON API for IT assets with node:http and TypeScript.",
      """Build a REST API for an IT asset inventory using only Node's built-in `node:http`: `GET /assets`, `GET /assets/:id`, `POST /assets`, `PATCH /assets/:id`, `DELETE /assets/:id`, with validation of every request body, proper status codes, and data saved to a JSON file. TypeScript types make sure the handlers and the data agree.""",
      ["node:http", "routing by method and path", "type guards for request bodies", "status codes", "node:test"],
      ["Types: `Asset` and `NewAsset`, and a type guard `isNewAsset(x: unknown)`.",
       "Router: match method and path, including `/assets/:id`.",
       "Handlers: the five endpoints, returning 200, 201, 204, 400 or 404 correctly.",
       "Persistence: load assets.json at start, write it after each change.",
       "Tests: start the server on a random port in a test and call it with `fetch`."],
      ["`curl` and the tests can create, read, update and delete assets, and invalid bodies get a 400 with a reason."],
      ["How do I read a request body with node:http?",
       "Is my type guard really checking everything the type promises?"],
      {"server.ts": '''// An IT asset inventory API using only node:http. Run: node server.ts
import { createServer, IncomingMessage, ServerResponse } from "node:http";

export interface Asset {
  id: number;
  hostname: string;
  owner: string;
  kind: "laptop" | "desktop" | "server" | "phone";
}
export type NewAsset = Omit<Asset, "id">;

export function isNewAsset(x: unknown): x is NewAsset {
  // TODO: check every field and its type
  return false;
}

const assets: Asset[] = [];

function send(res: ServerResponse, status: number, body?: unknown): void {
  res.writeHead(status, { "content-type": "application/json" });
  res.end(body === undefined ? "" : JSON.stringify(body));
}

export function handle(req: IncomingMessage, res: ServerResponse): void {
  // TODO: route GET/POST /assets and GET/PATCH/DELETE /assets/:id
  if (req.method === "GET" && req.url === "/assets") return send(res, 200, assets);
  send(res, 404, { error: "not found" });
}

createServer(handle).listen(3000, () => console.log("http://localhost:3000/assets"));
'''}),
    P("html", "Intermediate", "status_page", "Service Status Page", "A public status page for your company's IT services.",
      """When email is down, people want one page that says so. Build a clean, accessible status page: overall status at the top, a list of services with coloured badges, current incidents with updates, and a 90-day uptime bar per service, all in HTML and CSS (with a small JSON file you could later fill from monitoring).""",
      ["semantic HTML", "CSS grid", "status colours with text, not colour alone", "responsive layout"],
      ["Structure: header, overall status banner, services list, incidents, footer.",
       "Badges: Operational, Degraded, Outage, each with an icon and text.",
       "Uptime bars: 90 small boxes per service (CSS grid), colour by status.",
       "Incident timeline: Investigating → Identified → Monitoring → Resolved with times.",
       "Responsive and accessible: works on a phone; passes a keyboard and contrast check."],
      ["The page looks good on desktop and phone and makes sense with a screen reader."],
      ["How can I show status without relying only on colour?",
       "Review my HTML structure for accessibility."],
      {"index.html": '''<!doctype html>
<html lang="en">
<head>
  <meta charset="utf-8">
  <meta name="viewport" content="width=device-width, initial-scale=1">
  <title>Acme IT Status</title>
  <link rel="stylesheet" href="style.css">
</head>
<body>
  <header>
    <h1>Acme IT Status</h1>
  </header>
  <main>
    <section class="banner ok" aria-live="polite">
      <p>All systems operational</p>
    </section>
    <section aria-labelledby="services">
      <h2 id="services">Services</h2>
      <ul class="services">
        <li><span class="name">Email</span> <span class="badge ok">Operational</span></li>
        <!-- TODO: more services, each with a 90-day uptime bar -->
      </ul>
    </section>
    <!-- TODO: incidents section -->
  </main>
</body>
</html>
''', "style.css": '''body { font-family: system-ui, sans-serif; max-width: 860px; margin: 0 auto; padding: 1rem; }
.banner { padding: 1rem; border-radius: 8px; color: white; }
.banner.ok, .badge.ok { background: #1a7f37; }
.badge { color: white; padding: .1rem .5rem; border-radius: 999px; font-size: .85rem; }
/* TODO: degraded and outage colours, uptime bars, incident timeline */
'''}),
]
