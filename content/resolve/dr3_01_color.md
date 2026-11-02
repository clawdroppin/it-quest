@@lesson dr3-color The Color Page: Nodes, Primaries, Secondaries and LUTs
@@concepts numbers, lists, functions
@@lang python
@@text
Resolve began as a high-end colour grading system, and the **Color page** is still the industry standard for films, series and commercials. Grading happens in two stages: **correction** (make shots look right and match each other) and **grading** (create a look that serves the story).

## The Color page layout

- **Viewer** with split-screen and wipe comparisons (grab a **still** into the Gallery to compare).
- **Clips** strip (thumbnails of every timeline clip) and **Gallery** (stills, saved grades, PowerGrades shared across projects).
- **Node Editor**: the grade, built from nodes.
- **Palettes**: Color Wheels, **Primaries Bars**, Log wheels, **Curves** (custom, hue vs hue/sat/lum, lum vs sat, sat vs sat), **Color Warper**, **Qualifier**, **Power Windows**, **Tracker**, Blur/Sharpen, Key, Sizing, **Magic Mask**, Motion Effects (noise reduction, Studio), Stereo.
- **Scopes** (Workspace → Video Scopes, or the scope palette): waveform, parade, vectorscope, histogram, CIE chromaticity (Lesson 4).

## Nodes

Each node is a full set of corrections; the image flows left to right through them.

| Node | Shortcut | Use |
|---|---|---|
| **Serial** | Alt+S | one after another: the default |
| **Parallel** | Alt+P | several corrections mixed together from the same input |
| **Layer** | Alt+L | stacked; the lower node wins where they overlap, with composite modes |
| **Outside** | Alt+O | the inverse of the previous node's window/key |
| **Splitter/Combiner** | | grade R, G, B channels separately |

Label your nodes (right-click → Node Label). A common, readable tree:

1. **Input/normalise** (colour space transform or LUT from log to Rec.709, if not colour-managed)
2. **Exposure / balance** (primaries)
3. **Contrast** (curves)
4. **Saturation**
5. **Secondaries**: skin tones, sky, windows
6. **Look** (the creative grade, often a shared node)
7. **Vignette / output**

**Ctrl+D** toggles a node on and off; **Shift+D** bypasses the whole grade; **Alt+Shift+S** adds a serial node before the current one.

## Primaries: lift, gamma, gain, offset

The **Color Wheels** each have a colour balance wheel and a master wheel (the ring below):

| Control | Affects | Maths (simplified) |
|---|---|---|
| **Lift** | shadows (blacks), least in highlights | raises the low end |
| **Gamma** | midtones | bends the curve |
| **Gain** | highlights (whites), multiplies | scales the top |
| **Offset** | everything equally | adds |

A classic formulation (ASC CDL uses **slope, offset, power**): `out = (in × slope + offset) ^ power`. Lift/gamma/gain are similar: `out = (gain × (in + lift × (1 − in))) ^ (1 / gamma)`. The first exercise implements both. Primaries Bars give the same controls as RGB sliders; **Contrast/Pivot**, **Saturation**, **Hue**, **Color Boost**, **Temp/Tint** and **Midtone Detail** are on the page too.

Workflow: set black and white points with lift and gain while watching the **waveform** (blacks near 0, highlights below 100 IRE / 1023), balance neutrals with the **parade** (R, G, B aligned on things that should be grey), then adjust midtones and contrast.

## Secondaries

- **Qualifier** (HSL key): pick a colour (skin, a sky), refine hue, saturation and luminance ranges, clean with **Matte Finesse** (denoise, blur, shrink/grow). Then grade only that.
- **Power Windows**: circle, linear, polygon, curve, gradient shapes; soft edges; combine with qualifiers to isolate a face's skin. **Tracker** (Clip mode tracks the window through the shot; Frame mode keyframes).
- **Magic Mask** (Studio): AI-based person/object isolation by drawing a stroke.
- **Hue vs Sat / Hue vs Hue / Hue vs Lum curves**: push greens to teal, desaturate the reds of a sign, darken a blue sky.
- **Color Warper**: a mesh in hue/saturation space to move colours with ease.

