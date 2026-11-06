@@lesson gu5-icon-export App Icons: Designing, Generating and Exporting Every Size
@@concepts strings, files, functions
@@lang python
@@text
UI icons live inside your app. The **app icon** is your app's face: on the taskbar, the Start menu, the dock, the browser tab, the home screen. It has different rules, and it has to be exported at many sizes and in several file formats.

## Designing an app icon

- **One bold silhouette.** An app icon is often seen at 16 or 32 pixels. A single recognisable shape (a leaf, a bolt, a cat face) beats a detailed scene.
- **A background shape and colour.** Most platforms show icons on a rounded square or circle. A strong background colour makes it findable among dozens of others.
- **Squint test**: blur your eyes (or shrink it to 16 px). Can you still tell what it is?
- **Few colours**, high contrast, no thin lines, **no text** (a single letter at most).
- **Platform shapes**: macOS uses a rounded square ("squircle") with a little margin and a soft shadow; iOS and Android apply their own masks (Android "adaptive icons" have a foreground and a background layer and may be cut into circles or squircles); Windows icons can be any shape, and should look good on light and dark taskbars.
- Draw at a large size (e.g. 1024 × 1024) as SVG, then make simplified versions for 16 and 32 px if fine details disappear. Big icon sets do this: the 16 px version is redrawn, not just shrunk.

## The sizes and formats you need

| Where | Format | Sizes |
|---|---|---|
| Windows app (.exe, taskbar, shortcuts) | `.ico` (several PNGs in one file) | 16, 24, 32, 48, 64, 256 |
| macOS app | `.icns` | 16 to 1024 (with @2x versions) |
| Linux desktop | PNGs + SVG | 16 to 512, `hicolor` theme folders |
| Browser tab (favicon) | `favicon.ico` + `icon.svg` | 16, 32 (+ SVG for any size) |
| iPhone home screen | `apple-touch-icon.png` | 180 |
| Installed web app (PWA) | PNGs in `manifest.webmanifest` | 192, 512 (+ a "maskable" one) |
| Android | adaptive icon layers | 108 dp foreground/background |

```html
<!-- A modern favicon setup -->
<link rel="icon" href="/favicon.ico" sizes="32x32">
<link rel="icon" href="/icon.svg" type="image/svg+xml">
<link rel="apple-touch-icon" href="/apple-touch-icon.png">
<link rel="manifest" href="/manifest.webmanifest">
```

An SVG favicon can even support dark mode, because it can contain its own `<style>` with `@media (prefers-color-scheme: dark)`.

## Exporting: let tools do it

You almost never resize icons by hand:

```text
# Inkscape (free): SVG to PNG at any size
inkscape icon.svg --export-type=png --export-width=256 --export-filename=icon-256.png

# ImageMagick: many PNGs into one Windows .ico
magick icon-16.png icon-32.png icon-48.png icon-256.png app.ico

# Tauri: one 1024 px PNG (or SVG) to every size and format the app needs
npx tauri icon app-icon.png

# Electron: electron-builder reads build/icon.png (512+) and makes .ico and .icns
```

With Python and **Pillow** (`pip install pillow`), one line makes a multi-size `.ico`:

```python,norun
from PIL import Image

big = Image.open("icon-1024.png")
big.save("app.ico", sizes=[(16, 16), (24, 24), (32, 32), (48, 48), (64, 64), (256, 256)])

for size in (192, 512):
    big.resize((size, size), Image.LANCZOS).save(f"icon-{size}.png")
```

Pointing your app at its icon:

```python,norun
# tkinter (Windows): the window and taskbar icon
root.iconbitmap("app.ico")
# cross-platform: a PNG
root.iconphoto(True, tk.PhotoImage(file="icon-64.png"))
```

```xml
<!-- C#: in the .csproj, the icon of the .exe -->
<PropertyGroup>
  <ApplicationIcon>app.ico</ApplicationIcon>
</PropertyGroup>
<!-- WPF: the window icon -->
<Window Icon="app.ico" ...>
```

PyInstaller takes `--icon app.ico`; in Tauri the `bundle.icon` list in `tauri.conf.json` points at the generated files.

