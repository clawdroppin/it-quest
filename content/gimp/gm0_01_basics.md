@@lesson gm0-basics GIMP Basics: Interface, Images and Exporting
@@concepts numbers, conditions
@@lang python
@@text
**GIMP** (GNU Image Manipulation Program) is a free, open-source image editor for Linux, Windows and macOS: photo retouching, compositing, digital painting, graphics for the web and print. **GIMP 3** (2025) brought a modern GTK 3 interface, non-destructive filters, multi-layer selection, better colour management and Python 3 plug-ins. This course teaches GIMP itself, and each lesson's exercises rebuild a piece of what GIMP does to pixels in Python, so the sliders stop being magic.

Install it from your package manager or Flathub (`flatpak install flathub org.gimp.GIMP`), the Microsoft Store or gimp.org on Windows, or gimp.org on macOS.

## The window

| Part | What it's for |
|---|---|
| **Toolbox** (left) | the tools; tools are **grouped**: click and hold (or right-click) a group to see all of them |
| **Tool Options** (below the toolbox) | settings for the active tool: size, opacity, mode, feather... always check them |
| **Canvas** (middle) | the image; tabs at the top when several are open |
| **Layers / Channels / Paths** (right) | the layer stack and more |
| **Brushes / Patterns / Fonts** (right) | resources |
| Menus | **Image** (whole image: size, canvas, mode), **Layer** (current layer), **Colors**, **Filters** |

Lost a panel? **Windows → Dockable Dialogs**, or **Windows → Hide Docks** (Tab) toggled by accident. Reset everything in **Edit → Preferences → Interface → Reset Saved Window Positions**. Many people prefer **Windows → Single-Window Mode** (on by default now).

## Essential tools and shortcuts

| Tool | Key | Use |
|---|---|---|
| Move | M | move layers, selections or guides (check "Move the active layer" in options) |
| Rectangle / Ellipse select | R / E | select areas |
| Free select (lasso) | F | click points or drag freehand |
| Fuzzy select (magic wand) | U | similar colours touching the click |
| Paths | B | precise Bézier selections and shapes |
| Crop | Shift+C | cut the image down |
| Unified Transform | Shift+T | scale, rotate, shear and perspective at once |
| Paintbrush / Pencil / Eraser | P / N / Shift+E | painting |
| Bucket fill / Gradient | Shift+B / G | fill areas |
| Healing / Clone | H / C | retouching |
| Text | T | text layers |
| Color picker | O | sample a colour |
| Zoom | Z, or 1 for 100%, Shift+Ctrl+J to fit | |

Also: **X** swaps foreground and background colours, **D** resets them to black and white, **[ ]** change the brush size, **Ctrl+Z** undo (the **Undo History** dialog goes back many steps), **Space + drag** pans.

## Pixels, size and resolution

A digital image is a grid of **pixels**. Three different things get called "size":

- **Image → Scale Image**: changes the number of pixels (resamples). Shrinking loses detail; enlarging can't add any (it just blurs).
- **Image → Canvas Size**: changes the area without scaling the content: adds a border or trims.
- **Image → Print Size**: changes only the **resolution** (PPI/DPI) metadata: how big it prints. The pixels stay the same.

Print size = pixels ÷ PPI. A 3000 × 2000 photo at **300 PPI** (good print quality) prints at 10 × 6.67 inches = 25.4 × 16.9 cm. For screens only the pixel size matters; "72 DPI for the web" is a myth. The first exercise does this arithmetic.

