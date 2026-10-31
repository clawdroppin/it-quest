@@lesson q2-onboarding User Onboarding Automation
@@text
Monday morning, HR sends a spreadsheet: twelve new people start next week. Each needs an account, an email address, the right groups, a temporary password, and a welcome letter. By hand that's an afternoon of clicking, with typos. With a script it's ten seconds, and identical every time.

In this build you'll write **Start-Onboarding.ps1**:

```text
.\Start-Onboarding.ps1 -CsvPath new_hires.csv -Domain contoso.com            # plan only: shows what it would do
.\Start-Onboarding.ps1 -CsvPath new_hires.csv -Domain contoso.com -Apply     # writes letters and the plan
```

It reads `new_hires.csv` (as HR sends it), and for each person:

1. creates a **unique username**: first initial + last name, lowercase, letters only (`Ana Silva` → `asilva`; if `asilva` already exists, `asilva2`),
2. builds the **UPN** (`asilva@contoso.com`), the **OU** for their department, and their **groups**,
3. generates a **random temporary password** with a cryptographic random number generator,
4. writes a **welcome letter** and an `onboarding_plan.csv` for the records.

The real `New-ADUser` call is in the script but only runs when the ActiveDirectory module is present, so you can use it at home today and at work later. Default is a **dry run**: nothing is changed without `-Apply`. That's how professionals write anything that creates accounts.

@@project onboarding User Onboarding
Write the functions in `Onboarding.ps1` until **▶ tests.ps1** prints `All tests passed`, then try **▶ Start-Onboarding.ps1** (a dry run).
@@file Onboarding.ps1
# Onboarding helpers: pure functions, tested by tests.ps1.

$DepartmentGroups = @{
    'Sales'   = @('All-Staff', 'Sales-Team', 'CRM-Users')
    'IT'      = @('All-Staff', 'IT-Team', 'VPN-Users')
    'Finance' = @('All-Staff', 'Finance-Team', 'ERP-Users')
}

function New-Username {
    # First letter of $First + $Last, lowercase, letters a-z only (drop spaces, dashes, apostrophes).
    # If the name is in $Existing (any case), add 2, then 3, ... until it is free.
    param([string]$First, [string]$Last, [string[]]$Existing = @())
    throw "TODO: not done yet"
}

function Get-RandomIndex([int]$Max) {
    # A cryptographically random whole number from 0 to $Max - 1. (Get-Random is NOT meant for passwords.)
    $bytes = New-Object byte[] 4
    [System.Security.Cryptography.RandomNumberGenerator]::Create().GetBytes($bytes)
    return [int]([BitConverter]::ToUInt32($bytes, 0) % $Max)
}

function New-TempPassword {
    # A random password of $Length characters with at least one uppercase letter, one lowercase
    # letter, one digit and one symbol from the sets below. Use Get-RandomIndex to pick characters.
    # Tip: pick one from each set first, fill the rest from all sets, then shuffle.
    param([int]$Length = 14)
    $upper = 'ABCDEFGHJKLMNPQRSTUVWXYZ'   # no I or O: easy to read out over the phone
    $lower = 'abcdefghijkmnpqrstuvwxyz'   # no l
    $digit = '23456789'                   # no 0 or 1
    $symbol = '!#$%*+-=?@'
    throw "TODO: not done yet"
}

function Get-OUPath([string]$Dept, [string]$Domain) {
    # "Sales", "contoso.com" -> "OU=Sales,OU=Staff,DC=contoso,DC=com"
    throw "TODO: not done yet"
}

function ConvertTo-OnboardingPlan {
    # One plan object per CSV row, with: DisplayName ("First Last"), Username, UPN (username@domain),
    # Department, OU, Groups (joined with ";"), Manager, StartDate.
    # Unknown departments get only "All-Staff". Usernames must be unique, also among each other.
    param($Rows, [string]$Domain, [string[]]$Existing = @())
    throw "TODO: not done yet"
}
@@file tests.ps1
# Run: powershell -ExecutionPolicy Bypass -File tests.ps1
. "$PSScriptRoot\Onboarding.ps1"
$script:failed = 0

function Test-That([string]$Name, [scriptblock]$Body) {
    try {
        $result = & $Body
        if ($result -eq $true) { Write-Host "  ok    $Name" -ForegroundColor Green }
        else { Write-Host "  FAIL  $Name" -ForegroundColor Red; $script:failed++ }
    }
    catch {
        Write-Host "  FAIL  $Name : $($_.Exception.Message)" -ForegroundColor Red
        $script:failed++
    }
}

