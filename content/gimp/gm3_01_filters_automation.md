@@lesson gm3-filters Text, Filters, Painting and Automating GIMP
@@concepts lists, loops, automation
@@lang python
@@text
The last lesson rounds out the toolkit: text and graphics for posters and thumbnails, how filters really work, painting, and making GIMP do repetitive work for you with batch processing and Python plug-ins.

## Text

The **Text tool** (T) creates a **text layer**: click for text that grows, drag for a box with wrapping. Change font, size, colour, spacing and alignment in the tool options or the on-canvas box. Text stays editable until you paint on it or merge it.

- **Effects**: **Filters → Light and Shadow → Drop Shadow**, **Layer → Transparency → Alpha to Selection** then Select → Grow and fill on a layer below for an outline, or **Text along Path** (right-click the text layer) for curved text.
- **Fonts**: install into your user fonts folder and restart GIMP (or refresh the Fonts dialog). For Japanese, choose a font with CJK glyphs (Noto Sans JP, M PLUS, BIZ UDPGothic); vertical text is in the text options.
- Keep text crisp: design at the final pixel size, and avoid scaling text layers as pixels (change the font size instead).

## Painting and drawing

- **Paintbrush** (P) with **dynamics** (pressure, velocity) for drawing tablets; set up the tablet in **Edit → Input Devices**.
- **MyPaint Brush** tool: natural-media brushes.
- Straight lines: click, then **Shift+click** the end.
- Shapes: make a selection, then **Edit → Stroke Selection** (outline) or **Fill** (Ctrl+, / Ctrl+. / Ctrl+;); or draw with **Paths** and **Stroke Path** for vector-like precision.
- **Symmetry painting** (Windows → Dockable Dialogs → Symmetry Painting): mirrors and mandalas.
- **Gradients** (G): drag across a selection or a mask; Shift snaps angles.
- **Grid and guides**: drag guides from the rulers; **View → Snap to Guides/Grid**; **Image → Configure Grid**.

## How filters work

Many filters are **convolutions**: each output pixel is a weighted sum of the pixel and its neighbours, using a small grid of weights called a **kernel**:

```text
Box blur          Sharpen           Edge detect
1/9 1/9 1/9        0  -1   0        -1  -1  -1
1/9 1/9 1/9       -1   5  -1        -1   8  -1
1/9 1/9 1/9        0  -1   0        -1  -1  -1
```

Blur averages neighbours; sharpen boosts a pixel against its neighbours; edge detection is zero on flat areas. **Gaussian Blur** uses bell-curve weights; **Unsharp Mask** subtracts a blurred copy to exaggerate edges. At the image border there are no neighbours, so the edge pixels are repeated ("clamp"). The first exercise runs kernels by hand; GIMP's own **Filters → Generic → Convolution Matrix** lets you type a kernel too.

Filter menu highlights: **Blur** (Gaussian, Motion, Lens/Focus Blur with a mask for fake depth of field), **Enhance** (Sharpen, Noise Reduction, Red Eye), **Distorts** (Ripple, Whirl and Pinch, Lens Distortion), **Light and Shadow** (Drop Shadow, Lens Flare, Vignette), **Noise**, **Edge-Detect**, **Artistic** (Cartoon, Oilify, Apply Canvas), **Render** (Clouds, patterns, Gfig), **Map** (Bump Map, Displace, Tile Seamless), and **G'MIC** (a huge third-party filter collection). Filters are previewed live with a **split view** to compare before and after; in GIMP 3 they stay editable in the layer's filter list.

## Exporting for real destinations

| Destination | Size | Format |
|---|---|---|
| Website photo | longest side 1600–2400 px | WebP or JPEG ~82, sRGB |
| Social media | per platform (e.g. 1080 × 1350 portrait, 1200 × 630 link preview) | JPEG/PNG |
| YouTube thumbnail | 1280 × 720 | JPEG/PNG < 2 MB |
| App or favicon icon | 16, 32, 48, 256 px | ICO with layers / PNG |
| Print | full resolution at 300 PPI | TIFF or high-quality JPEG/PDF |

Strip metadata (GPS location!) in the export dialog when sharing photos publicly. **Image → Scale Image** with **NoHalo** or **LoHalo** interpolation gives the best downscaling. The second exercise scales images to fit a box, like a batch export would.

## Automating GIMP

