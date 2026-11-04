@@lesson vu4-colour-systems Colour Systems: Tonal Scales, Contrast and Dark Mode
@@concepts numbers, maps
@@lang python
@@text
A design system's palette isn't a handful of pretty colours: it's **tonal scales** (50 to 950) for each hue, chosen so that text and UI meet contrast requirements, plus **semantic** roles that map to different tones in light and dark themes. This lesson explains how scales are built, how contrast is measured, and how to design dark mode deliberately rather than by inverting.

## Tonal scales

- Each hue gets steps such as 50, 100, 200 … 900, 950 (Tailwind, Radix, Material's tones 0–100). Lightness changes evenly **in a perceptual space** (OKLCH), so steps look evenly spaced; equal steps in HSL don't.
- Chroma (colourfulness) usually peaks in the middle of the scale and falls off towards very light and very dark tones, which can't hold much colour.
- **Neutrals** (greys) often carry a hint of the brand hue so they don't look dead.
- Tools: Penpot/Figma plugins, Radix Colors, Material Theme Builder, OKLCH pickers (oklch.com), and CSS `oklch()` directly.

## Contrast

WCAG 2 contrast ratio = (L1 + 0.05) / (L2 + 0.05), where L is relative luminance (linear-light weighted RGB). Requirements:

| Use | AA | AAA |
|---|---|---|
| Body text | **4.5:1** | 7:1 |
| Large text (24 px, or 18.7 px bold) | **3:1** | 4.5:1 |
| UI components and meaningful graphics | **3:1** | — |

WCAG 2's formula is imperfect for dark backgrounds and thin fonts. **APCA** (proposed for WCAG 3) models perception better, but WCAG 2 AA is still the legal and procurement standard, so check both if you can.

## Semantic tokens and dark mode

- Components use semantic tokens (`bg`, `surface`, `text`, `text-muted`, `border`, `primary`, `on-primary`, `danger`…) that point to scale steps: `primary = blue-600` in light, `blue-400` in dark.
- Dark mode is **not inversion**: use dark grey (around #0f172a–#1e1e1e), not pure black, to reduce halation; **lighten** surfaces to show elevation instead of relying on shadows; **desaturate** bright accents slightly; re-check every text pair's contrast.
- CSS: `color-scheme: light dark;` plus `light-dark(#fff, #0f172a)` or `@media (prefers-color-scheme: dark)` with custom properties; let the user override with a `data-theme` attribute.

@@quiz
Why build tonal scales in OKLCH rather than HSL?
* OKLCH lightness steps look evenly spaced to people; HSL's don't
- HSL can't represent blue
- OKLCH files are smaller
- Browsers don't support HSL
? Perceptual uniformity.

@@quiz
What contrast does body text need for WCAG AA?
- 3:1
* 4.5:1
- 7:1
- 21:1
? Large text needs 3:1.

@@quiz
How should elevation usually be shown in dark mode?
- Darker surfaces for higher elements
* Lighter surfaces for higher elements
- Pure black for everything
- Thicker shadows only
? Shadows are hard to see on dark backgrounds.

@@quiz
What is the role of a semantic token like `text-muted`?
- It's a raw colour value
* It names a purpose and maps to different scale steps per theme
- It's only for documentation
- It disables text
? Components never reference blue-600 directly.

@@quiz
What is the maximum possible WCAG contrast ratio?
- 7:1
- 10:1
* 21:1
- 100:1
? Black on white.

@@flashcards
tonal scale :: 50 to 950 per hue
OKLCH :: lightness, chroma, hue
chroma peak :: middle of the scale
tinted neutrals :: greys with a hint of hue
contrast ratio :: (L1 + 0.05) / (L2 + 0.05)
AA body text :: 4.5:1
AA large text :: 3:1
UI components :: 3:1
APCA :: perceptual contrast for WCAG 3
semantic token :: purpose-named colour
dark mode surfaces :: dark grey, not black
elevation in dark mode :: lighter surfaces
light-dark() :: CSS per-scheme value
color-scheme :: tell the browser both themes

@@drill
What contrast ratio does large text need for WCAG AA?
= 3:1
= 3
? 4.5:1 for body text.

@@drill
In dark mode, should higher (elevated) surfaces be lighter or darker?
= lighter
? They catch more light.

@@exercise Build a tonal scale in OKLCH
Generate a 10-step scale (50 to 900) for a hue from OKLCH lightness targets, with chroma peaking in the middle (`max_chroma × sin(π × position)` with a floor), convert each step to sRGB hex (reduce chroma until it fits in gamut), and print each step's contrast against white and black, plus which steps can carry body text on white.
@@starter
import math

STEPS = [50, 100, 200, 300, 400, 500, 600, 700, 800, 900]
LIGHTNESS = [0.97, 0.93, 0.86, 0.78, 0.69, 0.60, 0.52, 0.44, 0.36, 0.28]
M2_INV = [[1, 0.3963377774, 0.2158037573], [1, -0.1055613458, -0.0638541728], [1, -0.0894841775, -1.2914855480]]
M1_INV = [[4.0767416621, -3.3077115913, 0.2309699292], [-1.2684380046, 2.6097574011, -0.3413193965], [-0.0041960863, -0.7034186147, 1.7076147010]]

def oklch_to_linear(l, c, h):
    raise NotImplementedError
@@expected
blue
   50 #f0f5ff C=0.014  vs white  1.09  vs black 19.26
  100 #dce9fe C=0.032  vs white  1.23  vs black 17.10
  200 #b7d2ff C=0.069  vs white  1.53  vs black 13.72
  300 #8fb8fc C=0.106  vs white  2.01  vs black 10.46
  400 #5d99fd C=0.160  vs white  2.81  vs black  7.48
  500 #3a7ce8 C=0.178  vs white  4.03  vs black  5.21
  600 #2b64c3 C=0.160  vs white  5.63  vs black  3.73
  700 #245097 C=0.127  vs white  7.89  vs black  2.66
  800 #223c68 C=0.082  vs white 10.93  vs black  1.92
  900 #212937 C=0.028  vs white 14.60  vs black  1.44
  body text on white: 600, 700, 800, 900
green
   50 #eafaec C=0.025  vs white  1.08  vs black 19.42
  100 #c6f6cf C=0.073  vs white  1.20  vs black 17.52
  200 #9ae6aa C=0.113  vs white  1.47  vs black 14.29
  300 #6dd186 C=0.143  vs white  1.90  vs black 11.08
  400 #41b664 C=0.158  vs white  2.60  vs black  8.08
  500 #189949 C=0.158  vs white  3.68  vs black  5.70
  600 #027e38 C=0.143  vs white  5.16  vs black  4.07
  700 #10632e C=0.113  vs white  7.37  vs black  2.85
  800 #1c4828 C=0.073  vs white 10.52  vs black  2.00
  900 #202c22 C=0.025  vs white 14.46  vs black  1.45
  body text on white: 600, 700, 800, 900
@@requires
def oklch_to_linear
def contrast
math.cos(
@@hint
OKLCH → OKLab: `a = c·cos(h°)`, `b = c·sin(h°)`; then `lms = M2_INV · (l, a, b)`, cube each, `rgb = M1_INV · lms`. In gamut means every channel is between 0 and 1: otherwise multiply chroma by 0.95 and retry. Luminance uses the linear values directly: `0.2126 r + 0.7152 g + 0.0722 b`.
@@solution
import math

STEPS = [50, 100, 200, 300, 400, 500, 600, 700, 800, 900]
LIGHTNESS = [0.97, 0.93, 0.86, 0.78, 0.69, 0.60, 0.52, 0.44, 0.36, 0.28]
M2_INV = [[1, 0.3963377774, 0.2158037573], [1, -0.1055613458, -0.0638541728], [1, -0.0894841775, -1.2914855480]]
M1_INV = [[4.0767416621, -3.3077115913, 0.2309699292], [-1.2684380046, 2.6097574011, -0.3413193965], [-0.0041960863, -0.7034186147, 1.7076147010]]

def mat(m, v):
    return [sum(a * b for a, b in zip(row, v)) for row in m]

def oklch_to_linear(l, c, h):
    a, b = c * math.cos(math.radians(h)), c * math.sin(math.radians(h))
    return mat(M1_INV, [v ** 3 for v in mat(M2_INV, [l, a, b])])

def encode(c):
    return 12.92 * c if c <= 0.0031308 else 1.055 * c ** (1 / 2.4) - 0.055

def luminance(lin):
    return 0.2126 * lin[0] + 0.7152 * lin[1] + 0.0722 * lin[2]

def contrast(l1, l2):
    hi, lo = max(l1, l2), min(l1, l2)
    return (hi + 0.05) / (lo + 0.05)

def scale(hue, max_chroma):
    rows = []
    for i, (step, l) in enumerate(zip(STEPS, LIGHTNESS)):
        c = max(0.02, max_chroma * math.sin(math.pi * (i + 0.5) / len(STEPS)))
        lin = oklch_to_linear(l, c, hue)
        while any(v < 0 or v > 1 for v in lin):
            c *= 0.95
            lin = oklch_to_linear(l, c, hue)
        hexcode = "#" + "".join(f"{round(encode(v) * 255):02x}" for v in lin)
        y = luminance(lin)
        rows.append((step, hexcode, c, contrast(y, 1.0), contrast(y, 0.0)))
    return rows

for name, hue, chroma in (("blue", 260, 0.18), ("green", 150, 0.16)):
    print(name)
    rows = scale(hue, chroma)
    for step, hexcode, c, on_white, on_black in rows:
        print(f"  {step:>3} {hexcode} C={c:.3f}  vs white {on_white:5.2f}  vs black {on_black:5.2f}")
    print("  body text on white:", ", ".join(str(r[0]) for r in rows if r[3] >= 4.5))

@@exercise Check a light and dark theme
Map semantic tokens to hex colours for a light and a dark theme, then check each text/background pair against its requirement (4.5 for text, 3 for large text and UI). Print failures with the ratio and suggest the nearest step from the scale that passes.
@@starter
scale = {"slate-50": "#f8fafc", "slate-200": "#e2e8f0", "slate-400": "#94a3b8", "slate-500": "#64748b",
         "slate-600": "#475569", "slate-700": "#334155", "slate-800": "#1e293b", "slate-900": "#0f172a",
         "blue-300": "#93c5fd", "blue-400": "#60a5fa", "blue-500": "#3b82f6", "blue-600": "#2563eb", "blue-700": "#1d4ed8"}
themes = {
    "light": {"bg": "slate-50", "surface": "slate-50", "text": "slate-900", "text-muted": "slate-400", "primary": "blue-500", "border": "slate-200"},
    "dark": {"bg": "slate-900", "surface": "slate-800", "text": "slate-50", "text-muted": "slate-400", "primary": "blue-400", "border": "slate-700"},
}
checks = [("text", "bg", 4.5), ("text-muted", "surface", 4.5), ("primary", "bg", 3.0), ("border", "surface", 3.0)]

def contrast(hex_a, hex_b):
    raise NotImplementedError
@@expected
light
  ok   text on bg: 17.06
  FAIL text-muted (slate-400) on surface: 2.45 < 4.5; try slate-500
  ok   primary on bg: 3.52
  FAIL border (slate-200) on surface: 1.18 < 3.0; try slate-500
dark
  ok   text on bg: 17.06
  ok   text-muted on surface: 5.71
  ok   primary on bg: 7.02
  FAIL border (slate-700) on surface: 1.41 < 3.0; try slate-500
@@requires
def contrast
0.05
12.92
@@hint
Relative luminance: decode each channel (`c / 12.92` or `((c + 0.055) / 1.055) ** 2.4`) and weight 0.2126/0.7152/0.0722. For a failure, try the other steps of the same hue (same prefix before `-`) and pick the passing one whose number is closest to the current step.
@@solution
scale = {"slate-50": "#f8fafc", "slate-200": "#e2e8f0", "slate-400": "#94a3b8", "slate-500": "#64748b",
         "slate-600": "#475569", "slate-700": "#334155", "slate-800": "#1e293b", "slate-900": "#0f172a",
         "blue-300": "#93c5fd", "blue-400": "#60a5fa", "blue-500": "#3b82f6", "blue-600": "#2563eb", "blue-700": "#1d4ed8"}
themes = {
    "light": {"bg": "slate-50", "surface": "slate-50", "text": "slate-900", "text-muted": "slate-400", "primary": "blue-500", "border": "slate-200"},
    "dark": {"bg": "slate-900", "surface": "slate-800", "text": "slate-50", "text-muted": "slate-400", "primary": "blue-400", "border": "slate-700"},
}
checks = [("text", "bg", 4.5), ("text-muted", "surface", 4.5), ("primary", "bg", 3.0), ("border", "surface", 3.0)]

def luminance(hexcode):
    def lin(c):
        return c / 12.92 if c <= 0.04045 else ((c + 0.055) / 1.055) ** 2.4
    r, g, b = (lin(int(hexcode[i:i + 2], 16) / 255) for i in (1, 3, 5))
    return 0.2126 * r + 0.7152 * g + 0.0722 * b

def contrast(hex_a, hex_b):
    la, lb = sorted((luminance(hex_a), luminance(hex_b)), reverse=True)
    return (la + 0.05) / (lb + 0.05)

def suggest(token, against, need):
    hue, step = token.split("-")
    options = [name for name in scale if name.startswith(hue + "-") and contrast(scale[name], scale[against]) >= need]
    return min(options, key=lambda n: abs(int(n.split("-")[1]) - int(step)), default=None)

for theme, tokens in themes.items():
    print(theme)
    for fg, bg, need in checks:
        ratio = contrast(scale[tokens[fg]], scale[tokens[bg]])
        if ratio >= need:
            print(f"  ok   {fg} on {bg}: {ratio:.2f}")
        else:
            fix = suggest(tokens[fg], tokens[bg], need)
            print(f"  FAIL {fg} ({tokens[fg]}) on {bg}: {ratio:.2f} < {need}; try {fix}")