Write-Host 'Usernames'
Test-That 'simple' { (New-Username -First 'Ana' -Last 'Silva') -ceq 'asilva' }
Test-That 'cleans the name' { (New-Username -First 'Sean' -Last "O'Brien-Smith") -ceq 'sobriensmith' }
Test-That 'unique' { (New-Username -First 'Ana' -Last 'Silva' -Existing 'ASILVA', 'asilva2') -ceq 'asilva3' }

Write-Host 'Passwords'
$pws = 1..50 | ForEach-Object { New-TempPassword }
Test-That 'default length 14' { @($pws | Where-Object Length -ne 14).Count -eq 0 }
Test-That 'custom length' { (New-TempPassword -Length 20).Length -eq 20 }
Test-That 'always has all four kinds' {
    @($pws | Where-Object { $_ -cnotmatch '[A-Z]' -or $_ -cnotmatch '[a-z]' -or $_ -notmatch '\d' -or $_ -notmatch '[!#$%*+\-=?@]' }).Count -eq 0
}
Test-That 'no look-alike characters' { @($pws | Where-Object { $_ -cmatch '[IOl01]' }).Count -eq 0 }
Test-That 'random' { @($pws | Select-Object -Unique).Count -eq 50 }

Write-Host 'Plan'
Test-That 'OU path' { (Get-OUPath -Dept 'Sales' -Domain 'tokyo.contoso.com') -eq 'OU=Sales,OU=Staff,DC=tokyo,DC=contoso,DC=com' }
$rows = @'
FirstName,LastName,Department,Manager,StartDate
Ana,Silva,Sales,mlee,2026-11-02
Kenji,Tanaka,IT,rchen,2026-11-02
Anton,Silva,Marketing,mlee,2026-11-09
'@ | ConvertFrom-Csv
$plan = @(ConvertTo-OnboardingPlan -Rows $rows -Domain 'contoso.com' -Existing 'ktanaka')
Test-That 'one entry per row' { $plan.Count -eq 3 }
Test-That 'first entry' {
    $p = $plan[0]
    $p.DisplayName -eq 'Ana Silva' -and $p.Username -eq 'asilva' -and $p.UPN -eq 'asilva@contoso.com' -and
    $p.OU -eq 'OU=Sales,OU=Staff,DC=contoso,DC=com' -and $p.Groups -eq 'All-Staff;Sales-Team;CRM-Users' -and
    $p.Manager -eq 'mlee' -and $p.StartDate -eq '2026-11-02'
}
Test-That 'existing names are avoided' { $plan[1].Username -eq 'ktanaka2' }
Test-That 'names in the same file are avoided' { $plan[2].Username -eq 'asilva2' }
Test-That 'unknown department' { $plan[2].Groups -eq 'All-Staff' }

if ($script:failed -gt 0) { Write-Host "$script:failed test(s) failed" -ForegroundColor Red; exit 1 }
Write-Host 'All tests passed' -ForegroundColor Green
@@file new_hires.csv
FirstName,LastName,Department,Manager,StartDate
Ana,Silva,Sales,mlee,2026-11-02
Kenji,Tanaka,IT,rchen,2026-11-02
Sean,O'Brien,Finance,bjones,2026-11-02
Mia,Lee,IT,rchen,2026-11-09
Anton,Silva,Sales,mlee,2026-11-09
@@file Start-Onboarding.ps1
<#
.SYNOPSIS
Plans and performs onboarding for new hires from an HR CSV file.
.DESCRIPTION
Without -Apply this is a dry run: it only shows the plan. With -Apply it writes a welcome
letter per person and onboarding_plan.csv, and creates the AD accounts if the
ActiveDirectory module is available.
.EXAMPLE
.\Start-Onboarding.ps1 -CsvPath new_hires.csv -Domain contoso.com -Apply
#>
[CmdletBinding()]
param(
    [string]$CsvPath = (Join-Path $PSScriptRoot 'new_hires.csv'),
    [string]$Domain = 'contoso.com',
    [string]$OutDir = (Join-Path $PSScriptRoot 'output'),
    [switch]$Apply
)
$ErrorActionPreference = 'Stop'
. "$PSScriptRoot\Onboarding.ps1"

$haveAD = [bool](Get-Module -ListAvailable -Name ActiveDirectory)
$existing = @()
if ($haveAD) {
    Import-Module ActiveDirectory
    $existing = @(Get-ADUser -Filter * | ForEach-Object { $_.SamAccountName })
}

