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
