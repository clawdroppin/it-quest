@@lesson q3-asset-tracker IT Asset Tracker: a Module and a Desktop App
@@text
Every IT department tracks its hardware somewhere: who has which laptop, what's in the cupboard, what's away for repair, and whose warranty is about to run out. Small teams often use a spreadsheet that nobody trusts. You'll build the real thing: a **PowerShell module** that other scripts can use, with tests, and a **Windows desktop app** on top of it.

```text
PS> Import-Module .\AssetTracker.psd1
PS> Open-AssetStore -Path .\assets.json
PS> Add-Asset -Type Laptop -Model 'ThinkPad T14' -Serial PF3A9K01 -PurchaseDate 2023-12-01

Tag     Type   Model        Serial   Status  AssignedTo
---     ----   -----        ------   ------  ----------
LT-0001 Laptop ThinkPad T14 PF3A9K01 InStock

PS> Get-Asset -Type Laptop -Status InStock | Select-Object -First 1 | Set-AssetOwner -User yuki
PS> Get-WarrantyReport -Days 60

Tag     Model        AssignedTo WarrantyEnds DaysLeft
---     -----        ---------- ------------ --------
LT-0001 ThinkPad T14 yuki       2026-12-01         30
```

## How it's built

| File | Job |
|---|---|
| `AssetTracker.psd1` | the **manifest**: version, author, and the list of exported commands |
| `AssetTracker.psm1` | the module: loads `Assets.ps1` |
| `Assets.ps1` | all the logic. A JSON store kept in `$script:` variables, saved after every change |
| `tests.ps1` | 31 tests, in the same style as the User Onboarding project |
| `Show-AssetTracker.ps1` | a WinForms app: search box, grid, and buttons that call the module |

The design points from this module's lessons are all here: approved verbs, `[CmdletBinding(SupportsShouldProcess)]` so `-WhatIf` works on every change, validation attributes instead of hand-written checks, pipeline input by property name, an atomic save (write a temp file, then rename), and a GUI that is only a thin layer over tested functions.

## Your tasks

Four functions in `Assets.ps1` throw `TODO`. The comment above each one is its specification:

1. `New-AssetTag`: find the highest number for the type's prefix with `-match` and `$Matches`, add one, and format with `'{0}-{1:D4}'`. Watch out: `-f` with `D4` needs an `[int]`.
2. `Get-Asset`: one `Where-Object` with all the filters; a filter that wasn't given matches everything.
3. `Set-AssetOwner`: the rules, then `$PSCmdlet.ShouldProcess(...)`, then change the object, add history, and save. The `process` block runs once per piped asset.
4. `Get-WarrantyReport`: `([datetime]$a.PurchaseDate).AddMonths(...)`, the days left, and `Sort-Object DaysLeft, Tag`.

Run the tests after each one:

```text
powershell -ExecutionPolicy Bypass -File tests.ps1
```

When they all pass, start the app with `powershell -ExecutionPolicy Bypass -File Show-AssetTracker.ps1`.
@@project asset_tracker IT Asset Tracker
Write the four functions that throw `TODO` in `Assets.ps1` until `tests.ps1` passes, then run `Show-AssetTracker.ps1`.
@@file AssetTracker.psd1
@{
    RootModule        = 'AssetTracker.psm1'
    ModuleVersion     = '1.0.0'
    GUID              = '6f3b2a1e-8c4d-4e7a-9b15-2d0c7e9a4f11'
    Author            = 'IT Quest'
    Description       = 'Track laptops, desktops, monitors and phones: who has what, and when warranties end.'
    PowerShellVersion = '5.1'
    FunctionsToExport = @(
        'Open-AssetStore', 'New-AssetTag', 'Add-Asset', 'Get-Asset', 'Set-AssetOwner',
        'Set-AssetStatus', 'Get-WarrantyReport', 'Export-AssetReport'
    )
    CmdletsToExport   = @()
    VariablesToExport = @()
    AliasesToExport   = @()
}
@@file AssetTracker.psm1
# The module file: loads the code. The manifest (AssetTracker.psd1) decides what is exported.
. (Join-Path $PSScriptRoot 'Assets.ps1')
@@file Assets.ps1
# Assets.ps1: the logic of the AssetTracker module. AssetTracker.psm1 loads this file.
# The store is a JSON file; every change is saved straight away.

$script:Assets = [System.Collections.Generic.List[object]]::new()
$script:StorePath = $null
$script:Prefixes = @{ Laptop = 'LT'; Desktop = 'DT'; Monitor = 'MN'; Phone = 'PH' }

function Open-AssetStore {
    # Loads the store from Path (an empty store if the file doesn't exist yet).
    [CmdletBinding()]
    param([Parameter(Mandatory)][string]$Path)
    $script:StorePath = $Path
    $script:Assets = [System.Collections.Generic.List[object]]::new()
    if (Test-Path -LiteralPath $Path) {
        $data = Get-Content -LiteralPath $Path -Raw -Encoding UTF8 | ConvertFrom-Json
        foreach ($a in @($data)) {
            if ($null -ne $a) { $script:Assets.Add($a) }
        }
    }
    Write-Verbose "Loaded $($script:Assets.Count) asset(s) from $Path"
}

function Save-AssetStore {
    # Writes to a temporary file first, then replaces the store, so a crash never leaves half a file.
    if (-not $script:StorePath) { throw 'No store is open. Run Open-AssetStore first.' }
    $json = ConvertTo-Json -InputObject @($script:Assets) -Depth 5
    $tmp = "$script:StorePath.tmp"
    [System.IO.File]::WriteAllText($tmp, $json, [System.Text.UTF8Encoding]::new($false))
    Move-Item -LiteralPath $tmp -Destination $script:StorePath -Force
}

function Add-History($Asset, [string]$Action) {
    $entry = '{0} {1}' -f (Get-Date -Format 'yyyy-MM-dd'), $Action
    $Asset.History = @(@($Asset.History | Where-Object { $_ }) + $entry)
}

