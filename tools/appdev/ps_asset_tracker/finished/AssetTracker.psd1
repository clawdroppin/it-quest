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
