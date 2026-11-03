@@lesson j2-collections Map, Set, WeakMap and Grouping
@@concepts maps, lists
@@text
Plain objects work as dictionaries, but they have traps: keys are always strings, they inherit keys like `toString`, and their size isn't tracked. ES2015 added real collection types.

## Map

```javascript
const owners = new Map();
owners.set("web01", "Ana").set("db01", "Kenji");
owners.set(42, "a number key works too");
console.log(owners.get("web01"), owners.has("mail01"), owners.size);
owners.delete(42);
for (const [host, owner] of owners) console.log(host, "→", owner);
console.log([...owners.keys()], Object.fromEntries(owners));
```

| | Object | Map |
|---|---|---|
| Key types | strings and symbols | **anything** (objects, numbers) |
| Order | mostly insertion, but integer-like keys first | always insertion order |
| Size | `Object.keys(o).length` | `m.size` |
| Inherited keys | yes (`"constructor" in {}` is true) | no |
| JSON | direct | convert with `Object.fromEntries` |
| Best for | fixed-shape records | dictionaries with dynamic keys, frequent adds/removes |

## Set

A `Set` holds unique values (compared like `===`, except NaN equals NaN):

```javascript
const seen = new Set(["10.0.0.1", "10.0.0.2", "10.0.0.1"]);
seen.add("10.0.0.3");
console.log(seen.size, seen.has("10.0.0.2"), [...seen]);

const a = new Set(["ssh", "http", "https"]);
const b = new Set(["https", "rdp"]);
console.log([...a].filter((x) => b.has(x)), [...a].filter((x) => !b.has(x)));
console.log(a.intersection ? [...a.intersection(b)] : "set methods need Node 22+");
```

Node.js 22+ and current browsers have built-in set operations: `union`, `intersection`, `difference`, `symmetricDifference`, `isSubsetOf`, `isSupersetOf`, `isDisjointFrom`.

Objects in a Set are compared by identity: `new Set([{a: 1}, {a: 1}]).size` is 2. To deduplicate records, use a Map keyed by an ID or a string key.

## Grouping

```javascript
const tickets = [
  { id: 1, team: "net", minutes: 30 },
  { id: 2, team: "desk", minutes: 5 },
  { id: 3, team: "net", minutes: 45 },
];
const byTeam = Object.groupBy(tickets, (t) => t.team);
console.log(Object.keys(byTeam), byTeam.net.length);
const byLength = Map.groupBy(tickets, (t) => (t.minutes > 20 ? "long" : "short"));
console.log([...byLength.keys()]);
```

`Object.groupBy` (Node 21+) returns an object; `Map.groupBy` returns a Map, useful when keys aren't strings.

## WeakMap and WeakSet

A `WeakMap` holds its keys **weakly**: if nothing else references a key object, it can be garbage-collected along with its entry. Use it to attach extra data to objects you don't own without causing memory leaks (caching per DOM element, private data per instance). Keys must be objects, and you can't iterate or count them.

```javascript
const lastChecked = new WeakMap();
const serverObj = { name: "web01" };
lastChecked.set(serverObj, Date.now());
console.log(lastChecked.has(serverObj));
```

## Counting

```javascript
const words = "error warn error info error warn".split(" ");
const counts = new Map();
for (const w of words) counts.set(w, (counts.get(w) ?? 0) + 1);
const top = [...counts].sort((x, y) => y[1] - x[1]);
console.log(top);
```

@@quiz
When is a Map better than a plain object?
* When keys are dynamic or not strings, and entries are often added and removed
- When you need JSON output directly
- For records with a fixed shape
- Never
? Maps also keep insertion order and track size.

@@quiz
`new Set([{ id: 1 }, { id: 1 }]).size` is?
- 1
* 2, because objects are compared by identity
- 0
- an error
? Deduplicate records with a Map keyed by id.

@@quiz
What's the main use of a WeakMap?
- Faster lookups than Map
* Attaching data to objects without stopping them from being garbage-collected
- Storing weak passwords
- Iterating in random order
? Keys must be objects and can't be listed.

@@flashcards
Map :: key-value collection with any key type, ordered, has size
Set :: collection of unique values
`Object.fromEntries(map)` :: Map (or entries) to plain object
`Object.groupBy(items, fn)` :: group into an object of arrays
`Map.groupBy(items, fn)` :: group into a Map of arrays
WeakMap :: object keys held weakly; not iterable
Set methods :: union, intersection, difference, isSubsetOf...

