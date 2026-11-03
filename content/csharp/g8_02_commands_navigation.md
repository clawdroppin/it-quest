@@lesson g8-commands Async Commands, Navigation and Messaging
@@concepts objects, concurrency
@@text
Real desktop apps call APIs, databases and PowerShell scripts from buttons. This lesson covers the patterns that keep them responsive and organised.

## Async commands

A button bound to an `ICommand` calls `Execute` and asks `CanExecute` to decide whether it's enabled. For async work you want more than a plain `RelayCommand`:

- disable the button while the operation runs (no double-submits)
- expose `IsRunning` so the view can show a spinner
- catch exceptions and surface them, because an exception escaping an `async void` handler crashes the app
- optionally support cancellation (a Cancel button bound to a second command)

```csharp,norun
[RelayCommand(IncludeCancelCommand = true)]
private async Task SyncUsersAsync(CancellationToken ct)
{
    Status = "Syncing...";
    var count = await directory.SyncAsync(progress, ct);    // UI stays responsive
    Status = $"{count} users synced";
}
// generates SyncUsersCommand (an AsyncRelayCommand with IsRunning) and SyncUsersCancelCommand
```

## Navigation

ViewModel-first navigation: you navigate to a **ViewModel**, and a lookup (DataTemplates in WPF and Avalonia, Shell routes in MAUI) picks the view that renders it.

```csharp,norun
await navigation.NavigateToAsync<TicketEditViewModel>(ticketId);
// MAUI Shell: await Shell.Current.GoToAsync($"tickets/edit?id={ticketId}");
```

A navigation service keeps a back stack, passes parameters to the target (`OnNavigatedTo(parameter)`), and asks the current page whether it may be left. That's the "You have unsaved changes" dialog.

## Messaging

ViewModels shouldn't reference each other. A **messenger** (CommunityToolkit's `WeakReferenceMessenger`) lets the edit page announce `TicketSaved(42)` and the list page refresh itself, without either knowing about the other. Weak references mean a closed page that forgot to unsubscribe doesn't leak.

```csharp,norun
WeakReferenceMessenger.Default.Send(new TicketSaved(ticket.Id));
WeakReferenceMessenger.Default.Register<TicketSaved>(this, (r, m) => ((TicketListViewModel)r).Reload());
```

@@quiz
Why should an async command catch exceptions itself?
- For speed
* An exception escaping an async void event handler crashes the application
- Commands can't throw
- To retry automatically
? Surface the error in the UI instead.

@@quiz
In ViewModel-first navigation, what do you navigate to?
* A ViewModel type; the framework picks the matching view
- A XAML file path
- A window handle
- A URL only
? DataTemplates or Shell routes map ViewModels to views.

@@quiz
Why use a weak-reference messenger?
- It's faster
* Recipients that are no longer used can be garbage collected even if they never unregistered
- Messages are encrypted
- It works across processes
? It avoids a classic event-handler memory leak.

@@flashcards
`ICommand` :: Execute / CanExecute / CanExecuteChanged
`AsyncRelayCommand` :: async command with IsRunning
`IncludeCancelCommand` :: generate a cancel command
ViewModel-first navigation :: navigate to ViewModels
`OnNavigatedTo` :: receive the navigation parameter
Back stack :: history for GoBack
`WeakReferenceMessenger` :: decoupled ViewModel messages

@@exercise An async command
Write `AsyncRelayCommand : ICommand` (from `System.Windows.Input`):

- constructor `(Func<Task> execute, Func<bool>? canExecute = null)`
- `CanExecute` is false while running
- `Execute` ignores calls when it can't execute; otherwise starts the work and stores it in `ExecutionTask`
- sets `IsRunning` and raises `CanExecuteChanged` when it starts and when it finishes
- exceptions are caught into `Error` (and cleared on the next run)

Use a `TaskCompletionSource` to control when the work finishes, so the test is deterministic.
@@starter
using System.Windows.Input;

var log = new List<string>();
Console.WriteLine("TODO");
@@expected
running: True, can execute: False
second click ignored, runs: 1
runs: 1, running: False
error: ldap down, running: False
CanExecuteChanged -> False
CanExecuteChanged -> True
@@requires
ICommand
TaskCompletionSource
@@hint
`Execute(object? p) { if (!CanExecute(p)) return; ExecutionTask = RunAsync(); }`. `RunAsync`: set `IsRunning = true`, clear `Error`, raise; `try { await execute(); } catch (Exception e) { Error = e; } finally { IsRunning = false; raise; }`. The code before the first `await` runs synchronously, so `IsRunning` is already true when `Execute` returns. Collect the `CanExecuteChanged` lines in `log` and print them at the end.
@@solution
using System.Windows.Input;

var log = new List<string>();
var gate = new TaskCompletionSource();
int runs = 0;
var cmd = new AsyncRelayCommand(async () => { runs++; await gate.Task; });
cmd.CanExecuteChanged += (_, _) => log.Add($"CanExecuteChanged -> {cmd.CanExecute(null)}");

