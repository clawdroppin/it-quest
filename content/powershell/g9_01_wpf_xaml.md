@@lesson g9-wpf WPF from PowerShell: XAML, Layout and Events
@@concepts objects, input
@@text
Module 6 introduced WinForms. **WPF** (Windows Presentation Foundation) is the more modern Windows desktop toolkit: the layout is described in **XAML** (an XML language), scales cleanly on high-DPI screens, and supports data binding, which the next lesson uses to separate logic from the screen.

## Loading XAML

```powershell,norun
Add-Type -AssemblyName PresentationFramework
[xml]$xaml = @'
<Window xmlns="http://schemas.microsoft.com/winfx/2006/xaml/presentation"
        xmlns:x="http://schemas.microsoft.com/winfx/2006/xaml"
        Title="Reset password" Width="360" SizeToContent="Height">
  <StackPanel Margin="12">
    <TextBlock Text="User name"/>
    <TextBox x:Name="UserBox" Margin="0,4,0,8"/>
    <CheckBox x:Name="MustChange" Content="Must change at next logon" IsChecked="True"/>
    <Button x:Name="ResetButton" Content="Reset" Margin="0,12,0,0" IsDefault="True"/>
    <TextBlock x:Name="Status" Foreground="Gray"/>
  </StackPanel>
</Window>
'@
$window = [Windows.Markup.XamlReader]::Load([System.Xml.XmlNodeReader]::new($xaml))
$userBox = $window.FindName('UserBox')
$window.FindName('ResetButton').Add_Click({ $window.FindName('Status').Text = "reset $($userBox.Text)" })
$window.ShowDialog() | Out-Null
```

- `x:Name` gives a control a name; `FindName` gets it from PowerShell.
- Events: `$control.Add_EventName({ ... })`, the same pattern as WinForms.
- Designers like Visual Studio's XAML editor produce attributes PowerShell can't use (`x:Class`, `mc:Ignorable`, `Click="Handler"`): remove them before loading.
- `ShowDialog()` blocks until the window closes. WPF requires an **STA** thread: `powershell.exe` 5.1 is STA by default; for `pwsh` start with `-STA` (the default on Windows since 7.0).

## Layout panels

| Panel | Arranges children |
|---|---|
| `StackPanel` | in a line, vertical (default) or `Orientation="Horizontal"` |
| `Grid` | in rows and columns: `RowDefinitions`, `Grid.Row="1"`, `Grid.ColumnSpan` |
| `DockPanel` | along edges: `DockPanel.Dock="Top"`, last child fills the rest |
| `WrapPanel` | in a line that wraps |
| `UniformGrid` | equal-sized cells |

Sizes: `Auto` (fit content), a number (device-independent pixels, 1/96 inch) or `*` (share of the remaining space: `2*` takes twice as much as `*`). Use `Margin` (outside) and `Padding` (inside) instead of absolute positions, and the window adapts to fonts, languages and DPI.

## Testing a window without showing it

Every control is an object, so you can drive the UI from code without `ShowDialog()`: set `TextBox.Text`, then **raise** the click event:

```powershell,norun
$button.RaiseEvent([Windows.RoutedEventArgs]::new([Windows.Controls.Button]::ClickEvent))
```

That runs the same handlers a real click runs, which is how the exercises here check your work. Some WPF work (data binding, layout) is queued on the window's **dispatcher**, so tests "pump" it once to let queued work finish:

```powershell,norun
function Wait-Dispatcher {
    $frame = [Windows.Threading.DispatcherFrame]::new()
    [void][Windows.Threading.Dispatcher]::CurrentDispatcher.BeginInvoke('Background', [Action]{ $frame.Continue = $false })
    [Windows.Threading.Dispatcher]::PushFrame($frame)
}
```

## Keeping the UI responsive

Long work in a click handler freezes the window ("Not responding"). Run it in a runspace (module 7) and update controls back on the UI thread with `$window.Dispatcher.Invoke({ ... })`: WPF controls can only be touched from the thread that created them.

