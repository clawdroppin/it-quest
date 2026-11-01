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
