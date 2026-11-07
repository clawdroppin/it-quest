@@lesson bl8-vse Video Sequence Editor, Motion Graphics Projects and Python Scripting
@@concepts lists, maps, functions, automation
@@lang python
@@text
Blender can finish a whole project: the **Video Sequence Editor (VSE)** cuts footage, renders and audio together, and **Python scripting** automates anything you'd otherwise click a thousand times.

## The Video Sequence Editor

Open the **Video Editing** workspace (File → New → Video Editing). The **Sequencer** shows **channels** (horizontal tracks; higher channels sit on top) holding **strips**:

| Strip type | Use |
|---|---|
| Movie / Image / Image sequence | footage and rendered frames |
| Sound | music, voice, effects (waveforms shown) |
| **Scene** | another scene of the same file, rendered live: put your 3D shots straight on the timeline |
| Color / Text | solid colours, titles and lower thirds |
| Adjustment Layer | apply effects or modifiers to everything below |
| Meta strip | group strips (Ctrl+G) |
| Effect strips | Cross (dissolve), Gamma Cross, **Wipe**, Add, Multiply, Alpha Over, **Transform**, **Speed Control**, Glow, Gaussian Blur, Color Mix |

Editing keys: **K** split (soft cut at the playhead), **Shift+K** hard split, **G** move, **E** extend (trim), **Shift+D** duplicate, **X** delete, **Ctrl+Click** select a handle for trimming; **snapping** to strip edges and the playhead; **M** add a marker. **Strip modifiers** (sidebar) add colour balance, curves, hue correct, masks and tone mapping per strip.

Set the output in Properties → **Output**: resolution, frame rate (match your footage before you start), FFmpeg video with **H.264 in MP4** (or ProRes/DNxHR for masters), audio AAC. Render → Render Animation.

Use **proxies** (Strip → Movie Strip → Set Render Size/Proxy) for smooth playback of 4K footage. For long-form editing and serious colour grading, DaVinci Resolve (next course) is more capable; the VSE shines for assembling your Blender shots, animatics and quick edits.

## A motion graphics project from start to finish

