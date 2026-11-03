@@lesson r11-datetime Dates, Times and Time Zones
@@concepts strings, numbers
@@text
Expiring certificates, maintenance windows, log timestamps from servers in three countries, SLA deadlines that skip weekends: IT work is full of time calculations, and they're a classic source of bugs. Python's `datetime` and `zoneinfo` modules handle them correctly if you follow a few rules.

## The types

```python
from datetime import date, time, datetime, timedelta, timezone

d = date(2026, 11, 3)
t = time(9, 30)
dt = datetime(2026, 11, 3, 9, 30)
print(d.isoformat(), t, dt, d.weekday())        # weekday: Monday = 0
print(dt + timedelta(hours=36), date(2026, 12, 25) - d)
```

- `timedelta` is a duration: `timedelta(days=1, hours=2)`; `.total_seconds()` converts it.
- Subtracting two dates or datetimes gives a `timedelta`.
- `date.today()`, `datetime.now()`.

## Naive vs aware

> **On Windows:** Python has no built-in time-zone database there, so `ZoneInfo("Asia/Tokyo")` raises `ZoneInfoNotFoundError` until you run `pip install tzdata` (or `uv add tzdata`). The two examples below are shown without a Run button for that reason; try them after installing it.

A **naive** datetime has no time zone: `datetime(2026, 11, 3, 9, 30)` could be 9:30 anywhere. An **aware** datetime carries a `tzinfo` and represents one exact instant:

```python,norun
from datetime import datetime, timezone
from zoneinfo import ZoneInfo

now_utc = datetime.now(timezone.utc)                        # aware, UTC
tokyo = datetime(2026, 11, 3, 9, 0, tzinfo=ZoneInfo("Asia/Tokyo"))
print(tokyo, tokyo.astimezone(ZoneInfo("Europe/Prague")), tokyo.astimezone(timezone.utc))
```

The golden rules:

1. **Store and compute in UTC** (databases, logs, APIs). Convert to local time only for display.
2. **Never mix** naive and aware datetimes (comparing them raises `TypeError`).
3. Use **named zones** (`ZoneInfo("Europe/Prague")`), never fixed offsets like +01:00 for places, because offsets change with **daylight saving time** (DST) and politics. On Windows, install the `tzdata` package so `zoneinfo` has its database.
4. `datetime.utcnow()` is deprecated: it returns a naive value. Use `datetime.now(timezone.utc)`.

## DST traps

On the day clocks change, a local day has 23 or 25 hours, some local times don't exist (02:30 when clocks jump to 03:00) and some happen twice:

```python,norun
from datetime import datetime, timedelta, timezone
from zoneinfo import ZoneInfo

prague = ZoneInfo("Europe/Prague")
before = datetime(2026, 3, 28, 12, 0, tzinfo=prague)       # day before clocks go forward
naive_next = before + timedelta(days=1)                      # same wall-clock time next day
exact_next = (before.astimezone(timezone.utc) + timedelta(hours=24)).astimezone(prague)
print(naive_next, "|", exact_next)
```

Adding `timedelta(days=1)` to an aware datetime keeps the **wall-clock** time; adding 24 hours in UTC keeps the **elapsed** time. Decide which one your rule means ("every day at noon" vs "24 hours later").

## Parsing and formatting

```python
from datetime import datetime, timezone

dt = datetime.fromisoformat("2026-11-03T09:30:00+09:00")   # ISO 8601: always prefer it
print(dt.isoformat(), dt.strftime("%d.%m.%Y %H:%M"), dt.strftime("%a %b %d"))
syslog = datetime.strptime("Nov  3 09:30:01 2026", "%b %d %H:%M:%S %Y")
print(syslog, int(dt.timestamp()))
print(datetime.fromtimestamp(1762130000, tz=timezone.utc))
```

| Code | Meaning | Example |
|---|---|---|
| `%Y-%m-%d` | year, month, day | 2026-11-03 |
| `%H:%M:%S` | 24-hour time | 09:30:01 |
| `%a` / `%A` | weekday name | Tue / Tuesday |
| `%b` / `%B` | month name | Nov / November |
| `%z` | UTC offset | +0900 |
| `%j` | day of year | 307 |
| `%G-W%V` | ISO week | 2026-W45 |

Exchange data as **ISO 8601** (`2026-11-03T00:30:00Z`) or **Unix timestamps** (seconds since 1970-01-01 UTC). Month and weekday names in `strftime` depend on the locale; don't parse localised dates if you can avoid it.

## Calendar arithmetic