@@quiz
How does PowerShell get a reference to a control defined in XAML?
* Give it an `x:Name` and call `$window.FindName('Name')`
- Controls are automatically PowerShell variables
- With Get-Control
- By index in the XAML file
? Some scripts loop over all x:Name attributes and create variables for each.

@@quiz
What does `Width="2*"` mean for a Grid column?
- Twice the content width
* Two shares of the remaining space (twice as wide as a `*` column)
- 2 pixels
- 200%
? Star sizing distributes free space proportionally.

@@quiz
Why must long-running work not run directly in a click handler?
- Click handlers can't run cmdlets
* It blocks the UI thread, so the window stops repainting and responding
- It runs twice
- WPF cancels it after 5 seconds
? Use a runspace and Dispatcher.Invoke for updates.

@@drill
Which .NET class loads a XAML string into a WPF window?
= XamlReader
= System.Windows.Markup.XamlReader
= [Windows.Markup.XamlReader]
? [Windows.Markup.XamlReader]::Parse($xaml).

@@flashcards
XAML :: XML description of a WPF UI
`x:Name` :: names a control
`FindName()` :: get a named control
`Add_Click({ })` :: attach an event handler
`ShowDialog()` :: show and wait
`StackPanel` / `Grid` / `DockPanel` :: layout panels
`Auto` / `*` :: fit content / share space
`Margin` vs `Padding` :: outside vs inside spacing
STA :: thread model WPF requires
`RaiseEvent` :: trigger an event from code
`Dispatcher.Invoke` :: run on the UI thread

@@exercise A password reset form, tested without a screen
Load the XAML from the starter (a `StackPanel`, so no window opens), then:

- find all named controls by walking the XAML for `x:Name` attributes, and print their names and types
- attach a click handler to `ResetButton` that validates the user name (`^[a-z][a-z0-9.-]{2,19}$`), and sets `Status` to `reset NAME (must change: True/False)` or `invalid user name`
- simulate: type a valid name and click; uncheck the box, type an invalid name and click; print the status after each
@@starter
Add-Type -AssemblyName PresentationFramework
$xamlText = @'
<StackPanel xmlns="http://schemas.microsoft.com/winfx/2006/xaml/presentation"
            xmlns:x="http://schemas.microsoft.com/winfx/2006/xaml" Margin="12">
  <TextBlock Text="User name"/>
  <TextBox x:Name="UserBox"/>
  <CheckBox x:Name="MustChange" Content="Must change at next logon" IsChecked="True"/>
  <Button x:Name="ResetButton" Content="Reset"/>
  <TextBlock x:Name="Status"/>
</StackPanel>
'@
'TODO'
@@expected
UserBox: TextBox
MustChange: CheckBox
ResetButton: Button
Status: TextBlock
reset asato (must change: True)
invalid user name
@@requires
XamlReader
FindName
Add_Click
RaiseEvent
@@hint
`[xml]$xml = $xamlText; $root = [Windows.Markup.XamlReader]::Load([System.Xml.XmlNodeReader]::new($xml))`. Find the names with XPath and a namespace manager, or simply `$xml.SelectNodes('//*[@*[local-name()="Name"]]')` and read `$node.GetAttribute('Name', 'http://schemas.microsoft.com/winfx/2006/xaml')`. A CheckBox's `IsChecked` is a nullable bool: `[bool]$check.IsChecked`.
@@solution
Add-Type -AssemblyName PresentationFramework
$xamlText = @'
<StackPanel xmlns="http://schemas.microsoft.com/winfx/2006/xaml/presentation"
            xmlns:x="http://schemas.microsoft.com/winfx/2006/xaml" Margin="12">
  <TextBlock Text="User name"/>
  <TextBox x:Name="UserBox"/>
  <CheckBox x:Name="MustChange" Content="Must change at next logon" IsChecked="True"/>
  <Button x:Name="ResetButton" Content="Reset"/>
  <TextBlock x:Name="Status"/>
</StackPanel>
'@

[xml]$xml = $xamlText
$root = [Windows.Markup.XamlReader]::Load([System.Xml.XmlNodeReader]::new($xml))
$xNs = 'http://schemas.microsoft.com/winfx/2006/xaml'
$ui = @{}
foreach ($node in $xml.SelectNodes('//*[@*[local-name()="Name"]]')) {
    $name = $node.GetAttribute('Name', $xNs)
    $ui[$name] = $root.FindName($name)
    "${name}: $($ui[$name].GetType().Name)"
}

