@@lesson gm1-cutouts Cutouts and Transparency: Colour to Alpha, Chroma Key, Feathering and Halos
@@concepts lists, numbers, functions
@@lang python
@@text
Removing a background is one of the most common jobs in GIMP: a product photo for a shop, a logo on a transparent background, a person composited onto a new scene. A rough cutout is easy. Avoiding the tell-tale signs is harder: a white or green fringe around the edge, jagged hair, a hard edge where there should be a soft one. This lesson covers the tools GIMP offers and the maths behind the two most useful ones.

## The tools

- **Fuzzy Select (magic wand)** and **Select by Color**: good for flat, evenly coloured backgrounds. The **threshold** sets how similar a pixel must be. Antialiasing and **feathering** soften the edge.
- **Foreground Select**: draw a rough outline, paint over the foreground, and GIMP's matting engine (Matting Levin or Matting Global) computes a soft mask, which is good for hair and fur.
- **Paths tool**: precise outlines for hard-edged objects such as products, which you then convert to a selection (Select → From Path).
- **Colors → Color to Alpha**: removes a colour while keeping partial transparency, which is ideal for a logo scanned on white, smoke or glass.
- **Layer masks**: never delete pixels if you can mask them. Paint the mask with black and white and fix mistakes later.

## Alpha and how edges work

- Every pixel has an **alpha** (0 = transparent, 255 = opaque). Edge pixels of a smooth cutout are partly transparent: a hair pixel might be 40% hair and 60% background.
- If the edge pixel's colour still contains the old background, compositing it onto a new background shows a **halo** or **fringe**: light around a subject cut from white, green around one cut from a green screen.
- **Decontaminating** edge colours means working out what the foreground colour would be without the background. Color to Alpha does this: for a pixel p, a background colour C and an alpha a, the observed colour is p = a × F + (1 − a) × C, so F = C + (p − C) / a.
- **Layer → Transparency → Remove Color to Alpha** reverses this when needed, and **Select → Shrink** by 1–2 px followed by a small feather is the quick fix when the fringe is only a pixel wide.

## Color to Alpha in detail

For each channel, work out the smallest alpha that can explain the pixel as a mix of the background colour and some valid foreground value (0–255):
- If p > C, the foreground must be at most 255, so a ≥ (p − C) / (255 − C).
- If p < C, the foreground must be at least 0, so a ≥ (C − p) / C.
- If p = C, that channel needs no alpha at all.

The pixel's alpha is the **largest** of the three channel values, and the foreground colour is then recovered with F = C + (p − C) / a. Pure white on white becomes fully transparent, mid grey on white becomes black at 50% alpha, and pure red stays opaque red.

## Chroma keying (green screen)

- Measure each pixel's **distance** from the key colour. Below an **inner** tolerance it is fully transparent, above an **outer** tolerance fully opaque, and in between alpha ramps linearly: a soft edge.
- **Spill suppression**: green light bounces onto the subject. A classic fix is to clamp green to at most the larger of red and blue (g = min(g, max(r, b))) on edge and foreground pixels. Skin and hair lose the green cast without turning magenta.
- Light the screen evenly and keep the subject a metre or two in front of it: shadows and spill are much harder to fix than to avoid.

## Feathering, shrinking and the final check

- **Feather** (Select → Feather) blurs the selection mask, so the transition is gradual. A 1–2 px feather suits most product photos, and more looks blurry.
- **Grow and shrink** move the edge in or out. Shrinking by one pixel before feathering removes most fringes.
- Always check a cutout on several backgrounds: **black, white and a saturated colour**. A fringe that's invisible on white jumps out on black.
- Export with transparency as PNG or WebP. JPEG has no alpha and fills transparent areas with a colour.

@@quiz
What causes a light halo around a subject cut from a white background and placed on dark grey?
* Edge pixels still contain some of the old white background
- The PNG export added a border
- The subject is too sharp
- The layer mode is wrong
? Decontaminate edge colours or shrink the selection.

@@quiz
Which tool is designed for hair and fur against a busy background?
- Rectangle Select
* Foreground Select (matting)
- Select by Color
- Crop tool
? It computes a soft mask from a rough trimap.

@@quiz
Color to Alpha removes white. What does a mid-grey pixel become?
* Black at about 50% alpha
- Fully transparent
- White at 50% alpha
- Unchanged and opaque
? 50% black over white looks grey.

@@quiz
What is spill suppression in chroma keying?
* Removing the green cast that bounced onto the subject, e.g. by clamping green to max(red, blue)
- Making the screen greener
- Blurring the background
- Increasing the outer tolerance
? It fixes colour, not alpha.

@@quiz
Why use a layer mask rather than deleting pixels?
* You can fix mistakes later by repainting the mask
- Masks make files smaller
- Deleting pixels is impossible in GIMP
- Masks remove halos automatically
? Non-destructive editing.

@@quiz
Why check a finished cutout on black, white and a saturated colour?
* Fringes that are invisible on one background show up on another
- To choose the export format
- GIMP requires it
- To test colour management
? Especially white fringes on black.

@@flashcards
alpha :: 0 transparent, 255 opaque
halo / fringe :: old background colour left in edge pixels
decontaminate :: recover the foreground colour of edge pixels
compositing equation :: p = aF + (1 − a)C
recover foreground :: F = C + (p − C) / a
Color to Alpha :: removes a colour, keeps partial transparency
Foreground Select :: matting for hair and fur
Fuzzy Select :: contiguous similar colours
Select by Color :: all similar colours in the layer
chroma key :: transparency from distance to the key colour
inner / outer tolerance :: fully transparent / fully opaque thresholds
spill suppression :: g = min(g, max(r, b))
feather :: soften the selection edge
shrink then feather :: quick fringe fix
check backgrounds :: black, white and a saturated colour

