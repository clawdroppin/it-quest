@@lesson ru1-colour-maths Colour Maths in Rust: HSL, Mixing, Tints and Generated Palettes
@@concepts numbers, functions, objects
@@lang rust
@@text
A theme with 40 hand-picked hex codes is hard to maintain. Most Rust UI codebases generate the variants instead (hover and pressed shades, disabled tones, a 50–900 scale from one brand colour) with a few colour functions. This lesson implements those functions in plain Rust (no crates), so you know what `palette`, `egui::Color32::linear_multiply` or iced's colour helpers do inside.

## HSL

- **HSL** (hue 0–360°, saturation and lightness 0–1) is easy to reason about: change lightness for tints and shades, rotate the hue for an accent.
- RGB → HSL: with max and min the largest and smallest channels (0–1), L = (max + min) / 2, and S = d / (1 − |2L − 1|) where d = max − min (0 if d = 0). The hue depends on which channel is the max.
- HSL isn't perceptually uniform (yellow at L = 0.5 looks much lighter than blue at L = 0.5). For carefully balanced palettes use OKLCH (see the vector UI track), but HSL is fine for quick state colours.

## Mixing

- **Mix** two colours: a + (b − a) × t per channel. For more correct results, mix in **linear** light (convert from sRGB first, mix, convert back): mixing red and green in sRGB gives a muddy dark yellow, while linear mixing gives a brighter one.
- **Tint** = mix with white, **shade** = mix with black, **tone** = mix with grey.
- **Alpha compositing** (source over): out = src × α + dst × (1 − α), again ideally in linear light.

## State colours from one base

A common recipe (as used in the theme lesson):
- hover = 8% towards white (light theme) or 8% towards black (dark theme)
- pressed = 16%
- disabled = 40% opacity over the background
- focus ring = the base at full saturation, but check its contrast against the background

## Generated scales

From one brand colour, build 50–900 by setting HSL lightness to fixed targets (0.97, 0.93, 0.86, 0.76, 0.64, 0.52, 0.42, 0.33, 0.25, 0.18) and keeping the hue and saturation. Then check the contrast of text tokens against each step.

@@quiz
In HSL, which component do you change to make a tint or shade?
- Hue
- Saturation
* Lightness
- Alpha
? Hue rotation gives a different colour.

@@quiz
Why mix colours in linear light rather than sRGB?
* sRGB values are gamma-encoded, so averaging them gives results that are too dark and muddy
- Linear light is faster
- sRGB can't be mixed
- It changes the hue on purpose
? Convert, mix, convert back.

@@quiz
What is a tint?
* A colour mixed with white
- A colour mixed with black
- A colour mixed with grey
- A colour with lower alpha
? Shade = black, tone = grey.

@@quiz
Why isn't HSL ideal for balanced palettes?
* Equal L values don't look equally light across hues
- It can't represent red
- It only has 256 values
- It's slower than RGB
? Use OKLCH for perceptual balance.

@@quiz
What does source-over alpha compositing compute?
* src × α + dst × (1 − α)
- src + dst
- max(src, dst)
- src × dst
? Ideally in linear light.

@@quiz
In RGB → HSL, what is the lightness?
* (max + min) / 2 of the channels
- The average of all three channels
- The maximum channel
- 0.2126R + 0.7152G + 0.0722B
? That last one is relative luminance.

@@flashcards
HSL :: hue, saturation, lightness
lightness :: (max + min) / 2
saturation :: d / (1 − |2L − 1|)
hue :: depends on which channel is max
mix :: a + (b − a) × t
linear-light mixing :: convert from sRGB, mix, convert back
tint / shade / tone :: mix with white / black / grey
source over :: src α + dst (1 − α)
hover :: about 8% towards white or black
pressed :: about 16%
disabled :: about 40% opacity
generated scale :: fixed lightness targets, same hue

@@drill
What is the HSL lightness of pure red (255, 0, 0), as a decimal?
= 0.5
? (1 + 0) / 2.

@@drill
Mixing black and white at t = 0.5 in sRGB gives which channel value (0–255, rounded)?
= 128
= 127.5
? Linear mixing would give 188.

@@exercise RGB to HSL and back
Implement `to_hsl` and `from_hsl` (hue in degrees 0–360, saturation and lightness 0–1), and `lighten(c, amount)` that adds to the lightness (clamped to 0–1) and converts back.
@@starter
type Rgb = (u8, u8, u8);

fn to_hsl(c: Rgb) -> (f64, f64, f64) {
    todo!()
}

fn from_hsl(h: f64, s: f64, l: f64) -> Rgb {
    todo!()
}

fn lighten(c: Rgb, amount: f64) -> Rgb {
    todo!()
}

