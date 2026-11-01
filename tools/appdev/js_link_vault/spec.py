OUT = "content/javascript/k3_01_link_vault.md"
BASE = "tools/appdev/js_link_vault"
FILES = ["links.js", "store.js", "router.js", "server.js", "links.test.js", "api.test.js", "package.json",
         "public/index.html", "public/style.css", "public/app.js"]

PARTS = [
    ("text", r'''
@@lesson k3-link-vault Link Vault (a full-stack application)
@@text
A complete web application with **no dependencies at all**: a Node.js back end with its own small web framework, a JSON file database written safely, and a front end in plain JavaScript. You save links, tag them, search them, and see which ones you actually use.

```text
npm start          (or: node server.js)   then open http://localhost:3000
npm test           (or: node --test)
```

## The pieces

| File | Job |
|---|---|
| `links.js` | the business rules: clean URLs, refuse duplicates, search, sort, count tags |
| `store.js` | keeps the data in `links.json`, written atomically (temp file + rename) |
| `router.js` | a mini framework: route patterns, JSON bodies, errors, static files |
| `server.js` | wires it together: the routes, and starting/stopping the server |
| `public/` | the front end: `index.html`, `style.css`, `app.js` |

The API:

| Request | Does |
|---|---|
| `GET /api/links?q=docs&tag=k8s&sort=popular` | search |
| `POST /api/links` `{"url", "title", "tags"}` | save (201, or 409 if it's already saved) |
| `PATCH /api/links/:id` | rename or retag |
| `DELETE /api/links/:id` | delete (204) |
| `POST /api/links/:id/visit` | count a visit |
| `GET /api/tags` | tags with counts |

## Your tasks

Four functions in the starter throw `TODO: not done yet`. Each has a comment above it saying exactly what it must do.

1. **`normalizeUrl`** (`links.js`): `WWW.Example.com/docs/?utm_source=mail#top` and `example.com/docs` are the same page. Use the built-in `URL` class. Only allow `http` and `https`: a saved `javascript:` link would run code when clicked.
2. **`list`** (`links.js`): search words, a tag filter and three sort orders.
3. **`compile`** (`router.js`): turn `/api/links/:id` into a `RegExp` and return the params. Escape the other characters, because `.` means "any character" in a regex.
4. **`resolveStatic`** (`router.js`): map a URL path to a file in `public/` and **refuse anything outside it**. `/../server.js` and `/%2e%2e/server.js` are real attacks that leak source code and passwords from badly written servers.

Do them in this order and run `node --test` after each. The router tests need `compile`, so the API tests only pass once it's done.
'''),
    ("project", "link_vault", "Link Vault",
     "Write the four functions that throw `TODO`, until `node --test` passes. Then `npm start` and open http://localhost:3000.",
     BASE + "/starter", FILES),
    ("text", r'''
@@text
## Stretch goals

- **Fetch titles automatically**: when no title is given, `fetch()` the page on the server and read its `<title>`. Use a timeout (`AbortSignal.timeout(5000)`), a size limit, and refuse private addresses (`10.x`, `192.168.x`, `localhost`) so your server can't be used to probe the internal network (that attack is called **SSRF**).
- **Import and export**: read a browser bookmarks HTML export; download everything as JSON.
- **Accounts**: users, `crypto.scrypt` password hashes, a session cookie (`HttpOnly; SameSite=Lax`), and every link belongs to a user.
- **SQLite instead of JSON**: Node 22.5+ has `node:sqlite` built in. Only `store.js` and `links.js` should need to change.
- **Rewrite the front end in React or Svelte**, keeping the same API. Then rewrite the back end with Express or Fastify, keeping the same tests.
- **Ship it**: a `Dockerfile` (`FROM node:24-slim`, `USER node`, data in a volume), behind Caddy for HTTPS.
'''),
    ("project", "link_vault_finished", "Link Vault: the finished version",
     "A complete solution to compare with yours.",
     BASE + "/finished", FILES),
    ("text", r'''
@@text
🎉 You've built every layer of a web application yourself: HTTP routing, input validation, safe file storage, security headers and a reactive front end. Frameworks will now feel like shortcuts you understand rather than magic you depend on.
'''),
]
