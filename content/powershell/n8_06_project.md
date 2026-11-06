@@lesson n8-project Project: A User Access Review
@@concepts objects, files, automation
@@text
Auditors (ISO 27001, SOC 2, J-SOX in Japan) ask every quarter: **who has access to what, and should they?** Doing it by hand in Excel takes days. In this project you'll automate it: combine an HR export and an Active Directory export, apply rules, and produce findings and an HTML report for managers to sign off.

## Inputs

- **HR export** (CSV): employee id, name, department, status (active/leaver), leave date, manager.
- **AD export** (CSV): sAMAccountName, employeeID, enabled, last logon, group memberships.

In a real environment:

```powershell,norun
Get-ADUser -Filter * -Properties employeeID, LastLogonDate, MemberOf, Enabled |
    Select-Object SamAccountName, employeeID, Enabled, LastLogonDate,
        @{ n = 'Groups'; e = { ($_.MemberOf | ForEach-Object { ($_ -split ',')[0] -replace '^CN=' }) -join ';' } } |
    Export-Csv ad.csv -NoTypeInformation -Encoding UTF8
```

## Rules (findings)

| Rule | Severity |
|---|---|
| Account of a **leaver** is still enabled | High |
| Enabled account with **no HR record** (orphan) | High |
| Member of a **privileged group** (Domain Admins...) not on the approved list | High |
| Enabled, but no logon for **90+ days** (stale) | Medium |
| Group membership not expected for the department (e.g. Sales in `Finance-Share`) | Low |

Service accounts (`svc-*`) are excluded from HR matching but still checked for privileged groups.

## Design

Use the module's patterns:

- import with `Import-Csv`, convert types immediately (dates with `ParseExact`, booleans from text)
- index HR by employee id in a hashtable (no nested `Where-Object`)
- each rule is a small function that takes an account and returns zero or more finding objects: easy to test one by one, easy to add new rules
- output findings as objects, then format: console summary, CSV for the auditors, HTML for managers

## HTML reports

`ConvertTo-Html` turns objects into a table; add a `<style>` block with `-Head`, and a summary with `-PreContent`. Encode any data you insert into hand-written HTML with `[System.Net.WebUtility]::HtmlEncode()`, because a display name like `<script>` must not become markup.

@@quiz
Why convert CSV fields to proper types right after `Import-Csv`?
* Everything from CSV is text; comparing dates or booleans as strings gives wrong answers
- Import-Csv is slow otherwise
- Export-Csv requires it
- To save memory
? '2026-11-03' -lt '2026-9-01' is a string comparison.

@@quiz
Why HtmlEncode values inserted into a hand-built HTML report?
- To make the file smaller
* So that text like `<script>` in a display name is shown as text, not run as markup
- Browsers require it
- To support Japanese characters
? Reports are opened by managers' browsers: treat data as untrusted.

@@quiz
Which finding is typical in a user access review?
* An enabled account for someone who has left the company
- A user with a long display name
- A user whose password is 14 characters
- A department with many users
? Leavers with active accounts are a classic audit failure.

@@flashcards
access review :: periodic check of who has access
leaver :: person who left the company
orphan account :: account without an HR record
stale account :: no logon for a long time
privileged group :: Domain Admins, Enterprise Admins...
rule function :: account in, findings out
`ConvertTo-Html -Head` :: HTML table with styles
`HtmlEncode` :: escape data for HTML

@@exercise Find the access violations
Implement the rules and print the findings sorted by severity (High, Medium, Low) then account, plus a count per severity:

- parse `LastLogon` (`yyyy-MM-dd`, or empty = never) and `Enabled` (`True`/`False` text)
- today is 2026-11-03; stale = enabled and no logon for more than 90 days (or never)
- approved privileged members: `it-admin`; privileged group: `Domain Admins`
- department-to-allowed-groups map: anything not in the department's list and not `Domain Users` is a Low finding
@@starter
$hrCsv = @'
EmployeeId,Name,Department,Status,LeaveDate
1001,Aiko Sato,IT,active,
1002,Ben Kowalski,Sales,active,
1003,Chen Wei,Finance,leaver,2026-09-30
1004,Dana Morgan,Sales,active,
1005,Emil Nowak,IT,active,
'@
$adCsv = @'
Account,EmployeeId,Enabled,LastLogon,Groups
asato,1001,True,2026-11-02,Domain Users;IT-Share
bkowalski,1002,True,2026-10-30,Domain Users;Sales-Share;Finance-Share
cwei,1003,True,2026-09-29,Domain Users;Finance-Share
dmorgan,1004,True,2026-06-15,Domain Users;Sales-Share
enowak,1005,True,2026-11-01,Domain Users;IT-Share;Domain Admins
it-admin,,True,2026-11-03,Domain Users;Domain Admins
temp01,,True,,Domain Users
svc-backup,,True,2026-11-03,Domain Users;Backup Operators
'@
$allowed = @{ IT = @('IT-Share'); Sales = @('Sales-Share'); Finance = @('Finance-Share') }
'TODO'
@@expected
High   cwei        leaver account still enabled (left 2026-09-30)
High   enowak      unapproved member of Domain Admins
High   temp01      enabled account without HR record
Medium dmorgan     no logon for 141 days
Medium temp01      never logged on
Low    bkowalski   unexpected group Finance-Share for Sales
summary: High 3, Medium 2, Low 1
@@requires
ConvertFrom-Csv
ParseExact
function Test-
@@hint
`$hr = $hrCsv | ConvertFrom-Csv` (same as `Import-Csv` for a string). Index: `$byId = @{}; foreach ($p in $hr) { $byId[$p.EmployeeId] = $p }`. Write one function per rule: `Test-Leaver`, `Test-Orphan` (skip `svc-*` and the approved admins), `Test-Privileged`, `Test-Stale`, `Test-Groups`, each returning `[pscustomobject]@{ Severity; Account; Text }`. Sort with a rank map: `Sort-Object @{ e = { $rank[$_.Severity] } }, Account`. Format with `$f.Severity.PadRight(6) + ' ' + $f.Account.PadRight(11) + ' ' + $f.Text`.
@@solution
$hrCsv = @'
EmployeeId,Name,Department,Status,LeaveDate
1001,Aiko Sato,IT,active,
1002,Ben Kowalski,Sales,active,
1003,Chen Wei,Finance,leaver,2026-09-30
1004,Dana Morgan,Sales,active,
1005,Emil Nowak,IT,active,
'@
$adCsv = @'
Account,EmployeeId,Enabled,LastLogon,Groups
asato,1001,True,2026-11-02,Domain Users;IT-Share
bkowalski,1002,True,2026-10-30,Domain Users;Sales-Share;Finance-Share
cwei,1003,True,2026-09-29,Domain Users;Finance-Share
dmorgan,1004,True,2026-06-15,Domain Users;Sales-Share
enowak,1005,True,2026-11-01,Domain Users;IT-Share;Domain Admins
it-admin,,True,2026-11-03,Domain Users;Domain Admins
temp01,,True,,Domain Users
svc-backup,,True,2026-11-03,Domain Users;Backup Operators
'@
$allowed = @{ IT = @('IT-Share'); Sales = @('Sales-Share'); Finance = @('Finance-Share') }

$today = [datetime]'2026-11-03'
$approvedAdmins = @('it-admin')
$privileged = @('Domain Admins')

$byId = @{}
foreach ($p in ($hrCsv | ConvertFrom-Csv)) { $byId[$p.EmployeeId] = $p }

$accounts = $adCsv | ConvertFrom-Csv | ForEach-Object {
    $logon = $null
    if ($_.LastLogon) { $logon = [datetime]::ParseExact($_.LastLogon, 'yyyy-MM-dd', $null) }
    [pscustomobject]@{
        Account   = $_.Account
        Person    = $(if ($_.EmployeeId) { $byId[$_.EmployeeId] } else { $null })
        Enabled   = $_.Enabled -eq 'True'
        LastLogon = $logon
        Groups    = $_.Groups -split ';'
    }
}

function New-Finding($Severity, $Account, $Text) { [pscustomobject]@{ Severity = $Severity; Account = $Account; Text = $Text } }

function Test-Leaver($a) {
    if ($a.Enabled -and $a.Person -and $a.Person.Status -eq 'leaver') {
        New-Finding High $a.Account "leaver account still enabled (left $($a.Person.LeaveDate))"
    }
}
function Test-Orphan($a) {
    if ($a.Enabled -and -not $a.Person -and $a.Account -notlike 'svc-*' -and $a.Account -notin $approvedAdmins) {
        New-Finding High $a.Account 'enabled account without HR record'
    }
}
function Test-Privileged($a) {
    foreach ($g in $a.Groups) {
        if ($g -in $privileged -and $a.Account -notin $approvedAdmins) {
            New-Finding High $a.Account "unapproved member of $g"
        }
    }
}
function Test-Stale($a) {
    if (-not $a.Enabled) { return }
    if ($null -eq $a.LastLogon) { New-Finding Medium $a.Account 'never logged on'; return }
    $days = ($today - $a.LastLogon).Days
    if ($days -gt 90) { New-Finding Medium $a.Account "no logon for $days days" }
}
function Test-Groups($a) {
    if (-not $a.Person) { return }
    $ok = $allowed[$a.Person.Department]
    foreach ($g in $a.Groups) {
        if ($g -ne 'Domain Users' -and $g -notin $ok -and $g -notin $privileged) {
            New-Finding Low $a.Account "unexpected group $g for $($a.Person.Department)"
        }
    }
}

