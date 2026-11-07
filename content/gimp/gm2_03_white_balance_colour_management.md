@@lesson gm2-colour-management White Balance and Colour Management: Neutral Greys, ICC Profiles and Print
@@concepts lists, numbers, functions
@@lang python
@@text
A photo taken under tungsten light looks orange, one under shade looks blue, and an image that looked perfect on your monitor prints dull and dark. Two separate skills fix these problems. **White balance** removes a colour cast from the scene's lighting. **Colour management** makes the same numbers look the same on different screens and on paper.

## White balance

- Light sources have a **colour temperature**: candle about 1900 K, tungsten about 2800 K, daylight about 5500–6500 K, overcast shade 7000 K or more. Cameras correct for it, sometimes badly.
- The fix is to scale the red, green and blue channels so that something that should be **neutral** (grey or white) has equal R, G and B.
- In GIMP:
  - **Levels → grey point picker**: click on something that should be neutral grey (a grey card, concrete, a white shirt in shade) and GIMP scales the channels to make it neutral.
  - **Colors → Auto → White Balance**: stretches each channel separately, discarding the extreme 0.05% of pixels. It's quick, but it fails when the image really is dominated by one colour (a sunset, a forest).
  - **Curves** per channel for fine control, e.g. warming shadows and cooling highlights.
- Two classic automatic methods:
  - **Grey world** assumes the average of the whole scene is grey: gain_c = mean of all channels' averages ÷ the channel's average.
  - **White patch** assumes the brightest area should be white: gain_c = 255 ÷ that channel's maximum (or a high percentile, to ignore specular highlights).
- Do white balance **first**, in a raw developer (darktable, RawTherapee) if you shoot raw, where it's applied to linear sensor data and can't clip.

## Colour spaces and ICC profiles

