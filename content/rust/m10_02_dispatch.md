@@lesson m10-dispatch Static and Dynamic Dispatch: impl Trait, dyn Trait and Function Values
@@text
There are two ways Rust calls a trait method on a value whose exact type your code doesn't name. Knowing the difference explains error messages, binary sizes, performance, and how to build plugin systems.

## Static dispatch: generics and `impl Trait`

```rust,norun
fn total_area<S: Shape>(shapes: &[S]) -> f64 { ... }
fn print_it(x: impl Display) { ... }          // same thing, shorter
```

The compiler generates a **separate copy** of the function for each concrete type you use (`total_area::<Circle>`, `total_area::<Square>`). This is called **monomorphization**. Every call is a direct call that can be inlined: zero overhead. The cost: more compiled code, and a `&[S]` holds only *one* kind of shape.

`impl Trait` in **return position** means "some single concrete type I won't name":

```rust,norun
fn evens(limit: u32) -> impl Iterator<Item = u32> {
    (0..limit).filter(|n| n % 2 == 0)
}
```

That's how you return closures and iterator chains, whose real types are long or unnameable.

## Dynamic dispatch: `dyn Trait`

```rust,norun
fn total_area(shapes: &[Box<dyn Shape>]) -> f64 { ... }
```

A `Box<dyn Shape>` (or `&dyn Shape`, `Rc<dyn Shape>`) is a **fat pointer**: a pointer to the data plus a pointer to a **vtable**, a table of the methods for that concrete type. Calls go through the table at runtime. Benefits: one compiled function, and a list can mix circles and squares. Cost: an indirect call (a few nanoseconds) and no inlining.

| | Generics / `impl Trait` | `dyn Trait` |
|---|---|---|
| Decided | at compile time | at runtime |
| Mixed types in one collection | no | yes |
| Speed | fastest, inlinable | slightly slower |
| Binary size | grows per type | one copy |
| Typical use | libraries, hot loops, iterators | plugins, UI widgets, heterogeneous lists, reducing compile times |

## Dyn compatibility (object safety)

Not every trait can be a `dyn Trait`. The vtable needs one entry per method, so methods that are generic (`fn map<T>(...)`) or return `Self` can't go in it. A trait is **dyn compatible** when its methods take `self` by reference (or `Box<Self>`) and are not generic. Workarounds: move generic methods to another trait, or mark them `where Self: Sized` so they're excluded from the vtable.

## Downcasting with `Any`

Occasionally you need to get the concrete type back from a `dyn` value (an event system, a plugin that exposes extra features). `std::any::Any` allows `value.downcast_ref::<Concrete>()`, returning `Option<&Concrete>`. Treat it as a last resort; an enum is usually clearer.

## Enums versus trait objects

When the set of variants is **closed** (you know them all, like `Shape::Circle | Shape::Square`), an **enum** with `match` is simpler, faster and lets the compiler check you handled everything. When the set is **open** (users or other crates add new kinds, plugins, UI widgets), use **trait objects**. This is the "expression problem": enums make adding operations easy, traits make adding types easy.

## Functions as values: `fn`, `Fn`, `FnMut`, `FnOnce`

Closures are values too, and there are three traits for calling them, depending on what they do with captured variables:

| Trait | The closure... | Can be called |
|---|---|---|
| `Fn` | only reads what it captured | many times, even concurrently |
| `FnMut` | changes what it captured | many times, one at a time (`mut`) |
| `FnOnce` | consumes what it captured | once |

Every `Fn` is also `FnMut` and `FnOnce`, so accept the **most general** one you can: `FnOnce` for a callback called once, `FnMut` for `for_each`-style callbacks, `Fn` when you share it. Plain functions (`fn double(x: i32) -> i32`) have the type `fn(i32) -> i32` and implement all three.

Storing closures:

```rust
struct Button {
    label: String,
    on_click: Box<dyn Fn() -> String>,
}

fn main() {
    let name = String::from("Ferris");
    let buttons = vec![
        Button { label: "hello".into(), on_click: Box::new(move || format!("hi {name}")) },
        Button { label: "time".into(), on_click: Box::new(|| "noon".to_string()) },
    ];
    for b in &buttons {
        println!("{}: {}", b.label, (b.on_click)());
    }
}
```

