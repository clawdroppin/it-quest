@@lesson t6-project-query Project: A Type-Safe Query Builder
@@concepts functions, objects
@@text
Query builders like Kysely and Drizzle give you autocomplete for table and column names and an exact result type for each query. This project builds a small in-memory version with the same typing ideas: generics that **accumulate** information as methods are chained.

## The idea

```ts,norun
const rows = from(tickets)
  .where("status", "=", "open")
  .where("priority", ">=", 3)
  .orderBy("priority", "desc")
  .select("id", "title")
  .limit(2)
  .run();
// rows: { id: number; title: string }[]
```

Each `select` returns a builder with a **different type parameter**: `Query<Row, "id" | "title">`. The class stays immutable: every method returns a new builder, so a base query can be reused.

## Key typing techniques

```ts
type Op = "=" | "!=" | "<" | "<=" | ">" | ">=" | "in";
type ValueFor<V, O extends Op> = O extends "in" ? readonly V[] : V;

interface Ticket { id: number; title: string; status: "open" | "closed"; priority: number }
function where<Row, K extends keyof Row, O extends Op>(key: K, op: O, value: ValueFor<Row[K], O>): string {
  return `${String(key)} ${op} ${JSON.stringify(value)}`;
}
console.log(where<Ticket, "status", "in">("status", "in", ["open", "closed"]));
// where<Ticket, "status", "=">("status", "=", "pending");   // error: not "open" | "closed"
```

- `K extends keyof Row` restricts column names.
- `ValueFor<Row[K], O>` makes the value's type depend on both the column and the operator.
- `Pick<Row, Selected>` is the result row type.
- For schemas (exercise 2), a mapped type over the schema object computes the row type, the same trick Zod and Drizzle use.

## Schemas that produce types

```ts
type ColumnKind = "int" | "text" | "bool";
type KindType<K extends ColumnKind> = K extends "int" ? number : K extends "bool" ? boolean : string;
type RowOf<S extends Record<string, ColumnKind>> = { [C in keyof S]: KindType<S[C]> };

const ticketSchema = { id: "int", title: "text", urgent: "bool" } as const;
type TicketRow = RowOf<typeof ticketSchema>;      // { readonly id: number; readonly title: string; readonly urgent: boolean }
const t: TicketRow = { id: 1, title: "VPN", urgent: true };
console.log(t);
```

One runtime object is both the validator and the source of the static type, so they can never drift apart.

@@quiz
Why does each builder method return a new object instead of modifying `this`?
* So a base query can be reused safely and each step can have a different type
- Classes can't modify themselves
- For performance
- Because of erasure
? Immutable builders also make chained types honest.

@@quiz
In `Query<Row, Selected extends keyof Row>`, what is the result type of `run()`?
- Row[]
* `Pick<Row, Selected>[]`
- unknown[]
- Selected[]
? Only the selected columns are present.

@@quiz
Why does the builder reject `.where("agee", 30)` at compile time?
* The column parameter is typed as keyof Row, and 'agee' isn't a key
- It checks the database at runtime
- Strings are never allowed
- where() takes no arguments
? Typos in column names become type errors.

@@flashcards
Accumulating generics :: each chained call returns a builder with a refined type parameter
`Pick<Row, Selected>` :: result row with selected columns
`ValueFor<V, O>` :: value type depends on the operator
Immutable builder :: methods return new instances
Schema-derived type :: `RowOf<typeof schema>` keeps validator and type in sync

@@exercise The query builder
Write `from<Row>(rows: readonly Row[]): Query<Row, keyof Row>` and the class `Query<Row, Selected extends keyof Row>` with:

- `where<K extends keyof Row, O extends Op>(key: K, op: O, value: ValueFor<Row[K], O>): Query<Row, Selected>`: several `where`s combine with AND
- `orderBy(key: keyof Row, dir: "asc" | "desc" = "asc"): Query<Row, Selected>`: several `orderBy`s act as tie-breakers in call order
- `select<K extends keyof Row>(...keys: K[]): Query<Row, K>`
- `limit(n: number): Query<Row, Selected>`
- `run(): Pick<Row, Selected>[]`: filter, then sort (a stable sort), then limit, then pick columns (in the order given to `select`; all columns by default)
- `count(): number`: number of rows matching the filters, ignoring the limit

The builder must be immutable: the tests reuse a base query.
@@starter
type Op = "=" | "!=" | "<" | "<=" | ">" | ">=" | "in";
type ValueFor<V, O extends Op> = O extends "in" ? readonly V[] : V;

class Query<Row, Selected extends keyof Row> {
  // TODO: store rows, filters, ordering, selected columns and limit
  run(): Pick<Row, Selected>[] {
    throw new Error("TODO: not done yet");
  }
}

function from<Row>(rows: readonly Row[]): Query<Row, keyof Row> {
  throw new Error("TODO: not done yet");
}

