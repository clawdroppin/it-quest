@@lesson p6-apis REST APIs, Secrets, Config and Logging
@@concepts json, errors, automation
@@text
Almost every modern system has a REST API: Microsoft Graph (Entra ID, Intune, Teams, Exchange Online), ServiceNow, Jira, GitHub, Slack, Okta, VMware, firewalls, monitoring tools. PowerShell's `Invoke-RestMethod` turns their JSON into objects, which makes PowerShell an excellent "glue" language for connecting systems.

## Calling an API

```powershell,norun
$headers = @{ Authorization = "Bearer $token"; Accept = 'application/json' }

# GET: JSON comes back as objects
$users = Invoke-RestMethod -Uri 'https://api.example.com/v1/users?department=IT' -Headers $headers
$users | Select-Object id, displayName, mail

# POST: send a body
$body = @{ title = 'Printer on 3F offline'; priority = 'high' } | ConvertTo-Json
Invoke-RestMethod -Method Post -Uri 'https://api.example.com/v1/tickets' -Headers $headers `
    -Body $body -ContentType 'application/json; charset=utf-8'
```

Build query strings safely: `[uri]::EscapeDataString($value)` for each value. Never paste user input straight into a URL.

## Paging

APIs return results a page at a time. Each response says how to get the next page: a `nextLink` URL (Microsoft Graph), a `next` cursor, or a `Link` header. Keep going until there's none:

```powershell,norun
function Get-GraphAll([string]$Uri, [hashtable]$Headers) {
    while ($Uri) {
        $page = Invoke-RestMethod -Uri $Uri -Headers $Headers
        $page.value                         # output this page's items
        $Uri = $page.'@odata.nextLink'      # $null on the last page ends the loop
    }
}
```

Forgetting to page is one of the most common bugs in automation: the report "works" in testing with 50 users and silently stops at 100 in production.

## Errors, throttling and retries

APIs fail temporarily: `429 Too Many Requests` (you're being throttled), `503 Service Unavailable`, timeouts. The professional answer is to **retry with exponential backoff**: wait 1 s, then 2 s, then 4 s... and honour a `Retry-After` header when there is one. Errors like `400 Bad Request`, `401` and `404` are not temporary and must not be retried.

```powershell,norun
try {
    Invoke-RestMethod -Uri $uri -Headers $headers
}
catch {
    $status = [int]$_.Exception.Response.StatusCode     # e.g. 404, 429
    if ($status -eq 429) { ... wait and retry ... }
    else { throw }
}
```

## Secrets: never in the script

```powershell,norun
# Once, as the person or service account that runs the script:
Install-Module Microsoft.PowerShell.SecretManagement, Microsoft.PowerShell.SecretStore -Scope CurrentUser
Register-SecretVault -Name Local -ModuleName Microsoft.PowerShell.SecretStore -DefaultVault
Set-Secret -Name ServiceNowToken -Secret (Read-Host -AsSecureString 'Token')

# In the script:
$token = Get-Secret -Name ServiceNowToken -AsPlainText
```

SecretManagement also has vault extensions for Azure Key Vault, KeePass, 1Password and others, so the same `Get-Secret` line works whichever one your company uses. In Azure Automation, use a managed identity and no secret at all.

## Configuration files

Settings that differ per environment (URLs, thresholds, email addresses) belong in a config file, not in the code:

```powershell,norun
# config.psd1: a PowerShell data file, safe to load (it can't run code)
@{
    ApiBase      = 'https://tokyo.service-now.com/api/now'
    DiskWarnPct  = 85
    AlertEmail   = 'it-alerts@contoso.com'
}

