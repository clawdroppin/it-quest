@@lesson gu1-desktop-languages Desktop GUIs in C#, PowerShell, Web Tech, C++ and Go
@@concepts objects, functions, automation
@@lang python
@@text
You know the ideas: widgets, layout, events, the event loop, and keeping logic separate. Here they are in the toolkits you're most likely to meet. You don't need to master all of them; read them side by side and notice how similar they are.

## C# WinForms: the quick Windows tool

WinForms (2002) is old but still everywhere in IT departments: fast to build, easy to understand. Create one with `dotnet new winforms -n PingTool`.

```csharp,norun
using System.Windows.Forms;

var form = new Form { Text = "Ping tool", Width = 360, Height = 180 };
var layout = new FlowLayoutPanel { Dock = DockStyle.Fill, Padding = new Padding(12) };
var host = new TextBox { Width = 200, PlaceholderText = "hostname" };
var go = new Button { Text = "Ping", AutoSize = true };
var result = new Label { AutoSize = true };

go.Click += async (s, e) =>                     // += attaches an event handler
{
    go.Enabled = false;
    var reply = await new System.Net.NetworkInformation.Ping().SendPingAsync(host.Text, 2000);
    result.Text = $"{reply.Status} in {reply.RoundtripTime} ms";
    go.Enabled = true;                          // await kept the window responsive
};

layout.Controls.AddRange(new Control[] { host, go, result });
form.Controls.Add(layout);
form.AcceptButton = go;                         // Enter presses Ping
Application.Run(form);                          // the event loop
```

Visual Studio has a drag-and-drop **designer** for WinForms, which writes the layout code into `Form1.Designer.cs` for you.

## C# WPF and XAML: separating the look from the code

WPF describes the window in **XAML**, an XML language, and the logic in C#. It draws everything itself (vector graphics, GPU accelerated), so you can style and animate *anything*.

```xml
<Window x:Class="Inventory.MainWindow" Title="Inventory" Width="420" Height="260"
        xmlns="http://schemas.microsoft.com/winfx/2006/xaml/presentation"
        xmlns:x="http://schemas.microsoft.com/winfx/2006/xaml">
  <StackPanel Margin="16">
    <TextBlock Text="Hostname" />
    <TextBox Text="{Binding Hostname, UpdateSourceTrigger=PropertyChanged}" />
    <TextBlock Text="{Binding Summary}" Margin="0,8" FontWeight="SemiBold" />
    <Button Content="Save" Command="{Binding SaveCommand}" HorizontalAlignment="Right" Padding="16,6" />
  </StackPanel>
</Window>
```

`{Binding Hostname}` is **data binding**: the text box and the `Hostname` property of a **ViewModel** class stay in sync automatically. The ViewModel raises `PropertyChanged` whenever a property changes, and WPF updates every control bound to it:

```csharp,norun
public class DeviceViewModel : INotifyPropertyChanged
{
    string hostname = "";
    public string Hostname
    {
        get => hostname;
        set { hostname = value; OnChanged(); OnChanged(nameof(Summary)); }
    }
    public string Summary => Hostname.Length == 0 ? "No name yet" : $"Will save {Hostname.ToUpper()}";

    public event PropertyChangedEventHandler? PropertyChanged;
    void OnChanged([CallerMemberName] string? name = null) =>
        PropertyChanged?.Invoke(this, new PropertyChangedEventArgs(name));
}
```

This is **MVVM**: the View (XAML) knows nothing but bindings, the ViewModel holds the state and commands, the Model holds the data. The ViewModel never touches a control, so it's unit-testable. The `CommunityToolkit.Mvvm` package writes the boilerplate for you (`[ObservableProperty]`, `[RelayCommand]`). WinUI 3, .NET MAUI and Avalonia use the same XAML-and-binding ideas.

## PowerShell windows

PowerShell can use WinForms (or WPF) directly, which is perfect for a small helper you hand to the service desk. It needs `-STA` on Windows PowerShell 5.1 for some dialogs, and it's Windows-only.

