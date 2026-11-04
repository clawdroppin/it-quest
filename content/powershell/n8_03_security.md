@@lesson n8-security Secure PowerShell: Injection, Secrets, Signing and JEA
@@concepts errors, strings
@@text
PowerShell is the most powerful tool on a Windows machine, which is why attackers love it and defenders log it. Writing secure scripts and running PowerShell securely are both part of an admin's job.

## 1. Never build code from data

`Invoke-Expression` (alias `iex`) runs a string as code. With any outside data in it, that's command injection:

```powershell,norun
$name = Read-Host 'Service'               # user types:  x; Remove-Item C:\data -Recurse
Invoke-Expression "Get-Service $name"     # runs both commands!
Get-Service -Name $name                   # safe: the value is a parameter, never code
```

The same applies to `powershell.exe -Command "...$var..."`, `cmd /c "...$var..."`, `[scriptblock]::Create("...$var...")` and `Add-Type` with pasted source. Pass data as **parameters**: `Start-Process -ArgumentList`, `Invoke-Command -ArgumentList`, splatting. When building a command line for a native program is unavoidable, validate the value against a strict pattern first.

## 2. Validate input with attributes

```powershell,norun
param(
    [ValidatePattern('^[a-z][a-z0-9-]{1,30}$')][string]$Hostname,
    [ValidateSet('Start', 'Stop', 'Restart')][string]$Action,
    [ValidateRange(1, 65535)][int]$Port,
    [ValidateScript({ Test-Path $_ -PathType Container })][string]$Folder
)
```

## 3. Secrets

- Never hard-code passwords, API keys or connection strings: they end up in Git, backups and screenshots.
- Ask with `Get-Credential` / `Read-Host -AsSecureString`, or read from the **SecretManagement** module (`Get-Secret -Name ApiKey -AsPlainText`), Windows Credential Manager, or a vault.
- `Export-Clixml` a credential: it's encrypted with DPAPI for **that user on that machine** only, a reasonable way to store a credential for a scheduled task.
- `SecureString` hides a value in memory and from accidental printing; it's not strong encryption on non-Windows systems.
- Don't log secrets: `Start-Transcript` records what's on screen, and script block logging records the code, including literal passwords in it.

## 4. Execution policy is not a security boundary

`Set-ExecutionPolicy AllSigned` or `RemoteSigned` prevents **accidental** running of untrusted scripts. Anyone can bypass it (`powershell -ExecutionPolicy Bypass`, or pasting the code). Real controls are: **application control** (WDAC / AppLocker), which puts PowerShell into **Constrained Language Mode** for untrusted code, least privilege, and logging.

## 5. Code signing

```powershell,norun
$cert = Get-ChildItem Cert:\CurrentUser\My -CodeSigningCert | Select-Object -First 1
Set-AuthenticodeSignature -FilePath .\Deploy.ps1 -Certificate $cert -TimestampServer http://timestamp.digicert.com
Get-AuthenticodeSignature .\Deploy.ps1 | Select-Object Status, SignerCertificate
```

A signature proves who published the script and that it hasn't been changed since. Any edit invalidates it.

## 6. JEA: Just Enough Administration

JEA lets help-desk staff run **specific** commands as admin over remoting, without being admins. A role capability file lists allowed cmdlets and even allowed parameter values; a session configuration maps AD groups to roles, runs as a temporary virtual account, and logs everything:

```powershell,norun
New-PSRoleCapabilityFile -Path .\HelpDesk.psrc -VisibleCmdlets @{
    Name = 'Restart-Service'; Parameters = @{ Name = 'Name'; ValidateSet = 'Spooler', 'W32Time' } }
# then: Register-PSSessionConfiguration -Name HelpDesk -Path .\HelpDesk.pssc
Enter-PSSession -ComputerName srv01 -ConfigurationName HelpDesk
```

## 7. Logging for defenders

Enable via Group Policy: **Script Block Logging** (event 4104 records all code that runs, even deobfuscated), **Module Logging** (4103) and **Transcription**. Forward the events to a SIEM. Attackers use encoded commands (`-EncodedCommand`), download cradles (`iwr ... | iex`) and AMSI bypasses; script block logging and AMSI (the Antimalware Scan Interface) let defenders see what actually ran.

@@quiz
Why is `Invoke-Expression "Get-Service $name"` dangerous?
* If `$name` contains `;` or other syntax, the extra text runs as additional commands
- Invoke-Expression is slow
- It doesn't support Get-Service
- It runs as administrator
? Pass values as parameters: Get-Service -Name $name.

