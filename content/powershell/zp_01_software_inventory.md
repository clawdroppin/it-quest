@@lesson xp-software_inventory Software Inventory Report
@@text
Every IT department needs to know what's installed where. Read the installed programs from the registry (both 64-bit and 32-bit uninstall keys), clean the list up, and export it as CSV and as a sortable HTML page.

**You'll practise:** registry provider · Select-Object and calculated properties · Sort-Object -Unique · Export-Csv · ConvertTo-Html

## Milestones

1. **Read**: get DisplayName, DisplayVersion, Publisher and InstallDate from both uninstall registry keys.
2. **Clean**: drop entries without a name, remove duplicates, turn InstallDate (20260131) into a real date.
3. **Export**: write inventory.csv and inventory.html with the computer name and date in the title.
4. **Compare**: given yesterday's CSV, list what was installed or removed since.

## You're done when

- Running the script produces both files and the comparison works on two saved CSVs.

## Getting help

Open the project, then the 🤖 **AI Helper**: it sees the lesson and your files. Good questions to start with:

- *"Why do I need both the WOW6432Node key and the normal one?"*
- *"How do I compare two CSVs of objects in PowerShell?"*

Ask for hints, reviews and explanations rather than finished code: you learn by writing it.
@@project software_inventory Software Inventory Report
Create the folder, open it in VS Code, and work through `README.md` one milestone at a time.
@@file README.md
# Software Inventory Report

Every IT department needs to know what's installed where. Read the installed programs from the registry (both 64-bit and 32-bit uninstall keys), clean the list up, and export it as CSV and as a sortable HTML page.

## Milestones

- [ ] 1. Read: get DisplayName, DisplayVersion, Publisher and InstallDate from both uninstall registry keys.
- [ ] 2. Clean: drop entries without a name, remove duplicates, turn InstallDate (20260131) into a real date.
- [ ] 3. Export: write inventory.csv and inventory.html with the computer name and date in the title.
- [ ] 4. Compare: given yesterday's CSV, list what was installed or removed since.

## Done when

- [ ] Running the script produces both files and the comparison works on two saved CSVs.
@@file Get-SoftwareInventory.ps1
<#
.SYNOPSIS
    Lists installed software and exports CSV and HTML reports.
#>
param(
    [string]$OutDir = ".",
    [string]$Previous
)

$keys = @(
    "HKLM:\Software\Microsoft\Windows\CurrentVersion\Uninstall\*",
    "HKLM:\Software\WOW6432Node\Microsoft\Windows\CurrentVersion\Uninstall\*"
)

# TODO 1: read DisplayName, DisplayVersion, Publisher, InstallDate from $keys
$software = @()

# TODO 2: clean up the list
# TODO 3: Export-Csv and ConvertTo-Html into $OutDir
# TODO 4: if $Previous is given, compare with it

Write-Output "Found $($software.Count) programs"