@@tests
type Equal<X, Y> = (<T>() => T extends X ? 1 : 2) extends (<T>() => T extends Y ? 1 : 2) ? true : false;
type Expect<T extends true> = T;
interface Ticket { id: number; title: string; status: "open" | "closed"; priority: number; team: string }
const tickets: Ticket[] = [
  { id: 1, title: "VPN down", status: "open", priority: 5, team: "net" },
  { id: 2, title: "New laptop", status: "closed", priority: 2, team: "desk" },
  { id: 3, title: "Disk full", status: "open", priority: 4, team: "infra" },
  { id: 4, title: "Printer jam", status: "open", priority: 2, team: "desk" },
  { id: 5, title: "Cert expiry", status: "open", priority: 4, team: "infra" },
];
const open = from(tickets).where("status", "=", "open");
const top = open.where("priority", ">=", 3).orderBy("priority", "desc").orderBy("id", "desc").select("id", "title").limit(2).run();
type _t1 = Expect<Equal<typeof top, Pick<Ticket, "id" | "title">[]>>;
check(top, [{ id: 1, title: "VPN down" }, { id: 5, title: "Cert expiry" }], "chained query");
check(open.count(), 4, "base query reused");
check(open.where("team", "in", ["desk", "net"]).select("id").run(), [{ id: 1 }, { id: 4 }], "in");
check(from(tickets).where("title", "!=", "VPN down").where("priority", "<", 4).orderBy("team").select("team", "id").run(), [{ team: "desk", id: 2 }, { team: "desk", id: 4 }], "multiple filters and stable sort");
check(from(tickets).limit(1).run(), [tickets[0]], "all columns by default");
function _typeTests(): void {
  // @ts-expect-error unknown column
  from(tickets).where("owner", "=", "ana");
  // @ts-expect-error wrong value type for the column
  from(tickets).where("status", "=", "pending");
}
@@hint
Store state in one private readonly object `{ rows, filters: ((r: Row) => boolean)[], orders: { key: keyof Row; dir: 1 | -1 }[], columns: (keyof Row)[] | null, max: number | null }` and write a private `with(changes)` that returns `new Query({ ...this.state, ...changes })`. For `in`, `(value as readonly unknown[]).includes(r[key])`. Comparisons on generic values: `r[key] < (value as Row[K])` works because both have type `Row[K]`. `select` returns `new Query<Row, K>(...)`, which needs a cast: `as unknown as Query<Row, K>` is fine since the state doesn't depend on Selected.
@@solution
type Op = "=" | "!=" | "<" | "<=" | ">" | ">=" | "in";
type ValueFor<V, O extends Op> = O extends "in" ? readonly V[] : V;

interface QueryState<Row> {
  rows: readonly Row[];
  filters: ((row: Row) => boolean)[];
  orders: { key: keyof Row; dir: 1 | -1 }[];
  columns: (keyof Row)[] | null;
  max: number | null;
}

class Query<Row, Selected extends keyof Row> {
  private readonly state: QueryState<Row>;

  constructor(state: QueryState<Row>) {
    this.state = state;
  }

  private with(changes: Partial<QueryState<Row>>): Query<Row, Selected> {
    return new Query<Row, Selected>({ ...this.state, ...changes });
  }

  where<K extends keyof Row, O extends Op>(key: K, op: O, value: ValueFor<Row[K], O>): Query<Row, Selected> {
    const test = (row: Row): boolean => {
      const actual = row[key];
      if (op === "in") return (value as readonly unknown[]).includes(actual);
      const v = value as Row[K];
      switch (op) {
        case "=": return actual === v;
        case "!=": return actual !== v;
        case "<": return actual < v;
        case "<=": return actual <= v;
        case ">": return actual > v;
        default: return actual >= v;
      }
    };
    return this.with({ filters: [...this.state.filters, test] });
  }

  orderBy(key: keyof Row, dir: "asc" | "desc" = "asc"): Query<Row, Selected> {
    return this.with({ orders: [...this.state.orders, { key, dir: dir === "asc" ? 1 : -1 }] });
  }

  select<K extends keyof Row>(...keys: K[]): Query<Row, K> {
    return this.with({ columns: keys }) as unknown as Query<Row, K>;
  }

  limit(n: number): Query<Row, Selected> {
    return this.with({ max: n });
  }

  private matching(): Row[] {
    return this.state.rows.filter((row) => this.state.filters.every((f) => f(row)));
  }

  count(): number {
    return this.matching().length;
  }

  run(): Pick<Row, Selected>[] {
    const { orders, columns, max } = this.state;
    const sorted = this.matching().sort((a, b) => {
      for (const { key, dir } of orders) {
        if (a[key] < b[key]) return -dir;
        if (a[key] > b[key]) return dir;
      }
      return 0;
    });
    const limited = max === null ? sorted : sorted.slice(0, max);
    if (columns === null) return limited;
    return limited.map((row) => {
      const out: Partial<Row> = {};
      for (const c of columns) out[c] = row[c];
      return out as Pick<Row, Selected>;
    });
  }
}