@@quiz
Is execution policy a security boundary?
- Yes, it stops all malicious scripts
* No: it prevents accidents but can be bypassed by any user; use application control and logging for security
- Only AllSigned is
- Only on servers
? Microsoft documents this explicitly.

@@quiz
What does JEA provide?
- Faster remoting
* Constrained remote endpoints where users can run only specific commands, as a temporary privileged account, with logging
- Encrypted scripts
- Automatic patching
? Just Enough Administration.

@@flashcards
`Invoke-Expression` :: runs a string as code (avoid)
`ValidatePattern` / `ValidateSet` :: parameter allow-lists
SecretManagement :: standard secrets API
`Export-Clixml` credential :: DPAPI, this user and machine
execution policy :: safety feature, not security
Constrained Language Mode :: restricted PowerShell under app control
`Set-AuthenticodeSignature` :: sign a script
JEA :: role-based constrained endpoints
event 4104 :: script block logging
AMSI :: antimalware scanning of scripts

@@exercise Find the injection risks
Write `Find-RiskyCode` that scans script text **with the PowerShell parser** (`[System.Management.Automation.Language.Parser]::ParseInput`), not regexes, and reports:

- calls to `Invoke-Expression` or its alias `iex`
- `ConvertTo-SecureString` with `-AsPlainText` (a password in the script)
- variables named like `password`, `secret` or `apikey` assigned a **string literal**
- `[scriptblock]::Create` calls

Print `line N: finding` sorted by line. Parsing means comments and strings that merely mention `iex` don't produce false positives.
@@starter
$script = @'
# this comment mentions iex but is harmless
$name = Read-Host 'Service'
Invoke-Expression "Get-Service $name"
$apiKey = 'sk-live-12345'
$pw = ConvertTo-SecureString 'Winter2026!' -AsPlainText -Force
Write-Host "do not use iex here"
$sb = [scriptblock]::Create("Get-Process $name")
iex $payload
$password = Get-Secret -Name Db -AsPlainText
'@
'TODO'
@@expected
line 3: Invoke-Expression call
line 4: hard-coded secret in $apiKey
line 5: plain-text password with ConvertTo-SecureString
line 7: [scriptblock]::Create call
line 8: Invoke-Expression call
@@requires
ParseInput
FindAll
CommandAst
AssignmentStatementAst
@@hint
`$ast = [System.Management.Automation.Language.Parser]::ParseInput($script, [ref]$null, [ref]$null)`. Commands: `$ast.FindAll({ $args[0] -is [System.Management.Automation.Language.CommandAst] }, $true)` and `$c.GetCommandName()`. Parameters: `$c.CommandElements | Where-Object { $_ -is [System.Management.Automation.Language.CommandParameterAst] }`. Assignments: check `$a.Left.VariablePath.UserPath -match 'password|secret|apikey'` and `$a.Right.Expression -is [System.Management.Automation.Language.StringConstantExpressionAst]`. Method calls: `InvokeMemberExpressionAst` whose `.Expression.Extent.Text` is `[scriptblock]` and `.Member.Value` is `Create`. Each AST has `.Extent.StartLineNumber`.
@@solution
$script = @'
# this comment mentions iex but is harmless
$name = Read-Host 'Service'
Invoke-Expression "Get-Service $name"
$apiKey = 'sk-live-12345'
$pw = ConvertTo-SecureString 'Winter2026!' -AsPlainText -Force
Write-Host "do not use iex here"
$sb = [scriptblock]::Create("Get-Process $name")
iex $payload
$password = Get-Secret -Name Db -AsPlainText
'@

