@@lesson g8-project Project: An IT Asset Desk App
@@concepts objects, lists
@@text
The capstone for the desktop module is an **asset desk**: a Windows (WPF/WinUI) or cross-platform (Avalonia/MAUI) app where the service desk registers laptops and phones, assigns them to people, and edits them safely. The structure:

```text,norun
AssetDesk/
  AssetDesk.Core/          models, validation, services (no UI references: unit-testable)
  AssetDesk.ViewModels/    ObservableObject ViewModels, commands, history
  AssetDesk.App/           XAML views, DataTemplates, DI setup, styles
  AssetDesk.Tests/         xUnit tests for Core and ViewModels
```

Features that make it feel professional:

- **validation as you type** with `INotifyDataErrorInfo`: red borders and messages under the fields, Save disabled while there are errors
- **undo/redo** (Ctrl+Z / Ctrl+Y) where typing a word counts as one step, not one step per key
- an **unsaved changes** indicator (`*` in the title) that disappears when you undo back to the saved state
- the list page from the previous lesson, a search box with debouncing, and export to CSV off the UI thread
- packaging: MSIX or Velopack for Windows, `dotnet publish -r osx-arm64` / `linux-x64` for Avalonia

## INotifyDataErrorInfo

```csharp,norun
public interface INotifyDataErrorInfo
{
    bool HasErrors { get; }
    event EventHandler<DataErrorsChangedEventArgs>? ErrorsChanged;
    IEnumerable GetErrors(string? propertyName);
}
```

Controls bound to a property call `GetErrors` for it whenever `ErrorsChanged` names it, and show the messages. Raise `ErrorsChanged` only when a property's errors actually change; otherwise every keystroke redraws the error templates. CommunityToolkit's `ObservableValidator` implements this with `[Required]`, `[RegularExpression]` and custom attributes.

## Undo with coalescing

Each edit pushes an entry `(property, old value, new value)`. To make typing one step, merge an edit into the previous entry when it targets the same property within a short window (say 1 second). Keep only the latest N entries. Track a **state id** per entry so "dirty" means "current state differs from the saved one", which stays correct after undo and redo.

@@quiz
When should a ViewModel raise `ErrorsChanged`?
- On every property set
* When the set of errors for a property actually changes
- Only on save
- Never; controls poll GetErrors
? Avoid redrawing error templates on every keystroke.

@@quiz
Why coalesce consecutive edits of the same field?
* So typing a word is one undo step instead of one per keystroke
- To save memory only
- Because the stack can't hold strings
- To make redo impossible
? Merge within a short time window.

@@quiz
What does the Undo stack store in the asset desk app?
* Entries recording the property, its old value and its new value
- Screenshots of the window
- The whole database after every keypress
- Only the last value
? Undo applies the old value; redo applies the new one.

@@drill
Which interface reports per-property validation errors to WPF?
= INotifyDataErrorInfo
? The view shows the errors next to each field.

@@flashcards
`INotifyDataErrorInfo` :: per-property validation errors
`ErrorsChanged` :: tell bindings to re-read errors
`HasErrors` :: disable Save
`ObservableValidator` :: CommunityToolkit validation base
Coalescing :: merge edits within a window
Save point :: the state id when last saved
MSIX / Velopack :: Windows packaging and updates

@@exercise Validation with INotifyDataErrorInfo
Write `AssetViewModel : INotifyDataErrorInfo` for the asset form:

- constructor `(DateOnly today)`; validate `AssetTag` and `Owner` immediately (a new form starts invalid)
- `AssetTag`: `required`, else must match `^IT-\d{5}$` (`must look like IT-12345`)
- `Owner`: `required`
- `PurchaseDate` (`DateOnly?`): `cannot be in the future`
- raise `ErrorsChanged` only when a property's error changes; `CanSave` is `!HasErrors`

Print each step, the events it raised, and the current errors in the order AssetTag, Owner, PurchaseDate.
@@starter
using System.Collections;
using System.ComponentModel;
using System.Text.RegularExpressions;

var vm = new AssetViewModel(new DateOnly(2026, 11, 3));
Console.WriteLine("TODO");
@@expected
initial: AssetTag: required; Owner: required | can save: False
AssetTag=IT-12
  ErrorsChanged(AssetTag)
  AssetTag: must look like IT-12345; Owner: required | can save: False
AssetTag=IT-12345
  ErrorsChanged(AssetTag)
  Owner: required | can save: False
AssetTag=IT-54321
  Owner: required | can save: False
Owner=ana
  ErrorsChanged(Owner)
  none | can save: True
PurchaseDate=2027-01-10
  ErrorsChanged(PurchaseDate)
  PurchaseDate: cannot be in the future | can save: False
