@@lesson gm0-formats Image Data: Bit Depth, Compression, Formats and Metadata
@@concepts numbers, lists, maps
@@lang python
@@text
Why is one screenshot 200 KB as PNG and 2 MB as JPEG, while a photo is the other way round? Why do some images band when you edit them, and why can a holiday photo reveal your home address? This lesson explains what's inside an image file: bit depth and channels, lossless and lossy compression, the main formats GIMP exports, and metadata.

## Pixels in memory

Uncompressed size = width × height × channels × bytes per channel.
- **Channels**: grayscale 1, grayscale + alpha 2, RGB 3, RGBA 4.
- **Bit depth** per channel: 8-bit (256 levels), 16-bit (65,536), 32-bit float (HDR, unbounded). GIMP's *Image → Precision* sets it. Edit in 16-bit or 32-bit to avoid **banding** and posterisation after strong Curves; export to 8-bit at the end.
- **Indexed** mode stores a palette of up to 256 colours and one index per pixel (GIF, small PNGs).

A 6000 × 4000 RGB photo at 8 bits is 72 MB uncompressed, 144 MB at 16 bits.

## Lossless vs lossy

- **Lossless** (PNG, WebP lossless, TIFF with LZW/ZIP, QOI): the decoded pixels are identical. Works best on flat colours, text and screenshots: long runs and repeated patterns compress well (PNG uses per-row prediction filters plus DEFLATE).
- **Lossy** (JPEG, WebP, AVIF, JPEG XL lossy): discards detail people don't notice. JPEG converts to **YCbCr**, often **subsamples chroma** (4:2:0 keeps colour at quarter resolution), splits into 8 × 8 blocks, applies the DCT and quantises (the **quality** slider). Great for photos; produces ringing around sharp edges and text.
- Re-saving a JPEG loses a little each time (**generation loss**). Keep the XCF or a lossless master.

## Formats at a glance

| Format | Use | Alpha | Notes |
|---|---|---|---|
| **XCF** | GIMP working file | yes | layers, masks, paths; not for sharing |
| **PNG** | screenshots, UI, logos, transparency | yes | lossless, 8/16-bit |
| **JPEG** | photos | no | quality 80–90 is a good default |
| **WebP** | web images | yes | lossy or lossless; widely supported |
| **AVIF** | web photos | yes | smaller than JPEG at the same quality; slower to encode |
| **JPEG XL** | archival and photography | yes | lossless JPEG recompression; browser support growing |
| **GIF** | legacy animation | 1-bit | 256 colours; prefer WebP/AVIF/video |
| **TIFF** | print, scanning | yes | layers, 16/32-bit, many compressions |
| **SVG** | vector | yes | GIMP only imports it (rasterises) |

## Metadata

- **EXIF** (camera, settings, date, orientation, **GPS location**), **IPTC** (captions, copyright), **XMP** (edit history, ratings), and an embedded **ICC colour profile**.
- The **Orientation** tag tells viewers to rotate; GIMP offers to apply it on import. Some apps ignore it, so photos appear sideways.
- **Privacy**: strip GPS (and often all EXIF) before posting photos publicly. In GIMP's export dialog untick Exif/XMP/IPTC, or keep only copyright. Keep the colour profile so colours display correctly.

@@quiz
How big is a 4000 × 3000 RGBA image at 8 bits per channel, uncompressed?
- 12 MB
- 36 MB
* 48 MB
- 96 MB
? 4000 × 3000 × 4 bytes.

@@quiz
Which format suits a screenshot with text and flat colours?
* PNG
- JPEG
- GIF with dithering
- BMP
? Lossless, and flat areas compress very well.

@@quiz
What does 4:2:0 chroma subsampling do?
- Halves the brightness resolution
* Stores colour at a quarter of the resolution while keeping full brightness detail
- Removes the alpha channel
- Converts to grayscale
? Eyes are less sensitive to colour detail.

@@quiz
Why edit in 16-bit precision?
- Files are smaller
* Strong adjustments cause less banding and posterisation
- Exports are faster
- JPEG requires it
? More levels between black and white.

@@quiz
Which metadata should you usually remove before posting a photo publicly?
- The ICC profile
* GPS location in EXIF
- The width and height
- The alpha channel
? Location can reveal where you live.

