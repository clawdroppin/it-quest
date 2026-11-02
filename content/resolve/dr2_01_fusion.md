@@lesson dr2-fusion Fusion: Node-Based VFX and Motion Graphics
@@concepts lists, maps, functions, numbers
@@lang python
@@text
**Fusion** is a full node-based compositor built into Resolve (it also exists as standalone Fusion Studio). It was used on films like *Avatar* and *The Martian*, and it's where you build titles, motion graphics, screen replacements, keying, particle effects and 3D compositions without leaving your edit.

## Getting into Fusion

Put the playhead over a clip on the Edit page and click the **Fusion** page. You'll see **MediaIn1 → MediaOut1**: the clip in, the result out. Everything you add goes between them. To start from nothing, add a **Fusion Composition** from Effects → Effects (Edit page) and open it in Fusion.

Layout: two viewers (press **1** or **2** with a node selected to view it in the left or right viewer), the **Node Editor**, the **Inspector**, the **Keyframes** and **Spline** editors (animation curves), and the **Effects Library**.

## Nodes and flow

- Nodes have coloured inputs: **yellow** is the **background**, **green** the **foreground**, **blue** the **effect mask**.
- **Merge** is the heart of compositing: foreground over background, with blend modes (normal, screen, add, multiply...), and its own transform. Dragging one node's output onto another node's output creates a Merge automatically.
- Shift+Space opens **Select Tool** search. Common shortcuts: Ctrl+T swaps a Merge's inputs; Ctrl+K disables nodes... use the toolbar for the most common tools.
- The flow evaluates from inputs to outputs: each node needs its inputs computed first (a **topological order**: the first exercise computes it).

## Essential tools

| Category | Tools |
|---|---|
| Composite | **Merge**, Dissolve, Channel Booleans, Matte Control |
| Generators | **Background** (solid/gradient), **Text+**, FastNoise (clouds, smoke textures), Paint, **pEmitter** (particles) |
| Transform | **Transform**, Crop, Resize, Letterbox, Camera Shake, DVE |
| Masks | **Polygon**, **B-Spline**, Rectangle, Ellipse, Bitmap, Wand, Magic Mask (Studio) |
| Keying | **Delta Keyer** (the best default), Ultra Keyer, Chroma Keyer, Luma Keyer, Clean Plate |
| Tracking | **Tracker** (1–many points), **Planar Tracker** (surfaces), **Camera Tracker** (Studio: 3D camera solve) |
| Blur/Filter | Blur, Glow, Soft Glow, Defocus, Directional Blur, Erode/Dilate, Film Grain |
| Color | Color Corrector, Brightness Contrast, Color Curves, Hue Curves, File LUT |
| Warp | Corner Positioner, Grid Warp, Lens Distort, Displace |
| 3D | Shape 3D, Text 3D, Image Plane 3D, **Merge 3D**, Camera 3D, lights, **Renderer 3D**, FBX/Alembic import |
| Particles | pEmitter, pRender, pDirectionalForce, pTurbulence, pKill, pBounce |
| Effects | Shadow, Highlight, Rays, Hot Spot (lens flares), Trails |

## Keyframes, splines and modifiers

- Click the diamond next to any Inspector control to keyframe it. The **Spline Editor** shows curves: select keys and press **S** (smooth), **L** (linear), **F** (flat ease); or use **Ease In/Out** in the context menu.
- **Modifiers** (right-click a control): **Expression** (math with `time`, e.g. `sin(time/10)*0.1`), **Perturb** (noise wiggle), **Shake**, **Anim Curves** (ease an animation without keyframes), **Follower** (per-character text offsets), **Publish/Connect To** (link controls).
- **Text+** has a Follower modifier for per-character animation: characters arrive staggered with easing: the classic kinetic typography look.

## Tracking and screen replacements

1. Planar Tracker on the phone screen in the footage (Track mode, draw a polygon on the screen, track forward/back).
2. Switch the Planar Tracker to **Corner Pin** mode and connect your app screenshot or screen recording to its foreground: it's warped onto the tracked plane.
3. Merge over the plate, add a tiny blur and grain so it matches, and a reflection layer in screen mode for realism.

For text that sticks to a building, use the **Tracker** node: track a point, then **Match Move** mode, or connect a Transform's Center to the tracker's offset (right-click → Connect To → Tracker1 → Offset position).

## Green screen with Delta Keyer

1. MediaIn → **Delta Keyer**: pick the background colour with the eyedropper.
2. View the alpha (press **A** in the viewer); in the Matte tab, raise **Clean Background** and **Clean Foreground** until the background is black and the subject white without crunching edges.
3. **Garbage matte**: draw a polygon around the subject into the garbage input to remove stands and the edges of the screen.
4. Spill suppression is automatic; refine with **Replace Color**.
5. Merge over the new background; add **Light Wrap** (a blurred background screened onto the edges) and match colour.

