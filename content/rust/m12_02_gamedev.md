@@lesson m12-gamedev 🎓 Beyond IT: Game Development with Rust and Bevy
@@text
> **Beyond IT scope.** Game programming is its own career path, separate from IT operations and business software. It's included so you see how Rust's ownership model shapes a whole different kind of program, and because the patterns here (data-oriented design, fixed time steps, spatial queries) show up in simulations, dashboards and visualisation tools too.

## The game loop

Every game runs the same loop many times per second:

```text
loop {
    read input            (keyboard, mouse, gamepad, network)
    update the world      (physics, AI, rules)    ← fixed time step
    render                (draw the current state)
}
```

Frame rates vary (a slow frame takes 30 ms, a fast one 4 ms). If physics used the raw frame time, a jump would go higher on fast computers. The standard fix is a **fixed time step with an accumulator**: add each frame's real time to an accumulator, then run as many fixed `update(1/60 s)` steps as fit, and carry the remainder. The second exercise implements it together with collision detection.

## Entity Component System (ECS)

Object-oriented games use class hierarchies (`Enemy extends Character extends GameObject`) that become tangled and fight Rust's borrow checker (everything wants a mutable reference to everything). **ECS** splits things differently:

- **Entities**: just IDs (`Entity(42)`).
- **Components**: plain data attached to entities: `Position`, `Velocity`, `Health`, `Player`.
- **Systems**: functions that run over every entity with a given set of components: "for everything with Position and Velocity, move it".

Components of one type are stored together in arrays, which is **cache-friendly** (see the performance lesson) and lets the engine run systems in parallel when they touch different components. The first exercise builds a tiny ECS.

## Bevy

**Bevy** is the most popular Rust game engine, built entirely on ECS:

```rust,norun
use bevy::prelude::*;

#[derive(Component)]
struct Velocity(Vec2);

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, (movement, bounce))
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
    commands.spawn((Sprite::from_color(Color::srgb(0.9, 0.3, 0.2), Vec2::splat(30.0)),
                    Transform::default(), Velocity(Vec2::new(150.0, 90.0))));
}

fn movement(time: Res<Time>, mut q: Query<(&mut Transform, &Velocity)>) {
    for (mut t, v) in &mut q {
        t.translation += v.0.extend(0.0) * time.delta_secs();
    }
}

fn bounce(mut q: Query<(&Transform, &mut Velocity)>) {
    for (t, mut v) in &mut q {
        if t.translation.x.abs() > 400.0 { v.0.x = -v.0.x; }
        if t.translation.y.abs() > 300.0 { v.0.y = -v.0.y; }
    }
}
```

- A system's **parameters** declare what it reads and writes (`Query<(&mut Transform, &Velocity)>`, `Res<Time>`), so Bevy schedules non-conflicting systems in parallel automatically.
- **Resources** are global singletons (score, settings); **events/observers** decouple systems; **states** handle menus vs gameplay.
- Plugins: `bevy_rapier`/`avian` (physics), `bevy_egui` (debug UI), `leafwing-input-manager`.
- Bevy compiles to Wasm, so games run in the browser (lesson 11.8).

## Other options

| Crate | Good for |
|---|---|
| **macroquad** | simple 2D games, very quick start, no ECS required |
| **ggez** | 2D, LÖVE-like |
| **Fyrox** | 3D engine with a scene editor |
| **wgpu** | low-level graphics API (WebGPU) under Bevy and many tools |
| **rapier** | 2D/3D physics used by many engines |
| **kira**, **rodio** | audio |

## Core game maths

- **Vectors** (`Vec2`, `Vec3` from `glam`): position, velocity, direction; length, normalise, dot product (is the enemy in front of me?).
- **AABB** (axis-aligned bounding box) collisions: two boxes overlap when they overlap on both the x and y axes. Cheap enough for thousands of checks; use a **spatial grid** or quadtree when there are many more.
- **Interpolation**: `a + (b - a) * t` (lerp) for smooth movement, cameras and animations.
- **Seeded randomness** for reproducible procedural levels (the xorshift generator from the testing lesson works fine).

@@quiz
In ECS, what is an entity?
- A class with methods
* An ID that components are attached to
- A rendering pipeline
- A physics engine
? Data lives in components; behaviour lives in systems.