@@exercise Top talkers
Firewall log lines look like `"10.0.0.5 -> 8.8.8.8:53 1200"` (source, destination:port, bytes). Write `topTalkers(lines, n)` returning the `n` source IPs with the most total bytes as `[ip, bytes]` pairs, largest first, ties by IP ascending (string order). Skip lines that don't have exactly three space-separated parts or whose bytes aren't an integer. Also write `uniquePorts(lines)` returning the sorted array of distinct destination ports (as numbers) from valid lines.
@@starter
function topTalkers(lines, n) {
  throw new Error("TODO: not done yet");
}

function uniquePorts(lines) {
  throw new Error("TODO: not done yet");
}

@@tests
const log = [
  "10.0.0.5 -> 8.8.8.8:53 1200",
  "10.0.0.7 -> 1.1.1.1:443 5000",
  "garbage line",
  "10.0.0.5 -> 9.9.9.9:443 4000",
  "10.0.0.9 -> 1.1.1.1:443 5200",
  "10.0.0.8 -> 1.1.1.1:22 abc",
];
check(topTalkers(log, 2), [["10.0.0.5", 5200], ["10.0.0.9", 5200]], "top two, tie by ip");
check(topTalkers(log, 10).length, 3, "only valid sources");
check(uniquePorts(log), [53, 443], "ports");
@@hint
The arrow `->` makes four space-separated parts: `const parts = line.split(" ")` → `[src, "->", dest, bytes]`. Check `parts.length === 4 && /^\d+$/.test(parts[3])`. Accumulate in a `Map`, then `[...map].sort((a, b) => b[1] - a[1] || a[0].localeCompare(b[0]))`.
@@solution
function parse(lines) {
  const out = [];
  for (const line of lines) {
    const parts = line.split(" ");
    if (parts.length !== 4 || parts[1] !== "->" || !/^\d+$/.test(parts[3])) continue;
    const port = Number(parts[2].split(":")[1]);
    out.push({ src: parts[0], port, bytes: Number(parts[3]) });
  }
  return out;
}

function topTalkers(lines, n) {
  const totals = new Map();
  for (const { src, bytes } of parse(lines)) totals.set(src, (totals.get(src) ?? 0) + bytes);
  return [...totals]
    .sort((a, b) => b[1] - a[1] || (a[0] < b[0] ? -1 : a[0] > b[0] ? 1 : 0))
    .slice(0, n);
}

function uniquePorts(lines) {
  return [...new Set(parse(lines).map((r) => r.port))].sort((a, b) => a - b);
}

@@exercise Access diff
Write `diffAccess(before, after)` where each argument maps a user name to an array of group names (a plain object). Return `{ added: [...], removed: [...] }` listing changes as `"user:group"` strings, each list sorted. A user missing from one side counts as having no groups.
@@starter
function diffAccess(before, after) {
  throw new Error("TODO: not done yet");
}

@@tests
const before = { ana: ["staff", "vpn"], bo: ["staff"], cy: ["admins"] };
const after = { ana: ["staff", "admins"], bo: ["staff"], dee: ["staff"] };
check(diffAccess(before, after), { added: ["ana:admins", "dee:staff"], removed: ["ana:vpn", "cy:admins"] }, "diff");
check(diffAccess({}, {}), { added: [], removed: [] }, "empty");
check(diffAccess({ x: ["a", "a"] }, { x: ["a"] }), { added: [], removed: [] }, "duplicates ignored");
@@hint
Turn each side into a `Set` of `"user:group"` strings with `Object.entries(side).flatMap(([u, gs]) => gs.map((g) => `${u}:${g}`))`. Then filter each set by `!other.has(x)` and sort.
@@solution
function pairs(side) {
  return new Set(Object.entries(side).flatMap(([user, groups]) => groups.map((g) => `${user}:${g}`)));
}

function diffAccess(before, after) {
  const b = pairs(before);
  const a = pairs(after);
  return {
    added: [...a].filter((x) => !b.has(x)).sort(),
    removed: [...b].filter((x) => !a.has(x)).sort(),
  };
}
