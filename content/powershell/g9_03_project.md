@@lesson g9-project Project: A Help Desk Ticket App
@@concepts objects, lists, automation
@@text
The capstone: a small WPF help desk app in PowerShell, built the MVVM way. Technicians see open tickets sorted by priority, add new tickets with validation, filter by text, and close the selected ticket. You'll build and test it in two steps: first the view model on its own, then the window wired to it.

## The screen

```text
┌ Help desk ─────────────────────────────────────────┐
│ Filter: [vpn          ]          3 open, 1 high    │
│ ┌────────────────────────────────────────────────┐ │
│ │ #102 High   VPN drops every hour   (asato)     │ │
│ │ #105 Normal VPN slow from Osaka    (bkowalski) │ │
│ └────────────────────────────────────────────────┘ │
│ New: [title...........] [Normal ▾] [Add]  [Close]  │
│ Error: title must be 5-60 characters               │
└────────────────────────────────────────────────────┘
```

## View model responsibilities

| Member | Kind | Purpose |
|---|---|---|
| `Visible` | ObservableCollection | the filtered, sorted tickets the ListBox shows |
| `FilterText` | property | what the user typed (two-way binding) |
| `NewTitle`, `NewPriority` | properties | the "new ticket" inputs |
| `Summary`, `Error` | properties | text shown in the window |
| `AddCommand`, `CloseCommand` | commands | buttons |
| `Refresh()` | method | recompute `Visible` and `Summary` from all tickets |

All rules (validation, sorting, numbering) live in the view model, so tests cover them without a screen. The XAML only binds.

## Item templates

A `DataTemplate` describes how one item looks in a list:

```xml
<ListBox ItemsSource="{Binding Visible}" SelectedItem="{Binding Selected}">
  <ListBox.ItemTemplate>
    <DataTemplate>
      <TextBlock>
        <Run Text="{Binding Id, StringFormat=#{0}}"/>
        <Run Text="{Binding Priority}" FontWeight="Bold"/>
        <Run Text="{Binding Title}"/>
      </TextBlock>
    </DataTemplate>
  </ListBox.ItemTemplate>
</ListBox>
```

## Persisting tickets

Save to JSON on every change (`ConvertTo-Json -Depth 3 | Set-Content -Encoding UTF8`) and load at start-up; or, for a team, put the data behind a REST API or SharePoint list and keep the app as a thin client.

## Running it for real

```powershell,norun
$window.DataContext = $vm
$window.ShowDialog() | Out-Null
```

Package it as a `.ps1` with a shortcut (`powershell.exe -WindowStyle Hidden -File HelpDesk.ps1`) or wrap it with a tool like PS2EXE, and sign it (module 8).

@@quiz
Why keep the validation rules in the view model and not in the button's click handler?
* They can be tested by calling methods, without creating a window or simulating clicks
- WPF doesn't allow click handlers
- It's faster
- XAML can't contain buttons
? The view only binds; the logic is plain PowerShell.

@@quiz
What does a DataTemplate define?
- The window's size
* How each item of a bound list is displayed
- The data's JSON format
- The sort order
? One template, applied to every item.

@@quiz
Why does the ticket list use an ObservableCollection?
* It notifies the UI when items are added or removed, so the list updates by itself
- It saves tickets to disk
- It sorts tickets automatically
- It's faster than an array for reading
? Bindings watch the collection's change events.

@@drill
Which collection type updates a bound WPF list automatically when items are added?
= ObservableCollection
= ObservableCollection[T]
= System.Collections.ObjectModel.ObservableCollection
? A plain List doesn't notify the view.

@@flashcards
`DataTemplate` :: look of one list item
`ItemsSource` :: the list a control shows
`SelectedItem` binding :: selection into the view model
`StringFormat` :: format a bound value
`Refresh()` :: recompute derived state
`Mode=TwoWay` :: view ↔ view model
thin client :: UI over a shared data store

@@exercise The view model, tested on its own
Write `HelpDeskViewModel` (with `ObservableObject` from the previous lesson) and test it without any XAML:

- tickets have `Id` (starting at 101), `Title`, `Priority` (`High`, `Normal`, `Low`), `Owner`, `Open`
- `Add()` validates `NewTitle` (trimmed, 5–60 characters) and `NewPriority`; on error sets `Error` and adds nothing; on success adds the ticket, clears `NewTitle` and `Error`
- `Refresh()` fills `Visible` with **open** tickets whose title contains `FilterText`, sorted by priority (High first) then id, and sets `Summary` to `N open, M high`
- `CloseSelected()` closes `Selected` and refreshes
@@starter
'TODO'
@@expected
error: title must be 5-60 characters
#102 High VPN drops every hour (asato)
#101 Normal Printer on 3F jams (bkowalski)
#103 Normal New laptop for intern (asato)
#104 Low Update wiki page (cnguyen)
summary: 4 open, 1 high
filter 'in': 2 shown
after closing #102: 3 open, 0 high
@@requires
class HelpDeskViewModel
[void] Refresh()
ObservableCollection
@@hint
Keep all tickets in a `List[object]` and rebuild `Visible`: `$this.Visible.Clear(); $rank = @{ High = 0; Normal = 1; Low = 2 }; $this.All | Where-Object { $_.Open -and $_.Title -like "*$($this.FilterText)*" } | Sort-Object { $rank[$_.Priority] }, Id | ForEach-Object { $this.Visible.Add($_) }`. Inside a class method, `$rank` must be a local variable (classes can't see script variables unless you use `$script:`). Add a helper `AddTicket($title, $priority, $owner)` that sets the inputs and calls `Add()`.
@@solution
class ObservableObject : System.ComponentModel.INotifyPropertyChanged {
    hidden [System.Collections.Generic.List[System.ComponentModel.PropertyChangedEventHandler]] $handlers =
        [System.Collections.Generic.List[System.ComponentModel.PropertyChangedEventHandler]]::new()
    [void] add_PropertyChanged([System.ComponentModel.PropertyChangedEventHandler]$h) { $this.handlers.Add($h) }
    [void] remove_PropertyChanged([System.ComponentModel.PropertyChangedEventHandler]$h) { [void]$this.handlers.Remove($h) }
    [void] Notify([string]$property) {
        foreach ($h in @($this.handlers)) { $h.Invoke($this, [System.ComponentModel.PropertyChangedEventArgs]::new($property)) }
    }
}

class HelpDeskViewModel : ObservableObject {
    $All = [System.Collections.Generic.List[object]]::new()
    $Visible = [System.Collections.ObjectModel.ObservableCollection[object]]::new()
    $Selected = $null
    [string]$FilterText = ''
    [string]$NewTitle = ''
    [string]$NewPriority = 'Normal'
    [string]$NewOwner = ''
    [string]$Summary = ''
    [string]$Error = ''
    [int]$NextId = 101

    [void] Add() {
        $title = $this.NewTitle.Trim()
        if ($title.Length -lt 5 -or $title.Length -gt 60) {
            $this.Error = 'title must be 5-60 characters'
        } elseif ($this.NewPriority -notin 'High', 'Normal', 'Low') {
            $this.Error = "unknown priority $($this.NewPriority)"
        } else {
            $this.All.Add([pscustomobject]@{ Id = $this.NextId; Title = $title; Priority = $this.NewPriority; Owner = $this.NewOwner; Open = $true })
            $this.NextId++
            $this.NewTitle = ''
            $this.Error = ''
            $this.Notify('NewTitle')
            $this.Refresh()
        }
        $this.Notify('Error')
    }

    [void] AddTicket([string]$title, [string]$priority, [string]$owner) {
        $this.NewTitle = $title
        $this.NewPriority = $priority
        $this.NewOwner = $owner
        $this.Add()
    }

    [void] Refresh() {
        $rank = @{ High = 0; Normal = 1; Low = 2 }
        $filter = $this.FilterText
        $this.Visible.Clear()
        $this.All | Where-Object { $_.Open -and $_.Title -like "*$filter*" } |
            Sort-Object { $rank[$_.Priority] }, Id |
            ForEach-Object { $this.Visible.Add($_) }
        $open = @($this.All | Where-Object Open)
        $high = @($open | Where-Object Priority -eq 'High')
        $this.Summary = "$($open.Count) open, $($high.Count) high"
        $this.Notify('Summary')
    }

    [void] CloseSelected() {
        if ($null -eq $this.Selected) { return }
        $this.Selected.Open = $false
        $this.Selected = $null
        $this.Refresh()
    }
}

$vm = [HelpDeskViewModel]::new()
$vm.AddTicket('jam', 'Normal', 'x')
"error: $($vm.Error)"
$vm.AddTicket('Printer on 3F jams', 'Normal', 'bkowalski')
$vm.AddTicket('  VPN drops every hour ', 'High', 'asato')
$vm.AddTicket('New laptop for intern', 'Normal', 'asato')
$vm.AddTicket('Update wiki page', 'Low', 'cnguyen')
foreach ($t in $vm.Visible) { "#$($t.Id) $($t.Priority) $($t.Title) ($($t.Owner))" }
"summary: $($vm.Summary)"
$vm.FilterText = 'in'
$vm.Refresh()
"filter 'in': $($vm.Visible.Count) shown"
$vm.FilterText = ''
$vm.Selected = $vm.All | Where-Object Id -eq 102
$vm.CloseSelected()
"after closing #102: $($vm.Summary)"

@@exercise Wire up the window
Now connect a real `Window` (parsed, never shown) to the view model. The starter contains the view model and a `RelayCommand`; you write the XAML and the wiring:

- a `TextBox` bound two-way to `FilterText`, a `ListBox` with an item template showing `#Id Priority Title`, `TextBox` for `NewTitle`, a `ComboBox` for `NewPriority`, Add and Close buttons bound to commands, and `TextBlock`s for `Summary` and `Error`
- because two-way bindings write properties without calling `Refresh`, refresh when the filter box's `TextChanged` event fires
- drive it like a user: type a title, press Add (automation peer), type a filter, select the first visible ticket in the ListBox, press Close; print what the window shows after each step (the ListBox items through `$list.Items`)
@@starter
Add-Type -AssemblyName PresentationFramework, UIAutomationProvider
function Wait-Dispatcher {
    $frame = [Windows.Threading.DispatcherFrame]::new()
    [void][Windows.Threading.Dispatcher]::CurrentDispatcher.BeginInvoke('Background', [Action]{ $frame.Continue = $false })
    [Windows.Threading.Dispatcher]::PushFrame($frame)
}
class ObservableObject : System.ComponentModel.INotifyPropertyChanged {
    hidden [System.Collections.Generic.List[System.ComponentModel.PropertyChangedEventHandler]] $handlers = [System.Collections.Generic.List[System.ComponentModel.PropertyChangedEventHandler]]::new()
    [void] add_PropertyChanged([System.ComponentModel.PropertyChangedEventHandler]$h) { $this.handlers.Add($h) }
    [void] remove_PropertyChanged([System.ComponentModel.PropertyChangedEventHandler]$h) { [void]$this.handlers.Remove($h) }
    [void] Notify([string]$p) { foreach ($h in @($this.handlers)) { $h.Invoke($this, [System.ComponentModel.PropertyChangedEventArgs]::new($p)) } }
}
class RelayCommand : System.Windows.Input.ICommand {
    hidden [scriptblock]$Run; hidden [scriptblock]$Can
    hidden [System.Collections.Generic.List[EventHandler]]$handlers = [System.Collections.Generic.List[EventHandler]]::new()
    RelayCommand([scriptblock]$run, [scriptblock]$can) { $this.Run = $run; $this.Can = $can }
    [void] add_CanExecuteChanged([EventHandler]$h) { $this.handlers.Add($h) }
    [void] remove_CanExecuteChanged([EventHandler]$h) { [void]$this.handlers.Remove($h) }
    [bool] CanExecute([object]$p) { return [bool](& $this.Can $p) }
    [void] Execute([object]$p) { & $this.Run $p }
    [void] Changed() { foreach ($h in @($this.handlers)) { $h.Invoke($this, [EventArgs]::Empty) } }
}
class HelpDeskViewModel : ObservableObject {
    $All = [System.Collections.Generic.List[object]]::new()
    $Visible = [System.Collections.ObjectModel.ObservableCollection[object]]::new()
    $Selected = $null
    [string]$FilterText = ''; [string]$NewTitle = ''; [string]$NewPriority = 'Normal'
    [string]$Summary = ''; [string]$Error = ''; [int]$NextId = 101
    [RelayCommand]$AddCommand; [RelayCommand]$CloseCommand
    [void] Add() {
        $t = $this.NewTitle.Trim()
        if ($t.Length -lt 5 -or $t.Length -gt 60) { $this.Error = 'title must be 5-60 characters' }
        else {
            $this.All.Add([pscustomobject]@{ Id = $this.NextId; Title = $t; Priority = $this.NewPriority; Open = $true })
            $this.NextId++; $this.NewTitle = ''; $this.Error = ''; $this.Notify('NewTitle'); $this.Refresh()
        }
        $this.Notify('Error')
    }
    [void] Refresh() {
        $rank = @{ High = 0; Normal = 1; Low = 2 }; $f = $this.FilterText
        $this.Visible.Clear()
        $this.All | Where-Object { $_.Open -and $_.Title -like "*$f*" } | Sort-Object { $rank[$_.Priority] }, Id | ForEach-Object { $this.Visible.Add($_) }
        $open = @($this.All | Where-Object Open)
        $this.Summary = "$($open.Count) open, $(@($open | Where-Object Priority -eq 'High').Count) high"
        $this.Notify('Summary')
        if ($this.CloseCommand) { $this.CloseCommand.Changed() }
    }
    [void] CloseSelected() { $this.Selected.Open = $false; $this.Selected = $null; $this.Notify('Selected'); $this.Refresh() }
}
$vm = [HelpDeskViewModel]::new()
$vm.AddCommand = [RelayCommand]::new({ param($p) $vm.Add() }, { param($p) $true })
$vm.CloseCommand = [RelayCommand]::new({ param($p) $vm.CloseSelected() }, { param($p) $null -ne $vm.Selected })
foreach ($t in @(@('Printer on 3F jams', 'Normal'), @('VPN drops every hour', 'High'), @('VPN slow from Osaka', 'Normal'))) {
    $vm.NewTitle = $t[0]; $vm.NewPriority = $t[1]; $vm.Add()
}
'TODO'
@@expected
start: 3 open, 1 high | #102 High VPN drops every hour; #101 Normal Printer on 3F jams; #103 Normal VPN slow from Osaka
add 'Mon': title must be 5-60 characters | close enabled: False
add 'Monitor flickers' (Low): 4 open, 1 high | #102 High VPN drops every hour; #101 Normal Printer on 3F jams; #103 Normal VPN slow from Osaka; #104 Low Monitor flickers
filter 'vpn': #102 High VPN drops every hour; #103 Normal VPN slow from Osaka
selected #102, close enabled: True
closed: 3 open, 0 high | #103 Normal VPN slow from Osaka
@@requires
<Window
{Binding FilterText
ItemTemplate
Add_TextChanged
ButtonAutomationPeer
@@hint
Set the window's `DataContext = $vm`, `Wait-Dispatcher`, then find controls by name. Type into a TextBox by setting `.Text` (the binding pushes it into the view model because of `UpdateSourceTrigger=PropertyChanged`). Choose the priority with `$combo.SelectedItem = 'Low'` when the ComboBox's `ItemsSource` is a string array (`$combo.ItemsSource = 'High', 'Normal', 'Low'`) bound with `SelectedItem="{Binding NewPriority}"`. Select a ticket with `$list.SelectedIndex = 0`. Describe the list: `($list.Items | ForEach-Object { "#$($_.Id) $($_.Priority) $($_.Title)" }) -join '; '`.
@@solution
Add-Type -AssemblyName PresentationFramework, UIAutomationProvider
function Wait-Dispatcher {
    $frame = [Windows.Threading.DispatcherFrame]::new()
    [void][Windows.Threading.Dispatcher]::CurrentDispatcher.BeginInvoke('Background', [Action]{ $frame.Continue = $false })
    [Windows.Threading.Dispatcher]::PushFrame($frame)
}
class ObservableObject : System.ComponentModel.INotifyPropertyChanged {
    hidden [System.Collections.Generic.List[System.ComponentModel.PropertyChangedEventHandler]] $handlers = [System.Collections.Generic.List[System.ComponentModel.PropertyChangedEventHandler]]::new()
    [void] add_PropertyChanged([System.ComponentModel.PropertyChangedEventHandler]$h) { $this.handlers.Add($h) }
    [void] remove_PropertyChanged([System.ComponentModel.PropertyChangedEventHandler]$h) { [void]$this.handlers.Remove($h) }
    [void] Notify([string]$p) { foreach ($h in @($this.handlers)) { $h.Invoke($this, [System.ComponentModel.PropertyChangedEventArgs]::new($p)) } }
}
class RelayCommand : System.Windows.Input.ICommand {
    hidden [scriptblock]$Run; hidden [scriptblock]$Can
    hidden [System.Collections.Generic.List[EventHandler]]$handlers = [System.Collections.Generic.List[EventHandler]]::new()
    RelayCommand([scriptblock]$run, [scriptblock]$can) { $this.Run = $run; $this.Can = $can }
    [void] add_CanExecuteChanged([EventHandler]$h) { $this.handlers.Add($h) }
    [void] remove_CanExecuteChanged([EventHandler]$h) { [void]$this.handlers.Remove($h) }
    [bool] CanExecute([object]$p) { return [bool](& $this.Can $p) }
    [void] Execute([object]$p) { & $this.Run $p }
    [void] Changed() { foreach ($h in @($this.handlers)) { $h.Invoke($this, [EventArgs]::Empty) } }
}
class HelpDeskViewModel : ObservableObject {
    $All = [System.Collections.Generic.List[object]]::new()
    $Visible = [System.Collections.ObjectModel.ObservableCollection[object]]::new()
    $Selected = $null
    [string]$FilterText = ''; [string]$NewTitle = ''; [string]$NewPriority = 'Normal'
    [string]$Summary = ''; [string]$Error = ''; [int]$NextId = 101
    [RelayCommand]$AddCommand; [RelayCommand]$CloseCommand
    [void] Add() {
        $t = $this.NewTitle.Trim()
        if ($t.Length -lt 5 -or $t.Length -gt 60) { $this.Error = 'title must be 5-60 characters' }
        else {
            $this.All.Add([pscustomobject]@{ Id = $this.NextId; Title = $t; Priority = $this.NewPriority; Open = $true })
            $this.NextId++; $this.NewTitle = ''; $this.Error = ''; $this.Notify('NewTitle'); $this.Refresh()
        }
        $this.Notify('Error')
    }
    [void] Refresh() {
        $rank = @{ High = 0; Normal = 1; Low = 2 }; $f = $this.FilterText
        $this.Visible.Clear()
        $this.All | Where-Object { $_.Open -and $_.Title -like "*$f*" } | Sort-Object { $rank[$_.Priority] }, Id | ForEach-Object { $this.Visible.Add($_) }
        $open = @($this.All | Where-Object Open)
        $this.Summary = "$($open.Count) open, $(@($open | Where-Object Priority -eq 'High').Count) high"
        $this.Notify('Summary')
        if ($this.CloseCommand) { $this.CloseCommand.Changed() }
    }
    [void] CloseSelected() { $this.Selected.Open = $false; $this.Selected = $null; $this.Notify('Selected'); $this.Refresh() }
}
$vm = [HelpDeskViewModel]::new()
$vm.AddCommand = [RelayCommand]::new({ param($p) $vm.Add() }, { param($p) $true })
$vm.CloseCommand = [RelayCommand]::new({ param($p) $vm.CloseSelected() }, { param($p) $null -ne $vm.Selected })
foreach ($t in @(@('Printer on 3F jams', 'Normal'), @('VPN drops every hour', 'High'), @('VPN slow from Osaka', 'Normal'))) {
    $vm.NewTitle = $t[0]; $vm.NewPriority = $t[1]; $vm.Add()
}

$window = [Windows.Markup.XamlReader]::Parse(@'
<Window xmlns="http://schemas.microsoft.com/winfx/2006/xaml/presentation" Title="Help desk" Width="520" Height="360">
  <DockPanel Margin="8">
    <DockPanel DockPanel.Dock="Top">
      <TextBlock Name="SummaryText" DockPanel.Dock="Right" Text="{Binding Summary}"/>
      <TextBlock Text="Filter: "/>
      <TextBox Name="FilterBox" Text="{Binding FilterText, UpdateSourceTrigger=PropertyChanged}"/>
    </DockPanel>
    <TextBlock Name="ErrorText" DockPanel.Dock="Bottom" Foreground="Firebrick" Text="{Binding Error}"/>
    <StackPanel DockPanel.Dock="Bottom" Orientation="Horizontal">
      <TextBox Name="TitleBox" Width="200" Text="{Binding NewTitle, UpdateSourceTrigger=PropertyChanged}"/>
      <ComboBox Name="PriorityBox" Width="80" SelectedItem="{Binding NewPriority}"/>
      <Button Name="AddButton" Content="Add" Command="{Binding AddCommand}"/>
      <Button Name="CloseButton" Content="Close" Command="{Binding CloseCommand}"/>
    </StackPanel>
    <ListBox Name="TicketList" ItemsSource="{Binding Visible}" SelectedItem="{Binding Selected}">
      <ListBox.ItemTemplate>
        <DataTemplate>
          <TextBlock>
            <Run Text="{Binding Id, StringFormat=#{0}, Mode=OneWay}"/>
            <Run Text="{Binding Priority, Mode=OneWay}" FontWeight="Bold"/>
            <Run Text="{Binding Title, Mode=OneWay}"/>
          </TextBlock>
        </DataTemplate>
      </ListBox.ItemTemplate>
    </ListBox>
  </DockPanel>
</Window>
'@)

$ui = @{}
foreach ($n in 'SummaryText', 'FilterBox', 'ErrorText', 'TitleBox', 'PriorityBox', 'AddButton', 'CloseButton', 'TicketList') { $ui[$n] = $window.FindName($n) }
$ui.PriorityBox.ItemsSource = 'High', 'Normal', 'Low'
$ui.FilterBox.Add_TextChanged({ $vm.Refresh() })
$window.DataContext = $vm
Wait-Dispatcher

function Show-List { ($ui.TicketList.Items | ForEach-Object { "#$($_.Id) $($_.Priority) $($_.Title)" }) -join '; ' }
function Press($button) {
    ([Windows.Automation.Peers.ButtonAutomationPeer]::new($button)).GetPattern('Invoke').Invoke()
    Wait-Dispatcher
}

"start: $($ui.SummaryText.Text) | $(Show-List)"
$ui.TitleBox.Text = 'Mon'; Wait-Dispatcher
Press $ui.AddButton
"add 'Mon': $($ui.ErrorText.Text) | close enabled: $($ui.CloseButton.IsEnabled)"
$ui.TitleBox.Text = 'Monitor flickers'
$ui.PriorityBox.SelectedItem = 'Low'; Wait-Dispatcher
Press $ui.AddButton
"add 'Monitor flickers' (Low): $($ui.SummaryText.Text) | $(Show-List)"
$ui.FilterBox.Text = 'vpn'; Wait-Dispatcher
"filter 'vpn': $(Show-List)"
$ui.TicketList.SelectedIndex = 0; Wait-Dispatcher
$vm.CloseCommand.Changed(); Wait-Dispatcher
"selected #$($vm.Selected.Id), close enabled: $($ui.CloseButton.IsEnabled)"
Press $ui.CloseButton
"closed: $($ui.SummaryText.Text) | $(Show-List)"
