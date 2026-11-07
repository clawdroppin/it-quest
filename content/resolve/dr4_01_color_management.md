@@lesson dr4-colormgmt Scopes, Colour Management, Log, ACES and HDR
@@concepts numbers, lists, functions
@@lang python
@@text
Your eyes adapt and your monitor may lie; **scopes** don't. And modern cameras record **log** footage in wide colour spaces that must be transformed correctly to look right on a TV, a phone or an HDR display. This lesson covers the technical foundation that makes grades consistent.

## Video scopes

| Scope | Shows | Use |
|---|---|---|
| **Waveform** | brightness (luma) of every column of the image, bottom 0 to top 1023 (or 0–100 IRE) | exposure, contrast, clipping, matching levels |
| **RGB Parade** | three waveforms side by side: R, G, B | colour casts, balancing, matching |
| **Vectorscope** | hue (angle) and saturation (distance from centre) | skin tone line, oversaturation, casts |
| **Histogram** | how many pixels at each level | overall distribution |
| **CIE chromaticity** | colours plotted against gamut triangles | gamut checks for wide gamut/HDR |

Rules of thumb for Rec.709 delivery: blacks just above 0, highlights under ~95–100 IRE (legal range ends at 940 of 1023 in video levels), skin tones sitting on the **skin tone indicator** line in the vectorscope (about 123° from the red axis, between red and yellow), no colours spiking past the 75% targets on the vectorscope for broadcast.

**Luma** (Rec.709) is a weighted sum: `Y = 0.2126 R + 0.7152 G + 0.0722 B`: green contributes most to perceived brightness, blue the least. The first exercise builds a tiny waveform and finds clipped pixels.

## Data levels vs video levels

Digital video often uses **video (legal) levels**: 10-bit black at 64 and white at 940 (8-bit 16–235). Computer graphics and most web output use **full (data) levels**: 0–1023 (0–255). Resolve handles this automatically for most clips; a mismatch shows as washed-out blacks or crushed shadows. Clip attributes → Data Levels lets you override per clip.

## Log footage

Cameras record **log** (Sony S-Log3, Canon C-Log, Panasonic V-Log, Blackmagic Film, ARRI LogC, Apple Log on iPhones, DJI D-Log): a flat, grey-looking image that squeezes a huge dynamic range into the file. It **must** be transformed to a display space; grading log directly with ordinary controls gives poor results. Options:

1. A **manufacturer LUT** (log → Rec.709) late in the node tree (simple, less flexible).
2. A **Color Space Transform** (CST) node: input colour space and gamma (e.g. S-Gamut3.Cine / S-Log3) → output (Rec.709 / Gamma 2.4), with **tone mapping** and gamut mapping.
3. **Colour-managed** project (below): Resolve does it automatically from clip metadata.

The second exercise converts a log-encoded value back to linear light and computes exposure in **stops** (each stop doubles the light).

## DaVinci colour management (RCM)

Project Settings → Color Management → Color science **DaVinci YRGB Color Managed**:

- **Input**: automatic from camera metadata, or set per clip (Input Color Space).
- **Timeline / working colour space**: **DaVinci Wide Gamut / DaVinci Intermediate**: a huge gamut and log-like encoding so grades behave consistently across all cameras.
- **Output colour space**: Rec.709 Gamma 2.4 (TV), Rec.709-A or sRGB for the web on Macs, **Rec.2100 ST2084 (PQ)** for HDR10, **Rec.2100 HLG** for broadcast HDR, P3-D65 for cinema DCP.
- **Tone mapping and gamut mapping** compress highlights and saturated colours smoothly into the output.

Benefits: mix cameras, swap output for HDR and SDR deliveries, and grading controls feel the same on every clip.

## ACES

**ACES** (Academy Color Encoding System) is the film industry's open standard: **IDTs** (input transforms per camera) → working space **ACEScct** (log, for grading) or ACEScg (linear, for CG/VFX) → **ODTs/output transforms** for each display. Choose **ACEScct** color science in Resolve when collaborating with VFX houses or Blender renders in ACES. **ACES 2.0** brings improved output transforms. EXR files in ACEScg move between Blender, Fusion and Resolve without colour shifts.

## Gamma, gamuts and displays

- **Gamma 2.4** for TV in dim rooms (Rec.709/BT.1886), **2.2** / **sRGB** for computer displays in bright rooms. The "my export looks washed out on QuickTime/Mac" problem comes from gamma tag mismatches: Resolve offers Rec.709-A and output tagging options.
- **Gamuts**: Rec.709 (HD), **DCI-P3** (cinema and modern phones), **Rec.2020** (UHD/HDR, very wide).
- Grade on a **calibrated** monitor fed by a dedicated video output (Blackmagic DeckLink/UltraStudio) if colour matters professionally; computer GUI viewers are colour-managed by the OS but less reliable.

