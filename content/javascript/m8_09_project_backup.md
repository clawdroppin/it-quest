@@lesson m8-project 🏆 Module Project: An Incremental Backup Tool
@@concepts files, automation, processes
@@text
This project builds the core of an **incremental backup and sync tool**, the kind of thing behind `rsync`, robocopy `/MIR`, restic or a nightly "copy the shares to the NAS" job. It uses the module's Node.js skills: the file system, streams and hashing, careful paths, concurrency limits, structured results and tests with real temporary folders.

## How incremental backup works

```text
 source/                        manifest (JSON)                         backup/
 ├─ docs/a.txt   ─┐             { "docs/a.txt": { size, mtime, sha256 },  ├─ docs/a.txt
 ├─ docs/b.txt    ├─ scan ──▶     "docs/b.txt": {...},             ─diff─▶├─ ...
 └─ img/logo.png ─┘               "img/logo.png": {...} }                 └─ .manifest.json
                                         │
                       plan: copy changed/new files, delete removed ones (mirror mode)
```

1. **Scan** the source: walk the directory tree and record each file's relative path, size, modification time and a **SHA-256 hash** (computed with a stream, so big files don't fill memory).
2. **Diff** against the manifest saved with the previous backup: new files, changed files (different hash) and deleted files.
3. **Plan**, then **apply**: copy what changed, optionally delete what disappeared, and save the new manifest. Planning first allows a `--dry-run` that shows what would happen.

## Design points

- Use **relative paths with forward slashes** in the manifest so it's portable between Windows and Linux.
- Hashing every file is slow for big trees; real tools first compare **size + mtime** and only hash when those changed (a "quick check").
- Copy to a temporary name and **rename** into place, so an interrupted backup never leaves half-written files with the right name.
- Limit concurrency (the task queue from the previous module) so you don't open thousands of files at once.
- **Never follow paths outside the root**: symlinks and `..` can point anywhere; skip symlinks or resolve and check them.
- Report results as data (`{ copied, deleted, skipped, errors }`), and log one line per action.

```javascript
const fs = require("node:fs/promises");
const os = require("node:os");
const path = require("node:path");

async function demo() {
  const dir = await fs.mkdtemp(path.join(os.tmpdir(), "demo-"));
  await fs.mkdir(path.join(dir, "docs"), { recursive: true });
  await fs.writeFile(path.join(dir, "docs", "a.txt"), "hello");
  const entries = await fs.readdir(dir, { recursive: true, withFileTypes: true });
  console.log(entries.filter((e) => e.isFile()).map((e) => path.relative(dir, path.join(e.parentPath, e.name)).replaceAll("\\", "/")));
  await fs.rm(dir, { recursive: true, force: true });
}
demo();
```

`fs.readdir(dir, { recursive: true, withFileTypes: true })` (Node 20+) walks the whole tree; each entry has `parentPath` and `name`.

## Scheduling it

- Windows: Task Scheduler running `node backup.js \\fileserver\share D:\backup` nightly.
- Linux: a systemd timer or cron.
- Alert when a backup fails or hasn't run for a day, and **test restores** regularly: a backup you've never restored is only a hope.

@@quiz
Why compute file hashes with a stream rather than `readFile`?
* Large files are hashed chunk by chunk with constant memory
- Streams make hashes shorter
- readFile can't read binary files
- Hashes require streams
? `hash.update(chunk)` for each chunk gives the same digest as hashing it all at once.

@@quiz
Why copy to a temporary file and then rename?
- It's faster
* An interrupted copy never leaves a partial file under the real name
- Rename compresses the file
- Windows requires it
? Rename within the same volume is atomic.

@@quiz
Why plan before applying changes?
* It enables a dry run and makes the logic easy to test without touching files
- Planning is required by Node.js
- It makes copying faster
- To avoid hashing
? Separate deciding from doing.

@@drill
Which `crypto` function creates a hash object for checking whether a file changed?
= createHash
= crypto.createHash
? createHash("sha256").update(data).digest("hex").

@@flashcards
Incremental backup :: copy only what changed since last time
Manifest :: saved list of files with size, mtime and hash
Quick check :: compare size and mtime before hashing
Dry run :: show the plan without making changes
Mirror mode :: delete files from the backup that left the source
Atomic rename :: write temp file, then rename into place
`fs.mkdtemp` :: create a unique temporary directory
Restore test :: proving backups actually work

