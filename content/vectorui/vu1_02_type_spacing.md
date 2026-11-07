@@lesson vu1-type-spacing Typography and Spacing Systems: Type Scales, Line Height and the 4/8-Point Grid
@@concepts numbers, lists, functions
@@lang python
@@text
Good interfaces look calm because their sizes aren't arbitrary. Font sizes come from a **type scale**, spacing from a **spacing scale**, and both sit on a shared **grid**. In Penpot you store these as typography styles and spacing tokens. In code they become CSS variables or Rust constants. This lesson builds both scales and the checks that keep a design consistent.

## Type scales

- A **modular scale** multiplies a base size by a ratio: base × ratio^n. Common ratios are 1.125 (major second, dense apps), 1.2 (minor third), 1.25 (major third) and 1.333 (perfect fourth, marketing pages).
- Base 16 px with a 1.25 ratio gives 12.8, **16**, 20, 25, 31.25, 39.06... Round to whole pixels, and express sizes in **rem** (÷ 16) so users' browser font settings scale them.
- Keep to about **5–7 sizes**. More than that and nobody can tell them apart.
- **Line height**: about 1.4–1.6 for body text, and 1.1–1.25 for large headings (big text with loose line height looks disconnected). Snap the line height in pixels to the **4 px grid** so text blocks align with everything else.
- **Line length**: 45–75 characters for comfortable reading (`max-width: 65ch`).
- **Font weights**: two or three (e.g. 400, 600, 700). Variable fonts make in-between weights cheap, but restraint still helps.

## Spacing scales and the grid

- **4-point grid**: every size, padding, gap and line height is a multiple of 4 (8-point systems use 8, with 4 for fine adjustments).
- A spacing scale such as 4, 8, 12, 16, 24, 32, 48, 64 grows faster as it gets bigger, because small differences matter more at small sizes.
- **Inner ≤ outer**: the padding inside a group should be smaller than the space between groups (the **proximity** principle from Gestalt), or items won't look grouped.
- Penpot's flex layout gap and padding map directly to CSS `gap` and `padding`. Store them as spacing tokens (`space.2 = 8px`).

## Putting the systems together

Type and spacing scales work best when they're designed together, because text sizes and gaps sit side by side in every component:

- **Tie line height to the spacing grid.** On a 4 px grid, choose line heights that are multiples of 4 (a 16 px body size with a 24 px line height, a 20 px subheading with 28 px). Then text blocks stack neatly with your spacing tokens and baselines line up across columns.
- **Space by relationship.** Use smaller gaps between things that belong together (a label and its field: 4–8 px) and larger gaps between groups (between form sections: 24–32 px). This is the Gestalt principle of **proximity**, and it does more to organise a screen than lines and boxes.
- **Limit the number of sizes in use.** A screen with eight different font sizes looks chaotic even if each is on the scale. Most interfaces need about five: caption, body, subheading, heading and a display size for rare large titles.
- **Mind the measure.** Body text is most comfortable at about 45–75 characters per line. In CSS, `max-width: 65ch` on text containers keeps paragraphs readable on wide screens.
- **Use weight and colour before size.** To create hierarchy within a component, a semibold weight or a muted secondary colour is often better than another size step.

## Checking your work

Turn on the layout grid in Penpot and look at a screen at 100% zoom. Ask: do the gaps fall on the grid? Do related items sit closer together than unrelated ones? Can you name every font size you used and say why it's that size? If a value doesn't fit, it's usually a sign that a token is missing, or that a one-off value crept in and should be replaced with the nearest step.

@@quiz
What does a modular type scale with ratio 1.25 and base 16 px give for the next size up?
- 18 px
* 20 px
- 24 px
- 32 px
? 16 × 1.25.

@@quiz
Why express font sizes in rem rather than px?
* They scale with the user's browser font size setting
- rem is faster to render
- px doesn't work in CSS
- rem makes text bolder
? Accessibility: respect user preferences.

@@quiz
Which line height suits large headings?
- 2.0
- 1.6
* 1.1 to 1.25
- 0.8
? Body text needs more; headings less.

@@quiz
Cards have 24 px padding and an 8 px gap between them. What goes wrong?
* The cards don't look like separate groups, because the outer space is smaller than the inner space
- Nothing
- Text becomes unreadable
- The grid breaks the CSS
? Inner spacing should be smaller than outer spacing.

@@quiz
What is a comfortable line length for body text?
- 20–30 characters
* 45–75 characters
- 100–120 characters
- As wide as the screen
? max-width: 65ch is a good default.

@@flashcards
modular scale :: base × ratio^n
1.2 / 1.25 / 1.333 :: minor third / major third / perfect fourth
rem :: relative to the root font size (16 px by default)
body line height :: about 1.4–1.6
heading line height :: about 1.1–1.25
line length :: 45–75 characters, max-width 65ch
4-point grid :: sizes and spacing in multiples of 4
spacing scale :: 4, 8, 12, 16, 24, 32, 48, 64
proximity :: closer things look related
inner ≤ outer :: padding smaller than the gap between groups
spacing token :: space.2 = 8px

