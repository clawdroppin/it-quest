@@lesson b10-graphics 🎓 Beyond IT: Generative Graphics with System.Drawing and SVG
@@concepts files, numbers, loops
@@text
> **Beyond IT scope.** Generative art and image processing belong to creative coding, not to IT administration. They're a fun way to practise loops, maths and file formats with tools you already have.

PowerShell can create images two ways without installing anything:

- **Bitmaps** with `System.Drawing` (GDI+, Windows): pixels, shapes, text, saved as PNG or JPEG.
- **SVG** text files: vector graphics described in XML, which any browser can open and which scale to any size.

## System.Drawing in 30 seconds

```powershell,norun
Add-Type -AssemblyName System.Drawing
$bmp = [System.Drawing.Bitmap]::new(400, 300)
$g = [System.Drawing.Graphics]::FromImage($bmp)
$g.SmoothingMode = 'AntiAlias'
$g.Clear([System.Drawing.Color]::White)
$g.FillEllipse([System.Drawing.Brushes]::SteelBlue, 50, 50, 120, 120)
$g.DrawString('Hello', [System.Drawing.Font]::new('Segoe UI', 24), [System.Drawing.Brushes]::Black, 200, 120)
$bmp.SetPixel(10, 10, [System.Drawing.Color]::Red)
$bmp.Save("$env:TEMP\art.png", [System.Drawing.Imaging.ImageFormat]::Png)
$g.Dispose(); $bmp.Dispose()
```

Always `Dispose()` GDI+ objects: they hold unmanaged memory and file locks. `SetPixel` is slow (fine for small images); for big ones, `LockBits` gives raw memory access.

## Colour from maths

Generative pictures map coordinates to colours with a formula. Some classics:

- **Plasma**: `sin(x/16) + sin(y/8) + sin((x+y)/16)`, mapped to a colour palette.
- **Mandelbrot**: iteration count of `z = z² + c` (you met it in the Bash track).
- **XOR pattern**: `(x -bxor y) % 256` as a grey value.
- **HSV colour wheels**: hue from the angle around a centre, saturation from the distance.

## SVG: graphics as text

```xml
<svg xmlns="http://www.w3.org/2000/svg" width="200" height="200" viewBox="0 0 200 200">
  <rect width="200" height="200" fill="#fafafa"/>
  <circle cx="100" cy="100" r="60" fill="none" stroke="teal" stroke-width="2"/>
  <path d="M 20 180 L 100 20 L 180 180 Z" fill="orange" opacity="0.6"/>
</svg>
```

Because SVG is text, you can generate it with string building, diff it in Git, and its size doesn't depend on the resolution. A `path` with many points (`M x y L x y L ...`) draws any curve.

## Curves from parametric equations

A **spirograph** (hypotrochoid) traces

```text
x = (R - r) cos t + d cos((R - r) / r · t)
y = (R - r) sin t - d sin((R - r) / r · t)
```

for t from 0 to 2π · r / gcd(R, r). Changing R, r and d gives endlessly different flowers. **Lissajous** curves (`x = sin(a t + δ)`, `y = sin(b t)`) are what old oscilloscopes drew.

Use the **invariant culture** when writing numbers into SVG: a Polish or German Windows would otherwise write `12,5` instead of `12.5` and break the file.

@@quiz
Why call `Dispose()` on Bitmap and Graphics objects?
* They hold unmanaged GDI+ resources (memory, file handles) that the garbage collector doesn't free promptly
- To save the image
- To clear the colours
- It's optional cleanup with no effect
? Use try/finally for long-running scripts.

@@quiz
Why format SVG coordinates with the invariant culture?
- SVG requires integers
* Some cultures use a comma as the decimal separator, which would produce invalid SVG numbers
- To make the file smaller
- Browsers only read US English
? 12,5 is two numbers in SVG.

