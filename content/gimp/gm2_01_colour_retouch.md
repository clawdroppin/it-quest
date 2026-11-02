@@lesson gm2-colour Colour Correction and Photo Retouching
@@concepts numbers, lists, functions
@@lang python
@@text
Most photo editing is two jobs: making the **colours and tones** right (too dark, too flat, too blue) and **retouching** (removing dust, blemishes, stray people and power lines). GIMP's **Colors** menu handles the first; the healing and clone tools the second.

## Read the histogram first

**Windows → Dockable Dialogs → Histogram** (or inside Levels) shows how many pixels have each brightness from black (left) to white (right):

- bunched on the left: underexposed (dark); on the right: overexposed;
- a gap at both ends: low contrast, flat: there's room to stretch;
- a tall spike at an edge: **clipping**: detail lost to pure black or white.

## The Colors menu

| Tool | Use it for |
|---|---|
| **Levels** | set the black point, white point and midtone (gamma) per channel: the first fix for most photos |
| **Curves** | precise tonal control: an S-curve adds contrast; per-channel curves fix colour casts |
| **Brightness-Contrast** | quick and crude |
| **Exposure** | like changing the camera exposure, in stops |
| **Shadows-Highlights** | lift dark areas and recover bright ones separately |
| **Color Balance** | shift shadows, midtones and highlights towards colours |
| **Color Temperature** | warm (yellow) or cool (blue) |
| **Hue-Saturation** | change hue, lightness, saturation, per colour range (just the blues, say) |
| **Hue-Chroma** | perceptual version of the above |
| **White Balance** / Auto → Stretch Contrast | automatic fixes, good starting points |
| **Desaturate** | to black and white: Luminance, Lightness, Average, or **Mono Mixer** for control |
| **Colorize** | tint a greyscale image |
| **Threshold** / Posterize | pure black and white / few tones |
| **Color to Alpha** | turn a colour transparent (white background → transparency) |

Use them **on a duplicate layer** or as **non-destructive filters** (GIMP 3), and compare with the eye icon. The Levels dialog's **eyedroppers** set the black, grey and white points by clicking the image: clicking something that should be neutral grey removes a colour cast in one go.

### What Levels does to a pixel

```text
n = (value − input_black) / (input_white − input_black)     clamp to 0..1
n = n ^ (1 / gamma)                                         gamma > 1 brightens the midtones
out = output_black + n × (output_white − output_black)
```

The first exercise implements this, plus the automatic **stretch contrast**.

### Black and white

Converting to grey is a weighted sum: the eye is most sensitive to green and least to blue, so **Luminance** uses `0.2126 R + 0.7152 G + 0.0722 B`. **Lightness** uses `(max + min) / 2`, **Average** `(R + G + B) / 3`. The same red rose can turn light or dark grey depending on the method; the **Mono Mixer** lets you choose the weights like a colour filter on black-and-white film. The second exercise compares them.

### Hue and saturation

**HSV** describes a colour as **hue** (0–360° around the colour wheel: red 0°, yellow 60°, green 120°, cyan 180°, blue 240°, magenta 300°), **saturation** (grey to vivid) and **value** (dark to bright). Hue-Saturation rotates hue and scales saturation, which is why +180° turns orange into sky blue. The third exercise uses Python's `colorsys` to do it.

## Retouching

| Tool | Key | How |
|---|---|---|
| **Heal** | H | **Ctrl+click** a clean source area, then paint over the blemish: it copies texture and blends the colour |
| **Clone** | C | Ctrl+click a source, paint: an exact copy, for patterns and edges |
| **Perspective Clone** | | clone along a perspective plane (buildings, floors) |
| **Smudge / Blur / Sharpen** | S / Shift+U | local touch-ups |
| **Dodge / Burn** | Shift+D | lighten / darken areas by painting |
| **Filters → Enhance → Heal Selection** (Resynthesizer plug-in) | | content-aware removal of a selected object |

Retouching tips:

- Work on a **new transparent layer** with "Sample merged" ticked in the tool options, so the original stays untouched.
- Use a **soft brush** a little bigger than the spot, many small strokes, and change the source point often so no repeating pattern appears.
- Zoom to 100% (key 1) to check; zoom out to judge the whole.
- **Red eye**: Filters → Enhance → Red Eye Removal on a selection of the eyes.
- **Noise**: Filters → Enhance → Noise Reduction; **sharpening last**: Filters → Enhance → Sharpen (Unsharp Mask), and only for the final size.
- **Straighten**: the Measure tool (Shift+M) along the horizon → Straighten button; **Crop** with guides (rule of thirds in the crop tool options).

A typical photo workflow: crop and straighten → levels/curves → colour (white balance, saturation) → retouch → noise reduction → resize for the destination → sharpen → export.

@@quiz
The histogram is bunched in the middle with empty space at both ends. What does that mean, and what fixes it?
* The photo is flat (low contrast); move the Levels black and white input points to the ends of the data (or Stretch Contrast)
- It's overexposed; lower the exposure
- It's clipped; nothing can fix it
- It's in Indexed mode
? Stretching maps the darkest real value to black and the brightest to white.

