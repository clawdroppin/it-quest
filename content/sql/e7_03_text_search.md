@@lesson e7-search Searching Text: LIKE, Collations and Full-Text Search
@@concepts strings, text-processing
@@text
"Find the ticket about the printer" sounds simple until you hit capital letters, accents, plurals and a million rows. This lesson goes from `LIKE` to real full-text search.

## LIKE, GLOB and case

- `LIKE` uses `%` (any text) and `_` (one character). In SQLite it's **case-insensitive for ASCII** letters only: `'É' LIKE 'é'` is false. In PostgreSQL `LIKE` is case-sensitive; use `ILIKE` for insensitive.
- `GLOB` (SQLite) uses Unix wildcards `*`, `?` and `[a-z]`, and is case-sensitive.
- To search for a literal `%` or `_`, add `ESCAPE`: `title LIKE '%100\%%' ESCAPE '\'`.

```sql
SELECT id, title FROM tickets WHERE title LIKE '%lock%' ORDER BY id;
```

## Why LIKE '%word%' is slow

An index on `title` is sorted text. A pattern with a **fixed prefix** (`'Locked%'`) can use it, like finding a word in a dictionary. A pattern starting with `%` can't: the database must read every row. That's fine for thousands of rows and painful for millions. Full-text search fixes it.

## Collations

A **collation** decides how text compares and sorts. SQLite has `BINARY` (default: byte by byte), `NOCASE` (ASCII case-insensitive) and `RTRIM` (ignore trailing spaces). Apply one per column or per comparison:

```sql
SELECT username FROM employees WHERE username = 'ASILVA' COLLATE NOCASE;
```

PostgreSQL and SQL Server have locale-aware collations (`ja-JP`, `de-DE`, case- and accent-insensitive variants). Collation affects ORDER BY, comparisons, UNIQUE constraints and which indexes can be used: an index built with BINARY can't serve a NOCASE comparison. For Japanese text, sorting by reading (furigana) usually needs a separate reading column.

## Full-text search (FTS)

Full-text search builds an **inverted index**: for every word (token), the list of documents that contain it, like the index at the back of a book. Searching for a word is then a lookup, no matter how many documents there are.

In SQLite it's the **FTS5** extension (included in standard builds, including Python's):

```sql
CREATE VIRTUAL TABLE kb USING fts5(title, body);
INSERT INTO kb VALUES
  ('Reset a password', 'Use the self-service portal to reset your password.'),
  ('VPN troubleshooting', 'If the VPN disconnects, check the Wi-Fi and reinstall the client.'),
  ('Printer setup', 'Add the printer by IP address. Drivers are on the file server.');
SELECT title FROM kb WHERE kb MATCH 'printer';
```

FTS5 query syntax:

| Query | Matches |
|---|---|
| `vpn wifi` | both words (implicit AND) |
| `vpn OR wifi` | either |
| `vpn NOT client` | vpn without client |
| `"file server"` | the exact phrase |
| `pass*` | prefix: password, passwords, passkey |
| `title:printer` | only in the title column |
| `NEAR(vpn client, 5)` | within 5 tokens |

Results can be sorted by relevance with `ORDER BY rank` (BM25 scoring: rare words and short documents score higher), and `highlight(kb, 1, '[', ']')` or `snippet(...)` show where the match is.

### Tokenizers

The tokenizer splits text into words. FTS5's default `unicode61` lowercases and removes accents, so `café` matches `cafe`. `porter` adds English **stemming** (`connect`, `connected`, `connecting` all match). Japanese and Chinese don't separate words with spaces, so they need a different approach: the `trigram` tokenizer (indexes every 3-character sequence, so it supports substring search in any language) or a morphological analyser like MeCab in PostgreSQL extensions.

### Keeping FTS in sync

An FTS table is separate from your real table. Either store the text only in FTS, or use an **external content** table (`content='tickets'`) plus triggers that update the index whenever the base table changes.

## Full text in other databases

- **PostgreSQL**: `to_tsvector('english', body) @@ to_tsquery('vpn & client')`, with a GIN index; `pg_trgm` for fuzzy and substring matching.
- **SQL Server**: full-text catalogs with `CONTAINS` and `FREETEXT`.
- **MySQL**: `FULLTEXT` indexes with `MATCH ... AGAINST`.
- Beyond that: dedicated engines like Elasticsearch/OpenSearch for logs and big search features.

## Fuzzy matching