function Find-RiskyCode([string]$Text) {
    $L = 'System.Management.Automation.Language'
    $ast = [System.Management.Automation.Language.Parser]::ParseInput($Text, [ref]$null, [ref]$null)

    foreach ($c in $ast.FindAll({ $args[0] -is [System.Management.Automation.Language.CommandAst] }, $true)) {
        $name = $c.GetCommandName()
        if ($name -in 'Invoke-Expression', 'iex') {
            [pscustomobject]@{ Line = $c.Extent.StartLineNumber; Text = 'Invoke-Expression call' }
        }
        if ($name -eq 'ConvertTo-SecureString') {
            $params = $c.CommandElements | Where-Object { $_ -is [System.Management.Automation.Language.CommandParameterAst] }
            if ($params.ParameterName -contains 'AsPlainText') {
                [pscustomobject]@{ Line = $c.Extent.StartLineNumber; Text = 'plain-text password with ConvertTo-SecureString' }
            }
        }
    }

    foreach ($a in $ast.FindAll({ $args[0] -is [System.Management.Automation.Language.AssignmentStatementAst] }, $true)) {
        $var = $a.Left.VariablePath.UserPath
        $right = $a.Right.Expression
        if ($var -match 'password|secret|apikey' -and $right -is [System.Management.Automation.Language.StringConstantExpressionAst]) {
            [pscustomobject]@{ Line = $a.Extent.StartLineNumber; Text = "hard-coded secret in `$$var" }
        }
    }

    foreach ($m in $ast.FindAll({ $args[0] -is [System.Management.Automation.Language.InvokeMemberExpressionAst] }, $true)) {
        if ($m.Expression.Extent.Text -eq '[scriptblock]' -and $m.Member.Value -eq 'Create') {
            [pscustomobject]@{ Line = $m.Extent.StartLineNumber; Text = '[scriptblock]::Create call' }
        }
    }
}

Find-RiskyCode $script | Sort-Object Line | ForEach-Object { "line $($_.Line): $($_.Text)" }

@@exercise Safe command building and stored credentials
1. A help-desk script takes a service name and an action. Replace `Invoke-Expression` with a validated advanced function `Invoke-ServiceAction` using `ValidateSet` for the action and `ValidatePattern` for the name; show that the injection attempt is rejected **before** anything runs
2. Build arguments for a native program (`robocopy`) as an **array** and show it, instead of a single string
3. Round-trip a credential through `Export-Clixml` / `Import-Clixml` in a temp file, and show that the file doesn't contain the password in clear text
@@starter
$requests = @(
    @{ Name = 'Spooler'; Action = 'Restart' }
    @{ Name = 'Spooler; Remove-Item C:\data -Recurse'; Action = 'Restart' }
    @{ Name = 'W32Time'; Action = 'Format' }
)
'TODO'
@@expected
would Restart service Spooler
rejected: Name failed validation
rejected: Action failed validation
robocopy args: [C:\source dir] [D:\backup] [/MIR] [/R:2] [/LOG:C:\logs\copy.log]
credential user: svc-backup, password restored: True, plain text in file: False
@@requires
ValidateSet
ValidatePattern
Export-Clixml
@@hint
Catch `[System.Management.Automation.ParameterBindingException]` (validation failures) and print which parameter failed: `$_.Exception.ParameterName`. Build the credential: `[pscredential]::new('svc-backup', (ConvertTo-SecureString 'S3cure!pass' -AsPlainText -Force))` (fine in an exercise, not in a real script). Check the file with `(Get-Content $file -Raw) -match 'S3cure'`, and compare `$restored.GetNetworkCredential().Password`.
@@solution
$requests = @(
    @{ Name = 'Spooler'; Action = 'Restart' }
    @{ Name = 'Spooler; Remove-Item C:\data -Recurse'; Action = 'Restart' }
    @{ Name = 'W32Time'; Action = 'Format' }
)

function Invoke-ServiceAction {
    [CmdletBinding()]
    param(
        [Parameter(Mandatory)][ValidatePattern('^[A-Za-z0-9_.-]{1,64}$')][string]$Name,
        [Parameter(Mandatory)][ValidateSet('Start', 'Stop', 'Restart')][string]$Action
    )
    "would $Action service $Name"
}

foreach ($r in $requests) {
    try {
        Invoke-ServiceAction @r
    } catch [System.Management.Automation.ParameterBindingException] {
        "rejected: $($_.Exception.ParameterName) failed validation"
    }
}

$source = 'C:\source dir'
$robocopyArgs = @($source, 'D:\backup', '/MIR', '/R:2', '/LOG:C:\logs\copy.log')
"robocopy args: $(($robocopyArgs | ForEach-Object { "[$_]" }) -join ' ')"
# & robocopy @robocopyArgs   (each element is passed as one argument, spaces included)

$file = Join-Path ([System.IO.Path]::GetTempPath()) "cred-$PID.xml"
$cred = [pscredential]::new('svc-backup', (ConvertTo-SecureString 'S3cure!pass' -AsPlainText -Force))
$cred | Export-Clixml -Path $file
$restored = Import-Clixml -Path $file
$plain = (Get-Content $file -Raw) -match 'S3cure'
"credential user: $($restored.UserName), password restored: $($restored.GetNetworkCredential().Password -eq 'S3cure!pass'), plain text in file: $plain"
Remove-Item $file