cmd.Execute(null);
Console.WriteLine($"running: {cmd.IsRunning}, can execute: {cmd.CanExecute(null)}");
cmd.Execute(null);
Console.WriteLine($"second click ignored, runs: {runs}");
gate.SetResult();
await cmd.ExecutionTask!;
Console.WriteLine($"runs: {runs}, running: {cmd.IsRunning}");

var failing = new AsyncRelayCommand(() => throw new InvalidOperationException("ldap down"));
failing.Execute(null);
await failing.ExecutionTask!;
Console.WriteLine($"error: {failing.Error?.Message}, running: {failing.IsRunning}");

foreach (var line in log) Console.WriteLine(line);

class AsyncRelayCommand(Func<Task> execute, Func<bool>? canExecute = null) : ICommand
{
    public event EventHandler? CanExecuteChanged;
    public bool IsRunning { get; private set; }
    public Exception? Error { get; private set; }
    public Task? ExecutionTask { get; private set; }

    public bool CanExecute(object? parameter) => !IsRunning && (canExecute?.Invoke() ?? true);

    public void Execute(object? parameter)
    {
        if (!CanExecute(parameter)) return;
        ExecutionTask = RunAsync();
    }

    private async Task RunAsync()
    {
        IsRunning = true;
        Error = null;
        CanExecuteChanged?.Invoke(this, EventArgs.Empty);
        try
        {
            await execute();
        }
        catch (Exception e)
        {
            Error = e;
        }
        finally
        {
            IsRunning = false;
            CanExecuteChanged?.Invoke(this, EventArgs.Empty);
        }
    }
}

@@exercise A navigation service with a leave guard
Write a ViewModel-first `Navigator`:

- pages implement `interface IPage { string Title { get; } void OnNavigatedTo(object? parameter); bool CanLeave(); }`
- `NavigateTo<T>(object? parameter = null) where T : IPage, new()` creates the page, calls `OnNavigatedTo` and pushes it
- `GoBack()` returns false at the root, or when `CanLeave()` of the current page is false; otherwise pops
- `Path` joins the titles from root to current with ` > `
- pages: `HomePage` (`Home`), `TicketListPage` (`Tickets (<filter>)`), `TicketEditPage` (`Ticket #<id>`, has `IsDirty` and `Save()`, can't be left while dirty)
@@starter
var nav = new Navigator();
Console.WriteLine("TODO");

interface IPage
{
    string Title { get; }
    void OnNavigatedTo(object? parameter);
    bool CanLeave();
}
@@expected
Home
Home > Tickets (open)
Home > Tickets (open) > Ticket #42
back blocked: unsaved changes in Ticket #42
saved Ticket #42
back: Home > Tickets (open)
back: Home
back: False (already at root)
@@hint
Keep a `Stack<IPage>`; `Path` is `string.Join(" > ", stack.Reverse().Select(p => p.Title))`. `NavigateTo` can return the new page so the test can set `IsDirty` on it. Give `IPage` default implementations (`void OnNavigatedTo(object? parameter) { }`, `bool CanLeave() => true;`) so simple pages don't need them.
@@solution
var nav = new Navigator();
nav.NavigateTo<HomePage>();
Console.WriteLine(nav.Path);
nav.NavigateTo<TicketListPage>("open");
Console.WriteLine(nav.Path);
var edit = nav.NavigateTo<TicketEditPage>(42);
Console.WriteLine(nav.Path);

edit.IsDirty = true;
if (!nav.GoBack()) Console.WriteLine($"back blocked: unsaved changes in {edit.Title}");
edit.Save();
Console.WriteLine($"saved {edit.Title}");
nav.GoBack();
Console.WriteLine($"back: {nav.Path}");
nav.GoBack();
Console.WriteLine($"back: {nav.Path}");
Console.WriteLine($"back: {nav.GoBack()} (already at root)");

interface IPage
{
    string Title { get; }
    void OnNavigatedTo(object? parameter) { }
    bool CanLeave() => true;
}

class Navigator
{
    private readonly Stack<IPage> stack = new();

    public string Path => string.Join(" > ", stack.Reverse().Select(p => p.Title));

    public T NavigateTo<T>(object? parameter = null) where T : IPage, new()
    {
        var page = new T();
        page.OnNavigatedTo(parameter);
        stack.Push(page);
        return page;
    }

    public bool GoBack()
    {
        if (stack.Count <= 1 || !stack.Peek().CanLeave()) return false;
        stack.Pop();
        return true;
    }
}

class HomePage : IPage
{
    public string Title => "Home";
}

class TicketListPage : IPage
{
    private string filter = "all";
    public string Title => $"Tickets ({filter})";
    public void OnNavigatedTo(object? parameter) => filter = parameter as string ?? "all";
}

class TicketEditPage : IPage
{
    private int id;
    public bool IsDirty { get; set; }
    public string Title => $"Ticket #{id}";
    public void OnNavigatedTo(object? parameter) => id = (int)parameter!;
    public bool CanLeave() => !IsDirty;
    public void Save() => IsDirty = false;
}