## Matching shots

- Grab a still of the hero shot; **split screen** or wipe against the current shot.
- Match black levels, then whites, then midtones, using parades.
- **Shot Match** (right-click a clip → Shot Match to This Clip) gets you close automatically.
- **Copy grades**: middle-click a clip thumbnail to apply its grade to the current one; **Ctrl+Shift+V** pastes; append node graphs (Gallery right-click → Append Node Graph).
- **Groups** (right-click → Add into a New Group): Pre-Clip and Post-Clip node trees apply to every clip in the group (all shots from one camera).
- **Timeline grade** (node mode dropdown → Timeline) applies a look to everything at once.

## LUTs

A **LUT** (look-up table) maps input colours to output colours.

- **1D LUT**: a curve per channel (contrast, gamma conversions).
- **3D LUT**: a cube (17³, 33³ or 65³ points) mapping RGB triplets, with trilinear or tetrahedral interpolation in between: colour space conversions (camera log → Rec.709) and creative looks. Stored as `.cube` text files.
- Apply on a node (right-click → LUT) or as the **timeline output LUT** (Project Settings → Color Management) or with the **Color Space Transform** Resolve FX, which is more accurate than a LUT.
- **Generate LUT** from a grade (right-click a clip → Generate LUT → 33 Point Cube) to give the look to a camera monitor or another program.
- Never grade *after* a creative LUT that crushes highlights: put the LUT late in the node tree and correct before it.

The second exercise parses a `.cube` file and applies a 1D LUT with interpolation.

@@quiz
Which primary control mainly affects the shadows?
* Lift
- Gain
- Gamma
- Offset
? Gain affects highlights most, gamma the midtones, offset everything.

@@quiz
Which node type lets you grade a key and its inverse separately?
- Parallel node
* An Outside node after the node with the window or qualifier
- Splitter node
- A LUT
? The Outside node uses the inverse of the previous node's key.

@@quiz
Which scope do you use to balance neutrals so R, G and B line up on grey objects?
- Histogram
* RGB Parade
- CIE chromaticity
- Audio meters
? Equal heights on grey surfaces mean no colour cast.

@@quiz
What is a 3D LUT?
- A 3D model's texture
* A table mapping each RGB input (on a cube grid like 33³) to an output colour, interpolated in between
- A curve per channel
- A camera lens profile
? .cube files store them as text.

@@quiz
How do you apply one grade to all the shots from the same camera at once?
- Copy and paste 500 times
* Put them in a group and grade in the Pre-Clip (or Post-Clip) node tree
- Use the Deliver page
- Change the project frame rate
? Group grades sit before or after each clip's own grade.

@@flashcards
correction vs grading :: make right and matching vs create a look
serial / parallel / layer / outside nodes :: Alt+S / Alt+P / Alt+L / Alt+O
lift / gamma / gain / offset :: shadows / mids / highlights / everything
ASC CDL :: slope, offset, power (+ saturation)
Contrast and Pivot :: contrast around a chosen grey level
qualifier :: HSL key isolating a colour range
Power Window :: shape mask, trackable
Magic Mask :: AI object/person isolation (Studio)
Hue vs Sat / Hue vs Hue curves :: adjust colours by hue
Color Warper :: mesh to move colours in hue/sat space
Shot Match :: automatic match to a reference clip
groups (Pre-Clip / Post-Clip) :: grades shared by a set of clips
Timeline grade :: node tree applied to everything
1D LUT / 3D LUT :: per-channel curve / RGB cube mapping
.cube :: text LUT format
Color Space Transform :: accurate colour space conversion effect
PowerGrade :: saved grade available in all projects

@@exercise Lift, gamma, gain and CDL
Values are 0–1 floats. Write `lgg(value, lift, gamma, gain)`: `out = (gain * (value + lift * (1 - value))) ** (1 / gamma)`, clamped to 0–1, rounded to 4. Write `cdl(value, slope, offset, power)`: `out = max(0, value * slope + offset) ** power`, clamped to 0–1, rounded to 4. Then `grade_rgb(rgb, slopes, offsets, powers, saturation)`: apply CDL per channel, then the CDL saturation step: luma `l = 0.2126 r + 0.7152 g + 0.0722 b`, each channel becomes `l + saturation * (c - l)`, clamped to 0–1, rounded to 4.
@@starter
def lgg(value, lift, gamma, gain):
    pass