@@quiz
Why use a fixed time step for physics?
* So the simulation behaves the same regardless of frame rate
- It makes rendering faster
- It reduces memory use
- Bevy requires it
? Variable steps make jumps and collisions depend on hardware speed.

@@quiz
When do two axis-aligned boxes overlap?
- When their centres are equal
* When their ranges overlap on both the x axis and the y axis
- When either axis overlaps
- When their areas are equal
? A gap on any one axis means no collision.

@@quiz
How does Bevy know which systems can run in parallel?
- It guesses
* From each system's parameters, which declare the components and resources it reads and writes
- Systems never run in parallel
- From comments
? Two systems writing the same component run one after the other.

@@flashcards
Game loop :: input, update, render, repeated every frame
Fixed time step :: physics updates with a constant delta using an accumulator
ECS :: entities (IDs), components (data), systems (logic)
Bevy :: data-driven Rust game engine built on ECS
Query (Bevy) :: system parameter selecting entities with given components
Resource (Bevy) :: global singleton data such as score or time
AABB :: axis-aligned bounding box used for cheap collision checks
Lerp :: linear interpolation `a + (b - a) * t`

@@exercise A tiny ECS
Store each component type in its own `Vec<Option<T>>` indexed by entity id.

- `spawn(&mut self) -> usize` adds an entity (pushing `None` to every component vector) and returns its id.
- `movement_system(&mut self, dt)`: for every entity that has **both** a position and a velocity, move it by `velocity * dt`.
- `damage_system(&mut self, hits: &[(usize, i32)]) -> Vec<usize>`: subtract damage from entities that have health; return the ids whose health dropped to 0 or below **in this call** (sorted), and remove their health component.
- `alive_with_position(&self) -> Vec<usize>`: ids that have a position and health.
@@starter
#[derive(Debug, Clone, Copy, PartialEq)]
struct Position { x: f32, y: f32 }
#[derive(Debug, Clone, Copy, PartialEq)]
struct Velocity { dx: f32, dy: f32 }

#[derive(Default)]
struct World {
    positions: Vec<Option<Position>>,
    velocities: Vec<Option<Velocity>>,
    health: Vec<Option<i32>>,
}

impl World {
    fn spawn(&mut self) -> usize {
        todo!()
    }

    fn movement_system(&mut self, dt: f32) {
        todo!()
    }

    fn damage_system(&mut self, hits: &[(usize, i32)]) -> Vec<usize> {
        todo!()
    }

    fn alive_with_position(&self) -> Vec<usize> {
        todo!()
    }
}

fn main() {
    let mut w = World::default();
    let player = w.spawn();
    w.positions[player] = Some(Position { x: 0.0, y: 0.0 });
    w.velocities[player] = Some(Velocity { dx: 2.0, dy: 1.0 });
    w.health[player] = Some(100);
    let wall = w.spawn();
    w.positions[wall] = Some(Position { x: 5.0, y: 5.0 });
    let enemy = w.spawn();
    w.positions[enemy] = Some(Position { x: 10.0, y: 0.0 });
    w.velocities[enemy] = Some(Velocity { dx: -1.0, dy: 0.0 });
    w.health[enemy] = Some(30);

    w.movement_system(0.5);
    w.movement_system(0.5);
    println!("{:?} {:?} {:?}", w.positions[player], w.positions[wall], w.positions[enemy]);
    println!("{:?}", w.damage_system(&[(enemy, 20), (player, 10), (wall, 99)]));
    println!("{:?}", w.damage_system(&[(enemy, 15)]));
    println!("{:?} {:?}", w.alive_with_position(), w.health);
}
@@expected
Some(Position { x: 2.0, y: 1.0 }) Some(Position { x: 5.0, y: 5.0 }) Some(Position { x: 9.0, y: 0.0 })
[]
[2]
[0] [Some(90), None, None]
@@tests
#[test]
fn checks() {
    let mut w = World::default();
    let a = w.spawn();
    let b = w.spawn();
    assert_eq!((a, b), (0, 1));
    assert_eq!(w.positions.len(), 2);
    assert_eq!(w.health.len(), 2);
    w.velocities[a] = Some(Velocity { dx: 1.0, dy: 1.0 });
    w.movement_system(1.0);
    assert_eq!(w.positions[a], None, "velocity without position does nothing");
    w.health[a] = Some(5);
    w.health[b] = Some(5);
    assert_eq!(w.damage_system(&[(b, 5), (a, 9)]), vec![0, 1]);
}
@@requires
fn movement_system
@@hint
`movement_system`: `for (p, v) in self.positions.iter_mut().zip(&self.velocities) { if let (Some(p), Some(v)) = (p, v) { p.x += v.dx * dt; p.y += v.dy * dt; } }`. In `damage_system`, use `if let Some(h) = self.health[id].as_mut()`, and when `*h <= 0` set `self.health[id] = None` and record the id.
@@solution
#[derive(Debug, Clone, Copy, PartialEq)]
struct Position { x: f32, y: f32 }
#[derive(Debug, Clone, Copy, PartialEq)]
struct Velocity { dx: f32, dy: f32 }