Note `(b.on_click)()`: the parentheses tell Rust to call the field, not a method named `on_click`.

Returning closures: `fn adder(n: i32) -> impl Fn(i32) -> i32 { move |x| x + n }` (one concrete closure), or `Box<dyn Fn(i32) -> i32>` when different branches return different closures.

## A plugin pipeline

Real tools combine these ideas: a list of `Box<dyn Stage>` steps run one after another (log processors, image filters, middleware in web servers). The first exercise builds one; the second stores closures as event handlers.

@@quiz
What is monomorphization?
- Turning a trait into a vtable
* Generating a separate copy of a generic function for each concrete type used
- Removing unused code
- Converting closures to function pointers
? That's why generics are as fast as hand-written code for each type.

@@quiz
Why might `Vec<Box<dyn Shape>>` be chosen over `Vec<S>` with `S: Shape`?
* It can hold different concrete shapes in one list
- It's always faster
- It uses less memory per item
- Generics can't be stored in a Vec
? A `Vec<S>` holds only one type `S`.

@@quiz
Which trait does a closure implement if it pushes to a captured `Vec`?
- `Fn`
* `FnMut`
- Only `FnOnce`
- None
? It mutates its captured state, so it needs `&mut` access each call.

@@quiz
When is an enum better than trait objects?
* When the set of variants is closed and known, so `match` can check every case
- Never
- When third-party crates need to add new variants
- When you need dynamic dispatch
? Open sets (plugins) suit trait objects.

@@exercise A text pipeline
Write a pipeline of text stages. The `Stage` trait has `name(&self) -> String` and `apply(&self, input: String) -> String`. Implement three stages:

- `Trim`: trims whitespace at both ends
- `Upper`: converts to uppercase
- `Replace { from: String, to: String }`: replaces every occurrence

Then write `run(stages: &[Box<dyn Stage>], input: &str) -> (String, Vec<String>)` that applies every stage in order and also returns the stage names in order.
@@starter
trait Stage {
    fn name(&self) -> String;
    fn apply(&self, input: String) -> String;
}

struct Trim;
struct Upper;
struct Replace {
    from: String,
    to: String,
}

// implement Stage for each

fn run(stages: &[Box<dyn Stage>], input: &str) -> (String, Vec<String>) {
    todo!()
}

fn main() {
    let stages: Vec<Box<dyn Stage>> = vec![
        Box::new(Trim),
        Box::new(Replace { from: "rust".into(), to: "ferris".into() }),
        Box::new(Upper),
    ];
    let (out, names) = run(&stages, "  hello rust  ");
    println!("{out}");
    println!("{}", names.join(" -> "));
}
@@expected
HELLO FERRIS
trim -> replace rust -> upper
@@tests
#[test]
fn checks() {
    let none: Vec<Box<dyn Stage>> = vec![];
    assert_eq!(run(&none, " x "), (" x ".to_string(), vec![]));
    let one: Vec<Box<dyn Stage>> = vec![Box::new(Upper)];
    assert_eq!(run(&one, "abc").0, "ABC");
    let order: Vec<Box<dyn Stage>> = vec![Box::new(Upper), Box::new(Replace { from: "a".into(), to: "b".into() })];
    assert_eq!(run(&order, "aA").0, "AA", "order matters: upper first, then nothing to replace");
}
@@requires
impl Stage for Trim
dyn Stage
@@hint
`Trim`'s name is `"trim"`, `Upper`'s is `"upper"`, and `Replace`'s is `format!("replace {}", self.from)`. In `run`, start with `let mut text = input.to_string();` and loop: `text = stage.apply(text);`.
@@solution
trait Stage {
    fn name(&self) -> String;
    fn apply(&self, input: String) -> String;
}

struct Trim;
struct Upper;
struct Replace {
    from: String,
    to: String,
}

impl Stage for Trim {
    fn name(&self) -> String {
        "trim".into()
    }
    fn apply(&self, input: String) -> String {
        input.trim().to_string()
    }
}

impl Stage for Upper {
    fn name(&self) -> String {
        "upper".into()
    }
    fn apply(&self, input: String) -> String {
        input.to_uppercase()
    }
}