$plan = @(ConvertTo-OnboardingPlan -Rows (Import-Csv $CsvPath) -Domain $Domain -Existing $existing)
$plan | Format-Table DisplayName, Username, Department, Groups, StartDate -AutoSize

if (-not $Apply) {
    Write-Host "Dry run: nothing was changed. Run again with -Apply to write letters$(if ($haveAD) { ' and create accounts' })." -ForegroundColor Yellow
    exit 0
}

New-Item -ItemType Directory -Path $OutDir -Force | Out-Null
foreach ($p in $plan) {
    $password = New-TempPassword
    if ($haveAD) {
        New-ADUser -Name $p.DisplayName -SamAccountName $p.Username -UserPrincipalName $p.UPN -Path $p.OU `
            -Department $p.Department -AccountPassword (ConvertTo-SecureString $password -AsPlainText -Force) `
            -ChangePasswordAtLogon $true -Enabled $true
        foreach ($g in $p.Groups -split ';') { Add-ADGroupMember -Identity $g -Members $p.Username }
    }
    $letter = @"
Welcome to the team, $($p.DisplayName)!

Your first day: $($p.StartDate)
Username:       $($p.Username)
Email:          $($p.UPN)
Temporary password (you must change it when you first sign in): $password

Your manager is $($p.Manager). The IT help desk is at extension 4357 (H-E-L-P).
"@
    Set-Content -Path (Join-Path $OutDir "welcome_$($p.Username).txt") -Value $letter -Encoding UTF8
    Write-Host "Prepared $($p.Username)"
}
$plan | Export-Csv -Path (Join-Path $OutDir 'onboarding_plan.csv') -NoTypeInformation -Encoding UTF8
Write-Host "Done: $($plan.Count) people. Letters and the plan are in $OutDir"
Write-Host 'Hand letters over in person or by a secure channel, and delete them afterwards: they contain passwords.' -ForegroundColor Yellow
@@text
> **Shuffling** in PowerShell 5.1: `$chars | Sort-Object { Get-RandomIndex 1000000 }` sorts the characters by a random key. And to turn an array of characters back into a string: `-join $chars`.

@@project onboarding_finished User Onboarding: the finished version
The complete project. Extensions that real companies use: **offboarding** (disable the account, remove groups, move it to a "Leavers" OU, export the group memberships first), a check that the manager exists in AD, creating the Microsoft 365 mailbox and licence with Microsoft Graph, or posting a summary to a Teams channel.
@@file Onboarding.ps1
# Onboarding helpers: pure functions, tested by tests.ps1.

$DepartmentGroups = @{
    'Sales'   = @('All-Staff', 'Sales-Team', 'CRM-Users')
    'IT'      = @('All-Staff', 'IT-Team', 'VPN-Users')
    'Finance' = @('All-Staff', 'Finance-Team', 'ERP-Users')
}

function New-Username {
    # First letter of $First + $Last, lowercase, letters a-z only (drop spaces, dashes, apostrophes).
    # If the name is in $Existing (any case), add 2, then 3, ... until it is free.
    param([string]$First, [string]$Last, [string[]]$Existing = @())
    $base = ("$($First.Trim()[0])$Last".ToLower()) -replace '[^a-z]', ''
    $name = $base
    $n = 2
    while ($name -in $Existing) {
        $name = "$base$n"
        $n++
    }
    return $name
}

function Get-RandomIndex([int]$Max) {
    # A cryptographically random whole number from 0 to $Max - 1. (Get-Random is NOT meant for passwords.)
    $bytes = New-Object byte[] 4
    [System.Security.Cryptography.RandomNumberGenerator]::Create().GetBytes($bytes)
    return [int]([BitConverter]::ToUInt32($bytes, 0) % $Max)
}

function New-TempPassword {
    # A random password of $Length characters with at least one uppercase letter, one lowercase
    # letter, one digit and one symbol from the sets below. Use Get-RandomIndex to pick characters.
    # Tip: pick one from each set first, fill the rest from all sets, then shuffle.
    param([int]$Length = 14)
    $upper = 'ABCDEFGHJKLMNPQRSTUVWXYZ'   # no I or O: easy to read out over the phone
    $lower = 'abcdefghijkmnpqrstuvwxyz'   # no l
    $digit = '23456789'                   # no 0 or 1
    $symbol = '!#$%*+-=?@'
    $all = $upper + $lower + $digit + $symbol
    $chars = @(foreach ($set in $upper, $lower, $digit, $symbol) { $set[(Get-RandomIndex $set.Length)] })
    while ($chars.Count -lt $Length) {
        $chars += $all[(Get-RandomIndex $all.Length)]
    }
    $shuffled = $chars | Sort-Object { Get-RandomIndex 1000000 }
    return -join $shuffled
}

