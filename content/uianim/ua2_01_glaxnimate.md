@@lesson ua2-glaxnimate Glaxnimate and the Lottie Format
@@concepts json, maps, lists, functions
@@lang python
@@text
**Glaxnimate** (glaxnimate.org, free for Windows, macOS and Linux, a KDE project) is a small, fast vector animation editor built around **Lottie**. Think of it as "After Effects + Bodymovin" for UI animations without the price tag: draw or import SVG, animate with keyframes, and export **Lottie JSON**, **dotLottie**, **animated SVG**, Telegram stickers (`.tgs`), GIF, WebP or video.

## The interface

| Area | Use |
|---|---|
| **Canvas** | draw, select, transform; onion skin for previous/next frames |
| **Tools** | Select, Edit nodes, Draw Bézier, Freehand, Rectangle, Ellipse, Star/Polygon, Text, Fill, Eyedropper |
| **Layers** | layers, groups and **precompositions** (reusable animated pieces) |
| **Properties** | every animatable property: transform (anchor, position, scale, rotation, opacity), fill, stroke, trim path... |
| **Timeline** | keyframes per property; right-click a keyframe for easing |
| **Fill / Stroke / Gradients / Swatches** | colour editing with document swatches |

Document settings: **width × height**, **frame rate**, **in** and **out** points (first and last frame). For UI, 60 fps and a duration between 0.3 and 2 seconds are typical.

## Animating

1. Select a property and click its **keyframe** toggle (or turn on "record" so any change creates keyframes).
2. Move the timeline cursor, change the value: a new keyframe appears.
3. Right-click a keyframe → **Easing**: Hold, Linear, Ease (in, out, in-out) or **Custom** with Bézier handles in the curve editor.
4. **Trim Path** (start, end, offset) is the classic "line draws itself" effect: animate end from 0 to 100% on a checkmark's stroke.
5. **Shape morphing**: keyframe a path's shape; keep the node count equal for clean morphs.
6. **Precomps** let you build a sparkle once and reuse it five times with different start times.
7. Use **Import** for SVG from Inkscape or Penpot (the Vector UI course): layer and group IDs become layer names, which you'll target from code.

## Lottie under the hood

Lottie started as **Bodymovin** (an After Effects plugin by Hernan Torrisi) and Airbnb's player libraries; it is now an open standard maintained by the **Lottie Animation Community** (Linux Foundation). A Lottie file is JSON:

```text
{
  "v": "5.7.0",          version
  "fr": 60,              frame rate
  "ip": 0, "op": 60,     in point and out point (frames): 60 frames = 1 s
  "w": 512, "h": 512,    size
  "nm": "like",          name
  "assets": [...],       precomps and images
  "markers": [{"cm": "hover", "tm": 0, "dr": 20}],   named segments
  "layers": [...]
}
```

Each layer has a type `ty` (0 precomp, 1 solid, 2 image, 3 null, **4 shape**, 5 text), `ind` (index), `ip`/`op`, `ks` (transform) and, for shape layers, `shapes`. The transform `ks` holds **a** (anchor), **p** (position), **s** (scale, in %), **r** (rotation, degrees) and **o** (opacity, 0–100). Each property is either static or animated:

```text
"o": {"a": 0, "k": 100}                                   static opacity 100
"s": {"a": 1, "k": [                                      animated scale
   {"t": 0,  "s": [100, 100], "o": {"x": [0.33], "y": [0]}, "i": {"x": [0.67], "y": [1]}},
   {"t": 12, "s": [120, 120], ...},
   {"t": 24, "s": [100, 100]}
]}
```

`"a": 1` means animated; `"k"` is then a list of keyframes with time `t`, start value `s`, and easing handles `o` (out) and `i` (in): the same cubic-bezier as the previous lesson, so `o.x=0.33, o.y=0, i.x=0.67, i.y=1` is a standard ease-in-out. A keyframe with `"h": 1` is a hold.

Shapes (`ty` inside `shapes`): `gr` group, `rc` rectangle, `el` ellipse, `sr` star, `sh` path, `fl` fill, `st` stroke, `gf`/`gs` gradient fill/stroke, `tr` group transform, `tm` trim path, `rp` repeater, `rd` round corners. Colours are **0–1 floats**: `[1, 0.27, 0.4, 1]` is RGBA.