def cdl(value, slope, offset, power):
    pass


def grade_rgb(rgb, slopes, offsets, powers, saturation):
    pass


print([lgg(v, 0.05, 1.2, 0.95) for v in (0, 0.18, 0.5, 1)])
print(grade_rgb((0.4, 0.3, 0.2), (1.1, 1.0, 0.9), (0, 0, 0.02), (1, 1, 1), 1.2))
@@tests
check(lgg(0.5, 0, 1, 1), 0.5, "neutral controls change nothing")
check(lgg(0, 0.1, 1, 1), 0.1, "lift raises black")
check(lgg(1, 0.1, 1, 1), 1.0, "lift leaves white")
check(lgg(1, 0, 1, 0.8), 0.8, "gain lowers white")
check(lgg(0.25, 0, 2, 1), 0.5, "gamma 2 brightens mids")
check(lgg(0.9, 0, 1, 1.5), 1.0, "clamped")
check(cdl(0.5, 1, 0, 1), 0.5, "identity CDL")
check(cdl(0.5, 1.2, -0.1, 1), 0.5, "slope and offset")
check(cdl(0.04, 1, -0.1, 1), 0.0, "never negative")
check(cdl(0.25, 1, 0, 0.5), 0.5, "power")
check(grade_rgb((0.5, 0.5, 0.5), (1, 1, 1), (0, 0, 0), (1, 1, 1), 2.0), (0.5, 0.5, 0.5), "saturation doesn't change grey")
check(grade_rgb((0.6, 0.4, 0.4), (1, 1, 1), (0, 0, 0), (1, 1, 1), 0.0), (0.4425, 0.4425, 0.4425), "zero saturation is luma")
check(grade_rgb((0.6, 0.4, 0.4), (1, 1, 1), (0, 0, 0), (1, 1, 1), 2.0), (0.7575, 0.3575, 0.3575), "double saturation")
@@hint
Clamp with `max(0.0, min(1.0, x))`. Python's `**` with a float exponent on a negative base gives a complex number, which is why CDL clamps before the power.
@@solution
def clamp(x):
    return max(0.0, min(1.0, x))


def lgg(value, lift, gamma, gain):
    base = gain * (value + lift * (1 - value))
    return round(clamp(max(0.0, base) ** (1 / gamma)), 4)


def cdl(value, slope, offset, power):
    return round(clamp(max(0.0, value * slope + offset) ** power), 4)


def grade_rgb(rgb, slopes, offsets, powers, saturation):
    c = [max(0.0, v * s + o) ** p for v, s, o, p in zip(rgb, slopes, offsets, powers)]
    l = 0.2126 * c[0] + 0.7152 * c[1] + 0.0722 * c[2]
    return tuple(round(clamp(l + saturation * (x - l)), 4) for x in c)


print([lgg(v, 0.05, 1.2, 0.95) for v in (0, 0.18, 0.5, 1)])
print(grade_rgb((0.4, 0.3, 0.2), (1.1, 1.0, 0.9), (0, 0, 0.02), (1, 1, 1), 1.2))

@@exercise Read and apply a LUT
A `.cube` file has keyword lines (`TITLE "..."`, `LUT_1D_SIZE n` or `LUT_3D_SIZE n`, `DOMAIN_MIN`, `DOMAIN_MAX`), comments starting with `#`, and data lines of three floats. Write `parse_cube(text)` returning `{"title": str or None, "kind": "1D" or "3D", "size": n, "table": [(r, g, b), ...]}`, and raise `ValueError` if the number of data rows isn't `n` (1D) or `n³` (3D). Then `apply_1d(table, rgb)`: for each channel, its value 0–1 is positioned at `v * (n - 1)` in the table; interpolate linearly between the two nearest rows using that channel's column; clamp input to 0–1; round to 4.
@@starter
def parse_cube(text):
    pass


def apply_1d(table, rgb):
    pass