function Find-AssetByTag([string]$Tag) {
    $asset = $script:Assets | Where-Object Tag -eq $Tag | Select-Object -First 1
    if (-not $asset) { throw "No asset with tag $Tag" }
    $asset
}

function New-AssetTag {
    # The next free tag for a type: its prefix, a dash, and the number after the highest one in use,
    # with 4 digits. Laptop -> LT-0001, LT-0002, ...  Desktop DT, Monitor MN, Phone PH.
    # Tags of other types don't count, and gaps aren't reused (LT-0001, LT-0007 -> LT-0008).
    param(
        [Parameter(Mandatory)][ValidateSet('Laptop', 'Desktop', 'Monitor', 'Phone')][string]$Type,
        [string[]]$Existing = @()
    )
    throw "TODO: not done yet"
}

function Add-Asset {
    # Registers a new asset as InStock and returns it.
    [CmdletBinding(SupportsShouldProcess)]
    param(
        [Parameter(Mandatory)][ValidateSet('Laptop', 'Desktop', 'Monitor', 'Phone')][string]$Type,
        [Parameter(Mandatory)][string]$Model,
        [Parameter(Mandatory)][ValidatePattern('^[A-Z0-9-]{4,}$')][string]$Serial,
        [datetime]$PurchaseDate = (Get-Date).Date,
        [ValidateRange(0, 120)][int]$WarrantyMonths = 36
    )
    if ($script:Assets | Where-Object Serial -eq $Serial) { throw "Serial $Serial is already registered" }
    $tag = New-AssetTag -Type $Type -Existing @(foreach ($a in $script:Assets) { $a.Tag })
    if ($PSCmdlet.ShouldProcess($tag, "Add $Type $Model ($Serial)")) {
        $asset = [pscustomobject]@{
            Tag            = $tag
            Type           = $Type
            Model          = $Model
            Serial         = $Serial
            Status         = 'InStock'
            AssignedTo     = $null
            PurchaseDate   = $PurchaseDate.ToString('yyyy-MM-dd')
            WarrantyMonths = $WarrantyMonths
            History        = @()
        }
        Add-History $asset 'added'
        $script:Assets.Add($asset)
        Save-AssetStore
        $asset
    }
}

function Get-Asset {
    # Returns the assets that match every filter given. -Tag accepts wildcards (LT-*).
    # -AssignedTo matches the user exactly, ignoring case.
    [CmdletBinding()]
    param(
        [string]$Tag = '*',
        [ValidateSet('Laptop', 'Desktop', 'Monitor', 'Phone')][string]$Type,
        [ValidateSet('InStock', 'Assigned', 'Repair', 'Retired')][string]$Status,
        [string]$AssignedTo
    )
    throw "TODO: not done yet"
}

function Set-AssetOwner {
    # Assigns assets to a user (Status becomes Assigned) and records "assigned to <user>" in the history.
    # Takes tags from the pipeline, so: Get-Asset -Type Laptop -Status InStock | Select -First 1 | Set-AssetOwner -User yuki
    # Retired assets and assets in Repair can't be assigned: throw "<tag> is <status>".
    # Assigning to the user who already has it changes nothing (no history entry).
    [CmdletBinding(SupportsShouldProcess)]
    param(
        [Parameter(Mandatory, ValueFromPipelineByPropertyName)][string]$Tag,
        [Parameter(Mandatory)][string]$User
    )
    process {
        throw "TODO: not done yet"
    }
}

function Set-AssetStatus {
    # Moves assets to InStock (returned), Repair or Retired. Anything but Repair clears the owner.
    [CmdletBinding(SupportsShouldProcess)]
    param(
        [Parameter(Mandatory, ValueFromPipelineByPropertyName)][string]$Tag,
        [Parameter(Mandatory)][ValidateSet('InStock', 'Repair', 'Retired')][string]$Status
    )
    process {
        $asset = Find-AssetByTag $Tag
        if ($asset.Status -eq 'Retired') { throw "$Tag is Retired" }
        if ($PSCmdlet.ShouldProcess($Tag, "Set status to $Status")) {
            $asset.Status = $Status
            if ($Status -ne 'Repair') { $asset.AssignedTo = $null }
            Add-History $asset ($Status.ToLower())
            Save-AssetStore
        }
    }
}

function Get-WarrantyReport {
    # Assets (not Retired) whose warranty ends within -Days of -AsOf, or has already ended.
    # The warranty ends PurchaseDate + WarrantyMonths. Output objects with Tag, Model, AssignedTo,
    # WarrantyEnds (yyyy-MM-dd) and DaysLeft (negative = expired), sorted by DaysLeft, then Tag.
    [CmdletBinding()]
    param(
        [datetime]$AsOf = (Get-Date),
        [int]$Days = 90
    )
    throw "TODO: not done yet"
}

function Export-AssetReport {
    # Writes an HTML page with every asset and the warranty report. ConvertTo-Html encodes the text.
    [CmdletBinding()]
    param([Parameter(Mandatory)][string]$Path)
    $style = '<style>body{font-family:Segoe UI,sans-serif;margin:2em}table{border-collapse:collapse}' +
        'td,th{border:1px solid #ccc;padding:4px 8px}th{background:#eef}</style>'
    $all = Get-Asset | Select-Object Tag, Type, Model, Serial, Status, AssignedTo |
        ConvertTo-Html -Fragment -PreContent '<h2>All assets</h2>'
    $warranty = Get-WarrantyReport | ConvertTo-Html -Fragment -PreContent '<h2>Warranty ending within 90 days</h2>'
    ConvertTo-Html -Title 'Asset report' -Head $style -Body ($all + $warranty) |
        Set-Content -LiteralPath $Path -Encoding UTF8
    Get-Item -LiteralPath $Path
}
@@file tests.ps1
# Run: powershell -ExecutionPolicy Bypass -File tests.ps1     (or: pwsh -File tests.ps1)
Import-Module (Join-Path $PSScriptRoot 'AssetTracker.psd1') -Force
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