@@flashcards
`System.Drawing.Bitmap` :: pixel image (GDI+)
`Graphics.FromImage` :: draw on a bitmap
`SetPixel` / `GetPixel` :: per-pixel access (slow)
`LockBits` :: fast raw pixel access
`Save(..., Png)` :: write a PNG
SVG :: vector graphics as XML text
`<path d="M ... L ...">` :: draw a polyline
spirograph :: hypotrochoid curve
Lissajous :: sin(a t), sin(b t)
PNG signature :: 89 50 4E 47

@@exercise A plasma bitmap
Render a 64×48 "plasma" image with `System.Drawing`:

- value `v = sin(x/8) + sin(y/6) + sin((x + y)/10)` (range -3..3), mapped to 0..255 with `[int](($v + 3) / 6 * 255)`
- colour: red = v, green = 255 - v, blue = 128 (use `SetPixel`)
- save as PNG in a temp file, then read it back and print: the PNG signature bytes, the image size from `[System.Drawing.Image]::FromFile`, the colour at (0,0) and (32,24), and how many pixels have red > 200
- dispose every GDI+ object (and release the file) before deleting it
@@starter
Add-Type -AssemblyName System.Drawing
$path = Join-Path ([System.IO.Path]::GetTempPath()) "plasma-$PID.png"
'TODO'
@@expected
signature: 89 50 4E 47
size: 64x48
pixel (0,0): R=128 G=127 B=128
pixel (32,24): R=36 G=219 B=128
bright red pixels: 447
@@requires
System.Drawing.Bitmap
SetPixel
Dispose()
ImageFormat]::Png
@@hint
`$bmp = [System.Drawing.Bitmap]::new(64, 48)`, loop `for ($y...) { for ($x...) { ... $bmp.SetPixel($x, $y, [System.Drawing.Color]::FromArgb($v, 255 - $v, 128)) } }`. Count bright pixels while drawing. After saving and disposing, read back with `$img = [System.Drawing.Bitmap]::FromFile($path)` and `$img.GetPixel(32, 24)`; dispose `$img` before `Remove-Item`, or the file stays locked.
@@solution
Add-Type -AssemblyName System.Drawing
$path = Join-Path ([System.IO.Path]::GetTempPath()) "plasma-$PID.png"

$bmp = [System.Drawing.Bitmap]::new(64, 48)
$bright = 0
try {
    for ($y = 0; $y -lt 48; $y++) {
        for ($x = 0; $x -lt 64; $x++) {
            $v = [math]::Sin($x / 8) + [math]::Sin($y / 6) + [math]::Sin(($x + $y) / 10)
            $c = [int](($v + 3) / 6 * 255)
            if ($c -gt 200) { $bright++ }
            $bmp.SetPixel($x, $y, [System.Drawing.Color]::FromArgb($c, 255 - $c, 128))
        }
    }
    $bmp.Save($path, [System.Drawing.Imaging.ImageFormat]::Png)
} finally {
    $bmp.Dispose()
}

$bytes = [System.IO.File]::ReadAllBytes($path)
"signature: $(($bytes[0..3] | ForEach-Object { $_.ToString('X2') }) -join ' ')"
$img = [System.Drawing.Bitmap]::FromFile($path)
try {
    "size: $($img.Width)x$($img.Height)"
    foreach ($p in @(@(0, 0), @(32, 24))) {
        $px = $img.GetPixel($p[0], $p[1])
        "pixel ($($p[0]),$($p[1])): R=$($px.R) G=$($px.G) B=$($px.B)"
    }
} finally {
    $img.Dispose()
}
"bright red pixels: $bright"
Remove-Item $path

@@exercise A spirograph in SVG
Generate an SVG file with three spirograph curves:

