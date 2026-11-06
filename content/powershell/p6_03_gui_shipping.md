@@lesson p6-gui-shipping Desktop GUIs, Testing in CI and Shipping Your Tools
@@concepts testing, automation
@@text
Not everyone you write tools for will open a console. The service desk, HR or a site office can use a small window with a search box and three buttons. PowerShell can build those with the same .NET GUI libraries C# uses. This lesson also covers getting your tools tested automatically and into other people's hands.

## WinForms: quick and simple

```powershell,norun
Add-Type -AssemblyName System.Windows.Forms, System.Drawing

$form = New-Object System.Windows.Forms.Form
$form.Text = 'Password Reset'
$form.Size = New-Object System.Drawing.Size(360, 170)
$form.StartPosition = 'CenterScreen'

$box = New-Object System.Windows.Forms.TextBox
$box.Location = New-Object System.Drawing.Point(16, 16)
$box.Width = 310

$button = New-Object System.Windows.Forms.Button
$button.Text = 'Reset'
$button.Location = New-Object System.Drawing.Point(16, 56)
$button.Add_Click({
    try {
        $result = Reset-UserPassword -User $box.Text      # the tested logic lives in a module
        [System.Windows.Forms.MessageBox]::Show("Temporary password: $($result.Password)")
    }
    catch {
        [System.Windows.Forms.MessageBox]::Show($_.Exception.Message, 'Error', 'OK', 'Error')
    }
})

$form.Controls.AddRange(@($box, $button))
[void]$form.ShowDialog()                                   # runs the event loop until the window closes
```

The handlers (`Add_Click`, `Add_TextChanged`) are **events**: the form waits, and calls your script block when something happens.

## WPF: layouts in XAML

For anything bigger than a few controls, describe the window in **XAML** (the same markup Visual Studio uses) and wire it up in PowerShell:

```powershell,norun
Add-Type -AssemblyName PresentationFramework
[xml]$xaml = @'
<Window xmlns="http://schemas.microsoft.com/winfx/2006/xaml/presentation"
        xmlns:x="http://schemas.microsoft.com/winfx/2006/xaml"
        Title="Service Checker" Width="420" Height="300">
  <DockPanel Margin="10">
    <StackPanel DockPanel.Dock="Top" Orientation="Horizontal">
      <TextBox x:Name="Computer" Width="250" Margin="0,0,8,0"/>
      <Button x:Name="Check" Content="Check services" Padding="10,2"/>
    </StackPanel>
    <DataGrid x:Name="Results" Margin="0,10,0,0" IsReadOnly="True"/>
  </DockPanel>
</Window>
'@
$window = [Windows.Markup.XamlReader]::Load([System.Xml.XmlNodeReader]::new($xaml))
$computer = $window.FindName('Computer')
$results = $window.FindName('Results')
$window.FindName('Check').Add_Click({
    $results.ItemsSource = @(Get-Service -ComputerName $computer.Text | Where-Object Status -eq Stopped |
        Select-Object Name, DisplayName, StartType)
})
[void]$window.ShowDialog()
```

## Rules for GUI tools

- **Keep the logic out of the form.** Event handlers should call functions from a module, which you test without a GUI. The form only reads inputs, calls a function, and shows the result.
- **Never let an exception escape a handler**: catch it and show a message.
- **Don't freeze the window.** A handler that takes 30 seconds makes Windows say "Not Responding". Run long work in a background runspace or job, and update the UI when it finishes (WinForms: a `Timer` that checks the job; WPF: `$window.Dispatcher.Invoke`).
- **Validate input before acting**, and show all the problems at once, not one per click.
- GUIs need Windows PowerShell or PowerShell 7 **on Windows**: WinForms and WPF don't exist on Linux or macOS.

## Test in CI

Every push should run your Pester tests and the script analyzer automatically. A GitHub Actions workflow:

```text
# .github/workflows/test.yml
name: test
on: [push, pull_request]
jobs:
  test:
    runs-on: windows-latest
    steps:
      - uses: actions/checkout@v4
      - name: Lint
        shell: pwsh
        run: |
          Install-Module PSScriptAnalyzer -Force -Scope CurrentUser
          $issues = Invoke-ScriptAnalyzer -Path . -Recurse -Severity Error, Warning
          $issues
          if ($issues) { exit 1 }
      - name: Test
        shell: pwsh
        run: Invoke-Pester -Path ./tests -CI       # -CI: fail the job on any failed test
```

Azure DevOps Pipelines and GitLab CI work the same way.

## Shipping

| Audience | How |
|---|---|
| your team | a Git repo, or the module on a file share listed in `$env:PSModulePath` |
| the whole company | an internal NuGet feed (Azure Artifacts, Nexus, ProGet) + `Register-PSRepository`, then `Install-Module` and `Update-Module` |
| everyone | the PowerShell Gallery: `Publish-Module -Path .\NetTools -NuGetApiKey ...` |
| computers, not people | Intune, Group Policy, or Configuration Manager deploys the module or script |

**Sign your scripts** with a code-signing certificate (`Set-AuthenticodeSignature`) when your company uses the `AllSigned` execution policy, and use **semantic versioning** in the manifest (`1.4.2`: breaking.feature.fix), with release notes in `PrivateData.PSData.ReleaseNotes`.

@@quiz
A WinForms button runs a 40-second scan and the window shows "Not Responding". The fix?
- Make the window smaller
* Run the scan in the background (a job or runspace) and update the UI when it's done
- Use WPF instead of WinForms
- Call `Start-Sleep` in the handler
? The handler runs on the UI thread. While it's busy, the window can't redraw or react.

@@quiz
Why should a GUI's button handler call a module function instead of containing the logic?
* The logic can then be tested without a GUI, and reused from scripts
- Handlers can't contain more than 10 lines
- Modules run faster than script blocks
- WinForms requires it
? A thin GUI over tested functions is the same pattern as a thin `main` over a tested library.

@@quiz
What does `Invoke-Pester -CI` add in a pipeline?
- It installs Pester
* It makes the run fail (non-zero exit) when tests fail, and writes a test result file
- It runs tests in parallel
- It skips slow tests
? CI needs a failing exit code to mark the build red.

@@quiz
How do you share a module with the whole company so people can `Install-Module` and `Update-Module` it?
- Email the .psm1
* Publish it to an internal repository (a NuGet feed) registered with `Register-PSRepository`
- Put it in C:\Windows\System32
- Paste it into a Group Policy
? An internal feed gives versioning and updates, like the PowerShell Gallery but private.

@@drill
Which .NET assembly do you load for WinForms?
= System.Windows.Forms
? Add-Type -AssemblyName System.Windows.Forms.

@@flashcards
WinForms :: classic .NET GUI library: Form, TextBox, Button, DataGridView
WPF :: .NET GUI library with XAML layouts
`[Windows.Markup.XamlReader]::Load()` :: builds a WPF window from XAML
`$window.FindName('x')` :: gets a named control from XAML
`Add_Click({ ... })` :: attach an event handler
`ShowDialog()` :: shows the window and runs its event loop
UI thread :: must stay free, or the window "stops responding"
`Invoke-Pester -CI` :: test run that fails the build on failures
`Publish-Module` :: uploads a module to a repository
`Register-PSRepository` :: adds an internal module feed
`Set-AuthenticodeSignature` :: signs a script with a certificate

@@exercise Logic outside the form
The search box in a GUI calls `Select-MatchingRow` every time the text changes. Write it: given `-Rows` (objects) and `-Text`, return the rows where **any property value** contains the text, ignoring case. Empty or blank text returns every row. (This is the testable part of a GUI.)
@@starter
function Select-MatchingRow {
    param([object[]]$Rows, [string]$Text)
    throw "TODO: not done yet"
}

