@@lesson u9-forms Forms, Validation and Schemas
@@concepts objects, text-processing
@@text
Most business apps are forms: new ticket, add user, edit asset, change settings. Good forms validate clearly, keep what the user typed, and are validated **again** on the server, because anything in the browser can be bypassed.

## Controlled and uncontrolled inputs

```jsx,norun
// controlled: React state is the source of truth
const [title, setTitle] = useState("");
<input value={title} onChange={(e) => setTitle(e.target.value)} />

// uncontrolled: the DOM holds the value; read it on submit
<form action={async (formData) => { await createTicket(formData.get("title")); }}>
  <input name="title" required minLength={3} />
  <button>Create</button>
</form>
```

Controlled inputs make instant validation and dependent fields easy. Uncontrolled inputs with `FormData` are simpler and faster for big forms; React 19's form **actions** and `useActionState` build on them. Libraries like **React Hook Form** and **TanStack Form** handle the bookkeeping either way.

## Built-in HTML validation

`required`, `type="email"`, `min`/`max`, `minLength`, `pattern` give free browser validation and the right mobile keyboards. Style invalid fields with `:user-invalid` (only after the user has interacted). For custom messages, use `setCustomValidity`.

## Schemas: validate once, use everywhere

Instead of scattered `if` statements, describe the data's shape in a **schema**, and use it on both the client (instant feedback) and the server (security). **Zod** is the most popular schema library (Valibot, ArkType and Yup are alternatives):

```javascript,norun
import { z } from "zod";

const TicketInput = z.object({
  title: z.string().trim().min(3, "Title is too short").max(120),
  priority: z.coerce.number().int().min(1).max(4),
  email: z.string().email().optional(),
  tags: z.array(z.string()).max(5).default([]),
});

const result = TicketInput.safeParse(formValues);
if (!result.success) showErrors(result.error.flatten().fieldErrors);
else await save(result.data);              // typed and cleaned data
```

With TypeScript, `z.infer<typeof TicketInput>` gives you the type for free.

## Good form UX

- Validate a field when the user **leaves** it (blur), and re-validate as they fix it; don't shout errors before they've typed.
- Show messages **next to the field**, linked with `aria-describedby`, and mark the field with `aria-invalid="true"`.
- On submit with errors, move focus to the **first invalid field** and summarise errors at the top for long forms.
- Disable the submit button only **while submitting** (to prevent double submission), not while invalid (users then don't know why they can't click).
- Keep the user's input after a server error; never clear the form.
- Server errors (e.g. "email already registered") map back to fields too.

## Dirty state and leaving

Track which fields changed (`dirty`) to enable Save only when something changed, and warn before leaving a page with unsaved changes (`beforeunload` in browsers, a router blocker in single-page apps).

@@quiz
Why validate on the server when the browser already validates?
* Client-side checks can be bypassed (DevTools, scripts, direct API calls); the server must protect itself
- Browsers don't support validation
- It's faster
- To show nicer messages
? Client validation is for user experience; server validation is for correctness and security.

@@quiz
When is the best time to show a field's error message?
- As soon as the page loads
* After the user leaves the field (or submits), then live while they fix it
- Only after the server responds
- Never; just disable the button
? Early errors before typing are annoying.

@@quiz
What's the benefit of a schema library like Zod?
* One declarative description of the data used for validation, cleaning and types on both client and server
- It styles forms
- It replaces HTML
- It sends emails
? `safeParse` returns either cleaned data or structured errors.

@@flashcards
Controlled input :: value held in component state
Uncontrolled input :: value held by the DOM, read on submit
FormData :: browser object with a form's field values
Schema :: declarative description of valid data
`safeParse` :: validate without throwing; returns success or errors
`aria-invalid` :: marks a field as invalid for assistive tech
`aria-describedby` :: links a field to its error message
Dirty field :: changed from its initial value
`:user-invalid` :: CSS for invalid fields after interaction

@@exercise A tiny schema library
Write a mini Zod with chainable validators:

- `s.string()` with `.trim()`, `.min(n, msg)`, `.max(n, msg)`, `.email(msg)` (a simple `x@y.z` check), `.optional()`
- `s.number()` that also **coerces** numeric strings (`"3"` → 3; non-numeric → error `"Expected a number"`), with `.int(msg)`, `.min(n, msg)`, `.max(n, msg)`
- `s.object(shape)` with `.safeParse(input)` returning `{ success: true, data }` or `{ success: false, errors: { field: firstMessage } }`