**Markers** name segments of the timeline (`"cm"` name, `"tm"` start frame, `"dr"` duration). Players can play a marker by name, which is how one file holds `hover`, `press` and `success` states.

## Exporting well

- **Lottie JSON**: universal; minify it (Glaxnimate does) and keep it under ~50 KB for UI.
- **dotLottie** (`.lottie`): a zip containing `manifest.json` and one or more animations plus images; smaller and can hold themes and **state machines** in the newer spec.
- **Animated SVG**: no player needed in browsers; fine for simple loops.
- Avoid things players can't render everywhere: expressions, some effects, huge embedded images, text layers with unusual fonts (convert text to paths).
- Test in the **LottieFiles** preview or `lottie-web` before shipping; check the **frame rate** and the **in/out points**.

## Telegram stickers and more

Glaxnimate also exports `.tgs` (gzipped Lottie, 512 × 512, 60 fps, max 3 s) for animated Telegram stickers, a fun way to practise.

@@quiz
In a Lottie file, what do `ip` and `op` mean?
- Image path and output path
* The in point and out point (first and last frame) of the animation
- Input and output ports
- Index and opacity
? Duration in seconds = (op − ip) / fr.

@@quiz
What does `"a": 1` mean on a Lottie property?
- The layer is active
* The property is animated and `k` holds keyframes
- Alpha is 1
- Anchor is set
? `"a": 0` means `k` is a static value.

@@quiz
How are colours written in Lottie?
- Hex strings like "#ff4466"
* Arrays of 0–1 floats, e.g. [1, 0.27, 0.4, 1]
- 0–255 integers
- HSL strings
? Divide 0–255 values by 255.

@@quiz
How can one Lottie file hold "hover", "press" and "success" animations?
- It can't; use three files
* Named markers (segments) on the timeline, played by name from code
- Three layers with different colours
- By setting op to 3
? Markers have a name, start frame and duration.

@@quiz
Which Glaxnimate/Lottie feature makes a checkmark "draw itself"?
- A blur effect
* Trim Path, animating the stroke's end from 0 to 100%
- A precomp
- Opacity
? The stroke reveals along its path.

@@flashcards
Glaxnimate :: open-source vector animation editor for Lottie, SVG and .tgs
Bodymovin :: the After Effects exporter Lottie came from
v / fr / ip / op / w / h :: Lottie version, frame rate, in, out, width, height
ty 4 :: shape layer
ks :: layer transform: a, p, s, r, o
"a": 0 vs "a": 1 :: static vs animated property
keyframe t / s / i / o / h :: time, value, in handle, out handle, hold
gr / rc / el / sh / fl / st / tm :: group, rect, ellipse, path, fill, stroke, trim
Lottie colours :: RGBA floats 0–1
markers :: named segments: cm, tm, dr
dotLottie :: zipped Lottie with manifest, images, themes, state machines
.tgs :: gzipped Lottie Telegram sticker
precomp :: reusable animated composition

@@exercise Build a Lottie file
Write `hex_to_lottie(hex)` turning `"#FF4466"` into `[1.0, 0.267, 0.4, 1]` (RGB divided by 255, rounded to 3 decimals, alpha 1). Write `kf(t, value)` returning a keyframe dict `{"t": t, "s": value, "o": {"x": [0.33], "y": [0]}, "i": {"x": [0.67], "y": [1]}}` where a number value becomes a one-item list. Then `pulse_lottie(color, fps, seconds, size)` returning a Lottie dict: `v` "5.7.0", `fr` fps, `ip` 0, `op` fps × seconds (int), `w` and `h` size, `nm` "pulse", and one shape layer (`ty` 4, `ind` 1, `ip` 0, `op` like the file, `nm` "dot") whose `ks` has anchor `[0, 0]`, position at the centre, opacity static 100, rotation static 0, and scale animated 100 → 130 → 100 at frames 0, op/2 (int) and op. Its `shapes` hold one group: an ellipse (`ty` "el", `p` [0,0], `s` [size/2, size/2]) and a fill (`ty` "fl", `c` the colour, `o` 100). Static properties use `{"a": 0, "k": value}`.
@@starter
import json