$config = Import-PowerShellDataFile -Path "$PSScriptRoot\config.psd1"
```

JSON (`Get-Content config.json -Raw | ConvertFrom-Json`) also works and is easier for other tools to read. Never dot-source a config `.ps1`: that runs whatever someone put in it.

## Logging

A script that runs at 3 a.m. as a scheduled task has no one watching. It needs a log:

```powershell
function Write-Log {
    param([ValidateSet('INFO', 'WARN', 'ERROR')][string]$Level = 'INFO', [Parameter(Mandatory)][string]$Message)
    $line = '{0} {1,-5} {2}' -f (Get-Date -Format 'yyyy-MM-dd HH:mm:ss'), $Level, $Message
    # Add-Content -Path $LogPath -Value $line    # in a real script, append to a file
    $line
}

Write-Log -Message 'Sync started'
Write-Log -Level WARN -Message '3 users have no manager'
```

Also useful: `Start-Transcript` (records everything, good for troubleshooting), the Windows Event Log (`Write-EventLog` / `New-WinEvent`, where monitoring tools already look), and structured JSON logs (one JSON object per line) when logs go to Splunk, Elastic or Azure Monitor.

## Doing many things at once

```powershell,norun
# PowerShell 7: run the script block for up to 10 items at the same time
$servers | ForEach-Object -Parallel {
    [pscustomobject]@{ Server = $_; Up = Test-Connection $_ -Count 1 -Quiet }
} -ThrottleLimit 10

