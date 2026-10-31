@@lesson p2-regex Text Processing and Regular Expressions
@@concepts text-processing
@@text
Logs, command output from old tools (`ipconfig`, `netstat`), CSV exports from other systems: admins parse text constantly. PowerShell's regex operators use the **.NET regex** engine.

## -match and $Matches

```powershell
$line = '2026-10-31 08:15:02 ERROR [disk] C: has 3% free'
if ($line -match '(\d{4}-\d\d-\d\d) (\S+) (\w+) \[(\w+)\]') {
    "date=$($Matches[1]) level=$($Matches[3]) area=$($Matches[4])"
}
```

After a successful `-match`, `$Matches[0]` is the whole match and `$Matches[1]`, `[2]`, ... are the groups. **Named groups** are clearer:

```powershell
$line = 'user=ana ip=10.0.0.4 action=login'
if ($line -match 'user=(?<user>\w+) ip=(?<ip>[\d.]+)') {
    "$($Matches.user) from $($Matches.ip)"
}
```

## -replace and -split

```powershell
'Server: WEB01   Status:   OK' -replace '\s+', ' '
'2026-10-31' -replace '(\d+)-(\d+)-(\d+)', '$3/$2/$1'    # use single quotes so $1 isn't a variable!
'a, b;c  d' -split '[,;\s]+'
```

## -match on arrays filters

Used on an **array**, `-match` returns the items that match, like a quick `grep`:

```powershell
$log = 'INFO start', 'ERROR disk full', 'INFO ok', 'ERROR timeout'
$log -match '^ERROR'
```

## Select-String: grep for PowerShell

```powershell,norun
Select-String -Path C:\Logs\*.log -Pattern 'error|failed' | Select-Object Filename, LineNumber, Line
```

## Parsing old-style command output

```powershell,norun
# ipconfig prints text, but we can pick out the IPv4 addresses:
ipconfig | Select-String 'IPv4' | ForEach-Object { ($_ -split ':\s*')[-1] }
# (the modern way: Get-NetIPAddress -AddressFamily IPv4)
```

## Regex cheat sheet

| Pattern | Matches |
|---|---|
| `\d` `\w` `\s` | digit, word character, whitespace |
| `\D` `\W` `\S` | NOT digit / word / whitespace |
| `.` | any character |
| `+` `*` `?` `{2,4}` | 1+, 0+, 0 or 1, 2 to 4 |
| `^` `$` | start, end of the text |
| `[abc]` `[^abc]` | one of / none of |
| `( )` `(?<name> )` | group, named group |
| `a\|b` | a or b |
| `\.` | a literal dot |

@@quiz
After `'id=42' -match 'id=(\d+)'`, what is `$Matches[1]`?
* `42`
- `id=42`
- `id`
- `$true`
? `$Matches[0]` is the whole match, `$Matches[1]` the first group.

@@quiz
Why use single quotes in `-replace '(\w+)@(\w+)', '$2\$1'`?
- Regex needs single quotes
* In double quotes, PowerShell would treat `$1` and `$2` as variables before the regex sees them
- Single quotes are faster
- It doesn't matter
? Single quotes keep `$1` literal so `-replace` can use it as a group reference.

@@drill
Write a regex that matches an IPv4-looking address like `10.0.0.4` (digits and dots; don't worry about values over 255).
= \d+\.\d+\.\d+\.\d+
= \d{1,3}\.\d{1,3}\.\d{1,3}\.\d{1,3}
= (\d{1,3}\.){3}\d{1,3}
= (\d+\.){3}\d+
? Escape the dots with `\.`, since a bare `.` matches any character.

@@exercise Errors only
Print only the **message** part (after `ERROR `) of the lines in `$log` that start with `ERROR`, in order.
@@starter
$log = @(
    'INFO  service started'
    'ERROR disk C: almost full'
    'WARN  high CPU'
    'ERROR backup job failed'
)

@@expected
disk C: almost full
backup job failed
@@hint
`$log -match '^ERROR'` gives the error lines; `-replace '^ERROR\s+', ''` cuts the prefix. Both operators work on whole arrays.
@@solution
$log = @(
    'INFO  service started'
    'ERROR disk C: almost full'
    'WARN  high CPU'
    'ERROR backup job failed'
)
($log -match '^ERROR') -replace '^ERROR\s+', ''

@@exercise Parse an event line
Write `ConvertFrom-LogLine` that turns a line like `2026-10-31 08:15:02 WARN [web01] slow response 2300ms` into a `[pscustomobject]` with `Date`, `Time`, `Level`, `Host` and `Message`. Return `$null` for lines that don't match.
@@starter
function ConvertFrom-LogLine($line) {
    throw "TODO: not done yet"
}

@@tests
$e = ConvertFrom-LogLine '2026-10-31 08:15:02 WARN [web01] slow response 2300ms'
check $e.Date '2026-10-31' "Date"
check $e.Time '08:15:02' "Time"
check $e.Level 'WARN' "Level"
check $e.Host 'web01' "Host"
check $e.Message 'slow response 2300ms' "Message"
check (ConvertFrom-LogLine 'garbage') $null "ConvertFrom-LogLine 'garbage'"
@@hint
`if ($line -match '^(?<date>\S+) (?<time>\S+) (?<level>\w+) \[(?<host>[^\]]+)\] (?<msg>.*)$') { return [pscustomobject]@{ Date = $Matches.date; ... } }`. Without a match, `return $null`.
@@solution
function ConvertFrom-LogLine($line) {
    if ($line -match '^(?<date>\S+) (?<time>\S+) (?<level>\w+) \[(?<host>[^\]]+)\] (?<msg>.*)$') {
        return [pscustomobject]@{
            Date    = $Matches.date
            Time    = $Matches.time
            Level   = $Matches.level
            Host    = $Matches.host
            Message = $Matches.msg
        }
    }
    return $null
}