CUBE = """# a gentle S-curve with warm highlights
TITLE "Warm S"
LUT_1D_SIZE 5
0.0 0.0 0.0
0.18 0.17 0.15
0.5 0.5 0.48
0.84 0.82 0.78
1.0 0.98 0.94
"""
lut = parse_cube(CUBE)
print(lut["title"], lut["kind"], lut["size"])
print(apply_1d(lut["table"], (0.5, 0.5, 0.5)), apply_1d(lut["table"], (0.1, 0.6, 0.9)))
@@tests
CUBE = """# a gentle S-curve with warm highlights
TITLE "Warm S"
LUT_1D_SIZE 5
0.0 0.0 0.0
0.18 0.17 0.15
0.5 0.5 0.48
0.84 0.82 0.78
1.0 0.98 0.94
"""
lut = parse_cube(CUBE)
check((lut["title"], lut["kind"], lut["size"], len(lut["table"])), ("Warm S", "1D", 5, 5), "header")
check(lut["table"][1], (0.18, 0.17, 0.15), "a row")
check(apply_1d(lut["table"], (0.5, 0.5, 0.5)), (0.5, 0.5, 0.48), "on a row")
check(apply_1d(lut["table"], (0.125, 0.125, 0.125)), (0.09, 0.085, 0.075), "between rows 0 and 1")
check(apply_1d(lut["table"], (0.1, 0.6, 0.9)), (0.072, 0.628, 0.876), "per channel")
check(apply_1d(lut["table"], (-1, 2, 1)), (0.0, 0.98, 0.94), "clamped input")
cube3 = "LUT_3D_SIZE 2\n" + "\n".join("%d %d %d" % (r, g, b) for b in (0, 1) for g in (0, 1) for r in (0, 1))
c3 = parse_cube(cube3)
check((c3["kind"], c3["size"], len(c3["table"]), c3["title"]), ("3D", 2, 8, None), "a 3D identity cube")
try:
    parse_cube("LUT_1D_SIZE 4\n0 0 0\n1 1 1\n")
    check("no error", "ValueError", "wrong row count")
except ValueError:
    check("ValueError", "ValueError", "wrong row count")
@@hint
For each line: strip it; skip blanks and `#`. If it starts with a letter, split into keyword and the rest. Otherwise `tuple(float(x) for x in line.split())`. For the 1D lookup: `pos = v * (n - 1)`, `i = min(int(pos), n - 2)`, `t = pos - i`.
@@solution
def parse_cube(text):
    title, kind, size, table = None, None, None, []
    for line in text.splitlines():
        line = line.strip()
        if not line or line.startswith("#"):
            continue
        if line[0].isalpha():
            key, _, rest = line.partition(" ")
            if key == "TITLE":
                title = rest.strip().strip('"')
            elif key == "LUT_1D_SIZE":
                kind, size = "1D", int(rest)
            elif key == "LUT_3D_SIZE":
                kind, size = "3D", int(rest)
            continue
        table.append(tuple(float(x) for x in line.split()))
    expected = size if kind == "1D" else size ** 3
    if len(table) != expected:
        raise ValueError(f"expected {expected} rows, found {len(table)}")
    return {"title": title, "kind": kind, "size": size, "table": table}


def apply_1d(table, rgb):
    n = len(table)
    out = []
    for ch, v in enumerate(rgb):
        v = max(0.0, min(1.0, v))
        pos = v * (n - 1)
        i = min(int(pos), n - 2)
        t = pos - i
        a, b = table[i][ch], table[i + 1][ch]
        out.append(round(a + (b - a) * t, 4))
    return tuple(out)


CUBE = """# a gentle S-curve with warm highlights
TITLE "Warm S"
LUT_1D_SIZE 5
0.0 0.0 0.0
0.18 0.17 0.15
0.5 0.5 0.48
0.84 0.82 0.78
1.0 0.98 0.94
"""
lut = parse_cube(CUBE)
print(lut["title"], lut["kind"], lut["size"])
print(apply_1d(lut["table"], (0.5, 0.5, 0.5)), apply_1d(lut["table"], (0.1, 0.6, 0.9)))