$ui.ResetButton.Add_Click({
    $user = $ui.UserBox.Text
    if ($user -cmatch '^[a-z][a-z0-9.-]{2,19}$') {
        $ui.Status.Text = "reset $user (must change: $([bool]$ui.MustChange.IsChecked))"
    } else {
        $ui.Status.Text = 'invalid user name'
    }
})

$click = [Windows.RoutedEventArgs]::new([Windows.Controls.Button]::ClickEvent)
$ui.UserBox.Text = 'asato'
$ui.ResetButton.RaiseEvent($click)
$ui.Status.Text
$ui.MustChange.IsChecked = $false
$ui.UserBox.Text = 'A$!'
$ui.ResetButton.RaiseEvent($click)
$ui.Status.Text

@@exercise Layout with a Grid
Build (in XAML) a server details panel with a `Grid` of 3 rows and 2 columns: labels in column 0 (`Auto` width), values in column 1 (`*`), and a `Button` in row 2 spanning both columns. Then, from PowerShell:

- measure and arrange it at 400×200 (`Measure` / `Arrange`), then print each child's row, column, span and its actual width (rounded)
- check that the value column gets the remaining space and that the button spans the full width
@@starter
Add-Type -AssemblyName PresentationFramework
'TODO'
@@expected
TextBlock row 0 col 0 span 1 width 60
TextBox row 0 col 1 span 1 width 340
TextBlock row 1 col 0 span 1 width 60
TextBox row 1 col 1 span 1 width 340
Button row 2 col 0 span 2 width 400
label column: 60, value column: 340
@@requires
Grid.ColumnSpan
ColumnDefinition
Arrange
@@hint
Give the labels `Width="60"` so the `Auto` column's size doesn't depend on fonts installed on the machine. Read attached properties with `[Windows.Controls.Grid]::GetRow($child)`, `GetColumn`, `GetColumnSpan`. After `$grid.Measure([Windows.Size]::new(400, 200))` and `$grid.Arrange([Windows.Rect]::new(0, 0, 400, 200))`, each element has `ActualWidth`, and each column definition has `ActualWidth`.
@@solution
Add-Type -AssemblyName PresentationFramework
$xamlText = @'
<Grid xmlns="http://schemas.microsoft.com/winfx/2006/xaml/presentation">
  <Grid.RowDefinitions>
    <RowDefinition Height="Auto"/>
    <RowDefinition Height="Auto"/>
    <RowDefinition Height="*"/>
  </Grid.RowDefinitions>
  <Grid.ColumnDefinitions>
    <ColumnDefinition Width="Auto"/>
    <ColumnDefinition Width="*"/>
  </Grid.ColumnDefinitions>
  <TextBlock Grid.Row="0" Grid.Column="0" Width="60" Text="Name"/>
  <TextBox Grid.Row="0" Grid.Column="1" Text="web01"/>
  <TextBlock Grid.Row="1" Grid.Column="0" Width="60" Text="IP"/>
  <TextBox Grid.Row="1" Grid.Column="1" Text="10.0.0.5"/>
  <Button Grid.Row="2" Grid.Column="0" Grid.ColumnSpan="2" Content="Save"/>
</Grid>
'@
$grid = [Windows.Markup.XamlReader]::Parse($xamlText)
$grid.Measure([Windows.Size]::new(400, 200))
$grid.Arrange([Windows.Rect]::new(0, 0, 400, 200))

foreach ($child in $grid.Children) {
    $row = [Windows.Controls.Grid]::GetRow($child)
    $col = [Windows.Controls.Grid]::GetColumn($child)
    $span = [Windows.Controls.Grid]::GetColumnSpan($child)
    "$($child.GetType().Name) row $row col $col span $span width $([math]::Round($child.ActualWidth))"
}
"label column: $([math]::Round($grid.ColumnDefinitions[0].ActualWidth)), value column: $([math]::Round($grid.ColumnDefinitions[1].ActualWidth))"