## Generating icons with code

Since SVG is just text, a program can **write** icons: avatar badges with someone's initials, coloured status dots, chart thumbnails, a different colour for each environment (dev green, staging amber, production red) so nobody deploys to the wrong one. The exercises do exactly that.

## Tools worth knowing

- **Figma** (free tier, in the browser): the industry standard for UI and icon design; exports SVG and PNG at any scale.
- **Inkscape**: free, open-source vector editor; great SVG output and a command line.
- **SVGOMG / svgo**: shrinks SVG files by removing editor junk (often 50 to 80% smaller).
- **Iconify** and **icones.js.org**: search 200,000+ open-source icons from every set.
- **Pixel art** icons (retro, very cute) are made in Aseprite, Piskel or LibreSprite, drawn at 16 or 32 px and scaled up by whole numbers with `image-rendering: pixelated`.

Always check the **licence** of icons you didn't draw: MIT, ISC and Apache (Lucide, Heroicons, Tabler, Material) are fine for commercial use; some sets need credit (CC BY), and some aren't free at all.

@@quiz
What's in a Windows `.ico` file?
- One 256 px image that Windows shrinks
* Several images of different sizes, so Windows picks the best one for each place
- Only vector shapes
- A link to a PNG online
? Include at least 16, 32, 48 and 256 so the taskbar, Explorer and shortcuts all look sharp.

@@quiz
What makes a good app icon at 16 × 16 pixels?
- Lots of fine detail and the app's full name
* One bold, simple silhouette with strong contrast
- A photograph
- Thin 1-pixel outlines in light grey
? Detail and text turn into mush at small sizes; test with the squint test.

@@quiz
What does `npx tauri icon app-icon.png` do?
- Uploads the icon to the store
* Generates every icon size and format (.ico, .icns, PNGs) a Tauri app needs from one big image
- Converts the app to SVG
- Sets the desktop wallpaper
? Electron's electron-builder does the same from build/icon.png.

@@quiz
Why might you generate SVG icons with code?
- SVG can only be written by programs
* SVG is text, so a program can make icons from data: initials, status colours, environment badges
- It's the only way to make PNGs
- It makes them load faster than inline SVG
? A few lines of string formatting can create hundreds of consistent icons.

@@quiz
Why do macOS and iOS icons need extra care compared with Windows icons?
* The platform applies its own rounded-square mask, so the artwork must fill the grid without its own rounded corners or padding baked in the wrong way
- They must be monochrome
- They can only be 16 × 16 pixels
- They must be saved as .ico files
? Follow each platform's template grid and let the system shape the icon.

@@flashcards
app icon :: the app's face on the taskbar, dock, Start menu and home screen
squint test :: blur or shrink the icon: is it still recognisable?
`.ico` :: Windows icon file containing several sizes (16 to 256)
`.icns` :: macOS icon file
favicon :: the browser tab icon: `favicon.ico` + `icon.svg`
`apple-touch-icon.png` :: 180 px icon for the iPhone home screen
PWA icons :: 192 and 512 px PNGs in the web manifest, plus a maskable one
adaptive icon (Android) :: separate foreground and background layers the OS masks
`inkscape --export-width` :: export an SVG to PNG at a size
`magick a.png b.png app.ico` :: combine PNGs into an .ico
`npx tauri icon` :: generate every size for a Tauri app
Pillow `img.save("app.ico", sizes=[...])` :: make an .ico from Python
`root.iconbitmap("app.ico")` :: set a tkinter window's icon on Windows
`<ApplicationIcon>` :: a C# project's .exe icon
svgo / SVGOMG :: shrink SVG files
icon licences :: MIT / ISC / Apache OK; CC BY needs credit

@@exercise Pick the right icon size
An `.ico` file contains a few sizes. Windows wants to show the icon at some size, and picks the **smallest size that is at least as big** as it needs (shrinking looks better than enlarging). If none is big enough, it uses the **largest** one. Write `pick(sizes, wanted)` for that (the sizes may be in any order), and `missing(sizes)`, which returns, sorted, which of the recommended Windows sizes `16, 24, 32, 48, 256` aren't in the file.
@@starter
RECOMMENDED = [16, 24, 32, 48, 256]