def hex_to_lottie(hex):
    pass


def kf(t, value):
    pass


def pulse_lottie(color, fps, seconds, size):
    pass


anim = pulse_lottie("#FF4466", 60, 1, 128)
print(json.dumps(anim)[:200])
@@tests
check(hex_to_lottie("#FF4466"), [1.0, 0.267, 0.4, 1], "hex to Lottie colour")
check(hex_to_lottie("000000"), [0.0, 0.0, 0.0, 1], "black, without #")
check(kf(12, 120)["s"], [120], "numbers become lists")
check(kf(0, [100, 100])["i"], {"x": [0.67], "y": [1]}, "ease-in-out handles")
a = pulse_lottie("#FF4466", 60, 1, 128)
check((a["fr"], a["ip"], a["op"], a["w"], a["h"]), (60, 0, 60, 128, 128), "header")
layer = a["layers"][0]
check(layer["ty"], 4, "a shape layer")
check(layer["ks"]["p"], {"a": 0, "k": [64, 64]}, "centred")
check(layer["ks"]["o"], {"a": 0, "k": 100}, "static opacity")
scale = layer["ks"]["s"]
check(scale["a"], 1, "scale is animated")
check([k["t"] for k in scale["k"]], [0, 30, 60], "keyframe times")
check([k["s"] for k in scale["k"]], [[100, 100], [130, 130], [100, 100]], "keyframe values")
group = layer["shapes"][0]
check([s["ty"] for s in group["it"]], ["el", "fl"], "ellipse then fill")
check(group["it"][1]["c"], {"a": 0, "k": [1.0, 0.267, 0.4, 1]}, "fill colour")
import json
check(json.loads(json.dumps(a)) == a, True, "valid JSON round trip")
@@hint
Strip the `#` with `hex.lstrip("#")`, then `int(h[i:i+2], 16) / 255` for i in 0, 2, 4. A group shape is `{"ty": "gr", "it": [ellipse, fill]}`. Remember: static is `{"a": 0, "k": value}`, animated is `{"a": 1, "k": [kf(...), ...]}`.
@@solution
import json


def hex_to_lottie(hex):
    h = hex.lstrip("#")
    return [round(int(h[i:i + 2], 16) / 255, 3) for i in (0, 2, 4)] + [1]


def kf(t, value):
    if not isinstance(value, list):
        value = [value]
    return {"t": t, "s": value, "o": {"x": [0.33], "y": [0]}, "i": {"x": [0.67], "y": [1]}}


def static(value):
    return {"a": 0, "k": value}


