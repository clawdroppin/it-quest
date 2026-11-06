@@lesson p4-remoting Remoting, Active Directory and Microsoft 365
@@concepts networking, automation
@@text
One admin, 300 computers. Remoting is how PowerShell scales.

## PowerShell remoting (WinRM)

Remoting runs commands **on** other computers, over WinRM (HTTP port **5985**, HTTPS **5986**). It's on by default on Windows Server; on clients, `Enable-PSRemoting` turns it on.

```powershell,norun
Invoke-Command -ComputerName SRV01, SRV02, SRV03 -ScriptBlock {
    Get-Service -Name Spooler | Select-Object Status
}
# Each result gets a PSComputerName property telling you where it came from.

Enter-PSSession -ComputerName SRV01      # an interactive remote prompt; Exit-PSSession to leave

$s = New-PSSession -ComputerName SRV01   # a reusable session (faster for many commands)
Invoke-Command -Session $s -ScriptBlock { $env:COMPUTERNAME }
Remove-PSSession $s
```

Pass local variables into the remote script block with `$using:`:

```powershell,norun
$service = 'Spooler'
Invoke-Command -ComputerName SRV01 -ScriptBlock { Restart-Service -Name $using:service }
```

`Invoke-Command` runs on all computers **in parallel** (32 at a time by default), so checking 300 servers takes seconds, not hours.

## Credentials, the right way

```powershell,norun
$cred = Get-Credential                     # a secure prompt; the password stays encrypted in memory
Invoke-Command -ComputerName SRV01 -Credential $cred -ScriptBlock { whoami }
```

**Never** put passwords in scripts as plain text. For unattended scripts, use a managed service account, the **SecretManagement** module, or Azure Key Vault.

## Active Directory

The `ActiveDirectory` module (part of RSAT, or on a domain controller) is daily help desk work:

```powershell,norun
Get-ADUser -Identity asilva -Properties LastLogonDate, LockedOut, PasswordLastSet
Unlock-ADAccount -Identity asilva
Set-ADAccountPassword -Identity asilva -Reset -NewPassword (Read-Host -AsSecureString 'New password')
Set-ADUser -Identity asilva -ChangePasswordAtLogon $true

Get-ADUser -Filter { Enabled -eq $true } -Properties LastLogonDate |
    Where-Object LastLogonDate -lt (Get-Date).AddDays(-90)                 # stale accounts

New-ADUser -Name 'Ana Silva' -SamAccountName asilva -UserPrincipalName asilva@contoso.com `
    -Path 'OU=Sales,DC=contoso,DC=com' -AccountPassword $pw -Enabled $true
