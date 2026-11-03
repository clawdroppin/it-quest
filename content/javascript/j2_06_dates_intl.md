@@lesson j2-dates Dates, Times and Internationalisation
@@concepts numbers, strings
@@text
Dates are where JavaScript has its oldest, most awkward API. Understanding it, and the newer `Intl` and `Temporal` APIs, saves you from off-by-one-month and time-zone bugs in logs, schedules and reports.

## Date is a timestamp

A `Date` is just a number: milliseconds since 1970-01-01T00:00:00 **UTC** (the Unix epoch). How it's *displayed* depends on the time zone.

```javascript
const d = new Date("2026-11-03T14:30:00Z");       // Z = UTC: unambiguous
console.log(d.getTime(), d.toISOString());
console.log(d.getUTCFullYear(), d.getUTCMonth(), d.getUTCDate(), d.getUTCDay());   // month 10 = November!
console.log(Date.now() > d.getTime());
```

The traps:

- **Months are 0-based** (`0` = January), days of the month are 1-based, `getDay()` is the weekday (`0` = Sunday).
- `getHours()` etc. use the **local** time zone of the machine; `getUTCHours()` use UTC. Servers should think in UTC.
- `new Date("2026-11-03")` (date only) is parsed as UTC midnight, but `new Date("2026-11-03T10:00")` (no `Z`) as **local** time. Always include `Z` or an offset like `+09:00`.
- Dates are **mutable**: `d.setDate(d.getDate() + 1)` changes `d` in place.

## Store and send ISO 8601 in UTC

`toISOString()` gives `2026-11-03T14:30:00.000Z`: sortable as text, understood everywhere, unambiguous. Store that (or the epoch number) in files, databases and APIs; convert to local time only for display.

## Date arithmetic

```javascript
const DAY = 24 * 60 * 60 * 1000;
const opened = new Date("2026-10-30T09:00:00Z");
const closed = new Date("2026-11-03T15:30:00Z");
const hours = (closed - opened) / 3600000;         // subtracting Dates gives ms
console.log(hours.toFixed(1), "hours");
const inAWeek = new Date(opened.getTime() + 7 * DAY);
console.log(inAWeek.toISOString().slice(0, 10));
```

Adding `24 hours` isn't always "tomorrow" in local time: daylight-saving changes make some days 23 or 25 hours long. For calendar maths in a time zone, use `Temporal` or a library.

## Intl: formatting for people

```javascript
const when = new Date("2026-11-03T14:30:00Z");
console.log(new Intl.DateTimeFormat("en-GB", { dateStyle: "medium", timeStyle: "short", timeZone: "UTC" }).format(when));
console.log(new Intl.DateTimeFormat("ja-JP", { dateStyle: "full", timeZone: "Asia/Tokyo" }).format(when));
console.log(new Intl.NumberFormat("de-DE", { style: "currency", currency: "EUR" }).format(1234.5));
console.log(new Intl.NumberFormat("en", { notation: "compact" }).format(1530000));
console.log(new Intl.RelativeTimeFormat("en", { numeric: "auto" }).format(-1, "day"));
console.log(new Intl.ListFormat("en", { type: "conjunction" }).format(["web01", "db01", "mail01"]));
```

`Intl` knows each locale's date order, separators, currency symbols and plural rules, so you never hand-format those for users. `Intl.Collator` sorts text correctly per language (`"ä"` in German vs Swedish).

## Temporal

`Temporal` is the modern replacement for `Date`, with separate types for different meanings:

```javascript,norun
const meeting = Temporal.ZonedDateTime.from("2026-11-03T10:00[Europe/Warsaw]");
meeting.add({ days: 1 }).toString();          // respects DST
Temporal.PlainDate.from("2026-11-03").add({ months: 1 });   // 2026-12-03
Temporal.Now.instant();                        // an exact moment
```

It's shipping in browsers and Node.js through 2026; check support or use the `@js-temporal/polyfill`. Until then, `date-fns` and `Luxon` are popular libraries.

@@quiz
`new Date(2026, 10, 3)` is which date?
- 3 October 2026
* 3 November 2026, because months are 0-based
- 10 March 2026
- 2026-10-03 in UTC
? January is month 0.

@@quiz
How should timestamps be stored in files and APIs?
- Local time strings like "3/11/26 2:30 PM"
* ISO 8601 in UTC, e.g. `2026-11-03T14:30:00Z` (or epoch milliseconds)
- Day names
- Whatever `toString()` gives
? Convert to local time only when displaying.

@@quiz
What does subtracting two Date objects give?
* The difference in milliseconds
- A new Date
- The difference in days
- NaN
? Divide by 3,600,000 for hours.

@@flashcards
Epoch :: 1970-01-01T00:00:00Z; Date stores ms since then
`getUTCMonth()` :: 0-based month in UTC
ISO 8601 :: `2026-11-03T14:30:00Z`; sortable, unambiguous
`toISOString()` :: Date to ISO string in UTC
`Intl.DateTimeFormat` :: locale-aware date formatting
`Intl.NumberFormat` :: locale-aware numbers and currency
`Intl.RelativeTimeFormat` :: "yesterday", "in 3 hours"
Temporal :: modern date/time API with zone-aware types

@@exercise Ticket age and SLA
Write `businessMinutes(startIso, endIso)` that counts the minutes between two UTC timestamps that fall inside **business hours**: Monday–Friday, 09:00–17:00 UTC. Then write `slaStatus(openedIso, nowIso, limitMinutes)` returning `"ok"`, `"warning"` (at least 80% of the limit used) or `"breached"` (more than the limit), based on business minutes.

