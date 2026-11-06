@@lesson n8-testing Testing with Pester: Describe, It, Mocks
@@concepts testing, functions
@@text
**Pester** is PowerShell's test framework, used by Microsoft and almost every public module. Windows ships an old Pester 3.4; install the current version (5.x) with `Install-Module Pester -Force -SkipPublisherCheck` and use it in CI.

## Anatomy of a Pester 5 test file

```powershell,norun
# Get-DiskReport.Tests.ps1
BeforeAll {
    . $PSScriptRoot/Get-DiskReport.ps1            # load the code under test
}

Describe 'Get-DiskReport' {
    Context 'when a disk is nearly full' {
        BeforeEach {
            Mock Get-CimInstance {
                [pscustomobject]@{ DeviceID = 'C:'; Size = 100GB; FreeSpace = 5GB }
            }
        }
        It 'flags the disk as critical' {
            $r = Get-DiskReport
            $r.Status | Should -Be 'Critical'
        }
        It 'asks CIM exactly once' {
            Get-DiskReport | Out-Null
            Should -Invoke Get-CimInstance -Times 1 -Exactly
        }
    }
    It 'rejects a negative threshold' {
        { Get-DiskReport -Threshold -1 } | Should -Throw '*threshold*'
    }
    It 'computes percent for <Free> of <Size>' -TestCases @(
        @{ Size = 100; Free = 25; Expected = 75 }
        @{ Size = 200; Free = 200; Expected = 0 }
    ) {
        Get-UsedPercent -Size $Size -Free $Free | Should -Be $Expected
    }
}
```

Run with `Invoke-Pester -Output Detailed`, or `Invoke-Pester -CI` in a pipeline (writes NUnit XML and sets the exit code).

## Should assertions you'll use

| Assertion | Checks |
|---|---|
| `Should -Be 5` / `-BeExactly 'A'` | equality (case-insensitive / sensitive) |
| `Should -BeTrue`, `-BeNullOrEmpty` | truthiness, empty |
| `Should -Match 'regex'`, `-BeLike 'x*'` | patterns |
| `Should -Contain 'x'`, `-HaveCount 3` | collections |
| `Should -BeOfType [int]` | types |
| `Should -Throw -ErrorId 'X'` | errors (pass a script block) |
| `Should -Invoke Cmd -Times 2 -ParameterFilter { $Name -eq 'x' }` | mocks were called |
| `Should -Not -Be ...` | negation |

## Mocking

`Mock Restart-Service { }` replaces the command **inside the scope of the test**: your function calls the mock instead of restarting anything. With `-ParameterFilter`, different calls can get different answers. `-ModuleName` mocks commands called from inside a module.

Mock the **boundaries** (`Get-ADUser`, `Invoke-RestMethod`, `Get-CimInstance`, `Send-MailMessage`), not your own logic.

## TestDrive and TestRegistry

`TestDrive:` is a temporary folder created per `Describe` and deleted afterwards: perfect for functions that read or write files. `$TestDrive` holds its real path.

## What to test

- the logic: calculations, decisions, parsing (fast, many cases with `-TestCases`)
- the contract: output types, properties, errors and `-WhatIf` behaviour
- the integration points through mocks: "it calls Disable-ADAccount for each stale user, and only those"

## How Pester works

`Describe` and `It` are just functions that take script blocks; `Should` compares and **throws** on failure; `It` catches the exception and records a failure; `Mock` defines a function with the same name (functions win over cmdlets). The exercises rebuild this to make the model concrete.

@@quiz
What does `Mock Restart-Service { }` do in a Pester test?
* Replaces Restart-Service with an empty fake for that test scope, so nothing is really restarted
- Restarts the service in a sandbox
- Disables the cmdlet permanently
- Logs calls to a file
? Combine with Should -Invoke to check it was called.

@@quiz
How do you assert that a call fails?
- `Get-Thing | Should -Throw`
* `{ Get-Thing } | Should -Throw`
- `Should -Throw Get-Thing`
- `Get-Thing -ErrorAction Stop`
? Pass a script block so Pester can run it inside a try.

@@quiz
What is `TestDrive:`?
- The C: drive in test mode
* A temporary folder Pester creates per Describe block and removes afterwards
- A network share
- A registry hive
? Use it for file-handling tests.