function Test-Throws([string]$Name, [string]$Like, [scriptblock]$Body) {
    try { & $Body; Write-Host "  FAIL  $Name : no error" -ForegroundColor Red; $script:failed++ }
    catch {
        if ($_.Exception.Message -like $Like) { Write-Host "  ok    $Name" -ForegroundColor Green }
        else { Write-Host "  FAIL  $Name : $($_.Exception.Message)" -ForegroundColor Red; $script:failed++ }
    }
}

$store = Join-Path ([System.IO.Path]::GetTempPath()) ("assets-test-{0}.json" -f [guid]::NewGuid())

Write-Host 'Tags'
Test-That 'first tag' { (New-AssetTag -Type Laptop) -ceq 'LT-0001' }
Test-That 'after the highest' { (New-AssetTag -Type Laptop -Existing 'LT-0001', 'LT-0007', 'LT-0003') -ceq 'LT-0008' }
Test-That 'other types ignored' { (New-AssetTag -Type Monitor -Existing 'LT-0004', 'MN-0002', 'PH-0009') -ceq 'MN-0003' }
Test-That 'four digits' { (New-AssetTag -Type Phone -Existing 'PH-0099') -ceq 'PH-0100' }

Write-Host 'Adding and finding'
Open-AssetStore -Path $store
$lt1 = Add-Asset -Type Laptop -Model 'ThinkPad T14' -Serial 'PF3A9K01' -PurchaseDate '2023-12-01' -WarrantyMonths 36
$lt2 = Add-Asset -Type Laptop -Model 'ThinkPad T14' -Serial 'PF3A9K02' -PurchaseDate '2024-06-15' -WarrantyMonths 36
$mn1 = Add-Asset -Type Monitor -Model 'Dell U2724D' -Serial 'CN0X7Y21' -PurchaseDate '2021-03-10' -WarrantyMonths 60
$ph1 = Add-Asset -Type Phone -Model 'iPhone 15' -Serial 'F2LZQ9XK' -PurchaseDate '2024-11-20' -WarrantyMonths 12
Test-That 'tags are given out' { $lt1.Tag -eq 'LT-0001' -and $lt2.Tag -eq 'LT-0002' -and $mn1.Tag -eq 'MN-0001' }
Test-That 'new assets are in stock' { $lt1.Status -eq 'InStock' -and $null -eq $lt1.AssignedTo }
Test-Throws 'duplicate serial' '*already registered*' { Add-Asset -Type Laptop -Model 'X' -Serial 'PF3A9K01' }
Test-That '-WhatIf adds nothing' {
    Add-Asset -Type Desktop -Model 'OptiPlex' -Serial 'WHATIF01' -WhatIf
    @(Get-Asset).Count -eq 4
}
Test-That 'all' { @(Get-Asset).Count -eq 4 }
Test-That 'by type' { @(Get-Asset -Type Laptop).Count -eq 2 }
Test-That 'by tag wildcard' { (@(Get-Asset -Tag 'MN-*') | ForEach-Object Tag) -join ',' -eq 'MN-0001' }
Test-That 'by exact tag' { (Get-Asset -Tag 'LT-0002').Serial -eq 'PF3A9K02' }
Test-That 'filters combine' { @(Get-Asset -Type Laptop -Status Assigned).Count -eq 0 }

Write-Host 'Assigning'
Get-Asset -Type Laptop -Status InStock | Select-Object -First 1 | Set-AssetOwner -User 'yuki'
Test-That 'from the pipeline' { (Get-Asset -Tag 'LT-0001').AssignedTo -eq 'yuki' -and (Get-Asset -Tag 'LT-0001').Status -eq 'Assigned' }
Test-That 'history' { @((Get-Asset -Tag 'LT-0001').History)[-1] -like '*assigned to yuki' }
Set-AssetOwner -Tag 'LT-0001' -User 'yuki'
Test-That 'same user: no new history' { @((Get-Asset -Tag 'LT-0001').History).Count -eq 2 }
Test-That 'by user, any case' { (Get-Asset -AssignedTo 'YUKI').Tag -eq 'LT-0001' }
Set-AssetOwner -Tag 'MN-0001' -User 'ken' -WhatIf
Test-That '-WhatIf changes nothing' { $null -eq (Get-Asset -Tag 'MN-0001').AssignedTo }
Set-AssetStatus -Tag 'LT-0002' -Status Repair
Test-Throws 'not while in repair' '*LT-0002 is Repair*' { Set-AssetOwner -Tag 'LT-0002' -User 'ken' }
Set-AssetStatus -Tag 'PH-0001' -Status Retired
Test-Throws 'not when retired' '*PH-0001 is Retired*' { Set-AssetOwner -Tag 'PH-0001' -User 'ken' }
Test-Throws 'unknown tag' '*No asset with tag XX-0001*' { Set-AssetOwner -Tag 'XX-0001' -User 'ken' }

Write-Host 'Warranty report'
$report = @(Get-WarrantyReport -AsOf '2026-11-01' -Days 60)
Test-That 'expired and soon, not retired' { ($report | ForEach-Object Tag) -join ',' -eq 'MN-0001,LT-0001' }
Test-That 'end date' { $report[1].WarrantyEnds -eq '2026-12-01' -and $report[1].DaysLeft -eq 30 }
Test-That 'expired is negative' { $report[0].WarrantyEnds -eq '2026-03-10' -and $report[0].DaysLeft -eq -236 }
Test-That 'owner shown' { $report[1].AssignedTo -eq 'yuki' }
Test-That 'nothing within 0 days but the expired one' { @(Get-WarrantyReport -AsOf '2026-11-01' -Days 0).Count -eq 1 }