#[derive(Default)]
struct World {
    positions: Vec<Option<Position>>,
    velocities: Vec<Option<Velocity>>,
    health: Vec<Option<i32>>,
}

impl World {
    fn spawn(&mut self) -> usize {
        self.positions.push(None);
        self.velocities.push(None);
        self.health.push(None);
        self.positions.len() - 1
    }

    fn movement_system(&mut self, dt: f32) {
        for (p, v) in self.positions.iter_mut().zip(&self.velocities) {
            if let (Some(p), Some(v)) = (p, v) {
                p.x += v.dx * dt;
                p.y += v.dy * dt;
            }
        }
    }

    fn damage_system(&mut self, hits: &[(usize, i32)]) -> Vec<usize> {
        let mut dead = vec![];
        for &(id, dmg) in hits {
            if let Some(h) = self.health.get_mut(id).and_then(|h| h.as_mut()) {
                *h -= dmg;
                if *h <= 0 {
                    self.health[id] = None;
                    dead.push(id);
                }
            }
        }
        dead.sort();
        dead
    }

    fn alive_with_position(&self) -> Vec<usize> {
        (0..self.positions.len())
            .filter(|&i| self.positions[i].is_some() && self.health[i].is_some())
            .collect()
    }
}

fn main() {
    let mut w = World::default();
    let player = w.spawn();
    w.positions[player] = Some(Position { x: 0.0, y: 0.0 });
    w.velocities[player] = Some(Velocity { dx: 2.0, dy: 1.0 });
    w.health[player] = Some(100);
    let wall = w.spawn();
    w.positions[wall] = Some(Position { x: 5.0, y: 5.0 });
    let enemy = w.spawn();
    w.positions[enemy] = Some(Position { x: 10.0, y: 0.0 });
    w.velocities[enemy] = Some(Velocity { dx: -1.0, dy: 0.0 });
    w.health[enemy] = Some(30);

    w.movement_system(0.5);
    w.movement_system(0.5);
    println!("{:?} {:?} {:?}", w.positions[player], w.positions[wall], w.positions[enemy]);
    println!("{:?}", w.damage_system(&[(enemy, 20), (player, 10), (wall, 99)]));
    println!("{:?}", w.damage_system(&[(enemy, 15)]));
    println!("{:?} {:?}", w.alive_with_position(), w.health);
}

@@exercise Fixed time step and collisions
Two pieces every game needs.

- `Aabb { x, y, w, h }` (top-left corner, width, height). `overlaps(&self, other) -> bool`: true when the boxes overlap with positive area (boxes that only touch edges do **not** overlap).
- `FixedStep { step, accumulator }`: `advance(&mut self, frame_time) -> u32` adds the frame time and returns how many whole steps to run, keeping the remainder; to avoid the "spiral of death" after a long freeze, never return more than 5 steps (drop the excess time).
- `simulate(frames: &[f64]) -> (f64, u32)`: a ball starts at `x = 0` with speed 10 units/s and moves only in fixed steps of 0.1 s; return its final `x` and the total number of steps run. Use `FixedStep`.
@@starter
struct Aabb {
    x: f64,
    y: f64,
    w: f64,
    h: f64,
}

impl Aabb {
    fn overlaps(&self, other: &Aabb) -> bool {
        todo!()
    }
}

struct FixedStep {
    step: f64,
    accumulator: f64,
}

impl FixedStep {
    fn advance(&mut self, frame_time: f64) -> u32 {
        todo!()
    }
}

