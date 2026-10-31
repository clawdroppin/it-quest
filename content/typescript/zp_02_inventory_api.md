@@lesson xp-inventory_api Typed Inventory REST API
@@text
Build a REST API for an IT asset inventory using only Node's built-in `node:http`: `GET /assets`, `GET /assets/:id`, `POST /assets`, `PATCH /assets/:id`, `DELETE /assets/:id`, with validation of every request body, proper status codes, and data saved to a JSON file. TypeScript types make sure the handlers and the data agree.

**You'll practise:** node:http · routing by method and path · type guards for request bodies · status codes · node:test

## Milestones

1. **Types**: `Asset` and `NewAsset`, and a type guard `isNewAsset(x: unknown)`.
2. **Router**: match method and path, including `/assets/:id`.
3. **Handlers**: the five endpoints, returning 200, 201, 204, 400 or 404 correctly.
4. **Persistence**: load assets.json at start, write it after each change.
5. **Tests**: start the server on a random port in a test and call it with `fetch`.

## You're done when

- `curl` and the tests can create, read, update and delete assets, and invalid bodies get a 400 with a reason.

## Getting help

Open the project, then the 🤖 **AI Helper**: it sees the lesson and your files. Good questions to start with:

- *"How do I read a request body with node:http?"*
- *"Is my type guard really checking everything the type promises?"*

Ask for hints, reviews and explanations rather than finished code: you learn by writing it.
@@project inventory_api Typed Inventory REST API
Create the folder, open it in VS Code, and work through `README.md` one milestone at a time.
@@file README.md
# Typed Inventory REST API

Build a REST API for an IT asset inventory using only Node's built-in `node:http`: `GET /assets`, `GET /assets/:id`, `POST /assets`, `PATCH /assets/:id`, `DELETE /assets/:id`, with validation of every request body, proper status codes, and data saved to a JSON file. TypeScript types make sure the handlers and the data agree.

## Milestones

- [ ] 1. Types: `Asset` and `NewAsset`, and a type guard `isNewAsset(x: unknown)`.
- [ ] 2. Router: match method and path, including `/assets/:id`.
- [ ] 3. Handlers: the five endpoints, returning 200, 201, 204, 400 or 404 correctly.
- [ ] 4. Persistence: load assets.json at start, write it after each change.
- [ ] 5. Tests: start the server on a random port in a test and call it with `fetch`.

## Done when

- [ ] `curl` and the tests can create, read, update and delete assets, and invalid bodies get a 400 with a reason.
@@file server.ts
// An IT asset inventory API using only node:http. Run: node server.ts
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
