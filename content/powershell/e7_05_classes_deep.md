@@lesson e7-classes Classes in Depth: Inheritance, Interfaces and Enums
@@concepts objects, modules
@@text
Module 5 introduced classes. This lesson covers what you need to model real systems: inheritance, overriding, static members, interfaces that plug into .NET (sorting, equality, formatting) and flag enums.

## Inheritance and overriding

```powershell
class Device {
    [string]$Name
    Device([string]$name) { $this.Name = $name }
    [string] Describe() { return "device $($this.Name)" }
}
class Printer : Device {
    [int]$PagesLeft
    Printer([string]$name, [int]$pages) : base($name) { $this.PagesLeft = $pages }
    [string] Describe() { return "printer $($this.Name), $($this.PagesLeft) pages left" }
}
$all = [Device]::new('nas01'), [Printer]::new('prn-3f', 120)
$all | ForEach-Object { $_.Describe() }
```

- `: base(...)` calls the parent constructor.
- A method with the same signature **overrides** the parent's; calls go to the most specific version (polymorphism).
- Methods must declare their return type and use `return`; unlike functions, only the returned value is output.

## Static members and hidden properties

```powershell
class IdGen {
    static [int]$Next = 1
    hidden [string]$Secret = 'not shown by default'
    static [string] New() { $id = 'ID-{0:D4}' -f [IdGen]::Next; [IdGen]::Next++; return $id }
}
[IdGen]::New(); [IdGen]::New()
```

`hidden` members don't appear in `Get-Member` or default output, but they're still accessible: it's tidiness, not security.

## Interfaces: plugging into .NET

Implementing a .NET interface lets your objects work with .NET features:

- `System.IComparable`: `CompareTo($other)` makes `Sort-Object` and `[array]::Sort` order your objects naturally.
- `System.IEquatable[T]` and overriding `Equals` / `GetHashCode`: two objects with the same data count as equal (in `-eq`, `HashSet`, `Dictionary` keys).
- Overriding `ToString()`: controls how the object looks in strings and `-join`.

## Enums and flags

```powershell
enum Severity { Info = 1; Warning = 2; Critical = 3 }
[Severity]'Warning' -lt [Severity]::Critical       # True: enums compare by value

[Flags()] enum Rights { None = 0; Read = 1; Write = 2; Delete = 4; Admin = 7 }
$r = [Rights]'Read, Write'
"$r, has Write: $($r.HasFlag([Rights]::Write)), value $([int]$r)"
```

A `[Flags()]` enum combines values with `-bor` and prints combinations as names. NTFS rights, AD's userAccountControl and file attributes all work like this.

## Validation in classes

Property attributes work like parameter attributes: `[ValidateRange(1, 65535)][int]$Port`, `[ValidateNotNullOrEmpty()][string]$Name`. Assigning an invalid value throws.

## Classes and modules: the catch

A class defined in a module (`.psm1`) is **not** exported by `Import-Module`. Load it with `using module MyModule` at the top of the script. Also, in 5.1 you can't redefine a class in the same session after changing it: start a new session while developing.

@@quiz
What does `: base($name)` in a constructor do?
* Calls the parent class's constructor with $name
- Creates a new base object
- Makes the class abstract
- Imports a module
? Initialise the inherited part first.

@@quiz
Why implement `System.IComparable` on a class?
- To make it serialisable
* So sorting (`Sort-Object`, `[array]::Sort`) knows the natural order of your objects
- To hide properties
- It's required for classes
? CompareTo returns negative, zero or positive.

@@quiz
`[Flags()] enum Rights { Read = 1; Write = 2; Delete = 4 }`. What is `[int][Rights]'Read, Delete'`?
- 2
* 5
- 1
- 7
? Flags combine bits: 1 + 4.

@@drill
Which attribute on an enum lets its values be combined as bit flags?
= [Flags()]
= [Flags]
= Flags
? [Flags()] enum Perm { Read = 1; Write = 2 }.

@@flashcards
`class B : A` :: inheritance
`: base(...)` :: call the parent constructor
override :: same method signature in the child
`static` :: belongs to the class, not instances
`hidden` :: not shown by default
`IComparable` :: natural sort order
`Equals` / `GetHashCode` :: value equality
`ToString()` :: string representation
`enum` :: named constants
`[Flags()]` :: combinable bit values
`.HasFlag()` :: test a flag
`using module` :: import classes from a module

@@exercise A device hierarchy
Model the IT inventory:

- `Device` (Name, Location) with `[string] Describe()` and a `ToString()` override returning the name
- `Server : Device` with `[ValidateRange(1, 256)][int]$Cores` and an overridden `Describe()`
- `Printer : Device` with `PagesLeft`; `Describe()` adds ` (LOW TONER)` when under 50 pages
- a static `[Device]::Count` incremented by every constructor

