@@lesson h1-tables Data Tables Done Properly: Structure, Headers, Spans and Sorting
@@text
Tables have a bad reputation from the years when whole page layouts were built out of them. Used for what they're for, **data with rows and columns**, they're the most accessible and most useful structure HTML has: screen readers announce the headers for each cell, browsers copy them cleanly into spreadsheets, and CSS can make them both readable and responsive. This lesson covers the markup that makes a table work for everyone, then styling, responsiveness and sorting.

## When to use a table

Use a table when each cell's meaning depends on **both** its row and its column: a timetable, a price comparison, server metrics, an exam result list. Don't use a table to place things side by side; that's what flex and grid are for. A quick test: if it would make sense to paste it into a spreadsheet, it's a table.

## The structure

```html,norun
<table>
  <caption>Server load, 6 November (percent)</caption>
  <thead>
    <tr>
      <th scope="col">Server</th>
      <th scope="col">CPU</th>
      <th scope="col">Memory</th>
    </tr>
  </thead>
  <tbody>
    <tr>
      <th scope="row">web01</th>
      <td>42</td>
      <td>61</td>
    </tr>
    <tr>
      <th scope="row">db01</th>
      <td>91</td>
      <td>78</td>
    </tr>
  </tbody>
  <tfoot>
    <tr>
      <th scope="row">Average</th>
      <td>66.5</td>
      <td>69.5</td>
    </tr>
  </tfoot>
</table>
```

- **`<caption>`** is the table's title. It must be the first child, and screen readers announce it when the user reaches the table, so they know what it's about before hearing hundreds of numbers. It's better than a heading above the table because it's programmatically tied to it.
- **`<thead>`, `<tbody>`, `<tfoot>`** group the rows. They help assistive technology, let you style groups, and when printing a long table the browser repeats the `thead` on every page. You can have several `<tbody>` elements to group rows (one per department, say).
- **`<th>`** is a header cell, **`<td>`** a data cell. The first cell of each row is often a header too: "web01" labels its row.
- **`scope="col"`** and **`scope="row"`** say which cells a header applies to. For simple tables browsers can often guess, but stating it removes the guesswork. For a header that spans a group, use `scope="colgroup"` or `scope="rowgroup"`.

## Spanning cells

`colspan` and `rowspan` merge cells:

```html,norun
<tr>
  <th scope="col" rowspan="2">Region</th>
  <th scope="colgroup" colspan="2">2025</th>
  <th scope="colgroup" colspan="2">2026</th>
</tr>
<tr>
  <th scope="col">H1</th><th scope="col">H2</th>
  <th scope="col">H1</th><th scope="col">H2</th>
</tr>
```

Every row must still add up to the same number of columns once spans are counted; a missing cell shifts everything after it, a bug that's easy to miss visually. Keep spans for real groupings. When a table needs many levels of headers, it's often clearer to split it into two tables.

For truly irregular tables, where `scope` can't express the relationships, give header cells an `id` and list them on each data cell with `headers="id1 id2"`. It's tedious, which is another hint that the table could be simpler.

## Column groups

`<colgroup>` and `<col>` describe columns, mainly for styling a whole column at once:

```html,norun
<colgroup>
  <col>
  <col span="2" class="numbers">
</colgroup>
```

Only a few properties work on `<col>`: `background`, `border`, `width` and `visibility: collapse`. To align text in a column, use a selector like `td:nth-child(2)` instead.

## Styling tables

A few rules make most tables look professional:

```css
table { border-collapse: collapse; width: 100%; }
caption { text-align: start; font-weight: 600; padding-block: 0.5rem; }
th, td { padding: 0.5rem 0.75rem; text-align: start; border-bottom: 1px solid #e2e8f0; }
thead th { background: #f8fafc; }
td.num, th.num { text-align: end; font-variant-numeric: tabular-nums; }
tbody tr:nth-child(even) { background: #f8fafc; }
tbody tr:hover { background: #eef2ff; }
```