Write-Host 'Saving'
Open-AssetStore -Path $store
Test-That 'reloaded from disk' { @(Get-Asset).Count -eq 4 -and (Get-Asset -Tag 'LT-0001').AssignedTo -eq 'yuki' }
Test-That 'history survives' { @((Get-Asset -Tag 'LT-0001').History).Count -eq 2 }
$html = Join-Path ([System.IO.Path]::GetTempPath()) ("assets-test-{0}.html" -f [guid]::NewGuid())
Test-That 'HTML report' { (Export-AssetReport -Path $html) -and ((Get-Content $html -Raw) -match 'ThinkPad T14') }
Remove-Item $store, $html -ErrorAction SilentlyContinue

if ($script:failed -gt 0) { Write-Host "$script:failed test(s) failed" -ForegroundColor Red; exit 1 }
Write-Host 'All tests passed' -ForegroundColor Green
@@file Show-AssetTracker.ps1
# A Windows desktop app on top of the module: a searchable grid, and buttons for the common jobs.
# Run: powershell -ExecutionPolicy Bypass -File Show-AssetTracker.ps1 [-Path assets.json]
param([string]$Path = (Join-Path $PSScriptRoot 'assets.json'))

Import-Module (Join-Path $PSScriptRoot 'AssetTracker.psd1') -Force
Add-Type -AssemblyName System.Windows.Forms, System.Drawing, Microsoft.VisualBasic
[System.Windows.Forms.Application]::EnableVisualStyles()
Open-AssetStore -Path $Path

$form = New-Object System.Windows.Forms.Form
$form.Text = 'Asset Tracker'
$form.Size = New-Object System.Drawing.Size(900, 520)
$form.StartPosition = 'CenterScreen'
$form.Font = New-Object System.Drawing.Font('Segoe UI', 10)

$search = New-Object System.Windows.Forms.TextBox
$search.Location = New-Object System.Drawing.Point(12, 12)
$search.Width = 260

$grid = New-Object System.Windows.Forms.DataGridView
$grid.Location = New-Object System.Drawing.Point(12, 48)
$grid.Size = New-Object System.Drawing.Size(860, 380)
$grid.Anchor = 'Top, Bottom, Left, Right'
$grid.ReadOnly = $true
$grid.AllowUserToAddRows = $false
$grid.SelectionMode = 'FullRowSelect'
$grid.AutoSizeColumnsMode = 'Fill'

function Update-Grid {
    $text = $search.Text.Trim()
    $rows = Get-Asset | Where-Object {
        -not $text -or "$($_.Tag) $($_.Model) $($_.Serial) $($_.AssignedTo) $($_.Status)" -like "*$text*"
    } | Select-Object Tag, Type, Model, Serial, Status, AssignedTo, PurchaseDate
    $table = New-Object System.Data.DataTable
    foreach ($col in 'Tag', 'Type', 'Model', 'Serial', 'Status', 'AssignedTo', 'PurchaseDate') { [void]$table.Columns.Add($col) }
    foreach ($r in $rows) { [void]$table.Rows.Add($r.Tag, $r.Type, $r.Model, $r.Serial, $r.Status, $r.AssignedTo, $r.PurchaseDate) }
    $grid.DataSource = $table
}

function Get-SelectedTag {
    if ($grid.SelectedRows.Count -eq 0) { return $null }
    $grid.SelectedRows[0].Cells['Tag'].Value
}

function Invoke-Safely([scriptblock]$Action) {
    # A GUI must never crash on a bad click: show the error instead.
    try { & $Action; Update-Grid }
    catch { [void][System.Windows.Forms.MessageBox]::Show($_.Exception.Message, 'Asset Tracker', 'OK', 'Warning') }
}

function New-Button([string]$Text, [int]$X, [scriptblock]$OnClick) {
    $b = New-Object System.Windows.Forms.Button
    $b.Text = $Text
    $b.Location = New-Object System.Drawing.Point($X, 440)
    $b.Size = New-Object System.Drawing.Size(130, 32)
    $b.Anchor = 'Bottom, Left'
    $b.Add_Click($OnClick)
    $form.Controls.Add($b)
}

New-Button 'Add...' 12 {
    Invoke-Safely {
        $type = [Microsoft.VisualBasic.Interaction]::InputBox('Type (Laptop, Desktop, Monitor, Phone):', 'Add asset', 'Laptop')
        if (-not $type) { return }
        $model = [Microsoft.VisualBasic.Interaction]::InputBox('Model:', 'Add asset')
        $serial = [Microsoft.VisualBasic.Interaction]::InputBox('Serial number:', 'Add asset')
        if ($model -and $serial) { [void](Add-Asset -Type $type -Model $model -Serial $serial.ToUpper()) }
    }
}
New-Button 'Assign...' 152 {
    Invoke-Safely {
        $tag = Get-SelectedTag
        if (-not $tag) { return }
        $user = [Microsoft.VisualBasic.Interaction]::InputBox("Assign $tag to user:", 'Assign')
        if ($user) { Set-AssetOwner -Tag $tag -User $user }
    }
}
New-Button 'Return to stock' 292 { Invoke-Safely { $t = Get-SelectedTag; if ($t) { Set-AssetStatus -Tag $t -Status InStock } } }
New-Button 'Send to repair' 432 { Invoke-Safely { $t = Get-SelectedTag; if ($t) { Set-AssetStatus -Tag $t -Status Repair } } }
New-Button 'HTML report' 572 {
    Invoke-Safely {
        $file = Export-AssetReport -Path (Join-Path ([System.IO.Path]::GetTempPath()) 'asset-report.html')
        Start-Process $file.FullName
    }
}

$search.Add_TextChanged({ Update-Grid })
$form.Controls.AddRange(@($search, $grid))
$form.Add_Shown({ Update-Grid; $search.Focus() })
[void]$form.ShowDialog()
@@text
## Stretch goals