1. **Brief and styleframes**: 2–3 still frames defining the look (colours from your app's theme, typography, lighting).
2. **Animatic**: rough timing with Grease Pencil sketches or blocky geometry, cut to music in the VSE.
3. **Build**: logos from SVG (File → Import → SVG, extrude), Geometry Nodes for procedural parts, EEVEE for speed or Cycles for realism.
4. **Animate**: strong easing in the Graph Editor, staggered timing, loops with Cycles modifiers; keep camera moves smooth (Bézier F-curves, Follow Path).
5. **Render** EXR or PNG sequences with passes; render tests at 50%.
6. **Composite**: glow, colour balance, vignette, film grain, chromatic aberration (Lens Distortion dispersion).
7. **Edit and sound**: assemble in the VSE or Resolve, add music and sound effects; sync hits to keyframes with markers.
8. **Deliver**: 1080p/4K H.264 for the web, a vertical 1080 × 1920 version for social media, a looping GIF/WebM for your app's README.

Ideas that connect with your apps: an animated app icon reveal, a product-demo video of your Tauri app with 3D device mock-ups (an image texture on a laptop screen), an animated logo sting for a YouTube channel, a looping hero background for a landing page.

## Python scripting with bpy

The **Scripting** workspace has a Python console and a Text Editor (Alt+P runs a script). Hover over any button with **Python Tooltips** enabled (Preferences → Interface) to see its API path; the **Info** editor logs every operator you run as Python code you can copy.

```text
import bpy, math, random

# Make a ring of 24 cubes with random heights, keyed to pop up one after another
bpy.ops.mesh.primitive_cube_add(size=0.3)
base = bpy.context.active_object
for i in range(24):
    a = 2 * math.pi * i / 24
    ob = base.copy()                      # linked duplicate: shares the mesh
    bpy.context.collection.objects.link(ob)
    ob.location = (3 * math.cos(a), 3 * math.sin(a), 0)
    ob.scale = (1, 1, 0.01)
    ob.keyframe_insert("scale", frame=1 + i * 2)
    ob.scale = (1, 1, random.uniform(1, 4))
    ob.keyframe_insert("scale", frame=12 + i * 2)
bpy.data.objects.remove(base)
```

Key parts of the API:

| API | What |
|---|---|
| `bpy.data` | every datablock: `objects`, `meshes`, `materials`, `scenes`, `images`, `node_groups`, `actions` |
| `bpy.context` | the current state: `active_object`, `selected_objects`, `scene`, `collection` |
| `bpy.ops` | operators (what buttons do); context-sensitive, so prefer `bpy.data` in scripts |
| `ob.keyframe_insert(path, frame=n)` | key any property |
| `scene.frame_set(n)` / `scene.render.filepath` / `bpy.ops.render.render(write_still=True)` | step frames and render |
| `mathutils` | Vector, Matrix, Euler, Quaternion, noise |
| `bmesh` | low-level mesh editing |

**Command-line rendering** (render farms, CI, overnight batches):

```text
blender -b scene.blend -o //render/frame_#### -s 1 -e 240 -a        render frames 1–240
blender -b scene.blend --python make_variants.py                   run a script headless
```

**Add-ons**: a script with a `bl_info` (or the newer `blender_manifest.toml` **extension** format, published on extensions.blender.org), operator and panel classes, and `register()`/`unregister()` functions. Turn your repetitive setup into a button in the sidebar.

## Data in, visuals out

Scripts shine for **data-driven motion graphics**: read a CSV or JSON (sales numbers, GitHub stars, app downloads) and create animated bars, or generate variants of a logo animation for every colour theme. The exercises do the planning half in plain Python: building an edit decision list for the VSE, and computing keyframe plans for a data-driven bar chart animation.

@@quiz
Which VSE strip lets you put a 3D scene from the same file directly on the timeline?
- Movie strip
* Scene strip
- Color strip
- Meta strip
? It renders the scene's camera live.

@@quiz
Which key splits a strip at the playhead in the VSE?
- S
* K
- X
- E
? Shift+K is a hard split.

@@quiz
Why prefer `bpy.data` over `bpy.ops` in scripts?
* Operators depend on the UI context (selection, active area), while bpy.data accesses and changes data directly and reliably
- bpy.ops doesn't exist
- bpy.data renders faster
- bpy.ops only works in EEVEE
? Use ops when there's no data-level equivalent, with the right context.

@@quiz
How do you render frames 1–240 of a file without opening the interface?
- It's impossible
* `blender -b scene.blend -s 1 -e 240 -a`
- `blender --gui render`
- Press F12 240 times
? -b is background mode; -a renders the animation.

@@quiz
How can you find the Python path of any button?
- Read the source code
* Enable Python Tooltips, or check the Info editor which logs operators as Python
- Use the Outliner
- It's not possible
? Then copy the call into your script.

@@drill
What does VSE stand for in Blender?
= video sequence editor
= video sequencer
? Blender's built-in video editor.

@@flashcards
VSE :: Blender's Video Sequence Editor
channels :: tracks; higher channels are on top
Scene strip :: a 3D scene placed on the timeline
Adjustment Layer strip :: effects on everything below
K / Shift+K :: soft split / hard split
strip modifiers :: per-strip colour and mask adjustments
proxies :: low-resolution copies for smooth playback
H.264 MP4 :: common web delivery format
bpy.data / bpy.context / bpy.ops :: data / current state / operators
keyframe_insert :: key any property from Python
mathutils :: Vector, Matrix, Euler, Quaternion
bmesh :: low-level mesh editing API
blender -b ... -a :: headless animation render
Alt+P :: run the script in the Text Editor
Python Tooltips :: show API paths on hover
extensions (blender_manifest.toml) :: packaged add-ons on extensions.blender.org

@@exercise Plan an edit
A strip is a dict `{"name", "channel", "start", "length"}` (frames). Write `timeline_end(strips)`: the last frame used (max of `start + length`, 0 for none). Write `overlaps(strips)`: a sorted list of `(name_a, name_b)` pairs (names sorted inside each pair) of strips on the **same channel** whose frame ranges overlap (`[start, start+length)`). Write `place_sequence(names, lengths, channel, start, gap)`: strips placed one after another with `gap` frames between them. Then `to_timecode(frame, fps)` returns `"HH:MM:SS:FF"`.
@@starter
def timeline_end(strips):
    pass


def overlaps(strips):
    pass


def place_sequence(names, lengths, channel, start, gap):
    pass


def to_timecode(frame, fps):
    pass


shots = place_sequence(["intro", "logo", "demo"], [48, 72, 120], 1, 1, 0)
music = {"name": "music", "channel": 2, "start": 1, "length": 240}
print(shots, timeline_end(shots + [music]), to_timecode(241, 24))
@@tests
shots = place_sequence(["intro", "logo", "demo"], [48, 72, 120], 1, 1, 0)
check([s["start"] for s in shots], [1, 49, 121], "back to back")
check(shots[1], {"name": "logo", "channel": 1, "start": 49, "length": 72}, "a strip")
check(timeline_end(shots), 241, "end frame")
check(timeline_end([]), 0, "empty")
gapped = place_sequence(["a", "b"], [10, 10], 3, 0, 5)
check([s["start"] for s in gapped], [0, 15], "with a gap")
music = {"name": "music", "channel": 2, "start": 1, "length": 240}
check(overlaps(shots + [music]), [], "different channels never overlap")
clash = shots + [{"name": "title", "channel": 1, "start": 100, "length": 30}]
check(overlaps(clash), [("demo", "title"), ("logo", "title")], "a title over two shots")
check(overlaps([{"name": "x", "channel": 1, "start": 0, "length": 10}, {"name": "y", "channel": 1, "start": 10, "length": 5}]), [], "touching is fine")
check(to_timecode(241, 24), "00:00:10:01", "frame 241 at 24 fps")
check(to_timecode(90000, 25), "01:00:00:00", "one hour")
@@hint
Two ranges overlap when `a_start < b_end and b_start < a_end`. For timecode: `ff = frame % fps`, `total = frame // fps`, then hours, minutes and seconds with `divmod`, formatted with `:02d`.
@@solution
def timeline_end(strips):
    return max((s["start"] + s["length"] for s in strips), default=0)


def overlaps(strips):
    out = []
    for i, a in enumerate(strips):
        for b in strips[i + 1:]:
            if a["channel"] != b["channel"]:
                continue
            if a["start"] < b["start"] + b["length"] and b["start"] < a["start"] + a["length"]:
                out.append(tuple(sorted((a["name"], b["name"]))))
    return sorted(out)


def place_sequence(names, lengths, channel, start, gap):
    out = []
    t = start
    for name, length in zip(names, lengths):
        out.append({"name": name, "channel": channel, "start": t, "length": length})
        t += length + gap
    return out


def to_timecode(frame, fps):
    ff = frame % fps
    total = frame // fps
    hh, rest = divmod(total, 3600)
    mm, ss = divmod(rest, 60)
    return f"{hh:02d}:{mm:02d}:{ss:02d}:{ff:02d}"


shots = place_sequence(["intro", "logo", "demo"], [48, 72, 120], 1, 1, 0)
music = {"name": "music", "channel": 2, "start": 1, "length": 240}
print(shots, timeline_end(shots + [music]), to_timecode(241, 24))

@@exercise A data-driven bar chart animation
You'll script an animated bar chart. Write `bar_plan(data, max_height, start, stagger, grow)`: `data` is a list of `(label, value)`; each bar's final height is `value / max(values) * max_height` (rounded to 3), and its keyframes are `(start + i * stagger, 0.0)` and `(start + i * stagger + grow, height)`. Return a list of dicts `{"label", "x", "keys"}` where `x = i * 1.5` (bar spacing). Then `sorted_plan(data, ...)` with the same arguments returns the plan for the data sorted by value, largest first. Finally `last_frame(plan)` returns the frame of the last keyframe.
@@starter
def bar_plan(data, max_height, start, stagger, grow):
    pass


def sorted_plan(data, max_height, start, stagger, grow):
    pass


def last_frame(plan):
    pass


downloads = [("Linux", 1200), ("Windows", 3000), ("macOS", 1800)]
plan = sorted_plan(downloads, 4.0, 1, 6, 18)
for bar in plan:
    print(bar)
print(last_frame(plan))
@@tests
downloads = [("Linux", 1200), ("Windows", 3000), ("macOS", 1800)]
plan = bar_plan(downloads, 4.0, 1, 6, 18)
check([b["label"] for b in plan], ["Linux", "Windows", "macOS"], "original order")
check(plan[0]["keys"], [(1, 0.0), (19, 1.6)], "first bar")
check(plan[1]["keys"], [(7, 0.0), (25, 4.0)], "the biggest bar reaches max height")
check(plan[2]["x"], 3.0, "spacing")
s = sorted_plan(downloads, 4.0, 1, 6, 18)
check([b["label"] for b in s], ["Windows", "macOS", "Linux"], "sorted largest first")
check(s[1]["keys"], [(7, 0.0), (25, 2.4)], "macOS second")
check(last_frame(s), 31, "the last bar finishes on frame 31")
check(last_frame([]), 0, "empty plan")
@@hint
Compute `top = max(v for _, v in data)` once. `sorted(data, key=lambda d: d[1], reverse=True)` sorts by value. The last keyframe is the max of each bar's `keys[-1][0]`.
@@solution
def bar_plan(data, max_height, start, stagger, grow):
    top = max(v for _, v in data)
    plan = []
    for i, (label, value) in enumerate(data):
        h = round(value / top * max_height, 3)
        s = start + i * stagger
        plan.append({"label": label, "x": i * 1.5, "keys": [(s, 0.0), (s + grow, h)]})
    return plan


def sorted_plan(data, max_height, start, stagger, grow):
    return bar_plan(sorted(data, key=lambda d: d[1], reverse=True), max_height, start, stagger, grow)


def last_frame(plan):
    return max((bar["keys"][-1][0] for bar in plan), default=0)


downloads = [("Linux", 1200), ("Windows", 3000), ("macOS", 1800)]
plan = sorted_plan(downloads, 4.0, 1, 6, 18)
for bar in plan:
    print(bar)
print(last_frame(plan))
