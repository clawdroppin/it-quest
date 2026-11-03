@@lesson t8-tauri Desktop Apps with Tauri: Typed Commands, Events and Persisted Settings
@@concepts functions, json, files
@@text
**Tauri** builds desktop apps from a web frontend (React, Svelte, Vue, plain TS) and a **Rust** backend, producing small native installers for Windows, macOS and Linux (and mobile). This very learning app is a Tauri app. **Electron** is the bigger, older alternative with a Node.js backend.

## How the pieces talk

```text
┌──────────── WebView (TypeScript UI) ────────────┐
│ invoke("list_devices", { site: "tokyo" })  ──────┼──► #[tauri::command] fn list_devices(site: String) -> Result<Vec<Device>, String>
│ listen("scan-progress", (e) => ...)        ◄─────┼─── app.emit("scan-progress", Progress { done: 3, total: 10 })
└──────────────────────────────────────────────────┘
```

- **Commands**: the frontend calls Rust functions with `invoke`, passing JSON arguments; the result comes back as a Promise. Rust `Result::Err` becomes a rejected Promise.
- **Events**: either side emits named events with JSON payloads; the other side listens.
- **Capabilities/permissions**: Tauri 2 only lets the frontend call the commands and plugins (fs, dialog, shell...) that the config allows. Keep that list minimal.

```rust,norun
#[derive(serde::Serialize)]
struct Device { serial: String, site: String }

#[tauri::command]
fn list_devices(site: Option<String>) -> Result<Vec<Device>, String> {
    // talk to SQLite, the OS, the network...
    Ok(vec![])
}
```

## Keeping both sides' types in sync

`invoke` is untyped by default (`invoke<T>(cmd: string, args?: object): Promise<T>`). Typos in command names or argument names only fail at runtime. Fixes:

1. Write a **command map type** and a typed wrapper (the exercise below).
2. **Generate** TypeScript from the Rust types with `tauri-specta` or `ts-rs`, so a changed Rust struct breaks the TS build.

Note serde's naming: Tauri converts command **argument** names from camelCase in JS to snake_case in Rust automatically, but struct **fields** keep their Rust names unless you add `#[serde(rename_all = "camelCase")]`.

## Persisted settings and migrations

Desktop apps store settings and data locally (a JSON file in the app data folder, SQLite, or the `store` plugin). Because users skip versions, stored data needs a **version number** and **migrations** that upgrade old shapes step by step, much like database migrations. Validate after loading: files get corrupted and hand-edited.

## Desktop UX checklist

- Native-feeling menus and keyboard shortcuts (Ctrl/Cmd+S, Ctrl/Cmd+F), and remember window size/position.
- Long work runs in the backend with progress events, never blocking the UI thread.
- Offline first: the app should work without a network and sync later.
- Auto-update with signed releases, crash reporting, and logs the user can send you.
- Accessibility still applies: focus order, labels, contrast, scalable text.

@@quiz
How does a Tauri frontend call Rust code?
* `invoke("command_name", args)`, which returns a Promise resolved with the command's serialized result
- Through `require`
- By importing the Rust file
- With HTTP requests to localhost only
? Commands are declared with `#[tauri::command]`.

@@quiz
What happens to a Rust command returning `Err("disk full".into())`?
- The app crashes
* The `invoke` Promise rejects with that error
- It returns undefined
- It's ignored
? Handle it with try/catch or `.catch`.

@@quiz
Why do persisted settings need a version number?
- For display purposes
* So the app can migrate older stored shapes step by step when users upgrade
- Tauri requires it
- To encrypt them
? Users skip versions; files outlive app releases.

@@flashcards
Tauri :: web UI + Rust backend desktop framework
Electron :: web UI + Node.js backend desktop framework
`invoke` :: call a Rust command from the frontend
`#[tauri::command]` :: marks a Rust function callable from the UI
Events :: `emit` / `listen` with JSON payloads, both directions
Capabilities :: allowlist of commands and plugins per window
tauri-specta / ts-rs :: generate TS types from Rust
Settings migration :: upgrade stored data from version N to N+1