- `Get-Spirograph -R -SmallR -D -Steps` returns points of the hypotrochoid for t from 0 to 2π · r / gcd(R, r), centred at (150, 150)
- format coordinates with one decimal in the **invariant** culture, and build a `path` (`M x y L x y ...`) for each curve with a different stroke colour
- write the SVG (UTF-8, no BOM), then load it back as `[xml]` and print: the root element and size, the number of paths, the points per path, the first point of the first path and the colours
@@starter
$curves = @(
    @{ R = 100; SmallR = 30; D = 50; Color = 'teal' }
    @{ R = 100; SmallR = 45; D = 70; Color = 'orange' }
    @{ R = 90;  SmallR = 24; D = 30; Color = 'purple' }
)
$path = Join-Path ([System.IO.Path]::GetTempPath()) "spiro-$PID.svg"
'TODO'
@@expected
root: svg 300x300
paths: 3
points per path: 361, 361, 361
first point: 270.0 150.0
colours: teal, orange, purple
@@requires
InvariantCulture
[xml]
<path
@@hint
GCD: `function Get-Gcd($a, $b) { while ($b) { $a, $b = $b, ($a % $b) }; $a }`. With `$Steps = 360`, produce `$Steps + 1` points for i = 0..Steps: `$t = $tMax * $i / $Steps`. Format: `$x.ToString('0.0', $inv)`. Read back: `[xml]$doc = Get-Content $path -Raw`; paths are `$doc.svg.path`, and `$p.d -split ' L '` counts the points.
@@solution
$curves = @(
    @{ R = 100; SmallR = 30; D = 50; Color = 'teal' }
    @{ R = 100; SmallR = 45; D = 70; Color = 'orange' }
    @{ R = 90;  SmallR = 24; D = 30; Color = 'purple' }
)
$path = Join-Path ([System.IO.Path]::GetTempPath()) "spiro-$PID.svg"
$inv = [cultureinfo]::InvariantCulture

function Get-Gcd([int]$a, [int]$b) { while ($b) { $a, $b = $b, ($a % $b) }; $a }

function Get-Spirograph([double]$R, [double]$SmallR, [double]$D, [int]$Steps = 360) {
    $tMax = 2 * [math]::PI * $SmallR / (Get-Gcd $R $SmallR)
    $k = ($R - $SmallR) / $SmallR
    for ($i = 0; $i -le $Steps; $i++) {
        $t = $tMax * $i / $Steps
        $x = 150 + ($R - $SmallR) * [math]::Cos($t) + $D * [math]::Cos($k * $t)
        $y = 150 + ($R - $SmallR) * [math]::Sin($t) - $D * [math]::Sin($k * $t)
        [pscustomobject]@{ X = $x; Y = $y }
    }
}

$sb = [System.Text.StringBuilder]::new()
[void]$sb.AppendLine('<svg xmlns="http://www.w3.org/2000/svg" width="300" height="300" viewBox="0 0 300 300">')
[void]$sb.AppendLine('  <rect width="300" height="300" fill="#fafafa"/>')
foreach ($c in $curves) {
    $points = Get-Spirograph -R $c.R -SmallR $c.SmallR -D $c.D
    $coords = $points | ForEach-Object { "$($_.X.ToString('0.0', $inv)) $($_.Y.ToString('0.0', $inv))" }
    $d = 'M ' + ($coords -join ' L ')
    [void]$sb.AppendLine("  <path d=""$d"" fill=""none"" stroke=""$($c.Color)"" stroke-width=""1""/>")
}
[void]$sb.AppendLine('</svg>')
[System.IO.File]::WriteAllText($path, $sb.ToString(), [System.Text.UTF8Encoding]::new($false))

[xml]$doc = Get-Content $path -Raw
"root: $($doc.DocumentElement.LocalName) $($doc.svg.width)x$($doc.svg.height)"
$paths = @($doc.svg.path)
"paths: $($paths.Count)"
"points per path: $(($paths | ForEach-Object { ($_.d -split ' L ').Count }) -join ', ')"
"first point: $(($paths[0].d -split ' L ')[0] -replace '^M ', '')"
"colours: $(($paths | ForEach-Object stroke) -join ', ')"
Remove-Item $path
