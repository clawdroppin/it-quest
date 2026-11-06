@@lesson n7-blazor Blazor: Web UIs in C#
@@concepts objects, functions
@@text
**Blazor** builds interactive web UIs with C# components instead of JavaScript. It's popular for internal tools and admin portals in .NET shops, where the same team writes the API and the UI and can share model classes and validation between them.

## A component

```razor,norun
@* HostCard.razor *@
<div class="card @(Host.Online ? "ok" : "down")">
    <h3>@Title</h3>
    <p>@Host.Name: @(Host.Online ? "online" : "offline")</p>
    <button @onclick="Restart" disabled="@busy">Restart</button>
</div>

@code {
    [Parameter, EditorRequired] public HostDto Host { get; set; } = default!;
    [Parameter] public string Title { get; set; } = "Host";
    [Parameter] public EventCallback<string> OnRestarted { get; set; }   // notify the parent
    [Inject] public IHostApi Api { get; set; } = default!;              // DI works in components

    private bool busy;

    private async Task Restart()
    {
        busy = true;                     // UI re-renders automatically after event handlers
        await Api.RestartAsync(Host.Name);
        busy = false;
        await OnRestarted.InvokeAsync(Host.Name);
    }
}
```

Use it from another component: `<HostCard Host="h" Title="Primary" OnRestarted="Refresh" />`.

## How rendering works

- A component renders into a **render tree**; Blazor diffs it against the previous one and applies only the changes to the DOM.
- It re-renders after its event handlers run, when its parameters change, or when you call `StateHasChanged()` (needed after background work, e.g. a timer).
- `ShouldRender()` can skip re-renders. For simple-typed parameters (strings, numbers), Blazor skips re-rendering a child when the parent passes the same values again; complex objects always count as "maybe changed".
- Lifecycle methods: `OnInitialized(Async)` (load data once), `OnParametersSet(Async)`, `OnAfterRender(Async)` (JS interop), `Dispose` (unsubscribe!).

## Render modes (.NET 8+)

| Mode | Runs | Good for |
|---|---|---|
| Static SSR | server renders HTML, no interactivity | content pages, fast first load |
| Interactive Server | on the server; UI events over a SignalR connection | intranet apps, direct database access |
| Interactive WebAssembly | in the browser on .NET WebAssembly | offline-capable, scales without server state |
| Interactive Auto | Server first, WebAssembly once downloaded | best of both |

## Forms and validation

```razor,norun
<EditForm Model="model" OnValidSubmit="Save" FormName="new-host">
    <DataAnnotationsValidator />
    <InputText @bind-Value="model.Name" />
    <ValidationMessage For="() => model.Name" />
    <InputNumber @bind-Value="model.Cores" />
    <button type="submit">Save</button>
</EditForm>
```

An `EditContext` tracks which fields were modified and their validation messages; the same `[Required]`/`[Range]` attributes validate on the client and in your API.

## Beyond the browser

**Blazor Hybrid** hosts the same components in a native MAUI, WPF or WinForms window (a BlazorWebView), with full access to the device: one UI codebase for web and desktop. Component libraries such as MudBlazor, Radzen and Fluent UI Blazor provide data grids, dialogs and charts.

@@quiz
In Interactive Server mode, where does your component's C# code run?
* On the server; UI events and DOM updates travel over a SignalR connection
- In the browser via WebAssembly
- In a desktop window
- Nowhere, it's static
? Low download size, but each user holds a server connection.

@@quiz
When must you call `StateHasChanged()` yourself?
- After every event handler
* When state changes outside Blazor's own events, e.g. in a timer or background callback
- Never
- In OnInitialized only
? Event handlers trigger a render automatically.

@@quiz
How does a child component tell its parent something happened?
- By changing the parent's fields
* By invoking an `EventCallback` parameter
- With a static event
- With JavaScript
? `OnRestarted.InvokeAsync(name)`.

@@drill
Which file extension does a Blazor component use?
= .razor
= razor
? Markup and C# together.