- **`border-collapse: collapse`** merges the double borders between cells.
- **Right-align numbers** and use **`font-variant-numeric: tabular-nums`**, so digits have equal widths and columns of figures line up by place value.
- **Horizontal lines only** usually look cleaner than a full grid.
- **Zebra stripes** help the eye follow long rows; a hover highlight helps too.
- Keep text left-aligned (start-aligned) and give headers the same alignment as their data.

## Sticky headers and wide tables

For long tables, keep the header visible while scrolling:

```css
thead th { position: sticky; top: 0; z-index: 1; background: #f8fafc; }
```

`position: sticky` works on `th` cells. The header sticks to the nearest scrolling ancestor, so if the table sits in a scroll container, it sticks to the top of that container.

Wide tables are the hard part on phones. The simplest reliable fix is to let the table scroll **inside** a wrapper, so the page itself doesn't scroll sideways:

```html,norun
<div class="table-wrap" role="region" aria-labelledby="cap1" tabindex="0">
  <table>
    <caption id="cap1">Server load, 6 November</caption>
    ...
  </table>
</div>
```

```css
.table-wrap { overflow-x: auto; }
```

The `role="region"`, the accessible name and `tabindex="0"` make the scroll area reachable and scrollable by keyboard. You can also make the first column sticky (`position: sticky; left: 0` on `th[scope="row"]`), so row labels stay visible while scrolling sideways. Turning rows into stacked "cards" with CSS on small screens is possible, but it changes how screen readers see the table unless you're careful, so prefer the scroll wrapper.

## Sortable tables

When users can sort by a column, make the header a **button** and report the state with **`aria-sort`** on the `th`:

```html,norun
<th scope="col" aria-sort="ascending">
  <button type="button" data-key="cpu">CPU <span aria-hidden="true">▲</span></button>
</th>
```

- `aria-sort` takes `ascending`, `descending` or `none`, and it should be on **one column at a time** (remove it from the others).
- The button makes the header focusable and operable with Enter or Space, which a clickable `th` alone wouldn't be.
- Sort the data, not the DOM text: keep the rows in a JavaScript array, sort it with a comparator (numbers as numbers, text with `localeCompare`), and re-render the `tbody`.
- `Intl.Collator` with `{ numeric: true }` sorts strings such as "web2" before "web10" the way people expect.

## Checklist

1. Is it really tabular data?
2. Does it have a `<caption>`?
3. Are header cells `<th>` with the right `scope`?
4. Do all rows add up to the same number of columns?
5. Are numbers right-aligned with tabular figures?
6. Does it work at 320 px wide without the page scrolling sideways?
7. If it sorts, do headers use buttons and `aria-sort`?

@@quiz
Why use `<caption>` rather than a heading above the table?
* The caption is programmatically tied to the table and announced when a screen reader reaches it
- Captions are larger by default
- Headings aren't allowed near tables
- Only captions can be styled
? A heading is just nearby text; the caption is the table's name.

@@quiz
What does `scope="row"` on a `<th>` mean?
* The header labels the other cells in its row
- The header spans the whole row
- The row can be sorted
- The cell is hidden from screen readers
? Use scope="col" for column headers and scope="row" for row headers.

@@quiz
Which CSS makes columns of numbers line up by place value?
* text-align: end with font-variant-numeric: tabular-nums
- text-align: center
- letter-spacing: 1px
- font-family: monospace on the whole table
? Tabular figures give every digit the same width.

@@quiz
What is the most reliable way to keep a wide table usable on a phone?
* Put it in a horizontally scrollable wrapper with an accessible name and tabindex="0"
- Set the font size to 6px
- Use display: block on every cell
- Hide the extra columns with display: none
? The page stays put and only the table scrolls.

@@quiz
How should a sortable column header expose its state?
* A button inside the th, with aria-sort on the th
- An onclick handler on the th only
- A title attribute saying "sortable"
- aria-sort on every column at once
? aria-sort belongs on the currently sorted column only.

