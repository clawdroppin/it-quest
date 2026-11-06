@@lesson b9-coroutines 🎓 Beyond IT: Coroutines and Game AI State Machines
@@concepts functions, loops
@@text
> **Beyond IT scope.** Game programming techniques. Fun, and they sharpen your understanding of iterators and state, but not needed for IT jobs.

## Coroutines: sequences over time

Gameplay is full of "do this, wait, then do that": a door that unlocks, waits a second and slides open; a spawner that sends waves. Unity builds these on C# **iterators**: a method returning `IEnumerator` pauses at every `yield return` and the engine resumes it later.

```csharp,norun
IEnumerator OpenDoor()
{
    alarm.Play();
    yield return new WaitForSeconds(1f);   // resume after 1 second
    yield return StartCoroutine(Slide());  // wait for another coroutine
    yield return null;                     // resume next frame
    Debug.Log("door open");
}

StartCoroutine(OpenDoor());
```

The compiler turns the method into a state machine class with `MoveNext()` and `Current`, exactly like any `yield return` iterator. The engine checks `Current` to know what to wait for. Coroutines run on the main thread; they're not threads. In modern Unity you can also use `async`/`await` with `Awaitable`, and in Godot `await ToSignal(timer, "timeout")`.

## Finite state machines for AI

Enemies are usually **finite state machines**: one current state (Patrol, Chase, Attack, Flee), each with enter/update/exit logic and rules for switching.

```text,norun
            sees player              in range
  Patrol ─────────────▶ Chase ─────────────▶ Attack
     ▲                    ▲ ◀───────────────── │
     │  healed            │     out of range    │
     └──── Flee ◀─────────┴──── low health ─────┘
```

Keep the transition rules in one place, log transitions while debugging, and give higher-priority rules (low health → Flee) precedence over the rest. Bigger games move on to **behaviour trees** and **utility AI**, but FSMs are the foundation and are also used for UI flows, network protocols and parsers.

@@quiz
What does `yield return null` mean in a Unity coroutine?
* Resume this coroutine on the next frame
- Stop the coroutine
- Return null to the caller
- Wait one second
? Other yield values wait for time, other coroutines or conditions.

@@quiz
Do coroutines run on a separate thread?
- Yes, each on its own thread
* No, the engine resumes them on the main thread between frames
- Only WaitForSeconds does
- Only in builds
? They're iterators driven by the game loop.

@@quiz
In a state machine for an enemy, what triggers a change from Patrol to Chase?
* A transition condition, such as the player coming within range
- A random timer only
- The frame rate
- Restarting the game
? Each state checks its transitions every update.

@@drill
Which keyword pauses a Unity coroutine until the next step?
= yield return
= yield
? yield return new WaitForSeconds(1);

@@flashcards
Coroutine :: iterator resumed by the engine
`yield return null` :: next frame
`WaitForSeconds` :: resume after a delay
`StopCoroutine` :: cancel a coroutine
FSM :: one current state plus transition rules
Enter / Update / Exit :: state hooks
Behaviour tree :: hierarchical AI decisions

@@exercise A coroutine scheduler
Write the engine side of coroutines:

- `Scheduler` with `Time`, `StartCoroutine(IEnumerator)` returning a `Coroutine` handle, `StopCoroutine(handle)`, `Tick(dt)` and `Count`
- starting a coroutine runs it immediately up to its first `yield`
- a coroutine can yield `null` (next tick), `new WaitForSeconds(s)`, or another `IEnumerator` (run it now, nested, and continue the parent when it finishes, in the same tick)
- `Tick` advances `Time` and resumes every coroutine whose wait has elapsed, in start order

Run the door sequence and a blinking light together at 0.25 s per tick; stop the light as soon as the door is open.
@@starter
using System.Collections;

var sched = new Scheduler();
void Log(string s) => Console.WriteLine($"{sched.Time:F2} {s}");
Console.WriteLine("TODO");