@@flashcards
`.razor` :: Blazor component file
`[Parameter]` :: input from the parent
`EventCallback<T>` :: notify the parent
`StateHasChanged()` :: request a re-render
`ShouldRender()` :: skip re-renders
`OnInitializedAsync` :: load data once
`EditForm` / `EditContext` :: form state and validation
Interactive Server :: UI over SignalR
Interactive WebAssembly :: .NET in the browser
Blazor Hybrid :: components in native apps

@@exercise An EditContext for a sign-up form
Recreate how Blazor forms decide which validation messages to show:

- `EditContext` holds the model, a set of **modified** fields and a flag for whether the user has tried to submit
- `SetField(name, value)`: updates the model, marks the field modified, and re-validates
- validation rules: `Email` required and must contain `@` (`must contain @`); `Password` at least 12 characters (`must be at least 12 characters`); `Confirm` must equal `Password` (`must match Password`)
- `VisibleMessages()`: messages for fields that are modified, or for **all** fields after a submit attempt, in the order Email, Password, Confirm
- `Submit()`: marks the attempt; returns `submitted <email>` if there are no errors, else `blocked (<n> errors)`
@@starter
var form = new EditContext(new SignUp());
Console.WriteLine("TODO");

class SignUp
{
    public string Email { get; set; } = "";
    public string Password { get; set; } = "";
    public string Confirm { get; set; } = "";
}

class EditContext(SignUp model)
{
}
@@expected
email=ana: Email must contain @
password=short: Email must contain @ | Password must be at least 12 characters
submit: blocked (3 errors)
  Email must contain @ | Password must be at least 12 characters | Confirm must match Password
email fixed: Password must be at least 12 characters | Confirm must match Password
password fixed: Confirm must match Password
confirm fixed: none
submit: submitted ana@example.jp
@@hint
`Dictionary<string, string> Validate()` returns all current errors keyed by field (in rule order). `VisibleMessages()`: `Validate().Where(kv => submitted || modified.Contains(kv.Key)).Select(kv => $"{kv.Key} {kv.Value}")`, joined with ` | ` (or `none`). Set fields with a `switch` on the name.
@@solution
var form = new EditContext(new SignUp());
Show("email=ana", () => form.SetField("Email", "ana"));
Show("password=short", () => form.SetField("Password", "short"));
Console.WriteLine($"submit: {form.Submit()}");
Console.WriteLine($"  {form.VisibleMessages()}");
Show("email fixed", () => form.SetField("Email", "ana@example.jp"));
Show("password fixed", () => form.SetField("Password", "correct horse battery"));
Show("confirm fixed", () => form.SetField("Confirm", "correct horse battery"));
Console.WriteLine($"submit: {form.Submit()}");

void Show(string label, Action action)
{
    action();
    Console.WriteLine($"{label}: {form.VisibleMessages()}");
}

class SignUp
{
    public string Email { get; set; } = "";
    public string Password { get; set; } = "";
    public string Confirm { get; set; } = "";
}

class EditContext(SignUp model)
{
    private readonly HashSet<string> modified = new();
    private bool submitted;

    public void SetField(string name, string value)
    {
        switch (name)
        {
            case "Email": model.Email = value; break;
            case "Password": model.Password = value; break;
            case "Confirm": model.Confirm = value; break;
            default: throw new ArgumentException($"unknown field {name}");
        }
        modified.Add(name);
    }

    private Dictionary<string, string> Validate()
    {
        var errors = new Dictionary<string, string>();
        if (!model.Email.Contains('@')) errors["Email"] = "must contain @";
        if (model.Password.Length < 12) errors["Password"] = "must be at least 12 characters";
        if (model.Confirm != model.Password) errors["Confirm"] = "must match Password";
        return errors;
    }

    public string VisibleMessages()
    {
        var visible = Validate().Where(kv => submitted || modified.Contains(kv.Key)).Select(kv => $"{kv.Key} {kv.Value}").ToList();
        return visible.Count == 0 ? "none" : string.Join(" | ", visible);
    }

