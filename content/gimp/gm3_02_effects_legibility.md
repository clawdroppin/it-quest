@@lesson gm3-effects-legibility Layer Effects and Legible Text: Shadows, Outlines, Glows and Scrims
@@concepts lists, loops, numbers
@@lang python
@@text
Thumbnails, posters and banners rely on a few effects: **drop shadows**, **outlines** (strokes) and **glows**, plus readable text on top of a busy photo. GIMP 3 has filters for these (Filters → Light and Shadow → Drop Shadow, and Long Shadow, Vignette), but building them by hand from layers and masks gives you control and shows what the filters do. Each one is a simple operation on the layer's **alpha channel**.

## Effects from the alpha channel

- **Drop shadow**: Layer → Transparency → Alpha to Selection, fill a new layer below with black, **offset** it (Layer → Transform → Offset), **Gaussian blur** it, and lower the opacity (40–70%). Offset gives the direction of the light, blur gives softness, and opacity sets strength.
- **Outline / stroke**: Alpha to Selection → Select → **Grow** by N px (morphological **dilation**), and fill a layer below. Or use Edit → Stroke Selection for a line centred on the edge. Rounded growth keeps corners round.
- **Glow**: like a shadow but with no offset, a light colour, more blur, and blend mode Screen or Addition.
- **Inner shadow**: invert the alpha selection, then blur and clip it to the shape.
- In GIMP 3, filters applied as **non-destructive effects** (NDE) stay editable in the Layers dialog. Turn them off or tweak them later.

## Legible text over images