@@drill
Removing white (255) with Color to Alpha, a channel value of 0 needs what alpha (0–1)?
= 1
= 1.0
? (C − p) / C = 255 / 255.

@@drill
A pixel is (90, 200, 60). After spill suppression g = min(g, max(r, b)), what is the green value?
= 90
? max(90, 60) = 90.

@@exercise Colour to Alpha
Implement GIMP-style Color to Alpha for an 8-bit RGB pixel and a background colour: compute the alpha needed per channel, take the maximum, and recover the foreground colour. Return `(r, g, b, alpha)` with alpha 0–255 (rounded). A fully transparent result returns `(0, 0, 0, 0)`.
@@starter
def channel_alpha(p, c):
    raise NotImplementedError

def colour_to_alpha(pixel, bg):
    raise NotImplementedError

white = (255, 255, 255)
for px in [(255, 255, 255), (128, 128, 128), (0, 0, 0), (255, 0, 0), (255, 200, 200), (200, 220, 255)]:
    print(px, "->", colour_to_alpha(px, white))
@@expected
(255, 255, 255) -> (0, 0, 0, 0)
(128, 128, 128) -> (0, 0, 0, 127)
(0, 0, 0) -> (0, 0, 0, 255)
(255, 0, 0) -> (255, 0, 0, 255)
(255, 200, 200) -> (255, 0, 0, 55)
(200, 220, 255) -> (0, 93, 255, 55)
@@requires
def channel_alpha
def colour_to_alpha
max(
@@hint
`channel_alpha`: if `p > c` return `(p - c) / (255 - c)`; if `p < c` return `(c - p) / c`; otherwise 0. (Guard the divisions: when c is 255 nothing can be above it.) In `colour_to_alpha`, `a = max(...)` over the three channels; if a is 0 return `(0, 0, 0, 0)`; otherwise each foreground channel is `round(c + (p - c) / a)` and alpha is `round(a * 255)`.
@@solution
def channel_alpha(p, c):
    if p > c:
        return (p - c) / (255 - c)
    if p < c:
        return (c - p) / c
    return 0.0

def colour_to_alpha(pixel, bg):
    a = max(channel_alpha(p, c) for p, c in zip(pixel, bg))
    if a == 0:
        return (0, 0, 0, 0)
    fg = tuple(round(c + (p - c) / a) for p, c in zip(pixel, bg))
    return fg + (round(a * 255),)

white = (255, 255, 255)
for px in [(255, 255, 255), (128, 128, 128), (0, 0, 0), (255, 0, 0), (255, 200, 200), (200, 220, 255)]:
    print(px, "->", colour_to_alpha(px, white))

@@exercise Green screen key with a soft edge
Key out green from a row of pixels: compute the Euclidean distance from the key colour, give alpha 0 below the inner tolerance, 255 above the outer tolerance and a linear ramp between, then apply spill suppression to every pixel that isn't fully transparent. Print each result and a one-character preview (`.` transparent, `+` partial, `#` opaque).
@@starter
import math

KEY = (40, 200, 60)
INNER, OUTER = 40, 120
row = [(40, 200, 60), (55, 190, 70), (90, 180, 80), (150, 170, 120), (200, 160, 140), (210, 170, 150), (60, 210, 55)]

def key_alpha(px):
    raise NotImplementedError

def despill(px):
    raise NotImplementedError

preview = ""
for px in row:
    a = key_alpha(px)
    out = despill(px) if a > 0 else (0, 0, 0)
    preview += "." if a == 0 else "#" if a == 255 else "+"
    print(f"{str(px):<16} alpha {a:>3}  colour {out}")
print(preview)
@@expected
(40, 200, 60)    alpha   0  colour (0, 0, 0)
(55, 190, 70)    alpha   0  colour (0, 0, 0)
(90, 180, 80)    alpha  56  colour (90, 90, 80)
(150, 170, 120)  alpha 255  colour (150, 150, 120)
(200, 160, 140)  alpha 255  colour (200, 160, 140)
(210, 170, 150)  alpha 255  colour (210, 170, 150)
(60, 210, 55)    alpha   0  colour (0, 0, 0)
..+###.
@@requires
def key_alpha
def despill
math.dist(
@@hint
`d = math.dist(px, KEY)`. If `d <= INNER` alpha is 0, if `d >= OUTER` it's 255, otherwise `round((d - INNER) / (OUTER - INNER) * 255)`. `despill`: `r, g, b = px` and return `(r, min(g, max(r, b)), b)`.
@@solution
import math

KEY = (40, 200, 60)
INNER, OUTER = 40, 120
row = [(40, 200, 60), (55, 190, 70), (90, 180, 80), (150, 170, 120), (200, 160, 140), (210, 170, 150), (60, 210, 55)]

def key_alpha(px):
    d = math.dist(px, KEY)
    if d <= INNER:
        return 0
    if d >= OUTER:
        return 255
    return round((d - INNER) / (OUTER - INNER) * 255)

def despill(px):
    r, g, b = px
    return (r, min(g, max(r, b)), b)

preview = ""
for px in row:
    a = key_alpha(px)
    out = despill(px) if a > 0 else (0, 0, 0)
    preview += "." if a == 0 else "#" if a == 255 else "+"
    print(f"{str(px):<16} alpha {a:>3}  colour {out}")
print(preview)
