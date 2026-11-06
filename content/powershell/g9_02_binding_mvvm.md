@@lesson g9-mvvm Data Binding and MVVM: View Models and Commands
@@concepts objects, lists
@@text
In the last lesson, click handlers read and wrote controls directly. That works for small forms, but logic tangled with controls is hard to test and change. **MVVM** (Model–View–ViewModel) separates them:

- **Model**: your data and business logic (users, tickets, servers).
- **View**: the XAML. It contains no logic, only **bindings**.
- **ViewModel**: an object with properties and commands that the view binds to. It knows nothing about controls, so you can test it without any window.

## Bindings

```xml
<TextBox Text="{Binding UserName, UpdateSourceTrigger=PropertyChanged}"/>
<TextBlock Text="{Binding Status}"/>
<ListBox ItemsSource="{Binding Tickets}" DisplayMemberPath="Title"
         SelectedItem="{Binding Selected}"/>
<Button Content="Close ticket" Command="{Binding CloseCommand}"/>
```

Set `$window.DataContext = $viewModel` and every `{Binding X}` reads property `X` of the view model. TextBox bindings are **two-way**: typing updates the property.

## Telling the view that something changed

A view only refreshes a binding when the source raises **`INotifyPropertyChanged.PropertyChanged`**. PowerShell classes can implement it: the interface's event is implemented as two methods, `add_PropertyChanged` and `remove_PropertyChanged`:

```powershell,norun
class ObservableObject : System.ComponentModel.INotifyPropertyChanged {
    hidden [System.Collections.Generic.List[System.ComponentModel.PropertyChangedEventHandler]] $handlers =
        [System.Collections.Generic.List[System.ComponentModel.PropertyChangedEventHandler]]::new()
    [void] add_PropertyChanged([System.ComponentModel.PropertyChangedEventHandler]$h) { $this.handlers.Add($h) }
    [void] remove_PropertyChanged([System.ComponentModel.PropertyChangedEventHandler]$h) { [void]$this.handlers.Remove($h) }
    [void] Notify([string]$property) {
        foreach ($h in $this.handlers) { $h.Invoke($this, [System.ComponentModel.PropertyChangedEventArgs]::new($property)) }
    }
}
```

PowerShell class properties have no setter code, so change values through methods that call `Notify` afterwards: `[void] SetStatus([string]$s) { $this.Status = $s; $this.Notify('Status') }`.

For lists, use **`ObservableCollection[object]`**: adding or removing items updates a bound `ListBox` automatically.

## Commands

A button's `Command` binds to an object implementing **`System.Windows.Input.ICommand`**: `Execute($parameter)`, `CanExecute($parameter)` and a `CanExecuteChanged` event. When `CanExecute` returns false, WPF **disables** the button automatically. A reusable `RelayCommand` takes two script blocks:

```powershell,norun
class RelayCommand : System.Windows.Input.ICommand {
    hidden [scriptblock]$ExecuteBlock
    hidden [scriptblock]$CanExecuteBlock
    hidden [System.Collections.Generic.List[EventHandler]]$handlers = [System.Collections.Generic.List[EventHandler]]::new()
    RelayCommand([scriptblock]$execute, [scriptblock]$canExecute) { $this.ExecuteBlock = $execute; $this.CanExecuteBlock = $canExecute }
    [void] add_CanExecuteChanged([EventHandler]$h) { $this.handlers.Add($h) }
    [void] remove_CanExecuteChanged([EventHandler]$h) { [void]$this.handlers.Remove($h) }
    [bool] CanExecute([object]$p) { return [bool](& $this.CanExecuteBlock $p) }
    [void] Execute([object]$p) { & $this.ExecuteBlock $p }
    [void] RaiseCanExecuteChanged() { foreach ($h in $this.handlers) { $h.Invoke($this, [EventArgs]::Empty) } }
}
```

## Limits in Windows PowerShell 5.1

- A class can only use types that are loaded when the script is **parsed**. `INotifyPropertyChanged` and `ICommand` live in `System.dll`, which is always loaded, so they work; deriving from WPF types (`DependencyObject`) needs `using assembly PresentationFramework` at the very top of the script.
- Two-way bindings write directly into the property, without calling `Notify`; refresh dependent values yourself (or recompute them in a method the command calls).

## Testing view models

Because the view model has no controls, tests just call methods and check properties. In the exercises you'll also bind a real (hidden) view to check the wiring, pumping the dispatcher so WPF processes the bindings.