## Motion graphics with Fusion

- **Fusion Titles** templates (Effects → Titles → Fusion Titles) are Fusion compositions: open them and change everything.
- **Macros**: select nodes → right-click → Macro → Create Macro, expose the controls you want, save to the Templates folder: your own animated lower thirds, callouts and buttons with your brand colours appear in the Effects Library.
- **Shape tools** (sEllipse, sRectangle, sStar, sMerge, sBoolean, sRender) create vector shapes with crisp edges: UI-like graphics and animated icons.
- **3D**: Text 3D with extrusion and bevel, lit in Merge 3D with a Camera 3D, rendered with Renderer 3D (or import Blender renders and FBX/Alembic cameras).
- **Particles**: pEmitter (style: bitmap/blob/point) → pTurbulence → pRender → Merge for sparkles, dust, confetti.

## Resolution independence

Fusion works in **normalised coordinates**: (0, 0) is the bottom left, (1, 1) the top right, and (0.5, 0.5) the centre, regardless of resolution. Sizes are relative to the image width. A Transform with Center (0.6, 0.5) moves the image 10% of the width to the right. The second exercise converts between pixels and Fusion coordinates.

@@quiz
What do the yellow and green inputs of a Merge node represent?
* Yellow is the background, green is the foreground
- Yellow is the mask, green is the background
- They are both masks
- Yellow is audio, green is video
? Blue inputs are effect masks.

@@quiz
Which Fusion node replaces a phone screen in moving footage?
- Delta Keyer
* Planar Tracker in Corner Pin mode
- Text+
- pEmitter
? It warps a new image onto the tracked flat surface.

@@quiz
In Fusion's coordinates, where is (0.5, 0.5)?
- The top left corner
* The centre of the image, at any resolution
- 0.5 pixels from the corner
- The bottom right corner
? (0, 0) is bottom left, (1, 1) top right.

@@quiz
How do you make a reusable animated lower third with exposed controls?
- Render it as a video
* Build it from nodes, create a Macro exposing the controls, and save it to the Templates folder
- Use the Cut page
- It isn't possible
? It then appears in the Effects Library like the built-in titles.

@@quiz
Which modifier animates text characters with a staggered offset?
- Perturb
- Shake
* Follower
- Expression
? Set the delay per character and the easing.

@@flashcards
MediaIn / MediaOut :: the clip into and out of a Fusion comp
Merge :: foreground over background with blend modes
yellow / green / blue inputs :: background / foreground / effect mask
1 / 2 (Fusion) :: view the node in the left / right viewer
Shift+Space :: Select Tool search
Text+ :: animatable text with per-character Follower
Background / FastNoise :: generators for colours and noise
Delta Keyer :: the go-to green screen keyer
garbage matte :: rough mask removing unwanted areas
Planar Tracker / Corner Pin :: track a surface and warp an image onto it
Tracker / Match Move :: point tracking to stick elements
Expression modifier :: control computed from time and maths
macro :: grouped nodes with exposed controls
Spline Editor :: animation curves
Merge 3D / Renderer 3D :: 3D scene and its render
normalised coordinates :: 0–1 across the image, centre 0.5
pEmitter / pRender :: particle generator and renderer