record WaitForSeconds(double Seconds);
@@expected
0.00 alarm on
0.00 blink
0.50 blink
1.00 door unlocked
1.00 sliding 1/3
1.00 blink
1.25 sliding 2/3
1.50 sliding 3/3
1.50 blink
1.75 door open
stopped blink at 1.75, coroutines left: 0
@@requires
IEnumerator
yield return
@@hint
A `Coroutine` holds a `Stack<IEnumerator>`, `WaitUntil` and `Done`. `Step()`: loop { if the top's `MoveNext()` is false, pop it (done when the stack is empty, otherwise continue with the parent); else switch on `Current`: a `WaitForSeconds` sets `WaitUntil = Time + s` and returns; an `IEnumerator` is pushed and the loop continues; `null` sets `WaitUntil = Time` and returns }.
@@solution
using System.Collections;

var sched = new Scheduler();
void Log(string s) => Console.WriteLine($"{sched.Time:F2} {s}");

var door = sched.StartCoroutine(OpenDoor());
var blink = sched.StartCoroutine(Blink());
while (!door.Done) sched.Tick(0.25);
sched.StopCoroutine(blink);
Console.WriteLine($"stopped blink at {sched.Time:F2}, coroutines left: {sched.Count}");

IEnumerator OpenDoor()
{
    Log("alarm on");
    yield return new WaitForSeconds(1);
    Log("door unlocked");
    yield return Slide(3);
    Log("door open");
}

IEnumerator Slide(int steps)
{
    for (int i = 1; i <= steps; i++)
    {
        Log($"sliding {i}/{steps}");
        yield return null;
    }
}

IEnumerator Blink()
{
    while (true)
    {
        Log("blink");
        yield return new WaitForSeconds(0.5);
    }
}

record WaitForSeconds(double Seconds);

class Coroutine(IEnumerator routine)
{
    public Stack<IEnumerator> Stack { get; } = new([routine]);
    public double WaitUntil { get; set; }
    public bool Done { get; set; }
}

class Scheduler
{
    private readonly List<Coroutine> running = new();
    public double Time { get; private set; }
    public int Count => running.Count;

    public Coroutine StartCoroutine(IEnumerator routine)
    {
        var c = new Coroutine(routine);
        running.Add(c);
        Step(c);
        return c;
    }

    public void StopCoroutine(Coroutine c)
    {
        c.Done = true;
        running.Remove(c);
    }

    public void Tick(double dt)
    {
        Time += dt;
        foreach (var c in running.ToList())
            if (!c.Done && Time >= c.WaitUntil) Step(c);
        running.RemoveAll(c => c.Done);
    }

    private void Step(Coroutine c)
    {
        while (true)
        {
            var top = c.Stack.Peek();
            if (!top.MoveNext())
            {
                c.Stack.Pop();
                if (c.Stack.Count == 0)
                {
                    c.Done = true;
                    return;
                }
                continue;
            }
            switch (top.Current)
            {
                case WaitForSeconds w:
                    c.WaitUntil = Time + w.Seconds;
                    return;
                case IEnumerator nested:
                    c.Stack.Push(nested);
                    continue;
                default:
                    c.WaitUntil = Time;
                    return;
            }
        }
    }
}

@@exercise An enemy state machine
Write an enemy AI as a finite state machine:

- states `Patrol`, `Chase`, `Attack`, `Flee`, each a class deriving from `abstract class State` with `Name`, `virtual State? Next(Sense s)` and `virtual void Update(Enemy e, Sense s)`
- `record Sense(int Tick, double Distance, int Health)`
- rules, checked each tick before updating: any state except Flee: health < 30 → Flee. Patrol: distance < 10 → Chase. Chase: distance < 2 → Attack, distance > 15 → Patrol. Attack: distance ≥ 2 → Chase. Flee: health ≥ 60 → Patrol
- log each transition with the sense data; `Attack.Update` logs `attack`; count the ticks spent in each state (after transitions)
@@starter
Sense[] senses =
[
    new(1, 20, 100), new(2, 8, 100), new(3, 5, 100), new(4, 1.5, 100), new(5, 1, 80),
    new(6, 3, 80), new(7, 1, 25), new(8, 12, 40), new(9, 30, 65),
];
Console.WriteLine("TODO");

record Sense(int Tick, double Distance, int Health);
@@expected
t=2: Patrol -> Chase (player at 8, health 100)
t=4: Chase -> Attack (player at 1.5, health 100)
t=4: attack
t=5: attack
t=6: Attack -> Chase (player at 3, health 80)
t=7: Chase -> Flee (player at 1, health 25)
t=9: Flee -> Patrol (player at 30, health 65)
time in states: Patrol 2, Chase 3, Attack 2, Flee 2
@@hint
Put the shared low-health rule in the `Enemy.Tick` method: `var next = s.Health < 30 && state is not Flee ? new Flee() : state.Next(s);`. Count ticks in a `Dictionary<string, int>` and print it in the order Patrol, Chase, Attack, Flee.
@@solution
Sense[] senses =
[
    new(1, 20, 100), new(2, 8, 100), new(3, 5, 100), new(4, 1.5, 100), new(5, 1, 80),
    new(6, 3, 80), new(7, 1, 25), new(8, 12, 40), new(9, 30, 65),
];
var enemy = new Enemy();
foreach (var s in senses) enemy.Tick(s);
string[] order = ["Patrol", "Chase", "Attack", "Flee"];
Console.WriteLine($"time in states: {string.Join(", ", order.Select(n => $"{n} {enemy.Ticks.GetValueOrDefault(n)}"))}");

record Sense(int Tick, double Distance, int Health);

abstract class State
{
    public string Name => GetType().Name;
    public virtual State? Next(Sense s) => null;
    public virtual void Update(Enemy e, Sense s) { }
}

class Patrol : State
{
    public override State? Next(Sense s) => s.Distance < 10 ? new Chase() : null;
}

class Chase : State
{
    public override State? Next(Sense s) => s.Distance < 2 ? new Attack() : s.Distance > 15 ? new Patrol() : null;
}

class Attack : State
{
    public override State? Next(Sense s) => s.Distance >= 2 ? new Chase() : null;
    public override void Update(Enemy e, Sense s) => Console.WriteLine($"t={s.Tick}: attack");
}

class Flee : State
{
    public override State? Next(Sense s) => s.Health >= 60 ? new Patrol() : null;
}

class Enemy
{
    private State state = new Patrol();
    public Dictionary<string, int> Ticks { get; } = new();

    public void Tick(Sense s)
    {
        var next = s.Health < 30 && state is not Flee ? new Flee() : state.Next(s);
        if (next is not null)
        {
            Console.WriteLine($"t={s.Tick}: {state.Name} -> {next.Name} (player at {s.Distance}, health {s.Health})");
            state = next;
        }
        state.Update(this, s);
        Ticks[state.Name] = Ticks.GetValueOrDefault(state.Name) + 1;
    }
}
