@@lesson gm1-transforms Scaling, Rotating and Transforming: Resampling Explained
@@concepts numbers, lists, functions
@@lang python
@@text
Every time you scale, rotate or apply a perspective transform, GIMP has to invent new pixels: the transformed grid no longer lines up with the original one. How it chooses those pixels (**interpolation** or **resampling**) decides whether the result is crisp, soft, jagged or haloed. This lesson explains the transform tools, the interpolation methods, and the canvas maths behind rotation.

## The transform tools

| Tool | Does |
|---|---|
| **Scale** (Shift+S) and *Image → Scale Image* | change pixel dimensions |
| **Rotate** (Shift+R) | rotate around a pivot |
| **Perspective** (Shift+P) | move four corners (fix converging verticals) |
| **Unified Transform** (Shift+T) | scale, rotate, shear and perspective in one |
| **Flip** (Shift+F) | mirror (lossless) |
| **Warp Transform** (W) | push pixels like liquify |
| **Cage / Handle Transform** | bend a region by handles |
| *Image → Canvas Size* | change the canvas without scaling |
| **Crop** (Shift+C) | cut to a rectangle; guides for thirds and golden ratio |

Flips and 90° rotations (*Image → Transform*) only move pixels: they're lossless. Everything else resamples. Transform **once** from the original rather than repeatedly; each pass softens the image.

## Interpolation methods

| Method | How | Good for |
|---|---|---|
| **None** (nearest neighbour) | copy the closest source pixel | pixel art, hard edges, exact 2×/3× scaling |
| **Linear** (bilinear) | weighted average of the 4 nearest pixels | fast, slightly soft |
| **Cubic** (bicubic) | 16 nearest pixels, smooth curve | general photos; slight sharpening |
| **NoHalo / LoHalo** | GEGL's high-quality methods | big downscales (LoHalo) and upscales without halos (NoHalo) |

Downscaling throws information away: average many source pixels (LoHalo, or a slight blur first) to avoid **aliasing** and moiré. Upscaling can't add real detail: it only smooths. AI upscalers (outside GIMP) hallucinate plausible detail, which can be wrong.

## Mapping coordinates

Resampling works **backwards**: for every destination pixel, find where it comes from in the source. With pixel centres at +0.5, scaling by s maps destination x to source `(x + 0.5) / s − 0.5`. Bilinear then blends the four surrounding pixels by distance.

## Rotation and the canvas

Rotating a w × h image by θ needs a canvas of `|w·cos θ| + |h·sin θ|` by `|w·sin θ| + |h·cos θ|` to fit everything (*Clipping: Adjust*). *Clip* keeps the original size and cuts corners; *Crop with aspect* finds the largest rectangle without transparent corners: what you want when straightening a horizon (use the **Measure** tool and *Straighten*).

@@quiz
Which interpolation keeps pixel art crisp when scaling 4×?
* None (nearest neighbour)
- Linear
- Cubic
- LoHalo
? It copies whole pixels.

@@quiz
Which transforms are lossless?
- Any rotation
* Flips and 90° rotations
- Scaling by 50%
- Perspective with NoHalo
? They only move pixels.

@@quiz
Why does heavy downscaling with nearest neighbour look bad?
- It's too slow
* It skips most source pixels, causing aliasing and moiré
- It changes colours
- It adds transparency
? Average the source pixels instead.

@@quiz
A 1000 × 500 image is rotated 90°. What canvas fits it?
- 1000 × 500
* 500 × 1000
- 1118 × 1118
- 1500 × 1500
? |cos| = 0, |sin| = 1.

@@quiz
Why transform once from the original rather than in several steps?
* Each resampling pass softens the image further
- GIMP limits the number of transforms
- It changes the file format
- Layers can only be transformed once
? Resampling loses a little every time.

@@flashcards
interpolation :: choosing new pixel values
nearest neighbour :: copy closest pixel
bilinear :: 4-pixel weighted average
bicubic :: 16 pixels, smooth
NoHalo :: upscaling without halos
LoHalo :: good downscaling
aliasing :: jaggies and moiré
lossless transforms :: flip, 90° rotations
backward mapping :: destination to source
pixel centre :: +0.5
rotated canvas width :: |w cos| + |h sin|
Clipping: Adjust :: grow canvas
Crop with aspect :: no empty corners
Measure + Straighten :: level a horizon
Unified Transform :: all-in-one

@@drill
What is the GIMP name for nearest-neighbour interpolation?
= None
= none
? It copies pixels without blending.

@@drill
How many source pixels does bilinear interpolation blend?
= 4
= four
? Bicubic uses 16.

@@exercise Nearest vs bilinear scaling
Scale a tiny grayscale image up with nearest neighbour and bilinear interpolation using backward mapping with pixel centres (`src = (dst + 0.5) / scale − 0.5`, clamped to the image), and print both results as rows of numbers. Then halve a pattern of 1-pixel stripes with nearest neighbour (the stripes alias into a solid colour) and with box averaging (the correct mid grey).
@@starter
img = [
    [0, 100, 200],
    [100, 200, 255],
]
stripes = [[255 if x % 2 == 0 else 0 for x in range(12)] for _ in range(3)]

def nearest(img, scale):
    raise NotImplementedError
@@expected
nearest x2
    0   0 100 100 200 200
    0   0 100 100 200 200
  100 100 200 200 255 255
  100 100 200 200 255 255