@@quiz
What does raising the Levels gamma (the middle input slider moved left) do?
- Darkens the whole image
* Brightens the midtones while keeping black and white where they are
- Removes colour
- Clips the highlights
? out = in ^ (1 / gamma), with gamma > 1.

@@quiz
A photo has a blue cast from shade. What's the quickest precise fix?
- Desaturate it
* Levels or Curves: click the grey-point eyedropper on something that should be neutral grey (or use Color Temperature)
- Raise the brightness
- Posterize
? Neutral areas tell GIMP what "no colour" looks like.

@@quiz
Heal versus Clone: which do you use to remove a pimple on skin?
* Heal: it copies texture and blends with the surrounding colour
- Clone: it blends colours automatically
- Smudge
- Dodge
? Clone copies pixels exactly, best for edges and patterns.

@@quiz
Why convert to black and white with Luminance (or Mono Mixer) rather than Average?
- Average is not available in GIMP
* Luminance weights green most and blue least, matching how bright colours look to the eye
- Luminance is faster
- Average makes colour images
? Different weights give different moods, like colour filters on film.

@@flashcards
histogram :: count of pixels per brightness, black left, white right
clipping :: detail lost to pure black or white
Levels :: black point, white point, gamma
gamma > 1 :: brighter midtones
Curves S-shape :: more contrast
grey-point eyedropper :: remove a colour cast
Stretch Contrast :: auto levels to the full range
Luminance :: 0.2126 R + 0.7152 G + 0.0722 B
Mono Mixer :: black and white with your own channel weights
HSV :: hue, saturation, value
hue 0 / 120 / 240 :: red / green / blue
Color to Alpha :: turn a colour into transparency
Heal (H) :: Ctrl+click source, paint; blends texture and colour
Clone (C) :: exact copy from a source point
Sample merged :: retouch on a new layer using all layers below
Dodge / Burn :: lighten / darken by painting
Unsharp Mask :: sharpening, done last at final size
Measure tool → Straighten :: level a tilted horizon

@@exercise Levels and auto stretch
Write `levels(value, in_black=0, in_white=255, gamma=1.0, out_black=0, out_white=255)` for one channel value (formula in the lesson; clamp to 0–1 before the gamma; return a rounded int). Then `stretch(values)`: map the smallest value to 0 and the largest to 255 with `levels`, returning the new list; if all values are equal, return them unchanged.
@@starter
def levels(value, in_black=0, in_white=255, gamma=1.0, out_black=0, out_white=255):
    pass


def stretch(values):
    pass


print(levels(100, in_black=50, in_white=200))
print(levels(64, gamma=2.0))
print(stretch([50, 100, 150]))
@@tests
check(levels(128), 128, "no change")
check(levels(100, in_black=50, in_white=200), 85, "a flat photo, stretched")
check(levels(30, in_black=50, in_white=200), 0, "below the black point clamps")
check(levels(230, in_black=50, in_white=200), 255, "above the white point clamps")
check(levels(64, gamma=2.0), 128, "gamma brightens midtones")
check(levels(255, out_black=20, out_white=235), 235, "output range")
check(stretch([50, 100, 150]), [0, 128, 255], "stretch")
check(stretch([7, 7]), [7, 7], "flat stays flat")
@@hint
`n = (value - in_black) / (in_white - in_black)`, then `n = max(0.0, min(1.0, n))`, `n = n ** (1 / gamma)`, `return round(out_black + n * (out_white - out_black))`. For `stretch`, `lo, hi = min(values), max(values)` and call `levels(v, lo, hi)`.
@@solution
def levels(value, in_black=0, in_white=255, gamma=1.0, out_black=0, out_white=255):
    n = (value - in_black) / (in_white - in_black)
    n = max(0.0, min(1.0, n))
    n = n ** (1 / gamma)
    return round(out_black + n * (out_white - out_black))


def stretch(values):
    lo, hi = min(values), max(values)
    if lo == hi:
        return list(values)
    return [levels(v, lo, hi) for v in values]


print(levels(100, in_black=50, in_white=200))
print(levels(64, gamma=2.0))
print(stretch([50, 100, 150]))