- **Filters → Repeat Last** (Ctrl+F) and **Re-Show Last** (Shift+Ctrl+F).
- **Batch processing**: the **BIMP** plug-in (batch resize, watermark, rename, convert), or from the terminal: GIMP 3 can run without a window: `gimp-console-3.0 -i --batch-interpreter python-fu-eval -b "..."`. For simple conversions, **ImageMagick** (`magick in.jpg -resize 1600x1600 out.webp`) is often easier.
- **Plug-ins in Python 3** (GIMP 3): a Python file in your plug-ins folder (**Edit → Preferences → Folders → Plug-ins**, each plug-in in its own sub-folder with the same name, executable on Linux/macOS) that registers a procedure and appears in the menus. The **Python-Fu console** (Filters → Development → Python-Fu → Console) runs code live against open images:

```python,norun
# In the Python-Fu console: halve every open image and export it as WebP
from gi.repository import Gimp, Gio

for image in Gimp.get_images():
    w, h = image.get_width(), image.get_height()
    image.scale(w // 2, h // 2)
    path = image.get_file().get_path().rsplit(".", 1)[0] + "-small.webp"
    Gimp.file_save(Gimp.RunMode.NONINTERACTIVE, image, Gio.File.new_for_path(path), None)
```

- **Script-Fu** (Scheme) is the older scripting language; many existing scripts use it, and it still works in GIMP 3.

The third exercise plans a batch job: names, sizes and a watermark position, the logic you'd put inside such a plug-in.

## Where to go next

Practise with real projects: restore an old family photo, make a poster with text and a photo, cut a product out onto a white background, design a YouTube thumbnail, paint a mandala with symmetry. **Krita** is the painting-focused cousin of GIMP; **Inkscape** handles vector graphics (logos, icons, SVG); **darktable** and **RawTherapee** process RAW camera files before GIMP. And the **GUI Design & Creation** course in Extras uses these skills for app icons.

@@quiz
Why does a sharpen kernel like `0 -1 0 / -1 5 -1 / 0 -1 0` leave flat areas unchanged?
* The weights add up to 1: on a flat area the neighbours cancel and the pixel keeps its value
- Because it only works on edges marked by the user
- It doesn't; it brightens everything
- GIMP skips flat areas
? Edge detection kernels add up to 0, so flat areas go black.

@@quiz
You want to change some words on a poster you made yesterday. What should you have kept?
- The exported JPEG
* The XCF file with the text layer still editable
- A screenshot
- The PNG with transparency
? Text layers stay editable until they are converted or merged.

@@quiz
Before posting a holiday photo publicly, what should you remove in the export dialog?
- The colour profile
* The metadata, including the GPS location
- The alpha channel
- The thumbnail
? Exif data can include where and when the photo was taken, and the camera serial.

@@quiz
Which tool is best for resizing and converting 500 photos to WebP?
- Doing them one by one in GIMP
* A batch tool: BIMP, a GIMP Python script, or ImageMagick in the terminal
- Filters → Repeat Last
- The Clone tool
? Automation pays off after the second or third repetition.

@@quiz
Where do GIMP 3 Python plug-ins go?
- Anywhere; GIMP scans the whole disk
* In a sub-folder of the plug-ins folder (see Preferences → Folders → Plug-ins), named like the .py file, executable on Linux and macOS
- Inside the XCF file
- In /tmp
? The plug-in registers procedures that then appear in the menus.

@@drill
What's the name of GIMP's built-in Scheme scripting language?
= Script-Fu
= script fu
? Python-Fu is the Python equivalent.

@@flashcards
text layer :: editable text until merged or painted on
Text along Path :: curved text
Shift+click with a brush :: straight line
Stroke Selection / Stroke Path :: draw an outline along it
Symmetry Painting :: mirrored brush strokes
convolution :: each pixel becomes a weighted sum of its neighbours
kernel :: the grid of weights for a convolution
box blur :: all weights equal, summing to 1
sharpen kernel :: centre weight above 1, neighbours negative
edge detect :: weights sum to 0
Unsharp Mask :: subtract a blurred copy to strengthen edges
Convolution Matrix :: GIMP filter with your own kernel
G'MIC :: big third-party filter collection
NoHalo / LoHalo :: high-quality downscaling interpolation
strip metadata :: remove GPS and camera info when exporting
BIMP :: batch image manipulation plug-in
Python-Fu console :: run Python against open images
Script-Fu :: Scheme scripting in GIMP
Krita / Inkscape / darktable :: painting / vector / RAW companions to GIMP