@@quiz
In MVVM, where does the logic for "close the selected ticket" live?
- In the button's click handler in the view
* In the view model, as a command the button binds to
- In the XAML
- In the model's constructor
? The view stays free of logic; the view model is testable.

@@quiz
Why does a TextBlock bound to `Status` not update when you assign `$vm.Status = 'x'`?
* The view model didn't raise PropertyChanged for `Status`, so the binding doesn't know it changed
- TextBlocks are read-only
- Bindings only work once
- Status is a reserved name
? Call Notify('Status') after changing it.

@@quiz
What happens to a button bound to a command whose CanExecute returns false?
- It's hidden
* WPF disables it automatically
- It throws when clicked
- Nothing
? Raise CanExecuteChanged when the answer may have changed.

@@drill
Which interface tells a WPF view that a property has changed?
= INotifyPropertyChanged
? Raise PropertyChanged with the property name.

@@flashcards
MVVM :: Model, View, ViewModel
`DataContext` :: object that bindings read from
`{Binding Prop}` :: bind to a property
`UpdateSourceTrigger=PropertyChanged` :: update while typing
`INotifyPropertyChanged` :: tells views a property changed
`add_PropertyChanged` :: implementing the event in a PS class
`ObservableCollection[object]` :: list that notifies changes
`ICommand` :: Execute, CanExecute, CanExecuteChanged
RelayCommand :: ICommand from script blocks
`using assembly` :: load an assembly before parsing

@@exercise An observable view model
Implement `ObservableObject` (as in the lesson) and a `SearchViewModel` that derives from it:

- properties `Filter` (string) and `Results` (an `ObservableCollection[object]`), plus `Status`
- `SetFilter($text)` stores the filter, refills `Results` from `$all` with the hosts that contain the text (case-insensitive), sets `Status` to `N of M shown`, and notifies for `Filter` and `Status`