- **CSV import**: `Import-AssetCsv` that reads the purchasing department's export and calls `Add-Asset` for each row, with `-WhatIf` support for free (it inherits `$WhatIfPreference`).
- **Inventory from the network**: fill in Model and Serial automatically with `Get-CimInstance Win32_ComputerSystem` and `Win32_BIOS` (run against remote PCs with `-ComputerName` or `Invoke-Command`).
- **Pester**: rewrite `tests.ps1` as `AssetTracker.Tests.ps1` with `Describe`/`It`/`Should`, and use `Mock Get-Date` to test the history dates.
- **Shared store**: put `assets.json` on a file share, and handle two people saving at once (a lock file, or compare a version number before saving and refuse if someone else saved first).
- **SQL instead of JSON**: the same commands on top of SQLite or SQL Server. The tests shouldn't need to change, which is the point of hiding the storage behind functions.
- **Publish it**: put the module in a folder on `$env:PSModulePath`, or publish it to an internal repository with `Publish-Module`.
@@project asset_tracker_finished IT Asset Tracker: the finished version
A complete solution to compare with yours.
@@file AssetTracker.psd1
@{
    RootModule        = 'AssetTracker.psm1'
    ModuleVersion     = '1.0.0'
    GUID              = '6f3b2a1e-8c4d-4e7a-9b15-2d0c7e9a4f11'
    Author            = 'IT Quest'
    Description       = 'Track laptops, desktops, monitors and phones: who has what, and when warranties end.'
    PowerShellVersion = '5.1'
    FunctionsToExport = @(
        'Open-AssetStore', 'New-AssetTag', 'Add-Asset', 'Get-Asset', 'Set-AssetOwner',
        'Set-AssetStatus', 'Get-WarrantyReport', 'Export-AssetReport'
    )
    CmdletsToExport   = @()
    VariablesToExport = @()
    AliasesToExport   = @()
}
@@file AssetTracker.psm1
# The module file: loads the code. The manifest (AssetTracker.psd1) decides what is exported.
. (Join-Path $PSScriptRoot 'Assets.ps1')
@@file Assets.ps1
# Assets.ps1: the logic of the AssetTracker module. AssetTracker.psm1 loads this file.
# The store is a JSON file; every change is saved straight away.

$script:Assets = [System.Collections.Generic.List[object]]::new()
$script:StorePath = $null
$script:Prefixes = @{ Laptop = 'LT'; Desktop = 'DT'; Monitor = 'MN'; Phone = 'PH' }

function Open-AssetStore {
    # Loads the store from Path (an empty store if the file doesn't exist yet).
    [CmdletBinding()]
    param([Parameter(Mandatory)][string]$Path)
    $script:StorePath = $Path
    $script:Assets = [System.Collections.Generic.List[object]]::new()
    if (Test-Path -LiteralPath $Path) {
        $data = Get-Content -LiteralPath $Path -Raw -Encoding UTF8 | ConvertFrom-Json
        foreach ($a in @($data)) {
            if ($null -ne $a) { $script:Assets.Add($a) }
        }
    }
    Write-Verbose "Loaded $($script:Assets.Count) asset(s) from $Path"
}

function Save-AssetStore {
    # Writes to a temporary file first, then replaces the store, so a crash never leaves half a file.
    if (-not $script:StorePath) { throw 'No store is open. Run Open-AssetStore first.' }
    $json = ConvertTo-Json -InputObject @($script:Assets) -Depth 5
    $tmp = "$script:StorePath.tmp"
    [System.IO.File]::WriteAllText($tmp, $json, [System.Text.UTF8Encoding]::new($false))
    Move-Item -LiteralPath $tmp -Destination $script:StorePath -Force
}

function Add-History($Asset, [string]$Action) {
    $entry = '{0} {1}' -f (Get-Date -Format 'yyyy-MM-dd'), $Action
    $Asset.History = @(@($Asset.History | Where-Object { $_ }) + $entry)
}

function Find-AssetByTag([string]$Tag) {
    $asset = $script:Assets | Where-Object Tag -eq $Tag | Select-Object -First 1
    if (-not $asset) { throw "No asset with tag $Tag" }
    $asset
}

function New-AssetTag {
    # The next free tag for a type: its prefix, a dash, and the number after the highest one in use,
    # with 4 digits. Laptop -> LT-0001, LT-0002, ...  Desktop DT, Monitor MN, Phone PH.
    # Tags of other types don't count, and gaps aren't reused (LT-0001, LT-0007 -> LT-0008).
    param(
        [Parameter(Mandatory)][ValidateSet('Laptop', 'Desktop', 'Monitor', 'Phone')][string]$Type,
        [string[]]$Existing = @()
    )
    $prefix = $script:Prefixes[$Type]
    $numbers = foreach ($t in $Existing) {
        if ($t -match "^$prefix-(\d+)$") { [int]$Matches[1] }
    }
    $max = 0
    foreach ($n in $numbers) { if ($n -gt $max) { $max = $n } }
    '{0}-{1:D4}' -f $prefix, ($max + 1)
}

function Add-Asset {
    # Registers a new asset as InStock and returns it.
    [CmdletBinding(SupportsShouldProcess)]
    param(
        [Parameter(Mandatory)][ValidateSet('Laptop', 'Desktop', 'Monitor', 'Phone')][string]$Type,
        [Parameter(Mandatory)][string]$Model,
        [Parameter(Mandatory)][ValidatePattern('^[A-Z0-9-]{4,}$')][string]$Serial,
        [datetime]$PurchaseDate = (Get-Date).Date,
        [ValidateRange(0, 120)][int]$WarrantyMonths = 36
    )
    if ($script:Assets | Where-Object Serial -eq $Serial) { throw "Serial $Serial is already registered" }
    $tag = New-AssetTag -Type $Type -Existing @(foreach ($a in $script:Assets) { $a.Tag })
    if ($PSCmdlet.ShouldProcess($tag, "Add $Type $Model ($Serial)")) {
        $asset = [pscustomobject]@{
            Tag            = $tag
            Type           = $Type
            Model          = $Model
            Serial         = $Serial
            Status         = 'InStock'
            AssignedTo     = $null
            PurchaseDate   = $PurchaseDate.ToString('yyyy-MM-dd')
            WarrantyMonths = $WarrantyMonths
            History        = @()
        }
        Add-History $asset 'added'
        $script:Assets.Add($asset)
        Save-AssetStore
        $asset
    }
}

