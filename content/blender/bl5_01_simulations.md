@@lesson bl5-sims Simulations and VFX: Particles, Fire, Smoke, Fluids, Rigid Bodies and Cloth
@@concepts numbers, lists, loops, functions
@@lang python
@@text
Explosions, smoke, splashing liquid, collapsing walls, flags in the wind: Blender simulates them all. Simulations are computed frame by frame from physical rules, then **cached** or **baked** so they play back and render consistently.

## How every simulation works

Each frame, the solver takes the current **state** (positions, velocities, densities), applies **forces** (gravity, wind, collisions), and advances by a **time step**. Smaller steps (more **substeps**) are more accurate and slower. The exercises build the simplest solver, explicit **Euler integration**:

```text
velocity += acceleration * dt
position += velocity * dt
```

Everything in Blender's Physics tab is a much more advanced version of this loop.

## Particle systems

Properties → **Particles**: an **Emitter** shoots particles from a mesh (count, frame start/end, lifetime, velocity from normals, random). Render them as **Halo**, **Object** or **Collection** (debris, confetti, raindrops). **Hair** particles (now mostly replaced by **curves hair** with Geometry Nodes) grow strands for fur and grass.

**Force fields** (Shift+A → Force Field): Wind, Turbulence, Vortex, Force (attract/repel), Drag, Harmonic, Magnetic. They affect particles, rigid bodies, cloth and smoke.

For full control, modern setups use **Geometry Nodes Simulation Zones** (the previous lesson): points with a velocity attribute, updated each frame.

## Fire and smoke (Mantaflow)

1. Select an object → Object → **Quick Effects → Quick Smoke** (creates a domain box, a flow source and materials). Or set up manually in **Physics → Fluid**:
   - **Domain** (type Gas): the box the simulation lives in. **Resolution divisions** (32 for tests, 128–256+ for finals). Adaptive domain shrinks to the smoke.
   - **Flow**: the source, type Smoke, Fire, or Fire + Smoke; behaviour Inflow; use flow source from mesh or particles; **temperature** and **fuel**.
   - **Effector**: colliders and guides.
2. Domain settings: **vorticity** (swirls), **buoyancy** (density/heat: how fast smoke rises), **dissolve** (fades smoke), **Noise** (upscales detail cheaply), **Fire: reaction speed, flame smoke, flame vorticity**.
3. **Cache**: type Modular or All; **Bake Data** then **Bake Noise**. Bake to a folder; caches can be gigabytes.
4. Render with a **Principled Volume** shader: density from the density attribute, **blackbody** emission from the temperature/flame attribute for fire colour. EEVEE previews; Cycles gives the best volumes.

## Liquids

Physics → Fluid, Domain type **Liquid** (FLIP solver): **Flow** objects (Geometry for an initial volume, Inflow for a tap, Outflow to remove), **Effectors** as containers (collision with a thin plane needs "Is Planar"). Enable **Mesh** in the domain for a renderable surface and **Particles** (spray, foam, bubbles) for whitewater. Bake data, then mesh. Liquid needs a **Glass** or Principled with Transmission 1 and IOR 1.33.

Scale matters: the domain's real-world size (in metres) changes the behaviour; a coffee cup and an ocean wave need different scale settings (**Domain → Settings → Real World Size**).

## Rigid bodies

Physics → **Rigid Body**: **Active** (falls, collides) or **Passive** (static ground, or animated kinematic). Shape: Box, Sphere, Convex Hull, Mesh (slow). Mass, friction, **bounciness** (restitution), damping. **Rigid body constraints** (Fixed, Hinge, Point, Spring) hold pieces together until a **breaking threshold**: perfect for destruction. Use the **Cell Fracture** add-on to shatter an object into pieces, then Object → Rigid Body → **Add Active** to all. Scene properties → Rigid Body World sets substeps and solver iterations; bake with Bake to Keyframes for editing.

## Cloth and soft bodies

Physics → **Cloth**: presets (cotton, silk, leather, denim, rubber); **pin groups** (vertex group of vertices that hold still: a flag's edge, a curtain's rod); **collisions** on other objects (and self-collision); **pressure** for inflating balloons and cushions; **shrinking** for vacuum-sealed looks. Subdivide the mesh enough for folds. Soft bodies jiggle (jelly, bellies).

## Dynamic paint and Ocean

