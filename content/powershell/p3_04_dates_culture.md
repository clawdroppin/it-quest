@@lesson p3-dates Dates, Times, Numbers and Culture
@@concepts numbers, strings
@@text
Password ages, certificate expiry, log rotation, "servers not patched in 30 days": admin work is full of dates. And because Windows machines run in many languages, how numbers and dates are **written** depends on the machine's culture. A script that works in New York can break in Warsaw or Tokyo.

## DateTime and TimeSpan

```powershell,norun
$now = Get-Date
$now.AddDays(-30)                 # 30 days ago
$now.Date                         # today at 00:00
(Get-Date '2026-12-31') - $now    # a TimeSpan
```

```powershell
$start = [datetime]'2026-11-01 08:00'
$end = [datetime]'2026-11-03 17:30'
$span = $end - $start
"$($span.Days) days, $($span.Hours) hours, total $($span.TotalHours) hours"
$start.AddHours(36).ToString('yyyy-MM-dd HH:mm')
[datetime]'2026-11-03' -gt [datetime]'2026-10-31'
New-TimeSpan -Days 1 -Hours 6 | Select-Object -ExpandProperty TotalMinutes
```

`TotalDays` is fractional; `Days` is just the days component. Mixing them up is a classic bug ("1 day" for 47 hours).

## Formatting dates

Use **custom format strings**, and ISO 8601 (`yyyy-MM-dd`) for anything a machine reads or sorts: file names, logs, CSV.

| Format | Result |
|---|---|
| `yyyy-MM-dd` | 2026-11-03 |
| `yyyy-MM-ddTHH:mm:ssZ` with `ToUniversalTime()` | 2026-11-03T08:30:00Z |
| `HH:mm` vs `hh:mm tt` | 24-hour vs 12-hour clock |
| `ddd`, `dddd`, `MMM` | day and month **names**: culture-dependent |
| `'o'` | round-trip ISO with fractions and offset |

Careful: `mm` is minutes, `MM` is months.

## Time zones and UTC

Servers log in UTC; people think in local time. Store and compare in UTC, convert only for display:

```powershell
$utc = [datetime]::SpecifyKind([datetime]'2026-11-03 08:30', 'Utc')
$tokyo = [TimeZoneInfo]::FindSystemTimeZoneById('Tokyo Standard Time')
[TimeZoneInfo]::ConvertTimeFromUtc($utc, $tokyo).ToString('yyyy-MM-dd HH:mm')
```

`[datetimeoffset]` keeps the offset with the value, which is safer for timestamps from APIs.

## Culture: the silent bug

```powershell
$de = [cultureinfo]'de-DE'
(1234.5).ToString('N2', $de)                       # 1.234,50
(1234.5).ToString('N2', [cultureinfo]::InvariantCulture)  # 1,234.50
[datetime]::Parse('03.11.2026', $de).ToString('yyyy-MM-dd')
```

- `'{0:N2}' -f $x` and `"$x"` inside strings behave differently: the `-f` operator uses the **current culture**, string interpolation uses the **invariant** culture.
- Parsing text with `[datetime]'...'` (a cast) uses the invariant culture, US style: month/day. `Get-Date '03/11/2026'` uses the **current** culture.
- When the format is known, parse exactly: `[datetime]::ParseExact('20261103', 'yyyyMMdd', $null)`.
- For files other programs read, use `[cultureinfo]::InvariantCulture` explicitly.

## Number formatting

```powershell
$inv = [cultureinfo]::InvariantCulture
(0.4567).ToString('P1', $inv)            # percent
(42).ToString('D5')                       # 00042
(255).ToString('X2')                      # FF (hex)
(1536MB / 1GB).ToString('0.0', $inv)      # 1.5
[math]::Round(2.345, 2)                   # rounding (banker's by default)
[math]::Round(2.5, [MidpointRounding]::AwayFromZero)
```

@@quiz
A TimeSpan is 1 day and 23 hours. What are `.Days` and `.TotalDays`?
* 1 and about 1.96
- 2 and 2
- 1 and 1
- 47 and 1.96
? Days is a component; TotalDays is the whole span.

