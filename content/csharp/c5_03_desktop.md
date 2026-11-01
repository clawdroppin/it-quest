@@lesson c5-desktop Desktop and Cross-Platform Apps: MVVM, WPF, MAUI and Shipping
@@text
C# is also the main language for **Windows desktop applications**: the internal tools, point-of-sale systems and factory software that run much of Japanese business. And with .NET MAUI and Avalonia, the same skills reach macOS, Linux, Android and iOS.

## Which UI framework?

| Framework | Runs on | Notes |
|---|---|---|
| **WinForms** | Windows | oldest and simplest; drag-and-drop designer; still everywhere in companies |
| **WPF** | Windows | XAML, data binding, MVVM; the standard for serious Windows apps |
| **WinUI 3** | Windows | Microsoft's newest Windows UI |
| **.NET MAUI** | Windows, macOS, Android, iOS | one XAML codebase for desktop and mobile |
| **Avalonia** | Windows, macOS, Linux, (mobile, web) | WPF-like XAML that also runs on Linux |
| **Blazor** | browser (and desktop via Blazor Hybrid) | web UI written in C# instead of JavaScript |

WPF, MAUI and Avalonia share one pattern, so learning it once is what matters: **MVVM**.

## MVVM: Model, View, ViewModel

```text
  View (XAML)  ──binds to──►  ViewModel (C#)  ──uses──►  Model / services
  buttons, text boxes          properties + commands       business rules, data
  no logic                     no UI types: testable        no UI at all
```

- The **View** is XAML. It doesn't contain logic; it **binds** to the ViewModel's properties: `<TextBox Text="{Binding Title}"/>`.
- The **ViewModel** exposes properties and **commands**. When a property changes, it raises `PropertyChanged`, and the bound controls update themselves.
- Because the ViewModel has no UI types in it, you can **test the whole screen's behaviour** without opening a window.

```text
<!-- TicketView.xaml -->
<StackPanel Margin="16">
  <TextBox Text="{Binding Title, UpdateSourceTrigger=PropertyChanged}" />
  <ComboBox ItemsSource="{Binding Priorities}" SelectedItem="{Binding Priority}" />
  <Button Content="Create ticket" Command="{Binding CreateCommand}" />
  <TextBlock Text="{Binding Message}" />
</StackPanel>
```

## INotifyPropertyChanged, by hand

This is what binding needs. It runs as a console program here, because the ViewModel doesn't know about windows:

```csharp
using System.ComponentModel;
using System.Runtime.CompilerServices;

var vm = new CounterViewModel();
vm.PropertyChanged += (_, e) => Console.WriteLine($"changed: {e.PropertyName}");   // what WPF does when you bind
vm.Count = 1;
vm.Count = 1;          // no change, no event
vm.Count = 5;
Console.WriteLine(vm.Label);

public class CounterViewModel : INotifyPropertyChanged
{
    public event PropertyChangedEventHandler? PropertyChanged;

    int count;
    public int Count
    {
        get => count;
        set
        {
            if (count == value) return;
            count = value;
            OnPropertyChanged();                       // [CallerMemberName] fills in "Count"
            OnPropertyChanged(nameof(Label));          // Label depends on Count
        }
    }

    public string Label => Count == 1 ? "1 ticket" : $"{Count} tickets";

    void OnPropertyChanged([CallerMemberName] string? name = null) =>
        PropertyChanged?.Invoke(this, new PropertyChangedEventArgs(name));
}
```

In real projects the **CommunityToolkit.Mvvm** package generates all of this from attributes:

```csharp,norun
public partial class TicketViewModel(ITicketService service) : ObservableObject
{
    [ObservableProperty]
    [NotifyCanExecuteChangedFor(nameof(CreateCommand))]
    private string title = "";

    [ObservableProperty] private string message = "";

    [RelayCommand(CanExecute = nameof(CanCreate))]
    private async Task CreateAsync()
    {
        var ticket = await service.CreateAsync(Title);
        Message = $"Created #{ticket.Id}";
        Title = "";
    }

    private bool CanCreate() => Title.Trim().Length >= 3;   // the button is disabled until this is true
}
```

## Commands

A **command** is an action the View can bind a button to, with a `CanExecute` rule that enables or disables the button automatically. That replaces click handlers in code-behind.

## Keep the UI responsive

The UI thread draws the window. Run slow work with `async`/`await`: the `await` frees the UI thread while waiting, and execution continues on the UI thread afterwards, so you can update bound properties safely.