@@drill
Which Pester command replaces a real command with a fake one during a test?
= Mock
? Mock Get-Service { ... }.

@@flashcards
`Describe` / `Context` / `It` :: group / sub-group / test
`BeforeAll` / `BeforeEach` :: setup
`Should -Be` :: equality assertion
`Should -Throw` :: expects an error
`Mock Cmd { }` :: fake a command
`Should -Invoke` :: assert a mock was called
`-ParameterFilter` :: match specific calls
`-TestCases` :: data-driven tests
`TestDrive:` :: temporary test folder
`Invoke-Pester -CI` :: CI mode with exit code

@@exercise Build a mini Pester
Write a tiny framework with the same shape as Pester:

- `Describe NAME { }` prints the name and runs the block
- `It NAME { }` runs the block in `try`; on success prints `  [+] NAME`, on failure `  [-] NAME: MESSAGE` and counts it
- `Should-Be EXPECTED` (pipeline input) and `Should-Throw` (pipeline input is a script block) throw on failure with messages like `expected 75 but got 70`
- at the end, print `Tests: N passed, M failed`

Test `Get-UsedPercent` including one deliberately wrong expectation.
@@starter
function Get-UsedPercent {
    param([double]$Size, [double]$Free)
    if ($Size -le 0) { throw 'size must be positive' }
    [int][math]::Round(100 * ($Size - $Free) / $Size)
}
'TODO'
@@expected
Get-UsedPercent
  [+] computes 75 for 25 free of 100
  [+] computes 0 for an empty disk
  [+] rounds 66.67 to 67
  [+] rejects a zero size
  [-] is wrong on purpose: expected 50 but got 30
Tests: 4 passed, 1 failed
@@requires
function Describe
function It
function Should-Be
[scriptblock]
@@hint
Keep counters in a script-level hashtable: `$script:results = @{ Passed = 0; Failed = 0 }`. `function Should-Be { param([Parameter(ValueFromPipeline)]$Actual, [Parameter(Position = 0)]$Expected) process { if ($Actual -ne $Expected) { throw "expected $Expected but got $Actual" } } }`. `Should-Throw` runs the block in `try`; if nothing was thrown, it throws `expected an error`.
@@solution
function Get-UsedPercent {
    param([double]$Size, [double]$Free)
    if ($Size -le 0) { throw 'size must be positive' }
    [int][math]::Round(100 * ($Size - $Free) / $Size)
}

$script:results = @{ Passed = 0; Failed = 0 }

function Describe([string]$Name, [scriptblock]$Body) {
    $Name
    & $Body
}

function It([string]$Name, [scriptblock]$Body) {
    try {
        & $Body
        "  [+] $Name"
        $script:results.Passed++
    } catch {
        "  [-] ${Name}: $($_.Exception.Message)"
        $script:results.Failed++
    }
}

function Should-Be {
    param([Parameter(ValueFromPipeline)]$Actual, [Parameter(Position = 0)]$Expected)
    process {
        if ($Actual -ne $Expected) { throw "expected $Expected but got $Actual" }
    }
}

function Should-Throw {
    param([Parameter(ValueFromPipeline)][scriptblock]$Action)
    process {
        $threw = $false
        try { & $Action } catch { $threw = $true }
        if (-not $threw) { throw 'expected an error' }
    }
}

Describe 'Get-UsedPercent' {
    It 'computes 75 for 25 free of 100' { Get-UsedPercent -Size 100 -Free 25 | Should-Be 75 }
    It 'computes 0 for an empty disk' { Get-UsedPercent -Size 200 -Free 200 | Should-Be 0 }
    It 'rounds 66.67 to 67' { Get-UsedPercent -Size 3 -Free 1 | Should-Be 67 }
    It 'rejects a zero size' { { Get-UsedPercent -Size 0 -Free 0 } | Should-Throw }
    It 'is wrong on purpose' { Get-UsedPercent -Size 10 -Free 7 | Should-Be 50 }
}
"Tests: $($script:results.Passed) passed, $($script:results.Failed) failed"

@@exercise Mock the boundaries
`Disable-StaleAccount` disables accounts that haven't logged in for 90 days. It calls `Get-ADUserList` (the boundary that would query Active Directory) and `Disable-Account` (the one that changes things). Test it without a domain:

- write a `Mock NAME BODY` helper that defines a global function with that name, and records each call's arguments in `$script:calls[NAME]`
- mock `Get-ADUserList` to return test users, and `Disable-Account` to do nothing
- assert: exactly the stale, enabled users were disabled; nobody is disabled when all are fresh; `-WhatIf` disables nobody
- print each assertion's result
@@starter
function Disable-StaleAccount {
    [CmdletBinding(SupportsShouldProcess)]
    param([datetime]$Now, [int]$Days = 90)
    foreach ($u in Get-ADUserList) {
        if ($u.Enabled -and ($Now - $u.LastLogon).Days -gt $Days) {
            if ($PSCmdlet.ShouldProcess($u.Name, 'disable')) { Disable-Account -Name $u.Name }
        }
    }
}
$now = [datetime]'2026-11-03'
'TODO'
@@expected
stale users disabled: asato, dmorgan -> PASS
fresh users only: 0 call(s) -> PASS
What if: Performing the operation "disable" on target "asato".
What if: Performing the operation "disable" on target "dmorgan".
WhatIf: 0 call(s) -> PASS
@@requires
function Mock
Set-Item
$script:calls
@@hint
`function Mock([string]$Name, [scriptblock]$Body) { $script:calls[$Name] = [System.Collections.Generic.List[object]]::new(); $wrapper = { $script:calls[$Name].Add($args); & $Body @args }.GetNewClosure(); Set-Item -Path "function:global:$Name" -Value $wrapper }`. Because the mock records `$args`, `Disable-Account -Name x` stores `'-Name:', 'x'` style values; it's easier to have the mock body record `$Name` itself: define the mock with `param($Name)`. The `What if:` lines in the expected output come from PowerShell itself when you call `Disable-StaleAccount -WhatIf`.
@@solution
function Disable-StaleAccount {
    [CmdletBinding(SupportsShouldProcess)]
    param([datetime]$Now, [int]$Days = 90)
    foreach ($u in Get-ADUserList) {
        if ($u.Enabled -and ($Now - $u.LastLogon).Days -gt $Days) {
            if ($PSCmdlet.ShouldProcess($u.Name, 'disable')) { Disable-Account -Name $u.Name }
        }
    }
}
$now = [datetime]'2026-11-03'

$script:calls = @{}
function Mock([string]$Name, [scriptblock]$Body) {
    $script:calls[$Name] = [System.Collections.Generic.List[object]]::new()
    $log = $script:calls[$Name]
    $wrapper = { param($Name) $log.Add($Name); & $Body $Name }.GetNewClosure()
    Set-Item -Path "function:global:$Name" -Value $wrapper
}

function Assert([string]$Label, [bool]$Condition) {
    if ($Condition) { "$Label -> PASS" } else { "$Label -> FAIL" }
}

$users = @(
    [pscustomobject]@{ Name = 'asato'; Enabled = $true; LastLogon = [datetime]'2026-06-01' }
    [pscustomobject]@{ Name = 'bkowalski'; Enabled = $true; LastLogon = [datetime]'2026-10-30' }
    [pscustomobject]@{ Name = 'cold-svc'; Enabled = $false; LastLogon = [datetime]'2025-01-01' }
    [pscustomobject]@{ Name = 'dmorgan'; Enabled = $true; LastLogon = [datetime]'2026-07-20' }
)

Mock Get-ADUserList { $users }.GetNewClosure()
Mock Disable-Account { }
Disable-StaleAccount -Now $now
$disabled = @($script:calls['Disable-Account'])
Assert "stale users disabled: $($disabled -join ', ')" (($disabled -join ',') -eq 'asato,dmorgan')

$fresh = $users | Where-Object Name -eq 'bkowalski'
Mock Get-ADUserList { $fresh }.GetNewClosure()
Mock Disable-Account { }
Disable-StaleAccount -Now $now
Assert "fresh users only: $($script:calls['Disable-Account'].Count) call(s)" ($script:calls['Disable-Account'].Count -eq 0)

Mock Get-ADUserList { $users }.GetNewClosure()
Mock Disable-Account { }
Disable-StaleAccount -Now $now -WhatIf
Assert "WhatIf: $($script:calls['Disable-Account'].Count) call(s)" ($script:calls['Disable-Account'].Count -eq 0)