@@exercise Evaluate a node graph
A Fusion comp is a dict mapping each node name to the list of nodes connected to its inputs. Write `eval_order(graph, output)`: a list of the nodes needed to compute `output`, each appearing after all of its inputs (a depth-first post-order: visit inputs in the order they're listed, then the node itself; each node once). Nodes not connected to the output are skipped. Then `unused(graph, output)`: sorted names of nodes that don't contribute. Then `has_cycle(graph)`: True if any node depends on itself (directly or indirectly).
@@starter
def eval_order(graph, output):
    pass


def unused(graph, output):
    pass


def has_cycle(graph):
    pass


comp = {
    "MediaIn1": [],
    "Background1": [],
    "Text1": [],
    "Transform1": ["Text1"],
    "Merge1": ["MediaIn1", "Transform1"],
    "Glow1": ["Merge1"],
    "Merge2": ["Background1", "Glow1"],
    "MediaOut1": ["Merge2"],
    "OldBlur": ["MediaIn1"],
}
print(eval_order(comp, "MediaOut1"))
print(unused(comp, "MediaOut1"), has_cycle(comp))
@@tests
comp = {
    "MediaIn1": [],
    "Background1": [],
    "Text1": [],
    "Transform1": ["Text1"],
    "Merge1": ["MediaIn1", "Transform1"],
    "Glow1": ["Merge1"],
    "Merge2": ["Background1", "Glow1"],
    "MediaOut1": ["Merge2"],
    "OldBlur": ["MediaIn1"],
}
order = eval_order(comp, "MediaOut1")
check(order, ["Background1", "MediaIn1", "Text1", "Transform1", "Merge1", "Glow1", "Merge2", "MediaOut1"], "evaluation order")
check(order.index("Text1") < order.index("Transform1") < order.index("Merge1"), True, "inputs first")
check(unused(comp, "MediaOut1"), ["OldBlur"], "a disconnected node")
check(has_cycle(comp), False, "a valid comp")
check(has_cycle({"A": ["B"], "B": ["C"], "C": ["A"]}), True, "A needs C needs B needs A")
check(eval_order({"Out": ["X", "X"], "X": []}, "Out"), ["X", "Out"], "each node once")
@@hint
Use a recursive `visit(node)` with a `seen` set: if seen, return; mark it, visit each input, then append the node. For cycles, track the nodes on the current path (`visiting`): reaching one of them again means a cycle.
@@solution
def eval_order(graph, output):
    order, seen = [], set()

    def visit(node):
        if node in seen:
            return
        seen.add(node)
        for src in graph.get(node, []):
            visit(src)
        order.append(node)

    visit(output)
    return order


def unused(graph, output):
    used = set(eval_order(graph, output))
    return sorted(n for n in graph if n not in used)


def has_cycle(graph):
    done, visiting = set(), set()

    def visit(node):
        if node in visiting:
            return True
        if node in done:
            return False
        visiting.add(node)
        found = any(visit(src) for src in graph.get(node, []))
        visiting.discard(node)
        done.add(node)
        return found

    return any(visit(n) for n in graph)


comp = {
    "MediaIn1": [],
    "Background1": [],
    "Text1": [],
    "Transform1": ["Text1"],
    "Merge1": ["MediaIn1", "Transform1"],
    "Glow1": ["Merge1"],
    "Merge2": ["Background1", "Glow1"],
    "MediaOut1": ["Merge2"],
    "OldBlur": ["MediaIn1"],
}
print(eval_order(comp, "MediaOut1"))
print(unused(comp, "MediaOut1"), has_cycle(comp))

@@exercise Fusion coordinates and a Follower
Write `to_fusion(px, py, width, height)`: pixel coordinates (origin top left, y down) to Fusion's normalised coordinates (origin bottom left, y up), rounded to 4. Write `to_pixels(x, y, width, height)` for the reverse, rounded to 1. Then `follower_offsets(text, delay, duration, start)` returns, for each non-space character, `(char, first_frame, last_frame)`: character k (counting only non-space characters) starts animating at `start + k * delay` and finishes `duration` frames later.
@@starter
def to_fusion(px, py, width, height):
    pass


def to_pixels(x, y, width, height):
    pass


def follower_offsets(text, delay, duration, start):
    pass


print(to_fusion(960, 540, 1920, 1080), to_fusion(1728, 972, 1920, 1080), to_pixels(0.1, 0.9, 3840, 2160))
print(follower_offsets("Hi you", 2, 10, 1))
@@tests
check(to_fusion(960, 540, 1920, 1080), (0.5, 0.5), "centre")
check(to_fusion(0, 0, 1920, 1080), (0.0, 1.0), "the top left pixel is y = 1")
check(to_fusion(1728, 972, 1920, 1080), (0.9, 0.1), "a lower-right corner bug")
check(to_pixels(0.5, 0.5, 3840, 2160), (1920.0, 1080.0), "centre in 4K")
check(to_pixels(0.1, 0.9, 3840, 2160), (384.0, 216.0), "top left title safe")
check(to_pixels(*to_fusion(480, 270, 1920, 1080), 1920, 1080), (480.0, 270.0), "round trip")
check(follower_offsets("Hi you", 2, 10, 1), [("H", 1, 11), ("i", 3, 13), ("y", 5, 15), ("o", 7, 17), ("u", 9, 19)], "staggered letters")
check(follower_offsets("", 2, 10, 1), [], "no text")
@@hint
`x = px / width`, `y = 1 - py / height`. Count characters with a separate counter that only increases for non-spaces.
@@solution
def to_fusion(px, py, width, height):
    return round(px / width, 4), round(1 - py / height, 4)


def to_pixels(x, y, width, height):
    return round(x * width, 1), round((1 - y) * height, 1)


def follower_offsets(text, delay, duration, start):
    out = []
    k = 0
    for ch in text:
        if ch == " ":
            continue
        first = start + k * delay
        out.append((ch, first, first + duration))
        k += 1
    return out


print(to_fusion(960, 540, 1920, 1080), to_fusion(1728, 972, 1920, 1080), to_pixels(0.1, 0.9, 3840, 2160))
print(follower_offsets("Hi you", 2, 10, 1))
