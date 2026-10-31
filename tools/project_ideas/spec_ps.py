from gen_projects import P

PROJECTS = [
    P("powershell", "Intermediate", "software_inventory", "Software Inventory Report", "List installed programs on a PC and export a tidy report.",
      """Every IT department needs to know what's installed where. Read the installed programs from the registry (both 64-bit and 32-bit uninstall keys), clean the list up, and export it as CSV and as a sortable HTML page.""",
      ["registry provider", "Select-Object and calculated properties", "Sort-Object -Unique", "Export-Csv", "ConvertTo-Html"],
      ["Read: get DisplayName, DisplayVersion, Publisher and InstallDate from both uninstall registry keys.",
       "Clean: drop entries without a name, remove duplicates, turn InstallDate (20260131) into a real date.",
       "Export: write inventory.csv and inventory.html with the computer name and date in the title.",
       "Compare: given yesterday's CSV, list what was installed or removed since."],
      ["Running the script produces both files and the comparison works on two saved CSVs."],
      ["Why do I need both the WOW6432Node key and the normal one?",
       "How do I compare two CSVs of objects in PowerShell?"],
      {"Get-SoftwareInventory.ps1": '''<#
.SYNOPSIS
    Lists installed software and exports CSV and HTML reports.
#>
param(
    [string]$OutDir = ".",
    [string]$Previous
)

$keys = @(
    "HKLM:\\Software\\Microsoft\\Windows\\CurrentVersion\\Uninstall\\*",
    "HKLM:\\Software\\WOW6432Node\\Microsoft\\Windows\\CurrentVersion\\Uninstall\\*"
)

# TODO 1: read DisplayName, DisplayVersion, Publisher, InstallDate from $keys
$software = @()

# TODO 2: clean up the list
# TODO 3: Export-Csv and ConvertTo-Html into $OutDir
# TODO 4: if $Previous is given, compare with it

Write-Output "Found $($software.Count) programs"
'''}),
    P("powershell", "Advanced", "cleanup_task", "Scheduled Disk Cleanup Task", "A safe cleanup script that registers itself as a scheduled task.",
      """Temp folders, old logs and the Downloads folder fill disks. Write a cleanup script that deletes files older than N days from a list of folders, supports `-WhatIf` so you can see what it *would* do, logs everything, and can register itself to run every Sunday night.""",
      ["SupportsShouldProcess / -WhatIf", "Get-ChildItem filters", "logging", "ScheduledTasks module"],
      ["Configuration: read folders and ages from cleanup.json.",
       "Find: list files older than the age in each folder, with their total size.",
       "Delete safely: use `[CmdletBinding(SupportsShouldProcess)]` so `-WhatIf` shows the plan.",
       "Log: append a line per run with the space freed to cleanup.log.",
       "Schedule: a `-Register` switch that creates a weekly scheduled task (run as admin)."],
      ["`-WhatIf` deletes nothing and lists everything; a real run frees space and logs it."],
      ["How does SupportsShouldProcess work with Remove-Item?",
       "What's the safest way to test a script that deletes files?"],
      {"Invoke-Cleanup.ps1": '''[CmdletBinding(SupportsShouldProcess)]
param(
    [string]$Config = "$PSScriptRoot\\cleanup.json",
    [switch]$Register
)

$settings = Get-Content $Config -Raw | ConvertFrom-Json

foreach ($rule in $settings.rules) {
    $cutoff = (Get-Date).AddDays(-$rule.days)
    # TODO: find files older than $cutoff in $rule.path, delete them with ShouldProcess, add up the sizes
}

# TODO: log the result; if $Register, create the scheduled task
''', "cleanup.json": '''{
  "rules": [
    { "path": "C:\\\\Temp", "days": 7 },
    { "path": "C:\\\\Logs\\\\App", "days": 30 }
  ]
}'''}),
]