@@flashcards
<caption> :: the table's title; first child of <table>
<thead> / <tbody> / <tfoot> :: row groups; thead repeats when printing
scope="col" / scope="row" :: which cells a header labels
scope="colgroup" :: a header over a spanned group of columns
colspan / rowspan :: merge cells across columns / rows
headers="a b" :: link a data cell to header ids in irregular tables
border-collapse: collapse :: single borders between cells
tabular-nums :: equal-width digits for aligned numbers
position: sticky; top: 0 on th :: header stays visible while scrolling
aria-sort :: ascending, descending or none on the sorted header
Intl.Collator({ numeric: true }) :: sorts "web2" before "web10"

@@drill
Which attribute value marks a header that applies to the cells below it?
= scope="col"
= col
= scope=col

@@drill
Which element gives a table its accessible title?
= caption
= <caption>

@@exercise An accessible exam timetable
@@concepts strings
Turn the div soup into a real table: a `<caption>` "Exam week timetable", a `<thead>` whose first header spans two rows (`rowspan="2"`) for "Day" and a grouped header "Morning" over two columns (`colspan="2"`, `scope="colgroup"`), the room and time headers with `scope="col"`, row headers with `scope="row"` for each day, `border-collapse: collapse`, and the time columns right-aligned with `tabular-nums`.
@@starter
<!doctype html>
<html lang="en">
<head><meta charset="utf-8"><title>Timetable</title>
<style>
  .row { display: flex; gap: 2rem; }
</style>
</head>
<body>
  <div class="row"><b>Day</b><b>Morning room</b><b>Morning time</b></div>
  <div class="row"><b>Monday</b><span>A-101</span><span>09:30</span></div>
  <div class="row"><b>Tuesday</b><span>B-204</span><span>10:00</span></div>
  <div class="row"><b>Wednesday</b><span>A-101</span><span>09:00</span></div>