# Windows PowerShell 5.1: background jobs
$jobs = $servers | ForEach-Object { Start-Job -ScriptBlock { param($s) Test-Connection $s -Count 1 -Quiet } -ArgumentList $_ }
$jobs | Wait-Job | Receive-Job
```

@@quiz
A report lists all users from an API, but in production it always shows exactly 100. The most likely cause?
- The API is broken
* The script reads only the first page and doesn't follow the next-page link
- PowerShell arrays are limited to 100 items
- The token expired
? Always loop until the response has no next link or cursor.

@@quiz
Which API errors should a script retry with backoff?
* 429 Too Many Requests and 503 Service Unavailable (temporary)
- 401 Unauthorized
- 404 Not Found
- 400 Bad Request
? Retrying a 400 or 404 will fail the same way every time. Temporary errors deserve a retry; others deserve a clear message.

@@quiz
Where should an API token used by a scheduled script live?
- In the script, on the first line
- In a config.psd1 next to the script
* In a secret store (SecretManagement vault, Key Vault), or use a managed identity
- In an environment variable set in the script itself
? Code and config end up in Git, emails and backups. Secrets go in a vault.

@@quiz
Why use `Import-PowerShellDataFile` for config instead of dot-sourcing `config.ps1`?
- It's faster
* A .psd1 data file can only contain data, so loading it can't run code
- Dot-sourcing doesn't support hashtables
- It encrypts the file
? A config file is often editable by more people than the script. It must not be able to run commands.

@@drill
Which cmdlet calls a REST API and converts the JSON response to objects?
= Invoke-RestMethod
= irm
? Invoke-WebRequest returns the raw response.

@@flashcards
`Invoke-RestMethod` :: calls a REST API; JSON responses become objects
`ConvertTo-Json -Depth` :: object to JSON (default depth is only 2)
`[uri]::EscapeDataString()` :: safely encodes a value for a URL
paging :: follow `nextLink` / cursor until there's none
429 :: Too Many Requests: you're being throttled
exponential backoff :: wait 1, 2, 4, 8... seconds between retries
`Retry-After` :: response header: how long the server wants you to wait
SecretManagement :: `Get-Secret` / `Set-Secret` with pluggable vaults
`Import-PowerShellDataFile` :: loads a .psd1 config safely
`Start-Transcript` :: records a whole session to a file
`ForEach-Object -Parallel` :: PowerShell 7 parallel pipeline

@@exercise Follow every page
`Get-AllItems` receives `-Fetch`, a script block that takes a cursor and returns a page: `@{ items = @(...); next = <cursor or $null> }`. Call it with `$null` first, output every item, and keep calling with the returned `next` until it's `$null`. (In real life `-Fetch` would call `Invoke-RestMethod`; passing it in makes the function testable.)
@@starter
function Get-AllItems {
    param([Parameter(Mandatory)][scriptblock]$Fetch)
    throw "TODO: not done yet"
}

@@tests
$pages = @{
    'start' = @{ items = @('ana', 'ben'); next = 'p2' }
    'p2'    = @{ items = @('chie', 'dan'); next = 'p3' }
    'p3'    = @{ items = @('emi'); next = $null }
}
$calls = [System.Collections.Generic.List[string]]::new()
$fake = { param($cursor) $key = if ($cursor) { $cursor } else { 'start' }; $calls.Add($key); $pages[$key] }
check @(Get-AllItems -Fetch $fake) @('ana', 'ben', 'chie', 'dan', 'emi') "all items from all pages"
check ($calls -join ',') 'start,p2,p3' "each page fetched once"
check @(Get-AllItems -Fetch { param($c) @{ items = @(); next = $null } }).Count 0 "an empty result"
@@hint
`$cursor = $null; do { $page = & $Fetch $cursor; $page.items; $cursor = $page.next } while ($cursor)`.
@@solution
function Get-AllItems {
    param([Parameter(Mandatory)][scriptblock]$Fetch)
    $cursor = $null
    do {
        $page = & $Fetch $cursor
        $page.items
        $cursor = $page.next
    } while ($cursor)
}

@@exercise Retry with backoff
Write `Invoke-WithRetry -Action <scriptblock> -MaxAttempts 4 -BaseDelayMs 100 -Sleep <scriptblock>`. It runs `-Action` and returns its result. If it throws, wait and try again, up to `MaxAttempts` tries in total, doubling the wait each time: 100, 200, 400 ms... After the last failed try, throw the last error again. Wait by calling `& $Sleep <milliseconds>` (the default does `Start-Sleep -Milliseconds`; the tests pass a fake so they run instantly).
@@starter
function Invoke-WithRetry {
    param(
        [Parameter(Mandatory)][scriptblock]$Action,
        [int]$MaxAttempts = 4,
        [int]$BaseDelayMs = 100,
        [scriptblock]$Sleep = { param($ms) Start-Sleep -Milliseconds $ms }
    )
    throw "TODO: not done yet"
}

@@tests
$waits = [System.Collections.Generic.List[int]]::new()
$fakeSleep = { param($ms) $waits.Add($ms) }
$script:tries = 0
$flaky = { $script:tries++; if ($script:tries -lt 3) { throw "503 Service Unavailable" }; "ok after $script:tries" }
check (Invoke-WithRetry -Action $flaky -Sleep $fakeSleep) 'ok after 3' "succeeds on the third try"
check ($waits -join ',') '100,200' "waited 100 then 200 ms"
$waits.Clear()
$message = try { Invoke-WithRetry -Action { throw 'still down' } -MaxAttempts 3 -BaseDelayMs 50 -Sleep $fakeSleep; 'no error' } catch { $_.Exception.Message }
check $message 'still down' "gives up and rethrows"
check ($waits -join ',') '50,100' "no wait after the last try"
@@hint
`for ($attempt = 1; ; $attempt++) { try { return & $Action } catch { if ($attempt -ge $MaxAttempts) { throw }; & $Sleep ($BaseDelayMs * [math]::Pow(2, $attempt - 1)) } }`. A bare `throw` inside `catch` rethrows the current error.
@@solution
function Invoke-WithRetry {
    param(
        [Parameter(Mandatory)][scriptblock]$Action,
        [int]$MaxAttempts = 4,
        [int]$BaseDelayMs = 100,
        [scriptblock]$Sleep = { param($ms) Start-Sleep -Milliseconds $ms }
    )
    for ($attempt = 1; ; $attempt++) {
        try {
            return & $Action
        }
        catch {
            if ($attempt -ge $MaxAttempts) { throw }
            & $Sleep ([int]($BaseDelayMs * [math]::Pow(2, $attempt - 1)))
        }
    }
}