Print the descriptions, the names joined with `-join` (uses ToString), the count, and show that `Cores = 999` is rejected.
@@starter
'TODO'
@@expected
server web01 in Tokyo-DC1, 16 cores
printer prn-2f in Osaka office (LOW TONER)
printer prn-3f in Osaka office
devices: web01, prn-2f, prn-3f
created: 3
invalid cores rejected: True
@@requires
: Device
: base(
static [int]$Count
ValidateRange
@@hint
In `Device`'s constructor: `[Device]::Count++`. Child constructors call `: base($name, $location)`, which runs that code. Override `[string] ToString() { return $this.Name }`. To test validation: `try { $s.Cores = 999; $false } catch { $true }`.
@@solution
class Device {
    static [int]$Count = 0
    [string]$Name
    [string]$Location
    Device([string]$name, [string]$location) {
        $this.Name = $name
        $this.Location = $location
        [Device]::Count++
    }
    [string] Describe() { return "device $($this.Name) in $($this.Location)" }
    [string] ToString() { return $this.Name }
}

class Server : Device {
    [ValidateRange(1, 256)][int]$Cores
    Server([string]$name, [string]$location, [int]$cores) : base($name, $location) { $this.Cores = $cores }
    [string] Describe() { return "server $($this.Name) in $($this.Location), $($this.Cores) cores" }
}

class Printer : Device {
    [int]$PagesLeft
    Printer([string]$name, [string]$location, [int]$pages) : base($name, $location) { $this.PagesLeft = $pages }
    [string] Describe() {
        $text = "printer $($this.Name) in $($this.Location)"
        if ($this.PagesLeft -lt 50) { $text += ' (LOW TONER)' }
        return $text
    }
}

$devices = @(
    [Server]::new('web01', 'Tokyo-DC1', 16)
    [Printer]::new('prn-2f', 'Osaka office', 12)
    [Printer]::new('prn-3f', 'Osaka office', 480)
)
$devices | ForEach-Object { $_.Describe() }
"devices: $($devices -join ', ')"
"created: $([Device]::Count)"
$rejected = try { $devices[0].Cores = 999; $false } catch { $true }
"invalid cores rejected: $rejected"

@@exercise Sortable, comparable versions
Write a `SemVer` class:

- parse `MAJOR.MINOR.PATCH[-PRE]` in a constructor (throw `ArgumentException` on bad input)
- implement `System.IComparable` so versions sort numerically, and a pre-release sorts **before** its release (`2.0.0-rc.1 < 2.0.0`; compare pre-release strings ordinally)
- override `Equals`, `GetHashCode` and `ToString` so equal versions de-duplicate in a `HashSet[object]` and `-eq` works

Also define a `[Flags()]` enum `Channel { None = 0; Stable = 1; Beta = 2; Nightly = 4 }` and print which channels a build is published to.
@@starter
$raw = '1.10.0', '1.9.3', '2.0.0-rc.1', '2.0.0', '1.9.3', '0.9.12', '2.0.0-beta.2'
'TODO'
@@expected
sorted: 0.9.12, 1.9.3, 1.9.3, 1.10.0, 2.0.0-beta.2, 2.0.0-rc.1, 2.0.0
unique: 6
newest: 2.0.0
1.9.3 -eq 1.9.3: True
rejected: 'v1' is not a semantic version
channels: Stable, Beta (3), nightly: False
@@requires
System.IComparable
CompareTo
GetHashCode
[Flags()]
@@hint
`[int] CompareTo([object]$other) { $o = [SemVer]$other; foreach ($p in 'Major','Minor','Patch') { if ($this.$p -ne $o.$p) { return $this.$p.CompareTo($o.$p) } }; ... }`. For pre-releases: no pre-release beats any pre-release; otherwise `[string]::CompareOrdinal($this.Pre, $o.Pre)`. `Equals` can reuse `CompareTo(...) -eq 0`; `GetHashCode` can return `$this.ToString().GetHashCode()`. Sort with `Sort-Object` (it uses IComparable).
@@solution
$raw = '1.10.0', '1.9.3', '2.0.0-rc.1', '2.0.0', '1.9.3', '0.9.12', '2.0.0-beta.2'

class SemVer : System.IComparable {
    [int]$Major
    [int]$Minor
    [int]$Patch
    [string]$Pre = ''

    SemVer([string]$text) {
        if ($text -notmatch '^(\d+)\.(\d+)\.(\d+)(?:-([0-9A-Za-z.-]+))?$') {
            throw [System.ArgumentException]::new("'$text' is not a semantic version")
        }
        $this.Major = [int]$Matches[1]
        $this.Minor = [int]$Matches[2]
        $this.Patch = [int]$Matches[3]
        if ($Matches[4]) { $this.Pre = $Matches[4] }
    }

    [int] CompareTo([object]$other) {
        $o = [SemVer]$other
        foreach ($p in 'Major', 'Minor', 'Patch') {
            if ($this.$p -ne $o.$p) { return $this.$p.CompareTo($o.$p) }
        }
        if ($this.Pre -eq $o.Pre) { return 0 }
        if (-not $this.Pre) { return 1 }
        if (-not $o.Pre) { return -1 }
        return [string]::CompareOrdinal($this.Pre, $o.Pre)
    }

    [bool] Equals([object]$other) {
        if ($other -isnot [SemVer]) { return $false }
        return $this.CompareTo($other) -eq 0
    }

    [int] GetHashCode() { return $this.ToString().GetHashCode() }

    [string] ToString() {
        $s = "$($this.Major).$($this.Minor).$($this.Patch)"
        if ($this.Pre) { $s += "-$($this.Pre)" }
        return $s
    }
}

[Flags()] enum Channel { None = 0; Stable = 1; Beta = 2; Nightly = 4 }

$versions = $raw | ForEach-Object { [SemVer]::new($_) }
$sorted = $versions | Sort-Object
"sorted: $($sorted -join ', ')"
$set = [System.Collections.Generic.HashSet[object]]::new()
foreach ($v in $versions) { [void]$set.Add($v) }
"unique: $($set.Count)"
"newest: $($sorted[-1])"
"1.9.3 -eq 1.9.3: $([SemVer]::new('1.9.3') -eq [SemVer]::new('1.9.3'))"
try { [SemVer]::new('v1') | Out-Null } catch { "rejected: $($_.Exception.GetBaseException().Message)" }
$published = [Channel]::Stable -bor [Channel]::Beta
"channels: $published ($([int]$published)), nightly: $($published.HasFlag([Channel]::Nightly))"