## HDR

HDR (Studio) uses **PQ (ST 2084)** with absolute brightness up to 10,000 nits (typical mastering at 1,000 nits) or **HLG** (relative, backwards-compatible with SDR TVs). Resolve offers **HDR palette** zones, **Dolby Vision** analysis and trim passes, and **HDR10+**. Tone-map a single HDR master to SDR for other deliverables. Grading HDR needs an HDR-capable monitor.

## Noise reduction and finishing (Studio)

- **Temporal and Spatial NR** (Motion Effects palette) before the grade in the node tree.
- **Film grain**, **Halation**, **Film Look Creator** for a filmic finish; **Glow**, **Lens flare**, **Face refinement**, **Sharpen** via Resolve FX.
- Put heavy effects late and **cache** nodes (right-click → Node Cache → On) to keep playback real-time.

@@quiz
Which scope shows hue as an angle and saturation as distance from the centre?
- Waveform
- Histogram
* Vectorscope
- RGB Parade
? The skin tone line helps check faces.

@@quiz
Why does log footage look flat and grey?
* It squeezes a wide dynamic range into the file with a log curve; it must be transformed to a display space
- The camera was broken
- It's always 8-bit
- It has no colour information
? Use a CST, a LUT or colour management.

@@quiz
Which output colour space would you choose for an HDR10 delivery?
- Rec.709 Gamma 2.4
* Rec.2100 ST2084 (PQ)
- sRGB
- Rec.709-A
? HLG is the broadcast HDR alternative.

@@quiz
Which colour contributes most to luma in Rec.709?
- Red
* Green
- Blue
- They contribute equally
? Y = 0.2126 R + 0.7152 G + 0.0722 B.

@@quiz
What does one stop of exposure mean in linear light?
- Adding 10%
* Doubling (or halving) the amount of light
- Adding 1 to the code value
- Changing the gamma
? log2 of the ratio of linear values gives stops.

@@drill
What does ACES stand for in colour management?
= academy color encoding system
= academy colour encoding system
? A standard colour pipeline for film and TV.

@@flashcards
waveform :: luma per image column, exposure and clipping
RGB Parade :: per-channel waveforms for balance
vectorscope :: hue angle and saturation
skin tone line :: where skin should sit on the vectorscope
luma (Rec.709) :: 0.2126 R + 0.7152 G + 0.0722 B
video vs data levels :: 64–940 vs 0–1023 (10-bit)
log footage :: flat high dynamic range encoding
CST :: Color Space Transform node
DaVinci Wide Gamut / Intermediate :: Resolve's working space and encoding
RCM :: DaVinci YRGB Color Managed
ACES / ACEScct / ACEScg :: film standard / log grading space / linear CG space
IDT / ODT :: input / output transforms in ACES
Rec.709 / DCI-P3 / Rec.2020 :: HD / cinema / UHD-HDR gamuts
Gamma 2.4 vs sRGB :: TV in dim rooms vs computer displays
PQ (ST 2084) / HLG :: absolute / relative HDR curves
nits :: cd/m², display brightness
stop :: doubling of light

@@exercise A waveform in code
An image is a list of rows; each pixel is `(r, g, b)` 0–1. Write `luma(px)` (Rec.709, rounded to 4). Write `waveform(image, bins)`: for each column, a list of counts of how many pixels fall in each of `bins` equal brightness bins (bin = `min(int(y * bins), bins - 1)`), from dark to bright. Write `clipped(image, low=0.01, high=0.99)` returning `{"crushed": n, "clipped": m}`: pixels with luma ≤ low and ≥ high. Then `to_ire(y)`: luma 0–1 to IRE 0–100 rounded to 1, and `legal_10bit(y)`: luma to 10-bit video levels (64–940), rounded to an int.
@@starter
def luma(px):
    pass


def waveform(image, bins):
    pass


def clipped(image, low=0.01, high=0.99):
    pass


def to_ire(y):
    pass


def legal_10bit(y):
    pass