Rules: a missing (`undefined` or empty-string) value fails with `"Required"` unless `.optional()` (then the key is left out of `data`). A string schema given a non-string fails with `"Expected a string"`. Default messages: min → `"Must be at least <n>"`, max → `"Must be at most <n>"`, email → `"Invalid email"`, int → `"Must be a whole number"`. Each method returns a **new** schema (don't mutate).
@@starter
const s = {
  string() { throw new Error("TODO: not done yet"); },
  number() { throw new Error("TODO: not done yet"); },
  object(shape) { throw new Error("TODO: not done yet"); },
};

@@tests
const Ticket = s.object({
  title: s.string().trim().min(3, "Title is too short").max(20),
  priority: s.number().int().min(1).max(4),
  email: s.string().email().optional(),
});
check(Ticket.safeParse({ title: "  VPN down ", priority: "2" }), { success: true, data: { title: "VPN down", priority: 2 } }, "valid and cleaned");
check(Ticket.safeParse({ title: "a", priority: 9, email: "nope" }), { success: false, errors: { title: "Title is too short", priority: "Must be at most 4", email: "Invalid email" } }, "errors");
check(Ticket.safeParse({ title: "x".repeat(30), priority: "2.5" }), { success: false, errors: { title: "Must be at most 20", priority: "Must be a whole number" } }, "max and int");
check(Ticket.safeParse({ priority: "abc" }), { success: false, errors: { title: "Required", priority: "Expected a number" } }, "required and coercion");
check(Ticket.safeParse({ title: 42, priority: 1 }).errors, { title: "Expected a string" }, "type error");
const base = s.string();
base.min(5);
check(s.object({ x: base }).safeParse({ x: "ab" }).success, true, "methods don't mutate");
@@hint
Represent a schema as `{ steps: [], isOptional: false, kind }` with a `parse(value)` that returns `{ ok, value }` or `{ ok: false, message }`. Each method returns `make(kind, [...steps, newStep], optional)`. A step is a function `(value) => ({ value }) | ({ error })`. The required/type/coercion checks run first, then the steps in order.
@@solution
function makeSchema(kind, steps = [], isOptional = false) {
  const add = (step) => makeSchema(kind, [...steps, step], isOptional);
  const schema = {
    kind,
    isOptional,
    optional: () => makeSchema(kind, steps, true),
    parse(input) {
      if (input === undefined || input === "") return { missing: true };
      let value = input;
      if (kind === "string" && typeof value !== "string") return { error: "Expected a string" };
      if (kind === "number") {
        const n = typeof value === "number" ? value : typeof value === "string" && value.trim() !== "" ? Number(value) : NaN;
        if (Number.isNaN(n)) return { error: "Expected a number" };
        value = n;
      }
      for (const step of steps) {
        const r = step(value);
        if (r.error) return r;
        value = r.value;
      }
      return { value };
    },
  };
  if (kind === "string") {
    Object.assign(schema, {
      trim: () => add((v) => ({ value: v.trim() })),
      min: (n, msg = `Must be at least ${n}`) => add((v) => (v.length < n ? { error: msg } : { value: v })),
      max: (n, msg = `Must be at most ${n}`) => add((v) => (v.length > n ? { error: msg } : { value: v })),
      email: (msg = "Invalid email") => add((v) => (/^[^\s@]+@[^\s@]+\.[^\s@]+$/.test(v) ? { value: v } : { error: msg })),
    });
  } else if (kind === "number") {
    Object.assign(schema, {
      int: (msg = "Must be a whole number") => add((v) => (Number.isInteger(v) ? { value: v } : { error: msg })),
      min: (n, msg = `Must be at least ${n}`) => add((v) => (v < n ? { error: msg } : { value: v })),
      max: (n, msg = `Must be at most ${n}`) => add((v) => (v > n ? { error: msg } : { value: v })),
    });
  }
  return schema;
}

const s = {
  string: () => makeSchema("string"),
  number: () => makeSchema("number"),
  object(shape) {
    return {
      safeParse(input) {
        const data = {};
        const errors = {};
        for (const [key, field] of Object.entries(shape)) {
          const r = field.parse(input?.[key]);
          if (r.missing) {
            if (!field.isOptional) errors[key] = "Required";
          } else if (r.error) {
            errors[key] = r.error;
          } else {
            data[key] = r.value;
          }
        }
        return Object.keys(errors).length ? { success: false, errors } : { success: true, data };
      },
    };
  },
};

@@exercise Form state
Write `createForm({ initial, validate })` (where `validate(values)` returns an object of field → message) returning an object with:

- `values` (starts as a copy of `initial`), `touched` (fields the user left), `errors` (shown errors), `submitting`
- `change(field, value)`: update the value; if the field is touched, re-validate it so errors update live
- `blur(field)`: mark touched and validate it
- `isDirty()`: true if any value differs from `initial`
- `async submit(save)`: mark **all** fields touched and validate everything; if invalid, return `{ ok: false, focus: <first field with an error in the order of initial's keys> }`. Otherwise set `submitting` while awaiting `save(values)`; if `save` throws an error with a `fieldErrors` object, merge those into `errors` and return `{ ok: false, focus }`; on success return `{ ok: true }` and reset `initial` to the saved values (so `isDirty()` is false).

Only touched fields' errors appear in `errors`.
@@starter
function createForm({ initial, validate }) {
  throw new Error("TODO: not done yet");
}

@@tests
const validate = (v) => {
  const e = {};
  if (v.title.trim().length < 3) e.title = "Too short";
  if (!/^\S+@\S+$/.test(v.email)) e.email = "Invalid email";
  return e;
};
(async () => {
  const f = createForm({ initial: { title: "", email: "", notes: "" }, validate });
  f.change("title", "a");
  check(f.errors, {}, "no error before blur");
  f.blur("title");
  check(f.errors, { title: "Too short" }, "error after blur");
  f.change("title", "abc");
  check(f.errors, {}, "fixed live");
  check(f.isDirty(), true, "dirty");
  check(await f.submit(async () => {}), { ok: false, focus: "email" }, "submit shows all errors");
  check(f.errors, { email: "Invalid email" }, "email error");
  f.change("email", "ana@corp");
  const taken = Object.assign(new Error("taken"), { fieldErrors: { email: "Already registered" } });
  check(await f.submit(async () => { throw taken; }), { ok: false, focus: "email" }, "server error mapped");
  check(f.errors.email, "Already registered", "server message");
  let during;
  check(await f.submit(async () => { during = f.submitting; }), { ok: true }, "saved");
  check([during, f.submitting, f.isDirty()], [true, false, false], "submitting flag and clean after save");
})();
@@hint
Keep `const form = { values: { ...initial }, touched: {}, errors: {}, submitting: false }`. A helper `refresh(fields)` runs `validate(form.values)` and, for each given field that is touched, sets or deletes `form.errors[field]`. Field order: `Object.keys(initial)`.
@@solution
function createForm({ initial, validate }) {
  let baseline = { ...initial };
  const order = Object.keys(initial);
  const form = { values: { ...initial }, touched: {}, errors: {}, submitting: false };

  function refresh(fields) {
    const all = validate(form.values);
    for (const field of fields) {
      if (!form.touched[field]) continue;
      if (all[field]) form.errors[field] = all[field];
      else delete form.errors[field];
    }
  }

  const firstError = () => order.find((f) => form.errors[f]);

  form.change = (field, value) => {
    form.values[field] = value;
    refresh([field]);
  };

  form.blur = (field) => {
    form.touched[field] = true;
    refresh([field]);
  };

  form.isDirty = () => order.some((f) => !Object.is(form.values[f], baseline[f]));

  form.submit = async (save) => {
    for (const f of order) form.touched[f] = true;
    refresh(order);
    if (firstError()) return { ok: false, focus: firstError() };
    form.submitting = true;
    try {
      await save({ ...form.values });
    } catch (error) {
      if (!error.fieldErrors) throw error;
      Object.assign(form.errors, error.fieldErrors);
      return { ok: false, focus: firstError() };
    } finally {
      form.submitting = false;
    }
    baseline = { ...form.values };
    return { ok: true };
  };

  return form;
}