Assume both timestamps are whole minutes; looping minute by minute is fine for ranges up to a few weeks, but stepping day by day is better.
@@starter
function businessMinutes(startIso, endIso) {
  throw new Error("TODO: not done yet");
}

function slaStatus(openedIso, nowIso, limitMinutes) {
  throw new Error("TODO: not done yet");
}

@@tests
check(businessMinutes("2026-11-02T08:00:00Z", "2026-11-02T10:30:00Z"), 90, "Monday morning");
check(businessMinutes("2026-11-06T16:00:00Z", "2026-11-09T10:00:00Z"), 120, "over a weekend");
check(businessMinutes("2026-11-07T10:00:00Z", "2026-11-08T12:00:00Z"), 0, "all weekend");
check(businessMinutes("2026-11-02T10:00:00Z", "2026-11-02T09:00:00Z"), 0, "end before start");
check(slaStatus("2026-11-02T09:00:00Z", "2026-11-02T12:00:00Z", 240), "ok", "ok");
check(slaStatus("2026-11-02T09:00:00Z", "2026-11-02T12:20:00Z", 240), "warning", "warning at 83%");
check(slaStatus("2026-11-02T16:00:00Z", "2026-11-03T13:01:00Z", 240), "breached", "breached next day");
@@hint
Walk day by day in UTC: for each day from the start date, if `getUTCDay()` is 1–5, the window is that day's 09:00–17:00 (`Date.UTC(y, m, d, 9)` and `Date.UTC(y, m, d, 17)`). Add `max(0, min(end, windowEnd) - max(start, windowStart))`. Stop when the day starts after `end`.
@@solution
function businessMinutes(startIso, endIso) {
  const start = Date.parse(startIso);
  const end = Date.parse(endIso);
  if (end <= start) return 0;
  let total = 0;
  const day = new Date(start);
  day.setUTCHours(0, 0, 0, 0);
  while (day.getTime() < end) {
    const weekday = day.getUTCDay();
    if (weekday >= 1 && weekday <= 5) {
      const y = day.getUTCFullYear(), m = day.getUTCMonth(), d = day.getUTCDate();
      const open = Date.UTC(y, m, d, 9);
      const close = Date.UTC(y, m, d, 17);
      total += Math.max(0, Math.min(end, close) - Math.max(start, open));
    }
    day.setUTCDate(day.getUTCDate() + 1);
  }
  return Math.round(total / 60000);
}

function slaStatus(openedIso, nowIso, limitMinutes) {
  const used = businessMinutes(openedIso, nowIso);
  if (used > limitMinutes) return "breached";
  if (used >= limitMinutes * 0.8) return "warning";
  return "ok";
}

@@exercise Human-friendly times
Write `timeAgo(thenIso, nowIso)` returning English text with `Intl.RelativeTimeFormat("en", { numeric: "auto" })`: pick the largest unit where the absolute difference is at least 1 among `year` (365 days), `month` (30 days), `week`, `day`, `hour`, `minute`; under a minute → `"just now"`. Round the amount toward zero (`Math.trunc`). Future times work too ("in 2 hours").

Then `formatBytes(n)` using `Intl.NumberFormat("en", { maximumFractionDigits: 1 })` with units `B`, `KB`, `MB`, `GB`, `TB` (powers of 1024): `1536` → `"1.5 KB"`.
@@starter
function timeAgo(thenIso, nowIso) {
  throw new Error("TODO: not done yet");
}

function formatBytes(n) {
  throw new Error("TODO: not done yet");
}

@@tests
const now = "2026-11-03T12:00:00Z";
check(timeAgo("2026-11-03T11:59:30Z", now), "just now", "seconds");
check(timeAgo("2026-11-03T09:10:00Z", now), "2 hours ago", "hours, truncated");
check(timeAgo("2026-11-02T12:00:00Z", now), "yesterday", "yesterday");
check(timeAgo("2026-10-20T12:00:00Z", now), "2 weeks ago", "weeks");
check(timeAgo("2026-11-03T15:00:00Z", now), "in 3 hours", "future");
check(timeAgo("2024-11-01T12:00:00Z", now), "2 years ago", "years");
check([0, 1023, 1536, 5 * 1024 ** 3, 3.25 * 1024 ** 4].map(formatBytes), ["0 B", "1,023 B", "1.5 KB", "5 GB", "3.3 TB"], "bytes");
@@hint
Compute `diff = Date.parse(thenIso) - Date.parse(nowIso)` in ms (negative = past). Loop over `[["year", 365 * DAY], ["month", 30 * DAY], ["week", 7 * DAY], ["day", DAY], ["hour", HOUR], ["minute", MINUTE]]` and use the first where `Math.abs(diff) >= size`. For bytes, divide by 1024 while the value is at least 1024 and units remain.
@@solution
const MINUTE = 60000, HOUR = 60 * MINUTE, DAY = 24 * HOUR;

function timeAgo(thenIso, nowIso) {
  const diff = Date.parse(thenIso) - Date.parse(nowIso);
  const rtf = new Intl.RelativeTimeFormat("en", { numeric: "auto" });
  const units = [["year", 365 * DAY], ["month", 30 * DAY], ["week", 7 * DAY], ["day", DAY], ["hour", HOUR], ["minute", MINUTE]];
  for (const [unit, size] of units) {
    if (Math.abs(diff) >= size) return rtf.format(Math.trunc(diff / size), unit);
  }
  return "just now";
}

function formatBytes(n) {
  const units = ["B", "KB", "MB", "GB", "TB"];
  let value = n;
  let i = 0;
  while (value >= 1024 && i < units.length - 1) {
    value /= 1024;
    i++;
  }
  return `${new Intl.NumberFormat("en", { maximumFractionDigits: 1 }).format(value)} ${units[i]}`;
}
