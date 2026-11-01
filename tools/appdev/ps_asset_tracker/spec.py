OUT = "content/powershell/q3_01_asset_tracker.md"
BASE = "tools/appdev/ps_asset_tracker"
FILES = ["AssetTracker.psd1", "AssetTracker.psm1", "Assets.ps1", "tests.ps1", "Show-AssetTracker.ps1"]

PARTS = [
    ("text", r'''
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
'''),
    ("project", "asset_tracker", "IT Asset Tracker",
     "Write the four functions that throw `TODO` in `Assets.ps1` until `tests.ps1` passes, then run `Show-AssetTracker.ps1`.",
     BASE + "/starter", FILES),
    ("text", r'''
@@text
## Stretch goals

- **CSV import**: `Import-AssetCsv` that reads the purchasing department's export and calls `Add-Asset` for each row, with `-WhatIf` support for free (it inherits `$WhatIfPreference`).
- **Inventory from the network**: fill in Model and Serial automatically with `Get-CimInstance Win32_ComputerSystem` and `Win32_BIOS` (run against remote PCs with `-ComputerName` or `Invoke-Command`).
- **Pester**: rewrite `tests.ps1` as `AssetTracker.Tests.ps1` with `Describe`/`It`/`Should`, and use `Mock Get-Date` to test the history dates.
- **Shared store**: put `assets.json` on a file share, and handle two people saving at once (a lock file, or compare a version number before saving and refuse if someone else saved first).
- **SQL instead of JSON**: the same commands on top of SQLite or SQL Server. The tests shouldn't need to change, which is the point of hiding the storage behind functions.
- **Publish it**: put the module in a folder on `$env:PSModulePath`, or publish it to an internal repository with `Publish-Module`.
'''),
    ("project", "asset_tracker_finished", "IT Asset Tracker: the finished version",
     "A complete solution to compare with yours.",
     BASE + "/finished", FILES),
    ("text", r'''
@@text
🎉 You've written a PowerShell module the way Microsoft's own modules are built: a manifest, safe commands with `-WhatIf`, pipeline support, tests, and a GUI for colleagues who'll never open a console.
'''),
]
