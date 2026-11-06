@@lesson b9-unity 🎓 Beyond IT: Game Development with Unity
@@concepts objects, loops
@@text
> **Beyond IT scope.** This module goes past IT and business software into game development. C# is the language of Unity and is supported by Godot, so it's a fun direction to take your skills, but it's not needed for IT jobs. Skip it freely if that's your goal.

**Unity** is one of the most used game engines, for mobile, PC, console and VR games, and also for simulations, architecture visualisation and digital twins. You write gameplay in C# scripts attached to objects in a scene.

## GameObjects and components

A scene is a tree of **GameObjects**. A GameObject is just a container with a `Transform` (position, rotation, scale); behaviour comes from the **components** you attach: a `Rigidbody` for physics, a `Collider`, a `MeshRenderer`, and your own scripts, which derive from `MonoBehaviour`.

```csharp,norun
using UnityEngine;

public class Mover : MonoBehaviour
{
    [SerializeField] private float speed = 4f;      // editable in the Inspector
    private Rigidbody body;

    void Awake() => body = GetComponent<Rigidbody>();   // cache references

    void Update()                                    // once per rendered frame: input, visuals
    {
        var input = new Vector3(Input.GetAxis("Horizontal"), 0, Input.GetAxis("Vertical"));
        transform.Translate(input * speed * Time.deltaTime);   // frame-rate independent
    }

    void FixedUpdate()                               // fixed rate (50 Hz by default): physics
    {
        body.AddForce(Vector3.up * 2f);
    }

    void OnCollisionEnter(Collision other) => Debug.Log($"hit {other.gameObject.name}");
}
```

## The lifecycle

| Method | When |
|---|---|
| `Awake` | right after the object is created, even if disabled; set up your own references |
| `OnEnable` | each time the component is enabled |
| `Start` | before its **first** `Update`, after every object's `Awake`; safe to talk to other objects |
| `FixedUpdate` | 0 or more times per frame, at a fixed timestep, for physics |
| `Update` | once per frame |
| `LateUpdate` | after every `Update`; cameras that follow the player |
| `OnDestroy` | when destroyed; `Destroy(obj)` takes effect at the end of the frame |

An object spawned with `Instantiate` during a frame gets `Awake` immediately but its first `Update` only in the **next** frame.

## Data and assets

- **Prefabs** are reusable GameObject templates (an enemy, a bullet), instantiated at runtime.
- **ScriptableObjects** are data assets (weapon stats, level configs) designed in the editor and shared by many objects.
- Avoid `GameObject.Find` and `GetComponent` in `Update` (slow); cache references in `Awake`.
- Spawning and destroying many bullets causes GC spikes: use **object pools** (`UnityEngine.Pool.ObjectPool<T>`).

## Fixed timestep

Rendering runs as fast as it can (16 ms one frame, 40 ms the next), but physics needs a constant step to be stable. The engine keeps an **accumulator**: add each frame's time, then run `FixedUpdate` once per whole step it contains. The leftover fraction (alpha) is used to interpolate positions for smooth rendering. A cap on steps per frame prevents the "spiral of death" when the game falls behind.

@@quiz
Which method should cache `GetComponent` results?
* Awake
- Update
- LateUpdate
- OnDestroy
? It runs once, right after creation.

@@quiz
Why multiply movement by `Time.deltaTime` in Update?
- To slow the game down
* So movement speed is the same at any frame rate
- Physics requires it
- To avoid garbage collection
? Speed becomes units per second instead of units per frame.

@@quiz
When does an object spawned during a frame get its first Update?
- Immediately
* In the next frame, after its Start runs
- Never
- After the next FixedUpdate only
? Awake runs immediately, Start and Update next frame.

@@drill
Which Unity method runs at a fixed timestep, for physics?
= FixedUpdate
= FixedUpdate()
? Update runs once per rendered frame.