</body>
</html>
@@requires
<caption>Exam week timetable</caption>
rowspan="2"
colspan="2"
scope="colgroup"
scope="col"
scope="row"
border-collapse: collapse
tabular-nums
@@hint
The head has two rows: the first holds `<th scope="col" rowspan="2">Day</th>` and `<th scope="colgroup" colspan="2">Morning</th>`; the second holds `<th scope="col">Room</th><th scope="col">Time</th>`. Each body row starts with `<th scope="row">Monday</th>`.
@@solution
<!doctype html>
<html lang="en">
<head><meta charset="utf-8"><title>Timetable</title>
<style>
  table { border-collapse: collapse; font-family: system-ui, sans-serif; }
  caption { text-align: start; font-weight: 600; padding-block: 0.5rem; }
  th, td { padding: 0.4rem 0.8rem; border-bottom: 1px solid #cbd5e1; text-align: start; }
  thead th { background: #f1f5f9; }
  .time { text-align: end; font-variant-numeric: tabular-nums; }
</style>
</head>
<body>
  <table>
    <caption>Exam week timetable</caption>
    <thead>
      <tr>
        <th scope="col" rowspan="2">Day</th>
        <th scope="colgroup" colspan="2">Morning</th>
      </tr>
      <tr>
        <th scope="col">Room</th>
        <th scope="col" class="time">Time</th>
      </tr>
    </thead>
    <tbody>
      <tr><th scope="row">Monday</th><td>A-101</td><td class="time">09:30</td></tr>
      <tr><th scope="row">Tuesday</th><td>B-204</td><td class="time">10:00</td></tr>
      <tr><th scope="row">Wednesday</th><td>A-101</td><td class="time">09:00</td></tr>
    </tbody>
  </table>
</body>
</html>

@@exercise A sortable server table
@@concepts lists, functions
Make the table sortable. Keep the rows in a JavaScript array and render the `<tbody>` from it. Each sortable header holds a `<button type="button">`; clicking it sorts by that column (clicking again reverses the order), sets `aria-sort` to "ascending" or "descending" on that `th` and removes it from the others. Sort numbers numerically and names with an `Intl.Collator` using `numeric: true`, so "web2" comes before "web10". Wrap the table in a scrollable region with `overflow-x: auto`.
@@starter
<!doctype html>
<html lang="en">
<head><meta charset="utf-8"><title>Servers</title></head>
<body>
  <table>
    <caption>Servers</caption>
    <thead><tr><th scope="col">Name</th><th scope="col">CPU %</th></tr></thead>
    <tbody></tbody>
  </table>
  <script>
    const rows = [
      { name: "web10", cpu: 35 }, { name: "web2", cpu: 81 }, { name: "db1", cpu: 64 }, { name: "web1", cpu: 12 },
    ];
    // Render rows and add sorting here
  </script>
</body>
</html>
@@requires
<button type="button"
aria-sort
removeAttribute("aria-sort")
new Intl.Collator(undefined, { numeric: true })
overflow-x: auto
@@hint
Store `let sortKey = "name", dir = 1`. In the click handler, flip `dir` if the same key was clicked, otherwise set the key and `dir = 1`. Sort a copy: `[...rows].sort((a, b) => dir * (key === "name" ? collator.compare(a.name, b.name) : a.cpu - b.cpu))`. Then loop over all `th` elements, `removeAttribute("aria-sort")`, and set it on the clicked one.
@@solution
<!doctype html>
<html lang="en">
<head><meta charset="utf-8"><title>Servers</title>
<style>
  .table-wrap { overflow-x: auto; }
  table { border-collapse: collapse; min-width: 20rem; }
  th, td { padding: 0.4rem 0.8rem; border-bottom: 1px solid #cbd5e1; text-align: start; }
  th button { font: inherit; font-weight: 600; background: none; border: 0; padding: 0; cursor: pointer; }
  .num { text-align: end; font-variant-numeric: tabular-nums; }
</style>
</head>
<body>
  <div class="table-wrap" role="region" aria-labelledby="cap" tabindex="0">
    <table>
      <caption id="cap">Servers</caption>
      <thead>
        <tr>
          <th scope="col" data-key="name"><button type="button">Name</button></th>
          <th scope="col" data-key="cpu" class="num"><button type="button">CPU %</button></th>
        </tr>
      </thead>
      <tbody></tbody>
    </table>
  </div>
  <script>
    const rows = [
      { name: "web10", cpu: 35 }, { name: "web2", cpu: 81 }, { name: "db1", cpu: 64 }, { name: "web1", cpu: 12 },
    ];
    const collator = new Intl.Collator(undefined, { numeric: true });
    const tbody = document.querySelector("tbody");
    const headers = document.querySelectorAll("th[data-key]");
    let sortKey = null;
    let dir = 1;

    function render(list) {
      tbody.replaceChildren(...list.map((r) => {
        const tr = document.createElement("tr");
        const name = document.createElement("th");
        name.scope = "row";
        name.textContent = r.name;
        const cpu = document.createElement("td");
        cpu.className = "num";
        cpu.textContent = r.cpu;
        tr.append(name, cpu);
        return tr;
      }));
    }

    function sortBy(key) {
      dir = key === sortKey ? -dir : 1;
      sortKey = key;
      const sorted = [...rows].sort((a, b) =>
        dir * (key === "name" ? collator.compare(a.name, b.name) : a.cpu - b.cpu));
      headers.forEach((th) => th.removeAttribute("aria-sort"));
      const th = document.querySelector(`th[data-key="${key}"]`);
      th.setAttribute("aria-sort", dir === 1 ? "ascending" : "descending");
      render(sorted);
    }

    headers.forEach((th) => th.querySelector("button").addEventListener("click", () => sortBy(th.dataset.key)));
    render(rows);
  </script>
</body>
</html>
