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
