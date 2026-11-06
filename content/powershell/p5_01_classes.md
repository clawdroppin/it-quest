@@lesson p5-classes Classes, Enums and .NET
@@concepts objects
@@text
`[pscustomobject]` is enough most of the time. When you want **rules** (a type, validation, methods that belong to the data), PowerShell 5 has classes.

```powershell
enum Severity {
    Info = 0
    Warning = 1
    Critical = 2
}

class Alert {
    [string]$Server
    [Severity]$Level
    [string]$Message
    [datetime]$Time = (Get-Date)

    Alert([string]$server, [Severity]$level, [string]$message) {
        $this.Server = $server
        $this.Level = $level
        $this.Message = $message
    }

    [string] ToString() {
        return "[$($this.Level)] $($this.Server): $($this.Message)"
    }

    [bool] IsUrgent() {
        return $this.Level -ge [Severity]::Critical
    }
}

$a = [Alert]::new('db01', 'Critical', 'disk 98% full')
$a.ToString()
$a.IsUrgent()
[Alert]::new('web01', [Severity]::Info, 'deployed').IsUrgent()
```

Rules of thumb:

- Inside methods, use `$this.Property`, and **declare the return type**: `[string] ToString()`. A method without `return` returns nothing; unlike functions, stray output is *not* returned.
- An enum turns magic numbers and strings into names: `'Critical'` is converted automatically, and a typo like `'Critcal'` is an error.
- Classes are defined when the script is **parsed**, so in a script they must come before you use them, and they can't be redefined in the same session (restart PowerShell after changing one).

## .NET: the whole toolbox

PowerShell sits on .NET, so every .NET class is available:

```powershell
[System.Net.Dns]::GetHostAddresses('localhost') | Select-Object -First 1 -ExpandProperty IPAddressToString
[guid]::NewGuid().Guid.Length
[System.IO.Path]::GetExtension('report.final.xlsx')
[Environment]::ProcessorCount -gt 0
[System.Text.Encoding]::UTF8.GetBytes('日本').Length
[Convert]::ToBase64String([Text.Encoding]::UTF8.GetBytes('admin:secret'))
$sw = [System.Diagnostics.Stopwatch]::StartNew(); Start-Sleep -Milliseconds 50; $sw.ElapsedMilliseconds -ge 50
```

`Add-Type -AssemblyName System.Windows.Forms` even gives you GUI dialogs. Hash a file with `Get-FileHash -Algorithm SHA256`, which you'll use when verifying downloads.
## When to use a class

PowerShell scripts usually work fine with `[pscustomobject]` and hashtables. Reach for a class when you want:

- **Validation in one place**: a constructor that refuses a server name that's empty, or a port outside 1–65535.
- **Behaviour that belongs to the data**: a `Ticket` object with `Close()` and `Escalate()` methods.
- **Real types** for parameters: `function Send-Alert([Alert]$Alert)` rejects anything that isn't an `Alert`.

```powershell
class Server {
    [string]$Name
    [int]$Port
    Server([string]$name, [int]$port) {
        if ($port -lt 1 -or $port -gt 65535) { throw "Bad port $port" }
        $this.Name = $name
        $this.Port = $port
    }
    [string] ToString() { return "$($this.Name):$($this.Port)" }
}
$s = [Server]::new('web01', 443)
"$s"
```

Two quirks to remember: inside methods you must use **`$this`** to reach properties, and methods only return what you pass to **`return`** (unlike functions, where all output is returned). Classes defined in a script are also hard to reload in the same session: if you change a class, start a new PowerShell window.

@@quiz
In a class method declared `[int] Count() { ... }`, how is the result returned?
* with `return`; stray output inside methods is not returned
- like functions: everything output is returned
- by assigning to `$result`
- with `Write-Output`
? Class methods behave like methods in C#: only `return` returns a value.

@@quiz
Why use an `enum` for a severity instead of a plain string?
* Typos become errors, and the allowed values are documented in one place
- Enums are faster to print
- Strings can't be compared
- Enums are required in classes
? An enum limits a value to known names, and they compare by their numbers.

@@quiz
Inside a class method, how do you read the object's Name property?
* $this.Name
- $Name
- $_.Name
- $self.Name
? Class methods don't see properties as plain variables; use $this.

@@flashcards
class Name { } :: define a class
[type]$Property :: a typed property
constructor :: method named like the class
$this :: the current object inside a method
return :: required for values from class methods
enum Severity { Low; High } :: a fixed set of names
create an object :: [Name]::new()

@@exercise A Server class
Write a class `Server` with properties `[string]$Name`, `[int]$CpuPercent` and `[int]$FreeDiskGB`, a constructor taking those three values, and a method `[string] Health()` that returns `critical` if CPU is **over 90** or free disk is **under 5**, `warning` if CPU is over 75 or free disk under 20, otherwise `ok`.
@@starter
class Server {
    # TODO: properties, a constructor, and Health()
}

@@tests
check ([Server]::new('web01', 40, 100).Health()) 'ok' "web01 (cpu 40, disk 100)"
check ([Server]::new('web02', 80, 100).Health()) 'warning' "web02 (cpu 80)"
check ([Server]::new('db01', 20, 3).Health()) 'critical' "db01 (disk 3)"
check ([Server]::new('db02', 95, 50).Health()) 'critical' "db02 (cpu 95)"
check ([Server]::new('db03', 10, 19).Health()) 'warning' "db03 (disk 19)"
@@hint
The constructor is named like the class: `Server([string]$n, [int]$c, [int]$d) { $this.Name = $n; ... }`. In `Health()`, test critical first and `return` each result.
@@solution
class Server {
    [string]$Name
    [int]$CpuPercent
    [int]$FreeDiskGB

    Server([string]$name, [int]$cpu, [int]$disk) {
        $this.Name = $name
        $this.CpuPercent = $cpu
        $this.FreeDiskGB = $disk
    }

    [string] Health() {
        if ($this.CpuPercent -gt 90 -or $this.FreeDiskGB -lt 5) { return 'critical' }
        if ($this.CpuPercent -gt 75 -or $this.FreeDiskGB -lt 20) { return 'warning' }
        return 'ok'
    }
}

@@exercise Hash a string
Write `Get-Sha256` that returns the SHA-256 hash of a **string** (UTF-8) as lowercase hex. `Get-FileHash` only does files, so use .NET: `[System.Security.Cryptography.SHA256]::Create()`.
@@starter
function Get-Sha256([string]$Text) {
    throw "TODO: not done yet"
}

@@tests
check (Get-Sha256 'hello') '2cf24dba5fb0a30e26e83b2ac5b9e29e1b161e5c1fa7425e73043362938b9824' "Get-Sha256 'hello'"
check (Get-Sha256 '') 'e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855' "Get-Sha256 ''"
@@hint
`$bytes = [Text.Encoding]::UTF8.GetBytes($Text)`, `$hash = [System.Security.Cryptography.SHA256]::Create().ComputeHash($bytes)`, then `($hash | ForEach-Object { $_.ToString('x2') }) -join ''`.
@@solution
function Get-Sha256([string]$Text) {
    $bytes = [Text.Encoding]::UTF8.GetBytes($Text)
    $hash = [System.Security.Cryptography.SHA256]::Create().ComputeHash($bytes)
    return ($hash | ForEach-Object { $_.ToString('x2') }) -join ''
}