bilinear x2
    0  25  75 125 175 200
   25  50 100 147 192 214
   75 100 150 192 225 241
  100 125 175 214 241 255
stripes, nearest /2
    0   0   0   0   0   0
    0   0   0   0   0   0
stripes, box average /2
  128 128 128 128 128 128
@@requires
def nearest
def bilinear
+ 0.5
@@hint
For nearest, the source index is `min(int((x + 0.5) / scale), w - 1)`. For bilinear, compute the fractional source position, clamp it to `0..w-1`, take `x0 = floor`, `x1 = min(x0 + 1, w - 1)`, `fx = sx - x0`, and blend the top and bottom pairs then vertically. Box averaging by a factor f averages each f × f block of source pixels.
@@solution
import math

img = [
    [0, 100, 200],
    [100, 200, 255],
]
stripes = [[255 if x % 2 == 0 else 0 for x in range(12)] for _ in range(3)]

def nearest(img, scale):
    h, w = len(img), len(img[0])
    return [[img[min(int((y + 0.5) / scale), h - 1)][min(int((x + 0.5) / scale), w - 1)]
             for x in range(round(w * scale))] for y in range(round(h * scale))]

def source(d, scale, size):
    return min(max((d + 0.5) / scale - 0.5, 0), size - 1)

def bilinear(img, scale):
    h, w = len(img), len(img[0])
    out = []
    for y in range(round(h * scale)):
        sy = source(y, scale, h)
        y0 = math.floor(sy); y1 = min(y0 + 1, h - 1); fy = sy - y0
        row = []
        for x in range(round(w * scale)):
            sx = source(x, scale, w)
            x0 = math.floor(sx); x1 = min(x0 + 1, w - 1); fx = sx - x0
            top = img[y0][x0] * (1 - fx) + img[y0][x1] * fx
            bottom = img[y1][x0] * (1 - fx) + img[y1][x1] * fx
            row.append(round(top * (1 - fy) + bottom * fy))
        out.append(row)
    return out

def box_down(img, factor):
    h, w = len(img), len(img[0])
    return [[round(sum(img[y * factor + j][x * factor + i] for j in range(factor) for i in range(factor) if y * factor + j < h) /
                   (factor * min(factor, h - y * factor)))
             for x in range(w // factor)] for y in range(max(1, h // factor))]

def show(title, rows):
    print(title)
    for r in rows:
        print("  " + " ".join(f"{v:>3}" for v in r))

show("nearest x2", nearest(img, 2))
show("bilinear x2", bilinear(img, 2))
show("stripes, nearest /2", nearest(stripes, 1 / 2))
show("stripes, box average /2", box_down(stripes, 2))

@@exercise Canvas for a rotation
For each image and angle, compute the canvas that fits the whole rotated image (Clipping: Adjust), how many pixels of that canvas are empty (transparent corners), and the largest axis-aligned rectangle with the original aspect ratio that fits inside the rotated image (Crop with aspect), as used when straightening a horizon.
@@starter
import math

jobs = [(6000, 4000, 2.5), (6000, 4000, 90), (1000, 1000, 45), (4000, 3000, -7)]

def adjusted_canvas(w, h, degrees):
    raise NotImplementedError
@@expected
6000x4000 rotated 2.5°: canvas 6169x4258 (9% empty); crop with aspect 5636x3757 (88% of the pixels kept)
6000x4000 rotated 90°: canvas 4000x6000 (0% empty); crop with aspect 4000x2666 (44% of the pixels kept)
1000x1000 rotated 45°: canvas 1415x1415 (50% empty); crop with aspect 707x707 (50% of the pixels kept)
4000x3000 rotated -7°: canvas 4336x3466 (20% empty); crop with aspect 3463x2597 (75% of the pixels kept)
@@requires
def adjusted_canvas
def crop_with_aspect
math.radians(
@@hint
Canvas: `W = |w cos| + |h sin|`, `H = |w sin| + |h cos|`, rounded up. Empty pixels = canvas area − w·h. For the crop with the same aspect ratio r = w / h, a rectangle of width `W' = r·H'` fits when `W'·|cos| + H'·|sin| <= w` and `W'·|sin| + H'·|cos| <= h`; solve each for H' and take the smaller.
@@solution
import math

jobs = [(6000, 4000, 2.5), (6000, 4000, 90), (1000, 1000, 45), (4000, 3000, -7)]

def adjusted_canvas(w, h, degrees):
    t = math.radians(degrees)
    c, s = abs(math.cos(t)), abs(math.sin(t))
    return math.ceil(w * c + h * s - 1e-9), math.ceil(w * s + h * c - 1e-9)

def crop_with_aspect(w, h, degrees):
    t = math.radians(degrees)
    c, s = abs(math.cos(t)), abs(math.sin(t))
    r = w / h
    height = min(w / (r * c + s), h / (r * s + c))
    return math.floor(r * height + 1e-9), math.floor(height + 1e-9)

for w, h, deg in jobs:
    cw, ch = adjusted_canvas(w, h, deg)
    empty = cw * ch - w * h
    kw, kh = crop_with_aspect(w, h, deg)
    print(f"{w}x{h} rotated {deg}°: canvas {cw}x{ch} ({empty / (cw * ch):.0%} empty); "
          f"crop with aspect {kw}x{kh} ({kw * kh / (w * h):.0%} of the pixels kept)")
