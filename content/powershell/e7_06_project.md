@@lesson e7-project Project: A Configuration Drift Detector
@@concepts objects, maps, automation
@@text
Servers drift: someone enables a service "just for a test", a hotfix changes a registry value, a firewall rule appears. **Desired state configuration** fixes this by describing how a machine *should* look, comparing that with how it *does* look, and repairing the difference. That's the idea behind PowerShell DSC, Ansible, Puppet and Group Policy.

In this project you'll build a small drift detector with the patterns from this module: classes for resources, a pipeline of results, proper errors and a summary.

## The Test / Set pattern

Every resource type answers two questions:

- **Test**: is the actual state equal to the desired state? (read-only, safe to run any time)
- **Set**: make it so. Must be **idempotent**: running it twice has the same effect as running it once.

```powershell,norun
class ServiceResource {
    [string]$Name
    [string]$StartupType
    [string]$State
    [bool] Test() {
        $svc = Get-Service $this.Name -ErrorAction Stop
        return $svc.StartType -eq $this.StartupType -and $svc.Status -eq $this.State
    }
    [void] Set() {
        Set-Service $this.Name -StartupType $this.StartupType
        if ($this.State -eq 'Running') { Start-Service $this.Name } else { Stop-Service $this.Name }
    }
}
```

## Desired state as data

Keep the desired state in a data file (`.psd1`, JSON or YAML), not in code, so it can be reviewed in Git like any other change:

```powershell,norun
@{
    Services = @(
        @{ Name = 'Spooler';   StartupType = 'Disabled';  State = 'Stopped' }
        @{ Name = 'W32Time';   StartupType = 'Automatic'; State = 'Running' }
    )
    Registry = @(
        @{ Path = 'HKLM:\SYSTEM\CurrentControlSet\Control\Lsa'; Name = 'LmCompatibilityLevel'; Value = 5 }
    )
}
# $desired = Import-PowerShellDataFile .\baseline.psd1   (safe: data only, no code runs)
```

## Reporting drift

A useful report lists **what** differs, **expected vs actual**, and what would be done. Always run in report-only mode (`-WhatIf`) first; repair only after review, or automatically for low-risk settings.

## The exercises

To run anywhere, the "machine" is a hashtable you can read and change, so `Test` and `Set` are pure logic. Swapping it for `Get-Service` and `Get-ItemProperty` is a small step afterwards.

@@quiz
What makes a Set operation idempotent?
* Running it once or several times leaves the system in the same state
- It runs only once per day
- It never fails
- It doesn't change anything
? "Ensure X" rather than "do X".

@@quiz
Why use `Import-PowerShellDataFile` for a baseline instead of dot-sourcing a .ps1?
- It's faster
* It only accepts data, so a baseline file can't run code
- It supports more types
- It's required by DSC
? A .psd1 is a hashtable literal, read safely.

@@flashcards
desired state :: how the system should be
drift :: difference between desired and actual
Test :: check, read-only
Set :: repair, idempotent
`Import-PowerShellDataFile` :: read a .psd1 safely
report-only / -WhatIf :: show changes without making them
DSC / Ansible / Puppet :: configuration management tools

@@exercise Test every resource
Build the detection side:

- an abstract-ish base class `Resource` with `[string]$Id`, `[string] Actual($machine)`, `[string] Expected()` and `[bool] Test($machine)` (compares the two strings)
- `ServiceResource` (expected `StartupType/State`, read from `$machine.Services[name]`; a missing service gives `missing`)
- `RegistryResource` (expected value as text, read from `$machine.Registry["path\name"]`, missing gives `missing`)
- build the resources from the `$desired` data, test each against `$machine`, and print `OK` / `DRIFT` lines with expected and actual, then a summary
@@starter
$desired = @{
    Services = @(
        @{ Name = 'Spooler'; StartupType = 'Disabled'; State = 'Stopped' }
        @{ Name = 'W32Time'; StartupType = 'Automatic'; State = 'Running' }
        @{ Name = 'WinRM'; StartupType = 'Automatic'; State = 'Running' }
    )
    Registry = @(
        @{ Path = 'HKLM:\SYSTEM\CurrentControlSet\Control\Lsa'; Name = 'LmCompatibilityLevel'; Value = 5 }
        @{ Path = 'HKLM:\SOFTWARE\Policies\Microsoft\Windows NT\Terminal Services'; Name = 'fDenyTSConnections'; Value = 1 }
    )
}
$machine = @{
    Services = @{
        Spooler = @{ StartupType = 'Automatic'; State = 'Running' }
        W32Time = @{ StartupType = 'Automatic'; State = 'Running' }
    }
    Registry = @{
        'HKLM:\SYSTEM\CurrentControlSet\Control\Lsa\LmCompatibilityLevel' = 3
        'HKLM:\SOFTWARE\Policies\Microsoft\Windows NT\Terminal Services\fDenyTSConnections' = 1
    }
}
'TODO'
@@expected
DRIFT service:Spooler expected Disabled/Stopped, actual Automatic/Running
OK    service:W32Time
DRIFT service:WinRM expected Automatic/Running, actual missing
DRIFT registry:Lsa\LmCompatibilityLevel expected 5, actual 3
OK    registry:Terminal Services\fDenyTSConnections
5 resources, 2 compliant, 3 drifted (40% compliant)
@@requires
class Resource
: Resource
[bool] Test(
@@hint
In the base class, give `Actual` and `Expected` bodies that throw (`throw 'override me'`); child classes override them. For the registry Id, use the last part of the path: `Split-Path $Path -Leaf`. Compute the percentage with integers: `[int](100 * $ok / $total)`.
@@solution
$desired = @{
    Services = @(
        @{ Name = 'Spooler'; StartupType = 'Disabled'; State = 'Stopped' }
        @{ Name = 'W32Time'; StartupType = 'Automatic'; State = 'Running' }
        @{ Name = 'WinRM'; StartupType = 'Automatic'; State = 'Running' }
    )
    Registry = @(
        @{ Path = 'HKLM:\SYSTEM\CurrentControlSet\Control\Lsa'; Name = 'LmCompatibilityLevel'; Value = 5 }
        @{ Path = 'HKLM:\SOFTWARE\Policies\Microsoft\Windows NT\Terminal Services'; Name = 'fDenyTSConnections'; Value = 1 }
    )
}
$machine = @{
    Services = @{
        Spooler = @{ StartupType = 'Automatic'; State = 'Running' }
        W32Time = @{ StartupType = 'Automatic'; State = 'Running' }
    }
    Registry = @{
        'HKLM:\SYSTEM\CurrentControlSet\Control\Lsa\LmCompatibilityLevel' = 3
        'HKLM:\SOFTWARE\Policies\Microsoft\Windows NT\Terminal Services\fDenyTSConnections' = 1
    }
}

class Resource {
    [string]$Id
    [string] Expected() { throw 'override Expected()' }
    [string] Actual([hashtable]$machine) { throw 'override Actual()' }
    [bool] Test([hashtable]$machine) { return $this.Actual($machine) -eq $this.Expected() }
}

class ServiceResource : Resource {
    [string]$Name
    [string]$StartupType
    [string]$State
    ServiceResource([hashtable]$d) {
        $this.Name = $d.Name
        $this.StartupType = $d.StartupType
        $this.State = $d.State
        $this.Id = "service:$($d.Name)"
    }
    [string] Expected() { return "$($this.StartupType)/$($this.State)" }
    [string] Actual([hashtable]$machine) {
        $s = $machine.Services[$this.Name]
        if ($null -eq $s) { return 'missing' }
        return "$($s.StartupType)/$($s.State)"
    }
}

class RegistryResource : Resource {
    [string]$Key
    [string]$Value
    RegistryResource([hashtable]$d) {
        $this.Key = "$($d.Path)\$($d.Name)"
        $this.Value = [string]$d.Value
        $this.Id = "registry:$(Split-Path $d.Path -Leaf)\$($d.Name)"
    }
    [string] Expected() { return $this.Value }
    [string] Actual([hashtable]$machine) {
        if (-not $machine.Registry.ContainsKey($this.Key)) { return 'missing' }
        return [string]$machine.Registry[$this.Key]
    }
}

$resources = @()
$resources += $desired.Services | ForEach-Object { [ServiceResource]::new($_) }
$resources += $desired.Registry | ForEach-Object { [RegistryResource]::new($_) }

$ok = 0
foreach ($r in $resources) {
    if ($r.Test($machine)) {
        "OK    $($r.Id)"
        $ok++
    } else {
        "DRIFT $($r.Id) expected $($r.Expected()), actual $($r.Actual($machine))"
    }
}
$total = $resources.Count
"$total resources, $ok compliant, $($total - $ok) drifted ($([int](100 * $ok / $total))% compliant)"

@@exercise Repair with -WhatIf, idempotently
Add the repair side as an advanced function `Repair-Drift` with `SupportsShouldProcess`:

- `Set($machine)` methods: a service gets the desired startup type and state (created if missing); a registry value is written
- `Repair-Drift -Resources -Machine` tests each resource and, for drifted ones, calls `Set` only `if ($PSCmdlet.ShouldProcess($r.Id, "set to $expected"))`; it outputs one result object per resource (`Id`, `Action` = `none`, `would fix` or `fixed`)
- run it with `-WhatIf` (nothing changes; print the actions), then for real, then again to prove idempotency (all `none`)

The starter includes compact versions of the classes from the previous exercise.
@@starter
class Resource {
    [string]$Id
    [string] Expected() { throw 'override' }
    [string] Actual([hashtable]$m) { throw 'override' }
    [void] Set([hashtable]$m) { throw 'override' }
    [bool] Test([hashtable]$m) { return $this.Actual($m) -eq $this.Expected() }
}
class ServiceResource : Resource {
    [string]$Name; [string]$Want
    ServiceResource([string]$name, [string]$want) { $this.Name = $name; $this.Want = $want; $this.Id = "service:$name" }
    [string] Expected() { return $this.Want }
    [string] Actual([hashtable]$m) { if ($m.Services.ContainsKey($this.Name)) { return $m.Services[$this.Name] }; return 'missing' }
}
class RegistryResource : Resource {
    [string]$Key; [string]$Want
    RegistryResource([string]$key, [string]$want) { $this.Key = $key; $this.Want = $want; $this.Id = "registry:$key" }
    [string] Expected() { return $this.Want }
    [string] Actual([hashtable]$m) { if ($m.Registry.ContainsKey($this.Key)) { return [string]$m.Registry[$this.Key] }; return 'missing' }
}
$machine = @{ Services = @{ Spooler = 'Automatic/Running'; W32Time = 'Automatic/Running' }; Registry = @{ LmCompatibilityLevel = 3 } }
$resources = @(
    [ServiceResource]::new('Spooler', 'Disabled/Stopped')
    [ServiceResource]::new('W32Time', 'Automatic/Running')
    [ServiceResource]::new('WinRM', 'Automatic/Running')
    [RegistryResource]::new('LmCompatibilityLevel', '5')
)
'TODO'
@@expected
== WhatIf
What if: Performing the operation "set to Disabled/Stopped" on target "service:Spooler".
service:Spooler would fix
service:W32Time none
What if: Performing the operation "set to Automatic/Running" on target "service:WinRM".
service:WinRM would fix
What if: Performing the operation "set to 5" on target "registry:LmCompatibilityLevel".
registry:LmCompatibilityLevel would fix
Spooler still: Automatic/Running
== Apply
service:Spooler fixed
service:W32Time none
service:WinRM fixed
registry:LmCompatibilityLevel fixed
== Apply again
service:Spooler none
service:W32Time none
service:WinRM none
registry:LmCompatibilityLevel none
@@requires
SupportsShouldProcess
ShouldProcess
[void] Set(
@@hint
Add `Set` overrides to the child classes: `[void] Set([hashtable]$m) { $m.Services[$this.Name] = $this.Want }`. In `Repair-Drift`, `$WhatIfPreference` is true under `-WhatIf`, and `ShouldProcess` returns false (printing a "What if:" line to the host, not to output). Report `would fix` when `ShouldProcess` is false. The `What if:` lines are written by PowerShell itself, straight to the host, so they appear in the output between your result lines (on a non-English Windows they may be translated).
@@solution
class Resource {
    [string]$Id
    [string] Expected() { throw 'override' }
    [string] Actual([hashtable]$m) { throw 'override' }
    [void] Set([hashtable]$m) { throw 'override' }
    [bool] Test([hashtable]$m) { return $this.Actual($m) -eq $this.Expected() }
}
class ServiceResource : Resource {
    [string]$Name; [string]$Want
    ServiceResource([string]$name, [string]$want) { $this.Name = $name; $this.Want = $want; $this.Id = "service:$name" }
    [string] Expected() { return $this.Want }
    [string] Actual([hashtable]$m) { if ($m.Services.ContainsKey($this.Name)) { return $m.Services[$this.Name] }; return 'missing' }
    [void] Set([hashtable]$m) { $m.Services[$this.Name] = $this.Want }
}
class RegistryResource : Resource {
    [string]$Key; [string]$Want
    RegistryResource([string]$key, [string]$want) { $this.Key = $key; $this.Want = $want; $this.Id = "registry:$key" }
    [string] Expected() { return $this.Want }
    [string] Actual([hashtable]$m) { if ($m.Registry.ContainsKey($this.Key)) { return [string]$m.Registry[$this.Key] }; return 'missing' }
    [void] Set([hashtable]$m) { $m.Registry[$this.Key] = [int]$this.Want }
}
$machine = @{ Services = @{ Spooler = 'Automatic/Running'; W32Time = 'Automatic/Running' }; Registry = @{ LmCompatibilityLevel = 3 } }
$resources = @(
    [ServiceResource]::new('Spooler', 'Disabled/Stopped')
    [ServiceResource]::new('W32Time', 'Automatic/Running')
    [ServiceResource]::new('WinRM', 'Automatic/Running')
    [RegistryResource]::new('LmCompatibilityLevel', '5')
)

function Repair-Drift {
    [CmdletBinding(SupportsShouldProcess)]
    param([Resource[]]$Resources, [hashtable]$Machine)
    foreach ($r in $Resources) {
        $action = 'none'
        if (-not $r.Test($Machine)) {
            if ($PSCmdlet.ShouldProcess($r.Id, "set to $($r.Expected())")) {
                $r.Set($Machine)
                $action = 'fixed'
            } else {
                $action = 'would fix'
            }
        }
        [pscustomobject]@{ Id = $r.Id; Action = $action }
    }
}

'== WhatIf'
Repair-Drift -Resources $resources -Machine $machine -WhatIf | ForEach-Object { "$($_.Id) $($_.Action)" }
"Spooler still: $($machine.Services['Spooler'])"
'== Apply'
Repair-Drift -Resources $resources -Machine $machine | ForEach-Object { "$($_.Id) $($_.Action)" }
'== Apply again'
Repair-Drift -Resources $resources -Machine $machine | ForEach-Object { "$($_.Id) $($_.Action)" }
