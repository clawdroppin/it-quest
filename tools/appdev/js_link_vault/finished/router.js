// router.js: a small web framework in one file: routes, JSON bodies, errors and static files.
// Express and Fastify do the same jobs; writing it once shows you there's no magic.
const fs = require("node:fs");
const path = require("node:path");
const { AppError } = require("./links");

const TYPES = { ".html": "text/html; charset=utf-8", ".js": "text/javascript", ".css": "text/css", ".svg": "image/svg+xml", ".png": "image/png" };

/**
 * Turn a route pattern into a matcher: "/api/links/:id" matches "/api/links/7" and gives { id: "7" }.
 * A ":name" segment matches exactly one non-empty path segment (no "/"). Returns a function
 * path => params object, or null when the path doesn't match. Params are URL-decoded.
 */
function compile(pattern) {
  const names = [];
  const source = pattern
    .split("/")
    .map((part) => {
      if (part.startsWith(":")) {
        names.push(part.slice(1));
        return "([^/]+)";
      }
      return part.replace(/[.*+?^${}()|[\]\\]/g, "\\$&");
    })
    .join("/");
  const regex = new RegExp(`^${source}$`);
  return (p) => {
    const m = regex.exec(p);
    if (!m) return null;
    return Object.fromEntries(names.map((n, i) => [n, decodeURIComponent(m[i + 1])]));
  };
}

/**
 * The file to serve for a URL path, or null. "/" means index.html. The result must stay inside
 * publicDir: "/../server.js" or "/%2e%2e/secret" must return null (a "path traversal" attack).
 */
function resolveStatic(publicDir, urlPath) {
  let decoded;
  try {
    decoded = decodeURIComponent(urlPath);
  } catch {
    return null;
  }
  if (decoded.includes("\0")) return null;
  const root = path.resolve(publicDir);
  const file = path.resolve(root, "." + (decoded === "/" ? "/index.html" : decoded));
  if (file !== root && !file.startsWith(root + path.sep)) return null;
  return file;
}

function send(res, status, body) {
  const data = body === undefined ? "" : JSON.stringify(body);
  res.writeHead(status, { "Content-Type": "application/json", "Content-Length": Buffer.byteLength(data) });
  res.end(data);
}

async function readJson(req, limit = 100_000) {
  let size = 0;
  const chunks = [];
  for await (const chunk of req) {
    size += chunk.length;
    if (size > limit) throw new AppError(413, "body is too large");
    chunks.push(chunk);
  }
  if (size === 0) return {};
  try {
    const body = JSON.parse(Buffer.concat(chunks).toString("utf8"));
    if (typeof body !== "object" || body === null || Array.isArray(body)) throw new Error();
    return body;
  } catch {
    throw new AppError(400, "body must be a JSON object");
  }
}

function createRouter({ publicDir = null, log = () => {} } = {}) {
  const routes = [];

  async function handle(req, res) {
    const started = Date.now();
    const url = new URL(req.url, "http://localhost");
    res.on("finish", () => log(`${req.method} ${url.pathname} ${res.statusCode} ${Date.now() - started}ms`));
    try {
      let pathKnown = false;
      for (const r of routes) {
        const params = r.match(url.pathname);
        if (!params) continue;
        if (r.method !== req.method) {
          pathKnown = true;
          continue;
        }
        const body = req.method === "POST" || req.method === "PATCH" ? await readJson(req) : {};
        const query = Object.fromEntries(url.searchParams);
        const [status, result] = await r.handler({ params, query, body, req });
        send(res, status, result);
        return;
      }
      if (pathKnown) throw new AppError(405, "method not allowed");
      if (req.method === "GET" && publicDir && !url.pathname.startsWith("/api/")) {
        const file = resolveStatic(publicDir, url.pathname);
        if (file && fs.existsSync(file) && fs.statSync(file).isFile()) {
          res.writeHead(200, {
            "Content-Type": TYPES[path.extname(file)] ?? "application/octet-stream",
            "Content-Security-Policy": "default-src 'self'",
            "X-Content-Type-Options": "nosniff",
          });
          fs.createReadStream(file).pipe(res);
          return;
        }
      }
      throw new AppError(404, "not found");
    } catch (err) {
      if (err instanceof AppError) send(res, err.status, { error: err.message });
      else {
        log(`ERROR ${err.stack}`);
        send(res, 500, { error: "internal error" });
      }
    }
  }

  return {
    add(method, pattern, handler) {
      routes.push({ method, match: compile(pattern), handler });
      return this;
    },
    handle,
  };
}

module.exports = { compile, resolveStatic, readJson, send, createRouter };
