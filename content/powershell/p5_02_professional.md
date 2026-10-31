@@lesson p5-professional Professional PowerShell: Testing, Style, Security
@@concepts testing, automation
@@text
The difference between "I can write PowerShell" and "you can trust my scripts in production".

## Test with Pester

**Pester** is PowerShell's test framework (Windows ships an old version 3; install the current one with `Install-Module Pester -Force -SkipPublisherCheck`). Tests live in `*.Tests.ps1` files:

```powershell,norun
# Get-PortType.Tests.ps1
BeforeAll { . $PSScriptRoot\Get-PortType.ps1 }

Describe 'Get-PortType' {
    It 'knows well-known ports' {
        Get-PortType -Port 443 | Should -Be 'well-known'
    }
    It 'rejects <port>' -ForEach @(@{ port = -1 }, @{ port = 70000 }) {
        Get-PortType -Port $port | Should -Be 'invalid'
    }
}
```

Run `Invoke-Pester` in the folder. Tests let you change a script used by 300 servers without fear.

## Lint with PSScriptAnalyzer

```powershell,norun
Install-Module PSScriptAnalyzer -Scope CurrentUser
Invoke-ScriptAnalyzer -Path .\MyScript.ps1
```

It flags aliases in scripts, unapproved verbs, plain-text passwords, `$null` on the wrong side, unused variables. The VS Code PowerShell extension runs it as you type.

## Style that colleagues will thank you for

- Full cmdlet and parameter names: `Get-ChildItem -Path C:\ -Filter *.log`, not `gci C:\ *.log`.
- One function, one job; Verb-Noun names; comment-based help.
- **Splatting** for long commands, instead of backtick line continuations (a backtick followed by an invisible space breaks the line).
- `$ErrorActionPreference = 'Stop'` and `try/catch` in automation; write errors with `Write-Error`, progress with `Write-Verbose`, never mix status text into the output.
- Output **objects**, not formatted text, so the caller can sort, filter and export.
- Support `-WhatIf` on anything that changes things.
- Keep scripts in **Git**.

## Security for script authors

- Never store secrets in scripts or in Git. Use `Get-Credential`, the **SecretManagement** module, or a vault.
- Watch for **injection**: building a command from user input and passing it to `Invoke-Expression` is dangerous. `Invoke-Expression` is almost never needed.
- **Code signing** (`Set-AuthenticodeSignature`) with an `AllSigned` policy proves who wrote a script.
- **Script block logging** and **transcription** (via Group Policy) record what PowerShell ran. Defenders rely on them, because attackers love PowerShell too.
- **Constrained Language Mode** and **JEA** (Just Enough Administration) limit what remote users can do: a help desk operator could get *only* `Restart-Service` and `Unlock-ADAccount`.

## Performance, briefly

- Filter **left**: `Get-ChildItem -Filter *.log` is far faster than `Get-ChildItem | Where-Object Name -like *.log`, because the filtering happens at the source. Same for `Get-ADUser -Filter` and `Get-WinEvent -FilterHashtable`.
- Avoid `+=` on arrays in big loops (collect loop output, or use a `List`).
- `ForEach-Object -Parallel` (PowerShell 7) and `Start-Job` / `Start-ThreadJob` run work concurrently.

@@quiz
Which is faster for finding log files, and why?
* `Get-ChildItem -Filter *.log`, because the file system does the filtering
- `Get-ChildItem | Where-Object Name -like *.log`, because Where-Object is optimized
- they're the same
- `dir *.log | Sort-Object`
? "Filter left": filter at the source, so fewer objects are created and sent down the pipeline.

@@quiz
Which is the safest way to get a password into an unattended script?
- write it in the script, then hide the script
- pass it as a plain parameter in the scheduled task
* use a secret vault (SecretManagement, Key Vault) or a managed service account
- save it in a text file next to the script
? Plain-text secrets end up in Git, backups and screenshots.

@@quiz
What is JEA?
- A faster version of PowerShell
* Just Enough Administration: remote endpoints that only allow specific commands
- A Java engine for PowerShell
- A code signing certificate
? JEA lets a help desk operator run exactly the commands they need, as admin, without being admin.

@@flashcards
Pester :: PowerShell's test framework: Describe, It, Should
PSScriptAnalyzer :: Linter that finds bad practices in scripts
Filter left :: Filter as early as possible, at the source (-Filter, -FilterHashtable)
Invoke-Expression :: Runs a string as code; risky, avoid it with user input
JEA :: Just Enough Administration: limited, audited admin endpoints
Script block logging :: Records the PowerShell code that runs, for defenders
SecretManagement :: Module for storing and reading secrets from vaults

@@exercise A test helper
Write your own tiny assertion function, the heart of every test framework: `Assert-Equal` takes `-Actual`, `-Expected` and `-Name`. It returns `PASS <name>` when they're equal and `FAIL <name>: expected <e>, got <a>` when they aren't.
@@starter
function Assert-Equal {
    param($Actual, $Expected, [string]$Name)
    throw "TODO: not done yet"
}

@@tests
check (Assert-Equal -Actual 4 -Expected 4 -Name 'math') 'PASS math' "equal values"
check (Assert-Equal -Actual 'web' -Expected 'db' -Name 'role') 'FAIL role: expected db, got web' "different values"
@@hint
`if ($Actual -eq $Expected) { return "PASS $Name" }`, otherwise return the FAIL text.
@@solution
function Assert-Equal {
    param($Actual, $Expected, [string]$Name)
    if ($Actual -eq $Expected) { return "PASS $Name" }
    return "FAIL ${Name}: expected $Expected, got $Actual"
}

@@exercise Find risky lines
A mini script analyzer: write `Find-RiskyLine` that takes an array of script lines and returns the **line numbers** (starting at 1) of lines that contain `Invoke-Expression`, the alias `iex `, or something that looks like a plain-text password: `-Password '...'` or `$password = '...'` (any case).
@@starter
function Find-RiskyLine([string[]]$Lines) {
    throw "TODO: not done yet"
}

@@tests
$script = @(
    '$ErrorActionPreference = "Stop"'
    '$password = ''Summer2026!'''
    'Get-Service | Where-Object Status -eq Running'
    'Invoke-Expression $userInput'
    'Connect-Thing -User admin -Password ''hunter2'''
    'iex (New-Object Net.WebClient).DownloadString($url)'
    '$cred = Get-Credential'
)
check (Find-RiskyLine $script) @(2, 4, 5, 6) "risky lines in the sample script"
check (Find-RiskyLine @('Get-Date')) $null "a safe script"
@@hint
Loop with an index: `for ($i = 0; $i -lt $Lines.Count; $i++)`, and output `$i + 1` when `$Lines[$i] -match 'Invoke-Expression|\biex\s|-Password\s+''|\$password\s*=\s*'''`. (Inside single quotes, a literal `'` is written `''`.)
@@solution
function Find-RiskyLine([string[]]$Lines) {
    $pattern = 'Invoke-Expression|\biex\s|-Password\s+[''"]|\$password\s*=\s*[''"]'
    for ($i = 0; $i -lt $Lines.Count; $i++) {
        if ($Lines[$i] -match $pattern) { $i + 1 }
    }
}