function Get-OUPath([string]$Dept, [string]$Domain) {
    # "Sales", "contoso.com" -> "OU=Sales,OU=Staff,DC=contoso,DC=com"
    $dc = ($Domain.Split('.') | ForEach-Object { "DC=$_" }) -join ','
    return "OU=$Dept,OU=Staff,$dc"
}

function ConvertTo-OnboardingPlan {
    # One plan object per CSV row, with: DisplayName ("First Last"), Username, UPN (username@domain),
    # Department, OU, Groups (joined with ";"), Manager, StartDate.
    # Unknown departments get only "All-Staff". Usernames must be unique, also among each other.
    param($Rows, [string]$Domain, [string[]]$Existing = @())
    $taken = [System.Collections.Generic.List[string]]::new()
    foreach ($e in $Existing) { $taken.Add($e) }
    foreach ($row in $Rows) {
        $user = New-Username -First $row.FirstName -Last $row.LastName -Existing $taken
        $taken.Add($user)
        $groups = if ($DepartmentGroups.ContainsKey($row.Department)) { $DepartmentGroups[$row.Department] } else { @('All-Staff') }
        [pscustomobject]@{
            DisplayName = "$($row.FirstName) $($row.LastName)"
            Username    = $user
            UPN         = "$user@$Domain"
            Department  = $row.Department
            OU          = Get-OUPath -Dept $row.Department -Domain $Domain
            Groups      = $groups -join ';'
            Manager     = $row.Manager
            StartDate   = $row.StartDate
        }
    }
}
@@file tests.ps1
# Run: powershell -ExecutionPolicy Bypass -File tests.ps1
. "$PSScriptRoot\Onboarding.ps1"
$script:failed = 0

function Test-That([string]$Name, [scriptblock]$Body) {
    try {
        $result = & $Body
        if ($result -eq $true) { Write-Host "  ok    $Name" -ForegroundColor Green }
        else { Write-Host "  FAIL  $Name" -ForegroundColor Red; $script:failed++ }
    }
    catch {
        Write-Host "  FAIL  $Name : $($_.Exception.Message)" -ForegroundColor Red
        $script:failed++
    }
}

Write-Host 'Usernames'
Test-That 'simple' { (New-Username -First 'Ana' -Last 'Silva') -ceq 'asilva' }
Test-That 'cleans the name' { (New-Username -First 'Sean' -Last "O'Brien-Smith") -ceq 'sobriensmith' }
Test-That 'unique' { (New-Username -First 'Ana' -Last 'Silva' -Existing 'ASILVA', 'asilva2') -ceq 'asilva3' }

Write-Host 'Passwords'
$pws = 1..50 | ForEach-Object { New-TempPassword }
Test-That 'default length 14' { @($pws | Where-Object Length -ne 14).Count -eq 0 }
Test-That 'custom length' { (New-TempPassword -Length 20).Length -eq 20 }
Test-That 'always has all four kinds' {
    @($pws | Where-Object { $_ -cnotmatch '[A-Z]' -or $_ -cnotmatch '[a-z]' -or $_ -notmatch '\d' -or $_ -notmatch '[!#$%*+\-=?@]' }).Count -eq 0
}
Test-That 'no look-alike characters' { @($pws | Where-Object { $_ -cmatch '[IOl01]' }).Count -eq 0 }
Test-That 'random' { @($pws | Select-Object -Unique).Count -eq 50 }

Write-Host 'Plan'
Test-That 'OU path' { (Get-OUPath -Dept 'Sales' -Domain 'tokyo.contoso.com') -eq 'OU=Sales,OU=Staff,DC=tokyo,DC=contoso,DC=com' }
$rows = @'
FirstName,LastName,Department,Manager,StartDate
Ana,Silva,Sales,mlee,2026-11-02
Kenji,Tanaka,IT,rchen,2026-11-02
Anton,Silva,Marketing,mlee,2026-11-09
'@ | ConvertFrom-Csv
$plan = @(ConvertTo-OnboardingPlan -Rows $rows -Domain 'contoso.com' -Existing 'ktanaka')
Test-That 'one entry per row' { $plan.Count -eq 3 }
Test-That 'first entry' {
    $p = $plan[0]
    $p.DisplayName -eq 'Ana Silva' -and $p.Username -eq 'asilva' -and $p.UPN -eq 'asilva@contoso.com' -and
    $p.OU -eq 'OU=Sales,OU=Staff,DC=contoso,DC=com' -and $p.Groups -eq 'All-Staff;Sales-Team;CRM-Users' -and
    $p.Manager -eq 'mlee' -and $p.StartDate -eq '2026-11-02'
}
Test-That 'existing names are avoided' { $plan[1].Username -eq 'ktanaka2' }
Test-That 'names in the same file are avoided' { $plan[2].Username -eq 'asilva2' }
Test-That 'unknown department' { $plan[2].Groups -eq 'All-Staff' }

