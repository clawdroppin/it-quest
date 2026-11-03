@@lesson g10-3d 🎓 Beyond IT: 3D Graphics with three.js and WebGPU
@@concepts numbers, objects
@@text
> **Beyond IT scope.** 3D graphics is its own specialty (games, visualisation, CAD, VR). It's included because the browser can render real-time 3D, and the maths behind it (vectors, matrices, projection) is worth seeing once. Skip it freely if your goal is an IT job.

Browsers expose the GPU through **WebGL** (since 2011) and the newer **WebGPU** (in all major browsers by 2025). Writing raw WebGL/WebGPU takes hundreds of lines for one triangle, so most people use **three.js** (or Babylon.js, PlayCanvas).

## A spinning cube in three.js

```javascript,norun
import * as THREE from "three";

const scene = new THREE.Scene();
const camera = new THREE.PerspectiveCamera(60, innerWidth / innerHeight, 0.1, 100);  // fov, aspect, near, far
camera.position.set(0, 1.5, 4);
camera.lookAt(0, 0, 0);

const renderer = new THREE.WebGLRenderer({ antialias: true });
renderer.setSize(innerWidth, innerHeight);
document.body.append(renderer.domElement);

const cube = new THREE.Mesh(
  new THREE.BoxGeometry(1, 1, 1),                                   // shape
  new THREE.MeshStandardMaterial({ color: 0x2f6fde, roughness: 0.4 }), // surface
);
scene.add(cube);
scene.add(new THREE.AmbientLight(0xffffff, 0.3));
const sun = new THREE.DirectionalLight(0xffffff, 2);
sun.position.set(3, 5, 2);
scene.add(sun);

renderer.setAnimationLoop((time) => {
  cube.rotation.y = time / 1000;
  renderer.render(scene, camera);
});
```

The core concepts: a **scene graph** of objects, each a **mesh** = **geometry** (vertices and triangles) + **material** (how light affects the surface); **lights**; and a **camera**. Models are usually loaded from **glTF** files (`GLTFLoader`), the "JPEG of 3D", exported from Blender (see the Blender lesson in the creative extras track).

## The maths underneath

Everything is triangles, and every vertex goes through a chain of transformations:

```text
model space ──model matrix──▶ world space ──view matrix──▶ camera space ──projection──▶ clip space ──÷w──▶ screen
 (cube's own     (position,        (relative to           (perspective:          (pixels)
  coordinates)    rotation, scale)   the camera)             far things smaller)
```

- **Vectors** represent positions and directions. Key operations: add, scale, **length**, **normalise** (length 1), **dot product** (`a·b = |a||b|cos θ`: how aligned two directions are, used for lighting) and **cross product** (a vector perpendicular to both: surface normals).
- **4×4 matrices** encode translation, rotation and scale, and combine by multiplication, so one matrix can do all three. The fourth coordinate (`w`) makes translation and perspective work.
- **Perspective projection** divides by depth: a point twice as far away appears half as big.
- **Lighting** (Lambert diffuse): brightness = max(0, normal · direction-to-light). That one dot product is why a sphere looks round.

## Shaders

The GPU runs small programs called **shaders** for every vertex (position it) and every pixel (colour it), in parallel. three.js writes them for you, but you can write your own in GLSL (WebGL) or WGSL (WebGPU) for water, glows and other effects.

## Performance

- Fewer **draw calls**: merge geometry, use `InstancedMesh` for thousands of identical objects (a server-rack visualisation, a forest).
- Keep textures power-of-two-ish and compressed (KTX2), and reuse materials.
- Dispose of geometries, materials and textures you remove, or GPU memory leaks.

## Beyond games

3D in the browser is used for data-centre and network visualisations, digital twins of buildings, product configurators, maps (CesiumJS, deck.gl), medical imaging and WebXR (VR/AR).

@@quiz
In three.js, what is a mesh?
* A geometry (shape) combined with a material (surface appearance)
- A light source
- A camera setting
- A texture file
? Meshes are added to the scene and drawn by the renderer.

@@quiz
What does the dot product of a surface normal and the direction to a light tell you?
- The distance to the light
* How directly the surface faces the light, which sets its diffuse brightness
- The colour of the light
- Whether the object is visible
? max(0, n·l) is Lambert lighting.

@@quiz
Why do far-away points look smaller in a perspective projection?
- They are scaled down in the model
* Their screen coordinates are divided by their depth
- The GPU blurs them
- Cameras zoom out automatically
? This is the divide by w after projection.

@@flashcards
WebGL / WebGPU :: browser APIs for GPU rendering
three.js :: popular 3D library for the web
Scene graph :: tree of objects with transforms
Mesh :: geometry + material
glTF :: standard 3D model format for the web
Normal :: vector perpendicular to a surface
Dot product :: measures alignment; used for lighting
Cross product :: vector perpendicular to two others
Perspective divide :: dividing by depth for 3D-to-2D projection
Shader :: GPU program for vertices or pixels
InstancedMesh :: draws many copies in one draw call

@@exercise Vector maths
Write a small immutable `Vec3` class:

- `new Vec3(x, y, z)`; methods `add(v)`, `sub(v)`, `scale(s)`, `dot(v)`, `cross(v)`, `length()`, `normalize()` (a zero vector stays zero), `toArray()` (each component rounded to 4 decimals, with `-0` turned into `0`)
- `static faceNormal(a, b, c)`: the normalised normal of the triangle `a, b, c` (counter-clockwise: `(b − a) × (c − a)`)
- `static lambert(normal, toLight)`: `max(0, n̂ · l̂)` (normalise both), rounded to 4 decimals
@@starter
class Vec3 {
  constructor(x, y, z) {
    throw new Error("TODO: not done yet");
  }
}