@@flashcards
GameObject :: container with a Transform
Component :: behaviour attached to a GameObject
`MonoBehaviour` :: base class for Unity scripts
`Awake` / `Start` :: setup before first frame
`Update` :: every frame
`FixedUpdate` :: fixed physics step
`LateUpdate` :: after all Updates (cameras)
`Time.deltaTime` :: seconds since last frame
Prefab :: reusable object template
ScriptableObject :: data asset

@@exercise A mini engine lifecycle
Write a tiny engine with Unity's lifecycle rules:

- `abstract class Behaviour(string name)` with virtual `Awake`, `Start`, `Update(float dt)`, `LateUpdate`, `OnDestroy`, and `Log(message)` printing `[<frame>] <name>: <message>`
- `Engine.Instantiate(b)`: calls `Awake` immediately and queues the object to start
- `Engine.Tick(dt)`: increments `Frame`, calls `Start` on queued objects (making them active), then `Update` on all active objects, then `LateUpdate` on all, then destroys the objects requested with `Destroy` (calling `OnDestroy`)
- a `Spawner` that instantiates a `Bullet` on its **second** update; the bullet moves at 4 units/s, logs its position each update, and destroys itself at x ≥ 4 (in `LateUpdate` it logs only when dying)

Run 5 frames of 0.5 s, then print the alive objects.
@@starter
var engine = new Engine();
Console.WriteLine("TODO");

abstract class Behaviour(string name)
{
    public string Name { get; } = name;
    public Engine Engine { get; set; } = null!;
    protected void Log(string message) => Console.WriteLine($"[{Engine.Frame}] {Name}: {message}");
    public virtual void Awake() { }
    public virtual void Start() { }
    public virtual void Update(float dt) { }
    public virtual void LateUpdate() { }
    public virtual void OnDestroy() { }
}
@@expected
[0] spawner: Awake
[1] spawner: Start
[2] bullet: Awake
[3] bullet: Start
[3] bullet: x=2
[4] bullet: x=4
[4] bullet: destroy requested
[4] bullet: LateUpdate (alive until the end of the frame)
[4] bullet: OnDestroy
alive after 5 frames: spawner
@@hint
Keep three lists in `Engine`: `active`, `pendingStart`, `pendingDestroy`. Iterate over copies (`active.ToList()`) because behaviours can spawn or destroy during the loop. The spawner counts its updates; spawning in update 2 calls `Awake` in frame 2, and the bullet's `Start` and first `Update` come in frame 3.
@@solution
var engine = new Engine();
engine.Instantiate(new Spawner());
for (int i = 0; i < 5; i++) engine.Tick(0.5f);
Console.WriteLine($"alive after 5 frames: {string.Join(", ", engine.Alive)}");

abstract class Behaviour(string name)
{
    public string Name { get; } = name;
    public Engine Engine { get; set; } = null!;
    protected void Log(string message) => Console.WriteLine($"[{Engine.Frame}] {Name}: {message}");
    public virtual void Awake() { }
    public virtual void Start() { }
    public virtual void Update(float dt) { }
    public virtual void LateUpdate() { }
    public virtual void OnDestroy() { }
}

class Engine
{
    private readonly List<Behaviour> active = new();
    private readonly List<Behaviour> pendingStart = new();
    private readonly List<Behaviour> pendingDestroy = new();

    public int Frame { get; private set; }
    public IEnumerable<string> Alive => active.Concat(pendingStart).Select(b => b.Name);

    public T Instantiate<T>(T b) where T : Behaviour
    {
        b.Engine = this;
        b.Awake();
        pendingStart.Add(b);
        return b;
    }

    public void Destroy(Behaviour b)
    {
        if (!pendingDestroy.Contains(b)) pendingDestroy.Add(b);
    }