@@exercise Typed invoke and events
Describe the backend with type maps and write typed wrappers around an untyped bridge:

- `type Commands = { list_devices: { args: { site?: string }; result: Device[] }; rename_device: { args: { serial: string; name: string }; result: Device }; scan_network: { args: { cidr: string }; result: number } }`
- `type Events = { "scan-progress": { done: number; total: number }; "device-added": Device }`
- `createBridge(raw: RawBridge)` returning:
  - `invoke<C extends keyof Commands>(cmd: C, args: Commands[C]["args"]): Promise<Commands[C]["result"]>`; a rejection from the raw bridge with a string becomes `Error("<cmd> failed: <message>")`
  - `listen<E extends keyof Events>(event: E, handler: (payload: Events[E]) => void): () => void`
  - `once<E extends keyof Events>(event: E): Promise<Events[E]>`: resolves with the next payload, then stops listening

`RawBridge` (given) is untyped, like Tauri's real API.
@@starter
type Device = { serial: string; name: string; site: string };
type Commands = {
  list_devices: { args: { site?: string }; result: Device[] };
  rename_device: { args: { serial: string; name: string }; result: Device };
  scan_network: { args: { cidr: string }; result: number };
};
type Events = { "scan-progress": { done: number; total: number }; "device-added": Device };
type RawBridge = {
  invoke(cmd: string, args: object): Promise<unknown>;
  listen(event: string, handler: (payload: unknown) => void): () => void;
};

function createBridge(raw: RawBridge) {
  return {
    invoke<C extends keyof Commands>(cmd: C, args: Commands[C]["args"]): Promise<Commands[C]["result"]> {
      throw new Error("TODO: not done yet");
    },
    listen<E extends keyof Events>(event: E, handler: (payload: Events[E]) => void): () => void {
      throw new Error("TODO: not done yet");
    },
    once<E extends keyof Events>(event: E): Promise<Events[E]> {
      throw new Error("TODO: not done yet");
    },
  };
}

@@tests
const handlers = new Map<string, Set<(p: unknown) => void>>();
const devices: Device[] = [{ serial: "LT-1", name: "ana-laptop", site: "tokyo" }];
const raw: RawBridge = {
  async invoke(cmd, args) {
    const a = args as Record<string, string>;
    if (cmd === "list_devices") return devices.filter((d) => !a.site || d.site === a.site);
    if (cmd === "rename_device") {
      const d = devices.find((x) => x.serial === a.serial);
      if (!d) throw "no such device";
      d.name = a.name ?? d.name;
      return d;
    }
    throw "unknown command";
  },
  listen(event, handler) {
    if (!handlers.has(event)) handlers.set(event, new Set());
    handlers.get(event)?.add(handler);
    return () => handlers.get(event)?.delete(handler);
  },
};
const emit = (event: string, payload: unknown) => handlers.get(event)?.forEach((h) => h(payload));
const bridge = createBridge(raw);
(async () => {
  check((await bridge.invoke("list_devices", { site: "tokyo" })).map((d) => d.serial), ["LT-1"], "invoke");
  const renamed = await bridge.invoke("rename_device", { serial: "LT-1", name: "spare" });
  check(renamed.name, "spare", "typed result");
  let msg = "";
  try { await bridge.invoke("rename_device", { serial: "XX", name: "x" }); } catch (e) { msg = (e as Error).message; }
  check(msg, "rename_device failed: no such device", "error wrapped");
  const progress: string[] = [];
  const stop = bridge.listen("scan-progress", (p) => progress.push(`${p.done}/${p.total}`));
  emit("scan-progress", { done: 1, total: 4 });
  emit("scan-progress", { done: 2, total: 4 });
  stop();
  emit("scan-progress", { done: 3, total: 4 });
  check(progress, ["1/4", "2/4"], "listen and unlisten");
  const next = bridge.once("device-added");
  emit("device-added", { serial: "PH-9", name: "new", site: "osaka" });
  emit("device-added", { serial: "PH-10", name: "later", site: "osaka" });
  check([(await next).serial, handlers.get("device-added")?.size], ["PH-9", 0], "once");
})();
function _typeTests(): void {
  // @ts-expect-error unknown command
  bridge.invoke("delete_everything", {});
  // @ts-expect-error missing name
  bridge.invoke("rename_device", { serial: "LT-1" });
}
@@hint
`invoke`: `raw.invoke(cmd, args).then((r) => r as Commands[C]["result"], (e: unknown) => { throw new Error(`${cmd} failed: ${e instanceof Error ? e.message : String(e)}`); })`. `listen`: `raw.listen(event, (p) => handler(p as Events[E]))`. `once`: `new Promise((resolve) => { const stop = raw.listen(event, (p) => { stop(); resolve(p as Events[E]); }); })`.
@@solution
type Device = { serial: string; name: string; site: string };
type Commands = {
  list_devices: { args: { site?: string }; result: Device[] };
  rename_device: { args: { serial: string; name: string }; result: Device };
  scan_network: { args: { cidr: string }; result: number };
};
type Events = { "scan-progress": { done: number; total: number }; "device-added": Device };
type RawBridge = {
  invoke(cmd: string, args: object): Promise<unknown>;
  listen(event: string, handler: (payload: unknown) => void): () => void;
};