PurchaseDate=2026-10-01
  ErrorsChanged(PurchaseDate)
  none | can save: True
@@requires
INotifyDataErrorInfo
ErrorsChanged
@@hint
Store the errors in a `Dictionary<string, string>`. A `Check(string property)` method with a `switch` on the property name returns the message or null. `Validate(property)`: compare the new message with the stored one; if they differ, update the dictionary and raise `ErrorsChanged`. `GetErrors` returns a one-element array or an empty one.
@@solution
using System.Collections;
using System.ComponentModel;
using System.Text.RegularExpressions;

var vm = new AssetViewModel(new DateOnly(2026, 11, 3));
vm.ErrorsChanged += (_, e) => Console.WriteLine($"  ErrorsChanged({e.PropertyName})");
Console.WriteLine($"initial: {Describe()}");
Step("AssetTag=IT-12", () => vm.AssetTag = "IT-12");
Step("AssetTag=IT-12345", () => vm.AssetTag = "IT-12345");
Step("AssetTag=IT-54321", () => vm.AssetTag = "IT-54321");
Step("Owner=ana", () => vm.Owner = "ana");
Step("PurchaseDate=2027-01-10", () => vm.PurchaseDate = new DateOnly(2027, 1, 10));
Step("PurchaseDate=2026-10-01", () => vm.PurchaseDate = new DateOnly(2026, 10, 1));

void Step(string label, Action action)
{
    Console.WriteLine(label);
    action();
    Console.WriteLine($"  {Describe()}");
}

string Describe()
{
    var errors = new[] { "AssetTag", "Owner", "PurchaseDate" }
        .SelectMany(p => vm.GetErrors(p).Cast<string>().Select(e => $"{p}: {e}"))
        .ToList();
    return $"{(errors.Count == 0 ? "none" : string.Join("; ", errors))} | can save: {vm.CanSave}";
}

class AssetViewModel : INotifyDataErrorInfo
{
    private readonly DateOnly today;
    private readonly Dictionary<string, string> errors = new();
    private string assetTag = "";
    private string owner = "";
    private DateOnly? purchaseDate;

    public AssetViewModel(DateOnly today)
    {
        this.today = today;
        Validate(nameof(AssetTag));
        Validate(nameof(Owner));
    }

    public event EventHandler<DataErrorsChangedEventArgs>? ErrorsChanged;
    public bool HasErrors => errors.Count > 0;
    public bool CanSave => !HasErrors;

    public string AssetTag
    {
        get => assetTag;
        set { assetTag = value; Validate(nameof(AssetTag)); }
    }

    public string Owner
    {
        get => owner;
        set { owner = value; Validate(nameof(Owner)); }
    }

    public DateOnly? PurchaseDate
    {
        get => purchaseDate;
        set { purchaseDate = value; Validate(nameof(PurchaseDate)); }
    }

    public IEnumerable GetErrors(string? propertyName) =>
        propertyName is not null && errors.TryGetValue(propertyName, out var e) ? new[] { e } : Array.Empty<string>();

    private string? Check(string property) => property switch
    {
        nameof(AssetTag) when assetTag.Length == 0 => "required",
        nameof(AssetTag) when !Regex.IsMatch(assetTag, @"^IT-\d{5}$") => "must look like IT-12345",
        nameof(Owner) when string.IsNullOrWhiteSpace(owner) => "required",
        nameof(PurchaseDate) when purchaseDate > today => "cannot be in the future",
        _ => null,
    };

    private void Validate(string property)
    {
        string? error = Check(property);
        errors.TryGetValue(property, out var old);
        if (error == old) return;
        if (error is null) errors.Remove(property);
        else errors[property] = error;
        ErrorsChanged?.Invoke(this, new DataErrorsChangedEventArgs(property));
    }
}

@@exercise Undo/redo with coalescing and a save point
Write `History` for the asset editor:

- constructor `(TimeSpan mergeWindow, int capacity)`
- `Do(string property, string oldValue, string newValue, Action<string> set, DateTime at)`: applies `newValue`; merges into the top entry if it has the same property and was made within `mergeWindow` (keep the top's old value, take the new value and time); otherwise pushes a new entry, dropping the oldest when over capacity; always clears redo
- `Undo()` / `Redo()` apply the old/new value and move the entry between the stacks
- `MarkSaved()` and `IsDirty`: give every state an id (each entry has `Before` and `After` ids, and a merge gets a fresh `After`), and compare the current id with the saved one
- `UndoCount`, `RedoCount`
@@starter
var asset = new Asset { Name = "laptop", Owner = "ana" };
var t0 = new DateTime(2026, 11, 3, 9, 0, 0);
Console.WriteLine("TODO");

class Asset
{
    public string Name { get; set; } = "";
    public string Owner { get; set; } = "";
}
@@expected
typed laptop-01: name=laptop-01 owner=ana undo=1 redo=0 dirty=True
saved: name=laptop-01 owner=ana undo=1 redo=0 dirty=False
owner=ben: name=laptop-01 owner=ben undo=2 redo=0 dirty=True
undo: name=laptop-01 owner=ana undo=1 redo=1 dirty=False
undo: name=laptop owner=ana undo=0 redo=2 dirty=True
redo: name=laptop-01 owner=ana undo=1 redo=1 dirty=False
owner=cho: name=laptop-01 owner=cho undo=2 redo=0 dirty=True
three more edits: name=x2 owner=dee undo=3 redo=0 dirty=True
undo all: name=laptop-01 owner=cho undo=0 redo=3 dirty=True
@@requires
MarkSaved
@@hint
`record Entry(string Property, string Old, string New, Action<string> Set, DateTime At, int Before, int After)`. Keep the undo history in a `List<Entry>` (so you can `RemoveAt(0)` when over capacity) and redo in a `Stack<Entry>`. Keep `current` and `saved` state ids and a `nextId` counter. A merge replaces the top: `top with { New = newValue, At = at, After = ++nextId }`. In main, a helper `Edit("Name", "laptop-0", t0.AddSeconds(0.3))` reads the old value and picks the setter with a `switch`. The typing is `laptop-`, `laptop-0`, `laptop-01` 0.3 s apart; the three more edits are Name `x1` (+20 s), Owner `dee` (+21 s) and Name `x2` (+30 s).
@@solution
var asset = new Asset { Name = "laptop", Owner = "ana" };
var t0 = new DateTime(2026, 11, 3, 9, 0, 0);
var history = new History(TimeSpan.FromSeconds(1), capacity: 3);

Edit("Name", "laptop-", t0);
Edit("Name", "laptop-0", t0.AddSeconds(0.3));
Edit("Name", "laptop-01", t0.AddSeconds(0.6));
Show("typed laptop-01");
history.MarkSaved();
Show("saved");
Edit("Owner", "ben", t0.AddSeconds(5));
Show("owner=ben");
history.Undo();
Show("undo");
history.Undo();
Show("undo");
history.Redo();
Show("redo");
Edit("Owner", "cho", t0.AddSeconds(10));
Show("owner=cho");
Edit("Name", "x1", t0.AddSeconds(20));
Edit("Owner", "dee", t0.AddSeconds(21));
Edit("Name", "x2", t0.AddSeconds(30));
Show("three more edits");
while (history.UndoCount > 0) history.Undo();
Show("undo all");

void Edit(string property, string value, DateTime at)
{
    switch (property)
    {
        case "Name": history.Do(property, asset.Name, value, v => asset.Name = v, at); break;
        case "Owner": history.Do(property, asset.Owner, value, v => asset.Owner = v, at); break;
    }
}

void Show(string label) =>
    Console.WriteLine($"{label}: name={asset.Name} owner={asset.Owner} undo={history.UndoCount} redo={history.RedoCount} dirty={history.IsDirty}");

class Asset
{
    public string Name { get; set; } = "";
    public string Owner { get; set; } = "";
}

record Entry(string Property, string Old, string New, Action<string> Set, DateTime At, int Before, int After);

class History(TimeSpan mergeWindow, int capacity)
{
    private readonly List<Entry> undo = new();
    private readonly Stack<Entry> redo = new();
    private int current;
    private int saved;
    private int nextId;

    public int UndoCount => undo.Count;
    public int RedoCount => redo.Count;
    public bool IsDirty => current != saved;

    public void MarkSaved() => saved = current;

    public void Do(string property, string oldValue, string newValue, Action<string> set, DateTime at)
    {
        set(newValue);
        redo.Clear();
        int after = ++nextId;
        if (undo.Count > 0 && undo[^1] is var top && top.Property == property && at - top.At <= mergeWindow)
        {
            undo[^1] = top with { New = newValue, At = at, After = after };
        }
        else
        {
            undo.Add(new Entry(property, oldValue, newValue, set, at, current, after));
            if (undo.Count > capacity) undo.RemoveAt(0);
        }
        current = after;
    }

    public void Undo()
    {
        if (undo.Count == 0) return;
        var e = undo[^1];
        undo.RemoveAt(undo.Count - 1);
        e.Set(e.Old);
        redo.Push(e);
        current = e.Before;
    }

    public void Redo()
    {
        if (redo.Count == 0) return;
        var e = redo.Pop();
        e.Set(e.New);
        undo.Add(e);
        current = e.After;
    }
}
