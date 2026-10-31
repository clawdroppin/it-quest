@@lesson xp-csv_typed Type-safe CSV Importer
@@text
Real data is messy. Write a generic CSV importer: describe the columns once (name, type, required), and get back fully typed records plus a list of row-level errors ("row 7: 'ram_gb' is not a number"), using TypeScript generics and mapped types so the result type follows from the description.

**You'll practise:** generics · mapped and conditional types · parsing quoted CSV · error reporting

## Milestones

1. **Parser**: split lines into fields, handling quotes and commas inside quotes.
2. **Schema**: `{ hostname: "string", ram_gb: "number", active: "boolean" }`.
3. **Types**: a `Row<S>` type that turns that schema into `{ hostname: string; ram_gb: number; active: boolean }`.
4. **Validate**: convert each field, collect errors with row and column.
5. **Use it**: import assets.csv and print a summary.

## You're done when

- Good rows come out typed (your editor shows the right types) and every bad row has a clear error.

## Getting help

Open the project, then the 🤖 **AI Helper**: it sees the lesson and your files. Good questions to start with:

- *"Explain how a mapped type can turn "number" into number."*
- *"Is my quote handling correct for a field like "Tokyo, Japan"?"*

Ask for hints, reviews and explanations rather than finished code: you learn by writing it.
@@project csv_typed Type-safe CSV Importer
Create the folder, open it in VS Code, and work through `README.md` one milestone at a time.
@@file README.md
# Type-safe CSV Importer

Real data is messy. Write a generic CSV importer: describe the columns once (name, type, required), and get back fully typed records plus a list of row-level errors ("row 7: 'ram_gb' is not a number"), using TypeScript generics and mapped types so the result type follows from the description.

## Milestones

- [ ] 1. Parser: split lines into fields, handling quotes and commas inside quotes.
- [ ] 2. Schema: `{ hostname: "string", ram_gb: "number", active: "boolean" }`.
- [ ] 3. Types: a `Row<S>` type that turns that schema into `{ hostname: string; ram_gb: number; active: boolean }`.
- [ ] 4. Validate: convert each field, collect errors with row and column.
- [ ] 5. Use it: import assets.csv and print a summary.

## Done when

- [ ] Good rows come out typed (your editor shows the right types) and every bad row has a clear error.
@@file importer.ts
// A typed CSV importer. Run: node importer.ts
type Kind = "string" | "number" | "boolean";
type Schema = Record<string, Kind>;
type TypeOf<K extends Kind> = K extends "number" ? number : K extends "boolean" ? boolean : string;
export type Row<S extends Schema> = { [C in keyof S]: TypeOf<S[C]> };

export function parseLine(line: string): string[] {
  // TODO: handle quoted fields
  return line.split(",");
}

export function importCsv<S extends Schema>(text: string, schema: S): { rows: Row<S>[]; errors: string[] } {
  // TODO: header, convert each field by its kind, collect errors
  return { rows: [], errors: [] };
}

const schema = { hostname: "string", ram_gb: "number", active: "boolean" } as const;
const result = importCsv("hostname,ram_gb,active\npc-01,16,true\npc-02,lots,false", schema);
console.log(result);