function createBridge(raw: RawBridge) {
  return {
    invoke<C extends keyof Commands>(cmd: C, args: Commands[C]["args"]): Promise<Commands[C]["result"]> {
      return raw.invoke(cmd, args).then(
        (r) => r as Commands[C]["result"],
        (e: unknown) => {
          throw new Error(`${cmd} failed: ${e instanceof Error ? e.message : String(e)}`);
        },
      );
    },
    listen<E extends keyof Events>(event: E, handler: (payload: Events[E]) => void): () => void {
      return raw.listen(event, (p) => handler(p as Events[E]));
    },
    once<E extends keyof Events>(event: E): Promise<Events[E]> {
      return new Promise((resolve) => {
        const stop = raw.listen(event, (p) => {
          stop();
          resolve(p as Events[E]);
        });
      });
    },
  };
}

@@exercise Versioned settings with migrations
Settings are stored as JSON with a `version` field. Write `loadSettings(json: string | null): { settings: SettingsV3; migratedFrom: number | null; warnings: string[] }`:

- `null` (no file) → defaults, `migratedFrom: null`
- invalid JSON or a missing/unknown version → defaults with warning `"settings unreadable, using defaults"`
- version 1 `{ version: 1, dark: boolean, fontSize: number }` → v2: `theme: dark ? "dark" : "light"`, keeps `fontSize`
- version 2 `{ version: 2, theme, fontSize }` → v3: `ui: { theme, fontScale: fontSize / 16 }`, adds `recent: []`
- version 3 is current: `{ version: 3, ui: { theme: "light" | "dark" | "system", fontScale: number }, recent: string[] }`
- apply migrations step by step (1 → 2 → 3); `migratedFrom` is the starting version if any migration ran
- after migrating, clamp `fontScale` to 0.75–2 (warning `"fontScale out of range, clamped"`), and keep only the 10 most recent entries (the first 10)

Also write `saveSettings(s: SettingsV3): string` (pretty JSON with 2 spaces).
@@starter
type Theme = "light" | "dark" | "system";
type SettingsV1 = { version: 1; dark: boolean; fontSize: number };
type SettingsV2 = { version: 2; theme: "light" | "dark"; fontSize: number };
type SettingsV3 = { version: 3; ui: { theme: Theme; fontScale: number }; recent: string[] };
const DEFAULTS: SettingsV3 = { version: 3, ui: { theme: "system", fontScale: 1 }, recent: [] };

function loadSettings(json: string | null): { settings: SettingsV3; migratedFrom: number | null; warnings: string[] } {
  throw new Error("TODO: not done yet");
}