    public void Tick(float dt)
    {
        Frame++;
        var starting = pendingStart.ToList();
        pendingStart.Clear();
        foreach (var b in starting)
        {
            b.Start();
            active.Add(b);
        }
        foreach (var b in active.ToList()) b.Update(dt);
        foreach (var b in active.ToList()) b.LateUpdate();
        foreach (var b in pendingDestroy)
        {
            active.Remove(b);
            b.OnDestroy();
        }
        pendingDestroy.Clear();
    }
}

class Spawner() : Behaviour("spawner")
{
    private int updates;
    public override void Awake() => Log("Awake");
    public override void Start() => Log("Start");

    public override void Update(float dt)
    {
        if (++updates == 2) Engine.Instantiate(new Bullet());
    }
}

class Bullet() : Behaviour("bullet")
{
    private float x;
    private bool dying;
    public override void Awake() => Log("Awake");
    public override void Start() => Log("Start");

    public override void Update(float dt)
    {
        x += 4 * dt;
        Log($"x={x}");
        if (x >= 4)
        {
            dying = true;
            Log("destroy requested");
            Engine.Destroy(this);
        }
    }

    public override void LateUpdate()
    {
        if (dying) Log("LateUpdate (alive until the end of the frame)");
    }

    public override void OnDestroy() => Log("OnDestroy");
}

@@exercise A fixed timestep with an accumulator
Simulate a falling ball the way game engines do:

- physics runs in `FixedUpdate` at a fixed step of 20 ms: semi-implicit Euler (`v += g * dt; y += v * dt`) with `g = -9.81`, starting at `y = 10`
- render frames arrive with these durations in ms: 16, 33, 50, 100, 250, 16
- keep an integer accumulator in ms; run one fixed step per 20 ms it holds, at most 5 per frame. If the cap is hit, drop the excess (`acc %= 20`) and mark the frame `(slowed down)`
- per frame print the steps run, `y` (3 decimals) and `alpha = acc / 20` (2 decimals); finally the simulated time and total frame time
@@starter
int[] frames = [16, 33, 50, 100, 250, 16];
Console.WriteLine("TODO");
@@expected
frame 1: dt=16ms steps=0 y=10.000 alpha=0.80
frame 2: dt=33ms steps=2 y=9.988 alpha=0.45
frame 3: dt=50ms steps=2 y=9.961 alpha=0.95
frame 4: dt=100ms steps=5 y=9.823 alpha=0.95
frame 5: dt=250ms steps=5 y=9.588 alpha=0.45 (slowed down)
frame 6: dt=16ms steps=1 y=9.529 alpha=0.25
simulated 0.30 s in 465 ms of frames
@@requires
FixedUpdate
@@hint
`while (acc >= Step && steps < MaxSteps) { ball.FixedUpdate(Step / 1000.0); acc -= Step; steps++; }`, then `bool slowed = acc >= Step; if (slowed) acc %= Step;`. Integer milliseconds avoid floating-point surprises in the accumulator. Count total steps for the simulated time.
@@solution
int[] frames = [16, 33, 50, 100, 250, 16];
const int Step = 20, MaxSteps = 5;
var ball = new Ball();
int acc = 0, totalSteps = 0;

for (int i = 0; i < frames.Length; i++)
{
    acc += frames[i];
    int steps = 0;
    while (acc >= Step && steps < MaxSteps)
    {
        ball.FixedUpdate(Step / 1000.0);
        acc -= Step;
        steps++;
    }
    bool slowed = acc >= Step;
    if (slowed) acc %= Step;
    totalSteps += steps;
    Console.WriteLine($"frame {i + 1}: dt={frames[i]}ms steps={steps} y={ball.Y:F3} alpha={acc / (double)Step:F2}{(slowed ? " (slowed down)" : "")}");
}
Console.WriteLine($"simulated {totalSteps * Step / 1000.0:F2} s in {frames.Sum()} ms of frames");

class Ball
{
    private const double Gravity = -9.81;
    private double velocity;
    public double Y { get; private set; } = 10;

    public void FixedUpdate(double dt)
    {
        velocity += Gravity * dt;
        Y += velocity * dt;
    }
}
