@@lesson g8-collections Live Collections and the UI Thread
@@concepts lists, concurrency
@@text
Most business apps are lists: tickets, users, devices, invoices. Two things make lists in XAML apps work well: **observable collections with views** and **respecting the UI thread**.

## ObservableCollection and collection views

`ObservableCollection<T>` raises `CollectionChanged` when items are added, removed or moved, so a bound `ListView`/`DataGrid` updates itself. A **collection view** sits between the collection and the control and adds filtering, sorting and grouping without changing the underlying data:

```csharp,norun
// WPF
var view = CollectionViewSource.GetDefaultView(Tickets);
view.Filter = o => ((Ticket)o).Open;
view.SortDescriptions.Add(new SortDescription(nameof(Ticket.Id), ListSortDirection.Ascending));
view.GroupDescriptions.Add(new PropertyGroupDescription(nameof(Ticket.Priority)));
((ICollectionViewLiveShaping)view).IsLiveFiltering = true;   // re-filter when an item's property changes
```

Avalonia uses `DataGridCollectionView` or the DynamicData library; MAUI's `CollectionView` binds to a list you filter in the ViewModel. With thousands of rows, keep **UI virtualization** on (only visible rows get controls).

## The UI thread

UI objects belong to the thread that created them. Touching them from another thread throws (WPF: "The calling thread cannot access this object because a different thread owns it"). The rules:

- do slow work off the UI thread: `await` I/O, or `Task.Run` for CPU work
- come back to the UI thread to update: `await` does this automatically, because it captures the **SynchronizationContext**
- report progress with `IProgress<T>`: `Progress<T>` captures the context when it's created and runs its callback there
- when you really must, marshal explicitly: `Dispatcher.InvokeAsync` (WPF), `Dispatcher.UIThread.Post` (Avalonia), `MainThread.BeginInvokeOnMainThread` (MAUI)

```csharp,norun
private async Task ExportAsync()
{
    var progress = new Progress<int>(p => ExportPercent = p);        // created on the UI thread
    await Task.Run(() => exporter.Export(Tickets.ToList(), progress)); // runs on the thread pool
    Status = "Export finished";                                        // back on the UI thread
}
```

Never block the UI thread with `.Result` or `.Wait()` on a task that needs to come back to it: that's the classic UI deadlock.

@@quiz
What does a collection view add on top of an ObservableCollection?
* Filtering, sorting and grouping without changing the underlying collection
- Thread safety
- Persistence
- Undo
? Several views can show the same data differently.

@@quiz
Why does `Progress<T>` run its callback on the UI thread?
- It always uses thread 1
* It captures the SynchronizationContext when it's created and posts callbacks to it
- It uses a lock
- It doesn't; you must use the Dispatcher
? Create it on the UI thread.

@@quiz
What causes the classic UI deadlock?
- Too many threads
* Blocking the UI thread with `.Result` on a task whose continuation needs the UI thread
- Using async void
- ObservableCollection
? Use await all the way.

@@flashcards
`ObservableCollection<T>` :: raises CollectionChanged
Collection view :: filter, sort, group a source
Live filtering :: re-filter on item property changes
UI virtualization :: controls only for visible rows
`SynchronizationContext` :: where continuations run
`Progress<T>` :: callbacks on the captured context
`Dispatcher.InvokeAsync` :: marshal to the WPF UI thread
`.Result` on UI thread :: deadlock risk

@@exercise A live collection view
Write `CollectionView<T>` over an `ObservableCollection<T>` whose items implement `INotifyPropertyChanged`:

- constructor `(source, Func<T, bool>? filter, Func<T, object> groupKey, Func<T, object> sortKey)`; it refreshes once
- a settable `Filter` property that refreshes
- refreshes automatically when the collection changes and when any item raises `PropertyChanged` (live shaping): subscribe to added items, unsubscribe from removed ones
- `Refreshes` counts refreshes; `Describe()` prints the groups ordered by key, items ordered by sort key: `High: T-101 T-104 | Low: T-102`
- tickets have `Id`, `Title`, `Priority` (enum `High, Medium, Low`) and an observable `Open`
@@starter
using System.Collections.ObjectModel;
using System.Collections.Specialized;
using System.ComponentModel;

