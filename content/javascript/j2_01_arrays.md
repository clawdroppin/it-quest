@@lesson j2-arrays Arrays: map, filter, reduce, sort
@@concepts lists
@@text
```javascript
const servers = ["web01", "web02", "db01"];
servers.push("mail01");             // add at the end
console.log(servers.length, servers[0], servers.at(-1));
console.log(servers.includes("db01"), servers.indexOf("web02"));
console.log(servers.slice(1, 3));   // a copy of items 1 and 2
console.log(servers.join(", "));
```

## The big three

These methods take a function and return something new, without changing the original:

```javascript
const disks = [
  { drive: "C:", sizeGB: 476, freeGB: 38 },
  { drive: "D:", sizeGB: 931, freeGB: 610 },
  { drive: "E:", sizeGB: 119, freeGB: 4 },
];

const low = disks.filter((d) => d.freeGB < 50);               // keep some
const names = low.map((d) => d.drive);                        // transform each
const totalFree = disks.reduce((sum, d) => sum + d.freeGB, 0); // combine into one

console.log(names, totalFree);
```

- `filter(fn)`: the items where `fn` returns true.
- `map(fn)`: a new array of `fn(item)` for each item.
- `reduce(fn, start)`: carry a value through the array: `fn(accumulator, item)`.

Others you'll use: `find` (first match), `some` / `every` (any / all match), `forEach` (just do something), `flatMap`, `Array.from({ length: 3 }, (_, i) => i)`.

## Sorting: a classic trap

```javascript
const disks = [
  { drive: "C:", sizeGB: 476, freeGB: 38 },
  { drive: "D:", sizeGB: 931, freeGB: 610 },
  { drive: "E:", sizeGB: 119, freeGB: 4 },
];
console.log([10, 9, 1, 100].sort());                   // [1, 10, 100, 9]: sorted as TEXT!
console.log([10, 9, 1, 100].sort((a, b) => a - b));    // numbers, ascending
const byFree = [...disks].sort((a, b) => b.freeGB - a.freeGB);   // copy, then sort descending
console.log(byFree.map((d) => d.drive));
console.log(["web02", "Web01", "db01"].sort((a, b) => a.localeCompare(b)));
```

`sort` changes the array **in place**; copy first with `[...arr]` (or use the newer `toSorted`). The compare function returns a negative number if `a` comes first.

## Chaining

```javascript
const disks = [
  { drive: "C:", sizeGB: 476, freeGB: 38 },
  { drive: "E:", sizeGB: 119, freeGB: 4 },
];
const report = disks
  .filter((d) => d.freeGB / d.sizeGB < 0.1)
  .map((d) => `${d.drive} ${Math.round((d.freeGB / d.sizeGB) * 100)}% free`);
console.log(report);
```

@@quiz
What does `[3, 20, 100].sort()` return?
* `[100, 20, 3]`
- `[3, 20, 100]`
- `[100, 3, 20]`
- an error
? Without a compare function, sort converts to strings: "100" < "20" < "3".

@@quiz
Which method gives the **total** of a property across an array?
- `map`
- `filter`
* `reduce`
- `find`
? `reduce((sum, x) => sum + x.value, 0)`.

@@quiz
What does `["web01", "db01", "web02"].filter((s) => s.startsWith("web"))` return?
* `["web01", "web02"]`
- `true`
- `["db01"]`
- `2`
? filter keeps the items for which the function returns true.

@@flashcards
map :: new array with each item transformed
filter :: new array with only the items that pass a test
reduce :: combine all items into one value
find :: the first item that passes a test (or undefined)
some / every :: does any / does every item pass?
sort((a, b) => a - b) :: numeric ascending sort
toSorted :: sorted copy, leaves the original alone
includes :: is this value in the array?

@@exercise Online servers
Using `filter` and `map`, return the **names** of servers whose `status` is `"up"`, in `upNames(servers)`.
@@starter
function upNames(servers) {
  throw new Error("TODO: not done yet");
}

@@tests
const fleet = [
  { name: "web01", status: "up" },
  { name: "web02", status: "down" },
  { name: "db01", status: "up" },
];
check(upNames(fleet), ["web01", "db01"], "upNames(fleet)");
check(upNames([]), [], "upNames([])");
@@hint
`return servers.filter((s) => s.status === "up").map((s) => s.name);`
@@solution
function upNames(servers) {
  return servers.filter((s) => s.status === "up").map((s) => s.name);
}

@@exercise Average response time
Write `stats(times)` that returns `{ min, max, avg }` for an array of response times in ms, with `avg` rounded to a whole number. For an empty array return `null`.
@@starter
function stats(times) {
  throw new Error("TODO: not done yet");
}

@@tests
check(stats([120, 80, 300, 95]), { min: 80, max: 300, avg: 149 }, "stats([120, 80, 300, 95])");
check(stats([42]), { min: 42, max: 42, avg: 42 }, "stats([42])");
check(stats([]), null, "stats([])");
@@hint
`Math.min(...times)`, `Math.max(...times)`, and `times.reduce((a, b) => a + b, 0) / times.length` inside `Math.round`.
@@solution
function stats(times) {
  if (times.length === 0) return null;
  const sum = times.reduce((a, b) => a + b, 0);
  return { min: Math.min(...times), max: Math.max(...times), avg: Math.round(sum / times.length) };
}

@@exercise Sort the tickets
Write `sortTickets(tickets)` that returns a **new** array (don't change the input) sorted by priority (`1` first), then by `opened` time ascending (older first).
@@starter
function sortTickets(tickets) {
  throw new Error("TODO: not done yet");
}

@@tests
const queue = [
  { id: "INC3", priority: 2, opened: 300 },
  { id: "INC1", priority: 1, opened: 500 },
  { id: "INC4", priority: 3, opened: 100 },
  { id: "INC2", priority: 1, opened: 200 },
];
const before = JSON.stringify(queue);
check(sortTickets(queue).map((t) => t.id), ["INC2", "INC1", "INC3", "INC4"], "sortTickets order");
check(JSON.stringify(queue), before, "the input array is unchanged");
@@hint
`[...tickets].sort((a, b) => a.priority - b.priority || a.opened - b.opened)`. When the priorities are equal, the first difference is 0 (falsy), so `||` moves on to the second.
@@solution
function sortTickets(tickets) {
  return [...tickets].sort((a, b) => a.priority - b.priority || a.opened - b.opened);
}