image = [
    [(1, 1, 1), (0.5, 0.5, 0.5), (0, 0, 0)],
    [(0.9, 0.2, 0.2), (0.2, 0.9, 0.2), (0.2, 0.2, 0.9)],
]
print(waveform(image, 4), clipped(image), to_ire(0.5), legal_10bit(1.0))
@@tests
check(luma((1, 1, 1)), 1.0, "white")
check(luma((1, 0, 0)), 0.2126, "red")
check(luma((0, 1, 0)), 0.7152, "green is brightest")
image = [
    [(1, 1, 1), (0.5, 0.5, 0.5), (0, 0, 0)],
    [(0.9, 0.2, 0.2), (0.2, 0.9, 0.2), (0.2, 0.2, 0.9)],
]
check(waveform(image, 4), [[0, 1, 0, 1], [0, 0, 2, 0], [1, 1, 0, 0]], "per-column histograms")
check(clipped(image), {"crushed": 1, "clipped": 1}, "one black, one white pixel")
check(to_ire(0.5), 50.0, "half")
check(legal_10bit(0.0), 64, "video black")
check(legal_10bit(1.0), 940, "video white")
check(legal_10bit(0.5), 502, "middle")
@@hint
Columns: `for x in range(len(image[0]))`, then loop over rows. Legal range: `64 + y * (940 - 64)`.
@@solution
def luma(px):
    r, g, b = px
    return round(0.2126 * r + 0.7152 * g + 0.0722 * b, 4)


def waveform(image, bins):
    cols = []
    for x in range(len(image[0])):
        counts = [0] * bins
        for row in image:
            y = luma(row[x])
            counts[min(int(y * bins), bins - 1)] += 1
        cols.append(counts)
    return cols


def clipped(image, low=0.01, high=0.99):
    ys = [luma(px) for row in image for px in row]
    return {"crushed": sum(1 for y in ys if y <= low), "clipped": sum(1 for y in ys if y >= high)}


def to_ire(y):
    return round(y * 100, 1)


def legal_10bit(y):
    return round(64 + y * (940 - 64))


image = [
    [(1, 1, 1), (0.5, 0.5, 0.5), (0, 0, 0)],
    [(0.9, 0.2, 0.2), (0.2, 0.9, 0.2), (0.2, 0.2, 0.9)],
]
print(waveform(image, 4), clipped(image), to_ire(0.5), legal_10bit(1.0))

@@exercise Log, linear and stops
A simplified log curve (similar in spirit to camera log formats) encodes linear light `x` as `code = 0.25 * log10(x / 0.18) + 0.41`, so 18% grey sits at code 0.41. Write `log_encode(x)` and `log_decode(code)` (the inverse), both rounded to 4. Write `stops(a, b)`: how many stops brighter linear value `b` is than `a` (`log2(b / a)`), rounded to 2. Then `dynamic_range(min_code, max_code)`: the number of stops between the decoded linear values of two codes, rounded to 1. Finally `exposure_shift(x, stops)`: linear value after changing exposure by a number of stops, rounded to 4.
@@starter
import math


def log_encode(x):
    pass


def log_decode(code):
    pass


def stops(a, b):
    pass


def dynamic_range(min_code, max_code):
    pass


def exposure_shift(x, stops):
    pass


print(log_encode(0.18), log_encode(0.36), log_decode(0.41), stops(0.18, 0.72), dynamic_range(0.0, 1.0))
@@tests
check(log_encode(0.18), 0.41, "middle grey")
check(log_decode(0.41), 0.18, "and back")
check(log_encode(1.8), 0.66, "ten times brighter = +0.25")
check(round(log_decode(log_encode(0.5)), 2), 0.5, "round trip (within rounding)")
check(stops(0.18, 0.36), 1.0, "double = one stop")
check(stops(0.18, 0.72), 2.0, "four times = two stops")
check(stops(0.36, 0.09), -2.0, "darker is negative")
check(dynamic_range(0.0, 1.0), 13.3, "about 13 stops in the code range")
check(exposure_shift(0.18, 1), 0.36, "one stop up")
check(exposure_shift(0.18, -2), 0.045, "two stops down")
@@hint
Inverse: `x = 0.18 * 10 ** ((code - 0.41) / 0.25)`. Exposure: `x * 2 ** stops`. For the dynamic range, decode both codes and take `log2` of the ratio.
@@solution
import math


def log_encode(x):
    return round(0.25 * math.log10(x / 0.18) + 0.41, 4)


def log_decode(code):
    return round(0.18 * 10 ** ((code - 0.41) / 0.25), 4)


def stops(a, b):
    return round(math.log2(b / a), 2)


def dynamic_range(min_code, max_code):
    lo = 0.18 * 10 ** ((min_code - 0.41) / 0.25)
    hi = 0.18 * 10 ** ((max_code - 0.41) / 0.25)
    return round(math.log2(hi / lo), 1)


def exposure_shift(x, stops):
    return round(x * 2 ** stops, 4)


print(log_encode(0.18), log_encode(0.36), log_decode(0.41), stops(0.18, 0.72), dynamic_range(0.0, 1.0))
