@@lesson gm3-web-graphics Graphics for Screens: Indexed Colour, Dithering, Sprite Sheets and Responsive Exports
@@concepts lists, numbers, loops
@@lang python
@@text
Images made for screens (website headers, app screenshots, icons, game sprites, GIFs) have their own constraints: small files, sharp at every pixel density, sometimes a limited palette. GIMP handles all of this, but you need to know which options matter. This lesson covers indexed colour and dithering, sprite sheets, and how to plan a set of exports for responsive web pages.

## Indexed colour and palettes

- **Image → Mode → Indexed** reduces the image to a palette of at most 256 colours. GIF and PNG-8 require it, and many pixel-art workflows use it on purpose.
- Palette options:
  - **Generate optimum palette** (median cut on the image's own colours) with a maximum number of colours.
  - **Use web-optimised** or **black and white** palettes.
  - **Use a custom palette**, e.g. a game's fixed 16-colour palette (Windows → Dockable Dialogs → Palettes).
- Fewer colours means smaller files. For flat graphics (logos, diagrams, screenshots of UI), PNG-8 with 16–64 colours is often a fifth of the size of PNG-24 with no visible difference.

## Dithering

- With few colours, smooth gradients turn into visible **bands**. **Dithering** mixes palette colours in patterns so the eye averages them.
- **Floyd–Steinberg**: quantise each pixel to the nearest palette colour, then spread the **error** (wanted − got) to unprocessed neighbours: 7/16 to the right, 3/16 below-left, 5/16 below and 1/16 below-right. GIMP offers normal and "reduced colour bleeding" versions.
- **Positioned** (ordered, Bayer) dithering uses a fixed threshold pattern. It looks more regular and **compresses and animates better**, because the pattern doesn't shimmer between frames.
- **None**: best for flat graphics and pixel art, where dithering would only add noise.
- Dithering makes files **larger** (more detail for the compressor), so for photos on the web use JPEG/WebP/AVIF instead of indexed PNG.

## Sprite sheets and slicing

- Games and some web UIs pack many small images into one **sprite sheet** (an atlas): one file, one download, one texture.
- Frames in a grid: frame i is at column i mod cols, row i div cols, so x = col × (w + spacing) + margin.
- Leave **padding** (1–2 px, or extruded edge pixels) between frames, or texture filtering bleeds neighbouring frames into each other.
- GIMP: arrange frames as layers, then use a script, or Filters → Combine → Filmstrip, or the free "Sprite sheet" plug-ins. **Image → Guides → New Guides** plus the **Guillotine** command slice an image back into pieces.
- For animated GIF/WebP, layers are frames. Name layers like `frame 1 (100ms) (replace)` to set timing and disposal, and use Filters → Animation → Optimize (for GIF) to store only what changes.

## Responsive exports

- A browser picks from a **srcset** list (`hero-640.webp 640w, hero-1280.webp 1280w ...`) based on the layout width and the screen's pixel density.
- Plan sizes from the **display widths** your layout uses and the densities you support: a 600 px wide slot at 1× and 2× needs 600 and 1200 px images.
- **Never upscale**: if the source is 1600 px wide, don't export a 2400 px version, because it only adds bytes.
- Use **File → Export As** with quality around 75–85 for WebP/JPEG, strip metadata (unless you need the copyright), and keep the original XCF.
- Batch it: BIMP (a batch plug-in), a Script-Fu or Python-Fu script, or `gimp -i -b` from the command line (see the automation lesson).

@@quiz
Why does dithering help when reducing an image to 8 colours?
* It mixes palette colours in patterns so gradients don't show bands
- It removes colours from the palette
- It makes the file smaller
- It sharpens edges
? The eye averages the pattern.

@@quiz
In Floyd–Steinberg dithering, what fraction of the error goes to the pixel on the right?
* 7/16
- 1/2
- 3/16
- 1/16
? Then 3, 5 and 1 sixteenths to the next row.

@@quiz
Which dithering suits an animated GIF best?
* Positioned (ordered) dithering, because the pattern doesn't shimmer between frames
- Floyd–Steinberg
- Random noise
- None ever
? Error diffusion changes with every frame.

@@quiz
Why leave padding between frames in a sprite sheet?
* Texture filtering would bleed neighbouring frames into each other
- To make the file larger
- GIMP requires it
- To store metadata
? 1–2 px, or extrude edges.

@@quiz
A layout shows an image 600 px wide; you support 1× and 2× screens and the source is 1000 px wide. Which exports make sense?
* 600 and 1000 px (don't upscale to 1200)
- 600 and 1200 px
- Only 300 px
- 1000 and 2000 px
? Never upscale.

@@quiz
For a flat diagram with a dozen colours, which export is usually smallest?
* PNG-8 (indexed) without dithering
- JPEG quality 100
- PNG-24 with dithering
- BMP
? Few colours, no noise.

@@flashcards
Indexed mode :: palette of up to 256 colours
optimum palette :: median cut of the image's colours
banding :: visible steps in a gradient
dithering :: patterns of palette colours
Floyd–Steinberg :: error diffusion 7, 3, 5, 1 / 16
positioned dithering :: fixed Bayer pattern, good for animation
dither cost :: larger files
sprite sheet :: many frames in one image
frame position :: col = i mod cols, row = i div cols
padding :: avoid filtering bleed
Guillotine :: slice an image along guides
srcset :: list of image widths for the browser
never upscale :: cap exports at the source width
GIF frame name :: frame 1 (100ms) (replace)

@@drill
In a sprite sheet with 8 columns, which row (counting from 0) holds frame 19?
= 2
? 19 div 8 = 2.

@@drill
A pixel should be 100 and is quantised to 0. How much error goes to the right-hand pixel with Floyd–Steinberg? (Round to one decimal.)
= 43.8
= 43.75
? 100 × 7/16.

@@exercise Floyd–Steinberg versus threshold
Reduce an 8 × 32 grey gradient to black and white (0 or 255) by plain thresholding at 128 and by Floyd–Steinberg dithering (the error kept as floats, pixels outside the image ignored). Print both as text (`#` for white, `.` for black) and the average brightness of each column group compared to the original.
@@starter
W, H = 32, 8
gradient = [[round(x * 255 / (W - 1)) for x in range(W)] for _ in range(H)]

def threshold(img):
    raise NotImplementedError

def floyd_steinberg(img):
    raise NotImplementedError

def show(name, img):
    print(name)
    for row in img[:4]:
        print("  " + "".join("#" if v >= 128 else "." for v in row))
    groups = [sum(img[y][x] for y in range(H) for x in range(g, g + 8)) / (8 * H) for g in range(0, W, 8)]
    print("  group means " + " ".join(f"{m:5.1f}" for m in groups))

show("original", gradient)
show("threshold", threshold(gradient))
show("floyd-steinberg", floyd_steinberg(gradient))
@@expected
original
  ................################
  ................################
  ................################
  ................################
  group means  28.8  94.5 160.5 226.2
threshold
  ................################
  ................################
  ................################
  ................################
  group means   0.0   0.0 255.0 255.0
floyd-steinberg
  ..........#..#.#.#.##.##########
  ......#.#..#.#.#.##.###.##.#####
  .........#..#..#.#.#.#.#########
  ....#..#..#..#.#.#.#####.###.###
  group means  27.9  95.6 159.4 231.1
@@requires
def threshold
def floyd_steinberg
7 / 16
@@hint
Copy the image into floats. Scan rows top to bottom, left to right: `new = 255 if old >= 128 else 0`, `err = old - new`, then add `err * 7 / 16` to (x+1, y), `err * 3 / 16` to (x−1, y+1), `err * 5 / 16` to (x, y+1) and `err * 1 / 16` to (x+1, y+1), skipping positions outside the image. The original's text rows look like the threshold, but its group means are the true brightness to compare against.
@@solution
W, H = 32, 8
gradient = [[round(x * 255 / (W - 1)) for x in range(W)] for _ in range(H)]

def threshold(img):
    return [[255 if v >= 128 else 0 for v in row] for row in img]

def floyd_steinberg(img):
    buf = [[float(v) for v in row] for row in img]
    out = [[0] * W for _ in range(H)]
    for y in range(H):
        for x in range(W):
            old = buf[y][x]
            new = 255 if old >= 128 else 0
            out[y][x] = new
            err = old - new
            for dx, dy, f in ((1, 0, 7 / 16), (-1, 1, 3 / 16), (0, 1, 5 / 16), (1, 1, 1 / 16)):
                nx, ny = x + dx, y + dy
                if 0 <= nx < W and ny < H:
                    buf[ny][nx] += err * f
    return out

def show(name, img):
    print(name)
    for row in img[:4]:
        print("  " + "".join("#" if v >= 128 else "." for v in row))
    groups = [sum(img[y][x] for y in range(H) for x in range(g, g + 8)) / (8 * H) for g in range(0, W, 8)]
    print("  group means " + " ".join(f"{m:5.1f}" for m in groups))

show("original", gradient)
show("threshold", threshold(gradient))
show("floyd-steinberg", floyd_steinberg(gradient))

@@exercise Plan responsive exports and a sprite sheet
Part 1: given a source width, the layout's display widths and the densities to support, list the export widths (sorted, unique, never above the source width) and the `srcset` string. Part 2: given a frame size, a frame count, a column count, padding between frames and a margin, return the sheet size and each frame's position.
@@starter
def export_widths(source_w, display_widths, densities):
    raise NotImplementedError

def srcset(name, widths):
    return ", ".join(f"{name}-{w}.webp {w}w" for w in widths)

def sprite_sheet(fw, fh, count, cols, pad=2, margin=1):
    raise NotImplementedError

ws = export_widths(1600, [360, 600, 1200], [1, 2])
print("widths", ws)
print("srcset", srcset("hero", ws))
size, frames = sprite_sheet(32, 48, 10, 4)
print("sheet", size)
for i, pos in enumerate(frames):
    print(f"  frame {i}: {pos}")
@@expected
widths [360, 600, 720, 1200, 1600]
srcset hero-360.webp 360w, hero-600.webp 600w, hero-720.webp 720w, hero-1200.webp 1200w, hero-1600.webp 1600w
sheet (136, 150)
  frame 0: (1, 1)
  frame 1: (35, 1)
  frame 2: (69, 1)
  frame 3: (103, 1)
  frame 4: (1, 51)
  frame 5: (35, 51)
  frame 6: (69, 51)
  frame 7: (103, 51)
  frame 8: (1, 101)
  frame 9: (35, 101)
@@requires
def export_widths
def sprite_sheet
min(
@@hint
`export_widths`: `sorted({min(source_w, w * d) for w in display_widths for d in densities})`. Sprite sheet: `rows = ceil(count / cols)`, width `2 * margin + cols * fw + (cols - 1) * pad`, the height likewise, and frame i is at `(margin + col * (fw + pad), margin + row * (fh + pad))`.
@@solution
import math

def export_widths(source_w, display_widths, densities):
    return sorted({min(source_w, w * d) for w in display_widths for d in densities})

def srcset(name, widths):
    return ", ".join(f"{name}-{w}.webp {w}w" for w in widths)

def sprite_sheet(fw, fh, count, cols, pad=2, margin=1):
    rows = math.ceil(count / cols)
    size = (2 * margin + cols * fw + (cols - 1) * pad, 2 * margin + rows * fh + (rows - 1) * pad)
    frames = [(margin + (i % cols) * (fw + pad), margin + (i // cols) * (fh + pad)) for i in range(count)]
    return size, frames

ws = export_widths(1600, [360, 600, 1200], [1, 2])
print("widths", ws)
print("srcset", srcset("hero", ws))
size, frames = sprite_sheet(32, 48, 10, 4)
print("sheet", size)
for i, pos in enumerate(frames):
    print(f"  frame {i}: {pos}")