- Text needs **contrast** against what's behind it. The WCAG ratio is (L_light + 0.05) ÷ (L_dark + 0.05), using relative luminance. Aim for **4.5:1** (3:1 for large, bold headline text).
- Busy or mid-grey backgrounds fail with both black and white text. Fixes, from subtle to strong:
  1. a **shadow or outline** on the text (it helps edges, but isn't counted in the ratio)
  2. a **scrim**: a semi-transparent dark (or light) layer or gradient behind the text
  3. blur or darken that part of the photo
  4. a solid box behind the text
- A neat fact: against a **flat** colour, either black or white text always reaches 4.5:1 (the worst case, luminance about 0.18, still gives about 4.6:1). Scrims are needed for **busy** backgrounds, whose brightest and darkest spots both matter, and for the stricter AAA target of 7:1.
- With a scrim of black at opacity α over a background of luminance L, the new luminance is about L × (1 − α) (in linear light), which you can solve for the α that reaches the target.

@@quiz
Which selection operation creates an outline around a shape?
* Select → Grow, then fill a layer below
- Select → Shrink
- Select → Invert only
- Colors → Threshold
? Growing the alpha selection is morphological dilation.

@@quiz
What makes a drop shadow look softer?
- More offset
* More blur
- Higher opacity
- A darker colour
? Offset sets direction, blur sets softness.

@@quiz
White text on a mid-grey photo is hard to read. What is the cleanest fix that keeps the photo visible?
- Make the text red
* Add a semi-transparent dark scrim or gradient behind the text
- Lower the text opacity
- Use a thinner font
? Scrims raise contrast without hiding the image.

@@quiz
What contrast ratio should normal-size text reach?
- 2:1
- 3:1
* 4.5:1
- 21:1
? 3:1 is enough for large, bold headline text.

@@quiz
How is a glow usually built?
* Like a shadow without offset, with a light colour, more blur and the Screen or Addition blend mode
- By sharpening the edges
- With a hard black outline
- By lowering saturation
? Light-adding blend modes brighten what's below.

@@flashcards
Alpha to Selection :: select the layer's opaque pixels
drop shadow :: offset + blur + opacity under the layer
Select → Grow :: dilation, for outlines
Stroke Selection :: line along the selection edge
glow :: no offset, light colour, Screen mode
inner shadow :: inverted alpha, clipped to the shape
non-destructive effects :: GIMP 3 editable filters
contrast ratio :: (L1 + 0.05) ÷ (L2 + 0.05)
4.5:1 :: normal text target
3:1 :: large bold text target
scrim :: semi-transparent layer behind text
scrim maths :: L × (1 − α)

@@drill
In GIMP, which menu command grows a selection outwards to make an outline?
= Select → Grow
= grow
= Select > Grow
? Then fill a new layer below the shape.

@@drill
What contrast ratio does WCAG ask for large, bold text?
= 3:1
= 3
? 4.5:1 for normal text.

@@exercise Build a shadow and an outline from alpha
Starting from a small alpha mask (1 = shape), build an outline by dilation (grow by 1 pixel in 8 directions), and a drop shadow by offsetting the mask by 3 pixels and box-blurring it. Then composite them and print a character map: `#` for the shape, `o` for the outline, and shades for the shadow.
@@starter
shape = [
    "............",
    "..####......",
    "..####......",
    "..#######...",
    "..#######...",
    "............",
    "............",
    "............",
]
alpha = [[1 if c == "#" else 0 for c in row] for row in shape]
H, W = len(alpha), len(alpha[0])

def grow(mask):
    raise NotImplementedError

def offset_blur(mask, dx, dy):
    raise NotImplementedError
@@expected
 oooooo     
 o####o     
 o####oooo  
 o#######o  
 o#######o  
 ooooooooo..
    .:+++::.
    ..:::::.
outline pixels: 26
strongest shadow outside the outline: 1.0
@@requires
def grow
def offset_blur
range(-1, 2)
@@hint
`grow`: a pixel is 1 if any pixel in its 3×3 neighbourhood (`range(-1, 2)` in both directions, inside the bounds) is 1. `offset_blur`: read the mask at `(y - dy, x - dx)` (0 outside), then average each 3×3 neighbourhood. When compositing, the shape wins, then the outline, then the shadow. Shade shadow values with `" .:+"` by strength.
@@solution
shape = [
    "............",
    "..####......",
    "..####......",
    "..#######...",
    "..#######...",
    "............",
    "............",
    "............",
]
alpha = [[1 if c == "#" else 0 for c in row] for row in shape]
H, W = len(alpha), len(alpha[0])

def at(mask, y, x):
    return mask[y][x] if 0 <= y < H and 0 <= x < W else 0

def grow(mask):
    return [[max(at(mask, y + j, x + i) for j in range(-1, 2) for i in range(-1, 2)) for x in range(W)] for y in range(H)]

def offset_blur(mask, dx, dy):
    moved = [[at(mask, y - dy, x - dx) for x in range(W)] for y in range(H)]
    return [[sum(at(moved, y + j, x + i) for j in range(-1, 2) for i in range(-1, 2)) / 9 for x in range(W)] for y in range(H)]

outline = grow(alpha)
shadow = offset_blur(alpha, 3, 3)
SHADES = " .:+"
for y in range(H):
    row = ""
    for x in range(W):
        if alpha[y][x]:
            row += "#"
        elif outline[y][x]:
            row += "o"
        else:
            row += SHADES[min(3, round(shadow[y][x] * 3))]
    print(row)
print("outline pixels:", sum(map(sum, outline)) - sum(map(sum, alpha)))
print("strongest shadow outside the outline:", round(max(shadow[y][x] for y in range(H) for x in range(W) if not outline[y][x]), 2))

@@exercise Choose text colour and scrim opacity
For regions of a photo behind a headline (given as average sRGB colours), compute the luminance, pick black or white text by the better contrast ratio, and if neither reaches the AAA target of 7:1, compute the opacity of a black scrim that lets white text reach it.
@@starter
regions = {"sky": (120, 170, 230), "snow": (240, 242, 245), "forest": (30, 60, 35), "street": (128, 120, 115), "sunset": (230, 130, 60)}
TARGET = 7.0   # WCAG AAA

def luminance(rgb):
    raise NotImplementedError

def contrast(l1, l2):
    raise NotImplementedError
@@expected
sky     L=0.385: black text, 8.7:1
snow    L=0.886: black text, 18.7:1
forest  L=0.036: white text, 12.2:1
street  L=0.193: best is black at 4.9:1 -> add a black scrim at 48% for white text
sunset  L=0.331: black text, 7.6:1
@@requires
def luminance
def contrast
0.04045
TARGET
@@hint
Linearise each channel: c/255, then `c / 12.92` if c ≤ 0.04045 else `((c + 0.055) / 1.055) ** 2.4`. L = 0.2126R + 0.7152G + 0.0722B. White has L = 1 and black L = 0. For the scrim, solve 1.05 / (L(1 − α) + 0.05) = TARGET for α, giving α = 1 − (1.05 / TARGET − 0.05) / L.
@@solution
regions = {"sky": (120, 170, 230), "snow": (240, 242, 245), "forest": (30, 60, 35), "street": (128, 120, 115), "sunset": (230, 130, 60)}
TARGET = 7.0   # WCAG AAA

def linear(c):
    c /= 255
    return c / 12.92 if c <= 0.04045 else ((c + 0.055) / 1.055) ** 2.4

def luminance(rgb):
    r, g, b = (linear(c) for c in rgb)
    return 0.2126 * r + 0.7152 * g + 0.0722 * b

def contrast(l1, l2):
    hi, lo = max(l1, l2), min(l1, l2)
    return (hi + 0.05) / (lo + 0.05)

for name, rgb in regions.items():
    L = luminance(rgb)
    white, black = contrast(1.0, L), contrast(0.0, L)
    best, ratio = ("white", white) if white >= black else ("black", black)
    if ratio >= TARGET:
        print(f"{name:<7} L={L:.3f}: {best} text, {ratio:.1f}:1")
    else:
        alpha = 1 - (1.05 / TARGET - 0.05) / L
        print(f"{name:<7} L={L:.3f}: best is {best} at {ratio:.1f}:1 -> add a black scrim at {alpha:.0%} for white text")