- RGB numbers mean nothing on their own. (255, 0, 0) is a different red in **sRGB**, **Display P3** and **Adobe RGB**, because each space has different primaries (gamut) and a different white point.
- An **ICC profile** describes a colour space or a device. GIMP works in the image's profile (Image → Color Management) and converts to your monitor's profile for display.
- **Assign** a profile: keep the numbers and change their meaning, so colours look different. Use this when a file had no profile or the wrong one.
- **Convert** to a profile: change the numbers to keep the appearance the same. Use this before exporting for the web (convert to sRGB) or for print (to the printer's CMYK profile).
- For the web, export in **sRGB** and embed the profile. Wide-gamut images without a profile look dull or oversaturated in some browsers.

## Print: CMYK, rendering intents and soft proofing

- Printers mix **cyan, magenta, yellow and black** ink on paper. The gamut is smaller than sRGB: saturated blues, greens and bright oranges often can't be printed.
- **Rendering intents** decide what happens to out-of-gamut colours:
  - **Perceptual** compresses the whole gamut, which suits photos.
  - **Relative colorimetric** clips only the out-of-gamut colours, which suits images that are mostly in gamut and logos.
  - **Saturation** is for business graphics.
  - **Absolute colorimetric** simulates paper white, for proofing.
- **Soft proofing** (View → Color Management → Proof Colors, with a gamut warning) shows on screen roughly how the print will look.
- **Total ink limit (TAC)**: paper can only take so much ink before it bleeds or smudges, often 300% for coated paper and 240–260% for newsprint. CMYK separation uses **GCR/UCR** (replacing equal amounts of C, M and Y with K) to stay under it.
- GIMP 2.10 and 3 can soft-proof and export CMYK JPEG/TIFF in GIMP 3, but print shops often prefer an RGB PDF with an embedded profile, which they convert themselves. Ask them.
- Screens are bright and backlit, paper is not. Calibrate the monitor and turn its brightness down to about 100–120 cd/m² when preparing print work, or every print will look too dark.

@@quiz
A photo taken under tungsten light has an orange cast. What does white balancing do?
* Scales the channels so neutral objects have equal R, G and B
- Increases saturation
- Converts the image to CMYK
- Assigns the Adobe RGB profile
? Here it boosts blue and reduces red.

@@quiz
When does the grey-world method fail?
* When the scene is dominated by one colour, such as a forest or a sunset
- When the image is too small
- When the image has a profile
- Never
? Its assumption, an average grey, is false.

@@quiz
What is the difference between assigning and converting a profile?
* Assign keeps the numbers and changes the appearance, convert changes the numbers to keep the appearance
- They are the same
- Assign is only for CMYK
- Convert keeps the numbers
? Convert to sRGB before web export.

@@quiz
Which rendering intent suits photographs with many out-of-gamut colours?
* Perceptual
- Absolute colorimetric
- Saturation
- None
? It compresses the whole gamut smoothly.

@@quiz
Why does a print shop specify a total ink limit?
* Too much ink on paper bleeds, smudges and dries slowly
- Ink is expensive
- Screens can't show more than 100%
- To make blacks lighter
? GCR/UCR replace CMY with K to stay under it.

@@quiz
Which colour space should web images be exported in?
* sRGB, with the profile embedded
- Adobe RGB without a profile
- CMYK
- Linear light
? The safe default for browsers.

@@flashcards
colour temperature :: tungsten ~2800 K, daylight ~5500–6500 K
white balance :: make neutrals have equal R, G, B
grey point picker :: Levels: click something neutral
grey world :: average of the scene is grey
white patch :: brightest area is white
ICC profile :: describes a colour space or device
assign profile :: same numbers, new meaning
convert to profile :: new numbers, same appearance
sRGB :: default for web export
gamut :: range of colours a space or device can show
perceptual :: compress the whole gamut
relative colorimetric :: clip only out-of-gamut colours
soft proofing :: preview print on screen
TAC :: total ink limit, e.g. 300%
GCR / UCR :: replace CMY with black ink

@@drill
In grey world balancing, the channel averages are R 120, G 100, B 80. What is the gain for red? Give two decimals.
= 0.83
? Mean 100 ÷ 120.

@@drill
A CMYK colour is C 80%, M 70%, Y 70%, K 90%. What is its total ink coverage in percent?
= 310
= 310%
? Over a 300% limit.

@@exercise Grey world and white patch
Implement both automatic white balance methods for a list of RGB pixels with a warm (orange) cast. Return per-channel gains, apply them (rounding and clamping to 0–255), and print the corrected pixels and the corrected averages.
@@starter
pixels = [(200, 150, 100), (240, 200, 150), (120, 90, 60), (90, 80, 70), (250, 230, 190), (160, 120, 80)]

def grey_world_gains(px):
    raise NotImplementedError

def white_patch_gains(px):
    raise NotImplementedError

def apply(px, gains):
    return [tuple(min(255, round(v * g)) for v, g in zip(p, gains)) for p in px]

def averages(px):
    return tuple(round(sum(p[i] for p in px) / len(px)) for i in range(3))

print("before", averages(pixels))
for name, fn in (("grey world", grey_world_gains), ("white patch", white_patch_gains)):
    gains = fn(pixels)
    out = apply(pixels, gains)
    print(f"{name:<11} gains {tuple(round(g, 2) for g in gains)}  averages {averages(out)}")
    print("           ", out[:3])
@@expected
before (177, 145, 108)
grey world  gains (0.81, 0.99, 1.32)  averages (143, 143, 143)
            [(162, 148, 132), (195, 198, 198), (97, 89, 79)]
white patch gains (1.02, 1.11, 1.34)  averages (180, 161, 145)
            [(204, 166, 134), (245, 222, 201), (122, 100, 81)]
@@requires
def grey_world_gains
def white_patch_gains
max(
@@hint
Grey world: average each channel, take the mean of the three averages, and the gain is `mean / channel_average`. White patch: the gain is `255 / max(p[i] for p in px)` for each channel.
@@solution
pixels = [(200, 150, 100), (240, 200, 150), (120, 90, 60), (90, 80, 70), (250, 230, 190), (160, 120, 80)]

def grey_world_gains(px):
    avg = [sum(p[i] for p in px) / len(px) for i in range(3)]
    grey = sum(avg) / 3
    return tuple(grey / a for a in avg)

def white_patch_gains(px):
    return tuple(255 / max(p[i] for p in px) for i in range(3))

def apply(px, gains):
    return [tuple(min(255, round(v * g)) for v, g in zip(p, gains)) for p in px]

def averages(px):
    return tuple(round(sum(p[i] for p in px) / len(px)) for i in range(3))

print("before", averages(pixels))
for name, fn in (("grey world", grey_world_gains), ("white patch", white_patch_gains)):
    gains = fn(pixels)
    out = apply(pixels, gains)
    print(f"{name:<11} gains {tuple(round(g, 2) for g in gains)}  averages {averages(out)}")
    print("           ", out[:3])

@@exercise CMYK with a total ink limit
Convert RGB to CMYK with grey component replacement: K = gcr × (1 − max(r, g, b)) on a 0–1 scale, then C = (1 − r − K) / (1 − K), and M and Y likewise (a simplified separation without a printer profile; pure black is (0, 0, 0, 1)). If the total ink (C + M + Y + K) exceeds the limit, scale C, M and Y down so the total equals the limit. Print percentages and the total for each colour with gcr 0.5 and gcr 1.0 and a 260% limit.
@@starter
LIMIT = 2.6

def to_cmyk(rgb, gcr):
    raise NotImplementedError

def limit_ink(cmyk, limit=LIMIT):
    raise NotImplementedError

def pct(cmyk):
    return " ".join(f"{round(v * 100):>3}" for v in cmyk) + f"  total {round(sum(cmyk) * 100):>3}%"

colours = {"rich black": (10, 10, 15), "dark red": (120, 10, 20), "navy": (20, 30, 90), "mid grey": (128, 128, 128)}
for gcr in (0.5, 1.0):
    print(f"gcr {gcr}")
    for name, rgb in colours.items():
        raw = to_cmyk(rgb, gcr)
        print(f"  {name:<10} {pct(raw)}  ->  {pct(limit_ink(raw))}")
@@expected
gcr 0.5
  rich black  93  93  89  47  total 321%  ->   72  72  69  47  total 260%
  dark red    36  95  89  26  total 246%  ->   36  95  89  26  total 246%
  navy        88  83  48  32  total 251%  ->   88  83  48  32  total 251%
  mid grey    33  33  33  25  total 124%  ->   33  33  33  25  total 124%
gcr 1.0
  rich black  33  33   0  94  total 161%  ->   33  33   0  94  total 161%
  dark red     0  92  83  53  total 228%  ->    0  92  83  53  total 228%
  navy        78  67   0  65  total 209%  ->   78  67   0  65  total 209%
  mid grey     0   0   0  50  total  50%  ->    0   0   0  50  total  50%
@@requires
def to_cmyk
def limit_ink
max(
@@hint
Scale RGB to 0–1, `k = gcr * (1 - max(r, g, b))`, then `c = (1 - r - k) / (1 - k)` and so on (return `(0, 0, 0, 1)` when k is 1). In `limit_ink`, if `sum(cmyk) <= limit` return it unchanged; otherwise `factor = (limit - k) / (c + m + y)` and multiply C, M and Y by it.
@@solution
LIMIT = 2.6

def to_cmyk(rgb, gcr):
    r, g, b = (v / 255 for v in rgb)
    k = gcr * (1 - max(r, g, b))
    if k >= 1:
        return (0.0, 0.0, 0.0, 1.0)
    return ((1 - r - k) / (1 - k), (1 - g - k) / (1 - k), (1 - b - k) / (1 - k), k)

def limit_ink(cmyk, limit=LIMIT):
    c, m, y, k = cmyk
    if c + m + y + k <= limit:
        return cmyk
    factor = (limit - k) / (c + m + y)
    return (c * factor, m * factor, y * factor, k)

def pct(cmyk):
    return " ".join(f"{round(v * 100):>3}" for v in cmyk) + f"  total {round(sum(cmyk) * 100):>3}%"

colours = {"rich black": (10, 10, 15), "dark red": (120, 10, 20), "navy": (20, 30, 90), "mid grey": (128, 128, 128)}
for gcr in (0.5, 1.0):
    print(f"gcr {gcr}")
    for name, rgb in colours.items():
        raw = to_cmyk(rgb, gcr)
        print(f"  {name:<10} {pct(raw)}  ->  {pct(limit_ink(raw))}")