fn simulate(frames: &[f64]) -> (f64, u32) {
    todo!()
}

fn main() {
    let player = Aabb { x: 0.0, y: 0.0, w: 10.0, h: 10.0 };
    let others = [Aabb { x: 5.0, y: 5.0, w: 10.0, h: 10.0 }, Aabb { x: 10.0, y: 0.0, w: 5.0, h: 5.0 }, Aabb { x: 2.0, y: 20.0, w: 3.0, h: 3.0 }];
    let hits: Vec<String> = others.iter().map(|o| player.overlaps(o).to_string()).collect();
    println!("{}", hits.join(" "));
    let (x, steps) = simulate(&[0.05, 0.05, 0.25, 0.01]);
    println!("x = {x:.1} after {steps} steps");
    let (x, steps) = simulate(&[3.0]);
    println!("x = {x:.1} after {steps} steps");
}
@@expected
true false false
x = 3.0 after 3 steps
x = 5.0 after 5 steps
@@tests
#[test]
fn checks() {
    let a = Aabb { x: 0.0, y: 0.0, w: 4.0, h: 4.0 };
    assert!(a.overlaps(&Aabb { x: 1.0, y: 1.0, w: 1.0, h: 1.0 }), "contained");
    assert!(!a.overlaps(&Aabb { x: 0.0, y: 4.0, w: 4.0, h: 4.0 }), "touching edge");
    assert!(!a.overlaps(&Aabb { x: 5.0, y: 1.0, w: 1.0, h: 1.0 }));
    let mut s = FixedStep { step: 0.25, accumulator: 0.0 };
    assert_eq!(s.advance(0.6), 2);
    assert!((s.accumulator - 0.1).abs() < 1e-9);
    assert_eq!(s.advance(0.15), 1);
    assert_eq!(s.advance(10.0), 5);
    assert_eq!(s.accumulator, 0.0, "excess time is dropped");
}
@@requires
accumulator
@@hint
`overlaps`: `self.x < other.x + other.w && other.x < self.x + self.w` and the same for y. `advance`: add the time, then `while self.accumulator >= self.step - 1e-9 && n < 5 { self.accumulator -= self.step; n += 1; }`; if you hit 5, set the accumulator to 0. The small epsilon protects against floating-point sums like 0.05 + 0.05 being 0.09999….
@@solution
struct Aabb {
    x: f64,
    y: f64,
    w: f64,
    h: f64,
}

impl Aabb {
    fn overlaps(&self, other: &Aabb) -> bool {
        self.x < other.x + other.w
            && other.x < self.x + self.w
            && self.y < other.y + other.h
            && other.y < self.y + self.h
    }
}

struct FixedStep {
    step: f64,
    accumulator: f64,
}

impl FixedStep {
    fn advance(&mut self, frame_time: f64) -> u32 {
        self.accumulator += frame_time;
        let mut n = 0;
        while self.accumulator >= self.step - 1e-9 && n < 5 {
            self.accumulator -= self.step;
            n += 1;
        }
        if n == 5 {
            self.accumulator = 0.0;
        }
        self.accumulator = self.accumulator.max(0.0);
        n
    }
}

fn simulate(frames: &[f64]) -> (f64, u32) {
    let mut clock = FixedStep { step: 0.1, accumulator: 0.0 };
    let (mut x, mut total) = (0.0, 0);
    for &f in frames {
        for _ in 0..clock.advance(f) {
            x += 10.0 * clock.step;
            total += 1;
        }
    }
    (x, total)
}

fn main() {
    let player = Aabb { x: 0.0, y: 0.0, w: 10.0, h: 10.0 };
    let others = [Aabb { x: 5.0, y: 5.0, w: 10.0, h: 10.0 }, Aabb { x: 10.0, y: 0.0, w: 5.0, h: 5.0 }, Aabb { x: 2.0, y: 20.0, w: 3.0, h: 3.0 }];
    let hits: Vec<String> = others.iter().map(|o| player.overlaps(o).to_string()).collect();
    println!("{}", hits.join(" "));
    let (x, steps) = simulate(&[0.05, 0.05, 0.25, 0.01]);
    println!("x = {x:.1} after {steps} steps");
    let (x, steps) = simulate(&[3.0]);
    println!("x = {x:.1} after {steps} steps");
}
