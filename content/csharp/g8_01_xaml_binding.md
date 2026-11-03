@@lesson g8-binding XAML and Data Binding in Depth
@@concepts objects, functions
@@text
WPF, WinUI, .NET MAUI and Avalonia all describe the UI in **XAML** and connect it to your code through **data binding**. Once you understand the binding engine, all four feel the same. (WPF and WinUI are Windows-only; MAUI targets Windows, macOS, iOS and Android; Avalonia targets Windows, macOS, Linux, mobile and the browser.)

## XAML is object construction

```xml,norun
<StackPanel Margin="12" Spacing="8">
    <TextBox Text="{Binding Name, Mode=TwoWay, UpdateSourceTrigger=PropertyChanged}" />
    <TextBlock Text="{Binding Cpu, StringFormat='CPU {0}%'}" />
    <Border IsVisible="{Binding Online}" Background="Green" />
    <TextBlock Text="{Binding Owner.Email, FallbackValue='(no owner)'}" />
    <Button Content="Restart" Command="{Binding RestartCommand}" />
</StackPanel>
```

Every element is a `new StackPanel { Margin = ..., Children = { ... } }`. `{Binding Name}` means "read `Name` from my **DataContext**", which is inherited down the tree, so you usually set it once on the window to your ViewModel.

## Binding modes

| Mode | Direction | Typical use |
|---|---|---|
| `OneTime` | source → target once | values that never change |
| `OneWay` | source → target, on every change | labels, indicators |
| `TwoWay` | both ways | text boxes, checkboxes, sliders |
| `OneWayToSource` | target → source | rare: reading UI state into the ViewModel |

The engine listens to `INotifyPropertyChanged.PropertyChanged` on the source and to the control's own change notifications on the target. A **converter** (`IValueConverter`) translates between the two sides: bool to visibility, enum to color, bytes to "1.2 GB".

## Compiled bindings

Classic `{Binding}` uses reflection at runtime: typos only show up as warnings in the output window. **Compiled bindings** (`x:Bind` in WinUI, `x:DataType` in Avalonia and MAUI) are checked by the compiler and are faster. Turn them on.

## Less boilerplate with source generators

```csharp,norun
// CommunityToolkit.Mvvm generates the property, the change notifications and the command
public partial class ServerViewModel : ObservableObject
{
    [ObservableProperty]
    [NotifyPropertyChangedFor(nameof(Status))]
    private bool online;

    [ObservableProperty]
    private int cpu;

    public string Status => Online ? $"online, CPU {Cpu}%" : "offline";

    [RelayCommand(CanExecute = nameof(Online))]
    private async Task RestartAsync() { ... }      // generates RestartCommand
}
```

Under the hood it's exactly what you write by hand in the exercises: a `SetProperty` helper that compares, assigns and raises `PropertyChanged`, using `[CallerMemberName]` so you don't repeat property names as strings.

@@quiz
Where does `{Binding Name}` look for `Name`?
* On the element's DataContext, which is inherited from its parents
- On the element itself
- In a global dictionary
- In the code-behind class only
? Usually the window's DataContext is the ViewModel.

@@quiz
Which mode suits a TextBox that edits a ViewModel property?
- OneTime
- OneWay
* TwoWay
- OneWayToSource
? Changes flow in both directions.

@@quiz
What does `[CallerMemberName]` give you?
- The class name
* The name of the calling member, filled in by the compiler
- The current thread
- A stack trace
? `OnPropertyChanged()` inside the `Name` setter receives "Name".

@@flashcards
XAML :: markup that constructs UI objects
DataContext :: inherited binding source
`INotifyPropertyChanged` :: raise PropertyChanged
`[CallerMemberName]` :: compiler-supplied member name
`IValueConverter` :: Convert / ConvertBack
TwoWay :: source and target update each other
Compiled bindings :: x:Bind / x:DataType, checked at build time
`[ObservableProperty]` :: CommunityToolkit.Mvvm generated property
`[NotifyPropertyChangedFor]` :: also notify a dependent property

@@exercise ObservableObject with dependent properties
Write the base class every MVVM toolkit gives you:

- `abstract class ObservableObject : INotifyPropertyChanged` with `protected bool SetProperty<T>(ref T field, T value, [CallerMemberName] string? name = null)`: if the value is equal (`EqualityComparer<T>.Default`), return false and raise nothing; otherwise assign, raise `PropertyChanged` and return true
- `PersonViewModel` with `FirstName` and `LastName`, and a read-only `FullName` that must also be announced whenever either name changes
- record the names of raised properties and print them after each step
@@starter
using System.ComponentModel;
using System.Runtime.CompilerServices;

