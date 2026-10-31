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
    P("go", "Advanced", "log_tail", "Multi-file Log Follower", "tail -f for many files at once, with filters and colours.",
      """Follow several log files at the same time, like `tail -f`, prefixing each line with its file name in its own colour, and showing only lines that match a regular expression. One goroutine per file, one channel for all lines.""",
      ["goroutines and channels", "bufio.Scanner", "regexp", "os.Signal for clean shutdown", "flag package"],
      ["Flags: `-grep REGEX` and a list of files.",
       "Follow: a goroutine per file that seeks to the end and polls for new lines.",
       "Merge: all goroutines send `Line{File, Text}` to one channel; main prints them.",
       "Colour: pick a colour per file from a small palette.",
       "Shutdown: Ctrl+C stops every goroutine cleanly with a context."],
      ["Appending to two files in another terminal shows their lines live, filtered and coloured."],
      ["How do I detect that a log file was rotated (truncated or replaced)?",
       "Is my use of context for cancellation correct?"],
      {"go.mod": "module logtail\n\ngo 1.22", "main.go": '''// logtail follows several files at once: go run . -grep ERROR app.log web.log
package main

import (
	"flag"
	"fmt"
	"os"
)

type Line struct {
	File string
	Text string
}

func main() {
	grep := flag.String("grep", "", "only show lines matching this regular expression")
	flag.Parse()
	if flag.NArg() == 0 {
		fmt.Fprintln(os.Stderr, "usage: logtail [-grep REGEX] FILE...")
		os.Exit(2)
	}
	lines := make(chan Line)
	// TODO: start a follower goroutine per file that sends to lines
	_ = grep
	for l := range lines {
		fmt.Printf("%s: %s\\n", l.File, l.Text)
	}
}
'''}),
    P("csharp", "Intermediate", "password_tool", "Password Generator and Strength Meter", "Generate strong passwords and passphrases, and rate existing ones.",
      """A console tool for the help desk: generate random passwords or Diceware-style passphrases with a cryptographically secure generator, and rate a password's strength by estimated entropy and simple rules (length, character classes, common words).""",
      ["System.Security.Cryptography.RandomNumberGenerator", "string building", "command-line arguments", "classes and static methods"],
      ["Generate: `gen 16` makes a 16-character password from upper, lower, digits and symbols.",
       "Passphrase: `phrase 5` picks 5 words from words.txt, joined by dashes.",
       "Entropy: estimate bits as length × log2(pool size), and for phrases words × log2(word count).",
       "Rate: Weak / OK / Strong with reasons (too short, only one character class, a common password).",
       "Never use `System.Random` for passwords: explain why in a comment."],
      ["The generator only uses RandomNumberGenerator, and ratings match your expectations on a few test passwords."],
      ["Why is System.Random not safe for passwords?",
       "How do I pick a random index without modulo bias?"],
      {"PasswordTool.csproj": CSPROJ, "Program.cs": '''using System.Security.Cryptography;

// password tool: dotnet run -- gen 16 | phrase 5 | rate "P@ssw0rd"
if (args.Length == 0)
{
    Console.WriteLine("usage: gen N | phrase N | rate PASSWORD");
    return;
}

switch (args[0])
{
    case "gen":
        Console.WriteLine(Passwords.Generate(int.Parse(args[1])));
        break;
    default:
        Console.WriteLine("TODO: phrase and rate");
        break;
}

static class Passwords
{
    const string Pool = "ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz23456789!@#$%&*?";

    public static string Generate(int length)
    {
        // TODO: use RandomNumberGenerator.GetInt32(Pool.Length) for each character
        return new string('x', length);
    }
}
''', "words.txt": "apple\nriver\nstone\ncloud\nmaple\ntiger\nlantern\nsilver\norbit\nkettle\nmeadow\nharbor\npencil\nsummit\nvelvet\nwillow"}),
    P("cpp", "Advanced", "ring_logger", "Ring Buffer Logger", "A fixed-size, fast in-memory log that never allocates after start.",
      """Embedded devices and high-performance services often keep the last N log messages in a **ring buffer**: a fixed array where new entries overwrite the oldest. Write a templated `RingBuffer<T, N>` and a logger on top of it with levels and timestamps, then dump the buffer to a file when something goes wrong.""",
      ["templates with non-type parameters", "std::array", "modular arithmetic", "RAII and file streams", "std::chrono"],
      ["RingBuffer: `push`, `size`, `full`, and iteration from oldest to newest.",
       "Overwrite: pushing into a full buffer drops the oldest entry.",
       "Logger: `log(Level, std::string_view)` stores time, level and message.",
       "Dump: write all entries to a file, oldest first.",
       "Tests: a small tests.cpp that checks wrap-around with N = 3."],
      ["The tests pass, including pushing 10 items into a buffer of 3 and reading back the last 3 in order."],
      ["How do I write an iterator for my ring buffer?",
       "Why does a fixed-size buffer help in real-time systems?"],
      {"main.cpp": '''// A fixed-size ring buffer and a logger built on it.
#include <array>
#include <cstddef>
#include <iostream>
#include <string>

template <typename T, std::size_t N>
class RingBuffer {
public:
    void push(const T& value) {
        // TODO: write at head_, advance head_ modulo N, grow size_ up to N
        (void)value;
    }
    std::size_t size() const { return size_; }
    bool full() const { return size_ == N; }

private:
    std::array<T, N> items_{};
    std::size_t head_ = 0;
    std::size_t size_ = 0;
};

int main() {
    RingBuffer<std::string, 3> log;
    for (int i = 1; i <= 5; ++i) log.push("message " + std::to_string(i));
    std::cout << "stored " << log.size() << " messages\\n";  // should be 3
}
'''}),
    P("sql", "Intermediate", "helpdesk_db", "Help Desk Ticket Database", "Design a ticketing database and write the reports a manager asks for.",
      """Design the database behind a help desk: users, agents, tickets, comments and categories. Then answer the questions managers ask every week with SQL: open tickets per agent, average time to resolve per category, tickets breaching their SLA, and the busiest hour of the day.""",
      ["schema design and foreign keys", "JOIN and GROUP BY", "date functions", "window functions", "indexes"],
      ["Schema: write schema.sql with tables, primary and foreign keys, and CHECK constraints for status and priority.",
       "Data: add 30+ realistic rows in data.sql (or generate them).",
       "Reports: write each question in reports.sql and check the answers by hand.",
       "SLA: priority 1 must be resolved in 4 hours, 2 in 1 day, 3 in 3 days; list breaches.",
       "Speed: add indexes for the reports and look at `EXPLAIN QUERY PLAN`."],
      ["Every report in reports.sql runs and gives answers you've checked."],
      ["Review my schema: are my foreign keys and constraints right?",
       "How do I compute hours between two timestamps in SQLite?"],
      {"schema.sql": '''-- Help desk schema (SQLite). Run: sqlite3 helpdesk.db < schema.sql
PRAGMA foreign_keys = ON;

CREATE TABLE agent (
    id    INTEGER PRIMARY KEY,
    name  TEXT NOT NULL,
    team  TEXT NOT NULL
);

CREATE TABLE ticket (
    id          INTEGER PRIMARY KEY,
    title       TEXT NOT NULL,
    priority    INTEGER NOT NULL CHECK (priority BETWEEN 1 AND 3),
    status      TEXT NOT NULL CHECK (status IN ('open', 'in_progress', 'resolved')),
    agent_id    INTEGER REFERENCES agent(id),
    opened_at   TEXT NOT NULL,
    resolved_at TEXT
);

-- TODO: users, categories, comments
''', "reports.sql": '''-- 1. Open tickets per agent
SELECT a.name, COUNT(*) AS open_tickets
FROM ticket t JOIN agent a ON a.id = t.agent_id
WHERE t.status <> 'resolved'
GROUP BY a.name
ORDER BY open_tickets DESC;

-- 2. TODO: average hours to resolve, per category
-- 3. TODO: SLA breaches
-- 4. TODO: busiest hour of the day
'''}),
]