@@exercise Run a convolution kernel
An image is a list of rows of grey values 0–255. Write `convolve(image, kernel)` for a 3 × 3 kernel: each output pixel is the sum of `kernel[dy + 1][dx + 1] * pixel(x + dx, y + dy)` for `dx, dy` in −1..1, where pixels outside the image use the nearest edge pixel (clamp the coordinates). Round the result and clamp it to 0–255.
@@starter
BOX = [[1 / 9] * 3 for _ in range(3)]
SHARPEN = [[0, -1, 0], [-1, 5, -1], [0, -1, 0]]
EDGE = [[-1, -1, -1], [-1, 8, -1], [-1, -1, -1]]


def convolve(image, kernel):
    pass


img = [[10, 10, 10],
       [10, 100, 10],
       [10, 10, 10]]
for name, k in (("box blur", BOX), ("sharpen", SHARPEN), ("edges", EDGE)):
    print(name, convolve(img, k))
@@tests
img = [[10, 10, 10],
       [10, 100, 10],
       [10, 10, 10]]
check(convolve(img, BOX), [[20, 20, 20], [20, 20, 20], [20, 20, 20]], "a bright dot spreads out")
check(convolve(img, SHARPEN), [[10, 0, 10], [0, 255, 0], [10, 0, 10]], "sharpen exaggerates and clamps")
check(convolve(img, EDGE)[1][1], 255, "the dot is an edge")
check(convolve([[50, 50], [50, 50]], SHARPEN), [[50, 50], [50, 50]], "flat stays flat")
check(convolve([[50, 50], [50, 50]], EDGE), [[0, 0], [0, 0]], "no edges in a flat image")
check(convolve([[0, 0, 255, 255]], EDGE), [[0, 0, 255, 0]], "only the bright side of the edge lights up")
@@hint
Write a helper `px(x, y)` that clamps `x` to `0..width-1` and `y` to `0..height-1` with `max(0, min(...))`. Then loop `y` over the rows and `x` over the columns, add up the nine products, and append `max(0, min(255, round(total)))`.
@@solution
BOX = [[1 / 9] * 3 for _ in range(3)]
SHARPEN = [[0, -1, 0], [-1, 5, -1], [0, -1, 0]]
EDGE = [[-1, -1, -1], [-1, 8, -1], [-1, -1, -1]]


def convolve(image, kernel):
    height, width = len(image), len(image[0])

    def px(x, y):
        return image[max(0, min(height - 1, y))][max(0, min(width - 1, x))]

    out = []
    for y in range(height):
        row = []
        for x in range(width):
            total = 0
            for dy in (-1, 0, 1):
                for dx in (-1, 0, 1):
                    total += kernel[dy + 1][dx + 1] * px(x + dx, y + dy)
            row.append(max(0, min(255, round(total))))
        out.append(row)
    return out


img = [[10, 10, 10],
       [10, 100, 10],
       [10, 10, 10]]
for name, k in (("box blur", BOX), ("sharpen", SHARPEN), ("edges", EDGE)):
    print(name, convolve(img, k))

@@exercise Scale to fit
Write `fit(width, height, max_w, max_h, enlarge=False)`: the largest size that fits inside the box keeping the aspect ratio, each side rounded to whole pixels; unless `enlarge` is true, images already smaller are returned unchanged. Then `cover(width, height, box_w, box_h)`: the size to scale to so the image **fills** the box (one side matches, the other is bigger, for cropping afterwards), and `crop_offset(scaled_w, scaled_h, box_w, box_h)`: the `(x, y)` offset that centres the crop (rounded down).
@@starter
def fit(width, height, max_w, max_h, enlarge=False):
    pass


def cover(width, height, box_w, box_h):
    pass


def crop_offset(scaled_w, scaled_h, box_w, box_h):
    pass


print(fit(4000, 3000, 1920, 1080))
w, h = cover(4000, 3000, 1280, 720)
print((w, h), crop_offset(w, h, 1280, 720))
@@tests
check(fit(4000, 3000, 1920, 1080), (1440, 1080), "landscape photo into full HD")
check(fit(3000, 4000, 1600, 1600), (1200, 1600), "portrait into a square box")
check(fit(800, 600, 1920, 1080), (800, 600), "never enlarge by default")
check(fit(800, 600, 1600, 1600, enlarge=True), (1600, 1200), "enlarge when asked")
check(cover(4000, 3000, 1280, 720), (1280, 960), "fill a YouTube thumbnail")
check(cover(1000, 3000, 1080, 1080), (1080, 3240), "a tall image fills a square")
check(crop_offset(1280, 960, 1280, 720), (0, 120), "centre crop")
check(crop_offset(1081, 1080, 1080, 1080), (0, 0), "rounded down")
@@hint
`scale = min(max_w / width, max_h / height)` for fit (and `min(scale, 1)` unless enlarging); `max(...)` of the two ratios for cover. The offset is `((scaled_w - box_w) // 2, (scaled_h - box_h) // 2)`.
@@solution
def fit(width, height, max_w, max_h, enlarge=False):
    scale = min(max_w / width, max_h / height)
    if not enlarge:
        scale = min(scale, 1)
    return round(width * scale), round(height * scale)