@@tests
$rows = @(
    [pscustomobject]@{ Name = 'tokyo-sw01'; Owner = 'Network'; Status = 'Up' }
    [pscustomobject]@{ Name = 'osaka-fs02'; Owner = 'Storage'; Status = 'Down' }
    [pscustomobject]@{ Name = 'tokyo-db01'; Owner = 'Data'; Status = 'Up' }
)
check @(Select-MatchingRow -Rows $rows -Text 'TOKYO' | ForEach-Object Name) @('tokyo-sw01', 'tokyo-db01') "matches a name, any case"
check @(Select-MatchingRow -Rows $rows -Text 'down' | ForEach-Object Name) @('osaka-fs02') "matches another property"
check @(Select-MatchingRow -Rows $rows -Text '  ').Count 3 "blank text shows everything"
check @(Select-MatchingRow -Rows $rows -Text 'nagoya').Count 0 "no match"
@@hint
`$t = $Text.Trim(); if (-not $t) { return $Rows }`, then `$Rows | Where-Object { @($_.PSObject.Properties.Value) -join ' ' -like "*$t*" }`. `-like` already ignores case.
@@solution
function Select-MatchingRow {
    param([object[]]$Rows, [string]$Text)
    $t = "$Text".Trim()
    if (-not $t) { return $Rows }
    $Rows | Where-Object { (@($_.PSObject.Properties.Value) -join ' ') -like "*$t*" }
}

@@exercise Validate a form
Before a "New starter" form creates an account, `Test-NewStarterForm` checks the fields and returns **all** the problems as strings, in this order (nothing when everything is fine):

- `Name is required` when `Name` is blank
- `Email must end with @contoso.com` unless `Email` matches `something@contoso.com` (any case)
- `Department must be one of: IT, Sales, HR` unless `Department` is one of those
- `Start date can't be in the past` when `StartDate` is before `Today` (both `[datetime]`; same day is fine)
@@starter
function Test-NewStarterForm {
    param([string]$Name, [string]$Email, [string]$Department, [datetime]$StartDate, [datetime]$Today)
    throw "TODO: not done yet"
}

@@tests
$today = [datetime]'2026-11-01'
check @(Test-NewStarterForm -Name 'Aiko Mori' -Email 'amori@Contoso.com' -Department 'Sales' -StartDate $today -Today $today).Count 0 "a valid form"
check @(Test-NewStarterForm -Name ' ' -Email 'aiko@gmail.com' -Department 'Legal' -StartDate '2026-10-30' -Today $today) @(
    'Name is required', 'Email must end with @contoso.com', 'Department must be one of: IT, Sales, HR', "Start date can't be in the past"
) "every problem, in order"
check @(Test-NewStarterForm -Name 'Ken' -Email '@contoso.com' -Department 'IT' -StartDate '2026-12-01' -Today $today) @('Email must end with @contoso.com') "an email needs a name part"
@@hint
Collect into a list: `if (-not $Name.Trim()) { 'Name is required' }`, `if ($Email -notmatch '^[^@\s]+@contoso\.com$') { ... }` (`-notmatch` ignores case), `if ($Department -notin 'IT', 'Sales', 'HR') { ... }`, `if ($StartDate.Date -lt $Today.Date) { ... }`. Output each string; PowerShell collects them.
@@solution
function Test-NewStarterForm {
    param([string]$Name, [string]$Email, [string]$Department, [datetime]$StartDate, [datetime]$Today)
    if (-not $Name.Trim()) { 'Name is required' }
    if ($Email -notmatch '^[^@\s]+@contoso\.com$') { 'Email must end with @contoso.com' }
    if ($Department -notin 'IT', 'Sales', 'HR') { 'Department must be one of: IT, Sales, HR' }
    if ($StartDate.Date -lt $Today.Date) { "Start date can't be in the past" }
}