$findings = foreach ($a in $accounts) {
    Test-Leaver $a
    Test-Orphan $a
    Test-Privileged $a
    Test-Stale $a
    Test-Groups $a
}
$rank = @{ High = 0; Medium = 1; Low = 2 }
$findings = $findings | Sort-Object @{ Expression = { $rank[$_.Severity] } }, Account
foreach ($f in $findings) { $f.Severity.PadRight(6) + ' ' + $f.Account.PadRight(11) + ' ' + $f.Text }
$counts = foreach ($s in 'High', 'Medium', 'Low') { "$s $(@($findings | Where-Object Severity -eq $s).Count)" }
"summary: $($counts -join ', ')"

@@exercise The manager's HTML report
Produce the report from a list of findings (given in the starter):

- a summary paragraph and a table with `ConvertTo-Html -Fragment`, inside a full page built with a here-string (title, `<style>`, generated-on date)
- encode the free-text fields with `[System.Net.WebUtility]::HtmlEncode()` **before** building the table: one finding contains a malicious display name
- add a CSS class to High rows by post-processing the fragment (`-replace '<tr><td>High', '<tr class="high"><td>High'`)
- write the file as UTF-8, then print checks: number of table rows, whether the raw `<script>` tag appears, the number of `high` rows, and the `<title>`
@@starter
$findings = @(
    [pscustomobject]@{ Severity = 'High'; Account = 'cwei'; Text = 'leaver account still enabled' }
    [pscustomobject]@{ Severity = 'High'; Account = 'temp01'; Text = 'display name: <script>alert(1)</script>' }
    [pscustomobject]@{ Severity = 'Medium'; Account = 'dmorgan'; Text = 'no logon for 141 days' }
)
$path = Join-Path ([System.IO.Path]::GetTempPath()) "review-$PID.html"
'TODO'
Remove-Item $path -ErrorAction SilentlyContinue
@@expected
table rows: 3
raw script tag present: False
encoded script present: True
high rows: 2
title: Access review Q4 2026
@@requires
ConvertTo-Html
HtmlEncode
-Fragment
@@hint
Encode first: `$safe = $findings | Select-Object Severity, Account, @{ n = 'Finding'; e = { [System.Net.WebUtility]::HtmlEncode($_.Text) } }`. Depending on the version, `ConvertTo-Html` may encode the text again (`&amp;lt;`), so count either form as encoded: `-match '&(amp;)?lt;script'`. Count rows with `([regex]::Matches($html, '<tr( class="high")?><td>')).Count`.
@@solution
$findings = @(
    [pscustomobject]@{ Severity = 'High'; Account = 'cwei'; Text = 'leaver account still enabled' }
    [pscustomobject]@{ Severity = 'High'; Account = 'temp01'; Text = 'display name: <script>alert(1)</script>' }
    [pscustomobject]@{ Severity = 'Medium'; Account = 'dmorgan'; Text = 'no logon for 141 days' }
)
$path = Join-Path ([System.IO.Path]::GetTempPath()) "review-$PID.html"

$title = 'Access review Q4 2026'
$safe = $findings | Select-Object Severity, Account, @{ Name = 'Finding'; Expression = { [System.Net.WebUtility]::HtmlEncode($_.Text) } }
$table = ($safe | ConvertTo-Html -Fragment) -join "`n"
$table = $table -replace '<tr><td>High', '<tr class="high"><td>High'
$high = @($findings | Where-Object Severity -eq 'High').Count

$html = @"
<!DOCTYPE html>
<html><head><meta charset="utf-8"><title>$title</title>
<style>
body { font-family: Segoe UI, sans-serif; }
table { border-collapse: collapse; }
td, th { border: 1px solid #ccc; padding: 4px 8px; }
tr.high td { background: #fde2e2; }
</style></head>
<body>
<h1>$title</h1>
<p>$($findings.Count) findings, $high high severity. Generated 2026-11-03.</p>
$table
</body></html>
"@
[System.IO.File]::WriteAllText($path, $html, [System.Text.UTF8Encoding]::new($false))

$saved = [System.IO.File]::ReadAllText($path)
"table rows: $(([regex]::Matches($saved, '<tr( class=""high"")?><td>')).Count)"
"raw script tag present: $($saved.Contains('<script>'))"
"encoded script present: $($saved -match '&(amp;)?lt;script&(amp;)?gt;')"
"high rows: $(([regex]::Matches($saved, '<tr class=""high"">')).Count)"
"title: $(([regex]::Match($saved, '<title>(.*?)</title>')).Groups[1].Value)"
Remove-Item $path -ErrorAction SilentlyContinue
