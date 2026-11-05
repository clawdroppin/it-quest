@@lesson gm2-sharpen Blur, Sharpening and Noise Reduction
@@concepts lists, numbers, functions
@@lang python
@@text
Sharpening and noise reduction pull in opposite directions: one exaggerates edges and fine detail, the other smooths it away, and noise is itself fine detail. Getting both right is what makes an edited photo look clean instead of crunchy or plastic. This lesson explains Gaussian blur, unsharp masking, and the main denoising filters, with exercises that build a Gaussian kernel and compare mean and median filters.

## Blurs

- **Gaussian Blur** (*Filters → Blur*): each pixel becomes a weighted average of its neighbours, with weights from the bell curve `exp(−x² / 2σ²)`. Larger σ (radius) = stronger blur. It's **separable**: blur rows with a 1D kernel, then columns. That's much faster than a full 2D kernel.
- **Mean / box blur**: equal weights; cheaper but blocky.
- **Lens Blur**, **Variable Blur** (depth-of-field effects), **Motion Blur**.
- **Selective Gaussian** and **Bilateral** blur only average similar values, so edges stay sharp: a basic edge-preserving smoother.

## Sharpening

**Unsharp Mask** (*Filters → Enhance → Sharpen (Unsharp Mask)*): `sharpened = original + amount × (original − blurred)`. The difference image is the detail; adding it back increases local contrast at edges.
- **Radius**: size of detail affected (0.5–1.5 px for fine detail; 20–50 px with a low amount for "local contrast").
- **Amount**: strength (0.3–1.0 typical).
- **Threshold**: ignore differences smaller than this, so smooth areas (skin, sky) and noise aren't sharpened.
Over-sharpening creates **halos** (bright and dark outlines). Sharpen last, for the output size: an image downscaled for the web needs a little extra sharpening after scaling. Sharpening the luminance only (LAB L or a Value layer) avoids colour fringes.

## Noise

- **Luminance noise** (grain) and **colour noise** (blotches of colour), worse at high ISO and in shadows.
- **Median** filter (*Filters → Blur → Median Blur*): replaces each pixel with the median of its neighbours. Excellent for **salt-and-pepper** noise (isolated wrong pixels) and preserves edges better than a mean filter.
- **Noise Reduction** and **Selective Gaussian** for general grain; **Despeckle** (adaptive median) for dust.
- Reduce colour noise more aggressively than luminance noise (blur only the colour channels), and keep a little grain: too much smoothing looks plastic.
- The best noise reduction is in the raw developer (darktable, RawTherapee) before GIMP.

@@quiz
What does unsharp masking add to the original image?
* A scaled difference between the original and a blurred copy
- A blurred copy
- Random noise
- A brightness curve
? The difference holds the edge detail.

@@quiz
Which filter is best for salt-and-pepper noise?
- Gaussian blur
* Median blur
- Unsharp mask
- Motion blur
? Isolated outliers never become the median.

@@quiz
What does the Threshold setting in Unsharp Mask do?
- Sets the blur radius
* Skips small differences so smooth areas and noise aren't sharpened
- Limits the output brightness
- Chooses which layer to sharpen
? It protects skin and skies.

@@quiz
When should you sharpen an image for the web?
- Before any other edit
* Last, after scaling to the final size
- Never
- Before noise reduction
? Sharpening depends on output size.

@@quiz
Why is Gaussian blur called separable?
- It works on separate layers
* A 2D Gaussian equals a 1D blur on rows followed by one on columns
- It separates colour channels
- It splits the image in tiles
? Much faster for large radii.

@@flashcards
Gaussian weight :: exp(−x² / 2σ²)
separable blur :: rows then columns
box blur :: equal weights
bilateral / selective :: edge-preserving smoothing
unsharp mask :: original + amount × (original − blur)
radius :: detail size
amount :: strength
threshold :: ignore small differences
halo :: over-sharpening outline
sharpen last :: at output size
luminance noise :: grain
colour noise :: colour blotches
median filter :: middle value of neighbours
salt-and-pepper :: isolated wrong pixels
despeckle :: adaptive median

@@drill
Which blur filter replaces each pixel with the middle value of its neighbourhood?
= median
= median blur
? Robust against outliers.

@@drill
Unsharp mask: original + amount × (original − what)?
= blurred
= blur
= the blurred copy
= blurred copy
? The difference is the detail.

@@exercise Gaussian blur and unsharp mask
Build a normalised 1D Gaussian kernel for a given σ (radius = ceil(3σ)), blur a 1D edge profile with it (clamping at the borders), then apply unsharp masking with amount and threshold, and report the overshoot (halo) on each side of the edge.
@@starter
import math

profile = [40] * 8 + [200] * 8

def gaussian_kernel(sigma):
    raise NotImplementedError