@@quiz
Why use `yyyy-MM-dd` in file names and logs?
- It's shorter
* It's unambiguous across cultures and sorts correctly as text
- Windows requires it
- It includes the time zone
? 03/11/2026 means March 11 in the US and 3 November in Europe.

@@quiz
Which uses the current culture?
- `"$x"` string interpolation
* `'{0:N2}' -f $x`
- `[datetime]'2026-11-03'`
- `$x.ToString([cultureinfo]::InvariantCulture)`
? Interpolation and casts are invariant; -f and Get-Date parsing are culture-aware.

@@flashcards
`.AddDays(-30)` :: 30 days ago
`$b - $a` :: a TimeSpan
`.Days` vs `.TotalDays` :: component vs whole
`yyyy-MM-dd` :: ISO date
`MM` vs `mm` :: month vs minute
`ToUniversalTime()` :: to UTC
`[TimeZoneInfo]` :: convert between zones
`InvariantCulture` :: culture-neutral formatting
`ParseExact` :: parse a known format
`'N2'` / `'P1'` / `'X2'` :: number / percent / hex
`[MidpointRounding]::AwayFromZero` :: school rounding

@@exercise Certificate expiry report
Given certificates with expiry dates (as text, ISO format) and a fixed "today" of 2026-11-03:

- parse each date with `ParseExact`
- compute the days left with a TimeSpan (`.Days`)
- status: `EXPIRED` (< 0), `RENEW` (≤ 30), `OK` otherwise
- print them sorted by days left, aligned: name padded to 18, days right-aligned in 5, then status and the expiry date written as `dd MMM yyyy` in the **invariant** culture
- finish with the next expiry date in ISO format and the weekday it falls on
@@starter
$today = [datetime]::ParseExact('2026-11-03', 'yyyy-MM-dd', $null)
$certs = @(
    [pscustomobject]@{ Name = 'www.example.jp'; Expires = '2026-11-20' }
    [pscustomobject]@{ Name = 'vpn.example.jp'; Expires = '2027-04-01' }
    [pscustomobject]@{ Name = 'old.example.jp'; Expires = '2026-10-15' }
    [pscustomobject]@{ Name = 'mail.example.jp'; Expires = '2026-12-01' }
)
'TODO'
@@expected
old.example.jp      -19 EXPIRED 15 Oct 2026
www.example.jp       17 RENEW   20 Nov 2026
mail.example.jp      28 RENEW   01 Dec 2026
vpn.example.jp      149 OK      01 Apr 2027
next to expire: 2026-11-20 (Friday)
@@requires
ParseExact
InvariantCulture
.Days
@@hint
Build objects with `Name`, `Date` and `Days = ($date - $today).Days`. Format a line with `$_.Name.PadRight(18) + $_.Days.ToString().PadLeft(5) + ' ' + $status.PadRight(7) + ' ' + $_.Date.ToString('dd MMM yyyy', $inv)`. "Next to expire" is the first one with `Days -ge 0`; `.DayOfWeek` gives the weekday name in English regardless of culture.
@@solution
$today = [datetime]::ParseExact('2026-11-03', 'yyyy-MM-dd', $null)
$certs = @(
    [pscustomobject]@{ Name = 'www.example.jp'; Expires = '2026-11-20' }
    [pscustomobject]@{ Name = 'vpn.example.jp'; Expires = '2027-04-01' }
    [pscustomobject]@{ Name = 'old.example.jp'; Expires = '2026-10-15' }
    [pscustomobject]@{ Name = 'mail.example.jp'; Expires = '2026-12-01' }
)
$inv = [cultureinfo]::InvariantCulture

$rows = foreach ($c in $certs) {
    $date = [datetime]::ParseExact($c.Expires, 'yyyy-MM-dd', $inv)
    [pscustomobject]@{ Name = $c.Name; Date = $date; Days = ($date - $today).Days }
}
$rows = $rows | Sort-Object Days
foreach ($r in $rows) {
    if ($r.Days -lt 0) { $status = 'EXPIRED' }
    elseif ($r.Days -le 30) { $status = 'RENEW' }
    else { $status = 'OK' }
    $r.Name.PadRight(18) + $r.Days.ToString().PadLeft(5) + ' ' + $status.PadRight(7) + ' ' + $r.Date.ToString('dd MMM yyyy', $inv)
}
$next = $rows | Where-Object Days -ge 0 | Select-Object -First 1
"next to expire: $($next.Date.ToString('yyyy-MM-dd')) ($($next.Date.DayOfWeek))"