```powershell,norun
Add-Type -AssemblyName System.Windows.Forms
[System.Windows.Forms.Application]::EnableVisualStyles()

$form = New-Object System.Windows.Forms.Form -Property @{ Text = 'Unlock account'; Width = 340; Height = 160; StartPosition = 'CenterScreen' }
$user = New-Object System.Windows.Forms.TextBox -Property @{ Left = 16; Top = 16; Width = 200 }
$go   = New-Object System.Windows.Forms.Button  -Property @{ Left = 224; Top = 14; Text = 'Unlock'; AutoSize = $true }
$msg  = New-Object System.Windows.Forms.Label   -Property @{ Left = 16; Top = 56; Width = 300 }

$go.Add_Click({
    try   { Unlock-ADAccount -Identity $user.Text -ErrorAction Stop; $msg.Text = "Unlocked $($user.Text)" }
    catch { $msg.Text = "Failed: $($_.Exception.Message)" }
})
$form.Controls.AddRange(@($user, $go, $msg))
$form.AcceptButton = $go
[void]$form.ShowDialog()
```

For WPF, load XAML with `[Windows.Markup.XamlReader]::Parse($xaml)` and find controls with `$window.FindName('SaveButton')`. **Out-GridView** is the zero-effort GUI: `Get-Process | Out-GridView -PassThru | Stop-Process` gives you a searchable, multi-select picker in one line.

## Web technology on the desktop: Electron and Tauri

Many modern desktop apps are web pages in a window: VS Code, Slack, Discord, Teams (Electron), and this app (Tauri). You write the GUI in **HTML, CSS and JavaScript**, which is the most styleable and animatable GUI technology there is, and the rest of this track teaches it.

- **Electron** bundles Chromium and Node.js: easy, huge ecosystem, but apps are 100 MB+ and use a lot of memory.
- **Tauri** uses the system's own web view plus a Rust back end: apps of a few MB, with commands the page calls through `invoke`:

```js,norun
// in the page (front end)
import { invoke } from "@tauri-apps/api/core";
const free = await invoke("disk_free", { drive: "C:" });
```

```rust,norun
// in src-tauri (back end)
#[tauri::command]
fn disk_free(drive: String) -> Result<u64, String> { /* ... */ Ok(42) }
```

Start one with `npm create tauri-app@latest`. The front end can be plain HTML, or React, Vue, Svelte and friends.

## C++ with Qt

Qt is the classic choice for fast, professional cross-platform apps (KDE, many CAD, audio and medical tools). Its **signals and slots** connect events to handlers:

```cpp,norun
#include <QApplication>
#include <QPushButton>
#include <QVBoxLayout>
#include <QLabel>

int main(int argc, char *argv[]) {
    QApplication app(argc, argv);
    QWidget window;
    auto *layout = new QVBoxLayout(&window);
    auto *label = new QLabel("Ready");
    auto *button = new QPushButton("Run backup");
    layout->addWidget(label);
    layout->addWidget(button);

    QObject::connect(button, &QPushButton::clicked, [&] { label->setText("Backup started"); });

    window.show();
    return app.exec();
}
```

**Qt Quick / QML** is Qt's declarative, animation-friendly alternative, similar in spirit to XAML. Qt Creator includes a visual designer.

## Go with Fyne

```go,norun
package main

import (
	"fyne.io/fyne/v2/app"
	"fyne.io/fyne/v2/container"
	"fyne.io/fyne/v2/widget"
)

func main() {
	a := app.New()
	w := a.NewWindow("Hello Fyne")
	status := widget.NewLabel("Waiting")
	w.SetContent(container.NewVBox(
		status,
		widget.NewButton("Check", func() { status.SetText("All good") }),
	))
	w.ShowAndRun()
}
```

Fyne draws its own Material-style widgets and builds for Windows, macOS, Linux, Android and iOS from one code base.

## The same thing, everywhere