@@exercise Scan a directory into a manifest
Write `async function scan(root)` returning a manifest object: relative path (forward slashes, sorted keys) → `{ size, sha256 }` for every **regular file** under `root` (skip directories and symlinks). Hash each file with a read stream and `crypto.createHash("sha256")`.

Also write `diff(oldManifest, newManifest)` returning `{ added, changed, removed, unchanged }`, each a sorted array of paths (changed = same path, different hash).
@@starter
const fs = require("node:fs");
const fsp = require("node:fs/promises");
const path = require("node:path");
const crypto = require("node:crypto");

async function scan(root) {
  throw new Error("TODO: not done yet");
}

function diff(oldManifest, newManifest) {
  throw new Error("TODO: not done yet");
}

@@tests
const os = require("node:os");
(async () => {
  const root = await fsp.mkdtemp(path.join(os.tmpdir(), "scan-"));
  try {
    await fsp.mkdir(path.join(root, "docs", "deep"), { recursive: true });
    await fsp.writeFile(path.join(root, "docs", "a.txt"), "hello");
    await fsp.writeFile(path.join(root, "docs", "deep", "b.txt"), "x".repeat(200000));
    await fsp.writeFile(path.join(root, "z.cfg"), "");
    const m = await scan(root);
    check(Object.keys(m), ["docs/a.txt", "docs/deep/b.txt", "z.cfg"], "relative sorted paths");
    check(m["docs/a.txt"], { size: 5, sha256: "2cf24dba5fb0a30e26e83b2ac5b9e29e1b161e5c1fa7425e73043362938b9824" }, "size and hash");
    check(m["docs/deep/b.txt"].size, 200000, "big file");
    const later = { ...m, "docs/a.txt": { size: 5, sha256: "different" }, "new.txt": { size: 1, sha256: "n" } };
    delete later["z.cfg"];
    check(diff(m, later), { added: ["new.txt"], changed: ["docs/a.txt"], removed: ["z.cfg"], unchanged: ["docs/deep/b.txt"] }, "diff");
  } finally {
    await fsp.rm(root, { recursive: true, force: true });
  }
})();
@@hint
List files with `await fsp.readdir(root, { recursive: true, withFileTypes: true })` and keep `entry.isFile()`. The relative path is `path.relative(root, path.join(entry.parentPath, entry.name)).split(path.sep).join("/")`. Hash with `for await (const chunk of fs.createReadStream(file)) hash.update(chunk);`. Build the result from sorted keys.
@@solution
const fs = require("node:fs");
const fsp = require("node:fs/promises");
const path = require("node:path");
const crypto = require("node:crypto");

async function hashFile(file) {
  const hash = crypto.createHash("sha256");
  for await (const chunk of fs.createReadStream(file)) hash.update(chunk);
  return hash.digest("hex");
}

async function scan(root) {
  const entries = await fsp.readdir(root, { recursive: true, withFileTypes: true });
  const files = entries.filter((e) => e.isFile()).map((e) => path.join(e.parentPath, e.name));
  const manifest = {};
  const rows = await Promise.all(files.map(async (file) => {
    const rel = path.relative(root, file).split(path.sep).join("/");
    const { size } = await fsp.stat(file);
    return [rel, { size, sha256: await hashFile(file) }];
  }));
  for (const [rel, info] of rows.sort((a, b) => (a[0] < b[0] ? -1 : 1))) manifest[rel] = info;
  return manifest;
}

function diff(oldManifest, newManifest) {
  const result = { added: [], changed: [], removed: [], unchanged: [] };
  for (const [p, info] of Object.entries(newManifest)) {
    if (!Object.hasOwn(oldManifest, p)) result.added.push(p);
    else if (oldManifest[p].sha256 !== info.sha256) result.changed.push(p);
    else result.unchanged.push(p);
  }
  for (const p of Object.keys(oldManifest)) if (!Object.hasOwn(newManifest, p)) result.removed.push(p);
  for (const list of Object.values(result)) list.sort();
  return result;
}

@@exercise Plan and apply a backup
Write:

- `plan(changes, { mirror = false })`: from a `diff` result, return a list of actions: `{ op: "copy", path }` for added and changed paths, then (only in mirror mode) `{ op: "delete", path }` for removed paths, all in sorted path order within each group
- `async function apply(actions, source, target, { dryRun = false } = {})`: for each action, log a line `"copy docs/a.txt"` / `"delete old.txt"` into the returned `log`. Unless `dryRun`, perform it: copies go to `<target>/<path>.tmp` first (create folders as needed) and are then renamed to the final name; deletes remove the file (a missing file is fine). Return `{ log, copied, deleted }` (counts of actions **performed**, so 0 in dry-run mode).
@@starter
const fsp = require("node:fs/promises");
const path = require("node:path");

function plan(changes, { mirror = false } = {}) {
  throw new Error("TODO: not done yet");
}

async function apply(actions, source, target, { dryRun = false } = {}) {
  throw new Error("TODO: not done yet");
}

@@tests
const os = require("node:os");
const changes = { added: ["new/x.txt"], changed: ["a.txt"], removed: ["old.txt"], unchanged: ["same.txt"] };
check(plan(changes), [{ op: "copy", path: "a.txt" }, { op: "copy", path: "new/x.txt" }], "copy only");
check(plan(changes, { mirror: true }).at(-1), { op: "delete", path: "old.txt" }, "mirror deletes");
(async () => {
  const src = await fsp.mkdtemp(path.join(os.tmpdir(), "src-"));
  const dst = await fsp.mkdtemp(path.join(os.tmpdir(), "dst-"));
  try {
    await fsp.mkdir(path.join(src, "new"));
    await fsp.writeFile(path.join(src, "a.txt"), "v2");
    await fsp.writeFile(path.join(src, "new", "x.txt"), "x");
    await fsp.writeFile(path.join(dst, "a.txt"), "v1");
    await fsp.writeFile(path.join(dst, "old.txt"), "gone");
    const actions = plan(changes, { mirror: true });
    const dry = await apply(actions, src, dst, { dryRun: true });
    check([dry.log, dry.copied, dry.deleted], [["copy a.txt", "copy new/x.txt", "delete old.txt"], 0, 0], "dry run");
    check(await fsp.readFile(path.join(dst, "a.txt"), "utf8"), "v1", "dry run changed nothing");
    const real = await apply(actions, src, dst);
    check([real.copied, real.deleted], [2, 1], "counts");
    check(await fsp.readFile(path.join(dst, "new", "x.txt"), "utf8"), "x", "copied into a new folder");
    check((await fsp.readdir(dst, { recursive: true })).map((p) => p.replaceAll("\\", "/")).sort(), ["a.txt", "new", "new/x.txt"], "no temp files, old deleted");
    await apply([{ op: "delete", path: "old.txt" }], src, dst);
    check(true, true, "deleting a missing file is fine");
  } finally {
    await fsp.rm(src, { recursive: true, force: true });
    await fsp.rm(dst, { recursive: true, force: true });
  }
})();
@@hint
Build `[...changes.added, ...changes.changed].sort()` for copies. In `apply`, convert each relative path with `path.join(target, ...p.split("/"))`, `await fsp.mkdir(path.dirname(dest), { recursive: true })`, `await fsp.copyFile(from, dest + ".tmp")`, `await fsp.rename(dest + ".tmp", dest)`. For deletes use `await fsp.rm(dest, { force: true })`.
@@solution
const fsp = require("node:fs/promises");
const path = require("node:path");

function plan(changes, { mirror = false } = {}) {
  const actions = [...changes.added, ...changes.changed].sort().map((p) => ({ op: "copy", path: p }));
  if (mirror) actions.push(...[...changes.removed].sort().map((p) => ({ op: "delete", path: p })));
  return actions;
}

async function apply(actions, source, target, { dryRun = false } = {}) {
  const log = [];
  let copied = 0;
  let deleted = 0;
  for (const action of actions) {
    log.push(`${action.op} ${action.path}`);
    if (dryRun) continue;
    const parts = action.path.split("/");
    const dest = path.join(target, ...parts);
    if (action.op === "copy") {
      await fsp.mkdir(path.dirname(dest), { recursive: true });
      await fsp.copyFile(path.join(source, ...parts), dest + ".tmp");
      await fsp.rename(dest + ".tmp", dest);
      copied++;
    } else if (action.op === "delete") {
      await fsp.rm(dest, { force: true });
      deleted++;
    }
  }
  return { log, copied, deleted };
}