- **Dynamic Paint**: a brush object leaves wet maps, ripples or displacement on a canvas (footprints in snow, rain ripples).
- **Ocean modifier**: an infinite procedural ocean with foam maps, fast and great for wide shots.

## VFX shot workflow

1. **Plate**: film the live-action shot (or render a background).
2. **Track** the camera (Lesson 7) so CG matches the movement.
3. **Simulate** at low resolution for timing, then high-resolution with noise for detail.
4. **Render** CG elements in separate view layers with **shadow catcher** (Object → Visibility → Shadow Catcher in Cycles) so shadows land on the real floor.
5. **Composite** CG over the plate with colour matching, glow and grain (Lesson 7, or Fusion in DaVinci Resolve).

## Caching tips

Simulations are deterministic only if nothing changes: bake before rendering on several machines, store caches on a fast SSD, and name folders per version (`cache_v03_hires`). Change the start frame or scale and you must re-bake.

@@quiz
In Mantaflow, which object type defines the box where smoke or liquid can exist?
* Domain
- Flow
- Effector
- Emitter
? Flows are sources, effectors are colliders and guides.

@@quiz
What makes rigid body pieces stay together until hit hard enough?
- Passive type
* Rigid body constraints with a breaking threshold
- Higher resolution
- A Principled Volume shader
? Fixed constraints between fractured pieces break when forces exceed the threshold.

@@quiz
How do you make a flag stay attached to its pole in a cloth simulation?
- Make the pole passive
* Pin the edge vertices with a pin vertex group
- Increase gravity
- Use Dynamic Paint
? Pinned vertices follow the mesh's animation and don't simulate.

@@quiz
Why does the real-world size of a fluid domain matter?
- It changes the render resolution
* Physics behaves differently at different scales: a cup of coffee and an ocean splash move differently
- Larger domains always bake faster
- It doesn't
? Set it to match the scene for believable motion.

@@quiz
How do CG shadows land on a real floor in a VFX shot rendered with Cycles?
- Paint them in by hand
* Use a plane marked as a Shadow Catcher, rendering only the shadows with transparency
- Use EEVEE
- Turn off the world light
? Composite the result over the plate.

@@drill
What's the name of Blender's fluid, fire and smoke simulation system?
= Mantaflow
? Simulations are baked to disk before rendering.

@@flashcards
simulation step :: apply forces, update velocity, update position
substeps :: more, smaller time steps for accuracy
cache / bake :: store simulation results on disk
particle emitter :: mesh that emits particles
force fields :: wind, turbulence, vortex, force, drag...
Mantaflow :: Blender's smoke, fire and liquid solver
Domain / Flow / Effector :: sim box / source / collider
vorticity / buoyancy / dissolve :: swirls / rising / fading smoke
blackbody :: temperature-based fire colour in shaders
FLIP :: liquid solver with particles and mesh
rigid body active / passive :: simulated / static or animated
Cell Fracture :: shatter an object into pieces
breaking threshold :: force at which a constraint breaks
cloth pin group :: vertices that don't simulate
Shadow Catcher :: plane that renders only received shadows
Dynamic Paint :: brushes leave maps, ripples, displacement
Ocean modifier :: procedural ocean surface

@@exercise A particle solver
Write `step(p, v, gravity, drag, dt)` for one particle: acceleration is `gravity - drag * v` per component, then semi-implicit Euler: `v = v + a * dt`, then `p = p + v * dt`. Return `(p, v)` as tuples rounded to 4. Then `simulate(p, v, gravity, drag, fps, frames, floor=0.0, bounce=0.5)`: substep 4 times per frame (`dt = 1 / fps / 4`); when a particle goes below the floor height (z), put it on the floor and reverse its z velocity multiplied by `bounce`. Return the list of positions at the end of each frame (rounded to 3).
@@starter
def step(p, v, gravity, drag, dt):
    pass


def simulate(p, v, gravity, drag, fps, frames, floor=0.0, bounce=0.5):
    pass