def pulse_lottie(color, fps, seconds, size):
    op = int(fps * seconds)
    ks = {
        "a": static([0, 0]),
        "p": static([size // 2, size // 2]),
        "s": {"a": 1, "k": [kf(0, [100, 100]), kf(op // 2, [130, 130]), kf(op, [100, 100])]},
        "r": static(0),
        "o": static(100),
    }
    group = {"ty": "gr", "it": [
        {"ty": "el", "p": static([0, 0]), "s": static([size / 2, size / 2])},
        {"ty": "fl", "c": static(hex_to_lottie(color)), "o": static(100)},
    ]}
    layer = {"ty": 4, "ind": 1, "ip": 0, "op": op, "nm": "dot", "ks": ks, "shapes": [group]}
    return {"v": "5.7.0", "fr": fps, "ip": 0, "op": op, "w": size, "h": size, "nm": "pulse", "layers": [layer]}


anim = pulse_lottie("#FF4466", 60, 1, 128)
print(json.dumps(anim)[:200])

@@exercise Inspect and validate a Lottie file
Write `summary(text)` that parses Lottie JSON text and returns a dict: `duration` (seconds, `(op - ip) / fr` rounded to 2), `layers` (number of layers), `animated` (sorted list of `"layer name.prop"` for each transform property in `ks` with `"a": 1`), and `markers` (dict of marker name → `(start, end)` frames, end = tm + dr). Then `problems(text)` returns a list of strings: `"bad json"` if it can't be parsed (and nothing else); `"missing <key>"` for each of `v, fr, ip, op, w, h, layers` that's absent; `"op must be after ip"`; and `"layer <name> outside timeline"` when a layer's `ip` is ≥ the file's `op`.
@@starter
import json


def summary(text):
    pass


def problems(text):
    pass


doc = json.dumps({
    "v": "5.7.0", "fr": 60, "ip": 0, "op": 90, "w": 256, "h": 256,
    "markers": [{"cm": "hover", "tm": 0, "dr": 30}, {"cm": "press", "tm": 30, "dr": 60}],
    "layers": [
        {"nm": "heart", "ip": 0, "op": 90, "ks": {"s": {"a": 1, "k": []}, "o": {"a": 0, "k": 100}, "r": {"a": 1, "k": []}}},
        {"nm": "sparkles", "ip": 20, "op": 90, "ks": {"o": {"a": 1, "k": []}}},
    ],
})
print(summary(doc))
print(problems(doc))
@@tests
doc = json.dumps({
    "v": "5.7.0", "fr": 60, "ip": 0, "op": 90, "w": 256, "h": 256,
    "markers": [{"cm": "hover", "tm": 0, "dr": 30}, {"cm": "press", "tm": 30, "dr": 60}],
    "layers": [
        {"nm": "heart", "ip": 0, "op": 90, "ks": {"s": {"a": 1, "k": []}, "o": {"a": 0, "k": 100}, "r": {"a": 1, "k": []}}},
        {"nm": "sparkles", "ip": 20, "op": 90, "ks": {"o": {"a": 1, "k": []}}},
    ],
})
s = summary(doc)
check(s["duration"], 1.5, "90 frames at 60 fps")
check(s["layers"], 2, "two layers")
check(s["animated"], ["heart.r", "heart.s", "sparkles.o"], "animated properties")
check(s["markers"], {"hover": (0, 30), "press": (30, 90)}, "markers")
check(problems(doc), [], "a valid file")
check(problems("{not json"), ["bad json"], "broken JSON")
bad = json.dumps({"v": "5", "fr": 30, "ip": 10, "op": 10, "w": 1, "layers": [{"nm": "late", "ip": 12, "ks": {}}]})
check(problems(bad), ["missing h", "op must be after ip", "layer late outside timeline"], "several problems")
check(summary(json.dumps({"fr": 30, "ip": 0, "op": 45, "layers": []}))["markers"], {}, "no markers")
@@hint
`json.loads` raises `json.JSONDecodeError` (a `ValueError`) on bad text. Use `.get("markers", [])` and `.get("ks", {})`. Build the problem list in the order: missing keys, timeline, layers.
@@solution
import json


def summary(text):
    d = json.loads(text)
    animated = []
    for layer in d.get("layers", []):
        for prop, val in layer.get("ks", {}).items():
            if isinstance(val, dict) and val.get("a") == 1:
                animated.append(layer.get("nm", "?") + "." + prop)
    markers = {m["cm"]: (m["tm"], m["tm"] + m["dr"]) for m in d.get("markers", [])}
    return {
        "duration": round((d["op"] - d["ip"]) / d["fr"], 2),
        "layers": len(d.get("layers", [])),
        "animated": sorted(animated),
        "markers": markers,
    }


def problems(text):
    try:
        d = json.loads(text)
    except ValueError:
        return ["bad json"]
    out = ["missing " + k for k in ("v", "fr", "ip", "op", "w", "h", "layers") if k not in d]
    if "ip" in d and "op" in d and d["op"] <= d["ip"]:
        out.append("op must be after ip")
    for layer in d.get("layers", []):
        if "op" in d and layer.get("ip", 0) >= d["op"]:
            out.append("layer " + layer.get("nm", "?") + " outside timeline")
    return out


doc = json.dumps({
    "v": "5.7.0", "fr": 60, "ip": 0, "op": 90, "w": 256, "h": 256,
    "markers": [{"cm": "hover", "tm": 0, "dr": 30}, {"cm": "press", "tm": 30, "dr": 60}],
    "layers": [
        {"nm": "heart", "ip": 0, "op": 90, "ks": {"s": {"a": 1, "k": []}, "o": {"a": 0, "k": 100}, "r": {"a": 1, "k": []}}},
        {"nm": "sparkles", "ip": 20, "op": 90, "ks": {"o": {"a": 1, "k": []}}},
    ],
})
print(summary(doc))
print(problems(doc))
