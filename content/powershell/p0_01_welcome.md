@@lesson p0-welcome Welcome to PowerShell
@@concepts hello
@@text
**PowerShell** is the scripting language and shell built into Windows. If you work in IT anywhere Windows is used (help desk, desktop support, sysadmin, Microsoft 365 or Azure admin), you'll use it. Job postings in Japan and everywhere else list it constantly: *"PowerShell scripting experience"*.

What makes it special: commands don't return plain **text**, they return **objects**. A process is an object with a `Name`, an `Id` and a `CPU` time. A file is an object with a `Length` and a `LastWriteTime`. You can sort, filter and export them without cutting text apart.

This course uses **Windows PowerShell 5.1**, the version installed on every Windows 10 and 11 PC and every Windows Server, so your scripts run on the machines you'll actually manage. (PowerShell 7 is the newer cross-platform version; almost everything here works there too.)

## Your first command

```powershell
Write-Output "Hello, IT world!"
Write-Output "PowerShell is version $($PSVersionTable.PSVersion)"
```

Press **▶ Try it**. `Write-Output` sends something to the output. Inside **double quotes**, `$variable` is replaced by its value; `$( ... )` runs an expression inside the string.

## Verb-Noun

Every command (a **cmdlet**) is named **Verb-Noun**: `Get-Process`, `Stop-Service`, `New-Item`, `Restart-Computer`. The verbs come from a short approved list (`Get`, `Set`, `New`, `Remove`, `Start`, `Stop`, `Test`, ...), so you can often *guess* a command's name.

```powershell
Get-Date
Get-Command -Verb Get -Noun *Service*
```

## The three commands that teach you everything else

| Command | What it does |
|---|---|
| `Get-Command *network*` | find commands by name |
| `Get-Help Get-Process -Examples` | examples of how to use one |
| `Get-Process \| Get-Member` | what properties and methods its objects have |

In an interactive window, `Update-Help` (as administrator) downloads the full help once.

## Aliases

Familiar names work too: `ls` and `dir` are aliases for `Get-ChildItem`, `cd` for `Set-Location`, `cat` for `Get-Content`. Aliases are fine at the prompt, but **write full names in scripts**, so the next person can read them.

@@quiz
What is the naming pattern of PowerShell cmdlets?
- noun_verb, like `process_get`
* Verb-Noun, like `Get-Process`
- whatever the author liked
- all lowercase, like `getprocess`
? Cmdlets use an approved **Verb** and a singular **Noun**: `Get-Service`, `Stop-Service`, `Restart-Service`.

@@quiz
You want to know what properties the objects from `Get-Service` have. Which command?
- `Get-Help Get-Service`
* `Get-Service | Get-Member`
- `Get-Command Get-Service`
- `Show-Properties Get-Service`
? `Get-Member` (alias `gm`) lists the type, properties and methods of whatever is piped into it.

@@flashcards
Get-Command :: Find commands, for example `Get-Command *dns*`
Get-Help :: Show a command's help; add `-Examples` for examples
Get-Member :: Show the properties and methods of the objects in the pipeline
cmdlet :: A PowerShell command named Verb-Noun, such as `Get-Process`
Windows PowerShell 5.1 :: The version built into Windows 10, 11 and Windows Server
alias :: A short name for a cmdlet, like `ls` for `Get-ChildItem`

@@exercise Hello, admin
Print exactly two lines: `Hello from PowerShell` and then `Ready to automate`.
@@starter
# Use Write-Output twice.

@@expected
Hello from PowerShell
Ready to automate
@@hint
`Write-Output "Hello from PowerShell"` on the first line, the second message on the next.
@@solution
Write-Output "Hello from PowerShell"
Write-Output "Ready to automate"

@@exercise Inside a string
The variable `$computer` is given. Print `Connecting to SRV-FILE01...` by putting the variable **inside** a double-quoted string.
@@starter
$computer = "SRV-FILE01"

@@expected
Connecting to SRV-FILE01...
@@requires
"Connecting to $computer
@@hint
`Write-Output "Connecting to $computer..."`
@@solution
$computer = "SRV-FILE01"
Write-Output "Connecting to $computer..."