@@tests
const a = new Vec3(1, 2, 3);
const b = new Vec3(4, 5, 6);
check([a.add(b).toArray(), b.sub(a).toArray(), a.scale(2).toArray(), a.dot(b)], [[5, 7, 9], [3, 3, 3], [2, 4, 6], 32], "basics");
check(new Vec3(1, 0, 0).cross(new Vec3(0, 1, 0)).toArray(), [0, 0, 1], "x cross y = z");
check([new Vec3(3, 4, 0).length(), new Vec3(3, 4, 0).normalize().toArray(), new Vec3(0, 0, 0).normalize().toArray()], [5, [0.6, 0.8, 0], [0, 0, 0]], "length and normalize");
check(Vec3.faceNormal(new Vec3(0, 0, 0), new Vec3(1, 0, 0), new Vec3(0, 1, 0)).toArray(), [0, 0, 1], "face normal");
check([Vec3.lambert(new Vec3(0, 1, 0), new Vec3(0, 10, 0)), Vec3.lambert(new Vec3(0, 1, 0), new Vec3(1, 1, 0)), Vec3.lambert(new Vec3(0, 1, 0), new Vec3(0, -1, 0))], [1, 0.7071, 0], "lighting");
const frozen = new Vec3(1, 1, 1);
frozen.add(new Vec3(1, 1, 1));
check(frozen.toArray(), [1, 1, 1], "immutable");
@@hint
Each method returns a `new Vec3(...)`. Cross product: `(ay*bz − az*by, az*bx − ax*bz, ax*by − ay*bx)`. For rounding: `const r = (n) => Math.round(n * 10000) / 10000 + 0;` (adding 0 turns `-0` into `0`).
@@solution
const round4 = (n) => Math.round(n * 10000) / 10000 + 0;

class Vec3 {
  constructor(x, y, z) {
    this.x = x;
    this.y = y;
    this.z = z;
    Object.freeze(this);
  }

  add(v) { return new Vec3(this.x + v.x, this.y + v.y, this.z + v.z); }
  sub(v) { return new Vec3(this.x - v.x, this.y - v.y, this.z - v.z); }
  scale(s) { return new Vec3(this.x * s, this.y * s, this.z * s); }
  dot(v) { return this.x * v.x + this.y * v.y + this.z * v.z; }

  cross(v) {
    return new Vec3(
      this.y * v.z - this.z * v.y,
      this.z * v.x - this.x * v.z,
      this.x * v.y - this.y * v.x,
    );
  }

  length() { return Math.hypot(this.x, this.y, this.z); }

  normalize() {
    const len = this.length();
    return len === 0 ? new Vec3(0, 0, 0) : this.scale(1 / len);
  }

  toArray() { return [this.x, this.y, this.z].map(round4); }

  static faceNormal(a, b, c) {
    return b.sub(a).cross(c.sub(a)).normalize();
  }

  static lambert(normal, toLight) {
    return round4(Math.max(0, normal.normalize().dot(toLight.normalize())));
  }
}

@@exercise Project 3D points to the screen
Write `project(point, camera, screen)` that converts a 3D point to pixel coordinates with a simple pinhole camera looking down the **negative z axis** from `camera.position` (no rotation):

1. camera space: `p = point − camera.position` (arrays `[x, y, z]`)
2. if `-p.z < camera.near` (behind or too close) → return `null`
3. focal length `f = (screen.height / 2) / tan(fov / 2)` with `camera.fov` in **degrees**
4. `sx = screen.width / 2 + f × p.x / -p.z` and `sy = screen.height / 2 − f × p.y / -p.z` (screen y grows downwards)
5. return `[round(sx), round(sy)]`

Then `wireframe(vertices, edges, camera, screen)` returning a list of line segments `[[x1, y1], [x2, y2]]` for every edge (pairs of vertex indexes) whose **both** ends project successfully.
@@starter
function project(point, camera, screen) {
  throw new Error("TODO: not done yet");
}

function wireframe(vertices, edges, camera, screen) {
  throw new Error("TODO: not done yet");
}

@@tests
const camera = { position: [0, 0, 5], fov: 90, near: 0.1 };
const screen = { width: 800, height: 600 };
check(project([0, 0, 0], camera, screen), [400, 300], "centre");
check(project([1, 1, 0], camera, screen), [460, 240], "up and right");
check(project([1, 1, -5], camera, screen), [430, 270], "twice as far looks half as big");
check(project([0, 0, 6], camera, screen), null, "behind the camera");
const cube = [[-1, -1, -1], [1, -1, -1], [1, 1, -1], [-1, 1, -1], [0, 0, 10]];
check(wireframe(cube, [[0, 1], [1, 2], [2, 3], [3, 4]], camera, screen), [[[350, 350], [450, 350]], [[450, 350], [450, 250]], [[450, 250], [350, 250]]], "edges with a hidden point skipped");
@@hint
`const [x, y, z] = point.map((v, i) => v - camera.position[i]);` then `const depth = -z; if (depth < camera.near) return null;` and `const f = screen.height / 2 / Math.tan((camera.fov * Math.PI) / 360);`.
@@solution
function project(point, camera, screen) {
  const [x, y, z] = point.map((v, i) => v - camera.position[i]);
  const depth = -z;
  if (depth < camera.near) return null;
  const f = screen.height / 2 / Math.tan((camera.fov * Math.PI) / 360);
  return [Math.round(screen.width / 2 + (f * x) / depth), Math.round(screen.height / 2 - (f * y) / depth)];
}

function wireframe(vertices, edges, camera, screen) {
  const projected = vertices.map((v) => project(v, camera, screen));
  return edges
    .filter(([a, b]) => projected[a] && projected[b])
    .map(([a, b]) => [projected[a], projected[b]]);
}