```csharp,norun
[RelayCommand]
private async Task LoadAsync()
{
    IsBusy = true;                                  // show a spinner
    try { Tickets = new(await service.GetOpenTicketsAsync()); }
    catch (HttpRequestException) { Message = "Can't reach the server"; }
    finally { IsBusy = false; }
}
```

CPU-heavy work (parsing a 200 MB log) goes to `await Task.Run(() => Parse(file))`.

## Shipping a .NET app

```text
dotnet publish -c Release -r win-x64 --self-contained -p:PublishSingleFile=true
    → one .exe that runs without .NET installed (like IT Quest!)

dotnet publish -c Release -r linux-x64 --self-contained
dotnet publish -c Release -p:PublishAot=true      # native AOT: fast start-up, small, no JIT (console/API apps)
```

| Installer | Notes |
|---|---|
| **MSIX** | modern Windows packaging; clean install/uninstall; Microsoft Store or Intune |
| **Velopack** | installers with automatic updates for Windows, macOS and Linux |
| **WiX / Inno Setup** | classic .msi / .exe installers |
| **Intune / Configuration Manager** | how company IT deploys apps to thousands of PCs (you'll meet these in the Azure and A+ tracks) |

Sign your executables with a code-signing certificate, or Windows SmartScreen warns users about an "unknown publisher".

@@quiz
In MVVM, where does the logic for "the Create button is enabled only when the title has 3+ characters" go?
- In the XAML
- In a click handler in code-behind
* In the ViewModel, as the command's CanExecute
- In the database
? The ViewModel holds UI logic in a testable form. The View only binds to it.

@@quiz
What does raising `PropertyChanged` do?
* Tells bound controls to read the property again and update themselves
- Saves the property to disk
- Validates the property
- Creates a new window
? Data binding listens for this event. Forget to raise it and the screen shows stale data.

@@quiz
A WPF app freezes for 5 seconds while loading data. What's the usual fix?
- Use a faster PC
- Call `Thread.Sleep` less
* Make the load `async` and `await` the I/O, so the UI thread stays free
- Move the code into XAML
? While awaiting, the UI thread keeps drawing and handling clicks.

@@quiz
Which UI framework runs the same XAML app on Windows, macOS and Linux?
- WinForms
- WPF
* Avalonia
- WinUI 3
? WPF, WinForms and WinUI are Windows-only. MAUI covers Windows, macOS, Android and iOS, but not Linux desktop.

@@flashcards
MVVM :: Model, View (XAML), ViewModel (properties + commands, testable)
data binding :: `{Binding Title}` connects a control to a ViewModel property
`INotifyPropertyChanged` :: raises `PropertyChanged` so bound controls update
`[CallerMemberName]` :: the compiler fills in the calling property's name
command / `CanExecute` :: an action for a button, with an enabled rule
CommunityToolkit.Mvvm :: `[ObservableProperty]`, `[RelayCommand]` source generators
WPF / WinForms / WinUI :: Windows desktop UI frameworks
.NET MAUI :: cross-platform (Windows, macOS, Android, iOS) UI
Avalonia :: cross-platform XAML UI including Linux
`dotnet publish --self-contained -p:PublishSingleFile=true` :: one exe, no .NET needed
MSIX / Velopack :: Windows packaging / installers with auto-update

@@exercise A ViewModel with validation
Write `LoginViewModel` with `Email` and `Password` properties that raise `PropertyChanged` when they change (and only then), plus a read-only `CanLogin` that's true when the email contains `@` and the password has at least 8 characters. Whenever `Email` or `Password` changes, also raise `PropertyChanged` for `CanLogin`.
@@starter
using System.ComponentModel;
using System.Runtime.CompilerServices;

var vm = new LoginViewModel();
vm.PropertyChanged += (_, e) => Console.WriteLine($"{e.PropertyName} -> CanLogin={vm.CanLogin}");
vm.Email = "yuki@example.jp";
vm.Email = "yuki@example.jp";
vm.Password = "short";
vm.Password = "long-enough";

public class LoginViewModel : INotifyPropertyChanged
{
    public event PropertyChangedEventHandler? PropertyChanged;

    // TODO: Email, Password, CanLogin
}
@@expected
Email -> CanLogin=False
CanLogin -> CanLogin=False
Password -> CanLogin=False
CanLogin -> CanLogin=False
Password -> CanLogin=True
CanLogin -> CanLogin=True
@@hint
Use a backing field and a setter like: `set { if (email == value) return; email = value; OnPropertyChanged(); OnPropertyChanged(nameof(CanLogin)); }`. `CanLogin => Email.Contains('@') && Password.Length >= 8;` (start both fields as `""`). Add the `OnPropertyChanged([CallerMemberName] string? name = null)` helper.
@@solution
using System.ComponentModel;
using System.Runtime.CompilerServices;

var vm = new LoginViewModel();
vm.PropertyChanged += (_, e) => Console.WriteLine($"{e.PropertyName} -> CanLogin={vm.CanLogin}");
vm.Email = "yuki@example.jp";
vm.Email = "yuki@example.jp";
vm.Password = "short";
vm.Password = "long-enough";

public class LoginViewModel : INotifyPropertyChanged
{
    public event PropertyChangedEventHandler? PropertyChanged;

    string email = "";
    public string Email
    {
        get => email;
        set
        {
            if (email == value) return;
            email = value;
            OnPropertyChanged();
            OnPropertyChanged(nameof(CanLogin));
        }
    }

    string password = "";
    public string Password
    {
        get => password;
        set
        {
            if (password == value) return;
            password = value;
            OnPropertyChanged();
            OnPropertyChanged(nameof(CanLogin));
        }
    }

    public bool CanLogin => Email.Contains('@') && Password.Length >= 8;

    void OnPropertyChanged([CallerMemberName] string? name = null) =>
        PropertyChanged?.Invoke(this, new PropertyChangedEventArgs(name));
}

@@exercise A command with undo
Desktop apps need undo. Write `Editor` with a `Text` property, and an undo stack of **commands**: `Execute(IEditCommand cmd)` applies it and remembers it; `Undo()` reverts the latest one (and does nothing when there's nothing to undo). Write `AppendCommand(string text)` and `ReplaceCommand(string from, string to)` implementing `IEditCommand` (`Apply(string) → string` and `Revert(string) → string`). Hint: a command may remember what it needs to revert.
@@starter
var editor = new Editor();
editor.Execute(new AppendCommand("Server down"));
editor.Execute(new AppendCommand(" since 09:00"));
editor.Execute(new ReplaceCommand("down", "unreachable"));
Console.WriteLine(editor.Text);
editor.Undo();
Console.WriteLine(editor.Text);
editor.Undo();
editor.Undo();
editor.Undo();
Console.WriteLine($"[{editor.Text}]");

public interface IEditCommand
{
    string Apply(string text);
    string Revert(string text);
}

// TODO: Editor, AppendCommand, ReplaceCommand
@@expected
Server unreachable since 09:00
Server down since 09:00
[]
@@hint
`Editor`: `public string Text { get; private set; } = "";` and a `Stack<IEditCommand>`. `Execute`: `Text = cmd.Apply(Text); history.Push(cmd);`. `Undo`: `if (history.TryPop(out var cmd)) Text = cmd.Revert(Text);`. `ReplaceCommand` should store the text before it applied (`before = text`) and return it in `Revert`, because replacing back could change other places.
@@solution
var editor = new Editor();
editor.Execute(new AppendCommand("Server down"));
editor.Execute(new AppendCommand(" since 09:00"));
editor.Execute(new ReplaceCommand("down", "unreachable"));
Console.WriteLine(editor.Text);
editor.Undo();
Console.WriteLine(editor.Text);
editor.Undo();
editor.Undo();
editor.Undo();
Console.WriteLine($"[{editor.Text}]");

public interface IEditCommand
{
    string Apply(string text);
    string Revert(string text);
}

public class Editor
{
    readonly Stack<IEditCommand> history = new();
    public string Text { get; private set; } = "";

    public void Execute(IEditCommand cmd)
    {
        Text = cmd.Apply(Text);
        history.Push(cmd);
    }

    public void Undo()
    {
        if (history.TryPop(out var cmd)) Text = cmd.Revert(Text);
    }
}

public class AppendCommand(string added) : IEditCommand
{
    public string Apply(string text) => text + added;
    public string Revert(string text) => text[..^added.Length];
}

public class ReplaceCommand(string from, string to) : IEditCommand
{
    string before = "";

    public string Apply(string text)
    {
        before = text;
        return text.Replace(from, to);
    }

    public string Revert(string text) => before;
}