Typos ("pritner") defeat exact search. Options: trigram similarity (`pg_trgm`), edit distance (Levenshtein; PostgreSQL's `fuzzystrmatch`), or phonetic codes (`soundex`, useful for English names only).

@@quiz
Why can't a normal index speed up `WHERE title LIKE '%printer%'`?
- Indexes don't work on text
* The pattern starts with a wildcard, so there's no prefix to look up in the sorted index
- LIKE is always case-sensitive
- Because of NULLs
? `'printer%'` can use an index; `'%printer%'` can't.

@@quiz
What does a full-text inverted index store?
- Every row sorted by length
* For each token, the documents (and positions) that contain it
- A hash of each document
- Only the first word of each document
? Like the index at the back of a book.

@@quiz
Which FTS5 tokenizer helps with Japanese text, which has no spaces between words?
- porter
- unicode61
* trigram
- ascii
? It indexes every 3-character sequence, so substring search works.

@@drill
Which SQLite virtual table module provides full-text search?
= FTS5
= fts
? CREATE VIRTUAL TABLE docs USING fts5(body).

@@flashcards
`LIKE '%x%'` :: substring match, can't use a normal index
`ILIKE` :: case-insensitive LIKE (PostgreSQL)
`GLOB` :: case-sensitive wildcards * ? (SQLite)
`COLLATE NOCASE` :: ASCII case-insensitive compare
inverted index :: token → documents
`CREATE VIRTUAL TABLE t USING fts5(...)` :: SQLite full-text table
`MATCH 'a OR b'` :: FTS query
`ORDER BY rank` :: BM25 relevance
`porter` tokenizer :: English stemming
`trigram` tokenizer :: substring search, CJK
`to_tsvector` / `to_tsquery` :: PostgreSQL full text

@@exercise Case-insensitive account lookups
Users type usernames in any case. Show the `input`, the matching employee `name` (or `no match`) for the inputs `ASilva`, `MLEE`, `kTanaka` and `jdoe`, comparing with `COLLATE NOCASE`. Keep the input order.
@@starter
WITH inputs(pos, input) AS (VALUES (1, 'ASilva'), (2, 'MLEE'), (3, 'kTanaka'), (4, 'jdoe'))
SELECT i.input, e.name FROM inputs i JOIN employees e ON e.username = i.input ORDER BY i.pos;
@@expected
input   | name
--------+-------------
ASilva  | Ana Silva
MLEE    | Mia Lee
kTanaka | Kenji Tanaka
jdoe    | no match
@@requires
COLLATE NOCASE
LEFT JOIN
@@hint
`LEFT JOIN employees e ON e.username = i.input COLLATE NOCASE` and `COALESCE(e.name, 'no match')`.
@@solution
WITH inputs(pos, input) AS (VALUES (1, 'ASilva'), (2, 'MLEE'), (3, 'kTanaka'), (4, 'jdoe'))
SELECT i.input, COALESCE(e.name, 'no match') AS name
FROM inputs i LEFT JOIN employees e ON e.username = i.input COLLATE NOCASE
ORDER BY i.pos;

@@exercise A searchable knowledge base
Create an FTS5 table `kb(title, body)` with the porter tokenizer (`tokenize='porter unicode61'`), load the starter articles, then run three searches and show `query` and the matching `title`s (sorted by title within each query):

1. `printer`
2. `reset* AND password` (prefix and AND)
3. `connecting` (stemming should match "connect" and "disconnects")
@@starter
-- CREATE VIRTUAL TABLE kb USING fts5(...);
INSERT INTO kb VALUES
  ('Reset a password', 'Use the self-service portal to reset your password.'),
  ('VPN troubleshooting', 'If the VPN disconnects, check the Wi-Fi and reinstall the client.'),
  ('Printer setup', 'Add the printer by IP address. Drivers are on the file server.'),
  ('Connect to Wi-Fi', 'Choose SAKURA-CORP and sign in with your account.'),
  ('Shared printers', 'Each floor has a colour printer. Scans go to your email.');
@@expected
query               | title
--------------------+-----------------
printer             | Printer setup
printer             | Shared printers
reset* AND password | Reset a password
connecting          | Connect to Wi-Fi
@@requires
USING fts5
MATCH
porter
@@hint
`CREATE VIRTUAL TABLE kb USING fts5(title, body, tokenize = 'porter unicode61');`. Then three SELECTs joined with UNION ALL, each like `SELECT 'printer' AS query, title FROM kb WHERE kb MATCH 'printer'`. Sort with `ORDER BY` on a query number and title (put the number in a hidden column via an outer query, or just order by query text in a fixed order using CASE).
@@solution
CREATE VIRTUAL TABLE kb USING fts5(title, body, tokenize = 'porter unicode61');
INSERT INTO kb VALUES
  ('Reset a password', 'Use the self-service portal to reset your password.'),
  ('VPN troubleshooting', 'If the VPN disconnects, check the Wi-Fi and reinstall the client.'),
  ('Printer setup', 'Add the printer by IP address. Drivers are on the file server.'),
  ('Connect to Wi-Fi', 'Choose SAKURA-CORP and sign in with your account.'),
  ('Shared printers', 'Each floor has a colour printer. Scans go to your email.');

SELECT query, title FROM (
    SELECT 1 AS n, 'printer' AS query, title FROM kb WHERE kb MATCH 'printer'
    UNION ALL
    SELECT 2, 'reset* AND password', title FROM kb WHERE kb MATCH 'reset* AND password'
    UNION ALL
    SELECT 3, 'connecting', title FROM kb WHERE kb MATCH 'connecting'
)
ORDER BY n, title;

@@exercise Highlighted search results
Search the help desk `tickets` with FTS5: build `ticket_fts(title)` from the tickets table (keep the ticket id as the FTS `rowid`), then search for `locked OR login` and show `id` and the `title` with matches wrapped in `[` `]` by `highlight()`. Sort by id.
@@starter
SELECT id, title FROM tickets WHERE title LIKE '%lock%' OR title LIKE '%login%' ORDER BY id;
@@expected
id | title
---+-------------------------------
3  | [Locked] out of account
11 | Suspicious [login] from abroad
14 | [Locked] out again
@@requires
fts5
highlight(
@@hint
`CREATE VIRTUAL TABLE ticket_fts USING fts5(title); INSERT INTO ticket_fts(rowid, title) SELECT id, title FROM tickets;` Then `SELECT rowid AS id, highlight(ticket_fts, 0, '[', ']') AS title FROM ticket_fts WHERE ticket_fts MATCH 'locked OR login' ORDER BY rowid;`
@@solution
CREATE VIRTUAL TABLE ticket_fts USING fts5(title);
INSERT INTO ticket_fts (rowid, title) SELECT id, title FROM tickets;

SELECT rowid AS id, highlight(ticket_fts, 0, '[', ']') AS title
FROM ticket_fts
WHERE ticket_fts MATCH 'locked OR login'
ORDER BY rowid;