@@expected
sigma 0.8: 7 taps, centre weight 0.499, sum 1.000
sigma 2.0: 13 taps, centre weight 0.200, sum 1.000
blurred: [40, 40, 40, 40, 40, 41, 49, 88, 152, 191, 199, 200, 200, 200, 200, 200]
USM sigma=1.0 amount=0.6 threshold=0: [40, 34, 11, 229, 206, 200] halo dark 29, bright 29
USM sigma=1.0 amount=1.5 threshold=0: [39, 26, 0, 255, 214, 201] halo dark 40, bright 55
USM sigma=3.0 amount=0.4 threshold=0: [27, 20, 12, 228, 220, 213] halo dark 28, bright 28
USM sigma=1.0 amount=1.5 threshold=30: [40, 40, 0, 255, 200, 200] halo dark 40, bright 55
@@requires
def gaussian_kernel
math.exp(
def unsharp
@@hint
`r = math.ceil(3 * sigma)`, weights `math.exp(-x * x / (2 * sigma * sigma))` for x in `-r..r`, divided by their sum. Convolve with `row[min(max(i + k, 0), n - 1)]`. Unsharp: `diff = o - b`; if `abs(diff) < threshold` keep `o`, else `o + amount * diff`, clamped to 0–255. Halo = min below 40 and max above 200.
@@solution
import math

profile = [40] * 8 + [200] * 8

def gaussian_kernel(sigma):
    r = math.ceil(3 * sigma)
    weights = [math.exp(-x * x / (2 * sigma * sigma)) for x in range(-r, r + 1)]
    total = sum(weights)
    return [w / total for w in weights]

def convolve(row, kernel):
    r, n = len(kernel) // 2, len(row)
    return [sum(w * row[min(max(i + k - r, 0), n - 1)] for k, w in enumerate(kernel)) for i in range(n)]

def unsharp(row, sigma, amount, threshold=0):
    blurred = convolve(row, gaussian_kernel(sigma))
    out = []
    for o, b in zip(row, blurred):
        diff = o - b
        value = o if abs(diff) < threshold else o + amount * diff
        out.append(round(min(255, max(0, value))))
    return out

for sigma in (0.8, 2.0):
    k = gaussian_kernel(sigma)
    print(f"sigma {sigma}: {len(k)} taps, centre weight {k[len(k) // 2]:.3f}, sum {sum(k):.3f}")
print("blurred:", [round(v) for v in convolve(profile, gaussian_kernel(1.0))])
for sigma, amount, threshold in ((1.0, 0.6, 0), (1.0, 1.5, 0), (3.0, 0.4, 0), (1.0, 1.5, 30)):
    sharp = unsharp(profile, sigma, amount, threshold)
    print(f"USM sigma={sigma} amount={amount} threshold={threshold}: {sharp[5:11]} halo dark {40 - min(sharp)}, bright {max(sharp) - 200}")

@@exercise Mean vs median against noise
Add salt-and-pepper noise to a small image with a sharp edge, then filter it with a 3 × 3 mean and a 3 × 3 median filter (borders clamped). Compare the mean absolute error against the clean image and whether the edge stays sharp.
@@starter
import random

clean = [[30 if x < 6 else 220 for x in range(12)] for _ in range(8)]

def add_noise(img, amount, seed=7):
    raise NotImplementedError
@@expected
clean      error   0.0  row 3:  30  30  30  30  30  30 220 220 220 220 220 220  edge [30, 220]
noisy      error  18.0  row 3: 255  30  30  30  30  30 255 220 220 220 220 220  edge [30, 255]
mean 3x3   error  25.3  row 3: 127  77  27  30  27  94 157 224 224 224 224 220  edge [94, 157]
median 3x3 error   0.3  row 3:  30  30  30  30  30  30 220 220 220 220 220 220  edge [30, 220]
@@requires
def add_noise
def median_filter
sorted(
@@hint
`add_noise`: copy the image; for each pixel, with probability `amount` set it to 0 or 255 (use `random.Random(seed)`). For both filters collect the 9 neighbours with clamped coordinates; the mean rounds the average, the median takes `sorted(values)[4]`. Error = mean of `abs(a - b)` over all pixels. Print row 3 of each image.
@@solution
import random

clean = [[30 if x < 6 else 220 for x in range(12)] for _ in range(8)]

def add_noise(img, amount, seed=7):
    rng = random.Random(seed)
    return [[(rng.choice((0, 255)) if rng.random() < amount else v) for v in row] for row in img]

def neighbours(img, y, x):
    h, w = len(img), len(img[0])
    return [img[min(max(y + dy, 0), h - 1)][min(max(x + dx, 0), w - 1)] for dy in (-1, 0, 1) for dx in (-1, 0, 1)]

def mean_filter(img):
    return [[round(sum(neighbours(img, y, x)) / 9) for x in range(len(img[0]))] for y in range(len(img))]

def median_filter(img):
    return [[sorted(neighbours(img, y, x))[4] for x in range(len(img[0]))] for y in range(len(img))]

def error(a, b):
    values = [abs(p - q) for ra, rb in zip(a, b) for p, q in zip(ra, rb)]
    return sum(values) / len(values)

noisy = add_noise(clean, 0.12)
for name, img in (("clean", clean), ("noisy", noisy), ("mean 3x3", mean_filter(noisy)), ("median 3x3", median_filter(noisy))):
    edge = img[3][5:7]
    print(f"{name:<10} error {error(img, clean):5.1f}  row 3: {' '.join(f'{v:>3}' for v in img[3])}  edge {edge}")