function Get-Asset {
    # Returns the assets that match every filter given. -Tag accepts wildcards (LT-*).
    # -AssignedTo matches the user exactly, ignoring case.
    [CmdletBinding()]
    param(
        [string]$Tag = '*',
        [ValidateSet('Laptop', 'Desktop', 'Monitor', 'Phone')][string]$Type,
        [ValidateSet('InStock', 'Assigned', 'Repair', 'Retired')][string]$Status,
        [string]$AssignedTo
    )
    $script:Assets | Where-Object {
        $_.Tag -like $Tag -and
        (-not $Type -or $_.Type -eq $Type) -and
        (-not $Status -or $_.Status -eq $Status) -and
        (-not $AssignedTo -or $_.AssignedTo -eq $AssignedTo)
    }
}

function Set-AssetOwner {
    # Assigns assets to a user (Status becomes Assigned) and records "assigned to <user>" in the history.
    # Takes tags from the pipeline, so: Get-Asset -Type Laptop -Status InStock | Select -First 1 | Set-AssetOwner -User yuki
    # Retired assets and assets in Repair can't be assigned: throw "<tag> is <status>".
    # Assigning to the user who already has it changes nothing (no history entry).
    [CmdletBinding(SupportsShouldProcess)]
    param(
        [Parameter(Mandatory, ValueFromPipelineByPropertyName)][string]$Tag,
        [Parameter(Mandatory)][string]$User
    )
    process {
        $asset = Find-AssetByTag $Tag
        if ($asset.Status -in 'Retired', 'Repair') { throw "$Tag is $($asset.Status)" }
        if ($asset.AssignedTo -eq $User) { return }
        if ($PSCmdlet.ShouldProcess($Tag, "Assign to $User")) {
            $asset.AssignedTo = $User
            $asset.Status = 'Assigned'
            Add-History $asset "assigned to $User"
            Save-AssetStore
        }
    }
}

function Set-AssetStatus {
    # Moves assets to InStock (returned), Repair or Retired. Anything but Repair clears the owner.
    [CmdletBinding(SupportsShouldProcess)]
    param(
        [Parameter(Mandatory, ValueFromPipelineByPropertyName)][string]$Tag,
        [Parameter(Mandatory)][ValidateSet('InStock', 'Repair', 'Retired')][string]$Status
    )
    process {
        $asset = Find-AssetByTag $Tag
        if ($asset.Status -eq 'Retired') { throw "$Tag is Retired" }
        if ($PSCmdlet.ShouldProcess($Tag, "Set status to $Status")) {
            $asset.Status = $Status
            if ($Status -ne 'Repair') { $asset.AssignedTo = $null }
            Add-History $asset ($Status.ToLower())
            Save-AssetStore
        }
    }
}

function Get-WarrantyReport {
    # Assets (not Retired) whose warranty ends within -Days of -AsOf, or has already ended.
    # The warranty ends PurchaseDate + WarrantyMonths. Output objects with Tag, Model, AssignedTo,
    # WarrantyEnds (yyyy-MM-dd) and DaysLeft (negative = expired), sorted by DaysLeft, then Tag.
    [CmdletBinding()]
    param(
        [datetime]$AsOf = (Get-Date),
        [int]$Days = 90
    )
    $today = $AsOf.Date
    $rows = foreach ($a in $script:Assets) {
        if ($a.Status -eq 'Retired') { continue }
        $ends = ([datetime]$a.PurchaseDate).AddMonths([int]$a.WarrantyMonths)
        $left = [int][math]::Floor(($ends - $today).TotalDays)
        if ($left -le $Days) {
            [pscustomobject]@{
                Tag          = $a.Tag
                Model        = $a.Model
                AssignedTo   = $a.AssignedTo
                WarrantyEnds = $ends.ToString('yyyy-MM-dd')
                DaysLeft     = $left
            }
        }
    }
    $rows | Sort-Object DaysLeft, Tag
}

function Export-AssetReport {
    # Writes an HTML page with every asset and the warranty report. ConvertTo-Html encodes the text.
    [CmdletBinding()]
    param([Parameter(Mandatory)][string]$Path)
    $style = '<style>body{font-family:Segoe UI,sans-serif;margin:2em}table{border-collapse:collapse}' +
        'td,th{border:1px solid #ccc;padding:4px 8px}th{background:#eef}</style>'
    $all = Get-Asset | Select-Object Tag, Type, Model, Serial, Status, AssignedTo |
        ConvertTo-Html -Fragment -PreContent '<h2>All assets</h2>'
    $warranty = Get-WarrantyReport | ConvertTo-Html -Fragment -PreContent '<h2>Warranty ending within 90 days</h2>'
    ConvertTo-Html -Title 'Asset report' -Head $style -Body ($all + $warranty) |
        Set-Content -LiteralPath $Path -Encoding UTF8
    Get-Item -LiteralPath $Path
}
@@file tests.ps1
# Run: powershell -ExecutionPolicy Bypass -File tests.ps1     (or: pwsh -File tests.ps1)
Import-Module (Join-Path $PSScriptRoot 'AssetTracker.psd1') -Force
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