Layer size and canvas size are separate too: a layer can be bigger or smaller than the canvas (the yellow-black dashed line shows a layer's edge). **Layer → Layer to Image Size** fixes odd edges.

## Image modes and precision

- **Image → Mode**: **RGB** (normal), **Grayscale**, **Indexed** (a palette of up to 256 colours: GIF, some PNGs).
- **Image → Precision**: 8-bit integer (standard JPEG/PNG), 16 or 32-bit (more headroom for heavy editing of RAW photos; GIMP opens RAW through darktable or RawTherapee).
- **Colour management**: images can carry an ICC profile (sRGB for the web; Display P3, Adobe RGB for wide-gamut work). When GIMP asks, usually keep the embedded profile.

## Save versus Export

- **File → Save** writes **XCF**, GIMP's own format: every layer, mask, path, text layer, channel and guide preserved. Always keep an XCF of work in progress.
- **File → Export As** (Shift+Ctrl+E) writes a flat file for everyone else: pick the format by the extension you type. **Overwrite** (Ctrl+E) re-exports to the same file.

| Format | Use for | Notes |
|---|---|---|
| **JPEG** | photos | lossy; quality 85–92 is a good balance; no transparency |
| **PNG** | graphics, screenshots, anything with transparency | lossless; larger for photos |
| **WebP** / **AVIF** | web images | much smaller; lossy or lossless, transparency |
| **GIF** | simple animations | 256 colours only |
| **TIFF** | print and archives | lossless, layers possible, 16-bit |
| **PSD** | exchanging with Photoshop | GIMP opens and exports layers |
| **PDF** | documents, print | **File → Export As → .pdf**, layers as pages optional |
| **ICO** | Windows icons | several sizes as layers |

The second exercise picks a format the way you should.

@@quiz
You need a photo to print at 20 × 15 cm at 300 PPI. What resolution must it have, roughly?
- 800 × 600 pixels
* About 2362 × 1772 pixels
- 300 × 300 pixels
- Any size; change the print size in GIMP
? 20 cm = 7.87 in × 300 = 2362 px; 15 cm = 5.91 in × 300 = 1772 px.

@@quiz
What's the difference between Image → Scale Image and Image → Canvas Size?
* Scale Image resamples the pixels (resizes the content); Canvas Size changes the area around the content without scaling it
- They do the same thing
- Canvas Size only changes the DPI
- Scale Image only works on layers
? Print Size changes only the PPI metadata.

@@quiz
You want to keep your layers and continue editing tomorrow. Which command?
- File → Export As → JPEG
* File → Save (XCF)
- File → Overwrite
- Image → Flatten, then Save
? Export writes a flat copy for other apps; XCF keeps everything.

@@quiz
Which format for a logo with a transparent background?
- JPEG
* PNG (or WebP)
- BMP
- GIF at quality 90
? JPEG has no transparency.

@@quiz
You clicked something and all the panels vanished. What probably happened?
- GIMP crashed
* Tab hides the docks; press Tab again (or Windows → Hide Docks)
- The image is in Indexed mode
- The XCF is corrupted
? Windows → Dockable Dialogs reopens any closed panel.

@@flashcards
GIMP 3 :: GTK 3 interface, non-destructive filters, Python 3 plug-ins
Tool Options :: settings of the active tool, check them first
Tab :: hide or show the docks
D / X :: default colours / swap foreground and background
[ and ] :: smaller / bigger brush
Scale Image :: resample: change the number of pixels
Canvas Size :: change the area without scaling
Print Size :: change PPI only
print size :: pixels ÷ PPI
300 PPI :: good print quality
XCF :: GIMP's format with all layers
Export As (Shift+Ctrl+E) :: write JPEG, PNG, WebP...
JPEG :: lossy photos, no transparency
PNG :: lossless, transparency
WebP / AVIF :: small modern web formats
Indexed mode :: up to 256 palette colours
sRGB :: the standard colour profile for screens and the web

@@exercise Print size calculator
Write `print_size_cm(width_px, height_px, ppi)` returning `(width_cm, height_cm)` rounded to 1 decimal (1 inch = 2.54 cm), `pixels_for(width_cm, height_cm, ppi)` returning the pixel size needed, rounded to whole pixels, and `max_ppi(width_px, height_px, width_cm, height_cm)`: the highest PPI the image can print at for that size without enlarging, rounded down to a whole number (the smaller of the two directions).
@@starter
def print_size_cm(width_px, height_px, ppi):
    pass


def pixels_for(width_cm, height_cm, ppi):
    pass


def max_ppi(width_px, height_px, width_cm, height_cm):
    pass


print(print_size_cm(3000, 2000, 300))
print(pixels_for(20, 15, 300))
print(max_ppi(4000, 3000, 30, 20), "PPI on a 30 x 20 cm print")
@@tests
check(print_size_cm(3000, 2000, 300), (25.4, 16.9), "a 6 MP photo at 300 PPI")
check(print_size_cm(1920, 1080, 120), (40.6, 22.9), "a screenshot at 120 PPI")
check(pixels_for(20, 15, 300), (2362, 1772), "20 x 15 cm at 300 PPI")
check(pixels_for(2.54, 2.54, 600), (600, 600), "one inch")
check(max_ppi(4000, 3000, 30, 20), 338, "limited by the width")
check(max_ppi(3000, 3000, 10, 25), 304, "limited by the height")
@@hint
Inches are `cm / 2.54`. Print size: `px / ppi * 2.54`. Pixels: `cm / 2.54 * ppi`. Max PPI: `min(width_px / (width_cm / 2.54), height_px / (height_cm / 2.54))`, then `int()`.
@@solution
def print_size_cm(width_px, height_px, ppi):
    return round(width_px / ppi * 2.54, 1), round(height_px / ppi * 2.54, 1)


def pixels_for(width_cm, height_cm, ppi):
    return round(width_cm / 2.54 * ppi), round(height_cm / 2.54 * ppi)


def max_ppi(width_px, height_px, width_cm, height_cm):
    return int(min(width_px / (width_cm / 2.54), height_px / (height_cm / 2.54)))


print(print_size_cm(3000, 2000, 300))
print(pixels_for(20, 15, 300))
print(max_ppi(4000, 3000, 30, 20), "PPI on a 30 x 20 cm print")

@@exercise Pick an export format
Write `export_format(kind, transparent=False, animated=False, for_web=False, editing=False)` with these rules, checked in order: `editing` → `"xcf"`; `animated` → `"webp"` if `for_web` else `"gif"`; `kind == "photo"` → `"webp"` if `for_web` else (`"png"` if `transparent` else `"jpeg"`); `kind == "print"` → `"tiff"`; anything else (graphics, logos, screenshots) → `"webp"` if `for_web` else `"png"`. Then `jpeg_quality(purpose)`: `"web"` → 82, `"share"` → 88, `"archive"` → 95, else 90.
@@starter
def export_format(kind, transparent=False, animated=False, for_web=False, editing=False):
    pass


def jpeg_quality(purpose):
    pass


print(export_format("photo"), jpeg_quality("share"))
print(export_format("logo", transparent=True))
print(export_format("photo", for_web=True))
@@tests
check(export_format("photo"), "jpeg", "a photo")
check(export_format("photo", transparent=True), "png", "a cut-out photo")
check(export_format("photo", for_web=True), "webp", "a photo for a website")
check(export_format("logo", transparent=True), "png", "a logo")
check(export_format("screenshot", for_web=True), "webp", "a screenshot for the web")
check(export_format("sticker", animated=True), "gif", "an animation")
check(export_format("sticker", animated=True, for_web=True), "webp", "a web animation")
check(export_format("print"), "tiff", "print")
check(export_format("photo", editing=True), "xcf", "keep editing")
check([jpeg_quality(p) for p in ("web", "share", "archive", "other")], [82, 88, 95, 90], "quality")
@@hint
Write the rules as `if` statements in exactly the given order, returning as soon as one applies. A dict works well for `jpeg_quality`: `{"web": 82, ...}.get(purpose, 90)`.
@@solution
def export_format(kind, transparent=False, animated=False, for_web=False, editing=False):
    if editing:
        return "xcf"
    if animated:
        return "webp" if for_web else "gif"
    if kind == "photo":
        if for_web:
            return "webp"
        return "png" if transparent else "jpeg"
    if kind == "print":
        return "tiff"
    return "webp" if for_web else "png"


def jpeg_quality(purpose):
    return {"web": 82, "share": 88, "archive": 95}.get(purpose, 90)


print(export_format("photo"), jpeg_quality("share"))
print(export_format("logo", transparent=True))
print(export_format("photo", for_web=True))
