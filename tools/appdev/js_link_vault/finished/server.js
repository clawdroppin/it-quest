// server.js: wires the pieces together and starts the web server.
//   node server.js              then open http://localhost:3000
//   PORT=8080 DATA_FILE=/data/links.json node server.js
const http = require("node:http");
const path = require("node:path");
const { createStore } = require("./store");
const { createLinkService } = require("./links");
const { createRouter } = require("./router");

function createApp(service, options = {}) {
  const router = createRouter(options)
    .add("GET", "/api/links", ({ query }) => [200, service.list(query)])
    .add("POST", "/api/links", ({ body }) => [201, service.add(body)])
    .add("GET", "/api/links/:id", ({ params }) => [200, service.get(params.id)])
    .add("PATCH", "/api/links/:id", ({ params, body }) => [200, service.update(params.id, body)])
    .add("DELETE", "/api/links/:id", ({ params }) => {
      service.remove(params.id);
      return [204];
    })
    .add("POST", "/api/links/:id/visit", ({ params }) => [200, service.visit(params.id)])
    .add("GET", "/api/tags", () => [200, service.tags()])
    .add("GET", "/api/health", () => [200, { status: "ok" }]);
  return http.createServer(router.handle);
}

module.exports = { createApp };

if (require.main === module) {
  const port = Number(process.env.PORT ?? 3000);
  const store = createStore(process.env.DATA_FILE ?? path.join(__dirname, "links.json"));
  const server = createApp(createLinkService(store), {
    publicDir: path.join(__dirname, "public"),
    log: (line) => console.log(new Date().toISOString(), line),
  });
  server.listen(port, "127.0.0.1", () => console.log(`Link Vault on http://localhost:${port}`));
  // Finish requests in progress before exiting (Ctrl+C, or "docker stop").
  for (const signal of ["SIGINT", "SIGTERM"]) {
    process.on(signal, () => server.close(() => process.exit(0)));
  }
}