function Test-Throws([string]$Name, [string]$Like, [scriptblock]$Body) {
    try { & $Body; Write-Host "  FAIL  $Name : no error" -ForegroundColor Red; $script:failed++ }
    catch {
        if ($_.Exception.Message -like $Like) { Write-Host "  ok    $Name" -ForegroundColor Green }
        else { Write-Host "  FAIL  $Name : $($_.Exception.Message)" -ForegroundColor Red; $script:failed++ }
    }
}

$store = Join-Path ([System.IO.Path]::GetTempPath()) ("assets-test-{0}.json" -f [guid]::NewGuid())

Write-Host 'Tags'
Test-That 'first tag' { (New-AssetTag -Type Laptop) -ceq 'LT-0001' }
Test-That 'after the highest' { (New-AssetTag -Type Laptop -Existing 'LT-0001', 'LT-0007', 'LT-0003') -ceq 'LT-0008' }
Test-That 'other types ignored' { (New-AssetTag -Type Monitor -Existing 'LT-0004', 'MN-0002', 'PH-0009') -ceq 'MN-0003' }
Test-That 'four digits' { (New-AssetTag -Type Phone -Existing 'PH-0099') -ceq 'PH-0100' }

Write-Host 'Adding and finding'
Open-AssetStore -Path $store
$lt1 = Add-Asset -Type Laptop -Model 'ThinkPad T14' -Serial 'PF3A9K01' -PurchaseDate '2023-12-01' -WarrantyMonths 36
$lt2 = Add-Asset -Type Laptop -Model 'ThinkPad T14' -Serial 'PF3A9K02' -PurchaseDate '2024-06-15' -WarrantyMonths 36
$mn1 = Add-Asset -Type Monitor -Model 'Dell U2724D' -Serial 'CN0X7Y21' -PurchaseDate '2021-03-10' -WarrantyMonths 60
$ph1 = Add-Asset -Type Phone -Model 'iPhone 15' -Serial 'F2LZQ9XK' -PurchaseDate '2024-11-20' -WarrantyMonths 12
Test-That 'tags are given out' { $lt1.Tag -eq 'LT-0001' -and $lt2.Tag -eq 'LT-0002' -and $mn1.Tag -eq 'MN-0001' }
Test-That 'new assets are in stock' { $lt1.Status -eq 'InStock' -and $null -eq $lt1.AssignedTo }
Test-Throws 'duplicate serial' '*already registered*' { Add-Asset -Type Laptop -Model 'X' -Serial 'PF3A9K01' }
Test-That '-WhatIf adds nothing' {
    Add-Asset -Type Desktop -Model 'OptiPlex' -Serial 'WHATIF01' -WhatIf
    @(Get-Asset).Count -eq 4
}
Test-That 'all' { @(Get-Asset).Count -eq 4 }
Test-That 'by type' { @(Get-Asset -Type Laptop).Count -eq 2 }
Test-That 'by tag wildcard' { (@(Get-Asset -Tag 'MN-*') | ForEach-Object Tag) -join ',' -eq 'MN-0001' }
Test-That 'by exact tag' { (Get-Asset -Tag 'LT-0002').Serial -eq 'PF3A9K02' }
Test-That 'filters combine' { @(Get-Asset -Type Laptop -Status Assigned).Count -eq 0 }

Write-Host 'Assigning'
Get-Asset -Type Laptop -Status InStock | Select-Object -First 1 | Set-AssetOwner -User 'yuki'
Test-That 'from the pipeline' { (Get-Asset -Tag 'LT-0001').AssignedTo -eq 'yuki' -and (Get-Asset -Tag 'LT-0001').Status -eq 'Assigned' }
Test-That 'history' { @((Get-Asset -Tag 'LT-0001').History)[-1] -like '*assigned to yuki' }
Set-AssetOwner -Tag 'LT-0001' -User 'yuki'
Test-That 'same user: no new history' { @((Get-Asset -Tag 'LT-0001').History).Count -eq 2 }
Test-That 'by user, any case' { (Get-Asset -AssignedTo 'YUKI').Tag -eq 'LT-0001' }
Set-AssetOwner -Tag 'MN-0001' -User 'ken' -WhatIf
Test-That '-WhatIf changes nothing' { $null -eq (Get-Asset -Tag 'MN-0001').AssignedTo }
Set-AssetStatus -Tag 'LT-0002' -Status Repair
Test-Throws 'not while in repair' '*LT-0002 is Repair*' { Set-AssetOwner -Tag 'LT-0002' -User 'ken' }
Set-AssetStatus -Tag 'PH-0001' -Status Retired
Test-Throws 'not when retired' '*PH-0001 is Retired*' { Set-AssetOwner -Tag 'PH-0001' -User 'ken' }
Test-Throws 'unknown tag' '*No asset with tag XX-0001*' { Set-AssetOwner -Tag 'XX-0001' -User 'ken' }

Write-Host 'Warranty report'
$report = @(Get-WarrantyReport -AsOf '2026-11-01' -Days 60)
Test-That 'expired and soon, not retired' { ($report | ForEach-Object Tag) -join ',' -eq 'MN-0001,LT-0001' }
Test-That 'end date' { $report[1].WarrantyEnds -eq '2026-12-01' -and $report[1].DaysLeft -eq 30 }
Test-That 'expired is negative' { $report[0].WarrantyEnds -eq '2026-03-10' -and $report[0].DaysLeft -eq -236 }
Test-That 'owner shown' { $report[1].AssignedTo -eq 'yuki' }
Test-That 'nothing within 0 days but the expired one' { @(Get-WarrantyReport -AsOf '2026-11-01' -Days 0).Count -eq 1 }