function from<Row>(rows: readonly Row[]): Query<Row, keyof Row> {
  return new Query<Row, keyof Row>({ rows, filters: [], orders: [], columns: null, max: null });
}

@@exercise Typed CSV tables from a schema
Write `defineTable<const S extends Record<string, ColumnKind>>(name: string, schema: S)` returning an object with:

- `parse(csv: string): RowOf<S>[]`: the first line is a header (columns may come in any order and must include every schema column, else `Error("<name>: missing column <col>")`); extra columns are ignored; blank lines are skipped. `int` must match `/^-?\d+$/`, `bool` is `true`/`false`; a bad value throws `Error("<name> line <n>: bad <kind> in <col>")` where `n` is the 1-based line number in the file
- `columns: (keyof S)[]` in schema order

with `type ColumnKind = "int" | "text" | "bool"`, `KindType<K>` and `RowOf<S>` as in the lesson. Keep it simple: values don't contain commas.
@@starter
type ColumnKind = "int" | "text" | "bool";
type KindType<K extends ColumnKind> = K extends "int" ? number : K extends "bool" ? boolean : string;
type RowOf<S extends Record<string, ColumnKind>> = { [C in keyof S]: KindType<S[C]> };

function defineTable<const S extends Record<string, ColumnKind>>(name: string, schema: S): { columns: (keyof S)[]; parse(csv: string): RowOf<S>[] } {
  throw new Error("TODO: not done yet");
}

@@tests
type Equal<X, Y> = (<T>() => T extends X ? 1 : 2) extends (<T>() => T extends Y ? 1 : 2) ? true : false;
type Expect<T extends true> = T;
const hosts = defineTable("hosts", { name: "text", port: "int", prod: "bool" });
const rows = hosts.parse("port,owner,name,prod\n443,ana,web01,true\n\n5432,kenji,db01,false\n");
type _t1 = Expect<Equal<(typeof rows)[number], { readonly name: string; readonly port: number; readonly prod: boolean }>>;
check(rows, [{ name: "web01", port: 443, prod: true }, { name: "db01", port: 5432, prod: false }], "parsed rows");
check(hosts.columns, ["name", "port", "prod"], "columns");
let msg = "";
try { hosts.parse("name,port\nweb01,443"); } catch (e) { msg = (e as Error).message; }
check(msg, "hosts: missing column prod", "missing column");
try { hosts.parse("name,port,prod\nweb01,443,true\ndb01,54x,false"); } catch (e) { msg = (e as Error).message; }
check(msg, "hosts line 3: bad int in port", "bad value");
@@hint
Split into lines with their numbers first (`csv.split("\n").map((text, i) => ({ text, n: i + 1 }))`), take the header, then skip lines where `text.trim() === ""`. Map each schema column to its index in the header. Build each row in a `Record<string, unknown>` and cast to `RowOf<S>`.
@@solution
type ColumnKind = "int" | "text" | "bool";
type KindType<K extends ColumnKind> = K extends "int" ? number : K extends "bool" ? boolean : string;
type RowOf<S extends Record<string, ColumnKind>> = { [C in keyof S]: KindType<S[C]> };

function convert(kind: ColumnKind, raw: string): unknown {
  if (kind === "int") return /^-?\d+$/.test(raw) ? Number(raw) : undefined;
  if (kind === "bool") return raw === "true" ? true : raw === "false" ? false : undefined;
  return raw;
}

function defineTable<const S extends Record<string, ColumnKind>>(name: string, schema: S): { columns: (keyof S)[]; parse(csv: string): RowOf<S>[] } {
  const entries = Object.entries(schema) as [string, ColumnKind][];
  return {
    columns: entries.map(([col]) => col) as (keyof S)[],
    parse(csv: string): RowOf<S>[] {
      const lines = csv.split("\n").map((text, i) => ({ text: text.trim(), n: i + 1 }));
      const header = (lines[0]?.text ?? "").split(",");
      const index = new Map<string, number>();
      for (const [col] of entries) {
        const i = header.indexOf(col);
        if (i < 0) throw new Error(`${name}: missing column ${col}`);
        index.set(col, i);
      }
      return lines.slice(1).filter((l) => l.text !== "").map(({ text, n }) => {
        const cells = text.split(",");
        const row: Record<string, unknown> = {};
        for (const [col, kind] of entries) {
          const value = convert(kind, cells[index.get(col) ?? -1] ?? "");
          if (value === undefined) throw new Error(`${name} line ${n}: bad ${kind} in ${col}`);
          row[col] = value;
        }
        return row as RowOf<S>;
      });
    },
  };
}