@@exercise Desaturate three ways
Write `to_grey(rgb, method)` for the methods `"luminance"` (`0.2126 R + 0.7152 G + 0.0722 B`), `"lightness"` (`(max + min) / 2`) and `"average"`, rounding to an int. Then `mono_mixer(rgb, r, g, b)`: multiply each channel by its weight, add them, and clamp the result to 0–255 (weights don't have to add up to 1; a red filter is something like `r=1.2, g=0.2, b=-0.4`).
@@starter
def to_grey(rgb, method):
    pass


def mono_mixer(rgb, r, g, b):
    pass


rose = (200, 30, 40)
leaf = (40, 160, 60)
for m in ("luminance", "lightness", "average"):
    print(f"{m:10} rose {to_grey(rose, m):3}  leaf {to_grey(leaf, m):3}")
print("red filter:", mono_mixer(rose, 1.2, 0.2, -0.4), mono_mixer(leaf, 1.2, 0.2, -0.4))
@@tests
check(to_grey((255, 128, 0), "luminance"), 146, "luminance")
check(to_grey((255, 128, 0), "lightness"), 128, "lightness")
check(to_grey((255, 128, 0), "average"), 128, "average")
check(to_grey((200, 30, 40), "luminance"), 67, "a red rose turns dark")
check(to_grey((40, 160, 60), "luminance"), 127, "a green leaf stays bright")
check(to_grey((90, 90, 90), "luminance"), 90, "grey stays grey")
check(mono_mixer((200, 30, 40), 1.2, 0.2, -0.4), 230, "a red filter lightens red")
check(mono_mixer((40, 160, 60), 1.2, 0.2, -0.4), 56, "and darkens green")
check(mono_mixer((255, 255, 255), 1, 1, 1), 255, "clamped")
check(mono_mixer((0, 0, 255), 0, 0, -1), 0, "never below 0")
@@hint
Unpack with `r, g, b = rgb`. Lightness: `(max(rgb) + min(rgb)) / 2`. For the mixer, compute the weighted sum and `max(0, min(255, round(total)))` (the weights are named `r`, `g`, `b`, so name the channels differently, like `cr, cg, cb`).
@@solution
def to_grey(rgb, method):
    r, g, b = rgb
    if method == "luminance":
        return round(0.2126 * r + 0.7152 * g + 0.0722 * b)
    if method == "lightness":
        return round((max(rgb) + min(rgb)) / 2)
    if method == "average":
        return round((r + g + b) / 3)
    raise ValueError(method)


def mono_mixer(rgb, r, g, b):
    cr, cg, cb = rgb
    total = cr * r + cg * g + cb * b
    return max(0, min(255, round(total)))


rose = (200, 30, 40)
leaf = (40, 160, 60)
for m in ("luminance", "lightness", "average"):
    print(f"{m:10} rose {to_grey(rose, m):3}  leaf {to_grey(leaf, m):3}")
print("red filter:", mono_mixer(rose, 1.2, 0.2, -0.4), mono_mixer(leaf, 1.2, 0.2, -0.4))

@@exercise Hue-Saturation
Write `hue_saturation(rgb, hue_degrees=0, saturation=1.0)`: convert to HSV with `colorsys.rgb_to_hsv` (values 0–1), rotate the hue by `hue_degrees` (wrapping around), multiply the saturation (capped at 1), convert back and round each channel. Then `hue_name(rgb)`: the name of the nearest of the six hues red (0°), yellow (60°), green (120°), cyan (180°), blue (240°), magenta (300°), or `"grey"` when the saturation is below 0.1.
@@starter
import colorsys


def hue_saturation(rgb, hue_degrees=0, saturation=1.0):
    pass


def hue_name(rgb):
    pass


orange = (255, 128, 0)
print(hue_saturation(orange, 180), hue_name(orange), "->", hue_name(hue_saturation(orange, 180)))
print(hue_saturation((200, 100, 50), saturation=0.5))
@@tests
check(hue_saturation((255, 0, 0), 120), (0, 255, 0), "red + 120 = green")
check(hue_saturation((255, 0, 0), -120), (0, 0, 255), "red - 120 = blue")
check(hue_saturation((255, 128, 0), 180), (0, 127, 255), "orange + 180 = sky blue")
check(hue_saturation((200, 100, 50), saturation=0.5), (200, 150, 125), "half saturation")
check(hue_saturation((10, 20, 30), 90), (30, 10, 30), "dark colours rotate too")
check(hue_saturation((120, 120, 120), 77, 3.0), (120, 120, 120), "grey has no hue")
check([hue_name(c) for c in [(255, 0, 0), (250, 240, 10), (0, 200, 180), (90, 0, 255), (128, 128, 130)]], ["red", "yellow", "cyan", "blue", "grey"], "names")
check(hue_name((255, 0, 30)), "red", "close to 360 wraps to red")
@@hint
`h, s, v = colorsys.rgb_to_hsv(*(c / 255 for c in rgb))`, `h = (h + hue_degrees / 360) % 1`, `s = min(1.0, s * saturation)`. For the name, the hue in degrees is `h * 360`; the nearest of the six is `round(deg / 60) % 6` as an index into the list.
@@solution
import colorsys

NAMES = ["red", "yellow", "green", "cyan", "blue", "magenta"]


def hue_saturation(rgb, hue_degrees=0, saturation=1.0):
    h, s, v = colorsys.rgb_to_hsv(*(c / 255 for c in rgb))
    h = (h + hue_degrees / 360) % 1
    s = min(1.0, s * saturation)
    return tuple(round(c * 255) for c in colorsys.hsv_to_rgb(h, s, v))


def hue_name(rgb):
    h, s, v = colorsys.rgb_to_hsv(*(c / 255 for c in rgb))
    if s < 0.1:
        return "grey"
    return NAMES[round(h * 360 / 60) % 6]


orange = (255, 128, 0)
print(hue_saturation(orange, 180), hue_name(orange), "->", hue_name(hue_saturation(orange, 180)))
print(hue_saturation((200, 100, 50), saturation=0.5))