fn main() {
    let brand = (0x63, 0x66, 0xf1);
    let (h, s, l) = to_hsl(brand);
    println!("brand hsl({h:.0}, {:.0}%, {:.0}%)", s * 100.0, l * 100.0);
    println!("lighter {:?}, darker {:?}", lighten(brand, 0.15), lighten(brand, -0.15));
}
@@tests
#[test]
fn hsl_round_trip() {
    let close = |a: f64, b: f64| (a - b).abs() < 0.6;
    let (h, s, l) = to_hsl((255, 0, 0));
    assert!(close(h, 0.0) && (s - 1.0).abs() < 1e-9 && (l - 0.5).abs() < 1e-9, "red");
    let (h, _, _) = to_hsl((0, 255, 0));
    assert!(close(h, 120.0), "green hue 120");
    let (h, _, _) = to_hsl((0, 0, 255));
    assert!(close(h, 240.0), "blue hue 240");
    let (_, s, l) = to_hsl((128, 128, 128));
    assert_eq!(s, 0.0, "grey has no saturation");
    assert!((l - 0.502).abs() < 0.01);
    for c in [(0x63, 0x66, 0xf1), (12, 200, 99), (250, 128, 4), (0, 0, 0), (255, 255, 255)] {
        let (h, s, l) = to_hsl(c);
        assert_eq!(from_hsl(h, s, l), c, "round trip {c:?}");
    }
}

#[test]
fn lighten_clamps() {
    assert_eq!(lighten((0x63, 0x66, 0xf1), 1.0), (255, 255, 255));
    assert_eq!(lighten((0x63, 0x66, 0xf1), -1.0), (0, 0, 0));
    let (_, _, l0) = to_hsl((0x63, 0x66, 0xf1));
    let (_, _, l1) = to_hsl(lighten((0x63, 0x66, 0xf1), 0.1));
    assert!((l1 - l0 - 0.1).abs() < 0.01);
}
@@hint
`to_hsl`: scale to 0–1, find `max`, `min`, `d = max - min`, `l = (max + min) / 2`. If d is 0 return `(0.0, 0.0, l)`. Otherwise `s = d / (1 - (2l - 1).abs())`, and h is `60 × ((g - b) / d mod 6)` when red is max, `60 × ((b - r) / d + 2)` for green, and `60 × ((r - g) / d + 4)` for blue. `from_hsl`: `c = (1 - |2l - 1|) × s`, `x = c × (1 - |(h / 60) mod 2 - 1|)`, `m = l - c / 2`, pick (r, g, b) from the sector, add m, multiply by 255 and round.
@@solution
type Rgb = (u8, u8, u8);

fn to_hsl(c: Rgb) -> (f64, f64, f64) {
    let (r, g, b) = (c.0 as f64 / 255.0, c.1 as f64 / 255.0, c.2 as f64 / 255.0);
    let max = r.max(g).max(b);
    let min = r.min(g).min(b);
    let d = max - min;
    let l = (max + min) / 2.0;
    if d == 0.0 {
        return (0.0, 0.0, l);
    }
    let s = d / (1.0 - (2.0 * l - 1.0).abs());
    let h = if max == r {
        60.0 * ((g - b) / d).rem_euclid(6.0)
    } else if max == g {
        60.0 * ((b - r) / d + 2.0)
    } else {
        60.0 * ((r - g) / d + 4.0)
    };
    (h, s, l)
}

fn from_hsl(h: f64, s: f64, l: f64) -> Rgb {
    let c = (1.0 - (2.0 * l - 1.0).abs()) * s;
    let x = c * (1.0 - ((h / 60.0).rem_euclid(2.0) - 1.0).abs());
    let m = l - c / 2.0;
    let (r, g, b) = match (h.rem_euclid(360.0) / 60.0) as u32 {
        0 => (c, x, 0.0),
        1 => (x, c, 0.0),
        2 => (0.0, c, x),
        3 => (0.0, x, c),
        4 => (x, 0.0, c),
        _ => (c, 0.0, x),
    };
    let to = |v: f64| ((v + m) * 255.0).round().clamp(0.0, 255.0) as u8;
    (to(r), to(g), to(b))
}

fn lighten(c: Rgb, amount: f64) -> Rgb {
    let (h, s, l) = to_hsl(c);
    from_hsl(h, s, (l + amount).clamp(0.0, 1.0))
}

fn main() {
    let brand = (0x63, 0x66, 0xf1);
    let (h, s, l) = to_hsl(brand);
    println!("brand hsl({h:.0}, {:.0}%, {:.0}%)", s * 100.0, l * 100.0);
    println!("lighter {:?}, darker {:?}", lighten(brand, 0.15), lighten(brand, -0.15));
}