def cover(width, height, box_w, box_h):
    scale = max(box_w / width, box_h / height)
    return round(width * scale), round(height * scale)


def crop_offset(scaled_w, scaled_h, box_w, box_h):
    return (scaled_w - box_w) // 2, (scaled_h - box_h) // 2


print(fit(4000, 3000, 1920, 1080))
w, h = cover(4000, 3000, 1280, 720)
print((w, h), crop_offset(w, h, 1280, 720))

@@exercise Plan a batch export
Write `batch_plan(files, max_side, fmt, prefix="web-")` for a list of `(name, width, height)` tuples: for each, return `(new_name, new_w, new_h)` where the name gets the prefix and the extension is replaced by `fmt` (`"IMG_0042.JPG"` → `"web-IMG_0042.webp"`), and the size is fitted so the **longest side** is at most `max_side` (use your rounding: `round`). Skip files that aren't images (`.jpg .jpeg .png .tif .tiff .webp`, any case). Then `watermark_pos(img_w, img_h, mark_w, mark_h, margin)`: the top-left `(x, y)` to put a watermark in the bottom-right corner with that margin.
@@starter
IMAGE_EXTS = {".jpg", ".jpeg", ".png", ".tif", ".tiff", ".webp"}


def batch_plan(files, max_side, fmt, prefix="web-"):
    pass


def watermark_pos(img_w, img_h, mark_w, mark_h, margin):
    pass


files = [("IMG_0042.JPG", 6000, 4000), ("scan.tiff", 2480, 3508), ("notes.txt", 0, 0), ("logo.png", 512, 512)]
for row in batch_plan(files, 2048, "webp"):
    print(row)
print(watermark_pos(2048, 1365, 200, 60, 24))
@@tests
files = [("IMG_0042.JPG", 6000, 4000), ("scan.tiff", 2480, 3508), ("notes.txt", 0, 0), ("logo.png", 512, 512)]
check(batch_plan(files, 2048, "webp"), [("web-IMG_0042.webp", 2048, 1365), ("web-scan.webp", 1448, 2048), ("web-logo.webp", 512, 512)], "the plan")
check(batch_plan([("a.b.Jpeg", 100, 50)], 50, "png", prefix=""), [("a.b.png", 50, 25)], "only the last extension, no prefix")
check(watermark_pos(2048, 1365, 200, 60, 24), (1824, 1281), "bottom right")
check(watermark_pos(100, 100, 100, 100, 0), (0, 0), "a full-size mark")
@@hint
`stem, dot, ext = name.rpartition(".")` splits off the last extension; check `("." + ext).lower() in IMAGE_EXTS`. The scale is `min(1, max_side / max(w, h))`. The watermark goes at `(img_w - mark_w - margin, img_h - mark_h - margin)`.
@@solution
IMAGE_EXTS = {".jpg", ".jpeg", ".png", ".tif", ".tiff", ".webp"}


def batch_plan(files, max_side, fmt, prefix="web-"):
    plan = []
    for name, w, h in files:
        stem, dot, ext = name.rpartition(".")
        if not dot or ("." + ext).lower() not in IMAGE_EXTS:
            continue
        scale = min(1, max_side / max(w, h))
        plan.append((f"{prefix}{stem}.{fmt}", round(w * scale), round(h * scale)))
    return plan


def watermark_pos(img_w, img_h, mark_w, mark_h, margin):
    return img_w - mark_w - margin, img_h - mark_h - margin


files = [("IMG_0042.JPG", 6000, 4000), ("scan.tiff", 2480, 3508), ("notes.txt", 0, 0), ("logo.png", 512, 512)]
for row in batch_plan(files, 2048, "webp"):
    print(row)
print(watermark_pos(2048, 1365, 200, 60, 24))