| | tkinter | WinForms | WPF | Qt | Web |
|---|---|---|---|---|---|
| handler | `command=fn` | `Click += fn` | `Command="{Binding}"` | `connect(sig, slot)` | `addEventListener` |
| layout | `grid`, `pack` | `TableLayoutPanel` | `Grid`, `StackPanel` | `QGridLayout` | CSS grid, flexbox |
| loop | `mainloop()` | `Application.Run` | `App.Run` | `app.exec()` | the browser |
| styling | ttk themes | properties | styles, templates | QSS | CSS |
| timer | `after()` | `Timer` | `DispatcherTimer` | `QTimer` | `setTimeout` |

## Data binding, by hand

Binding looks like magic, but it's just the **observer pattern**: an object keeps a list of functions to call when a value changes. The exercises below build it, so you know exactly what WPF, Vue, Svelte, Qt properties and tkinter's `StringVar` are doing for you.

@@quiz
In WPF, what does `Text="{Binding Hostname}"` do?
- Copies the value once when the window opens
* Keeps the control and the ViewModel's `Hostname` property in sync
- Calls a function named Hostname
- Validates the input
? Change the property and the control updates (via PropertyChanged); type in the box and the property updates.

@@quiz
Why should a ViewModel never touch a control (like `textBox1.Text = ...`) directly?
- It's slower
* So it can be unit-tested without a window and the view can change freely
- WPF doesn't allow it
- Controls are read-only
? The ViewModel exposes properties and commands; the XAML binds to them.

@@quiz
What's the main trade-off between Electron and Tauri?
- Electron can't use CSS
* Electron bundles a whole browser (big, memory-hungry, very consistent); Tauri uses the system web view with a Rust back end (tiny)
- Tauri only runs on Linux
- Electron is only for games
? Both let you build the GUI with HTML, CSS and JS.

@@quiz
What does `Get-Service | Out-GridView -PassThru | Restart-Service` do?
- Restarts every service
* Shows a searchable list window, and restarts only the services the user selects
- Exports services to a CSV
- Nothing, Out-GridView can't pass objects
? `-PassThru` sends the selected rows down the pipeline when the user clicks OK.

@@quiz
In Qt, what are signals and slots?
- Two kinds of layout
* Qt's events and the handlers connected to them
- Threads and locks
- Styles and themes
? `connect(button, &QPushButton::clicked, handler)` runs the handler when the button emits `clicked`.

@@flashcards
WinForms :: classic C# Windows GUI: controls with properties and `+=` event handlers
WPF :: C# + XAML; draws everything itself, so it's fully styleable and animatable
XAML :: an XML language describing a window's controls and layout
data binding :: a control and a property kept in sync automatically
`INotifyPropertyChanged` :: how a WPF ViewModel says "this property changed"
MVVM :: Model (data) / View (XAML) / ViewModel (state + commands, bound to the view)
`CommunityToolkit.Mvvm` :: `[ObservableProperty]` and `[RelayCommand]` remove MVVM boilerplate
`Add-Type -AssemblyName System.Windows.Forms` :: use WinForms from PowerShell
`Out-GridView -PassThru` :: a pick-from-a-list GUI in one PowerShell command
Electron :: Chromium + Node.js desktop apps (VS Code, Slack)
Tauri :: system web view + Rust back end; tiny desktop apps
`invoke` (Tauri) :: call a Rust command from the page
Qt / QML :: C++ (or Python) cross-platform GUI / its declarative UI language
Fyne :: a cross-platform GUI toolkit for Go
observer pattern :: subscribers are called when a value changes; the basis of data binding

@@exercise An observable value
Build the heart of data binding. `Observable(value)` holds a value. `subscribe(fn)` registers a function to call with the **new value** whenever it changes, and returns a function that **unsubscribes** it. `set(new)` stores the value and calls every subscriber, but **only if the value actually changed**. `get()` returns the value.
@@starter
class Observable:
    def __init__(self, value):
        pass

    def get(self):
        pass

    def set(self, new):
        pass

    def subscribe(self, fn):
        pass