impl Stage for Replace {
    fn name(&self) -> String {
        format!("replace {}", self.from)
    }
    fn apply(&self, input: String) -> String {
        input.replace(&self.from, &self.to)
    }
}

fn run(stages: &[Box<dyn Stage>], input: &str) -> (String, Vec<String>) {
    let mut text = input.to_string();
    let mut names = Vec::new();
    for stage in stages {
        text = stage.apply(text);
        names.push(stage.name());
    }
    (text, names)
}

fn main() {
    let stages: Vec<Box<dyn Stage>> = vec![
        Box::new(Trim),
        Box::new(Replace { from: "rust".into(), to: "ferris".into() }),
        Box::new(Upper),
    ];
    let (out, names) = run(&stages, "  hello rust  ");
    println!("{out}");
    println!("{}", names.join(" -> "));
}

@@exercise Event handlers
Build a tiny event bus. `Bus` stores handlers as `Vec<(String, Box<dyn FnMut(&str) -> String>)>`: an event name and a closure.

- `on(&mut self, event: &str, handler: impl FnMut(&str) -> String + 'static)` registers a handler.
- `emit(&mut self, event: &str, payload: &str) -> Vec<String>` calls every handler registered for that event, in order, and collects their results.

In `main`, a counting handler keeps state between calls (that's why the type is `FnMut`).
@@starter
struct Bus {
    handlers: Vec<(String, Box<dyn FnMut(&str) -> String>)>,
}

impl Bus {
    fn new() -> Self {
        Bus { handlers: Vec::new() }
    }

    fn on(&mut self, event: &str, handler: impl FnMut(&str) -> String + 'static) {
        todo!()
    }

    fn emit(&mut self, event: &str, payload: &str) -> Vec<String> {
        todo!()
    }
}

fn main() {
    let mut bus = Bus::new();
    let mut count = 0;
    bus.on("click", move |p| {
        count += 1;
        format!("click #{count} on {p}")
    });
    bus.on("click", |p| format!("log: {p}"));
    bus.on("key", |p| format!("key {p}"));
    println!("{:?}", bus.emit("click", "save"));
    println!("{:?}", bus.emit("click", "open"));
    println!("{:?}", bus.emit("scroll", "down"));
}
@@expected
["click #1 on save", "log: save"]
["click #2 on open", "log: open"]
[]
@@tests
#[test]
fn checks() {
    let mut bus = Bus::new();
    assert!(bus.emit("x", "y").is_empty());
    bus.on("x", |p| p.to_uppercase());
    assert_eq!(bus.emit("x", "hi"), vec!["HI".to_string()]);
    let mut seen = Vec::new();
    bus.on("x", move |p| {
        seen.push(p.to_string());
        seen.join(",")
    });
    bus.emit("x", "a");
    assert_eq!(bus.emit("x", "b"), vec!["B".to_string(), "a,b".to_string()]);
}
@@requires
Box::new(handler)
@@hint
`on`: `self.handlers.push((event.to_string(), Box::new(handler)));`. `emit`: loop with `for (name, h) in self.handlers.iter_mut()`, and when `name == event`, push `h(payload)`.
@@solution
struct Bus {
    handlers: Vec<(String, Box<dyn FnMut(&str) -> String>)>,
}

impl Bus {
    fn new() -> Self {
        Bus { handlers: Vec::new() }
    }

    fn on(&mut self, event: &str, handler: impl FnMut(&str) -> String + 'static) {
        self.handlers.push((event.to_string(), Box::new(handler)));
    }

    fn emit(&mut self, event: &str, payload: &str) -> Vec<String> {
        let mut out = Vec::new();
        for (name, h) in self.handlers.iter_mut() {
            if name == event {
                out.push(h(payload));
            }
        }
        out
    }
}

fn main() {
    let mut bus = Bus::new();
    let mut count = 0;
    bus.on("click", move |p| {
        count += 1;
        format!("click #{count} on {p}")
    });
    bus.on("click", |p| format!("log: {p}"));
    bus.on("key", |p| format!("key {p}"));
    println!("{:?}", bus.emit("click", "save"));
    println!("{:?}", bus.emit("click", "open"));
    println!("{:?}", bus.emit("scroll", "down"));
}