    public string Submit()
    {
        submitted = true;
        var errors = Validate();
        return errors.Count == 0 ? $"submitted {model.Email}" : $"blocked ({errors.Count} errors)";
    }
}

@@exercise Component re-render rules
Model Blazor's rendering rules in a tiny component base class:

- `abstract class Component` with `Renders` (count), `Html` (last output), abstract `BuildHtml()`, virtual `ShouldRender()` (true by default)
- `SetParameters(Dictionary<string, object?> ps)`: if every value is a string or number **equal** to the current one, do nothing (no render); otherwise store them, call `OnParametersSet()` and render
- `StateHasChanged()`: renders only if `ShouldRender()` returns true; the **first** render always happens
- `Counter : Component`: parameter `Title`; `Click()` increments `Count` and calls `StateHasChanged`; while `Paused` is true, `ShouldRender` returns false; `Resume()` clears it and calls `StateHasChanged`
- its HTML is `<h3>{Title}</h3><button>{Count}</button>`
@@starter
var c = new Counter();
Console.WriteLine("TODO");

abstract class Component
{
}
@@expected
set Title=Clicks: renders=1 <h3>Clicks</h3><button>0</button>
same Title again: renders=1 <h3>Clicks</h3><button>0</button>
two clicks: renders=3 <h3>Clicks</h3><button>2</button>
set Title=Visits: renders=4 <h3>Visits</h3><button>2</button>
paused click: renders=4 <h3>Visits</h3><button>2</button>
resume: renders=5 <h3>Visits</h3><button>3</button>
@@hint
Keep parameters in a `Dictionary<string, object?>`. "Unchanged" check: `ps.All(kv => kv.Value is string or int or double && parameters.TryGetValue(kv.Key, out var old) && Equals(old, kv.Value))`. Render: `Renders++; Html = BuildHtml();`. In `Counter`, read the title from the parameters in `OnParametersSet` (make it a virtual no-op in the base class).
@@solution
var c = new Counter();
Step("set Title=Clicks", () => c.SetParameters(new() { ["Title"] = "Clicks" }));
Step("same Title again", () => c.SetParameters(new() { ["Title"] = "Clicks" }));
Step("two clicks", () => { c.Click(); c.Click(); });
Step("set Title=Visits", () => c.SetParameters(new() { ["Title"] = "Visits" }));
Step("paused click", () => { c.Paused = true; c.Click(); });
Step("resume", c.Resume);

void Step(string label, Action action)
{
    action();
    Console.WriteLine($"{label}: renders={c.Renders} {c.Html}");
}

abstract class Component
{
    protected readonly Dictionary<string, object?> Parameters = new();
    public int Renders { get; private set; }
    public string Html { get; private set; } = "";

    protected abstract string BuildHtml();
    protected virtual bool ShouldRender() => true;
    protected virtual void OnParametersSet() { }

    public void SetParameters(Dictionary<string, object?> ps)
    {
        bool unchanged = Renders > 0 && ps.All(kv =>
            kv.Value is string or int or long or double or decimal
            && Parameters.TryGetValue(kv.Key, out var old) && Equals(old, kv.Value));
        if (unchanged) return;
        foreach (var (k, v) in ps) Parameters[k] = v;
        OnParametersSet();
        Render();
    }

    protected void StateHasChanged()
    {
        if (Renders == 0 || ShouldRender()) Render();
    }

    private void Render()
    {
        Renders++;
        Html = BuildHtml();
    }
}

class Counter : Component
{
    public string Title { get; private set; } = "";
    public int Count { get; private set; }
    public bool Paused { get; set; }

    protected override void OnParametersSet() => Title = Parameters.GetValueOrDefault("Title") as string ?? "";
    protected override bool ShouldRender() => !Paused;
    protected override string BuildHtml() => $"<h3>{Title}</h3><button>{Count}</button>";

    public void Click()
    {
        Count++;
        StateHasChanged();
    }

    public void Resume()
    {
        Paused = false;
        StateHasChanged();
    }
}