- "One month later" isn't a timedelta (months have different lengths). `dateutil.relativedelta(months=1)` handles it; or write the rule yourself (exercise).
- `calendar.monthrange(year, month)` gives the number of days in a month.
- Business days: skip weekends and a holiday list (`holidays` package for national holidays, e.g. Japan's).
- `time.monotonic()` / `perf_counter()` for measuring durations: they never jump when the system clock is adjusted.

## Libraries

`python-dateutil` (flexible parsing, relativedelta, recurrence rules), **pendulum** and **whenever** (friendlier, stricter APIs), `croniter` (cron schedules).

@@quiz
What should applications store in databases and logs?
* Aware datetimes in UTC (or Unix timestamps)
- Local time of the server
- Naive datetimes
- Formatted strings in the user's language
? Convert to local time only for display.

@@quiz
Why use `ZoneInfo("Europe/Prague")` instead of a fixed +01:00 offset?
- It's shorter
* The offset changes with daylight saving time (and politics); the named zone knows the rules
- Fixed offsets are not allowed
- For speed
? Prague is +01:00 in winter and +02:00 in summer.

@@quiz
What happens when you compare a naive and an aware datetime?
- Python assumes UTC
- Python assumes local time
* TypeError
- It returns False
? Never mix them.

@@quiz
Which clock should you use to measure how long an operation took?
- datetime.now()
* time.monotonic() or time.perf_counter()
- time.time()
- date.today()
? They never jump when the system clock changes.

@@flashcards
timedelta :: a duration
Naive datetime :: no time zone attached
Aware datetime :: has tzinfo; an exact instant
zoneinfo :: standard library IANA time zones (`ZoneInfo("Asia/Tokyo")`)
UTC :: Coordinated Universal Time, the reference for storage
DST :: daylight saving time; local days of 23 or 25 hours
ISO 8601 :: `2026-11-03T09:30:00+09:00`
Unix timestamp :: seconds since 1970-01-01 UTC
`strptime` / `strftime` :: parse / format with codes like `%Y-%m-%d`
time.monotonic :: clock for measuring durations

@@exercise Certificate expiry report
Write `expiry_report(certs, now)` where `certs` is a dict name → expiry as an ISO 8601 string with offset (e.g. `"2026-11-10T00:00:00+09:00"`) and `now` is an aware UTC datetime. Return a list of lines sorted by expiry (soonest first):

- expired: `"<name>: EXPIRED 3 days ago"`
- expiring within 14 days: `"<name>: expires in 5 days (2026-11-09 15:00 UTC)"`
- otherwise: `"<name>: ok (expires 2027-01-31)"` with the UTC date

"Days" are whole days, rounded down, from the `timedelta` (`delta.days`); for expired, use the days of `now - expiry`.
@@starter
from datetime import datetime, timezone


def expiry_report(certs, now):
    pass


NOW = datetime(2026, 11, 3, 12, 0, tzinfo=timezone.utc)
CERTS = {
    "vpn.corp.example": "2026-11-09T00:00:00+09:00",
    "old.corp.example": "2026-10-31T00:00:00+00:00",
    "www.corp.example": "2027-01-31T23:00:00-05:00",
}
for line in expiry_report(CERTS, NOW):
    print(line)
@@expected
old.corp.example: EXPIRED 3 days ago
vpn.corp.example: expires in 5 days (2026-11-08 15:00 UTC)
www.corp.example: ok (expires 2027-02-01)
@@tests
NOW = datetime(2026, 11, 3, 12, 0, tzinfo=timezone.utc)
check(expiry_report({}, NOW), [], "empty")
check(expiry_report({"a": "2026-11-17T12:00:00+00:00"}, NOW), ["a: expires in 14 days (2026-11-17 12:00 UTC)"], "exactly 14 days")
check(expiry_report({"b": "2026-11-17T12:00:01+00:00"}, NOW), ["b: ok (expires 2026-11-17)"], "just over 14 days")
@@hint
Parse with `datetime.fromisoformat(text).astimezone(timezone.utc)`. Sort the `(expiry, name)` pairs. `delta = expiry - now`; if `delta < timedelta(0)` it's expired (`(now - expiry).days`); if `delta <= timedelta(days=14)` it's expiring soon; format with `strftime("%Y-%m-%d %H:%M")`.
@@solution
from datetime import datetime, timedelta, timezone


def expiry_report(certs, now):
    parsed = sorted((datetime.fromisoformat(text).astimezone(timezone.utc), name) for name, text in certs.items())
    lines = []
    for expiry, name in parsed:
        delta = expiry - now
        if delta < timedelta(0):
            lines.append(f"{name}: EXPIRED {(now - expiry).days} days ago")
        elif delta <= timedelta(days=14):
            lines.append(f"{name}: expires in {delta.days} days ({expiry.strftime('%Y-%m-%d %H:%M')} UTC)")
        else:
            lines.append(f"{name}: ok (expires {expiry.strftime('%Y-%m-%d')})")
    return lines


NOW = datetime(2026, 11, 3, 12, 0, tzinfo=timezone.utc)
CERTS = {
    "vpn.corp.example": "2026-11-09T00:00:00+09:00",
    "old.corp.example": "2026-10-31T00:00:00+00:00",
    "www.corp.example": "2027-01-31T23:00:00-05:00",
}
for line in expiry_report(CERTS, NOW):
    print(line)

@@exercise SLA deadlines in business hours
A ticket's SLA counts only **business hours**: Monday–Friday, 09:00–17:00 local time, excluding holidays. Write `sla_deadline(opened, hours, holidays)`:

- `opened` is a naive local datetime, `hours` a whole number of business hours, `holidays` a set of `date` objects
- if the ticket is opened outside business hours, the clock starts at the next business-hours start
- return the naive local datetime when the SLA expires

Also write `add_months(d, n)`: the same day `n` months later, clamped to the month's last day (`2026-01-31` + 1 → `2026-02-28`).
@@starter
from datetime import datetime, date, timedelta
import calendar


def sla_deadline(opened, hours, holidays):
    pass


def add_months(d, n):
    pass


HOLIDAYS = {date(2026, 11, 23)}
print(sla_deadline(datetime(2026, 11, 20, 15, 30), 4, HOLIDAYS))
print(add_months(date(2026, 1, 31), 1), add_months(date(2026, 11, 30), 3))
@@expected
2026-11-24 11:30:00
2026-02-28 2027-02-28
@@tests
H = set()
check(sla_deadline(datetime(2026, 11, 3, 10, 0), 2, H), datetime(2026, 11, 3, 12, 0), "same day")
check(sla_deadline(datetime(2026, 11, 3, 16, 0), 8, H), datetime(2026, 11, 4, 16, 0), "overnight")
check(sla_deadline(datetime(2026, 11, 3, 20, 0), 1, H), datetime(2026, 11, 4, 10, 0), "opened after hours")
check(sla_deadline(datetime(2026, 11, 7, 11, 0), 1, H), datetime(2026, 11, 9, 10, 0), "opened on saturday")
check(sla_deadline(datetime(2026, 11, 3, 9, 0), 8, H), datetime(2026, 11, 3, 17, 0), "full day ends at 17:00")
check(add_months(date(2028, 1, 31), 1), date(2028, 2, 29), "leap year")
check(add_months(date(2026, 5, 15), -6), date(2025, 11, 15), "backwards")
@@hint
Work in minutes. A helper `is_open(dt)` checks weekday < 5, the date not in holidays and `9 <= hour < 17`. Simplest correct approach: first move `t` forward to the next open minute (to 09:00 of the next working day if needed), then `remaining = hours * 60`; while remaining: `end_of_day = t.replace(hour=17, minute=0)`; `take = min(remaining, minutes until end_of_day)`; advance; if remaining is left, jump to the next working day's 09:00. For months: `total = d.month - 1 + n; y, m = d.year + total // 12, total % 12 + 1; day = min(d.day, calendar.monthrange(y, m)[1])`.
@@solution
from datetime import datetime, date, timedelta
import calendar


def _workday(d, holidays):
    return d.weekday() < 5 and d not in holidays


def _next_start(t, holidays):
    if _workday(t.date(), holidays) and t.hour < 9:
        return t.replace(hour=9, minute=0, second=0)
    if _workday(t.date(), holidays) and 9 <= t.hour < 17:
        return t
    d = t.date() + timedelta(days=1)
    while not _workday(d, holidays):
        d += timedelta(days=1)
    return datetime(d.year, d.month, d.day, 9, 0)


def sla_deadline(opened, hours, holidays):
    t = _next_start(opened, holidays)
    remaining = timedelta(hours=hours)
    while True:
        end_of_day = t.replace(hour=17, minute=0, second=0)
        available = end_of_day - t
        if remaining <= available:
            return t + remaining
        remaining -= available
        t = _next_start(end_of_day, holidays)


def add_months(d, n):
    total = d.month - 1 + n
    y, m = d.year + total // 12, total % 12 + 1
    day = min(d.day, calendar.monthrange(y, m)[1])
    return date(y, m, day)


HOLIDAYS = {date(2026, 11, 23)}
print(sla_deadline(datetime(2026, 11, 20, 15, 30), 4, HOLIDAYS))
print(add_months(date(2026, 1, 31), 1), add_months(date(2026, 11, 30), 3))
