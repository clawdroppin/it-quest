@@lesson xp-uptime_cli Website Uptime Checker CLI
@@text
A Node.js command-line tool that reads sites.json, requests every URL at the same time with `fetch` and a timeout, and prints status, response time and TLS certificate days left. Run it every few minutes and it appends to a history file you can chart.

**You'll practise:** async/await · Promise.allSettled · AbortController timeouts · node:fs · node:test

## Milestones

1. Read sites.json and request all URLs with Promise.allSettled.
2. **Timeout**: abort any request slower than 5 seconds.
3. **Report**: an aligned table with status, milliseconds and a ✔ or ✘.
4. **History**: append results to history.jsonl and show each site's uptime percentage.
5. **Tests**: test the formatting and uptime functions with node:test.

## You're done when

- `node check.js` reports every site within 5 seconds and `node --test` passes.

## Getting help

Open the project, then the 🤖 **AI Helper**: it sees the lesson and your files. Good questions to start with:

- *"What's the difference between Promise.all and Promise.allSettled here?"*
- *"How do I add a timeout to fetch?"*

Ask for hints, reviews and explanations rather than finished code: you learn by writing it.
@@project uptime_cli Website Uptime Checker CLI
Create the folder, open it in VS Code, and work through `README.md` one milestone at a time.
@@file README.md
# Website Uptime Checker CLI

A Node.js command-line tool that reads sites.json, requests every URL at the same time with `fetch` and a timeout, and prints status, response time and TLS certificate days left. Run it every few minutes and it appends to a history file you can chart.

## Milestones

- [ ] 1. Read sites.json and request all URLs with Promise.allSettled.
- [ ] 2. Timeout: abort any request slower than 5 seconds.
- [ ] 3. Report: an aligned table with status, milliseconds and a ✔ or ✘.
- [ ] 4. History: append results to history.jsonl and show each site's uptime percentage.
- [ ] 5. Tests: test the formatting and uptime functions with node:test.

## Done when

- [ ] `node check.js` reports every site within 5 seconds and `node --test` passes.
@@file check.js
// Check websites in parallel: node check.js
import { readFile } from "node:fs/promises";

const sites = JSON.parse(await readFile(new URL("./sites.json", import.meta.url), "utf8"));

async function check(url) {
  const start = performance.now();
  // TODO: AbortController with a 5 s timeout
  const res = await fetch(url);
  return { url, status: res.status, ms: Math.round(performance.now() - start) };
}

const results = await Promise.allSettled(sites.map(check));
for (const r of results) console.log(r.status === "fulfilled" ? r.value : r.reason.message);
@@file sites.json
["https://example.com", "https://www.iana.org", "https://httpbin.org/status/503"]
@@file package.json
{ "type": "module" }