var raised = new List<string>();
Console.WriteLine("TODO");
@@expected
set FirstName=Ana: FirstName, FullName
set LastName=Sato: LastName, FullName
same LastName again: (none)
set FirstName=Kenji: FirstName, FullName
FullName = Kenji Sato
@@requires
[CallerMemberName]
EqualityComparer<T>.Default
@@hint
In the `FirstName` setter: `if (SetProperty(ref firstName, value)) OnPropertyChanged(nameof(FullName));`. Give the base class a `protected void OnPropertyChanged(string? name)`. In main, subscribe once with `vm.PropertyChanged += (_, e) => raised.Add(e.PropertyName!)`, and clear the list before each step.
@@solution
using System.ComponentModel;
using System.Runtime.CompilerServices;

var raised = new List<string>();
var vm = new PersonViewModel();
vm.PropertyChanged += (_, e) => raised.Add(e.PropertyName!);

Step("set FirstName=Ana", () => vm.FirstName = "Ana");
Step("set LastName=Sato", () => vm.LastName = "Sato");
Step("same LastName again", () => vm.LastName = "Sato");
Step("set FirstName=Kenji", () => vm.FirstName = "Kenji");
Console.WriteLine($"FullName = {vm.FullName}");

void Step(string label, Action action)
{
    raised.Clear();
    action();
    Console.WriteLine($"{label}: {(raised.Count == 0 ? "(none)" : string.Join(", ", raised))}");
}

abstract class ObservableObject : INotifyPropertyChanged
{
    public event PropertyChangedEventHandler? PropertyChanged;

    protected void OnPropertyChanged([CallerMemberName] string? name = null) =>
        PropertyChanged?.Invoke(this, new PropertyChangedEventArgs(name));

    protected bool SetProperty<T>(ref T field, T value, [CallerMemberName] string? name = null)
    {
        if (EqualityComparer<T>.Default.Equals(field, value)) return false;
        field = value;
        OnPropertyChanged(name);
        return true;
    }
}

class PersonViewModel : ObservableObject
{
    private string firstName = "";
    private string lastName = "";

    public string FirstName
    {
        get => firstName;
        set { if (SetProperty(ref firstName, value)) OnPropertyChanged(nameof(FullName)); }
    }

    public string LastName
    {
        get => lastName;
        set { if (SetProperty(ref lastName, value)) OnPropertyChanged(nameof(FullName)); }
    }

    public string FullName => $"{FirstName} {LastName}".Trim();
}

@@exercise A mini binding engine
Build the core of `{Binding}`:

- `class Binding(INotifyPropertyChanged source, string sourceProperty, INotifyPropertyChanged target, string targetProperty, Mode mode, IValueConverter? converter = null)` with `enum Mode { OneTime, OneWay, TwoWay }`
- on creation, copy source → target. `OneWay`/`TwoWay` keep copying on the source's `PropertyChanged`; `TwoWay` also copies target → source on the target's `PropertyChanged`
- use reflection (`GetProperty`, `GetValue`, `SetValue`). Without a converter, source → target uses `ToString()` and target → source uses `Convert.ChangeType` to the source property's type
- guard against update loops with a flag
- `interface IValueConverter { object? Convert(object? value); object? ConvertBack(object? value); }`; write `BoolToVisibility` (`Visible`/`Collapsed`) and `FormatConverter(string format)`
- `Control` is an observable class with `Text` and `Visibility` string properties
@@starter
using System.ComponentModel;
using System.Runtime.CompilerServices;

var vm = new ServerViewModel { Name = "web01", Online = true, Cpu = 42 };
var nameBox = new Control();
var badge = new Control();
var cpuLabel = new Control();
Console.WriteLine("TODO");

enum Mode { OneTime, OneWay, TwoWay }

interface IValueConverter
{
    object? Convert(object? value);
    object? ConvertBack(object? value);
}

abstract class Observable : INotifyPropertyChanged
{
    public event PropertyChangedEventHandler? PropertyChanged;

    protected void Set<T>(ref T field, T value, [CallerMemberName] string? name = null)
    {
        if (EqualityComparer<T>.Default.Equals(field, value)) return;
        field = value;
        PropertyChanged?.Invoke(this, new PropertyChangedEventArgs(name));
    }
}

class ServerViewModel : Observable
{
    private string name = "";
    private bool online;
    private int cpu;
    public string Name { get => name; set => Set(ref name, value); }
    public bool Online { get => online; set => Set(ref online, value); }
    public int Cpu { get => cpu; set => Set(ref cpu, value); }
}

