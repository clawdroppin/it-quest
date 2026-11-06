@@lesson j2-objects Objects, Maps and JSON
@@concepts maps, json
@@text
## Objects

```javascript
const server = { name: "web01", ip: "10.0.0.11", ports: [80, 443], online: true };
console.log(server.name, server["ip"]);
server.owner = "ana";            // add a property
delete server.online;            // remove one
console.log(Object.keys(server));
for (const [key, value] of Object.entries(server)) {
  console.log(`${key} = ${value}`);
}
console.log("owner" in server, server.location?.city);   // ?. stops at undefined instead of crashing
```

## Destructuring and spread

```javascript
const { name, ip, ports: [firstPort] } = { name: "db01", ip: "10.0.0.21", ports: [5432] };
console.log(name, ip, firstPort);

const defaults = { port: 22, timeout: 5, retries: 3 };
const options = { ...defaults, timeout: 10 };   // copy and override
console.log(options);
```

Destructuring in parameters makes "options objects" readable: `function connect({ host, port = 22 }) { ... }`.

## Counting with objects (and Map)

```javascript
const events = ["login", "logout", "login", "failed", "login"];
const counts = {};
for (const e of events) {
  counts[e] = (counts[e] ?? 0) + 1;
}
console.log(counts);

const byHost = new Map();       // a Map can have any key type and keeps insertion order
byHost.set("web01", 3).set("db01", 1);
console.log(byHost.get("web01"), byHost.size, [...byHost.keys()]);
```

`Set` holds unique values: `new Set(["a", "b", "a"]).size` is 2, and `[...new Set(list)]` removes duplicates.

## JSON

JSON is JavaScript Object Notation, and it's how almost every web API talks:

```javascript
const text = '{"host":"web01","checks":[{"name":"disk","ok":true},{"name":"cpu","ok":false}]}';
const data = JSON.parse(text);                   // text -> objects
const failing = data.checks.filter((c) => !c.ok).map((c) => c.name);
console.log(data.host, failing);
console.log(JSON.stringify({ host: "web01", failing }));          // objects -> text
console.log(JSON.stringify({ a: 1, b: [1, 2] }, null, 2));        // pretty-printed
```

JSON is stricter than JavaScript: keys need double quotes, no trailing commas, no comments. `JSON.parse` throws on invalid text, so wrap it in `try/catch` when the input comes from outside.

@@quiz
`const opts = { ...{ a: 1, b: 2 }, b: 3 };` What is `opts.b`?
* 3
- 2
- [2, 3]
- undefined
? Later properties override earlier ones, which is how defaults + overrides work.

@@quiz
`user.address?.city` when `user.address` is undefined gives...
* `undefined`
- a TypeError
- `null`
- `""`
? Optional chaining `?.` stops and returns undefined instead of throwing.

@@quiz
`JSON.parse('{"port": 8080}').port + 1` gives...
* `8081`
- `"80801"`
- `NaN`
- an error
? JSON numbers become JavaScript numbers; JSON.parse builds real values.

@@flashcards
Object.keys / values / entries :: Arrays of an object's keys / values / [key, value] pairs
destructuring :: const { name, ip } = server
spread :: { ...defaults, timeout: 10 } copies and overrides
?. :: Optional chaining: stops at null/undefined
?? :: Default only for null/undefined
Map :: Key-value collection with any key type, keeps insertion order
Set :: Collection of unique values
JSON.parse / JSON.stringify :: Text to objects / objects to text

@@exercise Merge settings
Write `withDefaults(options)` that returns the given options combined with the defaults `{ port: 443, timeout: 30, retries: 3 }`. Given options win. Don't change the input.
@@starter
function withDefaults(options) {
  throw new Error("TODO: not done yet");
}

@@tests
check(withDefaults({ timeout: 5 }), { port: 443, timeout: 5, retries: 3 }, "withDefaults({ timeout: 5 })");
check(withDefaults({}), { port: 443, timeout: 30, retries: 3 }, "withDefaults({})");
check(withDefaults({ port: 8443, host: "api" }), { port: 8443, timeout: 30, retries: 3, host: "api" }, "extra keys are kept");
@@hint
`return { port: 443, timeout: 30, retries: 3, ...options };`
@@solution
function withDefaults(options) {
  return { port: 443, timeout: 30, retries: 3, ...options };
}

@@exercise Group by status
Write `groupByStatus(servers)` that returns an object mapping each status to an array of server names, e.g. `{ up: ["web01", "db01"], down: ["web02"] }`.
@@starter
function groupByStatus(servers) {
  throw new Error("TODO: not done yet");
}

@@tests
check(
  groupByStatus([
    { name: "web01", status: "up" },
    { name: "web02", status: "down" },
    { name: "db01", status: "up" },
    { name: "mail01", status: "maintenance" },
  ]),
  { up: ["web01", "db01"], down: ["web02"], maintenance: ["mail01"] },
  "groupByStatus",
);
check(groupByStatus([]), {}, "groupByStatus([])");
@@hint
Start with `const groups = {};`, then for each server: `(groups[s.status] ??= []).push(s.name);`. (`??=` assigns only when the value is null or undefined.)
@@solution
function groupByStatus(servers) {
  const groups = {};
  for (const s of servers) {
    (groups[s.status] ??= []).push(s.name);
  }
  return groups;
}

@@exercise Parse an API response
Write `failingChecks(jsonText)` that parses the JSON and returns the names of checks where `ok` is false. If the text isn't valid JSON, return the string `"invalid response"`.
@@starter
function failingChecks(jsonText) {
  throw new Error("TODO: not done yet");
}

@@tests
check(failingChecks('{"checks":[{"name":"disk","ok":true},{"name":"cpu","ok":false},{"name":"ram","ok":false}]}'), ["cpu", "ram"], "two failing checks");
check(failingChecks('{"checks":[]}'), [], "no checks");
check(failingChecks("<html>502 Bad Gateway</html>"), "invalid response", "not JSON");
@@hint
`try { const data = JSON.parse(jsonText); return data.checks.filter(...).map(...); } catch { return "invalid response"; }`
@@solution
function failingChecks(jsonText) {
  try {
    const data = JSON.parse(jsonText);
    return data.checks.filter((c) => !c.ok).map((c) => c.name);
  } catch {
    return "invalid response";
  }
}