var tickets = new ObservableCollection<Ticket>
{
    new("T-101", "VPN drops", Priority.High, true),
    new("T-102", "printer jam", Priority.Low, true),
    new("T-103", "new laptop", Priority.Medium, false),
    new("T-104", "disk full on db01", Priority.High, true),
};
Console.WriteLine("TODO");

enum Priority { High, Medium, Low }
@@expected
initial: High: T-101 T-104 | Low: T-102
added T-105: High: T-101 T-104 | Medium: T-105 | Low: T-102
closed T-101: High: T-104 | Medium: T-105 | Low: T-102
removed T-102: High: T-104 | Medium: T-105
show all: High: T-101 T-104 | Medium: T-103 T-105
refreshes: 5
@@requires
CollectionChanged
PropertyChanged
@@hint
In the `CollectionChanged` handler, use `e.NewItems` and `e.OldItems` (both may be null) with `.OfType<T>()` to subscribe and unsubscribe `OnItemChanged`. `Refresh()` stores `source.Where(filter ?? (_ => true)).OrderBy(sortKey).GroupBy(groupKey).OrderBy(g => g.Key).ToList()`; `object` keys sort fine because enums and strings are `IComparable`.
@@solution
using System.Collections.ObjectModel;
using System.Collections.Specialized;
using System.ComponentModel;

var tickets = new ObservableCollection<Ticket>
{
    new("T-101", "VPN drops", Priority.High, true),
    new("T-102", "printer jam", Priority.Low, true),
    new("T-103", "new laptop", Priority.Medium, false),
    new("T-104", "disk full on db01", Priority.High, true),
};
var view = new CollectionView<Ticket>(tickets, t => t.Open, t => t.Priority, t => t.Id);

Console.WriteLine($"initial: {view.Describe()}");
tickets.Add(new("T-105", "password reset", Priority.Medium, true));
Console.WriteLine($"added T-105: {view.Describe()}");
tickets[0].Open = false;
Console.WriteLine($"closed T-101: {view.Describe()}");
tickets.Remove(tickets.Single(t => t.Id == "T-102"));
Console.WriteLine($"removed T-102: {view.Describe()}");
view.Filter = null;
Console.WriteLine($"show all: {view.Describe()}");
Console.WriteLine($"refreshes: {view.Refreshes}");

enum Priority { High, Medium, Low }

class Ticket(string id, string title, Priority priority, bool open) : INotifyPropertyChanged
{
    public event PropertyChangedEventHandler? PropertyChanged;
    public string Id { get; } = id;
    public string Title { get; } = title;
    public Priority Priority { get; } = priority;

    public bool Open
    {
        get => open;
        set
        {
            if (open == value) return;
            open = value;
            PropertyChanged?.Invoke(this, new PropertyChangedEventArgs(nameof(Open)));
        }
    }
}

class CollectionView<T> where T : INotifyPropertyChanged
{
    private readonly ObservableCollection<T> source;
    private readonly Func<T, object> groupKey;
    private readonly Func<T, object> sortKey;
    private Func<T, bool>? filter;
    private List<IGrouping<object, T>> groups = new();

    public CollectionView(ObservableCollection<T> source, Func<T, bool>? filter, Func<T, object> groupKey, Func<T, object> sortKey)
    {
        this.source = source;
        this.filter = filter;
        this.groupKey = groupKey;
        this.sortKey = sortKey;
        foreach (var item in source) item.PropertyChanged += OnItemChanged;
        source.CollectionChanged += OnCollectionChanged;
        Refresh();
    }

    public int Refreshes { get; private set; }

    public Func<T, bool>? Filter
    {
        get => filter;
        set
        {
            filter = value;
            Refresh();
        }
    }

    private void OnCollectionChanged(object? sender, NotifyCollectionChangedEventArgs e)
    {
        foreach (var item in e.OldItems?.OfType<T>() ?? []) item.PropertyChanged -= OnItemChanged;
        foreach (var item in e.NewItems?.OfType<T>() ?? []) item.PropertyChanged += OnItemChanged;
        Refresh();
    }