class Control : Observable
{
    private string? text;
    private string? visibility;
    public string? Text { get => text; set => Set(ref text, value); }
    public string? Visibility { get => visibility; set => Set(ref visibility, value); }
}
@@expected
initial: name=web01 badge=Visible cpu=CPU 42%
vm changed: name=web01 badge=Collapsed cpu=CPU 42%
user typed web02: vm.Name=web02
vm.Name=db01: name=db01
@@requires
GetProperty
@@hint
In the constructor, look up both `PropertyInfo`s, then define two local functions `ToTarget()` and `ToSource()` that return early if `updating` is set, set it, copy the value (through the converter if there is one), and clear it in a `finally`. Call `ToTarget()` once, then subscribe depending on the mode. The CPU label uses `OneTime`, so it keeps showing 42 after `vm.Cpu = 97`.
@@solution
using System.ComponentModel;
using System.Runtime.CompilerServices;

var vm = new ServerViewModel { Name = "web01", Online = true, Cpu = 42 };
var nameBox = new Control();
var badge = new Control();
var cpuLabel = new Control();

var bindings = new List<Binding>
{
    new(vm, nameof(vm.Name), nameBox, nameof(Control.Text), Mode.TwoWay),
    new(vm, nameof(vm.Online), badge, nameof(Control.Visibility), Mode.OneWay, new BoolToVisibility()),
    new(vm, nameof(vm.Cpu), cpuLabel, nameof(Control.Text), Mode.OneTime, new FormatConverter("CPU {0}%")),
};

Console.WriteLine($"initial: name={nameBox.Text} badge={badge.Visibility} cpu={cpuLabel.Text}");
vm.Online = false;
vm.Cpu = 97;
Console.WriteLine($"vm changed: name={nameBox.Text} badge={badge.Visibility} cpu={cpuLabel.Text}");
nameBox.Text = "web02";
Console.WriteLine($"user typed web02: vm.Name={vm.Name}");
vm.Name = "db01";
Console.WriteLine($"vm.Name=db01: name={nameBox.Text}");

enum Mode { OneTime, OneWay, TwoWay }

interface IValueConverter
{
    object? Convert(object? value);
    object? ConvertBack(object? value);
}

class BoolToVisibility : IValueConverter
{
    public object? Convert(object? value) => value is true ? "Visible" : "Collapsed";
    public object? ConvertBack(object? value) => value is "Visible";
}

class FormatConverter(string format) : IValueConverter
{
    public object? Convert(object? value) => string.Format(format, value);
    public object? ConvertBack(object? value) => throw new NotSupportedException();
}

class Binding
{
    public Binding(INotifyPropertyChanged source, string sourceProperty, INotifyPropertyChanged target, string targetProperty, Mode mode, IValueConverter? converter = null)
    {
        var sp = source.GetType().GetProperty(sourceProperty) ?? throw new ArgumentException(sourceProperty);
        var tp = target.GetType().GetProperty(targetProperty) ?? throw new ArgumentException(targetProperty);
        bool updating = false;

        void ToTarget()
        {
            if (updating) return;
            updating = true;
            try
            {
                var value = sp.GetValue(source);
                tp.SetValue(target, converter is null ? value?.ToString() : converter.Convert(value));
            }
            finally
            {
                updating = false;
            }
        }

        void ToSource()
        {
            if (updating) return;
            updating = true;
            try
            {
                var value = tp.GetValue(target);
                sp.SetValue(source, converter is null ? Convert.ChangeType(value, sp.PropertyType) : converter.ConvertBack(value));
            }
            finally
            {
                updating = false;
            }
        }

        ToTarget();
        if (mode != Mode.OneTime)
            source.PropertyChanged += (_, e) => { if (e.PropertyName == sourceProperty) ToTarget(); };
        if (mode == Mode.TwoWay)
            target.PropertyChanged += (_, e) => { if (e.PropertyName == targetProperty) ToSource(); };
    }
}

abstract class Observable : INotifyPropertyChanged
{
    public event PropertyChangedEventHandler? PropertyChanged;

    protected void Set<T>(ref T field, T value, [CallerMemberName] string? name = null)
    {
        if (EqualityComparer<T>.Default.Equals(field, value)) return;
        field = value;
        PropertyChanged?.Invoke(this, new PropertyChangedEventArgs(name));
    }
}

class ServerViewModel : Observable
{
    private string name = "";
    private bool online;
    private int cpu;
    public string Name { get => name; set => Set(ref name, value); }
    public bool Online { get => online; set => Set(ref online, value); }
    public int Cpu { get => cpu; set => Set(ref cpu, value); }
}

class Control : Observable
{
    private string? text;
    private string? visibility;
    public string? Text { get => text; set => Set(ref text, value); }
    public string? Visibility { get => visibility; set => Set(ref visibility, value); }
}