if ($script:failed -gt 0) { Write-Host "$script:failed test(s) failed" -ForegroundColor Red; exit 1 }
Write-Host 'All tests passed' -ForegroundColor Green
@@file new_hires.csv
FirstName,LastName,Department,Manager,StartDate
Ana,Silva,Sales,mlee,2026-11-02
Kenji,Tanaka,IT,rchen,2026-11-02
Sean,O'Brien,Finance,bjones,2026-11-02
Mia,Lee,IT,rchen,2026-11-09
Anton,Silva,Sales,mlee,2026-11-09
@@file Start-Onboarding.ps1
<#
.SYNOPSIS
Plans and performs onboarding for new hires from an HR CSV file.
.DESCRIPTION
Without -Apply this is a dry run: it only shows the plan. With -Apply it writes a welcome
letter per person and onboarding_plan.csv, and creates the AD accounts if the
ActiveDirectory module is available.
.EXAMPLE
.\Start-Onboarding.ps1 -CsvPath new_hires.csv -Domain contoso.com -Apply
#>
[CmdletBinding()]
param(
    [string]$CsvPath = (Join-Path $PSScriptRoot 'new_hires.csv'),
    [string]$Domain = 'contoso.com',
    [string]$OutDir = (Join-Path $PSScriptRoot 'output'),
    [switch]$Apply
)
$ErrorActionPreference = 'Stop'
. "$PSScriptRoot\Onboarding.ps1"

$haveAD = [bool](Get-Module -ListAvailable -Name ActiveDirectory)
$existing = @()
if ($haveAD) {
    Import-Module ActiveDirectory
    $existing = @(Get-ADUser -Filter * | ForEach-Object { $_.SamAccountName })
}

$plan = @(ConvertTo-OnboardingPlan -Rows (Import-Csv $CsvPath) -Domain $Domain -Existing $existing)
$plan | Format-Table DisplayName, Username, Department, Groups, StartDate -AutoSize

if (-not $Apply) {
    Write-Host "Dry run: nothing was changed. Run again with -Apply to write letters$(if ($haveAD) { ' and create accounts' })." -ForegroundColor Yellow
    exit 0
}

New-Item -ItemType Directory -Path $OutDir -Force | Out-Null
foreach ($p in $plan) {
    $password = New-TempPassword
    if ($haveAD) {
        New-ADUser -Name $p.DisplayName -SamAccountName $p.Username -UserPrincipalName $p.UPN -Path $p.OU `
            -Department $p.Department -AccountPassword (ConvertTo-SecureString $password -AsPlainText -Force) `
            -ChangePasswordAtLogon $true -Enabled $true
        foreach ($g in $p.Groups -split ';') { Add-ADGroupMember -Identity $g -Members $p.Username }
    }
    $letter = @"
Welcome to the team, $($p.DisplayName)!

Your first day: $($p.StartDate)
Username:       $($p.Username)
Email:          $($p.UPN)
Temporary password (you must change it when you first sign in): $password

Your manager is $($p.Manager). The IT help desk is at extension 4357 (H-E-L-P).
"@
    Set-Content -Path (Join-Path $OutDir "welcome_$($p.Username).txt") -Value $letter -Encoding UTF8
    Write-Host "Prepared $($p.Username)"
}
$plan | Export-Csv -Path (Join-Path $OutDir 'onboarding_plan.csv') -NoTypeInformation -Encoding UTF8
Write-Host "Done: $($plan.Count) people. Letters and the plan are in $OutDir"
Write-Host 'Hand letters over in person or by a secure channel, and delete them afterwards: they contain passwords.' -ForegroundColor Yellow
@@text
🎉 Account automation is one of the most valued skills on a help desk or sysadmin team. Talk about the **dry run by default**, the **cryptographic random passwords**, and the **unique-name handling** in interviews. Those details show you think about safety, not just speed.