    private void OnItemChanged(object? sender, PropertyChangedEventArgs e) => Refresh();

    private void Refresh()
    {
        Refreshes++;
        groups = source.Where(filter ?? (_ => true)).OrderBy(sortKey).GroupBy(groupKey).OrderBy(g => g.Key).ToList();
    }

    public string Describe() =>
        string.Join(" | ", groups.Select(g => $"{g.Key}: {string.Join(" ", g.Select(t => sortKey(t)))}"));
}

@@exercise A UI thread, progress and cross-thread access
Recreate a UI framework's threading model:

- `UiContext : SynchronizationContext` with a queue: `Post` enqueues; `Run(Func<Task> main)` installs itself as the current context, runs `main`, and pumps the queue on the calling thread until `main`'s task completes
- `Label` remembers the thread that created it; setting `Text` from another thread throws `InvalidOperationException("The calling thread cannot access this object because a different thread owns it.")`
- inside `Run`: create a label and a `Progress<int>` that sets the label and prints whether it ran on the UI thread; in `Task.Run`, first try to set the label directly (catch and print the message), then report 25, 50, 75, 100; after awaiting, print where the continuation ran
@@starter
using System.Collections.Concurrent;

Console.WriteLine("TODO");
@@expected
from background: The calling thread cannot access this object because a different thread owns it.
label: 25% (UI thread: True)
label: 50% (UI thread: True)
label: 75% (UI thread: True)
label: 100% (UI thread: True)
done on UI thread: True, label = 100%
@@requires
SynchronizationContext
IProgress<int>
@@hint
Use a `BlockingCollection<(SendOrPostCallback, object?)>`. In `Run`: `SetSynchronizationContext(this)`, `var task = main(); task.ContinueWith(_ => queue.CompleteAdding(), TaskScheduler.Default); foreach (var (d, s) in queue.GetConsumingEnumerable()) d(s); task.GetAwaiter().GetResult();`, and restore the previous context in `finally`. Compare `Environment.CurrentManagedThreadId` with the id captured at the start of `main`. Call `Report` through `IProgress<int>`, because `Progress<T>.Report` is an explicit interface implementation.
@@solution
using System.Collections.Concurrent;

new UiContext().Run(async () =>
{
    int uiThread = Environment.CurrentManagedThreadId;
    var label = new Label();
    IProgress<int> progress = new Progress<int>(p =>
    {
        label.Text = $"{p}%";
        Console.WriteLine($"label: {label.Text} (UI thread: {Environment.CurrentManagedThreadId == uiThread})");
    });

    await Task.Run(() =>
    {
        try
        {
            label.Text = "from worker";
        }
        catch (InvalidOperationException e)
        {
            Console.WriteLine($"from background: {e.Message}");
        }
        for (int p = 25; p <= 100; p += 25) progress.Report(p);
    });

    Console.WriteLine($"done on UI thread: {Environment.CurrentManagedThreadId == uiThread}, label = {label.Text}");
});

class UiContext : SynchronizationContext
{
    private readonly BlockingCollection<(SendOrPostCallback Callback, object? State)> queue = new();

    public override void Post(SendOrPostCallback d, object? state) => queue.Add((d, state));

    public void Run(Func<Task> main)
    {
        var previous = Current;
        SetSynchronizationContext(this);
        try
        {
            var task = main();
            task.ContinueWith(_ => queue.CompleteAdding(), TaskScheduler.Default);
            foreach (var (callback, state) in queue.GetConsumingEnumerable()) callback(state);
            task.GetAwaiter().GetResult();
        }
        finally
        {
            SetSynchronizationContext(previous);
        }
    }
}

class Label
{
    private readonly int owner = Environment.CurrentManagedThreadId;
    private string text = "";

    public string Text
    {
        get => text;
        set
        {
            if (Environment.CurrentManagedThreadId != owner)
                throw new InvalidOperationException("The calling thread cannot access this object because a different thread owns it.");
            text = value;
        }
    }
}