function saveSettings(s: SettingsV3): string {
  throw new Error("TODO: not done yet");
}

@@tests
check(loadSettings(null), { settings: DEFAULTS, migratedFrom: null, warnings: [] }, "no file");
check(loadSettings('{"version":1,"dark":true,"fontSize":20}'), { settings: { version: 3, ui: { theme: "dark", fontScale: 1.25 }, recent: [] }, migratedFrom: 1, warnings: [] }, "from v1");
check(loadSettings('{"version":2,"theme":"light","fontSize":48}'), { settings: { version: 3, ui: { theme: "light", fontScale: 2 }, recent: [] }, migratedFrom: 2, warnings: ["fontScale out of range, clamped"] }, "from v2 with clamp");
const many = Array.from({ length: 12 }, (_, i) => `file${i}.md`);
const v3 = loadSettings(JSON.stringify({ version: 3, ui: { theme: "system", fontScale: 1.1 }, recent: many }));
check([v3.migratedFrom, v3.settings.recent.length, v3.settings.recent[0]], [null, 10, "file0.md"], "current version, recent trimmed");
check(loadSettings("{oops").warnings, ["settings unreadable, using defaults"], "corrupt");
check(loadSettings('{"version":9}').settings, DEFAULTS, "unknown version");
check(saveSettings(DEFAULTS).split("\n").length, 8, "pretty JSON");
@@hint
Parse to `unknown`, read `version`, then use a loop: `let current: SettingsV1 | SettingsV2 | SettingsV3 = parsed as ...; while (current.version !== 3) current = current.version === 1 ? v1to2(current) : v2to3(current);`. TypeScript narrows on `current.version` inside the loop. Copy objects rather than mutating `DEFAULTS`.
@@solution
type Theme = "light" | "dark" | "system";
type SettingsV1 = { version: 1; dark: boolean; fontSize: number };
type SettingsV2 = { version: 2; theme: "light" | "dark"; fontSize: number };
type SettingsV3 = { version: 3; ui: { theme: Theme; fontScale: number }; recent: string[] };
const DEFAULTS: SettingsV3 = { version: 3, ui: { theme: "system", fontScale: 1 }, recent: [] };

const v1to2 = (s: SettingsV1): SettingsV2 => ({ version: 2, theme: s.dark ? "dark" : "light", fontSize: s.fontSize });
const v2to3 = (s: SettingsV2): SettingsV3 => ({ version: 3, ui: { theme: s.theme, fontScale: s.fontSize / 16 }, recent: [] });
const fresh = (): SettingsV3 => ({ ...DEFAULTS, ui: { ...DEFAULTS.ui }, recent: [] });

function loadSettings(json: string | null): { settings: SettingsV3; migratedFrom: number | null; warnings: string[] } {
  if (json === null) return { settings: fresh(), migratedFrom: null, warnings: [] };
  const unreadable = { settings: fresh(), migratedFrom: null, warnings: ["settings unreadable, using defaults"] };
  let parsed: unknown;
  try {
    parsed = JSON.parse(json);
  } catch {
    return unreadable;
  }
  if (typeof parsed !== "object" || parsed === null || !("version" in parsed)) return unreadable;
  const version = parsed.version;
  if (version !== 1 && version !== 2 && version !== 3) return unreadable;
  let current = parsed as SettingsV1 | SettingsV2 | SettingsV3;
  while (current.version !== 3) current = current.version === 1 ? v1to2(current) : v2to3(current);
  const warnings: string[] = [];
  let fontScale = current.ui.fontScale;
  if (fontScale < 0.75 || fontScale > 2) {
    fontScale = Math.min(2, Math.max(0.75, fontScale));
    warnings.push("fontScale out of range, clamped");
  }
  const settings: SettingsV3 = { version: 3, ui: { theme: current.ui.theme, fontScale }, recent: current.recent.slice(0, 10) };
  return { settings, migratedFrom: version === 3 ? null : version, warnings };
}

function saveSettings(s: SettingsV3): string {
  return JSON.stringify(s, null, 2);
}