@@exercise Logs from three countries
Three offices send log lines with local formats. Normalise them:

- Tokyo: `2026/11/03 17:30:05` in Japan time (UTC+9)
- Warsaw: `03.11.2026 09:31:10`, Polish format, in Central European time (UTC+1 in November)
- New York: `11/03/2026 03:32:00`, US format, Eastern time (UTC-5 in November)

Parse each with `ParseExact` and its offset as a `[datetimeoffset]`, convert to UTC, print the events sorted by UTC time as ISO strings, then the gap between the first and last event, and the total bytes formatted with a thousands separator in the invariant culture **and** in `pl-PL` (show both).
@@starter
$logs = @(
    [pscustomobject]@{ Office = 'Tokyo';    Stamp = '2026/11/03 17:30:05'; Format = 'yyyy/MM/dd HH:mm:ss'; Offset = 9;  Bytes = 120400 }
    [pscustomobject]@{ Office = 'Warsaw';   Stamp = '03.11.2026 09:31:10'; Format = 'dd.MM.yyyy HH:mm:ss'; Offset = 1;  Bytes = 98250 }
    [pscustomobject]@{ Office = 'New York'; Stamp = '11/03/2026 03:32:00'; Format = 'MM/dd/yyyy HH:mm:ss'; Offset = -5; Bytes = 1504 }
)
'TODO'
@@expected
2026-11-03T08:30:05Z Tokyo
2026-11-03T08:31:10Z Warsaw
2026-11-03T08:32:00Z New York
first to last: 1 min 55 s
total bytes (invariant): 220,154
total bytes (pl-PL): 220 154
@@requires
ParseExact
[datetimeoffset]
UtcDateTime
@@hint
`$local = [datetime]::ParseExact($l.Stamp, $l.Format, $inv)`, then `$dto = [datetimeoffset]::new($local, [timespan]::FromHours($l.Offset))` and `$dto.UtcDateTime.ToString("yyyy-MM-dd'T'HH:mm:ss'Z'")`. Polish uses a non-breaking space as the group separator; replace `[char]0xA0` (and `[char]0x202F`) with a normal space for display.
@@solution
$logs = @(
    [pscustomobject]@{ Office = 'Tokyo';    Stamp = '2026/11/03 17:30:05'; Format = 'yyyy/MM/dd HH:mm:ss'; Offset = 9;  Bytes = 120400 }
    [pscustomobject]@{ Office = 'Warsaw';   Stamp = '03.11.2026 09:31:10'; Format = 'dd.MM.yyyy HH:mm:ss'; Offset = 1;  Bytes = 98250 }
    [pscustomobject]@{ Office = 'New York'; Stamp = '11/03/2026 03:32:00'; Format = 'MM/dd/yyyy HH:mm:ss'; Offset = -5; Bytes = 1504 }
)
$inv = [cultureinfo]::InvariantCulture

$events = foreach ($l in $logs) {
    $local = [datetime]::ParseExact($l.Stamp, $l.Format, $inv)
    $dto = [datetimeoffset]::new($local, [timespan]::FromHours($l.Offset))
    [pscustomobject]@{ Office = $l.Office; Utc = $dto.UtcDateTime }
}
$events = $events | Sort-Object Utc
foreach ($e in $events) { "$($e.Utc.ToString("yyyy-MM-dd'T'HH:mm:ss'Z'", $inv)) $($e.Office)" }
$gap = $events[-1].Utc - $events[0].Utc
"first to last: $([int][math]::Floor($gap.TotalMinutes)) min $($gap.Seconds) s"

$total = ($logs | Measure-Object Bytes -Sum).Sum
"total bytes (invariant): $($total.ToString('N0', $inv))"
$pl = $total.ToString('N0', [cultureinfo]'pl-PL') -replace "[  ]", ' '
"total bytes (pl-PL): $pl"
