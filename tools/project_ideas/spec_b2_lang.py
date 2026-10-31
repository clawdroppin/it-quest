from gen_projects import P

CSPROJ = '''<Project Sdk="Microsoft.NET.Sdk">
  <PropertyGroup>
    <OutputType>Exe</OutputType>
    <TargetFramework>net8.0</TargetFramework>
    <ImplicitUsings>enable</ImplicitUsings>
    <Nullable>enable</Nullable>
    <InvariantGlobalization>true</InvariantGlobalization>
    <RollForward>Major</RollForward>
  </PropertyGroup>
</Project>'''

PROJECTS = [
    P("javascript", "Advanced", "uptime_cli", "Website Uptime Checker CLI", "Check a list of URLs in parallel with fetch and report response times.",
      """A Node.js command-line tool that reads sites.json, requests every URL at the same time with `fetch` and a timeout, and prints status, response time and TLS certificate days left. Run it every few minutes and it appends to a history file you can chart.""",
      ["async/await", "Promise.allSettled", "AbortController timeouts", "node:fs", "node:test"],
      ["Read sites.json and request all URLs with Promise.allSettled.",
       "Timeout: abort any request slower than 5 seconds.",
       "Report: an aligned table with status, milliseconds and a ✔ or ✘.",
       "History: append results to history.jsonl and show each site's uptime percentage.",
       "Tests: test the formatting and uptime functions with node:test."],
      ["`node check.js` reports every site within 5 seconds and `node --test` passes."],
      ["What's the difference between Promise.all and Promise.allSettled here?",
       "How do I add a timeout to fetch?"],
      {"check.js": '''// Check websites in parallel: node check.js
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
''', "sites.json": '["https://example.com", "https://www.iana.org", "https://httpbin.org/status/503"]', "package.json": '{ "type": "module" }'}),
    P("typescript", "Intermediate", "csv_typed", "Type-safe CSV Importer", "Parse CSV into typed records with validation errors per row.",
      """Real data is messy. Write a generic CSV importer: describe the columns once (name, type, required), and get back fully typed records plus a list of row-level errors ("row 7: 'ram_gb' is not a number"), using TypeScript generics and mapped types so the result type follows from the description.""",
      ["generics", "mapped and conditional types", "parsing quoted CSV", "error reporting"],
      ["Parser: split lines into fields, handling quotes and commas inside quotes.",
       "Schema: `{ hostname: \"string\", ram_gb: \"number\", active: \"boolean\" }`.",
       "Types: a `Row<S>` type that turns that schema into `{ hostname: string; ram_gb: number; active: boolean }`.",
       "Validate: convert each field, collect errors with row and column.",
       "Use it: import assets.csv and print a summary."],
      ["Good rows come out typed (your editor shows the right types) and every bad row has a clear error."],
      ["Explain how a mapped type can turn \"number\" into number.",
       "Is my quote handling correct for a field like \"Tokyo, Japan\"?"],
      {"importer.ts": '''// A typed CSV importer. Run: node importer.ts
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
const result = importCsv("hostname,ram_gb,active\\npc-01,16,true\\npc-02,lots,false", schema);
console.log(result);
'''}),
    P("html", "Advanced", "it_handbook", "Searchable IT Handbook", "A multi-page help site for staff, with a search box and print styles.",
      """Write the IT handbook every company needs: how to connect to Wi-Fi and VPN, set up MFA, report a phishing email, request software. Several linked pages, a shared stylesheet, a table of contents, a small JavaScript search over the page titles and headings, and a print stylesheet so people can print a page.""",
      ["multi-page structure", "navigation and breadcrumbs", "shared CSS", "accessible forms", "print CSS"],
      ["Pages: index plus at least four how-to pages with numbered steps and screenshots placeholders.",
       "Navigation: the same header and nav on every page, with the current page marked.",
       "Search: an input that filters a list of all headings as you type.",
       "Print: hide the nav and show link URLs when printed.",
       "Check: test keyboard navigation and run Lighthouse's accessibility audit."],
      ["A colleague can find and follow the VPN guide without asking you."],
      ["How do I mark the current page in the navigation accessibly?",
       "Help me write clear step-by-step instructions for non-technical staff."],
      {"index.html": '''<!doctype html>
<html lang="en">
<head>
  <meta charset="utf-8">
  <meta name="viewport" content="width=device-width, initial-scale=1">
  <title>IT Handbook</title>
  <link rel="stylesheet" href="style.css">
</head>
<body>
  <header><a href="index.html">IT Handbook</a></header>
  <nav aria-label="Guides">
    <ul>
      <li><a href="vpn.html">Connect to the VPN</a></li>
      <!-- TODO: more guides -->
    </ul>
  </nav>
  <main>
    <h1>How can we help?</h1>
    <label for="q">Search the guides</label>
    <input id="q" type="search">
  </main>
</body>
</html>
''', "vpn.html": '''<!doctype html>
<html lang="en">
<head>
  <meta charset="utf-8">
  <title>Connect to the VPN · IT Handbook</title>
  <link rel="stylesheet" href="style.css">
</head>
<body>
  <main>
    <h1>Connect to the VPN</h1>
    <ol>
      <li>Open the VPN app from the Start menu.</li>
      <!-- TODO -->
    </ol>
  </main>
</body>
</html>
''', "style.css": '''body { font-family: system-ui, sans-serif; max-width: 70ch; margin: 0 auto; padding: 1rem; line-height: 1.6; }
@media print { nav, header { display: none; } a::after { content: " (" attr(href) ")"; } }
'''}),
    P("go", "Intermediate", "url_shortener", "URL Shortener Service", "A small HTTP service with JSON, a map behind a mutex, and tests.",
      """Build a link shortener: `POST /shorten` with a URL returns a short code, `GET /{code}` redirects, `GET /stats/{code}` shows the click count. Store links in a map protected by a mutex, save them to a JSON file, and test the handlers with `httptest`.""",
      ["net/http and the 1.22 router patterns", "encoding/json", "sync.Mutex", "httptest"],
      ["Store: a struct with a map and a mutex, and Add/Get/Hit methods.",
       "Handlers: shorten, redirect (302) and stats, with JSON errors.",
       "Codes: 6 random characters from crypto/rand, retry on collision.",
       "Persistence: load and save links.json.",
       "Tests: httptest.NewRecorder for each handler, including bad input."],
      ["`go test ./...` passes and curl can shorten and follow a link."],
      ["How do the new path patterns like GET /{code} work in Go 1.22?",
       "Is my mutex usage free of data races? How do I check with -race?"],
      {"go.mod": "module shortener\n\ngo 1.22", "main.go": '''package main

import (
	"log"
	"net/http"
	"sync"
)

type Store struct {
	mu    sync.Mutex
	links map[string]string
	hits  map[string]int
}

func main() {
	s := &Store{links: map[string]string{}, hits: map[string]int{}}
	mux := http.NewServeMux()
	mux.HandleFunc("GET /{code}", func(w http.ResponseWriter, r *http.Request) {
		s.mu.Lock()
		url, ok := s.links[r.PathValue("code")]
		s.mu.Unlock()
		if !ok {
			http.NotFound(w, r)
			return
		}
		http.Redirect(w, r, url, http.StatusFound)
	})
	// TODO: POST /shorten and GET /stats/{code}
	log.Fatal(http.ListenAndServe(":8080", mux))
}
'''}),
    P("csharp", "Advanced", "event_log_parser", "Log Analyzer with LINQ", "Parse application logs and answer questions with LINQ queries.",
      """Load a large application log into records, then answer ops questions with LINQ: errors per hour, the top 10 slowest requests, which users hit the most errors, and the first error after each deployment. Then make it fast on a 1-million-line file.""",
      ["records", "LINQ (GroupBy, OrderBy, Take)", "DateTime parsing", "streaming with File.ReadLines", "Stopwatch"],
      ["Model: a `record LogEntry(DateTime Time, string Level, string User, int Ms, string Message)`.",
       "Parse: TryParse each line, count bad lines instead of crashing.",
       "Queries: the four questions, each as one LINQ expression.",
       "Scale: generate 1,000,000 lines and measure; switch to File.ReadLines and compare memory.",
       "Output: a short report and a CSV of errors per hour."],
      ["All four answers are correct on the sample and the big file is processed in a few seconds."],
      ["Show me how GroupBy works with a time bucket like the hour.",
       "Why does File.ReadLines use less memory than File.ReadAllLines?"],
      {"LogAnalyzer.csproj": CSPROJ, "Program.cs": '''using System.Globalization;

var lines = File.ReadLines(args.Length > 0 ? args[0] : "app.log");
var entries = new List<LogEntry>();
var bad = 0;
foreach (var line in lines)
{
    var parts = line.Split(' ', 5);
    if (parts.Length == 5 && DateTime.TryParse(parts[0], CultureInfo.InvariantCulture, DateTimeStyles.None, out var time)
        && int.TryParse(parts[3].TrimEnd('m', 's'), out var ms))
        entries.Add(new LogEntry(time, parts[1], parts[2], ms, parts[4]));
    else
        bad++;
}
Console.WriteLine($"{entries.Count} entries, {bad} bad lines");
// TODO: errors per hour, slowest requests, users with most errors, first error after each deploy

record LogEntry(DateTime Time, string Level, string User, int Ms, string Message);
''', "app.log": '''2026-10-30T09:00:01 INFO ann 120ms GET /orders
2026-10-30T09:00:03 ERROR ken 1430ms POST /orders timeout from db
2026-10-30T09:15:10 INFO deploy 0ms deployed version 2.4.1
2026-10-30T09:15:42 ERROR ann 310ms GET /cart null reference
2026-10-30T10:02:00 WARN yui 980ms GET /search slow query'''}),
    P("cpp", "Intermediate", "packet_parser", "Network Packet Header Parser", "Decode Ethernet, IPv4 and TCP headers from raw bytes.",
      """Read a captured packet as bytes and decode its headers by hand: Ethernet (MACs, EtherType), IPv4 (version, header length, TTL, protocol, addresses, checksum) and TCP (ports, flags). You'll learn byte order, bit masks and safe bounds checking: the foundations of every network tool.""",
      ["std::vector<uint8_t> and std::span", "big-endian byte order", "bit masks and shifts", "bounds checking", "std::format or iostreams"],
      ["Input: the sample packet as a hex string in packet.txt; convert to bytes.",
       "Ethernet: destination and source MAC, EtherType 0x0800.",
       "IPv4: IHL × 4 = header length; protocol 6 = TCP; dotted addresses; verify the checksum.",
       "TCP: ports and flags (SYN, ACK, FIN, RST, PSH) from the flags byte.",
       "Safety: reject truncated packets instead of reading past the end."],
      ["The sample decodes to the values in the comments and a truncated packet is rejected cleanly."],
      ["How do I read a 16-bit big-endian number from two bytes?",
       "Explain the IPv4 header checksum algorithm."],
      {"main.cpp": '''// Decode Ethernet / IPv4 / TCP headers from a hex dump.
#include <cstdint>
#include <fstream>
#include <iostream>
#include <string>
#include <vector>

std::vector<std::uint8_t> fromHex(const std::string& hex) {
    std::vector<std::uint8_t> out;
    for (std::size_t i = 0; i + 1 < hex.size(); i += 2) out.push_back(static_cast<std::uint8_t>(std::stoi(hex.substr(i, 2), nullptr, 16)));
    return out;
}

std::uint16_t be16(const std::vector<std::uint8_t>& b, std::size_t at) { return static_cast<std::uint16_t>(b.at(at) << 8 | b.at(at + 1)); }

int main() {
    std::ifstream in("packet.txt");
    std::string hex;
    in >> hex;
    auto bytes = fromHex(hex);
    std::cout << bytes.size() << " bytes, EtherType 0x" << std::hex << be16(bytes, 12) << "\\n";
    // TODO: MAC addresses, IPv4 header, TCP ports and flags, checksum
}
''', "packet.txt": "00155d0a0b0c3c22fb112233080045000034a6f240004006b1e6c0a80164c0a8010ad3a401bbe1c4b0a700000000802272102c8d0000020405b40103030801010402"}),
    P("sql", "Advanced", "sales_analytics", "Sales Analytics with Window Functions", "Answer business questions with CTEs and window functions.",
      """Load a year of shop orders and answer what a business analyst would ask: monthly revenue and growth, running totals, each customer's first and latest order, top 3 products per category, customers who haven't ordered in 90 days, and a cohort table of retention by first-order month.""",
      ["CTEs", "window functions (ROW_NUMBER, LAG, SUM OVER)", "date functions", "cohort analysis"],
      ["Schema and data: orders and products tables (generate a year of data with a script or by hand).",
       "Growth: monthly revenue with LAG for the change from last month.",
       "Ranking: top 3 products per category with ROW_NUMBER.",
       "Churn: customers inactive for 90 days.",
       "Cohorts: first-order month vs months since, counting active customers."],
      ["Every query runs, and you can explain each result in one sentence a manager would understand."],
      ["Explain PARTITION BY versus GROUP BY with my data.",
       "How do I build a cohort table in SQLite?"],
      {"schema.sql": '''CREATE TABLE product (id INTEGER PRIMARY KEY, name TEXT NOT NULL, category TEXT NOT NULL, price INTEGER NOT NULL);
CREATE TABLE orders (id INTEGER PRIMARY KEY, customer TEXT NOT NULL, product_id INTEGER NOT NULL REFERENCES product(id),
                     qty INTEGER NOT NULL, ordered_on TEXT NOT NULL);
''', "queries.sql": '''-- Monthly revenue and change from the previous month
WITH monthly AS (
  SELECT substr(o.ordered_on, 1, 7) AS month, SUM(o.qty * p.price) AS revenue
  FROM orders o JOIN product p ON p.id = o.product_id
  GROUP BY month
)
SELECT month, revenue, revenue - LAG(revenue) OVER (ORDER BY month) AS change
FROM monthly;

-- TODO: top 3 per category, inactive customers, cohorts
'''}),
]