Add-ADGroupMember -Identity 'Sales-Team' -Members asilva
Get-ADGroupMember -Identity 'Domain Admins'                                # who has the keys?
Search-ADAccount -LockedOut
```

## Microsoft 365 and Entra ID (Microsoft Graph)

Cloud users and licenses are managed with the **Microsoft Graph PowerShell SDK**:

```powershell,norun
Connect-MgGraph -Scopes 'User.Read.All', 'Group.ReadWrite.All'
Get-MgUser -Filter "department eq 'Sales'" -All
Get-MgUserLicenseDetail -UserId asilva@contoso.com
Connect-ExchangeOnline           # mailboxes (ExchangeOnlineManagement module)
Get-Mailbox -ResultSize 10
```

(The older `MSOnline` and `AzureAD` modules are retired: if a tutorial uses `Connect-MsolService`, it's out of date.)

## Practice without a company network

Build a lab: one **Windows Server evaluation** VM (free for 180 days) promoted to a domain controller with `Install-WindowsFeature AD-Domain-Services` and `Install-ADDSForest`, and a Windows 11 VM joined to it. That lab is exactly what interviewers mean by "hands-on experience".

@@quiz
Which command runs a script block on 50 servers in parallel?
* `Invoke-Command -ComputerName $servers -ScriptBlock { ... }`
- `foreach ($s in $servers) { Enter-PSSession $s }`
- `Start-Process -ComputerName $servers`
- `Get-Service -ComputerName $servers -Parallel`
? Invoke-Command fans out over WinRM, 32 at a time by default.

@@quiz
How do you use a local variable `$name` inside `Invoke-Command -ScriptBlock { }` on a remote computer?
- `$name` works as-is
* `$using:name`
- `$global:name`
- `$remote:name`
? The script block runs on the other computer, where `$name` doesn't exist. `$using:` sends the value along.

@@quiz
A user is locked out of their domain account. Which cmdlet unlocks it?
- `Set-ADUser -Locked $false`
* `Unlock-ADAccount`
- `Enable-ADAccount`
- `Reset-ADPassword`
? `Unlock-ADAccount -Identity user`. `Search-ADAccount -LockedOut` finds all locked accounts.

@@drill
Which cmdlet runs a script block on remote computers?
= Invoke-Command
? Invoke-Command -ComputerName srv1, srv2 { ... }.

@@flashcards
WinRM ports :: 5985 (HTTP) and 5986 (HTTPS)
Invoke-Command :: Run a script block on one or many remote computers
Enter-PSSession :: Interactive remote PowerShell prompt
$using: :: Send a local variable's value into a remote script block
Get-Credential :: Securely ask for a username and password
Get-ADUser :: Read Active Directory users; add -Properties for more fields
Unlock-ADAccount :: Unlock a locked-out AD account
Microsoft Graph PowerShell :: The current module for Entra ID and Microsoft 365 (Connect-MgGraph)

@@exercise Results from many servers
`$results` imitates the output of `Invoke-Command`: each object has a `PSComputerName` and a `Status`. Print `<computer>: <status>` for every server whose status is **not** `Running`, then `<n> of <total> servers need attention`.
@@starter
$results = @(
    [pscustomobject]@{ PSComputerName = 'SRV01'; Status = 'Running' }
    [pscustomobject]@{ PSComputerName = 'SRV02'; Status = 'Stopped' }
    [pscustomobject]@{ PSComputerName = 'SRV03'; Status = 'Running' }
    [pscustomobject]@{ PSComputerName = 'SRV04'; Status = 'StopPending' }
)

@@expected
SRV02: Stopped
SRV04: StopPending
2 of 4 servers need attention
@@hint
Save the filtered list in a variable with `@( ... )` so `.Count` works, print it, then the summary line.
@@solution
$results = @(
    [pscustomobject]@{ PSComputerName = 'SRV01'; Status = 'Running' }
    [pscustomobject]@{ PSComputerName = 'SRV02'; Status = 'Stopped' }
    [pscustomobject]@{ PSComputerName = 'SRV03'; Status = 'Running' }
    [pscustomobject]@{ PSComputerName = 'SRV04'; Status = 'StopPending' }
)
$bad = @($results | Where-Object Status -ne 'Running')
$bad | ForEach-Object { "$($_.PSComputerName): $($_.Status)" }
"$($bad.Count) of $($results.Count) servers need attention"

@@exercise Distinguished names
AD objects live in **OUs**, written as a distinguished name like `OU=Sales,DC=contoso,DC=com`. Write `Get-OUPath` that takes an OU name and a domain like `contoso.com` and returns that DN. Domains can have more parts: `tokyo.corp.example.jp` becomes `DC=tokyo,DC=corp,DC=example,DC=jp`.
@@starter
function Get-OUPath([string]$OU, [string]$Domain) {
    throw "TODO: not done yet"
}

@@tests
check (Get-OUPath 'Sales' 'contoso.com') 'OU=Sales,DC=contoso,DC=com' "Get-OUPath 'Sales' 'contoso.com'"
check (Get-OUPath 'IT' 'tokyo.corp.example.jp') 'OU=IT,DC=tokyo,DC=corp,DC=example,DC=jp' "Get-OUPath 'IT' 'tokyo.corp.example.jp'"
@@hint
Split the domain on dots, turn each part into `DC=part` with `ForEach-Object`, and `-join ','`. Note `-split '\.'`: in a regex, a dot must be escaped (or use `.Split('.')`).
@@solution
function Get-OUPath([string]$OU, [string]$Domain) {
    $dc = ($Domain.Split('.') | ForEach-Object { "DC=$_" }) -join ','
    return "OU=$OU,$dc"
}
