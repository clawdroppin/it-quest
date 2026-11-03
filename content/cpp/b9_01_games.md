@@lesson b9-games 🎓 Beyond IT: Game Development with raylib, SDL and ECS
@@concepts objects, loops
@@text
> **Beyond IT scope.** This module goes past IT and business software into games, embedded systems and SIMD. C++ dominates all three, and they teach you a lot about performance, but none of them is needed for IT jobs. Skip it freely if that's your goal.

Most commercial game engines (Unreal, id Tech, Frostbite and Unity's core) are written in C++. To learn the fundamentals without a huge engine, use a small library: **raylib** (very beginner-friendly) or **SDL3** (windowing, input, audio and 2D rendering, used by Valve and countless indies).

```cpp,norun
#include "raylib.h"

int main() {
    InitWindow(800, 450, "Dodge");
    SetTargetFPS(60);
    Vector2 player = {400, 225};
    while (!WindowShouldClose()) {                       // the game loop
        float dt = GetFrameTime();
        if (IsKeyDown(KEY_RIGHT)) player.x += 200 * dt;  // input + update
        if (IsKeyDown(KEY_LEFT)) player.x -= 200 * dt;
        BeginDrawing();                                  // render
        ClearBackground(RAYWHITE);
        DrawCircleV(player, 20, MAROON);
        EndDrawing();
    }
    CloseWindow();
}
```

## Entity Component System

Deep inheritance trees (`Enemy : Character : Actor : Object`) get rigid fast. Many engines use **ECS** instead:

- an **entity** is just an id
- **components** are plain data (`Position`, `Velocity`, `Health`) stored in tightly packed arrays, one per component type
- **systems** are functions that loop over every entity having a certain set of components (movement: Position + Velocity)

Storage is often a **sparse set**: a dense array of components (fast iteration, cache-friendly), a dense array of their entity ids, and a sparse array from entity id to dense index (O(1) lookup). Removing swaps the last element into the hole, so the arrays stay packed. EnTT (used in Minecraft Bedrock) and flecs are popular C++ ECS libraries.

## Collision detection

The workhorse is the **axis-aligned bounding box** (AABB): two boxes overlap when they overlap on both axes. For platformers, move one axis at a time and push the box back out of whatever it hits; that tells you whether you hit a wall (x) or landed (y). Bigger worlds add a broad phase (spatial grid, quadtree) so you only test nearby pairs.

@@quiz
In an ECS, what is an entity?
* Just an identifier; its data lives in component arrays
- A class with virtual update()
- A system
- A scene file
? Behaviour lives in systems.

@@quiz
Why does a sparse set swap the last element into a removed slot?
- To sort the components
* To keep the dense array packed, so iteration stays fast and removal is O(1)
- To free memory
- It's required by the standard library
? The sparse index of the moved entity is updated.

@@quiz
Why resolve platformer collisions one axis at a time?
- It's faster to compute
* You know which axis caused the hit, so you can stop horizontal movement at walls and detect landing on floors
- AABBs only work on one axis
- To avoid floating point
? Move x, resolve; move y, resolve.

@@flashcards
Game loop :: input → update → render, every frame
`GetFrameTime()` :: seconds since last frame (raylib)
ECS :: entities, components, systems
Component :: plain data
System :: logic over entities with certain components
Sparse set :: dense data + sparse id→index map
AABB :: axis-aligned bounding box
Broad phase :: cheap filtering before exact collision tests
raylib / SDL3 :: lightweight C/C++ game libraries

@@exercise A sparse-set ECS
Write a tiny ECS:

- `Pool<T>`: a sparse set with `insert(e, value)`, `contains(e)`, `get(e)`, `remove(e)` (swap with the last element), and `entities()` in dense order
- `World`: `create(name)` reuses ids of destroyed entities (most recently freed first), `destroy(e)` removes the entity from every pool
- components `Position { float x, y; }`, `Velocity { float dx, dy; }`, `Health { int hp, decay; }`
- each tick: the movement system adds velocity to position; the health system subtracts `decay` and destroys entities that reach 0

Entities: player (0,0) moving (1,0) with 100 hp, enemy (10,0) moving (-1,0) with 30 hp losing 15 per tick, a rock at (5,5) with only a position, and a bullet at (0,0) moving (5,0). Print positions in the pool's dense order after each tick.
@@starter
#include <cstdint>
#include <iostream>
#include <string>
#include <vector>

using Entity = uint32_t;

int main() {
    std::cout << "TODO\n";
}
@@expected
tick 1: player(1,0) enemy(9,0) rock(5,5) bullet(5,0)
tick 2: enemy destroyed
tick 2: player(2,0) bullet(10,0) rock(5,5)
tick 3: player(3,0) bullet(15,0) rock(5,5)
reused id for spark: 1
@@requires
sparse
dense
@@hint
`std::vector<size_t> sparse_` (entity → dense index, `SIZE_MAX` for none), `std::vector<Entity> dense_` and `std::vector<T> data_`. Remove: `size_t i = sparse_[e], last = dense_.size() - 1; dense_[i] = dense_[last]; data_[i] = data_[last]; sparse_[dense_[i]] = i; dense_.pop_back(); data_.pop_back(); sparse_[e] = SIZE_MAX;`. Collect dying entities first and destroy them after the loop.
@@solution
#include <cstdint>
#include <iostream>
#include <string>
#include <vector>

using Entity = uint32_t;

template <class T>
class Pool {
public:
    void insert(Entity e, T value) {
        if (e >= sparse_.size()) sparse_.resize(e + 1, npos);
        sparse_[e] = dense_.size();
        dense_.push_back(e);
        data_.push_back(value);
    }

    bool contains(Entity e) const { return e < sparse_.size() && sparse_[e] != npos; }
    T& get(Entity e) { return data_[sparse_[e]]; }

    void remove(Entity e) {
        if (!contains(e)) return;
        size_t i = sparse_[e], last = dense_.size() - 1;
        dense_[i] = dense_[last];
        data_[i] = data_[last];
        sparse_[dense_[i]] = i;
        dense_.pop_back();
        data_.pop_back();
        sparse_[e] = npos;
    }

    const std::vector<Entity>& entities() const { return dense_; }

private:
    static constexpr size_t npos = SIZE_MAX;
    std::vector<size_t> sparse_;
    std::vector<Entity> dense_;
    std::vector<T> data_;
};

struct Position { float x, y; };
struct Velocity { float dx, dy; };
struct Health { int hp, decay; };

struct World {
    Pool<Position> positions;
    Pool<Velocity> velocities;
    Pool<Health> healths;
    std::vector<std::string> names;
    std::vector<Entity> free_ids;

    Entity create(const std::string& name) {
        if (!free_ids.empty()) {
            Entity e = free_ids.back();
            free_ids.pop_back();
            names[e] = name;
            return e;
        }
        names.push_back(name);
        return static_cast<Entity>(names.size() - 1);
    }

    void destroy(Entity e) {
        positions.remove(e);
        velocities.remove(e);
        healths.remove(e);
        free_ids.push_back(e);
    }
};

void movement(World& w) {
    for (Entity e : w.velocities.entities()) {
        if (!w.positions.contains(e)) continue;
        auto& p = w.positions.get(e);
        const auto& v = w.velocities.get(e);
        p.x += v.dx;
        p.y += v.dy;
    }
}

void health(World& w, int tick) {
    std::vector<Entity> dying;
    for (Entity e : w.healths.entities()) {
        auto& h = w.healths.get(e);
        h.hp -= h.decay;
        if (h.hp <= 0) dying.push_back(e);
    }
    for (Entity e : dying) {
        std::cout << "tick " << tick << ": " << w.names[e] << " destroyed\n";
        w.destroy(e);
    }
}

int main() {
    World w;
    Entity player = w.create("player"), enemy = w.create("enemy"), rock = w.create("rock"), bullet = w.create("bullet");
    w.positions.insert(player, {0, 0});
    w.velocities.insert(player, {1, 0});
    w.healths.insert(player, {100, 0});
    w.positions.insert(enemy, {10, 0});
    w.velocities.insert(enemy, {-1, 0});
    w.healths.insert(enemy, {30, 15});
    w.positions.insert(rock, {5, 5});
    w.positions.insert(bullet, {0, 0});
    w.velocities.insert(bullet, {5, 0});

    for (int tick = 1; tick <= 3; ++tick) {
        movement(w);
        health(w, tick);
        std::cout << "tick " << tick << ':';
        for (Entity e : w.positions.entities()) {
            const auto& p = w.positions.get(e);
            std::cout << ' ' << w.names[e] << '(' << p.x << ',' << p.y << ')';
        }
        std::cout << '\n';
    }
    std::cout << "reused id for spark: " << w.create("spark") << '\n';
}

@@exercise AABB collisions in a platformer
Move a 10×10 player box from (0,0) with velocity (4, 6) per step through a level with a floor `{0, 40, 100, 10}` and a wall `{50, 0, 10, 40}` (boxes are `{x, y, w, h}`, half-open: `[x, x + w)`):

- `overlaps(a, b)` is true when the boxes overlap on both axes
- each step: move on x, and if the box overlaps a solid, push it back to the solid's edge (and note a wall hit); then the same on y, where landing on something sets `on_ground` and stops vertical movement
- print the first landing and the first wall hit with their step numbers, then the final state after 12 steps
@@starter
#include <iostream>
#include <vector>

struct Box {
    int x, y, w, h;
};

int main() {
    std::vector<Box> solids = {{0, 40, 100, 10}, {50, 0, 10, 40}};
    std::cout << "TODO\n";
}
@@expected
step 6: landed at y=30
step 11: hit wall at x=40
final after 12 steps: (40,30) on ground: true
@@requires
overlaps
@@hint
`bool overlaps(const Box& a, const Box& b) { return a.x < b.x + b.w && b.x < a.x + a.w && a.y < b.y + b.h && b.y < a.y + a.h; }`. Moving right into a solid: `p.x = s.x - p.w;` moving left: `p.x = s.x + s.w;` (and the same for y). Use two booleans to print each event only once.
@@solution
#include <iostream>
#include <vector>

struct Box {
    int x, y, w, h;
};

bool overlaps(const Box& a, const Box& b) {
    return a.x < b.x + b.w && b.x < a.x + a.w && a.y < b.y + b.h && b.y < a.y + a.h;
}

int main() {
    std::vector<Box> solids = {{0, 40, 100, 10}, {50, 0, 10, 40}};
    Box p{0, 0, 10, 10};
    int vx = 4, vy = 6;
    bool on_ground = false, reported_land = false, reported_wall = false;

    for (int step = 1; step <= 12; ++step) {
        p.x += vx;
        for (const auto& s : solids) {
            if (!overlaps(p, s)) continue;
            p.x = vx > 0 ? s.x - p.w : s.x + s.w;
            if (!reported_wall) {
                std::cout << "step " << step << ": hit wall at x=" << p.x << '\n';
                reported_wall = true;
            }
        }
        p.y += vy;
        for (const auto& s : solids) {
            if (!overlaps(p, s)) continue;
            if (vy > 0) {
                p.y = s.y - p.h;
                on_ground = true;
                if (!reported_land) {
                    std::cout << "step " << step << ": landed at y=" << p.y << '\n';
                    reported_land = true;
                }
            } else {
                p.y = s.y + s.h;
            }
            vy = 0;
        }
    }
    std::cout << std::boolalpha << "final after 12 steps: (" << p.x << ',' << p.y << ") on ground: " << on_ground << '\n';
}