Write-Host 'Saving'
Open-AssetStore -Path $store
Test-That 'reloaded from disk' { @(Get-Asset).Count -eq 4 -and (Get-Asset -Tag 'LT-0001').AssignedTo -eq 'yuki' }
Test-That 'history survives' { @((Get-Asset -Tag 'LT-0001').History).Count -eq 2 }
$html = Join-Path ([System.IO.Path]::GetTempPath()) ("assets-test-{0}.html" -f [guid]::NewGuid())
Test-That 'HTML report' { (Export-AssetReport -Path $html) -and ((Get-Content $html -Raw) -match 'ThinkPad T14') }
Remove-Item $store, $html -ErrorAction SilentlyContinue

if ($script:failed -gt 0) { Write-Host "$script:failed test(s) failed" -ForegroundColor Red; exit 1 }
Write-Host 'All tests passed' -ForegroundColor Green
@@file Show-AssetTracker.ps1
# A Windows desktop app on top of the module: a searchable grid, and buttons for the common jobs.
# Run: powershell -ExecutionPolicy Bypass -File Show-AssetTracker.ps1 [-Path assets.json]
param([string]$Path = (Join-Path $PSScriptRoot 'assets.json'))

Import-Module (Join-Path $PSScriptRoot 'AssetTracker.psd1') -Force
Add-Type -AssemblyName System.Windows.Forms, System.Drawing, Microsoft.VisualBasic
[System.Windows.Forms.Application]::EnableVisualStyles()
Open-AssetStore -Path $Path

$form = New-Object System.Windows.Forms.Form
$form.Text = 'Asset Tracker'
$form.Size = New-Object System.Drawing.Size(900, 520)
$form.StartPosition = 'CenterScreen'
$form.Font = New-Object System.Drawing.Font('Segoe UI', 10)

$search = New-Object System.Windows.Forms.TextBox
$search.Location = New-Object System.Drawing.Point(12, 12)
$search.Width = 260

$grid = New-Object System.Windows.Forms.DataGridView
$grid.Location = New-Object System.Drawing.Point(12, 48)
$grid.Size = New-Object System.Drawing.Size(860, 380)
$grid.Anchor = 'Top, Bottom, Left, Right'
$grid.ReadOnly = $true
$grid.AllowUserToAddRows = $false
$grid.SelectionMode = 'FullRowSelect'
$grid.AutoSizeColumnsMode = 'Fill'

function Update-Grid {
    $text = $search.Text.Trim()
    $rows = Get-Asset | Where-Object {
        -not $text -or "$($_.Tag) $($_.Model) $($_.Serial) $($_.AssignedTo) $($_.Status)" -like "*$text*"
    } | Select-Object Tag, Type, Model, Serial, Status, AssignedTo, PurchaseDate
    $table = New-Object System.Data.DataTable
    foreach ($col in 'Tag', 'Type', 'Model', 'Serial', 'Status', 'AssignedTo', 'PurchaseDate') { [void]$table.Columns.Add($col) }
    foreach ($r in $rows) { [void]$table.Rows.Add($r.Tag, $r.Type, $r.Model, $r.Serial, $r.Status, $r.AssignedTo, $r.PurchaseDate) }
    $grid.DataSource = $table
}

function Get-SelectedTag {
    if ($grid.SelectedRows.Count -eq 0) { return $null }
    $grid.SelectedRows[0].Cells['Tag'].Value
}

function Invoke-Safely([scriptblock]$Action) {
    # A GUI must never crash on a bad click: show the error instead.
    try { & $Action; Update-Grid }
    catch { [void][System.Windows.Forms.MessageBox]::Show($_.Exception.Message, 'Asset Tracker', 'OK', 'Warning') }
}

function New-Button([string]$Text, [int]$X, [scriptblock]$OnClick) {
    $b = New-Object System.Windows.Forms.Button
    $b.Text = $Text
    $b.Location = New-Object System.Drawing.Point($X, 440)
    $b.Size = New-Object System.Drawing.Size(130, 32)
    $b.Anchor = 'Bottom, Left'
    $b.Add_Click($OnClick)
    $form.Controls.Add($b)
}

New-Button 'Add...' 12 {
    Invoke-Safely {
        $type = [Microsoft.VisualBasic.Interaction]::InputBox('Type (Laptop, Desktop, Monitor, Phone):', 'Add asset', 'Laptop')
        if (-not $type) { return }
        $model = [Microsoft.VisualBasic.Interaction]::InputBox('Model:', 'Add asset')
        $serial = [Microsoft.VisualBasic.Interaction]::InputBox('Serial number:', 'Add asset')
        if ($model -and $serial) { [void](Add-Asset -Type $type -Model $model -Serial $serial.ToUpper()) }
    }
}
New-Button 'Assign...' 152 {
    Invoke-Safely {
        $tag = Get-SelectedTag
        if (-not $tag) { return }
        $user = [Microsoft.VisualBasic.Interaction]::InputBox("Assign $tag to user:", 'Assign')
        if ($user) { Set-AssetOwner -Tag $tag -User $user }
    }
}
New-Button 'Return to stock' 292 { Invoke-Safely { $t = Get-SelectedTag; if ($t) { Set-AssetStatus -Tag $t -Status InStock } } }
New-Button 'Send to repair' 432 { Invoke-Safely { $t = Get-SelectedTag; if ($t) { Set-AssetStatus -Tag $t -Status Repair } } }
New-Button 'HTML report' 572 {
    Invoke-Safely {
        $file = Export-AssetReport -Path (Join-Path ([System.IO.Path]::GetTempPath()) 'asset-report.html')
        Start-Process $file.FullName
    }
}

$search.Add_TextChanged({ Update-Grid })
$form.Controls.AddRange(@($search, $grid))
$form.Add_Shown({ Update-Grid; $search.Focus() })
[void]$form.ShowDialog()

@@drill
Which cmdlet loads a module from a path or by name?
= Import-Module
? Import-Module ./AssetTracker.

@@text
🎉 You've written a PowerShell module the way Microsoft's own modules are built: a manifest, safe commands with `-WhatIf`, pipeline support, tests, and a GUI for colleagues who'll never open a console.