Bind it to the view in the starter, pump the dispatcher, and print the TextBlock text and ListBox item count after `SetFilter('web')` and `SetFilter('db0')`. Also count how many PropertyChanged notifications were raised with a handler of your own.
@@starter
Add-Type -AssemblyName PresentationFramework
$all = 'web01', 'web02', 'db01', 'db02', 'cache01', 'WEB03'
$view = [Windows.Markup.XamlReader]::Parse(@'
<StackPanel xmlns="http://schemas.microsoft.com/winfx/2006/xaml/presentation">
  <TextBlock Name="StatusText" Text="{Binding Status}"/>
  <ListBox Name="List" ItemsSource="{Binding Results}"/>
</StackPanel>
'@)
function Wait-Dispatcher {
    $frame = [Windows.Threading.DispatcherFrame]::new()
    [void][Windows.Threading.Dispatcher]::CurrentDispatcher.BeginInvoke('Background', [Action]{ $frame.Continue = $false })
    [Windows.Threading.Dispatcher]::PushFrame($frame)
}
'TODO'
@@expected
'web': 3 of 6 shown, list items: 3 (web01, web02, WEB03)
'db0': 2 of 6 shown, list items: 2 (db01, db02)
notifications seen: 4
@@requires
INotifyPropertyChanged
add_PropertyChanged
ObservableCollection
DataContext
@@hint
`class SearchViewModel : ObservableObject { [string]$Filter = ''; [string]$Status = ''; $Results = [System.Collections.ObjectModel.ObservableCollection[object]]::new(); [object[]]$All; SearchViewModel([object[]]$all) { $this.All = $all } ... }`. In `SetFilter`, call `$this.Results.Clear()` then add matches (`-like "*$text*"` is case-insensitive). Count notifications: `$vm.add_PropertyChanged({ param($s, $e) $script:count++ })`. Print the list items with `$list.Items -join ', '`.
@@solution
Add-Type -AssemblyName PresentationFramework
$all = 'web01', 'web02', 'db01', 'db02', 'cache01', 'WEB03'
$view = [Windows.Markup.XamlReader]::Parse(@'
<StackPanel xmlns="http://schemas.microsoft.com/winfx/2006/xaml/presentation">
  <TextBlock Name="StatusText" Text="{Binding Status}"/>
  <ListBox Name="List" ItemsSource="{Binding Results}"/>
</StackPanel>
'@)
function Wait-Dispatcher {
    $frame = [Windows.Threading.DispatcherFrame]::new()
    [void][Windows.Threading.Dispatcher]::CurrentDispatcher.BeginInvoke('Background', [Action]{ $frame.Continue = $false })
    [Windows.Threading.Dispatcher]::PushFrame($frame)
}

class ObservableObject : System.ComponentModel.INotifyPropertyChanged {
    hidden [System.Collections.Generic.List[System.ComponentModel.PropertyChangedEventHandler]] $handlers =
        [System.Collections.Generic.List[System.ComponentModel.PropertyChangedEventHandler]]::new()
    [void] add_PropertyChanged([System.ComponentModel.PropertyChangedEventHandler]$h) { $this.handlers.Add($h) }
    [void] remove_PropertyChanged([System.ComponentModel.PropertyChangedEventHandler]$h) { [void]$this.handlers.Remove($h) }
    [void] Notify([string]$property) {
        foreach ($h in @($this.handlers)) { $h.Invoke($this, [System.ComponentModel.PropertyChangedEventArgs]::new($property)) }
    }
}

class SearchViewModel : ObservableObject {
    [string]$Filter = ''
    [string]$Status = ''
    $Results = [System.Collections.ObjectModel.ObservableCollection[object]]::new()
    [object[]]$All
    SearchViewModel([object[]]$all) { $this.All = $all }

    [void] SetFilter([string]$text) {
        $this.Filter = $text
        $this.Results.Clear()
        foreach ($h in $this.All) {
            if ($h -like "*$text*") { $this.Results.Add($h) }
        }
        $this.Status = "$($this.Results.Count) of $($this.All.Count) shown"
        $this.Notify('Filter')
        $this.Notify('Status')
    }
}

$vm = [SearchViewModel]::new($all)
$script:count = 0
$vm.add_PropertyChanged({ param($sender, $e) $script:count++ })
$view.DataContext = $vm
$status = $view.FindName('StatusText')
$list = $view.FindName('List')

foreach ($f in 'web', 'db0') {
    $vm.SetFilter($f)
    Wait-Dispatcher
    "'$f': $($status.Text), list items: $($list.Items.Count) ($($list.Items -join ', '))"
}
"notifications seen: $script:count"

@@exercise Commands that enable and disable buttons
Implement `RelayCommand` (as in the lesson) and a `TicketViewModel` with:

- `Tickets` (ObservableCollection of `[pscustomobject]@{ Id; Title; Open }`) and `Selected`
- `CloseCommand`: can execute only when a ticket is selected **and** open; executing sets `Open = $false`, sets `Status` to `closed #ID` and raises `CanExecuteChanged`
- `SelectTicket($ticket)` sets `Selected` and raises `CanExecuteChanged`

Bind a button to `CloseCommand` and show that WPF enables/disables it (`IsEnabled`) as the selection changes. Press the button through its automation peer (`ButtonAutomationPeer`, `Invoke` pattern), as a screen reader or UI test would.
@@starter
Add-Type -AssemblyName PresentationFramework, UIAutomationProvider
$view = [Windows.Markup.XamlReader]::Parse(@'
<StackPanel xmlns="http://schemas.microsoft.com/winfx/2006/xaml/presentation">
  <Button Name="CloseButton" Content="Close ticket" Command="{Binding CloseCommand}"/>
  <TextBlock Name="StatusText" Text="{Binding Status}"/>
</StackPanel>
'@)
function Wait-Dispatcher {
    $frame = [Windows.Threading.DispatcherFrame]::new()
    [void][Windows.Threading.Dispatcher]::CurrentDispatcher.BeginInvoke('Background', [Action]{ $frame.Continue = $false })
    [Windows.Threading.Dispatcher]::PushFrame($frame)
}
'TODO'
@@expected
nothing selected: button enabled = False
#102 selected: button enabled = True
after pressing: closed #102, button enabled = False
#103 selected: button enabled = True
open tickets: 101, 103
@@requires
System.Windows.Input.ICommand
add_CanExecuteChanged
ButtonAutomationPeer
@@hint
`$vm.CloseCommand = [RelayCommand]::new({ param($p) $vm.CloseSelected() }, { param($p) $null -ne $vm.Selected -and $vm.Selected.Open })`: the script blocks refer to `$vm`, which exists by the time they run. Press: `$peer = [Windows.Automation.Peers.ButtonAutomationPeer]::new($button); $peer.GetPattern('Invoke').Invoke()`, then `Wait-Dispatcher` (the invoke is queued). The ViewModel needs `ObservableObject` too for `Status` (copy it from the previous exercise).
@@solution
Add-Type -AssemblyName PresentationFramework, UIAutomationProvider
$view = [Windows.Markup.XamlReader]::Parse(@'
<StackPanel xmlns="http://schemas.microsoft.com/winfx/2006/xaml/presentation">
  <Button Name="CloseButton" Content="Close ticket" Command="{Binding CloseCommand}"/>
  <TextBlock Name="StatusText" Text="{Binding Status}"/>
</StackPanel>
'@)
function Wait-Dispatcher {
    $frame = [Windows.Threading.DispatcherFrame]::new()
    [void][Windows.Threading.Dispatcher]::CurrentDispatcher.BeginInvoke('Background', [Action]{ $frame.Continue = $false })
    [Windows.Threading.Dispatcher]::PushFrame($frame)
}

class ObservableObject : System.ComponentModel.INotifyPropertyChanged {
    hidden [System.Collections.Generic.List[System.ComponentModel.PropertyChangedEventHandler]] $handlers =
        [System.Collections.Generic.List[System.ComponentModel.PropertyChangedEventHandler]]::new()
    [void] add_PropertyChanged([System.ComponentModel.PropertyChangedEventHandler]$h) { $this.handlers.Add($h) }
    [void] remove_PropertyChanged([System.ComponentModel.PropertyChangedEventHandler]$h) { [void]$this.handlers.Remove($h) }
    [void] Notify([string]$property) {
        foreach ($h in @($this.handlers)) { $h.Invoke($this, [System.ComponentModel.PropertyChangedEventArgs]::new($property)) }
    }
}

class RelayCommand : System.Windows.Input.ICommand {
    hidden [scriptblock]$ExecuteBlock
    hidden [scriptblock]$CanExecuteBlock
    hidden [System.Collections.Generic.List[EventHandler]]$handlers = [System.Collections.Generic.List[EventHandler]]::new()
    RelayCommand([scriptblock]$execute, [scriptblock]$canExecute) { $this.ExecuteBlock = $execute; $this.CanExecuteBlock = $canExecute }
    [void] add_CanExecuteChanged([EventHandler]$h) { $this.handlers.Add($h) }
    [void] remove_CanExecuteChanged([EventHandler]$h) { [void]$this.handlers.Remove($h) }
    [bool] CanExecute([object]$p) { return [bool](& $this.CanExecuteBlock $p) }
    [void] Execute([object]$p) { & $this.ExecuteBlock $p }
    [void] RaiseCanExecuteChanged() { foreach ($h in @($this.handlers)) { $h.Invoke($this, [EventArgs]::Empty) } }
}

class TicketViewModel : ObservableObject {
    $Tickets = [System.Collections.ObjectModel.ObservableCollection[object]]::new()
    $Selected = $null
    [string]$Status = ''
    [RelayCommand]$CloseCommand

    [void] SelectTicket($ticket) {
        $this.Selected = $ticket
        $this.Notify('Selected')
        $this.CloseCommand.RaiseCanExecuteChanged()
    }
    [void] CloseSelected() {
        $this.Selected.Open = $false
        $this.Status = "closed #$($this.Selected.Id)"
        $this.Notify('Status')
        $this.CloseCommand.RaiseCanExecuteChanged()
    }
}

$vm = [TicketViewModel]::new()
foreach ($t in @(@(101, 'Printer jam'), @(102, 'VPN drops'), @(103, 'New laptop'))) {
    $vm.Tickets.Add([pscustomobject]@{ Id = $t[0]; Title = $t[1]; Open = $true })
}
$vm.CloseCommand = [RelayCommand]::new(
    { param($p) $vm.CloseSelected() },
    { param($p) $null -ne $vm.Selected -and $vm.Selected.Open })

$view.DataContext = $vm
$button = $view.FindName('CloseButton')
Wait-Dispatcher
"nothing selected: button enabled = $($button.IsEnabled)"

$vm.SelectTicket($vm.Tickets[1]); Wait-Dispatcher
"#102 selected: button enabled = $($button.IsEnabled)"

$peer = [Windows.Automation.Peers.ButtonAutomationPeer]::new($button)
$peer.GetPattern('Invoke').Invoke()
Wait-Dispatcher
"after pressing: $($view.FindName('StatusText').Text), button enabled = $($button.IsEnabled)"

$vm.SelectTicket($vm.Tickets[2]); Wait-Dispatcher
"#103 selected: button enabled = $($button.IsEnabled)"
"open tickets: $(($vm.Tickets | Where-Object Open | ForEach-Object Id) -join ', ')"