def pick(sizes, wanted):
    pass


def missing(sizes):
    pass


print(pick([256, 16, 48], 32))
print(missing([256, 16, 48]))
@@tests
check(pick([256, 16, 48], 32), 48, "smallest that is big enough")
check(pick([16, 32, 48], 32), 32, "an exact match")
check(pick([16, 32], 64), 32, "nothing big enough: the largest")
check(pick([256], 16), 256, "only one size")
check(missing([256, 16, 48]), [24, 32], "the gaps")
check(missing([48, 32, 256, 24, 16, 64]), [], "a complete icon")
check(missing([]), [16, 24, 32, 48, 256], "an empty file")
@@hint
`big_enough = [s for s in sizes if s >= wanted]`, then `return min(big_enough) if big_enough else max(sizes)`. For `missing`, `[s for s in RECOMMENDED if s not in sizes]`.
@@solution
RECOMMENDED = [16, 24, 32, 48, 256]


def pick(sizes, wanted):
    big_enough = [s for s in sizes if s >= wanted]
    return min(big_enough) if big_enough else max(sizes)


def missing(sizes):
    return [s for s in RECOMMENDED if s not in sizes]


print(pick([256, 16, 48], 32))
print(missing([256, 16, 48]))

@@exercise Generate avatar badge icons
Write `initials(name)`: the first letter of the first word and of the last word, in upper case (`"mika sato"` → `"MS"`); a single word gives one letter; extra spaces don't matter. Then `badge(name, color)` returns an SVG string, exactly:

```text
<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24"><circle cx="12" cy="12" r="12" fill="COLOR"/><text x="12" y="16" text-anchor="middle" font-size="10" fill="white">INITIALS</text></svg>
```

Finally `color_for(name, palette)` picks a colour so the same person always gets the same one: add up the `ord()` of every character in the name, and use that total modulo the length of the palette as the index.
@@starter
def initials(name):
    pass


def badge(name, color):
    pass


def color_for(name, palette):
    pass


palette = ["#f472b6", "#60a5fa", "#34d399", "#fbbf24"]
print(initials("mika sato"))
print(badge("Mika Sato", color_for("Mika Sato", palette)))
@@tests
check(initials("mika sato"), "MS", "two words")
check(initials("  Aiko  "), "A", "one word, with spaces")
check(initials("jan de  vries"), "JV", "first and last word")
check(badge("Ken", "#000"), '<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24"><circle cx="12" cy="12" r="12" fill="#000"/><text x="12" y="16" text-anchor="middle" font-size="10" fill="white">K</text></svg>', "the exact SVG")
p = ["#a", "#b", "#c"]
check(color_for("ab", p), "#a", "ord('a') + ord('b') = 195, 195 % 3 = 0")
check(color_for("abd", p), "#b", "97 + 98 + 100 = 295, 295 % 3 = 1")
check(color_for("ad", p), "#c", "97 + 100 = 197, 197 % 3 = 2")
check(color_for("Mika Sato", p), color_for("Mika Sato", p), "the same name always gets the same colour")
@@hint
`words = name.split()` (with no argument it ignores extra spaces). `words[0][0]` and `words[-1][0]`; if there's one word, only one letter. Build the SVG with an f-string. `palette[sum(ord(c) for c in name) % len(palette)]`.
@@solution
def initials(name):
    words = name.split()
    if len(words) == 1:
        return words[0][0].upper()
    return (words[0][0] + words[-1][0]).upper()


def badge(name, color):
    return (
        '<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24">'
        f'<circle cx="12" cy="12" r="12" fill="{color}"/>'
        f'<text x="12" y="16" text-anchor="middle" font-size="10" fill="white">{initials(name)}</text>'
        "</svg>"
    )


def color_for(name, palette):
    return palette[sum(ord(c) for c in name) % len(palette)]


palette = ["#f472b6", "#60a5fa", "#34d399", "#fbbf24"]
print(initials("mika sato"))
print(badge("Mika Sato", color_for("Mika Sato", palette)))