name = Observable("")
name.subscribe(lambda v: print("label shows:", v))
name.set("tk-lt-01")
@@tests
seen = []
o = Observable(1)
check(o.get(), 1, "get returns the starting value")
stop = o.subscribe(seen.append)
o.set(2)
check(seen, [2], "a subscriber hears the new value")
o.set(2)
check(seen, [2], "setting the same value doesn't notify")
other = []
o.subscribe(other.append)
o.set(3)
check((seen, other), ([2, 3], [3]), "every subscriber is called")
stop()
o.set(4)
check((seen, other), ([2, 3], [3, 4]), "unsubscribed functions aren't called")
check(o.get(), 4, "get returns the latest value")
@@hint
Keep `self._subs = []`. In `subscribe`, append `fn` and return `lambda: self._subs.remove(fn)`. In `set`, `if new == self._value: return`, then store it and loop over a copy: `for fn in list(self._subs): fn(new)`.
@@solution
class Observable:
    def __init__(self, value):
        self._value = value
        self._subs = []

    def get(self):
        return self._value

    def set(self, new):
        if new == self._value:
            return
        self._value = new
        for fn in list(self._subs):
            fn(new)

    def subscribe(self, fn):
        self._subs.append(fn)
        return lambda: self._subs.remove(fn)


name = Observable("")
name.subscribe(lambda v: print("label shows:", v))
name.set("tk-lt-01")

@@exercise A ViewModel with a computed property
Use the observer idea to build a tiny ViewModel, like the WPF `DeviceViewModel` above. `DeviceVM()` has `hostname` (starts `""`). `on_change(fn)` registers `fn(property_name)`. Setting `vm.set_hostname(text)` stores the text **trimmed** and, if it changed, notifies `"hostname"` **and then** `"summary"` (because summary depends on it). `summary()` returns `"No name yet"` for an empty name, else `"Will save "` plus the name in upper case. `can_save()` is true when the name is 3 or more characters.
@@starter
class DeviceVM:
    def __init__(self):
        pass

    def on_change(self, fn):
        pass

    def set_hostname(self, text):
        pass

    def summary(self):
        pass

    def can_save(self):
        pass


vm = DeviceVM()
vm.on_change(lambda prop: print("changed:", prop))
vm.set_hostname("  tk-sw-01 ")
print(vm.summary())
@@tests
vm = DeviceVM()
changes = []
vm.on_change(changes.append)
check(vm.summary(), "No name yet", "empty summary")
check(vm.can_save(), False, "can't save an empty name")
vm.set_hostname("  pc ")
check(changes, ["hostname", "summary"], "notifies hostname then summary")
check(vm.hostname, "pc", "the name is trimmed")
check(vm.can_save(), False, "two letters is too short")
vm.set_hostname("pc")
check(changes, ["hostname", "summary"], "no notification when nothing changed")
vm.set_hostname("tk-sw-01")
check(vm.summary(), "Will save TK-SW-01", "summary in upper case")
check(vm.can_save(), True, "can save a real name")
check(len(changes), 4, "two more notifications")
@@hint
Store `self.hostname = ""` and `self._listeners = []`. In `set_hostname`, `text = text.strip()`; if it equals `self.hostname`, return; otherwise store it and call each listener with `"hostname"`, then each with `"summary"`.
@@solution
class DeviceVM:
    def __init__(self):
        self.hostname = ""
        self._listeners = []

    def on_change(self, fn):
        self._listeners.append(fn)

    def _notify(self, prop):
        for fn in self._listeners:
            fn(prop)

    def set_hostname(self, text):
        text = text.strip()
        if text == self.hostname:
            return
        self.hostname = text
        self._notify("hostname")
        self._notify("summary")

    def summary(self):
        if not self.hostname:
            return "No name yet"
        return "Will save " + self.hostname.upper()

    def can_save(self):
        return len(self.hostname) >= 3


vm = DeviceVM()
vm.on_change(lambda prop: print("changed:", prop))
vm.set_hostname("  tk-sw-01 ")
print(vm.summary())