@@exercise Mix in linear light and composite
Implement `mix_srgb` (naive per-channel), `mix_linear` (convert to linear, mix, convert back) and `over(src, alpha, dst)` (source-over compositing in linear light). Then show the difference on red and green.
@@starter
type Rgb = (u8, u8, u8);

fn to_linear(v: u8) -> f64 {
    let v = v as f64 / 255.0;
    if v <= 0.04045 { v / 12.92 } else { ((v + 0.055) / 1.055).powf(2.4) }
}

fn to_srgb(v: f64) -> u8 {
    let v = if v <= 0.0031308 { v * 12.92 } else { 1.055 * v.powf(1.0 / 2.4) - 0.055 };
    (v * 255.0).round().clamp(0.0, 255.0) as u8
}

fn mix_srgb(a: Rgb, b: Rgb, t: f64) -> Rgb {
    todo!()
}

fn mix_linear(a: Rgb, b: Rgb, t: f64) -> Rgb {
    todo!()
}

fn over(src: Rgb, alpha: f64, dst: Rgb) -> Rgb {
    todo!()
}

fn main() {
    let (red, green) = ((255, 0, 0), (0, 255, 0));
    println!("sRGB mix   {:?}", mix_srgb(red, green, 0.5));
    println!("linear mix {:?}", mix_linear(red, green, 0.5));
    println!("40% black over white {:?}", over((0, 0, 0), 0.4, (255, 255, 255)));
}
@@tests
#[test]
fn mixing() {
    assert_eq!(mix_srgb((0, 0, 0), (255, 255, 255), 0.5), (128, 128, 128));
    assert_eq!(mix_linear((0, 0, 0), (255, 255, 255), 0.5), (188, 188, 188), "linear midpoint is lighter");
    assert_eq!(mix_srgb((255, 0, 0), (0, 255, 0), 0.5), (128, 128, 0));
    assert_eq!(mix_linear((255, 0, 0), (0, 255, 0), 0.5), (188, 188, 0));
    assert_eq!(mix_linear((10, 20, 30), (200, 100, 50), 0.0), (10, 20, 30), "t = 0 keeps a");
    assert_eq!(mix_linear((10, 20, 30), (200, 100, 50), 1.0), (200, 100, 50), "t = 1 gives b");
}

#[test]
fn compositing() {
    assert_eq!(over((0, 0, 0), 1.0, (255, 255, 255)), (0, 0, 0));
    assert_eq!(over((0, 0, 0), 0.0, (255, 255, 255)), (255, 255, 255));
    assert_eq!(over((0, 0, 0), 0.5, (255, 255, 255)), mix_linear((255, 255, 255), (0, 0, 0), 0.5));
}
@@hint
`mix_srgb`: `(a as f64 + (b as f64 - a as f64) * t).round() as u8` per channel. `mix_linear`: `to_srgb(la + (lb - la) * t)` with `la = to_linear(a.0)` and so on. `over(src, alpha, dst)` is `mix_linear(dst, src, alpha)`.
@@solution
type Rgb = (u8, u8, u8);

fn to_linear(v: u8) -> f64 {
    let v = v as f64 / 255.0;
    if v <= 0.04045 { v / 12.92 } else { ((v + 0.055) / 1.055).powf(2.4) }
}

fn to_srgb(v: f64) -> u8 {
    let v = if v <= 0.0031308 { v * 12.92 } else { 1.055 * v.powf(1.0 / 2.4) - 0.055 };
    (v * 255.0).round().clamp(0.0, 255.0) as u8
}

fn mix_srgb(a: Rgb, b: Rgb, t: f64) -> Rgb {
    let m = |x: u8, y: u8| (x as f64 + (y as f64 - x as f64) * t).round() as u8;
    (m(a.0, b.0), m(a.1, b.1), m(a.2, b.2))
}

fn mix_linear(a: Rgb, b: Rgb, t: f64) -> Rgb {
    let m = |x: u8, y: u8| {
        let (lx, ly) = (to_linear(x), to_linear(y));
        to_srgb(lx + (ly - lx) * t)
    };
    (m(a.0, b.0), m(a.1, b.1), m(a.2, b.2))
}

fn over(src: Rgb, alpha: f64, dst: Rgb) -> Rgb {
    mix_linear(dst, src, alpha)
}

fn main() {
    let (red, green) = ((255, 0, 0), (0, 255, 0));
    println!("sRGB mix   {:?}", mix_srgb(red, green, 0.5));
    println!("linear mix {:?}", mix_linear(red, green, 0.5));
    println!("40% black over white {:?}", over((0, 0, 0), 0.4, (255, 255, 255)));
}