path = simulate((0, 0, 2), (1, 0, 0), (0, 0, -9.81), 0.0, 24, 30)
print(path[::6])
@@tests
p, v = step((0, 0, 0), (0, 0, 0), (0, 0, -10), 0.0, 0.1)
check(v, (0, 0, -1.0), "gravity speeds it up")
check(p, (0, 0, -0.1), "then it moves")
p, v = step((0, 0, 0), (2, 0, 0), (0, 0, 0), 0.5, 0.1)
check(v, (1.9, 0, 0), "drag slows it")
path = simulate((0, 0, 2), (1, 0, 0), (0, 0, -9.81), 0.0, 24, 30)
check(len(path), 30, "one position per frame")
check(path[0][0], 0.042, "x after one frame")
check(all(z >= 0 for _, _, z in path), True, "never below the floor")
check(path[11][2] < 1.0, True, "falling")
check(max(z for _, _, z in path[20:]) < 2, True, "bounces lower")
still = simulate((0, 0, 0), (0, 0, 0), (0, 0, 0), 0.0, 24, 3)
check(still, [(0, 0, 0), (0, 0, 0), (0, 0, 0)], "no forces, no motion")
@@hint
Work with lists or zip over the three components. After each substep, if `p[2] < floor`, set `p = (p[0], p[1], floor)` and `v = (v[0], v[1], -v[2] * bounce)`. Round only when storing each frame's position.
@@solution
def step(p, v, gravity, drag, dt):
    a = [g - drag * vi for g, vi in zip(gravity, v)]
    v = [vi + ai * dt for vi, ai in zip(v, a)]
    p = [pi + vi * dt for pi, vi in zip(p, v)]
    return tuple(round(x, 4) for x in p), tuple(round(x, 4) for x in v)


def simulate(p, v, gravity, drag, fps, frames, floor=0.0, bounce=0.5):
    dt = 1 / fps / 4
    out = []
    for _ in range(frames):
        for _ in range(4):
            p, v = step(p, v, gravity, drag, dt)
            if p[2] < floor:
                p = (p[0], p[1], floor)
                v = (v[0], v[1], -v[2] * bounce)
        out.append(tuple(round(x, 3) for x in p))
    return out


path = simulate((0, 0, 2), (1, 0, 0), (0, 0, -9.81), 0.0, 24, 30)
print(path[::6])

@@exercise Rigid body bounces and fracture budgets
A ball dropped from height `h` hits the ground at speed `sqrt(2 g h)`; with restitution `e` it leaves at `e` times that speed and rises to `e² h`. Write `bounce_heights(h, e, n)`: the peak heights after each of `n` bounces, rounded to 3. Write `time_to_rest(h, e, g=9.81, min_h=0.001)`: total time in seconds (rounded to 2) until a bounce peak drops below `min_h`: the first fall takes `sqrt(2h/g)`, each later bounce up and down takes `2 sqrt(2 h_i / g)` while `h_i >= min_h`. Then `constraint_count(pieces, neighbours)`: with Cell Fracture, every piece links to `neighbours` others with a fixed constraint, each constraint shared by two pieces: return the number of constraints.
@@starter
import math


def bounce_heights(h, e, n):
    pass


def time_to_rest(h, e, g=9.81, min_h=0.001):
    pass


def constraint_count(pieces, neighbours):
    pass


print(bounce_heights(2, 0.7, 4), time_to_rest(2, 0.7), constraint_count(50, 6))
@@tests
check(bounce_heights(2, 0.5, 3), [0.5, 0.125, 0.031], "a soft ball")
check(bounce_heights(1, 1.0, 2), [1.0, 1.0], "perfectly elastic")
check(bounce_heights(5, 0.0, 2), [0.0, 0.0], "a dead stop")
check(time_to_rest(1, 0.0), 0.45, "falls once and stops")
check(time_to_rest(2, 0.5), 1.88, "a few bounces")
check(time_to_rest(2, 0.7) > time_to_rest(2, 0.5), True, "bouncier rests later")
check(constraint_count(50, 6), 150, "shared constraints")
check(constraint_count(2, 1), 1, "two pieces, one link")
@@hint
Each bounce multiplies the height by `e * e`. For the time, start with the fall time, then loop: `h *= e * e`; while `h >= min_h`, add `2 * sqrt(2 * h / g)`.
@@solution
import math


def bounce_heights(h, e, n):
    out = []
    for _ in range(n):
        h = h * e * e
        out.append(round(h, 3))
    return out


def time_to_rest(h, e, g=9.81, min_h=0.001):
    t = math.sqrt(2 * h / g)
    h = h * e * e
    while h >= min_h:
        t += 2 * math.sqrt(2 * h / g)
        h = h * e * e
    return round(t, 2)


def constraint_count(pieces, neighbours):
    return pieces * neighbours // 2


print(bounce_heights(2, 0.7, 4), time_to_rest(2, 0.7), constraint_count(50, 6))
