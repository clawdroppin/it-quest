@@lesson dr3-balance Balancing, Charts and Skin Tones
@@lang python
@@text
Before creative grading comes **balancing**: neutral whites, correct exposure, and shots that match each other. Resolve gives you colour charts, auto tools and scopes; underneath they all do simple arithmetic on RGB values. This lesson covers balancing with the colour chart, matching exposure in stops, and checking skin tones on the vectorscope.

## Balance first, then look

The usual node order: **balance** (exposure, white balance, contrast) → **match** shots in a scene → **secondaries** (skin, sky) → **look** (the creative grade, often on a group or timeline node). Grading a look on unbalanced footage means every shot needs different fixes.

## White balance

- A neutral grey surface should have **R = G = B**. Pick it with the **Offset/Gain picker** or the Temp/Tint controls, or with the **Color Match** palette using a chart (X-Rite ColorChecker, Datacolor SpyderCheckr) shot under the scene's light.
- In linear light, white balancing multiplies each channel by a **gain** so the grey patch becomes neutral: `gain_r = target / R`. Applying gains to log footage is wrong; do it in a linear or scene-referred space (Resolve's colour-managed pipelines, or the RAW/Camera RAW tab for raw clips: the cleanest place).
- Keep green fixed and adjust red and blue to avoid changing brightness much.

## Exposure in stops

One **stop** doubles or halves the light: `stops = log2(new / old)` in linear values. An 18% grey card reading 0.09 linear is one stop under. In Resolve, the HDR palette's Exposure control and the Offset wheel (on log footage) work in stops-like units.

## Shot matching

Compare shots on the **waveform** and **parade** side by side (split screen or a still in the Gallery). Match blacks, mids and highlights for each channel; match the subject, not the background. *Shot Match* (right-click → Shot Match to This Clip) gets you close automatically.

## Skin tones

On the **vectorscope**, healthy skin of every complexion falls near the **skin tone indicator** line (between red and yellow, at about **123°** measured as `atan2(Cr, Cb)`, roughly 20° from the red target at 103° towards yellow at 175°). Brightness and saturation differ between people; the **hue** stays similar. Grade skin with a qualifier and a soft key, and keep the hue near the line unless the look demands otherwise.

@@quiz
What should a neutral grey patch read after white balancing?
* Equal R, G and B
- R higher than B
- G at zero
- Any values
? Neutral means equal channels.

@@quiz
A grey card reads 0.36 linear where 0.18 is expected. How far over is it?
- Half a stop
* One stop
- Two stops
- 18 stops
? log2(0.36 / 0.18) = 1.

@@quiz
Where is the cleanest place to fix white balance on raw footage?
- A look node at the end
* The Camera RAW settings
- The Deliver page
- Fairlight
? Before the image is debayered into a display space.

@@quiz
What stays similar for skin tones across complexions on the vectorscope?
- Saturation
- Brightness
* Hue (near the skin tone line)
- Luminance
? They differ in saturation and brightness.

@@quiz
In which order should you usually grade?
- Look, then balance
* Balance, match, secondaries, then look
- Secondaries first
- Only the look
? Build on a neutral base.

@@flashcards
balance :: exposure, white balance, contrast
grey patch :: R = G = B
white balance gain :: target / channel
apply gains :: in linear light
stop :: doubling of light
stops :: log2(new / old)
18% grey :: middle grey reference
parade :: R, G, B waveforms side by side
Shot Match :: automatic matching
Color Match :: chart-based balancing
skin tone line :: about 123° on the vectorscope
qualifier :: select a colour range
node order :: balance, match, secondaries, look

@@drill
How many stops brighter is a value four times larger in linear light?
= 2
= two
? log2(4).

@@drill
Which Resolve scope shows the hue and saturation of every pixel?
= vectorscope
= the vectorscope
? Skin tones line up on its indicator.

@@exercise Balance from a chart
Each clip has linear RGB readings of the chart's grey patch (expected 0.18) and a white patch. Compute per-channel gains that make the grey neutral at 0.18, the exposure correction in stops (from the green channel), and check that the white patch becomes neutral after the gains. Flag clips needing more than 1 stop of correction or a gain outside 0.5–2.
@@starter
import math

clips = {
    "A001 daylight": {"grey": (0.17, 0.18, 0.19), "white": (0.85, 0.90, 0.95)},
    "A002 tungsten": {"grey": (0.24, 0.17, 0.09), "white": (1.18, 0.83, 0.44)},
    "A003 under": {"grey": (0.05, 0.06, 0.07), "white": (0.25, 0.29, 0.34)},
}
TARGET = 0.18

def gains(grey):
    raise NotImplementedError
@@expected
A001 daylight: gains R 1.06 G 1.00 B 0.95, exposure +0.00 stops, white -> [0.9, 0.9, 0.9] neutral
A002 tungsten: gains R 0.75 G 1.06 B 2.00, exposure +0.08 stops, white -> [0.885, 0.879, 0.88] neutral
A003 under: gains R 3.60 G 3.00 B 2.57, exposure +1.58 stops, white -> [0.9, 0.87, 0.874] not neutral; CHECK: +1.58 stops of exposure, large channel gain
@@requires
def gains
math.log2(
TARGET
@@hint
`gains = [TARGET / v for v in grey]`. Exposure = `math.log2(TARGET / grey[1])`. Apply gains to the white patch and print it; it's neutral when max − min is under 0.02.
@@solution
import math

clips = {
    "A001 daylight": {"grey": (0.17, 0.18, 0.19), "white": (0.85, 0.90, 0.95)},
    "A002 tungsten": {"grey": (0.24, 0.17, 0.09), "white": (1.18, 0.83, 0.44)},
    "A003 under": {"grey": (0.05, 0.06, 0.07), "white": (0.25, 0.29, 0.34)},
}
TARGET = 0.18

def gains(grey):
    return [TARGET / v for v in grey]

for name, patches in clips.items():
    g = gains(patches["grey"])
    stops = math.log2(TARGET / patches["grey"][1])
    white = [round(v * k, 3) for v, k in zip(patches["white"], g)]
    neutral = max(white) - min(white) < 0.02
    flags = []
    if abs(stops) > 1:
        flags.append(f"{stops:+.2f} stops of exposure")
    if any(k < 0.5 or k > 2 for k in g):
        flags.append("large channel gain")
    print(f"{name}: gains R {g[0]:.2f} G {g[1]:.2f} B {g[2]:.2f}, exposure {stops:+.2f} stops, "
          f"white -> {white} {'neutral' if neutral else 'not neutral'}{'; CHECK: ' + ', '.join(flags) if flags else ''}")

@@exercise Check skin tones on the vectorscope
Convert sampled skin RGB values (0–1, display-referred) to Rec.709 Y'CbCr, compute the vectorscope hue angle (`atan2(Cr, Cb)` in degrees, 0–360) and saturation, and compare each sample's hue with the skin tone line at 123°. Report samples more than 10° off and which way they lean (towards red/magenta or towards yellow/green).
@@starter
import math

samples = {
    "light skin": (0.92, 0.74, 0.64),
    "medium skin": (0.78, 0.55, 0.40),
    "deep skin": (0.42, 0.27, 0.19),
    "too magenta": (0.80, 0.50, 0.60),
    "too green": (0.65, 0.62, 0.40),
}
SKIN_LINE = 123

def ycbcr(r, g, b):
    raise NotImplementedError
@@expected
light skin   Y 0.77  hue 126.7°  sat 0.118   +3.7° on the line
medium skin  Y 0.59  hue 129.7°  sat 0.159   +6.7° on the line
deep skin    Y 0.30  hue 126.0°  sat 0.097   +3.0° on the line
too magenta  Y 0.57  hue  83.9°  sat 0.146  -39.1° leans red/magenta
too green    Y 0.61  hue 167.5°  sat 0.116  +44.5° leans yellow/green
@@requires
def ycbcr
math.atan2(
0.2126
@@hint
Rec.709: `Y = 0.2126R + 0.7152G + 0.0722B`, `Cb = (B − Y) / 1.8556`, `Cr = (R − Y) / 1.5748`. Hue = `math.degrees(math.atan2(Cr, Cb)) % 360`; saturation = `math.hypot(Cb, Cr)`. Red sits near 103° and yellow near 175°, so a hue above the line leans towards yellow/green and one below it towards red/magenta.
@@solution
import math

samples = {
    "light skin": (0.92, 0.74, 0.64),
    "medium skin": (0.78, 0.55, 0.40),
    "deep skin": (0.42, 0.27, 0.19),
    "too magenta": (0.80, 0.50, 0.60),
    "too green": (0.65, 0.62, 0.40),
}
SKIN_LINE = 123

def ycbcr(r, g, b):
    y = 0.2126 * r + 0.7152 * g + 0.0722 * b
    return y, (b - y) / 1.8556, (r - y) / 1.5748

for name, rgb in samples.items():
    y, cb, cr = ycbcr(*rgb)
    hue = math.degrees(math.atan2(cr, cb)) % 360
    sat = math.hypot(cb, cr)
    off = hue - SKIN_LINE
    verdict = "on the line" if abs(off) <= 10 else ("leans yellow/green" if off > 0 else "leans red/magenta")
    print(f"{name:<12} Y {y:.2f}  hue {hue:5.1f}°  sat {sat:.3f}  {off:+5.1f}° {verdict}")
