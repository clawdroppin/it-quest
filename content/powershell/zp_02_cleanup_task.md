@@lesson xp-cleanup_task Scheduled Disk Cleanup Task
@@text
Temp folders, old logs and the Downloads folder fill disks. Write a cleanup script that deletes files older than N days from a list of folders, supports `-WhatIf` so you can see what it *would* do, logs everything, and can register itself to run every Sunday night.

**You'll practise:** SupportsShouldProcess / -WhatIf · Get-ChildItem filters · logging · ScheduledTasks module

## Milestones

1. **Configuration**: read folders and ages from cleanup.json.
2. **Find**: list files older than the age in each folder, with their total size.
3. **Delete safely**: use `[CmdletBinding(SupportsShouldProcess)]` so `-WhatIf` shows the plan.
4. **Log**: append a line per run with the space freed to cleanup.log.
5. **Schedule**: a `-Register` switch that creates a weekly scheduled task (run as admin).

## You're done when

- `-WhatIf` deletes nothing and lists everything; a real run frees space and logs it.

## Getting help

Open the project, then the 🤖 **AI Helper**: it sees the lesson and your files. Good questions to start with:

- *"How does SupportsShouldProcess work with Remove-Item?"*
- *"What's the safest way to test a script that deletes files?"*

Ask for hints, reviews and explanations rather than finished code: you learn by writing it.
@@project cleanup_task Scheduled Disk Cleanup Task
Create the folder, open it in VS Code, and work through `README.md` one milestone at a time.
@@file README.md
# Scheduled Disk Cleanup Task

Temp folders, old logs and the Downloads folder fill disks. Write a cleanup script that deletes files older than N days from a list of folders, supports `-WhatIf` so you can see what it *would* do, logs everything, and can register itself to run every Sunday night.

## Milestones

- [ ] 1. Configuration: read folders and ages from cleanup.json.
- [ ] 2. Find: list files older than the age in each folder, with their total size.
- [ ] 3. Delete safely: use `[CmdletBinding(SupportsShouldProcess)]` so `-WhatIf` shows the plan.
- [ ] 4. Log: append a line per run with the space freed to cleanup.log.
- [ ] 5. Schedule: a `-Register` switch that creates a weekly scheduled task (run as admin).

## Done when

- [ ] `-WhatIf` deletes nothing and lists everything; a real run frees space and logs it.
@@file Invoke-Cleanup.ps1
[CmdletBinding(SupportsShouldProcess)]
param(
    [string]$Config = "$PSScriptRoot\cleanup.json",
    [switch]$Register
)

$settings = Get-Content $Config -Raw | ConvertFrom-Json

foreach ($rule in $settings.rules) {
    $cutoff = (Get-Date).AddDays(-$rule.days)
    # TODO: find files older than $cutoff in $rule.path, delete them with ShouldProcess, add up the sizes
}

# TODO: log the result; if $Register, create the scheduled task
@@file cleanup.json
{
  "rules": [
    { "path": "C:\\Temp", "days": 7 },
    { "path": "C:\\Logs\\App", "days": 30 }
  ]
}