@@flashcards
uncompressed size :: w × h × channels × bytes
8-bit channel :: 256 levels
16-bit channel :: 65,536 levels
indexed mode :: palette of up to 256 colours
banding :: visible steps in gradients
lossless :: identical pixels after decoding
lossy :: discards unnoticed detail
PNG :: prediction filters + DEFLATE
JPEG pipeline :: YCbCr, subsample, DCT, quantise
4:2:0 :: quarter-resolution colour
generation loss :: re-saving lossy files
XCF :: GIMP's layered working format
WebP / AVIF :: modern web formats
EXIF :: camera data, GPS, orientation
ICC profile :: colour space description

@@drill
How many levels does one 8-bit channel have?
= 256
? 2^8.

@@drill
What is GIMP's native working file format?
= XCF
= .xcf
? It keeps layers, masks and paths.

@@exercise Size up images
Compute the uncompressed size of several images from width, height, channels and bit depth, then run-length encode one row of a flat UI image and one row of a noisy photo to show why lossless compression works on one and not the other. Finally choose an export format from the image's properties.
@@starter
import random

images = [
    ("phone photo", 4032, 3024, 3, 8),
    ("raw edit", 6000, 4000, 3, 16),
    ("UI icon", 512, 512, 4, 8),
    ("HDR render", 3840, 2160, 4, 32),
]

def size_mb(w, h, channels, bits):
    raise NotImplementedError
@@expected
phone photo: 4032x3024, 3 channels, 8-bit = 34.9 MiB uncompressed
raw edit: 6000x4000, 3 channels, 16-bit = 137.3 MiB uncompressed
UI icon: 512x512, 4 channels, 8-bit = 1.0 MiB uncompressed
HDR render: 3840x2160, 4 channels, 32-bit = 126.6 MiB uncompressed
flat UI row: 100 bytes -> 3 runs = 6 bytes (6%)
noisy photo row: 100 bytes -> 90 runs = 180 bytes (180%)
photo=True  alpha=False editing=False web=False -> JPEG
photo=True  alpha=False editing=False web=True  -> WebP or AVIF (JPEG fallback)
photo=False alpha=True  editing=False web=True  -> PNG
photo=True  alpha=True  editing=False web=True  -> WebP or AVIF
photo=True  alpha=False editing=True  web=False -> XCF
@@requires
def size_mb
def rle
1024
@@hint
Bytes = `w * h * channels * bits // 8`; show MiB with `/ 1024 / 1024`. `rle(row)` returns a list of `(value, count)` runs. For the noisy row use `random.seed(1)` and values like `120 + random.randint(-6, 6)`. Format choice: transparency and flat → PNG; photo → JPEG (or WebP/AVIF for the web); anything still being edited → XCF.
@@solution
import random

images = [
    ("phone photo", 4032, 3024, 3, 8),
    ("raw edit", 6000, 4000, 3, 16),
    ("UI icon", 512, 512, 4, 8),
    ("HDR render", 3840, 2160, 4, 32),
]

def size_mb(w, h, channels, bits):
    return w * h * channels * bits // 8 / 1024 / 1024

def rle(row):
    runs = []
    for value in row:
        if runs and runs[-1][0] == value:
            runs[-1][1] += 1
        else:
            runs.append([value, 1])
    return [tuple(r) for r in runs]

for name, w, h, c, bits in images:
    print(f"{name}: {w}x{h}, {c} channels, {bits}-bit = {size_mb(w, h, c, bits):.1f} MiB uncompressed")

flat = [255] * 40 + [33] * 8 + [255] * 52
random.seed(1)
noisy = [120 + random.randint(-6, 6) for _ in range(100)]
for name, row in (("flat UI row", flat), ("noisy photo row", noisy)):
    runs = rle(row)
    print(f"{name}: {len(row)} bytes -> {len(runs)} runs = {len(runs) * 2} bytes ({len(runs) * 2 / len(row):.0%})")

def choose(photo, transparency, still_editing, for_web):
    if still_editing:
        return "XCF"
    if transparency and not photo:
        return "PNG"
    if photo and for_web:
        return "WebP or AVIF (JPEG fallback)" if not transparency else "WebP or AVIF"
    return "JPEG" if photo else "PNG"

for case in [(True, False, False, False), (True, False, False, True), (False, True, False, True), (True, True, False, True), (True, False, True, False)]:
    print(f"photo={case[0]!s:<5} alpha={case[1]!s:<5} editing={case[2]!s:<5} web={case[3]!s:<5} -> {choose(*case)}")