@@drill
What is 24 px in rem with the default root size?
= 1.5
= 1.5rem
? 24 ÷ 16.

@@drill
Snap a line height of 27 px to the nearest multiple of 4.
= 28
? 27 is closer to 28 than to 24.

@@exercise Generate a type scale
Generate a modular type scale from a base size and ratio (two steps down, five up), round to whole pixels, convert to rem, and pick a line height for each size (1.5 for small text and down to 1.2 for big headings) snapped to the 4 px grid. Print the CSS custom properties.
@@starter
BASE, RATIO = 16, 1.25
NAMES = ["xs", "sm", "base", "lg", "xl", "2xl", "3xl", "4xl"]

def scale():
    raise NotImplementedError
@@expected
:root {
  --text-xs: 0.625rem; --leading-xs: 1rem;  /* 10px / 16px */
  --text-sm: 0.8125rem; --leading-sm: 1.25rem;  /* 13px / 20px */
  --text-base: 1rem; --leading-base: 1.5rem;  /* 16px / 24px */
  --text-lg: 1.25rem; --leading-lg: 1.75rem;  /* 20px / 28px */
  --text-xl: 1.5625rem; --leading-xl: 2rem;  /* 25px / 32px */
  --text-2xl: 1.9375rem; --leading-2xl: 2.25rem;  /* 31px / 36px */
  --text-3xl: 2.4375rem; --leading-3xl: 3rem;  /* 39px / 48px */
  --text-4xl: 3.0625rem; --leading-4xl: 3.75rem;  /* 49px / 60px */
}
@@requires
def scale
RATIO **
/ 16
% 4
@@hint
Size n (from −2 to 5) is `BASE * RATIO ** n`. Round it. Line-height factor: 1.5 at or below 16 px, 1.35 up to 24 px, 1.2 above. Snap with `round(px / 4) * 4`. Check with `px % 4 == 0`. Print rem with `:g` to drop trailing zeros.
@@solution
BASE, RATIO = 16, 1.25
NAMES = ["xs", "sm", "base", "lg", "xl", "2xl", "3xl", "4xl"]

def scale():
    rows = []
    for name, n in zip(NAMES, range(-2, 6)):
        size = round(BASE * RATIO ** n)
        factor = 1.5 if size <= 16 else 1.35 if size <= 24 else 1.2
        line = round(size * factor / 4) * 4
        assert line % 4 == 0
        rows.append((name, size, line))
    return rows

print(":root {")
for name, size, line in scale():
    print(f"  --text-{name}: {size / 16:g}rem; --leading-{name}: {line / 16:g}rem;  /* {size}px / {line}px */")
print("}")

@@exercise Audit spacing against the grid
Check the spacing values used in a design (padding, gaps, sizes) against a 4 px grid and the spacing scale: snap each value to the nearest scale step, report values that aren't on the scale, and check that each group's inner padding is smaller than the gap between groups.
@@starter
SCALE = [0, 4, 8, 12, 16, 24, 32, 48, 64]
components = {
    "card": {"padding": 20, "gap_inside": 10, "gap_between": 24, "radius": 12},
    "toolbar": {"padding": 8, "gap_inside": 6, "gap_between": 16, "radius": 8},
    "settings list": {"padding": 16, "gap_inside": 16, "gap_between": 12, "radius": 0},
    "dialog": {"padding": 24, "gap_inside": 12, "gap_between": 32, "radius": 15},
}

def snap(value):
    raise NotImplementedError
@@expected
card           padding 20 -> 16; gap_inside 10 -> 8
toolbar        gap_inside 6 -> 4
settings list  inner spacing >= outer gap: groups won't read as separate
dialog         radius 15 -> 16
@@requires
def snap
SCALE
gap_between
abs(
@@hint
`snap` returns the scale step with the smallest `abs(step - value)` (ties go to the smaller step if you use `min` with that key over the sorted list). Report each value whose snapped value differs. Proximity: inner padding and the inner gap must both be smaller than `gap_between`.
@@solution
SCALE = [0, 4, 8, 12, 16, 24, 32, 48, 64]
components = {
    "card": {"padding": 20, "gap_inside": 10, "gap_between": 24, "radius": 12},
    "toolbar": {"padding": 8, "gap_inside": 6, "gap_between": 16, "radius": 8},
    "settings list": {"padding": 16, "gap_inside": 16, "gap_between": 12, "radius": 0},
    "dialog": {"padding": 24, "gap_inside": 12, "gap_between": 32, "radius": 15},
}

def snap(value):
    return min(SCALE, key=lambda step: abs(step - value))

for name, props in components.items():
    fixes = [f"{k} {v} -> {snap(v)}" for k, v in props.items() if snap(v) != v]
    problems = []
    if not (props["gap_inside"] < props["gap_between"] and props["padding"] <= props["gap_between"]):
        problems.append("inner spacing >= outer gap: groups won't read as separate")
    status = "; ".join(fixes + problems) or "on the grid"
    print(f"{name:<14} {status}")
