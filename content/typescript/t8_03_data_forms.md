@@lesson t8-data-forms Typed Data Fetching and Forms
@@concepts functions, objects, networking
@@text
Most screens in business apps are "load data, show it, edit it, save it". This lesson types both ends: a fetch client that knows every endpoint, and form state with typed values and errors.

## A typed API client

Describe the API once as a type map, and the client derives everything from it:

```ts
type Api = {
  "GET /devices": { query: { site?: string }; response: { serial: string; site: string }[] };
  "POST /devices": { body: { serial: string; site: string }; response: { serial: string } };
};

type Endpoint = keyof Api;                        // "GET /devices" | "POST /devices"
type ResponseOf<E extends Endpoint> = Api[E]["response"];
const r: ResponseOf<"POST /devices"> = { serial: "LT-1" };
console.log(r);
```

In real projects this map is often **generated** from an OpenAPI spec (`openapi-typescript`) or inferred from the server (tRPC, Hono). The exercise builds a client on top of such a map.

## Loading states as a union

Avoid three independent booleans (`loading`, `error`, `data`) that can contradict each other. Use one union:

```ts
type Remote<T> =
  | { state: "idle" }
  | { state: "loading" }
  | { state: "error"; message: string }
  | { state: "ready"; data: T };

function label(r: Remote<string[]>): string {
  switch (r.state) {
    case "idle": return "";
    case "loading": return "Loading…";
    case "error": return `Couldn't load: ${r.message}`;
    case "ready": return `${r.data.length} items`;
  }
}
console.log(label({ state: "ready", data: ["a", "b"] }));
```

`data` can only be read when the state is `ready`, which removes a classic UI bug.

## Fetching in React

```tsx,norun
const { data, error, isPending } = useQuery({
  queryKey: ["devices", site],
  queryFn: ({ signal }) => api.get("GET /devices", { query: { site } }, signal),
});
```

TanStack Query handles caching, deduplication, retries, refetch on focus and cancellation (`signal`). Mutations (`useMutation`) invalidate queries after saving.

## Forms

Forms need: values, which fields were touched, validation errors per field, and submission state. Libraries like React Hook Form and TanStack Form type all of it from the values type, and accept a Zod schema as the validator.

```tsx,norun
const form = useForm<DeviceForm>({ resolver: zodResolver(DeviceSchema), defaultValues: { serial: "", ramGb: 16 } });
<input {...form.register("serial")} />
{form.formState.errors.serial?.message}
```

Good form UX rules:

- Validate on blur and on submit, not on every keystroke for untouched fields.
- Show errors next to the field, connected with `aria-describedby`; move focus to the first invalid field on submit.
- Disable the submit button only while submitting, not while invalid (users need to see why it's invalid).
- Keep server-side validation too: the client can be bypassed.

@@quiz
Why model loading state as a union instead of `loading`, `error` and `data` booleans/fields?
* Impossible combinations (like loading with an error and data) can't be represented
- It's faster
- React requires it
- It uses less memory
? Make illegal states unrepresentable.

@@quiz
When should a form typically show a field's validation error?
- On every keystroke from the start
* After the field is touched (blurred) or on submit
- Only after a successful submit
- Never; disable the button instead
? Don't scold users before they've finished typing.

@@quiz
Why validate on the server even with client-side validation?
- It's faster
* The client can be bypassed or outdated; the server is the trust boundary
- Browsers don't support validation
- To show nicer messages
? Client validation is UX; server validation is security.

@@flashcards
API type map :: endpoint → { query, body, response }
`Remote<T>` :: idle / loading / error / ready union
TanStack Query :: server-state caching and fetching
`queryKey` :: cache key for a query
Touched :: field the user has interacted with
Field errors :: `Partial<Record<keyof Values, string>>`
`aria-describedby` :: connects an error message to its input
openapi-typescript :: generates TS types from an OpenAPI spec

@@exercise A typed API client
Given an API map type, write `createClient<A extends ApiMap>(fetcher: Fetcher)` returning an object with:

- `call<E extends keyof A & string>(endpoint: E, input: InputOf<A[E]>): Promise<A[E]["response"]>`

where `type ApiMap = Record<string, { query?: object; body?: unknown; response: unknown }>` and `InputOf<D> = { query?: D["query"]; body?: D["body"] }` (both optional, but typed per endpoint).

Runtime behaviour:

- the endpoint string is `"METHOD /path"`; `:param` segments in the path are filled from `query` (and removed from it, URL-encoded)
- remaining defined `query` entries become a query string, sorted by key
- `fetcher({ method, url, body })` returns `{ status: number; json: unknown }`; status 2xx → resolve with `json`; otherwise reject with `ApiError` (`status`, `message` = `json.error` if it's a string, else `"HTTP <status>"`)
@@starter
type ApiMap = Record<string, { query?: object; body?: unknown; response: unknown }>;
type InputOf<D extends { query?: object; body?: unknown }> = { query?: D["query"]; body?: D["body"] };
type Fetcher = (req: { method: string; url: string; body?: unknown }) => Promise<{ status: number; json: unknown }>;

class ApiError extends Error {
  readonly status: number;
  constructor(status: number, message: string) {
    super(message);
    this.name = "ApiError";
    this.status = status;
  }
}

function createClient<A extends ApiMap>(fetcher: Fetcher) {
  return {
    call<E extends keyof A & string>(endpoint: E, input: InputOf<A[E]> = {}): Promise<A[E]["response"]> {
      throw new Error("TODO: not done yet");
    },
  };
}

@@tests
type DeviceApi = {
  "GET /devices": { query: { site?: string; owner?: string }; response: { serial: string }[] };
  "GET /devices/:serial": { query: { serial: string }; response: { serial: string; owner: string } };
  "POST /devices": { body: { serial: string }; response: { serial: string } };
};
const requests: string[] = [];
const fake: Fetcher = async ({ method, url, body }) => {
  requests.push(`${method} ${url}${body === undefined ? "" : " " + JSON.stringify(body)}`);
  if (url.startsWith("/devices/missing")) return { status: 404, json: { error: "no such device" } };
  if (url === "/devices/boom") return { status: 500, json: null };
  if (method === "POST") return { status: 201, json: body };
  return { status: 200, json: url.includes("?") ? [{ serial: "LT-1" }] : { serial: "LT 7", owner: "ana" } };
};
const api = createClient<DeviceApi>(fake);
(async () => {
  const list = await api.call("GET /devices", { query: { site: "tokyo", owner: undefined } });
  check(list, [{ serial: "LT-1" }], "list");
  const one = await api.call("GET /devices/:serial", { query: { serial: "LT 7" } });
  check(one.owner, "ana", "typed response");
  await api.call("POST /devices", { body: { serial: "PH-1" } });
  let e1 = "";
  try { await api.call("GET /devices/:serial", { query: { serial: "missing" } }); } catch (e) { e1 = e instanceof ApiError ? `${e.status} ${e.message}` : "?"; }
  let e2 = "";
  try { await api.call("GET /devices/:serial", { query: { serial: "boom" } }); } catch (e) { e2 = (e as ApiError).message; }
  check([e1, e2], ["404 no such device", "HTTP 500"], "errors");
  check(requests, ["GET /devices?site=tokyo", "GET /devices/LT%207", 'POST /devices {"serial":"PH-1"}', "GET /devices/missing", "GET /devices/boom"], "requests");
})();
function _typeTests(): void {
  // @ts-expect-error unknown endpoint
  api.call("DELETE /devices", {});
  // @ts-expect-error body has the wrong shape
  api.call("POST /devices", { body: { id: 1 } });
}
@@hint
`const [method = "GET", path = ""] = endpoint.split(" ")`. Copy the query into `const q: Record<string, unknown> = { ...(input.query ?? {}) }`, then `path.replace(/:(\w+)/g, (_, name: string) => { const v = q[name]; delete q[name]; return encodeURIComponent(String(v)); })`. Build the rest with `URLSearchParams` over sorted, defined entries.
@@solution
type ApiMap = Record<string, { query?: object; body?: unknown; response: unknown }>;
type InputOf<D extends { query?: object; body?: unknown }> = { query?: D["query"]; body?: D["body"] };
type Fetcher = (req: { method: string; url: string; body?: unknown }) => Promise<{ status: number; json: unknown }>;

class ApiError extends Error {
  readonly status: number;
  constructor(status: number, message: string) {
    super(message);
    this.name = "ApiError";
    this.status = status;
  }
}

function createClient<A extends ApiMap>(fetcher: Fetcher) {
  return {
    async call<E extends keyof A & string>(endpoint: E, input: InputOf<A[E]> = {}): Promise<A[E]["response"]> {
      const [method = "GET", path = ""] = endpoint.split(" ");
      const q: Record<string, unknown> = { ...(input.query ?? {}) };
      const filled = path.replace(/:(\w+)/g, (_m, name: string) => {
        const v = q[name];
        delete q[name];
        return encodeURIComponent(String(v));
      });
      const entries = Object.entries(q)
        .filter(([, v]) => v !== undefined)
        .sort(([a], [b]) => a.localeCompare(b))
        .map(([k, v]): [string, string] => [k, String(v)]);
      const qs = new URLSearchParams(entries).toString();
      const url = qs ? `${filled}?${qs}` : filled;
      const res = await fetcher(input.body === undefined ? { method, url } : { method, url, body: input.body });
      if (res.status >= 200 && res.status < 300) return res.json as A[E]["response"];
      const j = res.json;
      const message = typeof j === "object" && j !== null && "error" in j && typeof j.error === "string" ? j.error : `HTTP ${res.status}`;
      throw new ApiError(res.status, message);
    },
  };
}

@@exercise Form state with typed errors
Write `createForm<V extends Record<string, unknown>>(initial: V, validate: (values: V) => Partial<Record<keyof V, string>>)` returning:

- `set<K extends keyof V>(field: K, value: V[K]): void`
- `blur(field: keyof V): void`: marks the field touched
- `submit(onValid: (values: V) => void): boolean`: marks **all** fields touched; calls `onValid` with a copy of the values and returns `true` only if there are no errors
- `view(): { values: V; visibleErrors: Partial<Record<keyof V, string>>; firstInvalid: keyof V | null; dirty: boolean }`: errors are visible only for touched fields; `firstInvalid` is the first field (in `initial`'s key order) with a visible error; `dirty` is true if any value differs from `initial` (`Object.is`)
@@starter
function createForm<V extends Record<string, unknown>>(initial: V, validate: (values: V) => Partial<Record<keyof V, string>>) {
  return {
    set<K extends keyof V>(field: K, value: V[K]): void {
      throw new Error("TODO: not done yet");
    },
    blur(field: keyof V): void {
      throw new Error("TODO: not done yet");
    },
    submit(onValid: (values: V) => void): boolean {
      throw new Error("TODO: not done yet");
    },
    view(): { values: V; visibleErrors: Partial<Record<keyof V, string>>; firstInvalid: keyof V | null; dirty: boolean } {
      throw new Error("TODO: not done yet");
    },
  };
}

@@tests
type DeviceForm = { serial: string; ramGb: number; owner: string };
const form = createForm<DeviceForm>({ serial: "", ramGb: 16, owner: "" }, (v) => {
  const e: Partial<Record<keyof DeviceForm, string>> = {};
  if (!/^[A-Z]{2}-\d+$/.test(v.serial)) e.serial = "Use a serial like LT-12";
  if (v.ramGb < 4) e.ramGb = "At least 4 GB";
  if (v.owner.trim() === "") e.owner = "Owner is required";
  return e;
});
check(form.view().visibleErrors, {}, "no errors before touching");
form.set("serial", "lt12");
form.blur("serial");
check([form.view().visibleErrors, form.view().dirty], [{ serial: "Use a serial like LT-12" }, true], "touched field shows error");
const saved: DeviceForm[] = [];
check(form.submit((v) => saved.push(v)), false, "invalid submit");
check([form.view().firstInvalid, Object.keys(form.view().visibleErrors)], ["serial", ["serial", "owner"]], "all touched after submit");
form.set("serial", "LT-12");
form.set("owner", "ana");
check([form.submit((v) => saved.push(v)), saved], [true, [{ serial: "LT-12", ramGb: 16, owner: "ana" }]], "valid submit");
form.set("serial", "");
form.set("serial", "LT-12");
form.set("owner", "");
form.set("owner", "ana");
check(saved[0] === form.view().values, false, "submit passes a copy");
@@hint
Keep `let values = { ...initial }` and a `Set<keyof V>` of touched fields. In `view`, run `validate(values)` and keep the entries whose field is touched; iterate `Object.keys(initial) as (keyof V)[]` to keep a stable order.
@@solution
function createForm<V extends Record<string, unknown>>(initial: V, validate: (values: V) => Partial<Record<keyof V, string>>) {
  let values: V = { ...initial };
  const touched = new Set<keyof V>();
  const fields = Object.keys(initial) as (keyof V)[];
  return {
    set<K extends keyof V>(field: K, value: V[K]): void {
      values = { ...values, [field]: value };
    },
    blur(field: keyof V): void {
      touched.add(field);
    },
    submit(onValid: (values: V) => void): boolean {
      for (const f of fields) touched.add(f);
      const errors = validate(values);
      if (fields.some((f) => errors[f] !== undefined)) return false;
      onValid({ ...values });
      return true;
    },
    view(): { values: V; visibleErrors: Partial<Record<keyof V, string>>; firstInvalid: keyof V | null; dirty: boolean } {
      const errors = validate(values);
      const visibleErrors: Partial<Record<keyof V, string>> = {};
      let firstInvalid: keyof V | null = null;
      for (const f of fields) {
        const message = errors[f];
        if (message !== undefined && touched.has(f)) {
          visibleErrors[f] = message;
          firstInvalid ??= f;
        }
      }
      return { values, visibleErrors, firstInvalid, dirty: fields.some((f) => !Object.is(values[f], initial[f])) };
    },
  };
}