@@exercise Fix orientation and strip private metadata
Each photo has EXIF tags. Apply the Orientation tag (1 = normal, 3 = 180°, 6 = rotate 90° clockwise, 8 = rotate 90° counter-clockwise) to report the displayed size, then produce a cleaned tag set for public sharing: drop GPS and serial numbers, keep copyright and the colour profile, and reset Orientation to 1 because the pixels were rotated.
@@starter
photos = {
    "IMG_0412.jpg": {"Width": 4032, "Height": 3024, "Orientation": 6, "Make": "Canon", "BodySerialNumber": "0412998",
                     "GPSLatitude": 35.6586, "GPSLongitude": 139.7454, "Copyright": "Aiko Sato", "ICCProfile": "sRGB"},
    "beach.jpg": {"Width": 4000, "Height": 3000, "Orientation": 1, "Make": "Pixel", "GPSLatitude": 26.2, "GPSLongitude": 127.7,
                  "DateTimeOriginal": "2026:08:14 10:22:01", "ICCProfile": "Display P3"},
    "scan.jpg": {"Width": 2480, "Height": 3508, "Orientation": 3, "Software": "SANE"},
}
PRIVATE_PREFIXES = ("GPS",)
PRIVATE_TAGS = {"BodySerialNumber", "LensSerialNumber", "OwnerName"}

def displayed_size(tags):
    raise NotImplementedError
@@expected
IMG_0412.jpg: rotate 90° clockwise, shown as 3024x4032
  removed: BodySerialNumber, GPSLatitude, GPSLongitude
  kept: Copyright=Aiko Sato, Height=4032, ICCProfile=sRGB, Make=Canon, Orientation=1, Width=3024
beach.jpg: none, shown as 4000x3000
  removed: GPSLatitude, GPSLongitude
  kept: DateTimeOriginal=2026:08:14 10:22:01, Height=3000, ICCProfile=Display P3, Make=Pixel, Orientation=1, Width=4000
scan.jpg: rotate 180°, shown as 2480x3508
  removed: nothing
  kept: Height=3508, Orientation=1, Software=SANE, Width=2480
@@requires
def displayed_size
def clean
PRIVATE_PREFIXES
@@hint
Orientations 5–8 swap width and height. `clean` builds a new dict without tags whose name starts with a private prefix or is in `PRIVATE_TAGS`, stores the displayed width and height, and sets `Orientation` to 1. Print what was removed.
@@solution
photos = {
    "IMG_0412.jpg": {"Width": 4032, "Height": 3024, "Orientation": 6, "Make": "Canon", "BodySerialNumber": "0412998",
                     "GPSLatitude": 35.6586, "GPSLongitude": 139.7454, "Copyright": "Aiko Sato", "ICCProfile": "sRGB"},
    "beach.jpg": {"Width": 4000, "Height": 3000, "Orientation": 1, "Make": "Pixel", "GPSLatitude": 26.2, "GPSLongitude": 127.7,
                  "DateTimeOriginal": "2026:08:14 10:22:01", "ICCProfile": "Display P3"},
    "scan.jpg": {"Width": 2480, "Height": 3508, "Orientation": 3, "Software": "SANE"},
}
PRIVATE_PREFIXES = ("GPS",)
PRIVATE_TAGS = {"BodySerialNumber", "LensSerialNumber", "OwnerName"}
ROTATION = {1: "none", 3: "rotate 180°", 6: "rotate 90° clockwise", 8: "rotate 90° counter-clockwise"}

def displayed_size(tags):
    w, h = tags["Width"], tags["Height"]
    return (h, w) if tags.get("Orientation", 1) in (5, 6, 7, 8) else (w, h)

def clean(tags):
    w, h = displayed_size(tags)
    kept = {k: v for k, v in tags.items() if not k.startswith(PRIVATE_PREFIXES) and k not in PRIVATE_TAGS}
    kept.update({"Width": w, "Height": h, "Orientation": 1})
    removed = sorted(set(tags) - set(kept))
    return kept, removed

for name, tags in photos.items():
    w, h = displayed_size(tags)
    kept, removed = clean(tags)
    print(f"{name}: {ROTATION[tags.get('Orientation', 1)]}, shown as {w}x{h}")
    print(f"  removed: {', '.join(removed) or 'nothing'}")
    print(f"  kept: {', '.join(f'{k}={v}' for k, v in sorted(kept.items()))}")
